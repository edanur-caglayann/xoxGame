use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Player{
    pub game_id:u8,
    pub player_address: [u8;32],
    pub wins: u64, // kazandigi oyun saysii
} 

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct PlayerCount{
    pub player_count: u8, 
} 

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Game{
    pub game_id:u8,
    pub player1: [u8;32], // 1.oyuncunun publickeyi
    pub player2: [u8;32], // 2.oyuncunun publickeyi
    pub deposit_amount: u64, // oyun icin odedigi miktar
    pub game_board: [[u8; 3]; 3], // 3*3 oyun tahtasi 9 byte
    pub turn: u8, // o anki oyuncunun sirasi
    pub prize_pool:u64, // odul havuzu
    pub game_active: u8, //oyun aktif mi
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct GameId{
    pub game_id: u8, 
} 

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct MakeMove{
    pub x: usize,
    pub y: usize,
} 

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct JoinGame{
    pub deposit_amount: u64, 
    pub player_address: [u8;32],

} 


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct ClosePda{
    pub player_address: [u8;32],

} 