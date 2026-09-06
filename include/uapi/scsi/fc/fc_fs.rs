//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/fc/fc_fs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Fibre Channel Framing and Signalling definitions.
// From T11 FC-FS-2 Rev 0.90 - 9 August 2005.
//
// Frame header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_frame_header {
    pub /: *mut *mut __u8 fh_r_ctl; / routing control,
    pub /: *mut *mut __u8 fh_d_id[3]; / Destination ID,
    pub /: *mut *mut __u8 fh_cs_ctl; / class of service control / pri,
    pub /: *mut *mut __u8 fh_s_id[3]; / Source ID,
    pub /: *mut *mut __u8 fh_type; / see enum fc_fh_type below,
    pub /: *mut *mut __u8 fh_f_ctl[3]; / frame control,
    pub /: *mut *mut __u8 fh_seq_id; / sequence ID,
    pub /: *mut *mut __u8 fh_df_ctl; / data field control,
    pub /: *mut *mut __be16 fh_seq_cnt; / sequence count,
    pub /: *mut *mut __be16 fh_ox_id; / originator exchange ID,
    pub /: *mut *mut __be16 fh_rx_id; / responder exchange ID,
    pub /: *mut *mut __be32 fh_parm_offset; / parameter or relative offset,
}

//
// fh_r_ctl - Routing control definitions.
//
// FC-4 device_data.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rctl {
    FC_RCTL_DD_UNCAT = 0x00,	/* uncategorized information */
    FC_RCTL_DD_SOL_DATA = 0x01,	/* solicited data */
    FC_RCTL_DD_UNSOL_CTL = 0x02,	/* unsolicited control */
    FC_RCTL_DD_SOL_CTL = 0x03,	/* solicited control or reply */
    FC_RCTL_DD_UNSOL_DATA = 0x04,	/* unsolicited data */
    FC_RCTL_DD_DATA_DESC = 0x05,	/* data descriptor */
    FC_RCTL_DD_UNSOL_CMD = 0x06,	/* unsolicited command */
    FC_RCTL_DD_CMD_STATUS = 0x07,	/* command status */

//
// Extended Link_Data
//
    FC_RCTL_ELS_REQ = 0x22,	/* extended link services request */
    FC_RCTL_ELS_REP = 0x23,	/* extended link services reply */
    FC_RCTL_ELS4_REQ = 0x32, /* FC-4 ELS request */
    FC_RCTL_ELS4_REP = 0x33, /* FC-4 ELS reply */
//
// Optional Extended Headers
//
    FC_RCTL_VFTH = 0x50,	/* virtual fabric tagging header */
    FC_RCTL_IFRH = 0x51,	/* inter-fabric routing header */
    FC_RCTL_ENCH = 0x52,	/* encapsulation header */
//
// Basic Link Services fh_r_ctl values.
//
    FC_RCTL_BA_NOP = 0x80,	/* basic link service NOP */
    FC_RCTL_BA_ABTS = 0x81,	/* basic link service abort */
    FC_RCTL_BA_RMC = 0x82,	/* remove connection */
    FC_RCTL_BA_ACC = 0x84,	/* basic accept */
    FC_RCTL_BA_RJT = 0x85,	/* basic reject */
    FC_RCTL_BA_PRMT = 0x86,	/* dedicated connection preempted */
//
// Link Control Information.
//
    FC_RCTL_ACK_1 = 0xc0,	/* acknowledge_1 */
    FC_RCTL_ACK_0 = 0xc1,	/* acknowledge_0 */
    FC_RCTL_P_RJT = 0xc2,	/* port reject */
    FC_RCTL_F_RJT = 0xc3,	/* fabric reject */
    FC_RCTL_P_BSY = 0xc4,	/* port busy */
    FC_RCTL_F_BSY = 0xc5,	/* fabric busy to data frame */
    FC_RCTL_F_BSYL = 0xc6,	/* fabric busy to link control frame */
    FC_RCTL_LCR = 0xc7,	/* link credit reset */
    FC_RCTL_END = 0xc9,	/* end */
}

// incomplete list of definitions
//
// R_CTL names initializer.
// Please keep this matching the above definitions.
//

