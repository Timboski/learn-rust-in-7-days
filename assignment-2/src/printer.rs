
/// Create a printable representation of currency
///
/// ```
/// use money_typesafe::printer::display_curr;
/// assert_eq!(&display_curr(5,'£',2),"£0.05");
/// assert_eq!(&display_curr(-3,'£',2),"-£0.03");
// assert_eq!(&display_curr(-3456,'£',2),"-£34.56");
// assert_eq!(&display_curr(3,'£',2),"£0.03");
// assert_eq!(&display_curr(3456,'£',2),"£34.56");
/// ```
pub fn display_curr(mut v:i32,sym:char,dp:usize)->String{
    let mut prefix = String::new();
    let mut value = (v as f32) / 100.0;

    if value < 0.0 {
        prefix.push('-');
        value = -value;
    }

    prefix.push(sym);
 
    format!("{}{}",prefix,value)
}
