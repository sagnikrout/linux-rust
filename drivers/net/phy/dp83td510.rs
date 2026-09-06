//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/dp83td510.c
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


// SPDX-License-Identifier: GPL-2.0
// Driver for the Texas Instruments DP83TD510 PHY
// Copyright (c) 2022 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
//

pub const DP83TD510E_PHY_ID: c_uint = 0x20000181;
// MDIO_MMD_VEND2 registers
pub const DP83TD510E_PHY_STS: c_uint = 0x10;
// Bit 7 - mii_interrupt, active high. Clears on read.
// Note: Clearing does not necessarily deactivate IRQ pin if interrupts pending.
// This differs from the DP83TD510E datasheet (2020) which states this bit
// clears on write 0.
//

pub const DP83TD510E_GEN_CFG: c_uint = 0x11;

pub const DP83TD510E_INTERRUPT_REG_1: c_uint = 0x12;

pub const DP83TD510E_CTRL: c_uint = 0x1f;

//
// DP83TD510E_PKT_STAT_x registers correspond to similarly named registers
// in the datasheet (PKT_STAT_1 through PKT_STAT_6). These registers store
// 32-bit or 16-bit counters for TX and RX statistics and must be read in
// sequence to ensure the counters are cleared correctly.
//
// - DP83TD510E_PKT_STAT_1: Contains TX packet count bits [15:0].
// - DP83TD510E_PKT_STAT_2: Contains TX packet count bits [31:16].
// - DP83TD510E_PKT_STAT_3: Contains TX error packet count.
// - DP83TD510E_PKT_STAT_4: Contains RX packet count bits [15:0].
// - DP83TD510E_PKT_STAT_5: Contains RX packet count bits [31:16].
// - DP83TD510E_PKT_STAT_6: Contains RX error packet count.
//
// Keeping the register names as defined in the datasheet helps maintain
// clarity and alignment with the documentation.
//
pub const DP83TD510E_PKT_STAT_1: c_uint = 0x12b;
pub const DP83TD510E_PKT_STAT_2: c_uint = 0x12c;
pub const DP83TD510E_PKT_STAT_3: c_uint = 0x12d;
pub const DP83TD510E_PKT_STAT_4: c_uint = 0x12e;
pub const DP83TD510E_PKT_STAT_5: c_uint = 0x12f;
pub const DP83TD510E_PKT_STAT_6: c_uint = 0x130;
pub const DP83TD510E_AN_STAT_1: c_uint = 0x60c;

pub const DP83TD510E_MSE_DETECT: c_uint = 0xa85;

