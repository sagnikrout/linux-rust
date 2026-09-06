//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/tp.h
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
// $Date: 2005/03/07 23:59:05 $ $RCSfile: tp.h,v $ $Revision: 1.20 $

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_mib_statistics {
// IP
    pub ipInReceive_hi: u32,
    pub ipInReceive_lo: u32,
    pub ipInHdrErrors_hi: u32,
    pub ipInHdrErrors_lo: u32,
    pub ipInAddrErrors_hi: u32,
    pub ipInAddrErrors_lo: u32,
    pub ipInUnknownProtos_hi: u32,
    pub ipInUnknownProtos_lo: u32,
    pub ipInDiscards_hi: u32,
    pub ipInDiscards_lo: u32,
    pub ipInDelivers_hi: u32,
    pub ipInDelivers_lo: u32,
    pub ipOutRequests_hi: u32,
    pub ipOutRequests_lo: u32,
    pub ipOutDiscards_hi: u32,
    pub ipOutDiscards_lo: u32,
    pub ipOutNoRoutes_hi: u32,
    pub ipOutNoRoutes_lo: u32,
    pub ipReasmTimeout: u32,
    pub ipReasmReqds: u32,
    pub ipReasmOKs: u32,
    pub ipReasmFails: u32,
    pub reserved: [u32; 8],
// TCP
    pub tcpActiveOpens: u32,
    pub tcpPassiveOpens: u32,
    pub tcpAttemptFails: u32,
    pub tcpEstabResets: u32,
    pub tcpOutRsts: u32,
    pub tcpCurrEstab: u32,
    pub tcpInSegs_hi: u32,
    pub tcpInSegs_lo: u32,
    pub tcpOutSegs_hi: u32,
    pub tcpOutSegs_lo: u32,
    pub tcpRetransSeg_hi: u32,
    pub tcpRetransSeg_lo: u32,
    pub tcpInErrs_hi: u32,
    pub tcpInErrs_lo: u32,
    pub tcpRtoMin: u32,
    pub tcpRtoMax: u32,
}

extern "C" {
    pub fn t1_tp_destroy(tp: *mut petp);
}
extern "C" {
    pub fn t1_tp_intr_disable(tp: *mut petp);
}
extern "C" {
    pub fn t1_tp_intr_enable(tp: *mut petp);
}
extern "C" {
    pub fn t1_tp_intr_clear(tp: *mut petp);
}
extern "C" {
    pub fn t1_tp_intr_handler(tp: *mut petp) -> c_int;
}
extern "C" {
    pub fn t1_tp_set_tcp_checksum_offload(tp: *mut petp, enable: c_int);
}
extern "C" {
    pub fn t1_tp_set_ip_checksum_offload(tp: *mut petp, enable: c_int);
}
extern "C" {
    pub fn t1_tp_reset(tp: *mut petp, p: *mut tp_params, tp_clk: c_uint) -> c_int;
}
