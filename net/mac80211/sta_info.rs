//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/sta_info.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2002-2005, Devicescape Software, Inc.
// Copyright 2013-2014  Intel Mobile Communications GmbH
// Copyright(c) 2015-2017 Intel Deutschland GmbH
// Copyright(c) 2020-2026 Intel Corporation
//

//
// enum ieee80211_sta_info_flags - Stations flags
//
// These flags are used with &struct sta_info's @flags member, but
// only indirectly with set_sta_flag() and friends.
//
// @WLAN_STA_AUTH: Station is authenticated.
// @WLAN_STA_ASSOC: Station is associated.
// @WLAN_STA_PS_STA: Station is in power-save mode
// @WLAN_STA_AUTHORIZED: Station is authorized to send/receive traffic.
// This bit is always checked so needs to be enabled for all stations
// when virtual port control is not in use.
// @WLAN_STA_SHORT_PREAMBLE: Station is capable of receiving short-preamble
// frames.
// @WLAN_STA_WDS: Station is one of our WDS peers.
// @WLAN_STA_CLEAR_PS_FILT: Clear PS filter in hardware (using the
// IEEE80211_TX_CTL_CLEAR_PS_FILT control flag) when the next
// frame to this station is transmitted.
// @WLAN_STA_MFP: Management frame protection is used with this STA.
// @WLAN_STA_BLOCK_BA: Used to deny ADDBA requests (both TX and RX)
// during suspend/resume and station removal.
// @WLAN_STA_PS_DRIVER: driver requires keeping this station in
// power-save mode logically to flush frames that might still
// be in the queues
// @WLAN_STA_PSPOLL: Station sent PS-poll while driver was keeping
// station in power-save mode, reply when the driver unblocks.
// @WLAN_STA_TDLS_PEER: Station is a TDLS peer.
// @WLAN_STA_TDLS_PEER_AUTH: This TDLS peer is authorized to send direct
// packets. This means the link is enabled.
// @WLAN_STA_TDLS_INITIATOR: We are the initiator of the TDLS link with this
// station.
// @WLAN_STA_TDLS_CHAN_SWITCH: This TDLS peer supports TDLS channel-switching
// @WLAN_STA_TDLS_OFF_CHANNEL: The local STA is currently off-channel with this
// TDLS peer
// @WLAN_STA_TDLS_WIDER_BW: This TDLS peer supports working on a wider bw on
// the BSS base channel.
// @WLAN_STA_UAPSD: Station requested unscheduled SP while driver was
// keeping station in power-save mode, reply when the driver
// unblocks the station.
// @WLAN_STA_SP: Station is in a service period, so don't try to
// reply to other uAPSD trigger frames or PS-Poll.
// @WLAN_STA_4ADDR_EVENT: 4-addr event was already sent for this frame.
// @WLAN_STA_INSERTED: This station is inserted into the hash table.
// @WLAN_STA_RATE_CONTROL: rate control was initialized for this station.
// @WLAN_STA_TOFFSET_KNOWN: toffset calculated for this station is valid.
// @WLAN_STA_MPSP_OWNER: local STA is owner of a mesh Peer Service Period.
// @WLAN_STA_MPSP_RECIPIENT: local STA is recipient of a MPSP.
// @WLAN_STA_PS_DELIVER: station woke up, but we're still blocking TX
// until pending frames are delivered
// @WLAN_STA_USES_ENCRYPTION: This station was configured for encryption,
// so drop all packets without a key later.
// @WLAN_STA_DECAP_OFFLOAD: This station uses rx decap offload
//
// @NUM_WLAN_STA_FLAGS: number of defined flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_sta_info_flags {
    WLAN_STA_AUTH,
    WLAN_STA_ASSOC,
    WLAN_STA_PS_STA,
    WLAN_STA_AUTHORIZED,
    WLAN_STA_SHORT_PREAMBLE,
    WLAN_STA_WDS,
    WLAN_STA_CLEAR_PS_FILT,
    WLAN_STA_MFP,
    WLAN_STA_BLOCK_BA,
    WLAN_STA_PS_DRIVER,
    WLAN_STA_PSPOLL,
    WLAN_STA_TDLS_PEER,
    WLAN_STA_TDLS_PEER_AUTH,
    WLAN_STA_TDLS_INITIATOR,
    WLAN_STA_TDLS_CHAN_SWITCH,
    WLAN_STA_TDLS_OFF_CHANNEL,
    WLAN_STA_TDLS_WIDER_BW,
    WLAN_STA_UAPSD,
    WLAN_STA_SP,
    WLAN_STA_4ADDR_EVENT,
    WLAN_STA_INSERTED,
    WLAN_STA_RATE_CONTROL,
    WLAN_STA_TOFFSET_KNOWN,
    WLAN_STA_MPSP_OWNER,
    WLAN_STA_MPSP_RECIPIENT,
    WLAN_STA_PS_DELIVER,
    WLAN_STA_USES_ENCRYPTION,
    WLAN_STA_DECAP_OFFLOAD,

    NUM_WLAN_STA_FLAGS,
}

