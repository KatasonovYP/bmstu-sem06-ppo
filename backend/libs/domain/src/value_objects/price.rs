use std::ops::{
    Add,
    Mul,
    Sub,
};

use crate::errors::DomainError;

#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Currency {
    pub value: String,
}

impl Currency {
    pub fn rub() -> Self {
        Currency {
            value: "RUB".to_string(),
        }
    }

    pub fn usd() -> Self {
        Currency {
            value: "USD".to_string(),
        }
    }
}

#[cfg_attr(not(feature = "production"), derive(fake::Dummy))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Price {
    pub amount: f64,
    pub currency: Currency,
}

impl Price {
    pub fn new(amount: f64, currency: String) -> Self {
        Self {
            amount,
            currency: Currency { value: currency },
        }
    }

    pub fn rub(amount: f64) -> Self {
        Self {
            amount,
            currency: Currency::rub(),
        }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: 0.0,
            currency,
        }
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl Sub for Price {
    type Output = Result<Price, DomainError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.currency == rhs.currency {
            Ok(Price {
                amount: self.amount - rhs.amount,
                currency: self.currency,
            })
        } else {
            Err(DomainError::ValidationError(format!(
                "Currency Mismatch {} != {}",
                self.currency.value, rhs.currency.value,
            )))
        }
    }
}

impl Add for Price {
    type Output = Result<Price, DomainError>;

    fn add(self, rhs: Self) -> Self::Output {
        self - (Price::zero(rhs.currency.clone()) - rhs)?
    }
}

impl Mul<u32> for Price {
    type Output = Price;

    fn mul(self, rhs: u32) -> Self::Output {
        Price {
            amount: self.amount * rhs as f64,
            currency: self.currency,
        }
    }
}
