//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/hw.h
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

pub const ATHEROS_VENDOR_ID: c_uint = 0x168c;
pub const AR5416_DEVID_PCI: c_uint = 0x0023;
pub const AR5416_DEVID_PCIE: c_uint = 0x0024;
pub const AR9160_DEVID_PCI: c_uint = 0x0027;
pub const AR9280_DEVID_PCI: c_uint = 0x0029;
pub const AR9280_DEVID_PCIE: c_uint = 0x002a;
pub const AR9285_DEVID_PCIE: c_uint = 0x002b;
pub const AR2427_DEVID_PCIE: c_uint = 0x002c;
pub const AR9287_DEVID_PCI: c_uint = 0x002d;
pub const AR9287_DEVID_PCIE: c_uint = 0x002e;
pub const AR9300_DEVID_PCIE: c_uint = 0x0030;
pub const AR9300_DEVID_AR9340: c_uint = 0x0031;
pub const AR9300_DEVID_AR9485_PCIE: c_uint = 0x0032;
pub const AR9300_DEVID_AR9580: c_uint = 0x0033;
pub const AR9300_DEVID_AR9462: c_uint = 0x0034;
pub const AR9300_DEVID_AR9330: c_uint = 0x0035;
pub const AR9300_DEVID_QCA955X: c_uint = 0x0038;
pub const AR9485_DEVID_AR1111: c_uint = 0x0037;
pub const AR9300_DEVID_AR9565: c_uint = 0x0036;
pub const AR9300_DEVID_AR953X: c_uint = 0x003d;
pub const AR9300_DEVID_QCA956X: c_uint = 0x003f;
pub const AR5416_AR9100_DEVID: c_uint = 0x000b;
pub const AR_SUBVENDOR_ID_NOG: c_uint = 0x0e11;
pub const AR_SUBVENDOR_ID_NEW_A: c_uint = 0x7065;
pub const AR5416_MAGIC: c_uint = 0x19641014;
pub const AR9280_COEX2WIRE_SUBSYSID: c_uint = 0x309b;
pub const AT9285_COEX3WIRE_SA_SUBSYSID: c_uint = 0x30aa;
pub const AT9285_COEX3WIRE_DA_SUBSYSID: c_uint = 0x30ab;

pub const ATH9K_NUM_CHANNELS: c_int = 38;
// Register read/write primitives

pub const AR_GPIO_OUTPUT_MUX_AS_OUTPUT: c_int = 0;
pub const AR_GPIO_OUTPUT_MUX_AS_PCIE_ATTENTION_LED: c_int = 1;
pub const AR_GPIO_OUTPUT_MUX_AS_PCIE_POWER_LED: c_int = 2;
pub const AR_GPIO_OUTPUT_MUX_AS_TX_FRAME: c_int = 3;
pub const AR_GPIO_OUTPUT_MUX_AS_RX_CLEAR_EXTERNAL: c_int = 4;
pub const AR_GPIO_OUTPUT_MUX_AS_MAC_NETWORK_LED: c_int = 5;
pub const AR_GPIO_OUTPUT_MUX_AS_MAC_POWER_LED: c_int = 6;
pub const AR_GPIO_OUTPUT_MUX_AS_MCI_WLAN_DATA: c_uint = 0x16;
pub const AR_GPIO_OUTPUT_MUX_AS_MCI_WLAN_CLK: c_uint = 0x17;
pub const AR_GPIO_OUTPUT_MUX_AS_MCI_BT_DATA: c_uint = 0x18;
pub const AR_GPIO_OUTPUT_MUX_AS_MCI_BT_CLK: c_uint = 0x19;
pub const AR_GPIO_OUTPUT_MUX_AS_WL_IN_TX: c_uint = 0x14;
pub const AR_GPIO_OUTPUT_MUX_AS_WL_IN_RX: c_uint = 0x13;
pub const AR_GPIO_OUTPUT_MUX_AS_BT_IN_TX: c_int = 9;
pub const AR_GPIO_OUTPUT_MUX_AS_BT_IN_RX: c_int = 8;
pub const AR_GPIO_OUTPUT_MUX_AS_RUCKUS_STROBE: c_uint = 0x1d;
pub const AR_GPIO_OUTPUT_MUX_AS_RUCKUS_DATA: c_uint = 0x1e;
pub const AR_GPIOD_MASK: c_uint = 0x00001FFF;
pub const BASE_ACTIVATE_DELAY: c_int = 100;

pub const COEF_SCALE_S: c_int = 24;
pub const HT40_CHANNEL_CENTER_SHIFT: c_int = 10;
pub const ATH9K_ANTENNA0_CHAINMASK: c_uint = 0x1;
pub const ATH9K_ANTENNA1_CHAINMASK: c_uint = 0x2;
pub const ATH9K_NUM_DMA_DEBUG_REGS: c_int = 8;
pub const ATH9K_NUM_QUEUES: c_int = 10;
pub const MAX_RATE_POWER: c_int = 63;

