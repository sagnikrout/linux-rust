//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/debug.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_reset_type {
    RESET_TYPE_USER,
    RESET_TYPE_BB_HANG,
    RESET_TYPE_BB_WATCHDOG,
    RESET_TYPE_FATAL_INT,
    RESET_TYPE_TX_ERROR,
    RESET_TYPE_TX_GTT,
    RESET_TYPE_TX_HANG,
    RESET_TYPE_PLL_HANG,
    RESET_TYPE_MAC_HANG,
    RESET_TYPE_BEACON_STUCK,
    RESET_TYPE_MCI,
    RESET_TYPE_CALIBRATION,
    RESET_TX_DMA_ERROR,
    RESET_RX_DMA_ERROR,
    RESET_TYPE_RX_INACTIVE,
    __RESET_TYPE_MAX
}

//
// struct ath_interrupt_stats - Contains statistics about interrupts
// @total: Total no. of interrupts generated so far
// @rxok: RX with no errors
// @rxlp: RX with low priority RX
// @rxhp: RX with high priority, uapsd only
// @rxeol: RX with no more RXDESC available
// @rxorn: RX FIFO overrun
// @txok: TX completed at the requested rate
// @txurn: TX FIFO underrun
// @mib: MIB regs reaching its threshold
// @rxphyerr: RX with phy errors
// @rx_keycache_miss: RX with key cache misses
// @swba: Software Beacon Alert
// @bmiss: Beacon Miss
// @bnr: Beacon Not Ready
// @cst: Carrier Sense TImeout
// @gtt: Global TX Timeout
// @tim: RX beacon TIM occurrence
// @cabend: RX End of CAB traffic
// @dtimsync: DTIM sync lossage
// @dtim: RX Beacon with DTIM
// @bb_watchdog: Baseband watchdog
// @tsfoor: TSF out of range, indicates that the corrected TSF received
// from a beacon differs from the PCU's internal TSF by more than a
// (programmable) threshold
// @local_timeout: Internal bus timeout.
// @mci: MCI interrupt, specific to MCI based BTCOEX chipsets
// @gen_timer: Generic hardware timer interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_interrupt_stats {
    pub total: u32,
    pub rxok: u32,
    pub rxlp: u32,
    pub rxhp: u32,
    pub rxeol: u32,
    pub rxorn: u32,
    pub txok: u32,
    pub txeol: u32,
    pub txurn: u32,
    pub mib: u32,
    pub rxphyerr: u32,
    pub rx_keycache_miss: u32,
    pub swba: u32,
    pub bmiss: u32,
    pub bnr: u32,
    pub cst: u32,
    pub gtt: u32,
    pub tim: u32,
    pub cabend: u32,
    pub dtimsync: u32,
    pub dtim: u32,
    pub bb_watchdog: u32,
    pub tsfoor: u32,
    pub mci: u32,
    pub gen_timer: u32,
// Sync-cause stats
    pub sync_cause_all: u32,
    pub sync_rtc_irq: u32,
    pub sync_mac_irq: u32,
    pub eeprom_illegal_access: u32,
    pub apb_timeout: u32,
    pub pci_mode_conflict: u32,
    pub host1_fatal: u32,
    pub host1_perr: u32,
    pub trcv_fifo_perr: u32,
    pub radm_cpl_ep: u32,
    pub radm_cpl_dllp_abort: u32,
    pub radm_cpl_tlp_abort: u32,
    pub radm_cpl_ecrc_err: u32,
    pub radm_cpl_timeout: u32,
    pub local_timeout: u32,
    pub pm_access: u32,
    pub mac_awake: u32,
    pub mac_asleep: u32,
    pub mac_sleep_access: u32,
}

//
// struct ath_tx_stats - Statistics about TX
// @tx_pkts_all:  No. of total frames transmitted, including ones that
// may have had errors.
// @tx_bytes_all:  No. of total bytes transmitted, including ones that
// may have had errors.
// @queued: Total MPDUs (non-aggr) queued
// @completed: Total MPDUs (non-aggr) completed
// @xretries: Total MPDUs with xretries
// @a_aggr: Total no. of aggregates queued
// @a_queued_hw: Total AMPDUs queued to hardware
// @a_completed: Total AMPDUs completed
// @a_retries: No. of AMPDUs retried (SW)
// @a_xretries: No. of AMPDUs dropped due to xretries
// @txerr_filtered: No. of frames with TXERR_FILT flag set.
// @fifo_underrun: FIFO underrun occurrences
// Valid only for:
// - non-aggregate condition.
// - first packet of aggregate.
// @xtxop: No. of frames filtered because of TXOP limit
// @timer_exp: Transmit timer expiry
// @desc_cfg_err: Descriptor configuration errors
// @data_underrun: TX data underrun errors
// @delim_underrun: TX delimiter underrun errors
// @puttxbuf: Number of times hardware was given txbuf to write.
// @txstart:  Number of times hardware was told to start tx.
// @txprocdesc:  Number of times tx descriptor was processed
// @txfailed:  Out-of-memory or other errors in xmit path.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_tx_stats {
    pub tx_pkts_all: u32,
    pub tx_bytes_all: u32,
    pub queued: u32,
    pub completed: u32,
    pub xretries: u32,
    pub a_aggr: u32,
    pub a_queued_hw: u32,
    pub a_completed: u32,
    pub a_retries: u32,
    pub a_xretries: u32,
    pub txerr_filtered: u32,
    pub fifo_underrun: u32,
    pub xtxop: u32,
    pub timer_exp: u32,
    pub desc_cfg_err: u32,
    pub data_underrun: u32,
    pub delim_underrun: u32,
    pub puttxbuf: u32,
    pub txstart: u32,
    pub txprocdesc: u32,
    pub txfailed: u32,
}

//
// Various utility macros to print TX/Queue counters.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_rx_rate_stats {
    pub ht20_cnt: u32,
    pub ht40_cnt: u32,
    pub sgi_cnt: u32,
    pub lgi_cnt: u32,
    pub ht_stats: [}; 24],
    pub ofdm_cnt: u32,
    pub ofdm_stats: [}; 8],
    pub cck_lp_cnt: u32,
    pub cck_sp_cnt: u32,
    pub cck_stats: [}; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_airtime_stats {
    pub rx_airtime: u32,
    pub tx_airtime: u32,
}

pub const ANT_MAIN: c_int = 0;
pub const ANT_ALT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_antenna_stats {
    pub recv_cnt: u32,
    pub rssi_avg: u32,
    pub lna_recv_cnt: [u32; 4],
    pub lna_attempt_cnt: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_stats {
    pub istats: ath_interrupt_stats,
    pub txstats: [ath_tx_stats; ATH9K_NUM_TX_QUEUES],
    pub rxstats: ath_rx_stats,
    pub dfs_stats: ath_dfs_stats,
    pub ant_stats: [ath_antenna_stats; 2],
    pub reset: [u32; __RESET_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_debug {
    pub debugfs_phy: *mut dentry,
    pub regidx: u32,
    pub stats: ath_stats,
}

extern "C" {
    pub fn ath9k_init_debug(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ath9k_deinit_debug(sc: *mut ath_softc);
}
extern "C" {
    pub fn ath_debug_stat_interrupt(sc: *mut ath_softc, status: ath9k_int);
}
extern "C" {
    pub fn ath_debug_stat_rx(sc: *mut ath_softc, rs: *mut ath_rx_status);
}
extern "C" {
    pub fn ath9k_debug_sync_cause(sc: *mut ath_softc, sync_cause: u32);
}

