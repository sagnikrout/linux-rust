//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_target.h
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
// Copyright (C) 2004 - 2010 Vladislav Bolkhovitin <vst@vlnb.net>
// Copyright (C) 2004 - 2005 Leonid Stoljar
// Copyright (C) 2006 Nathaniel Clark <nate@misrule.us>
// Copyright (C) 2007 - 2010 ID7 Ltd.
//
// Forward port and refactoring to modern qla2xxx and target/configfs
//
// Copyright (C) 2010-2011 Nicholas A. Bellinger <nab@kernel.org>
//
// Additional file for the target driver support.
//
// This is the global def file that is useful for including from the
// target portion.
//

//
// Must be changed on any change in any initiator visible interfaces or
// data in the target add-on
//
pub const QLA2XXX_TARGET_MAGIC: c_int = 269;
//
// Must be changed on any change in any target visible interfaces or
// data in the initiator
//
pub const QLA2XXX_INITIATOR_MAGIC: c_int = 57222;

pub const QLA2XXX_INI_MODE_EXCLUSIVE: c_int = 0;
pub const QLA2XXX_INI_MODE_DISABLED: c_int = 1;
pub const QLA2XXX_INI_MODE_ENABLED: c_int = 2;
pub const QLA2XXX_INI_MODE_DUAL: c_int = 3;
pub const QLA2XXX_COMMAND_COUNT_INIT: c_int = 250;
pub const QLA2XXX_IMMED_NOTIFY_COUNT_INIT: c_int = 250;
//
// Used to mark which completion handles (for RIO Status's) are for CTIO's
// vs. regular (non-target) info. This is checked for in
// qla2x00_process_response_queue() to see if a handle coming back in a
// multi-complete should come to the tgt driver or be handled there by qla2xxx
//

// Used to mark CTIO as intermediate

pub const QLA_TGT_NULL_HANDLE: c_int = 0;
pub const QLA_TGT_HANDLE_MASK: c_uint = 0xF0000000;
pub const QLA_QPID_HANDLE_MASK: c_uint = 0x00FF0000 /* qpair id mask */;
pub const QLA_CMD_HANDLE_MASK: c_uint = 0x0000FFFF;

pub const QLA_QPID_HANDLE_SHIFT: c_int = 16;

//
// ISP target entries - Flags bit definitions.
//
pub const OF_SS_MODE_0: c_int = 0;
pub const OF_SS_MODE_1: c_int = 1;
pub const OF_SS_MODE_2: c_int = 2;
pub const OF_SS_MODE_3: c_int = 3;

// (data from target to initiator)

// (data from initiator to target)

pub const QLA_TGT_DATASEGS_PER_CMD32: c_int = 3;
pub const QLA_TGT_DATASEGS_PER_CONT32: c_int = 7;

pub const QLA_TGT_DATASEGS_PER_CMD64: c_int = 2;
pub const QLA_TGT_DATASEGS_PER_CONT64: c_int = 5;

pub const QLA_TGT_DATASEGS_PER_CMD_24XX: c_int = 1;
pub const QLA_TGT_DATASEGS_PER_CONT_24XX: c_int = 5;

pub const NOTIFY_ACK_TYPE: c_uint = 0x0E	  /* Notify acknowledge entry. */;
//
// ISP queue -	notify acknowledge entry structure definition.
// This is sent to the ISP from the target driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nack_to_isp {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 sys_define_2; / System defined.,
    pub target: target_id_t,
    pub target_id: u8,
    pub reserved_1: u8,
    pub flags: __le16,
    pub resp_code: __le16,
    pub status: __le16,
    pub task_flags: __le16,
    pub seq_id: __le16,
    pub srr_rx_id: __le16,
    pub srr_rel_offs: __le32,
    pub srr_ui: __le16,
    pub srr_flags: __le16,
    pub srr_reject_code: __le16,
    pub srr_reject_vendor_uniq: u8,
    pub srr_reject_code_expl: u8,
    pub reserved_2: [u8; 24],
    pub isp2x: },
    pub handle: u32,
    pub nport_handle: __le16,
    pub reserved_1: u16,
    pub flags: __le16,
    pub srr_rx_id: __le16,
    pub status: __le16,
    pub status_subcode: u8,
    pub fw_handle: u8,
    pub exchange_address: __le32,
    pub srr_rel_offs: __le32,
    pub srr_ui: __le16,
    pub srr_flags: __le16,
    pub reserved_4: [u8; 19],
    pub vp_index: u8,
    pub srr_reject_vendor_uniq: u8,
    pub srr_reject_code_expl: u8,
    pub srr_reject_code: u8,
    pub reserved_5: [u8; 5],
    pub isp24: },
    pub u: },
    pub reserved: [u8; 2],
    pub ox_id: __le16,
    pub __packed: },

