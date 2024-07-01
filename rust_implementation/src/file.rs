// use std::error::Error;
// use std::fs::File;
// use std::path::Path;
// use csv::WriterBuilder;
// use rand::prelude::*;
// extern crate matrix_display;
// use matrix_display::*;
// use std::thread;
// use std::time::Duration;
// use std::{time};
// extern crate image;

// use std::io::Cursor;
// use image::{RgbImage, Rgb, ImageFormat, DynamicImage};
// use csv::{ReaderBuilder, StringRecord};
// use ndarray::{Array2, ArrayView2, s};
// use image::io::Reader as ImageReader;

// // Step 1: Generating random matrices (without saving to files)
// fn generate_random_matrix(shape: (usize, usize)) -> Array2<f64> {
//     let mut rng = rand::thread_rng();
//     Array2::from_shape_fn(shape, |_| rng.gen())
// }

// // Step 2: Convolution operation
// fn convolution(input_matrix: &Array2<f64>, filter_matrix: &Array2<f64>, stride: usize, padding: usize) -> Array2<f64> {
//     let filter_size = filter_matrix.shape()[0];
//     let padded_size = input_matrix.shape()[0] + 2 * padding;
//     let output_size = (padded_size - filter_size) / stride + 1;

//     let mut input_padded = Array2::zeros((padded_size, padded_size));
//     input_padded.slice_mut(s![padding..padded_size-padding, padding..padded_size-padding])
//         .assign(input_matrix);

//     let mut output_matrix = Array2::zeros((output_size, output_size));

//     for i in 0..output_size {
//         for j in 0..output_size {
//             let region = input_padded.slice(s![
//                 i*stride..i*stride+filter_size,
//                 j*stride..j*stride+filter_size
//             ]).to_owned(); // Convert view to owned array
//             output_matrix[(i, j)] = (region * filter_matrix).sum();

//         }
//     }

//     output_matrix
// }

// // Step 3: ReLU activation function
// fn relu(matrix: &Array2<f64>) -> Array2<f64> {
//     matrix.mapv(|x| x.max(0.0))
// }

// // Step 4: Max pooling operation
// fn max_pooling(matrix: &Array2<f64>, pool_size: usize, stride: usize) -> Array2<f64> {
//     let output_size = (matrix.shape()[0] - pool_size) / stride + 1;
//     let mut output_matrix = Array2::zeros((output_size, output_size));
    
//     for i in 0..output_size {
//         for j in 0..output_size {
//             let region = matrix.slice(s![
//                 i * stride..i * stride + pool_size,
//                 j * stride..j * stride + pool_size
//             ]);
//             print!("{}[2J", 27 as char);
//             output_matrix[(i, j)] = region.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
//             let k = i*2;
//             let l = j*2;
//             let format = Format::new(5,5);
//             let board = matrix.clone().into_raw_vec().iter()
//             .enumerate()
//             .map(|(i, &x)| {  // Explicitly destructure to get &f64
//                 let mut ansi_fg = 0;
//                 let mut ansi_bg = 70;
//                 if i == k*8+l || i == k*8+l+1 || i == (k+1)*8+l || i == (k+1)*8+l+1{
//                     ansi_bg = 7;
//                     ansi_fg = 33;
//                 }
//                 cell::Cell::new((x * 100.0).round() / 100.0, ansi_fg, ansi_bg)
//             })
//             .collect::<Vec<_>>();
//             let data = matrix::Matrix::new(8, board); // Assuming Matrix::new takes size and initial data
//             let display = MatrixDisplay::new(format, data); 
//             display.print(&mut std::io::stdout(), &style::BordersStyle::Thin);
//             println!("=========MATRIX BEFORE MAX POOL STEP===========");

//             let kk = i*4;
//             let ll = j;
//             let format = Format::new(5,5);
//             let board = output_matrix.clone().into_raw_vec().iter()
//             .enumerate()
//             .map(|(i, &x)| {  // Explicitly destructure to get &f64
//                 let mut ansi_fg = 0;
//                 let mut ansi_bg = 33;
//                 if i == kk+ll {
//                     ansi_bg = 7;
//                     ansi_fg = 33;
//                 }
//                 cell::Cell::new((x * 100.0).round() / 100.0, ansi_fg, ansi_bg)
//             })
//             .collect::<Vec<_>>();
//             let data = matrix::Matrix::new(output_size, board); // Assuming Matrix::new takes size and initial data
//             let display = MatrixDisplay::new(format, data); 
//             display.print(&mut std::io::stdout(), &style::BordersStyle::Thin);
//             println!("=========MATRIX AFTER MAX POOL STEP===========");

//             let t = time::Duration::from_millis(3000);
//             thread::sleep(t);
//         }
//     }
//     output_matrix
// }



// fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Array2<f64>, Box<dyn Error>> {
//     // Open the file
//     let file = File::open(&filename)?;
    
//     // Create a CSV reader
//     let mut rdr = ReaderBuilder::new().from_reader(file);

//     // Initialize a vector to hold the rows
//     let mut rows = Vec::new();

//     // Read the CSV records
//     for result in rdr.records() {
//         let record = result?;
        
//         // Parse each field in the record to f64 and collect into a Vec<f64>
//         let row: Result<Vec<f64>, _> = record.iter().map(|field| field.parse::<f64>()).collect();
//         println!("Parsed row: {:?}", row);
//         // Add the parsed row to the rows vector
//         rows.push(row?);
//     }
//     println!("Input matrix shape: {:?}", rows);

//     // Assume all rows have the same length
//     let num_cols = rows.get(0).map_or(0, |row| row.len());
//     println!("Input matrix shape: {:?}", num_cols);
    
