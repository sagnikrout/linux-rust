//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_83xx.h
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
// QLogic iSCSI HBA Driver
// Copyright (c)  2003-2013 QLogic Corporation
//
// Indirectly Mapped Registers
pub const QLA83XX_FLASH_SPI_STATUS: c_uint = 0x2808E010;
pub const QLA83XX_FLASH_SPI_CONTROL: c_uint = 0x2808E014;
pub const QLA83XX_FLASH_STATUS: c_uint = 0x42100004;
pub const QLA83XX_FLASH_CONTROL: c_uint = 0x42110004;
pub const QLA83XX_FLASH_ADDR: c_uint = 0x42110008;
pub const QLA83XX_FLASH_WRDATA: c_uint = 0x4211000C;
pub const QLA83XX_FLASH_RDDATA: c_uint = 0x42110018;
pub const QLA83XX_FLASH_DIRECT_WINDOW: c_uint = 0x42110030;

// Directly Mapped Registers in 83xx register table
// Flash access regs
pub const QLA83XX_FLASH_LOCK: c_uint = 0x3850;
pub const QLA83XX_FLASH_UNLOCK: c_uint = 0x3854;
pub const QLA83XX_FLASH_LOCK_ID: c_uint = 0x3500;
// Driver Lock regs
pub const QLA83XX_DRV_LOCK: c_uint = 0x3868;
pub const QLA83XX_DRV_UNLOCK: c_uint = 0x386C;
pub const QLA83XX_DRV_LOCK_ID: c_uint = 0x3504;
pub const QLA83XX_DRV_LOCKRECOVERY: c_uint = 0x379C;
// IDC version
pub const QLA83XX_IDC_VER_MAJ_VALUE: c_uint = 0x1;
pub const QLA83XX_IDC_VER_MIN_VALUE: c_uint = 0x0;
// IDC Registers : Driver Coexistence Defines
pub const QLA83XX_CRB_IDC_VER_MAJOR: c_uint = 0x3780;
pub const QLA83XX_CRB_IDC_VER_MINOR: c_uint = 0x3798;
pub const QLA83XX_IDC_DRV_CTRL: c_uint = 0x3790;
pub const QLA83XX_IDC_DRV_AUDIT: c_uint = 0x3794;
pub const QLA83XX_SRE_SHIM_CONTROL: c_uint = 0x0D200284;
pub const QLA83XX_PORT0_RXB_PAUSE_THRS: c_uint = 0x0B2003A4;
pub const QLA83XX_PORT1_RXB_PAUSE_THRS: c_uint = 0x0B2013A4;
pub const QLA83XX_PORT0_RXB_TC_MAX_CELL: c_uint = 0x0B200388;
pub const QLA83XX_PORT1_RXB_TC_MAX_CELL: c_uint = 0x0B201388;
pub const QLA83XX_PORT0_RXB_TC_STATS: c_uint = 0x0B20039C;
pub const QLA83XX_PORT1_RXB_TC_STATS: c_uint = 0x0B20139C;
pub const QLA83XX_PORT2_IFB_PAUSE_THRS: c_uint = 0x0B200704;
pub const QLA83XX_PORT3_IFB_PAUSE_THRS: c_uint = 0x0B201704;
// set value to pause threshold value
pub const QLA83XX_SET_PAUSE_VAL: c_uint = 0x0;
pub const QLA83XX_SET_TC_MAX_CELL_VAL: c_uint = 0x03FF03FF;
pub const QLA83XX_RESET_CONTROL: c_uint = 0x28084E50;
pub const QLA83XX_RESET_REG: c_uint = 0x28084E60;
pub const QLA83XX_RESET_PORT0: c_uint = 0x28084E70;
pub const QLA83XX_RESET_PORT1: c_uint = 0x28084E80;
pub const QLA83XX_RESET_PORT2: c_uint = 0x28084E90;
pub const QLA83XX_RESET_PORT3: c_uint = 0x28084EA0;
pub const QLA83XX_RESET_SRE_SHIM: c_uint = 0x28084EB0;
pub const QLA83XX_RESET_EPG_SHIM: c_uint = 0x28084EC0;
pub const QLA83XX_RESET_ETHER_PCS: c_uint = 0x28084ED0;
// qla_83xx_reg_tbl registers
pub const QLA83XX_PEG_HALT_STATUS1: c_uint = 0x34A8;
pub const QLA83XX_PEG_HALT_STATUS2: c_uint = 0x34AC;
pub const QLA83XX_PEG_ALIVE_COUNTER: c_uint = 0x34B0 /* FW_HEARTBEAT */;
pub const QLA83XX_FW_CAPABILITIES: c_uint = 0x3528;
pub const QLA83XX_CRB_DRV_ACTIVE: c_uint = 0x3788 /* IDC_DRV_PRESENCE */;
pub const QLA83XX_CRB_DEV_STATE: c_uint = 0x3784 /* IDC_DEV_STATE */;
pub const QLA83XX_CRB_DRV_STATE: c_uint = 0x378C /* IDC_DRV_ACK */;
pub const QLA83XX_CRB_DRV_SCRATCH: c_uint = 0x3548;
pub const QLA83XX_CRB_DEV_PART_INFO1: c_uint = 0x37E0;
pub const QLA83XX_CRB_DEV_PART_INFO2: c_uint = 0x37E4;
pub const QLA83XX_FW_VER_MAJOR: c_uint = 0x3550;
pub const QLA83XX_FW_VER_MINOR: c_uint = 0x3554;
pub const QLA83XX_FW_VER_SUB: c_uint = 0x3558;
pub const QLA83XX_NPAR_STATE: c_uint = 0x359C;
pub const QLA83XX_FW_IMAGE_VALID: c_uint = 0x35FC;
pub const QLA83XX_CMDPEG_STATE: c_uint = 0x3650;
pub const QLA83XX_ASIC_TEMP: c_uint = 0x37B4;
pub const QLA83XX_FW_API: c_uint = 0x356C;
pub const QLA83XX_DRV_OP_MODE: c_uint = 0x3570;
pub const QLA83XX_CRB_WIN_BASE: c_uint = 0x3800;

