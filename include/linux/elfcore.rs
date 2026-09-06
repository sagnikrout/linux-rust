//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/elfcore.h
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
// Definitions to generate Intel SVR4-like core files.
// These mostly have the same names as the SVR4 types with "elf_"
// tacked on the front to prevent clashes with linux definitions,
// and the typedef forms have been avoided.  This is mostly like
// the SVR4 structure, but more Linuxy, with things that Linux does
// not support and which gdb doesn't really use excluded.
//

// Lots missing
//
// The hard-coded 16 is derived from TASK_COMM_LEN, but it can't be
// changed as it is exposed to userspace. We'd better make it hard-coded
// here.
//

// (struct pt_regs *)elfregs = *regs;

extern "C" {
    pub fn ELF_CORE_COPY_TASK_REGS(_arg: t, _arg: elfregs) -> return;
}

extern "C" {
    pub fn elf_core_copy_task_fpregs(t: *mut task_struct, fpu: *mut elf_fpregset_t) -> c_int;
}

//
// These functions parameterize elf_core_dump in fs/binfmt_elf.c to write out
// extra segments containing the gate DSO contents.  Dumping its
// contents makes post-mortem fully interpretable later without matching up
// the same kernel and hardware config to see what PC values meant.
// Dumping its extra ELF program headers includes all the other information
// a debugger needs to easily find how the gate DSO was being used.
//
extern "C" {
    pub fn elf_core_extra_phdrs(cprm: *mut coredump_params) -> Elf_Half;
}
extern "C" {
    pub fn elf_core_extra_data_size(cprm: *mut coredump_params) -> usize;
}

