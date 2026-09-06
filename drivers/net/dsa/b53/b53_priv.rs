//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/b53/b53_priv.h
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


//
// B53 common definitions
//
// Copyright (C) 2011-2013 Jonas Gorski <jogo@openwrt.org>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_io_ops {
    pub value): *mut *mut *mut int (read8)(struct b53_device dev, u8 page, u8 reg, u8,
    pub value): *mut *mut *mut int (read16)(struct b53_device dev, u8 page, u8 reg, u16,
    pub value): *mut *mut *mut int (read32)(struct b53_device dev, u8 page, u8 reg, u32,
    pub value): *mut *mut *mut int (read48)(struct b53_device dev, u8 page, u8 reg, u64,
    pub value): *mut *mut *mut int (read64)(struct b53_device dev, u8 page, u8 reg, u64,
    pub value): *mut *mut *mut int (write8)(struct b53_device dev, u8 page, u8 reg, u8,
    pub value): *mut *mut *mut int (write16)(struct b53_device dev, u8 page, u8 reg, u16,
    pub value): *mut *mut *mut int (write32)(struct b53_device dev, u8 page, u8 reg, u32,
    pub value): *mut *mut *mut int (write48)(struct b53_device dev, u8 page, u8 reg, u64,
    pub value): *mut *mut *mut int (write64)(struct b53_device dev, u8 page, u8 reg, u64,
    pub value): *mut *mut *mut int (phy_read16)(struct b53_device dev, int addr, int reg, u16,
    pub value): *mut *mut *mut int (phy_write16)(struct b53_device dev, int addr, int reg, u16,
    pub port): *mut *mut *mut int (irq_enable)(struct b53_device dev, int,
    pub port): *mut *mut *mut void (irq_disable)(struct b53_device dev, int,
    pub port): *mut *mut *mut void (phy_enable)(struct b53_device dev, int,
    pub port): *mut *mut *mut void (phy_disable)(struct b53_device dev, int,
    pub config): *mut phylink_config,
    pub interface): phy_interface_t,
    pub port): *mut *mut *mut u8 (serdes_map_lane)(struct b53_device dev, int,
    pub link_up): bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_arl_ops {
    pub idx): *mut *mut b53_arl_entry ent, u8,
    pub idx): *const *const b53_arl_entry ent, u8,
    pub ent): *mut b53_arl_entry,
}

