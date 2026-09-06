//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/module.h
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
// Thanks to Paul M for explaining this.
//
// PPC can only do rel jumps += 32MB, and often the kernel and other
// modules are further away than this.  So, we jump to a table of
// trampolines attached to the module (the Procedure Linkage Table)
// whenever that happens.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_plt_entry {
// 16 byte jump instruction sequence (4 instructions)
    pub jump: [c_uint; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_arch_specific {

    pub /: *mut *mut unsigned int stubs_section; / Index of stubs section in module,
    pub /: *mut *mut unsigned int stub_count; / Number of stubs used,

    pub /: *mut *mut unsigned int got_section; / What section is the GOT?,
    pub /: *mut *mut unsigned int pcpu_section; / .data..percpu section,

    pub /: *mut *mut unsigned int toc_section; / What section is the TOC?,
    pub /: *mut *mut bool toc_fixed; / Have we fixed up .TOC.?,

// For module function descriptor dereference
    pub start_opd: c_ulong,
    pub end_opd: c_ulong,

// Indices of PLT sections within module.
    pub core_plt_section: c_uint,
    pub init_plt_section: c_uint,

    pub tramp: c_ulong,
    pub tramp_regs: c_ulong,

    pub ool_stubs: *mut ftrace_ool_stub,
    pub ool_stub_count: c_uint,
    pub ool_stub_index: c_uint,

}

//
// Select ELF headers.
// Make empty sections for module_frob_arch_sections to expand.
//

extern "C" {
    pub fn module_finalize_ftrace(mod: *mut module, sechdrs: *const Elf_Shdr) -> c_int;
}