pub const QLA83XX_SEM_LOCK_BASE: c_uint = 0x3840;
pub const QLA83XX_SEM_UNLOCK_BASE: c_uint = 0x3844;

pub const QLA83XX_LINK_SPEED_FACTOR: c_int = 10;
// FLASH API Defines
pub const QLA83xx_FLASH_MAX_WAIT_USEC: c_int = 100;
pub const QLA83XX_FLASH_LOCK_TIMEOUT: c_int = 10000;
pub const QLA83XX_FLASH_SECTOR_SIZE: c_int = 65536;
pub const QLA83XX_DRV_LOCK_TIMEOUT: c_int = 2000;
pub const QLA83XX_FLASH_SECTOR_ERASE_CMD: c_uint = 0xdeadbeef;
pub const QLA83XX_FLASH_WRITE_CMD: c_uint = 0xdacdacda;
pub const QLA83XX_FLASH_BUFFER_WRITE_CMD: c_uint = 0xcadcadca;
pub const QLA83XX_FLASH_READ_RETRY_COUNT: c_int = 2000;
pub const QLA83XX_FLASH_STATUS_READY: c_uint = 0x6;
pub const QLA83XX_FLASH_BUFFER_WRITE_MIN: c_int = 2;
pub const QLA83XX_FLASH_BUFFER_WRITE_MAX: c_int = 64;
pub const QLA83XX_FLASH_STATUS_REG_POLL_DELAY: c_int = 1;
pub const QLA83XX_ERASE_MODE: c_int = 1;
pub const QLA83XX_WRITE_MODE: c_int = 2;
pub const QLA83XX_DWORD_WRITE_MODE: c_int = 3;
pub const QLA83XX_GLOBAL_RESET: c_uint = 0x38CC;
pub const QLA83XX_WILDCARD: c_uint = 0x38F0;
pub const QLA83XX_INFORMANT: c_uint = 0x38FC;
pub const QLA83XX_HOST_MBX_CTRL: c_uint = 0x3038;
pub const QLA83XX_FW_MBX_CTRL: c_uint = 0x303C;
pub const QLA83XX_BOOTLOADER_ADDR: c_uint = 0x355C;
pub const QLA83XX_BOOTLOADER_SIZE: c_uint = 0x3560;
pub const QLA83XX_FW_IMAGE_ADDR: c_uint = 0x3564;
pub const QLA83XX_MBX_INTR_ENABLE: c_uint = 0x1000;
pub const QLA83XX_MBX_INTR_MASK: c_uint = 0x1200;
// IDC Control Register bit defines
pub const DONTRESET_BIT0: c_uint = 0x1;
pub const GRACEFUL_RESET_BIT1: c_uint = 0x2;

