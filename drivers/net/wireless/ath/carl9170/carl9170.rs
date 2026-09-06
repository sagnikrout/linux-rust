//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/carl9170.h
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
// Atheros CARL9170 driver
//
// Driver specific definitions
//
// Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
// Copyright 2009, 2010, Christian Lamparter <chunkeey@googlemail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, see
// http://www.gnu.org/licenses/.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// Copyright (c) 2007-2008 Atheros Communications, Inc.
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

pub const CARL9170_MAX_RX_BUFFER_SIZE: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_device_state {
    CARL9170_UNKNOWN_STATE,
    CARL9170_STOPPED,
    CARL9170_IDLE,
    CARL9170_STARTED,
}

pub const WME_BA_BMP_SIZE: c_int = 64;
pub const CARL9170_TX_USER_RATE_TRIES: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_tid_state {
    CARL9170_TID_STATE_INVALID,
    CARL9170_TID_STATE_KILLED,
    CARL9170_TID_STATE_SHUTDOWN,
    CARL9170_TID_STATE_SUSPEND,
    CARL9170_TID_STATE_PROGRESS,
    CARL9170_TID_STATE_IDLE,
    CARL9170_TID_STATE_XMIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_sta_tid {
// must be the first entry!
    pub list: list_head,
// temporary list for RCU unlink procedure
    pub tmp_list: list_head,
// lock for the following data structures
    pub lock: spinlock_t,
    pub counter: c_uint,
    pub state: carl9170_tid_state,
    pub /: *mut *mut u8 tid; / TID number ( 0 - 15 ),
    pub /: *mut *mut u16 max; / max. AMPDU size,
    pub /: *mut *mut u16 snx; / awaiting _next_ frame,
    pub /: *mut *mut u16 hsn; / highest _queued_ sequence,
    pub /: *mut *mut u16 bsn; / base of the tx/agg bitmap,
    pub bitmap: [c_ulong; CARL9170_BAW_SIZE],
// Preaggregation reorder queue
    pub queue: sk_buff_head,
    pub sta: *mut ieee80211_sta,
    pub vif: *mut ieee80211_vif,
}

pub const CARL9170_QUEUE_TIMEOUT: c_int = 256;
pub const CARL9170_BUMP_QUEUE: c_int = 1000;
pub const CARL9170_TX_TIMEOUT: c_int = 2500;
pub const CARL9170_JANITOR_DELAY: c_int = 128;
pub const CARL9170_QUEUE_STUCK_TIMEOUT: c_int = 5500;
pub const CARL9170_STAT_WORK: c_int = 30000;
pub const CARL9170_NUM_TX_AGG_MAX: c_int = 30;
//
// Tradeoff between stability/latency and speed.
//
// AR9170_TXQ_DEPTH is devised by dividing the amount of available
// tx buffers with the size of a full ethernet frame + overhead.
//
// Naturally: The higher the limit, the faster the device CAN send.
// However, even a slight over-commitment at the wrong time and the
// hardware is doomed to send all already-queued frames at suboptimal
// rates. This in turn leads to an enormous amount of unsuccessful
// retries => Latency goes up, whereas the throughput goes down. CRASH!
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tx_queue_stats {
    pub count: c_uint,
    pub limit: c_uint,
    pub len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_vif {
    pub id: c_uint,
    pub vif: *mut ieee80211_vif __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_vif_info {
    pub list: list_head,
    pub active: bool,
    pub id: c_uint,
    pub beacon: *mut sk_buff,
    pub enable_beacon: bool,
}

pub const AR9170_NUM_RX_URBS: c_int = 16;
pub const AR9170_NUM_RX_URBS_MUL: c_int = 2;
pub const AR9170_NUM_TX_URBS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_device_features {
    CARL9170_WPS_BUTTON		= BIT(0),
    CARL9170_ONE_LED		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_led {
    pub ar: *mut ar9170,
    pub l: led_classdev,
    pub name: [c_char; 32],
    pub toggled: c_uint,
    pub last_state: bool,
    pub registered: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_restart_reasons {
    CARL9170_RR_NO_REASON = 0,
    CARL9170_RR_FATAL_FIRMWARE_ERROR,
    CARL9170_RR_TOO_MANY_FIRMWARE_ERRORS,
    CARL9170_RR_WATCHDOG,
    CARL9170_RR_STUCK_TX,
    CARL9170_RR_UNRESPONSIVE_DEVICE,
    CARL9170_RR_COMMAND_TIMEOUT,
    CARL9170_RR_TOO_MANY_PHY_ERRORS,
    CARL9170_RR_LOST_RSP,
    CARL9170_RR_INVALID_RSP,
    CARL9170_RR_USER_REQUEST,

    __CARL9170_RR_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_erp_modes {
    CARL9170_ERP_INVALID,
    CARL9170_ERP_AUTO,
    CARL9170_ERP_MAC80211,
    CARL9170_ERP_OFF,
    CARL9170_ERP_CTS,
    CARL9170_ERP_RTS,
    __CARL9170_ERP_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170 {
    pub common: ath_common,
    pub hw: *mut ieee80211_hw,
    pub mutex: mutex,
    pub state: carl9170_device_state,
    pub state_lock: spinlock_t,
    pub last_reason: carl9170_restart_reasons,
    pub registered: bool,
// USB
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub rx_anch: usb_anchor,
    pub rx_work: usb_anchor,
    pub rx_pool: usb_anchor,
    pub tx_wait: usb_anchor,
    pub tx_anch: usb_anchor,
    pub tx_cmd: usb_anchor,
    pub tx_err: usb_anchor,
    pub usb_tasklet: tasklet_struct,
    pub tx_cmd_urbs: core::sync::atomic::AtomicI32,
    pub tx_anch_urbs: core::sync::atomic::AtomicI32,
    pub rx_anch_urbs: core::sync::atomic::AtomicI32,
    pub rx_work_urbs: core::sync::atomic::AtomicI32,
    pub rx_pool_urbs: core::sync::atomic::AtomicI32,
    pub features: kernel_ulong_t,
    pub usb_ep_cmd_is_bulk: bool,
// firmware settings
    pub fw_load_wait: completion,
    pub fw_boot_wait: completion,
    pub desc: *const carl9170fw_desc_head,
    pub fw: *const firmware,
    pub offset: c_uint,
    pub address: c_uint,
    pub cmd_bufs: c_uint,
    pub api_version: c_uint,
    pub vif_num: c_uint,
    pub err_counter: c_uint,
    pub bug_counter: c_uint,
    pub beacon_addr: u32,
    pub beacon_max_len: c_uint,
    pub rx_stream: bool,
    pub tx_stream: bool,
    pub rx_filter: bool,
    pub hw_counters: bool,
    pub mem_blocks: c_uint,
    pub mem_block_size: c_uint,
    pub rx_size: c_uint,
    pub tx_seq_table: c_uint,
    pub ba_filter: bool,
    pub disable_offload_fw: bool,
    pub fw: },
// interface configuration combinations
    pub if_comb_limits: [ieee80211_iface_limit; 1],
    pub if_combs: [ieee80211_iface_combination; 1],
// reset / stuck frames/queue detection
    pub restart_work: work_struct,
    pub ping_work: work_struct,
    pub restart_counter: c_uint,
    pub queue_stop_timeout: [c_ulong; __AR9170_NUM_TXQ],
    pub max_queue_stop_timeout: [c_ulong; __AR9170_NUM_TXQ],
    pub needs_full_reset: bool,
    pub force_usb_reset: bool,
    pub pending_restarts: core::sync::atomic::AtomicI32,
// interface mode settings
    pub vif_list: list_head,
    pub vif_bitmap: c_ulong,
    pub vifs: c_uint,
    pub vif_priv: [carl9170_vif; AR9170_MAX_VIRTUAL_MAC],
// beaconing
    pub beacon_lock: spinlock_t,
    pub global_pretbtt: c_uint,
    pub global_beacon_int: c_uint,
    pub beacon_iter: *mut carl9170_vif_info __rcu,
    pub beacon_enabled: c_uint,
// cryptographic engine
    pub usedkeys: u64,
    pub rx_software_decryption: bool,
    pub disable_offload: bool,
// filter settings
    pub cur_mc_hash: u64,
    pub cur_filter: u32,
    pub filter_state: c_uint,
    pub rx_filter_caps: c_uint,
    pub sniffer_enabled: bool,
// MAC
    pub erp_mode: carl9170_erp_modes,
// PHY
    pub channel: *mut ieee80211_channel,
    pub num_channels: c_uint,
    pub noise: [c_int; 4],
    pub chan_fail: c_uint,
    pub total_chan_fail: c_uint,
    pub heavy_clip: u8,
    pub ht_settings: u8,
    pub /: *mut *mut u64 active; / usec,
    pub /: *mut *mut u64 cca; / usec,
    pub /: *mut *mut u64 tx_time; / usec,
    pub rx_total: u64,
    pub rx_overrun: u64,
    pub tally: },
    pub stat_work: delayed_work,
    pub survey: *mut survey_info,
// power calibration data
    pub power_5G_leg: [u8; 4],
    pub power_2G_cck: [u8; 4],
    pub power_2G_ofdm: [u8; 4],
    pub power_5G_ht20: [u8; 8],
    pub power_5G_ht40: [u8; 8],
    pub power_2G_ht20: [u8; 8],
    pub power_2G_ht40: [u8; 8],
// LED
    pub led_work: delayed_work,
    pub leds: [carl9170_led; AR9170_NUM_LEDS],
// qos queue settings
    pub tx_stats_lock: spinlock_t,
    pub tx_stats: [carl9170_tx_queue_stats; __AR9170_NUM_TXQ],
    pub edcf: [ieee80211_tx_queue_params; 5],
    pub tx_flush: completion,
// CMD
    pub cmd_seq: c_int,
    pub readlen: c_int,
    pub readbuf: *mut u8,
    pub cmd_lock: spinlock_t,
    pub cmd_wait: completion,
// statistics
    pub tx_dropped: c_uint,
    pub tx_ack_failures: c_uint,
    pub tx_fcs_errors: c_uint,
    pub rx_dropped: c_uint,
    pub rx_phy_errors: c_uint,
// EEPROM
    pub eeprom: ar9170_eeprom,
// tx queuing
    pub tx_pending: [sk_buff_head; __AR9170_NUM_TXQ],
    pub tx_status: [sk_buff_head; __AR9170_NUM_TXQ],
    pub tx_janitor: delayed_work,
    pub tx_janitor_last_run: c_ulong,
    pub tx_schedule: bool,
// tx ampdu
    pub ampdu_work: work_struct,
    pub tx_ampdu_list_lock: spinlock_t,
    pub tx_ampdu_iter: *mut carl9170_sta_tid __rcu,
    pub tx_ampdu_list: list_head,
    pub tx_ampdu_upload: core::sync::atomic::AtomicI32,
    pub tx_ampdu_scheduler: core::sync::atomic::AtomicI32,
    pub tx_total_pending: core::sync::atomic::AtomicI32,
    pub tx_total_queued: core::sync::atomic::AtomicI32,
    pub tx_ampdu_list_len: c_uint,
    pub current_density: c_int,
    pub current_factor: c_int,
    pub tx_ampdu_schedule: bool,
// internal memory management
    pub mem_lock: spinlock_t,
    pub mem_bitmap: *mut c_ulong,
    pub mem_free_blocks: core::sync::atomic::AtomicI32,
    pub mem_allocs: core::sync::atomic::AtomicI32,
// rxstream mpdu merge
    pub rx_plcp: ar9170_rx_head,
    pub rx_has_plcp: bool,
    pub rx_failover: *mut sk_buff,
    pub rx_failover_missing: c_int,
    pub ampdu_ref: u32,
// FIFO for collecting outstanding BlockAckRequest
    pub bar_list: [list_head; __AR9170_NUM_TXQ],
    pub bar_list_lock: [spinlock_t; __AR9170_NUM_TXQ],
    pub pbc_state: bool,
    pub pbc: *mut input_dev,
    pub name: [c_char; 32],
    pub phys: [c_char; 32],
    pub wps: },

    pub debug: carl9170_debug,
    pub debug_dir: *mut dentry,

// PSM
    pub ps_work: work_struct,
    pub dtim_counter: c_uint,
    pub last_beacon: c_ulong,
    pub last_action: c_ulong,
    pub last_slept: c_ulong,
    pub sleep_ms: c_uint,
    pub off_override: c_uint,
    pub state: bool,
    pub ps: },

    pub rng: hwrng,
    pub 1]: char name[30 +,
    pub sizeof(u16)]: u16 cache[CARL9170_HWRNG_CACHE_SIZE /,
    pub cache_idx: c_uint,
    pub rng: },

// Must be last as it ends in a flexible-array member.
    pub 1]: __le32 cmd_buf[PAYLOAD_MAX +,
    pub cmd: carl9170_cmd,
    pub rsp: carl9170_rsp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_ps_off_override_reasons {
    PS_OFF_VIF	= BIT(0),
    PS_OFF_BCN	= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_bar_list_entry {
    pub list: list_head,
    pub head: rcu_head,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_ba_stats {
    pub ampdu_len: u8,
    pub ampdu_ack_len: u8,
    pub clear: bool,
    pub req: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_sta_info {
    pub ht_sta: bool,
    pub sleeping: bool,
    pub pending_frames: core::sync::atomic::AtomicI32,
    pub ampdu_max_len: c_uint,
    pub agg: [*mut carl9170_sta_tid __rcu; IEEE80211_NUM_TIDS],
    pub stats: [carl9170_ba_stats; IEEE80211_NUM_TIDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tx_info {
    pub timeout: c_ulong,
    pub ar: *mut ar9170,
    pub ref: kref,
}

// exported interface
extern "C" {
    pub fn carl9170_register(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_unregister(ar: *mut ar9170);
}
extern "C" {
    pub fn carl9170_free(ar: *mut ar9170);
}
extern "C" {
    pub fn carl9170_restart(ar: *mut ar9170, r: carl9170_restart_reasons);
}
extern "C" {
    pub fn carl9170_ps_check(ar: *mut ar9170);
}
// USB back-end
extern "C" {
    pub fn carl9170_usb_open(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_usb_stop(ar: *mut ar9170);
}
extern "C" {
    pub fn carl9170_usb_tx(ar: *mut ar9170, skb: *mut sk_buff);
}
extern "C" {
    pub fn carl9170_usb_handle_tx_err(ar: *mut ar9170);
}
extern "C" {
    pub fn carl9170_usb_restart(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_usb_reset(ar: *mut ar9170);
}
// MAC
extern "C" {
    pub fn carl9170_init_mac(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_qos(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_update_multicast(ar: *mut ar9170, mc_hast: u64) -> c_int;
}
extern "C" {
    pub fn carl9170_set_operating_mode(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_beacon_timers(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_dyn_sifs_ack(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_rts_cts_rate(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_ampdu_settings(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_slot_time(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_mac_rates(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_set_hwretry_limit(ar: *mut ar9170, max_retry: u32) -> c_int;
}
extern "C" {
    pub fn carl9170_disable_key(ar: *mut ar9170, id: u8) -> c_int;
}
extern "C" {
    pub fn carl9170_set_mac_tpc(ar: *mut ar9170, channel: *mut ieee80211_channel) -> c_int;
}
// RX
extern "C" {
    pub fn carl9170_rx(ar: *mut ar9170, buf: *mut c_void, len: c_uint);
}
extern "C" {
    pub fn carl9170_handle_command_response(ar: *mut ar9170, buf: *mut c_void, len: u32);
}
// TX
extern "C" {
    pub fn carl9170_tx_janitor(work: *mut work_struct);
}
extern "C" {
    pub fn carl9170_tx_callback(ar: *mut ar9170, skb: *mut sk_buff);
}
extern "C" {
    pub fn carl9170_tx_drop(ar: *mut ar9170, skb: *mut sk_buff);
}
extern "C" {
    pub fn carl9170_tx_scheduler(ar: *mut ar9170);
}
extern "C" {
    pub fn carl9170_tx_get_skb(skb: *mut sk_buff);
}
extern "C" {
    pub fn carl9170_tx_put_skb(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn carl9170_update_beacon(ar: *mut ar9170, submit: bool) -> c_int;
}
// LEDs

extern "C" {
    pub fn carl9170_led_register(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_led_unregister(ar: *mut ar9170);
}

extern "C" {
    pub fn carl9170_led_init(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn carl9170_led_set_state(ar: *mut ar9170, led_state: u32) -> c_int;
}
// PHY / RF
extern "C" {
    pub fn carl9170_get_noisefloor(ar: *mut ar9170) -> c_int;
}
// FW
extern "C" {
    pub fn carl9170_parse_firmware(ar: *mut ar9170) -> c_int;
}
extern "C" {
    pub fn get_seq_h(_arg: carl9170_get_hdr(skb)) -> return;
}
extern "C" {
    pub fn ieee80211_get_tid(_arg: carl9170_get_hdr(skb)) -> return;
}
extern "C" {
    pub fn container_of()priv: *mut (void, ieee80211_vif: struct, _arg: drv_priv) -> return;
}
// Protected by ar->mutex or RCU
extern "C" {
    pub fn carl9170_get_vif(_arg: cvif) -> return;
}
