//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_transport_sas.h
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

extern "C" {
    pub fn scsi_is_sas_rphy(: *const device) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_linkrate {
// These Values are defined in the SAS standard
    SAS_LINK_RATE_UNKNOWN = 0,
    SAS_PHY_DISABLED = 1,
    SAS_PHY_RESET_PROBLEM = 2,
    SAS_SATA_SPINUP_HOLD = 3,
    SAS_SATA_PORT_SELECTOR = 4,
    SAS_PHY_RESET_IN_PROGRESS = 5,
    SAS_LINK_RATE_1_5_GBPS = 8,
    SAS_LINK_RATE_G1 = SAS_LINK_RATE_1_5_GBPS,
    SAS_LINK_RATE_3_0_GBPS = 9,
    SAS_LINK_RATE_G2 = SAS_LINK_RATE_3_0_GBPS,
    SAS_LINK_RATE_6_0_GBPS = 10,
    SAS_LINK_RATE_12_0_GBPS = 11,
    SAS_LINK_RATE_22_5_GBPS = 12,
// These are virtual to the transport class and may never
// be signalled normally since the standard defined field
// is only 4 bits
    SAS_LINK_RATE_FAILED = 0x10,
    SAS_PHY_VIRTUAL = 0x11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_identify {
    pub device_type: sas_device_type,
    pub initiator_port_protocols: sas_protocol,
    pub target_port_protocols: sas_protocol,
    pub sas_address: u64,
    pub phy_identifier: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_phy {
    pub dev: device,
    pub number: c_int,
    pub enabled: c_int,
// phy identification
    pub identify: sas_identify,
// phy attributes
    pub negotiated_linkrate: sas_linkrate,
    pub minimum_linkrate_hw: sas_linkrate,
    pub minimum_linkrate: sas_linkrate,
    pub maximum_linkrate_hw: sas_linkrate,
    pub maximum_linkrate: sas_linkrate,
// link error statistics
    pub invalid_dword_count: u32,
    pub running_disparity_error_count: u32,
    pub loss_of_dword_sync_count: u32,
    pub phy_reset_problem_count: u32,
// for the list of phys belonging to a port
    pub port_siblings: list_head,
// available to the lldd
    pub hostdata: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_rphy {
    pub dev: device,
    pub identify: sas_identify,
    pub list: list_head,
    pub q: *mut request_queue,
    pub scsi_target_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_end_device {
    pub rphy: sas_rphy,
// flags
    pub ready_led_meaning:1: unsigned,
    pub tlr_supported:1: unsigned,
    pub tlr_enabled:1: unsigned,
// parameters
    pub I_T_nexus_loss_timeout: u16,
    pub initiator_response_timeout: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_expander_device {
    pub level: c_int,
    pub next_port_id: c_int,
pub const SAS_EXPANDER_VENDOR_ID_LEN: c_int = 8;
    pub vendor_id: [c_char; SAS_EXPANDER_VENDOR_ID_LEN+1],
pub const SAS_EXPANDER_PRODUCT_ID_LEN: c_int = 16;
    pub product_id: [c_char; SAS_EXPANDER_PRODUCT_ID_LEN+1],
pub const SAS_EXPANDER_PRODUCT_REV_LEN: c_int = 4;
    pub product_rev: [c_char; SAS_EXPANDER_PRODUCT_REV_LEN+1],
pub const SAS_EXPANDER_COMPONENT_VENDOR_ID_LEN: c_int = 8;
    pub component_vendor_id: [c_char; SAS_EXPANDER_COMPONENT_VENDOR_ID_LEN+1],
    pub component_id: u16,
    pub component_revision_id: u8,
    pub rphy: sas_rphy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_port {
    pub dev: device,
    pub port_identifier: c_int,
    pub num_phys: c_int,
// port flags
    pub is_backlink:1: c_uint,
// the other end of the link
    pub rphy: *mut sas_rphy,
    pub phy_list_mutex: mutex,
    pub phy_list: list_head,
    pub /: *mut *mut list_head del_list; / libsas only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_phy_linkrates {
    pub maximum_linkrate: sas_linkrate,
    pub minimum_linkrate: sas_linkrate,
}

// The functions by which the transport class and the driver communicate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_function_template {
    pub ): *mut *mut int (get_linkerrors)(struct sas_phy,
    pub ): *mut *mut *mut int (get_enclosure_identifier)(struct sas_rphy , u64,
    pub ): *mut *mut int (get_bay_identifier)(struct sas_rphy,
    pub int): *mut *mut *mut int (phy_reset)(struct sas_phy ,,
    pub int): *mut *mut *mut int (phy_enable)(struct sas_phy ,,
    pub ): *mut *mut int (phy_setup)(struct sas_phy,
    pub ): *mut *mut void (phy_release)(struct sas_phy,
    pub ): *mut *mut *mut int (set_phy_speed)(struct sas_phy , struct sas_phy_linkrates,
    pub ): *mut sas_rphy,
}

extern "C" {
    pub fn sas_remove_children(: *mut device);
}
extern "C" {
    pub fn sas_remove_host(: *mut Scsi_Host);
}
extern "C" {
    pub fn sas_phy_free(: *mut sas_phy);
}
extern "C" {
    pub fn sas_phy_add(: *mut sas_phy) -> c_int;
}
extern "C" {
    pub fn sas_phy_delete(: *mut sas_phy);
}
extern "C" {
    pub fn scsi_is_sas_phy(: *const device) -> c_int;
}
extern "C" {
    pub fn sas_get_address(: *mut scsi_device) -> u64;
}
extern "C" {
    pub fn sas_tlr_supported(: *mut scsi_device) -> c_uint;
}
extern "C" {
    pub fn sas_is_tlr_enabled(: *mut scsi_device) -> c_uint;
}
extern "C" {
    pub fn sas_disable_tlr(: *mut scsi_device);
}
extern "C" {
    pub fn sas_enable_tlr(: *mut scsi_device);
}
extern "C" {
    pub fn sas_ata_ncq_prio_supported(sdev: *mut scsi_device) -> bool;
}
extern "C" {
    pub fn sas_rphy_free(: *mut sas_rphy);
}
extern "C" {
    pub fn sas_rphy_add(: *mut sas_rphy) -> c_int;
}
extern "C" {
    pub fn sas_rphy_remove(: *mut sas_rphy);
}
extern "C" {
    pub fn sas_rphy_delete(: *mut sas_rphy);
}
extern "C" {
    pub fn sas_rphy_unlink(: *mut sas_rphy);
}
extern "C" {
    pub fn sas_port_add(: *mut sas_port) -> c_int;
}
extern "C" {
    pub fn sas_port_free(: *mut sas_port);
}
extern "C" {
    pub fn sas_port_delete(: *mut sas_port);
}
extern "C" {
    pub fn sas_port_add_phy(: *mut sas_port, : *mut sas_phy);
}
extern "C" {
    pub fn sas_port_delete_phy(: *mut sas_port, : *mut sas_phy);
}
extern "C" {
    pub fn sas_port_mark_backlink(: *mut sas_port);
}
extern "C" {
    pub fn scsi_is_sas_port(: *const device) -> c_int;
}
extern "C" {
    pub fn sas_release_transport(: *mut scsi_transport_template);
}
extern "C" {
    pub fn sas_read_port_mode_page(: *mut scsi_device) -> c_int;
}

