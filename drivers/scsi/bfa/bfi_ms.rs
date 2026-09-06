//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfi_ms.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_iocfc_h2i_msgs {
    BFI_IOCFC_H2I_CFG_REQ		= 1,
    BFI_IOCFC_H2I_SET_INTR_REQ	= 2,
    BFI_IOCFC_H2I_UPDATEQ_REQ	= 3,
    BFI_IOCFC_H2I_FAA_QUERY_REQ	= 4,
    BFI_IOCFC_H2I_ADDR_REQ		= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_iocfc_i2h_msgs {
    BFI_IOCFC_I2H_CFG_REPLY		= BFA_I2HM(1),
    BFI_IOCFC_I2H_UPDATEQ_RSP	= BFA_I2HM(3),
    BFI_IOCFC_I2H_FAA_QUERY_RSP	= BFA_I2HM(4),
    BFI_IOCFC_I2H_ADDR_MSG		= BFA_I2HM(5),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_cfg_s {
    pub /: *mut *mut u8 num_cqs; / Number of CQs to be used,
    pub /: *mut *mut u8 sense_buf_len; / SCSI sense length,
    pub rsvd_1: u16,
    pub /: *mut *mut u32 endian_sig; / endian signature of host,
    pub rsvd_2: u8,
    pub single_msix_vec: u8,
    pub rsvd: [u8; 2],
    pub num_ioim_reqs: __be16,
    pub num_fwtio_reqs: __be16,
//
// Request and response circular queue base addresses, size and
// shadow index pointers.
//
    pub req_cq_ba: [bfi_addr_u; BFI_IOC_MAX_CQS],
    pub req_shadow_ci: [bfi_addr_u; BFI_IOC_MAX_CQS],
    pub req_cq_elems: [__be16; BFI_IOC_MAX_CQS],
    pub rsp_cq_ba: [bfi_addr_u; BFI_IOC_MAX_CQS],
    pub rsp_shadow_pi: [bfi_addr_u; BFI_IOC_MAX_CQS],
    pub rsp_cq_elems: [__be16; BFI_IOC_MAX_CQS],
    pub /: *mut *mut bfi_addr_u stats_addr; / DMA-able address for stats,
    pub /: *mut *mut bfi_addr_u cfgrsp_addr; / config response dma address,
    pub ioim_snsbase: [bfi_addr_u; BFI_IOIM_SNSBUF_SEGS],
// IO sense buf base addr segments
    pub /: *mut *mut bfa_iocfc_intr_attr_s intr_attr; / IOC interrupt attributes,
}

//
// Boot target wwn information for this port. This contains either the stored
// or discovered boot target port wwns for the port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_bootwwns {
    pub wwn: [wwn_t; BFA_BOOT_BOOTLUN_MAX],
    pub nwwns: u8,
    pub rsvd: [u8; 7],
}

//
// Queue configuration response from firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_qreg_s {
    pub cpe_q_ci_off: [u32; BFI_IOC_MAX_CQS],
    pub cpe_q_pi_off: [u32; BFI_IOC_MAX_CQS],
    pub cpe_qctl_off: [u32; BFI_IOC_MAX_CQS],
    pub rme_q_ci_off: [u32; BFI_IOC_MAX_CQS],
    pub rme_q_pi_off: [u32; BFI_IOC_MAX_CQS],
    pub rme_qctl_off: [u32; BFI_IOC_MAX_CQS],
    pub hw_qid: [u8; BFI_IOC_MAX_CQS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_cfgrsp_s {
    pub fwcfg: bfa_iocfc_fwcfg_s,
    pub intr_attr: bfa_iocfc_intr_attr_s,
    pub bootwwns: bfi_iocfc_bootwwns,
    pub pbc_cfg: bfi_pbc_s,
    pub qreg: bfi_iocfc_qreg_s,
}

//
// BFI_IOCFC_H2I_CFG_REQ message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_cfg_req_s {
    pub mh: bfi_mhdr_s,
    pub ioc_cfg_dma_addr: bfi_addr_u,
}

