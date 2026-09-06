//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/fotg210/fotg210.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gemini_port {
    GEMINI_PORT_NONE = 0,
    GEMINI_PORT_0,
    GEMINI_PORT_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210 {
    pub dev: *mut device,
    pub res: *mut resource,
    pub base: *mut void __iomem,
    pub pclk: *mut clk,
    pub map: *mut regmap,
    pub port: gemini_port,
}

extern "C" {
    pub fn fotg210_vbus(fotg: *mut fotg210, enable: bool);
}

extern "C" {
    pub fn fotg210_hcd_probe(pdev: *mut platform_device, fotg: *mut fotg210) -> c_int;
}
extern "C" {
    pub fn fotg210_hcd_remove(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn fotg210_hcd_init() -> c_int;
}
extern "C" {
    pub fn fotg210_hcd_cleanup();
}

extern "C" {
    pub fn fotg210_udc_probe(pdev: *mut platform_device, fotg: *mut fotg210) -> c_int;
}
extern "C" {
    pub fn fotg210_udc_remove(pdev: *mut platform_device) -> c_int;
}

