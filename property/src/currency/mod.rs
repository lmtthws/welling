#[derive(Clone,PartialEq,Debug)]
pub struct Amount {
    pub units: u32,
    pub fractional: u32,
}

#[derive(Clone,PartialEq,Debug)]
pub struct Price {
    pub total: Amount,
    pub change: Bookkeeping,
    pub currency: Currency
}

#[derive(Clone,PartialEq,Debug)]
pub enum Bookkeeping {
    Debit,
    Credit
}

#[derive(Clone,PartialEq,Debug)]
pub enum Currency {
    DollarsUS
}