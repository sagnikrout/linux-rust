//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/switchtec.h
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
// Microsemi Switchtec PCIe Driver
// Copyright (c) 2017, Microsemi Corporation
//

pub const SWITCHTEC_MRPC_PAYLOAD_SIZE: c_int = 1024;
pub const SWITCHTEC_MAX_PFF_CSR: c_int = 255;

pub const MRPC_GAS_READ: c_uint = 0x29;
pub const MRPC_GAS_WRITE: c_uint = 0x87;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum switchtec_gen {
    SWITCHTEC_GEN3,
    SWITCHTEC_GEN4,
    SWITCHTEC_GEN5,
    SWITCHTEC_GEN6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrpc_regs {
    pub input_data: [u8; SWITCHTEC_MRPC_PAYLOAD_SIZE],
    pub output_data: [u8; SWITCHTEC_MRPC_PAYLOAD_SIZE],
    pub cmd: u32,
    pub status: u32,
    pub ret_value: u32,
    pub dma_en: u32,
    pub dma_addr: u64,
    pub dma_vector: u32,
    pub dma_ver: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrpc_status {
    SWITCHTEC_MRPC_STATUS_INPROGRESS = 1,
    SWITCHTEC_MRPC_STATUS_DONE = 2,
    SWITCHTEC_MRPC_STATUS_ERROR = 0xFF,
    SWITCHTEC_MRPC_STATUS_INTERRUPTED = 0x100,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_event_regs {
    pub event_report_ctrl: u64,
    pub reserved1: u64,
    pub part_event_bitmap: u64,
    pub reserved2: u64,
    pub global_summary: u32,
    pub reserved3: [u32; 3],
    pub stack_error_event_hdr: u32,
    pub stack_error_event_data: u32,
    pub reserved4: [u32; 4],
    pub ppu_error_event_hdr: u32,
    pub ppu_error_event_data: u32,
    pub reserved5: [u32; 4],
    pub isp_error_event_hdr: u32,
    pub isp_error_event_data: u32,
    pub reserved6: [u32; 4],
    pub sys_reset_event_hdr: u32,
    pub reserved7: [u32; 5],
    pub fw_exception_hdr: u32,
    pub reserved8: [u32; 5],
    pub fw_nmi_hdr: u32,
    pub reserved9: [u32; 5],
    pub fw_non_fatal_hdr: u32,
    pub reserved10: [u32; 5],
    pub fw_fatal_hdr: u32,
    pub reserved11: [u32; 5],
    pub twi_mrpc_comp_hdr: u32,
    pub twi_mrpc_comp_data: u32,
    pub reserved12: [u32; 4],
    pub twi_mrpc_comp_async_hdr: u32,
    pub twi_mrpc_comp_async_data: u32,
    pub reserved13: [u32; 4],
    pub cli_mrpc_comp_hdr: u32,
    pub cli_mrpc_comp_data: u32,
    pub reserved14: [u32; 4],
    pub cli_mrpc_comp_async_hdr: u32,
    pub cli_mrpc_comp_async_data: u32,
    pub reserved15: [u32; 4],
    pub gpio_interrupt_hdr: u32,
    pub gpio_interrupt_data: u32,
    pub reserved16: [u32; 4],
    pub gfms_event_hdr: u32,
    pub gfms_event_data: u32,
    pub reserved17: [u32; 4],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_info_regs_gen3 {
    pub reserved1: u32,
    pub vendor_table_revision: u32,
    pub table_format_version: u32,
    pub partition_id: u32,
    pub cfg_file_fmt_version: u32,
    pub cfg_running: u16,
    pub img_running: u16,
    pub reserved2: [u32; 57],
    pub vendor_id: [c_char; 8],
    pub product_id: [c_char; 16],
    pub product_revision: [c_char; 4],
    pub component_vendor: [c_char; 8],
    pub component_id: u16,
    pub component_revision: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_info_regs_gen4 {
    pub gas_layout_ver: u16,
    pub evlist_ver: u8,
    pub reserved1: u8,
    pub mgmt_cmd_set_ver: u16,
    pub fabric_cmd_set_ver: u16,
    pub reserved2: [u32; 2],
    pub mrpc_uart_ver: u8,
    pub mrpc_twi_ver: u8,
    pub mrpc_eth_ver: u8,
    pub mrpc_inband_ver: u8,
    pub reserved3: [u32; 7],
    pub fw_update_tmo: u32,
    pub xml_version_cfg: u32,
    pub xml_version_img: u32,
    pub partition_id: u32,
    pub bl2_running: u16,
    pub cfg_running: u16,
    pub img_running: u16,
    pub key_running: u16,
    pub reserved4: [u32; 43],
    pub vendor_seeprom_twi: u32,
    pub vendor_table_revision: u32,
    pub vendor_specific_info: [u32; 2],
    pub p2p_vendor_id: u16,
    pub p2p_device_id: u16,
    pub p2p_revision_id: u8,
    pub reserved5: [u8; 3],
    pub p2p_class_id: u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub p2p_serial_number: [u32; 2],
    pub mac_addr: [u8; 6],
    pub reserved6: [u8; 2],
    pub reserved7: [u32; 3],
    pub vendor_id: [c_char; 8],
    pub product_id: [c_char; 24],
    pub product_revision: [c_char; 2],
    pub reserved8: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_info_regs {
    pub device_id: u32,
    pub device_version: u32,
    pub firmware_version: u32,
    pub gen3: sys_info_regs_gen3,
    pub gen4: sys_info_regs_gen4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct partition_info {
    pub address: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_info_regs_gen3 {
    pub flash_part_map_upd_idx: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct active_partition_info_gen3 {
    pub address: u32,
    pub build_version: u32,
    pub build_string: u32,
    pub active_img: },
    pub active_cfg: active_partition_info_gen3,
    pub inactive_img: active_partition_info_gen3,
    pub inactive_cfg: active_partition_info_gen3,
    pub flash_length: u32,
    pub cfg0: partition_info,
    pub cfg1: partition_info,
    pub img0: partition_info,
    pub img1: partition_info,
    pub nvlog: partition_info,
    pub vendor: [partition_info; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_info_regs_gen4 {
    pub flash_address: u32,
    pub flash_length: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct active_partition_info_gen4 {
    pub bl2: c_uchar,
    pub cfg: c_uchar,
    pub img: c_uchar,
    pub key: c_uchar,
    pub active_flag: },
    pub reserved: [u32; 3],
    pub map0: partition_info,
    pub map1: partition_info,
    pub key0: partition_info,
    pub key1: partition_info,
    pub bl2_0: partition_info,
    pub bl2_1: partition_info,
    pub cfg0: partition_info,
    pub cfg1: partition_info,
    pub img0: partition_info,
    pub img1: partition_info,
    pub nvlog: partition_info,
    pub vendor: [partition_info; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_info_regs {
    pub gen3: flash_info_regs_gen3,
    pub gen4: flash_info_regs_gen4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_info_regs {
    pub partition_count: u8,
    pub partition_id: u8,
    pub reserved1: u16,
    pub ep_map: u64,
    pub requester_id: u16,
    pub reserved2: u16,
    pub reserved3: [u32; 4],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nt_partition_info {
    pub xlink_enabled: u32,
    pub target_part_low: u32,
    pub target_part_high: u32,
    pub reserved: u32,
    pub ntp_info: [}; 48],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct part_cfg_regs {
    pub status: u32,
    pub state: u32,
    pub port_cnt: u32,
    pub usp_port_mode: u32,
    pub usp_pff_inst_id: u32,
    pub vep_pff_inst_id: u32,
    pub dsp_pff_inst_id: [u32; 47],
    pub reserved1: [u32; 11],
    pub vep_vector_number: u16,
    pub usp_vector_number: u16,
    pub port_event_bitmap: u32,
    pub reserved2: [u32; 3],
    pub part_event_summary: u32,
    pub reserved3: [u32; 3],
    pub part_reset_hdr: u32,
    pub part_reset_data: [u32; 5],
    pub mrpc_comp_hdr: u32,
    pub mrpc_comp_data: [u32; 5],
    pub mrpc_comp_async_hdr: u32,
    pub mrpc_comp_async_data: [u32; 5],
    pub dyn_binding_hdr: u32,
    pub dyn_binding_data: [u32; 5],
    pub intercomm_notify_hdr: u32,
    pub intercomm_notify_data: [u32; 5],
    pub reserved4: [u32; 153],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_ctrl_regs {
    pub partition_status: u32,
    pub partition_op: u32,
    pub partition_ctrl: u32,
    pub bar_setup: u32,
    pub bar_error: u32,
    pub lut_table_entries: u16,
    pub lut_table_offset: u16,
    pub lut_error: u32,
    pub req_id_table_size: u16,
    pub req_id_table_offset: u16,
    pub req_id_error: u32,
    pub reserved1: [u32; 7],
    pub ctl: u32,
    pub win_size: u32,
    pub xlate_addr: u64,
    pub bar_entry: [}; 6],
    pub win_size: u32,
    pub reserved: [u32; 3],
    pub bar_ext_entry: [}; 6],
    pub reserved2: [u32; 192],
    pub req_id_table: [u32; 512],
    pub reserved3: [u32; 256],
    pub lut_entry: [u64; 512],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_dbmsg_regs {
    pub reserved1: [u32; 1024],
    pub odb: u64,
    pub odb_mask: u64,
    pub idb: u64,
    pub idb_mask: u64,
    pub idb_vec_map: [u8; 64],
    pub msg_map: u32,
    pub reserved2: u32,
    pub msg: u32,
    pub status: u32,
    pub omsg: [}; 4],
    pub msg: u32,
    pub status: u8,
    pub mask: u8,
    pub src: u8,
    pub reserved: u8,
    pub imsg: [}; 4],
    pub reserved3: [u8; 3928],
    pub msix_table: [u8; 1024],
    pub reserved4: [u8; 3072],
    pub pba: [u8; 24],
    pub reserved5: [u8; 4072],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pff_csr_regs {
    pub vendor_id: u16,
    pub device_id: u16,
    pub pcicmd: u16,
    pub pcists: u16,
    pub pci_class: u32,
    pub pci_opts: u32,
    pub pci_bar: [u32; 6],
    pub pci_bar64: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_mrpc_output {
    pub status: u32,
    pub cmd_id: u32,
    pub rtn_code: u32,
    pub output_size: u32,
    pub data: [u8; SWITCHTEC_MRPC_PAYLOAD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_dev {
    pub pdev: *mut pci_dev,
    pub dev: device,
    pub cdev: cdev,
    pub gen: switchtec_gen,
    pub partition: c_int,
    pub partition_count: c_int,
    pub pff_csr_count: c_int,
    pub pff_local: [c_char; SWITCHTEC_MAX_PFF_CSR],
    pub mmio: *mut void __iomem,
    pub mmio_mrpc: *mut mrpc_regs __iomem,
    pub mmio_sw_event: *mut sw_event_regs __iomem,
    pub mmio_sys_info: *mut sys_info_regs __iomem,
    pub mmio_flash_info: *mut flash_info_regs __iomem,
    pub mmio_ntb: *mut ntb_info_regs __iomem,
    pub mmio_part_cfg: *mut part_cfg_regs __iomem,
    pub mmio_part_cfg_all: *mut part_cfg_regs __iomem,
    pub mmio_pff_csr: *mut pff_csr_regs __iomem,
//
// The mrpc mutex must be held when accessing the other
// mrpc_ fields, alive flag and stuser->state field
//
    pub mrpc_mutex: mutex,
    pub mrpc_queue: list_head,
    pub mrpc_busy: c_int,
    pub mrpc_work: work_struct,
    pub mrpc_timeout: delayed_work,
    pub alive: bool,
    pub event_wq: wait_queue_head_t,
    pub event_cnt: core::sync::atomic::AtomicI32,
    pub link_event_work: work_struct,
    pub stdev): *mut *mut void (link_notifier)(struct switchtec_dev,
    pub link_event_count: [u8; SWITCHTEC_MAX_PFF_CSR],
    pub sndev: *mut switchtec_ntb,
    pub dma_mrpc: *mut dma_mrpc_output,
    pub dma_mrpc_dma_addr: dma_addr_t,
}

extern "C" {
    pub fn container_of(_arg: dev, switchtec_dev: struct, _arg: dev) -> return;
}
