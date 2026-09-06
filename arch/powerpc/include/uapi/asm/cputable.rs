//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/cputable.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// in AT_HWCAP
pub const PPC_FEATURE_32: c_uint = 0x80000000;
pub const PPC_FEATURE_64: c_uint = 0x40000000;
pub const PPC_FEATURE_601_INSTR: c_uint = 0x20000000;
pub const PPC_FEATURE_HAS_ALTIVEC: c_uint = 0x10000000;
pub const PPC_FEATURE_HAS_FPU: c_uint = 0x08000000;
pub const PPC_FEATURE_HAS_MMU: c_uint = 0x04000000;
pub const PPC_FEATURE_HAS_4xxMAC: c_uint = 0x02000000;
pub const PPC_FEATURE_UNIFIED_CACHE: c_uint = 0x01000000;
pub const PPC_FEATURE_HAS_SPE: c_uint = 0x00800000;
pub const PPC_FEATURE_HAS_EFP_SINGLE: c_uint = 0x00400000;
pub const PPC_FEATURE_HAS_EFP_DOUBLE: c_uint = 0x00200000;
pub const PPC_FEATURE_NO_TB: c_uint = 0x00100000;
pub const PPC_FEATURE_POWER4: c_uint = 0x00080000;
pub const PPC_FEATURE_POWER5: c_uint = 0x00040000;
pub const PPC_FEATURE_POWER5_PLUS: c_uint = 0x00020000;
pub const PPC_FEATURE_CELL: c_uint = 0x00010000;
pub const PPC_FEATURE_BOOKE: c_uint = 0x00008000;
pub const PPC_FEATURE_SMT: c_uint = 0x00004000;
pub const PPC_FEATURE_ICACHE_SNOOP: c_uint = 0x00002000;
pub const PPC_FEATURE_ARCH_2_05: c_uint = 0x00001000;
pub const PPC_FEATURE_PA6T: c_uint = 0x00000800;
pub const PPC_FEATURE_HAS_DFP: c_uint = 0x00000400;
pub const PPC_FEATURE_POWER6_EXT: c_uint = 0x00000200;
pub const PPC_FEATURE_ARCH_2_06: c_uint = 0x00000100;
pub const PPC_FEATURE_HAS_VSX: c_uint = 0x00000080;

// Reserved - do not use		0x00000004
pub const PPC_FEATURE_TRUE_LE: c_uint = 0x00000002;
pub const PPC_FEATURE_PPC_LE: c_uint = 0x00000001;
// in AT_HWCAP2
pub const PPC_FEATURE2_ARCH_2_07: c_uint = 0x80000000;
pub const PPC_FEATURE2_HTM: c_uint = 0x40000000;
pub const PPC_FEATURE2_DSCR: c_uint = 0x20000000;
pub const PPC_FEATURE2_EBB: c_uint = 0x10000000;
pub const PPC_FEATURE2_ISEL: c_uint = 0x08000000;
pub const PPC_FEATURE2_TAR: c_uint = 0x04000000;
pub const PPC_FEATURE2_VEC_CRYPTO: c_uint = 0x02000000;
pub const PPC_FEATURE2_HTM_NOSC: c_uint = 0x01000000;
pub const PPC_FEATURE2_ARCH_3_00: c_uint = 0x00800000 /* ISA 3.00 */;
pub const PPC_FEATURE2_HAS_IEEE128: c_uint = 0x00400000 /* VSX IEEE Binary Float 128-bit */;
pub const PPC_FEATURE2_DARN: c_uint = 0x00200000 /* darn random number insn */;
pub const PPC_FEATURE2_SCV: c_uint = 0x00100000 /* scv syscall */;
pub const PPC_FEATURE2_HTM_NO_SUSPEND: c_uint = 0x00080000 /* TM w/out suspended state */;
pub const PPC_FEATURE2_ARCH_3_1: c_uint = 0x00040000 /* ISA 3.1 */;
pub const PPC_FEATURE2_MMA: c_uint = 0x00020000 /* Matrix Multiply Assist */;
pub const PPC_FEATURE2_ARCH_3_2: c_uint = 0x00010000 /* ISA 3.2 */;
pub const PPC_FEATURE2_DMF: c_uint = 0x00008000 /* Dense Math Facility */;
//
// IMPORTANT!
// All future PPC_FEATURE definitions should be allocated in cooperation with
// OPAL / skiboot firmware, in accordance with the ibm,powerpc-cpu-features
// device tree binding.
//
