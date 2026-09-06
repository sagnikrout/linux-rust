//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/ocelot/ocelot_ext.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright 2021-2022 Innovative Advantage Inc.
//

pub const VSC7514_NUM_PORTS: c_int = 11;

    OCELOT_PORT_MODE_QSGMII)
    static const u32 vsc7512_port_modes[VSC7514_NUM_PORTS] = {
    OCELOT_PORT_MODE_INTERNAL,
    OCELOT_PORT_MODE_INTERNAL,
    OCELOT_PORT_MODE_INTERNAL,
    OCELOT_PORT_MODE_INTERNAL,
    OCELOT_PORT_MODE_SERDES,
    OCELOT_PORT_MODE_SERDES,
    OCELOT_PORT_MODE_SERDES,
    OCELOT_PORT_MODE_SERDES,
    OCELOT_PORT_MODE_SERDES,
    OCELOT_PORT_MODE_SGMII,
    OCELOT_PORT_MODE_SERDES,
    };
    static const struct ocelot_ops ocelot_ext_ops = {
    .reset		= ocelot_reset,
    .wm_enc		= ocelot_wm_enc,
    .wm_dec		= ocelot_wm_dec,
    .wm_stat	= ocelot_wm_stat,
    .port_to_netdev	= felix_port_to_netdev,
    .netdev_to_port	= felix_netdev_to_port,
    };
    static const char * const vsc7512_resource_names[TARGET_MAX] = {
    [SYS] = "sys",
    [REW] = "rew",
    [S0] = "s0",
    [S1] = "s1",
    [S2] = "s2",
    [QS] = "qs",
    [QSYS] = "qsys",
    [ANA] = "ana",
    };
    static const struct felix_info vsc7512_info = {
    .resource_names			= vsc7512_resource_names,
    .regfields			= vsc7514_regfields,
    .map				= vsc7514_regmap,
    .ops				= &ocelot_ext_ops,
    .vcap				= vsc7514_vcap_props,
    .num_mact_rows			= 1024,
    .num_ports			= VSC7514_NUM_PORTS,
    .port_modes			= vsc7512_port_modes,
    .phylink_mac_config		= ocelot_phylink_mac_config,
    .configure_serdes		= ocelot_port_configure_serdes,
    };
#[no_mangle]
unsafe extern "C" fn ocelot_ext_probe(pdev: *mut platform_device) -> c_int {
    static int ocelot_ext_probe(struct platform_device *pdev)
    {
    return felix_register_switch(&pdev.dev, 0, 1, false, false,
    DSA_TAG_PROTO_OCELOT, &vsc7512_info);
    }
#[no_mangle]
unsafe extern "C" fn ocelot_ext_remove(pdev: *mut platform_device) {
    static void ocelot_ext_remove(struct platform_device *pdev)
    {
    struct felix *felix = dev_get_drvdata(&pdev.dev);
    if (!felix)
    return;
    dsa_unregister_switch(felix.ds);
    }
#[no_mangle]
unsafe extern "C" fn ocelot_ext_shutdown(pdev: *mut platform_device) {
    static void ocelot_ext_shutdown(struct platform_device *pdev)
    {
    struct felix *felix = dev_get_drvdata(&pdev.dev);
    if (!felix)
    return;
    dsa_switch_shutdown(felix.ds);
    dev_set_drvdata(&pdev.dev, core::ptr::null_mut());
    }
    static const struct of_device_id ocelot_ext_switch_of_match[] = {
    { .compatible = "mscc,vsc7512-switch" },
    { },
    };
    MODULE_DEVICE_TABLE(of, ocelot_ext_switch_of_match);
    static struct platform_driver ocelot_ext_switch_driver = {
    .driver = {
    .name = "ocelot-ext-switch",
    .of_match_table = ocelot_ext_switch_of_match,
    },
    .probe = ocelot_ext_probe,
    .remove = ocelot_ext_remove,
    .shutdown = ocelot_ext_shutdown,
    };
    module_platform_driver(ocelot_ext_switch_driver);
    MODULE_DESCRIPTION("External Ocelot Switch driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("MFD_OCELOT");
