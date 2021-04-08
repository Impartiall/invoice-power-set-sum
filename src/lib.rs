extern crate csv;
extern crate itertools;

mod invoice;
mod invoice_csv;
mod power_set;

use invoice::power_set_sums;
use invoice_csv::{read_csv, write_csv};
use std::cmp::Ordering;

pub fn main(input: String, output: String) {
    let invoices = match read_csv(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut new_invoices = power_set_sums(invoices);
    new_invoices.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap_or(Ordering::Equal));
    match write_csv(output, new_invoices) {
        Err(e) => panic!("{}", e),
        _ => (),
    }
}
