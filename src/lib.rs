use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum StringElement<'a> {
    Digits(&'a str),
    Characters(&'a str),
}

struct StringElementIter<'a> {
    str: &'a str,
}

impl<'a> Iterator for StringElementIter<'a> {
    type Item = StringElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.str.chars().next() {
            None => None,
            Some(c) => {
                if c.is_digit(10) {
                    match self.str.find(|c: char| !c.is_digit(10)) {
                        None => {
                            let result = self.str;
                            self.str = "";
                            Some(StringElement::Digits(result))
                        }
                        Some(len) => {
                            let (result, rest) = self.str.split_at(len);
                            self.str = rest;
                            Some(StringElement::Digits(result))
                        }
                    }
                } else {
                    match self.str.find(|c: char| c.is_digit(10)) {
                        None => {
                            let result = self.str;
                            self.str = "";
                            Some(StringElement::Characters(result))
                        }
                        Some(len) => {
                            let (result, rest) = self.str.split_at(len);
                            self.str = rest;
                            Some(StringElement::Characters(result))
                        }
                    }
                }
            }
        }
    }
}

fn cmp_digit_str(lhs: &str, rhs: &str) -> Ordering {
    fn contains_only_digits(s: &str) -> bool {
        s.chars().find(|c| !c.is_digit(10)) == None
    }
    debug_assert!(contains_only_digits(lhs));
    debug_assert!(contains_only_digits(rhs));

    #[inline]
    fn trim_leading_zeroes(s: &str) -> &str {
        s.chars()
            .position(|c| c != '0')
            .map_or("", |pos| &s[pos..])
    }
    let lhs = trim_leading_zeroes(lhs);
    let rhs = trim_leading_zeroes(rhs);

    match lhs.len().cmp(&rhs.len()) {
        Ordering::Equal => lhs.cmp(&rhs),
        result @ _ => result
    }
}

pub fn natural_cmp(lhs: &str, rhs: &str) -> Ordering {
    let first_non_equal = StringElementIter { str: lhs }
        .zip(StringElementIter { str: rhs })
        .map(|pair| match pair {
            (StringElement::Digits(a), StringElement::Digits(b)) => cmp_digit_str(a, b),
            (StringElement::Characters(a), StringElement::Characters(b)) |
            (StringElement::Characters(a), StringElement::Digits(b)) |
            (StringElement::Digits(a), StringElement::Characters(b)) => a.cmp(b),
        })
        .find(|&o| o != Ordering::Equal);

    match first_non_equal {
        Some(o) => o,
        None => lhs.cmp(rhs),
    }
}

#[cfg(test)]
mod tests {
    use super::{cmp_digit_str, StringElement, StringElementIter, natural_cmp};
    use std::cmp::Ordering;

    #[test]
    fn cmp_digit_str_different_lengths() {
        assert_eq!(cmp_digit_str("12", "4"), Ordering::Greater);
        assert_eq!(cmp_digit_str("4", "12"), Ordering::Less);
        assert_eq!(cmp_digit_str("43623", "3445"), Ordering::Greater);
        assert_eq!(cmp_digit_str("3445", "43623"), Ordering::Less);
    }

    #[test]
    fn cmp_digit_str_empty_slices() {
        assert_eq!(cmp_digit_str("12", ""), Ordering::Greater);
        assert_eq!(cmp_digit_str("", "12"), Ordering::Less);
        assert_eq!(cmp_digit_str("", ""), Ordering::Equal);
    }

    #[test]
    fn cmp_digit_str_same_length() {
        assert_eq!(cmp_digit_str("125", "114"), Ordering::Greater);
        assert_eq!(cmp_digit_str("114", "125"), Ordering::Less);
        assert_eq!(cmp_digit_str("114", "114"), Ordering::Equal);
        assert_eq!(cmp_digit_str("7654321", "1234567"), Ordering::Greater);
        assert_eq!(cmp_digit_str("1234567", "7654321"), Ordering::Less);
        assert_eq!(cmp_digit_str("7654321", "7654321"), Ordering::Equal);
    }

    #[test]
    fn cmp_digit_str_with_leading_zeroes() {
        assert_eq!(cmp_digit_str("00", "00"), Ordering::Equal);
        assert_eq!(cmp_digit_str("00", "000"), Ordering::Equal);
        assert_eq!(cmp_digit_str("000", "00"), Ordering::Equal);
        assert_eq!(cmp_digit_str("12", "04"), Ordering::Greater);
        assert_eq!(cmp_digit_str("04", "12"), Ordering::Less);
        assert_eq!(cmp_digit_str("0043623", "0003445"), Ordering::Greater);
        assert_eq!(cmp_digit_str("03445", "00043623"), Ordering::Less);
    }

