#[derive(PartialEq, Debug)]
pub enum TItem<'a> {
    KeyValuePair(TKeyValuePair<'a>),
    String(&'a str)
}

#[derive(PartialEq, Debug)]
pub struct TKeyValuePair<'a> {
    key: &'a str,
    value: &'a str
}

#[derive(PartialEq, Debug)]
pub struct TArray<'a>(Vec<TItem<'a>>);

/*
pub fn parse(document: &str) -> TArray {
    parse_array(document)
}
*/

fn parse_array(fragment: &str) -> TArray {
    let items: Vec<TItem> = fragment.lines().map(|item| {
        if item.contains('\t') {
            TItem::KeyValuePair(parse_kvp(item))
        } else {
            TItem::String(item)
        }
    }).collect();
    TArray(items)
}

fn parse_kvp(line: &str) -> TKeyValuePair {
    let kvp: Vec<&str> = line.split('\t').filter(|item| item.len() > 0).collect();
    TKeyValuePair { key: kvp[0], value: kvp[1] }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simple_key_value_pair() {
        let document = "key\tvalue";
        let expected = TKeyValuePair { key: "key", value: "value" };
        assert_eq!(expected, parse_kvp(document));
    }

    #[test]
    fn simple_key_value_pair_with_multiple_tabs() {
        let document = "key\t\t\tvalue";
        let expected = TKeyValuePair { key: "key", value: "value" };
        assert_eq!(expected, parse_kvp(document));
    }

    #[test]
    fn simple_array() {
        let document = "item1\nitem2";
        let expected = TArray(vec![TItem::String("item1"), TItem::String("item2")]);
        assert_eq!(expected, parse_array(document));
    }

    #[test]
    fn array_of_kvp() {
        let document = "key\tvalue\nkey2\tvalue2";
        let expected = TArray(vec![TItem::KeyValuePair(TKeyValuePair {key: "key", value: "value"}),
                                   TItem::KeyValuePair(TKeyValuePair {key: "key2", value: "value2"})]);
        assert_eq!(expected, parse_array(document));
    }
}
