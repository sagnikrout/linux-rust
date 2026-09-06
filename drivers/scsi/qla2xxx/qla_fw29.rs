//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_fw29.h
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
// Copyright (c)  2026- Marvell.
//
// See LICENSE.qla2xxx for copyright and licensing details.
//

// Control Flags 2 common for cmd6 and 7

//
// vp_index layout for 29xx extended command IOCBs
// (cmd_type_6_ext, cmd_type_7_ext, cmd_type_crc_2_ext, ...):
// bits [8:0]   - VP index (9 bits)
// bits [15:9]  - reserved, must be zero
// Access on a host-endian value via le16_to_cpu(vp_index) & CMD_EXT_VP_INDEX_MASK.
//
pub const CMD_EXT_VP_INDEX_MASK: c_uint = 0x01ff;
//
// Combined vp_index/sof_type field layout (used by ELS and ABTS ext IOCBs):
// bits [8:0]   - VP index (9 bits)
// bits [11:9]  - reserved
// bits [15:12] - SOF type (4 bits)
//
pub const EXT_VP_SOF_VP_INDEX_MASK: c_uint = 0x01ff;
pub const EXT_VP_SOF_SOF_TYPE_SHIFT: c_int = 12;
pub const EXT_VP_SOF_SOF_TYPE_MASK: c_uint = 0xf000;
//
// Combined vp_idx/vp_status field layout (vp_rpt_id_entry_24xx_ext):
// bits [8:0]   - VP index (9 bits)
// bits [15:9]  - VP status (7 bits)
//
pub const EXT_VP_STATUS_VP_INDEX_MASK: c_uint = 0x01ff;
pub const EXT_VP_STATUS_VP_STATUS_SHIFT: c_int = 9;
pub const EXT_VP_STATUS_VP_STATUS_MASK: c_uint = 0xfe00;
//
// ISP queue - command entry structure definition.
//
pub const NUM_CMD67_DSDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_6_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le16 fcp_rsp_dsd_len; / FCP_RSP DSD length.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub /: *mut *mut __le16 fcp_cmnd_dseg_len; / Data segment length.,
// Data segment address.
    pub __packed: __le64 fcp_cmnd_dseg_address,
// Data segment address.
    pub __packed: __le64 fcp_rsp_dseg_address,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut __le16 control_flags_2; / Control flags 2.,
    pub 9bits*/: *mut *mut __le16 vp_index; / VP Index,
    pub /: *mut *mut __le32 fburstlen_rxid; / First Burst length/RX ID,
    pub /: *mut *mut __le16 io_tag; / I/O Tag,
    pub /: *mut *mut uint8_t vl_n_fctl; / VL (7-4) | RSVD (3-2) | F_CTL [17] (1) | RSVD (0),
    pub /: *mut *mut uint8_t prtag_csctl; / Priority Tag or CS_CTL,
    pub /: *mut *mut __le32 src_vm_id; / Source VM ID,
    pub /: *mut *mut uint8_t reserved_2[16]; / Reserved,
    pub /: *mut *mut dsd64 dsd[NUM_CMD67_DSDS]; / Data Segment Descriptors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_7_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub reserved_1: u16,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 task_mgmt_flags; / Task management flags.,
    pub task: u8,
    pub crn: u8,
    pub /: *mut *mut uint8_t fcp_cdb[MAX_CMDSZ]; / SCSI command words.,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut __le16 ctrl_flags_2; / Control flags 2,
    pub 9bits*/: *mut *mut __le16 vp_index; / VP Index,
    pub /: *mut *mut __le32 rx_id; / Receive Exchange ID,
    pub /: *mut *mut __le16 io_tag; / I/O Tag,
    pub /: *mut *mut uint8_t vl_n_fctl; / VL (7-4) | RSVD (3-2) | F_CTL [17] (1) | RSVD (0),
    pub /: *mut *mut uint8_t reserved_3[21]; / Reserved,
    pub /: *mut *mut dsd64 dsd[NUM_CMD67_DSDS]; / Data Segment Descriptors,
}

