use crate::error::Result;
use crate::fetch::get_data;
use charts_rs::{Box, CandlestickChart, LineChart};

pub fn gen_line_svg() -> Result<()> {
    let mut line_chart = LineChart::new_with_theme(
        vec![(
            "BTCUSDT",
            vec![
                95912.39, 96027.88, 95869.23, 95928.94, 95928.94, 96486.39, 95928.94, 96455.51,
                96455.51, 96469.27, 96392.01, 96393.91,
            ],
        )
            .into()],
        vec![
            "2013/1/24".to_string(),
            "2013/1/25".to_string(),
            "2013/1/28".to_string(),
            "2013/1/29".to_string(),
            "2013/1/30".to_string(),
            "2013/1/31".to_string(),
            "2013/2/1".to_string(),
            "2013/2/4".to_string(),
            "2013/2/5".to_string(),
            "2013/2/6".to_string(),
            "2013/2/7".to_string(),
            "2013/2/8".to_string(),
        ],
        "vintage",
    );
    line_chart.title_text = "BTCUSDT".to_string();
    line_chart.series_smooth = true;

    line_chart.y_axis_configs[0].axis_min = Some(95000.00);
    line_chart.y_axis_configs[0].axis_max = Some(97000.00);

    line_chart.legend_margin = Some(Box {
        top: 50.0,
        bottom: 10.0,
        ..Default::default()
    });
    std::fs::write("out.svg", line_chart.svg()?)?;
    Ok(())
}

pub async fn gen_candlestick_svg() -> Result<()> {
    let d = get_data("BTCUSDT", 5, 40).await?;
    let mut chart = CandlestickChart::new_with_theme(d.series_list, d.x_axis_data, "vintage");

    chart.title_text = "BTCUSDT".to_string();
    chart.y_axis_configs[0].axis_min = Some(d.min - 100.00);
    chart.y_axis_configs[0].axis_max = Some(d.max + 100.00);
    chart.legend_margin = Some(Box {
        top: 50.0,
        bottom: 10.0,
        ..Default::default()
    });

    std::fs::write("candles.svg", chart.svg()?)?;
    Ok(())
}
