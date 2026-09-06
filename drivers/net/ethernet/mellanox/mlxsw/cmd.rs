//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/cmd.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2015-2018 Mellanox Technologies. All rights reserved

pub const MLXSW_CMD_MBOX_SIZE: c_int = 4096;
extern "C" {
    pub fn kzalloc(_arg: MLXSW_CMD_MBOX_SIZE, _arg: GFP_KERNEL) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_opcode {
    MLXSW_CMD_OPCODE_QUERY_FW		= 0x004,
    MLXSW_CMD_OPCODE_QUERY_BOARDINFO	= 0x006,
    MLXSW_CMD_OPCODE_QUERY_AQ_CAP		= 0x003,
    MLXSW_CMD_OPCODE_MAP_FA			= 0xFFF,
    MLXSW_CMD_OPCODE_UNMAP_FA		= 0xFFE,
    MLXSW_CMD_OPCODE_CONFIG_PROFILE		= 0x100,
    MLXSW_CMD_OPCODE_ACCESS_REG		= 0x040,
    MLXSW_CMD_OPCODE_SW2HW_DQ		= 0x201,
    MLXSW_CMD_OPCODE_HW2SW_DQ		= 0x202,
    MLXSW_CMD_OPCODE_2ERR_DQ		= 0x01E,
    MLXSW_CMD_OPCODE_QUERY_DQ		= 0x022,
    MLXSW_CMD_OPCODE_SW2HW_CQ		= 0x016,
    MLXSW_CMD_OPCODE_HW2SW_CQ		= 0x017,
    MLXSW_CMD_OPCODE_QUERY_CQ		= 0x018,
    MLXSW_CMD_OPCODE_SW2HW_EQ		= 0x013,
    MLXSW_CMD_OPCODE_HW2SW_EQ		= 0x014,
    MLXSW_CMD_OPCODE_QUERY_EQ		= 0x015,
    MLXSW_CMD_OPCODE_QUERY_RESOURCES	= 0x101,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_status {
// Command execution succeeded.
    MLXSW_CMD_STATUS_OK		= 0x00,
// Internal error (e.g. bus error) occurred while processing command.
    MLXSW_CMD_STATUS_INTERNAL_ERR	= 0x01,
// Operation/command not supported or opcode modifier not supported.
    MLXSW_CMD_STATUS_BAD_OP		= 0x02,
// Parameter not supported, parameter out of range.
    MLXSW_CMD_STATUS_BAD_PARAM	= 0x03,
// System was not enabled or bad system state.
    MLXSW_CMD_STATUS_BAD_SYS_STATE	= 0x04,
// Attempt to access reserved or unallocated resource, or resource in
// inappropriate ownership.
//
    MLXSW_CMD_STATUS_BAD_RESOURCE	= 0x05,
// Requested resource is currently executing a command.
    MLXSW_CMD_STATUS_RESOURCE_BUSY	= 0x06,
// Required capability exceeds device limits.
    MLXSW_CMD_STATUS_EXCEED_LIM	= 0x08,
// Resource is not in the appropriate state or ownership.
    MLXSW_CMD_STATUS_BAD_RES_STATE	= 0x09,
// Index out of range (might be beyond table size or attempt to
// access a reserved resource).
//
    MLXSW_CMD_STATUS_BAD_INDEX	= 0x0A,
// NVMEM checksum/CRC failed.
    MLXSW_CMD_STATUS_BAD_NVMEM	= 0x0B,
// Device is currently running reset
    MLXSW_CMD_STATUS_RUNNING_RESET	= 0x26,
// Bad management packet (silently discarded).
    MLXSW_CMD_STATUS_BAD_PKT	= 0x30,
}

// QUERY_FW - Query Firmware
// -------------------------
// OpMod == 0, INMmod == 0
// -----------------------
// The QUERY_FW command retrieves information related to firmware, command
// interface version and the amount of resources that should be allocated to
// the firmware.
//
// cmd_mbox_query_fw_fw_pages
// Amount of physical memory to be allocatedfor firmware usage in 4KB pages.
//
// cmd_mbox_query_fw_fw_rev_major
// Firmware Revision - Major
//
// cmd_mbox_query_fw_fw_rev_subminor
// Firmware Sub-minor version (Patch level)
//
// cmd_mbox_query_fw_fw_rev_minor
// Firmware Revision - Minor
//
// cmd_mbox_query_fw_core_clk
// Internal Clock Frequency (in MHz)
//
// cmd_mbox_query_fw_cmd_interface_rev
// Command Interface Interpreter Revision ID. This number is bumped up
// every time a non-backward-compatible change is done for the command
// interface. The current cmd_interface_rev is 1.
//
// cmd_mbox_query_fw_dt
// If set, Debug Trace is supported
//
// cmd_mbox_query_fw_api_version
// Indicates the version of the API, to enable software querying
// for compatibility. The current api_version is 1.
//
// cmd_mbox_query_fw_fw_hour
// Firmware timestamp - hour
//
// cmd_mbox_query_fw_fw_minutes
// Firmware timestamp - minutes
//
// cmd_mbox_query_fw_fw_seconds
// Firmware timestamp - seconds
//
// cmd_mbox_query_fw_fw_year
// Firmware timestamp - year
//
// cmd_mbox_query_fw_fw_month
// Firmware timestamp - month
//
// cmd_mbox_query_fw_fw_day
// Firmware timestamp - day
//
// cmd_mbox_query_fw_lag_mode_support
// 0: CONFIG_PROFILE.lag_mode is not supported by FW
// 1: CONFIG_PROFILE.lag_mode is supported by FW
//
// cmd_mbox_query_fw_cff_support
// 0: CONFIG_PROFILE.flood_mode = 5 (CFF) is not supported by FW
// 1: CONFIG_PROFILE.flood_mode = 5 (CFF) is supported by FW
//
// cmd_mbox_query_fw_clr_int_base_offset
// Clear Interrupt register's offset from clr_int_bar register
// in PCI address space.
//
// cmd_mbox_query_fw_clr_int_bar
// PCI base address register (BAR) where clr_int register is located.
// 00 - BAR 0-1 (64 bit BAR)
//
// cmd_mbox_query_fw_error_buf_offset
// Read Only buffer for internal error reports of offset
// from error_buf_bar register in PCI address space).
//
// cmd_mbox_query_fw_error_buf_size
// Internal error buffer size in DWORDs
//
// cmd_mbox_query_fw_error_int_bar
// PCI base address register (BAR) where error buffer
// register is located.
// 00 - BAR 0-1 (64 bit BAR)
//
// cmd_mbox_query_fw_doorbell_page_offset
// Offset of the doorbell page
//
// cmd_mbox_query_fw_doorbell_page_bar
// PCI base address register (BAR) of the doorbell page
// 00 - BAR 0-1 (64 bit BAR)
//
// cmd_mbox_query_fw_free_running_clock_offset
// The offset of the free running clock page
//
// cmd_mbox_query_fw_fr_rn_clk_bar
// PCI base address register (BAR) of the free running clock page
// 0: BAR 0
// 1: 64 bit BAR
//
// cmd_mbox_query_fw_utc_sec_offset
// The offset of the UTC_Sec page
//
// cmd_mbox_query_fw_utc_sec_bar
// PCI base address register (BAR) of the UTC_Sec page
// 0: BAR 0
// 1: 64 bit BAR
// Reserved on SwitchX/-2, Switch-IB/2, Spectrum-1
//
// cmd_mbox_query_fw_utc_nsec_offset
// The offset of the UTC_nSec page
//
// cmd_mbox_query_fw_utc_nsec_bar
// PCI base address register (BAR) of the UTC_nSec page
// 0: BAR 0
// 1: 64 bit BAR
// Reserved on SwitchX/-2, Switch-IB/2, Spectrum-1
//
// QUERY_BOARDINFO - Query Board Information
// -----------------------------------------
// OpMod == 0 (N/A), INMmod == 0 (N/A)
// -----------------------------------
// The QUERY_BOARDINFO command retrieves adapter specific parameters.
//
// cmd_mbox_boardinfo_intapin
// When PCIe interrupt messages are being used, this value is used for clearing
// an interrupt. When using MSI-X, this register is not used.
//
// cmd_mbox_boardinfo_vsd_vendor_id
// PCISIG Vendor ID (www.pcisig.com/membership/vid_search) of the vendor
// specifying/formatting the VSD. The vsd_vendor_id identifies the management
// domain of the VSD/PSID data. Different vendors may choose different VSD/PSID
// format and encoding as long as they use their assigned vsd_vendor_id.
//
// cmd_mbox_boardinfo_vsd
// Vendor Specific Data. The VSD string that is burnt to the Flash
// with the firmware.
//
pub const MLXSW_CMD_BOARDINFO_VSD_LEN: c_int = 208;
// cmd_mbox_boardinfo_psid
// The PSID field is a 16-ascii (byte) character string which acts as
// the board ID. The PSID format is used in conjunction with
// Mellanox vsd_vendor_id (15B3h).
//
pub const MLXSW_CMD_BOARDINFO_PSID_LEN: c_int = 16;
// QUERY_AQ_CAP - Query Asynchronous Queues Capabilities
// -----------------------------------------------------
// OpMod == 0 (N/A), INMmod == 0 (N/A)
// -----------------------------------
// The QUERY_AQ_CAP command returns the device asynchronous queues
// capabilities supported.
//
// cmd_mbox_query_aq_cap_log_max_sdq_sz
// Log (base 2) of max WQEs allowed on SDQ.
//
// cmd_mbox_query_aq_cap_max_num_sdqs
// Maximum number of SDQs.
//
// cmd_mbox_query_aq_cap_log_max_rdq_sz
// Log (base 2) of max WQEs allowed on RDQ.
//
// cmd_mbox_query_aq_cap_max_num_rdqs
// Maximum number of RDQs.
//
// cmd_mbox_query_aq_cap_log_max_cq_sz
// Log (base 2) of the Maximum CQEs allowed in a CQ for CQEv0 and CQEv1.
//
// cmd_mbox_query_aq_cap_log_max_cqv2_sz
// Log (base 2) of the Maximum CQEs allowed in a CQ for CQEv2.
//
// cmd_mbox_query_aq_cap_max_num_cqs
// Maximum number of CQs.
//
// cmd_mbox_query_aq_cap_log_max_eq_sz
// Log (base 2) of max EQEs allowed on EQ.
//
// cmd_mbox_query_aq_cap_max_num_eqs
// Maximum number of EQs.
//
// cmd_mbox_query_aq_cap_max_sg_sq
// The maximum S/G list elements in an DSQ. DSQ must not contain
// more S/G entries than indicated here.
//
// cmd_mbox_query_aq_cap_
// The maximum S/G list elements in an DRQ. DRQ must not contain
// more S/G entries than indicated here.
//
// MAP_FA - Map Firmware Area
// --------------------------
// OpMod == 0 (N/A), INMmod == Number of VPM entries
// -------------------------------------------------
// The MAP_FA command passes physical pages to the switch. These pages
// are used to store the device firmware. MAP_FA can be executed multiple
// times until all the firmware area is mapped (the size that should be
// mapped is retrieved through the QUERY_FW command). All required pages
// must be mapped to finish the initialization phase. Physical memory
// passed in this command must be pinned.
//
pub const MLXSW_CMD_MAP_FA_VPM_ENTRIES_MAX: c_int = 32;
// cmd_mbox_map_fa_pa
// Physical Address.
//
// cmd_mbox_map_fa_log2size
// Log (base 2) of the size in 4KB pages of the physical and contiguous memory
// that starts at PA_L/H.
//
// UNMAP_FA - Unmap Firmware Area
// ------------------------------
// OpMod == 0 (N/A), INMmod == 0 (N/A)
// -----------------------------------
// The UNMAP_FA command unload the firmware and unmaps all the
// firmware area. After this command is completed the device will not access
// the pages that were mapped to the firmware area. After executing UNMAP_FA
// command, software reset must be done prior to execution of MAP_FW command.
//
extern "C" {
    pub fn mlxsw_cmd_exec_none(_arg: mlxsw_core, _arg: MLXSW_CMD_OPCODE_UNMAP_FA, _arg: 0, _arg: 0) -> return;
}
// QUERY_RESOURCES - Query chip resources
// --------------------------------------
// OpMod == 0 (N/A) , INMmod is index
// ----------------------------------
// The QUERY_RESOURCES command retrieves information related to chip resources
// by resource ID. Every command returns 32 entries. INmod is being use as base.
// for example, index 1 will return entries 32-63. When the tables end and there
// are no more sources in the table, will return resource id 0xFFF to indicate
// it.
//
pub const MLXSW_CMD_QUERY_RESOURCES_TABLE_END_ID: c_uint = 0xffff;
pub const MLXSW_CMD_QUERY_RESOURCES_MAX_QUERIES: c_int = 100;
pub const MLXSW_CMD_QUERY_RESOURCES_PER_QUERY: c_int = 32;
// cmd_mbox_query_resource_id
// The resource id. 0xFFFF indicates table's end.
//
// cmd_mbox_query_resource_data
// The resource
//
// CONFIG_PROFILE (Set) - Configure Switch Profile
// ------------------------------
// OpMod == 1 (Set), INMmod == 0 (N/A)
// -----------------------------------
// The CONFIG_PROFILE command sets the switch profile. The command can be
// executed on the device only once at startup in order to allocate and
// configure all switch resources and prepare it for operational mode.
// It is not possible to change the device profile after the chip is
// in operational mode.
// Failure of the CONFIG_PROFILE command leaves the hardware in an indeterminate
// state therefore it is required to perform software reset to the device
// following an unsuccessful completion of the command. It is required
// to perform software reset to the device to change an existing profile.
//
// cmd_mbox_config_profile_set_max_vepa_channels
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_lag
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_port_per_lag
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_mid
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_pgt
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_system_port
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_vlan_groups
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_regions
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_flood_mode
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_flood_tables
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_ib_mc
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_max_pkey
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_adaptive_routing_group_cap
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_ar_sec
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_ubridge
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_kvd_linear_size
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_kvd_hash_single_size
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_kvd_hash_double_size
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_cqe_version
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_cqe_time_stamp_type
// Capability bit. Setting a bit to 1 configures the profile
// according to the mailbox contents.
//
// cmd_mbox_config_profile_set_lag_mode
// Capability bit. Setting a bit to 1 configures the lag_mode
// according to the mailbox contents.
//
// cmd_mbox_config_profile_max_vepa_channels
// Maximum number of VEPA channels per port (0 through 16)
// 0 - multi-channel VEPA is disabled
//
// cmd_mbox_config_profile_max_lag
// Maximum number of LAG IDs requested.
// Reserved when Spectrum-1/2/3, supported from Spectrum-4 and above.
// For Spectrum-4, firmware sets 128 for values between 1-128 and 256 for values
// between 129-256.
//
// cmd_mbox_config_profile_max_port_per_lag
// Maximum number of ports per LAG requested.
//
// cmd_mbox_config_profile_max_mid
// Maximum Multicast IDs.
// Multicast IDs are allocated from 0 to max_mid-1
//
// cmd_mbox_config_profile_max_pgt
// Maximum records in the Port Group Table per Switch Partition.
// Port Group Table indexes are from 0 to max_pgt-1
//
// cmd_mbox_config_profile_max_system_port
// The maximum number of system ports that can be allocated.
//
// cmd_mbox_config_profile_max_vlan_groups
// Maximum number VLAN Groups for VLAN binding.
//
// cmd_mbox_config_profile_max_regions
// Maximum number of TCAM Regions.
//
// cmd_mbox_config_profile_max_flood_tables
// Maximum number of single-entry flooding tables. Different flooding tables
// can be associated with different packet types.
//
// cmd_mbox_config_profile_max_vid_flood_tables
// Maximum number of per-vid flooding tables. Flooding tables are associated
// to the different packet types for the different switch partitions.
// Table size is 4K entries covering all VID space.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_mbox_config_profile_flood_mode {
// Mixed mode, where:
// max_flood_tables indicates the number of single-entry tables.
// max_vid_flood_tables indicates the number of per-VID tables.
// max_fid_offset_flood_tables indicates the number of FID-offset
// tables. max_fid_flood_tables indicates the number of per-FID tables.
// Reserved when unified bridge model is used.
//
    MLXSW_CMD_MBOX_CONFIG_PROFILE_FLOOD_MODE_MIXED = 3,
// Controlled flood tables. Reserved when legacy bridge model is
// used.
//
    MLXSW_CMD_MBOX_CONFIG_PROFILE_FLOOD_MODE_CONTROLLED = 4,
// CFF - Compressed FID Flood (CFF) mode.
// Reserved when legacy bridge model is used.
// Supported only by Spectrum-2+.
//
    MLXSW_CMD_MBOX_CONFIG_PROFILE_FLOOD_MODE_CFF = 5,
}

// cmd_mbox_config_profile_flood_mode
// Flooding mode to use.
//
// cmd_mbox_config_profile_max_fid_offset_flood_tables
// Maximum number of FID-offset flooding tables.
//
// cmd_mbox_config_profile_fid_offset_flood_table_size
// The size (number of entries) of each FID-offset flood table.
//
// cmd_mbox_config_profile_max_fid_flood_tables
// Maximum number of per-FID flooding tables.
//
// Note: This flooding tables cover special FIDs only (vFIDs), starting at
// FID value 4K and higher.
//
// cmd_mbox_config_profile_fid_flood_table_size
// The size (number of entries) of each per-FID table.
//
// cmd_mbox_config_profile_max_ib_mc
// Maximum number of multicast FDB records for InfiniBand
// FDB (in 512 chunks) per InfiniBand switch partition.
//
// cmd_mbox_config_profile_max_pkey
// Maximum per port PKEY table size (for PKEY enforcement)
//
// cmd_mbox_config_profile_ar_sec
// Primary/secondary capability
// Describes the number of adaptive routing sub-groups
// 0 - disable primary/secondary (single group)
// 1 - enable primary/secondary (2 sub-groups)
// 2 - 3 sub-groups: Not supported in SwitchX, SwitchX-2
// 3 - 4 sub-groups: Not supported in SwitchX, SwitchX-2
//
// cmd_mbox_config_profile_adaptive_routing_group_cap
// Adaptive Routing Group Capability. Indicates the number of AR groups
// supported. Note that when Primary/secondary is enabled, each
// primary/secondary couple consumes 2 adaptive routing entries.
//
// cmd_mbox_config_profile_arn
// Adaptive Routing Notification Enable
// Not supported in SwitchX, SwitchX-2
//
// cmd_mbox_config_profile_ubridge
// Unified Bridge
// 0 - non unified bridge
// 1 - unified bridge
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_mbox_config_profile_lag_mode {
// FW manages PGT LAG table
    MLXSW_CMD_MBOX_CONFIG_PROFILE_LAG_MODE_FW,
// SW manages PGT LAG table
    MLXSW_CMD_MBOX_CONFIG_PROFILE_LAG_MODE_SW,
}

// cmd_mbox_config_profile_lag_mode
// LAG mode
// Configured if set_lag_mode is set
// Supported from Spectrum-2 and above.
// Supported only when ubridge = 1
//
// cmd_mbox_config_kvd_linear_size
// KVD Linear Size
// Valid for Spectrum only
// Allowed values are 128*N where N=0 or higher
//
// cmd_mbox_config_profile_kvd_hash_single_size
// KVD Hash single-entries size
// Valid for Spectrum only
// Allowed values are 128*N where N=0 or higher
// Must be greater or equal to cap_min_kvd_hash_single_size
// Must be smaller or equal to cap_kvd_size - kvd_linear_size
//
// cmd_mbox_config_profile_kvd_hash_double_size
// KVD Hash double-entries size (units of single-size entries)
// Valid for Spectrum only
// Allowed values are 128*N where N=0 or higher
// Must be either 0 or greater or equal to cap_min_kvd_hash_double_size
// Must be smaller or equal to cap_kvd_size - kvd_linear_size
//
// cmd_mbox_config_profile_swid_config_mask
// Modify Switch Partition Configuration mask. When set, the configu-
// ration value for the Switch Partition are taken from the mailbox.
// When clear, the current configuration values are used.
// Bit 0 - set type
// Bit 1 - properties
// Other - reserved
//
// cmd_mbox_config_profile_swid_config_type
// Switch Partition type.
// 0000 - disabled (Switch Partition does not exist)
// 0001 - InfiniBand
// 0010 - Ethernet
// 1000 - router port (SwitchX-2 only)
// Other - reserved
//
// cmd_mbox_config_profile_swid_config_properties
// Switch Partition properties.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_mbox_config_profile_cqe_time_stamp_type {
// uSec - 1.024uSec (default). Only bits 15:0 are valid.
    MLXSW_CMD_MBOX_CONFIG_PROFILE_CQE_TIME_STAMP_TYPE_USEC,
// FRC - Free Running Clock, units of 1nSec.
// Reserved when SwitchX/-2, Switch-IB/2 and Spectrum-1.
//
    MLXSW_CMD_MBOX_CONFIG_PROFILE_CQE_TIME_STAMP_TYPE_FRC,
// UTC. time_stamp[37:30] = Sec, time_stamp[29:0] = nSec.
// Reserved when SwitchX/2, Switch-IB/2 and Spectrum-1.
//
    MLXSW_CMD_MBOX_CONFIG_PROFILE_CQE_TIME_STAMP_TYPE_UTC,
}

