//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/sd_ops.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// linux/drivers/mmc/core/sd_ops.h
//
// Copyright 2006-2007 Pierre Ossman
//

extern "C" {
    pub fn mmc_app_set_bus_width(card: *mut mmc_card, width: c_int) -> c_int;
}
extern "C" {
    pub fn mmc_send_app_op_cond(host: *mut mmc_host, ocr: u32, rocr: *mut u32) -> c_int;
}
extern "C" {
    pub fn mmc_send_if_cond(host: *mut mmc_host, ocr: u32) -> c_int;
}
extern "C" {
    pub fn mmc_send_if_cond_pcie(host: *mut mmc_host, ocr: u32) -> c_int;
}
extern "C" {
    pub fn mmc_send_relative_addr(host: *mut mmc_host, rca: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn mmc_app_send_scr(card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_app_sd_status(card: *mut mmc_card, ssr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mmc_app_cmd(host: *mut mmc_host, card: *mut mmc_card) -> c_int;
}
extern "C" {
    pub fn mmc_send_ext_addr(host: *mut mmc_host, addr: u32) -> c_int;
}
extern "C" {
    pub fn mmc_uhs2_prepare_cmd(host: *mut mmc_host, mrq: *mut mmc_request);
}
