use std::collections::HashMap;

pub fn run() {
    let mut obj = Bank::new(vec![10, 100, 20, 50, 30]);

    println!("Bank: {:#?}", obj);

    let ret_1: bool = obj.transfer(5, 1, 20);

    println!("ret_1 : {}", ret_1);
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

    fn deposit(&self, account: i32, money: i64) -> bool {
        true
    }

    fn withdraw(&self, account: i32, money: i64) -> bool {
        true
    }
}
