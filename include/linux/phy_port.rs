//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy_port.h
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
// enum phy_port_parent - The device this port is attached to
//
// @PHY_PORT_PHY: Indicates that the port is driven by a PHY device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_port_parent {
    PHY_PORT_PHY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_port_ops {
// Sometimes, the link state can be retrieved from physical,
// out-of-band channels such as the LOS signal on SFP. These
// callbacks allows notifying the port about state changes
//
    pub port): *mut *mut void (link_up)(struct phy_port,
    pub port): *mut *mut void (link_down)(struct phy_port,
// If the port acts as a Media Independent Interface (Serdes port),
// configures the port with the relevant state and mode. When enable is
// not set, interface should be ignored
//
    pub interface): *mut *mut *mut int (configure_mii)(struct phy_port port, bool enable, phy_interface_t,
}

//
// struct phy_port - A representation of a network device physical interface
//
// @head: Used by the port's parent to list ports
// @parent_type: The type of device this port is directly connected to
// @phy: If the parent is PHY_PORT_PHYDEV, the PHY controlling that port
// @ops: Callback ops implemented by the port controller
// @pairs: The number of  pairs this port has, 0 if not applicable
// @mediums: Bitmask of the physical mediums this port provides access to
// @supported: The link modes this port can expose, if this port is MDI (not MII)
// @interfaces: The MII interfaces this port supports, if this port is MII
// @not_described: Indicates to the parent driver if this port isn't described,
// so it's up to the parent to filter its capabilities.
// @active: Indicates if the port is currently part of the active link.
// @is_mii: Indicates if this port is MII (Media Independent Interface),
// or MDI (Media Dependent Interface).
// @is_sfp: Indicates if this port drives an SFP cage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_port {
    pub head: list_head,
    pub parent_type: phy_port_parent,
    pub phy: *mut phy_device,
}

extern "C" {
    pub fn phy_port_destroy(port: *mut phy_port);
}
extern "C" {
    pub fn phy_port_update_supported(port: *mut phy_port);
}
extern "C" {
    pub fn phy_port_restrict_mediums(port: *mut phy_port, mediums: c_ulong) -> c_int;
}
extern "C" {
    pub fn phy_port_get_type(port: *mut phy_port) -> c_int;
}
