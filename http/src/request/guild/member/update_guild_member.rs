use crate::request::prelude::*;
use dawn_model::{
    guild::Member,
    id::{ChannelId, GuildId, RoleId, UserId},
};

#[derive(Default, Serialize)]
struct UpdateGuildMemberFields {
    channel_id: Option<ChannelId>,
    deaf: Option<bool>,
    mute: Option<bool>,
    nick: Option<String>,
    roles: Option<Vec<RoleId>>,
}

pub struct UpdateGuildMember<'a> {
    fields: UpdateGuildMemberFields,
    fut: Option<Pending<'a, Member>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
    reason: Option<String>,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: UpdateGuildMemberFields::default(),
            fut: None,
            guild_id,
            http,
            user_id,
            reason: None,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

        self
    }

    pub fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf.replace(deaf);

        self
    }

    pub fn mute(mut self, mute: bool) -> Self {
        self.fields.mute.replace(mute);

        self
    }

    pub fn nick(mut self, nick: impl Into<String>) -> Self {
        self.fields.nick.replace(nick.into());

        self
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                serde_json::to_vec(&self.fields)?,
                headers,
                Route::UpdateMember {
                    guild_id: self.guild_id.0,
                    user_id: self.user_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::UpdateMember {
                    guild_id: self.guild_id.0,
                    user_id: self.user_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateGuildMember<'_>, Member);
