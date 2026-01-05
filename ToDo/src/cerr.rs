use colored::Colorize;

pub fn throw_error(message: &str){
    println!("{}", String::from(message).red().italic());
}

macro_rules! function_error_wrap {
    ($match_exp:expr, $fn_name:tt, $error_message:expr $(, $args:expr)*) => {
        $fn_name($($args),*)
        fn wrap(){
            match $match_exp{
                Some(_) => {
                    throw_error($error_message);
                }
                None => {
                    $fn_name($($args:expr),*)
                }
            }
        }
    }
}
pub(crate) use function_error_wrap;