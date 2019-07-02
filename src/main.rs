mod helper;
use helper::tritvector::TritVector;
use helper::tritconverter::TritConverter;


fn main() {
    let trit_vector = TritVector::from(10);
    println!("{:?}", &trit_vector); 

    let trit_vector2 = TritVector::from(vec![-1, 1, 0, 1, 0, 0, -1, 0, 1, -1, 0]);

    let trit_vector3 = TritVector::from(&trit_vector2);

    println!("Tritvector3: {:?}", &trit_vector3);
    println!("Tritvector2: {:?}", &trit_vector2);

    let result = TritConverter::from_decimal(String::from("-12378945"));
    println!("{}", result);
}