pub const NOTIFY_ACK_SRR_FLAGS_ACCEPT: c_int = 0;
pub const NOTIFY_ACK_SRR_FLAGS_REJECT: c_int = 1;
pub const NOTIFY_ACK_SRR_REJECT_REASON_UNABLE_TO_PERFORM: c_uint = 0x9;
pub const NOTIFY_ACK_SRR_FLAGS_REJECT_EXPL_NO_EXPL: c_int = 0;
pub const NOTIFY_ACK_SRR_FLAGS_REJECT_EXPL_INVALID_OX_ID_RX_ID: c_uint = 0x17;
pub const NOTIFY_ACK_SRR_FLAGS_REJECT_EXPL_UNABLE_TO_SUPPLY_DATA: c_uint = 0x2a;
pub const NOTIFY_ACK_SUCCESS: c_uint = 0x01;

pub const ACCEPT_TGT_IO_TYPE: c_uint = 0x16 /* Accept target I/O entry. */;

pub const CONTINUE_TGT_IO_TYPE: c_uint = 0x17;
//
// ISP queue -	Continue Target I/O (CTIO) entry for status mode 0 structure.
// This structure is sent to the ISP 2xxx from target driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_to_2xxx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System defined handle,
    pub target: target_id_t,
    pub rx_id: __le16,
    pub flags: __le16,
    pub status: __le16,
    pub /: *mut *mut __le16 timeout; / 0 = 30 seconds, 0xFFFF = disable,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub relative_offset: __le32,
    pub residual: __le32,
    pub reserved_1: [__le16; 3],
    pub scsi_status: __le16,
    pub transfer_length: __le32,
    pub dsd: [dsd32; 3],
    pub __packed: },
pub const ATIO_PATH_INVALID: c_uint = 0x07;
pub const ATIO_CANT_PROV_CAP: c_uint = 0x16;
pub const ATIO_CDB_VALID: c_uint = 0x3D;

pub const CTIO_A64_TYPE: c_uint = 0x1F;
pub const CTIO_SUCCESS: c_uint = 0x01;
pub const CTIO_ABORTED: c_uint = 0x02;
pub const CTIO_INVALID_RX_ID: c_uint = 0x08;
pub const CTIO_TIMEOUT: c_uint = 0x0B;
pub const CTIO_DIF_ERROR: c_uint = 0x0C     /* DIF error detected  */;
pub const CTIO_LIP_RESET: c_uint = 0x0E;
pub const CTIO_TARGET_RESET: c_uint = 0x17;
pub const CTIO_PORT_UNAVAILABLE: c_uint = 0x28;
pub const CTIO_PORT_LOGGED_OUT: c_uint = 0x29;
pub const CTIO_PORT_CONF_CHANGED: c_uint = 0x2A;
pub const CTIO_SRR_RECEIVED: c_uint = 0x45;
pub const CTIO_FAST_AUTH_ERR: c_uint = 0x63;
pub const CTIO_FAST_INCOMP_PAD_LEN: c_uint = 0x65;
pub const CTIO_FAST_INVALID_REQ: c_uint = 0x66;
pub const CTIO_FAST_SPI_ERR: c_uint = 0x67;

pub const CTIO_RET_TYPE: c_uint = 0x17		/* CTIO return entry */;
pub const ATIO_TYPE7: c_uint = 0x06 /* Accept target I/O entry for 24xx */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_hdr {
    pub r_ctl: u8,
    pub d_id: be_id_t,
    pub cs_ctl: u8,
    pub s_id: be_id_t,
    pub type: u8,
    pub f_ctl: [u8; 3],
    pub seq_id: u8,
    pub df_ctl: u8,
    pub seq_cnt: u16,
    pub ox_id: __be16,
    pub rx_id: u16,
    pub parameter: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_hdr_le {
    pub d_id: le_id_t,
    pub r_ctl: u8,
    pub s_id: le_id_t,
    pub cs_ctl: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub parameter: __le32,
}

pub const R_CTL_BASIC_LINK_SERV: c_uint = 0x80;
pub const R_CTL_B_ACC: c_uint = 0x4;
pub const R_CTL_B_RJT: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atio7_fcp_cmnd {
    pub lun: u64,
    pub cmnd_ref: u8,
    pub task_attr:3: u8,
    pub reserved:5: u8,
    pub task_mgmt_flags: u8,
pub const FCP_CMND_TASK_MGMT_CLEAR_ACA: c_int = 6;
pub const FCP_CMND_TASK_MGMT_TARGET_RESET: c_int = 5;
pub const FCP_CMND_TASK_MGMT_LU_RESET: c_int = 4;
pub const FCP_CMND_TASK_MGMT_CLEAR_TASK_SET: c_int = 2;
pub const FCP_CMND_TASK_MGMT_ABORT_TASK_SET: c_int = 1;
    pub wrdata:1: u8,
    pub rddata:1: u8,
    pub add_cdb_len:6: u8,
    pub cdb: [u8; 16],
//
// add_cdb is optional and can absent from struct atio7_fcp_cmnd. Size 4
// only to make sizeof(struct atio7_fcp_cmnd) be as expected by
// BUILD_BUG_ON in qlt_init().
//
    pub add_cdb: [u8; 4],
// __le32	data_length;
    pub __packed: },
//
// ISP queue -	Accept Target I/O (ATIO) type entry IOCB structure.
// This is sent from the ISP to the target driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atio_from_isp {
    pub entry_hdr: __le16,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 sys_define_2; / System defined.,
    pub target: target_id_t,
    pub rx_id: __le16,
    pub flags: __le16,
    pub status: __le16,
    pub command_ref: u8,
    pub task_codes: u8,
    pub task_flags: u8,
    pub execution_codes: u8,
    pub cdb: [u8; MAX_CMDSZ],
    pub data_length: __le32,
    pub lun: __le16,
    pub /: *mut *mut uint8_t initiator_port_name[WWN_SIZE]; / on qla23xx,
    pub reserved_32: [__le16; 6],
    pub ox_id: __le16,
    pub isp2x: },
    pub entry_hdr: __le16,
    pub fcp_cmnd_len_low: u8,
    pub fcp_cmnd_len_high:4: u8,
    pub attr:4: u8,
    pub exchange_addr: __le32,
