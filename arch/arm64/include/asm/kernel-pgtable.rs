//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kernel-pgtable.h
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
//
// Kernel page table mapping
//
// Copyright (C) 2015 ARM Ltd.
//

//
// The physical and virtual addresses of the start of the kernel image are
// equal modulo 2 MiB (per the arm64 booting.txt requirements). Hence we can
// use section mapping with 4K (section size = 2M) but not with 16K (section
// size = 32M) or 64K (section size = 512M).
//

pub const SWAPPER_SKIP_LEVEL: c_int = 1;

pub const SWAPPER_SKIP_LEVEL: c_int = 0;

pub const IDMAP_VA_BITS: c_int = 48;

//
// A relocatable kernel may execute from an address that differs from the one at
// which it was linked. In the worst case, its runtime placement may intersect
// with two adjacent PGDIR entries, which means that an additional page table
// may be needed at each subordinate level.
//

// The number of segments in the kernel image (text, rodata, inittext, initdata, data+bss)
pub const KERNEL_SEGMENT_COUNT: c_int = 5;

//
// KERNEL_SEGMENT_COUNT counts the permanent kernel VMAs. The early mapping
// has one additional split, [_text, _stext). Reserve one more page for the
// SWAPPER_BLOCK_SIZE-unaligned boundaries.
//

//
// The initial ID map consists of the kernel image, mapped as two separate
// segments, and may appear misaligned wrt the swapper block size. This means
// we need 3 additional pages. The DT could straddle a swapper block boundary,
// so it may need 2.
//
pub const EARLY_IDMAP_EXTRA_PAGES: c_int = 3;
pub const EARLY_IDMAP_EXTRA_FDT_PAGES: c_int = 2;

pub const EARLY_SEGMENT_EXTRA_PAGES: c_int = 0;
pub const EARLY_IDMAP_EXTRA_PAGES: c_int = 0;
pub const EARLY_IDMAP_EXTRA_FDT_PAGES: c_int = 0;

