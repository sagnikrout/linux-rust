//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/hns/hns_roce_restrack.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
// Copyright (c) 2019 Hisilicon Limited.

#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_cq_entry(msg: *mut sk_buff, ib_cq: *mut ib_cq) -> c_int {
    int hns_roce_fill_res_cq_entry(struct sk_buff *msg, struct ib_cq *ib_cq)
    {
    struct hns_roce_cq *hr_cq = to_hr_cq(ib_cq);
    struct nlattr *table_attr;
    table_attr = nla_nest_start(msg, RDMA_NLDEV_ATTR_DRIVER);
    if (!table_attr)
    return -EMSGSIZE;
    if (rdma_nl_put_driver_u32(msg, "cq_depth", hr_cq.cq_depth))
    goto err;
    if (rdma_nl_put_driver_u32(msg, "cons_index", hr_cq.cons_index))
    goto err;
    if (rdma_nl_put_driver_u32(msg, "cqe_size", hr_cq.cqe_size))
    goto err;
    if (rdma_nl_put_driver_u32(msg, "arm_sn", hr_cq.arm_sn))
    goto err;
    nla_nest_end(msg, table_attr);
    return 0;
    err:
    nla_nest_cancel(msg, table_attr);
    return -EMSGSIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_cq_entry_raw(msg: *mut sk_buff, ib_cq: *mut ib_cq) -> c_int {
    int hns_roce_fill_res_cq_entry_raw(struct sk_buff *msg, struct ib_cq *ib_cq)
    {
    struct hns_roce_dev *hr_dev = to_hr_dev(ib_cq.device);
    struct hns_roce_cq *hr_cq = to_hr_cq(ib_cq);
    struct hns_roce_v2_cq_context context;
    int ret;
    if (!hr_dev.hw.query_cqc)
    return -EINVAL;
    ret = hr_dev.hw.query_cqc(hr_dev, hr_cq.cqn, &context);
    if (ret)
    return ret;
    ret = nla_put(msg, RDMA_NLDEV_ATTR_RES_RAW, sizeof(context), &context);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_qp_entry(msg: *mut sk_buff, ib_qp: *mut ib_qp) -> c_int {
    int hns_roce_fill_res_qp_entry(struct sk_buff *msg, struct ib_qp *ib_qp)
    {
    struct hns_roce_qp *hr_qp = to_hr_qp(ib_qp);
    struct nlattr *table_attr;
    table_attr = nla_nest_start(msg, RDMA_NLDEV_ATTR_DRIVER);
    if (!table_attr)
    return -EMSGSIZE;
    if (rdma_nl_put_driver_u32_hex(msg, "sq_wqe_cnt", hr_qp.sq.wqe_cnt))
    goto err;
    if (rdma_nl_put_driver_u32_hex(msg, "sq_max_gs", hr_qp.sq.max_gs))
    goto err;
    if (rdma_nl_put_driver_u32_hex(msg, "rq_wqe_cnt", hr_qp.rq.wqe_cnt))
    goto err;
    if (rdma_nl_put_driver_u32_hex(msg, "rq_max_gs", hr_qp.rq.max_gs))
    goto err;
    if (rdma_nl_put_driver_u32_hex(msg, "ext_sge_sge_cnt", hr_qp.sge.sge_cnt))
    goto err;
    nla_nest_end(msg, table_attr);
    return 0;
    err:
    nla_nest_cancel(msg, table_attr);
    return -EMSGSIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_qp_entry_raw(msg: *mut sk_buff, ib_qp: *mut ib_qp) -> c_int {
    int hns_roce_fill_res_qp_entry_raw(struct sk_buff *msg, struct ib_qp *ib_qp)
    {
    struct hns_roce_dev *hr_dev = to_hr_dev(ib_qp.device);
    struct hns_roce_qp *hr_qp = to_hr_qp(ib_qp);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_full_qp_ctx {
    pub qpc: hns_roce_v2_qp_context,
    pub sccc: hns_roce_v2_scc_context,
    pub {}: } context =,
    pub hr_qp->qpn: u32 sccn =,
    pub ret: c_int,
    if (!hr_dev.hw.query_qpc)
    pub -EINVAL: return,
    pub &context.qpc): ret = hr_dev->hw->query_qpc(hr_dev, hr_qp->qpn,,
    if (ret)
    pub ret: return,
// If SCC is disabled or the query fails, the queried SCCC will
// be all 0.
//
    if (!(hr_dev.caps.flags & HNS_ROCE_CAP_FLAG_QP_FLOW_CTRL) ||
    !hr_dev.hw.query_sccc)
    pub out: goto,
    if (hr_qp.cong_type == CONG_TYPE_DIP) {
    if (!hr_qp.dip)
    pub out: goto,
    pub hr_qp->dip->dip_idx: sccn =,
    }
    pub &context.sccc): ret = hr_dev->hw->query_sccc(hr_dev, sccn,,
    if (ret)
    ibdev_warn_ratelimited(&hr_dev.ib_dev,
    "failed to query SCCC, ret = %d.\n",
    out:
    pub &context): ret = nla_put(msg, RDMA_NLDEV_ATTR_RES_RAW, sizeof(context),,
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_mr_entry(msg: *mut sk_buff, ib_mr: *mut ib_mr) -> c_int {
    int hns_roce_fill_res_mr_entry(struct sk_buff *msg, struct ib_mr *ib_mr)
    {
    pub to_hr_mr(ib_mr): *mut *mut hns_roce_mr hr_mr =,
    pub table_attr: *mut nlattr,
    pub RDMA_NLDEV_ATTR_DRIVER): table_attr = nla_nest_start(msg,,
    if (!table_attr)
    pub -EMSGSIZE: return,
    if (rdma_nl_put_driver_u32_hex(msg, "pbl_hop_num", hr_mr.pbl_hop_num))
    pub err: goto,
    if (rdma_nl_put_driver_u32_hex(msg, "ba_pg_shift",
    hr_mr.pbl_mtr.hem_cfg.ba_pg_shift))
    pub err: goto,
    if (rdma_nl_put_driver_u32_hex(msg, "buf_pg_shift",
    hr_mr.pbl_mtr.hem_cfg.buf_pg_shift))
    pub err: goto,
    pub table_attr): nla_nest_end(msg,,
    pub 0: return,
    err:
    pub table_attr): nla_nest_cancel(msg,,
    pub -EMSGSIZE: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_mr_entry_raw(msg: *mut sk_buff, ib_mr: *mut ib_mr) -> c_int {
    int hns_roce_fill_res_mr_entry_raw(struct sk_buff *msg, struct ib_mr *ib_mr)
    {
    pub to_hr_dev(ib_mr->device): *mut *mut hns_roce_dev hr_dev =,
    pub to_hr_mr(ib_mr): *mut *mut hns_roce_mr hr_mr =,
    pub context: hns_roce_v2_mpt_entry,
    pub ret: c_int,
    if (!hr_dev.hw.query_mpt)
    pub -EINVAL: return,
    pub &context): ret = hr_dev->hw->query_mpt(hr_dev, hr_mr->key,,
    if (ret)
    pub ret: return,
    pub &context): ret = nla_put(msg, RDMA_NLDEV_ATTR_RES_RAW, sizeof(context),,
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_srq_entry(msg: *mut sk_buff, ib_srq: *mut ib_srq) -> c_int {
    int hns_roce_fill_res_srq_entry(struct sk_buff *msg, struct ib_srq *ib_srq)
    {
    pub to_hr_srq(ib_srq): *mut *mut hns_roce_srq hr_srq =,
    pub table_attr: *mut nlattr,
    pub RDMA_NLDEV_ATTR_DRIVER): table_attr = nla_nest_start(msg,,
    if (!table_attr)
    pub -EMSGSIZE: return,
    if (rdma_nl_put_driver_u32_hex(msg, "srqn", hr_srq.srqn))
    pub err: goto,
    if (rdma_nl_put_driver_u32_hex(msg, "wqe_cnt", hr_srq.wqe_cnt))
    pub err: goto,
    if (rdma_nl_put_driver_u32_hex(msg, "max_gs", hr_srq.max_gs))
    pub err: goto,
    if (rdma_nl_put_driver_u32_hex(msg, "xrcdn", hr_srq.xrcdn))
    pub err: goto,
    pub table_attr): nla_nest_end(msg,,
    pub 0: return,
    err:
    pub table_attr): nla_nest_cancel(msg,,
    pub -EMSGSIZE: return,
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_fill_res_srq_entry_raw(msg: *mut sk_buff, ib_srq: *mut ib_srq) -> c_int {
    int hns_roce_fill_res_srq_entry_raw(struct sk_buff *msg, struct ib_srq *ib_srq)
    {
    pub to_hr_dev(ib_srq->device): *mut *mut hns_roce_dev hr_dev =,
    pub to_hr_srq(ib_srq): *mut *mut hns_roce_srq hr_srq =,
    pub context: hns_roce_srq_context,
    pub ret: c_int,
    if (!hr_dev.hw.query_srqc)
    pub -EINVAL: return,
    pub &context): ret = hr_dev->hw->query_srqc(hr_dev, hr_srq->srqn,,
    if (ret)
    pub ret: return,
    pub &context): ret = nla_put(msg, RDMA_NLDEV_ATTR_RES_RAW, sizeof(context),,
    pub ret: return,
    }
