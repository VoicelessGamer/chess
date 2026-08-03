# Chess

 A library for processing the game logic of a chess game.

 Contruct an instance of the Game struct by providing a custom GameConfig or by using the GameConfig::default() implementation. To progress the game, simply pass a PieceMove into the game.process_move() function.

 A pgn_notation_util is also available to provide the standard pgn notation for a move by passing it and the current state of the board into the pgn_notation_util.calculate_pgn() function.