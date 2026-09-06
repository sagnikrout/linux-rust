//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_dbg_hsi.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2019-2021 Marvell International Ltd.
//

//
// Debug Tools HSI constants and macros
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum block_id {
    BLOCK_GRC,
    BLOCK_MISCS,
    BLOCK_MISC,
    BLOCK_DBU,
    BLOCK_PGLUE_B,
    BLOCK_CNIG,
    BLOCK_CPMU,
    BLOCK_NCSI,
    BLOCK_OPTE,
    BLOCK_BMB,
    BLOCK_PCIE,
    BLOCK_MCP,
    BLOCK_MCP2,
    BLOCK_PSWHST,
    BLOCK_PSWHST2,
    BLOCK_PSWRD,
    BLOCK_PSWRD2,
    BLOCK_PSWWR,
    BLOCK_PSWWR2,
    BLOCK_PSWRQ,
    BLOCK_PSWRQ2,
    BLOCK_PGLCS,
    BLOCK_DMAE,
    BLOCK_PTU,
    BLOCK_TCM,
    BLOCK_MCM,
    BLOCK_UCM,
    BLOCK_XCM,
    BLOCK_YCM,
    BLOCK_PCM,
    BLOCK_QM,
    BLOCK_TM,
    BLOCK_DORQ,
    BLOCK_BRB,
    BLOCK_SRC,
    BLOCK_PRS,
    BLOCK_TSDM,
    BLOCK_MSDM,
    BLOCK_USDM,
    BLOCK_XSDM,
    BLOCK_YSDM,
    BLOCK_PSDM,
    BLOCK_TSEM,
    BLOCK_MSEM,
    BLOCK_USEM,
    BLOCK_XSEM,
    BLOCK_YSEM,
    BLOCK_PSEM,
    BLOCK_RSS,
    BLOCK_TMLD,
    BLOCK_MULD,
    BLOCK_YULD,
    BLOCK_XYLD,
    BLOCK_PRM,
    BLOCK_PBF_PB1,
    BLOCK_PBF_PB2,
    BLOCK_RPB,
    BLOCK_BTB,
    BLOCK_PBF,
    BLOCK_RDIF,
    BLOCK_TDIF,
    BLOCK_CDU,
    BLOCK_CCFC,
    BLOCK_TCFC,
    BLOCK_IGU,
    BLOCK_CAU,
    BLOCK_UMAC,
    BLOCK_XMAC,
    BLOCK_MSTAT,
    BLOCK_DBG,
    BLOCK_NIG,
    BLOCK_WOL,
    BLOCK_BMBN,
    BLOCK_IPC,
    BLOCK_NWM,
    BLOCK_NWS,
    BLOCK_MS,
    BLOCK_PHY_PCIE,
    BLOCK_LED,
    BLOCK_AVS_WRAP,
    BLOCK_PXPREQBUS,
    BLOCK_BAR0_MAP,
    BLOCK_MCP_FIO,
    BLOCK_LAST_INIT,
    BLOCK_PRS_FC,
    BLOCK_PBF_FC,
    BLOCK_NIG_LB_FC,
    BLOCK_NIG_LB_FC_PLLH,
    BLOCK_NIG_TX_FC_PLLH,
    BLOCK_NIG_TX_FC,
    BLOCK_NIG_RX_FC_PLLH,
    BLOCK_NIG_RX_FC,
    MAX_BLOCK_ID
}

// binary debug buffer types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bin_dbg_buffer_type {
    BIN_BUF_DBG_MODE_TREE,
    BIN_BUF_DBG_DUMP_REG,
    BIN_BUF_DBG_DUMP_MEM,
    BIN_BUF_DBG_IDLE_CHK_REGS,
    BIN_BUF_DBG_IDLE_CHK_IMMS,
    BIN_BUF_DBG_IDLE_CHK_RULES,
    BIN_BUF_DBG_IDLE_CHK_PARSING_DATA,
    BIN_BUF_DBG_ATTN_BLOCKS,
    BIN_BUF_DBG_ATTN_REGS,
    BIN_BUF_DBG_ATTN_INDEXES,
    BIN_BUF_DBG_ATTN_NAME_OFFSETS,
    BIN_BUF_DBG_BLOCKS,
    BIN_BUF_DBG_BLOCKS_CHIP_DATA,
    BIN_BUF_DBG_BUS_LINES,
    BIN_BUF_DBG_BLOCKS_USER_DATA,
    BIN_BUF_DBG_BLOCKS_CHIP_USER_DATA,
    BIN_BUF_DBG_BUS_LINE_NAME_OFFSETS,
    BIN_BUF_DBG_RESET_REGS,
    BIN_BUF_DBG_PARSING_STRINGS,
    MAX_BIN_DBG_BUFFER_TYPE
}

// Attention bit mapping
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_attn_bit_mapping {
    pub data: u16,
pub const DBG_ATTN_BIT_MAPPING_VAL_MASK: c_uint = 0x7FFF;
pub const DBG_ATTN_BIT_MAPPING_VAL_SHIFT: c_int = 0;
pub const DBG_ATTN_BIT_MAPPING_IS_UNUSED_BIT_CNT_MASK: c_uint = 0x1;
pub const DBG_ATTN_BIT_MAPPING_IS_UNUSED_BIT_CNT_SHIFT: c_int = 15;
}

// Attention block per-type data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_attn_block_type_data {
    pub names_offset: u16,
    pub reserved1: u16,
    pub num_regs: u8,
    pub reserved2: u8,
    pub regs_offset: u16,
}

// Block attentions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_attn_block {
    pub per_type_data: [dbg_attn_block_type_data; 2],
}

// Attention register result
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_attn_reg_result {
    pub data: u32,
pub const DBG_ATTN_REG_RESULT_STS_ADDRESS_MASK: c_uint = 0xFFFFFF;
pub const DBG_ATTN_REG_RESULT_STS_ADDRESS_SHIFT: c_int = 0;
pub const DBG_ATTN_REG_RESULT_NUM_REG_ATTN_MASK: c_uint = 0xFF;
pub const DBG_ATTN_REG_RESULT_NUM_REG_ATTN_SHIFT: c_int = 24;
    pub block_attn_offset: u16,
    pub reserved: u16,
    pub sts_val: u32,
    pub mask_val: u32,
}