//     // Flatten the Vec<Vec<f64>> into a single Vec<f64>
//     let flat_vec: Vec<f64> = rows.into_iter().flatten().collect();
    
//     // Calculate the number of rows
//     let num_rows = flat_vec.len() / num_cols;
//     println!("Input matrix shape: {:?}", flat_vec.len());
    
//     // Convert the flat Vec<f64> to Array2<f64>
//     let matrix = Array2::from_shape_vec((num_rows, num_cols), flat_vec)?;

//     Ok(matrix)
// }

// fn main() {
//     // let filename = "random_matrix.csv";
//     // read_csv(filename);
//     // let input_shape = (64, 64);
//     // let filter_shape = (5, 5);

//     // let input_matrix = generate_random_matrix(input_shape);
//     // save_matrix_to_csv(&input_matrix, "random_matrix.csv");
//     // let filter_matrix = generate_random_matrix(filter_shape);

//     let input_matrix = read_csv("../input_matrix.csv").unwrap();
//     let filter_matrix = read_csv("../filter_matrix.csv").unwrap();
//     println!("Input matrix shape: {:?}", input_matrix.shape());
//     println!("Filter matrix shape: {:?}", filter_matrix.shape());

//     let conv_output = convolution(&input_matrix, &filter_matrix, 1, 1);
//     // println!("Convolution Output:\n{:?}", conv_output);

//     let relu_output = relu(&conv_output);
//     // println!("ReLU Output:\n{:?}", relu_output);

//     let pool_output = max_pooling(&relu_output, 2, 2);
//     println!("Max Pooling Output:\n{:?}", pool_output);
// }

// use plotters::prelude::*;
// use std::error::Error;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::Path;

// fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
//     let file = File::open(file_path)?;
//     let mut rdr = csv::Reader::from_reader(BufReader::new(file));

//     let mut col1 = Vec::new();
//     let mut col2 = Vec::new();

//     for result in rdr.records() {
//         let record = result?;
//         col1.push(record[0].parse::<f64>()?);
//         col2.push(record[1].parse::<f64>()?);
//     }

//     Ok((col1, col2))
// }

// fn mean(values: &[f64]) -> f64 {
//     values.iter().copied().sum::<f64>() / values.len() as f64
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let file_paths = vec![
//         "../python_implementation/weak_scaling_data/sequential_256.csv",
//         "../python_implementation/weak_scaling_data/sequential_512.csv",
//         "../python_implementation/weak_scaling_data/sequential_768.csv",
//         "../python_implementation/weak_scaling_data/sequential_1024.csv",
//         "../python_implementation/weak_scaling_data/sequential_1280.csv",
//         "../python_implementation/weak_scaling_data/sequential_1536.csv",
//     ];

//     let mut means_seq = Vec::new();
//     let mut means_ts = Vec::new();
//     let mut means_par = Vec::new();
//     let mut means_n_process = Vec::new();
//     let mut speedup = Vec::new();
//     let mut theory = Vec::new();

//     for file_path in file_paths {
//         let (col1, col2) = read_csv(file_path)?;
//         means_seq.push(mean(&col1));
//         means_ts.push(mean(&col2));
//     }

//     let file_paths_p = vec![
//         "../python_implementation/weak_scaling_data/parallel_1.csv",
//         "../python_implementation/weak_scaling_data/parallel_512.csv",
//         "../python_implementation/weak_scaling_data/parallel_768.csv",
//         "../python_implementation/weak_scaling_data/parallel_1024.csv",
//         "../python_implementation/weak_scaling_data/parallel_1280.csv",
//         "../python_implementation/weak_scaling_data/parallel_1536.csv",
//     ];

//     for file_path in file_paths_p {
//         let (col1, col2) = read_csv(file_path)?;
//         // means_seq.push(mean(&col1));
//         means_par.push(mean(&col1));
//         means_n_process.push(mean(&col2));
//     }

//     for i in 0..means_seq.len() {
//         speedup.push(means_seq[i] / means_par[i]);
//     }

//     for i in 0..means_seq.len() {
        
//         if i==means_seq.len(){
//             println!("ovde");
//             theory.push(50.0);
//         } else {
//             theory.push(means_ts[i]/means_seq[i] + (1.0-means_ts[i]/means_seq[i])*means_n_process[i]);
//         }
//         println!("{}", means_ts[i]/means_seq[i] + (1.0-means_ts[i]/means_seq[i])*means_n_process[i]);
//     }

//     let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;

//     // let x_range = *speedup.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().min(
//     //     speedup.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
//     // )..*theory.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().max(
//     //     theory.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
//     // );

//     let y_range = 0.0..40.0;

//     let x_range =0.0..40.0;

//     let mut chart = ChartBuilder::on(&root)
//         .caption("Mean Values Plot", ("sans-serif", 50).into_font())
//         .margin(20)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(x_range, y_range)?;

//     chart
//         .configure_mesh()
//         .x_desc("Speedup (Sequential/Parallel)")
//         .y_desc("Mean of Number of Processes")
//         .draw()?;

//     chart.draw_series(LineSeries::new(
//         means_n_process.iter().zip(speedup.iter()).map(|(x, y)| (*x, *y)),
//         &BLUE,
//     ))?
//     .label("Speedup")
//     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

//     chart.draw_series(LineSeries::new(
//         means_n_process.iter().zip(theory.iter()).map(|(x, y)| (*x, *y)),
//         &RED,
//     ))?
//     .label("Additional Speedup")
//     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//     chart.configure_series_labels().draw()?;

//     root.present()?;
//     Ok(())
// }

