#![allow(dead_code)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

const HEIGHT: usize = 8;
const WIDTH: usize = 8;
const N: usize = 64;

#[wasm_bindgen]
pub struct Reversi {
    turn: usize,
    isfirst: bool,
    my_pieces: u64,
    op_pieces: u64,
    flip_list: Vec<u8>,
    eval_value : i32
}


#[wasm_bindgen]
impl Reversi {
    pub fn new() -> Reversi {
        let black = (1u64 << (8*3 + 4)) | (1u64 << (8*4 + 3));
        let white = (1u64 << (8*3 + 3)) | (1u64 << (8*4 + 4));
        Reversi {
            turn: 0, isfirst: false, my_pieces: white, op_pieces: black, flip_list: Vec::new(), eval_value: 0
        }
    }

    pub fn clear(&mut self) {
        self.turn = 0;
        self.my_pieces = (1u64 << (8*3 + 3)) | (1u64 << (8*4 + 4));
        self.op_pieces = (1u64 << (8*3 + 4)) | (1u64 << (8*4 + 3));
        if self.isfirst {
            std::mem::swap(&mut self.my_pieces, &mut self.op_pieces);
        }
        self.eval_value = 0;
    }

    pub fn change(&mut self) {
        self.isfirst = !self.isfirst;
        std::mem::swap(&mut self.my_pieces, &mut self.op_pieces);
    }

    pub fn piece_count(&self) -> i32 {
        let mut sub = self.my_pieces.count_ones() as i32 - self.op_pieces.count_ones() as i32;
        if !self.isfirst {sub = -sub;}
        sub
    }

    pub fn flip_list(&self) -> *const u8 {
        self.flip_list.as_ptr()
    }

    pub fn eval_value(&self) -> i32 {
        self.eval_value
    }

    pub fn is_movable(&self) -> bool {
        let moves = Reversi::get_moves(self.op_pieces, self.my_pieces);
        if moves == 0 {false} else {true}
    }

    pub fn set_op_piece(&mut self, pos: usize) -> u32 {
        let moves = Reversi::get_moves(self.op_pieces, self.my_pieces);
        if (moves >> pos) & 1 == 0 {return 0;}
        let mut flip = Reversi::flip_pieces(pos, self.op_pieces, self.my_pieces);
        flip |= 1 << pos;
        self.op_pieces |= flip;
        self.my_pieces &= !flip;
        self.turn += 1;
        self.set_fliplist(flip);
        self.flip_list.len() as u32
    }

    pub fn search_next_piece(&mut self) -> u32 {
        let mut moves = Reversi::get_moves(self.my_pieces, self.op_pieces);
        if moves == 0 {return 0;}
        let mut next_pos = 0;
        let (mut low, high) = (-10000, 10000);
        let depth = if self.turn <= 40 {10} else {15};
        while moves > 0 {
            let pos = moves & !moves + 1;

            let flip = Reversi::flip_pieces(pos.trailing_zeros() as usize, self.my_pieces, self.op_pieces);
            self.my_pieces |= flip; self.my_pieces |= pos;
            self.op_pieces &= !flip;
            
            let value = -self.alpha_beta_search(depth, -high, -low, self.turn + 1, false);
            
            self.my_pieces &= !flip; self.my_pieces &= !pos;
            self.op_pieces |= flip;

            if low < value {
                low = value;
                next_pos = pos.trailing_zeros() as usize;
            }
            moves &= moves - 1;
        }

        if !self.isfirst {low = -low;}
        self.eval_value = low;

        let mut flip = Reversi::flip_pieces(next_pos, self.my_pieces, self.op_pieces);
        flip |= 1 << next_pos;
        self.my_pieces |= flip;
        self.op_pieces &= !flip;
        self.turn += 1;
        self.set_fliplist(flip);
        self.flip_list.len() as u32
    }
    
}

impl Reversi {

    pub fn evaluate(&self) -> i32 {
        self.my_pieces.count_ones() as i32 - self.op_pieces.count_ones() as i32
    }

    pub fn alpha_beta_search(&mut self, depth: usize, mut low: i32, high: i32, turn: usize, is_myTurn: bool) -> i32 {
        if depth == 0 || turn == N {
            return if is_myTurn {self.evaluate()} else {-self.evaluate()};
        }
        let mut moves =
        if is_myTurn {
            Reversi::get_moves(self.my_pieces, self.op_pieces)
        }
        else {
            Reversi::get_moves(self.op_pieces, self.my_pieces)
        };

        if moves == 0 {
            return -self.alpha_beta_search(depth - 1, -high, -low, turn, !is_myTurn);
        }
        let (pre_my, pre_op) = (self.my_pieces, self.op_pieces);
        while moves > 0 {
            let pos = moves & !moves + 1;
            if is_myTurn {
                let flip = Reversi::flip_pieces(pos.trailing_zeros() as usize, self.my_pieces, self.op_pieces);
                self.my_pieces |= flip; self.my_pieces |= pos;
                self.op_pieces &= !flip;
            }
            else {
                let flip = Reversi::flip_pieces(pos.trailing_zeros() as usize, self.op_pieces, self.my_pieces);
                self.op_pieces |= flip; self.op_pieces |= pos;
                self.my_pieces &= !flip;
            }

            let value = -self.alpha_beta_search(depth - 1, -high, -low, turn + 1, !is_myTurn);
            
            self.my_pieces = pre_my;
            self.op_pieces = pre_op;

            low = std::cmp::max(low, value);
            if low >= high {break;}

            moves &= moves - 1;
        }
        return low;
    }

