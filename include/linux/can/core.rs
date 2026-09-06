//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/core.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// linux/can/core.h
//
// Prototypes and definitions for CAN protocol modules using the PF_CAN core
//
// Authors: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
// Urs Thuermann   <urs.thuermann@volkswagen.de>
// Copyright (c) 2002-2017 Volkswagen Group Electronic Research
// All rights reserved.
//

//
// struct can_proto - CAN protocol structure
// @type:       type argument in socket() syscall, e.g. SOCK_DGRAM.
// @protocol:   protocol number in socket() syscall.
// @ops:        pointer to struct proto_ops for sock->ops.
// @prot:       pointer to struct proto structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_proto {
    pub type: c_int,
    pub protocol: c_int,
    pub ops: *const proto_ops,
    pub prot: *mut proto,
}

// required_size
// macro to find the minimum size of a struct
// that includes a requested member
//

// function prototypes for the CAN networklayer core (af_can.c)
extern "C" {
    pub fn can_proto_register(cp: *const can_proto) -> c_int;
}
extern "C" {
    pub fn can_proto_unregister(cp: *const can_proto);
}
extern "C" {
    pub fn can_send(skb: *mut sk_buff, loop: c_int) -> c_int;
}
extern "C" {
    pub fn can_set_skb_uid(skb: *mut sk_buff);
}
extern "C" {
    pub fn can_sock_destruct(sk: *mut sock);
}
