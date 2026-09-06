//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_lib.h
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
// WangXun Gigabit PCI Express Linux driver
// Copyright (c) 2019 - 2022 Beijing WangXun Technology Co., Ltd.
//
extern "C" {
    pub fn wx_decode_ptype(ptype: u8) -> wx_dec_ptype;
}
extern "C" {
    pub fn wx_alloc_rx_buffers(rx_ring: *mut wx_ring, cleaned_count: u16);
}
extern "C" {
    pub fn wx_desc_unused(ring: *mut wx_ring) -> u16;
}
extern "C" {
    pub fn wx_napi_enable_all(wx: *mut wx);
}
extern "C" {
    pub fn wx_napi_disable_all(wx: *mut wx);
}
extern "C" {
    pub fn wx_reset_interrupt_capability(wx: *mut wx);
}
extern "C" {
    pub fn wx_clear_interrupt_scheme(wx: *mut wx);
}
extern "C" {
    pub fn wx_init_interrupt_scheme(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_msix_clean_rings(irq: int __always_unused, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn wx_free_irq(wx: *mut wx);
}
extern "C" {
    pub fn wx_setup_isb_resources(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_free_isb_resources(wx: *mut wx);
}
extern "C" {
    pub fn wx_misc_isb(wx: *mut wx, idx: wx_isb_idx) -> u32;
}
extern "C" {
    pub fn wx_write_eitr(q_vector: *mut wx_q_vector);
}
extern "C" {
    pub fn wx_configure_vectors(wx: *mut wx);
}
extern "C" {
    pub fn wx_clean_all_rx_rings(wx: *mut wx);
}
extern "C" {
    pub fn wx_clean_all_tx_rings(wx: *mut wx);
}
extern "C" {
    pub fn wx_free_resources(wx: *mut wx);
}
extern "C" {
    pub fn wx_setup_resources(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_set_features(netdev: *mut net_device, features: netdev_features_t) -> c_int;
}
extern "C" {
    pub fn wx_service_event_schedule(wx: *mut wx);
}
extern "C" {
    pub fn wx_service_event_complete(wx: *mut wx);
}
extern "C" {
    pub fn wx_service_timer(t: *mut timer_list);
}
extern "C" {
    pub fn wx_soft_quiesce(wx: *mut wx);
}
