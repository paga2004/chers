use crate::utils::INF;
use crate::BitMove;
use crate::Position;

impl Position {
    fn negamax(&mut self, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        if depth == 0 {
            return self.quiescence_search(alpha, beta);
        }

        let mut moves = self.generate_pseudo_legal_moves(false);
        moves.sort();

        let mut any_legal_move = false;
        for m in moves {
            self.make_bit_move(m);
            if self.in_check(!self.side_to_move) {
                self.undo_move();
                continue;
            }
            any_legal_move = true;
            let evaluation = -self.negamax(depth - 1, -beta, -alpha);
            self.undo_move();
            if evaluation >= beta {
                return beta;
            }
            alpha = alpha.max(evaluation);
        }

        if !any_legal_move {
            if self.is_check() {
                // checkmate
                return -INF;
            }
            // stalemate
            return 0;
        }
        alpha
    }

    fn quiescence_search(&mut self, mut alpha: i32, beta: i32) -> i32 {
        let evaluation = self.evaluate();
        if evaluation >= beta {
            return beta;
        }
        alpha = alpha.max(evaluation);

        let mut capture_moves = self.generate_pseudo_legal_moves(true);
        capture_moves.sort();

        for m in capture_moves {
            self.make_bit_move(m);
            if self.in_check(!self.side_to_move) {
                self.undo_move();
                continue;
            }
            let evaluation = -self.evaluate();
            self.undo_move();
            if evaluation >= beta {
                return beta;
            }
            alpha = alpha.max(evaluation);
        }
        alpha
    }

    /// Searches for the best move with a given depth
    ///
    /// # Saftey
    ///
    /// This function will panic with an invalid board (stalemate, checkmate etc.)
    pub fn search(&mut self, depth: u32) -> BitMove {
        let mut best_move = BitMove::NULL;
        let mut max = -INF;
        for m in self.generate_legal_moves() {
            self.make_bit_move(m);
            let score = -self.negamax(depth, -INF, INF);
            self.undo_move();
            if score > max {
                max = score;
                best_move = m;
            }
        }
        best_move
    }
}
