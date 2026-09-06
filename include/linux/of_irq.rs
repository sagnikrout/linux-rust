//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_irq.h
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

extern "C" {
    pub fn int(: *mut *mut of_irq_init_cb_t)(struct device_node, : *mut device_node) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_imap_parser {
    pub node: *mut device_node,
    pub imap: *const __be32,
    pub imap_end: *const __be32,
    pub parent_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_imap_item {
    pub parent_args: of_phandle_args,
    pub child_imap_count: u32,
    pub size.: *mut *mut u32 child_imap[16]; / Arbitrary,
// Should be #address-cells + #interrupt-cells but
// avoid using allocation and so, expect that 16
// should be enough
//
}

//
// If the iterator is exited prematurely (break, goto, return) of_node_put() has
// to be called on item.parent_args.np
//

//
// Workarounds only applied to 32bit powermac machines
//
pub const OF_IMAP_OLDWORLD_MAC: c_uint = 0x00000001;
pub const OF_IMAP_NO_PHANDLE: c_uint = 0x00000002;

extern "C" {
    pub fn of_irq_parse_raw(addr: *const __be32, out_irq: *mut of_phandle_args) -> c_int;
}
extern "C" {
    pub fn irq_create_of_mapping(irq_data: *mut of_phandle_args) -> c_uint;
}

extern "C" {
    pub fn of_irq_init(matches: *const of_device_id);
}
extern "C" {
    pub fn of_irq_count(dev: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_irq_get(dev: *mut device_node, index: c_int) -> c_int;
}
extern "C" {
    pub fn of_irq_get_byname(dev: *mut device_node, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn of_msi_configure(dev: *mut device, np: *const device_node);
}
extern "C" {
    pub fn of_msi_xlate(dev: *mut device, msi_np: *mut device_node, id_in: u32) -> u32;
}

//
// irq_of_parse_and_map() is used by all OF enabled platforms; but SPARC
// implements it differently.  However, the prototype is the same for all,
// so declare it here regardless of the CONFIG_OF_IRQ setting.
//
extern "C" {
    pub fn irq_of_parse_and_map(node: *mut device_node, index: c_int) -> c_uint;
}

