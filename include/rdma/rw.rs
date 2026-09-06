//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rw.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2016 HGST, a Western Digital Company.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_rw_ctx {
// number of RDMA READ/WRITE WRs (not counting MR WRs)
    pub nr_ops: u32,
// tag for the union below:
    pub type: u8,
// for mapping a single SGE:
    pub sge: ib_sge,
    pub wr: ib_rdma_wr,
    pub single: },
// for mapping of multiple SGEs:
    pub sges: *mut ib_sge,
    pub wrs: *mut ib_rdma_wr,
    pub map: },
// for IOVA-based mapping of bvecs into contiguous DMA range:
    pub state: dma_iova_state,
    pub sge: ib_sge,
    pub wr: ib_rdma_wr,
    pub mapped_len: usize,
    pub iova: },
// for registering multiple WRs:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_rw_reg_ctx {
    pub sge: ib_sge,
    pub wr: ib_rdma_wr,
    pub reg_wr: ib_reg_wr,
    pub inv_wr: ib_send_wr,
    pub mr: *mut ib_mr,
    pub sgt: sg_table,
    pub reg: *mut },
}

extern "C" {
    pub fn rdma_rw_init_qp(dev: *mut ib_device, attr: *mut ib_qp_init_attr);
}
extern "C" {
    pub fn rdma_rw_init_mrs(qp: *mut ib_qp, attr: *mut ib_qp_init_attr) -> c_int;
}
extern "C" {
    pub fn rdma_rw_cleanup_mrs(qp: *mut ib_qp);
}
