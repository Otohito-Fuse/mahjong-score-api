use mahjong_score::mahjong::Points;
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub r#type: String,
    #[serde(rename = "mahjongRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mahjong_request: Option<MahjongRequest>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MahjongRequest {
    #[serde(rename = "handTiles")]
    pub hand_tiles: Vec<TileRepresentation>,
    pub fuuro: Vec<FuuroRepresentaion>,
    #[serde(rename = "agariTile")]
    pub agari_tile: TileRepresentation,
    pub tsumo: bool,
    #[serde(deserialize_with = "deserialize_kaze")]
    pub bakaze: usize,
    #[serde(deserialize_with = "deserialize_kaze")]
    pub jikaze: usize,
    pub dora: Vec<TileRepresentation>,
    #[serde(rename = "uraDora")]
    pub ura_dora: Vec<TileRepresentation>,
    #[serde(rename = "nukiDora")]
    pub nuki_dora: Vec<TileRepresentation>,
    #[serde(rename = "yakuFlags")]
    pub yaku_flags: YakuFlags,
    #[serde(rename = "disabledYakuId")]
    pub disabled_yaku_id: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileRepresentation {
    pub r#type: usize,
    pub num: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dora: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FuuroRepresentaion {
    pub r#type: String,
    pub tiles: Vec<TileRepresentation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YakuFlags {
    pub menzentsumo: bool,
    pub riichi: bool,
    #[serde(rename = "doubleRiichi")]
    pub double_riichi: Option<bool>,
    pub ippatsu: bool,
    pub haiteiraoyue: Option<bool>,
    pub houteiraoyui: Option<bool>,
    pub rinshankaihou: Option<bool>,
    pub chankan: Option<bool>,
    pub tenhou: Option<bool>,
    pub tiihou: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    #[serde(rename = "mahjongResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mahjong_response: Option<MahjongResponse>,
    pub request: Request,
}

impl Response {
    pub fn new(
        message: String,
        mahjong_response: Option<MahjongResponse>,
        request: Request,
    ) -> Self {
        Self {
            message,
            mahjong_response,
            request,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MahjongResponse {
    pub yaku: Vec<Yaku>,
    #[serde(rename = "doraNum")]
    pub dora_num: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fu: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yakuman: Option<usize>,
    pub score_oya: Points,
    pub score_ko: Points,
    #[serde(rename = "scoreTitle")]
    pub score_title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Yaku {
    pub name: String,
    pub fan: usize,
}

fn deserialize_kaze<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "1" | "ton" | "TON" | "???" => Ok(1),
        "2" | "nan" | "NAN" | "???" => Ok(2),
        "3" | "shaa" | "SHAA" | "???" => Ok(3),
        "4" | "pei" | "PEI" | "???" => Ok(4),
        _ => Err(de::Error::unknown_variant(
            s,
            &[
                "1", "ton", "TON", "???", "2", "nan", "NAN", "???", "3", "shaa", "SHAA", "???", "4",
                "pei", "PEI", "???",
            ],
        )),
    }
}