pub const AH_TIME_QUANTUM: c_int = 10;
pub const AR_KEYTABLE_SIZE: c_int = 128;
pub const POWER_UP_TIME: c_int = 10000;
pub const SPUR_RSSI_THRESH: c_int = 40;
pub const UPPER_5G_SUB_BAND_START: c_int = 5700;
pub const MID_5G_SUB_BAND_START: c_int = 5400;
pub const CAB_TIMEOUT_VAL: c_int = 10;
pub const BEACON_TIMEOUT_VAL: c_int = 10;
pub const MIN_BEACON_TIMEOUT_VAL: c_int = 1;

pub const INIT_CONFIG_STATUS: c_uint = 0x00000000;
pub const INIT_RSSI_THR: c_uint = 0x00000700;
pub const INIT_BCON_CNTRL_REG: c_uint = 0x00000000;

pub const ATH9K_HW_RX_HP_QDEPTH: c_int = 16;
pub const ATH9K_HW_RX_LP_QDEPTH: c_int = 128;
pub const PAPRD_GAIN_TABLE_ENTRIES: c_int = 32;
pub const PAPRD_TABLE_SZ: c_int = 24;
pub const PAPRD_IDEAL_AGC2_PWR_RANGE: c_uint = 0xe0;
//
// Wake on Wireless
//
// Keep Alive Frame
pub const KAL_FRAME_LEN: c_int = 28;
pub const KAL_FRAME_TYPE: c_uint = 0x2	/* data frame */;
pub const KAL_FRAME_SUB_TYPE: c_uint = 0x4	/* null data frame */;
pub const KAL_DURATION_ID: c_uint = 0x3d;
pub const KAL_NUM_DATA_WORDS: c_int = 6;
pub const KAL_NUM_DESC_WORDS: c_int = 12;
pub const KAL_ANTENNA_MODE: c_int = 1;
pub const KAL_TO_DS: c_int = 1;

pub const KAL_TIMEOUT: c_int = 900;
pub const MAX_PATTERN_SIZE: c_int = 256;
pub const MAX_PATTERN_MASK_SIZE: c_int = 32;
pub const MAX_NUM_PATTERN: c_int = 16;
pub const MAX_NUM_PATTERN_LEGACY: c_int = 8;