//
// Well-known fabric addresses.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_well_known_fid {
    FC_FID_NONE =           0x000000,       /* No destination */
    FC_FID_BCAST =		0xffffff,	/* broadcast */
    FC_FID_FLOGI =		0xfffffe,	/* fabric login */
    FC_FID_FCTRL =		0xfffffd,	/* fabric controller */
    FC_FID_DIR_SERV =	0xfffffc,	/* directory server */
    FC_FID_TIME_SERV =	0xfffffb,	/* time server */
    FC_FID_MGMT_SERV =	0xfffffa,	/* management server */
    FC_FID_QOS =		0xfffff9,	/* QoS Facilitator */
    FC_FID_ALIASES =	0xfffff8,	/* alias server (FC-PH2) */
    FC_FID_SEC_KEY =	0xfffff7,	/* Security key dist. server */
    FC_FID_CLOCK =		0xfffff6,	/* clock synch server */
    FC_FID_MCAST_SERV =	0xfffff5,	/* multicast server */
}

pub const FC_FID_WELL_KNOWN_MAX: c_uint = 0xffffff /* highest well-known fabric ID */;
pub const FC_FID_WELL_KNOWN_BASE: c_uint = 0xfffff5 /* start of well-known fabric ID */;
//
// Other well-known addresses, outside the above contiguous range.
//
pub const FC_FID_DOM_MGR: c_uint = 0xfffc00	/* domain manager base */;
//
// Fabric ID bytes.
//
pub const FC_FID_DOMAIN: c_int = 0;
pub const FC_FID_PORT: c_int = 1;
pub const FC_FID_LINK: c_int = 2;
//
// fh_type codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fh_type {
    FC_TYPE_BLS =	0x00,	/* basic link service */
    FC_TYPE_ELS =	0x01,	/* extended link service */
    FC_TYPE_IP =	0x05,	/* IP over FC, RFC 4338 */
    FC_TYPE_FCP =	0x08,	/* SCSI FCP */
    FC_TYPE_CT =	0x20,	/* Fibre Channel Services (FC-CT) */
    FC_TYPE_ILS =	0x22,	/* internal link service */
    FC_TYPE_NVME =	0x28,	/* FC-NVME */
}

//
// FC_TYPE names initializer.
// Please keep this matching the above definitions.
//

//
// Exchange IDs.
//
pub const FC_XID_UNKNOWN: c_uint = 0xffff	/* unknown exchange ID */;
pub const FC_XID_MIN: c_uint = 0x0	/* supported min exchange ID */;
pub const FC_XID_MAX: c_uint = 0xfffe	/* supported max exchange ID */;
//
// fh_f_ctl - Frame control flags.
//

//
// BA_ACC payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ba_acc {
    pub /: *mut *mut __u8 ba_seq_id_val; / SEQ_ID validity,
pub const FC_BA_SEQ_ID_VAL: c_uint = 0x80;
    pub /: *mut *mut __u8 ba_seq_id; / SEQ_ID of seq last deliverable,
    pub /: *mut *mut __u8 ba_resvd[2]; / reserved,
    pub /: *mut *mut __be16 ba_ox_id; / OX_ID for aborted seq or exch,
    pub /: *mut *mut __be16 ba_rx_id; / RX_ID for aborted seq or exch,
    pub /: *mut *mut __be16 ba_low_seq_cnt; / low SEQ_CNT of aborted seq,
    pub /: *mut *mut __be16 ba_high_seq_cnt; / high SEQ_CNT of aborted seq,
}

//
// BA_RJT: Basic Reject payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ba_rjt {
    pub /: *mut *mut __u8 br_resvd; / reserved,
    pub /: *mut *mut __u8 br_reason; / reason code,
    pub /: *mut *mut __u8 br_explan; / reason explanation,
    pub /: *mut *mut __u8 br_vendor; / vendor unique code,
}

//
// BA_RJT reason codes.
// From FS-2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ba_rjt_reason {
    FC_BA_RJT_NONE =	0,	/* in software this means no reject */
    FC_BA_RJT_INVL_CMD =	0x01,	/* invalid command code */
    FC_BA_RJT_LOG_ERR =	0x03,	/* logical error */
    FC_BA_RJT_LOG_BUSY =	0x05,	/* logical busy */
    FC_BA_RJT_PROTO_ERR =	0x07,	/* protocol error */
    FC_BA_RJT_UNABLE =	0x09,	/* unable to perform request */
    FC_BA_RJT_VENDOR =	0xff,	/* vendor-specific (see br_vendor) */
}

