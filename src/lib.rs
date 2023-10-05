use algonaut_core::Address;
use std::fmt;

pub struct AlgorandUrn {
    address: Option<Address>,
    params: Vec<Param>,
}

impl AlgorandUrn {
    pub fn builder() -> AlgorandUrnBuilder {
        Default::default()
    }
}

impl fmt::Display for AlgorandUrn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorand://")?;

        if let Some(addr) = self.address {
            write!(f, "{}", addr)?;
        }

        if !self.params.is_empty() {
            write!(f, "?")?;

            for p in &self.params[..&self.params.len() - 1] {
                write!(f, "{}&", p)?;
            }

            if let Some(p) = &self.params.last() {
                write!(f, "{}", p)?;
            }
        }

        Ok(())
    }
}

pub enum Param {
    Amount(u64),
    Label(String),
    Asset(u64),
    XNote(String),
    Note(String),
    Other(String, String),
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Amount(amt) => write!(f, "amount={}", amt),
            Param::Label(label) => write!(f, "label={}", label),
            Param::Asset(id) => write!(f, "asset={}", id),
            Param::XNote(text) => write!(f, "xnote={}", text),
            Param::Note(text) => write!(f, "note={}", text),
            Param::Other(key, value) => write!(f, "{}={}", key, value),
        }
    }
}

#[derive(Default)]
pub struct AlgorandUrnBuilder {
    address: Option<Address>,
    params: Vec<Param>,
}

impl AlgorandUrnBuilder {
    pub fn address(mut self, addr: Address) -> Self {
        self.address = Some(addr);
        self
    }

    pub fn amount(mut self, amt: u64) -> Self {
        self.params.push(Param::Amount(amt));
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.params.push(Param::Label(label));
        self
    }

    pub fn asset(mut self, asa_id: u64) -> Self {
        self.params.push(Param::Asset(asa_id));
        self
    }

    pub fn xnote(mut self, xnote: String) -> Self {
        self.params.push(Param::XNote(xnote));
        self
    }

    pub fn note(mut self, note: String) -> Self {
        self.params.push(Param::Note(note));
        self
    }

    pub fn build(self) -> AlgorandUrn {
        AlgorandUrn {
            address: self.address,
            params: self.params,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const ALGO_ADDRESS: &'static str = "TMTAD6N22HCS2LKH7677L2KFLT3PAQWY6M4JFQFXQS32ECBFC23F57RYX4";

    #[test]
    fn share_address() {
        let uri = AlgorandUrn::builder()
            .address(Address::from_str(ALGO_ADDRESS).unwrap())
            .build();

        insta::assert_yaml_snapshot!(uri.to_string());
    }

    #[test]
    fn share_address_with_label() {
        let uri = AlgorandUrn::builder()
            .address(Address::from_str(ALGO_ADDRESS).unwrap())
            .label("Silvio".to_string())
            .build();

        insta::assert_yaml_snapshot!(uri.to_string());
    }

    #[test]
    fn request_payment() {
        let uri = AlgorandUrn::builder()
            .address(Address::from_str(ALGO_ADDRESS).unwrap())
            .amount(150500000)
            .build();

        insta::assert_yaml_snapshot!(uri.to_string());
    }

    #[test]
    fn request_payment_with_note() {
        let uri = AlgorandUrn::builder()
            .address(Address::from_str(ALGO_ADDRESS).unwrap())
            .amount(150500000)
            .note("Lunch".to_string())
            .build();

        insta::assert_yaml_snapshot!(uri.to_string());
    }

    #[test]
    fn request_asset_units() {
        let uri = AlgorandUrn::builder()
            .address(Address::from_str(ALGO_ADDRESS).unwrap())
            .amount(150)
            .asset(31566704)
            .build();

        insta::assert_yaml_snapshot!(uri.to_string());
    }

    #[test]
    fn optin_asset() {
        let uri = AlgorandUrn::builder().amount(0).asset(31566704).build();

        insta::assert_yaml_snapshot!(uri.to_string());
    }
}
