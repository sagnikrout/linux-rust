//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mdio.h
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
// linux/mdio.h: definitions for MDIO (clause 45) transceivers
// Copyright 2006-2009 Solarflare Communications Inc.
//

// Multiple levels of nesting are possible. However typically this is
// limited to nested DSA like layer, a MUX layer, and the normal
// user. Instead of trying to handle the general case, just define
// these cases.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdio_mutex_lock_class {
    MDIO_MUTEX_NORMAL,
    MDIO_MUTEX_MUX,
    MDIO_MUTEX_NESTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_device {
    pub dev: device,
    pub bus: *mut mii_bus,
    pub drv): *const *const *const int (bus_match)(struct device dev, struct device_driver,
    pub mdiodev): *mut *mut void (device_free)(struct mdio_device,
    pub mdiodev): *mut *mut void (device_remove)(struct mdio_device,
// Bus address of the MDIO device (0-31)
    pub addr: c_int,
    pub flags: c_int,
    pub reset_state: c_int,
    pub reset_gpio: *mut gpio_desc,
    pub reset_ctrl: *mut reset_control,
    pub reset_assert_delay: c_uint,
    pub reset_deassert_delay: c_uint,
}

// struct mdio_driver_common: Common to all MDIO drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_driver_common {
    pub driver: device_driver,
    pub flags: c_int,
}

pub const MDIO_DEVICE_FLAG_PHY: c_int = 1;

// struct mdio_driver: Generic MDIO driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_driver {
    pub mdiodrv: mdio_driver_common,
//
// Called during discovery.  Used to set
// up device-specific structures, if any
//
    pub mdiodev): *mut *mut int (probe)(struct mdio_device,
// Clears up any memory if needed
    pub mdiodev): *mut *mut void (remove)(struct mdio_device,
// Quiesces the device on system shutdown, turns off interrupts etc
    pub mdiodev): *mut *mut void (shutdown)(struct mdio_device,
}

// device driver data
extern "C" {
    pub fn dev_get_drvdata(_arg: &mdio->dev) -> return;
}
extern "C" {
    pub fn mdio_device_free(mdiodev: *mut mdio_device);
}
extern "C" {
    pub fn mdio_device_register(mdiodev: *mut mdio_device) -> c_int;
}
extern "C" {
    pub fn mdio_device_remove(mdiodev: *mut mdio_device);
}
extern "C" {
    pub fn mdio_device_reset(mdiodev: *mut mdio_device, value: c_int);
}
extern "C" {
    pub fn mdio_driver_register(drv: *mut mdio_driver) -> c_int;
}
extern "C" {
    pub fn mdio_driver_unregister(drv: *mut mdio_driver);
}
//
// struct mdio_if_info - Ethernet controller MDIO interface
// @prtad: PRTAD of the PHY (%MDIO_PRTAD_NONE if not present/unknown)
// @mmds: Mask of MMDs expected to be present in the PHY.  This must be
// non-zero unless @prtad = %MDIO_PRTAD_NONE.
// @mode_support: MDIO modes supported.  If %MDIO_SUPPORTS_C22 is set then
// MII register access will be passed through with @devad =
// %MDIO_DEVAD_NONE.  If %MDIO_EMULATE_C22 is set then access to
// commonly used clause 22 registers will be translated into
// clause 45 registers.
// @dev: Net device structure
// @mdio_read: Register read function; returns value or negative error code
// @mdio_write: Register write function; returns 0 or negative error code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_if_info {
    pub prtad: c_int,
    pub mmds: u32,
    pub mode_support: unsigned,
    pub dev: *mut net_device,
    pub addr): u16,
    pub val): u16 addr, u16,
}