pub const DP83TD510_SQI_MAX: c_int = 7;
// Register values are converted to SNR(dB) as suggested by
// "Application Report - DP83TD510E Cable Diagnostics Toolkit":
// SNR(dB) = -10 * log10 (VAL/2^17) - 1.76 dB.
// SQI ranges are implemented according to "OPEN ALLIANCE - Advanced diagnostic
// features for 100BASE-T1 automotive Ethernet PHYs"
//
    static const u16 dp83td510_mse_sqi_map[] = {
    0x0569, /* < 18dB */
    0x044c, /* 18dB =< SNR < 19dB */
    0x0369, /* 19dB =< SNR < 20dB */
    0x02b6, /* 20dB =< SNR < 21dB */
    0x0227, /* 21dB =< SNR < 22dB */
    0x01b6, /* 22dB =< SNR < 23dB */
    0x015b, /* 23dB =< SNR < 24dB */
    0x0000  /* 24dB =< SNR */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp83td510_stats {
    pub tx_pkt_cnt: u64,
    pub tx_err_pkt_cnt: u64,
    pub rx_pkt_cnt: u64,
    pub rx_err_pkt_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp83td510_priv {
    pub alcd_test_active: bool,
    pub stats: dp83td510_stats,
}

// Time Domain Reflectometry (TDR) Functionality of DP83TD510 PHY
//
// I assume that this PHY is using a variation of Spread Spectrum Time Domain
// Reflectometry (SSTDR) rather than the commonly used TDR found in many PHYs.
// Here are the following observations which likely confirm this:
// - The DP83TD510 PHY transmits a modulated signal of configurable length
// (default 16000 µs) instead of a single pulse pattern, which is typical
// for traditional TDR.
// - The pulse observed on the wire, triggered by the HW RESET register, is not
// part of the cable testing process.
//
// I assume that SSTDR seems to be a logical choice for the 10BaseT1L
// environment due to improved noise resistance, making it suitable for
// environments  with significant electrical noise, such as long 10BaseT1L cable
// runs.
//
// Configuration Variables:
// The SSTDR variation used in this PHY involves more configuration variables
// that can dramatically affect the functionality and precision of cable
// testing. Since most of  these configuration options are either not well
// documented or documented with minimal details, the following sections
// describe my understanding and observations of these variables and their
// impact on TDR functionality.
//
// Timeline:
// ,<--cfg_pre_silence_time
// |            ,<-SSTDR Modulated Transmission
// |	    |            ,<--cfg_post_silence_time
// |	    |            |             ,<--Force Link Mode
// |<--'-->|<-------'------->|<--'-->|<--------'------->|
//
// - cfg_pre_silence_time: Optional silence time before TDR transmission starts.
// - SSTDR Modulated Transmission: Transmission duration configured by
// cfg_tdr_tx_duration and amplitude configured by cfg_tdr_tx_type.
// - cfg_post_silence_time: Silence time after TDR transmission.
// - Force Link Mode: If nothing is configured after cfg_post_silence_time,
// the PHY continues in force link mode without autonegotiation.
//
pub const DP83TD510E_TDR_CFG: c_uint = 0x1e;

pub const DP83TD510E_TDR_CFG1: c_uint = 0x300;
// cfg_tdr_tx_type: Transmit voltage level for TDR.
// 0 = 1V, 1 = 2.4V
// Note: Using different voltage levels may not work
// in all configuration variations. For example, setting
// 2.4V may give different cable length measurements.
// Other settings may be needed to make it work properly.
//

pub const DP83TD510E_TDR_TX_TYPE_1V: c_int = 0;
pub const DP83TD510E_TDR_TX_TYPE_2_4V: c_int = 1;
// cfg_post_silence_time: Time after the TDR sequence. Since we force master mode
// for the TDR will proceed with forced link state after this time. For Linux
// it is better to set max value to avoid false link state detection.
//

pub const DP83TD510E_TDR_CFG1_POST_SILENCE_TIME_0MS: c_int = 0;
pub const DP83TD510E_TDR_CFG1_POST_SILENCE_TIME_10MS: c_int = 1;
pub const DP83TD510E_TDR_CFG1_POST_SILENCE_TIME_100MS: c_int = 2;
pub const DP83TD510E_TDR_CFG1_POST_SILENCE_TIME_1000MS: c_int = 3;
// cfg_pre_silence_time: Time before the TDR sequence. It should be enough to
// settle down all pulses and reflections. Since for 10BASE-T1L we have
// maximum 2000m cable length, we can set it to 1ms.
//

pub const DP83TD510E_TDR_CFG1_PRE_SILENCE_TIME_0MS: c_int = 0;
pub const DP83TD510E_TDR_CFG1_PRE_SILENCE_TIME_10MS: c_int = 1;
pub const DP83TD510E_TDR_CFG1_PRE_SILENCE_TIME_100MS: c_int = 2;
pub const DP83TD510E_TDR_CFG1_PRE_SILENCE_TIME_1000MS: c_int = 3;
pub const DP83TD510E_TDR_CFG2: c_uint = 0x301;

pub const DP83TD510E_TDR_END_TAP_INDEX_1_DEF: c_int = 36;

pub const DP83TD510E_TDR_START_TAP_INDEX_1_DEF: c_int = 4;
pub const DP83TD510E_TDR_CFG3: c_uint = 0x302;
// cfg_tdr_tx_duration: Duration of the TDR transmission in microseconds.
// This value sets the duration of the modulated signal used for TDR
// measurements.
// - Default: 16000 µs
// - Observation: A minimum duration of 6000 µs is recommended to ensure
// accurate detection of cable faults. Durations shorter than 6000 µs may
// result in incomplete data, especially for shorter cables (e.g., 20 meters),
// leading to false "OK" results. Longer durations (e.g., 6000 µs or more)
// provide better accuracy, particularly for detecting open circuits.
//

pub const DP83TD510E_TDR_TX_DURATION_US_DEF: c_int = 16000;
pub const DP83TD510E_TDR_FAULT_CFG1: c_uint = 0x303;

pub const DP83TD510E_TDR_FLT_LOC_OFFSET_1_DEF: c_int = 4;

pub const DP83TD510E_TDR_FLT_INIT_1_DEF: c_int = 62;
pub const DP83TD510E_TDR_FAULT_STAT: c_uint = 0x30c;

// Not documented registers and values but recommended according to
// "DP83TD510E Cable Diagnostics Toolkit revC"
//
pub const DP83TD510E_UNKN_030E: c_uint = 0x30e;
pub const DP83TD510E_030E_VAL: c_uint = 0x2520;
pub const DP83TD510E_LEDS_CFG_1: c_uint = 0x460;

// link OK
pub const DP83TD510E_LED_MODE_LINK_OK: c_uint = 0x0;
// TX/RX activity
pub const DP83TD510E_LED_MODE_TX_RX_ACTIVITY: c_uint = 0x1;
// TX activity
pub const DP83TD510E_LED_MODE_TX_ACTIVITY: c_uint = 0x2;
// RX activity
pub const DP83TD510E_LED_MODE_RX_ACTIVITY: c_uint = 0x3;
// LR
pub const DP83TD510E_LED_MODE_LR: c_uint = 0x4;
// SR
pub const DP83TD510E_LED_MODE_SR: c_uint = 0x5;
// LED SPEED: High for 10Base-T
pub const DP83TD510E_LED_MODE_LED_SPEED: c_uint = 0x6;
// Duplex mode
pub const DP83TD510E_LED_MODE_DUPLEX: c_uint = 0x7;
// link + blink on activity with stretch option
pub const DP83TD510E_LED_MODE_LINK_BLINK: c_uint = 0x8;
// blink on activity with stretch option
pub const DP83TD510E_LED_MODE_BLINK_ACTIVITY: c_uint = 0x9;
// blink on tx activity with stretch option
pub const DP83TD510E_LED_MODE_BLINK_TX: c_uint = 0xa;
// blink on rx activity with stretch option
pub const DP83TD510E_LED_MODE_BLINK_RX: c_uint = 0xb;
// link_lost
pub const DP83TD510E_LED_MODE_LINK_LOST: c_uint = 0xc;
// PRBS error: toggles on error
pub const DP83TD510E_LED_MODE_PRBS_ERROR: c_uint = 0xd;
// XMII TX/RX Error with stretch option
pub const DP83TD510E_LED_MODE_XMII_ERR: c_uint = 0xe;
pub const DP83TD510E_LED_COUNT: c_int = 4;
pub const DP83TD510E_LEDS_CFG_2: c_uint = 0x469;

pub const DP83TD510E_ALCD_STAT: c_uint = 0xa9f;

    static int dp83td510_get_mse_capability(struct phy_device *phydev,
    struct phy_mse_capability *cap)
    {
// DP83TD510E documents only a single (average) MSE register
// (used to derive SQI); no peak or worst-peak counters are
// described. Advertise only PHY_MSE_CAP_AVG.
//
    cap.supported_caps = PHY_MSE_CAP_AVG;
// 10BASE-T1L is a single-pair medium, so there are no B/C/D channels.
// We still advertise PHY_MSE_CAP_CHANNEL_A to indicate that the PHY
// can attribute the measurement to a specific pair (the only one),
// rather than exposing it only as a link-aggregate.
//
// Rationale:
// - Keeps the ethtool MSE_GET selection logic consistent: per-channel
// (A/B/C/D) is preferred over WORST/LINK, so userspace receives a
// CHANNEL_A nest instead of LINK.
// - Signals to tools that "per-pair" data is available (even if there's
// just one pair), avoiding the impression that only aggregate values
// are supported.
// - Remains compatible with multi-pair PHYs and uniform UI handling.
//
// Note: WORST and other channels are not advertised on 10BASE-T1L.
//
    cap.supported_caps |= PHY_MSE_CHANNEL_A | PHY_MSE_CAP_LINK;
    cap.max_average_mse = DP83TD510E_MSE_MAX;
// The datasheet does not specify the refresh rate or symbol count,
// but based on similar PHYs and standards, we can assume a common
// value. For 10BASE-T1L, the symbol rate is 7.5 MBd. A common
// diagnostic interval is around 1ms.
// 7.5e6 symbols/sec * 0.001 sec = 7500 symbols.
//
    cap.refresh_rate_ps = 1000000000; /* 1 ms */
    cap.num_symbols = 7500;
    return 0;
    }
    static int dp83td510_get_mse_snapshot(struct phy_device *phydev,
    enum phy_mse_channel channel,
    struct phy_mse_snapshot *snapshot)
    {
    int ret;
    if (channel != PHY_MSE_CHANNEL_LINK &&
    channel != PHY_MSE_CHANNEL_A)
    return -EOPNOTSUPP;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_MSE_DETECT);
    if (ret < 0)
    return ret;
    snapshot.average_mse = ret;
    return 0;
    }
    static int dp83td510_led_brightness_set(struct phy_device *phydev, u8 index,
    enum led_brightness brightness)
    {
    u32 val;
    if (index >= DP83TD510E_LED_COUNT)
    return -EINVAL;
    val = DP83TD510E_LED_DRV_EN(index);
    if (brightness)
    val |= DP83TD510E_LED_DRV_VAL(index);
    return phy_modify_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_LEDS_CFG_2,
    DP83TD510E_LED_DRV_VAL(index) |
    DP83TD510E_LED_DRV_EN(index), val);
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_led_mode(index: u8, rules: c_ulong) -> c_int {
    static int dp83td510_led_mode(u8 index, unsigned long rules)
    {
    if (index >= DP83TD510E_LED_COUNT)
    return -EINVAL;
    switch (rules) {
    case BIT(TRIGGER_NETDEV_LINK):
    return DP83TD510E_LED_MODE_LINK_OK;
    case BIT(TRIGGER_NETDEV_LINK_10):
    return DP83TD510E_LED_MODE_LED_SPEED;
    case BIT(TRIGGER_NETDEV_FULL_DUPLEX):
    return DP83TD510E_LED_MODE_DUPLEX;
    case BIT(TRIGGER_NETDEV_TX):
    return DP83TD510E_LED_MODE_TX_ACTIVITY;
    case BIT(TRIGGER_NETDEV_RX):
    return DP83TD510E_LED_MODE_RX_ACTIVITY;
    case BIT(TRIGGER_NETDEV_TX) | BIT(TRIGGER_NETDEV_RX):
    return DP83TD510E_LED_MODE_TX_RX_ACTIVITY;
    case BIT(TRIGGER_NETDEV_LINK) | BIT(TRIGGER_NETDEV_TX) |
    BIT(TRIGGER_NETDEV_RX):
    return DP83TD510E_LED_MODE_LINK_BLINK;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int dp83td510_led_hw_is_supported(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    int ret;
    ret = dp83td510_led_mode(index, rules);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int dp83td510_led_hw_control_set(struct phy_device *phydev, u8 index,
    unsigned long rules)
    {
    int mode, ret;
    mode = dp83td510_led_mode(index, rules);
    if (mode < 0)
    return mode;
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_LEDS_CFG_1,
    DP83TD510E_LED_FN_MASK(index),
    DP83TD510E_LED_FN(index, mode));
    if (ret)
    return ret;
    return phy_modify_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_LEDS_CFG_2,
    DP83TD510E_LED_DRV_EN(index), 0);
    }
    static int dp83td510_led_hw_control_get(struct phy_device *phydev,
    u8 index, unsigned long *rules)
    {
    int val;
    val = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_LEDS_CFG_1);
    if (val < 0)
    return val;
    val &= DP83TD510E_LED_FN_MASK(index);
    val >>= index * 4;
    switch (val) {
    case DP83TD510E_LED_MODE_LINK_OK:
// rules = BIT(TRIGGER_NETDEV_LINK);
    break;
// LED mode: LED SPEED (10BaseT1L indicator)
    case DP83TD510E_LED_MODE_LED_SPEED:
// rules = BIT(TRIGGER_NETDEV_LINK_10);
    break;
    case DP83TD510E_LED_MODE_DUPLEX:
// rules = BIT(TRIGGER_NETDEV_FULL_DUPLEX);
    break;
    case DP83TD510E_LED_MODE_TX_ACTIVITY:
// rules = BIT(TRIGGER_NETDEV_TX);
    break;
    case DP83TD510E_LED_MODE_RX_ACTIVITY:
// rules = BIT(TRIGGER_NETDEV_RX);
    break;
    case DP83TD510E_LED_MODE_TX_RX_ACTIVITY:
// rules = BIT(TRIGGER_NETDEV_TX) | BIT(TRIGGER_NETDEV_RX);
    break;
    case DP83TD510E_LED_MODE_LINK_BLINK:
// rules = BIT(TRIGGER_NETDEV_LINK) |
    BIT(TRIGGER_NETDEV_TX) |
    BIT(TRIGGER_NETDEV_RX);
    break;
    default:
// rules = 0;
    break;
    }
    return 0;
    }
    static int dp83td510_led_polarity_set(struct phy_device *phydev, int index,
    unsigned long modes)
    {
    let mut polarity: u16 = DP83TD510E_LED_POLARITY(index);
    u32 mode;
    for_each_set_bit(mode, &modes, __PHY_LED_MODES_NUM) {
    switch (mode) {
    case PHY_LED_ACTIVE_LOW:
    polarity = 0;
    break;
    default:
    return -EINVAL;
    }
    }
    return phy_modify_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_LEDS_CFG_2,
    DP83TD510E_LED_POLARITY(index), polarity);
    }
//
// dp83td510_update_stats - Update the PHY statistics for the DP83TD510 PHY.
// @phydev: Pointer to the phy_device structure.
//
// The function reads the PHY statistics registers and updates the statistics
// structure.
//
// Returns: 0 on success or a negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn dp83td510_update_stats(phydev: *mut phy_device) -> c_int {
    static int dp83td510_update_stats(struct phy_device *phydev)
    {
    struct dp83td510_priv *priv = phydev.priv;
    u32 count;
    int ret;
// The DP83TD510E_PKT_STAT registers are divided into two groups:
// - Group 1 (TX stats): DP83TD510E_PKT_STAT_1 to DP83TD510E_PKT_STAT_3
// - Group 2 (RX stats): DP83TD510E_PKT_STAT_4 to DP83TD510E_PKT_STAT_6
//
// Registers in each group are cleared only after reading them in a
// plain sequence (e.g., 1, 2, 3 for Group 1 or 4, 5, 6 for Group 2).
// Any deviation from the sequence, such as reading 1, 2, 1, 2, 3, will
// prevent the group from being cleared. Additionally, the counters
// for a group are frozen as soon as the first register in that group
// is accessed.
//
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_PKT_STAT_1);
    if (ret < 0)
    return ret;
// tx_pkt_cnt_15_0
    count = ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_PKT_STAT_2);
    if (ret < 0)
    return ret;
