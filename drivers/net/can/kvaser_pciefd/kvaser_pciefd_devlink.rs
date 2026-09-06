//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/kvaser_pciefd/kvaser_pciefd_devlink.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
// kvaser_pciefd devlink functions
//
// Copyright (C) 2025 KVASER AB, Sweden. All rights reserved.
//

    static int kvaser_pciefd_devlink_info_get(struct devlink *devlink,
    struct devlink_info_req *req,
    struct netlink_ext_ack *extack)
    {
    struct kvaser_pciefd *pcie = devlink_priv(devlink);
    char buf[] = "xxx.xxx.xxxxx";
    int ret;
    if (pcie.fw_version.major) {
    snprintf(buf, sizeof(buf), "%u.%u.%u",
    pcie.fw_version.major,
    pcie.fw_version.minor,
    pcie.fw_version.build);
    ret = devlink_info_version_running_put(req,
    DEVLINK_INFO_VERSION_GENERIC_FW,
    buf);
    if (ret)
    return ret;
    }
    return 0;
    }
    const struct devlink_ops kvaser_pciefd_devlink_ops = {
    .info_get = kvaser_pciefd_devlink_info_get,
    };
#[no_mangle]
pub unsafe extern "C" fn kvaser_pciefd_devlink_port_register(can: *mut kvaser_pciefd_can) -> c_int {
    int kvaser_pciefd_devlink_port_register(struct kvaser_pciefd_can *can)
    {
    int ret;
    struct devlink_port_attrs attrs = {
    .flavour = DEVLINK_PORT_FLAVOUR_PHYSICAL,
    .phys.port_number = can.can.dev.dev_port,
    };
    devlink_port_attrs_set(&can.devlink_port, &attrs);
    ret = devlink_port_register(priv_to_devlink(can.kv_pcie),
    &can.devlink_port, can.can.dev.dev_port);
    if (ret)
    return ret;
    SET_NETDEV_DEVLINK_PORT(can.can.dev, &can.devlink_port);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvaser_pciefd_devlink_port_unregister(can: *mut kvaser_pciefd_can) {
    void kvaser_pciefd_devlink_port_unregister(struct kvaser_pciefd_can *can)
    {
    devlink_port_unregister(&can.devlink_port);
    }
