fn main() 
{
    println!("Hello, world!");
    println!("Ando aprendiendo Rust");
    println!("Ta god");
    println!("Hello World!\nThis line was broken up!");

    let nombre= "Josue";
    let edad = 20;
    println!("Mi nombre es {} y mi edad es de {}", nombre, edad);

    let mut x = 5;
    println!("x antes = {}", x);
    x = 10;
    println!("x despues = {}", x);

    // Declaracion de datos
    let my_num: i32 = 5;          // integer
    let my_unum: u32 = 10;         // unsigned integer (No permite negativos)
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // string
    println!("my_num = {}, my_unum = {}, my_double = {}, my_letter = {}, my_bool = {}, my_text = {}", my_num, my_unum, my_double, my_letter, my_bool, my_text);

}
//comentario

/* comentario 
Alargado xd
waos */