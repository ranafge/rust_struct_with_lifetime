///Create two structs, Book and Library,
/// where Library holds a reference to a Book.
/// Implement a function add_book that tries to add a Book to a Library,
/// but intentionally make a mistake with lifetimes to create a compile-time error.
/// Explain why the error occurs.


struct Book {
    title: String,
}

struct Library<'a> {
    book: &'a Book,
}
impl<'a> Library<'a> {
    fn add_book(&mut self, book: &'a Book) {
        self.book = book
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_book() {
        let book = Book {
            title: String::from("Rust Programming"),
        };
        let mut library = Library { book: &book };
        Library::add_book(
            &mut library,
            &Book {
                title: String::from("New Book"),
            },
        );
    }
}
