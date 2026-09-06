//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_def.h
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

// Big endian Fibre Channel S_ID (source ID) or D_ID (destination ID).
// Little endian Fibre Channel S_ID (source ID) or D_ID (destination ID).
//
// 24 bit port ID type definition.
//

pub const INVALID_PORT_ID: c_uint = 0xFFFFFF;

//
// We have MAILBOX_REGISTER_COUNT sized arrays in a few places,
// but that's fine as we don't look at the last 24 ones for
// ISP2100 HBAs.
//
pub const MAILBOX_REGISTER_COUNT_2100: c_int = 8;
pub const MAILBOX_REGISTER_COUNT_2200: c_int = 24;
pub const MAILBOX_REGISTER_COUNT: c_int = 32;
pub const QLA2200A_RISC_ROM_VER: c_int = 4;
pub const FPM_2300: c_int = 6;
pub const FPM_2310: c_int = 7;

//
// Data bit definitions
//
pub const BIT_0: c_uint = 0x1;
pub const BIT_1: c_uint = 0x2;
pub const BIT_2: c_uint = 0x4;
pub const BIT_3: c_uint = 0x8;
pub const BIT_4: c_uint = 0x10;
pub const BIT_5: c_uint = 0x20;
pub const BIT_6: c_uint = 0x40;
pub const BIT_7: c_uint = 0x80;
pub const BIT_8: c_uint = 0x100;
pub const BIT_9: c_uint = 0x200;
pub const BIT_10: c_uint = 0x400;
pub const BIT_11: c_uint = 0x800;
pub const BIT_12: c_uint = 0x1000;
pub const BIT_13: c_uint = 0x2000;
pub const BIT_14: c_uint = 0x4000;
pub const BIT_15: c_uint = 0x8000;
pub const BIT_16: c_uint = 0x10000;
pub const BIT_17: c_uint = 0x20000;
pub const BIT_18: c_uint = 0x40000;
pub const BIT_19: c_uint = 0x80000;
pub const BIT_20: c_uint = 0x100000;
pub const BIT_21: c_uint = 0x200000;
pub const BIT_22: c_uint = 0x400000;
pub const BIT_23: c_uint = 0x800000;
pub const BIT_24: c_uint = 0x1000000;
pub const BIT_25: c_uint = 0x2000000;
pub const BIT_26: c_uint = 0x4000000;
pub const BIT_27: c_uint = 0x8000000;
pub const BIT_28: c_uint = 0x10000000;
pub const BIT_29: c_uint = 0x20000000;
pub const BIT_30: c_uint = 0x40000000;
pub const BIT_31: c_uint = 0x80000000;

//
// I/O register
//
extern "C" {
    pub fn readb(_arg: addr) -> return;
}
extern "C" {
    pub fn readw(_arg: addr) -> return;
}
extern "C" {
    pub fn readl(_arg: addr) -> return;
}
extern "C" {
    pub fn readb_relaxed(_arg: addr) -> return;
}
extern "C" {
    pub fn readw_relaxed(_arg: addr) -> return;
}
extern "C" {
    pub fn readl_relaxed(_arg: addr) -> return;
}
extern "C" {
    pub fn writeb(_arg: data, _arg: addr) -> return;
}
extern "C" {
    pub fn writew(_arg: data, _arg: addr) -> return;
}
extern "C" {
    pub fn writel(_arg: data, _arg: addr) -> return;
}
//
// ISP83XX specific remote register addresses
//
pub const QLA83XX_LED_PORT0: c_uint = 0x00201320;
pub const QLA83XX_LED_PORT1: c_uint = 0x00201328;
pub const QLA83XX_IDC_DEV_STATE: c_uint = 0x22102384;
pub const QLA83XX_IDC_MAJOR_VERSION: c_uint = 0x22102380;
pub const QLA83XX_IDC_MINOR_VERSION: c_uint = 0x22102398;
pub const QLA83XX_IDC_DRV_PRESENCE: c_uint = 0x22102388;
pub const QLA83XX_IDC_DRIVER_ACK: c_uint = 0x2210238c;
pub const QLA83XX_IDC_CONTROL: c_uint = 0x22102390;
pub const QLA83XX_IDC_AUDIT: c_uint = 0x22102394;
pub const QLA83XX_IDC_LOCK_RECOVERY: c_uint = 0x2210239c;
pub const QLA83XX_DRIVER_LOCKID: c_uint = 0x22102104;
pub const QLA83XX_DRIVER_LOCK: c_uint = 0x8111c028;
pub const QLA83XX_DRIVER_UNLOCK: c_uint = 0x8111c02c;
pub const QLA83XX_FLASH_LOCKID: c_uint = 0x22102100;
pub const QLA83XX_FLASH_LOCK: c_uint = 0x8111c010;
pub const QLA83XX_FLASH_UNLOCK: c_uint = 0x8111c014;
pub const QLA83XX_DEV_PARTINFO1: c_uint = 0x221023e0;
pub const QLA83XX_DEV_PARTINFO2: c_uint = 0x221023e4;
pub const QLA83XX_FW_HEARTBEAT: c_uint = 0x221020b0;
pub const QLA83XX_PEG_HALT_STATUS1: c_uint = 0x221020a8;
pub const QLA83XX_PEG_HALT_STATUS2: c_uint = 0x221020ac;
// 83XX: Macros defining 8200 AEN Reason codes

// 83XX: Macros defining 8200 AEN Error-levels
pub const ERR_LEVEL_NON_FATAL: c_uint = 0x1;
pub const ERR_LEVEL_RECOVERABLE_FATAL: c_uint = 0x2;
pub const ERR_LEVEL_UNRECOVERABLE_FATAL: c_uint = 0x4;
// 83XX: Macros for IDC Version
pub const QLA83XX_SUPP_IDC_MAJOR_VERSION: c_uint = 0x01;
pub const QLA83XX_SUPP_IDC_MINOR_VERSION: c_uint = 0x0;
// 83XX: Macros for scheduling dpc tasks
pub const QLA83XX_NIC_CORE_RESET: c_uint = 0x1;
pub const QLA83XX_IDC_STATE_HANDLER: c_uint = 0x2;
pub const QLA83XX_NIC_CORE_UNRECOVERABLE: c_uint = 0x3;
// 83XX: Macros for defining IDC-Control bits

// 83XX: Macros for different timeouts
pub const QLA83XX_IDC_INITIALIZATION_TIMEOUT: c_int = 30;
pub const QLA83XX_IDC_RESET_ACK_TIMEOUT: c_int = 10;

// 83XX: Macros for defining class in DEV-Partition Info register
pub const QLA83XX_CLASS_TYPE_NONE: c_uint = 0x0;
pub const QLA83XX_CLASS_TYPE_NIC: c_uint = 0x1;
pub const QLA83XX_CLASS_TYPE_FCOE: c_uint = 0x2;
pub const QLA83XX_CLASS_TYPE_ISCSI: c_uint = 0x3;
// 83XX: Macros for IDC Lock-Recovery stages
pub const IDC_LOCK_RECOVERY_STAGE1: c_uint = 0x1 /* Stage1: Intent for;
// lock-recovery
//
pub const IDC_LOCK_RECOVERY_STAGE2: c_uint = 0x2 /* Stage2: Perform lock-recovery */;
// 83XX: Macros for IDC Audit type
pub const IDC_AUDIT_TIMESTAMP: c_uint = 0x0 /* IDC-AUDIT: Record timestamp of;
// dev-state change to NEED-RESET
// or NEED-QUIESCENT
//
pub const IDC_AUDIT_COMPLETION: c_uint = 0x1 /* IDC-AUDIT: Record duration of;
// reset-recovery completion is
// second
//
// ISP2031: Values for laser on/off
pub const PORT_0_2031: c_uint = 0x00201340;
pub const PORT_1_2031: c_uint = 0x00201350;
pub const LASER_ON_2031: c_uint = 0x01800100;
pub const LASER_OFF_2031: c_uint = 0x01800180;
//
// The ISP2312 v2 chip cannot access the FLASH/GPIO registers via MMIO in an
// 133Mhz slot.
//

//
// Fibre Channel device definitions.
//

pub const MAX_FIBRE_DEVICES_2100: c_int = 512;
pub const MAX_FIBRE_DEVICES_2400: c_int = 2048;
pub const MAX_FIBRE_DEVICES_LOOP: c_int = 128;

pub const MAX_FIBRE_LUNS: c_uint = 0xFFFF;
pub const MAX_HOST_COUNT: c_int = 16;
//
// Host adapter default definitions.
//

pub const MIN_LUNS: c_int = 8;

pub const MAX_CMDS_PER_LUN: c_int = 255;
//
// Fibre Channel device definitions.
//
pub const SNS_LAST_LOOP_ID_2100: c_uint = 0xfe;
pub const SNS_LAST_LOOP_ID_2300: c_uint = 0x7ff;
pub const LAST_LOCAL_LOOP_ID: c_uint = 0x7d;
pub const SNS_FL_PORT: c_uint = 0x7e;
pub const FABRIC_CONTROLLER: c_uint = 0x7f;
pub const SIMPLE_NAME_SERVER: c_uint = 0x80;
pub const SNS_FIRST_LOOP_ID: c_uint = 0x81;
pub const MANAGEMENT_SERVER: c_uint = 0xfe;
pub const BROADCAST: c_uint = 0xff;
//
// There is no correspondence between an N-PORT id and an AL_PA.  Therefore the
// valid range of an N-PORT id is 0 through 0x7ef.
//
pub const NPH_LAST_HANDLE: c_uint = 0x7ee;
pub const NPH_MGMT_SERVER: c_uint = 0x7ef		/*  FFFFEF */;
pub const NPH_SNS: c_uint = 0x7fc		/*  FFFFFC */;
pub const NPH_FABRIC_CONTROLLER: c_uint = 0x7fd		/*  FFFFFD */;
pub const NPH_F_PORT: c_uint = 0x7fe		/*  FFFFFE */;
pub const NPH_IP_BROADCAST: c_uint = 0x7ff		/*  FFFFFF */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name_list_extended {
    pub l: *mut get_name_list_extended,
    pub ldma: dma_addr_t,
    pub fcports: list_head,
    pub size: u32,
    pub sent: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_nvme_fc_rjt {
    pub c: *mut fcnvme_ls_rjt,
    pub cdma: dma_addr_t,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_reject {
    pub c: *mut fc_els_ls_rjt,
    pub cdma: dma_addr_t,
    pub size: u16,
}

//
// Timeout timer counts in seconds
//
pub const PORT_RETRY_TIME: c_int = 1;
pub const LOOP_DOWN_TIMEOUT: c_int = 60;

pub const DEFAULT_OUTSTANDING_COMMANDS: c_int = 4096;
pub const MIN_OUTSTANDING_COMMANDS: c_int = 128;
// ISP request and response entry counts (37-65535)

pub const FW_DEF_EXCHANGES_CNT: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_buf_dsc {
    pub tag: u16,
pub const TAG_FREED: c_uint = 0xffff;
    pub buf: *mut c_void,
    pub buf_dma: dma_addr_t,
}

//
// SCSI Request Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srb_cmd {
    pub /: *mut *mut *mut scsi_cmnd cmd; / Linux SCSI command pkt,
    pub request_sense_length: u32,
    pub fw_sense_length: u32,
    pub request_sense_ptr: *mut u8,
    pub crc_ctx: *mut crc_context,
    pub ct6_ctx: ct6_dsd,
    pub buf_dsc: qla_buf_dsc,
}

//
// SRB flag definitions
//

// To identify if a srb is of T10-CRC type. @sp => srb_t pointer

pub const ISP_REG16_DISCONNECT: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmf_arg {
    pub tmf_elem: list_head,
    pub qpair: *mut qla_qpair,
    pub fcport: *mut fc_port,
    pub vha: *mut scsi_qla_host,
    pub lun: u64,
    pub flags: u32,
    pub modifier: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_logo_payload {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub s_id: [u8; 3],
    pub rsvd1: [u8; 1],
    pub wwpn: [u8; WWN_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_plogi_payload {
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub 4]: __be32 data[112 /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_arg {
    pub iocb: *mut c_void,
    pub nport_handle: u16,
    pub req_dma: dma_addr_t,
    pub rsp_dma: dma_addr_t,
    pub req_size: u32,
    pub rsp_size: u32,
    pub req_allocated_size: u32,
    pub rsp_allocated_size: u32,
    pub req: *mut c_void,
    pub rsp: *mut c_void,
    pub id: port_id_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_nvme_lsrjt_pt_arg {
    pub fcport: *mut fc_port,
    pub opcode: u8,
    pub vp_idx: u8,
    pub reason: u8,
    pub explanation: u8,
    pub nport_handle: __le16,
    pub control_flags: u16,
    pub ox_id: __le16,
    pub xchg_address: __le32,
    pub rx_byte_count: u32 tx_byte_count,,
    pub rx_addr: dma_addr_t tx_addr,,
}

//
// SRB extensions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srb_iocb {
    pub flags: u16,
    pub data: [u16; 2],
    pub iop: [u32; 2],
    pub logio: },
pub const ELS_DCMD_TIMEOUT: c_int = 20;
pub const ELS_DCMD_LOGO: c_uint = 0x5;
    pub flags: u32,
    pub els_cmd: u32,
    pub comp: completion,
    pub els_logo_pyld: *mut els_logo_payload,
    pub els_logo_pyld_dma: dma_addr_t,
    pub els_logo: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct els_plogi {
pub const ELS_DCMD_PLOGI: c_uint = 0x3;
    pub flags: u32,
    pub els_cmd: u32,
    pub comp: completion,
    pub els_plogi_pyld: *mut els_plogi_payload,
    pub els_resp_pyld: *mut els_plogi_payload,
    pub tx_size: u32,
    pub rx_size: u32,
    pub els_plogi_pyld_dma: dma_addr_t,
    pub els_resp_pyld_dma: dma_addr_t,
    pub fw_status: [__le32; 3],
    pub comp_status: __le16,
    pub len: __le16,
    pub els_plogi: },
//
// Values for flags field below are as
// defined in tsk_mgmt_entry struct
// for control_flags field in qla_fw.h.
//
    pub lun: u64,
    pub flags: u32,
    pub data: u32,
    pub comp: completion,
    pub comp_status: __le16,
    pub modifier: u8,
    pub vp_index: u16,
    pub loop_id: u16,
    pub tmf: },

pub const FXDISC_TIMEOUT: c_int = 20;
    pub flags: u8,
    pub req_len: u32,
    pub rsp_len: u32,
    pub req_addr: *mut c_void,
    pub rsp_addr: *mut c_void,
    pub req_dma_handle: dma_addr_t,
    pub rsp_dma_handle: dma_addr_t,
    pub adapter_id: __le32,
    pub adapter_id_hi: __le32,
    pub req_func_type: __le16,
    pub req_data: __le32,
    pub req_data_extra: __le32,
    pub result: __le32,
    pub seq_number: __le32,
    pub fw_flags: __le16,
    pub fxiocb_comp: completion,
    pub reserved_0: __le32,
    pub reserved_1: u8,
    pub fxiocb: },
    pub cmd_hndl: u32,
    pub comp_status: __le16,
    pub req_que_no: __le16,
    pub comp: completion,
    pub abt: },
    pub ctarg: ct_arg,
pub const MAX_IOCB_MB_REG: c_int = 28;

    pub /: *mut *mut u16 in_mb[MAX_IOCB_MB_REG]; / from FW,
    pub /: *mut *mut u16 out_mb[MAX_IOCB_MB_REG]; / to FW,
    pub in: *mut *mut void out,,
    pub in_dma: dma_addr_t out_dma,,
    pub comp: completion,
    pub rc: c_int,
    pub mbx: },
    pub ntfy: *mut imm_ntfy_from_isp,
    pub nack: },
    pub comp_status: __le16,
    pub rsp_pyld_len: __le16,
    pub aen_op: u8,
    pub desc: *mut c_void,
// These are only used with ls4 requests
    pub cmd_len: __le32,
    pub rsp_len: __le32,
    pub cmd_dma: dma_addr_t,
    pub rsp_dma: dma_addr_t,
    pub dir: nvmefc_fcp_datadir,
    pub dl: u32,
    pub timeout_sec: u32,
    pub exchange_address: __le32,
    pub nport_handle: __le16,
    pub ox_id: __le16,
    pub entry: list_head,
    pub nvme: },
    pub cmd: u16,
    pub vp_index: u16,
    pub ctrlvp: },
    pub sa_ctl: *mut edif_sa_ctl,
    pub sa_frame: qla_sa_update_frame,
    pub sa_update: },
    pub u: },
    pub timer: timer_list,
    pub ): *mut *mut void (timeout)(void,
}

// Values for srb_ctx type
pub const SRB_LOGIN_CMD: c_int = 1;
pub const SRB_LOGOUT_CMD: c_int = 2;
pub const SRB_ELS_CMD_RPT: c_int = 3;
pub const SRB_ELS_CMD_HST: c_int = 4;
pub const SRB_CT_CMD: c_int = 5;
pub const SRB_ADISC_CMD: c_int = 6;
pub const SRB_TM_CMD: c_int = 7;
pub const SRB_SCSI_CMD: c_int = 8;
pub const SRB_BIDI_CMD: c_int = 9;
pub const SRB_FXIOCB_DCMD: c_int = 10;
pub const SRB_FXIOCB_BCMD: c_int = 11;
pub const SRB_ABT_CMD: c_int = 12;
pub const SRB_ELS_DCMD: c_int = 13;
pub const SRB_MB_IOCB: c_int = 14;
pub const SRB_CT_PTHRU_CMD: c_int = 15;
pub const SRB_NACK_PLOGI: c_int = 16;
pub const SRB_NACK_PRLI: c_int = 17;
pub const SRB_NACK_LOGO: c_int = 18;
pub const SRB_NVME_CMD: c_int = 19;
pub const SRB_NVME_LS: c_int = 20;
pub const SRB_PRLI_CMD: c_int = 21;
pub const SRB_CTRL_VP: c_int = 22;
pub const SRB_PRLO_CMD: c_int = 23;
pub const SRB_SA_UPDATE: c_int = 25;
pub const SRB_ELS_CMD_HST_NOLOGIN: c_int = 26;
pub const SRB_SA_REPLACE: c_int = 27;
pub const SRB_MARKER: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_els_pt_arg {
    pub els_opcode: u8,
    pub vp_idx: u8,
    pub nport_handle: __le16,
    pub ox_id: u16 control_flags,,
    pub rx_xchg_address: __le32,
    pub sid: port_id_t did,,
    pub rx_byte_count: u32 tx_len, tx_byte_count, rx_len,,
    pub rx_addr: dma_addr_t tx_addr,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iocb_resource {
    pub res_type: u8,
    pub exch_cnt: u8,
    pub iocb_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_cmd {
    pub bsg_job: *mut bsg_job,
    pub els_arg: qla_els_pt_arg,
    pub u: },
}

//
// Do not move cmd_type field, it needs to
// line up with qla_tgt_cmd->cmd_type
//
// Report completion status @res and call sp_put(@sp). @res is
// an NVMe status code, a SCSI result (e.g. DID_OK << 16) or a
// QLA_* status value.
//
// Stop the timer and free @sp. Only used by the FCP code.
//
// Call nvme_private->fd->done() and free @sp. Only used by the NVMe
// code.
//
// Report completion for asynchronous commands.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_echo_lb {
    pub send_dma: dma_addr_t,
    pub rcv_dma: dma_addr_t,
    pub req_sg_cnt: u16,
    pub rsp_sg_cnt: u16,
    pub options: u16,
    pub transfer_size: u32,
    pub iteration_count: u32,
}

