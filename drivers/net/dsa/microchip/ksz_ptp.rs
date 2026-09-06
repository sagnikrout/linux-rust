//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/ksz_ptp.h
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
// Microchip KSZ PTP Implementation
//
// Copyright (C) 2020 ARRI Lighting
// Copyright (C) 2022 Microchip Technology Inc.
//

pub const KSZ_PTP_N_GPIO: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_ptp_tou_mode {
    KSZ_PTP_TOU_IDLE,
    KSZ_PTP_TOU_PEROUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_ptp_data {
    pub caps: ptp_clock_info,
    pub clock: *mut ptp_clock,
    pub pin_config: [ptp_pin_desc; KSZ_PTP_N_GPIO],
// Serializes all operations on the PTP hardware clock
    pub lock: mutex,
// lock for accessing the clock_time
    pub clock_lock: spinlock_t,
    pub clock_time: timespec64,
    pub tou_mode: ksz_ptp_tou_mode,
    pub /: *mut *mut timespec64 perout_target_time_first; / start of first pulse,
    pub perout_period: timespec64,
}

extern "C" {
    pub fn ksz_ptp_clock_register(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn ksz_ptp_clock_unregister(ds: *mut dsa_switch);
}
extern "C" {
    pub fn ksz_port_txtstamp(ds: *mut dsa_switch, port: c_int, skb: *mut sk_buff);
}
extern "C" {
    pub fn ksz_port_deferred_xmit(work: *mut kthread_work);
}
extern "C" {
    pub fn ksz_ptp_irq_setup(ds: *mut dsa_switch, p: u8) -> c_int;
}
extern "C" {
    pub fn ksz_ptp_irq_free(ds: *mut dsa_switch, p: u8);
}
extern "C" {
    pub fn ksz8463_ptp_irq_setup(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn ksz8463_ptp_irq_free(ds: *mut dsa_switch);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_ptp_data {
// Serializes all operations on the PTP hardware clock
    pub lock: mutex,
}

