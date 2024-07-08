
use crate::models::Item;

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
            self.items.push((item,1));
        }
    }

}

