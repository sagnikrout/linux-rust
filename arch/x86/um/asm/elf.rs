//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/um/asm/elf.h
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


//
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

// Macro flag: #define CORE_DUMP_USE_REGSET

pub const R_386_NONE: c_int = 0;
pub const R_386_32: c_int = 1;
pub const R_386_PC32: c_int = 2;
pub const R_386_GOT32: c_int = 3;
pub const R_386_PLT32: c_int = 4;
pub const R_386_COPY: c_int = 5;
pub const R_386_GLOB_DAT: c_int = 6;
pub const R_386_JMP_SLOT: c_int = 7;
pub const R_386_RELATIVE: c_int = 8;
pub const R_386_GOTOFF: c_int = 9;
pub const R_386_GOTPC: c_int = 10;
pub const R_386_NUM: c_int = 11;
//
// This is used to ensure we don't load something for the wrong architecture.
//

// Shamelessly stolen from include/asm-i386/elf.h

// fake once used fs and gs selectors? */	\

// x86-64 relocation types, taken from asm-x86_64/elf.h

//
// This is used to ensure we don't load something for the wrong architecture.
//

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: c_int = 1;
pub const AT_SYSINFO_EHDR: c_int = 33;

pub type elf_greg_t = c_ulong;

pub type elf_fpregset_t = user_i387_struct;
pub const ELF_EXEC_PAGESIZE: c_int = 4096;

