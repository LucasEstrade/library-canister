pub mod types;
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;
use types::Book;

thread_local! {
    static LIBRARY: std::cell::RefCell<HashMap<String, Book>> = std::cell::RefCell::new(HashMap::new());
}

#[init]
fn init() {
    ic_cdk::print("Library canister initialized.");
}

#[update]
fn add_book(title: String, author: String) -> String {
    LIBRARY.with(|library| {
        library.borrow_mut().insert(title.clone(), Book {
            title: title.clone(),
            author,
            is_borrowed: false,
        });
    });
    format!("Book '{}' added.", title)
}

#[query]
fn get_book(title: String) -> Option<Book> {
    LIBRARY.with(|library| {
        library.borrow().get(&title).cloned()
    })
}

#[update]
fn borrow_book(title: String) -> bool {
    LIBRARY.with(|library| {
        if let Some(book) = library.borrow_mut().get_mut(&title) {
            if !book.is_borrowed {
                book.is_borrowed = true;
                return true;
            }
        }
        false
    })
}

#[update]
fn return_book(title: String) -> bool {
    LIBRARY.with(|library| {
        if let Some(book) = library.borrow_mut().get_mut(&title) {
            if book.is_borrowed {
                book.is_borrowed = false;
                return true;
            }
        }
        false
    })
}

#[query]
fn get_all_books() -> Vec<Book> {
    LIBRARY.with(|library| {
        library.borrow().values().cloned().collect()
    })
}

#[query]
fn get_available_books() -> Vec<Book> {
    LIBRARY.with(|library| {
        library
            .borrow()
            .values()
            .filter(|book| !book.is_borrowed)
            .cloned()
            .collect()
    })
}

#[query]
fn get_borrowed_books() -> Vec<Book> {
    LIBRARY.with(|library| {
        library
            .borrow()
            .values()
            .filter(|book| book.is_borrowed)
            .cloned()
            .collect()
    })
}


#[cfg(test)]
mod tests {
    use candid::{decode_one, encode_args, encode_one, Error, Principal};
    use pocket_ic::{PocketIc, WasmResult};
    use std::fs;

    use crate::types::Book;

    const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/icp_hello_world_rust_backend.wasm";

    fn setup() -> (PocketIc, Principal) {
        let pic = PocketIc::new();

        let backend_canister = pic.create_canister();
        pic.add_cycles(backend_canister, 2_000_000_000_000); // 2T Cycles
        let wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
        pic.install_canister(backend_canister, wasm, vec![], None);

        let add_book_calls = vec![
            ("The Great Gatsby", "F. Scott Fitzgerald"),
            ("1984", "G. Orwell"),
            ("Le Petit Prince", "A. de Saint-Exupéry"),
        ];

        for (title, author) in add_book_calls {
            let result = pic.update_call(
                backend_canister,
                Principal::anonymous(),
                "add_book",
                encode_args((title, author)).unwrap(),
            );
            if let Ok(WasmResult::Reply(_)) = result {
                println!("Successfully added book: {} by {}", title, author);
            } else {
                panic!("Failed to add book: {} by {}. Result: {:?}", title, author, result);
            }
        }
        
        (pic, backend_canister)
    }


    #[test]
    fn test_borrow_book(){
        let (pic, backend_canister) = setup();

        let result = pic.update_call(backend_canister, Principal::anonymous(), "borrow_book", encode_one("1984").unwrap());

        if let Ok(WasmResult::Reply(response)) = result {
            let books: Result<bool, Error> = decode_one(&response);
            match books {
                Ok(b) => assert_eq!(b, true),
                Err(e) => panic!("Error decoding response: {:?}", e),
            }
        } else {
            panic!("Expected reply, got: {:?}", result);
        }
    }

