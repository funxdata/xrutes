use crate::response::response::Response;
pub struct Application{
    // header:http::request::Request<u8>,
    pub request:http::Request<Vec<u8>>,
    pub reponse:Response,
}
impl Application {
    pub fn new() -> Self {
        let defaul_req = http::Request::builder()
        .uri("https://www.funxdata.com");
        let body = "0".as_bytes().to_vec().into();
        let req = defaul_req.body(body).unwrap();
        Application { 
            request:req, 
            reponse:Response::new(),
        }
    }
}