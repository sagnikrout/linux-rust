//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_nx2.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//
pub const QSNT_ACK_TOV: c_int = 30;
pub const INTENT_TO_RECOVER: c_uint = 0x01;
pub const PROCEED_TO_RECOVER: c_uint = 0x02;
pub const IDC_LOCK_RECOVERY_OWNER_MASK: c_uint = 0x3C;
pub const IDC_LOCK_RECOVERY_STATE_MASK: c_uint = 0x3;
pub const IDC_LOCK_RECOVERY_STATE_SHIFT_BITS: c_int = 2;
pub const QLA8044_DRV_LOCK_MSLEEP: c_int = 200;

pub const MD_MIU_TEST_AGT_WRDATA_LO: c_uint = 0x410000A0;
pub const MD_MIU_TEST_AGT_WRDATA_HI: c_uint = 0x410000A4;
pub const MD_MIU_TEST_AGT_WRDATA_ULO: c_uint = 0x410000B0;
pub const MD_MIU_TEST_AGT_WRDATA_UHI: c_uint = 0x410000B4;
// MIU_TEST_AGT_CTRL flags. work for SIU as well

// Imbus address bit used to indicate a host address. This bit is
// eliminated by the pcie bar and bar select before presentation
// over pcie.
// host memory via IMBUS

// PCI Windowing for DDR regions.
// Indirectly Mapped Registers
pub const QLA8044_FLASH_SPI_STATUS: c_uint = 0x2808E010;
pub const QLA8044_FLASH_SPI_CONTROL: c_uint = 0x2808E014;
pub const QLA8044_FLASH_STATUS: c_uint = 0x42100004;
pub const QLA8044_FLASH_CONTROL: c_uint = 0x42110004;
pub const QLA8044_FLASH_ADDR: c_uint = 0x42110008;
pub const QLA8044_FLASH_WRDATA: c_uint = 0x4211000C;
pub const QLA8044_FLASH_RDDATA: c_uint = 0x42110018;
pub const QLA8044_FLASH_DIRECT_WINDOW: c_uint = 0x42110030;

// Flash access regs
pub const QLA8044_FLASH_LOCK: c_uint = 0x3850;
pub const QLA8044_FLASH_UNLOCK: c_uint = 0x3854;
pub const QLA8044_FLASH_LOCK_ID: c_uint = 0x3500;
// Driver Lock regs
pub const QLA8044_DRV_LOCK: c_uint = 0x3868;
pub const QLA8044_DRV_UNLOCK: c_uint = 0x386C;
pub const QLA8044_DRV_LOCK_ID: c_uint = 0x3504;
pub const QLA8044_DRV_LOCKRECOVERY: c_uint = 0x379C;
// IDC version
pub const QLA8044_IDC_VER_MAJ_VALUE: c_uint = 0x1;
pub const QLA8044_IDC_VER_MIN_VALUE: c_uint = 0x0;
// IDC Registers : Driver Coexistence Defines
pub const QLA8044_CRB_IDC_VER_MAJOR: c_uint = 0x3780;
pub const QLA8044_CRB_IDC_VER_MINOR: c_uint = 0x3798;
pub const QLA8044_IDC_DRV_AUDIT: c_uint = 0x3794;
pub const QLA8044_SRE_SHIM_CONTROL: c_uint = 0x0D200284;
pub const QLA8044_PORT0_RXB_PAUSE_THRS: c_uint = 0x0B2003A4;
pub const QLA8044_PORT1_RXB_PAUSE_THRS: c_uint = 0x0B2013A4;
pub const QLA8044_PORT0_RXB_TC_MAX_CELL: c_uint = 0x0B200388;
pub const QLA8044_PORT1_RXB_TC_MAX_CELL: c_uint = 0x0B201388;
pub const QLA8044_PORT0_RXB_TC_STATS: c_uint = 0x0B20039C;
pub const QLA8044_PORT1_RXB_TC_STATS: c_uint = 0x0B20139C;
pub const QLA8044_PORT2_IFB_PAUSE_THRS: c_uint = 0x0B200704;
pub const QLA8044_PORT3_IFB_PAUSE_THRS: c_uint = 0x0B201704;
// set value to pause threshold value
pub const QLA8044_SET_PAUSE_VAL: c_uint = 0x0;
pub const QLA8044_SET_TC_MAX_CELL_VAL: c_uint = 0x03FF03FF;
pub const QLA8044_PEG_HALT_STATUS1: c_uint = 0x34A8;
pub const QLA8044_PEG_HALT_STATUS2: c_uint = 0x34AC;
pub const QLA8044_PEG_ALIVE_COUNTER: c_uint = 0x34B0 /* FW_HEARTBEAT */;
pub const QLA8044_FW_CAPABILITIES: c_uint = 0x3528;
pub const QLA8044_CRB_DRV_ACTIVE: c_uint = 0x3788 /* IDC_DRV_PRESENCE */;
pub const QLA8044_CRB_DEV_STATE: c_uint = 0x3784 /* IDC_DEV_STATE */;
pub const QLA8044_CRB_DRV_STATE: c_uint = 0x378C /* IDC_DRV_ACK */;
pub const QLA8044_CRB_DRV_SCRATCH: c_uint = 0x3548;
pub const QLA8044_CRB_DEV_PART_INFO1: c_uint = 0x37E0;
pub const QLA8044_CRB_DEV_PART_INFO2: c_uint = 0x37E4;
pub const QLA8044_FW_VER_MAJOR: c_uint = 0x3550;
pub const QLA8044_FW_VER_MINOR: c_uint = 0x3554;
pub const QLA8044_FW_VER_SUB: c_uint = 0x3558;
pub const QLA8044_NPAR_STATE: c_uint = 0x359C;
pub const QLA8044_FW_IMAGE_VALID: c_uint = 0x35FC;
pub const QLA8044_CMDPEG_STATE: c_uint = 0x3650;
pub const QLA8044_ASIC_TEMP: c_uint = 0x37B4;
pub const QLA8044_FW_API: c_uint = 0x356C;
pub const QLA8044_DRV_OP_MODE: c_uint = 0x3570;
pub const QLA8044_CRB_WIN_BASE: c_uint = 0x3800;

