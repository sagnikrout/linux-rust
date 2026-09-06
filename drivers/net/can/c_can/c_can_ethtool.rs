//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/c_can/c_can_ethtool.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2021, Dario Binacchi <dariobin@libero.it>
//

    static void c_can_get_ringparam(struct net_device *netdev,
    struct ethtool_ringparam *ring,
    struct kernel_ethtool_ringparam *kernel_ring,
    struct netlink_ext_ack *extack)
    {
    struct c_can_priv *priv = netdev_priv(netdev);
    ring.rx_max_pending = priv.msg_obj_num;
    ring.tx_max_pending = priv.msg_obj_num;
    ring.rx_pending = priv.msg_obj_rx_num;
    ring.tx_pending = priv.msg_obj_tx_num;
    }
    const struct ethtool_ops c_can_ethtool_ops = {
    .get_ringparam = c_can_get_ringparam,
    .get_ts_info = ethtool_op_get_ts_info,
    };
