//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pmcraid.h
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
// pmcraid.h -- PMC Sierra MaxRAID controller driver header file
//
// Written By: Anil Ravindranath<anil_ravindranath@pmc-sierra.com>
// PMC-Sierra Inc
//
// Copyright (C) 2008, 2009 PMC Sierra Inc.
//

//
// Driver name   : string representing the driver name
// Device file   : /dev file to be used for management interfaces
// Driver version: version string in major_version.minor_version.patch format
// Driver date   : date information in "Mon dd yyyy" format
//

pub const PMCRAID_FW_VERSION_1: c_uint = 0x002;
// Maximum number of adapters supported by current version of the driver
pub const PMCRAID_MAX_ADAPTERS: c_int = 1024;
// Bit definitions as per firmware, bit position [0][1][2].....[31]

// PMC PCI vendor ID and device ID values
pub const PCI_VENDOR_ID_PMC: c_uint = 0x11F8;
pub const PCI_DEVICE_ID_PMC_MAXRAID: c_uint = 0x5220;
//
// MAX_CMD          : maximum commands that can be outstanding with IOA
// MAX_IO_CMD       : command blocks available for IO commands
// MAX_HCAM_CMD     : command blocks available for HCAMS
// MAX_INTERNAL_CMD : command blocks available for internal commands like reset
//
pub const PMCRAID_MAX_CMD: c_int = 1024;
pub const PMCRAID_MAX_IO_CMD: c_int = 1020;
pub const PMCRAID_MAX_HCAM_CMD: c_int = 2;
pub const PMCRAID_MAX_INTERNAL_CMD: c_int = 2;
// MAX_IOADLS       : max number of scatter-gather lists supported by IOA
// IOADLS_INTERNAL  : number of ioadls included as part of IOARCB.
// IOADLS_EXTERNAL  : number of ioadls allocated external to IOARCB
//
pub const PMCRAID_IOADLS_INTERNAL: c_int = 27;
pub const PMCRAID_IOADLS_EXTERNAL: c_int = 37;

// HRRQ_ENTRY_SIZE  : size of hrrq buffer
// IOARCB_ALIGNMENT : alignment required for IOARCB
// IOADL_ALIGNMENT  : alignment requirement for IOADLs
// MSIX_VECTORS     : number of MSIX vectors supported
//

pub const PMCRAID_IOARCB_ALIGNMENT: c_int = 32;
pub const PMCRAID_IOADL_ALIGNMENT: c_int = 16;
pub const PMCRAID_IOASA_ALIGNMENT: c_int = 4;
pub const PMCRAID_NUM_MSIX_VECTORS: c_int = 16;
// various other limits
pub const PMCRAID_VENDOR_ID_LEN: c_int = 8;
pub const PMCRAID_PRODUCT_ID_LEN: c_int = 16;
pub const PMCRAID_SERIAL_NUM_LEN: c_int = 8;
pub const PMCRAID_LUN_LEN: c_int = 8;
pub const PMCRAID_MAX_CDB_LEN: c_int = 16;
pub const PMCRAID_DEVICE_ID_LEN: c_int = 8;
pub const PMCRAID_SENSE_DATA_LEN: c_int = 256;
pub const PMCRAID_ADD_CMD_PARAM_LEN: c_int = 48;
pub const PMCRAID_MAX_BUS_TO_SCAN: c_int = 1;
pub const PMCRAID_MAX_NUM_TARGETS_PER_BUS: c_int = 256;
pub const PMCRAID_MAX_NUM_LUNS_PER_TARGET: c_int = 8;
// IOA bus/target/lun number of IOA resources
pub const PMCRAID_IOA_BUS_ID: c_uint = 0xfe;
pub const PMCRAID_IOA_TARGET_ID: c_uint = 0xff;
pub const PMCRAID_IOA_LUN_ID: c_uint = 0xff;
pub const PMCRAID_VSET_BUS_ID: c_uint = 0x1;
pub const PMCRAID_VSET_LUN_ID: c_uint = 0x0;
pub const PMCRAID_PHYS_BUS_ID: c_uint = 0x0;
pub const PMCRAID_VIRTUAL_ENCL_BUS_ID: c_uint = 0x8;
pub const PMCRAID_MAX_VSET_TARGETS: c_uint = 0x7F;
pub const PMCRAID_MAX_VSET_LUNS_PER_TARGET: c_int = 8;
pub const PMCRAID_IOA_MAX_SECTORS: c_int = 32767;
pub const PMCRAID_VSET_MAX_SECTORS: c_int = 512;
pub const PMCRAID_MAX_CMD_PER_LUN: c_int = 254;
// Number of configuration table entries (resources), includes 1 FP,
// 1 Enclosure device
//
pub const PMCRAID_MAX_RESOURCES: c_int = 256;
// Adapter Commands used by driver
pub const PMCRAID_QUERY_RESOURCE_STATE: c_uint = 0xC2;
pub const PMCRAID_RESET_DEVICE: c_uint = 0xC3;
// options to select reset target
pub const ENABLE_RESET_MODIFIER: c_uint = 0x80;
pub const RESET_DEVICE_LUN: c_uint = 0x40;
pub const RESET_DEVICE_TARGET: c_uint = 0x20;
pub const RESET_DEVICE_BUS: c_uint = 0x10;
pub const PMCRAID_IDENTIFY_HRRQ: c_uint = 0xC4;
pub const PMCRAID_QUERY_IOA_CONFIG: c_uint = 0xC5;
pub const PMCRAID_QUERY_CMD_STATUS: c_uint = 0xCB;
pub const PMCRAID_ABORT_CMD: c_uint = 0xC7;
// CANCEL ALL command, provides option for setting SYNC_COMPLETE
// on the target resources for which commands got cancelled
//
pub const PMCRAID_CANCEL_ALL_REQUESTS: c_uint = 0xCE;

