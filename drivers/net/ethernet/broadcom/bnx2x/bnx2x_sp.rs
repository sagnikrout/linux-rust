//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_sp.h
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


// bnx2x_sp.h: Qlogic Everest network driver.
//
// Copyright 2011-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// Unless you and Qlogic execute a separate written software license
// agreement governing use of this software, this software is licensed to you
// under the terms of the GNU General Public License version 2, available
// at http://www.gnu.org/licenses/gpl-2.0.html (the "GPL").
//
// Notwithstanding the above, under no circumstances may you combine this
// software in any way with any other Qlogic software provided under a
// license other than the GPL, without Qlogic's express prior written
// consent.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Vladislav Zolotarov
//

// Macro flag: #define BNX2X_SP_VERBS
// Bits representing general command's configuration
// Wait until all pending commands complete
// Don't send a ramrod, only update a registry
// Configure HW according to the current object state
// Execute the next command now
// Don't add a new command and continue execution of postponed
// commands. If not set a new command will be added to the
// pending commands list.
//
// If there is another pending ramrod, wait until it finishes and
// re-try to submit this one. This flag can be set only in sleepable
// context, and should not be set from the context that completes the
// ramrods as deadlock will occur.
//
// Public slow path states
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_raw_obj {
    pub func_id: u8,
// Queue params
    pub cl_id: u8,
    pub cid: u32,
// Ramrod data buffer params
    pub rdata: *mut c_void,
    pub rdata_mapping: dma_addr_t,
// Ramrod state params
    pub /: *mut *mut int state; / "ramrod is pending" state bit,
    pub /: *mut *mut *mut unsigned long pstate; / pointer to state buffer,
    pub obj_type: bnx2x_obj_type,
    pub o): *mut bnx2x_raw_obj,
    pub o): *mut *mut bool (check_pending)(struct bnx2x_raw_obj,
    pub o): *mut *mut void (clear_pending)(struct bnx2x_raw_obj,
    pub o): *mut *mut void (set_pending)(struct bnx2x_raw_obj,
}

