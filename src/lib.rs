use std::collections::HashMap;

/// Translates number into words
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = number_to_words_ru::number_to_words(arg);
///
/// assert_eq!("пять", answer);
/// ```
pub fn number_to_words(value: i64) -> String {
    let mut buffer = String::new();

    let mut value = value;
    if value < 0 {
        value = value.abs();
        append("минус", &mut buffer);
    }

    if value == 0 {
        append("ноль", &mut buffer);
    } else {
        let billions = (value % 1_000_000_000_000) / 1_000_000_000;
        translate(billions, &mut buffer, get_billions_vocabulary(), "миллиардов");

        let millions = (value % 1_000_000_000) / 1_000_000;
        translate(millions, &mut buffer, get_millions_vocabulary(), "миллионов");

        let thousands = (value % 1_000_000) / 1000;
        translate(thousands, &mut buffer, get_thousands_vocabulary(), "тысяч");

        let one_to_999 = value % 1000;
        translate(one_to_999, &mut buffer, get_one_to_nineteen_vocabulary(), "");
    }
    buffer
}


fn translate(one_to_thousand: i64, mut buffer: &mut String, nominal_unit_voc: HashMap<i64, &str>, nominal_unit: &str) {
    let third_digit = one_to_thousand / 100;
    get_hundreds_vocabulary().get(&third_digit).map(|word| { append(word, &mut buffer) });
    let one_to_ninety_nine = one_to_thousand % 100;
    match one_to_ninety_nine {
        0 if third_digit != 0 => { append(nominal_unit, &mut buffer); }
        1..=19 => { nominal_unit_voc.get(&one_to_ninety_nine).map(|word| { append(word, &mut buffer) }); }
        20..=99 => {
            let second_digit = one_to_ninety_nine / 10;
            get_tens_vocabulary().get(&second_digit).map(|word| { append(word, &mut buffer) });
            let first_digit = one_to_ninety_nine % 10;
            if first_digit == 0 {
                append(nominal_unit, &mut buffer);
            } else {
                nominal_unit_voc.get(&first_digit).map(|word| { append(word, &mut buffer) });
            }
        }
        _ => {}
    }
}

fn append(word: &str, buffer: &mut String) {
    if !buffer.is_empty() && !word.is_empty() {
        buffer.push_str(" ");
    }
    buffer.push_str(word);
}

fn get_one_to_nineteen_vocabulary() -> HashMap<i64, &'static str> {
    HashMap::from([
        (1, "один"),
        (2, "два"),
        (3, "три"),
        (4, "четыре"),
        (5, "пять"),
        (6, "шесть"),
        (7, "семь"),
        (8, "восемь"),
        (9, "девять"),
        (10, "десять"),
        (11, "одиннадцать"),
        (12, "двенадцать"),
        (13, "тринадцать"),
        (14, "четырнадцать"),
        (15, "пятьнадцать"),
        (16, "шестьнадцать"),
        (17, "семтьнадцать"),
        (18, "восемьнадцать"),
        (19, "девятьнадцать")
    ])
}

fn get_tens_vocabulary() -> HashMap<i64, &'static str> {
    HashMap::from([
        (1, "десять"),
        (2, "двадцать"),
        (3, "тридцать"),
        (4, "сорок"),
        (5, "пятьдесят"),
        (6, "шестьдесят"),
        (7, "семьдесят"),
        (8, "восемьдесят"),
        (9, "девяносто")
    ])
}

fn get_hundreds_vocabulary() -> HashMap<i64, &'static str> {
    HashMap::from([
        (1, "сто"),
        (2, "двести"),
        (3, "триста"),
        (4, "четыреста"),
        (5, "пятьсот"),
        (6, "шестьсот"),
        (7, "семьсот"),
        (8, "восемьсот"),
        (9, "девятьсот")
    ])
}

fn get_thousands_vocabulary() -> HashMap<i64, &'static str> {
    HashMap::from([
        (1, "одна тысяча"),
        (2, "две тысячи"),
        (3, "три тысячи"),
        (4, "четыре тысячи"),
        (5, "пять тысяч"),
        (6, "шесть тысяч"),
        (7, "семь тысяч"),
        (8, "восемь тысяч"),
        (9, "девять тысяч"),
        (10, "десять тысяч"),
        (11, "одиннадцать тысяч"),
        (12, "двенадцать тысяч"),
        (13, "тринадцать тысяч"),
        (14, "четырнадцать тысяч"),
        (15, "пятьнадцать тысяч"),
        (16, "шестьнадцать тысяч"),
        (17, "семьнадцать тысяч"),
        (18, "восемьнадцать тысяч"),
        (19, "девятьнадцать тысяч")
    ])
}

