use crate::power_set::power_set;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Foreign Amount")]
    pub amount: f32,
    #[serde(rename = "Currency")]
    pub currency: String,
}

pub type Invoices = Vec<Invoice>;

pub fn group_by_account(invoices: Invoices) -> Vec<Invoices> {
    let mut array: Vec<Invoices> = vec![];
    for account in invoices
        .iter()
        .map(|invoice| invoice.account.clone())
        .unique()
    {
        array.push(
            invoices
                .clone()
                .into_iter()
                .filter(|invoice| invoice.account == account)
                .collect::<Invoices>(),
        );
    }
    array
}

pub fn sum(invoices: Invoices) -> Invoice {
    let invoice = &invoices[0].clone();
    Invoice {
        account: invoice.account.clone(),
        amount: invoices.into_iter().map(|invoice| invoice.amount).sum(),
        currency: invoice.currency.clone(),
    }
}

pub fn power_set_sum(items: &[Invoice]) -> Vec<Invoice> {
    power_set(items)
        .into_iter()
        .map(|s| sum(s))
        .collect::<Vec<Invoice>>()
}

pub fn power_set_sums(invoices: Invoices) -> Invoices {
    let mut array: Invoices = vec![];
    for group in group_by_account(invoices) {
        array.append(&mut power_set_sum(&group));
    }
    array
}
