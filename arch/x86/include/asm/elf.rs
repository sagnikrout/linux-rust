//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/elf.h
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
// ELF register definitions..
//

pub type elf_greg_t = c_ulong;

pub type elf_fpregset_t = user_i387_struct;

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
// These are used to set parameters in the core dumps.
//

// x86-64 relocation types

//
// These are used to set parameters in the core dumps.
//

//
// This is used to ensure we don't load something for the wrong architecture.
//

// SVR4/i386 ABI (pages 3-31, 3-32) says that when the program starts %edx
//

//
// regs is struct pt_regs, pr_reg is elf_gregset_t (which is
// now struct_user_regs, they are different)
//

//
// This is used to ensure we don't load something for the wrong architecture.
//

// ax gets execve's return value.
// regs->ax = */ regs->bx = regs->cx = regs->dx = 0;

extern "C" {
    pub fn compat_start_thread(regs: *mut pt_regs, new_ip: u32, new_sp: u32, x32: bool);
}

extern "C" {
    pub fn set_personality_ia32(_arg: bool);
}

//
// regs is struct pt_regs, pr_reg is elf_gregset_t (which is
// now struct_user_regs, they are different). Assumes current is the process
// getting dumped.
//

// I'm not sure if we can use '-' here

extern "C" {
    pub fn set_personality_64bit();
}

// Macro flag: #define CORE_DUMP_USE_REGSET
pub const ELF_EXEC_PAGESIZE: c_int = 4096;
//
// This is the base location for PIE (ET_DYN with INTERP) loads. On
// 64-bit, this is above 4GB to leave the entire 32-bit address
// space open for things that want to use the area for 32-bit pointers.
//

// This yields a mask that user programs can use to figure out what

//
// HWCAP2 supplies mask with kernel enabled CPU features, so that
// the application can discover that it can safely use them.
// The bits are defined in uapi/asm/hwcap2.h.
//

// This yields a string that ld.so will use to load implementation

//
// An executable for which elf_read_implies_exec() returns TRUE will
// have the READ_IMPLIES_EXEC personality flag set automatically.
//
// The decision process for determining the results are:
//
// CPU: | lacks NX*  | has NX, ia32     | has NX, x86_64 |
// ELF:                 |            |                  |                |
// ---------------------|------------|------------------|----------------|
// missing PT_GNU_STACK | exec-all   | exec-all         | exec-none      |
// PT_GNU_STACK == RWX  | exec-stack | exec-stack       | exec-stack     |
// PT_GNU_STACK == RW   | exec-none  | exec-none        | exec-none      |
//
// exec-all  : all PROT_READ user mappings are executable, except when
// backed by files on a noexec-filesystem.
// exec-none : only PROT_EXEC user mappings are executable.
// exec-stack: only the stack and PROT_EXEC user mappings are executable.
//
// *this column has no architectural effect: NX markings are ignored by
// hardware, but may have behavioral effects when "wants X" collides with
// "cannot be X" constraints in memory permission flags, as in
// https://lkml.kernel.org/r/20190418055759.GA3155@mellanox.com
//

//
// True on X86_32 or when emulating IA32 on X86_64
//
extern "C" {
    pub fn task_size_32bit() -> c_ulong;
}
extern "C" {
    pub fn task_size_64bit(full_addr_space: c_int) -> c_ulong;
}
extern "C" {
    pub fn get_mmap_base(is_legacy: c_int) -> c_ulong;
}
extern "C" {
    pub fn mmap_address_hint_valid(addr: c_ulong, len: c_ulong) -> bool;
}
extern "C" {
    pub fn get_sigframe_size() -> c_ulong;
}

// update AT_VECTOR_SIZE_ARCH if the number of NEW_AUX_ENT entries changes

// 16GB for 64bit, 8MB for 32bit

// As a historical oddity, the x32 and x86_64 vDSOs are controlled together.

pub const AT_SYSINFO: c_int = 32;

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: c_int = 1;

extern "C" {
    pub fn arch_syscall_is_vdso_sigreturn(regs: *mut pt_regs) -> bool;
}
// Do not change the values. See get_align_mask()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum align_flags {
    ALIGN_VA_32	= BIT(0),
    ALIGN_VA_64	= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct va_alignment {
    pub flags: c_int,
    pub mask: c_ulong,
    pub bits: c_ulong,
    pub ____cacheline_aligned: },
    pub va_align: extern struct va_alignment,