// Attention block result
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_attn_block_result {
    pub block_id: u8,
    pub data: u8,
pub const DBG_ATTN_BLOCK_RESULT_ATTN_TYPE_MASK: c_uint = 0x3;
pub const DBG_ATTN_BLOCK_RESULT_ATTN_TYPE_SHIFT: c_int = 0;
pub const DBG_ATTN_BLOCK_RESULT_NUM_REGS_MASK: c_uint = 0x3F;
pub const DBG_ATTN_BLOCK_RESULT_NUM_REGS_SHIFT: c_int = 2;
    pub names_offset: u16,
    pub reg_results: [dbg_attn_reg_result; 15],
}

// Mode header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_mode_hdr {
    pub data: u16,
pub const DBG_MODE_HDR_EVAL_MODE_MASK: c_uint = 0x1;
pub const DBG_MODE_HDR_EVAL_MODE_SHIFT: c_int = 0;
pub const DBG_MODE_HDR_MODES_BUF_OFFSET_MASK: c_uint = 0x7FFF;
pub const DBG_MODE_HDR_MODES_BUF_OFFSET_SHIFT: c_int = 1;
}

// Attention register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_attn_reg {
    pub mode: dbg_mode_hdr,
    pub block_attn_offset: u16,
    pub data: u32,
pub const DBG_ATTN_REG_STS_ADDRESS_MASK: c_uint = 0xFFFFFF;
pub const DBG_ATTN_REG_STS_ADDRESS_SHIFT: c_int = 0;
pub const DBG_ATTN_REG_NUM_REG_ATTN_MASK: c_uint = 0xFF;
pub const DBG_ATTN_REG_NUM_REG_ATTN_SHIFT: c_int = 24;
    pub sts_clr_address: u32,
    pub mask_address: u32,
}

// Attention types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_attn_type {
    ATTN_TYPE_INTERRUPT,
    ATTN_TYPE_PARITY,
    MAX_DBG_ATTN_TYPE
}

// Block debug data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_block {
    pub name: [u8; 15],
    pub associated_storm_letter: u8,
}

// Chip-specific block debug data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_block_chip {
    pub flags: u8,
pub const DBG_BLOCK_CHIP_IS_REMOVED_MASK: c_uint = 0x1;
pub const DBG_BLOCK_CHIP_IS_REMOVED_SHIFT: c_int = 0;
pub const DBG_BLOCK_CHIP_HAS_RESET_REG_MASK: c_uint = 0x1;
pub const DBG_BLOCK_CHIP_HAS_RESET_REG_SHIFT: c_int = 1;
pub const DBG_BLOCK_CHIP_UNRESET_BEFORE_DUMP_MASK: c_uint = 0x1;
pub const DBG_BLOCK_CHIP_UNRESET_BEFORE_DUMP_SHIFT: c_int = 2;
pub const DBG_BLOCK_CHIP_HAS_DBG_BUS_MASK: c_uint = 0x1;
pub const DBG_BLOCK_CHIP_HAS_DBG_BUS_SHIFT: c_int = 3;
pub const DBG_BLOCK_CHIP_HAS_LATENCY_EVENTS_MASK: c_uint = 0x1;
pub const DBG_BLOCK_CHIP_HAS_LATENCY_EVENTS_SHIFT: c_int = 4;
pub const DBG_BLOCK_CHIP_RESERVED0_MASK: c_uint = 0x7;
pub const DBG_BLOCK_CHIP_RESERVED0_SHIFT: c_int = 5;
    pub dbg_client_id: u8,
    pub reset_reg_id: u8,
    pub reset_reg_bit_offset: u8,
    pub dbg_bus_mode: dbg_mode_hdr,
    pub reserved1: u16,
    pub reserved2: u8,
    pub num_of_dbg_bus_lines: u8,
    pub dbg_bus_lines_offset: u16,
    pub dbg_select_reg_addr: u32,
    pub dbg_dword_enable_reg_addr: u32,
    pub dbg_shift_reg_addr: u32,
    pub dbg_force_valid_reg_addr: u32,
    pub dbg_force_frame_reg_addr: u32,
}

// Chip-specific block user debug data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_block_chip_user {
    pub num_of_dbg_bus_lines: u8,
    pub has_latency_events: u8,
    pub names_offset: u16,
}

// Block user debug data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_block_user {
    pub name: [u8; 16],
}

// Block Debug line data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_line {
    pub data: u8,
pub const DBG_BUS_LINE_NUM_OF_GROUPS_MASK: c_uint = 0xF;
pub const DBG_BUS_LINE_NUM_OF_GROUPS_SHIFT: c_int = 0;
pub const DBG_BUS_LINE_IS_256B_MASK: c_uint = 0x1;
pub const DBG_BUS_LINE_IS_256B_SHIFT: c_int = 4;
pub const DBG_BUS_LINE_RESERVED_MASK: c_uint = 0x7;
pub const DBG_BUS_LINE_RESERVED_SHIFT: c_int = 5;
    pub group_sizes: u8,
}

// Condition header for registers dump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_dump_cond_hdr {
    pub /: *mut *mut dbg_mode_hdr mode; / Mode header,
    pub /: *mut *mut u8 block_id; / block ID,
    pub /: *mut *mut u8 data_size; / size in dwords of the data following this header,
}

// Memory data for registers dump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_dump_mem {
    pub dword0: u32,
pub const DBG_DUMP_MEM_ADDRESS_MASK: c_uint = 0xFFFFFF;
pub const DBG_DUMP_MEM_ADDRESS_SHIFT: c_int = 0;
pub const DBG_DUMP_MEM_MEM_GROUP_ID_MASK: c_uint = 0xFF;
pub const DBG_DUMP_MEM_MEM_GROUP_ID_SHIFT: c_int = 24;
    pub dword1: u32,
