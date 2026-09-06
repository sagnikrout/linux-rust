//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/elf.h
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
// ELF register definitions..
//

//
// This is used to ensure we don't load something for the wrong architecture.
//

// Macro flag: #define CORE_DUMP_USE_REGSET

//
// This is the base location for PIE (ET_DYN with INTERP) loads. On
// 64-bit, this is raised to 4GB to leave the entire 32-bit address
// space open for things that want to use the area for 32-bit pointers.
//

//
// Our registers are always unsigned longs, whether we're a 32 bit
// process or 64 bit, on either a 64 bit or 32 bit kernel.
//
// This macro relies on elf_regs[i] having the right type to truncate to,
// either u32 or u64.  It defines the body of the elf_core_copy_regs
// function, either the native one with elf_gregset_t elf_regs or
// the 32-bit one with elf_gregset_t32 elf_regs.
//

// Common routine for both 32-bit and 64-bit native processes

// ELF_HWCAP yields a mask that user programs can use to figure out what

// This yields a string that ld.so will use to load implementation

// While ELF_PLATFORM indicates the ISA supported by the platform, it
// may not accurately reflect the underlying behavior of the hardware
// (as in the case of running in Power5+ compatibility mode on a
// Power6 machine).  ELF_BASE_PLATFORM allows ld.so to load libraries
// that are tuned for the real hardware.
//

//
// An executable for which elf_read_implies_exec() returns TRUE will
// have the READ_IMPLIES_EXEC personality flag set automatically. This
// is only required to work around bugs in old 32bit toolchains. Since
// the 64bit ABI has never had these issues dont enable the workaround
// even if we have an executable stack.
//

// vDSO has arch_setup_additional_pages
// Macro flag: #define ARCH_HAS_SETUP_ADDITIONAL_PAGES

// 1GB for 64bit, 8MB for 32bit

// Notes used in ET_CORE. Note name is "SPU/<fd>/<filename>".
pub const NT_SPU: c_int = 1;

// Macro flag: #define ARCH_DLINFO_CACHE_GEOMETRY

//
// The requirements here are:
// - keep the final alignment of sp (sp & 0xf)
// - make sure the 32-bit value at the first 16 byte aligned position of
// AUXV is greater than 16 for glibc compatibility.
// AT_IGNOREPPC is used for that.
// - for compatibility with glibc ARCH_DLINFO must always be defined on PPC,
// even if DLINFO_ARCH_ITEMS goes to zero or is undefined.
// update AT_VECTOR_SIZE_ARCH if the number of NEW_AUX_ENT entries changes
//

// Handle glibc compatibility. */				\
// Cache size items */						\

// Relocate the kernel image to @final_address
extern "C" {
    pub fn relocate(final_address: c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct func_desc {
    pub addr: c_ulong,
    pub toc: c_ulong,
    pub env: c_ulong,
}
