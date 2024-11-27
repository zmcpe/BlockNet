#![allow(deprecated)]

pub mod actor_event;
pub mod actor_pick_request;
pub mod add_actor;
pub mod add_behaviour_tree;
pub mod add_entity;
pub mod add_item_actor;
pub mod add_painting;
pub mod add_player;
pub mod add_volume_entity;
pub mod adventure_settings;
pub mod agent_action;
pub mod agent_animation;
pub mod animate;
pub mod animate_entity;
pub mod anvil_damage;
pub mod automation_client_connect;
pub mod available_actor_identifiers;
pub mod available_commands;
pub mod biome_definition_list;
pub mod block_actor_data;
pub mod block_event;
pub mod block_pick_request;
pub mod book_edit;
pub mod boss_event;
pub mod camera;
pub mod camera_instruction;
pub mod camera_presets;
pub mod camera_shake;
pub mod change_dimension;
pub mod change_mob_property;
pub mod chunk_radius_updated;
pub mod client_bound_debug_renderer;
pub mod client_bound_map_item_data;
pub mod client_cache_blob_status;
pub mod client_cache_miss_response;
pub mod client_cache_status;
pub mod client_cheat_ability;
pub mod client_start_item_cooldown;
pub mod client_to_server_handshake;
pub mod code_builder;
pub mod code_builder_source;
pub mod command_block_update;
pub mod command_output;
pub mod command_request;
pub mod completed_using_item;
pub mod compressed_biome_definition_list;
pub mod container_close;
pub mod container_open;
pub mod container_set_data;
pub mod correct_player_move_prediction;
pub mod crafting_data;
pub mod create_photo;
pub mod creative_content;
pub mod death_info;
pub mod debug_info;
pub mod dimension_data;
pub mod disconnect;
pub mod editor_network;
pub mod education_resource_uri;
pub mod education_settings;
pub mod emote;
pub mod emote_list;
pub mod feature_registry;
pub mod filter_text;
pub mod game_rules_changed;
pub mod game_test_request;
pub mod game_test_results;
pub mod gui_data_pick_item;
pub mod hurt_armour;
pub mod interact;
pub mod inventory_content;
pub mod inventory_slot;
pub mod inventory_transaction;
pub mod item_component;
pub mod item_frame_drop_item;
pub mod item_stack_request;
pub mod item_stack_response;
pub mod lab_table;
pub mod lectern_update;
pub mod legacy_telemetry_event;
pub mod lesson_progress;
pub mod level_chunk;
pub mod level_event;
pub mod level_event_generic;
pub mod level_sound_event;
pub mod login;
pub mod map_create_locked_copy;
pub mod map_info_request;
pub mod mob_armour_equipment;
pub mod mob_effect;
pub mod mob_equipment;
pub mod modal_form_request;
pub mod modal_form_response;
pub mod motion_prediction_hints;
pub mod move_actor_absolute;
pub mod move_actor_delta;
pub mod move_player;
pub mod multi_player_settings;
pub mod network_chunk_publisher_update;
pub mod network_settings;
pub mod network_stack_latency;
pub mod npc_dialogue;
pub mod npc_request;
pub mod on_screen_texture_animation;
pub mod open_sign;
pub mod packet_violation_warning;
pub mod passenger_jump;
pub mod photo_info_request;
pub mod photo_transfer;
pub mod play_sound;
pub mod play_status;
pub mod player_action;
pub mod player_armour_damage;
pub mod player_auth_input;
pub mod player_enchant_options;
pub mod player_fog;
pub mod player_hot_bar;
pub mod player_input;
pub mod player_list;
pub mod player_skin;
pub mod player_toggle_crafter_slot_request;
pub mod position_tracking_db_client_request;
pub mod position_tracking_db_server_broadcast;
pub mod purchase_receipt;
pub mod refresh_entitlements;
pub mod remove_actor;
pub mod remove_entity;
pub mod remove_objective;
pub mod remove_volume_entity;
pub mod request_ability;
pub mod request_chunk_radius;
pub mod request_network_settings;
pub mod request_permissions;
pub mod resource_pack_chunk_data;
pub mod resource_pack_chunk_request;
pub mod resource_pack_client_response;
pub mod resource_pack_data_info;
pub mod resource_pack_stack;
pub mod resource_packs_info;
pub mod respawn;
pub mod script_custom_event;
pub mod script_message;
pub mod server_settings_request;
pub mod server_settings_response;
pub mod server_stats;
pub mod server_to_client_handshake;
pub mod set_actor_data;
pub mod set_actor_link;
pub mod set_actor_motion;
pub mod set_commands_enabled;
pub mod set_default_game_type;
pub mod set_difficulty;
pub mod set_display_objective;
pub mod set_health;
pub mod set_last_hurt_by;
pub mod set_local_player_as_initialised;
pub mod set_player_game_type;
pub mod set_player_inventory_options;
pub mod set_score;
pub mod set_scoreboard_identity;
pub mod set_spawn_position;
pub mod set_time;
pub mod set_title;
pub mod settings_command;
pub mod show_credits;
pub mod show_profile;
pub mod show_store_offer;
pub mod simple_event;
pub mod simulation_type;
pub mod spawn_experience_orb;
pub mod spawn_particle_effect;
pub mod start_game;
pub mod stop_sound;
pub mod structure_block_update;
pub mod structure_template_data_request;
pub mod structure_template_data_response;
pub mod sub_chunk;
pub mod sub_chunk_request;
pub mod sub_client_login;
pub mod sync_actor_property;
pub mod take_item_actor;
pub mod text;
pub mod tick_sync;
pub mod ticking_area_load_status;
pub mod toast_request;
pub mod transfer;
pub mod trim_data;
pub mod unlocked_recipes;
pub mod update_abilities;
pub mod update_adventure_settings;
pub mod update_attributes;
pub mod update_block;
pub mod update_block_synced;
pub mod update_client_input_locks;
pub mod update_equip;
pub mod update_player_game_type;
pub mod update_soft_enum;
pub mod update_sub_chunk_blocks;
pub mod update_trade;