pub const HT_AGG_MAX_RETRIES: c_int = 15;
pub const HT_AGG_BURST_RETRIES: c_int = 3;

pub const HT_AGG_STATE_DRV_READY: c_int = 0;
pub const HT_AGG_STATE_RESPONSE_RECEIVED: c_int = 1;
pub const HT_AGG_STATE_OPERATIONAL: c_int = 2;
pub const HT_AGG_STATE_STOPPING: c_int = 3;
pub const HT_AGG_STATE_WANT_START: c_int = 4;
pub const HT_AGG_STATE_WANT_STOP: c_int = 5;
pub const HT_AGG_STATE_START_CB: c_int = 6;
pub const HT_AGG_STATE_STOP_CB: c_int = 7;
pub const HT_AGG_STATE_SENT_ADDBA: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_agg_stop_reason {
    AGG_STOP_DECLINED,
    AGG_STOP_LOCAL_REQUEST,
    AGG_STOP_PEER_REQUEST,
    AGG_STOP_DESTROY_STA,
}

// Debugfs flags to enable/disable use of RX/TX airtime in scheduler

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airtime_info {
    pub rx_airtime: u64,
    pub tx_airtime: u64,
    pub last_active: c_ulong,
    pub deficit: i32,
    pub /: *mut *mut atomic_t aql_tx_pending; / Estimated airtime for frames pending,
    pub aql_limit_low: u32,
    pub aql_limit_high: u32,
}

//
// struct tid_ampdu_tx - TID aggregation information (Tx).
//
// @rcu_head: rcu head for freeing structure
// @session_timer: check if we keep Tx-ing on the TID (by timeout value)
// @addba_resp_timer: timer for peer's response to addba request
// @pending: pending frames queue -- use sta's spinlock to protect
// @sta: station we are attached to
// @dialog_token: dialog token for aggregation session
// @timeout: session timeout value to be filled in ADDBA requests
// @tid: TID number
// @state: session state (see above)
// @last_tx: jiffies of last tx activity
// @stop_initiator: initiator of a session stop
// @tx_stop: TX DelBA frame when stopping
// @buf_size: reorder buffer size at receiver
// @failed_bar_ssn: ssn of the last failed BAR tx attempt
// @bar_pending: BAR needs to be re-sent
// @amsdu: support A-MSDU within A-MDPU
// @ssn: starting sequence number of the session
// @ndp: this session is using NDP Block ACKs
//
// This structure's lifetime is managed by RCU, assignments to
// the array holding it must hold the aggregation mutex.
//
// The TX path can access it under RCU lock-free if, and
// only if, the state has the flag %HT_AGG_STATE_OPERATIONAL
// set. Otherwise, the TX path must also acquire the spinlock
// and re-check the state, see comments in the tx code
// touching it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_ampdu_tx {
    pub rcu_head: rcu_head,
    pub session_timer: timer_list,
    pub addba_resp_timer: timer_list,
    pub pending: sk_buff_head,
    pub sta: *mut sta_info,
    pub state: c_ulong,
    pub last_tx: c_ulong,
    pub timeout: u16,
    pub dialog_token: u8,
    pub stop_initiator: u8,
    pub tx_stop: bool,
    pub buf_size: u16,
    pub ssn: u16,
    pub failed_bar_ssn: u16,
    pub bar_pending: bool,
    pub amsdu: bool,
    pub ndp: bool,
    pub tid: u8,
}

//
// struct tid_ampdu_rx - TID aggregation information (Rx).
//
// @reorder_buf_filtered: bitmap indicating where there are filtered frames in
// the reorder buffer that should be ignored when releasing frames
// @session_timer: check if peer keeps Tx-ing on the TID (by timeout value)
// @reorder_timer: releases expired frames from the reorder buffer.
// @sta: station we are attached to
// @last_rx: jiffies of last rx activity
// @head_seq_num: head sequence number in reordering buffer.
// @stored_mpdu_num: number of MPDUs in reordering buffer
// @ssn: Starting Sequence Number expected to be aggregated.
// @buf_size: buffer size for incoming A-MPDUs
// @timeout: reset timer value (in TUs).
// @tid: TID number
// @rcu_head: RCU head used for freeing this struct
// @reorder_lock: serializes access to reorder buffer, see below.
// @auto_seq: used for offloaded BA sessions to automatically pick head_seq_and
// and ssn.
// @removed: this session is removed (but might have been found due to RCU)
// @started: this session has started (head ssn or higher was received)
// @reorder: reorder buffer entries
// @reorder.buf: &struct sk_buff_head for the frames, since there could be
// multiple at each entry from an A-MSDU reported as individual subframes
// @reorder.time: time when this entry was filled (jiffies)
//
// This structure's lifetime is managed by RCU, assignments to
// the array holding it must hold the aggregation mutex.
//
// The @reorder_lock is used to protect the members of this
// struct, except for @timeout, @buf_size and @dialog_token,
// which are constant across the lifetime of the struct (the
// dialog token being used only for debugging).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_ampdu_rx {
    pub rcu_head: rcu_head,
    pub reorder_lock: spinlock_t,
    pub reorder_buf_filtered: u64,
    pub sta: *mut sta_info,
    pub session_timer: timer_list,
    pub reorder_timer: timer_list,
    pub last_rx: c_ulong,
    pub head_seq_num: u16,
    pub stored_mpdu_num: u16,
    pub ssn: u16,
    pub buf_size: u16,
    pub timeout: u16,
    pub tid: u8,
    pub buf: sk_buff_head,
    pub time: c_ulong,
    pub reorder: [}; ],
}