//
// WoW trigger mapping to hardware code
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_hw_txq_subtype {
    ATH_TXQ_AC_BK = 0,
    ATH_TXQ_AC_BE = 1,
    ATH_TXQ_AC_VI = 2,
    ATH_TXQ_AC_VO = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_ini_subsys {
    ATH_INI_PRE = 0,
    ATH_INI_CORE,
    ATH_INI_POST,
    ATH_INI_NUM_SPLIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_hw_caps {
    ATH9K_HW_CAP_HT                         = BIT(0),
    ATH9K_HW_CAP_RFSILENT                   = BIT(1),
    ATH9K_HW_CAP_AUTOSLEEP                  = BIT(2),
    ATH9K_HW_CAP_4KB_SPLITTRANS             = BIT(3),
    ATH9K_HW_CAP_EDMA			= BIT(4),
    ATH9K_HW_CAP_RAC_SUPPORTED		= BIT(5),
    ATH9K_HW_CAP_LDPC			= BIT(6),
    ATH9K_HW_CAP_FASTCLOCK			= BIT(7),
    ATH9K_HW_CAP_SGI_20			= BIT(8),
    ATH9K_HW_CAP_ANT_DIV_COMB		= BIT(10),
    ATH9K_HW_CAP_2GHZ			= BIT(11),
    ATH9K_HW_CAP_5GHZ			= BIT(12),
    ATH9K_HW_CAP_APM			= BIT(13),

    ATH9K_HW_CAP_RTT			= BIT(14),
    ATH9K_HW_CAP_MCI			= BIT(15),
    ATH9K_HW_CAP_BT_ANT_DIV			= BIT(17),

    ATH9K_HW_CAP_RTT			= 0,
    ATH9K_HW_CAP_MCI			= 0,
    ATH9K_HW_CAP_BT_ANT_DIV			= 0,

    ATH9K_HW_CAP_DFS			= BIT(18),
    ATH9K_HW_CAP_PAPRD			= BIT(19),
    ATH9K_HW_CAP_FCC_BAND_SWITCH		= BIT(20),
}

//
// WoW device capabilities
// @ATH9K_HW_WOW_DEVICE_CAPABLE: device revision is capable of WoW.
// @ATH9K_HW_WOW_PATTERN_MATCH_EXACT: device is capable of matching
// an exact user defined pattern or de-authentication/disassoc pattern.
// @ATH9K_HW_WOW_PATTERN_MATCH_DWORD: device requires the first four
// bytes of the pattern for user defined pattern, de-authentication and
// disassociation patterns for all types of possible frames received
// of those types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_hw_wow {
    pub wow_event_mask: u32,
    pub wow_event_mask2: u32,
    pub max_patterns: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_hw_capabilities {
    pub /: *mut *mut *mut u32 hw_caps; / ATH9K_HW_CAP_ from ath9k_hw_caps,
    pub rts_aggr_limit: u16,
    pub tx_chainmask: u8,
    pub rx_chainmask: u8,
    pub chip_chainmask: u8,
    pub max_txchains: u8,
    pub max_rxchains: u8,
    pub num_gpio_pins: u8,
    pub gpio_mask: u32,
    pub rx_hp_qdepth: u8,
    pub rx_lp_qdepth: u8,
    pub rx_status_len: u8,
    pub tx_desc_len: u8,
    pub txs_len: u8,
}

pub const AR_NO_SPUR: c_uint = 0x8000;
pub const AR_BASE_FREQ_2GHZ: c_int = 2300;
pub const AR_BASE_FREQ_5GHZ: c_int = 4900;
pub const AR_SPUR_FEEQ_BOUND_HT40: c_int = 19;
pub const AR_SPUR_FEEQ_BOUND_HT20: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_hw_hang_checks {
    HW_BB_WATCHDOG            = BIT(0),
    HW_PHYRESTART_CLC_WAR     = BIT(1),
    HW_BB_RIFS_HANG           = BIT(2),
    HW_BB_DFS_HANG            = BIT(3),
    HW_BB_RX_CLEAR_STUCK_HANG = BIT(4),
    HW_MAC_HANG               = BIT(5),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_ops_config {
    pub dma_beacon_response_time: c_int,
    pub sw_beacon_response_time: c_int,
    pub cwm_ignore_extcca: bool,
    pub pcie_waen: u32,
    pub analog_shiftreg: u8,
    pub ofdm_trig_low: u32,
    pub ofdm_trig_high: u32,
    pub cck_trig_high: u32,
    pub cck_trig_low: u32,
    pub enable_paprd: bool,
    pub serialize_regmode: c_int,
    pub rx_intr_mitigation: bool,
    pub tx_intr_mitigation: bool,
    pub max_txtrig_level: u8,
    pub /: *mut *mut u16 ani_poll_interval; / ANI poll interval in ms,
    pub hw_hang_checks: u16,
    pub rimt_first: u16,
    pub rimt_last: u16,
// Platform specific config
    pub aspm_l1_fix: u32,
    pub xlna_gpio: u32,
    pub ant_ctrl_comm2g_switch_enable: u32,
    pub xatten_margin_cfg: bool,
    pub alt_mingainidx: bool,
    pub pll_pwrsave: u8,
    pub tx_gain_buffalo: bool,
    pub led_active_high: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_int {
    ATH9K_INT_RX = 0x00000001,
    ATH9K_INT_RXDESC = 0x00000002,
    ATH9K_INT_RXHP = 0x00000001,
    ATH9K_INT_RXLP = 0x00000002,
    ATH9K_INT_RXNOFRM = 0x00000008,
    ATH9K_INT_RXEOL = 0x00000010,
    ATH9K_INT_RXORN = 0x00000020,
    ATH9K_INT_TX = 0x00000040,
    ATH9K_INT_TXDESC = 0x00000080,
    ATH9K_INT_TIM_TIMER = 0x00000100,
    ATH9K_INT_MCI = 0x00000200,
    ATH9K_INT_BB_WATCHDOG = 0x00000400,
    ATH9K_INT_TXURN = 0x00000800,
    ATH9K_INT_MIB = 0x00001000,
    ATH9K_INT_RXPHY = 0x00004000,
    ATH9K_INT_RXKCM = 0x00008000,
    ATH9K_INT_SWBA = 0x00010000,
    ATH9K_INT_BMISS = 0x00040000,
    ATH9K_INT_BNR = 0x00100000,
    ATH9K_INT_TIM = 0x00200000,
    ATH9K_INT_DTIM = 0x00400000,
    ATH9K_INT_DTIMSYNC = 0x00800000,
    ATH9K_INT_GPIO = 0x01000000,
    ATH9K_INT_CABEND = 0x02000000,
    ATH9K_INT_TSFOOR = 0x04000000,
    ATH9K_INT_GENTIMER = 0x08000000,
    ATH9K_INT_CST = 0x10000000,
    ATH9K_INT_GTT = 0x20000000,
    ATH9K_INT_FATAL = 0x40000000,
    ATH9K_INT_GLOBAL = 0x80000000,
    ATH9K_INT_BMISC = ATH9K_INT_TIM |
    ATH9K_INT_DTIM |
    ATH9K_INT_DTIMSYNC |
    ATH9K_INT_TSFOOR |
    ATH9K_INT_CABEND,
    ATH9K_INT_COMMON = ATH9K_INT_RXNOFRM |
    ATH9K_INT_RXDESC |
    ATH9K_INT_RXEOL |
    ATH9K_INT_RXORN |
    ATH9K_INT_TXURN |
    ATH9K_INT_TXDESC |
    ATH9K_INT_MIB |
    ATH9K_INT_RXPHY |
    ATH9K_INT_RXKCM |
    ATH9K_INT_SWBA |
    ATH9K_INT_BMISS |
    ATH9K_INT_GPIO,
    ATH9K_INT_NOCARD = 0xffffffff
}

pub const MAX_RTT_TABLE_ENTRY: c_int = 6;
pub const MAX_IQCAL_MEASUREMENT: c_int = 8;
pub const MAX_CL_TAB_ENTRY: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_cal_flags {
    RTT_DONE,
    PAPRD_PACKET_SENT,
    PAPRD_DONE,
    NFCAL_PENDING,
    NFCAL_INTF,
    TXIQCAL_DONE,
    TXCLCAL_DONE,
    SW_PKDET_DONE,
    LONGCAL_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_hw_cal_data {
    pub channel: u16,
    pub channelFlags: u16,
    pub cal_flags: c_ulong,
    pub CalValid: i32,
    pub iCoff: i8,
    pub qCoff: i8,
    pub caldac: [u8; 2],
    pub small_signal_gain: [u16; AR9300_MAX_CHAINS],
    pub pa_table: [u32; AR9300_MAX_CHAINS][PAPRD_TABLE_SZ],
    pub num_measures: [u32; AR9300_MAX_CHAINS],
    pub tx_corr_coeff: [c_int; MAX_IQCAL_MEASUREMENT][AR9300_MAX_CHAINS],
    pub tx_clcal: [u32; AR9300_MAX_CHAINS][MAX_CL_TAB_ENTRY],
    pub rtt_table: [u32; AR9300_MAX_CHAINS][MAX_RTT_TABLE_ENTRY],
    pub nfCalHist: [ath9k_nfcal_hist; NUM_NF_READINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_channel {
    pub chan: *mut ieee80211_channel,
    pub channel: u16,
    pub channelFlags: u16,
    pub noisefloor: i16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_power_mode {
    ATH9K_PM_AWAKE = 0,
    ATH9K_PM_FULL_SLEEP,
    ATH9K_PM_NETWORK_SLEEP,
    ATH9K_PM_UNDEFINED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ser_reg_mode {
    SER_REG_MODE_OFF = 0,
    SER_REG_MODE_ON = 1,
    SER_REG_MODE_AUTO = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_rx_qtype {
    ATH9K_RX_QUEUE_HP,
    ATH9K_RX_QUEUE_LP,
    ATH9K_RX_QUEUE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_beacon_state {
    pub bs_nexttbtt: u32,
    pub bs_nextdtim: u32,
    pub bs_intval: u32,
pub const ATH9K_TSFOOR_THRESHOLD: c_uint = 0x00004240 /* 16k us */;
    pub bs_dtimperiod: u32,
    pub bs_bmissthreshold: u16,
    pub bs_sleepduration: u32,
    pub bs_tsfoor_threshold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_centers {
    pub synth_center: u16,
    pub ctl_center: u16,
    pub ext_center: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_hw_version {
    pub magic: u32,
    pub devid: u16,
    pub subvendorid: u16,
    pub macVersion: u32,
    pub macRev: u16,
    pub phyRev: u16,
    pub analog5GhzRev: u16,
    pub analog2GhzRev: u16,
    pub usbdev: ath_usb_dev,
}

// Generic TSF timer definitions
pub const ATH_MAX_GEN_TIMER: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_gen_timer_configuration {
    pub next_addr: u32,
    pub period_addr: u32,
    pub mode_addr: u32,
    pub mode_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_gen_timer {
    pub arg): *mut *mut void (trigger)(void,
    pub arg): *mut *mut void (overflow)(void,
    pub arg: *mut c_void,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_gen_timer_table {
    pub timers: [*mut ath_gen_timer; ATH_MAX_GEN_TIMER],
    pub timer_mask: u16,
    pub tsf2_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_hw_antcomb_conf {
    pub main_lna_conf: u8,
    pub alt_lna_conf: u8,
    pub fast_div_bias: u8,
    pub main_gaintb: u8,
    pub alt_gaintb: u8,
    pub lna1_lna2_delta: c_int,
    pub lna1_lna2_switch_delta: c_int,
    pub div_group: u8,
}

//
// struct ath_hw_radar_conf - radar detection initialization parameters
//
// @pulse_inband: threshold for checking the ratio of in-band power
// to total power for short radar pulses (half dB steps)
// @pulse_inband_step: threshold for checking an in-band power to total
// power ratio increase for short radar pulses (half dB steps)
// @pulse_height: threshold for detecting the beginning of a short
// radar pulse (dB step)
// @pulse_rssi: threshold for detecting if a short radar pulse is
// gone (dB step)
// @pulse_maxlen: maximum pulse length (0.8 us steps)
//
// @radar_rssi: RSSI threshold for starting long radar detection (dB steps)
// @radar_inband: threshold for checking the ratio of in-band power
// to total power for long radar pulses (half dB steps)
// @fir_power: threshold for detecting the end of a long radar pulse (dB)
//
// @ext_channel: enable extension channel radar detection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_hw_radar_conf {
    pub pulse_inband: c_uint,
    pub pulse_inband_step: c_uint,
    pub pulse_height: c_uint,
    pub pulse_rssi: c_uint,
    pub pulse_maxlen: c_uint,
    pub radar_rssi: c_uint,
    pub radar_inband: c_uint,
    pub fir_power: c_int,
    pub ext_channel: bool,
}

//
// struct ath_hw_private_ops - callbacks used internally by hardware code
//
// This structure contains private callbacks designed to only be used internally
// by the hardware core.
//
// @init_cal_settings: setup types of calibrations supported
// @init_cal: starts actual calibration
//
// @init_mode_gain_regs: Initialize TX/RX gain registers
//
// @rf_set_freq: change frequency
// @spur_mitigate_freq: spur mitigation
// @set_rf_regs:
// @compute_pll_control: compute the PLL control value to use for
// AR_RTC_PLL_CONTROL for a given channel
// @setup_calibration: set up calibration
// @iscal_supported: used to query if a type of calibration is supported
//
// @ani_cache_ini_regs: cache the values for ANI from the initial
// register settings through the register initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_hw_private_ops {
    pub ah): *mut *mut void (init_hang_checks)(struct ath_hw,
    pub ah): *mut *mut bool (detect_mac_hang)(struct ath_hw,
    pub ah): *mut *mut bool (detect_bb_hang)(struct ath_hw,
// Calibration ops
    pub ah): *mut *mut void (init_cal_settings)(struct ath_hw,
    pub chan): *mut *mut *mut bool (init_cal)(struct ath_hw ah, struct ath9k_channel,
    pub ah): *mut *mut void (init_mode_gain_regs)(struct ath_hw,
    pub currCal): *mut ath9k_cal_list,
// PHY ops
    pub chan): *mut ath9k_channel,
    pub chan): *mut ath9k_channel,
    pub modesIndex): u16,
    pub chan): *mut *mut *mut void (set_channel_regs)(struct ath_hw ah, struct ath9k_channel,
    pub chan): *mut ath9k_channel,
    pub chan): *mut *mut *mut int (process_ini)(struct ath_hw ah, struct ath9k_channel,
    pub ah): *mut *mut void (olc_init)(struct ath_hw,
    pub chan): *mut *mut *mut void (set_rfmode)(struct ath_hw ah, struct ath9k_channel,
    pub ah): *mut *mut void (mark_phy_inactive)(struct ath_hw,
    pub chan): *mut *mut *mut void (set_delta_slope)(struct ath_hw ah, struct ath9k_channel,
    pub ah): *mut *mut bool (rfbus_req)(struct ath_hw,
    pub ah): *mut *mut void (rfbus_done)(struct ath_hw,
    pub ah): *mut *mut void (restore_chainmask)(struct ath_hw,
    pub chan): *mut ath9k_channel,
    pub param): c_int,
    pub nfarray[NUM_NF_READINGS]): *mut *mut *mut void (do_getnf)(struct ath_hw ah, int16_t,
    pub conf): *mut ath_hw_radar_conf,
    pub ini_reloaded): *mut u8,
// ANI
    pub ah): *mut *mut void (ani_cache_ini_regs)(struct ath_hw,

    pub ah): *mut *mut bool (is_aic_enabled)(struct ath_hw,

}

//
// struct ath_spec_scan - parameters for Atheros spectral scan
//
// @enabled: enable/disable spectral scan
// @short_repeat: controls whether the chip is in spectral scan mode
// for 4 usec (enabled) or 204 usec (disabled)
// @count: number of scan results requested. There are special meanings
// in some chip revisions:
// AR92xx: highest bit set (>=128) for endless mode
// (spectral scan won't stopped until explicitly disabled)
// AR9300 and newer: 0 for endless mode
// @endless: true if endless mode is intended. Otherwise, count value is
// corrected to the next possible value.
// @period: time duration between successive spectral scan entry points
// (period*256*Tclk). Tclk = ath_common->clockrate
// @fft_period: PHY passes FFT frames to MAC every (fft_period+1)*4uS
//
// Note: Tclk = 40MHz or 44MHz depending upon operating mode.
// Typically it's 44MHz in 2/5GHz on later chips, but there's
// a "fast clock" check for this in 5GHz.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_spec_scan {
    pub enabled: bool,
    pub short_repeat: bool,
    pub endless: bool,
    pub count: u8,
    pub period: u8,
    pub fft_period: u8,
}

//
// struct ath_hw_ops - callbacks used by hardware code and driver code
//
// This structure contains callbacks designed to be used internally by
// hardware code and also by the lower level driver.
//
// @config_pci_powersave:
// @calibrate: periodic calibration for NF, ANI, IQ, ADC gain, ADC-DC
//
// @spectral_scan_config: set parameters for spectral scan and enable/disable it
// @spectral_scan_trigger: trigger a spectral scan run
// @spectral_scan_wait: wait for a spectral scan run to finish
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_hw_ops {
    pub power_off): bool,
    pub ah): *mut *mut void (rx_enable)(struct ath_hw,
    pub link): *mut *mut *mut void (set_desc_link)(void ds, u32,
    pub longcal): u8 rxchainmask, bool,
    pub sync_cause_p): *mut u32,
    pub i): *mut ath_tx_info,
    pub ts): *mut ath_tx_status,
    pub index): *const *const *const *const int (get_duration)(struct ath_hw ah, void ds, int,
    pub antconf): *mut ath_hw_antcomb_conf,
    pub antconf): *mut ath_hw_antcomb_conf,
    pub param): *mut ath_spec_scan,
    pub ah): *mut *mut void (spectral_scan_trigger)(struct ath_hw,
    pub ah): *mut *mut void (spectral_scan_wait)(struct ath_hw,
    pub qnum): *mut *mut *mut void (tx99_start)(struct ath_hw ah, u32,
    pub ah): *mut *mut void (tx99_stop)(struct ath_hw,
    pub power): *mut *mut *mut void (tx99_set_txpower)(struct ath_hw ah, u8,

    pub enable): *mut *mut *mut void (set_bt_ant_diversity)(struct ath_hw hw, bool,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_nf_limits {
    pub max: i16,
    pub min: i16,
    pub nominal: i16,
    pub cal: [i16; AR5416_MAX_CHAINS],
    pub pwr: [i16; AR5416_MAX_CHAINS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_cal_list {
    TX_IQ_CAL         =	BIT(0),
    TX_IQ_ON_AGC_CAL  =	BIT(1),
    TX_CL_CAL         =	BIT(2),
}

// ah_flags
pub const AH_USE_EEPROM: c_uint = 0x1;
pub const AH_UNPLUGGED: c_uint = 0x2 /* The card has been physically removed. */;
pub const AH_FASTCC: c_uint = 0x4;
pub const AH_NO_EEP_SWAP: c_uint = 0x8 /* Do not swap EEPROM data */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_hw {
    pub reg_ops: ath_ops,
    pub dev: *mut device,
    pub hw: *mut ieee80211_hw,
    pub common: ath_common,
    pub hw_version: ath9k_hw_version,
    pub config: ath9k_ops_config,
    pub caps: ath9k_hw_capabilities,
    pub channels: [ath9k_channel; ATH9K_NUM_CHANNELS],
    pub curchan: *mut ath9k_channel,
    pub gpiods: [*mut gpio_desc; 32],
    pub def: ar5416_eeprom_def,
    pub map4k: ar5416_eeprom_4k,
    pub map9287: ar9287_eeprom,
    pub ar9300_eep: ar9300_eeprom,
    pub eeprom: },
    pub eep_ops: *const eeprom_ops,
    pub sw_mgmt_crypto_tx: bool,
    pub sw_mgmt_crypto_rx: bool,
    pub is_pciexpress: bool,
    pub aspm_enabled: bool,
    pub is_monitoring: bool,
    pub need_an_top2_fixup: bool,
    pub tx_trig_level: u16,
    pub nf_regs: [u32; 6],
    pub nf_2g: ath_nf_limits,
    pub nf_5g: ath_nf_limits,
    pub rfsilent: u16,
    pub rfkill_gpio: u32,
    pub rfkill_polarity: u32,
    pub ah_flags: u32,
    pub nf_override: i16,
    pub reset_power_on: bool,
    pub htc_reset_init: bool,
    pub opmode: nl80211_iftype,
    pub power_mode: ath9k_power_mode,
    pub noise: i8,
    pub caldata: *mut ath9k_hw_cal_data,
    pub pacal_info: ath9k_pacal_info,
    pub stats: ar5416Stats,
    pub txq: [ath9k_tx_queue_info; ATH9K_NUM_TX_QUEUES],
    pub ATH_KEYMAX): DECLARE_BITMAP(pending_del_keymap,,
    pub imask: ath9k_int,
    pub imrs2_reg: u32,
    pub txok_interrupt_mask: u32,
    pub txerr_interrupt_mask: u32,
    pub txdesc_interrupt_mask: u32,
    pub txeol_interrupt_mask: u32,
    pub txurn_interrupt_mask: u32,
    pub intr_ref_cnt: core::sync::atomic::AtomicI32,
    pub chip_fullsleep: bool,
    pub modes_index: u32,
// Calibration
    pub supp_cals: u32,
    pub cal_start_time: c_ulong,
    pub iq_caldata: ath9k_cal_list,
    pub adcgain_caldata: ath9k_cal_list,
    pub adcdc_caldata: ath9k_cal_list,
    pub cal_list: *mut ath9k_cal_list,
    pub cal_list_last: *mut ath9k_cal_list,
    pub cal_list_curr: *mut ath9k_cal_list,
    pub unsign: [u32; AR5416_MAX_CHAINS],
    pub sign: [i32; AR5416_MAX_CHAINS],
    pub meas0: },
    pub unsign: [u32; AR5416_MAX_CHAINS],
    pub sign: [i32; AR5416_MAX_CHAINS],
    pub meas1: },
    pub unsign: [u32; AR5416_MAX_CHAINS],
    pub sign: [i32; AR5416_MAX_CHAINS],
    pub meas2: },
    pub unsign: [u32; AR5416_MAX_CHAINS],
    pub sign: [i32; AR5416_MAX_CHAINS],
    pub meas3: },
    pub cal_samples: u16,
    pub enabled_cals: u8,
    pub sta_id1_defaults: u32,
    pub misc_mode: u32,
