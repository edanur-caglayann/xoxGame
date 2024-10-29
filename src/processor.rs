


use core::borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use num_bigint::BigInt;
use solana_program::{ 
    account_info::{next_account_info, AccountInfo}, address_lookup_table::instruction, clock, config, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, lamports, msg, program::{invoke, invoke_signed}, program_error::ProgramError, pubkey::{self, Pubkey}, rent::Rent, system_instruction::{self, transfer}, system_program, sysvar::Sysvar
    };
    use crate::{instruction::RNGProgramInstruction, state::{CreateGame, Game, GameId, JoinGame, MakeMove, Player, PlayerCount, WinningUser}};
    use crate::error::RNGProgramError::{InvalidInstruction};
    pub struct Processor;
    impl Processor {
    pub fn process(
      _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
      ) -> ProgramResult {
        let instruction: RNGProgramInstruction = RNGProgramInstruction::unpack(instruction_data)?;
    
    
        match instruction { 
          RNGProgramInstruction::PlayerCount => {
              Self::player_count(accounts, _program_id)
          },
          RNGProgramInstruction::GameId => {
              Self::game_id(accounts, _program_id)
          },
          RNGProgramInstruction::CreatePlayer {player_address}=> {
              Self::create_player(accounts, _program_id, player_address)
          },
          RNGProgramInstruction::CreateGame {data}=> {
              Self::create_game(accounts, _program_id, data)
          },
          RNGProgramInstruction::JoinGame {join_data } => {
              Self::join_game(accounts, _program_id, join_data)
          },
          RNGProgramInstruction::MakeMove { move_data} => {
              Self::make_move(accounts, _program_id, move_data)
          },
          RNGProgramInstruction::CheckWinner {gameCounter} => {
              Self::check_winner(accounts, _program_id, gameCounter)
          },
          RNGProgramInstruction::DistributePrize {winner_data} => {
              Self::distribute_prize(accounts, _program_id, winner_data)
          },
          RNGProgramInstruction::ClosePda => { 
            Self::close_pda(accounts, _program_id )
          },
      }
      
      }

     
     pub fn player_count (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
     ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let player_count = next_account_info(account_info_iter)?;

      let rent = Rent:: default();
      let player_count_rent = rent.minimum_balance(1);

      let (count_pda, bump) = Pubkey::find_program_address(&[b"PlayerrCount"], program_id);

      invoke_signed(&system_instruction::create_account(payer.key, &count_pda, player_count_rent, 1, program_id),
         &[player_count.clone(), payer.clone()],
         &[
            &[b"PlayerrCount", &[bump]]
          ]
        )?;
      Ok(())
     }
     
     pub fn game_id (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
  ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let game_id = next_account_info(account_info_iter)?; 
  
      let rent = Rent::default();
      let game_count_rent = rent.minimum_balance(1); 
  
      let (game_count_pda, bump) = Pubkey::find_program_address(&[b"Gameidd"], program_id);
  
      invoke_signed(
          &system_instruction::create_account(
              payer.key, 
              &game_count_pda, 
              game_count_rent, 
              1, 
              program_id,
          ),
          &[game_id.clone(), payer.clone()],
          &[
              &[b"Gameidd", &[bump]]
          ]
      )?;

    Ok(())
}

     pub fn create_player (
      accounts: &[AccountInfo],
      program_id: &Pubkey,
      player_address: [u8;32],
     ) -> ProgramResult {
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let player_count = next_account_info(account_info_iter)?;
      let game_id = next_account_info(account_info_iter)?;
      let player = next_account_info(account_info_iter)?;
  
      if !payer.is_signer {
          msg!("payer is not a signer");
          return Err(ProgramError::MissingRequiredSignature);
      }
  
      let mut player_count_read = PlayerCount::try_from_slice(&player_count.data.borrow())?;
      let game_id_account_read = GameId::try_from_slice(&game_id.data.borrow())?;
      
    
      player_count_read.player_count = player_count_read.player_count.checked_add(1).ok_or(ProgramError::InvalidArgument)?;
      
      let (player_pda, bump) = Pubkey::find_program_address(
          &[b"player", &player_address],
          program_id,
      );
      

      if player_pda != *player.key {
          msg!("Provided player account does not match derived PDA.");
          return Err(ProgramError::InvalidArgument);
      }
  
  
      let rent = Rent::default();
      let player_rent = rent.minimum_balance(41);
      msg!("3");
      msg!("{}", payer.key.to_string());
      
      // hesap bossa yeni hesap olusturcaz
      if player.data_is_empty() {
        invoke_signed(
          &system_instruction::create_account(
              payer.key,
              &player_pda,
              player_rent,
              41,
              program_id,
          ),
          &[player.clone(), payer.clone()],
          &[&[b"player",&player_address, &[bump]]],
      )?;

      msg!("New player account created.");
      }
      // pda zaten mevcutsa
      else{
        msg!("Player account already exists, skipping creation.");
      }
           
      let player_info = Player {
          game_id: game_id_account_read.game_id,
          player_address:player_address,
          wins: 0,
      };

      msg!("4");
    
      game_id_account_read.serialize(&mut &mut game_id.try_borrow_mut_data()?[..])?;
      player_count_read.serialize(&mut &mut player_count.try_borrow_mut_data()?[..])?;
      player_info.serialize(&mut &mut player.try_borrow_mut_data()?[..])?;
  
      Ok(())
  }
  
      pub fn create_game (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        data: CreateGame,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;
        let game_id_counter = next_account_info(account_info_iter)?;
        let game = next_account_info(account_info_iter)?;
    
         if !payer.is_signer {
          msg!("Payer is not a signer");
          return Err(ProgramError::MissingRequiredSignature);
        }
      
      let mut game_id_account_read = GameId::try_from_slice(&game_id_counter.data.borrow())?;
      let player_read = Player::try_from_slice(&player.data.borrow())?;

      game_id_account_read.game_id = game_id_account_read.game_id.checked_add(1).ok_or(ProgramError::InvalidArgument)?;
      
      msg!("gameId -> {} " , game_id_account_read.game_id);
      
      let (game_pda, bump) = Pubkey::find_program_address(
          &[b"game", &game_id_account_read.game_id.to_be_bytes()],
          program_id,
      );

      msg!("gameId -> {} " , game_pda);

         if game_pda != *game.key {
          msg!("Provided game account does not match derived PDA.");
          return Err(ProgramError::InvalidArgument);
      }

      // ilk oyuncunun adresi
      msg!("Player 1 Address: {}", Pubkey::new_from_array(data.player_address).to_string());
    
      if data.player_address != player_read.player_address{
        return Err(ProgramError::InvalidArgument);
      } 

      
    let rent = Rent::default();
    let game_rent = rent.minimum_balance(92);
    let totalLamports = game_rent.checked_add(data.deposit_amount).ok_or(ProgramError::InvalidArgument)?;
    
      // let playeraddresfrombytes = Pubkey::new_from_array(player_read.player_address);

      if game.data_is_empty() {
        invoke_signed(
          &system_instruction::create_account(
              payer.key,
              &game_pda,
              totalLamports,
              92,
              program_id,
          ),
          &[game.clone(), payer.clone()],
          &[&[b"game", &game_id_account_read.game_id.to_le_bytes(), &[bump]]], 
      )?;
      msg!("New game account created.");
      }

      else{
        msg!("Game account already exists, skipping creation.");

      }
   

    let mut game_info = Game {
        game_id: game_id_account_read.game_id,
        player1: player_read.player_address,
        player2: [0; 32], // 2. oyuncu henüz yok
        deposit_amount: data.deposit_amount,
        game_board: [[0; 3]; 3], // 3*3'lük boş tahta
        turn: 0,
        prize_pool: data.deposit_amount, // odul havuznuna baslangicta 1. oyuncunun yatirdigi miktar ile baslar
        game_active: 0,
    };
    // data.deposit_amount.checked_mul(2).ok_or(ProgramError::InvalidArgument)?, 
    msg!("6");

    
    // game_info.prize_pool = game_info.prize_pool.checked_add(data.deposit_amount).ok_or(ProgramError::InvalidArgument)?; 

    game_id_account_read.serialize(&mut &mut game_id_counter.try_borrow_mut_data()?[..])?;
    player_read.serialize(&mut &mut player.try_borrow_mut_data()?[..])?;
    game_info.serialize(&mut &mut game.try_borrow_mut_data()?[..])?;

    Ok(())
      }

      pub fn join_game (
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        join_data: JoinGame,
       ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;
        let game = next_account_info(account_info_iter)?;

        if !payer.is_signer {
         msg!("Payer is not a signer");
         return Err(ProgramError::MissingRequiredSignature);
        }

        let player_read = Player::try_from_slice(&player.data.borrow())?;

        let (game_pda, _bump) = Pubkey::find_program_address(
            &[b"game", &join_data.game_counter.to_be_bytes()],
            program_id,
        );

        // pda ile game hesabi uyusuyor mu
        if game_pda != *game.key {
          msg!("Provided game account does not match derived PDA.");
        return Err(ProgramError::InvalidArgument);
        }

        let mut game_info = Game::try_from_slice(&game.data.borrow())?;
        
         if game_info.player2 != [0;32] {
          msg!("This game already has two players.");
           return Err(ProgramError::InvalidArgument);
         }
         
         if join_data.deposit_amount <= 0 {
          msg!("Invalid deposit amount.");
          return Err(ProgramError::InvalidArgument);
         }

         game_info.player2 = player_read.player_address;
         game_info.prize_pool = game_info.prize_pool.checked_add(join_data.deposit_amount).ok_or(ProgramError::InvalidArgument)?;
        
         let transfer = system_instruction::transfer(
          payer.key, 
          game.key,
          join_data.deposit_amount);
      
         invoke(
          &transfer,
           &[payer.clone(), game.clone()])?;
         game_info.game_active = 1; // 2. oyuncu katildiktan sonra oyun aktif olur

        
        game_info.serialize(&mut &mut game.try_borrow_mut_data()?[..])?;
        player_read.serialize(&mut &mut player.try_borrow_mut_data()?[..])?;
        
        msg!("Player 2 has joined the game. Game is now active!");

      Ok(())
       }

      pub fn make_move(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        move_data: MakeMove,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;
        let game = next_account_info(account_info_iter)?;

        if !payer.is_signer {
          msg!("payer is not a signer");
          // return Err(AuthorityError.into());
        }

        let player_read = Player::try_from_slice(&player.data.borrow())?;
        let mut game_read = Game::try_from_slice(&game.data.borrow())?;

        if game.owner != program_id {
          msg!("This game does not belong to the current program");
          return Err(ProgramError::IncorrectProgramId);
          
        } 

        // if game_read.game_active == 0 {
        //   msg!("Game is not active");
        //   return Err(ProgramError::InvalidAccountData);
        // }

        // hamle sirasi dogru ksiide mi
        let player_index = if game_read.player1 == player_read.player_address {
          0 }
         
        else if game_read.player2 == player_read.player_address {
          1 } 
        
        else {
          msg!("Player is not part of this game");
          return Err(ProgramError::InvalidAccountData);
        };

        if game_read.turn != player_index as u8 {
          msg!("It's not your turn");
          return Err(ProgramError::InvalidInstructionData);
       }

        // tahtadaki alan bos mu
       if game_read.game_board[move_data.x as usize][move_data.y as usize] != 0 {
        msg!("This spot is already taken");
        return Err(ProgramError::InvalidInstructionData);
        }

        // hamle yapildi
        game_read.game_board[move_data.x as usize][move_data.y as usize] = move_data.symbol;
      
       // hamle yapma sirasini degistir
        game_read.turn = 1 -  game_read.turn; 

        game_read.serialize(&mut &mut game.try_borrow_mut_data()?[..])?;
        player_read.serialize(&mut &mut player.try_borrow_mut_data()?[..])?;


        Ok(())
      }

      pub fn check_winner(
      accounts: &[AccountInfo],
      program_id: &Pubkey,
      gameCounter: u8,
     ) -> ProgramResult {
     let account_info_iter = &mut accounts.iter();
     let game = next_account_info(account_info_iter)?;

      let mut game_read = Game::try_from_slice(&game.data.borrow())?;

      let mut winner:u8 = 0;

        // Oyun durumu kontrolü
     if game_read.game_active == 0 {
      msg!("The game is already over.");
      return Err(ProgramError::InvalidArgument);
     }

     for i in 0..3 {
      // Satr kontrolü
      if game_read.game_board[i][0] != 0 &&
         game_read.game_board[i][0] == game_read.game_board[i][1] &&
         game_read.game_board[i][1] == game_read.game_board[i][2] {
          game_read.game_active = 0;
          winner = game_read.turn;
          msg!("Winner is Player {}", winner);
          break; // Kazanan belirlendikten sonra döngüyü kır
      }

      // Sütun kontrolü
      if game_read.game_board[0][i] != 0 &&
         game_read.game_board[0][i] == game_read.game_board[1][i] &&
         game_read.game_board[1][i] == game_read.game_board[2][i] {
          game_read.game_active = 0; 
          winner = game_read.turn;
          msg!("Winner is Player {}", winner);
          break; // Kazanan belirlendikten sonra döngüyü kır
      }
  }

  // Çapraz kontrol
  // Sol üst sağ alt çapraz
  if game_read.game_board[0][0] != 0 &&
     game_read.game_board[0][0] == game_read.game_board[1][1] &&
     game_read.game_board[1][1] == game_read.game_board[2][2] {
      game_read.game_active = 0; 
     winner = game_read.turn;
      msg!("Winner is Player {}", winner);
  }

  // Sağ üst sol alt çapraz
  if game_read.game_board[0][2] != 0 &&
     game_read.game_board[0][2] == game_read.game_board[1][1] &&
     game_read.game_board[1][1] == game_read.game_board[2][0] {
      game_read.game_active = 0;
      winner = game_read.turn;
      msg!("Winner is Player {}", winner);
  }

   if game_read.game_active == 0 && winner == 0 {
    msg!("1st player to win");
    msg!("winning player's address -> {:?} " , game_read.player1);
   }

   if game_read.game_active == 0 && winner == 1 {
    msg!("2st player to win");
    msg!("winning player's address ->{:?} " , game_read.player2);
   }
    
   if game_read.game_active == 1{
    msg!("It's the other player's turn to move")
   }
   
    game_read.serialize(&mut &mut game.try_borrow_mut_data()?[..])?;
    Ok(())
}

      pub fn distribute_prize(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        winner_data: WinningUser,
      ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let game = next_account_info(account_info_iter)?;
        let player = next_account_info(account_info_iter)?;

          // otorite imzalayici mi
          if !payer.is_signer {
            msg!("Authority is not a signer");
            // return Err(AuthorityError.into());
             }
 
           if game.owner!= program_id {
            msg!("not a program for authority");
            // return Err(OwnershipError.into());
            }

           if player.owner!= program_id {
            msg!("not a program for authority");
            // return Err(OwnershipError.into());
            }

            let (game_pda, _bump) = Pubkey::find_program_address(
              &[b"game", &winner_data.game_counter.to_be_bytes()],
              program_id,
          );
  
          // pda ile game hesabi uyusuyor mu
          if game_pda != *game.key {
            msg!("Provided game account does not match derived PDA.");
          return Err(ProgramError::InvalidArgument);
          }

          let (player_pda, bump) = Pubkey::find_program_address(
            &[b"player", &winner_data.player_address],
            program_id,
          );
        
  
         if player_pda != *player.key {
            msg!("Provided player account does not match derived PDA.");
            return Err(ProgramError::InvalidArgument);
         }
            // winneri kontrol et

        let game_read = Game::try_from_slice(&game.data.borrow())?;
        let player_read = Player::try_from_slice(&player.data.borrow())?;

        if game_read.game_active == 0 {
          msg!("The game is not active");
            return Err(ProgramError::InvalidAccountData);
        }
    
        if game_read.prize_pool <= 0 {
          msg!("No prize to distribute");
          return Err(ProgramError::InsufficientFunds);
        }   

      // kazanana odulu yolla
       if game_read.prize_pool > 0 {

      **game.try_borrow_mut_lamports()? -= game_read.prize_pool;
      **player.try_borrow_mut_lamports()? += game_read.prize_pool;

      game_read.serialize(&mut &mut game.try_borrow_mut_data()?[..])?;
      player_read.serialize(&mut &mut player.try_borrow_mut_data()?[..])?;
      } 
        Ok(())
    }
    
    pub fn close_pda(
      accounts: &[AccountInfo],
      program_id: &Pubkey,
    )-> ProgramResult{
      let account_info_iter = &mut accounts.iter();
      let payer = next_account_info(account_info_iter)?;
      let game_id = next_account_info(account_info_iter)?;
      // let player_count = next_account_info(account_info_iter)?;
      // let game = next_account_info(account_info_iter)?;

      if game_id.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
     }

      //  if player_count.owner != program_id {
      // return Err(ProgramError::IncorrectProgramId);
      // }
      
      // if game.owner != program_id {
      //   return Err(ProgramError::IncorrectProgramId);
      //   }

      //pda'daki lamports mijtarini aliriz
      let lamports  = **game_id.try_borrow_lamports()?; 

      // game id 'nin rentini kim olusturduysa ona geri gondererek silebilriiz
      **game_id.try_borrow_mut_lamports()? -= lamports;
      **payer.try_borrow_mut_lamports()? += lamports ;

      Ok(())
    }
      
      
       }

       


              //plyaer account olusturduktan sonra cuzdan adresimi degistirmem lazim
     // acc programa ait oldugunu nasil anliyoruz
     //programa ait degilse imza atamsi gerekiyro 