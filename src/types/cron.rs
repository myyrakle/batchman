use crate::errors;

#[derive(Debug, Clone, PartialEq)]
pub enum CronExpressionField {
    All,                                  // * = wildcard, matches any value
    Elements(Vec<CronExpressionElement>), // e.g., 1,2,3
}

#[derive(Debug, Clone, PartialEq)]
pub enum CronExpressionElement {
    Single(u32),     // e.g., 5
    Range(u32, u32), // e.g., 1-5
    Step(u32, u32),  // e.g., 0/15 (every 15 minutes)
}

impl CronExpressionElement {
    pub fn contains(&self, value: u32) -> bool {
        match self {
            CronExpressionElement::Single(v) => *v == value,
            CronExpressionElement::Range(start, end) => *start <= value && value <= *end,
            CronExpressionElement::Step(base, step) => {
                if *step == 0 {
                    return false; // Step cannot be zero
                }

                (value - base) % step == 0
            }
        }
    }
}

// The CronExpression struct represents a cron expression.
// example expression: "* * * * ? *"
#[derive(Debug, Clone, PartialEq)]
pub struct CronExpression {
    pub minutes: CronExpressionField,
    pub hours: CronExpressionField,
    pub day_of_month: CronExpressionField,
    pub month: CronExpressionField,
    pub day_of_week: CronExpressionField,
    pub year: Option<CronExpressionField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CronExpressionParseError {
    pub message: String,
}

impl std::fmt::Display for CronExpressionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cron expression parse error: {}", self.message)
    }
}
impl std::error::Error for CronExpressionParseError {}

impl CronExpression {
    pub fn parse(expression: &str) -> Result<Self, errors::Error> {
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() < 5 {
            return Err(errors::Error::CronExpressionIsInvalid(
                "Invalid cron expression format. Expected at least 5 fields.".to_string(),
            ));
        }

        if parts.len() > 6 {
            return Err(errors::Error::CronExpressionIsInvalid(
                "Invalid cron expression format. Expected at most 6 fields.".to_string(),
            ));
        }

        let has_year = parts.len() == 6;

        let minutes = CronExpression::parse_field(parts[0])?;
        let hours = CronExpression::parse_field(parts[1])?;
        let day_of_month = CronExpression::parse_field(parts[2])?;
        let month = CronExpression::parse_field(parts[3])?;
        let day_of_week = CronExpression::parse_field(parts[4])?;

        let year = if has_year {
            Some(CronExpression::parse_field(parts[5])?)
        } else {
            None
        };

        Ok(CronExpression {
            minutes,
            hours,
            day_of_month,
            month,
            day_of_week,
            year,
        })
    }

    fn parse_field(field: &str) -> Result<CronExpressionField, errors::Error> {
        if field == "*" {
            return Ok(CronExpressionField::All);
        }

        if field == "?" {
            return Ok(CronExpressionField::All); // '?' is often used in day of month or day of week fields
        }

        let elements: Vec<CronExpressionElement> = field
            .split(',')
            .map(|part| CronExpression::parse_element(part))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CronExpressionField::Elements(elements))
    }

    fn parse_element(part: &str) -> Result<CronExpressionElement, errors::Error> {
        if part.contains('/') {
            let parts: Vec<&str> = part.split('/').collect();
            if parts.len() != 2 {
                return Err(errors::Error::CronExpressionIsInvalid(format!(
                    "Invalid step expression: {}",
                    part
                )));
            }
            let base = parts[0].parse::<u32>().map_err(|_| {
                errors::Error::CronExpressionIsInvalid(format!(
                    "Invalid step base in expression: {}",
                    part
                ))
            })?;
            let step = parts[1].parse::<u32>().map_err(|_| {
                errors::Error::CronExpressionIsInvalid(format!(
                    "Invalid step value in expression: {}",
                    part
                ))
            })?;
            return Ok(CronExpressionElement::Step(base, step));
        }

        if part.contains('-') {
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() != 2 {
                return Err(errors::Error::CronExpressionIsInvalid(format!(
                    "Invalid range expression: {}",
                    part
                )));
            }
            let start = parts[0].parse::<u32>().map_err(|_| {
                errors::Error::CronExpressionIsInvalid(format!(
                    "Invalid start of range in expression: {}",
                    part
                ))
            })?;
            let end = parts[1].parse::<u32>().map_err(|_| {
                errors::Error::CronExpressionIsInvalid(format!(
                    "Invalid end of range in expression: {}",
                    part
                ))
            })?;
            return Ok(CronExpressionElement::Range(start, end));
        }

        let value = part.parse::<u32>().map_err(|_| {
            errors::Error::CronExpressionIsInvalid(format!(
                "Invalid single value in expression: {}",
                part
            ))
        })?;

        Ok(CronExpressionElement::Single(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_parse() {
        struct TestCase {
            expression: &'static str,
            expected: Result<CronExpression, errors::Error>,
        }

        let test_cases = vec![
            TestCase {
                expression: "* * * * ? *",
                expected: Ok(CronExpression {
                    minutes: CronExpressionField::All,
                    hours: CronExpressionField::All,
                    day_of_month: CronExpressionField::All,
                    month: CronExpressionField::All,
                    day_of_week: CronExpressionField::All,
                    year: Some(CronExpressionField::All),
                }),
            },
            TestCase {
                expression: "0 12 * * ? *",
                expected: Ok(CronExpression {
                    minutes: CronExpressionField::Elements(vec![CronExpressionElement::Single(0)]),
                    hours: CronExpressionField::Elements(vec![CronExpressionElement::Single(12)]),
                    day_of_month: CronExpressionField::All,
                    month: CronExpressionField::All,
                    day_of_week: CronExpressionField::All,
                    year: Some(CronExpressionField::All),
                }),
            },
            TestCase {
                expression: "0 0/15 * * ? *",
                expected: Ok(CronExpression {
                    minutes: CronExpressionField::Elements(vec![CronExpressionElement::Single(0)]),
                    hours: CronExpressionField::Elements(vec![CronExpressionElement::Step(0, 15)]),
                    day_of_month: CronExpressionField::All,
                    month: CronExpressionField::All,
                    day_of_week: CronExpressionField::All,
                    year: Some(CronExpressionField::All),
                }),
            },
            TestCase {
                expression: "5-10 1-3 * * ? *",
                expected: Ok(CronExpression {
                    minutes: CronExpressionField::Elements(vec![CronExpressionElement::Range(
                        5, 10,
                    )]),
                    hours: CronExpressionField::Elements(vec![CronExpressionElement::Range(1, 3)]),
                    day_of_month: CronExpressionField::All,
                    month: CronExpressionField::All,
                    day_of_week: CronExpressionField::All,
                    year: Some(CronExpressionField::All),
                }),
            },
        ];

        for test_case in test_cases {
            let result = CronExpression::parse(test_case.expression);
            assert_eq!(
                result, test_case.expected,
                "Failed for expression: {}",
                test_case.expression
            );
        }
    }
}