fn get_millions_vocabulary() -> HashMap<i64, &'static str> {
    HashMap::from([
        (1, "один миллион"),
        (2, "два миллиона"),
        (3, "три миллиона"),
        (4, "четыре миллиона"),
        (5, "пять миллионов"),
        (6, "шесть миллионов"),
        (7, "семь миллионов"),
        (8, "восемь миллионов"),
        (9, "девять миллионов"),
        (10, "десять миллионов"),
        (11, "одиннадцать миллионов"),
        (12, "двенадцать миллионов"),
        (13, "тринадцать миллионов"),
        (14, "четырнадцать миллионов"),
        (15, "пятьнадцать миллионов"),
        (16, "шестьнадцать миллионов"),
        (17, "семьнадцать миллионов"),
        (18, "восемьнадцать миллионов"),
        (19, "девятьнадцать миллионов")
    ])
}

fn get_billions_vocabulary() -> HashMap<i64, &'static str> {
    HashMap::from([
        (1, "один миллиард"),
        (2, "два миллиарда"),
        (3, "три миллиарда"),
        (4, "четыре миллиарда"),
        (5, "пять миллиардов"),
        (6, "шесть миллиардов"),
        (7, "семь миллиардов"),
        (8, "восемь миллиардов"),
        (9, "девять миллиардов"),
        (10, "десять миллиардов"),
        (11, "одиннадцать миллиардов"),
        (12, "двенадцать миллиардов"),
        (13, "тринадцать миллиардов"),
        (14, "четырнадцать миллиардов"),
        (15, "пятьнадцать миллиардов"),
        (16, "шестьнадцать миллиардов"),
        (17, "семьнадцать миллиардов"),
        (18, "восемьнадцать миллиардов"),
        (19, "девятьнадцать миллиардов")
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("ноль", number_to_words(0));
        assert_eq!("один", number_to_words(1));
        assert_eq!("минус один", number_to_words(-1));
        assert_eq!("пять", number_to_words(5));
        assert_eq!("девять", number_to_words(9));
        assert_eq!("десять", number_to_words(10));
        assert_eq!("одиннадцать", number_to_words(11));
        assert_eq!("двадцать", number_to_words(20));
        assert_eq!("двадцать один", number_to_words(21));
        assert_eq!("минус сорок", number_to_words(-40));
        assert_eq!("минус сорок шесть", number_to_words(-46));
        assert_eq!("сто", number_to_words(100));
        assert_eq!("сто один", number_to_words(101));
        assert_eq!("сто тринадцать", number_to_words(113));
        assert_eq!("двести пятьнадцать", number_to_words(215));
        assert_eq!("двести тридцать восемь", number_to_words(238));
        assert_eq!("минус триста", number_to_words(-300));
        assert_eq!("девятьсот девяносто девять", number_to_words(999));
        assert_eq!("одна тысяча", number_to_words(1_000));
        assert_eq!("одна тысяча один", number_to_words(1_001));
        assert_eq!("три тысячи тринадцать", number_to_words(3_013));
        assert_eq!("пять тысяч двадцать семь", number_to_words(5_027));
        assert_eq!("семь тысяч сто", number_to_words(7_100));
        assert_eq!("одиннадцать тысяч двести тридцать шесть", number_to_words(11_236));
        assert_eq!("десять тысяч", number_to_words(10_000));
        assert_eq!("двадцать тысяч", number_to_words(20_000));
        assert_eq!("двадцать одна тысяча", number_to_words(21_000));
        assert_eq!("двадцать семь тысяч пятьсот двенадцать", number_to_words(27_512));
        assert_eq!("сто тысяч", number_to_words(100_000));
        assert_eq!("сто тринадцать тысяч", number_to_words(113_000));
        assert_eq!("сто двадцать тысяч", number_to_words(120_000));
        assert_eq!("девятьсот девяносто девять тысяч девятьсот девяносто девять",
                   number_to_words(999_999));
        assert_eq!("один миллион", number_to_words(1_000_000));
        assert_eq!("два миллиона", number_to_words(2_000_000));
        assert_eq!("десять миллионов", number_to_words(10_000_000));
        assert_eq!("четырнадцать миллионов", number_to_words(14_000_000));
        assert_eq!("двадцать миллионов", number_to_words(20_000_000));
        assert_eq!("двадцать один миллион", number_to_words(21_000_000));
        assert_eq!("двадцать два миллиона", number_to_words(22_000_000));
        assert_eq!("двадцать пять миллионов", number_to_words(25_000_000));
        assert_eq!("сто миллионов", number_to_words(100_000_000));
        assert_eq!("сто один миллион", number_to_words(101_000_000));
        assert_eq!("сто два миллиона", number_to_words(102_000_000));
        assert_eq!("сто пять миллионов", number_to_words(105_000_000));
        assert_eq!("сто двенадцать миллионов", number_to_words(112_000_000));
        assert_eq!("двести миллионов", number_to_words(200_000_000));
        assert_eq!("триста тринадцать миллионов", number_to_words(313_000_000));
        assert_eq!("пятьсот двадцать семь миллионов", number_to_words(527_000_000));
        assert_eq!("девятьсот девяносто девять миллионов девятьсот девяносто девять тысяч девятьсот девяносто девять",
                   number_to_words(999_999_999));
        assert_eq!(
            "девятьсот девяносто один миллиард девятьсот девяносто один миллион четыреста одна тысяча двести восемьдесят",
            number_to_words(991_991_401_280)
        );
    }
}
