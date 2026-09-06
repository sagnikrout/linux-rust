//! Automatically rewritten from C to Rust
//! Source: drivers/md/bcache/trace.c
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

// Macro flag: #define CREATE_TRACE_POINTS

    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_request_start);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_request_end);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_bypass_sequential);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_bypass_congested);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_read);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_write);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_read_retry);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_cache_insert);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_journal_replay_key);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_journal_write);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_journal_full);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_journal_entry_full);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_cache_cannibalize);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_read);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_write);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_node_alloc);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_node_alloc_fail);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_node_free);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_gc_coalesce);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_gc_start);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_gc_end);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_gc_copy);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_gc_copy_collision);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_insert_key);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_node_split);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_node_compact);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_btree_set_root);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_invalidate);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_alloc_fail);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_writeback);
    EXPORT_TRACEPOINT_SYMBOL_GPL(bcache_writeback_collision);
