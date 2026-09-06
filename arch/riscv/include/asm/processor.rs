//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/processor.h
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
// Copyright (C) 2012 Regents of the University of California
//

pub const STACK_ALIGN: c_int = 16;

pub const user_max_virt_addr(): c_int = 0;

//
// This decides where the kernel will search for a free chunk of vm
// space during mmap's.
//

//
// We use a flag to track in-kernel Vector context. Currently the flag has the
// following meaning:
//
// - bit 0: indicates whether the in-kernel Vector context is active. The
// activation of this state disables the preemption. On a non-RT kernel, it
// also disable bh.
// - bit 1: tells kvm that the vcpu process has guest context saved in vcpu's
// context memory and need to be restore upon returing back to the guest.
// - bit 2: represents that the vector context has now loaded and belongs to
// the guest kernel. Any non-scheduler context saving routing needs to save
// the register file to vcpu's context memory. The bit is set upon returing
// back to the guest and cleared after loading the host's vector context.
// - bits 8: is used for tracking preemptible kernel-mode Vector, when
// RISCV_ISA_V_PREEMPTIVE is enabled. Calling kernel_vector_begin() does not
// disable the preemption if the thread's kernel_vstate.datap is allocated.
// Instead, the kernel set this bit field. Then the trap entry/exit code
// knows if we are entering/exiting the context that owns preempt_v.
// - 0: the task is not using preempt_v
// - 1: the task is actively using preempt_v. But whether does the task own
// the preempt_v context is decided by bits in RISCV_V_CTX_DEPTH_MASK.
// - bit 16-23 are RISCV_V_CTX_DEPTH_MASK, used by context tracking routine
// when preempt_v starts:
// - 0: the task is actively using, and own preempt_v context.
// - non-zero: the task was using preempt_v, but then took a trap within.
// Thus, the task does not own preempt_v. Any use of Vector will have to
// save preempt_v, if dirty, and fallback to non-preemptible kernel-mode
// Vector.
// - bit 29: The thread voluntarily calls schedule() while holding an active
// preempt_v. All preempt_v context should be dropped in such case because
// V-regs are caller-saved. Only sstatus.VS=ON is persisted across a
// schedule() call.
// - bit 30: The in-kernel preempt_v context is saved, and is required to be
// restored when returning to the context that owns the preempt_v.
// - bit 31: The in-kernel preempt_v context is dirty, as signaled by the
// trap entry code. Any context switches out-of current task need to save
// it to the task's in-kernel V context. Also, any traps nesting on-top-of
// preempt_v requesting to use V needs a save.
//
pub const RISCV_V_CTX_DEPTH_MASK: c_uint = 0x00ff0000;
pub const RISCV_V_CTX_UNIT_DEPTH: c_uint = 0x00010000;
pub const RISCV_KERNEL_MODE_V: c_uint = 0x00000001;
pub const RISCV_V_VCPU_NEED_RESTORE: c_uint = 0x00000002;
pub const RISCV_V_VCPU_CTX: c_uint = 0x00000004;
pub const RISCV_PREEMPT_V: c_uint = 0x00000100;
pub const RISCV_PREEMPT_V_DIRTY: c_uint = 0x80000000;
pub const RISCV_PREEMPT_V_NEED_RESTORE: c_uint = 0x40000000;
pub const RISCV_PREEMPT_V_IN_SCHEDULE: c_uint = 0x20000000;
// CPU-specific state of a task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_struct {
// Callee-saved registers
    pub ra: c_ulong,
    pub /: *mut *mut unsigned long sp; / Kernel mode stack,
    pub /: *mut *mut unsigned long s[12]; / s[0]: frame pointer,
    pub fstate: __riscv_d_ext_state,
    pub bad_cause: c_ulong,
    pub envcfg: c_ulong,
    pub sum: c_ulong,
    pub riscv_v_flags: u32,
    pub vstate_ctrl: u32,
    pub vstate: __riscv_v_ext_state,
    pub align_ctl: c_ulong,
    pub kernel_vstate: __riscv_v_ext_state,

// Flush the icache on migration
    pub force_icache_flush: bool,
// A forced icache flush is not needed if migrating to the previous cpu.
    pub prev_cpu: c_uint,

    pub srmcfg: u32,

}

// Whitelist the fstate from the task_struct for hardened usercopy
// offset = offsetof(struct thread_struct, fstate);
// size = sizeof_field(struct thread_struct, fstate);

// Macro flag: #define ARCH_HAS_PREFETCH
extern "C" {
    pub fn __volatile__("memory": PREFETCH_ASM(%0) : : "r" (x) :) -> __asm__;
}
// Macro flag: #define ARCH_HAS_PREFETCHW
extern "C" {
    pub fn __volatile__("memory": PREFETCHW_ASM(%0) : : "r" (x) :) -> __asm__;
}

// Do necessary setup to start up a newly executed thread.
extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn __volatile__(_arg: "wfi") -> __asm__;
}
extern "C" {
    pub fn riscv_of_processor_hartid(node: *mut device_node, hartid: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn riscv_early_of_processor_hartid(node: *mut device_node, hartid: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn riscv_of_parent_hartid(node: *mut device_node, hartid: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn riscv_fill_hwcap();
}
extern "C" {
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int;
}

// Userspace interface for PR_RISCV_V_{SET,GET}_VS prctl()s:

extern "C" {
    pub fn riscv_v_vstate_ctrl_set_current(arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn riscv_v_vstate_ctrl_get_current() -> c_long;
}

extern "C" {
    pub fn get_unalign_ctl(tsk: *mut task_struct, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_unalign_ctl(tsk: *mut task_struct, val: c_uint) -> c_int;
}

extern "C" {
    pub fn riscv_set_icache_flush_ctx(ctx: c_ulong, per_thread: c_ulong) -> c_int;
}

// PR_{SET,GET}_TAGGED_ADDR_CTRL prctl
extern "C" {
    pub fn set_tagged_addr_ctrl(task: *mut task_struct, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn get_tagged_addr_ctrl(task: *mut task_struct) -> c_long;
}

