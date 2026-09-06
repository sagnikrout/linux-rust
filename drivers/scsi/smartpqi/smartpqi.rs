//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/smartpqi/smartpqi.h
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
//
// driver for Microchip PQI-based storage controllers
// Copyright (c) 2019-2023 Microchip Technology Inc. and its subsidiaries
// Copyright (c) 2016-2018 Microsemi Corporation
// Copyright (c) 2016 PMC-Sierra, Inc.
//
// Questions/Comments/Bugfixes to storagedev@microchip.com
//

// This structure is defined by the PQI specification.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_device_registers {
    pub signature: __le64,
    pub function_and_status_code: u8,
    pub reserved: [u8; 7],
    pub max_admin_iq_elements: u8,
    pub max_admin_oq_elements: u8,
    pub /: *mut *mut u8 admin_iq_element_length; / in 16-byte units,
    pub /: *mut *mut u8 admin_oq_element_length; / in 16-byte units,
    pub /: *mut *mut __le16 max_reset_timeout; / in 100-millisecond units,
    pub reserved1: [u8; 2],
    pub legacy_intx_status: __le32,
    pub legacy_intx_mask_set: __le32,
    pub legacy_intx_mask_clear: __le32,
    pub reserved2: [u8; 28],
    pub device_status: __le32,
    pub reserved3: [u8; 4],
    pub admin_iq_pi_offset: __le64,
    pub admin_oq_ci_offset: __le64,
    pub admin_iq_element_array_addr: __le64,
    pub admin_oq_element_array_addr: __le64,
    pub admin_iq_ci_addr: __le64,
    pub admin_oq_pi_addr: __le64,
    pub admin_iq_num_elements: u8,
    pub admin_oq_num_elements: u8,
    pub admin_queue_int_msg_num: __le16,
    pub reserved4: [u8; 4],
    pub device_error: __le32,
    pub reserved5: [u8; 4],
    pub error_details: __le64,
    pub device_reset: __le32,
    pub power_action: __le32,
    pub reserved6: [u8; 104],
}

