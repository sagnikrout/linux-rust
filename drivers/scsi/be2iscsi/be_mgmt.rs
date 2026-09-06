//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/be2iscsi/be_mgmt.h
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
// Copyright 2017 Broadcom. All Rights Reserved.
// The term "Broadcom" refers to Broadcom Limited and/or its subsidiaries.
//
// Contact Information:
// linux-drivers@broadcom.com
//

pub const IP_ACTION_ADD: c_uint = 0x01;
pub const IP_ACTION_DEL: c_uint = 0x02;
pub const IP_V6_LEN: c_int = 16;
pub const IP_V4_LEN: c_int = 4;
// UE Status and Mask register
pub const PCICFG_UE_STATUS_LOW: c_uint = 0xA0;
pub const PCICFG_UE_STATUS_HIGH: c_uint = 0xA4;
pub const PCICFG_UE_STATUS_MASK_LOW: c_uint = 0xA8;
pub const PCICFG_UE_STATUS_MASK_HI: c_uint = 0xAC;
pub const BE_INVLDT_CMD_TBL_SZ: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct invldt_cmd_tbl {
    pub icd: c_ushort,
    pub cid: c_ushort,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct invldt_cmds_params_in {
    pub hdr: be_cmd_req_hdr,
    pub ref_handle: c_uint,
    pub icd_count: c_uint,
    pub table: [invldt_cmd_tbl; BE_INVLDT_CMD_TBL_SZ],
    pub cleanup_type: c_ushort,
    pub unused: c_ushort,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct invldt_cmds_params_out {
    pub hdr: be_cmd_resp_hdr,
    pub ref_handle: c_uint,
    pub icd_count: c_uint,
    pub icd_status: [c_uint; BE_INVLDT_CMD_TBL_SZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union be_invldt_cmds_params {
    pub request: invldt_cmds_params_in,
    pub response: invldt_cmds_params_out,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_hba_attributes {
    pub flashrom_version_string: [u8; BEISCSI_VER_STRLEN],
    pub manufacturer_name: [u8; BEISCSI_VER_STRLEN],
    pub supported_modes: u32,
    pub seeprom_version_lo: u8,
    pub seeprom_version_hi: u8,
    pub rsvd0: [u8; 2],
    pub fw_cmd_data_struct_version: u32,
    pub ep_fw_data_struct_version: u32,
    pub ncsi_version_string: [u8; 12],
    pub default_extended_timeout: u32,
    pub controller_model_number: [u8; BEISCSI_VER_STRLEN],
    pub controller_description: [u8; 64],
    pub controller_serial_number: [u8; BEISCSI_VER_STRLEN],
    pub ip_version_string: [u8; BEISCSI_VER_STRLEN],
    pub firmware_version_string: [u8; BEISCSI_VER_STRLEN],
    pub bios_version_string: [u8; BEISCSI_VER_STRLEN],
    pub redboot_version_string: [u8; BEISCSI_VER_STRLEN],
    pub driver_version_string: [u8; BEISCSI_VER_STRLEN],
    pub fw_on_flash_version_string: [u8; BEISCSI_VER_STRLEN],
    pub functionalities_supported: u32,
    pub max_cdblength: u16,
    pub asic_revision: u8,
    pub generational_guid: [u8; 16],
    pub hba_port_count: u8,
    pub default_link_down_timeout: u16,
    pub iscsi_ver_min_max: u8,
    pub multifunction_device: u8,
    pub cache_valid: u8,
    pub hba_status: u8,
    pub max_domains_supported: u8,
    pub phy_port: u8,
    pub firmware_post_status: u32,
    pub hba_mtu: [u32; 8],
    pub iscsi_features: u8,
    pub asic_generation: u8,
    pub future_u8: [u8; 2],
    pub future_u32: [u32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_controller_attributes {
    pub hba_attribs: mgmt_hba_attributes,
    pub pci_vendor_id: u16,
    pub pci_device_id: u16,
    pub pci_sub_vendor_id: u16,
    pub pci_sub_system_id: u16,
    pub pci_bus_number: u8,
    pub pci_device_number: u8,
    pub pci_function_number: u8,
    pub interface_type: u8,
    pub unique_identifier: u64,
    pub netfilters: u8,
    pub rsvd0: [u8; 3],
    pub future_u32: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mgmt_controller_attributes {
    pub hdr: be_cmd_req_hdr,
    pub params: mgmt_controller_attributes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mgmt_controller_attributes_resp {
    pub hdr: be_cmd_resp_hdr,
    pub params: mgmt_controller_attributes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_bsg_vendor_cmd {
    pub hdr: be_cmd_req_hdr,
    pub region: c_ushort,
    pub offset: c_ushort,
    pub sector: c_ushort,
    pub __packed: },
// configuration management

    pub \: bus_address.u.a32.address_lo;,
    pub \: bus_address.u.a32.address_hi;,
pub const BEISCSI_WRITE_FLASH: c_int = 0;
pub const BEISCSI_READ_FLASH: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_endpoint {
    pub phba: *mut beiscsi_hba,
    pub conn: *mut beiscsi_conn,
    pub openiscsi_ep: *mut iscsi_endpoint,
    pub ip_type: c_ushort,
    pub dst6_addr: [c_char; ISCSI_ADDRESS_BUF_LEN],
    pub dst_addr: c_ulong,
    pub ep_cid: c_ushort,
    pub fw_handle: c_uint,
    pub dst_tcpport: u16,
    pub cid_vld: u16,
}

extern "C" {
    pub fn beiscsi_get_initiator_name(phba: *mut beiscsi_hba, name: *mut c_char, cfg: bool) -> c_int;
}
extern "C" {
    pub fn beiscsi_if_en_dhcp(phba: *mut beiscsi_hba, ip_type: u32) -> c_int;
}
extern "C" {
    pub fn beiscsi_if_set_gw(phba: *mut beiscsi_hba, ip_type: u32, gw: *mut u8) -> c_int;
}
extern "C" {
    pub fn beiscsi_if_get_handle(phba: *mut beiscsi_hba) -> c_uint;
}
extern "C" {
    pub fn beiscsi_if_set_vlan(phba: *mut beiscsi_hba, vlan_tag: u16) -> c_int;
}
extern "C" {
    pub fn beiscsi_boot_logout_sess(phba: *mut beiscsi_hba) -> c_uint;
}
extern "C" {
    pub fn beiscsi_boot_reopen_sess(phba: *mut beiscsi_hba) -> c_uint;
}
extern "C" {
    pub fn beiscsi_boot_get_sinfo(phba: *mut beiscsi_hba) -> c_uint;
}
extern "C" {
    pub fn __beiscsi_boot_get_shandle(phba: *mut beiscsi_hba, async: c_int) -> c_uint;
}
extern "C" {
    pub fn beiscsi_boot_get_shandle(phba: *mut beiscsi_hba, s_handle: *mut c_uint) -> c_int;
}
