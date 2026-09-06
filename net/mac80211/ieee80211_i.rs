//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/ieee80211_i.h
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
// Copyright 2002-2005, Instant802 Networks, Inc.
// Copyright 2005, Devicescape Software, Inc.
// Copyright 2006-2007	Jiri Benc <jbenc@suse.cz>
// Copyright 2007-2010	Johannes Berg <johannes@sipsolutions.net>
// Copyright 2013-2015  Intel Mobile Communications GmbH
// Copyright (C) 2018-2026 Intel Corporation
//

// Maximum number of broadcast/multicast frames to buffer when some of the
// associated stations are using power saving.
pub const AP_MAX_BC_BUFFER: c_int = 128;
// Maximum number of frames buffered to all STAs, including multicast frames.
// Note: increasing this limit increases the potential memory requirement. Each
// frame can be up to about 2 kB long.
pub const TOTAL_MAX_TX_BUFFER: c_int = 512;
// Required encryption head and tailroom
pub const IEEE80211_ENCRYPT_HEADROOM: c_int = 8;
pub const IEEE80211_ENCRYPT_TAILROOM: c_int = 18;
// power level hasn't been configured (or set to automatic)

//
// Some APs experience problems when working with U-APSD. Decreasing the
// probability of that happening by using legacy mode for all ACs but VO isn't
// enough.
//
// Cisco 4410N originally forced us to enable VO by default only because it
// treated non-VO ACs as legacy.
//
// However some APs (notably Netgear R7000) silently reclassify packets to
// different ACs. Since u-APSD ACs require trigger frames for frame retrieval
// clients would never see some frames (e.g. ARP responses) or would fetch them
// accidentally after a long time.
//
// It makes little sense to enable u-APSD queues by default because it needs
// userspace applications to be aware of it to actually take advantage of the
// possible additional powersavings. Implicitly depending on driver autotrigger
// frame support doesn't make much sense.
//
pub const IEEE80211_DEFAULT_UAPSD_QUEUES: c_int = 0;

pub const IEEE80211_MAX_NAN_INSTANCE_ID: c_int = 255;
//
// Current mac80211 implementation supports a maximum of 1600 AIDS
// for S1G interfaces. With regards to an S1G TIM, this covers 25 blocks
// as each block is 64 AIDs.
//
pub const IEEE80211_MAX_SUPPORTED_S1G_AID: c_int = 1600;
pub const IEEE80211_MAX_SUPPORTED_S1G_TIM_BLOCKS: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_status_data {
    IEEE80211_STATUS_TYPE_MASK	= 0x00f,
    IEEE80211_STATUS_TYPE_INVALID	= 0,
    IEEE80211_STATUS_TYPE_SMPS	= 1,
    IEEE80211_STATUS_TYPE_NEG_TTLM	= 2,
    IEEE80211_STATUS_TYPE_UHR_OMP	= 3,
    IEEE80211_STATUS_SUBDATA_MASK	= 0x1ff0,
}

// Keep a station's queues on the active list for deficit accounting
// purposes if it was active or queued during the last 100ms.
//
extern "C" {
    pub fn time_before_eq(_arg: jiffies, 10: sta->airtime[ac].last_active + HZ /) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bss {
    pub device_ts_presp: u32 device_ts_beacon,,
    pub wmm_used: bool,
    pub uapsd_supported: bool,
pub const IEEE80211_MAX_SUPP_RATES: c_int = 32;
    pub supp_rates: [u8; IEEE80211_MAX_SUPP_RATES],
    pub supp_rates_len: usize,
    pub beacon_rate: *mut ieee80211_rate,
    pub vht_cap_info: u32,
//
// During association, we save an ERP value from a probe response so
// that we can feed ERP info to the driver when handling the
// association completes. these fields probably won't be up-to-date
// otherwise, you probably don't want to use them.
//
    pub has_erp_value: bool,
    pub erp_value: u8,
// Keep track of the corruption of the last beacon/probe response.
    pub corrupt_data: u8,
// Keep track of what bits of information we have valid info for.
    pub valid_data: u8,
}

//
// enum ieee80211_bss_corrupt_data_flags - BSS data corruption flags
// @IEEE80211_BSS_CORRUPT_BEACON: last beacon frame received was corrupted
// @IEEE80211_BSS_CORRUPT_PROBE_RESP: last probe response received was corrupted
//
// These are bss flags that are attached to a bss in the
// @corrupt_data field of &struct ieee80211_bss.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_bss_corrupt_data_flags {
    IEEE80211_BSS_CORRUPT_BEACON		= BIT(0),
    IEEE80211_BSS_CORRUPT_PROBE_RESP	= BIT(1)
}

//
// enum ieee80211_bss_valid_data_flags - BSS valid data flags
// @IEEE80211_BSS_VALID_WMM: WMM/UAPSD data was gathered from non-corrupt IE
// @IEEE80211_BSS_VALID_RATES: Supported rates were gathered from non-corrupt IE
// @IEEE80211_BSS_VALID_ERP: ERP flag was gathered from non-corrupt IE
//
// These are bss flags that are attached to a bss in the
// @valid_data field of &struct ieee80211_bss.  They show which parts
// of the data structure were received as a result of an un-corrupted
// beacon/probe response.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_bss_valid_data_flags {
    IEEE80211_BSS_VALID_WMM			= BIT(1),
    IEEE80211_BSS_VALID_RATES		= BIT(2),
    IEEE80211_BSS_VALID_ERP			= BIT(3)
}

pub type ieee80211_tx_result = unsigned ;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tx_data {
    pub skb: *mut sk_buff,
    pub skbs: sk_buff_head,
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub sta: *mut sta_info,
    pub key: *mut ieee80211_key,
    pub rate: ieee80211_tx_rate,
    pub flags: c_uint,
}

//
// enum ieee80211_packet_rx_flags - packet RX flags
// @IEEE80211_RX_AMSDU: a-MSDU packet
// @IEEE80211_RX_MALFORMED_ACTION_FRM: action frame is malformed
// @IEEE80211_RX_DEFERRED_RELEASE: frame was subjected to receive reordering
//
// These are per-frame flags that are attached to a frame in the
// @rx_flags field of &struct ieee80211_rx_status.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_packet_rx_flags {
    IEEE80211_RX_AMSDU			= BIT(3),
    IEEE80211_RX_MALFORMED_ACTION_FRM	= BIT(4),
    IEEE80211_RX_DEFERRED_RELEASE		= BIT(5),
}

