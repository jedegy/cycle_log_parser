// Copyright (c) 2023
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This module contains all that can kill the player in the game.

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::RwLock;

/// Struct representing a weapon in the game.
#[derive(Debug, Clone)]
pub struct Weapon {
    /// The name of the weapon.
    pub name: String,
    /// The rarity of the weapon.
    pub rarity: super::Rarity,
    /// The name of the weapon in the game logs.
    pub log_name: String,
}

impl Weapon {
    /// Retrieves a weapon given its name.
    ///
    /// # Arguments
    ///
    /// * `weapon` - The name of the weapon to retrieve.
    ///
    /// # Return
    ///
    /// This function will return an `Option<Weapon>`. If a weapon with the given name exists,
    /// it will return `Some(Weapon)`. If no such weapon exists, it will return `None`.
    pub fn get(weapon: &str) -> Option<Self> {
        let weapons = get_weapons();
        let map = weapons.read().unwrap();
        let weapon = map.get(&weapon.to_lowercase());
        weapon.map(|weapon| weapon.clone())
    }
}

/// Retrieve the weapons stored in the WEAPONS lazy static variable.
///
/// # Return
///
/// This function will return a reference to the WEAPONS static variable.
fn get_weapons() -> &'static WEAPONS {
    &WEAPONS
}

