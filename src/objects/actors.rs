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

//! This module contains all actors in the game.

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::sync::RwLock;

/// Struct representing an actor in the game.
#[derive(Debug, Clone)]
pub struct Actor {
    /// The name of the actor.
    pub name: String,
    /// The rarity of the actor.
    pub rarity: super::Rarity,
    /// The name of the actor in the game logs.
    pub log_name: String,
}

impl Actor {
    /// Retrieves an actor given its name.
    ///
    /// # Arguments
    ///
    /// * `actor` - The name of the actor to retrieve.
    ///
    /// # Return
    ///
    /// This function will return an `Option<Actor>`. If an actor with the given name exists,
    /// it will return `Some(Actor)`. If no such actor exists, it will return `None`.
    pub fn get(actor: String) -> Option<Self> {
        let actors = get_actors();
        let map = actors.read().unwrap();
        let actor = map.get(&actor.to_lowercase());
        actor.map(|actor| actor.clone())
    }
}

/// Retrieve the actors stored in the ACTORS lazy static variable.
///
/// # Return
///
/// This function will return a reference to the ACTORS static variable.
fn get_actors() -> &'static ACTORS {
    &ACTORS
}

lazy_static! {
    /// Store all actors in a thread-safe data structure.
    #[derive(Debug)]
    static ref ACTORS: RwLock<HashMap<String, Actor>> = {
        let mut actors = HashMap::new();

        /// Helper function to create an actor.
        fn create_actor(name: &str, rarity: super::Rarity, log_name: &str) -> Actor {
            Actor {
                name: name.to_string(),
                rarity,
                log_name: log_name.to_string(),
            }
        }

        // List of all actors.
        let actor_list = vec![
            create_actor("None", super::Rarity::Common, "None"),
            create_actor("Player", super::Rarity::Common, "PRO_PlayerCharacter"),
            create_actor("Strider", super::Rarity::Common, "AIChar_Strider_BP"),
            create_actor("Rattler", super::Rarity::Uncommon, "AIChar_Rattler_BP"),
            create_actor("Crusher", super::Rarity::Epic, "AIChar_Crusher_BP"),
            create_actor("Weremole", super::Rarity::Rainbow, "AIChar_Weremole_BP"),
            create_actor("Howler", super::Rarity::Rainbow, "AIChar_Howler_BP"),
        ];

        // Insert each actor into the HashMap.
        for actor in actor_list {
            actors.insert(actor.log_name.to_lowercase(), actor);
        }

        // Return the HashMap as a RwLock for thread-safety.
        RwLock::new(actors)
    };
}
