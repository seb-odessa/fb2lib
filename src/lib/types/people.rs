use std::convert::From;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct People {
    first_name: String,
    middle_name: String,
    last_name: String,
    nickname: String,
}
impl From<(String, String,String, String)> for People {
    fn from(src: (String, String,String, String)) -> Self {
        People {
            first_name: src.0,
            middle_name: src.1,
            last_name: src.2,
            nickname: src.3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::People;

    #[test]
    fn people_from_tuple() {
        let src = (String::from("First"), String::from("Middle"), String::from("Last"), String::from("Nickname"));
        let people = People::from(src.clone());
        assert_eq!(people.first_name, src.0);
        assert_eq!(people.middle_name, src.1);
        assert_eq!(people.last_name, src.2);
        assert_eq!(people.nickname, src.3);

    }

}