// HCAM command and types of HCAM supported by IOA
pub const PMCRAID_HOST_CONTROLLED_ASYNC: c_uint = 0xCF;
pub const PMCRAID_HCAM_CODE_CONFIG_CHANGE: c_uint = 0x01;
pub const PMCRAID_HCAM_CODE_LOG_DATA: c_uint = 0x02;
// IOA shutdown command and various shutdown types
pub const PMCRAID_IOA_SHUTDOWN: c_uint = 0xF7;
pub const PMCRAID_SHUTDOWN_NORMAL: c_uint = 0x00;
pub const PMCRAID_SHUTDOWN_PREPARE_FOR_NORMAL: c_uint = 0x40;
pub const PMCRAID_SHUTDOWN_NONE: c_uint = 0x100;
pub const PMCRAID_SHUTDOWN_ABBREV: c_uint = 0x80;
// SET SUPPORTED DEVICES command and the option to select all the
// devices to be supported
//
pub const PMCRAID_SET_SUPPORTED_DEVICES: c_uint = 0xFB;

// This option is used with SCSI WRITE_BUFFER command
pub const PMCRAID_WR_BUF_DOWNLOAD_AND_SAVE: c_uint = 0x05;
// IOASC Codes used by driver
pub const PMCRAID_IOASC_SENSE_MASK: c_uint = 0xFFFFFF00;

pub const PMCRAID_IOASC_GOOD_COMPLETION: c_uint = 0x00000000;
pub const PMCRAID_IOASC_GC_IOARCB_NOTFOUND: c_uint = 0x005A0000;
pub const PMCRAID_IOASC_NR_INIT_CMD_REQUIRED: c_uint = 0x02040200;
pub const PMCRAID_IOASC_NR_IOA_RESET_REQUIRED: c_uint = 0x02048000;
pub const PMCRAID_IOASC_NR_SYNC_REQUIRED: c_uint = 0x023F0000;
pub const PMCRAID_IOASC_ME_READ_ERROR_NO_REALLOC: c_uint = 0x03110C00;
pub const PMCRAID_IOASC_HW_CANNOT_COMMUNICATE: c_uint = 0x04050000;
pub const PMCRAID_IOASC_HW_DEVICE_TIMEOUT: c_uint = 0x04080100;
pub const PMCRAID_IOASC_HW_DEVICE_BUS_STATUS_ERROR: c_uint = 0x04448500;
pub const PMCRAID_IOASC_HW_IOA_RESET_REQUIRED: c_uint = 0x04448600;
pub const PMCRAID_IOASC_IR_INVALID_RESOURCE_HANDLE: c_uint = 0x05250000;
pub const PMCRAID_IOASC_AC_TERMINATED_BY_HOST: c_uint = 0x0B5A0000;
pub const PMCRAID_IOASC_UA_BUS_WAS_RESET: c_uint = 0x06290000;
pub const PMCRAID_IOASC_TIME_STAMP_OUT_OF_SYNC: c_uint = 0x06908B00;
pub const PMCRAID_IOASC_UA_BUS_WAS_RESET_BY_OTHER: c_uint = 0x06298000;
// Driver defined IOASCs
pub const PMCRAID_IOASC_IOA_WAS_RESET: c_uint = 0x10000001;
pub const PMCRAID_IOASC_PCI_ACCESS_ERROR: c_uint = 0x10000002;
// Various timeout values (in milliseconds) used. If any of these are chip
// specific, move them to pmcraid_chip_details structure.
//
pub const PMCRAID_PCI_DEASSERT_TIMEOUT: c_int = 2000;
pub const PMCRAID_BIST_TIMEOUT: c_int = 2000;
pub const PMCRAID_AENWAIT_TIMEOUT: c_int = 5000;
pub const PMCRAID_TRANSOP_TIMEOUT: c_int = 60000;

