//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/pru_rproc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// PRUSS Remote Processor specific types
//
// Copyright (C) 2014-2020 Texas Instruments Incorporated - https://www.ti.com
// Suman Anna <s-anna@ti.com>
//
// struct pruss_int_map - PRU system events _to_ channel and host mapping
// @event: number of the system event
// @chnl: channel number assigned to a given @event
// @host: host number assigned to a given @chnl
//
// PRU system events are mapped to channels, and these channels are mapped
// to host interrupts. Events can be mapped to channels in a one-to-one or
// many-to-one ratio (multiple events per channel), and channels can be
// mapped to host interrupts in a one-to-one or many-to-one ratio (multiple
// channels per interrupt).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pruss_int_map {
    pub event: u8,
    pub chnl: u8,
    pub host: u8,
}

//
// struct pru_irq_rsc - PRU firmware section header for IRQ data
// @type: resource type
// @num_evts: number of described events
// @pru_intc_map: PRU interrupt routing description
//
// The PRU firmware blob can contain optional .pru_irq_map ELF section, which
// provides the PRUSS interrupt mapping description. The pru_irq_rsc struct
// describes resource entry format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pru_irq_rsc {
    pub type: u8,
    pub num_evts: u8,
    pub pru_intc_map: [pruss_int_map; ],
    pub __packed: },
