//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/amd/hsmp/hsmp.h
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
// AMD HSMP Platform Driver
// Copyright (c) 2024, AMD.
// All Rights Reserved.
//
// Header file for HSMP driver
//

pub const HSMP_ATTR_GRP_NAME_SIZE: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsmp_mbaddr_info {
    pub base_addr: u32,
    pub msg_id_off: u32,
    pub msg_resp_off: u32,
    pub msg_arg_off: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsmp_socket {
    pub hsmp_attr: bin_attribute,
    pub mbinfo: hsmp_mbaddr_info,
    pub metric_tbl_addr: *mut void __iomem,
// Size of the region mapped at @metric_tbl_addr, as reported by SMU
    pub metric_tbl_size: usize,
    pub virt_base_addr: *mut void __iomem,
    pub hsmp_sem: semaphore,
// Serializes HSMP_GET_METRIC_TABLE fill-and-copy for this socket
    pub metric_read_lock: mutex,
    pub name: [c_char; HSMP_ATTR_GRP_NAME_SIZE],
    pub dev: *mut device,
    pub sock_ind: u16,
    pub rw): *mut *mut *mut *mut int (amd_hsmp_rdwr)(struct hsmp_socket sock, u32 off, u32 val, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsmp_plat_device {
    pub mdev: miscdevice,
    pub sock: *mut hsmp_socket,
    pub proto_ver: u32,
    pub num_sockets: u16,
}

extern "C" {
    pub fn hsmp_cache_proto_ver(sock_ind: u16) -> c_int;
}
extern "C" {
    pub fn hsmp_test(sock_ind: u16, value: u32) -> c_int;
}
extern "C" {
    pub fn hsmp_ioctl(fp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn hsmp_misc_deregister();
}
extern "C" {
    pub fn hsmp_misc_register(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn hsmp_get_tbl_dram_base(sock_ind: u16) -> c_int;
}
extern "C" {
    pub fn hsmp_unmap_metric_tbls(pdev: *mut hsmp_plat_device);
}
extern "C" {
    pub fn hsmp_init_metric_read_locks(pdev: *mut hsmp_plat_device);
}
extern "C" {
    pub fn hsmp_destroy_metric_read_locks(pdev: *mut hsmp_plat_device);
}
extern "C" {
    pub fn hsmp_metric_tbl_read(sock: *mut hsmp_socket, buf: *mut c_char, size: usize) -> isize;
}

extern "C" {
    pub fn hsmp_create_sensor(dev: *mut device, sock_ind: u16) -> c_int;
}

extern "C" {
    pub fn hsmp_msg_get_nargs(sock_ind: u16, msg_id: u32, data: *mut u32, num_args: u8) -> c_int;
}
//
// Gates the HSMP data plane: hsmp_send_message() takes it for read; probe and
// remove take it for write to bring sockets up and tear them down.
//