//
// struct sta_ampdu_mlme - STA aggregation information.
//
// @tid_rx: aggregation info for Rx per TID -- RCU protected
// @tid_rx_token: dialog tokens for valid aggregation sessions
// @tid_rx_timer_expired: bitmap indicating on which TIDs the
// RX timer expired until the work for it runs
// @tid_rx_stop_requested:  bitmap indicating which BA sessions per TID the
// driver requested to close until the work for it runs
// @tid_rx_manage_offl: bitmap indicating which BA sessions were requested
// to be treated as started/stopped due to offloading
// @agg_session_valid: bitmap indicating which TID has a rx BA session open on
// @unexpected_agg: bitmap indicating which TID already sent a delBA due to
// unexpected aggregation related frames outside a session
// @work: work struct for starting/stopping aggregation
// @tid_tx: aggregation info for Tx per TID
// @tid_start_tx: sessions where start was requested, not just protected
// by wiphy mutex but also sta->lock
// @last_addba_req_time: timestamp of the last addBA request.
// @addba_req_num: number of times addBA request has been sent.
// @dialog_token_allocator: dialog token enumerator for each new session;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_ampdu_mlme {
// rx
    pub tid_rx: [*mut tid_ampdu_rx __rcu; IEEE80211_NUM_TIDS],
    pub tid_rx_token: [u8; IEEE80211_NUM_TIDS],
    pub tid_rx_timer_expired: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub tid_rx_stop_requested: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub IEEE80211_NUM_TIDS)]: *mut *mut unsigned long tid_rx_manage_offl[BITS_TO_LONGS(2,
    pub agg_session_valid: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
    pub unexpected_agg: [c_ulong; BITS_TO_LONGS(IEEE80211_NUM_TIDS)],
// tx
    pub work: wiphy_work,
    pub tid_tx: [*mut tid_ampdu_tx __rcu; IEEE80211_NUM_TIDS],
    pub tid_start_tx: [*mut tid_ampdu_tx; IEEE80211_NUM_TIDS],
    pub last_addba_req_time: [c_ulong; IEEE80211_NUM_TIDS],
    pub addba_req_num: [u8; IEEE80211_NUM_TIDS],
    pub dialog_token_allocator: u8,
}

// Value to indicate no TID reservation
pub const IEEE80211_TID_UNRESERVED: c_uint = 0xff;
pub const IEEE80211_FAST_XMIT_MAX_IV: c_int = 18;
//
// struct ieee80211_fast_tx - TX fastpath information
// @key: key to use for hw crypto
// @hdr: the 802.11 header to put with the frame
// @hdr_len: actual 802.11 header length
// @sa_offs: offset of the SA
// @da_offs: offset of the DA
// @pn_offs: offset where to put PN for crypto (or 0 if not needed)
// @band: band this will be transmitted on, for tx_info
// @rcu_head: RCU head to free this struct
//
// This struct is small enough so that the common case (maximum crypto
// header length of 8 like for CCMP/GCMP) fits into a single 64-byte
// cache line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_fast_tx {
    pub key: *mut ieee80211_key,
    pub hdr_len: u8,
    pub pn_offs: u8 sa_offs, da_offs,,
    pub band: u8,
    pub __aligned(2): sizeof(rfc1042_header)],
    pub rcu_head: rcu_head,
}

