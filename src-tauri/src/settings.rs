use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum IntervalMode {
    Random = 0,
    Fixed = 1,
}

impl Default for IntervalMode {
    fn default() -> Self {
        IntervalMode::Fixed
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FixedIntervalOptions {
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub milliseconds: u16,
}

impl Default for FixedIntervalOptions {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 1,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RandomIntervalOptions {
    pub min: u16,
    pub max: u16,
}

impl Default for RandomIntervalOptions {
    fn default() -> Self {
        Self { min: 0, max: 1 }
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum RepeatMode {
    Forever = 0,
    Count = 1,
}

impl Default for RepeatMode {
    fn default() -> Self {
        RepeatMode::Forever
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ClickButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}

impl Default for ClickButton {
    fn default() -> Self {
        ClickButton::Left
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ClickQuantity {
    Single = 0,
    Double = 1,
}

impl Default for ClickQuantity {
    fn default() -> Self {
        ClickQuantity::Single
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct MousePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppSettingsData {
    pub interval_mode: IntervalMode,
    pub interval_fixed_options: FixedIntervalOptions,
    pub interval_rand_options: RandomIntervalOptions,

    pub repeat_mode: RepeatMode,
    pub repeat_count: u32,

    pub click_button: ClickButton,
    pub click_quantity: ClickQuantity,

    pub set_mouse_position: bool,
    pub mouse_position: MousePos
}

impl Default for AppSettingsData {
    fn default() -> Self {
        Self {
            interval_mode: Default::default(),
            interval_fixed_options: Default::default(),
            interval_rand_options: Default::default(),
            repeat_mode: Default::default(),
            repeat_count: Default::default(),
            click_button: Default::default(),
            click_quantity: Default::default(),
            set_mouse_position: Default::default(),
            mouse_position: Default::default()
        }
    }
}