// structure to represent a scatter-gather element (IOADL descriptor)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioadl_desc {
    pub address: __le64,
    pub data_len: __le32,
    pub reserved: [__u8; 3],
    pub flags: __u8,
// C attribute field omitted
// pmcraid_ioadl_desc.flags values

// additional IOARCB data which can be CDB or additional request parameters
// or list of IOADLs. Firmware supports max of 512 bytes for IOARCB, hence then
// number of IOADLs are limted to 27. In case they are more than 27, they will
// be used in chained form
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioarcb_add_data {
    pub ioadl: [pmcraid_ioadl_desc; PMCRAID_IOADLS_INTERNAL],
    pub add_cmd_params: [__u8; PMCRAID_ADD_CMD_PARAM_LEN],
    pub u: },
}

//
// IOA Request Control Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioarcb {
    pub ioarcb_bus_addr: __le64,
    pub resource_handle: __le32,
    pub response_handle: __le32,
    pub ioadl_bus_addr: __le64,
    pub ioadl_length: __le32,
    pub data_transfer_length: __le32,
    pub ioasa_bus_addr: __le64,
    pub ioasa_len: __le16,
    pub cmd_timeout: __le16,
    pub add_cmd_param_offset: __le16,
    pub add_cmd_param_length: __le16,
    pub reserved1: [__le32; 2],
    pub reserved2: __le32,
    pub request_type: __u8,
    pub request_flags0: __u8,
    pub request_flags1: __u8,
    pub hrrq_id: __u8,
    pub cdb: [__u8; PMCRAID_MAX_CDB_LEN],
    pub add_data: pmcraid_ioarcb_add_data,
}

// well known resource handle values
pub const PMCRAID_IOA_RES_HANDLE: c_uint = 0xffffffff;
pub const PMCRAID_INVALID_RES_HANDLE: c_int = 0;
// pmcraid_ioarcb.request_type values
pub const REQ_TYPE_SCSI: c_uint = 0x00;
pub const REQ_TYPE_IOACMD: c_uint = 0x01;
pub const REQ_TYPE_HCAM: c_uint = 0x02;
// pmcraid_ioarcb.flags0 values

// pmcraid_ioarcb.flags1 values

pub const TASK_TAG_SIMPLE: c_uint = 0x10;
pub const TASK_TAG_ORDERED: c_uint = 0x20;
pub const TASK_TAG_QUEUE_HEAD: c_uint = 0x30;
// toggle bit offset in response handle
pub const HRRQ_TOGGLE_BIT: c_uint = 0x01;
pub const HRRQ_RESPONSE_BIT: c_uint = 0x02;
// IOA Status Area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioasa_vset {
    pub failing_lba_hi: __le32,
    pub failing_lba_lo: __le32,
    pub reserved: __le32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioasa {
    pub ioasc: __le32,
    pub returned_status_length: __le16,
    pub available_status_length: __le16,
    pub residual_data_length: __le32,
    pub ilid: __le32,
    pub fd_ioasc: __le32,
    pub fd_res_address: __le32,
    pub fd_res_handle: __le32,
    pub reserved: __le32,
// resource specific sense information
    pub vset: pmcraid_ioasa_vset,
    pub u: },