// cmd_mbox_config_profile_cqe_time_stamp_type
// CQE time_stamp_type for non-mirror-packets.
// Configured if set_cqe_time_stamp_type is set.
// Reserved when SwitchX/-2, Switch-IB/2 and Spectrum-1.
//
// cmd_mbox_config_profile_cqe_version
// CQE version:
// 0: CQE version is 0
// 1: CQE version is either 1 or 2
// CQE ver 1 or 2 is configured by Completion Queue Context field cqe_ver.
//
// ACCESS_REG - Access EMAD Supported Register
// ----------------------------------
// OpMod == 0 (N/A), INMmod == 0 (N/A)
// -------------------------------------
// The ACCESS_REG command supports accessing device registers. This access
// is mainly used for bootstrapping.
//
// SW2HW_DQ - Software to Hardware DQ
// ----------------------------------
// OpMod == 0 (send DQ) / OpMod == 1 (receive DQ)
// INMmod == DQ number
// ----------------------------------------------
// The SW2HW_DQ command transitions a descriptor queue from software to
// hardware ownership. The command enables posting WQEs and ringing DoorBells
// on the descriptor queue.
//
// cmd_mbox_sw2hw_dq_cq
// Number of the CQ that this Descriptor Queue reports completions to.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_mbox_sw2hw_dq_sdq_lp {
    MLXSW_CMD_MBOX_SW2HW_DQ_SDQ_LP_WQE,
    MLXSW_CMD_MBOX_SW2HW_DQ_SDQ_LP_IGNORE_WQE,
}

