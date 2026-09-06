//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ak4531_codec.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Universal interface for Audio Codec '97
//
// For more details look to AC '97 component specification revision 2.1
// by Intel Corporation (http://developer.intel.com).
//

//
// ASAHI KASEI - AK4531 codec
// - not really AC'97 codec, but it uses very similar interface as AC'97
//
// AK4531 codec registers
//
pub const AK4531_LMASTER: c_uint = 0x00	/* master volume left */;
pub const AK4531_RMASTER: c_uint = 0x01	/* master volume right */;
pub const AK4531_LVOICE: c_uint = 0x02	/* channel volume left */;
pub const AK4531_RVOICE: c_uint = 0x03	/* channel volume right */;
pub const AK4531_LFM: c_uint = 0x04	/* FM volume left */;
pub const AK4531_RFM: c_uint = 0x05	/* FM volume right */;
pub const AK4531_LCD: c_uint = 0x06	/* CD volume left */;
pub const AK4531_RCD: c_uint = 0x07	/* CD volume right */;
pub const AK4531_LLINE: c_uint = 0x08	/* LINE volume left */;
pub const AK4531_RLINE: c_uint = 0x09	/* LINE volume right */;
pub const AK4531_LAUXA: c_uint = 0x0a	/* AUXA volume left */;
pub const AK4531_RAUXA: c_uint = 0x0b	/* AUXA volume right */;
pub const AK4531_MONO1: c_uint = 0x0c	/* MONO1 volume left */;
pub const AK4531_MONO2: c_uint = 0x0d	/* MONO1 volume right */;
pub const AK4531_MIC: c_uint = 0x0e	/* MIC volume */;
pub const AK4531_MONO_OUT: c_uint = 0x0f	/* Mono-out volume */;
pub const AK4531_OUT_SW1: c_uint = 0x10	/* Output mixer switch 1 */;
pub const AK4531_OUT_SW2: c_uint = 0x11	/* Output mixer switch 2 */;
pub const AK4531_LIN_SW1: c_uint = 0x12	/* Input left mixer switch 1 */;
pub const AK4531_RIN_SW1: c_uint = 0x13	/* Input right mixer switch 1 */;
pub const AK4531_LIN_SW2: c_uint = 0x14	/* Input left mixer switch 2 */;
pub const AK4531_RIN_SW2: c_uint = 0x15	/* Input right mixer switch 2 */;
pub const AK4531_RESET: c_uint = 0x16	/* Reset & power down */;
pub const AK4531_CLOCK: c_uint = 0x17	/* Clock select */;
pub const AK4531_AD_IN: c_uint = 0x18	/* AD input select */;
pub const AK4531_MIC_GAIN: c_uint = 0x19	/* MIC amplified gain */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ak4531 {
    pub val): c_ushort,
    pub private_data: *mut c_void,
    pub ak4531): *mut *mut void (private_free) (struct snd_ak4531,
// ---
    pub regs: [c_uchar; 0x20],
    pub reg_mutex: mutex,
}

extern "C" {
    pub fn snd_ak4531_suspend(ak4531: *mut snd_ak4531);
}
extern "C" {
    pub fn snd_ak4531_resume(ak4531: *mut snd_ak4531);
}

