
use bcrypt::{hash, verify};



pub fn _create_hash_password(password: String) -> String {

    let senha: String = password;
    let hash: String = hash(senha, 10).unwrap();
     
    hash
}

pub fn _velidate_hash_password(password: String, hash: String) -> bool {

    let password: String = password;
    let verify_pw: bool = verify(password, &hash).unwrap();

    if verify_pw { true }
    else { false }
}

pub fn _verify_password(password: String, hash: &str) {

    let hash_in_database: &str = hash;
    let password_user: String = password;

    match verify(password_user, hash_in_database) {
        Ok(true) => println!("A senha está correta!"),
        Ok(false) => println!("A senha está incorreta!"),
        Err(_) => println!("Erro ao verificar a senha."),
    }
}
