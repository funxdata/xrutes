#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::rout::inner::RuteInner;
use http::Method;
// use xcores::application::app;
use crate::application::app;
pub struct Rute{
    len:Option<usize>,     //路由长度
    pub nodes:RuteInner,    // 
    pub hooks:Vec<fn(&mut app::Application)>,  // 路由节点
}

impl Rute {
    pub fn new() -> Self {
        Rute{
            len:None,
            nodes:RuteInner::new(),
            hooks:Vec::new()
        }
    }
    // 绑定
    pub fn get(&mut self,uri:&str,F:fn(&mut app::Application)){
        self.bind(uri, Method::GET, F)
    }
    pub fn post(&mut self,uri:&str,F:fn(&mut app::Application)){
        self.bind(uri, Method::POST, F)
    }
    pub fn put(&mut self,uri:&str,F:fn(&mut app::Application)){
        self.bind(uri, Method::PUT, F)
    }
    pub fn delete(&mut self,uri:&str,F:fn(&mut app::Application)){
        self.bind(uri, Method::DELETE, F)
    }
    pub fn patch(&mut self,uri:&str,F:fn(&mut app::Application)){
        self.bind(uri, Method::PATCH, F)
    }
    fn bind(&mut self,path:&str,mt:Method,F:fn(&mut app::Application)){
        let mut hook_id = 0;
        match self.len {
            Some(len)=>{
                self.hooks.insert(len, F);
                self.len = Some(len+1);
                hook_id = len +1;
            },
            None=>{
                self.len = Some(1);
                hook_id = 1;
                self.hooks.insert(0, F);
            }
        }
        self.nodes.insert(path.to_string(), mt, hook_id);
    }

     // 匹配内容
     pub fn matching(&self,uri:&str,mt:&Method)->Option<usize>{
        let resData= self.nodes.search(uri.to_string(), mt);
        match resData {
            Ok((id,_))=>{
                return Some(id);
            }
            Err(e)=>{
                println!("{:?}",e);
                return None;
            }
        }
   }
}