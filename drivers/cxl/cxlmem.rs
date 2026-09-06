//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cxl/cxlmem.h
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
// Copyright(c) 2020-2021 Intel Corporation.

// CXL 2.0 8.2.8.5.1.1 Memory Device Status Register
pub const CXLMDEV_STATUS_OFFSET: c_uint = 0x0;

pub const CXLMDEV_MS_NOT_READY: c_int = 0;
pub const CXLMDEV_MS_READY: c_int = 1;
pub const CXLMDEV_MS_ERROR: c_int = 2;
pub const CXLMDEV_MS_DISABLED: c_int = 3;

pub const CXLMDEV_RESET_NEEDED_NOT: c_int = 0;
pub const CXLMDEV_RESET_NEEDED_COLD: c_int = 1;
pub const CXLMDEV_RESET_NEEDED_WARM: c_int = 2;
pub const CXLMDEV_RESET_NEEDED_HOT: c_int = 3;
pub const CXLMDEV_RESET_NEEDED_CXL: c_int = 4;

//
// struct cxl_memdev - CXL bus object representing a Type-3 Memory Device
// @dev: driver core device object
// @cdev: char dev core object for ioctl operations
// @cxlds: The device state backing this device
// @detach_work: active memdev lost a port in its ancestry
// @cxl_nvb: coordinate removal of @cxl_nvd if present
// @cxl_nvd: optional bridge to an nvdimm if the device supports pmem
// @endpoint: connection to the CXL port topology for this memory device
// @attach: creator of this memdev depends on CXL link attach to operate
// @id: id number of this memdev instance.
// @depth: endpoint port depth
// @scrub_cycle: current scrub cycle set for this device
// @scrub_region_id: id number of a backed region (if any) for which current scrub cycle set
// @err_rec_array: List of xarrarys to store the memdev error records to
// check attributes for a memory repair operation are from
// current boot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev {
    pub dev: device,
    pub cdev: cdev,
    pub cxlds: *mut cxl_dev_state,
    pub detach_work: work_struct,
    pub cxl_nvb: *mut cxl_nvdimm_bridge,
    pub cxl_nvd: *mut cxl_nvdimm,
    pub endpoint: *mut cxl_port,
    pub attach: *const cxl_memdev_attach,
    pub id: c_int,
    pub depth: c_int,
    pub scrub_cycle: u8,
    pub scrub_region_id: c_int,
    pub err_rec_array: *mut cxl_mem_err_rec,
}