//
// Inline data-DSD capacity of the 29xx cmd_type_crc_2_ext IOCB.  Unlike
// cmd_type_6_ext / cmd_type_7_ext (which carry NUM_CMD67_DSDS inline DSDs),
// CRC_2 places the bulk of its DSDs in the separate CRC-context DMA; only
// a single data_dsd is carried inline in both the u.nobundling and
// u.bundling variants.  Use this constant wherever the IOCB-reservation
// calculator needs the CRC_2 ext inline capacity so it stays in sync with
// the firmware-facing layout below.
//
pub const NUM_CRC2_EXT_INLINE_DSDS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_type_crc_2_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le16 fcp_rsp_dseg_len; / FCP_RSP DSD length.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le16 control_flags_1; / Control flags.,
    pub /: *mut *mut __le16 fcp_cmnd_dseg_len; / Data segment length.,
    pub __packed: __le64 fcp_cmnd_dseg_address,
// Data segment address.
    pub __packed: __le64 fcp_rsp_dseg_address,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub /: *mut *mut __le16 control_flags_2; / Control flags - 2,
    pub reserved.: *mut *mut __le16 vp_index; / VP Index (bits [8:0]); bits [15:9],
// See CMD_EXT_VP_INDEX_MASK.
//
    pub reserved_1: u32,
    pub /: *mut *mut __le16 iocb_tag; / Unused,
    pub /: *mut *mut __le16 vl_prio; / Bit 1 - F_CTL, Bits 4-7 VL, rest are rsvd,
    pub /: *mut *mut uint32_t reserved_2; / 3C-3F offset,
    pub ref_tag: __le32,
    pub Mask*/: *mut *mut uint8_t ref_tag_mask[4]; / Validation/Replacement,
    pub app_tag: __le16,
    pub Mask*/: *mut *mut uint8_t app_tag_mask[2]; / Validation/Replacement,
    pub /: *mut *mut __le16 blk_size; / Data size in bytes,
    pub /: *mut *mut __le16 prot_opts; / Requested Data Protection Mode,
    pub data: *mut *mut __le32 tot_byte_count; / Total byte count/ total,
// transfer count
//
    pub /: *mut *mut uint32_t reserved_1; / offset 54,
    pub reserved_2: u16,
    pub /: *mut *mut __le16 guard_seed; / offset 5A,
    pub data_dsd: [dsd64; 1],
    pub reserved_5: [u32; 2],
    pub reserved_6: u32,
    pub nobundling: },
    pub byte: *mut *mut __le32 dif_byte_count; / Total DIF,
// count
//
    pub /: *mut *mut __le16 dseg_count; / Data segment count,
    pub /: *mut *mut __le16 guard_seed; / Initial Guard Seed,
    pub data_dsd: [dsd64; 1],
    pub dif_dsd: dsd64,
    pub bundling: },
    pub u: },
    pub /: *mut *mut uint8_t reserved_3[12]; / MUST be set to 0.,
}

//
// ISP queue - status entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sts_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 ox_id; / OX_ID used by the firmware.,
    pub /: *mut *mut __le32 residual_len; / FW calc residual transfer length.,
    pub reserved_1: __le16,
    pub nvme_rsp_pyld_len: __le16,
    pub u1: },
    pub /: *mut *mut __le16 state_flags; / State flags.,
    pub read_sa_index: __le16,
    pub wr_sa_index: __le16,
    pub reserved_2: [u8; 8],
    pub act_dif: [u8; 8],
    pub exp_dif: [u8; 8],
    pub /: *mut *mut __le32 rsp_data_len_dma; / FCP response data length,
    pub reserved_3: [u8; 76],
}

//
// If DIF Error is set in comp_status, these additional fields are
// defined:
//
// !!! NOTE: Firmware sends expected/actual DIF data in big endian
// format; but all of the "data" field gets swab32-d in the beginning
// of qla2900_status_entry().
//
// &data[10] : uint8_t report_runt_bg[2];	- computed guard
// &data[12] : uint8_t actual_dif[8];		- DIF Data received
// &data[20] : uint8_t expected_dif[8];		- DIF Data computed
//
// ISP queue - marker entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrk_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut uint8_t modifier; / Modifier (7-0).,
    pub reserved_1: u8,
    pub 9bits*/: *mut *mut __le16 vp_index; / VP Index.,
    pub reserved_3: u16,
    pub /: *mut *mut uint8_t lun[8]; / FCP LUN (BE).,
    pub reserved_4: [u8; 104],
}

