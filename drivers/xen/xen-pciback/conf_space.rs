//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/xen/xen-pciback/conf_space.h
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
// PCI Backend - Common data structures for overriding the configuration space
//
// Author: Ryan Wilson <hap9@epoch.ncsc.mil>
//

// conf_field_init can return an errno in a ptr with ERR_PTR()
extern "C" {
    pub fn void(dev: *mut *mut conf_field_reset) (struct pci_dev, offset: c_int, data: *mut c_void) -> typedef;
}
extern "C" {
    pub fn void(dev: *mut *mut conf_field_free) (struct pci_dev, offset: c_int, data: *mut c_void) -> typedef;
}
// These are the fields within the configuration space which we
// are interested in intercepting reads/writes to and changing their
// values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_field {
    pub offset: c_uint,
    pub size: c_uint,
    pub mask: c_uint,
    pub init: conf_field_init,
    pub reset: conf_field_reset,
    pub release: conf_field_free,
    pub field): *mut *mut void (clean) (struct config_field,
    pub write: conf_dword_write,
    pub read: conf_dword_read,
    pub dw: },
    pub write: conf_word_write,
    pub read: conf_word_read,
    pub w: },
    pub write: conf_byte_write,
    pub read: conf_byte_read,
    pub b: },
    pub u: },
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_field_entry {
    pub list: list_head,
    pub field: *const config_field,
    pub base_offset: c_uint,
    pub data: *mut c_void,
}

// Add fields to a device - the add_fields macro expects to get a pointer to
// the first entry in an array (of which the ending is marked by size==0)
//
extern "C" {
    pub fn xen_pcibk_config_add_field_offset(_arg: dev, _arg: field, _arg: 0) -> return;
}
// Read/Write the real configuration space
extern "C" {
    pub fn xen_pcibk_config_capability_init() -> c_int;
}
extern "C" {
    pub fn xen_pcibk_config_header_add_fields(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn xen_pcibk_config_capability_add_fields(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn xen_pcibk_get_interrupt_type(dev: *mut pci_dev) -> c_int;
}
