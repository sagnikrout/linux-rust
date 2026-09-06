//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/common_hsi.h
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
// Copyright (c) 2015-2016  QLogic Corporation
// Copyright (c) 2019-2021 Marvell International Ltd.
//

// dma_addr_t manip

//
// PROTOCOL COMMON FW CONSTANTS
//
pub const X_FINAL_CLEANUP_AGG_INT: c_int = 1;
pub const EVENT_RING_PAGE_SIZE_BYTES: c_int = 4096;
pub const NUM_OF_GLOBAL_QUEUES: c_int = 128;
pub const COMMON_QUEUE_ENTRY_MAX_BYTE_SIZE: c_int = 64;
pub const ISCSI_CDU_TASK_SEG_TYPE: c_int = 0;
pub const FCOE_CDU_TASK_SEG_TYPE: c_int = 0;
pub const RDMA_CDU_TASK_SEG_TYPE: c_int = 1;
pub const ETH_CDU_TASK_SEG_TYPE: c_int = 2;
pub const FW_ASSERT_GENERAL_ATTN_IDX: c_int = 32;
// Queue Zone sizes in bytes
pub const TSTORM_QZONE_SIZE: c_int = 8;
pub const MSTORM_QZONE_SIZE: c_int = 16;
pub const USTORM_QZONE_SIZE: c_int = 8;
pub const XSTORM_QZONE_SIZE: c_int = 8;
pub const YSTORM_QZONE_SIZE: c_int = 0;
pub const PSTORM_QZONE_SIZE: c_int = 0;
pub const MSTORM_VF_ZONE_DEFAULT_SIZE_LOG: c_int = 7;
pub const ETH_MAX_RXQ_VF_DEFAULT: c_int = 16;
pub const ETH_MAX_RXQ_VF_DOUBLE: c_int = 48;
pub const ETH_MAX_RXQ_VF_QUAD: c_int = 112;
pub const ETH_RGSRC_CTX_SIZE: c_int = 6;
pub const ETH_TGSRC_CTX_SIZE: c_int = 6;
//
// CORE (LIGHT L2) FW CONSTANTS
//
pub const CORE_LL2_MAX_RAMROD_PER_CON: c_int = 8;
pub const CORE_LL2_TX_BD_PAGE_SIZE_BYTES: c_int = 4096;
pub const CORE_LL2_RX_BD_PAGE_SIZE_BYTES: c_int = 4096;
pub const CORE_LL2_RX_CQE_PAGE_SIZE_BYTES: c_int = 4096;
pub const CORE_LL2_RX_NUM_NEXT_PAGE_BDS: c_int = 1;
pub const CORE_LL2_TX_MAX_BDS_PER_PACKET: c_int = 12;
pub const CORE_SPQE_PAGE_SIZE_BYTES: c_int = 4096;
// Number of LL2 RAM based queues
pub const MAX_NUM_LL2_RX_RAM_QUEUES: c_int = 32;
// Number of LL2 context based queues
pub const MAX_NUM_LL2_RX_CTX_QUEUES: c_int = 208;

pub const MAX_NUM_LL2_TX_STATS_COUNTERS: c_int = 48;
pub const FW_MAJOR_VERSION: c_int = 8;
pub const FW_MINOR_VERSION: c_int = 59;
pub const FW_REVISION_VERSION: c_int = 1;
pub const FW_ENGINEERING_VERSION: c_int = 0;
//
// COMMON HW CONSTANTS
//
// PCI functions

// Traffic classes in network-facing blocks (PBF, BTB, NIG, BRB, PRS and QM)

// CIDs

// Global PXP windows (GTT)
pub const NUM_OF_GTT: c_int = 19;
pub const GTT_DWORD_SIZE_BITS: c_int = 10;

// Tools Version
pub const TOOLS_VERSION: c_int = 11;
//
// CDU CONSTANTS
//

//
// DQ CONSTANTS
//
// DEMS
pub const DQ_DEMS_LEGACY: c_int = 0;
pub const DQ_DEMS_TOE_MORE_TO_SEND: c_int = 3;
pub const DQ_DEMS_TOE_LOCAL_ADV_WND: c_int = 4;
pub const DQ_DEMS_ROCE_CQ_CONS: c_int = 7;
// XCM agg val selection (HW)
pub const DQ_XCM_AGG_VAL_SEL_WORD2: c_int = 0;
pub const DQ_XCM_AGG_VAL_SEL_WORD3: c_int = 1;
pub const DQ_XCM_AGG_VAL_SEL_WORD4: c_int = 2;
pub const DQ_XCM_AGG_VAL_SEL_WORD5: c_int = 3;
pub const DQ_XCM_AGG_VAL_SEL_REG3: c_int = 4;
pub const DQ_XCM_AGG_VAL_SEL_REG4: c_int = 5;
pub const DQ_XCM_AGG_VAL_SEL_REG5: c_int = 6;
pub const DQ_XCM_AGG_VAL_SEL_REG6: c_int = 7;
// XCM agg val selection (FW)

// UCM agg val selection (HW)
pub const DQ_UCM_AGG_VAL_SEL_WORD0: c_int = 0;
pub const DQ_UCM_AGG_VAL_SEL_WORD1: c_int = 1;
pub const DQ_UCM_AGG_VAL_SEL_WORD2: c_int = 2;
pub const DQ_UCM_AGG_VAL_SEL_WORD3: c_int = 3;
pub const DQ_UCM_AGG_VAL_SEL_REG0: c_int = 4;
pub const DQ_UCM_AGG_VAL_SEL_REG1: c_int = 5;
pub const DQ_UCM_AGG_VAL_SEL_REG2: c_int = 6;
pub const DQ_UCM_AGG_VAL_SEL_REG3: c_int = 7;
// UCM agg val selection (FW)

// TCM agg val selection (HW)
pub const DQ_TCM_AGG_VAL_SEL_WORD0: c_int = 0;
pub const DQ_TCM_AGG_VAL_SEL_WORD1: c_int = 1;
pub const DQ_TCM_AGG_VAL_SEL_WORD2: c_int = 2;
pub const DQ_TCM_AGG_VAL_SEL_WORD3: c_int = 3;
pub const DQ_TCM_AGG_VAL_SEL_REG1: c_int = 4;
pub const DQ_TCM_AGG_VAL_SEL_REG2: c_int = 5;
pub const DQ_TCM_AGG_VAL_SEL_REG6: c_int = 6;
pub const DQ_TCM_AGG_VAL_SEL_REG9: c_int = 7;
// TCM agg val selection (FW)

// XCM agg counter flag selection (HW)
pub const DQ_XCM_AGG_FLG_SHIFT_BIT14: c_int = 0;
pub const DQ_XCM_AGG_FLG_SHIFT_BIT15: c_int = 1;
pub const DQ_XCM_AGG_FLG_SHIFT_CF12: c_int = 2;
pub const DQ_XCM_AGG_FLG_SHIFT_CF13: c_int = 3;
pub const DQ_XCM_AGG_FLG_SHIFT_CF18: c_int = 4;
pub const DQ_XCM_AGG_FLG_SHIFT_CF19: c_int = 5;
pub const DQ_XCM_AGG_FLG_SHIFT_CF22: c_int = 6;
pub const DQ_XCM_AGG_FLG_SHIFT_CF23: c_int = 7;
// XCM agg counter flag selection (FW)

// UCM agg counter flag selection (HW)
pub const DQ_UCM_AGG_FLG_SHIFT_CF0: c_int = 0;
pub const DQ_UCM_AGG_FLG_SHIFT_CF1: c_int = 1;
pub const DQ_UCM_AGG_FLG_SHIFT_CF3: c_int = 2;
pub const DQ_UCM_AGG_FLG_SHIFT_CF4: c_int = 3;
pub const DQ_UCM_AGG_FLG_SHIFT_CF5: c_int = 4;
pub const DQ_UCM_AGG_FLG_SHIFT_CF6: c_int = 5;
pub const DQ_UCM_AGG_FLG_SHIFT_RULE0EN: c_int = 6;
pub const DQ_UCM_AGG_FLG_SHIFT_RULE1EN: c_int = 7;
// UCM agg counter flag selection (FW)

// TCM agg counter flag selection (HW)
pub const DQ_TCM_AGG_FLG_SHIFT_CF0: c_int = 0;
pub const DQ_TCM_AGG_FLG_SHIFT_CF1: c_int = 1;
pub const DQ_TCM_AGG_FLG_SHIFT_CF2: c_int = 2;
pub const DQ_TCM_AGG_FLG_SHIFT_CF3: c_int = 3;
pub const DQ_TCM_AGG_FLG_SHIFT_CF4: c_int = 4;
pub const DQ_TCM_AGG_FLG_SHIFT_CF5: c_int = 5;
pub const DQ_TCM_AGG_FLG_SHIFT_CF6: c_int = 6;
pub const DQ_TCM_AGG_FLG_SHIFT_CF7: c_int = 7;
// TCM agg counter flag selection (FW)

