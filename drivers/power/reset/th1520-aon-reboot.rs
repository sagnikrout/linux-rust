//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/th1520-aon-reboot.c
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
// T-HEAD TH1520 AON Firmware Reboot Driver
//
// Copyright (c) 2025 Icenowy Zheng <uwu@icenowy.me>
//

pub const TH1520_AON_REBOOT_PRIORITY: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_aon_msg_empty_body {
    pub hdr: th1520_aon_rpc_msg_hdr,
    pub reserved: [u16; 12],
    pub __aligned(1): } __packed,
#[no_mangle]
unsafe extern "C" fn th1520_aon_pwroff_handler(data: *mut sys_off_data) -> c_int {
    static int th1520_aon_pwroff_handler(struct sys_off_data *data)
    {
    pub data->cb_data: *mut *mut th1520_aon_chan aon_chan =,
    pub {}: th1520_aon_msg_empty_body msg =,
    pub TH1520_AON_RPC_SVC_WDG: msg.hdr.svc =,
    pub TH1520_AON_WDG_FUNC_POWER_OFF: msg.hdr.func =,
    pub TH1520_AON_RPC_MSG_NUM: msg.hdr.size =,
    pub &msg): th1520_aon_call_rpc(aon_chan,,
    pub NOTIFY_DONE: return,
    }
#[no_mangle]
unsafe extern "C" fn th1520_aon_restart_handler(data: *mut sys_off_data) -> c_int {
    static int th1520_aon_restart_handler(struct sys_off_data *data)
    {
    pub data->cb_data: *mut *mut th1520_aon_chan aon_chan =,
    pub {}: th1520_aon_msg_empty_body msg =,
    pub TH1520_AON_RPC_SVC_WDG: msg.hdr.svc =,
    pub TH1520_AON_WDG_FUNC_RESTART: msg.hdr.func =,
    pub TH1520_AON_RPC_MSG_NUM: msg.hdr.size =,
    pub &msg): th1520_aon_call_rpc(aon_chan,,
    pub NOTIFY_DONE: return,
    }
    static int th1520_aon_reboot_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    pub &adev->dev: *mut *mut device dev =,
    pub ret: c_int,
// Expect struct th1520_aon_chan to be passed via platform_data
    ret = devm_register_sys_off_handler(dev, SYS_OFF_MODE_POWER_OFF,
    TH1520_AON_REBOOT_PRIORITY,
    th1520_aon_pwroff_handler,
    if (ret) {
    pub handler\n"): dev_err(dev, "Failed to register power off,
    pub ret: return,
    }
    ret = devm_register_sys_off_handler(dev, SYS_OFF_MODE_RESTART,
    TH1520_AON_REBOOT_PRIORITY,
    th1520_aon_restart_handler,
    if (ret) {
    pub handler\n"): dev_err(dev, "Failed to register restart,
    pub ret: return,
    }
    pub 0: return,
    }
    static const struct auxiliary_device_id th1520_aon_reboot_id_table[] = {
    { .name = "th1520_pm_domains.reboot" },
    {},
}

    MODULE_DEVICE_TABLE(auxiliary, th1520_aon_reboot_id_table);
    static struct auxiliary_driver th1520_aon_reboot_driver = {
    .driver = {
    .name = "th1520-aon-reboot",
    },
    .probe = th1520_aon_reboot_probe,
    .id_table = th1520_aon_reboot_id_table,
    };
    module_auxiliary_driver(th1520_aon_reboot_driver);
    MODULE_AUTHOR("Icenowy Zheng <uwu@icenowy.me>");
    MODULE_DESCRIPTION("T-HEAD TH1520 AON-firmware-based reboot driver");
    MODULE_LICENSE("GPL");
