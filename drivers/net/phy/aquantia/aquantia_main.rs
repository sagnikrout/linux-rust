//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/aquantia/aquantia_main.c
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
//
// Driver for Aquantia PHY
//
// Author: Shaohui Xie <Shaohui.Xie@freescale.com>
//
// Copyright 2015 Freescale Semiconductor, Inc.
//

pub const PHY_ID_AQ1202: c_uint = 0x03a1b445;
pub const PHY_ID_AQ2104: c_uint = 0x03a1b460;
pub const PHY_ID_AQR105: c_uint = 0x03a1b4a2;
pub const PHY_ID_AQR106: c_uint = 0x03a1b4d0;
pub const PHY_ID_AQR107: c_uint = 0x03a1b4e0;
pub const PHY_ID_AQCS109: c_uint = 0x03a1b5c2;
pub const PHY_ID_AQR405: c_uint = 0x03a1b4b0;
pub const PHY_ID_AQR111: c_uint = 0x03a1b610;
pub const PHY_ID_AQR111B0: c_uint = 0x03a1b612;
pub const PHY_ID_AQR112: c_uint = 0x03a1b662;
pub const PHY_ID_AQR412: c_uint = 0x03a1b6f2;
pub const PHY_ID_AQR412C: c_uint = 0x03a1b712;
pub const PHY_ID_AQR113: c_uint = 0x31c31c40;
pub const PHY_ID_AQR113C: c_uint = 0x31c31c12;
pub const PHY_ID_AQR114C: c_uint = 0x31c31c22;
pub const PHY_ID_AQR115: c_uint = 0x31c31c63;
pub const PHY_ID_AQR115C: c_uint = 0x31c31c33;
pub const PHY_ID_AQR813: c_uint = 0x31c31cb2;
pub const MDIO_PHYXS_VEND_PROV2: c_uint = 0xc441;

pub const MDIO_PHYXS_VEND_IF_STATUS: c_uint = 0xe812;

pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_KR: c_int = 0;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_KX: c_int = 1;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_XFI: c_int = 2;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_USXGMII: c_int = 3;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_XAUI: c_int = 4;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_SGMII: c_int = 6;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_RXAUI: c_int = 7;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_OFF: c_int = 9;
pub const MDIO_PHYXS_VEND_IF_STATUS_TYPE_OCSGMII: c_int = 10;
pub const MDIO_AN_VEND_PROV: c_uint = 0xc400;

pub const MDIO_AN_VEND_PROV_DOWNSHIFT_DFLT: c_int = 4;
pub const MDIO_AN_RESVD_VEND_PROV: c_uint = 0xc410;
pub const MDIO_AN_RESVD_VEND_PROV_MDIX_AUTO: c_int = 0;
pub const MDIO_AN_RESVD_VEND_PROV_MDIX_MDI: c_int = 1;
pub const MDIO_AN_RESVD_VEND_PROV_MDIX_MDIX: c_int = 2;

pub const MDIO_AN_TX_VEND_STATUS1: c_uint = 0xc800;

pub const MDIO_AN_TX_VEND_STATUS1_10BASET: c_int = 0;
pub const MDIO_AN_TX_VEND_STATUS1_100BASETX: c_int = 1;
pub const MDIO_AN_TX_VEND_STATUS1_1000BASET: c_int = 2;
pub const MDIO_AN_TX_VEND_STATUS1_10GBASET: c_int = 3;
pub const MDIO_AN_TX_VEND_STATUS1_2500BASET: c_int = 4;
pub const MDIO_AN_TX_VEND_STATUS1_5000BASET: c_int = 5;

pub const MDIO_AN_RESVD_VEND_STATUS1: c_uint = 0xc810;

pub const MDIO_AN_TX_VEND_INT_STATUS1: c_uint = 0xcc00;

pub const MDIO_AN_TX_VEND_INT_STATUS2: c_uint = 0xcc01;

pub const MDIO_AN_TX_VEND_INT_MASK2: c_uint = 0xd401;

pub const PMAPMD_FW_MISC_ID: c_uint = 0xc41d;
pub const PMAPMD_FW_MISC_VER: c_uint = 0xc41e;
pub const PMAPMD_RSVD_VEND_PROV: c_uint = 0xe400;

pub const MDIO_AN_RX_LP_STAT1: c_uint = 0xe820;

pub const MDIO_AN_RX_LP_STAT4: c_uint = 0xe823;

pub const MDIO_AN_RX_VEND_STAT3: c_uint = 0xe832;

