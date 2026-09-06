//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/dbg-tlv.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2018-2025 Intel Corporation
//

// Macro flag: #define __iwl_fw_dbg_tlv_h__

pub const IWL_FW_INI_MAX_REGION_ID: c_int = 64;
pub const IWL_FW_INI_MAX_NAME: c_int = 32;
pub const IWL_FW_INI_MAX_CFG_NAME: c_int = 64;
pub const IWL_FW_INI_DOMAIN_ALWAYS_ON: c_int = 0;

pub const IWL_FW_INI_PRESET_DISABLE: c_uint = 0xff;
//
// struct iwl_fw_ini_hcmd - debug configuration host command
//
// @id: the debug configuration command type for instance: 0xf6 / 0xf5 / DHC
// @group: the desired cmd group
// @reserved: to align to FW struct
// @data: all of the relevant command data to be sent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_hcmd {
    pub id: u8,
    pub group: u8,
    pub reserved: __le16,
    pub data: [u8; ],
    pub /: *mut *mut } __packed; / FW_DEBUG_TLV_HCMD_DATA_API_S_VER_1,
//
// struct iwl_fw_ini_header - Common Header for all ini debug TLV's structures
//
// @version: TLV version
// @domain: domain of the TLV. One of &enum iwl_fw_ini_dbg_domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_header {
    pub version: __le32,
    pub domain: __le32,
