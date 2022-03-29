use {
    crate::ast::{Expr, Function},
    std::iter::empty,
};

impl Function {
    pub fn as_exprs(&self) -> impl ExactSizeIterator<Item = &Expr> {
        #[derive(iter_enum::Iterator, iter_enum::ExactSizeIterator)]
        enum Exprs<I0, I1, I2, I3> {
            Empty(I0),
            Single(I1),
            Double(I2),
            Tripple(I3),
        }

        match self {
            Self::Now() | Function::Pi() | Function::GenerateUuid() => Exprs::Empty(empty()),
            Self::Lower(expr)
            | Self::Upper(expr)
            | Self::Sin(expr)
            | Self::Cos(expr)
            | Self::Tan(expr)
            | Self::ASin(expr)
            | Self::ACos(expr)
            | Self::ATan(expr)
            | Self::Radians(expr)
            | Self::Degrees(expr)
            | Self::Ceil(expr)
            | Self::Round(expr)
            | Self::Floor(expr)
            | Self::Exp(expr)
            | Self::Ln(expr)
            | Self::Log2(expr)
            | Self::Log10(expr)
            | Self::Sqrt(expr)
            | Self::Ltrim { expr, chars: None }
            | Self::Rtrim { expr, chars: None }
            | Self::Trim {
                expr,
                filter_chars: None,
                ..
            }
            | Self::Reverse(expr) => Exprs::Single([expr].into_iter()),
            Self::Left { expr, size: expr2 }
            | Self::Right { expr, size: expr2 }
            | Self::Lpad {
                expr,
                size: expr2,
                fill: None,
            }
            | Self::Rpad {
                expr,
                size: expr2,
                fill: None,
            }
            | Self::Trim {
                expr,
                filter_chars: Some(expr2),
                ..
            }
            | Self::Log {
                antilog: expr,
                base: expr2,
            }
            | Self::Div {
                dividend: expr,
                divisor: expr2,
            }
            | Self::Mod {
                dividend: expr,
                divisor: expr2,
            }
            | Self::Gcd {
                left: expr,
                right: expr2,
            }
            | Self::Lcm {
                left: expr,
                right: expr2,
            }
            | Self::Power { expr, power: expr2 }
            | Self::Ltrim {
                expr,
                chars: Some(expr2),
            }
            | Self::Rtrim {
                expr,
                chars: Some(expr2),
            }
            | Self::Repeat { expr, num: expr2 }
            | Self::Substr {
                expr,
                start: expr2,
                count: None,
            }
            | Self::Unwrap {
                expr,
                selector: expr2,
            } => Exprs::Double([expr, expr2].into_iter()),
            Self::Lpad {
                expr,
                size: expr2,
                fill: Some(expr3),
            }
            | Self::Rpad {
                expr,
                size: expr2,
                fill: Some(expr3),
            }
            | Self::Substr {
                expr,
                start: expr2,
                count: Some(expr3),
            } => Exprs::Tripple([expr, expr2, expr3].into_iter()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Expr, parse_sql::parse_expr, translate::translate_expr};

    fn expr(sql: &str) -> Expr {
        let parsed = parse_expr(sql).expect(sql);

        translate_expr(&parsed).expect(sql)
    }

    fn test(sql: &str, expected: &[&str]) {
        let function = match expr(sql) {
            Expr::Function(function) => *function,
            _ => unreachable!("only for function tests"),
        };
        let actual = function.as_exprs();

        assert_eq!(actual.len(), expected.len(), "{sql}");

        for (expected, actual) in expected.iter().zip(actual) {
            assert_eq!(actual, &expr(expected), "{sql}");
        }
    }

    #[test]
    fn as_exprs() {
        // Empty
        test("NOW()", &[]);
        test("PI()", &[]);
        test("GENERATE_UUID()", &[]);

        // Single
        test("LOWER(id)", &["id"]);
        test(r#"UPPER("Hello")"#, &[r#""Hello""#]);
        test("SIN(3.14)", &["3.14"]);
        test("COS(3.14)", &["3.14"]);
        test("TAN(3.14)", &["3.14"]);
        test("ASIN(3.14)", &["3.14"]);
        test("ACOS(3.14)", &["3.14"]);
        test("ATAN(3.14)", &["3.14"]);
        test("RADIANS(180)", &["180"]);
        test("DEGREES(3.14)", &["3.14"]);
        test("CEIL(1.23)", &["1.23"]);
        test("ROUND(1.23)", &["1.23"]);
        test("FLOOR(1.23)", &["1.23"]);
        test("EXP(1.23)", &["1.23"]);
        test("LN(col + 1)", &["col + 1"]);
        test("LOG2(16)", &["16"]);
        test("LOG10(150 - 50)", &["150 - 50"]);
        test("SQRT(144)", &["144"]);
        test(r#"LTRIM("  hello")"#, &[r#""  hello""#]);
        test(r#"RTRIM("world  ")"#, &[r#""world  ""#]);
        test(r#"TRIM("  rust  ")"#, &[r#""  rust  ""#]);
        test(r#"REVERSE("abcde")"#, &[r#""abcde""#]);

        // Double
        test(r#"LEFT("hello", 2)"#, &[r#""hello""#, "2"]);
        test(r#"RIGHT("hello", 2)"#, &[r#""hello""#, "2"]);
        test(r#"LPAD(value, 5)"#, &["value", "5"]);
        test(r#"RPAD(value, 5)"#, &["value", "5"]);
        test(
            r#"TRIM(LEADING "_" FROM "__hello")"#,
            &[r#""__hello""#, r#""_""#],
        );
        test("LOG(rate, 2)", &["rate", "2"]);
        test("DIV(6, 2)", &["6", "2"]);
        test("MOD(6, 2)", &["6", "2"]);
        test("GCD(6, 2)", &["6", "2"]);
        test("LCM(6, 2)", &["6", "2"]);
        test("POWER(base, 10)", &["base", "10"]);
        test(r#"LTRIM(name, "xyz")"#, &["name", r#""xyz""#]);
        test(r#"RTRIM(name, "xyz")"#, &["name", r#""xyz""#]);
        test("REPEAT(col || col2, 3)", &["col || col2", "3"]);
        test("REPEAT(column, 2)", &["column", "2"]);
        test(r#"UNWRAP(field, "foo.1")"#, &["field", r#""foo.1""#]);

        // Tripple
        test(
            r#"LPAD(name, 20, '>")++++<')"#,
            &["name", "20", r#"'>")++++<'"#],
        );
        test(
            r#"RPAD(name, 20, '>")++++<')"#,
            &["name", "20", r#"'>")++++<'"#],
        );
        test(
            r#"SUBSTR('   >++++("<   ', 3, 11)"#,
            &[r#"'   >++++("<   '"#, "3", "11"],
        );
    }
}