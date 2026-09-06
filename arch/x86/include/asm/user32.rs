//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/user32.h
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
// IA32 compatible user structures for ptrace.
// These should be used for 32bit coredumps too.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_i387_ia32_struct {
    pub cwd: u32,
    pub swd: u32,
    pub twd: u32,
    pub fip: u32,
    pub fcs: u32,
    pub foo: u32,
    pub fos: u32,
    pub /: *mut *mut *mut u32 st_space[20]; / 810 bytes for each FP-reg = 80 bytes,
}

// FSAVE frame with extensions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user32_fxsr_struct {
    pub cwd: c_ushort,
    pub swd: c_ushort,
    pub /: *mut *mut unsigned short twd; / not compatible to 64bit twd,
    pub fop: c_ushort,
    pub fip: c_int,
    pub fcs: c_int,
    pub foo: c_int,
    pub fos: c_int,
    pub mxcsr: c_int,
    pub reserved: c_int,
    pub /: *mut *mut *mut int st_space[32]; / 816 bytes for each FP-reg = 128 bytes,
    pub /: *mut *mut *mut int xmm_space[32]; / 816 bytes for each XMM-reg = 128 bytes,
    pub padding: [c_int; 56],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_regs_struct32 {
    pub eax: __u32 ebx, ecx, edx, esi, edi, ebp,,
    pub __es: unsigned short ds, __ds, es,,
    pub __gs: unsigned short fs, __fs, gs,,
    pub eip: __u32 orig_eax,,
    pub __cs: unsigned short cs,,
    pub esp: __u32 eflags,,
    pub __ss: unsigned short ss,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user32 {
    pub /: *mut *mut user_regs_struct32 regs; / Where the registers are actually stored,
    pub /: *mut *mut int u_fpvalid; / True if math co-processor being used.,
// for this mess. Not yet used.
    pub /: *mut *mut user_i387_ia32_i387; / Math Co-processor registers.,
// The rest of this junk is to help gdb figure out what goes where
    pub /: *mut *mut __u32 u_tsize; / Text segment size (pages).,
    pub /: *mut *mut __u32 u_dsize; / Data segment size (pages).,
    pub /: *mut *mut __u32 u_ssize; / Stack segment size (pages).,
    pub /: *mut *mut __u32 start_code; / Starting virtual address of text.,
    pub area.: *mut *mut __u32 start_stack; / Starting virtual address of stack,
    pub /: *mut *mut __u32 signal; / Signal that caused the core dump.,
    pub /: *mut *mut int reserved; / No __u32er used,
    pub /: *mut *mut __u32 u_ar0; / Used by gdb to help find the values for,
// the registers.
    pub /: *mut *mut __u32 u_fpstate; / Math Co-processor pointer.,
    pub /: *mut *mut __u32 magic; / To uniquely identify a core file,
    pub /: *mut *mut char u_comm[32]; / User command that was responsible,
    pub u_debugreg: [c_int; 8],
}
