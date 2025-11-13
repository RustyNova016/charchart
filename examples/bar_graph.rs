use charchart::bar_graph::BarGraph;
use charchart::bar_graph::colors::Color;
use charchart::bar_graph::data::Data;

pub fn main() {
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
        .space_between(0)
        .bar_color(Color(20, 150, 50))
        .build();

    println!("{}", graph.format_data(&data));
}
