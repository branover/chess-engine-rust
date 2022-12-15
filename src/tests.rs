extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::board::{
        Board,
        Coord,
        Square,
    };
    use crate::pieces::{
        Piece,
        PieceColor,
        PieceKind,
    };
    use crate::engine::make_best_move;


    #[bench]
    fn test_all_possible_moves(b: &mut Bencher) {
        b.iter(|| {
            let board = Board::default();
            board.list_all_valid_moves();
        });
        // println!("{:?}", result);
    }

    #[bench]
    fn queen_capture_bench(b: &mut Bencher) {
        b.iter(|| {
            queen_capture()
        });
    }

    #[test]
    fn board_builder() {
        let fen_board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let default_board: Board = Board::default();
        assert!(default_board == fen_board)
    }

    #[test]
    fn board_builder_black_starting() {
        let fen_board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();

        assert!(fen_board.turn == PieceColor::Black);
    }

    #[test]
    fn test_notation() {
        let from = "a3";
        assert!(Coord::from_notation(from).unwrap() == Coord { x: 0, y: 5 });

        let from = "h8";
        assert!(Coord::from_notation(from).unwrap() == Coord { x: 7, y: 0 });
    }

    #[test]
    fn test_notation_invalid() {
        let from = "i3";
        assert!(Coord::from_notation(from).is_err());

        let from = "a9";
        assert!(Coord::from_notation(from).is_err());
    }

    #[test]
    fn invalid_piece_move() {
        let mut board = Board::default();
        let moves = [
            ("e2", "e2")
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn pawn_move() {
        let mut board = Board::default();
        let moves = [
            ("e2", "e4"),
            ("d7", "d5"),
            ("e4", "e5"),
            ("h7", "h6"),
            ("a2", "a4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn pawn_move_invalid() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();

        let moves = [
            ("e4", "e6"),
            ("e4", "e3"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn knight_move() {
        let mut board = Board::default();
        let moves = [
            ("g1", "f3"),
            ("b8", "c6"),
            ("b1", "c3"),
            ("g8", "f6"),
            ("f3", "g5"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn knight_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("f3", "g6"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn bishop_move() {
        let mut board = Board::default();

        // Get pawn out of the way
        board.do_move("d2", "d4").unwrap();
        board.do_move("e7", "e5").unwrap();

        let moves = [
            ("c1", "f4"),
            ("a7", "a6"),
            ("f4", "g5"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn bishop_move_invalid() {
        let mut board = Board::default();

        // Get pawn out of the way
        board.do_move("d2", "d4").unwrap();

        let moves = [
            ("c1", "f5"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn rook_move() {
        let mut board = Board::default();
        // Get pawn out of the way
        board.do_move("a2", "a4").unwrap();
        board.do_move("h7", "h5").unwrap();

        let moves = [
            ("a1", "a3"),
            ("h5", "h4"),
            ("a3", "h3"),
            ("a7", "a6"),
            ("h3", "d3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn rook_move_invalid() {
        let mut board = Board::default();

        let moves = [
            ("a1", "b2"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn queen_move() {
        let mut board = Board::default();
        // Get pawn out of the way
        board.do_move("d2", "d4").unwrap();
        board.do_move("e7", "e5").unwrap();

        let moves = [
            ("d1", "d3"),
            ("a7", "a6"),
            ("d3", "g6"),
            ("a6", "a5"),
            ("g6", "g3"),
            ("a5", "a4"),
            ("g3", "a3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn queen_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("d1", "e3"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());
        }
    }

    #[test]
    fn king_move() {
        let mut board = Board::default();

        // Get pawn out of the way
        board.do_move("e2", "e4").unwrap();

        let moves = [
            ("e7", "e6"),
            ("e1", "e2"),
            ("e8", "e7"),
            ("e2", "e3"),
            ("e7", "d6"),
            ("e3", "e2"),
            ("d6", "c6"),
            ("e2", "f3"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn king_move_invalid() {
        let mut board = Board::default();
        let moves = [
            ("e1", "e4"),
        ];
        for (from, to) in moves.iter() {
            let result = board.do_move(from, to);
            assert!(result.is_err());        
        }
    }

    #[test]
    fn queen_capture() {
        let mut board = Board::default();
        let moves = [
            ("d2", "d4"),
            ("e7", "e5"),
            ("d1", "d3"),
            ("e5", "d4"),
            ("d3", "d4"),
        ];
        for (from, to) in moves.iter() {
            board.do_move(from, to).unwrap();
        }
    }

    #[test]
    fn move_blocked() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("e7", "e5").unwrap();

        // Should be blocked by pawn
        let result = board.do_move("d1", "d4");
        assert!(result.is_err());
    }

    #[test]
    fn move_blocked_knight() {
        let mut board = Board::default();
        // Should not be blocked
        let result = board.do_move("b1", "c3");
        assert!(result.is_ok());

        board.do_move("e7", "e5").unwrap();

        // Should be blocked by queen
        let result = board.do_move("c3", "d1");
        assert!(result.is_err());

        // Should not be blocked
        let result = board.do_move("c3", "b1");
        assert!(result.is_ok());
    }

    #[test]
    fn pawn_cant_capture_in_front() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("e7", "e5").unwrap();

        // Should not be able to capture in front
        let result = board.do_move("e4", "e5");
        assert!(result.is_err());
    }

    #[test]
    fn pawn_can_capture_diagonal() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("d7", "d5").unwrap();

        // Should be able to capture diagonal
        let result = board.do_move("e4", "d5");
        assert!(result.is_ok());
    }

    #[test]
    fn same_color_cant_move_twice() {
        let mut board = Board::default();
        board.do_move("e2", "e4").unwrap();
        board.do_move("e7", "e5").unwrap();

        // Black should not be able to move twice
        let result = board.do_move("e8", "e7");
        assert!(result.is_err());
    }

    #[test]
    fn kingside_castle() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R w KQkq - 0 1").unwrap();
        board.do_move("e1", "g1").unwrap();
    }

    #[test]
    fn queenside_castle() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R b KQkq - 0 1").unwrap();
        board.do_move("e8", "c8").unwrap();
    }

    #[test]
    fn kingside_castle_invalid() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R w Qkq - 0 1").unwrap();
        let result = board.do_move("e1", "g1"); 
        assert!(result.is_err()); 
    }

    #[test]
    fn queenside_castle_invalid() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R b Qk - 0 1").unwrap();
        let result = board.do_move("e8", "c8");   
        assert!(result.is_err()); 
    }

    #[test]
    fn queenside_castle_invalid_turn() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R w Qk - 0 1").unwrap();
        let result = board.do_move("e8", "c8");   
        assert!(result.is_err()); 
    }

    #[test]
    fn queenside_castle_invalid_after_moving_king() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R b KQkq - 0 1").unwrap();
        board.do_move("e8", "d8").unwrap();
        board.do_move("g2", "g4").unwrap();
        board.do_move("d8", "e8").unwrap();
        board.do_move("g4", "g5").unwrap();
        let result = board.do_move("e8", "c8");
        assert!(result.is_err());
    }

    #[test]
    fn queenside_castle_invalid_after_moving_rook() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R b KQkq - 0 1").unwrap();
        board.do_move("a8", "b8").unwrap();
        board.do_move("g2", "g4").unwrap();
        board.do_move("b8", "a8").unwrap();
        board.do_move("g4", "g5").unwrap();
        let result = board.do_move("e8", "c8");
        assert!(result.is_err());
    }

    #[test]
    fn kingside_castle_invalid_after_moving_rook() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R w KQkq - 0 1").unwrap();
        board.do_move("h1", "g1").unwrap();
        board.do_move("g7", "g6").unwrap();
        board.do_move("g1", "h1").unwrap();
        board.do_move("g6", "g5").unwrap();
        let result = board.do_move("e1", "g1"); 
        assert!(result.is_err()); 
    }

    #[test]
    fn kingside_castle_invalid_after_moving_king() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R w KQkq - 0 1").unwrap();
        board.do_move("e1", "e2").unwrap();
        board.do_move("g7", "g6").unwrap();
        board.do_move("e2", "e1").unwrap();
        board.do_move("g6", "g5").unwrap();
        let result = board.do_move("e1", "g1"); 
        assert!(result.is_err()); 
    }
    
    #[test]
    fn kingside_castle_after_moving_queenside_rook() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p4/4P3/P2B1N1P/1PPP1PP1/RNBQK2R w KQkq - 0 1").unwrap();
        board.do_move("a1", "a2").unwrap();
        board.do_move("g7", "g6").unwrap();
        board.do_move("g2", "g3").unwrap();
        board.do_move("g6", "g5").unwrap();
        board.do_move("e1", "g1").unwrap();
    }

    #[test]
    fn en_passant() {
        let mut board = Board::from_fen("r3kb1r/pppqppp1/2n1bn2/3p2Pp/4P3/P2B1N1P/1PPP1P2/RNBQK2R w KQkq h6 0 1").unwrap();
        board.do_move("g5", "h6").unwrap();
        assert_eq!(board.piece_at(Coord{x: 7, y: 2}).unwrap(), Piece {kind: PieceKind::Pawn, color: PieceColor::White});
        assert_eq!(board.board[3][7], Square::Empty);
    }

    #[test]
    fn en_passant_move() {
        let mut board = Board::from_fen("r3kb1r/pppqpppp/2n1bn2/3p2P1/4P3/P2B1N1P/1PPP1P2/RNBQK2R b KQkq - 0 1").unwrap();
        board.do_move("h7", "h5").unwrap();
        board.do_move("g5", "h6").unwrap();
        assert_eq!(board.piece_at(Coord{x: 7, y: 2}).unwrap(), Piece {kind: PieceKind::Pawn, color: PieceColor::White});
        assert_eq!(board.board[3][7], Square::Empty);
    }

    #[test]
    fn en_passant_invalid() {
        let mut board = Board::from_fen("r3kb1r/pppqppp1/2n1bn1p/3p2P1/4P3/P2B1N1P/1PPP1P2/RNBQK2R b KQkq - 0 1").unwrap();
        board.do_move("h6", "h5").unwrap();
        let result = board.do_move("g5", "h6");
        assert!(result.is_err());
    }

    #[test]
    fn in_check() {
        let board = Board::from_fen("rnb1kbnr/pppp1ppp/8/4p3/4P2q/5P2/PPPP2PP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert!(board.in_check.0 == true);
    }

    #[test]
    fn in_check_after_move() {
        let mut board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert!(board.in_check.0 == false);
        board.do_move("d8", "h4").unwrap();
        assert!(board.in_check.0 == true);
    }

    #[test]
    fn block_check() {
        let mut board = Board::from_fen("rnbqkbnr/pppp2pp/5p2/4p3/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert!(board.in_check.0 == false);
        board.do_move("f8", "b4").unwrap();
        assert!(board.in_check.0 == true);
        board.do_move("c2", "c3").unwrap();
        assert!(board.in_check.0 == false);
    }

    #[test]
    fn cant_move_without_blocking_check() {
        let mut board = Board::from_fen("rnbqkbnr/pppp2pp/5p2/4p3/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert!(board.in_check.0 == false);
        board.do_move("f8", "b4").unwrap();
        assert!(board.in_check.0 == true);
        let result = board.do_move("a2", "a3");
        assert!(result.is_err());
    }

    #[test]
    fn cant_castle_out_of_check() {
        let mut board = Board::from_fen("r1bqk1nr/ppp3pp/2np1p2/4p3/1b1PP3/3B1P2/PPP1N1PP/RNBQK2R w KQkq - 0 1").unwrap();
        assert!(board.get_check() == true);
        let result = board.do_move("e1", "g1");
        assert!(result.is_err());
    }   

    #[test]
    fn cant_castle_into_check() {
        let mut board = Board::from_fen("r1bqk1nr/ppp3pp/2np1p2/4p3/3bP3/3B1P2/PPP1N1PP/RNBQK2R w KQkq - 0 1").unwrap();
        assert!(board.get_check() == false);
        let result = board.do_move("e1", "g1");
        assert!(result.is_err());
    }

    #[test]
    fn checkmate() {
        let board = Board::from_fen("r1b1k1nr/ppp3pp/2np1p2/4p3/3bP3/3B1P2/PPPBNqPP/RN1QK2R w kq - 0 1").unwrap();
        assert!(board.get_check() == true);
        assert!(board.in_checkmate.0 == true);
        assert!(board.get_checkmate() == true);
    }

    #[test]
    fn checkmate_after_move() {
        let mut board = Board::from_fen("r1b1k1nr/ppp3pp/2np1p2/4p3/3bP3/3BqP2/PPPBN1PP/RN1QK2R b kq - 0 1").unwrap();
        assert!(board.get_check() == false);
        board.do_move("e3", "f2").unwrap();

        assert!(board.in_checkmate.0 == true);
        assert!(board.get_checkmate() == true);
    }

    #[test]
    fn not_checkmate() {
        let mut board = Board::from_fen("r1b1k1nr/ppp3pp/2np1p2/4p3/3bP3/1Q1B1P2/PPPBNqPP/RN2K2R w kq - 0 1").unwrap();
        assert!(board.get_check() == true);
        assert!(board.in_checkmate.0 == false);
        assert!(board.get_checkmate() == false);
        board.do_move("e1", "d1").unwrap();
        assert!(board.in_check.0 == false);
        assert!(board.in_checkmate.0 == false);

    }

    #[test]
    fn engine() {
        let mut board = Board::default();
        for _ in 0..10 {
            let mv = make_best_move(2, &board).unwrap();
            board.do_move_from_coord(mv).unwrap();
        }
    }

}