// Private to hardware code
    pub private_ops: ath_hw_private_ops,
// Accessed by the lower level driver
    pub ops: ath_hw_ops,
// Used to program the radio on non single-chip devices
    pub analogBank6Data: *mut u32,
    pub coverage_class: c_int,
    pub slottime: u32,
    pub globaltxtimeout: u32,
// ANI
    pub aniperiod: u32,
    pub ani_function: ath9k_ani_cmd,
    pub ani_skip_count: u32,
    pub ani: ar5416AniState,

    pub btcoex_hw: ath_btcoex_hw,

    pub intr_txqs: u32,
    pub txchainmask: u8,
    pub rxchainmask: u8,
    pub radar_conf: ath_hw_radar_conf,
    pub originalGain: [u32; 22],
    pub initPDADC: c_int,
    pub PDADCdelta: c_int,
    pub led_pin: c_int,
    pub gpio_mask: u32,
    pub gpio_val: u32,
    pub ini_dfs: ar5416IniArray,
    pub iniModes: ar5416IniArray,
    pub iniCommon: ar5416IniArray,
    pub iniBB_RfGain: ar5416IniArray,
    pub iniBank6: ar5416IniArray,
    pub iniAddac: ar5416IniArray,
    pub iniPcieSerdes: ar5416IniArray,
    pub iniPcieSerdesLowPower: ar5416IniArray,
    pub iniModesFastClock: ar5416IniArray,
    pub iniAdditional: ar5416IniArray,
    pub iniModesRxGain: ar5416IniArray,
    pub ini_modes_rx_gain_bounds: ar5416IniArray,
    pub iniModesTxGain: ar5416IniArray,
    pub iniCckfirNormal: ar5416IniArray,
    pub iniCckfirJapan2484: ar5416IniArray,
    pub iniModes_9271_ANI_reg: ar5416IniArray,
    pub ini_radio_post_sys2ant: ar5416IniArray,
    pub ini_modes_rxgain_xlna: ar5416IniArray,
    pub ini_modes_rxgain_bb_core: ar5416IniArray,
    pub ini_modes_rxgain_bb_postamble: ar5416IniArray,
    pub iniMac: [ar5416IniArray; ATH_INI_NUM_SPLIT],
    pub iniBB: [ar5416IniArray; ATH_INI_NUM_SPLIT],
    pub iniRadio: [ar5416IniArray; ATH_INI_NUM_SPLIT],
    pub iniSOC: [ar5416IniArray; ATH_INI_NUM_SPLIT],
    pub intr_gen_timer_trigger: u32,
    pub intr_gen_timer_thresh: u32,
    pub hw_gen_timers: ath_gen_timer_table,
    pub ts_ring: *mut ar9003_txs,
    pub ts_paddr_start: u32,
    pub ts_paddr_end: u32,
    pub ts_tail: u16,
    pub ts_size: u16,
    pub bb_watchdog_last_status: u32,
    pub /: *mut *mut u32 bb_watchdog_timeout_ms; / in ms, 0 to disable,
    pub /: *mut *mut u8 bb_hang_rx_ofdm; / true if bb hang due to rx_ofdm,
    pub paprd_target_power: c_uint,
    pub paprd_training_power: c_uint,
    pub paprd_ratemask: c_uint,
    pub paprd_ratemask_ht40: c_uint,
    pub paprd_table_write_done: bool,
    pub paprd_gain_table_entries: [u32; PAPRD_GAIN_TABLE_ENTRIES],
    pub paprd_gain_table_index: [u8; PAPRD_GAIN_TABLE_ENTRIES],
