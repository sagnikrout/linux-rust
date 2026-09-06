//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/sta.h
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
//
// Copyright (C) 2012-2014, 2018-2025 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2015-2016 Intel Deutschland GmbH
//

// Macro flag: #define __sta_h__

//
// DOC: DQA - Dynamic Queue Allocation -introduction
//
// Dynamic Queue Allocation (AKA "DQA") is a feature implemented in iwlwifi
// driver to allow dynamic allocation of queues on-demand, rather than allocate
// them statically ahead of time. Ideally, we would like to allocate one queue
// per RA/TID, thus allowing an AP - for example - to send BE traffic to STA2
// even if it also needs to send traffic to a sleeping STA1, without being
// blocked by the sleeping station.
//
// Although the queues in DQA mode are dynamically allocated, there are still
// some queues that are statically allocated:
// TXQ #0 - command queue
// TXQ #1 - aux frames
// TXQ #2 - P2P device frames
// TXQ #3 - P2P GO/SoftAP GCAST/BCAST frames
// TXQ #4 - BSS DATA frames queue
// TXQ #5-8 - Non-QoS and MGMT frames queue pool
// TXQ #9 - P2P GO/SoftAP probe responses
// TXQ #10-31 - DATA frames queue pool
// The queues are dynamically taken from either the MGMT frames queue pool or
// the DATA frames one. See the %iwl_mvm_dqa_txq for more information on every
// queue.
//
// When a frame for a previously unseen RA/TID comes in, it needs to be deferred
// until a queue is allocated for it, and only then can be TXed. Therefore, it
// is placed into %iwl_mvm_tid_data.deferred_tx_frames, and a worker called
// %mvm->add_stream_wk later allocates the queues and TXes the deferred frames.
//
// For convenience, MGMT is considered as if it has TID=8, and go to the MGMT
// queues in the pool. If there is no longer a free MGMT queue to allocate, a
// queue will be allocated from the DATA pool instead. Since QoS NDPs can create
// a problem for aggregations, they too will use a MGMT queue.
//
// When adding a STA, a DATA queue is reserved for it so that it can TX from
// it. If no such free queue exists for reserving, the STA addition will fail.
//
// If the DATA queue pool gets exhausted, no new STA will be accepted, and if a
// new RA/TID comes in for an existing STA, one of the STA's queues will become
// shared and will serve more than the single TID (but always for the same RA!).
//
// When a RA/TID needs to become aggregated, no new queue is required to be
// allocated, only mark the queue as aggregated via the ADD_STA command. Note,
// however, that a shared queue cannot be aggregated, and only after the other
// TIDs become inactive and are removed - only then can the queue be
// reconfigured and become aggregated.
//
// When removing a station, its queues are returned to the pool for reuse. Here
// we also need to make sure that we are synced with the worker thread that TXes
// the deferred frames so we don't get into a situation where the queues are
// removed and then the worker puts deferred frames onto the released queues or
// tries to allocate new queues for a STA we don't need anymore.
//
// DOC: station table - introduction
//
// The station table is a list of data structure that reprensent the stations.
// In STA/P2P client mode, the driver will hold one station for the AP/ GO.
// In GO/AP mode, the driver will have as many stations as associated clients.
// All these stations are reflected in the fw's station table. The driver
// keeps the fw's station table up to date with the ADD_STA command. Stations
// can be removed by the REMOVE_STA command.
//
// All the data related to a station is held in the structure %iwl_mvm_sta
// which is embed in the mac80211's %ieee80211_sta (in the drv_priv) area.
// This data includes the index of the station in the fw, per tid information
// (sequence numbers, Block-ack state machine, etc...). The stations are
// created and deleted by the %sta_state callback from %ieee80211_ops.
//
// The driver holds a map: %fw_id_to_mac_id that allows to fetch a
// %ieee80211_sta (and the %iwl_mvm_sta embedded into it) based on a fw
// station index. That way, the driver is able to get the tid related data in
// O(1) in time sensitive paths (Tx / Tx response / BA notification). These
// paths are triggered by the fw, and the driver needs to get a pointer to the
// %ieee80211 structure. This map helps to get that pointer quickly.
//
// DOC: station table - locking
//
// As stated before, the station is created / deleted by mac80211's %sta_state
// callback from %ieee80211_ops which can sleep. The next paragraph explains
// the locking of a single stations, the next ones relates to the station
// table.
//
// The station holds the sequence number per tid. So this data needs to be
// accessed in the Tx path (which is softIRQ). It also holds the Block-Ack
// information (the state machine / and the logic that checks if the queues
// were drained), so it also needs to be accessible from the Tx response flow.
// In short, the station needs to be access from sleepable context as well as
// from tasklets, so the station itself needs a spinlock.
//
// The writers of %fw_id_to_mac_id map are serialized by the global mutex of
// the mvm op_mode. This is possible since %sta_state can sleep.
// The pointers in this map are RCU protected, hence we won't replace the
// station while we have Tx / Tx response / BA notification running.
//
// If a station is deleted while it still has packets in its A-MPDU queues,
// then the reclaim flow will notice that there is no station in the map for
// sta_id and it will dump the responses.
//
// DOC: station table - internal stations
//
// The FW needs a few internal stations that are not reflected in
// mac80211, such as broadcast station in AP / GO mode, or AUX sta for
// scanning and P2P device (during the GO negotiation).
// For these kind of stations we have %iwl_mvm_int_sta struct which holds the
// data relevant for them from both %iwl_mvm_sta and %ieee80211_sta.
// Usually the data for these stations is static, so no locking is required,
// and no TID data as this is also not needed.
// One thing to note, is that these stations have an ID in the fw, but not
// in mac80211. In order to "reserve" them a sta_id in %fw_id_to_mac_id
// we fill ERR_PTR(-EINVAL) in this mapping and all other dereferencing of
// pointers from this mapping need to check that the value is not error
// or NULL.
//
// Currently there is only one auxiliary station for scanning, initialized
// on init.
//
// DOC: station table - AP Station in STA mode
//
// %iwl_mvm_vif includes the index of the AP station in the fw's STA table:
// %ap_sta_id. To get the point to the corresponding %ieee80211_sta,
// &fw_id_to_mac_id can be used. Due to the way the fw works, we must not remove
// the AP station from the fw before setting the MAC context as unassociated.
// Hence, %fw_id_to_mac_id[%ap_sta_id] will be NULLed when the AP station is
// removed by mac80211, but the station won't be removed in the fw until the
// VIF is set as unassociated. Then, %ap_sta_id will be invalidated.
//
// DOC: station table - Drain vs. Flush
//
// Flush means that all the frames in the SCD queue are dumped regardless the
// station to which they were sent. We do that when we disassociate and before
// we remove the STA of the AP. The flush can be done synchronously against the
// fw.
// Drain means that the fw will drop all the frames sent to a specific station.
// This is useful when a client (if we are IBSS / GO or AP) disassociates.
//
// DOC: station table - fw restart
//
// When the fw asserts, or we have any other issue that requires to reset the
// driver, we require mac80211 to reconfigure the driver. Since the private
// data of the stations is embed in mac80211's %ieee80211_sta, that data will
// not be zeroed and needs to be reinitialized manually.
// %IWL_MVM_STATUS_IN_HW_RESTART is set during restart and that will hint us
// that we must not allocate a new sta_id but reuse the previous one. This
// means that the stations being re-added after the reset will have the same
// place in the fw as before the reset. We do need to zero the %fw_id_to_mac_id
// map, since the stations aren't in the fw any more. Internal stations that
// are not added by mac80211 will be re-added in the init flow that is called
// after the restart: mac80211 call's %iwl_mvm_mac_start which calls to
// %iwl_mvm_up.
//
// DOC: AP mode - PS
//
// When a station is asleep, the fw will set it as "asleep". All frames on
// shared queues (i.e. non-aggregation queues) to that station will be dropped
// by the fw (%TX_STATUS_FAIL_DEST_PS failure code).
//
// AMPDUs are in a separate queue that is stopped by the fw. We just need to
// let mac80211 know when there are frames in these queues so that it can
// properly handle trigger frames.
//
// When a trigger frame is received, mac80211 tells the driver to send frames
// from the AMPDU queues or sends frames to non-aggregation queues itself,
// depending on which ACs are delivery-enabled and what TID has frames to
// transmit. Note that mac80211 has all the knowledge since all the non-agg
// frames are buffered / filtered, and the driver tells mac80211 about agg
// frames). The driver needs to tell the fw to let frames out even if the
// station is asleep. This is done by %iwl_mvm_sta_modify_sleep_tx_count.
//
// When we receive a frame from that station with PM bit unset, the driver
// needs to let the fw know that this station isn't asleep any more. This is
// done by %iwl_mvm_sta_modify_ps_wake in response to mac80211 signaling the
// station's wakeup.
//
// For a GO, the Service Period might be cut short due to an absence period
// of the GO. In this (and all other cases) the firmware notifies us with the
// EOSP_NOTIFICATION, and we notify mac80211 of that. Further frames that we
// already sent to the device will be rejected again.
//
// See also "AP support for powersaving clients" in mac80211.h.
//
// enum iwl_mvm_agg_state - aggregation session state
//
// The state machine of the BA agreement establishment / tear down.
// These states relate to a specific RA / TID.
//
// @IWL_AGG_OFF: aggregation is not used
// @IWL_AGG_QUEUED: aggregation start work has been queued
// @IWL_AGG_STARTING: aggregation are starting (between start and oper)
// @IWL_AGG_ON: aggregation session is up
// @IWL_EMPTYING_HW_QUEUE_ADDBA: establishing a BA session - waiting for the
// HW queue to be empty from packets for this RA /TID.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_agg_state {
    IWL_AGG_OFF = 0,
    IWL_AGG_QUEUED,
    IWL_AGG_STARTING,
    IWL_AGG_ON,
    IWL_EMPTYING_HW_QUEUE_ADDBA,
}

