//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/bwc_complex.h
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
// Copyright (c) 2024 NVIDIA Corporation & Affiliates
pub const MLX5HWS_BWC_COMPLEX_MAX_SUBMATCHERS: c_int = 4;
// A matcher can't contain two rules with the same match tag, but it is possible
// that two different complex rules' subrules have the same match tag. In that
// case, those subrules correspond to a single rule, and we need to refcount.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_bwc_complex_subrule_data {
    pub match_tag: mlx5hws_rule_match_tag,
    pub refcount: refcount_t,
// The chain_id is what glues individual subrules into larger complex
// rules. It is the value that this subrule writes to register C6, and
// that the next subrule matches against.
//
    pub chain_id: u32,
    pub rtc_0: u32,
    pub rtc_1: u32,
// During rehash we iterate through all the subrules to move them. But
// two or more subrules can share the same physical rule in the
// submatcher, so we use `was_moved` to keep track if a given rule was
// already moved.
//
    pub was_moved: bool,
    pub hash_node: rhash_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_bwc_complex_submatcher {
// Isolated table that the matcher lives in. Not set for the first
// matcher, which lives in the original table.
//
    pub tbl: *mut mlx5hws_table,
// Match a rule with this action to go to `tbl`. This is set in all
// submatchers but the first.
//
    pub action_tbl: *mut mlx5hws_action,
// This submatcher's simple matcher. The first submatcher points to the
// outer (complex) matcher.
//
    pub bwc_matcher: *mut mlx5hws_bwc_matcher,
    pub rules_hash: rhashtable,
    pub chain_ida: ida,
    pub /: *mut *mut mutex hash_lock; / Protect the hash and ida.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_bwc_matcher_complex_data {
    pub num_submatchers: c_int,
// Actions used by all but the last submatcher to point to the next
// submatcher in the chain. The last submatcher uses the action template
// from the complex matcher, to perform the actions that the user
// originally requested.
//
    pub action_metadata: *mut mlx5hws_action,
    pub action_last: *mut mlx5hws_action,
}

extern "C" {
    pub fn mlx5hws_bwc_matcher_destroy_complex(bwc_matcher: *mut mlx5hws_bwc_matcher);
}
extern "C" {
    pub fn mlx5hws_bwc_matcher_complex_move(bwc_matcher: *mut mlx5hws_bwc_matcher) -> c_int;
}
extern "C" {
    pub fn mlx5hws_bwc_rule_destroy_complex(bwc_rule: *mut mlx5hws_bwc_rule) -> c_int;
}