// tx_pkt_cnt_31_16
    count |= ret << 16;
    priv.stats.tx_pkt_cnt += count;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_PKT_STAT_3);
    if (ret < 0)
    return ret;
// tx_err_pkt_cnt
    priv.stats.tx_err_pkt_cnt += ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_PKT_STAT_4);
    if (ret < 0)
    return ret;
// rx_pkt_cnt_15_0
    count = ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_PKT_STAT_5);
    if (ret < 0)
    return ret;
// rx_pkt_cnt_31_16
    count |= ret << 16;
    priv.stats.rx_pkt_cnt += count;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_PKT_STAT_6);
    if (ret < 0)
    return ret;
// rx_err_pkt_cnt
    priv.stats.rx_err_pkt_cnt += ret;
    return 0;
    }
    static void dp83td510_get_phy_stats(struct phy_device *phydev,
    struct ethtool_eth_phy_stats *eth_stats,
    struct ethtool_phy_stats *stats)
    {
    struct dp83td510_priv *priv = phydev.priv;
    stats.tx_packets = priv.stats.tx_pkt_cnt;
    stats.tx_errors = priv.stats.tx_err_pkt_cnt;
    stats.rx_packets = priv.stats.rx_pkt_cnt;
    stats.rx_errors = priv.stats.rx_err_pkt_cnt;
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_config_intr(phydev: *mut phy_device) -> c_int {
    static int dp83td510_config_intr(struct phy_device *phydev)
    {
    int ret;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED) {
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2,
    DP83TD510E_INTERRUPT_REG_1,
    DP83TD510E_INT1_LINK_EN);
    if (ret)
    return ret;
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_VEND2,
    DP83TD510E_GEN_CFG,
    DP83TD510E_GENCFG_INT_POLARITY |
    DP83TD510E_GENCFG_INT_EN |
    DP83TD510E_GENCFG_INT_OE);
    } else {
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2,
    DP83TD510E_INTERRUPT_REG_1, 0x0);
    if (ret)
    return ret;
    ret = phy_clear_bits_mmd(phydev, MDIO_MMD_VEND2,
    DP83TD510E_GEN_CFG,
    DP83TD510E_GENCFG_INT_EN);
    if (ret)
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t dp83td510_handle_interrupt(struct phy_device *phydev)
    {
    int  ret;
// Read the current enabled interrupts
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_INTERRUPT_REG_1);
    if (ret < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    } else if (!(ret & DP83TD510E_INT1_LINK_EN) ||
    !(ret & DP83TD510E_INT1_LINK)) {
    return IRQ_NONE;
    }
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_read_status(phydev: *mut phy_device) -> c_int {
    static int dp83td510_read_status(struct phy_device *phydev)
    {
    u16 phy_sts;
    int ret;
    phydev.speed = SPEED_UNKNOWN;
    phydev.duplex = DUPLEX_UNKNOWN;
    phydev.pause = 0;
    phydev.asym_pause = 0;
    linkmode_zero(phydev.lp_advertising);
    phy_sts = phy_read(phydev, DP83TD510E_PHY_STS);
    phydev.link = !!(phy_sts & DP83TD510E_LINK_STATUS);
    if (phydev.link) {
// This PHY supports only one link mode: 10BaseT1L_Full
    phydev.duplex = DUPLEX_FULL;
    phydev.speed = SPEED_10;
    if (phydev.autoneg == AUTONEG_ENABLE) {
    ret = genphy_c45_read_lpa(phydev);
    if (ret)
    return ret;
    phy_resolve_aneg_linkmode(phydev);
    }
    }
    if (phydev.autoneg == AUTONEG_ENABLE) {
    ret = genphy_c45_baset1_read_status(phydev);
    if (ret < 0)
    return ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    DP83TD510E_AN_STAT_1);
    if (ret < 0)
    return ret;
    if (ret & DP83TD510E_MASTER_SLAVE_RESOL_FAIL)
    phydev.master_slave_state = MASTER_SLAVE_STATE_ERR;
    } else {
    return genphy_c45_pma_baset1_read_master_slave(phydev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_config_aneg(phydev: *mut phy_device) -> c_int {
    static int dp83td510_config_aneg(struct phy_device *phydev)
    {
    let mut changed: bool = false;
    int ret;
    ret = genphy_c45_pma_baset1_setup_master_slave(phydev);
    if (ret < 0)
    return ret;
    if (phydev.autoneg == AUTONEG_DISABLE)
    return genphy_c45_an_disable_aneg(phydev);
    ret = genphy_c45_an_config_aneg(phydev);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    return genphy_c45_check_and_restart_aneg(phydev, changed);
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_get_sqi(phydev: *mut phy_device) -> c_int {
    static int dp83td510_get_sqi(struct phy_device *phydev)
    {
    int sqi, ret;
    u16 mse_val;
    if (!phydev.link)
    return 0;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_MSE_DETECT);
    if (ret < 0)
    return ret;
    mse_val = 0xFFFF & ret;
    for (sqi = 0; sqi < ARRAY_SIZE(dp83td510_mse_sqi_map); sqi++) {
    if (mse_val >= dp83td510_mse_sqi_map[sqi])
    return sqi;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_get_sqi_max(phydev: *mut phy_device) -> c_int {
    static int dp83td510_get_sqi_max(struct phy_device *phydev)
    {
    return DP83TD510_SQI_MAX;
    }
//
// dp83td510_cable_test_start - Start the cable test for the DP83TD510 PHY.
// @phydev: Pointer to the phy_device structure.
//
// This sequence is implemented according to the "Application Note DP83TD510E
// Cable Diagnostics Toolkit revC".
//
// Returns: 0 on success, a negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn dp83td510_cable_test_start(phydev: *mut phy_device) -> c_int {
    static int dp83td510_cable_test_start(struct phy_device *phydev)
    {
    struct dp83td510_priv *priv = phydev.priv;
    int ret;
// If link partner is active, we won't be able to use TDR, since
// we can't force link partner to be silent. The autonegotiation
// pulses will be too frequent and the TDR sequence will be
// too long. So, TDR will always fail. Since the link is established
// we already know that the cable is working, so we can get some
// extra information line the cable length using ALCD.
//
    if (phydev.link) {
    priv.alcd_test_active = true;
    return 0;
    }
    priv.alcd_test_active = false;
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_CTRL,
    DP83TD510E_CTRL_HW_RESET);
    if (ret)
    return ret;
    ret = genphy_c45_an_disable_aneg(phydev);
    if (ret)
    return ret;
// Force master mode
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_PMA_PMD_BT1_CTRL,
    MDIO_PMA_PMD_BT1_CTRL_CFG_MST);
    if (ret)
    return ret;
// There is no official recommendation for this register, but it is
// better to use 1V for TDR since other values seems to be optimized
// for this amplitude. Except of amplitude, it is better to configure
// pre TDR silence time to 10ms to avoid false reflections (value 0
// seems to be too short, otherwise we need to implement own silence
// time). Also, post TDR silence time should be set to 1000ms to avoid
// false link state detection, it fits to the polling time of the
// PHY framework. The idea is to wait until
// dp83td510_cable_test_get_status() will be called and reconfigure
// the PHY to the default state within the post silence time window.
//
    ret = phy_modify_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_TDR_CFG1,
    DP83TD510E_TDR_TX_TYPE |
    DP83TD510E_TDR_CFG1_POST_SILENCE_TIME |
    DP83TD510E_TDR_CFG1_PRE_SILENCE_TIME,
    DP83TD510E_TDR_TX_TYPE_1V |
    DP83TD510E_TDR_CFG1_PRE_SILENCE_TIME_10MS |
    DP83TD510E_TDR_CFG1_POST_SILENCE_TIME_1000MS);
    if (ret)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_TDR_CFG2,
    FIELD_PREP(DP83TD510E_TDR_END_TAP_INDEX_1,
    DP83TD510E_TDR_END_TAP_INDEX_1_DEF) |
    FIELD_PREP(DP83TD510E_TDR_START_TAP_INDEX_1,
    DP83TD510E_TDR_START_TAP_INDEX_1_DEF));
    if (ret)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_TDR_FAULT_CFG1,
    FIELD_PREP(DP83TD510E_TDR_FLT_LOC_OFFSET_1,
    DP83TD510E_TDR_FLT_LOC_OFFSET_1_DEF) |
    FIELD_PREP(DP83TD510E_TDR_FLT_INIT_1,
    DP83TD510E_TDR_FLT_INIT_1_DEF));
    if (ret)
    return ret;
// Undocumented register, from the "Application Note DP83TD510E Cable
// Diagnostics Toolkit revC".
//
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_UNKN_030E,
    DP83TD510E_030E_VAL);
    if (ret)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_TDR_CFG3,
    DP83TD510E_TDR_TX_DURATION_US_DEF);
    if (ret)
    return ret;
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_CTRL,
    DP83TD510E_CTRL_SW_RESET);
    if (ret)
    return ret;
    return phy_set_bits_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_TDR_CFG,
    DP83TD510E_TDR_START);
    }
