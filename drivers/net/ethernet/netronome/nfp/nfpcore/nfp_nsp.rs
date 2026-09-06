//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_nsp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
pub const NSP_NSP_H: c_int = 1;

extern "C" {
    pub fn nfp_nsp_close(state: *mut nfp_nsp);
}
extern "C" {
    pub fn nfp_nsp_get_abi_ver_major(state: *mut nfp_nsp) -> u16;
}
extern "C" {
    pub fn nfp_nsp_get_abi_ver_minor(state: *mut nfp_nsp) -> u16;
}
extern "C" {
    pub fn nfp_nsp_wait(state: *mut nfp_nsp) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_device_soft_reset(state: *mut nfp_nsp) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_load_fw(state: *mut nfp_nsp, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_write_flash(state: *mut nfp_nsp, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_mac_reinit(state: *mut nfp_nsp) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_load_stored_fw(state: *mut nfp_nsp) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_hwinfo_lookup(state: *mut nfp_nsp, buf: *mut c_void, size: c_uint) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_hwinfo_set(state: *mut nfp_nsp, buf: *mut c_void, size: c_uint) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_fw_loaded(state: *mut nfp_nsp) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_eth_interface {
    NFP_INTERFACE_NONE	= 0,
    NFP_INTERFACE_SFP	= 1,
    NFP_INTERFACE_SFPP	= 10,
    NFP_INTERFACE_SFP28	= 28,
    NFP_INTERFACE_QSFP	= 40,
    NFP_INTERFACE_RJ45	= 45,
    NFP_INTERFACE_CXP	= 100,
    NFP_INTERFACE_QSFP28	= 112,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_eth_media {
    NFP_MEDIA_DAC_PASSIVE = 0,
    NFP_MEDIA_DAC_ACTIVE,
    NFP_MEDIA_FIBRE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_eth_aneg {
    NFP_ANEG_AUTO = 0,
    NFP_ANEG_SEARCH,
    NFP_ANEG_25G_CONSORTIUM,
    NFP_ANEG_25G_IEEE,
    NFP_ANEG_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_eth_fec {
    NFP_FEC_AUTO_BIT = 0,
    NFP_FEC_BASER_BIT,
    NFP_FEC_REED_SOLOMON_BIT,
    NFP_FEC_DISABLED_BIT,
}

// link modes about RJ45 haven't been used, so there's no mapping to them
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_ethtool_link_mode_list {
    NFP_MEDIA_W0_RJ45_10M,
    NFP_MEDIA_W0_RJ45_10M_HD,
    NFP_MEDIA_W0_RJ45_100M,
    NFP_MEDIA_W0_RJ45_100M_HD,
    NFP_MEDIA_W0_RJ45_1G,
    NFP_MEDIA_W0_RJ45_2P5G,
    NFP_MEDIA_W0_RJ45_5G,
    NFP_MEDIA_W0_RJ45_10G,
    NFP_MEDIA_1000BASE_CX,
    NFP_MEDIA_1000BASE_KX,
    NFP_MEDIA_10GBASE_KX4,
    NFP_MEDIA_10GBASE_KR,
    NFP_MEDIA_10GBASE_CX4,
    NFP_MEDIA_10GBASE_CR,
    NFP_MEDIA_10GBASE_SR,
    NFP_MEDIA_10GBASE_ER,
    NFP_MEDIA_25GBASE_KR,
    NFP_MEDIA_25GBASE_KR_S,
    NFP_MEDIA_25GBASE_CR,
    NFP_MEDIA_25GBASE_CR_S,
    NFP_MEDIA_25GBASE_SR,
    NFP_MEDIA_40GBASE_CR4,
    NFP_MEDIA_40GBASE_KR4,
    NFP_MEDIA_40GBASE_SR4,
    NFP_MEDIA_40GBASE_LR4,
    NFP_MEDIA_50GBASE_KR,
    NFP_MEDIA_50GBASE_SR,
    NFP_MEDIA_50GBASE_CR,
    NFP_MEDIA_50GBASE_LR,
    NFP_MEDIA_50GBASE_ER,
    NFP_MEDIA_50GBASE_FR,
    NFP_MEDIA_100GBASE_KR4,
    NFP_MEDIA_100GBASE_SR4,
    NFP_MEDIA_100GBASE_CR4,
    NFP_MEDIA_100GBASE_KP4,
    NFP_MEDIA_100GBASE_CR10,
    NFP_MEDIA_10GBASE_LR,
    NFP_MEDIA_25GBASE_LR,
    NFP_MEDIA_25GBASE_ER,
    NFP_MEDIA_LINK_MODES_NUMBER
}

// Defines the valid values of the 'abi_drv_reset' hwinfo key
pub const NFP_NSP_DRV_RESET_DISK: c_int = 0;
pub const NFP_NSP_DRV_RESET_ALWAYS: c_int = 1;
pub const NFP_NSP_DRV_RESET_NEVER: c_int = 2;

// Defines the valid values of the 'app_fw_from_flash' hwinfo key
pub const NFP_NSP_APP_FW_LOAD_DISK: c_int = 0;
pub const NFP_NSP_APP_FW_LOAD_FLASH: c_int = 1;
pub const NFP_NSP_APP_FW_LOAD_PREF: c_int = 2;

// Define the default value for the 'abi_drv_load_ifc' key

//
// struct nfp_eth_table - ETH table information
// @count:	number of table entries
// @max_index:	max of @index fields of all @ports
// @ports:	table of ports
//
// @ports.eth_index:	port index according to legacy ethX numbering
// @ports.index:	chip-wide first channel index
// @ports.nbi:		NBI index
// @ports.base:		first channel index (within NBI)
// @ports.lanes:	number of channels
// @ports.speed:	interface speed (in Mbps)
// @ports.interface:	interface (module) plugged in
// @ports.media:	media type of the @interface
// @ports.fec:		forward error correction mode
// @ports.act_fec:	active forward error correction mode
// @ports.aneg:		auto negotiation mode
// @ports.mac_addr:	interface MAC address
// @ports.label_port:	port id
// @ports.label_subport:  id of interface within port (for split ports)
// @ports.enabled:	is enabled?
// @ports.tx_enabled:	is TX enabled?
// @ports.rx_enabled:	is RX enabled?
// @ports.rx_pause:	Switch of RX pause frame
// @ports.tx_pause:	Switch of Tx pause frame
// @ports.override_changed: is media reconfig pending?
//
// @ports.port_type:	one of %PORT_* defines for ethtool
// @ports.port_lanes:	total number of lanes on the port (sum of lanes of all
// subports)
// @ports.is_split:	is interface part of a split port
// @ports.fec_modes_supported:	bitmap of FEC modes supported
//
// @ports.link_modes_supp:	bitmap of link modes supported
// @ports.link_modes_ad:	bitmap of link modes advertised
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_eth_table {
    pub count: c_uint,
    pub max_index: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_eth_table_port {
    pub eth_index: c_uint,
    pub index: c_uint,
    pub nbi: c_uint,
    pub base: c_uint,
    pub lanes: c_uint,
    pub speed: c_uint,
    pub interface: c_uint,
    pub media: nfp_eth_media,
    pub fec: nfp_eth_fec,
    pub act_fec: nfp_eth_fec,
    pub aneg: nfp_eth_aneg,
    pub mac_addr: [u8; ETH_ALEN],
    pub label_port: u8,
    pub label_subport: u8,
    pub enabled: bool,
    pub tx_enabled: bool,
    pub rx_enabled: bool,
    pub supp_aneg: bool,
    pub rx_pause: bool,
    pub tx_pause: bool,
    pub override_changed: bool,
// Computed fields
    pub port_type: u8,
    pub port_lanes: c_uint,
    pub is_split: bool,
    pub fec_modes_supported: c_uint,
    pub link_modes_supp: [u64; 2],
    pub link_modes_ad: [u64; 2],
    pub __counted_by(count): } ports[],
}

extern "C" {
    pub fn nfp_eth_set_mod_enable(cpp: *mut nfp_cpp, idx: c_uint, enable: bool) -> c_int;
}
extern "C" {
    pub fn nfp_eth_set_idmode(cpp: *mut nfp_cpp, idx: c_uint, state: bool) -> c_int;
}
extern "C" {
    pub fn nfp_eth_config_commit_end(nsp: *mut nfp_nsp) -> c_int;
}
extern "C" {
    pub fn nfp_eth_config_cleanup_end(nsp: *mut nfp_nsp);
}
extern "C" {
    pub fn __nfp_eth_set_aneg(nsp: *mut nfp_nsp, mode: nfp_eth_aneg) -> c_int;
}
extern "C" {
    pub fn __nfp_eth_set_speed(nsp: *mut nfp_nsp, speed: c_uint) -> c_int;
}
extern "C" {
    pub fn __nfp_eth_set_split(nsp: *mut nfp_nsp, lanes: c_uint) -> c_int;
}
//
// struct nfp_nsp_identify - NSP static information
// @version:      opaque version string
// @flags:        version flags
// @br_primary:   branch id of primary bootloader
// @br_secondary: branch id of secondary bootloader
// @br_nsp:       branch id of NSP
// @primary:      version of primarary bootloader
// @secondary:    version id of secondary bootloader
// @nsp:          version id of NSP
// @sensor_mask:  mask of present sensors available on NIC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_nsp_identify {
    pub version: [c_char; 40],
    pub flags: u8,
    pub br_primary: u8,
    pub br_secondary: u8,
    pub br_nsp: u8,
    pub primary: u16,
    pub secondary: u16,
    pub nsp: u16,
    pub sensor_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_nsp_sensor_id {
    NFP_SENSOR_CHIP_TEMPERATURE,
    NFP_SENSOR_ASSEMBLY_POWER,
    NFP_SENSOR_ASSEMBLY_12V_POWER,
    NFP_SENSOR_ASSEMBLY_3V3_POWER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_eth_media_buf {
    pub eth_index: u8,
    pub reserved: [u8; 7],
    pub supported_modes: [__le64; 2],
    pub advertised_modes: [__le64; 2],
}

extern "C" {
    pub fn nfp_nsp_read_media(state: *mut nfp_nsp, buf: *mut c_void, size: c_uint) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_nsp_versions {
    NFP_VERSIONS_BSP,
    NFP_VERSIONS_CPLD,
    NFP_VERSIONS_APP,
    NFP_VERSIONS_BUNDLE,
    NFP_VERSIONS_UNDI,
    NFP_VERSIONS_NCSI,
    NFP_VERSIONS_CFGR,
}

extern "C" {
    pub fn nfp_nsp_versions(state: *mut nfp_nsp, buf: *mut c_void, size: c_uint) -> c_int;
}
