#![allow(non_snake_case)]
#![allow(dead_code)]
use http::Method;
use std::collections::HashMap;
pub struct RuteInner{
    path:Option<String>,        // 路由节点名称
    params:Vec<String>,         // 参数
    pub nodes:Vec<RuteInner>,       // 节点   
    hook_id:Option<usize>,              // 回调函数
    method:Option<Method>,      // 方法
}

impl RuteInner {
    pub fn new()->Self{
        RuteInner{
            path:None,
            params:Vec::new(),
            hook_id:None,
            nodes:Vec::new(),
            method:None,
        }
    }
    // 接收并处理path
    fn uris(&self,uri:String)->(String,String){
        let uri = uri.trim_matches('/');
        let uris: Vec<&str> = uri.split("/").collect();
        return (uris[0].to_string(),uris[1..].join("/"));
    }

    // 插入节点
    pub fn insert(&mut self,uri:String,mt:Method,id:usize){
        let (path,uris) = self.uris(uri); 
        if uris.len()<=0 &&path.len()<=0{
            if self.method!=None||self.nodes.len()>0 {
                panic!("路由重复...");
            }
            self.insert_other(mt.clone(), id);
            return;
        }
        // 匹配全节点
        if path.starts_with('*'){
            // 匹配为全节点
        }
        // 匹配参数
        if path.starts_with(':'){
            // 参数
            self.insert_param(path);
            self.insert(uris, mt, id);
            return;
        }
        let mut temp_mt: Option<Method> = None;
        if uris.starts_with(':'){
            temp_mt = Some(mt.clone());
        }
        // 匹配正常值
       let node_id=  match self.search_node(path.clone(),temp_mt){
            Some(index)=>{
                index
            }
            None=>{
                self.insert_node(path)
                
            }
        };
        self.nodes[node_id].insert(uris, mt, id)
    }
    
    // 插入节点
    fn insert_node(&mut self,path:String)->usize{
        let mut node = RuteInner::new();
        node.path = Some(path.to_string());
        let index = self.nodes.len();
        self.nodes.insert(index, node);
        return index;
    }
    // 插入参数值
    fn insert_param(&mut self,para:String){
        let para = para.trim_start_matches(":").to_string();
        self.params.push(para);
    }

    // 插入信息
    fn insert_other(&mut self,mt:Method,id:usize){
        self.method = Some(mt);
        self.hook_id = Some(id);
    }
    // 返回数据
    fn back_param(&self,params:String)->HashMap<String,String>{
        let uris: Vec<&str> = params.split("/").collect();
        let mut k:usize = 0;
        let mut param_data: HashMap<String, String> =HashMap::new(); 
        for uri in uris {
            println!("{:?}",k);
            println!("{:?}",uri);
            if  k>= self.params.len(){
                return  param_data;
            }
            param_data.insert(self.params[k].clone(),uri.to_string());
            k = k+1
        }

        return param_data;
    }
    // 查找节点
    fn search_node(&self,path:String,mt:Option<Method>)->Option<usize>{
        let mut index: usize = 0;
        for node in &self.nodes {
            // 匹配路由
            if node.path==Some(path.clone())&&node.method==mt{
                return  Some(index)
            }
            index = index +1;
        }
        return None;
    }

    // 查找
    pub fn search(&self,uri:String,mt:&Method)-> Result<(usize,HashMap<String,String>),&str>{
        let (path,uris) = self.uris(uri); 
        if uris.len()<=0 &&path.len()<=0{
            match self.hook_id {
                Some(index)=>{
                    return Ok((index,HashMap::new()));
                }
                None=>{
                    return Err("未匹配路由");
                }
            }
        }
        let mut tmp_mt: Option<Method> =None;
        if uris.len()<=0 {
            tmp_mt = Some(mt.clone())
        }
        // 正常的路由匹配
        match self.search_node(path.clone(), tmp_mt) {
            Some(index)=>{
                self.nodes[index].search(uris, mt)
            },
            None=>{
                match self.search_node(path, Some(mt.clone())){
                    Some(index)=>{
                        let params = self.nodes[index].back_param(uris);
                        match self.nodes[index].hook_id {
                            Some(hookid)=>{
                                return Ok((hookid,params));
                            }
                            None=>{
                                return Err("路由未匹配");
                            }
                        }
        
                    }
                    None=>{
                        return Err("路由未匹配");
                    }
                }
            }
        }
        // return Err("路由未匹配");
        // let param: HashMap<String,String> = HashMap::new();
    }
    // 页面显示
    pub fn view(&self){
        println!("node_name:{:?},method:{:?},hooks_id:{:?},params:{:?},",self.path,self.method,self.hook_id,self.params);
        for node in &self.nodes {
            node.view();
        }
    }
}