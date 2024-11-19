fn type_of<T: std::fmt::Debug>(s: &str, _: &T) -> &'static str {
    // Get the type name of the generic parameter T and print it along with the string s
    let tn = std::any::type_name::<T>();
    println!("{s} has type of {tn}");
    tn
}

fn option_type<T: std::fmt::Display>(opt: &Option<T>) {
    // Match on the Option to print the type name and value if Some, or "None" if None
    match opt {
        Some(v) => {
            let tn = std::any::type_name::<T>();
            println!("{tn} has value of <{v}>");
        }
        None => println!("None"),
    }
}

fn main() {
    println!("Template Project in Codespaces");
    let s = "Some day...";
    let s2 = type_of(s, &s);
    println!("{s2}");

    let opt = Some(35.09f64);
    option_type(&opt);
}
