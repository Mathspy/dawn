use crate::id::{ChannelId, GenericId, MessageId, UserId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AuditLogOptionalEntryInfo {
    pub channel_id: Option<ChannelId>,
    pub member_id: Option<UserId>,
    pub message_id: Option<MessageId>,
    pub count: Option<String>,
    pub delete_member_days: Option<String>,
    pub id: Option<GenericId>,
    pub members_removed: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: Option<String>,
    pub role_name: Option<String>,
}
