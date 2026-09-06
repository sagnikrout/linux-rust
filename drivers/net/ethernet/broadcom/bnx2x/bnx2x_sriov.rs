//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_sriov.h
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


// bnx2x_sriov.h: QLogic Everest network driver.
//
// Copyright 2009-2013 Broadcom Corporation
// Copyright 2014 QLogic Corporation
// All rights reserved
//
// Unless you and QLogic execute a separate written software license
// agreement governing use of this software, this software is licensed to you
// under the terms of the GNU General Public License version 2, available
// at http://www.gnu.org/licenses/old-licenses/gpl-2.0.html (the "GPL").
//
// Notwithstanding the above, under no circumstances may you combine this
// software in any way with any other QLogic software provided under a
// license other than the GPL, without QLogic's express prior written
// consent.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Shmulik Ravid
// Ariel Elior <ariel.elior@qlogic.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sample_bulletin_result {
    PFVF_BULLETIN_UNCHANGED,
    PFVF_BULLETIN_UPDATED,
    PFVF_BULLETIN_CRC_ERR
}

// The bnx2x device structure holds vfdb structure described below.
// The VF array is indexed by the relative vfid.
//
pub const BNX2X_VF_MAX_QUEUES: c_int = 16;
pub const BNX2X_VF_MAX_TPA_AGG_QUEUES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_sriov {
    pub first_vf_in_pf: u32,
// standard SRIOV capability fields, mostly for debugging
    pub /: *mut *mut int pos; / capability position,
    pub /: *mut *mut int nres; / number of resources,
    pub /: *mut *mut u32 cap; / SR-IOV Capabilities,
    pub /: *mut *mut u16 ctrl; / SR-IOV Control,
    pub /: *mut *mut u16 total; / total VFs associated with the PF,
    pub /: *mut *mut u16 initial; / initial VFs associated with the PF,
    pub /: *mut *mut u16 nr_virtfn; / number of VFs available,
    pub /: *mut *mut u16 offset; / first VF Routing ID offset,
    pub /: *mut *mut u16 stride; / following VF stride,
    pub /: *mut *mut u32 pgsz; / page size for BAR alignment,
    pub /: *mut *mut u8 link; / Function Dependency Link,
}

// bars
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_bar {
    pub bar: u64,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_bar_info {
    pub bars: [bnx2x_vf_bar; PCI_SRIOV_NUM_BARS],
    pub nr_bars: u8,
}

// vf queue (used both for rx or tx)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_queue {
    pub cxt: *mut eth_context,
// MACs object
    pub mac_obj: bnx2x_vlan_mac_obj,
// VLANs object
    pub vlan_obj: bnx2x_vlan_mac_obj,
// VLAN-MACs object
    pub vlan_mac_obj: bnx2x_vlan_mac_obj,
    pub /: *mut *mut unsigned long accept_flags; / last accept flags configured,
// Queue Slow-path State object
    pub sp_obj: bnx2x_queue_sp_obj,
    pub cid: u32,
    pub index: u16,
    pub sb_idx: u16,
    pub is_leading: bool,
    pub sp_initialized: bool,
}

// struct bnx2x_vf_queue_construct_params - prepare queue construction
// parameters: q-init, q-setup and SB index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_queue_construct_params {
    pub qstate: bnx2x_queue_state_params,
    pub prep_qsetup: bnx2x_queue_setup_params,
}

// forward
// VFOP definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_mac_vlan_filter {
    pub type: c_int,

    pub add: bool,
    pub applied: bool,
    pub mac: *mut u8,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_mac_vlan_filters {
    pub count: c_int,
    pub filters: [bnx2x_vf_mac_vlan_filter; ],
}

// vf context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_virtf {
    pub cfg_flags: u16,
pub const VF_CFG_STATS_COALESCE: c_uint = 0x1;
pub const VF_CFG_EXT_BULLETIN: c_uint = 0x2;
pub const VF_CFG_VLAN_FILTER: c_uint = 0x4;
    pub IFLA_VF_LINK_STATE_AUTO: *mut *mut u8 link_cfg; /,
// IFLA_VF_LINK_STATE_ENABLE
// IFLA_VF_LINK_STATE_DISABLE
//
    pub state: u8,

    pub /: *mut *mut bool flr_clnup_stage; / true during flr cleanup,
    pub /: *mut *mut bool malicious; / true if FW indicated so, until FLR,
// 1(true) if spoof check is enabled
    pub spoofchk: u8,
