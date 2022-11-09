use num::complex::Complex;
use plotters::prelude::*;

fn main() {
    draw_mandelbrot().unwrap()
}

type ComplexDouble = Complex<f64>;
const OUT_FILE_NAME: &'static str = "mandelbrot.png";

/// Method implementing the mandelbrot condition
/// $$f_c(z) = z^2 + c$$
///
/// * `c`: Complex number input (e.g. pixel coordinate in mandelbrot image)
/// * `num_iterations`: Number of iterations to perform
fn mandelbrot(c: &ComplexDouble, num_iterations: u32) -> u32 {
    let mut diverge_count: u32 = 0;

    let mut z = ComplexDouble::new(0.0, 0.0);
    while diverge_count <= num_iterations {
        if z.norm() > 2. {
            return diverge_count;
        }

        z = z.powi(2) + c;
        diverge_count += 1;
    }
    num_iterations
}

fn draw_mandelbrot() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1600, 1200)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20 as i32)
        .x_label_area_size(10 as i32)
        .y_label_area_size(10 as i32)
        .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let samples = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (real, complex) = (chart.x_range(), chart.y_range());

    let step = (
        (real.end - real.start) / samples.0 as f64,
        (complex.end - complex.start) / samples.1 as f64,
    );

    const NUM_CONVERGE: u32 = 100;

    for k in 0..(samples.0 * samples.1) {
        let z = ComplexDouble::new(
            real.start + step.0 * (k % samples.0) as f64,
            complex.start + step.1 * (k / samples.0) as f64,
        );

        let count = mandelbrot(&z, NUM_CONVERGE);

        let ComplexDouble { re: a, im: b } = z;

        if count != NUM_CONVERGE {
            plotting_area.draw_pixel((a, b), &HSLColor(count as f64 / 100.0, 1.0, 0.5))?;
        } else {
            plotting_area.draw_pixel((a, b), &BLACK)?;
        }
    }

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandelbrot_test() {
        const NUM_ITERATIONS: u32 = 20;

        //  Not in the mandelbrot set
        let z1 = ComplexDouble::new(0.25, 0.75);
        assert_ne!(mandelbrot(&z1, NUM_ITERATIONS), NUM_ITERATIONS);

        let z2 = ComplexDouble::new(-1., 0.5);
        assert_ne!(mandelbrot(&z2, NUM_ITERATIONS), NUM_ITERATIONS);

        //  In the mandelbrot set
        let z3 = ComplexDouble::new(0., 0.);
        assert_eq!(mandelbrot(&z3, NUM_ITERATIONS), NUM_ITERATIONS);

        let z4 = ComplexDouble::new(1. / 8., -1. / 8.);
        assert_eq!(mandelbrot(&z4, NUM_ITERATIONS), NUM_ITERATIONS);
    }
}
