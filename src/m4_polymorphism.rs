use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, & str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid address"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, & str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T)  {
    let converted_address = address.convert_address();
    dbg!(converted_address);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_poly() {
        // let addr = Address::from_str("0xFDeBf682AE960A2cFCE2E10cf297970E3296E483").unwrap();

        // assert_eq!(get_ethereum_data("0xFDeBf682AE960A2cFCE2E10cf297970E3296E483"), addr);

        dbg!(get_ethereum_data("0xFDeBf682AE960A2cFCE2E10cf297970E3296E4"));
    }

}