// IOA autosense data
    pub auto_sense_length: __le16,
    pub error_data_length: __le16,
    pub sense_data: [__u8; PMCRAID_SENSE_DATA_LEN],
// C attribute field omitted
pub const PMCRAID_DRIVER_ILID: c_uint = 0xffffffff;
// Config Table Entry per Resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_config_table_entry {
    pub resource_type: __u8,
    pub bus_protocol: __u8,
    pub array_id: __le16,
    pub common_flags0: __u8,
    pub common_flags1: __u8,
    pub unique_flags0: __u8,
    pub /: *mut *mut __u8 unique_flags1; /also used as vset target_id,
    pub resource_handle: __le32,
    pub resource_address: __le32,
    pub device_id: [__u8; PMCRAID_DEVICE_ID_LEN],
    pub lun: [__u8; PMCRAID_LUN_LEN],
// C attribute field omitted
// extended configuration table sizes are also of 32 bytes in size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_config_table_entry_ext {
    pub cfgte: pmcraid_config_table_entry,
}

// resource types (config_table_entry.resource_type values)
pub const RES_TYPE_AF_DASD: c_uint = 0x00;
pub const RES_TYPE_GSCSI: c_uint = 0x01;
pub const RES_TYPE_VSET: c_uint = 0x02;
pub const RES_TYPE_IOA_FP: c_uint = 0xFF;

// bus_protocol values used by driver
pub const RES_TYPE_VENCLOSURE: c_uint = 0x8;
// config_table_entry.common_flags0

// unique_flags1

// well known resource handle values
pub const RES_HANDLE_IOA: c_uint = 0xFFFFFFFF;
pub const RES_HANDLE_NONE: c_uint = 0x00000000;
// well known resource address values
pub const RES_ADDRESS_IOAFP: c_uint = 0xFEFFFFFF;
pub const RES_ADDRESS_INVALID: c_uint = 0xFFFFFFFF;
// BUS/TARGET/LUN values from resource_addrr

pub const RES_LUN(res_addr): c_uint = 0x0;
// configuration table structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_config_table {
    pub num_entries: __le16,
    pub table_format: __u8,
    pub reserved1: __u8,
    pub flags: __u8,
    pub reserved2: [__u8; 11],
}

// config_table.flags value

