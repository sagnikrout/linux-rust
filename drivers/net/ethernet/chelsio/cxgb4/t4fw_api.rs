//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4fw_api.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2009-2016 Chelsio Communications, Inc. All rights reserved.
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_retval {
    FW_SUCCESS		= 0,	/* completed successfully */
    FW_EPERM		= 1,	/* operation not permitted */
    FW_ENOENT		= 2,	/* no such file or directory */
    FW_EIO			= 5,	/* input/output error; hw bad */
    FW_ENOEXEC		= 8,	/* exec format error; inv microcode */
    FW_EAGAIN		= 11,	/* try again */
    FW_ENOMEM		= 12,	/* out of memory */
    FW_EFAULT		= 14,	/* bad address; fw bad */
    FW_EBUSY		= 16,	/* resource busy */
    FW_EEXIST		= 17,	/* file exists */
    FW_ENODEV		= 19,	/* no such device */
    FW_EINVAL		= 22,	/* invalid argument */
    FW_ENOSPC		= 28,	/* no space left on device */
    FW_ENOSYS		= 38,	/* functionality not implemented */
    FW_ENODATA		= 61,	/* no data available */
    FW_EPROTO		= 71,	/* protocol error */
    FW_EADDRINUSE		= 98,	/* address already in use */
    FW_EADDRNOTAVAIL	= 99,	/* cannot assigned requested address */
    FW_ENETDOWN		= 100,	/* network is down */
    FW_ENETUNREACH		= 101,	/* network is unreachable */
    FW_ENOBUFS		= 105,	/* no buffer space available */
    FW_ETIMEDOUT		= 110,	/* timeout */
    FW_EINPROGRESS		= 115,	/* fw internal */
    FW_SCSI_ABORT_REQUESTED	= 128,	/* */
    FW_SCSI_ABORT_TIMEDOUT	= 129,	/* */
    FW_SCSI_ABORTED		= 130,	/* */
    FW_SCSI_CLOSE_REQUESTED	= 131,	/* */
    FW_ERR_LINK_DOWN	= 132,	/* */
    FW_RDEV_NOT_READY	= 133,	/* */
    FW_ERR_RDEV_LOST	= 134,	/* */
    FW_ERR_RDEV_LOGO	= 135,	/* */
    FW_FCOE_NO_XCHG		= 136,	/* */
    FW_SCSI_RSP_ERR		= 137,	/* */
    FW_ERR_RDEV_IMPL_LOGO	= 138,	/* */
    FW_SCSI_UNDER_FLOW_ERR  = 139,	/* */
    FW_SCSI_OVER_FLOW_ERR   = 140,	/* */
    FW_SCSI_DDP_ERR		= 141,	/* DDP error*/
    FW_SCSI_TASK_ERR	= 142,	/* No SCSI tasks available */
}

