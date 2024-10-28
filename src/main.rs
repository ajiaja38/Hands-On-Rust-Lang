fn main() {
    println!("Hello world!");
}

#[test]
fn hello_test() {
    println!("Hello test!");
} 

#[test]
fn test_variable() {
    let name: &str = "M. Aji Perdana";
    
    println!("Hello, {}", name)
}

#[test]
fn test_variable_mutable() {
    let mut name: &str = "Erlin Novasia";
    println!("Hello, {}", name);

    name = "Eyyyinnn Nopasiaaaaa 😹";
    println!("Hello, {}", name)
}

#[test]
fn test_static_typing() {
    let name: &str = "Hello Static type";
    println!("Hello, {}", name);

    // name = 32;
    println!("Hello, {}", name)
}

#[test]
fn shadowin() {
    let var: &str = "M. Aji Perdana";
    println!("Hello, {}", var);

    let var: i32 = 10;
    println!("Ini adalah angka: {}", var)
}