//
// ISP I/O Register Set structure definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_2xxx {
    pub /: *mut *mut __le16 flash_address; / Flash BIOS address,
    pub /: *mut *mut __le16 flash_data; / Flash BIOS data,
    pub /: *mut *mut __le16 unused_1[1]; / Gap,
    pub /: *mut *mut __le16 ctrl_status; / Control/Status,

    pub /: *mut *mut __le16 ictrl; / Interrupt control,

    pub /: *mut *mut __le16 istatus; / Interrupt status,

    pub /: *mut *mut __le16 semaphore; / Semaphore,
    pub /: *mut *mut __le16 nvram; / NVRAM register.,
pub const NVR_DESELECT: c_int = 0;

pub const NVR_WAIT_CNT: c_int = 20000;
    pub mailbox0: __le16,
    pub mailbox1: __le16,
    pub mailbox2: __le16,
    pub mailbox3: __le16,
    pub mailbox4: __le16,
    pub mailbox5: __le16,
    pub mailbox6: __le16,
    pub mailbox7: __le16,
    pub /: *mut *mut __le16 unused_2[59]; / Gap,
// C attribute field omitted
// Request Queue
    pub /: *mut *mut __le16 req_q_in; / In-Pointer,
    pub /: *mut *mut __le16 req_q_out; / Out-Pointer,
// Response Queue
    pub /: *mut *mut __le16 rsp_q_in; / In-Pointer,
    pub /: *mut *mut __le16 rsp_q_out; / Out-Pointer,
// RISC to Host Status
    pub host_status: __le32,

// Host to Host Semaphore
    pub host_semaphore: __le16,
    pub /: *mut *mut __le16 unused_3[17]; / Gap,
    pub mailbox0: __le16,
    pub mailbox1: __le16,
    pub mailbox2: __le16,
    pub mailbox3: __le16,
    pub mailbox4: __le16,
    pub mailbox5: __le16,
    pub mailbox6: __le16,
    pub mailbox7: __le16,
    pub mailbox8: __le16,
    pub mailbox9: __le16,
    pub mailbox10: __le16,
    pub mailbox11: __le16,
    pub mailbox12: __le16,
    pub mailbox13: __le16,
    pub mailbox14: __le16,
    pub mailbox15: __le16,
    pub mailbox16: __le16,
    pub mailbox17: __le16,
    pub mailbox18: __le16,
    pub mailbox19: __le16,
    pub mailbox20: __le16,
    pub mailbox21: __le16,
    pub mailbox22: __le16,
    pub mailbox23: __le16,
    pub mailbox24: __le16,
    pub mailbox25: __le16,
    pub mailbox26: __le16,
    pub mailbox27: __le16,
    pub mailbox28: __le16,
    pub mailbox29: __le16,
    pub mailbox30: __le16,
    pub mailbox31: __le16,
    pub fb_cmd: __le16,
    pub /: *mut *mut __le16 unused_4[10]; / Gap,
// C attribute field omitted
    pub u: },
    pub fpm_diag_config: __le16,
    pub /: *mut *mut __le16 unused_5[0x4]; / Gap,
    pub risc_hw: __le16,
    pub /: *mut *mut __le16 unused_5_1; / Gap,
    pub /: *mut *mut __le16 pcr; / Processor Control Register.,
    pub /: *mut *mut __le16 unused_6[0x5]; / Gap,
    pub /: *mut *mut __le16 mctr; / Memory Configuration and Timing.,
    pub /: *mut *mut __le16 unused_7[0x3]; / Gap,
    pub /: *mut *mut __le16 fb_cmd_2100; / Unused on 23XX,
    pub /: *mut *mut __le16 unused_8[0x3]; / Gap,
    pub /: *mut *mut __le16 hccr; / Host command & control register.,

// HCCR commands
pub const HCCR_RESET_RISC: c_uint = 0x1000	/* Reset RISC */;
pub const HCCR_PAUSE_RISC: c_uint = 0x2000	/* Pause RISC */;
pub const HCCR_RELEASE_RISC: c_uint = 0x3000	/* Release RISC from reset. */;
pub const HCCR_SET_HOST_INT: c_uint = 0x5000	/* Set host interrupt */;
pub const HCCR_CLR_HOST_INT: c_uint = 0x6000	/* Clear HOST interrupt */;
pub const HCCR_CLR_RISC_INT: c_uint = 0x7000	/* Clear RISC interrupt */;
pub const HCCR_DISABLE_PARITY_PAUSE: c_uint = 0x4001 /* Disable parity error RISC pause. */;
pub const HCCR_ENABLE_PARITY: c_uint = 0xA000	/* Enable PARITY interrupt */;
    pub /: *mut *mut __le16 unused_9[5]; / Gap,
    pub /: *mut *mut __le16 gpiod; / GPIO Data register.,
    pub /: *mut *mut __le16 gpioe; / GPIO Enable register.,
pub const GPIO_LED_MASK: c_uint = 0x00C0;
pub const GPIO_LED_GREEN_OFF_AMBER_OFF: c_uint = 0x0000;
pub const GPIO_LED_GREEN_ON_AMBER_OFF: c_uint = 0x0040;
pub const GPIO_LED_GREEN_OFF_AMBER_ON: c_uint = 0x0080;
pub const GPIO_LED_GREEN_ON_AMBER_ON: c_uint = 0x00C0;
pub const GPIO_LED_ALL_OFF: c_uint = 0x0000;
pub const GPIO_LED_RED_ON_OTHER_OFF: c_uint = 0x0001	/* isp2322 */;
pub const GPIO_LED_RGA_ON: c_uint = 0x00C1	/* isp2322: red green amber */;
    pub /: *mut *mut __le16 unused_10[8]; / Gap,
    pub mailbox8: __le16,
    pub mailbox9: __le16,
    pub mailbox10: __le16,
    pub mailbox11: __le16,
    pub mailbox12: __le16,
    pub mailbox13: __le16,
    pub mailbox14: __le16,
    pub mailbox15: __le16,
    pub mailbox16: __le16,
    pub mailbox17: __le16,
    pub mailbox18: __le16,
    pub mailbox19: __le16,
    pub mailbox20: __le16,
    pub mailbox21: __le16,
    pub mailbox22: __le16,
    pub /: *mut *mut __le16 mailbox23; / Also probe reg.,