    #[test]
    fn test_get_book(){
        let (pic, backend_canister) = setup();

        let book_result = pic.query_call(backend_canister, Principal::anonymous(), "get_book", encode_one("1984").unwrap());

        if let Ok(WasmResult::Reply(response)) = book_result {
            println!("Response: {:?}", response); // Print response for debugging
            let book: Option<Book> = decode_one(&response).unwrap();
            if let Some(book) = book {
                assert_eq!(book.title, "1984");
                assert_eq!(book.author, "G. Orwell");
                assert_eq!(book.is_borrowed, false);
            } else {
                panic!("Expected Some(Book), got None.");
            }
        } else {
            panic!("Expected reply, got: {:?}", book_result);
        }
    }

    #[test]
    fn test_get_all_books(){
        let (pic, backend_canister) = setup();

        let result = pic.query_call(backend_canister, Principal::anonymous(), "get_all_books", encode_one("").unwrap());

        if let Ok(WasmResult::Reply(response)) = result {
            let books: Option<Vec<Book>> = decode_one(&response).unwrap();
            if let Some(books) = books{
                assert_eq!(books.len(), 3);
            } else {
                panic!("Expected Some(Vec<Book>), got None.");
            }
        } else {
            panic!("Expected reply, got: {:?}", result);
        }
    }


    #[test]
    fn test_get_borrowed_books(){
        let (pic, backend_canister) = setup();

        let borrow = pic.update_call(backend_canister, Principal::anonymous(), "borrow_book", encode_one("1984").unwrap());

        if let Ok(WasmResult::Reply(response)) = borrow {
            let borrow: bool = decode_one(&response).unwrap();
            assert_eq!(borrow, true);
        } else {
            panic!("Expected reply when borrowing book, got: {:?}", borrow);
        }

        let borrowed_books = pic.query_call(backend_canister, Principal::anonymous(), "get_borrowed_books", encode_one("").unwrap());
        if let Ok(WasmResult::Reply(response)) = borrowed_books {
            let borrowed_books: Vec<Book> = decode_one(&response).unwrap();
            assert_eq!(borrowed_books.len(), 1);
            assert_eq!(borrowed_books[0].title, "1984");
        } else {
            panic!("Expected reply when querying borrowed books, got: {:?}", borrowed_books);
        }
    }

    #[test]
    fn test_get_available_books(){
        let (pic, backend_canister) = setup();

        let borrow = pic.update_call(backend_canister, Principal::anonymous(), "borrow_book", encode_one("1984").unwrap());

        if let Ok(WasmResult::Reply(response)) = borrow {
            let borrow: bool = decode_one(&response).unwrap();
            assert_eq!(borrow, true);
        } else {
            panic!("Expected reply when borrowing book, got: {:?}", borrow);
        }

        let available_books = pic.query_call(backend_canister, Principal::anonymous(), "get_available_books", encode_one("").unwrap());
        if let Ok(WasmResult::Reply(response)) = available_books {
            let available_books: Vec<Book> = decode_one(&response).unwrap();
            assert_eq!(available_books.len(), 2);
        } else {
            panic!("Expected reply when querying available books, got: {:?}", available_books);
        }
    }

    #[test]
    fn test_add_book() {
        let (pic, backend_canister) = setup();

        let result = pic.update_call(
            backend_canister,
            Principal::anonymous(),
            "add_book",
            encode_args(("Le Rouge et le Noir", "Stendhal")).unwrap(),
        );

        if let Ok(WasmResult::Reply(_)) = result {

        let book_result = pic.query_call(backend_canister,
            Principal::anonymous(), "get_book", 
        encode_one("Le Rouge et le Noir").unwrap());

        if  let Ok(WasmResult::Reply(response)) = book_result {
            let book: Option<Book> = decode_one(&response).unwrap();
            if let Some(book) = book {
                assert_eq!(book.title, "Le Rouge et le Noir");
                assert_eq!(book.author, "Stendhal");
                assert_eq!(book.is_borrowed, false);
            } else {
                panic!("Expected Some(Book), got None.");
            }
        } else {
            panic!("Expected reply, got: {:?}", result);
        }
        }
    }   
}