//
// HCAM format
//
pub const PMCRAID_HOSTRCB_LDNSIZE: c_int = 4056;
// Error log notification format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_hostrcb_error {
    pub fd_ioasc: __le32,
    pub fd_ra: __le32,
    pub fd_rh: __le32,
    pub prc: __le32,
    pub data: [__u8; PMCRAID_HOSTRCB_LDNSIZE],
    pub u: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_hcam_hdr {
    pub op_code: __u8,
    pub notification_type: __u8,
    pub notification_lost: __u8,
    pub flags: __u8,
    pub overlay_id: __u8,
    pub reserved1: [__u8; 3],
    pub ilid: __le32,
    pub timestamp1: __le32,
    pub timestamp2: __le32,
    pub data_len: __le32,
// C attribute field omitted
pub const PMCRAID_AEN_GROUP: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_hcam_ccn {
    pub header: pmcraid_hcam_hdr,
    pub cfg_entry: pmcraid_config_table_entry,
    pub cfg_entry_old: pmcraid_config_table_entry,
// C attribute field omitted
pub const PMCRAID_CCN_EXT_SIZE: c_int = 3944;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_hcam_ccn_ext {
    pub header: pmcraid_hcam_hdr,
    pub cfg_entry: pmcraid_config_table_entry_ext,
    pub cfg_entry_old: pmcraid_config_table_entry_ext,
    pub reserved: [__u8; PMCRAID_CCN_EXT_SIZE],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_hcam_ldn {
    pub header: pmcraid_hcam_hdr,
    pub error_log: pmcraid_hostrcb_error,
// C attribute field omitted
// pmcraid_hcam.op_code values
pub const HOSTRCB_TYPE_CCN: c_uint = 0xE1;
pub const HOSTRCB_TYPE_LDN: c_uint = 0xE2;
// pmcraid_hcam.notification_type values
pub const NOTIFICATION_TYPE_ENTRY_CHANGED: c_uint = 0x0;
pub const NOTIFICATION_TYPE_ENTRY_NEW: c_uint = 0x1;
pub const NOTIFICATION_TYPE_ENTRY_DELETED: c_uint = 0x2;
pub const NOTIFICATION_TYPE_STATE_CHANGE: c_uint = 0x3;
pub const NOTIFICATION_TYPE_ENTRY_STATECHANGED: c_uint = 0x4;
pub const NOTIFICATION_TYPE_ERROR_LOG: c_uint = 0x10;
pub const NOTIFICATION_TYPE_INFORMATION_LOG: c_uint = 0x11;

// pmcraid_hcam.flags values

// pmcraid_hcam.overlay_id values
pub const HOSTRCB_OVERLAY_ID_08: c_uint = 0x08;
pub const HOSTRCB_OVERLAY_ID_09: c_uint = 0x09;
pub const HOSTRCB_OVERLAY_ID_11: c_uint = 0x11;
pub const HOSTRCB_OVERLAY_ID_12: c_uint = 0x12;
pub const HOSTRCB_OVERLAY_ID_13: c_uint = 0x13;
pub const HOSTRCB_OVERLAY_ID_14: c_uint = 0x14;
pub const HOSTRCB_OVERLAY_ID_16: c_uint = 0x16;
pub const HOSTRCB_OVERLAY_ID_17: c_uint = 0x17;
pub const HOSTRCB_OVERLAY_ID_20: c_uint = 0x20;
pub const HOSTRCB_OVERLAY_ID_FF: c_uint = 0xFF;
// Implementation specific card details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_chip_details {
// hardware register offsets
    pub ioastatus: c_ulong,
    pub ioarrin: c_ulong,
    pub mailbox: c_ulong,
    pub global_intr_mask: c_ulong,
    pub ioa_host_intr: c_ulong,
    pub ioa_host_msix_intr: c_ulong,
    pub ioa_host_intr_clr: c_ulong,
    pub ioa_host_mask: c_ulong,
    pub ioa_host_mask_clr: c_ulong,
    pub host_ioa_intr: c_ulong,
    pub host_ioa_intr_clr: c_ulong,
// timeout used during transitional to operational state
    pub transop_timeout: c_ulong,
}

// IOA to HOST doorbells (interrupts)

// Host to IOA Doorbells

// Global interrupt mask register value
pub const GLOBAL_INTERRUPT_MASK: c_uint = 0x5ULL;

// control_block, associated with each of the commands contains IOARCB, IOADLs
// memory for IOASA. Additional 3 * 16 bytes are allocated in order to support
// additional request parameters (of max size 48) any command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_control_block {
    pub ioarcb: pmcraid_ioarcb,
    pub 3]: pmcraid_ioadl_desc ioadl[PMCRAID_IOADLS_EXTERNAL +,
    pub ioasa: pmcraid_ioasa,
// C attribute field omitted
// pmcraid_sglist - Scatter-gather list allocated for passthrough ioctls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_sglist {
    pub order: u32,
    pub num_sg: u32,
    pub num_dma_sg: u32,
    pub scatterlist: *mut scatterlist,
}

// page D0 inquiry data of focal point resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_inquiry_data {
    pub ph_dev_type: __u8,
    pub page_code: __u8,
    pub reserved1: __u8,
    pub add_page_len: __u8,
    pub length: __u8,
    pub reserved2: __u8,
    pub fw_version: __be16,
    pub reserved3: [__u8; 16],
}

pub const PMCRAID_TIMESTAMP_LEN: c_int = 12;
pub const PMCRAID_REQ_TM_STR_LEN: c_int = 6;
pub const PMCRAID_SCSI_SET_TIMESTAMP: c_uint = 0xA4;
pub const PMCRAID_SCSI_SERVICE_ACTION: c_uint = 0x0F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_timestamp_data {
    pub reserved1: [__u8; 4],
    pub /: *mut *mut __u8 timestamp[PMCRAID_REQ_TM_STR_LEN]; / current time value,
    pub reserved2: [__u8; 2],
}

// pmcraid_cmd - LLD representation of SCSI command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_cmd {
// Ptr and bus address of DMA.able control block for this command
    pub ioa_cb: *mut pmcraid_control_block,
    pub ioa_cb_bus_addr: dma_addr_t,
    pub dma_handle: dma_addr_t,
// pointer to mid layer structure of SCSI commands
    pub scsi_cmd: *mut scsi_cmnd,
    pub free_list: list_head,
    pub wait_for_completion: completion,
    pub /: *mut *mut timer_list timer; / needed for internal commands,
    pub /: *mut *mut u32 timeout; / current timeout value,
    pub /: *mut *mut u32 index; / index into the command list,
    pub /: *mut *mut u8 completion_req; / for handling internal commands,
    pub /: *mut *mut u8 release; / for handling completions,
    pub ): *mut *mut void (cmd_done) (struct pmcraid_cmd,
    pub drv_inst: *mut pmcraid_instance,
    pub /: *mut *mut *mut pmcraid_sglist sglist; / used for passthrough IOCTLs,
// scratch used
// during reset sequence
    pub time_left: c_ulong,
    pub res: *mut pmcraid_resource_entry,
    pub hrrq_index: c_int,
// used during IO command error handling. Sense buffer
// for REQUEST SENSE command if firmware is not sending
// auto sense data
//
    pub sense_buffer: *mut u8,
    pub sense_buffer_dma: dma_addr_t,
}

