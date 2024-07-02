
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for result in rdr.records() {
        let record = result?;
        col1.push(record[0].parse::<f64>()?);
        col2.push(record[1].parse::<f64>()?);
    }

    Ok((col1, col2))
}

fn mean(values: &[f64]) -> f64 {
    values.iter().copied().sum::<f64>() / values.len() as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut means_seq = Vec::new();
    let mut means_ts = Vec::new();
    let mut means_par = Vec::new();
    let mut means_n_process = Vec::new();
    let mut speedup = Vec::new();
    let mut theory = Vec::new();
    
    let file_paths_sequential = vec![
        "../rust_implementation/strong_scaling_data/sequential_3600.csv",
        // "../python_implementation/weak_scaling_data/sequential_512.csv",
        // "../python_implementation/weak_scaling_data/sequential_1024.csv",
        // "../python_implementation/weak_scaling_data/sequential_1536.csv",
        // "../python_implementation/weak_scaling_data/sequential_2048.csv",
        // "../python_implementation/weak_scaling_data/sequential_2560.csv",
        // "../python_implementation/weak_scaling_data/sequential_3072.csv",

    ];

    for file_path in file_paths_sequential {
        let (col1, col2) = read_csv(file_path)?;
        // getting time for sequential execution
        means_seq.push(mean(&col1));
        // getting time for code that can't be parallelized
        means_ts.push(mean(&col2));
    }

    let file_paths_parallel = vec![
        "../rust_implementation/strong_scaling_data/parallel_1.csv",
        "../rust_implementation/strong_scaling_data/parallel_4.csv",
        "../rust_implementation/strong_scaling_data/parallel_9.csv",
        "../rust_implementation/strong_scaling_data/parallel_16.csv",
        "../rust_implementation/strong_scaling_data/parallel_25.csv",
        "../rust_implementation/strong_scaling_data/parallel_36.csv",
        // "../python_implementation/weak_scaling_data/parallel_512.csv",
        // "../python_implementation/weak_scaling_data/parallel_1024.csv",
        // "../python_implementation/weak_scaling_data/parallel_1536.csv",
        // "../python_implementation/weak_scaling_data/parallel_2048.csv",
        // "../python_implementation/weak_scaling_data/parallel_2560.csv",
        // "../python_implementation/weak_scaling_data/parallel_3072.csv",
    ];

    for file_path in file_paths_parallel {
        let (col1, col2) = read_csv(file_path)?;
        // getting time for parallel execution
        means_par.push(mean(&col1));
        // getting time for number of processes
        means_n_process.push(mean(&col2));
    }
    // println!("{}", means_ts[0]/means_seq[0]  );

    for i in 0..means_par.len() {
        // Strong scaling

        speedup.push(means_seq[0] / means_par[i]);
        theory.push(1.0/(means_ts[0]/means_seq[0] + (1.0-means_ts[0]/means_seq[0])/means_n_process[i]));
        println!("No of processes: {}", means_n_process[i]);
        println!("Mean of ts (can't be parallelized): {}", means_ts[0]);
        println!("Mean of seq code: {}", means_seq[0]);
        println!("Divided: {}",  means_ts[0]/means_seq[0]);
        println!("Mean of par code: {}", means_par[i]);
        println!("Real speedup: {}", means_seq[0] / means_par[i]);
        println!("Theoretical speedup: {}", 1.0/(means_ts[0]/means_seq[0] + (1.0-means_ts[0]/means_seq[0])/means_n_process[i]));
        println!("");
        // println!("{}", 1.0/(means_ts[0]/means_seq[0] + (1.0-means_ts[0]/means_seq[0])/means_n_process[i]));

        // Weak scaling
        // speedup.push(means_seq[i] / means_par[i]);
        // theory.push(means_ts[i]/means_seq[i] + (1.0-means_ts[i]/means_seq[i])*means_n_process[i]);
        // println!("No of processes: {}", means_n_process[i]);
        // println!("Mean of ts (can't be parallelized): {}", means_ts[i]);
        // println!("Mean of seq code: {}", means_seq[i]);
        // println!("Mean of par code: {}", means_par[i]);
        // println!("Real speedup: {}", means_seq[i] / means_par[i]);
        // println!("Theoretical speedup: {}", means_ts[i]/means_seq[i] + (1.0-means_ts[i]/means_seq[i])*means_n_process[i]);
        // println!("");
    }

    // let root = BitMapBackend::new("strong_python_plot_with_theory.png", (800, 600)).into_drawing_area();
    // root.fill(&WHITE)?;

    // let y_range = 0.0..5.0;
    // let x_range =0.0..40.0;

    // let mut chart = ChartBuilder::on(&root)
    //     .caption("Strong scaling - Python", ("sans-serif", 50).into_font())
    //     .margin(30)
    //     .x_label_area_size(30)
    //     .y_label_area_size(30)
    //     .build_cartesian_2d(x_range, y_range)?;

    // chart
    //     .configure_mesh()
    //     .x_desc("No. of Processes")
    //     .y_desc("Speedup")
    //     .draw()?;

    // chart.draw_series(LineSeries::new(
    //     means_n_process.iter().zip(speedup.iter()).map(|(x, y)| (*x, *y)),
    //     &BLUE,
    // ))?
    // .label("Real Speedup")
    // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // chart.draw_series(LineSeries::new(
    //     means_n_process.iter().zip(theory.iter()).map(|(x, y)| (*x, *y)),
    //     &RED,
    // ))?
    // .label("Theoretical Speedup")
    // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // chart.configure_series_labels().draw()?;

    // root.present()?;
    Ok(())
}

