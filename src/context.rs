

use crate::{error::ServerError, models::Item};

#[derive(Debug, Clone)]
pub struct Context
{
    name: String,
    balance: i32,
    items: Vec<(Item,i32)>,
}

impl Context
{
    pub fn new(name: String, balance: i32, items: Vec<(Item,i32)>) -> Self
    {
        Self
        {
            name,
            balance,
            items,
        }
    }
}

impl Context
{
    pub fn get_name(&self) -> &str
    {
        &self.name
    }

    pub fn get_balance(&self) -> &i32
    {
        &self.balance
    }

    pub fn get_items(&self) -> Vec<(Item,i32)>
    {
        self.items.clone()
    }

    pub fn add_item(&mut self, item: Item, num: i32)
    {
        // Check if the item is already in the list if so increment the number else add it to the list
        let mut found = false;
        for i in self.items.iter_mut()
        {
            if i.0 == item
            {
                println!(">>> FOUND ITEM\n");
                i.1 += num;
                found = true;
                break;
            }
        }
        if !found
        {
            println!(">>> ITEM NOT FOUND\n");
            self.items.push((item,num));
        }
    }

    pub fn remove_item(&mut self, item: Item, num: i32) -> Result<i32, ServerError>
    {
        // Check if the item is already in the list if so decrement the number else remove it from the list
        let mut found = false;
        for i in self.items.iter_mut()
        {
            if i.0 == item
            {
                println!(">>> FOUND ITEM\n");
                let ret;
                if i.1 - num < 0
                {
                    ret = i.1;
                }
                else 
                {
                    ret = num;    
                }
                i.1 -= num;
                if i.1 <= 0
                {
                    println!(">>> REMOVING ITEM\n");
                    self.items.retain(|x| x.0 != item);
                    return Ok(ret);
                }
                found = true;
                break;
            }
        }
        if !found
        {
            println!(">>> ITEM NOT FOUND\n");
            return Err(ServerError::OtherError);
        }
        else {
            Ok(0)
        }
    }

}

