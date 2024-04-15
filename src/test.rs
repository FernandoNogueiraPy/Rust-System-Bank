//Desafio - Create a program for manager payments of bank accounts

mod entitys {pub mod entity_client; pub mod register_client;}
mod functions { pub mod collector_dates; }
mod tools { pub mod manager_password; }

use crate::entitys::register_client::RegisterClient;
use crate::tools::manager_password::{_velidate_hash_password,_verify_password,_create_hash_password};
use crate::functions::collector_dates::colletor_info_register;
use std::io;









//Method Enter Account 
fn enter_account(){

    println!("Your login:");
    let mut login:String = String::new();
    io::stdin().read_line(&mut login).expect("Falha ao ler a entrada"); 

    println!("Your password: ");
    let mut password:String = String::new();
    io::stdin().read_line(&mut password).expect("Falha ao ler a entrada"); 

    let hash: String = _create_hash_password(password.clone());
    let verify: bool = _velidate_hash_password(password,hash);
    if verify {println!("Login efetuado com sucesso");}
    else {println!("Senha ou login incorretos");}


}

// Method create account
fn create_account() {

    let (name, cpf_register, password, agency) = colletor_info_register();
    let teste: RegisterClient = RegisterClient::new(
        
    cpf_register,
    name,
    password,
agency
    
    );
    println!("====================================================================");
    println!("Registro efetuado com sucesso ! \n
    Esses são seus dados:\n
    
    ID: {}
    Name: {}
    CPF: {}
    ID Bank: {}
    Password: {}
    Number Account: {}        
    
    ",teste.id,teste.name,teste.cpf,teste.id_agent_bank,teste.password,teste.number_account);
    println!("====================================================================");
}




fn main() {

    println!("\n\n\n\n\n");
    println!("------ > Welcome to the bank account manager <-------");
    println!("choice a service in system ");
    
    println!("1 - Create account");
    println!("2 - Enter your account");



    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada"); 

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,Err(_) => {println!("Por favor, insira um número válido."); return; }};


    if number == 1 {create_account();}
    else if number == 2 {println!("Enter your account");}


    

}