// cmd_mbox_sw2hw_dq_sdq_lp
// SDQ local Processing
// 0: local processing by wqe.lp
// 1: local processing (ignoring wqe.lp)
//
// cmd_mbox_sw2hw_dq_sdq_tclass
// SDQ: CPU Egress TClass
// RDQ: Reserved
//
// cmd_mbox_sw2hw_dq_log2_dq_sz
// Log (base 2) of the Descriptor Queue size in 4KB pages.
//
// cmd_mbox_sw2hw_dq_pa
// Physical Address.
//
// HW2SW_DQ - Hardware to Software DQ
// ----------------------------------
// OpMod == 0 (send DQ) / OpMod == 1 (receive DQ)
// INMmod == DQ number
// ----------------------------------------------
// The HW2SW_DQ command transitions a descriptor queue from hardware to
// software ownership. Incoming packets on the DQ are silently discarded,
// SW should not post descriptors on nonoperational DQs.
//
// 2ERR_DQ - To Error DQ
// ---------------------
// OpMod == 0 (send DQ) / OpMod == 1 (receive DQ)
// INMmod == DQ number
// ----------------------------------------------
// The 2ERR_DQ command transitions the DQ into the error state from the state
// in which it has been. While the command is executed, some in-process
// descriptors may complete. Once the DQ transitions into the error state,
// if there are posted descriptors on the RDQ/SDQ, the hardware writes
// a completion with error (flushed) for all descriptors posted in the RDQ/SDQ.
// When the command is completed successfully, the DQ is already in
// the error state.
//
// QUERY_DQ - Query DQ
// ---------------------
// OpMod == 0 (send DQ) / OpMod == 1 (receive DQ)
// INMmod == DQ number
// ----------------------------------------------
// The QUERY_DQ command retrieves a snapshot of DQ parameters from the hardware.
//
// Note: Output mailbox has the same format as SW2HW_DQ.
//
// SW2HW_CQ - Software to Hardware CQ
// ----------------------------------
// OpMod == 0 (N/A), INMmod == CQ number
// -------------------------------------
// The SW2HW_CQ command transfers ownership of a CQ context entry from software
// to hardware. The command takes the CQ context entry from the input mailbox
// and stores it in the CQC in the ownership of the hardware. The command fails
// if the requested CQC entry is already in the ownership of the hardware.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_cmd_mbox_sw2hw_cq_cqe_ver {
    MLXSW_CMD_MBOX_SW2HW_CQ_CQE_VER_1,
    MLXSW_CMD_MBOX_SW2HW_CQ_CQE_VER_2,
}

