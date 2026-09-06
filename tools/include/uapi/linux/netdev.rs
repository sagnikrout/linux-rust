//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/netdev.h
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
// Documentation/netlink/specs/netdev.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const NETDEV_FAMILY_VERSION: c_int = 1;
//
// enum netdev_xdp_act
// @NETDEV_XDP_ACT_BASIC: XDP features set supported by all drivers
// (XDP_ABORTED, XDP_DROP, XDP_PASS, XDP_TX)
// @NETDEV_XDP_ACT_REDIRECT: The netdev supports XDP_REDIRECT
// @NETDEV_XDP_ACT_NDO_XMIT: This feature informs if netdev implements
// ndo_xdp_xmit callback.
// @NETDEV_XDP_ACT_XSK_ZEROCOPY: This feature informs if netdev supports AF_XDP
// in zero copy mode.
// @NETDEV_XDP_ACT_HW_OFFLOAD: This feature informs if netdev supports XDP hw
// offloading.
// @NETDEV_XDP_ACT_RX_SG: This feature informs if netdev implements non-linear
// XDP buffer support in the driver napi callback.
// @NETDEV_XDP_ACT_NDO_XMIT_SG: This feature informs if netdev implements
// non-linear XDP buffer support in ndo_xdp_xmit callback.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_xdp_act {
    NETDEV_XDP_ACT_BASIC = 1,
    NETDEV_XDP_ACT_REDIRECT = 2,
    NETDEV_XDP_ACT_NDO_XMIT = 4,
    NETDEV_XDP_ACT_XSK_ZEROCOPY = 8,
    NETDEV_XDP_ACT_HW_OFFLOAD = 16,
    NETDEV_XDP_ACT_RX_SG = 32,
    NETDEV_XDP_ACT_NDO_XMIT_SG = 64,

// private:
    NETDEV_XDP_ACT_MASK = 127,
}

//
// enum netdev_xdp_rx_metadata
// @NETDEV_XDP_RX_METADATA_TIMESTAMP: Device is capable of exposing receive HW
// timestamp via bpf_xdp_metadata_rx_timestamp().
// @NETDEV_XDP_RX_METADATA_HASH: Device is capable of exposing receive packet
// hash via bpf_xdp_metadata_rx_hash().
// @NETDEV_XDP_RX_METADATA_VLAN_TAG: Device is capable of exposing receive
// packet VLAN tag via bpf_xdp_metadata_rx_vlan_tag().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_xdp_rx_metadata {
    NETDEV_XDP_RX_METADATA_TIMESTAMP = 1,
    NETDEV_XDP_RX_METADATA_HASH = 2,
    NETDEV_XDP_RX_METADATA_VLAN_TAG = 4,
}

//
// enum netdev_xsk_flags
// @NETDEV_XSK_FLAGS_TX_TIMESTAMP: HW timestamping egress packets is supported
// by the driver.
// @NETDEV_XSK_FLAGS_TX_CHECKSUM: L3 checksum HW offload is supported by the
// driver.
// @NETDEV_XSK_FLAGS_TX_LAUNCH_TIME_FIFO: Launch time HW offload is supported
// by the driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_xsk_flags {
    NETDEV_XSK_FLAGS_TX_TIMESTAMP = 1,
    NETDEV_XSK_FLAGS_TX_CHECKSUM = 2,
    NETDEV_XSK_FLAGS_TX_LAUNCH_TIME_FIFO = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_queue_type {
    NETDEV_QUEUE_TYPE_RX,
    NETDEV_QUEUE_TYPE_TX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_qstats_scope {
    NETDEV_QSTATS_SCOPE_QUEUE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_napi_threaded {
    NETDEV_NAPI_THREADED_DISABLED,
    NETDEV_NAPI_THREADED_ENABLED,
    NETDEV_NAPI_THREADED_BUSY_POLL,
}