// PWM address mapping
pub const DQ_PWM_OFFSET_DPM_BASE: c_uint = 0x0;
pub const DQ_PWM_OFFSET_DPM_END: c_uint = 0x27;
pub const DQ_PWM_OFFSET_XCM32_24ICID_BASE: c_uint = 0x28;
pub const DQ_PWM_OFFSET_UCM32_24ICID_BASE: c_uint = 0x30;
pub const DQ_PWM_OFFSET_TCM32_24ICID_BASE: c_uint = 0x38;
pub const DQ_PWM_OFFSET_XCM16_BASE: c_uint = 0x40;
pub const DQ_PWM_OFFSET_XCM32_BASE: c_uint = 0x44;
pub const DQ_PWM_OFFSET_UCM16_BASE: c_uint = 0x48;
pub const DQ_PWM_OFFSET_UCM32_BASE: c_uint = 0x4C;
pub const DQ_PWM_OFFSET_UCM16_4: c_uint = 0x50;
pub const DQ_PWM_OFFSET_TCM16_BASE: c_uint = 0x58;
pub const DQ_PWM_OFFSET_TCM32_BASE: c_uint = 0x5C;
pub const DQ_PWM_OFFSET_XCM_FLAGS: c_uint = 0x68;
pub const DQ_PWM_OFFSET_UCM_FLAGS: c_uint = 0x69;
pub const DQ_PWM_OFFSET_TCM_FLAGS: c_uint = 0x6B;

// DQ_DEMS_AGG_VAL_BASE

// DPM

// Conn type ranges

//
// QM CONSTANTS
//
// Number of TX queues in the QM
pub const MAX_QM_TX_QUEUES_K2: c_int = 512;
pub const MAX_QM_TX_QUEUES_BB: c_int = 448;

// Number of Other queues in the QM
pub const MAX_QM_OTHER_QUEUES_BB: c_int = 64;
pub const MAX_QM_OTHER_QUEUES_K2: c_int = 128;

// Number of queues in a PF queue group
pub const QM_PF_QUEUE_GROUP_SIZE: c_int = 8;
// The size of a single queue element in bytes
pub const QM_PQ_ELEMENT_SIZE: c_int = 4;
// Base number of Tx PQs in the CM PQ representation.
// Should be used when storing PQ IDs in CM PQ registers and context.
//
pub const CM_TX_PQ_BASE: c_uint = 0x200;
// Number of global Vport/QCN rate limiters
pub const MAX_QM_GLOBAL_RLS: c_int = 256;

// QM registers data
pub const QM_LINE_CRD_REG_WIDTH: c_int = 16;

pub const QM_BYTE_CRD_REG_WIDTH: c_int = 24;

pub const QM_WFQ_CRD_REG_WIDTH: c_int = 32;

pub const QM_RL_CRD_REG_WIDTH: c_int = 32;

//
// CAU CONSTANTS
//
pub const CAU_FSM_ETH_RX: c_int = 0;
pub const CAU_FSM_ETH_TX: c_int = 1;
// Number of Protocol Indices per Status Block
pub const PIS_PER_SB: c_int = 12;

pub const CAU_HC_STOPPED_STATE: c_int = 3;
pub const CAU_HC_DISABLE_STATE: c_int = 4;
pub const CAU_HC_ENABLE_STATE: c_int = 0;
//
// IGU CONSTANTS
//

pub const MAX_SB_PER_PF_MIMD: c_int = 129;
pub const MAX_SB_PER_PF_SIMD: c_int = 64;
pub const MAX_SB_PER_VF: c_int = 64;
// Memory addresses on the BAR for the IGU Sub Block
pub const IGU_MEM_BASE: c_uint = 0x0000;
pub const IGU_MEM_MSIX_BASE: c_uint = 0x0000;
pub const IGU_MEM_MSIX_UPPER: c_uint = 0x0101;
pub const IGU_MEM_MSIX_RESERVED_UPPER: c_uint = 0x01ff;
pub const IGU_MEM_PBA_MSIX_BASE: c_uint = 0x0200;
pub const IGU_MEM_PBA_MSIX_UPPER: c_uint = 0x0202;
pub const IGU_MEM_PBA_MSIX_RESERVED_UPPER: c_uint = 0x03ff;
pub const IGU_CMD_INT_ACK_BASE: c_uint = 0x0400;
pub const IGU_CMD_INT_ACK_RESERVED_UPPER: c_uint = 0x05ff;
pub const IGU_CMD_ATTN_BIT_UPD_UPPER: c_uint = 0x05f0;
pub const IGU_CMD_ATTN_BIT_SET_UPPER: c_uint = 0x05f1;
pub const IGU_CMD_ATTN_BIT_CLR_UPPER: c_uint = 0x05f2;
pub const IGU_REG_SISR_MDPC_WMASK_UPPER: c_uint = 0x05f3;
pub const IGU_REG_SISR_MDPC_WMASK_LSB_UPPER: c_uint = 0x05f4;
pub const IGU_REG_SISR_MDPC_WMASK_MSB_UPPER: c_uint = 0x05f5;
pub const IGU_REG_SISR_MDPC_WOMASK_UPPER: c_uint = 0x05f6;
pub const IGU_CMD_PROD_UPD_BASE: c_uint = 0x0600;
pub const IGU_CMD_PROD_UPD_RESERVED_UPPER: c_uint = 0x07ff;
//
// PXP CONSTANTS
//
// Bars for Blocks
pub const PXP_BAR_GRC: c_int = 0;
pub const PXP_BAR_TSDM: c_int = 0;
pub const PXP_BAR_USDM: c_int = 0;
pub const PXP_BAR_XSDM: c_int = 0;
pub const PXP_BAR_MSDM: c_int = 0;
pub const PXP_BAR_YSDM: c_int = 0;
pub const PXP_BAR_PSDM: c_int = 0;
pub const PXP_BAR_IGU: c_int = 0;
pub const PXP_BAR_DQ: c_int = 1;
// PTT and GTT
pub const PXP_PER_PF_ENTRY_SIZE: c_int = 8;
pub const PXP_NUM_GLOBAL_WINDOWS: c_int = 243;
pub const PXP_GLOBAL_ENTRY_SIZE: c_int = 4;
pub const PXP_ADMIN_WINDOW_ALLOWED_LENGTH: c_int = 4;
pub const PXP_PF_WINDOW_ADMIN_START: c_int = 0;
pub const PXP_PF_WINDOW_ADMIN_LENGTH: c_uint = 0x1000;

pub const PXP_PF_WINDOW_ADMIN_PER_PF_START: c_int = 0;

pub const PXP_PF_WINDOW_ADMIN_GLOBAL_START: c_uint = 0x200;

pub const PXP_PF_GLOBAL_PRETEND_ADDR: c_uint = 0x1f0;
pub const PXP_PF_ME_OPAQUE_MASK_ADDR: c_uint = 0xf4;
pub const PXP_PF_ME_OPAQUE_ADDR: c_uint = 0x1f8;
pub const PXP_PF_ME_CONCRETE_ADDR: c_uint = 0x1fc;
pub const PXP_NUM_PF_WINDOWS: c_int = 12;
pub const PXP_EXTERNAL_BAR_PF_WINDOW_START: c_uint = 0x1000;

pub const PXP_EXTERNAL_BAR_PF_WINDOW_SINGLE_SIZE: c_uint = 0x1000;

pub const PXP_EXTERNAL_BAR_GLOBAL_WINDOW_SINGLE_SIZE: c_uint = 0x1000;

// PF BAR
pub const PXP_BAR0_START_GRC: c_uint = 0x0000;
pub const PXP_BAR0_GRC_LENGTH: c_uint = 0x1C00000;

pub const PXP_BAR0_START_IGU: c_uint = 0x1C00000;
pub const PXP_BAR0_IGU_LENGTH: c_uint = 0x10000;

pub const PXP_BAR0_START_TSDM: c_uint = 0x1C80000;
pub const PXP_BAR0_SDM_LENGTH: c_uint = 0x40000;
pub const PXP_BAR0_SDM_RESERVED_LENGTH: c_uint = 0x40000;

pub const PXP_BAR0_START_MSDM: c_uint = 0x1D00000;

pub const PXP_BAR0_START_USDM: c_uint = 0x1D80000;

pub const PXP_BAR0_START_XSDM: c_uint = 0x1E00000;

pub const PXP_BAR0_START_YSDM: c_uint = 0x1E80000;

pub const PXP_BAR0_START_PSDM: c_uint = 0x1F00000;

// VF BAR
pub const PXP_VF_BAR0: c_int = 0;
pub const PXP_VF_BAR0_START_IGU: c_int = 0;
pub const PXP_VF_BAR0_IGU_LENGTH: c_uint = 0x3000;