pub use actor_event::*;
pub use actor_pick_request::*;
pub use add_actor::*;
pub use add_behaviour_tree::*;
pub use add_entity::*;
pub use add_item_actor::*;
pub use add_painting::*;
pub use add_player::*;
pub use add_volume_entity::*;
pub use adventure_settings::*;
pub use agent_action::*;
pub use agent_animation::*;
pub use animate::*;
pub use animate_entity::*;
pub use anvil_damage::*;
pub use automation_client_connect::*;
pub use available_actor_identifiers::*;
pub use available_commands::*;
pub use biome_definition_list::*;
pub use block_actor_data::*;
pub use block_event::*;
pub use block_pick_request::*;
pub use book_edit::*;
pub use boss_event::*;
pub use camera::*;
pub use camera_instruction::*;
pub use camera_presets::*;
pub use camera_shake::*;
pub use change_dimension::*;
pub use change_mob_property::*;
pub use chunk_radius_updated::*;
pub use client_bound_debug_renderer::*;
pub use client_bound_map_item_data::*;
pub use client_cache_blob_status::*;
pub use client_cache_miss_response::*;
pub use client_cache_status::*;
pub use client_cheat_ability::*;
pub use client_start_item_cooldown::*;
pub use client_to_server_handshake::*;
pub use code_builder::*;
pub use code_builder_source::*;
pub use command_block_update::*;
pub use command_output::*;
pub use command_request::*;
pub use completed_using_item::*;
pub use compressed_biome_definition_list::*;
pub use container_close::*;
pub use container_open::*;
pub use container_set_data::*;
pub use correct_player_move_prediction::*;
pub use crafting_data::*;
pub use create_photo::*;
pub use creative_content::*;
pub use death_info::*;
pub use debug_info::*;
pub use dimension_data::*;
pub use disconnect::*;
pub use editor_network::*;
pub use education_resource_uri::*;
pub use education_settings::*;
pub use emote::*;
pub use emote_list::*;
pub use feature_registry::*;
pub use filter_text::*;
pub use game_rules_changed::*;
pub use game_test_request::*;
pub use game_test_results::*;
pub use gui_data_pick_item::*;
pub use hurt_armour::*;
pub use interact::*;
pub use inventory_content::*;
pub use inventory_slot::*;
pub use inventory_transaction::*;
pub use item_component::*;
pub use item_frame_drop_item::*;
pub use item_stack_request::*;
pub use item_stack_response::*;
pub use lab_table::*;
pub use lectern_update::*;
pub use legacy_telemetry_event::*;
pub use lesson_progress::*;
pub use level_chunk::*;
pub use level_event::*;
pub use level_event_generic::*;
pub use level_sound_event::*;
pub use login::*;
pub use map_create_locked_copy::*;
pub use map_info_request::*;
pub use mob_armour_equipment::*;
pub use mob_effect::*;
pub use mob_equipment::*;
pub use modal_form_request::*;
pub use modal_form_response::*;
pub use motion_prediction_hints::*;
pub use move_actor_absolute::*;
pub use move_actor_delta::*;
pub use move_player::*;
pub use multi_player_settings::*;
pub use network_chunk_publisher_update::*;
pub use network_settings::*;
pub use network_stack_latency::*;
pub use npc_dialogue::*;
pub use npc_request::*;
pub use on_screen_texture_animation::*;
pub use open_sign::*;
pub use packet_violation_warning::*;
pub use passenger_jump::*;
pub use photo_info_request::*;
pub use photo_transfer::*;
pub use play_sound::*;
pub use play_status::*;
pub use player_action::*;
pub use player_armour_damage::*;
pub use player_auth_input::*;
pub use player_enchant_options::*;
pub use player_fog::*;
pub use player_hot_bar::*;
pub use player_input::*;
pub use player_list::*;
pub use player_skin::*;
pub use player_toggle_crafter_slot_request::*;
pub use position_tracking_db_client_request::*;
pub use position_tracking_db_server_broadcast::*;
pub use purchase_receipt::*;
pub use refresh_entitlements::*;
pub use remove_actor::*;
pub use remove_entity::*;
pub use remove_objective::*;
pub use remove_volume_entity::*;
pub use request_ability::*;
pub use request_chunk_radius::*;
pub use request_network_settings::*;
pub use request_permissions::*;
pub use resource_pack_chunk_data::*;
pub use resource_pack_chunk_request::*;
pub use resource_pack_client_response::*;
pub use resource_pack_data_info::*;
pub use resource_pack_stack::*;
pub use resource_packs_info::*;
pub use respawn::*;
pub use script_custom_event::*;
pub use script_message::*;
pub use server_settings_request::*;
pub use server_settings_response::*;
pub use server_stats::*;
pub use server_to_client_handshake::*;
pub use set_actor_data::*;
pub use set_actor_link::*;
pub use set_actor_motion::*;
pub use set_commands_enabled::*;
pub use set_default_game_type::*;
pub use set_difficulty::*;
pub use set_display_objective::*;
pub use set_health::*;
pub use set_last_hurt_by::*;
pub use set_local_player_as_initialised::*;
pub use set_player_game_type::*;
pub use set_player_inventory_options::*;
pub use set_score::*;
pub use set_scoreboard_identity::*;
pub use set_spawn_position::*;
pub use set_time::*;
pub use set_title::*;
pub use settings_command::*;
pub use show_credits::*;
pub use show_profile::*;
pub use show_store_offer::*;
pub use simple_event::*;
pub use simulation_type::*;
pub use spawn_experience_orb::*;
pub use spawn_particle_effect::*;
pub use start_game::*;
pub use stop_sound::*;
pub use structure_block_update::*;
pub use structure_template_data_request::*;
pub use structure_template_data_response::*;
pub use sub_chunk::*;
pub use sub_chunk_request::*;
pub use sub_client_login::*;
pub use sync_actor_property::*;
pub use take_item_actor::*;
pub use text::*;
pub use tick_sync::*;
pub use ticking_area_load_status::*;
pub use toast_request::*;
pub use transfer::*;
pub use trim_data::*;
pub use unlocked_recipes::*;
pub use update_abilities::*;
pub use update_adventure_settings::*;
pub use update_attributes::*;
pub use update_block::*;
pub use update_block_synced::*;
pub use update_client_input_locks::*;
pub use update_equip::*;
pub use update_player_game_type::*;
pub use update_soft_enum::*;
pub use update_sub_chunk_blocks::*;
pub use update_trade::*;

