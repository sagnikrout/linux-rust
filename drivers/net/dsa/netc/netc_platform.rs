//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/netc/netc_platform.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// NXP NETC switch driver
// Copyright 2025-2026 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_switch_platform {
    pub revision: u16,
    pub info: *const netc_switch_info,
}

    static void imx94_switch_phylink_get_caps(int port,
    struct phylink_config *config)
    {
    config.mac_capabilities = MAC_ASYM_PAUSE | MAC_SYM_PAUSE | MAC_1000FD;
    switch (port) {
    case 0 ... 1:
    __set_bit(PHY_INTERFACE_MODE_SGMII,
    config.supported_interfaces);
    __set_bit(PHY_INTERFACE_MODE_2500BASEX,
    config.supported_interfaces);
    config.mac_capabilities |= MAC_2500FD;
    fallthrough;
    case 2:
    config.mac_capabilities |= MAC_10 | MAC_100;
    __set_bit(PHY_INTERFACE_MODE_MII,
    config.supported_interfaces);
    __set_bit(PHY_INTERFACE_MODE_RMII,
    config.supported_interfaces);
// Port 0 and 1 do not support REVMII
    if (port == 2)
    __set_bit(PHY_INTERFACE_MODE_REVMII,
    config.supported_interfaces);
    phy_interface_set_rgmii(config.supported_interfaces);
    break;
    case 3: /* CPU port */
    __set_bit(PHY_INTERFACE_MODE_INTERNAL,
    config.supported_interfaces);
    config.mac_capabilities |= MAC_10FD | MAC_100FD |
    MAC_2500FD;
    break;
    default:
    break;
    }
    }
    static const struct netc_switch_info imx94_info = {
    .num_ports = 4,
    .phylink_get_caps = imx94_switch_phylink_get_caps,
    };
    static const struct netc_switch_platform netc_platforms[] = {
    { .revision = NETC_SWITCH_REV_4_3, .info = &imx94_info, },
    { }
    };
    static const struct netc_switch_info *
    netc_switch_get_info(struct netc_switch *priv)
    {
    int i;
// Matching based on IP revision
    for (i = 0; i < ARRAY_SIZE(netc_platforms); i++) {
    if (priv.revision == netc_platforms[i].revision)
    return netc_platforms[i].info;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn netc_switch_platform_probe(priv: *mut netc_switch) -> c_int {
    int netc_switch_platform_probe(struct netc_switch *priv)
    {
    const struct netc_switch_info *info = netc_switch_get_info(priv);
    if (!info) {
    dev_err(priv.dev, "Cannot find switch platform info\n");
    return -EINVAL;
    }
    priv.info = info;
    return 0;
    }
