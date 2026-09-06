//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/user_64.h
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

// Core file format: The core file is written in such a way that gdb
//
// Pentium III FXSR, SSE support
// Gareth Hughes <gareth@valinux.com>, May 2000
//
// Provide support for the GDB 5.0+ PTRACE_{GET|SET}FPXREGS requests for
// interacting with the FXSR-format floating point environment.  Floating
// point data can be accessed in the regular format in the usual manner,
// and both the standard and SIMD floating point data can be accessed via
// the new ptrace requests.  In either case, changes to the FPU environment
// will be reflected in the task's state as expected.
//
// x86-64 support by Andi Kleen.
//
// This matches the 64bit FXSAVE format as defined by AMD. It is the same
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_i387_struct {
    pub cwd: c_ushort,
    pub swd: c_ushort,
    pub as: *mut *mut unsigned short twd; / Note this is not the same,
    pub fop: c_ushort,
    pub rip: __u64,
    pub rdp: __u64,
    pub mxcsr: __u32,
    pub mxcsr_mask: __u32,
    pub /: *mut *mut *mut __u32 st_space[32]; / 816 bytes for each FP-reg = 128 bytes,
    pub /: *mut *mut *mut __u32 xmm_space[64]; / 1616 bytes for each XMM-reg = 256 bytes,
    pub padding: [__u32; 24],
}

//
// Segment register layout in coredumps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_regs_struct {
    pub r15: c_ulong,
    pub r14: c_ulong,
    pub r13: c_ulong,
    pub r12: c_ulong,
    pub bp: c_ulong,
    pub bx: c_ulong,
    pub r11: c_ulong,
    pub r10: c_ulong,
    pub r9: c_ulong,
    pub r8: c_ulong,
    pub ax: c_ulong,
    pub cx: c_ulong,
    pub dx: c_ulong,
    pub si: c_ulong,
    pub di: c_ulong,
    pub orig_ax: c_ulong,
    pub ip: c_ulong,
    pub cs: c_ulong,
    pub flags: c_ulong,
    pub sp: c_ulong,
    pub ss: c_ulong,
    pub fs_base: c_ulong,
    pub gs_base: c_ulong,
    pub ds: c_ulong,
    pub es: c_ulong,
    pub fs: c_ulong,
    pub gs: c_ulong,
}

// When the kernel dumps core, it starts by dumping the user struct -
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user {
// We start with the registers, to mimic the way that "memory" is returned
    pub /: *mut *mut user_regs_regs; / Where the registers are actually stored,
// ptrace does not yet supply these.  Someday....
    pub /: *mut *mut int u_fpvalid; / True if math co-processor being used.,
// for this mess. Not yet used.
    pub pad0: c_int,
    pub /: *mut *mut user_i387_i387; / Math Co-processor registers.,
// The rest of this junk is to help gdb figure out what goes where
    pub /: *mut *mut unsigned long int u_tsize; / Text segment size (pages).,
    pub /: *mut *mut unsigned long int u_dsize; / Data segment size (pages).,
    pub /: *mut *mut unsigned long int u_ssize; / Stack segment size (pages).,
    pub /: *mut *mut unsigned long start_code; / Starting virtual address of text.,
    pub area.: *mut *mut unsigned long start_stack; / Starting virtual address of stack,
    pub /: *mut *mut long int signal; / Signal that caused the core dump.,
    pub /: *mut *mut int reserved; / No longer used,
    pub pad1: c_int,
    pub /: *mut *mut unsigned long u_ar0; / Used by gdb to help find the values for,
// the registers.
    pub /: *mut *mut *mut user_i387_u_fpstate; / Math Co-processor pointer.,
    pub /: *mut *mut unsigned long magic; / To uniquely identify a core file,
    pub /: *mut *mut char u_comm[32]; / User command that was responsible,
    pub u_debugreg: [c_ulong; 8],
    pub /: *mut *mut unsigned long error_code; / CPU error code or 0,
    pub /: *mut *mut unsigned long fault_address; / CR3 or 0,
}
