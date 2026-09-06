//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ata/pata_parport/pata_parport.h
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
// pata_parport.h	(c) 1997-8  Grant R. Guenther <grant@torque.net>
// Under the terms of the GPL.
//
// This file defines the interface for parallel port IDE adapter chip drivers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pi_adapter {
    pub dev: device,
    pub /: *mut *mut *mut pi_protocol proto; / adapter protocol,
    pub /: *mut *mut int port; / base address of parallel port,
    pub /: *mut *mut int mode; / transfer mode in use,
    pub /: *mut *mut int delay; / adapter delay setting,
    pub /: *mut *mut int unit; / unit number for chained adapters,
    pub /: *mut *mut int saved_r0; / saved port state,
    pub /: *mut *mut int saved_r2; / saved port state,
    pub /: *mut *mut unsigned long private; / for protocol module,
    pub /: *mut *mut *mut pardevice pardev; / pointer to pardevice,
}

// registers are addressed as (cont,regr)
// cont: 0 for command register file, 1 for control register(s)
// regr: 0-7 for register number.
//
// macros and functions exported to the protocol modules

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pi_protocol {
    pub name: [c_char; 8],
    pub max_mode: c_int,
    pub /: *mut *mut int epp_first; / modes >= this use 8 ports,
    pub default_delay: c_int,
    pub /: *mut *mut int max_units; / max chained units probed for,
    pub val): *mut *mut *mut void (write_regr)(struct pi_adapter pi, int cont, int regr, int,
    pub regr): *mut *mut *mut int (read_regr)(struct pi_adapter pi, int cont, int,
    pub count): *mut *mut *mut *mut void (write_block)(struct pi_adapter pi, char buf, int,
    pub count): *mut *mut *mut *mut void (read_block)(struct pi_adapter pi, char buf, int,
    pub pi): *mut *mut void (connect)(struct pi_adapter,
    pub pi): *mut *mut void (disconnect)(struct pi_adapter,
    pub pi): *mut *mut int (test_port)(struct pi_adapter,
    pub pi): *mut *mut int (probe_unit)(struct pi_adapter,
    pub pi): *mut *mut int (test_proto)(struct pi_adapter,
    pub pi): *mut *mut void (log_adapter)(struct pi_adapter,
    pub pi): *mut *mut int (init_proto)(struct pi_adapter,
    pub pi): *mut *mut void (release_proto)(struct pi_adapter,
    pub owner: *mut module,
    pub driver: device_driver,
    pub sht: scsi_host_template,
}

extern "C" {
    pub fn pata_parport_register_driver(pr: *mut pi_protocol) -> c_int;
}
extern "C" {
    pub fn pata_parport_unregister_driver(pr: *mut pi_protocol);
}
//
// module_pata_parport_driver() - Helper macro for registering a pata_parport driver
// @__pi_protocol: pi_protocol struct
//
// Helper macro for pata_parport drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

