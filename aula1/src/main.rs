fn main() {
    let resultado = str_invert("Data structure");
    println!("{}", resultado); 
}

fn str_invert(entrada: &str) -> String {
    return entrada.chars().rev().collect();
}