//
// ISP queue - CT Pass-Through entry structure definition.
//
pub const NUM_CT_DSDS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub cmd_dsd_count: __le16,
    pub bits*/: *mut *mut __le16 vp_index; / vp index 9,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub reserved_2: u16,
    pub rsp_dsd_count: __le16,
    pub reserved_3: [u8; 10],
    pub /: *mut *mut uint8_t reserved_4[28]; / Reserved.,
    pub rsp_byte_count: __le32,
    pub cmd_byte_count: __le32,
    pub /: *mut *mut dsd64 dsd[NUM_CT_DSDS]; / Data Segment Descriptors,
}

//
// 29xx extended Link Service pass-through request IOCB (128 bytes).
//
// Same wire purpose as the 64-byte struct pt_ls4_request used on 24xx-class
// adapters, but laid out for the 128-byte 29xx request ring:
// - vp_index widened to __le16 (bits [8:0] meaningful, see
// CMD_EXT_VP_INDEX_MASK).
// - reserved area expanded to 32 bytes between exchange_address and
// rx_byte_count.
// - inline DSD capacity grown from 2 to 5.
// Header through 'tx_dseg_count' (offset 14) and the control_flags
// exchange_address fields keep the same offsets as struct pt_ls4_request,
// so common code can populate them via either type once IS_QLA29XX(ha) is
// branched for the layout-divergent fields.
//
pub const NUM_PT_LS4_EXT_DSDS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_ls4_request_ext {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_define: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub status: __le16,
    pub nport_handle: __le16,
    pub tx_dseg_count: __le16,
    pub /: *mut *mut __le16 vp_index; / VP Index 9 bits; see CMD_EXT_VP_INDEX_MASK,
    pub timeout: __le16,
    pub /: *mut *mut *mut __le16 control_flags; / CF_LS4_ (see struct pt_ls4_request),
    pub rx_dseg_count: __le16,
    pub rsvd2: __le16,
    pub exchange_address: __le32,
    pub rsvd3: [u8; 32],
    pub rx_byte_count: __le32,
    pub tx_byte_count: __le32,
    pub dsd: [dsd64; NUM_PT_LS4_EXT_DSDS],
}

//
// ISP queue - PUREX IOCB entry structure definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct purex_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved1: __le16,
    pub bits*/: *mut *mut __le16 vp_idx; / VP index 9,
    pub status_flags: __le16,
    pub nport_handle: __le16,
    pub frame_size: __le16,
    pub trunc_frame_size: __le16,
    pub rx_xchg_addr: __le32,
    pub d_id: [u8; 3],
    pub r_ctl: u8,
    pub s_id: [u8; 3],
    pub cs_ctl: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub param: __le32,
    pub els_frame_payload: [u8; 84],
}