pub const B53_INVALID_LANE: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b53_variant_id {
    B53_VARIANT_NONE = 0,
    B53_VARIANT_5325E,
    B53_VARIANT_5325M,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_pcs {
    pub pcs: phylink_pcs,
    pub dev: *mut b53_device,
    pub lane: u8,
}

pub const B53_N_PORTS: c_int = 9;
pub const B53_N_PORTS_25: c_int = 6;
pub const B53_N_PCS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_port {
    pub vlan_ctl_mask: u16,
    pub pvid: u16,
    pub eee: ethtool_keee,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_vlan {
    pub members: u16,
    pub untag: u16,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_device {
    pub ds: *mut dsa_switch,
    pub pdata: *mut b53_platform_data,
    pub name: *const c_char,
    pub reg_mutex: mutex,
    pub stats_mutex: mutex,
    pub arl_mutex: mutex,
    pub ops: *const b53_io_ops,
    pub arl_ops: *const b53_arl_ops,
// chip specific data
    pub chip_id: u32,
    pub variant_id: b53_variant_id,
    pub core_rev: u8,
    pub vta_regs: [u8; 3],
    pub duplex_reg: u8,
    pub jumbo_pm_reg: u8,
    pub jumbo_size_reg: u8,
    pub reset_gpio: *mut gpio_desc,
    pub num_arl_bins: u8,
    pub num_arl_buckets: u16,
    pub tag_protocol: dsa_tag_protocol,
// used ports mask
    pub enabled_ports: u16,
    pub imp_port: c_uint,
// connect specific data
    pub current_page: u8,
    pub dev: *mut device,
    pub serdes_lane: u8,
// Master MDIO bus we got probed from
    pub bus: *mut mii_bus,
    pub priv: *mut c_void,
// run time configuration
    pub enable_jumbo: bool,
    pub num_vlans: c_uint,
    pub vlans: *mut b53_vlan,
    pub vlan_enabled: bool,
    pub vlan_filtering: bool,
    pub num_ports: c_uint,
    pub ports: *mut b53_port,
    pub pcs: [b53_pcs; B53_N_PCS],
}

pub const B53_63XX_RGMII0: c_int = 4;
pub const B53_CPU_PORT_25: c_int = 5;
pub const B53_CPU_PORT: c_int = 8;
extern "C" {
    pub fn b53_switch_detect(dev: *mut b53_device) -> c_int;
}
extern "C" {
    pub fn b53_switch_register(dev: *mut b53_device) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b53_arl_entry {
    pub port: u16,
    pub mac: [u8; ETH_ALEN],
    pub vid: u16,
    pub is_valid:1: u8,
    pub is_age:1: u8,
    pub is_static:1: u8,
}

// mac_vid = ether_addr_to_u64(ent->mac);
// mac_vid |= (u64)(ent->vid & ARLTBL_VID_MASK) << ARLTBL_VID_S;
// fwd_entry = ent->port & ARLTBL_DATA_PORT_ID_MASK;
// fwd_entry |= ARLTBL_VALID;
// fwd_entry |= ARLTBL_STATIC;
// fwd_entry |= ARLTBL_AGE;
// mac_vid = ether_addr_to_u64(ent->mac);
// mac_vid |= (u64)B53_CPU_PORT << ARLTBL_DATA_PORT_ID_S_25;
// mac_vid |= ((u64)ent->port << ARLTBL_DATA_PORT_ID_S_25) &
// mac_vid |= ARLTBL_VALID_25;
// mac_vid |= ARLTBL_STATIC_25;
// mac_vid |= ARLTBL_AGE_25;
// vid_entry = ent->vid;
// mac_vid = ether_addr_to_u64(ent->mac);
// mac_vid |= (u64)(ent->vid & ARLTBL_VID_MASK) << ARLTBL_VID_S;
// fwd_entry = ent->port & ARLTBL_DATA_PORT_ID_MASK_89;
// fwd_entry |= ARLTBL_VALID_89;
// fwd_entry |= ARLTBL_STATIC_89;
// fwd_entry |= ARLTBL_AGE_89;

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: ret) -> return;
}
extern "C" {
    pub fn gpio_to_desc(_arg: gpio) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

// Exported functions towards other drivers
extern "C" {
    pub fn b53_imp_vlan_setup(ds: *mut dsa_switch, cpu_port: c_int);
}
extern "C" {
    pub fn b53_configure_vlan(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn b53_get_ethtool_stats(ds: *mut dsa_switch, port: c_int, data: *mut u64);
}
extern "C" {
    pub fn b53_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
extern "C" {
    pub fn b53_get_ethtool_phy_stats(ds: *mut dsa_switch, port: c_int, data: *mut u64);
}
extern "C" {
    pub fn b53_set_ageing_time(ds: *mut dsa_switch, msecs: c_uint) -> c_int;
}
extern "C" {
    pub fn b53_br_leave(ds: *mut dsa_switch, port: c_int, bridge: dsa_bridge);
}
extern "C" {
    pub fn b53_br_set_stp_state(ds: *mut dsa_switch, port: c_int, state: u8);
}
extern "C" {
    pub fn b53_br_fast_age(ds: *mut dsa_switch, port: c_int);
}
extern "C" {
    pub fn b53_setup_devlink_resources(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn b53_port_event(ds: *mut dsa_switch, port: c_int);
}
extern "C" {
    pub fn b53_setup_port(ds: *mut dsa_switch, port: c_int) -> c_int;
}
extern "C" {
    pub fn b53_enable_port(ds: *mut dsa_switch, port: c_int, phy: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn b53_disable_port(ds: *mut dsa_switch, port: c_int);
}
extern "C" {
    pub fn b53_brcm_hdr_setup(ds: *mut dsa_switch, port: c_int);
}
extern "C" {
    pub fn b53_eee_init(ds: *mut dsa_switch, port: c_int, phy: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn b53_support_eee(ds: *mut dsa_switch, port: c_int) -> bool;
}
extern "C" {
    pub fn b53_set_mac_eee(ds: *mut dsa_switch, port: c_int, e: *mut ethtool_keee) -> c_int;
}
