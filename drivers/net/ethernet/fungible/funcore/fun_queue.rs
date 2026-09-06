//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/fungible/funcore/fun_queue.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_rq_info {
    pub dma: dma_addr_t,
    pub page: *mut page,
}

// A queue group consisting of an SQ, a CQ, and an optional RQ.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_queue {
    pub fdev: *mut fun_dev,
    pub sq_lock: spinlock_t,
    pub cq_dma_addr: dma_addr_t,
    pub sq_dma_addr: dma_addr_t,
    pub rq_dma_addr: dma_addr_t,
    pub cq_db: *mut u32 __iomem,
    pub sq_db: *mut u32 __iomem,
    pub rq_db: *mut u32 __iomem,
    pub cqes: *mut c_void,
    pub sq_cmds: *mut c_void,
    pub rqes: *mut fun_eprq_rqbuf,
    pub rq_info: *mut fun_rq_info,
    pub cqid: u32,
    pub sqid: u32,
    pub rqid: u32,
    pub cq_depth: u32,
    pub sq_depth: u32,
    pub rq_depth: u32,
    pub cq_head: u16,
    pub sq_tail: u16,
    pub rq_tail: u16,
    pub cqe_size_log2: u8,
    pub sqe_size_log2: u8,
    pub cqe_info_offset: u16,
    pub rq_buf_idx: u16,
    pub rq_buf_offset: c_int,
    pub num_rqe_to_fill: u16,
    pub cq_intcoal_usec: u8,
    pub cq_intcoal_nentries: u8,
    pub sq_intcoal_usec: u8,
    pub sq_intcoal_nentries: u8,
    pub cq_flags: u16,
    pub sq_flags: u16,
    pub rq_flags: u16,
// SQ head writeback
    pub sq_comp: u16,
    pub sq_head: *mut volatile __be64,
    pub cq_cb: cq_callback_t,
    pub cb_data: *mut c_void,
    pub irq_handler: irq_handler_t,
    pub irq_data: *mut c_void,
    pub cq_vector: i16,
    pub cq_phase: u8,
// I/O q index
    pub qid: u16,
    pub irqname: [c_char; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_queue_alloc_req {
    pub cqe_size_log2: u8,
    pub sqe_size_log2: u8,
    pub cq_flags: u16,
    pub sq_flags: u16,
    pub rq_flags: u16,
    pub cq_depth: u32,
    pub sq_depth: u32,
    pub rq_depth: u32,
    pub cq_intcoal_usec: u8,
    pub cq_intcoal_nentries: u8,
    pub sq_intcoal_usec: u8,
    pub sq_intcoal_nentries: u8,
}

extern "C" {
    pub fn fun_free_queue(funq: *mut fun_queue);
}
extern "C" {
    pub fn fun_create_rq(funq: *mut fun_queue) -> c_int;
}
extern "C" {
    pub fn fun_free_irq(funq: *mut fun_queue);
}
extern "C" {
    pub fn __fun_process_cq(funq: *mut fun_queue, max: c_uint) -> c_uint;
}
extern "C" {
    pub fn fun_process_cq(funq: *mut fun_queue, max: c_uint) -> c_uint;
}
