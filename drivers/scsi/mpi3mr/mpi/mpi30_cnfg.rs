//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi/mpi30_cnfg.h
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
// Copyright 2017-2026 Broadcom Inc. All rights reserved.
//
pub const MPI30_CNFG_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_config_request {
    pub host_tag: __le16,
    pub ioc_use_only02: u8,
    pub function: u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: u8,
    pub msg_flags: u8,
    pub change_count: __le16,
    pub proxy_ioc_number: u8,
    pub reserved0b: u8,
    pub page_version: u8,
    pub page_number: u8,
    pub page_type: u8,
    pub action: u8,
    pub page_address: __le32,
    pub page_length: __le16,
    pub reserved16: __le16,
    pub reserved18: [__le32; 2],
    pub sgl: mpi3_sge_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_config_page_header {
    pub page_version: u8,
    pub reserved01: u8,
    pub page_number: u8,
    pub page_attribute: u8,
    pub page_length: __le16,
    pub page_type: u8,
    pub reserved07: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page0 {
    pub header: mpi3_config_page_header,
    pub chip_revision: [u8; 8],
    pub chip_name: [u8; 32],
    pub board_name: [u8; 32],
    pub board_assembly: [u8; 32],
    pub board_tracer_number: [u8; 32],
    pub board_power: __le32,
    pub reserved94: __le32,
    pub reserved98: __le32,
    pub oem: u8,
    pub profile_identifier: u8,
    pub flags: __le16,
    pub board_mfg_day: u8,
    pub board_mfg_month: u8,
    pub board_mfg_year: __le16,
    pub board_rework_day: u8,
    pub board_rework_month: u8,
    pub board_rework_year: __le16,
    pub board_revision: [u8; 8],
    pub e_pack_fru: [u8; 16],
    pub product_name: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page1 {
    pub header: mpi3_config_page_header,
    pub reserved08: [__le32; 2],
    pub vpd: [u8; MPI3_MAN1_VPD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page2 {
    pub header: mpi3_config_page_header,
    pub flags: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: [__le32; 3],
    pub oem_board_tracer_number: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man5_phy_entry {
    pub ioc_wwid: __le64,
    pub device_name: __le64,
    pub sata_wwid: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page5 {
    pub header: mpi3_config_page_header,
    pub num_phys: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub phy: [mpi3_man5_phy_entry; MPI3_MAN5_PHY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man6_gpio_entry {
    pub function_code: u8,
    pub function_flags: u8,
    pub flags: __le16,
    pub param1: u8,
    pub param2: u8,
    pub reserved06: __le16,
    pub param3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page6 {
    pub header: mpi3_config_page_header,
    pub flags: __le16,
    pub reserved0a: __le16,
    pub num_gpio: u8,
    pub reserved0d: [u8; 3],
    pub gpio: [mpi3_man6_gpio_entry; MPI3_MAN6_GPIO_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man7_receptacle_info {
    pub name: [__le32; 4],
    pub location: u8,
    pub connector_type: u8,
    pub ped_clk: u8,
    pub connector_id: u8,
    pub reserved14: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page7 {
    pub header: mpi3_config_page_header,
    pub flags: __le32,
    pub num_receptacles: u8,
    pub reserved0d: [u8; 3],
    pub enclosure_name: [__le32; 4],
    pub receptacle_info: [mpi3_man7_receptacle_info; MPI3_MAN7_RECEPTACLE_INFO_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man8_phy_info {
    pub receptacle_id: u8,
    pub connector_lane: u8,
    pub reserved02: __le16,
    pub slotx1: __le16,
    pub slotx2: __le16,
    pub slotx4: __le16,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page8 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_phys: u8,
    pub reserved0d: [u8; 3],
    pub phy_info: [mpi3_man8_phy_info; MPI3_MAN8_PHY_INFO_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man9_rsrc_entry {
    pub maximum: __le32,
    pub decrement: __le32,
    pub minimum: __le32,
    pub actual: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpi3_man9_resources {
    MPI3_MAN9_RSRC_OUTSTANDING_REQS    = 0,
    MPI3_MAN9_RSRC_TARGET_CMDS         = 1,
    MPI3_MAN9_RSRC_RESERVED02          = 2,
    MPI3_MAN9_RSRC_NVME                = 3,
    MPI3_MAN9_RSRC_INITIATORS          = 4,
    MPI3_MAN9_RSRC_VDS                 = 5,
    MPI3_MAN9_RSRC_ENCLOSURES          = 6,
    MPI3_MAN9_RSRC_ENCLOSURE_PHYS      = 7,
    MPI3_MAN9_RSRC_EXPANDERS           = 8,
    MPI3_MAN9_RSRC_PCIE_SWITCHES       = 9,
    MPI3_MAN9_RSRC_RESERVED10          = 10,
    MPI3_MAN9_RSRC_HOST_PD_DRIVES      = 11,
    MPI3_MAN9_RSRC_ADV_HOST_PD_DRIVES  = 12,
    MPI3_MAN9_RSRC_RAID_PD_DRIVES      = 13,
    MPI3_MAN9_RSRC_DRV_DIAG_BUF        = 14,
    MPI3_MAN9_RSRC_NAMESPACE_COUNT     = 15,
    MPI3_MAN9_RSRC_NUM_RESOURCES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page9 {
    pub header: mpi3_config_page_header,
    pub num_resources: u8,
    pub reserved09: u8,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
    pub reserved10: __le32,
    pub reserved14: __le32,
    pub reserved18: __le32,
    pub reserved1c: __le32,
    pub resource: [mpi3_man9_rsrc_entry; MPI3_MAN9_RSRC_NUM_RESOURCES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man10_istwi_ctrlr_entry {
    pub target_address: __le16,
    pub flags: __le16,
    pub scl_low_override: u8,
    pub scl_high_override: u8,
    pub reserved06: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page10 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_istwi_ctrl: u8,
    pub reserved0d: [u8; 3],
    pub istwi_controller: [mpi3_man10_istwi_ctrlr_entry; MPI3_MAN10_ISTWI_CTRLR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_mux_device_format {
    pub max_channel: u8,
    pub reserved01: [u8; 3],
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_temp_sensor_device_format {
    pub type: u8,
    pub reserved01: [u8; 3],
    pub temp_channel: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_seeprom_device_format {
    pub size: u8,
    pub page_write_size: u8,
    pub reserved02: __le16,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_ddr_spd_device_format {
    pub channel: u8,
    pub reserved01: [u8; 3],
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_cable_mgmt_device_format {
    pub type: u8,
    pub receptacle_id: u8,
    pub reserved02: __le16,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_bkplane_spec_ubm_format {
    pub flags: __le16,
    pub reserved02: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_bkplane_spec_non_ubm_format {
    pub flags: __le16,
    pub reserved02: u8,
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_man11_bkplane_spec_format {
    pub ubm: mpi3_man11_bkplane_spec_ubm_format,
    pub non_ubm: mpi3_man11_bkplane_spec_non_ubm_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_bkplane_mgmt_device_format {
    pub type: u8,
    pub receptacle_id: u8,
    pub reset_info: u8,
    pub reserved03: u8,
    pub backplane_mgmt_specific: mpi3_man11_bkplane_spec_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_gas_gauge_device_format {
    pub type: u8,
    pub reserved01: [u8; 3],
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_mgmt_ctrlr_device_format {
    pub reserved00: __le32,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_board_fan_device_format {
    pub flags: u8,
    pub reserved01: u8,
    pub min_fan_speed: u8,
    pub max_fan_speed: u8,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_man11_device_specific_format {
    pub mux: mpi3_man11_mux_device_format,
    pub temp_sensor: mpi3_man11_temp_sensor_device_format,
    pub seeprom: mpi3_man11_seeprom_device_format,
    pub ddr_spd: mpi3_man11_ddr_spd_device_format,
    pub cable_mgmt: mpi3_man11_cable_mgmt_device_format,
    pub bkplane_mgmt: mpi3_man11_bkplane_mgmt_device_format,
    pub gas_gauge: mpi3_man11_gas_gauge_device_format,
    pub mgmt_controller: mpi3_man11_mgmt_ctrlr_device_format,
    pub board_fan: mpi3_man11_board_fan_device_format,
    pub words: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man11_istwi_device_format {
    pub device_type: u8,
    pub controller: u8,
    pub reserved02: u8,
    pub flags: u8,
    pub device_address: __le16,
    pub mux_channel: u8,
    pub mux_index: u8,
    pub device_specific: mpi3_man11_device_specific_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page11 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_istwi_dev: u8,
    pub reserved0d: [u8; 3],
    pub istwi_device: [mpi3_man11_istwi_device_format; MPI3_MAN11_ISTWI_DEVICE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man12_sgpio_info {
    pub slot_count: u8,
    pub reserved01: [u8; 3],
    pub reserved04: __le32,
    pub phy_order: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page12 {
    pub header: mpi3_config_page_header,
    pub flags: __le32,
    pub s_clock_freq: __le32,
    pub activity_modulation: __le32,
    pub num_sgpio: u8,
    pub reserved15: [u8; 3],
    pub reserved18: __le32,
    pub reserved1c: __le32,
    pub pattern: [__le32; 8],
    pub sgpio_info: [mpi3_man12_sgpio_info; MPI3_MAN12_NUM_SGPIO_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man13_translation_info {
    pub slot_status: __le32,
    pub mask: __le32,
    pub activity: u8,
    pub locate: u8,
    pub error: u8,
    pub reserved0b: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page13 {
    pub header: mpi3_config_page_header,
    pub num_trans: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub translation: [mpi3_man13_translation_info; MPI3_MAN13_NUM_TRANSLATION_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page14 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_slot_groups: u8,
    pub num_slots: u8,
    pub max_cert_chain_length: __le16,
    pub sealed_slots: __le32,
    pub populated_slots: __le32,
    pub mgmt_pt_updatable_slots: __le32,
}

pub const MPI3_MAN15_VERSION_RECORD_MAX: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man15_version_record {
    pub spdm_version: __le16,
    pub reserved02: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page15 {
    pub header: mpi3_config_page_header,
    pub num_version_records: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub version_record: [mpi3_man15_version_record; MPI3_MAN15_VERSION_RECORD_MAX],
}

pub const MPI3_MAN16_CERT_ALGO_MAX: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man16_certificate_algorithm {
    pub slot_group: u8,
    pub reserved01: [u8; 3],
    pub base_asym_algo: __le32,
    pub base_hash_algo: __le32,
    pub reserved0c: [__le32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page16 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_cert_algos: u8,
    pub reserved0d: [u8; 3],
    pub certificate_algorithm: [mpi3_man16_certificate_algorithm; MPI3_MAN16_CERT_ALGO_MAX],
}

pub const MPI3_MAN17_HASH_ALGORITHM_MAX: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man17_hash_algorithm {
    pub meas_specification: u8,
    pub reserved01: [u8; 3],
    pub measurement_hash_algo: __le32,
    pub reserved08: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page17 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_hash_algos: u8,
    pub reserved0d: [u8; 3],
    pub hash_algorithm: [mpi3_man17_hash_algorithm; MPI3_MAN17_HASH_ALGORITHM_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page20 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub nonpremium_features: __le32,
    pub allowed_personalities: u8,
    pub reserved11: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page21 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_man_page_product_specific {
    pub header: mpi3_config_page_header,
    pub product_specific_info: [__le32; MPI3_MAN_PROD_SPECIFIC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page0 {
    pub header: mpi3_config_page_header,
    pub unique_value: __le64,
    pub nvdata_version_default: __le32,
    pub nvdata_version_persistent: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page1 {
    pub header: mpi3_config_page_header,
    pub flags: __le32,
    pub dmd_io_delay: u8,
    pub dmd_report_pcie: u8,
    pub dmd_report_sata: u8,
    pub dmd_report_sas: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page2 {
    pub header: mpi3_config_page_header,
    pub gpio_count: u8,
    pub reserved09: [u8; 3],
    pub gpio_val: [__le16; MPI3_IO_UNIT2_GPIO_VAL_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit3_sensor {
    pub flags: __le16,
    pub threshold_margin: u8,
    pub reserved03: u8,
    pub threshold: [__le16; 3],
    pub reserved0a: __le16,
    pub reserved0c: __le32,
    pub reserved10: __le32,
    pub reserved14: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page3 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_sensors: u8,
    pub nominal_poll_interval: u8,
    pub warning_poll_interval: u8,
    pub reserved0f: u8,
    pub sensor: [mpi3_io_unit3_sensor; MPI3_IO_UNIT3_SENSOR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit4_sensor {
    pub current_temperature: __le16,
    pub reserved02: __le16,
    pub flags: u8,
    pub reserved05: [u8; 3],
    pub istwi_index: __le16,
    pub channel: u8,
    pub reserved0b: u8,
    pub reserved0c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page4 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_sensors: u8,
    pub reserved0d: [u8; 3],
    pub sensor: [mpi3_io_unit4_sensor; MPI3_IO_UNIT4_SENSOR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit5_spinup_group {
    pub max_target_spinup: u8,
    pub spinup_delay: u8,
    pub spinup_flags: u8,
    pub reserved03: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page5 {
    pub header: mpi3_config_page_header,
    pub spinup_group_parameters: [mpi3_io_unit5_spinup_group; 4],
    pub reserved18: __le32,
    pub reserved1c: __le32,
    pub device_shutdown: __le16,
    pub reserved22: __le16,
    pub pcie_device_wait_time: u8,
    pub sata_device_wait_time: u8,
    pub spinup_encl_drive_count: u8,
    pub spinup_encl_delay: u8,
    pub num_phys: u8,
    pub pe_initial_spinup_delay: u8,
    pub topology_stable_time: u8,
    pub flags: u8,
    pub phy: [u8; MPI3_IO_UNIT5_PHY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page6 {
    pub header: mpi3_config_page_header,
    pub board_power_requirement: __le32,
    pub pci_slot_power_allocation: __le32,
    pub flags: u8,
    pub reserved11: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_iounit8_digest {
    pub dword: [__le32; 16],
    pub word: [__le16; 32],
    pub byte: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page8 {
    pub header: mpi3_config_page_header,
    pub sb_mode: u8,
    pub sb_state: u8,
    pub reserved0a: __le16,
    pub num_slots: u8,
    pub slots_available: u8,
    pub current_key_encryption_algo: u8,
    pub key_digest_hash_algo: u8,
    pub current_svn: mpi3_version_union,
    pub pending_svn_time: __le16,
    pub reserved16: __le16,
    pub current_key: [__le32; 128],
    pub digest: [mpi3_iounit8_digest; MPI3_IOUNIT8_DIGEST_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page9 {
    pub header: mpi3_config_page_header,
    pub flags: __le32,
    pub first_device: __le16,
    pub reserved0e: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page10 {
    pub header: mpi3_config_page_header,
    pub flags: u8,
    pub reserved09: [u8; 3],
    pub silicon_id: __le32,
    pub fw_version_minor: u8,
    pub fw_version_major: u8,
    pub hw_version_minor: u8,
    pub hw_version_major: u8,
    pub part_number: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit11_profile {
    pub profile_identifier: u8,
    pub reserved01: [u8; 3],
    pub max_vds: __le16,
    pub max_host_pds: __le16,
    pub max_adv_host_pds: __le16,
    pub max_raid_pds: __le16,
    pub max_nvme: __le16,
    pub max_outstanding_requests: __le16,
    pub subsystem_id: __le16,
    pub reserved12: __le16,
    pub reserved14: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page11 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_profiles: u8,
    pub current_profile_identifier: u8,
    pub reserved0e: __le16,
    pub profile: [mpi3_iounit11_profile; MPI3_IOUNIT11_PROFILE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit12_bucket {
    pub coalescing_depth: u8,
    pub coalescing_timeout: u8,
    pub io_count_low_boundary: __le16,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page12 {
    pub header: mpi3_config_page_header,
    pub flags: __le32,
    pub reserved0c: [__le32; 4],
    pub num_buckets: u8,
    pub reserved1d: [u8; 3],
    pub bucket: [mpi3_iounit12_bucket; MPI3_IOUNIT12_BUCKET_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit13_allowed_function {
    pub sub_function: __le16,
    pub function_code: u8,
    pub function_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page13 {
    pub header: mpi3_config_page_header,
    pub flags: __le16,
    pub reserved0a: __le16,
    pub num_allowed_functions: u8,
    pub reserved0d: [u8; 3],
    pub allowed_function: [mpi3_iounit13_allowed_function; MPI3_IOUNIT13_FUNC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit14_pagemetadata {
    pub page_type: u8,
    pub page_number: u8,
    pub reserved02: u8,
    pub page_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page14 {
    pub header: mpi3_config_page_header,
    pub flags: u8,
    pub reserved09: [u8; 3],
    pub num_pages: u8,
    pub reserved0d: [u8; 3],
    pub page_metadata: [mpi3_iounit14_pagemetadata; MPI3_IOUNIT14_MD_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page15 {
    pub header: mpi3_config_page_header,
    pub flags: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub power_budgeting_capability: u8,
    pub reserved11: [u8; 3],
    pub num_power_budget_data: u8,
    pub reserved15: [u8; 3],
    pub power_budget_data: [__le32; MPI3_IOUNIT15_PBD_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page17 {
    pub header: mpi3_config_page_header,
    pub num_instances: u8,
    pub instance: u8,
    pub reserved0a: __le16,
    pub reserved0c: [__le32; 4],
    pub key_length: __le16,
    pub encryption_algorithm: u8,
    pub reserved1f: u8,
    pub current_key: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page18 {
    pub header: mpi3_config_page_header,
    pub flags: u8,
    pub poll_interval: u8,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_iounit19_device {
    pub temperature: __le16,
    pub dev_handle: __le16,
    pub persistent_id: __le16,
    pub reserved06: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_io_unit_page19 {
    pub header: mpi3_config_page_header,
    pub num_devices: __le16,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
    pub device: [mpi3_iounit19_device; MPI3_IOUNIT19_DEVICE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ioc_page0 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub vendor_id: __le16,
    pub device_id: __le16,
    pub revision_id: u8,
    pub reserved11: [u8; 3],
    pub class_code: __le32,
    pub subsystem_vendor_id: __le16,
    pub subsystem_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ioc_page1 {
    pub header: mpi3_config_page_header,
    pub coalescing_timeout: __le32,
    pub coalescing_depth: u8,
    pub obsolete: u8,
    pub reserved0e: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ioc_page2 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub sas_broadcast_primitive_masks: __le16,
    pub sas_notify_primitive_masks: __le16,
    pub event_masks: [__le32; MPI3_IOC2_EVENTMASK_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_allowed_cmd_scsi {
    pub service_action: __le16,
    pub operation_code: u8,
    pub command_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_allowed_cmd_ata {
    pub subcommand: u8,
    pub reserved01: u8,
    pub command: u8,
    pub command_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_allowed_cmd_nvme {
    pub reserved00: u8,
    pub nvme_cmd_flags: u8,
    pub op_code: u8,
    pub command_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_allowed_cmd {
    pub scsi: mpi3_allowed_cmd_scsi,
    pub ata: mpi3_allowed_cmd_ata,
    pub nvme: mpi3_allowed_cmd_nvme,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_page0 {
    pub header: mpi3_config_page_header,
    pub bsd_options: __le32,
    pub ssu_timeout: u8,
    pub io_timeout: u8,
    pub tur_retries: u8,
    pub tur_interval: u8,
    pub reserved10: u8,
    pub security_key_timeout: u8,
    pub first_device: __le16,
    pub reserved14: __le32,
    pub reserved18: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_page1 {
    pub header: mpi3_config_page_header,
    pub flags: __le32,
    pub time_stamp_update: u8,
    pub reserved0d: [u8; 3],
    pub host_diag_trace_max_size: __le16,
    pub host_diag_trace_min_size: __le16,
    pub host_diag_trace_decrement_size: __le16,
    pub reserved16: __le16,
    pub host_diag_fw_max_size: __le16,
    pub host_diag_fw_min_size: __le16,
    pub host_diag_fw_decrement_size: __le16,
    pub reserved1e: __le16,
    pub host_diag_driver_max_size: __le16,
    pub host_diag_driver_min_size: __le16,
    pub host_diag_driver_decrement_size: __le16,
    pub reserved26: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver2_trigger_event {
    pub type: u8,
    pub flags: u8,
    pub reserved02: u8,
    pub event: u8,
    pub reserved04: [__le32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver2_trigger_scsi_sense {
    pub type: u8,
    pub flags: u8,
    pub reserved02: __le16,
    pub ascq: u8,
    pub asc: u8,
    pub sense_key: u8,
    pub reserved07: u8,
    pub reserved08: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver2_trigger_reply {
    pub type: u8,
    pub flags: u8,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub ioc_log_info_mask: __le32,
    pub reserved0c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_driver2_trigger_element {
    pub event: mpi3_driver2_trigger_event,
    pub scsi_sense: mpi3_driver2_trigger_scsi_sense,
    pub reply: mpi3_driver2_trigger_reply,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_page2 {
    pub header: mpi3_config_page_header,
    pub global_trigger: __le64,
    pub reserved10: [__le32; 3],
    pub num_triggers: u8,
    pub reserved1d: [u8; 3],
    pub trigger: [mpi3_driver2_trigger_element; MPI3_DRIVER2_TRIGGER_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_page10 {
    pub header: mpi3_config_page_header,
    pub flags: __le16,
    pub reserved0a: __le16,
    pub num_allowed_commands: u8,
    pub reserved0d: [u8; 3],
    pub allowed_command: [mpi3_allowed_cmd; MPI3_ALLOWED_CMDS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_page20 {
    pub header: mpi3_config_page_header,
    pub flags: __le16,
    pub reserved0a: __le16,
    pub num_allowed_commands: u8,
    pub reserved0d: [u8; 3],
    pub allowed_command: [mpi3_allowed_cmd; MPI3_ALLOWED_CMDS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_page30 {
    pub header: mpi3_config_page_header,
    pub flags: __le16,
    pub reserved0a: __le16,
    pub num_allowed_commands: u8,
    pub reserved0d: [u8; 3],
    pub allowed_command: [mpi3_allowed_cmd; MPI3_ALLOWED_CMDS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_security_mac {
    pub dword: [__le32; 16],
    pub word: [__le16; 32],
    pub byte: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_security_nonce {
    pub dword: [__le32; 16],
    pub word: [__le16; 32],
    pub byte: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_security_root_digest {
    pub dword: [__le32; 16],
    pub word: [__le16; 32],
    pub byte: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_security0_cert_chain {
    pub dword: [__le32; 1024],
    pub word: [__le16; 2048],
    pub byte: [u8; 4096],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page0 {
    pub header: mpi3_config_page_header,
    pub slot_num_group: u8,
    pub slot_num: u8,
    pub cert_chain_length: __le16,
    pub cert_chain_flags: u8,
    pub reserved0d: [u8; 3],
    pub base_asym_algo: __le32,
    pub base_hash_algo: __le32,
    pub reserved18: [__le32; 4],
    pub mac: mpi3_security_mac,
    pub nonce: mpi3_security_nonce,
    pub certificate_chain: mpi3_security0_cert_chain,
}

pub const MPI3_SECURITY1_KEY_RECORD_MAX: c_int = 1;

pub const MPI3_SECURITY1_PAD_MAX: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_security1_key_data {
    pub dword: [__le32; 128],
    pub word: [__le16; 256],
    pub byte: [u8; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security1_key_record {
    pub flags: u8,
    pub consumer: u8,
    pub key_data_size: __le16,
    pub additional_key_data: __le32,
    pub library_version: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub key_data: mpi3_security1_key_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page1 {
    pub header: mpi3_config_page_header,
    pub reserved08: [__le32; 2],
    pub mac: mpi3_security_mac,
    pub nonce: mpi3_security_nonce,
    pub num_keys: u8,
    pub reserved91: [u8; 3],
    pub reserved94: [__le32; 3],
    pub key_record: [mpi3_security1_key_record; MPI3_SECURITY1_KEY_RECORD_MAX],
    pub pad: [u8; MPI3_SECURITY1_PAD_MAX],
}

pub const MPI3_SECURITY2_TRUSTED_ROOT_MAX: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security2_trusted_root {
    pub level: u8,
    pub hash_algorithm: u8,
    pub trusted_root_flags: __le16,
    pub reserved04: [__le32; 3],
    pub root_digest: mpi3_security_root_digest,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page2 {
    pub header: mpi3_config_page_header,
    pub reserved08: [__le32; 2],
    pub mac: mpi3_security_mac,
    pub nonce: mpi3_security_nonce,
    pub reserved90: [__le32; 3],
    pub num_roots: u8,
    pub reserved9d: [u8; 3],
    pub trusted_root: [mpi3_security2_trusted_root; MPI3_SECURITY2_TRUSTED_ROOT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page3 {
    pub header: mpi3_config_page_header,
    pub key_data_length: __le16,
    pub reserved0a: __le16,
    pub key_number: u8,
    pub reserved0d: [u8; 3],
    pub mac: mpi3_security_mac,
    pub nonce: mpi3_security_nonce,
    pub reserved90: [__le32; 12],
    pub flags: u8,
    pub consumer: u8,
    pub key_data_size: __le16,
    pub additional_key_data: __le32,
    pub library_version: u8,
    pub reserved_c9: [u8; 3],
    pub reserved_cc: __le32,
    pub key_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page10 {
    pub header: mpi3_config_page_header,
    pub reserved08: [__le32; 2],
    pub mac: mpi3_security_mac,
    pub nonce: mpi3_security_nonce,
    pub current_token_nonce: __le64,
    pub previous_token_nonce: __le64,
    pub reserved_a0: [__le32; 8],
    pub diagnostic_auth_id: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page11 {
    pub header: mpi3_config_page_header,
    pub flags: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub diagnostic_token_length: __le32,
    pub reserved14: [__le32; 3],
    pub diagnostic_token: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security12_diag_feature {
    pub feature_identifier: __le32,
    pub feature_size: u8,
    pub feature_type: u8,
    pub reserved06: __le16,
    pub status: u8,
    pub section: u8,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
    pub feature_data: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_security_page12 {
    pub header: mpi3_config_page_header,
    pub reserved08: [__le32; 2],
    pub num_diag_features: u8,
    pub reserved11: [u8; 3],
    pub reserved14: [__le32; 3],
    pub diag_feature: [mpi3_security12_diag_feature; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit0_phy_data {
    pub io_unit_port: u8,
    pub port_flags: u8,
    pub phy_flags: u8,
    pub negotiated_link_rate: u8,
    pub controller_phy_device_info: __le16,
    pub reserved06: __le16,
    pub attached_dev_handle: __le16,
    pub controller_dev_handle: __le16,
    pub discovery_status: __le32,
    pub reserved10: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit_page0 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_phys: u8,
    pub init_status: u8,
    pub reserved0e: __le16,
    pub phy_data: [mpi3_sas_io_unit0_phy_data; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit1_phy_data {
    pub io_unit_port: u8,
    pub port_flags: u8,
    pub phy_flags: u8,
    pub max_min_link_rate: u8,
    pub controller_phy_device_info: __le16,
    pub max_target_port_connect_time: __le16,
    pub reserved08: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit_page1 {
    pub header: mpi3_config_page_header,
    pub control_flags: __le16,
    pub sas_narrow_max_queue_depth: __le16,
    pub additional_control_flags: __le16,
    pub sas_wide_max_queue_depth: __le16,
    pub num_phys: u8,
    pub sata_max_q_depth: u8,
    pub reserved12: __le16,
    pub phy_data: [mpi3_sas_io_unit1_phy_data; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit2_phy_pm_settings {
    pub control_flags: u8,
    pub reserved01: u8,
    pub inactivity_timer_exponent: __le16,
    pub sata_partial_timeout: u8,
    pub reserved05: u8,
    pub sata_slumber_timeout: u8,
    pub reserved07: u8,
    pub sas_partial_timeout: u8,
    pub reserved09: u8,
    pub sas_slumber_timeout: u8,
    pub reserved0b: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit_page2 {
    pub header: mpi3_config_page_header,
    pub num_phys: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub sas_phy_power_management_settings: [mpi3_sas_io_unit2_phy_pm_settings; MPI3_SAS_IO_UNIT2_PHY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_io_unit_page3 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub power_management_capabilities: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_expander_page0 {
    pub header: mpi3_config_page_header,
    pub io_unit_port: u8,
    pub report_gen_length: u8,
    pub enclosure_handle: __le16,
    pub reserved0c: __le32,
    pub sas_address: __le64,
    pub discovery_status: __le32,
    pub dev_handle: __le16,
    pub parent_dev_handle: __le16,
    pub expander_change_count: __le16,
    pub expander_route_indexes: __le16,
    pub num_phys: u8,
    pub sas_level: u8,
    pub flags: __le16,
    pub stp_bus_inactivity_time_limit: __le16,
    pub stp_max_connect_time_limit: __le16,
    pub stp_smp_nexus_loss_time: __le16,
    pub max_num_routed_sas_addresses: __le16,
    pub active_zone_manager_sas_address: __le64,
    pub zone_lock_inactivity_limit: __le16,
    pub reserved3a: __le16,
    pub time_to_reduced_func: u8,
    pub initial_time_to_reduced_func: u8,
    pub max_reduced_func_time: u8,
    pub exp_status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_expander_page1 {
    pub header: mpi3_config_page_header,
    pub io_unit_port: u8,
    pub reserved09: [u8; 3],
    pub num_phys: u8,
    pub phy: u8,
    pub num_table_entries_programmed: __le16,
    pub programmed_link_rate: u8,
    pub hw_link_rate: u8,
    pub attached_dev_handle: __le16,
    pub phy_info: __le32,
    pub attached_device_info: __le16,
    pub reserved1a: __le16,
    pub expander_dev_handle: __le16,
    pub change_count: u8,
    pub negotiated_link_rate: u8,
    pub phy_identifier: u8,
    pub attached_phy_identifier: u8,
    pub reserved22: u8,
    pub discovery_info: u8,
    pub attached_phy_info: __le32,
    pub zone_group: u8,
    pub self_config_status: u8,
    pub reserved2a: __le16,
    pub slot: __le16,
    pub slot_index: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sasexpander2_phy_element {
    pub link_change_count: u8,
    pub reserved01: u8,
    pub rate_change_count: __le16,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_expander_page2 {
    pub header: mpi3_config_page_header,
    pub num_phys: u8,
    pub reserved09: u8,
    pub dev_handle: __le16,
    pub reserved0c: __le32,
    pub phy: [mpi3_sasexpander2_phy_element; MPI3_SASEXPANDER2_MAX_NUM_PHYS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_port_page0 {
    pub header: mpi3_config_page_header,
    pub port_number: u8,
    pub reserved09: u8,
    pub port_width: u8,
    pub reserved0b: u8,
    pub zone_group: u8,
    pub reserved0d: [u8; 3],
    pub sas_address: __le64,
    pub device_info: __le16,
    pub reserved1a: __le16,
    pub reserved1c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy_page0 {
    pub header: mpi3_config_page_header,
    pub owner_dev_handle: __le16,
    pub reserved0a: __le16,
    pub attached_dev_handle: __le16,
    pub attached_phy_identifier: u8,
    pub reserved0f: u8,
    pub attached_phy_info: __le32,
    pub programmed_link_rate: u8,
    pub hw_link_rate: u8,
    pub change_count: u8,
    pub flags: u8,
    pub phy_info: __le32,
    pub negotiated_link_rate: u8,
    pub reserved1d: [u8; 3],
    pub slot: __le16,
    pub slot_index: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy_page1 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub invalid_dword_count: __le32,
    pub running_disparity_error_count: __le32,
    pub loss_dword_synch_count: __le32,
    pub phy_reset_problem_count: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy2_phy_event {
    pub phy_event_code: u8,
    pub reserved01: [u8; 3],
    pub phy_event_info: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy_page2 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_phy_events: u8,
    pub reserved0d: [u8; 3],
    pub phy_event: [mpi3_sas_phy2_phy_event; MPI3_SAS_PHY2_PHY_EVENT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy3_phy_event_config {
    pub phy_event_code: u8,
    pub reserved01: [u8; 3],
    pub counter_type: u8,
    pub threshold_window: u8,
    pub time_units: u8,
    pub reserved07: u8,
    pub event_threshold: __le32,
    pub threshold_flags: __le16,
    pub reserved0e: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy_page3 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_phy_events: u8,
    pub reserved0d: [u8; 3],
    pub phy_event_config: [mpi3_sas_phy3_phy_event_config; MPI3_SAS_PHY3_PHY_EVENT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sas_phy_page4 {
    pub header: mpi3_config_page_header,
    pub reserved08: [u8; 3],
    pub flags: u8,
    pub initial_frame: [u8; 28],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit0_phy_data {
    pub link: u8,
    pub link_flags: u8,
    pub phy_flags: u8,
    pub negotiated_link_rate: u8,
    pub attached_dev_handle: __le16,
    pub controller_dev_handle: __le16,
    pub enumeration_status: __le32,
    pub io_unit_port: u8,
    pub reserved0d: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit_page0 {
    pub header: mpi3_config_page_header,
    pub reserved08: __le32,
    pub num_phys: u8,
    pub init_status: u8,
    pub aspm: u8,
    pub reserved0f: u8,
    pub phy_data: [mpi3_pcie_io_unit0_phy_data; MPI3_PCIE_IO_UNIT0_PHY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit1_phy_data {
    pub link: u8,
    pub link_flags: u8,
    pub phy_flags: u8,
    pub max_min_link_rate: u8,
    pub reserved04: __le32,
    pub reserved08: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit_page1 {
    pub header: mpi3_config_page_header,
    pub control_flags: __le32,
    pub reserved0c: __le32,
    pub num_phys: u8,
    pub reserved11: u8,
    pub aspm: u8,
    pub reserved13: u8,
    pub phy_data: [mpi3_pcie_io_unit1_phy_data; MPI3_PCIE_IO_UNIT1_PHY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit_page2 {
    pub header: mpi3_config_page_header,
    pub nvme_max_q_dx1: __le16,
    pub nvme_max_q_dx2: __le16,
    pub nvme_abort_to: u8,
    pub reserved0d: u8,
    pub nvme_max_q_dx4: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit3_error {
    pub threshold_count: __le16,
    pub reserved02: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_io_unit_page3 {
    pub header: mpi3_config_page_header,
    pub threshold_window: u8,
    pub threshold_action: u8,
    pub escalation_count: u8,
    pub escalation_action: u8,
    pub num_errors: u8,
    pub reserved0d: [u8; 3],
    pub error: [mpi3_pcie_io_unit3_error; MPI3_PCIEIOUNIT3_NUM_ERROR_INDEX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_switch_page0 {
    pub header: mpi3_config_page_header,
    pub io_unit_port: u8,
    pub switch_status: u8,
    pub reserved0a: [u8; 2],
    pub dev_handle: __le16,
    pub parent_dev_handle: __le16,
    pub num_ports: u8,
    pub pcie_level: u8,
    pub reserved12: __le16,
    pub reserved14: __le32,
    pub reserved18: __le32,
    pub reserved1c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_switch_page1 {
    pub header: mpi3_config_page_header,
    pub io_unit_port: u8,
    pub flags: u8,
    pub reserved0a: __le16,
    pub num_ports: u8,
    pub port_num: u8,
    pub attached_dev_handle: __le16,
    pub switch_dev_handle: __le16,
    pub negotiated_port_width: u8,
    pub negotiated_link_rate: u8,
    pub slot: __le16,
    pub slot_index: __le16,
    pub reserved18: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcieswitch2_port_element {
    pub link_change_count: __le16,
    pub rate_change_count: __le16,
    pub reserved04: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_switch_page2 {
    pub header: mpi3_config_page_header,
    pub num_ports: u8,
    pub reserved09: u8,
    pub dev_handle: __le16,
    pub reserved0c: __le32,
    pub port: [mpi3_pcieswitch2_port_element; MPI3_PCIESWITCH2_MAX_NUM_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_pcie_link_page0 {
    pub header: mpi3_config_page_header,
    pub link: u8,
    pub reserved09: [u8; 3],
    pub reserved0c: __le32,
    pub receiver_error_count: __le32,
    pub recovery_count: __le32,
    pub corr_error_msg_count: __le32,
    pub non_fatal_error_msg_count: __le32,
    pub fatal_error_msg_count: __le32,
    pub non_fatal_error_count: __le32,
    pub fatal_error_count: __le32,
    pub bad_dllp_count: __le32,
    pub bad_tlp_count: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_enclosure_page0 {
    pub header: mpi3_config_page_header,
    pub enclosure_logical_id: __le64,
    pub flags: __le16,
    pub enclosure_handle: __le16,
    pub num_slots: __le16,
    pub reserved16: __le16,
    pub io_unit_port: u8,
    pub enclosure_level: u8,
    pub sep_dev_handle: __le16,
    pub chassis_slot: u8,
    pub reserved1d: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device0_sas_sata_format {
    pub sas_address: __le64,
    pub flags: __le16,
    pub device_info: __le16,
    pub phy_num: u8,
    pub attached_phy_identifier: u8,
    pub max_port_connections: u8,
    pub zone_group: u8,
    pub reserved10: [u8; 3],
    pub negotiated_link_rate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device0_pcie_format {
    pub supported_link_rates: u8,
    pub max_port_width: u8,
    pub negotiated_port_width: u8,
    pub negotiated_link_rate: u8,
    pub port_num: u8,
    pub controller_reset_to: u8,
    pub device_info: __le16,
    pub maximum_data_transfer_size: __le32,
    pub capabilities: __le32,
    pub noiob: __le16,
    pub nvme_abort_to: u8,
    pub page_size: u8,
    pub shutdown_latency: __le16,
    pub recovery_info: u8,
    pub reserved17: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device0_vd_format {
    pub vd_state: u8,
    pub raid_level: u8,
    pub device_info: __le16,
    pub flags: __le16,
    pub io_throttle_group: __le16,
    pub io_throttle_group_low: __le16,
    pub io_throttle_group_high: __le16,
    pub vd_abort_to: u8,
    pub vd_reset_to: u8,
    pub reserved0e: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_device0_dev_spec_format {
    pub sas_sata_format: mpi3_device0_sas_sata_format,
    pub pcie_format: mpi3_device0_pcie_format,
    pub vd_format: mpi3_device0_vd_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device_page0 {
    pub header: mpi3_config_page_header,
    pub dev_handle: __le16,
    pub parent_dev_handle: __le16,
    pub slot: __le16,
    pub enclosure_handle: __le16,
    pub wwid: __le64,
    pub persistent_id: __le16,
    pub io_unit_port: u8,
    pub access_status: u8,
    pub flags: __le16,
    pub reserved1e: __le16,
    pub slot_index: __le16,
    pub queue_depth: __le16,
    pub reserved24: [u8; 3],
    pub device_form: u8,
    pub device_specific: mpi3_device0_dev_spec_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device1_sas_sata_format {
    pub reserved00: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device1_pcie_format {
    pub vendor_id: __le16,
    pub device_id: __le16,
    pub subsystem_vendor_id: __le16,
    pub subsystem_id: __le16,
    pub reserved08: __le32,
    pub revision_id: u8,
    pub reserved0d: u8,
    pub pci_parameters: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device1_vd_format {
    pub reserved00: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_device1_dev_spec_format {
    pub sas_sata_format: mpi3_device1_sas_sata_format,
    pub pcie_format: mpi3_device1_pcie_format,
    pub vd_format: mpi3_device1_vd_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_device_page1 {
    pub header: mpi3_config_page_header,
    pub dev_handle: __le16,
    pub reserved0a: __le16,
    pub link_change_count: __le16,
    pub rate_change_count: __le16,
    pub tm_count: __le16,
    pub reserved12: __le16,
    pub reserved14: [__le32; 10],
    pub reserved3c: [u8; 3],
    pub device_form: u8,
    pub device_specific: mpi3_device1_dev_spec_format,
}

