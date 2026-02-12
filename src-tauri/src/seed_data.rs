use crate::database::Database;
use crate::models::{Bundle, Item};
use rusqlite::Result;

pub fn seed_database(db: &Database) -> Result<()> {
    // Verificar se já tem dados
    match db.get_all_bundles() {
        Ok(bundles) => {
            if !bundles.is_empty() {
                println!("Database already seeded, skipping...");
                return Ok(());
            }
        }
        Err(e) => {
            println!("Warning checking existing data: {}", e);
        }
    }

    println!("Seeding database with Community Center bundles...");

    // PANTRY (6 bundles)
    seed_pantry_bundles(db)?;

    // CRAFTS ROOM (6 bundles)
    seed_crafts_room_bundles(db)?;

    // FISH TANK (6 bundles)
    seed_fish_tank_bundles(db)?;

    // BOILER ROOM (3 bundles)
    seed_boiler_room_bundles(db)?;

    // BULLETIN BOARD (5 bundles)
    seed_bulletin_board_bundles(db)?;

    // VAULT (4 bundles)
    seed_vault_bundles(db)?;

    println!("✅ Database seeded successfully with all 30 bundles!");
    Ok(())
}

// ========================================
// PANTRY - 6 Bundles
// ========================================

fn seed_pantry_bundles(db: &Database) -> Result<()> {
    // Spring Crops Bundle
    let bundle = Bundle {
        id: "spring_crops".to_string(),
        name: "Spring Crops Bundle".to_string(),
        room: "Pantry".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("spring_parsnip", "spring_crops", "Parsnip"),
        Item::new("spring_green_bean", "spring_crops", "Green Bean"),
        Item::new("spring_cauliflower", "spring_crops", "Cauliflower"),
        Item::new("spring_potato", "spring_crops", "Potato"),
    ] {
        db.insert_item(&item)?;
    }

    // Summer Crops Bundle
    let bundle = Bundle {
        id: "summer_crops".to_string(),
        name: "Summer Crops Bundle".to_string(),
        room: "Pantry".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("summer_tomato", "summer_crops", "Tomato"),
        Item::new("summer_hot_pepper", "summer_crops", "Hot Pepper"),
        Item::new("summer_blueberry", "summer_crops", "Blueberry"),
        Item::new("summer_melon", "summer_crops", "Melon"),
    ] {
        db.insert_item(&item)?;
    }

    // Fall Crops Bundle
    let bundle = Bundle {
        id: "fall_crops".to_string(),
        name: "Fall Crops Bundle".to_string(),
        room: "Pantry".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fall_corn", "fall_crops", "Corn"),
        Item::new("fall_eggplant", "fall_crops", "Eggplant"),
        Item::new("fall_pumpkin", "fall_crops", "Pumpkin"),
        Item::new("fall_yam", "fall_crops", "Yam"),
    ] {
        db.insert_item(&item)?;
    }

    // Quality Crops Bundle
    let bundle = Bundle {
        id: "quality_crops".to_string(),
        name: "Quality Crops Bundle".to_string(),
        room: "Pantry".to_string(),
        required_items: 3,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new_with_quality("quality_parsnip", "quality_crops", "Parsnip", "gold"),
        Item::new_with_quality("quality_melon", "quality_crops", "Melon", "gold"),
        Item::new_with_quality("quality_pumpkin", "quality_crops", "Pumpkin", "gold"),
        Item::new_with_quality("quality_corn", "quality_crops", "Corn", "gold"),
    ] {
        db.insert_item(&item)?;
    }

    // Animal Bundle
    let bundle = Bundle {
        id: "animal".to_string(),
        name: "Animal Bundle".to_string(),
        room: "Pantry".to_string(),
        required_items: 5,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("animal_large_milk", "animal", "Large Milk"),
        Item::new("animal_large_egg_brown", "animal", "Large Brown Egg"),
        Item::new("animal_large_egg_white", "animal", "Large Egg"),
        Item::new("animal_large_goat_milk", "animal", "Large Goat Milk"),
        Item::new("animal_wool", "animal", "Wool"),
        Item::new("animal_duck_egg", "animal", "Duck Egg"),
    ] {
        db.insert_item(&item)?;
    }

    // Artisan Bundle
    let bundle = Bundle {
        id: "artisan".to_string(),
        name: "Artisan Bundle".to_string(),
        room: "Pantry".to_string(),
        required_items: 6,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("artisan_truffle_oil", "artisan", "Truffle Oil"),
        Item::new("artisan_cloth", "artisan", "Cloth"),
        Item::new("artisan_goat_cheese", "artisan", "Goat Cheese"),
        Item::new("artisan_cheese", "artisan", "Cheese"),
        Item::new("artisan_honey", "artisan", "Honey"),
        Item::new("artisan_jelly", "artisan", "Jelly"),
        Item::new("artisan_apple", "artisan", "Apple"),
        Item::new("artisan_apricot", "artisan", "Apricot"),
        Item::new("artisan_orange", "artisan", "Orange"),
        Item::new("artisan_peach", "artisan", "Peach"),
        Item::new("artisan_pomegranate", "artisan", "Pomegranate"),
        Item::new("artisan_cherry", "artisan", "Cherry"),
    ] {
        db.insert_item(&item)?;
    }

    Ok(())
}