//
// Store the permanent value of Reg 0x4004in WARegVal
// so we dont have to R/M/W. We should not be reading
// this register when in sleep states.
//
    pub WARegVal: u32,
// Enterprise mode cap
    pub ent_mode: u32,

    pub wow: ath9k_hw_wow,

    pub is_clk_25mhz: bool,
    pub (*get_mac_revision)(void): *mut c_int,
    pub (*external_reset)(void): *mut c_int,
    pub eeprom_blob: *const firmware,
    pub /: *mut *mut *mut u16 nvmem_blob; / devres managed,
    pub nvmem_blob_len: usize,
    pub dynack: ath_dynack,
    pub tpc_enabled: bool,
    pub tx_power: [u8; Ar5416RateSize],
    pub tx_power_stbc: [u8; Ar5416RateSize],
    pub msi_enabled: bool,
    pub msi_mask: u32,
    pub msi_reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_bus_ops {
    pub ath_bus_type: ath_bus_type,
    pub csz): *mut *mut *mut void (read_cachesize)(struct ath_common common, int,
    pub data): *mut *mut *mut bool (eeprom_read)(struct ath_common common, u32 off, u16,
    pub common): *mut *mut void (bt_coex_prep)(struct ath_common,
    pub common): *mut *mut void (aspm_init)(struct ath_common,
}