//
// dp83td510_cable_test_get_tdr_status - Get the status of the TDR test for the
// DP83TD510 PHY.
// @phydev: Pointer to the phy_device structure.
// @finished: Pointer to a boolean that indicates whether the test is finished.
//
// The function sets the @finished flag to true if the test is complete.
//
// Returns: 0 on success or a negative error code on failure.
//
    static int dp83td510_cable_test_get_tdr_status(struct phy_device *phydev,
    bool *finished)
    {
    int ret, stat;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_TDR_CFG);
    if (ret < 0)
    return ret;
    if (!(ret & DP83TD510E_TDR_DONE))
    return 0;
    if (!(ret & DP83TD510E_TDR_FAIL)) {
    int location;
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    DP83TD510E_TDR_FAULT_STAT);
    if (ret < 0)
    return ret;
    if (ret & DP83TD510E_TDR_PEAK_DETECT) {
    if (ret & DP83TD510E_TDR_PEAK_SIGN)
    stat = ETHTOOL_A_CABLE_RESULT_CODE_OPEN;
    else
    stat = ETHTOOL_A_CABLE_RESULT_CODE_SAME_SHORT;
    location = FIELD_GET(DP83TD510E_TDR_PEAK_LOCATION,
    ret) * 100;
    ethnl_cable_test_fault_length(phydev,
    ETHTOOL_A_CABLE_PAIR_A,
    location);
    } else {
    stat = ETHTOOL_A_CABLE_RESULT_CODE_OK;
    }
    } else {
// Most probably we have active link partner
    stat = ETHTOOL_A_CABLE_RESULT_CODE_UNSPEC;
    }
