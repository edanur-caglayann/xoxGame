import { serialize, deserialize, Schema } from "borsh";

export class Player { 
    game_id:number = 0;
    player_address: Uint8Array = new Uint8Array(32);
    wins: bigint = BigInt(0);
  
    constructor(fields: {game_id: number; player_address:Uint8Array;wins:bigint;} | undefined = undefined) {
      if (fields) {
        this.game_id = fields.game_id;
        this.player_address = fields.player_address;
        this.wins = fields.wins;
      }
    }
  }
  
  export const PlayerSchema = new Map([
    [Player, {
      kind: "struct",
      fields: [
        ["game_id", "u8"],
        ["player_address", ["u8", 32]],
        ["wins", "u64"],
      ]
    }]
  ]);

  
  export class PlayerCount { 
    player_count:  number = 0;
  
    constructor(fields: {player_count:number;} | undefined = undefined) {
      if (fields) {
        this.player_count = fields.player_count;
      }
    }
  }
  
  export const PlayerCountSchema = new Map([
    [PlayerCount, {
      kind: "struct",
      fields: [
        ["player_count", "u8"],
      ]
    }]
  ]);


  export class Game { 
    game_id: number = 0;
    player1: Uint8Array = new Uint8Array(32);
    player2: Uint8Array = new Uint8Array(32);
    deposit_amount: bigint = BigInt(0);
    game_board: number[][] = [
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0]
    ]; // ??
    turn: number = 0;
    prize_pool: bigint = BigInt(0);
    game_active: number = 0;

    constructor(fields: {game_id: number; player1:Uint8Array; player2:Uint8Array; deposit_amount:bigint; game_board: number[][]; turn: number; prize_pool: bigint; game_active: number;} | undefined = undefined) {
      if (fields) {
        this.game_id = fields.game_id;
        this.player1 = fields.player1;
        this.player2 = fields.player2;
        this.game_board = fields.game_board;
        this.turn = fields.turn;
        this.prize_pool = fields.prize_pool;
        this.game_active = fields.game_active;
        this.deposit_amount = fields.deposit_amount;

      }
    }
}

export const GameSchema = new Map([
  [Game, {
    kind: "struct",
    fields: [
      ["game_id", "u8"],
      ["player1", ["u8", 32]],
      ["player2", ["u8", 32]],
      ["deposit_amount", "u64"],
      ["game_board", [["u8", 3], 3]],
      ["turn", "u8"],
      ["prize_pool", "u64"],
      ["game_active", "u8"],
    ]
  }]
]);

export class GameId { 
  game_id: number = 0;


  constructor(fields: {game_id:number;} | undefined = undefined) {
    if (fields) {
      this.game_id = fields.game_id;
    }
  }
}

export const GameIdSchema = new Map([
  [GameId, {
    kind: "struct",
    fields: [
      ["game_id",  "u8"],
    ]
  }]
]);


export class MakeMove { 
  x: number;  
  y: number;  

  constructor(fields: {x: number; y: number;} | undefined = undefined) {
    if (fields) {
      this.x = fields.x;
      this.y = fields.y;
    } else {
      this.x = 0;  
      this.y = 0;  
    }
  }
}

export const MakeMoveSchema = new Map([
  [MakeMove, {
    kind: "struct",
    fields: [
      ["x", "usize"],
      ["y", "usize"],  
    ]
  }]
]);

export class JoinGame { 
  deposit_amount: bigint = BigInt(0);
  player_address: Uint8Array = new Uint8Array(32);
  game_counter: number = 0;

  constructor(fields: {deposit_amount: bigint; player_address: Uint8Array; game_counter: number;} | undefined = undefined) {
    if (fields) {
      this.deposit_amount = fields.deposit_amount;
      this.player_address = fields.player_address;
      this.game_counter = fields.game_counter;
    } 
  }
}

export const JoinGameSchema = new Map([
  [JoinGame, {
    kind: "struct",
    fields: [
      ["deposit_amount", "u64"],
      ["player_address", ["u8", 32]],  
    ]
  }]
]);

export class CreateGame { 
  deposit_amount: bigint = BigInt(0);
  player_address: Uint8Array = new Uint8Array(32);

  constructor(fields: {deposit_amount: bigint; player_address: Uint8Array;} | undefined = undefined) {
    if (fields) {
      this.deposit_amount = fields.deposit_amount;
      this.player_address = fields.player_address;
    } 
  }
}

export const CreateGameSchema = new Map([
  [CreateGame, {
    kind: "struct",
    fields: [
      ["deposit_amount", "u64"],
      ["player_address", ["u8", 32]],  
    ]
  }]
]);

export class ClosePda { 
  player_address: Uint8Array = new Uint8Array(32);

  constructor(fields: {player_address: Uint8Array;} | undefined = undefined) {
    if (fields) {
      this.player_address = fields.player_address;
    } 
  }
}

export const ClosePdaSchema = new Map([
  [ClosePda, {
    kind: "struct",
    fields: [
      ["player_address", ["u8", 32]],  
    ]
  }]
]);