// Initialization, Detach, Reset
extern "C" {
    pub fn ath9k_hw_deinit(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_init(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ath9k_hw_fill_cap_info(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ath9k_regd_get_ctl(reg: *mut ath_regulatory, chan: *mut ath9k_channel) -> u32;
}
// GPIO / RFKILL / Antennae
extern "C" {
    pub fn ath9k_hw_gpio_request_in(ah: *mut ath_hw, gpio: u32, label: *const c_char);
}
extern "C" {
    pub fn ath9k_hw_gpio_free(ah: *mut ath_hw, gpio: u32);
}
extern "C" {
    pub fn ath9k_hw_gpio_get(ah: *mut ath_hw, gpio: u32) -> u32;
}
extern "C" {
    pub fn ath9k_hw_set_gpio(ah: *mut ath_hw, gpio: u32, val: u32);
}
extern "C" {
    pub fn ath9k_hw_setantenna(ah: *mut ath_hw, antenna: u32);
}
// General Operation
extern "C" {
    pub fn ath9k_hw_wait(ah: *mut ath_hw, reg: u32, mask: u32, val: u32, timeout: u32) -> bool;
}
extern "C" {
    pub fn ath9k_hw_read_array(ah: *mut ath_hw, array[][2]: u32, size: c_int);
}
extern "C" {
    pub fn ath9k_hw_reverse_bits(val: u32, n: u32) -> u32;
}
extern "C" {
    pub fn ath9k_hw_getrxfilter(ah: *mut ath_hw) -> u32;
}
extern "C" {
    pub fn ath9k_hw_setrxfilter(ah: *mut ath_hw, bits: u32);
}
extern "C" {
    pub fn ath9k_hw_phy_disable(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ath9k_hw_disable(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ath9k_hw_set_txpowerlimit(ah: *mut ath_hw, limit: u32, test: bool);
}
extern "C" {
    pub fn ath9k_hw_setopmode(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_setmcastfilter(ah: *mut ath_hw, filter0: u32, filter1: u32);
}
extern "C" {
    pub fn ath9k_hw_write_associd(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_gettsf32(ah: *mut ath_hw) -> u32;
}
extern "C" {
    pub fn ath9k_hw_gettsf64(ah: *mut ath_hw) -> u64;
}
extern "C" {
    pub fn ath9k_hw_settsf64(ah: *mut ath_hw, tsf64: u64);
}
extern "C" {
    pub fn ath9k_hw_reset_tsf(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_get_tsf_offset(last: ktime_t, cur: ktime_t) -> u32;
}
extern "C" {
    pub fn ath9k_hw_set_tsfadjust(ah: *mut ath_hw, set: bool);
}
extern "C" {
    pub fn ath9k_hw_init_global_settings(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_get_pll_sqsum_dvc(ah: *mut ath_hw) -> u32;
}
extern "C" {
    pub fn ath9k_hw_set11nmac2040(ah: *mut ath_hw, chan: *mut ath9k_channel);
}
extern "C" {
    pub fn ath9k_hw_beaconinit(ah: *mut ath_hw, next_beacon: u32, beacon_period: u32);
}
extern "C" {
    pub fn ath9k_hw_check_nav(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_check_alive(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ath9k_hw_setpower(ah: *mut ath_hw, mode: ath9k_power_mode) -> bool;
}
// Generic hw timer primitives
extern "C" {
    pub fn ath9k_hw_gen_timer_start_tsf2(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_gen_timer_stop(ah: *mut ath_hw, timer: *mut ath_gen_timer);
}
extern "C" {
    pub fn ath_gen_timer_free(ah: *mut ath_hw, timer: *mut ath_gen_timer);
}
extern "C" {
    pub fn ath_gen_timer_isr(hw: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_name(ah: *mut ath_hw, hw_name: *mut c_char, len: usize);
}
// PHY
//
// Code Specific to AR5008, AR9001 or AR9002,
// we stuff these here to avoid callbacks for AR9003.
//
extern "C" {
    pub fn ar9002_hw_rf_claim(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ar9002_hw_enable_async_fifo(ah: *mut ath_hw);
}
//
// Code specific to AR9003, we stuff these here to avoid callbacks
// for older families
//
extern "C" {
    pub fn ar9003_hw_bb_watchdog_check(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ar9003_hw_bb_watchdog_config(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_hw_bb_watchdog_read(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_hw_bb_watchdog_dbg_info(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_hw_disable_phy_restart(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_paprd_enable(ah: *mut ath_hw, val: bool);
}
extern "C" {
    pub fn ar9003_paprd_setup_gain_table(ah: *mut ath_hw, chain: c_int);
}
extern "C" {
    pub fn ar9003_paprd_init_table(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ar9003_paprd_is_done(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ar9003_is_paprd_enabled(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ar9003_hw_set_chain_masks(ah: *mut ath_hw, rx: u8, tx: u8);
}
// Hardware family op attach helpers
extern "C" {
    pub fn ar5008_hw_attach_phy_ops(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ar9002_hw_attach_phy_ops(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_hw_attach_phy_ops(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9002_hw_attach_calib_ops(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_hw_attach_calib_ops(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9002_hw_attach_ops(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ar9003_hw_attach_ops(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9002_hw_load_ani_reg(ah: *mut ath_hw, chan: *mut ath9k_channel);
}
extern "C" {
    pub fn ath9k_ani_reset(ah: *mut ath_hw, is_scanning: bool);
}
extern "C" {
    pub fn ath9k_hw_ani_monitor(ah: *mut ath_hw, chan: *mut ath9k_channel);
}
extern "C" {
    pub fn ath9k_hw_set_ack_timeout(ah: *mut ath_hw, us: u32);
}
extern "C" {
    pub fn ath9k_hw_set_cts_timeout(ah: *mut ath_hw, us: u32);
}
extern "C" {
    pub fn ath9k_hw_setslottime(ah: *mut ath_hw, us: u32);
}

extern "C" {
    pub fn ar9003_hw_attach_aic_ops(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_btcoex_enable(ah: *mut ath_hw);
}

extern "C" {
    pub fn ath9k_hw_wow_wakeup(ah: *mut ath_hw) -> u32;
}
extern "C" {
    pub fn ath9k_hw_wow_enable(ah: *mut ath_hw, pattern_enable: u32);
}

pub const ATH9K_CLOCK_RATE_CCK: c_int = 22;
pub const ATH9K_CLOCK_RATE_5GHZ_OFDM: c_int = 40;
pub const ATH9K_CLOCK_RATE_2GHZ_OFDM: c_int = 44;
pub const ATH9K_CLOCK_FAST_RATE_5GHZ_OFDM: c_int = 44;