pub const QLA8044_SEM_LOCK_BASE: c_uint = 0x3840;
pub const QLA8044_SEM_UNLOCK_BASE: c_uint = 0x3844;

pub const QLA8044_LINK_SPEED_FACTOR: c_int = 10;
pub const QLA8044_FUN7_ACTIVE_INDEX: c_uint = 0x80;
// FLASH API Defines
pub const QLA8044_FLASH_MAX_WAIT_USEC: c_int = 100;
pub const QLA8044_FLASH_LOCK_TIMEOUT: c_int = 10000;
pub const QLA8044_FLASH_SECTOR_SIZE: c_int = 65536;
pub const QLA8044_DRV_LOCK_TIMEOUT: c_int = 2000;
pub const QLA8044_FLASH_SECTOR_ERASE_CMD: c_uint = 0xdeadbeef;
pub const QLA8044_FLASH_WRITE_CMD: c_uint = 0xdacdacda;
pub const QLA8044_FLASH_BUFFER_WRITE_CMD: c_uint = 0xcadcadca;
pub const QLA8044_FLASH_READ_RETRY_COUNT: c_int = 2000;
pub const QLA8044_FLASH_STATUS_READY: c_uint = 0x6;
pub const QLA8044_FLASH_BUFFER_WRITE_MIN: c_int = 2;
pub const QLA8044_FLASH_BUFFER_WRITE_MAX: c_int = 64;
pub const QLA8044_FLASH_STATUS_REG_POLL_DELAY: c_int = 1;
pub const QLA8044_ERASE_MODE: c_int = 1;
pub const QLA8044_WRITE_MODE: c_int = 2;
pub const QLA8044_DWORD_WRITE_MODE: c_int = 3;
pub const QLA8044_GLOBAL_RESET: c_uint = 0x38CC;
pub const QLA8044_WILDCARD: c_uint = 0x38F0;
pub const QLA8044_INFORMANT: c_uint = 0x38FC;
pub const QLA8044_HOST_MBX_CTRL: c_uint = 0x3038;
pub const QLA8044_FW_MBX_CTRL: c_uint = 0x303C;
pub const QLA8044_BOOTLOADER_ADDR: c_uint = 0x355C;
pub const QLA8044_BOOTLOADER_SIZE: c_uint = 0x3560;
pub const QLA8044_FW_IMAGE_ADDR: c_uint = 0x3564;
pub const QLA8044_MBX_INTR_ENABLE: c_uint = 0x1000;
pub const QLA8044_MBX_INTR_MASK: c_uint = 0x1200;
// IDC Control Register bit defines
pub const DONTRESET_BIT0: c_uint = 0x1;
pub const GRACEFUL_RESET_BIT1: c_uint = 0x2;
// ISP8044 PEG_HALT_STATUS1 bits