pub const DBG_DUMP_MEM_LENGTH_MASK: c_uint = 0xFFFFFF;
pub const DBG_DUMP_MEM_LENGTH_SHIFT: c_int = 0;
pub const DBG_DUMP_MEM_WIDE_BUS_MASK: c_uint = 0x1;
pub const DBG_DUMP_MEM_WIDE_BUS_SHIFT: c_int = 24;
pub const DBG_DUMP_MEM_RESERVED_MASK: c_uint = 0x7F;
pub const DBG_DUMP_MEM_RESERVED_SHIFT: c_int = 25;
}

// Register data for registers dump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_dump_reg {
    pub data: u32,
pub const DBG_DUMP_REG_ADDRESS_MASK: c_uint = 0x7FFFFF;
pub const DBG_DUMP_REG_ADDRESS_SHIFT: c_int = 0;
pub const DBG_DUMP_REG_WIDE_BUS_MASK: c_uint = 0x1;
pub const DBG_DUMP_REG_WIDE_BUS_SHIFT: c_int = 23;
pub const DBG_DUMP_REG_LENGTH_MASK: c_uint = 0xFF;
pub const DBG_DUMP_REG_LENGTH_SHIFT: c_int = 24;
}

// Split header for registers dump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_dump_split_hdr {
    pub hdr: u32,
pub const DBG_DUMP_SPLIT_HDR_DATA_SIZE_MASK: c_uint = 0xFFFFFF;
pub const DBG_DUMP_SPLIT_HDR_DATA_SIZE_SHIFT: c_int = 0;
pub const DBG_DUMP_SPLIT_HDR_SPLIT_TYPE_ID_MASK: c_uint = 0xFF;
pub const DBG_DUMP_SPLIT_HDR_SPLIT_TYPE_ID_SHIFT: c_int = 24;
}

// Condition header for idle check
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_cond_hdr {
    pub /: *mut *mut dbg_mode_hdr mode; / Mode header,
    pub /: *mut *mut u16 data_size; / size in dwords of the data following this header,
}

// Idle Check condition register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_cond_reg {
    pub data: u32,
pub const DBG_IDLE_CHK_COND_REG_ADDRESS_MASK: c_uint = 0x7FFFFF;
pub const DBG_IDLE_CHK_COND_REG_ADDRESS_SHIFT: c_int = 0;
pub const DBG_IDLE_CHK_COND_REG_WIDE_BUS_MASK: c_uint = 0x1;
pub const DBG_IDLE_CHK_COND_REG_WIDE_BUS_SHIFT: c_int = 23;
pub const DBG_IDLE_CHK_COND_REG_BLOCK_ID_MASK: c_uint = 0xFF;
pub const DBG_IDLE_CHK_COND_REG_BLOCK_ID_SHIFT: c_int = 24;
    pub num_entries: u16,
    pub entry_size: u8,
    pub start_entry: u8,
}

// Idle Check info register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_info_reg {
    pub data: u32,
pub const DBG_IDLE_CHK_INFO_REG_ADDRESS_MASK: c_uint = 0x7FFFFF;
pub const DBG_IDLE_CHK_INFO_REG_ADDRESS_SHIFT: c_int = 0;
pub const DBG_IDLE_CHK_INFO_REG_WIDE_BUS_MASK: c_uint = 0x1;
pub const DBG_IDLE_CHK_INFO_REG_WIDE_BUS_SHIFT: c_int = 23;
pub const DBG_IDLE_CHK_INFO_REG_BLOCK_ID_MASK: c_uint = 0xFF;
pub const DBG_IDLE_CHK_INFO_REG_BLOCK_ID_SHIFT: c_int = 24;
    pub /: *mut *mut u16 size; / register size in dwords,
    pub /: *mut *mut dbg_mode_hdr mode; / Mode header,
}

// Idle Check register
#[repr(C)]
#[derive(Copy, Clone)]
pub union dbg_idle_chk_reg {
    pub /: *mut *mut dbg_idle_chk_cond_reg cond_reg; / condition register,
    pub /: *mut *mut dbg_idle_chk_info_reg info_reg; / info register,
}

// Idle Check result header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_result_hdr {
    pub /: *mut *mut u16 rule_id; / Failing rule index,
    pub /: *mut *mut u16 mem_entry_id; / Failing memory entry index,
    pub /: *mut *mut u8 num_dumped_cond_regs; / number of dumped condition registers,
    pub /: *mut *mut u8 num_dumped_info_regs; / number of dumped condition registers,
    pub /: *mut *mut u8 severity; / from dbg_idle_chk_severity_types enum,
    pub reserved: u8,
}

// Idle Check result register header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_result_reg_hdr {
    pub data: u8,
pub const DBG_IDLE_CHK_RESULT_REG_HDR_IS_MEM_MASK: c_uint = 0x1;
pub const DBG_IDLE_CHK_RESULT_REG_HDR_IS_MEM_SHIFT: c_int = 0;
pub const DBG_IDLE_CHK_RESULT_REG_HDR_REG_ID_MASK: c_uint = 0x7F;
pub const DBG_IDLE_CHK_RESULT_REG_HDR_REG_ID_SHIFT: c_int = 1;
    pub /: *mut *mut u8 start_entry; / index of the first checked entry,
    pub /: *mut *mut u16 size; / register size in dwords,
}

// Idle Check rule
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_rule {
    pub /: *mut *mut u16 rule_id; / Idle Check rule ID,
    pub /: *mut *mut u8 severity; / value from dbg_idle_chk_severity_types enum,
    pub /: *mut *mut u8 cond_id; / Condition ID,
    pub /: *mut *mut u8 num_cond_regs; / number of condition registers,
    pub /: *mut *mut u8 num_info_regs; / number of info registers,
    pub /: *mut *mut u8 num_imms; / number of immediates in the condition,
    pub reserved1: u8,
    pub check: *mut *mut u16 reg_offset; / offset of this rules registers in the idle,
// register array (in dbg_idle_chk_reg units).
//
    pub the: *mut *mut u16 imm_offset; / offset of this rules immediate values in,
// immediate values array (in dwords).
//
}

// Idle Check rule parsing data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_idle_chk_rule_parsing_data {
    pub data: u32,
