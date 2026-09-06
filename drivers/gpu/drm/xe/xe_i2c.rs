//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_i2c.h
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


// SPDX-License-Identifier: MIT

pub const XE_I2C_EP_COOKIE_DEVICE: c_uint = 0xde;
// Endpoint Capabilities

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum XE_I2C_CLIENT {
    XE_I2C_CLIENT_AMC = 1,
    XE_I2C_MAX_CLIENTS = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_i2c_endpoint {
    pub cookie: u8,
    pub capabilities: u8,
    pub addr: [u16; XE_I2C_MAX_CLIENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_i2c {
    pub pdev: *mut platform_device,
    pub adapter: *mut i2c_adapter,
    pub client: [*mut i2c_client; XE_I2C_MAX_CLIENTS],
    pub ic_enable: c_uint,
    pub bus_notifier: notifier_block,
    pub work: work_struct,
    pub ep: xe_i2c_endpoint,
    pub drm_dev: *mut device,
    pub mmio: *mut xe_mmio,
    pub amc: *mut xe_amc,
}

extern "C" {
    pub fn xe_i2c_probe(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_i2c_present(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_i2c_irq_handler(xe: *mut xe_device, master_ctl: u32);
}
extern "C" {
    pub fn xe_i2c_irq_postinstall(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_i2c_irq_reset(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_i2c_pm_suspend(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_i2c_pm_resume(xe: *mut xe_device, d3cold: bool);
}

