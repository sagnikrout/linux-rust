//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctvmem.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File    ctvmem.h
//
// @Brief
// This file contains the definition of virtual memory management object
// for card device.
//
// @Author Liu Chun
// @Date Mar 28 2008
//

// The chip can handle the page table of 4k pages
// (emu20k1 can handle even 8k pages, but we don't use it right now)
//
pub const CT_PAGE_SIZE: c_int = 4096;
pub const CT_PAGE_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_vm_block {
    pub /: *mut *mut unsigned int addr; / starting logical addr of this block,
    pub /: *mut *mut unsigned int size; / size of this device virtual mem block,
    pub list: list_head,
}

// Virtual memory management object for card device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_vm {
    pub /: *mut *mut snd_dma_buffer ptp[CT_PTP_NUM]; / Device page table pages,
    pub /: *mut *mut unsigned int size; / Available addr space in bytes,
    pub /: *mut *mut list_head unused; / List of unused blocks,
    pub /: *mut *mut list_head used; / List of used blocks,
    pub lock: mutex,
// Map host addr (kmalloced/vmalloced) to device logical addr.
    pub size): c_int,
// Unmap device logical addr area.
    pub block): *mut *mut *mut void (unmap)(struct ct_vm , struct ct_vm_block,
    pub index): *mut *mut *mut dma_addr_t (get_ptp_phys)(struct ct_vm vm, int,
}

extern "C" {
    pub fn ct_vm_create(rvm: *mut ct_vm, pci: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn ct_vm_destroy(vm: *mut ct_vm);
}
