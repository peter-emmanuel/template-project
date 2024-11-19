fn type_of<T: std::fmt::Debug>(s: &str, _:&T) -> &'static str {
    let tn = std::any::type_name::<T>();
    println!("{s} has type of {tn}");
    tn
}

fn option_type<T: std::fmt::Display>(opt: &Option<T>) {
    match opt {
        Some(v) => {
            let tn = std::any::type_name::<T>();
            println!("{tn} has value of <{v}>");
        },
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

