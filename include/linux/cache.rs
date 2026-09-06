//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cache.h
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

//
// SMP_CACHE_ALIGN - align a value to the L2 cacheline size
// @x: value to align
//
// On some architectures, L2 ("SMP") CL size is bigger than L1, and sometimes,
// this needs to be accounted.
//
// Return: aligned value.
//

//
// ``__aligned_largest`` aligns a field to the value most optimal for the
// target architecture to perform memory operations. Get the actual value
// to be able to use it anywhere else.
//

//
// __read_mostly is used to keep rarely changing variables out of frequently
// updated cachelines. Its use should be reserved for data that is used
// frequently in hot paths. Performance traces can help decide when to use
// this. You want __read_mostly data to be tightly packed, so that in the
// best case multiple frequently read variables for a hot path will be next
// to each other in order to reduce the number of cachelines needed to
// execute a critical path. We should be mindful and selective of its use.
// ie: if you're going to use it please supply a *good* justification in your
// commit log
//

// Macro flag: #define __read_mostly

//
// __ro_after_init is used to mark things that are read-only after init (i.e.
// after mark_rodata_ro() has been called). These are effectively read-only,
// but may get written to during init, so can't live in .rodata (via "const").
//

// Macro flag: #define ____cacheline_aligned_in_smp

// Macro flag: #define __cacheline_aligned_in_smp

//
// The maximum alignment needed for some critical structures
// These could be inter-node cacheline sizes/L3 cacheline
// size etc.  Define this in asm/cache.h for your arch
//

// Macro flag: #define ____cacheline_internodealigned_in_smp

//
// __cacheline_group_begin_aligned - declare an aligned group start
// @GROUP: name of the group
// @...: optional group alignment
//
// The following block inside a struct:
//
// __cacheline_group_begin_aligned(grp);
// field a;
// field b;
// __cacheline_group_end_aligned(grp);
//
// will always be aligned to either the specified alignment or
// ``SMP_CACHE_BYTES``.
//

//
// __cacheline_group_end_aligned - declare an aligned group end
// @GROUP: name of the group
// @...: optional alignment (same as was in __cacheline_group_begin_aligned())
//
// Note that the end marker is aligned to sizeof(long) to allow more precise
// size assertion. It also declares a padding at the end to avoid next field
// falling into this cacheline.
//

//
// Helper to add padding within a struct to ensure data fall into separate
// cachelines.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cacheline_padding {
    pub x: [c_char; 0],
    pub ____cacheline_internodealigned_in_smp: },

// Macro flag: #define CACHELINE_PADDING(name)

// Macro flag: #define ARCH_HAS_DMA_MINALIGN