//
// Interrupt registers of IOA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_interrupts {
    pub ioa_host_interrupt_reg: *mut void __iomem,
    pub ioa_host_msix_interrupt_reg: *mut void __iomem,
    pub ioa_host_interrupt_clr_reg: *mut void __iomem,
    pub ioa_host_interrupt_mask_reg: *mut void __iomem,
    pub ioa_host_interrupt_mask_clr_reg: *mut void __iomem,
    pub global_interrupt_mask_reg: *mut void __iomem,
    pub host_ioa_interrupt_reg: *mut void __iomem,
    pub host_ioa_interrupt_clr_reg: *mut void __iomem,
}

// ISR parameters LLD allocates (one for each MSI-X if enabled) vectors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_isr_param {
    pub drv_inst: *mut pmcraid_instance,
    pub /: *mut *mut u8 hrrq_id; / hrrq entry index,
}

// AEN message header sent as part of event data to applications
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_aen_msg {
    pub hostno: u32,
    pub length: u32,
    pub reserved: [u8; 8],
    pub data: [u8; ],
}

// Controller state event message type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_state_msg {
    pub msg: pmcraid_aen_msg,
    pub ioa_state: u32,
}

pub const PMC_DEVICE_EVENT_RESET_START: c_uint = 0x11000000;
pub const PMC_DEVICE_EVENT_RESET_SUCCESS: c_uint = 0x11000001;
pub const PMC_DEVICE_EVENT_RESET_FAILED: c_uint = 0x11000002;
pub const PMC_DEVICE_EVENT_SHUTDOWN_START: c_uint = 0x11000003;
pub const PMC_DEVICE_EVENT_SHUTDOWN_SUCCESS: c_uint = 0x11000004;
pub const PMC_DEVICE_EVENT_SHUTDOWN_FAILED: c_uint = 0x11000005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_hostrcb {
    pub drv_inst: *mut pmcraid_instance,
    pub msg: *mut pmcraid_aen_msg,
    pub /: *mut *mut *mut pmcraid_hcam_hdr hcam; / pointer to hcam buffer,
    pub /: *mut *mut *mut pmcraid_cmd cmd; / pointer to command block used,
    pub /: *mut *mut dma_addr_t baddr; / system address of hcam buffer,
    pub /: *mut *mut atomic_t ignore; / process HCAM response ?,
}

//
// Per adapter structure maintained by LLD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_instance {
// Array of allowed-to-be-exposed resources, initialized from
// Configuration Table, later updated with CCNs
//
    pub res_entries: *mut pmcraid_resource_entry,
    pub /: *mut *mut list_head free_res_q; / res_entries lists for easy lookup,
    pub /: *mut *mut list_head used_res_q; / List of to be exposed resources,
    pub /: *mut *mut spinlock_t resource_lock; / spinlock to protect resource list,
    pub mapped_dma_addr: *mut void __iomem,
    pub /: *mut *mut *mut void __iomem ioa_status; / Iomapped IOA status register,
    pub /: *mut *mut *mut void __iomem mailbox; / Iomapped mailbox register,
    pub /: *mut *mut *mut void __iomem ioarrin; / IOmapped IOARR IN register,
    pub int_regs: pmcraid_interrupts,
    pub chip_cfg: *mut pmcraid_chip_details,
