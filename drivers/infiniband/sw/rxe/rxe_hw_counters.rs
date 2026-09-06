//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_hw_counters.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2017 Mellanox Technologies Ltd. All rights reserved.
//
// when adding counters to enum also add
// them to rxe_counter_name[] vector.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_counters {
    RXE_CNT_SENT_PKTS,
    RXE_CNT_RCVD_PKTS,
    RXE_CNT_DUP_REQ,
    RXE_CNT_OUT_OF_SEQ_REQ,
    RXE_CNT_RCV_RNR,
    RXE_CNT_SND_RNR,
    RXE_CNT_RCV_SEQ_ERR,
    RXE_CNT_SENDER_SCHED,
    RXE_CNT_RETRY_EXCEEDED,
    RXE_CNT_RNR_RETRY_EXCEEDED,
    RXE_CNT_COMP_RETRY,
    RXE_CNT_SEND_ERR,
    RXE_CNT_LINK_DOWNED,
    RXE_CNT_RDMA_SEND,
    RXE_CNT_RDMA_RECV,
    RXE_CNT_SENT_BYTES,
    RXE_CNT_RCVD_BYTES,
    RXE_NUM_OF_COUNTERS
}