extern "C" {
    pub fn container_of(_arg: dev, cxl_memdev: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn to_cxl_port(_arg: cxled->cxld.dev.parent) -> return;
}
extern "C" {
    pub fn to_cxl_port(_arg: cxlrd->cxlsd.cxld.dev.parent) -> return;
}
extern "C" {
    pub fn to_cxl_memdev(_arg: port->uport_dev) -> return;
}
extern "C" {
    pub fn is_cxl_memdev(dev: *const device) -> bool;
}
extern "C" {
    pub fn is_cxl_memdev(_arg: port->uport_dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_attach {
    pub cxlmd): *mut *mut int (probe)(struct cxl_memdev,
}

//
// struct cxl_attach_region - coordinate mapping a region at memdev registration
// @attach: common core attachment descriptor
// @hpa_range: physical address range of the region
//
// For the common simple case of a CXL device with private (non-general purpose
// / "accelerator") memory, enumerate firmware instantiated region, or
// instantiate a region for the device's capacity. Destroy the region on detach.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_attach_region {
    pub attach: cxl_memdev_attach,
    pub hpa_range: range,
}

extern "C" {
    pub fn cxl_memdev_attach_region(cxlmd: *mut cxl_memdev) -> c_int;
}

extern "C" {
    pub fn devm_cxl_setup_fw_upload(host: *mut device, mds: *mut cxl_memdev_state) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dpa_info {
    pub size: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dpa_part_info {
    pub range: range,
    pub mode: cxl_partition_mode,
    pub part: [}; CXL_NR_PARTITIONS_MAX],
    pub nr_partitions: c_int,
}

extern "C" {
    pub fn cxl_dpa_setup(cxlds: *mut cxl_dev_state, info: *const cxl_dpa_info) -> c_int;
}
extern "C" {
    pub fn xa_load(_arg: &port->endpoints, long)&cxlmd->dev: (unsigned) -> return;
}
//
// Per CXL 3.0 Section 8.2.8.4.5.1
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_cmd_rc {
    pub err: c_int,
    pub desc: *const c_char,
}

//
// CXL 2.0 - Memory capacity multiplier
// See Section 8.2.9.5
//
// Volatile, Persistent, and Partition capacities are specified to be in
// multiples of 256MB - define a multiplier to convert to/from bytes.
//

//
// Event Interrupt Policy
//
// CXL rev 3.0 section 8.2.9.2.4; Table 8-52
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_event_int_mode {
    CXL_INT_NONE		= 0x00,
    CXL_INT_MSI_MSIX	= 0x01,
    CXL_INT_FW		= 0x02
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_interrupt_policy {
    pub info_settings: u8,
    pub warn_settings: u8,
    pub failure_settings: u8,
    pub fatal_settings: u8,
    pub __packed: },
//
// struct cxl_event_state - Event log driver state
//
// @buf: Buffer to receive event data
// @log_lock: Serialize event_buf and log use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_state {
    pub buf: *mut cxl_get_event_payload,
    pub log_lock: mutex,
}

// Device enabled poison commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum poison_cmd_enabled_bits {
    CXL_POISON_ENABLED_LIST,
    CXL_POISON_ENABLED_INJECT,
    CXL_POISON_ENABLED_CLEAR,
    CXL_POISON_ENABLED_SCAN_CAPS,
    CXL_POISON_ENABLED_SCAN_MEDIA,
    CXL_POISON_ENABLED_SCAN_RESULTS,
    CXL_POISON_ENABLED_MAX
}

// Device enabled security commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum security_cmd_enabled_bits {
    CXL_SEC_ENABLED_SANITIZE,
    CXL_SEC_ENABLED_SECURE_ERASE,
    CXL_SEC_ENABLED_GET_SECURITY_STATE,
    CXL_SEC_ENABLED_SET_PASSPHRASE,
    CXL_SEC_ENABLED_DISABLE_PASSPHRASE,
    CXL_SEC_ENABLED_UNLOCK,
    CXL_SEC_ENABLED_FREEZE_SECURITY,
    CXL_SEC_ENABLED_PASSPHRASE_SECURE_ERASE,
    CXL_SEC_ENABLED_MAX
}

//
// struct cxl_poison_state - Driver poison state info
//
// @max_errors: Maximum media error records held in device cache
// @enabled_cmds: All poison commands enabled in the CEL
// @list_out: The poison list payload returned by device
// @mutex: Protect reads of the poison list
//
// Reads of the poison list are synchronized to ensure that a reader
// does not get an incomplete list because their request overlapped
// (was interrupted or preceded by) another read request of the same
// DPA range. CXL Spec 3.0 Section 8.2.9.8.4.1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_poison_state {
    pub max_errors: u32,
    pub CXL_POISON_ENABLED_MAX): DECLARE_BITMAP(enabled_cmds,,
    pub list_out: *mut cxl_mbox_poison_out,
    pub /: *mut *mut mutex mutex; / Protect reads of poison list,
}

//
// Get FW Info
// CXL rev 3.0 section 8.2.9.3.1; Table 8-56
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_fw_info {
    pub num_slots: u8,
    pub slot_info: u8,
    pub activation_cap: u8,
    pub reserved: [u8; 13],
    pub slot_1_revision: [c_char; 16],
    pub slot_2_revision: [c_char; 16],
    pub slot_3_revision: [c_char; 16],
    pub slot_4_revision: [c_char; 16],
    pub __packed: },

pub const CXL_FW_INFO_SLOT_INFO_NEXT_SHIFT: c_int = 3;

