//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/core.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const MAX_ATH6KL: c_int = 1;
pub const ATH6KL_MAX_RX_BUFFERS: c_int = 16;
pub const ATH6KL_BUFFER_SIZE: c_int = 1664;
pub const ATH6KL_MAX_AMSDU_RX_BUFFERS: c_int = 4;
pub const ATH6KL_AMSDU_REFILL_THRESHOLD: c_int = 3;

pub const MAX_MSDU_SUBFRAME_PAYLOAD_LEN: c_int = 1508;
pub const MIN_MSDU_SUBFRAME_PAYLOAD_LEN: c_int = 46;
pub const USER_SAVEDKEYS_STAT_INIT: c_int = 0;
pub const USER_SAVEDKEYS_STAT_RUN: c_int = 1;
pub const ATH6KL_TX_TIMEOUT: c_int = 10;
pub const ATH6KL_MAX_ENDPOINTS: c_int = 4;
pub const MAX_NODE_NUM: c_int = 15;
pub const ATH6KL_APSD_ALL_FRAME: c_uint = 0xFFFF;
pub const ATH6KL_APSD_NUM_OF_AC: c_uint = 0x4;
pub const ATH6KL_APSD_FRAME_MASK: c_uint = 0xF;
// Extra bytes for htc header alignment
pub const ATH6KL_HTC_ALIGN_BYTES: c_int = 3;
// MAX_HI_COOKIE_NUM are reserved for high priority traffic
pub const MAX_DEF_COOKIE_NUM: c_int = 180;

// Channel dwell time in fg scan

