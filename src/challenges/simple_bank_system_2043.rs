use std::collections::HashMap;

pub fn run() {
    let mut obj = Bank::new(vec![0]);

    println!("Bank: {:#?}", obj);

    let ret_1: bool = obj.transfer(5, 1, 20);

    println!("ret_1 : {}", ret_1);
    println!("Bank: {:#?}", obj);

    let ret_2 = obj.withdraw(37, 377);
    println!("withdraw  ===  ret_2 : {}", ret_2);

    let ret_3 = obj.deposit(1, 1000000000000);
    println!("deposit ==  ret_3 : {}", ret_3);
    println!("Bank: {:#?}", obj);

    let ret_3 = obj.transfer(1, 1, 1000000000000);

    println!("transfer  ===  ret_3 : {}", ret_3);
    println!("Bank: {:#?}", obj);

    let ret_3 = obj.withdraw(1, 1000000000000);

    println!("withdraw  ===  ret_3 : {}", ret_3);
    println!("Bank: {:#?}", obj);
}
#[derive(Debug)]
struct Bank {
    accounts: HashMap<i32, i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        let mut i = 1;
        let mut accounts = HashMap::new();

        for item in balance {
            accounts.insert(i, item);
            i += 1;
        }

        Self { accounts }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.accounts.contains_key(&account1) || !self.accounts.contains_key(&account2) {
            return false;
        }

        let from = self.accounts.get_mut(&account1).unwrap();
        if *from < money {
            return false;
        }
        *from -= money;

        let to = self.accounts.get_mut(&account2).unwrap();
        *to += money;

        return true;
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.accounts.contains_key(&account) {
            return false;
        }

        let source = self.accounts.get_mut(&account).unwrap();
        *source += money;

        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.accounts.contains_key(&account) {
            return false;
        }

        let source = self.accounts.get_mut(&account).unwrap();

        if *source >= money {
            *source -= money;
            return true;
        } else {
            false
        }
    }
}
