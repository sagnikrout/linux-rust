//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ath9k.h
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
// Copyright (c) 2008-2011 Atheros Communications Inc.
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

//
// Descriptor Management
//
pub const ATH_TXSTATUS_RING_SIZE: c_int = 512;
// Macro to expand scalars to 64-bit objects

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_descdma {
    pub dd_desc: *mut c_void,
    pub dd_desc_paddr: dma_addr_t,
    pub dd_desc_len: u32,
}

//
// RX / TX
//

// increment with wrap-around

pub const ATH_RXBUF: c_int = 512;
pub const ATH_TXBUF: c_int = 512;
pub const ATH_TXBUF_RESERVE: c_int = 5;
pub const ATH_TXMAXTRY: c_int = 13;
pub const ATH_MAX_SW_RETRIES: c_int = 30;

pub const ATH_AGGR_DELIM_SZ: c_int = 4;

// number of delimiters for encryption padding
pub const ATH_AGGR_ENCRYPTDELIM: c_int = 10;
// minimum h/w qdepth to be sustained to maximize aggregation
pub const ATH_AGGR_MIN_QDEPTH: c_int = 2;
// minimum h/w qdepth for non-aggregated traffic
pub const ATH_NON_AGGR_MIN_QDEPTH: c_int = 8;
pub const ATH_HW_CHECK_POLL_INT: c_int = 1000;
pub const ATH_TXFIFO_DEPTH: c_int = 8;
pub const ATH_TX_ERROR: c_uint = 0x01;
// Stop tx traffic 1ms before the GO goes away
pub const ATH_P2P_PS_STOP_TIME: c_int = 1000;
pub const IEEE80211_SEQ_SEQ_SHIFT: c_int = 4;
pub const IEEE80211_SEQ_MAX: c_int = 4096;
pub const IEEE80211_WEP_IVLEN: c_int = 3;
pub const IEEE80211_WEP_KIDLEN: c_int = 1;
pub const IEEE80211_WEP_CRCLEN: c_int = 4;

// return whether a bit at index _n in bitmap _bm is set
// _sz is the size of the bitmap

// return block-ack bitmap index given sequence and starting sequence

// return the seqno for _start + _offset