// Firmware image definitions
pub const QLA8044_BOOTLOADER_FLASH_ADDR: c_uint = 0x10000;
pub const QLA8044_BOOT_FROM_FLASH: c_int = 0;
pub const QLA8044_IDC_PARAM_ADDR: c_uint = 0x3e8020;
// FLASH related definitions
pub const QLA8044_OPTROM_BURST_SIZE: c_uint = 0x100;

pub const QLA8044_MIN_OPTROM_BURST_DWORDS: c_int = 2;

pub const QLA8044_FLASH_SPI_CTL: c_uint = 0x4;
pub const QLA8044_FLASH_FIRST_TEMP_VAL: c_uint = 0x00800000;
pub const QLA8044_FLASH_SECOND_TEMP_VAL: c_uint = 0x00800001;
pub const QLA8044_FLASH_FIRST_MS_PATTERN: c_uint = 0x43;
pub const QLA8044_FLASH_SECOND_MS_PATTERN: c_uint = 0x7F;
pub const QLA8044_FLASH_LAST_MS_PATTERN: c_uint = 0x7D;
pub const QLA8044_FLASH_STATUS_WRITE_DEF_SIG: c_uint = 0xFD0100;
pub const QLA8044_FLASH_SECOND_ERASE_MS_VAL: c_uint = 0x5;
pub const QLA8044_FLASH_ERASE_SIG: c_uint = 0xFD0300;
pub const QLA8044_FLASH_LAST_ERASE_MS_VAL: c_uint = 0x3D;
// Reset template definitions
pub const QLA8044_MAX_RESET_SEQ_ENTRIES: c_int = 16;
pub const QLA8044_RESTART_TEMPLATE_SIZE: c_uint = 0x2000;
pub const QLA8044_RESET_TEMPLATE_ADDR: c_uint = 0x4F0000;
pub const QLA8044_RESET_SEQ_VERSION: c_uint = 0x0101;
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
pub const QLA8044_IDC_DRV_CTRL: c_uint = 0x3790;

pub const MINIDUMP_SIZE_36K: c_int = 36864;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_reset_template_hdr {
    pub version: u16,
    pub signature: u16,
    pub size: u16,
    pub entries: u16,
    pub hdr_size: u16,
    pub checksum: u16,
    pub init_seq_offset: u16,
    pub start_seq_offset: u16,
    pub __packed: },
// Common Entry Header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_reset_entry_hdr {
    pub cmd: u16,
    pub size: u16,
    pub count: u16,
    pub delay: u16,
    pub __packed: },
// Generic poll entry type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_poll {
    pub test_mask: u32,
    pub test_value: u32,
    pub __packed: },
// Read modify write entry type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_rmw {
    pub test_mask: u32,
    pub xor_value: u32,
    pub or_value: u32,
    pub shl: u8,
    pub shr: u8,
    pub index_a: u8,
    pub rsvd: u8,
    pub __packed: },
// Generic Entry Item with 2 DWords.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_entry {
    pub arg1: u32,
    pub arg2: u32,
    pub __packed: },
// Generic Entry Item with 4 DWords.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_quad_entry {
    pub dr_addr: u32,
    pub dr_value: u32,
    pub ar_addr: u32,
    pub ar_value: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_reset_template {
    pub seq_index: c_int,
    pub seq_error: c_int,
    pub array_index: c_int,
    pub array: [u32; QLA8044_MAX_RESET_SEQ_ENTRIES],
    pub buff: *mut u8,
    pub stop_offset: *mut u8,
    pub start_offset: *mut u8,
    pub init_offset: *mut u8,
    pub hdr: *mut qla8044_reset_template_hdr,
    pub seq_end: u8,
    pub template_end: u8,
}