// ========================================
// CRAFTS ROOM - 6 Bundles
// ========================================

fn seed_crafts_room_bundles(db: &Database) -> Result<()> {
    // Spring Foraging Bundle
    let bundle = Bundle {
        id: "spring_foraging".to_string(),
        name: "Spring Foraging Bundle".to_string(),
        room: "Crafts Room".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new(
            "forage_wild_horseradish",
            "spring_foraging",
            "Wild Horseradish",
        ),
        Item::new("forage_daffodil", "spring_foraging", "Daffodil"),
        Item::new("forage_leek", "spring_foraging", "Leek"),
        Item::new("forage_dandelion", "spring_foraging", "Dandelion"),
    ] {
        db.insert_item(&item)?;
    }

    // Summer Foraging Bundle
    let bundle = Bundle {
        id: "summer_foraging".to_string(),
        name: "Summer Foraging Bundle".to_string(),
        room: "Crafts Room".to_string(),
        required_items: 3,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("forage_grape", "summer_foraging", "Grape"),
        Item::new("forage_spice_berry", "summer_foraging", "Spice Berry"),
        Item::new("forage_sweet_pea", "summer_foraging", "Sweet Pea"),
    ] {
        db.insert_item(&item)?;
    }

    // Fall Foraging Bundle
    let bundle = Bundle {
        id: "fall_foraging".to_string(),
        name: "Fall Foraging Bundle".to_string(),
        room: "Crafts Room".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("forage_common_mushroom", "fall_foraging", "Common Mushroom"),
        Item::new("forage_wild_plum", "fall_foraging", "Wild Plum"),
        Item::new("forage_hazelnut", "fall_foraging", "Hazelnut"),
        Item::new("forage_blackberry", "fall_foraging", "Blackberry"),
    ] {
        db.insert_item(&item)?;
    }

    // Winter Foraging Bundle
    let bundle = Bundle {
        id: "winter_foraging".to_string(),
        name: "Winter Foraging Bundle".to_string(),
        room: "Crafts Room".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("forage_winter_root", "winter_foraging", "Winter Root"),
        Item::new("forage_crystal_fruit", "winter_foraging", "Crystal Fruit"),
        Item::new("forage_snow_yam", "winter_foraging", "Snow Yam"),
        Item::new("forage_crocus", "winter_foraging", "Crocus"),
    ] {
        db.insert_item(&item)?;
    }

    // Construction Bundle
    let bundle = Bundle {
        id: "construction".to_string(),
        name: "Construction Bundle".to_string(),
        room: "Crafts Room".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("construction_wood", "construction", "Wood (99)"),
        Item::new("construction_stone", "construction", "Stone (99)"),
        Item::new("construction_hardwood", "construction", "Hardwood (10)"),
        Item::new("construction_clay", "construction", "Clay (10)"),
    ] {
        db.insert_item(&item)?;
    }

    // Exotic Foraging Bundle
    let bundle = Bundle {
        id: "exotic_foraging".to_string(),
        name: "Exotic Foraging Bundle".to_string(),
        room: "Crafts Room".to_string(),
        required_items: 5,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("exotic_coconut", "exotic_foraging", "Coconut"),
        Item::new("exotic_cactus_fruit", "exotic_foraging", "Cactus Fruit"),
        Item::new("exotic_cave_carrot", "exotic_foraging", "Cave Carrot"),
        Item::new("exotic_red_mushroom", "exotic_foraging", "Red Mushroom"),
        Item::new(
            "exotic_purple_mushroom",
            "exotic_foraging",
            "Purple Mushroom",
        ),
        Item::new("exotic_maple_syrup", "exotic_foraging", "Maple Syrup"),
        Item::new("exotic_oak_resin", "exotic_foraging", "Oak Resin"),
        Item::new("exotic_pine_tar", "exotic_foraging", "Pine Tar"),
        Item::new("exotic_morel", "exotic_foraging", "Morel"),
    ] {
        db.insert_item(&item)?;
    }

    Ok(())
}

