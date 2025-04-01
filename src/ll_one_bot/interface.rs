use actix_web::cookie::time::ext;
use serde::{Serialize, Deserialize};
use crate::llm_api::interface::Response;


#[derive(Serialize,Deserialize, Debug)]
pub struct SenderInfo{
  user_id: u64,
  nickname: String,
  card: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct QQMessage{
  pub r#type : String,
  pub data: MessageData
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(untagged)] // 使其反序列化时匹配内部类型而非枚举类型Text/Face
pub enum MessageData{
  Text{text: String},
  Face{id: String},
}
impl MessageData{
  pub fn get_text(&self) -> String{
    match self{
      MessageData::Text{text} => text.clone(),
      MessageData::Face{id} => id.clone(),
    }
  }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct LLOneBotPrivate{
  pub self_id: u64,
  pub user_id: u64,
  pub time: u64,
  message_id: u64,
  message_seq: u64,
  message_type: String, // private
  sender: SenderInfo,
  pub raw_message: String,
  font: u8,
  sub_type: String, //friend、group、group_self、other
  message: Vec<QQMessage>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LLOneBotGroup{
  pub self_id: u64,
  pub user_id: u64,
  pub group_id: u64,
  pub time: u64, 
  message_id: u64,
  message_type: String, // group
  sender: SenderInfo,
  pub raw_message: String,
  font: u8,
  sub_type: String, //friend、group、group_self、other
  message: Vec<QQMessage>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LLOneBot{
  Group(LLOneBotGroup),
  Private(LLOneBotPrivate),
  
}

impl LLOneBot{
  pub fn get_self_id(&self) -> u64{
    match self{
      LLOneBot::Private(message) => message.self_id,
      LLOneBot::Group(message) => message.self_id,
    }
  }
  pub fn get_time(&self) -> u64{
    match self{
      LLOneBot::Private(message) => message.time,
      LLOneBot::Group(message) => message.time,
    }
  }
  pub fn get_raw_message(&self) -> String{
    match self{
      LLOneBot::Private(message) => message.raw_message.clone(),
      LLOneBot::Group(message) => message.raw_message.clone(),
    }
  }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackPrivate{
  pub user_id: u64,
  pub message: Vec<QQMessage>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackGroup{
  pub group_id: u64,
  pub user_id: u64,
  pub message: Vec<QQMessage>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct SendBackIntermediate{ // 用于中间转换
  message: Vec<QQMessage>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SendBack{
  Private(SendBackPrivate),
  Group(SendBackGroup),
}

impl SendBack{
  pub fn get_content(&self) -> String{
    match self{
      SendBack::Private(sendback) => {
        let mut content = String::new();
        for message in &sendback.message {
          match &message.data {
            MessageData::Text{text} => {content.push_str(text);},
            MessageData::Face{id} => {content.push_str(&format!("[CQ:face,id={}]",id));},
          }
        }
        return content;
      },
      SendBack::Group(sendback) => {
        let mut content = String::new();
        //println!("{}", sendback.user_id);
        content.push_str(&format!("@{} ", sendback.user_id).as_str());
        //println!("第一次拼接后: {:?}", content); 
        for message in &sendback.message {
          match &message.data {
            MessageData::Text{text} => {content.push_str(text);},
            MessageData::Face{id} => {content.push_str(&format!("[CQ:face,id={}]",id));},
          }
        }
        //println!("第二次拼接后: {:?}", content); 
        return content;
      },
    }
  }
}

impl From<&Response> for SendBackIntermediate{
  fn from(response: &Response) -> Self{
    // 这里需要加入表情支持
    let raw_message = response.get_content();
    let message = extract_face(raw_message);
    Self{
      message,
    }
  }
}

impl SendBackIntermediate{ // 中间件，用完即消失
  pub fn set_user_id(self, user_id: u64) -> SendBack {
    SendBack::Private(SendBackPrivate {
      user_id,
      message: self.message,
    })
  }
  pub fn set_group_id(self, group_id: u64, user_id: u64)-> SendBack{
    SendBack::Group(SendBackGroup{
      group_id,
      user_id,
      message: self.message
    })
  }
} 

pub fn extract_face(raw: String)->Vec<QQMessage>{
  use regex::Regex;
  let re = Regex::new(r"\[CQ:face,id=(\d+)\]").unwrap();
  let mut parts = re.split(&raw)
    .map(|s| QQMessage{
      r#type: "text".to_string(),
      data: MessageData::Text{
        text: s.to_string()
      }
    })
    .collect::<Vec<QQMessage>>();  

  let mut index = 1;
  // 遍历所有匹配项
  for caps in re.captures_iter(&raw) {
      if let Some(id_match) = caps.get(1) {
          // 将捕获的 id 转换为 u64
          parts.insert(index,QQMessage { 
            r#type: "face".to_string(),
            data: MessageData::Face{
              id: id_match.as_str().to_string()
            } 
          });
          index += 2;
      }
  };
  parts
}