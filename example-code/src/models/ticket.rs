// FILE: models/ticket.rs
// ___________________________________________________________

use std::io::Error;
use iso_currency::Currency;
use utility_lib::utils::utilities::Void;
use crate::utils::helpers::output_result;
// ___________________________________________________________

// PARAMETERS: BUYER-NAME, CONCERT, PRICE
// If ticket defaults == `0` `price` = 50
#[derive(Debug, Clone)]
pub struct BackStage(String, String, f64, Currency);

#[derive(Debug, Clone)]
pub struct Standard(String, String, f64, Currency);

#[derive(Debug, Clone)]
pub struct Vip(String, String, f64, Currency);
// ___________________________________________________________

impl BackStage {
  pub fn new(concert_name: &str, buyer_name: &str, price: f64, currency_type: Currency) -> Self {
    BackStage(concert_name.to_owned(), buyer_name.to_owned(), price, currency_type)
  }
}

impl Standard {
  pub fn new(concert_name: &str, buyer_name: &str, price: f64, currency_type: Currency) -> Self {
    Standard(concert_name.to_owned(), buyer_name.to_owned(), price, currency_type)
  }
}

impl Vip {
  pub fn new(concert_name: &str, buyer_name: &str, price: f64, currency_type: Currency) -> Self {
    Vip(concert_name.to_owned(), buyer_name.to_owned(), price, currency_type)
  }
}
// ___________________________________________________________

pub enum TicketType {
  BackStageTicket(BackStage),
  StandardTicket(Standard),
  VipTicket(Vip),
}
// ___________________________________________________________

pub fn print_ticket_info(ticket: &TicketType) -> Result<Void, Error> {
  match ticket {
    TicketType::BackStageTicket(BackStage(concert_name, buyer_name, price, currency)) => {
      output_result(concert_name, buyer_name, price, currency)?;
    }
    TicketType::StandardTicket(Standard(concert_name, buyer_name, price, currency)) => {
      output_result(concert_name, buyer_name, price, currency)?;
    }
    TicketType::VipTicket(Vip(concert_name, buyer_name, price, currency)) => {
      output_result(concert_name, buyer_name, price, currency)?;
    }
  }

  Ok(())
}
// ___________________________________________________________