// includes also the null byte

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_fw_ie_type {
    ATH6KL_FW_IE_FW_VERSION = 0,
    ATH6KL_FW_IE_TIMESTAMP = 1,
    ATH6KL_FW_IE_OTP_IMAGE = 2,
    ATH6KL_FW_IE_FW_IMAGE = 3,
    ATH6KL_FW_IE_PATCH_IMAGE = 4,
    ATH6KL_FW_IE_RESERVED_RAM_SIZE = 5,
    ATH6KL_FW_IE_CAPABILITIES = 6,
    ATH6KL_FW_IE_PATCH_ADDR = 7,
    ATH6KL_FW_IE_BOARD_ADDR = 8,
    ATH6KL_FW_IE_VIF_MAX = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_fw_capability {
    ATH6KL_FW_CAPABILITY_HOST_P2P = 0,
    ATH6KL_FW_CAPABILITY_SCHED_SCAN = 1,

//
// Firmware is capable of supporting P2P mgmt operations on a
// station interface. After group formation, the station
// interface will become a P2P client/GO interface as the case may be
//
    ATH6KL_FW_CAPABILITY_STA_P2PDEV_DUPLEX,

//
// Firmware has support to cleanup inactive stations
// in AP mode.
//
    ATH6KL_FW_CAPABILITY_INACTIVITY_TIMEOUT,

// Firmware has support to override rsn cap of rsn ie
    ATH6KL_FW_CAPABILITY_RSN_CAP_OVERRIDE,

//
// Multicast support in WOW and host awake mode.
// Allow all multicast in host awake mode.
// Apply multicast filter in WOW mode.
//
    ATH6KL_FW_CAPABILITY_WOW_MULTICAST_FILTER,

// Firmware supports enhanced bmiss detection
    ATH6KL_FW_CAPABILITY_BMISS_ENHANCE,

//
// FW supports matching of ssid in schedule scan
//
    ATH6KL_FW_CAPABILITY_SCHED_SCAN_MATCH_LIST,

// Firmware supports filtering BSS results by RSSI
    ATH6KL_FW_CAPABILITY_RSSI_SCAN_THOLD,

// FW sets mac_addr[4] ^= 0x80 for newly created interfaces
    ATH6KL_FW_CAPABILITY_CUSTOM_MAC_ADDR,

// Firmware supports TX error rate notification
    ATH6KL_FW_CAPABILITY_TX_ERR_NOTIFY,

// supports WMI_SET_REGDOMAIN_CMDID command
    ATH6KL_FW_CAPABILITY_REGDOMAIN,

// Firmware supports sched scan decoupled from host sleep
    ATH6KL_FW_CAPABILITY_SCHED_SCAN_V2,

//
// Firmware capability for hang detection through heart beat
// challenge messages.
//
    ATH6KL_FW_CAPABILITY_HEART_BEAT_POLL,

// WMI_SET_TX_SELECT_RATES_CMDID uses 64 bit size rate table
    ATH6KL_FW_CAPABILITY_64BIT_RATES,

// WMI_AP_CONN_INACT_CMDID uses minutes as units
    ATH6KL_FW_CAPABILITY_AP_INACTIVITY_MINS,

// use low priority endpoint for all data
    ATH6KL_FW_CAPABILITY_MAP_LP_ENDPOINT,

// ratetable is the 2 stream version (max MCS15)
    ATH6KL_FW_CAPABILITY_RATETABLE_MCS15,

// firmware doesn't support IP checksumming
    ATH6KL_FW_CAPABILITY_NO_IP_CHECKSUM,

// this needs to be last
    ATH6KL_FW_CAPABILITY_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_fw_ie {
    pub id: __le32,
    pub len: __le32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_hw_flags {
    ATH6KL_HW_SDIO_CRC_ERROR_WAR	= BIT(3),
}

// AR6003 1.0 definitions
pub const AR6003_HW_1_0_VERSION: c_uint = 0x300002ba;
// AR6003 2.0 definitions
pub const AR6003_HW_2_0_VERSION: c_uint = 0x30000384;
pub const AR6003_HW_2_0_PATCH_DOWNLOAD_ADDRESS: c_uint = 0x57e910;

// AR6003 3.0 definitions
pub const AR6003_HW_2_1_1_VERSION: c_uint = 0x30000582;

// AR6004 1.0 definitions
pub const AR6004_HW_1_0_VERSION: c_uint = 0x30000623;

// AR6004 1.1 definitions
pub const AR6004_HW_1_1_VERSION: c_uint = 0x30000001;

// AR6004 1.2 definitions
pub const AR6004_HW_1_2_VERSION: c_uint = 0x300007e8;

// AR6004 1.3 definitions
pub const AR6004_HW_1_3_VERSION: c_uint = 0x31c8088a;

// AR6004 3.0 definitions
pub const AR6004_HW_3_0_VERSION: c_uint = 0x31C809F8;

// Per STA data, used in AP mode

// HTC TX packet tagging definitions

pub const AR6003_CUST_DATA_SIZE: c_int = 16;

pub const ATH6KL_MAX_SEQ_NO: c_uint = 0xFFF;

pub const NUM_OF_TIDS: c_int = 8;
pub const AGGR_SZ_DEFAULT: c_int = 8;
pub const AGGR_WIN_SZ_MIN: c_int = 2;
pub const AGGR_WIN_SZ_MAX: c_int = 8;

pub const AGGR_NUM_OF_FREE_NETBUFS: c_int = 16;

pub const MBOX_YIELD_LIMIT: c_int = 99;

pub const ATH6KL_DEFAULT_BMISS_TIME: c_int = 1500;

pub const ATH6KL_MAX_BMISS_TIME: c_int = 5000;
// configuration lags
//
// ATH6KL_CONF_IGNORE_ERP_BARKER: Ignore the barker premable in
// ERP IE of beacon to determine the short premable support when
// sending (Re)Assoc req.
// ATH6KL_CONF_IGNORE_PS_FAIL_EVT_IN_SCAN: Don't send the power
// module state transition failure events which happen during
// scan, to the host.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlan_low_pwr_state {
    WLAN_POWER_STATE_ON,
    WLAN_POWER_STATE_CUT_PWR,
    WLAN_POWER_STATE_DEEP_SLEEP,
    WLAN_POWER_STATE_WOW
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sme_state {
    SME_DISCONNECTED,
    SME_CONNECTING,
    SME_CONNECTED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_hold_q {
    pub skb: *mut sk_buff,
    pub is_amsdu: bool,
    pub seq_no: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxtid {
    pub aggr: bool,
    pub timer_mon: bool,
    pub win_sz: u16,
    pub seq_next: u16,
    pub hold_q_sz: u32,
    pub hold_q: *mut skb_hold_q,
    pub q: sk_buff_head,
//
// lock mainly protects seq_next and hold_q. Movement of seq_next
// needs to be protected between aggr_timeout() and
// aggr_process_recv_frm(). hold_q will be holding the pending
// reorder frames and it's access should also be protected.
// Some of the other fields like hold_q_sz, win_sz and aggr are
// initialized/reset when receiving addba/delba req, also while
// deleting aggr state all the pending buffers are flushed before
// resetting these fields, so there should not be any race in accessing
// these fields.
//
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxtid_stats {
    pub num_into_aggr: u32,
    pub num_dups: u32,
    pub num_oow: u32,
    pub num_mpdu: u32,
    pub num_amsdu: u32,
    pub num_delivered: u32,
    pub num_timeouts: u32,
    pub num_hole: u32,
    pub num_bar: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_info_conn {
    pub aggr_sz: u8,
    pub timer_scheduled: u8,
    pub timer: timer_list,
    pub dev: *mut net_device,
    pub rx_tid: [rxtid; NUM_OF_TIDS],
    pub stat: [rxtid_stats; NUM_OF_TIDS],
    pub aggr_info: *mut aggr_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_info {
    pub aggr_conn: *mut aggr_info_conn,
    pub rx_amsdu_freeq: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_wep_key {
    pub key_index: u8,
    pub key_len: u8,
    pub key: [u8; 64],
}

pub const ATH6KL_KEY_SEQ_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_key {
    pub key: [u8; WLAN_MAX_KEY_LEN],
    pub key_len: u8,
    pub seq: [u8; ATH6KL_KEY_SEQ_LEN],
    pub seq_len: u8,
    pub cipher: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_node_mapping {
    pub mac_addr: [u8; ETH_ALEN],
    pub ep_id: u8,
    pub tx_pend: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_cookie {
    pub skb: *mut sk_buff,
    pub map_no: u32,
    pub htc_pkt: htc_packet,
    pub arc_list_next: *mut ath6kl_cookie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_mgmt_buff {
    pub list: list_head,
    pub freq: u32,
    pub wait: u32,
    pub id: u32,
    pub cookie: u64,
    pub no_cck: bool,
    pub len: usize,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_sta {
    pub sta_flags: u16,
    pub mac: [u8; ETH_ALEN],
    pub aid: u8,
    pub keymgmt: u8,
    pub ucipher: u8,
    pub auth: u8,
    pub wpa_ie: [u8; ATH6KL_MAX_IE],
    pub psq: sk_buff_head,
// protects psq, mgmt_psq, apsdq, and mgmt_psq_len fields
    pub psq_lock: spinlock_t,
    pub mgmt_psq: list_head,
    pub mgmt_psq_len: usize,
    pub apsd_info: u8,
    pub apsdq: sk_buff_head,
    pub aggr_conn: *mut aggr_info_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_version {
    pub target_ver: u32,
    pub wlan_ver: u32,
    pub abi_ver: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_bmi {
    pub cmd_credits: u32,
    pub done_sent: bool,
    pub cmd_buf: *mut u8,
    pub max_data_size: u32,
    pub max_cmd_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_stats {
    pub tx_pkt: u64,
    pub tx_byte: u64,
    pub tx_ucast_pkt: u64,
    pub tx_ucast_byte: u64,
    pub tx_mcast_pkt: u64,
    pub tx_mcast_byte: u64,
    pub tx_bcast_pkt: u64,
    pub tx_bcast_byte: u64,
    pub tx_rts_success_cnt: u64,
    pub tx_pkt_per_ac: [u64; 4],
    pub tx_err: u64,
    pub tx_fail_cnt: u64,
    pub tx_retry_cnt: u64,
    pub tx_mult_retry_cnt: u64,
    pub tx_rts_fail_cnt: u64,
    pub rx_pkt: u64,
    pub rx_byte: u64,
    pub rx_ucast_pkt: u64,
    pub rx_ucast_byte: u64,
    pub rx_mcast_pkt: u64,
    pub rx_mcast_byte: u64,
    pub rx_bcast_pkt: u64,
    pub rx_bcast_byte: u64,
    pub rx_frgment_pkt: u64,
    pub rx_err: u64,
    pub rx_crc_err: u64,
    pub rx_key_cache_miss: u64,
    pub rx_decrypt_err: u64,
    pub rx_dupl_frame: u64,
    pub tkip_local_mic_fail: u64,
    pub tkip_cnter_measures_invoked: u64,
    pub tkip_replays: u64,
    pub tkip_fmt_err: u64,
    pub ccmp_fmt_err: u64,
    pub ccmp_replays: u64,
    pub pwr_save_fail_cnt: u64,
    pub cs_bmiss_cnt: u64,
    pub cs_low_rssi_cnt: u64,
    pub cs_connect_cnt: u64,
    pub cs_discon_cnt: u64,
    pub tx_ucast_rate: i32,
    pub rx_ucast_rate: i32,
    pub lq_val: u32,
    pub wow_pkt_dropped: u32,
    pub wow_evt_discarded: u16,
    pub noise_floor_calib: i16,
    pub cs_rssi: i16,
    pub cs_ave_beacon_rssi: i16,
    pub cs_ave_beacon_snr: u8,
    pub cs_last_roam_msec: u8,
    pub cs_snr: u8,
    pub wow_host_pkt_wakeups: u8,
    pub wow_host_evt_wakeups: u8,
    pub arp_received: u32,
    pub arp_matched: u32,
    pub arp_replied: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_mbox_info {
    pub htc_addr: u32,
    pub htc_ext_addr: u32,
    pub htc_ext_sz: u32,
    pub block_size: u32,
    pub gmbox_addr: u32,
    pub gmbox_sz: u32,
}

//
// 802.11i defines an extended IV for use with non-WEP ciphers.
// When the EXTIV bit is set in the key id byte an additional
// 4 bytes immediately follow the IV for TKIP.  For CCMP the
// EXTIV bit is likewise set but the 8 bytes represent the
// CCMP header rather than IV+extended-IV.
//
pub const ATH6KL_KEYBUF_SIZE: c_int = 16;

pub const ATH6KL_KEY_XMIT: c_uint = 0x01;
pub const ATH6KL_KEY_RECV: c_uint = 0x02;
pub const ATH6KL_KEY_DEFAULT: c_uint = 0x80	/* default xmit key */;
// Initial group key for AP mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_req_key {
    pub valid: bool,
    pub key_index: u8,
    pub key_type: c_int,
    pub key: [u8; WLAN_MAX_KEY_LEN],
    pub key_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_hif_type {
    ATH6KL_HIF_TYPE_SDIO,
    ATH6KL_HIF_TYPE_USB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_htc_type {
    ATH6KL_HTC_TYPE_MBOX,
    ATH6KL_HTC_TYPE_PIPE,
}

// Max number of filters that hw supports
pub const ATH6K_MAX_MC_FILTERS_PER_LIST: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_mc_filter {
    pub list: list_head,
    pub hw_addr: [c_char; ATH6KL_MCAST_FILTER_MAC_ADDR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_htcap {
    pub ht_enable: bool,
    pub ampdu_factor: u8,
    pub cap_info: c_ushort,
}

//
// Driver's maximum limit, note that some firmwares support only one vif
// and the runtime (current) limit must be checked from ar->vif_max.
//
pub const ATH6KL_VIF_MAX: c_int = 3;
// vif flags info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_vif_state {
    CONNECTED,
    CONNECT_PEND,
    WMM_ENABLED,
    NETQ_STOPPED,
    DTIM_EXPIRED,
    CLEAR_BSSFILTER_ON_BEACON,
    DTIM_PERIOD_AVAIL,
    WLAN_ENABLED,
    STATS_UPDATE_PEND,
    HOST_SLEEP_MODE_CMD_PROCESSED,
    NETDEV_MCAST_ALL_ON,
    NETDEV_MCAST_ALL_OFF,
    SCHED_SCANNING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_vif {
    pub list: list_head,
    pub wdev: wireless_dev,
    pub ndev: *mut net_device,
    pub ar: *mut ath6kl,
// Lock to protect vif specific net_stats and flags
    pub if_lock: spinlock_t,
    pub fw_vif_idx: u8,
    pub flags: c_ulong,
    pub ssid_len: c_int,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub dot11_auth_mode: u8,
    pub auth_mode: u8,
    pub prwise_crypto: u8,
    pub prwise_crypto_len: u8,
    pub grp_crypto: u8,
    pub grp_crypto_len: u8,
    pub def_txkey_index: u8,
    pub next_mode: u8,
    pub nw_type: u8,
    pub bssid: [u8; ETH_ALEN],
    pub req_bssid: [u8; ETH_ALEN],
    pub ch_hint: u16,
    pub bss_ch: u16,
    pub 1]: ath6kl_wep_key wep_key_list[WMI_MAX_KEY_INDEX +,
    pub 1]: ath6kl_key keys[WMI_MAX_KEY_INDEX +,
    pub aggr_cntxt: *mut aggr_info,
    pub htcap: [ath6kl_htcap; NUM_NL80211_BANDS],
    pub disconnect_timer: timer_list,
    pub sched_scan_timer: timer_list,
    pub scan_req: *mut cfg80211_scan_request,
    pub sme_state: sme_state,
    pub reconnect_flag: c_int,
    pub last_roc_id: u64,
    pub last_cancel_roc_id: u64,
    pub send_action_id: u32,
    pub probe_req_report: bool,
    pub assoc_bss_beacon_int: u16,
    pub listen_intvl_t: u16,
    pub bmiss_time_t: u16,
    pub txe_intvl: u32,
    pub bg_scan_period: u16,
    pub assoc_bss_dtim_period: u8,
    pub target_stats: target_stats,
    pub profile: wmi_connect_cmd,
    pub rsn_capab: u16,
    pub mc_filter: list_head,
}

extern "C" {
    pub fn container_of(_arg: wdev, ath6kl_vif: struct, _arg: wdev) -> return;
}
pub const WOW_LIST_ID: c_int = 0;

// Flag info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_dev_state {
    WMI_ENABLED,
    WMI_READY,
    WMI_CTRL_EP_FULL,
    TESTMODE,
    DESTROY_IN_PROGRESS,
    SKIP_SCAN,
    ROAM_TBL_PEND,
    FIRST_BOOT,
    RECOVERY_CLEANUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_state {
    ATH6KL_STATE_OFF,
    ATH6KL_STATE_ON,
    ATH6KL_STATE_SUSPENDING,
    ATH6KL_STATE_RESUMING,
    ATH6KL_STATE_DEEPSLEEP,
    ATH6KL_STATE_CUTPOWER,
    ATH6KL_STATE_WOW,
    ATH6KL_STATE_RECOVERY,
}

// Fw error recovery
pub const ATH6KL_HB_RESP_MISS_THRES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_fw_err {
    ATH6KL_FW_ASSERT,
    ATH6KL_FW_HB_RESP_FAILURE,
    ATH6KL_FW_EP_FULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl {
    pub dev: *mut device,
    pub wiphy: *mut wiphy,
    pub state: ath6kl_state,
    pub testmode: c_uint,
    pub bmi: ath6kl_bmi,
    pub hif_ops: *const ath6kl_hif_ops,
    pub htc_ops: *const ath6kl_htc_ops,
    pub wmi: *mut wmi,
    pub tx_pending: [c_int; ENDPOINT_MAX],
    pub total_tx_data_pend: c_int,
    pub htc_target: *mut htc_target,
    pub hif_type: ath6kl_hif_type,
    pub hif_priv: *mut c_void,
    pub vif_list: list_head,
// Lock to avoid race in vif_list entries among add/del/traverse
    pub list_lock: spinlock_t,
    pub num_vif: u8,
    pub vif_max: c_uint,
    pub max_norm_iface: u8,
    pub avail_idx_map: u8,
//
// Protects at least amsdu_rx_buffer_queue, ath6kl_alloc_cookie()
// calls, tx_pending and total_tx_data_pend.
//
    pub lock: spinlock_t,
    pub sem: semaphore,
    pub lrssi_roam_threshold: u8,
    pub version: ath6kl_version,
    pub target_type: u32,
    pub tx_pwr: u8,
    pub node_map: [ath6kl_node_mapping; MAX_NODE_NUM],
    pub ibss_ps_enable: u8,
    pub ibss_if_active: bool,
    pub node_num: u8,
    pub next_ep_id: u8,
    pub cookie_list: *mut ath6kl_cookie,
    pub cookie_count: u32,
    pub ac2ep_map: [htc_endpoint_id; WMM_NUM_AC],
    pub ac_stream_active: [bool; WMM_NUM_AC],
    pub ac_stream_pri_map: [u8; WMM_NUM_AC],
    pub hiac_stream_active_pri: u8,
    pub ep2ac_map: [u8; ENDPOINT_MAX],
    pub ctrl_ep: htc_endpoint_id,
    pub credit_state_info: ath6kl_htc_credit_info,
    pub connect_ctrl_flags: u32,
    pub user_key_ctrl: u32,
    pub usr_bss_filter: u8,
    pub sta_list: [ath6kl_sta; AP_MAX_NUM_STA],
    pub sta_list_index: u8,
    pub ap_mode_bkey: ath6kl_req_key,
    pub mcastpsq: sk_buff_head,
    pub want_ch_switch: u32,
    pub last_ch: u16,
//
// FIXME: protects access to mcastpsq but is actually useless as
// all skbe_queue_*() functions provide serialisation themselves
//
    pub mcastpsq_lock: spinlock_t,
    pub intra_bss: u8,
    pub ap_stats: wmi_ap_mode_stat,
    pub ap_country_code: [u8; 3],
    pub amsdu_rx_buffer_queue: list_head,
    pub rx_meta_ver: u8,
    pub wlan_pwr_state: wlan_low_pwr_state,
    pub mac_addr: [u8; ETH_ALEN],
pub const AR_MCAST_FILTER_MAC_ADDR_SIZE: c_int = 4;
    pub rx_report: *mut c_void,
    pub rx_report_len: usize,
    pub tm: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_hw {
    pub id: u32,
    pub name: *const c_char,
    pub dataset_patch_addr: u32,
    pub app_load_addr: u32,
    pub app_start_override_addr: u32,
    pub board_ext_data_addr: u32,
    pub reserved_ram_size: u32,
    pub board_addr: u32,
    pub refclk_hz: u32,
    pub uarttx_pin: u32,
    pub uarttx_rate: u32,
    pub testscript_addr: u32,
    pub tx_ant: u8,
    pub rx_ant: u8,
    pub cap: wmi_phy_cap,
    pub flags: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_hw_fw {
    pub dir: *const c_char,
    pub otp: *const c_char,
    pub fw: *const c_char,
    pub tcmd: *const c_char,
    pub patch: *const c_char,
    pub utf: *const c_char,
    pub testscript: *const c_char,
    pub fw: },
    pub fw_board: *const c_char,
    pub fw_default_board: *const c_char,
    pub hw: },
    pub conf_flags: u16,
    pub suspend_mode: u16,
    pub wow_suspend_mode: u16,
    pub event_wq: wait_queue_head_t,
    pub mbox_info: ath6kl_mbox_info,
    pub cookie_mem: [ath6kl_cookie; MAX_COOKIE_NUM],
    pub flag: c_ulong,
    pub fw_board: *mut u8,
    pub fw_board_len: usize,
    pub fw_otp: *mut u8,
    pub fw_otp_len: usize,
    pub fw: *mut u8,
    pub fw_len: usize,
    pub fw_patch: *mut u8,
    pub fw_patch_len: usize,
    pub fw_testscript: *mut u8,
    pub fw_testscript_len: usize,
    pub fw_api: c_uint,
    pub fw_capabilities: [c_ulong; ATH6KL_CAPABILITY_LEN],
    pub ath6kl_wq: *mut workqueue_struct,
    pub debugfs_phy: *mut dentry,
    pub p2p: bool,
    pub wiphy_registered: bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_fw_recovery {
    pub recovery_work: work_struct,
    pub err_reason: c_ulong,
    pub hb_poll: c_ulong,
    pub hb_timer: timer_list,
    pub seq_num: u32,
    pub hb_pending: bool,
    pub hb_misscnt: u8,
    pub enable: bool,
    pub fw_recovery: },

    pub fwlog_queue: sk_buff_head,
    pub fwlog_completion: completion,
    pub fwlog_open: bool,
    pub fwlog_mask: u32,
    pub dbgfs_diag_reg: c_uint,
    pub diag_reg_addr_wr: u32,
    pub diag_reg_val_wr: u32,
    pub invalid_rate: c_uint,
    pub war_stats: },
    pub roam_tbl: *mut u8,
    pub roam_tbl_len: c_uint,
    pub keepalive: u8,
    pub disc_timeout: u8,
    pub debug: },

}

extern "C" {
    pub fn ath6kl_configure_target(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_detect_error(ptr: c_ulong);
}
extern "C" {
    pub fn disconnect_timer_handler(t: *mut timer_list);
}
extern "C" {
    pub fn init_netdev(dev: *mut net_device);
}
extern "C" {
    pub fn ath6kl_cookie_init(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_cookie_cleanup(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_rx(target: *mut htc_target, packet: *mut htc_packet);
}
extern "C" {
    pub fn ath6kl_stop_txrx(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_cleanup_amsdu_rxbufs(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_diag_write32(ar: *mut ath6kl, address: u32, value: __le32) -> c_int;
}
extern "C" {
    pub fn ath6kl_diag_write(ar: *mut ath6kl, address: u32, data: *mut c_void, length: u32) -> c_int;
}
extern "C" {
    pub fn ath6kl_diag_read32(ar: *mut ath6kl, address: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn ath6kl_diag_read(ar: *mut ath6kl, address: u32, data: *mut c_void, length: u32) -> c_int;
}
extern "C" {
    pub fn ath6kl_read_fwlogs(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_init_profile_info(vif: *mut ath6kl_vif);
}
extern "C" {
    pub fn ath6kl_tx_data_cleanup(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_free_cookie(ar: *mut ath6kl, cookie: *mut ath6kl_cookie);
}
extern "C" {
    pub fn ath6kl_data_tx(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn ath6kl_refill_amsdu_rxbufs(ar: *mut ath6kl, count: c_int);
}
extern "C" {
    pub fn aggr_module_destroy(aggr_info: *mut aggr_info);
}
extern "C" {
    pub fn aggr_reset_state(aggr_conn: *mut aggr_info_conn);
}
extern "C" {
    pub fn ath6kl_connect_ap_mode_bss(vif: *mut ath6kl_vif, channel: u16);
}
extern "C" {
    pub fn ath6kl_tkip_micerr_event(vif: *mut ath6kl_vif, keyid: u8, ismcast: bool);
}
extern "C" {
    pub fn ath6kl_txpwr_rx_evt(devt: *mut c_void, tx_pwr: u8);
}
extern "C" {
    pub fn ath6kl_scan_complete_evt(vif: *mut ath6kl_vif, status: c_int);
}
extern "C" {
    pub fn ath6kl_tgt_stats_event(vif: *mut ath6kl_vif, ptr: *mut u8, len: u32);
}
extern "C" {
    pub fn ath6kl_indicate_tx_activity(devt: *mut c_void, traffic_class: u8, active: bool);
}
extern "C" {
    pub fn ath6kl_ac2_endpoint_id(devt: *mut c_void, ac: u8) -> htc_endpoint_id;
}
extern "C" {
    pub fn ath6kl_pspoll_event(vif: *mut ath6kl_vif, aid: u8);
}
extern "C" {
    pub fn ath6kl_dtimexpiry_event(vif: *mut ath6kl_vif);
}
extern "C" {
    pub fn ath6kl_disconnect(vif: *mut ath6kl_vif);
}
extern "C" {
    pub fn aggr_recv_delba_req_evt(vif: *mut ath6kl_vif, tid: u8);
}
extern "C" {
    pub fn ath6kl_wakeup_event(dev: *mut c_void);
}
extern "C" {
    pub fn ath6kl_init_control_info(vif: *mut ath6kl_vif);
}
extern "C" {
    pub fn ath6kl_cfg80211_vif_stop(vif: *mut ath6kl_vif, wmi_ready: bool);
}
extern "C" {
    pub fn ath6kl_init_hw_start(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_init_hw_stop(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_init_fetch_firmwares(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_init_hw_params(ar: *mut ath6kl) -> c_int;
}
extern "C" {
    pub fn ath6kl_check_wow_status(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_core_tx_complete(ar: *mut ath6kl, skb: *mut sk_buff);
}
extern "C" {
    pub fn ath6kl_core_rx_complete(ar: *mut ath6kl, skb: *mut sk_buff, pipe: u8);
}
extern "C" {
    pub fn ath6kl_core_init(ar: *mut ath6kl, htc_type: ath6kl_htc_type) -> c_int;
}
extern "C" {
    pub fn ath6kl_core_cleanup(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_core_destroy(ar: *mut ath6kl);
}
// Fw error recovery
extern "C" {
    pub fn ath6kl_init_hw_restart(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_recovery_err_notify(ar: *mut ath6kl, reason: ath6kl_fw_err);
}
extern "C" {
    pub fn ath6kl_recovery_hb_event(ar: *mut ath6kl, cookie: u32);
}
extern "C" {
    pub fn ath6kl_recovery_init(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_recovery_cleanup(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_recovery_suspend(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_recovery_resume(ar: *mut ath6kl);
}