//
// BA_RJT reason code explanations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ba_rjt_explan {
    FC_BA_RJT_EXP_NONE =	0x00,	/* no additional expanation */
    FC_BA_RJT_INV_XID =	0x03,	/* invalid OX_ID-RX_ID combination */
    FC_BA_RJT_ABT =		0x05,	/* sequence aborted, no seq info */
}

//
// P_RJT or F_RJT: Port Reject or Fabric Reject parameter field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_pf_rjt {
    pub /: *mut *mut __u8 rj_action; / reserved,
    pub /: *mut *mut __u8 rj_reason; / reason code,
    pub /: *mut *mut __u8 rj_resvd; / reserved,
    pub /: *mut *mut __u8 rj_vendor; / vendor unique code,
}

//
// P_RJT and F_RJT reject reason codes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_pf_rjt_reason {
    FC_RJT_NONE =		0,	/* non-reject (reserved by standard) */
    FC_RJT_INVL_DID =	0x01,	/* invalid destination ID */
    FC_RJT_INVL_SID =	0x02,	/* invalid source ID */
    FC_RJT_P_UNAV_T =	0x03,	/* port unavailable, temporary */
    FC_RJT_P_UNAV =		0x04,	/* port unavailable, permanent */
    FC_RJT_CLS_UNSUP =	0x05,	/* class not supported */
    FC_RJT_DEL_USAGE =	0x06,	/* delimiter usage error */
    FC_RJT_TYPE_UNSUP =	0x07,	/* type not supported */
    FC_RJT_LINK_CTL =	0x08,	/* invalid link control */
    FC_RJT_R_CTL =		0x09,	/* invalid R_CTL field */
    FC_RJT_F_CTL =		0x0a,	/* invalid F_CTL field */
    FC_RJT_OX_ID =		0x0b,	/* invalid originator exchange ID */
    FC_RJT_RX_ID =		0x0c,	/* invalid responder exchange ID */
    FC_RJT_SEQ_ID =		0x0d,	/* invalid sequence ID */
    FC_RJT_DF_CTL =		0x0e,	/* invalid DF_CTL field */
    FC_RJT_SEQ_CNT =	0x0f,	/* invalid SEQ_CNT field */
    FC_RJT_PARAM =		0x10,	/* invalid parameter field */
    FC_RJT_EXCH_ERR =	0x11,	/* exchange error */
    FC_RJT_PROTO =		0x12,	/* protocol error */
    FC_RJT_LEN =		0x13,	/* incorrect length */
    FC_RJT_UNEXP_ACK =	0x14,	/* unexpected ACK */
    FC_RJT_FAB_CLASS =	0x15,	/* class unsupported by fabric entity */
    FC_RJT_LOGI_REQ =	0x16,	/* login required */
    FC_RJT_SEQ_XS =		0x17,	/* excessive sequences attempted */
    FC_RJT_EXCH_EST =	0x18,	/* unable to establish exchange */
    FC_RJT_FAB_UNAV =	0x1a,	/* fabric unavailable */
    FC_RJT_VC_ID =		0x1b,	/* invalid VC_ID (class 4) */
    FC_RJT_CS_CTL =		0x1c,	/* invalid CS_CTL field */
    FC_RJT_INSUF_RES =	0x1d,	/* insuff. resources for VC (Class 4) */
    FC_RJT_INVL_CLS =	0x1f,	/* invalid class of service */
    FC_RJT_PREEMT_RJT =	0x20,	/* preemption request rejected */
    FC_RJT_PREEMT_DIS =	0x21,	/* preemption not enabled */
    FC_RJT_MCAST_ERR =	0x22,	/* multicast error */
    FC_RJT_MCAST_ET =	0x23,	/* multicast error terminate */
    FC_RJT_PRLI_REQ =	0x24,	/* process login required */
    FC_RJT_INVL_ATT =	0x25,	/* invalid attachment */
    FC_RJT_VENDOR =		0xff,	/* vendor specific reject */
}

// default timeout values