// dma
    pub fw_stat_map: dma_addr_t,
    pub stats_stride: u16,
    pub bulletin_map: dma_addr_t,
// Allocated resources counters. Before the VF is acquired, the
// counters hold the following values:
//
// - xxq_count = 0 as the queues memory is not allocated yet.
//
// - sb_count  = The number of status blocks configured for this VF in
// the IGU CAM. Initially read during probe.
//
// - xx_rules_count = The number of rules statically and equally
// allocated for each VF, during PF load.
//
    pub alloc_resc: vf_pf_resc_request,

    pub /: *mut *mut u8 sb_count; / actual number of SBs,
    pub /: *mut *mut u8 igu_base_id; / base igu status block id,
    pub vfqs: *mut bnx2x_vf_queue,
pub const LEADING_IDX: c_int = 0;

    pub /: *mut *mut u8 index; / index in the vf array,
    pub abs_vfid: u8,
    pub sp_cl_id: u8,
    pub /: *mut *mut u32 error; / 0 means all's-well,
// BDF
    pub domain: c_uint,
    pub bus: c_uint,
    pub devfn: c_uint,
// bars
    pub bars: [bnx2x_vf_bar; PCI_SRIOV_NUM_BARS],
// set-mac ramrod state 1-pending, 0-done
    pub filter_state: c_ulong,
// leading rss client id ~~ the client id of the first rxq, must be
// set for each txq.
//
    pub leading_rss: c_int,
// MCAST object
    pub mcast_obj: bnx2x_mcast_obj,
// RSS configuration object
    pub rss_conf_obj: bnx2x_rss_config_obj,
// slow-path operations
    pub /: *mut *mut mutex op_mutex; / one vfop at a time mutex,
    pub op_current: channel_tlvs,
    pub fp_hsi: u8,
    pub vf_vlans_pool: bnx2x_credit_pool_obj,
    pub vf_macs_pool: bnx2x_credit_pool_obj,
}

pub const FW_PF_MAX_HANDLE: c_int = 8;

pub const VF_MAC_CREDIT_CNT: c_int = 1;

