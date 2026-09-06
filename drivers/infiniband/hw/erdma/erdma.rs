//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/erdma/erdma.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Cheng Xu <chengyou@linux.alibaba.com>
// Kai Shen <kaishen@linux.alibaba.com>
// Copyright (c) 2020-2022, Alibaba Group.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_eq {
    pub qbuf: *mut c_void,
    pub qbuf_dma_addr: dma_addr_t,
    pub lock: spinlock_t,
    pub depth: u32,
    pub ci: u16,
    pub rsvd: u16,
    pub event_num: core::sync::atomic::AtomicI64,
    pub notify_num: core::sync::atomic::AtomicI64,
    pub db: *mut void __iomem,
    pub dbrec: *mut u64,
    pub dbrec_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_sq {
    pub qbuf: *mut c_void,
    pub qbuf_dma_addr: dma_addr_t,
    pub lock: spinlock_t,
    pub depth: u32,
    pub ci: u16,
    pub pi: u16,
    pub wqebb_cnt: u16,
    pub dbrec: *mut u64,
    pub dbrec_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_cq {
    pub qbuf: *mut c_void,
    pub qbuf_dma_addr: dma_addr_t,
    pub lock: spinlock_t,
    pub depth: u32,
    pub ci: u32,
    pub cmdsn: u32,
    pub dbrec: *mut u64,
    pub dbrec_dma: dma_addr_t,
    pub armed_num: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_comp_wait {
    pub wait_event: completion,
    pub cmd_status: u32,
    pub ctx_id: u32,
    pub sq_pi: u16,
    pub comp_status: u8,
    pub rsvd: u8,
    pub comp_data: [u32; 4],
}

pub const ERDMA_CMDQ_TIMEOUT_MS: c_int = 15000;
pub const ERDMA_REG_ACCESS_WAIT_MS: c_int = 20;
pub const ERDMA_WAIT_DEV_DONE_CNT: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq {
    pub comp_wait_bitmap: *mut c_ulong,
    pub wait_pool: *mut erdma_comp_wait,
    pub lock: spinlock_t,
    pub sq: erdma_cmdq_sq,
    pub cq: erdma_cmdq_cq,
    pub eq: erdma_eq,
    pub state: c_ulong,
    pub credits: semaphore,
    pub max_outstandings: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_cc_alg {
    ERDMA_CC_NEWRENO = 0,
    ERDMA_CC_CUBIC,
    ERDMA_CC_HPCC_RTT,
    ERDMA_CC_HPCC_ECN,
    ERDMA_CC_HPCC_INT,
    ERDMA_CC_METHODS_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_devattr {
    pub fw_version: u32,
    pub peer_addr: [c_uchar; ETH_ALEN],
    pub cap_flags: c_ulong,
    pub cc: erdma_cc_alg,
    pub irq_num: u32,
    pub max_qp: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_ord: u32,
    pub max_ird: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_sge_rd: u32,
    pub max_cq: u32,
    pub max_cqe: u32,
    pub max_mr_size: u64,
    pub max_mr: u32,
    pub max_pd: u32,
    pub max_mw: u32,
    pub max_gid: u32,
    pub max_ah: u32,
    pub local_dma_key: u32,
}

pub const ERDMA_IRQNAME_SIZE: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_irq {
    pub name: [c_char; ERDMA_IRQNAME_SIZE],
    pub msix_vector: u32,
    pub affinity_hint_mask: cpumask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_eq_cb {
    pub ready: bool,
    pub /: *mut *mut *mut void dev; / All EQs use this fields to get erdma_dev struct,
    pub irq: erdma_irq,
    pub eq: erdma_eq,
    pub tasklet: tasklet_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_resource_cb {
    pub bitmap: *mut c_ulong,
    pub lock: spinlock_t,
    pub next_alloc_idx: u32,
    pub max_cap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_dev {
    pub ibdev: ib_device,
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub netdev_nb: notifier_block,
    pub reflush_wq: *mut workqueue_struct,
    pub func_bar_addr: resource_size_t,
    pub func_bar_len: resource_size_t,
    pub func_bar: *mut u8 __iomem,
    pub attrs: erdma_devattr,
    pub mtu: u32,
// cmdq and aeq use the same msix vector
    pub comm_irq: erdma_irq,
    pub cmdq: erdma_cmdq,
    pub aeq: erdma_eq,
    pub 1]: erdma_eq_cb ceqs[ERDMA_NUM_MSIX_VEC -,
    pub lock: spinlock_t,
    pub res_cb: [erdma_resource_cb; ERDMA_RES_CNT],
    pub qp_xa: xarray,
    pub cq_xa: xarray,
    pub next_alloc_qpn: u32,
    pub next_alloc_cqn: u32,
    pub num_ctx: core::sync::atomic::AtomicI32,
    pub cep_list: list_head,
    pub db_pool: *mut dma_pool,
    pub resp_pool: *mut dma_pool,
    pub proto: erdma_proto_type,
}

extern "C" {
    pub fn container_of(_arg: ibdev, erdma_dev: struct, _arg: ibdev) -> return;
}
extern "C" {
    pub fn readl(reg: dev->func_bar +) -> return;
}
extern "C" {
    pub fn readq(reg: dev->func_bar +) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: filed_mask, _arg: val) -> return;
}

extern "C" {
    pub fn erdma_cmdq_init(dev: *mut erdma_dev) -> c_int;
}
extern "C" {
    pub fn erdma_finish_cmdq_init(dev: *mut erdma_dev);
}
extern "C" {
    pub fn erdma_cmdq_destroy(dev: *mut erdma_dev);
}
extern "C" {
    pub fn erdma_cmdq_build_reqhdr(hdr: *mut u64, mod: u32, op: u32);
}
extern "C" {
    pub fn erdma_cmdq_completion_handler(cmdq: *mut erdma_cmdq);
}
extern "C" {
    pub fn erdma_ceqs_init(dev: *mut erdma_dev) -> c_int;
}
extern "C" {
    pub fn erdma_ceqs_uninit(dev: *mut erdma_dev);
}
extern "C" {
    pub fn notify_eq(eq: *mut erdma_eq);
}
extern "C" {
    pub fn erdma_aeq_init(dev: *mut erdma_dev) -> c_int;
}
extern "C" {
    pub fn erdma_eq_common_init(dev: *mut erdma_dev, eq: *mut erdma_eq, depth: u32) -> c_int;
}
extern "C" {
    pub fn erdma_eq_destroy(dev: *mut erdma_dev, eq: *mut erdma_eq);
}
extern "C" {
    pub fn erdma_aeq_event_handler(dev: *mut erdma_dev);
}
extern "C" {
    pub fn erdma_ceq_completion_handler(ceq_cb: *mut erdma_eq_cb);
}