//
// struct iwl_mvm_tid_data - holds the states for each RA / TID
// @seq_number: the next WiFi sequence number to use
// @next_reclaimed: the WiFi sequence number of the next packet to be acked.
// This is basically (last acked packet++).
// @rate_n_flags: Rate at which Tx was attempted. Holds the data between the
// Tx response (TX_CMD), and the block ack notification (COMPRESSED_BA).
// @lq_color: the color of the LQ command as it appears in tx response.
// @amsdu_in_ampdu_allowed: true if A-MSDU in A-MPDU is allowed.
// @state: state of the BA agreement establishment / tear down.
// @txq_id: Tx queue used by the BA session / DQA
// @ssn: the first packet to be sent in AGG HW queue in Tx AGG start flow, or
// the first packet to be sent in legacy HW queue in Tx AGG stop flow.
// Basically when next_reclaimed reaches ssn, we can tell mac80211 that
// we are ready to finish the Tx AGG stop / start flow.
// @tx_time: medium time consumed by this A-MPDU
// @tpt_meas_start: time of the throughput measurements start, is reset every HZ
// @tx_count_last: number of frames transmitted during the last second
// @tx_count: counts the number of frames transmitted since the last reset of
// tpt_meas_start
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_tid_data {
    pub seq_number: u16,
    pub next_reclaimed: u16,
// The rest is Tx AGG related
    pub rate_n_flags: __le32,
    pub lq_color: u8,
    pub amsdu_in_ampdu_allowed: bool,
    pub state: iwl_mvm_agg_state,
    pub txq_id: u16,
    pub ssn: u16,
    pub tx_time: u16,
    pub tpt_meas_start: c_ulong,
    pub tx_count_last: u32,
    pub tx_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_key_pn {
    pub rcu_head: rcu_head,
    pub pn: [u8; IWL_MAX_TID_COUNT][IEEE80211_CCMP_PN_LEN],
    pub q: [} ____cacheline_aligned_in_smp; ],
}