// ========================================
// FISH TANK - 6 Bundles
// ========================================

fn seed_fish_tank_bundles(db: &Database) -> Result<()> {
    // River Fish Bundle
    let bundle = Bundle {
        id: "river_fish".to_string(),
        name: "River Fish Bundle".to_string(),
        room: "Fish Tank".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fish_sunfish", "river_fish", "Sunfish"),
        Item::new("fish_catfish", "river_fish", "Catfish"),
        Item::new("fish_shad", "river_fish", "Shad"),
        Item::new("fish_tiger_trout", "river_fish", "Tiger Trout"),
    ] {
        db.insert_item(&item)?;
    }

    // Lake Fish Bundle
    let bundle = Bundle {
        id: "lake_fish".to_string(),
        name: "Lake Fish Bundle".to_string(),
        room: "Fish Tank".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fish_largemouth_bass", "lake_fish", "Largemouth Bass"),
        Item::new("fish_carp", "lake_fish", "Carp"),
        Item::new("fish_bullhead", "lake_fish", "Bullhead"),
        Item::new("fish_sturgeon", "lake_fish", "Sturgeon"),
    ] {
        db.insert_item(&item)?;
    }

    // Ocean Fish Bundle
    let bundle = Bundle {
        id: "ocean_fish".to_string(),
        name: "Ocean Fish Bundle".to_string(),
        room: "Fish Tank".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fish_sardine", "ocean_fish", "Sardine"),
        Item::new("fish_tuna", "ocean_fish", "Tuna"),
        Item::new("fish_red_snapper", "ocean_fish", "Red Snapper"),
        Item::new("fish_tilapia", "ocean_fish", "Tilapia"),
    ] {
        db.insert_item(&item)?;
    }

    // Night Fishing Bundle
    let bundle = Bundle {
        id: "night_fishing".to_string(),
        name: "Night Fishing Bundle".to_string(),
        room: "Fish Tank".to_string(),
        required_items: 3,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fish_walleye", "night_fishing", "Walleye"),
        Item::new("fish_bream", "night_fishing", "Bream"),
        Item::new("fish_eel", "night_fishing", "Eel"),
    ] {
        db.insert_item(&item)?;
    }

    // Specialty Fish Bundle
    let bundle = Bundle {
        id: "specialty_fish".to_string(),
        name: "Specialty Fish Bundle".to_string(),
        room: "Fish Tank".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fish_pufferfish", "specialty_fish", "Pufferfish"),
        Item::new("fish_ghostfish", "specialty_fish", "Ghostfish"),
        Item::new("fish_sandfish", "specialty_fish", "Sandfish"),
        Item::new("fish_woodskip", "specialty_fish", "Woodskip"),
    ] {
        db.insert_item(&item)?;
    }

    // Crab Pot Bundle
    let bundle = Bundle {
        id: "crab_pot".to_string(),
        name: "Crab Pot Bundle".to_string(),
        room: "Fish Tank".to_string(),
        required_items: 5,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("crab_lobster", "crab_pot", "Lobster"),
        Item::new("crab_crayfish", "crab_pot", "Crayfish"),
        Item::new("crab_crab", "crab_pot", "Crab"),
        Item::new("crab_cockle", "crab_pot", "Cockle"),
        Item::new("crab_mussel", "crab_pot", "Mussel"),
        Item::new("crab_shrimp", "crab_pot", "Shrimp"),
        Item::new("crab_snail", "crab_pot", "Snail"),
        Item::new("crab_periwinkle", "crab_pot", "Periwinkle"),
        Item::new("crab_oyster", "crab_pot", "Oyster"),
        Item::new("crab_clam", "crab_pot", "Clam"),
    ] {
        db.insert_item(&item)?;
    }

    Ok(())
}