pub const PXP_VF_BAR0_START_DQ: c_uint = 0x3000;
pub const PXP_VF_BAR0_DQ_LENGTH: c_uint = 0x200;
pub const PXP_VF_BAR0_DQ_OPAQUE_OFFSET: c_int = 0;

pub const PXP_VF_BAR0_START_TSDM_ZONE_B: c_uint = 0x3200;
pub const PXP_VF_BAR0_SDM_LENGTH_ZONE_B: c_uint = 0x200;

pub const PXP_VF_BAR0_START_MSDM_ZONE_B: c_uint = 0x3400;

pub const PXP_VF_BAR0_START_USDM_ZONE_B: c_uint = 0x3600;

pub const PXP_VF_BAR0_START_XSDM_ZONE_B: c_uint = 0x3800;

pub const PXP_VF_BAR0_START_YSDM_ZONE_B: c_uint = 0x3a00;

pub const PXP_VF_BAR0_START_PSDM_ZONE_B: c_uint = 0x3c00;

pub const PXP_VF_BAR0_START_GRC: c_uint = 0x3E00;
pub const PXP_VF_BAR0_GRC_LENGTH: c_uint = 0x200;

pub const PXP_VF_BAR0_START_SDM_ZONE_A: c_uint = 0x4000;
pub const PXP_VF_BAR0_END_SDM_ZONE_A: c_uint = 0x10000;
pub const PXP_VF_BAR0_START_IGU2: c_uint = 0x10000;
pub const PXP_VF_BAR0_IGU2_LENGTH: c_uint = 0xD000;

pub const PXP_VF_BAR0_GRC_WINDOW_LENGTH: c_int = 32;
pub const PXP_ILT_PAGE_SIZE_NUM_BITS_MIN: c_int = 12;
pub const PXP_ILT_BLOCK_FACTOR_MULTIPLIER: c_int = 1024;
// ILT Records
pub const PXP_NUM_ILT_RECORDS_BB: c_int = 7600;
pub const PXP_NUM_ILT_RECORDS_K2: c_int = 11000;

// Host Interface
pub const PXP_QUEUES_ZONE_MAX_NUM: c_int = 320;
//
// PRM CONSTANTS
//
pub const PRM_DMA_PAD_BYTES_NUM: c_int = 2;
//
// SDMs CONSTANTS
//
pub const SDM_OP_GEN_TRIG_NONE: c_int = 0;
pub const SDM_OP_GEN_TRIG_WAKE_THREAD: c_int = 1;
pub const SDM_OP_GEN_TRIG_AGG_INT: c_int = 2;
pub const SDM_OP_GEN_TRIG_LOADER: c_int = 4;
pub const SDM_OP_GEN_TRIG_INDICATE_ERROR: c_int = 6;
pub const SDM_OP_GEN_TRIG_INC_ORDER_CNT: c_int = 9;
//
// Completion types
//
pub const SDM_COMP_TYPE_NONE: c_int = 0;
pub const SDM_COMP_TYPE_WAKE_THREAD: c_int = 1;
pub const SDM_COMP_TYPE_AGG_INT: c_int = 2;
pub const SDM_COMP_TYPE_CM: c_int = 3;
pub const SDM_COMP_TYPE_LOADER: c_int = 4;
pub const SDM_COMP_TYPE_PXP: c_int = 5;
pub const SDM_COMP_TYPE_INDICATE_ERROR: c_int = 6;
pub const SDM_COMP_TYPE_RELEASE_THREAD: c_int = 7;
pub const SDM_COMP_TYPE_RAM: c_int = 8;
pub const SDM_COMP_TYPE_INC_ORDER_CNT: c_int = 9;
//
// PBF CONSTANTS
//
// Number of PBF command queue lines. Each line is 32B.
pub const PBF_MAX_CMD_LINES: c_int = 3328;
// Number of BTB blocks. Each block is 256B.
pub const BTB_MAX_BLOCKS_BB: c_int = 1440;
pub const BTB_MAX_BLOCKS_K2: c_int = 1840;
//
// PRS CONSTANTS
//
pub const PRS_GFT_CAM_LINES_NO_MATCH: c_int = 31;
// Interrupt coalescing TimeSet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coalescing_timeset {
    pub value: u8,
pub const COALESCING_TIMESET_TIMESET_MASK: c_uint = 0x7F;
pub const COALESCING_TIMESET_TIMESET_SHIFT: c_int = 0;
pub const COALESCING_TIMESET_VALID_MASK: c_uint = 0x1;
pub const COALESCING_TIMESET_VALID_SHIFT: c_int = 7;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_queue_zone {
    pub ring_drv_data_consumer: __le16,
    pub reserved: __le16,
}

// ETH Rx producers data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_prod_data {
    pub bd_prod: __le16,
    pub cqe_prod: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ulp_connect_done_params {
    pub mss: __le16,
    pub snd_wnd_scale: u8,
    pub flags: u8,
pub const TCP_ULP_CONNECT_DONE_PARAMS_TS_EN_MASK: c_uint = 0x1;
pub const TCP_ULP_CONNECT_DONE_PARAMS_TS_EN_SHIFT: c_int = 0;
pub const TCP_ULP_CONNECT_DONE_PARAMS_RESERVED_MASK: c_uint = 0x7F;
pub const TCP_ULP_CONNECT_DONE_PARAMS_RESERVED_SHIFT: c_int = 1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_connect_done_results {
    pub icid: __le16,
    pub conn_id: __le16,
    pub params: tcp_ulp_connect_done_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_eqe_data {
    pub icid: __le16,
    pub conn_id: __le16,
    pub reserved: __le16,
    pub error_code: u8,
    pub error_pdu_opcode_reserved: u8,
pub const ISCSI_EQE_DATA_ERROR_PDU_OPCODE_MASK: c_uint = 0x3F;
pub const ISCSI_EQE_DATA_ERROR_PDU_OPCODE_SHIFT: c_int = 0;
pub const ISCSI_EQE_DATA_ERROR_PDU_OPCODE_VALID_MASK: c_uint = 0x1;
pub const ISCSI_EQE_DATA_ERROR_PDU_OPCODE_VALID_SHIFT: c_int = 6;
pub const ISCSI_EQE_DATA_RESERVED0_MASK: c_uint = 0x1;
pub const ISCSI_EQE_DATA_RESERVED0_SHIFT: c_int = 7;
}

// Multi function mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mf_mode {
    ERROR_MODE /* Unsupported mode */,
    MF_OVLAN,
    MF_NPAR,
    MAX_MF_MODE
}

// Per protocol packet duplication enable bit vector. If set, duplicate
// offloaded traffic to LL2 debug queueu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct offload_pkt_dup_enable {
    pub enable_vector: __le16,
}

// Per-protocol connection types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum protocol_type {
    PROTOCOLID_TCP_ULP,
    PROTOCOLID_FCOE,
    PROTOCOLID_ROCE,
    PROTOCOLID_CORE,
    PROTOCOLID_ETH,
    PROTOCOLID_IWARP,
    PROTOCOLID_RESERVED0,
    PROTOCOLID_PREROCE,
    PROTOCOLID_COMMON,
    PROTOCOLID_RESERVED1,
    PROTOCOLID_RDMA,
    PROTOCOLID_SCSI,
    MAX_PROTOCOL_TYPE
}

// Pstorm packet duplication config
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_pkt_dup_cfg {
    pub enable: offload_pkt_dup_enable,
    pub reserved: [__le16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regpair {
    pub lo: __le32,
    pub hi: __le32,
}

// RoCE Destroy Event Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_eqe_destroy_qp {
    pub cid: __le32,
    pub reserved: [u8; 4],
}

// RoCE Suspend Event Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_eqe_suspend_qp {
    pub cid: __le32,
    pub reserved: [u8; 4],
}

// RDMA Event Data Union
#[repr(C)]
#[derive(Copy, Clone)]
pub union rdma_eqe_data {
    pub async_handle: regpair,
    pub rdma_destroy_qp_data: rdma_eqe_destroy_qp,
    pub rdma_suspend_qp_data: rdma_eqe_suspend_qp,
}

// Tstorm packet duplication config
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_pkt_dup_cfg {
    pub enable: offload_pkt_dup_enable,
    pub reserved: __le16,
    pub cid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_queue_zone {
    pub reserved: [__le32; 2],
}

// Ustorm Queue Zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_eth_queue_zone {
    pub int_coalescing_timeset: coalescing_timeset,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_queue_zone {
    pub eth: ustorm_eth_queue_zone,
    pub common: common_queue_zone,
}

// Status block structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cau_pi_entry {
    pub prod: __le32,
pub const CAU_PI_ENTRY_PROD_VAL_MASK: c_uint = 0xFFFF;
pub const CAU_PI_ENTRY_PROD_VAL_SHIFT: c_int = 0;
pub const CAU_PI_ENTRY_PI_TIMESET_MASK: c_uint = 0x7F;
pub const CAU_PI_ENTRY_PI_TIMESET_SHIFT: c_int = 16;
pub const CAU_PI_ENTRY_FSM_SEL_MASK: c_uint = 0x1;
pub const CAU_PI_ENTRY_FSM_SEL_SHIFT: c_int = 23;
pub const CAU_PI_ENTRY_RESERVED_MASK: c_uint = 0xFF;
pub const CAU_PI_ENTRY_RESERVED_SHIFT: c_int = 24;
}

