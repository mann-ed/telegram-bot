use crate::types::*;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatMemberUpdated {
    //Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    //Date the change was done in Unix time
    pub date: Option<Integer>,
    //Previous information about the chat member
    pub old_chat_member: ChatMember,
    //New information about the chat member
    pub new_chat_member: ChatMember,
    //Optional. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    pub invite_link: Option<String>,
}
