//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_dma.h
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
// OF helpers for DMA request / controller
//
// Based on of_gpio.h
//
// Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_dma {
    pub of_dma_controllers: list_head,
    pub of_node: *mut device_node,
    pub ): *mut *mut (struct of_phandle_args , struct of_dma,
    pub ): *mut *mut (struct of_phandle_args , struct of_dma,
    pub dma_router: *mut dma_router,
    pub of_dma_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_dma_filter_info {
    pub dma_cap: dma_cap_mask_t,
    pub filter_fn: dma_filter_fn,
}

extern "C" {
    pub fn of_dma_controller_free(np: *mut device_node);
}
extern "C" {
    pub fn devm_add_action_or_reset(_arg: dev, _arg: __of_dma_controller_free, _arg: np) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