// followed by the data
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_HEADER_S_VER_1,
//
// struct iwl_fw_ini_addr_size - Base address and size that defines
// a chunk of memory
//
// @addr: the base address (fixed size - 4 bytes)
// @size: the size to read
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_addr_size {
    pub addr: __le32,
    pub size: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_ADDR_SIZE_VER_1,
//
// struct iwl_fw_ini_region_dev_addr_range - Configuration to read
// device address range
//
// @offset: offset to add to the base address of each chunk
// The addrs[] array will be treated as an array of &iwl_fw_ini_addr_size -
// an array of (addr, size) pairs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_dev_addr_range {
    pub offset: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_DEVICE_ADDR_RANGE_API_S_VER_1,
//
// struct iwl_fw_ini_region_dev_addr - Configuration to read device addresses
//
// @size: size of each memory chunk
// @offset: offset to add to the base address of each chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_dev_addr {
    pub size: __le32,
    pub offset: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_DEVICE_ADDR_API_S_VER_1,
//
// struct iwl_fw_ini_region_fifos - Configuration to read Tx/Rx fifos
//
// @fid: fifos ids array. Used to determine what fifos to collect
// @hdr_only: if non zero, collect only the registers
// @offset: offset to add to the registers addresses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_fifos {
    pub fid: [__le32; 2],
    pub hdr_only: __le32,
    pub offset: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_REGION_FIFOS_API_S_VER_1,
//
// struct iwl_fw_ini_region_err_table - error table region data
//
// Configuration to read Umac/Lmac error table
//
// @version: version of the error table
// @base_addr: base address of the error table
// @size: size of the error table
// @offset: offset to add to &base_addr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_err_table {
    pub version: __le32,
    pub base_addr: __le32,
    pub size: __le32,
    pub offset: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_REGION_ERROR_TABLE_API_S_VER_1,
//
// struct iwl_fw_ini_region_special_device_memory - special device memory
//
// Configuration to read a special memory
//
// @type: type of the special memory
// @version: version of the special memory
// @base_addr: base address of the error table
// @size: size of the error table
// @offset: offset to add to &base_addr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_special_device_memory {
    pub type: __le16,
    pub version: __le16,
    pub base_addr: __le32,
    pub size: __le32,
    pub offset: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_REGION_SPECIAL_DEVICE_ADDR_API_S_VER_1,
//
// struct iwl_fw_ini_region_internal_buffer - internal buffer region data
//
// Configuration to read internal monitor buffer
//
// @alloc_id: allocation id one of &enum iwl_fw_ini_allocation_id
// @base_addr: internal buffer base address
// @size: size internal buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_internal_buffer {
    pub alloc_id: __le32,
    pub base_addr: __le32,
    pub size: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_REGION_INTERNAL_BUFFER_API_S_VER_1,
//
// struct iwl_fw_ini_region_tlv - region TLV
//
// Configures parameters for region data collection
//
// @hdr: debug header
// @id: region id. Max id is %IWL_FW_INI_MAX_REGION_ID
// @type: region type. One of &enum iwl_fw_ini_region_type
// @sub_type: region sub type
// @sub_type_ver: region sub type version
// @reserved: not in use
// @name: region name
// @dev_addr: device address configuration. Used by
// %IWL_FW_INI_REGION_DEVICE_MEMORY, %IWL_FW_INI_REGION_PERIPHERY_MAC,
// %IWL_FW_INI_REGION_PERIPHERY_PHY, %IWL_FW_INI_REGION_PERIPHERY_AUX,
// %IWL_FW_INI_REGION_PAGING, %IWL_FW_INI_REGION_CSR,
// %IWL_FW_INI_REGION_DRAM_IMR and %IWL_FW_INI_REGION_PCI_IOSF_CONFIG
// %IWL_FW_INI_REGION_DBGI_SRAM, %FW_TLV_DEBUG_REGION_TYPE_DBGI_SRAM,
// %IWL_FW_INI_REGION_PERIPHERY_SNPS_DPHYIP,
// @dev_addr_range: device address range configuration. Used by
// %IWL_FW_INI_REGION_PERIPHERY_MAC_RANGE and
// %IWL_FW_INI_REGION_PERIPHERY_PHY_RANGE
// @fifos: fifos configuration. Used by %IWL_FW_INI_REGION_TXF and
// %IWL_FW_INI_REGION_RXF
// @err_table: error table configuration. Used by
// %IWL_FW_INI_REGION_LMAC_ERROR_TABLE and
// %IWL_FW_INI_REGION_UMAC_ERROR_TABLE
// @internal_buffer: internal monitor buffer configuration. Used by
// %IWL_FW_INI_REGION_INTERNAL_BUFFER
// @special_mem: special device memory region, used by
// %IWL_FW_INI_REGION_SPECIAL_DEVICE_MEMORY
// @dram_alloc_id: dram allocation id. One of &enum iwl_fw_ini_allocation_id.
// Used by %IWL_FW_INI_REGION_DRAM_BUFFER
// @tlv_mask: tlv collection mask. Used by %IWL_FW_INI_REGION_TLV
// @addrs: array of addresses attached to the end of the region tlv
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_region_tlv {
    pub hdr: iwl_fw_ini_header,
    pub id: __le32,
    pub type: u8,
    pub sub_type: u8,
    pub sub_type_ver: u8,
    pub reserved: u8,
    pub name: [u8; IWL_FW_INI_MAX_NAME],
    pub dev_addr: iwl_fw_ini_region_dev_addr,
    pub dev_addr_range: iwl_fw_ini_region_dev_addr_range,
    pub fifos: iwl_fw_ini_region_fifos,
    pub err_table: iwl_fw_ini_region_err_table,
    pub internal_buffer: iwl_fw_ini_region_internal_buffer,
    pub special_mem: iwl_fw_ini_region_special_device_memory,
    pub dram_alloc_id: __le32,
    pub tlv_mask: __le32,
}

//
// struct iwl_fw_ini_debug_info_tlv - debug info TLV
//
// debug configuration name for a specific image
//
// @hdr: debug header
// @image_type: image type
// @debug_cfg_name: debug configuration name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_debug_info_tlv {
    pub hdr: iwl_fw_ini_header,
    pub image_type: __le32,
    pub debug_cfg_name: [u8; IWL_FW_INI_MAX_CFG_NAME],
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_INFO_API_S_VER_1,
//
// struct iwl_fw_ini_allocation_tlv - Allocates DRAM buffers
//
// @hdr: debug header
// @alloc_id: allocation id. One of &enum iwl_fw_ini_allocation_id
// @buf_location: buffer location. One of &enum iwl_fw_ini_buffer_location
// @req_size: requested buffer size
// @max_frags_num: maximum number of fragments
// @min_size: minimum buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_allocation_tlv {
    pub hdr: iwl_fw_ini_header,
    pub alloc_id: __le32,
    pub buf_location: __le32,
    pub req_size: __le32,
    pub max_frags_num: __le32,
    pub min_size: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_BUFFER_ALLOCATION_API_S_VER_1,
//
// struct iwl_fw_ini_trigger_tlv - trigger TLV
//
// Trigger that upon firing, determines what regions to collect
//
// @hdr: debug header
// @time_point: time point. One of &enum iwl_fw_ini_time_point
// @trigger_reason: trigger reason
// @apply_policy: uses &enum iwl_fw_ini_trigger_apply_policy
// @dump_delay: delay from trigger fire to dump, in usec
// @occurrences: max trigger fire occurrences allowed
// @reserved: unused
// @ignore_consec: ignore consecutive triggers, in usec
// @reset_fw: if non zero, will reset and reload the FW
// @multi_dut: initiate debug dump data on several DUTs
// @regions_mask: mask of regions to collect
// @data: trigger data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_trigger_tlv {
    pub hdr: iwl_fw_ini_header,
    pub time_point: __le32,
    pub trigger_reason: __le32,
    pub apply_policy: __le32,
    pub dump_delay: __le32,
    pub occurrences: __le32,
    pub reserved: __le32,
    pub ignore_consec: __le32,
    pub reset_fw: __le32,
    pub multi_dut: __le32,
    pub regions_mask: __le64,
    pub data: [__le32; ],
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_TRIGGER_API_S_VER_1,
//
// struct iwl_fw_ini_hcmd_tlv - Generic Host command pass through TLV
//
// @hdr: debug header
// @time_point: time point. One of &enum iwl_fw_ini_time_point
// @period_msec: interval at which the hcmd will be sent to the FW.
// Measured in msec (0 = one time command)
// @hcmd: a variable length host-command to be sent to apply the configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_hcmd_tlv {
    pub hdr: iwl_fw_ini_header,
    pub time_point: __le32,
    pub period_msec: __le32,
    pub hcmd: iwl_fw_ini_hcmd,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_HCMD_API_S_VER_1,
//
// struct iwl_fw_ini_addr_val - Address and value to set it to
//
// @address: the base address
// @value: value to set at address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_addr_val {
    pub address: __le32,
    pub value: __le32,
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_ADDR_VALUE_VER_1,
//
// struct iwl_fw_ini_conf_set_tlv - configuration TLV to set register/memory.
//
// @hdr: debug header
// @time_point: time point to apply config. One of &enum iwl_fw_ini_time_point
// @set_type: write access type preset token for time point.
// one of &enum iwl_fw_ini_config_set_type
// @addr_offset: the offset to add to any item in address[0] field
// @addr_val: address value pair
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_ini_conf_set_tlv {
    pub hdr: iwl_fw_ini_header,
    pub time_point: __le32,
    pub set_type: __le32,
    pub addr_offset: __le32,
    pub addr_val: [iwl_fw_ini_addr_val; ],
    pub /: *mut *mut } __packed; / FW_TLV_DEBUG_CONFIG_SET_API_S_VER_1,
//
// enum iwl_fw_ini_config_set_type - configuration set type
//
// @IWL_FW_INI_CONFIG_SET_TYPE_INVALID: invalid config set
// @IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_PERIPHERY_MAC: for PERIPHERY MAC configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_PERIPHERY_PHY: for PERIPHERY PHY configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_PERIPHERY_AUX: for PERIPHERY AUX configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_MEMORY: for DEVICE MEMORY configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_CSR: for CSR configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_DBGC_DRAM_ADDR: for DBGC_DRAM_ADDR configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_PERIPH_SCRATCH_HWM: for PERIPH SCRATCH HWM configuration
// @IWL_FW_INI_CONFIG_SET_TYPE_MAX_NUM: max number of configuration supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_config_set_type {
    IWL_FW_INI_CONFIG_SET_TYPE_INVALID = 0,
    IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_PERIPHERY_MAC,
    IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_PERIPHERY_PHY,
    IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_PERIPHERY_AUX,
    IWL_FW_INI_CONFIG_SET_TYPE_DEVICE_MEMORY,
    IWL_FW_INI_CONFIG_SET_TYPE_CSR,
    IWL_FW_INI_CONFIG_SET_TYPE_DBGC_DRAM_ADDR,
    IWL_FW_INI_CONFIG_SET_TYPE_PERIPH_SCRATCH_HWM,
    IWL_FW_INI_CONFIG_SET_TYPE_MAX_NUM,
    } __packed;

