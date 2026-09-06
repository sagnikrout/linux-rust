//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wkup_m3_ipc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// TI Wakeup M3 for AMx3 SoCs Power Management Routines
//
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Dave Gerlach <d-gerlach@ti.com>
//
pub const WKUP_M3_DEEPSLEEP: c_int = 1;
pub const WKUP_M3_STANDBY: c_int = 2;
pub const WKUP_M3_IDLE: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wkup_m3_ipc {
    pub rproc: *mut rproc,
    pub ipc_mem_base: *mut void __iomem,
    pub dev: *mut device,
    pub mem_type: c_int,
    pub resume_addr: c_ulong,
    pub vtt_conf: c_int,
    pub isolation_conf: c_int,
    pub state: c_int,
    pub halt: u32,
    pub volt_scale_offsets: c_ulong,
    pub sd_fw_name: *const c_char,
    pub sync_complete: completion,
    pub mbox_client: mbox_client,
    pub mbox: *mut mbox_chan,
    pub ops: *mut wkup_m3_ipc_ops,
    pub is_rtc_only: c_int,
    pub dbg_path: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wkup_m3_wakeup_src {
    pub irq_nr: c_int,
    pub src: [c_char; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wkup_m3_scale_data_header {
    pub magic: u16,
    pub sleep_offset: u8,
    pub wake_offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wkup_m3_ipc_ops {
    pub mem_type): *mut *mut *mut void (set_mem_type)(struct wkup_m3_ipc m3_ipc, int,
    pub addr): *mut *mut *mut void (set_resume_address)(struct wkup_m3_ipc m3_ipc, void,
    pub state): *mut *mut *mut int (prepare_low_power)(struct wkup_m3_ipc m3_ipc, int,
    pub m3_ipc): *mut *mut int (finish_low_power)(struct wkup_m3_ipc,
    pub m3_ipc): *mut *mut int (request_pm_status)(struct wkup_m3_ipc,
    pub m3_ipc): *const *const *const char (request_wake_src)(struct wkup_m3_ipc,
    pub m3_ipc): *mut *mut void (set_rtc_only)(struct wkup_m3_ipc,
}

extern "C" {
    pub fn wkup_m3_ipc_put(m3_ipc: *mut wkup_m3_ipc);
}
extern "C" {
    pub fn wkup_m3_set_rtc_only_mode();
}
