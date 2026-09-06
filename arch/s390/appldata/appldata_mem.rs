//! Automatically rewritten from C to Rust
//! Source: arch/s390/appldata/appldata_mem.c
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
// Data gathering module for Linux-VM Monitor Stream, Stage 1.
// Collects data related to memory management.
//
// Copyright IBM Corp. 2003, 2006
//
// Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
//

//
// Memory data
//
// This is accessed as binary data by z/VM. If changes to it can't be avoided,
// the structure version (product ID, see appldata_base.c) needs to be changed
// as well and all documentation and z/VM applications using it must be
// updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_mem_data {
    pub timestamp: u64,
    pub /: *mut *mut u32 sync_count_1; / after VM collected the record data,,
    pub the: *mut *mut u32 sync_count_2; / sync_count_1 and sync_count_2 should be,
    same. If not, the record has been updated on
    the Linux side while VM was collecting the
    (possibly corrupt) data */
    pub /: *mut *mut u64 pgpgin; / data read from disk,
    pub /: *mut *mut u64 pgpgout; / data written to disk,
    pub /: *mut *mut u64 pswpin; / pages swapped in,
    pub /: *mut *mut u64 pswpout; / pages swapped out,
    pub /: *mut *mut u64 sharedram; / sharedram is currently set to 0,
    pub /: *mut *mut u64 totalram; / total main memory size,
    pub /: *mut *mut u64 freeram; / free main memory size,
    pub /: *mut *mut u64 totalhigh; / total high memory size,
    pub /: *mut *mut u64 freehigh; / free high memory size,
    pub /: *mut *mut u64 bufferram; / memory reserved for buffers, free cache,
    pub /: *mut *mut u64 cached; / size of (used) cache, w/o buffers,
    pub /: *mut *mut u64 totalswap; / total swap space size,
    pub /: *mut *mut u64 freeswap; / free swap space,
// New in 2.6 -->
    pub /: *mut *mut u64 pgalloc; / page allocations,
    pub /: *mut *mut u64 pgfault; / page faults (major+minor),
    pub /: *mut *mut u64 pgmajfault; / page faults (major only),
// <-- New in 2.6
    pub __packed: },
//
// appldata_get_mem_data()
//
// gather memory data
//
#[no_mangle]
unsafe extern "C" fn appldata_get_mem_data(data: *mut c_void) {
    static void appldata_get_mem_data(void *data)
    {
//
// don't put large structures on the stack, we are
// serialized through the appldata_ops_mutex and can use static
//
    pub val: static struct sysinfo,
    pub ev: [c_ulong; NR_VM_EVENT_ITEMS],
    pub mem_data: *mut appldata_mem_data,
    pub data: mem_data =,
    pub 1: mem_data->pgpgin = ev[PGPGIN] >>,
    pub 1: mem_data->pgpgout = ev[PGPGOUT] >>,
    pub ev: [mem_data->pswpin =; PSWPIN],
    pub ev: [mem_data->pswpout =; PSWPOUT],
    pub ev: [mem_data->pgalloc =; PGALLOC_NORMAL],
    pub ev: [mem_data->pgalloc +=; PGALLOC_DMA],
    pub ev: [mem_data->pgfault =; PGFAULT],
    pub ev: [mem_data->pgmajfault =; PGMAJFAULT],
    pub val.sharedram: mem_data->sharedram =,
    pub P2K(val.totalram): mem_data->totalram =,
    pub P2K(val.freeram): mem_data->freeram =,
    pub P2K(val.totalhigh): mem_data->totalhigh =,
    pub P2K(val.freehigh): mem_data->freehigh =,
    pub P2K(val.bufferram): mem_data->bufferram =,
    mem_data.cached    = P2K(global_node_page_state(NR_FILE_PAGES)
    pub val.bufferram): -,
    pub P2K(val.totalswap): mem_data->totalswap =,
    pub P2K(val.freeswap): mem_data->freeswap =,
    pub get_tod_clock(): mem_data->timestamp =,
    }
    static struct appldata_ops ops = {
    .name      = "mem",
    .record_nr = APPLDATA_RECORD_MEM_ID,
    .size	   = sizeof(struct appldata_mem_data),
    .callback  = &appldata_get_mem_data,
    .owner     = THIS_MODULE,
    .mod_lvl   = {0xF0, 0xF0},		/* EBCDIC "00" */
}

//
// appldata_mem_init()
//
// init_data, register ops
//
#[no_mangle]
unsafe extern "C" fn appldata_mem_init() -> int __init {
    static int __init appldata_mem_init(void)
    {
    int ret;
    ops.data = kzalloc_obj(struct appldata_mem_data);
    if (!ops.data)
    return -ENOMEM;
    ret = appldata_register_ops(&ops);
    if (ret)
    kfree(ops.data);
    return ret;
    }
//
// appldata_mem_exit()
//
// unregister ops
//
#[no_mangle]
unsafe extern "C" fn appldata_mem_exit() -> void __exit {
    static void __exit appldata_mem_exit(void)
    {
    appldata_unregister_ops(&ops);
    kfree(ops.data);
    }
    module_init(appldata_mem_init);
    module_exit(appldata_mem_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Gerald Schaefer");
    MODULE_DESCRIPTION("Linux-VM Monitor Stream, MEMORY statistics");