// VLAN-MAC commands related parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_mac_ramrod_data {
    pub mac: [u8; ETH_ALEN],
    pub is_inner_mac: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_ramrod_data {
    pub vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_mac_ramrod_data {
    pub mac: [u8; ETH_ALEN],
    pub is_inner_mac: u8,
    pub vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bnx2x_classification_ramrod_data {
    pub mac: bnx2x_mac_ramrod_data,
    pub vlan: bnx2x_vlan_ramrod_data,
    pub vlan_mac: bnx2x_vlan_mac_ramrod_data,
}

// VLAN_MAC commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_vlan_mac_cmd {
    BNX2X_VLAN_MAC_ADD,
    BNX2X_VLAN_MAC_DEL,
    BNX2X_VLAN_MAC_MOVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_mac_data {
// Requested command: BNX2X_VLAN_MAC_XX
    pub cmd: bnx2x_vlan_mac_cmd,
// used to contain the data related vlan_mac_flags bits from
// ramrod parameters.
//
    pub vlan_mac_flags: c_ulong,
// Needed for MOVE command
    pub target_obj: *mut bnx2x_vlan_mac_obj,
    pub u: bnx2x_classification_ramrod_data,
}

// Exe Queue obj
#[repr(C)]
#[derive(Copy, Clone)]
pub union bnx2x_exe_queue_cmd_data {
    pub vlan_mac: bnx2x_vlan_mac_data,
// TODO
    pub mcast: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_exeq_elem {
    pub link: list_head,
// Length of this element in the exe_chunk.
    pub cmd_len: c_int,
    pub cmd_data: bnx2x_exe_queue_cmd_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bnx2x_exeq_comp_elem {
    pub elem: *mut event_ring_elem,
}

// Return positive if entry was optimized, 0 - if not, negative
// in case of an error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_exe_queue_obj {
// Commands pending for an execution.
    pub exe_queue: list_head,
// Commands pending for an completion.
    pub pending_comp: list_head,
    pub lock: spinlock_t,
// Maximum length of commands' list for one execution
    pub exe_chunk_len: c_int,
    pub owner: *mut bnx2x_qable_obj,
// Virtual functions
//
// Called before commands execution for commands that are really
// going to be executed (after 'optimize').
//
// Must run under exe_queue->lock
//
    pub validate: exe_q_validate,
//
// Called before removing pending commands, cleaning allocated
// resources (e.g., credits from validate)
//
    pub remove: exe_q_remove,
//
// This will try to cancel the current pending commands list
// considering the new command.
//
// Returns the number of optimized commands or a negative error code
//
// Must run under exe_queue->lock
//
    pub optimize: exe_q_optimize,
//
// Run the next commands chunk (owner specific).
//
    pub execute: exe_q_execute,
//
// Return the exe_queue element containing the specific command
// if any. Otherwise return NULL.
//
    pub get: exe_q_get,
}

// Classification verbs: Set/Del MAC/VLAN/VLAN-MAC
//
// Element in the VLAN_MAC registry list having all currently configured
// rules.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_mac_registry_elem {
    pub link: list_head,
// Used to store the cam offset used for the mac/vlan/vlan-mac.
// Relevant for 57710 and 57711 only. VLANs and MACs share the
// same CAM for these chips.
//
    pub cam_offset: c_int,
// Needed for DEL and RESTORE flows
    pub vlan_mac_flags: c_ulong,
    pub u: bnx2x_classification_ramrod_data,
}

// Bits representing VLAN_MAC commands specific flags
// When looking for matching filters, some flags are not interesting

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_mac_ramrod_params {
// Object to run the command from
    pub vlan_mac_obj: *mut bnx2x_vlan_mac_obj,
// General command flags: COMP_WAIT, etc.
    pub ramrod_flags: c_ulong,
// Command specific configuration request
    pub user_req: bnx2x_vlan_mac_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_vlan_mac_obj {
    pub raw: bnx2x_raw_obj,
// Bookkeeping list: will prevent the addition of already existing
// entries.
//
    pub head: list_head,
// Implement a simple reader/writer lock on the head list.
// all these fields should only be accessed under the exe_queue lock
//
    pub /: *mut *mut u8 head_reader; / Num. of readers accessing head list,
    pub /: *mut *mut bool head_exe_request; / Pending execution request.,
    pub /: *mut *mut unsigned long saved_ramrod_flags; / Ramrods of pending execution,
// TODO: Add it's initialization in the init functions
    pub exe_queue: bnx2x_exe_queue_obj,
// MACs credit pool
    pub macs_pool: *mut bnx2x_credit_pool_obj,
// VLANs credit pool
    pub vlans_pool: *mut bnx2x_credit_pool_obj,
// RAMROD command to be used
    pub ramrod_cmd: c_int,
// copy first n elements onto preallocated buffer
//
// @param n number of elements to get
// @param buf buffer preallocated by caller into which elements
// will be copied. Note elements are 4-byte aligned
// so buffer size must be able to accommodate the
// aligned elements.
//
// @return number of copied bytes
//
    pub size): u8 stride, u8,
//
// Checks if ADD-ramrod with the given params may be performed.
//
// @return zero if the element may be added
//
    pub data): *mut bnx2x_classification_ramrod_data,
//
// Checks if DEL-ramrod with the given params may be performed.
//
// @return true if the element may be deleted
//
    pub data): *mut bnx2x_classification_ramrod_data,
//
// Checks if DEL-ramrod with the given params may be performed.
//
// @return true if the element may be deleted
//
    pub data): *mut bnx2x_classification_ramrod_data,
//
// Update the relevant credit object(s) (consume/return
// correspondingly).
//
    pub o): *mut *mut bool (get_credit)(struct bnx2x_vlan_mac_obj,
    pub o): *mut *mut bool (put_credit)(struct bnx2x_vlan_mac_obj,
    pub offset): *mut *mut *mut bool (get_cam_offset)(struct bnx2x_vlan_mac_obj o, int,
    pub offset): *mut *mut *mut bool (put_cam_offset)(struct bnx2x_vlan_mac_obj o, int,
//
// Configures one rule in the ramrod data buffer.
//
    pub cam_offset): c_int,
//
// Delete all configured elements having the given
// vlan_mac_flags specification. Assumes no pending for
// execution commands. Will schedule all currently
// configured MACs/VLANs/VLAN-MACs matching the vlan_mac_flags
// specification for deletion and will use the given
// ramrod_flags for the last DEL operation.
//
// @param bp
// @param o
// @param ramrod_flags RAMROD_XX flags
//
// @return 0 if the last operation has completed successfully
// and there are no more elements left, positive value
// if there are pending for completion commands,
// negative value in case of failure.
//
    pub ramrod_flags): *mut c_ulong,
