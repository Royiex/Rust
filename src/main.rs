use std::path::Path;
use std::io::stdin;

use image::GenericImageView;


fn info(input_path: String) {

    println!("{}", input_path);

    let img_path = Path::new(&input_path);


    let img = image::open(img_path).unwrap();


    let img_s1 = img.dimensions().0;
    let img_s2 = img.dimensions().1;

    println!("your images x dimension is: {}", img_s1);
    println!("your images y dimension is: {}", img_s2);
}


fn main() {
    let mut input_path = String::new();

    println!("Enter image path: ");
    stdin().read_line(&mut input_path);

    println!("your image path is: {}", input_path);

    println!("---------------------------------------");

    info(input_path);

}