//
// ISP queue - ELS Pass-Through entry structure definition.
// ELS_EXT_EST_SOFI*: 4-bit sof_type for extended IOCBs (qla_fw.h EST_SOFI
// is for els_entry_24xx byte layout).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / response only,
    pub nport_handle: __le16,
    pub tx_dsd_count: __le16,
    pub /: *mut *mut __le16 vp_index_sof; / bits [8:0]=VP index, [15:12]=SOF type,
    pub /: *mut *mut __le32 rx_xchg_address; / Receive exchange address.,
    pub rx_dsd_count: __le16,
    pub opcode: u8,
    pub reserved_2: u8,
    pub d_id: [u8; 3],
    pub s_id: [u8; 3],
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub rx_byte_count: __le32,
    pub tx_byte_count: __le32,
    pub /: *mut *mut __le64 tx_address __packed; / DSD 0 address.,
    pub /: *mut *mut __le32 tx_len; / DSD 0 length.,
    pub /: *mut *mut __le64 rx_address __packed; / DSD 1 address.,
    pub /: *mut *mut __le32 rx_len; / DSD 1 length.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_sts_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 handle; / System handle.,
    pub comp_status: __le16,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub reserved_1: __le16,
    pub /: *mut *mut __le16 vp_index_sof; / bits [8:0]=VP index, [15:12]=SOF type,
    pub /: *mut *mut __le32 rx_xchg_address; / Receive exchange address.,
    pub reserved_2: __le16,
    pub opcode: u8,
    pub reserved_3: u8,
    pub d_id: [u8; 3],
    pub s_id: [u8; 3],
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub total_byte_count: __le32,
    pub error_subcode_1: __le32,
    pub error_subcode_2: __le32,
    pub error_subcode_3: __le32,
    pub reserved_4: [u8; 80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logio_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub 9bits*/: *mut *mut __le16 vp_index; / VP Index,
    pub /: *mut *mut uint8_t port_id[3]; / PortID of destination port.,
    pub /: *mut *mut uint8_t rsp_size; / Response size in 32bit words.,
    pub /: *mut *mut __le32 io_parameter[11]; / General I/O parameters.,
    pub Reserved*/: *mut *mut uint8_t reserved_2[64]; /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsk_mgmt_entry_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub reserved_1: __le16,
    pub /: *mut *mut __le16 delay; / Activity delay in seconds.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut scsi_lun lun; / FCP LUN (BE).,
    pub /: *mut *mut __le32 control_flags; / Control Flags.,
    pub /: *mut *mut __le16 vp_index; / VP Index 9bits,
    pub reserved_3: [u8; 98],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t handle_count; / Handle count.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abts_entry_24xx_ext {
    pub entry_type: u8,
    pub entry_count: u8,
    pub handle_count: u8,
    pub entry_status: u8,
    pub /: *mut *mut __le32 handle; / type 0x55 only,
    pub /: *mut *mut __le16 comp_status; / type 0x55 only,
    pub /: *mut *mut __le16 nport_handle; / type 0x54 only,
    pub /: *mut *mut __le16 control_flags; / type 0x55 only,
    pub /: *mut *mut __le16 vp_idx_sof; / bits [8:0]=VP index, [15:12]=SOF type,
    pub rx_xch_addr: __le32,
    pub d_id: [u8; 3],
    pub r_ctl: u8,
    pub s_id: [u8; 3],
    pub cs_ctl: u8,
    pub f_ctl: [u8; 3],
    pub type: u8,
    pub seq_cnt: __le16,
    pub df_ctl: u8,
    pub seq_id: u8,
    pub rx_id: __le16,
    pub ox_id: __le16,
    pub param: __le32,
    pub subcode3: __le32,
    pub rsvd: __le32,
    pub subcode1: __le32,
    pub subcode2: __le32,
    pub error: },
    pub rsrvd1: __le16,
    pub last_seq_id: u8,
    pub seq_id_valid: u8,
    pub aborted_rx_id: __le16,
    pub aborted_ox_id: __le16,
    pub high_seq_cnt: __le16,
    pub low_seq_cnt: __le16,
    pub ba_acc: },
    pub vendor_unique: u8,
    pub explanation: u8,
    pub reason: u8,
    pub ba_rjt: },
    pub payload: },
    pub rx_xch_addr_to_abort: __le32,
    pub reserved_2: [u8; 64],
    pub __packed: },
//
// Virtual Port Control IOCB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_ctrl_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub vp_idx_failed: __le16,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub command: __le16,
    pub vp_count: __le16,
    pub vp_idx_map: [u8; 16],
    pub flags: __le16,
    pub id: __le16,
    pub reserved_4: u16,
    pub hopct: __le16,
    pub reserved_5: [u8; 88],
}