pub const DBG_IDLE_CHK_RULE_PARSING_DATA_HAS_FW_MSG_MASK: c_uint = 0x1;
pub const DBG_IDLE_CHK_RULE_PARSING_DATA_HAS_FW_MSG_SHIFT: c_int = 0;
pub const DBG_IDLE_CHK_RULE_PARSING_DATA_STR_OFFSET_MASK: c_uint = 0x7FFFFFFF;
pub const DBG_IDLE_CHK_RULE_PARSING_DATA_STR_OFFSET_SHIFT: c_int = 1;
}

// Idle check severity types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_idle_chk_severity_types {
// idle check failure should cause an error
    IDLE_CHK_SEVERITY_ERROR,
// idle check failure should cause an error only if theres no traffic
    IDLE_CHK_SEVERITY_ERROR_NO_TRAFFIC,
// idle check failure should cause a warning
    IDLE_CHK_SEVERITY_WARNING,
    MAX_DBG_IDLE_CHK_SEVERITY_TYPES
}

// Reset register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_reset_reg {
    pub data: u32,
pub const DBG_RESET_REG_ADDR_MASK: c_uint = 0xFFFFFF;
pub const DBG_RESET_REG_ADDR_SHIFT: c_int = 0;
pub const DBG_RESET_REG_IS_REMOVED_MASK: c_uint = 0x1;
pub const DBG_RESET_REG_IS_REMOVED_SHIFT: c_int = 24;
pub const DBG_RESET_REG_RESERVED_MASK: c_uint = 0x7F;
pub const DBG_RESET_REG_RESERVED_SHIFT: c_int = 25;
}

// Debug Bus block data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_block_data {
    pub enable_mask: u8,
    pub right_shift: u8,
    pub force_valid_mask: u8,
    pub force_frame_mask: u8,
    pub dword_mask: u8,
    pub line_num: u8,
    pub hw_id: u8,
    pub flags: u8,
pub const DBG_BUS_BLOCK_DATA_IS_256B_LINE_MASK: c_uint = 0x1;
pub const DBG_BUS_BLOCK_DATA_IS_256B_LINE_SHIFT: c_int = 0;
pub const DBG_BUS_BLOCK_DATA_RESERVED_MASK: c_uint = 0x7F;
pub const DBG_BUS_BLOCK_DATA_RESERVED_SHIFT: c_int = 1;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_bus_clients {
    DBG_BUS_CLIENT_RBCN,
    DBG_BUS_CLIENT_RBCP,
    DBG_BUS_CLIENT_RBCR,
    DBG_BUS_CLIENT_RBCT,
    DBG_BUS_CLIENT_RBCU,
    DBG_BUS_CLIENT_RBCF,
    DBG_BUS_CLIENT_RBCX,
    DBG_BUS_CLIENT_RBCS,
    DBG_BUS_CLIENT_RBCH,
    DBG_BUS_CLIENT_RBCZ,
    DBG_BUS_CLIENT_OTHER_ENGINE,
    DBG_BUS_CLIENT_TIMESTAMP,
    DBG_BUS_CLIENT_CPU,
    DBG_BUS_CLIENT_RBCY,
    DBG_BUS_CLIENT_RBCQ,
    DBG_BUS_CLIENT_RBCM,
    DBG_BUS_CLIENT_RBCB,
    DBG_BUS_CLIENT_RBCW,
    DBG_BUS_CLIENT_RBCV,
    MAX_DBG_BUS_CLIENTS
}

// Debug Bus constraint operation types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_bus_constraint_ops {
    DBG_BUS_CONSTRAINT_OP_EQ,
    DBG_BUS_CONSTRAINT_OP_NE,
    DBG_BUS_CONSTRAINT_OP_LT,
    DBG_BUS_CONSTRAINT_OP_LTC,
    DBG_BUS_CONSTRAINT_OP_LE,
    DBG_BUS_CONSTRAINT_OP_LEC,
    DBG_BUS_CONSTRAINT_OP_GT,
    DBG_BUS_CONSTRAINT_OP_GTC,
    DBG_BUS_CONSTRAINT_OP_GE,
    DBG_BUS_CONSTRAINT_OP_GEC,
    MAX_DBG_BUS_CONSTRAINT_OPS
}

// Debug Bus trigger state data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_trigger_state_data {
    pub msg_len: u8,
    pub constraint_dword_mask: u8,
    pub storm_id: u8,
    pub reserved: u8,
}

// Debug Bus memory address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_mem_addr {
    pub lo: u32,
    pub hi: u32,
}

// Debug Bus PCI buffer data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_pci_buf_data {
    pub /: *mut *mut dbg_bus_mem_addr phys_addr; / PCI buffer physical address,
    pub /: *mut *mut dbg_bus_mem_addr virt_addr; / PCI buffer virtual address,
    pub /: *mut *mut u32 size; / PCI buffer size in bytes,
}

// Debug Bus Storm EID range filter params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_storm_eid_range_params {
    pub /: *mut *mut u8 min; / Minimal event ID to filter on,
    pub /: *mut *mut u8 max; / Maximal event ID to filter on,
}

// Debug Bus Storm EID mask filter params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_storm_eid_mask_params {
    pub /: *mut *mut u8 val; / Event ID value,
    pub /: *mut *mut u8 mask; / Event ID mask. 1s in the mask = dont care bits.,
}

// Debug Bus Storm EID filter params
#[repr(C)]
#[derive(Copy, Clone)]
pub union dbg_bus_storm_eid_params {
    pub range: dbg_bus_storm_eid_range_params,
    pub mask: dbg_bus_storm_eid_mask_params,
}

// Debug Bus Storm data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_storm_data {
    pub enabled: u8,
    pub mode: u8,
    pub hw_id: u8,
    pub eid_filter_en: u8,
    pub eid_range_not_mask: u8,
    pub cid_filter_en: u8,
    pub eid_filter_params: dbg_bus_storm_eid_params,
    pub cid: u32,
}

