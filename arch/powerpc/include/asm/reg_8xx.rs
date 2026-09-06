//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/reg_8xx.h
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
// Contains register definitions common to PowerPC 8xx CPUs.  Notice
//

// Macro flag: #define _ASM_POWERPC_REG_8xx_H
// Cache control on the MPC8xx is provided through some additional
// special purpose registers.
//

// Misc Debug
pub const SPRN_DPDR: c_int = 630;
pub const SPRN_MI_CAM: c_int = 816;
pub const SPRN_MI_RAM0: c_int = 817;
pub const SPRN_MI_RAM1: c_int = 818;
pub const SPRN_MD_CAM: c_int = 824;
pub const SPRN_MD_RAM0: c_int = 825;
pub const SPRN_MD_RAM1: c_int = 826;
// Special MSR manipulation registers

// Debug registers
pub const SPRN_CMPA: c_int = 144;
pub const SPRN_COUNTA: c_int = 150;
pub const SPRN_CMPE: c_int = 152;
pub const SPRN_CMPF: c_int = 153;
pub const SPRN_LCTRL1: c_int = 156;
pub const LCTRL1_CTE_GT: c_uint = 0xc0000000;
pub const LCTRL1_CTF_LT: c_uint = 0x14000000;
pub const LCTRL1_CRWE_RW: c_uint = 0x00000000;
pub const LCTRL1_CRWE_RO: c_uint = 0x00040000;
pub const LCTRL1_CRWE_WO: c_uint = 0x000c0000;
pub const LCTRL1_CRWF_RW: c_uint = 0x00000000;
pub const LCTRL1_CRWF_RO: c_uint = 0x00010000;
pub const LCTRL1_CRWF_WO: c_uint = 0x00030000;
pub const SPRN_LCTRL2: c_int = 157;
pub const LCTRL2_LW0EN: c_uint = 0x80000000;
pub const LCTRL2_LW0LA_E: c_uint = 0x00000000;
pub const LCTRL2_LW0LA_F: c_uint = 0x04000000;
pub const LCTRL2_LW0LA_EandF: c_uint = 0x08000000;
pub const LCTRL2_LW0LADC: c_uint = 0x02000000;
pub const LCTRL2_SLW0EN: c_uint = 0x00000002;

pub const SPRN_ICTRL: c_int = 158;

pub const SPRN_BAR: c_int = 159;
// Commands.  Only the first few are available to the instruction cache.
//
pub const IDC_ENABLE: c_uint = 0x02000000	/* Cache enable */;
pub const IDC_DISABLE: c_uint = 0x04000000	/* Cache disable */;
pub const IDC_LDLCK: c_uint = 0x06000000	/* Load and lock */;
pub const IDC_UNLINE: c_uint = 0x08000000	/* Unlock line */;
pub const IDC_UNALL: c_uint = 0x0a000000	/* Unlock all */;
pub const IDC_INVALL: c_uint = 0x0c000000	/* Invalidate all */;
pub const DC_FLINE: c_uint = 0x0e000000	/* Flush data cache line */;
pub const DC_SFWT: c_uint = 0x01000000	/* Set forced writethrough mode */;
pub const DC_CFWT: c_uint = 0x03000000	/* Clear forced writethrough mode */;
pub const DC_SLES: c_uint = 0x05000000	/* Set little endian swap mode */;
pub const DC_CLES: c_uint = 0x07000000	/* Clear little endian swap mode */;
// Status.
//
pub const IDC_ENABLED: c_uint = 0x80000000	/* Cache is enabled */;
pub const IDC_CERR1: c_uint = 0x00200000	/* Cache error 1 */;
pub const IDC_CERR2: c_uint = 0x00100000	/* Cache error 2 */;
pub const IDC_CERR3: c_uint = 0x00080000	/* Cache error 3 */;
pub const DC_DFWT: c_uint = 0x40000000	/* Data cache is forced write through */;
pub const DC_LES: c_uint = 0x20000000	/* Caches are little endian mode */;
