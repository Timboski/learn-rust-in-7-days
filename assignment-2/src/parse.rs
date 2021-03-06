
#[derive(Debug,PartialEq)]
pub enum ParseMoneyError {
    SymbolErr,
    NoStringErr, 
    TwoPointsErr,
    NonDigitErr(char),
    TooFarErr,
}


/// ```
/// use money_typesafe::parse::parse_sym_money;
/// assert_eq!(parse_sym_money("£34.3",'£',2),Ok(3430));
/// assert_eq!(parse_sym_money("-£34", '£',2),Ok(-3400));
/// assert!(parse_sym_money("£34", '$',2).is_err());
/// assert!(parse_sym_money("-£34", '$',2).is_err());
/// ```
pub fn parse_sym_money(s:&str,sym:char,dpoint:usize)->Result<i32,ParseMoneyError>{
    let (c,v) = parse_money(s,dpoint)?;
    if c != sym {
        return Err(ParseMoneyError::SymbolErr);
    }
    Ok(v)
}

struct MoneyValueBuilder {
    digits: String,
    max_decimal_places:usize,
    num_decimal_places:usize,
    num_decimal_points:usize,
}

impl MoneyValueBuilder {
    fn new() -> MoneyValueBuilder {
        MoneyValueBuilder {
            digits: String::new(),
            max_decimal_places: 0,
            num_decimal_places: 0,
            num_decimal_points: 0,
        }
    }

    fn add(&mut self, c:char) {
        match c {
            '.' => self.num_decimal_points += 1,
            _ => {
                self.digits.push(c);
                if self.num_decimal_points > 0 {
                    self.num_decimal_places += 1;
                }
            }
        }
    }

    fn with_decimal_places(&mut self, digits:usize) {
        self.max_decimal_places = digits;
    }

    fn build(mut self) -> Result<i32,ParseMoneyError> {
        if self.num_decimal_points > 1 {
            return Err(ParseMoneyError::TwoPointsErr);
        }

        if self.num_decimal_places > self.max_decimal_places {
            return Err(ParseMoneyError::TooFarErr);
        }

        self.pad_decimal_places();
        Ok(self.digits.parse().unwrap())
    }

    fn pad_decimal_places(&mut self) {
        while self.num_decimal_places < self.max_decimal_places {
            self.digits.push('0');
            self.num_decimal_places += 1;
        }
    }
}

/// Parse currency notation returning currency symbol and value as i32
///
/// ```
/// use money_typesafe::parse::parse_money;
/// let (c,v) = parse_money("£34.3",2).unwrap();
/// assert_eq!(c,'£');
/// assert_eq!(v,3430);
///
/// assert!(parse_money("£34.304",2).is_err());
/// assert!(parse_money("£34..04",2).is_err());
///
/// assert_eq!(parse_money("£.34",2),Ok(('£',34)));
/// 
/// assert_eq!(parse_money("-£123.45",2),Ok(('£',-12345)));
/// ```
pub fn parse_money(s:&str, dpoint:usize)->Result<(char,i32),ParseMoneyError>{
    let mut it = s.chars();
    let mut symbol;

    let mut value_builder = MoneyValueBuilder::new();
    value_builder.with_decimal_places(dpoint);

    loop  {
        symbol = it.next().ok_or(ParseMoneyError::NoStringErr)?;
        match symbol {
            '-' => value_builder.add('-'),
            _ => break,
        }
    }

    while let Some(next_character) = it.next() {
        if next_character != '.' && (next_character < '0' || next_character > '9') {
            return Err(ParseMoneyError::NonDigitErr(next_character));
        }

        value_builder.add(next_character);
    }

    let value = value_builder.build()?;
    Ok((symbol, value))
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn given_valid_gbp_string_when_parse_money_then_returns_pound_currency_symbol() {
        let (c,_v) = parse_money("£34.3",2).unwrap();
        assert_eq!(c,'£');
    }

    #[test]
    fn given_valid_usd_string_when_parse_money_then_returns_dollar_currency_symbol() {
        let (c,_v) = parse_money("$15.50",2).unwrap();
        assert_eq!(c,'$');
    }

    #[test]
    fn given_valid_euro_string_when_parse_money_then_returns_euro_currency_symbol() {
        let (c,_v) = parse_money("€50",2).unwrap();
        assert_eq!(c,'€');
    }

    #[test]
    fn given_empty_string_when_parse_money_then_returns_no_string_error() {
        let err = parse_money("",2).unwrap_err();
        assert_eq!(err,ParseMoneyError::NoStringErr);
    }

    #[test]
    fn given_full_string_when_parse_money_then_returns_expected_currency_value() {
        let (_c, v) = parse_money("£12.34", 2).unwrap();
        assert_eq!(v, 1234);
    }
    
    #[test]
    fn given_valid_string_when_parse_money_then_returns_expected_currency_value() {
        let (_c,v) = parse_money("£34.3",2).unwrap();
        assert_eq!(v,3430);
    }
    
    #[test]
    fn given_valid_string_when_parse_money_with_3_decimal_places_then_returns_expected_currency_value() {
        let (_c,v) = parse_money("£34.3",3).unwrap();
        assert_eq!(v,34300);
    }

    #[test]
    fn given_too_many_decimal_places_when_parse_money_then_returns_err() {
        assert!(parse_money("£34.304",2).is_err());
    }

    #[test]
    fn given_invalid_format_when_parse_money_then_returns_err() {
        assert!(parse_money("£34..04",2).is_err());
    }

    #[test]
    fn given_only_pence_when_parse_money_then_returns_expected_result() {
        assert_eq!(parse_money("£.34",2),Ok(('£',34)));
    }

    #[test]
    fn given_negative_value_when_parse_money_then_returns_expected_currency_symbol() {
        let (c,_v) = parse_money("-£123.45",2).unwrap();
        assert_eq!(c,'£');
    }

    #[test]
    fn given_negative_value_when_parse_money_then_returns_expected_result() {
        let (_c,v) = parse_money("-£123.45",2).unwrap();
        assert_eq!(v,-12345);
    }

    #[test]
    fn given_invalid_character_at_end_of_string_when_parse_money_then_returns_err_containing_invalid_character() {
        let err = parse_money("-£123.45p",2).unwrap_err();
        assert_eq!(err, ParseMoneyError::NonDigitErr('p'));
    }

    #[test]
    fn given_invalid_character_at_start_of_string_when_parse_money_then_returns_err_containing_first_invalid_character() {
        let err = parse_money("£GB",2).unwrap_err();
        assert_eq!(err, ParseMoneyError::NonDigitErr('G'));
    }

    #[test]
    fn given_invalid_character_in_middle_of_number_when_parse_money_then_returns_err_containing_first_invalid_character() {
        let err = parse_money("£15point3",2).unwrap_err();
        assert_eq!(err, ParseMoneyError::NonDigitErr('p'));
    }
}