// finished = true;
    ethnl_cable_test_result(phydev, ETHTOOL_A_CABLE_PAIR_A, stat);
    return phy_init_hw(phydev);
    }
//
// dp83td510_cable_test_get_alcd_status - Get the status of the ALCD test for the
// DP83TD510 PHY.
// @phydev: Pointer to the phy_device structure.
// @finished: Pointer to a boolean that indicates whether the test is finished.
//
// The function sets the @finished flag to true if the test is complete.
// The function reads the cable length and reports it to the user.
//
// Returns: 0 on success or a negative error code on failure.
//
    static int dp83td510_cable_test_get_alcd_status(struct phy_device *phydev,
    bool *finished)
    {
    unsigned int location;
    int ret, phy_sts;
    phy_sts = phy_read(phydev, DP83TD510E_PHY_STS);
    if (!(phy_sts & DP83TD510E_LINK_STATUS)) {
// If the link is down, we can't do any thing usable now
    ethnl_cable_test_result_with_src(phydev, ETHTOOL_A_CABLE_PAIR_A,
    ETHTOOL_A_CABLE_RESULT_CODE_UNSPEC,
    ETHTOOL_A_CABLE_INF_SRC_ALCD);
// finished = true;
    return 0;
    }
    ret = phy_read_mmd(phydev, MDIO_MMD_VEND2, DP83TD510E_ALCD_STAT);
    if (ret < 0)
    return ret;
    if (!(ret & DP83TD510E_ALCD_COMPLETE))
    return 0;
    location = FIELD_GET(DP83TD510E_ALCD_CABLE_LENGTH, ret) * 100;
    ethnl_cable_test_fault_length_with_src(phydev, ETHTOOL_A_CABLE_PAIR_A,
    location,
    ETHTOOL_A_CABLE_INF_SRC_ALCD);
    ethnl_cable_test_result_with_src(phydev, ETHTOOL_A_CABLE_PAIR_A,
    ETHTOOL_A_CABLE_RESULT_CODE_OK,
    ETHTOOL_A_CABLE_INF_SRC_ALCD);
// finished = true;
    return 0;
    }
