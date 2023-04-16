use std::collections::{HashSet, HashMap};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::message::{ChatMessageType, WebSocketMessageType, RoomMessage};

/// チャットの内容を記憶する構造体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageDb {
    romms: HashMap<Uuid, MessageRoom>
}

impl MessageDb {
    pub fn new() -> Self {
        Default::default()
    }

    /// チャット内容を追加する
    pub fn add_message(&mut self, msg: ChatMessageType) {
        let msg = msg.into_room_message().unwrap();
        let room = self.romms.entry(msg.room_id).or_insert_with(MessageRoom::new);
        room.add_comment(msg.comment);
    }

    /// 過去のチャット内容を送信する
    pub fn get_message_history(&self, id: Uuid) -> ChatMessageType {
        let room = self.romms.get(&id).unwrap();
        ChatMessageType::Init(room.comments.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageRoom {
    pub name: Option<String>,
    pub members: HashSet<Uuid>,
    pub comments: Vec<Comment>,
}

impl MessageRoom {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    from: Uuid,
    message: String,
}

/// チャットアプリケーションに登録されているユーザーの情報を保持する構造体
pub struct Members( HashMap<Uuid, String> );

impl Members {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// ユーザーを登録する
    /// 登録したユーザーのidと名前を返す
    pub fn add_member(&mut self, name: &str) -> (Uuid, String) {
        let id = Uuid::new_v4();
        self.0.insert(id, name.to_string());
        (id, name.to_string())
    }

    pub fn get_name(&self, id: Uuid) -> Option<&String> {
        self.0.get(&id)
    }

    pub fn to_vec(&self) -> Vec<(Uuid, String)> {
        self.0.clone().into_iter().collect()
    }
}
