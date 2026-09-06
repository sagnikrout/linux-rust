//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/dpaa/dpaa_eth_sysfs.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2008 - 2016 Freescale Semiconductor Inc.
//

    static ssize_t dpaa_eth_show_addr(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dpaa_priv *priv = netdev_priv(to_net_dev(dev));
    struct mac_device *mac_dev = priv.mac_dev;
    if (mac_dev)
    return sprintf(buf, "%llx",
    (unsigned long long)mac_dev.res.start);
    else
    return sprintf(buf, "none");
    }
    static ssize_t dpaa_eth_show_fqids(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dpaa_priv *priv = netdev_priv(to_net_dev(dev));
    struct dpaa_fq *prev = core::ptr::null_mut();
    char *prevstr = core::ptr::null_mut();
    struct dpaa_fq *tmp;
    struct dpaa_fq *fq;
    let mut first_fqid: u32 = 0;
    let mut last_fqid: u32 = 0;
    let mut bytes: isize = 0;
    char *str;
    list_for_each_entry_safe(fq, tmp, &priv.dpaa_fq_list, list) {
    switch (fq.fq_type) {
    case FQ_TYPE_RX_DEFAULT:
    str = "Rx default";
    break;
    case FQ_TYPE_RX_ERROR:
    str = "Rx error";
    break;
    case FQ_TYPE_RX_PCD:
    str = "Rx PCD";
    break;
    case FQ_TYPE_TX_CONFIRM:
    str = "Tx default confirmation";
    break;
    case FQ_TYPE_TX_CONF_MQ:
    str = "Tx confirmation (mq)";
    break;
    case FQ_TYPE_TX_ERROR:
    str = "Tx error";
    break;
    case FQ_TYPE_TX:
    str = "Tx";
    break;
    default:
    str = "Unknown";
    }
    if (prev && (abs(fq.fqid - prev.fqid) != 1 ||
    str != prevstr)) {
    if (last_fqid == first_fqid)
    bytes += sprintf(buf + bytes,
    "%s: %d\n", prevstr, prev.fqid);
    else
    bytes += sprintf(buf + bytes,
    "%s: %d - %d\n", prevstr,
    first_fqid, last_fqid);
    }
    if (prev && abs(fq.fqid - prev.fqid) == 1 &&
    str == prevstr) {
    last_fqid = fq.fqid;
    } else {
    first_fqid = fq.fqid;
    last_fqid = fq.fqid;
    }
    prev = fq;
    prevstr = str;
    }
    if (prev) {
    if (last_fqid == first_fqid)
    bytes += sprintf(buf + bytes, "%s: %d\n", prevstr,
    prev.fqid);
    else
    bytes += sprintf(buf + bytes, "%s: %d - %d\n", prevstr,
    first_fqid, last_fqid);
    }
    return bytes;
    }
    static ssize_t dpaa_eth_show_bpids(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dpaa_priv *priv = netdev_priv(to_net_dev(dev));
    let mut bytes: isize = 0;
    bytes += snprintf(buf + bytes, PAGE_SIZE - bytes, "%u\n",
    priv.dpaa_bp.bpid);
    return bytes;
    }
    static struct device_attribute dpaa_eth_attrs[] = {
    __ATTR(device_addr, 0444, dpaa_eth_show_addr, core::ptr::null_mut()),
    __ATTR(fqids, 0444, dpaa_eth_show_fqids, core::ptr::null_mut()),
    __ATTR(bpids, 0444, dpaa_eth_show_bpids, core::ptr::null_mut()),
    };
#[no_mangle]
pub unsafe extern "C" fn dpaa_eth_sysfs_init(dev: *mut device) {
    void dpaa_eth_sysfs_init(struct device *dev)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(dpaa_eth_attrs); i++)
    if (device_create_file(dev, &dpaa_eth_attrs[i])) {
    dev_err(dev, "Error creating sysfs file\n");
    while (i > 0)
    device_remove_file(dev, &dpaa_eth_attrs[--i]);
    return;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dpaa_eth_sysfs_remove(dev: *mut device) {
    void dpaa_eth_sysfs_remove(struct device *dev)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(dpaa_eth_attrs); i++)
    device_remove_file(dev, &dpaa_eth_attrs[i]);
    }
