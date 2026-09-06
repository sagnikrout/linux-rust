//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/eq.h
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
// Copyright (c) 2018 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eq_param {
    pub nent: c_int,
    pub mask: [u64; 4],
    pub irq: *mut mlx5_irq,
}

extern "C" {
    pub fn mlx5_eq_update_ci(eq: *mut mlx5_eq, cc: u32, arm: bool);
}
// The HCA will think the queue has overflowed if we
// don't tell it we've been processing events.  We
// create EQs with MLX5_NUM_SPARE_EQE extra entries,
// so we must update our consumer index at
// least that often.
//
// mlx5_eq_update_cc must be called on every EQE @EQ irq handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_nb {
    pub nb: notifier_block,
    pub event_type: u8,
}

