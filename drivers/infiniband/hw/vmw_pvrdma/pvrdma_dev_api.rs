//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/vmw_pvrdma/pvrdma_dev_api.h
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

//
// PVRDMA version macros. Some new features require updates to PVRDMA_VERSION.
// These macros allow us to check for different features if necessary.
//
pub const PVRDMA_ROCEV1_VERSION: c_int = 17;
pub const PVRDMA_ROCEV2_VERSION: c_int = 18;
pub const PVRDMA_PPN64_VERSION: c_int = 19;
pub const PVRDMA_QPHANDLE_VERSION: c_int = 20;

pub const PVRDMA_BOARD_ID: c_int = 1;
pub const PVRDMA_REV_ID: c_int = 1;
//
// Masks and accessors for page directory, which is a two-level lookup:
// page directory -> page table -> page. Only one directory for now, but we
// could expand that easily. 9 bits for tables, 9 bits for pages, gives one
// gigabyte for memory regions and so forth.
//
pub const PVRDMA_PDIR_SHIFT: c_int = 18;
pub const PVRDMA_PTABLE_SHIFT: c_int = 9;

pub const PVRDMA_MAX_FAST_REG_PAGES: c_int = 128;
//
// Max MSI-X vectors.
//
pub const PVRDMA_MAX_INTERRUPTS: c_int = 3;
// Register offsets within PCI resource on BAR1.
pub const PVRDMA_REG_VERSION: c_uint = 0x00	/* R: Version of device. */;
pub const PVRDMA_REG_DSRLOW: c_uint = 0x04	/* W: Device shared region low PA. */;
pub const PVRDMA_REG_DSRHIGH: c_uint = 0x08	/* W: Device shared region high PA. */;
pub const PVRDMA_REG_CTL: c_uint = 0x0c	/* W: PVRDMA_DEVICE_CTL */;
pub const PVRDMA_REG_REQUEST: c_uint = 0x10	/* W: Indicate device request. */;
pub const PVRDMA_REG_ERR: c_uint = 0x14	/* R: Device error. */;
pub const PVRDMA_REG_ICR: c_uint = 0x18	/* R: Interrupt cause. */;
pub const PVRDMA_REG_IMR: c_uint = 0x1c	/* R/W: Interrupt mask. */;
pub const PVRDMA_REG_MACL: c_uint = 0x20	/* R/W: MAC address low. */;
pub const PVRDMA_REG_MACH: c_uint = 0x24	/* R/W: MAC address high. */;
// Object flags.

//
// Atomic operation capability (masked versions are extended atomic
// operations.
//

//
// Base Memory Management Extension flags to support Fast Reg Memory Regions
// and Fast Reg Work Requests. Each flag represents a verb operation and we
// must support all of them to qualify for the BMME device cap.
//

//
// GID types. The interpretation of the gid_types bit field in the device
// capabilities will depend on the device mode. For now, the device only
// supports RoCE as mode, so only the different GID types for RoCE are
// defined.
//

//
// Version checks. This checks whether each version supports specific
// capabilities from the device.
//

//
// Get capability values based on device version.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_pci_resource {
    PVRDMA_PCI_RESOURCE_MSIX,	/* BAR0: MSI-X, MMIO. */
    PVRDMA_PCI_RESOURCE_REG,	/* BAR1: Registers, MMIO. */
    PVRDMA_PCI_RESOURCE_UAR,	/* BAR2: UAR pages, MMIO, 64-bit. */
    PVRDMA_PCI_RESOURCE_LAST,	/* Last. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_device_ctl {
    PVRDMA_DEVICE_CTL_ACTIVATE,	/* Activate device. */
    PVRDMA_DEVICE_CTL_UNQUIESCE,	/* Unquiesce device. */
    PVRDMA_DEVICE_CTL_RESET,	/* Reset device. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_intr_vector {
    PVRDMA_INTR_VECTOR_RESPONSE,	/* Command response. */
    PVRDMA_INTR_VECTOR_ASYNC,	/* Async events. */
    PVRDMA_INTR_VECTOR_CQ,		/* CQ notification. */