// HostRCBs needed for HCAM
    pub ldn: pmcraid_hostrcb,
    pub ccn: pmcraid_hostrcb,
    pub /: *mut *mut pmcraid_state_msg scn; / controller state change msg,
// Bus address of start of HRRQ
    pub hrrq_start_bus_addr: [dma_addr_t; PMCRAID_NUM_MSIX_VECTORS],
// Pointer to 1st entry of HRRQ
    pub hrrq_start: [*mut __le32; PMCRAID_NUM_MSIX_VECTORS],
// Pointer to last entry of HRRQ
    pub hrrq_end: [*mut __le32; PMCRAID_NUM_MSIX_VECTORS],
// Pointer to current pointer of hrrq
    pub hrrq_curr: [*mut __le32; PMCRAID_NUM_MSIX_VECTORS],
// Lock for HRRQ access
    pub hrrq_lock: [spinlock_t; PMCRAID_NUM_MSIX_VECTORS],
    pub inq_data: *mut pmcraid_inquiry_data,
    pub inq_data_baddr: dma_addr_t,
    pub timestamp_data: *mut pmcraid_timestamp_data,
    pub timestamp_data_baddr: dma_addr_t,
// size of configuration table entry, varies based on the firmware
    pub config_table_entry_size: u32,
// Expected toggle bit at host
    pub host_toggle_bit: [u8; PMCRAID_NUM_MSIX_VECTORS],
// Wait Q for  threads to wait for Reset IOA completion
    pub reset_wait_q: wait_queue_head_t,
    pub reset_cmd: *mut pmcraid_cmd,
// structures for supporting SIGIO based AEN.
    pub aen_queue: *mut fasync_struct,
    pub /: *mut *mut mutex aen_queue_lock; / lock for aen subscribers list,
    pub cdev: cdev,
    pub /: *mut *mut *mut Scsi_Host host; / mid layer interface structure handle,
    pub /: *mut *mut *mut pci_dev pdev; / PCI device structure handle,
// No of Reset IOA retries . IOA marked dead if threshold exceeds
    pub ioa_reset_attempts: u8,
pub const PMCRAID_RESET_ATTEMPTS: c_int = 3;
    pub /: *mut *mut u8 current_log_level; / default level for logging IOASC errors,
    pub /: *mut *mut u8 num_hrrq; / Number of interrupt vectors allocated,
    pub /: *mut *mut u8 interrupt_mode; / current interrupt mode legacy or msix,
    pub /: *mut *mut dev_t dev; / Major-Minor numbers for Char device,
// Used as ISR handler argument
    pub hrrq_vector: [pmcraid_isr_param; PMCRAID_NUM_MSIX_VECTORS],
// Message id as filled in last fired IOARCB, used to identify HRRQ
    pub last_message_id: core::sync::atomic::AtomicI32,
// configuration table
    pub cfg_table: *mut pmcraid_config_table,
    pub cfg_table_bus_addr: dma_addr_t,
// structures related to command blocks
    pub /: *mut *mut *mut kmem_cache cmd_cachep; / cache for cmd blocks,
    pub /: *mut *mut *mut dma_pool control_pool; / pool for control blocks,
    pub /: *mut *mut char cmd_pool_name[64]; / name of cmd cache,
    pub /: *mut *mut char ctl_pool_name[64]; / name of control cache,
    pub cmd_list: [*mut pmcraid_cmd; PMCRAID_MAX_CMD],
    pub free_cmd_pool: list_head,
    pub pending_cmd_pool: list_head,
    pub /: *mut *mut spinlock_t free_pool_lock; / free pool lock,
    pub /: *mut *mut spinlock_t pending_pool_lock; / pending pool lock,
// Tasklet to handle deferred processing
    pub isr_tasklet: [tasklet_struct; PMCRAID_NUM_MSIX_VECTORS],
// Work-queue (Shared) for deferred reset processing
    pub worker_q: work_struct,
// No of IO commands pending with FW
    pub outstanding_cmds: core::sync::atomic::AtomicI32,
// should add/delete resources to mid-layer now ?
    pub expose_resources: core::sync::atomic::AtomicI32,
    pub /: *mut *mut u32 ioa_state:4; / For IOA Reset sequence FSM,
