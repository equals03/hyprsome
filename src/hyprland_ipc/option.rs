use hyprland::keyword::{Keyword, OptionValue};

const GENERAL_GAPS_OUT: &str = "general:gaps_out";

pub fn get_gaps() -> i16 {
    let gaps_out = match Keyword::get(GENERAL_GAPS_OUT) {
        Ok(gaps_out_option) => match gaps_out_option.value {
            OptionValue::Int(i) => i,
            _ => panic!("gaps_out can only be an int"),
        },
        _ => 0,
    };

    gaps_out as i16
}
