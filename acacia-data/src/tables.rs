use crate::schemas::{self, BosspvpBossChallengeData, SupportAffixPoolData};
use std::sync::OnceLock;

static TABLES: OnceLock<Tables> = OnceLock::new();

pub fn tables() -> &'static Tables {
    TABLES.get().expect("Failed to use tables")
}

pub fn load_tables() -> anyhow::Result<()> {
    let tables = Tables::new()?;
    TABLES.set(tables).expect("Failed to load tables");
    Ok(())
}

crate::define_tables! {
    pub struct Tables {

        // Activity

        #[path="activity/activities.txt"]
        #[index("id_1", id)]
        #[index("class_1", class)]
        pub activity: DataTable<schemas::ActivityData>,

        #[path="activity/sign/signday.txt"]
        #[index("activity_id_1", activity_id)]
        #[index("activity_id_day_id_1", activity_id, day_id)]
        pub activity_sign_day: DataTable<schemas::ActivitySignDayData>,

        #[path="activity/sign/signshort.txt"]
        #[index("activity_id_1", activity_id)]
        #[index("activity_id_day_id_1", activity_id, day_id)]
        pub activity_sign_short: DataTable<schemas::ActivitySignShortData>,

        #[path="activity/gacha_try/gacha_try.txt"]
        #[index("act_id_1", act_id)]
        pub activity_gacha_try: DataTable<schemas::ActivityGachaTryData>,

        #[path="activity/gacha_try/try_girl.txt"]
        #[index("id_1", id)]
        pub activity_gacha_try_girl: DataTable<schemas::ActivityGachaTryGirlData>,

        // Item

        #[path="item/templates/card.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub card_template: DataTable<schemas::CardTemplateData>,

        #[path="item/templates/weapon.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub weapon_template: DataTable<schemas::WeaponTemplateData>,

        #[path="item/templates/support_card.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub support_card_template: DataTable<schemas::SupportCardTemplateData>,

        #[path="item/templates/others.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub others_template: DataTable<schemas::OthersTemplateData>,

        #[path="item/templates/suplies.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub suplies_template: DataTable<schemas::SupliesTemplateData>,

        #[path="item/templates/weapon_parts.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub weapon_parts_template: DataTable<schemas::WeaponPartsTemplateData>,

        #[path="item/templates/card_skin.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub card_skin_template: DataTable<schemas::CardSkinTemplateData>,

        #[path="item/templates/dorm_gift.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub dorm_gift_template: DataTable<schemas::DormGiftTemplateData>,

        #[path="item/templates/profile.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub profile_template: DataTable<schemas::ProfileTemplateData>,

        #[path="item/templates/weapon_skin.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub weapon_skin_template: DataTable<schemas::WeaponSkinTemplateData>,

        #[path="item/templates/monster_card.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub monster_card_template: DataTable<schemas::MonsterCardTemplateData>,

        #[path="item/templates/manifestation.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub manifestation_template: DataTable<schemas::ManifestationTemplateData>,

        #[path="item/templates/main_scene.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub main_scene_template: DataTable<schemas::MainSceneTemplateData>,

        #[path="item/templates/house_photo.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub house_photo_template: DataTable<schemas::HousePhotoTemplateData>,

        #[path="item/templates/house_commonphoto.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub house_commonphoto_template: DataTable<schemas::HouseCommonphotoTemplateData>,

        #[path="item/templates/ar_item.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub ar_item_template: DataTable<schemas::ArItemTemplateData>,

        #[path="item/templates/call_item.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub call_item_template: DataTable<schemas::CallItemTemplateData>,

        #[path="item/templates/card_skin_parts.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub card_skin_parts_template: DataTable<schemas::CardSkinPartsTemplateData>,

        #[path="item/break.txt"]
        #[index("id_1", id)]
        pub item_break: DataTable<schemas::ItemBreakData>,

        #[path="item/skill/spine.txt"]
        #[index("id_1", id)]
        pub item_spine: DataTable<schemas::ItemSpineData>,

        #[path="item/skill/nodecondition.txt"]
        #[index("id_1", id)]
        pub item_node_condition: DataTable<schemas::ItemNodeConditionData>,

        #[path="item/cardbreak/break.txt"]
        #[index("id_1", id)]
        pub card_break: DataTable<schemas::CardBreakData>,

        #[path="item/cardbreak/breaknew.txt"]
        #[index("id_1", id)]
        pub card_break_new: DataTable<schemas::CardBreakNewData>,

        #[path="item/support/affix_pool.txt"]
        #[index("id_1", id)]
        pub support_affix_pool: DataTable<schemas::SupportAffixPoolData>,

        // House

        #[path="house/gift.txt"]
        #[index("gift_id_1", gift_id)]
        pub gift: DataTable<schemas::GiftData>,

        #[path="house/gift_exchange.txt"]
        #[index("index_1", index)]
        #[index("gift_id_1", gift_id)]
        pub gift_exchange: DataTable<schemas::GiftExchangeData>,

        #[path="house/FurniturePos.txt"]
        #[index("area_id_1", area_id)]
        #[index("area_id_group_id_1", area_id, group_id)]
        pub furniture_pos: DataTable<schemas::FurniturePosData>,

        #[path="house/support_furniture.txt"]
        #[index("area_id_1", area_id)]
        #[index("area_id_index_1", area_id, index)]
        #[index("furniture_tmp_id_1", furniture_tmp_id)]
        pub support_furniture: DataTable<schemas::SupportFurnitureData>,

        #[path="house/open_girl.txt"]
        pub house_open_girl: DataTable<schemas::HouseOpenGirlData>,

        // Others

        #[path="player/rename.txt"]
        #[index("times_1", times)]
        pub rename: DataTable<schemas::RenameData>,

        #[path="cash/cash.txt"]
        #[index("gdpl_1", gdpl)]
        pub cash: DataTable<schemas::CashData>,

        #[path="battlepass/timelist.txt"]
        #[index("id_1", id)]
        pub battlepass_timelist: DataTable<schemas::BattlepassTimelistData>,

        #[path="chapter/level.txt"]
        #[index("id_1", id)]
        pub chapter_level: DataTable<schemas::ChapterLevelData>,

        #[path="drop/drop.txt"]
        #[index("id_1", id)]
        pub drop: DataTable<schemas::DropData>,

        #[path="drop/drop_grop.txt"]
        #[index("id_1", id)]
        pub drop_grop: DataTable<schemas::DropGropData>,

        #[path="riki/Riki.txt"]
        #[index("type_condition_1", r#type, condition)]
        pub riki: DataTable<schemas::RikiData>,

        #[path="challenge/bosspvp/boss_challenge.txt"]
        pub bosspvp_boss_challenge: DataTable<schemas::BosspvpBossChallengeData>,
    }
}

#[cfg(test)]
mod tests {
    use crate::tables;

    #[test]
    fn test_data() -> anyhow::Result<()> {
        tables::load_tables()?;
        let data = tables::tables().gift_exchange.find_one("gift_id_1", 2306);
        println!("{:?}", data);
        let data = tables::tables().dorm_gift_template.find_one("gdpl", [8, 23, 11, 3]).unwrap();
        println!("{:?}", data);
        let data = tables::tables().gift.find_one("gift_id_1", data.param1);
        println!("{:?}", data);
        let data = tables::tables().chapter_level.find_one("id_1", 50174);
        println!("{:?}", data);
        let (g, d, p, l) = (1, 1, 2, 1);
        let condition = format!("{{{g},{d},{p},{l}}}");
        let data = tables::tables().riki.find_one("type_condition_1", (1, &condition));
        println!("{:?}", data);
        // println!("{:?}", tables::tables().battlepass_timelist.all());
        Ok(())
    }
}
