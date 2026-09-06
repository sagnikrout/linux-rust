//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/vitesse-vsc73xx.h
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

// The VSC7395 switch chips have 5+1 ports which means 5 ordinary ports and
// a sixth CPU port facing the processor with an RGMII interface. These ports
// are numbered 0..4 and 6, so they leave a "hole" in the port map for port 5,
// which is invalid.
//
// The VSC7398 has 8 ports, port 7 is again the CPU port.
//
// We allocate 8 ports and avoid access to the nonexistent ports.
//
pub const VSC73XX_MAX_NUM_PORTS: c_int = 8;
//
// struct vsc73xx_portinfo - port data structure: contains storage data
// @pvid_vlan_filtering: pvid vlan number used in vlan filtering mode
// @pvid_tag_8021q: pvid vlan number used in tag_8021q mode
// @pvid_vlan_filtering_configured: informs if port has configured pvid in vlan
// filtering mode
// @pvid_tag_8021q_configured: imforms if port have configured pvid in tag_8021q
// mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_portinfo {
    pub pvid_vlan_filtering: u16,
    pub pvid_tag_8021q: u16,
    pub pvid_vlan_filtering_configured: bool,
    pub pvid_tag_8021q_configured: bool,
}

//
// struct vsc73xx - VSC73xx state container: main data structure
// @dev: The device pointer
// @reset: The descriptor for the GPIO line tied to the reset pin
// @ds: Pointer to the DSA core structure
// @gc: Main structure of the GPIO controller
// @chipid: Storage for the Chip ID value read from the CHIPID register of the
// switch
// @addr: MAC address used in flow control frames
// @ops: Structure with hardware-dependent operations
// @priv: Pointer to the configuration interface structure
// @portinfo: Storage table portinfo structructures
// @vlans: List of configured vlans. Contains port mask and untagged status of
// every vlan configured in port vlan operation. It doesn't cover tag_8021q
// vlans.
// @fdb_lock: Mutex protects fdb access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx {
    pub dev: *mut device,
    pub reset: *mut gpio_desc,
    pub ds: *mut dsa_switch,
    pub gc: gpio_chip,
    pub chipid: u16,
    pub addr: [u8; ETH_ALEN],
    pub ops: *const vsc73xx_ops,
    pub priv: *mut c_void,
    pub portinfo: [vsc73xx_portinfo; VSC73XX_MAX_NUM_PORTS],
    pub vlans: list_head,
    pub fdb_lock: mutex,
}

//
// struct vsc73xx_ops - VSC73xx methods container
// @read: Method for register reading over the hardware-dependent interface
// @write: Method for register writing over the hardware-dependent interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_ops {
    pub val): *mut u32,
    pub val): u32,
}

//
// struct vsc73xx_bridge_vlan - VSC73xx driver structure which keeps vlan
// database copy
// @vid: VLAN number
// @portmask: each bit represents one port
// @untagged: each bit represents one port configured with @vid untagged
// @list: list structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc73xx_bridge_vlan {
    pub vid: u16,
    pub portmask: u8,
    pub untagged: u8,
    pub list: list_head,
}

extern "C" {
    pub fn vsc73xx_is_addr_valid(block: u8, subblock: u8) -> c_int;
}
extern "C" {
    pub fn vsc73xx_probe(vsc: *mut vsc73xx) -> c_int;
}
extern "C" {
    pub fn vsc73xx_remove(vsc: *mut vsc73xx);
}
extern "C" {
    pub fn vsc73xx_shutdown(vsc: *mut vsc73xx);
}