// C attribute field omitted
    pub u_end: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_25xxmq {
    pub req_q_in: __le32,
    pub req_q_out: __le32,
    pub rsp_q_in: __le32,
    pub rsp_q_out: __le32,
    pub atio_q_in: __le32,
    pub atio_q_out: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_reg_fx00 {
    pub /: *mut *mut __le32 mailbox0; / 00,
    pub /: *mut *mut __le32 mailbox1; / 04,
    pub /: *mut *mut __le32 mailbox2; / 08,
    pub /: *mut *mut __le32 mailbox3; / 0C,
    pub /: *mut *mut __le32 mailbox4; / 10,
    pub /: *mut *mut __le32 mailbox5; / 14,
    pub /: *mut *mut __le32 mailbox6; / 18,
    pub /: *mut *mut __le32 mailbox7; / 1C,
    pub /: *mut *mut __le32 mailbox8; / 20,
    pub /: *mut *mut __le32 mailbox9; / 24,
    pub /: *mut *mut __le32 mailbox10; / 28,
    pub mailbox11: __le32,
    pub mailbox12: __le32,
    pub mailbox13: __le32,
    pub mailbox14: __le32,
    pub mailbox15: __le32,
    pub mailbox16: __le32,
    pub mailbox17: __le32,
    pub mailbox18: __le32,
    pub mailbox19: __le32,
    pub mailbox20: __le32,
    pub mailbox21: __le32,
    pub mailbox22: __le32,
    pub mailbox23: __le32,
    pub mailbox24: __le32,
    pub mailbox25: __le32,
    pub mailbox26: __le32,
    pub mailbox27: __le32,
    pub mailbox28: __le32,
    pub mailbox29: __le32,
    pub mailbox30: __le32,
    pub mailbox31: __le32,
    pub aenmailbox0: __le32,
    pub aenmailbox1: __le32,
    pub aenmailbox2: __le32,
    pub aenmailbox3: __le32,
    pub aenmailbox4: __le32,
    pub aenmailbox5: __le32,
    pub aenmailbox6: __le32,
    pub aenmailbox7: __le32,
// Request Queue.
    pub /: *mut *mut __le32 req_q_in; / A0 - Request Queue In-Pointer,
    pub /: *mut *mut __le32 req_q_out; / A4 - Request Queue Out-Pointer,
// Response Queue.
    pub /: *mut *mut __le32 rsp_q_in; / A8 - Response Queue In-Pointer,
    pub /: *mut *mut __le32 rsp_q_out; / AC - Response Queue Out-Pointer,
// Init values shadowed on FW Up Event
    pub /: *mut *mut __le32 initval0; / B0,
    pub /: *mut *mut __le32 initval1; / B4,
    pub /: *mut *mut __le32 initval2; / B8,
    pub /: *mut *mut __le32 initval3; / BC,
    pub /: *mut *mut __le32 initval4; / C0,
    pub /: *mut *mut __le32 initval5; / C4,
    pub /: *mut *mut __le32 initval6; / C8,
    pub /: *mut *mut __le32 initval7; / CC,
    pub /: *mut *mut __le32 fwheartbeat; / D0,
    pub /: *mut *mut __le32 pseudoaen; / D4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_cmd_32 {
    pub /: *mut *mut uint32_t out_mb; / outbound from driver,
    pub /: *mut *mut uint32_t in_mb; / Incoming from RISC,
    pub mb: [u32; MAILBOX_REGISTER_COUNT],
    pub buf_size: c_long,
    pub bufp: *mut c_void,
    pub tov: u32,
    pub flags: u8,

}

pub const MBX_TOV_SECONDS: c_int = 30;
//
// ISP product identification definitions in mailboxes after reset.
//
pub const PROD_ID_1: c_uint = 0x4953;
pub const PROD_ID_2: c_uint = 0x0000;
pub const PROD_ID_2a: c_uint = 0x5020;
pub const PROD_ID_3: c_uint = 0x2020;
//
// ISP mailbox Self-Test status codes
//

//
// ISP mailbox command complete status codes
//
pub const MBS_COMMAND_COMPLETE: c_uint = 0x4000;
pub const MBS_INVALID_COMMAND: c_uint = 0x4001;
pub const MBS_HOST_INTERFACE_ERROR: c_uint = 0x4002;
pub const MBS_TEST_FAILED: c_uint = 0x4003;
pub const MBS_COMMAND_ERROR: c_uint = 0x4005;
pub const MBS_COMMAND_PARAMETER_ERROR: c_uint = 0x4006;
pub const MBS_PORT_ID_USED: c_uint = 0x4007;
pub const MBS_LOOP_ID_USED: c_uint = 0x4008;
pub const MBS_ALL_IDS_IN_USE: c_uint = 0x4009;
pub const MBS_NOT_LOGGED_IN: c_uint = 0x400A;
pub const MBS_LINK_DOWN_ERROR: c_uint = 0x400B;
pub const MBS_DIAG_ECHO_TEST_ERROR: c_uint = 0x400C;
//
// ISP mailbox asynchronous event status codes
//
pub const MBA_ASYNC_EVENT: c_uint = 0x8000	/* Asynchronous event. */;
pub const MBA_RESET: c_uint = 0x8001	/* Reset Detected. */;
pub const MBA_SYSTEM_ERR: c_uint = 0x8002	/* System Error. */;
pub const MBA_REQ_TRANSFER_ERR: c_uint = 0x8003	/* Request Transfer Error. */;
pub const MBA_RSP_TRANSFER_ERR: c_uint = 0x8004	/* Response Transfer Error. */;
pub const MBA_WAKEUP_THRES: c_uint = 0x8005	/* Request Queue Wake-up. */;
pub const MBA_LIP_OCCURRED: c_uint = 0x8010	/* Loop Initialization Procedure */;
// occurred.
pub const MBA_LOOP_UP: c_uint = 0x8011	/* FC Loop UP. */;
pub const MBA_LOOP_DOWN: c_uint = 0x8012	/* FC Loop Down. */;
pub const MBA_LIP_RESET: c_uint = 0x8013	/* LIP reset occurred. */;
pub const MBA_PORT_UPDATE: c_uint = 0x8014	/* Port Database update. */;
pub const MBA_RSCN_UPDATE: c_uint = 0x8015	/* Register State Chg Notification. */;
pub const MBA_LIP_F8: c_uint = 0x8016	/* Received a LIP F8. */;
pub const MBA_LOOP_INIT_ERR: c_uint = 0x8017	/* Loop Initialization Error. */;
pub const MBA_FABRIC_AUTH_REQ: c_uint = 0x801b	/* Fabric Authentication Required. */;
pub const MBA_CONGN_NOTI_RECV: c_uint = 0x801e	/* Congestion Notification Received */;
pub const MBA_SCSI_COMPLETION: c_uint = 0x8020	/* SCSI Command Complete. */;
pub const MBA_CTIO_COMPLETION: c_uint = 0x8021	/* CTIO Complete. */;
pub const MBA_IP_COMPLETION: c_uint = 0x8022	/* IP Transmit Command Complete. */;
pub const MBA_IP_RECEIVE: c_uint = 0x8023	/* IP Received. */;
pub const MBA_IP_BROADCAST: c_uint = 0x8024	/* IP Broadcast Received. */;
pub const MBA_IP_LOW_WATER_MARK: c_uint = 0x8025	/* IP Low Water Mark reached. */;
pub const MBA_IP_RCV_BUFFER_EMPTY: c_uint = 0x8026	/* IP receive buffer queue empty. */;
pub const MBA_IP_HDR_DATA_SPLIT: c_uint = 0x8027	/* IP header/data splitting feature */;
// used.
pub const MBA_TRACE_NOTIFICATION: c_uint = 0x8028	/* Trace/Diagnostic notification. */;
pub const MBA_POINT_TO_POINT: c_uint = 0x8030	/* Point to point mode. */;
pub const MBA_CMPLT_1_16BIT: c_uint = 0x8031	/* Completion 1 16bit IOSB. */;
pub const MBA_CMPLT_2_16BIT: c_uint = 0x8032	/* Completion 2 16bit IOSB. */;
pub const MBA_CMPLT_3_16BIT: c_uint = 0x8033	/* Completion 3 16bit IOSB. */;
pub const MBA_CMPLT_4_16BIT: c_uint = 0x8034	/* Completion 4 16bit IOSB. */;
pub const MBA_CMPLT_5_16BIT: c_uint = 0x8035	/* Completion 5 16bit IOSB. */;
pub const MBA_CHG_IN_CONNECTION: c_uint = 0x8036	/* Change in connection mode. */;
pub const MBA_RIO_RESPONSE: c_uint = 0x8040	/* RIO response queue update. */;
pub const MBA_ZIO_RESPONSE: c_uint = 0x8040	/* ZIO response queue update. */;
pub const MBA_CMPLT_2_32BIT: c_uint = 0x8042	/* Completion 2 32bit IOSB. */;
pub const MBA_BYPASS_NOTIFICATION: c_uint = 0x8043	/* Auto bypass notification. */;
pub const MBA_DISCARD_RND_FRAME: c_uint = 0x8048	/* discard RND frame due to error. */;
pub const MBA_REJECTED_FCP_CMD: c_uint = 0x8049	/* rejected FCP_CMD. */;
pub const MBA_FW_NOT_STARTED: c_uint = 0x8050	/* Firmware not started */;
pub const MBA_FW_STARTING: c_uint = 0x8051	/* Firmware starting */;
pub const MBA_FW_RESTART_CMPLT: c_uint = 0x8060	/* Firmware restart complete */;
pub const MBA_INIT_REQUIRED: c_uint = 0x8061	/* Initialization required */;
pub const MBA_SHUTDOWN_REQUESTED: c_uint = 0x8062	/* Shutdown Requested */;
pub const MBA_TEMPERATURE_ALERT: c_uint = 0x8070	/* Temperature Alert */;
pub const MBA_DPORT_DIAGNOSTICS: c_uint = 0x8080	/* D-port Diagnostics */;
pub const MBA_TRANS_INSERT: c_uint = 0x8130	/* Transceiver Insertion */;
pub const MBA_TRANS_REMOVE: c_uint = 0x8131	/* Transceiver Removal */;
pub const MBA_FW_INIT_FAILURE: c_uint = 0x8401	/* Firmware initialization failure */;
pub const MBA_MIRROR_LUN_CHANGE: c_uint = 0x8402	/* Mirror LUN State Change;
pub const MBA_FW_POLL_STATE: c_uint = 0x8600  /* Firmware in poll diagnostic state */;
pub const MBA_FW_RESET_FCT: c_uint = 0x8502	/* Firmware reset factory defaults */;
pub const MBA_FW_INIT_INPROGRESS: c_uint = 0x8500	/* Firmware boot in progress */;
// 83XX FCoE specific
pub const MBA_IDC_AEN: c_uint = 0x8200  /* FCoE: NIC Core state change AEN */;
// Interrupt type codes
pub const INTR_ROM_MB_SUCCESS: c_uint = 0x1;
pub const INTR_ROM_MB_FAILED: c_uint = 0x2;
pub const INTR_MB_SUCCESS: c_uint = 0x10;
pub const INTR_MB_FAILED: c_uint = 0x11;
pub const INTR_ASYNC_EVENT: c_uint = 0x12;
pub const INTR_RSP_QUE_UPDATE: c_uint = 0x13;
pub const INTR_RSP_QUE_UPDATE_83XX: c_uint = 0x14;
pub const INTR_ATIO_QUE_UPDATE: c_uint = 0x1C;
pub const INTR_ATIO_RSP_QUE_UPDATE: c_uint = 0x1D;
pub const INTR_ATIO_QUE_UPDATE_27XX: c_uint = 0x1E;
// ISP mailbox loopback echo diagnostic error code
pub const MBS_LB_RESET: c_uint = 0x17;
// AEN mailbox Port Diagnostics test
pub const AEN_START_DIAG_TEST: c_uint = 0x0	/* start the diagnostics */;
pub const AEN_DONE_DIAG_TEST_WITH_NOERR: c_uint = 0x1	/* Done with no errors */;
pub const AEN_DONE_DIAG_TEST_WITH_ERR: c_uint = 0x2	/* Done with error.*/;
//
// Firmware options 1, 2, 3.
//

// 24XX additional firmware options
pub const ADD_FO_COUNT: c_int = 3;

//
// ISP mailbox commands
//

pub const MBC_DUMP_RISC_RAM: c_uint = 0xa	/* Dump RAM command. */;
pub const MBC_SECURE_FLASH_UPDATE: c_uint = 0xa	/* Secure Flash Update(28xx) */;
pub const MBC_RD_WR_FLASH: c_uint = 0xa	/* Read/write Dword/block Flash(29xx) */;
pub const MBC_LOAD_RISC_RAM_EXTENDED: c_uint = 0xb	/* Load RAM extended. */;
pub const MBC_DUMP_RISC_RAM_EXTENDED: c_uint = 0xc	/* Dump RAM extended. */;
pub const MBC_WRITE_RAM_WORD_EXTENDED: c_uint = 0xd	/* Write RAM word extended */;
pub const MBC_READ_RAM_EXTENDED: c_uint = 0xf	/* Read RAM extended. */;
pub const MBC_IOCB_COMMAND: c_uint = 0x12	/* Execute IOCB command. */;
pub const MBC_STOP_FIRMWARE: c_uint = 0x14	/* Stop firmware. */;
pub const MBC_ABORT_COMMAND: c_uint = 0x15	/* Abort IOCB command. */;
pub const MBC_ABORT_DEVICE: c_uint = 0x16	/* Abort device (ID/LUN). */;
pub const MBC_ABORT_TARGET: c_uint = 0x17	/* Abort target (ID). */;
pub const MBC_RESET: c_uint = 0x18	/* Reset. */;
pub const MBC_GET_ADAPTER_LOOP_ID: c_uint = 0x20	/* Get loop id of ISP2200. */;
pub const MBC_GET_SET_ZIO_THRESHOLD: c_uint = 0x21	/* Get/SET ZIO THRESHOLD. */;
pub const MBC_GET_RETRY_COUNT: c_uint = 0x22	/* Get f/w retry cnt/delay. */;
pub const MBC_DISABLE_VI: c_uint = 0x24	/* Disable VI operation. */;
pub const MBC_ENABLE_VI: c_uint = 0x25	/* Enable VI operation. */;
pub const MBC_GET_FIRMWARE_OPTION: c_uint = 0x28	/* Get Firmware Options. */;
pub const MBC_GET_MEM_OFFLOAD_CNTRL_STAT: c_uint = 0x34	/* Memory Offload ctrl/Stat*/;
pub const MBC_SET_FIRMWARE_OPTION: c_uint = 0x38	/* Set Firmware Options. */;
pub const MBC_SET_GET_FC_LED_CONFIG: c_uint = 0x3b	/* Set/Get FC LED config */;
pub const MBC_LOOP_PORT_BYPASS: c_uint = 0x40	/* Loop Port Bypass. */;
pub const MBC_LOOP_PORT_ENABLE: c_uint = 0x41	/* Loop Port Enable. */;
pub const MBC_GET_RESOURCE_COUNTS: c_uint = 0x42	/* Get Resource Counts. */;
pub const MBC_NON_PARTICIPATE: c_uint = 0x43	/* Non-Participating Mode. */;
pub const MBC_DIAGNOSTIC_ECHO: c_uint = 0x44	/* Diagnostic echo. */;
pub const MBC_DIAGNOSTIC_LOOP_BACK: c_uint = 0x45	/* Diagnostic loop back. */;
pub const MBC_ONLINE_SELF_TEST: c_uint = 0x46	/* Online self-test. */;
pub const MBC_ENHANCED_GET_PORT_DATABASE: c_uint = 0x47	/* Get port database + login */;
pub const MBC_CONFIGURE_VF: c_uint = 0x4b	/* Configure VFs */;
pub const MBC_RESET_LINK_STATUS: c_uint = 0x52	/* Reset Link Error Status */;
pub const MBC_IOCB_COMMAND_A64: c_uint = 0x54	/* Execute IOCB command (64) */;
pub const MBC_PORT_LOGOUT: c_uint = 0x56	/* Port Logout request */;
pub const MBC_SEND_RNID_ELS: c_uint = 0x57	/* Send RNID ELS request */;
pub const MBC_SET_RNID_PARAMS: c_uint = 0x59	/* Set RNID parameters */;
pub const MBC_GET_RNID_PARAMS: c_uint = 0x5a	/* Get RNID parameters */;
pub const MBC_DATA_RATE: c_uint = 0x5d	/* Data Rate */;
pub const MBC_INITIALIZE_FIRMWARE: c_uint = 0x60	/* Initialize firmware */;
pub const MBC_INITIATE_LIP: c_uint = 0x62	/* Initiate Loop */;
// Initialization Procedure
pub const MBC_GET_FC_AL_POSITION_MAP: c_uint = 0x63	/* Get FC_AL Position Map. */;
pub const MBC_GET_PORT_DATABASE: c_uint = 0x64	/* Get Port Database. */;
pub const MBC_CLEAR_ACA: c_uint = 0x65	/* Clear ACA. */;
pub const MBC_TARGET_RESET: c_uint = 0x66	/* Target Reset. */;
pub const MBC_CLEAR_TASK_SET: c_uint = 0x67	/* Clear Task Set. */;
pub const MBC_ABORT_TASK_SET: c_uint = 0x68	/* Abort Task Set. */;
pub const MBC_GET_FIRMWARE_STATE: c_uint = 0x69	/* Get firmware state. */;
pub const MBC_GET_PORT_NAME: c_uint = 0x6a	/* Get port name. */;
pub const MBC_GET_LINK_STATUS: c_uint = 0x6b	/* Get port link status. */;
pub const MBC_LIP_RESET: c_uint = 0x6c	/* LIP reset. */;
pub const MBC_SEND_SNS_COMMAND: c_uint = 0x6e	/* Send Simple Name Server */;
// commandd.
pub const MBC_LOGIN_FABRIC_PORT: c_uint = 0x6f	/* Login fabric port. */;
pub const MBC_SEND_CHANGE_REQUEST: c_uint = 0x70	/* Send Change Request. */;
pub const MBC_LOGOUT_FABRIC_PORT: c_uint = 0x71	/* Logout fabric port. */;
pub const MBC_LIP_FULL_LOGIN: c_uint = 0x72	/* Full login LIP. */;
pub const MBC_LOGIN_LOOP_PORT: c_uint = 0x74	/* Login Loop Port. */;
pub const MBC_PORT_NODE_NAME_LIST: c_uint = 0x75	/* Get port/node name list. */;
pub const MBC_INITIALIZE_RECEIVE_QUEUE: c_uint = 0x77	/* Initialize receive queue */;
pub const MBC_UNLOAD_IP: c_uint = 0x79	/* Shutdown IP */;
pub const MBC_GET_ID_LIST: c_uint = 0x7C	/* Get Port ID list. */;
pub const MBC_SEND_LFA_COMMAND: c_uint = 0x7D	/* Send Loop Fabric Address */;
pub const MBC_LUN_RESET: c_uint = 0x7E	/* Send LUN reset */;
//
// all the Mt. Rainier mailbox command codes that clash with FC/FCoE ones
// should be defined with MBC_MR_
//
pub const MBC_MR_DRV_SHUTDOWN: c_uint = 0x6A;
//
// ISP24xx mailbox commands
//
pub const MBC_WRITE_SERDES: c_uint = 0x3	/* Write serdes word. */;
pub const MBC_READ_SERDES: c_uint = 0x4	/* Read serdes word. */;
pub const MBC_LOAD_DUMP_MPI_RAM: c_uint = 0x5	/* Load/Dump MPI RAM. */;
pub const MBC_SERDES_PARAMS: c_uint = 0x10	/* Serdes Tx Parameters. */;
pub const MBC_GET_IOCB_STATUS: c_uint = 0x12	/* Get IOCB status command. */;
pub const MBC_PORT_PARAMS: c_uint = 0x1A	/* Port iDMA Parameters. */;
pub const MBC_GET_TIMEOUT_PARAMS: c_uint = 0x22	/* Get FW timeouts. */;
pub const MBC_TRACE_CONTROL: c_uint = 0x27	/* Trace control command. */;
pub const MBC_GEN_SYSTEM_ERROR: c_uint = 0x2a	/* Generate System Error. */;
pub const MBC_WRITE_SFP: c_uint = 0x30	/* Write SFP Data. */;
pub const MBC_READ_SFP: c_uint = 0x31	/* Read SFP Data. */;
pub const MBC_SET_TIMEOUT_PARAMS: c_uint = 0x32	/* Set FW timeouts. */;
pub const MBC_DPORT_DIAGNOSTICS: c_uint = 0x47	/* D-Port Diagnostics */;
pub const MBC_MID_INITIALIZE_FIRMWARE: c_uint = 0x48	/* MID Initialize firmware. */;
pub const MBC_MID_GET_VP_DATABASE: c_uint = 0x49	/* MID Get VP Database. */;
pub const MBC_MID_GET_VP_ENTRY: c_uint = 0x4a	/* MID Get VP Entry. */;
pub const MBC_HOST_MEMORY_COPY: c_uint = 0x53	/* Host Memory Copy. */;
pub const MBC_SEND_RNFT_ELS: c_uint = 0x5e	/* Send RNFT ELS request */;
pub const MBC_GET_LINK_PRIV_STATS: c_uint = 0x6d	/* Get link & private data. */;
pub const MBC_LINK_INITIALIZATION: c_uint = 0x72	/* Do link initialization. */;
pub const MBC_SET_VENDOR_ID: c_uint = 0x76	/* Set Vendor ID. */;
pub const MBC_PORT_RESET: c_uint = 0x120	/* Port Reset */;
pub const MBC_SET_PORT_CONFIG: c_uint = 0x122	/* Set port configuration */;
pub const MBC_GET_PORT_CONFIG: c_uint = 0x123	/* Get port configuration */;
//
// ISP81xx mailbox commands
//
pub const MBC_WRITE_MPI_REGISTER: c_uint = 0x01    /* Write MPI Register. */;
//
// ISP8044 mailbox commands
//
pub const MBC_SET_GET_ETH_SERDES_REG: c_uint = 0x150;
pub const HCS_WRITE_SERDES: c_uint = 0x3;
pub const HCS_READ_SERDES: c_uint = 0x4;
//
// ISP2[7|8]xx mailbox commands.
//
pub const MBC_MPI_PASSTHROUGH: c_uint = 0x200;
// MBC_MPI_PASSTHROUGH
pub const MPIPT_REQ_V1: c_int = 1;
// Firmware return data sizes
pub const FCAL_MAP_SIZE: c_int = 128;
// Mailbox bit definitions for out_mb and in_mb

pub const RNID_TYPE_ELS_CMD: c_uint = 0x5;
pub const RNID_TYPE_PORT_LOGIN: c_uint = 0x7;
pub const RNID_BUFFER_CREDITS: c_uint = 0x8;
pub const RNID_TYPE_SET_VERSION: c_uint = 0x9;
pub const RNID_TYPE_ASIC_TEMP: c_uint = 0xC;
pub const ELS_CMD_MAP_SIZE: c_int = 32;
//
// Firmware state codes from get firmware state mailbox command
//
pub const FSTATE_CONFIG_WAIT: c_int = 0;
pub const FSTATE_WAIT_AL_PA: c_int = 1;
pub const FSTATE_WAIT_LOGIN: c_int = 2;
pub const FSTATE_READY: c_int = 3;
pub const FSTATE_LOSS_OF_SYNC: c_int = 4;
pub const FSTATE_ERROR: c_int = 5;
pub const FSTATE_REINIT: c_int = 6;
pub const FSTATE_NON_PART: c_int = 7;
pub const FSTATE_CONFIG_CORRECT: c_int = 0;
pub const FSTATE_P2P_RCV_LIP: c_int = 1;
pub const FSTATE_P2P_CHOOSE_LOOP: c_int = 2;
pub const FSTATE_P2P_RCV_UNIDEN_LIP: c_int = 3;
pub const FSTATE_FATAL_ERROR: c_int = 4;
pub const FSTATE_LOOP_BACK_CONN: c_int = 5;
pub const QLA27XX_IMG_STATUS_VER_MAJOR: c_uint = 0x01;
pub const QLA27XX_IMG_STATUS_VER_MINOR: c_uint = 0x00;
pub const QLA27XX_IMG_STATUS_SIGN: c_uint = 0xFACEFADE;
pub const QLA28XX_IMG_STATUS_SIGN: c_uint = 0xFACEFADF;
pub const QLA28XX_IMG_STATUS_SIGN: c_uint = 0xFACEFADF;
pub const QLA28XX_AUX_IMG_STATUS_SIGN: c_uint = 0xFACEFAED;
pub const QLA27XX_DEFAULT_IMAGE: c_int = 0;
pub const QLA27XX_PRIMARY_IMAGE: c_int = 1;
pub const QLA27XX_SECONDARY_IMAGE: c_int = 2;
//
// Port Database structure definition
// Little endian except where noted.
//

// Bits 15-0 of word 0
// Bits 15-0 of word 3
//
// Port database slave/master states
//
pub const PD_STATE_DISCOVERY: c_int = 0;
pub const PD_STATE_WAIT_DISCOVERY_ACK: c_int = 1;
pub const PD_STATE_PORT_LOGIN: c_int = 2;
pub const PD_STATE_WAIT_PORT_LOGIN_ACK: c_int = 3;
pub const PD_STATE_PROCESS_LOGIN: c_int = 4;
pub const PD_STATE_WAIT_PROCESS_LOGIN_ACK: c_int = 5;
pub const PD_STATE_PORT_LOGGED_IN: c_int = 6;
pub const PD_STATE_PORT_UNAVAILABLE: c_int = 7;
pub const PD_STATE_PROCESS_LOGOUT: c_int = 8;
pub const PD_STATE_WAIT_PROCESS_LOGOUT_ACK: c_int = 9;
pub const PD_STATE_PORT_LOGOUT: c_int = 10;
pub const PD_STATE_WAIT_PORT_LOGOUT_ACK: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qla29xx_mpi_optrom_op {
    QLA29XX_MPI_OP_DUMP,
    QLA29XX_MPI_OP_LOAD,
}

pub const QLA_ZIO_DISABLED: c_int = 0;
pub const QLA_ZIO_DEFAULT_TIMER: c_int = 2;
//
// ISP Initialization Control Block.
// Little endian except where noted.
//
pub const ICB_VERSION: c_int = 1;
//
// LSB BIT 0  = Enable Hard Loop Id
// LSB BIT 1  = Enable Fairness
// LSB BIT 2  = Enable Full-Duplex
// LSB BIT 3  = Enable Fast Posting
// LSB BIT 4  = Enable Target Mode
// LSB BIT 5  = Disable Initiator Mode
// LSB BIT 6  = Enable ADISC
// LSB BIT 7  = Enable Target Inquiry Data
//
// MSB BIT 0  = Enable PDBC Notify
// MSB BIT 1  = Non Participating LIP
// MSB BIT 2  = Descending Loop ID Search
// MSB BIT 3  = Acquire Loop ID in LIPA
// MSB BIT 4  = Stop PortQ on Full Status
// MSB BIT 5  = Full Login after LIP
// MSB BIT 6  = Node Name Option
// MSB BIT 7  = Ext IFWCB enable bit
//
// LSB BIT 0 = Timer Operation mode bit 0
// LSB BIT 1 = Timer Operation mode bit 1
// LSB BIT 2 = Timer Operation mode bit 2
// LSB BIT 3 = Timer Operation mode bit 3
// LSB BIT 4 = Init Config Mode bit 0
// LSB BIT 5 = Init Config Mode bit 1
// LSB BIT 6 = Init Config Mode bit 2
// LSB BIT 7 = Enable Non part on LIHA failure
//
// MSB BIT 0 = Enable class 2
// MSB BIT 1 = Enable ACK0
// MSB BIT 2 =
// MSB BIT 3 =
// MSB BIT 4 = FC Tape Enable
// MSB BIT 5 = Enable FC Confirm
// MSB BIT 6 = Enable command queuing in target mode
// MSB BIT 7 = No Logo On Link Down
//
// LSB BIT 0 = Enable Read xfr_rdy
// LSB BIT 1 = Soft ID only
// LSB BIT 2 =
// LSB BIT 3 =
// LSB BIT 4 = FCP RSP Payload [0]
// LSB BIT 5 = FCP RSP Payload [1] / Sbus enable - 2200
// LSB BIT 6 = Enable Out-of-Order frame handling
// LSB BIT 7 = Disable Automatic PLOGI on Local Loop
//
// MSB BIT 0 = Sbus enable - 2300
// MSB BIT 1 =
// MSB BIT 2 =
// MSB BIT 3 =
// MSB BIT 4 = LED mode
// MSB BIT 5 = enable 50 ohm termination
// MSB BIT 6 = Data Rate (2300 only)
// MSB BIT 7 = Data Rate (2300 only)
//
// Special Features Control Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_sf_cb {
    pub format: u8,
    pub reserved0: u8,
//
// BIT 15-14 = Reserved
// BIT_13 = SAN Congestion Management (1 - Enabled, 0 - Disabled)
// BIT_12 = Remote Write Optimization (1 - Enabled, 0 - Disabled)
// BIT 11-0 = Reserved
//
    pub flags: __le16,
    pub reserved1: [u8; 32],
    pub discard_OHRB_timeout_value: u16,
    pub remote_write_opt_queue_num: u16,
    pub reserved2: [u8; 40],
    pub scm_related_parameter: [u8; 16],
    pub reserved3: [u8; 32],
}

//
// Get Link Status mailbox command return buffer.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_statistics {
    pub link_fail_cnt: __le32,
    pub loss_sync_cnt: __le32,
    pub loss_sig_cnt: __le32,
    pub prim_seq_err_cnt: __le32,
    pub inval_xmit_word_cnt: __le32,
    pub inval_crc_cnt: __le32,
    pub lip_cnt: __le32,
    pub link_up_cnt: __le32,
    pub link_down_loop_init_tmo: __le32,
    pub link_down_los: __le32,
    pub link_down_loss_rcv_clk: __le32,
    pub reserved0: [u32; 5],
    pub port_cfg_chg: __le32,
    pub reserved1: [u32; 11],
    pub rsp_q_full: __le32,
    pub atio_q_full: __le32,
    pub drop_ae: __le32,
    pub els_proto_err: __le32,
    pub reserved2: __le32,
    pub tx_frames: __le32,
    pub rx_frames: __le32,
    pub discarded_frames: __le32,
    pub dropped_frames: __le32,
    pub reserved3: u32,
    pub nos_rcvd: __le32,
    pub reserved4: [u32; 4],
    pub tx_prjt: __le32,
    pub rcv_exfail: __le32,
    pub rcv_abts: __le32,
    pub seq_frm_miss: __le32,
    pub corr_err: __le32,
    pub mb_rqst: __le32,
    pub nport_full: __le32,
    pub eofa: __le32,
    pub reserved5: u32,
    pub fpm_recv_word_cnt: __le64,
    pub fpm_disc_word_cnt: __le64,
    pub fpm_xmit_word_cnt: __le64,
    pub reserved6: [u32; 70],
}

//
// NVRAM Command values.
//

