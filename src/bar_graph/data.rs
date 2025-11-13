use bon::Builder;
use rust_decimal::Decimal;

use crate::bar_graph::colors::Color;

#[derive(Debug, Builder, Clone)]
pub struct Data {
    /// Label of the bar
    #[builder(into)]   
    pub label: String,

    /// The value of the bar
    #[builder(into)]   
    pub value: Decimal,

    /// The display version of the value in case the value needs a special formating (Ex: A duration)
    value_display: Option<String>,

    /// The color of the bar. This overwrite the default set in the bar graph
    pub(crate) bar_color: Option<Color>,

    /// The color of the bar's value. This overwrite the default set in the bar graph
    pub(crate) bar_value_color: Option<Color>,

    /// The character used in the bar. This overwrite the default set in the bar graph
    pub(crate) bar_character: Option<char>,
}

impl Data {
    pub fn value_display(&self) -> String {
        self.value_display.as_ref().map(|s| s.to_string()).unwrap_or(self.value.to_string())
    }
}