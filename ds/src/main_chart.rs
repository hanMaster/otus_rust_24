use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use charts_rs::{Box, LineChart};
    let mut line_chart = LineChart::new_with_theme(
        vec![
            ("Email", vec![120.0, 132.0, 101.0, 134.0, 90.0, 230.0, 210.0]).into(),
            ("Union Ads", vec![220.0, 182.0, 191.0, 234.0, 290.0, 330.0, 310.0]).into(),
            ("Direct", vec![320.0, 332.0, 301.0, 334.0, 390.0, 330.0, 320.0]).into(),
            ("Search Engine", vec![820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0]).into(),
        ],
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ],
        "vintage",
    );
    line_chart.title_text = "Stacked Area Chart".to_string();
    line_chart.series_smooth = true;
    // line_chart.sub_title_text = "Hello World".to_string();
    line_chart.legend_margin = Some(Box {
        top: 50.0,
        bottom: 10.0,
        ..Default::default()
    });
    // line_chart.series_list[3].label_show = true;
    fs::write("out.svg", line_chart.svg()? )?;
    Ok(())
}