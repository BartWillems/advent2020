struct PassportField;
enum ParseError<'a> {
    InvalidValue(&'a str),
}

impl PassportField {
    fn valid_number(input: &str, min: usize, max: usize) -> bool {
        match input.parse::<usize>() {
            Err(_) => false,
            Ok(val) => val >= min && val <= max,
        }
    }
    fn parse(input: &str) -> Result<(), ParseError> {
        let dink: Vec<&str> = input.split(":").collect();

        let pair: (&str, &str) = match &dink[..] {
            &[key, value, ..] => (key, value),
            _ => {
                unreachable!()
            }
        };

        match pair {
            ("byr", val) => {
                if PassportField::valid_number(val, 1920, 2020) {
                    Ok(())
                } else {
                    Err(ParseError::InvalidValue("birth year out of range"))
                }
            }
            ("iyr", val) => {
                if PassportField::valid_number(val, 2010, 2020) {
                    Ok(())
                } else {
                    Err(ParseError::InvalidValue("birth year out of range"))
                }
            }
            ("eyr", val) => {
                if PassportField::valid_number(val, 2020, 2030) {
                    Ok(())
                } else {
                    Err(ParseError::InvalidValue("birth year out of range"))
                }
            }
            ("hgt", val) => {
                if val.ends_with("cm") {
                    if PassportField::valid_number(&val.replace("cm", ""), 150, 193) {
                        return Ok(());
                    } else {
                        return Err(ParseError::InvalidValue("birth year out of range"));
                    }
                } else if val.ends_with("in") {
                    if PassportField::valid_number(&val.replace("in", ""), 59, 76) {
                        return Ok(());
                    } else {
                        return Err(ParseError::InvalidValue("birth year out of range"));
                    }
                }
                return Err(ParseError::InvalidValue("invalid height"));
            }
            ("hcl", val) => {
                if !val.len() == 7 {
                    return Err(ParseError::InvalidValue("invalid hair color str len"));
                }
                if !val.starts_with("#") {
                    return Err(ParseError::InvalidValue("missing #"));
                }
                i32::from_str_radix(&val.replace("#", ""), 16)
                    .map_err(|_| ParseError::InvalidValue("unable to parse pid"))?;
                Ok(())
            }
            ("ecl", val) => match val {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return Ok(()),
                _ => return Err(ParseError::InvalidValue("invalid eye color")),
            },
            ("pid", val) => {
                if val.len() != 9 {
                    return Err(ParseError::InvalidValue("invalid length"));
                }
                val.parse::<usize>()
                    .map_err(|_| ParseError::InvalidValue("unable to parse pid"))?;
                Ok(())
            }
            ("cid", _) => Ok(()),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("unable to read input");
    let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let now = std::time::Instant::now();
    let valid_passports = part_one(&input, &required_fields);
    let elapsed = now.elapsed();

    println!(
        "there are {} valid passports in {} micros",
        valid_passports,
        elapsed.as_micros()
    );

    let now = std::time::Instant::now();
    let valid_passports = part_two(&input, &required_fields);
    let elapsed = now.elapsed();

    println!(
        "there are {} valid passports in {} micros",
        valid_passports,
        elapsed.as_micros()
    );
}

fn part_one(input: &str, required_fields: &[&str]) -> usize {
    input
        .split("\n\n")
        .filter(|passport| {
            for field in required_fields {
                if !passport.contains(field) {
                    return false;
                }
            }
            return true;
        })
        .count()
}

fn part_two(input: &str, required_fields: &[&str]) -> usize {
    input
        .split("\n\n")
        .map(|passport| passport.replace("\n", " "))
        // remove all passwords that miss required fields
        .filter(|passport| {
            for field in required_fields {
                if !passport.contains(field) {
                    return false;
                }
            }
            return true;
        })
        .filter(|passport| {
            let pairs: Vec<&str> = passport.trim().split(" ").collect();
            for pair in pairs {
                if PassportField::parse(pair).is_err() {
                    return false;
                }
            }

            true
        })
        .count()
}
