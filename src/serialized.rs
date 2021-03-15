use serde::{Deserialize, Serialize};

use crate::character::npc::NPCType;
use crate::map::chunk::world_chunk_map::WorldChunkMap;
use crate::map::set::manager::WorldMapSetManager;

#[derive(Deserialize, Serialize)]
pub struct SerializedWorld {

    pub chunks: WorldChunkMap,
    pub map_sets: WorldMapSetManager,

    pub npc_types: Vec<SerializedNPCType>,
    pub palettes: Vec<Palette>,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedNPCType {

    pub identifier: String,
    pub data: NPCType,
    pub sprite: Vec<u8>,
    pub battle_sprite: Option<Vec<u8>>,

}

#[derive(Deserialize, Serialize)]
pub struct Palette {

    pub id: u8,
    pub bottom: Vec<u8>,

}