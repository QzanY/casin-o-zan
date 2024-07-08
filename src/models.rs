use std::str::FromStr;

use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone,Serialize,Deserialize,Eq,PartialEq,Hash)]
pub enum Item
{
    None,
    Car,
    House,
    Boat,
    Plane,
    Bike,
    Motorcycle,
    Computer,
    Phone,
}


impl FromStr for Item
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "Car" => Ok(Item::Car),
            "House" => Ok(Item::House),
            "Boat" => Ok(Item::Boat),
            "Plane" => Ok(Item::Plane),
            "Bike" => Ok(Item::Bike),
            "Motorcycle" => Ok(Item::Motorcycle),
            "Computer" => Ok(Item::Computer),
            "Phone" => Ok(Item::Phone),
            _ => Err(()),
        }
    }
}

impl ToString for Item
{
    fn to_string(&self) -> String
    {
        match self
        {
            Item::None => "None".to_string(),
            Item::Car => "Car".to_string(),
            Item::House => "House".to_string(),
            Item::Boat => "Boat".to_string(),
            Item::Plane => "Plane".to_string(),
            Item::Bike => "Bike".to_string(),
            Item::Motorcycle => "Motorcycle".to_string(),
            Item::Computer => "Computer".to_string(),
            Item::Phone => "Phone".to_string(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ItemRequest
{
    pub item: Item,
    pub number: i32,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct User {
    pub name: String,
    pub password: String,
    pub balance: i32,
    pub items: Vec<(Item,i32)>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserRequest
{
    pub name: String,
    pub password: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Token
{
    pub name: String,
    pub time: i64,
}
