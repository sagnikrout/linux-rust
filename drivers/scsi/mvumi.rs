//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mvumi.h
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
// Marvell UMI head file
//
// Copyright 2011 Marvell. <jyli@marvell.com>
//
pub const MAX_BASE_ADDRESS: c_int = 6;
pub const VER_MAJOR: c_int = 1;
pub const VER_MINOR: c_int = 1;
pub const VER_OEM: c_int = 0;
pub const VER_BUILD: c_int = 1500;

pub const PCI_DEVICE_ID_MARVELL_MV9143: c_uint = 0x9143;
pub const PCI_DEVICE_ID_MARVELL_MV9580: c_uint = 0x9580;
pub const MVUMI_INTERNAL_CMD_WAIT_TIME: c_int = 45;
pub const MVUMI_INQUIRY_LENGTH: c_int = 44;
pub const MVUMI_INQUIRY_UUID_OFF: c_int = 36;
pub const MVUMI_INQUIRY_UUID_LEN: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvumi_qc_result {
    MV_QUEUE_COMMAND_RESULT_SENT = 0,
    MV_QUEUE_COMMAND_RESULT_NO_RESOURCE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hw_regs {
// For CPU
    pub main_int_cause_reg: *mut c_void,
    pub enpointa_mask_reg: *mut c_void,
    pub enpointb_mask_reg: *mut c_void,
    pub rstoutn_en_reg: *mut c_void,
    pub ctrl_sts_reg: *mut c_void,
    pub rstoutn_mask_reg: *mut c_void,
    pub sys_soft_rst_reg: *mut c_void,
// For Doorbell
    pub pciea_to_arm_drbl_reg: *mut c_void,
    pub arm_to_pciea_drbl_reg: *mut c_void,
    pub arm_to_pciea_mask_reg: *mut c_void,
    pub pciea_to_arm_msg0: *mut c_void,
    pub pciea_to_arm_msg1: *mut c_void,
    pub arm_to_pciea_msg0: *mut c_void,
    pub arm_to_pciea_msg1: *mut c_void,
// reset register
    pub reset_request: *mut c_void,
    pub reset_enable: *mut c_void,
// For Message Unit
    pub inb_list_basel: *mut c_void,
    pub inb_list_baseh: *mut c_void,
    pub inb_aval_count_basel: *mut c_void,
    pub inb_aval_count_baseh: *mut c_void,
    pub inb_write_pointer: *mut c_void,
    pub inb_read_pointer: *mut c_void,
    pub outb_list_basel: *mut c_void,
    pub outb_list_baseh: *mut c_void,
    pub outb_copy_basel: *mut c_void,
    pub outb_copy_baseh: *mut c_void,
    pub outb_copy_pointer: *mut c_void,
    pub outb_read_pointer: *mut c_void,
    pub inb_isr_cause: *mut c_void,
    pub outb_isr_cause: *mut c_void,
    pub outb_coal_cfg: *mut c_void,
    pub outb_coal_timeout: *mut c_void,
// Bit setting for HW
    pub int_comaout: u32,
    pub int_comaerr: u32,
    pub int_dl_cpu2pciea: u32,
    pub int_mu: u32,
    pub int_drbl_int_mask: u32,
    pub int_main_int_mask: u32,
    pub cl_pointer_toggle: u32,
    pub cl_slot_num_mask: u32,
    pub clic_irq: u32,
    pub clic_in_err: u32,
    pub clic_out_err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_dyn_list_entry {
    pub src_low_addr: u32,
    pub src_high_addr: u32,
    pub if_length: u32,
    pub reserve: u32,
}

pub const SCSI_CMD_MARVELL_SPECIFIC: c_uint = 0xE1;
pub const CDB_CORE_MODULE: c_uint = 0x1;
pub const CDB_CORE_SHUTDOWN: c_uint = 0xB;
//
// Command flag is the flag for the CDB command itself
//
// 1-non data; 0-data command
// 1-host read data
// 1-host write data
pub const APICDB0_EVENT: c_uint = 0xF4;
pub const APICDB1_EVENT_GETEVENT: c_int = 0;
pub const APICDB1_HOST_GETEVENT: c_int = 1;
pub const MAX_EVENTS_RETURNED: c_int = 6;
pub const DEVICE_OFFLINE: c_int = 0;
pub const DEVICE_ONLINE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hotplug_event {
    pub size: u16,
    pub dummy: [u8; 2],
    pub bitmap: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_driver_event {
    pub time_stamp: u32,
    pub sequence_no: u32,
    pub event_id: u32,
    pub severity: u8,
    pub param_count: u8,
    pub device_id: u16,
    pub params: [u32; 4],
    pub sense_data_length: u8,
    pub Reserved1: u8,
    pub sense_data: [u8; 30],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_event_req {
    pub count: c_uchar,
    pub reserved: [c_uchar; 3],
    pub events: [mvumi_driver_event; MAX_EVENTS_RETURNED],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_events_wq {
    pub work_q: work_struct,
    pub mhba: *mut mvumi_hba,
    pub event: c_uint,
    pub param: *mut c_void,
}

pub const MVUMI_MAX_SG_ENTRY: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_sgl {
    pub baseaddr_l: u32,
    pub baseaddr_h: u32,
    pub flags: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_compact_sgl {
    pub baseaddr_l: u32,
    pub baseaddr_h: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_res {
    pub entry: list_head,
    pub bus_addr: dma_addr_t,
    pub virt_addr: *mut c_void,
    pub size: c_uint,
    pub /: *mut *mut unsigned short type; / enum Resource_Type,
}

// Resource type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resource_type {
    RESOURCE_CACHED_MEMORY = 0,
    RESOURCE_UNCACHED_MEMORY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_sense_data {
    pub error_code:7: u8,
    pub valid:1: u8,
    pub segment_number: u8,
    pub sense_key:4: u8,
    pub reserved:1: u8,
    pub incorrect_length:1: u8,
    pub end_of_media:1: u8,
    pub file_mark:1: u8,
    pub information: [u8; 4],
    pub additional_sense_length: u8,
    pub command_specific_information: [u8; 4],
    pub additional_sense_code: u8,
    pub additional_sense_code_qualifier: u8,
    pub field_replaceable_unit_code: u8,
    pub sense_key_specific: [u8; 3],
}

// Request initiator must set the status to REQ_STATUS_PENDING.
pub const REQ_STATUS_PENDING: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_cmd {
    pub queue_pointer: list_head,
    pub frame: *mut mvumi_msg_frame,
    pub frame_phys: dma_addr_t,
    pub scmd: *mut scsi_cmnd,
    pub sync_cmd: core::sync::atomic::AtomicI32,
    pub data_buf: *mut c_void,
    pub request_id: c_ushort,
    pub cmd_status: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_cmd_priv {
    pub cmd_priv: *mut mvumi_cmd,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
//
// the function type of the in bound frame
//
pub const CL_FUN_SCSI_CMD: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_msg_frame {
    pub device_id: u16,
    pub tag: u16,
    pub cmd_flag: u8,
    pub req_function: u8,
    pub cdb_length: u8,
    pub sg_counts: u8,
    pub data_transfer_length: u32,
    pub request_id: u16,
    pub reserved1: u16,
    pub cdb: [u8; MAX_COMMAND_SIZE],
    pub payload: [u32; ],
}

//
// the respond flag for data_payload of the out bound frame
//
pub const CL_RSP_FLAG_NODATA: c_uint = 0x0;
pub const CL_RSP_FLAG_SENSEDATA: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_rsp_frame {
    pub device_id: u16,
    pub tag: u16,
    pub req_status: u8,
    pub Data_Payload.*/: *mut *mut u8 rsp_flag; / Indicates the type of,
    pub request_id: u16,
    pub payload: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_ob_data {
    pub list: list_head,
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct version_info {
    pub ver_major: u32,
    pub ver_minor: u32,
    pub ver_oem: u32,
    pub ver_build: u32,
}

pub const FW_MAX_DELAY: c_int = 30;

//
// State is the state of the MU
//
pub const FW_STATE_IDLE: c_int = 0;
pub const FW_STATE_STARTING: c_int = 1;
pub const FW_STATE_HANDSHAKING: c_int = 2;
pub const FW_STATE_STARTED: c_int = 3;
pub const FW_STATE_ABORT: c_int = 4;
pub const HANDSHAKE_SIGNATURE: c_uint = 0x5A5A5A5AL;
pub const HANDSHAKE_READYSTATE: c_uint = 0x55AA5AA5L;
pub const HANDSHAKE_DONESTATE: c_uint = 0x55AAA55AL;
// HandShake Status definition
pub const HS_STATUS_OK: c_int = 1;
pub const HS_STATUS_ERR: c_int = 2;
pub const HS_STATUS_INVALID: c_int = 3;
// HandShake State/Cmd definition
pub const HS_S_START: c_int = 1;
pub const HS_S_RESET: c_int = 2;
pub const HS_S_PAGE_ADDR: c_int = 3;
pub const HS_S_QUERY_PAGE: c_int = 4;
pub const HS_S_SEND_PAGE: c_int = 5;
pub const HS_S_END: c_int = 6;
pub const HS_S_ABORT: c_int = 7;
pub const HS_PAGE_VERIFY_SIZE: c_int = 128;

// handshake frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hs_frame {
    pub size: u16,
// host information
    pub host_type: u8,
    pub reserved_1: [u8; 1],
    pub /: *mut *mut version_info host_ver; / bios or driver version,
// controller information
    pub system_io_bus: u32,
    pub slot_number: u32,
    pub intr_level: u32,
    pub intr_vector: u32,
// communication list configuration
    pub ib_baseaddr_l: u32,
    pub ib_baseaddr_h: u32,
    pub ob_baseaddr_l: u32,
    pub ob_baseaddr_h: u32,
    pub ib_entry_size: u8,
    pub ob_entry_size: u8,
    pub ob_depth: u8,
    pub ib_depth: u8,
// system time
    pub seconds_since1970: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hs_header {
    pub page_code: u8,
    pub checksum: u8,
    pub frame_length: u16,
    pub frame_content: [u32; ],
}

//
// the page code type of the handshake header
//
pub const HS_PAGE_FIRM_CAP: c_uint = 0x1;
pub const HS_PAGE_HOST_INFO: c_uint = 0x2;
pub const HS_PAGE_FIRM_CTL: c_uint = 0x3;
pub const HS_PAGE_CL_INFO: c_uint = 0x4;
pub const HS_PAGE_TOTAL: c_uint = 0x5;

// The format of the page code for Firmware capability
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hs_page1 {
    pub pagecode: u8,
    pub checksum: u8,
    pub frame_length: u16,
    pub number_of_ports: u16,
    pub max_devices_support: u16,
    pub max_io_support: u16,
    pub umi_ver: u16,
    pub max_transfer_size: u32,
    pub fw_ver: version_info,
    pub cl_in_max_entry_size: u8,
    pub cl_out_max_entry_size: u8,
    pub cl_inout_list_depth: u8,
    pub total_pages: u8,
    pub capability: u16,
    pub reserved1: u16,
}

// The format of the page code for Host information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hs_page2 {
    pub pagecode: u8,
    pub checksum: u8,
    pub frame_length: u16,
    pub host_type: u8,
    pub host_cap: u8,
    pub reserved: [u8; 2],
    pub host_ver: version_info,
    pub system_io_bus: u32,
    pub slot_number: u32,
    pub intr_level: u32,
    pub intr_vector: u32,
    pub seconds_since1970: u64,
}

// The format of the page code for firmware control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hs_page3 {
    pub pagecode: u8,
    pub checksum: u8,
    pub frame_length: u16,
    pub control: u16,
    pub reserved: [u8; 2],
    pub host_bufferaddr_l: u32,
    pub host_bufferaddr_h: u32,
    pub host_eventaddr_l: u32,
    pub host_eventaddr_h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hs_page4 {
    pub pagecode: u8,
    pub checksum: u8,
    pub frame_length: u16,
    pub ib_baseaddr_l: u32,
    pub ib_baseaddr_h: u32,
    pub ob_baseaddr_l: u32,
    pub ob_baseaddr_h: u32,
    pub ib_entry_size: u8,
    pub ob_entry_size: u8,
    pub ob_depth: u8,
    pub ib_depth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_tag {
    pub stack: *mut c_ushort,
    pub top: c_ushort,
    pub size: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_device {
    pub list: list_head,
    pub sdev: *mut scsi_device,
    pub wwid: u64,
    pub dev_type: u8,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_hba {
    pub base_addr: [*mut c_void; MAX_BASE_ADDRESS],
    pub pci_base: [u32; MAX_BASE_ADDRESS],
    pub mmio: *mut c_void,
    pub cmd_pool: list_head,
    pub shost: *mut Scsi_Host,
    pub int_cmd_wait_q: wait_queue_head_t,
    pub pdev: *mut pci_dev,
    pub unique_id: c_uint,
    pub fw_outstanding: core::sync::atomic::AtomicI32,
    pub instancet: *mut mvumi_instance_template,
    pub ib_list: *mut c_void,
    pub ib_list_phys: dma_addr_t,
    pub ib_frame: *mut c_void,
    pub ib_frame_phys: dma_addr_t,
    pub ob_list: *mut c_void,
    pub ob_list_phys: dma_addr_t,
    pub ib_shadow: *mut c_void,
    pub ib_shadow_phys: dma_addr_t,
    pub ob_shadow: *mut c_void,
    pub ob_shadow_phys: dma_addr_t,
    pub handshake_page: *mut c_void,
    pub handshake_page_phys: dma_addr_t,
    pub global_isr: c_uint,
    pub isr_status: c_uint,
    pub max_sge: c_ushort,
    pub max_target_id: c_ushort,
    pub target_map: *mut c_uchar,
    pub max_io: c_uint,
    pub list_num_io: c_uint,
    pub ib_max_size: c_uint,
    pub ob_max_size: c_uint,
    pub ib_max_size_setting: c_uint,
    pub ob_max_size_setting: c_uint,
    pub max_transfer_size: c_uint,
    pub hba_total_pages: c_uchar,
    pub fw_flag: c_uchar,
    pub request_id_enabled: c_uchar,
    pub eot_flag: c_uchar,
    pub hba_capability: c_ushort,
    pub io_seq: c_ushort,
    pub ib_cur_slot: c_uint,
    pub ob_cur_slot: c_uint,
    pub fw_state: c_uint,
    pub sas_discovery_mutex: mutex,
    pub ob_data_list: list_head,
    pub free_ob_list: list_head,
    pub res_list: list_head,
    pub waiting_req_list: list_head,
    pub tag_pool: mvumi_tag,
    pub tag_cmd: *mut mvumi_cmd,
    pub regs: *mut mvumi_hw_regs,
    pub device_lock: mutex,
    pub mhba_dev_list: list_head,
    pub shost_dev_list: list_head,
    pub dm_thread: *mut task_struct,
    pub pnp_count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvumi_instance_template {
    pub ): *mut *mut *mut void (fire_cmd) (struct mvumi_hba , struct mvumi_cmd,
    pub ): *mut *mut void (enable_intr) (struct mvumi_hba,
    pub ): *mut *mut void (disable_intr) (struct mvumi_hba,
    pub ): *mut *mut int (clear_intr) (void,
    pub ): *mut *mut unsigned int (read_fw_status_reg) (struct mvumi_hba,
    pub ): *mut *mut unsigned int (check_ib_list) (struct mvumi_hba,
    pub ): *mut c_uint,
    pub ): *mut *mut int (reset_host) (struct mvumi_hba,
}