// Sleep and timeout for checking if the Processor-Intensive
// MDIO operation is finished
//
pub const AQR107_OP_IN_PROG_SLEEP: c_int = 1000;
pub const AQR107_OP_IN_PROG_TIMEOUT: c_int = 100000;
#[no_mangle]
unsafe extern "C" fn aqr107_get_sset_count(phydev: *mut phy_device) -> c_int {
    static int aqr107_get_sset_count(struct phy_device *phydev)
    {
    return AQR107_SGMII_STAT_SZ;
    }
#[no_mangle]
unsafe extern "C" fn aqr107_get_strings(phydev: *mut phy_device, data: *mut u8) {
    static void aqr107_get_strings(struct phy_device *phydev, u8 *data)
    {
    int i;
    for (i = 0; i < AQR107_SGMII_STAT_SZ; i++)
    strscpy(data + i * ETH_GSTRING_LEN, aqr107_hw_stats[i].name,
    ETH_GSTRING_LEN);
    }
#[no_mangle]
unsafe extern "C" fn aqr107_get_stat(phydev: *mut phy_device, index: c_int) -> u64 {
    static u64 aqr107_get_stat(struct phy_device *phydev, int index)
    {
    const struct aqr107_hw_stat *stat = aqr107_hw_stats + index;
    let mut len_l: c_int = min(stat.size, 16);
    let mut len_h: c_int = stat.size - len_l;
    u64 ret;
    int val;
    val = phy_read_mmd(phydev, MDIO_MMD_C22EXT, stat.reg);
    if (val < 0)
    return U64_MAX;
    ret = val & GENMASK(len_l - 1, 0);
    if (len_h) {
    val = phy_read_mmd(phydev, MDIO_MMD_C22EXT, stat.reg + 1);
    if (val < 0)
    return U64_MAX;
    ret += (val & GENMASK(len_h - 1, 0)) << 16;
    }
    return ret;
    }
    static void aqr107_get_stats(struct phy_device *phydev,
    struct ethtool_stats *stats, u64 *data)
    {
    struct aqr107_priv *priv = phydev.priv;
    u64 val;
    int i;
    for (i = 0; i < AQR107_SGMII_STAT_SZ; i++) {
    val = aqr107_get_stat(phydev, i);
    if (val == U64_MAX)
    phydev_err(phydev, "Reading HW Statistics failed for %s\n",
    aqr107_hw_stats[i].name);
    else
    priv.sgmii_stats[i] += val;
    data[i] = priv.sgmii_stats[i];
    }
    }
#[no_mangle]
unsafe extern "C" fn aqr_set_mdix(phydev: *mut phy_device, mdix: c_int) -> c_int {
    static int aqr_set_mdix(struct phy_device *phydev, int mdix)
    {
    let mut val: u16 = 0;
    switch (mdix) {
    case ETH_TP_MDI:
    val = MDIO_AN_RESVD_VEND_PROV_MDIX_MDI;
    break;
    case ETH_TP_MDI_X:
    val = MDIO_AN_RESVD_VEND_PROV_MDIX_MDIX;
    break;
    case ETH_TP_MDI_AUTO:
    case ETH_TP_MDI_INVALID:
    default:
    val = MDIO_AN_RESVD_VEND_PROV_MDIX_AUTO;
    break;
    }
    return phy_modify_mmd_changed(phydev, MDIO_MMD_AN, MDIO_AN_RESVD_VEND_PROV,
    MDIO_AN_RESVD_VEND_PROV_MDIX_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn aqr_config_aneg(phydev: *mut phy_device) -> c_int {
    static int aqr_config_aneg(struct phy_device *phydev)
    {
    let mut changed: bool = false;
    u16 reg;
    int ret;
    ret = aqr_set_mdix(phydev, phydev.mdix_ctrl);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    if (phydev.autoneg == AUTONEG_DISABLE)
    return genphy_c45_pma_setup_forced(phydev);
    ret = genphy_c45_an_config_aneg(phydev);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
// Clause 45 has no standardized support for 1000BaseT, therefore
// use vendor registers for this mode.
//
    reg = 0;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_1000BASET_FULL;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_1000BASET_HALF;
// Handle the case when the 2.5G and 5G speeds are not advertised
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_2500BASET_FULL;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_5000baseT_Full_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_5000BASET_FULL;
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_AN, MDIO_AN_VEND_PROV,
    MDIO_AN_VEND_PROV_1000BASET_HALF |
    MDIO_AN_VEND_PROV_1000BASET_FULL |
    MDIO_AN_VEND_PROV_2500BASET_FULL |
    MDIO_AN_VEND_PROV_5000BASET_FULL, reg);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    return genphy_c45_check_and_restart_aneg(phydev, changed);
    }
#[no_mangle]
unsafe extern "C" fn aqr_config_intr(phydev: *mut phy_device) -> c_int {
    static int aqr_config_intr(struct phy_device *phydev)
    {
    let mut en: bool = phydev.interrupts == PHY_INTERRUPT_ENABLED;
    int err;
    if (en) {
// Clear any pending interrupts before enabling them
    err = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_TX_VEND_INT_STATUS2);
    if (err < 0)
    return err;
    }
    err = phy_write_mmd(phydev, MDIO_MMD_AN, MDIO_AN_TX_VEND_INT_MASK2,
    en ? MDIO_AN_TX_VEND_INT_MASK2_LINK : 0);
    if (err < 0)
    return err;
    err = phy_write_mmd(phydev, MDIO_MMD_VEND1, VEND1_GLOBAL_INT_STD_MASK,
    en ? VEND1_GLOBAL_INT_STD_MASK_ALL : 0);
    if (err < 0)
    return err;
    err = phy_write_mmd(phydev, MDIO_MMD_VEND1, VEND1_GLOBAL_INT_VEND_MASK,
    en ? VEND1_GLOBAL_INT_VEND_MASK_GLOBAL3 |
    VEND1_GLOBAL_INT_VEND_MASK_AN : 0);
    if (err < 0)
    return err;
    if (!en) {
// Clear any pending interrupts after we have disabled them
    err = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_TX_VEND_INT_STATUS2);
    if (err < 0)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t aqr_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read_mmd(phydev, MDIO_MMD_AN,
    MDIO_AN_TX_VEND_INT_STATUS2);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & MDIO_AN_TX_VEND_INT_STATUS2_MASK))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn aqr_read_status(phydev: *mut phy_device) -> c_int {
    static int aqr_read_status(struct phy_device *phydev)
    {
    int val;
    if (phydev.autoneg == AUTONEG_ENABLE) {
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_RX_LP_STAT1);
    if (val < 0)
    return val;
    linkmode_mod_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    phydev.lp_advertising,
    val & MDIO_AN_RX_LP_STAT1_1000BASET_FULL);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT,
    phydev.lp_advertising,
    val & MDIO_AN_RX_LP_STAT1_1000BASET_HALF);
    }
    val = genphy_c45_aneg_done(phydev);
    if (val < 0)
    return val;
    if (val) {
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_RESVD_VEND_STATUS1);
    if (val < 0)
    return val;
    if (val & MDIO_AN_RESVD_VEND_STATUS1_MDIX)
    phydev.mdix = ETH_TP_MDI_X;
    else
    phydev.mdix = ETH_TP_MDI;
    } else {
    phydev.mdix = ETH_TP_MDI_INVALID;
    }
    return genphy_c45_read_status(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr105_get_features(phydev: *mut phy_device) -> c_int {
    static int aqr105_get_features(struct phy_device *phydev)
    {
    int ret;
// Normal feature discovery
    ret = genphy_c45_pma_read_abilities(phydev);
    if (ret)
    return ret;
// The AQR105 PHY misses to indicate the 2.5G and 5G modes, so add them
// here
//
    linkmode_set_bit(ETHTOOL_LINK_MODE_5000baseT_Full_BIT,
    phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT,
    phydev.supported);
// The AQR105 PHY suppports both RJ45 and SFP+ interfaces
    linkmode_set_bit(ETHTOOL_LINK_MODE_TP_BIT, phydev.supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_FIBRE_BIT, phydev.supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr105_setup_forced(phydev: *mut phy_device) -> c_int {
    static int aqr105_setup_forced(struct phy_device *phydev)
    {
    let mut vend: c_int = MDIO_AN_VEND_PROV_EXC_PHYID_INFO;
    let mut ctrl10: c_int = 0;
    let mut adv: c_int = ADVERTISE_CSMA;
    int ret;
    switch (phydev.speed) {
    case SPEED_100:
    adv |= ADVERTISE_100FULL;
    break;
    case SPEED_1000:
    adv |= ADVERTISE_NPAGE;
    if (phydev.duplex == DUPLEX_FULL)
    vend |= MDIO_AN_VEND_PROV_1000BASET_FULL;
    else
    vend |= MDIO_AN_VEND_PROV_1000BASET_HALF;
    break;
    case SPEED_2500:
    adv |= (ADVERTISE_NPAGE | ADVERTISE_XNP);
    vend |= MDIO_AN_VEND_PROV_2500BASET_FULL;
    break;
    case SPEED_5000:
    adv |= (ADVERTISE_NPAGE | ADVERTISE_XNP);
    vend |= MDIO_AN_VEND_PROV_5000BASET_FULL;
    break;
    case SPEED_10000:
    adv |= (ADVERTISE_NPAGE | ADVERTISE_XNP);
    ctrl10 |= MDIO_AN_10GBT_CTRL_ADV10G;
    break;
    default:
    return -EINVAL;
    }
    ret = phy_write_mmd(phydev, MDIO_MMD_AN, MDIO_AN_ADVERTISE, adv);
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_AN, MDIO_AN_VEND_PROV, vend);
    if (ret < 0)
    return ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_AN, MDIO_AN_10GBT_CTRL, ctrl10);
    if (ret < 0)
    return ret;
// set by vendor driver, but should be on by default
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_AN, MDIO_CTRL1,
    MDIO_AN_CTRL1_XNP);
    if (ret < 0)
    return ret;
    return genphy_c45_an_disable_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr105_config_aneg(phydev: *mut phy_device) -> c_int {
    static int aqr105_config_aneg(struct phy_device *phydev)
    {
    let mut changed: bool = false;
    u16 reg;
    int ret;
    ret = aqr_set_mdix(phydev, phydev.mdix_ctrl);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    if (phydev.autoneg == AUTONEG_DISABLE)
    return aqr105_setup_forced(phydev);
    ret = genphy_c45_an_config_aneg(phydev);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
// Clause 45 has no standardized support for 1000BaseT, therefore
// use vendor registers for this mode.
//
    reg = 0;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_1000BASET_FULL;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_1000BASET_HALF;
// Handle the case when the 2.5G and 5G speeds are not advertised
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_2500BASET_FULL;
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_5000baseT_Full_BIT,
    phydev.advertising))
    reg |= MDIO_AN_VEND_PROV_5000BASET_FULL;
    ret = phy_modify_mmd_changed(phydev, MDIO_MMD_AN, MDIO_AN_VEND_PROV,
    MDIO_AN_VEND_PROV_1000BASET_HALF |
    MDIO_AN_VEND_PROV_1000BASET_FULL |
    MDIO_AN_VEND_PROV_2500BASET_FULL |
    MDIO_AN_VEND_PROV_5000BASET_FULL, reg);
    if (ret < 0)
    return ret;
    if (ret > 0)
    changed = true;
    return genphy_c45_check_and_restart_aneg(phydev, changed);
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen1_read_rate(phydev: *mut phy_device) -> c_int {
    static int aqr_gen1_read_rate(struct phy_device *phydev)
    {
    int val;
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_TX_VEND_STATUS1);
    if (val < 0)
    return val;
    if (val & MDIO_AN_TX_VEND_STATUS1_FULL_DUPLEX)
    phydev.duplex = DUPLEX_FULL;
    else
    phydev.duplex = DUPLEX_HALF;
    switch (FIELD_GET(MDIO_AN_TX_VEND_STATUS1_RATE_MASK, val)) {
    case MDIO_AN_TX_VEND_STATUS1_10BASET:
    phydev.speed = SPEED_10;
    break;
    case MDIO_AN_TX_VEND_STATUS1_100BASETX:
    phydev.speed = SPEED_100;
    break;
    case MDIO_AN_TX_VEND_STATUS1_1000BASET:
    phydev.speed = SPEED_1000;
    break;
    case MDIO_AN_TX_VEND_STATUS1_2500BASET:
    phydev.speed = SPEED_2500;
    break;
    case MDIO_AN_TX_VEND_STATUS1_5000BASET:
    phydev.speed = SPEED_5000;
    break;
    case MDIO_AN_TX_VEND_STATUS1_10GBASET:
    phydev.speed = SPEED_10000;
    break;
    default:
    phydev.speed = SPEED_UNKNOWN;
    }
    return 0;
    }
// Quad port PHYs like AQR412(C) have 4 system interfaces, but they can also be
// used with a single system interface over which all 4 ports are multiplexed
// (10G-QXGMII). To the MDIO registers, this mode is indistinguishable from
// USXGMII (which implies a single 10G port).
//
// To not rely solely on the device tree, we allow the regular system interface
// detection to work as usual, but we replace USXGMII with 10G-QXGMII based on
// the specific fingerprint of firmware images that are known to be for MUSX.
//
    static phy_interface_t aqr_translate_interface(struct phy_device *phydev,
    phy_interface_t interface)
    {
    struct aqr107_priv *priv = phydev.priv;
    if (phy_id_compare(phydev.drv.phy_id, PHY_ID_AQR412C, phydev.drv.phy_id_mask) &&
    priv.fingerprint == AQR_G3_V4_3_C_AQR_NXP_SPF_30841_MUSX_ID40019_VER1198 &&
    interface == PHY_INTERFACE_MODE_USXGMII)
    return PHY_INTERFACE_MODE_10G_QXGMII;
    return interface;
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen1_read_status(phydev: *mut phy_device) -> c_int {
    static int aqr_gen1_read_status(struct phy_device *phydev)
    {
    phy_interface_t interface;
    int ret;
    int val;
    ret = aqr_read_status(phydev);
    if (ret)
    return ret;
    if (!phydev.link || phydev.autoneg == AUTONEG_DISABLE)
    return 0;
// The status register is not immediately correct on line side link up.
// Poll periodically until it reflects the correct ON state.
// Only return fail for read error, timeout defaults to OFF state.
//
    ret = phy_read_mmd_poll_timeout(phydev, MDIO_MMD_PHYXS,
    MDIO_PHYXS_VEND_IF_STATUS, val,
    (FIELD_GET(MDIO_PHYXS_VEND_IF_STATUS_TYPE_MASK, val) !=
    MDIO_PHYXS_VEND_IF_STATUS_TYPE_OFF),
    AQR107_OP_IN_PROG_SLEEP,
    AQR107_OP_IN_PROG_TIMEOUT, false);
    if (ret && ret != -ETIMEDOUT)
    return ret;
    switch (FIELD_GET(MDIO_PHYXS_VEND_IF_STATUS_TYPE_MASK, val)) {
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_KR:
    interface = PHY_INTERFACE_MODE_10GKR;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_KX:
    interface = PHY_INTERFACE_MODE_1000BASEKX;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_XFI:
    interface = PHY_INTERFACE_MODE_10GBASER;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_USXGMII:
    interface = PHY_INTERFACE_MODE_USXGMII;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_XAUI:
    interface = PHY_INTERFACE_MODE_XAUI;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_SGMII:
    interface = PHY_INTERFACE_MODE_SGMII;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_RXAUI:
    interface = PHY_INTERFACE_MODE_RXAUI;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_OCSGMII:
    interface = PHY_INTERFACE_MODE_2500BASEX;
    break;
    case MDIO_PHYXS_VEND_IF_STATUS_TYPE_OFF:
    default:
    phydev.link = false;
    interface = PHY_INTERFACE_MODE_NA;
    break;
    }
    phydev.interface = aqr_translate_interface(phydev, interface);
// Read rate from vendor register
    return aqr_gen1_read_rate(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen2_read_status(phydev: *mut phy_device) -> c_int {
    static int aqr_gen2_read_status(struct phy_device *phydev)
    {
    struct aqr107_priv *priv = phydev.priv;
    int i, ret;
    ret = aqr_gen1_read_status(phydev);
    if (ret)
    return ret;
    for (i = 0; i < AQR_NUM_GLOBAL_CFG; i++) {
    struct aqr_global_syscfg *syscfg = &priv.global_cfg[i];
    if (syscfg.speed != phydev.speed)
    continue;
    if (syscfg.rate_adapt == AQR_RATE_ADAPT_PAUSE)
    phydev.rate_matching = RATE_MATCH_PAUSE;
    else
    phydev.rate_matching = RATE_MATCH_NONE;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr107_get_downshift(phydev: *mut phy_device, data: *mut u8) -> c_int {
    static int aqr107_get_downshift(struct phy_device *phydev, u8 *data)
    {
    int val, cnt, enable;
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_VEND_PROV);
    if (val < 0)
    return val;
    enable = FIELD_GET(MDIO_AN_VEND_PROV_DOWNSHIFT_EN, val);
    cnt = FIELD_GET(MDIO_AN_VEND_PROV_DOWNSHIFT_MASK, val);
// data = enable && cnt ? cnt : DOWNSHIFT_DEV_DISABLE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr107_set_downshift(phydev: *mut phy_device, cnt: u8) -> c_int {
    static int aqr107_set_downshift(struct phy_device *phydev, u8 cnt)
    {
    let mut val: c_int = 0;
    if (!FIELD_FIT(MDIO_AN_VEND_PROV_DOWNSHIFT_MASK, cnt))
    return -E2BIG;
    if (cnt != DOWNSHIFT_DEV_DISABLE) {
    val = MDIO_AN_VEND_PROV_DOWNSHIFT_EN;
    val |= FIELD_PREP(MDIO_AN_VEND_PROV_DOWNSHIFT_MASK, cnt);
    }
    return phy_modify_mmd(phydev, MDIO_MMD_AN, MDIO_AN_VEND_PROV,
    MDIO_AN_VEND_PROV_DOWNSHIFT_EN |
    MDIO_AN_VEND_PROV_DOWNSHIFT_MASK, val);
    }
    static int aqr107_get_tunable(struct phy_device *phydev,
    struct ethtool_tunable *tuna, void *data)
    {
    switch (tuna.id) {
    case ETHTOOL_PHY_DOWNSHIFT:
    return aqr107_get_downshift(phydev, data);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int aqr107_set_tunable(struct phy_device *phydev,
    struct ethtool_tunable *tuna, const void *data)
    {
    switch (tuna.id) {
    case ETHTOOL_PHY_DOWNSHIFT:
    return aqr107_set_downshift(phydev, *(const u8 *)data);
    default:
    return -EOPNOTSUPP;
    }
    }
pub const AQR_FW_WAIT_SLEEP_US: c_int = 20000;
pub const AQR_FW_WAIT_TIMEOUT_US: c_int = 2000000;
// If we configure settings whilst firmware is still initializing the chip,
// then these settings may be overwritten. Therefore make sure chip
// initialization has completed. Use presence of the firmware ID as
// indicator for initialization having completed.
// The chip also provides a "reset completed" bit, but it's cleared after
// read. Therefore function would time out if called again.
//
#[no_mangle]
pub unsafe extern "C" fn aqr_wait_reset_complete(phydev: *mut phy_device) -> c_int {
    int aqr_wait_reset_complete(struct phy_device *phydev)
    {
    int ret, val;
    ret = read_poll_timeout(phy_read_mmd, val, val != 0,
    AQR_FW_WAIT_SLEEP_US, AQR_FW_WAIT_TIMEOUT_US,
    false, phydev, MDIO_MMD_VEND1,
    VEND1_GLOBAL_FW_ID);
    if (val < 0) {
    phydev_err(phydev, "Failed to read VEND1_GLOBAL_FW_ID: %pe\n",
    ERR_PTR(val));
    return val;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aqr_build_fingerprint(phydev: *mut phy_device) -> c_int {
    static int aqr_build_fingerprint(struct phy_device *phydev)
    {
    u8 fw_major, fw_minor, build_id, prov_id;
    struct aqr107_priv *priv = phydev.priv;
    u16 misc_id, misc_ver;
    int val;
    val = phy_read_mmd(phydev, MDIO_MMD_VEND1, VEND1_GLOBAL_FW_ID);
    if (val < 0)
    return val;
    fw_major = FIELD_GET(VEND1_GLOBAL_FW_ID_MAJOR, val);
    fw_minor = FIELD_GET(VEND1_GLOBAL_FW_ID_MINOR, val);
    val = phy_read_mmd(phydev, MDIO_MMD_VEND1, VEND1_GLOBAL_RSVD_STAT1);
    if (val < 0)
    return val;
    build_id = FIELD_GET(VEND1_GLOBAL_RSVD_STAT1_FW_BUILD_ID, val);
    prov_id = FIELD_GET(VEND1_GLOBAL_RSVD_STAT1_PROV_ID, val);
    val = phy_read_mmd(phydev, MDIO_MMD_PMAPMD, PMAPMD_FW_MISC_ID);
    if (val < 0)
    return val;
    misc_id = val;
    val = phy_read_mmd(phydev, MDIO_MMD_PMAPMD, PMAPMD_FW_MISC_VER);
    if (val < 0)
    return val;
    misc_ver = val;
    priv.fingerprint = AQR_FW_FINGERPRINT(fw_major, fw_minor, build_id,
    prov_id, misc_id, misc_ver);
    phydev_dbg(phydev, "FW %u.%u, Build %u, Provisioning %u, Misc ID %u, Version %u\n",
    fw_major, fw_minor, build_id, prov_id, misc_id, misc_ver);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr107_config_mdi(phydev: *mut phy_device) -> c_int {
    static int aqr107_config_mdi(struct phy_device *phydev)
    {
    struct device_node *np = phydev.mdio.dev.of_node;
    u32 mdi_conf;
    int ret;
    ret = of_property_read_u32(np, "marvell,mdi-cfg-order", &mdi_conf);
// Do nothing in case property "marvell,mdi-cfg-order" is not present
    if (ret == -EINVAL || ret == -ENOSYS)
    return 0;
    if (ret)
    return ret;
    if (mdi_conf & ~PMAPMD_RSVD_VEND_PROV_MDI_REVERSE)
    return -EINVAL;
    return phy_modify_mmd(phydev, MDIO_MMD_PMAPMD, PMAPMD_RSVD_VEND_PROV,
    PMAPMD_RSVD_VEND_PROV_MDI_CONF,
    mdi_conf | PMAPMD_RSVD_VEND_PROV_MDI_FORCE);
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen1_config_init(phydev: *mut phy_device) -> c_int {
    static int aqr_gen1_config_init(struct phy_device *phydev)
    {
    struct aqr107_priv *priv = phydev.priv;
    u32 led_idx;
    int ret;
// Check that the PHY interface type is compatible
    if (phydev.interface != PHY_INTERFACE_MODE_SGMII &&
    phydev.interface != PHY_INTERFACE_MODE_1000BASEKX &&
    phydev.interface != PHY_INTERFACE_MODE_2500BASEX &&
    phydev.interface != PHY_INTERFACE_MODE_XGMII &&
    phydev.interface != PHY_INTERFACE_MODE_USXGMII &&
    phydev.interface != PHY_INTERFACE_MODE_10G_QXGMII &&
    phydev.interface != PHY_INTERFACE_MODE_10GKR &&
    phydev.interface != PHY_INTERFACE_MODE_10GBASER &&
    phydev.interface != PHY_INTERFACE_MODE_XAUI &&
    phydev.interface != PHY_INTERFACE_MODE_RXAUI)
    return -ENODEV;
    WARN(phydev.interface == PHY_INTERFACE_MODE_XGMII,
    "Your devicetree is out of date, please update it. The AQR107 family doesn't support XGMII, maybe you mean USXGMII.\n");
    ret = aqr_wait_reset_complete(phydev);
    if (!ret) {
// The PHY might work without a firmware image, so only build a
// fingerprint if the firmware was initialized.
//
    ret = aqr_build_fingerprint(phydev);
    if (ret)
    return ret;
    }
    ret = aqr107_set_downshift(phydev, MDIO_AN_VEND_PROV_DOWNSHIFT_DFLT);
    if (ret)
    return ret;
    ret = aqr107_config_mdi(phydev);
    if (ret)
    return ret;
// Restore LED polarity state after reset
    for_each_set_bit(led_idx, &priv.leds_active_low, AQR_MAX_LEDS) {
    ret = aqr_phy_led_active_low_set(phydev, led_idx, true);
    if (ret)
    return ret;
    }
    for_each_set_bit(led_idx, &priv.leds_active_high, AQR_MAX_LEDS) {
    ret = aqr_phy_led_active_low_set(phydev, led_idx, false);
    if (ret)
    return ret;
    }
    return 0;
    }
// Walk the media-speed configuration registers to determine which
// host-side serdes modes may be used by the PHY depending on the
// negotiated media speed.
//
#[no_mangle]
unsafe extern "C" fn aqr_gen2_read_global_syscfg(phydev: *mut phy_device) -> c_int {
    static int aqr_gen2_read_global_syscfg(struct phy_device *phydev)
    {
    struct aqr107_priv *priv = phydev.priv;
    unsigned int serdes_mode, rate_adapt;
    phy_interface_t interface;
    int i, val;
    for (i = 0; i < AQR_NUM_GLOBAL_CFG; i++) {
    struct aqr_global_syscfg *syscfg = &priv.global_cfg[i];
    syscfg.speed = aqr_global_cfg_regs[i].speed;
    val = phy_read_mmd(phydev, MDIO_MMD_VEND1,
    aqr_global_cfg_regs[i].reg);
    if (val < 0)
    return val;
    serdes_mode = FIELD_GET(VEND1_GLOBAL_CFG_SERDES_MODE, val);
    rate_adapt = FIELD_GET(VEND1_GLOBAL_CFG_RATE_ADAPT, val);
    switch (serdes_mode) {
    case VEND1_GLOBAL_CFG_SERDES_MODE_XFI:
    if (rate_adapt == VEND1_GLOBAL_CFG_RATE_ADAPT_USX)
    interface = PHY_INTERFACE_MODE_USXGMII;
    else
    interface = PHY_INTERFACE_MODE_10GBASER;
    break;
    case VEND1_GLOBAL_CFG_SERDES_MODE_XFI5G:
    interface = PHY_INTERFACE_MODE_5GBASER;
    break;
    case VEND1_GLOBAL_CFG_SERDES_MODE_OCSGMII:
    interface = PHY_INTERFACE_MODE_2500BASEX;
    break;
    case VEND1_GLOBAL_CFG_SERDES_MODE_SGMII:
    interface = PHY_INTERFACE_MODE_SGMII;
    break;
    default:
    phydev_warn(phydev, "unrecognised serdes mode %u\n",
    serdes_mode);
    interface = PHY_INTERFACE_MODE_NA;
    break;
    }
    syscfg.interface = aqr_translate_interface(phydev, interface);
    switch (rate_adapt) {
    case VEND1_GLOBAL_CFG_RATE_ADAPT_NONE:
    syscfg.rate_adapt = AQR_RATE_ADAPT_NONE;
    break;
    case VEND1_GLOBAL_CFG_RATE_ADAPT_USX:
    syscfg.rate_adapt = AQR_RATE_ADAPT_USX;
    break;
    case VEND1_GLOBAL_CFG_RATE_ADAPT_PAUSE:
    syscfg.rate_adapt = AQR_RATE_ADAPT_PAUSE;
    break;
    default:
    phydev_warn(phydev, "unrecognized rate adapt mode %u\n",
    rate_adapt);
    break;
    }
    phydev_dbg(phydev,
    "Media speed %d uses host interface %s with %s\n",
    syscfg.speed, phy_modes(syscfg.interface),
    syscfg.rate_adapt == AQR_RATE_ADAPT_NONE ? "no rate adaptation" :
    syscfg.rate_adapt == AQR_RATE_ADAPT_PAUSE ? "rate adaptation through flow control" :
    syscfg.rate_adapt == AQR_RATE_ADAPT_USX ? "rate adaptation through symbol replication" :
    "unrecognized rate adaptation type");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen2_fill_interface_modes(phydev: *mut phy_device) -> c_int {
    static int aqr_gen2_fill_interface_modes(struct phy_device *phydev)
    {
    unsigned long *possible = phydev.possible_interfaces;
    struct aqr107_priv *priv = phydev.priv;
    phy_interface_t interface;
    int i, val, ret;
// It's been observed on some models that - when coming out of suspend
// - the FW signals that the PHY is ready but the GLOBAL_CFG registers
// continue on returning zeroes for some time. Let's poll the 100M
// register until it returns a real value as both 113c and 115c support
// this mode.
//
    if (priv.wait_on_global_cfg) {
    ret = phy_read_mmd_poll_timeout(phydev, MDIO_MMD_VEND1,
    VEND1_GLOBAL_CFG_100M, val,
    val != 0, 1000, 100000, false);
    if (ret)
    return ret;
    }
    ret = aqr_gen2_read_global_syscfg(phydev);
    if (ret)
    return ret;
    for (i = 0; i < AQR_NUM_GLOBAL_CFG; i++) {
    interface = priv.global_cfg[i].interface;
    if (interface != PHY_INTERFACE_MODE_NA)
    __set_bit(interface, possible);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen2_config_init(phydev: *mut phy_device) -> c_int {
    static int aqr_gen2_config_init(struct phy_device *phydev)
    {
    int ret;
    ret = aqr_gen1_config_init(phydev);
    if (ret)
    return ret;
    return aqr_gen2_fill_interface_modes(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen3_config_init(phydev: *mut phy_device) -> c_int {
    static int aqr_gen3_config_init(struct phy_device *phydev)
    {
    return aqr_gen2_config_init(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqcs109_config_init(phydev: *mut phy_device) -> c_int {
    static int aqcs109_config_init(struct phy_device *phydev)
    {
// Check that the PHY interface type is compatible
    if (phydev.interface != PHY_INTERFACE_MODE_SGMII &&
    phydev.interface != PHY_INTERFACE_MODE_2500BASEX)
    return -ENODEV;
    return aqr_gen2_config_init(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr107_link_change_notify(phydev: *mut phy_device) {
    static void aqr107_link_change_notify(struct phy_device *phydev)
    {
    u8 fw_major, fw_minor;
    bool downshift, short_reach, afr;
    int mode, val;
    if (phydev.state != PHY_RUNNING || phydev.autoneg == AUTONEG_DISABLE)
    return;
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_RX_LP_STAT1);
// call failed or link partner is no Aquantia PHY
    if (val < 0 || !(val & MDIO_AN_RX_LP_STAT1_AQ_PHY))
    return;
    short_reach = val & MDIO_AN_RX_LP_STAT1_SHORT_REACH;
    downshift = val & MDIO_AN_RX_LP_STAT1_AQRATE_DOWNSHIFT;
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_RX_LP_STAT4);
    if (val < 0)
    return;
    fw_major = FIELD_GET(MDIO_AN_RX_LP_STAT4_FW_MAJOR, val);
    fw_minor = FIELD_GET(MDIO_AN_RX_LP_STAT4_FW_MINOR, val);
    val = phy_read_mmd(phydev, MDIO_MMD_AN, MDIO_AN_RX_VEND_STAT3);
    if (val < 0)
    return;
    afr = val & MDIO_AN_RX_VEND_STAT3_AFR;
    phydev_dbg(phydev, "Link partner is Aquantia PHY, FW %u.%u%s%s%s\n",
    fw_major, fw_minor,
    short_reach ? ", short reach mode" : "",
    downshift ? ", fast-retrain downshift advertised" : "",
    afr ? ", fast reframe advertised" : "");
    val = phy_read_mmd(phydev, MDIO_MMD_VEND1, VEND1_GLOBAL_RSVD_STAT9);
    if (val < 0)
    return;
    mode = FIELD_GET(VEND1_GLOBAL_RSVD_STAT9_MODE, val);
    if (mode == VEND1_GLOBAL_RSVD_STAT9_1000BT2)
    phydev_info(phydev, "Aquantia 1000Base-T2 mode active\n");
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen1_wait_processor_intensive_op(phydev: *mut phy_device) -> c_int {
    static int aqr_gen1_wait_processor_intensive_op(struct phy_device *phydev)
    {
    int val, err;
// The datasheet notes to wait at least 1ms after issuing a
// processor intensive operation before checking.
// We cannot use the 'sleep_before_read' parameter of read_poll_timeout
// because that just determines the maximum time slept, not the minimum.
//
    usleep_range(1000, 5000);
    err = phy_read_mmd_poll_timeout(phydev, MDIO_MMD_VEND1,
    VEND1_GLOBAL_GEN_STAT2, val,
    !(val & VEND1_GLOBAL_GEN_STAT2_OP_IN_PROG),
    AQR107_OP_IN_PROG_SLEEP,
    AQR107_OP_IN_PROG_TIMEOUT, false);
    if (err) {
    phydev_err(phydev, "timeout: processor-intensive MDIO operation\n");
    return err;
    }
    return 0;
    }
    static int aqr_gen2_get_rate_matching(struct phy_device *phydev,
    phy_interface_t iface)
    {
    if (iface == PHY_INTERFACE_MODE_10GBASER ||
    iface == PHY_INTERFACE_MODE_2500BASEX)
    return RATE_MATCH_PAUSE;
    return RATE_MATCH_NONE;
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen1_suspend(phydev: *mut phy_device) -> c_int {
    static int aqr_gen1_suspend(struct phy_device *phydev)
    {
    int err;
    err = phy_set_bits_mmd(phydev, MDIO_MMD_VEND1, MDIO_CTRL1,
    MDIO_CTRL1_LPOWER);
    if (err)
    return err;
    return aqr_gen1_wait_processor_intensive_op(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen1_resume(phydev: *mut phy_device) -> c_int {
    static int aqr_gen1_resume(struct phy_device *phydev)
    {
    int err;
    err = phy_clear_bits_mmd(phydev, MDIO_MMD_VEND1, MDIO_CTRL1,
    MDIO_CTRL1_LPOWER);
    if (err)
    return err;
    return aqr_gen1_wait_processor_intensive_op(phydev);
    }
#[no_mangle]
unsafe extern "C" fn aqr115c_get_features(phydev: *mut phy_device) -> c_int {
    static int aqr115c_get_features(struct phy_device *phydev)
    {
    unsigned long *supported = phydev.supported;
// PHY supports speeds up to 2.5G with autoneg. PMA capabilities
// are not useful.
//
    linkmode_or(supported, supported, phy_gbit_features);
    linkmode_set_bit(ETHTOOL_LINK_MODE_2500baseT_Full_BIT, supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr111_get_features(phydev: *mut phy_device) -> c_int {
    static int aqr111_get_features(struct phy_device *phydev)
    {
// PHY supports speeds up to 5G with autoneg. PMA capabilities
// are not useful.
//
    aqr115c_get_features(phydev);
    linkmode_set_bit(ETHTOOL_LINK_MODE_5000baseT_Full_BIT,
    phydev.supported);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen4_config_init(phydev: *mut phy_device) -> c_int {
    static int aqr_gen4_config_init(struct phy_device *phydev)
    {
    struct aqr107_priv *priv = phydev.priv;
    int ret;
    priv.wait_on_global_cfg = true;
    ret = aqr_gen3_config_init(phydev);
    if (ret < 0)
    return ret;
    ret = phy_clear_bits_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_PMA_TXDIS,
    MDIO_PMD_TXDIS_GLOBAL);
    if (ret)
    return ret;
    return aqr_gen1_wait_processor_intensive_op(phydev);
    }
    static unsigned int aqr_gen2_inband_caps(struct phy_device *phydev,
    phy_interface_t interface)
    {
    if (interface == PHY_INTERFACE_MODE_SGMII ||
    interface == PHY_INTERFACE_MODE_USXGMII ||
    interface == PHY_INTERFACE_MODE_10G_QXGMII)
    return LINK_INBAND_ENABLE | LINK_INBAND_DISABLE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr_gen2_config_inband(phydev: *mut phy_device, modes: c_uint) -> c_int {
    static int aqr_gen2_config_inband(struct phy_device *phydev, unsigned int modes)
    {
    struct aqr107_priv *priv = phydev.priv;
    if (phydev.interface == PHY_INTERFACE_MODE_USXGMII ||
    phydev.interface == PHY_INTERFACE_MODE_10G_QXGMII) {
    let mut set: u16 = 0;
    if (modes == LINK_INBAND_ENABLE)
    set = MDIO_PHYXS_VEND_PROV2_USX_AN;
    return phy_modify_mmd(phydev, MDIO_MMD_PHYXS,
    MDIO_PHYXS_VEND_PROV2,
    MDIO_PHYXS_VEND_PROV2_USX_AN, set);
    }
    for (int i = 0; i < AQR_NUM_GLOBAL_CFG; i++) {
    struct aqr_global_syscfg *syscfg = &priv.global_cfg[i];
    let mut set: u16 = 0;
    int err;
    if (syscfg.interface != phydev.interface)
    continue;
    if (modes == LINK_INBAND_ENABLE)
    set = VEND1_GLOBAL_CFG_AUTONEG_ENA;
    err = phy_modify_mmd(phydev, MDIO_MMD_VEND1,
    aqr_global_cfg_regs[i].reg,
    VEND1_GLOBAL_CFG_AUTONEG_ENA, set);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr107_probe(phydev: *mut phy_device) -> c_int {
    static int aqr107_probe(struct phy_device *phydev)
    {
    int ret;
    phydev.priv = devm_kzalloc(&phydev.mdio.dev,
    sizeof(struct aqr107_priv), GFP_KERNEL);
    if (!phydev.priv)
    return -ENOMEM;
    ret = aqr_firmware_load(phydev);
    if (ret)
    return ret;
    return aqr_hwmon_probe(phydev);
    }
    static struct phy_driver aqr_driver[] = {
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQ1202),
    .name		= "Aquantia AQ1202",
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_read_status,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQ2104),
    .name		= "Aquantia AQ2104",
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_read_status,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR105),
    .name		= "Aquantia AQR105",
    .get_features	= aqr105_get_features,
    .probe		= aqr107_probe,
    .config_init	= aqr_gen1_config_init,
    .config_aneg    = aqr105_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen1_read_status,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR106),
    .name		= "Aquantia AQR106",
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_read_status,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR107),
    .name		= "Aquantia AQR107",
    .probe		= aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init	= aqr_gen2_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .get_sset_count	= aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQCS109),
    .name		= "Aquantia AQCS109",
    .probe		= aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init	= aqcs109_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .get_sset_count	= aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .get_features   = aqr115c_get_features,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR111),
    .name		= "Aquantia AQR111",
    .probe		= aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init	= aqr_gen3_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .get_sset_count	= aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .get_features   = aqr111_get_features,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR111B0),
    .name		= "Aquantia AQR111B0",
    .probe		= aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init	= aqr_gen3_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .get_sset_count	= aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .get_features   = aqr111_get_features,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR405),
    .name		= "Aquantia AQR405",
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_read_status,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR112),
    .name		= "Aquantia AQR112",
    .probe		= aqr107_probe,
    .config_init	= aqr_gen3_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .read_status	= aqr_gen2_read_status,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR412),
    .name		= "Aquantia AQR412",
    .probe		= aqr107_probe,
    .config_init	= aqr_gen3_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .read_status	= aqr_gen2_read_status,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR412C),
    .name		= "Aquantia AQR412C",
    .probe		= aqr107_probe,
    .config_init	= aqr_gen3_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .read_status	= aqr_gen2_read_status,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR113),
    .name		= "Aquantia AQR113",
    .probe          = aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init    = aqr_gen4_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr    = aqr_config_intr,
    .handle_interrupt       = aqr_handle_interrupt,
    .read_status    = aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend        = aqr_gen1_suspend,
    .resume         = aqr_gen1_resume,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings    = aqr107_get_strings,
    .get_stats      = aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR113C),
    .name           = "Aquantia AQR113C",
    .probe          = aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init    = aqr_gen4_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr    = aqr_config_intr,
    .handle_interrupt       = aqr_handle_interrupt,
    .read_status    = aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend        = aqr_gen1_suspend,
    .resume         = aqr_gen1_resume,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings    = aqr107_get_strings,
    .get_stats      = aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps    = aqr_gen2_inband_caps,
    .config_inband  = aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR114C),
    .name           = "Aquantia AQR114C",
    .probe          = aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init    = aqr_gen4_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr    = aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status    = aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend        = aqr_gen1_suspend,
    .resume         = aqr_gen1_resume,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings    = aqr107_get_strings,
    .get_stats      = aqr107_get_stats,
    .get_features   = aqr111_get_features,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps    = aqr_gen2_inband_caps,
    .config_inband  = aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR115),
    .name		= "Aquantia AQR115",
    .probe		= aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init	= aqr_gen4_config_init,
    .config_aneg	= aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .get_sset_count	= aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .get_features	= aqr115c_get_features,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR115C),
    .name           = "Aquantia AQR115C",
    .probe          = aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init    = aqr_gen4_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr    = aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status    = aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend        = aqr_gen1_suspend,
    .resume         = aqr_gen1_resume,
    .get_sset_count = aqr107_get_sset_count,
    .get_strings    = aqr107_get_strings,
    .get_stats      = aqr107_get_stats,
    .get_features   = aqr115c_get_features,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    {
    PHY_ID_MATCH_MODEL(PHY_ID_AQR813),
    .name		= "Aquantia AQR813",
    .probe		= aqr107_probe,
    .get_rate_matching = aqr_gen2_get_rate_matching,
    .config_init	= aqr_gen4_config_init,
    .config_aneg    = aqr_config_aneg,
    .config_intr	= aqr_config_intr,
    .handle_interrupt = aqr_handle_interrupt,
    .read_status	= aqr_gen2_read_status,
    .get_tunable    = aqr107_get_tunable,
    .set_tunable    = aqr107_set_tunable,
    .suspend	= aqr_gen1_suspend,
    .resume		= aqr_gen1_resume,
    .get_sset_count	= aqr107_get_sset_count,
    .get_strings	= aqr107_get_strings,
    .get_stats	= aqr107_get_stats,
    .link_change_notify = aqr107_link_change_notify,
    .led_brightness_set = aqr_phy_led_brightness_set,
    .led_hw_is_supported = aqr_phy_led_hw_is_supported,
    .led_hw_control_set = aqr_phy_led_hw_control_set,
    .led_hw_control_get = aqr_phy_led_hw_control_get,
    .led_polarity_set = aqr_phy_led_polarity_set,
    .inband_caps	= aqr_gen2_inband_caps,
    .config_inband	= aqr_gen2_config_inband,
    },
    };
    module_phy_driver(aqr_driver);
    static const struct mdio_device_id __maybe_unused aqr_tbl[] = {
    { PHY_ID_MATCH_MODEL(PHY_ID_AQ1202) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQ2104) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR105) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR106) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR107) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQCS109) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR405) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR111) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR111B0) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR112) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR412) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR412C) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR113) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR113C) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR114C) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR115) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR115C) },
    { PHY_ID_MATCH_MODEL(PHY_ID_AQR813) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, aqr_tbl);
    MODULE_DESCRIPTION("Aquantia PHY driver");
    MODULE_AUTHOR("Shaohui Xie <Shaohui.Xie@freescale.com>");
    MODULE_LICENSE("GPL v2");