//
// struct ieee80211_fast_rx - RX fastpath information
// @dev: netdevice for reporting the SKB
// @vif_type: (P2P-less) interface type of the original sdata (sdata->vif.type)
// @vif_addr: interface address
// @rfc1042_hdr: copy of the RFC 1042 SNAP header (to have in cache)
// @control_port_protocol: control port protocol copied from sdata
// @expected_ds_bits: from/to DS bits expected
// @icv_len: length of the MIC if present
// @key: bool indicating encryption is expected (key is set)
// @internal_forward: forward froms internally on AP/VLAN type interfaces
// @uses_rss: copy of USES_RSS hw flag
// @da_offs: offset of the DA in the header (for header conversion)
// @sa_offs: offset of the SA in the header (for header conversion)
// @rcu_head: RCU head for freeing this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_fast_rx {
    pub dev: *mut net_device,
    pub vif_type: nl80211_iftype,
    pub __aligned(2): u8 vif_addr[ETH_ALEN],
    pub __aligned(2): u8 rfc1042_hdr[6],
    pub control_port_protocol: __be16,
    pub expected_ds_bits: __le16,
    pub icv_len: u8,
    pub sa_offs: u8 da_offs,,
    pub rcu_head: rcu_head,
}

// we use only values in the range 0-100, so pick a large precision
//
// struct mesh_sta - mesh STA information
// @plink_lock: serialize access to plink fields
// @llid: Local link ID
// @plid: Peer link ID
// @aid: local aid supplied by peer
// @reason: Cancel reason on PLINK_HOLDING state
// @plink_retries: Retries in establishment
// @plink_state: peer link state
// @plink_timeout: timeout of peer link
// @plink_timer: peer link watch timer
// @plink_sta: peer link watch timer's sta_info
// @t_offset: timing offset relative to this host
// @t_offset_setpoint: reference timing offset of this sta to be used when
// calculating clockdrift
// @local_pm: local link-specific power save mode
// @peer_pm: peer-specific power save mode towards local STA
// @nonpeer_pm: STA power save mode towards non-peer neighbors
// @processed_beacon: set to true after peer rates and capabilities are
// processed
// @connected_to_gate: true if mesh STA has a path to a mesh gate
// @connected_to_as: true if mesh STA has a path to a authentication server
// @fail_avg: moving percentage of failed MSDUs
// @tx_rate_avg: moving average of tx bitrate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_sta {
    pub plink_timer: timer_list,
    pub plink_sta: *mut sta_info,
    pub t_offset: i64,
    pub t_offset_setpoint: i64,
    pub plink_lock: spinlock_t,
    pub llid: u16,
    pub plid: u16,
    pub aid: u16,
    pub reason: u16,
    pub plink_retries: u8,
    pub processed_beacon: bool,
    pub connected_to_gate: bool,
    pub connected_to_as: bool,
    pub plink_state: nl80211_plink_state,
    pub plink_timeout: u32,
// mesh power save
    pub local_pm: nl80211_mesh_power_mode,
    pub peer_pm: nl80211_mesh_power_mode,
    pub nonpeer_pm: nl80211_mesh_power_mode,
// moving percentage of failed MSDUs
    pub fail_avg: ewma_mesh_fail_avg,
// moving average of tx bitrate
    pub tx_rate_avg: ewma_mesh_tx_rate_avg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_rx_stats {
    pub packets: c_ulong,
    pub last_rx: c_ulong,
    pub num_duplicates: c_ulong,
    pub fragments: c_ulong,
    pub dropped: c_ulong,
    pub last_signal: c_int,
    pub chains: u8,
    pub chain_signal_last: [i8; IEEE80211_MAX_CHAINS],
    pub last_rate: u32,
    pub syncp: u64_stats_sync,
    pub bytes: u64_stats_t,
    pub 1]: u64_stats_t msdu[IEEE80211_NUM_TIDS +,
}

