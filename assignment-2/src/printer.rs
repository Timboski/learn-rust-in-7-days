
/// Create a printable representation of currency
///
/// ```
/// use money_typesafe::printer::display_curr;
/// assert_eq!(&display_curr(5,'£',2),"£0.05");
/// assert_eq!(&display_curr(3,'£',2),"£0.03");
/// assert_eq!(&display_curr(-3,'£',2),"-£0.03");
/// assert_eq!(&display_curr(-3456,'£',2),"-£34.56");
/// assert_eq!(&display_curr(3456,'£',2),"£34.56");
/// assert_eq!(&display_curr(1230,'£',2),"£12.30");
/// assert_eq!(&display_curr(123456,'T',4),"T12.3456");
/// assert_eq!(&display_curr(-1234560,'T',5),"-T12.34560");
/// ```
pub fn display_curr(v:i32,sym:char,dp:usize)->String{
    let mut result = String::new();

    // Figure out if we are negative.
    let mut value = v;
    if value < 0 {
        result.push('-');
        value = -value;
    }

    // Add the currency symbol.
    result.push(sym);

    // Add digits without decimal place (padded to minimum length).
    let digits = value.to_string();
    let mut missing_digits = (dp as i32 + 1) - (digits.len() as i32);
    while missing_digits > 0 {
        result.push('0');
        missing_digits -= 1;
    }
    result += &digits;

    // Insert the decimal point.
    let (a,b) = (&result).split_at(result.len()-dp);
    format!("{}.{}", a, b)
 }
