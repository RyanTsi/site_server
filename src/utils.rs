use crate::Uuid;


/// 生成一个 uuid
pub fn get_uuid() -> Uuid {
    uuid::Uuid::new_v4().to_string()
}