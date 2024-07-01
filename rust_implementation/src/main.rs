use ndarray::Array2;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::fs;
use std::path::Path;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use rust_implementation::seq::sequential_processing;
use hello_cargo::par::parallel_processing;
use hello_cargo::csv_writer::save_matrix;
use std::time::{Duration, Instant};
use std::io::Write;
use csv::Writer;


pub fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Array2<f64>, Box<dyn Error>> {
    let file = File::open(&filename)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row: Result<Vec<f64>, _> = record.iter().map(|field| field.parse::<f64>()).collect();
        rows.push(row?);
    }

    let num_cols = rows.get(0).map_or(0, |row| row.len());
    let flat_vec: Vec<f64> = rows.into_iter().flatten().collect();
    let num_rows = flat_vec.len() / num_cols;
    let matrix = Array2::from_shape_vec((num_rows, num_cols), flat_vec)?;

    Ok(matrix)
}

pub fn write_to_csv(filename: &str, time: u128, midtime: u128) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).create(true).open(filename)?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);

    // Write the header
    //wtr.write_record(&["time", "midtime"])?;

    // Write the time and midtime
    wtr.write_record(&[time.to_string(), midtime.to_string()])?;

    wtr.flush()?;
    Ok(())
}

fn main() {
    let now = Instant::now();
    let input_matrix = read_csv("../data/input_matrix_8.csv").unwrap();
    let filter_matrix = read_csv("../data/filter_matrix_2.csv").unwrap();
    let midtime = now.elapsed().as_millis();

    let stride = 2;
    let padding = 0;
    let pool_size = 2;
    let pool_stride = 2;
    let num_parts = 4;

    let seq_directory = format!("seq_visualization_i{}_f{}_s{}_p{}_ps{}_pstr{}", input_matrix.shape()[0], filter_matrix.shape()[0],
                            stride, padding, pool_size, pool_stride);
    fs::create_dir(&seq_directory);
    fs::create_dir(Path::new(&seq_directory).join("input"));
    fs::create_dir(Path::new(&seq_directory).join("convolution"));
    fs::create_dir(Path::new(&seq_directory).join("relu"));
    fs::create_dir(Path::new(&seq_directory).join("pooling"));
    fs::create_dir(Path::new(&seq_directory).join("output"));
    save_matrix(&input_matrix, Path::new(&seq_directory).join("input").join("input_matrix.csv").to_str().unwrap());
    save_matrix(&filter_matrix, Path::new(&seq_directory).join("input").join("filter_matrix.csv").to_str().unwrap());

    let seq_output = sequential_processing(&input_matrix, &filter_matrix, stride, padding, pool_size, pool_stride, seq_directory.clone());
    // println!("Sequential Processing Output:\n{:?}", seq_output);
    save_matrix(&seq_output, Path::new(&seq_directory).join("output").join("output_matrix.csv").to_str().unwrap());


    let par_directory = format!("par_visualization_np{}_i{}_f{}_s{}_p{}_ps{}_pstr{}", num_parts, input_matrix.shape()[0], filter_matrix.shape()[0],
                            stride, padding, pool_size, pool_stride);
    fs::create_dir(&par_directory);
    fs::create_dir(Path::new(&par_directory).join("input"));
    fs::create_dir(Path::new(&par_directory).join("submatrices"));
    fs::create_dir(Path::new(&par_directory).join("convolution"));
    fs::create_dir(Path::new(&par_directory).join("relu"));
    fs::create_dir(Path::new(&par_directory).join("pooling"));
    fs::create_dir(Path::new(&par_directory).join("output"));
    save_matrix(&input_matrix, Path::new(&par_directory).join("input").join("input_matrix.csv").to_str().unwrap());
    save_matrix(&filter_matrix, Path::new(&par_directory).join("input").join("filter_matrix.csv").to_str().unwrap());

    let par_output = parallel_processing(&input_matrix, &filter_matrix, num_parts, stride, padding, pool_size, pool_stride, par_directory.clone());
    save_matrix(&par_output, Path::new(&par_directory).join("output").join("output_matrix.csv").to_str().unwrap());
        // let time = now.elapsed().as_millis();
        // println!("Sequential Processing Output:\n{:?}", par_output);
        // write_to_csv("strong_scaling_data_2/parallel_9.csv", time, 9);


    // println!("Sequential Processing Output:\n{:?}", seq_output);

    // Parallel processing
    // let now2 = Instant::now();
    // let num_parts = 4;
    // let par_output = parallel_processing(&input_matrix, &filter_matrix, num_parts, stride, padding, pool_size, pool_stride);
    // // println!("{}", now.elapsed().as_millis());

    // println!("{}", now2.elapsed().as_millis());
    // println!("Parallel Processing Output:\n{:?}", par_output);
}