// Status block structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cau_sb_entry {
    pub data: __le32,
pub const CAU_SB_ENTRY_SB_PROD_MASK: c_uint = 0xFFFFFF;
pub const CAU_SB_ENTRY_SB_PROD_SHIFT: c_int = 0;
pub const CAU_SB_ENTRY_STATE0_MASK: c_uint = 0xF;
pub const CAU_SB_ENTRY_STATE0_SHIFT: c_int = 24;
pub const CAU_SB_ENTRY_STATE1_MASK: c_uint = 0xF;
pub const CAU_SB_ENTRY_STATE1_SHIFT: c_int = 28;
    pub params: __le32,
pub const CAU_SB_ENTRY_SB_TIMESET0_MASK: c_uint = 0x7F;
pub const CAU_SB_ENTRY_SB_TIMESET0_SHIFT: c_int = 0;
pub const CAU_SB_ENTRY_SB_TIMESET1_MASK: c_uint = 0x7F;
pub const CAU_SB_ENTRY_SB_TIMESET1_SHIFT: c_int = 7;
pub const CAU_SB_ENTRY_TIMER_RES0_MASK: c_uint = 0x3;
pub const CAU_SB_ENTRY_TIMER_RES0_SHIFT: c_int = 14;
pub const CAU_SB_ENTRY_TIMER_RES1_MASK: c_uint = 0x3;
pub const CAU_SB_ENTRY_TIMER_RES1_SHIFT: c_int = 16;
pub const CAU_SB_ENTRY_VF_NUMBER_MASK: c_uint = 0xFF;
pub const CAU_SB_ENTRY_VF_NUMBER_SHIFT: c_int = 18;
pub const CAU_SB_ENTRY_VF_VALID_MASK: c_uint = 0x1;
pub const CAU_SB_ENTRY_VF_VALID_SHIFT: c_int = 26;
pub const CAU_SB_ENTRY_PF_NUMBER_MASK: c_uint = 0xF;
pub const CAU_SB_ENTRY_PF_NUMBER_SHIFT: c_int = 27;
pub const CAU_SB_ENTRY_TPH_MASK: c_uint = 0x1;
pub const CAU_SB_ENTRY_TPH_SHIFT: c_int = 31;
}

// Igu cleanup bit values to distinguish between clean or producer consumer
// update.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum command_type_bit {
    IGU_COMMAND_TYPE_NOP = 0,
    IGU_COMMAND_TYPE_SET = 1,
    MAX_COMMAND_TYPE_BIT
}

// Core doorbell data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_db_data {
    pub params: u8,
pub const CORE_DB_DATA_DEST_MASK: c_uint = 0x3;
pub const CORE_DB_DATA_DEST_SHIFT: c_int = 0;
pub const CORE_DB_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const CORE_DB_DATA_AGG_CMD_SHIFT: c_int = 2;
pub const CORE_DB_DATA_BYPASS_EN_MASK: c_uint = 0x1;
pub const CORE_DB_DATA_BYPASS_EN_SHIFT: c_int = 4;
pub const CORE_DB_DATA_RESERVED_MASK: c_uint = 0x1;
pub const CORE_DB_DATA_RESERVED_SHIFT: c_int = 5;
pub const CORE_DB_DATA_AGG_VAL_SEL_MASK: c_uint = 0x3;
pub const CORE_DB_DATA_AGG_VAL_SEL_SHIFT: c_int = 6;
    pub agg_flags: u8,
    pub spq_prod: __le16,
}

// Enum of doorbell aggregative command selection
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum db_agg_cmd_sel {
    DB_AGG_CMD_NOP,
    DB_AGG_CMD_SET,
    DB_AGG_CMD_ADD,
    DB_AGG_CMD_MAX,
    MAX_DB_AGG_CMD_SEL
}

// Enum of doorbell destination
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum db_dest {
    DB_DEST_XCM,
    DB_DEST_UCM,
    DB_DEST_TCM,
    DB_NUM_DESTINATIONS,
    MAX_DB_DEST
}

// Enum of doorbell DPM types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum db_dpm_type {
    DPM_LEGACY,
    DPM_RDMA,
    DPM_L2_INLINE,
    DPM_L2_BD,
    MAX_DB_DPM_TYPE
}

// Structure for doorbell data, in L2 DPM mode, for 1st db in a DPM burst
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_l2_dpm_data {
    pub icid: __le16,
    pub bd_prod: __le16,
    pub params: __le32,
pub const DB_L2_DPM_DATA_SIZE_MASK: c_uint = 0x3F;
pub const DB_L2_DPM_DATA_SIZE_SHIFT: c_int = 0;
pub const DB_L2_DPM_DATA_DPM_TYPE_MASK: c_uint = 0x3;
pub const DB_L2_DPM_DATA_DPM_TYPE_SHIFT: c_int = 6;
pub const DB_L2_DPM_DATA_NUM_BDS_MASK: c_uint = 0xFF;
pub const DB_L2_DPM_DATA_NUM_BDS_SHIFT: c_int = 8;
pub const DB_L2_DPM_DATA_PKT_SIZE_MASK: c_uint = 0x7FF;
pub const DB_L2_DPM_DATA_PKT_SIZE_SHIFT: c_int = 16;
pub const DB_L2_DPM_DATA_RESERVED0_MASK: c_uint = 0x1;
pub const DB_L2_DPM_DATA_RESERVED0_SHIFT: c_int = 27;
pub const DB_L2_DPM_DATA_SGE_NUM_MASK: c_uint = 0x7;
pub const DB_L2_DPM_DATA_SGE_NUM_SHIFT: c_int = 28;
pub const DB_L2_DPM_DATA_TGFS_SRC_EN_MASK: c_uint = 0x1;
pub const DB_L2_DPM_DATA_TGFS_SRC_EN_SHIFT: c_int = 31;
}

// Structure for SGE in a DPM doorbell of type DPM_L2_BD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_l2_dpm_sge {
    pub addr: regpair,
    pub nbytes: __le16,
    pub bitfields: __le16,
pub const DB_L2_DPM_SGE_TPH_ST_INDEX_MASK: c_uint = 0x1FF;
pub const DB_L2_DPM_SGE_TPH_ST_INDEX_SHIFT: c_int = 0;
pub const DB_L2_DPM_SGE_RESERVED0_MASK: c_uint = 0x3;
pub const DB_L2_DPM_SGE_RESERVED0_SHIFT: c_int = 9;
pub const DB_L2_DPM_SGE_ST_VALID_MASK: c_uint = 0x1;
pub const DB_L2_DPM_SGE_ST_VALID_SHIFT: c_int = 11;
pub const DB_L2_DPM_SGE_RESERVED1_MASK: c_uint = 0xF;
pub const DB_L2_DPM_SGE_RESERVED1_SHIFT: c_int = 12;
    pub reserved2: __le32,
}

// Structure for doorbell address, in legacy mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_legacy_addr {
    pub addr: __le32,
pub const DB_LEGACY_ADDR_RESERVED0_MASK: c_uint = 0x3;
pub const DB_LEGACY_ADDR_RESERVED0_SHIFT: c_int = 0;
pub const DB_LEGACY_ADDR_DEMS_MASK: c_uint = 0x7;
pub const DB_LEGACY_ADDR_DEMS_SHIFT: c_int = 2;
pub const DB_LEGACY_ADDR_ICID_MASK: c_uint = 0x7FFFFFF;
pub const DB_LEGACY_ADDR_ICID_SHIFT: c_int = 5;
}

// Structure for doorbell address, in legacy mode, without DEMS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_legacy_wo_dems_addr {
    pub addr: __le32,
pub const DB_LEGACY_WO_DEMS_ADDR_RESERVED0_MASK: c_uint = 0x3;
pub const DB_LEGACY_WO_DEMS_ADDR_RESERVED0_SHIFT: c_int = 0;
pub const DB_LEGACY_WO_DEMS_ADDR_ICID_MASK: c_uint = 0x3FFFFFFF;
pub const DB_LEGACY_WO_DEMS_ADDR_ICID_SHIFT: c_int = 2;
}

// Structure for doorbell address, in PWM mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_pwm_addr {
    pub addr: __le32,
