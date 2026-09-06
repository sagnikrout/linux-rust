//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//

//
// Version 1 and Version 2 are identical on 64 bit machines, but on 32 bit
// machines Version 2 has a different struct layout.
//
pub const RXE_UVERBS_ABI_VERSION: c_int = 2;

extern "C" {
    pub fn rxe_set_mtu(rxe: *mut rxe_dev, dev_mtu: c_uint);
}
extern "C" {
    pub fn rxe_rcv(skb: *mut sk_buff);
}
// The caller must do a matching ib_device_put(&dev->ib_dev)
extern "C" {
    pub fn container_of(_arg: ibdev, rxe_dev: struct, _arg: ib_dev) -> return;
}
extern "C" {
    pub fn rxe_port_up(rxe: *mut rxe_dev);
}
extern "C" {
    pub fn rxe_port_down(rxe: *mut rxe_dev);
}
extern "C" {
    pub fn rxe_set_port_state(rxe: *mut rxe_dev);
}
