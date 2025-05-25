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
