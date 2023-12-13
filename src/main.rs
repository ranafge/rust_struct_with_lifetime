struct Book {
    title: String,
}

struct Library<'a> {
    book: &'a Book,
}
impl<'a> Library<'a> {
    fn add_book(&mut self, book: &'a Book) -> String{
        self.book = book;
        println!("{} book is added", self.book.title);
        self.book.title.clone()      
    }
}

fn main() {
   let book = Book{title: "Book title".to_string()};
   let mut library = Library{
    book:&book
   };
   println!("{:?}", Library::add_book(&mut library, &book));
}
