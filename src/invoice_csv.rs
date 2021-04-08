use crate::invoice::{Invoice, Invoices};
use csv::{Reader, Writer};

pub fn read_csv(file: String) -> Result<Invoices, csv::Error> {
    let mut invoices: Invoices = vec![];
    let mut rdr = Reader::from_path(file)?;
    for result in rdr.deserialize() {
        let invoice: Invoice = result?;
        invoices.push(invoice);
    }
    Ok(invoices)
}

pub fn write_csv(file: String, invoices: Invoices) -> Result<(), csv::Error> {
    let mut wtr = Writer::from_path(file)?;
    for invoice in invoices {
        wtr.serialize(invoice)?;
    };
    wtr.flush()?;
    Ok(())
}
