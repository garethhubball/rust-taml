#[derive(PartialEq, Debug)]
pub enum TItem<'a> {
    KeyValuePair(TKeyValuePair<'a>),
    String(&'a str),
    Array(TArray<'a>)
}

#[derive(PartialEq, Debug)]
pub struct TKeyValuePair<'a> {
    key: &'a str,
    value: Box<TItem<'a>>
}

#[derive(PartialEq, Debug)]
pub struct TArray<'a>(Vec<Box<TItem<'a>>>);

pub fn parse(document: &str) -> TArray {
    parse_array(document)
}

fn parse_array(fragment: &str) -> TArray {
    let items: Vec<Box<TItem>> = fragment.lines().map(|item| {
        if item.contains('\t') {
            Box::from(TItem::KeyValuePair(parse_kvp(item)))
        } else {
            Box::from(TItem::String(item))
        }
    }).collect();
    TArray(items)
}

fn parse_kvp(fragment: &str) -> TKeyValuePair {
    let kvp: Vec<&str> = fragment.split('\t').filter(|item| item.len() > 0).collect();
    TKeyValuePair { key: kvp[0], value: Box::from(TItem::String(kvp[1])) }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_key_value_pair() {
        let document = "key\tvalue";
        let expected = TKeyValuePair { key: "key", value: Box::from(TItem::String("value")) };
        assert_eq!(expected, parse_kvp(document));
    }

    #[test]
    fn simple_key_value_pair_with_multiple_tabs() {
        let document = "key\t\t\tvalue";
        let expected = TKeyValuePair { key: "key", value: Box::from(TItem::String("value")) };
        assert_eq!(expected, parse_kvp(document));
    }

    #[test]
    fn simple_array() {
        let document = "item1\nitem2";
        let expected = TArray(vec![Box::from(TItem::String("item1")), Box::from(TItem::String("item2"))]);
        assert_eq!(expected, parse_array(document));
    }

    #[test]
    fn array_of_kvp() {
        let document = "key\tvalue\nkey2\tvalue2";
        let expected = TArray(vec![Box::from(TItem::KeyValuePair(TKeyValuePair {key: "key", value: Box::from(TItem::String("value"))})),
                                   Box::from(TItem::KeyValuePair(TKeyValuePair {key: "key2", value: Box::from(TItem::String("value2"))}))]);
        assert_eq!(expected, parse_array(document));
    }

    #[test]
    #[ignore]
    /// TODO: Does not yet work and requires significant work to fix.
    fn kvp_with_array_value() {
        let document = "fruits\n\tapple\n\tbanana";
        let expected = TKeyValuePair {
            key: "fruits",
            value: Box::from(TItem::Array(
                TArray(
                    vec![
                        Box::from(TItem::String("apple")),
                        Box::from(TItem::String("banana"))]))) };
        assert_eq!(expected, parse_kvp(document));
    }
}