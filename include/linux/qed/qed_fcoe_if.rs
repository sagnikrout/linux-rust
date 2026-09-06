//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_fcoe_if.h
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
// Copyright (c) 2019-2020 Marvell International Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_stats {
    pub fcoe_rx_byte_cnt: u64,
    pub fcoe_rx_data_pkt_cnt: u64,
    pub fcoe_rx_xfer_pkt_cnt: u64,
    pub fcoe_rx_other_pkt_cnt: u64,
    pub fcoe_silent_drop_pkt_cmdq_full_cnt: u32,
    pub fcoe_silent_drop_pkt_rq_full_cnt: u32,
    pub fcoe_silent_drop_pkt_crc_error_cnt: u32,
    pub fcoe_silent_drop_pkt_task_invalid_cnt: u32,
    pub fcoe_silent_drop_total_pkt_cnt: u32,
    pub fcoe_tx_byte_cnt: u64,
    pub fcoe_tx_data_pkt_cnt: u64,
    pub fcoe_tx_xfer_pkt_cnt: u64,
    pub fcoe_tx_other_pkt_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev_fcoe_info {
    pub common: qed_dev_info,
    pub primary_dbq_rq_addr: *mut void __iomem,
    pub secondary_bdq_rq_addr: *mut void __iomem,
    pub wwpn: u64,
    pub wwnn: u64,
    pub num_cqs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_params_offload {
    pub sq_pbl_addr: dma_addr_t,
    pub sq_curr_page_addr: dma_addr_t,
    pub sq_next_page_addr: dma_addr_t,
    pub src_mac: [u8; ETH_ALEN],
    pub dst_mac: [u8; ETH_ALEN],
    pub tx_max_fc_pay_len: u16,
    pub e_d_tov_timer_val: u16,
    pub rec_tov_timer_val: u16,
    pub rx_max_fc_pay_len: u16,
    pub vlan_tag: u16,
    pub s_id: fc_addr_nw,
    pub max_conc_seqs_c3: u8,
    pub d_id: fc_addr_nw,
    pub flags: u8,
    pub def_q_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_tid {
    pub /: *mut *mut u32 size; / In bytes per task,
    pub num_tids_per_block: u32,
    pub blocks: [*mut u8; MAX_TID_BLOCKS_FCOE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_cb_ops {
    pub common: qed_common_cb_ops,
    pub cookie): *mut *mut u32 (get_login_failures)(void,
}

//
// struct qed_fcoe_ops - qed FCoE operations.
// @common:		common operations pointer
// @fill_dev_info:	fills FCoE specific information
// @param cdev
// @param info
// @return 0 on success, otherwise error value.
// @register_ops:	register FCoE operations
// @param cdev
// @param ops - specified using qed_iscsi_cb_ops
// @param cookie - driver private
// @ll2:		light L2 operations pointer
// @start:		fcoe in FW
// @param cdev
// @param tasks - qed will fill information about tasks
// return 0 on success, otherwise error value.
// @stop:		stops fcoe in FW
// @param cdev
// return 0 on success, otherwise error value.
// @acquire_conn:	acquire a new fcoe connection
// @param cdev
// @param handle - qed will fill handle that should be
// used henceforth as identifier of the
// connection.
// @param p_doorbell - qed will fill the address of the
// doorbell.
// return 0 on success, otherwise error value.
// @release_conn:	release a previously acquired fcoe connection
// @param cdev
// @param handle - the connection handle.
// return 0 on success, otherwise error value.
// @offload_conn:	configures an offloaded connection
// @param cdev
// @param handle - the connection handle.
// @param conn_info - the configuration to use for the
// offload.
// return 0 on success, otherwise error value.
// @destroy_conn:	stops an offloaded connection
// @param cdev
// @param handle - the connection handle.
// @param terminate_params
// return 0 on success, otherwise error value.
// @get_stats:		gets FCoE related statistics
// @param cdev
// @param stats - pointer to struck that would be filled
// we stats
// return 0 on success, error otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_ops {
    pub common: *const qed_common_ops,
    pub info): *mut qed_dev_fcoe_info,
    pub cookie): *mut *mut qed_fcoe_cb_ops ops, void,
    pub ll2: *const qed_ll2_ops,
    pub tasks): *mut *mut *mut int (start)(struct qed_dev cdev, struct qed_fcoe_tid,
    pub cdev): *mut *mut int (stop)(struct qed_dev,
    pub p_doorbell): *mut *mut u32 fw_cid, void __iomem,
    pub handle): *mut *mut *mut int (release_conn)(struct qed_dev cdev, u32,
    pub conn_info): *mut qed_fcoe_params_offload,
    pub terminate_params): u32 handle, dma_addr_t,
    pub stats): *mut *mut *mut int (get_stats)(struct qed_dev cdev, struct qed_fcoe_stats,
}

extern "C" {
    pub fn qed_put_fcoe_ops();
}