use derive::{Decode, Encode};
use binary::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode, PartialEq)]
#[encoding(type = VarU32)]
pub enum PacketId {
    Login = 1,
    PlayStatus = 2,
    ServerToClientHandshake = 3,
    ClientToServerHandshake = 4,
    Disconnect = 5,
    ResourcePacksInfo = 6,
    ResourcePackStack = 7,
    ResourcePackClientResponse = 8,
    Text = 9,
    SetTime = 10,
    StartGame = 11,
    AddPlayer = 12,
    AddActor = 13,
    RemoveActor = 14,
    AddItemActor = 15,
    TakeItemActor = 17,
    MoveActorAbsolute = 18,
    MovePlayer = 19,
    PassengerJump = 20,
    UpdateBlock = 21,
    AddPainting = 22,
    TickSync = 23,
    LevelEvent = 25,
    BlockEvent = 26,
    ActorEvent = 27,
    MobEffect = 28,
    UpdateAttributes = 29,
    InventoryTransaction = 30,
    MobEquipment = 31,
    MobArmourEquipment = 32,
    Interact = 33,
    BlockPickRequest = 34,
    ActorPickRequest = 35,
    PlayerAction = 36,
    HurtArmour = 38,
    SetActorData = 39,
    SetActorMotion = 40,
    SetActorLink = 41,
    SetHealth = 42,
    SetSpawnPosition = 43,
    Animate = 44,
    Respawn = 45,
    ContainerOpen = 46,
    ContainerClose = 47,
    PlayerHotBar = 48,
    InventoryContent = 49,
    InventorySlot = 50,
    ContainerSetData = 51,
    CraftingData = 52,
    GUIDataPickItem = 54,
    AdventureSettings = 55,
    BlockActorData = 56,
    PlayerInput = 57,
    LevelChunk = 58,
    SetCommandsEnabled = 59,
    SetDifficulty = 60,
    ChangeDimension = 61,
    SetPlayerGameType = 62,
    PlayerList = 63,
    SimpleEvent = 64,
    LegacyTelemetryEvent = 65,
    SpawnExperienceOrb = 66,
    ClientBoundMapItemData = 67,
    MapInfoRequest = 68,
    RequestChunkRadius = 69,
    ChunkRadiusUpdated = 70,
    ItemFrameDropItem = 71,
    GameRulesChanged = 72,
    Camera = 73,
    BossEvent = 74,
    ShowCredits = 75,
    AvailableCommands = 76,
    CommandRequest = 77,
    CommandBlockUpdate = 78,
    CommandOutput = 79,
    UpdateTrade = 80,
    UpdateEquip = 81,
    ResourcePackDataInfo = 82,
    ResourcePackChunkData = 83,
    ResourcePackChunkRequest = 84,
    Transfer = 85,
    PlaySound = 86,
    StopSound = 87,
    SetTitle = 88,
    AddBehaviourTree = 89,
    StructureBlockUpdate = 90,
    ShowStoreOffer = 91,
    PurchaseReceipt = 92,
    PlayerSkin = 93,
    SubClientLogin = 94,
    AutomationClientConnect = 95,
    SetLastHurtBy = 96,
    BookEdit = 97,
    NPCRequest = 98,
    PhotoTransfer = 99,
    ModalFormRequest = 100,
    ModalFormResponse = 101,
    ServerSettingsRequest = 102,
    ServerSettingsResponse = 103,
    ShowProfile = 104,
    SetDefaultGameType = 105,
    RemoveObjective = 106,
    SetDisplayObjective = 107,
    SetScore = 108,
    LabTable = 109,
    UpdateBlockSynced = 110,
    MoveActorDelta = 111,
    SetScoreboardIdentity = 112,
    SetLocalPlayerAsInitialised = 113,
    UpdateSoftEnum = 114,
    NetworkStackLatency = 115,
    ScriptCustomEvent = 117,
    SpawnParticleEffect = 118,
    AvailableActorIdentifiers = 119,
    NetworkChunkPublisherUpdate = 121,
    BiomeDefinitionList = 122,
    LevelSoundEvent = 123,
    LevelEventGeneric = 124,
    LecternUpdate = 125,
    AddEntity = 127,
    RemoveEntity = 128,
    ClientCacheStatus = 129,
    MapCreateLockedCopy = 130,
    OnScreenTextureAnimation = 131,
    StructureTemplateDataRequest = 132,
    StructureTemplateDataResponse = 133,
    ClientCacheBlobStatus = 135,
    ClientCacheMissResponse = 136,
    EducationSettings = 137,
    Emote = 138,
    MultiPlayerSettings = 139,
    SettingsCommand = 140,
    AnvilDamage = 141,
    CompletedUsingItem = 142,
    NetworkSettings = 143,
    PlayerAuthInput = 144,
    CreativeContent = 145,
    PlayerEnchantOptions = 146,
    ItemStackRequest = 147,
    ItemStackResponse = 148,
    PlayerArmourDamage = 149,
    CodeBuilder = 150,
    UpdatePlayerGameType = 151,
    EmoteList = 152,
    PositionTrackingDBServerBroadcast = 153,
    PositionTrackingDBClientRequest = 154,
    DebugInfo = 155,
    PacketViolationWarning = 156,
    MotionPredictionHints = 157,
    AnimateEntity = 158,
    CameraShake = 159,
    PlayerFog = 160,
    CorrectPlayerMovePrediction = 161,
    ItemComponent = 162,
    FilterText = 163,
    ClientBoundDebugRenderer = 164,
    SyncActorProperty = 165,
    AddVolumeEntity = 166,
    RemoveVolumeEntity = 167,
    SimulationType = 168,
    NPCDialogue = 169,
    EducationResourceURI = 170,
    CreatePhoto = 171,
    UpdateSubChunkBlocks = 172,
    PhotoInfoRequest = 173,
    SubChunk = 174,
    SubChunkRequest = 175,
    ClientStartItemCooldown = 176,
    ScriptMessage = 177,
    CodeBuilderSource = 178,
    TickingAreasLoadStatus = 179,
    DimensionData = 180,
    AgentAction = 181,
    ChangeMobProperty = 182,
    LessonProgress = 183,
    RequestAbility = 184,
    RequestPermissions = 185,
    ToastRequest = 186,
    UpdateAbilities = 187,
    UpdateAdventureSettings = 188,
    DeathInfo = 189,
    EditorNetwork = 190,
    FeatureRegistry = 191,
    ServerStats = 192,
    RequestNetworkSettings = 193,
    GameTestRequest = 194,
    GameTestResults = 195,
    UpdateClientInputLocks = 196,
    ClientCheatAbility = 197,
    CameraPresets = 198,
    UnlockedRecipes = 199,
    CameraInstruction = 300,
    CompressedBiomeDefinitionList = 301,
    TrimData = 302,
    OpenSign = 303,
    AgentAnimation = 304,
    RefreshEntitlements = 305,
    PlayerToggleCrafterSlotRequest = 306,
    SetPlayerInventoryOptions = 307,
}

pub trait Packet<'a> : Encode + Decode<'a> {
    fn id(&self) -> PacketId;
}