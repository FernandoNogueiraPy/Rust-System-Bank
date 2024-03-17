
use uuid::Uuid;



impl RegisterClient{ 

    // Intancia os dados básicos para o cadastro
    // Instance the date basic for register

    pub fn new(cpf:String,name:String,password:String,id_bank:i32) -> Self{ 
        
        RegisterClient {

            id:Uuid::new_v4(),
            name:name,
            id_agent_bank:id_bank,
            cpf:cpf,
            password:password,
            number_account:1

        }
    
    }

    //fn register_account_in_database(&self){}

}





#[derive(Debug)]
pub struct RegisterClient{

    pub id: Uuid,
    pub name: String,
    pub id_agent_bank: i32,
    pub cpf: String,
    pub password: String,
    pub number_account: i32,

}