//
// enum iwl_mvm_rxq_notif_type - Internal message identifier
//
// @IWL_MVM_RXQ_EMPTY: empty sync notification
// @IWL_MVM_RXQ_NOTIF_DEL_BA: notify RSS queues of delBA
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mvm_rxq_notif_type {
    IWL_MVM_RXQ_EMPTY,
    IWL_MVM_RXQ_NOTIF_DEL_BA,
}

//
// struct iwl_mvm_internal_rxq_notif - Internal representation of the data sent
// in &iwl_rxq_sync_cmd. Should be DWORD aligned.
// FW is agnostic to the payload, so there are no endianity requirements.
//
// @type: value from &iwl_mvm_rxq_notif_type
// @sync: ctrl path is waiting for all notifications to be received
// @cookie: internal cookie to identify old notifications
// @data: payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_internal_rxq_notif {
    pub type: u16,
    pub sync: u16,
    pub cookie: u32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_delba_data {
    pub baid: u32,
    pub __packed: },
//
// struct iwl_mvm_rxq_dup_data - per station per rx queue data
// @last_seq: last sequence per tid for duplicate packet detection
// @last_sub_frame: last subframe packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_rxq_dup_data {
    pub 1]: __le16 last_seq[IWL_MAX_TID_COUNT +,
    pub 1]: u8 last_sub_frame[IWL_MAX_TID_COUNT +,
    pub ____cacheline_aligned_in_smp: },