//
// IEEE 802.11-2016 (10.6 "Defragmentation") recommends support for "concurrent
// reception of at least one MSDU per access category per associated STA"
// on APs, or "at least one MSDU per access category" on other interface types.
//
// This limit can be increased by changing this define, at the cost of slower
// frame reassembly and increased memory use while fragments are pending.
//
pub const IEEE80211_FRAGMENT_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_fragment_entry {
    pub skb_list: sk_buff_head,
    pub first_frag_time: c_ulong,
    pub seq: u16,
    pub extra_len: u16,
    pub last_frag: u16,
    pub rx_queue: u8,
    pub /: *mut *mut u8 last_pn[6]; / PN of the last fragment if CCMP was used,
    pub key_color: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_fragment_cache {
    pub entries: [ieee80211_fragment_entry; IEEE80211_FRAGMENT_MAX],
    pub next: c_uint,
}

//
// struct link_sta_info - Link STA information
// All link specific sta info are stored here for reference. This can be
// a single entry for non-MLD STA or multiple entries for MLD STA
// @addr: Link MAC address - Can be same as MLD STA mac address and is always
// same for non-MLD STA. This is used as key for searching link STA
// @link_id: Link ID uniquely identifying the link STA. This is 0 for non-MLD
// and set to the corresponding vif LinkId for MLD STA
// @op_mode_nss: NSS limit as set by operating mode notification, or 0
// @capa_nss: NSS limit as determined by local and peer capabilities
// @link_hash_node: hash node for rhashtable
// @sta: Points to the STA info
// @gtk: group keys negotiated with this station, if any
// @tx_stats: TX statistics
// @tx_stats.packets: # of packets transmitted
// @tx_stats.bytes: # of bytes in all packets transmitted
// @tx_stats.last_rate: last TX rate
// @tx_stats.msdu: # of transmitted MSDUs per TID
// @rx_stats: RX statistics
// @rx_stats_avg: averaged RX statistics
// @rx_stats_avg.signal: averaged signal
// @rx_stats_avg.chain_signal: averaged per-chain signal
// @pcpu_rx_stats: per-CPU RX statistics, assigned only if the driver needs
// this (by advertising the USES_RSS hw flag)
// @status_stats: TX status statistics
// @status_stats.filtered: # of filtered frames
// @status_stats.retry_failed: # of frames that failed after retry
// @status_stats.retry_count: # of retries attempted
// @status_stats.lost_packets: # of lost packets
// @status_stats.last_pkt_time: timestamp of last ACKed packet
// @status_stats.msdu_retries: # of MSDU retries
// @status_stats.msdu_failed: # of failed MSDUs
// @status_stats.last_ack: last ack timestamp (jiffies)
// @status_stats.last_ack_signal: last ACK signal
// @status_stats.ack_signal_filled: last ACK signal validity
// @status_stats.avg_ack_signal: average ACK signal
// @op_mode_bw: dynamic bandwidth limit to transmit to the STA,
// taken from HT/VHT capabilities or VHT operating mode notification.
// Invalid for NAN since that is operating on multiple bands.
// @rx_omi_bw_rx: RX OMI bandwidth restriction to apply for RX
// @rx_omi_bw_tx: RX OMI bandwidth restriction to apply for TX
// @rx_omi_bw_staging: RX OMI bandwidth restriction to apply later
// during finalize
// @uhr_usable_tx_width: bandwidth restriction for UHR for TX, only when
// the link_sta is an AP, to restrict TX to BSS width during DBE
// enablement
// @uhr_dbe_enabled: for STAs as clients to an AP interface indicates
// DBE is enabled by the STA
// @debugfs_dir: debug filesystem directory dentry
// @pub: public (driver visible) link STA data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_sta_info {
    pub addr: [u8; ETH_ALEN],
    pub link_id: u8,
    pub capa_nss: u8 op_mode_nss,,
    pub link_hash_node: rhlist_head,
    pub sta: *mut sta_info,
    pub pcpu_rx_stats: *mut ieee80211_sta_rx_stats __percpu,
// Updated from RX path only, no locking requirements
    pub rx_stats: ieee80211_sta_rx_stats,
    pub signal: ewma_signal,
    pub chain_signal: [ewma_signal; IEEE80211_MAX_CHAINS],
    pub rx_stats_avg: },
// Updated from TX status path only, no locking requirements
    pub filtered: c_ulong,
    pub retry_count: unsigned long retry_failed,,
    pub lost_packets: c_uint,
    pub last_pkt_time: c_ulong,
    pub 1]: u64 msdu_retries[IEEE80211_NUM_TIDS +,
    pub 1]: u64 msdu_failed[IEEE80211_NUM_TIDS +,
    pub last_ack: c_ulong,
    pub last_ack_signal: i8,
    pub ack_signal_filled: bool,
    pub avg_ack_signal: ewma_avg_signal,
    pub status_stats: },
// Updated from TX path only, no locking requirements
    pub packets: [u64; IEEE80211_NUM_ACS],
    pub bytes: [u64; IEEE80211_NUM_ACS],
    pub last_rate: ieee80211_tx_rate,
    pub last_rate_info: rate_info,
    pub 1]: u64 msdu[IEEE80211_NUM_TIDS +,
    pub tx_stats: },
    pub op_mode_bw: ieee80211_sta_rx_bandwidth,
    pub uhr_usable_tx_width: ieee80211_sta_rx_bandwidth,
    pub uhr_dbe_enabled: bool,

    pub debugfs_dir: *mut dentry,

    pub pub: *mut ieee80211_link_sta,
}

