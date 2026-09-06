//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/eeprom.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018-2019, 2021, 2024-2025 Intel Corporation
//

// EEPROM offset definitions
// indirect access definitions
pub const ADDRESS_MSK: c_uint = 0x0000FFFF;
pub const INDIRECT_TYPE_MSK: c_uint = 0x000F0000;
pub const INDIRECT_HOST: c_uint = 0x00010000;
pub const INDIRECT_GENERAL: c_uint = 0x00020000;
pub const INDIRECT_REGULATORY: c_uint = 0x00030000;
pub const INDIRECT_CALIBRATION: c_uint = 0x00040000;
pub const INDIRECT_PROCESS_ADJST: c_uint = 0x00050000;
pub const INDIRECT_OTHERS: c_uint = 0x00060000;
pub const INDIRECT_TXP_LIMIT: c_uint = 0x00070000;
pub const INDIRECT_TXP_LIMIT_SIZE: c_uint = 0x00080000;
pub const INDIRECT_ADDRESS: c_uint = 0x00100000;
// corresponding link offsets in EEPROM

// General

// calibration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_eeprom_calib_hdr {
    pub version: u8,
    pub pa_type: u8,
    pub voltage: __le16,
    pub __packed: },

// temperature

// SKU Capabilities (actual values from EEPROM definition)
    enum eeprom_sku_bits {
    EEPROM_SKU_CAP_BAND_24GHZ	= BIT(4),
    EEPROM_SKU_CAP_BAND_52GHZ	= BIT(5),
    EEPROM_SKU_CAP_11N_ENABLE	= BIT(6),
    EEPROM_SKU_CAP_AMT_ENABLE	= BIT(7),
    EEPROM_SKU_CAP_IPAN_ENABLE	= BIT(8)
}

// radio config bits (actual values from EEPROM definition)

//
// EEPROM bands
// These are the channel numbers from each band in the order
// that they are stored in the EEPROM band information. Note
// that EEPROM bands aren't the same as mac80211 bands, and
// there are even special "ht40 bands" in the EEPROM.
//
    static const u8 iwl_eeprom_band_1[14] = { /* 2.4 GHz */
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
    };
    static const u8 iwl_eeprom_band_2[] = {	/* 4915-5080MHz */
    183, 184, 185, 187, 188, 189, 192, 196, 7, 8, 11, 12, 16
    };
    static const u8 iwl_eeprom_band_3[] = {	/* 5170-5320MHz */
    34, 36, 38, 40, 42, 44, 46, 48, 52, 56, 60, 64
    };
    static const u8 iwl_eeprom_band_4[] = {	/* 5500-5700MHz */
    100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140
    };
    static const u8 iwl_eeprom_band_5[] = {	/* 5725-5825MHz */
    145, 149, 153, 157, 161, 165
    };
    static const u8 iwl_eeprom_band_6[] = {	/* 2.4 ht40 channel */
    1, 2, 3, 4, 5, 6, 7
    };
    static const u8 iwl_eeprom_band_7[] = {	/* 5.2 ht40 channel */
    36, 44, 52, 60, 100, 108, 116, 124, 132, 149, 157
    };

    ARRAY_SIZE(iwl_eeprom_band_2) + \
    ARRAY_SIZE(iwl_eeprom_band_3) + \
    ARRAY_SIZE(iwl_eeprom_band_4) + \
    ARRAY_SIZE(iwl_eeprom_band_5))
// rate data (static)
    static struct ieee80211_rate iwl_cfg80211_rates[] = {
    { .bitrate = 1 * 10, .hw_value = 0, .hw_value_short = 0, },
    { .bitrate = 2 * 10, .hw_value = 1, .hw_value_short = 1,
    .flags = IEEE80211_RATE_SHORT_PREAMBLE, },
    { .bitrate = 5.5 * 10, .hw_value = 2, .hw_value_short = 2,
    .flags = IEEE80211_RATE_SHORT_PREAMBLE, },
    { .bitrate = 11 * 10, .hw_value = 3, .hw_value_short = 3,
    .flags = IEEE80211_RATE_SHORT_PREAMBLE, },
    { .bitrate = 6 * 10, .hw_value = 4, .hw_value_short = 4, },
    { .bitrate = 9 * 10, .hw_value = 5, .hw_value_short = 5, },
    { .bitrate = 12 * 10, .hw_value = 6, .hw_value_short = 6, },
    { .bitrate = 18 * 10, .hw_value = 7, .hw_value_short = 7, },
    { .bitrate = 24 * 10, .hw_value = 8, .hw_value_short = 8, },
    { .bitrate = 36 * 10, .hw_value = 9, .hw_value_short = 9, },
    { .bitrate = 48 * 10, .hw_value = 10, .hw_value_short = 10, },
    { .bitrate = 54 * 10, .hw_value = 11, .hw_value_short = 11, },
    };
pub const RATES_24_OFFS: c_int = 0;

pub const RATES_52_OFFS: c_int = 4;

