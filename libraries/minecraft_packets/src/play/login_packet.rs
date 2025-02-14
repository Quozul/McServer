use minecraft_protocol::prelude::*;

/// This packet is called "Join Game" prior to 1.19 on wiki.vg
#[derive(PacketOut)]
#[packet_id("play/clientbound/minecraft:login")]
pub struct LoginPacket {
    /// The player's Entity ID (EID).
    pub entity_id: i32,
    pub is_hardcore: bool,
    #[pvn(..764)]
    pub game_mode: u8,
    #[pvn(..764)]
    pub previous_game_mode: i8,
    /// Size of the following array.
    /// Identifiers for all dimensions on the server.
    pub dimension_names: LengthPaddedVec<Identifier>,
    /// Registry codec is called dimension codec prior to 1.19
    #[pvn(..764)]
    pub registry_codec: Nbt,
    #[pvn(759..764)]
    pub v1_19_dimension_type: Identifier,
    #[pvn(..759)]
    pub dimension: Nbt,
    /// Name of the dimension being spawned into.
    #[pvn(..764)]
    pub dimension_name: Identifier,
    /// First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
    #[pvn(..764)]
    pub hashed_seed: i64,
    /// Was once used by the client to draw the player list, but now is ignored.
    pub max_players: VarInt,
    /// Render distance (2-32).
    pub view_distance: VarInt,
    /// The distance that the client will process specific things, such as entities.
    pub simulation_distance: VarInt,
    /// If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
    pub reduced_debug_info: bool,
    /// Set to false when the doImmediateRespawn gamerule is true.
    pub enable_respawn_screen: bool,
    /// Whether players can only craft recipes they have already unlocked. Currently unused by the client.
    #[pvn(764..)]
    pub v_1_20_2_do_limited_crafting: bool,
    /// The ID of the type of dimension in the minecraft:dimension_type registry, defined by the Registry Data packet.
    /// 0: overworld, 1: overworld_caves, 2: the_end, 3: the_nether
    #[pvn(766..)]
    pub v_1_20_5_dimension_type: VarInt,
    #[pvn(764..766)]
    pub v_1_20_2_dimension_type: Identifier,
    /// Name of the dimension being spawned into.
    #[pvn(764..)]
    pub v_1_20_2_dimension_name: Identifier,
    /// First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
    #[pvn(764..)]
    pub v_1_20_2_hashed_seed: i64,
    /// 0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
    #[pvn(764..)]
    pub v_1_20_2_game_mode: u8,
    /// -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed)
    #[pvn(764..)]
    pub v_1_20_2_previous_game_mode: i8,
    /// True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks.
    pub is_debug: bool,
    /// True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63.
    pub is_flat: bool,
    /// If true, then the next two fields are present.
    #[pvn(759..)]
    pub has_death_location: bool,
    /// Name of the dimension the player died in.
    #[pvn(759..)]
    pub death_dimension_name: Option<Identifier>,
    /// The location that the player died at.
    #[pvn(759..)]
    pub death_location: Option<Position>,
    /// The number of ticks until the player can use the portal again.
    #[pvn(763..)]
    pub portal_cooldown: VarInt,
    #[pvn(768..)]
    pub sea_level: VarInt,
    #[pvn(766..)]
    pub enforces_secure_chat: bool,
}

impl Default for LoginPacket {
    fn default() -> Self {
        let overworld = Identifier::minecraft("overworld");
        Self {
            entity_id: 0,
            is_hardcore: false,
            game_mode: 3,
            previous_game_mode: -1,
            dimension_names: Vec::new().into(),
            registry_codec: Nbt::End, // This must be defined by the caller
            max_players: VarInt::new(1),
            view_distance: VarInt::new(10),
            simulation_distance: VarInt::new(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            v_1_20_2_do_limited_crafting: false,
            v_1_20_5_dimension_type: VarInt::new(0),
            v1_19_dimension_type: overworld.clone(),
            dimension: Nbt::End, // This must be defined by the caller
            dimension_name: overworld.clone(),
            hashed_seed: 0,
            v_1_20_2_game_mode: 3,
            v_1_20_2_previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::default(),
            sea_level: VarInt::default(),
            enforces_secure_chat: true,
            v_1_20_2_dimension_name: overworld.clone(),
            v_1_20_2_dimension_type: overworld,
            v_1_20_2_hashed_seed: 0,
        }
    }
}
