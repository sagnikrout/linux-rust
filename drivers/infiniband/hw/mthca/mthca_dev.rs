//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mthca/mthca_dev.h
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
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
// Copyright (c) 2004 Voltaire, Inc. All rights reserved.
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

// Arbel FW gives us these, but we need them for Tavor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_cmd {
    pub pool: *mut dma_pool,
    pub hcr_mutex: mutex,
    pub poll_sem: semaphore,
    pub event_sem: semaphore,
    pub max_cmds: c_int,
    pub context_lock: spinlock_t,
    pub free_head: c_int,
    pub context: *mut mthca_cmd_context,
    pub token_mask: u16,
    pub flags: u32,
    pub dbell_map: *mut void __iomem,
    pub dbell_offsets: [u16; MTHCA_CMD_NUM_DBELL_DWORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_limits {
    pub num_ports: c_int,
    pub vl_cap: c_int,
    pub mtu_cap: c_int,
    pub gid_table_len: c_int,
    pub pkey_table_len: c_int,
    pub local_ca_ack_delay: c_int,
    pub num_uars: c_int,
    pub max_sg: c_int,
    pub num_qps: c_int,
    pub max_wqes: c_int,
    pub max_desc_sz: c_int,
    pub max_qp_init_rdma: c_int,
    pub reserved_qps: c_int,
    pub num_srqs: c_int,
    pub max_srq_wqes: c_int,
    pub max_srq_sge: c_int,
    pub reserved_srqs: c_int,
    pub num_eecs: c_int,
    pub reserved_eecs: c_int,
    pub num_cqs: c_int,
    pub max_cqes: c_int,
    pub reserved_cqs: c_int,
    pub num_eqs: c_int,
    pub reserved_eqs: c_int,
    pub num_mpts: c_int,
    pub num_mtt_segs: c_int,
    pub mtt_seg_size: c_int,
    pub fmr_reserved_mtts: c_int,
    pub reserved_mtts: c_int,
    pub reserved_mrws: c_int,
    pub reserved_uars: c_int,
    pub num_mgms: c_int,
    pub num_amgms: c_int,
    pub reserved_mcgs: c_int,
    pub num_pds: c_int,
    pub reserved_pds: c_int,
    pub page_size_cap: u32,
    pub flags: u32,
    pub stat_rate_support: u16,
    pub port_width_cap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_alloc {
    pub last: u32,
    pub top: u32,
    pub max: u32,
    pub mask: u32,
    pub lock: spinlock_t,
    pub table: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_array {
    pub page: *mut c_void,
    pub used: c_int,
    pub page_list: *mut },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_uar_table {
    pub alloc: mthca_alloc,
    pub uarc_base: u64,
    pub uarc_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_pd_table {
    pub alloc: mthca_alloc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_buddy {
    pub bits: *mut c_ulong,
    pub num_free: *mut c_int,
    pub max_order: c_int,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mr_table {
    pub mpt_alloc: mthca_alloc,
    pub mtt_buddy: mthca_buddy,
    pub fmr_mtt_buddy: *mut mthca_buddy,
    pub mtt_base: u64,
    pub mpt_base: u64,
    pub mtt_table: *mut mthca_icm_table,
    pub mpt_table: *mut mthca_icm_table,
    pub mpt_base: *mut void __iomem,
    pub mtt_base: *mut void __iomem,
    pub mtt_buddy: mthca_buddy,
    pub tavor_fmr: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_eq_table {
    pub alloc: mthca_alloc,
    pub clr_int: *mut void __iomem,
    pub clr_mask: u32,
    pub arm_mask: u32,
    pub eq: [mthca_eq; MTHCA_NUM_EQ],
    pub icm_virt: u64,
    pub icm_page: *mut page,
    pub icm_dma: dma_addr_t,
    pub have_irq: c_int,
    pub inta_pin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_cq_table {
    pub alloc: mthca_alloc,
    pub lock: spinlock_t,
    pub cq: mthca_array,
    pub table: *mut mthca_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_srq_table {
    pub alloc: mthca_alloc,
    pub lock: spinlock_t,
    pub srq: mthca_array,
    pub table: *mut mthca_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_qp_table {
    pub alloc: mthca_alloc,
    pub rdb_base: u32,
    pub rdb_shift: c_int,
    pub sqp_start: c_int,
    pub lock: spinlock_t,
    pub qp: mthca_array,
    pub qp_table: *mut mthca_icm_table,
    pub eqp_table: *mut mthca_icm_table,
    pub rdb_table: *mut mthca_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_av_table {
    pub pool: *mut dma_pool,
    pub num_ddr_avs: c_int,
    pub ddr_av_base: u64,
    pub av_map: *mut void __iomem,
    pub alloc: mthca_alloc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mcg_table {
    pub mutex: mutex,
    pub alloc: mthca_alloc,
    pub table: *mut mthca_icm_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_catas_err {
    pub addr: u64,
    pub map: *mut u32 __iomem,
    pub size: u32,
    pub timer: timer_list,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_dev {
    pub ib_dev: ib_device,
    pub pdev: *mut pci_dev,
    pub hca_type: c_int,
    pub mthca_flags: c_ulong,
    pub device_cap_flags: c_ulong,
    pub rev_id: u32,
    pub board_id: [c_char; MTHCA_BOARD_ID_LEN],
// firmware info
    pub fw_ver: u64,
    pub fw_start: u64,
    pub fw_end: u64,
    pub tavor: },
    pub clr_int_base: u64,
    pub eq_arm_base: u64,
    pub eq_set_ci_base: u64,
    pub fw_icm: *mut mthca_icm,
    pub aux_icm: *mut mthca_icm,
    pub fw_pages: u16,
    pub arbel: },
    pub fw: },
    pub ddr_start: u64,
    pub ddr_end: u64,
    pub cap_mask_mutex: mutex,
    pub hcr: *mut void __iomem,
    pub kar: *mut void __iomem,
    pub clr_base: *mut void __iomem,
    pub ecr_base: *mut void __iomem,
    pub tavor: },
    pub eq_arm: *mut void __iomem,
    pub eq_set_ci_base: *mut void __iomem,
    pub arbel: },
    pub eq_regs: },
    pub cmd: mthca_cmd,
    pub limits: mthca_limits,
    pub uar_table: mthca_uar_table,
    pub pd_table: mthca_pd_table,
    pub mr_table: mthca_mr_table,
    pub eq_table: mthca_eq_table,
    pub cq_table: mthca_cq_table,
    pub srq_table: mthca_srq_table,
    pub qp_table: mthca_qp_table,
    pub av_table: mthca_av_table,
    pub mcg_table: mthca_mcg_table,
    pub catas_err: mthca_catas_err,
    pub driver_uar: mthca_uar,
    pub db_tab: *mut mthca_db_table,
    pub driver_pd: mthca_pd,
    pub driver_mr: mthca_mr,
    pub send_agent: [*mut ib_mad_agent; MTHCA_MAX_PORTS][2],
    pub sm_ah: [*mut ib_ah; MTHCA_MAX_PORTS],
    pub sm_lock: spinlock_t,
    pub rate: [u8; MTHCA_MAX_PORTS],
    pub active: bool,
}

extern "C" {
    pub fn __buggy_use_of_MTHCA_GET();
}
extern "C" {
    pub fn __buggy_use_of_MTHCA_PUT();
}

extern "C" {
    pub fn mthca_reset(mdev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_alloc(alloc: *mut mthca_alloc) -> u32;
}
extern "C" {
    pub fn mthca_free(alloc: *mut mthca_alloc, obj: u32);
}
extern "C" {
    pub fn mthca_alloc_cleanup(alloc: *mut mthca_alloc);
}
extern "C" {
    pub fn mthca_array_set(array: *mut mthca_array, index: c_int, value: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mthca_array_clear(array: *mut mthca_array, index: c_int);
}
extern "C" {
    pub fn mthca_array_init(array: *mut mthca_array, nent: c_int) -> c_int;
}
extern "C" {
    pub fn mthca_array_cleanup(array: *mut mthca_array, nent: c_int);
}
extern "C" {
    pub fn mthca_init_uar_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_pd_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_mr_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_eq_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_cq_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_srq_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_qp_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_av_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_init_mcg_table(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_cleanup_uar_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_pd_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_mr_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_eq_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_cq_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_srq_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_qp_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_av_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_cleanup_mcg_table(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_register_device(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_unregister_device(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_start_catas_poll(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_stop_catas_poll(dev: *mut mthca_dev);
}
extern "C" {
    pub fn __mthca_restart_one(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn mthca_catas_init() -> c_int;
}
extern "C" {
    pub fn mthca_catas_cleanup();
}
extern "C" {
    pub fn mthca_uar_alloc(dev: *mut mthca_dev, uar: *mut mthca_uar) -> c_int;
}
extern "C" {
    pub fn mthca_uar_free(dev: *mut mthca_dev, uar: *mut mthca_uar);
}
extern "C" {
    pub fn mthca_pd_alloc(dev: *mut mthca_dev, privileged: c_int, pd: *mut mthca_pd) -> c_int;
}
extern "C" {
    pub fn mthca_pd_free(dev: *mut mthca_dev, pd: *mut mthca_pd);
}
extern "C" {
    pub fn mthca_write_mtt_size(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_free_mtt(dev: *mut mthca_dev, mtt: *mut mthca_mtt);
}
extern "C" {
    pub fn mthca_free_mr(dev: *mut mthca_dev, mr: *mut mthca_mr);
}
extern "C" {
    pub fn mthca_map_eq_icm(dev: *mut mthca_dev, icm_virt: u64) -> c_int;
}
extern "C" {
    pub fn mthca_unmap_eq_icm(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_tavor_arm_cq(cq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn mthca_arbel_arm_cq(cq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn mthca_cq_completion(dev: *mut mthca_dev, cqn: u32);
}
extern "C" {
    pub fn mthca_cq_resize_copy_cqes(cq: *mut mthca_cq);
}
extern "C" {
    pub fn mthca_alloc_cq_buf(dev: *mut mthca_dev, buf: *mut mthca_cq_buf, nent: c_int) -> c_int;
}
extern "C" {
    pub fn mthca_free_cq_buf(dev: *mut mthca_dev, buf: *mut mthca_cq_buf, cqe: c_int);
}
extern "C" {
    pub fn mthca_free_srq(dev: *mut mthca_dev, srq: *mut mthca_srq);
}
extern "C" {
    pub fn mthca_query_srq(srq: *mut ib_srq, srq_attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn mthca_max_srq_sge(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_free_srq_wqe(srq: *mut mthca_srq, wqe_addr: u32);
}
extern "C" {
    pub fn mthca_free_qp(dev: *mut mthca_dev, qp: *mut mthca_qp);
}
extern "C" {
    pub fn mthca_destroy_ah(dev: *mut mthca_dev, ah: *mut mthca_ah) -> c_int;
}
extern "C" {
    pub fn mthca_ah_query(ibah: *mut ib_ah, attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn mthca_ah_grh_present(ah: *mut mthca_ah) -> c_int;
}
extern "C" {
    pub fn mthca_get_rate(dev: *mut mthca_dev, static_rate: c_int, port: u32) -> u8;
}
extern "C" {
    pub fn mthca_rate_to_ib(dev: *mut mthca_dev, mthca_rate: u8, port: u32) -> ib_rate;
}
extern "C" {
    pub fn mthca_multicast_attach(ibqp: *mut ib_qp, gid: *mut ib_gid, lid: u16) -> c_int;
}
extern "C" {
    pub fn mthca_multicast_detach(ibqp: *mut ib_qp, gid: *mut ib_gid, lid: u16) -> c_int;
}
extern "C" {
    pub fn mthca_create_agents(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_free_agents(dev: *mut mthca_dev);
}
extern "C" {
    pub fn container_of(_arg: ibdev, mthca_dev: struct, _arg: ib_dev) -> return;
}
