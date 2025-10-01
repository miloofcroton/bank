// use crate::account::Account;
use super::account::Account;

#[derive(Debug)]
pub struct Bank {
  accounts: Vec<Account>,
}

impl Bank {
  pub fn new() -> Self {
    return Bank { accounts: vec![] };
  }

  pub fn summary(&self) -> Vec<String> {
    return self.accounts
      .iter()
      .map(|account| account.summary())
      .collect::<Vec<String>>();
  }

  pub fn addAccount(&mut self, account: Account) {
    self.accounts.push(account);
  }

  pub fn totalBalance(&self) -> i32 {
    return self.accounts
      .iter()
      .map(|account| account.balance)
      .sum();
  }
}
