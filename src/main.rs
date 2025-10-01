mod finance;
// mod bank;

use finance::bank::Bank;
use finance::account::Account;

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