pub const MDIO_SUPPORTS_C22: c_int = 1;
pub const MDIO_SUPPORTS_C45: c_int = 2;
pub const MDIO_EMULATE_C22: c_int = 4;
extern "C" {
    pub fn mdio45_probe(mdio: *mut mdio_if_info, prtad: c_int) -> c_int;
}
extern "C" {
    pub fn mdio45_links_ok(mdio: *const mdio_if_info, mmds: u32) -> c_int;
}
extern "C" {
    pub fn mdio45_nway_restart(mdio: *const mdio_if_info) -> c_int;
}
//
// mdio45_ethtool_ksettings_get - get settings for ETHTOOL_GLINKSETTINGS
// @mdio: MDIO interface
// @cmd: Ethtool request structure
//
// Since the CSRs for auto-negotiation using next pages are not fully
// standardised, this function does not attempt to decode them.  Use
// mdio45_ethtool_ksettings_get_npage() to specify advertisement bits
// from next pages.
//
// mmd_eee_cap_to_ethtool_sup_t
// @eee_cap: value of the MMD EEE Capability register
//
// A small helper function that translates MMD EEE Capability (3.20) bits
// to ethtool supported settings.
//
// mmd_eee_adv_to_ethtool_adv_t
// @eee_adv: value of the MMD EEE Advertisement/Link Partner Ability registers
//
// A small helper function that translates the MMD EEE Advertisment (7.60)
// and MMD EEE Link Partner Ability (7.61) bits to ethtool advertisement
// settings.
//
// ethtool_adv_to_mmd_eee_adv_t
// @adv: the ethtool advertisement settings
//
// A small helper function that translates ethtool advertisement settings
// to EEE advertisements for the MMD EEE Advertisement (7.60) and
// MMD EEE Link Partner Ability (7.61) registers.
//
// linkmode_adv_to_mii_10gbt_adv_t
// @advertising: the linkmode advertisement settings
//
// A small helper function that translates linkmode advertisement
// settings to phy autonegotiation advertisements for the C45
// 10GBASE-T AN CONTROL (7.32) register.
//
// mii_10gbt_stat_mod_linkmode_lpa_t
// @advertising: target the linkmode advertisement settings
// @lpa: value of the C45 10GBASE-T AN STATUS register
//
// A small helper function that translates C45 10GBASE-T AN STATUS register bits
// to linkmode advertisement settings. Other bits in advertising aren't changed.
//
// mii_t1_adv_l_mod_linkmode_t
// @advertising: target the linkmode advertisement settings
// @lpa: value of the BASE-T1 Autonegotiation Advertisement [15:0] Register
//
// A small helper function that translates BASE-T1 Autonegotiation
// Advertisement [15:0] Register bits to linkmode advertisement settings.
// Other bits in advertising aren't changed.
//
// mii_t1_adv_m_mod_linkmode_t
// @advertising: target the linkmode advertisement settings
// @lpa: value of the BASE-T1 Autonegotiation Advertisement [31:16] Register
//
// A small helper function that translates BASE-T1 Autonegotiation
// Advertisement [31:16] Register bits to linkmode advertisement settings.
// Other bits in advertising aren't changed.
//
// linkmode_adv_to_mii_t1_adv_l_t
// @advertising: the linkmode advertisement settings
//
// A small helper function that translates linkmode advertisement
// settings to phy autonegotiation advertisements for the
// BASE-T1 Autonegotiation Advertisement [15:0] Register.
//
// linkmode_adv_to_mii_t1_adv_m_t
// @advertising: the linkmode advertisement settings
//
// A small helper function that translates linkmode advertisement
// settings to phy autonegotiation advertisements for the
// BASE-T1 Autonegotiation Advertisement [31:16] Register.
//
// mii_eee_cap1_mod_linkmode_t()
// @adv: target the linkmode advertisement settings
// @val: register value
//
// A function that translates value of following registers to the linkmode:
// IEEE 802.3-2018 45.2.3.10 "EEE control and capability 1" register (3.20)
// IEEE 802.3-2018 45.2.7.13 "EEE advertisement 1" register (7.60)
// IEEE 802.3-2018 45.2.7.14 "EEE link partner ability 1" register (7.61)
//
// mii_eee_cap2_mod_linkmode_sup_t()
// @adv: target the linkmode settings
// @val: register value
//
// A function that translates value of following registers to the linkmode:
// IEEE 802.3-2022 45.2.3.11 "EEE control and capability 2" register (3.21)
//
// mii_eee_cap2_mod_linkmode_adv_t()
// @adv: target the linkmode advertisement settings
// @val: register value
//
// A function that translates value of following registers to the linkmode:
// IEEE 802.3-2022 45.2.7.16 "EEE advertisement 2" register (7.62)
// IEEE 802.3-2022 45.2.7.17 "EEE link partner ability 2" register (7.63)
// Note: Currently this function is the same as mii_eee_cap2_mod_linkmode_sup_t.
// For certain, not yet supported, modes however the bits differ.
// Therefore create separate functions already.
//
// linkmode_to_mii_eee_cap1_t()
// @adv: the linkmode advertisement settings
//
// A function that translates linkmode to value for IEEE 802.3-2018 45.2.7.13
// "EEE advertisement 1" register (7.60)
//
// linkmode_to_mii_eee_cap2_t()
// @adv: the linkmode advertisement settings
//
// A function that translates linkmode to value for IEEE 802.3-2022 45.2.7.16
// "EEE advertisement 2" register (7.62)
//
// mii_10base_t1_adv_mod_linkmode_t()
// @adv: linkmode advertisement settings
// @val: register value
//
// A function that translates IEEE 802.3cg-2019 45.2.7.26 "10BASE-T1 AN status"
// register (7.527) value to the linkmode.
//
// linkmode_adv_to_mii_10base_t1_t()
// @adv: linkmode advertisement settings
//
// A function that translates the linkmode to IEEE 802.3cg-2019 45.2.7.25
// "10BASE-T1 AN control" register (7.526) value.
//
// mii_c73_mod_linkmode - convert a Clause 73 advertisement to linkmodes
// @adv: linkmode advertisement setting
// @lpa: array of three u16s containing the advertisement
//
// Convert an IEEE 802.3 Clause 73 advertisement to ethtool link modes.
//
// 100GBASE_CR10 and 100GBASE_KP4 not implemented
// 25GBASE_R_S not implemented
// The 25GBASE_R bit can be used for 25Gbase KR or CR modes
// 5GBASE_KR not implemented
extern "C" {
    pub fn __mdiobus_read(bus: *mut mii_bus, addr: c_int, regnum: u32) -> c_int;
}
extern "C" {
    pub fn __mdiobus_write(bus: *mut mii_bus, addr: c_int, regnum: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn mdiobus_read(bus: *mut mii_bus, addr: c_int, regnum: u32) -> c_int;
}
extern "C" {
    pub fn mdiobus_read_nested(bus: *mut mii_bus, addr: c_int, regnum: u32) -> c_int;
}
extern "C" {
    pub fn mdiobus_write(bus: *mut mii_bus, addr: c_int, regnum: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn mdiobus_write_nested(bus: *mut mii_bus, addr: c_int, regnum: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn __mdiobus_c45_read(bus: *mut mii_bus, addr: c_int, devad: c_int, regnum: u32) -> c_int;
}
extern "C" {
    pub fn mdiobus_c45_read(bus: *mut mii_bus, addr: c_int, devad: c_int, regnum: u32) -> c_int;
}
extern "C" {
    pub fn __mdiobus_read(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: regnum) -> return;
}
extern "C" {
    pub fn __mdiobus_write(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: regnum, _arg: val) -> return;
}
extern "C" {
    pub fn __mdiobus_modify(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: regnum, _arg: mask, _arg: set) -> return;
}
extern "C" {
    pub fn mdiobus_read(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: regnum) -> return;
}
extern "C" {
    pub fn mdiobus_write(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: regnum, _arg: val) -> return;
}
extern "C" {
    pub fn mdiobus_modify(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: regnum, _arg: mask, _arg: set) -> return;
}
extern "C" {
    pub fn __mdiobus_c45_read(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: devad, _arg: regnum) -> return;
}
extern "C" {
    pub fn mdiobus_c45_read(_arg: mdiodev->bus, _arg: mdiodev->addr, _arg: devad, _arg: regnum) -> return;
}
extern "C" {
    pub fn mdiobus_is_registered_device(bus: *mut mii_bus, addr: c_int) -> bool;
}
//
// mdio_module_driver() - Helper macro for registering mdio drivers
// @_mdio_driver: driver to register
//
// Helper macro for MDIO drivers which do not do anything special in module
// init/exit. Each module may only use this macro once, and calling it
// replaces module_init() and module_exit().
//