// locking and unlocking the channel mutex
// VF mail box (aka vf-pf channel)
// a container for the bi-directional vf<-->pf messages.
// The actual response will be placed according to the offset parameter
// provided in the request
//
pub const MBX_MSG_ALIGN: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_mbx_msg {
    pub req: vfpf_tlvs,
    pub resp: pfvf_tlvs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_mbx {
    pub msg: *mut bnx2x_vf_mbx_msg,
    pub msg_mapping: dma_addr_t,
// VF GPA address
    pub vf_addr_lo: u32,
    pub vf_addr_hi: u32,
    pub /: *mut *mut vfpf_first_tlv first_tlv; / saved VF request header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vf_sp {
    pub e2: eth_classify_rules_ramrod_data,
    pub mac_rdata: },
    pub e2: eth_classify_rules_ramrod_data,
    pub vlan_rdata: },
    pub e2: eth_classify_rules_ramrod_data,
    pub vlan_mac_rdata: },
    pub e2: eth_filter_rules_ramrod_data,
    pub rx_mode_rdata: },
    pub e2: eth_multicast_rules_ramrod_data,
    pub mcast_rdata: },
    pub init_data: client_init_ramrod_data,
    pub update_data: client_update_ramrod_data,
    pub q_data: },
    pub e2: eth_rss_update_ramrod_data,
    pub rss_rdata: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_dma {
    pub addr: *mut c_void,
    pub mapping: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vfdb {

// vf array
    pub vfs: *mut bnx2x_virtf,

// queue array - for all vfs
    pub vfqs: *mut bnx2x_vf_queue,
// vf HW contexts
    pub context: [hw_dma; BNX2X_VF_CIDS/ILT_PAGE_CIDS],
// SR-IOV information
    pub sriov: bnx2x_sriov,
    pub mbx_dma: hw_dma,
    pub mbxs: [bnx2x_vf_mbx; BNX2X_MAX_NUM_OF_VFS],
    pub bulletin_dma: hw_dma,

    pub sp_dma: hw_dma,
    pub flrd_vfs: [u32; FLRD_VFS_DWORDS],
// the number of msix vectors belonging to this PF designated for VFs
    pub vf_sbs_pool: u16,
    pub first_vf_igu_entry: u16,
// sp_rtnl synchronization
    pub event_mutex: mutex,
    pub event_occur: u64,
// bulletin board update synchronization
    pub bulletin_mutex: mutex,
}

// queue access
// FW ids
extern "C" {
    pub fn vf_igu_sb(_arg: vf, _arg: sb_idx) -> return;
}
extern "C" {
    pub fn vfq_cl_id(_arg: vf, _arg: q) -> return;
}
extern "C" {
    pub fn vfq_cl_id(_arg: vf, _arg: q) -> return;
}
// global iov routines
extern "C" {
    pub fn bnx2x_iov_init_ilt(bp: *mut bnx2x, line: u16) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_init_one(bp: *mut bnx2x, int_mode_param: c_int, num_vfs_param: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_remove_one(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_free_mem(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_alloc_mem(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_nic_init(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_chip_cleanup(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_init_dq(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_init_dmae(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_eq_sp_event(bp: *mut bnx2x, elem: *mut event_ring_elem) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_adjust_stats_req(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_storm_stats_update(bp: *mut bnx2x);
}
// global vf mailbox routines
extern "C" {
    pub fn bnx2x_vf_mbx(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_vf_enable_mbx(bp: *mut bnx2x, abs_vfid: u8);
}
// CORE VF API
// acquire
// init
// VFOP queue construction helpers
extern "C" {
    pub fn bnx2x_vf_queue_teardown(bp: *mut bnx2x, vf: *mut bnx2x_virtf, qid: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2x_vf_close(bp: *mut bnx2x, vf: *mut bnx2x_virtf) -> c_int;
}
extern "C" {
    pub fn bnx2x_vf_free(bp: *mut bnx2x, vf: *mut bnx2x_virtf) -> c_int;
}
// VF release ~ VF close + VF release-resources
//
// Release is the ultimate SW shutdown and is called whenever an
// irrecoverable error is encountered.
//
extern "C" {
    pub fn bnx2x_vf_release(bp: *mut bnx2x, vf: *mut bnx2x_virtf) -> c_int;
}
extern "C" {
    pub fn bnx2x_vf_idx_by_abs_fid(bp: *mut bnx2x, abs_vfid: u16) -> c_int;
}
extern "C" {
    pub fn bnx2x_vf_max_queue_cnt(bp: *mut bnx2x, vf: *mut bnx2x_virtf) -> u8;
}
// FLR routines
// VF FLR helpers
extern "C" {
    pub fn bnx2x_vf_flr_clnup_epilog(bp: *mut bnx2x, abs_vfid: u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_vf_enable_access(bp: *mut bnx2x, abs_vfid: u8);
}
// Handles an FLR (or VF_DISABLE) notification form the MCP
extern "C" {
    pub fn bnx2x_vf_handle_flr_event(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_tlv_supported(tlvtype: u16) -> bool;
}
extern "C" {
    pub fn bnx2x_crc_vf_bulletin(bulletin: *mut pf_vf_bulletin_content) -> u32;
}
extern "C" {
    pub fn bnx2x_post_vf_bulletin(bp: *mut bnx2x, vf: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2x_sample_bulletin(bp: *mut bnx2x) -> sample_bulletin_result;
}
// VF side vfpf channel functions
extern "C" {
    pub fn bnx2x_vfpf_acquire(bp: *mut bnx2x, tx_count: u8, rx_count: u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_vfpf_release(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_vfpf_init(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_vfpf_close_vf(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_vfpf_set_mcast(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn bnx2x_vfpf_storm_rx_mode(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_sample_bulletin(bp: *mut bnx2x) -> sample_bulletin_result;
}
extern "C" {
    pub fn bnx2x_timer_sriov(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_vf_pci_dealloc(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_vf_pci_alloc(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_enable_sriov(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_disable_sriov(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_pf_set_vfs_vlan(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_sriov_configure(dev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2x_iov_channel_down(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_task(work: *mut work_struct);
}
extern "C" {
    pub fn bnx2x_schedule_iov_task(bp: *mut bnx2x, flag: bnx2x_iov_flag);
}
extern "C" {
    pub fn bnx2x_iov_link_update(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_iov_link_update_vf(bp: *mut bnx2x, idx: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_vf_link_state(dev: *mut net_device, vf: c_int, link_state: c_int) -> c_int;
}
extern "C" {
    pub fn bnx2x_vfpf_update_vlan(bp: *mut bnx2x, vid: u16, vf_qid: u8, add: bool) -> c_int;
}

pub const GET_NUM_VFS_PER_PATH(bp): c_int = 0;
pub const GET_NUM_VFS_PER_PF(bp): c_int = 0;
pub const VF_MAC_CREDIT_CNT: c_int = 0;
pub const VF_VLAN_CREDIT_CNT: c_int = 0;