//
// enum iwl_fw_ini_allocation_id - allocation ID
//
// @IWL_FW_INI_ALLOCATION_INVALID: invalid
// @IWL_FW_INI_ALLOCATION_ID_DBGC1: allocation meant for DBGC1 configuration
// @IWL_FW_INI_ALLOCATION_ID_DBGC2: allocation meant for DBGC2 configuration
// @IWL_FW_INI_ALLOCATION_ID_DBGC3: allocation meant for DBGC3 configuration
// @IWL_FW_INI_ALLOCATION_ID_DBGC4: allocation meant for DBGC4 configuration
// @IWL_FW_INI_ALLOCATION_NUM: number of allocation ids
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_allocation_id {
    IWL_FW_INI_ALLOCATION_INVALID,
    IWL_FW_INI_ALLOCATION_ID_DBGC1,
    IWL_FW_INI_ALLOCATION_ID_DBGC2,
    IWL_FW_INI_ALLOCATION_ID_DBGC3,
    IWL_FW_INI_ALLOCATION_ID_DBGC4,
    IWL_FW_INI_ALLOCATION_NUM,
}

//
// enum iwl_fw_ini_buffer_location - buffer location
//
// @IWL_FW_INI_LOCATION_INVALID: invalid
// @IWL_FW_INI_LOCATION_SRAM_PATH: SRAM location
// @IWL_FW_INI_LOCATION_DRAM_PATH: DRAM location
// @IWL_FW_INI_LOCATION_NPK_PATH: NPK location
// @IWL_FW_INI_LOCATION_NUM: number of valid locations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_buffer_location {
    IWL_FW_INI_LOCATION_INVALID,
    IWL_FW_INI_LOCATION_SRAM_PATH,
    IWL_FW_INI_LOCATION_DRAM_PATH,
    IWL_FW_INI_LOCATION_NPK_PATH,
    IWL_FW_INI_LOCATION_NUM,
}