//
// Modify Virtual Port Configuration IOCB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_config_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub handle_count: u8,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub flags: __le16,
    pub /: *mut *mut __le16 comp_status; / Completion status.,
    pub command: u8,
    pub vp_count: u8,
    pub vp_index1: u8,
    pub vp_index2: u8,
    pub options_idx1: u8,
    pub hard_address_idx1: u8,
    pub reserved_vp1: u16,
    pub port_name_idx1: [u8; WWN_SIZE],
    pub node_name_idx1: [u8; WWN_SIZE],
    pub options_idx2: u8,
    pub hard_address_idx2: u8,
    pub reserved_vp2: u16,
    pub port_name_idx2: [u8; WWN_SIZE],
    pub node_name_idx2: [u8; WWN_SIZE],
    pub id: __le16,
    pub reserved_4: u16,
    pub hopct: __le16,
    pub reserved_5: [u8; 66],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_rpt_id_entry_24xx_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub resv1: __le32,
    pub vp_acquired: u8,
    pub vp_setup: u8,
    pub /: *mut *mut __le16 vp_idx_status; / bits [8:0]=VP index, [15:9]=VP status,
    pub port_id: [u8; 3],
    pub format: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_rpt_id_ext_f1 {
// format 1 fabric
    pub /: *mut *mut uint8_t vpstat1_subcode; / vp_status=1 subcode,
    pub flags: u8,
    pub fip_flags: __le16,
    pub rsv2: [u8; 12],
    pub ls_rjt_vendor: u8,
    pub ls_rjt_explanation: u8,
    pub ls_rjt_reason: u8,
    pub rsv3: u8,
    pub rsv8: __le16,
    pub /: *mut *mut __le16 flogi_acc_payload_size; / bits [8:0] meaningful,
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
    pub bbcr: __le16,
    pub reserved_5: [u8; 6],
    pub f1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp_rpt_id_ext_f2 {
    pub vpstat1_subcode: u8,
    pub flags: u8,
    pub fip_flags: __le16,
    pub rsv2: [u8; 12],
    pub ls_rjt_vendor: u8,
    pub ls_rjt_explanation: u8,
    pub ls_rjt_reason: u8,
    pub rsv3: [u8; 5],
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
    pub bbcr: __le16,
    pub reserved_5: [u8; 2],
    pub remote_nport_id: [u8; 4],
    pub f2: },
    pub u: },
    pub reserved_end: [u8; 64],
}

//
// ISP queue - 64-Bit addressing, continuation entry structure definition
// for the 29xx extended (128-byte) IOCB ring.  Mirrors cont_a64_entry_t
// but carries 10 DSDs per entry instead of 5.
//
pub const NUM_CONT1_DSDS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cont_a64_entry_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub reserved: u32,
    pub dsd: [dsd64; NUM_CONT1_DSDS],
}

//
// 29xx extended Command Type FC-NVMe IOCB (128 bytes).
//
// The header layout up through 'byte_count' (offset 48) is identical to the
// 64-byte struct cmd_nvme used by 24xx-class adapters, so common code can
// populate those fields via either type.  Fields beyond 'byte_count' diverge:
// 29xx adds control_flags_2/vp_index/first_burst_rx_id/io_tag/..., drops
// port_id[3]+vp_index(byte), and carries NUM_NVME_DSDS inline DSDs.
//
pub const NUM_NVME_DSDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_nvme_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut __le16 nport_handle; / N_PORT handle.,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut __le16 nvme_rsp_dsd_len; / NVMe RSP DSD length,
    pub rsvd: u64,
    pub /: *mut *mut __le16 control_flags; / Control Flags (see struct cmd_nvme),
    pub /: *mut *mut __le16 nvme_cmnd_dseg_len; / Data segment length.,
    pub /: *mut *mut __le64 nvme_cmnd_dseg_address __packed; / Data segment address.,
    pub /: *mut *mut __le64 nvme_rsp_dseg_address __packed; / Data segment address.,
    pub /: *mut *mut __le32 byte_count; / Total byte count.,
    pub control_flags_2: __le16,
//
// vp_index layout matches the other 29xx extended IOCBs: only bits
// [8:0] are meaningful (see CMD_EXT_VP_INDEX_MASK).
//
    pub vp_index: __le16,
    pub first_burst_rx_id: __le32,
    pub io_tag: __le16,
    pub /: *mut *mut uint8_t vl_n_fctl; / VL(7:4) | RSVD(3:2) | F_CTL[17](1) | RSVD(0),
    pub /: *mut *mut uint8_t prtag_csctl; / Priority Tag or CS_CTL,
    pub /: *mut *mut __le32 src_vm_id; / Source VM ID,
    pub reserved_2: [u8; 16],
    pub nvme_dsd: [dsd64; NUM_NVME_DSDS],
}
