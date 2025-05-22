
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

fn to_uppercase(el : &[String]) -> Vec<String>{

    return el.iter()
        .map(|el| {
            el.to_uppercase()
        })
        .collect()
}

fn explode(el: &[String]) -> Vec<Vec<String>>{

    el.iter()
    .map(|el|{
         el.chars().map(|c| {
            c.to_string()
        }).collect()
    })
    .collect()

}


fn shorten_strings(el : &mut [String]){

    el.iter_mut().for_each(|el| {
        el.truncate(1);
    });

    println!("{:#?}", el)
}





fn main() {
    let colors = vec![ String::from("red"), String::from("blue"),String::from("green")];
    let mut colors2 = vec![ String::from("red"), String::from("blue"),String::from("green")];

    //let mut colors_iter = colors.iter(); 

    print_elements(&colors);
    print_elements_slice(&colors[1..3]); // slice
    print_elements_slice(&colors); // still full vector

    shorten_strings(&mut colors2);

    let upper_colors = to_uppercase(&colors);

    println!("{:#?}", upper_colors);

    println!("{:#?}", explode(&colors[1..colors.len()]));

}
