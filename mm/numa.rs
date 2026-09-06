//! Automatically rewritten from C to Rust
//! Source: mm/numa.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

    struct pglist_data *node_data[MAX_NUMNODES];
    EXPORT_SYMBOL(node_data);
// Allocate NODE_DATA for a node on the local memory
#[no_mangle]
pub unsafe extern "C" fn alloc_node_data(nid: c_int) -> void __init {
    void __init alloc_node_data(int nid)
    {
    let mut nd_size: usize = roundup(sizeof(pg_data_t), SMP_CACHE_BYTES);
    u64 nd_pa;
    int tnid;
// Allocate node data.  Try node-local memory and then any node.
    nd_pa = memblock_phys_alloc_try_nid(nd_size, SMP_CACHE_BYTES, nid);
    if (!nd_pa)
    panic("Cannot allocate %zu bytes for node %d data\n",
    nd_size, nid);
// report and initialize
    pr_info("NODE_DATA(%d) allocated [mem %#010Lx-%#010Lx]\n", nid,
    nd_pa, nd_pa + nd_size - 1);
    tnid = early_pfn_to_nid(nd_pa >> PAGE_SHIFT);
    if (tnid != nid)
    pr_info("    NODE_DATA(%d) on node %d\n", nid, tnid);
    node_data[nid] = __va(nd_pa);
    memset(NODE_DATA(nid), 0, sizeof(pg_data_t));
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_offline_node_data(nid: c_int) -> void __init {
    void __init alloc_offline_node_data(int nid)
    {
    pg_data_t *pgdat;
    node_data[nid] = memblock_alloc_or_panic(sizeof(*pgdat), SMP_CACHE_BYTES);
    }
// Stub functions:

#[no_mangle]
pub unsafe extern "C" fn memory_add_physaddr_to_nid(start: u64) -> c_int {
    int memory_add_physaddr_to_nid(u64 start)
    {
    pr_info_once("Unknown online node for memory at 0x%llx, assuming node 0\n",
    start);
    return 0;
    }
    EXPORT_SYMBOL_GPL(memory_add_physaddr_to_nid);

#[no_mangle]
pub unsafe extern "C" fn phys_to_target_node(start: u64) -> c_int {
    int phys_to_target_node(u64 start)
    {
    pr_info_once("Unknown target node for memory at 0x%llx, assuming node 0\n",
    start);
    return 0;
    }
    EXPORT_SYMBOL_GPL(phys_to_target_node);
