//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pnp/base.h
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
// Copyright (C) 2008 Hewlett-Packard Development Company, L.P.
// Bjorn Helgaas <bjorn.helgaas@hp.com>
//
extern "C" {
    pub fn pnp_register_protocol(protocol: *mut pnp_protocol) -> c_int;
}
pub const PNP_EISA_ID_MASK: c_uint = 0x7fffffff;
extern "C" {
    pub fn pnp_eisa_id_to_string(id: u32, str: *mut c_char);
}
extern "C" {
    pub fn pnp_add_device(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_add_card(card: *mut pnp_card) -> c_int;
}
extern "C" {
    pub fn pnp_add_card_device(card: *mut pnp_card, dev: *mut pnp_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_port {
    pub /: *mut *mut resource_size_t min; / min base number,
    pub /: *mut *mut resource_size_t max; / max base number,
    pub /: *mut *mut resource_size_t align; / align boundary,
    pub /: *mut *mut resource_size_t size; / size of range,
    pub /: *mut *mut unsigned char flags; / port flags,
}

pub const PNP_IRQ_NR: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_irq {
    pub /: *mut *mut pnp_irq_mask_t map; / bitmap for IRQ lines,
    pub /: *mut *mut unsigned char flags; / IRQ flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_dma {
    pub /: *mut *mut unsigned char map; / bitmask for DMA channels,
    pub /: *mut *mut unsigned char flags; / DMA flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_mem {
    pub /: *mut *mut resource_size_t min; / min base number,
    pub /: *mut *mut resource_size_t max; / max base number,
    pub /: *mut *mut resource_size_t align; / align boundary,
    pub /: *mut *mut resource_size_t size; / size of range,
    pub /: *mut *mut unsigned char flags; / memory flags,
}

pub const PNP_OPTION_DEPENDENT: c_uint = 0x80000000;
pub const PNP_OPTION_SET_MASK: c_uint = 0xffff;
pub const PNP_OPTION_SET_SHIFT: c_int = 12;
pub const PNP_OPTION_PRIORITY_MASK: c_uint = 0xfff;
pub const PNP_OPTION_PRIORITY_SHIFT: c_int = 0;
pub const PNP_RES_PRIORITY_PREFERRED: c_int = 0;
pub const PNP_RES_PRIORITY_ACCEPTABLE: c_int = 1;
pub const PNP_RES_PRIORITY_FUNCTIONAL: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_option {
    pub list: list_head,
    pub /: *mut *mut unsigned int flags; / independent/dependent, set, priority,
    pub /: *mut *mut unsigned long type; / IORESOURCE_{IO,MEM,IRQ,DMA},
    pub port: pnp_port,
    pub irq: pnp_irq,
    pub dma: pnp_dma,
    pub mem: pnp_mem,
    pub u: },
}

extern "C" {
    pub fn dbg_pnp_show_option(dev: *mut pnp_dev, option: *mut pnp_option);
}
extern "C" {
    pub fn pnp_init_resources(dev: *mut pnp_dev);
}
extern "C" {
    pub fn pnp_fixup_device(dev: *mut pnp_dev);
}
extern "C" {
    pub fn pnp_free_options(dev: *mut pnp_dev);
}
extern "C" {
    pub fn __pnp_add_device(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_check_port(dev: *mut pnp_dev, res: *mut resource) -> c_int;
}
extern "C" {
    pub fn pnp_check_mem(dev: *mut pnp_dev, res: *mut resource) -> c_int;
}
extern "C" {
    pub fn pnp_check_irq(dev: *mut pnp_dev, res: *mut resource) -> c_int;
}

extern "C" {
    pub fn pnp_check_dma(dev: *mut pnp_dev, res: *mut resource) -> c_int;
}

extern "C" {
    pub fn dbg_pnp_show_resources(dev: *mut pnp_dev, desc: *mut c_char);
}
extern "C" {
    pub fn pnp_free_resources(dev: *mut pnp_dev);
}
extern "C" {
    pub fn pnp_resource_type(res: *mut resource) -> c_ulong;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_resource {
    pub list: list_head,
    pub res: resource,
}

extern "C" {
    pub fn pnp_free_resource(pnp_res: *mut pnp_resource);
}

