//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cacheflush.h
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

//
// This flag is used to indicate that the page pointed to by a pte is clean
// and does not require cleaning before returning it to the user.
//

//
// Book3s has no ptesync after setting a pte, so without this ptesync it's
// possible for a kernel virtual mapping access to return a spurious fault
// if it's accessed right after the pte is set. The page fault handler does
// not expect this type of fault. flush_cache_vmap is not exactly the right
// place to put this, but it seems to work well enough.
//
extern "C" {
    pub fn volatile("memory": "ptesync" :::) -> asm;
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: c_int = 1;
//
// This is called when a page has been modified by the kernel.
// It just marks the page as not i-cache clean.  We do the i-cache
// flush later when the page is given to a user process, if necessary.
//
// avoid an atomic op if possible

extern "C" {
    pub fn flush_icache_range(start: c_ulong, stop: c_ulong);
}

extern "C" {
    pub fn flush_dcache_icache_folio(folio: *mut folio);
}
//
// flush_dcache_range(): Write any modified data cache blocks out to memory and
// invalidate them. Does not invalidate the corresponding instruction cache
// blocks.
//
// @start: the start address
// @stop: the stop address (exclusive)
//
// Write any modified data cache blocks out to memory.
// Does not invalidate the corresponding cache lines (especially for
// any corresponding instruction cache).
//
// Like above, but invalidate the D-cache.  This is used by the 8xx
// to invalidate the cache so the PPC core doesn't get stale data
// from the CPM (no cache snooping here :-).
//

extern "C" {
    pub fn flush_instruction_cache();
}