//
// enum iwl_fw_ini_region_type - region type
//
// @IWL_FW_INI_REGION_INVALID: invalid
// @IWL_FW_INI_REGION_TLV: uCode and debug TLVs
// @IWL_FW_INI_REGION_INTERNAL_BUFFER: monitor SMEM buffer
// @IWL_FW_INI_REGION_DRAM_BUFFER: monitor DRAM buffer
// @IWL_FW_INI_REGION_TXF: TX fifos
// @IWL_FW_INI_REGION_RXF: RX fifo
// @IWL_FW_INI_REGION_LMAC_ERROR_TABLE: lmac error table
// @IWL_FW_INI_REGION_UMAC_ERROR_TABLE: umac error table
// @IWL_FW_INI_REGION_RSP_OR_NOTIF: FW response or notification data
// @IWL_FW_INI_REGION_DEVICE_MEMORY: device internal memory
// @IWL_FW_INI_REGION_PERIPHERY_MAC: periphery registers of MAC
// @IWL_FW_INI_REGION_PERIPHERY_PHY: periphery registers of PHY
// @IWL_FW_INI_REGION_PERIPHERY_AUX: periphery registers of AUX
// @IWL_FW_INI_REGION_PAGING: paging memory
// @IWL_FW_INI_REGION_CSR: CSR registers
// @IWL_FW_INI_REGION_DRAM_IMR: IMR memory
// @IWL_FW_INI_REGION_PCI_IOSF_CONFIG: PCI/IOSF config
// @IWL_FW_INI_REGION_SPECIAL_DEVICE_MEMORY: special device memory
// @IWL_FW_INI_REGION_DBGI_SRAM: periphery registers of DBGI SRAM
// @IWL_FW_INI_REGION_PERIPHERY_MAC_RANGE: a range of periphery registers of MAC
// @IWL_FW_INI_REGION_PERIPHERY_PHY_RANGE: a range of periphery registers of PHY
// @IWL_FW_INI_REGION_PERIPHERY_SNPS_DPHYIP: periphery registers of SNPS DPHYIP
// @IWL_FW_INI_REGION_NUM: number of region types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_region_type {
    IWL_FW_INI_REGION_INVALID,
    IWL_FW_INI_REGION_TLV,
    IWL_FW_INI_REGION_INTERNAL_BUFFER,
    IWL_FW_INI_REGION_DRAM_BUFFER,
    IWL_FW_INI_REGION_TXF,
    IWL_FW_INI_REGION_RXF,
    IWL_FW_INI_REGION_LMAC_ERROR_TABLE,
    IWL_FW_INI_REGION_UMAC_ERROR_TABLE,
    IWL_FW_INI_REGION_RSP_OR_NOTIF,
    IWL_FW_INI_REGION_DEVICE_MEMORY,
    IWL_FW_INI_REGION_PERIPHERY_MAC,
    IWL_FW_INI_REGION_PERIPHERY_PHY,
    IWL_FW_INI_REGION_PERIPHERY_AUX,
    IWL_FW_INI_REGION_PAGING,
    IWL_FW_INI_REGION_CSR,
    IWL_FW_INI_REGION_DRAM_IMR,
    IWL_FW_INI_REGION_PCI_IOSF_CONFIG,
    IWL_FW_INI_REGION_SPECIAL_DEVICE_MEMORY,
    IWL_FW_INI_REGION_DBGI_SRAM,
    IWL_FW_INI_REGION_PERIPHERY_MAC_RANGE,
    IWL_FW_INI_REGION_PERIPHERY_PHY_RANGE,
    IWL_FW_INI_REGION_PERIPHERY_SNPS_DPHYIP,
    IWL_FW_INI_REGION_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_region_device_memory_subtype {
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_HW_SMEM = 1,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_UMAC_ERROR_TABLE = 5,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_LMAC_1_ERROR_TABLE = 7,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_LMAC_2_ERROR_TABLE = 10,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_TCM_1_ERROR_TABLE = 14,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_TCM_2_ERROR_TABLE = 16,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_RCM_1_ERROR_TABLE = 18,
    IWL_FW_INI_REGION_DEVICE_MEMORY_SUBTYPE_RCM_2_ERROR_TABLE = 20,
}

