//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/verbs_txreq.h
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
// Copyright(c) 2016 - 2018 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct verbs_txreq {
    pub phdr: hfi1_sdma_header,
    pub txreq: sdma_txreq,
    pub qp: *mut rvt_qp,
    pub wqe: *mut rvt_swqe,
    pub mr: *mut rvt_mregion,
    pub ss: *mut rvt_sge_state,
    pub sde: *mut sdma_engine,
    pub psc: *mut send_context,
    pub hdr_dwords: u16,
    pub s_cur_size: u16,
}

// call slow path to get the lock
// so that we can test if the sdma descriptors are there
// Set the header type
extern "C" {
    pub fn container_of(_arg: stx, verbs_txreq: struct, _arg: txreq) -> return;
}
extern "C" {
    pub fn iowait_packet_queued(_arg: w) -> return;
}
extern "C" {
    pub fn hfi1_put_txreq(tx: *mut verbs_txreq);
}
extern "C" {
    pub fn verbs_txreq_init(dev: *mut hfi1_ibdev) -> c_int;
}
extern "C" {
    pub fn verbs_txreq_exit(dev: *mut hfi1_ibdev);
}
