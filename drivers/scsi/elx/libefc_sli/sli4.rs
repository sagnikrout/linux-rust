//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/libefc_sli/sli4.h
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
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//
// All common SLI-4 structures and function prototypes.
//

//
// Common SLI-4 register offsets and field definitions
//
// SLI_INTF - SLI Interface Definition Register
pub const SLI4_INTF_REG: c_uint = 0x0058;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_intf {
    SLI4_INTF_REV_SHIFT		= 4,
    SLI4_INTF_REV_MASK		= 0xf0,

    SLI4_INTF_REV_S3		= 0x30,
    SLI4_INTF_REV_S4		= 0x40,

    SLI4_INTF_FAMILY_SHIFT		= 8,
    SLI4_INTF_FAMILY_MASK		= 0x0f00,

    SLI4_FAMILY_CHECK_ASIC_TYPE	= 0x0f00,

    SLI4_INTF_IF_TYPE_SHIFT		= 12,
    SLI4_INTF_IF_TYPE_MASK		= 0xf000,

    SLI4_INTF_IF_TYPE_2		= 0x2000,
    SLI4_INTF_IF_TYPE_6		= 0x6000,

    SLI4_INTF_VALID_SHIFT		= 29,
    SLI4_INTF_VALID_MASK		= 0xe0000000,

    SLI4_INTF_VALID_VALUE		= 0xc0000000,
}

// ASIC_ID - SLI ASIC Type and Revision Register
pub const SLI4_ASIC_ID_REG: c_uint = 0x009c;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_asic {
    SLI4_ASIC_GEN_SHIFT	= 8,
    SLI4_ASIC_GEN_MASK	= 0xff00,
    SLI4_ASIC_GEN_5		= 0x0b00,
    SLI4_ASIC_GEN_6		= 0x0c00,
    SLI4_ASIC_GEN_7		= 0x0d00,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_acic_revisions {
    SLI4_ASIC_REV_A0	= 0x00,
    SLI4_ASIC_REV_A1	= 0x01,
    SLI4_ASIC_REV_A2	= 0x02,
    SLI4_ASIC_REV_A3	= 0x03,
    SLI4_ASIC_REV_B0	= 0x10,
    SLI4_ASIC_REV_B1	= 0x11,
    SLI4_ASIC_REV_B2	= 0x12,
    SLI4_ASIC_REV_C0	= 0x20,
    SLI4_ASIC_REV_C1	= 0x21,
    SLI4_ASIC_REV_C2	= 0x22,
    SLI4_ASIC_REV_D0	= 0x30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_asic_entry_t {
    pub rev_id: u32,
    pub family: u32,
}

// BMBX - Bootstrap Mailbox Register
pub const SLI4_BMBX_REG: c_uint = 0x0160;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_bmbx {
    SLI4_BMBX_MASK_HI	= 0x3,
    SLI4_BMBX_MASK_LO	= 0xf,
    SLI4_BMBX_RDY		= 1 << 0,
    SLI4_BMBX_HI		= 1 << 1,
    SLI4_BMBX_SIZE		= 256,
}

// SLIPORT_CONTROL - SLI Port Control Register
pub const SLI4_PORT_CTRL_REG: c_uint = 0x0408;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_port_ctrl {
    SLI4_PORT_CTRL_IP	= 1u << 27,
    SLI4_PORT_CTRL_IDIS	= 1u << 22,
    SLI4_PORT_CTRL_FDD	= 1u << 31,
}

// SLI4_SLIPORT_ERROR - SLI Port Error Register
pub const SLI4_PORT_ERROR1: c_uint = 0x040c;
pub const SLI4_PORT_ERROR2: c_uint = 0x0410;
// EQCQ_DOORBELL - EQ and CQ Doorbell Register
pub const SLI4_EQCQ_DB_REG: c_uint = 0x120;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_eqcq_e {
    SLI4_EQ_ID_LO_MASK	= 0x01ff,

    SLI4_CQ_ID_LO_MASK	= 0x03ff,

    SLI4_EQCQ_CI_EQ		= 0x0200,

    SLI4_EQCQ_QT_EQ		= 0x00000400,
    SLI4_EQCQ_QT_CQ		= 0x00000000,

    SLI4_EQCQ_ID_HI_SHIFT	= 11,
    SLI4_EQCQ_ID_HI_MASK	= 0xf800,

    SLI4_EQCQ_NUM_SHIFT	= 16,
    SLI4_EQCQ_NUM_MASK	= 0x1fff0000,

    SLI4_EQCQ_ARM		= 0x20000000,
    SLI4_EQCQ_UNARM		= 0x00000000,
}

// EQ_DOORBELL - EQ Doorbell Register for IF_TYPE = 6
pub const SLI4_IF6_EQ_DB_REG: c_uint = 0x120;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_eq_e {
    SLI4_IF6_EQ_ID_MASK	= 0x0fff,

    SLI4_IF6_EQ_NUM_SHIFT	= 16,
    SLI4_IF6_EQ_NUM_MASK	= 0x1fff0000,
}

// CQ_DOORBELL - CQ Doorbell Register for IF_TYPE = 6
pub const SLI4_IF6_CQ_DB_REG: c_uint = 0xc0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cq_e {
    SLI4_IF6_CQ_ID_MASK	= 0xffff,

    SLI4_IF6_CQ_NUM_SHIFT	= 16,
    SLI4_IF6_CQ_NUM_MASK	= 0x1fff0000,
}

// MQ_DOORBELL - MQ Doorbell Register
pub const SLI4_MQ_DB_REG: c_uint = 0x0140;
pub const SLI4_IF6_MQ_DB_REG: c_uint = 0x0160;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_mq_e {
    SLI4_MQ_ID_MASK		= 0xffff,

    SLI4_MQ_NUM_SHIFT	= 16,
    SLI4_MQ_NUM_MASK	= 0x3fff0000,
}

// RQ_DOORBELL - RQ Doorbell Register
pub const SLI4_RQ_DB_REG: c_uint = 0x0a0;
pub const SLI4_IF6_RQ_DB_REG: c_uint = 0x0080;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rq_e {
    SLI4_RQ_DB_ID_MASK	= 0xffff,

    SLI4_RQ_DB_NUM_SHIFT	= 16,
    SLI4_RQ_DB_NUM_MASK	= 0x3fff0000,
}

// WQ_DOORBELL - WQ Doorbell Register
pub const SLI4_IO_WQ_DB_REG: c_uint = 0x040;
pub const SLI4_IF6_WQ_DB_REG: c_uint = 0x040;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_wq_e {
    SLI4_WQ_ID_MASK		= 0xffff,

    SLI4_WQ_IDX_SHIFT	= 16,
    SLI4_WQ_IDX_MASK	= 0xff0000,

    SLI4_WQ_NUM_SHIFT	= 24,
    SLI4_WQ_NUM_MASK	= 0x0ff00000,
}

// SLIPORT_STATUS - SLI Port Status Register
pub const SLI4_PORT_STATUS_REGOFF: c_uint = 0x0404;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_port_status {
    SLI4_PORT_STATUS_FDP	= 1u << 21,
    SLI4_PORT_STATUS_RDY	= 1u << 23,
    SLI4_PORT_STATUS_RN	= 1u << 24,
    SLI4_PORT_STATUS_DIP	= 1u << 25,
    SLI4_PORT_STATUS_OTI	= 1u << 29,
    SLI4_PORT_STATUS_ERR	= 1u << 31,
}

pub const SLI4_PHYDEV_CTRL_REG: c_uint = 0x0414;

// Register name enums
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_regname_en {
    SLI4_REG_BMBX,
    SLI4_REG_EQ_DOORBELL,
    SLI4_REG_CQ_DOORBELL,
    SLI4_REG_RQ_DOORBELL,
    SLI4_REG_IO_WQ_DOORBELL,
    SLI4_REG_MQ_DOORBELL,
    SLI4_REG_PHYSDEV_CONTROL,
    SLI4_REG_PORT_CONTROL,
    SLI4_REG_PORT_ERROR1,
    SLI4_REG_PORT_ERROR2,
    SLI4_REG_PORT_SEMAPHORE,
    SLI4_REG_PORT_STATUS,
    SLI4_REG_UNKWOWN			/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_reg {
    pub rset: u32,
    pub off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_dmaaddr {
    pub low: __le32,
    pub high: __le32,
}

//
// a 3-word Buffer Descriptor Entry with
// address 1st 2 words, length last word
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_bufptr {
    pub addr: sli4_dmaaddr,
    pub length: __le32,
}

// Buffer Descriptor Entry (BDE)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_bde_e {
    SLI4_BDE_LEN_MASK	= 0x00ffffff,
    SLI4_BDE_TYPE_MASK	= 0xff000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_bde {
    pub bde_type_buflen: __le32,
    pub data: sli4_dmaaddr,
    pub offset: __le32,
    pub rsvd2: __le32,
    pub imm: },
    pub blp: sli4_dmaaddr,
    pub u: },
}

// Buffer Descriptors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_bde_type {
    SLI4_BDE_TYPE_SHIFT	= 24,
    SLI4_BDE_TYPE_64	= 0x00,	/* Generic 64-bit data */
    SLI4_BDE_TYPE_IMM	= 0x01,	/* Immediate data */
    SLI4_BDE_TYPE_BLP	= 0x40,	/* Buffer List Pointer */
}

// Scatter-Gather Entry (SGE)
pub const SLI4_SGE_MAX_RESERVED: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_sge_type {
// DW2
    SLI4_SGE_DATA_OFFSET_MASK	= 0x07ffffff,
// DW2W1
    SLI4_SGE_TYPE_SHIFT		= 27,
    SLI4_SGE_TYPE_MASK		= 0x78000000,
// SGE Types
    SLI4_SGE_TYPE_DATA		= 0x00,
    SLI4_SGE_TYPE_DIF		= 0x04,	/* Data Integrity Field */
    SLI4_SGE_TYPE_LSP		= 0x05,	/* List Segment Pointer */
    SLI4_SGE_TYPE_PEDIF		= 0x06,	/* Post Encryption Engine DIF */
    SLI4_SGE_TYPE_PESEED		= 0x07,	/* Post Encryption DIF Seed */
    SLI4_SGE_TYPE_DISEED		= 0x08,	/* DIF Seed */
    SLI4_SGE_TYPE_ENC		= 0x09,	/* Encryption */
    SLI4_SGE_TYPE_ATM		= 0x0a,	/* DIF Application Tag Mask */
    SLI4_SGE_TYPE_SKIP		= 0x0c,	/* SKIP */

    SLI4_SGE_LAST			= 1u << 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_sge {
    pub buffer_address_high: __le32,
    pub buffer_address_low: __le32,
    pub dw2_flags: __le32,
    pub buffer_length: __le32,
}

// T10 DIF Scatter-Gather Entry (SGE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_dif_sge {
    pub buffer_address_high: __le32,
    pub buffer_address_low: __le32,
    pub dw2_flags: __le32,
    pub rsvd12: __le32,
}

// Data Integrity Seed (DISEED) SGE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_diseed_sge_flags {
// DW2W1
    SLI4_DISEED_SGE_HS		= 1 << 2,
    SLI4_DISEED_SGE_WS		= 1 << 3,
    SLI4_DISEED_SGE_IC		= 1 << 4,
    SLI4_DISEED_SGE_ICS		= 1 << 5,
    SLI4_DISEED_SGE_ATRT		= 1 << 6,
    SLI4_DISEED_SGE_AT		= 1 << 7,
    SLI4_DISEED_SGE_FAT		= 1 << 8,
    SLI4_DISEED_SGE_NA		= 1 << 9,
    SLI4_DISEED_SGE_HI		= 1 << 10,

// DW3W1
    SLI4_DISEED_SGE_BS_MASK		= 0x0007,
    SLI4_DISEED_SGE_AI		= 1 << 3,
    SLI4_DISEED_SGE_ME		= 1 << 4,
    SLI4_DISEED_SGE_RE		= 1 << 5,
    SLI4_DISEED_SGE_CE		= 1 << 6,
    SLI4_DISEED_SGE_NR		= 1 << 7,

    SLI4_DISEED_SGE_OP_RX_SHIFT	= 8,
    SLI4_DISEED_SGE_OP_RX_MASK	= 0x0f00,
    SLI4_DISEED_SGE_OP_TX_SHIFT	= 12,
    SLI4_DISEED_SGE_OP_TX_MASK	= 0xf000,
}

// Opcode values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_diseed_sge_opcodes {
    SLI4_DISEED_SGE_OP_IN_NODIF_OUT_CRC,
    SLI4_DISEED_SGE_OP_IN_CRC_OUT_NODIF,
    SLI4_DISEED_SGE_OP_IN_NODIF_OUT_CSUM,
    SLI4_DISEED_SGE_OP_IN_CSUM_OUT_NODIF,
    SLI4_DISEED_SGE_OP_IN_CRC_OUT_CRC,
    SLI4_DISEED_SGE_OP_IN_CSUM_OUT_CSUM,
    SLI4_DISEED_SGE_OP_IN_CRC_OUT_CSUM,
    SLI4_DISEED_SGE_OP_IN_CSUM_OUT_CRC,
    SLI4_DISEED_SGE_OP_IN_RAW_OUT_RAW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_diseed_sge {
    pub ref_tag_cmp: __le32,
    pub ref_tag_repl: __le32,
    pub app_tag_repl: __le16,
    pub dw2w1_flags: __le16,
    pub app_tag_cmp: __le16,
    pub dw3w1_flags: __le16,
}