// Firmware image definitions
pub const QLA83XX_BOOTLOADER_FLASH_ADDR: c_uint = 0x10000;
pub const QLA83XX_BOOT_FROM_FLASH: c_int = 0;
pub const QLA83XX_IDC_PARAM_ADDR: c_uint = 0x3e8020;
// Reset template definitions
pub const QLA83XX_MAX_RESET_SEQ_ENTRIES: c_int = 16;
pub const QLA83XX_RESTART_TEMPLATE_SIZE: c_uint = 0x2000;
pub const QLA83XX_RESET_TEMPLATE_ADDR: c_uint = 0x4F0000;
pub const QLA83XX_RESET_SEQ_VERSION: c_uint = 0x0101;
// Reset template entry opcodes
pub const OPCODE_NOP: c_uint = 0x0000;
pub const OPCODE_WRITE_LIST: c_uint = 0x0001;
pub const OPCODE_READ_WRITE_LIST: c_uint = 0x0002;
pub const OPCODE_POLL_LIST: c_uint = 0x0004;
pub const OPCODE_POLL_WRITE_LIST: c_uint = 0x0008;
pub const OPCODE_READ_MODIFY_WRITE: c_uint = 0x0010;
pub const OPCODE_SEQ_PAUSE: c_uint = 0x0020;
pub const OPCODE_SEQ_END: c_uint = 0x0040;
pub const OPCODE_TMPL_END: c_uint = 0x0080;
pub const OPCODE_POLL_READ_LIST: c_uint = 0x0100;
// Template Header
pub const RESET_TMPLT_HDR_SIGNATURE: c_uint = 0xCAFE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_reset_template_hdr {
    pub version: __le16,
    pub signature: __le16,
    pub size: __le16,
    pub entries: __le16,
    pub hdr_size: __le16,
    pub checksum: __le16,
    pub init_seq_offset: __le16,
    pub start_seq_offset: __le16,
    pub __packed: },
// Common Entry Header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_reset_entry_hdr {
    pub cmd: __le16,
    pub size: __le16,
    pub count: __le16,
    pub delay: __le16,
    pub __packed: },
// Generic poll entry type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_poll {
    pub test_mask: __le32,
    pub test_value: __le32,
    pub __packed: },
// Read modify write entry type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_rmw {
    pub test_mask: __le32,
    pub xor_value: __le32,
    pub or_value: __le32,
    pub shl: u8,
    pub shr: u8,
    pub index_a: u8,
    pub rsvd: u8,
    pub __packed: },
// Generic Entry Item with 2 DWords.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_entry {
    pub arg1: __le32,
    pub arg2: __le32,
    pub __packed: },
// Generic Entry Item with 4 DWords.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_quad_entry {
    pub dr_addr: __le32,
    pub dr_value: __le32,
    pub ar_addr: __le32,
    pub ar_value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_reset_template {
    pub seq_index: c_int,
    pub seq_error: c_int,
    pub array_index: c_int,
    pub array: [u32; QLA83XX_MAX_RESET_SEQ_ENTRIES],
    pub buff: *mut u8,
    pub stop_offset: *mut u8,
    pub start_offset: *mut u8,
    pub init_offset: *mut u8,
    pub hdr: *mut qla4_83xx_reset_template_hdr,
    pub seq_end: u8,
    pub template_end: u8,
}

