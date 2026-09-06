//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/vmw_pvrdma/pvrdma.h
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
// Copyright (c) 2012-2016 VMware, Inc.  All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of EITHER the GNU General Public License
// version 2 as published by the Free Software Foundation or the BSD
// 2-Clause License. This program is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY; WITHOUT EVEN THE IMPLIED
// WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License version 2 for more details at
// http://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html.
//
// You should have received a copy of the GNU General Public License
// along with this program available in the file COPYING in the main
// directory of this source tree.
//
// The BSD 2-Clause License
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
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.
//

// NOT the same as BIT_MASK().

//
// VMware PVRDMA PCI device id.
//
pub const PCI_DEVICE_ID_VMWARE_PVRDMA: c_uint = 0x0820;
pub const PVRDMA_NUM_RING_PAGES: c_int = 4;
pub const PVRDMA_QP_NUM_HEADER_PAGES: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_page_dir {
    pub dir_dma: dma_addr_t,
    pub dir: *mut u64,
    pub ntables: c_int,
    pub tables: *mut u64,
    pub npages: u64,
    pub pages: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cq {
    pub ibcq: ib_cq,
    pub offset: c_int,
    pub /: *mut *mut spinlock_t cq_lock; / Poll lock.,
    pub uar: *mut pvrdma_uar_map,
    pub umem: *mut ib_umem,
    pub ring_state: *mut pvrdma_ring_state,
    pub pdir: pvrdma_page_dir,
    pub cq_handle: u32,
    pub is_kernel: bool,
    pub refcnt: refcount_t,
    pub free: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_id_table {
    pub last: u32,
    pub top: u32,
    pub max: u32,
    pub mask: u32,
    pub /: *mut *mut spinlock_t lock; / Table lock.,
    pub table: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_uar_map {
    pub pfn: c_ulong,
    pub map: *mut void __iomem,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_uar_table {
    pub tbl: pvrdma_id_table,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ucontext {
    pub ibucontext: ib_ucontext,
    pub dev: *mut pvrdma_dev,
    pub uar: pvrdma_uar_map,
    pub ctx_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_pd {
    pub ibpd: ib_pd,
    pub pdn: u32,
    pub pd_handle: u32,
    pub privileged: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_mr {
    pub mr_handle: u32,
    pub iova: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_user_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub mmr: pvrdma_mr,
    pub pdir: pvrdma_page_dir,
    pub pages: *mut u64,
    pub npages: u32,
    pub max_pages: u32,
    pub page_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_wq {
    pub ring: *mut pvrdma_ring,
    pub /: *mut *mut spinlock_t lock; / Work queue lock.,
    pub wqe_cnt: c_int,
    pub wqe_size: c_int,
    pub max_sg: c_int,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ah {
    pub ibah: ib_ah,
    pub av: pvrdma_av,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_srq {
    pub ibsrq: ib_srq,
    pub offset: c_int,
    pub /: *mut *mut spinlock_t lock; / SRQ lock.,
    pub wqe_cnt: c_int,
    pub wqe_size: c_int,
    pub max_gs: c_int,
    pub umem: *mut ib_umem,
    pub ring: *mut pvrdma_ring_state,
    pub pdir: pvrdma_page_dir,
    pub srq_handle: u32,
    pub npages: c_int,
    pub refcnt: refcount_t,
    pub free: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_qp {
    pub ibqp: ib_qp,
    pub qp_handle: u32,
    pub qkey: u32,
    pub sq: pvrdma_wq,
    pub rq: pvrdma_wq,
    pub rumem: *mut ib_umem,
    pub sumem: *mut ib_umem,
    pub pdir: pvrdma_page_dir,
    pub srq: *mut pvrdma_srq,
    pub npages: c_int,
    pub npages_send: c_int,
    pub npages_recv: c_int,
    pub flags: u32,
    pub port: u8,
    pub state: u8,
    pub is_kernel: bool,
    pub /: *mut *mut mutex mutex; / QP state mutex.,
    pub refcnt: refcount_t,
    pub free: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_dev {
// PCI device-related information.
    pub ib_dev: ib_device,
    pub pdev: *mut pci_dev,
    pub regs: *mut void __iomem,
    pub /: *mut *mut *mut pvrdma_device_shared_region dsr; / Shared region pointer,
    pub /: *mut *mut dma_addr_t dsrbase; / Shared region base address,
    pub cmd_slot: *mut c_void,
    pub resp_slot: *mut c_void,
    pub flags: c_ulong,
    pub device_link: list_head,
    pub dsr_version: c_uint,
// Locking and interrupt information.
    pub /: *mut *mut spinlock_t cmd_lock; / Command lock.,
    pub cmd_sema: semaphore,
    pub cmd_done: completion,
    pub nr_vectors: c_uint,
// RDMA-related device information.
    pub sgid_tbl: *mut ib_gid,
    pub async_ring_state: *mut pvrdma_ring_state,
    pub async_pdir: pvrdma_page_dir,
    pub cq_ring_state: *mut pvrdma_ring_state,
    pub cq_pdir: pvrdma_page_dir,
    pub cq_tbl: *mut pvrdma_cq,
    pub cq_tbl_lock: spinlock_t,
    pub srq_tbl: *mut pvrdma_srq,
    pub srq_tbl_lock: spinlock_t,
    pub qp_tbl: *mut pvrdma_qp,
    pub qp_tbl_lock: spinlock_t,
    pub uar_table: pvrdma_uar_table,
    pub driver_uar: pvrdma_uar_map,
    pub sys_image_guid: __be64,
    pub /: *mut *mut spinlock_t desc_lock; / Device modification lock.,
    pub port_cap_mask: u32,
    pub /: *mut *mut mutex port_mutex; / Port modification mutex.,
    pub ib_active: bool,
    pub num_qps: core::sync::atomic::AtomicI32,
    pub num_cqs: core::sync::atomic::AtomicI32,
    pub num_srqs: core::sync::atomic::AtomicI32,
    pub num_pds: core::sync::atomic::AtomicI32,
    pub num_ahs: core::sync::atomic::AtomicI32,
// Network device information.
    pub netdev: *mut net_device,
    pub nb_netdev: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_netdevice_work {
    pub work: work_struct,
    pub event_netdev: *mut net_device,
    pub event: c_ulong,
}

extern "C" {
    pub fn container_of(_arg: ibdev, pvrdma_dev: struct, _arg: ib_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibucontext, pvrdma_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, pvrdma_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibcq, pvrdma_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, pvrdma_srq: struct, _arg: ibsrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, pvrdma_user_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, pvrdma_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, pvrdma_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn le32_to_cpu(reg): readl(dev->regs +) -> return;
}
extern "C" {
    pub fn pvrdma_gid_to_ib(dst: *mut ib_gid, src: *const pvrdma_gid);
}
extern "C" {
    pub fn ib_gid_to_pvrdma(dst: *mut pvrdma_gid, src: *const ib_gid);
}
extern "C" {
    pub fn ib_gid_type_to_pvrdma(gid_type: ib_gid_type) -> u8;
}
extern "C" {
    pub fn pvrdma_uar_table_init(dev: *mut pvrdma_dev) -> c_int;
}
extern "C" {
    pub fn pvrdma_uar_table_cleanup(dev: *mut pvrdma_dev);
}
extern "C" {
    pub fn pvrdma_uar_alloc(dev: *mut pvrdma_dev, uar: *mut pvrdma_uar_map) -> c_int;
}
extern "C" {
    pub fn pvrdma_uar_free(dev: *mut pvrdma_dev, uar: *mut pvrdma_uar_map);
}
extern "C" {
    pub fn _pvrdma_flush_cqe(qp: *mut pvrdma_qp, cq: *mut pvrdma_cq);
}
extern "C" {
    pub fn pvrdma_page_dir_get_dma(pdir: *mut pvrdma_page_dir, idx: u64) -> dma_addr_t;
}