pub const FW_T4VF_SGE_BASE_ADDR: c_uint = 0x0000;
pub const FW_T4VF_MPS_BASE_ADDR: c_uint = 0x0100;
pub const FW_T4VF_PL_BASE_ADDR: c_uint = 0x0200;
pub const FW_T4VF_MBDATA_BASE_ADDR: c_uint = 0x0240;
pub const FW_T4VF_CIM_BASE_ADDR: c_uint = 0x0300;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_wr_opcodes {
    FW_FILTER_WR                   = 0x02,
    FW_ULPTX_WR                    = 0x04,
    FW_TP_WR                       = 0x05,
    FW_ETH_TX_PKT_WR               = 0x08,
    FW_ETH_TX_EO_WR                = 0x1c,
    FW_OFLD_CONNECTION_WR          = 0x2f,
    FW_FLOWC_WR                    = 0x0a,
    FW_OFLD_TX_DATA_WR             = 0x0b,
    FW_CMD_WR                      = 0x10,
    FW_ETH_TX_PKT_VM_WR            = 0x11,
    FW_RI_RES_WR                   = 0x0c,
    FW_RI_INIT_WR                  = 0x0d,
    FW_RI_RDMA_WRITE_WR            = 0x14,
    FW_RI_SEND_WR                  = 0x15,
    FW_RI_RDMA_READ_WR             = 0x16,
    FW_RI_RECV_WR                  = 0x17,
    FW_RI_BIND_MW_WR               = 0x18,
    FW_RI_FR_NSMR_WR               = 0x19,
    FW_RI_FR_NSMR_TPTE_WR	       = 0x20,
    FW_RI_RDMA_WRITE_CMPL_WR       = 0x21,
    FW_RI_INV_LSTAG_WR             = 0x1a,
    FW_ISCSI_TX_DATA_WR	       = 0x45,
    FW_PTP_TX_PKT_WR               = 0x46,
    FW_TLSTX_DATA_WR	       = 0x68,
    FW_CRYPTO_LOOKASIDE_WR         = 0X6d,
    FW_LASTC2E_WR                  = 0x70,
    FW_FILTER2_WR		       = 0x77
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_wr_hdr {
    pub hi: __be32,
    pub lo: __be32,
}

// work request opcode (hi)
pub const FW_WR_OP_S: c_int = 24;
pub const FW_WR_OP_M: c_uint = 0xff;

// atomic flag (hi) - firmware encapsulates CPLs in CPL_BARRIER
pub const FW_WR_ATOMIC_S: c_int = 23;

// flush flag (hi) - firmware flushes flushable work request buffered
// in the flow context.
//
pub const FW_WR_FLUSH_S: c_int = 22;

// completion flag (hi) - firmware generates a cpl_fw6_ack
pub const FW_WR_COMPL_S: c_int = 21;

// work request immediate data length (hi)
pub const FW_WR_IMMDLEN_S: c_int = 0;
pub const FW_WR_IMMDLEN_M: c_uint = 0xff;

// egress queue status update to associated ingress queue entry (lo)
pub const FW_WR_EQUIQ_S: c_int = 31;

// egress queue status update to egress queue status entry (lo)
pub const FW_WR_EQUEQ_S: c_int = 30;

// flow context identifier (lo)
pub const FW_WR_FLOWID_S: c_int = 8;

// length in units of 16-bytes (lo)
pub const FW_WR_LEN16_S: c_int = 0;

// filter wr reply code in cookie in CPL_SET_TCB_RPL
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_filter_wr_cookie {
    FW_FILTER_WR_SUCCESS,
    FW_FILTER_WR_FLT_ADDED,
    FW_FILTER_WR_FLT_DELETED,
    FW_FILTER_WR_SMT_TBL_FULL,
    FW_FILTER_WR_EINVAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_filter_wr {
    pub op_pkd: __be32,
    pub len16_pkd: __be32,
    pub r3: __be64,
    pub tid_to_iq: __be32,
    pub del_filter_to_l2tix: __be32,
    pub ethtype: __be16,
    pub ethtypem: __be16,
    pub frag_to_ovlan_vldm: __u8,
    pub smac_sel: __u8,
    pub rx_chan_rx_rpl_iq: __be16,
    pub maci_to_matchtypem: __be32,
    pub ptcl: __u8,
    pub ptclm: __u8,
    pub ttyp: __u8,
    pub ttypm: __u8,
    pub ivlan: __be16,
    pub ivlanm: __be16,
    pub ovlan: __be16,
    pub ovlanm: __be16,
    pub lip: [__u8; 16],
    pub lipm: [__u8; 16],
    pub fip: [__u8; 16],
    pub fipm: [__u8; 16],
    pub lp: __be16,
    pub lpm: __be16,
    pub fp: __be16,
    pub fpm: __be16,
    pub r7: __be16,
    pub sma: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_filter2_wr {
    pub op_pkd: __be32,
    pub len16_pkd: __be32,
    pub r3: __be64,
    pub tid_to_iq: __be32,
    pub del_filter_to_l2tix: __be32,
    pub ethtype: __be16,
    pub ethtypem: __be16,
    pub frag_to_ovlan_vldm: __u8,
    pub smac_sel: __u8,
    pub rx_chan_rx_rpl_iq: __be16,
    pub maci_to_matchtypem: __be32,
    pub ptcl: __u8,
    pub ptclm: __u8,
    pub ttyp: __u8,
    pub ttypm: __u8,
    pub ivlan: __be16,
    pub ivlanm: __be16,
    pub ovlan: __be16,
    pub ovlanm: __be16,
    pub lip: [__u8; 16],
    pub lipm: [__u8; 16],
    pub fip: [__u8; 16],
    pub fipm: [__u8; 16],
    pub lp: __be16,
    pub lpm: __be16,
    pub fp: __be16,
    pub fpm: __be16,
    pub r7: __be16,
    pub sma: [__u8; 6],
    pub r8: __be16,
    pub filter_type_swapmac: __u8,
    pub natmode_to_ulp_type: __u8,
    pub newlport: __be16,
    pub newfport: __be16,
    pub newlip: [__u8; 16],
    pub newfip: [__u8; 16],
    pub natseqcheck: __be32,
    pub r9: __be32,
    pub r10: __be64,
    pub r11: __be64,
    pub r12: __be64,
    pub r13: __be64,
}

pub const FW_FILTER_WR_TID_S: c_int = 12;
pub const FW_FILTER_WR_TID_M: c_uint = 0xfffff;

pub const FW_FILTER_WR_RQTYPE_S: c_int = 11;
pub const FW_FILTER_WR_RQTYPE_M: c_uint = 0x1;

pub const FW_FILTER_WR_NOREPLY_S: c_int = 10;
pub const FW_FILTER_WR_NOREPLY_M: c_uint = 0x1;

pub const FW_FILTER_WR_IQ_S: c_int = 0;
pub const FW_FILTER_WR_IQ_M: c_uint = 0x3ff;

pub const FW_FILTER_WR_DEL_FILTER_S: c_int = 31;
pub const FW_FILTER_WR_DEL_FILTER_M: c_uint = 0x1;

pub const FW_FILTER_WR_RPTTID_S: c_int = 25;
pub const FW_FILTER_WR_RPTTID_M: c_uint = 0x1;

pub const FW_FILTER_WR_DROP_S: c_int = 24;
pub const FW_FILTER_WR_DROP_M: c_uint = 0x1;

pub const FW_FILTER_WR_DIRSTEER_S: c_int = 23;
pub const FW_FILTER_WR_DIRSTEER_M: c_uint = 0x1;

pub const FW_FILTER_WR_MASKHASH_S: c_int = 22;
pub const FW_FILTER_WR_MASKHASH_M: c_uint = 0x1;

pub const FW_FILTER_WR_DIRSTEERHASH_S: c_int = 21;
pub const FW_FILTER_WR_DIRSTEERHASH_M: c_uint = 0x1;

pub const FW_FILTER_WR_LPBK_S: c_int = 20;
pub const FW_FILTER_WR_LPBK_M: c_uint = 0x1;

pub const FW_FILTER_WR_DMAC_S: c_int = 19;
pub const FW_FILTER_WR_DMAC_M: c_uint = 0x1;

pub const FW_FILTER_WR_SMAC_S: c_int = 18;
pub const FW_FILTER_WR_SMAC_M: c_uint = 0x1;

pub const FW_FILTER_WR_INSVLAN_S: c_int = 17;
pub const FW_FILTER_WR_INSVLAN_M: c_uint = 0x1;

pub const FW_FILTER_WR_RMVLAN_S: c_int = 16;
pub const FW_FILTER_WR_RMVLAN_M: c_uint = 0x1;

pub const FW_FILTER_WR_HITCNTS_S: c_int = 15;
pub const FW_FILTER_WR_HITCNTS_M: c_uint = 0x1;

pub const FW_FILTER_WR_TXCHAN_S: c_int = 13;
pub const FW_FILTER_WR_TXCHAN_M: c_uint = 0x3;

pub const FW_FILTER_WR_PRIO_S: c_int = 12;
pub const FW_FILTER_WR_PRIO_M: c_uint = 0x1;

pub const FW_FILTER_WR_L2TIX_S: c_int = 0;
pub const FW_FILTER_WR_L2TIX_M: c_uint = 0xfff;

pub const FW_FILTER_WR_FRAG_S: c_int = 7;
pub const FW_FILTER_WR_FRAG_M: c_uint = 0x1;

pub const FW_FILTER_WR_FRAGM_S: c_int = 6;
pub const FW_FILTER_WR_FRAGM_M: c_uint = 0x1;

pub const FW_FILTER_WR_IVLAN_VLD_S: c_int = 5;
pub const FW_FILTER_WR_IVLAN_VLD_M: c_uint = 0x1;

pub const FW_FILTER_WR_OVLAN_VLD_S: c_int = 4;
pub const FW_FILTER_WR_OVLAN_VLD_M: c_uint = 0x1;

pub const FW_FILTER_WR_IVLAN_VLDM_S: c_int = 3;
pub const FW_FILTER_WR_IVLAN_VLDM_M: c_uint = 0x1;

pub const FW_FILTER_WR_OVLAN_VLDM_S: c_int = 2;
pub const FW_FILTER_WR_OVLAN_VLDM_M: c_uint = 0x1;

pub const FW_FILTER_WR_RX_CHAN_S: c_int = 15;
pub const FW_FILTER_WR_RX_CHAN_M: c_uint = 0x1;

pub const FW_FILTER_WR_RX_RPL_IQ_S: c_int = 0;
pub const FW_FILTER_WR_RX_RPL_IQ_M: c_uint = 0x3ff;

pub const FW_FILTER2_WR_FILTER_TYPE_S: c_int = 1;
pub const FW_FILTER2_WR_FILTER_TYPE_M: c_uint = 0x1;

pub const FW_FILTER2_WR_NATMODE_S: c_int = 5;
pub const FW_FILTER2_WR_NATMODE_M: c_uint = 0x7;

pub const FW_FILTER2_WR_NATFLAGCHECK_S: c_int = 4;
pub const FW_FILTER2_WR_NATFLAGCHECK_M: c_uint = 0x1;

pub const FW_FILTER2_WR_ULP_TYPE_S: c_int = 0;
pub const FW_FILTER2_WR_ULP_TYPE_M: c_uint = 0xf;

pub const FW_FILTER_WR_MACI_S: c_int = 23;
pub const FW_FILTER_WR_MACI_M: c_uint = 0x1ff;

pub const FW_FILTER_WR_MACIM_S: c_int = 14;
pub const FW_FILTER_WR_MACIM_M: c_uint = 0x1ff;

pub const FW_FILTER_WR_FCOE_S: c_int = 13;
pub const FW_FILTER_WR_FCOE_M: c_uint = 0x1;

pub const FW_FILTER_WR_FCOEM_S: c_int = 12;
pub const FW_FILTER_WR_FCOEM_M: c_uint = 0x1;

pub const FW_FILTER_WR_PORT_S: c_int = 9;
pub const FW_FILTER_WR_PORT_M: c_uint = 0x7;

pub const FW_FILTER_WR_PORTM_S: c_int = 6;
pub const FW_FILTER_WR_PORTM_M: c_uint = 0x7;

pub const FW_FILTER_WR_MATCHTYPE_S: c_int = 3;
pub const FW_FILTER_WR_MATCHTYPE_M: c_uint = 0x7;

pub const FW_FILTER_WR_MATCHTYPEM_S: c_int = 0;
pub const FW_FILTER_WR_MATCHTYPEM_M: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ulptx_wr {
    pub op_to_compl: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
}

pub const FW_ULPTX_WR_DATA_S: c_int = 28;
pub const FW_ULPTX_WR_DATA_M: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_tp_wr {
    pub op_to_immdlen: __be32,
    pub flowid_len16: __be32,
    pub cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eth_tx_pkt_wr {
    pub op_immdlen: __be32,
    pub equiq_to_len16: __be32,
    pub r3: __be64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_eth_tx_eo_type {
    FW_ETH_TX_EO_TYPE_UDPSEG = 0,
    FW_ETH_TX_EO_TYPE_TCPSEG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eth_tx_eo_wr {
    pub op_immdlen: __be32,
    pub equiq_to_len16: __be32,
    pub r3: __be64,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_eth_tx_eo {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eth_tx_eo_udpseg {
    pub type: __u8,
    pub ethlen: __u8,
    pub iplen: __be16,
    pub udplen: __u8,
    pub rtplen: __u8,
    pub r4: __be16,
    pub mss: __be16,
    pub schedpktsize: __be16,
    pub plen: __be32,
    pub udpseg: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eth_tx_eo_tcpseg {
    pub type: __u8,
    pub ethlen: __u8,
    pub iplen: __be16,
    pub tcplen: __u8,
    pub tsclk_tsoff: __u8,
    pub r4: __be16,
    pub mss: __be16,
    pub r5: __be16,
    pub plen: __be32,
    pub tcpseg: },
    pub u: },
}

pub const FW_ETH_TX_EO_WR_IMMDLEN_S: c_int = 0;
pub const FW_ETH_TX_EO_WR_IMMDLEN_M: c_uint = 0x1ff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ofld_connection_wr {
    pub op_compl: __be32,
    pub len16_pkd: __be32,
    pub cookie: __u64,
    pub r2: __be64,
    pub r3: __be64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ofld_connection_le {
    pub version_cpl: __be32,
    pub filter: __be32,
    pub r1: __be32,
    pub lport: __be16,
    pub pport: __be16,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ofld_connection_leip {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ofld_connection_le_ipv4 {
    pub pip: __be32,
    pub lip: __be32,
    pub r0: __be64,
    pub r1: __be64,
    pub r2: __be64,
    pub ipv4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ofld_connection_le_ipv6 {
    pub pip_hi: __be64,
    pub pip_lo: __be64,
    pub lip_hi: __be64,
    pub lip_lo: __be64,
    pub ipv6: },
    pub u: },
    pub le: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ofld_connection_tcb {
    pub t_state_to_astid: __be32,
    pub cplrxdataack_cplpassacceptrpl: __be16,
    pub rcv_adv: __be16,
    pub rcv_nxt: __be32,
    pub tx_max: __be32,
    pub opt0: __be64,
    pub opt2: __be32,
    pub r1: __be32,
    pub r2: __be64,
    pub r3: __be64,
    pub tcb: },
}

pub const FW_OFLD_CONNECTION_WR_VERSION_S: c_int = 31;
pub const FW_OFLD_CONNECTION_WR_VERSION_M: c_uint = 0x1;

pub const FW_OFLD_CONNECTION_WR_CPL_S: c_int = 30;
pub const FW_OFLD_CONNECTION_WR_CPL_M: c_uint = 0x1;

pub const FW_OFLD_CONNECTION_WR_T_STATE_S: c_int = 28;
pub const FW_OFLD_CONNECTION_WR_T_STATE_M: c_uint = 0xf;

pub const FW_OFLD_CONNECTION_WR_RCV_SCALE_S: c_int = 24;
pub const FW_OFLD_CONNECTION_WR_RCV_SCALE_M: c_uint = 0xf;

pub const FW_OFLD_CONNECTION_WR_ASTID_S: c_int = 0;
pub const FW_OFLD_CONNECTION_WR_ASTID_M: c_uint = 0xffffff;

pub const FW_OFLD_CONNECTION_WR_CPLRXDATAACK_S: c_int = 15;
pub const FW_OFLD_CONNECTION_WR_CPLRXDATAACK_M: c_uint = 0x1;

pub const FW_OFLD_CONNECTION_WR_CPLPASSACCEPTRPL_S: c_int = 14;
pub const FW_OFLD_CONNECTION_WR_CPLPASSACCEPTRPL_M: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_flowc_mnem_tcpstate {
    FW_FLOWC_MNEM_TCPSTATE_CLOSED   = 0, /* illegal */
    FW_FLOWC_MNEM_TCPSTATE_LISTEN   = 1, /* illegal */
    FW_FLOWC_MNEM_TCPSTATE_SYNSENT  = 2, /* illegal */
    FW_FLOWC_MNEM_TCPSTATE_SYNRECEIVED = 3, /* illegal */
    FW_FLOWC_MNEM_TCPSTATE_ESTABLISHED = 4, /* default */
    FW_FLOWC_MNEM_TCPSTATE_CLOSEWAIT = 5, /* got peer close already */
    FW_FLOWC_MNEM_TCPSTATE_FINWAIT1 = 6, /* haven't gotten ACK for FIN and
// will resend FIN - equiv ESTAB
//
    FW_FLOWC_MNEM_TCPSTATE_CLOSING  = 7, /* haven't gotten ACK for FIN and
// will resend FIN but have
// received FIN
//
    FW_FLOWC_MNEM_TCPSTATE_LASTACK  = 8, /* haven't gotten ACK for FIN and
// will resend FIN but have
// received FIN
//
    FW_FLOWC_MNEM_TCPSTATE_FINWAIT2 = 9, /* sent FIN and got FIN + ACK,
// waiting for FIN
//
    FW_FLOWC_MNEM_TCPSTATE_TIMEWAIT = 10, /* not expected */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_flowc_mnem_eostate {
    FW_FLOWC_MNEM_EOSTATE_ESTABLISHED = 1, /* default */
// graceful close, after sending outstanding payload
    FW_FLOWC_MNEM_EOSTATE_CLOSING = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_flowc_mnem {
    FW_FLOWC_MNEM_PFNVFN,		/* PFN [15:8] VFN [7:0] */
    FW_FLOWC_MNEM_CH,
    FW_FLOWC_MNEM_PORT,
    FW_FLOWC_MNEM_IQID,
    FW_FLOWC_MNEM_SNDNXT,
    FW_FLOWC_MNEM_RCVNXT,
    FW_FLOWC_MNEM_SNDBUF,
    FW_FLOWC_MNEM_MSS,
    FW_FLOWC_MNEM_TXDATAPLEN_MAX,
    FW_FLOWC_MNEM_TCPSTATE,
    FW_FLOWC_MNEM_EOSTATE,
    FW_FLOWC_MNEM_SCHEDCLASS,
    FW_FLOWC_MNEM_DCBPRIO,
    FW_FLOWC_MNEM_SND_SCALE,
    FW_FLOWC_MNEM_RCV_SCALE,
    FW_FLOWC_MNEM_ULD_MODE,
    FW_FLOWC_MNEM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flowc_mnemval {
    pub mnemonic: u8,
    pub r4: [u8; 3],
    pub val: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flowc_wr {
    pub op_to_nparams: __be32,
    pub flowid_len16: __be32,
    pub mnemval: [fw_flowc_mnemval; ],
}

pub const FW_FLOWC_WR_NPARAMS_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ofld_tx_data_wr {
    pub op_to_immdlen: __be32,
    pub flowid_len16: __be32,
    pub plen: __be32,
    pub tunnel_to_proxy: __be32,
}

pub const FW_OFLD_TX_DATA_WR_ALIGNPLD_S: c_int = 30;

pub const FW_OFLD_TX_DATA_WR_SHOVE_S: c_int = 29;

pub const FW_OFLD_TX_DATA_WR_TUNNEL_S: c_int = 19;

pub const FW_OFLD_TX_DATA_WR_SAVE_S: c_int = 18;

pub const FW_OFLD_TX_DATA_WR_FLUSH_S: c_int = 17;

pub const FW_OFLD_TX_DATA_WR_URGENT_S: c_int = 16;

pub const FW_OFLD_TX_DATA_WR_MORE_S: c_int = 15;

pub const FW_OFLD_TX_DATA_WR_ULPMODE_S: c_int = 10;

pub const FW_OFLD_TX_DATA_WR_ULPSUBMODE_S: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_cmd_wr {
    pub op_dma: __be32,
    pub len16_pkd: __be32,
    pub cookie_daddr: __be64,
}

pub const FW_CMD_WR_DMA_S: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eth_tx_pkt_vm_wr {
    pub op_immdlen: __be32,
    pub equiq_to_len16: __be32,
    pub r3: [__be32; 2],
    pub ethmacdst: [u8; ETH_ALEN],
    pub ethmacsrc: [u8; ETH_ALEN],
    pub ethtype: __be16,
    pub vlantci: __be16,
}

pub const FW_CMD_MAX_TIMEOUT: c_int = 10000;
//
// If a host driver does a HELLO and discovers that there's already a MASTER
// selected, we may have to wait for that MASTER to finish issuing RESET,
// configuration and INITIALIZE commands.  Also, there's a possibility that
// our own HELLO may get lost if it happens right as the MASTER is issuign a
// RESET command, so we need to be willing to make a few retries of our HELLO.
//

pub const FW_CMD_HELLO_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_cmd_opcodes {
    FW_LDST_CMD                    = 0x01,
    FW_RESET_CMD                   = 0x03,
    FW_HELLO_CMD                   = 0x04,
    FW_BYE_CMD                     = 0x05,
    FW_INITIALIZE_CMD              = 0x06,
    FW_CAPS_CONFIG_CMD             = 0x07,
    FW_PARAMS_CMD                  = 0x08,
    FW_PFVF_CMD                    = 0x09,
    FW_IQ_CMD                      = 0x10,
    FW_EQ_MNGT_CMD                 = 0x11,
    FW_EQ_ETH_CMD                  = 0x12,
    FW_EQ_CTRL_CMD                 = 0x13,
    FW_EQ_OFLD_CMD                 = 0x21,
    FW_VI_CMD                      = 0x14,
    FW_VI_MAC_CMD                  = 0x15,
    FW_VI_RXMODE_CMD               = 0x16,
    FW_VI_ENABLE_CMD               = 0x17,
    FW_ACL_MAC_CMD                 = 0x18,
    FW_ACL_VLAN_CMD                = 0x19,
    FW_VI_STATS_CMD                = 0x1a,
    FW_PORT_CMD                    = 0x1b,
    FW_PORT_STATS_CMD              = 0x1c,
    FW_PORT_LB_STATS_CMD           = 0x1d,
    FW_PORT_TRACE_CMD              = 0x1e,
    FW_PORT_TRACE_MMAP_CMD         = 0x1f,
    FW_RSS_IND_TBL_CMD             = 0x20,
    FW_RSS_GLB_CONFIG_CMD          = 0x22,
    FW_RSS_VI_CONFIG_CMD           = 0x23,
    FW_SCHED_CMD                   = 0x24,
    FW_DEVLOG_CMD                  = 0x25,
    FW_CLIP_CMD                    = 0x28,
    FW_PTP_CMD                     = 0x3e,
    FW_HMA_CMD                     = 0x3f,
    FW_LASTC2E_CMD                 = 0x40,
    FW_ERROR_CMD                   = 0x80,
    FW_DEBUG_CMD                   = 0x81,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_cmd_cap {
    FW_CMD_CAP_PF                  = 0x01,
    FW_CMD_CAP_DMAQ                = 0x02,
    FW_CMD_CAP_PORT                = 0x04,
    FW_CMD_CAP_PORTPROMISC         = 0x08,
    FW_CMD_CAP_PORTSTATS           = 0x10,
    FW_CMD_CAP_VF                  = 0x80,
}

//
// Generic command header flit0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_cmd_hdr {
    pub hi: __be32,
    pub lo: __be32,
}

pub const FW_CMD_OP_S: c_int = 24;
pub const FW_CMD_OP_M: c_uint = 0xff;

pub const FW_CMD_REQUEST_S: c_int = 23;

pub const FW_CMD_READ_S: c_int = 22;

pub const FW_CMD_WRITE_S: c_int = 21;

pub const FW_CMD_EXEC_S: c_int = 20;

pub const FW_CMD_RAMASK_S: c_int = 20;

pub const FW_CMD_RETVAL_S: c_int = 8;
pub const FW_CMD_RETVAL_M: c_uint = 0xff;

pub const FW_CMD_LEN16_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ldst_addrspc {
    FW_LDST_ADDRSPC_FIRMWARE  = 0x0001,
    FW_LDST_ADDRSPC_SGE_EGRC  = 0x0008,
    FW_LDST_ADDRSPC_SGE_INGC  = 0x0009,
    FW_LDST_ADDRSPC_SGE_FLMC  = 0x000a,
    FW_LDST_ADDRSPC_SGE_CONMC = 0x000b,
    FW_LDST_ADDRSPC_TP_PIO    = 0x0010,
    FW_LDST_ADDRSPC_TP_TM_PIO = 0x0011,
    FW_LDST_ADDRSPC_TP_MIB    = 0x0012,
    FW_LDST_ADDRSPC_MDIO      = 0x0018,
    FW_LDST_ADDRSPC_MPS       = 0x0020,
    FW_LDST_ADDRSPC_FUNC      = 0x0028,
    FW_LDST_ADDRSPC_FUNC_PCIE = 0x0029,
    FW_LDST_ADDRSPC_I2C       = 0x0038,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ldst_mps_fid {
    FW_LDST_MPS_ATRB,
    FW_LDST_MPS_RPLC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ldst_func_access_ctl {
    FW_LDST_FUNC_ACC_CTL_VIID,
    FW_LDST_FUNC_ACC_CTL_FID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ldst_func_mod_index {
    FW_LDST_FUNC_MPS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_cmd {
    pub op_to_addrspace: __be32,
    pub cycles_to_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ldst {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_addrval {
    pub addr: __be32,
    pub val: __be32,
    pub addrval: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_idctxt {
    pub physid: __be32,
    pub msg_ctxtflush: __be32,
    pub ctxt_data7: __be32,
    pub ctxt_data6: __be32,
    pub ctxt_data5: __be32,
    pub ctxt_data4: __be32,
    pub ctxt_data3: __be32,
    pub ctxt_data2: __be32,
    pub ctxt_data1: __be32,
    pub ctxt_data0: __be32,
    pub idctxt: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_mdio {
    pub paddr_mmd: __be16,
    pub raddr: __be16,
    pub vctl: __be16,
    pub rval: __be16,
    pub mdio: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_cim_rq {
    pub req_first64: [u8; 8],
    pub req_second64: [u8; 8],
    pub resp_first64: [u8; 8],
    pub resp_second64: [u8; 8],
    pub r3: [__be32; 2],
    pub cim_rq: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ldst_mps {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_mps_rplc {
    pub fid_idx: __be16,
    pub rplcpf_pkd: __be16,
    pub rplc255_224: __be32,
    pub rplc223_192: __be32,
    pub rplc191_160: __be32,
    pub rplc159_128: __be32,
    pub rplc127_96: __be32,
    pub rplc95_64: __be32,
    pub rplc63_32: __be32,
    pub rplc31_0: __be32,
    pub rplc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_mps_atrb {
    pub fid_mpsid: __be16,
    pub r2: [__be16; 3],
    pub r3: [__be32; 2],
    pub r4: __be32,
    pub atrb: __be32,
    pub vlan: [__be16; 16],
    pub atrb: },
    pub mps: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_func {
    pub access_ctl: u8,
    pub mod_index: u8,
    pub ctl_id: __be16,
    pub offset: __be32,
    pub data0: __be64,
    pub data1: __be64,
    pub func: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_pcie {
    pub ctrl_to_fn: u8,
    pub bnum: u8,
    pub r: u8,
    pub ext_r: u8,
    pub select_naccess: u8,
    pub pcie_fn: u8,
    pub nset_pkd: __be16,
    pub data: [__be32; 12],
    pub pcie: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_i2c_deprecated {
    pub pid_pkd: u8,
    pub base: u8,
    pub boffset: u8,
    pub data: u8,
    pub r9: __be32,
    pub i2c_deprecated: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_i2c {
    pub pid: u8,
    pub did: u8,
    pub boffset: u8,
    pub blen: u8,
    pub r9: __be32,
    pub data: [__u8; 48],
    pub i2c: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ldst_le {
    pub index: __be32,
    pub r9: __be32,
    pub val: [u8; 33],
    pub r11: [u8; 7],
    pub le: },
    pub u: },
}

pub const FW_LDST_CMD_ADDRSPACE_S: c_int = 0;

pub const FW_LDST_CMD_MSG_S: c_int = 31;

pub const FW_LDST_CMD_CTXTFLUSH_S: c_int = 30;

pub const FW_LDST_CMD_PADDR_S: c_int = 8;

pub const FW_LDST_CMD_MMD_S: c_int = 0;

pub const FW_LDST_CMD_FID_S: c_int = 15;

pub const FW_LDST_CMD_IDX_S: c_int = 0;

pub const FW_LDST_CMD_RPLCPF_S: c_int = 0;

pub const FW_LDST_CMD_LC_S: c_int = 4;

pub const FW_LDST_CMD_FN_S: c_int = 0;

pub const FW_LDST_CMD_NACCESS_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_reset_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
    pub val: __be32,
    pub halt_pkd: __be32,
}

pub const FW_RESET_CMD_HALT_S: c_int = 31;
pub const FW_RESET_CMD_HALT_M: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_hellow_cmd {
    fw_hello_cmd_stage_os		= 0x0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_hello_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
    pub err_to_clearinit: __be32,
    pub fwrev: __be32,
}

pub const FW_HELLO_CMD_ERR_S: c_int = 31;

pub const FW_HELLO_CMD_INIT_S: c_int = 30;

pub const FW_HELLO_CMD_MASTERDIS_S: c_int = 29;

pub const FW_HELLO_CMD_MASTERFORCE_S: c_int = 28;

pub const FW_HELLO_CMD_MBMASTER_S: c_int = 24;
pub const FW_HELLO_CMD_MBMASTER_M: c_uint = 0xfU;

pub const FW_HELLO_CMD_MBASYNCNOTINT_S: c_int = 23;

pub const FW_HELLO_CMD_MBASYNCNOT_S: c_int = 20;

pub const FW_HELLO_CMD_STAGE_S: c_int = 17;

pub const FW_HELLO_CMD_CLEARINIT_S: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_bye_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
    pub r3: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_initialize_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
    pub r3: __be64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_hm {
    FW_CAPS_CONFIG_HM_PCIE		= 0x00000001,
    FW_CAPS_CONFIG_HM_PL		= 0x00000002,
    FW_CAPS_CONFIG_HM_SGE		= 0x00000004,
    FW_CAPS_CONFIG_HM_CIM		= 0x00000008,
    FW_CAPS_CONFIG_HM_ULPTX		= 0x00000010,
    FW_CAPS_CONFIG_HM_TP		= 0x00000020,
    FW_CAPS_CONFIG_HM_ULPRX		= 0x00000040,
    FW_CAPS_CONFIG_HM_PMRX		= 0x00000080,
    FW_CAPS_CONFIG_HM_PMTX		= 0x00000100,
    FW_CAPS_CONFIG_HM_MC		= 0x00000200,
    FW_CAPS_CONFIG_HM_LE		= 0x00000400,
    FW_CAPS_CONFIG_HM_MPS		= 0x00000800,
    FW_CAPS_CONFIG_HM_XGMAC		= 0x00001000,
    FW_CAPS_CONFIG_HM_CPLSWITCH	= 0x00002000,
    FW_CAPS_CONFIG_HM_T4DBG		= 0x00004000,
    FW_CAPS_CONFIG_HM_MI		= 0x00008000,
    FW_CAPS_CONFIG_HM_I2CM		= 0x00010000,
    FW_CAPS_CONFIG_HM_NCSI		= 0x00020000,
    FW_CAPS_CONFIG_HM_SMB		= 0x00040000,
    FW_CAPS_CONFIG_HM_MA		= 0x00080000,
    FW_CAPS_CONFIG_HM_EDRAM		= 0x00100000,
    FW_CAPS_CONFIG_HM_PMU		= 0x00200000,
    FW_CAPS_CONFIG_HM_UART		= 0x00400000,
    FW_CAPS_CONFIG_HM_SF		= 0x00800000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_nbm {
    FW_CAPS_CONFIG_NBM_IPMI		= 0x00000001,
    FW_CAPS_CONFIG_NBM_NCSI		= 0x00000002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_link {
    FW_CAPS_CONFIG_LINK_PPP		= 0x00000001,
    FW_CAPS_CONFIG_LINK_QFC		= 0x00000002,
    FW_CAPS_CONFIG_LINK_DCBX	= 0x00000004,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_switch {
    FW_CAPS_CONFIG_SWITCH_INGRESS	= 0x00000001,
    FW_CAPS_CONFIG_SWITCH_EGRESS	= 0x00000002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_nic {
    FW_CAPS_CONFIG_NIC		= 0x00000001,
    FW_CAPS_CONFIG_NIC_VM		= 0x00000002,
    FW_CAPS_CONFIG_NIC_HASHFILTER	= 0x00000020,
    FW_CAPS_CONFIG_NIC_ETHOFLD	= 0x00000040,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_ofld {
    FW_CAPS_CONFIG_OFLD		= 0x00000001,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_rdma {
    FW_CAPS_CONFIG_RDMA_RDDP	= 0x00000001,
    FW_CAPS_CONFIG_RDMA_RDMAC	= 0x00000002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_iscsi {
    FW_CAPS_CONFIG_ISCSI_INITIATOR_PDU = 0x00000001,
    FW_CAPS_CONFIG_ISCSI_TARGET_PDU = 0x00000002,
    FW_CAPS_CONFIG_ISCSI_INITIATOR_CNXOFLD = 0x00000004,
    FW_CAPS_CONFIG_ISCSI_TARGET_CNXOFLD = 0x00000008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_crypto {
    FW_CAPS_CONFIG_CRYPTO_LOOKASIDE = 0x00000001,
    FW_CAPS_CONFIG_TLS_INLINE = 0x00000002,
    FW_CAPS_CONFIG_IPSEC_INLINE = 0x00000004,
    FW_CAPS_CONFIG_TLS_HW = 0x00000008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps_config_fcoe {
    FW_CAPS_CONFIG_FCOE_INITIATOR	= 0x00000001,
    FW_CAPS_CONFIG_FCOE_TARGET	= 0x00000002,
    FW_CAPS_CONFIG_FCOE_CTRL_OFLD	= 0x00000004,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_memtype_cf {
    FW_MEMTYPE_CF_EDC0		= 0x0,
    FW_MEMTYPE_CF_EDC1		= 0x1,
    FW_MEMTYPE_CF_EXTMEM		= 0x2,
    FW_MEMTYPE_CF_FLASH		= 0x4,
    FW_MEMTYPE_CF_INTERNAL		= 0x5,
    FW_MEMTYPE_CF_EXTMEM1           = 0x6,
    FW_MEMTYPE_CF_HMA		= 0x7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_caps_config_cmd {
    pub op_to_write: __be32,
    pub cfvalid_to_len16: __be32,
    pub r2: __be32,
    pub hwmbitmap: __be32,
    pub nbmcaps: __be16,
    pub linkcaps: __be16,
    pub switchcaps: __be16,
    pub r3: __be16,
    pub niccaps: __be16,
    pub ofldcaps: __be16,
    pub rdmacaps: __be16,
    pub cryptocaps: __be16,
    pub iscsicaps: __be16,
    pub fcoecaps: __be16,
    pub cfcsum: __be32,
    pub finiver: __be32,
    pub finicsum: __be32,
}

pub const FW_CAPS_CONFIG_CMD_CFVALID_S: c_int = 27;

pub const FW_CAPS_CONFIG_CMD_MEMTYPE_CF_S: c_int = 24;

pub const FW_CAPS_CONFIG_CMD_MEMADDR64K_CF_S: c_int = 16;

//
// params command mnemonics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_mnem {
    FW_PARAMS_MNEM_DEV		= 1,	/* device params */
    FW_PARAMS_MNEM_PFVF		= 2,	/* function params */
    FW_PARAMS_MNEM_REG		= 3,	/* limited register access */
    FW_PARAMS_MNEM_DMAQ		= 4,	/* dma queue params */
    FW_PARAMS_MNEM_CHNET            = 5,    /* chnet params */
    FW_PARAMS_MNEM_LAST
}

//
// device parameters
//
pub const FW_PARAMS_PARAM_FILTER_MODE_S: c_int = 16;
pub const FW_PARAMS_PARAM_FILTER_MODE_M: c_uint = 0xffff;

pub const FW_PARAMS_PARAM_FILTER_MASK_S: c_int = 0;
pub const FW_PARAMS_PARAM_FILTER_MASK_M: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dev {
    FW_PARAMS_PARAM_DEV_CCLK	= 0x00, /* chip core clock in khz */
    FW_PARAMS_PARAM_DEV_PORTVEC	= 0x01, /* the port vector */
    FW_PARAMS_PARAM_DEV_NTID	= 0x02, /* reads the number of TIDs
// allocated by the device's
// Lookup Engine
//
    FW_PARAMS_PARAM_DEV_FLOWC_BUFFIFO_SZ = 0x03,
    FW_PARAMS_PARAM_DEV_INTVER_NIC	= 0x04,
    FW_PARAMS_PARAM_DEV_INTVER_VNIC = 0x05,
    FW_PARAMS_PARAM_DEV_INTVER_OFLD = 0x06,
    FW_PARAMS_PARAM_DEV_INTVER_RI	= 0x07,
    FW_PARAMS_PARAM_DEV_INTVER_ISCSIPDU = 0x08,
    FW_PARAMS_PARAM_DEV_INTVER_ISCSI = 0x09,
    FW_PARAMS_PARAM_DEV_INTVER_FCOE = 0x0A,
    FW_PARAMS_PARAM_DEV_FWREV = 0x0B,
    FW_PARAMS_PARAM_DEV_TPREV = 0x0C,
    FW_PARAMS_PARAM_DEV_CF = 0x0D,
    FW_PARAMS_PARAM_DEV_PHYFW = 0x0F,
    FW_PARAMS_PARAM_DEV_DIAG = 0x11,
    FW_PARAMS_PARAM_DEV_MAXORDIRD_QP = 0x13, /* max supported QP IRD/ORD */
    FW_PARAMS_PARAM_DEV_MAXIRD_ADAPTER = 0x14, /* max supported adap IRD */
    FW_PARAMS_PARAM_DEV_ULPTX_MEMWRITE_DSGL = 0x17,
    FW_PARAMS_PARAM_DEV_FWCACHE = 0x18,
    FW_PARAMS_PARAM_DEV_SCFGREV = 0x1A,
    FW_PARAMS_PARAM_DEV_VPDREV = 0x1B,
    FW_PARAMS_PARAM_DEV_RI_FR_NSMR_TPTE_WR	= 0x1C,
    FW_PARAMS_PARAM_DEV_FILTER2_WR  = 0x1D,
    FW_PARAMS_PARAM_DEV_MPSBGMAP	= 0x1E,
    FW_PARAMS_PARAM_DEV_TPCHMAP     = 0x1F,
    FW_PARAMS_PARAM_DEV_HMA_SIZE	= 0x20,
    FW_PARAMS_PARAM_DEV_RDMA_WRITE_WITH_IMM = 0x21,
    FW_PARAMS_PARAM_DEV_PPOD_EDRAM  = 0x23,
    FW_PARAMS_PARAM_DEV_RI_WRITE_CMPL_WR    = 0x24,
    FW_PARAMS_PARAM_DEV_HPFILTER_REGION_SUPPORT = 0x26,
    FW_PARAMS_PARAM_DEV_OPAQUE_VIID_SMT_EXTN = 0x27,
    FW_PARAMS_PARAM_DEV_HASHFILTER_WITH_OFLD = 0x28,
    FW_PARAMS_PARAM_DEV_DBQ_TIMER	= 0x29,
    FW_PARAMS_PARAM_DEV_DBQ_TIMERTICK = 0x2A,
    FW_PARAMS_PARAM_DEV_NUM_TM_CLASS = 0x2B,
    FW_PARAMS_PARAM_DEV_FILTER = 0x2E,
    FW_PARAMS_PARAM_DEV_KTLS_HW = 0x31,
}

//
// physical and virtual function parameters
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_pfvf {
    FW_PARAMS_PARAM_PFVF_RWXCAPS	= 0x00,
    FW_PARAMS_PARAM_PFVF_ROUTE_START = 0x01,
    FW_PARAMS_PARAM_PFVF_ROUTE_END = 0x02,
    FW_PARAMS_PARAM_PFVF_CLIP_START = 0x03,
    FW_PARAMS_PARAM_PFVF_CLIP_END = 0x04,
    FW_PARAMS_PARAM_PFVF_FILTER_START = 0x05,
    FW_PARAMS_PARAM_PFVF_FILTER_END = 0x06,
    FW_PARAMS_PARAM_PFVF_SERVER_START = 0x07,
    FW_PARAMS_PARAM_PFVF_SERVER_END = 0x08,
    FW_PARAMS_PARAM_PFVF_TDDP_START = 0x09,
    FW_PARAMS_PARAM_PFVF_TDDP_END = 0x0A,
    FW_PARAMS_PARAM_PFVF_ISCSI_START = 0x0B,
    FW_PARAMS_PARAM_PFVF_ISCSI_END = 0x0C,
    FW_PARAMS_PARAM_PFVF_STAG_START = 0x0D,
    FW_PARAMS_PARAM_PFVF_STAG_END = 0x0E,
    FW_PARAMS_PARAM_PFVF_RQ_START = 0x1F,
    FW_PARAMS_PARAM_PFVF_RQ_END	= 0x10,
    FW_PARAMS_PARAM_PFVF_PBL_START = 0x11,
    FW_PARAMS_PARAM_PFVF_PBL_END	= 0x12,
    FW_PARAMS_PARAM_PFVF_L2T_START = 0x13,
    FW_PARAMS_PARAM_PFVF_L2T_END = 0x14,
    FW_PARAMS_PARAM_PFVF_SQRQ_START = 0x15,
    FW_PARAMS_PARAM_PFVF_SQRQ_END	= 0x16,
    FW_PARAMS_PARAM_PFVF_CQ_START	= 0x17,
    FW_PARAMS_PARAM_PFVF_CQ_END	= 0x18,
    FW_PARAMS_PARAM_PFVF_SRQ_START  = 0x19,
    FW_PARAMS_PARAM_PFVF_SRQ_END    = 0x1A,
    FW_PARAMS_PARAM_PFVF_SCHEDCLASS_ETH = 0x20,
    FW_PARAMS_PARAM_PFVF_VIID       = 0x24,
    FW_PARAMS_PARAM_PFVF_CPMASK     = 0x25,
    FW_PARAMS_PARAM_PFVF_OCQ_START  = 0x26,
    FW_PARAMS_PARAM_PFVF_OCQ_END    = 0x27,
    FW_PARAMS_PARAM_PFVF_CONM_MAP   = 0x28,
    FW_PARAMS_PARAM_PFVF_IQFLINT_START = 0x29,
    FW_PARAMS_PARAM_PFVF_IQFLINT_END = 0x2A,
    FW_PARAMS_PARAM_PFVF_EQ_START	= 0x2B,
    FW_PARAMS_PARAM_PFVF_EQ_END	= 0x2C,
    FW_PARAMS_PARAM_PFVF_ACTIVE_FILTER_START = 0x2D,
    FW_PARAMS_PARAM_PFVF_ACTIVE_FILTER_END = 0x2E,
    FW_PARAMS_PARAM_PFVF_ETHOFLD_START = 0x2F,
    FW_PARAMS_PARAM_PFVF_ETHOFLD_END = 0x30,
    FW_PARAMS_PARAM_PFVF_CPLFW4MSG_ENCAP = 0x31,
    FW_PARAMS_PARAM_PFVF_HPFILTER_START = 0x32,
    FW_PARAMS_PARAM_PFVF_HPFILTER_END = 0x33,
    FW_PARAMS_PARAM_PFVF_TLS_START = 0x34,
    FW_PARAMS_PARAM_PFVF_TLS_END = 0x35,
    FW_PARAMS_PARAM_PFVF_RAWF_START = 0x36,
    FW_PARAMS_PARAM_PFVF_RAWF_END = 0x37,
    FW_PARAMS_PARAM_PFVF_NCRYPTO_LOOKASIDE = 0x39,
    FW_PARAMS_PARAM_PFVF_PORT_CAPS32 = 0x3A,
    FW_PARAMS_PARAM_PFVF_PPOD_EDRAM_START = 0x3B,
    FW_PARAMS_PARAM_PFVF_PPOD_EDRAM_END = 0x3C,
    FW_PARAMS_PARAM_PFVF_LINK_STATE = 0x40,
}

// Virtual link state as seen by the specified VF
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf_link_states {
    FW_VF_LINK_STATE_AUTO		= 0x00,
    FW_VF_LINK_STATE_ENABLE		= 0x01,
    FW_VF_LINK_STATE_DISABLE	= 0x02,
}

//
// dma queue parameters
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dmaq {
    FW_PARAMS_PARAM_DMAQ_IQ_DCAEN_DCACPU = 0x00,
    FW_PARAMS_PARAM_DMAQ_IQ_INTCNTTHRESH = 0x01,
    FW_PARAMS_PARAM_DMAQ_EQ_CMPLIQID_MNGT = 0x10,
    FW_PARAMS_PARAM_DMAQ_EQ_CMPLIQID_CTRL = 0x11,
    FW_PARAMS_PARAM_DMAQ_EQ_SCHEDCLASS_ETH = 0x12,
    FW_PARAMS_PARAM_DMAQ_EQ_DCBPRIO_ETH = 0x13,
    FW_PARAMS_PARAM_DMAQ_EQ_TIMERIX	= 0x15,
    FW_PARAMS_PARAM_DMAQ_CONM_CTXT = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dev_ktls_hw {
    FW_PARAMS_PARAM_DEV_KTLS_HW_DISABLE      = 0x00,
    FW_PARAMS_PARAM_DEV_KTLS_HW_ENABLE       = 0x01,
    FW_PARAMS_PARAM_DEV_KTLS_HW_USER_ENABLE  = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dev_phyfw {
    FW_PARAMS_PARAM_DEV_PHYFW_DOWNLOAD = 0x00,
    FW_PARAMS_PARAM_DEV_PHYFW_VERSION = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dev_diag {
    FW_PARAM_DEV_DIAG_TMP		= 0x00,
    FW_PARAM_DEV_DIAG_VDD		= 0x01,
    FW_PARAM_DEV_DIAG_MAXTMPTHRESH	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dev_filter {
    FW_PARAM_DEV_FILTER_VNIC_MODE   = 0x00,
    FW_PARAM_DEV_FILTER_MODE_MASK   = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_params_param_dev_fwcache {
    FW_PARAM_DEV_FWCACHE_FLUSH      = 0x00,
    FW_PARAM_DEV_FWCACHE_FLUSHINV   = 0x01,
}

pub const FW_PARAMS_MNEM_S: c_int = 24;

pub const FW_PARAMS_PARAM_X_S: c_int = 16;

pub const FW_PARAMS_PARAM_Y_S: c_int = 8;
pub const FW_PARAMS_PARAM_Y_M: c_uint = 0xffU;

pub const FW_PARAMS_PARAM_Z_S: c_int = 0;
pub const FW_PARAMS_PARAM_Z_M: c_uint = 0xffu;

pub const FW_PARAMS_PARAM_XYZ_S: c_int = 0;

pub const FW_PARAMS_PARAM_YZ_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_params_cmd {
    pub op_to_vfn: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_params_param {
    pub mnem: __be32,
    pub val: __be32,
    pub param: [}; 7],
}

pub const FW_PARAMS_CMD_PFN_S: c_int = 8;

pub const FW_PARAMS_CMD_VFN_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_pfvf_cmd {
    pub op_to_vfn: __be32,
    pub retval_len16: __be32,
    pub niqflint_niq: __be32,
    pub type_to_neq: __be32,
    pub tc_to_nexactf: __be32,
    pub r_caps_to_nethctrl: __be32,
    pub nricq: __be16,
    pub nriqp: __be16,
    pub r4: __be32,
}

pub const FW_PFVF_CMD_PFN_S: c_int = 8;

pub const FW_PFVF_CMD_VFN_S: c_int = 0;

pub const FW_PFVF_CMD_NIQFLINT_S: c_int = 20;
pub const FW_PFVF_CMD_NIQFLINT_M: c_uint = 0xfff;

pub const FW_PFVF_CMD_NIQ_S: c_int = 0;
pub const FW_PFVF_CMD_NIQ_M: c_uint = 0xfffff;

pub const FW_PFVF_CMD_TYPE_S: c_int = 31;
pub const FW_PFVF_CMD_TYPE_M: c_uint = 0x1;

pub const FW_PFVF_CMD_CMASK_S: c_int = 24;
pub const FW_PFVF_CMD_CMASK_M: c_uint = 0xf;

pub const FW_PFVF_CMD_PMASK_S: c_int = 20;
pub const FW_PFVF_CMD_PMASK_M: c_uint = 0xf;

pub const FW_PFVF_CMD_NEQ_S: c_int = 0;
pub const FW_PFVF_CMD_NEQ_M: c_uint = 0xfffff;

pub const FW_PFVF_CMD_TC_S: c_int = 24;
pub const FW_PFVF_CMD_TC_M: c_uint = 0xff;

pub const FW_PFVF_CMD_NVI_S: c_int = 16;
pub const FW_PFVF_CMD_NVI_M: c_uint = 0xff;

pub const FW_PFVF_CMD_NEXACTF_S: c_int = 0;
pub const FW_PFVF_CMD_NEXACTF_M: c_uint = 0xffff;

pub const FW_PFVF_CMD_R_CAPS_S: c_int = 24;
pub const FW_PFVF_CMD_R_CAPS_M: c_uint = 0xff;

pub const FW_PFVF_CMD_WX_CAPS_S: c_int = 16;
pub const FW_PFVF_CMD_WX_CAPS_M: c_uint = 0xff;

pub const FW_PFVF_CMD_NETHCTRL_S: c_int = 0;
pub const FW_PFVF_CMD_NETHCTRL_M: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_iq_type {
    FW_IQ_TYPE_FL_INT_CAP,
    FW_IQ_TYPE_NO_FL_INT_CAP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_iq_iqtype {
    FW_IQ_IQTYPE_OTHER,
    FW_IQ_IQTYPE_NIC,
    FW_IQ_IQTYPE_OFLD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_iq_cmd {
    pub op_to_vfn: __be32,
    pub alloc_to_len16: __be32,
    pub physiqid: __be16,
    pub iqid: __be16,
    pub fl0id: __be16,
    pub fl1id: __be16,
    pub type_to_iqandstindex: __be32,
    pub iqdroprss_to_iqesize: __be16,
    pub iqsize: __be16,
    pub iqaddr: __be64,
    pub iqns_to_fl0congen: __be32,
    pub fl0dcaen_to_fl0cidxfthresh: __be16,
    pub fl0size: __be16,
    pub fl0addr: __be64,
    pub fl1cngchmap_to_fl1congen: __be32,
    pub fl1dcaen_to_fl1cidxfthresh: __be16,
    pub fl1size: __be16,
    pub fl1addr: __be64,
}

pub const FW_IQ_CMD_PFN_S: c_int = 8;

pub const FW_IQ_CMD_VFN_S: c_int = 0;

pub const FW_IQ_CMD_ALLOC_S: c_int = 31;

pub const FW_IQ_CMD_FREE_S: c_int = 30;

pub const FW_IQ_CMD_MODIFY_S: c_int = 29;

pub const FW_IQ_CMD_IQSTART_S: c_int = 28;

pub const FW_IQ_CMD_IQSTOP_S: c_int = 27;

pub const FW_IQ_CMD_TYPE_S: c_int = 29;

pub const FW_IQ_CMD_IQASYNCH_S: c_int = 28;

pub const FW_IQ_CMD_VIID_S: c_int = 16;

pub const FW_IQ_CMD_IQANDST_S: c_int = 15;

pub const FW_IQ_CMD_IQANUS_S: c_int = 14;

pub const FW_IQ_CMD_IQANUD_S: c_int = 12;

pub const FW_IQ_CMD_IQANDSTINDEX_S: c_int = 0;

pub const FW_IQ_CMD_IQDROPRSS_S: c_int = 15;

pub const FW_IQ_CMD_IQGTSMODE_S: c_int = 14;

pub const FW_IQ_CMD_IQPCIECH_S: c_int = 12;

pub const FW_IQ_CMD_IQDCAEN_S: c_int = 11;

pub const FW_IQ_CMD_IQDCACPU_S: c_int = 6;

pub const FW_IQ_CMD_IQINTCNTTHRESH_S: c_int = 4;

pub const FW_IQ_CMD_IQO_S: c_int = 3;

pub const FW_IQ_CMD_IQCPRIO_S: c_int = 2;

pub const FW_IQ_CMD_IQESIZE_S: c_int = 0;

pub const FW_IQ_CMD_IQNS_S: c_int = 31;

pub const FW_IQ_CMD_IQRO_S: c_int = 30;

pub const FW_IQ_CMD_IQFLINTIQHSEN_S: c_int = 28;

pub const FW_IQ_CMD_IQFLINTCONGEN_S: c_int = 27;

pub const FW_IQ_CMD_IQFLINTISCSIC_S: c_int = 26;

pub const FW_IQ_CMD_IQTYPE_S: c_int = 24;
pub const FW_IQ_CMD_IQTYPE_M: c_uint = 0x3;

pub const FW_IQ_CMD_FL0CNGCHMAP_S: c_int = 20;

pub const FW_IQ_CMD_FL0CACHELOCK_S: c_int = 15;

pub const FW_IQ_CMD_FL0DBP_S: c_int = 14;

pub const FW_IQ_CMD_FL0DATANS_S: c_int = 13;

pub const FW_IQ_CMD_FL0DATARO_S: c_int = 12;

pub const FW_IQ_CMD_FL0CONGCIF_S: c_int = 11;

pub const FW_IQ_CMD_FL0ONCHIP_S: c_int = 10;

pub const FW_IQ_CMD_FL0STATUSPGNS_S: c_int = 9;

pub const FW_IQ_CMD_FL0STATUSPGRO_S: c_int = 8;

pub const FW_IQ_CMD_FL0FETCHNS_S: c_int = 7;

pub const FW_IQ_CMD_FL0FETCHRO_S: c_int = 6;

pub const FW_IQ_CMD_FL0HOSTFCMODE_S: c_int = 4;

pub const FW_IQ_CMD_FL0CPRIO_S: c_int = 3;

pub const FW_IQ_CMD_FL0PADEN_S: c_int = 2;

pub const FW_IQ_CMD_FL0PACKEN_S: c_int = 1;

pub const FW_IQ_CMD_FL0CONGEN_S: c_int = 0;

pub const FW_IQ_CMD_FL0DCAEN_S: c_int = 15;

pub const FW_IQ_CMD_FL0DCACPU_S: c_int = 10;

pub const FW_IQ_CMD_FL0FBMIN_S: c_int = 7;

pub const FW_IQ_CMD_FL0FBMAX_S: c_int = 4;

pub const FW_IQ_CMD_FL0CIDXFTHRESHO_S: c_int = 3;

pub const FW_IQ_CMD_FL0CIDXFTHRESH_S: c_int = 0;

pub const FW_IQ_CMD_FL1CNGCHMAP_S: c_int = 20;

pub const FW_IQ_CMD_FL1CACHELOCK_S: c_int = 15;

pub const FW_IQ_CMD_FL1DBP_S: c_int = 14;

pub const FW_IQ_CMD_FL1DATANS_S: c_int = 13;

pub const FW_IQ_CMD_FL1DATARO_S: c_int = 12;

pub const FW_IQ_CMD_FL1CONGCIF_S: c_int = 11;

pub const FW_IQ_CMD_FL1ONCHIP_S: c_int = 10;

pub const FW_IQ_CMD_FL1STATUSPGNS_S: c_int = 9;

pub const FW_IQ_CMD_FL1STATUSPGRO_S: c_int = 8;

pub const FW_IQ_CMD_FL1FETCHNS_S: c_int = 7;

pub const FW_IQ_CMD_FL1FETCHRO_S: c_int = 6;

pub const FW_IQ_CMD_FL1HOSTFCMODE_S: c_int = 4;

pub const FW_IQ_CMD_FL1CPRIO_S: c_int = 3;

pub const FW_IQ_CMD_FL1PADEN_S: c_int = 2;

pub const FW_IQ_CMD_FL1PACKEN_S: c_int = 1;

pub const FW_IQ_CMD_FL1CONGEN_S: c_int = 0;

pub const FW_IQ_CMD_FL1DCAEN_S: c_int = 15;

pub const FW_IQ_CMD_FL1DCACPU_S: c_int = 10;

pub const FW_IQ_CMD_FL1FBMIN_S: c_int = 7;

pub const FW_IQ_CMD_FL1FBMAX_S: c_int = 4;

pub const FW_IQ_CMD_FL1CIDXFTHRESHO_S: c_int = 3;

pub const FW_IQ_CMD_FL1CIDXFTHRESH_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eq_eth_cmd {
    pub op_to_vfn: __be32,
    pub alloc_to_len16: __be32,
    pub eqid_pkd: __be32,
    pub physeqid_pkd: __be32,
    pub fetchszm_to_iqid: __be32,
    pub dcaen_to_eqsize: __be32,
    pub eqaddr: __be64,
    pub autoequiqe_to_viid: __be32,
    pub timeren_timerix: __be32,
    pub r9: __be64,
}

pub const FW_EQ_ETH_CMD_PFN_S: c_int = 8;

pub const FW_EQ_ETH_CMD_VFN_S: c_int = 0;

pub const FW_EQ_ETH_CMD_ALLOC_S: c_int = 31;

pub const FW_EQ_ETH_CMD_FREE_S: c_int = 30;

pub const FW_EQ_ETH_CMD_MODIFY_S: c_int = 29;

pub const FW_EQ_ETH_CMD_EQSTART_S: c_int = 28;

pub const FW_EQ_ETH_CMD_EQSTOP_S: c_int = 27;

pub const FW_EQ_ETH_CMD_EQID_S: c_int = 0;
pub const FW_EQ_ETH_CMD_EQID_M: c_uint = 0xfffff;

pub const FW_EQ_ETH_CMD_PHYSEQID_S: c_int = 0;
pub const FW_EQ_ETH_CMD_PHYSEQID_M: c_uint = 0xfffff;

pub const FW_EQ_ETH_CMD_FETCHSZM_S: c_int = 26;

pub const FW_EQ_ETH_CMD_STATUSPGNS_S: c_int = 25;

pub const FW_EQ_ETH_CMD_STATUSPGRO_S: c_int = 24;

pub const FW_EQ_ETH_CMD_FETCHNS_S: c_int = 23;

pub const FW_EQ_ETH_CMD_FETCHRO_S: c_int = 22;

pub const FW_EQ_ETH_CMD_HOSTFCMODE_S: c_int = 20;

pub const FW_EQ_ETH_CMD_CPRIO_S: c_int = 19;

pub const FW_EQ_ETH_CMD_ONCHIP_S: c_int = 18;

pub const FW_EQ_ETH_CMD_PCIECHN_S: c_int = 16;

pub const FW_EQ_ETH_CMD_IQID_S: c_int = 0;

pub const FW_EQ_ETH_CMD_DCAEN_S: c_int = 31;

pub const FW_EQ_ETH_CMD_DCACPU_S: c_int = 26;

pub const FW_EQ_ETH_CMD_FBMIN_S: c_int = 23;

pub const FW_EQ_ETH_CMD_FBMAX_S: c_int = 20;

pub const FW_EQ_ETH_CMD_CIDXFTHRESHO_S: c_int = 19;

pub const FW_EQ_ETH_CMD_CIDXFTHRESH_S: c_int = 16;

pub const FW_EQ_ETH_CMD_EQSIZE_S: c_int = 0;

pub const FW_EQ_ETH_CMD_AUTOEQUIQE_S: c_int = 31;

pub const FW_EQ_ETH_CMD_AUTOEQUEQE_S: c_int = 30;

pub const FW_EQ_ETH_CMD_VIID_S: c_int = 16;

pub const FW_EQ_ETH_CMD_TIMEREN_S: c_int = 3;
pub const FW_EQ_ETH_CMD_TIMEREN_M: c_uint = 0x1;

pub const FW_EQ_ETH_CMD_TIMERIX_S: c_int = 0;
pub const FW_EQ_ETH_CMD_TIMERIX_M: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eq_ctrl_cmd {
    pub op_to_vfn: __be32,
    pub alloc_to_len16: __be32,
    pub cmpliqid_eqid: __be32,
    pub physeqid_pkd: __be32,
    pub fetchszm_to_iqid: __be32,
    pub dcaen_to_eqsize: __be32,
    pub eqaddr: __be64,
}

pub const FW_EQ_CTRL_CMD_PFN_S: c_int = 8;

pub const FW_EQ_CTRL_CMD_VFN_S: c_int = 0;

pub const FW_EQ_CTRL_CMD_ALLOC_S: c_int = 31;

pub const FW_EQ_CTRL_CMD_FREE_S: c_int = 30;

pub const FW_EQ_CTRL_CMD_MODIFY_S: c_int = 29;

pub const FW_EQ_CTRL_CMD_EQSTART_S: c_int = 28;

pub const FW_EQ_CTRL_CMD_EQSTOP_S: c_int = 27;

pub const FW_EQ_CTRL_CMD_CMPLIQID_S: c_int = 20;

pub const FW_EQ_CTRL_CMD_EQID_S: c_int = 0;
pub const FW_EQ_CTRL_CMD_EQID_M: c_uint = 0xfffff;

pub const FW_EQ_CTRL_CMD_PHYSEQID_S: c_int = 0;
pub const FW_EQ_CTRL_CMD_PHYSEQID_M: c_uint = 0xfffff;

pub const FW_EQ_CTRL_CMD_FETCHSZM_S: c_int = 26;

pub const FW_EQ_CTRL_CMD_STATUSPGNS_S: c_int = 25;

pub const FW_EQ_CTRL_CMD_STATUSPGRO_S: c_int = 24;

pub const FW_EQ_CTRL_CMD_FETCHNS_S: c_int = 23;

pub const FW_EQ_CTRL_CMD_FETCHRO_S: c_int = 22;

pub const FW_EQ_CTRL_CMD_HOSTFCMODE_S: c_int = 20;

pub const FW_EQ_CTRL_CMD_CPRIO_S: c_int = 19;

pub const FW_EQ_CTRL_CMD_ONCHIP_S: c_int = 18;

pub const FW_EQ_CTRL_CMD_PCIECHN_S: c_int = 16;

pub const FW_EQ_CTRL_CMD_IQID_S: c_int = 0;

pub const FW_EQ_CTRL_CMD_DCAEN_S: c_int = 31;

pub const FW_EQ_CTRL_CMD_DCACPU_S: c_int = 26;

pub const FW_EQ_CTRL_CMD_FBMIN_S: c_int = 23;

pub const FW_EQ_CTRL_CMD_FBMAX_S: c_int = 20;

pub const FW_EQ_CTRL_CMD_CIDXFTHRESHO_S: c_int = 19;

pub const FW_EQ_CTRL_CMD_CIDXFTHRESH_S: c_int = 16;

pub const FW_EQ_CTRL_CMD_EQSIZE_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_eq_ofld_cmd {
    pub op_to_vfn: __be32,
    pub alloc_to_len16: __be32,
    pub eqid_pkd: __be32,
    pub physeqid_pkd: __be32,
    pub fetchszm_to_iqid: __be32,
    pub dcaen_to_eqsize: __be32,
    pub eqaddr: __be64,
}

pub const FW_EQ_OFLD_CMD_PFN_S: c_int = 8;

pub const FW_EQ_OFLD_CMD_VFN_S: c_int = 0;

pub const FW_EQ_OFLD_CMD_ALLOC_S: c_int = 31;

pub const FW_EQ_OFLD_CMD_FREE_S: c_int = 30;

pub const FW_EQ_OFLD_CMD_MODIFY_S: c_int = 29;

pub const FW_EQ_OFLD_CMD_EQSTART_S: c_int = 28;

pub const FW_EQ_OFLD_CMD_EQSTOP_S: c_int = 27;

pub const FW_EQ_OFLD_CMD_EQID_S: c_int = 0;
pub const FW_EQ_OFLD_CMD_EQID_M: c_uint = 0xfffff;

pub const FW_EQ_OFLD_CMD_PHYSEQID_S: c_int = 0;
pub const FW_EQ_OFLD_CMD_PHYSEQID_M: c_uint = 0xfffff;

pub const FW_EQ_OFLD_CMD_FETCHSZM_S: c_int = 26;

pub const FW_EQ_OFLD_CMD_STATUSPGNS_S: c_int = 25;

pub const FW_EQ_OFLD_CMD_STATUSPGRO_S: c_int = 24;

pub const FW_EQ_OFLD_CMD_FETCHNS_S: c_int = 23;

pub const FW_EQ_OFLD_CMD_FETCHRO_S: c_int = 22;

pub const FW_EQ_OFLD_CMD_HOSTFCMODE_S: c_int = 20;

pub const FW_EQ_OFLD_CMD_CPRIO_S: c_int = 19;

pub const FW_EQ_OFLD_CMD_ONCHIP_S: c_int = 18;

pub const FW_EQ_OFLD_CMD_PCIECHN_S: c_int = 16;

pub const FW_EQ_OFLD_CMD_IQID_S: c_int = 0;

pub const FW_EQ_OFLD_CMD_DCAEN_S: c_int = 31;

pub const FW_EQ_OFLD_CMD_DCACPU_S: c_int = 26;

pub const FW_EQ_OFLD_CMD_FBMIN_S: c_int = 23;

pub const FW_EQ_OFLD_CMD_FBMAX_S: c_int = 20;

pub const FW_EQ_OFLD_CMD_CIDXFTHRESHO_S: c_int = 19;

pub const FW_EQ_OFLD_CMD_CIDXFTHRESH_S: c_int = 16;

pub const FW_EQ_OFLD_CMD_EQSIZE_S: c_int = 0;

//
// Macros for VIID parsing:
// VIID - [10:8] PFN, [7] VI Valid, [6:0] VI number
//
pub const FW_VIID_PFN_S: c_int = 8;
pub const FW_VIID_PFN_M: c_uint = 0x7;

pub const FW_VIID_VIVLD_S: c_int = 7;
pub const FW_VIID_VIVLD_M: c_uint = 0x1;

pub const FW_VIID_VIN_S: c_int = 0;
pub const FW_VIID_VIN_M: c_uint = 0x7F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_cmd {
    pub op_to_vfn: __be32,
    pub alloc_to_len16: __be32,
    pub type_viid: __be16,
    pub mac: [u8; 6],
    pub portid_pkd: u8,
    pub nmac: u8,
    pub nmac0: [u8; 6],
    pub rsssize_pkd: __be16,
    pub nmac1: [u8; 6],
    pub idsiiq_pkd: __be16,
    pub nmac2: [u8; 6],
    pub idseiq_pkd: __be16,
    pub nmac3: [u8; 6],
    pub r9: __be64,
    pub r10: __be64,
}

pub const FW_VI_CMD_PFN_S: c_int = 8;

pub const FW_VI_CMD_VFN_S: c_int = 0;

pub const FW_VI_CMD_ALLOC_S: c_int = 31;

pub const FW_VI_CMD_FREE_S: c_int = 30;

pub const FW_VI_CMD_VFVLD_S: c_int = 24;
pub const FW_VI_CMD_VFVLD_M: c_uint = 0x1;

pub const FW_VI_CMD_VIN_S: c_int = 16;
pub const FW_VI_CMD_VIN_M: c_uint = 0xff;

pub const FW_VI_CMD_VIID_S: c_int = 0;
pub const FW_VI_CMD_VIID_M: c_uint = 0xfff;

pub const FW_VI_CMD_PORTID_S: c_int = 4;
pub const FW_VI_CMD_PORTID_M: c_uint = 0xf;

pub const FW_VI_CMD_RSSSIZE_S: c_int = 0;
pub const FW_VI_CMD_RSSSIZE_M: c_uint = 0x7ff;

// Special VI_MAC command index ids
pub const FW_VI_MAC_ADD_MAC: c_uint = 0x3FF;
pub const FW_VI_MAC_ADD_PERSIST_MAC: c_uint = 0x3FE;
pub const FW_VI_MAC_MAC_BASED_FREE: c_uint = 0x3FD;
pub const FW_VI_MAC_ID_BASED_FREE: c_uint = 0x3FC;
pub const FW_CLS_TCAM_NUM_ENTRIES: c_int = 336;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_vi_mac_smac {
    FW_VI_MAC_MPS_TCAM_ENTRY,
    FW_VI_MAC_MPS_TCAM_ONLY,
    FW_VI_MAC_SMT_ONLY,
    FW_VI_MAC_SMT_AND_MPSTCAM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_vi_mac_result {
    FW_VI_MAC_R_SUCCESS,
    FW_VI_MAC_R_F_NONEXISTENT_NOMEM,
    FW_VI_MAC_R_SMAC_FAIL,
    FW_VI_MAC_R_F_ACL_CHECK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_vi_mac_entry_types {
    FW_VI_MAC_TYPE_EXACTMAC,
    FW_VI_MAC_TYPE_HASHVEC,
    FW_VI_MAC_TYPE_RAW,
    FW_VI_MAC_TYPE_EXACTMAC_VNI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_mac_cmd {
    pub op_to_viid: __be32,
    pub freemacs_to_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_vi_mac {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_mac_exact {
    pub valid_to_idx: __be16,
    pub macaddr: [u8; 6],
    pub exact: [}; 7],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_mac_hash {
    pub hashvec: __be64,
    pub hash: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_mac_raw {
    pub raw_idx_pkd: __be32,
    pub data0_pkd: __be32,
    pub data1: [__be32; 2],
    pub data0m_pkd: __be64,
    pub data1m: [__be32; 2],
    pub raw: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_mac_vni {
    pub valid_to_idx: __be16,
    pub macaddr: [__u8; 6],
    pub r7: __be16,
    pub macaddr_mask: [__u8; 6],
    pub lookup_type_to_vni: __be32,
    pub vni_mask_pkd: __be32,
    pub exact_vni: [}; 2],
    pub u: },
}

pub const FW_VI_MAC_CMD_SMTID_S: c_int = 12;
pub const FW_VI_MAC_CMD_SMTID_M: c_uint = 0xff;

pub const FW_VI_MAC_CMD_VIID_S: c_int = 0;

pub const FW_VI_MAC_CMD_FREEMACS_S: c_int = 31;

pub const FW_VI_MAC_CMD_ENTRY_TYPE_S: c_int = 23;
pub const FW_VI_MAC_CMD_ENTRY_TYPE_M: c_uint = 0x7;

pub const FW_VI_MAC_CMD_HASHVECEN_S: c_int = 23;

pub const FW_VI_MAC_CMD_HASHUNIEN_S: c_int = 22;

pub const FW_VI_MAC_CMD_VALID_S: c_int = 15;

pub const FW_VI_MAC_CMD_PRIO_S: c_int = 12;

pub const FW_VI_MAC_CMD_SMAC_RESULT_S: c_int = 10;
pub const FW_VI_MAC_CMD_SMAC_RESULT_M: c_uint = 0x3;

pub const FW_VI_MAC_CMD_IDX_S: c_int = 0;
pub const FW_VI_MAC_CMD_IDX_M: c_uint = 0x3ff;

pub const FW_VI_MAC_CMD_RAW_IDX_S: c_int = 16;
pub const FW_VI_MAC_CMD_RAW_IDX_M: c_uint = 0xffff;

pub const FW_VI_MAC_CMD_LOOKUP_TYPE_S: c_int = 31;
pub const FW_VI_MAC_CMD_LOOKUP_TYPE_M: c_uint = 0x1;

pub const FW_VI_MAC_CMD_DIP_HIT_S: c_int = 30;
pub const FW_VI_MAC_CMD_DIP_HIT_M: c_uint = 0x1;

pub const FW_VI_MAC_CMD_VNI_S: c_int = 0;
pub const FW_VI_MAC_CMD_VNI_M: c_uint = 0xffffff;

pub const FW_VI_MAC_CMD_VNI_MASK_S: c_int = 0;
pub const FW_VI_MAC_CMD_VNI_MASK_M: c_uint = 0xffffff;

pub const FW_RXMODE_MTU_NO_CHG: c_int = 65535;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_rxmode_cmd {
    pub op_to_viid: __be32,
    pub retval_len16: __be32,
    pub mtu_to_vlanexen: __be32,
    pub r4_lo: __be32,
}

pub const FW_VI_RXMODE_CMD_VIID_S: c_int = 0;

pub const FW_VI_RXMODE_CMD_MTU_S: c_int = 16;
pub const FW_VI_RXMODE_CMD_MTU_M: c_uint = 0xffff;

pub const FW_VI_RXMODE_CMD_PROMISCEN_S: c_int = 14;
pub const FW_VI_RXMODE_CMD_PROMISCEN_M: c_uint = 0x3;

pub const FW_VI_RXMODE_CMD_ALLMULTIEN_S: c_int = 12;
pub const FW_VI_RXMODE_CMD_ALLMULTIEN_M: c_uint = 0x3;

pub const FW_VI_RXMODE_CMD_BROADCASTEN_S: c_int = 10;
pub const FW_VI_RXMODE_CMD_BROADCASTEN_M: c_uint = 0x3;

pub const FW_VI_RXMODE_CMD_VLANEXEN_S: c_int = 8;
pub const FW_VI_RXMODE_CMD_VLANEXEN_M: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_enable_cmd {
    pub op_to_viid: __be32,
    pub ien_to_len16: __be32,
    pub blinkdur: __be16,
    pub r3: __be16,
    pub r4: __be32,
}

pub const FW_VI_ENABLE_CMD_VIID_S: c_int = 0;

pub const FW_VI_ENABLE_CMD_IEN_S: c_int = 31;

pub const FW_VI_ENABLE_CMD_EEN_S: c_int = 30;

pub const FW_VI_ENABLE_CMD_LED_S: c_int = 29;

pub const FW_VI_ENABLE_CMD_DCB_INFO_S: c_int = 28;

// VI VF stats offset definitions
pub const VI_VF_NUM_STATS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_vi_stats_vf_index {
    FW_VI_VF_STAT_TX_BCAST_BYTES_IX,
    FW_VI_VF_STAT_TX_BCAST_FRAMES_IX,
    FW_VI_VF_STAT_TX_MCAST_BYTES_IX,
    FW_VI_VF_STAT_TX_MCAST_FRAMES_IX,
    FW_VI_VF_STAT_TX_UCAST_BYTES_IX,
    FW_VI_VF_STAT_TX_UCAST_FRAMES_IX,
    FW_VI_VF_STAT_TX_DROP_FRAMES_IX,
    FW_VI_VF_STAT_TX_OFLD_BYTES_IX,
    FW_VI_VF_STAT_TX_OFLD_FRAMES_IX,
    FW_VI_VF_STAT_RX_BCAST_BYTES_IX,
    FW_VI_VF_STAT_RX_BCAST_FRAMES_IX,
    FW_VI_VF_STAT_RX_MCAST_BYTES_IX,
    FW_VI_VF_STAT_RX_MCAST_FRAMES_IX,
    FW_VI_VF_STAT_RX_UCAST_BYTES_IX,
    FW_VI_VF_STAT_RX_UCAST_FRAMES_IX,
    FW_VI_VF_STAT_RX_ERR_FRAMES_IX
}

// VI PF stats offset definitions
pub const VI_PF_NUM_STATS: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_vi_stats_pf_index {
    FW_VI_PF_STAT_TX_BCAST_BYTES_IX,
    FW_VI_PF_STAT_TX_BCAST_FRAMES_IX,
    FW_VI_PF_STAT_TX_MCAST_BYTES_IX,
    FW_VI_PF_STAT_TX_MCAST_FRAMES_IX,
    FW_VI_PF_STAT_TX_UCAST_BYTES_IX,
    FW_VI_PF_STAT_TX_UCAST_FRAMES_IX,
    FW_VI_PF_STAT_TX_OFLD_BYTES_IX,
    FW_VI_PF_STAT_TX_OFLD_FRAMES_IX,
    FW_VI_PF_STAT_RX_BYTES_IX,
    FW_VI_PF_STAT_RX_FRAMES_IX,
    FW_VI_PF_STAT_RX_BCAST_BYTES_IX,
    FW_VI_PF_STAT_RX_BCAST_FRAMES_IX,
    FW_VI_PF_STAT_RX_MCAST_BYTES_IX,
    FW_VI_PF_STAT_RX_MCAST_FRAMES_IX,
    FW_VI_PF_STAT_RX_UCAST_BYTES_IX,
    FW_VI_PF_STAT_RX_UCAST_FRAMES_IX,
    FW_VI_PF_STAT_RX_ERR_FRAMES_IX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_stats_cmd {
    pub op_to_viid: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_vi_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_stats_ctl {
    pub nstats_ix: __be16,
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
pub struct fw_vi_stats_pf {
    pub tx_bcast_bytes: __be64,
    pub tx_bcast_frames: __be64,
    pub tx_mcast_bytes: __be64,
    pub tx_mcast_frames: __be64,
    pub tx_ucast_bytes: __be64,
    pub tx_ucast_frames: __be64,
    pub tx_offload_bytes: __be64,
    pub tx_offload_frames: __be64,
    pub rx_pf_bytes: __be64,
    pub rx_pf_frames: __be64,
    pub rx_bcast_bytes: __be64,
    pub rx_bcast_frames: __be64,
    pub rx_mcast_bytes: __be64,
    pub rx_mcast_frames: __be64,
    pub rx_ucast_bytes: __be64,
    pub rx_ucast_frames: __be64,
    pub rx_err_frames: __be64,
    pub pf: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_vi_stats_vf {
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
    pub vf: },
    pub u: },
}

pub const FW_VI_STATS_CMD_VIID_S: c_int = 0;

pub const FW_VI_STATS_CMD_NSTATS_S: c_int = 12;

pub const FW_VI_STATS_CMD_IX_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_acl_mac_cmd {
    pub op_to_vfn: __be32,
    pub en_to_len16: __be32,
    pub nmac: u8,
    pub r3: [u8; 7],
    pub r4: __be16,
    pub macaddr0: [u8; 6],
    pub r5: __be16,
    pub macaddr1: [u8; 6],
    pub r6: __be16,
    pub macaddr2: [u8; 6],
    pub r7: __be16,
    pub macaddr3: [u8; 6],
}

pub const FW_ACL_MAC_CMD_PFN_S: c_int = 8;

pub const FW_ACL_MAC_CMD_VFN_S: c_int = 0;

pub const FW_ACL_MAC_CMD_EN_S: c_int = 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_acl_vlan_cmd {
    pub op_to_vfn: __be32,
    pub en_to_len16: __be32,
    pub nvlan: u8,
    pub dropnovlan_fm: u8,
    pub r3_lo: [u8; 6],
    pub vlanid: [__be16; 16],
}

pub const FW_ACL_VLAN_CMD_PFN_S: c_int = 8;

pub const FW_ACL_VLAN_CMD_VFN_S: c_int = 0;

pub const FW_ACL_VLAN_CMD_EN_S: c_int = 31;
pub const FW_ACL_VLAN_CMD_EN_M: c_uint = 0x1;

pub const FW_ACL_VLAN_CMD_DROPNOVLAN_S: c_int = 7;

pub const FW_ACL_VLAN_CMD_FM_S: c_int = 6;
pub const FW_ACL_VLAN_CMD_FM_M: c_uint = 0x1;

// old 16-bit port capabilities bitmap (fw_port_cap16_t)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_cap {
    FW_PORT_CAP_SPEED_100M		= 0x0001,
    FW_PORT_CAP_SPEED_1G		= 0x0002,
    FW_PORT_CAP_SPEED_25G		= 0x0004,
    FW_PORT_CAP_SPEED_10G		= 0x0008,
    FW_PORT_CAP_SPEED_40G		= 0x0010,
    FW_PORT_CAP_SPEED_100G		= 0x0020,
    FW_PORT_CAP_FC_RX		= 0x0040,
    FW_PORT_CAP_FC_TX		= 0x0080,
    FW_PORT_CAP_ANEG		= 0x0100,
    FW_PORT_CAP_MDIAUTO		= 0x0200,
    FW_PORT_CAP_MDISTRAIGHT		= 0x0400,
    FW_PORT_CAP_FEC_RS		= 0x0800,
    FW_PORT_CAP_FEC_BASER_RS	= 0x1000,
    FW_PORT_CAP_FORCE_PAUSE		= 0x2000,
    FW_PORT_CAP_802_3_PAUSE		= 0x4000,
    FW_PORT_CAP_802_3_ASM_DIR	= 0x8000,
}

pub const FW_PORT_CAP_SPEED_S: c_int = 0;
pub const FW_PORT_CAP_SPEED_M: c_uint = 0x3f;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_mdi {
    FW_PORT_CAP_MDI_UNCHANGED,
    FW_PORT_CAP_MDI_AUTO,
    FW_PORT_CAP_MDI_F_STRAIGHT,
    FW_PORT_CAP_MDI_F_CROSSOVER
}

pub const FW_PORT_CAP_MDI_S: c_int = 9;

// new 32-bit port capabilities bitmap (fw_port_cap32_t)
pub const FW_PORT_CAP32_SPEED_100M: c_uint = 0x00000001UL;
pub const FW_PORT_CAP32_SPEED_1G: c_uint = 0x00000002UL;
pub const FW_PORT_CAP32_SPEED_10G: c_uint = 0x00000004UL;
pub const FW_PORT_CAP32_SPEED_25G: c_uint = 0x00000008UL;
pub const FW_PORT_CAP32_SPEED_40G: c_uint = 0x00000010UL;
pub const FW_PORT_CAP32_SPEED_50G: c_uint = 0x00000020UL;
pub const FW_PORT_CAP32_SPEED_100G: c_uint = 0x00000040UL;
pub const FW_PORT_CAP32_SPEED_200G: c_uint = 0x00000080UL;
pub const FW_PORT_CAP32_SPEED_400G: c_uint = 0x00000100UL;
pub const FW_PORT_CAP32_SPEED_RESERVED1: c_uint = 0x00000200UL;
pub const FW_PORT_CAP32_SPEED_RESERVED2: c_uint = 0x00000400UL;
pub const FW_PORT_CAP32_SPEED_RESERVED3: c_uint = 0x00000800UL;
pub const FW_PORT_CAP32_RESERVED1: c_uint = 0x0000f000UL;
pub const FW_PORT_CAP32_FC_RX: c_uint = 0x00010000UL;
pub const FW_PORT_CAP32_FC_TX: c_uint = 0x00020000UL;
pub const FW_PORT_CAP32_802_3_PAUSE: c_uint = 0x00040000UL;
pub const FW_PORT_CAP32_802_3_ASM_DIR: c_uint = 0x00080000UL;
pub const FW_PORT_CAP32_ANEG: c_uint = 0x00100000UL;
pub const FW_PORT_CAP32_MDIAUTO: c_uint = 0x00200000UL;
pub const FW_PORT_CAP32_MDISTRAIGHT: c_uint = 0x00400000UL;
pub const FW_PORT_CAP32_FEC_RS: c_uint = 0x00800000UL;
pub const FW_PORT_CAP32_FEC_BASER_RS: c_uint = 0x01000000UL;
pub const FW_PORT_CAP32_FEC_RESERVED1: c_uint = 0x02000000UL;
pub const FW_PORT_CAP32_FEC_RESERVED2: c_uint = 0x04000000UL;
pub const FW_PORT_CAP32_FEC_RESERVED3: c_uint = 0x08000000UL;
pub const FW_PORT_CAP32_FORCE_PAUSE: c_uint = 0x10000000UL;
pub const FW_PORT_CAP32_RESERVED2: c_uint = 0xe0000000UL;
pub const FW_PORT_CAP32_SPEED_S: c_int = 0;
pub const FW_PORT_CAP32_SPEED_M: c_uint = 0xfff;

pub const FW_PORT_CAP32_FC_S: c_int = 16;
pub const FW_PORT_CAP32_FC_M: c_uint = 0x3;

pub const FW_PORT_CAP32_802_3_S: c_int = 18;
pub const FW_PORT_CAP32_802_3_M: c_uint = 0x3;

pub const FW_PORT_CAP32_ANEG_S: c_int = 20;
pub const FW_PORT_CAP32_ANEG_M: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_mdi32 {
    FW_PORT_CAP32_MDI_UNCHANGED,
    FW_PORT_CAP32_MDI_AUTO,
    FW_PORT_CAP32_MDI_F_STRAIGHT,
    FW_PORT_CAP32_MDI_F_CROSSOVER
}

pub const FW_PORT_CAP32_MDI_S: c_int = 21;
pub const FW_PORT_CAP32_MDI_M: c_int = 3;

pub const FW_PORT_CAP32_FEC_S: c_int = 23;
pub const FW_PORT_CAP32_FEC_M: c_uint = 0x1f;

// macros to isolate various 32-bit Port Capabilities sub-fields

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_action {
    FW_PORT_ACTION_L1_CFG		= 0x0001,
    FW_PORT_ACTION_L2_CFG		= 0x0002,
    FW_PORT_ACTION_GET_PORT_INFO	= 0x0003,
    FW_PORT_ACTION_L2_PPP_CFG	= 0x0004,
    FW_PORT_ACTION_L2_DCB_CFG	= 0x0005,
    FW_PORT_ACTION_DCB_READ_TRANS	= 0x0006,
    FW_PORT_ACTION_DCB_READ_RECV	= 0x0007,
    FW_PORT_ACTION_DCB_READ_DET	= 0x0008,
    FW_PORT_ACTION_L1_CFG32		= 0x0009,
    FW_PORT_ACTION_GET_PORT_INFO32	= 0x000a,
    FW_PORT_ACTION_LOW_PWR_TO_NORMAL = 0x0010,
    FW_PORT_ACTION_L1_LOW_PWR_EN	= 0x0011,
    FW_PORT_ACTION_L2_WOL_MODE_EN	= 0x0012,
    FW_PORT_ACTION_LPBK_TO_NORMAL	= 0x0020,
    FW_PORT_ACTION_L1_LPBK		= 0x0021,
    FW_PORT_ACTION_L1_PMA_LPBK	= 0x0022,
    FW_PORT_ACTION_L1_PCS_LPBK	= 0x0023,
    FW_PORT_ACTION_L1_PHYXS_CSIDE_LPBK = 0x0024,
    FW_PORT_ACTION_L1_PHYXS_ESIDE_LPBK = 0x0025,
    FW_PORT_ACTION_PHY_RESET	= 0x0040,
    FW_PORT_ACTION_PMA_RESET	= 0x0041,
    FW_PORT_ACTION_PCS_RESET	= 0x0042,
    FW_PORT_ACTION_PHYXS_RESET	= 0x0043,
    FW_PORT_ACTION_DTEXS_REEST	= 0x0044,
    FW_PORT_ACTION_AN_RESET		= 0x0045
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_l2cfg_ctlbf {
    FW_PORT_L2_CTLBF_OVLAN0	= 0x01,
    FW_PORT_L2_CTLBF_OVLAN1	= 0x02,
    FW_PORT_L2_CTLBF_OVLAN2	= 0x04,
    FW_PORT_L2_CTLBF_OVLAN3	= 0x08,
    FW_PORT_L2_CTLBF_IVLAN	= 0x10,
    FW_PORT_L2_CTLBF_TXIPG	= 0x20
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_dcb_versions {
    FW_PORT_DCB_VER_UNKNOWN,
    FW_PORT_DCB_VER_CEE1D0,
    FW_PORT_DCB_VER_CEE1D01,
    FW_PORT_DCB_VER_IEEE,
    FW_PORT_DCB_VER_AUTO = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_dcb_cfg {
    FW_PORT_DCB_CFG_PG	= 0x01,
    FW_PORT_DCB_CFG_PFC	= 0x02,
    FW_PORT_DCB_CFG_APPL	= 0x04
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_dcb_cfg_rc {
    FW_PORT_DCB_CFG_SUCCESS	= 0x0,
    FW_PORT_DCB_CFG_ERROR	= 0x1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_dcb_type {
    FW_PORT_DCB_TYPE_PGID		= 0x00,
    FW_PORT_DCB_TYPE_PGRATE		= 0x01,
    FW_PORT_DCB_TYPE_PRIORATE	= 0x02,
    FW_PORT_DCB_TYPE_PFC		= 0x03,
    FW_PORT_DCB_TYPE_APP_ID		= 0x04,
    FW_PORT_DCB_TYPE_CONTROL	= 0x05,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_dcb_feature_state {
    FW_PORT_DCB_FEATURE_STATE_PENDING = 0x0,
    FW_PORT_DCB_FEATURE_STATE_SUCCESS = 0x1,
    FW_PORT_DCB_FEATURE_STATE_ERROR	= 0x2,
    FW_PORT_DCB_FEATURE_STATE_TIMEOUT = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_cmd {
    pub op_to_portid: __be32,
    pub action_to_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_port {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_l1cfg {
    pub rcap: __be32,
    pub r: __be32,
    pub l1cfg: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_l2cfg {
    pub ctlbf: __u8,
    pub ovlan3_to_ivlan0: __u8,
    pub ivlantype: __be16,
    pub txipg_force_pinfo: __be16,
    pub mtu: __be16,
    pub ovlan0mask: __be16,
    pub ovlan0type: __be16,
    pub ovlan1mask: __be16,
    pub ovlan1type: __be16,
    pub ovlan2mask: __be16,
    pub ovlan2type: __be16,
    pub ovlan3mask: __be16,
    pub ovlan3type: __be16,
    pub l2cfg: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_info {
    pub lstatus_to_modtype: __be32,
    pub pcap: __be16,
    pub acap: __be16,
    pub mtu: __be16,
    pub cbllen: __u8,
    pub auxlinfo: __u8,
    pub dcbxdis_pkd: __u8,
    pub r8_lo: __u8,
    pub lpacap: __be16,
    pub r9: __be64,
    pub info: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_diags {
    pub diagop: __u8,
    pub r: [__u8; 3],
    pub diagval: __be32,
    pub diags: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_port_dcb {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_dcb_pgid {
    pub type: __u8,
    pub apply_pkd: __u8,
    pub r10_lo: [__u8; 2],
    pub pgid: __be32,
    pub r11: __be64,
    pub pgid: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_dcb_pgrate {
    pub type: __u8,
    pub apply_pkd: __u8,
    pub r10_lo: [__u8; 5],
    pub num_tcs_supported: __u8,
    pub pgrate: [__u8; 8],
    pub tsa: [__u8; 8],
    pub pgrate: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_dcb_priorate {
    pub type: __u8,
    pub apply_pkd: __u8,
    pub r10_lo: [__u8; 6],
    pub strict_priorate: [__u8; 8],
    pub priorate: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_dcb_pfc {
    pub type: __u8,
    pub pfcen: __u8,
    pub r10: [__u8; 5],
    pub max_pfc_tcs: __u8,
    pub r11: __be64,
    pub pfc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_app_priority {
    pub type: __u8,
    pub r10: [__u8; 2],
    pub idx: __u8,
    pub user_prio_map: __u8,
    pub sel_field: __u8,
    pub protocolid: __be16,
    pub r12: __be64,
    pub app_priority: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_dcb_control {
    pub type: __u8,
    pub all_syncd_pkd: __u8,
    pub dcb_version_to_app_state: __be16,
    pub r11: __be32,
    pub r12: __be64,
    pub control: },
    pub dcb: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_l1cfg32 {
    pub rcap32: __be32,
    pub r: __be32,
    pub l1cfg32: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_info32 {
    pub lstatus32_to_cbllen32: __be32,
    pub auxlinfo32_mtu32: __be32,
    pub linkattr32: __be32,
    pub pcaps32: __be32,
    pub acaps32: __be32,
    pub lpacaps32: __be32,
    pub info32: },
    pub u: },
}

pub const FW_PORT_CMD_READ_S: c_int = 22;

pub const FW_PORT_CMD_PORTID_S: c_int = 0;
pub const FW_PORT_CMD_PORTID_M: c_uint = 0xf;

pub const FW_PORT_CMD_ACTION_S: c_int = 16;
pub const FW_PORT_CMD_ACTION_M: c_uint = 0xffff;

pub const FW_PORT_CMD_OVLAN3_S: c_int = 7;

pub const FW_PORT_CMD_OVLAN2_S: c_int = 6;

pub const FW_PORT_CMD_OVLAN1_S: c_int = 5;

pub const FW_PORT_CMD_OVLAN0_S: c_int = 4;

pub const FW_PORT_CMD_IVLAN0_S: c_int = 3;

pub const FW_PORT_CMD_TXIPG_S: c_int = 3;

pub const FW_PORT_CMD_LSTATUS_S: c_int = 31;
pub const FW_PORT_CMD_LSTATUS_M: c_uint = 0x1;

pub const FW_PORT_CMD_LSPEED_S: c_int = 24;
pub const FW_PORT_CMD_LSPEED_M: c_uint = 0x3f;

pub const FW_PORT_CMD_TXPAUSE_S: c_int = 23;

pub const FW_PORT_CMD_RXPAUSE_S: c_int = 22;

pub const FW_PORT_CMD_MDIOCAP_S: c_int = 21;

pub const FW_PORT_CMD_MDIOADDR_S: c_int = 16;
pub const FW_PORT_CMD_MDIOADDR_M: c_uint = 0x1f;

pub const FW_PORT_CMD_LPTXPAUSE_S: c_int = 15;

pub const FW_PORT_CMD_LPRXPAUSE_S: c_int = 14;

pub const FW_PORT_CMD_PTYPE_S: c_int = 8;
pub const FW_PORT_CMD_PTYPE_M: c_uint = 0x1f;

pub const FW_PORT_CMD_LINKDNRC_S: c_int = 5;
pub const FW_PORT_CMD_LINKDNRC_M: c_uint = 0x7;

pub const FW_PORT_CMD_MODTYPE_S: c_int = 0;
pub const FW_PORT_CMD_MODTYPE_M: c_uint = 0x1f;

pub const FW_PORT_CMD_DCBXDIS_S: c_int = 7;

pub const FW_PORT_CMD_APPLY_S: c_int = 7;

pub const FW_PORT_CMD_ALL_SYNCD_S: c_int = 7;

pub const FW_PORT_CMD_DCB_VERSION_S: c_int = 12;
pub const FW_PORT_CMD_DCB_VERSION_M: c_uint = 0x7;

pub const FW_PORT_CMD_LSTATUS32_S: c_int = 31;
pub const FW_PORT_CMD_LSTATUS32_M: c_uint = 0x1;

pub const FW_PORT_CMD_LINKDNRC32_S: c_int = 28;
pub const FW_PORT_CMD_LINKDNRC32_M: c_uint = 0x7;

pub const FW_PORT_CMD_DCBXDIS32_S: c_int = 27;
pub const FW_PORT_CMD_DCBXDIS32_M: c_uint = 0x1;

pub const FW_PORT_CMD_MDIOCAP32_S: c_int = 26;
pub const FW_PORT_CMD_MDIOCAP32_M: c_uint = 0x1;

pub const FW_PORT_CMD_MDIOADDR32_S: c_int = 21;
pub const FW_PORT_CMD_MDIOADDR32_M: c_uint = 0x1f;

pub const FW_PORT_CMD_PORTTYPE32_S: c_int = 13;
pub const FW_PORT_CMD_PORTTYPE32_M: c_uint = 0xff;

pub const FW_PORT_CMD_MODTYPE32_S: c_int = 8;
pub const FW_PORT_CMD_MODTYPE32_M: c_uint = 0x1f;

pub const FW_PORT_CMD_CBLLEN32_S: c_int = 0;
pub const FW_PORT_CMD_CBLLEN32_M: c_uint = 0xff;

pub const FW_PORT_CMD_AUXLINFO32_S: c_int = 24;
pub const FW_PORT_CMD_AUXLINFO32_M: c_uint = 0xff;

pub const FW_PORT_AUXLINFO32_KX4_S: c_int = 2;
pub const FW_PORT_AUXLINFO32_KX4_M: c_uint = 0x1;

pub const FW_PORT_AUXLINFO32_KR_S: c_int = 1;
pub const FW_PORT_AUXLINFO32_KR_M: c_uint = 0x1;

pub const FW_PORT_CMD_MTU32_S: c_int = 0;
pub const FW_PORT_CMD_MTU32_M: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_type {
    FW_PORT_TYPE_FIBER_XFI,
    FW_PORT_TYPE_FIBER_XAUI,
    FW_PORT_TYPE_BT_SGMII,
    FW_PORT_TYPE_BT_XFI,
    FW_PORT_TYPE_BT_XAUI,
    FW_PORT_TYPE_KX4,
    FW_PORT_TYPE_CX4,
    FW_PORT_TYPE_KX,
    FW_PORT_TYPE_KR,
    FW_PORT_TYPE_SFP,
    FW_PORT_TYPE_BP_AP,
    FW_PORT_TYPE_BP4_AP,
    FW_PORT_TYPE_QSFP_10G,
    FW_PORT_TYPE_QSA,
    FW_PORT_TYPE_QSFP,
    FW_PORT_TYPE_BP40_BA,
    FW_PORT_TYPE_KR4_100G,
    FW_PORT_TYPE_CR4_QSFP,
    FW_PORT_TYPE_CR_QSFP,
    FW_PORT_TYPE_CR2_QSFP,
    FW_PORT_TYPE_SFP28,
    FW_PORT_TYPE_KR_SFP28,
    FW_PORT_TYPE_KR_XLAUI,

    FW_PORT_TYPE_NONE = FW_PORT_CMD_PTYPE_M
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_module_type {
    FW_PORT_MOD_TYPE_NA,
    FW_PORT_MOD_TYPE_LR,
    FW_PORT_MOD_TYPE_SR,
    FW_PORT_MOD_TYPE_ER,
    FW_PORT_MOD_TYPE_TWINAX_PASSIVE,
    FW_PORT_MOD_TYPE_TWINAX_ACTIVE,
    FW_PORT_MOD_TYPE_LRM,
    FW_PORT_MOD_TYPE_ERROR		= FW_PORT_CMD_MODTYPE_M - 3,
    FW_PORT_MOD_TYPE_UNKNOWN	= FW_PORT_CMD_MODTYPE_M - 2,
    FW_PORT_MOD_TYPE_NOTSUPPORTED	= FW_PORT_CMD_MODTYPE_M - 1,

    FW_PORT_MOD_TYPE_NONE = FW_PORT_CMD_MODTYPE_M
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_mod_sub_type {
    FW_PORT_MOD_SUB_TYPE_NA,
    FW_PORT_MOD_SUB_TYPE_MV88E114X = 0x1,
    FW_PORT_MOD_SUB_TYPE_TN8022 = 0x2,
    FW_PORT_MOD_SUB_TYPE_AQ1202 = 0x3,
    FW_PORT_MOD_SUB_TYPE_88x3120 = 0x4,
    FW_PORT_MOD_SUB_TYPE_BCM84834 = 0x5,
    FW_PORT_MOD_SUB_TYPE_BT_VSC8634 = 0x8,

// The following will never been in the VPD.  They are TWINAX cable
// lengths decoded from SFP+ module i2c PROMs.  These should
// almost certainly go somewhere else ...
//
    FW_PORT_MOD_SUB_TYPE_TWINAX_1 = 0x9,
    FW_PORT_MOD_SUB_TYPE_TWINAX_3 = 0xA,
    FW_PORT_MOD_SUB_TYPE_TWINAX_5 = 0xB,
    FW_PORT_MOD_SUB_TYPE_TWINAX_7 = 0xC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_stats_tx_index {
    FW_STAT_TX_PORT_BYTES_IX = 0,
    FW_STAT_TX_PORT_FRAMES_IX,
    FW_STAT_TX_PORT_BCAST_IX,
    FW_STAT_TX_PORT_MCAST_IX,
    FW_STAT_TX_PORT_UCAST_IX,
    FW_STAT_TX_PORT_ERROR_IX,
    FW_STAT_TX_PORT_64B_IX,
    FW_STAT_TX_PORT_65B_127B_IX,
    FW_STAT_TX_PORT_128B_255B_IX,
    FW_STAT_TX_PORT_256B_511B_IX,
    FW_STAT_TX_PORT_512B_1023B_IX,
    FW_STAT_TX_PORT_1024B_1518B_IX,
    FW_STAT_TX_PORT_1519B_MAX_IX,
    FW_STAT_TX_PORT_DROP_IX,
    FW_STAT_TX_PORT_PAUSE_IX,
    FW_STAT_TX_PORT_PPP0_IX,
    FW_STAT_TX_PORT_PPP1_IX,
    FW_STAT_TX_PORT_PPP2_IX,
    FW_STAT_TX_PORT_PPP3_IX,
    FW_STAT_TX_PORT_PPP4_IX,
    FW_STAT_TX_PORT_PPP5_IX,
    FW_STAT_TX_PORT_PPP6_IX,
    FW_STAT_TX_PORT_PPP7_IX,
    FW_NUM_PORT_TX_STATS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_stat_rx_index {
    FW_STAT_RX_PORT_BYTES_IX = 0,
    FW_STAT_RX_PORT_FRAMES_IX,
    FW_STAT_RX_PORT_BCAST_IX,
    FW_STAT_RX_PORT_MCAST_IX,
    FW_STAT_RX_PORT_UCAST_IX,
    FW_STAT_RX_PORT_MTU_ERROR_IX,
    FW_STAT_RX_PORT_MTU_CRC_ERROR_IX,
    FW_STAT_RX_PORT_CRC_ERROR_IX,
    FW_STAT_RX_PORT_LEN_ERROR_IX,
    FW_STAT_RX_PORT_SYM_ERROR_IX,
    FW_STAT_RX_PORT_64B_IX,
    FW_STAT_RX_PORT_65B_127B_IX,
    FW_STAT_RX_PORT_128B_255B_IX,
    FW_STAT_RX_PORT_256B_511B_IX,
    FW_STAT_RX_PORT_512B_1023B_IX,
    FW_STAT_RX_PORT_1024B_1518B_IX,
    FW_STAT_RX_PORT_1519B_MAX_IX,
    FW_STAT_RX_PORT_PAUSE_IX,
    FW_STAT_RX_PORT_PPP0_IX,
    FW_STAT_RX_PORT_PPP1_IX,
    FW_STAT_RX_PORT_PPP2_IX,
    FW_STAT_RX_PORT_PPP3_IX,
    FW_STAT_RX_PORT_PPP4_IX,
    FW_STAT_RX_PORT_PPP5_IX,
    FW_STAT_RX_PORT_PPP6_IX,
    FW_STAT_RX_PORT_PPP7_IX,
    FW_STAT_RX_PORT_LESS_64B_IX,
    FW_STAT_RX_PORT_MAC_ERROR_IX,
    FW_NUM_PORT_RX_STATS
}

// port stats

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_stats_cmd {
    pub op_to_portid: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_port_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_stats_ctl {
    pub nstats_bg_bm: u8,
    pub tx_ix: u8,
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
pub struct fw_port_stats_all {
    pub tx_bytes: __be64,
    pub tx_frames: __be64,
    pub tx_bcast: __be64,
    pub tx_mcast: __be64,
    pub tx_ucast: __be64,
    pub tx_error: __be64,
    pub tx_64b: __be64,
    pub tx_65b_127b: __be64,
    pub tx_128b_255b: __be64,
    pub tx_256b_511b: __be64,
    pub tx_512b_1023b: __be64,
    pub tx_1024b_1518b: __be64,
    pub tx_1519b_max: __be64,
    pub tx_drop: __be64,
    pub tx_pause: __be64,
    pub tx_ppp0: __be64,
    pub tx_ppp1: __be64,
    pub tx_ppp2: __be64,
    pub tx_ppp3: __be64,
    pub tx_ppp4: __be64,
    pub tx_ppp5: __be64,
    pub tx_ppp6: __be64,
    pub tx_ppp7: __be64,
    pub rx_bytes: __be64,
    pub rx_frames: __be64,
    pub rx_bcast: __be64,
    pub rx_mcast: __be64,
    pub rx_ucast: __be64,
    pub rx_mtu_error: __be64,
    pub rx_mtu_crc_error: __be64,
    pub rx_crc_error: __be64,
    pub rx_len_error: __be64,
    pub rx_sym_error: __be64,
    pub rx_64b: __be64,
    pub rx_65b_127b: __be64,
    pub rx_128b_255b: __be64,
    pub rx_256b_511b: __be64,
    pub rx_512b_1023b: __be64,
    pub rx_1024b_1518b: __be64,
    pub rx_1519b_max: __be64,
    pub rx_pause: __be64,
    pub rx_ppp0: __be64,
    pub rx_ppp1: __be64,
    pub rx_ppp2: __be64,
    pub rx_ppp3: __be64,
    pub rx_ppp4: __be64,
    pub rx_ppp5: __be64,
    pub rx_ppp6: __be64,
    pub rx_ppp7: __be64,
    pub rx_less_64b: __be64,
    pub rx_bg_drop: __be64,
    pub rx_bg_trunc: __be64,
    pub all: },
    pub u: },
}

// port loopback stats
pub const FW_NUM_LB_STATS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_port_lb_stats_index {
    FW_STAT_LB_PORT_BYTES_IX,
    FW_STAT_LB_PORT_FRAMES_IX,
    FW_STAT_LB_PORT_BCAST_IX,
    FW_STAT_LB_PORT_MCAST_IX,
    FW_STAT_LB_PORT_UCAST_IX,
    FW_STAT_LB_PORT_ERROR_IX,
    FW_STAT_LB_PORT_64B_IX,
    FW_STAT_LB_PORT_65B_127B_IX,
    FW_STAT_LB_PORT_128B_255B_IX,
    FW_STAT_LB_PORT_256B_511B_IX,
    FW_STAT_LB_PORT_512B_1023B_IX,
    FW_STAT_LB_PORT_1024B_1518B_IX,
    FW_STAT_LB_PORT_1519B_MAX_IX,
    FW_STAT_LB_PORT_DROP_FRAMES_IX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_lb_stats_cmd {
    pub op_to_lbport: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_port_lb_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_port_lb_stats_ctl {
    pub nstats_bg_bm: u8,
    pub ix_pkd: u8,
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
pub struct fw_port_lb_stats_all {
    pub tx_bytes: __be64,
    pub tx_frames: __be64,
    pub tx_bcast: __be64,
    pub tx_mcast: __be64,
    pub tx_ucast: __be64,
    pub tx_error: __be64,
    pub tx_64b: __be64,
    pub tx_65b_127b: __be64,
    pub tx_128b_255b: __be64,
    pub tx_256b_511b: __be64,
    pub tx_512b_1023b: __be64,
    pub tx_1024b_1518b: __be64,
    pub tx_1519b_max: __be64,
    pub rx_lb_drop: __be64,
    pub rx_lb_trunc: __be64,
    pub all: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ptp_subop {
// none
    FW_PTP_SC_INIT_TIMER            = 0x00,
    FW_PTP_SC_TX_TYPE               = 0x01,
// init
    FW_PTP_SC_RXTIME_STAMP          = 0x08,
    FW_PTP_SC_RDRX_TYPE             = 0x09,
// ts
    FW_PTP_SC_ADJ_FREQ              = 0x10,
    FW_PTP_SC_ADJ_TIME              = 0x11,
    FW_PTP_SC_ADJ_FTIME             = 0x12,
    FW_PTP_SC_WALL_CLOCK            = 0x13,
    FW_PTP_SC_GET_TIME              = 0x14,
    FW_PTP_SC_SET_TIME              = 0x15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ptp_cmd {
    pub op_to_portid: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ptp {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ptp_sc {
    pub sc: __u8,
    pub r3: [__u8; 7],
    pub scmd: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ptp_init {
    pub sc: __u8,
    pub txchan: __u8,
    pub absid: __be16,
    pub mode: __be16,
    pub r3: __be16,
    pub init: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ptp_ts {
    pub sc: __u8,
    pub sign: __u8,
    pub r3: __be16,
    pub ppb: __be32,
    pub tm: __be64,
    pub ts: },
    pub u: },
    pub r3: __be64,
}

pub const FW_PTP_CMD_PORTID_S: c_int = 0;
pub const FW_PTP_CMD_PORTID_M: c_uint = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_ind_tbl_cmd {
    pub op_to_viid: __be32,
    pub retval_len16: __be32,
    pub niqid: __be16,
    pub startidx: __be16,
    pub r3: __be32,
    pub iq0_to_iq2: __be32,
    pub iq3_to_iq5: __be32,
    pub iq6_to_iq8: __be32,
    pub iq9_to_iq11: __be32,
    pub iq12_to_iq14: __be32,
    pub iq15_to_iq17: __be32,
    pub iq18_to_iq20: __be32,
    pub iq21_to_iq23: __be32,
    pub iq24_to_iq26: __be32,
    pub iq27_to_iq29: __be32,
    pub iq30_iq31: __be32,
    pub r15_lo: __be32,
}

pub const FW_RSS_IND_TBL_CMD_VIID_S: c_int = 0;

pub const FW_RSS_IND_TBL_CMD_IQ0_S: c_int = 20;

pub const FW_RSS_IND_TBL_CMD_IQ1_S: c_int = 10;

pub const FW_RSS_IND_TBL_CMD_IQ2_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_glb_config_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_rss_glb_config {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_glb_config_manual {
    pub mode_pkd: __be32,
    pub r3: __be32,
    pub r4: __be64,
    pub r5: __be64,
    pub manual: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_glb_config_basicvirtual {
    pub mode_pkd: __be32,
    pub synmapen_to_hashtoeplitz: __be32,
    pub r8: __be64,
    pub r9: __be64,
    pub basicvirtual: },
    pub u: },
}

pub const FW_RSS_GLB_CONFIG_CMD_MODE_S: c_int = 28;
pub const FW_RSS_GLB_CONFIG_CMD_MODE_M: c_uint = 0xf;

pub const FW_RSS_GLB_CONFIG_CMD_MODE_MANUAL: c_int = 0;
pub const FW_RSS_GLB_CONFIG_CMD_MODE_BASICVIRTUAL: c_int = 1;
pub const FW_RSS_GLB_CONFIG_CMD_SYNMAPEN_S: c_int = 8;

pub const FW_RSS_GLB_CONFIG_CMD_SYN4TUPENIPV6_S: c_int = 7;

pub const FW_RSS_GLB_CONFIG_CMD_SYN2TUPENIPV6_S: c_int = 6;

pub const FW_RSS_GLB_CONFIG_CMD_SYN4TUPENIPV4_S: c_int = 5;

pub const FW_RSS_GLB_CONFIG_CMD_SYN2TUPENIPV4_S: c_int = 4;

pub const FW_RSS_GLB_CONFIG_CMD_OFDMAPEN_S: c_int = 3;

pub const FW_RSS_GLB_CONFIG_CMD_TNLMAPEN_S: c_int = 2;

pub const FW_RSS_GLB_CONFIG_CMD_TNLALLLKP_S: c_int = 1;

pub const FW_RSS_GLB_CONFIG_CMD_HASHTOEPLITZ_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_vi_config_cmd {
    pub op_to_viid: __be32,

    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_rss_vi_config {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_vi_config_manual {
    pub r3: __be64,
    pub r4: __be64,
    pub r5: __be64,
    pub manual: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rss_vi_config_basicvirtual {
    pub r6: __be32,
    pub defaultq_to_udpen: __be32,
    pub r9: __be64,
    pub r10: __be64,
    pub basicvirtual: },
    pub u: },
}

pub const FW_RSS_VI_CONFIG_CMD_VIID_S: c_int = 0;

pub const FW_RSS_VI_CONFIG_CMD_DEFAULTQ_S: c_int = 16;
pub const FW_RSS_VI_CONFIG_CMD_DEFAULTQ_M: c_uint = 0x3ff;

pub const FW_RSS_VI_CONFIG_CMD_IP6FOURTUPEN_S: c_int = 4;

pub const FW_RSS_VI_CONFIG_CMD_IP6TWOTUPEN_S: c_int = 3;

pub const FW_RSS_VI_CONFIG_CMD_IP4FOURTUPEN_S: c_int = 2;

pub const FW_RSS_VI_CONFIG_CMD_IP4TWOTUPEN_S: c_int = 1;

pub const FW_RSS_VI_CONFIG_CMD_UDPEN_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_sched_sc {
    FW_SCHED_SC_PARAMS		= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_sched_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_sched {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_sched_config {
    pub sc: __u8,
    pub type: __u8,
    pub minmaxen: __u8,
    pub r3: [__u8; 5],
    pub nclasses: [__u8; 4],
    pub r4: __be32,
    pub config: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_sched_params {
    pub sc: __u8,
    pub type: __u8,
    pub level: __u8,
    pub mode: __u8,
    pub unit: __u8,
    pub rate: __u8,
    pub ch: __u8,
    pub cl: __u8,
    pub min: __be32,
    pub max: __be32,
    pub weight: __be16,
    pub pktsize: __be16,
    pub burstsize: __be16,
    pub r4: __be16,
    pub params: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_clip_cmd {
    pub op_to_write: __be32,
    pub alloc_to_len16: __be32,
    pub ip_hi: __be64,
    pub ip_lo: __be64,
    pub r4: [__be32; 2],
}

pub const FW_CLIP_CMD_ALLOC_S: c_int = 31;

pub const FW_CLIP_CMD_FREE_S: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_error_type {
    FW_ERROR_TYPE_EXCEPTION		= 0x0,
    FW_ERROR_TYPE_HWMODULE		= 0x1,
    FW_ERROR_TYPE_WR		= 0x2,
    FW_ERROR_TYPE_ACL		= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_error_cmd {
    pub op_to_type: __be32,
    pub len16_pkd: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_error {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_error_exception {
    pub info: [__be32; 6],
    pub exception: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_error_hwmodule {
    pub regaddr: __be32,
    pub regval: __be32,
    pub hwmodule: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_error_wr {
    pub cidx: __be16,
    pub pfn_vfn: __be16,
    pub eqid: __be32,
    pub wrhdr: [u8; 16],
    pub wr: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_error_acl {
    pub cidx: __be16,
    pub pfn_vfn: __be16,
    pub eqid: __be32,
    pub mv_pkd: __be16,
    pub val: [u8; 6],
    pub r4: __be64,
    pub acl: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_debug_cmd {
    pub op_type: __be32,
    pub len16_pkd: __be32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_debug {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_debug_assert {
    pub fcid: __be32,
    pub line: __be32,
    pub x: __be32,
    pub y: __be32,
    pub filename_0_7: [u8; 8],
    pub filename_8_15: [u8; 8],
    pub r3: __be64,
    pub assert: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_debug_prt {
    pub dprtstridx: __be16,
    pub r3: [__be16; 3],
    pub dprtstrparam0: __be32,
    pub dprtstrparam1: __be32,
    pub dprtstrparam2: __be32,
    pub dprtstrparam3: __be32,
    pub prt: },
    pub u: },
}

pub const FW_DEBUG_CMD_TYPE_S: c_int = 0;
pub const FW_DEBUG_CMD_TYPE_M: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_hma_cmd {
    pub op_pkd: __be32,
    pub retval_len16: __be32,
    pub mode_to_pcie_params: __be32,
    pub naddr_size: __be32,
    pub addr_size_pkd: __be32,
    pub r6: __be32,
    pub phy_address: [__be64; 5],
}

pub const FW_HMA_CMD_MODE_S: c_int = 31;
pub const FW_HMA_CMD_MODE_M: c_uint = 0x1;

pub const FW_HMA_CMD_SOC_S: c_int = 30;
pub const FW_HMA_CMD_SOC_M: c_uint = 0x1;

pub const FW_HMA_CMD_EOC_S: c_int = 29;
pub const FW_HMA_CMD_EOC_M: c_uint = 0x1;

pub const FW_HMA_CMD_PCIE_PARAMS_S: c_int = 0;
pub const FW_HMA_CMD_PCIE_PARAMS_M: c_uint = 0x7ffffff;

pub const FW_HMA_CMD_NADDR_S: c_int = 12;
pub const FW_HMA_CMD_NADDR_M: c_uint = 0x3f;

pub const FW_HMA_CMD_SIZE_S: c_int = 0;
pub const FW_HMA_CMD_SIZE_M: c_uint = 0xfff;

pub const FW_HMA_CMD_ADDR_SIZE_S: c_int = 11;
pub const FW_HMA_CMD_ADDR_SIZE_M: c_uint = 0x1fffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcie_fw_eval {
    PCIE_FW_EVAL_CRASH = 0,
}

pub const PCIE_FW_ERR_S: c_int = 31;

pub const PCIE_FW_INIT_S: c_int = 30;

pub const PCIE_FW_HALT_S: c_int = 29;

pub const PCIE_FW_EVAL_S: c_int = 24;
pub const PCIE_FW_EVAL_M: c_uint = 0x7;

pub const PCIE_FW_MASTER_VLD_S: c_int = 15;

pub const PCIE_FW_MASTER_S: c_int = 12;
pub const PCIE_FW_MASTER_M: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_hdr {
    pub ver: u8,
    pub /: *mut *mut u8 chip; / terminator chip type,
    pub /: *mut *mut __be16 len512; / bin length in units of 512-bytes,
    pub /: *mut *mut __be32 fw_ver; / firmware version,
    pub tp_microcode_ver: __be32,
    pub intfver_nic: u8,
    pub intfver_vnic: u8,
    pub intfver_ofld: u8,
    pub intfver_ri: u8,
    pub intfver_iscsipdu: u8,
    pub intfver_iscsi: u8,
    pub intfver_fcoepdu: u8,
    pub intfver_fcoe: u8,
    pub reserved2: __u32,
    pub reserved3: __u32,
    pub reserved4: __u32,
    pub flags: __be32,
    pub reserved6: [__be32; 23],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_hdr_chip {
    FW_HDR_CHIP_T4,
    FW_HDR_CHIP_T5,
    FW_HDR_CHIP_T6
}

pub const FW_HDR_FW_VER_MAJOR_S: c_int = 24;
pub const FW_HDR_FW_VER_MAJOR_M: c_uint = 0xff;

pub const FW_HDR_FW_VER_MINOR_S: c_int = 16;
pub const FW_HDR_FW_VER_MINOR_M: c_uint = 0xff;

pub const FW_HDR_FW_VER_MICRO_S: c_int = 8;
pub const FW_HDR_FW_VER_MICRO_M: c_uint = 0xff;

pub const FW_HDR_FW_VER_BUILD_S: c_int = 0;
pub const FW_HDR_FW_VER_BUILD_M: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_hdr_intfver {
    FW_HDR_INTFVER_NIC      = 0x00,
    FW_HDR_INTFVER_VNIC     = 0x00,
    FW_HDR_INTFVER_OFLD     = 0x00,
    FW_HDR_INTFVER_RI       = 0x00,
    FW_HDR_INTFVER_ISCSIPDU = 0x00,
    FW_HDR_INTFVER_ISCSI    = 0x00,
    FW_HDR_INTFVER_FCOEPDU  = 0x00,
    FW_HDR_INTFVER_FCOE     = 0x00,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_hdr_flags {
    FW_HDR_FLAGS_RESET_HALT = 0x00000001,
}

// length of the formatting string
pub const FW_DEVLOG_FMT_LEN: c_int = 192;
// maximum number of the formatting string parameters
pub const FW_DEVLOG_FMT_PARAMS_NUM: c_int = 8;
// priority levels
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_devlog_level {
    FW_DEVLOG_LEVEL_EMERG	= 0x0,
    FW_DEVLOG_LEVEL_CRIT	= 0x1,
    FW_DEVLOG_LEVEL_ERR	= 0x2,
    FW_DEVLOG_LEVEL_NOTICE	= 0x3,
    FW_DEVLOG_LEVEL_INFO	= 0x4,
    FW_DEVLOG_LEVEL_DEBUG	= 0x5,
    FW_DEVLOG_LEVEL_MAX	= 0x5,
}

// facilities that may send a log message
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_devlog_facility {
    FW_DEVLOG_FACILITY_CORE		= 0x00,
    FW_DEVLOG_FACILITY_CF		= 0x01,
    FW_DEVLOG_FACILITY_SCHED	= 0x02,
    FW_DEVLOG_FACILITY_TIMER	= 0x04,
    FW_DEVLOG_FACILITY_RES		= 0x06,
    FW_DEVLOG_FACILITY_HW		= 0x08,
    FW_DEVLOG_FACILITY_FLR		= 0x10,
    FW_DEVLOG_FACILITY_DMAQ		= 0x12,
    FW_DEVLOG_FACILITY_PHY		= 0x14,
    FW_DEVLOG_FACILITY_MAC		= 0x16,
    FW_DEVLOG_FACILITY_PORT		= 0x18,
    FW_DEVLOG_FACILITY_VI		= 0x1A,
    FW_DEVLOG_FACILITY_FILTER	= 0x1C,
    FW_DEVLOG_FACILITY_ACL		= 0x1E,
    FW_DEVLOG_FACILITY_TM		= 0x20,
    FW_DEVLOG_FACILITY_QFC		= 0x22,
    FW_DEVLOG_FACILITY_DCB		= 0x24,
    FW_DEVLOG_FACILITY_ETH		= 0x26,
    FW_DEVLOG_FACILITY_OFLD		= 0x28,
    FW_DEVLOG_FACILITY_RI		= 0x2A,
    FW_DEVLOG_FACILITY_ISCSI	= 0x2C,
    FW_DEVLOG_FACILITY_FCOE		= 0x2E,
    FW_DEVLOG_FACILITY_FOISCSI	= 0x30,
    FW_DEVLOG_FACILITY_FOFCOE	= 0x32,
    FW_DEVLOG_FACILITY_CHNET        = 0x34,
    FW_DEVLOG_FACILITY_MAX          = 0x34,
}

// log message format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_devlog_e {
    pub timestamp: __be64,
    pub seqno: __be32,
    pub reserved1: __be16,
    pub level: __u8,
    pub facility: __u8,
    pub fmt: [__u8; FW_DEVLOG_FMT_LEN],
    pub params: [__be32; FW_DEVLOG_FMT_PARAMS_NUM],
    pub reserved3: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_devlog_cmd {
    pub op_to_write: __be32,
    pub retval_len16: __be32,
    pub level: __u8,
    pub r2: [__u8; 7],
    pub memtype_devlog_memaddr16_devlog: __be32,
    pub memsize_devlog: __be32,
    pub r3: [__be32; 2],
}

pub const FW_DEVLOG_CMD_MEMTYPE_DEVLOG_S: c_int = 28;
pub const FW_DEVLOG_CMD_MEMTYPE_DEVLOG_M: c_uint = 0xf;

pub const FW_DEVLOG_CMD_MEMADDR16_DEVLOG_S: c_int = 0;
pub const FW_DEVLOG_CMD_MEMADDR16_DEVLOG_M: c_uint = 0xfffffff;

// P C I E   F W   P F 7   R E G I S T E R
// PF7 stores the Firmware Device Log parameters which allows Host Drivers to
// access the "devlog" which needing to contact firmware.  The encoding is
// mostly the same as that returned by the DEVLOG command except for the size
// which is encoded as the number of entries in multiples-1 of 128 here rather
// than the memory size as is done in the DEVLOG command.  Thus, 0 means 128
// and 15 means 2048.  This of course in turn constrains the allowed values
// for the devlog size ...
//
pub const PCIE_FW_PF_DEVLOG: c_int = 7;
pub const PCIE_FW_PF_DEVLOG_NENTRIES128_S: c_int = 28;
pub const PCIE_FW_PF_DEVLOG_NENTRIES128_M: c_uint = 0xf;

pub const PCIE_FW_PF_DEVLOG_ADDR16_S: c_int = 4;
pub const PCIE_FW_PF_DEVLOG_ADDR16_M: c_uint = 0xffffff;

pub const PCIE_FW_PF_DEVLOG_MEMTYPE_S: c_int = 0;
pub const PCIE_FW_PF_DEVLOG_MEMTYPE_M: c_uint = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_crypto_lookaside_wr {
    pub op_to_cctx_size: __be32,
    pub len16_pkd: __be32,
    pub session_id: __be32,
    pub rx_chid_to_rx_q_id: __be32,
    pub key_addr: __be32,
    pub pld_size_hash_size: __be32,
    pub cookie: __be64,
}

pub const FW_CRYPTO_LOOKASIDE_WR_OPCODE_S: c_int = 24;
pub const FW_CRYPTO_LOOKASIDE_WR_OPCODE_M: c_uint = 0xff;

pub const FW_CRYPTO_LOOKASIDE_WR_COMPL_S: c_int = 23;
pub const FW_CRYPTO_LOOKASIDE_WR_COMPL_M: c_uint = 0x1;

pub const FW_CRYPTO_LOOKASIDE_WR_IMM_LEN_S: c_int = 15;
pub const FW_CRYPTO_LOOKASIDE_WR_IMM_LEN_M: c_uint = 0xff;

pub const FW_CRYPTO_LOOKASIDE_WR_CCTX_LOC_S: c_int = 5;
pub const FW_CRYPTO_LOOKASIDE_WR_CCTX_LOC_M: c_uint = 0x3;

pub const FW_CRYPTO_LOOKASIDE_WR_CCTX_SIZE_S: c_int = 0;
pub const FW_CRYPTO_LOOKASIDE_WR_CCTX_SIZE_M: c_uint = 0x1f;

pub const FW_CRYPTO_LOOKASIDE_WR_LEN16_S: c_int = 0;
pub const FW_CRYPTO_LOOKASIDE_WR_LEN16_M: c_uint = 0xff;

pub const FW_CRYPTO_LOOKASIDE_WR_RX_CHID_S: c_int = 29;
pub const FW_CRYPTO_LOOKASIDE_WR_RX_CHID_M: c_uint = 0x3;

pub const FW_CRYPTO_LOOKASIDE_WR_LCB_S: c_int = 27;
pub const FW_CRYPTO_LOOKASIDE_WR_LCB_M: c_uint = 0x3;

pub const FW_CRYPTO_LOOKASIDE_WR_PHASH_S: c_int = 25;
pub const FW_CRYPTO_LOOKASIDE_WR_PHASH_M: c_uint = 0x3;

pub const FW_CRYPTO_LOOKASIDE_WR_IV_S: c_int = 23;
pub const FW_CRYPTO_LOOKASIDE_WR_IV_M: c_uint = 0x3;

pub const FW_CRYPTO_LOOKASIDE_WR_FQIDX_S: c_int = 15;
pub const FW_CRYPTO_LOOKASIDE_WR_FQIDX_M: c_uint = 0xff;

pub const FW_CRYPTO_LOOKASIDE_WR_TX_CH_S: c_int = 10;
pub const FW_CRYPTO_LOOKASIDE_WR_TX_CH_M: c_uint = 0x3;

pub const FW_CRYPTO_LOOKASIDE_WR_RX_Q_ID_S: c_int = 0;
pub const FW_CRYPTO_LOOKASIDE_WR_RX_Q_ID_M: c_uint = 0x3ff;

pub const FW_CRYPTO_LOOKASIDE_WR_PLD_SIZE_S: c_int = 24;
pub const FW_CRYPTO_LOOKASIDE_WR_PLD_SIZE_M: c_uint = 0xff;

pub const FW_CRYPTO_LOOKASIDE_WR_HASH_SIZE_S: c_int = 17;
pub const FW_CRYPTO_LOOKASIDE_WR_HASH_SIZE_M: c_uint = 0x7f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_tlstx_data_wr {
    pub op_to_immdlen: __be32,
    pub flowid_len16: __be32,
    pub plen: __be32,
    pub lsodisable_to_flags: __be32,
    pub r5: __be32,
    pub ctxloc_to_exp: __be32,
    pub mfs: __be16,
    pub adjustedplen_pkd: __be16,
    pub expinplenmax_pkd: __be16,
    pub pdusinplenmax_pkd: u8,
    pub r10: u8,
}

pub const FW_TLSTX_DATA_WR_OPCODE_S: c_int = 24;
pub const FW_TLSTX_DATA_WR_OPCODE_M: c_uint = 0xff;

pub const FW_TLSTX_DATA_WR_COMPL_S: c_int = 21;
pub const FW_TLSTX_DATA_WR_COMPL_M: c_uint = 0x1;

pub const FW_TLSTX_DATA_WR_IMMDLEN_S: c_int = 0;
pub const FW_TLSTX_DATA_WR_IMMDLEN_M: c_uint = 0xff;

pub const FW_TLSTX_DATA_WR_FLOWID_S: c_int = 8;
pub const FW_TLSTX_DATA_WR_FLOWID_M: c_uint = 0xfffff;

pub const FW_TLSTX_DATA_WR_LEN16_S: c_int = 0;
pub const FW_TLSTX_DATA_WR_LEN16_M: c_uint = 0xff;

pub const FW_TLSTX_DATA_WR_LSODISABLE_S: c_int = 31;
pub const FW_TLSTX_DATA_WR_LSODISABLE_M: c_uint = 0x1;

pub const FW_TLSTX_DATA_WR_ALIGNPLD_S: c_int = 30;
pub const FW_TLSTX_DATA_WR_ALIGNPLD_M: c_uint = 0x1;

pub const FW_TLSTX_DATA_WR_ALIGNPLDSHOVE_S: c_int = 29;
pub const FW_TLSTX_DATA_WR_ALIGNPLDSHOVE_M: c_uint = 0x1;

pub const FW_TLSTX_DATA_WR_FLAGS_S: c_int = 0;
pub const FW_TLSTX_DATA_WR_FLAGS_M: c_uint = 0xfffffff;

pub const FW_TLSTX_DATA_WR_CTXLOC_S: c_int = 30;
pub const FW_TLSTX_DATA_WR_CTXLOC_M: c_uint = 0x3;

pub const FW_TLSTX_DATA_WR_IVDSGL_S: c_int = 29;
pub const FW_TLSTX_DATA_WR_IVDSGL_M: c_uint = 0x1;

pub const FW_TLSTX_DATA_WR_KEYSIZE_S: c_int = 24;
pub const FW_TLSTX_DATA_WR_KEYSIZE_M: c_uint = 0x1f;

pub const FW_TLSTX_DATA_WR_NUMIVS_S: c_int = 14;
pub const FW_TLSTX_DATA_WR_NUMIVS_M: c_uint = 0xff;

pub const FW_TLSTX_DATA_WR_EXP_S: c_int = 0;
pub const FW_TLSTX_DATA_WR_EXP_M: c_uint = 0x3fff;

pub const FW_TLSTX_DATA_WR_ADJUSTEDPLEN_S: c_int = 1;

pub const FW_TLSTX_DATA_WR_EXPINPLENMAX_S: c_int = 4;

pub const FW_TLSTX_DATA_WR_PDUSINPLENMAX_S: c_int = 2;

