// 数据返回
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Request<T>{
    body:T
}

impl <T>Request<T> {
    pub fn new(body: T) -> Request<T> {
        Request {
            body: body,
        }
    }
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }
    #[inline]
    pub fn body_mut(&mut self) -> &mut T {
        &mut self.body
    }
    #[inline]
    pub fn into_body(self) -> T {
        self.body
    }
}