//
// enum iwl_fw_ini_time_point - time point type
//
// Hard coded time points in which the driver can send hcmd or perform dump
// collection
//
// @IWL_FW_INI_TIME_POINT_INVALID: invalid timepoint
// @IWL_FW_INI_TIME_POINT_EARLY: pre loading the FW
// @IWL_FW_INI_TIME_POINT_AFTER_ALIVE: first cmd from host after alive notif
// @IWL_FW_INI_TIME_POINT_POST_INIT: last cmd in series of init sequence
// @IWL_FW_INI_TIME_POINT_FW_ASSERT: FW assert
// @IWL_FW_INI_TIME_POINT_FW_HW_ERROR: FW HW error
// @IWL_FW_INI_TIME_POINT_FW_TFD_Q_HANG: TFD queue hang
// @IWL_FW_INI_TIME_POINT_FW_DHC_NOTIFICATION: DHC cmd response and notif
// @IWL_FW_INI_TIME_POINT_FW_RSP_OR_NOTIF: FW response or notification.
// data field holds id and group
// @IWL_FW_INI_TIME_POINT_USER_TRIGGER: user trigger time point
// @IWL_FW_INI_TIME_POINT_PERIODIC: periodic timepoint that fires in constant
// intervals. data field holds the interval time in msec
// @IWL_FW_INI_TIME_POINT_RESERVED: reserved
// @IWL_FW_INI_TIME_POINT_HOST_ASSERT: Unused
// @IWL_FW_INI_TIME_POINT_HOST_ALIVE_TIMEOUT: alive timeout
// @IWL_FW_INI_TIME_POINT_HOST_DEVICE_ENABLE: device enable
// @IWL_FW_INI_TIME_POINT_HOST_DEVICE_DISABLE: device disable
// @IWL_FW_INI_TIME_POINT_HOST_D3_START: D3 start
// @IWL_FW_INI_TIME_POINT_HOST_D3_END: D3 end
// @IWL_FW_INI_TIME_POINT_MISSED_BEACONS: missed beacons
// @IWL_FW_INI_TIME_POINT_ASSOC_FAILED: association failure
// @IWL_FW_INI_TIME_POINT_TX_FAILED: Tx frame failed
// @IWL_FW_INI_TIME_POINT_TX_WFD_ACTION_FRAME_FAILED: wifi direct action
// frame failed
// @IWL_FW_INI_TIME_POINT_TX_LATENCY_THRESHOLD: Tx latency threshold
// @IWL_FW_INI_TIME_POINT_HANG_OCCURRED: hang occurred
// @IWL_FW_INI_TIME_POINT_EAPOL_FAILED: EAPOL failed
// @IWL_FW_INI_TIME_POINT_FAKE_TX: fake Tx
// @IWL_FW_INI_TIME_POINT_DEASSOC: de association
// @IWL_FW_INI_TIME_POINT_PRESET_OVERRIDE_EXT_REQ: request to override preset
// @IWL_FW_INI_TIME_POINT_PRESET_OVERRIDE_START: start handling override preset
// request
// @IWL_FW_INI_TIME_SCAN_FAILURE: failed scan channel list
// @IWL_FW_INI_TIME_ESR_LINK_UP: EMLSR is active (several links are activated)
// @IWL_FW_INI_TIME_ESR_LINK_DOWN: EMLSR is inactive (only one active link left)
// @IWL_FW_INI_TIME_POINT_NUM: number of time points
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_time_point {
    IWL_FW_INI_TIME_POINT_INVALID,
    IWL_FW_INI_TIME_POINT_EARLY,
    IWL_FW_INI_TIME_POINT_AFTER_ALIVE,
    IWL_FW_INI_TIME_POINT_POST_INIT,
    IWL_FW_INI_TIME_POINT_FW_ASSERT,
    IWL_FW_INI_TIME_POINT_FW_HW_ERROR,
    IWL_FW_INI_TIME_POINT_FW_TFD_Q_HANG,
    IWL_FW_INI_TIME_POINT_FW_DHC_NOTIFICATION,
    IWL_FW_INI_TIME_POINT_FW_RSP_OR_NOTIF,
    IWL_FW_INI_TIME_POINT_USER_TRIGGER,
    IWL_FW_INI_TIME_POINT_PERIODIC,
    IWL_FW_INI_TIME_POINT_RESERVED,
    IWL_FW_INI_TIME_POINT_HOST_ASSERT,
    IWL_FW_INI_TIME_POINT_HOST_ALIVE_TIMEOUT,
    IWL_FW_INI_TIME_POINT_HOST_DEVICE_ENABLE,
    IWL_FW_INI_TIME_POINT_HOST_DEVICE_DISABLE,
    IWL_FW_INI_TIME_POINT_HOST_D3_START,
    IWL_FW_INI_TIME_POINT_HOST_D3_END,
    IWL_FW_INI_TIME_POINT_MISSED_BEACONS,
    IWL_FW_INI_TIME_POINT_ASSOC_FAILED,
    IWL_FW_INI_TIME_POINT_TX_FAILED,
    IWL_FW_INI_TIME_POINT_TX_WFD_ACTION_FRAME_FAILED,
    IWL_FW_INI_TIME_POINT_TX_LATENCY_THRESHOLD,
    IWL_FW_INI_TIME_POINT_HANG_OCCURRED,
    IWL_FW_INI_TIME_POINT_EAPOL_FAILED,
    IWL_FW_INI_TIME_POINT_FAKE_TX,
    IWL_FW_INI_TIME_POINT_DEASSOC,
    IWL_FW_INI_TIME_POINT_PRESET_OVERRIDE_EXT_REQ,
    IWL_FW_INI_TIME_POINT_PRESET_OVERRIDE_START,
    IWL_FW_INI_TIME_SCAN_FAILURE,
    IWL_FW_INI_TIME_ESR_LINK_UP,
    IWL_FW_INI_TIME_ESR_LINK_DOWN,
    IWL_FW_INI_TIME_POINT_NUM,
}

