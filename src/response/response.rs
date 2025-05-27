// 数据返回
use serde_derive::{Deserialize, Serialize};
use super::err::err_code_to_msg;


#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    // #[serde(rename(serialize = "code", deserialize = "code"))]
    code:i32,
    // #[serde(rename(serialize = "data", deserialize = "data"))]
    data:String,
    // #[serde(rename(serialize = "msg", deserialize = "msg"))]
    msg:String,
}

impl Response {
    pub fn new() -> Self {
        Response { 
            code:200,
            data:"".to_string(), 
            msg:"success".to_string(),
        }
    }
    // 错误返回
    pub fn err(&mut self,code:i32){
        self.code = code;
        self.msg = err_code_to_msg(self.code).to_owned();
    }
    // 正常返回
    pub fn success(&mut self,data:String){
        // println!("data:{:?}",data);
        self.data = data;
    }
    // 返回json数据结构
    pub fn generate_json(&self)->Vec<u8>{
        let json: String = serde_json::to_string(&self).unwrap();
        return json.as_bytes().to_vec();
    }
    // 返回数据结构
    pub fn generate_static(&self)->Vec<u8>{
        return "..".as_bytes().to_vec();
    }
}