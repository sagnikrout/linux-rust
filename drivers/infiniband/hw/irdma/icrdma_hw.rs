//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/icrdma_hw.h
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
// Copyright (c) 2017 - 2021 Intel Corporation

pub const VFPE_CQPTAIL1: c_uint = 0x0000a000;
pub const VFPE_CQPDB1: c_uint = 0x0000bc00;
pub const VFPE_CCQPSTATUS1: c_uint = 0x0000b800;
pub const VFPE_CCQPHIGH1: c_uint = 0x00009800;
pub const VFPE_CCQPLOW1: c_uint = 0x0000ac00;
pub const VFPE_CQARM1: c_uint = 0x0000b400;
pub const VFPE_CQARM1: c_uint = 0x0000b400;
pub const VFPE_CQACK1: c_uint = 0x0000b000;
pub const VFPE_AEQALLOC1: c_uint = 0x0000a400;
pub const VFPE_CQPERRCODES1: c_uint = 0x00009c00;
pub const VFPE_WQEALLOC1: c_uint = 0x0000c000;

pub const PFPE_CQPTAIL: c_uint = 0x00500880;
pub const PFPE_CQPDB: c_uint = 0x00500800;
pub const PFPE_CCQPSTATUS: c_uint = 0x0050a000;
pub const PFPE_CCQPHIGH: c_uint = 0x0050a100;
pub const PFPE_CCQPLOW: c_uint = 0x0050a080;
pub const PFPE_CQARM: c_uint = 0x00502c00;
pub const PFPE_CQACK: c_uint = 0x00502c80;
pub const PFPE_AEQALLOC: c_uint = 0x00502d00;

pub const GLPCI_LBARCTRL: c_uint = 0x0009de74;
pub const GLPE_CPUSTATUS0: c_uint = 0x0050ba5c;
pub const GLPE_CPUSTATUS1: c_uint = 0x0050ba60;
pub const GLPE_CPUSTATUS2: c_uint = 0x0050ba64;
pub const PFINT_AEQCTL: c_uint = 0x0016cb00;
pub const PFPE_CQPERRCODES: c_uint = 0x0050a200;
pub const PFPE_WQEALLOC: c_uint = 0x00504400;

pub const PFHMC_PDINV: c_uint = 0x00520300;

pub const GLPE_CRITERR: c_uint = 0x00534000;

pub const PFHMC_ERRORINFO: c_uint = 0x00520400;
pub const PFHMC_ERRORDATA: c_uint = 0x00520500;

// shifts/masks for FLD_[LS/RS]_64 macros used in device table
pub const ICRDMA_CCQPSTATUS_CCQP_DONE_S: c_int = 0;

pub const ICRDMA_CCQPSTATUS_CCQP_ERR_S: c_int = 31;

pub const ICRDMA_CQPSQ_STAG_PDID_S: c_int = 46;

pub const ICRDMA_CQPSQ_CQ_CEQID_S: c_int = 22;

pub const ICRDMA_CQPSQ_CQ_CQID_S: c_int = 0;

pub const ICRDMA_COMMIT_FPM_CQCNT_S: c_int = 0;

pub const ICRDMA_CQPSQ_UPESD_HMCFNID_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icrdma_device_caps_const {
    ICRDMA_MAX_STATS_COUNT = 128,

    ICRDMA_MAX_IRD_SIZE			= 127,
    ICRDMA_MAX_ORD_SIZE			= 255,
    ICRDMA_MIN_WQ_SIZE                      = 8 /* WQEs */,
    ICRDMA_MAX_PUSH_PAGE_COUNT		= 256,
}

extern "C" {
    pub fn icrdma_init_hw(dev: *mut irdma_sc_dev);
}
