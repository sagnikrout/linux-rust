//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/elf.h
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
// Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
// Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
// Copyright (C) 2012 Regents of the University of California
//

//
// These are used to set parameters in the core dumps.
//

//
// This is used to ensure we don't load something for the wrong architecture.
//

extern "C" {
    pub fn compat_elf_check_arch(hdr: *mut Elf32_Ehdr) -> bool;
}

// Macro flag: #define CORE_DUMP_USE_REGSET
pub const ELF_FDPIC_CORE_EFLAGS: c_int = 0;

//
// This is the location that an ET_DYN program is loaded if exec'ed.  Typical
// use of this is to invoke "./ld.so someprog" to test out a new version of
// the loader.  We need to make sure that it is out of the way of the program
// that it will "exec", and that there is sufficient room for the brk.
//

//
// Provides information on the available set of ISA extensions to userspace,
// via a bitmap that corresponds to each single-letter ISA extension.  This is
// essentially defunct, but will remain for compatibility with userspace.
//

//
// This yields a string that ld.so will use to load implementation
// specific libraries for optimization.  This is more specific in
// intent than poking at uname or /proc/cpuinfo.
//

// \
// Note that we add ulong after elf_addr_t because	\
// casting current->mm->context.vdso triggers a cast	\
// warning of cast from pointer to integer for		\
// COMPAT ELFCLASS32.					\
// \
// Should always be nonzero unless there's a kernel bug. \
// If we haven't determined a sensible value to give to	 \
// userspace, omit the entry:				 \
// \

// Macro flag: #define ARCH_HAS_SETUP_ADDITIONAL_PAGES

// (struct user_regs_struct *)&(dest) =		\
// (struct user_regs_struct *)regs;	\

// rv32 registers
pub type compat_elf_greg_t = compat_ulong_t;

