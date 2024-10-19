use crate::{error::RNGProgramError::InvalidInstruction, state::{Game, JoinGame, MakeMove, Player}, };
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
pub enum RNGProgramInstruction { 
  PlayerCount,
  GameId,
  CreatePlayer{player_address: [u8;32]},
  CreateGame{join_data: JoinGame},
  JoinGame {join_data: JoinGame},
  MakeMove {x:usize, y:usize},
  CheckWinner,
  DistributePrize,
  ClosePda,
}

impl RNGProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
  
      let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
       
      Ok(match tag {
        0 => Self::PlayerCount,
        1 => Self::GameId,
        2 => {
            let create_player_data = Player::try_from_slice(&rest).map_err(|_| InvalidInstruction)?;
            Self::CreatePlayer {
                player_address: create_player_data.player_address,
            }
        },
        3 => Self::CreateGame {
            join_data: JoinGame::try_from_slice(&rest)?,
        },
        4 => Self::JoinGame {
            join_data: JoinGame::try_from_slice(&rest)?,
        },
        5 => {
            let make_move_data = MakeMove::try_from_slice(&rest).map_err(|_| InvalidInstruction)?;
            Self::MakeMove {
                x: make_move_data.x,
                y: make_move_data.y,
            }
        },
        6 => Self::CheckWinner,
        7 => Self::DistributePrize,
        8 => Self::ClosePda,
        _ => return Err(InvalidInstruction.into()),
    })
}
}
  
