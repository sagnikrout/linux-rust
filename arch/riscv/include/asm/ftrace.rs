//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/ftrace.h
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
// Copyright (C) 2017 Andes Technology Corporation
//
// The graph frame test is not possible if CONFIG_FRAME_POINTER is not enabled.
// Check arch/riscv/kernel/mcount.S for detail.
//

// Macro flag: #define HAVE_FUNCTION_GRAPH_FP_TEST

pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 1;

extern "C" {
    pub fn _mcount();
}
extern "C" {
    pub fn ftrace_call_adjust(addr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn arch_ftrace_get_symaddr(fentry_ip: c_ulong) -> c_ulong;
}

//
// Let's do like x86/arm64 and ignore the compat syscalls.
//
// Macro flag: #define ARCH_TRACE_IGNORE_COMPAT_SYSCALLS
extern "C" {
    pub fn is_compat_task() -> return;
}
// Macro flag: #define ARCH_HAS_SYSCALL_MATCH_SYM_NAME
//
// Since all syscall functions have __riscv_ prefix, we must skip it.
// However, as we described above, we decided to ignore compat
// syscalls, so we don't care about __riscv_compat_ prefix here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_arch_ftrace {
}

//
// A general call in RISC-V is a pair of insts:
// 1) auipc: setting high-20 pc-related bits to ra register
// 2) jalr: setting low-12 offset to ra, jump to ra, and set ra to
// return address (original pc + 4)
//
// The first 2 instructions for each tracable function is compiled to 2 nop
// instructions. Then, the kernel initializes the first instruction to auipc at
// boot time (<ftrace disable>). The second instruction is patched to jalr to
// start the trace.
//
// <Image>:
// 0: nop
// 4: nop
//
// <ftrace enable>:
// 0: auipc  t0, 0x?
// 4: jalr   t0, ?(t0)
//
// <ftrace disable>:
// 0: auipc  t0, 0x?
// 4: nop
//
// Dynamic ftrace generates probes to call sites, so we must deal with
// both auipc and jalr at the same time.
//

pub const JALR_SHIFT: c_int = 20;

//
// Only the jalr insn in the auipc+jalr is patched, so we make it 4
// bytes here.
//
pub const MCOUNT_INSN_SIZE: c_int = 4;
pub const MCOUNT_AUIPC_SIZE: c_int = 4;
pub const MCOUNT_JALR_SIZE: c_int = 4;
pub const MCOUNT_NOP4_SIZE: c_int = 4;
extern "C" {
    pub fn ftrace_init_nop(mod: *mut module, rec: *mut dyn_ftrace) -> c_int;
}

// Macro flag: #define HAVE_ARCH_FTRACE_REGS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __arch_ftrace_regs {
    pub epc: c_ulong,
    pub ra: c_ulong,
    pub sp: c_ulong,
    pub s0: c_ulong,
    pub t1: c_ulong,

    pub direct_tramp: c_ulong,
    pub args: [c_ulong; 8],
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
    pub a4: c_ulong,
    pub a5: c_ulong,
    pub a6: c_ulong,
    pub a7: c_ulong,

    pub t2: c_ulong,
    pub t3: c_ulong,
    pub t4: c_ulong,
    pub t5: c_ulong,
    pub t6: c_ulong,

}

// fregs)
extern "C" {
    pub fn ftrace_regs_query_register_offset(name: *const c_char) -> c_int;
}