//
// BFI_IOCFC_I2H_CFG_REPLY message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_cfg_reply_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u8 cfg_success; / cfg reply status,
    pub /: *mut *mut u8 lpu_bm; / LPUs assigned for this IOC,
    pub rsvd: [u8; 2],
}

//
// BFI_IOCFC_H2I_SET_INTR_REQ message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_set_intr_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 coalesce; / enable intr coalescing,
    pub rsvd: [u8; 3],
    pub /: *mut *mut __be16 delay; / delay timer 0..1125us,
    pub /: *mut *mut __be16 latency; / latency timer 0..225us,
}

//
// BFI_IOCFC_H2I_UPDATEQ_REQ message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_updateq_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u32 reqq_ba; / reqq base addr,
    pub /: *mut *mut u32 rspq_ba; / rspq base addr,
    pub /: *mut *mut u32 reqq_sci; / reqq shadow ci,
    pub /: *mut *mut u32 rspq_spi; / rspq shadow pi,
}

//
// BFI_IOCFC_I2H_UPDATEQ_RSP message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_iocfc_updateq_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 status; / updateq status,
    pub rsvd: [u8; 3],
}

//
// H2I Messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_iocfc_h2i_msg_u {
    pub mh: bfi_mhdr_s,
    pub cfg_req: bfi_iocfc_cfg_req_s,
    pub updateq_req: bfi_iocfc_updateq_req_s,
    pub mboxmsg: [u32; BFI_IOC_MSGSZ],
}

//
// I2H Messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_iocfc_i2h_msg_u {
    pub mh: bfi_mhdr_s,
    pub cfg_reply: bfi_iocfc_cfg_reply_s,
    pub updateq_rsp: bfi_iocfc_updateq_rsp_s,
    pub mboxmsg: [u32; BFI_IOC_MSGSZ],
}

//
// BFI_IOCFC_H2I_FAA_ENABLE_REQ BFI_IOCFC_H2I_FAA_DISABLE_REQ message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_faa_en_dis_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_faa_addr_msg_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub rsvd: [u8; 4],
    pub /: *mut *mut wwn_t pwwn; / Fabric acquired PWWN,
    pub /: *mut *mut wwn_t nwwn; / Fabric acquired PWWN,
}

//
// BFI_IOCFC_H2I_FAA_QUERY_REQ message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_faa_query_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 faa_status; / FAA status,
    pub /: *mut *mut u8 addr_source; / PWWN source,
    pub rsvd: [u8; 2],
    pub /: *mut *mut wwn_t faa; / Fabric acquired PWWN,
}

//
// BFI_IOCFC_I2H_FAA_ENABLE_RSP, BFI_IOCFC_I2H_FAA_DISABLE_RSP message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_faa_en_dis_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 status; / updateq status,
    pub rsvd: [u8; 3],
}

//
// BFI_IOCFC_I2H_FAA_QUERY_RSP message
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fcport_h2i {
    BFI_FCPORT_H2I_ENABLE_REQ		= (1),
    BFI_FCPORT_H2I_DISABLE_REQ		= (2),
    BFI_FCPORT_H2I_SET_SVC_PARAMS_REQ	= (3),
    BFI_FCPORT_H2I_STATS_GET_REQ		= (4),
    BFI_FCPORT_H2I_STATS_CLEAR_REQ		= (5),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fcport_i2h {
    BFI_FCPORT_I2H_ENABLE_RSP		= BFA_I2HM(1),
    BFI_FCPORT_I2H_DISABLE_RSP		= BFA_I2HM(2),
    BFI_FCPORT_I2H_SET_SVC_PARAMS_RSP	= BFA_I2HM(3),
    BFI_FCPORT_I2H_STATS_GET_RSP		= BFA_I2HM(4),
    BFI_FCPORT_I2H_STATS_CLEAR_RSP		= BFA_I2HM(5),
    BFI_FCPORT_I2H_EVENT			= BFA_I2HM(6),
    BFI_FCPORT_I2H_TRUNK_SCN		= BFA_I2HM(7),
    BFI_FCPORT_I2H_ENABLE_AEN		= BFA_I2HM(8),
    BFI_FCPORT_I2H_DISABLE_AEN		= BFA_I2HM(9),
}

//
// Generic REQ type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / msg header,
    pub /: *mut *mut u32 msgtag; / msgtag for reply,
}

