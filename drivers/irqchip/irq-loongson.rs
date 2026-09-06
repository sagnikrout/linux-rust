//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/irqchip/irq-loongson.h
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
// Copyright (C) 2024 Loongson Technology Corporation Limited
//
pub const AVEC_MSG_OFFSET: c_uint = 0x100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avecintc_data {
    pub entry: list_head,
    pub cpu: c_uint,
    pub vec: c_uint,
    pub prev_cpu: c_uint,
    pub prev_vec: c_uint,
    pub moving: c_uint,
}

extern "C" {
    pub fn find_pch_pic(gsi: u32) -> c_int;
}
extern "C" {
    pub fn avecintc_acpi_init(parent: *mut irq_domain) -> c_int;
}
extern "C" {
    pub fn redirect_acpi_init(parent: *mut irq_domain) -> c_int;
}
extern "C" {
    pub fn pch_msi_acpi_init_avec(parent: *mut irq_domain) -> c_int;
}
extern "C" {
    pub fn avecintc_sync(adata: *mut avecintc_data);
}
