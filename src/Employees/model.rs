// src/Employees/users.r
#[derive(Deserialize,Serialize)]
pub struct Users{
    pub id:i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}