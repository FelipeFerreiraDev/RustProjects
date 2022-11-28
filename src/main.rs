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
/* Exemplo de if e else
fn main() {
    let number1 = 10;
    let number2 = 20;

    if number1>number2 {
        println!("{} é maior que {}", number1, number2);
    } else if number1<number2 {
        println!("{} é maior que {}", number2, number1);
    } else {
        println!("{} é igual a {}", number1, number2);
    }
}
*/

use std::io; // Importa a biblioteca padrão de entrada e saída

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap(); // trim remove os espaços em branco, parse converte para inteiro e unwrap retorna o valor de dentro do container
    x
}
fn main() {
    let mut number1 = String::new(); // Cria uma variável mutável do tipo String
    io::stdin().read_line(&mut number1).expect("Erro ao ler o número"); // Lê a entrada do usuário e armazena na variável number1
    
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler o número");  

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("{} é maior que {}", number1, number2);
    } else if convert_to_int(&number1) < convert_to_int(&number2) {
        println!("{} é maior que {}", number2, number1);
    } else {
        println!("{} é igual a {}", number1, number2);
    }
}