pub const DB_PWM_ADDR_RESERVED0_MASK: c_uint = 0x7;
pub const DB_PWM_ADDR_RESERVED0_SHIFT: c_int = 0;
pub const DB_PWM_ADDR_OFFSET_MASK: c_uint = 0x7F;
pub const DB_PWM_ADDR_OFFSET_SHIFT: c_int = 3;
pub const DB_PWM_ADDR_WID_MASK: c_uint = 0x3;
pub const DB_PWM_ADDR_WID_SHIFT: c_int = 10;
pub const DB_PWM_ADDR_DPI_MASK: c_uint = 0xFFFF;
pub const DB_PWM_ADDR_DPI_SHIFT: c_int = 12;
pub const DB_PWM_ADDR_RESERVED1_MASK: c_uint = 0xF;
pub const DB_PWM_ADDR_RESERVED1_SHIFT: c_int = 28;
}

// Parameters to RDMA firmware, passed in EDPM doorbell
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_rdma_24b_icid_dpm_params {
    pub params: __le32,
pub const DB_RDMA_24B_ICID_DPM_PARAMS_SIZE_MASK: c_uint = 0x3F;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_SIZE_SHIFT: c_int = 0;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_DPM_TYPE_MASK: c_uint = 0x3;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_DPM_TYPE_SHIFT: c_int = 6;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_OPCODE_MASK: c_uint = 0xFF;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_OPCODE_SHIFT: c_int = 8;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_ICID_EXT_MASK: c_uint = 0xFF;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_ICID_EXT_SHIFT: c_int = 16;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_INV_BYTE_CNT_MASK: c_uint = 0x7;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_INV_BYTE_CNT_SHIFT: c_int = 24;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_EXT_ICID_MODE_EN_MASK: c_uint = 0x1;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_EXT_ICID_MODE_EN_SHIFT: c_int = 27;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_COMPLETION_FLG_MASK: c_uint = 0x1;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_COMPLETION_FLG_SHIFT: c_int = 28;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_S_FLG_MASK: c_uint = 0x1;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_S_FLG_SHIFT: c_int = 29;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_RESERVED1_MASK: c_uint = 0x1;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_RESERVED1_SHIFT: c_int = 30;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_CONN_TYPE_IS_IWARP_MASK: c_uint = 0x1;
pub const DB_RDMA_24B_ICID_DPM_PARAMS_CONN_TYPE_IS_IWARP_SHIFT: c_int = 31;
}

// Parameters to RDMA firmware, passed in EDPM doorbell
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_rdma_dpm_params {
    pub params: __le32,
pub const DB_RDMA_DPM_PARAMS_SIZE_MASK: c_uint = 0x3F;
pub const DB_RDMA_DPM_PARAMS_SIZE_SHIFT: c_int = 0;
pub const DB_RDMA_DPM_PARAMS_DPM_TYPE_MASK: c_uint = 0x3;
pub const DB_RDMA_DPM_PARAMS_DPM_TYPE_SHIFT: c_int = 6;
pub const DB_RDMA_DPM_PARAMS_OPCODE_MASK: c_uint = 0xFF;
pub const DB_RDMA_DPM_PARAMS_OPCODE_SHIFT: c_int = 8;
pub const DB_RDMA_DPM_PARAMS_WQE_SIZE_MASK: c_uint = 0x7FF;
pub const DB_RDMA_DPM_PARAMS_WQE_SIZE_SHIFT: c_int = 16;
pub const DB_RDMA_DPM_PARAMS_RESERVED0_MASK: c_uint = 0x1;
pub const DB_RDMA_DPM_PARAMS_RESERVED0_SHIFT: c_int = 27;
pub const DB_RDMA_DPM_PARAMS_ACK_REQUEST_MASK: c_uint = 0x1;
pub const DB_RDMA_DPM_PARAMS_ACK_REQUEST_SHIFT: c_int = 28;
pub const DB_RDMA_DPM_PARAMS_S_FLG_MASK: c_uint = 0x1;
pub const DB_RDMA_DPM_PARAMS_S_FLG_SHIFT: c_int = 29;
pub const DB_RDMA_DPM_PARAMS_COMPLETION_FLG_MASK: c_uint = 0x1;
pub const DB_RDMA_DPM_PARAMS_COMPLETION_FLG_SHIFT: c_int = 30;
pub const DB_RDMA_DPM_PARAMS_CONN_TYPE_IS_IWARP_MASK: c_uint = 0x1;
pub const DB_RDMA_DPM_PARAMS_CONN_TYPE_IS_IWARP_SHIFT: c_int = 31;
}

// Structure for doorbell data, in RDMA DPM mode, for the first doorbell in a
// DPM burst.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_rdma_dpm_data {
    pub icid: __le16,
    pub prod_val: __le16,
    pub params: db_rdma_dpm_params,
}

// Igu interrupt command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_int_cmd {
    IGU_INT_ENABLE	= 0,
    IGU_INT_DISABLE = 1,
    IGU_INT_NOP	= 2,
    IGU_INT_NOP2	= 3,
    MAX_IGU_INT_CMD
}

// IGU producer or consumer update command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_prod_cons_update {
    pub sb_id_and_flags: __le32,
pub const IGU_PROD_CONS_UPDATE_SB_INDEX_MASK: c_uint = 0xFFFFFF;
pub const IGU_PROD_CONS_UPDATE_SB_INDEX_SHIFT: c_int = 0;
pub const IGU_PROD_CONS_UPDATE_UPDATE_FLAG_MASK: c_uint = 0x1;
pub const IGU_PROD_CONS_UPDATE_UPDATE_FLAG_SHIFT: c_int = 24;
pub const IGU_PROD_CONS_UPDATE_ENABLE_INT_MASK: c_uint = 0x3;
pub const IGU_PROD_CONS_UPDATE_ENABLE_INT_SHIFT: c_int = 25;
pub const IGU_PROD_CONS_UPDATE_SEGMENT_ACCESS_MASK: c_uint = 0x1;
pub const IGU_PROD_CONS_UPDATE_SEGMENT_ACCESS_SHIFT: c_int = 27;
pub const IGU_PROD_CONS_UPDATE_TIMER_MASK_MASK: c_uint = 0x1;
pub const IGU_PROD_CONS_UPDATE_TIMER_MASK_SHIFT: c_int = 28;
pub const IGU_PROD_CONS_UPDATE_RESERVED0_MASK: c_uint = 0x3;
pub const IGU_PROD_CONS_UPDATE_RESERVED0_SHIFT: c_int = 29;
pub const IGU_PROD_CONS_UPDATE_COMMAND_TYPE_MASK: c_uint = 0x1;
pub const IGU_PROD_CONS_UPDATE_COMMAND_TYPE_SHIFT: c_int = 31;
    pub reserved1: __le32,
}

// Igu segments access for default status block only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_seg_access {
    IGU_SEG_ACCESS_REG	= 0,
    IGU_SEG_ACCESS_ATTN	= 1,
    MAX_IGU_SEG_ACCESS
}

// Enumeration for L3 type field of parsing_and_err_flags.
// L3Type: 0 - unknown (not ip), 1 - Ipv4, 2 - Ipv6
// (This field can be filled according to the last-ethertype)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l3_type {
    e_l3_type_unknown,
    e_l3_type_ipv4,
    e_l3_type_ipv6,
    MAX_L3_TYPE
}

// Enumeration for l4Protocol field of parsing_and_err_flags.
// L4-protocol: 0 - none, 1 - TCP, 2 - UDP.
// If the packet is IPv4 fragment, and its not the first fragment, the
// protocol-type should be set to none.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l4_protocol {
    e_l4_protocol_none,
    e_l4_protocol_tcp,
    e_l4_protocol_udp,
    MAX_L4_PROTOCOL
}

// Parsing and error flags field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parsing_and_err_flags {
    pub flags: __le16,
pub const PARSING_AND_ERR_FLAGS_L3TYPE_MASK: c_uint = 0x3;
pub const PARSING_AND_ERR_FLAGS_L3TYPE_SHIFT: c_int = 0;
pub const PARSING_AND_ERR_FLAGS_L4PROTOCOL_MASK: c_uint = 0x3;
pub const PARSING_AND_ERR_FLAGS_L4PROTOCOL_SHIFT: c_int = 2;
pub const PARSING_AND_ERR_FLAGS_IPV4FRAG_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_IPV4FRAG_SHIFT: c_int = 4;
pub const PARSING_AND_ERR_FLAGS_TAG8021QEXIST_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TAG8021QEXIST_SHIFT: c_int = 5;
pub const PARSING_AND_ERR_FLAGS_L4CHKSMWASCALCULATED_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_L4CHKSMWASCALCULATED_SHIFT: c_int = 6;
pub const PARSING_AND_ERR_FLAGS_TIMESYNCPKT_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TIMESYNCPKT_SHIFT: c_int = 7;
pub const PARSING_AND_ERR_FLAGS_TIMESTAMPRECORDED_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TIMESTAMPRECORDED_SHIFT: c_int = 8;
pub const PARSING_AND_ERR_FLAGS_IPHDRERROR_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_IPHDRERROR_SHIFT: c_int = 9;
pub const PARSING_AND_ERR_FLAGS_L4CHKSMERROR_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_L4CHKSMERROR_SHIFT: c_int = 10;
pub const PARSING_AND_ERR_FLAGS_TUNNELEXIST_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TUNNELEXIST_SHIFT: c_int = 11;
pub const PARSING_AND_ERR_FLAGS_TUNNEL8021QTAGEXIST_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TUNNEL8021QTAGEXIST_SHIFT: c_int = 12;
pub const PARSING_AND_ERR_FLAGS_TUNNELIPHDRERROR_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TUNNELIPHDRERROR_SHIFT: c_int = 13;
pub const PARSING_AND_ERR_FLAGS_TUNNELL4CHKSMWASCALCULATED_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TUNNELL4CHKSMWASCALCULATED_SHIFT: c_int = 14;
pub const PARSING_AND_ERR_FLAGS_TUNNELL4CHKSMERROR_MASK: c_uint = 0x1;
pub const PARSING_AND_ERR_FLAGS_TUNNELL4CHKSMERROR_SHIFT: c_int = 15;
}

