#![allow(unused_variables, dead_code, unused_mut, unused_imports)] 

extern crate num_bigint;
mod helper;
mod dispatcher;
mod model;

use crate::helper::tritvector::TritVector;
use crate::helper::tritconverter::TritConverter;
use crate::model::Model;

fn main() {
    let model = Model::new();
    println!("{:?}", model.current_quant); 

    let trit_vector = TritVector::from(10);
    println!("{:?}", &trit_vector); 

    let trit_vector2 = TritVector::from(vec![-1, 1, 0, 1, 0, 0, -1, 0, 1, -1, 0]);

    let trit_vector3 = TritVector::from(&trit_vector2);

    // println!("Tritvector3: {:?}", &trit_vector3);
    // println!("Tritvector2: {:?}", &trit_vector2);

    let result = TritConverter::from_decimal(String::from("30"));
    println!("{}", result);

    // TritConverter::get_power(10);

    println!("{}", TritConverter::from_float(String::from("30.5123"), 4, 4));
}
