//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/isa.h
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
// ISA bus.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isa_driver {
    pub int): *mut *mut *mut int (match)(struct device , unsigned,
    pub int): *mut *mut *mut int (probe)(struct device , unsigned,
    pub int): *mut *mut *mut void (remove)(struct device , unsigned,
    pub int): *mut *mut *mut void (shutdown)(struct device , unsigned,
    pub pm_message_t): *mut *mut *mut int (suspend)(struct device , unsigned int,,
    pub int): *mut *mut *mut int (resume)(struct device , unsigned,
    pub driver: device_driver,
    pub devices: *mut device,
}

extern "C" {
    pub fn isa_register_driver(: *mut isa_driver, int: unsigned) -> c_int;
}
extern "C" {
    pub fn isa_unregister_driver(: *mut isa_driver);
}

//
// module_isa_driver() - Helper macro for registering a ISA driver
// @__isa_driver: isa_driver struct
// @__num_isa_dev: number of devices to register
//
// Helper macro for ISA drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate code. Each module may only
// use this macro once, and calling it replaces module_init and module_exit.
//

//
// module_isa_driver_with_irq() - Helper macro for registering an ISA driver with irq
// @__isa_driver: isa_driver struct
// @__num_isa_dev: number of devices to register
// @__num_irq: number of IRQ to register
//
// Helper macro for ISA drivers with irq that do not do anything special in
// module init/exit. Each module may only use this macro once, and calling it
// replaces module_init and module_exit.
//

//
// max_num_isa_dev() - Maximum possible number registered of an ISA device
// @__ida_dev_ext: ISA device address extent
//
// The highest base address possible for an ISA device is 0x3FF; this results in
// 1024 possible base addresses. Dividing the number of possible base addresses
// by the address extent taken by each device results in the maximum number of
// devices on a system.
//