// POLLRD Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla83xx_minidump_entry_pollrd {
    pub h: qla8xxx_minidump_entry_hdr,
    pub select_addr: u32,
    pub read_addr: u32,
    pub select_value: u32,
    pub select_value_stride: u16,
    pub op_count: u16,
    pub poll_wait: u32,
    pub poll_mask: u32,
    pub data_size: u32,
    pub rsvd_1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rddfe {
    pub h: qla8xxx_minidump_entry_hdr,
    pub addr_1: u32,
    pub value: u32,
    pub stride: u8,
    pub stride2: u8,
    pub count: u16,
    pub poll: u32,
    pub mask: u32,
    pub modify_mask: u32,
    pub data_size: u32,
    pub rsvd: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rdmdio {
    pub h: qla8xxx_minidump_entry_hdr,
    pub addr_1: u32,
    pub addr_2: u32,
    pub value_1: u32,
    pub stride_1: u8,
    pub stride_2: u8,
    pub count: u16,
    pub poll: u32,
    pub mask: u32,
    pub value_2: u32,
    pub data_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_pollwr {
    pub h: qla8xxx_minidump_entry_hdr,
    pub addr_1: u32,
    pub addr_2: u32,
    pub value_1: u32,
    pub value_2: u32,
    pub poll: u32,
    pub mask: u32,
    pub data_size: u32,
    pub rsvd: u32,
    pub __packed: },
// RDMUX2 Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla83xx_minidump_entry_rdmux2 {
    pub h: qla8xxx_minidump_entry_hdr,
    pub select_addr_1: u32,
    pub select_addr_2: u32,
    pub select_value_1: u32,
    pub select_value_2: u32,
    pub op_count: u32,
    pub select_value_mask: u32,
    pub read_addr: u32,
    pub select_value_stride: u8,
    pub data_size: u8,
    pub rsvd: [u8; 2],
}

// POLLRDMWR Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla83xx_minidump_entry_pollrdmwr {
    pub h: qla8xxx_minidump_entry_hdr,
    pub addr_1: u32,
    pub addr_2: u32,
    pub value_1: u32,
    pub value_2: u32,
    pub poll_wait: u32,
    pub poll_mask: u32,
    pub modify_mask: u32,
    pub data_size: u32,
}

// IDC additional information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_idc_information {
    pub /: *mut *mut uint32_t request_desc; / IDC request descriptor,
    pub /: *mut *mut uint32_t info1; / IDC additional info,
    pub /: *mut *mut uint32_t info2; / IDC additional info,
    pub /: *mut *mut uint32_t info3; / IDC additional info,
}

pub const QLA83XX_PEX_DMA_ENGINE_INDEX: c_int = 8;
pub const QLA83XX_PEX_DMA_BASE_ADDRESS: c_uint = 0x77320000;
pub const QLA83XX_PEX_DMA_NUM_OFFSET: c_uint = 0x10000;
pub const QLA83XX_PEX_DMA_CMD_ADDR_LOW: c_uint = 0x0;
pub const QLA83XX_PEX_DMA_CMD_ADDR_HIGH: c_uint = 0x04;
pub const QLA83XX_PEX_DMA_CMD_STS_AND_CNTRL: c_uint = 0x08;

// Read Memory: For Pex-DMA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_minidump_entry_rdmem_pex_dma {
    pub h: qla8xxx_minidump_entry_hdr,
    pub desc_card_addr: u32,
    pub dma_desc_cmd: u16,
    pub rsvd: [u8; 2],
    pub start_dma_cmd: u32,
    pub rsvd2: [u8; 12],
    pub read_addr: u32,
    pub read_data_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_83xx_pex_dma_descriptor {
    pub /: *mut *mut uint32_t read_data_size; / 0-23: size, 24-31: rsvd,
    pub rsvd: [u8; 2],
    pub dma_desc_cmd: u16,
    pub cmd: },
    pub src_addr: u64,
    pub pci-func,: *mut *mut uint64_t dma_bus_addr; / 0-3: desc-cmd, 4-7:,
// 8-15: desc-cmd
    pub rsvd: [u8; 24],
    pub __packed: },
