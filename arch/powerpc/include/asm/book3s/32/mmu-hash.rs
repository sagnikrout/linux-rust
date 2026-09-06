//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/32/mmu-hash.h
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
// 32-bit hash table MMU support
//
// BATs
//
// Block size masks
pub const BL_128K: c_uint = 0x000;
pub const BL_256K: c_uint = 0x001;
pub const BL_512K: c_uint = 0x003;
pub const BL_1M: c_uint = 0x007;
pub const BL_2M: c_uint = 0x00F;
pub const BL_4M: c_uint = 0x01F;
pub const BL_8M: c_uint = 0x03F;
pub const BL_16M: c_uint = 0x07F;
pub const BL_32M: c_uint = 0x0FF;
pub const BL_64M: c_uint = 0x1FF;
pub const BL_128M: c_uint = 0x3FF;
pub const BL_256M: c_uint = 0x7FF;
// BAT Access Protection
pub const BPP_XX: c_uint = 0x00		/* No access */;
pub const BPP_RX: c_uint = 0x01		/* Read only */;
pub const BPP_RW: c_uint = 0x02		/* Read/write */;
// Contort a phys_addr_t into the right format/bits for a BAT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_bat {
    pub batu: u32,
    pub batl: u32,
}

//
// Hash table
//
// Values for PP (assumes Ks=0, Kp=1)

// Values for Segment Registers
pub const SR_NX: c_uint = 0x10000000	/* No Execute */;
pub const SR_KP: c_uint = 0x20000000	/* User key */;
pub const SR_KS: c_uint = 0x40000000	/* Supervisor key */;

//
// This isync() shouldn't be necessary as the kernel is not excepted to run
// any instruction in userspace soon after the update of segments and 'rfi'
// instruction is used to return to userspace, but hash based cores
// (at least G3) seem to exhibit a random behaviour when the 'isync' is not
// there. 603 cores don't have this behaviour so don't do the 'isync' as it
// saves several CPU cycles.
//

//
// This macro defines the mapping from contexts to VSIDs (virtual
// segment IDs).  We use a skew on both the context and the high 4 bits
// of the 32-bit virtual address (the "effective segment ID") in order
// to spread out the entries in the MMU hash table.  Note, if this
// function is changed then hash functions will have to be
// changed to correspond.
//

//
// Hardware Page Table Entry
// Note that the xpn and x bitfields are used only by processors that
// support extended addressing; otherwise, those bits are reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_pte {
    pub /: *mut *mut unsigned long v:1; / Entry is valid,
    pub /: *mut *mut unsigned long vsid:24; / Virtual segment identifier,
    pub /: *mut *mut unsigned long h:1; / Hash algorithm indicator,
    pub /: *mut *mut unsigned long api:6; / Abbreviated page index,
    pub /: *mut *mut unsigned long rpn:20; / Real (physical) page number,
    pub /: *mut *mut unsigned long xpn:3; / Real page number bits 0-2, optional,
    pub /: *mut *mut unsigned long r:1; / Referenced,
    pub /: *mut *mut unsigned long c:1; / Changed,
    pub /: *mut *mut unsigned long w:1; / Write-thru cache mode,
    pub /: *mut *mut unsigned long i:1; / Cache inhibited,
    pub /: *mut *mut unsigned long m:1; / Memory coherence,
    pub /: *mut *mut unsigned long g:1; / Guarded,
    pub /: *mut *mut unsigned long x:1; / Real page number bit 3, optional,
    pub /: *mut *mut unsigned long pp:2; / Page protection,
}

extern "C" {
    pub fn update_bats();
}
// patch sites

extern "C" {
    pub fn find_free_bat() -> int __init;
}
extern "C" {
    pub fn bat_block_size(base: c_ulong, top: c_ulong) -> c_uint;
}

// We happily ignore the smaller BATs on 601, we don't actually use
// those definitions on hash32 at the moment anyway
//

