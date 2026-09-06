//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/stp.h
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
// Copyright IBM Corp. 2006
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//

// notifier for syncs
// STP interruption parameter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_irq_parm {
    pub 14: u32 :,
    pub /: *mut *mut u32 tsc : 1; / Timing status change,
    pub /: *mut *mut u32 lac : 1; / Link availability change,
    pub /: *mut *mut u32 tcpc : 1; / Time control parameter change,
    pub 15: u32 :,
    pub __packed: },
pub const STP_OP_SYNC: c_int = 1;
pub const STP_OP_CTRL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_sstpi {
    pub 32: u32 :,
    pub 1: u32 tu :,
    pub 1: u32 lu :,
    pub 6: u32 :,
    pub 8: u32 stratum :,
    pub 16: u32 vbits :,
    pub 16: u32 leaps :,
    pub 4: u32 tmd :,
    pub 4: u32 ctn :,
    pub 3: u32 :,
    pub 1: u32 c :,
    pub 4: u32 tst :,
    pub 16: u32 tzo :,
    pub 16: u32 dsto :,
    pub 16: u32 ctrl :,
    pub 16: u32 :,
    pub tto: u32,
    pub 32: u32 :,
    pub ctnid: [u32; 3],
    pub 32: u32 :,
    pub todoff: u64,
    pub rsvd: [u32; 50],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_tzib {
    pub 16: u32 tzan :,
    pub 16: u32 :,
    pub 16: u32 tzo :,
    pub 16: u32 dsto :,
    pub stn: u32,
    pub dstn: u32,
    pub dst_on_alg: u64,
    pub dst_off_alg: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_tcpib {
    pub 4: u32 atcode :,
    pub 4: u32 ntcode :,
    pub 1: u32 d :,
    pub 23: u32 :,
    pub tto: i32,
    pub atzib: stp_tzib,
    pub ntzib: stp_tzib,
    pub 16: s32 adst_offset :,
    pub 16: s32 ndst_offset :,
    pub rsvd1: u32,
    pub ntzib_update: u64,
    pub ndsto_update: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_lsoib {
    pub 1: u32 p :,
    pub 31: u32 :,
    pub 16: s32 also :,
    pub 16: s32 nlso :,
    pub nlsout: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_stzi {
    pub rsvd0: [u32; 3],
    pub data_ts: u64,
    pub rsvd1: [u32; 22],
    pub tcpib: stp_tcpib,
    pub lsoib: stp_lsoib,
    pub __packed: },
// Functions needed by the machine check handler
    pub stp_sync_check(void): c_int,
    pub stp_island_check(void): c_int,
    pub stp_queue_work(void): c_void,
    pub stp_enabled(void): bool,
