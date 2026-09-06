//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mthca/mthca_provider.h
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


//
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
// Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_buf_list {
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mthca_buf {
    pub direct: mthca_buf_list,
    pub page_list: *mut mthca_buf_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_uar {
    pub pfn: c_ulong,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_ucontext {
    pub ibucontext: ib_ucontext,
    pub uar: mthca_uar,
    pub db_tab: *mut mthca_user_db_table,
    pub reg_mr_warned: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub mtt: *mut mthca_mtt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_pd {
    pub ibpd: ib_pd,
    pub pd_num: u32,
    pub sqp_count: core::sync::atomic::AtomicI32,
    pub ntmr: mthca_mr,
    pub privileged: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_eq {
    pub dev: *mut mthca_dev,
    pub eqn: c_int,
    pub eqn_mask: u32,
    pub cons_index: u32,
    pub msi_x_vector: u16,
    pub msi_x_entry: u16,
    pub have_irq: c_int,
    pub nent: c_int,
    pub page_list: *mut mthca_buf_list,
    pub mr: mthca_mr,
    pub irq_name: [c_char; IB_DEVICE_NAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mthca_ah_type {
    MTHCA_AH_ON_HCA,
    MTHCA_AH_PCI_POOL,
    MTHCA_AH_KMALLOC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_ah {
    pub ibah: ib_ah,
    pub type: mthca_ah_type,
    pub key: u32,
    pub av: *mut mthca_av,
    pub avdma: dma_addr_t,
}

//
// Quick description of our CQ/QP locking scheme:
//
// We have one global lock that protects dev->cq/qp_table.  Each
// struct mthca_cq/qp also has its own lock.  An individual qp lock
// may be taken inside of an individual cq lock.  Both cqs attached to
// a qp may be locked, with the cq with the lower cqn locked first.
// No other nesting should be done.
//
// Each struct mthca_cq/qp also has an ref count, protected by the
// corresponding table lock.  The pointer from the cq/qp_table to the
// struct counts as one reference.  This reference also is good for
// access through the consumer API, so modifying the CQ/QP etc doesn't
// need to take another reference.  Access to a QP because of a
// completion being polled does not need a reference either.
//
// Finally, each struct mthca_cq/qp has a wait_queue_head_t for the
// destroy function to sleep on.
//
// This means that access from the consumer API requires nothing but
// taking the struct's lock.
//
// Access because of a completion event should go as follows:
// - lock cq/qp_table and look up struct
// - increment ref count in struct
// - drop cq/qp_table lock
// - lock struct, do your thing, and unlock struct
// - decrement ref count; if zero, wake up waiters
//
// To destroy a CQ/QP, we can do the following:
// - lock cq/qp_table
// - remove pointer and decrement ref count
// - unlock cq/qp_table lock
// - wait_event until ref count is zero
//
// It is the consumer's responsibilty to make sure that no QP
// operations (WQE posting or state modification) are pending when a
// QP is destroyed.  Also, the consumer must make sure that calls to
// qp_modify are serialized.  Similarly, the consumer is responsible
// for ensuring that no CQ resize operations are pending when a CQ
// is destroyed.
//
// Possible optimizations (wait for profile data to see if/where we
// have locks bouncing between CPUs):
// - split cq/qp table lock into n separate (cache-aligned) locks,
// indexed (say) by the page in the table
// - split QP struct lock into three (one for common info, one for the
// send queue and one for the receive queue)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_cq_buf {
    pub queue: mthca_buf,
    pub mr: mthca_mr,
    pub is_direct: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_cq_resize {
    pub buf: mthca_cq_buf,
    pub cqe: c_int,
    pub state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_cq {
    pub ibcq: ib_cq,
    pub lock: spinlock_t,
    pub refcount: c_int,
    pub cqn: c_int,
    pub cons_index: u32,
    pub buf: mthca_cq_buf,
    pub resize_buf: *mut mthca_cq_resize,
    pub is_kernel: c_int,
// Next fields are Arbel only
    pub set_ci_db_index: c_int,
    pub set_ci_db: *mut __be32,
    pub arm_db_index: c_int,
    pub arm_db: *mut __be32,
    pub arm_sn: c_int,
    pub wait: wait_queue_head_t,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_srq {
    pub ibsrq: ib_srq,
    pub lock: spinlock_t,
    pub refcount: c_int,
    pub srqn: c_int,
    pub max: c_int,
    pub max_gs: c_int,
    pub wqe_shift: c_int,
    pub first_free: c_int,
    pub last_free: c_int,
    pub /: *mut *mut u16 counter; / Arbel only,
    pub /: *mut *mut int db_index; / Arbel only,
    pub /: *mut *mut *mut __be32 db; / Arbel only,
    pub last: *mut c_void,
    pub is_direct: c_int,
    pub wrid: *mut u64,
    pub queue: mthca_buf,
    pub mr: mthca_mr,
    pub wait: wait_queue_head_t,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_wq {
    pub lock: spinlock_t,
    pub max: c_int,
    pub next_ind: unsigned,
    pub last_comp: unsigned,
    pub head: unsigned,
    pub tail: unsigned,
    pub last: *mut c_void,
    pub max_gs: c_int,
    pub wqe_shift: c_int,
    pub /: *mut *mut int db_index; / Arbel only,
    pub db: *mut __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_sqp {
    pub pkey_index: c_int,
    pub qkey: u32,
    pub send_psn: u32,
    pub ud_header: ib_ud_header,
    pub header_buf_size: c_int,
    pub header_buf: *mut c_void,
    pub header_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_qp {
    pub ibqp: ib_qp,
    pub refcount: c_int,
    pub qpn: u32,
    pub is_direct: c_int,
    pub /: *mut *mut u8 port; / for SQP and memfree use only,
    pub /: *mut *mut u8 alt_port; / for memfree use only,
    pub transport: u8,
    pub state: u8,
    pub atomic_rd_en: u8,
    pub resp_depth: u8,
    pub mr: mthca_mr,
    pub rq: mthca_wq,
    pub sq: mthca_wq,
    pub sq_policy: ib_sig_type,
    pub send_wqe_offset: c_int,
    pub max_inline_data: c_int,
    pub wrid: *mut u64,
    pub queue: mthca_buf,
    pub wait: wait_queue_head_t,
    pub mutex: mutex,
    pub sqp: *mut mthca_sqp,
}

extern "C" {
    pub fn container_of(_arg: ibucontext, mthca_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, mthca_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, mthca_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, mthca_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, mthca_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, mthca_srq: struct, _arg: ibsrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, mthca_qp: struct, _arg: ibqp) -> return;
}
