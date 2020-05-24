

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::io::Read;
use bloom::{BloomFilter};

use std::convert::TryInto;


pub struct LoadThePassword {
    filter: BloomFilter
}


impl LoadThePassword {


pub fn initialize (file_path: &mut String) -> LoadThePassword{ 
    
    println!("File , {}!", file_path);
    if file_path.is_empty(){
        *file_path = String::from("./passwordlist/password.txt");
    }

    //Reads the content of the file

    let mut file = std::fs::File::open(file_path).unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
});

    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Maps the content of the file to an array
    let _lines: Vec<String> = contents.split("\n")
    .map(|s: &str| s.to_string())
    .collect();


    // Initialising the parameters of the strcut
       
        let _expected_num_items = _lines.len();
        let _false_positive_rate = 0.01;

   // Adding the values into BloomFilter

    
      let mut _filter =  BloomFilter::with_rate(_false_positive_rate,_expected_num_items.try_into().unwrap());

 
      for _password in &_lines {
        _filter.insert(&_password);
    }

  return  LoadThePassword{
        filter : _filter
    };



}



pub fn  check (&self,input_password: &str) -> bool {

    self.filter.contains(&input_password)

}

}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn check_password(_password: &str, filepath: &str){
    log(filepath);
    // let function_handler = LoadThePassword::initialize(& mut String::from(filepath));
    // let result_check = function_handler.check(password);
    // result_check
}









