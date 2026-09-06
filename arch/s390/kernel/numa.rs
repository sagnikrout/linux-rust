//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/numa.c
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
// NUMA support for s390
//
// Implement NUMA core code.
//
// Copyright IBM Corp. 2015
//

#[no_mangle]
pub unsafe extern "C" fn numa_setup() -> void __init {
    void __init numa_setup(void)
    {
    int nid;
    nodes_clear(node_possible_map);
    node_set(0, node_possible_map);
    node_set_online(0);
    for (nid = 0; nid < MAX_NUMNODES; nid++)
    NODE_DATA(nid) = memblock_alloc_or_panic(sizeof(pg_data_t), 8);
    NODE_DATA(0).node_spanned_pages = memblock_end_of_DRAM() >> PAGE_SHIFT;
    NODE_DATA(0).node_id = 0;
    }
