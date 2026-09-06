//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/tehuti/tn40_phy.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) Tehuti Networks Ltd.

    static struct tn40_priv *tn40_config_to_priv(struct phylink_config *config)
    {
    return container_of(config, struct tn40_priv, phylink_config);
    }
    static void tn40_link_up(struct phylink_config *config, struct phy_device *phy,
    unsigned int mode, phy_interface_t interface,
    int speed, int duplex, bool tx_pause, bool rx_pause)
    {
    struct tn40_priv *priv = tn40_config_to_priv(config);
    tn40_set_link_speed(priv, speed);
    netif_wake_queue(priv.ndev);
    }
    static void tn40_link_down(struct phylink_config *config, unsigned int mode,
    phy_interface_t interface)
    {
    struct tn40_priv *priv = tn40_config_to_priv(config);
    netif_stop_queue(priv.ndev);
    tn40_set_link_speed(priv, 0);
    }
    static void tn40_mac_config(struct phylink_config *config, unsigned int mode,
    const struct phylink_link_state *state)
    {
    }
    static const struct phylink_mac_ops tn40_mac_ops = {
    .mac_config = tn40_mac_config,
    .mac_link_up = tn40_link_up,
    .mac_link_down = tn40_link_down,
    };
#[no_mangle]
pub unsafe extern "C" fn tn40_phy_register(priv: *mut tn40_priv) -> c_int {
    int tn40_phy_register(struct tn40_priv *priv)
    {
    struct phylink_config *config;
    struct phy_device *phydev;
    struct phylink *phylink;
    phydev = phy_find_first(priv.mdio);
    if (!phydev) {
    dev_err(&priv.pdev.dev, "PHY isn't found\n");
    return -ENODEV;
    }
    config = &priv.phylink_config;
    config.dev = &priv.ndev.dev;
    config.type = PHYLINK_NETDEV;
    config.mac_capabilities = MAC_10000FD;
    __set_bit(PHY_INTERFACE_MODE_XAUI, config.supported_interfaces);
    phylink = phylink_create(config, core::ptr::null_mut(), PHY_INTERFACE_MODE_XAUI,
    &tn40_mac_ops);
    if (IS_ERR(phylink))
    return PTR_ERR(phylink);
    priv.phydev = phydev;
    priv.phylink = phylink;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tn40_phy_unregister(priv: *mut tn40_priv) {
    void tn40_phy_unregister(struct tn40_priv *priv)
    {
    phylink_destroy(priv.phylink);
    }
