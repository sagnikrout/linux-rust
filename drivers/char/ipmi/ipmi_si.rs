//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/ipmi/ipmi_si.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// ipmi_si.h
//
// Interface from the device-specific interfaces (OF, DMI, ACPI, PCI,
// etc) to the base ipmi system interface code.
//

pub const DEFAULT_REGSPACING: c_int = 1;
pub const DEFAULT_REGSIZE: c_int = 1;
// Numbers in this enumerator should be mapped to si_to_str[]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si_type {
    SI_TYPE_INVALID, SI_KCS, SI_SMIC, SI_BT, SI_TYPE_MAX
}

// Array is defined in the ipmi_si_intf.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipmi_match_info {
    pub type: si_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipmi_addr_space {
    IPMI_IO_ADDR_SPACE, IPMI_MEM_ADDR_SPACE
}

//
// The structure for doing I/O in the state machine.  The state
// machine doesn't have the actual I/O routines, they are done through
// this interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_sm_io {
    pub offset): *const *const *const unsigned char (inputb)(struct si_sm_io io, unsigned int,
    pub b): c_uchar,
//
// Generic info used by the actual handling routines, the
// state machine shouldn't touch these.
//
    pub addr: *mut void __iomem,
    pub regspacing: c_uint,
    pub regsize: c_uint,
    pub regshift: c_uint,
    pub addr_space: ipmi_addr_space,
    pub addr_data: c_ulong,
    pub /: *mut *mut ipmi_addr_src addr_source; / ACPI, PCI, SMBIOS, hardcode, etc.,
    pub addr_info: ipmi_smi_info_union,
    pub info): *mut *mut int (io_setup)(struct si_sm_io,
    pub info): *mut *mut void (io_cleanup)(struct si_sm_io,
    pub io_size: c_uint,
    pub irq: c_int,
    pub io): *mut *mut int (irq_setup)(struct si_sm_io,
    pub irq_handler_data: *mut c_void,
    pub io): *mut *mut void (irq_cleanup)(struct si_sm_io,
    pub slave_addr: u8,
    pub si_info: *const ipmi_match_info,
    pub dev: *mut device,
}

extern "C" {
    pub fn ipmi_si_add_smi(io: *mut si_sm_io) -> c_int;
}
extern "C" {
    pub fn ipmi_si_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ipmi_irq_start_cleanup(io: *mut si_sm_io);
}
extern "C" {
    pub fn ipmi_std_irq_setup(io: *mut si_sm_io) -> c_int;
}
extern "C" {
    pub fn ipmi_irq_finish_setup(io: *mut si_sm_io);
}
extern "C" {
    pub fn ipmi_si_remove_by_dev(dev: *mut device);
}
extern "C" {
    pub fn ipmi_hardcode_init();
}
extern "C" {
    pub fn ipmi_si_hardcode_exit();
}
extern "C" {
    pub fn ipmi_si_hotmod_exit();
}
extern "C" {
    pub fn ipmi_si_hardcode_match(addr_space: c_int, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn ipmi_si_platform_init();
}
extern "C" {
    pub fn ipmi_si_platform_shutdown();
}
extern "C" {
    pub fn ipmi_remove_platform_device_by_name(name: *mut c_char);
}

extern "C" {
    pub fn ipmi_si_pci_init();
}
extern "C" {
    pub fn ipmi_si_pci_shutdown();
}

extern "C" {
    pub fn ipmi_si_ls2k_init();
}
extern "C" {
    pub fn ipmi_si_ls2k_shutdown();
}

extern "C" {
    pub fn ipmi_si_parisc_init();
}
extern "C" {
    pub fn ipmi_si_parisc_shutdown();
}

extern "C" {
    pub fn ipmi_si_port_setup(io: *mut si_sm_io) -> c_int;
}
extern "C" {
    pub fn ipmi_si_mem_setup(io: *mut si_sm_io) -> c_int;
}
