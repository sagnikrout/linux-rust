//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/pio.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015-2017 Intel Corporation.
//
// send context types
pub const SC_KERNEL: c_int = 0;
pub const SC_VL15: c_int = 1;
pub const SC_ACK: c_int = 2;

// invalid send context index
pub const INVALID_SCI: c_uint = 0xff;
// PIO buffer release callback function
extern "C" {
    pub fn void(arg: *mut *mut pio_release_cb)(void, code: c_int) -> typedef;
}
// PIO release codes - in bits, as there could more than one that apply

pub const PRC_STATUS_ERR: c_uint = 0x01	/* credit return due to status error */;
pub const PRC_PBC: c_uint = 0x02	/* credit return due to PBC */;
pub const PRC_THRESHOLD: c_uint = 0x04	/* credit return due to threshold */;
pub const PRC_FILL_ERR: c_uint = 0x08	/* credit return due fill error */;
pub const PRC_FORCE: c_uint = 0x10	/* credit return due credit force */;
pub const PRC_SC_DISABLE: c_uint = 0x20	/* clean-up after a context disable */;
// byte helper
#[repr(C)]
#[derive(Copy, Clone)]
pub union mix {
    pub val64: u64,
    pub val32: [u32; 2],
    pub val8: [u8; 8],
}

// an allocated PIO buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio_buf {
    pub /: *mut *mut *mut send_context sc;/ back pointer to owning send context,
    pub /: *mut *mut pio_release_cb cb; / called when the buffer is released,
    pub /: *mut *mut *mut void arg; / argument for cb,
    pub /: *mut *mut *mut void __iomem start; / buffer start address,
    pub /: *mut *mut *mut void __iomem end; / context end address,
    pub /: *mut *mut unsigned long sent_at; / buffer is sent when <= free,
    pub /: *mut *mut mix carry; / pending unwritten bytes,
    pub /: *mut *mut u16 qw_written; / QW written so far,
    pub /: *mut *mut u8 carry_bytes; / number of valid bytes in carry,
}

// cache line aligned pio buffer array
#[repr(C)]
#[derive(Copy, Clone)]
pub union pio_shadow_ring {
    pub pbuf: pio_buf,
    pub ____cacheline_aligned: },
// per-NUMA send context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_context {
// read-only after init
    pub /: *mut *mut *mut hfi1_devdata dd; / device,
    pub /: *mut *mut *mut pio_shadow_ring sr; / shadow ring,
    pub /: *mut *mut *mut void __iomem base_addr; / start of PIO memory,
    pub /: *mut *mut *mut u32 __percpu buffers_allocated;/ count of buffers allocated,
    pub /: *mut *mut u32 size; / context size, in bytes,
    pub /: *mut *mut int node; / context home node,
    pub /: *mut *mut u32 sr_size; / size of the shadow ring,
    pub /: *mut *mut u16 flags; / flags,
    pub /: *mut *mut u8 type; / context type,
    pub /: *mut *mut u8 sw_index; / software index number,
    pub /: *mut *mut u8 hw_context; / hardware context number,
    pub /: *mut *mut u8 group; / credit return group,
// allocator fields
    pub ____cacheline_aligned_in_smp: spinlock_t alloc_lock,
    pub /: *mut *mut u32 sr_head; / shadow ring head,
    pub /: *mut *mut unsigned long fill; / official alloc count,
    pub /: *mut *mut unsigned long alloc_free; / copy of free (less cache thrash),
    pub /: *mut *mut u32 fill_wrap; / tracks fill within ring,
    pub /: *mut *mut u32 credits; / number of blocks in context,
// adding a new field here would make it part of this cacheline
// releaser fields
    pub ____cacheline_aligned_in_smp: spinlock_t release_lock,
    pub /: *mut *mut u32 sr_tail; / shadow ring tail,
    pub /: *mut *mut unsigned long free; / official free count,
    pub /: *mut *mut *mut volatile __le64 hw_free; / HW free counter,