//
// controller registers
//
// These are defined by the Microchip implementation.
//
// Some registers (those named sis_*) are only used when in
// legacy SIS mode before we transition the controller into
// PQI mode.  There are a number of other SIS mode registers,
// but we don't use them, so only the SIS registers that we
// care about are defined here.  The offsets mentioned in the
// comments are the offsets from the PCIe BAR 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_ctrl_registers {
    pub reserved: [u8; 0x20],
    pub /: *mut *mut __le32 sis_host_to_ctrl_doorbell; / 20h,
    pub sizeof(__le32))]: u8 reserved1[0x34 - (0x20 +,
    pub /: *mut *mut __le32 sis_interrupt_mask; / 34h,
    pub sizeof(__le32))]: u8 reserved2[0x9c - (0x34 +,
    pub /: *mut *mut __le32 sis_ctrl_to_host_doorbell; / 9Ch,
    pub sizeof(__le32))]: u8 reserved3[0xa0 - (0x9c +,
    pub /: *mut *mut __le32 sis_ctrl_to_host_doorbell_clear; / A0h,
    pub sizeof(__le32))]: u8 reserved4[0xb0 - (0xa0 +,
    pub /: *mut *mut __le32 sis_driver_scratch; / B0h,
    pub /: *mut *mut __le32 sis_product_identifier; / B4h,
    pub sizeof(__le32))]: u8 reserved5[0xbc - (0xb4 +,
    pub /: *mut *mut __le32 sis_firmware_status; / BCh,
    pub sizeof(__le32))]: u8 reserved6[0xcc - (0xbc +,
    pub /: *mut *mut __le32 sis_ctrl_shutdown_reason_code; / CCh,
    pub sizeof(__le32))]: u8 reserved7[0x1000 - (0xcc +,
    pub /: *mut *mut __le32 sis_mailbox[8]; / 1000h,
    pub 8))]: *mut *mut u8 reserved8[0x4000 - (0x1000 + (sizeof(__le32),
//
// The PQI spec states that the PQI registers should be at
// offset 0 from the PCIe BAR 0.  However, we can't map
// them at offset 0 because that would break compatibility
// with the SIS registers.  So we map them at offset 4000h.
//
    pub /: *mut *mut pqi_device_registers pqi_registers; / 4000h,
}

pub const PQI_DEVICE_REGISTERS_OFFSET: c_uint = 0x4000;
// shutdown reasons for taking the controller offline
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pqi_ctrl_shutdown_reason {
    PQI_IQ_NOT_DRAINED_TIMEOUT = 1,
    PQI_LUN_RESET_TIMEOUT = 2,
    PQI_IO_PENDING_POST_LUN_RESET_TIMEOUT = 3,
    PQI_NO_HEARTBEAT = 4,
    PQI_FIRMWARE_KERNEL_NOT_UP = 5,
    PQI_OFA_RESPONSE_TIMEOUT = 6,
    PQI_INVALID_REQ_ID = 7,
    PQI_UNMATCHED_REQ_ID = 8,
    PQI_IO_PI_OUT_OF_RANGE = 9,
    PQI_EVENT_PI_OUT_OF_RANGE = 10,
    PQI_UNEXPECTED_IU_TYPE = 11
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pqi_io_path {
    RAID_PATH = 0,
    AIO_PATH = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pqi_irq_mode {
    IRQ_MODE_NONE,
    IRQ_MODE_INTX,
    IRQ_MODE_MSIX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_sg_descriptor {
    pub address: __le64,
    pub length: __le32,
    pub flags: __le32,
}

// manifest constants for the flags field of pqi_sg_descriptor
pub const CISS_SG_LAST: c_uint = 0x40000000;
pub const CISS_SG_CHAIN: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_iu_header {
    pub iu_type: u8,
    pub reserved: u8,
    pub /: *mut *mut __le16 iu_length; / in bytes - does not include the length,
// of this header
    pub /: *mut *mut __le16 response_queue_id; / specifies the OQ where the,
// response IU is to be delivered
    pub /: *mut *mut u16 driver_flags; / reserved for driver use,
}

// manifest constants for pqi_iu_header.driver_flags
pub const PQI_DRIVER_NONBLOCKABLE_REQUEST: c_uint = 0x1;
//
// According to the PQI spec, the IU header is only the first 4 bytes of our
// pqi_iu_header structure.
//
pub const PQI_REQUEST_HEADER_LENGTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_general_admin_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub function_code: u8,
    pub reserved: [u8; 33],
    pub buffer_length: __le32,
    pub sg_descriptor: pqi_sg_descriptor,
    pub report_device_capability: },
    pub reserved: u8,
    pub queue_id: __le16,
    pub reserved1: [u8; 2],
    pub element_array_addr: __le64,
    pub ci_addr: __le64,
    pub num_elements: __le16,
    pub element_length: __le16,
    pub queue_protocol: u8,
    pub reserved2: [u8; 23],
    pub vendor_specific: __le32,
    pub create_operational_iq: },
    pub reserved: u8,
    pub queue_id: __le16,
    pub reserved1: [u8; 2],
    pub element_array_addr: __le64,
    pub pi_addr: __le64,
    pub num_elements: __le16,
    pub element_length: __le16,
    pub queue_protocol: u8,
    pub reserved2: [u8; 3],
    pub int_msg_num: __le16,
    pub coalescing_count: __le16,
    pub min_coalescing_time: __le32,
    pub max_coalescing_time: __le32,
    pub reserved3: [u8; 8],
    pub vendor_specific: __le32,
    pub create_operational_oq: },
    pub reserved: u8,
    pub queue_id: __le16,
    pub reserved1: [u8; 50],
    pub delete_operational_queue: },
    pub reserved: u8,
    pub queue_id: __le16,
    pub reserved1: [u8; 46],
    pub vendor_specific: __le32,
    pub change_operational_iq_properties: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_general_admin_response {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub function_code: u8,
    pub status: u8,
    pub status_descriptor: [u8; 4],
    pub iq_pi_offset: __le64,
    pub reserved: [u8; 40],
    pub create_operational_iq: },
    pub status_descriptor: [u8; 4],
    pub oq_ci_offset: __le64,
    pub reserved: [u8; 40],
    pub create_operational_oq: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_iu_layer_descriptor {
    pub 1: u8 inbound_spanning_supported :,
    pub 7: u8 reserved :,
    pub reserved1: [u8; 5],
    pub max_inbound_iu_length: __le16,
    pub 1: u8 outbound_spanning_supported :,
    pub 7: u8 reserved2 :,
    pub reserved3: [u8; 5],
    pub max_outbound_iu_length: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_device_capability {
    pub data_length: __le16,
    pub reserved: [u8; 6],
    pub iq_arbitration_priority_support_bitmask: u8,
    pub maximum_aw_a: u8,
    pub maximum_aw_b: u8,
    pub maximum_aw_c: u8,
    pub 3: u8 max_arbitration_burst :,
    pub 4: u8 reserved1 :,
    pub 1: u8 iqa :,
    pub reserved2: [u8; 2],
    pub 1: u8 iq_freeze :,
    pub 7: u8 reserved3 :,
    pub max_inbound_queues: __le16,
    pub max_elements_per_iq: __le16,
    pub reserved4: [u8; 4],
    pub max_iq_element_length: __le16,
    pub min_iq_element_length: __le16,
    pub reserved5: [u8; 2],
    pub max_outbound_queues: __le16,
    pub max_elements_per_oq: __le16,
    pub intr_coalescing_time_granularity: __le16,
    pub max_oq_element_length: __le16,
    pub min_oq_element_length: __le16,
    pub reserved6: [u8; 24],
    pub iu_layer_descriptors: [pqi_iu_layer_descriptor; 32],
}

pub const PQI_MAX_EMBEDDED_SG_DESCRIPTORS: c_int = 4;
pub const PQI_MAX_EMBEDDED_R56_SG_DESCRIPTORS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_raid_path_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub nexus_id: __le16,
    pub buffer_length: __le32,
    pub lun_number: [u8; 8],
    pub protocol_specific: __le16,
    pub 2: u8 data_direction :,
    pub 1: u8 partial :,
    pub 4: u8 reserved1 :,
    pub 1: u8 fence :,
    pub error_index: __le16,
    pub reserved2: u8,
    pub 3: u8 task_attribute :,
    pub 4: u8 command_priority :,
    pub 1: u8 reserved3 :,
    pub 2: u8 reserved4 :,
    pub 3: u8 additional_cdb_bytes_usage :,
    pub 3: u8 reserved5 :,
    pub cdb: [u8; 16],
    pub reserved6: [u8; 11],
    pub ml_device_lun_number: u8,
    pub timeout: __le32,
    pub sg_descriptors: [pqi_sg_descriptor; PQI_MAX_EMBEDDED_SG_DESCRIPTORS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_aio_path_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub reserved1: [u8; 2],
    pub nexus_id: __le32,
    pub buffer_length: __le32,
    pub 2: u8 data_direction :,
    pub 1: u8 partial :,
    pub 1: u8 memory_type :,
    pub 1: u8 fence :,
    pub 1: u8 encryption_enable :,
    pub 2: u8 reserved2 :,
    pub 3: u8 task_attribute :,
    pub 4: u8 command_priority :,
    pub 1: u8 reserved3 :,
    pub data_encryption_key_index: __le16,
    pub encrypt_tweak_lower: __le32,
    pub encrypt_tweak_upper: __le32,
    pub cdb: [u8; 16],
    pub error_index: __le16,
    pub num_sg_descriptors: u8,
    pub cdb_length: u8,
    pub lun_number: [u8; 8],
    pub reserved4: [u8; 4],
    pub sg_descriptors: [pqi_sg_descriptor; PQI_MAX_EMBEDDED_SG_DESCRIPTORS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_aio_r1_path_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub /: *mut *mut __le16 volume_id; / ID of the RAID volume,
    pub /: *mut *mut __le32 it_nexus_1; / IT nexus of the 1st drive in the RAID volume,
    pub /: *mut *mut __le32 it_nexus_2; / IT nexus of the 2nd drive in the RAID volume,
    pub /: *mut *mut __le32 it_nexus_3; / IT nexus of the 3rd drive in the RAID volume,
    pub /: *mut *mut __le32 data_length; / total bytes to read/write,
    pub 2: u8 data_direction :,
    pub 1: u8 partial :,
    pub 1: u8 memory_type :,
    pub 1: u8 fence :,
    pub 1: u8 encryption_enable :,
    pub 2: u8 reserved :,
    pub 3: u8 task_attribute :,
    pub 4: u8 command_priority :,
    pub 1: u8 reserved2 :,
    pub data_encryption_key_index: __le16,
    pub cdb: [u8; 16],
    pub error_index: __le16,
    pub num_sg_descriptors: u8,
    pub cdb_length: u8,
    pub /: *mut *mut u8 num_drives; / number of drives in the RAID volume (2 or 3),
    pub reserved3: [u8; 3],
    pub encrypt_tweak_lower: __le32,
    pub encrypt_tweak_upper: __le32,
    pub sg_descriptors: [pqi_sg_descriptor; PQI_MAX_EMBEDDED_SG_DESCRIPTORS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_aio_r56_path_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub /: *mut *mut __le16 volume_id; / ID of the RAID volume,
    pub /: *mut *mut __le32 data_it_nexus; / IT nexus for the data drive,
    pub /: *mut *mut __le32 p_parity_it_nexus; / IT nexus for the P parity drive,
    pub /: *mut *mut __le32 q_parity_it_nexus; / IT nexus for the Q parity drive,
    pub /: *mut *mut __le32 data_length; / total bytes to read/write,
    pub 2: u8 data_direction :,
    pub 1: u8 partial :,
    pub /: *mut *mut u8 mem_type : 1; / 0 = PCIe, 1 = DDR,
    pub 1: u8 fence :,
    pub 1: u8 encryption_enable :,
    pub 2: u8 reserved :,
    pub 3: u8 task_attribute :,
    pub 4: u8 command_priority :,
    pub 1: u8 reserved1 :,
    pub data_encryption_key_index: __le16,
    pub cdb: [u8; 16],
    pub error_index: __le16,
    pub num_sg_descriptors: u8,
    pub cdb_length: u8,
    pub xor_multiplier: u8,
    pub reserved2: [u8; 3],
    pub encrypt_tweak_lower: __le32,
    pub encrypt_tweak_upper: __le32,
    pub /: *mut *mut __le64 row; / row = logical LBA/blocks per row,
    pub reserved3: [u8; 8],
    pub sg_descriptors: [pqi_sg_descriptor; PQI_MAX_EMBEDDED_R56_SG_DESCRIPTORS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_io_response {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub error_index: __le16,
    pub reserved2: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_general_management_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub reserved: [u8; 2],
    pub buffer_length: __le32,
    pub sg_descriptors: [pqi_sg_descriptor; 3],
    pub report_event_configuration: },
    pub global_event_oq_id: __le16,
    pub buffer_length: __le32,
    pub sg_descriptors: [pqi_sg_descriptor; 3],
    pub set_event_configuration: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_event_descriptor {
    pub event_type: u8,
    pub reserved: u8,
    pub oq_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_event_config {
    pub reserved: [u8; 2],
    pub num_event_descriptors: u8,
    pub reserved1: u8,
    pub descriptors: [pqi_event_descriptor; ],
}

pub const PQI_MAX_EVENT_DESCRIPTORS: c_int = 255;
pub const PQI_EVENT_OFA_MEMORY_ALLOCATION: c_uint = 0x0;
pub const PQI_EVENT_OFA_QUIESCE: c_uint = 0x1;
pub const PQI_EVENT_OFA_CANCELED: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_event_response {
    pub header: pqi_iu_header,
    pub event_type: u8,
    pub 7: u8 reserved2 :,
    pub 1: u8 request_acknowledge :,
    pub event_id: __le16,
    pub additional_event_id: __le32,
    pub bytes_requested: __le32,
    pub reserved: [u8; 12],
    pub ofa_memory_allocation: },
    pub /: *mut *mut __le16 reason; / reason for cancellation,
    pub reserved: [u8; 14],
    pub ofa_cancelled: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_event_acknowledge_request {
    pub header: pqi_iu_header,
    pub event_type: u8,
    pub reserved2: u8,
    pub event_id: __le16,
    pub additional_event_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_task_management_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub nexus_id: __le16,
    pub reserved: u8,
    pub ml_device_lun_number: u8,
    pub timeout: __le16,
    pub lun_number: [u8; 8],
    pub protocol_specific: __le16,
    pub outbound_queue_id_to_manage: __le16,
    pub request_id_to_manage: __le16,
    pub task_management_function: u8,
    pub 7: u8 reserved2 :,
    pub 1: u8 fence :,
}

pub const SOP_TASK_MANAGEMENT_LUN_RESET: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_task_management_response {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub nexus_id: __le16,
    pub additional_response_info: [u8; 3],
    pub response_code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_vendor_general_request {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub function_code: __le16,
    pub first_section: __le16,
    pub last_section: __le16,
    pub reserved: [u8; 48],
    pub config_table_update: },
    pub buffer_address: __le64,
    pub buffer_length: __le32,
    pub reserved: [u8; 40],
    pub host_memory_allocation: },
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_vendor_general_response {
    pub header: pqi_iu_header,
    pub request_id: __le16,
    pub function_code: __le16,
    pub status: __le16,
    pub reserved: [u8; 2],
}

pub const PQI_VENDOR_GENERAL_CONFIG_TABLE_UPDATE: c_int = 0;
pub const PQI_VENDOR_GENERAL_OFA_MEMORY_UPDATE: c_int = 1;
pub const PQI_VENDOR_GENERAL_CTRL_LOG_MEMORY_UPDATE: c_int = 2;
pub const PQI_OFA_VERSION: c_int = 1;

pub const PQI_CTRL_LOG_VERSION: c_int = 1;

pub const PQI_HOST_MAX_SG_DESCRIPTORS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_host_memory {
    pub /: *mut *mut __le64 signature; / "OFA_QRM", "FW_DATA", etc.,
    pub /: *mut *mut __le16 version; / version of this struct (1 = 1st version),
    pub reserved: [u8; 62],
    pub /: *mut *mut __le32 bytes_allocated; / total allocated memory in bytes,
    pub num_memory_descriptors: __le16,
    pub reserved1: [u8; 2],
    pub sg_descriptor: [pqi_sg_descriptor; PQI_HOST_MAX_SG_DESCRIPTORS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_host_memory_descriptor {
    pub host_memory: *mut pqi_host_memory,
    pub host_memory_dma_handle: dma_addr_t,
    pub host_chunk_virt_address: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_aio_error_info {
    pub status: u8,
    pub service_response: u8,
    pub data_present: u8,
    pub reserved: u8,
    pub residual_count: __le32,
    pub data_length: __le16,
    pub reserved1: __le16,
    pub data: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_raid_error_info {
    pub data_in_result: u8,
    pub data_out_result: u8,
    pub reserved: [u8; 3],
    pub status: u8,
    pub status_qualifier: __le16,
    pub sense_data_length: __le16,
    pub response_data_length: __le16,
    pub data_in_transferred: __le32,
    pub data_out_transferred: __le32,
    pub data: [u8; 256],
}

pub const PQI_REQUEST_IU_TASK_MANAGEMENT: c_uint = 0x13;
pub const PQI_REQUEST_IU_RAID_PATH_IO: c_uint = 0x14;
pub const PQI_REQUEST_IU_AIO_PATH_IO: c_uint = 0x15;
pub const PQI_REQUEST_IU_AIO_PATH_RAID5_IO: c_uint = 0x18;
pub const PQI_REQUEST_IU_AIO_PATH_RAID6_IO: c_uint = 0x19;
pub const PQI_REQUEST_IU_AIO_PATH_RAID1_IO: c_uint = 0x1A;
pub const PQI_REQUEST_IU_GENERAL_ADMIN: c_uint = 0x60;
pub const PQI_REQUEST_IU_REPORT_VENDOR_EVENT_CONFIG: c_uint = 0x72;
pub const PQI_REQUEST_IU_SET_VENDOR_EVENT_CONFIG: c_uint = 0x73;
pub const PQI_REQUEST_IU_VENDOR_GENERAL: c_uint = 0x75;
pub const PQI_REQUEST_IU_ACKNOWLEDGE_VENDOR_EVENT: c_uint = 0xf6;
pub const PQI_RESPONSE_IU_GENERAL_MANAGEMENT: c_uint = 0x81;
pub const PQI_RESPONSE_IU_TASK_MANAGEMENT: c_uint = 0x93;
pub const PQI_RESPONSE_IU_GENERAL_ADMIN: c_uint = 0xe0;
pub const PQI_RESPONSE_IU_RAID_PATH_IO_SUCCESS: c_uint = 0xf0;
pub const PQI_RESPONSE_IU_AIO_PATH_IO_SUCCESS: c_uint = 0xf1;
pub const PQI_RESPONSE_IU_RAID_PATH_IO_ERROR: c_uint = 0xf2;
pub const PQI_RESPONSE_IU_AIO_PATH_IO_ERROR: c_uint = 0xf3;
pub const PQI_RESPONSE_IU_AIO_PATH_DISABLED: c_uint = 0xf4;
pub const PQI_RESPONSE_IU_VENDOR_EVENT: c_uint = 0xf5;
pub const PQI_RESPONSE_IU_VENDOR_GENERAL: c_uint = 0xf7;
pub const PQI_GENERAL_ADMIN_FUNCTION_REPORT_DEVICE_CAPABILITY: c_uint = 0x0;
pub const PQI_GENERAL_ADMIN_FUNCTION_CREATE_IQ: c_uint = 0x10;
pub const PQI_GENERAL_ADMIN_FUNCTION_CREATE_OQ: c_uint = 0x11;
pub const PQI_GENERAL_ADMIN_FUNCTION_DELETE_IQ: c_uint = 0x12;
pub const PQI_GENERAL_ADMIN_FUNCTION_DELETE_OQ: c_uint = 0x13;
pub const PQI_GENERAL_ADMIN_FUNCTION_CHANGE_IQ_PROPERTY: c_uint = 0x14;
pub const PQI_GENERAL_ADMIN_STATUS_SUCCESS: c_uint = 0x0;
pub const PQI_IQ_PROPERTY_IS_AIO_QUEUE: c_uint = 0x1;
pub const PQI_GENERAL_ADMIN_IU_LENGTH: c_uint = 0x3c;
pub const PQI_PROTOCOL_SOP: c_uint = 0x0;
pub const PQI_DATA_IN_OUT_GOOD: c_uint = 0x0;
pub const PQI_DATA_IN_OUT_UNDERFLOW: c_uint = 0x1;
pub const PQI_DATA_IN_OUT_BUFFER_ERROR: c_uint = 0x40;
pub const PQI_DATA_IN_OUT_BUFFER_OVERFLOW: c_uint = 0x41;
pub const PQI_DATA_IN_OUT_BUFFER_OVERFLOW_DESCRIPTOR_AREA: c_uint = 0x42;
pub const PQI_DATA_IN_OUT_BUFFER_OVERFLOW_BRIDGE: c_uint = 0x43;
pub const PQI_DATA_IN_OUT_PCIE_FABRIC_ERROR: c_uint = 0x60;
pub const PQI_DATA_IN_OUT_PCIE_COMPLETION_TIMEOUT: c_uint = 0x61;
pub const PQI_DATA_IN_OUT_PCIE_COMPLETER_ABORT_RECEIVED: c_uint = 0x62;
pub const PQI_DATA_IN_OUT_PCIE_UNSUPPORTED_REQUEST_RECEIVED: c_uint = 0x63;
pub const PQI_DATA_IN_OUT_PCIE_ECRC_CHECK_FAILED: c_uint = 0x64;
pub const PQI_DATA_IN_OUT_PCIE_UNSUPPORTED_REQUEST: c_uint = 0x65;
pub const PQI_DATA_IN_OUT_PCIE_ACS_VIOLATION: c_uint = 0x66;
pub const PQI_DATA_IN_OUT_PCIE_TLP_PREFIX_BLOCKED: c_uint = 0x67;
pub const PQI_DATA_IN_OUT_PCIE_POISONED_MEMORY_READ: c_uint = 0x6F;
pub const PQI_DATA_IN_OUT_ERROR: c_uint = 0xf0;
pub const PQI_DATA_IN_OUT_PROTOCOL_ERROR: c_uint = 0xf1;
pub const PQI_DATA_IN_OUT_HARDWARE_ERROR: c_uint = 0xf2;
pub const PQI_DATA_IN_OUT_UNSOLICITED_ABORT: c_uint = 0xf3;
pub const PQI_DATA_IN_OUT_ABORTED: c_uint = 0xf4;
pub const PQI_DATA_IN_OUT_TIMEOUT: c_uint = 0xf5;
pub const CISS_CMD_STATUS_SUCCESS: c_uint = 0x0;
pub const CISS_CMD_STATUS_TARGET_STATUS: c_uint = 0x1;
pub const CISS_CMD_STATUS_DATA_UNDERRUN: c_uint = 0x2;
pub const CISS_CMD_STATUS_DATA_OVERRUN: c_uint = 0x3;
pub const CISS_CMD_STATUS_INVALID: c_uint = 0x4;
pub const CISS_CMD_STATUS_PROTOCOL_ERROR: c_uint = 0x5;
pub const CISS_CMD_STATUS_HARDWARE_ERROR: c_uint = 0x6;
pub const CISS_CMD_STATUS_CONNECTION_LOST: c_uint = 0x7;
pub const CISS_CMD_STATUS_ABORTED: c_uint = 0x8;
pub const CISS_CMD_STATUS_ABORT_FAILED: c_uint = 0x9;
pub const CISS_CMD_STATUS_UNSOLICITED_ABORT: c_uint = 0xa;
pub const CISS_CMD_STATUS_TIMEOUT: c_uint = 0xb;
pub const CISS_CMD_STATUS_UNABORTABLE: c_uint = 0xc;
pub const CISS_CMD_STATUS_TMF: c_uint = 0xd;
pub const CISS_CMD_STATUS_AIO_DISABLED: c_uint = 0xe;

pub const PQI_NUM_EVENT_QUEUE_ELEMENTS: c_int = 32;

pub const PQI_EVENT_TYPE_HOTPLUG: c_uint = 0x1;
pub const PQI_EVENT_TYPE_HARDWARE: c_uint = 0x2;
pub const PQI_EVENT_TYPE_PHYSICAL_DEVICE: c_uint = 0x4;
pub const PQI_EVENT_TYPE_LOGICAL_DEVICE: c_uint = 0x5;
pub const PQI_EVENT_TYPE_OFA: c_uint = 0xfb;
pub const PQI_EVENT_TYPE_AIO_STATE_CHANGE: c_uint = 0xfd;
pub const PQI_EVENT_TYPE_AIO_CONFIG_CHANGE: c_uint = 0xfe;

// these values are based on our implementation
pub const PQI_ADMIN_IQ_NUM_ELEMENTS: c_int = 8;
pub const PQI_ADMIN_OQ_NUM_ELEMENTS: c_int = 20;
pub const PQI_ADMIN_IQ_ELEMENT_LENGTH: c_int = 64;
pub const PQI_ADMIN_OQ_ELEMENT_LENGTH: c_int = 64;
pub const PQI_OPERATIONAL_IQ_ELEMENT_LENGTH: c_int = 128;
pub const PQI_OPERATIONAL_OQ_ELEMENT_LENGTH: c_int = 16;
pub const PQI_MIN_MSIX_VECTORS: c_int = 1;
pub const PQI_MAX_MSIX_VECTORS: c_int = 64;
// these values are defined by the PQI spec
pub const PQI_MAX_NUM_ELEMENTS_ADMIN_QUEUE: c_int = 255;
pub const PQI_MAX_NUM_ELEMENTS_OPERATIONAL_QUEUE: c_int = 65535;
pub const PQI_QUEUE_ELEMENT_ARRAY_ALIGNMENT: c_int = 64;
pub const PQI_QUEUE_ELEMENT_LENGTH_ALIGNMENT: c_int = 16;
pub const PQI_ADMIN_INDEX_ALIGNMENT: c_int = 64;
pub const PQI_OPERATIONAL_INDEX_ALIGNMENT: c_int = 4;
pub const PQI_MIN_OPERATIONAL_QUEUE_ID: c_int = 1;
pub const PQI_MAX_OPERATIONAL_QUEUE_ID: c_int = 65535;
pub const PQI_AIO_SERV_RESPONSE_COMPLETE: c_int = 0;
pub const PQI_AIO_SERV_RESPONSE_FAILURE: c_int = 1;
pub const PQI_AIO_SERV_RESPONSE_TMF_COMPLETE: c_int = 2;
pub const PQI_AIO_SERV_RESPONSE_TMF_SUCCEEDED: c_int = 3;
pub const PQI_AIO_SERV_RESPONSE_TMF_REJECTED: c_int = 4;
pub const PQI_AIO_SERV_RESPONSE_TMF_INCORRECT_LUN: c_int = 5;
pub const PQI_AIO_STATUS_IO_ERROR: c_uint = 0x1;
pub const PQI_AIO_STATUS_IO_ABORTED: c_uint = 0x2;
pub const PQI_AIO_STATUS_NO_PATH_TO_DEVICE: c_uint = 0x3;
pub const PQI_AIO_STATUS_INVALID_DEVICE: c_uint = 0x4;
pub const PQI_AIO_STATUS_AIO_PATH_DISABLED: c_uint = 0xe;
pub const PQI_AIO_STATUS_UNDERRUN: c_uint = 0x51;
pub const PQI_AIO_STATUS_OVERRUN: c_uint = 0x75;
pub type pqi_index_t = u32;
// SOP data direction flags
pub const SOP_NO_DIRECTION_FLAG: c_int = 0;

// buffer

// buffer

// Data-Out buffer and data is
// transferred to the Data-In buffer
pub const SOP_TASK_ATTRIBUTE_SIMPLE: c_int = 0;
pub const SOP_TASK_ATTRIBUTE_HEAD_OF_QUEUE: c_int = 1;
pub const SOP_TASK_ATTRIBUTE_ORDERED: c_int = 2;
pub const SOP_TASK_ATTRIBUTE_ACA: c_int = 4;
pub const SOP_TMF_COMPLETE: c_uint = 0x0;
pub const SOP_TMF_REJECTED: c_uint = 0x4;
pub const SOP_TMF_FUNCTION_SUCCEEDED: c_uint = 0x8;
pub const SOP_TMF_INCORRECT_LOGICAL_UNIT: c_uint = 0x9;
// additional CDB bytes usage field codes

//
// The purpose of this structure is to obtain proper alignment of objects in
// an admin queue pair.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_admin_queues_aligned {
    pub iq_ci: __aligned(PQI_ADMIN_INDEX_ALIGNMENT) pqi_index_t,
    pub oq_pi: __aligned(PQI_ADMIN_INDEX_ALIGNMENT) pqi_index_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_admin_queues {
    pub iq_element_array: *mut c_void,
    pub oq_element_array: *mut c_void,
    pub iq_ci: *mut pqi_index_t __iomem,
    pub oq_pi: *mut pqi_index_t __iomem,
    pub iq_element_array_bus_addr: dma_addr_t,
    pub oq_element_array_bus_addr: dma_addr_t,
    pub iq_ci_bus_addr: dma_addr_t,
    pub oq_pi_bus_addr: dma_addr_t,
    pub iq_pi: *mut __le32 __iomem,
    pub iq_pi_copy: pqi_index_t,
    pub oq_ci: *mut __le32 __iomem,
    pub oq_ci_copy: pqi_index_t,
    pub task: *mut task_struct,
    pub int_msg_num: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_queue_group {
    pub /: *mut *mut *mut pqi_ctrl_info ctrl_info; / backpointer,
    pub iq_id: [u16; 2],
    pub oq_id: u16,
    pub int_msg_num: u16,
    pub iq_element_array: [*mut c_void; 2],
    pub oq_element_array: *mut c_void,
    pub iq_element_array_bus_addr: [dma_addr_t; 2],
    pub oq_element_array_bus_addr: dma_addr_t,
    pub iq_pi: [*mut __le32 __iomem; 2],
    pub iq_pi_copy: [pqi_index_t; 2],
    pub iq_ci: [*mut pqi_index_t __iomem; 2],
    pub oq_pi: *mut pqi_index_t __iomem,
    pub iq_ci_bus_addr: [dma_addr_t; 2],
    pub oq_pi_bus_addr: dma_addr_t,
    pub oq_ci: *mut __le32 __iomem,
    pub oq_ci_copy: pqi_index_t,
    pub /: *mut *mut spinlock_t submit_lock[2]; / protect submission queue,
    pub request_list: [list_head; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_event_queue {
    pub oq_id: u16,
    pub int_msg_num: u16,
    pub oq_element_array: *mut c_void,
    pub oq_pi: *mut pqi_index_t __iomem,
    pub oq_element_array_bus_addr: dma_addr_t,
    pub oq_pi_bus_addr: dma_addr_t,
    pub oq_ci: *mut __le32 __iomem,
    pub oq_ci_copy: pqi_index_t,
}

pub const PQI_DEFAULT_QUEUE_GROUP: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_encryption_info {
    pub data_encryption_key_index: u16,
    pub encrypt_tweak_lower: u32,
    pub encrypt_tweak_upper: u32,
}

// configuration table section IDs

pub const PQI_CONFIG_TABLE_SECTION_GENERAL_INFO: c_int = 0;
pub const PQI_CONFIG_TABLE_SECTION_FIRMWARE_FEATURES: c_int = 1;
pub const PQI_CONFIG_TABLE_SECTION_FIRMWARE_ERRATA: c_int = 2;
pub const PQI_CONFIG_TABLE_SECTION_DEBUG: c_int = 3;
pub const PQI_CONFIG_TABLE_SECTION_HEARTBEAT: c_int = 4;
pub const PQI_CONFIG_TABLE_SECTION_SOFT_RESET: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table {
    pub /: *mut *mut u8 signature[8]; / "CFGTABLE",
    pub /: *mut *mut __le32 first_section_offset; / offset in bytes from the base,
// address of this table to the
// first section
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table_section_header {
    pub /: *mut *mut __le16 section_id; / as defined by the,
// PQI_CONFIG_TABLE_SECTION_*
// manifest constants above
    pub /: *mut *mut __le16 next_section_offset; / offset in bytes from base,
// address of the table of the
// next section or 0 if last entry
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table_general_info {
    pub header: pqi_config_table_section_header,
    pub /: *mut *mut __le32 section_length; / size of this section in bytes,
// including the section header
    pub /: *mut *mut __le32 max_outstanding_requests; / max. outstanding,
// commands supported by
// the controller
    pub /: *mut *mut __le32 max_sg_size; / max. transfer size of a single,
// command
    pub /: *mut *mut __le32 max_sg_per_request; / max. number of scatter-gather,
// entries supported in a single
// command
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table_firmware_features {
    pub header: pqi_config_table_section_header,
    pub num_elements: __le16,
    pub features_supported: [u8; ],
// u8	features_requested_by_host[];
// u8	features_enabled[];
// The 2 fields below are only valid if the MAX_KNOWN_FEATURE bit is set.
// __le16	firmware_max_known_feature;
// __le16	host_max_known_feature;
}

pub const PQI_FIRMWARE_FEATURE_OFA: c_int = 0;
pub const PQI_FIRMWARE_FEATURE_SMP: c_int = 1;
pub const PQI_FIRMWARE_FEATURE_MAX_KNOWN_FEATURE: c_int = 2;
pub const PQI_FIRMWARE_FEATURE_RAID_0_READ_BYPASS: c_int = 3;
pub const PQI_FIRMWARE_FEATURE_RAID_1_READ_BYPASS: c_int = 4;
pub const PQI_FIRMWARE_FEATURE_RAID_5_READ_BYPASS: c_int = 5;
pub const PQI_FIRMWARE_FEATURE_RAID_6_READ_BYPASS: c_int = 6;
pub const PQI_FIRMWARE_FEATURE_RAID_0_WRITE_BYPASS: c_int = 7;
pub const PQI_FIRMWARE_FEATURE_RAID_1_WRITE_BYPASS: c_int = 8;
pub const PQI_FIRMWARE_FEATURE_RAID_5_WRITE_BYPASS: c_int = 9;
pub const PQI_FIRMWARE_FEATURE_RAID_6_WRITE_BYPASS: c_int = 10;
pub const PQI_FIRMWARE_FEATURE_SOFT_RESET_HANDSHAKE: c_int = 11;
pub const PQI_FIRMWARE_FEATURE_UNIQUE_SATA_WWN: c_int = 12;
pub const PQI_FIRMWARE_FEATURE_RAID_IU_TIMEOUT: c_int = 13;
pub const PQI_FIRMWARE_FEATURE_TMF_IU_TIMEOUT: c_int = 14;
pub const PQI_FIRMWARE_FEATURE_RAID_BYPASS_ON_ENCRYPTED_NVME: c_int = 15;
pub const PQI_FIRMWARE_FEATURE_UNIQUE_WWID_IN_REPORT_PHYS_LUN: c_int = 16;
pub const PQI_FIRMWARE_FEATURE_FW_TRIAGE: c_int = 17;
pub const PQI_FIRMWARE_FEATURE_RPL_EXTENDED_FORMAT_4_5: c_int = 18;
pub const PQI_FIRMWARE_FEATURE_MULTI_LUN_DEVICE_SUPPORT: c_int = 21;
pub const PQI_FIRMWARE_FEATURE_CTRL_LOGGING: c_int = 22;
pub const PQI_FIRMWARE_FEATURE_MAXIMUM: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table_debug {
    pub header: pqi_config_table_section_header,
    pub scratchpad: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table_heartbeat {
    pub header: pqi_config_table_section_header,
    pub heartbeat_counter: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_config_table_soft_reset {
    pub header: pqi_config_table_section_header,
    pub soft_reset_status: u8,
}

pub const PQI_SOFT_RESET_INITIATE: c_uint = 0x1;
pub const PQI_SOFT_RESET_ABORT: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pqi_soft_reset_status {
    RESET_INITIATE_FIRMWARE,
    RESET_INITIATE_DRIVER,
    RESET_ABORT,
    RESET_NORESPONSE,
    RESET_TIMEDOUT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pqi_reset_register {
    pub 3: u32 reset_type :,
    pub 2: u32 reserved :,
    pub 3: u32 reset_action :,
    pub 1: u32 hold_in_pd1 :,
    pub 23: u32 reserved2 :,
    pub bits: },
    pub all_bits: u32,
}

pub const PQI_RESET_ACTION_RESET: c_uint = 0x1;
pub const PQI_RESET_TYPE_NO_RESET: c_uint = 0x0;
pub const PQI_RESET_TYPE_SOFT_RESET: c_uint = 0x1;
pub const PQI_RESET_TYPE_FIRM_RESET: c_uint = 0x2;
pub const PQI_RESET_TYPE_HARD_RESET: c_uint = 0x3;
pub const PQI_RESET_ACTION_COMPLETED: c_uint = 0x2;
pub const PQI_RESET_POLL_INTERVAL_MSECS: c_int = 100;

pub const PQI_MAX_OUTSTANDING_REQUESTS_KDUMP: c_int = 32;

pub const RAID_MAP_MAX_ENTRIES: c_int = 1024;
pub const RAID_MAP_MAX_DATA_DISKS_PER_ROW: c_int = 128;
pub const PQI_PHYSICAL_DEVICE_BUS: c_int = 0;
pub const PQI_RAID_VOLUME_BUS: c_int = 1;
pub const PQI_HBA_BUS: c_int = 2;
pub const PQI_EXTERNAL_RAID_VOLUME_BUS: c_int = 3;

pub const PQI_VSEP_CISS_BTL: c_int = 379;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_lun_header {
    pub list_length: __be32,
    pub flags: u8,
    pub reserved: [u8; 3],
}

// for flags field of struct report_lun_header

pub const CISS_REPORT_PHYS_FLAG_EXTENDED_FORMAT_2: c_uint = 0x2;
pub const CISS_REPORT_PHYS_FLAG_EXTENDED_FORMAT_4: c_uint = 0x4;
pub const CISS_REPORT_PHYS_FLAG_EXTENDED_FORMAT_MASK: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_log_lun {
    pub lunid: [u8; 8],
    pub volume_id: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_log_lun_list {
    pub header: report_lun_header,
    pub lun_entries: [report_log_lun; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_phys_lun_8byte_wwid {
    pub lunid: [u8; 8],
    pub wwid: __be64,
    pub device_type: u8,
    pub device_flags: u8,
    pub /: *mut *mut u8 lun_count; / number of LUNs in a multi-LUN device,
    pub redundant_paths: u8,
    pub aio_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_phys_lun_16byte_wwid {
    pub lunid: [u8; 8],
    pub wwid: [u8; 16],
    pub device_type: u8,
    pub device_flags: u8,
    pub /: *mut *mut u8 lun_count; / number of LUNs in a multi-LUN device,
    pub redundant_paths: u8,
    pub aio_handle: u32,
}

// for device_flags field of struct report_phys_lun_extended_entry
pub const CISS_REPORT_PHYS_DEV_FLAG_AIO_ENABLED: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_phys_lun_8byte_wwid_list {
    pub header: report_lun_header,
    pub lun_entries: [report_phys_lun_8byte_wwid; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_phys_lun_16byte_wwid_list {
    pub header: report_lun_header,
    pub lun_entries: [report_phys_lun_16byte_wwid; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_map_disk_data {
    pub aio_handle: u32,
    pub xor_mult: [u8; 2],
    pub reserved: [u8; 2],
}

// for flags field of RAID map
pub const RAID_MAP_ENCRYPTION_ENABLED: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_map {
    pub /: *mut *mut __le32 structure_size; / size of entire structure in bytes,
    pub /: *mut *mut __le32 volume_blk_size; / bytes / block in the volume,
    pub /: *mut *mut __le64 volume_blk_cnt; / logical blocks on the volume,
    pub /: *mut *mut u8 phys_blk_shift; / shift factor to convert between,
// units of logical blocks and
// physical disk blocks
    pub /: *mut *mut u8 parity_rotation_shift; / shift factor to convert between,
// units of logical stripes and
// physical stripes
    pub /: *mut *mut __le16 strip_size; / blocks used on each disk / stripe,
    pub /: *mut *mut __le64 disk_starting_blk; / first disk block used in volume,
    pub /: *mut *mut __le64 disk_blk_cnt; / disk blocks used by volume / disk,
    pub /: *mut *mut __le16 data_disks_per_row; / data disk entries / row in the map,
    pub /: *mut *mut __le16 metadata_disks_per_row; / mirror/parity disk entries / row,
// in the map
    pub /: *mut *mut __le16 row_cnt; / rows in each layout map,
    pub /: *mut *mut __le16 layout_map_count; / layout maps (1 map per,
// mirror parity group)
    pub flags: __le16,
    pub data_encryption_key_index: __le16,
    pub reserved: [u8; 16],
    pub disk_data: [raid_map_disk_data; RAID_MAP_MAX_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_scsi_dev_raid_map_data {
    pub is_write: bool,
    pub raid_level: u8,
    pub map_index: u32,
    pub first_block: u64,
    pub last_block: u64,
    pub data_length: u32,
    pub block_cnt: u32,
    pub blocks_per_row: u32,
    pub first_row: u64,
    pub last_row: u64,
    pub first_row_offset: u32,
    pub last_row_offset: u32,
    pub first_column: u32,
    pub last_column: u32,
    pub r5or6_first_row: u64,
    pub r5or6_last_row: u64,
    pub r5or6_first_row_offset: u32,
    pub r5or6_last_row_offset: u32,
    pub r5or6_first_column: u32,
    pub r5or6_last_column: u32,
    pub data_disks_per_row: u16,
    pub total_disks_per_row: u32,
    pub layout_map_count: u16,
    pub stripesize: u32,
    pub strip_size: u16,
    pub first_group: u32,
    pub last_group: u32,
    pub map_row: u32,
    pub aio_handle: u32,
    pub disk_block: u64,
    pub disk_block_cnt: u32,
    pub cdb: [u8; 16],
    pub cdb_length: u8,
// RAID 1 specific
pub const NUM_RAID1_MAP_ENTRIES: c_int = 3;
    pub num_it_nexus_entries: u32,
    pub it_nexus: [u32; NUM_RAID1_MAP_ENTRIES],
// RAID 5 / RAID 6 specific
    pub /: *mut *mut u32 p_parity_it_nexus; / aio_handle,
    pub /: *mut *mut u32 q_parity_it_nexus; / aio_handle,
    pub xor_mult: u8,
    pub row: u64,
    pub stripe_lba: u64,
    pub p_index: u32,
    pub q_index: u32,
}

pub const NUM_STREAMS_PER_LUN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_stream_data {
    pub next_lba: u64,
    pub last_accessed: u32,
}

pub const PQI_MAX_LUNS_PER_DEVICE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_tmf_work {
    pub work_struct: work_struct,
    pub scmd: *mut scsi_cmnd,
    pub ctrl_info: *mut pqi_ctrl_info,
    pub device: *mut pqi_scsi_dev,
    pub lun: u8,
    pub scsi_opcode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_raid_io_stats {
    pub raid_bypass_cnt: u64,
    pub write_stream_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_scsi_dev {
    pub /: *mut *mut int devtype; / as reported by INQUIRY command,
    pub /: *mut *mut u8 device_type; / as reported by,
// BMIC_IDENTIFY_PHYSICAL_DEVICE
// only valid for devtype = TYPE_DISK
    pub bus: c_int,
    pub target: c_int,
    pub lun: c_int,
    pub scsi3addr: [u8; 8],
    pub wwid: [u8; 16],
    pub volume_id: [u8; 16],
    pub 1: u8 is_physical_device :,
    pub 1: u8 is_external_raid_device :,
    pub 1: u8 is_expander_smp_device :,
    pub 1: u8 target_lun_valid :,
    pub 1: u8 device_gone :,
    pub 1: u8 new_device :,
    pub 1: u8 keep_device :,
    pub 1: u8 volume_offline :,
    pub 1: u8 rescan :,
    pub 1: u8 ignore_device :,
    pub 1: u8 erase_in_progress :,
    pub /: *mut *mut bool aio_enabled; / only valid for physical disks,
    pub in_remove: bool,
    pub in_reset: [bool; PQI_MAX_LUNS_PER_DEVICE],
    pub device_offline: bool,
    pub /: *mut *mut u8 vendor[8]; / bytes 8-15 of inquiry data,
    pub /: *mut *mut u8 model[16]; / bytes 16-31 of inquiry data,
    pub sas_address: u64,
    pub raid_level: u8,
    pub /: *mut *mut u16 queue_depth; / max. queue_depth for this device,
    pub advertised_queue_depth: u16,
    pub aio_handle: u32,
    pub volume_status: u8,
    pub active_path_index: u8,
    pub path_map: u8,
    pub bay: u8,
    pub box_index: u8,
    pub phys_box_on_bus: u8,
    pub phy_connected_dev_type: u8,
    pub box: [u8; 8],
    pub phys_connector: [u16; 8],
    pub phy_id: u8,
    pub ncq_prio_enable: u8,
    pub ncq_prio_support: u8,
    pub lun_count: u8,
    pub /: *mut *mut bool raid_bypass_configured; / RAID bypass configured,
    pub /: *mut *mut bool raid_bypass_enabled; / RAID bypass enabled,
    pub next_bypass_group: [u32; RAID_MAP_MAX_DATA_DISKS_PER_ROW],
    pub /: *mut *mut *mut raid_map raid_map; / RAID bypass map,
    pub max_transfer_encrypted: u32,
    pub sas_port: *mut pqi_sas_port,
    pub sdev: *mut scsi_device,
    pub scsi_device_list_entry: list_head,
    pub new_device_list_entry: list_head,
    pub add_list_entry: list_head,
    pub delete_list_entry: list_head,
    pub stream_data: [pqi_stream_data; NUM_STREAMS_PER_LUN],
    pub scsi_cmds_outstanding: [core::sync::atomic::AtomicI32; PQI_MAX_LUNS_PER_DEVICE],
    pub raid_io_stats: *mut pqi_raid_io_stats __percpu,
    pub tmf_work: [pqi_tmf_work; PQI_MAX_LUNS_PER_DEVICE],
}

// VPD inquiry pages
pub const CISS_VPD_LV_DEVICE_GEOMETRY: c_uint = 0xc1	/* vendor-specific page */;
pub const CISS_VPD_LV_BYPASS_STATUS: c_uint = 0xc2	/* vendor-specific page */;
pub const CISS_VPD_LV_STATUS: c_uint = 0xc3	/* vendor-specific page */;

// structure for CISS_VPD_LV_STATUS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ciss_vpd_logical_volume_status {
    pub peripheral_info: u8,
    pub page_code: u8,
    pub reserved: u8,
    pub page_length: u8,
    pub volume_status: u8,
    pub reserved2: [u8; 3],
    pub flags: __be32,
}

// constants for volume_status field of ciss_vpd_logical_volume_status
pub const CISS_LV_OK: c_int = 0;
pub const CISS_LV_FAILED: c_int = 1;
pub const CISS_LV_NOT_CONFIGURED: c_int = 2;
pub const CISS_LV_DEGRADED: c_int = 3;
pub const CISS_LV_READY_FOR_RECOVERY: c_int = 4;
pub const CISS_LV_UNDERGOING_RECOVERY: c_int = 5;
pub const CISS_LV_WRONG_PHYSICAL_DRIVE_REPLACED: c_int = 6;
pub const CISS_LV_PHYSICAL_DRIVE_CONNECTION_PROBLEM: c_int = 7;
pub const CISS_LV_HARDWARE_OVERHEATING: c_int = 8;
pub const CISS_LV_HARDWARE_HAS_OVERHEATED: c_int = 9;
pub const CISS_LV_UNDERGOING_EXPANSION: c_int = 10;
pub const CISS_LV_NOT_AVAILABLE: c_int = 11;
pub const CISS_LV_QUEUED_FOR_EXPANSION: c_int = 12;
pub const CISS_LV_DISABLED_SCSI_ID_CONFLICT: c_int = 13;
pub const CISS_LV_EJECTED: c_int = 14;
pub const CISS_LV_UNDERGOING_ERASE: c_int = 15;
// state 16 not used
pub const CISS_LV_READY_FOR_PREDICTIVE_SPARE_REBUILD: c_int = 17;
pub const CISS_LV_UNDERGOING_RPI: c_int = 18;
pub const CISS_LV_PENDING_RPI: c_int = 19;
pub const CISS_LV_ENCRYPTED_NO_KEY: c_int = 20;
// state 21 not used
pub const CISS_LV_UNDERGOING_ENCRYPTION: c_int = 22;
pub const CISS_LV_UNDERGOING_ENCRYPTION_REKEYING: c_int = 23;
pub const CISS_LV_ENCRYPTED_IN_NON_ENCRYPTED_CONTROLLER: c_int = 24;
pub const CISS_LV_PENDING_ENCRYPTION: c_int = 25;
pub const CISS_LV_PENDING_ENCRYPTION_REKEYING: c_int = 26;
pub const CISS_LV_NOT_SUPPORTED: c_int = 27;
pub const CISS_LV_STATUS_UNAVAILABLE: c_int = 255;
// constants for flags field of ciss_vpd_logical_volume_status
pub const CISS_LV_FLAGS_NO_HOST_IO: c_uint = 0x1	/* volume not available for */;
// host I/O
// for SAS hosts and SAS expanders
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_sas_node {
    pub parent_dev: *mut device,
    pub port_list_head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_sas_port {
    pub port_list_entry: list_head,
    pub sas_address: u64,
    pub device: *mut pqi_scsi_dev,
    pub port: *mut sas_port,
    pub next_phy_index: c_int,
    pub phy_list_head: list_head,
    pub parent_node: *mut pqi_sas_node,
    pub rphy: *mut sas_rphy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_sas_phy {
    pub phy_list_entry: list_head,
    pub phy: *mut sas_phy,
    pub parent_port: *mut pqi_sas_port,
    pub added_to_port: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_io_request {
    pub refcount: core::sync::atomic::AtomicI32,
    pub index: u16,
    pub context): *mut c_void,
    pub context: *mut c_void,
    pub 1: u8 raid_bypass :,
    pub status: c_int,
    pub queue_group: *mut pqi_queue_group,
    pub scmd: *mut scsi_cmnd,
    pub error_info: *mut c_void,
    pub sg_chain_buffer: *mut pqi_sg_descriptor,
    pub sg_chain_buffer_dma_handle: dma_addr_t,
    pub iu: *mut c_void,
    pub request_list_entry: list_head,
}

pub const PQI_NUM_SUPPORTED_EVENTS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_event {
    pub pending: bool,
    pub event_type: u8,
    pub event_id: u16,
    pub additional_event_id: u32,
}

pub const PQI_RESERVED_IO_SLOTS_LUN_RESET: c_int = 1;

pub const PQI_RESERVED_IO_SLOTS_SYNCHRONOUS_REQUESTS: c_int = 3;

pub const PQI_CTRL_PRODUCT_ID_GEN1: c_int = 0;
pub const PQI_CTRL_PRODUCT_ID_GEN2: c_int = 7;
pub const PQI_CTRL_PRODUCT_REVISION_A: c_int = 0;
pub const PQI_CTRL_PRODUCT_REVISION_B: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pqi_ctrl_removal_state {
    PQI_CTRL_PRESENT = 0,
    PQI_CTRL_GRACEFUL_REMOVAL,
    PQI_CTRL_SURPRISE_REMOVAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pqi_ctrl_info {
    pub ctrl_id: c_uint,
    pub pci_dev: *mut pci_dev,
    pub firmware_version: [c_char; 32],
    pub serial_number: [c_char; 17],
    pub model: [c_char; 17],
    pub vendor: [c_char; 9],
    pub product_id: u8,
    pub product_revision: u8,
    pub iomem_base: *mut void __iomem,
    pub registers: *mut pqi_ctrl_registers __iomem,
    pub pqi_registers: *mut pqi_device_registers __iomem,
    pub max_sg_entries: u32,
    pub config_table_offset: u32,
    pub config_table_length: u32,
    pub max_inbound_queues: u16,
    pub max_elements_per_iq: u16,
    pub max_iq_element_length: u16,
    pub max_outbound_queues: u16,
    pub max_elements_per_oq: u16,
    pub max_oq_element_length: u16,
    pub max_transfer_size: u32,
    pub max_outstanding_requests: u32,
    pub max_io_slots: u32,
    pub scsi_ml_can_queue: c_uint,
    pub sg_tablesize: c_ushort,
    pub max_sectors: c_uint,
    pub error_buffer_length: u32,
    pub error_buffer: *mut c_void,
    pub error_buffer_dma_handle: dma_addr_t,
    pub sg_chain_buffer_length: usize,
    pub num_queue_groups: c_uint,
    pub num_elements_per_iq: u16,
    pub num_elements_per_oq: u16,
    pub max_inbound_iu_length_per_firmware: u16,
    pub max_inbound_iu_length: u16,
    pub max_sg_per_iu: c_uint,
    pub max_sg_per_r56_iu: c_uint,
    pub admin_queue_memory_base: *mut c_void,
    pub admin_queue_memory_length: u32,
    pub admin_queue_memory_base_dma_handle: dma_addr_t,
    pub queue_memory_base: *mut c_void,
    pub queue_memory_length: u32,
    pub queue_memory_base_dma_handle: dma_addr_t,
    pub admin_queues: pqi_admin_queues,
    pub queue_groups: [pqi_queue_group; PQI_MAX_QUEUE_GROUPS],
    pub event_queue: pqi_event_queue,
    pub irq_mode: pqi_irq_mode,
    pub max_msix_vectors: c_int,
    pub num_msix_vectors_enabled: c_int,
    pub num_msix_vectors_initialized: c_int,
    pub event_irq: c_int,
    pub scsi_host: *mut Scsi_Host,
    pub scan_mutex: mutex,
    pub lun_reset_mutex: mutex,
    pub controller_online: bool,
    pub block_requests: bool,
    pub scan_blocked: bool,
    pub 1: u8 inbound_spanning_supported :,
    pub 1: u8 outbound_spanning_supported :,
    pub 1: u8 pqi_mode_enabled :,
    pub 1: u8 pqi_reset_quiesce_supported :,
    pub 1: u8 soft_reset_handshake_supported :,
    pub 1: u8 raid_iu_timeout_supported :,
    pub 1: u8 tmf_iu_timeout_supported :,
    pub 1: u8 firmware_triage_supported :,
    pub 1: u8 rpl_extended_format_4_5_supported :,
    pub 1: u8 multi_lun_device_supported :,
    pub 1: u8 ctrl_logging_supported :,
    pub 1: u8 enable_r1_writes :,
    pub 1: u8 enable_r5_writes :,
    pub 1: u8 enable_r6_writes :,
    pub 1: u8 lv_drive_type_mix_valid :,
    pub 1: u8 enable_stream_detection :,
    pub 1: u8 disable_managed_interrupts :,
    pub ciss_report_log_flags: u8,
    pub max_transfer_encrypted_sas_sata: u32,
    pub max_transfer_encrypted_nvme: u32,
    pub max_write_raid_5_6: u32,
    pub max_write_raid_1_10_2drive: u32,
    pub max_write_raid_1_10_3drive: u32,
    pub numa_node: c_int,
    pub scsi_device_list: list_head,
    pub scsi_device_list_lock: spinlock_t,
    pub rescan_work: delayed_work,
    pub update_time_work: delayed_work,
    pub sas_host: *mut pqi_sas_node,
    pub sas_address: u64,
    pub io_request_pool: *mut pqi_io_request,
    pub events: [pqi_event; PQI_NUM_SUPPORTED_EVENTS],
    pub event_work: work_struct,
    pub num_interrupts: core::sync::atomic::AtomicI32,
    pub previous_num_interrupts: c_int,
    pub previous_heartbeat_count: u32,
    pub heartbeat_counter: *mut __le32 __iomem,
    pub soft_reset_status: *mut u8 __iomem,
    pub heartbeat_timer: timer_list,
    pub ctrl_offline_work: work_struct,
    pub sync_request_sem: semaphore,
    pub num_busy_threads: core::sync::atomic::AtomicI32,
    pub num_blocked_threads: core::sync::atomic::AtomicI32,
    pub block_requests_wait: wait_queue_head_t,
    pub ofa_mutex: mutex,
    pub ofa_memory_alloc_work: work_struct,
    pub ofa_quiesce_work: work_struct,
    pub ofa_bytes_requested: u32,
    pub ofa_cancel_reason: u16,
    pub ofa_memory: pqi_host_memory_descriptor,
    pub ctrl_log_memory: pqi_host_memory_descriptor,
    pub ctrl_removal_state: pqi_ctrl_removal_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pqi_ctrl_mode {
    SIS_MODE = 0,
    PQI_MODE
}

//
// assume worst case: SATA queue depth of 31 minus 4 internal firmware commands
//
pub const PQI_PHYSICAL_DISK_DEFAULT_MAX_QUEUE_DEPTH: c_int = 27;
// CISS commands
pub const CISS_READ: c_uint = 0xc0;
pub const CISS_REPORT_LOG: c_uint = 0xc2	/* Report Logical LUNs */;
pub const CISS_REPORT_PHYS: c_uint = 0xc3	/* Report Physical LUNs */;
pub const CISS_GET_RAID_MAP: c_uint = 0xc8;
// BMIC commands
pub const BMIC_IDENTIFY_CONTROLLER: c_uint = 0x11;
pub const BMIC_IDENTIFY_PHYSICAL_DEVICE: c_uint = 0x15;
pub const BMIC_READ: c_uint = 0x26;
pub const BMIC_WRITE: c_uint = 0x27;
pub const BMIC_SENSE_FEATURE: c_uint = 0x61;
pub const BMIC_SENSE_CONTROLLER_PARAMETERS: c_uint = 0x64;
pub const BMIC_SENSE_SUBSYSTEM_INFORMATION: c_uint = 0x66;
pub const BMIC_CSMI_PASSTHRU: c_uint = 0x68;
pub const BMIC_WRITE_HOST_WELLNESS: c_uint = 0xa5;
pub const BMIC_FLUSH_CACHE: c_uint = 0xc2;
pub const BMIC_SET_DIAG_OPTIONS: c_uint = 0xf4;
pub const BMIC_SENSE_DIAG_OPTIONS: c_uint = 0xf5;
pub const CSMI_CC_SAS_SMP_PASSTHRU: c_uint = 0x17;
pub const SA_FLUSH_CACHE: c_uint = 0x1;

pub const LV_DRIVE_TYPE_MIX_UNKNOWN: c_int = 0;
pub const LV_DRIVE_TYPE_MIX_NO_RESTRICTION: c_int = 1;
pub const LV_DRIVE_TYPE_MIX_SAS_HDD_ONLY: c_int = 2;
pub const LV_DRIVE_TYPE_MIX_SATA_HDD_ONLY: c_int = 3;
pub const LV_DRIVE_TYPE_MIX_SAS_OR_SATA_SSD_ONLY: c_int = 4;
pub const LV_DRIVE_TYPE_MIX_SAS_SSD_ONLY: c_int = 5;
pub const LV_DRIVE_TYPE_MIX_SATA_SSD_ONLY: c_int = 6;
pub const LV_DRIVE_TYPE_MIX_SAS_ONLY: c_int = 7;
pub const LV_DRIVE_TYPE_MIX_SATA_ONLY: c_int = 8;
pub const LV_DRIVE_TYPE_MIX_NVME_ONLY: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_identify_controller {
    pub configured_logical_drive_count: u8,
    pub configuration_signature: __le32,
    pub firmware_version_short: [u8; 4],
    pub reserved: [u8; 145],
    pub extended_logical_unit_count: __le16,
    pub reserved1: [u8; 34],
    pub firmware_build_number: __le16,
    pub reserved2: [u8; 8],
    pub vendor_id: [u8; 8],
    pub product_id: [u8; 16],
    pub reserved3: [u8; 62],
    pub extra_controller_flags: __le32,
    pub reserved4: [u8; 2],
    pub controller_mode: u8,
    pub spare_part_number: [u8; 32],
    pub firmware_version_long: [u8; 32],
}

// constants for extra_controller_flags field of bmic_identify_controller
pub const BMIC_IDENTIFY_EXTRA_FLAGS_LONG_FW_VERSION_SUPPORTED: c_uint = 0x20000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_sense_subsystem_info {
    pub reserved: [u8; 44],
    pub ctrl_serial_number: [u8; 16],
}

// constants for device_type field
pub const SA_DEVICE_TYPE_SATA: c_uint = 0x1;
pub const SA_DEVICE_TYPE_SAS: c_uint = 0x2;
pub const SA_DEVICE_TYPE_EXPANDER_SMP: c_uint = 0x5;
pub const SA_DEVICE_TYPE_SES: c_uint = 0x6;
pub const SA_DEVICE_TYPE_CONTROLLER: c_uint = 0x7;
pub const SA_DEVICE_TYPE_NVME: c_uint = 0x9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_identify_physical_device {
    pub /: *mut *mut u8 scsi_bus; / SCSI Bus number on controller,
    pub /: *mut *mut u8 scsi_id; / SCSI ID on this bus,
    pub /: *mut *mut __le16 block_size; / sector size in bytes,
    pub /: *mut *mut __le32 total_blocks; / number for sectors on drive,
    pub /: *mut *mut __le32 reserved_blocks; / controller reserved (RIS),
    pub /: *mut *mut u8 model[40]; / Physical Drive Model,
    pub /: *mut *mut u8 serial_number[40]; / Drive Serial Number,
    pub /: *mut *mut u8 firmware_revision[8]; / drive firmware revision,
    pub /: *mut *mut u8 scsi_inquiry_bits; / inquiry byte 7 bits,
    pub /: *mut *mut u8 compaq_drive_stamp; / 0 means drive not stamped,
    pub last_failure_reason: u8,
    pub flags: u8,
    pub more_flags: u8,
    pub /: *mut *mut u8 scsi_lun; / SCSI LUN for phys drive,
    pub yet_more_flags: u8,
    pub even_more_flags: u8,
    pub spi_speed_rules: __le32,
    pub /: *mut *mut u8 phys_connector[2]; / connector number on controller,
    pub /: *mut *mut u8 phys_box_on_bus; / phys enclosure this drive resides,
    pub /: *mut *mut u8 phys_bay_in_box; / phys drv bay this drive resides,
    pub /: *mut *mut __le32 rpm; / drive rotational speed in RPM,
    pub /: *mut *mut u8 device_type; / type of drive,
    pub /: *mut *mut u8 sata_version; / only valid when device_type =,
// SA_DEVICE_TYPE_SATA
    pub big_total_block_count: __le64,
    pub ris_starting_lba: __le64,
    pub ris_size: __le32,
    pub wwid: [u8; 20],
    pub controller_phy_map: [u8; 32],
    pub phy_count: __le16,
    pub phy_connected_dev_type: [u8; 256],
    pub phy_to_drive_bay_num: [u8; 256],
    pub phy_to_attached_dev_index: [__le16; 256],
    pub box_index: u8,
    pub reserved: u8,
    pub extra_physical_drive_flags: __le16,
    pub negotiated_link_rate: [u8; 256],
    pub phy_to_phy_map: [u8; 256],
    pub redundant_path_present_map: u8,
    pub redundant_path_failure_map: u8,
    pub active_path_number: u8,
    pub alternate_paths_phys_connector: [__le16; 8],
    pub alternate_paths_phys_box_on_port: [u8; 8],
    pub multi_lun_device_lun_count: u8,
    pub minimum_good_fw_revision: [u8; 8],
    pub unique_inquiry_bytes: [u8; 20],
    pub current_temperature_degrees: u8,
    pub temperature_threshold_degrees: u8,
    pub max_temperature_degrees: u8,
    pub logical_blocks_per_phys_block_exp: u8,
    pub current_queue_depth_limit: __le16,
    pub switch_name: [u8; 10],
    pub switch_port: __le16,
    pub alternate_paths_switch_name: [u8; 40],
    pub alternate_paths_switch_port: [u8; 8],
    pub power_on_hours: __le16,
    pub percent_endurance_used: __le16,
    pub drive_authentication: u8,
    pub smart_carrier_authentication: u8,
    pub smart_carrier_app_fw_version: u8,
    pub smart_carrier_bootloader_fw_version: u8,
    pub sanitize_flags: u8,
    pub encryption_key_flags: u8,
    pub encryption_key_name: [u8; 64],
    pub misc_drive_flags: __le32,
    pub dek_index: __le16,
    pub hba_drive_encryption_flags: __le16,
    pub max_overwrite_time: __le16,
    pub max_block_erase_time: __le16,
    pub max_crypto_erase_time: __le16,
    pub connector_info: [u8; 5],
    pub connector_name: [u8; 8][8],
    pub page_83_identifier: [u8; 16],
    pub maximum_link_rate: [u8; 256],
    pub negotiated_physical_link_rate: [u8; 256],
    pub box_connector_name: [u8; 8],
    pub padding_to_multiple_of_512: [u8; 9],
}

pub const BMIC_SENSE_FEATURE_IO_PAGE: c_uint = 0x8;
pub const BMIC_SENSE_FEATURE_IO_PAGE_AIO_SUBPAGE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_sense_feature_buffer_header {
    pub page_code: u8,
    pub subpage_code: u8,
    pub buffer_length: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_sense_feature_page_header {
    pub page_code: u8,
    pub subpage_code: u8,
    pub page_length: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_sense_feature_io_page_aio_subpage {
    pub header: bmic_sense_feature_page_header,
    pub firmware_read_support: u8,
    pub driver_read_support: u8,
    pub firmware_write_support: u8,
    pub driver_write_support: u8,
    pub max_transfer_encrypted_sas_sata: __le16,
    pub max_transfer_encrypted_nvme: __le16,
    pub max_write_raid_5_6: __le16,
    pub max_write_raid_1_10_2drive: __le16,
    pub max_write_raid_1_10_3drive: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_smp_request {
    pub frame_type: u8,
    pub function: u8,
    pub allocated_response_length: u8,
    pub request_length: u8,
    pub additional_request_bytes: [u8; 1016],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_smp_response {
    pub frame_type: u8,
    pub function: u8,
    pub function_result: u8,
    pub response_length: u8,
    pub additional_response_bytes: [u8; 1016],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_csmi_ioctl_header {
    pub header_length: __le32,
    pub signature: [u8; 8],
    pub timeout: __le32,
    pub control_code: __le32,
    pub return_code: __le32,
    pub length: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_csmi_smp_passthru {
    pub phy_identifier: u8,
    pub port_identifier: u8,
    pub connection_rate: u8,
    pub reserved: u8,
    pub destination_sas_address: __be64,
    pub request_length: __le32,
    pub request: bmic_smp_request,
    pub connection_status: u8,
    pub reserved1: [u8; 3],
    pub response_length: __le32,
    pub response: bmic_smp_response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_csmi_smp_passthru_buffer {
    pub ioctl_header: bmic_csmi_ioctl_header,
    pub parameters: bmic_csmi_smp_passthru,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_flush_cache {
    pub disable_flag: u8,
    pub system_power_action: u8,
    pub ndu_flush: u8,
    pub shutdown_event: u8,
    pub reserved: [u8; 28],
}

// for shutdown_event member of struct bmic_flush_cache
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bmic_flush_cache_shutdown_event {
    NONE_CACHE_FLUSH_ONLY = 0,
    SHUTDOWN = 1,
    HIBERNATE = 2,
    SUSPEND = 3,
    RESTART = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_diag_options {
    pub options: __le32,
}

extern "C" {
    pub fn pqi_add_sas_host(shost: *mut Scsi_Host, ctrl_info: *mut pqi_ctrl_info) -> c_int;
}
extern "C" {
    pub fn pqi_delete_sas_host(ctrl_info: *mut pqi_ctrl_info);
}
extern "C" {
    pub fn pqi_remove_sas_device(device: *mut pqi_scsi_dev);
}
extern "C" {
    pub fn pqi_prep_for_scsi_done(scmd: *mut scsi_cmnd);
}
