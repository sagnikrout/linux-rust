//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/macsec/MSS_Egress_registers.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//

// Macro flag: #define MSS_EGRESS_REGS_HEADER
pub const MSS_EGRESS_CTL_REGISTER_ADDR: c_uint = 0x00005002;
pub const MSS_EGRESS_SA_EXPIRED_STATUS_REGISTER_ADDR: c_uint = 0x00005060;
pub const MSS_EGRESS_SA_THRESHOLD_EXPIRED_STATUS_REGISTER_ADDR: c_uint = 0x00005062;
pub const MSS_EGRESS_LUT_ADDR_CTL_REGISTER_ADDR: c_uint = 0x00005080;
pub const MSS_EGRESS_LUT_CTL_REGISTER_ADDR: c_uint = 0x00005081;
pub const MSS_EGRESS_LUT_DATA_CTL_REGISTER_ADDR: c_uint = 0x000050A0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mss_egress_ctl_register {
    pub 1: unsigned int soft_reset :,
    pub 1: unsigned int drop_kay_packet :,
    pub 1: unsigned int drop_egprc_lut_miss :,
    pub 1: unsigned int gcm_start :,
    pub 1: unsigned int gcm_test_mode :,
    pub 1: unsigned int unmatched_use_sc_0 :,
    pub 1: unsigned int drop_invalid_sa_sc_packets :,
    pub 1: unsigned int reserved0 :,
// Should always be set to 0.
    pub 1: unsigned int external_classification_enable :,
    pub 1: unsigned int icv_lsb_8bytes_enable :,
    pub 1: unsigned int high_prio :,
    pub 1: unsigned int clear_counter :,
    pub 1: unsigned int clear_global_time :,
    pub 3: unsigned int ethertype_explicit_sectag_lsb :,
    pub bits_0: },
    pub word_0: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mss_egress_lut_addr_ctl_register {
    pub 9: unsigned int lut_addr :,
    pub 3: unsigned int reserved0 :,
// 0x0 : Egress MAC Control FIlter (CTLF) LUT
// 0x1 : Egress Classification LUT
// 0x2 : Egress SC/SA LUT
// 0x3 : Egress SMIB
//
    pub 4: unsigned int lut_select :,
    pub bits_0: },
    pub word_0: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mss_egress_lut_ctl_register {
    pub 14: unsigned int reserved0 :,
    pub 1: unsigned int lut_read :,
    pub 1: unsigned int lut_write :,
    pub bits_0: },
    pub word_0: c_ushort,
}
