use serde::Serialize;
#[derive(Queryable, Serialize)]
pub struct Wallet {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub parties: String,
    pub threshold: u32,
    pub balance: i32
}