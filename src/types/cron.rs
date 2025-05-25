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

#[derive(Debug, Clone, PartialEq)]
pub struct CronExpression {
    pub minutes: CronExpressionField,
    pub hours: CronExpressionField,
    pub day_of_month: CronExpressionField,
    pub month: CronExpressionField,
    pub day_of_week: CronExpressionField,
    // pub year: Option<CronExpressionField>, // Year is optional spec
}
