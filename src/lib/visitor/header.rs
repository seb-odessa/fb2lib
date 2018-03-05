use zip::ZipFile;
use archive;
use tools::into_utf8;
use algorithm;

pub struct Header{
    counter: usize,
}
impl Header {
    pub fn new() -> Self {
        Header {
            counter: 0,
        }
    }
}
impl <'a> algorithm::Visitor<ZipFile<'a>> for Header {
    fn visit(&mut self, zip: &mut ZipFile) {
        self.counter += 1;
        println!("{:16}{:10}{:10}",zip.name(),zip.size(),zip.compressed_size());
        if let Some(xml) = archive::load_header(zip).and_then(into_utf8).ok(){
            println!("{}", xml);
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        println!("self.counter: {}", self.counter);
    }
}
