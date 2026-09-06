//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_iscsi_if.h
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
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_stats {
    pub iscsi_rx_bytes_cnt: u64,
    pub iscsi_rx_packet_cnt: u64,
    pub iscsi_rx_new_ooo_isle_events_cnt: u64,
    pub iscsi_cmdq_threshold_cnt: u32,
    pub iscsi_rq_threshold_cnt: u32,
    pub iscsi_immq_threshold_cnt: u32,
    pub iscsi_rx_dropped_pdus_task_not_valid: u64,
    pub iscsi_rx_data_pdu_cnt: u64,
    pub iscsi_rx_r2t_pdu_cnt: u64,
    pub iscsi_rx_total_pdu_cnt: u64,
    pub iscsi_tx_go_to_slow_start_event_cnt: u64,
    pub iscsi_tx_fast_retransmit_event_cnt: u64,
    pub iscsi_tx_data_pdu_cnt: u64,
    pub iscsi_tx_r2t_pdu_cnt: u64,
    pub iscsi_tx_total_pdu_cnt: u64,
    pub iscsi_tx_bytes_cnt: u64,
    pub iscsi_tx_packet_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev_iscsi_info {
    pub common: qed_dev_info,
    pub primary_dbq_rq_addr: *mut void __iomem,
    pub secondary_bdq_rq_addr: *mut void __iomem,
    pub num_cqs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_id_params {
    pub mac: [u8; ETH_ALEN],
    pub ip: [u32; 4],
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_params_offload {
    pub layer_code: u8,
    pub sq_pbl_addr: dma_addr_t,
    pub initial_ack: u32,
    pub src: qed_iscsi_id_params,
    pub dst: qed_iscsi_id_params,
    pub vlan_id: u16,
    pub tcp_flags: u8,
    pub ip_version: u8,
    pub default_cq: u8,
    pub ka_max_probe_cnt: u8,
    pub dup_ack_theshold: u8,
    pub rcv_next: u32,
    pub snd_una: u32,
    pub snd_next: u32,
    pub snd_max: u32,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub snd_wl1: u32,
    pub cwnd: u32,
    pub ss_thresh: u32,
    pub srtt: u16,
    pub rtt_var: u16,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub total_rt: u32,
    pub ka_timeout_delta: u32,
    pub rt_timeout_delta: u32,
    pub dup_ack_cnt: u8,
    pub snd_wnd_probe_cnt: u8,
    pub ka_probe_cnt: u8,
    pub rt_cnt: u8,
    pub flow_label: u32,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub max_rt_time: u32,
    pub initial_rcv_wnd: u32,
    pub ttl: u8,
    pub tos_or_tc: u8,
    pub remote_port: u16,
    pub local_port: u16,
    pub mss: u16,
    pub snd_wnd_scale: u8,
    pub rcv_wnd_scale: u8,
    pub da_timeout_value: u16,
    pub ack_frequency: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_params_update {
    pub update_flag: u8,

    pub max_seq_size: u32,
    pub max_recv_pdu_length: u32,
    pub max_send_pdu_length: u32,
    pub first_seq_length: u32,
    pub exp_stat_sn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_tid {
    pub /: *mut *mut u32 size; / In bytes per task,
    pub num_tids_per_block: u32,
    pub blocks: [*mut u8; MAX_TID_BLOCKS_ISCSI],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_cb_ops {
    pub common: qed_common_cb_ops,
}

//
// struct qed_iscsi_ops - qed iSCSI operations.
// @common:		common operations pointer
// @ll2:		light L2 operations pointer
// @fill_dev_info:	fills iSCSI specific information
// @param cdev
// @param info
// @return 0 on success, otherwise error value.
// @register_ops:	register iscsi operations
// @param cdev
// @param ops - specified using qed_iscsi_cb_ops
// @param cookie - driver private
// @start:		iscsi in FW
// @param cdev
// @param tasks - qed will fill information about tasks
// return 0 on success, otherwise error value.
// @stop:		iscsi in FW
// @param cdev
// return 0 on success, otherwise error value.
// @acquire_conn:	acquire a new iscsi connection
// @param cdev
// @param handle - qed will fill handle that should be
// used henceforth as identifier of the
// connection.
// @param p_doorbell - qed will fill the address of the
// doorbell.
// @return 0 on success, otherwise error value.
// @release_conn:	release a previously acquired iscsi connection
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
// @get_stats:		iSCSI related statistics
// @param cdev
// @param stats - pointer to struck that would be filled
// we stats
// @return 0 on success, error otherwise.
// @change_mac:		Change MAC of interface
// @param cdev
// @param handle - the connection handle.
// @param mac - new MAC to configure.
// @return 0 on success, otherwise error value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iscsi_ops {
    pub common: *const qed_common_ops,
    pub ll2: *const qed_ll2_ops,
    pub info): *mut qed_dev_iscsi_info,
    pub cookie): *mut *mut qed_iscsi_cb_ops ops, void,
    pub async_event_cb): *mut *mut void event_context, iscsi_event_cb_t,
    pub cdev): *mut *mut int (stop)(struct qed_dev,
    pub p_doorbell): *mut *mut u32 fw_cid, void __iomem,
    pub handle): *mut *mut *mut int (release_conn)(struct qed_dev cdev, u32,
    pub conn_info): *mut qed_iscsi_params_offload,
    pub conn_info): *mut qed_iscsi_params_update,
    pub abrt_conn): *mut *mut *mut int (destroy_conn)(struct qed_dev cdev, u32 handle, u8,
    pub handle): *mut *mut *mut int (clear_sq)(struct qed_dev cdev, u32,
    pub stats): *mut qed_iscsi_stats,
    pub mac): *const *const *const int (change_mac)(struct qed_dev cdev, u32 handle, u8,
}

extern "C" {
    pub fn qed_put_iscsi_ops();
}
