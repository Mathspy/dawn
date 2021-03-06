use crate::id::{ChannelId, GuildId, MessageId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ReactionRemoveAll {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub guild_id: Option<GuildId>,
}
