//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hibmcge/hbg_hw.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2024 Hisilicon Limited.

extern "C" {
    pub fn readl(addr: priv->io_base +) -> return;
}
extern "C" {
    pub fn lo_hi_readq(addr: priv->io_base +) -> return;
}

extern "C" {
    pub fn hbg_hw_init(priv: *mut hbg_priv) -> c_int;
}
extern "C" {
    pub fn hbg_hw_adjust_link(priv: *mut hbg_priv, speed: u32, duplex: u32);
}
extern "C" {
    pub fn hbg_hw_get_irq_status(priv: *mut hbg_priv) -> u32;
}
extern "C" {
    pub fn hbg_hw_irq_clear(priv: *mut hbg_priv, mask: u32);
}
extern "C" {
    pub fn hbg_hw_irq_is_enabled(priv: *mut hbg_priv, mask: u32) -> bool;
}
extern "C" {
    pub fn hbg_hw_irq_enable(priv: *mut hbg_priv, mask: u32, enable: bool);
}
extern "C" {
    pub fn hbg_hw_set_mtu(priv: *mut hbg_priv, mtu: u16);
}
extern "C" {
    pub fn hbg_hw_mac_enable(priv: *mut hbg_priv, enable: u32);
}
extern "C" {
    pub fn hbg_hw_set_uc_addr(priv: *mut hbg_priv, mac_addr: u64, index: u32);
}
extern "C" {
    pub fn hbg_hw_get_fifo_used_num(priv: *mut hbg_priv, dir: hbg_dir) -> u32;
}
extern "C" {
    pub fn hbg_hw_set_tx_desc(priv: *mut hbg_priv, tx_desc: *mut hbg_tx_desc);
}
extern "C" {
    pub fn hbg_hw_fill_buffer(priv: *mut hbg_priv, buffer_dma_addr: u32);
}
extern "C" {
    pub fn hbg_hw_set_mac_filter_enable(priv: *mut hbg_priv, enable: u32);
}
extern "C" {
    pub fn hbg_hw_set_pause_enable(priv: *mut hbg_priv, tx_en: u32, rx_en: u32);
}
extern "C" {
    pub fn hbg_hw_get_pause_enable(priv: *mut hbg_priv, tx_en: *mut u32, rx_en: *mut u32);
}
extern "C" {
    pub fn hbg_hw_set_rx_pause_mac_addr(priv: *mut hbg_priv, mac_addr: u64);
}
