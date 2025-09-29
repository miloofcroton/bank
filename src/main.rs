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
}

#[derive(Debug)]
struct Bank {
  accounts: Vec<Account>,
}

impl Bank {
  fn new() -> Self {
    return Bank { accounts: vec![] };
  }
}

fn printAccount(account: &Account) {
  println!("{:#?}", account);
}

fn changeAccount(account: &mut Account) {
  account.balance = 10;
}

fn addAccount(bank: &mut Bank, account: Account) {
  bank.accounts.push(account);
}

// fn make_and_print

fn main() {
  let mut bank = Bank::new();
  let mut account = Account::new(1, String::from("Jack"));

  printAccount(&account);
  changeAccount(&mut account);
  addAccount(&mut bank, account);

  println!("{:#?}", bank);
}
