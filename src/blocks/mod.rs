use serde::{Deserialize, Serialize};
#[cfg(feature = "tabled")]
use tabled::Tabled;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct SoundType {
    pub name: String,
    pub volume: f64,
    pub pitch: f64,
    #[serde(rename = "breakSound")]
    pub break_sound: String,
    #[serde(rename = "stepSound")]
    pub step_sound: String,
    #[serde(rename = "placeSound")]
    pub place_sound: String,
    #[serde(rename = "hitSound")]
    pub hit_sound: String,
    #[serde(rename = "fallSound")]
    pub fall_sound: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct Material {
    pub name: String,
    pub color: i64,
    #[serde(rename = "isLiquid")]
    pub is_liquid: bool,
    #[serde(rename = "pushReaction")]
    pub push_reaction: String,
    #[serde(rename = "blockMotion")]
    pub block_motion: bool,
    pub flammable: bool,
    pub liquid: bool,
    #[serde(rename = "solidBlocking")]
    pub solid_blocking: bool,
    pub replaceable: bool,
    pub solid: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct BlockProperties {
    pub material: String,
    #[serde(rename = "hasCollision")]
    pub has_collision: bool,
    #[serde(rename = "soundType")]
    pub sound_type: String,
    #[serde(rename = "defaultLightEmission")]
    pub default_light_emission: i64,
    #[serde(rename = "explosionResistance")]
    pub explosion_resistance: f64,
    #[serde(rename = "destroyTime")]
    pub destroy_time: f64,
    #[serde(rename = "requiresCorrectToolForDrops")]
    pub requires_correct_tool_for_drops: bool,
    #[serde(rename = "isRandomlyTicking")]
    pub is_randomly_ticking: bool,
    pub friction: f64,
    #[serde(rename = "speedFactor")]
    pub speed_factor: f64,
    #[serde(rename = "jumpFactor")]
    pub jump_factor: f64,
    #[serde(rename = "canOcclude")]
    pub can_occlude: bool,
    #[serde(rename = "isAir")]
    pub is_air: bool,
    pub drops: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct Block {
    pub id: usize,
    pub name: String,
    #[cfg_attr(feature = "tabled",tabled(inline))]
    pub properties: BlockProperties,
}
