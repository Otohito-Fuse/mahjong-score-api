use super::model;
use super::transformation;
use mahjong_score::mahjong::{
    calc_dora, detect_winning, get_points, get_score_title, DoraInfo, HandContext, Points, Yaku,
    YakuContext,
};
use std::collections::HashMap;

pub fn calc(req: &model::MahjongRequest) -> Option<model::MahjongResponse> {
    let ctx: HandContext = transformation::request_tf(&req);
    let dora_info: DoraInfo = calc_dora(&ctx);
    let ctx_v: Vec<YakuContext> = detect_winning(&ctx);
    if ctx_v.len() == 0 {
        return None;
    }
    let is_open: bool = ctx_v[0].is_open();
    let dora_num = dora_info.dora_fan_sum;
    let m: HashMap<usize, bool> = transformation::yaku_enable_map_tf(&req);
    let mut yaku_v: Vec<(Vec<&'static Yaku>, usize, usize, usize)> = ctx_v
        .into_iter()
        .map(|ctx| (ctx.calc_yaku(&m), ctx.calc_fu()))
        .map(|((v, a, b), c)| (v, if a == 0 { 0 } else { a + dora_num }, b, c))
        .collect();
    yaku_v.sort_by_key(|(_, fan, yakuman, fu)| get_points(true, *fu, *fan, *yakuman).0);
    yaku_v.reverse();
    let (v, fan, yakuman, fu) = &yaku_v[0];
    let yaku: Vec<model::Yaku> = v
        .iter()
        .map(|v| model::Yaku {
            name: v.name.to_string(),
            fan: if is_open { v.fan_open } else { v.fan_close },
        })
        .collect();
    let score_oya: Points = get_points(true, *fu, *fan, *yakuman);
    let score_ko: Points = get_points(false, *fu, *fan, *yakuman);
    let score_title: String = get_score_title(*fu, *fan, *yakuman);
    Some(model::MahjongResponse {
        yaku,
        dora_num,
        fu: if *yakuman == 0 { Some(*fu) } else { None },
        fan: if *yakuman == 0 { Some(*fan) } else { None },
        yakuman: if *yakuman == 0 { None } else { Some(*yakuman) },
        score_oya,
        score_ko,
        score_title,
    })
}