//
// enum ieee80211_rx_flags - RX data flags
//
// @IEEE80211_RX_BEACON_REPORTED: This frame was already reported
// to cfg80211_report_obss_beacon().
//
// These flags are used across handling multiple interfaces
// for a single frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_rx_flags {
    IEEE80211_RX_BEACON_REPORTED	= BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_rx_data {
    pub list: *mut list_head,
    pub skb: *mut sk_buff,
    pub local: *mut ieee80211_local,
    pub sdata: *mut ieee80211_sub_if_data,
    pub link: *mut ieee80211_link_data,
    pub sta: *mut sta_info,
    pub link_sta: *mut link_sta_info,
    pub key: *mut ieee80211_key,
    pub flags: c_uint,
//
// Index into sequence numbers array, 0..16
// since the last (16) is used for non-QoS,
// will be 16 on non-QoS frames.
//
    pub seqno_idx: c_int,
//
// Index into the security IV/PN arrays, 0..16
// since the last (16) is used for CCMP-encrypted
// management frames, will be set to 16 on mgmt
// frames and 0 on non-QoS frames.
//
    pub security_idx: c_int,
    pub link_id: c_int,
    pub iv32: u32,
    pub iv16: u16,
    pub tkip: },
    pub pn: [u8; IEEE80211_CCMP_PN_LEN],
    pub ccm_gcm: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_csa_settings {
    pub counter_offsets_beacon: *const u16,
    pub counter_offsets_presp: *const u16,
    pub n_counter_offsets_beacon: c_int,
    pub n_counter_offsets_presp: c_int,
    pub count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_color_change_settings {
    pub counter_offset_beacon: u16,
    pub counter_offset_presp: u16,
    pub count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beacon_data {
    pub tail: *mut *mut u8 head,,
    pub tail_len: int head_len,,
    pub meshconf: *mut ieee80211_meshconf_ie,
    pub cntdwn_counter_offsets: [u16; IEEE80211_MAX_CNTDWN_COUNTERS_NUM],
    pub cntdwn_current_counter: u8,
    pub mbssid_ies: *mut cfg80211_mbssid_elems,
    pub rnr_ies: *mut cfg80211_rnr_elems,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe_resp {
    pub rcu_head: rcu_head,
    pub len: c_int,
    pub cntdwn_counter_offsets: [u16; IEEE80211_MAX_CNTDWN_COUNTERS_NUM],
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fils_discovery_data {
    pub rcu_head: rcu_head,
    pub len: c_int,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unsol_bcast_probe_resp_data {
    pub rcu_head: rcu_head,
    pub len: c_int,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s1g_short_beacon_data {
    pub rcu_head: rcu_head,
    pub short_head: *mut u8,
    pub short_tail: *mut u8,
    pub short_head_len: c_int,
    pub short_tail_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps_data {
// yes, this looks ugly, but guarantees that we can later use
// bitmap_empty :)
// NB: don't touch this bitmap, use sta_info_{set,clear}_tim_bit
    pub long)): __aligned(__alignof__(unsigned,
    pub bc_buf: sk_buff_head,
    pub /: *mut *mut atomic_t num_sta_ps; / number of stations in PS mode,
    pub dtim_count: c_int,
    pub dtim_bc_mc: bool,
    pub /: *mut *mut int sb_count; / num short beacons til next long beacon,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_ap {
    pub /: *mut *mut list_head vlans; / write-protected with RTNL and local->mtx,
    pub ps: ps_data,
    pub /: *mut *mut atomic_t num_mcast_sta; / number of stations receiving multicast,
    pub multicast_to_unicast: bool,
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_vlan {
    pub /: *mut *mut list_head list; / write-protected with RTNL and local->mtx,
// used for all tx if the VLAN is configured to 4-addr mode
    pub sta: *mut sta_info __rcu,
    pub /: *mut *mut atomic_t num_mcast_sta; / number of stations receiving multicast,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_stats {
    pub /: *mut *mut __u32 fwded_mcast; / Mesh forwarded multicast frames,
    pub /: *mut *mut __u32 fwded_unicast; / Mesh forwarded unicast frames,
    pub /: *mut *mut __u32 fwded_frames; / Mesh total forwarded frames,
    pub 0*/: *mut *mut __u32 dropped_frames_ttl; / Not transmitted since mesh_ttl ==,
    pub /: *mut *mut __u32 dropped_frames_no_route; / Not transmitted, no route found,
}

pub const PREQ_Q_F_START: c_uint = 0x1;
pub const PREQ_Q_F_REFRESH: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_preq_queue {
    pub list: list_head,
    pub dst: [u8; ETH_ALEN],
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_roc_work {
    pub list: list_head,
    pub sdata: *mut ieee80211_sub_if_data,
    pub chan: *mut ieee80211_channel,
    pub notified: bool started, abort, hw_begun,,
    pub on_channel: bool,
    pub start_time: c_ulong,
    pub req_duration: u32 duration,,
    pub frame: *mut sk_buff,
    pub mgmt_tx_cookie: u64 cookie,,
    pub type: ieee80211_roc_type,
}

// flags used in struct ieee80211_if_managed.flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_sta_flags {
    IEEE80211_STA_CONNECTION_POLL	= BIT(1),
    IEEE80211_STA_CONTROL_PORT	= BIT(2),
    IEEE80211_STA_MFP_ENABLED	= BIT(6),
    IEEE80211_STA_UAPSD_ENABLED	= BIT(7),
    IEEE80211_STA_NULLFUNC_ACKED	= BIT(8),
    IEEE80211_STA_ENABLE_RRM	= BIT(15),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_conn_mode {
    IEEE80211_CONN_MODE_S1G,
    IEEE80211_CONN_MODE_LEGACY,
    IEEE80211_CONN_MODE_HT,
    IEEE80211_CONN_MODE_VHT,
    IEEE80211_CONN_MODE_HE,
    IEEE80211_CONN_MODE_EHT,
    IEEE80211_CONN_MODE_UHR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_conn_bw_limit {
    IEEE80211_CONN_BW_LIMIT_20,
    IEEE80211_CONN_BW_LIMIT_40,
    IEEE80211_CONN_BW_LIMIT_80,
    IEEE80211_CONN_BW_LIMIT_160, /* also 80+80 */
    IEEE80211_CONN_BW_LIMIT_320,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_conn_settings {
    pub mode: ieee80211_conn_mode,
    pub bw_limit: ieee80211_conn_bw_limit,
    pub dbe_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mgd_auth_data {
    pub bss: *mut cfg80211_bss,
    pub timeout: c_ulong,
    pub tries: c_int,
    pub expected_transaction: u16 algorithm,,
    pub userspace_selectors: [c_ulong; BITS_TO_LONGS(128)],
    pub key: [u8; WLAN_KEY_LEN_WEP104],
    pub key_idx: u8 key_len,,
    pub waiting: bool done,,
    pub peer_confirmed: bool,
    pub timeout_started: bool,
    pub link_id: c_int,
    pub __aligned(2): u8 ap_addr[ETH_ALEN],
    pub status: u16 trans,,
    pub data_len: usize,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mgd_assoc_data {
    pub bss: *mut cfg80211_bss,
    pub __aligned(2): u8 addr[ETH_ALEN],
    pub ap_ht_param: u8,
    pub ap_vht_cap: ieee80211_vht_cap,
    pub elems_len: usize,
    pub /: *mut *mut *mut u8 elems; / pointing to inside ie[] below,
    pub conn: ieee80211_conn_settings,
    pub status: u16,
    pub link: [}; IEEE80211_MLD_MAX_NUM_LINKS],
    pub __aligned(2): u8 ap_addr[ETH_ALEN],
// this is for a workaround, so we use it only for non-MLO
    pub supp_rates: *const u8,
    pub supp_rates_len: u8,
    pub timeout: c_ulong,
    pub tries: c_int,
    pub prev_ap_addr: [u8; ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub uapsd: bool wmm,,
    pub need_beacon: bool,
    pub synced: bool,
    pub timeout_started: bool,
    pub /: *mut *mut bool comeback; / whether the AP has requested association comeback,
    pub s1g: bool,
    pub spp_amsdu: bool,
    pub assoc_link_id: i8,
    pub ext_mld_capa_ops: __le16,
    pub FILS_NONCE_LEN]: *mut *mut u8 fils_nonces[2,
    pub fils_kek: [u8; FILS_MAX_KEK_LEN],
    pub fils_kek_len: usize,
    pub ie_len: usize,
    pub /: *mut *mut *mut u8 ie_pos; / used to fill ie[] with link[].elems,
    pub ie: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sta_tx_tspec {
// timestamp of the first packet in the time slice
    pub time_slice_start: c_ulong,
    pub /: *mut *mut u32 admitted_time; / in usecs, unlike over the air,
    pub tsid: u8,
    pub /: *mut *mut s8 up; / signed to be able to invalidate with -1 during teardown,
// consumed TX time in microseconds in the time slice
    pub consumed_tx_time: u32,
    pub action: },
    pub downgraded: bool,
}

// Advertised TID-to-link mapping info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_adv_ttlm_info {
// time in TUs at which the new mapping is established, or 0 if there is
// no planned advertised TID-to-link mapping
//
    pub switch_time: u16,
    pub /: *mut *mut u32 duration; / duration of the planned T2L map in TUs,
    pub /: *mut *mut u16 map; / map of usable links for all TIDs,
    pub /: *mut *mut bool active; / whether the advertised mapping is active or not,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_managed {
    pub timer: timer_list,
    pub conn_mon_timer: timer_list,
    pub bcn_mon_timer: timer_list,
    pub monitor_work: wiphy_work,
    pub beacon_connection_loss_work: wiphy_work,
    pub csa_connection_drop_work: wiphy_work,
    pub beacon_timeout: c_ulong,
    pub probe_timeout: c_ulong,
    pub probe_send_count: c_int,
    pub nullfunc_failed: bool,
    pub auth_data: *mut ieee80211_mgd_auth_data,
    pub assoc_data: *mut ieee80211_mgd_assoc_data,
    pub userspace_selectors: [c_ulong; BITS_TO_LONGS(128)],
    pub /: *mut *mut bool powersave; / powersave requested for this iface,
    pub /: *mut *mut bool broken_ap; / AP is broken -- turn off powersave,
    pub flags: c_uint,
    pub mcast_seq_last: u16,
    pub status_acked: bool,
    pub status_received: bool,
    pub status_fc: __le16,
    pub /: *mut *mut } mfp; / management frame protection,
//
// Bitmask of enabled u-apsd queues,
// IEEE80211_WMM_IE_STA_QOSINFO_AC_BE & co. Needs a new association
// to take effect.
//
    pub uapsd_queues: c_uint,
//
// Maximum number of buffered frames AP can deliver during a
// service period, IEEE80211_WMM_IE_STA_QOSINFO_SP_ALL or similar.
// Needs a new association to take effect.
//
    pub uapsd_max_sp_len: c_uint,
    pub use_4addr: u8,
//
// State variables for keeping track of RSSI of the AP currently
// connected to and informing driver when RSSI has gone
// below/above a certain threshold.
//
    pub rssi_max_thold: int rssi_min_thold,,
    pub /: *mut *mut ieee80211_ht_cap ht_capa; / configured ht-cap over-rides,
    pub /: *mut *mut ieee80211_ht_cap ht_capa_mask; / Valid parts of ht_capa,
    pub /: *mut *mut ieee80211_vht_cap vht_capa; / configured VHT overrides,
    pub /: *mut *mut ieee80211_vht_cap vht_capa_mask; / Valid parts of vht_capa,
    pub /: *mut *mut ieee80211_s1g_cap s1g_capa; / configured S1G overrides,
    pub /: *mut *mut ieee80211_s1g_cap s1g_capa_mask; / valid s1g_capa bits,
// TDLS support
    pub __aligned(2): u8 tdls_peer[ETH_ALEN],
    pub tdls_peer_del_work: wiphy_delayed_work,
    pub /: *mut *mut *mut sk_buff orig_teardown_skb; / The original teardown skb,
    pub /: *mut *mut *mut sk_buff teardown_skb; / A copy to send through the AP,
    pub /: *mut *mut spinlock_t teardown_lock; / To lock changing teardown_skb,
    pub tdls_wider_bw_prohibited: bool,
// WMM-AC TSPEC support
    pub tx_tspec: [ieee80211_sta_tx_tspec; IEEE80211_NUM_ACS],
// Use a separate work struct so that we can do something here
// while the sdata->work is flushing the queues, for example.
// otherwise, in scenarios where we hardly get any traffic out
// on the BE queue, but there's a lot of VO traffic, we might
// get stuck in a downgraded situation and flush takes forever.
//
    pub tx_tspec_wk: wiphy_delayed_work,
// Information elements from the last transmitted (Re)Association
// Request frame.
//
    pub assoc_req_ies: *mut u8,
    pub assoc_req_ies_len: usize,
    pub ml_reconf_work: wiphy_hrtimer_work,
    pub removed_links: u16,
// TID-to-link mapping support
    pub ttlm_work: wiphy_hrtimer_work,
    pub ttlm_info: ieee80211_adv_ttlm_info,
    pub teardown_ttlm_work: wiphy_work,
// dialog token enumerator for neg TTLM request
    pub dialog_token_alloc: u8,
    pub neg_ttlm_timeout_work: wiphy_delayed_work,
// Locally initiated multi-link reconfiguration
    pub add_links_data: *mut ieee80211_mgd_assoc_data,
    pub wk: wiphy_delayed_work,
    pub removed_links: u16,
    pub added_links: u16,
    pub dialog_token: u8,
    pub reconf: },
// Support for epcs
    pub enabled: bool,
    pub dialog_token: u8,
    pub epcs: },
    pub status_work: wiphy_hrtimer_work,
    pub timeout_us: u32,
    pub links: u16,
    pub pending_init: u16 pending,,
    pub dialog_token: u8,
    pub acked: bool,
    pub uhr_omp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_ibss {
    pub timer: timer_list,
    pub csa_connection_drop_work: wiphy_work,
    pub last_scan_completed: c_ulong,
    pub basic_rates: u32,
    pub fixed_bssid: bool,
    pub fixed_channel: bool,
    pub privacy: bool,
    pub control_port: bool,
    pub userspace_handles_dfs: bool,
    pub __aligned(2): u8 bssid[ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ie_len: u8 ssid_len,,
    pub ie: *mut u8,
    pub chandef: cfg80211_chan_def,
    pub ibss_join_req: c_ulong,
// probe response/beacon for IBSS
    pub presp: *mut beacon_data __rcu,
    pub /: *mut *mut ieee80211_ht_cap ht_capa; / configured ht-cap over-rides,
    pub /: *mut *mut ieee80211_ht_cap ht_capa_mask; / Valid parts of ht_capa,
    pub incomplete_lock: spinlock_t,
    pub incomplete_stations: list_head,
    pub state: },
}

//
// struct ieee80211_if_ocb - OCB mode state
//
// @housekeeping_timer: timer for periodic invocation of a housekeeping task
// @wrkq_flags: OCB deferred task action
// @incomplete_lock: delayed STA insertion lock
// @incomplete_stations: list of STAs waiting for delayed insertion
// @joined: indication if the interface is connected to an OCB network
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_ocb {
    pub housekeeping_timer: timer_list,
    pub wrkq_flags: c_ulong,
    pub incomplete_lock: spinlock_t,
    pub incomplete_stations: list_head,
    pub joined: bool,
}

//
// struct ieee80211_mesh_sync_ops - Extensible synchronization framework interface
//
// these declarations define the interface, which enables
// vendor-specific mesh synchronization
//
// @rx_bcn_presp: beacon/probe response was received
// @adjust_tsf: TSF adjustment method
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mesh_sync_ops {
    pub rx_status): *mut ieee80211_rx_status,
// should be called with beacon_data under RCU read lock
    pub beacon): *mut beacon_data,
// add other framework functions here
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_csa_settings {
    pub rcu_head: rcu_head,
    pub settings: cfg80211_csa_settings,
}

//
// struct mesh_table - mesh hash table
//
// @known_gates: list of known mesh gates and their mpaths by the station. The
// gate's mpath may or may not be resolved and active.
// @gates_lock: protects updates to known_gates
// @rhead: the rhashtable containing struct mesh_paths, keyed by dest addr
// @walk_head: linked list containing all mesh_path objects
// @walk_lock: lock protecting walk_head
// @entries: number of entries in the table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_table {
    pub known_gates: hlist_head,
    pub gates_lock: spinlock_t,
    pub rhead: rhashtable,
    pub walk_head: hlist_head,
    pub walk_lock: spinlock_t,
    pub /: *mut *mut atomic_t entries; / Up to MAX_MESH_NEIGHBOURS,
}

//
// struct mesh_tx_cache - mesh fast xmit header cache
//
// @rht: hash table containing struct ieee80211_mesh_fast_tx, using skb DA as key
// @walk_head: linked list containing all ieee80211_mesh_fast_tx objects
// @walk_lock: lock protecting walk_head and rht
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_tx_cache {
    pub rht: rhashtable,
    pub walk_head: hlist_head,
    pub walk_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_mesh {
    pub housekeeping_timer: timer_list,
    pub mesh_path_timer: timer_list,
    pub mesh_path_root_timer: timer_list,
    pub wrkq_flags: c_ulong,
    pub BITS_PER_LONG]: unsigned long mbss_changed[64 /,
    pub userspace_handles_dfs: bool,
    pub mesh_id: [u8; IEEE80211_MAX_MESH_ID_LEN],
    pub mesh_id_len: usize,
// Active Path Selection Protocol Identifier
    pub mesh_pp_id: u8,
// Active Path Selection Metric Identifier
    pub mesh_pm_id: u8,
// Congestion Control Mode Identifier
    pub mesh_cc_id: u8,
// Synchronization Protocol Identifier
    pub mesh_sp_id: u8,
// Authentication Protocol Identifier
    pub mesh_auth_id: u8,
// Local mesh Sequence Number
    pub sn: u32,
// Last used PREQ ID
    pub preq_id: u32,
    pub mpaths: core::sync::atomic::AtomicI32,
// Timestamp of last SN update
    pub last_sn_update: c_ulong,
// Time when it's ok to send next PERR
    pub next_perr: c_ulong,
// Timestamp of last PREQ sent
    pub last_preq: c_ulong,
    pub rmc: *mut mesh_rmc,
    pub mesh_preq_queue_lock: spinlock_t,
    pub preq_queue: mesh_preq_queue,
    pub preq_queue_len: c_int,
    pub mshstats: mesh_stats,
    pub mshcfg: mesh_config,
    pub estab_plinks: core::sync::atomic::AtomicI32,
    pub mesh_seqnum: core::sync::atomic::AtomicI32,
    pub accepting_plinks: bool,
    pub num_gates: c_int,
    pub beacon: *mut beacon_data __rcu,
    pub ie: *const u8,
    pub ie_len: u8,
    pub security: },
    pub user_mpm: bool,
// Extensible Synchronization Framework
    pub sync_ops: *const ieee80211_mesh_sync_ops,
    pub sync_offset_clockdrift_max: i64,
    pub sync_offset_lock: spinlock_t,
// mesh power save
    pub nonpeer_pm: nl80211_mesh_power_mode,
    pub ps_peers_light_sleep: c_int,
    pub ps_peers_deep_sleep: c_int,
    pub ps: ps_data,
// Channel Switching Support
    pub csa: *mut mesh_csa_settings __rcu,
    pub csa_role: },
    pub chsw_ttl: u8,
    pub pre_value: u16,
// offset from skb->data while building IE
    pub meshconf_offset: c_int,
    pub mesh_paths: mesh_table,
    pub /: *mut *mut mesh_table mpp_paths; / Store paths for MPP&MAP,
    pub mesh_paths_generation: c_int,
    pub mpp_paths_generation: c_int,
    pub tx_cache: mesh_tx_cache,
}

//
// enum ieee80211_sub_if_data_flags - virtual interface flags
//
// @IEEE80211_SDATA_ALLMULTI: interface wants all multicast packets
// @IEEE80211_SDATA_DONT_BRIDGE_PACKETS: bridge packets between
// associated stations and deliver multicast frames both
// back to wireless media and to the local net stack.
// @IEEE80211_SDATA_DISCONNECT_RESUME: Disconnect after resume.
// @IEEE80211_SDATA_IN_DRIVER: indicates interface was added to driver
// @IEEE80211_SDATA_DISCONNECT_HW_RESTART: Disconnect after hardware restart
// recovery
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_sub_if_data_flags {
    IEEE80211_SDATA_ALLMULTI		= BIT(0),
    IEEE80211_SDATA_DONT_BRIDGE_PACKETS	= BIT(3),
    IEEE80211_SDATA_DISCONNECT_RESUME	= BIT(4),
    IEEE80211_SDATA_IN_DRIVER		= BIT(5),
    IEEE80211_SDATA_DISCONNECT_HW_RESTART	= BIT(6),
}

//
// enum ieee80211_sdata_state_bits - virtual interface state bits
// @SDATA_STATE_RUNNING: virtual interface is up & running; this
// mirrors netif_running() but is separate for interface type
// change handling while the interface is up
// @SDATA_STATE_OFFCHANNEL: This interface is currently in offchannel
// mode, so queues are stopped
// @SDATA_STATE_OFFCHANNEL_BEACON_STOPPED: Beaconing was stopped due
// to offchannel, reset when offchannel returns
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_sdata_state_bits {
    SDATA_STATE_RUNNING,
    SDATA_STATE_OFFCHANNEL,
    SDATA_STATE_OFFCHANNEL_BEACON_STOPPED,
}

//
// enum ieee80211_chanctx_mode - channel context configuration mode
//
// @IEEE80211_CHANCTX_SHARED: channel context may be used by
// multiple interfaces
// @IEEE80211_CHANCTX_EXCLUSIVE: channel context can be used
// only by a single interface. This can be used for example for
// non-fixed channel IBSS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_chanctx_mode {
    IEEE80211_CHANCTX_SHARED,
    IEEE80211_CHANCTX_EXCLUSIVE
}

//
// enum ieee80211_chanctx_replace_state - channel context replacement state
//
// This is used for channel context in-place reservations that require channel
// context switch/swap.
//
// @IEEE80211_CHANCTX_REPLACE_NONE: no replacement is taking place
// @IEEE80211_CHANCTX_WILL_BE_REPLACED: this channel context will be replaced
// by a (not yet registered) channel context pointed by %replace_ctx.
// @IEEE80211_CHANCTX_REPLACES_OTHER: this (not yet registered) channel context
// replaces an existing channel context pointed to by %replace_ctx.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_chanctx_replace_state {
    IEEE80211_CHANCTX_REPLACE_NONE,
    IEEE80211_CHANCTX_WILL_BE_REPLACED,
    IEEE80211_CHANCTX_REPLACES_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_chanctx {
    pub list: list_head,
    pub rcu_head: rcu_head,
    pub replace_state: ieee80211_chanctx_replace_state,
    pub replace_ctx: *mut ieee80211_chanctx,
    pub mode: ieee80211_chanctx_mode,
    pub driver_present: bool,
// temporary data for search algorithm etc.
    pub req: ieee80211_chan_req,
    pub radar_detected: bool,
// This chanctx is in process of getting used
    pub will_be_used: bool,
// MUST be last - ends in a flexible-array member.
    pub conf: ieee80211_chanctx_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac80211_qos_map {
    pub qos_map: cfg80211_qos_map,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txq_info_flags {
    IEEE80211_TXQ_STOP,
    IEEE80211_TXQ_AMPDU,
    IEEE80211_TXQ_NO_AMSDU,
    IEEE80211_TXQ_DIRTY,
}

//
// struct txq_info - per tid queue
//
// @tin: contains packets split into multiple flows
// @def_cvars: codel vars for the @tin's default_flow
// @cstats: code statistics for this queue
// @frags: used to keep fragments created after dequeue
// @schedule_order: used with ieee80211_local->active_txqs
// @schedule_round: counter to prevent infinite loops on TXQ scheduling
// @flags: TXQ flags from &enum txq_info_flags
// @txq: the driver visible part
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txq_info {
    pub tin: fq_tin,
    pub def_cvars: codel_vars,
    pub cstats: codel_stats,
    pub schedule_round: u16,
    pub schedule_order: list_head,
    pub frags: sk_buff_head,
    pub flags: c_ulong,
// keep last!
    pub txq: ieee80211_txq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_mntr {
    pub flags: u32,
    pub __aligned(2): u8 mu_follow_addr[ETH_ALEN],
    pub list: list_head,
}

//
// struct ieee80211_if_nan - NAN state
//
// @conf: current NAN configuration
// @started: true iff NAN is started
// @de: Discovery Engine state (only valid if !WIPHY_NAN_FLAGS_USERSPACE_DE)
// @de.func_lock: lock for @de.function_inst_ids
// @de.function_inst_ids: a bitmap of available instance_id's
// @removed_channels: bitmap of channels that should be removed from the NAN
// schedule once the deferred schedule update is completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_nan {
    pub conf: cfg80211_nan_conf,
    pub started: bool,
// protects function_inst_ids
    pub func_lock: spinlock_t,
    pub function_inst_ids: idr,
    pub de: },
    pub IEEE80211_NAN_MAX_CHANNELS): DECLARE_BITMAP(removed_channels,,
}

//
// struct ieee80211_if_nan_data - NAN data path state
//
// @nmi: pointer to the NAN management interface sdata. Used for data path,
// hence RCU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_if_nan_data {
    pub nmi: *mut ieee80211_sub_if_data __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_link_data_managed {
    pub __aligned(2): u8 bssid[ETH_ALEN],
    pub dtim_period: u8,
    pub /: *mut *mut driver_smps_mode; / smps mode request,
    pub conn: ieee80211_conn_settings,
    pub p2p_noa_index: i16,
    pub tdls_chan_switch_prohibited: bool,
    pub have_beacon: bool,
    pub tracking_signal_avg: bool,
    pub disable_wmm_tracking: bool,
    pub operating_11g_mode: bool,
    pub switch_work: wiphy_hrtimer_work,
    pub ap_chandef: cfg80211_chan_def,
    pub tpe: ieee80211_parsed_tpe,
    pub time: ktime_t,
    pub waiting_bcn: bool,
    pub ignored_same_chan: bool,
    pub blocked_tx: bool,
    pub csa: },
    pub request_smps_work: wiphy_work,
// used to reconfigure hardware SM PS
    pub recalc_smps: wiphy_work,
    pub beacon_crc_valid: bool,
    pub beacon_crc: u32,
    pub ave_beacon_signal: ewma_beacon_signal,
    pub last_ave_beacon_signal: c_int,
//
// Number of Beacon frames used in ave_beacon_signal. This can be used
// to avoid generating less reliable cqm events that would be based
// only on couple of received frames.
//
    pub count_beacon_signal: c_uint,
// Number of times beacon loss was invoked.
    pub beacon_loss_count: c_uint,
//
// Last Beacon frame signal strength average (ave_beacon_signal / 16)
// that triggered a cqm event. 0 indicates that no event has been
// generated for the current association.
//
    pub last_cqm_event_signal: c_int,
    pub wmm_last_param_set: c_int,
    pub mu_edca_last_param_set: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_link_data_ap {
    pub beacon: *mut beacon_data __rcu,
    pub probe_resp: *mut probe_resp __rcu,
    pub fils_discovery: *mut fils_discovery_data __rcu,
    pub unsol_bcast_probe_resp: *mut unsol_bcast_probe_resp_data __rcu,
    pub s1g_short_beacon: *mut s1g_short_beacon_data __rcu,
// to be used after channel switch.
    pub next_beacon: *mut cfg80211_beacon_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_link_data {
    pub sdata: *mut ieee80211_sub_if_data,
    pub link_id: c_uint,
// multicast keys only
    pub default_multicast_key: *mut ieee80211_key __rcu,
    pub default_mgmt_key: *mut ieee80211_key __rcu,
    pub default_beacon_key: *mut ieee80211_key __rcu,
    pub operating_11g_mode: bool,
    pub finalize_work: wiphy_work,
    pub chanreq: ieee80211_chan_req,
    pub csa: },
    pub color_change_finalize_work: wiphy_work,
    pub color_collision_detect_work: wiphy_delayed_work,
    pub color_bitmap: u64,
// context reservation -- protected with wiphy mutex
    pub reserved_chanctx: *mut ieee80211_chanctx,
    pub reserved: ieee80211_chan_req,
    pub reserved_radar_required: bool,
    pub reserved_ready: bool,
    pub needed_rx_chains: u8,
    pub smps_mode: ieee80211_smps_mode,
    pub /: *mut *mut int user_power_level; / in dBm,
    pub /: *mut *mut int ap_power_level; / in dBm,
    pub radar_required: bool,
    pub dfs_cac_timer_work: wiphy_hrtimer_work,
    pub mgd: ieee80211_link_data_managed,
    pub ap: ieee80211_link_data_ap,
    pub u: },
    pub tx_conf: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub conf: *mut ieee80211_bss_conf,
    pub he_and_lower: ieee80211_sta_rx_bandwidth,
    pub /: *mut *mut ieee80211_sta_rx_bandwidth eht; / and UHR non-DBE,
    pub bss_bw: },

    pub debugfs_dir: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sub_if_data {
    pub list: list_head,
    pub wdev: wireless_dev,
// keys
    pub key_list: list_head,
// count for keys needing tailroom space allocation
    pub crypto_tx_tailroom_needed_cnt: c_int,
    pub crypto_tx_tailroom_pending_dec: c_int,
    pub dec_tailroom_needed_wk: wiphy_delayed_work,
    pub dev: *mut net_device,
    pub local: *mut ieee80211_local,
    pub flags: c_uint,
    pub state: c_ulong,
    pub name: [c_char; IFNAMSIZ],
    pub frags: ieee80211_fragment_cache,
// TID bitmap for NoAck policy
    pub noack_map: u16,
// bit field of ACM bits (BIT(802.1D tag))
    pub wmm_acm: u8,
    pub keys: [*mut ieee80211_key __rcu; NUM_DEFAULT_KEYS],
    pub default_unicast_key: *mut ieee80211_key __rcu,
    pub sequence_number: u16,
    pub mld_mcast_seq: u16,
    pub control_port_protocol: __be16,
    pub control_port_no_encrypt: bool,
    pub control_port_no_preauth: bool,
    pub control_port_over_nl80211: bool,
    pub num_tx_queued: core::sync::atomic::AtomicI32,
    pub qos_map: *mut mac80211_qos_map __rcu,
    pub work: wiphy_work,
    pub skb_queue: sk_buff_head,
    pub status_queue: sk_buff_head,
//
// AP this belongs to: self in AP mode and
// corresponding AP in VLAN mode, NULL for
// all others (might be needed later in IBSS)
//
    pub bss: *mut ieee80211_if_ap,
// bitmap of allowed (non-MCS) rate indexes for rate control
    pub rc_rateidx_mask: [u32; NUM_NL80211_BANDS],
    pub rc_has_mcs_mask: [bool; NUM_NL80211_BANDS],
    pub rc_rateidx_mcs_mask: [u8; NUM_NL80211_BANDS][IEEE80211_HT_MCS_MASK_LEN],
    pub rc_has_vht_mcs_mask: [bool; NUM_NL80211_BANDS],
    pub rc_rateidx_vht_mcs_mask: [u16; NUM_NL80211_BANDS][NL80211_VHT_NSS_MAX],
// Beacon frame (non-MCS) rate (as a bitmap)
    pub beacon_rateidx_mask: [u32; NUM_NL80211_BANDS],
    pub beacon_rate_set: bool,
    pub ap: ieee80211_if_ap,
    pub vlan: ieee80211_if_vlan,
    pub mgd: ieee80211_if_managed,
    pub ibss: ieee80211_if_ibss,
    pub mesh: ieee80211_if_mesh,
    pub ocb: ieee80211_if_ocb,
    pub mntr: ieee80211_if_mntr,
    pub nan: ieee80211_if_nan,
    pub nan_data: ieee80211_if_nan_data,
    pub u: },
    pub deflink: ieee80211_link_data,
    pub link: [*mut ieee80211_link_data __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
// for ieee80211_set_active_links_async()
    pub activate_links_work: wiphy_work,
    pub desired_active_links: u16,
    pub restart_active_links: u16,

    pub subdir_stations: *mut dentry,
    pub default_unicast_key: *mut dentry,
    pub default_multicast_key: *mut dentry,
    pub default_mgmt_key: *mut dentry,
    pub default_beacon_key: *mut dentry,
    pub debugfs: },

    pub tx_handlers_drop: u32,
// must be last, dynamically sized area in this!
    pub vif: ieee80211_vif,
}

extern "C" {
    pub fn container_of(_arg: p, ieee80211_sub_if_data: struct, _arg: vif) -> return;
}

// outer loop just to define the variables ... */		\
//
// for_each_sdata_link_rcu() must be used under RCU read lock.
//

// outer loop just to define the variables ... */				\

// outer loop just to define the variable ... */			\
//
// for_each_link_data_rcu should be used under RCU read lock.
//

// outer loop just to define the variable ... */			\
// i == elems->cnt, calculate total length of all MBSSID elements
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum queue_stop_reason {
    IEEE80211_QUEUE_STOP_REASON_DRIVER,
    IEEE80211_QUEUE_STOP_REASON_PS,
    IEEE80211_QUEUE_STOP_REASON_CSA,
    IEEE80211_QUEUE_STOP_REASON_AGGREGATION,
    IEEE80211_QUEUE_STOP_REASON_SUSPEND,
    IEEE80211_QUEUE_STOP_REASON_SKB_ADD,
    IEEE80211_QUEUE_STOP_REASON_OFFCHANNEL,
    IEEE80211_QUEUE_STOP_REASON_FLUSH,
    IEEE80211_QUEUE_STOP_REASON_TDLS_TEARDOWN,
    IEEE80211_QUEUE_STOP_REASON_RESERVE_TID,
    IEEE80211_QUEUE_STOP_REASON_IFTYPE_CHANGE,

    IEEE80211_QUEUE_STOP_REASONS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpt_led_trigger {
    pub name: [c_char; 32],
    pub blink_table: *const ieee80211_tpt_blink,
    pub blink_table_len: c_uint,
    pub timer: timer_list,
    pub local: *mut ieee80211_local,
    pub prev_traffic: c_ulong,
    pub rx_bytes: unsigned long tx_bytes,,
    pub want: unsigned int active,,
    pub running: bool,
}

//
// enum mac80211_scan_flags - currently active scan mode
//
// @SCAN_SW_SCANNING: We're currently in the process of scanning but may as
// well be on the operating channel
// @SCAN_HW_SCANNING: The hardware is scanning for us, we have no way to
// determine if we are on the operating channel or not
// @SCAN_ONCHANNEL_SCANNING:  Do a software scan on only the current operating
// channel. This should not interrupt normal traffic.
// @SCAN_COMPLETED: Set for our scan work function when the driver reported
// that the scan completed.
// @SCAN_ABORTED: Set for our scan work function when the driver reported
// a scan complete for an aborted scan.
// @SCAN_HW_CANCELLED: Set for our scan work function when the scan is being
// cancelled.
// @SCAN_BEACON_WAIT: Set whenever we're passive scanning because of radar/no-IR
// and could send a probe request after receiving a beacon.
// @SCAN_BEACON_DONE: Beacon received, we can now send a probe request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac80211_scan_flags {
    SCAN_SW_SCANNING,
    SCAN_HW_SCANNING,
    SCAN_ONCHANNEL_SCANNING,
    SCAN_COMPLETED,
    SCAN_ABORTED,
    SCAN_HW_CANCELLED,
    SCAN_BEACON_WAIT,
    SCAN_BEACON_DONE,
}

//
// enum mac80211_scan_state - scan state machine states
//
// @SCAN_DECISION: Main entry point to the scan state machine, this state
// determines if we should keep on scanning or switch back to the
// operating channel
// @SCAN_SET_CHANNEL: Set the next channel to be scanned
// @SCAN_SEND_PROBE: Send probe requests and wait for probe responses
// @SCAN_SUSPEND: Suspend the scan and go back to operating channel to
// send out data
// @SCAN_RESUME: Resume the scan and scan the next channel
// @SCAN_ABORT: Abort the scan and go back to operating channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac80211_scan_state {
    SCAN_DECISION,
    SCAN_SET_CHANNEL,
    SCAN_SEND_PROBE,
    SCAN_SUSPEND,
    SCAN_RESUME,
    SCAN_ABORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_local {
// embed the driver visible part.
// don't cast (use the static inlines below), but we keep
// it first anyway so they become a no-op
    pub hw: ieee80211_hw,
    pub fq: fq,
    pub cvars: *mut codel_vars,
    pub cparams: codel_params,
// protects active_txqs and txqi->schedule_order
    pub active_txq_lock: [spinlock_t; IEEE80211_NUM_ACS],
    pub active_txqs: [list_head; IEEE80211_NUM_ACS],
    pub schedule_round: [u16; IEEE80211_NUM_ACS],
// serializes ieee80211_handle_wake_tx_queue
    pub handle_wake_tx_queue_lock: spinlock_t,
    pub airtime_flags: u16,
    pub aql_txq_limit_mc: u32,
    pub aql_txq_limit_low: [u32; IEEE80211_NUM_ACS],
    pub aql_txq_limit_high: [u32; IEEE80211_NUM_ACS],
    pub aql_threshold: u32,
    pub aql_total_pending_airtime: core::sync::atomic::AtomicI32,
    pub aql_mc_pending_airtime: core::sync::atomic::AtomicI32,
    pub aql_ac_pending_airtime: [core::sync::atomic::AtomicI32; IEEE80211_NUM_ACS],
    pub ops: *const ieee80211_ops,
//
// private workqueue to mac80211. mac80211 makes this accessible
// via ieee80211_queue_work()
//
    pub workqueue: *mut workqueue_struct,
    pub queue_stop_reasons: [c_ulong; IEEE80211_MAX_QUEUES],
    pub q_stop_reasons: [c_int; IEEE80211_MAX_QUEUES][IEEE80211_QUEUE_STOP_REASONS],
// also used to protect ampdu_ac_queue and amdpu_ac_stop_refcnt
    pub queue_stop_reason_lock: spinlock_t,
    pub open_count: c_int,
    pub tx_mntrs: int monitors, virt_monitors,,
// number of interfaces with corresponding FIF_ flags
    pub probe_req_reg: bool,
    pub rx_mcast_action_reg: bool,
    pub /: *mut *mut *mut unsigned int filter_flags; / FIF_,
    pub dflt_chandef: cfg80211_chan_def,
    pub emulate_chanctx: bool,
// protects the aggregated multicast list and filter calls
    pub filter_lock: spinlock_t,
// used for uploading changed mc list
    pub reconfig_filter: wiphy_work,
// aggregated multicast list
    pub mc_list: netdev_hw_addr_list,
    pub /: *mut *mut bool tim_in_locked_section; / see ieee80211_beacon_get(),
//
// suspended is true if we finished all the suspend _and_ we have
// not yet come up from resume. This is to be used by mac80211
// to ensure driver sanity during suspend and mac80211's own
// sanity. It can eventually be used for WoW as well.
//
    pub suspended: bool,
// suspending is true during the whole suspend process
    pub suspending: bool,
//
// Resuming is true while suspended, but when we're reprogramming the
// hardware -- at that time it's allowed to use ieee80211_queue_work()
// again even though some other parts of the stack are still suspended
// and we still drop received frames to avoid waking the stack.
//
    pub resuming: bool,
//
// quiescing is true during the suspend process _only_ to
// ease timer cancelling etc.
//
    pub quiescing: bool,
// device is started
    pub started: bool,
// device is during a HW reconfig
    pub in_reconfig: bool,
// reconfiguration failed ... suppress some warnings etc.
    pub reconfig_failure: bool,
// wowlan is enabled -- don't reconfig on resume
    pub wowlan: bool,
    pub radar_detected_work: wiphy_work,
// number of RX chains the hardware has
    pub rx_chains: u8,
// bitmap of which sbands were copied
    pub sband_allocated: u8,
    pub /: *mut *mut int tx_headroom; / required headroom for hardware/radiotap,
// Tasklet and skb queue to process calls from IRQ mode. All frames
// added to skb_queue will be processed, but frames in
// skb_queue_unreliable may be dropped if the total length of these
// queues increases over the limit.
pub const IEEE80211_IRQSAFE_QUEUE_LIMIT: c_int = 128;
    pub tasklet: tasklet_struct,
    pub skb_queue: sk_buff_head,
    pub skb_queue_unreliable: sk_buff_head,
    pub rx_path_lock: spinlock_t,
// Station data
//
// The list, hash table and counter are protected
// by the wiphy mutex, reads are done with RCU.
//
    pub tim_lock: spinlock_t,
    pub num_sta: c_ulong,
    pub sta_list: list_head,
    pub sta_hash: rhltable,
    pub link_sta_hash: rhltable,
    pub sta_cleanup: timer_list,
    pub sta_generation: c_int,
    pub pending: [sk_buff_head; IEEE80211_MAX_QUEUES],
    pub tx_pending_tasklet: tasklet_struct,
    pub wake_txqs_tasklet: tasklet_struct,
    pub agg_queue_stop: [core::sync::atomic::AtomicI32; IEEE80211_MAX_QUEUES],
// number of interfaces with allmulti RX
    pub iff_allmultis: core::sync::atomic::AtomicI32,
    pub rate_ctrl: *mut rate_control_ref,
    pub wep_tx_ctx: arc4_ctx,
    pub wep_rx_ctx: arc4_ctx,
    pub wep_iv: u32,
// see iface.c
    pub interfaces: list_head,
    pub /: *mut *mut list_head mon_list; / only that are IFF_UP,
    pub iflist_mtx: mutex,
// Scanning and BSS list
    pub scanning: c_ulong,
    pub scan_ssid: cfg80211_ssid,
    pub int_scan_req: *mut cfg80211_scan_request,
    pub scan_req: *mut cfg80211_scan_request __rcu,
    pub hw_scan_req: *mut ieee80211_scan_request,
    pub scan_chandef: cfg80211_chan_def,
    pub hw_scan_band: nl80211_band,
    pub scan_channel_idx: c_int,
    pub scan_ies_len: c_int,
    pub hw_scan_ies_bufsize: c_int,
    pub scan_info: cfg80211_scan_info,
    pub sched_scan_stopped_work: wiphy_work,
    pub sched_scan_sdata: *mut ieee80211_sub_if_data __rcu,
    pub sched_scan_req: *mut cfg80211_sched_scan_request __rcu,
    pub scan_addr: [u8; ETH_ALEN],
    pub leave_oper_channel_time: c_ulong,
    pub next_scan_state: mac80211_scan_state,
    pub scan_work: wiphy_delayed_work,
    pub scan_sdata: *mut ieee80211_sub_if_data __rcu,
// Temporary remain-on-channel for off-channel operations
    pub tmp_channel: *mut ieee80211_channel,
// channel contexts
    pub chanctx_list: list_head,

    pub radio_led: led_trigger tx_led, rx_led, assoc_led,,
    pub tpt_led: led_trigger,
    pub assoc_led_active: atomic_t tx_led_active, rx_led_active,,
    pub tpt_led_active: atomic_t radio_led_active,,
    pub tpt_led_trigger: *mut tpt_led_trigger,

// SNMP counters
// dot11CountersTable
    pub dot11TransmittedFragmentCount: u32,
    pub dot11MulticastTransmittedFrameCount: u32,
    pub dot11FailedCount: u32,
    pub dot11RetryCount: u32,
    pub dot11MultipleRetryCount: u32,
    pub dot11FrameDuplicateCount: u32,
    pub dot11ReceivedFragmentCount: u32,
    pub dot11MulticastReceivedFrameCount: u32,
    pub dot11TransmittedFrameCount: u32,
// TX/RX handler statistics
    pub tx_handlers_queued: c_uint,
    pub tx_handlers_drop_wep: c_uint,
    pub tx_handlers_drop_not_assoc: c_uint,
    pub tx_handlers_drop_unauth_port: c_uint,
    pub rx_handlers_drop: c_uint,
    pub rx_handlers_queued: c_uint,
    pub rx_handlers_drop_nullfunc: c_uint,
    pub rx_handlers_drop_defrag: c_uint,
    pub tx_expand_skb_head: c_uint,
    pub tx_expand_skb_head_cloned: c_uint,
    pub rx_expand_skb_head_defrag: c_uint,
    pub rx_handlers_fragments: c_uint,
    pub tx_status_drop: c_uint,

    pub and: *mut *mut int total_ps_buffered; / total number of all buffered unicast,
// multicast packets for power saving stations
//
    pub pspolling: bool,
//
// PS can only be enabled when we have exactly one managed
// interface (and monitors) in PS, this then points there.
//
    pub ps_sdata: *mut ieee80211_sub_if_data,
    pub dynamic_ps_enable_work: wiphy_work,
    pub dynamic_ps_disable_work: wiphy_work,
    pub dynamic_ps_timer: timer_list,
    pub ifa_notifier: notifier_block,
    pub ifa6_notifier: notifier_block,
//
// The dynamic ps timeout configured from user space via WEXT -
// this will override whatever chosen by mac80211 internally.
//
    pub dynamic_ps_forced_timeout: c_int,
    pub /: *mut *mut int user_power_level; / in dBm, for all interfaces,
    pub restart_work: work_struct,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_debugfsdentries {
    pub rcdir: *mut dentry,
    pub keys: *mut dentry,
    pub debugfs: },
    pub force_tx_status: bool,

//
// Remain-on-channel support
//
    pub roc_work: wiphy_delayed_work,
    pub roc_list: list_head,
    pub hw_roc_done: wiphy_work hw_roc_start,,
    pub hw_roc_start_time: c_ulong,
    pub ack_status_frames: idr,
    pub ack_status_lock: spinlock_t,
// virtual monitor interface
    pub monitor_sdata: *mut ieee80211_sub_if_data __rcu,
    pub monitor_chanreq: ieee80211_chan_req,
// extended capabilities provided by mac80211
    pub ext_capa: [u8; 8],
    pub wbrf_supported: bool,
}

extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: wdev, ieee80211_sub_if_data: struct, _arg: wdev) -> return;
}
// this struct holds the value parsing from channel switch IE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_csa_ie {
    pub chanreq: ieee80211_chan_req,
    pub mode: u8,
    pub count: u8,
    pub ttl: u8,
    pub pre_value: u16,
    pub reason_code: u16,
    pub max_switch_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_elems_parse_error {
    IEEE80211_PARSE_ERR_INVALID_END		= BIT(0),
    IEEE80211_PARSE_ERR_DUP_ELEM		= BIT(1),
    IEEE80211_PARSE_ERR_BAD_ELEM_SIZE	= BIT(2),
    IEEE80211_PARSE_ERR_UNEXPECTED_ELEM	= BIT(3),
    IEEE80211_PARSE_ERR_DUP_NEST_ML_BASIC	= BIT(4),
}

// Parsed Information Elements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802_11_elems {
    pub ie_start: *const u8,
    pub total_len: usize,
    pub crc: u32,
    pub frame_type: u8,
    pub from_ap: bool,
// pointers to IEs
    pub lnk_id: *const ieee80211_tdls_lnkie,
    pub ch_sw_timing: *const ieee80211_ch_switch_timing,
    pub ext_capab: *const u8,
    pub ssid: *const u8,
    pub supp_rates: *const u8,
    pub ds_params: *const u8,
    pub tim: *const ieee80211_tim_ie,
    pub rsn: *const u8,
    pub rsnx: *const u8,
    pub erp_info: *const u8,
    pub ext_supp_rates: *const u8,
    pub wmm_info: *const u8,
    pub wmm_param: *const u8,
    pub ht_cap_elem: *const ieee80211_ht_cap,
    pub ht_operation: *const ieee80211_ht_operation,
    pub vht_cap_elem: *const ieee80211_vht_cap,
    pub vht_operation: *const ieee80211_vht_operation,
    pub mesh_config: *const ieee80211_meshconf_ie,
    pub he_cap: *const u8,
    pub he_operation: *const ieee80211_he_operation,
    pub he_spr: *const ieee80211_he_spr,
    pub mu_edca_param_set: *const ieee80211_mu_edca_param_set,
    pub he_6ghz_capa: *const ieee80211_he_6ghz_capa,
    pub uora_element: *const u8,
    pub mesh_id: *const u8,
    pub peering: *const u8,
    pub awake_window: *const __le16,
    pub preq: *const u8,
    pub prep: *const u8,
    pub perr: *const u8,
    pub rann: *const ieee80211_rann_ie,
    pub ch_switch_ie: *const ieee80211_channel_sw_ie,
    pub ext_chansw_ie: *const ieee80211_ext_chansw_ie,
    pub wide_bw_chansw_ie: *const ieee80211_wide_bw_chansw_ie,
    pub max_channel_switch_time: *const u8,
    pub country_elem: *const u8,
    pub pwr_constr_elem: *const u8,
    pub cisco_dtpc_elem: *const u8,
    pub timeout_int: *const ieee80211_timeout_interval_ie,
    pub opmode_notif: *const u8,
    pub sec_chan_offs: *const ieee80211_sec_chan_offs_ie,
    pub mesh_chansw_params_ie: *mut ieee80211_mesh_chansw_params_ie,
    pub max_idle_period_ie: *const ieee80211_bss_max_idle_period_ie,
    pub mbssid_config_ie: *const ieee80211_multiple_bssid_configuration,
    pub bssid_index: *const ieee80211_bssid_index,
    pub max_bssid_indicator: u8,
    pub dtim_count: u8,
    pub dtim_period: u8,
    pub addba_ext_ie: *const ieee80211_addba_ext_ie,
    pub s1g_capab: *const ieee80211_s1g_cap,
    pub s1g_oper: *const ieee80211_s1g_oper_ie,
    pub s1g_bcn_compat: *const ieee80211_s1g_bcn_compat_ie,
    pub aid_resp: *const ieee80211_aid_response_ie,
    pub eht_cap: *const ieee80211_eht_cap_elem,
    pub eht_operation: *const ieee80211_eht_operation,
    pub ml_basic: *const ieee80211_multi_link_elem,
    pub ml_reconf: *const ieee80211_multi_link_elem,
    pub ml_epcs: *const ieee80211_multi_link_elem,
    pub bandwidth_indication: *const ieee80211_bandwidth_indication,
    pub ttlm: [*const ieee80211_ttlm_elem; IEEE80211_TTLM_MAX_CNT],
    pub uhr_cap: *const ieee80211_uhr_cap,
    pub uhr_operation: *const ieee80211_uhr_operation,
// not the order in the psd values is per element, not per chandef
    pub tpe: ieee80211_parsed_tpe,
    pub csa_tpe: ieee80211_parsed_tpe,
// length of them, respectively
    pub ext_capab_len: u8,
    pub ssid_len: u8,
    pub supp_rates_len: u8,
    pub tim_len: u8,
    pub rsn_len: u8,
    pub rsnx_len: u8,
    pub ext_supp_rates_len: u8,
    pub wmm_info_len: u8,
    pub wmm_param_len: u8,
    pub he_cap_len: u8,
    pub mesh_id_len: u8,
    pub peering_len: u8,
    pub preq_len: u8,
    pub prep_len: u8,
    pub perr_len: u8,
    pub country_elem_len: u8,
    pub bssid_index_len: u8,
    pub eht_cap_len: u8,
    pub uhr_cap_len: u8,
    pub uhr_operation_len: u8,
// mult-link element can be de-fragmented and thus u8 is not sufficient
    pub ml_basic_len: usize,
    pub ml_reconf_len: usize,
    pub ml_epcs_len: usize,
    pub ttlm_num: u8,
//
// store the per station profile pointer and length in case that the
// parsing also handled Multi-Link element parsing for a specific link
// ID.
//
    pub prof: *mut ieee80211_mle_per_sta_profile,
    pub sta_prof_len: usize,
//
// When parsing the beacon with MBSSID (from a transmitted BSS), this
// indicates that the profile the parser was instructed to look for
// (via the bss value in &struct ieee80211_elems_parse_params) couldn't
// be found (due to EMA, or perhaps broken AP) and the result cannot be
// considered complete.
//
    pub mbssid_nontx_profile_missing: bool,
// whether/which parse error occurred while retrieving these elements
    pub parse_error: u8,
}

extern "C" {
    pub fn container_of(_arg: hw, ieee80211_local: struct, _arg: hw) -> return;
}
extern "C" {
    pub fn container_of(_arg: txq, txq_info: struct, _arg: txq) -> return;
}
extern "C" {
    pub fn ieee80211_vif_inc_num_mcast(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_vif_dec_num_mcast(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_vif_block_queues_csa(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_vif_unblock_queues_csa(sdata: *mut ieee80211_sub_if_data);
}
// This function returns the number of multicast stations connected to this
// interface. It returns -1 if that number is not tracked, that is for netdevs
// not in AP or AP_VLAN mode or when using 4addr.
//
extern "C" {
    pub fn atomic_read(_arg: &sdata->u.ap.num_mcast_sta) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &sdata->u.vlan.num_mcast_sta) -> return;
}
extern "C" {
    pub fn test_bit(_arg: SDATA_STATE_RUNNING, _arg: &sdata->state) -> return;
}
extern "C" {
    pub fn ieee80211_hw_conf_chan(local: *mut ieee80211_local) -> c_int;
}
extern "C" {
    pub fn ieee80211_hw_conf_init(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_tx_set_protected(tx: *mut ieee80211_tx_data);
}
extern "C" {
    pub fn ieee80211_configure_filter(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_reset_erp_info(sdata: *mut ieee80211_sub_if_data) -> u64;
}
extern "C" {
    pub fn ieee80211_handle_queued_frames(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_check_fast_rx(sta: *mut sta_info);
}
extern "C" {
    pub fn __ieee80211_check_fast_rx_iface(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_check_fast_rx_iface(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_clear_fast_rx(sta: *mut sta_info);
}
// AP code
extern "C" {
    pub fn ieee80211_uhr_disable_dbe_all_stas(link: *mut ieee80211_link_data);
}
// STA code
extern "C" {
    pub fn ieee80211_sta_setup_sdata(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_recalc_ps(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_recalc_ps_vif(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_sta_work(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_sta_reset_beacon_monitor(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_sta_reset_conn_monitor(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_mgd_stop(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_mgd_quiesce(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_sta_restart(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_sta_handle_tspec_ac_params(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_mgd_setup_link(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_mgd_stop_link(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_mgd_set_link_qos_params(link: *mut ieee80211_link_data);
}
// IBSS code
extern "C" {
    pub fn ieee80211_ibss_notify_scan_completed(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_ibss_setup_sdata(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_ibss_leave(sdata: *mut ieee80211_sub_if_data) -> c_int;
}
extern "C" {
    pub fn ieee80211_ibss_work(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_ibss_stop(sdata: *mut ieee80211_sub_if_data);
}
// OCB code
extern "C" {
    pub fn ieee80211_ocb_work(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_ocb_setup_sdata(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_ocb_leave(sdata: *mut ieee80211_sub_if_data) -> c_int;
}
// mesh code
extern "C" {
    pub fn ieee80211_mesh_work(sdata: *mut ieee80211_sub_if_data);
}
// NAN code
extern "C" {
    pub fn ieee80211_nan_free_peer_sched(sched: *mut ieee80211_nan_peer_sched);
}
extern "C" {
    pub fn ieee80211_nan_update_ndi_carrier(ndi_sdata: *mut ieee80211_sub_if_data);
}
// Find the NAN interface - there can only be one
// scan/BSS handling
extern "C" {
    pub fn ieee80211_scan_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn ieee80211_scan_cancel(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_run_deferred_scan(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_scan_rx(local: *mut ieee80211_local, skb: *mut sk_buff);
}
extern "C" {
    pub fn ieee80211_mlme_notify_scan_completed(local: *mut ieee80211_local);
}
// scheduled scan handling
extern "C" {
    pub fn ieee80211_request_sched_scan_stop(local: *mut ieee80211_local) -> c_int;
}
extern "C" {
    pub fn ieee80211_sched_scan_end(local: *mut ieee80211_local);
}
// off-channel/mgmt-tx
extern "C" {
    pub fn ieee80211_offchannel_stop_vifs(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_offchannel_return(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_roc_setup(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_start_next_roc(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_reconfig_roc(local: *mut ieee80211_local);
}
// channel switch handling
extern "C" {
    pub fn ieee80211_csa_finalize_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
// color change handling
// interface handling

extern "C" {
    pub fn ieee80211_iface_init() -> c_int;
}
extern "C" {
    pub fn ieee80211_iface_exit();
}
extern "C" {
    pub fn ieee80211_if_remove(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_remove_interfaces(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_idle_off(local: *mut ieee80211_local) -> u32;
}
extern "C" {
    pub fn ieee80211_recalc_idle(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_do_open(wdev: *mut wireless_dev, coming_up: bool) -> c_int;
}
extern "C" {
    pub fn ieee80211_sdata_stop(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_del_virtual_monitor(local: *mut ieee80211_local);
}
extern "C" {
    pub fn __ieee80211_recalc_txpower(link: *mut ieee80211_link_data) -> bool;
}
extern "C" {
    pub fn ieee80211_recalc_offload(local: *mut ieee80211_local);
}
// link handling
extern "C" {
    pub fn ieee80211_link_setup(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_link_stop(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_apvlan_link_setup(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_apvlan_link_clear(sdata: *mut ieee80211_sub_if_data);
}
// tx handling
extern "C" {
    pub fn ieee80211_clear_tx_pending(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_tx_pending(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ieee80211_check_fast_xmit(sta: *mut sta_info);
}
extern "C" {
    pub fn ieee80211_check_fast_xmit_all(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_check_fast_xmit_iface(sdata: *mut ieee80211_sub_if_data);
}
extern "C" {
    pub fn ieee80211_clear_fast_xmit(sta: *mut sta_info);
}
// HT
extern "C" {
    pub fn ieee80211_ba_session_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn ieee80211_tx_ba_session_handle_start(sta: *mut sta_info, tid: c_int);
}
extern "C" {
    pub fn ieee80211_release_reorder_timeout(sta: *mut sta_info, tid: c_int);
}
extern "C" {
    pub fn ieee80211_mcs_to_chains(mcs: *const ieee80211_mcs_info) -> u8;
}
// VHT
// HE
// S1G
extern "C" {
    pub fn ieee80211_s1g_sta_rate_init(sta: *mut sta_info);
}
extern "C" {
    pub fn ieee80211_s1g_is_twt_setup(skb: *mut sk_buff) -> bool;
}
// Spectrum management
//
// ieee80211_parse_ch_switch_ie - parses channel switch IEs
// @sdata: the sdata of the interface which has received the frame
// @elems: parsed 802.11 elements received with the frame
// @current_band: indicates the current band
// @vht_cap_info: VHT capabilities of the transmitter
// @conn: contains information about own capabilities and restrictions
// to decide which channel switch announcements can be accepted
// @bssid: the currently connected bssid (for reporting)
// @unprot_action: whether the frame was an unprotected frame or not,
// used for reporting
// @csa_ie: parsed 802.11 csa elements on count, mode, chandef and mesh ttl.
// All of them will be filled with if success only.
// Return: 0 on success, <0 on error and >0 if there is nothing to parse.
//
// Suspend/resume and hw reconfiguration
extern "C" {
    pub fn ieee80211_reconfig(local: *mut ieee80211_local) -> c_int;
}
extern "C" {
    pub fn ieee80211_stop_device(local: *mut ieee80211_local, suspend: bool);
}
extern "C" {
    pub fn ieee80211_reconfig(_arg: hw_to_local(hw)) -> return;
}
// utility functions/constants
extern "C" {
    pub fn ieee80211_clear_tpe(tpe: *mut ieee80211_parsed_tpe);
}
// sta_out needs to be checked for ERR_PTR() before using
// Send all internal mgmt frames on VO. Accordingly set TID to 7.
//
// struct ieee80211_elems_parse_params - element parsing parameters
// @mode: connection mode for parsing
// @start: pointer to the elements
// @len: length of the elements
// @type: type of the frame the elements came from
// (action, probe response, beacon, etc.)
// @filter: bitmap of element IDs to filter out while calculating
// the element CRC
// @crc: CRC starting value
// @bss: the BSS to parse this as, for multi-BSSID cases this can
// represent a non-transmitting BSS in which case the data
// for that non-transmitting BSS is returned
// @link_id: the link ID to parse elements for, if a STA profile
// is present in the multi-link element, or -1 to ignore;
// note that the code currently assumes parsing an association
// (or re-association) response frame if this is given
// @from_ap: frame is received from an AP (currently used only
// for EHT capabilities parsing)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_elems_parse_params {
    pub mode: ieee80211_conn_mode,
    pub start: *const u8,
    pub len: usize,
    pub type: u8,
    pub filter: u64,
    pub crc: u32,
    pub bss: *mut cfg80211_bss,
    pub link_id: c_int,
    pub from_ap: bool,
}

extern "C" {
    pub fn ieee802_11_parse_elems_full(_arg: &params) -> return;
}
extern "C" {
    pub fn ieee80211_dynamic_ps_timer(t: *mut timer_list);
}
//
// It's unsafe to try to do any work during reconfigure flow.
// When the flow ends the work will be requeued.
//
// If quiescing is set, we are racing with __ieee80211_suspend.
// __ieee80211_suspend flushes the workers after setting quiescing,
// and we check quiescing / suspended before enqueuing new workers.
// We should abort the worker to avoid the races below.
//
// We might already be suspended if the following scenario occurs:
// __ieee80211_suspend		Control path
//
// if (local->quiescing)
// return;
// local->quiescing = true;
// flush_workqueue();
// queue_work(...);
// local->suspended = true;
// local->quiescing = false;
// worker starts running...
//
extern "C" {
    pub fn ieee80211_txq_setup_flows(local: *mut ieee80211_local) -> c_int;
}
extern "C" {
    pub fn ieee80211_txq_set_params(local: *mut ieee80211_local, radio_idx: c_int);
}
extern "C" {
    pub fn ieee80211_txq_teardown_flows(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_purge_sta_txqs(sta: *mut sta_info);
}
extern "C" {
    pub fn ieee80211_wake_txqs(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ieee80211_ie_split_vendor(ies: *const u8, ielen: usize, offset: usize) -> usize;
}
extern "C" {
    pub fn ieee80211_ie_len_he_cap(sdata: *mut ieee80211_sub_if_data) -> u8;
}
// element building in SKBs
// channel management
extern "C" {
    pub fn _ieee80211_link_use_channel(_arg: link, _arg: req, _arg: mode, _arg: false) -> return;
}
extern "C" {
    pub fn ieee80211_link_unreserve_chanctx(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_link_release_channel(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_link_vlan_copy_chanctx(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_dfs_cac_timer_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn ieee80211_recalc_sb_count(sdata: *mut ieee80211_sub_if_data, tsf: u64);
}
extern "C" {
    pub fn ieee80211_recalc_dtim(sdata: *mut ieee80211_sub_if_data, tsf: u64);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_check_combinations_data {
    pub chandef: *const cfg80211_chan_def,
    pub chanmode: ieee80211_chanctx_mode,
    pub radar_detect: u8,
    pub radio_idx: c_int,
    pub filter_data): *mut c_void,
    pub filter_data: *mut c_void,
}

extern "C" {
    pub fn ieee80211_check_combinations_ext(_arg: sdata, _arg: &data) -> return;
}
extern "C" {
    pub fn ieee80211_max_num_channels(local: *mut ieee80211_local, radio_idx: c_int) -> c_int;
}
extern "C" {
    pub fn ieee80211_get_radio_mask(wiphy: *mut wiphy, dev: *mut net_device) -> u32;
}
// TDLS
extern "C" {
    pub fn ieee80211_tdls_peer_del_work(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
extern "C" {
    pub fn ieee80211_teardown_tdls_peers(link: *mut ieee80211_link_data);
}
extern "C" {
    pub fn ieee80211_encode_usf(val: c_int) -> u16;
}

// Macro flag: #define debug_noinline

extern "C" {
    pub fn ieee80211_init_frag_cache(cache: *mut ieee80211_fragment_cache);
}
extern "C" {
    pub fn ieee80211_destroy_frag_cache(cache: *mut ieee80211_fragment_cache);
}
extern "C" {
    pub fn ieee80211_ie_len_eht_cap(sdata: *mut ieee80211_sub_if_data) -> u8;
}
extern "C" {
    pub fn ieee80211_check_wbrf_support(local: *mut ieee80211_local);
}
extern "C" {
    pub fn ieee80211_add_wbrf(local: *mut ieee80211_local, chandef: *mut cfg80211_chan_def);
}
extern "C" {
    pub fn ieee80211_remove_wbrf(local: *mut ieee80211_local, chandef: *mut cfg80211_chan_def);
}
extern "C" {
    pub fn ieee80211_mgd_set_epcs(sdata: *mut ieee80211_sub_if_data, enable: bool) -> c_int;
}
extern "C" {
    pub fn ieee80211_stop_mbssid(sdata: *mut ieee80211_sub_if_data);
}

// Macro flag: #define VISIBLE_IF_MAC80211_KUNIT

// Macro flag: #define EXPORT_SYMBOL_IF_MAC80211_KUNIT(sym)

