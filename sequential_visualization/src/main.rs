use std::error::Error;
use std::fs::File;
use std::path::Path;
extern crate matrix_display;
use matrix_display::*;
use std::thread;
use std::{time};

use csv::{ReaderBuilder};
use ndarray::{Array2};

fn main(){
    let stride = 2;
    let padding = 0;
    let pool_size = 2;
    let pool_stride = 2;
    print!("{}[2J", 27 as char);
    // START
    for _ in 0..1 {
        print!("{}[2J", 27 as char);
        println!("==============INPUT MATRIX ON START================");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/input/input_matrix.csv").unwrap();
        display_normal(&matrix, 8, -1, stride);

        println!("==============FILTER MATRIX ON START================");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/input/filter_matrix.csv").unwrap();
        display_normal(&matrix, 2, -1, stride);

        let t = time::Duration::from_millis(500);
        thread::sleep(t);
    }

    // CONVOLUTION
    for i in 0..15{
        print!("{}[2J", 27 as char);
        println!("==============INPUT MATRIX=====================");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/input/input_matrix.csv").unwrap();
        display_normal(&matrix, 8, i, stride);

        println!("==============FILTER MATRIX================");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/input/filter_matrix.csv").unwrap();
        display_normal(&matrix, 2, -1, stride);

        println!("==============CONVOLUTION STEP {}================", i);
        let filename = format!("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/convolution/conv_{}.csv", i);
        let matrix = read_csv(&filename).unwrap();
        display_with_exact_step(&matrix, i.try_into().unwrap(), 4);

        let t = time::Duration::from_millis(500);
        thread::sleep(t);
    }

    // RELU
    for i in 0..1{ 
        print!("{}[2J", 27 as char);
        println!("==============CONVOLUTION RESULT================");
        let filename = format!("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/convolution/conv_{}.csv", 15);
        let matrix = read_csv(&filename).unwrap();
        display_normal(&matrix, 4, -1, stride);

        println!("=========RELU RESULT=========");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/relu/relu.csv").unwrap();
        display_normal(&matrix, 4, -1, stride);
        
        let t = time::Duration::from_millis(500);
        thread::sleep(t);
    }

    // POOLING
    for i in 0..4{ 
        print!("{}[2J", 27 as char);
        println!("=========MATRIX AFTER RELU=========");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/relu/relu.csv").unwrap();
        display_normal(&matrix, 4, i, pool_stride);

        println!("=========POOLING STEP=========");
        let filename = format!("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/pooling/pool_{}.csv", i);
        let matrix = read_csv(&filename).unwrap();
        display_with_exact_step(&matrix, i.try_into().unwrap(), 2);
        
        let t = time::Duration::from_millis(500);
        thread::sleep(t);
    }

    // RESULTS
    for _ in 0..1{ 
        print!("{}[2J", 27 as char);
        println!("==============START================");
        let matrix = read_csv("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/input/input_matrix.csv").unwrap();
        display_normal(&matrix, 8, -1, stride);
        
        println!("=========END=========");
        let filename = format!("../python_implementation/seq_visualization_i8_f2_s2_p0_ps2_pstr2/pooling/pool_{}.csv", 3);
        let matrix = read_csv(&filename).unwrap();
        display_normal(&matrix, 2, -1, stride);
        
        let t = time::Duration::from_millis(500);
        thread::sleep(t);
    }
}

fn display_normal(matrix: &Array2<f64>, size:usize, step: i32, stride: i32){
    let format = Format::new(5,5);
    let board = matrix.clone().into_raw_vec().iter()
    .enumerate()
    .map(|(i, &x)| {  
        let mut ansi_fg = 0;
        let mut ansi_bg = 70;
        if step != -1 {
            if i == (step*stride+(size as i32)*(2 * step/(size as i32))).try_into().unwrap()
             || i == (step*stride+(size as i32)*(2 * step/(size as i32)) + 1).try_into().unwrap()
             || i == (step*stride+(size as i32)*(2 * step/(size as i32)) + (size as i32) ).try_into().unwrap()
             || i == (step*stride+(size as i32)*(2 * step/(size as i32)) + (size as i32) +1).try_into().unwrap()  {
                ansi_bg = 7;
                ansi_fg = 33;
            }
        }
        cell::Cell::new((x * 100.0).round() / 100.0, ansi_fg, ansi_bg)
    })
    .collect::<Vec<_>>();
    let data = matrix::Matrix::new(size, board);
    let display = MatrixDisplay::new(format, data); 
    display.print(&mut std::io::stdout(), &style::BordersStyle::Thin);
}

fn display_with_exact_step(matrix: &Array2<f64>, step:i32, size:usize){
    let format = Format::new(5,5);
    let board = matrix.clone().into_raw_vec().iter()
    .enumerate()
    .map(|(i, &x)| {  
        let mut ansi_fg = 0;
        let mut ansi_bg = 70; 
        if i == step.try_into().unwrap(){
            ansi_bg = 7;
            ansi_fg = 33;
        }
        cell::Cell::new((x * 100.0).round() / 100.0, ansi_fg, ansi_bg)
    })
    .collect::<Vec<_>>();
    let data = matrix::Matrix::new(size, board);
    let display = MatrixDisplay::new(format, data); 
    let mut output = Vec::new();
    display
        .print(&mut output, &style::BordersStyle::Thin);
    let t = String::from_utf8(output).unwrap();
    print!("{}", t);
}

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<Array2<f64>, Box<dyn Error>> {
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