//
// Reconfigures the next MAC/VLAN/VLAN-MAC element from the previously
// configured elements list.
//
// @param bp
// @param p Command parameters (RAMROD_COMP_WAIT bit in
// ramrod_flags is only taken into an account)
// @param ppos a pointer to the cookie that should be given back in the
// next call to make function handle the next element. If
// *ppos is set to NULL it will restart the iterator.
// If returned *ppos == NULL this means that the last
// element has been handled.
//
// @return int
//
    pub ppos): *mut bnx2x_vlan_mac_registry_elem,
//
// Should be called on a completion arrival.
//
// @param bp
// @param o
// @param cqe Completion element we are handling
// @param ramrod_flags if RAMROD_CONT is set the next bulk of
// pending commands will be executed.
// RAMROD_DRV_CLR_ONLY and RAMROD_RESTORE
// may also be set if needed.
//
// @return 0 if there are neither pending nor waiting for
// completion commands. Positive value if there are
// pending for execution or for completion commands.
// Negative value in case of an error (including an
// error in the cqe).
//
    pub ramrod_flags): *mut c_ulong,
//
// Wait for completion of all commands. Don't schedule new ones,
// just wait. It assumes that the completion code will schedule
// for new commands.
//
    pub o): *mut *mut *mut int (wait)(struct bnx2x bp, struct bnx2x_vlan_mac_obj,
}

// RX_MODE verbs:DROP_ALL/ACCEPT_ALL/ACCEPT_ALL_MULTI/ACCEPT_ALL_VLAN/NORMAL
// RX_MODE ramrod special flags: set in rx_mode_flags field in
// a bnx2x_rx_mode_ramrod_params.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_rx_mode_ramrod_params {
    pub rx_mode_obj: *mut bnx2x_rx_mode_obj,
    pub pstate: *mut c_ulong,
    pub state: c_int,
    pub cl_id: u8,
    pub cid: u32,
    pub func_id: u8,
    pub ramrod_flags: c_ulong,
    pub rx_mode_flags: c_ulong,
// rdata is either a pointer to eth_filter_rules_ramrod_data(e2) or to
// a tstorm_eth_mac_filter_config (e1x).
//
    pub rdata: *mut c_void,
    pub rdata_mapping: dma_addr_t,
// Rx mode settings
    pub rx_accept_flags: c_ulong,
// internal switching settings
    pub tx_accept_flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_rx_mode_obj {
    pub p): *mut bnx2x_rx_mode_ramrod_params,
    pub p): *mut bnx2x_rx_mode_ramrod_params,
}

// Set multicast group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_mcast_list_elem {
    pub link: list_head,
    pub mac: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bnx2x_mcast_config_data {
    pub mac: *mut u8,
    pub /: *mut *mut u8 bin; / used in a RESTORE flow,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_mcast_ramrod_params {
    pub mcast_obj: *mut bnx2x_mcast_obj,
// Relevant options are RAMROD_COMP_WAIT and RAMROD_DRV_CLR_ONLY
    pub ramrod_flags: c_ulong,
    pub /: *mut *mut list_head mcast_list; / list of bnx2x_mcast_list_elem,
// TODO:
// - rename it to macs_num.
// - Add a new command type for handling pending commands
// (remove "zero semantics").
//
// Length of mcast_list. If zero and ADD_CONT command - post
// pending commands.
//
    pub mcast_list_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_mcast_cmd {
    BNX2X_MCAST_CMD_ADD,
    BNX2X_MCAST_CMD_CONT,
    BNX2X_MCAST_CMD_DEL,
    BNX2X_MCAST_CMD_RESTORE,

// Following this, multicast configuration should equal to approx
// the set of MACs provided [i.e., remove all else].
// The two sub-commands are used internally to decide whether a given
// bin is to be added or removed
//
    BNX2X_MCAST_CMD_SET,
    BNX2X_MCAST_CMD_SET_ADD,
    BNX2X_MCAST_CMD_SET_DEL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_mcast_obj {
    pub raw: bnx2x_raw_obj,
pub const BNX2X_MCAST_BINS_NUM: c_int = 256;
    pub vec: [u64; BNX2X_MCAST_VEC_SZ],
// Number of BINs to clear. Should be updated
// immediately when a command arrives in order to
// properly create DEL commands.
//
    pub num_bins_set: c_int,
    pub aprox_match: },
    pub macs: list_head,
    pub num_macs_set: c_int,
    pub exact_match: },
    pub registry: },
// Pending commands
    pub pending_cmds_head: list_head,
// A state that is set in raw.pstate, when there are pending commands
    pub sched_state: c_int,
// Maximal number of mcast MACs configured in one command
    pub max_cmd_len: c_int,
// Total number of currently pending MACs to configure: both
// in the pending commands list and in the current command.
//
    pub total_pending_num: c_int,
    pub engine_id: u8,
//
// @param cmd command to execute (BNX2X_MCAST_CMD_X, see above)
//
    pub cmd): bnx2x_mcast_cmd,
