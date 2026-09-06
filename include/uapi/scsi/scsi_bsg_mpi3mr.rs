//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/scsi_bsg_mpi3mr.h
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


// SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note
//
// Driver for Broadcom MPI3 Storage Controllers
//
// Copyright (C) 2017-2022 Broadcom Inc.
// (mailto: mpi3mr-linuxdrv.pdl@broadcom.com)
//

// Macro flag: #define SCSI_BSG_MPI3MR_H_INCLUDED

// Definitions for BSG commands
pub const MPI3MR_IOCTL_VERSION: c_uint = 0x06;

pub const MPI3MR_BSG_ADPTYPE_UNKNOWN: c_int = 0;
pub const MPI3MR_BSG_ADPTYPE_AVGFAMILY: c_int = 1;
pub const MPI3MR_BSG_ADPSTATE_UNKNOWN: c_int = 0;
pub const MPI3MR_BSG_ADPSTATE_OPERATIONAL: c_int = 1;
pub const MPI3MR_BSG_ADPSTATE_FAULT: c_int = 2;
pub const MPI3MR_BSG_ADPSTATE_IN_RESET: c_int = 3;
pub const MPI3MR_BSG_ADPSTATE_UNRECOVERABLE: c_int = 4;
pub const MPI3MR_BSG_ADPRESET_UNKNOWN: c_int = 0;
pub const MPI3MR_BSG_ADPRESET_SOFT: c_int = 1;
pub const MPI3MR_BSG_ADPRESET_DIAG_FAULT: c_int = 2;
pub const MPI3MR_BSG_LOGDATA_MAX_ENTRIES: c_int = 400;
pub const MPI3MR_BSG_LOGDATA_ENTRY_HEADER_SZ: c_int = 4;
pub const MPI3MR_DRVBSG_OPCODE_UNKNOWN: c_int = 0;
pub const MPI3MR_DRVBSG_OPCODE_ADPINFO: c_int = 1;
pub const MPI3MR_DRVBSG_OPCODE_ADPRESET: c_int = 2;
pub const MPI3MR_DRVBSG_OPCODE_ALLTGTDEVINFO: c_int = 4;
pub const MPI3MR_DRVBSG_OPCODE_GETCHGCNT: c_int = 5;
pub const MPI3MR_DRVBSG_OPCODE_LOGDATAENABLE: c_int = 6;
pub const MPI3MR_DRVBSG_OPCODE_PELENABLE: c_int = 7;
pub const MPI3MR_DRVBSG_OPCODE_GETLOGDATA: c_int = 8;
pub const MPI3MR_DRVBSG_OPCODE_QUERY_HDB: c_int = 9;
pub const MPI3MR_DRVBSG_OPCODE_REPOST_HDB: c_int = 10;
pub const MPI3MR_DRVBSG_OPCODE_UPLOAD_HDB: c_int = 11;
pub const MPI3MR_DRVBSG_OPCODE_REFRESH_HDB_TRIGGERS: c_int = 12;
pub const MPI3MR_BSG_BUFTYPE_UNKNOWN: c_int = 0;
pub const MPI3MR_BSG_BUFTYPE_RAIDMGMT_CMD: c_int = 1;
pub const MPI3MR_BSG_BUFTYPE_RAIDMGMT_RESP: c_int = 2;
pub const MPI3MR_BSG_BUFTYPE_DATA_IN: c_int = 3;
pub const MPI3MR_BSG_BUFTYPE_DATA_OUT: c_int = 4;
pub const MPI3MR_BSG_BUFTYPE_MPI_REPLY: c_int = 5;
pub const MPI3MR_BSG_BUFTYPE_ERR_RESPONSE: c_int = 6;
pub const MPI3MR_BSG_BUFTYPE_MPI_REQUEST: c_uint = 0xFE;
pub const MPI3MR_BSG_MPI_REPLY_BUFTYPE_UNKNOWN: c_int = 0;
pub const MPI3MR_BSG_MPI_REPLY_BUFTYPE_STATUS: c_int = 1;
pub const MPI3MR_BSG_MPI_REPLY_BUFTYPE_ADDRESS: c_int = 2;
pub const MPI3MR_HDB_BUFTYPE_UNKNOWN: c_int = 0;
pub const MPI3MR_HDB_BUFTYPE_TRACE: c_int = 1;
pub const MPI3MR_HDB_BUFTYPE_FIRMWARE: c_int = 2;
pub const MPI3MR_HDB_BUFTYPE_RESERVED: c_int = 3;
pub const MPI3MR_HDB_BUFSTATUS_UNKNOWN: c_int = 0;
pub const MPI3MR_HDB_BUFSTATUS_NOT_ALLOCATED: c_int = 1;
pub const MPI3MR_HDB_BUFSTATUS_POSTED_UNPAUSED: c_int = 2;
pub const MPI3MR_HDB_BUFSTATUS_POSTED_PAUSED: c_int = 3;
pub const MPI3MR_HDB_BUFSTATUS_RELEASED: c_int = 4;
pub const MPI3MR_HDB_TRIGGER_TYPE_UNKNOWN: c_int = 0;
pub const MPI3MR_HDB_TRIGGER_TYPE_DIAGFAULT: c_int = 1;
pub const MPI3MR_HDB_TRIGGER_TYPE_ELEMENT: c_int = 2;
pub const MPI3MR_HDB_TRIGGER_TYPE_MASTER: c_int = 3;
// Supported BSG commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum command {
    MPI3MR_DRV_CMD = 1,
    MPI3MR_MPT_CMD = 2,
}