//
// Transfer FW Input Payload
// CXL rev 3.0 section 8.2.9.3.2; Table 8-57
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_transfer_fw {
    pub action: u8,
    pub slot: u8,
    pub reserved: [u8; 2],
    pub offset: __le32,
    pub reserved2: [u8; 0x78],
    pub data: [u8; ],
    pub __packed: },
pub const CXL_FW_TRANSFER_ACTION_FULL: c_uint = 0x0;
pub const CXL_FW_TRANSFER_ACTION_INITIATE: c_uint = 0x1;
pub const CXL_FW_TRANSFER_ACTION_CONTINUE: c_uint = 0x2;
pub const CXL_FW_TRANSFER_ACTION_END: c_uint = 0x3;
pub const CXL_FW_TRANSFER_ACTION_ABORT: c_uint = 0x4;
//
// CXL rev 3.0 section 8.2.9.3.2 mandates 128-byte alignment for FW packages
// and for each part transferred in a Transfer FW command.
//
pub const CXL_FW_TRANSFER_ALIGNMENT: c_int = 128;
//
// Activate FW Input Payload
// CXL rev 3.0 section 8.2.9.3.3; Table 8-58
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_activate_fw {
    pub action: u8,
    pub slot: u8,
    pub __packed: },
pub const CXL_FW_ACTIVATE_ONLINE: c_uint = 0x0;
pub const CXL_FW_ACTIVATE_OFFLINE: c_uint = 0x1;
// FW state bits
pub const CXL_FW_STATE_BITS: c_int = 32;
pub const CXL_FW_CANCEL: c_int = 0;
//
// struct cxl_fw_state - Firmware upload / activation state
//
// @state: fw_uploader state bitmask
// @oneshot: whether the fw upload fits in a single transfer
// @num_slots: Number of FW slots available
// @cur_slot: Slot number currently active
// @next_slot: Slot number for the new firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_fw_state {
    pub CXL_FW_STATE_BITS): DECLARE_BITMAP(state,,
    pub oneshot: bool,
    pub num_slots: c_int,
    pub cur_slot: c_int,
    pub next_slot: c_int,
}

//
// struct cxl_security_state - Device security state
//
// @state: state of last security operation
// @enabled_cmds: All security commands enabled in the CEL
// @poll_tmo_secs: polling timeout
// @sanitize_active: sanitize completion pending
// @poll_dwork: polling work item
// @sanitize_node: sanitation sysfs file to notify
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_security_state {
    pub state: c_ulong,
    pub CXL_SEC_ENABLED_MAX): DECLARE_BITMAP(enabled_cmds,,
    pub poll_tmo_secs: c_int,
    pub sanitize_active: bool,
    pub poll_dwork: delayed_work,
    pub sanitize_node: *mut kernfs_node,
}

//
// Static PMEM may be at partition index 0 when there is no static RAM
// capacity.
//
extern "C" {
    pub fn resource_size(_arg: &cxlds->part[i].res) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: cxl_mbox->host) -> return;
}
//
// struct cxl_memdev_state - Generic Type-3 Memory Device Class driver data
//
// CXL 8.1.12.1 PCI Header - Class Code Register Memory Device defines
// common memory device functionality like the presence of a mailbox and
// the functionality related to that like Identify Memory Device and Get
// Partition Info
// @cxlds: Core driver state common across Type-2 and Type-3 devices
// @lsa_size: Size of Label Storage Area
// (CXL 2.0 8.2.9.5.1.1 Identify Memory Device)
// @firmware_version: Firmware version for the memory device.
// @total_bytes: sum of all possible capacities
// @volatile_only_bytes: hard volatile capacity
// @persistent_only_bytes: hard persistent capacity
// @partition_align_bytes: alignment size for partition-able capacity
// @active_volatile_bytes: sum of hard + soft volatile
// @active_persistent_bytes: sum of hard + soft persistent
// @event: event log driver state
// @poison: poison driver state info
// @security: security driver state info
// @fw: firmware upload / activation state
//
// See CXL 3.0 8.2.9.8.2 Capacity Configuration and Label Storage for
// details on capacity parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_memdev_state {
    pub cxlds: cxl_dev_state,
    pub lsa_size: usize,
    pub firmware_version: [c_char; 0x10],
    pub total_bytes: u64,
    pub volatile_only_bytes: u64,
    pub persistent_only_bytes: u64,
    pub partition_align_bytes: u64,
    pub active_volatile_bytes: u64,
    pub active_persistent_bytes: u64,
    pub event: cxl_event_state,
    pub poison: cxl_poison_state,
    pub security: cxl_security_state,
    pub fw: cxl_fw_state,
}

