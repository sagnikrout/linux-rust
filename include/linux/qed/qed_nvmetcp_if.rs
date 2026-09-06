//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_nvmetcp_if.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// Copyright 2021 Marvell. All rights reserved.

pub const QED_NVMETCP_MAX_IO_SIZE: c_uint = 0x800000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev_nvmetcp_info {
    pub common: qed_dev_info,
    pub /: *mut *mut u8 port_id; / Physical port,
    pub num_cqs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_tid {
    pub /: *mut *mut u32 size; / In bytes per task,
    pub num_tids_per_block: u32,
    pub blocks: [*mut u8; MAX_TID_BLOCKS_NVMETCP],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_id_params {
    pub mac: [u8; ETH_ALEN],
    pub ip: [u32; 4],
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_params_offload {
// FW initializations
    pub sq_pbl_addr: dma_addr_t,
    pub nvmetcp_cccid_itid_table_addr: dma_addr_t,
    pub nvmetcp_cccid_max_range: u16,
    pub default_cq: u8,
// Networking and TCP stack initializations
    pub src: qed_nvmetcp_id_params,
    pub dst: qed_nvmetcp_id_params,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub max_rt_time: u32,
    pub cwnd: u32,
    pub mss: u16,
    pub vlan_id: u16,
    pub timestamp_en: bool,
    pub delayed_ack_en: bool,
    pub tcp_keep_alive_en: bool,
    pub ecn_en: bool,
    pub ip_version: u8,
    pub ka_max_probe_cnt: u8,
    pub ttl: u8,
    pub tos_or_tc: u8,
    pub rcv_wnd_scale: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_params_update {
    pub max_io_size: u32,
    pub max_recv_pdu_length: u32,
    pub max_send_pdu_length: u32,
// Placeholder: pfv, cpda, hpda
    pub hdr_digest_en: bool,
    pub data_digest_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_cb_ops {
    pub common: qed_common_cb_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_sge {
    pub /: *mut *mut regpair sge_addr; / SGE address,
    pub /: *mut *mut __le32 sge_len; / SGE length,
    pub reserved: __le32,
}

// IO path HSI function SGL params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct storage_sgl_task_params {
    pub sgl: *mut nvmetcp_sge,
    pub sgl_phys_addr: regpair,
    pub total_buffer_size: u32,
    pub num_sges: u16,
    pub small_mid_sge: bool,
}

// IO path HSI function FW task context params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmetcp_task_params {
    pub /: *mut *mut *mut void context; / Output parameter - set/filled by the HSI function,
    pub sqe: *mut nvmetcp_wqe,
    pub /: *mut *mut u32 tx_io_size; / in bytes (Without DIF, if exists),
    pub /: *mut *mut u32 rx_io_size; / in bytes (Without DIF, if exists),
    pub conn_icid: u16,
    pub itid: u16,
    pub /: *mut *mut regpair opq; / qedn_task_ctx address,
    pub host_cccid: u16,
    pub cq_rss_number: u8,
    pub send_write_incapsule: bool,
}

//
// struct qed_nvmetcp_ops - qed NVMeTCP operations.
// @common:		common operations pointer
// @ll2:		light L2 operations pointer
// @fill_dev_info:	fills NVMeTCP specific information
// @param cdev
// @param info
// @return 0 on success, otherwise error value.
// @register_ops:	register nvmetcp operations
// @param cdev
// @param ops - specified using qed_nvmetcp_cb_ops
// @param cookie - driver private
// @start:		nvmetcp in FW
// @param cdev
// @param tasks - qed will fill information about tasks
// return 0 on success, otherwise error value.
// @stop:		nvmetcp in FW
// @param cdev
// return 0 on success, otherwise error value.
// @acquire_conn:	acquire a new nvmetcp connection
// @param cdev
// @param handle - qed will fill handle that should be
// used henceforth as identifier of the
// connection.
// @param p_doorbell - qed will fill the address of the
// doorbell.
// @return 0 on success, otherwise error value.
// @release_conn:	release a previously acquired nvmetcp connection
// @param cdev
// @param handle - the connection handle.
// @return 0 on success, otherwise error value.
// @offload_conn:	configures an offloaded connection
// @param cdev
// @param handle - the connection handle.
// @param conn_info - the configuration to use for the
// offload.
// @return 0 on success, otherwise error value.
// @update_conn:	updates an offloaded connection
// @param cdev
// @param handle - the connection handle.
// @param conn_info - the configuration to use for the
// offload.
// @return 0 on success, otherwise error value.
// @destroy_conn:	stops an offloaded connection
// @param cdev
// @param handle - the connection handle.
// @return 0 on success, otherwise error value.
// @clear_sq:		clear all task in sq
// @param cdev
// @param handle - the connection handle.
// @return 0 on success, otherwise error value.
// @add_src_tcp_port_filter: Add source tcp port filter
// @param cdev
// @param src_port
// @remove_src_tcp_port_filter: Remove source tcp port filter
// @param cdev
// @param src_port
// @add_dst_tcp_port_filter: Add destination tcp port filter
// @param cdev
// @param dest_port
// @remove_dst_tcp_port_filter: Remove destination tcp port filter
// @param cdev
// @param dest_port
// @clear_all_filters: Clear all filters.
// @param cdev
// @init_read_io: Init read IO.
// @task_params
// @cmd_pdu_header
// @nvme_cmd
// @sgl_task_params
// @init_write_io: Init write IO.
// @task_params
// @cmd_pdu_header
// @nvme_cmd
// @sgl_task_params
// @init_icreq_exchange: Exchange ICReq.
// @task_params
// @init_conn_req_pdu_hdr
// @tx_sgl_task_params
// @rx_sgl_task_params
// @init_task_cleanup: Init task cleanup.
// @task_params
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_nvmetcp_ops {
    pub common: *const qed_common_ops,
    pub ll2: *const qed_ll2_ops,
    pub info): *mut qed_dev_nvmetcp_info,
    pub cookie): *mut *mut qed_nvmetcp_cb_ops ops, void,
    pub async_event_cb): *mut *mut void event_context, nvmetcp_event_cb_t,
    pub cdev): *mut *mut int (stop)(struct qed_dev,
    pub p_doorbell): *mut *mut u32 fw_cid, void __iomem,
    pub handle): *mut *mut *mut int (release_conn)(struct qed_dev cdev, u32,
    pub conn_info): *mut qed_nvmetcp_params_offload,
    pub conn_info): *mut qed_nvmetcp_params_update,
    pub abrt_conn): *mut *mut *mut int (destroy_conn)(struct qed_dev cdev, u32 handle, u8,
    pub handle): *mut *mut *mut int (clear_sq)(struct qed_dev cdev, u32,
    pub src_port): *mut *mut *mut int (add_src_tcp_port_filter)(struct qed_dev cdev, u16,
    pub src_port): *mut *mut *mut void (remove_src_tcp_port_filter)(struct qed_dev cdev, u16,
    pub dest_port): *mut *mut *mut int (add_dst_tcp_port_filter)(struct qed_dev cdev, u16,
    pub dest_port): *mut *mut *mut void (remove_dst_tcp_port_filter)(struct qed_dev cdev, u16,
    pub cdev): *mut *mut void (clear_all_filters)(struct qed_dev,
    pub sgl_task_params): *mut storage_sgl_task_params,
    pub sgl_task_params): *mut storage_sgl_task_params,
    pub rx_sgl_task_params): *mut storage_sgl_task_params,
    pub task_params): *mut *mut void (init_task_cleanup)(struct nvmetcp_task_params,
}

extern "C" {
    pub fn qed_put_nvmetcp_ops();
}
