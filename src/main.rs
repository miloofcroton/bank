#[derive(Debug)]
struct Account {
  id: u32,
  balance: i32,
  holder: String,

}

impl Account {
  fn new(id: u32, holder: String) -> Self {
    return Account { id, holder, balance: 0 };
  }

  fn summary(&self) -> String {
    return format!("{} has a balance of ${}", self.holder, self.balance)

  }

  fn deposit(&mut self, amount: i32) -> i32 {
    self.balance += amount;
    return self.balance;
  }

  fn withdraw(&mut self, amount: i32) -> i32 {
    self.balance -= amount;
    return self.balance;
  }
}

#[derive(Debug)]
struct Bank {
  accounts: Vec<Account>,
}

impl Bank {
  fn new() -> Self {
    return Bank { accounts: vec![] };
  }

  fn summary(&self) -> Vec<String> {
    return self.accounts
      .iter()
      .map(|account| account.summary())
      .collect::<Vec<String>>();
  }

  fn addAccount(&mut self, account: Account) {
    self.accounts.push(account);
  }

  fn totalBalance(&self) -> i32 {
    return self.accounts
      .iter()
      .map(|account| account.balance)
      .sum();
  }
}

// fn printBank(bank: Bank) {
//   println!("{:#?}", bank);
// }

fn main() {
  let mut bank = Bank::new();
  let mut account = Account::new(1, String::from("Jack"));

  account.deposit(500);
  account.withdraw(250);

  // println!("{}", account.summary());

  bank.addAccount(account);

  // printBank(bank);
  println!("{:#?}", bank.summary());
  println!("{:#?}", bank.totalBalance());
}
