


use plotters::prelude::*;

fn cmyk_to_rgb(c: f32, m: f32, y: f32, k: f32) -> RGBColor {
    let r = 255.0 * (1.0 - c) * (1.0 - k);
    let g = 255.0 * (1.0 - m) * (1.0 - k);
    let b = 255.0 * (1.0 - y) * (1.0 - k);

    RGBColor(r as u8, g as u8, b as u8)
}

pub fn draw_pixel_cmyk<DB: DrawingBackend>(
    chart: &mut ChartContext<DB, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    x: f32,
    y: f32,
    c: f32,
    m: f32,
    y_comp: f32, // Renamed to y_comp to avoid conflict with y coordinate
    k: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let color = cmyk_to_rgb(c, m, y_comp, k);
    let pixel_size = 0.001; // Adjust based on your plot's scale and desired pixel appearance

    chart.draw_series(std::iter::once(
        Rectangle::new(
            [(x, y), (x + pixel_size, y + pixel_size)],
            color.filled(),
        )
    ))?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("pixel_plot_cmyk.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::new(root)
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

    chart.configure_mesh().draw()?;

    // cyan dott
    draw_pixel_cmyk(&mut chart, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0)?;

    // magneta dott
    draw_pixel_cmyk(&mut chart, 7.0, 8.0, 0.0, 1.0, 0.0, 0.0)?;

    // yellow dott
    draw_pixel_cmyk(&mut chart, 5.0, 2.0, 0.0, 0.0, 1.0, 0.0)?;

    // black dott
    draw_pixel_cmyk(&mut chart, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0)?;

    // black dott
    draw_pixel_cmyk(&mut chart, 9.0, 9.0, 0.60, 0.40, 0.40, 1.0)?;

    // green dott
    draw_pixel_cmyk(&mut chart, 4.0, 6.0, 0.8, 0.2, 0.7, 0.1)?;


    chart.plotting_area().present()?;
    Ok(())
}
