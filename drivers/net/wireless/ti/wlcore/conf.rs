//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/conf.h
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
// This file is part of wl1271
//
// Copyright (C) 2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//
// MCS8+ rates overlap with 40Mhz rates
pub const CONF_HW_RXTX_RATE_UNSUPPORTED: c_uint = 0xff;
pub const WLCORE_CONF_SG_PARAMS_MAX: c_int = 67;
pub const WLCORE_CONF_SG_PARAMS_ALL: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_sg_settings {
    pub params: [u32; WLCORE_CONF_SG_PARAMS_MAX],
    pub state: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conf_rx_queue_type {
    CONF_RX_QUEUE_TYPE_LOW_PRIORITY,  /* All except the high priority */
    CONF_RX_QUEUE_TYPE_HIGH_PRIORITY, /* Management and voice packets */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_rx_settings {
//
// The maximum amount of time, in TU, before the
// firmware discards the MSDU.
//
// Range: 0 - 0xFFFFFFFF
//
    pub rx_msdu_life_time: u32,
//
// Packet detection threshold in the PHY.
//
// FIXME: details unknown.
//
    pub packet_detection_threshold: u32,
//
// The longest time the STA will wait to receive traffic from the AP
// after a PS-poll has been transmitted.
//
// Range: 0 - 200000
//
    pub ps_poll_timeout: u16,
//
// The longest time the STA will wait to receive traffic from the AP
// after a frame has been sent from an UPSD enabled queue.
//
// Range: 0 - 200000
//
    pub upsd_timeout: u16,
//
// The number of octets in an MPDU, below which an RTS/CTS
// handshake is not performed.
//
// Range: 0 - 4096
//
    pub rts_threshold: u16,
//
// The RX Clear Channel Assessment threshold in the PHY
// (the energy threshold).
//
// Range: ENABLE_ENERGY_D  == 0x140A
// DISABLE_ENERGY_D == 0xFFEF
//
    pub rx_cca_threshold: u16,
//
// Occupied Rx mem-blocks number which requires interrupting the host
// (0 = no buffering, 0xffff = disabled).
//
// Range: u16
//
    pub irq_blk_threshold: u16,
//
// Rx packets number which requires interrupting the host
// (0 = no buffering).
//
// Range: u16
//
    pub irq_pkt_threshold: u16,
//
// Max time in msec the FW may delay RX-Complete interrupt.
//
// Range: 1 - 100
//
    pub irq_timeout: u16,
//
// The RX queue type.
//
// Range: RX_QUEUE_TYPE_RX_LOW_PRIORITY, RX_QUEUE_TYPE_RX_HIGH_PRIORITY,
//
    pub queue_type: u8,
    pub __packed: },
pub const CONF_TX_MAX_RATE_CLASSES: c_int = 10;
pub const CONF_TX_RATE_MASK_UNSPECIFIED: c_int = 0;

pub const CONF_TX_RATE_RETRY_LIMIT: c_int = 10;
// basic rates for p2p operations (probe req/resp, etc.)

//
// Rates supported for data packets when operating as STA/AP. Note the absence
// of the 22Mbps rate. There is a FW limitation on 12 rates so we must drop
// one. The rate dropped is not mandatory under any operating mode.
//

//
// Default rates for management traffic when operating in AP mode. This
// should be configured according to the basic rate set of the AP
//

// default rates for working as IBSS (11b and OFDM)

    pub CONF_TX_OFDM_RATES): CONF_HW_BIT_RATE_11MBPS |,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_tx_rate_class {
//
// The rates enabled for this rate class.
//
// Range: CONF_HW_BIT_RATE_* bit mask
//
    pub enabled_rates: u32,
//
// The dot11 short retry limit used for TX retries.
//
// Range: u8
//
    pub short_retry_limit: u8,
//
// The dot11 long retry limit used for TX retries.
//
// Range: u8
//
    pub long_retry_limit: u8,
//
// Flags controlling the attributes of TX transmission.
//
// Range: bit 0: Truncate - when set, FW attempts to send a frame stop
// when the total valid per-rate attempts have
// been exhausted; otherwise transmissions
// will continue at the lowest available rate
// until the appropriate one of the
// short_retry_limit, long_retry_limit,
// dot11_max_transmit_msdu_life_time, or
// max_tx_life_time, is exhausted.
// 1: Preamble Override - indicates if the preamble type
// should be used in TX.
// 2: Preamble Type - the type of the preamble to be used by
// the policy (0 - long preamble, 1 - short preamble.
//
    pub aflags: u8,
    pub __packed: },
pub const CONF_TX_MAX_AC_COUNT: c_int = 4;
// Slot number setting to start transmission at PIFS interval
pub const CONF_TX_AIFS_PIFS: c_int = 1;
// Slot number setting to start transmission at DIFS interval normal
// DCF access
pub const CONF_TX_AIFS_DIFS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conf_tx_ac {
    CONF_TX_AC_BE = 0,         /* best effort / legacy */
    CONF_TX_AC_BK = 1,         /* background */
    CONF_TX_AC_VI = 2,         /* video */
    CONF_TX_AC_VO = 3,         /* voice */
    CONF_TX_AC_CTS2SELF = 4,   /* fictitious AC, follows AC_VO */
    CONF_TX_AC_ANY_TID = 0xff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_tx_ac_category {
//
// The AC class identifier.
//
// Range: enum conf_tx_ac
//
    pub ac: u8,
//
// The contention window minimum size (in slots) for the access
// class.
//
// Range: u8
//
    pub cw_min: u8,
//
// The contention window maximum size (in slots) for the access
// class.
//
// Range: u8
//
    pub cw_max: u16,
//
// The AIF value (in slots) for the access class.
//
// Range: u8
//
    pub aifsn: u8,
//
// The TX Op Limit (in microseconds) for the access class.
//
// Range: u16
//
    pub tx_op_limit: u16,
    pub __packed: },
pub const CONF_TX_MAX_TID_COUNT: c_int = 8;
// Allow TX BA on all TIDs but 6,7. These are currently reserved in the FW
pub const CONF_TX_BA_ENABLED_TID_BITMAP: c_uint = 0x3F;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_tx_tid {
    pub queue_id: u8,
    pub channel_type: u8,
    pub tsid: u8,
    pub ps_scheme: u8,
    pub ack_policy: u8,
    pub apsd_conf: [u32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_tx_settings {
//
// The TX ED value for TELEC Enable/Disable.
//
// Range: 0, 1
//
    pub tx_energy_detection: u8,
//
// Configuration for rate classes for TX (currently only one
// rate class supported). Used in non-AP mode.
//
    pub sta_rc_conf: conf_tx_rate_class,
//
// Configuration for access categories for TX rate control.
//
    pub ac_conf_count: u8,
    pub ac_conf: [conf_tx_ac_category; CONF_TX_MAX_AC_COUNT],
//
// AP-mode - allow this number of TX retries to a station before an
// event is triggered from FW.
// In AP-mode the hlids of unreachable stations are given in the
// "sta_tx_retry_exceeded" member in the event mailbox.
//
    pub max_tx_retries: u8,
//
// AP-mode - after this number of seconds a connected station is
// considered inactive.
//
    pub ap_aging_period: u16,
//
// Configuration for TID parameters.
//
    pub tid_conf_count: u8,
    pub tid_conf: [conf_tx_tid; CONF_TX_MAX_TID_COUNT],
//
// The TX fragmentation threshold.
//
// Range: u16
//
    pub frag_threshold: u16,
//
// Max time in msec the FW may delay frame TX-Complete interrupt.
//
// Range: u16
//
    pub tx_compl_timeout: u16,
//
// Completed TX packet count which requires to issue the TX-Complete
// interrupt.
//
// Range: u16
//
    pub tx_compl_threshold: u16,
//
// The rate used for control messages and scanning on the 2.4GHz band
//
// Range: CONF_HW_BIT_RATE_* bit mask
//
    pub basic_rate: u32,
//
// The rate used for control messages and scanning on the 5GHz band
//
// Range: CONF_HW_BIT_RATE_* bit mask
//
    pub basic_rate_5: u32,
//
// TX retry limits for templates
//
    pub tmpl_short_retry_limit: u8,
    pub tmpl_long_retry_limit: u8,
// Time in ms for Tx watchdog timer to expire
    pub tx_watchdog_timeout: u32,
//
// when a slow link has this much packets pending, it becomes a low
// priority link, scheduling-wise
//
    pub slow_link_thold: u8,
//
// when a fast link has this much packets pending, it becomes a low
// priority link, scheduling-wise
//
    pub fast_link_thold: u8,
    pub __packed: },
}

pub const CONF_MAX_BCN_FILT_IE_COUNT: c_int = 32;

pub const CONF_BCN_IE_OUI_LEN: c_int = 3;
pub const CONF_BCN_IE_VER_LEN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_bcn_filt_rule {
//
// IE number to which to associate a rule.
//
// Range: u8
//
    pub ie: u8,
//
// Rule to associate with the specific ie.
//
// Range: CONF_BCN_RULE_PASS_ON_
//
    pub rule: u8,
//
// OUI for the vendor specifie IE (221)
//
    pub oui: [u8; CONF_BCN_IE_OUI_LEN],
//
// Type for the vendor specifie IE (221)
//
    pub type: u8,
//
// Version for the vendor specifie IE (221)
//
    pub version: [u8; CONF_BCN_IE_VER_LEN],
    pub __packed: },
pub const CONF_MAX_RSSI_SNR_TRIGGERS: c_int = 8;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_sig_weights {
//
// RSSI from beacons average weight.
//
// Range: u8
//
    pub rssi_bcn_avg_weight: u8,
//
// RSSI from data average weight.
//
// Range: u8
//
    pub rssi_pkt_avg_weight: u8,
//
// SNR from beacons average weight.
//
// Range: u8
//
    pub snr_bcn_avg_weight: u8,
//
// SNR from data average weight.
//
// Range: u8
//
    pub snr_pkt_avg_weight: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conf_bcn_filt_mode {
    CONF_BCN_FILT_MODE_DISABLED = 0,
    CONF_BCN_FILT_MODE_ENABLED = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conf_bet_mode {
    CONF_BET_MODE_DISABLE = 0,
    CONF_BET_MODE_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_conn_settings {
//
// Firmware wakeup conditions configuration. The host may set only
// one bit.
//
// Range: CONF_WAKE_UP_EVENT_
//
    pub wake_up_event: u8,
//
// Listen interval for beacons or Dtims.
//
// Range: 0 for beacon and Dtim wakeup
// 1-10 for x Dtims
// 1-255 for x beacons
//
    pub listen_interval: u8,
//
// Firmware wakeup conditions during suspend
// Range: CONF_WAKE_UP_EVENT_
//
    pub suspend_wake_up_event: u8,
//
// Listen interval during suspend.
// Currently will be in DTIMs (1-10)
//
    pub suspend_listen_interval: u8,
//
// Enable or disable the beacon filtering.
//
// Range: CONF_BCN_FILT_MODE_
//
    pub bcn_filt_mode: u8,
//
// Configure Beacon filter pass-thru rules.
//
    pub bcn_filt_ie_count: u8,
    pub bcn_filt_ie: [conf_bcn_filt_rule; CONF_MAX_BCN_FILT_IE_COUNT],
//
// The number of consecutive beacons to lose, before the firmware
// becomes out of synch.
//
// Range: u32
//
    pub synch_fail_thold: u32,
//
// After out-of-synch, the number of TU's to wait without a further
// received beacon (or probe response) before issuing the BSS_EVENT_LOSE
// event.
//
// Range: u32
//
    pub bss_lose_timeout: u32,
//
// Beacon receive timeout.
//
// Range: u32
//
    pub beacon_rx_timeout: u32,
//
// Broadcast receive timeout.
//
// Range: u32
//
    pub broadcast_timeout: u32,
//
// Enable/disable reception of broadcast packets in power save mode
//
// Range: 1 - enable, 0 - disable
//
    pub rx_broadcast_in_ps: u8,
//
// Consecutive PS Poll failures before sending event to driver
//
// Range: u8
//
    pub ps_poll_threshold: u8,
//
// Configuration of signal average weights.
//
    pub sig_weights: conf_sig_weights,
//
// Specifies if beacon early termination procedure is enabled or
// disabled.
//
// Range: CONF_BET_MODE_
//
    pub bet_enable: u8,
//
// Specifies the maximum number of consecutive beacons that may be
// early terminated. After this number is reached at least one full
// beacon must be correctly received in FW before beacon ET
// resumes.
//
// Range 0 - 255
//
    pub bet_max_consecutive: u8,
//
// Specifies the maximum number of times to try PSM entry if it fails
// (if sending the appropriate null-func message fails.)
//
// Range 0 - 255
//
    pub psm_entry_retries: u8,
//
// Specifies the maximum number of times to try PSM exit if it fails
// (if sending the appropriate null-func message fails.)
//
// Range 0 - 255
//
    pub psm_exit_retries: u8,
//
// Specifies the maximum number of times to try transmit the PSM entry
// null-func frame for each PSM entry attempt
//
// Range 0 - 255
//
    pub psm_entry_nullfunc_retries: u8,
//
// Specifies the dynamic PS timeout in ms that will be used
// by the FW when in AUTO_PS mode
//
    pub dynamic_ps_timeout: u16,
//
// Specifies whether dynamic PS should be disabled and PSM forced.
// This is required for certain WiFi certification tests.
//
    pub forced_ps: u8,
//
// Specifies the interval of the connection keep-alive null-func
// frame in ms.
//
// Range: 1000 - 3600000
//
    pub keep_alive_interval: u32,
//
// Maximum listen interval supported by the driver in units of beacons.
//
// Range: u16
//
    pub max_listen_interval: u8,
//
// Default sleep authorization for a new STA interface. This determines
// whether we can go to ELP.
//
    pub sta_sleep_auth: u8,
//
// Default RX BA Activity filter configuration
//
    pub suspend_rx_ba_activity: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum single_dual_band_enum {
    CONF_SINGLE_BAND,
    CONF_DUAL_BAND
}

pub const CONF_RSSI_AND_PROCESS_COMPENSATION_SIZE: c_int = 15;
pub const CONF_NUMBER_OF_SUB_BANDS_5: c_int = 7;
pub const CONF_NUMBER_OF_RATE_GROUPS: c_int = 6;
pub const CONF_NUMBER_OF_CHANNELS_2_4: c_int = 14;
pub const CONF_NUMBER_OF_CHANNELS_5: c_int = 35;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_itrim_settings {
// enable dco itrim
    pub enable: u8,
// moderation timeout in microsecs from the last TX
    pub timeout: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conf_fast_wakeup {
    CONF_FAST_WAKEUP_ENABLE,
    CONF_FAST_WAKEUP_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_pm_config_settings {
//
// Host clock settling time
//
// Range: 0 - 30000 us
//
    pub host_clk_settling_time: u32,
//
// Host fast wakeup support
//
// Range: enum conf_fast_wakeup
//
    pub host_fast_wakeup_support: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_roam_trigger_settings {
//
// The minimum interval between two trigger events.
//
// Range: 0 - 60000 ms
//
    pub trigger_pacing: u16,
//
// The weight for rssi/beacon average calculation
//
// Range: 0 - 255
//
    pub avg_weight_rssi_beacon: u8,
//
// The weight for rssi/data frame average calculation
//
// Range: 0 - 255
//
    pub avg_weight_rssi_data: u8,
//
// The weight for snr/beacon average calculation
//
// Range: 0 - 255
//
    pub avg_weight_snr_beacon: u8,
//
// The weight for snr/data frame average calculation
//
// Range: 0 - 255
//
    pub avg_weight_snr_data: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_scan_settings {
//
// The minimum time to wait on each channel for active scans
// This value will be used whenever there's a connected interface.
//
// Range: u32 tu/1000
//
    pub min_dwell_time_active: u32,
//
// The maximum time to wait on each channel for active scans
// This value will be currently used whenever there's a
// connected interface. It shouldn't exceed 30000 (~30ms) to avoid
// possible interference of voip traffic going on while scanning.
//
// Range: u32 tu/1000
//
    pub max_dwell_time_active: u32,
// The minimum time to wait on each channel for active scans
// when it's possible to have longer scan dwell times.
// Currently this is used whenever we're idle on all interfaces.
// Longer dwell times improve detection of networks within a
// single scan.
//
// Range: u32 tu/1000
//
    pub min_dwell_time_active_long: u32,
// The maximum time to wait on each channel for active scans
// when it's possible to have longer scan dwell times.
// See min_dwell_time_active_long
//
// Range: u32 tu/1000
//
    pub max_dwell_time_active_long: u32,
// time to wait on the channel for passive scans (in TU/1000)
    pub dwell_time_passive: u32,
// time to wait on the channel for DFS scans (in TU/1000)
    pub dwell_time_dfs: u32,
//
// Number of probe requests to transmit on each active scan channel
//
// Range: u8
//
    pub num_probe_reqs: u16,
//
// Scan trigger (split scan) timeout. The FW will split the scan
// operation into slices of the given time and allow the FW to schedule
// other tasks in between.
//
// Range: u32 Microsecs
//
    pub split_scan_timeout: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_sched_scan_settings {
//
// The base time to wait on the channel for active scans (in TU/1000).
// The minimum dwell time is calculated according to this:
// min_dwell_time = base + num_of_probes_to_be_sent * delta_per_probe
// The maximum dwell time is calculated according to this:
// max_dwell_time = min_dwell_time + max_dwell_time_delta
//
    pub base_dwell_time: u32,
// The delta between the min dwell time and max dwell time for
// active scans (in TU/1000s). The max dwell time is used by the FW once
// traffic is detected on the channel.
//
    pub max_dwell_time_delta: u32,
// Delta added to min dwell time per each probe in 2.4 GHz (TU/1000)
    pub dwell_time_delta_per_probe: u32,
// Delta added to min dwell time per each probe in 5 GHz (TU/1000)
    pub dwell_time_delta_per_probe_5: u32,
// time to wait on the channel for passive scans (in TU/1000)
    pub dwell_time_passive: u32,
// time to wait on the channel for DFS scans (in TU/1000)
    pub dwell_time_dfs: u32,
// number of probe requests to send on each channel in active scans
    pub num_probe_reqs: u8,
// RSSI threshold to be used for filtering
    pub rssi_threshold: i8,
// SNR threshold to be used for filtering
    pub snr_threshold: i8,
//
// number of short intervals scheduled scan cycles before
// switching to long intervals
//
    pub num_short_intervals: u8,
// interval between each long scheduled scan cycle (in ms)
    pub long_interval: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_ht_setting {
    pub rx_ba_win_size: u8,
    pub tx_ba_win_size: u8,
    pub inactivity_timeout: u16,
// bitmap of enabled TIDs for TX BA sessions
    pub tx_ba_tid_bitmap: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_memory_settings {
// Number of stations supported in IBSS mode
    pub num_stations: u8,
// Number of ssid profiles used in IBSS mode
    pub ssid_profiles: u8,
// Number of memory buffers allocated to rx pool
    pub rx_block_num: u8,
// Minimum number of blocks allocated to tx pool
    pub tx_min_block_num: u8,
// Disable/Enable dynamic memory
    pub dynamic_memory: u8,
//
// Minimum required free tx memory blocks in order to assure optimum
// performance
//
// Range: 0-120
//
    pub min_req_tx_blocks: u8,
//
// Minimum required free rx memory blocks in order to assure optimum
// performance
//
// Range: 0-120
//
    pub min_req_rx_blocks: u8,
//
// Minimum number of mem blocks (free+used) guaranteed for TX
//
// Range: 0-120
//
    pub tx_min: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_fm_coex {
    pub enable: u8,
    pub swallow_period: u8,
    pub n_divider_fref_set_1: u8,
    pub n_divider_fref_set_2: u8,
    pub m_divider_fref_set_1: u16,
    pub m_divider_fref_set_2: u16,
    pub coex_pll_stabilization_time: u32,
    pub ldo_stabilization_time: u16,
    pub fm_disturbed_band_margin: u8,
    pub swallow_clk_diff: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_rx_streaming_settings {
//
// RX Streaming duration (in msec) from last tx/rx
//
// Range: u32
//
    pub duration: u32,
//
// Bitmap of tids to be polled during RX streaming.
// (Note: it doesn't look like it really matters)
//
// Range: 0x1-0xff
//
    pub queues: u8,
//
// RX Streaming interval.
// (Note:this value is also used as the rx streaming timeout)
// Range: 0 (disabled), 10 - 100
//
    pub interval: u8,
//
// enable rx streaming also when there is no coex activity
//
    pub always: u8,
    pub __packed: },
pub const CONF_FWLOG_MIN_MEM_BLOCKS: c_int = 2;
pub const CONF_FWLOG_MAX_MEM_BLOCKS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_fwlog {
// Continuous or on-demand
    pub mode: u8,
//
// Number of memory blocks dedicated for the FW logger
//
// Range: 2-16, or 0 to disable the FW logger
//
    pub mem_blocks: u8,
// Minimum log level threshold
    pub severity: u8,
// Include/exclude timestamps from the log messages
    pub timestamp: u8,
// See enum wl1271_fwlogger_output
    pub output: u8,
// Regulates the frequency of log messages
    pub threshold: u8,
    pub __packed: },
pub const ACX_RATE_MGMT_NUM_OF_RATES: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_rate_policy_settings {
    pub rate_retry_score: u16,
    pub per_add: u16,
    pub per_th1: u16,
    pub per_th2: u16,
    pub max_per: u16,
    pub inverse_curiosity_factor: u8,
    pub tx_fail_low_th: u8,
    pub tx_fail_high_th: u8,
    pub per_alpha_shift: u8,
    pub per_add_shift: u8,
    pub per_beta1_shift: u8,
    pub per_beta2_shift: u8,
    pub rate_check_up: u8,
    pub rate_check_down: u8,
    pub rate_retry_policy: [u8; ACX_RATE_MGMT_NUM_OF_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_hangover_settings {
    pub recover_time: u32,
    pub hangover_period: u8,
    pub dynamic_mode: u8,
    pub early_termination_mode: u8,
    pub max_period: u8,
    pub min_period: u8,
    pub increase_delta: u8,
    pub decrease_delta: u8,
    pub quiet_time: u8,
    pub increase_time: u8,
    pub window_size: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_recovery_settings {
// BUG() on fw recovery
    pub bug_on_recovery: u8,
// Prevent HW recovery. FW will remain stuck.
    pub no_recovery: u8,
    pub __packed: },
//
// The conf version consists of 4 bytes.  The two MSB are the wlcore
// version, the two LSB are the lower driver's private conf
// version.
//

pub const WLCORE_CONF_MASK: c_uint = 0xffff0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_conf_header {
    pub magic: __le32,
    pub version: __le32,
    pub checksum: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_conf {
    pub sg: conf_sg_settings,
    pub rx: conf_rx_settings,
    pub tx: conf_tx_settings,
    pub conn: conf_conn_settings,
    pub itrim: conf_itrim_settings,
    pub pm_config: conf_pm_config_settings,
    pub roam_trigger: conf_roam_trigger_settings,
    pub scan: conf_scan_settings,
    pub sched_scan: conf_sched_scan_settings,
    pub ht: conf_ht_setting,
    pub mem: conf_memory_settings,
    pub fm_coex: conf_fm_coex,
    pub rx_streaming: conf_rx_streaming_settings,
    pub fwlog: conf_fwlog,
    pub rate: conf_rate_policy_settings,
    pub hangover: conf_hangover_settings,
    pub recovery: conf_recovery_settings,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_conf_file {
    pub header: wlcore_conf_header,
    pub core: wlcore_conf,
    pub priv: [u8; ],
    pub __packed: },
