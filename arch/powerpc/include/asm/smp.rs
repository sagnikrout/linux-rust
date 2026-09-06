//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/smp.h
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
// smp.h: PowerPC-specific SMP code.
//
// Original was a copy of sparc smp.h.  Now heavily modified
// for PPC.
//
// Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
// Copyright (C) 1996-2001 Cort Dougan <cort@fsmlabs.com>
//

extern "C" {
    pub fn cpu_to_chip_id(cpu: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_ops_t {
    pub msg): *mut *mut void (message_pass)(int cpu, int,

    pub cpu): *mut *mut void (cause_ipi)(int,

    pub cpu): *mut *mut int (cause_nmi_ipi)(int,
    pub (*probe)(void): *mut c_void,
    pub nr): *mut *mut int (kick_cpu)(int,
    pub nr): *mut *mut int (prepare_cpu)(int,
    pub nr): *mut *mut void (setup_cpu)(int,
    pub (*bringup_done)(void): *mut c_void,
    pub (*take_timebase)(void): *mut c_void,
    pub (*give_timebase)(void): *mut c_void,
    pub (*cpu_disable)(void): *mut c_int,
    pub nr): *mut *mut void (cpu_die)(unsigned int,
    pub nr): *mut *mut int (cpu_bootable)(unsigned int,

    pub (*cpu_offline_self)(void): *mut c_void,

}

extern "C" {
    pub fn start_secondary(unused: *mut c_void);
}
extern "C" {
    pub fn smp_send_nmi_ipi(cpu: c_int, ): *mut *mut void (fn)(struct pt_regs, delay_us: u64) -> c_int;
}
extern "C" {
    pub fn smp_send_safe_nmi_ipi(cpu: c_int, ): *mut *mut void (fn)(struct pt_regs, delay_us: u64) -> c_int;
}
extern "C" {
    pub fn smp_send_debugger_break();
}
extern "C" {
    pub fn start_secondary_resume() -> void __noreturn;
}
extern "C" {
    pub fn smp_generic_give_timebase();
}
extern "C" {
    pub fn smp_generic_take_timebase();
}

extern "C" {
    pub fn generic_cpu_disable() -> c_int;
}
extern "C" {
    pub fn generic_cpu_die(cpu: c_uint);
}
extern "C" {
    pub fn generic_set_cpu_dead(cpu: c_uint);
}
extern "C" {
    pub fn generic_set_cpu_up(cpu: c_uint);
}
extern "C" {
    pub fn generic_check_cpu_restart(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn is_cpu_dead(cpu: c_uint) -> c_int;
}

// 32-bit

extern "C" {
    pub fn per_cpu(_arg: cpu_sibling_map, _arg: cpu) -> return;
}
extern "C" {
    pub fn per_cpu(_arg: cpu_core_map, _arg: cpu) -> return;
}
extern "C" {
    pub fn per_cpu(_arg: cpu_l2_cache_map, _arg: cpu) -> return;
}
extern "C" {
    pub fn per_cpu(_arg: cpu_smallcore_map, _arg: cpu) -> return;
}
extern "C" {
    pub fn cpu_to_core_id(cpu: c_int) -> c_int;
}

extern "C" {
    pub fn per_cpu(_arg: cpu_smallcore_map, _arg: cpu) -> return;
}
extern "C" {
    pub fn per_cpu(_arg: cpu_sibling_map, _arg: cpu) -> return;
}

// Since OpenPIC has only 4 IPIs, we use slightly different message numbers.
//
// Make sure this matches openpic_request_IPIs in open_pic.c, or what shows up
// in /proc/interrupts will be wrong!!! --Troy
pub const PPC_MSG_CALL_FUNCTION: c_int = 0;
pub const PPC_MSG_RESCHEDULE: c_int = 1;
pub const PPC_MSG_TICK_BROADCAST: c_int = 2;
pub const PPC_MSG_NMI_IPI: c_int = 3;
// This is only used by the powernv kernel
pub const PPC_MSG_RM_HOST_ACTION: c_int = 4;

extern "C" {
    pub fn smp_handle_nmi_ipi(regs: *mut pt_regs) -> c_int;
}

// for irq controllers that have dedicated ipis per message (4)
extern "C" {
    pub fn smp_request_message_ipi(virq: c_int, message: c_int) -> c_int;
}
// for irq controllers with only a single ipi
extern "C" {
    pub fn smp_muxed_ipi_message_pass(cpu: c_int, msg: c_int);
}
extern "C" {
    pub fn smp_muxed_ipi_set_message(cpu: c_int, msg: c_int);
}
extern "C" {
    pub fn smp_ipi_demux() -> irqreturn_t;
}
extern "C" {
    pub fn smp_ipi_demux_relaxed() -> irqreturn_t;
}
extern "C" {
    pub fn smp_init_pSeries();
}
extern "C" {
    pub fn smp_init_cell();
}
extern "C" {
    pub fn smp_setup_cpu_maps();
}
extern "C" {
    pub fn __cpu_disable() -> c_int;
}
extern "C" {
    pub fn __cpu_die(cpu: c_uint);
}

// for UP

// Macro flag: #define smp_setup_cpu_maps()
pub const thread_group_shares_l2: c_int = 0;
pub const thread_group_shares_l3: c_int = 0;
extern "C" {
    pub fn cpumask_of(_arg: cpu) -> return;
}
extern "C" {
    pub fn cpumask_of(_arg: cpu) -> return;
}
extern "C" {
    pub fn cpumask_of(_arg: cpu) -> return;
}

// 32-bit

extern "C" {
    pub fn smp_release_cpus();
}

extern "C" {
    pub fn smp_mpic_probe();
}
extern "C" {
    pub fn smp_mpic_setup_cpu(cpu: c_int);
}
extern "C" {
    pub fn smp_generic_kick_cpu(nr: c_int) -> c_int;
}
extern "C" {
    pub fn smp_generic_cpu_bootable(nr: c_uint) -> c_int;
}
extern "C" {
    pub fn smp_generic_give_timebase();
}
extern "C" {
    pub fn smp_generic_take_timebase();
}
extern "C" {
    pub fn arch_send_call_function_single_ipi(cpu: c_int);
}
extern "C" {
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
}
// Definitions relative to the secondary CPU spin loop
// and entry point. Not all of them exist on both 32 and
// 64-bit but defining them all here doesn't harm
//
extern "C" {
    pub fn generic_secondary_smp_init();
}
extern "C" {
    pub fn __early_start();
}

