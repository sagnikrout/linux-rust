//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/32/mmu-44x.h
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
// PPC440 support
//

pub const PPC44x_MMUCR_TID: c_uint = 0x000000ff;
pub const PPC44x_MMUCR_STS: c_uint = 0x00010000;
pub const PPC44x_TLB_PAGEID: c_int = 0;
pub const PPC44x_TLB_XLAT: c_int = 1;
pub const PPC44x_TLB_ATTRIB: c_int = 2;
// Page identification fields
pub const PPC44x_TLB_EPN_MASK: c_uint = 0xfffffc00      /* Effective Page Number */;
pub const PPC44x_TLB_VALID: c_uint = 0x00000200      /* Valid flag */;
pub const PPC44x_TLB_TS: c_uint = 0x00000100	/* Translation address space */;
pub const PPC44x_TLB_1K: c_uint = 0x00000000	/* Page sizes */;
pub const PPC44x_TLB_4K: c_uint = 0x00000010;
pub const PPC44x_TLB_16K: c_uint = 0x00000020;
pub const PPC44x_TLB_64K: c_uint = 0x00000030;
pub const PPC44x_TLB_256K: c_uint = 0x00000040;
pub const PPC44x_TLB_1M: c_uint = 0x00000050;
pub const PPC44x_TLB_16M: c_uint = 0x00000070;
pub const PPC44x_TLB_256M: c_uint = 0x00000090;
// Translation fields
pub const PPC44x_TLB_RPN_MASK: c_uint = 0xfffffc00      /* Real Page Number */;
pub const PPC44x_TLB_ERPN_MASK: c_uint = 0x0000000f;
// Storage attribute and access control fields
pub const PPC44x_TLB_ATTR_MASK: c_uint = 0x0000ff80;
pub const PPC44x_TLB_U0: c_uint = 0x00008000      /* User 0 */;
pub const PPC44x_TLB_U1: c_uint = 0x00004000      /* User 1 */;
pub const PPC44x_TLB_U2: c_uint = 0x00002000      /* User 2 */;
pub const PPC44x_TLB_U3: c_uint = 0x00001000      /* User 3 */;
pub const PPC44x_TLB_W: c_uint = 0x00000800      /* Caching is write-through */;
pub const PPC44x_TLB_I: c_uint = 0x00000400      /* Caching is inhibited */;
pub const PPC44x_TLB_M: c_uint = 0x00000200      /* Memory is coherent */;
pub const PPC44x_TLB_G: c_uint = 0x00000100      /* Memory is guarded */;
pub const PPC44x_TLB_E: c_uint = 0x00000080      /* Memory is little endian */;
pub const PPC44x_TLB_PERM_MASK: c_uint = 0x0000003f;
pub const PPC44x_TLB_UX: c_uint = 0x00000020      /* User execution */;
pub const PPC44x_TLB_UW: c_uint = 0x00000010      /* User write */;
pub const PPC44x_TLB_UR: c_uint = 0x00000008      /* User read */;
pub const PPC44x_TLB_SX: c_uint = 0x00000004      /* Super execution */;
pub const PPC44x_TLB_SW: c_uint = 0x00000002      /* Super write */;
pub const PPC44x_TLB_SR: c_uint = 0x00000001      /* Super read */;
// Number of TLB entries
pub const PPC44x_TLB_SIZE: c_int = 64;
// 47x bits
pub const PPC47x_MMUCR_TID: c_uint = 0x0000ffff;
pub const PPC47x_MMUCR_STS: c_uint = 0x00010000;
// Page identification fields
pub const PPC47x_TLB0_EPN_MASK: c_uint = 0xfffff000      /* Effective Page Number */;
pub const PPC47x_TLB0_VALID: c_uint = 0x00000800      /* Valid flag */;
pub const PPC47x_TLB0_TS: c_uint = 0x00000400	/* Translation address space */;
pub const PPC47x_TLB0_4K: c_uint = 0x00000000;
pub const PPC47x_TLB0_16K: c_uint = 0x00000010;
pub const PPC47x_TLB0_64K: c_uint = 0x00000030;
pub const PPC47x_TLB0_1M: c_uint = 0x00000070;
pub const PPC47x_TLB0_16M: c_uint = 0x000000f0;
pub const PPC47x_TLB0_256M: c_uint = 0x000001f0;
pub const PPC47x_TLB0_1G: c_uint = 0x000003f0;
pub const PPC47x_TLB0_BOLTED_R: c_uint = 0x00000008	/* tlbre only */;
// Translation fields
pub const PPC47x_TLB1_RPN_MASK: c_uint = 0xfffff000      /* Real Page Number */;
pub const PPC47x_TLB1_ERPN_MASK: c_uint = 0x000003ff;
// Storage attribute and access control fields
pub const PPC47x_TLB2_ATTR_MASK: c_uint = 0x0003ff80;
pub const PPC47x_TLB2_IL1I: c_uint = 0x00020000      /* Memory is guarded */;
pub const PPC47x_TLB2_IL1D: c_uint = 0x00010000      /* Memory is guarded */;
pub const PPC47x_TLB2_U0: c_uint = 0x00008000      /* User 0 */;
pub const PPC47x_TLB2_U1: c_uint = 0x00004000      /* User 1 */;
pub const PPC47x_TLB2_U2: c_uint = 0x00002000      /* User 2 */;
pub const PPC47x_TLB2_U3: c_uint = 0x00001000      /* User 3 */;
pub const PPC47x_TLB2_W: c_uint = 0x00000800      /* Caching is write-through */;
pub const PPC47x_TLB2_I: c_uint = 0x00000400      /* Caching is inhibited */;
pub const PPC47x_TLB2_M: c_uint = 0x00000200      /* Memory is coherent */;
pub const PPC47x_TLB2_G: c_uint = 0x00000100      /* Memory is guarded */;
pub const PPC47x_TLB2_E: c_uint = 0x00000080      /* Memory is little endian */;
pub const PPC47x_TLB2_PERM_MASK: c_uint = 0x0000003f;
pub const PPC47x_TLB2_UX: c_uint = 0x00000020      /* User execution */;
pub const PPC47x_TLB2_UW: c_uint = 0x00000010      /* User write */;
pub const PPC47x_TLB2_UR: c_uint = 0x00000008      /* User read */;
pub const PPC47x_TLB2_SX: c_uint = 0x00000004      /* Super execution */;
pub const PPC47x_TLB2_SW: c_uint = 0x00000002      /* Super write */;
pub const PPC47x_TLB2_SR: c_uint = 0x00000001      /* Super read */;

// patch sites

pub const PPC44x_EARLY_TLBS: c_int = 1;

pub const PPC44x_EARLY_TLBS: c_int = 2;

// Size of the TLBs used for pinning in lowmem