pub const NV_DELAY_COUNT: c_int = 10;
//
// QLogic ISP2100, ISP2200 and ISP2300 NVRAM structure definition.
//
// NVRAM header
//
// NVRAM RISC parameter block
//
// LSB BIT 0  = Enable Hard Loop Id
// LSB BIT 1  = Enable Fairness
// LSB BIT 2  = Enable Full-Duplex
// LSB BIT 3  = Enable Fast Posting
// LSB BIT 4  = Enable Target Mode
// LSB BIT 5  = Disable Initiator Mode
// LSB BIT 6  = Enable ADISC
// LSB BIT 7  = Enable Target Inquiry Data
//
// MSB BIT 0  = Enable PDBC Notify
// MSB BIT 1  = Non Participating LIP
// MSB BIT 2  = Descending Loop ID Search
// MSB BIT 3  = Acquire Loop ID in LIPA
// MSB BIT 4  = Stop PortQ on Full Status
// MSB BIT 5  = Full Login after LIP
// MSB BIT 6  = Node Name Option
// MSB BIT 7  = Ext IFWCB enable bit
//
// LSB BIT 0 = Timer Operation mode bit 0
// LSB BIT 1 = Timer Operation mode bit 1
// LSB BIT 2 = Timer Operation mode bit 2
// LSB BIT 3 = Timer Operation mode bit 3
// LSB BIT 4 = Init Config Mode bit 0
// LSB BIT 5 = Init Config Mode bit 1
// LSB BIT 6 = Init Config Mode bit 2
// LSB BIT 7 = Enable Non part on LIHA failure
//
// MSB BIT 0 = Enable class 2
// MSB BIT 1 = Enable ACK0
// MSB BIT 2 =
// MSB BIT 3 =
// MSB BIT 4 = FC Tape Enable
// MSB BIT 5 = Enable FC Confirm
// MSB BIT 6 = Enable command queuing in target mode
// MSB BIT 7 = No Logo On Link Down
//
// LSB BIT 0 = Enable Read xfr_rdy
// LSB BIT 1 = Soft ID only
// LSB BIT 2 =
// LSB BIT 3 =
// LSB BIT 4 = FCP RSP Payload [0]
// LSB BIT 5 = FCP RSP Payload [1] / Sbus enable - 2200
// LSB BIT 6 = Enable Out-of-Order frame handling
// LSB BIT 7 = Disable Automatic PLOGI on Local Loop
//
// MSB BIT 0 = Sbus enable - 2300
// MSB BIT 1 =
// MSB BIT 2 =
// MSB BIT 3 =
// MSB BIT 4 = LED mode
// MSB BIT 5 = enable 50 ohm termination
// MSB BIT 6 = Data Rate (2300 only)
// MSB BIT 7 = Data Rate (2300 only)
//
// Reserved for expanded RISC parameter block
//
// LSB BIT 0 = Tx Sensitivity 1G bit 0
// LSB BIT 1 = Tx Sensitivity 1G bit 1
// LSB BIT 2 = Tx Sensitivity 1G bit 2
// LSB BIT 3 = Tx Sensitivity 1G bit 3
// LSB BIT 4 = Rx Sensitivity 1G bit 0
// LSB BIT 5 = Rx Sensitivity 1G bit 1
// LSB BIT 6 = Rx Sensitivity 1G bit 2
// LSB BIT 7 = Rx Sensitivity 1G bit 3
//
// MSB BIT 0 = Tx Sensitivity 2G bit 0
// MSB BIT 1 = Tx Sensitivity 2G bit 1
// MSB BIT 2 = Tx Sensitivity 2G bit 2
// MSB BIT 3 = Tx Sensitivity 2G bit 3
// MSB BIT 4 = Rx Sensitivity 2G bit 0
// MSB BIT 5 = Rx Sensitivity 2G bit 1
// MSB BIT 6 = Rx Sensitivity 2G bit 2
// MSB BIT 7 = Rx Sensitivity 2G bit 3
//
// LSB BIT 0 = Output Swing 1G bit 0
// LSB BIT 1 = Output Swing 1G bit 1
// LSB BIT 2 = Output Swing 1G bit 2
// LSB BIT 3 = Output Emphasis 1G bit 0
// LSB BIT 4 = Output Emphasis 1G bit 1
// LSB BIT 5 = Output Swing 2G bit 0
// LSB BIT 6 = Output Swing 2G bit 1
// LSB BIT 7 = Output Swing 2G bit 2
//
// MSB BIT 0 = Output Emphasis 2G bit 0
// MSB BIT 1 = Output Emphasis 2G bit 1
// MSB BIT 2 = Output Enable
// MSB BIT 3 =
// MSB BIT 4 =
// MSB BIT 5 =
// MSB BIT 6 =
// MSB BIT 7 =
//
// NVRAM host parameter block
//
// LSB BIT 0 = Enable spinup delay
// LSB BIT 1 = Disable BIOS
// LSB BIT 2 = Enable Memory Map BIOS
// LSB BIT 3 = Enable Selectable Boot
// LSB BIT 4 = Disable RISC code load
// LSB BIT 5 = Set cache line size 1
// LSB BIT 6 = PCI Parity Disable
// LSB BIT 7 = Enable extended logging
//
// MSB BIT 0 = Enable 64bit addressing
// MSB BIT 1 = Enable lip reset
// MSB BIT 2 = Enable lip full login
// MSB BIT 3 = Enable target reset
// MSB BIT 4 = Enable database storage
// MSB BIT 5 = Enable cache flush read
// MSB BIT 6 = Enable database load
// MSB BIT 7 = Enable alternate WWN
//
// BIT 0 = Selective Login
// BIT 1 = Alt-Boot Enable
// BIT 2 =
// BIT 3 = Boot Order List
// BIT 4 =
// BIT 5 = Selective LUN
// BIT 6 =
// BIT 7 = unused
//
// Offset 200-215 : Model Number
// OEM related items
//
// NVRAM Adapter Features offset 232-239
//
// LSB BIT 0 = External GBIC
// LSB BIT 1 = Risc RAM parity
// LSB BIT 2 = Buffer Plus Module
// LSB BIT 3 = Multi Chip Adapter
// LSB BIT 4 = Internal connector
// LSB BIT 5 =
// LSB BIT 6 =
// LSB BIT 7 =
//
// MSB BIT 0 =
// MSB BIT 1 =
// MSB BIT 2 =
// MSB BIT 3 =
// MSB BIT 4 =
// MSB BIT 5 =
// MSB BIT 6 =
// MSB BIT 7 =
//
// Subsystem vendor ID for ISP2200
// Subsystem device ID for ISP2200
//
// ISP queue - response queue entry definition.
//
pub const RESPONSE_PROCESSED: c_uint = 0xDEADDEAD	/* Signature */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct response_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System defined handle,
    pub data: [u8; 52],
    pub signature: u32,
    pub reserved: [u8; 64],
pub const RESPONSE_PROCESSED: c_uint = 0xDEADDEAD	/* Signature */;
}

//
// ISP queue - ATIO queue entry definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atio {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub attr_n_length: __le16,
    pub data: [u8; 56],
    pub signature: u32,
pub const ATIO_PROCESSED: c_uint = 0xDEADDEAD		/* Signature */;
}

//
// ISP queue - command entry structure definition.
//
pub const COMMAND_TYPE: c_uint = 0x11		/* Command entry */;

//
// ISP queue - 64-Bit addressing, command entry structure definition.
//
pub const COMMAND_A64_TYPE: c_uint = 0x19	/* Command A64 entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub /: *mut *mut target_id_t target; / SCSI ID,
    pub /: *mut *mut __le16 lun; / SCSI LUN,
    pub /: *mut *mut __le16 control_flags; / Control flags.,
    pub reserved_1: u16,
    pub /: *mut *mut __le16 timeout; / Command timeout.,
    pub /: *mut *mut __le16 dseg_count; / Data segment count.,
    pub /: *mut *mut uint8_t scsi_cdb[MAX_CMDSZ]; / SCSI command words.,
    pub /: *mut *mut uint32_t byte_count; / Total byte count.,
    pub dsd: [dsd64; 2],
    pub reserved: [u8; 64],
}

//
// ISP queue - continuation entry structure definition.
//
pub const CONTINUE_TYPE: c_uint = 0x02	/* Continuation entry. */;
//
// ISP queue - 64-Bit addressing, continuation entry structure definition.
//
pub const CONTINUE_A64_TYPE: c_uint = 0x0A	/* Continuation A64 entry. */;
pub const PO_MODE_DIF_INSERT: c_int = 0;
pub const PO_MODE_DIF_REMOVE: c_int = 1;
pub const PO_MODE_DIF_PASS: c_int = 2;
pub const PO_MODE_DIF_REPLACE: c_int = 3;
pub const PO_MODE_DIF_TCP_CKSUM: c_int = 6;

//
// ISP queue - 64-Bit addressing, continuation crc entry structure definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_context {
    pub /: *mut *mut uint32_t handle; / System handle.,
    pub ref_tag: __le32,
    pub app_tag: __le16,
    pub Mask*/: *mut *mut uint8_t ref_tag_mask[4]; / Validation/Replacement,
    pub Mask*/: *mut *mut uint8_t app_tag_mask[2]; / Validation/Replacement,
    pub /: *mut *mut __le16 guard_seed; / Initial Guard Seed,
    pub /: *mut *mut __le16 prot_opts; / Requested Data Protection Mode,
    pub /: *mut *mut __le16 blk_size; / Data size in bytes,
    pub (tape: *mut *mut __le16 runt_blk_guard; / Guard value for runt block,
// only)
    pub data: *mut *mut __le32 byte_count; / Total byte count/ total,
// transfer count
    pub reserved_1: u32,
    pub reserved_2: u16,
    pub reserved_3: u16,
    pub reserved_4: u32,
    pub data_dsd: [dsd64; 1],
    pub reserved_5: [u32; 2],
    pub reserved_6: u32,
    pub nobundling: },
    pub byte: *mut *mut __le32 dif_byte_count; / Total DIF,
// count
    pub reserved_1: u16,
    pub /: *mut *mut __le16 dseg_count; / Data segment count,
    pub reserved_2: u32,
    pub data_dsd: [dsd64; 1],
    pub dif_dsd: dsd64,
    pub bundling: },
    pub u: },
    pub fcp_cmnd: fcp_cmnd,
    pub crc_ctx_dma: dma_addr_t,
// List of DMA context transfers
    pub dsd_list: list_head,
// List of DIF Bundling context DMA address
    pub ldif_dsd_list: list_head,
    pub no_ldif_dsd: u8,
    pub ldif_dma_hndl_list: list_head,
    pub dif_bundl_len: u32,
    pub no_dif_bundl: u8,
// This structure should not exceed 512 bytes
}

//
// ISP queue - status entry structure definition.
//
pub const STATUS_TYPE: c_uint = 0x03		/* Status entry. */;
//
// Status entry entry status
//

//
// Status entry SCSI status bit definitions.
//
pub const SS_MASK: c_uint = 0xfff	/* Reserved bits BIT_12-BIT_15*/;

pub const SS_SCSI_STATUS_BYTE: c_uint = 0xff;

//
// Status entry completion status
//
pub const CS_COMPLETE: c_uint = 0x0	/* No errors */;
pub const CS_INCOMPLETE: c_uint = 0x1	/* Incomplete transfer of cmd. */;
pub const CS_DMA: c_uint = 0x2	/* A DMA direction error. */;
pub const CS_TRANSPORT: c_uint = 0x3	/* Transport error. */;
pub const CS_RESET: c_uint = 0x4	/* SCSI bus reset occurred */;
pub const CS_ABORTED: c_uint = 0x5	/* System aborted command. */;
pub const CS_TIMEOUT: c_uint = 0x6	/* Timeout error. */;
pub const CS_DATA_OVERRUN: c_uint = 0x7	/* Data overrun. */;
pub const CS_DIF_ERROR: c_uint = 0xC	/* DIF error detected  */;
pub const CS_DATA_UNDERRUN: c_uint = 0x15	/* Data Underrun. */;
pub const CS_QUEUE_FULL: c_uint = 0x1C	/* Queue Full. */;
pub const CS_PORT_UNAVAILABLE: c_uint = 0x28	/* Port unavailable */;
// (selection timeout)
pub const CS_PORT_LOGGED_OUT: c_uint = 0x29	/* Port Logged Out */;
pub const CS_PORT_CONFIG_CHG: c_uint = 0x2A	/* Port Configuration Changed */;
pub const CS_PORT_BUSY: c_uint = 0x2B	/* Port Busy */;
pub const CS_COMPLETE_CHKCOND: c_uint = 0x30	/* Error? */;
pub const CS_IOCB_ERROR: c_uint = 0x31	/* Generic error for IOCB request;
pub const CS_REJECT_RECEIVED: c_uint = 0x4E	/* Reject received */;
pub const CS_EDIF_AUTH_ERROR: c_uint = 0x63	/* decrypt error */;
pub const CS_EDIF_PAD_LEN_ERROR: c_uint = 0x65	/* pad > frame size, not 4byte align */;
pub const CS_EDIF_INV_REQ: c_uint = 0x66	/* invalid request */;
pub const CS_EDIF_SPI_ERROR: c_uint = 0x67	/* rx frame unable to locate sa */;
pub const CS_EDIF_HDR_ERROR: c_uint = 0x69	/* data frame != expected len */;
pub const CS_BAD_PAYLOAD: c_uint = 0x80	/* Driver defined */;
pub const CS_UNKNOWN: c_uint = 0x81	/* Driver defined */;
pub const CS_RETRY: c_uint = 0x82	/* Driver defined */;
pub const CS_LOOP_DOWN_ABORT: c_uint = 0x83	/* Driver defined */;
pub const CS_BIDIR_RD_OVERRUN: c_uint = 0x700;
pub const CS_BIDIR_RD_WR_OVERRUN: c_uint = 0x707;
pub const CS_BIDIR_RD_OVERRUN_WR_UNDERRUN: c_uint = 0x715;
pub const CS_BIDIR_RD_UNDERRUN: c_uint = 0x1500;
pub const CS_BIDIR_RD_UNDERRUN_WR_OVERRUN: c_uint = 0x1507;
pub const CS_BIDIR_RD_WR_UNDERRUN: c_uint = 0x1515;
pub const CS_BIDIR_DMA: c_uint = 0x200;
//
// Status entry status flags
//

//
// ISP queue - status continuation entry structure definition.
//
pub const STATUS_CONT_TYPE: c_uint = 0x10	/* Status continuation entry. */;
//
// ISP queue -	RIO Type 1 status entry (32 bit I/O entry handles)
// structure definition.
//
pub const STATUS_TYPE_21: c_uint = 0x21		/* Status entry. */;
//
// ISP queue -	RIO Type 2 status entry (16 bit I/O entry handles)
// structure definition.
//
pub const STATUS_TYPE_22: c_uint = 0x22		/* Status entry. */;
//
// ISP queue - marker entry structure definition.
//
pub const MARKER_TYPE: c_uint = 0x04		/* Marker entry. */;

// clear port changed,
// use sequence number.
// 29xx definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sts_cont_entry_ext {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint8_t data[124]; / data,
}

//
// ISP queue - Management Server entry structure definition.
//
pub const MS_IOCB_TYPE: c_uint = 0x29	/* Management Server IOCB entry */;

//
// ISP queue - Mailbox Command entry structure definition.
//
pub const MBX_IOCB_TYPE: c_uint = 0x39;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_entry {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_define1: u8,
// Use sys_define1 for source type
pub const SOURCE_SCSI: c_uint = 0x00;
pub const SOURCE_IP: c_uint = 0x01;
pub const SOURCE_VI: c_uint = 0x02;
pub const SOURCE_SCTP: c_uint = 0x03;
pub const SOURCE_MP: c_uint = 0x04;
pub const SOURCE_MPIOCTL: c_uint = 0x05;
pub const SOURCE_ASYNC_IOCB: c_uint = 0x07;
    pub entry_status: u8,
    pub handle: u32,
    pub loop_id: target_id_t,
    pub status: __le16,
    pub state_flags: __le16,
    pub status_flags: __le16,
    pub sys_define2: [u32; 2],
    pub mb0: __le16,
    pub mb1: __le16,
    pub mb2: __le16,
    pub mb3: __le16,
    pub mb6: __le16,
    pub mb7: __le16,
    pub mb9: __le16,
    pub mb10: __le16,
    pub reserved_2: [u32; 2],
    pub node_name: [u8; WWN_SIZE],
    pub port_name: [u8; WWN_SIZE],
}

pub const IMMED_NOTIFY_TYPE: c_uint = 0x0D		/* Immediate notify entry. */;
//
// ISP queue -	immediate notify entry structure definition.
// This is sent by the ISP to the Target driver.
// This IOCB would have report of events sent by the
// initiator, that needs to be handled by the target
// driver immediately.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imm_ntfy_from_isp {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut __le32 sys_define_2; / System defined.,
    pub target: target_id_t,
    pub lun: __le16,
    pub target_id: u8,
    pub reserved_1: u8,
    pub status_modifier: __le16,
    pub status: __le16,
    pub task_flags: __le16,
    pub seq_id: __le16,
    pub srr_rx_id: __le16,
    pub srr_rel_offs: __le32,
    pub srr_ui: __le16,
pub const SRR_IU_DATA_IN: c_uint = 0x1;
pub const SRR_IU_DATA_OUT: c_uint = 0x5;
pub const SRR_IU_STATUS: c_uint = 0x7;
    pub srr_ox_id: __le16,
    pub reserved_2: [u8; 28],
    pub isp2x: },
    pub reserved: u32,
    pub nport_handle: __le16,
    pub reserved_2: u16,
    pub flags: __le16,

    pub srr_rx_id: __le16,
    pub status: __le16,
    pub status_subcode: u8,
    pub fw_handle: u8,
    pub exchange_address: __le32,
    pub srr_rel_offs: __le32,
    pub srr_ui: __le16,
    pub srr_ox_id: __le16,
    pub node_name: [u8; 8],
    pub /: *mut *mut } plogi; / PLOGI/ADISC/PDISC,
// PRLI word 3 bit 0-15
    pub wd3_lo: __le16,
    pub resv0: [u8; 6],
    pub prli: },
    pub port_id: [u8; 3],
    pub resv1: u8,
    pub nport_handle: __le16,
    pub resv2: u16,
    pub req_els: },
    pub u: },
    pub port_name: [u8; 8],
    pub resv3: [u8; 3],
    pub vp_index: u8,
    pub reserved_5: u32,
    pub port_id: [u8; 3],
    pub reserved_6: u8,
    pub isp24: },
    pub u: },
    pub reserved_7: u16,
    pub ox_id: __le16,
    pub __packed: },

//
// ISP request and response queue entry sizes
//

//
// 29xx (qla29xx) uses 128-byte ring entries for both request and response
// queues.  These macros give the size of an extended IOCB slot and are
// used when allocating from / zeroing the 29xx request ring via ring_ext_ptr.
//

//
// Switch info gathering structure.
//
    pub d_id: port_id_t,
    pub node_name: [u8; WWN_SIZE],
    pub port_name: [u8; WWN_SIZE],
    pub fabric_port_name: [u8; WWN_SIZE],
    pub fp_speed: u16,
    pub fc4_type: u8,
    pub fc4_features: u8,
    pub sw_info_t: },
// FCP-4 types
pub const FC4_TYPE_FCP_SCSI: c_uint = 0x08;
pub const FC4_TYPE_NVME: c_uint = 0x28;
pub const FC4_TYPE_OTHER: c_uint = 0x0;
pub const FC4_TYPE_UNKNOWN: c_uint = 0xff;
// mailbox command 4G & above
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_24xx_entry {
    pub entry_type: u8,
    pub entry_count: u8,
    pub sys_define1: u8,
    pub entry_status: u8,
    pub handle: u32,
    pub mb: [u16; 28],
}

