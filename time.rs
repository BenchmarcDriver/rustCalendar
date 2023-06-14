mod time {

    use std::time::{SystemTime, UNIX_EPOCH};

    const DAY_IN_SEC: i64 = 60 * 60 * 24;
    const HOUR_IN_SEC: i16 = 60 * 60;
    const LEAP_YEAR_IN_DAYS: i16 = 366;
    const YEAR_IN_DAYS: i16 = 365;

    //fn main() {println!("{:?}", get_date());}


    fn get_date() -> (i16, u8, i16, u8, u8){
        let v: (i16, i16);
        v = year(now_in_days());
        let (year, days) = v;
        //println!("{} {} {} - {}:{}", year, month(year, days), day(year, days), hours(), minutes());
        return (year, month(year, days), day(year, days), hours(), minutes())
    }

    fn now() -> i64{
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(dur) => dur.as_secs() as i64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    fn is_leap_year(year: i16) -> bool {
        if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {true}
        else {false}
    }
    fn now_in_days() -> i16{
        ((now() / DAY_IN_SEC) + 1) as i16
    }
    fn year(mut days: i16) -> (i16,i16){
        let mut year: i16 = 1970;
        while is_leap_year(year) && days >=366 || !is_leap_year(year) && days >=365 {
            year = year + 1;
            if is_leap_year(year) {days = days - LEAP_YEAR_IN_DAYS}
            else {days = days - YEAR_IN_DAYS}
        }
        return (year, days);
    }
    fn month(year: i16, days: i16) -> u8 {
        if days < 31 {return 1}
        else if is_leap_year(year) && days < 60 || is_leap_year(year) == false && days < 59 {return 2}
        else if is_leap_year(year) && days < 91 || is_leap_year(year) == false && days < 90 {return 3}
        else if is_leap_year(year) && days < 121 || is_leap_year(year) == false && days < 120 {return 4}
        else if is_leap_year(year) && days < 152 || is_leap_year(year) == false && days < 151 {return 5}
        else if is_leap_year(year) && days < 182 || is_leap_year(year) == false && days < 181 {return 6}
        else if is_leap_year(year) && days < 213 || is_leap_year(year) == false && days < 212 {return 7}
        else if is_leap_year(year) && days < 244 || is_leap_year(year) == false && days < 243 {return 8}
        else if is_leap_year(year) && days < 274 || is_leap_year(year) == false && days < 273 {return 9}
        else if is_leap_year(year) && days < 305 || is_leap_year(year) == false && days < 304 {return 10}
        else if is_leap_year(year) && days < 335 || is_leap_year(year) == false && days < 334 {return 11}
        else if is_leap_year(year) && days < 366 || is_leap_year(year) == false && days < 365 {return 12}
        else {panic!("to many days in a Year")}

    }
    fn day(year: i16, days: i16) -> i16{
        if is_leap_year(year) {
            match month(year, days) {
                1 => days,
                2 => days - 31,
                3 => days - 60,
                4 => days - 91,
                5 => days - 121,
                6 => days - 152,
                7 => days - 182,
                8 => days - 213,
                9 => days - 244,
                10 => days - 274,
                11 => days - 305,
                12 => days - 335,
                _ => panic!("Missing correct month")
            }
        }
        else {
            match month(year, days) {
                1 => days,
                2 => days - 31,
                3 => days - 59,
                4 => days - 90,
                5 => days - 120,
                6 => days - 151,
                7 => days - 181,
                8 => days - 212,
                9 => days - 243,
                10 => days - 273,
                11 => days - 304,
                12 => days - 334,
                _ => panic!("Missing correct month")
            }
        }
    }
    fn hours() -> u8{
        (now() % DAY_IN_SEC / HOUR_IN_SEC as i64) as u8
    }
    fn minutes() -> u8{
        (now() % DAY_IN_SEC % HOUR_IN_SEC as i64 / 60) as u8
    }
}