// returns delimiter padding required given the packet length

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_txq {
    pub /: *mut *mut int mac80211_qnum; / mac80211 queue number, -1 means not mac80211 Q,
    pub /: *mut *mut u32 axq_qnum; / ath9k hardware queue number,
    pub axq_link: *mut c_void,
    pub axq_q: list_head,
    pub axq_lock: spinlock_t,
    pub axq_depth: u32,
    pub axq_ampdu_depth: u32,
    pub axq_tx_inprogress: bool,
    pub txq_fifo: [list_head; ATH_TXFIFO_DEPTH],
    pub txq_headidx: u8,
    pub txq_tailidx: u8,
    pub pending_frames: c_int,
    pub complete_q: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_frame_info {
    pub bf: *mut ath_buf,
    pub framelen: u16,
    pub txq: i8,
    pub keyix: u8,
    pub rtscts_rate: u8,
    pub 6: u8 retries :,
    pub 1: u8 dyn_smps :,
    pub 1: u8 baw_tracked :,
    pub tx_power: u8,
    pub keytype:2: ath9k_key_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_rxbuf {
    pub list: list_head,
    pub bf_mpdu: *mut sk_buff,
    pub bf_desc: *mut c_void,
    pub bf_daddr: dma_addr_t,
    pub bf_buf_addr: dma_addr_t,
}

//
// enum buffer_type - Buffer type flags
//
// @BUF_AMPDU: This buffer is an ampdu, as part of an aggregate (during TX)
// @BUF_AGGR: Indicates whether the buffer can be aggregated
// (used in aggregation scheduling)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum buffer_type {
    BUF_AMPDU		= BIT(0),
    BUF_AGGR		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_buf_state {
    pub bf_type: u8,
    pub bfs_paprd: u8,
    pub ndelim: u8,
    pub stale: bool,
    pub seqno: u16,
    pub bfs_paprd_timestamp: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_buf {
    pub list: list_head,
    pub or: *mut *mut *mut ath_buf bf_lastbf; / last buf of this unit (a frame,
    pub /: *mut *mut *mut ath_buf bf_next; / next subframe in the aggregate,
    pub /: *mut *mut *mut sk_buff bf_mpdu; / enclosing frame structure,
    pub /: *mut *mut *mut void bf_desc; / virtual addr of desc,
    pub /: *mut *mut dma_addr_t bf_daddr; / physical addr of desc,
    pub /: *mut *mut dma_addr_t bf_buf_addr; / physical addr of data buffer, for DMA,
    pub rates: [ieee80211_tx_rate; 4],
    pub bf_state: ath_buf_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_atx_tid {
    pub list: list_head,
    pub retry_q: sk_buff_head,
    pub an: *mut ath_node,
    pub txq: *mut ath_txq,
    pub tx_buf: [c_ulong; BITS_TO_LONGS(ATH_TID_MAX_BUFS)],
    pub seq_start: u16,
    pub seq_next: u16,
    pub baw_size: u16,
    pub tidno: u8,
    pub /: *mut *mut int baw_head; / first un-acked tx buffer,
    pub /: *mut *mut int baw_tail; / next unused tx buffer slot,
    pub bar_index: i8,
    pub active: bool,
    pub clear_ps_filter: bool,
}

extern "C" {
    pub fn ath_tx_queue_tid(sc: *mut ath_softc, tid: *mut ath_atx_tid);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_node {
    pub sc: *mut ath_softc,
    pub /: *mut *mut *mut ieee80211_sta sta; / station we're part of,
    pub /: *mut *mut *mut ieee80211_vif vif; / interface with which we're associated,
    pub maxampdu: u16,
    pub mpdudensity: u8,
    pub ps_key: i8,
    pub sleeping: bool,
    pub no_ps_filter: bool,

    pub rx_rate_stats: ath_rx_rate_stats,
    pub key_idx: [u8; 4],
    pub ackto: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_tx_control {
    pub txq: *mut ath_txq,
    pub sta: *mut ieee80211_sta,
    pub paprd: u8,
}

//
// @txq_map:  Index is mac80211 queue number.  This is
// not necessarily the same as the hardware queue number
// (axq_qnum).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_tx {
    pub txqsetup: u32,
    pub txbuflock: spinlock_t,
    pub txbuf: list_head,
    pub txq: [ath_txq; ATH9K_NUM_TX_QUEUES],
    pub txdma: ath_descdma,
    pub txq_map: [*mut ath_txq; IEEE80211_NUM_ACS],
    pub uapsdq: *mut ath_txq,
    pub max_aggr_framelen: [u16; IEEE80211_NUM_ACS][4][32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_rx_edma {
    pub rx_fifo: sk_buff_head,
    pub rx_fifo_hwsize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_rx {
    pub defant: u8,
    pub rxotherant: u8,
    pub discard_next: bool,
    pub rxlink: *mut u32,
    pub num_pkts: u32,
    pub rxbuf: list_head,
    pub rxdma: ath_descdma,
    pub rx_edma: [ath_rx_edma; ATH9K_RX_QUEUE_MAX],
    pub buf_hold: *mut ath_rxbuf,
    pub frag: *mut sk_buff,
    pub ampdu_ref: u32,
}

//
// Channel Context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_acq {
    pub acq_new: list_head,
    pub acq_old: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_chanctx {
    pub chandef: cfg80211_chan_def,
    pub vifs: list_head,
    pub acq: [ath_acq; IEEE80211_NUM_ACS],
    pub hw_queue_base: c_int,
// do not dereference, use for comparison only
    pub primary_sta: *mut ieee80211_vif,
    pub beacon: ath_beacon_config,
    pub caldata: ath9k_hw_cal_data,
    pub tsf_ts: ktime_t,
    pub tsf_val: u64,
    pub last_beacon: u32,
    pub flush_timeout: c_int,
    pub txpower: u16,
    pub cur_txpower: u16,
    pub offchannel: bool,
    pub stopped: bool,
    pub active: bool,
    pub assigned: bool,
    pub switch_after_beacon: bool,
    pub nvifs: c_short,
    pub nvifs_assigned: c_short,
    pub rxfilter: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_chanctx_event {
    ATH_CHANCTX_EVENT_BEACON_PREPARE,
    ATH_CHANCTX_EVENT_BEACON_SENT,
    ATH_CHANCTX_EVENT_TSF_TIMER,
    ATH_CHANCTX_EVENT_BEACON_RECEIVED,
    ATH_CHANCTX_EVENT_AUTHORIZED,
    ATH_CHANCTX_EVENT_SWITCH,
    ATH_CHANCTX_EVENT_ASSIGN,
    ATH_CHANCTX_EVENT_UNASSIGN,
    ATH_CHANCTX_EVENT_CHANGE,
    ATH_CHANCTX_EVENT_ENABLE_MULTICHANNEL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_chanctx_state {
    ATH_CHANCTX_STATE_IDLE,
    ATH_CHANCTX_STATE_WAIT_FOR_BEACON,
    ATH_CHANCTX_STATE_WAIT_FOR_TIMER,
    ATH_CHANCTX_STATE_SWITCH,
    ATH_CHANCTX_STATE_FORCE_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_chanctx_sched {
    pub beacon_pending: bool,
    pub beacon_adjust: bool,
    pub offchannel_pending: bool,
    pub wait_switch: bool,
    pub force_noa_update: bool,
    pub extend_absence: bool,
    pub mgd_prepare_tx: bool,
    pub state: ath_chanctx_state,
    pub beacon_miss: u8,
    pub next_tbtt: u32,
    pub switch_start_time: u32,
    pub offchannel_duration: c_uint,
    pub channel_switch_time: c_uint,
// backup, in case the hardware timer fails
    pub timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_offchannel_state {
    ATH_OFFCHANNEL_IDLE,
    ATH_OFFCHANNEL_PROBE_SEND,
    ATH_OFFCHANNEL_PROBE_WAIT,
    ATH_OFFCHANNEL_SUSPEND,
    ATH_OFFCHANNEL_ROC_START,
    ATH_OFFCHANNEL_ROC_WAIT,
    ATH_OFFCHANNEL_ROC_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_roc_complete_reason {
    ATH_ROC_COMPLETE_EXPIRE,
    ATH_ROC_COMPLETE_ABORT,
    ATH_ROC_COMPLETE_CANCEL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_offchannel {
    pub chan: ath_chanctx,
    pub timer: timer_list,
    pub scan_req: *mut cfg80211_scan_request,
    pub scan_vif: *mut ieee80211_vif,
    pub scan_idx: c_int,
    pub state: ath_offchannel_state,
    pub roc_chan: *mut ieee80211_channel,
    pub roc_vif: *mut ieee80211_vif,
    pub roc_duration: c_int,
    pub duration: c_int,
}

extern "C" {
    pub fn ath_chanctx_init(sc: *mut ath_softc);
}

extern "C" {
    pub fn ath9k_is_chanctx_enabled() -> bool;
}
extern "C" {
    pub fn ath9k_fill_chanctx_ops();
}
extern "C" {
    pub fn ath9k_init_channel_context(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_offchannel_init(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_deinit_channel_context(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_init_p2p(sc: *mut ath_softc) -> c_int;
}
extern "C" {
    pub fn ath9k_deinit_p2p(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_p2p_beacon_sync(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_p2p_ps_timer(priv: *mut c_void);
}
extern "C" {
    pub fn ath9k_chanctx_wake_queues(sc: *mut ath_softc, ctx: *mut ath_chanctx);
}
extern "C" {
    pub fn ath9k_chanctx_stop_queues(sc: *mut ath_softc, ctx: *mut ath_chanctx);
}
extern "C" {
    pub fn ath_chanctx_check_active(sc: *mut ath_softc, ctx: *mut ath_chanctx);
}
extern "C" {
    pub fn ath_chanctx_set_next(sc: *mut ath_softc, force: bool);
}
extern "C" {
    pub fn ath_offchannel_next(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_scan_complete(sc: *mut ath_softc, abort: bool);
}
extern "C" {
    pub fn ath_is_go_chanctx_present(sc: *mut ath_softc) -> *mut ath_chanctx;
}

extern "C" {
    pub fn ath_startrecv(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_stoprecv(sc: *mut ath_softc) -> bool;
}
extern "C" {
    pub fn ath_calcrxfilter(sc: *mut ath_softc) -> u32;
}
extern "C" {
    pub fn ath_rx_init(sc: *mut ath_softc, nbufs: c_int) -> c_int;
}
extern "C" {
    pub fn ath_rx_cleanup(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_rx_tasklet(sc: *mut ath_softc, flush: c_int, hp: bool) -> c_int;
}
extern "C" {
    pub fn ath_txq_unlock_complete(sc: *mut ath_softc, txq: *mut ath_txq);
}
extern "C" {
    pub fn ath_tx_cleanupq(sc: *mut ath_softc, txq: *mut ath_txq);
}
extern "C" {
    pub fn ath_drain_all_txq(sc: *mut ath_softc) -> bool;
}
extern "C" {
    pub fn ath_draintxq(sc: *mut ath_softc, txq: *mut ath_txq);
}
extern "C" {
    pub fn ath_tx_node_init(sc: *mut ath_softc, an: *mut ath_node);
}
extern "C" {
    pub fn ath_tx_node_cleanup(sc: *mut ath_softc, an: *mut ath_node);
}
extern "C" {
    pub fn ath_txq_schedule(sc: *mut ath_softc, txq: *mut ath_txq);
}
extern "C" {
    pub fn ath_txq_schedule_all(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_tx_init(sc: *mut ath_softc, nbufs: c_int) -> c_int;
}
extern "C" {
    pub fn ath_update_max_aggr_framelen(sc: *mut ath_softc, queue: c_int, txop: c_int);
}
extern "C" {
    pub fn ath_assign_seq(common: *mut ath_common, skb: *mut sk_buff);
}
extern "C" {
    pub fn ath_tx_tasklet(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_tx_edma_tasklet(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_tx_aggr_stop(sc: *mut ath_softc, sta: *mut ieee80211_sta, tid: u16);
}
extern "C" {
    pub fn ath_tx_aggr_wakeup(sc: *mut ath_softc, an: *mut ath_node);
}
extern "C" {
    pub fn ath9k_wake_tx_queue(hw: *mut ieee80211_hw, queue: *mut ieee80211_txq);
}
//
// VIFs
//
pub const P2P_DEFAULT_CTWIN: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_vif {
    pub list: list_head,
    pub seq_no: u16,
// BSS info
    pub __aligned(2): u8 bssid[ETH_ALEN],
    pub aid: u16,
    pub assoc: bool,
    pub vif: *mut ieee80211_vif,
    pub mcast_node: ath_node,
    pub av_bslot: c_int,
    pub /: *mut *mut __le64 tsf_adjust; / TSF adjustment for staggered beacons,
    pub av_bcbuf: *mut ath_buf,
    pub chanctx: *mut ath_chanctx,
// P2P Client
    pub noa: ieee80211_noa_data,
// P2P GO
    pub noa_index: u8,
    pub offchannel_start: u32,
    pub offchannel_duration: u32,
// These are used for both periodic and one-shot
    pub noa_start: u32,
    pub noa_duration: u32,
    pub periodic_noa: bool,
    pub oneshot_noa: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_vif_iter_data {
    pub /: *mut *mut u8 hw_macaddr[ETH_ALEN]; / address of the first vif,
    pub /: *mut *mut u8 mask[ETH_ALEN]; / bssid mask,
    pub has_hw_macaddr: bool,
    pub slottime: u8,
    pub beacons: bool,
    pub /: *mut *mut int naps; / number of AP vifs,
    pub /: *mut *mut int nmeshes; / number of mesh vifs,
    pub /: *mut *mut int nstations; / number of station vifs,
    pub /: *mut *mut int nadhocs; / number of adhoc vifs,
    pub /: *mut *mut int nocbs; / number of OCB vifs,
    pub /: *mut *mut int nbcnvifs; / number of beaconing vifs,
    pub primary_beacon_vif: *mut ieee80211_vif,
    pub primary_sta: *mut ieee80211_vif,
}

extern "C" {
    pub fn ath9k_set_txpower(sc: *mut ath_softc, vif: *mut ieee80211_vif);
}
//
// Beacon Handling
//
// Regardless of the number of beacons we stagger, (i.e. regardless of the
// number of BSSIDs) if a given beacon does not go out even after waiting this
// number of beacon intervals, the game's up.
//
pub const BSTUCK_THRESH: c_int = 9;
pub const ATH_BCBUF: c_int = 8;

pub const ATH_DEFAULT_BMISS_LIMIT: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_beacon {
    pub /: *mut *mut } updateslot; / slot time update fsm,
    pub beaconq: u32,
    pub bmisscnt: u32,
    pub bslot: [*mut ieee80211_vif; ATH_BCBUF],
    pub slottime: c_int,
    pub slotupdate: c_int,
    pub bdma: ath_descdma,
    pub cabq: *mut ath_txq,
    pub bbuf: list_head,
    pub tx_processed: bool,
    pub tx_last: bool,
}

extern "C" {
    pub fn ath9k_beacon_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ath9k_beacon_assign_slot(sc: *mut ath_softc, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn ath9k_beacon_remove_slot(sc: *mut ath_softc, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn ath9k_beacon_ensure_primary_slot(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_set_beacon(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_csa_is_finished(sc: *mut ath_softc, vif: *mut ieee80211_vif) -> bool;
}
extern "C" {
    pub fn ath9k_csa_update(sc: *mut ath_softc);
}
//
// Link Monitoring
//

pub const ATH_ANI_MAX_SKIP_COUNT: c_int = 10;

pub const ATH_PLL_WORK_INTERVAL: c_int = 100;
extern "C" {
    pub fn ath_hw_check_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath_hw_check(sc: *mut ath_softc) -> bool;
}
extern "C" {
    pub fn ath_hw_pll_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath_paprd_calibrate(work: *mut work_struct);
}
extern "C" {
    pub fn ath_ani_calibrate(t: *mut timer_list);
}
extern "C" {
    pub fn ath_start_ani(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_stop_ani(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_check_ani(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_update_survey_stats(sc: *mut ath_softc) -> c_int;
}
extern "C" {
    pub fn ath_update_survey_nf(sc: *mut ath_softc, channel: c_int);
}
extern "C" {
    pub fn ath9k_queue_reset(sc: *mut ath_softc, type: ath_reset_type);
}
extern "C" {
    pub fn ath_ps_full_sleep(t: *mut timer_list);
}
//
// BTCOEX
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_op_flags {
    BT_OP_PRIORITY_DETECTED,
    BT_OP_SCAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_btcoex {
    pub btcoex_lock: spinlock_t,
    pub /: *mut *mut timer_list period_timer; / Timer for BT period,
    pub no_stomp_timer: timer_list,
    pub bt_priority_cnt: u32,
    pub bt_priority_time: c_ulong,
    pub op_flags: c_ulong,
    pub /: *mut *mut int bt_stomp_type; / Types of BT stomping,
    pub /: *mut *mut u32 btcoex_no_stomp; / in msec,
    pub /: *mut *mut u32 btcoex_period; / in msec,
    pub /: *mut *mut u32 btscan_no_stomp; / in msec,
    pub duty_cycle: u32,
    pub bt_wait_time: u32,
    pub rssi_count: c_int,
    pub mci: ath_mci_profile,
    pub stomp_audio: u8,
}

extern "C" {
    pub fn ath9k_init_btcoex(sc: *mut ath_softc) -> c_int;
}
extern "C" {
    pub fn ath9k_deinit_btcoex(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_start_btcoex(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_stop_btcoex(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_btcoex_timer_resume(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_btcoex_timer_pause(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_btcoex_handle_interrupt(sc: *mut ath_softc, status: u32);
}
extern "C" {
    pub fn ath9k_btcoex_aggr_limit(sc: *mut ath_softc, max_4ms_framelen: u32) -> u16;
}
extern "C" {
    pub fn ath9k_btcoex_stop_gen_timer(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_dump_btcoex(sc: *mut ath_softc, buf: *mut u8, size: u32) -> c_int;
}

//
// LED Control
//
pub const ATH_LED_PIN_DEF: c_int = 1;
pub const ATH_LED_PIN_9287: c_int = 8;
pub const ATH_LED_PIN_9300: c_int = 10;
pub const ATH_LED_PIN_9485: c_int = 6;
pub const ATH_LED_PIN_9462: c_int = 4;

extern "C" {
    pub fn ath_init_leds(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_deinit_leds(sc: *mut ath_softc);
}

//
// Wake on Wireless LAN
//

extern "C" {
    pub fn ath9k_init_wow(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn ath9k_deinit_wow(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn ath9k_resume(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn ath9k_set_wakeup(hw: *mut ieee80211_hw, enabled: bool);
}

//
// Antenna diversity/combining
//
pub const ATH_ANT_RX_CURRENT_SHIFT: c_int = 4;
pub const ATH_ANT_RX_MAIN_SHIFT: c_int = 2;
pub const ATH_ANT_RX_MASK: c_uint = 0x3;
pub const ATH_ANT_DIV_COMB_SHORT_SCAN_INTR: c_int = 50;
pub const ATH_ANT_DIV_COMB_SHORT_SCAN_PKTCOUNT: c_uint = 0x100;
pub const ATH_ANT_DIV_COMB_MAX_PKTCOUNT: c_uint = 0x200;
pub const ATH_ANT_DIV_COMB_INIT_COUNT: c_int = 95;
pub const ATH_ANT_DIV_COMB_MAX_COUNT: c_int = 100;
pub const ATH_ANT_DIV_COMB_ALT_ANT_RATIO: c_int = 30;
pub const ATH_ANT_DIV_COMB_ALT_ANT_RATIO2: c_int = 20;
pub const ATH_ANT_DIV_COMB_ALT_ANT_RATIO_LOW_RSSI: c_int = 50;
pub const ATH_ANT_DIV_COMB_ALT_ANT_RATIO2_LOW_RSSI: c_int = 50;

pub const ATH_ANT_DIV_COMB_LNA1_DELTA_LOW: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_ant_comb {
    pub count: u16,
    pub total_pkt_count: u16,
    pub scan: bool,
    pub scan_not_start: bool,
    pub main_total_rssi: c_int,
    pub alt_total_rssi: c_int,
    pub alt_recv_cnt: c_int,
    pub main_recv_cnt: c_int,
    pub rssi_lna1: c_int,
    pub rssi_lna2: c_int,
    pub rssi_add: c_int,
    pub rssi_sub: c_int,
    pub rssi_first: c_int,
    pub rssi_second: c_int,
    pub rssi_third: c_int,
    pub ant_ratio: c_int,
    pub ant_ratio2: c_int,
    pub alt_good: bool,
    pub quick_scan_cnt: c_int,
    pub main_conf: ath9k_ant_div_comb_lna_conf,
    pub first_quick_scan_conf: ath9k_ant_div_comb_lna_conf,
    pub second_quick_scan_conf: ath9k_ant_div_comb_lna_conf,
    pub first_ratio: bool,
    pub second_ratio: bool,
    pub scan_start_time: c_ulong,
//
// Card-specific config values.
//
    pub low_rssi_thresh: c_int,
    pub fast_div_bias: c_int,
}

extern "C" {
    pub fn ath_ant_comb_scan(sc: *mut ath_softc, rs: *mut ath_rx_status);
}
//
// Main driver core
//
pub const ATH9K_PCI_CUS198: c_uint = 0x0001;
pub const ATH9K_PCI_CUS230: c_uint = 0x0002;
pub const ATH9K_PCI_CUS217: c_uint = 0x0004;
pub const ATH9K_PCI_CUS252: c_uint = 0x0008;
pub const ATH9K_PCI_WOW: c_uint = 0x0010;
pub const ATH9K_PCI_BT_ANT_DIV: c_uint = 0x0020;
pub const ATH9K_PCI_D3_L1_WAR: c_uint = 0x0040;
pub const ATH9K_PCI_AR9565_1ANT: c_uint = 0x0080;
pub const ATH9K_PCI_AR9565_2ANT: c_uint = 0x0100;
pub const ATH9K_PCI_NO_PLL_PWRSAVE: c_uint = 0x0200;
pub const ATH9K_PCI_KILLER: c_uint = 0x0400;
pub const ATH9K_PCI_LED_ACT_HI: c_uint = 0x0800;
//
// Default cache line size, in bytes.
// Used when PCI device not fully initialized by bootrom/BIOS
//
pub const DEFAULT_CACHELINE: c_int = 32;

pub const MAX_GTT_CNT: c_int = 5;
// Powersave flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_softc {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub cur_survey: *mut survey_info,
    pub survey: [survey_info; ATH9K_NUM_CHANNELS],
    pub intr_lock: spinlock_t,
    pub intr_tq: tasklet_struct,
    pub bcon_tasklet: tasklet_struct,
    pub sc_ah: *mut ath_hw,
    pub mem: *mut void __iomem,
    pub irq: c_int,
    pub sc_serial_rw: spinlock_t,
    pub sc_pm_lock: spinlock_t,
    pub sc_pcu_lock: spinlock_t,
    pub mutex: mutex,
    pub paprd_work: work_struct,
    pub hw_reset_work: work_struct,
    pub paprd_complete: completion,
    pub tx_wait: wait_queue_head_t,

    pub chanctx_work: work_struct,
    pub p2p_ps_timer: *mut ath_gen_timer,
    pub p2p_ps_vif: *mut ath_vif,
    pub sched: ath_chanctx_sched,
    pub offchannel: ath_offchannel,
    pub next_chan: *mut ath_chanctx,
    pub go_beacon: completion,
    pub last_event_time: ktime_t,

    pub driver_data: c_ulong,
    pub gtt_cnt: u8,
    pub intrstatus: u32,
    pub rx_active_check_time: c_ulong,
    pub rx_active_count: u32,
    pub /: *mut *mut *mut u16 ps_flags; / PS_,
    pub ps_enabled: bool,
    pub ps_idle: bool,
    pub nbcnvifs: c_short,
    pub ps_usecount: c_ulong,
    pub rx: ath_rx,
    pub tx: ath_tx,
    pub beacon: ath_beacon,
    pub cur_chandef: cfg80211_chan_def,
    pub chanctx: [ath_chanctx; ATH9K_NUM_CHANCTX],
    pub cur_chan: *mut ath_chanctx,
    pub chan_lock: spinlock_t,

    pub led_registered: bool,
    pub led_name: [c_char; 32],
    pub led_cdev: led_classdev,

    pub debug: ath9k_debug,

    pub hw_check_work: delayed_work,
    pub hw_pll_work: delayed_work,
    pub sleep_timer: timer_list,

    pub btcoex: ath_btcoex,
    pub mci_coex: ath_mci_coex,
    pub mci_work: work_struct,

    pub txsdma: ath_descdma,
    pub ant_comb: ath_ant_comb,
    pub ant_rx: u8 ant_tx,,
    pub dfs_detector: *mut dfs_pattern_detector,
    pub dfs_prev_pulse_ts: u64,
    pub wow_enabled: u32,
    pub spec_priv: ath_spec_scan_priv,
    pub tx99_vif: *mut ieee80211_vif,
    pub tx99_skb: *mut sk_buff,
    pub tx99_state: bool,
    pub tx99_power: i16,

    pub wow_intr_before_sleep: u32,
    pub force_wow: bool,

    pub rng_ops: hwrng,
    pub rng_last: u32,
    pub rng_name: [c_char; sizeof("ath9k_65535")],
}

//
// TX99
//

extern "C" {
    pub fn ath9k_tx99_init_debug(sc: *mut ath_softc);
}

//
// Random Number Generator
//

extern "C" {
    pub fn ath9k_rng_start(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_rng_stop(sc: *mut ath_softc);
}

extern "C" {
    pub fn ath9k_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ath_cabq_update(: *mut ath_softc) -> c_int;
}
extern "C" {
    pub fn ath9k_parse_mpdudensity(mpdudensity: u8) -> u8;
}
extern "C" {
    pub fn ath_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ath_reset(sc: *mut ath_softc, hchan: *mut ath9k_channel) -> c_int;
}
extern "C" {
    pub fn ath_cancel_work(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_restart_work(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_deinit_device(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_txchainmask_reduction(sc: *mut ath_softc, chainmask: u8, rate: u32) -> u8;
}
extern "C" {
    pub fn ath_start_rfkill_poll(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_rfkill_poll_state(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn ath9k_ps_wakeup(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath9k_ps_restore(sc: *mut ath_softc);
}

extern "C" {
    pub fn ath_pci_init() -> c_int;
}
extern "C" {
    pub fn ath_pci_exit();
}

extern "C" {
    pub fn ath_ahb_init() -> c_int;
}
extern "C" {
    pub fn ath_ahb_exit();
}

