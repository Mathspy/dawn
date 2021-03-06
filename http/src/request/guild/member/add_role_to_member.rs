use crate::request::prelude::*;
use dawn_model::id::{GuildId, RoleId, UserId};

pub struct AddRoleToMember<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    user_id: UserId,
    reason: Option<String>,
}

impl<'a> AddRoleToMember<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
        role_id: impl Into<RoleId>,
    ) -> Self {
        Self {
            fut: None,
            guild_id: guild_id.into(),
            http,
            role_id: role_id.into(),
            user_id: user_id.into(),
            reason: None,
        }
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                headers,
                Route::AddMemberRole {
                    guild_id: self.guild_id.0,
                    role_id: self.role_id.0,
                    user_id: self.user_id.0,
                },
            ))
        } else {
            Request::from(Route::AddMemberRole {
                guild_id: self.guild_id.0,
                role_id: self.role_id.0,
                user_id: self.user_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(AddRoleToMember<'_>, ());
