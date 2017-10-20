use xmltree::Element;

pub fn query_path<'a>(root: &Option<&'a Element>, path: &[&str]) -> Option<&'a Element> {
    if let &Some(node) = root {
        let len = path.len();
        if len == 0 {
            return None;
        } else if len == 1 {
            println!("{}", path[0]);
            return node.get_child(path[0]);
        } else {
            println!("{}", path[0]);
            return query_path(&node.get_child(path[0]), &path[1..]);
        }
    }
    None
}

pub fn query<'a>(root: &'a Element, path: &str) -> Option<&'a Element> {
    let nodes: Vec<&str> = path.split('/').collect::<Vec<_>>();
    query_path(&Some(root), &nodes)
}

#[cfg(test)]
mod tests {
    use data::bench::XML;
    use xmltree::Element;
    use fb::FictionBook;

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

    #[test]
    fn query() {
        let fb = FictionBook::new(XML.as_bytes()).unwrap();
        assert!(super::query(&fb.root, "description").is_some());
        assert!(super::query(&fb.root, "description/title-info").is_some());
        assert!(super::query(&fb.root, "description/title-info/author").is_some());
    }


}
