//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/mediatek/mtk-eint.h
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
// Copyright (C) 2014-2025 MediaTek Inc.
//
// Author: Maoguang Meng <maoguang.meng@mediatek.com>
// Sean Wang <sean.wang@mediatek.com>
// Hao Chang <ot_chhao.chang@mediatek.com>
// Qingliang Li <qingliang.li@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint_regs {
    pub stat: c_uint,
    pub ack: c_uint,
    pub mask: c_uint,
    pub mask_set: c_uint,
    pub mask_clr: c_uint,
    pub sens: c_uint,
    pub sens_set: c_uint,
    pub sens_clr: c_uint,
    pub soft: c_uint,
    pub soft_set: c_uint,
    pub soft_clr: c_uint,
    pub pol: c_uint,
    pub pol_set: c_uint,
    pub pol_clr: c_uint,
    pub dom_en: c_uint,
    pub dbnc_ctrl: c_uint,
    pub dbnc_set: c_uint,
    pub dbnc_clr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint_hw {
    pub port_mask: u8,
    pub ports: u8,
    pub ap_num: c_uint,
    pub db_cnt: c_uint,
    pub db_time: *const c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint_pin {
    pub number: u16,
    pub instance: u8,
    pub index: u8,
    pub debounce: bool,
    pub dual_edge: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint_xt {
    pub gpio_chip): *mut gpio_chip,
    pub eint_n): *mut *mut *mut int (get_gpio_state)(void data, unsigned long,
    pub eint_n): *mut *mut *mut int (set_gpio_as_eint)(void data, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eint {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub nbase: c_int,
    pub base_pin_num: *mut u16,
    pub domain: *mut irq_domain,
    pub irq: c_int,
    pub dual_edge: *mut c_int,
    pub pin_list: *mut u16,
    pub wake_mask: *mut u32,
    pub cur_mask: *mut u32,
// Used to fit into various EINT device
    pub hw: *const mtk_eint_hw,
    pub regs: *const mtk_eint_regs,
    pub pins: *mut mtk_eint_pin,
    pub num_db_time: u16,
// Used to fit into various pinctrl device
    pub pctl: *mut c_void,
    pub gpio_xlate: *const mtk_eint_xt,
}

extern "C" {
    pub fn mtk_eint_do_init(eint: *mut mtk_eint, eint_pin: *mut mtk_eint_pin) -> c_int;
}
extern "C" {
    pub fn mtk_eint_do_suspend(eint: *mut mtk_eint) -> c_int;
}
extern "C" {
    pub fn mtk_eint_do_resume(eint: *mut mtk_eint) -> c_int;
}
extern "C" {
    pub fn mtk_eint_find_irq(eint: *mut mtk_eint, eint_n: c_ulong) -> c_int;
}

