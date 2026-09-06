//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/xics.h
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
// Common definitions across all variants of ICP and ICS interrupt
// controllers.
//

pub const XICS_IPI: c_int = 2;
pub const XICS_IRQ_SPURIOUS: c_int = 0;
// Want a priority other than 0.  Various HW issues require this.
pub const DEFAULT_PRIORITY: c_int = 5;
//
// Mark IPIs as higher priority so we can take them inside interrupts
// FIXME: still true now?
//
pub const IPI_PRIORITY: c_int = 4;
// The least favored priority
pub const LOWEST_PRIORITY: c_uint = 0xFF;
// The number of priorities defined above
pub const MAX_NUM_PRIORITIES: c_int = 3;
// Native ICP

extern "C" {
    pub fn icp_native_init() -> c_int;
}
extern "C" {
    pub fn icp_native_flush_interrupt();
}

// PAPR ICP

extern "C" {
    pub fn icp_hv_init() -> int __init;
}

extern "C" {
    pub fn icp_opal_init() -> int __init;
}
extern "C" {
    pub fn icp_opal_flush_interrupt();
}

// ICP ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_ops {
    pub (*get_irq)(void): *mut c_uint,
    pub d): *mut *mut void (eoi)(struct irq_data,
    pub prio): *mut *mut void (set_priority)(unsigned char,
    pub (*teardown_cpu)(void): *mut c_void,
    pub (*flush_ipi)(void): *mut c_void,

    pub cpu): *mut *mut void (cause_ipi)(int,
    pub ipi_action: irq_handler_t,

}

// Native ICS
extern "C" {
    pub fn ics_native_init() -> c_int;
}

// RTAS ICS

extern "C" {
    pub fn ics_rtas_init() -> c_int;
}

// HAL ICS

extern "C" {
    pub fn ics_opal_init() -> c_int;
}

// ICS instance, hooked up to chip_data of an irq
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ics {
    pub link: list_head,
    pub hwirq): *mut *mut *mut int (check)(struct ics ics, unsigned int,
    pub vec): *mut *mut *mut void (mask_unknown)(struct ics ics, unsigned long,
    pub vec): *mut *mut *mut long (get_server)(struct ics ics, unsigned long,
    pub node): *mut *mut *mut int (host_match)(struct ics ics, struct device_node,
    pub chip: *mut irq_chip,
    pub data: [c_char; ],
}

// Commons
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xics_cppr {
    pub stack: [c_uchar; MAX_NUM_PRIORITIES],
    pub index: c_int,
}

// we only really want to set the priority when there's
// just one cppr value on the stack
//
extern "C" {
    pub fn xics_init();
}
extern "C" {
    pub fn xics_setup_cpu();
}
extern "C" {
    pub fn xics_update_irq_servers();
}
extern "C" {
    pub fn xics_set_cpu_giq(gserver: c_uint, join: c_uint);
}
extern "C" {
    pub fn xics_mask_unknown_vec(vec: c_uint);
}
extern "C" {
    pub fn xics_smp_probe();
}
extern "C" {
    pub fn xics_register_ics(ics: *mut ics);
}
extern "C" {
    pub fn xics_teardown_cpu();
}
extern "C" {
    pub fn xics_kexec_teardown_cpu(secondary: c_int);
}
extern "C" {
    pub fn xics_migrate_irqs_away();
}
extern "C" {
    pub fn icp_native_eoi(d: *mut irq_data);
}
extern "C" {
    pub fn xics_set_irq_type(d: *mut irq_data, flow_type: c_uint) -> c_int;
}
extern "C" {
    pub fn xics_retrigger(data: *mut irq_data) -> c_int;
}