// Parsing error flags bitmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parsing_err_flags {
    pub flags: __le16,
pub const PARSING_ERR_FLAGS_MAC_ERROR_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_MAC_ERROR_SHIFT: c_int = 0;
pub const PARSING_ERR_FLAGS_TRUNC_ERROR_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_TRUNC_ERROR_SHIFT: c_int = 1;
pub const PARSING_ERR_FLAGS_PKT_TOO_SMALL_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_PKT_TOO_SMALL_SHIFT: c_int = 2;
pub const PARSING_ERR_FLAGS_ANY_HDR_MISSING_TAG_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ANY_HDR_MISSING_TAG_SHIFT: c_int = 3;
pub const PARSING_ERR_FLAGS_ANY_HDR_IP_VER_MISMTCH_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ANY_HDR_IP_VER_MISMTCH_SHIFT: c_int = 4;
pub const PARSING_ERR_FLAGS_ANY_HDR_IP_V4_HDR_LEN_TOO_SMALL_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ANY_HDR_IP_V4_HDR_LEN_TOO_SMALL_SHIFT: c_int = 5;
pub const PARSING_ERR_FLAGS_ANY_HDR_IP_BAD_TOTAL_LEN_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ANY_HDR_IP_BAD_TOTAL_LEN_SHIFT: c_int = 6;
pub const PARSING_ERR_FLAGS_IP_V4_CHKSM_ERROR_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_IP_V4_CHKSM_ERROR_SHIFT: c_int = 7;
pub const PARSING_ERR_FLAGS_ANY_HDR_L4_IP_LEN_MISMTCH_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ANY_HDR_L4_IP_LEN_MISMTCH_SHIFT: c_int = 8;
pub const PARSING_ERR_FLAGS_ZERO_UDP_IP_V6_CHKSM_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ZERO_UDP_IP_V6_CHKSM_SHIFT: c_int = 9;
pub const PARSING_ERR_FLAGS_INNER_L4_CHKSM_ERROR_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_INNER_L4_CHKSM_ERROR_SHIFT: c_int = 10;
pub const PARSING_ERR_FLAGS_ANY_HDR_ZERO_TTL_OR_HOP_LIM_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_ANY_HDR_ZERO_TTL_OR_HOP_LIM_SHIFT: c_int = 11;
pub const PARSING_ERR_FLAGS_NON_8021Q_TAG_EXISTS_IN_BOTH_HDRS_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_NON_8021Q_TAG_EXISTS_IN_BOTH_HDRS_SHIFT: c_int = 12;
pub const PARSING_ERR_FLAGS_GENEVE_OPTION_OVERSIZED_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_GENEVE_OPTION_OVERSIZED_SHIFT: c_int = 13;
pub const PARSING_ERR_FLAGS_TUNNEL_IP_V4_CHKSM_ERROR_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_TUNNEL_IP_V4_CHKSM_ERROR_SHIFT: c_int = 14;
pub const PARSING_ERR_FLAGS_TUNNEL_L4_CHKSM_ERROR_MASK: c_uint = 0x1;
pub const PARSING_ERR_FLAGS_TUNNEL_L4_CHKSM_ERROR_SHIFT: c_int = 15;
}

// Pb context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb_context {
    pub crc: [__le32; 4],
}

// Concrete Function ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp_concrete_fid {
    pub fid: __le16,
pub const PXP_CONCRETE_FID_PFID_MASK: c_uint = 0xF;
pub const PXP_CONCRETE_FID_PFID_SHIFT: c_int = 0;
pub const PXP_CONCRETE_FID_PORT_MASK: c_uint = 0x3;
pub const PXP_CONCRETE_FID_PORT_SHIFT: c_int = 4;
pub const PXP_CONCRETE_FID_PATH_MASK: c_uint = 0x1;
pub const PXP_CONCRETE_FID_PATH_SHIFT: c_int = 6;
pub const PXP_CONCRETE_FID_VFVALID_MASK: c_uint = 0x1;
pub const PXP_CONCRETE_FID_VFVALID_SHIFT: c_int = 7;
pub const PXP_CONCRETE_FID_VFID_MASK: c_uint = 0xFF;
pub const PXP_CONCRETE_FID_VFID_SHIFT: c_int = 8;
}

// Concrete Function ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp_pretend_concrete_fid {
    pub fid: __le16,
pub const PXP_PRETEND_CONCRETE_FID_PFID_MASK: c_uint = 0xF;
pub const PXP_PRETEND_CONCRETE_FID_PFID_SHIFT: c_int = 0;
pub const PXP_PRETEND_CONCRETE_FID_RESERVED_MASK: c_uint = 0x7;
pub const PXP_PRETEND_CONCRETE_FID_RESERVED_SHIFT: c_int = 4;
pub const PXP_PRETEND_CONCRETE_FID_VFVALID_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CONCRETE_FID_VFVALID_SHIFT: c_int = 7;
pub const PXP_PRETEND_CONCRETE_FID_VFID_MASK: c_uint = 0xFF;
pub const PXP_PRETEND_CONCRETE_FID_VFID_SHIFT: c_int = 8;
}

// Function ID
#[repr(C)]
#[derive(Copy, Clone)]
pub union pxp_pretend_fid {
    pub concrete_fid: pxp_pretend_concrete_fid,
    pub opaque_fid: __le16,
}

// Pxp Pretend Command Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp_pretend_cmd {
    pub fid: pxp_pretend_fid,
    pub control: __le16,
pub const PXP_PRETEND_CMD_PATH_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CMD_PATH_SHIFT: c_int = 0;
pub const PXP_PRETEND_CMD_USE_PORT_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CMD_USE_PORT_SHIFT: c_int = 1;
pub const PXP_PRETEND_CMD_PORT_MASK: c_uint = 0x3;
pub const PXP_PRETEND_CMD_PORT_SHIFT: c_int = 2;
pub const PXP_PRETEND_CMD_RESERVED0_MASK: c_uint = 0xF;
pub const PXP_PRETEND_CMD_RESERVED0_SHIFT: c_int = 4;
pub const PXP_PRETEND_CMD_RESERVED1_MASK: c_uint = 0xF;
pub const PXP_PRETEND_CMD_RESERVED1_SHIFT: c_int = 8;
pub const PXP_PRETEND_CMD_PRETEND_PATH_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CMD_PRETEND_PATH_SHIFT: c_int = 12;
pub const PXP_PRETEND_CMD_PRETEND_PORT_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CMD_PRETEND_PORT_SHIFT: c_int = 13;
pub const PXP_PRETEND_CMD_PRETEND_FUNCTION_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CMD_PRETEND_FUNCTION_SHIFT: c_int = 14;
pub const PXP_PRETEND_CMD_IS_CONCRETE_MASK: c_uint = 0x1;
pub const PXP_PRETEND_CMD_IS_CONCRETE_SHIFT: c_int = 15;
}

// PTT Record in PXP Admin Window
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp_ptt_entry {
    pub offset: __le32,
pub const PXP_PTT_ENTRY_OFFSET_MASK: c_uint = 0x7FFFFF;
pub const PXP_PTT_ENTRY_OFFSET_SHIFT: c_int = 0;
pub const PXP_PTT_ENTRY_RESERVED0_MASK: c_uint = 0x1FF;
pub const PXP_PTT_ENTRY_RESERVED0_SHIFT: c_int = 23;
    pub pretend: pxp_pretend_cmd,
}

// VF Zone A Permission Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxp_vf_zone_a_permission {
    pub control: __le32,