//
// Fills the ramrod data during the RESTORE flow.
//
// @param bp
// @param o
// @param start_idx Registry index to start from
// @param rdata_idx Index in the ramrod data to start from
//
// @return -1 if we handled the whole registry or index of the last
// handled registry element.
//
    pub rdata_idx): *mut int start_bin, int,
    pub cmd): bnx2x_mcast_cmd,
    pub cmd): bnx2x_mcast_cmd,
// Checks if there are more mcast MACs to be set or a previous
// command is still pending.
//
    pub o): *mut *mut bool (check_pending)(struct bnx2x_mcast_obj,
//
// Set/Clear/Check SCHEDULED state of the object
//
    pub o): *mut *mut void (set_sched)(struct bnx2x_mcast_obj,
    pub o): *mut *mut void (clear_sched)(struct bnx2x_mcast_obj,
    pub o): *mut *mut bool (check_sched)(struct bnx2x_mcast_obj,
// Wait until all pending commands complete
    pub o): *mut *mut *mut int (wait_comp)(struct bnx2x bp, struct bnx2x_mcast_obj,
//
// Handle the internal object counters needed for proper
// commands handling. Checks that the provided parameters are
// feasible.
//
    pub cmd): bnx2x_mcast_cmd,
//
// Restore the values of internal counters in case of a failure.
//
    pub cmd): bnx2x_mcast_cmd,
    pub o): *mut *mut int (get_registry_size)(struct bnx2x_mcast_obj,
    pub n): *mut *mut *mut void (set_registry_size)(struct bnx2x_mcast_obj o, int,
}

// Credit handling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_credit_pool_obj {
// Current amount of credit in the pool
    pub credit: core::sync::atomic::AtomicI32,
// Maximum allowed credit. put() will check against it.
    pub pool_sz: c_int,
// Allocate a pool table statically.
//
// Currently the maximum allowed size is MAX_MAC_CREDIT_E2(272)
//
// The set bit in the table will mean that the entry is available.
//
    pub pool_mirror: [u64; BNX2X_POOL_VEC_SIZE],
// Base pool offset (initialized differently
    pub base_pool_offset: c_int,
//
// Get the next free pool entry.
//
// @return true if there was a free entry in the pool
//
    pub entry): *mut *mut *mut bool (get_entry)(struct bnx2x_credit_pool_obj o, int,
//
// Return the entry back to the pool.
//
// @return true if entry is legal and has been successfully
// returned to the pool.
//
    pub entry): *mut *mut *mut bool (put_entry)(struct bnx2x_credit_pool_obj o, int,
//
// Get the requested amount of credit from the pool.
//
// @param cnt Amount of requested credit
// @return true if the operation is successful
//
    pub cnt): *mut *mut *mut bool (get)(struct bnx2x_credit_pool_obj o, int,
//
// Returns the credit to the pool.
//
// @param cnt Amount of credit to return
// @return true if the operation is successful
//
    pub cnt): *mut *mut *mut bool (put)(struct bnx2x_credit_pool_obj o, int,
//
// Reads the current amount of credit.
//
    pub o): *mut *mut int (check)(struct bnx2x_credit_pool_obj,
}

// RSS configuration
// RSS_MODE bits are mutually exclusive
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_config_rss_params {
    pub rss_obj: *mut bnx2x_rss_config_obj,
// may have RAMROD_COMP_WAIT set only
    pub ramrod_flags: c_ulong,
// BNX2X_RSS_X bits
    pub rss_flags: c_ulong,
// Number hash bits to take into an account
    pub rss_result_mask: u8,
// Indirection table
    pub ind_table: [u8; T_ETH_INDIRECTION_TABLE_SIZE],
// RSS hash values
    pub rss_key: [u32; 10],
// valid only iff BNX2X_RSS_UPDATE_TOE is set
    pub toe_rss_bitmap: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_rss_config_obj {
    pub raw: bnx2x_raw_obj,
// RSS engine to use
    pub engine_id: u8,
// Last configured indirection table
    pub ind_table: [u8; T_ETH_INDIRECTION_TABLE_SIZE],
// flags for enabling 4-tupple hash on UDP
    pub udp_rss_v4: u8,
    pub udp_rss_v6: u8,
    pub p): *mut bnx2x_config_rss_params,
}

