//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho_block.h
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
// Copyright (c) 2026, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

//
// struct kho_block - Internal representation of a serialization block.
// @list: List head for linking blocks in memory.
// @ser:  Pointer to the serialized header in preserved memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_block {
    pub list: list_head,
    pub ser: *mut kho_block_header_ser,
}

//
// struct kho_block_set - A set of blocks containing serialized entries of the same type.
// @blocks:          The list of serialization blocks (struct kho_block).
// @nblocks:         The number of allocated serialization blocks.
// @head_pa:         Physical address of the first block header.
// @entry_size:      The size of each entry in the blocks.
// @count_per_block: The maximum number of entries each block can hold.
// @incoming:        True if this block set was restored from the previous kernel.
//
// Note: Synchronization and locking are the responsibility of the caller.
// The block set structure itself is not internally synchronized.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_block_set {
    pub blocks: list_head,
    pub nblocks: c_long,
    pub head_pa: u64,
    pub entry_size: usize,
    pub count_per_block: u64,
    pub incoming: bool,
}

//
// struct kho_block_set_it - Iterator for serializing entries into blocks.
// @bs:         The block set being iterated.
// @block:      The current block.
// @i:          The current entry index within @block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_block_set_it {
    pub bs: *mut kho_block_set,
    pub block: *mut kho_block,
    pub i: u64,
}

//
// KHO_BLOCK_SET_INIT - Initialize a static kho_block_set.
// @_name:       Name of the kho_block_set variable.
// @_entry_size: The size of each entry in the block set.
//

extern "C" {
    pub fn kho_block_set_init(bs: *mut kho_block_set, entry_size: usize);
}
extern "C" {
    pub fn kho_block_set_grow(bs: *mut kho_block_set, count: u64) -> c_int;
}
extern "C" {
    pub fn kho_block_set_shrink(bs: *mut kho_block_set, count: u64);
}
extern "C" {
    pub fn kho_block_set_restore(bs: *mut kho_block_set, head_pa: u64) -> c_int;
}
extern "C" {
    pub fn kho_block_set_destroy(bs: *mut kho_block_set);
}
extern "C" {
    pub fn kho_block_set_clear(bs: *mut kho_block_set);
}
//
// kho_block_set_head_pa - Get the physical address of the first block header.
// @bs: The block set.
//
// Return: The physical address of the first block header, or 0 if empty.
//
// kho_block_set_is_empty - Check if the block set has no allocated blocks.
// @bs: The block set.
//
// Return: True if there are no blocks in the set, false otherwise.
//
extern "C" {
    pub fn list_empty(_arg: &bs->blocks) -> return;
}
extern "C" {
    pub fn kho_block_set_it_init(it: *mut kho_block_set_it, bs: *mut kho_block_set);
}
