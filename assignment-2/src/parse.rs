use self::ParseMoneyError::*;

#[derive(Debug,PartialEq)]
pub enum ParseMoneyError {
    SymbolErr,
    NoStringErr, 
    TwoPointsErr,
    NonDigitErr(char),
    TooFarErr,
}


/// ```
// use money_typesafe::parse::parse_sym_money;
// assert_eq!(parse_sym_money("£34.3",'£',2),Ok(3430));
// assert_eq!(parse_sym_money("-£34", '£',2),Ok(-3400));
// assert!(parse_sym_money("£34", '$',2).is_err());
// assert!(parse_sym_money("-£34", '$',2).is_err());
/// ```
pub fn parse_sym_money(s:&str,sym:char,dpoint:usize)->Result<i32,ParseMoneyError>{
    let (c,v) = parse_money(s,dpoint)?;
    if c != sym {
        return Err(ParseMoneyError::SymbolErr);
    }
    Ok(v)
}

/// Parse currency notation returning currency symbol and value as i32
///
/// ```
/// use money_typesafe::parse::parse_money;
/// let (c,v) = parse_money("£34.3",2).unwrap();
/// assert_eq!(c,'£');
// assert_eq!(v,3430);
//
// assert!(parse_money("£34.304",2).is_err());
// assert!(parse_money("£34..04",2).is_err());
//
// assert_eq!(parse_money("£.34",2),Ok(('£',34)));
/// ```
pub fn parse_money(s:&str, dpoint:usize)->Result<(char,i32),ParseMoneyError>{
    let first_character = s.chars().next();
    
    if first_character.is_none() {
        return Err(ParseMoneyError::NoStringErr);
    }

    let symbol = first_character.unwrap();
    Ok((symbol, 0))
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
    
    //#[test]
    fn given_valid_string_when_parse_money_then_returns_expected_currency_value() {
        let (c,v) = parse_money("£34.3",2).unwrap();
        assert_eq!(c,'£');
        assert_eq!(v,3430);
    }

    //#[test]
    fn given_too_many_decimal_places_when_parse_money_then_returns_err() {
        assert!(parse_money("£34.304",2).is_err());
    }

    //#[test]
    fn given_invalid_format_when_parse_money_then_returns_err() {
        assert!(parse_money("£34..04",2).is_err());
    }

    //#[test]
    fn given_only_pence_when_parse_money_then_returns_expected_result() {
        assert_eq!(parse_money("£.34",2),Ok(('£',34)));
    }
}