//
// Generic RSP type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 status; / port enable status,
    pub rsvd: [u8; 3],
    pub /: *mut *mut bfa_port_cfg_s port_cfg;/ port configuration,
    pub /: *mut *mut u32 msgtag; / msgtag for reply,
}

//
// BFI_FCPORT_H2I_ENABLE_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_enable_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / msg header,
    pub rsvd1: u32,
    pub /: *mut *mut wwn_t nwwn; / node wwn of physical port,
    pub /: *mut *mut wwn_t pwwn; / port wwn of physical port,
    pub /: *mut *mut bfa_port_cfg_s port_cfg; / port configuration,
    pub /: *mut *mut bfi_addr_u stats_dma_addr; / DMA address for stats,
    pub /: *mut *mut u32 msgtag; / msgtag for reply,
    pub /: *mut *mut u8 use_flash_cfg; / get prot cfg from flash,
    pub rsvd2: [u8; 3],
}

//
// BFI_FCPORT_H2I_SET_SVC_PARAMS_REQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_set_svc_params_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / msg header,
    pub /: *mut *mut __be16 tx_bbcredit; / Tx credits,
    pub rsvd: [u8; 2],
}

//
// BFI_FCPORT_I2H_EVENT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_event_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub link_state: bfa_port_link_s,
}

//
// BFI_FCPORT_I2H_TRUNK_SCN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_trunk_link_s {
    pub trunk_wwn: wwn_t,
    pub /: *mut *mut u8 fctl; / bfa_trunk_link_fctl_t,
    pub /: *mut *mut u8 state; / bfa_trunk_link_state_t,
    pub /: *mut *mut u8 speed; / bfa_port_speed_t,
    pub rsvd: u8,
    pub deskew: __be32,
}

pub const BFI_FCPORT_MAX_LINKS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcport_trunk_scn_s {
    pub mh: bfi_mhdr_s,
    pub /: *mut *mut u8 trunk_state; / bfa_trunk_state_t,
    pub /: *mut *mut u8 trunk_speed; / bfa_port_speed_t,
    pub rsvd_a: [u8; 2],
    pub tlink: [bfi_fcport_trunk_link_s; BFI_FCPORT_MAX_LINKS],
}

//
// fcport H2I message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_fcport_h2i_msg_u {
    pub mhdr: *mut bfi_mhdr_s,
    pub penable: *mut bfi_fcport_enable_req_s,
    pub pdisable: *mut bfi_fcport_req_s,
    pub psetsvcparams: *mut bfi_fcport_set_svc_params_req_s,
    pub pstatsget: *mut bfi_fcport_req_s,
    pub pstatsclear: *mut bfi_fcport_req_s,
}

//
// fcport I2H message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_fcport_i2h_msg_u {
    pub msg: *mut bfi_msg_s,
    pub penable_rsp: *mut bfi_fcport_rsp_s,
    pub pdisable_rsp: *mut bfi_fcport_rsp_s,
    pub psetsvcparams_rsp: *mut bfi_fcport_rsp_s,
    pub pstatsget_rsp: *mut bfi_fcport_rsp_s,
    pub pstatsclear_rsp: *mut bfi_fcport_rsp_s,
    pub event: *mut bfi_fcport_event_s,
    pub trunk_scn: *mut bfi_fcport_trunk_scn_s,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fcxp_h2i {
    BFI_FCXP_H2I_SEND_REQ = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_fcxp_i2h {
    BFI_FCXP_I2H_SEND_RSP = BFA_I2HM(1),
}

pub const BFA_FCXP_MAX_SGES: c_int = 2;
//
// FCXP send request structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcxp_send_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 fcxp_tag; / driver request tag,
    pub /: *mut *mut __be16 max_frmsz; / max send frame size,
    pub /: *mut *mut __be16 vf_id; / vsan tag if applicable,
    pub /: *mut *mut u16 rport_fw_hndl; / FW Handle for the remote port,
    pub /: *mut *mut u8 class; / FC class used for req/rsp,
    pub /: *mut *mut u8 rsp_timeout; / timeout in secs, 0-no response,
    pub /: *mut *mut u8 cts; / continue sequence,
    pub /: *mut *mut u8 lp_fwtag; / lport tag,
    pub /: *mut *mut fchs_s fchs; / request FC header structure,
    pub /: *mut *mut __be32 req_len; / request payload length,
    pub /: *mut *mut __be32 rsp_maxlen; / max response length expected,
    pub /: *mut *mut bfi_alen_s req_alen; / request buffer,
    pub /: *mut *mut bfi_alen_s rsp_alen; / response buffer,
}

