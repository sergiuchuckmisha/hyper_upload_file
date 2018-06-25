# hyper_upload_file

basic idea is to create prototype for upload file with hyper

## Prerequisites:
1. [nightly] rust version activated


## Running the example
1. run server with command 

       $ cargo run
or       

       $ cargo run --package hyper_upload_file --bin hyper_upload_file
  
2. run test example with command

       $ cargo test --package hyper_upload_file --bin hyper_upload_file client_post_request::tests::test_make_request_from_file -- --nocapture --exact

or

       $ cargo test --package hyper_upload_file --bin hyper_upload_file client_post_request::tests::test_upload_folder -- --nocapture --exact


## Most important points in code:
1. server at `src/main.rs:main()`
2. client at `src/client_post_request.rs`
3. config at `src/config.rs

## Configuration:
 * can be found at at src/config.rs
 * `TEMPORARY_FOLDER_PATH` - is used for files io tests at `mod files_io_api;`
 * `CLIENT_TEMPORARY_FOLDER_PATH` - where client will look cor files to upload them; is used at test `src/client_post_request.rs:test_upload_folder`
 * `SERVER_TEMPORARY_FOLDER_PATH` - where server will store uploaded files
 * `SERVER_ADDRESS` - api address; localhost is expected
 * `SERVER_ADDRESS_PREFIX` - either `https://` or `http://`; atm only `http://` is supported
 
[nightly]: https://doc.rust-lang.org/1.15.0/book/nightly-rust.html