// list for PIO waiters
    pub ____cacheline_aligned_in_smp: list_head piowait,
    pub waitlock: seqlock_t,
    pub ____cacheline_aligned_in_smp: spinlock_t credit_ctrl_lock,
    pub /: *mut *mut u32 credit_intr_count; / count of credit intr users,
    pub /: *mut *mut u64 credit_ctrl; / cache for credit control,
    pub /: *mut *mut wait_queue_head_t halt_wait; / wait until kernel sees interrupt,
    pub /: *mut *mut work_halt_work; / halted context work queue entry,
}

// send context flags
pub const SCF_ENABLED: c_uint = 0x01;
pub const SCF_IN_FREE: c_uint = 0x02;
pub const SCF_HALTED: c_uint = 0x04;
pub const SCF_FROZEN: c_uint = 0x08;
pub const SCF_LINK_DOWN: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_context_info {
    pub /: *mut *mut *mut send_context sc; / allocated working context,
    pub /: *mut *mut u16 allocated; / has this been allocated?,
    pub /: *mut *mut u16 type; / context type,
    pub /: *mut *mut u16 base; / base in PIO array,
    pub /: *mut *mut u16 credits; / size in PIO array,
}

// DMA credit return, index is always (context & 0x7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct credit_return {
    pub cr: [volatile __le64; 8],
}

// NUMA indexed credit return array
#[repr(C)]
#[derive(Copy, Clone)]
pub struct credit_return_base {
    pub va: *mut credit_return,
    pub dma: dma_addr_t,
}

// send context configuration sizes (one per type)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc_config_sizes {
    pub size: short int,
    pub count: short int,
}

//
// The diagram below details the relationship of the mapping structures
//
// Since the mapping now allows for non-uniform send contexts per vl, the
// number of send contexts for a vl is either the vl_scontexts[vl] or
// a computation based on num_kernel_send_contexts/num_vls:
//
// For example:
// nactual = vl_scontexts ? vl_scontexts[vl] : num_kernel_send_contexts/num_vls
//
// n = roundup to next highest power of 2 using nactual
//
// In the case where there are num_kernel_send_contexts/num_vls doesn't divide
// evenly, the extras are added from the last vl downward.
//
// For the case where n > nactual, the send contexts are assigned
// in a round robin fashion wrapping back to the first send context
// for a particular vl.
//
// dd->pio_map
// |                                   pio_map_elem[0]
// |                                +--------------------+
// v                                |       mask         |
// pio_vl_map                            |--------------------|
// +--------------------------+                   | ksc[0] -> sc 1     |
// |    list (RCU)            |                   |--------------------|
// |--------------------------|                 ->| ksc[1] -> sc 2     |
// |    mask                  |              --/  |--------------------|
// |--------------------------|            -/     |        *           |
// |    actual_vls (max 8)    |          -/       |--------------------|
// |--------------------------|       --/         | ksc[n-1] -> sc n   |
// |    vls (max 8)           |     -/            +--------------------+
// |--------------------------|  --
// |    map[0]                |-
// |--------------------------|                   +--------------------+
// |    map[1]                |---                |       mask         |
// |--------------------------|   \----           |--------------------|
// |           *              |        \--        | ksc[0] -> sc 1+n   |
// |           *              |           \----   |--------------------|
// |           *              |                \->| ksc[1] -> sc 2+n   |
// |--------------------------|                   |--------------------|
// |   map[vls - 1]           |-                  |         *          |
// +--------------------------+ \-                |--------------------|
// \-              | ksc[m-1] -> sc m+n |
// \             +--------------------+
// \-
// \
// \-        +----------------------+
// \-      |       mask           |
// \     |----------------------|
// \-   | ksc[0] -> sc 1+m+n   |
// \- |----------------------|
// >| ksc[1] -> sc 2+m+n   |
// |----------------------|
// |         *            |
// |----------------------|
// | ksc[o-1] -> sc o+m+n |
// +----------------------+
//
// Initial number of send contexts per VL
pub const INIT_SC_PER_VL: c_int = 2;
//
// struct pio_map_elem - mapping for a vl
// @mask - selector mask
// @ksc - array of kernel send contexts for this vl
//
// The mask is used to "mod" the selector to
// produce index into the trailing array of
// kscs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio_map_elem {
    pub mask: u32,
    pub ksc: [*mut send_context; ],
}