// List Segment Pointer Scatter-Gather Entry (SGE)
pub const SLI4_LSP_SGE_SEGLEN: c_uint = 0x00ffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_lsp_sge {
    pub buffer_address_high: __le32,
    pub buffer_address_low: __le32,
    pub dw2_flags: __le32,
    pub dw3_seglen: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_eqe_e {
    SLI4_EQE_VALID	= 1,
    SLI4_EQE_MJCODE	= 0xe,
    SLI4_EQE_MNCODE	= 0xfff0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_eqe {
    pub dw0w0_flags: __le16,
    pub resource_id: __le16,
}

pub const SLI4_MAJOR_CODE_STANDARD: c_int = 0;
pub const SLI4_MAJOR_CODE_SENTINEL: c_int = 1;
// Sentinel EQE indicating the EQ is full
pub const SLI4_EQE_STATUS_EQ_FULL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_mcqe_e {
    SLI4_MCQE_CONSUMED	= 1u << 27,
    SLI4_MCQE_COMPLETED	= 1u << 28,
    SLI4_MCQE_AE		= 1u << 30,
    SLI4_MCQE_VALID		= 1u << 31,
}

// Entry was consumed but not completed

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_mcqe {
    pub completion_status: __le16,
    pub extended_status: __le16,
    pub mqe_tag_low: __le32,
    pub mqe_tag_high: __le32,
    pub dw3_flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_acqe_e {
    SLI4_ACQE_AE	= 1 << 6, /* async event - this is an ACQE */
    SLI4_ACQE_VAL	= 1 << 7, /* valid - contents of CQE are valid */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_acqe {
    pub event_data: [__le32; 3],
    pub rsvd12: u8,
    pub event_code: u8,
    pub event_type: u8,
    pub ae_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_acqe_event_code {
    SLI4_ACQE_EVENT_CODE_LINK_STATE		= 0x01,
    SLI4_ACQE_EVENT_CODE_FIP		= 0x02,
    SLI4_ACQE_EVENT_CODE_DCBX		= 0x03,
    SLI4_ACQE_EVENT_CODE_ISCSI		= 0x04,
    SLI4_ACQE_EVENT_CODE_GRP_5		= 0x05,
    SLI4_ACQE_EVENT_CODE_FC_LINK_EVENT	= 0x10,
    SLI4_ACQE_EVENT_CODE_SLI_PORT_EVENT	= 0x11,
    SLI4_ACQE_EVENT_CODE_VF_EVENT		= 0x12,
    SLI4_ACQE_EVENT_CODE_MR_EVENT		= 0x13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_qtype {
    SLI4_QTYPE_EQ,
    SLI4_QTYPE_CQ,
    SLI4_QTYPE_MQ,
    SLI4_QTYPE_WQ,
    SLI4_QTYPE_RQ,
    SLI4_QTYPE_MAX,			/* must be last */
}

pub const SLI4_USER_MQ_COUNT: c_int = 1;
pub const SLI4_MAX_CQ_SET_COUNT: c_int = 16;
pub const SLI4_MAX_RQ_SET_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_qentry {
    SLI4_QENTRY_ASYNC,
    SLI4_QENTRY_MQ,
    SLI4_QENTRY_RQ,
    SLI4_QENTRY_WQ,
    SLI4_QENTRY_WQ_RELEASE,
    SLI4_QENTRY_OPT_WRITE_CMD,
    SLI4_QENTRY_OPT_WRITE_DATA,
    SLI4_QENTRY_XABT,
    SLI4_QENTRY_MAX			/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_queue_flags {
    SLI4_QUEUE_FLAG_MQ	= 1 << 0,	/* CQ has MQ/Async completion */
    SLI4_QUEUE_FLAG_HDR	= 1 << 1,	/* RQ for packet headers */
    SLI4_QUEUE_FLAG_RQBATCH	= 1 << 2,	/* RQ index increment by 8 */
}

// Generic Command Request header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cmd_version {
    CMD_V0,
    CMD_V1,
    CMD_V2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_hdr {
    pub opcode: u8,
    pub subsystem: u8,
    pub rsvd2: __le16,
    pub timeout: __le32,
    pub request_length: __le32,
    pub dw3_version: __le32,
}

// Generic Command Response header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_hdr {
    pub opcode: u8,
    pub subsystem: u8,
    pub rsvd2: __le16,
    pub status: u8,
    pub additional_status: u8,
    pub rsvd6: __le16,
    pub response_length: __le32,
    pub actual_response_length: __le32,
}

pub const SLI4_QUEUE_RQ_BATCH: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_create_cqv2_e {
// DW5_flags values
    SLI4_CREATE_CQV2_CLSWM_MASK	= 0x00003000,
    SLI4_CREATE_CQV2_NODELAY	= 0x00004000,
    SLI4_CREATE_CQV2_AUTOVALID	= 0x00008000,
    SLI4_CREATE_CQV2_CQECNT_MASK	= 0x18000000,
    SLI4_CREATE_CQV2_VALID		= 0x20000000,
    SLI4_CREATE_CQV2_EVT		= 0x80000000,
// DW6W1_flags values
    SLI4_CREATE_CQV2_ARM		= 0x8000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_create_cq_v2 {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub page_size: u8,
    pub rsvd19: u8,
    pub dw5_flags: __le32,
    pub eq_id: __le16,
    pub dw6w1_arm: __le16,
    pub cqe_count: __le16,
    pub rsvd30: __le16,
    pub rsvd32: __le32,
    pub page_phys_addr: [sli4_dmaaddr; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_create_cqset_e {
// DW5_flags values
    SLI4_CREATE_CQSETV0_CLSWM_MASK	= 0x00003000,
    SLI4_CREATE_CQSETV0_NODELAY	= 0x00004000,
    SLI4_CREATE_CQSETV0_AUTOVALID	= 0x00008000,
    SLI4_CREATE_CQSETV0_CQECNT_MASK	= 0x18000000,
    SLI4_CREATE_CQSETV0_VALID	= 0x20000000,
    SLI4_CREATE_CQSETV0_EVT		= 0x80000000,
// DW5W1_flags values
    SLI4_CREATE_CQSETV0_CQE_COUNT	= 0x7fff,
    SLI4_CREATE_CQSETV0_ARM		= 0x8000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_create_cq_set_v0 {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub page_size: u8,
    pub rsvd19: u8,
    pub dw5_flags: __le32,
    pub num_cq_req: __le16,
    pub dw6w1_flags: __le16,
    pub eq_id: [__le16; 16],
    pub page_phys_addr: [sli4_dmaaddr; ],
}

// CQE count
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cq_cnt {
    SLI4_CQ_CNT_256,
    SLI4_CQ_CNT_512,
    SLI4_CQ_CNT_1024,
    SLI4_CQ_CNT_LARGE,
}

pub const SLI4_CQ_CNT_SHIFT: c_int = 27;

pub const SLI4_CREATE_CQV2_MAX_PAGES: c_int = 8;
// Generic Common Create EQ/CQ/MQ/WQ/RQ Queue completion
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_create_queue {
    pub hdr: sli4_rsp_hdr,
    pub q_id: __le16,
    pub rsvd18: u8,
    pub ulp: u8,
    pub db_offset: __le32,
    pub db_rs: __le16,
    pub db_fmt: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_create_queue_set {
    pub hdr: sli4_rsp_hdr,
    pub q_id: __le16,
    pub num_q_allocated: __le16,
}

// Common Destroy Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_destroy_q {
    pub hdr: sli4_rqst_hdr,
    pub q_id: __le16,
    pub rsvd: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_destroy_q {
    pub hdr: sli4_rsp_hdr,
}

// Modify the delay multiplier for EQs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_eqdelay_rec {
    pub eq_id: __le32,
    pub phase: __le32,
    pub delay_multiplier: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_modify_eq_delay {
    pub hdr: sli4_rqst_hdr,
    pub num_eq: __le32,
    pub eq_delay_record: [sli4_eqdelay_rec; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_modify_eq_delay {
    pub hdr: sli4_rsp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_create_cq_e {
// DW5
    SLI4_CREATE_EQ_AUTOVALID		= 1u << 28,
    SLI4_CREATE_EQ_VALID			= 1u << 29,
    SLI4_CREATE_EQ_EQESZ			= 1u << 31,
// DW6
    SLI4_CREATE_EQ_COUNT			= 7 << 26,
    SLI4_CREATE_EQ_ARM			= 1u << 31,
// DW7
    SLI4_CREATE_EQ_DELAYMULTI_SHIFT		= 13,
    SLI4_CREATE_EQ_DELAYMULTI_MASK		= 0x007fe000,
    SLI4_CREATE_EQ_DELAYMULTI		= 0x00040000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_create_eq {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub rsvd18: __le16,
    pub dw5_flags: __le32,
    pub dw6_flags: __le32,
    pub dw7_delaymulti: __le32,
    pub rsvd32: __le32,
    pub page_address: [sli4_dmaaddr; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_create_eq {
    pub q_rsp: sli4_rsp_cmn_create_queue,
}

// EQ count
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_eq_cnt {
    SLI4_EQ_CNT_256,
    SLI4_EQ_CNT_512,
    SLI4_EQ_CNT_1024,
    SLI4_EQ_CNT_2048,
    SLI4_EQ_CNT_4096 = 3,
}

pub const SLI4_EQ_CNT_SHIFT: c_int = 26;

pub const SLI4_EQE_SIZE_4: c_int = 0;
pub const SLI4_EQE_SIZE_16: c_int = 1;
// Create a Mailbox Queue; accommodate v0 and v1 forms.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_create_mq_flags {
// DW6W1
    SLI4_CREATE_MQEXT_RINGSIZE	= 0xf,
    SLI4_CREATE_MQEXT_CQID_SHIFT	= 6,
    SLI4_CREATE_MQEXT_CQIDV0_MASK	= 0xffc0,
// DW7
    SLI4_CREATE_MQEXT_VAL		= 1u << 31,
// DW8
    SLI4_CREATE_MQEXT_ACQV		= 1u << 0,
    SLI4_CREATE_MQEXT_ASYNC_CQIDV0	= 0x7fe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_create_mq_ext {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub cq_id_v1: __le16,
    pub async_event_bitmap: __le32,
    pub async_cq_id_v1: __le16,
    pub dw6w1_flags: __le16,
    pub dw7_val: __le32,
    pub dw8_flags: __le32,
    pub rsvd36: __le32,
    pub page_phys_addr: [sli4_dmaaddr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_create_mq_ext {
    pub q_rsp: sli4_rsp_cmn_create_queue,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_mqe_size {
    SLI4_MQE_SIZE_16 = 0x05,
    SLI4_MQE_SIZE_32,
    SLI4_MQE_SIZE_64,
    SLI4_MQE_SIZE_128,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_async_evt {
    SLI4_ASYNC_EVT_LINK_STATE	= 1 << 1,
    SLI4_ASYNC_EVT_FIP		= 1 << 2,
    SLI4_ASYNC_EVT_GRP5		= 1 << 5,
    SLI4_ASYNC_EVT_FC		= 1 << 16,
    SLI4_ASYNC_EVT_SLI_PORT		= 1 << 17,
}

// Create a Completion Queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_create_cq_v0 {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub rsvd18: __le16,
    pub dw5_flags: __le32,
    pub dw6_flags: __le32,
    pub rsvd28: __le32,
    pub rsvd32: __le32,
    pub page_phys_addr: [sli4_dmaaddr; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_create_rq_e {
    SLI4_RQ_CREATE_DUA		= 0x1,
    SLI4_RQ_CREATE_BQU		= 0x2,

    SLI4_RQE_SIZE			= 8,
    SLI4_RQE_SIZE_8			= 0x2,
    SLI4_RQE_SIZE_16		= 0x3,
    SLI4_RQE_SIZE_32		= 0x4,
    SLI4_RQE_SIZE_64		= 0x5,
    SLI4_RQE_SIZE_128		= 0x6,

    SLI4_RQ_PAGE_SIZE_4096		= 0x1,
    SLI4_RQ_PAGE_SIZE_8192		= 0x2,
    SLI4_RQ_PAGE_SIZE_16384		= 0x4,
    SLI4_RQ_PAGE_SIZE_32768		= 0x8,
    SLI4_RQ_PAGE_SIZE_64536		= 0x10,

    SLI4_RQ_CREATE_V0_MAX_PAGES	= 8,
    SLI4_RQ_CREATE_V0_MIN_BUF_SIZE	= 128,
    SLI4_RQ_CREATE_V0_MAX_BUF_SIZE	= 2048,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_rq_create {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub dua_bqu_byte: u8,
    pub ulp: u8,
    pub rsvd16: __le16,
    pub rqe_count_byte: u8,
    pub rsvd19: u8,
    pub rsvd20: __le32,
    pub buffer_size: __le16,
    pub cq_id: __le16,
    pub rsvd28: __le32,
    pub page_phys_addr: [sli4_dmaaddr; SLI4_RQ_CREATE_V0_MAX_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_rq_create {
    pub rsp: sli4_rsp_cmn_create_queue,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_create_rqv1_e {
    SLI4_RQ_CREATE_V1_DNB		= 0x80,
    SLI4_RQ_CREATE_V1_MAX_PAGES	= 8,
    SLI4_RQ_CREATE_V1_MIN_BUF_SIZE	= 64,
    SLI4_RQ_CREATE_V1_MAX_BUF_SIZE	= 2048,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_rq_create_v1 {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub rsvd14: u8,
    pub dim_dfd_dnb: u8,
    pub page_size: u8,
    pub rqe_size_byte: u8,
    pub rqe_count: __le16,
    pub rsvd20: __le32,
    pub rsvd24: __le16,
    pub cq_id: __le16,
    pub buffer_size: __le32,
    pub page_phys_addr: [sli4_dmaaddr; SLI4_RQ_CREATE_V1_MAX_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_rq_create_v1 {
    pub rsp: sli4_rsp_cmn_create_queue,
}

pub const SLI4_RQCREATEV2_DNB: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_rq_create_v2 {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub rq_count: u8,
    pub dim_dfd_dnb: u8,
    pub page_size: u8,
    pub rqe_size_byte: u8,
    pub rqe_count: __le16,
    pub hdr_buffer_size: __le16,
    pub payload_buffer_size: __le16,
    pub base_cq_id: __le16,
    pub rsvd26: __le16,
    pub rsvd42: __le32,
    pub page_phys_addr: [sli4_dmaaddr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_rq_create_v2 {
    pub rsp: sli4_rsp_cmn_create_queue,
}

pub const SLI4_CQE_CODE_OFFSET: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cqe_code {
    SLI4_CQE_CODE_WORK_REQUEST_COMPLETION = 0x01,
    SLI4_CQE_CODE_RELEASE_WQE,
    SLI4_CQE_CODE_RSVD,
    SLI4_CQE_CODE_RQ_ASYNC,
    SLI4_CQE_CODE_XRI_ABORTED,
    SLI4_CQE_CODE_RQ_COALESCING,
    SLI4_CQE_CODE_RQ_CONSUMPTION,
    SLI4_CQE_CODE_MEASUREMENT_REPORTING,
    SLI4_CQE_CODE_RQ_ASYNC_V1,
    SLI4_CQE_CODE_RQ_COALESCING_V1,
    SLI4_CQE_CODE_OPTIMIZED_WRITE_CMD,
    SLI4_CQE_CODE_OPTIMIZED_WRITE_DATA,
}

pub const SLI4_WQ_CREATE_MAX_PAGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_wq_create {
    pub hdr: sli4_rqst_hdr,
    pub num_pages: __le16,
    pub cq_id: __le16,
    pub page_size: u8,
    pub wqe_size_byte: u8,
    pub wqe_count: __le16,
    pub rsvd: __le32,
    pub page_phys_addr: [sli4_dmaaddr; SLI4_WQ_CREATE_MAX_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_wq_create {
    pub rsp: sli4_rsp_cmn_create_queue,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_link_attention_flags {
    SLI4_LNK_ATTN_TYPE_LINK_UP		= 0x01,
    SLI4_LNK_ATTN_TYPE_LINK_DOWN		= 0x02,
    SLI4_LNK_ATTN_TYPE_NO_HARD_ALPA		= 0x03,

    SLI4_LNK_ATTN_P2P			= 0x01,
    SLI4_LNK_ATTN_FC_AL			= 0x02,
    SLI4_LNK_ATTN_INTERNAL_LOOPBACK		= 0x03,
    SLI4_LNK_ATTN_SERDES_LOOPBACK		= 0x04,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_link_attention {
    pub link_number: u8,
    pub attn_type: u8,
    pub topology: u8,
    pub port_speed: u8,
    pub port_fault: u8,
    pub shared_link_status: u8,
    pub logical_link_speed: __le16,
    pub event_tag: __le32,
    pub rsvd12: u8,
    pub event_code: u8,
    pub event_type: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_link_event_type {
    SLI4_EVENT_LINK_ATTENTION		= 0x01,
    SLI4_EVENT_SHARED_LINK_ATTENTION	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_wcqe_flags {
    SLI4_WCQE_XB = 0x10,
    SLI4_WCQE_QX = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_wcqe {
    pub hw_status: u8,
    pub status: u8,
    pub request_tag: __le16,
    pub wqe_specific_1: __le32,
    pub wqe_specific_2: __le32,
    pub rsvd12: u8,
    pub qx_byte: u8,
    pub code: u8,
    pub flags: u8,
}

// FC WQ consumed CQ queue entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_wqec {
    pub rsvd0: __le32,
    pub rsvd1: __le32,
    pub wqe_index: __le16,
    pub wq_id: __le16,
    pub rsvd12: __le16,
    pub code: u8,
    pub vld_byte: u8,
}

// FC Completion Status Codes.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_wcqe_status {
    SLI4_FC_WCQE_STATUS_SUCCESS,
    SLI4_FC_WCQE_STATUS_FCP_RSP_FAILURE,
    SLI4_FC_WCQE_STATUS_REMOTE_STOP,
    SLI4_FC_WCQE_STATUS_LOCAL_REJECT,
    SLI4_FC_WCQE_STATUS_NPORT_RJT,
    SLI4_FC_WCQE_STATUS_FABRIC_RJT,
    SLI4_FC_WCQE_STATUS_NPORT_BSY,
    SLI4_FC_WCQE_STATUS_FABRIC_BSY,
    SLI4_FC_WCQE_STATUS_RSVD,
    SLI4_FC_WCQE_STATUS_LS_RJT,
    SLI4_FC_WCQE_STATUS_RX_BUF_OVERRUN,
    SLI4_FC_WCQE_STATUS_CMD_REJECT,
    SLI4_FC_WCQE_STATUS_FCP_TGT_LENCHECK,
    SLI4_FC_WCQE_STATUS_RSVD1,
    SLI4_FC_WCQE_STATUS_ELS_CMPLT_NO_AUTOREG,
    SLI4_FC_WCQE_STATUS_RSVD2,
    SLI4_FC_WCQE_STATUS_RQ_SUCCESS,
    SLI4_FC_WCQE_STATUS_RQ_BUF_LEN_EXCEEDED,
    SLI4_FC_WCQE_STATUS_RQ_INSUFF_BUF_NEEDED,
    SLI4_FC_WCQE_STATUS_RQ_INSUFF_FRM_DISC,
    SLI4_FC_WCQE_STATUS_RQ_DMA_FAILURE,
    SLI4_FC_WCQE_STATUS_FCP_RSP_TRUNCATE,
    SLI4_FC_WCQE_STATUS_DI_ERROR,
    SLI4_FC_WCQE_STATUS_BA_RJT,
    SLI4_FC_WCQE_STATUS_RQ_INSUFF_XRI_NEEDED,
    SLI4_FC_WCQE_STATUS_RQ_INSUFF_XRI_DISC,
    SLI4_FC_WCQE_STATUS_RX_ERROR_DETECT,
    SLI4_FC_WCQE_STATUS_RX_ABORT_REQUEST,

// driver generated status codes
    SLI4_FC_WCQE_STATUS_DISPATCH_ERROR	= 0xfd,
    SLI4_FC_WCQE_STATUS_SHUTDOWN		= 0xfe,
    SLI4_FC_WCQE_STATUS_TARGET_WQE_TIMEOUT	= 0xff,
}

// DI_ERROR Extended Status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_fc_di_error_status {
    SLI4_FC_DI_ERROR_GE			= 1 << 0,
    SLI4_FC_DI_ERROR_AE			= 1 << 1,
    SLI4_FC_DI_ERROR_RE			= 1 << 2,
    SLI4_FC_DI_ERROR_TDPV			= 1 << 3,
    SLI4_FC_DI_ERROR_UDB			= 1 << 4,
    SLI4_FC_DI_ERROR_EDIR			= 1 << 5,
}

// WQE DIF field contents
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_dif_fields {
    SLI4_DIF_DISABLED,
    SLI4_DIF_PASS_THROUGH,
    SLI4_DIF_STRIP,
    SLI4_DIF_INSERT,
}

// Work Queue Entry (WQE) types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_wqe_types {
    SLI4_WQE_ABORT				= 0x0f,
    SLI4_WQE_ELS_REQUEST64			= 0x8a,
    SLI4_WQE_FCP_IBIDIR64			= 0xac,
    SLI4_WQE_FCP_IREAD64			= 0x9a,
    SLI4_WQE_FCP_IWRITE64			= 0x98,
    SLI4_WQE_FCP_ICMND64			= 0x9c,
    SLI4_WQE_FCP_TRECEIVE64			= 0xa1,
    SLI4_WQE_FCP_CONT_TRECEIVE64		= 0xe5,
    SLI4_WQE_FCP_TRSP64			= 0xa3,
    SLI4_WQE_FCP_TSEND64			= 0x9f,
    SLI4_WQE_GEN_REQUEST64			= 0xc2,
    SLI4_WQE_SEND_FRAME			= 0xe1,
    SLI4_WQE_XMIT_BCAST64			= 0x84,
    SLI4_WQE_XMIT_BLS_RSP			= 0x97,
    SLI4_WQE_ELS_RSP64			= 0x95,
    SLI4_WQE_XMIT_SEQUENCE64		= 0x82,
    SLI4_WQE_REQUEUE_XRI			= 0x93,
}

// WQE command types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_wqe_cmds {
    SLI4_CMD_FCP_IREAD64_WQE		= 0x00,
    SLI4_CMD_FCP_ICMND64_WQE		= 0x00,
    SLI4_CMD_FCP_IWRITE64_WQE		= 0x01,
    SLI4_CMD_FCP_TRECEIVE64_WQE		= 0x02,
    SLI4_CMD_FCP_TRSP64_WQE			= 0x03,
    SLI4_CMD_FCP_TSEND64_WQE		= 0x07,
    SLI4_CMD_GEN_REQUEST64_WQE		= 0x08,
    SLI4_CMD_XMIT_BCAST64_WQE		= 0x08,
    SLI4_CMD_XMIT_BLS_RSP64_WQE		= 0x08,
    SLI4_CMD_ABORT_WQE			= 0x08,
    SLI4_CMD_XMIT_SEQUENCE64_WQE		= 0x08,
    SLI4_CMD_REQUEUE_XRI_WQE		= 0x0a,
    SLI4_CMD_SEND_FRAME_WQE			= 0x0a,
}

pub const SLI4_WQE_SIZE: c_uint = 0x05;
pub const SLI4_WQE_EXT_SIZE: c_uint = 0x06;

// Mask for ccp (CS_CTL)
pub const SLI4_MASK_CCP: c_uint = 0xfe;
// Generic WQE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_gen_wqe_flags {
    SLI4_GEN_WQE_EBDECNT	= 0xf,
    SLI4_GEN_WQE_LEN_LOC	= 0x3 << 7,
    SLI4_GEN_WQE_QOSD	= 1 << 9,
    SLI4_GEN_WQE_XBL	= 1 << 11,
    SLI4_GEN_WQE_HLM	= 1 << 12,
    SLI4_GEN_WQE_IOD	= 1 << 13,
    SLI4_GEN_WQE_DBDE	= 1 << 14,
    SLI4_GEN_WQE_WQES	= 1 << 15,

    SLI4_GEN_WQE_PRI	= 0x7,
    SLI4_GEN_WQE_PV		= 1 << 3,
    SLI4_GEN_WQE_EAT	= 1 << 4,
    SLI4_GEN_WQE_XC		= 1 << 5,
    SLI4_GEN_WQE_CCPE	= 1 << 7,

    SLI4_GEN_WQE_CMDTYPE	= 0xf,
    SLI4_GEN_WQE_WQEC	= 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_generic_wqe {
    pub cmd_spec0_5: [__le32; 6],
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub dw10w0_flags: __le16,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmdtype_wqec_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
}

// WQE used to abort exchanges.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_abort_wqe_flags {
    SLI4_ABRT_WQE_IR	= 0x02,

    SLI4_ABRT_WQE_EBDECNT	= 0xf,
    SLI4_ABRT_WQE_LEN_LOC	= 0x3 << 7,
    SLI4_ABRT_WQE_QOSD	= 1 << 9,
    SLI4_ABRT_WQE_XBL	= 1 << 11,
    SLI4_ABRT_WQE_IOD	= 1 << 13,
    SLI4_ABRT_WQE_DBDE	= 1 << 14,
    SLI4_ABRT_WQE_WQES	= 1 << 15,

    SLI4_ABRT_WQE_PRI	= 0x7,
    SLI4_ABRT_WQE_PV	= 1 << 3,
    SLI4_ABRT_WQE_EAT	= 1 << 4,
    SLI4_ABRT_WQE_XC	= 1 << 5,
    SLI4_ABRT_WQE_CCPE	= 1 << 7,

    SLI4_ABRT_WQE_CMDTYPE	= 0xf,
    SLI4_ABRT_WQE_WQEC	= 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_abort_wqe {
    pub rsvd0: __le32,
    pub rsvd4: __le32,
    pub ext_t_tag: __le32,
    pub ia_ir_byte: u8,
    pub criteria: u8,
    pub rsvd10: __le16,
    pub ext_t_mask: __le32,
    pub t_mask: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_byte: u8,
    pub timer: u8,
    pub t_tag: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub dw10w0_flags: __le16,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmdtype_wqec_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_abort_criteria {
    SLI4_ABORT_CRITERIA_XRI_TAG = 0x01,
    SLI4_ABORT_CRITERIA_ABORT_TAG,
    SLI4_ABORT_CRITERIA_REQUEST_TAG,
    SLI4_ABORT_CRITERIA_EXT_ABORT_TAG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_abort_type {
    SLI4_ABORT_XRI,
    SLI4_ABORT_ABORT_ID,
    SLI4_ABORT_REQUEST_ID,
    SLI4_ABORT_MAX,		/* must be last */
}

// WQE used to create an ELS request.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_els_req_wqe_flags {
    SLI4_REQ_WQE_QOSD		= 0x2,
    SLI4_REQ_WQE_DBDE		= 0x40,
    SLI4_REQ_WQE_XBL		= 0x8,
    SLI4_REQ_WQE_XC			= 0x20,
    SLI4_REQ_WQE_IOD		= 0x20,
    SLI4_REQ_WQE_HLM		= 0x10,
    SLI4_REQ_WQE_CCPE		= 0x80,
    SLI4_REQ_WQE_EAT		= 0x10,
    SLI4_REQ_WQE_WQES		= 0x80,
    SLI4_REQ_WQE_PU_SHFT		= 4,
    SLI4_REQ_WQE_CT_SHFT		= 2,
    SLI4_REQ_WQE_CT			= 0xc,
    SLI4_REQ_WQE_ELSID_SHFT		= 4,
    SLI4_REQ_WQE_SP_SHFT		= 24,
    SLI4_REQ_WQE_LEN_LOC_BIT1	= 0x80,
    SLI4_REQ_WQE_LEN_LOC_BIT2	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_els_request64_wqe {
    pub els_request_payload: sli4_bde,
    pub els_request_payload_length: __le32,
    pub sid_sp_dword: __le32,
    pub remote_id_dword: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub temporary_rpi: __le16,
    pub len_loc1_byte: u8,
    pub qosd_xbl_hlm_iod_dbde_wqes: u8,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmdtype_elsid_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub els_response_payload_bde: sli4_bde,
    pub max_response_payload_length: __le32,
}

// WQE used to create an FCP initiator no data command.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_icmd_wqe_flags {
    SLI4_ICMD_WQE_DBDE		= 0x40,
    SLI4_ICMD_WQE_XBL		= 0x8,
    SLI4_ICMD_WQE_XC		= 0x20,
    SLI4_ICMD_WQE_IOD		= 0x20,
    SLI4_ICMD_WQE_HLM		= 0x10,
    SLI4_ICMD_WQE_CCPE		= 0x80,
    SLI4_ICMD_WQE_EAT		= 0x10,
    SLI4_ICMD_WQE_APPID		= 0x10,
    SLI4_ICMD_WQE_WQES		= 0x80,
    SLI4_ICMD_WQE_PU_SHFT		= 4,
    SLI4_ICMD_WQE_CT_SHFT		= 2,
    SLI4_ICMD_WQE_BS_SHFT		= 4,
    SLI4_ICMD_WQE_LEN_LOC_BIT1	= 0x80,
    SLI4_ICMD_WQE_LEN_LOC_BIT2	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_icmnd64_wqe {
    pub bde: sli4_bde,
    pub payload_offset_length: __le16,
    pub fcp_cmd_buffer_length: __le16,
    pub rsvd12: __le32,
    pub remote_n_port_id_dword: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub dif_ct_bs_byte: u8,
    pub command: u8,
    pub class_pu_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub len_loc1_byte: u8,
    pub qosd_xbl_hlm_iod_dbde_wqes: u8,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub rsvd44: __le32,
    pub rsvd48: __le32,
    pub rsvd52: __le32,
    pub rsvd56: __le32,
}

// WQE used to create an FCP initiator read.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_ir_wqe_flags {
    SLI4_IR_WQE_DBDE		= 0x40,
    SLI4_IR_WQE_XBL			= 0x8,
    SLI4_IR_WQE_XC			= 0x20,
    SLI4_IR_WQE_IOD			= 0x20,
    SLI4_IR_WQE_HLM			= 0x10,
    SLI4_IR_WQE_CCPE		= 0x80,
    SLI4_IR_WQE_EAT			= 0x10,
    SLI4_IR_WQE_APPID		= 0x10,
    SLI4_IR_WQE_WQES		= 0x80,
    SLI4_IR_WQE_PU_SHFT		= 4,
    SLI4_IR_WQE_CT_SHFT		= 2,
    SLI4_IR_WQE_BS_SHFT		= 4,
    SLI4_IR_WQE_LEN_LOC_BIT1	= 0x80,
    SLI4_IR_WQE_LEN_LOC_BIT2	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_iread64_wqe {
    pub bde: sli4_bde,
    pub payload_offset_length: __le16,
    pub fcp_cmd_buffer_length: __le16,
    pub total_transfer_length: __le32,
    pub remote_n_port_id_dword: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub dif_ct_bs_byte: u8,
    pub command: u8,
    pub class_pu_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub len_loc1_byte: u8,
    pub qosd_xbl_hlm_iod_dbde_wqes: u8,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub rsvd44: __le32,
    pub first_data_bde: sli4_bde,
}

// WQE used to create an FCP initiator write.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_iwr_wqe_flags {
    SLI4_IWR_WQE_DBDE		= 0x40,
    SLI4_IWR_WQE_XBL		= 0x8,
    SLI4_IWR_WQE_XC			= 0x20,
    SLI4_IWR_WQE_IOD		= 0x20,
    SLI4_IWR_WQE_HLM		= 0x10,
    SLI4_IWR_WQE_DNRX		= 0x10,
    SLI4_IWR_WQE_CCPE		= 0x80,
    SLI4_IWR_WQE_EAT		= 0x10,
    SLI4_IWR_WQE_APPID		= 0x10,
    SLI4_IWR_WQE_WQES		= 0x80,
    SLI4_IWR_WQE_PU_SHFT		= 4,
    SLI4_IWR_WQE_CT_SHFT		= 2,
    SLI4_IWR_WQE_BS_SHFT		= 4,
    SLI4_IWR_WQE_LEN_LOC_BIT1	= 0x80,
    SLI4_IWR_WQE_LEN_LOC_BIT2	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_iwrite64_wqe {
    pub bde: sli4_bde,
    pub payload_offset_length: __le16,
    pub fcp_cmd_buffer_length: __le16,
    pub total_transfer_length: __le16,
    pub initial_transfer_length: __le16,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub dif_ct_bs_byte: u8,
    pub command: u8,
    pub class_pu_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub len_loc1_byte: u8,
    pub qosd_xbl_hlm_iod_dbde_wqes: u8,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub remote_n_port_id_dword: __le32,
    pub first_data_bde: sli4_bde,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_128byte_wqe {
    pub dw: [u32; 32],
}

// WQE used to create an FCP target receive
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_trcv_wqe_flags {
    SLI4_TRCV_WQE_DBDE		= 0x40,
    SLI4_TRCV_WQE_XBL		= 0x8,
    SLI4_TRCV_WQE_AR		= 0x8,
    SLI4_TRCV_WQE_XC		= 0x20,
    SLI4_TRCV_WQE_IOD		= 0x20,
    SLI4_TRCV_WQE_HLM		= 0x10,
    SLI4_TRCV_WQE_DNRX		= 0x10,
    SLI4_TRCV_WQE_CCPE		= 0x80,
    SLI4_TRCV_WQE_EAT		= 0x10,
    SLI4_TRCV_WQE_APPID		= 0x10,
    SLI4_TRCV_WQE_WQES		= 0x80,
    SLI4_TRCV_WQE_PU_SHFT		= 4,
    SLI4_TRCV_WQE_CT_SHFT		= 2,
    SLI4_TRCV_WQE_BS_SHFT		= 4,
    SLI4_TRCV_WQE_LEN_LOC_BIT2	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_treceive64_wqe {
    pub bde: sli4_bde,
    pub payload_offset_length: __le32,
    pub relative_offset: __le32,
    pub sec_xri_tag: __le16,
    pub rsvd: __le16,
    pub dword: __le32,
    pub dword5: },
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub dif_ct_bs_byte: u8,
    pub command: u8,
    pub class_ar_pu_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub remote_xid: __le16,
    pub lloc1_appid: u8,
    pub qosd_xbl_hlm_iod_dbde_wqes: u8,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub fcp_data_receive_length: __le32,
    pub first_data_bde: sli4_bde,
}

// WQE used to create an FCP target response
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_trsp_wqe_flags {
    SLI4_TRSP_WQE_AG	= 0x8,
    SLI4_TRSP_WQE_DBDE	= 0x40,
    SLI4_TRSP_WQE_XBL	= 0x8,
    SLI4_TRSP_WQE_XC	= 0x20,
    SLI4_TRSP_WQE_HLM	= 0x10,
    SLI4_TRSP_WQE_DNRX	= 0x10,
    SLI4_TRSP_WQE_CCPE	= 0x80,
    SLI4_TRSP_WQE_EAT	= 0x10,
    SLI4_TRSP_WQE_APPID	= 0x10,
    SLI4_TRSP_WQE_WQES	= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_trsp64_wqe {
    pub bde: sli4_bde,
    pub fcp_response_length: __le32,
    pub rsvd12: __le32,
    pub dword5: __le32,
    pub xri_tag: __le16,
    pub rpi: __le16,
    pub ct_dnrx_byte: u8,
    pub command: u8,
    pub class_ag_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub remote_xid: __le16,
    pub lloc1_appid: u8,
    pub qosd_xbl_hlm_dbde_wqes: u8,
    pub eat_xc_ccpe: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub rsvd44: __le32,
    pub rsvd48: __le32,
    pub rsvd52: __le32,
    pub rsvd56: __le32,
}

// WQE used to create an FCP target send (DATA IN).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_tsend_wqe_flags {
    SLI4_TSEND_WQE_XBL	= 0x8,
    SLI4_TSEND_WQE_DBDE	= 0x40,
    SLI4_TSEND_WQE_IOD	= 0x20,
    SLI4_TSEND_WQE_QOSD	= 0x2,
    SLI4_TSEND_WQE_HLM	= 0x10,
    SLI4_TSEND_WQE_PU_SHFT	= 4,
    SLI4_TSEND_WQE_AR	= 0x8,
    SLI4_TSEND_CT_SHFT	= 2,
    SLI4_TSEND_BS_SHFT	= 4,
    SLI4_TSEND_LEN_LOC_BIT2 = 0x1,
    SLI4_TSEND_CCPE		= 0x80,
    SLI4_TSEND_APPID_VALID	= 0x20,
    SLI4_TSEND_WQES		= 0x80,
    SLI4_TSEND_XC		= 0x20,
    SLI4_TSEND_EAT		= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fcp_tsend64_wqe {
    pub bde: sli4_bde,
    pub payload_offset_length: __le32,
    pub relative_offset: __le32,
    pub dword5: __le32,
    pub xri_tag: __le16,
    pub rpi: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_pu_ar_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub remote_xid: __le16,
    pub dw10byte0: u8,
    pub ll_qd_xbl_hlm_iod_dbde: u8,
    pub dw10byte2: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd45: u8,
    pub cq_id: __le16,
    pub fcp_data_transmit_length: __le32,
    pub first_data_bde: sli4_bde,
}

// WQE used to create a general request.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_gen_req_wqe_flags {
    SLI4_GEN_REQ64_WQE_XBL	= 0x8,
    SLI4_GEN_REQ64_WQE_DBDE	= 0x40,
    SLI4_GEN_REQ64_WQE_IOD	= 0x20,
    SLI4_GEN_REQ64_WQE_QOSD	= 0x2,
    SLI4_GEN_REQ64_WQE_HLM	= 0x10,
    SLI4_GEN_REQ64_CT_SHFT	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_gen_request64_wqe {
    pub bde: sli4_bde,
    pub request_payload_length: __le32,
    pub relative_offset: __le32,
    pub rsvd17: u8,
    pub df_ctl: u8,
    pub type: u8,
    pub r_ctl: u8,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub dw10flags0: u8,
    pub dw10flags1: u8,
    pub dw10flags2: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub remote_n_port_id_dword: __le32,
    pub rsvd48: __le32,
    pub rsvd52: __le32,
    pub max_response_payload_length: __le32,
}

// WQE used to create a send frame request
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_sf_wqe_flags {
    SLI4_SF_WQE_DBDE	= 0x40,
    SLI4_SF_PU		= 0x30,
    SLI4_SF_CT		= 0xc,
    SLI4_SF_QOSD		= 0x2,
    SLI4_SF_LEN_LOC_BIT1	= 0x80,
    SLI4_SF_LEN_LOC_BIT2	= 0x1,
    SLI4_SF_XC		= 0x20,
    SLI4_SF_XBL		= 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_send_frame_wqe {
    pub bde: sli4_bde,
    pub frame_length: __le32,
    pub fc_header_0_1: [__le32; 2],
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub dw7flags0: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub eof: u8,
    pub sof: u8,
    pub dw10flags0: u8,
    pub dw10flags1: u8,
    pub dw10flags2: u8,
    pub ccp: u8,
    pub cmd_type_byte: u8,
    pub rsvd41: u8,
    pub cq_id: __le16,
    pub fc_header_2_5: [__le32; 4],
}

// WQE used to create a transmit sequence
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_seq_wqe_flags {
    SLI4_SEQ_WQE_DBDE		= 0x4000,
    SLI4_SEQ_WQE_XBL		= 0x800,
    SLI4_SEQ_WQE_SI			= 0x4,
    SLI4_SEQ_WQE_FT			= 0x8,
    SLI4_SEQ_WQE_XO			= 0x40,
    SLI4_SEQ_WQE_LS			= 0x80,
    SLI4_SEQ_WQE_DIF		= 0x3,
    SLI4_SEQ_WQE_BS			= 0x70,
    SLI4_SEQ_WQE_PU			= 0x30,
    SLI4_SEQ_WQE_HLM		= 0x1000,
    SLI4_SEQ_WQE_IOD_SHIFT		= 13,
    SLI4_SEQ_WQE_CT_SHIFT		= 2,
    SLI4_SEQ_WQE_LEN_LOC_SHIFT	= 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_xmit_sequence64_wqe {
    pub bde: sli4_bde,
    pub remote_n_port_id_dword: __le32,
    pub relative_offset: __le32,
    pub dw5flags0: u8,
    pub df_ctl: u8,
    pub type: u8,
    pub r_ctl: u8,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub dw7flags0: u8,
    pub command: u8,
    pub dw7flags1: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub remote_xid: __le16,
    pub dw10w0: __le16,
    pub dw10flags0: u8,
    pub ccp: u8,
    pub cmd_type_wqec_byte: u8,
    pub rsvd45: u8,
    pub cq_id: __le16,
    pub sequence_payload_len: __le32,
    pub rsvd48: __le32,
    pub rsvd52: __le32,
    pub rsvd56: __le32,
}

//
// WQE used unblock the specified XRI and to release
// it to the SLI Port's free pool.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_requeue_wqe_flags {
    SLI4_REQU_XRI_WQE_XC	= 0x20,
    SLI4_REQU_XRI_WQE_QOSD	= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_requeue_xri_wqe {
    pub rsvd0: __le32,
    pub rsvd4: __le32,
    pub rsvd8: __le32,
    pub rsvd12: __le32,
    pub rsvd16: __le32,
    pub rsvd20: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_byte: u8,
    pub timer: u8,
    pub rsvd32: __le32,
    pub request_tag: __le16,
    pub rsvd34: __le16,
    pub flags0: __le16,
    pub flags1: __le16,
    pub flags2: __le16,
    pub ccp: u8,
    pub cmd_type_wqec_byte: u8,
    pub rsvd42: u8,
    pub cq_id: __le16,
    pub rsvd44: __le32,
    pub rsvd48: __le32,
    pub rsvd52: __le32,
    pub rsvd56: __le32,
}

// WQE used to create a BLS response
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_bls_rsp_wqe_flags {
    SLI4_BLS_RSP_RID		= 0xffffff,
    SLI4_BLS_RSP_WQE_AR		= 0x40000000,
    SLI4_BLS_RSP_WQE_CT_SHFT	= 2,
    SLI4_BLS_RSP_WQE_QOSD		= 0x2,
    SLI4_BLS_RSP_WQE_HLM		= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_xmit_bls_rsp_wqe {
    pub payload_word0: __le32,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub high_seq_cnt: __le16,
    pub low_seq_cnt: __le16,
    pub rsvd12: __le32,
    pub local_n_port_id_dword: __le32,
    pub remote_id_dword: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub dw8flags0: u8,
    pub command: u8,
    pub dw8flags1: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub rsvd38: __le16,
    pub dw11flags0: u8,
    pub dw11flags1: u8,
    pub dw11flags2: u8,
    pub ccp: u8,
    pub dw12flags0: u8,
    pub rsvd45: u8,
    pub cq_id: __le16,
    pub temporary_rpi: __le16,
    pub rsvd50: u8,
    pub rsvd51: u8,
    pub rsvd52: __le32,
    pub rsvd56: __le32,
    pub rsvd60: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli_bls_type {
    SLI4_SLI_BLS_ACC,
    SLI4_SLI_BLS_RJT,
    SLI4_SLI_BLS_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli_bls_payload {
    pub type: sli_bls_type,
    pub ox_id: __le16,
    pub rx_id: __le16,
    pub seq_id_validity: u8,
    pub seq_id_last: u8,
    pub rsvd2: u8,
    pub rsvd3: u8,
    pub ox_id: u16,
    pub rx_id: u16,
    pub low_seq_cnt: __le16,
    pub high_seq_cnt: __le16,
    pub acc: },
    pub vendor_unique: u8,
    pub reason_explanation: u8,
    pub reason_code: u8,
    pub rsvd3: u8,
    pub rjt: },
    pub u: },
}

// WQE used to create an ELS response
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_els_rsp_flags {
    SLI4_ELS_SID		= 0xffffff,
    SLI4_ELS_RID		= 0xffffff,
    SLI4_ELS_DBDE		= 0x40,
    SLI4_ELS_XBL		= 0x8,
    SLI4_ELS_IOD		= 0x20,
    SLI4_ELS_QOSD		= 0x2,
    SLI4_ELS_XC		= 0x20,
    SLI4_ELS_CT_OFFSET	= 0X2,
    SLI4_ELS_SP		= 0X1000000,
    SLI4_ELS_HLM		= 0X10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_xmit_els_rsp64_wqe {
    pub els_response_payload: sli4_bde,
    pub els_response_payload_length: __le32,
    pub sid_dw: __le32,
    pub rid_dw: __le32,
    pub xri_tag: __le16,
    pub context_tag: __le16,
    pub ct_byte: u8,
    pub command: u8,
    pub class_byte: u8,
    pub timer: u8,
    pub abort_tag: __le32,
    pub request_tag: __le16,
    pub ox_id: __le16,
    pub flags1: u8,
    pub flags2: u8,
    pub flags3: u8,
    pub flags4: u8,
    pub cmd_type_wqec: u8,
    pub rsvd34: u8,
    pub cq_id: __le16,
    pub temporary_rpi: __le16,
    pub rsvd38: __le16,
    pub rsvd40: u32,
    pub rsvd44: u32,
    pub rsvd48: u32,
}

// Local Reject Reason Codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_fc_local_rej_codes {
    SLI4_FC_LOCAL_REJECT_UNKNOWN,
    SLI4_FC_LOCAL_REJECT_MISSING_CONTINUE,
    SLI4_FC_LOCAL_REJECT_SEQUENCE_TIMEOUT,
    SLI4_FC_LOCAL_REJECT_INTERNAL_ERROR,
    SLI4_FC_LOCAL_REJECT_INVALID_RPI,
    SLI4_FC_LOCAL_REJECT_NO_XRI,
    SLI4_FC_LOCAL_REJECT_ILLEGAL_COMMAND,
    SLI4_FC_LOCAL_REJECT_XCHG_DROPPED,
    SLI4_FC_LOCAL_REJECT_ILLEGAL_FIELD,
    SLI4_FC_LOCAL_REJECT_RPI_SUSPENDED,
    SLI4_FC_LOCAL_REJECT_RSVD,
    SLI4_FC_LOCAL_REJECT_RSVD1,
    SLI4_FC_LOCAL_REJECT_NO_ABORT_MATCH,
    SLI4_FC_LOCAL_REJECT_TX_DMA_FAILED,
    SLI4_FC_LOCAL_REJECT_RX_DMA_FAILED,
    SLI4_FC_LOCAL_REJECT_ILLEGAL_FRAME,
    SLI4_FC_LOCAL_REJECT_RSVD2,
    SLI4_FC_LOCAL_REJECT_NO_RESOURCES, //0x11
    SLI4_FC_LOCAL_REJECT_FCP_CONF_FAILURE,
    SLI4_FC_LOCAL_REJECT_ILLEGAL_LENGTH,
    SLI4_FC_LOCAL_REJECT_UNSUPPORTED_FEATURE,
    SLI4_FC_LOCAL_REJECT_ABORT_IN_PROGRESS,
    SLI4_FC_LOCAL_REJECT_ABORT_REQUESTED,
    SLI4_FC_LOCAL_REJECT_RCV_BUFFER_TIMEOUT,
    SLI4_FC_LOCAL_REJECT_LOOP_OPEN_FAILURE,
    SLI4_FC_LOCAL_REJECT_RSVD3,
    SLI4_FC_LOCAL_REJECT_LINK_DOWN,
    SLI4_FC_LOCAL_REJECT_CORRUPTED_DATA,
    SLI4_FC_LOCAL_REJECT_CORRUPTED_RPI,
    SLI4_FC_LOCAL_REJECT_OUTOFORDER_DATA,
    SLI4_FC_LOCAL_REJECT_OUTOFORDER_ACK,
    SLI4_FC_LOCAL_REJECT_DUP_FRAME,
    SLI4_FC_LOCAL_REJECT_LINK_CONTROL_FRAME, //0x20
    SLI4_FC_LOCAL_REJECT_BAD_HOST_ADDRESS,
    SLI4_FC_LOCAL_REJECT_RSVD4,
    SLI4_FC_LOCAL_REJECT_MISSING_HDR_BUFFER,
    SLI4_FC_LOCAL_REJECT_MSEQ_CHAIN_CORRUPTED,
    SLI4_FC_LOCAL_REJECT_ABORTMULT_REQUESTED,
    SLI4_FC_LOCAL_REJECT_BUFFER_SHORTAGE	= 0x28,
    SLI4_FC_LOCAL_REJECT_RCV_XRIBUF_WAITING,
    SLI4_FC_LOCAL_REJECT_INVALID_VPI	= 0x2e,
    SLI4_FC_LOCAL_REJECT_NO_FPORT_DETECTED,
    SLI4_FC_LOCAL_REJECT_MISSING_XRIBUF,
    SLI4_FC_LOCAL_REJECT_RSVD5,
    SLI4_FC_LOCAL_REJECT_INVALID_XRI,
    SLI4_FC_LOCAL_REJECT_INVALID_RELOFFSET	= 0x40,
    SLI4_FC_LOCAL_REJECT_MISSING_RELOFFSET,
    SLI4_FC_LOCAL_REJECT_INSUFF_BUFFERSPACE,
    SLI4_FC_LOCAL_REJECT_MISSING_SI,
    SLI4_FC_LOCAL_REJECT_MISSING_ES,
    SLI4_FC_LOCAL_REJECT_INCOMPLETE_XFER,
    SLI4_FC_LOCAL_REJECT_SLER_FAILURE,
    SLI4_FC_LOCAL_REJECT_SLER_CMD_RCV_FAILURE,
    SLI4_FC_LOCAL_REJECT_SLER_REC_RJT_ERR,
    SLI4_FC_LOCAL_REJECT_SLER_REC_SRR_RETRY_ERR,
    SLI4_FC_LOCAL_REJECT_SLER_SRR_RJT_ERR,
    SLI4_FC_LOCAL_REJECT_RSVD6,
    SLI4_FC_LOCAL_REJECT_SLER_RRQ_RJT_ERR,
    SLI4_FC_LOCAL_REJECT_SLER_RRQ_RETRY_ERR,
    SLI4_FC_LOCAL_REJECT_SLER_ABTS_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_async_rcqe_flags {
    SLI4_RACQE_RQ_EL_INDX	= 0xfff,
    SLI4_RACQE_FCFI		= 0x3f,
    SLI4_RACQE_HDPL		= 0x3f,
    SLI4_RACQE_RQ_ID	= 0xffc0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_async_rcqe {
    pub rsvd0: u8,
    pub status: u8,
    pub rq_elmt_indx_word: __le16,
    pub rsvd4: __le32,
    pub fcfi_rq_id_word: __le16,
    pub data_placement_length: __le16,
    pub sof_byte: u8,
    pub eof_byte: u8,
    pub code: u8,
    pub hdpl_byte: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_async_rcqe_v1 {
    pub rsvd0: u8,
    pub status: u8,
    pub rq_elmt_indx_word: __le16,
    pub fcfi_byte: u8,
    pub rsvd5: u8,
    pub rsvd6: __le16,
    pub rq_id: __le16,
    pub data_placement_length: __le16,
    pub sof_byte: u8,
    pub eof_byte: u8,
    pub code: u8,
    pub hdpl_byte: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_fc_async_rq_status {
    SLI4_FC_ASYNC_RQ_SUCCESS = 0x10,
    SLI4_FC_ASYNC_RQ_BUF_LEN_EXCEEDED,
    SLI4_FC_ASYNC_RQ_INSUFF_BUF_NEEDED,
    SLI4_FC_ASYNC_RQ_INSUFF_BUF_FRM_DISC,
    SLI4_FC_ASYNC_RQ_DMA_FAILURE,
}

pub const SLI4_RCQE_RQ_EL_INDX: c_uint = 0xfff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_coalescing_rcqe {
    pub rsvd0: u8,
    pub status: u8,
    pub rq_elmt_indx_word: __le16,
    pub rsvd4: __le32,
    pub rq_id: __le16,
    pub seq_placement_length: __le16,
    pub rsvd14: __le16,
    pub code: u8,
    pub vld_byte: u8,
}

pub const SLI4_FC_COALESCE_RQ_SUCCESS: c_uint = 0x10;
pub const SLI4_FC_COALESCE_RQ_INSUFF_XRI_NEEDED: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_optimized_write_cmd_cqe_flags {
    SLI4_OCQE_RQ_EL_INDX	= 0x7f,		/* DW0 bits 16:30 */
    SLI4_OCQE_FCFI		= 0x3f,		/* DW1 bits 0:6 */
    SLI4_OCQE_OOX		= 1 << 6,	/* DW1 bit 15 */
    SLI4_OCQE_AGXR		= 1 << 7,	/* DW1 bit 16 */
    SLI4_OCQE_HDPL		= 0x3f,		/* DW3 bits 24:29*/
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_optimized_write_cmd_cqe {
    pub rsvd0: u8,
    pub status: u8,
    pub w1: __le16,
    pub flags0: u8,
    pub flags1: u8,
    pub xri: __le16,
    pub rq_id: __le16,
    pub data_placement_length: __le16,
    pub rpi: __le16,
    pub code: u8,
    pub hdpl_vld: u8,
}

pub const SLI4_OCQE_XB: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_optimized_write_data_cqe {
    pub hw_status: u8,
    pub status: u8,
    pub xri: __le16,
    pub total_data_placed: __le32,
    pub extended_status: __le32,
    pub rsvd12: __le16,
    pub code: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_fc_xri_aborted_cqe {
    pub rsvd0: u8,
    pub status: u8,
    pub rsvd2: __le16,
    pub extended_status: __le32,
    pub xri: __le16,
    pub remote_xid: __le16,
    pub rsvd12: __le16,
    pub code: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_generic_ctx {
    SLI4_GENERIC_CONTEXT_RPI,
    SLI4_GENERIC_CONTEXT_VPI,
    SLI4_GENERIC_CONTEXT_VFI,
    SLI4_GENERIC_CONTEXT_FCFI,
}

pub const SLI4_GENERIC_CLASS_CLASS_2: c_uint = 0x1;
pub const SLI4_GENERIC_CLASS_CLASS_3: c_uint = 0x2;
pub const SLI4_ELS_REQUEST64_DIR_WRITE: c_uint = 0x0;
pub const SLI4_ELS_REQUEST64_DIR_READ: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_els_request {
    SLI4_ELS_REQUEST64_OTHER,
    SLI4_ELS_REQUEST64_LOGO,
    SLI4_ELS_REQUEST64_FDISC,
    SLI4_ELS_REQUEST64_FLOGIN,
    SLI4_ELS_REQUEST64_PLOGI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_els_cmd_type {
    SLI4_ELS_REQUEST64_CMD_GEN		= 0x08,
    SLI4_ELS_REQUEST64_CMD_NON_FABRIC	= 0x0c,
    SLI4_ELS_REQUEST64_CMD_FABRIC		= 0x0d,
}

pub const SLI4_BMBX_TIMEOUT_MSEC: c_int = 30000;
pub const SLI4_FW_READY_TIMEOUT_MSEC: c_int = 30000;

//
// SLI-4 mailbox command formats and definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_mbox_command_header {
    pub resvd0: u8,
    pub command: u8,
    pub /: *mut *mut __le16 status; / Port writes to indicate success/fail,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_mbx_cmd_value {
    SLI4_MBX_CMD_CONFIG_LINK	= 0x07,
    SLI4_MBX_CMD_DUMP		= 0x17,
    SLI4_MBX_CMD_DOWN_LINK		= 0x06,
    SLI4_MBX_CMD_INIT_LINK		= 0x05,
    SLI4_MBX_CMD_INIT_VFI		= 0xa3,
    SLI4_MBX_CMD_INIT_VPI		= 0xa4,
    SLI4_MBX_CMD_POST_XRI		= 0xa7,
    SLI4_MBX_CMD_RELEASE_XRI	= 0xac,
    SLI4_MBX_CMD_READ_CONFIG	= 0x0b,
    SLI4_MBX_CMD_READ_STATUS	= 0x0e,
    SLI4_MBX_CMD_READ_NVPARMS	= 0x02,
    SLI4_MBX_CMD_READ_REV		= 0x11,
    SLI4_MBX_CMD_READ_LNK_STAT	= 0x12,
    SLI4_MBX_CMD_READ_SPARM64	= 0x8d,
    SLI4_MBX_CMD_READ_TOPOLOGY	= 0x95,
    SLI4_MBX_CMD_REG_FCFI		= 0xa0,
    SLI4_MBX_CMD_REG_FCFI_MRQ	= 0xaf,
    SLI4_MBX_CMD_REG_RPI		= 0x93,
    SLI4_MBX_CMD_REG_RX_RQ		= 0xa6,
    SLI4_MBX_CMD_REG_VFI		= 0x9f,
    SLI4_MBX_CMD_REG_VPI		= 0x96,
    SLI4_MBX_CMD_RQST_FEATURES	= 0x9d,
    SLI4_MBX_CMD_SLI_CONFIG		= 0x9b,
    SLI4_MBX_CMD_UNREG_FCFI		= 0xa2,
    SLI4_MBX_CMD_UNREG_RPI		= 0x14,
    SLI4_MBX_CMD_UNREG_VFI		= 0xa1,
    SLI4_MBX_CMD_UNREG_VPI		= 0x97,
    SLI4_MBX_CMD_WRITE_NVPARMS	= 0x03,
    SLI4_MBX_CMD_CFG_AUTO_XFER_RDY	= 0xad,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_mbx_status {
    SLI4_MBX_STATUS_SUCCESS		= 0x0000,
    SLI4_MBX_STATUS_FAILURE		= 0x0001,
    SLI4_MBX_STATUS_RPI_NOT_REG	= 0x1400,
}

// CONFIG_LINK - configure link-oriented parameters,
// such as default N_Port_ID address and various timers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cmd_config_link_flags {
    SLI4_CFG_LINK_BBSCN = 0xf00,
    SLI4_CFG_LINK_CSCN  = 0x1000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_config_link {
    pub hdr: sli4_mbox_command_header,
    pub maxbbc: u8,
    pub rsvd5: u8,
    pub rsvd6: u8,
    pub rsvd7: u8,
    pub alpa: u8,
    pub n_port_id: __le16,
    pub rsvd11: u8,
    pub rsvd12: __le32,
    pub e_d_tov: __le32,
    pub lp_tov: __le32,
    pub r_a_tov: __le32,
    pub r_t_tov: __le32,
    pub al_tov: __le32,
    pub rsvd36: __le32,
    pub bbscn_dword: __le32,
}

pub const SLI4_DUMP4_TYPE: c_uint = 0xf;
pub const SLI4_WKI_TAG_SAT_TEM: c_uint = 0x1040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_dump4 {
    pub hdr: sli4_mbox_command_header,
    pub type_dword: __le32,
    pub wki_selection: __le16,
    pub rsvd10: __le16,
    pub rsvd12: __le32,
    pub returned_byte_cnt: __le32,
    pub resp_data: [__le32; 59],
}

// INIT_LINK - initialize the link for a FC port
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_init_link_flags {
    SLI4_INIT_LINK_F_LOOPBACK	= 1 << 0,

    SLI4_INIT_LINK_F_P2P_ONLY	= 1 << 1,
    SLI4_INIT_LINK_F_FCAL_ONLY	= 2 << 1,
    SLI4_INIT_LINK_F_FCAL_FAIL_OVER	= 0 << 1,
    SLI4_INIT_LINK_F_P2P_FAIL_OVER	= 1 << 1,

    SLI4_INIT_LINK_F_UNFAIR		= 1 << 6,
    SLI4_INIT_LINK_F_NO_LIRP	= 1 << 7,
    SLI4_INIT_LINK_F_LOOP_VALID_CHK	= 1 << 8,
    SLI4_INIT_LINK_F_NO_LISA	= 1 << 9,
    SLI4_INIT_LINK_F_FAIL_OVER	= 1 << 10,
    SLI4_INIT_LINK_F_FIXED_SPEED	= 1 << 11,
    SLI4_INIT_LINK_F_PICK_HI_ALPA	= 1 << 15,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_fc_link_speed {
    SLI4_LINK_SPEED_1G = 1,
    SLI4_LINK_SPEED_2G,
    SLI4_LINK_SPEED_AUTO_1_2,
    SLI4_LINK_SPEED_4G,
    SLI4_LINK_SPEED_AUTO_4_1,
    SLI4_LINK_SPEED_AUTO_4_2,
    SLI4_LINK_SPEED_AUTO_4_2_1,
    SLI4_LINK_SPEED_8G,
    SLI4_LINK_SPEED_AUTO_8_1,
    SLI4_LINK_SPEED_AUTO_8_2,
    SLI4_LINK_SPEED_AUTO_8_2_1,
    SLI4_LINK_SPEED_AUTO_8_4,
    SLI4_LINK_SPEED_AUTO_8_4_1,
    SLI4_LINK_SPEED_AUTO_8_4_2,
    SLI4_LINK_SPEED_10G,
    SLI4_LINK_SPEED_16G,
    SLI4_LINK_SPEED_AUTO_16_8_4,
    SLI4_LINK_SPEED_AUTO_16_8,
    SLI4_LINK_SPEED_32G,
    SLI4_LINK_SPEED_AUTO_32_16_8,
    SLI4_LINK_SPEED_AUTO_32_16,
    SLI4_LINK_SPEED_64G,
    SLI4_LINK_SPEED_AUTO_64_32_16,
    SLI4_LINK_SPEED_AUTO_64_32,
    SLI4_LINK_SPEED_128G,
    SLI4_LINK_SPEED_AUTO_128_64_32,
    SLI4_LINK_SPEED_AUTO_128_64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_init_link {
    pub hdr: sli4_mbox_command_header,
    pub sel_reset_al_pa_dword: __le32,
    pub flags0: __le32,
    pub link_speed_sel_code: __le32,
}

// INIT_VFI - initialize the VFI resource
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_init_vfi_flags {
    SLI4_INIT_VFI_FLAG_VP	= 0x1000,
    SLI4_INIT_VFI_FLAG_VF	= 0x2000,
    SLI4_INIT_VFI_FLAG_VT	= 0x4000,
    SLI4_INIT_VFI_FLAG_VR	= 0x8000,

    SLI4_INIT_VFI_VFID	= 0x1fff,
    SLI4_INIT_VFI_PRI	= 0xe000,

    SLI4_INIT_VFI_HOP_COUNT = 0xff000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_init_vfi {
    pub hdr: sli4_mbox_command_header,
    pub vfi: __le16,
    pub flags0_word: __le16,
    pub fcfi: __le16,
    pub vpi: __le16,
    pub vf_id_pri_dword: __le32,
    pub hop_cnt_dword: __le32,
}

// INIT_VPI - initialize the VPI resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_init_vpi {
    pub hdr: sli4_mbox_command_header,
    pub vpi: __le16,
    pub vfi: __le16,
}

// POST_XRI - post XRI resources to the SLI Port
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_post_xri_flags {
    SLI4_POST_XRI_COUNT	= 0xfff,
    SLI4_POST_XRI_FLAG_ENX	= 0x1000,
    SLI4_POST_XRI_FLAG_DL	= 0x2000,
    SLI4_POST_XRI_FLAG_DI	= 0x4000,
    SLI4_POST_XRI_FLAG_VAL	= 0x8000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_post_xri {
    pub hdr: sli4_mbox_command_header,
    pub xri_base: __le16,
    pub xri_count_flags: __le16,
}

// RELEASE_XRI - Release XRI resources from the SLI Port
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_release_xri_flags {
    SLI4_RELEASE_XRI_REL_XRI_CNT	= 0x1f,
    SLI4_RELEASE_XRI_COUNT		= 0x1f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_release_xri {
    pub hdr: sli4_mbox_command_header,
    pub rel_xri_count_word: __le16,
    pub xri_count_word: __le16,
    pub xri_tag0: __le16,
    pub xri_tag1: __le16,
    pub xri_tbl: [}; 62],
}

// READ_CONFIG - read SLI port configuration parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_config {
    pub hdr: sli4_mbox_command_header,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_cfg_resp_flags {
    SLI4_READ_CFG_RESP_RESOURCE_EXT = 0x80000000,	/* DW1 */
    SLI4_READ_CFG_RESP_TOPOLOGY	= 0xff000000,	/* DW2 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_cfg_topo {
    SLI4_READ_CFG_TOPO_FC		= 0x1,	/* FC topology unknown */
    SLI4_READ_CFG_TOPO_NON_FC_AL	= 0x2,	/* FC point-to-point or fabric */
    SLI4_READ_CFG_TOPO_FC_AL	= 0x3,	/* FC-AL topology */
}

// Link Module Type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_cfg_lmt {
    SLI4_LINK_MODULE_TYPE_1GB	= 0x0004,
    SLI4_LINK_MODULE_TYPE_2GB	= 0x0008,
    SLI4_LINK_MODULE_TYPE_4GB	= 0x0040,
    SLI4_LINK_MODULE_TYPE_8GB	= 0x0080,
    SLI4_LINK_MODULE_TYPE_16GB	= 0x0200,
    SLI4_LINK_MODULE_TYPE_32GB	= 0x0400,
    SLI4_LINK_MODULE_TYPE_64GB	= 0x0800,
    SLI4_LINK_MODULE_TYPE_128GB	= 0x1000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_read_config {
    pub hdr: sli4_mbox_command_header,
    pub ext_dword: __le32,
    pub topology_dword: __le32,
    pub resvd8: __le32,
    pub e_d_tov: __le16,
    pub resvd14: __le16,
    pub resvd16: __le32,
    pub r_a_tov: __le16,
    pub resvd22: __le16,
    pub resvd24: __le32,
    pub resvd28: __le32,
    pub lmt: __le16,
    pub resvd34: __le16,
    pub resvd36: __le32,
    pub resvd40: __le32,
    pub xri_base: __le16,
    pub xri_count: __le16,
    pub rpi_base: __le16,
    pub rpi_count: __le16,
    pub vpi_base: __le16,
    pub vpi_count: __le16,
    pub vfi_base: __le16,
    pub vfi_count: __le16,
    pub resvd60: __le16,
    pub fcfi_count: __le16,
    pub rq_count: __le16,
    pub eq_count: __le16,
    pub wq_count: __le16,
    pub cq_count: __le16,
    pub pad: [__le32; 45],
}

// READ_NVPARMS - read SLI port configuration parameters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_nvparms_flags {
    SLI4_READ_NVPARAMS_HARD_ALPA	  = 0xff,
    SLI4_READ_NVPARAMS_PREFERRED_D_ID = 0xffffff00,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_nvparms {
    pub hdr: sli4_mbox_command_header,
    pub resvd0: __le32,
    pub resvd4: __le32,
    pub resvd8: __le32,
    pub resvd12: __le32,
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
    pub hard_alpa_d_id: __le32,
}

// WRITE_NVPARMS - write SLI port configuration parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_write_nvparms {
    pub hdr: sli4_mbox_command_header,
    pub resvd0: __le32,
    pub resvd4: __le32,
    pub resvd8: __le32,
    pub resvd12: __le32,
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
    pub hard_alpa_d_id: __le32,
}

// READ_REV - read the Port revision levels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_rev {
    pub hdr: sli4_mbox_command_header,
    pub resvd0: __le16,
    pub flags0_word: __le16,
    pub first_hw_rev: __le32,
    pub second_hw_rev: __le32,
    pub resvd12: __le32,
    pub third_hw_rev: __le32,
    pub fc_ph_low: u8,
    pub fc_ph_high: u8,
    pub feature_level_low: u8,
    pub feature_level_high: u8,
    pub resvd24: __le32,
    pub first_fw_id: __le32,
    pub first_fw_name: [u8; 16],
    pub second_fw_id: __le32,
    pub second_fw_name: [u8; 16],
    pub rsvd18: [__le32; 30],
    pub available_length_dword: __le32,
    pub hostbuf: sli4_dmaaddr,
    pub returned_vpd_length: __le32,
    pub actual_vpd_length: __le32,
}

// READ_SPARM64 - read the Port service parameters

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_sparm64 {
    pub hdr: sli4_mbox_command_header,
    pub resvd0: __le32,
    pub resvd4: __le32,
    pub bde_64: sli4_bde,
    pub vpi: __le16,
    pub resvd22: __le16,
    pub port_name_start: __le16,
    pub port_name_len: __le16,
    pub node_name_start: __le16,
    pub node_name_len: __le16,
}

// READ_TOPOLOGY - read the link event information
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_topo_e {
    SLI4_READTOPO_ATTEN_TYPE	= 0xff,
    SLI4_READTOPO_FLAG_IL		= 0x100,
    SLI4_READTOPO_FLAG_PB_RECVD	= 0x200,

    SLI4_READTOPO_LINKSTATE_RECV	= 0x3,
    SLI4_READTOPO_LINKSTATE_TRANS	= 0xc,
    SLI4_READTOPO_LINKSTATE_MACHINE	= 0xf0,
    SLI4_READTOPO_LINKSTATE_SPEED	= 0xff00,
    SLI4_READTOPO_LINKSTATE_TF	= 0x40000000,
    SLI4_READTOPO_LINKSTATE_LU	= 0x80000000,

    SLI4_READTOPO_SCN_BBSCN		= 0xf,
    SLI4_READTOPO_SCN_CBBSCN	= 0xf0,

    SLI4_READTOPO_R_T_TOV		= 0x1ff,
    SLI4_READTOPO_AL_TOV		= 0xf000,

    SLI4_READTOPO_PB_FLAG		= 0x80,

    SLI4_READTOPO_INIT_N_PORTID	= 0xffffff,
}

pub const SLI4_MIN_LOOP_MAP_BYTES: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_topology {
    pub hdr: sli4_mbox_command_header,
    pub event_tag: __le32,
    pub dw2_attentype: __le32,
    pub topology: u8,
    pub lip_type: u8,
    pub lip_al_ps: u8,
    pub al_pa_granted: u8,
    pub bde_loop_map: sli4_bde,
    pub linkdown_state: __le32,
    pub currlink_state: __le32,
    pub max_bbc: u8,
    pub init_bbc: u8,
    pub scn_flags: u8,
    pub rsvd39: u8,
    pub dw10w0_al_rt_tov: __le16,
    pub lp_tov: __le16,
    pub acquired_al_pa: u8,
    pub pb_flags: u8,
    pub specified_al_pa: __le16,
    pub dw12_init_n_port_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_topo_link {
    SLI4_READ_TOPOLOGY_LINK_UP	= 0x1,
    SLI4_READ_TOPOLOGY_LINK_DOWN,
    SLI4_READ_TOPOLOGY_LINK_NO_ALPA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_topo {
    SLI4_READ_TOPO_UNKNOWN		= 0x0,
    SLI4_READ_TOPO_NON_FC_AL,
    SLI4_READ_TOPO_FC_AL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_topo_speed {
    SLI4_READ_TOPOLOGY_SPEED_NONE	= 0x00,
    SLI4_READ_TOPOLOGY_SPEED_1G	= 0x04,
    SLI4_READ_TOPOLOGY_SPEED_2G	= 0x08,
    SLI4_READ_TOPOLOGY_SPEED_4G	= 0x10,
    SLI4_READ_TOPOLOGY_SPEED_8G	= 0x20,
    SLI4_READ_TOPOLOGY_SPEED_10G	= 0x40,
    SLI4_READ_TOPOLOGY_SPEED_16G	= 0x80,
    SLI4_READ_TOPOLOGY_SPEED_32G	= 0x90,
    SLI4_READ_TOPOLOGY_SPEED_64G	= 0xa0,
    SLI4_READ_TOPOLOGY_SPEED_128G	= 0xb0,
}

// REG_FCFI - activate a FC Forwarder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_reg_fcfi_rq_cfg {
    pub r_ctl_mask: u8,
    pub r_ctl_match: u8,
    pub type_mask: u8,
    pub type_match: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_regfcfi_tag {
    SLI4_REGFCFI_VLAN_TAG		= 0xfff,
    SLI4_REGFCFI_VLANTAG_VALID	= 0x1000,
}

pub const SLI4_CMD_REG_FCFI_NUM_RQ_CFG: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_reg_fcfi {
    pub hdr: sli4_mbox_command_header,
    pub fcf_index: __le16,
    pub fcfi: __le16,
    pub rqid1: __le16,
    pub rqid0: __le16,
    pub rqid3: __le16,
    pub rqid2: __le16,
    pub dw8_vlan: __le32,
}

pub const SLI4_CMD_REG_FCFI_MRQ_NUM_RQ_CFG: c_int = 4;
pub const SLI4_CMD_REG_FCFI_MRQ_MAX_NUM_RQ: c_int = 32;
pub const SLI4_CMD_REG_FCFI_SET_FCFI_MODE: c_int = 0;
pub const SLI4_CMD_REG_FCFI_SET_MRQ_MODE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_reg_fcfi_mrq {
    SLI4_REGFCFI_MRQ_VLAN_TAG	= 0xfff,
    SLI4_REGFCFI_MRQ_VLANTAG_VALID	= 0x1000,
    SLI4_REGFCFI_MRQ_MODE		= 0x2000,

    SLI4_REGFCFI_MRQ_MASK_NUM_PAIRS	= 0xff,
    SLI4_REGFCFI_MRQ_FILTER_BITMASK = 0xf00,
    SLI4_REGFCFI_MRQ_RQ_SEL_POLICY	= 0xf000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_reg_fcfi_mrq {
    pub hdr: sli4_mbox_command_header,
    pub fcf_index: __le16,
    pub fcfi: __le16,
    pub rqid1: __le16,
    pub rqid0: __le16,
    pub rqid3: __le16,
    pub rqid2: __le16,
    pub dw8_vlan: __le32,
    pub dw9_mrqflags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_rq_cfg {
    pub rq_id: __le16,
    pub r_ctl_mask: u8,
    pub r_ctl_match: u8,
    pub type_mask: u8,
    pub type_match: u8,
}

// REG_RPI - register a Remote Port Indicator
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_reg_rpi {
    SLI4_REGRPI_REMOTE_N_PORTID	= 0xffffff,	/* DW2 */
    SLI4_REGRPI_UPD			= 0x1000000,
    SLI4_REGRPI_ETOW		= 0x8000000,
    SLI4_REGRPI_TERP		= 0x20000000,
    SLI4_REGRPI_CI			= 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_reg_rpi {
    pub hdr: sli4_mbox_command_header,
    pub rpi: __le16,
    pub rsvd2: __le16,
    pub dw2_rportid_flags: __le32,
    pub bde_64: sli4_bde,
    pub vpi: __le16,
    pub rsvd26: __le16,
}

pub const SLI4_REG_RPI_BUF_LEN: c_uint = 0x70;
// REG_VFI - register a Virtual Fabric Indicator
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli_reg_vfi {
    SLI4_REGVFI_VP			= 0x1000,	/* DW1 */
    SLI4_REGVFI_UPD			= 0x2000,

    SLI4_REGVFI_LOCAL_N_PORTID	= 0xffffff,	/* DW10 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_reg_vfi {
    pub hdr: sli4_mbox_command_header,
    pub vfi: __le16,
    pub dw0w1_flags: __le16,
    pub fcfi: __le16,
    pub vpi: __le16,
    pub wwpn: [u8; 8],
    pub sparm: sli4_bde,
    pub e_d_tov: __le32,
    pub r_a_tov: __le32,
    pub dw10_lportid_flags: __le32,
}

// REG_VPI - register a Virtual Port Indicator
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_reg_vpi {
    SLI4_REGVPI_LOCAL_N_PORTID	= 0xffffff,
    SLI4_REGVPI_UPD			= 0x1000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_reg_vpi {
    pub hdr: sli4_mbox_command_header,
    pub rsvd0: __le32,
    pub dw2_lportid_flags: __le32,
    pub wwpn: [u8; 8],
    pub rsvd12: __le32,
    pub vpi: __le16,
    pub vfi: __le16,
}

// REQUEST_FEATURES - request / query SLI features
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_req_features_flags {
    SLI4_REQFEAT_QRY	= 0x1,		/* Dw1 */

    SLI4_REQFEAT_IAAB	= 1 << 0,	/* DW2 & DW3 */
    SLI4_REQFEAT_NPIV	= 1 << 1,
    SLI4_REQFEAT_DIF	= 1 << 2,
    SLI4_REQFEAT_VF		= 1 << 3,
    SLI4_REQFEAT_FCPI	= 1 << 4,
    SLI4_REQFEAT_FCPT	= 1 << 5,
    SLI4_REQFEAT_FCPC	= 1 << 6,
    SLI4_REQFEAT_RSVD	= 1 << 7,
    SLI4_REQFEAT_RQD	= 1 << 8,
    SLI4_REQFEAT_IAAR	= 1 << 9,
    SLI4_REQFEAT_HLM	= 1 << 10,
    SLI4_REQFEAT_PERFH	= 1 << 11,
    SLI4_REQFEAT_RXSEQ	= 1 << 12,
    SLI4_REQFEAT_RXRI	= 1 << 13,
    SLI4_REQFEAT_DCL2	= 1 << 14,
    SLI4_REQFEAT_RSCO	= 1 << 15,
    SLI4_REQFEAT_MRQP	= 1 << 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_request_features {
    pub hdr: sli4_mbox_command_header,
    pub dw1_qry: __le32,
    pub cmd: __le32,
    pub resp: __le32,
}

//
// SLI_CONFIG - submit a configuration command to Port
//
// Command is either embedded as part of the payload (embed) or located
// in a separate memory buffer (mem)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_sli_config {
    SLI4_SLICONF_EMB		= 0x1,		/* DW1 */
    SLI4_SLICONF_PMDCMD_SHIFT	= 3,
    SLI4_SLICONF_PMDCMD_MASK	= 0xf8,
    SLI4_SLICONF_PMDCMD_VAL_1	= 8,
    SLI4_SLICONF_PMDCNT		= 0xf8,

    SLI4_SLICONF_PMD_LEN		= 0x00ffffff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_sli_config {
    pub hdr: sli4_mbox_command_header,
    pub dw1_flags: __le32,
    pub payload_len: __le32,
    pub rsvd12: [__le32; 3],
    pub sizeof(u32)]: *mut *mut u8 embed[58,
    pub mem: sli4_bufptr,
    pub payload: },
}

// READ_STATUS - read tx/rx status of a particular port
pub const SLI4_READSTATUS_CLEAR_COUNTERS: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_status {
    pub hdr: sli4_mbox_command_header,
    pub dw1_flags: __le32,
    pub rsvd4: __le32,
    pub trans_kbyte_cnt: __le32,
    pub recv_kbyte_cnt: __le32,
    pub trans_frame_cnt: __le32,
    pub recv_frame_cnt: __le32,
    pub trans_seq_cnt: __le32,
    pub recv_seq_cnt: __le32,
    pub tot_exchanges_orig: __le32,
    pub tot_exchanges_resp: __le32,
    pub recv_p_bsy_cnt: __le32,
    pub recv_f_bsy_cnt: __le32,
    pub no_rq_buf_dropped_frames_cnt: __le32,
    pub empty_rq_timeout_cnt: __le32,
    pub no_xri_dropped_frames_cnt: __le32,
    pub empty_xri_pool_cnt: __le32,
}

// READ_LNK_STAT - read link status of a particular port
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_read_link_stats_flags {
    SLI4_READ_LNKSTAT_REC	= 1u << 0,
    SLI4_READ_LNKSTAT_GEC	= 1u << 1,
    SLI4_READ_LNKSTAT_W02OF	= 1u << 2,
    SLI4_READ_LNKSTAT_W03OF	= 1u << 3,
    SLI4_READ_LNKSTAT_W04OF	= 1u << 4,
    SLI4_READ_LNKSTAT_W05OF	= 1u << 5,
    SLI4_READ_LNKSTAT_W06OF	= 1u << 6,
    SLI4_READ_LNKSTAT_W07OF	= 1u << 7,
    SLI4_READ_LNKSTAT_W08OF	= 1u << 8,
    SLI4_READ_LNKSTAT_W09OF	= 1u << 9,
    SLI4_READ_LNKSTAT_W10OF = 1u << 10,
    SLI4_READ_LNKSTAT_W11OF = 1u << 11,
    SLI4_READ_LNKSTAT_W12OF	= 1u << 12,
    SLI4_READ_LNKSTAT_W13OF	= 1u << 13,
    SLI4_READ_LNKSTAT_W14OF	= 1u << 14,
    SLI4_READ_LNKSTAT_W15OF	= 1u << 15,
    SLI4_READ_LNKSTAT_W16OF	= 1u << 16,
    SLI4_READ_LNKSTAT_W17OF	= 1u << 17,
    SLI4_READ_LNKSTAT_W18OF	= 1u << 18,
    SLI4_READ_LNKSTAT_W19OF	= 1u << 19,
    SLI4_READ_LNKSTAT_W20OF	= 1u << 20,
    SLI4_READ_LNKSTAT_W21OF	= 1u << 21,
    SLI4_READ_LNKSTAT_CLRC	= 1u << 30,
    SLI4_READ_LNKSTAT_CLOF	= 1u << 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_read_link_stats {
    pub hdr: sli4_mbox_command_header,
    pub dw1_flags: __le32,
    pub linkfail_errcnt: __le32,
    pub losssync_errcnt: __le32,
    pub losssignal_errcnt: __le32,
    pub primseq_errcnt: __le32,
    pub inval_txword_errcnt: __le32,
    pub crc_errcnt: __le32,
    pub primseq_eventtimeout_cnt: __le32,
    pub elastic_bufoverrun_errcnt: __le32,
    pub arbit_fc_al_timeout_cnt: __le32,
    pub adv_rx_buftor_to_buf_credit: __le32,
    pub curr_rx_buf_to_buf_credit: __le32,
    pub adv_tx_buf_to_buf_credit: __le32,
    pub curr_tx_buf_to_buf_credit: __le32,
    pub rx_eofa_cnt: __le32,
    pub rx_eofdti_cnt: __le32,
    pub rx_eofni_cnt: __le32,
    pub rx_soff_cnt: __le32,
    pub rx_dropped_no_aer_cnt: __le32,
    pub rx_dropped_no_avail_rpi_rescnt: __le32,
    pub rx_dropped_no_avail_xri_rescnt: __le32,
}

// Format a WQE with WQ_ID Association performance hint
//
// Set Word 10, bit 0 to zero
// Set Word 10, bits 15:1 to the WQ ID
//
// UNREG_FCFI - unregister a FCFI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_unreg_fcfi {
    pub hdr: sli4_mbox_command_header,
    pub rsvd0: __le32,
    pub fcfi: __le16,
    pub rsvd6: __le16,
}

// UNREG_RPI - unregister one or more RPI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_unreg_rpi {
    SLI4_UNREG_RPI_DP	= 0x2000,
    SLI4_UNREG_RPI_II_SHIFT	= 14,
    SLI4_UNREG_RPI_II_MASK	= 0xc000,
    SLI4_UNREG_RPI_II_RPI	= 0x0000,
    SLI4_UNREG_RPI_II_VPI	= 0x4000,
    SLI4_UNREG_RPI_II_VFI	= 0x8000,
    SLI4_UNREG_RPI_II_FCFI	= 0xc000,

    SLI4_UNREG_RPI_DEST_N_PORTID_MASK = 0x00ffffff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_unreg_rpi {
    pub hdr: sli4_mbox_command_header,
    pub index: __le16,
    pub dw1w1_flags: __le16,
    pub dw2_dest_n_portid: __le32,
}

// UNREG_VFI - unregister one or more VFI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_unreg_vfi {
    SLI4_UNREG_VFI_II_SHIFT	= 14,
    SLI4_UNREG_VFI_II_MASK	= 0xc000,
    SLI4_UNREG_VFI_II_VFI	= 0x0000,
    SLI4_UNREG_VFI_II_FCFI	= 0xc000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_unreg_vfi {
    pub hdr: sli4_mbox_command_header,
    pub rsvd0: __le32,
    pub index: __le16,
    pub dw2_flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_unreg_type {
    SLI4_UNREG_TYPE_PORT,
    SLI4_UNREG_TYPE_DOMAIN,
    SLI4_UNREG_TYPE_FCF,
    SLI4_UNREG_TYPE_ALL
}

// UNREG_VPI - unregister one or more VPI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_unreg_vpi {
    SLI4_UNREG_VPI_II_SHIFT	= 14,
    SLI4_UNREG_VPI_II_MASK	= 0xc000,
    SLI4_UNREG_VPI_II_VPI	= 0x0000,
    SLI4_UNREG_VPI_II_VFI	= 0x8000,
    SLI4_UNREG_VPI_II_FCFI	= 0xc000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_unreg_vpi {
    pub hdr: sli4_mbox_command_header,
    pub rsvd0: __le32,
    pub index: __le16,
    pub dw2w0_flags: __le16,
}

// AUTO_XFER_RDY - Configure the auto-generate XFER-RDY feature
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_config_auto_xfer_rdy {
    pub hdr: sli4_mbox_command_header,
    pub rsvd0: __le32,
    pub max_burst_len: __le32,
}

pub const SLI4_CONFIG_AUTO_XFERRDY_BLKSIZE: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_cmd_config_auto_xfer_rdy_hp {
    pub hdr: sli4_mbox_command_header,
    pub rsvd0: __le32,
    pub max_burst_len: __le32,
    pub dw3_esoc_flags: __le32,
    pub block_size: __le16,
    pub rsvd14: __le16,
}

//
// SLI-4 common configuration command formats and definitions
//
// Subsystem values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_subsystem {
    SLI4_SUBSYSTEM_COMMON	= 0x01,
    SLI4_SUBSYSTEM_LOWLEVEL	= 0x0b,
    SLI4_SUBSYSTEM_FC	= 0x0c,
    SLI4_SUBSYSTEM_DMTF	= 0x11,
}

//
// Common opcode (OPC) values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cmn_opcode {
    SLI4_CMN_FUNCTION_RESET		= 0x3d,
    SLI4_CMN_CREATE_CQ		= 0x0c,
    SLI4_CMN_CREATE_CQ_SET		= 0x1d,
    SLI4_CMN_DESTROY_CQ		= 0x36,
    SLI4_CMN_MODIFY_EQ_DELAY	= 0x29,
    SLI4_CMN_CREATE_EQ		= 0x0d,
    SLI4_CMN_DESTROY_EQ		= 0x37,
    SLI4_CMN_CREATE_MQ_EXT		= 0x5a,
    SLI4_CMN_DESTROY_MQ		= 0x35,
    SLI4_CMN_GET_CNTL_ATTRIBUTES	= 0x20,
    SLI4_CMN_NOP			= 0x21,
    SLI4_CMN_GET_RSC_EXTENT_INFO	= 0x9a,
    SLI4_CMN_GET_SLI4_PARAMS	= 0xb5,
    SLI4_CMN_QUERY_FW_CONFIG	= 0x3a,
    SLI4_CMN_GET_PORT_NAME		= 0x4d,

    SLI4_CMN_WRITE_FLASHROM		= 0x07,
// TRANSCEIVER Data
    SLI4_CMN_READ_TRANS_DATA	= 0x49,
    SLI4_CMN_GET_CNTL_ADDL_ATTRS	= 0x79,
    SLI4_CMN_GET_FUNCTION_CFG	= 0xa0,
    SLI4_CMN_GET_PROFILE_CFG	= 0xa4,
    SLI4_CMN_SET_PROFILE_CFG	= 0xa5,
    SLI4_CMN_GET_PROFILE_LIST	= 0xa6,
    SLI4_CMN_GET_ACTIVE_PROFILE	= 0xa7,
    SLI4_CMN_SET_ACTIVE_PROFILE	= 0xa8,
    SLI4_CMN_READ_OBJECT		= 0xab,
    SLI4_CMN_WRITE_OBJECT		= 0xac,
    SLI4_CMN_DELETE_OBJECT		= 0xae,
    SLI4_CMN_READ_OBJECT_LIST	= 0xad,
    SLI4_CMN_SET_DUMP_LOCATION	= 0xb8,
    SLI4_CMN_SET_FEATURES		= 0xbf,
    SLI4_CMN_GET_RECFG_LINK_INFO	= 0xc9,
    SLI4_CMN_SET_RECNG_LINK_ID	= 0xca,
}

// DMTF opcode (OPC) values
pub const DMTF_EXEC_CLP_CMD: c_uint = 0x01;
//
// COMMON_FUNCTION_RESET
//
// Resets the Port, returning it to a power-on state. This configuration
// command does not have a payload and should set/expect the lengths to
// be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_function_reset {
    pub hdr: sli4_rqst_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_function_reset {
    pub hdr: sli4_rsp_hdr,
}

//
// COMMON_GET_CNTL_ATTRIBUTES
//
// Query for information about the SLI Port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_cntrl_attr_flags {
    SLI4_CNTL_ATTR_PORTNUM	= 0x3f,
    SLI4_CNTL_ATTR_PORTTYPE	= 0xc0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_cntl_attributes {
    pub hdr: sli4_rsp_hdr,
    pub version_str: [u8; 32],
    pub manufacturer_name: [u8; 32],
    pub supported_modes: __le32,
    pub eprom_version_lo: u8,
    pub eprom_version_hi: u8,
    pub rsvd17: __le16,
    pub mbx_ds_version: __le32,
    pub ep_fw_ds_version: __le32,
    pub ncsi_version_str: [u8; 12],
    pub def_extended_timeout: __le32,
    pub model_number: [u8; 32],
    pub description: [u8; 64],
    pub serial_number: [u8; 32],
    pub ip_version_str: [u8; 32],
    pub fw_version_str: [u8; 32],
    pub bios_version_str: [u8; 32],
    pub redboot_version_str: [u8; 32],
    pub driver_version_str: [u8; 32],
    pub fw_on_flash_version_str: [u8; 32],
    pub functionalities_supported: __le32,
    pub max_cdb_length: __le16,
    pub asic_revision: u8,
    pub generational_guid0: u8,
    pub generational_guid1_12: [__le32; 3],
    pub generational_guid13_14: __le16,
    pub generational_guid15: u8,
    pub hba_port_count: u8,
    pub default_link_down_timeout: __le16,
    pub iscsi_version_min_max: u8,
    pub multifunctional_device: u8,
    pub cache_valid: u8,
    pub hba_status: u8,
    pub max_domains_supported: u8,
    pub port_num_type_flags: u8,
    pub firmware_post_status: __le32,
    pub hba_mtu: __le32,
    pub iscsi_features: u8,
    pub rsvd121: [u8; 3],
    pub pci_vendor_id: __le16,
    pub pci_device_id: __le16,
    pub pci_sub_vendor_id: __le16,
    pub pci_sub_system_id: __le16,
    pub pci_bus_number: u8,
    pub pci_device_number: u8,
    pub pci_function_number: u8,
    pub interface_type: u8,
    pub unique_identifier: __le64,
    pub number_of_netfilters: u8,
    pub rsvd122: [u8; 3],
}

//
// COMMON_GET_CNTL_ATTRIBUTES
//
// This command queries the controller information from the Flash ROM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_get_cntl_addl_attributes {
    pub hdr: sli4_rqst_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_cntl_addl_attributes {
    pub hdr: sli4_rsp_hdr,
    pub ipl_file_number: __le16,
    pub ipl_file_version: u8,
    pub rsvd4: u8,
    pub on_die_temperature: u8,
    pub rsvd5: [u8; 3],
    pub driver_advanced_features_supported: __le32,
    pub rsvd7: [__le32; 4],
    pub universal_bios_version: [c_char; 32],
    pub x86_bios_version: [c_char; 32],
    pub efi_bios_version: [c_char; 32],
    pub fcode_version: [c_char; 32],
    pub uefi_bios_version: [c_char; 32],
    pub uefi_nic_version: [c_char; 32],
    pub uefi_fcode_version: [c_char; 32],
    pub uefi_iscsi_version: [c_char; 32],
    pub iscsi_x86_bios_version: [c_char; 32],
    pub pxe_x86_bios_version: [c_char; 32],
    pub default_wwpn: [u8; 8],
    pub ext_phy_version: [u8; 32],
    pub fc_universal_bios_version: [u8; 32],
    pub fc_x86_bios_version: [u8; 32],
    pub fc_efi_bios_version: [u8; 32],
    pub fc_fcode_version: [u8; 32],
    pub ext_phy_crc_label: [u8; 8],
    pub ipl_file_name: [u8; 16],
    pub rsvd139: [u8; 72],
}

//
// COMMON_NOP
//
// This command does not do anything; it only returns
// the payload in the completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_nop {
    pub hdr: sli4_rqst_hdr,
    pub context: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_nop {
    pub hdr: sli4_rsp_hdr,
    pub context: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_get_resource_extent_info {
    pub hdr: sli4_rqst_hdr,
    pub resource_type: __le16,
    pub rsvd16: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rsc_type {
    SLI4_RSC_TYPE_VFI	= 0x20,
    SLI4_RSC_TYPE_VPI	= 0x21,
    SLI4_RSC_TYPE_RPI	= 0x22,
    SLI4_RSC_TYPE_XRI	= 0x23,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_resource_extent_info {
    pub hdr: sli4_rsp_hdr,
    pub resource_extent_count: __le16,
    pub resource_extent_size: __le16,
}

pub const SLI4_128BYTE_WQE_SUPPORT: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rsp_get_params_e {
// GENERIC
    SLI4_PARAM_Q_CNT_MTHD_SHFT	= 24,
    SLI4_PARAM_Q_CNT_MTHD_MASK	= 0xf << 24,
    SLI4_PARAM_QV_SHIFT		= 14,
    SLI4_PARAM_QV_MASK		= 3 << 14,

// DW4
    SLI4_PARAM_PROTO_TYPE_MASK	= 0xff,
// DW5
    SLI4_PARAM_FT			= 1 << 0,
    SLI4_PARAM_SLI_REV_MASK		= 0xf << 4,
    SLI4_PARAM_SLI_FAM_MASK		= 0xf << 8,
    SLI4_PARAM_IF_TYPE_MASK		= 0xf << 12,
    SLI4_PARAM_SLI_HINT1_MASK	= 0xff << 16,
    SLI4_PARAM_SLI_HINT2_MASK	= 0x1f << 24,
// DW6
    SLI4_PARAM_EQ_PAGE_CNT_MASK	= 0xf << 0,
    SLI4_PARAM_EQE_SZS_MASK		= 0xf << 8,
    SLI4_PARAM_EQ_PAGE_SZS_MASK	= 0xff << 16,
// DW8
    SLI4_PARAM_CQ_PAGE_CNT_MASK	= 0xf << 0,
    SLI4_PARAM_CQE_SZS_MASK		= 0xf << 8,
    SLI4_PARAM_CQ_PAGE_SZS_MASK	= 0xff << 16,
// DW10
    SLI4_PARAM_MQ_PAGE_CNT_MASK	= 0xf << 0,
    SLI4_PARAM_MQ_PAGE_SZS_MASK	= 0xff << 16,
// DW12
    SLI4_PARAM_WQ_PAGE_CNT_MASK	= 0xf << 0,
    SLI4_PARAM_WQE_SZS_MASK		= 0xf << 8,
    SLI4_PARAM_WQ_PAGE_SZS_MASK	= 0xff << 16,
// DW14
    SLI4_PARAM_RQ_PAGE_CNT_MASK	= 0xf << 0,
    SLI4_PARAM_RQE_SZS_MASK		= 0xf << 8,
    SLI4_PARAM_RQ_PAGE_SZS_MASK	= 0xff << 16,
// DW15W1
    SLI4_PARAM_RQ_DB_WINDOW_MASK	= 0xf000,
// DW16
    SLI4_PARAM_FC			= 1 << 0,
    SLI4_PARAM_EXT			= 1 << 1,
    SLI4_PARAM_HDRR			= 1 << 2,
    SLI4_PARAM_SGLR			= 1 << 3,
    SLI4_PARAM_FBRR			= 1 << 4,
    SLI4_PARAM_AREG			= 1 << 5,
    SLI4_PARAM_TGT			= 1 << 6,
    SLI4_PARAM_TERP			= 1 << 7,
    SLI4_PARAM_ASSI			= 1 << 8,
    SLI4_PARAM_WCHN			= 1 << 9,
    SLI4_PARAM_TCCA			= 1 << 10,
    SLI4_PARAM_TRTY			= 1 << 11,
    SLI4_PARAM_TRIR			= 1 << 12,
    SLI4_PARAM_PHOFF		= 1 << 13,
    SLI4_PARAM_PHON			= 1 << 14,
    SLI4_PARAM_PHWQ			= 1 << 15,
    SLI4_PARAM_BOUND_4GA		= 1 << 16,
    SLI4_PARAM_RXC			= 1 << 17,
    SLI4_PARAM_HLM			= 1 << 18,
    SLI4_PARAM_IPR			= 1 << 19,
    SLI4_PARAM_RXRI			= 1 << 20,
    SLI4_PARAM_SGLC			= 1 << 21,
    SLI4_PARAM_TIMM			= 1 << 22,
    SLI4_PARAM_TSMM			= 1 << 23,
    SLI4_PARAM_OAS			= 1 << 25,
    SLI4_PARAM_LC			= 1 << 26,
    SLI4_PARAM_AGXF			= 1 << 27,
    SLI4_PARAM_LOOPBACK_MASK	= 0xf << 28,
// DW18
    SLI4_PARAM_SGL_PAGE_CNT_MASK	= 0xf << 0,
    SLI4_PARAM_SGL_PAGE_SZS_MASK	= 0xff << 8,
    SLI4_PARAM_SGL_PP_ALIGN_MASK	= 0xff << 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_get_sli4_params {
    pub hdr: sli4_rqst_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_sli4_params {
    pub hdr: sli4_rsp_hdr,
    pub dw4_protocol_type: __le32,
    pub dw5_sli: __le32,
    pub dw6_eq_page_cnt: __le32,
    pub eqe_count_mask: __le16,
    pub rsvd26: __le16,
    pub dw8_cq_page_cnt: __le32,
    pub cqe_count_mask: __le16,
    pub rsvd34: __le16,
    pub dw10_mq_page_cnt: __le32,
    pub mqe_count_mask: __le16,
    pub rsvd42: __le16,
    pub dw12_wq_page_cnt: __le32,
    pub wqe_count_mask: __le16,
    pub rsvd50: __le16,
    pub dw14_rq_page_cnt: __le32,
    pub rqe_count_mask: __le16,
    pub dw15w1_rq_db_window: __le16,
    pub dw16_loopback_scope: __le32,
    pub sge_supported_length: __le32,
    pub dw18_sgl_page_cnt: __le32,
    pub min_rq_buffer_size: __le16,
    pub rsvd75: __le16,
    pub max_rq_buffer_size: __le32,
    pub physical_xri_max: __le16,
    pub physical_rpi_max: __le16,
    pub physical_vpi_max: __le16,
    pub physical_vfi_max: __le16,
    pub rsvd88: __le32,
    pub frag_num_field_offset: __le16,
    pub frag_num_field_size: __le16,
    pub sgl_index_field_offset: __le16,
    pub sgl_index_field_size: __le16,
    pub chain_sge_initial_value_lo: __le32,
    pub chain_sge_initial_value_hi: __le32,
}

// Port Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_port_types {
    SLI4_PORT_TYPE_ETH	= 0,
    SLI4_PORT_TYPE_FC	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_get_port_name {
    pub hdr: sli4_rqst_hdr,
    pub port_type: u8,
    pub rsvd4: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_port_name {
    pub hdr: sli4_rsp_hdr,
    pub port_name: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_write_flashrom {
    pub hdr: sli4_rqst_hdr,
    pub flash_rom_access_opcode: __le32,
    pub flash_rom_access_operation_type: __le32,
    pub data_buffer_size: __le32,
    pub offset: __le32,
    pub data_buffer: [u8; 4],
}

//
// COMMON_READ_TRANSCEIVER_DATA
//
// This command reads SFF transceiver data(Format is defined
// by the SFF-8472 specification).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_read_transceiver_data {
    pub hdr: sli4_rqst_hdr,
    pub page_number: __le32,
    pub port: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_read_transceiver_data {
    pub hdr: sli4_rsp_hdr,
    pub page_number: __le32,
    pub port: __le32,
    pub page_data: [u8; 128],
    pub page_data_2: [u8; 128],
}

pub const SLI4_REQ_DESIRE_READLEN: c_uint = 0xffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_read_object {
    pub hdr: sli4_rqst_hdr,
    pub desired_read_length_dword: __le32,
    pub read_offset: __le32,
    pub object_name: [u8; 104],
    pub host_buffer_descriptor_count: __le32,
    pub host_buffer_descriptor: [sli4_bde; ],
}

pub const RSP_COM_READ_OBJ_EOF: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_read_object {
    pub hdr: sli4_rsp_hdr,
    pub actual_read_length: __le32,
    pub eof_dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rqst_write_object_flags {
    SLI4_RQ_DES_WRITE_LEN		= 0xffffff,
    SLI4_RQ_DES_WRITE_LEN_NOC	= 0x40000000,
    SLI4_RQ_DES_WRITE_LEN_EOF	= 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_write_object {
    pub hdr: sli4_rqst_hdr,
    pub desired_write_len_dword: __le32,
    pub write_offset: __le32,
    pub object_name: [u8; 104],
    pub host_buffer_descriptor_count: __le32,
    pub host_buffer_descriptor: [sli4_bde; ],
}

pub const RSP_CHANGE_STATUS: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_write_object {
    pub hdr: sli4_rsp_hdr,
    pub actual_write_length: __le32,
    pub change_status_dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_delete_object {
    pub hdr: sli4_rqst_hdr,
    pub rsvd4: __le32,
    pub rsvd5: __le32,
    pub object_name: [u8; 104],
}

pub const SLI4_RQ_OBJ_LIST_READ_LEN: c_uint = 0xffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_read_object_list {
    pub hdr: sli4_rqst_hdr,
    pub desired_read_length_dword: __le32,
    pub read_offset: __le32,
    pub object_name: [u8; 104],
    pub host_buffer_descriptor_count: __le32,
    pub host_buffer_descriptor: [sli4_bde; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rqst_set_dump_flags {
    SLI4_CMN_SET_DUMP_BUFFER_LEN	= 0xffffff,
    SLI4_CMN_SET_DUMP_FDB		= 0x20000000,
    SLI4_CMN_SET_DUMP_BLP		= 0x40000000,
    SLI4_CMN_SET_DUMP_QRY		= 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_dump_location {
    pub hdr: sli4_rqst_hdr,
    pub buffer_length_dword: __le32,
    pub buf_addr_low: __le32,
    pub buf_addr_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_set_dump_location {
    pub hdr: sli4_rsp_hdr,
    pub buffer_length_dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_dump_level {
    SLI4_DUMP_LEVEL_NONE,
    SLI4_CHIP_LEVEL_DUMP,
    SLI4_FUNC_DESC_DUMP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_dump_state {
    SLI4_DUMP_STATE_NONE,
    SLI4_CHIP_DUMP_STATE_VALID,
    SLI4_FUNC_DUMP_STATE_VALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_dump_status {
    SLI4_DUMP_READY_STATUS_NOT_READY,
    SLI4_DUMP_READY_STATUS_DD_PRESENT,
    SLI4_DUMP_READY_STATUS_FDB_PRESENT,
    SLI4_DUMP_READY_STATUS_SKIP_DUMP,
    SLI4_DUMP_READY_STATUS_FAILED = -1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_set_features {
    SLI4_SET_FEATURES_DIF_SEED			= 0x01,
    SLI4_SET_FEATURES_XRI_TIMER			= 0x03,
    SLI4_SET_FEATURES_MAX_PCIE_SPEED		= 0x04,
    SLI4_SET_FEATURES_FCTL_CHECK			= 0x05,
    SLI4_SET_FEATURES_FEC				= 0x06,
    SLI4_SET_FEATURES_PCIE_RECV_DETECT		= 0x07,
    SLI4_SET_FEATURES_DIF_MEMORY_MODE		= 0x08,
    SLI4_SET_FEATURES_DISABLE_SLI_PORT_PAUSE_STATE	= 0x09,
    SLI4_SET_FEATURES_ENABLE_PCIE_OPTIONS		= 0x0a,
    SLI4_SET_FEAT_CFG_AUTO_XFER_RDY_T10PI		= 0x0c,
    SLI4_SET_FEATURES_ENABLE_MULTI_RECEIVE_QUEUE	= 0x0d,
    SLI4_SET_FEATURES_SET_FTD_XFER_HINT		= 0x0f,
    SLI4_SET_FEATURES_SLI_PORT_HEALTH_CHECK		= 0x11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_features {
    pub hdr: sli4_rqst_hdr,
    pub feature: __le32,
    pub param_len: __le32,
    pub params: [__le32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_features_dif_seed {
    pub seed: __le16,
    pub rsvd16: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rqst_set_mrq_features {
    SLI4_RQ_MULTIRQ_ISR		 = 0x1,
    SLI4_RQ_MULTIRQ_AUTOGEN_XFER_RDY = 0x2,

    SLI4_RQ_MULTIRQ_NUM_RQS		 = 0xff,
    SLI4_RQ_MULTIRQ_RQ_SELECT	 = 0xf00,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_features_multirq {
    pub auto_gen_xfer_dword: __le32,
    pub num_rqs_dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_rqst_health_check_flags {
    SLI4_RQ_HEALTH_CHECK_ENABLE	= 0x1,
    SLI4_RQ_HEALTH_CHECK_QUERY	= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_features_health_check {
    pub health_check_dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_features_set_fdt_xfer_hint {
    pub fdt_xfer_hint: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_dmtf_exec_clp_cmd {
    pub hdr: sli4_rqst_hdr,
    pub cmd_buf_length: __le32,
    pub resp_buf_length: __le32,
    pub cmd_buf_addr_low: __le32,
    pub cmd_buf_addr_high: __le32,
    pub resp_buf_addr_low: __le32,
    pub resp_buf_addr_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_dmtf_exec_clp_cmd {
    pub hdr: sli4_rsp_hdr,
    pub rsvd4: __le32,
    pub resp_length: __le32,
    pub rsvd6: __le32,
    pub rsvd7: __le32,
    pub rsvd8: __le32,
    pub rsvd9: __le32,
    pub clp_status: __le32,
    pub clp_detailed_status: __le32,
}

pub const SLI4_PROTOCOL_FC: c_uint = 0x10;
pub const SLI4_PROTOCOL_DEFAULT: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rspource_descriptor_v1 {
    pub descriptor_type: u8,
    pub descriptor_length: u8,
    pub rsvd16: __le16,
    pub type_specific: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_pcie_desc_flags {
    SLI4_PCIE_DESC_IMM		= 0x4000,
    SLI4_PCIE_DESC_NOSV		= 0x8000,

    SLI4_PCIE_DESC_PF_NO		= 0x3ff0000,

    SLI4_PCIE_DESC_MISSN_ROLE	= 0xff,
    SLI4_PCIE_DESC_PCHG		= 0x8000000,
    SLI4_PCIE_DESC_SCHG		= 0x10000000,
    SLI4_PCIE_DESC_XCHG		= 0x20000000,
    SLI4_PCIE_DESC_XROM		= 0xc0000000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_pcie_resource_descriptor_v1 {
    pub descriptor_type: u8,
    pub descriptor_length: u8,
    pub imm_nosv_dword: __le16,
    pub pf_number_dword: __le32,
    pub rsvd3: __le32,
    pub sriov_state: u8,
    pub pf_state: u8,
    pub pf_type: u8,
    pub rsvd4: u8,
    pub number_of_vfs: __le16,
    pub rsvd5: __le16,
    pub mission_roles_dword: __le32,
    pub rsvd7: [__le32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_get_function_config {
    pub hdr: sli4_rqst_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_function_config {
    pub hdr: sli4_rsp_hdr,
    pub desc_count: __le32,
    pub desc: [__le32; 54],
}

// Link Config Descriptor for link config functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_link_config_descriptor {
    pub link_config_id: u8,
    pub rsvd1: [u8; 3],
    pub config_description: [__le32; 8],
}

pub const MAX_LINK_DES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_get_reconfig_link_info {
    pub hdr: sli4_rqst_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_get_reconfig_link_info {
    pub hdr: sli4_rsp_hdr,
    pub active_link_config_id: u8,
    pub rsvd17: u8,
    pub next_link_config_id: u8,
    pub rsvd19: u8,
    pub link_configuration_descriptor_count: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_set_reconfig_link_flags {
    SLI4_SET_RECONFIG_LINKID_NEXT	= 0xff,
    SLI4_SET_RECONFIG_LINKID_FD	= 1u << 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_cmn_set_reconfig_link_id {
    pub hdr: sli4_rqst_hdr,
    pub dw4_flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_cmn_set_reconfig_link_id {
    pub hdr: sli4_rsp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_lowlevel_set_watchdog {
    pub hdr: sli4_rqst_hdr,
    pub watchdog_timeout: __le16,
    pub rsvd18: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_lowlevel_set_watchdog {
    pub hdr: sli4_rsp_hdr,
    pub rsvd: __le32,
}

// FC opcode (OPC) values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_fc_opcodes {
    SLI4_OPC_WQ_CREATE		= 0x1,
    SLI4_OPC_WQ_DESTROY		= 0x2,
    SLI4_OPC_POST_SGL_PAGES		= 0x3,
    SLI4_OPC_RQ_CREATE		= 0x5,
    SLI4_OPC_RQ_DESTROY		= 0x6,
    SLI4_OPC_READ_FCF_TABLE		= 0x8,
    SLI4_OPC_POST_HDR_TEMPLATES	= 0xb,
    SLI4_OPC_REDISCOVER_FCF		= 0x10,
}

// Use the default CQ associated with the WQ
pub const SLI4_CQ_DEFAULT: c_uint = 0xffff;
//
// POST_SGL_PAGES
//
// Register the scatter gather list (SGL) memory and
// associate it with an XRI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_post_sgl_pages {
    pub hdr: sli4_rqst_hdr,
    pub xri_start: __le16,
    pub xri_count: __le16,
    pub page0_low: __le32,
    pub page0_high: __le32,
    pub page1_low: __le32,
    pub page1_high: __le32,
    pub page_set: [}; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rsp_post_sgl_pages {
    pub hdr: sli4_rsp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_rqst_post_hdr_templates {
    pub hdr: sli4_rqst_hdr,
    pub rpi_offset: __le16,
    pub page_count: __le16,
    pub page_descriptor: [sli4_dmaaddr; ],
}

pub const SLI4_HDR_TEMPLATE_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_io_flags {
// The XRI associated with this IO is already active
    SLI4_IO_CONTINUATION		= 1 << 0,
// Automatically generate a good RSP frame
    SLI4_IO_AUTO_GOOD_RESPONSE	= 1 << 1,
    SLI4_IO_NO_ABORT		= 1 << 2,
// Set the DNRX bit because no auto xref rdy buffer is posted
    SLI4_IO_DNRX			= 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_callback {
    SLI4_CB_LINK,
    SLI4_CB_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_link_status {
    SLI4_LINK_STATUS_UP,
    SLI4_LINK_STATUS_DOWN,
    SLI4_LINK_STATUS_NO_ALPA,
    SLI4_LINK_STATUS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_link_topology {
    SLI4_LINK_TOPO_NON_FC_AL = 1,
    SLI4_LINK_TOPO_FC_AL,
    SLI4_LINK_TOPO_LOOPBACK_INTERNAL,
    SLI4_LINK_TOPO_LOOPBACK_EXTERNAL,
    SLI4_LINK_TOPO_NONE,
    SLI4_LINK_TOPO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_link_medium {
    SLI4_LINK_MEDIUM_ETHERNET,
    SLI4_LINK_MEDIUM_FC,
    SLI4_LINK_MEDIUM_MAX,
}

// Driver specific structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_queue {
// Common to all queue types
    pub dma: efc_dma,
    pub register: *mut *mut spinlock_t lock; / Lock to protect the doorbell,
// writes and queue reads
//
    pub /: *mut *mut u32 index; / current host entry index,
    pub /: *mut *mut u16 size; / entry size,
    pub /: *mut *mut u16 length; / number of entries,
    pub /: *mut *mut u16 n_posted; / number entries posted for CQ, EQ,
    pub /: *mut *mut u16 id; / Port assigned xQ_ID,
    pub /: *mut *mut u8 type; / queue type ie EQ, CQ, ...,
    pub /: *mut *mut *mut void __iomem db_regaddr; / register address for the doorbell,
    pub toggle: *mut *mut u16 phase; / For if_type = 6, this value,
// for each iteration of the queue,
// a queue entry is valid when a cqe
// valid bit matches this value
//
    pub /: *mut *mut u32 proc_limit; / limit CQE processed per iteration,
    pub /: *mut *mut u32 posted_limit; / CQE/EQE process before ring db,
    pub max_num_processed: u32,
    pub max_process_time: u64,
    pub /: *mut *mut u32 r_idx; / "read" index (MQ only),
    pub flag: u32,
    pub u: },
}

// Parameters used to populate WQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli_bls_params {
    pub s_id: u32,
    pub d_id: u32,
    pub ox_id: u16,
    pub rx_id: u16,
    pub rpi: u32,
    pub vpi: u32,
    pub rpi_registered: bool,
    pub payload: [u8; 12],
    pub xri: u16,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli_els_params {
    pub s_id: u32,
    pub d_id: u32,
    pub ox_id: u16,
    pub rpi: u32,
    pub vpi: u32,
    pub rpi_registered: bool,
    pub xmit_len: u32,
    pub rsp_len: u32,
    pub timeout: u8,
    pub cmd: u8,
    pub xri: u16,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli_ct_params {
    pub r_ctl: u8,
    pub type: u8,
    pub df_ctl: u8,
    pub timeout: u8,
    pub ox_id: u16,
    pub d_id: u32,
    pub rpi: u32,
    pub vpi: u32,
    pub rpi_registered: bool,
    pub xmit_len: u32,
    pub rsp_len: u32,
    pub xri: u16,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli_fcp_tgt_params {
    pub s_id: u32,
    pub d_id: u32,
    pub rpi: u32,
    pub vpi: u32,
    pub offset: u32,
    pub ox_id: u16,
    pub flags: u16,
    pub cs_ctl: u8,
    pub timeout: u8,
    pub app_id: u32,
    pub xmit_len: u32,
    pub xri: u16,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_link_event {
    pub status: sli4_link_status,
    pub topology: sli4_link_topology,
    pub medium: sli4_link_medium,
    pub speed: u32,
    pub loop_map: *mut u8,
    pub fc_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sli4_resource {
    SLI4_RSRC_VFI,
    SLI4_RSRC_VPI,
    SLI4_RSRC_RPI,
    SLI4_RSRC_XRI,
    SLI4_RSRC_FCFI,
    SLI4_RSRC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_extent {
    pub number: u32,
    pub size: u32,
    pub n_alloc: u32,
    pub base: *mut u32,
    pub use_map: *mut c_ulong,
    pub map_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_queue_info {
    pub max_qcount: [u16; SLI4_QTYPE_MAX],
    pub max_qentries: [u32; SLI4_QTYPE_MAX],
    pub count_mask: [u16; SLI4_QTYPE_MAX],
    pub count_method: [u16; SLI4_QTYPE_MAX],
    pub qpage_count: [u32; SLI4_QTYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_params {
    pub has_extents: u8,
    pub auto_reg: u8,
    pub auto_xfer_rdy: u8,
    pub hdr_template_req: u8,
    pub perf_hint: u8,
    pub perf_wq_id_association: u8,
    pub cq_create_version: u8,
    pub mq_create_version: u8,
    pub high_login_mode: u8,
    pub sgl_pre_registered: u8,
    pub sgl_pre_reg_required: u8,
    pub t10_dif_inline_capable: u8,
    pub t10_dif_separate_capable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4 {
    pub os: *mut c_void,
    pub pci: *mut pci_dev,
    pub reg: [*mut void __iomem; PCI_STD_NUM_BARS],
    pub sli_rev: u32,
    pub sli_family: u32,
    pub if_type: u32,
    pub asic_type: u16,
    pub asic_rev: u16,
    pub e_d_tov: u16,
    pub r_a_tov: u16,
    pub qinfo: sli4_queue_info,
    pub link_module_type: u16,
    pub rq_batch: u8,
    pub port_number: u8,
    pub port_name: [c_char; 2],
    pub rq_min_buf_size: u16,
    pub rq_max_buf_size: u32,
    pub topology: u8,
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
    pub fw_rev: [u32; 2],
    pub fw_name: [u8; 2][16],
    pub ipl_name: [c_char; 16],
    pub hw_rev: [u32; 3],
    pub modeldesc: [c_char; 64],
    pub bios_version_string: [c_char; 32],
    pub wqe_size: u32,
    pub vpd_length: u32,
//
// Tracks the port resources using extents metaphor. For
// devices that don't implement extents (i.e.
// has_extents == FALSE), the code models each resource as
// a single large extent.
//
    pub ext: [sli4_extent; SLI4_RSRC_MAX],
    pub features: u32,
    pub params: sli4_params,
    pub sge_supported_length: u32,
    pub sgl_page_sizes: u32,
    pub max_sgl_pages: u32,
//
// Callback functions
//
    pub event): *mut *mut *mut int (link)(void ctx, void,
    pub link_arg: *mut c_void,
    pub bmbx: efc_dma,
// Save pointer to physical memory descriptor for non-embedded
// SLI_CONFIG commands for BMBX dumping purposes
//
    pub bmbx_non_emb_pmd: *mut efc_dma,
    pub vpd_data: efc_dma,
}

//
// Get / set parameter functions
//
extern "C" {
    pub fn readl(SLI4_PORT_STATUS_REGOFF: sli->reg[0] +) -> return;
}
extern "C" {
    pub fn readl(SLI4_PORT_ERROR1: sli->reg[0] +) -> return;
}
extern "C" {
    pub fn readl(SLI4_PORT_ERROR2: sli->reg[0] +) -> return;
}
// len_hdr = *len_data = 0;
// len_hdr  = rcqe->hdpl_byte & SLI4_RACQE_HDPL;
// len_data = le16_to_cpu(rcqe->data_placement_length);
//
// Function prototypes
//
extern "C" {
    pub fn sli_cqe_mq(sli4: *mut sli4, buf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sli_cqe_async(sli4: *mut sli4, buf: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sli_calc_max_qentries(sli4: *mut sli4);
}
extern "C" {
    pub fn sli_init(sli4: *mut sli4) -> c_int;
}
extern "C" {
    pub fn sli_reset(sli4: *mut sli4) -> c_int;
}
extern "C" {
    pub fn sli_fw_reset(sli4: *mut sli4) -> c_int;
}
extern "C" {
    pub fn sli_teardown(sli4: *mut sli4);
}
extern "C" {
    pub fn sli_raise_ue(sli4: *mut sli4, dump: u8) -> c_int;
}
extern "C" {
    pub fn sli_dump_is_ready(sli4: *mut sli4) -> c_int;
}
extern "C" {
    pub fn sli_reset_required(sli4: *mut sli4) -> bool;
}
extern "C" {
    pub fn sli_fw_ready(sli4: *mut sli4) -> bool;
}
extern "C" {
    pub fn sli_fc_response_length(sli4: *mut sli4, cqe: *mut u8) -> u32;
}
extern "C" {
    pub fn sli_fc_io_length(sli4: *mut sli4, cqe: *mut u8) -> u32;
}
extern "C" {
    pub fn sli_fc_els_did(sli4: *mut sli4, cqe: *mut u8, d_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn sli_fc_ext_status(sli4: *mut sli4, cqe: *mut u8) -> u32;
}
extern "C" {
    pub fn sli_fc_get_rpi_requirements(sli4: *mut sli4, n_rpi: u32) -> u32;
}
