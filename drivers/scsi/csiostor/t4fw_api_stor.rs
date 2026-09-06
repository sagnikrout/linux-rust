//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/t4fw_api_stor.h
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


//
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2009-2010 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// R E T U R N   V A L U E S
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_fcoe_link_sub_op {
    FCOE_LINK_DOWN	= 0x0,
    FCOE_LINK_UP	= 0x1,
    FCOE_LINK_COND	= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_fcoe_link_status {
    FCOE_LINKDOWN	= 0x0,
    FCOE_LINKUP	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ofld_prot {
    PROT_FCOE	= 0x1,
    PROT_ISCSI	= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rport_type_fcoe {
    FLOGI_VFPORT	= 0x1,		/* 0xfffffe */
    FDISC_VFPORT	= 0x2,		/* 0xfffffe */
    NS_VNPORT	= 0x3,		/* 0xfffffc */
    REG_FC4_VNPORT	= 0x4,		/* any FC4 type VN_PORT */
    REG_VNPORT	= 0x5,		/* 0xfffxxx - non FC4 port in switch */
    FDMI_VNPORT	= 0x6,		/* 0xfffffa */
    FAB_CTLR_VNPORT	= 0x7,		/* 0xfffffd */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum event_cause_fcoe {
    PLOGI_ACC_RCVD		= 0x01,
    PLOGI_RJT_RCVD		= 0x02,
    PLOGI_RCVD		= 0x03,
    PLOGO_RCVD		= 0x04,
    PRLI_ACC_RCVD		= 0x05,
    PRLI_RJT_RCVD		= 0x06,
    PRLI_RCVD		= 0x07,
    PRLO_RCVD		= 0x08,
    NPORT_ID_CHGD		= 0x09,
    FLOGO_RCVD		= 0x0a,
    CLR_VIRT_LNK_RCVD	= 0x0b,
    FLOGI_ACC_RCVD		= 0x0c,
    FLOGI_RJT_RCVD		= 0x0d,
    FDISC_ACC_RCVD		= 0x0e,
    FDISC_RJT_RCVD		= 0x0f,
    FLOGI_TMO_MAX_RETRY	= 0x10,
    IMPL_LOGO_ADISC_ACC	= 0x11,
    IMPL_LOGO_ADISC_RJT	= 0x12,
    IMPL_LOGO_ADISC_CNFLT	= 0x13,
    PRLI_TMO		= 0x14,
    ADISC_TMO		= 0x15,
    RSCN_DEV_LOST		= 0x16,
    SCR_ACC_RCVD		= 0x17,
    ADISC_RJT_RCVD		= 0x18,
    LOGO_SNT		= 0x19,
    PROTO_ERR_IMPL_LOGO	= 0x1a,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_cmn_type {
    FCOE_ELS,
    FCOE_CT,
    FCOE_SCSI_CMD,
    FCOE_UNSOL_ELS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_wr_stor_opcodes {
    FW_RDEV_WR                     = 0x38,
    FW_FCOE_ELS_CT_WR              = 0x30,
    FW_SCSI_WRITE_WR               = 0x31,
    FW_SCSI_READ_WR                = 0x32,
    FW_SCSI_CMD_WR                 = 0x33,
    FW_SCSI_ABRT_CLS_WR            = 0x34,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rdev_wr {
    pub op_to_immdlen: __be32,
    pub alloc_to_len16: __be32,
    pub cookie: __be64,
    pub protocol: u8,
    pub event_cause: u8,
    pub cur_state: u8,
    pub prev_state: u8,
    pub flags_to_assoc_flowid: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union rdev_entry {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rdev_entry {
    pub flowid: __be32,
    pub protocol: u8,
    pub event_cause: u8,
    pub flags: u8,
    pub rjt_reason: u8,
    pub cur_login_st: u8,
    pub prev_login_st: u8,
    pub rcv_fr_sz: __be16,
    pub rd_xfer_rdy_to_rport_type: u8,
    pub vft_to_qos: u8,
    pub org_proc_assoc_to_acc_rsp_code: u8,
    pub enh_disc_to_tgt: u8,
    pub wwnn: [u8; 8],
    pub wwpn: [u8; 8],
    pub iqid: __be16,
    pub fc_oui: [u8; 3],
    pub r_id: [u8; 3],
    pub fcoe_rdev: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_rdev_entry {
    pub flowid: __be32,
    pub protocol: u8,
    pub event_cause: u8,
    pub flags: u8,
    pub r3: u8,
    pub iscsi_opts: __be16,
    pub tcp_opts: __be16,
    pub ip_opts: __be16,
    pub max_rcv_len: __be16,
    pub max_snd_len: __be16,
    pub first_brst_len: __be16,
    pub max_brst_len: __be16,
    pub r4: __be16,
    pub def_time2wait: __be16,
    pub def_time2ret: __be16,
    pub nop_out_intrvl: __be16,
    pub non_scsi_to: __be16,
    pub isid: __be16,
    pub tsid: __be16,
    pub port: __be16,
    pub tpgt: __be16,
    pub r5: [u8; 6],
    pub iqid: __be16,
    pub iscsi_rdev: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_els_ct_wr {
    pub op_immdlen: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
    pub iqid: __be16,
    pub tmo_val: u8,
    pub els_ct_type: u8,
    pub ctl_pri: u8,
    pub cp_en_class: u8,
    pub xfer_cnt: __be16,
    pub fl_to_sp: u8,
    pub l_id: [u8; 3],
    pub r5: u8,
    pub r_id: [u8; 3],
    pub rsp_dmaaddr: __be64,
    pub rsp_dmalen: __be32,
    pub r6: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_scsi_write_wr {
    pub op_immdlen: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
    pub iqid: __be16,
    pub tmo_val: u8,
    pub use_xfer_cnt: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_scsi_write_priv {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_write_priv {
    pub ctl_pri: u8,
    pub cp_en_class: u8,
    pub r3_lo: [u8; 2],
    pub fcoe: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_write_priv {
    pub r3: [u8; 4],
    pub iscsi: },
    pub u: },
    pub xfer_cnt: __be32,
    pub ini_xfer_cnt: __be32,
    pub rsp_dmaaddr: __be64,
    pub rsp_dmalen: __be32,
    pub r4: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_scsi_read_wr {
    pub op_immdlen: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
    pub iqid: __be16,
    pub tmo_val: u8,
    pub use_xfer_cnt: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_scsi_read_priv {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_read_priv {
    pub ctl_pri: u8,
    pub cp_en_class: u8,
    pub r3_lo: [u8; 2],
    pub fcoe: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_read_priv {
    pub r3: [u8; 4],
    pub iscsi: },
    pub u: },
    pub xfer_cnt: __be32,
    pub ini_xfer_cnt: __be32,
    pub rsp_dmaaddr: __be64,
    pub rsp_dmalen: __be32,
    pub r4: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_scsi_cmd_wr {
    pub op_immdlen: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
    pub iqid: __be16,
    pub tmo_val: u8,
    pub r3: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_scsi_cmd_priv {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cmd_priv {
    pub ctl_pri: u8,
    pub cp_en_class: u8,
    pub r4_lo: [u8; 2],
    pub fcoe: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cmd_priv {
    pub r4: [u8; 4],
    pub iscsi: },
    pub u: },
    pub r5: [u8; 8],
    pub rsp_dmaaddr: __be64,
    pub rsp_dmalen: __be32,
    pub r6: __be32,
}

pub const SCSI_ABORT: c_int = 0;
pub const SCSI_CLOSE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_scsi_abrt_cls_wr {
    pub op_immdlen: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
    pub iqid: __be16,
    pub tmo_val: u8,
    pub sub_opcode_to_chk_all_io: u8,
    pub r3: [u8; 4],
    pub t_cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_cmd_stor_opcodes {
    FW_FCOE_RES_INFO_CMD           = 0x31,
    FW_FCOE_LINK_CMD               = 0x32,
    FW_FCOE_VNP_CMD                = 0x33,
    FW_FCOE_SPARAMS_CMD            = 0x35,
    FW_FCOE_STATS_CMD              = 0x37,
    FW_FCOE_FCF_CMD                = 0x38,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_res_info_cmd {
    pub op_to_read: __be32,
    pub retval_len16: __be32,
    pub e_d_tov: __be16,
    pub r_a_tov_seq: __be16,
    pub r_a_tov_els: __be16,
    pub r_r_tov: __be16,
    pub max_xchgs: __be32,
    pub max_ssns: __be32,
    pub used_xchgs: __be32,
    pub used_ssns: __be32,
    pub max_fcfs: __be32,
    pub max_vnps: __be32,
    pub used_fcfs: __be32,
    pub used_vnps: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_link_cmd {
    pub op_to_portid: __be32,
    pub retval_len16: __be32,
    pub sub_opcode_fcfi: __be32,
    pub r3: u8,
    pub lstatus: u8,
    pub flags: __be16,
    pub r4: u8,
    pub set_vlan: u8,
    pub vlan_id: __be16,
    pub vnpi_pkd: __be32,
    pub r6: __be16,
    pub phy_mac: [u8; 6],
    pub vnport_wwnn: [u8; 8],
    pub vnport_wwpn: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_vnp_cmd {
    pub op_to_fcfi: __be32,
    pub alloc_to_len16: __be32,
    pub gen_wwn_to_vnpi: __be32,
    pub vf_id: __be32,
    pub iqid: __be16,
    pub vnport_mac: [u8; 6],
    pub vnport_wwnn: [u8; 8],
    pub vnport_wwpn: [u8; 8],
    pub cmn_srv_parms: [u8; 16],
    pub clsp_word_0_1: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_sparams_cmd {
    pub op_to_portid: __be32,
    pub retval_len16: __be32,
    pub r3: [u8; 7],
    pub cos: u8,
    pub lport_wwnn: [u8; 8],
    pub lport_wwpn: [u8; 8],
    pub cmn_srv_parms: [u8; 16],
    pub cls_srv_parms: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_stats_cmd {
    pub op_to_flowid: __be32,
    pub free_to_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_fcoe_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_stats_ctl {
    pub nstats_port: u8,
    pub port_valid_ix: u8,
    pub r6: __be16,
    pub r7: __be32,
    pub stat0: __be64,
    pub stat1: __be64,
    pub stat2: __be64,
    pub stat3: __be64,
    pub stat4: __be64,
    pub stat5: __be64,
    pub ctl: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_port_stats {
    pub tx_bcast_bytes: __be64,
    pub tx_bcast_frames: __be64,
    pub tx_mcast_bytes: __be64,
    pub tx_mcast_frames: __be64,
    pub tx_ucast_bytes: __be64,
    pub tx_ucast_frames: __be64,
    pub tx_drop_frames: __be64,
    pub tx_offload_bytes: __be64,
    pub tx_offload_frames: __be64,
    pub rx_bcast_bytes: __be64,
    pub rx_bcast_frames: __be64,
    pub rx_mcast_bytes: __be64,
    pub rx_mcast_frames: __be64,
    pub rx_ucast_bytes: __be64,
    pub rx_ucast_frames: __be64,
    pub rx_err_frames: __be64,
    pub port_stats: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_fcf_stats {
    pub fip_tx_bytes: __be32,
    pub fip_tx_fr: __be32,
    pub fcf_ka: __be64,
    pub mcast_adv_rcvd: __be64,
    pub ucast_adv_rcvd: __be16,
    pub sol_sent: __be16,
    pub vlan_req: __be16,
    pub vlan_rpl: __be16,
    pub clr_vlink: __be16,
    pub link_down: __be16,
    pub link_up: __be16,
    pub logo: __be16,
    pub flogi_req: __be16,
    pub flogi_rpl: __be16,
    pub fdisc_req: __be16,
    pub fdisc_rpl: __be16,
    pub fka_prd_chg: __be16,
    pub fc_map_chg: __be16,
    pub vfid_chg: __be16,
    pub no_fka_req: u8,
    pub no_vnp: u8,
    pub fcf_stats: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_pcb_stats {
    pub tx_bytes: __be64,
    pub tx_frames: __be64,
    pub rx_bytes: __be64,
    pub rx_frames: __be64,
    pub vnp_ka: __be32,
    pub unsol_els_rcvd: __be32,
    pub unsol_cmd_rcvd: __be64,
    pub implicit_logo: __be16,
    pub flogi_inv_sparm: __be16,
    pub fdisc_inv_sparm: __be16,
    pub flogi_rjt: __be16,
    pub fdisc_rjt: __be16,
    pub no_ssn: __be16,
    pub mac_flt_fail: __be16,
    pub inv_fr_rcvd: __be16,
    pub pcb_stats: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_scb_stats {
    pub tx_bytes: __be64,
    pub tx_frames: __be64,
    pub rx_bytes: __be64,
    pub rx_frames: __be64,
    pub host_abrt_req: __be32,
    pub adap_auto_abrt: __be32,
    pub adap_abrt_rsp: __be32,
    pub host_ios_req: __be32,
    pub ssn_offl_ios: __be16,
    pub ssn_not_rdy_ios: __be16,
    pub rx_data_ddp_err: u8,
    pub ddp_flt_set_err: u8,
    pub rx_data_fr_err: __be16,
    pub bad_st_abrt_req: u8,
    pub no_io_abrt_req: u8,
    pub abort_tmo: u8,
    pub abort_tmo_2: u8,
    pub abort_req: __be32,
    pub no_ppod_res_tmo: u8,
    pub bp_tmo: u8,
    pub adap_auto_cls: u8,
    pub no_io_cls_req: u8,
    pub host_cls_req: __be32,
    pub unsol_cmd_rcvd: __be64,
    pub plogi_req_rcvd: __be32,
    pub prli_req_rcvd: __be32,
    pub logo_req_rcvd: __be16,
    pub prlo_req_rcvd: __be16,
    pub plogi_rjt_rcvd: __be16,
    pub prli_rjt_rcvd: __be16,
    pub adisc_req_rcvd: __be32,
    pub rscn_rcvd: __be32,
    pub rrq_req_rcvd: __be32,
    pub unsol_els_rcvd: __be32,
    pub adisc_rjt_rcvd: u8,
    pub scr_rjt: u8,
    pub ct_rjt: u8,
    pub inval_bls_rcvd: u8,
    pub ba_rjt_rcvd: __be32,
    pub scb_stats: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_fcoe_fcf_cmd {
    pub op_to_fcfi: __be32,
    pub retval_len16: __be32,
    pub priority_pkd: __be16,
    pub mac: [u8; 6],
    pub name_id: [u8; 8],
    pub fabric: [u8; 8],
    pub vf_id: __be16,
    pub max_fcoe_size: __be16,
    pub vlan_id: u8,
    pub fc_map: [u8; 3],
    pub fka_adv: __be32,
    pub r6: __be32,
    pub r7_hi: u8,
    pub fpma_to_portid: u8,
    pub spma_mac: [u8; 6],
    pub r8: __be64,
}