// Queue state update
// UPDATE command options
// Allowed Queue states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_q_state {
    BNX2X_Q_STATE_RESET,
    BNX2X_Q_STATE_INITIALIZED,
    BNX2X_Q_STATE_ACTIVE,
    BNX2X_Q_STATE_MULTI_COS,
    BNX2X_Q_STATE_MCOS_TERMINATED,
    BNX2X_Q_STATE_INACTIVE,
    BNX2X_Q_STATE_STOPPED,
    BNX2X_Q_STATE_TERMINATED,
    BNX2X_Q_STATE_FLRED,
    BNX2X_Q_STATE_MAX,
}

// Allowed Queue states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_q_logical_state {
    BNX2X_Q_LOGICAL_STATE_ACTIVE,
    BNX2X_Q_LOGICAL_STATE_STOPPED,
}

// Allowed commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_queue_cmd {
    BNX2X_Q_CMD_INIT,
    BNX2X_Q_CMD_SETUP,
    BNX2X_Q_CMD_SETUP_TX_ONLY,
    BNX2X_Q_CMD_DEACTIVATE,
    BNX2X_Q_CMD_ACTIVATE,
    BNX2X_Q_CMD_UPDATE,
    BNX2X_Q_CMD_UPDATE_TPA,
    BNX2X_Q_CMD_HALT,
    BNX2X_Q_CMD_CFC_DEL,
    BNX2X_Q_CMD_TERMINATE,
    BNX2X_Q_CMD_EMPTY,
    BNX2X_Q_CMD_MAX,
}

// queue SETUP + INIT flags
// Queue type options: queue type may be a combination of below.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_q_type {
// TODO: Consider moving both these flags into the init()
// ramrod params.
//
    BNX2X_Q_TYPE_HAS_RX,
    BNX2X_Q_TYPE_HAS_TX,
}

pub const BNX2X_PRIMARY_CID_INDEX: c_int = 0;

pub const BNX2X_MULTI_TX_COS_E2_E3A0: c_int = 2;
pub const BNX2X_MULTI_TX_COS_E3B0: c_int = 3;

// DMAE channel to be used by FW for timesync workaroun. A driver that sends
// timesync-related ramrods must not use this DMAE command ID.
//
pub const FW_DMAE_CMD_ID: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_init_params {
    pub flags: c_ulong,
    pub hc_rate: u16,
    pub fw_sb_id: u8,
    pub sb_cq_index: u8,
    pub tx: },
    pub flags: c_ulong,
    pub hc_rate: u16,
    pub fw_sb_id: u8,
    pub sb_cq_index: u8,
    pub rx: },
// CID context in the host memory
    pub cxts: [*mut eth_context; BNX2X_MULTI_TX_COS],
