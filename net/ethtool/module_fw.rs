//! Automatically rewritten from C Header to Rust Module
//! Source: net/ethtool/module_fw.h
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
// struct ethnl_module_fw_flash_ntf_params - module firmware flashing
// notifications parameters
// @portid: Netlink portid of sender.
// @seq: Sequence number of sender.
// @closed_sock: Indicates whether the socket was closed from user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethnl_module_fw_flash_ntf_params {
    pub portid: u32,
    pub seq: u32,
    pub closed_sock: bool,
}

//
// struct ethtool_module_fw_flash_params - module firmware flashing parameters
// @password: Module password. Only valid when @pass_valid is set.
// @password_valid: Whether the module password is valid or not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_module_fw_flash_params {
    pub password: __be32,
    pub password_valid:1: u8,
}

//
// struct ethtool_cmis_fw_update_params - CMIS firmware update specific
// parameters
// @dev: Pointer to the net_device to be flashed.
// @params: Module firmware flashing parameters.
// @ntf_params: Module firmware flashing notification parameters.
// @fw: Firmware to flash.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_cmis_fw_update_params {
    pub dev: *mut net_device,
    pub params: ethtool_module_fw_flash_params,
    pub ntf_params: ethnl_module_fw_flash_ntf_params,
    pub fw: *const firmware,
}

//
// struct ethtool_module_fw_flash - module firmware flashing
// @list: List node for &module_fw_flash_work_list.
// @dev_tracker: Refcount tracker for @dev.
// @work: The flashing firmware work.
// @fw_update: CMIS firmware update specific parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_module_fw_flash {
    pub list: list_head,
    pub dev_tracker: netdevice_tracker,
    pub work: work_struct,
    pub fw_update: ethtool_cmis_fw_update_params,
}

extern "C" {
    pub fn ethnl_module_fw_flash_sock_destroy(sk_priv: *mut ethnl_sock_priv);
}
extern "C" {
    pub fn ethtool_cmis_fw_update(params: *mut ethtool_cmis_fw_update_params);
}
