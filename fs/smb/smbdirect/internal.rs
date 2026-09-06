//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/smbdirect/internal.h
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
// Copyright (c) 2025, Stefan Metzmacher
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_module_state {
    pub mutex: mutex,
    pub accept: *mut workqueue_struct,
    pub connect: *mut workqueue_struct,
    pub idle: *mut workqueue_struct,
    pub refill: *mut workqueue_struct,
    pub immediate: *mut workqueue_struct,
    pub cleanup: *mut workqueue_struct,
    pub workqueues: },
    pub lock: rwlock_t,
    pub list: list_head,
    pub devices: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_device {
    pub list: list_head,
    pub ib_dev: *mut ib_device,
//
// copy of ib_dev->name,
// in order to print renames
//
    pub ib_name: [c_char; IB_DEVICE_NAME_MAX],
}

extern "C" {
    pub fn smbdirect_socket_init_new(net: *mut net, sc: *mut smbdirect_socket) -> c_int;
}
extern "C" {
    pub fn smbdirect_socket_init_accepting(id: *mut rdma_cm_id, sc: *mut smbdirect_socket) -> c_int;
}

extern "C" {
    pub fn smbdirect_socket_destroy_sync(sc: *mut smbdirect_socket);
}
extern "C" {
    pub fn smbdirect_connection_rdma_established(sc: *mut smbdirect_socket);
}
extern "C" {
    pub fn smbdirect_connection_negotiation_done(sc: *mut smbdirect_socket);
}
extern "C" {
    pub fn smbdirect_connection_create_qp(sc: *mut smbdirect_socket) -> c_int;
}
extern "C" {
    pub fn smbdirect_connection_destroy_qp(sc: *mut smbdirect_socket);
}
extern "C" {
    pub fn smbdirect_connection_create_mem_pools(sc: *mut smbdirect_socket) -> c_int;
}
extern "C" {
    pub fn smbdirect_connection_destroy_mem_pools(sc: *mut smbdirect_socket);
}
extern "C" {
    pub fn smbdirect_connection_free_send_io(msg: *mut smbdirect_send_io);
}
extern "C" {
    pub fn smbdirect_connection_put_recv_io(msg: *mut smbdirect_recv_io);
}
extern "C" {
    pub fn smbdirect_connection_idle_timer_work(work: *mut work_struct);
}
extern "C" {
    pub fn smbdirect_connection_grant_recv_credits(sc: *mut smbdirect_socket) -> u16;
}
extern "C" {
    pub fn smbdirect_connection_post_recv_io(msg: *mut smbdirect_recv_io) -> c_int;
}
extern "C" {
    pub fn smbdirect_connection_recv_io_done(cq: *mut ib_cq, wc: *mut ib_wc);
}
extern "C" {
    pub fn smbdirect_connection_recv_io_refill(sc: *mut smbdirect_socket) -> c_int;
}
extern "C" {
    pub fn smbdirect_connection_create_mr_list(sc: *mut smbdirect_socket) -> c_int;
}
extern "C" {
    pub fn smbdirect_connection_destroy_mr_list(sc: *mut smbdirect_socket);
}
extern "C" {
    pub fn smbdirect_accept_negotiate_finish(sc: *mut smbdirect_socket, ntstatus: u32);
}
extern "C" {
    pub fn smbdirect_devices_init() -> __init int;
}
extern "C" {
    pub fn smbdirect_devices_exit() -> __exit void;
}