// Debug Bus data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_bus_data {
    pub app_version: u32,
    pub state: u8,
    pub mode_256b_en: u8,
    pub num_enabled_blocks: u8,
    pub num_enabled_storms: u8,
    pub target: u8,
    pub one_shot_en: u8,
    pub grc_input_en: u8,
    pub timestamp_input_en: u8,
    pub filter_en: u8,
    pub adding_filter: u8,
    pub filter_pre_trigger: u8,
    pub filter_post_trigger: u8,
    pub trigger_en: u8,
    pub filter_constraint_dword_mask: u8,
    pub next_trigger_state: u8,
    pub next_constraint_id: u8,
    pub trigger_states: [dbg_bus_trigger_state_data; 3],
    pub filter_msg_len: u8,
    pub rcv_from_other_engine: u8,
    pub blocks_dword_mask: u8,
    pub blocks_dword_overlap: u8,
    pub hw_id_mask: u32,
    pub pci_buf: dbg_bus_pci_buf_data,
    pub blocks: [dbg_bus_block_data; 132],
    pub storms: [dbg_bus_storm_data; 6],
}

// Debug bus states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_bus_states {
    DBG_BUS_STATE_IDLE,
    DBG_BUS_STATE_READY,
    DBG_BUS_STATE_RECORDING,
    DBG_BUS_STATE_STOPPED,
    MAX_DBG_BUS_STATES
}

// Debug Bus Storm modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_bus_storm_modes {
    DBG_BUS_STORM_MODE_PRINTF,
    DBG_BUS_STORM_MODE_PRAM_ADDR,
    DBG_BUS_STORM_MODE_DRA_RW,
    DBG_BUS_STORM_MODE_DRA_W,
    DBG_BUS_STORM_MODE_LD_ST_ADDR,
    DBG_BUS_STORM_MODE_DRA_FSM,
    DBG_BUS_STORM_MODE_FAST_DBGMUX,
    DBG_BUS_STORM_MODE_RH,
    DBG_BUS_STORM_MODE_RH_WITH_STORE,
    DBG_BUS_STORM_MODE_FOC,
    DBG_BUS_STORM_MODE_EXT_STORE,
    MAX_DBG_BUS_STORM_MODES
}

// Debug bus target IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_bus_targets {
    DBG_BUS_TARGET_ID_INT_BUF,
    DBG_BUS_TARGET_ID_NIG,
    DBG_BUS_TARGET_ID_PCI,
    MAX_DBG_BUS_TARGETS
}

// GRC Dump data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_grc_data {
    pub params_initialized: u8,
    pub reserved1: u8,
    pub reserved2: u16,
    pub param_val: [u32; 48],
}

// Debug GRC params
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_grc_params {
    DBG_GRC_PARAM_DUMP_TSTORM,
    DBG_GRC_PARAM_DUMP_MSTORM,
    DBG_GRC_PARAM_DUMP_USTORM,
    DBG_GRC_PARAM_DUMP_XSTORM,
    DBG_GRC_PARAM_DUMP_YSTORM,
    DBG_GRC_PARAM_DUMP_PSTORM,
    DBG_GRC_PARAM_DUMP_REGS,
    DBG_GRC_PARAM_DUMP_RAM,
    DBG_GRC_PARAM_DUMP_PBUF,
    DBG_GRC_PARAM_DUMP_IOR,
    DBG_GRC_PARAM_DUMP_VFC,
    DBG_GRC_PARAM_DUMP_CM_CTX,
    DBG_GRC_PARAM_DUMP_PXP,
    DBG_GRC_PARAM_DUMP_RSS,
    DBG_GRC_PARAM_DUMP_CAU,
    DBG_GRC_PARAM_DUMP_QM,
    DBG_GRC_PARAM_DUMP_MCP,
    DBG_GRC_PARAM_DUMP_DORQ,
    DBG_GRC_PARAM_DUMP_CFC,
    DBG_GRC_PARAM_DUMP_IGU,
    DBG_GRC_PARAM_DUMP_BRB,
    DBG_GRC_PARAM_DUMP_BTB,
    DBG_GRC_PARAM_DUMP_BMB,
    DBG_GRC_PARAM_RESERVD1,
    DBG_GRC_PARAM_DUMP_MULD,
    DBG_GRC_PARAM_DUMP_PRS,
    DBG_GRC_PARAM_DUMP_DMAE,
    DBG_GRC_PARAM_DUMP_TM,
    DBG_GRC_PARAM_DUMP_SDM,
    DBG_GRC_PARAM_DUMP_DIF,
    DBG_GRC_PARAM_DUMP_STATIC,
    DBG_GRC_PARAM_UNSTALL,
    DBG_GRC_PARAM_RESERVED2,
    DBG_GRC_PARAM_MCP_TRACE_META_SIZE,
    DBG_GRC_PARAM_EXCLUDE_ALL,
    DBG_GRC_PARAM_CRASH,
    DBG_GRC_PARAM_PARITY_SAFE,
    DBG_GRC_PARAM_DUMP_CM,
    DBG_GRC_PARAM_DUMP_PHY,
    DBG_GRC_PARAM_NO_MCP,
    DBG_GRC_PARAM_NO_FW_VER,
    DBG_GRC_PARAM_RESERVED3,
    DBG_GRC_PARAM_DUMP_MCP_HW_DUMP,
    DBG_GRC_PARAM_DUMP_ILT_CDUC,
    DBG_GRC_PARAM_DUMP_ILT_CDUT,
    DBG_GRC_PARAM_DUMP_CAU_EXT,
    MAX_DBG_GRC_PARAMS
}

