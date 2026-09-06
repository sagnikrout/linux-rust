//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/macsec/MSS_Ingress_registers.h
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

// Macro flag: #define MSS_INGRESS_REGS_HEADER
pub const MSS_INGRESS_CTL_REGISTER_ADDR: c_uint = 0x0000800E;
pub const MSS_INGRESS_LUT_ADDR_CTL_REGISTER_ADDR: c_uint = 0x00008080;
pub const MSS_INGRESS_LUT_CTL_REGISTER_ADDR: c_uint = 0x00008081;
pub const MSS_INGRESS_LUT_DATA_CTL_REGISTER_ADDR: c_uint = 0x000080A0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mss_ingress_ctl_register {
    pub 1: unsigned int soft_reset :,
    pub 1: unsigned int operation_point_to_point :,
    pub 1: unsigned int create_sci :,
// Unused
    pub 1: unsigned int mask_short_length_error :,
    pub 1: unsigned int drop_kay_packet :,
    pub 1: unsigned int drop_igprc_miss :,
// Unused
    pub 1: unsigned int check_icv :,
    pub 1: unsigned int clear_global_time :,
    pub 1: unsigned int clear_count :,
    pub 1: unsigned int high_prio :,
    pub 1: unsigned int remove_sectag :,
    pub 2: unsigned int global_validate_frames :,
    pub 1: unsigned int icv_lsb_8bytes_enabled :,
    pub 2: unsigned int reserved0 :,
    pub bits_0: },
    pub word_0: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mss_ingress_lut_addr_ctl_register {
    pub 9: unsigned int lut_addr :,
    pub 3: unsigned int reserved0 :,
// 0x0 : Ingress Pre-Security MAC Control FIlter
// (IGPRCTLF) LUT
// 0x1 : Ingress Pre-Security Classification LUT (IGPRC)
// 0x2 : Ingress Packet Format (IGPFMT) SAKey LUT
// 0x3 : Ingress Packet Format (IGPFMT) SC/SA LUT
// 0x4 : Ingress Post-Security Classification LUT
// (IGPOC)
// 0x5 : Ingress Post-Security MAC Control Filter
// (IGPOCTLF) LUT
// 0x6 : Ingress MIB (IGMIB)
//
    pub 4: unsigned int lut_select :,
    pub bits_0: },
    pub word_0: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mss_ingress_lut_ctl_register {
    pub 14: unsigned int reserved0 :,
    pub 1: unsigned int lut_read :,
    pub 1: unsigned int lut_write :,
    pub bits_0: },
    pub word_0: c_ushort,
}