pub const ATIO_EXCHANGE_ADDRESS_UNKNOWN: c_uint = 0xFFFFFFFF;
    pub fcp_hdr: fcp_hdr,
    pub fcp_cmnd: atio7_fcp_cmnd,
    pub isp24: },
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub attr_n_length: __le16,
pub const FCP_CMD_LENGTH_MASK: c_uint = 0x0fff;
pub const FCP_CMD_LENGTH_MIN: c_uint = 0x38;
    pub data: [u8; 56],
    pub signature: __le32,
pub const ATIO_PROCESSED: c_uint = 0xDEADDEAD		/* Signature */;
    pub raw: },
    pub u: },
    pub __packed: },
    pub 1: return,
    pub 0: return,
// adjust corrupted atio so we won't trip over the same entry again.
    pub cpu_to_le16(FCP_CMD_LENGTH_MIN): atio->u.raw.attr_n_length =,
    pub 0: atio->u.isp24.fcp_cmnd.add_cdb_len =,
    pub atio->u.isp24.fcp_cmnd.add_cdb_len: int len =,
    pub 4]): *mut *mut return get_unaligned_be32(&atio->u.isp24.fcp_cmnd.add_cdb[len,
pub const CTIO_TYPE7: c_uint = 0x12 /* Continue target I/O entry (for 24xx) */;
//
// ISP queue -	Continue Target I/O (ATIO) type 7 entry (for 24xx) structure.
// This structure is sent to the ISP 24xx from the target driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio7_to_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System defined handle,
    pub nport_handle: __le16,
pub const CTIO7_NHANDLE_UNRECOGNIZED: c_uint = 0xFFFF;
    pub timeout: __le16,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub vp_index: u8,
    pub add_flags: u8,
    pub initiator_id: le_id_t,
    pub reserved: u8,
    pub exchange_addr: __le32,
    pub reserved1: __le16,
    pub flags: __le16,
    pub residual: __le32,
    pub rsvd1: u8,
    pub edif_flags: u8,

    pub rsvd2: u16,
}

//
// ISP queue - CTIO type 7 from ISP 24xx to target driver
// returned entry structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio7_from_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System defined handle,
    pub status: __le16,
    pub timeout: __le16,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub vp_index: u8,
    pub reserved1: [u8; 5],
    pub exchange_address: __le32,
    pub edif_sa_index: __le16,
    pub flags: __le16,
    pub residual: __le32,
    pub ox_id: __le16,
    pub reserved3: __le16,
    pub relative_offset: __le32,
    pub reserved4: [u8; 24],
    pub __packed: },
// CTIO7 flags values

pub const CTIO7_FLAGS_STATUS_MODE_0: c_int = 0;