pub const IOCB_SIZE: c_int = 64;
//
// Fibre channel port type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qla_sess_deletion {
    QLA_SESS_DELETION_NONE		= 0,
    QLA_SESS_DELETION_IN_PROGRESS,
    QLA_SESS_DELETED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlt_plogi_link_t {
    QLT_PLOGI_LINK_SAME_WWN,
    QLT_PLOGI_LINK_CONFLICT,
    QLT_PLOGI_LINK_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlt_plogi_ack_t {
    pub list: list_head,
    pub iocb: imm_ntfy_from_isp,
    pub id: port_id_t,
    pub ref_count: c_int,
    pub fcport: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_desc {
    pub ct_sns: *mut ct_sns_pkt,
    pub ct_sns_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum discovery_state {
    DSC_DELETED,
    DSC_GNL,
    DSC_LOGIN_PEND,
    DSC_LOGIN_FAILED,
    DSC_GPDB,
    DSC_UPD_FCPORT,
    DSC_LOGIN_COMPLETE,
    DSC_ADISC,
    DSC_DELETE_PEND,
    DSC_LOGIN_AUTH_PEND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum login_state {
    DSC_LS_LLIOCB_SENT = 2,
    DSC_LS_PLOGI_PEND,
    DSC_LS_PLOGI_COMP,
    DSC_LS_PRLI_PEND,
    DSC_LS_PRLI_COMP,
    DSC_LS_PORT_UNAVAIL,
    DSC_LS_PRLO_PEND = 9,
    DSC_LS_LOGO_PEND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rscn_addr_format {
    RSCN_PORT_ADDR,
    RSCN_AREA_ADDR,
    RSCN_DOM_ADDR,
    RSCN_FAB_ADDR,
}

//
// Fibre channel port structure.
//
// Serializes unsol_ctx_head against ISR, DPC and NVMe transport.

pub const NVME_FLAG_REGISTERED: c_int = 4;
pub const NVME_FLAG_DELETING: c_int = 2;
pub const NVME_FLAG_RESETTING: c_int = 1;
//
// EDIF parameters for encryption.
//
pub const QLA_FCPORT_SCAN: c_int = 1;
pub const QLA_FCPORT_FOUND: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_arg {
    pub fcport: *mut fc_port_t,
    pub sp: *mut srb_t,
    pub id: port_id_t,
    pub rc: u16 data[2],,
    pub port_name: [u8; WWN_SIZE],
    pub iop: [u32; 2],
}

//
// Fibre channel port/lun states.
//
// FC port flags.
//

// No loop ID flag.
pub const FC_NO_LOOP_ID: c_uint = 0x1000;
//
// FC-CT interface
//
// NOTE: All structures are big-endian in form.
//
pub const CT_REJECT_RESPONSE: c_uint = 0x8001;
pub const CT_ACCEPT_RESPONSE: c_uint = 0x8002;
pub const CT_REASON_INVALID_COMMAND_CODE: c_uint = 0x01;
pub const CT_REASON_CANNOT_PERFORM: c_uint = 0x09;
pub const CT_REASON_COMMAND_UNSUPPORTED: c_uint = 0x0b;
pub const CT_EXPL_ALREADY_REGISTERED: c_uint = 0x10;
pub const CT_EXPL_HBA_ATTR_NOT_REGISTERED: c_uint = 0x11;
pub const CT_EXPL_MULTIPLE_HBA_ATTR: c_uint = 0x12;
pub const CT_EXPL_INVALID_HBA_BLOCK_LENGTH: c_uint = 0x13;
pub const CT_EXPL_MISSING_REQ_HBA_ATTR: c_uint = 0x14;
pub const CT_EXPL_PORT_NOT_REGISTERED_: c_uint = 0x15;
pub const CT_EXPL_MISSING_HBA_ID_PORT_LIST: c_uint = 0x16;
pub const CT_EXPL_HBA_NOT_REGISTERED: c_uint = 0x17;
pub const CT_EXPL_PORT_ATTR_NOT_REGISTERED: c_uint = 0x20;
pub const CT_EXPL_PORT_NOT_REGISTERED: c_uint = 0x21;
pub const CT_EXPL_MULTIPLE_PORT_ATTR: c_uint = 0x22;
pub const CT_EXPL_INVALID_PORT_BLOCK_LENGTH: c_uint = 0x23;
pub const NS_N_PORT_TYPE: c_uint = 0x01;
pub const NS_NL_PORT_TYPE: c_uint = 0x02;
pub const NS_NX_PORT_TYPE: c_uint = 0x7F;
pub const GA_NXT_CMD: c_uint = 0x100;

pub const GPN_FT_CMD: c_uint = 0x172;

pub const GNN_FT_CMD: c_uint = 0x173;

pub const GID_PT_CMD: c_uint = 0x1A1;

pub const GPN_ID_CMD: c_uint = 0x112;

pub const GNN_ID_CMD: c_uint = 0x113;

pub const GFT_ID_CMD: c_uint = 0x117;

pub const GID_PN_CMD: c_uint = 0x121;

pub const RFT_ID_CMD: c_uint = 0x217;

pub const RFT_ID_RSP_SIZE: c_int = 16;
pub const RFF_ID_CMD: c_uint = 0x21F;

pub const RFF_ID_RSP_SIZE: c_int = 16;
pub const RNN_ID_CMD: c_uint = 0x213;

pub const RNN_ID_RSP_SIZE: c_int = 16;
pub const RSNN_NN_CMD: c_uint = 0x239;

pub const RSNN_NN_RSP_SIZE: c_int = 16;
pub const GFPN_ID_CMD: c_uint = 0x11C;

pub const GPSC_CMD: c_uint = 0x127;

pub const GFF_ID_CMD: c_uint = 0x011F;

//
// FDMI HBA attribute types.
//
pub const FDMI1_HBA_ATTR_COUNT: c_int = 10;
pub const FDMI2_HBA_ATTR_COUNT: c_int = 17;
pub const FDMI_HBA_NODE_NAME: c_uint = 0x1;
pub const FDMI_HBA_MANUFACTURER: c_uint = 0x2;
pub const FDMI_HBA_SERIAL_NUMBER: c_uint = 0x3;
pub const FDMI_HBA_MODEL: c_uint = 0x4;
pub const FDMI_HBA_MODEL_DESCRIPTION: c_uint = 0x5;
pub const FDMI_HBA_HARDWARE_VERSION: c_uint = 0x6;
pub const FDMI_HBA_DRIVER_VERSION: c_uint = 0x7;
pub const FDMI_HBA_OPTION_ROM_VERSION: c_uint = 0x8;
pub const FDMI_HBA_FIRMWARE_VERSION: c_uint = 0x9;
pub const FDMI_HBA_OS_NAME_AND_VERSION: c_uint = 0xa;
pub const FDMI_HBA_MAXIMUM_CT_PAYLOAD_LENGTH: c_uint = 0xb;
pub const FDMI_HBA_NODE_SYMBOLIC_NAME: c_uint = 0xc;
pub const FDMI_HBA_VENDOR_SPECIFIC_INFO: c_uint = 0xd;
pub const FDMI_HBA_NUM_PORTS: c_uint = 0xe;
pub const FDMI_HBA_FABRIC_NAME: c_uint = 0xf;
pub const FDMI_HBA_BOOT_BIOS_NAME: c_uint = 0x10;
pub const FDMI_HBA_VENDOR_IDENTIFIER: c_uint = 0xe0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_fdmi_hba_attr {
    pub type: __be16,
    pub len: __be16,
    pub node_name: [u8; WWN_SIZE],
    pub manufacturer: [u8; 64],
    pub serial_num: [u8; 32],
    pub model: [u8; 16+1],
    pub model_desc: [u8; 80],
    pub hw_version: [u8; 32],
    pub driver_version: [u8; 32],
    pub orom_version: [u8; 16],
    pub fw_version: [u8; 32],
    pub os_version: [u8; 128],
    pub max_ct_len: __be32,
    pub sym_name: [u8; 256],
    pub vendor_specific_info: __be32,
    pub num_ports: __be32,
    pub fabric_name: [u8; WWN_SIZE],
    pub bios_name: [u8; 32],
    pub vendor_identifier: [u8; 8],
    pub a: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_fdmi1_hba_attributes {
    pub count: __be32,
    pub entry: [ct_fdmi_hba_attr; FDMI1_HBA_ATTR_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_fdmi2_hba_attributes {
    pub count: __be32,
    pub entry: [ct_fdmi_hba_attr; FDMI2_HBA_ATTR_COUNT],
}

//
// FDMI Port attribute types.
//
pub const FDMI1_PORT_ATTR_COUNT: c_int = 6;
pub const FDMI2_PORT_ATTR_COUNT: c_int = 16;
pub const FDMI2_SMARTSAN_PORT_ATTR_COUNT: c_int = 23;
pub const FDMI_PORT_FC4_TYPES: c_uint = 0x1;
pub const FDMI_PORT_SUPPORT_SPEED: c_uint = 0x2;
pub const FDMI_PORT_CURRENT_SPEED: c_uint = 0x3;
pub const FDMI_PORT_MAX_FRAME_SIZE: c_uint = 0x4;
pub const FDMI_PORT_OS_DEVICE_NAME: c_uint = 0x5;
pub const FDMI_PORT_HOST_NAME: c_uint = 0x6;
pub const FDMI_PORT_NODE_NAME: c_uint = 0x7;
pub const FDMI_PORT_NAME: c_uint = 0x8;
pub const FDMI_PORT_SYM_NAME: c_uint = 0x9;
pub const FDMI_PORT_TYPE: c_uint = 0xa;
pub const FDMI_PORT_SUPP_COS: c_uint = 0xb;
pub const FDMI_PORT_FABRIC_NAME: c_uint = 0xc;
pub const FDMI_PORT_FC4_TYPE: c_uint = 0xd;
pub const FDMI_PORT_STATE: c_uint = 0x101;
pub const FDMI_PORT_COUNT: c_uint = 0x102;
pub const FDMI_PORT_IDENTIFIER: c_uint = 0x103;
pub const FDMI_SMARTSAN_SERVICE: c_uint = 0xF100;
pub const FDMI_SMARTSAN_GUID: c_uint = 0xF101;
pub const FDMI_SMARTSAN_VERSION: c_uint = 0xF102;
pub const FDMI_SMARTSAN_PROD_NAME: c_uint = 0xF103;
pub const FDMI_SMARTSAN_PORT_INFO: c_uint = 0xF104;
pub const FDMI_SMARTSAN_QOS_SUPPORT: c_uint = 0xF105;
pub const FDMI_SMARTSAN_SECURITY_SUPPORT: c_uint = 0xF106;
pub const FDMI_PORT_SPEED_1GB: c_uint = 0x1;
pub const FDMI_PORT_SPEED_2GB: c_uint = 0x2;
pub const FDMI_PORT_SPEED_10GB: c_uint = 0x4;
pub const FDMI_PORT_SPEED_4GB: c_uint = 0x8;
pub const FDMI_PORT_SPEED_8GB: c_uint = 0x10;
pub const FDMI_PORT_SPEED_16GB: c_uint = 0x20;
pub const FDMI_PORT_SPEED_32GB: c_uint = 0x40;
pub const FDMI_PORT_SPEED_20GB: c_uint = 0x80;
pub const FDMI_PORT_SPEED_40GB: c_uint = 0x100;
pub const FDMI_PORT_SPEED_128GB: c_uint = 0x200;
pub const FDMI_PORT_SPEED_64GB: c_uint = 0x400;
pub const FDMI_PORT_SPEED_256GB: c_uint = 0x800;
pub const FDMI_PORT_SPEED_UNKNOWN: c_uint = 0x8000;
pub const FC_CLASS_2: c_uint = 0x04;
pub const FC_CLASS_3: c_uint = 0x08;
pub const FC_CLASS_2_3: c_uint = 0x0C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_fdmi_port_attr {
    pub type: __be16,
    pub len: __be16,
    pub fc4_types: [u8; 32],
    pub sup_speed: __be32,
    pub cur_speed: __be32,
    pub max_frame_size: __be32,
    pub os_dev_name: [u8; 32],
    pub host_name: [u8; 256],
    pub node_name: [u8; WWN_SIZE],
    pub port_name: [u8; WWN_SIZE],
    pub port_sym_name: [u8; 128],
    pub port_type: __be32,
    pub port_supported_cos: __be32,
    pub fabric_name: [u8; WWN_SIZE],
    pub port_fc4_type: [u8; 32],
    pub port_state: __be32,
    pub num_ports: __be32,
    pub port_id: __be32,
    pub smartsan_service: [u8; 24],
    pub smartsan_guid: [u8; 16],
    pub smartsan_version: [u8; 24],
    pub smartsan_prod_name: [u8; 16],
    pub smartsan_port_info: __be32,
    pub smartsan_qos_support: __be32,
    pub smartsan_security_support: __be32,
    pub a: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_fdmi1_port_attributes {
    pub count: __be32,
    pub entry: [ct_fdmi_port_attr; FDMI1_PORT_ATTR_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_fdmi2_port_attributes {
    pub count: __be32,
    pub entry: [ct_fdmi_port_attr; FDMI2_PORT_ATTR_COUNT],
}

// FDMI register call options
pub const CALLOPT_FDMI1: c_int = 0;
pub const CALLOPT_FDMI2: c_int = 1;
pub const CALLOPT_FDMI2_SMARTSAN: c_int = 2;
// FDMI definitions.
pub const GRHL_CMD: c_uint = 0x100;
pub const GHAT_CMD: c_uint = 0x101;
pub const GRPL_CMD: c_uint = 0x102;
pub const GPAT_CMD: c_uint = 0x110;
pub const RHBA_CMD: c_uint = 0x200;
pub const RHBA_RSP_SIZE: c_int = 16;
pub const RHAT_CMD: c_uint = 0x201;
pub const RPRT_CMD: c_uint = 0x210;
pub const RPRT_RSP_SIZE: c_int = 24;
pub const RPA_CMD: c_uint = 0x211;
pub const RPA_RSP_SIZE: c_int = 16;
pub const SMARTSAN_RPA_RSP_SIZE: c_int = 24;
pub const DHBA_CMD: c_uint = 0x300;

pub const DHBA_RSP_SIZE: c_int = 16;
pub const DHAT_CMD: c_uint = 0x301;
pub const DPRT_CMD: c_uint = 0x310;
pub const DPA_CMD: c_uint = 0x311;
// CT command header -- request/response common fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_cmd_hdr {
    pub revision: u8,
    pub in_id: [u8; 3],
    pub gs_type: u8,
    pub gs_subtype: u8,
    pub options: u8,
    pub reserved: u8,
}

// CT command request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_req {
    pub header: ct_cmd_hdr,
    pub command: __be16,
    pub max_rsp_size: __be16,
    pub fragment_id: u8,
    pub reserved: [u8; 3],
// GA_NXT, GPN_ID, GNN_ID, GFT_ID, GFPN_ID
    pub reserved: u8,
    pub port_id: be_id_t,
    pub port_id: },
    pub reserved: u8,
    pub domain: u8,
    pub area: u8,
    pub port_type: u8,
    pub gpn_ft: },
    pub port_type: u8,
    pub domain: u8,
    pub area: u8,
    pub reserved: u8,
    pub gid_pt: },
    pub reserved: u8,
    pub port_id: be_id_t,
    pub fc4_types: [u8; 32],
    pub rft_id: },
    pub reserved: u8,
    pub port_id: be_id_t,
    pub reserved2: u16,
    pub fc4_feature: u8,
    pub fc4_type: u8,
    pub rff_id: },
    pub reserved: u8,
    pub port_id: be_id_t,
    pub node_name: [u8; 8],
    pub rnn_id: },
    pub node_name: [u8; 8],
    pub name_len: u8,
    pub sym_node_name: [u8; 255],
    pub rsnn_nn: },
    pub hba_identifier: [u8; 8],
    pub ghat: },
    pub hba_identifier: [u8; 8],
    pub entry_count: __be32,
    pub port_name: [u8; 8],
    pub attrs: ct_fdmi2_hba_attributes,
    pub rhba: },
    pub hba_identifier: [u8; 8],
    pub attrs: ct_fdmi1_hba_attributes,
    pub rhat: },
    pub port_name: [u8; 8],
    pub attrs: ct_fdmi2_port_attributes,
    pub rpa: },
    pub hba_identifier: [u8; 8],
    pub port_name: [u8; 8],
    pub attrs: ct_fdmi2_port_attributes,
    pub rprt: },
    pub port_name: [u8; 8],
    pub dhba: },
    pub port_name: [u8; 8],
    pub dhat: },
    pub port_name: [u8; 8],
    pub dprt: },
    pub port_name: [u8; 8],
    pub dpa: },
    pub port_name: [u8; 8],
    pub gpsc: },
    pub reserved: u8,
    pub port_id: [u8; 3],
    pub gff_id: },
    pub port_name: [u8; 8],
    pub gid_pn: },
    pub req: },
}

// CT command response header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_rsp_hdr {
    pub header: ct_cmd_hdr,
    pub response: __be16,
    pub residual: u16,
    pub fragment_id: u8,
    pub reason_code: u8,
    pub explanation_code: u8,
    pub vendor_unique: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_gid_pt_data {
    pub control_byte: u8,
    pub port_id: be_id_t,
}

// It's the same for both GPN_FT and GNN_FT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_gpnft_rsp {
    pub header: ct_cmd_hdr,
    pub response: u16,
    pub residual: u16,
    pub fragment_id: u8,
    pub reason_code: u8,
    pub explanation_code: u8,
    pub vendor_unique: u8,
}

// Assume the largest number of targets for the union
// CT command response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_rsp {
    pub header: ct_rsp_hdr,
    pub port_type: u8,
    pub port_id: be_id_t,
    pub port_name: [u8; 8],
    pub sym_port_name_len: u8,
    pub sym_port_name: [u8; 255],
    pub node_name: [u8; 8],
    pub sym_node_name_len: u8,
    pub sym_node_name: [u8; 255],
    pub init_proc_assoc: [u8; 8],
    pub node_ip_addr: [u8; 16],
    pub class_of_service: [u8; 4],
    pub fc4_types: [u8; 32],
    pub ip_address: [u8; 16],
    pub fabric_port_name: [u8; 8],
    pub reserved: u8,
    pub hard_address: [u8; 3],
    pub ga_nxt: },
// Assume the largest number of targets for the union
    pub gid_pt: },
    pub port_name: [u8; 8],
    pub gpn_id: },
    pub node_name: [u8; 8],
    pub gnn_id: },
    pub fc4_types: [u8; 32],
    pub gft_id: },
    pub entry_count: u32,
    pub port_name: [u8; 8],
    pub attrs: ct_fdmi1_hba_attributes,
    pub ghat: },
    pub port_name: [u8; 8],
    pub gfpn_id: },
    pub speeds: __be16,
    pub speed: __be16,
    pub gpsc: },
