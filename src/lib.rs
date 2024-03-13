#[derive(Debug, PartialEq)]
pub struct Pesel([u8;11]);
impl Pesel {
    pub fn new(pesel: &str) -> Result<Pesel, String> {
        let mut digits = [0u8;11];
        if pesel.parse::<u64>().is_err() || pesel.len() != 11 {
            return Err(String::from("Wrong format"));
        } else {
            pesel.chars().enumerate().for_each(|(i, digit)| {
                let d = digit.to_digit(10).unwrap() as u8;
                digits[i] = d;
            });
            return Ok(Pesel(digits));
        }
    }
    fn checksum(digits: [u8;10]) -> u8 {
        let weights: [u8;10] = [1,3,7,9,1,3,7,9,1,3];
        let mut sumprod = 0u32;
        for i in 0..weights.len() {
            sumprod += weights[i] as u32 * digits[i] as u32;
        }
        let checksum = (10 - sumprod % 10) % 10;
        checksum as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pesel_93061412550() {
        let expected = Pesel([9, 3, 0, 6, 1, 4, 1, 2, 5, 5, 0]);
        assert_eq!(Pesel::new("93061412550").unwrap(), expected);
    }

    #[test]
    fn test_pesel_00124567890() {
        let expected = Pesel([0, 0, 1, 2, 4, 5, 6, 7, 8, 9, 0]);
        assert_eq!(Pesel::new("00124567890").unwrap(), expected);
    }

    #[test]
    fn test_pesel_91000000000() {
        let expected = Pesel([9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(Pesel::new("91000000000").unwrap(), expected);
    }

    #[test]
    fn test_incorrect_pesel_length_short() {
        assert!(Pesel::new("0012457897").is_err(), "PESEL with incorrect length (short) should return an error.");
    }

    #[test]
    fn test_incorrect_pesel_length_long() {
        assert!(Pesel::new("931212121212").is_err(), "PESEL with incorrect length (long) should return an error.");
    }

    #[test]
    fn test_incorrect_pesel_non_numeric() {
        assert!(Pesel::new("pesel878711").is_err(), "Non-numeric PESEL should return an error.");
    }

    // checksum function
    #[test]
    fn checksum_correct() {
        // Test case with a known checksum
        let digits: [u8; 10] = [9, 3, 0, 6, 1, 4, 1, 2, 5, 5];
        assert_eq!(Pesel::checksum(digits), 0, "Checksum should be 0 for this sequence.");
    }

    #[test]
    fn checksum_incorrect() {
        // Test case where modifying a digit should change the checksum
        let digits: [u8; 10] = [9, 3, 0, 6, 1, 4, 1, 2, 5, 6]; // Changed the last digit
        assert_ne!(Pesel::checksum(digits), 0, "Checksum should not be 0 for this modified sequence.");
    }

    #[test]
    fn checksum_boundary() {
        // Boundary test case where the checksum calculation might overflow
        let digits: [u8; 10] = [9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
        assert_eq!(Pesel::checksum(digits), 4, "Checksum calculation should handle overflow correctly.");
    }

    #[test]
    fn checksum_zeroes() {
        // Test case with all zeroes to check if function handles it correctly
        let digits: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Pesel::checksum(digits), 0, "Checksum should be 0 for all zeroes.");
    }
}