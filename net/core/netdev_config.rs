//! Automatically rewritten from C to Rust
//! Source: net/core/netdev_config.c
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

    static int netdev_nop_validate_qcfg(struct net_device *dev,
    struct netdev_queue_config *qcfg,
    struct netlink_ext_ack *extack)
    {
    return 0;
    }
    static int __netdev_queue_config(struct net_device *dev, int rxq_idx,
    struct netdev_queue_config *qcfg,
    struct netlink_ext_ack *extack,
    bool validate)
    {
    int (*validate_cb)(struct net_device *dev,
    struct netdev_queue_config *qcfg,
    struct netlink_ext_ack *extack);
    struct pp_memory_provider_params *mpp;
    int err;
    validate_cb = netdev_nop_validate_qcfg;
    if (validate && dev.queue_mgmt_ops.ndo_validate_qcfg)
    validate_cb = dev.queue_mgmt_ops.ndo_validate_qcfg;
    memset(qcfg, 0, sizeof(*qcfg));
// Get defaults from the driver, in case user config not set
    if (dev.queue_mgmt_ops.ndo_default_qcfg)
    dev.queue_mgmt_ops.ndo_default_qcfg(dev, qcfg);
    err = validate_cb(dev, qcfg, extack);
    if (err)
    return err;
// Apply MP overrides
    mpp = &__netif_get_rx_queue(dev, rxq_idx).mp_params;
    if (mpp.rx_page_size)
    qcfg.rx_page_size = mpp.rx_page_size;
    err = validate_cb(dev, qcfg, extack);
    if (err)
    return err;
    return 0;
    }
//
// netdev_queue_config() - get configuration for a given queue
// @dev:      net_device instance
// @rxq_idx:  index of the queue of interest
// @qcfg: queue configuration struct (output)
//
// Render the configuration for a given queue. This helper should be used
// by drivers which support queue configuration to retrieve config for
// a particular queue.
//
// @qcfg is an output parameter and is always fully initialized by this
// function. Some values may not be set by the user, drivers may either
// deal with the "unset" values in @qcfg, or provide the callback
// to populate defaults in queue_management_ops.
//
    void netdev_queue_config(struct net_device *dev, int rxq_idx,
    struct netdev_queue_config *qcfg)
    {
    __netdev_queue_config(dev, rxq_idx, qcfg, core::ptr::null_mut(), false);
    }
    EXPORT_SYMBOL(netdev_queue_config);
    int netdev_queue_config_validate(struct net_device *dev, int rxq_idx,
    struct netdev_queue_config *qcfg,
    struct netlink_ext_ack *extack)
    {
    return __netdev_queue_config(dev, rxq_idx, qcfg, extack, true);
    }