// Driver_code is for driver to write some info about the entry
// currently not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_hdr {
    pub entry_type: u32,
    pub entry_size: u32,
    pub entry_capture_size: u32,
    pub entry_capture_mask: u8,
    pub entry_code: u8,
    pub driver_code: u8,
    pub driver_flags: u8,
    pub d_ctrl: },
    pub __packed: },
// Read CRB entry header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_crb {
    pub h: qla8044_minidump_entry_hdr,
    pub addr: u32,
    pub addr_stride: u8,
    pub state_index_a: u8,
    pub poll_timeout: u16,
    pub crb_strd: },
    pub data_size: u32,
    pub op_count: u32,
    pub opcode: u8,
    pub state_index_v: u8,
    pub shl: u8,
    pub shr: u8,
    pub crb_ctrl: },
    pub value_1: u32,
    pub value_2: u32,
    pub value_3: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_cache {
    pub h: qla8044_minidump_entry_hdr,
    pub tag_reg_addr: u32,
    pub tag_value_stride: u16,
    pub init_tag_value: u16,
    pub addr_ctrl: },
    pub data_size: u32,
    pub op_count: u32,
    pub control_addr: u32,
    pub write_value: u16,
    pub poll_mask: u8,
    pub poll_wait: u8,
    pub cache_ctrl: },
    pub read_addr: u32,
    pub read_addr_stride: u8,
    pub read_addr_cnt: u8,
    pub rsvd_1: u16,
    pub read_ctrl: },
    pub __packed: },
// Read OCM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rdocm {
    pub h: qla8044_minidump_entry_hdr,
    pub rsvd_0: u32,
    pub rsvd_1: u32,
    pub data_size: u32,
    pub op_count: u32,
    pub rsvd_2: u32,
    pub rsvd_3: u32,
    pub read_addr: u32,
    pub read_addr_stride: u32,
    pub __packed: },
// Read Memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rdmem {
    pub h: qla8044_minidump_entry_hdr,
    pub rsvd: [u32; 6],
    pub read_addr: u32,
    pub read_data_size: u32,
}

// Read Memory: For Pex-DMA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rdmem_pex_dma {
    pub h: qla8044_minidump_entry_hdr,
    pub desc_card_addr: u32,
    pub dma_desc_cmd: u16,
    pub rsvd: [u8; 2],
    pub start_dma_cmd: u32,
    pub rsvd2: [u8; 12],
    pub read_addr: u32,
    pub read_data_size: u32,
    pub __packed: },
// Read ROM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rdrom {
    pub h: qla8044_minidump_entry_hdr,
    pub rsvd: [u32; 6],
    pub read_addr: u32,
    pub read_data_size: u32,
    pub __packed: },
// Mux entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_mux {
    pub h: qla8044_minidump_entry_hdr,
    pub select_addr: u32,
    pub rsvd_0: u32,
    pub data_size: u32,
    pub op_count: u32,
    pub select_value: u32,
    pub select_value_stride: u32,
    pub read_addr: u32,
    pub rsvd_1: u32,
    pub __packed: },
// Queue entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_queue {
    pub h: qla8044_minidump_entry_hdr,
    pub select_addr: u32,
    pub queue_id_stride: u16,
    pub rsvd_0: u16,
    pub q_strd: },
    pub data_size: u32,
    pub op_count: u32,
    pub rsvd_1: u32,
    pub rsvd_2: u32,
    pub read_addr: u32,
    pub read_addr_stride: u8,
    pub read_addr_cnt: u8,
    pub rsvd_3: u16,
    pub rd_strd: },
    pub __packed: },
