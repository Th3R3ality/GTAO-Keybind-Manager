use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct KeybindThing<IndexType> {
    pub index: IndexType,
    pub name: &'static str,
    pub pretty_name: &'static str,
    pub desc: &'static str,
}


pub type KeybindInputCategory = KeybindThing<usize>;
pub type KeybindInputCode = KeybindThing<usize>;
#[derive(Debug, Clone, PartialEq)]
pub struct KeybindInput {
    pub category: KeybindInputCategory,
    pub code: KeybindInputCode,
}

pub type KeybindSource = KeybindThing<usize>;
pub type KeybindKeycode = KeybindThing<usize>;
#[derive(Debug, Clone, PartialEq)]
pub struct KeybindKey{
    pub source: KeybindSource,
    pub keycode: KeybindKeycode,
}

pub type KeybindId = usize;
#[derive(Debug, Clone, PartialEq)]
pub struct Keybind {
    pub input: KeybindInput,
    pub key: KeybindKey,
    pub id: KeybindId,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeybindBuilder {
    pub input_category: Option<KeybindInputCategory>,
    pub input_code: Option<KeybindInputCode>,
    pub source: Option<KeybindSource>,
    pub keycode: Option<KeybindKeycode>,
}





impl KeybindInput {
    pub fn new(category: KeybindInputCategory, code: KeybindInputCode) -> Self {
        Self {
            category,
            code,
        }
    }
}
impl KeybindKey {
    pub fn new(source: KeybindSource, keycode: KeybindKeycode) -> Self {
        Self {
            source,
            keycode
        }
    }
}
impl Keybind {
    pub fn new(input: KeybindInput, key: KeybindKey, id: KeybindId) -> Self {
        Self {
            input,
            key,
            id,
        }
    }
    pub fn new_from_primitives(input_category: KeybindInputCategory, input_code: KeybindInputCode, source: KeybindSource, keycode: KeybindKeycode, id: KeybindId) -> Self {
        Self {
            input: KeybindInput::new(input_category, input_code),
            key: KeybindKey::new(source, keycode),
            id,
        }
    }
}

impl KeybindBuilder {
    pub fn new_empty() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn empty(&mut self) {
        self.input_category = None;
        self.input_code = None;
        self.source = None;
        self.keycode = None;
    }
}

impl From<KeybindInput> for (KeybindInputCategory, KeybindInputCode) {
    fn from(value: KeybindInput) -> Self {
        (value.category, value.code)
    }
}
impl From<KeybindKey> for (KeybindSource, KeybindKeycode) {
    fn from(value: KeybindKey) -> Self {
        (value.source, value.keycode)
    }
}

impl From<KeybindBuilder>
    for (
        Option<KeybindInputCategory>,
        Option<KeybindInputCode>,
        Option<KeybindSource>,
        Option<KeybindKeycode>,
    )
{
    fn from(builder: KeybindBuilder) -> Self {
        (
            builder.input_category,
            builder.input_code,
            builder.source,
            builder.keycode,
        )
    }
}

impl fmt::Display for KeybindThing<usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_name)
    }
}