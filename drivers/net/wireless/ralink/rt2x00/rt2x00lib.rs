//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00lib.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Interval defines
//

pub const LINK_TUNE_SECONDS: c_int = 1;

pub const AGC_SECONDS: c_int = 4;
pub const VCO_SECONDS: c_int = 10;
//
// rt2x00_rate: Per rate device information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_rate {
    pub flags: c_ushort,
pub const DEV_RATE_CCK: c_uint = 0x0001;
pub const DEV_RATE_OFDM: c_uint = 0x0002;
pub const DEV_RATE_SHORT_PREAMBLE: c_uint = 0x0004;
    pub /: *mut *mut unsigned short bitrate; / In 100kbit/s,
    pub ratemask: c_ushort,
    pub plcp: c_ushort,
    pub mcs: c_ushort,
}

//
// Radio control handlers.
//
extern "C" {
    pub fn rt2x00lib_enable_radio(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00lib_disable_radio(rt2x00dev: *mut rt2x00_dev);
}
//
// Initialization handlers.
//
extern "C" {
    pub fn rt2x00lib_start(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00lib_stop(rt2x00dev: *mut rt2x00_dev);
}
//
// Configuration handlers.
//
// DOC: Queue handlers
//
// rt2x00queue_alloc_rxskb - allocate a skb for RX purposes.
// @entry: The entry for which the skb will be applicable.
//
// rt2x00queue_free_skb - free a skb
// @entry: The entry for which the skb will be applicable.
//
extern "C" {
    pub fn rt2x00queue_free_skb(entry: *mut queue_entry);
}
//
// rt2x00queue_align_frame - Align 802.11 frame to 4-byte boundary
// @skb: The skb to align
//
// Align the start of the 802.11 frame to a 4-byte boundary, this could
// mean the payload is not aligned properly though.
//
extern "C" {
    pub fn rt2x00queue_align_frame(skb: *mut sk_buff);
}
//
// rt2x00queue_insert_l2pad - Align 802.11 header & payload to 4-byte boundary
// @skb: The skb to align
// @header_length: Length of 802.11 header
//
// Apply L2 padding to align both header and payload to 4-byte boundary
//
extern "C" {
    pub fn rt2x00queue_insert_l2pad(skb: *mut sk_buff, header_length: c_uint);
}
//
// rt2x00queue_insert_l2pad - Remove L2 padding from 802.11 frame
// @skb: The skb to align
// @header_length: Length of 802.11 header
//
// Remove L2 padding used to align both header and payload to 4-byte boundary,
// by removing the L2 padding the header will no longer be 4-byte aligned.
//
extern "C" {
    pub fn rt2x00queue_remove_l2pad(skb: *mut sk_buff, header_length: c_uint);
}
//
// rt2x00queue_write_tx_frame - Write TX frame to hardware
// @queue: Queue over which the frame should be send
// @skb: The skb to send
// @local: frame is not from mac80211
//
// rt2x00queue_update_beacon - Send new beacon from mac80211
// to hardware. Handles locking by itself (mutex).
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @vif: Interface for which the beacon should be updated.
//
// rt2x00queue_update_beacon_locked - Send new beacon from mac80211
// to hardware. Caller needs to ensure locking.
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @vif: Interface for which the beacon should be updated.
//
// rt2x00queue_clear_beacon - Clear beacon in hardware
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @vif: Interface for which the beacon should be updated.
//
// rt2x00queue_index_inc - Index incrementation function
// @entry: Queue entry (&struct queue_entry) to perform the action on.
// @index: Index type (&enum queue_index) to perform the action on.
//
// This function will increase the requested index on the entry's queue,
// it will grab the appropriate locks and handle queue overflow events by
// resetting the index to the start of the queue.
//
extern "C" {
    pub fn rt2x00queue_index_inc(entry: *mut queue_entry, index: queue_index);
}
//
// rt2x00queue_init_queues - Initialize all data queues
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// This function will loop through all available queues to clear all
// index numbers and set the queue entry to the correct initialization
// state.
//
extern "C" {
    pub fn rt2x00queue_init_queues(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00queue_initialize(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00queue_uninitialize(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00queue_allocate(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00queue_free(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00link_update_stats - Update link statistics from RX frame
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @skb: Received frame
// @rxdesc: Received frame descriptor
//
// Update link statistics based on the information from the
// received frame descriptor.
//
// rt2x00link_start_tuner - Start periodic link tuner work
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// This start the link tuner periodic work, this work will
// be executed periodically until &rt2x00link_stop_tuner has
// been called.
//
extern "C" {
    pub fn rt2x00link_start_tuner(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00link_stop_tuner - Stop periodic link tuner work
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// After this function completed the link tuner will not
// be running until &rt2x00link_start_tuner is called.
//
extern "C" {
    pub fn rt2x00link_stop_tuner(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00link_reset_tuner - Reset periodic link tuner work
// @rt2x00dev: Pointer to &struct rt2x00_dev.
// @antenna: Should the antenna tuning also be reset
//
// The VGC limit configured in the hardware will be reset to 0
// which forces the driver to rediscover the correct value for
// the current association. This is needed when configuration
// options have changed which could drastically change the
// SNR level or link quality (i.e. changing the antenna setting).
//
// Resetting the link tuner will also cause the periodic work counter
// to be reset. Any driver which has a fixed limit on the number
// of rounds the link tuner is supposed to work will accept the
// tuner actions again if this limit was previously reached.
//
// If @antenna is set to true a the software antenna diversity
// tuning will also be reset.
//
extern "C" {
    pub fn rt2x00link_reset_tuner(rt2x00dev: *mut rt2x00_dev, antenna: bool);
}
//
// rt2x00link_start_watchdog - Start periodic watchdog monitoring
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// This start the watchdog periodic work, this work will
// be executed periodically until &rt2x00link_stop_watchdog has
// been called.
//
extern "C" {
    pub fn rt2x00link_start_watchdog(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00link_stop_watchdog - Stop periodic watchdog monitoring
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// After this function completed the watchdog monitoring will not
// be running until &rt2x00link_start_watchdog is called.
//
extern "C" {
    pub fn rt2x00link_stop_watchdog(rt2x00dev: *mut rt2x00_dev);
}
//
// rt2x00link_register - Initialize link tuning & watchdog functionality
// @rt2x00dev: Pointer to &struct rt2x00_dev.
//
// Initialize work structure and all link tuning and watchdog related
// parameters. This will not start the periodic work itself.
//
extern "C" {
    pub fn rt2x00link_register(rt2x00dev: *mut rt2x00_dev);
}
//
// Firmware handlers.
//

extern "C" {
    pub fn rt2x00lib_load_firmware(rt2x00dev: *mut rt2x00_dev) -> c_int;
}
extern "C" {
    pub fn rt2x00lib_free_firmware(rt2x00dev: *mut rt2x00_dev);
}

//
// Debugfs handlers.
//

extern "C" {
    pub fn rt2x00debug_register(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00debug_deregister(rt2x00dev: *mut rt2x00_dev);
}

//
// Crypto handlers.
//

extern "C" {
    pub fn rt2x00crypto_key_to_cipher(key: *mut ieee80211_key_conf) -> cipher;
}
extern "C" {
    pub fn rt2x00crypto_tx_insert_iv(skb: *mut sk_buff, header_length: c_uint);
}

//
// RFkill handlers.
//
// LED handlers
//

extern "C" {
    pub fn rt2x00leds_led_quality(rt2x00dev: *mut rt2x00_dev, rssi: c_int);
}
extern "C" {
    pub fn rt2x00led_led_activity(rt2x00dev: *mut rt2x00_dev, enabled: bool);
}
extern "C" {
    pub fn rt2x00leds_led_assoc(rt2x00dev: *mut rt2x00_dev, enabled: bool);
}
extern "C" {
    pub fn rt2x00leds_led_radio(rt2x00dev: *mut rt2x00_dev, enabled: bool);
}
extern "C" {
    pub fn rt2x00leds_register(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00leds_unregister(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00leds_suspend(rt2x00dev: *mut rt2x00_dev);
}
extern "C" {
    pub fn rt2x00leds_resume(rt2x00dev: *mut rt2x00_dev);
}

