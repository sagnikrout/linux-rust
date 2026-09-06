//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/nand-ecc-mtk.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// MTK SDG1 ECC controller
//
// Copyright (c) 2016 Mediatek
// Authors:	Xiaolei Li		<xiaolei.li@mediatek.com>
// Jorge Ramirez-Ortiz	<jorge.ramirez-ortiz@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_ecc_mode {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_ecc_operation {

    struct device_node;
    struct mtk_ecc;

    struct mtk_ecc_stats {
    u32 corrected;
    u32 bitflips;
    u32 failed;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ecc_config {
    pub op: mtk_ecc_operation,
    pub mode: mtk_ecc_mode,
    pub addr: dma_addr_t,
    pub strength: u32,
    pub sectors: u32,
    pub len: u32,
}

extern "C" {
    pub fn mtk_ecc_encode(: *mut mtk_ecc, : *mut mtk_ecc_config, : *mut u8, _arg: u32) -> c_int;
}
extern "C" {
    pub fn mtk_ecc_get_stats(: *mut mtk_ecc, : *mut mtk_ecc_stats, _arg: c_int);
}
extern "C" {
    pub fn mtk_ecc_wait_done(: *mut mtk_ecc, mtk_ecc_operation: enum) -> c_int;
}
extern "C" {
    pub fn mtk_ecc_enable(: *mut mtk_ecc, : *mut mtk_ecc_config) -> c_int;
}
extern "C" {
    pub fn mtk_ecc_disable(: *mut mtk_ecc);
}
extern "C" {
    pub fn mtk_ecc_adjust_strength(ecc: *mut mtk_ecc, p: *mut u32);
}
extern "C" {
    pub fn mtk_ecc_get_parity_bits(ecc: *mut mtk_ecc) -> c_uint;
}
extern "C" {
    pub fn mtk_ecc_release(: *mut mtk_ecc);
}