pub const PXP_VF_ZONE_A_PERMISSION_VFID_MASK: c_uint = 0xFF;
pub const PXP_VF_ZONE_A_PERMISSION_VFID_SHIFT: c_int = 0;
pub const PXP_VF_ZONE_A_PERMISSION_VALID_MASK: c_uint = 0x1;
pub const PXP_VF_ZONE_A_PERMISSION_VALID_SHIFT: c_int = 8;
pub const PXP_VF_ZONE_A_PERMISSION_RESERVED0_MASK: c_uint = 0x7F;
pub const PXP_VF_ZONE_A_PERMISSION_RESERVED0_SHIFT: c_int = 9;
pub const PXP_VF_ZONE_A_PERMISSION_RESERVED1_MASK: c_uint = 0xFFFF;
pub const PXP_VF_ZONE_A_PERMISSION_RESERVED1_SHIFT: c_int = 16;
}

// Rdif context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdif_task_context {
    pub initial_ref_tag: __le32,
    pub app_tag_value: __le16,
    pub app_tag_mask: __le16,
    pub flags0: u8,
pub const RDIF_TASK_CONTEXT_IGNORE_APP_TAG_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_IGNORE_APP_TAG_SHIFT: c_int = 0;
pub const RDIF_TASK_CONTEXT_INITIAL_REF_TAG_VALID_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_INITIAL_REF_TAG_VALID_SHIFT: c_int = 1;
pub const RDIF_TASK_CONTEXT_HOST_GUARD_TYPE_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_HOST_GUARD_TYPE_SHIFT: c_int = 2;
pub const RDIF_TASK_CONTEXT_SET_ERROR_WITH_EOP_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_SET_ERROR_WITH_EOP_SHIFT: c_int = 3;
pub const RDIF_TASK_CONTEXT_PROTECTION_TYPE_MASK: c_uint = 0x3;
pub const RDIF_TASK_CONTEXT_PROTECTION_TYPE_SHIFT: c_int = 4;
pub const RDIF_TASK_CONTEXT_CRC_SEED_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_CRC_SEED_SHIFT: c_int = 6;
pub const RDIF_TASK_CONTEXT_KEEP_REF_TAG_CONST_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_KEEP_REF_TAG_CONST_SHIFT: c_int = 7;
    pub partial_dif_data: [u8; 7],
    pub partial_crc_value: __le16,
    pub partial_checksum_value: __le16,
    pub offset_in_io: __le32,
    pub flags1: __le16,
pub const RDIF_TASK_CONTEXT_VALIDATE_GUARD_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_VALIDATE_GUARD_SHIFT: c_int = 0;
pub const RDIF_TASK_CONTEXT_VALIDATE_APP_TAG_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_VALIDATE_APP_TAG_SHIFT: c_int = 1;
pub const RDIF_TASK_CONTEXT_VALIDATE_REF_TAG_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_VALIDATE_REF_TAG_SHIFT: c_int = 2;
pub const RDIF_TASK_CONTEXT_FORWARD_GUARD_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_FORWARD_GUARD_SHIFT: c_int = 3;
pub const RDIF_TASK_CONTEXT_FORWARD_APP_TAG_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_FORWARD_APP_TAG_SHIFT: c_int = 4;
pub const RDIF_TASK_CONTEXT_FORWARD_REF_TAG_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_FORWARD_REF_TAG_SHIFT: c_int = 5;
pub const RDIF_TASK_CONTEXT_INTERVAL_SIZE_MASK: c_uint = 0x7;
pub const RDIF_TASK_CONTEXT_INTERVAL_SIZE_SHIFT: c_int = 6;
pub const RDIF_TASK_CONTEXT_HOST_INTERFACE_MASK: c_uint = 0x3;
pub const RDIF_TASK_CONTEXT_HOST_INTERFACE_SHIFT: c_int = 9;
pub const RDIF_TASK_CONTEXT_DIF_BEFORE_DATA_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_DIF_BEFORE_DATA_SHIFT: c_int = 11;
pub const RDIF_TASK_CONTEXT_RESERVED0_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_RESERVED0_SHIFT: c_int = 12;
pub const RDIF_TASK_CONTEXT_NETWORK_INTERFACE_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_NETWORK_INTERFACE_SHIFT: c_int = 13;
pub const RDIF_TASK_CONTEXT_FORWARD_APP_TAG_WITH_MASK_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_FORWARD_APP_TAG_WITH_MASK_SHIFT: c_int = 14;
pub const RDIF_TASK_CONTEXT_FORWARD_REF_TAG_WITH_MASK_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_FORWARD_REF_TAG_WITH_MASK_SHIFT: c_int = 15;
    pub state: __le16,
pub const RDIF_TASK_CONTEXT_RECEIVED_DIF_BYTES_LEFT_MASK: c_uint = 0xF;
pub const RDIF_TASK_CONTEXT_RECEIVED_DIF_BYTES_LEFT_SHIFT: c_int = 0;
pub const RDIF_TASK_CONTEXT_TRANSMITED_DIF_BYTES_LEFT_MASK: c_uint = 0xF;
pub const RDIF_TASK_CONTEXT_TRANSMITED_DIF_BYTES_LEFT_SHIFT: c_int = 4;
pub const RDIF_TASK_CONTEXT_ERROR_IN_IO_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_ERROR_IN_IO_SHIFT: c_int = 8;
pub const RDIF_TASK_CONTEXT_CHECKSUM_OVERFLOW_MASK: c_uint = 0x1;
pub const RDIF_TASK_CONTEXT_CHECKSUM_OVERFLOW_SHIFT: c_int = 9;
pub const RDIF_TASK_CONTEXT_REF_TAG_MASK_MASK: c_uint = 0xF;
pub const RDIF_TASK_CONTEXT_REF_TAG_MASK_SHIFT: c_int = 10;
pub const RDIF_TASK_CONTEXT_RESERVED1_MASK: c_uint = 0x3;
pub const RDIF_TASK_CONTEXT_RESERVED1_SHIFT: c_int = 14;
    pub reserved2: __le32,
}

// Searcher Table struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_entry_header {
    pub flags: __le32,
pub const SRC_ENTRY_HEADER_NEXT_PTR_TYPE_MASK: c_uint = 0x1;
pub const SRC_ENTRY_HEADER_NEXT_PTR_TYPE_SHIFT: c_int = 0;
pub const SRC_ENTRY_HEADER_EMPTY_MASK: c_uint = 0x1;
pub const SRC_ENTRY_HEADER_EMPTY_SHIFT: c_int = 1;
pub const SRC_ENTRY_HEADER_RESERVED_MASK: c_uint = 0x3FFFFFFF;
pub const SRC_ENTRY_HEADER_RESERVED_SHIFT: c_int = 2;
    pub magic_number: __le32,
    pub next_ptr: regpair,
}

// Enumeration for address type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum src_header_next_ptr_type_enum {
    e_physical_addr,
    e_logical_addr,
    MAX_SRC_HEADER_NEXT_PTR_TYPE_ENUM
}

// Status block structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_block {
    pub pi_array: [__le16; PIS_PER_SB],
    pub sb_num: __le32,
pub const STATUS_BLOCK_SB_NUM_MASK: c_uint = 0x1FF;
pub const STATUS_BLOCK_SB_NUM_SHIFT: c_int = 0;
pub const STATUS_BLOCK_ZERO_PAD_MASK: c_uint = 0x7F;
pub const STATUS_BLOCK_ZERO_PAD_SHIFT: c_int = 9;
pub const STATUS_BLOCK_ZERO_PAD2_MASK: c_uint = 0xFFFF;
pub const STATUS_BLOCK_ZERO_PAD2_SHIFT: c_int = 16;
    pub prod_index: __le32,
pub const STATUS_BLOCK_PROD_INDEX_MASK: c_uint = 0xFFFFFF;
pub const STATUS_BLOCK_PROD_INDEX_SHIFT: c_int = 0;
pub const STATUS_BLOCK_ZERO_PAD3_MASK: c_uint = 0xFF;
pub const STATUS_BLOCK_ZERO_PAD3_SHIFT: c_int = 24;
}

// Tdif context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdif_task_context {
    pub initial_ref_tag: __le32,
    pub app_tag_value: __le16,
    pub app_tag_mask: __le16,
    pub partial_crc_value_b: __le16,
    pub partial_checksum_value_b: __le16,
    pub stateB: __le16,
pub const TDIF_TASK_CONTEXT_RECEIVED_DIF_BYTES_LEFT_B_MASK: c_uint = 0xF;
pub const TDIF_TASK_CONTEXT_RECEIVED_DIF_BYTES_LEFT_B_SHIFT: c_int = 0;
pub const TDIF_TASK_CONTEXT_TRANSMITED_DIF_BYTES_LEFT_B_MASK: c_uint = 0xF;
pub const TDIF_TASK_CONTEXT_TRANSMITED_DIF_BYTES_LEFT_B_SHIFT: c_int = 4;
pub const TDIF_TASK_CONTEXT_ERROR_IN_IO_B_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_ERROR_IN_IO_B_SHIFT: c_int = 8;
pub const TDIF_TASK_CONTEXT_CHECKSUM_VERFLOW_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_CHECKSUM_VERFLOW_SHIFT: c_int = 9;
pub const TDIF_TASK_CONTEXT_RESERVED0_MASK: c_uint = 0x3F;
pub const TDIF_TASK_CONTEXT_RESERVED0_SHIFT: c_int = 10;
    pub reserved1: u8,
    pub flags0: u8,
