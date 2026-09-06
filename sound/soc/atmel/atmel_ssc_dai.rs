//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/atmel/atmel_ssc_dai.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// atmel_ssc_dai.h - ALSA SSC interface for the Atmel  SoC
//
// Copyright (C) 2005 SAN People
// Copyright (C) 2008 Atmel
//
// Author: Sedji Gaouaou <sedji.gaouaou@atmel.com>
// ATMEL CORP.
//
// Based on at91-ssc.c by
// Frank Mandarino <fmandarino@endrelia.com>
// Based on pxa2xx Platform drivers by
// Liam Girdwood <lrg@slimlogic.co.uk>
//

// SSC system clock ids

// SSC divider ids

//
// SSC direction masks
//
pub const SSC_DIR_MASK_UNUSED: c_int = 0;
pub const SSC_DIR_MASK_PLAYBACK: c_int = 1;
pub const SSC_DIR_MASK_CAPTURE: c_int = 2;
//
// SSC register values that Atmel left out of <linux/atmel-ssc.h>.  These
// are expected to be used with SSC_BF
//
// START bit field values
pub const SSC_START_CONTINUOUS: c_int = 0;
pub const SSC_START_TX_RX: c_int = 1;
pub const SSC_START_LOW_RF: c_int = 2;
pub const SSC_START_HIGH_RF: c_int = 3;
pub const SSC_START_FALLING_RF: c_int = 4;
pub const SSC_START_RISING_RF: c_int = 5;
pub const SSC_START_LEVEL_RF: c_int = 6;
pub const SSC_START_EDGE_RF: c_int = 7;
pub const SSS_START_COMPARE_0: c_int = 8;
// CKI bit field values
pub const SSC_CKI_FALLING: c_int = 0;
pub const SSC_CKI_RISING: c_int = 1;
// CKO bit field values
pub const SSC_CKO_NONE: c_int = 0;
pub const SSC_CKO_CONTINUOUS: c_int = 1;
pub const SSC_CKO_TRANSFER: c_int = 2;
// CKS bit field values
pub const SSC_CKS_DIV: c_int = 0;
pub const SSC_CKS_CLOCK: c_int = 1;
pub const SSC_CKS_PIN: c_int = 2;
// FSEDGE bit field values
pub const SSC_FSEDGE_POSITIVE: c_int = 0;
pub const SSC_FSEDGE_NEGATIVE: c_int = 1;
// FSOS bit field values
pub const SSC_FSOS_NONE: c_int = 0;
pub const SSC_FSOS_NEGATIVE: c_int = 1;
pub const SSC_FSOS_POSITIVE: c_int = 2;
pub const SSC_FSOS_LOW: c_int = 3;
pub const SSC_FSOS_HIGH: c_int = 4;
pub const SSC_FSOS_TOGGLE: c_int = 5;
pub const START_DELAY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ssc_state {
    pub ssc_cmr: u32,
    pub ssc_rcmr: u32,
    pub ssc_rfmr: u32,
    pub ssc_tcmr: u32,
    pub ssc_tfmr: u32,
    pub ssc_sr: u32,
    pub ssc_imr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ssc_info {
    pub name: *mut c_char,
    pub ssc: *mut ssc_device,
    pub /: *mut *mut unsigned short dir_mask; / 0=unused, 1=playback, 2=capture,
    pub /: *mut *mut unsigned short initialized; / true if SSC has been initialized,
    pub daifmt: c_ushort,
    pub cmr_div: c_ushort,
    pub tcmr_period: c_ushort,
    pub rcmr_period: c_ushort,
    pub forced_divider: c_uint,
    pub dma_params: [*mut atmel_pcm_dma_params; 2],
    pub ssc_state: atmel_ssc_state,
    pub mck_rate: c_ulong,
}

extern "C" {
    pub fn atmel_ssc_set_audio(ssc_id: c_int) -> c_int;
}
extern "C" {
    pub fn atmel_ssc_put_audio(ssc_id: c_int);
}