//
// struct ieee80211_sta_removed_link_stats - Removed link sta data
//
// keep required accumulated removed link data for stats
//
// @rx_packets: accumulated packets (MSDUs & MMPDUs) received from
// this station for removed links
// @tx_packets: accumulated packets (MSDUs & MMPDUs) transmitted to
// this station for removed links
// @rx_bytes: accumulated bytes (size of MPDUs) received from this
// station for removed links
// @tx_bytes: accumulated bytes (size of MPDUs) transmitted to this
// station for removed links
// @tx_retries: cumulative retry counts (MPDUs) for removed links
// @tx_failed: accumulated number of failed transmissions (MPDUs)
// (retries exceeded, no ACK) for removed links
// @rx_dropped_misc: accumulated dropped packets for un-specified reason
// from this station for removed links
// @beacon_loss_count: Number of times beacon loss event has triggered
// from this station for removed links.
// @expected_throughput: expected throughput in kbps (including 802.11
// headers) towards this station for removed links
// @pertid_stats: accumulated per-TID statistics for removed link of
// station
// @pertid_stats.rx_msdu : accumulated number of received MSDUs towards
// this station for removed links.
// @pertid_stats.tx_msdu: accumulated number of (attempted) transmitted
// MSDUs towards this station for removed links
// @pertid_stats.tx_msdu_retries: accumulated number of retries (not
// counting the first) for transmitted MSDUs towards this station
// for removed links
// @pertid_stats.tx_msdu_failed: accumulated number of failed transmitted
// MSDUs towards this station for removed links
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_removed_link_stats {
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub tx_retries: u32,
    pub tx_failed: u32,
    pub rx_dropped_misc: u32,
    pub beacon_loss_count: u32,
    pub expected_throughput: u32,
    pub rx_msdu: u64,
    pub tx_msdu: u64,
    pub tx_msdu_retries: u64,
    pub tx_msdu_failed: u64,
    pub pertid_stats: },
}

//
// struct sta_info - STA information
//
// This structure collects information about a station that
// mac80211 is communicating with.
//
// @list: global linked list entry
// @free_list: list entry for keeping track of stations to free
// @hash_node: hash node for rhashtable
// @addr: station's MAC address - duplicated from public part to
// let the hash table work with just a single cacheline
// @local: pointer to the global information
// @sdata: virtual interface this station belongs to
// @ptk: peer keys negotiated with this station, if any
// @ptk_idx: last installed peer key index
// @rate_ctrl: rate control algorithm reference
// @rate_ctrl_lock: spinlock used to protect rate control data
// (data inside the algorithm, so serializes calls there)
// @rate_ctrl_priv: rate control private per-STA pointer
// @lock: used for locking all fields that require locking, see comments
// in the header file.
// @drv_deliver_wk: used for delivering frames after driver PS unblocking
// @listen_interval: listen interval of this station, when we're acting as AP
// @_flags: STA flags, see &enum ieee80211_sta_info_flags, do not use directly
// @ps_lock: used for powersave (when mac80211 is the AP) related locking
// @ps_tx_buf: buffers (per AC) of frames to transmit to this station
// when it leaves power saving state or polls
// @tx_filtered: buffers (per AC) of frames we already tried to
// transmit but were filtered by hardware due to STA having
// entered power saving state, these are also delivered to
// the station when it leaves powersave or polls for frames
// @driver_buffered_tids: bitmap of TIDs the driver has data buffered on
// @txq_buffered_tids: bitmap of TIDs that mac80211 has txq data buffered on
// @assoc_at: clock boottime (in ns) of last association
// @last_connected: time (in seconds) when a station got connected
// @last_seq_ctrl: last received seq/frag number from this STA (per TID
// plus one for non-QoS frames)
// @tid_seq: per-TID sequence numbers for sending to this STA
// @airtime: per-AC struct airtime_info describing airtime statistics for this
// station
// @airtime_weight: station weight for airtime fairness calculation purposes
// @ampdu_mlme: A-MPDU state machine state
// @mesh: mesh STA information
// @debugfs_dir: debug filesystem directory dentry
// @dead: set to true when sta is unlinked
// @removed: set to true when sta is being removed from sta_list
// @uploaded: set to true when sta is uploaded to the driver
// @sta: station information we share with the driver
// @sta_state: duplicates information about station state (for debug)
// @rcu_head: RCU head used for freeing this station struct
// @reserved_tid: reserved TID (if any, otherwise IEEE80211_TID_UNRESERVED)
// @amsdu_mesh_control: track the mesh A-MSDU format used by the peer:
//
// * -1: not yet known
// * 0: non-mesh A-MSDU length field
// * 1: big-endian mesh A-MSDU length field
// * 2: little-endian mesh A-MSDU length field
//
// @fast_tx: TX fastpath information
// @fast_rx: RX fastpath information
// @tdls_chandef: a TDLS peer can have a wider chandef that is compatible to
// the BSS one.
// @frags: fragment cache
// @cur: storage for aggregation data
// &struct ieee80211_sta points either here or to deflink.agg.
// @deflink: This is the default link STA information, for non MLO STA all link
// specific STA information is accessed through @deflink or through
// link[0] which points to address of @deflink. For MLO Link STA
// the first added link STA will point to deflink.
// @link: reference to Link Sta entries. For Non MLO STA, except 1st link,
// i.e link[0] all links would be assigned to NULL by default and
// would access link information via @deflink or link[0]. For MLO
// STA, first link STA being added will point its link pointer to
// @deflink address and remaining would be allocated and the address
// would be assigned to link[link_id] where link_id is the id assigned
// by the AP.
// @rem_link_stats: accumulated removed link stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_info {
// General information, mostly static
    pub free_list: list_head list,,
    pub rcu_head: rcu_head,
    pub hash_node: rhlist_head,
    pub addr: [u8; ETH_ALEN],
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub ptk: [*mut ieee80211_key __rcu; NUM_DEFAULT_KEYS],
    pub ptk_idx: u8,
    pub rate_ctrl: *mut rate_control_ref,
    pub rate_ctrl_priv: *mut c_void,
    pub rate_ctrl_lock: spinlock_t,
    pub lock: spinlock_t,
    pub fast_tx: *mut ieee80211_fast_tx __rcu,
    pub fast_rx: *mut ieee80211_fast_rx __rcu,

    pub mesh: *mut mesh_sta,

    pub drv_deliver_wk: work_struct,
    pub listen_interval: u16,
    pub dead: bool,
    pub removed: bool,
    pub uploaded: bool,
    pub sta_state: ieee80211_sta_state,
