use zip::ZipFile;
use archive;
use tools;
use algorithm;
use fb2parser::FictionBook;

pub enum Show {
    Zip,
    Xml,
    Fb2,
    Inf,
    Bad,
}

pub struct Header{
    counter: usize,
    output: Show,
}
impl Header {
    pub fn new(output: Show) -> Self {
        Header {
            counter: 0,
            output: output,
        }
    }
    fn zip<'a>(&self, zip: &mut ZipFile) -> Option<String> {
        Some(format!(
            "{:10} ({:10}) : Size {:8} Original Size: {:8} crc32: {:x}, offset: {}\n",
            &zip.name(),
            &zip.compression(),
            &zip.compressed_size(),
            &zip.size(),
            &zip.crc32(),
            &zip.offset()
        ))
    }
    fn xml<'a>(&self, zip: &mut ZipFile) -> Option<String> {
        archive::load_xml(zip).ok().and_then(|xml| Some(
            format!("{}\n", xml))
        )
    }
    fn fb2<'a>(&self, zip: &mut ZipFile) -> Option<String> {
        archive::load_fb2(zip).ok().and_then(|a| Some(
            format!("{:#?}\n", a))
        )
    }
    fn inf<'a>(&self, zip: &mut ZipFile) -> Option<String> {
        archive::load_fb2(zip).ok().and_then(|a| Some(
            format!("{:12} : {} : {}\n", zip.name(), a.get_book_title(), fmt(a.get_book_authors())))
        )
    }
    fn bad<'a>(&self, zip: &mut ZipFile) -> Option<String> {
        archive::load_fb2(zip).ok().and_then(|_| Some(String::new()))
    }    
}
impl <'a> algorithm::Visitor<ZipFile<'a>> for Header {
    fn visit(&mut self, zip: &mut ZipFile) {
        self.counter += 1;
        let result = match self.output {
            Show::Zip => self.zip(zip),
            Show::Xml => self.xml(zip),
            Show::Fb2 => self.fb2(zip),
            Show::Inf => self.inf(zip),
            Show::Bad => self.bad(zip),
        };
        match result {
            Some(string) => print!("{}", string),
            None => println!("Filed to process {} file.", zip.name()),
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        println!("Handled {} files in archive", self.counter);
    }
}


fn fmt(authors: Vec<(String,String,String,String)>) -> String {
    let mut result = Vec::new();
    for author in authors.into_iter() {
        result.push(format!("{} {} {} {}", author.0, author.1, author.2, author.3));
    }
    result.join(", ")
}
