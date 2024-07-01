# Library Canister

This project is inspired by the ICP Hello World Rust canister project.
The idea was to give a real world use case in order to get basic knowledge of canister smart contracts with ICP.


### Running the Project

After the IDE has opened, run `dfx deploy` in the terminal to deploy the frontend and backend. 
Click on the first green link at the end of the output to see your canister's frontend in the browser.
To interact with the backend canister, click on the second green link.

For interactive development of the frontend canister, you can also start a node server by running `npm start`.
You can find your canister's frontend running under http://localhost:8080.

If you make changes to the backend canister, remember to call `dfx deploy` first; it suffices to reload the frontend canister's webpage to reflect the changes you've made.
If your environment was restarted or has been inactive over some time, you might need to run `dfx start --background` before running `dfx deploy`.

## Testing your Project

To run the [backend tests](/src/icp_hello_world_rust_backend/src/lib.rs) for your backend canister, first run `dfx build` to build the canister Wasm, and then `cargo test`.
If the canisters have not yet been created, run `dfx canister create --all` before `dfx build`.


## Documentation

As of today, here are the functionalities of the library as per the did config file: 

``` 
type Book = record {
    title: text;
    author: text;
    is_borrowed: bool;
};

service : {
    "add_book": (text, text) -> ();
    "get_book": (text) -> (opt Book) query;
    "borrow_book": (text) -> (bool);
    "return_book": (text) -> (bool);
    "get_all_books": () -> (vec Book) query;
    "get_available_books": () -> (vec Book) query;
    "get_borrowed_books": () -> (vec Book) query;
}
```

