//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/simatic-ipc.h
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
// Siemens SIMATIC IPC drivers
//
// Copyright (c) Siemens AG, 2018-2023
//
// Authors:
// Henning Schild <henning.schild@siemens.com>
// Gerd Haeussler <gerd.haeussler.ext@siemens.com>
//

pub const SIMATIC_IPC_DMI_ENTRY_OEM: c_int = 129;
// binary type
pub const SIMATIC_IPC_DMI_TYPE: c_uint = 0xff;
pub const SIMATIC_IPC_DMI_GROUP: c_uint = 0x05;
pub const SIMATIC_IPC_DMI_ENTRY: c_uint = 0x02;
pub const SIMATIC_IPC_DMI_TID: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum simatic_ipc_station_ids {
    SIMATIC_IPC_INVALID_STATION_ID = 0,
    SIMATIC_IPC_IPC227D = 0x00000501,
    SIMATIC_IPC_IPC427D = 0x00000701,
    SIMATIC_IPC_IPC227E = 0x00000901,
    SIMATIC_IPC_IPC277E = 0x00000902,
    SIMATIC_IPC_IPC427E = 0x00000A01,
    SIMATIC_IPC_IPC477E = 0x00000A02,
    SIMATIC_IPC_IPC127E = 0x00000D01,
    SIMATIC_IPC_IPC227G = 0x00000F01,
    SIMATIC_IPC_IPC277G = 0x00000F02,
    SIMATIC_IPC_IPCBX_39A = 0x00001001,
    SIMATIC_IPC_IPCPX_39A = 0x00001002,
    SIMATIC_IPC_IPCBX_21A = 0x00001101,
    SIMATIC_IPC_IPCBX_56A = 0x00001201,
    SIMATIC_IPC_IPCBX_59A = 0x00001202,
}

extern "C" {
    pub fn le32_to_cpu(_arg: data_entry->station_id) -> return;
}
// id = simatic_ipc_get_station_id((u8 *)dh, dh->length);