// use the accessors defined below
    pub _flags: c_ulong,
// STA powersave lock and frame queues
    pub ps_lock: spinlock_t,
    pub ps_tx_buf: [sk_buff_head; IEEE80211_NUM_ACS],
    pub tx_filtered: [sk_buff_head; IEEE80211_NUM_ACS],
    pub driver_buffered_tids: c_ulong,
    pub txq_buffered_tids: c_ulong,
    pub assoc_at: u64,
    pub last_connected: c_long,
// Plus 1 for non-QoS frames
    pub 1]: __le16 last_seq_ctrl[IEEE80211_NUM_TIDS +,
    pub 1]: u16 tid_seq[IEEE80211_QOS_CTL_TID_MASK +,
    pub airtime: [airtime_info; IEEE80211_NUM_ACS],
    pub airtime_weight: u16,
//
// Aggregation information, locked with lock.
//
    pub ampdu_mlme: sta_ampdu_mlme,

    pub debugfs_dir: *mut dentry,

    pub reserved_tid: u8,
    pub amsdu_mesh_control: i8,
    pub tdls_chandef: cfg80211_chan_def,
    pub frags: ieee80211_fragment_cache,
    pub cur: ieee80211_sta_aggregates,
    pub deflink: link_sta_info,
    pub link: [*mut link_sta_info __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
    pub rem_link_stats: ieee80211_sta_removed_link_stats,
// keep last!
    pub sta: ieee80211_sta,
}

// TDLS STA can only have a single link

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &sta->_flags) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: flag, _arg: &sta->_flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: flag, _arg: &sta->_flags) -> return;
}

// Maximum number of frames to buffer per power saving station per AC
pub const STA_MAX_TX_BUFFER: c_int = 64;
// Minimum buffered frame expiry time. If STA uses listen interval that is
// smaller than this value, the minimum value here is used instead.

// How often station data is cleaned up (e.g., expiration of buffered frames)
//

//
// Get a STA info, must be under RCU read lock.
//
// user must hold wiphy mutex or be in RCU critical section