pub const GFF_FCP_SCSI_OFFSET: c_int = 7;
    pub fc4_features: [u8; 128],
    pub gff_id: },
    pub reserved: u8,
    pub port_id: [u8; 3],
    pub gid_pn: },
    pub rsp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_pkt {
    pub req: ct_sns_req,
    pub rsp: ct_sns_rsp,
    pub p: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_sns_gpnft_pkt {
    pub req: ct_sns_req,
    pub rsp: ct_sns_gpnft_rsp,
    pub p: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_flags_t {
    SF_SCANNING = BIT_0,
    SF_QUEUED = BIT_1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc4type_t {
    FS_FC4TYPE_FCP	= BIT_0,
    FS_FC4TYPE_NVME	= BIT_1,
    FS_FCP_IS_N2N = BIT_7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fab_scan_rp {
    pub id: port_id_t,
    pub fc4type: fc4type_t,
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_step {
    FAB_SCAN_START,
    FAB_SCAN_GPNFT_FCP,
    FAB_SCAN_GNNFT_FCP,
    FAB_SCAN_GPNFT_NVME,
    FAB_SCAN_GNNFT_NVME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fab_scan {
    pub l: *mut fab_scan_rp,
    pub size: u32,
    pub rscn_gen_start: u32,
    pub rscn_gen_end: u32,
    pub step: scan_step,
    pub scan_retry: u16,
pub const MAX_SCAN_RETRIES: c_int = 5;
    pub scan_flags: scan_flags_t,
    pub scan_work: delayed_work,
}

//
// SNS command structures -- for 2200 compatibility.
//
pub const RFT_ID_SNS_SCMD_LEN: c_int = 22;
pub const RFT_ID_SNS_CMD_SIZE: c_int = 60;
pub const RFT_ID_SNS_DATA_SIZE: c_int = 16;
pub const RNN_ID_SNS_SCMD_LEN: c_int = 10;
pub const RNN_ID_SNS_CMD_SIZE: c_int = 36;
pub const RNN_ID_SNS_DATA_SIZE: c_int = 16;
pub const GA_NXT_SNS_SCMD_LEN: c_int = 6;
pub const GA_NXT_SNS_CMD_SIZE: c_int = 28;

pub const GID_PT_SNS_SCMD_LEN: c_int = 6;
pub const GID_PT_SNS_CMD_SIZE: c_int = 28;
//
// Assume MAX_FIBRE_DEVICES_2100 as these defines are only used with older
// adapters.
//

pub const GPN_ID_SNS_SCMD_LEN: c_int = 6;
pub const GPN_ID_SNS_CMD_SIZE: c_int = 28;

pub const GNN_ID_SNS_SCMD_LEN: c_int = 6;
pub const GNN_ID_SNS_CMD_SIZE: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sns_cmd_pkt {
    pub buffer_length: __le16,
    pub reserved_1: __le16,
    pub __packed: __le64 buffer_address,
    pub subcommand_length: __le16,
    pub reserved_2: __le16,
    pub subcommand: __le16,
    pub size: __le16,
    pub reserved_3: u32,
    pub param: [u8; 36],
    pub cmd: },
    pub rft_data: [u8; RFT_ID_SNS_DATA_SIZE],
    pub rnn_data: [u8; RNN_ID_SNS_DATA_SIZE],
    pub gan_data: [u8; GA_NXT_SNS_DATA_SIZE],
    pub gid_data: [u8; GID_PT_SNS_DATA_SIZE],
    pub gpn_data: [u8; GPN_ID_SNS_DATA_SIZE],
    pub gnn_data: [u8; GNN_ID_SNS_DATA_SIZE],
    pub p: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_blob {
    pub name: *mut c_char,
    pub segs: [u32; 4],
    pub fw: *const firmware,
}

// Return data from MBC_GET_ID_LIST call.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gid_list_info {
    pub al_pa: u8,
    pub area: u8,
    pub domain: u8,
    pub /: *mut *mut uint8_t loop_id_2100; / ISP2100/ISP2200 -- 4 bytes.,
    pub /: *mut *mut __le16 loop_id; / ISP23XX -- 6 bytes.,
    pub /: *mut *mut uint16_t reserved_1; / ISP24XX -- 8 bytes.,
}

// NPIV

// NPIV - return codes of VP create and modify
pub const VP_RET_CODE_OK: c_int = 0;
pub const VP_RET_CODE_FATAL: c_int = 1;
pub const VP_RET_CODE_WRONG_ID: c_int = 2;
pub const VP_RET_CODE_WWPN: c_int = 3;
pub const VP_RET_CODE_RESOURCES: c_int = 4;
pub const VP_RET_CODE_NO_MEM: c_int = 5;
pub const VP_RET_CODE_NOT_FOUND: c_int = 6;
//
// ISP operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_operations {
    pub ): *mut *mut int (pci_config) (struct scsi_qla_host,
    pub ): *mut *mut int (reset_chip)(struct scsi_qla_host,
    pub ): *mut *mut int (chip_diag) (struct scsi_qla_host,
    pub ): *mut *mut void (config_rings) (struct scsi_qla_host,
    pub ): *mut *mut int (reset_adapter)(struct scsi_qla_host,
    pub ): *mut *mut int (nvram_config) (struct scsi_qla_host,
    pub ): *mut *mut void (update_fw_options) (struct scsi_qla_host,
    pub ): *mut *mut *mut int (load_risc) (struct scsi_qla_host , uint32_t,
    pub size_t): *mut *mut *mut *mut *mut char  (pci_info_str)(struct scsi_qla_host , char ,,
    pub size_t): *mut *mut *mut *mut *mut char  (fw_version_str)(struct scsi_qla_host , char ,,
    pub intr_handler: irq_handler_t,
    pub ): *mut *mut void (enable_intrs) (struct qla_hw_data,
    pub ): *mut *mut void (disable_intrs) (struct qla_hw_data,
    pub ): *mut *mut int (abort_command) (srb_t,
    pub int): *mut *mut *mut int (target_reset) (struct fc_port , uint64_t,,
    pub int): *mut *mut *mut int (lun_reset) (struct fc_port , uint64_t,,
    pub uint8_t): *mut *mut uint8_t, uint8_t, uint16_t ,,
    pub uint8_t): uint8_t,,
    pub (uint16_t): *mut *mut uint16_t (calc_req_entries),
    pub uint16_t): *mut *mut *mut *mut void (build_iocbs) (srb_t , cmd_entry_t ,,
    pub ): *mut *mut *mut *mut void (prep_ms_iocb) (struct scsi_qla_host , struct ct_arg,
    pub uint32_t): uint32_t,,
    pub vha): *mut *mut void (fw_dump)(struct scsi_qla_host,
    pub int): *mut *mut *mut void (mpi_fw_dump)(struct scsi_qla_host ,,
// Context: task, might sleep
    pub ): *mut *mut int (beacon_on) (struct scsi_qla_host,
    pub ): *mut *mut int (beacon_off) (struct scsi_qla_host,
    pub ): *mut *mut void (beacon_blink) (struct scsi_qla_host,
    pub uint32_t): uint32_t,,
    pub length): uint32_t offset, uint32_t,
    pub length): uint32_t offset, uint32_t,
    pub ): *mut *mut *mut int (get_flash_version) (struct scsi_qla_host , void,
    pub ): *mut *mut int (start_scsi) (srb_t,
    pub ): *mut *mut int (start_scsi_mq) (srb_t,
// Context: task, might sleep
    pub ): *mut *mut int (abort_isp) (struct scsi_qla_host,
    pub ): *mut *mut int (iospace_config)(struct qla_hw_data,
    pub ): *mut *mut int (initialize_adapter)(struct scsi_qla_host,
}

// MSI-X Support
pub const QLA_MSIX_CHIP_REV_24XX: c_int = 3;

pub const QLA_MSIX_RSP_Q: c_uint = 0x01;
pub const QLA_ATIO_VECTOR: c_uint = 0x02;
pub const QLA_MSIX_QPAIR_MULTIQ_RSP_Q: c_uint = 0x03;
pub const QLA_MIDX_DEFAULT: c_int = 0;
pub const QLA_MIDX_RSP_Q: c_int = 1;
pub const QLA_PCI_MSIX_CONTROL: c_uint = 0xa2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_msix_entry {
    pub have_irq: c_int,
    pub in_use: c_int,
    pub vector: u32,
    pub vector_base0: u32,
    pub entry: u16,
    pub name: [c_char; 30],
    pub handle: *mut c_void,
    pub cpuid: c_int,
}

// Work events.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qla_work_type {
    QLA_EVT_AEN,
    QLA_EVT_IDC_ACK,
    QLA_EVT_ASYNC_LOGIN,
    QLA_EVT_ASYNC_LOGOUT,
    QLA_EVT_ASYNC_ADISC,
    QLA_EVT_UEVENT,
    QLA_EVT_AENFX,
    QLA_EVT_UNMAP,
    QLA_EVT_NEW_SESS,
    QLA_EVT_GPDB,
    QLA_EVT_PRLI,
    QLA_EVT_GPSC,
    QLA_EVT_GNL,
    QLA_EVT_NACK,
    QLA_EVT_RELOGIN,
    QLA_EVT_ASYNC_PRLO,
    QLA_EVT_ASYNC_PRLO_DONE,
    QLA_EVT_SCAN_CMD,
    QLA_EVT_SCAN_FINISH,
    QLA_EVT_GFPNID,
    QLA_EVT_SP_RETRY,
    QLA_EVT_IIDMA,
    QLA_EVT_ELS_PLOGI,
    QLA_EVT_SA_REPLACE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_work_evt {
    pub list: list_head,
    pub type: qla_work_type,
    pub flags: u32,
pub const QLA_EVT_FLAG_FREE: c_uint = 0x1;
    pub code: fc_host_event_code,
    pub data: u32,
    pub aen: },
pub const QLA_IDC_ACK_REGS: c_int = 7;
    pub mb: [u16; QLA_IDC_ACK_REGS],
    pub idc_ack: },
    pub fcport: *mut fc_port,
    pub data: [u16; 2],
    pub logio: },
    pub code: u32,
pub const QLA_UEVENT_CODE_FW_DUMP: c_int = 0;
    pub uevent: },
    pub evtcode: u32,
    pub mbx: [u32; 8],
    pub count: u32,
    pub aenfx: },
    pub sp: *mut srb_t,
    pub iosb: },
    pub id: port_id_t,
    pub port_name: [u8; 8],
    pub node_name: [u8; 8],
    pub pla: *mut c_void,
    pub fc4_type: u8,
    pub new_sess: },
    pub fcport: *mut fc_port_t,
    pub opt: u8,
    pub fcport: },
    pub fcport: *mut fc_port_t,
    pub iocb: [u8; IOCB_SIZE],
    pub type: c_int,
    pub nack: },
    pub fc4_type: u8,
    pub sp: *mut srb_t,
    pub gpnft: },
    pub sa_ctl: *mut edif_sa_ctl,
    pub fcport: *mut fc_port_t,
    pub nport_handle: u16,
    pub sa_update: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_chip_state_84xx {
    pub list: list_head,
    pub kref: kref,
    pub bus: *mut c_void,
    pub access_lock: spinlock_t,
    pub fw_update_mutex: mutex,
    pub fw_update: u32,
    pub op_fw_version: u32,
    pub op_fw_size: u32,
    pub op_fw_seq_size: u32,
    pub diag_fw_version: u32,
    pub gold_fw_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_dif_statistics {
    pub dif_input_bytes: u64,
    pub dif_output_bytes: u64,
    pub dif_input_requests: u64,
    pub dif_output_requests: u64,
    pub dif_guard_err: u32,
    pub dif_ref_tag_err: u32,
    pub dif_app_tag_err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_statistics {
    pub total_isp_aborts: u32,
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub input_requests: u64,
    pub output_requests: u64,
    pub control_requests: u32,
    pub jiffies_at_last_reset: u64,
    pub stat_max_pend_cmds: u32,
    pub stat_max_qfull_cmds_alloc: u32,
    pub stat_max_qfull_cmds_dropped: u32,
    pub qla_dif_stats: qla_dif_statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bidi_statistics {
    pub io_count: c_ulonglong,
    pub transfer_bytes: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tc_param {
    pub vha: *mut scsi_qla_host,
    pub blk_sz: u32,
    pub bufflen: u32,
    pub sg: *mut scatterlist,
    pub prot_sg: *mut scatterlist,
    pub ctx: *mut crc_context,
    pub ctx_dsd_alloced: *mut u8,
}

// Multi queue support
pub const MBC_INITIALIZE_MULTIQ: c_uint = 0x1f;

pub const QLA_MQ_SIZE: c_int = 32;
pub const QLA_MAX_QUEUES: c_int = 256;

pub const QLA_DEFAULT_QUE_QOS: c_int = 5;
pub const QLA_PRECONFIG_VPORTS: c_int = 32;
pub const QLA_MAX_VPORTS_QLA24XX: c_int = 128;
pub const QLA_MAX_VPORTS_QLA25XX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_tgt_counters {
    pub qla_core_sbt_cmd: u64,
    pub core_qla_que_buf: u64,
    pub qla_core_ret_ctio: u64,
    pub core_qla_snd_status: u64,
    pub qla_core_ret_sta_ctio: u64,
    pub core_qla_free_cmd: u64,
    pub num_q_full_sent: u64,
    pub num_alloc_iocb_failed: u64,
    pub num_term_xchg_sent: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_counters {
    pub input_bytes: u64,
    pub input_requests: u64,
    pub output_bytes: u64,
    pub output_requests: u64,
}

// Response queue data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsp_que {
    pub dma: dma_addr_t,
    pub ring: *mut response_t,
    pub ring_ptr: *mut response_t,
//
// 29xx extended IOCB ring (128-byte entries) aliases of ring/ring_ptr.
// Allocated when IS_QLA29XX(ha); set up at queue-init time to point at
// the same DMA memory as 'ring' but typed for the 29xx stride.  24xx
// code paths walk via ring_ptr; 29xx paths walk via ring_ext_ptr.
//
    pub ring_ext: *mut response_ext,
    pub ring_ext_ptr: *mut response_ext,
    pub /: *mut *mut *mut __le32 __iomem rsp_q_in; / FWI2-capable only.,
    pub rsp_q_out: *mut __le32 __iomem,
    pub ring_index: u16,
    pub out_ptr: u16,
    pub /: *mut *mut *mut uint16_t in_ptr; / queue shadow in index,
    pub length: u16,
    pub options: u16,
    pub rid: u16,
    pub id: u16,
    pub vp_idx: u16,
    pub hw: *mut qla_hw_data,
    pub msix: *mut qla_msix_entry,
    pub req: *mut req_que,
    pub /: *mut *mut *mut srb_t status_srb; / status continuation entry,
    pub qpair: *mut qla_qpair,
    pub dma_fx00: dma_addr_t,
    pub ring_fx00: *mut response_t,
    pub length_fx00: u16,
    pub rsp_pkt: [u8; REQUEST_ENTRY_SIZE],
}

// Request queue data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct req_que {
    pub dma: dma_addr_t,
    pub ring: *mut request_t,
    pub ring_ptr: *mut request_t,
//
// 29xx extended IOCB ring (128-byte entries) aliases of ring/ring_ptr.
// Allocated when IS_QLA29XX(ha); set up at queue-init time to point at
// the same DMA memory as 'ring' but typed for the 29xx stride.  24xx
// code paths must not run on 29xx HW, and vice-versa.
//
    pub ring_ext: *mut request_ext,
    pub ring_ext_ptr: *mut request_ext,
    pub /: *mut *mut *mut __le32 __iomem req_q_in; / FWI2-capable only.,
    pub req_q_out: *mut __le32 __iomem,
    pub ring_index: u16,
    pub in_ptr: u16,
    pub /: *mut *mut *mut uint16_t out_ptr; / queue shadow out index,
    pub cnt: u16,
    pub length: u16,
    pub options: u16,
    pub rid: u16,
    pub id: u16,
    pub qos: u16,
    pub vp_idx: u16,
    pub rsp: *mut rsp_que,
    pub outstanding_cmds: *mut srb_t,
    pub current_outstanding_cmd: u32,
    pub num_outstanding_cmds: u16,
    pub max_q_depth: c_int,
    pub dma_fx00: dma_addr_t,
    pub ring_fx00: *mut request_t,
    pub length_fx00: u16,
    pub req_pkt: [u8; REQUEST_ENTRY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fw_resources {
    pub iocbs_total: u16,
    pub iocbs_limit: u16,
    pub iocbs_qp_limit: u16,
    pub iocbs_used: u16,
    pub exch_total: u16,
    pub exch_limit: u16,
    pub exch_used: u16,
    pub pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_fw_res {
    pub iocb_total: u16,
    pub iocb_limit: u16,
    pub iocb_used: core::sync::atomic::AtomicI32,
    pub exch_total: u16,
    pub exch_limit: u16,
    pub exch_used: core::sync::atomic::AtomicI32,
}

pub const QLA_IOCB_PCT_LIMIT: c_int = 95;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_buf_pool {
    pub num_bufs: u16,
    pub num_active: u16,
    pub max_used: u16,
    pub num_alloc: u16,
    pub prev_max: u16,
    pub pad: u16,
    pub take_snapshot:1: u32,
    pub buf_map: *mut c_ulong,
    pub buf_array: *mut c_void,
    pub dma_array: *mut dma_addr_t,
}

// Queue pair data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_qpair {
    pub qp_lock: spinlock_t,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub lun_cnt: u32,
//
// For qpair 0, qp_lock_ptr will point at hardware_lock due to
// legacy code. For other Qpair(s), it will point at qp_lock.
//
    pub qp_lock_ptr: *mut spinlock_t,
    pub vha: *mut scsi_qla_host,
    pub chip_reset: u32,
// distill these fields down to 'online=0/1'
// ha->flags.eeh_busy
// ha->flags.pci_channel_io_perm_failure
// base_vha->loop_state
//
    pub online:1: u32,
// move vha->flags.difdix_supported here
    pub difdix_supported:1: u32,
    pub delete_in_progress:1: u32,
    pub fw_started:1: u32,
    pub enable_class_2:1: u32,
    pub enable_explicit_conf:1: u32,
    pub use_shadow_reg:1: u32,
    pub rcv_intr:1: u32,
    pub /: *mut *mut uint16_t id; / qp number used with FW,
    pub /: *mut *mut uint16_t vp_idx; / vport ID,
    pub dsd_inuse: u16,
    pub dsd_avail: u16,
    pub dsd_list: list_head,
pub const NUM_DSD_CHAIN: c_int = 4096;
    pub srb_mempool: *mut mempool_t,
    pub pdev: *mut pci_dev,
    pub ): *mut *mut void (reqq_start_iocbs)(struct qla_qpair,
// to do: New driver: move queues to here instead of pointers
    pub req: *mut req_que,
    pub rsp: *mut rsp_que,
    pub atio: *mut atio_que,
    pub /: *mut *mut *mut qla_msix_entry msix; / point to &ha->msix_entries[x],
    pub hw: *mut qla_hw_data,
    pub q_work: work_struct,
    pub counters: qla_counters,
    pub /: *mut *mut list_head qp_list_elem; / vha->qp_list,
    pub hints_list: list_head,
    pub retry_term_cnt: u16,
    pub retry_term_exchg_addr: __le32,
    pub retry_term_jiff: u64,
    pub tgt_counters: qla_tgt_counters,
    pub cpuid: u16,
    pub cpu_mapped: bool,
    pub ____cacheline_aligned: qla_fw_resources fwres,
    pub buf_pool: qla_buf_pool,
    pub cmd_cnt: u32,
    pub cmd_completion_cnt: u32,
    pub prev_completion_cnt: u32,
}

// Place holder for FW buffer parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlfc_fw {
    pub fw_buf: *mut c_void,
    pub fw_dma: dma_addr_t,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdp_req_payload {
    pub els_request: u32,
    pub desc_list_len: u32,
// NPIV descriptor
    pub desc_tag: u32,
    pub desc_len: u32,
    pub reserved: u8,
    pub nport_id: [u8; 3],
    pub npiv_desc: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdp_rsp_payload {
    pub cmd: __be32,
    pub len: __be32,
    pub hdr: },
// LS Request Info descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub req_payload_word_0: __be32,
    pub ls_req_info_desc: },
// LS Request Info descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub req_payload_word_0: __be32,
    pub ls_req_info_desc2: },
// SFP diagnostic param descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub temperature: __be16,
    pub vcc: __be16,
    pub tx_bias: __be16,
    pub tx_power: __be16,
    pub rx_power: __be16,
    pub sfp_flags: __be16,
    pub sfp_diag_desc: },
// Port Speed Descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub speed_capab: __be16,
    pub operating_speed: __be16,
    pub port_speed_desc: },
// Link Error Status Descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub link_fail_cnt: __be32,
    pub loss_sync_cnt: __be32,
    pub loss_sig_cnt: __be32,
    pub prim_seq_err_cnt: __be32,
    pub inval_xmit_word_cnt: __be32,
    pub inval_crc_cnt: __be32,
    pub pn_port_phy_type: u8,
    pub reserved: [u8; 3],
    pub ls_err_desc: },
// Port name description with diag param
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub WWNN: [u8; WWN_SIZE],
    pub WWPN: [u8; WWN_SIZE],
    pub port_name_diag_desc: },