// POLLRD Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_pollrd {
    pub h: qla8044_minidump_entry_hdr,
    pub select_addr: u32,
    pub read_addr: u32,
    pub select_value: u32,
    pub select_value_stride: u16,
    pub op_count: u16,
    pub poll_wait: u32,
    pub poll_mask: u32,
    pub data_size: u32,
    pub rsvd_1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_rddfe {
    pub h: qla8044_minidump_entry_hdr,
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
    pub h: qla8044_minidump_entry_hdr,
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
    pub h: qla8044_minidump_entry_hdr,
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
pub struct qla8044_minidump_entry_rdmux2 {
    pub h: qla8044_minidump_entry_hdr,
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
    pub __packed: },
// POLLRDMWR Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_minidump_entry_pollrdmwr {
    pub h: qla8044_minidump_entry_hdr,
    pub addr_1: u32,
    pub addr_2: u32,
    pub value_1: u32,
    pub value_2: u32,
    pub poll_wait: u32,
    pub poll_mask: u32,
    pub modify_mask: u32,
    pub data_size: u32,
    pub __packed: },
// IDC additional information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_idc_information {
    pub /: *mut *mut uint32_t request_desc; / IDC request descriptor,
    pub /: *mut *mut uint32_t info1; / IDC additional info,
    pub /: *mut *mut uint32_t info2; / IDC additional info,
    pub /: *mut *mut uint32_t info3; / IDC additional info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qla_regs {
    QLA8044_PEG_HALT_STATUS1_INDEX = 0,
    QLA8044_PEG_HALT_STATUS2_INDEX,
    QLA8044_PEG_ALIVE_COUNTER_INDEX,
    QLA8044_CRB_DRV_ACTIVE_INDEX,
    QLA8044_CRB_DEV_STATE_INDEX,
    QLA8044_CRB_DRV_STATE_INDEX,
    QLA8044_CRB_DRV_SCRATCH_INDEX,
    QLA8044_CRB_DEV_PART_INFO_INDEX,
    QLA8044_CRB_DRV_IDC_VERSION_INDEX,
    QLA8044_FW_VERSION_MAJOR_INDEX,
    QLA8044_FW_VERSION_MINOR_INDEX,
    QLA8044_FW_VERSION_SUB_INDEX,
    QLA8044_CRB_CMDPEG_STATE_INDEX,
    QLA8044_CRB_TEMP_STATE_INDEX,
    } __packed;

pub const CRB_REG_INDEX_MAX: c_int = 14;
pub const CRB_CMDPEG_CHECK_RETRY_COUNT: c_int = 60;
pub const CRB_CMDPEG_CHECK_DELAY: c_int = 500;

// MiniDump Structures

// Driver_code is for driver to write some info about the entry
// currently not used.
//
pub const QLA8044_SS_OCM_WNDREG_INDEX: c_int = 3;
pub const QLA8044_DBG_STATE_ARRAY_LEN: c_int = 16;
pub const QLA8044_DBG_CAP_SIZE_ARRAY_LEN: c_int = 8;
pub const QLA8044_DBG_RSVD_ARRAY_LEN: c_int = 8;
pub const QLA8044_DBG_OCM_WNDREG_ARRAY_LEN: c_int = 16;
pub const QLA8044_SS_PCI_INDEX: c_int = 0;
pub const QLA8044_RDDFE: c_int = 38;
pub const QLA8044_RDMDIO: c_int = 39;
pub const QLA8044_POLLWR: c_int = 40;

    struct qla8044_minidump_template_hdr {
    uint32_t entry_type;
    uint32_t first_entry_offset;
    uint32_t size_of_template;
    uint32_t capture_debug_level;
    uint32_t num_of_entries;
    uint32_t version;
    uint32_t driver_timestamp;
    uint32_t checksum;

    uint32_t driver_capture_mask;
    uint32_t driver_info_word2;
    uint32_t driver_info_word3;
    uint32_t driver_info_word4;

    uint32_t saved_state_array[QLA8044_DBG_STATE_ARRAY_LEN];
    uint32_t capture_size_array[QLA8044_DBG_CAP_SIZE_ARRAY_LEN];
    uint32_t ocm_window_reg[QLA8044_DBG_OCM_WNDREG_ARRAY_LEN];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla8044_pex_dma_descriptor {
    pub /: *mut *mut uint32_t read_data_size; / 0-23: size, 24-31: rsvd,
    pub rsvd: [u8; 2],
    pub dma_desc_cmd: u16,
    pub cmd: },
    pub src_addr: u64,
    pub desc-cmd*/: *mut *mut uint64_t dma_bus_addr; /0-3: desc-cmd, 4-7: pci-func, 8-15:,
    pub rsvd: [u8; 24],
    pub __packed: },
