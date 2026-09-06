//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/asix.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ASIX AX8817X based USB 2.0 Ethernet Devices
// Copyright (C) 2003-2006 David Hollis <dhollis@davehollis.com>
// Copyright (C) 2005 Phil Chang <pchang23@sbcglobal.net>
// Copyright (C) 2006 James Painter <jamie.painter@iname.com>
// Copyright (c) 2002-2003 TiVo Inc.
//
// #define	DEBUG			// error path messages, extra info
// #define	VERBOSE			// more; success messages

// ASIX AX8817X based USB 2.0 Ethernet Devices
pub const AX_CMD_SET_SW_MII: c_uint = 0x06;
pub const AX_CMD_READ_MII_REG: c_uint = 0x07;
pub const AX_CMD_WRITE_MII_REG: c_uint = 0x08;
pub const AX_CMD_STATMNGSTS_REG: c_uint = 0x09;
pub const AX_CMD_SET_HW_MII: c_uint = 0x0a;
pub const AX_CMD_READ_EEPROM: c_uint = 0x0b;
pub const AX_CMD_WRITE_EEPROM: c_uint = 0x0c;
pub const AX_CMD_WRITE_ENABLE: c_uint = 0x0d;
pub const AX_CMD_WRITE_DISABLE: c_uint = 0x0e;
pub const AX_CMD_READ_RX_CTL: c_uint = 0x0f;
pub const AX_CMD_WRITE_RX_CTL: c_uint = 0x10;
pub const AX_CMD_READ_IPG012: c_uint = 0x11;
pub const AX_CMD_WRITE_IPG0: c_uint = 0x12;
pub const AX_CMD_WRITE_IPG1: c_uint = 0x13;
pub const AX_CMD_READ_NODE_ID: c_uint = 0x13;
pub const AX_CMD_WRITE_NODE_ID: c_uint = 0x14;
pub const AX_CMD_WRITE_IPG2: c_uint = 0x14;
pub const AX_CMD_WRITE_MULTI_FILTER: c_uint = 0x16;
pub const AX88172_CMD_READ_NODE_ID: c_uint = 0x17;
pub const AX_CMD_READ_PHY_ID: c_uint = 0x19;
pub const AX_CMD_READ_MEDIUM_STATUS: c_uint = 0x1a;
pub const AX_CMD_WRITE_MEDIUM_MODE: c_uint = 0x1b;
pub const AX_CMD_READ_MONITOR_MODE: c_uint = 0x1c;
pub const AX_CMD_WRITE_MONITOR_MODE: c_uint = 0x1d;
pub const AX_CMD_READ_GPIOS: c_uint = 0x1e;
pub const AX_CMD_WRITE_GPIOS: c_uint = 0x1f;
pub const AX_CMD_SW_RESET: c_uint = 0x20;
pub const AX_CMD_SW_PHY_STATUS: c_uint = 0x21;
pub const AX_CMD_SW_PHY_SELECT: c_uint = 0x22;
pub const AX_QCTCTRL: c_uint = 0x2A;
pub const AX_CHIPCODE_MASK: c_uint = 0x70;
pub const AX_AX88772_CHIPCODE: c_uint = 0x00;
pub const AX_AX88772A_CHIPCODE: c_uint = 0x10;
pub const AX_AX88772B_CHIPCODE: c_uint = 0x20;
pub const AX_HOST_EN: c_uint = 0x01;
pub const AX_PHYSEL_PSEL: c_uint = 0x01;
pub const AX_PHYSEL_SSMII: c_int = 0;
pub const AX_PHYSEL_SSEN: c_uint = 0x10;

pub const AX_PHY_SELECT_INTERNAL: c_int = 0;

pub const AX_MONITOR_MODE: c_uint = 0x01;
pub const AX_MONITOR_LINK: c_uint = 0x02;
pub const AX_MONITOR_MAGIC: c_uint = 0x04;
pub const AX_MONITOR_HSFS: c_uint = 0x10;
// AX88172 Medium Status Register values
pub const AX88172_MEDIUM_FD: c_uint = 0x02;
pub const AX88172_MEDIUM_TX: c_uint = 0x04;
pub const AX88172_MEDIUM_FC: c_uint = 0x10;

pub const AX_MCAST_FILTER_SIZE: c_int = 8;
pub const AX_MAX_MCAST: c_int = 64;
pub const AX_SWRESET_CLEAR: c_uint = 0x00;
pub const AX_SWRESET_RR: c_uint = 0x01;
pub const AX_SWRESET_RT: c_uint = 0x02;
pub const AX_SWRESET_PRTE: c_uint = 0x04;
pub const AX_SWRESET_PRL: c_uint = 0x08;
pub const AX_SWRESET_BZ: c_uint = 0x10;
pub const AX_SWRESET_IPRL: c_uint = 0x20;
pub const AX_SWRESET_IPPD: c_uint = 0x40;
pub const AX88772_IPG0_DEFAULT: c_uint = 0x15;
pub const AX88772_IPG1_DEFAULT: c_uint = 0x0c;
pub const AX88772_IPG2_DEFAULT: c_uint = 0x12;
// AX88772 & AX88178 Medium Mode Register
pub const AX_MEDIUM_PF: c_uint = 0x0080;
pub const AX_MEDIUM_JFE: c_uint = 0x0040;
pub const AX_MEDIUM_TFC: c_uint = 0x0020;
pub const AX_MEDIUM_RFC: c_uint = 0x0010;
pub const AX_MEDIUM_ENCK: c_uint = 0x0008;
pub const AX_MEDIUM_AC: c_uint = 0x0004;
pub const AX_MEDIUM_FD: c_uint = 0x0002;
pub const AX_MEDIUM_GM: c_uint = 0x0001;
pub const AX_MEDIUM_SM: c_uint = 0x1000;
pub const AX_MEDIUM_SBP: c_uint = 0x0800;
pub const AX_MEDIUM_PS: c_uint = 0x0200;
pub const AX_MEDIUM_RE: c_uint = 0x0100;