// EEPROM reading functions
#[no_mangle]
unsafe extern "C" fn iwl_eeprom_query16(eeprom: *const u8, eeprom_size: usize, offset: c_int) -> u16 {
    static u16 iwl_eeprom_query16(const u8 *eeprom, size_t eeprom_size, int offset)
    {
    if (WARN_ON(offset + sizeof(u16) > eeprom_size))
    return 0;
    return le16_to_cpup((const __le16 *)(eeprom + offset));
    }
    static u32 eeprom_indirect_address(const u8 *eeprom, size_t eeprom_size,
    u32 address)
    {
    let mut offset: u16 = 0;
    if ((address & INDIRECT_ADDRESS) == 0)
    return address;
    switch (address & INDIRECT_TYPE_MSK) {
    case INDIRECT_HOST:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_HOST);
    break;
    case INDIRECT_GENERAL:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_GENERAL);
    break;
    case INDIRECT_REGULATORY:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_REGULATORY);
    break;
    case INDIRECT_TXP_LIMIT:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_TXP_LIMIT);
    break;
    case INDIRECT_TXP_LIMIT_SIZE:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_TXP_LIMIT_SIZE);
    break;
    case INDIRECT_CALIBRATION:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_CALIBRATION);
    break;
    case INDIRECT_PROCESS_ADJST:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_PROCESS_ADJST);
    break;
    case INDIRECT_OTHERS:
    offset = iwl_eeprom_query16(eeprom, eeprom_size,
    EEPROM_LINK_OTHERS);
    break;
    default:
    WARN_ON(1);
    break;
    }
// translate the offset from words to byte
    return (address & ADDRESS_MSK) + (offset << 1);
    }
    static const void *iwl_eeprom_query_addr(const u8 *eeprom, size_t eeprom_size,
    u32 offset)
    {
    let mut address: u32 = eeprom_indirect_address(eeprom, eeprom_size, offset);
    if (WARN_ON(address >= eeprom_size))
    return core::ptr::null_mut();
    return &eeprom[address];
    }
    static int iwl_eeprom_read_calib(const u8 *eeprom, size_t eeprom_size,
    struct iwl_nvm_data *data)
    {
    const struct iwl_eeprom_calib_hdr *hdr;
    hdr = iwl_eeprom_query_addr(eeprom, eeprom_size, EEPROM_CALIB_ALL);
    if (!hdr)
    return -ENODATA;
    data.calib_version = hdr.version;
    data.calib_voltage = hdr.voltage;
    return 0;
    }
//
// enum iwl_eeprom_channel_flags - channel flags in EEPROM
// @EEPROM_CHANNEL_VALID: channel is usable for this SKU/geo
// @EEPROM_CHANNEL_IBSS: usable as an IBSS channel
// @EEPROM_CHANNEL_ACTIVE: active scanning allowed
// @EEPROM_CHANNEL_RADAR: radar detection required
// @EEPROM_CHANNEL_WIDE: 20 MHz channel okay (?)
// @EEPROM_CHANNEL_DFS: dynamic freq selection candidate
//
    enum iwl_eeprom_channel_flags {
    EEPROM_CHANNEL_VALID = BIT(0),
    EEPROM_CHANNEL_IBSS = BIT(1),
    EEPROM_CHANNEL_ACTIVE = BIT(3),
    EEPROM_CHANNEL_RADAR = BIT(4),
    EEPROM_CHANNEL_WIDE = BIT(5),
    EEPROM_CHANNEL_DFS = BIT(7),
    };
//
// struct iwl_eeprom_channel - EEPROM channel data
// @flags: %EEPROM_CHANNEL_* flags
// @max_power_avg: max power (in dBm) on this channel, at most 31 dBm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_eeprom_channel {
    pub flags: u8,
    pub max_power_avg: i8,
    pub __packed: },
    enum iwl_eeprom_enhanced_txpwr_flags {
    IWL_EEPROM_ENH_TXP_FL_VALID = BIT(0),
    IWL_EEPROM_ENH_TXP_FL_BAND_52G = BIT(1),
    IWL_EEPROM_ENH_TXP_FL_OFDM = BIT(2),
    IWL_EEPROM_ENH_TXP_FL_40MHZ = BIT(3),
    IWL_EEPROM_ENH_TXP_FL_HT_AP = BIT(4),
    IWL_EEPROM_ENH_TXP_FL_RES1 = BIT(5),
    IWL_EEPROM_ENH_TXP_FL_RES2 = BIT(6),
    IWL_EEPROM_ENH_TXP_FL_COMMON_TYPE = BIT(7),
}