// Debug status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_status {
    DBG_STATUS_OK,
    DBG_STATUS_APP_VERSION_NOT_SET,
    DBG_STATUS_UNSUPPORTED_APP_VERSION,
    DBG_STATUS_DBG_BLOCK_NOT_RESET,
    DBG_STATUS_INVALID_ARGS,
    DBG_STATUS_OUTPUT_ALREADY_SET,
    DBG_STATUS_INVALID_PCI_BUF_SIZE,
    DBG_STATUS_PCI_BUF_ALLOC_FAILED,
    DBG_STATUS_PCI_BUF_NOT_ALLOCATED,
    DBG_STATUS_INVALID_FILTER_TRIGGER_DWORDS,
    DBG_STATUS_NO_MATCHING_FRAMING_MODE,
    DBG_STATUS_VFC_READ_ERROR,
    DBG_STATUS_STORM_ALREADY_ENABLED,
    DBG_STATUS_STORM_NOT_ENABLED,
    DBG_STATUS_BLOCK_ALREADY_ENABLED,
    DBG_STATUS_BLOCK_NOT_ENABLED,
    DBG_STATUS_NO_INPUT_ENABLED,
    DBG_STATUS_NO_FILTER_TRIGGER_256B,
    DBG_STATUS_FILTER_ALREADY_ENABLED,
    DBG_STATUS_TRIGGER_ALREADY_ENABLED,
    DBG_STATUS_TRIGGER_NOT_ENABLED,
    DBG_STATUS_CANT_ADD_CONSTRAINT,
    DBG_STATUS_TOO_MANY_TRIGGER_STATES,
    DBG_STATUS_TOO_MANY_CONSTRAINTS,
    DBG_STATUS_RECORDING_NOT_STARTED,
    DBG_STATUS_DATA_DIDNT_TRIGGER,
    DBG_STATUS_NO_DATA_RECORDED,
    DBG_STATUS_DUMP_BUF_TOO_SMALL,
    DBG_STATUS_DUMP_NOT_CHUNK_ALIGNED,
    DBG_STATUS_UNKNOWN_CHIP,
    DBG_STATUS_VIRT_MEM_ALLOC_FAILED,
    DBG_STATUS_BLOCK_IN_RESET,
    DBG_STATUS_INVALID_TRACE_SIGNATURE,
    DBG_STATUS_INVALID_NVRAM_BUNDLE,
    DBG_STATUS_NVRAM_GET_IMAGE_FAILED,
    DBG_STATUS_NON_ALIGNED_NVRAM_IMAGE,
    DBG_STATUS_NVRAM_READ_FAILED,
    DBG_STATUS_IDLE_CHK_PARSE_FAILED,
    DBG_STATUS_MCP_TRACE_BAD_DATA,
    DBG_STATUS_MCP_TRACE_NO_META,
    DBG_STATUS_MCP_COULD_NOT_HALT,
    DBG_STATUS_MCP_COULD_NOT_RESUME,
    DBG_STATUS_RESERVED0,
    DBG_STATUS_SEMI_FIFO_NOT_EMPTY,
    DBG_STATUS_IGU_FIFO_BAD_DATA,
    DBG_STATUS_MCP_COULD_NOT_MASK_PRTY,
    DBG_STATUS_FW_ASSERTS_PARSE_FAILED,
    DBG_STATUS_REG_FIFO_BAD_DATA,
    DBG_STATUS_PROTECTION_OVERRIDE_BAD_DATA,
    DBG_STATUS_DBG_ARRAY_NOT_SET,
    DBG_STATUS_RESERVED1,
    DBG_STATUS_NON_MATCHING_LINES,
    DBG_STATUS_INSUFFICIENT_HW_IDS,
    DBG_STATUS_DBG_BUS_IN_USE,
    DBG_STATUS_INVALID_STORM_DBG_MODE,
    DBG_STATUS_OTHER_ENGINE_BB_ONLY,
    DBG_STATUS_FILTER_SINGLE_HW_ID,
    DBG_STATUS_TRIGGER_SINGLE_HW_ID,
    DBG_STATUS_MISSING_TRIGGER_STATE_STORM,
    MAX_DBG_STATUS
}

// Debug Storms IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_storms {
    DBG_TSTORM_ID,
    DBG_MSTORM_ID,
    DBG_USTORM_ID,
    DBG_XSTORM_ID,
    DBG_YSTORM_ID,
    DBG_PSTORM_ID,
    MAX_DBG_STORMS
}

// Idle Check data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idle_chk_data {
    pub buf_size: u32,
    pub buf_size_set: u8,
    pub reserved1: u8,
    pub reserved2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pretend_params {
    pub split_type: u8,
    pub reserved: u8,
    pub split_id: u16,
}

// Debug Tools data (per HW function)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_tools_data {
    pub grc: dbg_grc_data,
    pub bus: dbg_bus_data,
    pub idle_chk: idle_chk_data,
    pub mode_enable: [u8; 40],
    pub block_in_reset: [u8; 132],
    pub chip_id: u8,
    pub hw_type: u8,
    pub num_ports: u8,
    pub num_pfs_per_port: u8,
    pub num_vfs: u8,
    pub initialized: u8,
    pub use_dmae: u8,
    pub reserved: u8,
    pub pretend: pretend_params,
    pub num_regs_read: u32,
}

// ILT Clients
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ilt_clients {
    ILT_CLI_CDUC,
    ILT_CLI_CDUT,
    ILT_CLI_QM,
    ILT_CLI_TM,
    ILT_CLI_SRC,
    ILT_CLI_TSDM,
    ILT_CLI_RGFS,
    ILT_CLI_TGFS,
    MAX_ILT_CLIENTS
}