// maximum number of cos supported by hardware
    pub max_cos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_terminate_params {
// index within the tx_only cids of this queue object
    pub cid_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_cfc_del_params {
// index within the tx_only cids of this queue object
    pub cid_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_update_params {
    pub /: *mut *mut unsigned long update_flags; / BNX2X_Q_UPDATE_XX bits,
    pub def_vlan: u16,
    pub silent_removal_value: u16,
    pub silent_removal_mask: u16,
// index within the tx_only cids of this queue object
    pub cid_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_update_tpa_params {
    pub sge_map: dma_addr_t,
    pub update_ipv4: u8,
    pub update_ipv6: u8,
    pub max_tpa_queues: u8,
    pub max_sges_pkt: u8,
    pub complete_on_both_clients: u8,
    pub dont_verify_thr: u8,
    pub tpa_mode: u8,
    pub _pad: u8,
    pub sge_buff_sz: u16,
    pub max_agg_sz: u16,
    pub sge_pause_thr_low: u16,
    pub sge_pause_thr_high: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxq_pause_params {
    pub bd_th_lo: u16,
    pub bd_th_hi: u16,
    pub rcq_th_lo: u16,
    pub rcq_th_hi: u16,
    pub /: *mut *mut u16 sge_th_lo; / valid iff BNX2X_Q_FLG_TPA,
    pub /: *mut *mut u16 sge_th_hi; / valid iff BNX2X_Q_FLG_TPA,
    pub pri_map: u16,
}

// general
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_general_setup_params {
// valid iff BNX2X_Q_FLG_STATS
    pub stat_id: u8,
    pub spcl_id: u8,
    pub mtu: u16,
    pub cos: u8,
    pub fp_hsi: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_rxq_setup_params {
// dma
    pub dscr_map: dma_addr_t,
    pub sge_map: dma_addr_t,
    pub rcq_map: dma_addr_t,
    pub rcq_np_map: dma_addr_t,
    pub drop_flags: u16,
    pub buf_sz: u16,
    pub fw_sb_id: u8,
    pub cl_qzone_id: u8,
// valid iff BNX2X_Q_FLG_TPA
    pub tpa_agg_sz: u16,
    pub sge_buf_sz: u16,
    pub max_sges_pkt: u8,
    pub max_tpa_queues: u8,
    pub rss_engine_id: u8,
// valid iff BNX2X_Q_FLG_MCAST
    pub mcast_engine_id: u8,
    pub cache_line_log: u8,
    pub sb_cq_index: u8,
// valid iff BXN2X_Q_FLG_SILENT_VLAN_REM
    pub silent_removal_value: u16,
    pub silent_removal_mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_txq_setup_params {
// dma
    pub dscr_map: dma_addr_t,
    pub fw_sb_id: u8,
    pub sb_cq_index: u8,
    pub /: *mut *mut u8 cos; / valid iff BNX2X_Q_FLG_COS,
    pub traffic_type: u16,
// equals to the leading rss client id, used for TX classification
    pub tss_leading_cl_id: u8,
// valid iff BNX2X_Q_FLG_DEF_VLAN
    pub default_vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_setup_params {
    pub gen_params: bnx2x_general_setup_params,
    pub txq_params: bnx2x_txq_setup_params,
    pub rxq_params: bnx2x_rxq_setup_params,
    pub pause_params: rxq_pause_params,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_setup_tx_only_params {
    pub gen_params: bnx2x_general_setup_params,
    pub txq_params: bnx2x_txq_setup_params,
    pub flags: c_ulong,
// index within the tx_only cids of this queue object
    pub cid_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_state_params {
    pub q_obj: *mut bnx2x_queue_sp_obj,
// Current command
    pub cmd: bnx2x_queue_cmd,
// may have RAMROD_COMP_WAIT set only
    pub ramrod_flags: c_ulong,
// Params according to the current command
    pub update: bnx2x_queue_update_params,
    pub update_tpa: bnx2x_queue_update_tpa_params,
    pub setup: bnx2x_queue_setup_params,
    pub init: bnx2x_queue_init_params,
    pub tx_only: bnx2x_queue_setup_tx_only_params,
    pub terminate: bnx2x_queue_terminate_params,
    pub cfc_del: bnx2x_queue_cfc_del_params,
    pub params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_viflist_params {
    pub echo_res: u8,
    pub func_bit_map_res: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_queue_sp_obj {
    pub cids: [u32; BNX2X_MULTI_TX_COS],
    pub cl_id: u8,
    pub func_id: u8,
// number of traffic classes supported by queue.
// The primary connection of the queue supports the first traffic
// class. Any further traffic class is supported by a tx-only
// connection.
//
// Therefore max_cos is also a number of valid entries in the cids
// array.
//
    pub max_cos: u8,
    pub next_tx_only: u8 num_tx_only,,
    pub next_state: bnx2x_q_state state,,
// bits from enum bnx2x_q_type
    pub type: c_ulong,
// BNX2X_Q_CMD_XX bits. This object implements "one
// pending" paradigm but for debug and tracing purposes it's
// more convenient to have different bits for different
// commands.
//
    pub pending: c_ulong,
// Buffer to use as a ramrod data and its mapping
    pub rdata: *mut c_void,
    pub rdata_mapping: dma_addr_t,
//
// Performs one state change according to the given parameters.
//
// @return 0 in case of success and negative value otherwise.
//
    pub params): *mut bnx2x_queue_state_params,
//
// Sets the pending bit according to the requested transition.
//
    pub params): *mut bnx2x_queue_state_params,
//
// Checks that the requested state transition is legal.
//
    pub params): *mut bnx2x_queue_state_params,
//
// Completes the pending command.
//
    pub bnx2x_queue_cmd): enum,
    pub cmd): bnx2x_queue_cmd,
}

// Function state update
// UPDATE command options
// Allowed Function states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_func_state {
    BNX2X_F_STATE_RESET,
    BNX2X_F_STATE_INITIALIZED,
    BNX2X_F_STATE_STARTED,
    BNX2X_F_STATE_TX_STOPPED,
    BNX2X_F_STATE_MAX,
}

// Allowed Function commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_func_cmd {
    BNX2X_F_CMD_HW_INIT,
    BNX2X_F_CMD_START,
    BNX2X_F_CMD_STOP,
    BNX2X_F_CMD_HW_RESET,
    BNX2X_F_CMD_AFEX_UPDATE,
    BNX2X_F_CMD_AFEX_VIFLISTS,
    BNX2X_F_CMD_TX_STOP,
    BNX2X_F_CMD_TX_START,
    BNX2X_F_CMD_SWITCH_UPDATE,
    BNX2X_F_CMD_SET_TIMESYNC,
    BNX2X_F_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_hw_init_params {
// A load phase returned by MCP.
//
// May be:
// FW_MSG_CODE_DRV_LOAD_COMMON_CHIP
// FW_MSG_CODE_DRV_LOAD_COMMON
// FW_MSG_CODE_DRV_LOAD_PORT
// FW_MSG_CODE_DRV_LOAD_FUNCTION
//
    pub load_phase: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_hw_reset_params {
// A load phase returned by MCP.
//
// May be:
// FW_MSG_CODE_DRV_LOAD_COMMON_CHIP
// FW_MSG_CODE_DRV_LOAD_COMMON
// FW_MSG_CODE_DRV_LOAD_PORT
// FW_MSG_CODE_DRV_LOAD_FUNCTION
//
    pub reset_phase: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_start_params {
// Multi Function mode:
// - Single Function
// - Switch Dependent
// - Switch Independent
//
    pub mf_mode: u16,
// Switch Dependent mode outer VLAN tag
    pub sd_vlan_tag: u16,
// Function cos mode
    pub network_cos_mode: u8,
// UDP dest port for VXLAN
    pub vxlan_dst_port: u16,
// UDP dest port for Geneve
    pub geneve_dst_port: u16,
// Enable inner Rx classifications for L2GRE packets
    pub inner_clss_l2gre: u8,
// Enable inner Rx classifications for L2-Geneve packets
    pub inner_clss_l2geneve: u8,
// Enable inner Rx classification for vxlan packets
    pub inner_clss_vxlan: u8,
// Enable RSS according to inner header
    pub inner_rss: u8,
// Allows accepting of packets failing MF classification, possibly
// only matching a given ethertype
//
    pub class_fail: u8,
    pub class_fail_ethtype: u16,
// Override priority of output packets
    pub sd_vlan_force_pri: u8,
    pub sd_vlan_force_pri_val: u8,
// Replace vlan's ethertype
    pub sd_vlan_eth_type: u16,
// Prevent inner vlans from being added by FW
    pub no_added_tags: u8,
// Inner-to-Outer vlan priority mapping
    pub c2s_pri: [u8; MAX_VLAN_PRIORITIES],
    pub c2s_pri_default: u8,
    pub c2s_pri_valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_switch_update_params {
    pub /: *mut *mut unsigned long changes; / BNX2X_F_UPDATE_XX bits,
    pub vlan: u16,
    pub vlan_eth_type: u16,
    pub vlan_force_prio: u8,
    pub vxlan_dst_port: u16,
    pub geneve_dst_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_afex_update_params {
    pub vif_id: u16,
    pub afex_default_vlan: u16,
    pub allowed_priorities: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_afex_viflists_params {
    pub vif_list_index: u16,
    pub func_bit_map: u8,
    pub afex_vif_list_command: u8,
    pub func_to_clear: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_tx_start_params {
    pub traffic_type_to_priority_cos: [priority_cos; MAX_TRAFFIC_TYPES],
    pub dcb_enabled: u8,
    pub dcb_version: u8,
    pub dont_add_pri_0_en: u8,
    pub dcb_outer_pri: [u8; MAX_TRAFFIC_TYPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_set_timesync_params {
// Reset, set or keep the current drift value
    pub drift_adjust_cmd: u8,
// Dec, inc or keep the current offset
    pub offset_cmd: u8,
// Drift value direction
    pub add_sub_drift_adjust_value: u8,
// Drift, period and offset values to be used according to the commands
// above.
//
    pub drift_adjust_value: u8,
    pub drift_adjust_period: u32,
    pub offset_delta: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_state_params {
    pub f_obj: *mut bnx2x_func_sp_obj,
// Current command
    pub cmd: bnx2x_func_cmd,
// may have RAMROD_COMP_WAIT set only
    pub ramrod_flags: c_ulong,
// Params according to the current command
    pub hw_init: bnx2x_func_hw_init_params,
    pub hw_reset: bnx2x_func_hw_reset_params,
    pub start: bnx2x_func_start_params,
    pub switch_update: bnx2x_func_switch_update_params,
    pub afex_update: bnx2x_func_afex_update_params,
    pub afex_viflists: bnx2x_func_afex_viflists_params,
    pub tx_start: bnx2x_func_tx_start_params,
    pub set_timesync: bnx2x_func_set_timesync_params,
    pub params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_sp_drv_ops {
// Init tool + runtime initialization:
// - Common Chip
// - Common (per Path)
// - Port
// - Function phases
//
    pub bp): *mut *mut int (init_hw_cmn_chip)(struct bnx2x,
    pub bp): *mut *mut int (init_hw_cmn)(struct bnx2x,
    pub bp): *mut *mut int (init_hw_port)(struct bnx2x,
    pub bp): *mut *mut int (init_hw_func)(struct bnx2x,
// Reset Function HW: Common, Port, Function phases.
    pub bp): *mut *mut void (reset_hw_cmn)(struct bnx2x,
    pub bp): *mut *mut void (reset_hw_port)(struct bnx2x,
    pub bp): *mut *mut void (reset_hw_func)(struct bnx2x,
// Init/Free GUNZIP resources
    pub bp): *mut *mut int (gunzip_init)(struct bnx2x,
    pub bp): *mut *mut void (gunzip_end)(struct bnx2x,
// Prepare/Release FW resources
    pub bp): *mut *mut int (init_fw)(struct bnx2x,
    pub bp): *mut *mut void (release_fw)(struct bnx2x,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_func_sp_obj {
    pub next_state: bnx2x_func_state state,,
// BNX2X_FUNC_CMD_XX bits. This object implements "one
// pending" paradigm but for debug and tracing purposes it's
// more convenient to have different bits for different
// commands.
//
    pub pending: c_ulong,
// Buffer to use as a ramrod data and its mapping
    pub rdata: *mut c_void,
    pub rdata_mapping: dma_addr_t,
// Buffer to use as a afex ramrod data and its mapping.
// This can't be same rdata as above because afex ramrod requests
// can arrive to the object in parallel to other ramrod requests.
//
    pub afex_rdata: *mut c_void,
    pub afex_rdata_mapping: dma_addr_t,
// this mutex validates that when pending flag is taken, the next
// ramrod to be sent will be the one set the pending bit
//
    pub one_pending_mutex: mutex,
// Driver interface
    pub drv: *mut bnx2x_func_sp_drv_ops,
//
// Performs one state change according to the given parameters.
//
// @return 0 in case of success and negative value otherwise.
//
    pub params): *mut bnx2x_func_state_params,
//
// Checks that the requested state transition is legal.
//
    pub params): *mut bnx2x_func_state_params,
//
// Completes the pending command.
//
    pub cmd): bnx2x_func_cmd,
    pub cmd): bnx2x_func_cmd,
}

// Interfaces
// Queueable objects set
#[repr(C)]
#[derive(Copy, Clone)]
pub union bnx2x_qable_obj {
    pub vlan_mac: bnx2x_vlan_mac_obj,
}

// Function state update
// Queue State
// VLAN-MAC
// RX MODE
//
// bnx2x_config_rx_mode - Send and RX_MODE ramrod according to the provided parameters.
//
// @p: Command parameters
//
// Return: 0 - if operation was successful and there is no pending completions,
// positive number - if there are pending completions,
// negative - if there were errors
//
// MULTICASTS
//
// bnx2x_config_mcast - Configure multicast MACs list.
//
// @cmd: command to execute: BNX2X_MCAST_CMD_X
//
// May configure a new list
// provided in p->mcast_list (BNX2X_MCAST_CMD_ADD), clean up
// (BNX2X_MCAST_CMD_DEL) or restore (BNX2X_MCAST_CMD_RESTORE) a current
// configuration, continue to execute the pending commands
// (BNX2X_MCAST_CMD_CONT).
//
// If previous command is still pending or if number of MACs to
// configure is more that maximum number of MACs in one command,
// the current command will be enqueued to the tail of the
// pending commands list.
//
// Return: 0 is operation was successful and there are no pending completions,
// negative if there were errors, positive if there are pending
// completions.
//
// CREDIT POOL
// RSS CONFIGURATION
//
// bnx2x_config_rss - Updates RSS configuration according to provided parameters
//
// Return: 0 in case of success
//
// bnx2x_get_rss_ind_table - Return the current ind_table configuration.
//
// @ind_table: buffer to fill with the current indirection
// table content. Should be at least
// T_ETH_INDIRECTION_TABLE_SIZE bytes long.
//