// Additional CQ notification vectors.
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_intr_cause {
    PVRDMA_INTR_CAUSE_RESPONSE	= (1 << PVRDMA_INTR_VECTOR_RESPONSE),
    PVRDMA_INTR_CAUSE_ASYNC		= (1 << PVRDMA_INTR_VECTOR_ASYNC),
    PVRDMA_INTR_CAUSE_CQ		= (1 << PVRDMA_INTR_VECTOR_CQ),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_gos_bits {
    PVRDMA_GOS_BITS_UNK,		/* Unknown. */
    PVRDMA_GOS_BITS_32,		/* 32-bit. */
    PVRDMA_GOS_BITS_64,		/* 64-bit. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_gos_type {
    PVRDMA_GOS_TYPE_UNK,		/* Unknown. */
    PVRDMA_GOS_TYPE_LINUX,		/* Linux. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_device_mode {
    PVRDMA_DEVICE_MODE_ROCE,	/* RoCE. */
    PVRDMA_DEVICE_MODE_IWARP,	/* iWarp. */
    PVRDMA_DEVICE_MODE_IB,		/* InfiniBand. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_gos_info {
    pub /: *mut *mut u32 gos_bits:2; / W: PVRDMA_GOS_BITS_,
    pub /: *mut *mut u32 gos_type:4; / W: PVRDMA_GOS_TYPE_,
    pub /: *mut *mut u32 gos_ver:16; / W: Guest OS version.,
    pub /: *mut *mut u32 gos_misc:10; / W: Other.,
    pub /: *mut *mut u32 pad; / Pad to 8-byte alignment.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_device_caps {
    pub /: *mut *mut u64 fw_ver; / R: Query device.,
    pub node_guid: __be64,
    pub sys_image_guid: __be64,
    pub max_mr_size: u64,
    pub page_size_cap: u64,
    pub /: *mut *mut u64 atomic_arg_sizes; / EX verbs.,
    pub /: *mut *mut u32 ex_comp_mask; / EX verbs.,
    pub /: *mut *mut u32 device_cap_flags2; / EX verbs.,
    pub /: *mut *mut u32 max_fa_bit_boundary; / EX verbs.,
    pub /: *mut *mut u32 log_max_atomic_inline_arg; / EX verbs.,
    pub vendor_id: u32,
    pub vendor_part_id: u32,
    pub hw_ver: u32,
    pub max_qp: u32,
    pub max_qp_wr: u32,
    pub device_cap_flags: u32,
    pub max_sge: u32,
    pub max_sge_rd: u32,
    pub max_cq: u32,
    pub max_cqe: u32,
    pub max_mr: u32,
    pub max_pd: u32,
    pub max_qp_rd_atom: u32,
    pub max_ee_rd_atom: u32,
    pub max_res_rd_atom: u32,
    pub max_qp_init_rd_atom: u32,
    pub max_ee_init_rd_atom: u32,
    pub max_ee: u32,
    pub max_rdd: u32,
    pub max_mw: u32,
    pub max_raw_ipv6_qp: u32,
    pub max_raw_ethy_qp: u32,
    pub max_mcast_grp: u32,
    pub max_mcast_qp_attach: u32,
    pub max_total_mcast_qp_attach: u32,
    pub max_ah: u32,
    pub max_fmr: u32,
    pub max_map_per_fmr: u32,
    pub max_srq: u32,
    pub max_srq_wr: u32,
    pub max_srq_sge: u32,
    pub max_uar: u32,
    pub gid_tbl_len: u32,
    pub max_pkeys: u16,
    pub local_ca_ack_delay: u8,
    pub phys_port_cnt: u8,
    pub /: *mut *mut u8 mode; / PVRDMA_DEVICE_MODE_,
    pub /: *mut *mut *mut u8 atomic_ops; / PVRDMA_ATOMIC_OP_ bits,
    pub /: *mut *mut u8 bmme_flags; / FRWR Mem Mgmt Extensions,
    pub /: *mut *mut u8 gid_types; / PVRDMA_GID_TYPE_FLAG_,
    pub max_fast_reg_page_list_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ring_page_info {
    pub /: *mut *mut u32 num_pages; / Num pages incl. header.,
    pub /: *mut *mut u32 reserved; / Reserved.,
    pub /: *mut *mut u64 pdir_dma; / Page directory PA.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_device_shared_region {
    pub /: *mut *mut u32 driver_version; / W: Driver version.,
    pub /: *mut *mut u32 pad; / Pad to 8-byte align.,
    pub /: *mut *mut pvrdma_gos_info gos_info; / W: Guest OS information.,
    pub /: *mut *mut u64 cmd_slot_dma; / W: Command slot address.,
    pub /: *mut *mut u64 resp_slot_dma; / W: Response slot address.,
    pub async_ring_pages: pvrdma_ring_page_info,
// W: Async ring page info.
    pub cq_ring_pages: pvrdma_ring_page_info,
// W: CQ ring page info.
    pub /: *mut *mut u32 uar_pfn; / W: UAR pageframe.,
    pub /: *mut *mut u64 uar_pfn64; / W: 64-bit UAR page frame.,
}

// Event types. Currently a 1:1 mapping with enum ib_event.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_eqe_type {
    PVRDMA_EVENT_CQ_ERR,
    PVRDMA_EVENT_QP_FATAL,
    PVRDMA_EVENT_QP_REQ_ERR,
    PVRDMA_EVENT_QP_ACCESS_ERR,
    PVRDMA_EVENT_COMM_EST,
    PVRDMA_EVENT_SQ_DRAINED,
    PVRDMA_EVENT_PATH_MIG,
    PVRDMA_EVENT_PATH_MIG_ERR,
    PVRDMA_EVENT_DEVICE_FATAL,
    PVRDMA_EVENT_PORT_ACTIVE,
    PVRDMA_EVENT_PORT_ERR,
    PVRDMA_EVENT_LID_CHANGE,
    PVRDMA_EVENT_PKEY_CHANGE,
    PVRDMA_EVENT_SM_CHANGE,
    PVRDMA_EVENT_SRQ_ERR,
    PVRDMA_EVENT_SRQ_LIMIT_REACHED,
    PVRDMA_EVENT_QP_LAST_WQE_REACHED,
    PVRDMA_EVENT_CLIENT_REREGISTER,
    PVRDMA_EVENT_GID_CHANGE,
}

// Event queue element.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_eqe {
    pub /: *mut *mut u32 type; / Event type.,
    pub /: *mut *mut u32 info; / Handle, other.,
}

// CQ notification queue element.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cqne {
    pub /: *mut *mut u32 info; / Handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_hdr {
    pub /: *mut *mut u64 response; / Key for response lookup.,
    pub /: *mut *mut u32 cmd; / PVRDMA_CMD_,
    pub /: *mut *mut u32 reserved; / Reserved.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_resp_hdr {
    pub /: *mut *mut u64 response; / From cmd hdr.,
    pub /: *mut *mut u32 ack; / PVRDMA_CMD_XXX_RESP,
    pub /: *mut *mut u8 err; / Error.,
    pub /: *mut *mut u8 reserved[3]; / Reserved.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_port {
    pub hdr: pvrdma_cmd_hdr,
    pub port_num: u8,
    pub reserved: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_port_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub attrs: pvrdma_port_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_pkey {
    pub hdr: pvrdma_cmd_hdr,
    pub port_num: u8,
    pub index: u8,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_pkey_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub pkey: u16,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_uc {
    pub hdr: pvrdma_cmd_hdr,
    pub /: *mut *mut u32 pfn; / UAR page frame number,
    pub /: *mut *mut u64 pfn64; / 64-bit UAR page frame number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_uc_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub ctx_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_uc {
    pub hdr: pvrdma_cmd_hdr,
    pub ctx_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_pd {
    pub hdr: pvrdma_cmd_hdr,
    pub ctx_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_pd_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub pd_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_pd {
    pub hdr: pvrdma_cmd_hdr,
    pub pd_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_mr {
    pub hdr: pvrdma_cmd_hdr,
    pub start: u64,
    pub length: u64,
    pub pdir_dma: u64,
    pub pd_handle: u32,
    pub access_flags: u32,
    pub flags: u32,
    pub nchunks: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_mr_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub mr_handle: u32,
    pub lkey: u32,
    pub rkey: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_mr {
    pub hdr: pvrdma_cmd_hdr,
    pub mr_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_cq {
    pub hdr: pvrdma_cmd_hdr,
    pub pdir_dma: u64,
    pub ctx_handle: u32,
    pub cqe: u32,
    pub nchunks: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_cq_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub cq_handle: u32,
    pub cqe: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_resize_cq {
    pub hdr: pvrdma_cmd_hdr,
    pub cq_handle: u32,
    pub cqe: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_resize_cq_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub cqe: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_cq {
    pub hdr: pvrdma_cmd_hdr,
    pub cq_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_srq {
    pub hdr: pvrdma_cmd_hdr,
    pub pdir_dma: u64,
    pub pd_handle: u32,
    pub nchunks: u32,
    pub attrs: pvrdma_srq_attr,
    pub srq_type: u8,
    pub reserved: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_srq_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub srqn: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_modify_srq {
    pub hdr: pvrdma_cmd_hdr,
    pub srq_handle: u32,
    pub attr_mask: u32,
    pub attrs: pvrdma_srq_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_srq {
    pub hdr: pvrdma_cmd_hdr,
    pub srq_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_srq_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub attrs: pvrdma_srq_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_srq {
    pub hdr: pvrdma_cmd_hdr,
    pub srq_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_qp {
    pub hdr: pvrdma_cmd_hdr,
    pub pdir_dma: u64,
    pub pd_handle: u32,
    pub send_cq_handle: u32,
    pub recv_cq_handle: u32,
    pub srq_handle: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
    pub lkey: u32,
    pub access_flags: u32,
    pub total_chunks: u16,
    pub send_chunks: u16,
    pub max_atomic_arg: u16,
    pub sq_sig_all: u8,
    pub qp_type: u8,
    pub is_srq: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_qp_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub qpn: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_qp_resp_v2 {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub qpn: u32,
    pub qp_handle: u32,
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_modify_qp {
    pub hdr: pvrdma_cmd_hdr,
    pub qp_handle: u32,
    pub attr_mask: u32,
    pub attrs: pvrdma_qp_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_qp {
    pub hdr: pvrdma_cmd_hdr,
    pub qp_handle: u32,
    pub attr_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_query_qp_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub attrs: pvrdma_qp_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_qp {
    pub hdr: pvrdma_cmd_hdr,
    pub qp_handle: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_qp_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub events_reported: u32,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_create_bind {
    pub hdr: pvrdma_cmd_hdr,
    pub mtu: u32,
    pub vlan: u32,
    pub index: u32,
    pub new_gid: [u8; 16],
    pub gid_type: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cmd_destroy_bind {
    pub hdr: pvrdma_cmd_hdr,
    pub index: u32,
    pub dest_gid: [u8; 16],
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pvrdma_cmd_req {
    pub hdr: pvrdma_cmd_hdr,
    pub query_port: pvrdma_cmd_query_port,
    pub query_pkey: pvrdma_cmd_query_pkey,
    pub create_uc: pvrdma_cmd_create_uc,
    pub destroy_uc: pvrdma_cmd_destroy_uc,
    pub create_pd: pvrdma_cmd_create_pd,
    pub destroy_pd: pvrdma_cmd_destroy_pd,
    pub create_mr: pvrdma_cmd_create_mr,
    pub destroy_mr: pvrdma_cmd_destroy_mr,
    pub create_cq: pvrdma_cmd_create_cq,
    pub resize_cq: pvrdma_cmd_resize_cq,
    pub destroy_cq: pvrdma_cmd_destroy_cq,
    pub create_qp: pvrdma_cmd_create_qp,
    pub modify_qp: pvrdma_cmd_modify_qp,
    pub query_qp: pvrdma_cmd_query_qp,
    pub destroy_qp: pvrdma_cmd_destroy_qp,
    pub create_bind: pvrdma_cmd_create_bind,
    pub destroy_bind: pvrdma_cmd_destroy_bind,
    pub create_srq: pvrdma_cmd_create_srq,
    pub modify_srq: pvrdma_cmd_modify_srq,
    pub query_srq: pvrdma_cmd_query_srq,
    pub destroy_srq: pvrdma_cmd_destroy_srq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pvrdma_cmd_resp {
    pub hdr: pvrdma_cmd_resp_hdr,
    pub query_port_resp: pvrdma_cmd_query_port_resp,
    pub query_pkey_resp: pvrdma_cmd_query_pkey_resp,
    pub create_uc_resp: pvrdma_cmd_create_uc_resp,
    pub create_pd_resp: pvrdma_cmd_create_pd_resp,
    pub create_mr_resp: pvrdma_cmd_create_mr_resp,
    pub create_cq_resp: pvrdma_cmd_create_cq_resp,
    pub resize_cq_resp: pvrdma_cmd_resize_cq_resp,
    pub create_qp_resp: pvrdma_cmd_create_qp_resp,
    pub create_qp_resp_v2: pvrdma_cmd_create_qp_resp_v2,
    pub query_qp_resp: pvrdma_cmd_query_qp_resp,
    pub destroy_qp_resp: pvrdma_cmd_destroy_qp_resp,
    pub create_srq_resp: pvrdma_cmd_create_srq_resp,
    pub query_srq_resp: pvrdma_cmd_query_srq_resp,
}