//
// struct iwl_eeprom_enhanced_txpwr - enhanced regulatory TX power limits
// @flags: entry flags
// @channel: channel number
// @chain_a_max: chain a max power in 1/2 dBm
// @chain_b_max: chain b max power in 1/2 dBm
// @chain_c_max: chain c max power in 1/2 dBm
// @delta_20_in_40: 20-in-40 deltas (hi/lo)
// @mimo2_max: mimo2 max power in 1/2 dBm
// @mimo3_max: mimo3 max power in 1/2 dBm
//
// This structure presents the enhanced regulatory tx power limit layout
// in an EEPROM image.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_eeprom_enhanced_txpwr {
    pub flags: u8,
    pub channel: u8,
    pub chain_a_max: i8,
    pub chain_b_max: i8,
    pub chain_c_max: i8,
    pub delta_20_in_40: u8,
    pub mimo2_max: i8,
    pub mimo3_max: i8,
    pub __packed: },
    static s8 iwl_get_max_txpwr_half_dbm(const struct iwl_nvm_data *data,
    const struct iwl_eeprom_enhanced_txpwr *txp)
    {
    pub /: *mut *mut s8 result = 0; / (.5 dBm),
// Take the highest tx power from any valid chains
    if (data.valid_tx_ant & ANT_A && txp.chain_a_max > result)
    pub txp->chain_a_max: result =,
    if (data.valid_tx_ant & ANT_B && txp.chain_b_max > result)
    pub txp->chain_b_max: result =,
    if (data.valid_tx_ant & ANT_C && txp.chain_c_max > result)
    pub txp->chain_c_max: result =,
    if ((data.valid_tx_ant == ANT_AB ||
    data.valid_tx_ant == ANT_BC ||
    data.valid_tx_ant == ANT_AC) && txp.mimo2_max > result)
    pub txp->mimo2_max: result =,
    if (data.valid_tx_ant == ANT_ABC && txp.mimo3_max > result)
    pub txp->mimo3_max: result =,
    pub result: return,
    }

    ((txp.flags & IWL_EEPROM_ENH_TXP_FL_##x) ? # x " " : "")
    static void
    iwl_eeprom_enh_txp_read_element(struct iwl_nvm_data *data,
    const struct iwl_eeprom_enhanced_txpwr *txp,
    int n_channels, s8 max_txpower_avg)
    {
    pub ch_idx: c_int,
    pub band: enum nl80211_band,
    band = txp.flags & IWL_EEPROM_ENH_TXP_FL_BAND_52G ?
    pub NL80211_BAND_2GHZ: NL80211_BAND_5GHZ :,
    pub {: for (ch_idx = 0; ch_idx < n_channels; ch_idx++),
    pub &data->channels[ch_idx]: *mut *mut ieee80211_channel chan =,
// update matching channel or from common data only
    if (txp.channel != 0 && chan.hw_value != txp.channel)
// update matching band only
    if (band != chan.band)
    if (chan.max_power < max_txpower_avg &&
    !(txp.flags & IWL_EEPROM_ENH_TXP_FL_40MHZ))
    pub max_txpower_avg: chan->max_power =,
    }
    }
    static void iwl_eeprom_enhanced_txpower(struct device *dev,
    struct iwl_nvm_data *data,
    const u8 *eeprom, size_t eeprom_size,
    int n_channels)
    {
    pub txp: *const *const iwl_eeprom_enhanced_txpwr txp_array,,
    pub entries: int idx,,
    pub txp_len: *const __le16,
    pub max_txp_avg_halfdbm: i8,
    pub 8): BUILD_BUG_ON(sizeof(struct iwl_eeprom_enhanced_txpwr) !=,
// the length is in 16-bit words, but we want entries
    pub EEPROM_TXP_SZ_OFFS): txp_len = iwl_eeprom_query_addr(eeprom, eeprom_size,,
    pub EEPROM_TXP_ENTRY_LEN: *mut *mut entries = le16_to_cpup(txp_len)  2 /,
    pub EEPROM_TXP_OFFS): txp_array = iwl_eeprom_query_addr(eeprom, eeprom_size,,
    pub {: for (idx = 0; idx < entries; idx++),
    pub &txp_array[idx]: txp =,
// skip invalid entries
    if (!(txp.flags & IWL_EEPROM_ENH_TXP_FL_VALID))
    IWL_DEBUG_EEPROM(dev, "%s %d:\t %s%s%s%s%s%s%s%s (0x%02x)\n",
    (txp.channel && (txp.flags &
    IWL_EEPROM_ENH_TXP_FL_COMMON_TYPE)) ?
    "Common " : (txp.channel) ?
    "Channel" : "Common",
    (txp.channel),
    TXP_CHECK_AND_PRINT(VALID),
    TXP_CHECK_AND_PRINT(BAND_52G),
    TXP_CHECK_AND_PRINT(OFDM),
    TXP_CHECK_AND_PRINT(40MHZ),
    TXP_CHECK_AND_PRINT(HT_AP),
    TXP_CHECK_AND_PRINT(RES1),
    TXP_CHECK_AND_PRINT(RES2),
    TXP_CHECK_AND_PRINT(COMMON_TYPE),
    IWL_DEBUG_EEPROM(dev,
    "\t\t chain_A: %d chain_B: %d chain_C: %d\n",
    txp.chain_a_max, txp.chain_b_max,
    IWL_DEBUG_EEPROM(dev,
    "\t\t MIMO2: %d MIMO3: %d High 20_on_40: 0x%02x Low 20_on_40: 0x%02x\n",
    txp.mimo2_max, txp.mimo3_max,
    ((txp.delta_20_in_40 & 0xf0) >> 4),
    pub 0x0f)): (txp->delta_20_in_40 &,
    pub txp): max_txp_avg_halfdbm = iwl_get_max_txpwr_half_dbm(data,,
    iwl_eeprom_enh_txp_read_element(data, txp, n_channels,
    pub 2)): DIV_ROUND_UP(max_txp_avg_halfdbm,,
    if (max_txp_avg_halfdbm > data.max_tx_pwr_half_dbm)
    pub max_txp_avg_halfdbm: data->max_tx_pwr_half_dbm =,
    }
    }
    static void iwl_init_band_reference(const struct iwl_rf_cfg *cfg,
    const u8 *eeprom, size_t eeprom_size,
    int eeprom_band, int *eeprom_ch_count,
    const struct iwl_eeprom_channel **ch_info,
    const u8 **eeprom_ch_array)
    {
    pub 1]: u32 offset = cfg->eeprom_params->regulatory_bands[eeprom_band -,
    pub INDIRECT_REGULATORY: offset |= INDIRECT_ADDRESS |,
// ch_info = iwl_eeprom_query_addr(eeprom, eeprom_size, offset);
    switch (eeprom_band) {
    case 1:		/* 2.4GHz band */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_1);
// eeprom_ch_array = iwl_eeprom_band_1;
    case 2:		/* 4.9GHz band */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_2);
// eeprom_ch_array = iwl_eeprom_band_2;
    case 3:		/* 5.2GHz band */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_3);
// eeprom_ch_array = iwl_eeprom_band_3;
    case 4:		/* 5.5GHz band */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_4);
// eeprom_ch_array = iwl_eeprom_band_4;
    case 5:		/* 5.7GHz band */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_5);
// eeprom_ch_array = iwl_eeprom_band_5;
    case 6:		/* 2.4GHz ht40 channels */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_6);
