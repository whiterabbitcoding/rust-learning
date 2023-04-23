use std::cmp::Ordering;
extern crate store;



fn main() {
    store::print_blogs(store::blogs);
    let address_1 = Address { 
        address: 1234,
        currency: "Alex's currency",
        amount: 7,
    };

    let address_2 = Address { 
        address: 1222,
        currency: "Alex's currency",
        amount: 50,
    };
    get_address_amount(address_1);
    address_has_enough_for_transaction(address_2, 4);
}

struct Address<'a> {
    address:i32,
    currency: &'a str,
    amount: i32,
}

fn get_address_amount(address: Address) -> i32 {
    println!("{}", address.amount);
    address.amount
}

fn address_has_enough_for_transaction(
    address: Address, 
    transaction_amount: i32
    ) {
    let mut has_enough = false;

    println!("transaction amount is {}", transaction_amount);
    match address.amount.cmp(&transaction_amount){
        Ordering::Less      => println!("address does not have enough for the transaction, it has {}", address.amount),
        Ordering::Equal   => {
            println!("address has enough for the transaction!");
            has_enough = true;
        },
        Ordering::Greater   => {
            println!("address has enough for the transaction!");
            has_enough = true;
        },
    }

    println!("{}", has_enough);
    
}

    