//
// FCXP send response structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_fcxp_send_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 fcxp_tag; / send request tag,
    pub /: *mut *mut u8 req_status; / request status,
    pub rsvd: u8,
    pub /: *mut *mut __be32 rsp_len; / actual response length,
    pub /: *mut *mut __be32 residue_len; / residual response length,
    pub /: *mut *mut fchs_s fchs; / response FC header structure,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_uf_h2i {
    BFI_UF_H2I_BUF_POST = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_uf_i2h {
    BFI_UF_I2H_FRM_RCVD = BFA_I2HM(1),
}

pub const BFA_UF_MAX_SGES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_uf_buf_post_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u16 buf_tag; / buffer tag,
    pub /: *mut *mut __be16 buf_len; / total buffer length,
    pub /: *mut *mut bfi_alen_s alen; / buffer address/len pair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_uf_frm_rcvd_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut u16 buf_tag; / buffer tag,
    pub rsvd: u16,
    pub /: *mut *mut u16 frm_len; / received frame length,
    pub /: *mut *mut u16 xfr_len; / tranferred length,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_lps_h2i_msgs {
    BFI_LPS_H2I_LOGIN_REQ	= 1,
    BFI_LPS_H2I_LOGOUT_REQ	= 2,
    BFI_LPS_H2I_N2N_PID_REQ = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_lps_i2h_msgs {
    BFI_LPS_I2H_LOGIN_RSP	= BFA_I2HM(1),
    BFI_LPS_I2H_LOGOUT_RSP	= BFA_I2HM(2),
    BFI_LPS_I2H_CVL_EVENT	= BFA_I2HM(3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lps_login_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub bfa_tag: u8,
    pub alpa: u8,
    pub pdu_size: __be16,
    pub pwwn: wwn_t,
    pub nwwn: wwn_t,
    pub fdisc: u8,
    pub auth_en: u8,
    pub lps_role: u8,
    pub bb_scn: u8,
    pub vvl_flag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lps_login_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub fw_tag: u8,
    pub status: u8,
    pub lsrjt_rsn: u8,
    pub lsrjt_expl: u8,
    pub port_name: wwn_t,
    pub node_name: wwn_t,
    pub bb_credit: __be16,
    pub f_port: u8,
    pub npiv_en: u8,
    pub lp_pid:24: u32,
    pub auth_req:8: u32,
    pub lp_mac: mac_t,
    pub fcf_mac: mac_t,
    pub ext_status: u8,
    pub /: *mut *mut u8 brcd_switch; / attached peer is brcd switch,
    pub bfa_tag: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lps_logout_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub fw_tag: u8,
    pub rsvd: [u8; 3],
    pub port_name: wwn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lps_logout_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub bfa_tag: u8,
    pub status: u8,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lps_cvl_event_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub bfa_tag: u8,
    pub rsvd: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_lps_n2n_pid_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub fw_tag: u8,
    pub lp_pid:24: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_lps_h2i_msg_u {
    pub msg: *mut bfi_mhdr_s,
    pub login_req: *mut bfi_lps_login_req_s,
    pub logout_req: *mut bfi_lps_logout_req_s,
    pub n2n_pid_req: *mut bfi_lps_n2n_pid_req_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_lps_i2h_msg_u {
    pub msg: *mut bfi_msg_s,
    pub login_rsp: *mut bfi_lps_login_rsp_s,
    pub logout_rsp: *mut bfi_lps_logout_rsp_s,
    pub cvl_event: *mut bfi_lps_cvl_event_s,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_rport_h2i_msgs {
    BFI_RPORT_H2I_CREATE_REQ = 1,
    BFI_RPORT_H2I_DELETE_REQ = 2,
    BFI_RPORT_H2I_SET_SPEED_REQ  = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_rport_i2h_msgs {
    BFI_RPORT_I2H_CREATE_RSP = BFA_I2HM(1),
    BFI_RPORT_I2H_DELETE_RSP = BFA_I2HM(2),
    BFI_RPORT_I2H_QOS_SCN    = BFA_I2HM(3),
    BFI_RPORT_I2H_LIP_SCN_ONLINE =	BFA_I2HM(4),
    BFI_RPORT_I2H_LIP_SCN_OFFLINE = BFA_I2HM(5),
    BFI_RPORT_I2H_NO_DEV	= BFA_I2HM(6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_rport_create_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 bfa_handle; / host rport handle,
    pub /: *mut *mut __be16 max_frmsz; / max rcv pdu size,
    pub /: *mut *mut lp_fwtag:8; / local port tag,
    pub /: *mut *mut u8 fc_class; / supported FC classes,
    pub /: *mut *mut u8 vf_en; / virtual fabric enable,
    pub /: *mut *mut u16 vf_id; / virtual fabric ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_rport_create_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u8 status; / rport creation status,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u16 bfa_handle; / host rport handle,
    pub /: *mut *mut u16 fw_handle; / firmware rport handle,
    pub /: *mut *mut bfa_rport_qos_attr_s qos_attr; / QoS Attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_speed_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 fw_handle; / firmware rport handle,
    pub /: *mut *mut u8 speed; / rport's speed via RPSC,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_rport_delete_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 fw_handle; / firmware rport handle,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_rport_delete_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 bfa_handle; / host rport handle,
    pub /: *mut *mut u8 status; / rport deletion status,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_rport_qos_scn_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 bfa_handle; / host rport handle,
    pub rsvd: u16,
    pub /: *mut *mut bfa_rport_qos_attr_s old_qos_attr; / Old QoS Attributes,
    pub /: *mut *mut bfa_rport_qos_attr_s new_qos_attr; / New QoS Attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_rport_lip_scn_s {
    pub /: *mut *mut bfi_mhdr_s mh; /!< common msg header,
    pub /: *mut *mut u16 bfa_handle; /!< host rport handle,
    pub /: *mut *mut u8 status; /!< scn online status,
    pub rsvd: u8,
    pub loop_info: bfa_fcport_loop_info_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_rport_h2i_msg_u {
    pub msg: *mut bfi_msg_s,
    pub create_req: *mut bfi_rport_create_req_s,
    pub delete_req: *mut bfi_rport_delete_req_s,
    pub speed_req: *mut bfi_rport_speed_req_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_rport_i2h_msg_u {
    pub msg: *mut bfi_msg_s,
    pub create_rsp: *mut bfi_rport_create_rsp_s,
    pub delete_rsp: *mut bfi_rport_delete_rsp_s,
    pub qos_scn_evt: *mut bfi_rport_qos_scn_s,
    pub lip_scn: *mut bfi_rport_lip_scn_s,
}

//
// Initiator mode I-T nexus interface defines.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_itn_h2i {
    BFI_ITN_H2I_CREATE_REQ = 1,	/*  i-t nexus creation */
    BFI_ITN_H2I_DELETE_REQ = 2,	/*  i-t nexus deletion */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_itn_i2h {
    BFI_ITN_I2H_CREATE_RSP = BFA_I2HM(1),
    BFI_ITN_I2H_DELETE_RSP = BFA_I2HM(2),
    BFI_ITN_I2H_SLER_EVENT = BFA_I2HM(3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_itn_create_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 fw_handle; / f/w handle for itnim,
    pub /: *mut *mut u8 class; / FC class for IO,
    pub /: *mut *mut u8 seq_rec; / sequence recovery support,
    pub /: *mut *mut u8 msg_no; / seq id of the msg,
    pub role: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_itn_create_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 bfa_handle; / bfa handle for itnim,
    pub /: *mut *mut u8 status; / fcp request status,
    pub /: *mut *mut u8 seq_id; / seq id of the msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_itn_delete_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 fw_handle; / f/w itnim handle,
    pub /: *mut *mut u8 seq_id; / seq id of the msg,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_itn_delete_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 bfa_handle; / bfa handle for itnim,
    pub /: *mut *mut u8 status; / fcp request status,
    pub /: *mut *mut u8 seq_id; / seq id of the msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_itn_sler_event_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut u16 bfa_handle; / bfa handle for itnim,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_itn_h2i_msg_u {
    pub create_req: *mut bfi_itn_create_req_s,
    pub delete_req: *mut bfi_itn_delete_req_s,
    pub msg: *mut bfi_msg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfi_itn_i2h_msg_u {
    pub create_rsp: *mut bfi_itn_create_rsp_s,
    pub delete_rsp: *mut bfi_itn_delete_rsp_s,
    pub sler_event: *mut bfi_itn_sler_event_s,
    pub msg: *mut bfi_msg_s,
}

//
// Initiator mode IO interface defines.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioim_h2i {
    BFI_IOIM_H2I_IOABORT_REQ = 1,	/*  IO abort request	 */
    BFI_IOIM_H2I_IOCLEANUP_REQ = 2,	/*  IO cleanup request	 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioim_i2h {
    BFI_IOIM_I2H_IO_RSP = BFA_I2HM(1),	/*  non-fp IO response	 */
    BFI_IOIM_I2H_IOABORT_RSP = BFA_I2HM(2),	/*  ABORT rsp	 */
}

//
// IO command DIF info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioim_dif_s {
    pub dif_info: [u32; 4],
}

//
// FCP IO messages overview
//
// @note
// - Max CDB length supported is 64 bytes.
// - SCSI Linked commands and SCSI bi-directional Commands not
// supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioim_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 io_tag; / I/O tag,
    pub /: *mut *mut u16 rport_hdl; / itnim/rport firmware handle,
    pub /: *mut *mut fcp_cmnd_s cmnd; / IO request info,
//
// SG elements array within the IO request must be double word
// aligned. This alignment is required to optimize SGM setup for the IO.
//
    pub sges: [bfi_sge_s; BFI_SGE_INLINE_MAX],
    pub io_timeout: u8,
    pub dif_en: u8,
    pub rsvd_a: [u8; 2],
    pub dif: bfi_ioim_dif_s,
}

//
// This table shows various IO status codes from firmware and their
// meaning. Host driver can use these status codes to further process
// IO completions.
//
// BFI_IOIM_STS_OK		: IO completed with error free SCSI &
// transport status.
// io-tag can be reused.
//
// BFA_IOIM_STS_SCSI_ERR		: IO completed with scsi error.
// - io-tag can be reused.
//
// BFI_IOIM_STS_HOST_ABORTED	: IO was aborted successfully due to
// host request.
// - io-tag cannot be reused yet.
//
// BFI_IOIM_STS_ABORTED		: IO was aborted successfully
// internally by f/w.
// - io-tag cannot be reused yet.
//
// BFI_IOIM_STS_TIMEDOUT	: IO timedout and ABTS/RRQ is happening
// in the firmware and
// - io-tag cannot be reused yet.
//
// BFI_IOIM_STS_SQER_NEEDED	: Firmware could not recover the IO
// with sequence level error
// logic and hence host needs to retry
// this IO with a different IO tag
// - io-tag cannot be used yet.
//
// BFI_IOIM_STS_NEXUS_ABORT	: Second Level Error Recovery from host
// is required because 2 consecutive ABTS
// timedout and host needs logout and
// re-login with the target
// - io-tag cannot be used yet.
//
// BFI_IOIM_STS_UNDERRUN	: IO completed with SCSI status good,
// but the data tranferred is less than
// the fcp data length in the command.
// ex. SCSI INQUIRY where transferred
// data length and residue count in FCP
// response accounts for total fcp-dl
// - io-tag can be reused.
//
// BFI_IOIM_STS_OVERRUN	: IO completed with SCSI status good,
// but the data transerred is more than
// fcp data length in the command. ex.
// TAPE IOs where blocks can of unequal
// lengths.
// - io-tag can be reused.
//
// BFI_IOIM_STS_RES_FREE	: Firmware has completed using io-tag
// during abort process
// - io-tag can be reused.
//
// BFI_IOIM_STS_PROTO_ERR	: Firmware detected a protocol error.
// ex target sent more data than
// requested, or there was data frame
// loss and other reasons
// - io-tag cannot be used yet.
//
// BFI_IOIM_STS_DIF_ERR	: Firwmare detected DIF error. ex: DIF
// CRC err or Ref Tag err or App tag err.
// - io-tag can be reused.
//
// BFA_IOIM_STS_TSK_MGT_ABORT	: IO was aborted because of Task
// Management command from the host
// - io-tag can be reused.
//
// BFI_IOIM_STS_UTAG		: Firmware does not know about this
// io_tag.
// - io-tag can be reused.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_ioim_status {
    BFI_IOIM_STS_OK = 0,
    BFI_IOIM_STS_HOST_ABORTED = 1,
    BFI_IOIM_STS_ABORTED = 2,
    BFI_IOIM_STS_TIMEDOUT = 3,
    BFI_IOIM_STS_RES_FREE = 4,
    BFI_IOIM_STS_SQER_NEEDED = 5,
    BFI_IOIM_STS_PROTO_ERR = 6,
    BFI_IOIM_STS_UTAG = 7,
    BFI_IOIM_STS_PATHTOV = 8,
}

//
// I/O response message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioim_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / common msg header,
    pub /: *mut *mut __be16 io_tag; / completed IO tag,
    pub /: *mut *mut u16 bfa_rport_hndl; / releated rport handle,
    pub /: *mut *mut u8 io_status; / IO completion status,
    pub /: *mut *mut u8 reuse_io_tag; / IO tag can be reused,
    pub /: *mut *mut u16 abort_tag; / host abort request tag,
    pub /: *mut *mut u8 scsi_status; / scsi status from target,
    pub /: *mut *mut u8 sns_len; / scsi sense length,
    pub /: *mut *mut u8 resid_flags; / IO residue flags,
    pub rsvd_a: u8,
    pub /: *mut *mut __be32 residue; / IO residual length in bytes,
    pub rsvd_b: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_ioim_abort_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 io_tag; / I/O tag,
    pub /: *mut *mut u16 abort_tag; / unique request tag,
}

//
// Initiator mode task management command interface defines.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_tskim_h2i {
    BFI_TSKIM_H2I_TM_REQ	= 1, /*  task-mgmt command	*/
    BFI_TSKIM_H2I_ABORT_REQ = 2, /*  task-mgmt command	*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_tskim_i2h {
    BFI_TSKIM_I2H_TM_RSP = BFA_I2HM(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_tskim_req_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 tsk_tag; / task management tag,
    pub /: *mut *mut u16 itn_fhdl; / itn firmware handle,
    pub /: *mut *mut scsi_lun lun; / LU number,
    pub /: *mut *mut u8 tm_flags; / see enum fcp_tm_cmnd,
    pub /: *mut *mut u8 t_secs; / Timeout value in seconds,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_tskim_abortreq_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 tsk_tag; / task management tag,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfi_tskim_status {
//
// Following are FCP-4 spec defined status codes,
// **DO NOT CHANGE THEM
//
    BFI_TSKIM_STS_OK	= 0,
    BFI_TSKIM_STS_NOT_SUPP = 4,
    BFI_TSKIM_STS_FAILED	= 5,

//
// Defined by BFA
//
    BFI_TSKIM_STS_TIMEOUT  = 10,	/*  TM request timedout	*/
    BFI_TSKIM_STS_ABORTED  = 11,	/*  Aborted on host request */
    BFI_TSKIM_STS_UTAG     = 12,	/*  unknown tag for request */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfi_tskim_rsp_s {
    pub /: *mut *mut bfi_mhdr_s mh; / Common msg header,
    pub /: *mut *mut __be16 tsk_tag; / task mgmt cmnd tag,
    pub /: *mut *mut u8 tsk_status; / @ref bfi_tskim_status,
    pub rsvd: u8,
}

//
// Crossbow PCI MSI-X vector defines
//
// Catapult FC PCI MSI-X vector defines
//
