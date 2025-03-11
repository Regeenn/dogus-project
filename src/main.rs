

struct Bank {
     bank_name: String,
    bank_balance: u128,
    users: Vec<Users>,
}

impl Bank {
    pub fn new(bank_name: String, bank_balance: u128, users: Vec<Users>) -> Self {
        Self { bank_name, bank_balance, users }
    }

    pub fn get_bank_balance(&self) -> u128 {
        self.bank_balance
    }

    pub fn print_users(&self) {
        for user in &self.users {
            println!("Ad: {}, Soyadı: {}, ID: {}", user.name, user.surname, user.user_id);
            user.print_accounts(); 
        }
    }

    
}


struct Users {
    pub name: String,
    pub surname: String,
    pub user_id: String,
    accounts: Vec<Account>, 
}

impl Users {
    pub fn new(name: String, surname: String, user_id: String) -> Self {
        Self {
            name,
            surname,
            user_id,
            accounts: vec![], 
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn print_accounts(&self) {
        println!("{} {} kullanıcısının hesapları:", self.name, self.surname);
        for (i, account) in self.accounts.iter().enumerate() {
            println!("  {}. Hesap - Bakiye: {}", i + 1, account.get_balance());
        }
    }
}


struct Account {
    account_id: String,
    user_id: String, 
    balance: u32,
    
}

impl Account {
    fn new(user_id: String, balance: u32,account_id: String) -> Self {
        Self { user_id, balance, account_id }
    }

    pub fn get_balance(&self) -> u32 {
        self.balance
    }

    pub fn take_money(&mut self, amount: u32) -> Result<u32, String> {
        if amount > self.balance {
            Err(String::from("Yetersiz bakiye!"))
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }
    pub fn add_money(&mut self, amount: u32) -> u32 {
        self.balance += amount;
        self.balance
    }
    pub fn transfer_money(&mut self, amount: u32, receiver: &mut Account) -> Result<(), String> {
        if amount > self.balance {
            return Err("Yetersiz bakiye!".to_string());
        }
        self.balance -= amount; 
        receiver.balance += amount; 

        Ok(())
    }
   
    
}

fn main() {
    //kullanıcılar
    let mut user1 = Users::new("Furkan".to_string(), "Özkan".to_string(), "1".to_string());
    let mut user2 = Users::new("Ahmet".to_string(), "Demir".to_string(), "2".to_string());
    let mut user3 = Users::new("Veli".to_string(), "Büyük".to_string(), "3".to_string());

   //hesap ekleme
    user1.add_account(Account::new(user1.user_id.clone(), 5000,"1".to_string()));
    user1.add_account(Account::new(user1.user_id.clone(), 7000,"2".to_string()));

    user2.add_account(Account::new(user2.user_id.clone(), 10000,"3".to_string()));
    user2.add_account(Account::new(user2.user_id.clone(), 100000,"4".to_string()));

    user3.add_account(Account::new(user3.user_id.clone(), 10300,"5".to_string()));

    let mut account1 = Account::new("1".to_string(), 1000,"12".to_string());
    let mut account2 = Account::new("2".to_string(), 1000,"12".to_string());
// banka oluşturma
    let bank = Bank::new("Ziraat Bankası".to_string(), 1000000000000000, vec![user1, user2,user3]);

    
// kullanıcı bilgileri
   // println!("{:?}",bank.print_users());

    // para transferi
match account1.transfer_money(500, &mut account2) {
    Ok(()) => println!("transfer başarılı"),
    Err(error_msg) => println!("Hata: {}", error_msg),
}
println!("gönderen hesap bakiyesi: {}",account1.get_balance());
println!("alan hesap bakiyesi: {}",account2.get_balance());
}