pub const ELS_PLOGI: c_uint = 0x3;
pub const ELS_FLOGI: c_uint = 0x4;
pub const ELS_LOGO: c_uint = 0x5;
pub const ELS_PRLI: c_uint = 0x20;
pub const ELS_PRLO: c_uint = 0x21;
pub const ELS_TPRLO: c_uint = 0x24;
pub const ELS_PDISC: c_uint = 0x50;
pub const ELS_ADISC: c_uint = 0x52;
//
// CTIO Type CRC_2 IOCB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_crc2_to_fw {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
pub const CTIO_CRC2: c_uint = 0x7A;
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub vp_index: u8,
    pub /: *mut *mut uint8_t add_flags; / additional flags,

    pub /: *mut *mut le_id_t initiator_id; / initiator ID,
    pub reserved1: u8,
    pub /: *mut *mut __le32 exchange_addr; / rcv exchange address,
    pub reserved2: __le16,
    pub /: *mut *mut __le16 flags; / refer to CTIO7 flags values,
    pub residual: __le32,
    pub ox_id: __le16,
    pub scsi_status: __le16,
    pub relative_offset: __le32,
    pub reserved5: __le32,
    pub /: *mut *mut __le32 transfer_length; / total fc transfer length,
    pub reserved6: __le32,
    pub /: *mut *mut __le64 crc_context_address __packed; / Data segment address.,
    pub /: *mut *mut __le16 crc_context_len; / Data segment length.,
    pub /: *mut *mut __le16 reserved_1; / MUST be set to 0.,
}

// CTIO Type CRC_x Status IOCB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctio_crc_from_fw {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub status: __le16,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub reserved1: __le32,
    pub state_flags: __le16,

    pub /: *mut *mut __le32 exchange_address; / rcv exchange address,
    pub reserved2: __le16,
    pub flags: __le16,
    pub resid_xfer_length: __le32,
    pub ox_id: __le16,
    pub reserved3: [u8; 12],
    pub /: *mut *mut __le16 runt_guard; / reported runt blk guard,
    pub actual_dif: [u8; 8],
    pub expected_dif: [u8; 8],
    pub __packed: },
//
// ISP queue - ABTS received/response entries structure definition for 24xx.
//
pub const ABTS_RECV_24XX: c_uint = 0x54 /* ABTS received (for 24xx) */;
pub const ABTS_RESP_24XX: c_uint = 0x55 /* ABTS responce (for 24xx) */;
//
// ISP queue -	ABTS received IOCB entry structure definition for 24xx.
// The ABTS BLS received from the wire is sent to the
// target driver by the ISP 24xx.
// The IOCB is placed on the response queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abts_recv_from_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved_1: [u8; 6],
    pub nport_handle: __le16,
    pub reserved_2: [u8; 2],
    pub vp_index: u8,
    pub reserved_3:4: u8,
    pub sof_type:4: u8,
    pub exchange_address: __le32,
    pub fcp_hdr_le: fcp_hdr_le,
    pub reserved_4: [u8; 16],
    pub exchange_addr_to_abort: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ba_acc_le {
    pub reserved: __le16,
    pub seq_id_last: u8,
    pub seq_id_valid: u8,
pub const SEQ_ID_VALID: c_uint = 0x80;
pub const SEQ_ID_INVALID: c_uint = 0x00;
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub high_seq_cnt: __le16,
    pub low_seq_cnt: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ba_rjt_le {
    pub vendor_uniq: u8,
    pub reason_expl: u8,
    pub reason_code: u8,
pub const BA_RJT_REASON_CODE_INVALID_COMMAND: c_uint = 0x1;
pub const BA_RJT_REASON_CODE_UNABLE_TO_PERFORM: c_uint = 0x9;
    pub reserved: u8,
    pub __packed: },
//
// ISP queue -	ABTS Response IOCB entry structure definition for 24xx.
// The ABTS response to the ABTS received is sent by the
// target driver to the ISP 24xx.
// The IOCB is placed on the request queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abts_resp_to_24xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub handle: u32,
    pub reserved_1: __le16,
    pub nport_handle: __le16,
    pub control_flags: __le16,

    pub vp_index: u8,
    pub reserved_3:4: u8,
    pub sof_type:4: u8,
    pub exchange_address: __le32,
    pub fcp_hdr_le: fcp_hdr_le,
    pub ba_acct: ba_acc_le,
    pub ba_rjt: ba_rjt_le,
    pub payload: } __packed,
    pub reserved_4: __le32,
    pub exchange_addr_to_abort: __le32,
    pub __packed: },
//
// ISP queue -	ABTS Response IOCB from ISP24xx Firmware entry structure.
// The ABTS response with completion status to the ABTS response
// (sent by the target driver to the ISP 24xx) is sent by the
// ISP24xx firmware to the target driver.
// The IOCB is placed on the response queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abts_resp_from_24xx_fw {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub handle: u32,
    pub compl_status: __le16,
pub const ABTS_RESP_COMPL_SUCCESS: c_int = 0;
pub const ABTS_RESP_COMPL_SUBCODE_ERROR: c_uint = 0x31;
    pub nport_handle: __le16,
    pub reserved_1: __le16,
    pub reserved_2: u8,
    pub reserved_3:4: u8,
    pub sof_type:4: u8,
    pub exchange_address: __le32,
    pub fcp_hdr_le: fcp_hdr_le,
    pub reserved_4: [u8; 8],
    pub error_subcode1: __le32,
