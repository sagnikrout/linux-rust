//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/handshake.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/handshake.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const HANDSHAKE_FAMILY_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum handshake_handler_class {
    HANDSHAKE_HANDLER_CLASS_NONE,
    HANDSHAKE_HANDLER_CLASS_TLSHD,
    HANDSHAKE_HANDLER_CLASS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum handshake_msg_type {
    HANDSHAKE_MSG_TYPE_UNSPEC,
    HANDSHAKE_MSG_TYPE_CLIENTHELLO,
    HANDSHAKE_MSG_TYPE_SERVERHELLO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum handshake_auth {
    HANDSHAKE_AUTH_UNSPEC,
    HANDSHAKE_AUTH_UNAUTH,
    HANDSHAKE_AUTH_PSK,
    HANDSHAKE_AUTH_X509,
}

