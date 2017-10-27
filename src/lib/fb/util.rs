
use xmltree::Element;

pub trait HasNew<T> {
    fn new(value: &str) -> T;
}
pub fn load<T: HasNew<T>>(root: &Element, tag: &str) -> Option<T> {
    if let Some(node) = root.get_child(tag) {
        return Some(T::new(&node.text.clone().unwrap_or_default()));
    }
    None
}

pub trait HasFrom<T> {
    fn from(element: &Option<&Element>) -> Option<T>;
}
pub fn from<T: HasFrom<T>>(root: &Element, tag: &str) -> Option<T> {
    return T::from(&root.get_child(tag));
}

pub fn load_all<T: HasFrom<T>>(node: &Element, tag: &str) -> Vec<T> {
    let mut items = Vec::new();
    for element in &node.children {
        if element.name.to_lowercase() == tag {
            if let Some(author) = from(&node, tag) {
                items.push(author);
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
    fn genre() {
        let xmltree = Element::parse(XML.as_bytes()).unwrap();
        let description = xmltree.get_child("description").unwrap();
        let title_info = description.get_child("title-info").unwrap();
        let genre = title_info.get_child("genre");
        assert!(genre.is_some());
        assert_eq!(Some(String::from("sf_space")), genre.unwrap().text);
    }
}
