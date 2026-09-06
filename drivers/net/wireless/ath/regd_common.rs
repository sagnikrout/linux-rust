//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/regd_common.h
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


//
// Copyright (c) 2008-2009 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EnumRd {
    NO_ENUMRD = 0x00,
    NULL1_WORLD = 0x03,
    NULL1_ETSIB = 0x07,
    NULL1_ETSIC = 0x08,
    FCC1_FCCA = 0x10,
    FCC1_WORLD = 0x11,
    FCC4_FCCA = 0x12,
    FCC5_FCCA = 0x13,
    FCC6_FCCA = 0x14,

    FCC2_FCCA = 0x20,
    FCC2_WORLD = 0x21,
    FCC2_ETSIC = 0x22,
    FCC6_WORLD = 0x23,
    FRANCE_RES = 0x31,
    FCC3_FCCA = 0x3A,
    FCC3_WORLD = 0x3B,
    FCC3_ETSIC = 0x3F,

    ETSI1_WORLD = 0x37,
    ETSI3_ETSIA = 0x32,
    ETSI2_WORLD = 0x35,
    ETSI3_WORLD = 0x36,
    ETSI4_WORLD = 0x30,
    ETSI4_ETSIC = 0x38,
    ETSI5_WORLD = 0x39,
    ETSI6_WORLD = 0x34,
    ETSI8_WORLD = 0x3D,
    ETSI9_WORLD = 0x3E,
    ETSI_RESERVED = 0x33,

    MKK1_MKKA = 0x40,
    MKK1_MKKB = 0x41,
    APL4_WORLD = 0x42,
    MKK2_MKKA = 0x43,
    APL_RESERVED = 0x44,
    APL2_WORLD = 0x45,
    APL2_APLC = 0x46,
    APL3_WORLD = 0x47,
    MKK1_FCCA = 0x48,
    APL2_APLD = 0x49,
    MKK1_MKKA1 = 0x4A,
    MKK1_MKKA2 = 0x4B,
    MKK1_MKKC = 0x4C,
    APL2_FCCA = 0x4D,

    APL3_FCCA = 0x50,
    APL1_WORLD = 0x52,
    APL1_FCCA = 0x53,
    APL1_APLA = 0x54,
    APL1_ETSIC = 0x55,
    APL2_ETSIC = 0x56,
    APL5_WORLD = 0x58,
    APL13_WORLD = 0x5A,
    APL6_WORLD = 0x5B,
    APL7_FCCA = 0x5C,
    APL8_WORLD = 0x5D,
    APL9_WORLD = 0x5E,
    APL10_WORLD = 0x5F,

    WOR0_WORLD = 0x60,
    WOR1_WORLD = 0x61,
    WOR2_WORLD = 0x62,
    WOR3_WORLD = 0x63,
    WOR4_WORLD = 0x64,
    WOR5_ETSIC = 0x65,

    WOR01_WORLD = 0x66,
    WOR02_WORLD = 0x67,
    EU1_WORLD = 0x68,

    WOR9_WORLD = 0x69,
    WORA_WORLD = 0x6A,
    WORB_WORLD = 0x6B,
    WORC_WORLD = 0x6C,

    MKK3_MKKB = 0x80,
    MKK3_MKKA2 = 0x81,
    MKK3_MKKC = 0x82,

    MKK4_MKKB = 0x83,
    MKK4_MKKA2 = 0x84,
    MKK4_MKKC = 0x85,

    MKK5_MKKB = 0x86,
    MKK5_MKKA2 = 0x87,
    MKK5_MKKC = 0x88,

    MKK6_MKKB = 0x89,
    MKK6_MKKA2 = 0x8A,
    MKK6_MKKC = 0x8B,

    MKK7_MKKB = 0x8C,
    MKK7_MKKA2 = 0x8D,
    MKK7_MKKC = 0x8E,

    MKK8_MKKB = 0x8F,
    MKK8_MKKA2 = 0x90,
    MKK8_MKKC = 0x91,

    MKK14_MKKA1 = 0x92,
    MKK15_MKKA1 = 0x93,

    MKK10_FCCA = 0xD0,
    MKK10_MKKA1 = 0xD1,
    MKK10_MKKC = 0xD2,
    MKK10_MKKA2 = 0xD3,

    MKK11_MKKA = 0xD4,
    MKK11_FCCA = 0xD5,
    MKK11_MKKA1 = 0xD6,
    MKK11_MKKC = 0xD7,
    MKK11_MKKA2 = 0xD8,

    MKK12_MKKA = 0xD9,
    MKK12_FCCA = 0xDA,
    MKK12_MKKA1 = 0xDB,
    MKK12_MKKC = 0xDC,
    MKK12_MKKA2 = 0xDD,

    MKK13_MKKB = 0xDE,

    MKK3_MKKA = 0xF0,
    MKK3_MKKA1 = 0xF1,
    MKK3_FCCA = 0xF2,
    MKK4_MKKA = 0xF3,
    MKK4_MKKA1 = 0xF4,
    MKK4_FCCA = 0xF5,
    MKK9_MKKA = 0xF6,
    MKK10_MKKA = 0xF7,
    MKK6_MKKA1 = 0xF8,
    MKK6_FCCA = 0xF9,
    MKK7_MKKA1 = 0xFA,
    MKK7_FCCA = 0xFB,
    MKK9_FCCA = 0xFC,
    MKK9_MKKA1 = 0xFD,
    MKK9_MKKC = 0xFE,
    MKK9_MKKA2 = 0xFF,

    WORLD = 0x0199,
    DEBUG_REG_DMN = 0x01ff,
}

// Regpair to CTL band mapping
// regpair, 5 GHz CTL, 2 GHz CTL
// XXX: For ETSI3_ETSIA, Was NO_CTL meant for the 2 GHz band ?
// This "PS" is for US public safety actually... to support this we
// would need to assign new special alpha2 to CRDA db as with the world
// regdomain and use another alpha2
