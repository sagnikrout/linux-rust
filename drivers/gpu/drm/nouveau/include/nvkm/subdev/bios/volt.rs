//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/volt.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvbios_volt_type {
    NVBIOS_VOLT_GPIO = 0,
    NVBIOS_VOLT_PWM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_volt {
    pub type: nvbios_volt_type,
    pub min: u32,
    pub max: u32,
    pub base: u32,
// GPIO mode
    pub ranged: bool,
    pub vidmask: u8,
    pub step: i16,
// PWM mode
    pub pwm_freq: u32,
    pub pwm_range: u32,
}

extern "C" {
    pub fn nvbios_volt_table(: *mut nvkm_bios, ver: *mut u8, hdr: *mut u8, cnt: *mut u8, len: *mut u8) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_volt_entry {
    pub voltage: u32,
    pub vid: u8,
}

extern "C" {
    pub fn nvbios_volt_entry(: *mut nvkm_bios, idx: c_int, ver: *mut u8, len: *mut u8) -> u32;
}
