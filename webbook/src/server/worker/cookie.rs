use chrono::{Utc, DateTime};
pub struct Cookie{
    name: String,
    value: String,
    expires: Option<DateTime<Utc>>,
    max_age: Option<u32>,
    domain: String,
    path: String,
    secure: bool,
    http_only: bool,
}

impl Cookie {
    pub fn new(name: String, value: String) -> Cookie{  
        Cookie{
            name: name,
            value: value,
            expires: None,
            max_age: None,
            domain: String::new(),
            path: String::new(),
            secure: false,
            http_only: false,
        }
    }
    pub fn get_name(&self) -> String{return self.name.clone();}
    pub fn get_value(&self) -> String{return self.value.clone();}
    pub fn get_expires(&self) -> Option<DateTime<Utc>>{return self.expires.clone();}
    pub fn get_max_age(&self) -> Option<u32>{return self.max_age;}
    pub fn get_domain(&self) -> String{return self.domain.clone();}
    pub fn get_path(&self) -> String{return self.path.clone();}
    pub fn get_secure(&self) -> bool{return self.secure;}
    pub fn get_http_only(&self) ->bool{return self.http_only;}
    pub fn set_name(&mut self, name:String){self.name = name;}
    pub fn set_value(&mut self, value:String){self.value = value;}
    pub fn set_expires(&mut self, expires:Option<DateTime<Utc>>){self.expires = expires;}
    pub fn set_max_age(&mut self, max_age:Option<u32>){self.max_age = max_age;}
    pub fn set_domain(&mut self, domain:String){self.domain = domain;}
    pub fn set_path(&mut self, path:String){self.path = path;}
    pub fn set_secure(&mut self, secure:bool){self.secure = secure;}
    pub fn set_http_only(&mut self, http_only:bool){self.http_only = http_only;}

}