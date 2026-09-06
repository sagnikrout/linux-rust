//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/cxgb4/t4fw_ri_api.h
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
// Copyright (c) 2009-2010 Chelsio, Inc. All rights reserved.
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
pub enum fw_ri_wr_opcode {
    FW_RI_RDMA_WRITE		= 0x0,	/* IETF RDMAP v1.0 ... */
    FW_RI_READ_REQ			= 0x1,
    FW_RI_READ_RESP			= 0x2,
    FW_RI_SEND			= 0x3,
    FW_RI_SEND_WITH_INV		= 0x4,
    FW_RI_SEND_WITH_SE		= 0x5,
    FW_RI_SEND_WITH_SE_INV		= 0x6,
    FW_RI_TERMINATE			= 0x7,
    FW_RI_RDMA_INIT			= 0x8,	/* CHELSIO RI specific ... */
    FW_RI_BIND_MW			= 0x9,
    FW_RI_FAST_REGISTER		= 0xa,
    FW_RI_LOCAL_INV			= 0xb,
    FW_RI_QP_MODIFY			= 0xc,
    FW_RI_BYPASS			= 0xd,
    FW_RI_RECEIVE			= 0xe,

    FW_RI_SGE_EC_CR_RETURN		= 0xf,
    FW_RI_WRITE_IMMEDIATE           = FW_RI_RDMA_INIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_wr_flags {
    FW_RI_COMPLETION_FLAG		= 0x01,
    FW_RI_NOTIFICATION_FLAG		= 0x02,
    FW_RI_SOLICITED_EVENT_FLAG	= 0x04,
    FW_RI_READ_FENCE_FLAG		= 0x08,
    FW_RI_LOCAL_FENCE_FLAG		= 0x10,
    FW_RI_RDMA_READ_INVALIDATE	= 0x20,
    FW_RI_RDMA_WRITE_WITH_IMMEDIATE = 0x40
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_mpa_attrs {
    FW_RI_MPA_RX_MARKER_ENABLE	= 0x01,
    FW_RI_MPA_TX_MARKER_ENABLE	= 0x02,
    FW_RI_MPA_CRC_ENABLE		= 0x04,
    FW_RI_MPA_IETF_ENABLE		= 0x08
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_qp_caps {
    FW_RI_QP_RDMA_READ_ENABLE	= 0x01,
    FW_RI_QP_RDMA_WRITE_ENABLE	= 0x02,
    FW_RI_QP_BIND_ENABLE		= 0x04,
    FW_RI_QP_FAST_REGISTER_ENABLE	= 0x08,
    FW_RI_QP_STAG0_ENABLE		= 0x10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_addr_type {
    FW_RI_ZERO_BASED_TO		= 0x00,
    FW_RI_VA_BASED_TO		= 0x01
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_mem_perms {
    FW_RI_MEM_ACCESS_REM_WRITE	= 0x01,
    FW_RI_MEM_ACCESS_REM_READ	= 0x02,
    FW_RI_MEM_ACCESS_REM		= 0x03,
    FW_RI_MEM_ACCESS_LOCAL_WRITE	= 0x04,
    FW_RI_MEM_ACCESS_LOCAL_READ	= 0x08,
    FW_RI_MEM_ACCESS_LOCAL		= 0x0C
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_stag_type {
    FW_RI_STAG_NSMR			= 0x00,
    FW_RI_STAG_SMR			= 0x01,
    FW_RI_STAG_MW			= 0x02,
    FW_RI_STAG_MW_RELAXED		= 0x03
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_data_op {
    FW_RI_DATA_IMMD			= 0x81,
    FW_RI_DATA_DSGL			= 0x82,
    FW_RI_DATA_ISGL			= 0x83
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_sgl_depth {
    FW_RI_SGL_DEPTH_MAX_SQ		= 16,
    FW_RI_SGL_DEPTH_MAX_RQ		= 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_dsge_pair {
    pub len: [__be32; 2],
    pub addr: [__be64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_dsgl {
    pub op: __u8,
    pub r1: __u8,
    pub nsge: __be16,
    pub len0: __be32,
    pub addr0: __be64,
    pub sge: [fw_ri_dsge_pair; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_sge {
    pub stag: __be32,
    pub len: __be32,
    pub to: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_isgl {
    pub op: __u8,
    pub r1: __u8,
    pub nsge: __be16,
    pub r2: __be32,
    pub sge: [fw_ri_sge; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_immd {
    pub op: __u8,
    pub r1: __u8,
    pub r2: __be16,
    pub immdlen: __be32,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_tpte {
    pub valid_to_pdid: __be32,
    pub locread_to_qpid: __be32,
    pub nosnoop_pbladdr: __be32,
    pub len_lo: __be32,
    pub va_hi: __be32,
    pub va_lo_fbo: __be32,
    pub dca_mwbcnt_pstag: __be32,
    pub len_hi: __be32,
}

pub const FW_RI_TPTE_VALID_S: c_int = 31;
pub const FW_RI_TPTE_VALID_M: c_uint = 0x1;

pub const FW_RI_TPTE_STAGKEY_S: c_int = 23;
pub const FW_RI_TPTE_STAGKEY_M: c_uint = 0xff;

pub const FW_RI_TPTE_STAGSTATE_S: c_int = 22;
pub const FW_RI_TPTE_STAGSTATE_M: c_uint = 0x1;

pub const FW_RI_TPTE_STAGTYPE_S: c_int = 20;
pub const FW_RI_TPTE_STAGTYPE_M: c_uint = 0x3;

pub const FW_RI_TPTE_PDID_S: c_int = 0;
pub const FW_RI_TPTE_PDID_M: c_uint = 0xfffff;

pub const FW_RI_TPTE_PERM_S: c_int = 28;
pub const FW_RI_TPTE_PERM_M: c_uint = 0xf;

pub const FW_RI_TPTE_REMINVDIS_S: c_int = 27;
pub const FW_RI_TPTE_REMINVDIS_M: c_uint = 0x1;

pub const FW_RI_TPTE_ADDRTYPE_S: c_int = 26;
pub const FW_RI_TPTE_ADDRTYPE_M: c_int = 1;

pub const FW_RI_TPTE_MWBINDEN_S: c_int = 25;
pub const FW_RI_TPTE_MWBINDEN_M: c_uint = 0x1;

pub const FW_RI_TPTE_PS_S: c_int = 20;
pub const FW_RI_TPTE_PS_M: c_uint = 0x1f;

pub const FW_RI_TPTE_QPID_S: c_int = 0;
pub const FW_RI_TPTE_QPID_M: c_uint = 0xfffff;

pub const FW_RI_TPTE_NOSNOOP_S: c_int = 30;
pub const FW_RI_TPTE_NOSNOOP_M: c_uint = 0x1;

pub const FW_RI_TPTE_PBLADDR_S: c_int = 0;
pub const FW_RI_TPTE_PBLADDR_M: c_uint = 0x1fffffff;

pub const FW_RI_TPTE_DCA_S: c_int = 24;
pub const FW_RI_TPTE_DCA_M: c_uint = 0x1f;

pub const FW_RI_TPTE_MWBCNT_PSTAG_S: c_int = 0;
pub const FW_RI_TPTE_MWBCNT_PSTAG_M: c_uint = 0xffffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_res_type {
    FW_RI_RES_TYPE_SQ,
    FW_RI_RES_TYPE_RQ,
    FW_RI_RES_TYPE_CQ,
    FW_RI_RES_TYPE_SRQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_res_op {
    FW_RI_RES_OP_WRITE,
    FW_RI_RES_OP_RESET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_res {
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ri_restype {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_res_sqrq {
    pub restype: __u8,
    pub op: __u8,
    pub r3: __be16,
    pub eqid: __be32,
    pub r4: [__be32; 2],
    pub fetchszm_to_iqid: __be32,
    pub dcaen_to_eqsize: __be32,
    pub eqaddr: __be64,
    pub sqrq: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_res_cq {
    pub restype: __u8,
    pub op: __u8,
    pub r3: __be16,
    pub iqid: __be32,
    pub r4: [__be32; 2],
    pub iqandst_to_iqandstindex: __be32,
    pub iqdroprss_to_iqesize: __be16,
    pub iqsize: __be16,
    pub iqaddr: __be64,
    pub iqns_iqro: __be32,
    pub r6_lo: __be32,
    pub r7: __be64,
    pub cq: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_res_srq {
    pub restype: __u8,
    pub op: __u8,
    pub r3: __be16,
    pub eqid: __be32,
    pub r4: [__be32; 2],
    pub fetchszm_to_iqid: __be32,
    pub dcaen_to_eqsize: __be32,
    pub eqaddr: __be64,
    pub srqid: __be32,
    pub pdid: __be32,
    pub hwsrqsize: __be32,
    pub hwsrqaddr: __be32,
    pub srq: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_res_wr {
    pub op_nres: __be32,
    pub len16_pkd: __be32,
    pub cookie: __u64,
    pub res: [fw_ri_res; ],
}

pub const FW_RI_RES_WR_NRES_S: c_int = 0;
pub const FW_RI_RES_WR_NRES_M: c_uint = 0xff;

pub const FW_RI_RES_WR_FETCHSZM_S: c_int = 26;
pub const FW_RI_RES_WR_FETCHSZM_M: c_uint = 0x1;

pub const FW_RI_RES_WR_STATUSPGNS_S: c_int = 25;
pub const FW_RI_RES_WR_STATUSPGNS_M: c_uint = 0x1;

pub const FW_RI_RES_WR_STATUSPGRO_S: c_int = 24;
pub const FW_RI_RES_WR_STATUSPGRO_M: c_uint = 0x1;

pub const FW_RI_RES_WR_FETCHNS_S: c_int = 23;
pub const FW_RI_RES_WR_FETCHNS_M: c_uint = 0x1;

pub const FW_RI_RES_WR_FETCHRO_S: c_int = 22;
pub const FW_RI_RES_WR_FETCHRO_M: c_uint = 0x1;

pub const FW_RI_RES_WR_HOSTFCMODE_S: c_int = 20;
pub const FW_RI_RES_WR_HOSTFCMODE_M: c_uint = 0x3;

pub const FW_RI_RES_WR_CPRIO_S: c_int = 19;
pub const FW_RI_RES_WR_CPRIO_M: c_uint = 0x1;

pub const FW_RI_RES_WR_ONCHIP_S: c_int = 18;
pub const FW_RI_RES_WR_ONCHIP_M: c_uint = 0x1;

pub const FW_RI_RES_WR_PCIECHN_S: c_int = 16;
pub const FW_RI_RES_WR_PCIECHN_M: c_uint = 0x3;

pub const FW_RI_RES_WR_IQID_S: c_int = 0;
pub const FW_RI_RES_WR_IQID_M: c_uint = 0xffff;

pub const FW_RI_RES_WR_DCAEN_S: c_int = 31;
pub const FW_RI_RES_WR_DCAEN_M: c_uint = 0x1;

pub const FW_RI_RES_WR_DCACPU_S: c_int = 26;
pub const FW_RI_RES_WR_DCACPU_M: c_uint = 0x1f;

pub const FW_RI_RES_WR_FBMIN_S: c_int = 23;
pub const FW_RI_RES_WR_FBMIN_M: c_uint = 0x7;

pub const FW_RI_RES_WR_FBMAX_S: c_int = 20;
pub const FW_RI_RES_WR_FBMAX_M: c_uint = 0x7;

pub const FW_RI_RES_WR_CIDXFTHRESHO_S: c_int = 19;
pub const FW_RI_RES_WR_CIDXFTHRESHO_M: c_uint = 0x1;

pub const FW_RI_RES_WR_CIDXFTHRESH_S: c_int = 16;
pub const FW_RI_RES_WR_CIDXFTHRESH_M: c_uint = 0x7;

pub const FW_RI_RES_WR_EQSIZE_S: c_int = 0;
pub const FW_RI_RES_WR_EQSIZE_M: c_uint = 0xffff;

pub const FW_RI_RES_WR_IQANDST_S: c_int = 15;
pub const FW_RI_RES_WR_IQANDST_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQANUS_S: c_int = 14;
pub const FW_RI_RES_WR_IQANUS_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQANUD_S: c_int = 12;
pub const FW_RI_RES_WR_IQANUD_M: c_uint = 0x3;

pub const FW_RI_RES_WR_IQANDSTINDEX_S: c_int = 0;
pub const FW_RI_RES_WR_IQANDSTINDEX_M: c_uint = 0xfff;

pub const FW_RI_RES_WR_IQDROPRSS_S: c_int = 15;
pub const FW_RI_RES_WR_IQDROPRSS_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQGTSMODE_S: c_int = 14;
pub const FW_RI_RES_WR_IQGTSMODE_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQPCIECH_S: c_int = 12;
pub const FW_RI_RES_WR_IQPCIECH_M: c_uint = 0x3;

pub const FW_RI_RES_WR_IQDCAEN_S: c_int = 11;
pub const FW_RI_RES_WR_IQDCAEN_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQDCACPU_S: c_int = 6;
pub const FW_RI_RES_WR_IQDCACPU_M: c_uint = 0x1f;

pub const FW_RI_RES_WR_IQINTCNTTHRESH_S: c_int = 4;
pub const FW_RI_RES_WR_IQINTCNTTHRESH_M: c_uint = 0x3;

pub const FW_RI_RES_WR_IQO_S: c_int = 3;
pub const FW_RI_RES_WR_IQO_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQCPRIO_S: c_int = 2;
pub const FW_RI_RES_WR_IQCPRIO_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQESIZE_S: c_int = 0;
pub const FW_RI_RES_WR_IQESIZE_M: c_uint = 0x3;

pub const FW_RI_RES_WR_IQNS_S: c_int = 31;
pub const FW_RI_RES_WR_IQNS_M: c_uint = 0x1;

pub const FW_RI_RES_WR_IQRO_S: c_int = 30;
pub const FW_RI_RES_WR_IQRO_M: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_rdma_write_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
//
// Use union for immediate data to be consistent with stack's 32 bit
// data and iWARP spec's 64 bit data.
//
    pub imm_data32: __be32,
    pub reserved: u32,
    pub ib_imm_data: },
    pub imm_data64: __be64,
    pub iw_imm_data: },
    pub plen: __be32,
    pub stag_sink: __be32,
    pub to_sink: __be64,
    pub immd_src): DECLARE_FLEX_ARRAY(struct fw_ri_immd,,
    pub isgl_src): DECLARE_FLEX_ARRAY(struct fw_ri_isgl,,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_send_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub sendop_pkd: __be32,
    pub stag_inv: __be32,
    pub plen: __be32,
    pub r3: __be32,
    pub r4: __be64,
    pub immd_src): DECLARE_FLEX_ARRAY(struct fw_ri_immd,,
    pub isgl_src): DECLARE_FLEX_ARRAY(struct fw_ri_isgl,,
    pub u: },
}

pub const FW_RI_SEND_WR_SENDOP_S: c_int = 0;
pub const FW_RI_SEND_WR_SENDOP_M: c_uint = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_rdma_write_cmpl_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub r2: __u8,
    pub flags_send: __u8,
    pub wrid_send: __u16,
    pub stag_inv: __be32,
    pub plen: __be32,
    pub stag_sink: __be32,
    pub to_sink: __be64,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ri_cmpl {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_immd_cmpl {
    pub op: __u8,
    pub r1: [__u8; 6],
    pub immdlen: __u8,
    pub data: [__u8; 16],
    pub immd_src: },
    pub isgl_src: fw_ri_isgl,
    pub u_cmpl: },
    pub r3: __be64,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ri_write {
    pub immd_src): DECLARE_FLEX_ARRAY(struct fw_ri_immd,,
    pub isgl_src): DECLARE_FLEX_ARRAY(struct fw_ri_isgl,,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_rdma_read_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub r2: __be64,
    pub stag_sink: __be32,
    pub to_sink_hi: __be32,
    pub to_sink_lo: __be32,
    pub plen: __be32,
    pub stag_src: __be32,
    pub to_src_hi: __be32,
    pub to_src_lo: __be32,
    pub r5: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_recv_wr {
    pub opcode: __u8,
    pub r1: __u8,
    pub wrid: __u16,
    pub r2: [__u8; 3],
    pub len16: __u8,
    pub isgl: fw_ri_isgl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_bind_mw_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub qpbinde_to_dcacpu: __u8,
    pub pgsz_shift: __u8,
    pub addr_type: __u8,
    pub mem_perms: __u8,
    pub stag_mr: __be32,
    pub stag_mw: __be32,
    pub r3: __be32,
    pub len_mw: __be64,
    pub va_fbo: __be64,
    pub r4: __be64,
}

pub const FW_RI_BIND_MW_WR_QPBINDE_S: c_int = 6;
pub const FW_RI_BIND_MW_WR_QPBINDE_M: c_uint = 0x1;

pub const FW_RI_BIND_MW_WR_NS_S: c_int = 5;
pub const FW_RI_BIND_MW_WR_NS_M: c_uint = 0x1;

pub const FW_RI_BIND_MW_WR_DCACPU_S: c_int = 0;
pub const FW_RI_BIND_MW_WR_DCACPU_M: c_uint = 0x1f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_fr_nsmr_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub qpbinde_to_dcacpu: __u8,
    pub pgsz_shift: __u8,
    pub addr_type: __u8,
    pub mem_perms: __u8,
    pub stag: __be32,
    pub len_hi: __be32,
    pub len_lo: __be32,
    pub va_hi: __be32,
    pub va_lo_fbo: __be32,
}

pub const FW_RI_FR_NSMR_WR_QPBINDE_S: c_int = 6;
pub const FW_RI_FR_NSMR_WR_QPBINDE_M: c_uint = 0x1;

pub const FW_RI_FR_NSMR_WR_NS_S: c_int = 5;
pub const FW_RI_FR_NSMR_WR_NS_M: c_uint = 0x1;

pub const FW_RI_FR_NSMR_WR_DCACPU_S: c_int = 0;
pub const FW_RI_FR_NSMR_WR_DCACPU_M: c_uint = 0x1f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_fr_nsmr_tpte_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub r2: __be32,
    pub stag: __be32,
    pub tpte: fw_ri_tpte,
    pub pbl: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_inv_lstag_wr {
    pub opcode: __u8,
    pub flags: __u8,
    pub wrid: __u16,
    pub r1: [__u8; 3],
    pub len16: __u8,
    pub r2: __be32,
    pub stag_inv: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_type {
    FW_RI_TYPE_INIT,
    FW_RI_TYPE_FINI,
    FW_RI_TYPE_TERMINATE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_init_p2ptype {
    FW_RI_INIT_P2PTYPE_RDMA_WRITE		= FW_RI_RDMA_WRITE,
    FW_RI_INIT_P2PTYPE_READ_REQ		= FW_RI_READ_REQ,
    FW_RI_INIT_P2PTYPE_SEND			= FW_RI_SEND,
    FW_RI_INIT_P2PTYPE_SEND_WITH_INV	= FW_RI_SEND_WITH_INV,
    FW_RI_INIT_P2PTYPE_SEND_WITH_SE		= FW_RI_SEND_WITH_SE,
    FW_RI_INIT_P2PTYPE_SEND_WITH_SE_INV	= FW_RI_SEND_WITH_SE_INV,
    FW_RI_INIT_P2PTYPE_DISABLED		= 0xf,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_ri_init_rqeqid_srq {
    FW_RI_INIT_RQEQID_SRQ			= 1 << 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_wr {
    pub op_compl: __be32,
    pub flowid_len16: __be32,
    pub cookie: __u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ri {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_init {
    pub type: __u8,
    pub mpareqbit_p2ptype: __u8,
    pub r4: [__u8; 2],
    pub mpa_attrs: __u8,
    pub qp_caps: __u8,
    pub nrqe: __be16,
    pub pdid: __be32,
    pub qpid: __be32,
    pub sq_eqid: __be32,
    pub rq_eqid: __be32,
    pub scqid: __be32,
    pub rcqid: __be32,
    pub ord_max: __be32,
    pub ird_max: __be32,
    pub iss: __be32,
    pub irs: __be32,
    pub hwrqsize: __be32,
    pub hwrqaddr: __be32,
    pub r5: __be64,
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_ri_init_p2p {
    pub write: fw_ri_rdma_write_wr,
    pub read: fw_ri_rdma_read_wr,
    pub send: fw_ri_send_wr,
    pub u: },
    pub init: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_fini {
    pub type: __u8,
    pub r3: [__u8; 7],
    pub r4: __be64,
    pub fini: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ri_terminate {
    pub type: __u8,
    pub r3: [__u8; 3],
    pub immdlen: __be32,
    pub termmsg: [__u8; 40],
    pub terminate: },
    pub u: },
}

pub const FW_RI_WR_MPAREQBIT_S: c_int = 7;
pub const FW_RI_WR_MPAREQBIT_M: c_uint = 0x1;

pub const FW_RI_WR_P2PTYPE_S: c_int = 0;
pub const FW_RI_WR_P2PTYPE_M: c_uint = 0xf;