// eeprom_ch_array = iwl_eeprom_band_6;
    case 7:		/* 5 GHz ht40 channels */
// eeprom_ch_count = ARRAY_SIZE(iwl_eeprom_band_7);
// eeprom_ch_array = iwl_eeprom_band_7;
    default:
// eeprom_ch_count = 0;
// eeprom_ch_array = NULL;
    }
    }

    ((eeprom_ch.flags & EEPROM_CHANNEL_##x) ? # x " " : "")
    static void iwl_mod_ht40_chan_info(struct device *dev,
    struct iwl_nvm_data *data, int n_channels,
    enum nl80211_band band, u16 channel,
    const struct iwl_eeprom_channel *eeprom_ch,
    u8 clear_ht40_extension_channel)
    {
    pub NULL: *mut *mut ieee80211_channel chan =,
    pub i: c_int,
    pub {: for (i = 0; i < n_channels; i++),
    if (data.channels[i].band != band)
    if (data.channels[i].hw_value != channel)
    pub &data->channels[i]: chan =,
    }
    if (!chan)
    IWL_DEBUG_EEPROM(dev,
    "HT40 Ch. %d [%sGHz] %s%s%s%s%s(0x%02x %ddBm): Ad-Hoc %ssupported\n",
    channel,
    band == NL80211_BAND_5GHZ ? "5.2" : "2.4",
    CHECK_AND_PRINT(IBSS),
    CHECK_AND_PRINT(ACTIVE),
    CHECK_AND_PRINT(RADAR),
    CHECK_AND_PRINT(WIDE),
    CHECK_AND_PRINT(DFS),
    eeprom_ch.flags,
    eeprom_ch.max_power_avg,
    ((eeprom_ch.flags & EEPROM_CHANNEL_IBSS) &&
    !(eeprom_ch.flags & EEPROM_CHANNEL_RADAR)) ? ""
    pub "): : "not,
    if (eeprom_ch.flags & EEPROM_CHANNEL_VALID)
    pub ~clear_ht40_extension_channel: chan->flags &=,
    }

    ((eeprom_ch_info[ch_idx].flags & EEPROM_CHANNEL_##x) ? # x " " : "")
    static int iwl_init_channel_map(struct device *dev, const struct iwl_rf_cfg *cfg,
    struct iwl_nvm_data *data,
    const u8 *eeprom, size_t eeprom_size)
    {
    pub ch_idx: int band,,
    pub eeprom_ch_info: *const iwl_eeprom_channel,
    pub eeprom_ch_array: *const u8,
    pub eeprom_ch_count: c_int,
    pub 0: int n_channels =,
//
// Loop through the 5 EEPROM bands and add them to the parse list
//
    pub {: for (band = 1; band <= 5; band++),
    pub channel: *mut ieee80211_channel,
    iwl_init_band_reference(cfg, eeprom, eeprom_size, band,
    &eeprom_ch_count, &eeprom_ch_info,
// Loop through each band adding each of the channels
    pub {: for (ch_idx = 0; ch_idx < eeprom_ch_count; ch_idx++),
    pub eeprom_ch: *const iwl_eeprom_channel,
    pub &eeprom_ch_info[ch_idx]: eeprom_ch =,
    if (!(eeprom_ch.flags & EEPROM_CHANNEL_VALID)) {
    IWL_DEBUG_EEPROM(dev,
    "Ch. %d Flags %x [%sGHz] - No traffic\n",
    eeprom_ch_array[ch_idx],
    eeprom_ch_info[ch_idx].flags,
    pub "2.4"): (band != 1) ? "5.2" :,
    }
    pub &data->channels[n_channels]: channel =,
    pub eeprom_ch_array: [channel->hw_value =; ch_idx],
    channel.band = (band == 1) ? NL80211_BAND_2GHZ
    pub NL80211_BAND_5GHZ: :,
    channel.center_freq =
    ieee80211_channel_to_frequency(
    pub channel->band): channel->hw_value,,
// set no-HT40, will enable as appropriate later
    pub IEEE80211_CHAN_NO_HT40: channel->flags =,
    if (!(eeprom_ch.flags & EEPROM_CHANNEL_IBSS))
    pub IEEE80211_CHAN_NO_IR: channel->flags |=,
    if (!(eeprom_ch.flags & EEPROM_CHANNEL_ACTIVE))
    pub IEEE80211_CHAN_NO_IR: channel->flags |=,
    if (eeprom_ch.flags & EEPROM_CHANNEL_RADAR)
    pub IEEE80211_CHAN_RADAR: channel->flags |=,
// Initialize regulatory-based run-time data
    channel.max_power =
    IWL_DEBUG_EEPROM(dev,
    "Ch. %d [%sGHz] %s%s%s%s%s%s(0x%02x %ddBm): Ad-Hoc %ssupported\n",
    channel.hw_value,
    (band != 1) ? "5.2" : "2.4",
    CHECK_AND_PRINT_I(VALID),
    CHECK_AND_PRINT_I(IBSS),
    CHECK_AND_PRINT_I(ACTIVE),
    CHECK_AND_PRINT_I(RADAR),
    CHECK_AND_PRINT_I(WIDE),
    CHECK_AND_PRINT_I(DFS),
    eeprom_ch_info[ch_idx].flags,
    eeprom_ch_info[ch_idx].max_power_avg,
    ((eeprom_ch_info[ch_idx].flags &
    EEPROM_CHANNEL_IBSS) &&
    !(eeprom_ch_info[ch_idx].flags &
    EEPROM_CHANNEL_RADAR))
    pub "): ? "" : "not,
    }
    }
    if (cfg.eeprom_params.enhanced_txpower) {
//
// for newer device (6000 series and up)
// EEPROM contain enhanced tx power information
// driver need to process addition information
// to determine the max channel tx power limits
//
    iwl_eeprom_enhanced_txpower(dev, data, eeprom, eeprom_size,
    } else {
// All others use data from channel map
    pub i: c_int,
    pub -128: data->max_tx_pwr_half_dbm =,
    pub i++): for (i = 0; i < n_channels;,
    data.max_tx_pwr_half_dbm =
    max_t(s8, data.max_tx_pwr_half_dbm,
    pub 2): *mut *mut data->channels[i].max_power,
    }
// Check if we do have HT40 channels
    if (cfg.eeprom_params.regulatory_bands[5] ==
    EEPROM_REGULATORY_BAND_NO_HT40 &&
    cfg.eeprom_params.regulatory_bands[6] ==
    EEPROM_REGULATORY_BAND_NO_HT40)
    pub n_channels: return,
// Two additional EEPROM bands for 2.4 and 5 GHz HT40 channels
    pub {: for (band = 6; band <= 7; band++),
    pub ieeeband: enum nl80211_band,
    iwl_init_band_reference(cfg, eeprom, eeprom_size, band,
    &eeprom_ch_count, &eeprom_ch_info,
// EEPROM band 6 is 2.4, band 7 is 5 GHz
    ieeeband = (band == 6) ? NL80211_BAND_2GHZ
    pub NL80211_BAND_5GHZ: :,
// Loop through each band adding each of the channels
    pub {: for (ch_idx = 0; ch_idx < eeprom_ch_count; ch_idx++),
// Set up driver's info for lower half
    iwl_mod_ht40_chan_info(dev, data, n_channels, ieeeband,
    eeprom_ch_array[ch_idx],
    &eeprom_ch_info[ch_idx],
// Set up driver's info for upper half
    iwl_mod_ht40_chan_info(dev, data, n_channels, ieeeband,
    eeprom_ch_array[ch_idx] + 4,
    &eeprom_ch_info[ch_idx],
    }
    }
    pub n_channels: return,
    }
//
// EEPROM access time values:
//
// Driver initiates EEPROM read by writing byte address << 1 to CSR_EEPROM_REG.
// Driver then polls CSR_EEPROM_REG for CSR_EEPROM_REG_READ_VALID_MSK (0x1).
// When polling, wait 10 uSec between polling loops, up to a maximum 5000 uSec.
// Driver reads 16-bit value from bits 31-16 of CSR_EEPROM_REG.
//

//
// The device's EEPROM semaphore prevents conflicts between driver and uCode
// when accessing the EEPROM; each access is a series of pulses to/from the
// EEPROM chip, not a single event, so even reads could conflict if they
// weren't arbitrated by the semaphore.
//

#[no_mangle]
unsafe extern "C" fn iwl_eeprom_acquire_semaphore(trans: *mut iwl_trans) -> c_int {
    static int iwl_eeprom_acquire_semaphore(struct iwl_trans *trans)
    {
    pub count: u16,
    pub ret: c_int,
    pub {: for (count = 0; count < IWL_EEPROM_SEM_RETRY_LIMIT; count++),
// Request semaphore
    iwl_set_bit(trans, CSR_HW_IF_CONFIG_REG,
// See if we got it
    ret = iwl_poll_bits(trans, CSR_HW_IF_CONFIG_REG,
    CSR_HW_IF_CONFIG_REG_EEPROM_OWN_SEM,
    if (!ret) {
    IWL_DEBUG_EEPROM(trans.dev,
    "Acquired semaphore after %d tries.\n",
    pub 0: return,
    }
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn iwl_eeprom_release_semaphore(trans: *mut iwl_trans) {
    static void iwl_eeprom_release_semaphore(struct iwl_trans *trans)
    {
    iwl_clear_bit(trans, CSR_HW_IF_CONFIG_REG,
    }
#[no_mangle]
unsafe extern "C" fn iwl_eeprom_verify_signature(trans: *mut iwl_trans, nvm_is_otp: bool) -> c_int {
    static int iwl_eeprom_verify_signature(struct iwl_trans *trans, bool nvm_is_otp)
    {
    pub CSR_EEPROM_GP_VALID_MSK: u32 gp = iwl_read32(trans, CSR_EEPROM_GP) &,
    pub gp): IWL_DEBUG_EEPROM(trans->dev, "EEPROM signature=0x%08x\n",,
    switch (gp) {
    case CSR_EEPROM_GP_BAD_SIG_EEP_GOOD_SIG_OTP:
    if (!nvm_is_otp) {
    IWL_ERR(trans, "EEPROM with bad signature: 0x%08x\n",
    pub -ENOENT: return,
    }
    pub 0: return,
    case CSR_EEPROM_GP_GOOD_SIG_EEP_LESS_THAN_4K:
    case CSR_EEPROM_GP_GOOD_SIG_EEP_MORE_THAN_4K:
    if (nvm_is_otp) {
    pub gp): IWL_ERR(trans, "OTP with bad signature: 0x%08x\n",,
    pub -ENOENT: return,
    }
    pub 0: return,
    case CSR_EEPROM_GP_BAD_SIGNATURE_BOTH_EEP_AND_OTP:
    default:
    IWL_ERR(trans,
    "bad EEPROM/OTP signature, type=%s, EEPROM_GP=0x%08x\n",
    pub gp): nvm_is_otp ? "OTP" : "EEPROM",,
    pub -ENOENT: return,
    }
    }
//
// OTP related functions
//
#[no_mangle]
unsafe extern "C" fn iwl_set_otp_access_absolute(trans: *mut iwl_trans) {
    static void iwl_set_otp_access_absolute(struct iwl_trans *trans)
    {
    pub CSR_OTP_GP_REG): iwl_read32(trans,,
    iwl_clear_bit(trans, CSR_OTP_GP_REG,
    }
#[no_mangle]
unsafe extern "C" fn iwl_nvm_is_otp(trans: *mut iwl_trans) -> c_int {
    static int iwl_nvm_is_otp(struct iwl_trans *trans)
    {
    pub otpgp: u32,
// OTP only valid for CP/PP and after
    switch (trans.info.hw_rev & CSR_HW_REV_TYPE_MSK) {
    case CSR_HW_REV_TYPE_NONE:
    pub type\n"): IWL_ERR(trans, "Unknown hardware,
    pub -EIO: return,
    case CSR_HW_REV_TYPE_5300:
    case CSR_HW_REV_TYPE_5350:
    case CSR_HW_REV_TYPE_5100:
    case CSR_HW_REV_TYPE_5150:
    pub 0: return,
    default:
    pub CSR_OTP_GP_REG): otpgp = iwl_read32(trans,,
    if (otpgp & CSR_OTP_GP_REG_DEVICE_SELECT)
    pub 1: return,
    pub 0: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn iwl_init_otp_access(trans: *mut iwl_trans) -> c_int {
    static int iwl_init_otp_access(struct iwl_trans *trans)
    {
    pub ret: c_int,
    pub iwl_trans_activate_nic(trans): ret =,
    if (ret)
    pub ret: return,
    iwl_set_bits_prph(trans, APMG_PS_CTRL_REG,
    iwl_clear_bits_prph(trans, APMG_PS_CTRL_REG,
//
// CSR auto clock gate disable bit -
// this is only applicable for HW with OTP shadow RAM
//
    if (trans.mac_cfg.base.shadow_ram_support)
    iwl_set_bit(trans, CSR_DBG_LINK_PWR_MGMT_REG,
    pub 0: return,
    }
    static int iwl_read_otp_word(struct iwl_trans *trans, u16 addr,
    __le16 *eeprom_data)
    {
    pub 0: int ret =,
    pub r: u32,
    pub otpgp: u32,
    iwl_write32(trans, CSR_EEPROM_REG,
    pub 1)): CSR_EEPROM_REG_MSK_ADDR & (addr <<,
    ret = iwl_poll_bits(trans, CSR_EEPROM_REG,
    CSR_EEPROM_REG_READ_VALID_MSK,
    if (ret) {
    pub addr): IWL_ERR(trans, "Time out reading OTP[%d]\n",,
    pub ret: return,
    }
    pub CSR_EEPROM_REG): r = iwl_read32(trans,,
// check for ECC errors:
    pub CSR_OTP_GP_REG): otpgp = iwl_read32(trans,,
    if (otpgp & CSR_OTP_GP_REG_ECC_UNCORR_STATUS_MSK) {
// stop in this case
// set the uncorrectable OTP ECC bit for acknowledgment
    iwl_set_bit(trans, CSR_OTP_GP_REG,
    pub read\n"): IWL_ERR(trans, "Uncorrectable OTP ECC error, abort OTP,
    pub -EINVAL: return,
    }
    if (otpgp & CSR_OTP_GP_REG_ECC_CORR_STATUS_MSK) {
// continue in this case
// set the correctable OTP ECC bit for acknowledgment
    iwl_set_bit(trans, CSR_OTP_GP_REG,
    pub read\n"): IWL_ERR(trans, "Correctable OTP ECC error, continue,
    }
// eeprom_data = cpu_to_le16(r >> 16);
    pub 0: return,
    }
//
// iwl_is_otp_empty: check for empty OTP
//
#[no_mangle]
unsafe extern "C" fn iwl_is_otp_empty(trans: *mut iwl_trans) -> bool {
    static bool iwl_is_otp_empty(struct iwl_trans *trans)
    {
    pub 0: u16 next_link_addr =,
    pub link_value: __le16,
    pub false: bool is_empty =,
// locate the beginning of OTP link list
    if (!iwl_read_otp_word(trans, next_link_addr, &link_value)) {
    if (!link_value) {
    pub empty\n"): IWL_ERR(trans, "OTP is,
    pub true: is_empty =,
    }
    } else {
    pub list.\n"): IWL_ERR(trans, "Unable to read first block of OTP,
    pub true: is_empty =,
    }
    pub is_empty: return,
    }
//
// iwl_find_otp_image: find EEPROM image in OTP
// finding the OTP block that contains the EEPROM image.
// the last valid block on the link list (the block _before_ the last block)
// is the block we should read and used to configure the device.
// If all the available OTP blocks are full, the last block will be the block
// we should read and used to configure the device.
// only perform this operation if shadow RAM is disabled
//
    static int iwl_find_otp_image(struct iwl_trans *trans,
    u16 *validblockaddr)
    {
    pub valid_addr: u16 next_link_addr = 0,,
    pub 0: __le16 link_value =,
    pub 0: int usedblocks =,
// set addressing mode to absolute to traverse the link list
// checking for empty OTP or error
    if (iwl_is_otp_empty(trans))
    pub -EINVAL: return,
//
// start traverse link list
// until reach the max number of OTP blocks
// different devices have different number of OTP blocks
//
    do {
// save current valid block address
// check for more block on the link list
//
    pub next_link_addr: valid_addr =,
    pub sizeof(u16): *mut *mut next_link_addr = le16_to_cpu(link_value),
    IWL_DEBUG_EEPROM(trans.dev, "OTP blocks %d addr 0x%x\n",
    pub next_link_addr): usedblocks,,
    if (iwl_read_otp_word(trans, next_link_addr, &link_value))
    pub -EINVAL: return,
    if (!link_value) {
//
// reach the end of link list, return success and
// set address point to the starting address
// of the image
//
// validblockaddr = valid_addr;
// skip first 2 bytes (link list pointer)
// validblockaddr += 2;
    pub 0: return,
    }
// more in the link list, continue
    pub trans->mac_cfg->base->max_ll_items): } while (usedblocks <=,
// OTP has no valid blocks
    pub blocks\n"): IWL_DEBUG_EEPROM(trans->dev, "OTP has no valid,
    pub -EINVAL: return,
    }
//
// iwl_read_eeprom - read EEPROM contents
//
// Load the EEPROM contents from adapter and return it
// and its size.
//
// NOTE:  This routine uses the non-debug IO access functions.
//
#[no_mangle]
pub unsafe extern "C" fn iwl_read_eeprom(trans: *mut iwl_trans, eeprom: *mut u8, eeprom_size: *mut usize) -> c_int {
    int iwl_read_eeprom(struct iwl_trans *trans, u8 **eeprom, size_t *eeprom_size)
    {
    pub e: *mut __le16,
    pub CSR_EEPROM_GP): u32 gp = iwl_read32(trans,,
    pub sz: c_int,
    pub ret: c_int,
    pub addr: u16,
    pub 0: u16 validblockaddr =,
    pub 0: u16 cache_addr =,
    pub nvm_is_otp: c_int,
    if (!eeprom || !eeprom_size)
    pub -EINVAL: return,
    pub iwl_nvm_is_otp(trans): nvm_is_otp =,
    if (nvm_is_otp < 0)
    pub nvm_is_otp: return,
    pub trans->mac_cfg->base->eeprom_size: sz =,
    pub sz): IWL_DEBUG_EEPROM(trans->dev, "NVM size = %d\n",,
    pub GFP_KERNEL): e = kmalloc(sz,,
    if (!e)
    pub -ENOMEM: return,
    pub nvm_is_otp): ret = iwl_eeprom_verify_signature(trans,,
    if (ret) {
    pub gp): IWL_ERR(trans, "EEPROM not found, EEPROM_GP=0x%08x\n",,
    pub err_free: goto,
    }
// Make sure driver (instead of uCode) is allowed to read EEPROM
    pub iwl_eeprom_acquire_semaphore(trans): ret =,
    if (ret) {
    pub semaphore.\n"): IWL_ERR(trans, "Failed to acquire EEPROM,
    pub err_free: goto,
    }
    if (nvm_is_otp) {
    pub iwl_init_otp_access(trans): ret =,
    if (ret) {
    pub access.\n"): IWL_ERR(trans, "Failed to initialize OTP,
    pub err_unlock: goto,
    }
    iwl_write32(trans, CSR_EEPROM_GP,
    iwl_read32(trans, CSR_EEPROM_GP) &
    iwl_set_bit(trans, CSR_OTP_GP_REG,
    CSR_OTP_GP_REG_ECC_CORR_STATUS_MSK |
// traversing the linked list if no shadow ram supported
    if (!trans.mac_cfg.base.shadow_ram_support) {
    pub &validblockaddr): ret = iwl_find_otp_image(trans,,
    if (ret)
    pub err_unlock: goto,
    }
    pub sz: for (addr = validblockaddr; addr < validblockaddr +,
    addr += sizeof(u16)) {
    pub eeprom_data: __le16,
    pub &eeprom_data): ret = iwl_read_otp_word(trans, addr,,
    if (ret)
    pub err_unlock: goto,
    pub eeprom_data: e[cache_addr / 2] =,
    pub sizeof(u16): cache_addr +=,
    }
    } else {
// eeprom is an array of 16bit values
    pub {: for (addr = 0; addr < sz; addr += sizeof(u16)),
    pub r: u32,
    iwl_write32(trans, CSR_EEPROM_REG,
    pub 1)): CSR_EEPROM_REG_MSK_ADDR & (addr <<,
    ret = iwl_poll_bits(trans, CSR_EEPROM_REG,
    CSR_EEPROM_REG_READ_VALID_MSK,
    if (ret) {
    IWL_ERR(trans,
    pub addr): "Time out reading EEPROM[%d]\n",,
    pub err_unlock: goto,
    }
    pub CSR_EEPROM_REG): r = iwl_read32(trans,,
    pub 16): e[addr / 2] = cpu_to_le16(r >>,
    }
    }
    IWL_DEBUG_EEPROM(trans.dev, "NVM Type: %s\n",
    pub "EEPROM"): nvm_is_otp ? "OTP" :,
// eeprom_size = sz;
// eeprom = (u8 *)e;
    pub 0: return,
    err_unlock:
    err_free:
    pub ret: return,
    }
    static void iwl_init_sbands(struct iwl_trans *trans, const struct iwl_rf_cfg *cfg,
    struct iwl_nvm_data *data,
    const u8 *eeprom, size_t eeprom_size)
    {
    pub trans->dev: *mut *mut device dev =,
    int n_channels = iwl_init_channel_map(dev, cfg, data,
    pub eeprom_size): eeprom,,
    pub 0: int n_used =,
    pub sband: *mut ieee80211_supported_band,
    pub &data->bands[NL80211_BAND_2GHZ]: sband =,
    pub NL80211_BAND_2GHZ: sband->band =,
    pub &iwl_cfg80211_rates[RATES_24_OFFS]: sband->bitrates =,
    pub N_RATES_24: sband->n_bitrates =,
    n_used += iwl_init_sband_channels(data, sband, n_channels,
    iwl_init_ht_hw_capab(trans, data, &sband.ht_cap, NL80211_BAND_2GHZ,
    pub data->valid_rx_ant): data->valid_tx_ant,,
    pub &data->bands[NL80211_BAND_5GHZ]: sband =,
    pub NL80211_BAND_5GHZ: sband->band =,
    pub &iwl_cfg80211_rates[RATES_52_OFFS]: sband->bitrates =,
    pub N_RATES_52: sband->n_bitrates =,
    n_used += iwl_init_sband_channels(data, sband, n_channels,
    iwl_init_ht_hw_capab(trans, data, &sband.ht_cap, NL80211_BAND_5GHZ,
    pub data->valid_rx_ant): data->valid_tx_ant,,
    if (n_channels != n_used)
    IWL_ERR_DEV(dev, "EEPROM: used only %d of %d channels\n",
    pub n_channels): n_used,,
    }
// EEPROM data functions
    struct iwl_nvm_data *
    iwl_parse_eeprom_data(struct iwl_trans *trans, const struct iwl_rf_cfg *cfg,
    const u8 *eeprom, size_t eeprom_size)
    {
    pub data: *mut iwl_nvm_data,
    pub trans->dev: *mut *mut device dev =,
    pub tmp: *const c_void,
    pub sku: u16 radio_cfg,,
    if (WARN_ON(!cfg || !cfg.eeprom_params))
    pub NULL: return,
    pub IWL_NUM_CHANNELS): *mut *mut data = kzalloc_flex(data, channels,,
    if (!data)
    pub NULL: return,
// get MAC address(es)
    pub EEPROM_MAC_ADDRESS): tmp = iwl_eeprom_query_addr(eeprom, eeprom_size,,
    if (!tmp)
    pub err_free: goto,
    pub ETH_ALEN): memcpy(data->hw_addr, tmp,,
    data.n_hw_addrs = iwl_eeprom_query16(eeprom, eeprom_size,
    if (iwl_eeprom_read_calib(eeprom, eeprom_size, data))
    pub err_free: goto,
    pub EEPROM_XTAL): tmp = iwl_eeprom_query_addr(eeprom, eeprom_size,,
    if (!tmp)
    pub err_free: goto,
    pub sizeof(data->xtal_calib)): memcpy(data->xtal_calib, tmp,,
    tmp = iwl_eeprom_query_addr(eeprom, eeprom_size,
    if (!tmp)
    pub err_free: goto,
    pub )tmp: *const *const data->raw_temperature = (__le16,
    tmp = iwl_eeprom_query_addr(eeprom, eeprom_size,
    if (!tmp)
    pub err_free: goto,
    pub )tmp: *const *const data->kelvin_temperature = (__le16,
    pub 1): *const *const *const data->kelvin_voltage = ((__le16 )tmp +,
    radio_cfg =
    pub EEPROM_RADIO_CONFIG): iwl_eeprom_query16(eeprom, eeprom_size,,
    pub EEPROM_RF_CFG_DASH_MSK(radio_cfg): data->radio_cfg_dash =,
    pub EEPROM_RF_CFG_PNUM_MSK(radio_cfg): data->radio_cfg_pnum =,
    pub EEPROM_RF_CFG_STEP_MSK(radio_cfg): data->radio_cfg_step =,
    pub EEPROM_RF_CFG_TYPE_MSK(radio_cfg): data->radio_cfg_type =,
    pub EEPROM_RF_CFG_RX_ANT_MSK(radio_cfg): data->valid_rx_ant =,
    pub EEPROM_RF_CFG_TX_ANT_MSK(radio_cfg): data->valid_tx_ant =,
    sku = iwl_eeprom_query16(eeprom, eeprom_size,
    pub EEPROM_SKU_CAP_11N_ENABLE: data->sku_cap_11n_enable = sku &,
    pub EEPROM_SKU_CAP_AMT_ENABLE: data->sku_cap_amt_enable = sku &,
    pub EEPROM_SKU_CAP_BAND_24GHZ: data->sku_cap_band_24ghz_enable = sku &,
    pub EEPROM_SKU_CAP_BAND_52GHZ: data->sku_cap_band_52ghz_enable = sku &,
    pub EEPROM_SKU_CAP_IPAN_ENABLE: data->sku_cap_ipan_enable = sku &,
    if (iwlwifi_mod_params.disable_11n & IWL_DISABLE_HT_ALL)
    pub false: data->sku_cap_11n_enable =,
    data.nvm_version = iwl_eeprom_query16(eeprom, eeprom_size,
// check overrides (some devices have wrong EEPROM)
    if (cfg.valid_tx_ant)
    pub cfg->valid_tx_ant: data->valid_tx_ant =,
    if (cfg.valid_rx_ant)
    pub cfg->valid_rx_ant: data->valid_rx_ant =,
    if (!data.valid_tx_ant || !data.valid_rx_ant) {
    IWL_ERR_DEV(dev, "invalid antennas (0x%x, 0x%x)\n",
    pub data->valid_rx_ant): data->valid_tx_ant,,
    pub err_free: goto,
    }
    pub eeprom_size): iwl_init_sbands(trans, cfg, data, eeprom,,
    pub data: return,
    err_free:
    pub NULL: return,
    }
