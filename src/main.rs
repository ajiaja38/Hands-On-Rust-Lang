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