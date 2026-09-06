//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/microchip-ksz.h
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
// Microchip KSZ series switch platform data
//
// Copyright (C) 2017
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
pub enum ksz_chip_id {
    KSZ8463_CHIP_ID = 0x8463,
    KSZ8563_CHIP_ID = 0x8563,
    KSZ8795_CHIP_ID = 0x8795,
    KSZ8794_CHIP_ID = 0x8794,
    KSZ8765_CHIP_ID = 0x8765,
    KSZ88X3_CHIP_ID = 0x8830,
    KSZ8864_CHIP_ID = 0x8864,
    KSZ8895_CHIP_ID = 0x8895,
    KSZ9477_CHIP_ID = 0x00947700,
    KSZ9896_CHIP_ID = 0x00989600,
    KSZ9897_CHIP_ID = 0x00989700,
    KSZ9893_CHIP_ID = 0x00989300,
    KSZ9563_CHIP_ID = 0x00956300,
    KSZ8567_CHIP_ID = 0x00856700,
    KSZ9567_CHIP_ID = 0x00956700,
    LAN9370_CHIP_ID = 0x00937000,
    LAN9371_CHIP_ID = 0x00937100,
    LAN9372_CHIP_ID = 0x00937200,
    LAN9373_CHIP_ID = 0x00937300,
    LAN9374_CHIP_ID = 0x00937400,
    LAN9646_CHIP_ID = 0x00964600,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_platform_data {
// Must be first such that dsa_register_switch() can access it
    pub cd: dsa_chip_data,
    pub chip_id: u32,
}
