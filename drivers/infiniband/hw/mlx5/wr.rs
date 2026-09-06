//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/wr.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2020, Mellanox Technologies inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_eth_pad {
    pub rsvd0: [u8; 16],
}

// get_sq_edge - Get the next nearby edge.
//
// An 'edge' is defined as the first following address after the end
// of the fragment or the SQ. Accordingly, during the WQE construction
// which repetitively increases the pointer to write the next data, it
// simply should check if it gets to an edge.
//
// @sq - SQ buffer.
// @idx - Stride index in the SQ buffer.
//
// Return:
// The new edge.
//
// handle_post_send_edge - Check if we get to SQ edge. If yes, update to the
// next nearby edge and get new address translation for current WQE position.
// @sq: SQ buffer.
// @seg: Current WQE position (16B aligned).
// @wqe_sz: Total current WQE size [16B].
// @cur_edge: Updated current edge.
//
// cur_edge = get_sq_edge(sq, idx);
// seg = mlx5_frag_buf_get_wqe(&sq->fbc, idx);
// mlx5r_memcpy_send_wqe - copy data from src to WQE and update the relevant
// WQ's pointers. At the end @seg is aligned to 16B regardless the copied size.
// @sq: SQ buffer.
// @cur_edge: Updated current edge.
// @seg: Current WQE position (16B aligned).
// @wqe_sz: Total current WQE size [16B].
// @src: Pointer to copy from.
// @n: Number of bytes to copy.
//
// seg += stride;
// wqe_sz += stride >> 4;
extern "C" {
    pub fn mlx5r_wq_overflow(wq: *mut mlx5_ib_wq, nreq: c_int, ib_cq: *mut ib_cq) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_post_send(_arg: ibqp, _arg: wr, _arg: bad_wr, _arg: false) -> return;
}
extern "C" {
    pub fn mlx5_ib_post_send(_arg: ibqp, _arg: wr, _arg: bad_wr, _arg: true) -> return;
}
extern "C" {
    pub fn mlx5_ib_post_recv(_arg: ibqp, _arg: wr, _arg: bad_wr, _arg: false) -> return;
}
extern "C" {
    pub fn mlx5_ib_post_recv(_arg: ibqp, _arg: wr, _arg: bad_wr, _arg: true) -> return;
}
