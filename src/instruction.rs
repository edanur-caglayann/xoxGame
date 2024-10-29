use crate::{error::RNGProgramError::InvalidInstruction, state::{CreateGame, Game, JoinGame, MakeMove, Player, WinningUser}, };
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
pub enum RNGProgramInstruction { 
  PlayerCount,
  GameId,
  CreatePlayer{player_address: [u8;32]},
  CreateGame{data: CreateGame},
  JoinGame {join_data: JoinGame},
  MakeMove {move_data: MakeMove},
  CheckWinner{gameCounter: u8},
  DistributePrize{winner_data:WinningUser},
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
            data: CreateGame::try_from_slice(&rest)?,
        },
        4 => Self::JoinGame {
            join_data:JoinGame::try_from_slice(&rest)?,
        },
        5 => Self::MakeMove {
            move_data:MakeMove::try_from_slice(&rest)?,
        },
        6 => {
            let gameCounter = u8::try_from_slice(&rest).map_err(|_| InvalidInstruction)?;
            Self::CheckWinner {
                gameCounter: gameCounter,
            }
        },
        7 => Self::DistributePrize {
            winner_data: WinningUser::try_from_slice(&rest)?,
        },
        8 => Self::ClosePda,
        _ => return Err(InvalidInstruction.into()),
    })
}
}
  
