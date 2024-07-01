use ndarray::Array2;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::Writer;

pub fn save_matrix(matrix: &Array2<f64>, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = Writer::from_writer(file);

    for row in matrix.outer_iter() {
        let row_vec: Vec<String> = row.iter().map(|&x| x.to_string()).collect();
        wtr.write_record(&row_vec)?;
    }

    wtr.flush()?;
    Ok(())
}