//
// struct mpi3_driver_info_layout - Information about driver
//
// @information_length: Length of this structure in bytes
// @driver_signature: Driver Vendor name
// @os_name: Operating System Name
// @driver_name: Driver name
// @driver_version: Driver version
// @driver_release_date: Driver release date
// @driver_capabilities: Driver capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_driver_info_layout {
    pub information_length: __le32,
    pub driver_signature: [__u8; 12],
    pub os_name: [__u8; 16],
    pub os_version: [__u8; 12],
    pub driver_name: [__u8; 20],
    pub driver_version: [__u8; 32],
    pub driver_release_date: [__u8; 20],
    pub driver_capabilities: __le32,
}

//
// struct mpi3mr_bsg_in_adpinfo - Adapter information request
// data returned by the driver.
//
// @adp_type: Adapter type
// @rsvd1: Reserved
// @pci_dev_id: PCI device ID of the adapter
// @pci_dev_hw_rev: PCI revision of the adapter
// @pci_subsys_dev_id: PCI subsystem device ID of the adapter
// @pci_subsys_ven_id: PCI subsystem vendor ID of the adapter
// @pci_dev: PCI device
// @pci_func: PCI function
// @pci_bus: PCI bus
// @rsvd2: Reserved
// @pci_seg_id: PCI segment ID
// @app_intfc_ver: version of the application interface definition
// @rsvd3: Reserved
// @rsvd4: Reserved
// @rsvd5: Reserved
// @driver_info: Driver Information (Version/Name)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_in_adpinfo {
    pub adp_type: __u32,
    pub rsvd1: __u32,
    pub pci_dev_id: __u32,
    pub pci_dev_hw_rev: __u32,
    pub pci_subsys_dev_id: __u32,
    pub pci_subsys_ven_id: __u32,
    pub pci_dev:5: __u32,
    pub pci_func:3: __u32,
    pub pci_bus:8: __u32,
    pub rsvd2: __u16,
    pub pci_seg_id: __u32,
    pub app_intfc_ver: __u32,
    pub adp_state: __u8,
    pub rsvd3: __u8,
    pub rsvd4: __u16,
    pub rsvd5: [__u32; 2],
    pub driver_info: mpi3_driver_info_layout,
}

//
// struct mpi3mr_bsg_adp_reset - Adapter reset request
// payload data to the driver.
//
// @reset_type: Reset type
// @rsvd1: Reserved
// @rsvd2: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_adp_reset {
    pub reset_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
}

//
// struct mpi3mr_change_count - Topology change count
// returned by the driver.
//
// @change_count: Topology change count
// @rsvd: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_change_count {
    pub change_count: __u16,
    pub rsvd: __u16,
}

//
// struct mpi3mr_device_map_info - Target device mapping
// information
//
// @handle: Firmware device handle
// @perst_id: Persistent ID assigned by the firmware
// @target_id: Target ID assigned by the driver
// @bus_id: Bus ID assigned by the driver
// @rsvd1: Reserved
// @rsvd2: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_device_map_info {
    pub handle: __u16,
    pub perst_id: __u16,
    pub target_id: __u32,
    pub bus_id: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
}

