//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cobalt/cobalt-omnitek.h
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
// Omnitek Scatter-Gather DMA Controller
//
// Copyright 2012-2015 Cisco Systems, Inc. and/or its affiliates.
// All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_dma_descriptor {
    pub pci_l: u32,
    pub pci_h: u32,
    pub local: u32,
    pub reserved0: u32,
    pub next_l: u32,
    pub next_h: u32,
    pub bytes: u32,
    pub reserved1: u32,
}

extern "C" {
    pub fn omni_sg_dma_init(cobalt: *mut cobalt) -> c_int;
}
extern "C" {
    pub fn omni_sg_dma_abort_channel(s: *mut cobalt_stream);
}
extern "C" {
    pub fn omni_sg_dma_start(s: *mut cobalt_stream, desc: *mut sg_dma_desc_info);
}
extern "C" {
    pub fn is_dma_done(s: *mut cobalt_stream) -> bool;
}
extern "C" {
    pub fn descriptor_list_loopback(desc: *mut sg_dma_desc_info);
}
extern "C" {
    pub fn descriptor_list_end_of_chain(desc: *mut sg_dma_desc_info);
}
extern "C" {
    pub fn descriptor_list_free(desc: *mut sg_dma_desc_info);
}
extern "C" {
    pub fn descriptor_list_interrupt_enable(desc: *mut sg_dma_desc_info);
}
extern "C" {
    pub fn descriptor_list_interrupt_disable(desc: *mut sg_dma_desc_info);
}