pub const ABTS_RESP_SUBCODE_ERR_ABORTED_EXCH_NOT_TERM: c_uint = 0x1E;
    pub error_subcode2: __le32,
    pub exchange_addr_to_abort: __le32,
    pub __packed: },
// \
// Type Definitions used by initiator & target halves
    pub qla_tgt_mgmt_cmd: struct,
    pub fc_port: struct,
    pub qla_tgt_cmd: struct,
//
// This structure provides a template of function calls that the
// target driver (from within qla_target.c) can issue to the
// target module (tcm_qla2xxx).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_func_tmpl {
    pub uint64_t): *mut *mut *mut *mut qla_tgt_cmd (find_cmd_by_tag)(fc_port ,,
    pub int): *mut *mut unsigned char , uint32_t, int, int,,
    pub ): *mut *mut void (handle_data)(struct qla_tgt_cmd,
    pub ): *mut *mut *mut qla_tgt_cmd (get_cmd)(fc_port,
    pub cmd): *mut *mut int (get_cmd_ref)(struct qla_tgt_cmd,
    pub cmd): *mut *mut void (put_cmd_ref)(struct qla_tgt_cmd,
    pub ): *mut *mut void (rel_cmd)(struct qla_tgt_cmd,
    pub ): *mut *mut void (free_cmd)(struct qla_tgt_cmd,
    pub ): *mut *mut void (free_mcmd)(struct qla_tgt_mgmt_cmd,
    pub ): *mut *mut void (free_session)(struct fc_port,
    pub ): *mut fc_port,
    pub bool): *mut *mut *mut void (update_sess)(struct fc_port , port_id_t, uint16_t,,
    pub uint16_t): const,
    pub be_id_t): const,
    pub ): *mut *mut void (clear_nacl_from_fcport_map)(struct fc_port,
    pub ): *mut *mut void (put_sess)(struct fc_port,
    pub ): *mut *mut void (shutdown_sess)(struct fc_port,
    pub pfw_prot_opts): *mut *mut *mut int (get_dif_tags)(struct qla_tgt_cmd cmd, uint16_t,
    pub tag): *mut *mut int (chk_dif_tags)(uint32_t,
    pub ): *mut *mut void (add_target)(struct scsi_qla_host,
    pub ): *mut *mut void (remove_target)(struct scsi_qla_host,
}

extern "C" {
    pub fn qla2x00_wait_for_hba_online(: *mut scsi_qla_host) -> c_int;
}

// Immediate notify status constants
pub const IMM_NTFY_LIP_RESET: c_uint = 0x000E;
pub const IMM_NTFY_LIP_LINK_REINIT: c_uint = 0x000F;
pub const IMM_NTFY_IOCB_OVERFLOW: c_uint = 0x0016;
pub const IMM_NTFY_ABORT_TASK: c_uint = 0x0020;
pub const IMM_NTFY_PORT_LOGOUT: c_uint = 0x0029;
pub const IMM_NTFY_PORT_CONFIG: c_uint = 0x002A;
pub const IMM_NTFY_GLBL_TPRLO: c_uint = 0x002D;
pub const IMM_NTFY_GLBL_LOGO: c_uint = 0x002E;
pub const IMM_NTFY_RESOURCE: c_uint = 0x0034;
pub const IMM_NTFY_MSG_RX: c_uint = 0x0036;
pub const IMM_NTFY_SRR: c_uint = 0x0045;
pub const IMM_NTFY_ELS: c_uint = 0x0046;
// Immediate notify task flags
pub const IMM_NTFY_TASK_MGMT_SHIFT: c_int = 8;
pub const QLA_TGT_CLEAR_ACA: c_uint = 0x40;
pub const QLA_TGT_TARGET_RESET: c_uint = 0x20;
pub const QLA_TGT_LUN_RESET: c_uint = 0x10;
pub const QLA_TGT_CLEAR_TS: c_uint = 0x04;
pub const QLA_TGT_ABORT_TS: c_uint = 0x02;
pub const QLA_TGT_ABORT_ALL_SESS: c_uint = 0xFFFF;
pub const QLA_TGT_ABORT_ALL: c_uint = 0xFFFE;
pub const QLA_TGT_NEXUS_LOSS_SESS: c_uint = 0xFFFD;
pub const QLA_TGT_NEXUS_LOSS: c_uint = 0xFFFC;
pub const QLA_TGT_ABTS: c_uint = 0xFFFB;
pub const QLA_TGT_2G_ABORT_TASK: c_uint = 0xFFFA;
// Notify Acknowledge flags

// Command's states