// Port Name desc for Direct attached Fx_Port or Nx_Port
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub WWNN: [u8; WWN_SIZE],
    pub WWPN: [u8; WWN_SIZE],
    pub port_name_direct_desc: },
// Buffer Credit descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub fcport_b2b: __be32,
    pub attached_fcport_b2b: __be32,
    pub fcport_rtt: __be32,
    pub buffer_credit_desc: },
// Optical Element Data Descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub high_alarm: __be16,
    pub low_alarm: __be16,
    pub high_warn: __be16,
    pub low_warn: __be16,
    pub element_flags: __be32,
    pub optical_elmt_desc: [}; 5],
// Optical Product Data Descriptor
    pub desc_tag: __be32,
    pub desc_len: __be32,
    pub vendor_name: [u8; 16],
    pub part_number: [u8; 16],
    pub serial_number: [u8; 16],
    pub revision: [u8; 4],
    pub date: [u8; 8],
    pub optical_prod_desc: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_qlt_host {
    pub target_lport_ptr: *mut c_void,
    pub tgt_mutex: mutex,
    pub tgt_host_action_mutex: mutex,
    pub qla_tgt: *mut qla_tgt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlt_hw_data {
// Protected by hw lock
    pub node_name_set:1: u32,
    pub /: *mut *mut dma_addr_t atio_dma; / Physical address.,
    pub /: *mut *mut *mut atio atio_ring; / Base virtual address,
    pub /: *mut *mut *mut atio atio_ring_ptr; / Current address.,
    pub /: *mut *mut uint16_t atio_ring_index; / Current index.,
    pub atio_q_length: u16,
    pub atio_q_in: *mut __le32 __iomem,
    pub atio_q_out: *mut __le32 __iomem,
    pub tgt_ops: *const qla_tgt_func_tmpl,
    pub saved_set: c_int,
    pub saved_exchange_count: __le16,
    pub saved_firmware_options_1: __le32,
    pub saved_firmware_options_2: __le32,
    pub saved_firmware_options_3: __le32,
    pub saved_firmware_options: [u8; 2],
    pub saved_add_firmware_options: [u8; 2],
    pub tgt_node_name: [u8; WWN_SIZE],
    pub dfs_tgt_sess: *mut dentry,
    pub dfs_tgt_port_database: *mut dentry,
    pub dfs_naqp: *mut dentry,
    pub q_full_list: list_head,
    pub num_pend_cmds: u32,
    pub num_qfull_cmds_alloc: u32,
    pub num_qfull_cmds_dropped: u32,
    pub q_full_lock: spinlock_t,
    pub leak_exchg_thresh_hold: u32,
    pub sess_lock: spinlock_t,
    pub num_act_qpairs: c_int,
pub const DEFAULT_NAQP: c_int = 2;
    pub ____cacheline_aligned: spinlock_t atio_lock,
}

pub const MAX_QFULL_CMDS_ALLOC: c_int = 8192;
pub const Q_FULL_THRESH_HOLD_PERCENT: c_int = 90;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_hw_data_stat {
    pub num_fw_dump: u32,
    pub num_mpi_reset: u32,
}

// refer to pcie_do_recovery reference
//
// Qlogic host adapter specific data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_hw_data {
    pub pdev: *mut pci_dev,
// SRB cache.
pub const SRB_MIN_REQ: c_int = 128;
    pub srb_mempool: *mut mempool_t,
    pub port_name: [u8; WWN_SIZE],
    pub mbregs: [u16; 32],
    pub :1: uint32_t mbox_int,
    pub :1: uint32_t mbox_busy,
    pub :1: uint32_t disable_risc_code_load,
    pub :1: uint32_t enable_64bit_addressing,
    pub :1: uint32_t enable_lip_reset,
    pub :1: uint32_t enable_target_reset,
    pub :1: uint32_t enable_lip_full_login,
    pub :1: uint32_t enable_led_scheme,
    pub :1: uint32_t msi_enabled,
    pub :1: uint32_t msix_enabled,
    pub :1: uint32_t disable_serdes,
    pub :1: uint32_t gpsc_supported,
    pub :1: uint32_t npiv_supported,
    pub :1: uint32_t pci_channel_io_perm_failure,
    pub :1: uint32_t fce_enabled,
    pub :1: uint32_t user_enabled_fce,
    pub :1: uint32_t fce_dump_buf_alloced,
    pub :1: uint32_t fac_supported,
    pub :1: uint32_t chip_reset_done,
    pub :1: uint32_t running_gold_fw,
    pub :1: uint32_t eeh_busy,
    pub :1: uint32_t disable_msix_handshake,
    pub :1: uint32_t fcp_prio_enabled,
    pub isp82xx_fw_hung:1: u32,
    pub nic_core_hung:1: u32,
    pub quiesce_owner:1: u32,
    pub nic_core_reset_hdlr_active:1: u32,
    pub nic_core_reset_owner:1: u32,
    pub isp82xx_no_md_cap:1: u32,
    pub host_shutting_down:1: u32,
    pub idc_compl_status:1: u32,
    pub mr_reset_hdlr_active:1: u32,
    pub mr_intr_valid:1: u32,
    pub dport_enabled:1: u32,
    pub fawwpn_enabled:1: u32,
    pub exlogins_enabled:1: u32,
    pub exchoffld_enabled:1: u32,
    pub lip_ae:1: u32,
    pub n2n_ae:1: u32,
    pub fw_started:1: u32,
    pub fw_init_done:1: u32,
    pub lr_detected:1: u32,
    pub rida_fmt2:1: u32,
    pub purge_mbox:1: u32,
    pub n2n_bigger:1: u32,
    pub secure_adapter:1: u32,
    pub secure_fw:1: u32,
// Supported by Adapter
    pub scm_supported_a:1: u32,
// Supported by Firmware
    pub scm_supported_f:1: u32,
// Enabled in Driver
    pub scm_enabled:1: u32,
    pub edif_hw:1: u32,
    pub edif_enabled:1: u32,
    pub n2n_fw_acc_sec:1: u32,
    pub plogi_template_valid:1: u32,
    pub port_isolated:1: u32,
    pub eeh_flush:2: u32,
pub const EEH_FLUSH_RDY: c_int = 1;
pub const EEH_FLUSH_DONE: c_int = 2;
    pub t262_fail:1: u32,
    pub t272_fail:1: u32,
    pub secure_mcu:1: u32,
    pub valid_flt:1: u32,
    pub flags: },
    pub max_exchg: u16,
    pub /: *mut *mut uint16_t lr_distance; / 32G & above,
pub const LR_DISTANCE_5K: c_int = 1;
pub const LR_DISTANCE_10K: c_int = 0;
// This spinlock is used to protect "io transactions", you must
// acquire it before doing any IO to the card, eg with RD_REG*() and
// WRT_REG*() for the duration of your entire commandtransaction.
//
// This spinlock is of lower priority than the io request lock.
//
    pub ____cacheline_aligned: spinlock_t hardware_lock,
    pub bars: c_int,
    pub mem_only: c_int,
    pub /: *mut *mut *mut device_reg_t iobase; / Base I/O address,
    pub pio_address: resource_size_t,
pub const MIN_IOBASE_LEN: c_uint = 0x100;
    pub bar0_hdl: dma_addr_t,
    pub cregbase: *mut void __iomem,
    pub bar2_hdl: dma_addr_t,

    pub rqstq_intr_code: u32,
    pub mbx_intr_code: u32,
    pub req_que_len: u32,
    pub rsp_que_len: u32,
    pub req_que_off: u32,
    pub rsp_que_off: u32,
    pub eeh_jif: c_ulong,
// Multi queue data structs
    pub mqiobase: *mut device_reg_t,
    pub msixbase: *mut device_reg_t,
    pub msix_count: u16,
    pub mqenable: u8,
    pub req_q_map: *mut req_que,
    pub rsp_q_map: *mut rsp_que,
    pub queue_pair_map: *mut qla_qpair,
    pub qp_cpu_map: *mut qla_qpair,
    pub long)]: unsigned long req_qid_map[(QLA_MAX_QUEUES / 8) / sizeof(unsigned,
    pub long)]: unsigned long rsp_qid_map[(QLA_MAX_QUEUES / 8) / sizeof(unsigned,
    pub long)]: / sizeof(unsigned,
    pub max_req_queues: u8,
    pub max_rsp_queues: u8,
    pub max_qpairs: u8,
    pub num_qpairs: u8,
    pub base_qpair: *mut qla_qpair,
    pub npiv_info: *mut qla_npiv_entry,
    pub nvram_npiv_size: u16,
    pub switch_cap: u16,

    pub /: *mut *mut uint8_t port_no; / Physical port of adapter,
    pub exch_starvation: u8,
// Timeout timers.
    pub /: *mut *mut uint8_t loop_down_abort_time; / port down timer,
    pub /: *mut *mut atomic_t loop_down_timer; / loop down timer,
    pub /: *mut *mut uint8_t link_down_timeout; / link down timeout,
    pub max_loop_id: u16,
    pub /: *mut *mut uint16_t max_fibre_devices; / Maximum number of targets,
    pub fb_rev: u16,
    pub /: *mut *mut uint16_t min_external_loopid; / First external loop Id,
pub const PORT_SPEED_UNKNOWN: c_uint = 0xFFFF;
pub const PORT_SPEED_1GB: c_uint = 0x00;
pub const PORT_SPEED_2GB: c_uint = 0x01;
pub const PORT_SPEED_AUTO: c_uint = 0x02;
pub const PORT_SPEED_4GB: c_uint = 0x03;
pub const PORT_SPEED_8GB: c_uint = 0x04;
pub const PORT_SPEED_16GB: c_uint = 0x05;
pub const PORT_SPEED_32GB: c_uint = 0x06;
pub const PORT_SPEED_64GB: c_uint = 0x07;
pub const PORT_SPEED_128GB: c_uint = 0x08;
pub const PORT_SPEED_10GB: c_uint = 0x13;
    pub /: *mut *mut uint16_t link_data_rate; / F/W operating speed,
    pub /: *mut *mut uint16_t set_data_rate; / Set by user,
    pub current_topology: u8,
    pub prev_topology: u8,
pub const ISP_CFG_NL: c_int = 1;
pub const ISP_CFG_N: c_int = 2;
pub const ISP_CFG_FL: c_int = 4;
pub const ISP_CFG_F: c_int = 8;
    pub /: *mut *mut uint8_t operating_mode; / F/W operating mode,
pub const LOOP: c_int = 0;
pub const P2P: c_int = 1;
pub const LOOP_P2P: c_int = 2;
pub const P2P_LOOP: c_int = 3;
    pub interrupts_on: u8,
    pub isp_abort_cnt: u32,
pub const PCI_DEVICE_ID_QLOGIC_ISP2532: c_uint = 0x2532;
pub const PCI_DEVICE_ID_QLOGIC_ISP8432: c_uint = 0x8432;
pub const PCI_DEVICE_ID_QLOGIC_ISP8001: c_uint = 0x8001;
pub const PCI_DEVICE_ID_QLOGIC_ISP8031: c_uint = 0x8031;
pub const PCI_DEVICE_ID_QLOGIC_ISP2031: c_uint = 0x2031;
pub const PCI_DEVICE_ID_QLOGIC_ISP2071: c_uint = 0x2071;
pub const PCI_DEVICE_ID_QLOGIC_ISP2271: c_uint = 0x2271;
pub const PCI_DEVICE_ID_QLOGIC_ISP2261: c_uint = 0x2261;
pub const PCI_DEVICE_ID_QLOGIC_ISP2061: c_uint = 0x2061;
pub const PCI_DEVICE_ID_QLOGIC_ISP2081: c_uint = 0x2081;
pub const PCI_DEVICE_ID_QLOGIC_ISP2089: c_uint = 0x2089;
pub const PCI_DEVICE_ID_QLOGIC_ISP2281: c_uint = 0x2281;
pub const PCI_DEVICE_ID_QLOGIC_ISP2289: c_uint = 0x2289;
pub const PCI_DEVICE_ID_QLOGIC_ISP2099: c_uint = 0x2099;
pub const PCI_DEVICE_ID_QLOGIC_ISP2299: c_uint = 0x2299;
pub const PCI_DEVICE_ID_QLOGIC_ISP2091: c_uint = 0x2091;
pub const PCI_DEVICE_ID_QLOGIC_ISP2291: c_uint = 0x2291;
    pub isp_type: u32,

    pub device_type: u32,

// Bit 21 of fw_attributes decides the MCTP capabilities

// HBA serial number
    pub serial0: u8,
    pub serial1: u8,
    pub serial2: u8,
// NVRAM configuration data
pub const MAX_NVRAM_SIZE: c_int = 4096;

    pub nvram_size: u16,
    pub nvram_base: u16,
    pub nvram: *mut c_void,
    pub vpd_size: u16,
    pub vpd_base: u16,
    pub vpd: *mut c_void,
    pub fiv: *mut qla_flash_memo_block,
    pub loop_reset_delay: u16,
    pub retry_count: u8,
    pub login_timeout: u8,
    pub r_a_tov: u16,
    pub port_down_retry_count: c_int,
    pub mbx_count: u8,
    pub aen_mbx_count: u8,
    pub num_pend_mbx_stage1: core::sync::atomic::AtomicI32,
    pub num_pend_mbx_stage2: core::sync::atomic::AtomicI32,
    pub frame_payload_size: u16,
    pub login_retry_count: u32,
// SNS command interfaces.
    pub ms_iocb: *mut ms_iocb_entry_t,
    pub ms_iocb_dma: dma_addr_t,
    pub ct_sns: *mut ct_sns_pkt,
    pub ct_sns_dma: dma_addr_t,
// SNS command interfaces for 2200.
    pub sns_cmd: *mut sns_cmd_pkt,
    pub sns_cmd_dma: dma_addr_t,
pub const SFP_DEV_SIZE: c_int = 512;
pub const SFP_BLOCK_SIZE: c_int = 64;

    pub sfp_data: *mut c_void,
    pub sfp_data_dma: dma_addr_t,
    pub flt: *mut qla_flt_header,
    pub flt_dma: dma_addr_t,
    pub flt_data: *mut qla_flash_layout,
    pub fw_dump_tmplt_len: u32,
pub const XGMAC_DATA_SIZE: c_int = 4096;
    pub xgmac_data: *mut c_void,
    pub xgmac_data_dma: dma_addr_t,
pub const DCBX_TLV_DATA_SIZE: c_int = 4096;
    pub dcbx_tlv: *mut c_void,
    pub dcbx_tlv_dma: dma_addr_t,
    pub dpc_thread: *mut task_struct,
    pub /: *mut *mut uint8_t dpc_active; / DPC routine is active,
    pub gid_list_dma: dma_addr_t,
    pub gid_list: *mut gid_list_info,
    pub gid_list_info_size: c_int,
// Small DMA pool allocations -- maximum 256 bytes in length.
pub const DMA_POOL_SIZE: c_int = 256;
    pub s_dma_pool: *mut dma_pool,
    pub init_cb_dma: dma_addr_t,
    pub init_cb: *mut init_cb_t,
    pub init_cb_size: c_int,
    pub ex_init_cb_dma: dma_addr_t,
    pub ex_init_cb: *mut ex_init_cb_81xx,
    pub sf_init_cb_dma: dma_addr_t,
    pub sf_init_cb: *mut init_sf_cb,
    pub scm_fpin_els_buff: *mut c_void,
    pub scm_fpin_els_buff_size: u64,
    pub scm_fpin_valid: bool,
    pub scm_fpin_payload_size: bool,
    pub async_pd: *mut c_void,
    pub async_pd_dma: dma_addr_t,

// Extended Logins
    pub exlogin_buf: *mut c_void,
    pub exlogin_buf_dma: dma_addr_t,
    pub exlogin_size: u32,

// Exchange Offload
    pub exchoffld_buf: *mut c_void,
    pub exchoffld_buf_dma: dma_addr_t,
    pub exchoffld_size: c_int,
    pub exchoffld_count: c_int,
// n2n
    pub plogi_els_payld: fc_els_flogi,
    pub swl: *mut c_void,
// These are used by mailbox operations.
    pub mailbox_out: [u16; MAILBOX_REGISTER_COUNT],
    pub mailbox_out32: [u32; MAILBOX_REGISTER_COUNT],
    pub aenmb: [u32; AEN_MAILBOX_REGISTER_COUNT_FX00],
    pub mcp: *mut mbx_cmd_t,
    pub mcp32: *mut mbx_cmd_32,
    pub mbx_cmd_flags: c_ulong,
pub const MBX_INTERRUPT: c_int = 1;
pub const MBX_INTR_WAIT: c_int = 2;
pub const MBX_UPDATE_FLASH_ACTIVE: c_int = 3;
    pub /: *mut *mut mutex vport_lock; / Virtual port synchronization,
    pub /: *mut *mut spinlock_t vport_slock; / order is hardware_lock, then vport_slock,
    pub /: *mut *mut mutex mq_lock; / multi-queue synchronization,
    pub /: *mut *mut completion mbx_cmd_comp; / Serialize mbx access,
    pub /: *mut *mut completion mbx_intr_comp; / Used for completion notification,
    pub /: *mut *mut completion dcbx_comp; / For set port config notification,
    pub during: *mut *mut completion lb_portup_comp; / Used to wait for link up,
// loopback
pub const DCBX_COMP_TIMEOUT: c_int = 20;
pub const LB_PORTUP_COMP_TIMEOUT: c_int = 10;
    pub notify_dcbx_comp: c_int,
    pub notify_lb_portup_comp: c_int,
    pub selflogin_lock: mutex,
// Basic firmware related information.
    pub fw_major_version: u16,
    pub fw_minor_version: u16,
    pub fw_subminor_version: u16,
    pub fw_attributes: u16,
    pub fw_attributes_h: u16,

// About firmware SCM support

// Brocade fabric attached
pub const FW_ATTR_EXT0_SCM_BROCADE: c_uint = 0x00001000;
// Cisco fabric attached
pub const FW_ATTR_EXT0_SCM_CISCO: c_uint = 0x00002000;
    pub fw_attributes_ext: [u16; 2],
    pub fw_memory_size: u32,
    pub fw_transfer_size: u32,
    pub fw_srisc_address: u32,
pub const RISC_START_ADDRESS_2100: c_uint = 0x1000;
pub const RISC_START_ADDRESS_2300: c_uint = 0x800;
pub const RISC_START_ADDRESS_2400: c_uint = 0x100000;
    pub orig_fw_tgt_xcb_count: u16,
    pub cur_fw_tgt_xcb_count: u16,
    pub orig_fw_xcb_count: u16,
    pub cur_fw_xcb_count: u16,
    pub orig_fw_iocb_count: u16,
    pub cur_fw_iocb_count: u16,
    pub fw_max_fcf_count: u16,
    pub fw_shared_ram_start: u32,
    pub fw_shared_ram_end: u32,
    pub fw_ddr_ram_start: u32,
    pub fw_ddr_ram_end: u32,
    pub /: *mut *mut uint16_t fw_options[16]; / slots: 1,2,3,10,11,
    pub fw_seriallink_options: [u8; 4],
    pub fw_seriallink_options24: [__le16; 4],
    pub serdes_version: [u8; 3],
    pub mpi_version: [u8; 3],
    pub mpi_capabilities: u32,
    pub phy_version: [u8; 3],
    pub pep_version: [u8; 3],
// Firmware dump template
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwdt {
    pub template: *mut c_void,
    pub length: c_ulong,
    pub dump_size: c_ulong,
    pub fwdt: [}; 2],
    pub fw_dump: *mut qla2xxx_fw_dump,
    pub fw_dump_len: u32,
    pub fw_dump_alloc_len: u32,
    pub fw_dumped: bool,
    pub fw_dump_cap_flags: c_ulong,
pub const RISC_PAUSE_CMPL: c_int = 0;
pub const DMA_SHUTDOWN_CMPL: c_int = 1;
pub const ISP_RESET_CMPL: c_int = 2;
pub const RISC_RDY_AFT_RESET: c_int = 3;
pub const RISC_SRAM_DUMP_CMPL: c_int = 4;
pub const RISC_EXT_MEM_DUMP_CMPL: c_int = 5;
pub const ISP_MBX_RDY: c_int = 6;
pub const ISP_SOFT_RESET_CMPL: c_int = 7;
    pub fw_dump_reading: c_int,
    pub mpi_fw_dump: *mut c_void,
    pub mpi_fw_dump_len: u32,
    pub mpi_fw_dump_reading:1: c_uint,
    pub mpi_fw_dumped:1: c_uint,
    pub prev_minidump_failed: c_int,
    pub eft_dma: dma_addr_t,
    pub eft: *mut c_void,
// Current size of mctp dump is 0x086064 bytes
pub const MCTP_DUMP_SIZE: c_uint = 0x086064;
    pub mctp_dump_dma: dma_addr_t,
    pub mctp_dump: *mut c_void,
    pub mctp_dumped: c_int,
    pub mctp_dump_reading: c_int,
    pub chain_offset: u32,
    pub dfs_dir: *mut dentry,
    pub dfs_fce: *mut dentry,
    pub dfs_tgt_counters: *mut dentry,
    pub dfs_fw_resource_cnt: *mut dentry,
    pub fce_dma: dma_addr_t,
    pub fce: *mut c_void,
    pub fce_bufs: u32,
    pub fce_mb: [u16; 8],
    pub fce_rd: uint64_t fce_wr,,
    pub fce_mutex: mutex,
    pub pci_attr: u32,
    pub chip_revision: u16,
    pub product_id: [u16; 4],
    pub model_number: [u8; 16+1],
    pub model_desc: [c_char; 80],
    pub adapter_id: [u8; 16+1],
// Option ROM information.
    pub optrom_buffer: *mut c_char,
    pub optrom_size: u32,
    pub optrom_state: c_int,
pub const QLA_SWAITING: c_int = 0;
pub const QLA_SREADING: c_int = 1;
pub const QLA_SWRITING: c_int = 2;
    pub optrom_region_start: u32,
    pub optrom_region_size: u32,
    pub optrom_mutex: mutex,
// PCI expansion ROM image information.
pub const ROM_CODE_TYPE_BIOS: c_int = 0;
pub const ROM_CODE_TYPE_FCODE: c_int = 1;
pub const ROM_CODE_TYPE_EFI: c_int = 3;
    pub bios_revision: [u8; 2],
    pub efi_revision: [u8; 2],
    pub fcode_revision: [u8; 16],
    pub fw_revision: [u32; 4],
    pub gold_fw_version: [u32; 4],
// Offsets for flash/nvram access (set to ~0 if not used).
    pub flash_conf_off: u32,
    pub flash_data_off: u32,
    pub nvram_conf_off: u32,
    pub nvram_data_off: u32,
    pub fdt_wrt_disable: u32,
    pub fdt_wrt_enable: u32,
    pub fdt_erase_cmd: u32,
    pub fdt_block_size: u32,
    pub fdt_unprotect_sec_cmd: u32,
    pub fdt_protect_sec_cmd: u32,
    pub fdt_wrt_sts_reg_cmd: u32,
pub const QLA_SEGMENT_LENGTH: c_uint = 0x25000;
    pub flt_segment_length: u32,
    pub flt_region_flt: u32,
    pub flt_region_fdt: u32,
    pub flt_region_boot: u32,
    pub flt_region_boot_sec: u32,
    pub flt_region_fw: u32,
    pub flt_region_fw_sec: u32,
    pub flt_region_vpd_nvram: u32,
    pub flt_region_vpd_nvram_sec: u32,
    pub flt_region_vpd: u32,
    pub flt_region_vpd_sec: u32,
    pub flt_region_nvram: u32,
    pub flt_region_nvram_sec: u32,
    pub flt_region_npiv_conf: u32,
    pub flt_region_gold_fw: u32,
    pub flt_region_fcp_prio: u32,
    pub flt_region_bootload: u32,
    pub flt_region_img_status_pri: u32,
    pub flt_region_img_status_sec: u32,
    pub flt_region_aux_img_status_pri: u32,
    pub flt_region_aux_img_status_sec: u32,
}