// ========================================
// BOILER ROOM - 3 Bundles
// ========================================

fn seed_boiler_room_bundles(db: &Database) -> Result<()> {
    // Blacksmith's Bundle
    let bundle = Bundle {
        id: "blacksmith".to_string(),
        name: "Blacksmith's Bundle".to_string(),
        room: "Boiler Room".to_string(),
        required_items: 3,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("blacksmith_copper_bar", "blacksmith", "Copper Bar"),
        Item::new("blacksmith_iron_bar", "blacksmith", "Iron Bar"),
        Item::new("blacksmith_gold_bar", "blacksmith", "Gold Bar"),
    ] {
        db.insert_item(&item)?;
    }

    // Geologist's Bundle
    let bundle = Bundle {
        id: "geologist".to_string(),
        name: "Geologist's Bundle".to_string(),
        room: "Boiler Room".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("geo_quartz", "geologist", "Quartz"),
        Item::new("geo_earth_crystal", "geologist", "Earth Crystal"),
        Item::new("geo_frozen_tear", "geologist", "Frozen Tear"),
        Item::new("geo_fire_quartz", "geologist", "Fire Quartz"),
    ] {
        db.insert_item(&item)?;
    }

    // Adventurer's Bundle
    let bundle = Bundle {
        id: "adventurer".to_string(),
        name: "Adventurer's Bundle".to_string(),
        room: "Boiler Room".to_string(),
        required_items: 2,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("adv_slime", "adventurer", "Slime (99)"),
        Item::new("adv_bat_wing", "adventurer", "Bat Wing (10)"),
        Item::new("adv_solar_essence", "adventurer", "Solar Essence"),
        Item::new("adv_void_essence", "adventurer", "Void Essence"),
    ] {
        db.insert_item(&item)?;
    }

    Ok(())
}

// ========================================
// BULLETIN BOARD - 5 Bundles
// ========================================