//
// struct iwl_mvm_link_sta - link specific parameters of a station
// @rcu_head: used for freeing the data
// @sta_id: the index of the station in the fw
// @lq_sta: holds rate scaling data, either for the case when RS is done in
// the driver - %rs_drv or in the FW - %rs_fw.
// @orig_amsdu_len: used to save the original amsdu_len when it is changed via
// debugfs.  If it's set to 0, it means that it is it's not set via
// debugfs.
// @avg_energy: energy as reported by FW statistics notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_link_sta {
    pub rcu_head: rcu_head,
    pub sta_id: u32,
    pub rs_fw: iwl_lq_sta_rs_fw,
    pub rs_drv: iwl_lq_sta,
    pub lq_sta: },
    pub orig_amsdu_len: u16,
    pub avg_energy: u8,
}

//
// struct iwl_mvm_sta - representation of a station in the driver
// @vif: the interface the station belongs to
// @tfd_queue_msk: the tfd queues used by the station
// @mac_id_n_color: the MAC context this station is linked to
// @tid_disable_agg: bitmap: if bit(tid) is set, the fw won't send ampdus for
// tid.
// @sta_type: station type
// @authorized: indicates station is authorized
// @sta_state: station state according to enum %ieee80211_sta_state
// @bt_reduced_txpower: is reduced tx power enabled for this station
// @next_status_eosp: the next reclaimed packet is a PS-Poll response and
// we need to signal the EOSP
// @lock: lock to protect the whole struct. Since %tid_data is access from Tx
// and from Tx response flow, it needs a spinlock.
// @tid_data: per tid data + mgmt. Look at %iwl_mvm_tid_data.
// @tid_to_baid: a simple map of TID to baid
// @vif: a vif pointer
// @reserved_queue: the queue reserved for this STA for DQA purposes
// Every STA has is given one reserved queue to allow it to operate. If no
// such queue can be guaranteed, the STA addition will fail.
// @tx_protection: reference counter for controlling the Tx protection.
// @tt_tx_protection: is thermal throttling enable Tx protection?
// @disable_tx: is tx to this STA disabled?
// @amsdu_enabled: bitmap of TX AMSDU allowed TIDs.
// In case TLC offload is not active it is either 0xFFFF or 0.
// @max_amsdu_len: max AMSDU length
// @sleeping: indicates the station is sleeping (when not offloaded to FW)
// @agg_tids: bitmap of tids whose status is operational aggregated (IWL_AGG_ON)
// @sleeping: sta sleep transitions in power management
// @sleep_tx_count: the number of frames that we told the firmware to let out
// even when that station is asleep. This is useful in case the queue
// gets empty before all the frames were sent, which can happen when
// we are sending frames from an AMPDU queue and there was a hole in
// the BA window. To be used for UAPSD only.
// @ptk_pn: per-queue PTK PN data structures
// @dup_data: per queue duplicate packet detection data
// @tx_ant: the index of the antenna to use for data tx to this station. Only
// used during connection establishment (e.g. for the 4 way handshake
// exchange).
// @pairwise_cipher: used to feed iwlmei upon authorization
// @deflink: the default link station, for non-MLO STA, all link specific data
// is accessed via deflink (or link[0]). For MLO, it will hold data of the
// first added link STA.
// @link: per link sta entries. For non-MLO only link[0] holds data. For MLO,
// link[0] points to deflink and link[link_id] is allocated when new link
// sta is added.
//
// When mac80211 creates a station it reserves some space (hw->sta_data_size)
// in the structure for use by driver. This structure is placed in that
// space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_sta {
    pub tfd_queue_msk: u32,
    pub mac_id_n_color: u32,
    pub tid_disable_agg: u16,
    pub sta_type: u8,
    pub sta_state: ieee80211_sta_state,
    pub bt_reduced_txpower: bool,
    pub next_status_eosp: bool,
    pub authorized: bool,
    pub lock: spinlock_t,
    pub 1]: iwl_mvm_tid_data tid_data[IWL_MAX_TID_COUNT +,
    pub tid_to_baid: [u8; IWL_MAX_TID_COUNT],
    pub vif: *mut ieee80211_vif,
    pub ptk_pn: [*mut iwl_mvm_key_pn __rcu; 4],
    pub dup_data: *mut iwl_mvm_rxq_dup_data,
    pub reserved_queue: u8,
// Temporary, until the new TLC will control the Tx protection
    pub tx_protection: i8,
    pub tt_tx_protection: bool,
    pub disable_tx: bool,
    pub amsdu_enabled: u16,
    pub max_amsdu_len: u16,
    pub sleeping: bool,
    pub agg_tids: u8,
    pub sleep_tx_count: u8,
    pub tx_ant: u8,
    pub pairwise_cipher: u32,
    pub deflink: iwl_mvm_link_sta,
    pub link: [*mut iwl_mvm_link_sta __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
}

extern "C" {
    pub fn iwl_mvm_tid_queued(mvm: *mut iwl_mvm, tid_data: *mut iwl_mvm_tid_data) -> u16;
}
//
// struct iwl_mvm_int_sta - representation of an internal station (auxiliary or
// broadcast)
// @sta_id: the index of the station in the fw (will be replaced by id_n_color)
// @type: station type
// @tfd_queue_msk: the tfd queues used by the station
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_int_sta {
    pub sta_id: u32,
    pub type: u8,
    pub tfd_queue_msk: u32,
}

//
// iwl_mvm_sta_send_to_fw - Send the STA info to the FW.
//
// @mvm: the iwl_mvm* to use
// @sta: the STA
// @update: this is true if the FW is being updated about a STA it already knows
// about. Otherwise (if this is a new STA), this should be false.
// @flags: if update==true, this marks what is being changed via ORs of values
// from enum iwl_sta_modify_flag. Otherwise, this is ignored.
// Return: negative error code or 0 on success
//
extern "C" {
    pub fn iwl_mvm_find_free_sta_id(mvm: *mut iwl_mvm, iftype: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_sta_send_to_fw(_arg: mvm, _arg: sta, _arg: true, _arg: 0) -> return;
}
// AMPDU
extern "C" {
    pub fn iwl_mvm_add_aux_sta(mvm: *mut iwl_mvm, lmac_id: u32) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_rm_aux_sta(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_alloc_bcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_send_add_bcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_add_p2p_bcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_send_rm_bcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_rm_p2p_bcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_add_mcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_rm_mcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_dealloc_bcast_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mvm_dealloc_int_sta(mvm: *mut iwl_mvm, sta: *mut iwl_mvm_int_sta);
}
extern "C" {
    pub fn iwl_mvm_add_snif_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_rm_snif_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_dealloc_snif_sta(mvm: *mut iwl_mvm);
}
extern "C" {
    pub fn iwl_mvm_csa_client_absent(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn iwl_mvm_sta_ensure_queue(mvm: *mut iwl_mvm, txq: *mut ieee80211_txq) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_add_new_dqa_stream_wk(wk: *mut work_struct);
}
// Queues
// Sta state
//
// struct iwl_mvm_sta_state_ops - callbacks for the sta_state() ops
//
// Since the only difference between both MLD and
// non-MLD versions of sta_state() is these function calls,
// each version will send its specific function calls to
// %iwl_mvm_mac_sta_state_common().
//
// @add_sta: pointer to the function that adds a new sta
// @update_sta: pointer to the function that updates a sta
// @rm_sta: pointer to the functions that removes a sta
// @mac_ctxt_changed: pointer to the function that handles a change in mac ctxt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mvm_sta_state_ops {
    pub sta): *mut ieee80211_sta,
    pub sta): *mut ieee80211_sta,
    pub sta): *mut ieee80211_sta,
    pub force_assoc_off): bool,
}

// New MLD STA related APIs
// STA
extern "C" {
    pub fn iwl_mvm_mld_add_aux_sta(mvm: *mut iwl_mvm, lmac_id: u32) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mld_rm_snif_sta(mvm: *mut iwl_mvm, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mld_rm_aux_sta(mvm: *mut iwl_mvm) -> c_int;
}
extern "C" {
    pub fn iwl_mvm_mld_rm_sta_id(mvm: *mut iwl_mvm, sta_id: u8) -> c_int;
}
// Queues
