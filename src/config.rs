/**
idea of mod is single place for configuration data
*/

//slash in the end is required
//used in tests of file functions
pub const TEMPORARY_FOLDER_PATH: &str = "./tmp/";

//slash in the end is required
//used in tests to store client files
pub const CLIENT_TEMPORARY_FOLDER_PATH: &str = "./tmp_client/";

//slash in the end is required
//used in tests to store server files
pub const SERVER_TEMPORARY_FOLDER_PATH: &str = "./tmp_server/";



/**expected format is "127.0.0.1:3000"
without http(s)://
*/
pub const SERVER_ADDRESS: &str = "127.0.0.1:3000";

/**either https:// or http://
atm only http:// is supported
*/
pub const PROTOCOL: &str = "http://";//todo may be use enum http::Uri::Protocol? Does it worth to create additional dependency for this enum using?

pub const UPLOAD_FILE_METHOD: &str = "/upload_file";