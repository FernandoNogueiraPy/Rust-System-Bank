use std::io;



pub fn colletor_info_register() -> (String,String,String,i32) {

    println!("====================================================================");
    println!("Welcome to the bank of the future !\n");

    println!("Digit your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Falha ao ler a entrada");

    println!("Digit your CPF: ");
    let mut cpf_register = String::new();
    io::stdin().read_line(&mut cpf_register).expect("Falha ao ler a entrada");

    println!("Create password for your account: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Falha ao ler a entrada");

    println!("Digit of number agency of your choice: ");
    let mut agency: String = String::new();
    io::stdin().read_line(&mut agency).expect("Falha ao ler a entrada");
    let agency: i32 = agency.trim().parse().expect("Falha ao converter string para inteiro");

    println!("====================================================================");
  

    (name,cpf_register,password,agency)
}