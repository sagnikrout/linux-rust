//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/cxl/test/mock.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mock_ops {
    pub list: list_head,
    pub dev): *mut *mut bool (is_mock_adev)(struct acpi_device,
    pub arg): *mut c_void,
    pub dev): *mut *mut bool (is_mock_bridge)(struct device,
    pub data): *mut c_ulonglong,
    pub handle): *mut *mut *mut acpi_pci_root (acpi_pci_find_root)(acpi_handle,
    pub bus): *mut *mut bool (is_mock_bus)(struct pci_bus,
    pub dev): *mut *mut bool (is_mock_port)(struct device,
    pub dev): *mut *mut bool (is_mock_dev)(struct device,
    pub port): *mut *mut int (devm_cxl_switch_port_decoders_setup)(struct cxl_port,
    pub port): *mut *mut int (devm_cxl_endpoint_decoders_setup)(struct cxl_port,
    pub port): *mut *mut void (cxl_endpoint_parse_cdat)(struct cxl_port,
    pub dport_dev): *mut device,
    pub cache_size): *mut resource_size_t,
    pub fn): *mut *mut *mut int (walk_hmem_resources)(struct device host, walk_hmem_fn,
    pub desc): unsigned long flags, unsigned long,
    pub size): usize,
}

extern "C" {
    pub fn hmem_test_init() -> c_int;
}
extern "C" {
    pub fn hmem_test_exit();
}
extern "C" {
    pub fn register_cxl_mock_ops(ops: *mut cxl_mock_ops);
}
extern "C" {
    pub fn unregister_cxl_mock_ops(ops: *mut cxl_mock_ops);
}
extern "C" {
    pub fn put_cxl_mock_ops(index: c_int);
}
