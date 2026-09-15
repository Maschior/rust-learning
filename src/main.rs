fn main () {
    let name = "Matheus";

    // ! = means that this function is a macro, which is a special kind of function that 
    // can take a variable number of arguments and can generate code at compile time. 
    println!("Hello World from 12-09-2026, {name}!\n");

    println!("01: Variaveis e Tipos");
    
    println!("i32: 4 bytes, -2,147,483,648 to 2,147,483,647");
    let x: i32 = -5;
    println!("x: {}", x);

    println!("u32: 4 bytes, 0 to 4,294,967,295");
    let y: u32 = 10;
    println!("y: {}", y);

    println!("f64: 8 bytes, -1.7976931348623157e+308 to 1.7976931348623157e+308");
    let z: f64 = 3.14333;
    println!("z: {}", z);

    println!("No rust as variáveis são imutáveis por padrão, ou seja, não podem ser alteradas 
            depois de serem declaradas. Para declarar uma variável mutável, é necessário usar 
            a palavra-chave 'mut'.");

    let mut x = 10;
    println!("x: {}", x);
    x = 20;
    println!("x: {}", x);


    let nome: &str = "Matheus";
    println!("nome: {}", nome);
    {
        // Reassigning a new value to the variable 'nome' in this inner scope, but out of this scope, the variable 'nome' will still have the value "Matheus".
        let nome = "Oliver";
        println!("nome: {}", nome);
        // Shadowing is a feature in Rust that allows you to declare a new variable with the same name as a previous variable, 
        // effectively "shadowing" the previous variable. This can be useful for reusing variable names in different scopes or for transforming the value of a 
        // variable without mutating it.
        let x = x + 1;
        println!("x inside inner scope (): {}", x);
    }   
    println!("x after inner scope: {}", x);
    println!("nome: {}", nome);
    
    println!("\nShadowing examples: \n");
    let name2 = "Matheus";
    let name2 = "Oliver";
    // Compiler will acuse name2 for not being used, but this means that the first value assigned to anime2 is not being used
    println!("name2: {}", name2);

    println!("\nConstantes");
    // type inferece will not work for constants, so you need to specify the type of the constant.
    // const MAX_POINTS = 100_000; won't work, because the compiler will not know what type to assign to MAX_POINTS.
    const PI: f64 = std::f64::consts::PI;
    println!("PI: {}", PI);

    println!("\nChars");
    let c: char = 'z';
    println!("c: {}", c);

    println!("\nTuplas");
    let tupla: (i32, &str, f64) = (5, "Hello", 3.14);
    println!("tupla: {:?}", tupla);

    println!("\nArrays");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {array:?}"); 
    
    println!("\nSlices");
    let slice: &[i32] = &array[1..5];
    println!("slice: {:?}", slice);

    println!("\nStrings");
    let string: String = String::from("Hello");
    println!("string: {}", string);

    println!("\nString Slices");
    let string_slice: &str = &string[0..2];
    println!("string_slice: {}", string_slice);

    println!("Teste de consumo de memoria com Arrays");


    let mut array: Vec<i32> = vec![0; 2_147_483_648];

    println!("Alocado.");

    // Força cada página de memória a ser realmente tocada
    for i in (0..array.len()).step_by(1024) {
        array[i] = 1;
    }

    println!("Memória tocada.");
    println!("Elementos: {}", array.len());

    std::io::stdin().read_line(&mut String::new()).unwrap();
}