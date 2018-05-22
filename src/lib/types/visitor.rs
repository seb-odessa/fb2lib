

pub trait Visitor<'a> {
    type Type;
    fn visit(&mut self, target: &Self::Type);
    fn get_visited(&self) -> usize;
    fn get_accepted(&self) -> usize;
    fn get_already_known(&self) -> usize;
//    fn report(&self){
//        println!("Total books was visited {}, accepted by filter {} items, already known before {}.",
//                 self.get_visited(),
//                 self.get_accepted(),
//                 self.get_already_known()
//        );
//    }
}

pub trait MutVisitor<'a> { // @todo eliminate mutability
    type Type;
    fn visit(&mut self, target: &mut Self::Type);
    fn get_visited(&self) -> usize;
    fn get_accepted(&self) -> usize;
    fn get_already_known(&self) -> usize;
    fn report(&self){
        println!("Total books was visited {}, accepted {} already known {}.",
                 self.get_visited(),
                 self.get_accepted(),
                 self.get_already_known()
        );
    }
}