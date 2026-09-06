//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/x86-android-tablets/x86-android-tablets.h
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
// DMI based code to deal with broken DSDTs on X86 tablets which ship with
// Android as (part of) the factory image. The factory kernels shipped on these
// devices typically have a bunch of things hardcoded, rather than specified
// in their DSDT.
//
// Copyright (C) 2021-2023 Hans de Goede <hansg@kernel.org>
//

//
// Helpers to get Linux IRQ numbers given a description of the IRQ source
// (either IOAPIC index, or GPIO chip name + pin-number).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_acpi_irq_type {
    X86_ACPI_IRQ_TYPE_NONE,
    X86_ACPI_IRQ_TYPE_APIC,
    X86_ACPI_IRQ_TYPE_GPIOINT,
    X86_ACPI_IRQ_TYPE_PMIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_gpiochip_type {
    X86_GPIOCHIP_UNSPECIFIED = 0,
    X86_GPIOCHIP_BAYTRAIL,
    X86_GPIOCHIP_CHERRYVIEW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_acpi_irq_data {
    pub /: *mut *mut *mut char chip; / GPIO chip label (GPIOINT) or PMIC ACPI path (PMIC),
    pub type: x86_acpi_irq_type,
    pub domain: irq_domain_bus_token,
    pub index: c_int,
    pub /: *mut *mut int trigger; / ACPI_EDGE_SENSITIVE / ACPI_LEVEL_SENSITIVE,
    pub /: *mut *mut int polarity; / ACPI_ACTIVE_HIGH / ACPI_ACTIVE_LOW / ACPI_ACTIVE_BOTH,
    pub /: *mut *mut bool free_gpio; / Release GPIO after getting IRQ (for TYPE_GPIOINT),
    pub con_id: *const c_char,
}

// Structs to describe devices to instantiate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_i2c_client_info {
    pub board_info: i2c_board_info,
    pub adapter_path: *mut c_char,
    pub irq_data: x86_acpi_irq_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_spi_dev_info {
    pub board_info: spi_board_info,
    pub ctrl_path: *mut c_char,
    pub irq_data: x86_acpi_irq_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_serdev_info {
    pub hid: *const c_char,
    pub uid: *const c_char,
    pub acpi: },
    pub devfn: c_uint,
    pub pci: },
    pub ctrl: },
    pub ctrl_devname: *const c_char,
//
// ATM the serdev core only supports of or ACPI matching; and so far all
// Android x86 tablets DSDTs have usable serdev nodes, but sometimes
// under the wrong controller. So we just tie the existing serdev ACPI
// node to the right controller.
//
    pub serdev_hid: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_dev_info {
    pub modules: *const *const c_char,
    pub swnode_group: *const software_node,
    pub i2c_client_info: *const x86_i2c_client_info,
    pub spi_dev_info: *const x86_spi_dev_info,
    pub pdev_info: *const platform_device_info,
    pub serdev_info: *const x86_serdev_info,
    pub gpio_button_swnodes: *const software_node,
    pub i2c_client_count: c_int,
    pub spi_dev_count: c_int,
    pub pdev_count: c_int,
    pub serdev_count: c_int,
    pub dev): *mut *mut int (init)(struct device,
    pub (*exit)(void): *mut c_void,
    pub use_pci: bool,
    pub gpiochip_type: x86_gpiochip_type,
}

extern "C" {
    pub fn x86_acpi_irq_helper_get(data: *const x86_acpi_irq_data) -> c_int;
}
// Software nodes representing GPIO chips used by various tablets
//
// Extern declarations of x86_dev_info structs so there can be a single
// MODULE_DEVICE_TABLE(dmi, ...), while splitting the board descriptions.
//
