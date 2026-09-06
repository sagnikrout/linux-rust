//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sdca_interrupts.h
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
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//
// Copyright (C) 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

//
// struct sdca_interrupt - contains information about a single SDCA interrupt
// @name: The name of the interrupt.
// @dev: Pointer to the Function device.
// @device_regmap: Pointer to the IRQ regmap.
// @function_regmap: Pointer to the SDCA Function regmap.
// @component: Pointer to the ASoC component owns the interrupt.
// @function: Pointer to the Function that the interrupt is associated with.
// @entity: Pointer to the Entity that the interrupt is associated with.
// @control: Pointer to the Control that the interrupt is associated with.
// @handler: Handler function to be called for the IRQ.
// @priv: Pointer to private data for use by the handler.
// @free_priv: Pointer to a function that can be used to free the priv data.
// @irq: IRQ number allocated to this interrupt, also used internally to track
// the IRQ being assigned.
// @early_request: Flag to indicate this IRQ was requested at bus probe time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdca_interrupt {
    pub name: *const c_char,
    pub dev: *mut device,
    pub device_regmap: *mut regmap,
    pub function_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub function: *mut sdca_function_data,
    pub entity: *mut sdca_entity,
    pub control: *mut sdca_control,
    pub handler: irq_handler_t,
    pub priv: *mut c_void,
    pub interrupt): *mut *mut void (free_priv)(struct sdca_interrupt,
    pub irq: c_int,
    pub early_request: bool,
}

//
// struct sdca_interrupt_info - contains top-level SDCA interrupt information
// @irq_chip: regmap irq chip structure.
// @irq_data: regmap irq chip data structure.
// @irqs: Array of data for each individual IRQ.
// @irq_lock: Protects access to the list of sdca_interrupt structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdca_interrupt_info {
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irqs: [sdca_interrupt; SDCA_MAX_INTERRUPTS],
    pub /: *mut *mut mutex irq_lock; / Protect irqs list across functions,
}
