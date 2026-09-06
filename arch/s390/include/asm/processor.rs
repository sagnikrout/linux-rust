//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/processor.h
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
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Hartmut Penner (hp@de.ibm.com),
// Martin Schwidefsky (schwidefsky@de.ibm.com)
//
// Derived from "include/asm-i386/processor.h"
// Copyright (C) 1994, Linus Torvalds
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu {
    pub /: *mut *mut unsigned long ec_mask; / bit mask for ec_xxx functions,
    pub /: *mut *mut unsigned long ec_clk; / sigp timestamp for ec_xxx,
    pub /: *mut *mut unsigned long flags; / per CPU flags,
    pub /: *mut *mut signed char state; / physical cpu state,
    pub /: *mut *mut signed char polarization; / physical polarization,
    pub /: *mut *mut u16 address; / physical cpu address,
}

extern "C" {
    pub fn long(regs: *mut *mut sys_call_ptr_t)(struct pt_regs) -> typedef;
}
extern "C" {
    pub fn test_bit(_arg: flag, _arg: &this_pcpu()->flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: flag, _arg: &this_pcpu()->flags) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: flag, _arg: &this_pcpu()->flags) -> return;
}
//
// Test CIF flag of another CPU. The caller needs to ensure that
// CPU hotplug can not happen, e.g. by disabling preemption.
//
extern "C" {
    pub fn test_bit(_arg: flag, _arg: &per_cpu(pcpu_devices, _arg: cpu).flags) -> return;
}
extern "C" {
    pub fn volatile((*ptr): *mut "stidp %0" : "=Q") -> asm;
}
extern "C" {
    pub fn volatile((timer): "stpt %[timer]" : [timer] "=Q") -> asm;
}
extern "C" {
    pub fn s390_adjust_jiffies();
}
extern "C" {
    pub fn s390_update_cpu_mhz();
}
extern "C" {
    pub fn cpu_detect_mhz_feature();
}
extern "C" {
    pub fn execve_tail();
}
extern "C" {
    pub fn vdso_text_size() -> c_ulong;
}
extern "C" {
    pub fn vdso_size() -> c_ulong;
}

// Macro flag: #define HAVE_ARCH_PICK_MMAP_LAYOUT

//
// Thread structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_struct {
    pub acrs: [c_uint; NUM_ACRS],
    pub /: *mut *mut unsigned long ksp; / kernel stack pointer,
    pub /: *mut *mut unsigned long user_timer; / task cputime in user space,
    pub /: *mut *mut unsigned long guest_timer; / task cputime in kvm guest,
    pub /: *mut *mut unsigned long system_timer; / task cputime in kernel space,
    pub /: *mut *mut unsigned long hardirq_timer; / task cputime in hardirq context,
    pub /: *mut *mut unsigned long softirq_timer; / task cputime in softirq context,
    pub /: *mut *mut teid gmap_teid; / address and flags of last gmap fault,
    pub /: *mut *mut unsigned int gmap_int_code; / int code of last gmap fault,
    pub /: *mut *mut int ufpu_flags; / user fpu flags,
    pub /: *mut *mut int kfpu_flags; / kernel fpu flags,
// Per-thread information related to debugging
    pub /: *mut *mut per_regs per_user; / User specified PER registers,
    pub /: *mut *mut per_event per_event; / Cause of the last PER trap,
    pub /: *mut *mut unsigned long per_flags; / Flags to control debug behavior,
    pub /: *mut *mut unsigned int system_call; / system call number in signal,
    pub /: *mut *mut unsigned long last_break; / last breaking-event-address.,
// pfault_wait is used to block the process on a pfault event
    pub pfault_wait: c_ulong,
    pub list: list_head,
// cpu runtime instrumentation
    pub ri_cb: *mut runtime_instr_cb,
    pub /: *mut *mut *mut gs_cb gs_cb; / Current guarded storage cb,
    pub /: *mut *mut *mut gs_cb gs_bc_cb; / Broadcast guarded storage cb,
    pub /: *mut *mut pgm_tdb trap_tdb; / Transaction abort diagnose block,
    pub /: *mut *mut fpu ufpu; / User FP and VX register save area,
    pub /: *mut *mut fpu kfpu; / Kernel FP and VX register save area,
}

// Flag to disable transactions.

// Flag to enable random transaction aborts.

// Flag to specify random transaction abort mode:
// - abort each transaction at a random instruction before TEND if set.
// - abort random transactions at a random instruction if cleared.
//

pub type thread_struct = thread_struct;
pub const ARCH_MIN_TASKALIGN: c_int = 8;

//
// Do necessary setup to start up a new thread.
//

extern "C" {
    pub fn show_registers(regs: *mut pt_regs);
}
extern "C" {
    pub fn show_cacheinfo(m: *mut seq_file);
}
// Free guarded storage control block
extern "C" {
    pub fn guarded_storage_release(tsk: *mut task_struct);
}
extern "C" {
    pub fn gs_load_bc_cb(regs: *mut pt_regs);
}
extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}

// Has task runtime instrumentation enabled ?

// avoid using global register due to gcc bug in versions < 8.4

extern "C" {
    pub fn volatile(%0: "lgr, (sp): 15" : "=d") -> asm;
}
extern "C" {
    pub fn volatile((cpu_address): "stap %0" : "=Q") -> asm;
}
pub const ECAG_CACHE_ATTRIBUTE: c_int = 0;
pub const ECAG_CPU_ATTRIBUTE: c_int = 1;
extern "C" {
    pub fn volatile(%0: "ecag, _arg: 0, parm): 0(%1)" : "=d" (val) : "a" (asi << 8 |) -> asm;
}
extern "C" {
    pub fn volatile((key): "spka 0(%0)" : : "d") -> asm;
}
//
// Set PSW to specified value.
//
extern "C" {
    pub fn volatile("cc": "lpswe %0" : : "Q" (psw) :) -> asm;
}
//
// Set PSW mask to specified value, while leaving the
// PSW addr pointing to the next instruction.
//
// Extract current PSW mask
//
extern "C" {
    pub fn volatile(%0: "epsw, (reg1): %1" : "=d", (reg2): "=a") -> asm;
}

//
// Rewind PSW instruction address by specified number of bytes.
//
extern "C" {
    pub fn __rewind_psw(_arg: psw, _arg: -ilen) -> return;
}
//
// Function to drop a processor into disabled wait state
//
pub const ARCH_LOW_ADDRESS_LIMIT: c_uint = 0x7fffffffUL;
extern "C" {
    pub fn arch_irqs_disabled_flags(_arg: regs->psw.mask) -> return;
}