// Public Functions
//
// qed_dbg_set_bin_ptr(): Sets a pointer to the binary data with debug
// arrays.
//
// @p_hwfn: HW device data.
// @bin_ptr: A pointer to the binary data with debug arrays.
//
// Return: enum dbg status.
//
// qed_read_regs(): Reads registers into a buffer (using GRC).
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf: Destination buffer.
// @addr: Source GRC address in dwords.
// @len: Number of registers to read.
//
// Return: Void.
//
// qed_read_fw_info(): Reads FW info from the chip.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @fw_info: (Out) a pointer to write the FW info into.
//
// Return: True if the FW info was read successfully from one of the Storms,
// or false if all Storms are in reset.
//
// The FW info contains FW-related information, such as the FW version,
// FW image (main/L2B/kuku), FW timestamp, etc.
// The FW info is read from the internal RAM of the first Storm that is not in
// reset.
//
// qed_dbg_grc_config(): Sets the value of a GRC parameter.
//
// @p_hwfn: HW device data.
// @grc_param: GRC parameter.
// @val: Value to set.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - Grc_param is invalid.
// - Val is outside the allowed boundaries.
//
// qed_dbg_grc_set_params_default(): Reverts all GRC parameters to their
// default value.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_dbg_grc_set_params_default(p_hwfn: *mut qed_hwfn);
}
//
// qed_dbg_grc_get_dump_buf_size(): Returns the required buffer size for
// GRC Dump.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) required buffer size (in dwords) for the GRC Dump
// data.
//
// Return: Error if one of the following holds:
// - The version wasn't set
// Otherwise, returns ok.
//
// qed_dbg_grc_dump(): Dumps GRC data into the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the collected GRC data into.
// @buf_size_in_dwords:Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - The specified dump buffer is too small.
// Otherwise, returns ok.
//
// qed_dbg_idle_chk_get_dump_buf_size(): Returns the required buffer size
// for idle check results.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) required buffer size (in dwords) for the idle check
// data.
//
// return: Error if one of the following holds:
// - The version wasn't set.
// Otherwise, returns ok.
//
// qed_dbg_idle_chk_dump: Performs idle check and writes the results
// into the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the idle check data into.
// @buf_size_in_dwords: Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - The specified buffer is too small.
// Otherwise, returns ok.
//
// qed_dbg_mcp_trace_get_dump_buf_size(): Returns the required buffer size
// for mcp trace results.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) Required buffer size (in dwords) for mcp trace data.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - The trace data in MCP scratchpad contain an invalid signature.
// - The bundle ID in NVRAM is invalid.
// - The trace meta data cannot be found (in NVRAM or image file).
// Otherwise, returns ok.
//
// qed_dbg_mcp_trace_dump(): Performs mcp trace and writes the results
// into the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the mcp trace data into.
// @buf_size_in_dwords: Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - The specified buffer is too small.
// - The trace data in MCP scratchpad contain an invalid signature.
// - The bundle ID in NVRAM is invalid.
// - The trace meta data cannot be found (in NVRAM or image file).
// - The trace meta data cannot be read (from NVRAM or image file).
// Otherwise, returns ok.
//
// qed_dbg_reg_fifo_get_dump_buf_size(): Returns the required buffer size
// for grc trace fifo results.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) Required buffer size (in dwords) for reg fifo data.
//
// Return: Error if one of the following holds:
// - The version wasn't set
// Otherwise, returns ok.
//
// qed_dbg_reg_fifo_dump(): Reads the reg fifo and writes the results into
// the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the reg fifo data into.
// @buf_size_in_dwords: Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - The specified buffer is too small.
// - DMAE transaction failed.
// Otherwise, returns ok.
//
// qed_dbg_igu_fifo_get_dump_buf_size(): Returns the required buffer size
// for the IGU fifo results.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) Required buffer size (in dwords) for the IGU fifo
// data.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// Otherwise, returns ok.
//
// qed_dbg_igu_fifo_dump(): Reads the IGU fifo and writes the results into
// the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the IGU fifo data into.
// @buf_size_in_dwords: Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// Return: Error if one of the following holds:
// - The version wasn't set
// - The specified buffer is too small
// - DMAE transaction failed
// Otherwise, returns ok.
//
// qed_dbg_protection_override_get_dump_buf_size(): Returns the required
// buffer size for protection override window results.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) Required buffer size (in dwords) for protection
// override data.
//
// Return: Error if one of the following holds:
// - The version wasn't set
// Otherwise, returns ok.
//
// qed_dbg_protection_override_dump(): Reads protection override window
// entries and writes the results into the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the protection override data into.
// @buf_size_in_dwords: Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// @return: Error if one of the following holds:
// - The version wasn't set.
// - The specified buffer is too small.
// - DMAE transaction failed.
// Otherwise, returns ok.
//
// qed_dbg_fw_asserts_get_dump_buf_size(): Returns the required buffer
// size for FW Asserts results.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @buf_size: (OUT) Required buffer size (in dwords) for FW Asserts data.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// Otherwise, returns ok.
//
// qed_dbg_fw_asserts_dump(): Reads the FW Asserts and writes the results
// into the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dump_buf: Pointer to write the FW Asserts data into.
// @buf_size_in_dwords: Size of the specified buffer in dwords.
// @num_dumped_dwords: (OUT) number of dumped dwords.
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// - The specified buffer is too small.
// Otherwise, returns ok.
//
// qed_dbg_read_attn(): Reads the attention registers of the specified
// block and type, and writes the results into the specified buffer.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @block: Block ID.
// @attn_type: Attention type.
// @clear_status: Indicates if the attention status should be cleared.
// @results:  (OUT) Pointer to write the read results into.
//
// Return: Error if one of the following holds:
// - The version wasn't set
// Otherwise, returns ok.
//
// qed_dbg_print_attn(): Prints attention registers values in the
// specified results struct.
//
// @p_hwfn: HW device data.
// @results: Pointer to the attention read results
//
// Return: Error if one of the following holds:
// - The version wasn't set
// Otherwise, returns ok.
//
// Data Types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_trace_format {
    pub data: u32,
pub const MCP_TRACE_FORMAT_MODULE_MASK: c_uint = 0x0000ffff;
pub const MCP_TRACE_FORMAT_MODULE_OFFSET: c_int = 0;
pub const MCP_TRACE_FORMAT_LEVEL_MASK: c_uint = 0x00030000;
pub const MCP_TRACE_FORMAT_LEVEL_OFFSET: c_int = 16;
pub const MCP_TRACE_FORMAT_P1_SIZE_MASK: c_uint = 0x000c0000;
pub const MCP_TRACE_FORMAT_P1_SIZE_OFFSET: c_int = 18;
pub const MCP_TRACE_FORMAT_P2_SIZE_MASK: c_uint = 0x00300000;
pub const MCP_TRACE_FORMAT_P2_SIZE_OFFSET: c_int = 20;
pub const MCP_TRACE_FORMAT_P3_SIZE_MASK: c_uint = 0x00c00000;
pub const MCP_TRACE_FORMAT_P3_SIZE_OFFSET: c_int = 22;
pub const MCP_TRACE_FORMAT_LEN_MASK: c_uint = 0xff000000;
pub const MCP_TRACE_FORMAT_LEN_OFFSET: c_int = 24;
    pub format_str: *mut c_char,
}

