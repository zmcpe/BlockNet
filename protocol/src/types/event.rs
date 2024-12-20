use binary::{v32, v64};
use derive::{Decode, Encode};

#[repr(i32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum EventType {
    AchievementAwarded(AchievementAwarded),
    EntityInteract(EntityInteract),
    PortalBuilt(PortalBuilt),
    PortalUsed(PortalUsed),
    MobKilled(MobKilled),
    CauldronUsed(CauldronUsed),
    PlayerDied(PlayerDied),
    BossKilled(BossKilled),
    AgentCommand(AgentCommand),
    AgentCreated(AgentCreated),
    PatternRemoved(PatternRemoved),
    SlashCommandExecuted(SlashCommandExecuted),
    FishBucketed(FishBucketed),
    MobBorn(MobBorn),
    PetDied(PetDied),
    CauldronInteract(CauldronInteract),
    ComposterInteract(ComposterInteract),
    BellUsed(BellUsed),
    EntityDefinitionTrigger(EntityDefinitionTrigger),
    RaidUpdate(RaidUpdate),
    MovementAnomaly(MovementAnomaly),
    MovementCorrected(MovementCorrected),
    ExtractHoney(ExtractHoney),
    TargetBlockHit(TargetBlockHit),
    PiglinBarter(PiglinBarter),
    PlayerWaxedOrUnwaxedCopper(PlayerWaxedOrUnwaxedCopper),
    CodeBuilderRuntimeAction(CodeBuilderRuntimeAction),
    CodeBuilderScoreboard(CodeBuilderScoreboard),
    StriderRiddenInLavaInOverworld(StriderRiddenInLavaInOverworld),
    SneakCloseToSculkSensor(SneakCloseToSculkSensor),
    CarefulRestoration(CarefulRestoration),
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct StriderRiddenInLavaInOverworld {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CodeBuilderScoreboard {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CodeBuilderRuntimeAction {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PiglinBarter {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TargetBlockHit {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

/// The purpose of this event is unknown.
#[derive(Debug, Clone, Encode, Decode)]
pub struct ExtractHoney {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

/// Used to update a raids progress client side.
#[derive(Debug, Clone, Encode, Decode)]
pub struct RaidUpdate {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub current_raid_wave: v32,
    pub total_raid_waves: v32,
    pub won_raid: bool,
}

/// The purpose of this event is unknown.
#[derive(Debug, Clone, Encode, Decode)]
pub struct EntityDefinitionTrigger {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub event_name: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct EntityInteract {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub interaction_type: v32,
    pub interaction_entity_type: v32,
    pub entity_variant: v32,
    pub entity_colour: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CauldronInteract {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub block_interaction_type: v32,
    pub item_id: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CauldronUsed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub potion_id: v32,
    pub colour: v32,
    pub fill_level: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ComposterInteract {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub block_interaction_type: v32,
    pub item_id: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BossKilled {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub boss_entity_unique_id: v64,
    pub player_party_size: v32,
    pub interaction_entity_type: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AchievementAwarded {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub achievement_id: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AgentCommand {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub agent_result: v32,
    pub data_value: v32,
    pub command: String,
    pub data_key: String,
    pub output: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AgentCreated {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct SlashCommandExecuted {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub success_count: v32,
    pub message_count: v32,
    pub command_name: String,
    pub output_messages: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MobKilled {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub killer_entity_unique_id: v64,
    pub victim_entity_unique_id: v64,
    pub killer_entity_type: v32,
    pub entity_damage_cause: v32,
    pub villager_trade_tier: v32,
    pub villager_display_name: String,
}

/// Informs the receiver on movement data.
#[derive(Debug, Clone, Encode, Decode)]
pub struct MovementAnomaly {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub event_type: u8,
    pub cheating_score: f32,
    pub average_position_delta: f32,
    pub total_position_delta: f32,
    pub min_position_delta: f32,
    pub max_position_delta: f32,
}

/// Sent by the server to correct client-side movement.
#[derive(Debug, Clone, Encode, Decode)]
pub struct MovementCorrected {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub position_delta: f32,
    pub cheating_score: f32,
    pub score_threshold: f32,
    pub distance_threshold: f32,
    pub duration_threshold: v32,
}

/// This event is self-explanatory.
#[derive(Debug, Clone, Encode, Decode)]
pub struct BellUsed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub item_id: v32,
}

/// Sent when a fish is bucketed.
#[derive(Debug, Clone, Encode, Decode)]
pub struct FishBucketed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub pattern: v32,
    pub preset: v32,
    pub bucketed_entity_type: v32,
    pub release: bool,
}

/// Sent when a mob is born.
#[derive(Debug, Clone, Encode, Decode)]
pub struct MobBorn {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub entity_type: v32,
    pub variant: v32,
    pub colour: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PlayerWaxedOrUnwaxedCopper {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

/// Sent when a pet dies. This event is deprecated.
#[derive(Debug, Clone, Encode, Decode)]
pub struct PetDied {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub killed_by_owner: bool,
    pub killer_entity_unique_id: v64,
    pub pet_entity_unique_id: v64,
    pub entity_damage_cause: v32,
    pub pet_entity_type: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PlayerDied {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub attacker_entity_id: v32,
    pub attacker_variant: v32,
    pub entity_damage_cause: v32,
    pub in_raid: bool,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PortalBuilt {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub dimension_id: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PortalUsed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub from_dimension_id: v32,
    pub to_dimension_id: v32,
}

/// This event is self-explanatory.
#[derive(Debug, Clone, Encode, Decode)]
pub struct SneakCloseToSculkSensor {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CarefulRestoration {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PatternRemoved {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub item_id: v32,
    pub aux_value: v32,
    pub patterns_size: v32,
    pub pattern_index: v32,
    pub pattern_colour: v32,
}
