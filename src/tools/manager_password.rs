
use bcrypt::{hash, verify};



pub fn create_hash_password(password: String) -> String {

    let senha = password;
    let hash = hash(senha, 10).unwrap();
     
    hash
}


pub fn velidate_hash_password(password: String, hash: String) -> bool {

    let password = password;
    let verify_pw = verify(outra_senha, &hash).unwrap();

    if verify_pw { true }
    else { false }
}


pub fn verify_password(password: String, hash: String) {

    let hash_in_database = password;
    let password_user = hash;

    match verify(senha_usuario, hash_armazenado) {
        Ok(true) => println!("A senha está correta!"),
        Ok(false) => println!("A senha está incorreta!"),
        Err(_) => println!("Erro ao verificar a senha."),
    }
}