//
// enum iwl_fw_ini_trigger_apply_policy - Determines how to apply triggers
//
// @IWL_FW_INI_APPLY_POLICY_MATCH_TIME_POINT: match by time point
// @IWL_FW_INI_APPLY_POLICY_MATCH_DATA: match by trigger data
// @IWL_FW_INI_APPLY_POLICY_OVERRIDE_REGIONS: override regions mask.
// Append otherwise
// @IWL_FW_INI_APPLY_POLICY_OVERRIDE_CFG: override trigger configuration
// @IWL_FW_INI_APPLY_POLICY_OVERRIDE_DATA: override trigger data.
// Append otherwise
// @IWL_FW_INI_APPLY_POLICY_DUMP_COMPLETE_CMD: send cmd once dump collected
// @IWL_FW_INI_APPLY_POLICY_SPLIT_DUMP_RESET: split this dump into regions
// before and after the reset handshake
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_trigger_apply_policy {
    IWL_FW_INI_APPLY_POLICY_MATCH_TIME_POINT	= BIT(0),
    IWL_FW_INI_APPLY_POLICY_MATCH_DATA		= BIT(1),
    IWL_FW_INI_APPLY_POLICY_OVERRIDE_REGIONS	= BIT(8),
    IWL_FW_INI_APPLY_POLICY_OVERRIDE_CFG		= BIT(9),
    IWL_FW_INI_APPLY_POLICY_OVERRIDE_DATA		= BIT(10),
    IWL_FW_INI_APPLY_POLICY_DUMP_COMPLETE_CMD	= BIT(16),
    IWL_FW_INI_APPLY_POLICY_SPLIT_DUMP_RESET	= BIT(17),
}

