//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_heap_config.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// ROGUE Device Virtual Address Space Definitions
//
// This file defines the ROGUE virtual address heaps that are used in
// application memory contexts. It also shows where the Firmware memory heap
// fits into this, but the firmware heap is only ever created in the
// kernel driver and never exposed to userspace.
//
// ROGUE_PDSCODEDATA_HEAP_BASE and ROGUE_USCCODE_HEAP_BASE will be programmed,
// on a global basis, into ROGUE_CR_PDS_EXEC_BASE and ROGUE_CR_USC_CODE_BASE_
// respectively. Therefore if client drivers use multiple configs they must
// still be consistent with their definitions for these heaps.
//
// Base addresses have to be a multiple of 4MiB.
// Heaps must not start at 0x0000000000, as this is reserved for internal
// use within the driver.
// Range comments, those starting in column 0 below are a section heading of
// sorts and are above the heaps in that range. Often this is the reserved
// size of the heap within the range.
//
// 0x00_0000_0000
// 0x00_0000_0000 - 0x00_0040_0000
// 0 MiB to 4 MiB, size of 4 MiB : RESERVED
// 0x00_0040_0000 - 0x7F_FFC0_0000
// 4 MiB to 512 GiB, size of 512 GiB less 4 MiB : RESERVED
// 0x80_0000_0000
// 0x80_0000_0000 - 0x9F_FFFF_FFFF
// 512 GiB to 640 GiB, size of 128 GiB : GENERAL_HEAP
pub const ROGUE_GENERAL_HEAP_BASE: c_uint = 0x8000000000ull;

// 0xA0_0000_0000 - 0xAF_FFFF_FFFF
// 640 GiB to 704 GiB, size of 64 GiB : FREE
// B0_0000_0000 - 0xB7_FFFF_FFFF
// 704 GiB to 736 GiB, size of 32 GiB : FREE
// 0xB8_0000_0000 - 0xBF_FFFF_FFFF
// 736 GiB to 768 GiB, size of 32 GiB : RESERVED
// 0xC0_0000_0000
// 0xC0_0000_0000 - 0xD9_FFFF_FFFF
// 768 GiB to 872 GiB, size of 104 GiB : FREE
// 0xDA_0000_0000 - 0xDA_FFFF_FFFF
// 872 GiB to 876 GiB, size of 4 GiB : PDSCODEDATA_HEAP
pub const ROGUE_PDSCODEDATA_HEAP_BASE: c_uint = 0xDA00000000ull;

// 0xDB_0000_0000 - 0xDB_FFFF_FFFF
// 876 GiB to 880 GiB, size of 256 MiB (reserved 4GiB) : BRN
//
// The BRN63142 quirk workaround requires Region Header memory to be at the top
// of a 16GiB aligned range. This is so when masked with 0x03FFFFFFFF the
// address will avoid aliasing PB addresses. Start at 879.75GiB. Size of 256MiB.
//
pub const ROGUE_RGNHDR_HEAP_BASE: c_uint = 0xDBF0000000ull;

// 0xDC_0000_0000 - 0xDF_FFFF_FFFF
// 880 GiB to 896 GiB, size of 16 GiB : FREE
// 0xE0_0000_0000 - 0xE0_FFFF_FFFF
// 896 GiB to 900 GiB, size of 4 GiB : USCCODE_HEAP
pub const ROGUE_USCCODE_HEAP_BASE: c_uint = 0xE000000000ull;

// 0xE1_0000_0000 - 0xE1_BFFF_FFFF
// 900 GiB to 903 GiB, size of 3 GiB : RESERVED
// 0xE1_C000_0000 - 0xE1_FFFF_FFFF
// 903 GiB to 904 GiB, reserved 1 GiB, : FIRMWARE_HEAP
pub const ROGUE_FW_HEAP_BASE: c_uint = 0xE1C0000000ull;
// 0xE2_0000_0000 - 0xE3_FFFF_FFFF
// 904 GiB to 912 GiB, size of 8 GiB : FREE
// 0xE4_0000_0000 - 0xE7_FFFF_FFFF
// 912 GiB to 928 GiB, size of 16 GiB : TRANSFER_FRAG
pub const ROGUE_TRANSFER_FRAG_HEAP_BASE: c_uint = 0xE400000000ull;

// 0xE8_0000_0000 - 0xF1_FFFF_FFFF
// 928 GiB to 968 GiB, size of 40 GiB : RESERVED
// 0xF2_0000_0000 - 0xF2_001F_FFFF
// 968 GiB to 969 GiB, size of 2 MiB : VISTEST_HEAP
pub const ROGUE_VISTEST_HEAP_BASE: c_uint = 0xF200000000ull;

// 0xF2_4000_0000 - 0xF2_FFFF_FFFF
// 969 GiB to 972 GiB, size of 3 GiB : FREE
// 0xF3_0000_0000 - 0xFF_FFFF_FFFF
// 972 GiB to 1024 GiB, size of 52 GiB : FREE
// 0xFF_FFFF_FFFF
