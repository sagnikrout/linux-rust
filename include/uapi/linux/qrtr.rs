//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/qrtr.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const QRTR_NODE_BCAST: c_uint = 0xffffffffu;
pub const QRTR_PORT_CTRL: c_uint = 0xfffffffeu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_qrtr {
    pub sq_family: __kernel_sa_family_t,
    pub sq_node: __u32,
    pub sq_port: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qrtr_pkt_type {
    QRTR_TYPE_DATA		= 1,
    QRTR_TYPE_HELLO		= 2,
    QRTR_TYPE_BYE		= 3,
    QRTR_TYPE_NEW_SERVER	= 4,
    QRTR_TYPE_DEL_SERVER	= 5,
    QRTR_TYPE_DEL_CLIENT	= 6,
    QRTR_TYPE_RESUME_TX	= 7,
    QRTR_TYPE_EXIT          = 8,
    QRTR_TYPE_PING          = 9,
    QRTR_TYPE_NEW_LOOKUP	= 10,
    QRTR_TYPE_DEL_LOOKUP	= 11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qrtr_ctrl_pkt {
    pub cmd: __le32,
    pub service: __le32,
    pub instance: __le32,
    pub node: __le32,
    pub port: __le32,
    pub server: },
    pub node: __le32,
    pub port: __le32,
    pub client: },
}