//
// struct pio_vl_map - mapping for a vl
// @list - rcu head for free callback
// @mask - vl mask to "mod" the vl to produce an index to map array
// @actual_vls - number of vls
// @vls - numbers of vls rounded to next power of 2
// @map - array of pio_map_elem entries
//
// This is the parent mapping structure. The trailing members of the
// struct point to pio_map_elem entries, which in turn point to an
// array of kscs for that vl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio_vl_map {
    pub list: rcu_head,
    pub mask: u32,
    pub actual_vls: u8,
    pub vls: u8,
    pub map: [*mut pio_map_elem; ],
}

extern "C" {
    pub fn free_pio_map(dd: *mut hfi1_devdata);
}
// send context functions
extern "C" {
    pub fn init_credit_return(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn free_credit_return(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn init_sc_pools_and_sizes(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn init_send_contexts(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn init_pervl_scs(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn sc_free(sc: *mut send_context);
}
extern "C" {
    pub fn sc_enable(sc: *mut send_context) -> c_int;
}
extern "C" {
    pub fn sc_disable(sc: *mut send_context);
}
extern "C" {
    pub fn sc_restart(sc: *mut send_context) -> c_int;
}
extern "C" {
    pub fn sc_return_credits(sc: *mut send_context);
}
extern "C" {
    pub fn sc_flush(sc: *mut send_context);
}
extern "C" {
    pub fn sc_stop(sc: *mut send_context, bit: c_int);
}
extern "C" {
    pub fn sc_release_update(sc: *mut send_context);
}
extern "C" {
    pub fn sc_group_release_update(dd: *mut hfi1_devdata, hw_context: u32);
}
extern "C" {
    pub fn sc_add_credit_return_intr(sc: *mut send_context);
}
extern "C" {
    pub fn sc_del_credit_return_intr(sc: *mut send_context);
}
extern "C" {
    pub fn sc_set_cr_threshold(sc: *mut send_context, new_threshold: u32);
}
extern "C" {
    pub fn sc_percent_to_threshold(sc: *mut send_context, percent: u32) -> u32;
}
extern "C" {
    pub fn sc_mtu_to_threshold(sc: *mut send_context, mtu: u32, hdrqentsize: u32) -> u32;
}
extern "C" {
    pub fn hfi1_sc_wantpiobuf_intr(sc: *mut send_context, needint: u32);
}
extern "C" {
    pub fn sc_wait(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn set_pio_integrity(sc: *mut send_context);
}
// support functions
extern "C" {
    pub fn pio_reset_all(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn pio_freeze(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn pio_kernel_unfreeze(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn pio_kernel_linkup(dd: *mut hfi1_devdata);
}
// global PIO send control operations
pub const PSC_GLOBAL_ENABLE: c_int = 0;
pub const PSC_GLOBAL_DISABLE: c_int = 1;
pub const PSC_GLOBAL_VLARB_ENABLE: c_int = 2;
pub const PSC_GLOBAL_VLARB_DISABLE: c_int = 3;
pub const PSC_CM_RESET: c_int = 4;
pub const PSC_DATA_VL_ENABLE: c_int = 5;
pub const PSC_DATA_VL_DISABLE: c_int = 6;
extern "C" {
    pub fn __cm_reset(dd: *mut hfi1_devdata, sendctrl: u64);
}
extern "C" {
    pub fn pio_send_control(dd: *mut hfi1_devdata, op: c_int);
}
// PIO copy routines
extern "C" {
    pub fn seg_pio_copy_mid(pbuf: *mut pio_buf, from: *const c_void, nbytes: usize);
}
extern "C" {
    pub fn seg_pio_copy_end(pbuf: *mut pio_buf);
}
