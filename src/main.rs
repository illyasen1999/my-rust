// mod filecreation;
// mod miniapps;
mod chap3;

fn main() {
    let my_name = String::from("Illyasen");
    println!("Hello, {}!", my_name);
    let some_string = something(String::from("This is some string idk"));
    println!("{}", some_string);

    // let _created_file = filecreation::filecreate::create_text();
    // let _read_file = filecreation::filecreate::read_text();

    chap3::progconcepts::concepts();
    // miniapps::clock::clock_fn();
}


fn something(my_string: String) -> String {
    String::from(my_string)
}