fn seed_bulletin_board_bundles(db: &Database) -> Result<()> {
    // Chef's Bundle
    let bundle = Bundle {
        id: "chef".to_string(),
        name: "Chef's Bundle".to_string(),
        room: "Bulletin Board".to_string(),
        required_items: 6,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("chef_maple_syrup", "chef", "Maple Syrup"),
        Item::new("chef_fiddlehead_fern", "chef", "Fiddlehead Fern"),
        Item::new("chef_truffle", "chef", "Truffle"),
        Item::new("chef_poppy", "chef", "Poppy"),
        Item::new("chef_maki_roll", "chef", "Maki Roll"),
        Item::new("chef_fried_egg", "chef", "Fried Egg"),
    ] {
        db.insert_item(&item)?;
    }

    // Dye Bundle
    let bundle = Bundle {
        id: "dye".to_string(),
        name: "Dye Bundle".to_string(),
        room: "Bulletin Board".to_string(),
        required_items: 6,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("dye_red_mushroom", "dye", "Red Mushroom"),
        Item::new("dye_sea_urchin", "dye", "Sea Urchin"),
        Item::new("dye_sunflower", "dye", "Sunflower"),
        Item::new("dye_duck_feather", "dye", "Duck Feather"),
        Item::new("dye_aquamarine", "dye", "Aquamarine"),
        Item::new("dye_red_cabbage", "dye", "Red Cabbage"),
    ] {
        db.insert_item(&item)?;
    }

    // Field Research Bundle
    let bundle = Bundle {
        id: "field_research".to_string(),
        name: "Field Research Bundle".to_string(),
        room: "Bulletin Board".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("field_purple_mushroom", "field_research", "Purple Mushroom"),
        Item::new("field_nautilus_shell", "field_research", "Nautilus Shell"),
        Item::new("field_chub", "field_research", "Chub"),
        Item::new("field_frozen_geode", "field_research", "Frozen Geode"),
    ] {
        db.insert_item(&item)?;
    }

    // Fodder Bundle
    let bundle = Bundle {
        id: "fodder".to_string(),
        name: "Fodder Bundle".to_string(),
        room: "Bulletin Board".to_string(),
        required_items: 3,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("fodder_wheat", "fodder", "Wheat (10)"),
        Item::new("fodder_hay", "fodder", "Hay (10)"),
        Item::new("fodder_apple", "fodder", "Apple (3)"),
    ] {
        db.insert_item(&item)?;
    }

    // Enchanter's Bundle
    let bundle = Bundle {
        id: "enchanter".to_string(),
        name: "Enchanter's Bundle".to_string(),
        room: "Bulletin Board".to_string(),
        required_items: 4,
        items: None,
    };
    db.insert_bundle(&bundle)?;

    for item in vec![
        Item::new("ench_oak_resin", "enchanter", "Oak Resin"),
        Item::new("ench_wine", "enchanter", "Wine"),
        Item::new("ench_rabbit_foot", "enchanter", "Rabbit's Foot"),
        Item::new("ench_pomegranate", "enchanter", "Pomegranate"),
    ] {
        db.insert_item(&item)?;
    }

    Ok(())
}

// ========================================
// VAULT - 4 Bundles
// ========================================

fn seed_vault_bundles(db: &Database) -> Result<()> {
    // 2,500g Bundle
    let bundle = Bundle {
        id: "vault_2500".to_string(),
        name: "2,500g Bundle".to_string(),
        room: "Vault".to_string(),
        required_items: 1,
        items: None,
    };
    db.insert_bundle(&bundle)?;
    db.insert_item(&Item::new("vault_2500g", "vault_2500", "2,500g"))?;

    // 5,000g Bundle
    let bundle = Bundle {
        id: "vault_5000".to_string(),
        name: "5,000g Bundle".to_string(),
        room: "Vault".to_string(),
        required_items: 1,
        items: None,
    };
    db.insert_bundle(&bundle)?;
    db.insert_item(&Item::new("vault_5000g", "vault_5000", "5,000g"))?;

    // 10,000g Bundle
    let bundle = Bundle {
        id: "vault_10000".to_string(),
        name: "10,000g Bundle".to_string(),
        room: "Vault".to_string(),
        required_items: 1,
        items: None,
    };
    db.insert_bundle(&bundle)?;
    db.insert_item(&Item::new("vault_10000g", "vault_10000", "10,000g"))?;

    // 25,000g Bundle
    let bundle = Bundle {
        id: "vault_25000".to_string(),
        name: "25,000g Bundle".to_string(),
        room: "Vault".to_string(),
        required_items: 1,
        items: None,
    };
    db.insert_bundle(&bundle)?;
    db.insert_item(&Item::new("vault_25000g", "vault_25000", "25,000g"))?;

    Ok(())
}

// ========================================
// Helper Functions
// ========================================

impl Item {
    fn new(id: &str, bundle_id: &str, name: &str) -> Self {
        Item {
            id: id.to_string(),
            bundle_id: bundle_id.to_string(),
            name: name.to_string(),
            status: "missing".to_string(),
            quality: None,
        }
    }

    fn new_with_quality(id: &str, bundle_id: &str, name: &str, quality: &str) -> Self {
        Item {
            id: id.to_string(),
            bundle_id: bundle_id.to_string(),
            name: name.to_string(),
            status: "missing".to_string(),
            quality: Some(quality.to_string()),
        }
    }
}
