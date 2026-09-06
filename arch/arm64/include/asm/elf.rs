//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/elf.h
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
// Copyright (C) 2012 ARM Ltd.
//

//
// ELF register definitions..
//

//
// AArch64 static relocation types.
//
// Miscellaneous.
pub const R_ARM_NONE: c_int = 0;
pub const R_AARCH64_NONE: c_int = 256;
// Data.
pub const R_AARCH64_ABS64: c_int = 257;
pub const R_AARCH64_ABS32: c_int = 258;
pub const R_AARCH64_ABS16: c_int = 259;
pub const R_AARCH64_PREL64: c_int = 260;
pub const R_AARCH64_PREL32: c_int = 261;
pub const R_AARCH64_PREL16: c_int = 262;
// Instructions.
pub const R_AARCH64_MOVW_UABS_G0: c_int = 263;
pub const R_AARCH64_MOVW_UABS_G0_NC: c_int = 264;
pub const R_AARCH64_MOVW_UABS_G1: c_int = 265;
pub const R_AARCH64_MOVW_UABS_G1_NC: c_int = 266;
pub const R_AARCH64_MOVW_UABS_G2: c_int = 267;
pub const R_AARCH64_MOVW_UABS_G2_NC: c_int = 268;
pub const R_AARCH64_MOVW_UABS_G3: c_int = 269;
pub const R_AARCH64_MOVW_SABS_G0: c_int = 270;
pub const R_AARCH64_MOVW_SABS_G1: c_int = 271;
pub const R_AARCH64_MOVW_SABS_G2: c_int = 272;
pub const R_AARCH64_LD_PREL_LO19: c_int = 273;
pub const R_AARCH64_ADR_PREL_LO21: c_int = 274;
pub const R_AARCH64_ADR_PREL_PG_HI21: c_int = 275;
pub const R_AARCH64_ADR_PREL_PG_HI21_NC: c_int = 276;
pub const R_AARCH64_ADD_ABS_LO12_NC: c_int = 277;
pub const R_AARCH64_LDST8_ABS_LO12_NC: c_int = 278;
pub const R_AARCH64_TSTBR14: c_int = 279;
pub const R_AARCH64_CONDBR19: c_int = 280;
pub const R_AARCH64_JUMP26: c_int = 282;
pub const R_AARCH64_CALL26: c_int = 283;
pub const R_AARCH64_LDST16_ABS_LO12_NC: c_int = 284;
pub const R_AARCH64_LDST32_ABS_LO12_NC: c_int = 285;
pub const R_AARCH64_LDST64_ABS_LO12_NC: c_int = 286;
pub const R_AARCH64_LDST128_ABS_LO12_NC: c_int = 299;
pub const R_AARCH64_MOVW_PREL_G0: c_int = 287;
pub const R_AARCH64_MOVW_PREL_G0_NC: c_int = 288;
pub const R_AARCH64_MOVW_PREL_G1: c_int = 289;
pub const R_AARCH64_MOVW_PREL_G1_NC: c_int = 290;
pub const R_AARCH64_MOVW_PREL_G2: c_int = 291;
pub const R_AARCH64_MOVW_PREL_G2_NC: c_int = 292;
pub const R_AARCH64_MOVW_PREL_G3: c_int = 293;
pub const R_AARCH64_RELATIVE: c_int = 1027;
//
// These are used to set parameters in the core dumps.
//

//
// This yields a string that ld.so will use to load implementation
// specific libraries for optimization.  This is more specific in
// intent than poking at uname or /proc/cpuinfo.
//
pub const ELF_PLATFORM_SIZE: c_int = 16;

//
// This is used to ensure we don't load something for the wrong architecture.
//

//
// An executable for which elf_read_implies_exec() returns TRUE will
// have the READ_IMPLIES_EXEC personality flag set automatically.
//
// The decision process for determining the results are:
//
// CPU*: | arm32      | arm64      |
// ELF:                 |            |            |
// ---------------------|------------|------------|
// missing PT_GNU_STACK | exec-all   | exec-none  |
// PT_GNU_STACK == RWX  | exec-stack | exec-stack |
// PT_GNU_STACK == RW   | exec-none  | exec-none  |
//
// exec-all  : all PROT_READ user mappings are executable, except when
// backed by files on a noexec-filesystem.
// exec-none : only PROT_EXEC user mappings are executable.
// exec-stack: only the stack and PROT_EXEC user mappings are executable.
//
// *all arm64 CPUs support NX, so there is no "lacks NX" column.
//

// Macro flag: #define CORE_DUMP_USE_REGSET

//
// This is the base location for PIE (ET_DYN with INTERP) loads. On
// 64-bit, this is above 4GB to leave the entire 32-bit address
// space open for things that want to use the area for 32-bit pointers.
//

pub type elf_greg_t = c_ulong;

// (struct user_pt_regs *)&(dest) = (regs)->user_regs;
pub type elf_fpregset_t = user_fpsimd_state;
//
// When the program starts, a1 contains a pointer to a function to be
// registered with atexit, as per the SVR4 ABI.  A value of 0 means we have no
// such handler.
//

// update AT_VECTOR_SIZE_ARCH if the number of NEW_AUX_ENT entries changes

// \
// Should always be nonzero unless there's a kernel bug.	\
// If we haven't determined a sensible value to give to		\
// userspace, omit the entry:					\
// \
// Macro flag: #define ARCH_HAS_SETUP_ADDITIONAL_PAGES
// 1GB of VA

// AArch32 registers.
pub const COMPAT_ELF_NGREG: c_int = 18;
pub type compat_elf_greg_t = c_uint;

// PIE load location for compat arm. Must match ARM ELF_ET_DYN_BASE.
pub const COMPAT_ELF_ET_DYN_BASE: c_uint = 0x000400000UL;
// AArch32 EABI.
pub const EF_ARM_EABI_MASK: c_uint = 0xff000000;
extern "C" {
    pub fn compat_elf_check_arch(: *const elf32_hdr) -> c_int;
}

//
// Unlike the native SET_PERSONALITY macro, the compat version maintains
// READ_IMPLIES_EXEC across an execve() since this is the behaviour on
// arch/arm/.
//

// \
// Note that we use Elf64_Off instead of elf_addr_t because	\
// elf_addr_t in compat is defined as Elf32_Addr and casting	\
// current->mm->context.vdso to it triggers a cast warning of	\
// cast from pointer to integer of different size.		\
// \

// Macro flag: #define COMPAT_ARCH_DLINFO

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_elf_state {
    pub flags: c_int,
}

// No known properties for AArch32 yet

