//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/htc.h
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

pub const ATH_DEFAULT_BMISS_LIMIT: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_phymode {
    HTC_MODE_11NA		= 0,
    HTC_MODE_11NG		= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_opmode {
    HTC_M_STA	= 1,
    HTC_M_IBSS	= 0,
    HTC_M_AHDEMO	= 3,
    HTC_M_HOSTAP	= 6,
    HTC_M_MONITOR	= 8,
    HTC_M_WDS	= 2
}

pub const ATH9K_HTC_AMPDU: c_int = 1;
pub const ATH9K_HTC_NORMAL: c_int = 2;
pub const ATH9K_HTC_BEACON: c_int = 3;
pub const ATH9K_HTC_MGMT: c_int = 4;
pub const ATH9K_HTC_TX_CTSONLY: c_uint = 0x1;
pub const ATH9K_HTC_TX_RTSCTS: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_frame_hdr {
    pub data_type: u8,
    pub node_idx: u8,
    pub vif_idx: u8,
    pub tidno: u8,
    pub /: *mut *mut *mut __be32 flags; / ATH9K_HTC_TX_,
    pub key_type: u8,
    pub keyix: u8,
    pub cookie: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_mgmt_hdr {
    pub node_idx: u8,
    pub vif_idx: u8,
    pub tidno: u8,
    pub flags: u8,
    pub key_type: u8,
    pub keyix: u8,
    pub cookie: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_beacon_header {
    pub vif_index: u8,
    pub len_changed: u8,
    pub rev: u16,
    pub __packed: },
pub const MAX_TX_AMPDU_SUBFRAMES_9271: c_int = 17;
pub const MAX_TX_AMPDU_SUBFRAMES_7010: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_cap_target {
    pub ampdu_limit: __be32,
    pub ampdu_subframes: u8,
    pub enable_coex: u8,
    pub tx_chainmask: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_vif {
    pub index: u8,
    pub opmode: u8,
    pub myaddr: [u8; ETH_ALEN],
    pub ath_cap: u8,
    pub rtsthreshold: __be16,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_sta {
    pub macaddr: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub sta_index: u8,
    pub vif_index: u8,
    pub is_vif_sta: u8,
    pub flags: __be16,
    pub htcap: __be16,
    pub maxampdu: __be16,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_aggr {
    pub sta_index: u8,
    pub tidno: u8,
    pub aggr_enable: u8,
    pub padding: u8,
    pub __packed: },
pub const ATH_HTC_RATE_MAX: c_int = 30;
pub const WLAN_RC_DS_FLAG: c_uint = 0x01;
pub const WLAN_RC_40_FLAG: c_uint = 0x02;
pub const WLAN_RC_SGI_FLAG: c_uint = 0x04;
pub const WLAN_RC_HT_FLAG: c_uint = 0x08;
pub const ATH_RC_TX_STBC_FLAG: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_rateset {
    pub rs_nrates: u8,
    pub rs_rates: [u8; ATH_HTC_RATE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_rate {
    pub legacy_rates: ath9k_htc_rateset,
    pub ht_rates: ath9k_htc_rateset,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_rate {
    pub sta_index: u8,
    pub isnew: u8,
    pub capflags: __be32,
    pub rates: ath9k_htc_rate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_rate_mask {
    pub vif_index: u8,
    pub band: u8,
    pub mask: __be32,
    pub pad: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_int_stats {
    pub rx: __be32,
    pub rxorn: __be32,
    pub rxeol: __be32,
    pub txurn: __be32,
    pub txto: __be32,
    pub cst: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_tx_stats {
    pub xretries: __be32,
    pub fifoerr: __be32,
    pub filtered: __be32,
    pub timer_exp: __be32,
    pub shortretries: __be32,
    pub longretries: __be32,
    pub qnull: __be32,
    pub encap_fail: __be32,
    pub nobuf: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_target_rx_stats {
    pub nobuf: __be32,
    pub host_send: __be32,
    pub host_done: __be32,
    pub __packed: },
pub const ATH9K_HTC_MAX_VIF: c_int = 2;
pub const ATH9K_HTC_MAX_BCN_VIF: c_int = 2;

    pub \: _priv->num_sta_vif++;,
    pub \: break;,
    pub \: _priv->num_ibss_vif++;,
    pub \: break;,
    pub \: _priv->num_ap_vif++;,
    pub \: break;,
    pub \: _priv->num_mbss_vif++;,
    pub \: break;,
    pub \: break;,

    pub \: _priv->num_sta_vif--;,
    pub \: break;,
    pub \: _priv->num_ibss_vif--;,
    pub \: break;,
    pub \: _priv->num_ap_vif--;,
    pub \: break;,
    pub \: _priv->num_mbss_vif--;,
    pub \: break;,
    pub \: break;,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_vif {
    pub index: u8,
    pub seq_no: u16,
    pub beacon_configured: bool,
    pub bslot: c_int,
    pub tsfadjust: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_vif_iter_data {
    pub hw_macaddr: *const u8,
    pub mask: [u8; ETH_ALEN],
}

pub const ATH9K_HTC_MAX_STA: c_int = 8;
pub const ATH9K_HTC_MAX_TID: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tid_aggr_state {
    AGGR_STOP = 0,
    AGGR_PROGRESS,
    AGGR_START,
    AGGR_OPERATIONAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_sta {
    pub index: u8,
    pub tid_state: [tid_aggr_state; ATH9K_HTC_MAX_TID],
    pub rc_update_work: work_struct,
    pub htc_priv: *mut ath9k_htc_priv,
}

pub const ATH9K_HTC_RXBUF: c_int = 256;
pub const HTC_RX_FRAME_HEADER_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_rxbuf {
    pub in_process: bool,
    pub skb: *mut sk_buff,
    pub rxstatus: ath_htc_rx_status,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_rx {
    pub rxbuf: list_head,
    pub rxbuflock: spinlock_t,
    pub initialized: bool,
}

pub const ATH9K_HTC_TX_RESERVE: c_int = 10;
pub const ATH9K_HTC_TX_TIMEOUT_COUNT: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_tx {
    pub flags: u8,
    pub queued_cnt: c_int,
    pub mgmt_ep_queue: sk_buff_head,
    pub cab_ep_queue: sk_buff_head,
    pub data_be_queue: sk_buff_head,
    pub data_bk_queue: sk_buff_head,
    pub data_vi_queue: sk_buff_head,
    pub data_vo_queue: sk_buff_head,
    pub tx_failed: sk_buff_head,
    pub MAX_TX_BUF_NUM): DECLARE_BITMAP(tx_slot,,
    pub cleanup_timer: timer_list,
    pub tx_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_tx_ctl {
    pub /: *mut *mut *mut u8 type; / ATH9K_HTC_,
    pub epid: u8,
    pub txok: u8,
    pub sta_idx: u8,
    pub timestamp: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_tx_stats {
    pub buf_queued: u32,
    pub buf_completed: u32,
    pub skb_queued: u32,
    pub skb_success: u32,
    pub skb_success_bytes: u32,
    pub skb_failed: u32,
    pub cab_queued: u32,
    pub queue_stats: [u32; IEEE80211_NUM_ACS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_skbrx_stats {
    pub skb_allocated: u32,
    pub skb_completed: u32,
    pub skb_completed_bytes: u32,
    pub skb_dropped: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_debug {
    pub debugfs_phy: *mut dentry,
    pub tx_stats: ath_tx_stats,
    pub rx_stats: ath_rx_stats,
    pub skbrx_stats: ath_skbrx_stats,
}

// Macro flag: #define CAB_STAT_INC(priv)

pub const ATH_LED_PIN_DEF: c_int = 1;
pub const ATH_LED_PIN_9287: c_int = 10;
pub const ATH_LED_PIN_9271: c_int = 15;
pub const ATH_LED_PIN_7010: c_int = 12;
pub const BSTUCK_THRESHOLD: c_int = 10;
//
// Adjust these when the max. no of beaconing interfaces is
// increased.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_beacon {
    pub /: *mut *mut } updateslot; / slot time update fsm,
    pub bslot: [*mut ieee80211_vif; ATH9K_HTC_MAX_BCN_VIF],
    pub bmisscnt: u32,
    pub beaconq: u32,
    pub slottime: c_int,
    pub slotupdate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_btcoex {
    pub bt_priority_cnt: u32,
    pub bt_priority_time: c_ulong,
    pub /: *mut *mut int bt_stomp_type; / Types of BT stomping,
    pub btcoex_no_stomp: u32,
    pub btcoex_period: u32,
    pub btscan_no_stomp: u32,
}

extern "C" {
    pub fn ath9k_htc_init_btcoex(priv: *mut ath9k_htc_priv, product: *mut c_char);
}
extern "C" {
    pub fn ath9k_htc_start_btcoex(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_stop_btcoex(priv: *mut ath9k_htc_priv);
}

pub const OP_BT_PRIORITY_DETECTED: c_int = 3;
pub const OP_BT_SCAN: c_int = 4;
pub const OP_TSF_RESET: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_op_flags {
    HTC_FWFLAG_NO_RMW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_priv {
    pub dev: *mut device,
    pub hw: *mut ieee80211_hw,
    pub ah: *mut ath_hw,
    pub htc: *mut htc_target,
    pub wmi: *mut wmi,
    pub fw_version_major: u16,
    pub fw_version_minor: u16,
    pub wmi_cmd_ep: htc_endpoint_id,
    pub beacon_ep: htc_endpoint_id,
    pub cab_ep: htc_endpoint_id,
    pub uapsd_ep: htc_endpoint_id,
    pub mgmt_ep: htc_endpoint_id,
    pub data_be_ep: htc_endpoint_id,
    pub data_bk_ep: htc_endpoint_id,
    pub data_vi_ep: htc_endpoint_id,
    pub data_vo_ep: htc_endpoint_id,
    pub vif_slot: u8,
    pub mon_vif_idx: u8,
    pub sta_slot: u8,
    pub vif_sta_pos: [u8; ATH9K_HTC_MAX_VIF],
    pub num_ibss_vif: u8,
    pub num_mbss_vif: u8,
    pub num_sta_vif: u8,
    pub num_sta_assoc_vif: u8,
    pub num_ap_vif: u8,
    pub curtxpow: u16,
    pub txpowlimit: u16,
    pub nvifs: u16,
    pub nstations: u16,
    pub rearm_ani: bool,
    pub reconfig_beacon: bool,
    pub rxfilter: c_uint,
    pub op_flags: c_ulong,
    pub fw_flags: c_ulong,
    pub caldata: ath9k_hw_cal_data,
    pub spec_priv: ath_spec_scan_priv,
    pub beacon_lock: spinlock_t,
    pub cur_beacon_conf: ath_beacon_config,
    pub beacon: htc_beacon,
    pub rx: ath9k_htc_rx,
    pub tx: ath9k_htc_tx,
    pub swba_tasklet: tasklet_struct,
    pub rx_tasklet: tasklet_struct,
    pub ani_work: delayed_work,
    pub tx_failed_tasklet: tasklet_struct,
    pub ps_work: work_struct,
    pub fatal_work: work_struct,
    pub htc_pm_lock: mutex,
    pub ps_usecount: c_ulong,
    pub ps_enabled: bool,
    pub ps_idle: bool,
    pub initialized: bool,

    pub brightness: led_brightness,
    pub led_registered: bool,
    pub led_name: [c_char; 32],
    pub led_cdev: led_classdev,
    pub led_work: work_struct,

    pub cabq: c_int,
    pub hwq_map: [c_int; IEEE80211_NUM_ACS],
    pub btcoex: ath_btcoex,

    pub coex_period_work: delayed_work,
    pub duty_cycle_work: delayed_work,

    pub debug: ath9k_debug,

    pub mutex: mutex,
    pub csa_vif: *mut ieee80211_vif,
}

extern "C" {
    pub fn ath9k_htc_reset(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_beaconq_config(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_beacon_reconfig(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_ani_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath9k_htc_start_ani(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_stop_ani(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_tx_init(priv: *mut ath9k_htc_priv) -> c_int;
}
extern "C" {
    pub fn ath9k_tx_cleanup(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_txq_setup(priv: *mut ath9k_htc_priv, subtype: c_int) -> bool;
}
extern "C" {
    pub fn ath9k_htc_cabq_setup(priv: *mut ath9k_htc_priv) -> c_int;
}
extern "C" {
    pub fn get_hw_qnum(queue: u16, hwq_map: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ath9k_htc_check_stop_queues(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_check_wake_queues(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_tx_get_slot(priv: *mut ath9k_htc_priv) -> c_int;
}
extern "C" {
    pub fn ath9k_htc_tx_clear_slot(priv: *mut ath9k_htc_priv, slot: c_int);
}
extern "C" {
    pub fn ath9k_htc_tx_drain(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_txstatus(priv: *mut ath9k_htc_priv, wmi_event: *mut c_void);
}
extern "C" {
    pub fn ath9k_tx_failed_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ath9k_htc_tx_cleanup_timer(t: *mut timer_list);
}
extern "C" {
    pub fn ath9k_htc_csa_is_finished(priv: *mut ath9k_htc_priv) -> bool;
}
extern "C" {
    pub fn ath9k_rx_init(priv: *mut ath9k_htc_priv) -> c_int;
}
extern "C" {
    pub fn ath9k_rx_cleanup(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_host_rx_init(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_rx_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ath9k_htc_calcrxfilter(priv: *mut ath9k_htc_priv) -> u32;
}
extern "C" {
    pub fn ath9k_htc_ps_wakeup(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_ps_restore(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_ps_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath9k_start_rfkill_poll(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_htc_rfkill_poll_state(hw: *mut ieee80211_hw);
}

extern "C" {
    pub fn ath9k_configure_leds(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_init_leds(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_deinit_leds(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_led_work(work: *mut work_struct);
}

extern "C" {
    pub fn ath9k_htc_disconnect_device(htc_handle: *mut htc_target, hotunplug: bool);
}

extern "C" {
    pub fn ath9k_htc_suspend(htc_handle: *mut htc_target);
}
extern "C" {
    pub fn ath9k_htc_resume(htc_handle: *mut htc_target) -> c_int;
}

extern "C" {
    pub fn ath9k_htc_init_debug(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ath9k_htc_deinit_debug(priv: *mut ath9k_htc_priv);
}