    #[test]
    fn cmp_digit_str_huge_numbers() {
        assert_eq!(cmp_digit_str("897634568796798345679367842586285634567823765843278564325678234567845235684236584236874",
                                 "897634568796798345679367842586285634567823765843278564325678234567845235684236584236874"),
                   Ordering::Equal);
        assert_eq!(cmp_digit_str("897634568796798345679367842586285634567823765843278564325678234567845235684236584236874",
                                 "897634568796798345679367842586285634567823765843278564325678234567845235684236484236874"),
                   Ordering::Greater);
        assert_eq!(cmp_digit_str("897634568796798345679367842586285634567823765843278564325678234567845235684236484236874",
                                 "897634568796798345679367842586285634567823765843278564325678234567845235684236584236874"),
                   Ordering::Less);
        assert_eq!(cmp_digit_str("3478966924529346235565656555555647536456734562364523451243456236482537448965798569795687859697567",
                                 "1467560934792346753466794590787506879056673468756872346578634756734967549786237984578923459346574"),
                   Ordering::Greater);
        assert_eq!(cmp_digit_str("1467560934792346753466794590787506879056673468756872346578634756734967549786237984578923459346574",
                                 "3478966924529346235565656555555647536456734562364523451243456236482537448965798569795687859697567"),
                   Ordering::Less);
        assert_eq!(cmp_digit_str("57684568935646567346758923749678904578906458903450934567030976907836890676789096079607",
                                 "5768456893564656734675892374967890457890645890345093456703097690783689067678"),
                   Ordering::Greater);
        assert_eq!(cmp_digit_str("5768456893564656734675892374967890457890645890345093456703097690783689067678",
                                 "57684568935646567346758923749678904578906458903450934567030976907836890676789096079607"),
                   Ordering::Less);
    }

    #[test]
    fn split_empty_string() {
        assert_eq!(None, StringElementIter { str: "" }.next());
    }

    #[test]
    fn split_single_element_strings() {
        assert_eq!(Some(StringElement::Digits("123")),
                   StringElementIter { str: "123" }.next());
        assert_eq!(Some(StringElement::Characters("xyz")),
                   StringElementIter { str: "xyz" }.next());
    }

