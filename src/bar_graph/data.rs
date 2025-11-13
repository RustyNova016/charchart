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

    /// The color of the bar. This overwrite the default set in the bar graph
    pub(crate) bar_color: Option<Color>,

    /// The color of the bar's value. This overwrite the default set in the bar graph
    pub(crate) bar_value_color: Option<Color>,

    /// The character used in the bar. This overwrite the default set in the bar graph
    pub(crate) bar_character: Option<char>,
}

impl Data {

}