//
// struct mpi3mr_all_tgt_info - Target device mapping
// information returned by the driver
//
// @num_devices: The number of devices in driver's inventory
// @rsvd1: Reserved
// @rsvd2: Reserved
// @dmi: Variable length array of mapping information of targets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_all_tgt_info {
    pub num_devices: __u16,
    pub rsvd1: __u16,
    pub rsvd2: __u32,
    pub dmi: [mpi3mr_device_map_info; 1],
}

//
// struct mpi3mr_logdata_enable - Number of log data
// entries saved by the driver returned as payload data for
// enable logdata BSG request by the driver.
//
// @max_entries: Number of log data entries cached by the driver
// @rsvd: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_logdata_enable {
    pub max_entries: __u16,
    pub rsvd: __u16,
}

//
// struct mpi3mr_bsg_out_pel_enable - PEL enable request payload
// data to the driver.
//
// @pel_locale: PEL locale to the firmware
// @pel_class: PEL class to the firmware
// @rsvd: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_out_pel_enable {
    pub pel_locale: __u16,
    pub pel_class: __u8,
    pub rsvd: __u8,
}

//
// struct mpi3mr_logdata_entry - Log data entry cached by the
// driver.
//
// @valid_entry: Is the entry valid
// @rsvd1: Reserved
// @rsvd2: Reserved
// @data: Variable length Log entry data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_logdata_entry {
    pub valid_entry: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
    pub /: *mut *mut __u8 data[1]; / Variable length Array,
}

//
// struct mpi3mr_bsg_in_log_data - Log data entries saved by
// the driver returned as payload data for Get logdata request
// by the driver.
//
// @entry: Variable length Log data entry array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_in_log_data {
    pub entry: [mpi3mr_logdata_entry; 1],
}

//
// struct mpi3mr_hdb_entry - host diag buffer entry.
//
// @buf_type: Buffer type
// @status: Buffer status
// @trigger_type: Trigger type
// @rsvd1: Reserved
// @size: Buffer size
// @rsvd2: Reserved
// @trigger_data: Trigger specific data
// @rsvd3: Reserved
// @rsvd4: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_hdb_entry {
    pub buf_type: __u8,
    pub status: __u8,
    pub trigger_type: __u8,
    pub rsvd1: __u8,
    pub size: __u16,
    pub rsvd2: __u16,
    pub trigger_data: __u64,
    pub rsvd3: __u32,
    pub rsvd4: __u32,
}

//
// struct mpi3mr_bsg_in_hdb_status - This structure contains
// return data for the BSG request to retrieve the number of host
// diagnostic buffers supported by the driver and their current
// status and additional status specific data if any in forms of
// multiple hdb entries.
//
// @num_hdb_types: Number of host diag buffer types supported
// @element_trigger_format: Element trigger format
// @rsvd1: Reserved
// @rsvd2: Reserved
// @rsvd3: Reserved
// @entry: Variable length Diag buffer status entry array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_in_hdb_status {
    pub num_hdb_types: __u8,
    pub element_trigger_format: __u8,
    pub rsvd2: __u16,
    pub rsvd3: __u32,
    pub entry: [mpi3mr_hdb_entry; 1],
}

//
// struct mpi3mr_bsg_out_repost_hdb - Repost host diagnostic
// buffer request payload data to the driver.
//
// @buf_type: Buffer type
// @rsvd1: Reserved
// @rsvd2: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_out_repost_hdb {
    pub buf_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
}

//
// struct mpi3mr_bsg_out_upload_hdb - Upload host diagnostic
// buffer request payload data to the driver.
//
// @buf_type: Buffer type
// @rsvd1: Reserved
// @rsvd2: Reserved
// @start_offset: Start offset of the buffer from where to copy
// @length: Length of the buffer to copy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_out_upload_hdb {
    pub buf_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
    pub start_offset: __u32,
    pub length: __u32,
}

//
// struct mpi3mr_bsg_out_refresh_hdb_triggers - Refresh host
// diagnostic buffer triggers request payload data to the driver.
//
// @page_type: Page type
// @rsvd1: Reserved
// @rsvd2: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_out_refresh_hdb_triggers {
    pub page_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
}

//
// struct mpi3mr_bsg_drv_cmd -  Generic bsg data
// structure for all driver specific requests.
//
// @mrioc_id: Controller ID
// @opcode: Driver specific opcode
// @rsvd1: Reserved
// @rsvd2: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_drv_cmd {
    pub mrioc_id: __u8,
    pub opcode: __u8,
    pub rsvd1: __u16,
    pub rsvd2: [__u32; 4],
}