extern "C" {
    pub fn container_of(_arg: cxlds, cxl_memdev_state: struct, _arg: cxlds) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_opcode {
    CXL_MBOX_OP_INVALID		= 0x0000,
    CXL_MBOX_OP_RAW			= CXL_MBOX_OP_INVALID,
    CXL_MBOX_OP_GET_EVENT_RECORD	= 0x0100,
    CXL_MBOX_OP_CLEAR_EVENT_RECORD	= 0x0101,
    CXL_MBOX_OP_GET_EVT_INT_POLICY	= 0x0102,
    CXL_MBOX_OP_SET_EVT_INT_POLICY	= 0x0103,
    CXL_MBOX_OP_GET_FW_INFO		= 0x0200,
    CXL_MBOX_OP_TRANSFER_FW		= 0x0201,
    CXL_MBOX_OP_ACTIVATE_FW		= 0x0202,
    CXL_MBOX_OP_GET_TIMESTAMP	= 0x0300,
    CXL_MBOX_OP_SET_TIMESTAMP	= 0x0301,
    CXL_MBOX_OP_GET_SUPPORTED_LOGS	= 0x0400,
    CXL_MBOX_OP_GET_LOG		= 0x0401,
    CXL_MBOX_OP_GET_LOG_CAPS	= 0x0402,
    CXL_MBOX_OP_CLEAR_LOG           = 0x0403,
    CXL_MBOX_OP_GET_SUP_LOG_SUBLIST = 0x0405,
    CXL_MBOX_OP_GET_SUPPORTED_FEATURES	= 0x0500,
    CXL_MBOX_OP_GET_FEATURE		= 0x0501,
    CXL_MBOX_OP_SET_FEATURE		= 0x0502,
    CXL_MBOX_OP_DO_MAINTENANCE	= 0x0600,
    CXL_MBOX_OP_IDENTIFY		= 0x4000,
    CXL_MBOX_OP_GET_PARTITION_INFO	= 0x4100,
    CXL_MBOX_OP_SET_PARTITION_INFO	= 0x4101,
    CXL_MBOX_OP_GET_LSA		= 0x4102,
    CXL_MBOX_OP_SET_LSA		= 0x4103,
    CXL_MBOX_OP_GET_HEALTH_INFO	= 0x4200,
    CXL_MBOX_OP_GET_ALERT_CONFIG	= 0x4201,
    CXL_MBOX_OP_SET_ALERT_CONFIG	= 0x4202,
    CXL_MBOX_OP_GET_SHUTDOWN_STATE	= 0x4203,
    CXL_MBOX_OP_SET_SHUTDOWN_STATE	= 0x4204,
    CXL_MBOX_OP_GET_POISON		= 0x4300,
    CXL_MBOX_OP_INJECT_POISON	= 0x4301,
    CXL_MBOX_OP_CLEAR_POISON	= 0x4302,
    CXL_MBOX_OP_GET_SCAN_MEDIA_CAPS	= 0x4303,
    CXL_MBOX_OP_SCAN_MEDIA		= 0x4304,
    CXL_MBOX_OP_GET_SCAN_MEDIA	= 0x4305,
    CXL_MBOX_OP_SANITIZE		= 0x4400,
    CXL_MBOX_OP_SECURE_ERASE	= 0x4401,
    CXL_MBOX_OP_GET_SECURITY_STATE	= 0x4500,
    CXL_MBOX_OP_SET_PASSPHRASE	= 0x4501,
    CXL_MBOX_OP_DISABLE_PASSPHRASE	= 0x4502,
    CXL_MBOX_OP_UNLOCK		= 0x4503,
    CXL_MBOX_OP_FREEZE_SECURITY	= 0x4504,
    CXL_MBOX_OP_PASSPHRASE_SECURE_ERASE	= 0x4505,
    CXL_MBOX_OP_MAX			= 0x10000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_supported_logs {
    pub entries: __le16,
    pub rsvd: [u8; 6],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_gsl_entry {
    pub uuid: uuid_t,
    pub size: __le32,
    pub entry: [} __packed; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cel_entry {
    pub opcode: __le16,
    pub effect: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_log {
    pub uuid: uuid_t,
    pub offset: __le32,
    pub length: __le32,
    pub __packed: },
// See CXL 2.0 Table 175 Identify Memory Device Output Payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_identify {
    pub fw_revision: [c_char; 0x10],
    pub total_capacity: __le64,
    pub volatile_capacity: __le64,
    pub persistent_capacity: __le64,
    pub partition_align: __le64,
    pub info_event_log_size: __le16,
    pub warning_event_log_size: __le16,
    pub failure_event_log_size: __le16,
    pub fatal_event_log_size: __le16,
    pub lsa_size: __le32,
    pub poison_list_max_mer: [u8; 3],
    pub inject_poison_limit: __le16,
    pub poison_caps: u8,
    pub qos_telemetry_caps: u8,
    pub __packed: },
//
// General Media Event Record UUID
// CXL rev 3.0 Section 8.2.9.2.1.1; Table 8-43
//

//
// DRAM Event Record UUID
// CXL rev 3.0 section 8.2.9.2.1.2; Table 8-44
//

//
// Memory Module Event Record UUID
// CXL rev 3.0 section 8.2.9.2.1.3; Table 8-45
//

//
// Memory Sparing Event Record UUID
// CXL rev 3.2 section 8.2.10.2.1.4: Table 8-60
//

//
// Get Event Records output payload
// CXL rev 3.0 section 8.2.9.2.2; Table 8-50
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_get_event_payload {
    pub flags: u8,
    pub reserved1: u8,
    pub overflow_err_count: __le16,
    pub first_overflow_timestamp: __le64,
    pub last_overflow_timestamp: __le64,
    pub record_count: __le16,
    pub reserved2: [u8; 10],
    pub records: [cxl_event_record_raw; ],
    pub __packed: },
//
// CXL rev 3.0 section 8.2.9.2.2; Table 8-49
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_event_log_type {
    CXL_EVENT_TYPE_INFO = 0x00,
    CXL_EVENT_TYPE_WARN,
    CXL_EVENT_TYPE_FAIL,
    CXL_EVENT_TYPE_FATAL,
    CXL_EVENT_TYPE_MAX
}

//
// Clear Event Records input payload
// CXL rev 3.0 section 8.2.9.2.3; Table 8-51
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_clear_event_payload {
    pub /: *mut *mut u8 event_log; / enum cxl_event_log_type,
    pub clear_flags: u8,
    pub nr_recs: u8,
    pub reserved: [u8; 3],
    pub handles: [__le16; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_partition_info {
    pub active_volatile_cap: __le64,
    pub active_persistent_cap: __le64,
    pub next_volatile_cap: __le64,
    pub next_persistent_cap: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_lsa {
    pub offset: __le32,
    pub length: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_set_lsa {
    pub offset: __le32,
    pub reserved: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_set_partition_info {
    pub volatile_capacity: __le64,
    pub flags: u8,
    pub __packed: },

// Get Health Info Output Payload CXL 3.2 Spec 8.2.10.9.3.1 Table 8-148
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_health_info_out {
    pub health_status: u8,
    pub media_status: u8,
    pub additional_status: u8,
    pub life_used: u8,
    pub device_temperature: __le16,
    pub dirty_shutdown_cnt: __le32,
    pub corrected_volatile_error_cnt: __le32,
    pub corrected_persistent_error_cnt: __le32,
    pub __packed: },
// Set Shutdown State Input Payload CXL 3.2 Spec 8.2.10.9.3.5 Table 8-152
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_set_shutdown_state_in {
    pub state: u8,
    pub __packed: },
// Set Timestamp CXL 3.0 Spec 8.2.9.4.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_set_timestamp_in {
    pub timestamp: __le64,
    pub __packed: },
// Get Poison List  CXL 3.0 Spec 8.2.9.8.4.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_poison_in {
    pub offset: __le64,
    pub length: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_poison_out {
    pub flags: u8,
    pub rsvd1: u8,
    pub overflow_ts: __le64,
    pub count: __le16,
    pub rsvd2: [u8; 20],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_poison_record {
    pub address: __le64,
    pub length: __le32,
    pub rsvd: __le32,
    pub record: [} __packed; ],
    pub __packed: },
//
// Get Poison List address field encodes the starting
// address of poison, and the source of the poison.
//

// Get Poison List record length is in units of 64 bytes
pub const CXL_POISON_LEN_MULT: c_int = 64;
// Kernel defined maximum for a list of poison errors
pub const CXL_POISON_LIST_MAX: c_int = 1024;
// Get Poison List: Payload out flags

// Get Poison List: Poison Source
pub const CXL_POISON_SOURCE_UNKNOWN: c_int = 0;
pub const CXL_POISON_SOURCE_EXTERNAL: c_int = 1;
pub const CXL_POISON_SOURCE_INTERNAL: c_int = 2;
pub const CXL_POISON_SOURCE_INJECTED: c_int = 3;
pub const CXL_POISON_SOURCE_VENDOR: c_int = 7;
// Inject & Clear Poison  CXL 3.0 Spec 8.2.9.8.4.2/3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_inject_poison {
    pub address: __le64,
}

// Clear Poison  CXL 3.0 Spec 8.2.9.8.4.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_clear_poison {
    pub address: __le64,
    pub write_data: [u8; CXL_POISON_LEN_MULT],
    pub __packed: },
//
// struct cxl_mem_command - Driver representation of a memory device command
// @info: Command information as it exists for the UAPI
// @opcode: The actual bits used for the mailbox protocol
// @flags: Set of flags effecting driver behavior.
//
// * %CXL_CMD_FLAG_FORCE_ENABLE: In cases of error, commands with this flag
// will be enabled by the driver regardless of what hardware may have
// advertised.
//
// The cxl_mem_command is the driver's internal representation of commands that
// are supported by the driver. Some of these commands may not be supported by
// the hardware. The driver will use @info to validate the fields passed in by
// the user then submit the @opcode to the hardware.
//
// See struct cxl_command_info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mem_command {
    pub info: cxl_command_info,
    pub opcode: cxl_opcode,
    pub flags: u32,

}

pub const CXL_PMEM_SEC_STATE_USER_PASS_SET: c_uint = 0x01;
pub const CXL_PMEM_SEC_STATE_MASTER_PASS_SET: c_uint = 0x02;
pub const CXL_PMEM_SEC_STATE_LOCKED: c_uint = 0x04;
pub const CXL_PMEM_SEC_STATE_FROZEN: c_uint = 0x08;
pub const CXL_PMEM_SEC_STATE_USER_PLIMIT: c_uint = 0x10;
pub const CXL_PMEM_SEC_STATE_MASTER_PLIMIT: c_uint = 0x20;
// set passphrase input payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_set_pass {
    pub type: u8,
    pub reserved: [u8; 31],
// CXL field using NVDIMM define, same length
    pub old_pass: [u8; NVDIMM_PASSPHRASE_LEN],
    pub new_pass: [u8; NVDIMM_PASSPHRASE_LEN],
    pub __packed: },
// disable passphrase input payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_disable_pass {
    pub type: u8,
    pub reserved: [u8; 31],
    pub pass: [u8; NVDIMM_PASSPHRASE_LEN],
    pub __packed: },
// passphrase secure erase payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_pass_erase {
    pub type: u8,
    pub reserved: [u8; 31],
    pub pass: [u8; NVDIMM_PASSPHRASE_LEN],
    pub __packed: },
}

extern "C" {
    pub fn cxl_dev_state_identify(mds: *mut cxl_memdev_state) -> c_int;
}
extern "C" {
    pub fn cxl_await_media_ready(cxlds: *mut cxl_dev_state) -> c_int;
}
extern "C" {
    pub fn cxl_enumerate_cmds(mds: *mut cxl_memdev_state) -> c_int;
}
extern "C" {
    pub fn cxl_mem_dpa_fetch(mds: *mut cxl_memdev_state, info: *mut cxl_dpa_info) -> c_int;
}
extern "C" {
    pub fn cxl_mem_get_event_records(mds: *mut cxl_memdev_state, status: u32);
}
extern "C" {
    pub fn cxl_get_dirty_count(mds: *mut cxl_memdev_state, count: *mut u32) -> c_int;
}
extern "C" {
    pub fn cxl_arm_dirty_shutdown(mds: *mut cxl_memdev_state) -> c_int;
}
extern "C" {
    pub fn cxl_set_timestamp(mds: *mut cxl_memdev_state) -> c_int;
}
extern "C" {
    pub fn cxl_poison_state_init(mds: *mut cxl_memdev_state) -> c_int;
}
extern "C" {
    pub fn cxl_trigger_poison_list(cxlmd: *mut cxl_memdev) -> c_int;
}
extern "C" {
    pub fn cxl_inject_poison(cxlmd: *mut cxl_memdev, dpa: u64) -> c_int;
}
extern "C" {
    pub fn cxl_clear_poison(cxlmd: *mut cxl_memdev, dpa: u64) -> c_int;
}
extern "C" {
    pub fn cxl_inject_poison_locked(cxlmd: *mut cxl_memdev, dpa: u64) -> c_int;
}
extern "C" {
    pub fn cxl_clear_poison_locked(cxlmd: *mut cxl_memdev, dpa: u64) -> c_int;
}

extern "C" {
    pub fn devm_cxl_memdev_edac_register(cxlmd: *mut cxl_memdev) -> c_int;
}
extern "C" {
    pub fn devm_cxl_region_edac_register(cxlr: *mut cxl_region) -> c_int;
}
extern "C" {
    pub fn cxl_store_rec_gen_media(cxlmd: *mut cxl_memdev, evt: *mut cxl_event) -> c_int;
}
extern "C" {
    pub fn cxl_store_rec_dram(cxlmd: *mut cxl_memdev, evt: *mut cxl_event) -> c_int;
}

extern "C" {
    pub fn cxl_mem_active_inc();
}
extern "C" {
    pub fn cxl_mem_active_dec();
}

extern "C" {
    pub fn cxl_mem_sanitize(cxlmd: *mut cxl_memdev, cmd: u16) -> c_int;
}
//
// struct cxl_hdm - HDM Decoder registers and cached / decoded capabilities
// @regs: mapped registers, see devm_cxl_setup_hdm()
// @decoder_count: number of decoders for this port
// @target_count: for switch decoders, max downstream port targets
// @interleave_mask: interleave granularity capability, see check_interleave_cap()
// @iw_cap_mask: bitmask of supported interleave ways, see check_interleave_cap()
// @port: mapped cxl_port, see devm_cxl_setup_hdm()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_hdm {
    pub regs: cxl_component_regs,
    pub decoder_count: c_int,
    pub target_count: c_uint,
    pub interleave_mask: c_uint,
    pub iw_cap_mask: c_ulong,
    pub port: *mut cxl_port,
}

extern "C" {
    pub fn cxl_dpa_debug(file: *mut seq_file, cxlds: *mut cxl_dev_state);
}
