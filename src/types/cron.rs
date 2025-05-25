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
    pub fn parse(expression: &str) -> Result<Self, CronExpressionParseError> {
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() < 5 {
            return Err(CronExpressionParseError {
                message: "Invalid cron expression format. Expected at least 5 fields.".to_string(),
            });
        }

        if parts.len() > 6 {
            return Err(CronExpressionParseError {
                message: "Invalid cron expression format. Expected at most 6 fields.".to_string(),
            });
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

    fn parse_field(field: &str) -> Result<CronExpressionField, CronExpressionParseError> {
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

    fn parse_element(part: &str) -> Result<CronExpressionElement, CronExpressionParseError> {
        if part.contains('/') {
            let parts: Vec<&str> = part.split('/').collect();
            if parts.len() != 2 {
                return Err(CronExpressionParseError {
                    message: format!("Invalid step expression: {}", part),
                });
            }
            let base = parts[0]
                .parse::<u32>()
                .map_err(|_| CronExpressionParseError {
                    message: format!("Invalid step base in expression: {}", part),
                })?;
            let step = parts[1]
                .parse::<u32>()
                .map_err(|_| CronExpressionParseError {
                    message: format!("Invalid step value in expression: {}", part),
                })?;
            return Ok(CronExpressionElement::Step(base, step));
        }

        if part.contains('-') {
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() != 2 {
                return Err(CronExpressionParseError {
                    message: format!("Invalid range expression: {}", part),
                });
            }
            let start = parts[0]
                .parse::<u32>()
                .map_err(|_| CronExpressionParseError {
                    message: format!("Invalid start of range in expression: {}", part),
                })?;
            let end = parts[1]
                .parse::<u32>()
                .map_err(|_| CronExpressionParseError {
                    message: format!("Invalid end of range in expression: {}", part),
                })?;
            return Ok(CronExpressionElement::Range(start, end));
        }

        let value = part.parse::<u32>().map_err(|_| CronExpressionParseError {
            message: format!("Invalid single value in expression: {}", part),
        })?;

        Ok(CronExpressionElement::Single(value))
    }
}
