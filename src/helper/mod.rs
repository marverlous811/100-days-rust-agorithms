use std::fs::read_to_string;

pub fn read_i32_input_from_file(file_path: &str) -> Vec<i32> {
  let result = read_to_string(file_path);
  match result {
    Ok(content) => content.split(',').map(|x| x.parse::<i32>().unwrap()).collect(),
    Err(err) => {
      println!("Error reading file: {}", err);
      vec![]
    }
  }
}
