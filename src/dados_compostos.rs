/*
Tipos de dados compostos

- &str e String
- Arrays
- Vetores
- Tuplas
- Tupla vazia

 */

pub fn compound_data_types () {
    // Strings
    // &str , String
    let string_fixed_length: &str = "lol";
    let mut string_flexible_length: String = String::from("Lol");
    string_flexible_length.push('s');

    println!("{string_flexible_length}");
}