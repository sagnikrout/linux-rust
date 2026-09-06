//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/textsearch.h
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
// struct ts_state - search state
// @offset: offset for next match
// @cb: control buffer, for persistent variables of get_next_block()
//
// struct ts_ops - search module operations
// @name: name of search algorithm
// @init: initialization function to prepare a search
// @find: find the next occurrence of the pattern
// @destroy: destroy algorithm specific parts of a search configuration
// @get_pattern: return head of pattern
// @get_pattern_len: return length of pattern
// @owner: module reference to algorithm
// @list: list to search
//
extern "C" {
    pub fn int(: *mut *mut get_pattern_len)(struct ts_config) -> unsigned;
}
//
// struct ts_config - search configuration
// @ops: operations of chosen algorithm
// @flags: flags
// @get_next_block: callback to fetch the next block to search in
// @finish: callback to finalize a search
//
// @get_next_block: fetch next block of data
// @consumed: number of bytes consumed by the caller
// @dst: destination buffer
// @conf: search configuration
// @state: search state
//
// Called repeatedly until 0 is returned. Must assign the
// head of the next block of data to &*dst and return the length
// of the block or 0 if at the end. consumed == 0 indicates
// a new search. May store/read persistent values in state->cb.
//
// @finish: finalize/clean a series of get_next_block() calls
// @conf: search configuration
// @state: search state
//
// Called after the last use of get_next_block(), may be used
// to cleanup any leftovers.
//
// textsearch_next - continue searching for a pattern
// @conf: search configuration
// @state: search state
//
// Continues a search looking for more occurrences of the pattern.
// textsearch_find() must be called to find the first occurrence
// in order to reset the state.
//
// Returns the position of the next occurrence of the pattern or
// UINT_MAX if not match was found.
//
// textsearch_find - start searching for a pattern
// @conf: search configuration
// @state: search state
//
// Returns the position of first occurrence of the pattern or
// UINT_MAX if no match was found.
//
extern "C" {
    pub fn textsearch_next(_arg: conf, _arg: state) -> return;
}
//
// textsearch_get_pattern - return head of the pattern
// @conf: search configuration
//
// textsearch_get_pattern_len - return length of the pattern
// @conf: search configuration
//
extern "C" {
    pub fn textsearch_register(: *mut ts_ops) -> c_int;
}
extern "C" {
    pub fn textsearch_unregister(: *mut ts_ops) -> c_int;
}
extern "C" {
    pub fn textsearch_destroy(conf: *mut ts_config);
}
pub const TS_PRIV_ALIGNTO: c_int = 8;

extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
