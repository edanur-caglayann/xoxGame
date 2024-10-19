import {
    Connection,
    Keypair,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram,
    TransactionInstruction,
    LAMPORTS_PER_SOL,
    Transaction,
    sendAndConfirmTransaction,
  
  } from "@solana/web3.js";
  import {deserialize, deserializeUnchecked, serialize } from "borsh";
  import { Player,PlayerSchema,Game,GameSchema, PlayerCount, PlayerCountSchema, GameId, GameIdSchema, JoinGame, JoinGameSchema} from "./models";
  
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  const privatekey = [209,202,75,77,51,59,102,81,8,45,50,58,209,54,134,238,29,107,221,66,98,156,30,20,186,236,255,189,136,8,36,169,49,191,167,29,47,172,73,19,16,188,51,135,9,154,137,226,181,182,26,127,251,38,99,119,117,149,77,134,182,216,216,215]
  const payer = Keypair.fromSecretKey(Uint8Array.from(privatekey));
 
  const program_id =  new PublicKey("GSWdTfvgwo62NG61MxfSHfWbQ2yZzSU9Lu7Zjy3Ky1Qz");
  const player_count = new PublicKey("gTVAtHKsmM1wSukxFQ8iCfCm8kLFtmoS2Mx2Gr3XwaK");
  const game_id = new PublicKey("3q67A3TpCEiBtnmd3LHvhCEpzoHK25UpkC4u9Tro1mx4");

  const player_count_acc = async() => {
    const player_count = new PlayerCount();
    player_count.player_count = 0;

    const encoded = serialize(PlayerCountSchema, player_count);
    const concat = Uint8Array.of(0, ...encoded);

    const playerCountPda = PublicKey.findProgramAddressSync([Buffer.from("PlayerrCount")],program_id);

    const instruction = new TransactionInstruction({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: playerCountPda[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from(concat),
      programId: program_id
    })
    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("Player Count PDA => " + playerCountPda[0].toString())
  }

  const game_id_acc = async() => {
    const game_id = new GameId();
    game_id.game_id = 0;
    
    const encoded = serialize(GameIdSchema, game_id);
    const concat = Uint8Array.of(1, ...encoded);

    const gameIdPda = PublicKey.findProgramAddressSync([Buffer.from("Gameidd")],program_id);

    const instruction = new TransactionInstruction ({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: gameIdPda[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from(concat),
      programId: program_id
    })
    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    connection.sendTransaction(tx);
    console.log("Game Id PDA => " + gameIdPda[0].toString())
  
  }

  const create_player_acc = async(player_address:Uint8Array) => {

    const game_id_data = await connection.getAccountInfo(game_id);
    if (!game_id_data) {
      throw new Error("Game ID account not found");
    }
    const game_id_data_deserialize = deserialize(GameIdSchema, GameId, game_id_data!.data);

    const player_count_data = await connection.getAccountInfo(player_count);
    if (!player_count_data) {
      throw new Error("Player Count account not found");
    }
    const player_count_data_deserialize = deserialize(PlayerCountSchema, PlayerCount, player_count_data!.data);

    const create_player = new Player();
    create_player.game_id = game_id_data_deserialize.game_id;
    create_player.player_address = player_address;
    create_player.wins = BigInt(0);

    const encoded = serialize(PlayerSchema, create_player);
    const concat = Uint8Array.of(2, ...encoded);

    player_count_data_deserialize.player_count += 1;

    const playerPda = PublicKey.findProgramAddressSync(
        [Buffer.from("player"), Buffer.from(player_address)],
        program_id
    );

    const instruction = new TransactionInstruction ({
      keys: [
        {pubkey: payer.publicKey, isSigner: true, isWritable: true},
        {pubkey: player_count, isSigner: false, isWritable: true},
        {pubkey: game_id, isSigner: false, isWritable: true},
        {pubkey: playerPda[0], isSigner: false, isWritable: true},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from(concat),
      programId: program_id
    })

    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
    }).compileToV0Message();
  
    
    const tx = new VersionedTransaction(message);
     tx.sign([payer]);
  
    const transactionn = await connection.sendTransaction(tx);
    
    console.log("Player PDA => " + playerPda[0].toString())
    console.log(player_count_data_deserialize.player_count.toString())
    const data = await connection.getAccountInfo(playerPda[0])
    console.log(data!.data.buffer)


  }

  const player_read = async(player_address:Uint8Array) => {
    const player_count_data = await connection.getAccountInfo(player_count);
    const player_count_data_deserialize = deserialize(PlayerCountSchema, PlayerCount, player_count_data!.data);

    const playerPda = PublicKey.findProgramAddressSync(
        [Buffer.from("player"), Buffer.from(player_address)],
        program_id
    );
    console.log(player_count_data_deserialize.player_count.toString())
    console.log("Player PDA => " + playerPda[0].toString())

    const player_data = await connection.getAccountInfo(playerPda[0]);

    if (player_data === null) {
        console.error("Player account does not exist.");
        return; 
    }

    const player_data_deserialize = deserialize(PlayerSchema, Player, player_data!.data);
    console.log("Game Id -> " + player_data_deserialize.game_id);
    console.log("Player Address -> " + player_data_deserialize.player_address);
    console.log("Wins -> " + player_data_deserialize.wins);
  }

  const create_game_acc = async(data:JoinGame) => {
  const game_id_data = await connection.getAccountInfo(game_id);
  const game_id_data_deserialize = deserialize(GameIdSchema, GameId, game_id_data!.data);

  const player_count_data = await connection.getAccountInfo(player_count);
  const player_count_data_deserialize = deserialize(PlayerCountSchema, PlayerCount, player_count_data!.data);

  const playerPda = PublicKey.findProgramAddressSync(
      [Buffer.from("player"), Buffer.from(data.player_address)],
      program_id
  );

  const player_data = await connection.getAccountInfo(playerPda[0]);
  const player_data_deserialize = deserialize(PlayerSchema, Player, player_data!.data);
  

  const encoded = serialize(JoinGameSchema, data);
  const concat = Uint8Array.of(3, ...encoded);

  game_id_data_deserialize.game_id += 1;

  const gamePda = PublicKey.findProgramAddressSync(
      [Buffer.from("game"), Buffer.from(player_data_deserialize.player_address)],
      program_id
  );

  const instruction = new TransactionInstruction ({
      keys: [
          {pubkey: payer.publicKey, isSigner: true, isWritable: true},
          {pubkey: playerPda[0], isSigner: false, isWritable: true},
          {pubkey: game_id, isSigner: false, isWritable: true},
          {pubkey: gamePda[0], isSigner: false, isWritable: true}, 
          {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      data: Buffer.from(concat),
      programId: program_id
  });

  const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  }).compileToV0Message();

  
  const tx = new VersionedTransaction(message);
  tx.sign([payer]);

  const transactionn = await connection.sendTransaction(tx);
    
  console.log(" -> " +transactionn)
  console.log("Game PDA => " + gamePda[0].toString());
  console.log(game_id_data_deserialize.game_id.toString())
  }

  const game_read = async(player_address:Uint8Array) => {
    const player_count_data = await connection.getAccountInfo(player_count);
    if (!player_count_data) {
      console.error("Player count account does not exist.");
      return;
  }
    const player_count_data_deserialize = deserialize(PlayerCountSchema, PlayerCount, player_count_data!.data);

    const playerPda = PublicKey.findProgramAddressSync(
      [Buffer.from("player"), Buffer.from(player_address)],
      program_id
  );

  const player_data = await connection.getAccountInfo(playerPda[0]);
  if (!player_data) {
    console.error("Player account does not exist.");
    return;
}
  const player_data_deserialize = deserialize(PlayerSchema, Player, player_data!.data);

    const gamePda = PublicKey.findProgramAddressSync(
      [Buffer.from("game"), Buffer.from(player_data_deserialize.player_address)],
      program_id
  );

  
  const game_data = await connection.getAccountInfo(gamePda[0]);

  if (!game_data) {
    console.error("Game account does not exist.");
    return; 
}

  const game_data_deserialize = deserialize(GameSchema, Game, game_data!.data);

    console.log("Game Id -> " + game_data_deserialize.game_id);
    console.log("Player 1 -> " + game_data_deserialize.player1);
    console.log("Player 2 -> " + game_data_deserialize.player2);
    console.log("Game Board -> " + game_data_deserialize.game_board);
    console.log("Turn -> " + game_data_deserialize.turn);
    console.log("Prize Pool -> " + game_data_deserialize.prize_pool);
    console.log("Game Active -> " + game_data_deserialize.game_active);
  }

  const get_not_player2 = async() => {

  }
  
  const join_game_acc = async (join_data:JoinGame) => {
    
    const encoded = serialize(JoinGameSchema, join_data);
    const concat = Uint8Array.of(4, ...encoded);  
  
    const game_id_data = await connection.getAccountInfo(game_id);
    if (!game_id_data || !game_id_data.data) {
      throw new Error("Game ID data is null or empty");
    }

    const game_id_data_deserialize = deserialize(GameIdSchema, GameId, game_id_data!.data);
    
    const player_count_data = await connection.getAccountInfo(player_count);
    if (!player_count_data || !player_count_data.data) {
      throw new Error("Player count data is null or empty");
    }
    const player_count_data_deserialize = deserialize(PlayerCountSchema, PlayerCount, player_count_data.data);
  
    const playerPda = PublicKey.findProgramAddressSync(
      [Buffer.from("player"), Buffer.from(join_data.player_address)],
      program_id
    );
  
    const player_data = await connection.getAccountInfo(playerPda[0]);
    if (!player_data || !player_data.data) {
      throw new Error("Player data is null or empty");
    }

    const player_data_deserialize = deserialize(PlayerSchema, Player, player_data.data);
  
    const gamePda = PublicKey.findProgramAddressSync(
      [Buffer.from("game"), Buffer.from(player_data_deserialize.player_address)],
      program_id
    );
  
    const game_data = await connection.getAccountInfo(gamePda[0]);
    if (!game_data || !game_data.data) {
      throw new Error("Game data is null or empty");
    }
    const game_data_deserialize = deserialize(GameSchema, Game, game_data.data);
  
    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: playerPda[0], isSigner: false, isWritable: true },
        { pubkey: gamePda[0], isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, 
      ],
      data: Buffer.from(concat),
      programId: program_id,
    });
  
    const message = new TransactionMessage({
      instructions: [instruction],
      payerKey: payer.publicKey,
      recentBlockhash: (await connection.getLatestBlockhash()).blockhash,
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(message);
    tx.sign([payer]);

    await connection.sendTransaction(tx);
    
    console.log("Joined in the game");
    console.log("Address of the player participating in the game => " + player_data_deserialize.player_address);
  };
  
  // const make_move_acc = async(x: number, y: number) => {
  //   const game_data = await connection.getAccountInfo(game);
  //   const game_data_deserialize = deserialize(GameSchema, Game, game_data!.data);

  //   // const player_data = await connection.getAccountInfo(player);
  //   // const player_data_deserialize = deserialize(PlayerSchema, Player, player_data!.data);
      
  //       const instruction = new TransactionInstruction({
  //         keys: [
  //             { pubkey: payer.publicKey, isSigner: true, isWritable: true },
  //             // { pubkey: player, isSigner: false, isWritable: true },
  //             { pubkey: game, isSigner: false, isWritable: true },
  //         ],
  //         data: Buffer.concat([
  //           Buffer.from([5]), // Örneğin, hamle kodu
  //           Buffer.from([x]), // Hamle satırını ekleyin
  //           Buffer.from([y]), // Hamle sütununu ekleyin
  //       ]),
  //         programId: program_id
  //     });

  //     // İşlemi oluştur
  //     const message = new TransactionMessage({
  //         instructions: [instruction],
  //         payerKey: payer.publicKey,
  //         recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  //     }).compileToV0Message();

  //     const tx = new VersionedTransaction(message);
  //     tx.sign([payer]);

  
  //   connection.sendTransaction(tx);
  //   console.log("Make move successful");
  // }

  // const check_winner_acc = async() => {
  //   const game_data = await connection.getAccountInfo(game);
  //   const game_data_deserialize = deserialize(GameSchema, Game, game_data!.data);

  //   const instruction = new TransactionInstruction({
  //     keys: [
  //         { pubkey: game, isSigner: false, isWritable: true },
  //     ],
  //     data: Buffer.concat([
  //       Buffer.from([5]), 
  //   ]),
  //     programId: program_id
  //   });

  //   const message = new TransactionMessage({
  //     instructions: [instruction],
  //     payerKey: payer.publicKey,
  //     recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  //   }).compileToV0Message();

  //   const tx = new VersionedTransaction(message);
  //   tx.sign([payer]);


  //   connection.sendTransaction(tx);
  //   console.log("Check Winner");

  // }
  
  // const distribute_prize_acc = async() => {
  //   const game_data = await connection.getAccountInfo(game);
  //   const game_data_deserialize = deserialize(GameSchema, Game, game_data!.data);

  //   // const player_data = await connection.getAccountInfo(player);
  //   // const player_data_deserialize = deserialize(PlayerSchema, Player, player_data!.data);
      
  //       const instruction = new TransactionInstruction({
  //         keys: [
  //             { pubkey: payer.publicKey, isSigner: true, isWritable: true },
  //             { pubkey: game, isSigner: false, isWritable: true },
  //             // { pubkey: player, isSigner: false, isWritable: true },
  //         ],
  //         data: Buffer.concat([
  //           Buffer.from([5]), 
  //       ]),
  //         programId: program_id
  //     });

  //     const message = new TransactionMessage({
  //         instructions: [instruction],
  //         payerKey: payer.publicKey,
  //         recentBlockhash: (await connection.getLatestBlockhash()).blockhash
  //     }).compileToV0Message();

  //     const tx = new VersionedTransaction(message);
  //     tx.sign([payer]);

  
  //   connection.sendTransaction(tx);
  //   console.log("Distribute prize");
  // }
  
  
  const close_pda = async() => {
    const instruction = new TransactionInstruction ({
          keys: [
            {pubkey: payer.publicKey, isSigner: true, isWritable: true},
            {pubkey: game_id, isSigner: false, isWritable: true},
            // {pubkey: player_count, isSigner: false, isWritable: true},



          ],
          data: Buffer.from([8]),
          programId: program_id
        })
    
        const message = new TransactionMessage({
          instructions: [instruction],
          payerKey: payer.publicKey,
          recentBlockhash: (await connection.getLatestBlockhash()).blockhash
        }).compileToV0Message();
      
        
        const tx = new VersionedTransaction(message);
         tx.sign([payer]);
      
        connection.sendTransaction(tx);
        console.log(" Close Pda " )
  }

  // player_count_acc()
  // game_id_acc()
    
    const playerAddress1 = new Uint8Array([189, 153, 237, 247, 123, 129, 44, 245, 211, 215, 170, 9, 202, 36, 55, 170, 231, 169, 173, 114, 54, 172, 228, 31, 53, 103, 118, 246, 172, 173, 139, 41, 13, 104, 114, 227, 231, 145, 151, 104, 78, 118, 88, 6, 159, 158, 103, 77, 212, 26, 216, 134, 220, 208, 66, 180, 15, 103, 169, 210, 55, 84, 64, 73]);
    const player1Wallet = Keypair.fromSecretKey(Uint8Array.from(playerAddress1));

    const playerAddress2 = new Uint8Array([175, 180, 249, 14, 75, 128, 186, 109, 19, 245, 202, 130, 176, 251, 131, 153, 209, 180, 183, 65, 205, 217, 89, 206, 2, 131, 163, 102, 218, 87, 157, 104, 221, 116, 147, 172, 19, 29, 71, 54, 147, 104, 214, 181, 142, 233, 210, 75, 198, 216, 86, 190, 13, 169, 229, 202, 203, 24, 254, 227, 3, 125, 19, 123]);
    const player2Wallet = Keypair.fromSecretKey(Uint8Array.from(playerAddress2));
   
    // console.log("=> " + player2Wallet.publicKey)
   
    // create_player_acc(player2Wallet.publicKey.toBytes());

    // player_read(player2Wallet.publicKey.toBytes())

    // game_read(player1Wallet.publicKey.toBytes())

  const join_game = new JoinGame();
    join_game.deposit_amount = BigInt(0.0001 * 1_000_000_000);
    join_game.player_address = player1Wallet.publicKey.toBytes();

    create_game_acc(join_game)

  const joinPlayer2 = new JoinGame();
  joinPlayer2.player_address = player2Wallet.publicKey.toBytes();
  joinPlayer2.deposit_amount =  BigInt(0.0001 * 1_000_000_000);

  
  // join_game_acc(joinPlayer2)
  // close_pda()

