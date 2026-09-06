//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btbcm.h
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
// Bluetooth support for Broadcom devices
//
// Copyright (C) 2015  Intel Corporation
//
pub const BCM_UART_CLOCK_48MHZ: c_uint = 0x01;
pub const BCM_UART_CLOCK_24MHZ: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_update_uart_baud_rate {
    pub zero: __le16,
    pub baud_rate: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_write_uart_clock_setting {
    pub type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_set_sleep_mode {
    pub sleep_mode: __u8,
    pub idle_host: __u8,
    pub idle_dev: __u8,
    pub bt_wake_active: __u8,
    pub host_wake_active: __u8,
    pub allow_host_sleep: __u8,
    pub combine_modes: __u8,
    pub tristate_control: __u8,
    pub usb_auto_sleep: __u8,
    pub usb_resume_timeout: __u8,
    pub break_to_host: __u8,
    pub pulsed_host_wake: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_set_pcm_int_params {
    pub routing: __u8,
    pub rate: __u8,
    pub frame_sync: __u8,
    pub sync_mode: __u8,
    pub clock_mode: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_set_pcm_format_params {
    pub lsb_first: __u8,
    pub fill_value: __u8,
    pub fill_method: __u8,
    pub fill_num: __u8,
    pub right_justify: __u8,
    pub __packed: },

    pub hdev): *mut int btbcm_check_bdaddr(struct hci_dev,
    pub bdaddr): *const *const int btbcm_set_bdaddr(struct hci_dev hdev, bdaddr_t,
    pub fw): *const *const int btbcm_patchram(struct hci_dev hdev, struct firmware,
    pub params): *mut bcm_set_pcm_int_params,
    pub params): *const bcm_set_pcm_int_params,
    pub hdev): *mut int btbcm_setup_patchram(struct hci_dev,
    pub hdev): *mut int btbcm_setup_apple(struct hci_dev,
    pub use_autobaud_mode): *mut *mut *mut int btbcm_initialize(struct hci_dev hdev, bool fw_load_done, bool,
    pub use_autobaud_mode): *mut *mut *mut int btbcm_finalize(struct hci_dev hdev, bool fw_load_done, bool,

    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
