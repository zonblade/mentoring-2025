use super::BusinessLogic;

// local scoped struct
pub struct User {
    pub id: i32
}

pub trait Users {
    fn get_users()->Vec<User>;
}

impl Users for BusinessLogic {
    fn get_users()->Vec<User> {
      vec![
        User{
            id:1
        }
      ]  
    }
}