// ATIO task_codes field
pub const ATIO_SIMPLE_QUEUE: c_int = 0;
pub const ATIO_HEAD_OF_QUEUE: c_int = 1;
pub const ATIO_ORDERED_QUEUE: c_int = 2;
pub const ATIO_ACA_QUEUE: c_int = 4;
pub const ATIO_UNTAGGED: c_int = 5;
// TM failed response codes, see FCP (9.4.11 FCP_RSP_INFO)
pub const FC_TM_SUCCESS: c_int = 0;
pub const FC_TM_BAD_FCP_DATA: c_int = 1;
pub const FC_TM_BAD_CMD: c_int = 2;
pub const FC_TM_FCP_DATA_MISMATCH: c_int = 3;
pub const FC_TM_REJECT: c_int = 4;
pub const FC_TM_FAILED: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_port_24xx_data {
    pub port_name: [u8; WWN_SIZE],
    pub loop_id: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_qpair_hint {
    pub hint_elem: list_head,
    pub qpair: *mut qla_qpair,
    pub cpuid: u16,
    pub cmd_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt {
    pub vha: *mut scsi_qla_host,
    pub ha: *mut qla_hw_data,
    pub lun_qpair_map: btree_head64,
    pub qphints: *mut qla_qpair_hint,
//
// To sync between IRQ handlers and qlt_target_release(). Needed,
// because req_pkt() can drop/reaquire HW lock inside. Protected by
// HW lock.
//
    pub atio_irq_cmd_count: c_int,
    pub sg_tablesize: c_int,
// Target's flags, serialized by pha->hardware_lock
    pub link_reinit_iocb_pending:1: c_uint,
//
// Protected by tgt_mutex AND hardware_lock for writing and tgt_mutex
// OR hardware_lock for reading.
//
    pub /: *mut *mut int tgt_stop; / the target mode driver is being stopped,
    pub /: *mut *mut int tgt_stopped; / the target mode driver has been stopped,
// Count of sessions refering qla_tgt. Protected by hardware_lock.
    pub sess_count: c_int,
    pub sess_work_lock: spinlock_t,
    pub sess_works_list: list_head,
    pub sess_work: work_struct,
    pub link_reinit_iocb: imm_ntfy_from_isp,
    pub waitQ: wait_queue_head_t,
    pub notify_ack_expected: c_int,
    pub abts_resp_expected: c_int,
    pub modify_lun_expected: c_int,
    pub srr_lock: spinlock_t,
    pub srr_list: list_head,
    pub srr_work: work_struct,
    pub tgt_global_resets_count: core::sync::atomic::AtomicI32,
    pub tgt_list_entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_sess_op {
    pub vha: *mut scsi_qla_host,
    pub chip_reset: u32,
    pub work: work_struct,
    pub cmd_list: list_head,
    pub aborted: bool,
    pub rsp: *mut rsp_que,
    pub atio: atio_from_isp,
// DO NOT ADD ANYTHING ELSE HERE - atio must be last member
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_flags {
    TRC_NEW_CMD = BIT_0,
    TRC_DO_WORK = BIT_1,
    TRC_DO_WORK_ERR = BIT_2,
    TRC_XFR_RDY = BIT_3,
    TRC_XMIT_DATA = BIT_4,
    TRC_XMIT_STATUS = BIT_5,
    TRC_SRR_RSP =  BIT_6,
    TRC_SRR_XRDY = BIT_7,
    TRC_SRR_TERM = BIT_8,
    TRC_SRR_CTIO = BIT_9,
    TRC_FLUSH = BIT_10,
    TRC_CTIO_ERR = BIT_11,
    TRC_CTIO_DONE = BIT_12,
    TRC_CTIO_ABORTED =  BIT_13,
    TRC_CTIO_STRANGE = BIT_14,
    TRC_CMD_DONE = BIT_15,
    TRC_CMD_CHK_STOP = BIT_16,
    TRC_CMD_FREE = BIT_17,
    TRC_DATA_IN = BIT_18,
    TRC_ABORT = BIT_19,
    TRC_DIF_ERR = BIT_20,
    TRC_SRR_IMM = BIT_21,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_cmd {
//
// Do not move cmd_type field. it needs to line up with srb->cmd_type
//
    pub cmd_type: u8,
    pub pad: [u8; 7],
    pub se_cmd: se_cmd,
    pub sess_cmd_list: list_head,
    pub sess: *mut fc_port,
    pub qpair: *mut qla_qpair,
    pub reset_count: u32,
    pub state: c_int,
    pub work: work_struct,
// Sense buffer that will be mapped into outgoing status
    pub sense_buffer: [c_uchar; TRANSPORT_SENSE_BUFFER],
    pub conf_compl_supported:1: c_uint,
    pub sg_mapped:1: c_uint,
// Call qlt_free_sg() if set.
    pub free_sg:1: c_uint,
    pub write_data_transferred:1: c_uint,
// Set if the SCSI status was sent successfully.
    pub rsp_sent:1: c_uint,
    pub q_full:1: c_uint,
    pub term_exchg:1: c_uint,
    pub cmd_sent_to_fw:1: c_uint,
    pub cmd_in_wq:1: c_uint,
    pub edif:1: c_uint,
// Set if a SRR was rejected.
    pub srr_failed:1: c_uint,
// Set if the exchange has been terminated.
    pub sent_term_exchg:1: c_uint,
//
// Set if sent_term_exchg is set, or if the cmd was aborted by a TMR,
// or if some other error prevents normal processing of the command.
//
    pub aborted:1: c_uint,
    pub srr: *mut qla_tgt_srr,
    pub /: *mut *mut *mut scatterlist sg; / cmd data buffer SG vector,
    pub /: *mut *mut int sg_cnt; / SG segments count,
    pub /: *mut *mut int bufflen; / cmd buffer length,
    pub offset: c_int,
    pub unpacked_lun: u64,
    pub dma_data_direction: dma_data_direction,
    pub ctio_flags: u16,
    pub vp_idx: u16,
    pub /: *mut *mut uint16_t loop_id; / to save extra sess dereferences,
    pub /: *mut *mut *mut qla_tgt tgt; / to save extra sess dereferences,
    pub vha: *mut scsi_qla_host,
    pub cmd_list: list_head,
    pub atio: atio_from_isp,
    pub ctx_dsd_alloced: u8,
// T10-DIF
pub const DIF_ERR_NONE: c_int = 0;
pub const DIF_ERR_GRD: c_int = 1;
pub const DIF_ERR_REF: c_int = 2;
pub const DIF_ERR_APP: c_int = 3;
    pub dif_err_code: i8,
    pub prot_sg: *mut scatterlist,
    pub prot_sg_cnt: u32,
    pub num_blks: uint32_t blk_sz,,
    pub ascq: uint8_t scsi_status, sense_key, asc,,
    pub ctx: *mut crc_context,
    pub cdb: *mut u8,
    pub lba: u64,
    pub cdb_len: c_int,
    pub e_app_tag: uint16_t a_guard, e_guard, a_app_tag,,
    pub e_ref_tag: uint32_t a_ref_tag,,
pub const DIF_BUNDL_DMA_VALID: c_int = 1;
    pub prot_flags: u16,
    pub jiffies_at_term_exchg: c_ulong,
//
// jiffies64 when qlt_rdy_to_xfer() or qlt_xmit_response() first
// called, or 0 when not in those states.  Used to limit the number of
// SRR retries.
//
    pub jiffies_at_hw_st_entry: u64,
    pub jiffies_at_alloc: u64,
    pub jiffies_at_free: u64,
    pub trc_flags: trace_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_sess_work_param {
    pub sess_works_list_entry: list_head,
pub const QLA_TGT_SESS_WORK_ABORT: c_int = 1;
    pub type: c_int,
    pub abts: abts_recv_from_24xx,
    pub tm_iocb: imm_ntfy_from_isp,
    pub tm_iocb2: atio_from_isp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_mgmt_cmd {
    pub cmd_type: u8,
    pub pad: [u8; 3],
    pub tmr_func: u16,
    pub fc_tm_rsp: u8,
    pub abort_io_attr: u8,
    pub sess: *mut fc_port,
    pub qpair: *mut qla_qpair,
    pub vha: *mut scsi_qla_host,
    pub se_cmd: se_cmd,
    pub free_work: work_struct,
    pub flags: c_uint,

    pub reset_count: u32,
    pub work: work_struct,
    pub unpacked_lun: u64,
    pub atio: atio_from_isp,
    pub imm_ntfy: imm_ntfy_from_isp,
    pub abts: abts_recv_from_24xx,
    pub orig_iocb: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_prm {
    pub cmd: *mut qla_tgt_cmd,
    pub tgt: *mut qla_tgt,
    pub pkt: *mut c_void,
    pub /: *mut *mut *mut scatterlist sg; / cmd data buffer SG vector,
    pub sense_buffer: *mut c_uchar,
    pub seg_cnt: c_int,
    pub req_cnt: c_int,
    pub rq_result: u16,
    pub sense_buffer_len: c_int,
    pub residual: c_int,
    pub add_status_pkt: c_int,
// dif
    pub prot_sg: *mut scatterlist,
    pub prot_seg_cnt: u16,
    pub tot_dsds: u16,
}

//
// SRR (Sequence Retransmission Request) - resend or re-receive some or all
// data or status to recover from a transient I/O error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_srr {
//
// Copy of immediate notify SRR message received from hw; valid only if
// imm_ntfy_recvd is true.
//
    pub imm_ntfy: imm_ntfy_from_isp,
    pub srr_list_entry: list_head,
// The command affected by this SRR, or NULL if not yet determined.
    pub cmd: *mut qla_tgt_cmd,
// Used to detect if the HBA has been reset since receiving the SRR.
    pub reset_count: u32,
//
// The hardware sends two messages for each SRR - an immediate notify
// and a CTIO with CTIO_SRR_RECEIVED status.  These keep track of which
// messages have been received.  The SRR can be processed once both of
// these are true.
//
    pub imm_ntfy_recvd: bool,
    pub ctio_recvd: bool,
//
// This is set to true if the affected command was aborted (cmd may be
// set to NULL), in which case the immediate notify exchange also needs
// to be aborted.
//
    pub aborted: bool,
// This is set to true to force the SRR to be rejected.
    pub reject: bool,
}

// Check for Switch reserved address

pub const QLA_TGT_XMIT_DATA: c_int = 1;
pub const QLA_TGT_XMIT_STATUS: c_int = 2;

//
// Function prototypes for qla_target.c logic used by qla2xxx LLD code.
//
extern "C" {
    pub fn qlt_add_target(: *mut qla_hw_data, : *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qlt_remove_target(: *mut qla_hw_data, : *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qlt_lport_deregister(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlt_unreg_sess(: *mut fc_port);
}
extern "C" {
    pub fn qlt_fc_port_added(: *mut scsi_qla_host, : *mut fc_port_t);
}
extern "C" {
    pub fn qlt_init() -> int __init;
}
extern "C" {
    pub fn qlt_exit();
}
extern "C" {
    pub fn qlt_free_session_done(: *mut work_struct);
}
//
// This macro is used during early initializations when host->active_mode
// is not set. Right now, ha value is ignored.
//

//
// Free the scatterlist allocated by qlt_set_data_offset().  Call this only if
// cmd->free_sg is set.
//
// The scatterlist may be chained to the original scatterlist, but we
// only need to free the first segment here since that is the only part
// allocated by qlt_set_data_offset().
//
// Exported symbols from qla_target.c LLD logic used by qla2xxx code..
//
extern "C" {
    pub fn qlt_rdy_to_xfer(: *mut qla_tgt_cmd) -> c_int;
}
extern "C" {
    pub fn qlt_xmit_response(: *mut qla_tgt_cmd, _arg: c_int, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlt_abort_cmd(: *mut qla_tgt_cmd) -> c_int;
}
extern "C" {
    pub fn qlt_srr_abort(cmd: *mut qla_tgt_cmd, reject: bool);
}
extern "C" {
    pub fn qlt_xmit_tm_rsp(: *mut qla_tgt_mgmt_cmd);
}
extern "C" {
    pub fn qlt_free_ul_mcmd(ha: *mut qla_hw_data, mcmd: *mut qla_tgt_mgmt_cmd);
}
extern "C" {
    pub fn qlt_free_mcmd(: *mut qla_tgt_mgmt_cmd);
}
extern "C" {
    pub fn qlt_free_cmd(cmd: *mut qla_tgt_cmd);
}
extern "C" {
    pub fn qlt_unmap_sg(vha: *mut scsi_qla_host, cmd: *mut qla_tgt_cmd);
}
extern "C" {
    pub fn qlt_async_event(_arg: u16, : *mut scsi_qla_host, : *mut u16);
}
extern "C" {
    pub fn qlt_enable_vha(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlt_vport_create(: *mut scsi_qla_host, : *mut qla_hw_data);
}
extern "C" {
    pub fn qlt_rff_id(: *mut scsi_qla_host) -> u8;
}
extern "C" {
    pub fn qlt_init_atio_q_entries(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlt_24xx_process_atio_queue(: *mut scsi_qla_host, _arg: u8);
}
extern "C" {
    pub fn qlt_24xx_config_rings(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlt_config_nvram_with_fw_version(vha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlt_probe_one_stage1(: *mut scsi_qla_host, : *mut qla_hw_data);
}
extern "C" {
    pub fn qlt_mem_alloc(: *mut qla_hw_data) -> c_int;
}
extern "C" {
    pub fn qlt_mem_free(: *mut qla_hw_data);
}
extern "C" {
    pub fn qlt_stop_phase1(: *mut qla_tgt) -> c_int;
}
extern "C" {
    pub fn qlt_stop_phase2(: *mut qla_tgt);
}
extern "C" {
    pub fn qla83xx_msix_atio_q(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qlt_logo_completion_handler(: *mut fc_port_t, _arg: c_int);
}
extern "C" {
    pub fn qlt_do_generation_tick(: *mut scsi_qla_host, : *mut c_int);
}
