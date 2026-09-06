//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/tree-checker.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) Qu Wenruo 2017.  All rights reserved.
//

// All the extra info needed to verify the parentness of a tree block.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_tree_parent_check {
//
// The owner check against the tree block.
//
// Can be 0 to skip the owner check.
//
    pub owner_root: u64,
//
// Expected transid, can be 0 to skip the check, but such skip
// should only be utilized for backref walk related code.
//
    pub transid: u64,
//
// The expected first key.
//
// This check can be skipped if @has_first_key is false, such skip
// can happen for case where we don't have the parent node key,
// e.g. reading the tree root, doing backref walk.
//
    pub first_key: btrfs_key,
    pub has_first_key: bool,
// The expected level. Should always be set.
    pub level: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_tree_block_status {
    BTRFS_TREE_BLOCK_CLEAN,
    BTRFS_TREE_BLOCK_INVALID_NRITEMS,
    BTRFS_TREE_BLOCK_INVALID_PARENT_KEY,
    BTRFS_TREE_BLOCK_BAD_KEY_ORDER,
    BTRFS_TREE_BLOCK_INVALID_LEVEL,
    BTRFS_TREE_BLOCK_INVALID_FREE_SPACE,
    BTRFS_TREE_BLOCK_INVALID_OFFSETS,
    BTRFS_TREE_BLOCK_INVALID_BLOCKPTR,
    BTRFS_TREE_BLOCK_INVALID_ITEM,
    BTRFS_TREE_BLOCK_INVALID_OWNER,
    BTRFS_TREE_BLOCK_WRITTEN_NOT_SET,
}

//
// Exported simply for btrfs-progs which wants to have the
// btrfs_tree_block_status return codes.
//
extern "C" {
    pub fn __btrfs_check_leaf(leaf: *mut extent_buffer) -> btrfs_tree_block_status;
}
extern "C" {
    pub fn __btrfs_check_node(node: *mut extent_buffer) -> btrfs_tree_block_status;
}
extern "C" {
    pub fn btrfs_check_leaf(leaf: *mut extent_buffer) -> c_int;
}
extern "C" {
    pub fn btrfs_check_node(node: *mut extent_buffer) -> c_int;
}
extern "C" {
    pub fn btrfs_check_eb_owner(eb: *const extent_buffer, root_owner: u64) -> c_int;
}
