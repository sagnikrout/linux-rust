//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/wireguard.h
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
// Documentation/netlink/specs/wireguard.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const WG_GENL_VERSION: c_int = 1;
pub const WG_KEY_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wgdevice_flag {
    WGDEVICE_F_REPLACE_PEERS = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wgpeer_flag {
    WGPEER_F_REMOVE_ME = 1,
    WGPEER_F_REPLACE_ALLOWEDIPS = 2,
    WGPEER_F_UPDATE_ONLY = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wgallowedip_flag {
    WGALLOWEDIP_F_REMOVE_ME = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wgdevice_attribute {
    WGDEVICE_A_UNSPEC,
    WGDEVICE_A_IFINDEX,
    WGDEVICE_A_IFNAME,
    WGDEVICE_A_PRIVATE_KEY,
    WGDEVICE_A_PUBLIC_KEY,
    WGDEVICE_A_FLAGS,
    WGDEVICE_A_LISTEN_PORT,
    WGDEVICE_A_FWMARK,
    WGDEVICE_A_PEERS,

    __WGDEVICE_A_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wgpeer_attribute {
    WGPEER_A_UNSPEC,
    WGPEER_A_PUBLIC_KEY,
    WGPEER_A_PRESHARED_KEY,
    WGPEER_A_FLAGS,
    WGPEER_A_ENDPOINT,
    WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
    WGPEER_A_LAST_HANDSHAKE_TIME,
    WGPEER_A_RX_BYTES,
    WGPEER_A_TX_BYTES,
    WGPEER_A_ALLOWEDIPS,
    WGPEER_A_PROTOCOL_VERSION,

    __WGPEER_A_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wgallowedip_attribute {
    WGALLOWEDIP_A_UNSPEC,
    WGALLOWEDIP_A_FAMILY,
    WGALLOWEDIP_A_IPADDR,
    WGALLOWEDIP_A_CIDR_MASK,
    WGALLOWEDIP_A_FLAGS,

    __WGALLOWEDIP_A_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wg_cmd {
    WG_CMD_GET_DEVICE,
    WG_CMD_SET_DEVICE,

    __WG_CMD_MAX
}

