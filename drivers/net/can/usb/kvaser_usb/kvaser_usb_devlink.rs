//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/usb/kvaser_usb/kvaser_usb_devlink.c
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


// SPDX-License-Identifier: GPL-2.0
// kvaser_usb devlink functions
//
// Copyright (C) 2025 KVASER AB, Sweden. All rights reserved.
//

pub const KVASER_USB_EAN_MSB: c_uint = 0x00073301;
    static int kvaser_usb_devlink_info_get(struct devlink *devlink,
    struct devlink_info_req *req,
    struct netlink_ext_ack *extack)
    {
    struct kvaser_usb *dev = devlink_priv(devlink);
    char buf[] = "73301XXXXXXXXXX";
    int ret;
    if (dev.serial_number) {
    snprintf(buf, sizeof(buf), "%u", dev.serial_number);
    ret = devlink_info_serial_number_put(req, buf);
    if (ret)
    return ret;
    }
    if (dev.fw_version.major) {
    snprintf(buf, sizeof(buf), "%u.%u.%u",
    dev.fw_version.major,
    dev.fw_version.minor,
    dev.fw_version.build);
    ret = devlink_info_version_running_put(req,
    DEVLINK_INFO_VERSION_GENERIC_FW,
    buf);
    if (ret)
    return ret;
    }
    if (dev.hw_revision) {
    snprintf(buf, sizeof(buf), "%u", dev.hw_revision);
    ret = devlink_info_version_fixed_put(req,
    DEVLINK_INFO_VERSION_GENERIC_BOARD_REV,
    buf);
    if (ret)
    return ret;
    }
    if (dev.ean[1] == KVASER_USB_EAN_MSB) {
    snprintf(buf, sizeof(buf), "%x%08x", dev.ean[1], dev.ean[0]);
    ret = devlink_info_version_fixed_put(req,
    DEVLINK_INFO_VERSION_GENERIC_BOARD_ID,
    buf);
    if (ret)
    return ret;
    }
    return 0;
    }
    const struct devlink_ops kvaser_usb_devlink_ops = {
    .info_get = kvaser_usb_devlink_info_get,
    };
#[no_mangle]
pub unsafe extern "C" fn kvaser_usb_devlink_port_register(priv: *mut kvaser_usb_net_priv) -> c_int {
    int kvaser_usb_devlink_port_register(struct kvaser_usb_net_priv *priv)
    {
    int ret;
    struct devlink_port_attrs attrs = {
    .flavour = DEVLINK_PORT_FLAVOUR_PHYSICAL,
    .phys.port_number = priv.channel,
    };
    devlink_port_attrs_set(&priv.devlink_port, &attrs);
    ret = devlink_port_register(priv_to_devlink(priv.dev),
    &priv.devlink_port, priv.channel);
    if (ret)
    return ret;
    SET_NETDEV_DEVLINK_PORT(priv.netdev, &priv.devlink_port);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvaser_usb_devlink_port_unregister(priv: *mut kvaser_usb_net_priv) {
    void kvaser_usb_devlink_port_unregister(struct kvaser_usb_net_priv *priv)
    {
    devlink_port_unregister(&priv.devlink_port);
    }
