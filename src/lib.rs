use std::io;
use adversarial_search::alphabeta::alphabeta;

pub fn run() {
    println!("Search depth: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let depth = input.trim().parse::<i32>().unwrap();
    let root = Node::new();
    println!(
        "{:?} {}",
        root.board,
        evaluate_node(root, depth)
    );
}

fn evaluate_node(node: Node, depth: i32) -> f32 {
    alphabeta(
        &node,
        depth,
        &|n| n.get_children(),
        &|n| n.get_terminality(),
        &|n| n.value() as i32,
        node.player == Player::HUMAN,
        -std::f32::INFINITY,
        std::f32::INFINITY,
    )
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Player {
    COMPUTER,
    HUMAN,
}

fn next_player(player: Player) -> Player {
    match player {
        Player::COMPUTER => Player::HUMAN,
        Player::HUMAN => Player::COMPUTER,
    }
}

#[derive(Copy, Clone)]
struct Node {
    move_count: i32,
    board: [i8; 14],
    player: Player,
}

impl Node {
    const HUMAN_TILE_INDEX: usize = 6;
    const COMPUTER_TILE_INDEX: usize = 13;

    pub fn new() -> Node {
        Node {
            move_count: 0,
            board: [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0],
            player: Player::HUMAN,
        }
    }

    pub fn clone(&self) -> Node {
        let mut node = Node::new();
        node.move_count = self.move_count;
        node.board = self.board;
        node.player = self.player;
        node
    }

    fn current_player_tile_index(&self) -> usize {
        match self.player {
            Player::HUMAN => Node::HUMAN_TILE_INDEX,
            Player::COMPUTER => Node::COMPUTER_TILE_INDEX,
        }
    }

    fn get_adjacent_tile_index(tile_index: usize) -> usize {
        /*
          6
         7 5
         8 4
         9 3
        10 2
        11 1
        12 0
         13
        */
        12 - tile_index
    }

    fn steal_pieces(&mut self, tile_index: usize) {
        let adjacent_index = Node::get_adjacent_tile_index(tile_index);
        let player_index = self.current_player_tile_index();
        self.board[player_index] += 1 + self.board[adjacent_index];
        self.board[adjacent_index] = 0;
        self.board[tile_index] = 0;
    }

    pub fn move_piece(&mut self, tile_index: usize) {
        let mut pieces = self.board[tile_index];
        self.board[tile_index] = 0;
        let mut index = tile_index;
        while pieces > 0 {
            index += 1;
            if (self.player == Player::COMPUTER && index == Node::HUMAN_TILE_INDEX)
                || (self.player == Player::HUMAN && index == Node::COMPUTER_TILE_INDEX)
            {
                index += 1;
            }
            if index >= 14 {
                index = 0
            }
            if pieces == 1
                && index != Node::COMPUTER_TILE_INDEX
                && index != Node::HUMAN_TILE_INDEX
                && self.board[index] == 0
                && self.board[Node::get_adjacent_tile_index(index)] != 0
            {
                self.steal_pieces(index);
            } else {
                self.board[index] += 1;
            }
            pieces -= 1;
        }
        if self.current_player_tile_index() != index {
            self.player = next_player(self.player);
        }
        self.move_count += 1;
    }

    fn available_moves(&self) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();
        let player_index = self.current_player_tile_index();
        let tiles = (player_index - 6)..player_index;
        for i in tiles {
            if self.board[i] > 0 {
                moves.push(i);
            }
        }
        moves
    }

    pub fn get_children(&self) -> Vec<Node> {
        let moves = self.available_moves();
        let mut children: Vec<Node> = Vec::new();
        for m in moves {
            let mut child = self.clone();
            child.move_piece(m);
            children.push(child);
        }
        children.clone()
    }

    pub fn is_terminal(&self) -> bool {
        self.available_moves().len() == 0
    }

    pub fn get_terminality(&self) -> Option<f32> {
        if self.is_terminal() {
            let val = self.value();
            if val > 0 {
                Some(std::f32::INFINITY)
            } else if val < 0 {
                Some(-std::f32::INFINITY)
            } else {
                Some(0.0)
            }
        } else {
            None
        }
    }

    pub fn value(&self) -> i8 {
        self.board[Node::HUMAN_TILE_INDEX] - self.board[Node::COMPUTER_TILE_INDEX]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_logic() {
        let mut node = Node::new();
        assert_eq!(node.board, [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0]);
        assert_eq!(node.player, Player::HUMAN);
        node.move_piece(2);
        assert_eq!(node.board, [4, 4, 0, 5, 5, 5, 1, 4, 4, 4, 4, 4, 4, 0]);
        assert_eq!(node.player, Player::HUMAN);
        node.move_piece(5);
        assert_eq!(node.board, [4, 4, 0, 5, 5, 0, 2, 5, 5, 5, 5, 4, 4, 0]);
        assert_eq!(node.player, Player::COMPUTER);
        node.move_piece(12);
        assert_eq!(node.board, [5, 5, 0, 5, 5, 0, 2, 5, 5, 5, 0, 4, 0, 7]);
    }
    #[test]
    fn available_moves() {
        let node = Node::new();
        assert_eq!(node.available_moves(), vec![0, 1, 2, 3, 4, 5]);
    }
}