// MCP Trace Meta data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_trace_meta {
    pub modules_num: u32,
    pub modules: *mut c_char,
    pub formats_num: u32,
    pub formats: *mut mcp_trace_format,
    pub is_allocated: bool,
}

// Debug Tools user data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_tools_user_data {
    pub mcp_trace_meta: mcp_trace_meta,
    pub mcp_trace_user_meta_buf: *const u32,
}

// Constants
pub const MAX_NAME_LEN: c_int = 16;
// Public Functions
//
// qed_dbg_user_set_bin_ptr(): Sets a pointer to the binary data with
// debug arrays.
//
// @p_hwfn: HW device data.
// @bin_ptr: a pointer to the binary data with debug arrays.
//
// Return: dbg_status.
//
// qed_dbg_alloc_user_data(): Allocates user debug data.
//
// @p_hwfn: HW device data.
// @user_data_ptr: (OUT) a pointer to the allocated memory.
//
// Return: dbg_status.
//
// qed_dbg_get_status_str(): Returns a string for the specified status.
//
// @status: A debug status code.
//
// Return: A string for the specified status.
//
// qed_get_idle_chk_results_buf_size(): Returns the required buffer size
// for idle check results (in bytes).
//
// @p_hwfn: HW device data.
// @dump_buf: idle check dump buffer.
// @num_dumped_dwords: number of dwords that were dumped.
// @results_buf_size: (OUT) required buffer size (in bytes) for the parsed
// results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_print_idle_chk_results(): Prints idle check results
//
// @p_hwfn: HW device data.
// @dump_buf: idle check dump buffer.
// @num_dumped_dwords: number of dwords that were dumped.
// @results_buf: buffer for printing the idle check results.
// @num_errors: (OUT) number of errors found in idle check.
// @num_warnings: (OUT) number of warnings found in idle check.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_dbg_mcp_trace_set_meta_data(): Sets the MCP Trace meta data.
//
// @p_hwfn: HW device data.
// @meta_buf: Meta buffer.
//
// Return: Void.
//
// Needed in case the MCP Trace dump doesn't contain the meta data (e.g. due to
// no NVRAM access).
//
// qed_get_mcp_trace_results_buf_size(): Returns the required buffer size
// for MCP Trace results (in bytes).
//
// @p_hwfn: HW device data.
// @dump_buf: MCP Trace dump buffer.
// @num_dumped_dwords: number of dwords that were dumped.
// @results_buf_size: (OUT) required buffer size (in bytes) for the parsed
// results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_print_mcp_trace_results(): Prints MCP Trace results
//
// @p_hwfn: HW device data.
// @dump_buf: MCP trace dump buffer, starting from the header.
// @num_dumped_dwords: Member of dwords that were dumped.
// @results_buf: Buffer for printing the mcp trace results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_mcp_trace_free_meta_data(): Frees the MCP Trace meta data.
// Should be called after continuous MCP Trace parsing.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_mcp_trace_free_meta_data(p_hwfn: *mut qed_hwfn);
}
//
// qed_get_reg_fifo_results_buf_size(): Returns the required buffer size
// for reg_fifo results (in bytes).
//
// @p_hwfn: HW device data.
// @dump_buf: Reg fifo dump buffer.
// @num_dumped_dwords: Number of dwords that were dumped.
// @results_buf_size: (OUT) required buffer size (in bytes) for the parsed
// results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_print_reg_fifo_results(): Prints reg fifo results.
//
// @p_hwfn: HW device data.
// @dump_buf: Reg fifo dump buffer, starting from the header.
// @num_dumped_dwords: Number of dwords that were dumped.
// @results_buf: Buffer for printing the reg fifo results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_get_igu_fifo_results_buf_size(): Returns the required buffer size
// for igu_fifo results (in bytes).
//
// @p_hwfn: HW device data.
// @dump_buf: IGU fifo dump buffer.
// @num_dumped_dwords: number of dwords that were dumped.
// @results_buf_size: (OUT) required buffer size (in bytes) for the parsed
// results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_print_igu_fifo_results(): Prints IGU fifo results
//
// @p_hwfn: HW device data.
// @dump_buf: IGU fifo dump buffer, starting from the header.
// @num_dumped_dwords: Number of dwords that were dumped.
// @results_buf: Buffer for printing the IGU fifo results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_get_protection_override_results_buf_size(): Returns the required
// buffer size for protection override results (in bytes).
//
// @p_hwfn: HW device data.
// @dump_buf: Protection override dump buffer.
// @num_dumped_dwords: Number of dwords that were dumped.
// @results_buf_size: (OUT) required buffer size (in bytes) for the parsed
// results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_print_protection_override_results(): Prints protection override
// results.
//
// @p_hwfn: HW device data.
// @dump_buf: Protection override dump buffer, starting from the header.
// @num_dumped_dwords: Number of dwords that were dumped.
// @results_buf: Buffer for printing the reg fifo results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_get_fw_asserts_results_buf_size(): Returns the required buffer size
// for FW Asserts results (in bytes).
//
// @p_hwfn: HW device data.
// @dump_buf: FW Asserts dump buffer.
// @num_dumped_dwords: number of dwords that were dumped.
// @results_buf_size: (OUT) required buffer size (in bytes) for the parsed
// results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_print_fw_asserts_results(): Prints FW Asserts results.
//
// @p_hwfn: HW device data.
// @dump_buf: FW Asserts dump buffer, starting from the header.
// @num_dumped_dwords: number of dwords that were dumped.
// @results_buf: buffer for printing the FW Asserts results.
//
// Return: Error if the parsing fails, ok otherwise.
//
// qed_dbg_parse_attn(): Parses and prints attention registers values in
// the specified results struct.
//
// @p_hwfn: HW device data.
// @results: Pointer to the attention read results
//
// Return: Error if one of the following holds:
// - The version wasn't set.
// Otherwise, returns ok.
//