lazy_static! {
    /// Store all weapons in a thread-safe data structure.
    #[derive(Debug)]
    static ref WEAPONS: RwLock<HashMap<String, Weapon>> = {
        let mut weapons = HashMap::new();

        /// Helper function to create a weapon.
        fn create_weapon(name: &str, rarity: super::Rarity, log_name: &str) -> Weapon {
            Weapon {
                name: name.to_string(),
                rarity,
                log_name: log_name.to_string(),
            }
        }

        // List of all weapons.
        let weapon_list = vec![
            create_weapon("None", super::Rarity::Common, "None"),
            create_weapon(
                "K_28 (Scrappy)",
                super::Rarity::Common,
                "WP_E_Pistol_Bullet_01_scrappy",
            ),
            create_weapon("K_28", super::Rarity::Common, "WP_E_Pistol_Bullet_01"),
            create_weapon(
                "B9_Trenchgun (Scrappy)",
                super::Rarity::Common,
                "WP_E_SGun_Bullet_01_scrappy",
            ),
            create_weapon("B9_Trenchgun", super::Rarity::Common, "WP_E_SGun_Bullet_01"),
            create_weapon(
                "S_576 (Scrappy)",
                super::Rarity::Common,
                "WP_E_SMG_Bullet_01_scrappy",
            ),
            create_weapon("S_576", super::Rarity::Common, "WP_E_SMG_Bullet_01"),
            create_weapon("S_576", super::Rarity::Uncommon, "WP_E_SMG_Bullet_02"),
            create_weapon(
                "AR_55 (Scrappy)",
                super::Rarity::Common,
                "WP_E_AR_Energy_01_scrappy",
            ),
            create_weapon("AR_55", super::Rarity::Common, "WP_E_AR_Energy_01"),
            create_weapon("AR_55", super::Rarity::Uncommon, "WP_E_AR_Energy_02"),
            create_weapon("C_32_Bolt", super::Rarity::Common, "WP_E_Sniper_Bullet_01"),
            create_weapon("C_32_Bolt", super::Rarity::Uncommon, "WP_E_Sniper_Bullet_02"),
            create_weapon("Bulldog", super::Rarity::Uncommon, "WP_D_Pistol_Bullet_01"),
            create_weapon("Guarantee", super::Rarity::Uncommon, "WP_D_LMG_Energy_02"),
            create_weapon("Guarantee", super::Rarity::Rare, "WP_D_LMG_Energy_01"),
            create_weapon("Lacerator", super::Rarity::Rare, "WP_D_BR_Shard_01"),
            create_weapon("Shattergun", super::Rarity::Epic, "WP_D_SGun_Shard_01"),
            create_weapon("Advocate", super::Rarity::Epic, "WP_D_AR_Bullet_01"),
            create_weapon("Voltaic_brute", super::Rarity::Exotic, "WP_D_SMG_Energy_01"),
            create_weapon("Kinetic_arbiter", super::Rarity::Exotic, "WP_D_Sniper_Gauss_01"),
            create_weapon("Scrapper", super::Rarity::Uncommon, "WP_A_SMG_Shard_01"),
            create_weapon("Maelstorm", super::Rarity::Rare, "WP_A_SGun_Energy_01"),
            create_weapon("Longshot", super::Rarity::Rare, "WP_A_BR_Bullet_02"),
            create_weapon("Longshot", super::Rarity::Epic, "WP_A_BR_Bullet_01"),
            create_weapon("Hammer", super::Rarity::Rare, "WP_A_Pistol_Bullet_02"),
            create_weapon("Hammer", super::Rarity::Exotic, "WP_A_Pistol_Bullet_01"),
            create_weapon("KOR", super::Rarity::Exotic, "WP_A_AR_Bullet_01"),
            create_weapon("Scarab", super::Rarity::Uncommon, "WP_G_Pistol_Energy_01"),
            create_weapon("Scarab", super::Rarity::Rare, "WP_G_Pistol_Energy_02"),
            create_weapon("Manticore", super::Rarity::Uncommon, "WP_G_AR_Needle_01"),
            create_weapon("Manticore", super::Rarity::Rare, "WP_G_AR_Needle_02"),
            create_weapon("Phasic Lancer", super::Rarity::Rare, "WP_G_AR_Energy_01"),
            create_weapon("Flechette Gun", super::Rarity::Rare, "WP_G_SMG_Needle_02"),
            create_weapon("Flechette Gun", super::Rarity::Epic, "WP_G_SMG_Needle_01"),
            create_weapon("Gorgon", super::Rarity::Epic, "WP_G_AR_Beam_01"),
            create_weapon("Basilisk", super::Rarity::Exotic, "WP_G_Sniper_Energy_01"),
            create_weapon("KARMA", super::Rarity::Epic, "WP_A_Sniper_Gauss_02"),
            create_weapon("KARMA", super::Rarity::Legendary, "WP_A_Sniper_Gauss_01"),
            create_weapon("KOMRAD", super::Rarity::Legendary, "WP_A_Launch_MSL_01"),
            create_weapon("ZEUS", super::Rarity::Epic, "WP_G_HVY_Beam_02"),
            create_weapon("ZEUS", super::Rarity::Legendary, "WP_G_HVY_Beam_01"),
            create_weapon("Knife", super::Rarity::Rainbow, "Melee_Knife_01"),
            create_weapon("Shock Grenade", super::Rarity::Common, "ShockGrenade_01"),
            create_weapon("Shock Grenade", super::Rarity::Uncommon, "ShockGrenade_02"),
            create_weapon("Shock Grenade", super::Rarity::Rare, "ShockGrenade_03"),
            create_weapon("Shock Grenade", super::Rarity::Epic, "ShockGrenade_04"),
            create_weapon("Shock Grenade", super::Rarity::Exotic, "ShockGrenade_05"),
            create_weapon("Gas Grenade", super::Rarity::Uncommon, "Consumable_GasGrenade_01"),
            create_weapon("Suicide", super::Rarity::Common, "Suicide"),
            create_weapon("Fall", super::Rarity::Uncommon, "Fall"),
            create_weapon("Lightning Strike", super::Rarity::Rare, "LightningStrike_BP"),
        ];

        // Insert each weapon into the HashMap.
        for weapon in weapon_list {
            weapons.insert(weapon.log_name.to_lowercase(), weapon);
        }

        // Return the HashMap as a RwLock for thread-safety.
        RwLock::new(weapons)
    };
}
