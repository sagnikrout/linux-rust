//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/boot.h
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

// Minimum kernel alignment, as a power of two

//
// Zstd needs to allocate the ZSTD_DCtx in order to decompress the kernel.
// The ZSTD_DCtx is ~160KB, so set the heap size to 192KB because it is a
// round number and to allow some slack.
//

//
// Used by decompressor's startup_32() to allocate page tables for identity
// mapping of the 4G of RAM in 4-level paging mode:
// - 1 level4 table;
// - 1 level3 table;
// - 4 level2 table that maps everything with 2M pages;
//
// The additional level5 table needed for 5-level paging is allocated from
// trampoline_32bit memory.
//

//
// Total number of page tables kernel_add_identity_map() can allocate,
// including page tables consumed by startup_32().
//
// Worst-case scenario:
// - 5-level paging needs 1 level5 table;
// - KASLR needs to map kernel, boot_params, cmdline and randomized kernel,
// assuming all of them cross 256T boundary:
// + 4*2 level4 table;
// + 4*2 level3 table;
// + 4*2 level2 table;
// - X86_VERBOSE_BOOTUP needs to map the first 2M (video RAM):
// + 1 level4 table;
// + 1 level3 table;
// + 1 level2 table;
// Total: 28 tables
//
// Add 4 spare table in case decompressor touches anything beyond what is
// accounted above. Warn if it happens.
//

pub const TRAMPOLINE_32BIT_CODE_SIZE: c_uint = 0xA0;
extern "C" {
    pub fn trampoline_32bit_src(trampoline: *mut c_void, enable_5lvl: bool);
}

