use std::fmt::Write;

use bon::Builder;
use owo_colors::OwoColorize;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal::prelude::ToPrimitive;

use crate::bar_graph::colors::Color;
use crate::bar_graph::data::Data;

pub mod colors;
pub mod data;

#[derive(Debug, Builder, Clone)]
pub struct BarGraph {
    /// The max width of the graph
    width: u64,

    /// The number of blank lines between two bars (Default: 1)
    #[builder(default = 1)]
    space_between: u8,

    /// Group bars with the same label
    #[builder(default = true)]
    group_same_label: bool,

    /// The color of the bar
    #[builder(default = Color(255, 255, 255))]
    bar_color: Color,

    /// The color of the bar's value
    #[builder(default = Color(150, 150, 150))]
    bar_value_color: Color,

    /// The character used in the bar
    #[builder(default = '█')]
    bar_character: char,
}

impl BarGraph {
    pub fn get_bar_line(&self, data: &Data, max_y: Decimal) -> String {
        let mut line = String::new();

        // How many chars should it take?
        let chars = data.value.checked_div(max_y).unwrap_or_default() * Decimal::from(self.width);
        let bar_character = data.bar_character.as_ref().unwrap_or(&self.bar_character);

        for _ in 0..chars
            .round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero)
            .to_u16() // Anything bigger than an u16 is unrealistic, and timeout `cargo mutants` tests
            .unwrap()
        {
            line.push(*bar_character);
        }

        let bar_color = data.bar_color.as_ref().unwrap_or(&self.bar_color);

        line = line
            .truecolor(bar_color.0, bar_color.1, bar_color.2)
            .to_string();

        let bar_value_color = data
            .bar_value_color
            .as_ref()
            .unwrap_or(&self.bar_value_color);
        line.push_str(
            &format!(" - ({})", data.value_display())
                .truecolor(bar_value_color.0, bar_value_color.1, bar_value_color.2)
                .to_string(),
        );

        line
    }

    /// Format the label area for the current line
    fn get_line_label(&self, label: &str, label_area_width: usize) -> String {
        format!("{label:label_area_width$}│")
    }

    /// Format a complete bar line
    fn format_bar_line(
        &self,
        data: &Data,
        label_area_width: usize,
        max_y: Decimal,
        grouped: bool,
    ) -> String {
        let label = if grouped {
            self.get_line_label("", label_area_width)
        } else {
            self.get_line_label(&data.label, label_area_width)
        };

        format!("{label}{}", self.get_bar_line(data, max_y))
    }

    /// Return the lines between two bars
    fn get_lines_between_bars(&self, label_area_width: usize) -> String {
        let mut out = String::new();

        for _ in 0..self.space_between {
            writeln!(out, "{}", self.get_line_label("", label_area_width)).unwrap();
        }

        out
    }

    pub fn format_data(&self, data: &[Data]) -> String {
        if data.is_empty() {
            return String::new();
        }

        let label_area_width = data.iter().map(|data| data.label.len()).max().unwrap();
        let max_y = data.iter().map(|data| data.value).max().unwrap();

        let mut out = String::new();
        let mut last_label = None;
        for (i, data) in data.iter().enumerate() {
            let grouped =
                i != 0 && last_label.is_none_or(|lab| self.group_same_label && lab == data.label);

            // Add space between the bars
            if i != 0 && !grouped {
                out.push_str(&self.get_lines_between_bars(label_area_width));
            }
            last_label = Some(data.label.clone());

            writeln!(
                out,
                "{}",
                self.format_bar_line(data, label_area_width, max_y, grouped)
            )
            .unwrap();
        }

        out
    }
}

#[cfg(test)]
mod test {
    use crate::bar_graph::BarGraph;
    use crate::bar_graph::colors::Color;
    use crate::bar_graph::data::Data;

    #[test]
    fn graph_test() {
        let data = vec![
            Data::builder().label("January").value(1562).build(),
            Data::builder()
                .label("January")
                .value(1239)
                .bar_color(Color(0, 50, 200))
                .bar_value_color(Color(255, 150, 0))
                .build(),
            Data::builder().label("Feburary").value(2519).build(),
            Data::builder()
                .label("Feburary")
                .value(2619)
                .bar_color(Color(0, 50, 200))
                .bar_character('▒')
                .build(),
            Data::builder().label("March").value(2715).build(),
            Data::builder().label("April").value(2828).build(),
            Data::builder().label("May").value(2920).build(),
            Data::builder().label("June").value(2335).build(),
        ];

        let graph = BarGraph::builder()
            .width(60)
            .bar_color(Color(20, 150, 50))
            .build();

        assert_eq!(graph.format_data(&data), "January │[38;2;20;150;50m████████████████████████████████[39m[38;2;150;150;150m - (1562)[39m
        │[38;2;0;50;200m█████████████████████████[39m[38;2;255;150;0m - (1239)[39m
        │
Feburary│[38;2;20;150;50m████████████████████████████████████████████████████[39m[38;2;150;150;150m - (2519)[39m
        │[38;2;0;50;200m▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒[39m[38;2;150;150;150m - (2619)[39m
        │
March   │[38;2;20;150;50m████████████████████████████████████████████████████████[39m[38;2;150;150;150m - (2715)[39m
        │
April   │[38;2;20;150;50m██████████████████████████████████████████████████████████[39m[38;2;150;150;150m - (2828)[39m
        │
May     │[38;2;20;150;50m████████████████████████████████████████████████████████████[39m[38;2;150;150;150m - (2920)[39m
        │
June    │[38;2;20;150;50m████████████████████████████████████████████████[39m[38;2;150;150;150m - (2335)[39m\n")
    }
}