//
// enum iwl_fw_ini_trigger_reset_fw_policy - Determines how to handle reset
//
// @IWL_FW_INI_RESET_FW_MODE_NOTHING: do not stop FW and reload (default)
// @IWL_FW_INI_RESET_FW_MODE_STOP_FW_ONLY: stop FW without reload FW
// @IWL_FW_INI_RESET_FW_MODE_STOP_AND_RELOAD_FW: stop FW with reload FW
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_trigger_reset_fw_policy {
    IWL_FW_INI_RESET_FW_MODE_NOTHING = 0,
    IWL_FW_INI_RESET_FW_MODE_STOP_FW_ONLY,
    IWL_FW_INI_RESET_FW_MODE_STOP_AND_RELOAD_FW
}

//
// enum iwl_fw_ini_dump_policy - Determines how to handle dump based on enabled flags
//
// @IWL_FW_INI_DEBUG_DUMP_POLICY_NO_LIMIT: OS has no limit of dump size
// @IWL_FW_INI_DEBUG_DUMP_POLICY_MAX_LIMIT_600KB: mini dump only 600KB region dump
// @IWL_FW_IWL_DEBUG_DUMP_POLICY_MAX_LIMIT_5MB: mini dump 5MB size dump
// @IWL_FW_IWL_DEBUG_DUMP_POLICY_BEFORE_RESET: dump this region before reset
// handshake (if requested by %IWL_FW_INI_APPLY_POLICY_SPLIT_DUMP_RESET)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_dump_policy {
    IWL_FW_INI_DEBUG_DUMP_POLICY_NO_LIMIT           = BIT(0),
    IWL_FW_INI_DEBUG_DUMP_POLICY_MAX_LIMIT_600KB    = BIT(1),
    IWL_FW_IWL_DEBUG_DUMP_POLICY_MAX_LIMIT_5MB      = BIT(2),
    IWL_FW_IWL_DEBUG_DUMP_POLICY_BEFORE_RESET	= BIT(3),
}

//
// enum iwl_fw_ini_dump_type - Determines dump type based on size defined by FW.
//
// @IWL_FW_INI_DUMP_BRIEF : only dump the most important regions
// @IWL_FW_INI_DUMP_MEDIUM: dump more regions than "brief", but not all regions
// @IWL_FW_INI_DUMP_VERBOSE : dump all regions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_ini_dump_type {
    IWL_FW_INI_DUMP_BRIEF,
    IWL_FW_INI_DUMP_MEDIUM,
    IWL_FW_INI_DUMP_VERBOSE,
}
