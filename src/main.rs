/* Example of a simple program. 
fn main() {
    let name = "Felipe"; // String literal
    let mut age = 28; // Integer literal with mutation
    let is_active = true; // Boolean literal
    age = 20;
    println!("Hello {}! {} anos, está {}", name, age, is_active); // <- 
}
*/
/* Exemplos de tipos de dados

    integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (número de bits que ocupa na memória)
        - Se colocar o u significa unsigned (não negativo)
        - Se colocar o i significa signed (negativo)
        - O número que segue é os bits
        ex: let x: u64 = 23;
    float: f32, f64
        ex: let f: f64 = 2.5;
    boolean: (bool)
        ex: let is_active: bool = true;

*/
fn main() {
    println!("Hello World!");
}
