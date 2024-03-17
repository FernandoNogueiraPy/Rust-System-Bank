//Desafio - Create a program for manager payments of bank accounts

mod entitys {pub mod entity_client; pub mod register_client;}
mod functions { pub mod collector_dates; }

use crate::entitys::register_client::RegisterClient;
use crate::functions::collector_dates::colletor_info_register;
use std::io;









//Method Enter Account 
fn enter_account(password:String,login:String){




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