//
// struct mpi3mr_bsg_in_reply_buf - MPI reply buffer returned
// for MPI Passthrough request .
//
// @mpi_reply_type: Type of MPI reply
// @rsvd1: Reserved
// @rsvd2: Reserved
// @reply_buf: Variable Length buffer based on mpirep type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_in_reply_buf {
    pub mpi_reply_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
    pub reply_buf: [__u8; ],
}

//
// struct mpi3mr_buf_entry - User buffer descriptor for MPI
// Passthrough requests.
//
// @buf_type: Buffer type
// @rsvd1: Reserved
// @rsvd2: Reserved
// @buf_len: Buffer length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_buf_entry {
    pub buf_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
    pub buf_len: __u32,
}

//
// struct mpi3mr_buf_entry_list - list of user buffer
// descriptor for MPI Passthrough requests.
//
// @num_of_entries: Number of buffer descriptors
// @rsvd1: Reserved
// @rsvd2: Reserved
// @rsvd3: Reserved
// @buf_entry: Variable length array of buffer descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_buf_entry_list {
    pub num_of_entries: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
    pub rsvd3: __u32,
    pub buf_entry: [mpi3mr_buf_entry; 1],
}

//
// struct mpi3mr_bsg_mptcmd -  Generic bsg data
// structure for all MPI Passthrough requests .
//
// @mrioc_id: Controller ID
// @rsvd1: Reserved
// @timeout: MPI request timeout
// @rsvd2: Reserved
// @buf_entry_list: Buffer descriptor list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_mptcmd {
    pub mrioc_id: __u8,
    pub rsvd1: __u8,
    pub timeout: __u16,
    pub rsvd2: __u32,
    pub buf_entry_list: mpi3mr_buf_entry_list,
}

//
// struct mpi3mr_bsg_packet -  Generic bsg data
// structure for all supported requests .
//
// @cmd_type: represents drvrcmd or mptcmd
// @rsvd1: Reserved
// @rsvd2: Reserved
// @rsvd3: Reserved
// @cmd.drvrcmd: driver request structure
// @cmd.mptcmd: mpt request structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_bsg_packet {
    pub cmd_type: __u8,
    pub rsvd1: __u8,
    pub rsvd2: __u16,
    pub rsvd3: __u32,
    pub drvrcmd: mpi3mr_bsg_drv_cmd,
    pub mptcmd: mpi3mr_bsg_mptcmd,
    pub cmd: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_nvme_encapsulated_request {
    pub host_tag: __le16,
    pub ioc_use_only02: __u8,
    pub function: __u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: __u8,
    pub msg_flags: __u8,
    pub change_count: __le16,
    pub dev_handle: __le16,
    pub encapsulated_command_length: __le16,
    pub flags: __le16,
    pub data_length: __le32,
    pub reserved14: [__le32; 3],
    pub command: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_nvme_encapsulated_error_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: __u8,
    pub function: __u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: __u8,
    pub msg_flags: __u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub nvme_completion_entry: [__le32; 4],
}

pub const MPI3MR_NVME_DATA_FORMAT_PRP: c_int = 0;
pub const MPI3MR_NVME_DATA_FORMAT_SGL1: c_int = 1;
pub const MPI3MR_NVME_DATA_FORMAT_SGL2: c_int = 2;
pub const MPI3MR_NVMESGL_DATA_SEGMENT: c_uint = 0x00;
pub const MPI3MR_NVMESGL_LAST_SEGMENT: c_uint = 0x03;
// MPI3: task management related definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_scsi_task_mgmt_request {
    pub host_tag: __le16,
    pub ioc_use_only02: __u8,
    pub function: __u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: __u8,
    pub msg_flags: __u8,
    pub change_count: __le16,
    pub dev_handle: __le16,
    pub task_host_tag: __le16,
    pub task_type: __u8,
    pub reserved0f: __u8,
    pub task_request_queue_id: __le16,
    pub reserved12: __le16,
    pub reserved14: __le32,
    pub lun: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_scsi_task_mgmt_reply {
    pub host_tag: __le16,
    pub ioc_use_only02: __u8,
    pub function: __u8,
    pub ioc_use_only04: __le16,
    pub ioc_use_only06: __u8,
    pub msg_flags: __u8,
    pub ioc_use_only08: __le16,
    pub ioc_status: __le16,
    pub ioc_log_info: __le32,
    pub termination_count: __le32,
    pub response_data: __le32,
    pub reserved18: __le32,
}

// MPI3: PEL related definitions

// MPI3: Function definitions

