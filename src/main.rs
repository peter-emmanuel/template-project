fn type_of<T: std::fmt::Debug>(s: &str, _:&T) -> &'static str {
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
        },
        None => println!("None"),
    }
}

fn average(numbers: &[i32]) -> f64 {
    // Sum the numbers and divide by the length to get the average
    let total = numbers.iter().sum::<i32>() as f64; // Sum all elements in the array
    total / numbers.len() as f64 // Divide the total by the number of elements to get the average
}

fn main() {
    println!("Template Project in Codespaces");
    let s = "Some day...";
    let s2 = type_of(s, &s);
    println!("{s2}");

    let opt = Some(35.09f64);
    option_type(&opt);

    let numbers = [1, 2, 3, 4, 5];
    let avg = average(&numbers);

    // Print the list of numbers and their average
    println!("Numbers in list are {:?}", numbers);
    println!("Average numbers from list is {avg}");
}

