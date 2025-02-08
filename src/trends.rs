#[derive(Clone, Debug)]
pub struct TrendData<'a> {
    pub code: u8,             // code of trend (0-9)
    pub description: &'a str, // description of trend (ex. rising quickly)
    pub arrow: &'a str,       // unicode arrow for trend (ex. ↑↑)
}

impl<'b> TrendData<'b> {
    const fn new(code: u8, description: &'b str, arrow: &'b str) -> TrendData<'b> {
        TrendData {
            code,
            description,
            arrow,
        }
    }
}

pub fn get_trend<'c>(t: String) -> TrendData<'c> {
    TRENDS.get(&t).cloned().unwrap()
}

static TRENDS: phf::Map<&'static str, TrendData> = phf::phf_map! {
    "None" => TrendData::new(0, "", ""),
    "DoubleUp" => TrendData::new(1, "Rising quickly", "↑↑"),
    "SingleUp" => TrendData::new(2, "Rising", "↑"),
    "FortyFiveUp" => TrendData::new(3, "Rising slightly", "↗"),
    "Flat" => TrendData::new(4, "Steady", "→"),
    "FortyFiveDown" => TrendData::new(5, "Falling slightly", "↘"),
    "SingleDown" => TrendData::new(6, "Falling", "↓"),
    "DoubleDown" => TrendData::new(7, "Falling quickly", "↓↓"),
    "NotComputable" => TrendData::new(8, "Not computable", "??"),
    "RateOutOfRange" => TrendData::new(9, "Rate out of range", "!!"),
};
