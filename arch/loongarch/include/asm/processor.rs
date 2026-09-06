//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/processor.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

pub const TASK_SIZE: c_uint = 0x80000000UL;

pub const TASK_IS_32BIT_ADDR: c_int = 1;

pub const TASK_SIZE32: c_uint = 0x100000000UL;

extern "C" {
    pub fn stack_top() -> c_ulong;
}

//
// This decides where the kernel will search for a free chunk of vm
// space during mmap's.
//

pub const FPU_REG_WIDTH: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub union fpureg {
    pub 32]: __u32 val32[FPU_REG_WIDTH /,
    pub 64]: __u64 val64[FPU_REG_WIDTH /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_fpu {
    pub fpr: [fpureg; NUM_FPU_REGS],
    pub /: *mut *mut uint64_t fcc; / 8x8,
    pub fcsr: u32,
    pub ftop: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_lbt {
// Scratch registers
    pub scr0: c_ulong,
    pub scr1: c_ulong,
    pub scr2: c_ulong,
    pub scr3: c_ulong,
// Eflags register
    pub eflags: c_ulong,
}

pub const ARCH_MIN_TASKALIGN: c_int = 32;
//
// If you change thread_struct remember to change the #defines below too!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_struct {
// Main processor registers.
    pub /: *mut *mut unsigned long reg01, reg03, reg22; / ra sp fp,
    pub /: *mut *mut unsigned long reg23, reg24, reg25, reg26; / s0-s3,
    pub /: *mut *mut unsigned long reg27, reg28, reg29, reg30, reg31; / s4-s8,
// __schedule() return address / call frame address
    pub sched_ra: c_ulong,
    pub sched_cfa: c_ulong,
// CSR registers
    pub csr_prmd: c_ulong,
    pub csr_crmd: c_ulong,
    pub csr_euen: c_ulong,
    pub csr_ecfg: c_ulong,
    pub /: *mut *mut unsigned long csr_badvaddr; / Last user fault,
// Other stuff associated with the thread.
    pub trap_nr: c_ulong,
    pub error_code: c_ulong,
    pub /: *mut *mut unsigned long single_step; / Used by PTRACE_SINGLESTEP,
    pub vdso: *mut loongarch_vdso_info,
//
// FPU & vector registers, must be at the last of inherited
// context because they are conditionally copied at fork().
//
    pub FPU_ALIGN: loongarch_fpu fpu,
    pub /: *mut *mut loongarch_lbt lbt; / Also conditionally copied,
// Hardware breakpoints pinned to this task.
    pub hbp_break: [*mut perf_event; LOONGARCH_MAX_BRP],
    pub hbp_watch: [*mut perf_event; LOONGARCH_MAX_WRP],
}

// \
// Main processor registers				\
// \
// Other stuff associated with the process		\
// \
// FPU & vector registers				\
// \
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idle_boot_override {

    extern unsigned long		boot_option_idle_override;
//
// Do necessary setup to start up a newly executed thread.
//
    extern void start_thread(struct pt_regs *regs, unsigned long pc, unsigned long sp);

    unsigned long __get_wchan(struct task_struct *p);

    THREAD_SIZE - sizeof(struct pt_regs))

// Macro flag: #define ARCH_HAS_PREFETCH

// Macro flag: #define ARCH_HAS_PREFETCHW

