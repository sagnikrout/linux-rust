//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/cortina.c
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
//
// Copyright 2017 NXP
//
// CORTINA is a registered trademark of Cortina Systems, Inc.
//

pub const PHY_ID_CS4340: c_uint = 0x13e51002;
pub const VILLA_GLOBAL_CHIP_ID_LSB: c_uint = 0x0;
pub const VILLA_GLOBAL_CHIP_ID_MSB: c_uint = 0x1;
pub const VILLA_GLOBAL_GPIO_1_INTS: c_uint = 0x017;
#[no_mangle]
unsafe extern "C" fn cortina_read_reg(phydev: *mut phy_device, regnum: u16) -> c_int {
    static int cortina_read_reg(struct phy_device *phydev, u16 regnum)
    {
    return mdiobus_c45_read(phydev.mdio.bus, phydev.mdio.addr, 0, regnum);
    }
#[no_mangle]
unsafe extern "C" fn cortina_read_status(phydev: *mut phy_device) -> c_int {
    static int cortina_read_status(struct phy_device *phydev)
    {
    int gpio_int_status, ret = 0;
    gpio_int_status = cortina_read_reg(phydev, VILLA_GLOBAL_GPIO_1_INTS);
    if (gpio_int_status < 0) {
    ret = gpio_int_status;
    goto err;
    }
    if (gpio_int_status & 0x8) {
// up when edc_convergedS set
    phydev.speed = SPEED_10000;
    phydev.duplex = DUPLEX_FULL;
    phydev.link = 1;
    } else {
    phydev.link = 0;
    }
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cortina_probe(phydev: *mut phy_device) -> c_int {
    static int cortina_probe(struct phy_device *phydev)
    {
    let mut phy_id: u32 = 0;
    let mut id_lsb: c_int = 0, id_msb = 0;
// Read device id from phy registers.
    id_lsb = cortina_read_reg(phydev, VILLA_GLOBAL_CHIP_ID_LSB);
    if (id_lsb < 0)
    return -ENXIO;
    phy_id = id_lsb << 16;
    id_msb = cortina_read_reg(phydev, VILLA_GLOBAL_CHIP_ID_MSB);
    if (id_msb < 0)
    return -ENXIO;
    phy_id |= id_msb;
// Make sure the device tree binding matched the driver with the
// right device.
//
    if (phy_id != phydev.drv.phy_id) {
    phydev_err(phydev, "Error matching phy with %s driver\n",
    phydev.drv.name);
    return -ENODEV;
    }
    return 0;
    }
    static struct phy_driver cortina_driver[] = {
    {
    .phy_id		= PHY_ID_CS4340,
    .phy_id_mask	= 0xffffffff,
    .name		= "Cortina CS4340",
    .features       = PHY_10GBIT_FEATURES,
    .config_aneg	= gen10g_config_aneg,
    .read_status	= cortina_read_status,
    .probe		= cortina_probe,
    },
    };
    module_phy_driver(cortina_driver);
    static const struct mdio_device_id __maybe_unused cortina_tbl[] = {
    { PHY_ID_CS4340, 0xffffffff},
    {},
    };
    MODULE_DEVICE_TABLE(mdio, cortina_tbl);
    MODULE_DESCRIPTION("Cortina EDC CDR 10G Ethernet PHY driver");
    MODULE_AUTHOR("NXP");
    MODULE_LICENSE("GPL");
