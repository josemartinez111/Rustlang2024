// ticket_example.rs
// ________________________________________________________

use utility_lib::iso_currency::Currency;
use crate::models::ticket::{BackStage, print_ticket_info, Standard, TicketType, Vip};

mod models;
mod utils;

// ________________________________________________________

fn main() {
  // __________________________________________________

  // Create a vector of tickets
  let tickets: Vec<TicketType> = vec![
    // PARAMETERS: BUYER-NAME, CONCERT, PRICE, CURRENCY-TYPE
    TicketType::BackStageTicket(BackStage::new("Jose", "God-Smack", 300f64, Currency::USD)),
    TicketType::StandardTicket(Standard::new("Bison", "Ruff-Ryders", 0f64, Currency::DKK)),
    TicketType::VipTicket(Vip::new("Big Balla", "Beyonce", 3300f64, Currency::EUR)),
  ];

  for ticket in tickets {
    print_ticket_info(&ticket)
      .unwrap_or_else(|_| eprintln!("Couldn't notify printer: ticket info"));
  }
}
// __________________________________________________________