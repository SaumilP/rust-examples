// Sample application demonstrating how interface can be created...

struct Book {
    title : String,
    author: String,
}

struct Log {
    wood_type : String,
}

trait Burns {
    fn burn(&self);
}

impl Burns for Book {
    fn burn(&self) {
        println!("The book \"{}\" by {} is burning!", self.title, self.author);
    }
}

impl Burns for Log {
     fn burn(&self) {
         println!("The {} log is burnging!", self.wood_type);
     }
}

struct Incunable(Book);
impl Burns for Incunable {
    fn burn(&self) {
        let book : &Book = match *self {
            Incunable(ref book) => book
        };
        println!("The incunable \"{}\" by {} is burning!", book.title, book.author);
    }
}

// example of interface 
fn start_fire<T: Burns>(item : T) {
    item.burn();
}

fn main() {
    let lg = Log {
        wood_type : "Oak".to_string(),
    };

    let kamakazi_attacks = Book {
        title: "Kamakazi attacks of World War II".to_string(),
        author: "Robin L. Rielly".to_string(),
    };

    let okinawa_45 = Book {
        title: "Okinawa 45".to_string(),
        author: "Gordon L. Rottman".to_string(),
    };

    let incunable = Incunable(okinawa_45);

    // lets start burning the oak log!!!
    start_fire(lg);

    // Can we burn books !!!
    start_fire(kamakazi_attacks);
    start_fire(incunable);
}