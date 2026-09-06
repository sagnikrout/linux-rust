//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/qp.h
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
// Copyright(c) 2015 - 2018 Intel Corporation.
//

//
// Driver specific s_flags starting at bit 31 down to HFI1_S_MIN_BIT_MASK
//
// HFI1_S_AHG_VALID - ahg header valid on chip
// HFI1_S_AHG_CLEAR - have send engine clear ahg state
// HFI1_S_WAIT_PIO_DRAIN - qp waiting for PIOs to drain
// HFI1_S_WAIT_TID_SPACE - a QP is waiting for TID resource
// HFI1_S_WAIT_TID_RESP - waiting for a TID RDMA WRITE response
// HFI1_S_WAIT_HALT - halt the first leg send engine
// HFI1_S_MIN_BIT_MASK - the lowest bit that can be used by hfi1
//
pub const HFI1_S_AHG_VALID: c_uint = 0x80000000;
pub const HFI1_S_AHG_CLEAR: c_uint = 0x40000000;
pub const HFI1_S_WAIT_PIO_DRAIN: c_uint = 0x20000000;
pub const HFI1_S_WAIT_TID_SPACE: c_uint = 0x10000000;
pub const HFI1_S_WAIT_TID_RESP: c_uint = 0x08000000;
pub const HFI1_S_WAIT_HALT: c_uint = 0x04000000;
pub const HFI1_S_MIN_BIT_MASK: c_uint = 0x01000000;
//
// overload wait defines
//

//
// Send if not busy or waiting for I/O and either
// a RC response is pending or we can process send work requests.
//
// free_ahg - clear ahg from QP
//
// hfi1_qp_wakeup - wake up on the indicated event
// @qp: the QP
// @flag: flag the qp on which the qp is stalled
//
extern "C" {
    pub fn hfi1_qp_wakeup(qp: *mut rvt_qp, flag: u32);
}
extern "C" {
    pub fn qp_iter_print(s: *mut seq_file, iter: *mut rvt_qp_iter);
}
extern "C" {
    pub fn _hfi1_schedule_send(qp: *mut rvt_qp) -> bool;
}
extern "C" {
    pub fn hfi1_schedule_send(qp: *mut rvt_qp) -> bool;
}
extern "C" {
    pub fn hfi1_migrate_qp(qp: *mut rvt_qp);
}
//
// Functions provided by hfi1 driver for rdmavt to use
//
extern "C" {
    pub fn qp_priv_free(rdi: *mut rvt_dev_info, qp: *mut rvt_qp);
}
extern "C" {
    pub fn free_all_qps(rdi: *mut rvt_dev_info) -> unsigned;
}
extern "C" {
    pub fn notify_qp_reset(qp: *mut rvt_qp);
}
extern "C" {
    pub fn flush_qp_waiters(qp: *mut rvt_qp);
}
extern "C" {
    pub fn notify_error_qp(qp: *mut rvt_qp);
}
extern "C" {
    pub fn stop_send_queue(qp: *mut rvt_qp);
}
extern "C" {
    pub fn quiesce_qp(qp: *mut rvt_qp);
}
extern "C" {
    pub fn mtu_from_qp(rdi: *mut rvt_dev_info, qp: *mut rvt_qp, pmtu: u32) -> u32;
}
extern "C" {
    pub fn mtu_to_path_mtu(mtu: u32) -> c_int;
}
extern "C" {
    pub fn hfi1_error_port_qps(ibp: *mut hfi1_ibport, sl: u8);
}
extern "C" {
    pub fn hfi1_qp_unbusy(qp: *mut rvt_qp, wait: *mut iowait_work);
}