    pub fn set_fliplist(&mut self, mut flip: u64) {
        self.flip_list.clear();
        while flip > 0 {
            self.flip_list.push(flip.trailing_zeros() as u8);
            flip &= flip - 1;
        }
    }

    pub fn get_moves(P: u64, O: u64) -> u64 {
        let mut moves = 0;
        let mO = O & 0x7e7e7e7e7e7e7e7eu64;
        let mut flip = (mO & (P<<1), mO & (P<<7), O & (P<<8), mO & (P<<9));
        flip.0 |= mO & (flip.0 << 1);
        flip.1 |= mO & (flip.1 << 7);
        flip.2 |= O & (flip.2 << 8);
        flip.3 |= mO & (flip.3 << 9);
        let mut pre = (mO & (mO<<1), mO & (mO<<7), O & (O<<8), mO & (mO<<9));
        flip.0 |= pre.0 & (flip.0 << 2); flip.0 |= pre.0 & (flip.0 << 2); moves |= flip.0 << 1;
        flip.1 |= pre.1 & (flip.1 << 14); flip.1 |= pre.1 & (flip.1 << 14); moves |= flip.1 << 7;
        flip.2 |= pre.2 & (flip.2 << 16); flip.2 |= pre.2 & (flip.2 << 16); moves |= flip.2 << 8;
        flip.3 |= pre.3 & (flip.3 << 18); flip.3 |= pre.3 & (flip.3 << 18); moves |= flip.3 << 9;
        flip = (mO & (P>>1), mO & (P>>7), O & (P>>8), mO & (P>>9));
        flip.0 |= mO & (flip.0 >> 1);
        flip.1 |= mO & (flip.1 >> 7);
        flip.2 |= O & (flip.2 >> 8);
        flip.3 |= mO & (flip.3 >> 9);
        pre.0 >>= 1;
        pre.1 >>= 7;
        pre.2 >>= 8;
        pre.3 >>= 9;
        flip.0 |= pre.0 & (flip.0 >> 2); flip.0 |= pre.0 & (flip.0 >> 2); moves |= flip.0 >> 1;
        flip.1 |= pre.1 & (flip.1 >> 14); flip.1 |= pre.1 & (flip.1 >> 14); moves |= flip.1 >> 7;
        flip.2 |= pre.2 & (flip.2 >> 16); flip.2 |= pre.2 & (flip.2 >> 16); moves |= flip.2 >> 8;
        flip.3 |= pre.3 & (flip.3 >> 18); flip.3 |= pre.3 & (flip.3 >> 18); moves |= flip.3 >> 9;

        moves & !(P|O)
    }

    pub fn flip_pieces(pos: usize, P: u64, O: u64) -> u64 {
        let mut flip = 0;
        let OM = (O, O & 0x7e7e7e7e7e7e7e7eu64, O & 0x7e7e7e7e7e7e7e7eu64, O & 0x7e7e7e7e7e7e7e7eu64);
        let mut mask = (0x0080808080808080u64, 0x7f00000000000000u64, 0x0102040810204000u64, 0x0040201008040201u64);
        mask.0 >>= 63 - pos;
        mask.1 >>= 63 - pos;
        mask.2 >>= 63 - pos;
        mask.3 >>= 63 - pos;
        let mut outflank = (
            (0x8000000000000000u64 >> (!OM.0 & mask.0).leading_zeros()) & P,
            (0x8000000000000000u64 >> (!OM.1 & mask.1).leading_zeros()) & P,
            (0x8000000000000000u64 >> (!OM.2 & mask.2).leading_zeros()) & P,
            (0x8000000000000000u64 >> (!OM.3 & mask.3).leading_zeros()) & P,
        );
        flip |= ((!outflank.0 + 1) * 2) & mask.0;
        flip |= ((!outflank.1 + 1) * 2) & mask.1;
        flip |= ((!outflank.2 + 1) * 2) & mask.2;
        flip |= ((!outflank.3 + 1) * 2) & mask.3;
        mask = (0x0101010101010100u64, 0x00000000000000feu64, 0x0002040810204080u64, 0x8040201008040200u64);
        mask.0 <<= pos;
        mask.1 <<= pos;
        mask.2 <<= pos;
        mask.3 <<= pos;
        outflank = (
            mask.0 & ((OM.0 | !mask.0) + 1) & P,
            mask.1 & ((OM.1 | !mask.1) + 1) & P,
            mask.2 & ((OM.2 | !mask.2) + 1) & P,
            mask.3 & ((OM.3 | !mask.3) + 1) & P,
        );
        flip |= (outflank.0 - (outflank.0 != 0) as u64) & mask.0;
        flip |= (outflank.1 - (outflank.1 != 0) as u64) & mask.1;
        flip |= (outflank.2 - (outflank.2 != 0) as u64) & mask.2;
        flip |= (outflank.3 - (outflank.3 != 0) as u64) & mask.3;
        flip
    }

}
/*
use std::fmt;

impl fmt::Display for Reversi {
}
*/