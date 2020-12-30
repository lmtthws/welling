use std::time::{Instant};
use ::currency::*;

#[derive(Clone,PartialEq, Debug)]
pub struct Item {
    name: String, 
    brand: String,
    model_no: String,
    purchase_info: PurchaseInfo,
    //pictures: Vec<Picture>,
    //receipt: Receipt,
}

#[derive(Clone,PartialEq,Debug)]
pub struct PurchaseInfo {
    date: Instant,
    price: Price,
    location: String,
}