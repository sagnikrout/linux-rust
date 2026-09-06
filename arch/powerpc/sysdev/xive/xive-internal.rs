//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/xive/xive-internal.h
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
// Copyright 2016,2017 IBM Corporation.
//
// A "disabled" interrupt should never fire, to catch problems
// we set its logical number to this
//
pub const XIVE_BAD_IRQ: c_uint = 0x7fffffff;

// Each CPU carry one of these with various per-CPU state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xive_cpu {

// HW irq number and data of IPI
    pub hw_ipi: u32,
    pub ipi_data: xive_irq_data,

    pub chip_id: c_int,
// Queue datas. Only one is populated
pub const XIVE_MAX_QUEUES: c_int = 8;
    pub queue: [xive_q; XIVE_MAX_QUEUES],
//
// Pending mask. Each bit corresponds to a priority that
// potentially has pending interrupts.
//
    pub pending_prio: u8,
// Cache of HW CPPR
    pub cppr: u8,
}

// Backend ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xive_ops {
    pub data): *mut *mut int (populate_irq_data)(u32 hw_irq, struct xive_irq_data,
    pub sw_irq): *mut *mut int (configure_irq)(u32 hw_irq, u32 target, u8 prio, u32,
    pub sw_irq): *mut u32,
    pub prio): *mut *mut *mut int (setup_queue)(unsigned int cpu, struct xive_cpu xc, u8,
    pub prio): *mut *mut *mut void (cleanup_queue)(unsigned int cpu, struct xive_cpu xc, u8,
    pub xc): *mut *mut void (prepare_cpu)(unsigned int cpu, struct xive_cpu,
    pub xc): *mut *mut void (setup_cpu)(unsigned int cpu, struct xive_cpu,
    pub xc): *mut *mut void (teardown_cpu)(unsigned int cpu, struct xive_cpu,
    pub np): *mut *mut bool (match)(struct device_node,
    pub (*shutdown)(void): *mut c_void,
    pub xc): *mut *mut void (update_pending)(struct xive_cpu,
    pub hw_irq): *mut *mut void (sync_source)(u32,
    pub write): *mut *mut u64 (esb_rw)(u32 hw_irq, u32 offset, u64 data, bool,

    pub xc): *mut *mut int (get_ipi)(unsigned int cpu, struct xive_cpu,
    pub xc): *mut *mut void (put_ipi)(unsigned int cpu, struct xive_cpu,

    pub private): *mut *mut *mut int (debug_show)(struct seq_file m, void,
    pub xive_dir): *mut *mut int (debug_create)(struct dentry,
    pub name: *const c_char,
}

extern "C" {
    pub fn xive_core_debug_init() -> c_int;
}
