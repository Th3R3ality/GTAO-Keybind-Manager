pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("MULTIPLAYER_CHAT", "Multiplayer Chat", "Multiplayer Chat");
pub const MP_TEXT_CHAT_ALL: &'static(&'static str, &'static str, &'static str) = &("INPUT_MP_TEXT_CHAT_ALL", "Mp Text Chat All", "Unknown");
pub const MP_TEXT_CHAT_TEAM: &'static(&'static str, &'static str, &'static str) = &("INPUT_MP_TEXT_CHAT_TEAM", "Mp Text Chat Team", "Unknown");
pub const PUSH_TO_TALK: &'static(&'static str, &'static str, &'static str) = &("INPUT_PUSH_TO_TALK", "Push To Talk", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // MULTIPLAYER_CHAT
	MP_TEXT_CHAT_ALL,// INPUT_MP_TEXT_CHAT_ALL
	MP_TEXT_CHAT_TEAM,// INPUT_MP_TEXT_CHAT_TEAM
	PUSH_TO_TALK,// INPUT_PUSH_TO_TALK
];