//
// Get STA info by index, BROKEN!
//
// Create a new STA info, caller owns returned structure
// until sta_info_insert().
//
extern "C" {
    pub fn sta_info_free(local: *mut ieee80211_local, sta: *mut sta_info);
}
//
// Insert STA info into hash table/list, returns zero or a
// -EEXIST if (if the same MAC address is already present).
//
// Calling the non-rcu version makes the caller relinquish,
// the _rcu version calls read_lock_rcu() and must be called
// without it held.
//
extern "C" {
    pub fn sta_info_insert(sta: *mut sta_info) -> c_int;
}
extern "C" {
    pub fn sta_info_insert_rcu(__acquires(RCU: *mut *mut sta_info sta)) -> c_int;
}
extern "C" {
    pub fn __sta_info_destroy(sta: *mut sta_info) -> int __must_check;
}
extern "C" {
    pub fn sta_info_recalc_tim(sta: *mut sta_info);
}
extern "C" {
    pub fn sta_info_init(local: *mut ieee80211_local) -> c_int;
}
extern "C" {
    pub fn sta_info_stop(local: *mut ieee80211_local);
}
//
// __sta_info_flush - flush matching STA entries from the STA table
//
// Return: the number of removed STA entries.
//
// @sdata: sdata to remove all stations from
// @vlans: if the given interface is an AP interface, also flush VLANs
// @link_id: if given (>=0), all those STA entries using @link_id only
// will be removed. If -1 is passed, all STA entries will be
// removed.
// @do_not_flush_sta: a station that shouldn't be flushed.
//
// sta_info_flush - flush matching STA entries from the STA table
//
// Return: the number of removed STA entries.
//
// @sdata: sdata to remove all stations from
// @link_id: if given (>=0), all those STA entries using @link_id only
// will be removed. If -1 is passed, all STA entries will be
// removed.
//
extern "C" {
    pub fn __sta_info_flush(_arg: sdata, _arg: false, _arg: link_id, _arg: NULL) -> return;
}
extern "C" {
    pub fn sta_get_expected_throughput(sta: *mut sta_info) -> u32;
}
extern "C" {
    pub fn ieee80211_sta_allocate_link(sta: *mut sta_info, link_id: c_uint) -> c_int;
}
extern "C" {
    pub fn ieee80211_sta_free_link(sta: *mut sta_info, link_id: c_uint);
}
extern "C" {
    pub fn ieee80211_sta_activate_link(sta: *mut sta_info, link_id: c_uint) -> c_int;
}
extern "C" {
    pub fn ieee80211_sta_remove_link(sta: *mut sta_info, link_id: c_uint);
}
extern "C" {
    pub fn ieee80211_sta_ps_deliver_wakeup(sta: *mut sta_info);
}
extern "C" {
    pub fn ieee80211_sta_ps_deliver_poll_response(sta: *mut sta_info);
}
extern "C" {
    pub fn ieee80211_sta_ps_deliver_uapsd(sta: *mut sta_info);
}
extern "C" {
    pub fn ieee80211_sta_last_active(sta: *mut sta_info, link_id: c_int) -> c_ulong;
}
extern "C" {
    pub fn __ieee80211_sta_recalc_aggregates(sta: *mut sta_info, active_links: u16);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_sta_bw_direction {
    IEEE80211_STA_BW_RX_FROM_STA,
    IEEE80211_STA_BW_TX_TO_STA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sta_stats_type {
    STA_STATS_RATE_TYPE_INVALID = 0,
    STA_STATS_RATE_TYPE_LEGACY,
    STA_STATS_RATE_TYPE_HT,
    STA_STATS_RATE_TYPE_VHT,
    STA_STATS_RATE_TYPE_HE,
    STA_STATS_RATE_TYPE_S1G,
    STA_STATS_RATE_TYPE_EHT,
    STA_STATS_RATE_TYPE_UHR,
}

// common
pub const STA_STATS_FIELD_TYPE: c_uint = 0x0000000F;
pub const STA_STATS_FIELD_BW: c_uint = 0x000001F0;
pub const STA_STATS_FIELD_RESERVED: c_uint = 0x00000E00;
// STA_STATS_RATE_TYPE_LEGACY
pub const STA_STATS_FIELD_LEGACY_IDX: c_uint = 0x0000F000;
pub const STA_STATS_FIELD_LEGACY_BAND: c_uint = 0x000F0000;
// STA_STATS_RATE_TYPE_HT
pub const STA_STATS_FIELD_HT_MCS: c_uint = 0x000FF000;
// STA_STATS_RATE_TYPE_VHT
pub const STA_STATS_FIELD_VHT_MCS: c_uint = 0x0000F000;
pub const STA_STATS_FIELD_VHT_NSS: c_uint = 0x000F0000;
// HT, VHT & S1G
pub const STA_STATS_FIELD_SGI: c_uint = 0x00100000;
// STA_STATS_RATE_TYPE_HE
pub const STA_STATS_FIELD_HE_MCS: c_uint = 0x0000F000;
pub const STA_STATS_FIELD_HE_NSS: c_uint = 0x000F0000;
pub const STA_STATS_FIELD_HE_RU: c_uint = 0x00700000;
pub const STA_STATS_FIELD_HE_GI: c_uint = 0x01800000;
pub const STA_STATS_FIELD_HE_DCM: c_uint = 0x02000000;
// STA_STATS_RATE_TYPE_EHT
pub const STA_STATS_FIELD_EHT_MCS: c_uint = 0x0000F000;
pub const STA_STATS_FIELD_EHT_NSS: c_uint = 0x000F0000;
pub const STA_STATS_FIELD_EHT_RU: c_uint = 0x00F00000;
pub const STA_STATS_FIELD_EHT_GI: c_uint = 0x03000000;
// STA_STATS_RATE_TYPE_UHR
pub const STA_STATS_FIELD_UHR_MCS: c_uint = 0x0001F000;
pub const STA_STATS_FIELD_UHR_NSS: c_uint = 0x001E0000;
pub const STA_STATS_FIELD_UHR_RU: c_uint = 0x01E00000;
pub const STA_STATS_FIELD_UHR_GI: c_uint = 0x06000000;
pub const STA_STATS_FIELD_UHR_ELR: c_uint = 0x08000000;
pub const STA_STATS_FIELD_UHR_IM: c_uint = 0x10000000;
// STA_STATS_RATE_TYPE_S1G
pub const STA_STATS_FIELD_S1G_MCS: c_uint = 0x0000F000;
pub const STA_STATS_FIELD_S1G_NSS: c_uint = 0x000F0000;

pub const STA_STATS_RATE_INVALID: c_int = 0;