    #[test]
    fn split_multi_element_strings() {
        let mut iter = StringElementIter { str: "12xy" };
        assert_eq!(Some(StringElement::Digits("12")), iter.next());
        assert_eq!(Some(StringElement::Characters("xy")), iter.next());
        assert_eq!(None, iter.next());

        let mut iter = StringElementIter { str: "xy12" };
        assert_eq!(Some(StringElement::Characters("xy")), iter.next());
        assert_eq!(Some(StringElement::Digits("12")), iter.next());
        assert_eq!(None, iter.next());

        let mut iter = StringElementIter { str: "xy12ab" };
        assert_eq!(Some(StringElement::Characters("xy")), iter.next());
        assert_eq!(Some(StringElement::Digits("12")), iter.next());
        assert_eq!(Some(StringElement::Characters("ab")), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn split_string_with_multibyte_characters() {
        let mut iter = StringElementIter { str: "Löwe1老虎32Léopard" };
        assert_eq!(Some(StringElement::Characters("Löwe")), iter.next());
        assert_eq!(Some(StringElement::Digits("1")), iter.next());
        assert_eq!(Some(StringElement::Characters("老虎")), iter.next());
        assert_eq!(Some(StringElement::Digits("32")), iter.next());
        assert_eq!(Some(StringElement::Characters("Léopard")), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn natural_cmp_empty_strings() {
        assert_eq!(Ordering::Equal, natural_cmp("", ""));
        assert_eq!(Ordering::Less, natural_cmp("", "a"));
        assert_eq!(Ordering::Greater, natural_cmp("a", ""));
    }

    #[test]
    fn natural_cmp_strings() {
        let mut strings: Vec<String> =
            vec![String::from("zz"), String::from("ab12cd"), String::from("ab1")];
        strings.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(vec![String::from("ab1"), String::from("ab12cd"), String::from("zz")],
                   strings);
    }

    #[test]
    fn natural_cmp_str() {
        let mut strings: Vec<&str> = vec!["zz", "ab12cd", "ab1"];
        strings.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(vec!["ab1", "ab12cd", "zz"], strings);
    }

    #[test]
    fn natural_cmp_reverse_sort() {
        let mut strings: Vec<&str> = vec!["zz", "ab12cd", "ab1"];
        strings.sort_by(|a, b| natural_cmp(b, a));
        assert_eq!(vec!["zz", "ab12cd", "ab1"], strings);
    }

    #[test]
    // Test strings taken from http://www.davekoelle.com/alphanum.html
    fn natural_cmp_alphanum_examples() {
        let mut strings = vec!["z1.doc", "z10.doc", "z100.doc", "z101.doc", "z102.doc", "z11.doc",
                               "z12.doc", "z13.doc", "z14.doc", "z15.doc", "z16.doc", "z17.doc",
                               "z18.doc", "z19.doc", "z2.doc", "z20.doc", "z3.doc", "z4.doc",
                               "z5.doc", "z6.doc", "z7.doc", "z8.doc", "z9.doc"];
        strings.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(vec!["z1.doc", "z2.doc", "z3.doc", "z4.doc", "z5.doc", "z6.doc", "z7.doc",
                        "z8.doc", "z9.doc", "z10.doc", "z11.doc", "z12.doc", "z13.doc",
                        "z14.doc", "z15.doc", "z16.doc", "z17.doc", "z18.doc", "z19.doc",
                        "z20.doc", "z100.doc", "z101.doc", "z102.doc"],
                   strings);

        let mut strings = vec!["1000X Radonius Maximus",
                               "10X Radonius",
                               "200X Radonius",
                               "20X Radonius",
                               "20X Radonius Prime",
                               "30X Radonius",
                               "40X Radonius",
                               "Allegia 50 Clasteron",
                               "Allegia 500 Clasteron",
                               "Allegia 50B Clasteron",
                               "Allegia 51 Clasteron",
                               "Allegia 6R Clasteron",
                               "Alpha 100",
                               "Alpha 2",
                               "Alpha 200",
                               "Alpha 2A",
                               "Alpha 2A-8000",
                               "Alpha 2A-900",
                               "Callisto Morphamax",
                               "Callisto Morphamax 500",
                               "Callisto Morphamax 5000",
                               "Callisto Morphamax 600",
                               "Callisto Morphamax 6000 SE",
                               "Callisto Morphamax 6000 SE2",
                               "Callisto Morphamax 700",
                               "Callisto Morphamax 7000",
                               "Xiph Xlater 10000",
                               "Xiph Xlater 2000",
                               "Xiph Xlater 300",
                               "Xiph Xlater 40",
                               "Xiph Xlater 5",
                               "Xiph Xlater 50",
                               "Xiph Xlater 500",
                               "Xiph Xlater 5000",
                               "Xiph Xlater 58"];
        strings.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(vec!["10X Radonius",
                        "20X Radonius",
                        "20X Radonius Prime",
                        "30X Radonius",
                        "40X Radonius",
                        "200X Radonius",
                        "1000X Radonius Maximus",
                        "Allegia 6R Clasteron",
                        "Allegia 50 Clasteron",
                        "Allegia 50B Clasteron",
                        "Allegia 51 Clasteron",
                        "Allegia 500 Clasteron",
                        "Alpha 2",
                        "Alpha 2A",
                        "Alpha 2A-900",
                        "Alpha 2A-8000",
                        "Alpha 100",
                        "Alpha 200",
                        "Callisto Morphamax",
                        "Callisto Morphamax 500",
                        "Callisto Morphamax 600",
                        "Callisto Morphamax 700",
                        "Callisto Morphamax 5000",
                        "Callisto Morphamax 6000 SE",
                        "Callisto Morphamax 6000 SE2",
                        "Callisto Morphamax 7000",
                        "Xiph Xlater 5",
                        "Xiph Xlater 40",
                        "Xiph Xlater 50",
                        "Xiph Xlater 58",
                        "Xiph Xlater 300",
                        "Xiph Xlater 500",
                        "Xiph Xlater 2000",
                        "Xiph Xlater 5000",
                        "Xiph Xlater 10000"],
                   strings);
    }

    #[test]
    // Test strings taken from http://personal.inet.fi/cool/operator/Human%20Sort.py
    fn natural_cmp_human_sort_example() {
        let mut strings = vec!["z7.doc",
                               "z4.doc",
                               "z10.doc",
                               "z14.doc",
                               "z101.doc",
                               "z11.doc",
                               "z8.doc",
                               "z12.doc",
                               "z13.doc",
                               "z15.doc",
                               "z1.doc",
                               "z18.doc",
                               "z19.doc",
                               "z2.doc",
                               "z20.doc",
                               "z3.doc",
                               "z100.doc",
                               "z5.doc",
                               "z17.doc",
                               "z6.doc",
                               "z102.doc",
                               "z16.doc",
                               "z9.doc",
                               "1.2.3.4.123",
                               "123.1.2.3",
                               "2.2.3.4.123",
                               "11.2.3.4.123",
                               "Ängström",
                               "Ångström",
                               "Angstrom",
                               "Ängström12.3",
                               "Ångström12",
                               "Angstrom2"];
        strings.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(vec!["1.2.3.4.123",
                        "2.2.3.4.123",
                        "11.2.3.4.123",
                        "123.1.2.3",
                        "Angstrom",
                        "Angstrom2",
                        "z1.doc",
                        "z2.doc",
                        "z3.doc",
                        "z4.doc",
                        "z5.doc",
                        "z6.doc",
                        "z7.doc",
                        "z8.doc",
                        "z9.doc",
                        "z10.doc",
                        "z11.doc",
                        "z12.doc",
                        "z13.doc",
                        "z14.doc",
                        "z15.doc",
                        "z16.doc",
                        "z17.doc",
                        "z18.doc",
                        "z19.doc",
                        "z20.doc",
                        "z100.doc",
                        "z101.doc",
                        "z102.doc",
                        "Ängström",
                        "Ängström12.3",
                        "Ångström",
                        "Ångström12"],
                   strings);
    }
}
