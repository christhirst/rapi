use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country_month {
    //#[allow(dead_code)]
    //pub title: String,
    pub population_by_month: u64,
    pub month: month,
}
