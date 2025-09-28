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

fn print_account(account: &Account) {
  println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
  account.balance = 10;
}

fn add_account(bank: &mut Bank, account: Account) {
  bank.accounts.push(account);
}

fn main() {
  let mut bank = Bank::new();
  let mut account = Account::new(1, String::from("Jack"));

  print_account(&account);
  change_account(&mut account);
  add_account(&mut bank, account);

  println!("{:#?}", bank);
}
