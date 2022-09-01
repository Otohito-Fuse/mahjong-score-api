use super::model;
use mahjong_score::mahjong::{Fuuro, FuuroType, HandContext, Tile, TileWithDora, YakuFlags};
use std::collections::HashMap;

pub fn fuuro_tf(fuuro: &model::FuuroRepresentaion) -> Fuuro {
    let v: Vec<TileWithDora> = fuuro.tiles.iter().map(|t| tile_with_dora_tf(&t)).collect();
    match &*fuuro.r#type {
        "Pon" => Fuuro(FuuroType::Pon, v),
        "Chi" => Fuuro(FuuroType::Chi, v),
        "Ankan" => Fuuro(FuuroType::Ankan, v),
        "Minkan" => Fuuro(FuuroType::Minkan, v),
        _ => Fuuro(FuuroType::Pon, Vec::new()),
    }
}

fn tile_with_dora_tf(tile: &model::TileRepresentation) -> TileWithDora {
    TileWithDora(Tile(tile.r#type, tile.num), tile.dora.unwrap_or(0))
}

fn tile_tf(tile: &model::TileRepresentation) -> Tile {
    Tile(tile.r#type, tile.num)
}

pub fn request_tf(req: &model::MahjongRequest) -> HandContext {
    let hand_tiles: Vec<TileWithDora> = req
        .hand_tiles
        .iter()
        .map(|t| tile_with_dora_tf(&t))
        .collect();
    let fuuro: Vec<Fuuro> = req.fuuro.iter().map(|f| fuuro_tf(&f)).collect();
    let agari_tile: TileWithDora = tile_with_dora_tf(&req.agari_tile);
    let dora: Vec<Tile> = req.dora.iter().map(|t| tile_tf(&t)).collect();
    let ura_dora: Vec<Tile> = req.ura_dora.iter().map(|t| tile_tf(&t)).collect();
    let nuki_dora: Vec<TileWithDora> = req
        .nuki_dora
        .iter()
        .map(|t| tile_with_dora_tf(&t))
        .collect();
    let yaku_flags: YakuFlags = YakuFlags {
        menzentsumo: req.yaku_flags.menzentsumo,
        riichi: req.yaku_flags.riichi,
        double_riichi: req.yaku_flags.double_riichi.unwrap_or(false),
        ippatsu: req.yaku_flags.ippatsu,
        haiteiraoyue: req.yaku_flags.haiteiraoyue.unwrap_or(false),
        houteiraoyui: req.yaku_flags.houteiraoyui.unwrap_or(false),
        rinshankaihou: req.yaku_flags.rinshankaihou.unwrap_or(false),
        chankan: req.yaku_flags.chankan.unwrap_or(false),
        tenhou: req.yaku_flags.tenhou.unwrap_or(false),
        tiihou: req.yaku_flags.tiihou.unwrap_or(false),
    };
    HandContext::new(
        hand_tiles, fuuro, agari_tile, req.tsumo, req.bakaze, req.jikaze, dora, ura_dora,
        nuki_dora, yaku_flags,
    )
}

pub fn yaku_enable_map_tf(req: &model::MahjongRequest) -> HashMap<usize, bool> {
    let mut m: HashMap<usize, bool> = HashMap::new();
    for id in &req.disabled_yaku_id {
        m.insert(*id, false);
    }
    m
}
