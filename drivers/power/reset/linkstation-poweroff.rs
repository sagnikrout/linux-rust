//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/linkstation-poweroff.c
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
//
// LinkStation power off restart driver
// Copyright (C) 2020 Daniel González Cabanelas <dgcbueu@gmail.com>
//

// Defines from the eth phy Marvell driver
pub const MII_MARVELL_COPPER_PAGE: c_int = 0;
pub const MII_MARVELL_LED_PAGE: c_int = 3;
pub const MII_MARVELL_WOL_PAGE: c_int = 17;
pub const MII_MARVELL_PHY_PAGE: c_int = 22;
pub const MII_PHY_LED_CTRL: c_int = 16;
pub const MII_PHY_LED_POL_CTRL: c_int = 17;
pub const MII_88E1318S_PHY_LED_TCR: c_int = 18;
pub const MII_88E1318S_PHY_WOL_CTRL: c_int = 16;
pub const MII_M1011_IEVENT: c_int = 19;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_off_cfg {
    pub mdio_node_name: *mut c_char,
    pub restart): *mut *mut void (phy_set_reg)(bool,
}

    static struct phy_device *phydev;
    static const struct power_off_cfg *cfg;
#[no_mangle]
unsafe extern "C" fn linkstation_mvphy_reg_intn(restart: bool) {
    static void linkstation_mvphy_reg_intn(bool restart)
    {
    let mut rc: c_int = 0, saved_page;
    let mut data: u16 = 0;
    if (restart)
    data = MII_88E1318S_PHY_LED_TCR_FORCE_INT;
    saved_page = phy_select_page(phydev, MII_MARVELL_LED_PAGE);
    if (saved_page < 0)
    goto err;
// Force manual LED2 control to let INTn work
    __phy_modify(phydev, MII_PHY_LED_CTRL, LEDMASK, LED2_FORCE_ON);
// Set the LED[2]/INTn pin to the required state
    __phy_modify(phydev, MII_88E1318S_PHY_LED_TCR,
    MII_88E1318S_PHY_LED_TCR_FORCE_INT,
    MII_88E1318S_PHY_LED_TCR_INTn_ENABLE | data);
    if (!data) {
// Clear interrupts to ensure INTn won't be holded in high state
    __phy_write(phydev, MII_MARVELL_PHY_PAGE, MII_MARVELL_COPPER_PAGE);
    __phy_read(phydev, MII_M1011_IEVENT);
// If WOL was enabled and a magic packet was received before powering
// off, we won't be able to wake up by sending another magic packet.
// Clear WOL status.
//
    __phy_write(phydev, MII_MARVELL_PHY_PAGE, MII_MARVELL_WOL_PAGE);
    __phy_set_bits(phydev, MII_88E1318S_PHY_WOL_CTRL,
    MII_88E1318S_PHY_WOL_CTRL_CLEAR_WOL_STATUS);
    }
    err:
    rc = phy_restore_page(phydev, saved_page, rc);
    if (rc < 0)
    dev_err(&phydev.mdio.dev, "Write register failed, %d\n", rc);
    }
#[no_mangle]
unsafe extern "C" fn readynas_mvphy_set_reg(restart: bool) {
    static void readynas_mvphy_set_reg(bool restart)
    {
    let mut rc: c_int = 0, saved_page;
    let mut data: u16 = 0;
    if (restart)
    data = MII_88E1318S_PHY_LED_POL_LED2;
    saved_page = phy_select_page(phydev, MII_MARVELL_LED_PAGE);
    if (saved_page < 0)
    goto err;
// Set the LED[2].0 Polarity bit to the required state
    __phy_modify(phydev, MII_PHY_LED_POL_CTRL,
    MII_88E1318S_PHY_LED_POL_LED2, data);
    if (!data) {
// If WOL was enabled and a magic packet was received before powering
// off, we won't be able to wake up by sending another magic packet.
// Clear WOL status.
//
    __phy_write(phydev, MII_MARVELL_PHY_PAGE, MII_MARVELL_WOL_PAGE);
    __phy_set_bits(phydev, MII_88E1318S_PHY_WOL_CTRL,
    MII_88E1318S_PHY_WOL_CTRL_CLEAR_WOL_STATUS);
    }
    err:
    rc = phy_restore_page(phydev, saved_page, rc);
    if (rc < 0)
    dev_err(&phydev.mdio.dev, "Write register failed, %d\n", rc);
    }
    static const struct power_off_cfg linkstation_power_off_cfg = {
    .mdio_node_name = "mdio",
    .phy_set_reg = linkstation_mvphy_reg_intn,
    };
    static const struct power_off_cfg readynas_power_off_cfg = {
    .mdio_node_name = "mdio-bus",
    .phy_set_reg = readynas_mvphy_set_reg,
    };
    static int linkstation_reboot_notifier(struct notifier_block *nb,
    unsigned long action, void *unused)
    {
    if (action == SYS_RESTART)
    cfg.phy_set_reg(true);
    return NOTIFY_DONE;
    }
    static struct notifier_block linkstation_reboot_nb = {
    .notifier_call = linkstation_reboot_notifier,
    };
#[no_mangle]
unsafe extern "C" fn linkstation_poweroff() {
    static void linkstation_poweroff(void)
    {
    unregister_reboot_notifier(&linkstation_reboot_nb);
    cfg.phy_set_reg(false);
    kernel_restart("Power off");
    }
    static const struct of_device_id ls_poweroff_of_match[] = {
    { .compatible = "buffalo,ls421d",
    .data = &linkstation_power_off_cfg,
    },
    { .compatible = "buffalo,ls421de",
    .data = &linkstation_power_off_cfg,
    },
    { .compatible = "netgear,readynas-duo-v2",
    .data = &readynas_power_off_cfg,
    },
    { },
    };
#[no_mangle]
unsafe extern "C" fn linkstation_poweroff_init() -> int __init {
    static int __init linkstation_poweroff_init(void)
    {
    struct mii_bus *bus;
    struct device_node *dn;
    const struct of_device_id *match;
    dn = of_find_matching_node(core::ptr::null_mut(), ls_poweroff_of_match);
    if (!dn)
    return -ENODEV;
    match = of_match_node(ls_poweroff_of_match, dn);
    cfg = match.data;
    of_node_put(dn);
    dn = of_find_node_by_name(core::ptr::null_mut(), cfg.mdio_node_name);
    if (!dn)
    return -ENODEV;
    bus = of_mdio_find_bus(dn);
    of_node_put(dn);
    if (!bus)
    return -EPROBE_DEFER;
    phydev = phy_find_first(bus);
    put_device(&bus.dev);
    if (!phydev)
    return -EPROBE_DEFER;
    register_reboot_notifier(&linkstation_reboot_nb);
    pm_power_off = linkstation_poweroff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn linkstation_poweroff_exit() -> void __exit {
    static void __exit linkstation_poweroff_exit(void)
    {
    pm_power_off = core::ptr::null_mut();
    unregister_reboot_notifier(&linkstation_reboot_nb);
    }
    module_init(linkstation_poweroff_init);
    module_exit(linkstation_poweroff_exit);
    MODULE_AUTHOR("Daniel González Cabanelas <dgcbueu@gmail.com>");
    MODULE_DESCRIPTION("LinkStation power off driver");
    MODULE_LICENSE("GPL v2");
