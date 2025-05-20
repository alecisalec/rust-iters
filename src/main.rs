
fn print_elements(el : &Vec<String>){
    // for loop

    // for i in el {
    //     println!("{:#?}", i);
    // }

    // using iter
    el.iter()
    .map(|el| format!("{}, {}", el, el)) // adapter
    .for_each(|el| println!("color is: {}", el));  // consumer

}

fn print_elements_slice(el : &[String]){
    // using a vector slice
    // using iter
    el.iter()
    .map(|el| format!("{}, {}", el, el)) // adapter
    .for_each(|el| println!("color is: {}", el));  // consumer

}




fn main() {
    let colors = vec![ String::from("red"), String::from("blue"),String::from("green")];


    //let mut colors_iter = colors.iter(); 

    print_elements(&colors);
    print_elements_slice(&colors[1..3]); // slice
    print_elements_slice(&colors); // still full vector


}