// cmd_mbox_sw2hw_cq_cqe_ver
// CQE Version.
//
// cmd_mbox_sw2hw_cq_c_eqn
// Event Queue this CQ reports completion events to.
//
// cmd_mbox_sw2hw_cq_st
// Event delivery state machine
// 0x0 - FIRED
// 0x1 - ARMED (Request for Notification)
//
// cmd_mbox_sw2hw_cq_log_cq_size
// Log (base 2) of the CQ size (in entries).
//
// cmd_mbox_sw2hw_cq_producer_counter
// Producer Counter. The counter is incremented for each CQE that is
// written by the HW to the CQ.
// Maintained by HW (valid for the QUERY_CQ command only)
//
// cmd_mbox_sw2hw_cq_pa
// Physical Address.
//
// HW2SW_CQ - Hardware to Software CQ
// ----------------------------------
// OpMod == 0 (N/A), INMmod == CQ number
// -------------------------------------
// The HW2SW_CQ command transfers ownership of a CQ context entry from hardware
// to software. The CQC entry is invalidated as a result of this command.
//
// QUERY_CQ - Query CQ
// ----------------------------------
// OpMod == 0 (N/A), INMmod == CQ number
// -------------------------------------
// The QUERY_CQ command retrieves a snapshot of the current CQ context entry.
// The command stores the snapshot in the output mailbox in the software format.
// Note that the CQ context state and values are not affected by the QUERY_CQ
// command. The QUERY_CQ command is for debug purposes only.
//
// Note: Output mailbox has the same format as SW2HW_CQ.
//
// SW2HW_EQ - Software to Hardware EQ
// ----------------------------------
// OpMod == 0 (N/A), INMmod == EQ number
// -------------------------------------
// The SW2HW_EQ command transfers ownership of an EQ context entry from software
// to hardware. The command takes the EQ context entry from the input mailbox
// and stores it in the EQC in the ownership of the hardware. The command fails
// if the requested EQC entry is already in the ownership of the hardware.
//
// cmd_mbox_sw2hw_eq_int_msix
// When set, MSI-X cycles will be generated by this EQ.
// When cleared, an interrupt will be generated by this EQ.
//
// cmd_mbox_sw2hw_eq_st
// Event delivery state machine
// 0x0 - FIRED
// 0x1 - ARMED (Request for Notification)
// 0x11 - Always ARMED
// other - reserved
//
// cmd_mbox_sw2hw_eq_log_eq_size
// Log (base 2) of the EQ size (in entries).
//
// cmd_mbox_sw2hw_eq_producer_counter
// Producer Counter. The counter is incremented for each EQE that is written
// by the HW to the EQ.
// Maintained by HW (valid for the QUERY_EQ command only)
//
// cmd_mbox_sw2hw_eq_pa
// Physical Address.
//
// HW2SW_EQ - Hardware to Software EQ
// ----------------------------------
// OpMod == 0 (N/A), INMmod == EQ number
// -------------------------------------
//
// QUERY_EQ - Query EQ
// ----------------------------------
// OpMod == 0 (N/A), INMmod == EQ number
// -------------------------------------
//
// Note: Output mailbox has the same format as SW2HW_EQ.
//
