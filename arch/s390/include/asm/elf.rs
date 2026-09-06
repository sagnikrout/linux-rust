//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/elf.h
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
// S390 version
//
// Derived from "include/asm-i386/elf.h"
//
// s390 relocations defined by the ABIs

// Keep this the last entry.
pub const R_390_NUM: c_int = 61;
//
// HWCAP flags - for AT_HWCAP
//
// Bits 32-63 are reserved for use by libc.
// Bit 31 is reserved and will be used by libc to determine if a second
// argument is passed to IFUNC resolvers. This will be implemented when
// there is a need for AT_HWCAP2.
//
// Bits present in AT_HWCAP.

//
// These are used to set parameters in the core dumps.
//

//
// ELF register definitions..
//

pub type elf_fpregset_t = s390_fp_regs;
pub type elf_gregset_t = s390_regs;

//
// This is used to ensure we don't load something for the wrong architecture.
//

// For SVR4/S390 the function pointer to be registered with `atexit` is

// Macro flag: #define CORE_DUMP_USE_REGSET

// This is the location that an ET_DYN program is loaded if exec'ed.  Typical

// This yields a mask that user programs can use to figure out what

// This yields a string that ld.so will use to load implementation
pub const ELF_PLATFORM_SIZE: c_int = 8;

//
// Cache aliasing on the latest machines calls for a mapping granularity
// of 512KB for the anonymous mapping base. Use a 512KB alignment and a
// randomization of up to 1GB.
// For the additional randomization of the program break use 32MB.
//

// update AT_VECTOR_SIZE_ARCH if the number of NEW_AUX_ENT entries changes

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: c_int = 1;
extern "C" {
    pub fn arch_setup_additional_pages(: *mut linux_binprm, _arg: c_int) -> c_int;
}
