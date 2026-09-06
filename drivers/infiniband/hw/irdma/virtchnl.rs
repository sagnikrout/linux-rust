//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/virtchnl.h
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


// SPDX-License-Identifier: GPL-2.0 or Linux-OpenIB
// Copyright (c) 2015 - 2024 Intel Corporation

// IRDMA_VCHNL_CHNL_VER_V0 is for legacy hw, no longer supported.
pub const IRDMA_VCHNL_CHNL_VER_V2: c_int = 2;

pub const IRDMA_VCHNL_OP_GET_HMC_FCN_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_GET_HMC_FCN_V1: c_int = 1;
pub const IRDMA_VCHNL_OP_GET_HMC_FCN_V2: c_int = 2;
pub const IRDMA_VCHNL_OP_PUT_HMC_FCN_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_GET_REG_LAYOUT_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_QUEUE_VECTOR_MAP_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_QUEUE_VECTOR_UNMAP_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_ADD_VPORT_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_DEL_VPORT_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_GET_RDMA_CAPS_V0: c_int = 0;
pub const IRDMA_VCHNL_OP_GET_RDMA_CAPS_MIN_SIZE: c_int = 1;
pub const IRDMA_VCHNL_REG_ID_CQPTAIL: c_int = 0;
pub const IRDMA_VCHNL_REG_ID_CQPDB: c_int = 1;
pub const IRDMA_VCHNL_REG_ID_CCQPSTATUS: c_int = 2;
pub const IRDMA_VCHNL_REG_ID_CCQPHIGH: c_int = 3;
pub const IRDMA_VCHNL_REG_ID_CCQPLOW: c_int = 4;
pub const IRDMA_VCHNL_REG_ID_CQARM: c_int = 5;
pub const IRDMA_VCHNL_REG_ID_CQACK: c_int = 6;
pub const IRDMA_VCHNL_REG_ID_AEQALLOC: c_int = 7;
pub const IRDMA_VCHNL_REG_ID_CQPERRCODES: c_int = 8;
pub const IRDMA_VCHNL_REG_ID_WQEALLOC: c_int = 9;
pub const IRDMA_VCHNL_REG_ID_IPCONFIG0: c_int = 10;
pub const IRDMA_VCHNL_REG_ID_DB_ADDR_OFFSET: c_int = 11;
pub const IRDMA_VCHNL_REG_ID_DYN_CTL: c_int = 12;
pub const IRDMA_VCHNL_REG_ID_AEQITRMASK: c_int = 13;
pub const IRDMA_VCHNL_REG_ID_CEQITRMASK: c_int = 14;
pub const IRDMA_VCHNL_REG_INV_ID: c_uint = 0xFFFF;
pub const IRDMA_VCHNL_REG_PAGE_REL: c_uint = 0x8000;
pub const IRDMA_VCHNL_REGFLD_ID_CCQPSTATUS_CQP_OP_ERR: c_int = 2;
pub const IRDMA_VCHNL_REGFLD_ID_CCQPSTATUS_CCQP_DONE: c_int = 5;
pub const IRDMA_VCHNL_REGFLD_ID_CQPSQ_STAG_PDID: c_int = 6;
pub const IRDMA_VCHNL_REGFLD_ID_CQPSQ_CQ_CEQID: c_int = 7;
pub const IRDMA_VCHNL_REGFLD_ID_CQPSQ_CQ_CQID: c_int = 8;
pub const IRDMA_VCHNL_REGFLD_ID_COMMIT_FPM_CQCNT: c_int = 9;
pub const IRDMA_VCHNL_REGFLD_ID_UPESD_HMCN_ID: c_int = 10;
pub const IRDMA_VCHNL_REGFLD_INV_ID: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_vchnl_ops {
    IRDMA_VCHNL_OP_GET_VER = 0,
    IRDMA_VCHNL_OP_GET_HMC_FCN = 1,
    IRDMA_VCHNL_OP_PUT_HMC_FCN = 2,
    IRDMA_VCHNL_OP_GET_REG_LAYOUT = 11,
    IRDMA_VCHNL_OP_GET_RDMA_CAPS = 13,
    IRDMA_VCHNL_OP_QUEUE_VECTOR_MAP = 14,
    IRDMA_VCHNL_OP_QUEUE_VECTOR_UNMAP = 15,
    IRDMA_VCHNL_OP_ADD_VPORT = 16,
    IRDMA_VCHNL_OP_DEL_VPORT = 17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_req_hmc_info {
    pub protocol_used: u8,
    pub disable_qos: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_resp_hmc_info {
    pub hmc_func: u16,
    pub qs_handle: [u16; IRDMA_MAX_USER_PRIORITY],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_qv_info {
    pub v_idx: u32,
    pub ceq_idx: u16,
    pub aeq_idx: u16,
    pub itr_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_qvlist_info {
    pub num_vectors: u32,
    pub qv_info: [irdma_vchnl_qv_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_req_vport_info {
    pub vport_id: u16,
    pub qp1_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_resp_vport_info {
    pub qs_handle: [u16; IRDMA_MAX_USER_PRIORITY],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_op_buf {
    pub op_code: u16,
    pub op_ver: u16,
    pub buf_len: u16,
    pub rsvd: u16,
    pub op_ctx: u64,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_resp_buf {
    pub op_ctx: u64,
    pub buf_len: u16,
    pub op_ret: i16,
    pub rsvd: [u16; 2],
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_rdma_caps {
    pub hw_rev: u8,
    pub cqp_timeout_s: u16,
    pub cqp_def_timeout_s: u16,
    pub max_hw_push_len: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_init_info {
    pub vchnl_wq: *mut workqueue_struct,
    pub hw_rev: irdma_vers,
    pub privileged: bool,
    pub is_pf: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_reg_info {
    pub reg_offset: u32,
    pub field_cnt: u16,
    pub /: *mut *mut u16 reg_id; / High bit of reg_id: bar or page relative,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_reg_field_info {
    pub fld_shift: u8,
    pub fld_bits: u8,
    pub fld_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_req {
    pub vchnl_msg: *mut irdma_vchnl_op_buf,
    pub parm: *mut c_void,
    pub vf_id: u32,
    pub parm_len: u16,
    pub resp_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vchnl_req_init_info {
    pub req_parm: *mut c_void,
    pub resp_parm: *mut c_void,
    pub req_parm_len: u16,
    pub resp_parm_len: u16,
    pub op_code: u16,
    pub op_ver: u16,
    pub __packed: },
    pub irdma_qos: struct,
    pub info): *mut irdma_vchnl_init_info,
    pub ver_res): *mut u32,
    pub dev): *mut int irdma_vchnl_req_get_hmc_fcn(struct irdma_sc_dev,
    pub dev): *mut int irdma_vchnl_req_put_hmc_fcn(struct irdma_sc_dev,
    pub dev): *mut int irdma_vchnl_req_get_caps(struct irdma_sc_dev,
    pub vc_req): *mut irdma_vchnl_req,
    pub dev): *mut int irdma_vchnl_req_get_reg_layout(struct irdma_sc_dev,
    pub v_idx): *mut *mut int irdma_vchnl_req_aeq_vec_map(struct irdma_sc_dev dev, u32,
    pub v_idx): u32,
    pub qos): *mut u32 qp1_id, struct irdma_qos,
    pub qp1_id): u32,