pub const TDIF_TASK_CONTEXT_IGNORE_APP_TAG_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_IGNORE_APP_TAG_SHIFT: c_int = 0;
pub const TDIF_TASK_CONTEXT_INITIAL_REF_TAG_VALID_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_INITIAL_REF_TAG_VALID_SHIFT: c_int = 1;
pub const TDIF_TASK_CONTEXT_HOST_GUARD_TYPE_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_HOST_GUARD_TYPE_SHIFT: c_int = 2;
pub const TDIF_TASK_CONTEXT_SET_ERROR_WITH_EOP_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_SET_ERROR_WITH_EOP_SHIFT: c_int = 3;
pub const TDIF_TASK_CONTEXT_PROTECTION_TYPE_MASK: c_uint = 0x3;
pub const TDIF_TASK_CONTEXT_PROTECTION_TYPE_SHIFT: c_int = 4;
pub const TDIF_TASK_CONTEXT_CRC_SEED_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_CRC_SEED_SHIFT: c_int = 6;
pub const TDIF_TASK_CONTEXT_RESERVED2_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_RESERVED2_SHIFT: c_int = 7;
    pub flags1: __le32,
pub const TDIF_TASK_CONTEXT_VALIDATE_GUARD_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_VALIDATE_GUARD_SHIFT: c_int = 0;
pub const TDIF_TASK_CONTEXT_VALIDATE_APP_TAG_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_VALIDATE_APP_TAG_SHIFT: c_int = 1;
pub const TDIF_TASK_CONTEXT_VALIDATE_REF_TAG_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_VALIDATE_REF_TAG_SHIFT: c_int = 2;
pub const TDIF_TASK_CONTEXT_FORWARD_GUARD_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_FORWARD_GUARD_SHIFT: c_int = 3;
pub const TDIF_TASK_CONTEXT_FORWARD_APP_TAG_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_FORWARD_APP_TAG_SHIFT: c_int = 4;
pub const TDIF_TASK_CONTEXT_FORWARD_REF_TAG_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_FORWARD_REF_TAG_SHIFT: c_int = 5;
pub const TDIF_TASK_CONTEXT_INTERVAL_SIZE_MASK: c_uint = 0x7;
pub const TDIF_TASK_CONTEXT_INTERVAL_SIZE_SHIFT: c_int = 6;
pub const TDIF_TASK_CONTEXT_HOST_INTERFACE_MASK: c_uint = 0x3;
pub const TDIF_TASK_CONTEXT_HOST_INTERFACE_SHIFT: c_int = 9;
pub const TDIF_TASK_CONTEXT_DIF_BEFORE_DATA_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_DIF_BEFORE_DATA_SHIFT: c_int = 11;
pub const TDIF_TASK_CONTEXT_RESERVED3_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_RESERVED3_SHIFT: c_int = 12;
pub const TDIF_TASK_CONTEXT_NETWORK_INTERFACE_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_NETWORK_INTERFACE_SHIFT: c_int = 13;
pub const TDIF_TASK_CONTEXT_RECEIVED_DIF_BYTES_LEFT_A_MASK: c_uint = 0xF;
pub const TDIF_TASK_CONTEXT_RECEIVED_DIF_BYTES_LEFT_A_SHIFT: c_int = 14;
pub const TDIF_TASK_CONTEXT_TRANSMITED_DIF_BYTES_LEFT_A_MASK: c_uint = 0xF;
pub const TDIF_TASK_CONTEXT_TRANSMITED_DIF_BYTES_LEFT_A_SHIFT: c_int = 18;
pub const TDIF_TASK_CONTEXT_ERROR_IN_IO_A_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_ERROR_IN_IO_A_SHIFT: c_int = 22;
pub const TDIF_TASK_CONTEXT_CHECKSUM_OVERFLOW_A_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_CHECKSUM_OVERFLOW_A_SHIFT: c_int = 23;
pub const TDIF_TASK_CONTEXT_REF_TAG_MASK_MASK: c_uint = 0xF;
pub const TDIF_TASK_CONTEXT_REF_TAG_MASK_SHIFT: c_int = 24;
pub const TDIF_TASK_CONTEXT_FORWARD_APP_TAG_WITH_MASK_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_FORWARD_APP_TAG_WITH_MASK_SHIFT: c_int = 28;
pub const TDIF_TASK_CONTEXT_FORWARD_REF_TAG_WITH_MASK_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_FORWARD_REF_TAG_WITH_MASK_SHIFT: c_int = 29;
pub const TDIF_TASK_CONTEXT_KEEP_REF_TAG_CONST_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_KEEP_REF_TAG_CONST_SHIFT: c_int = 30;
pub const TDIF_TASK_CONTEXT_RESERVED4_MASK: c_uint = 0x1;
pub const TDIF_TASK_CONTEXT_RESERVED4_SHIFT: c_int = 31;
    pub offset_in_io_b: __le32,
    pub partial_crc_value_a: __le16,
    pub partial_checksum_value_a: __le16,
    pub offset_in_io_a: __le32,
    pub partial_dif_data_a: [u8; 8],
    pub partial_dif_data_b: [u8; 8],
}

// Timers context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timers_context {
    pub logical_client_0: __le32,
pub const TIMERS_CONTEXT_EXPIRATIONTIMELC0_MASK: c_uint = 0x7FFFFFF;
pub const TIMERS_CONTEXT_EXPIRATIONTIMELC0_SHIFT: c_int = 0;
pub const TIMERS_CONTEXT_RESERVED0_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_RESERVED0_SHIFT: c_int = 27;
pub const TIMERS_CONTEXT_VALIDLC0_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_VALIDLC0_SHIFT: c_int = 28;
pub const TIMERS_CONTEXT_ACTIVELC0_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_ACTIVELC0_SHIFT: c_int = 29;
pub const TIMERS_CONTEXT_RESERVED1_MASK: c_uint = 0x3;
pub const TIMERS_CONTEXT_RESERVED1_SHIFT: c_int = 30;
    pub logical_client_1: __le32,
pub const TIMERS_CONTEXT_EXPIRATIONTIMELC1_MASK: c_uint = 0x7FFFFFF;
pub const TIMERS_CONTEXT_EXPIRATIONTIMELC1_SHIFT: c_int = 0;
pub const TIMERS_CONTEXT_RESERVED2_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_RESERVED2_SHIFT: c_int = 27;
pub const TIMERS_CONTEXT_VALIDLC1_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_VALIDLC1_SHIFT: c_int = 28;
pub const TIMERS_CONTEXT_ACTIVELC1_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_ACTIVELC1_SHIFT: c_int = 29;
pub const TIMERS_CONTEXT_RESERVED3_MASK: c_uint = 0x3;
pub const TIMERS_CONTEXT_RESERVED3_SHIFT: c_int = 30;
    pub logical_client_2: __le32,
pub const TIMERS_CONTEXT_EXPIRATIONTIMELC2_MASK: c_uint = 0x7FFFFFF;
pub const TIMERS_CONTEXT_EXPIRATIONTIMELC2_SHIFT: c_int = 0;
pub const TIMERS_CONTEXT_RESERVED4_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_RESERVED4_SHIFT: c_int = 27;
pub const TIMERS_CONTEXT_VALIDLC2_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_VALIDLC2_SHIFT: c_int = 28;
pub const TIMERS_CONTEXT_ACTIVELC2_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_ACTIVELC2_SHIFT: c_int = 29;
pub const TIMERS_CONTEXT_RESERVED5_MASK: c_uint = 0x3;
pub const TIMERS_CONTEXT_RESERVED5_SHIFT: c_int = 30;
    pub host_expiration_fields: __le32,
pub const TIMERS_CONTEXT_HOSTEXPRIRATIONVALUE_MASK: c_uint = 0x7FFFFFF;
pub const TIMERS_CONTEXT_HOSTEXPRIRATIONVALUE_SHIFT: c_int = 0;
pub const TIMERS_CONTEXT_RESERVED6_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_RESERVED6_SHIFT: c_int = 27;
pub const TIMERS_CONTEXT_HOSTEXPRIRATIONVALID_MASK: c_uint = 0x1;
pub const TIMERS_CONTEXT_HOSTEXPRIRATIONVALID_SHIFT: c_int = 28;
pub const TIMERS_CONTEXT_RESERVED7_MASK: c_uint = 0x7;
pub const TIMERS_CONTEXT_RESERVED7_SHIFT: c_int = 29;
}

// Enum for next_protocol field of tunnel_parsing_flags / tunnelTypeDesc
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tunnel_next_protocol {
    e_unknown = 0,
    e_l2 = 1,
    e_ipv4 = 2,
    e_ipv6 = 3,
    MAX_TUNNEL_NEXT_PROTOCOL
}