//
// dp83td510_cable_test_get_status - Get the status of the cable test for the
// DP83TD510 PHY.
// @phydev: Pointer to the phy_device structure.
// @finished: Pointer to a boolean that indicates whether the test is finished.
//
// The function sets the @finished flag to true if the test is complete.
//
// Returns: 0 on success or a negative error code on failure.
//
    static int dp83td510_cable_test_get_status(struct phy_device *phydev,
    bool *finished)
    {
    struct dp83td510_priv *priv = phydev.priv;
// finished = false;
    if (priv.alcd_test_active)
    return dp83td510_cable_test_get_alcd_status(phydev, finished);
    return dp83td510_cable_test_get_tdr_status(phydev, finished);
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_get_features(phydev: *mut phy_device) -> c_int {
    static int dp83td510_get_features(struct phy_device *phydev)
    {
// This PHY can't respond on MDIO bus if no RMII clock is enabled.
// In case RMII mode is used (most meaningful mode for this PHY) and
// the PHY do not have own XTAL, and CLK providing MAC is not probed,
// we won't be able to read all needed ability registers.
// So provide it manually.
//
    linkmode_set_bit(ETHTOOL_LINK_MODE_Autoneg_BIT, phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_Pause_BIT, phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10baseT1L_Full_BIT,
    phydev.supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp83td510_probe(phydev: *mut phy_device) -> c_int {
    static int dp83td510_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct dp83td510_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    phydev.priv = priv;
    return 0;
    }
    static struct phy_driver dp83td510_driver[] = {
    {
    PHY_ID_MATCH_MODEL(DP83TD510E_PHY_ID),
    .name		= "TI DP83TD510E",
    .flags          = PHY_POLL_CABLE_TEST,
    .probe		= dp83td510_probe,
    .config_aneg	= dp83td510_config_aneg,
    .read_status	= dp83td510_read_status,
    .get_features	= dp83td510_get_features,
    .config_intr	= dp83td510_config_intr,
    .handle_interrupt = dp83td510_handle_interrupt,
    .get_sqi	= dp83td510_get_sqi,
    .get_sqi_max	= dp83td510_get_sqi_max,
    .cable_test_start = dp83td510_cable_test_start,
    .cable_test_get_status = dp83td510_cable_test_get_status,
    .get_phy_stats	= dp83td510_get_phy_stats,
    .update_stats	= dp83td510_update_stats,
    .get_mse_capability = dp83td510_get_mse_capability,
    .get_mse_snapshot = dp83td510_get_mse_snapshot,
    .led_brightness_set = dp83td510_led_brightness_set,
    .led_hw_is_supported = dp83td510_led_hw_is_supported,
    .led_hw_control_set = dp83td510_led_hw_control_set,
    .led_hw_control_get = dp83td510_led_hw_control_get,
    .led_polarity_set = dp83td510_led_polarity_set,
    .suspend	= genphy_suspend,
    .resume		= genphy_resume,
    } };
    module_phy_driver(dp83td510_driver);
    static const struct mdio_device_id __maybe_unused dp83td510_tbl[] = {
    { PHY_ID_MATCH_MODEL(DP83TD510E_PHY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, dp83td510_tbl);
    MODULE_DESCRIPTION("Texas Instruments DP83TD510E PHY driver");
    MODULE_AUTHOR("Oleksij Rempel <kernel@pengutronix.de>");
    MODULE_LICENSE("GPL v2");