pub const MAX_ACTIVE_TMF: c_int = 8;
// Needed for BEACON
pub const QLA_LED_GRN_ON: c_uint = 0x01;
pub const QLA_LED_YLW_ON: c_uint = 0x02;
pub const QLA_LED_ABR_ON: c_uint = 0x04;
pub const QLA_LED_ALL_ON: c_uint = 0x07	/* yellow, green, amber. */;
// ISP2322: red, green, amber.
// FCP_CMND priority support
pub const DSD_LIST_DMA_POOL_SIZE: c_int = 512;
pub const FCP_CMND_DMA_POOL_SIZE: c_int = 512;
// QLA83XX IDC specific fields
// DPC low-priority workqueue
// DPC high-priority workqueue
// DMA pool for the DIF bundling buffers
pub const DIF_BUNDLING_DMA_POOL_SIZE: c_int = 1024;
pub const DEFAULT_ZIO_THRESHOLD: c_int = 5;
pub const EDIF_NUM_SA_INDEX: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct active_regions {
    pub global: u8,
    pub board_config: u8,
    pub vpd_nvram: u8,
    pub npiv_config_0_1: u8,
    pub npiv_config_2_3: u8,
    pub nvme_params: u8,
    pub aux: },
}

pub const FW_ABILITY_MAX_SPEED_MASK: c_uint = 0xFUL;
pub const FW_ABILITY_MAX_SPEED_16G: c_uint = 0x0;
pub const FW_ABILITY_MAX_SPEED_32G: c_uint = 0x1;

pub const QLA_GET_DATA_RATE: c_int = 0;
pub const QLA_SET_DATA_RATE_NOLR: c_int = 1;

pub const QLA_DEFAULT_PAYLOAD_SIZE: c_int = 64;
pub const QLA_MAX_IOCB_SIZE: c_int = 128;
//
// This item might be allocated with a size > sizeof(struct purex_item).
// The "size" variable gives the size of the payload (which
// is variable) starting at "iocb".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct purex_item {
    pub purls_context: *mut c_void,
    pub list: list_head,
    pub vha: *mut scsi_qla_host,
    pub pkt): *mut purex_item,
    pub in_use: core::sync::atomic::AtomicI32,
    pub size: u16,
    pub iocb: [u8; QLA_MAX_IOCB_SIZE],
    pub iocb: },
}

pub const SCM_FLAG_RDF_REJECT: c_uint = 0x00;
pub const SCM_FLAG_RDF_COMPLETED: c_uint = 0x01;
pub const QLA_CON_PRIMITIVE_RECEIVED: c_uint = 0x1;
pub const QLA_CONGESTION_ARB_WARNING: c_uint = 0x1;

//
// Qlogic scsi host structure
//
// Commonly used flags and state information.
pub const LOOP_TIMEOUT: c_int = 1;
pub const LOOP_DOWN: c_int = 2;
pub const LOOP_UP: c_int = 3;
pub const LOOP_UPDATE: c_int = 4;
pub const LOOP_READY: c_int = 5;
pub const LOOP_DEAD: c_int = 6;

pub const RESET_ACTIVE: c_int = 1;

pub const LOOP_RESYNC_ACTIVE: c_int = 5;

pub const RELOGIN_NEEDED: c_int = 8;

pub const BEACON_BLINK_NEEDED: c_int = 11;
pub const REGISTER_FDMI_NEEDED: c_int = 12;

pub const UNLOADING: c_int = 15;
pub const NPIV_CONFIG_NEEDED: c_int = 16;
pub const ISP_UNRECOVERABLE: c_int = 17;

pub const N2N_LINK_RESET: c_int = 21;
pub const PORT_UPDATE_NEEDED: c_int = 22;
pub const FX00_RESET_RECOVERY: c_int = 23;
pub const FX00_TARGET_SCAN: c_int = 24;
pub const FX00_CRITEMP_RECOVERY: c_int = 25;
pub const FX00_HOST_INFO_RESEND: c_int = 26;
pub const QPAIR_ONLINE_CHECK_NEEDED: c_int = 27;
pub const DO_EEH_RECOVERY: c_int = 28;
pub const DETECT_SFP_CHANGE: c_int = 29;
pub const N2N_LOGIN_NEEDED: c_int = 30;
pub const IOCB_WORK_ACTIVE: c_int = 31;
pub const SET_ZIO_THRESHOLD_NEEDED: c_int = 32;
pub const ISP_ABORT_TO_ROM: c_int = 33;
pub const VPORT_DELETE: c_int = 34;
pub const PROCESS_PUREX_IOCB: c_int = 63;

// ISP configuration data.
// get it on self login
//
// no need of allocating it for
// each command
//
// Timeout timers.
// list of commands waiting on workqueue
// Counter to detect races between ELS and RSCN events
// Time when global fcport update has been scheduled
// List of pending LOGOs, protected by tgt_mutex
// List of pending PLOGI acks, protected by hw lock

pub const VP_CREATE_NEEDED: c_int = 1;
pub const VP_BIND_NEEDED: c_int = 2;
pub const VP_DELETE_NEEDED: c_int = 3;

pub const VP_OFFLINE: c_int = 0;
pub const VP_ACTIVE: c_int = 1;
pub const VP_FAILED: c_int = 2;
// #define VP_DISABLE		3
pub const VP_ERR_UNKWN: c_int = 0;
pub const VP_ERR_PORTDWN: c_int = 1;
pub const VP_ERR_FAB_UNSUPPORTED: c_int = 2;
pub const VP_ERR_FAB_NORESOURCES: c_int = 3;
pub const VP_ERR_FAB_LOGOUT: c_int = 4;
pub const VP_ERR_ADAP_NORESOURCES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct purex_list {
    pub head: list_head,
    pub lock: spinlock_t,
    pub purex_list: },
    pub default_item: purex_item,
    pub gnl: name_list_extended,
// Count of active session/fcport
    pub fcport_count: c_int,
    pub fcport_waitQ: wait_queue_head_t,
    pub vref_waitq: wait_queue_head_t,
    pub min_supported_speed: u8,
    pub n2n_node_name: [u8; WWN_SIZE],
    pub n2n_port_name: [u8; WWN_SIZE],
    pub n2n_id: u16,
    pub dport_data: [__le16; 4],
    pub scan: fab_scan,
    pub scm_fabric_connection_flags: u8,
    pub irq_offset: c_uint,
    pub hw_err_cnt: u64,
    pub interface_err_cnt: u64,
    pub cmd_timeout_cnt: u64,
    pub reset_cmd_err_cnt: u64,
    pub link_down_time: u64,
    pub short_link_down_cnt: u64,
    pub e_dbell: edif_dbell,
    pub pur_cinfo: pur_core,

    pub dport_status: u16,
    pub scsi_qla_host_t: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla27xx_image_status {
    pub image_status_mask: u8,
    pub generation: __le16,
    pub ver_major: u8,
    pub ver_minor: u8,
    pub /: *mut *mut uint8_t bitmap; / 28xx only,
    pub reserved: [u8; 2],
    pub checksum: __le32,
    pub signature: __le32,
    pub __packed: },
// 28xx aux image status bimap values

pub const SET_VP_IDX: c_int = 1;
pub const SET_AL_PA: c_int = 2;
pub const RESET_VP_IDX: c_int = 3;
pub const RESET_AL_PA: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_vp_map {
    pub idx: u8,
    pub vha: *mut scsi_qla_host_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla2_sgx {
    pub /: *mut *mut dma_addr_t dma_addr; / OUT,
    pub /: *mut *mut uint32_t dma_len; / OUT,
    pub /: *mut *mut uint32_t tot_bytes; / IN,
    pub /: *mut *mut *mut scatterlist cur_sg; / IN,
// for book keeping, bzero on initial invocation
    pub bytes_consumed: u32,
    pub num_bytes: u32,
    pub tot_partial: u32,
// for debugging
    pub num_sg: u32,
    pub sp: *mut srb_t,
}

pub const SFUB_CHECKSUM_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secure_flash_update_block {
    pub block_info: u32,
    pub signature_lo: u32,
    pub signature_hi: u32,
    pub signature_upper: [u32; 0x3e],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secure_flash_update_block_pk {
    pub block_info: u32,
    pub signature_lo: u32,
    pub signature_hi: u32,
    pub signature_upper: [u32; 0x3e],
    pub public_key: [u32; 0x41],
}

//
// Macros to help code, maintain, etc.
//

//
// qla2x00 local function return status codes
//
pub const MBS_MASK: c_uint = 0x3fff;

pub const QLA_FUNCTION_TIMEOUT: c_uint = 0x100;
pub const QLA_FUNCTION_PARAMETER_ERROR: c_uint = 0x101;
pub const QLA_FUNCTION_FAILED: c_uint = 0x102;
pub const QLA_MEMORY_ALLOC_FAILED: c_uint = 0x103;
pub const QLA_LOCK_TIMEOUT: c_uint = 0x104;
pub const QLA_ABORTED: c_uint = 0x105;
pub const QLA_SUSPENDED: c_uint = 0x106;
pub const QLA_BUSY: c_uint = 0x107;
pub const QLA_ALREADY_REGISTERED: c_uint = 0x109;
pub const QLA_OS_TIMER_EXPIRED: c_uint = 0x10a;
pub const QLA_ERR_NO_QPAIR: c_uint = 0x10b;
pub const QLA_ERR_NOT_FOUND: c_uint = 0x10c;
pub const QLA_ERR_FROM_FW: c_uint = 0x10d;

//
// Flash support definitions
//

pub const OPTROM_SIZE_2300: c_uint = 0x20000;
pub const OPTROM_SIZE_2322: c_uint = 0x100000;
pub const OPTROM_SIZE_24XX: c_uint = 0x100000;
pub const OPTROM_SIZE_25XX: c_uint = 0x200000;
pub const OPTROM_SIZE_81XX: c_uint = 0x400000;
pub const OPTROM_SIZE_82XX: c_uint = 0x800000;
pub const OPTROM_SIZE_83XX: c_uint = 0x1000000;
pub const OPTROM_SIZE_28XX: c_uint = 0x2000000;
pub const OPTROM_BURST_SIZE: c_uint = 0x1000;

pub const QLA_DSDS_PER_IOCB: c_int = 37;
pub const QLA_SG_ALL: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nexus_wait_type {
    WAIT_HOST = 0,
    WAIT_TARGET,
    WAIT_LUN,
}

pub const INVALID_EDIF_SA_INDEX: c_uint = 0xffff;
pub const RX_DELETE_NO_EDIF_SA_INDEX: c_uint = 0xfffe;

// edif hash element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_list_entry {
    pub /: *mut *mut uint16_t handle; / nport_handle,
    pub update_sa_index: u32,
    pub delete_sa_index: u32,
    pub /: *mut *mut uint32_t count; / counter for filtering sa_index,
pub const EDIF_ENTRY_FLAGS_CLEANUP: c_uint = 0x01	/* this index is being cleaned up */;
    pub /: *mut *mut uint32_t flags; / used by sadb cleanup code,
    pub /: *mut *mut *mut fc_port_t fcport; / needed by rx delay timer function,
    pub /: *mut *mut timer_list timer; / rx delay timer,
    pub next: list_head,
}

pub const EDIF_TX_INDX_BASE: c_int = 512;
pub const EDIF_RX_INDX_BASE: c_int = 0;

// entry in the sa_index free pool
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_index_pair {
    pub sa_index: u16,
    pub spi: u32,
}

// edif sa_index data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_sa_index_entry {
    pub sa_pair: [sa_index_pair; 2],
    pub fcport: *mut fc_port_t,
    pub handle: u16,
    pub next: list_head,
}

// Refer to SNIA SFF 8472
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sff_8247_a0 {
    pub /: *mut *mut u8 txid; / transceiver id,
    pub ext_txid: u8,
    pub connector: u8,
// compliance code
    pub /: *mut *mut u8 eth_infi_cc3; / ethernet, inifiband,
    pub sonet_cc4: [u8; 2],
    pub eth_cc6: u8,
// link length

    pub fc_ll_cc7: u8,
// FC technology

    pub fc_tec_cc8: u8,
// Transmission Media

    pub fc_med_cc9: u8,
// speed FC_SP_12: 12*100M = 1200 MB/s

    pub fc_sp_cc10: u8,
    pub encode: u8,
    pub bitrate: u8,
    pub rate_id: u8,
    pub /: *mut *mut u8 length_km; / offset 14/eh,
    pub length_100m: u8,
    pub length_50um_10m: u8,
    pub length_62um_10m: u8,
    pub length_om4_10m: u8,
    pub length_om3_10m: u8,
pub const SFF_VEN_NAME_LEN: c_int = 16;
    pub /: *mut *mut u8 vendor_name[SFF_VEN_NAME_LEN]; / offset 20/14h,
    pub tx_compat: u8,
    pub vendor_oui: [u8; 3],
pub const SFF_PART_NAME_LEN: c_int = 16;
    pub /: *mut *mut u8 vendor_pn[SFF_PART_NAME_LEN]; / part number,
    pub vendor_rev: [u8; 4],
    pub wavelength: [u8; 2],
    pub fiber_channel_speed2: u8,
    pub cc_base: u8,
    pub /: *mut *mut u8 options[2]; / offset 64,
    pub br_max: u8,
    pub br_min: u8,
    pub vendor_sn: [u8; 16],
    pub date_code: [u8; 8],
    pub diag: u8,
    pub enh_options: u8,
    pub sff_revision: u8,
    pub cc_ext: u8,
    pub vendor_specific: [u8; 32],
    pub resv2: [u8; 128],
}

// BPM -- Buffer Plus Management support.

pub const FLASH_SEMAPHORE_REGISTER_ADDR: c_uint = 0x00101016;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ql_vnd_host_stat_action {
    QLA_STOP = 0,
    QLA_START,
    QLA_CLEAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_mng_host_stats_param {
    pub stat_type: u32,
    pub action: ql_vnd_host_stat_action,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_mng_host_stats_resp {
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_stats_param {
    pub stat_type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_tgt_stats_param {
    pub tgt_id: i32,
    pub stat_type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ql_vnd_host_port_action {
    QLA_ENABLE = 0,
    QLA_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_mng_host_port_param {
    pub action: ql_vnd_host_port_action,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_mng_host_port_resp {
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_stat_entry {
    pub /: *mut *mut u32 stat_type; / Failure type,
    pub /: *mut *mut u32 tgt_num; / Target Num,
    pub /: *mut *mut u64 cnt; / Counter value,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_stats {
    pub /: *mut *mut u64 entry_count; / Num of entries,
    pub rservd: u64,
    pub /: *mut *mut ql_vnd_stat_entry entry[]; / Place holder of entries,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_host_stats_resp {
    pub status: u32,
    pub stats: ql_vnd_stats,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_vnd_tgt_stats_resp {
    pub status: u32,
    pub stats: ql_vnd_stats,
    pub __packed: },