// AX88772 & AX88178 RX_CTL values
pub const AX_RX_CTL_SO: c_uint = 0x0080;
pub const AX_RX_CTL_AP: c_uint = 0x0020;
pub const AX_RX_CTL_AM: c_uint = 0x0010;
pub const AX_RX_CTL_AB: c_uint = 0x0008;
pub const AX_RX_CTL_SEP: c_uint = 0x0004;
pub const AX_RX_CTL_AMALL: c_uint = 0x0002;
pub const AX_RX_CTL_PRO: c_uint = 0x0001;
pub const AX_RX_CTL_MFB_2048: c_uint = 0x0000;
pub const AX_RX_CTL_MFB_4096: c_uint = 0x0100;
pub const AX_RX_CTL_MFB_8192: c_uint = 0x0200;
pub const AX_RX_CTL_MFB_16384: c_uint = 0x0300;

// GPIO 0 .. 2 toggles
pub const AX_GPIO_GPO0EN: c_uint = 0x01	/* GPIO0 Output enable */;
pub const AX_GPIO_GPO_0: c_uint = 0x02	/* GPIO0 Output value */;
pub const AX_GPIO_GPO1EN: c_uint = 0x04	/* GPIO1 Output enable */;
pub const AX_GPIO_GPO_1: c_uint = 0x08	/* GPIO1 Output value */;
pub const AX_GPIO_GPO2EN: c_uint = 0x10	/* GPIO2 Output enable */;
pub const AX_GPIO_GPO_2: c_uint = 0x20	/* GPIO2 Output value */;
pub const AX_GPIO_RESERVED: c_uint = 0x40	/* Reserved */;
pub const AX_GPIO_RSE: c_uint = 0x80	/* Reload serial EEPROM */;
pub const AX_EEPROM_MAGIC: c_uint = 0xdeadbeef;
pub const AX_EEPROM_LEN: c_uint = 0x200;
pub const AX_EMBD_PHY_ADDR: c_uint = 0x10;
// This structure cannot exceed sizeof(unsigned long [5]) AKA 20 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asix_data {
    pub multi_filter: [u8; AX_MCAST_FILTER_SIZE],
    pub mac_addr: [u8; ETH_ALEN],
    pub phymode: u8,
    pub ledmode: u8,
    pub res: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asix_rx_fixup_info {
    pub ax_skb: *mut sk_buff,
    pub header: u32,
    pub remaining: u16,
    pub split_head: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asix_common_private {
    pub dev): *mut *mut void (resume)(struct usbnet,
    pub dev): *mut *mut void (suspend)(struct usbnet,
    pub in_pm): *mut *mut *mut int (reset)(struct usbnet dev, int,
    pub presvd_phy_advertise: u16,
    pub presvd_phy_bmcr: u16,
    pub rx_fixup_info: asix_rx_fixup_info,
    pub mdio: *mut mii_bus,
    pub phydev: *mut phy_device,
    pub phydev_int: *mut phy_device,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub phy_addr: u16,
    pub embd_phy: bool,
    pub chipcode: u8,
}

// ASIX specific flags

extern "C" {
    pub fn asix_rx_fixup_common(dev: *mut usbnet, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn asix_rx_fixup_common_free(dp: *mut asix_common_private);
}
extern "C" {
    pub fn asix_read_phy_addr(dev: *mut usbnet, internal: bool) -> c_int;
}
extern "C" {
    pub fn asix_sw_reset(dev: *mut usbnet, flags: u8, in_pm: c_int) -> c_int;
}
extern "C" {
    pub fn asix_read_rx_ctl(dev: *mut usbnet, in_pm: c_int) -> u16;
}
extern "C" {
    pub fn asix_write_rx_ctl(dev: *mut usbnet, mode: u16, in_pm: c_int) -> c_int;
}
extern "C" {
    pub fn asix_read_medium_status(dev: *mut usbnet, in_pm: c_int) -> u16;
}
extern "C" {
    pub fn asix_write_medium_mode(dev: *mut usbnet, mode: u16, in_pm: c_int) -> c_int;
}
extern "C" {
    pub fn asix_write_gpio(dev: *mut usbnet, value: u16, sleep: c_int, in_pm: c_int) -> c_int;
}
extern "C" {
    pub fn asix_set_multicast(net: *mut net_device);
}
extern "C" {
    pub fn asix_mdio_read(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int;
}
extern "C" {
    pub fn asix_mdio_write(netdev: *mut net_device, phy_id: c_int, loc: c_int, val: c_int);
}
extern "C" {
    pub fn asix_mdio_bus_read(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int;
}
extern "C" {
    pub fn asix_mdio_bus_write(bus: *mut mii_bus, phy_id: c_int, regnum: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn asix_mdio_read_nopm(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int;
}
extern "C" {
    pub fn asix_get_wol(net: *mut net_device, wolinfo: *mut ethtool_wolinfo);
}
extern "C" {
    pub fn asix_set_wol(net: *mut net_device, wolinfo: *mut ethtool_wolinfo) -> c_int;
}
extern "C" {
    pub fn asix_get_eeprom_len(net: *mut net_device) -> c_int;
}
extern "C" {
    pub fn asix_set_mac_address(net: *mut net_device, p: *mut c_void) -> c_int;
}
