//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/issei/fw_client.h
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
// Copyright (C) 2023-2026 Intel Corporation

//
// struct issei_fw_client - represents firmware queue
// @kobj: associated kobject
// @list: link in firmware clients list
// @id: firmware client id
// @ver: firmware client version
// @uuid: firmware client protocol id
// @mtu: firmware client maximum buffer size
// @flags: firmware client flags
// @cl: pointer to host client, if connected
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct issei_fw_client {
    pub kobj: kobject,
    pub list: list_head,
    pub id: u16,
    pub ver: u8,
    pub uuid: uuid_t,
    pub mtu: u32,
    pub flags: u32,
    pub cl: *mut issei_host_client,
}

extern "C" {
    pub fn issei_fw_cl_remove_all(idev: *mut issei_device);
}
extern "C" {
    pub fn issei_fw_cl_connect(fw_cl: *mut issei_fw_client, cl: *mut issei_host_client) -> c_int;
}
extern "C" {
    pub fn issei_fw_cl_disconnect(fw_cl: *mut issei_fw_client);
}
