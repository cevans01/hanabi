use std::fmt::Debug;

fn generate_uid() -> UID {
    rand::random::<u64>()
}

//#[derive(Debug)]
pub type UID = u64;

//#[derive(Debug)]
pub type PubID = u8;

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub public_id: PubID,

    pub uid: UID,
    pub cohorts: Vec<PubID>,
}

impl Player {
    pub fn new(public_id: u8, uid: u64) -> Player {
        Player {
            public_id,
            uid,
            cohorts: Vec::new(),
        }
    }
}

/**
 * @brief Create players, initialized with IDs
 */
pub fn generate_players(num_players: usize) -> Vec<Player> {
    assert!(num_players < 6 && num_players > 1);

    let mut players = Vec::new();
    let all_public_ids: Vec<u8> = (0..num_players as u8).collect();

    // Create the players
    for public_id in 0..num_players {
        let uid = generate_uid();
        let other_public_ids: Vec<u8> = all_public_ids
            .iter()
            .filter(|&p| *p != public_id as u8)
            .cloned()
            .collect();
        let new_player = Player {
            public_id: public_id as u8,
            uid,
            cohorts: other_public_ids,
        };
        players.push(new_player);
    }
    players
}
