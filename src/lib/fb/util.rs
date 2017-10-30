use xmltree::Element;

pub trait HasFrom<T> {
    fn from(element: &Option<&Element>) -> Option<T>;
}

pub fn from<T: HasFrom<T>>(root: &Element, tag: &str) -> Option<T> {
    //    println!("from({:?}, '{}')", root, tag);
    return T::from(&root.get_child(tag));
}

pub fn all_from<T: HasFrom<T>>(root: &Element, tag: &str) -> Vec<T> {
    //    println!("all_from({:?}, '{}')", root, tag);
    let mut items = Vec::new();
    for node in &root.children {
        if node.name.to_lowercase() == tag {
            if let Some(item) = T::from(&Some(&node)) {
                items.push(item);
            }
        }
    }
    return items;
}



#[cfg(test)]
mod tests {
    use data::bench::XML;
    use xmltree::Element;

    #[test]
    fn description() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description");
        assert!(description.is_some());
    }

    #[test]
    fn title_info() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description").unwrap();
        let title_info = description.get_child("title-info");
        assert!(title_info.is_some());
    }

    #[test]
    fn all_from() {
        use fb::{Author, Nickname};
        const TEST_DATA: &'static str = r##"
        <root>
            <author><nickname>A</nickname></author>
            <author><nickname>B</nickname></author>
        </root>"##;
        let root = Element::parse(TEST_DATA.as_bytes()).unwrap();

        let authors: Vec<Author> = super::all_from(&root, "author");
        let a = Author {
            first_name: None,
            middle_name: None,
            last_name: None,
            nickname: Some(Nickname { text: String::from("A") }),
        };
        let b = Author {
            first_name: None,
            middle_name: None,
            last_name: None,
            nickname: Some(Nickname { text: String::from("B") }),
        };
        assert_eq!(vec![a, b], authors);
    }


    #[test]
    fn genre() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description").unwrap();
        let title_info = description.get_child("title-info").unwrap();
        let genre = title_info.get_child("genre");
        assert!(genre.is_some());
        assert_eq!(Some(String::from("sf_space")), genre.unwrap().text);
    }
}