pub const IOA_STATE_OPERATIONAL: c_uint = 0x0;
pub const IOA_STATE_UNKNOWN: c_uint = 0x1;
pub const IOA_STATE_DEAD: c_uint = 0x2;
pub const IOA_STATE_IN_SOFT_RESET: c_uint = 0x3;
pub const IOA_STATE_IN_HARD_RESET: c_uint = 0x4;
pub const IOA_STATE_IN_RESET_ALERT: c_uint = 0x5;
pub const IOA_STATE_IN_BRINGDOWN: c_uint = 0x6;
pub const IOA_STATE_IN_BRINGUP: c_uint = 0x7;
    pub /: *mut *mut u32 ioa_reset_in_progress:1; / true if IOA reset is in progress,
    pub /: *mut *mut u32 ioa_hard_reset:1; / TRUE if Hard Reset is needed,
    pub /: *mut *mut u32 ioa_unit_check:1; / Indicates Unit Check condition,
    pub /: *mut *mut u32 ioa_bringdown:1; / whether IOA needs to be brought down,
    pub /: *mut *mut u32 force_ioa_reset:1; / force adapter reset ?,
    pub /: *mut *mut u32 reinit_cfg_table:1; / reinit config table due to lost CCN,
    pub /: *mut *mut u32 ioa_shutdown_type:2;/ shutdown type used during reset,
pub const SHUTDOWN_NONE: c_uint = 0x0;
pub const SHUTDOWN_NORMAL: c_uint = 0x1;
pub const SHUTDOWN_ABBREV: c_uint = 0x2;
    pub /: *mut *mut u32 timestamp_error:1; / indicate set timestamp for out of sync,
}

// LLD maintained resource entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_resource_entry {
    pub /: *mut *mut list_head queue; / link to "to be exposed" resources,
    pub cfg_entry: pmcraid_config_table_entry,
    pub cfg_entry_ext: pmcraid_config_table_entry_ext,
}

// To indicate add/delete/modify during CCN
pub const RES_CHANGE_ADD: c_uint = 0x1	/* add this to mid-layer */;
pub const RES_CHANGE_DEL: c_uint = 0x2	/* remove this from mid-layer */;
//
// When IOA asks for sync (i.e. IOASC = Not Ready, Sync Required), this
// flag will be set, mid layer will be asked to retry. In the next
// attempt, this flag will be checked in queuecommand() to set
// SYNC_COMPLETE flag in IOARCB (flag_0).
//
// target indicates the mapped target_id assigned to this resource if
// this is VSET resource. For non-VSET resources this will be un-used
// or zero
//
// Data structures used in IOASC error code logging
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioasc_error {
    pub /: *mut *mut u32 ioasc_code; / IOASC code,
    pub /: *mut *mut u8 log_level; / default log level assignment.,
    pub error_string: *mut c_char,
}

// Initial log_level assignments for various IOASCs
pub const IOASC_LOG_LEVEL_NONE: c_uint = 0x0 /* no logging */;
pub const IOASC_LOG_LEVEL_MUST: c_uint = 0x1	/* must log: all high-severity errors */;
pub const IOASC_LOG_LEVEL_HARD: c_uint = 0x2	/* optional – low severity errors */;
// Error information maintained by LLD. LLD initializes the pmcraid_error_table
// statically.
//
// macros to help in debugging

// check if given command is a SCSI READ or SCSI WRITE command
pub const SCSI_READ_CMD: c_uint = 0x1	/* any of SCSI READ commands */;
pub const SCSI_WRITE_CMD: c_uint = 0x2	/* any of SCSI WRITE commands */;

//
// pmcraid_ioctl_header - definition of header structure that precedes all the
// buffers given as ioctl arguments.
//
// .signature           : always ASCII string, "PMCRAID"
// .reserved            : not used
// .buffer_length       : length of the buffer following the header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcraid_ioctl_header {
    pub signature: [u8; 8],
    pub reserved: u32,
    pub buffer_length: u32,
}

//
// keys to differentiate between driver handled IOCTLs and passthrough
// IOCTLs passed to IOA. driver determines the ioctl type using macro
// _IOC_TYPE
//

//
// _ARGSIZE: macro that gives size of the argument type passed to an IOCTL cmd.
// This is to facilitate applications avoiding un-necessary memory allocations.
// For example, most of driver handled ioctls do not require ioarcb, ioasa.
//

// Driver handled IOCTL command definitions

