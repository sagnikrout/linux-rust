//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/sw/rdmavt/rc.c
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
// Copyright(c) 2016 Intel Corporation.
//

//
// Convert the AETH credit code into the number of credits.
//
    static const u16 credit_table[31] = {
    0,                      /* 0 */
    1,                      /* 1 */
    2,                      /* 2 */
    3,                      /* 3 */
    4,                      /* 4 */
    6,                      /* 5 */
    8,                      /* 6 */
    12,                     /* 7 */
    16,                     /* 8 */
    24,                     /* 9 */
    32,                     /* A */
    48,                     /* B */
    64,                     /* C */
    96,                     /* D */
    128,                    /* E */
    192,                    /* F */
    256,                    /* 10 */
    384,                    /* 11 */
    512,                    /* 12 */
    768,                    /* 13 */
    1024,                   /* 14 */
    1536,                   /* 15 */
    2048,                   /* 16 */
    3072,                   /* 17 */
    4096,                   /* 18 */
    6144,                   /* 19 */
    8192,                   /* 1A */
    12288,                  /* 1B */
    16384,                  /* 1C */
    24576,                  /* 1D */
    32768                   /* 1E */
    };
//
// rvt_compute_aeth - compute the AETH (syndrome + MSN)
// @qp: the queue pair to compute the AETH for
//
// Returns the AETH.
//
#[no_mangle]
pub unsafe extern "C" fn rvt_compute_aeth(qp: *mut rvt_qp) -> __be32 {
    __be32 rvt_compute_aeth(struct rvt_qp *qp)
    {
    let mut aeth: u32 = qp.r_msn & IB_MSN_MASK;
    if (qp.ibqp.srq) {
//
// Shared receive queues don't generate credits.
// Set the credit field to the invalid value.
//
    aeth |= IB_AETH_CREDIT_INVAL << IB_AETH_CREDIT_SHIFT;
    } else {
    u32 min, max, x;
    u32 credits;
    u32 head;
    u32 tail;
    credits = READ_ONCE(qp.r_rq.kwq.count);
    if (credits == 0) {
// sanity check pointers before trusting them
    if (qp.ip) {
    head = RDMA_READ_UAPI_ATOMIC(qp.r_rq.wq.head);
    tail = RDMA_READ_UAPI_ATOMIC(qp.r_rq.wq.tail);
    } else {
    head = READ_ONCE(qp.r_rq.kwq.head);
    tail = READ_ONCE(qp.r_rq.kwq.tail);
    }
    if (head >= qp.r_rq.size)
    head = 0;
    if (tail >= qp.r_rq.size)
    tail = 0;
//
// Compute the number of credits available (RWQEs).
// There is a small chance that the pair of reads are
// not atomic, which is OK, since the fuzziness is
// resolved as further ACKs go out.
//
    credits = rvt_get_rq_count(&qp.r_rq, head, tail);
    }
//
// Binary search the credit table to find the code to
// use.
//
    min = 0;
    max = 31;
    for (;;) {
    x = (min + max) / 2;
    if (credit_table[x] == credits)
    break;
    if (credit_table[x] > credits) {
    max = x;
    } else {
    if (min == x)
    break;
    min = x;
    }
    }
    aeth |= x << IB_AETH_CREDIT_SHIFT;
    }
    return cpu_to_be32(aeth);
    }
    EXPORT_SYMBOL(rvt_compute_aeth);
//
// rvt_get_credit - flush the send work queue of a QP
// @qp: the qp who's send work queue to flush
// @aeth: the Acknowledge Extended Transport Header
//
// The QP s_lock should be held.
//
#[no_mangle]
pub unsafe extern "C" fn rvt_get_credit(qp: *mut rvt_qp, aeth: u32) {
    void rvt_get_credit(struct rvt_qp *qp, u32 aeth)
    {
    struct rvt_dev_info *rdi = ib_to_rvt(qp.ibqp.device);
    let mut credit: u32 = (aeth >> IB_AETH_CREDIT_SHIFT) & IB_AETH_CREDIT_MASK;
    lockdep_assert_held(&qp.s_lock);
//
// If the credit is invalid, we can send
// as many packets as we like.  Otherwise, we have to
// honor the credit field.
//
    if (credit == IB_AETH_CREDIT_INVAL) {
    if (!(qp.s_flags & RVT_S_UNLIMITED_CREDIT)) {
    qp.s_flags |= RVT_S_UNLIMITED_CREDIT;
    if (qp.s_flags & RVT_S_WAIT_SSN_CREDIT) {
    qp.s_flags &= ~RVT_S_WAIT_SSN_CREDIT;
    rdi.driver_f.schedule_send(qp);
    }
    }
    } else if (!(qp.s_flags & RVT_S_UNLIMITED_CREDIT)) {
// Compute new LSN (i.e., MSN + credit)
    credit = (aeth + credit_table[credit]) & IB_MSN_MASK;
    if (rvt_cmp_msn(credit, qp.s_lsn) > 0) {
    qp.s_lsn = credit;
    if (qp.s_flags & RVT_S_WAIT_SSN_CREDIT) {
    qp.s_flags &= ~RVT_S_WAIT_SSN_CREDIT;
    rdi.driver_f.schedule_send(qp);
    }
    }
    }
    }
    EXPORT_SYMBOL(rvt_get_credit);
//
// rvt_restart_sge - rewind the sge state for a wqe
// @ss: the sge state pointer
// @wqe: the wqe to rewind
// @len: the data length from the start of the wqe in bytes
//
// Returns the remaining data length.
//
#[no_mangle]
pub unsafe extern "C" fn rvt_restart_sge(ss: *mut rvt_sge_state, wqe: *mut rvt_swqe, len: u32) -> u32 {
    u32 rvt_restart_sge(struct rvt_sge_state *ss, struct rvt_swqe *wqe, u32 len)
    {
    ss.sge = wqe.sg_list[0];
    ss.sg_list = wqe.sg_list + 1;
    ss.num_sge = wqe.wr.num_sge;
    ss.total_len = wqe.length;
    rvt_skip_sge(ss, len, false);
    return wqe.length - len;
    }
    EXPORT_SYMBOL(rvt_restart_sge);
