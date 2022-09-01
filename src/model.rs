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
    #[serde(deserialize_with = "deserialize_bool")]
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
    #[serde(deserialize_with = "deserialize_bool")]
    pub menzentsumo: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub riichi: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(rename = "doubleRiichi")]
    pub double_riichi: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ippatsu: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub haiteiraoyue: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub houteiraoyui: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rinshankaihou: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub chankan: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub tenhou: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub tiihou: bool,
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

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "yes" | "Yes" | "YES" | "true" => Ok(true),
        "no" | "No" | "NO" | "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(
            s,
            &["yes", "Yes", "YES", "true", "no", "No", "NO", "false"],
        )),
    }
}

fn deserialize_kaze<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "1" | "ton" | "TON" | "東" => Ok(1),
        "2" | "nan" | "NAN" | "南" => Ok(2),
        "3" | "shaa" | "SHAA" | "西" => Ok(3),
        "4" | "pei" | "PEI" | "北" => Ok(4),
        _ => Err(de::Error::unknown_variant(
            s,
            &[
                "1", "ton", "TON", "東", "2", "nan", "NAN", "南", "3", "shaa", "SHAA", "西", "4",
                "pei", "PEI", "北",
            ],
        )),
    }
}
