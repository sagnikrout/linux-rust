//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/adin1100.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Driver for Analog Devices Industrial Ethernet T1L PHYs
//
// Copyright 2020 Analog Devices Inc.
//

pub const PHY_ID_ADIN1100: c_uint = 0x0283bc81;
pub const PHY_ID_ADIN1110: c_uint = 0x0283bc91;
pub const PHY_ID_ADIN2111: c_uint = 0x0283bca1;
pub const ADIN_PHY_SUBSYS_IRQ_MASK: c_uint = 0x0021;

pub const ADIN_PHY_SUBSYS_IRQ_STATUS: c_uint = 0x0011;

pub const ADIN_FORCED_MODE: c_uint = 0x8000;

pub const ADIN_CRSM_SFT_RST: c_uint = 0x8810;

pub const ADIN_CRSM_SFT_PD_CNTRL: c_uint = 0x8812;

pub const ADIN_AN_PHY_INST_STATUS: c_uint = 0x8030;

pub const ADIN_CRSM_STAT: c_uint = 0x8818;

pub const ADIN_MSE_VAL: c_uint = 0x830B;
pub const ADIN_SQI_MAX: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adin_mse_sqi_range {
    pub start: u16,
    pub end: u16,
}

    static const struct adin_mse_sqi_range adin_mse_sqi_map[] = {
    { 0x0A74, 0xFFFF },
    { 0x084E, 0x0A74 },
    { 0x0698, 0x084E },
    { 0x053D, 0x0698 },
    { 0x0429, 0x053D },
    { 0x034E, 0x0429 },
    { 0x02A0, 0x034E },
    { 0x0000, 0x02A0 },
    };
//
// struct adin_priv - ADIN PHY driver private data
// @tx_level_2v4_able:		set if the PHY supports 2.4V TX levels (10BASE-T1L)
// @tx_level_2v4:		set if the PHY requests 2.4V TX levels (10BASE-T1L)
// @tx_level_prop_present:	set if the TX level is specified in DT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adin_priv {
    pub tx_level_2v4_able:1: c_uint,
    pub tx_level_2v4:1: c_uint,
    pub tx_level_prop_present:1: c_uint,
}

#[no_mangle]
unsafe extern "C" fn adin_read_status(phydev: *mut phy_device) -> c_int {
    static int adin_read_status(struct phy_device *phydev)
    {
    int ret;
    ret = genphy_c45_read_status(phydev);
    if (ret)
    return ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_AN, ADIN_AN_PHY_INST_STATUS);
    if (ret < 0)
    return ret;
    if (ret & ADIN_IS_CFG_SLV)
    phydev.master_slave_state = MASTER_SLAVE_STATE_SLAVE;
    if (ret & ADIN_IS_CFG_MST)
    phydev.master_slave_state = MASTER_SLAVE_STATE_MASTER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adin_config_aneg(phydev: *mut phy_device) -> c_int {
    static int adin_config_aneg(struct phy_device *phydev)
    {
    struct adin_priv *priv = phydev.priv;
    int ret;
    if (phydev.autoneg == AUTONEG_DISABLE) {
    ret = genphy_c45_pma_setup_forced(phydev);
    if (ret < 0)
    return ret;
    if (priv.tx_level_prop_present && priv.tx_level_2v4)
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_B10L_PMA_CTRL,
    MDIO_PMA_10T1L_CTRL_2V4_EN);
    else
    ret = phy_clear_bits_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_B10L_PMA_CTRL,
    MDIO_PMA_10T1L_CTRL_2V4_EN);
    if (ret < 0)
    return ret;
// Force PHY to use above configurations
    return phy_set_bits_mmd(phydev, MDIO_MMD_AN, ADIN_FORCED_MODE, ADIN_FORCED_MODE_EN);
    }
    ret = phy_clear_bits_mmd(phydev, MDIO_MMD_AN, ADIN_FORCED_MODE, ADIN_FORCED_MODE_EN);
    if (ret < 0)
    return ret;
// Request increased transmit level from LP.
    if (priv.tx_level_prop_present && priv.tx_level_2v4) {
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_AN, MDIO_AN_T1_ADV_H,
    MDIO_AN_T1_ADV_H_10L_TX_HI |
    MDIO_AN_T1_ADV_H_10L_TX_HI_REQ);
    if (ret < 0)
    return ret;
    }
// Disable 2.4 Vpp transmit level.
    if ((priv.tx_level_prop_present && !priv.tx_level_2v4) || !priv.tx_level_2v4_able) {
    ret = phy_clear_bits_mmd(phydev, MDIO_MMD_AN, MDIO_AN_T1_ADV_H,
    MDIO_AN_T1_ADV_H_10L_TX_HI |
    MDIO_AN_T1_ADV_H_10L_TX_HI_REQ);
    if (ret < 0)
    return ret;
    }
    return genphy_c45_config_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn adin_phy_ack_intr(phydev: *mut phy_device) -> c_int {
    static int adin_phy_ack_intr(struct phy_device *phydev)
    {
// Clear pending interrupts
    int rc = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    ADIN_PHY_SUBSYS_IRQ_STATUS);
    return rc < 0 ? rc : 0;
    }
#[no_mangle]
unsafe extern "C" fn adin_config_intr(phydev: *mut phy_device) -> c_int {
    static int adin_config_intr(struct phy_device *phydev)
    {
    u16 irq_mask;
    int ret;
    ret = adin_phy_ack_intr(phydev);
    if (ret)
    return ret;
    if (phydev.interrupts == PHY_INTERRUPT_ENABLED)
    irq_mask = ADIN_LINK_STAT_CHNG_IRQ_EN;
    else
    irq_mask = 0;
    return phy_modify_mmd(phydev, MDIO_MMD_VEND2,
    ADIN_PHY_SUBSYS_IRQ_MASK,
    ADIN_LINK_STAT_CHNG_IRQ_EN, irq_mask);
    }
#[no_mangle]
unsafe extern "C" fn adin_phy_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t {
    static irqreturn_t adin_phy_handle_interrupt(struct phy_device *phydev)
    {
    int irq_status;
    irq_status = phy_read_mmd(phydev, MDIO_MMD_VEND2,
    ADIN_PHY_SUBSYS_IRQ_STATUS);
    if (irq_status < 0) {
    phy_error(phydev);
    return IRQ_NONE;
    }
    if (!(irq_status & ADIN_LINK_STAT_CHNG))
    return IRQ_NONE;
    phy_trigger_machine(phydev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adin_set_powerdown_mode(phydev: *mut phy_device, en: bool) -> c_int {
    static int adin_set_powerdown_mode(struct phy_device *phydev, bool en)
    {
    int ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND1,
    ADIN_CRSM_SFT_PD_CNTRL,
    en ? ADIN_CRSM_SFT_PD_CNTRL_EN : 0);
    if (ret < 0)
    return ret;
    return phy_read_mmd_poll_timeout(phydev, MDIO_MMD_VEND1, ADIN_CRSM_STAT, ret,
    !!(ret & ADIN_CRSM_SFT_PD_RDY) == en,
    1000, 30000, true);
    }
#[no_mangle]
unsafe extern "C" fn adin_suspend(phydev: *mut phy_device) -> c_int {
    static int adin_suspend(struct phy_device *phydev)
    {
    return adin_set_powerdown_mode(phydev, true);
    }
#[no_mangle]
unsafe extern "C" fn adin_resume(phydev: *mut phy_device) -> c_int {
    static int adin_resume(struct phy_device *phydev)
    {
    return adin_set_powerdown_mode(phydev, false);
    }
#[no_mangle]
unsafe extern "C" fn adin_set_loopback(phydev: *mut phy_device, enable: bool, speed: c_int) -> c_int {
    static int adin_set_loopback(struct phy_device *phydev, bool enable, int speed)
    {
    if (enable && speed)
    return -EOPNOTSUPP;
    if (enable)
    return phy_set_bits_mmd(phydev, MDIO_MMD_PCS, MDIO_PCS_10T1L_CTRL,
    BMCR_LOOPBACK);
// PCS loopback (according to 10BASE-T1L spec)
    return phy_clear_bits_mmd(phydev, MDIO_MMD_PCS, MDIO_PCS_10T1L_CTRL,
    BMCR_LOOPBACK);
    }
#[no_mangle]
unsafe extern "C" fn adin_soft_reset(phydev: *mut phy_device) -> c_int {
    static int adin_soft_reset(struct phy_device *phydev)
    {
    int ret;
    ret = phy_set_bits_mmd(phydev, MDIO_MMD_VEND1, ADIN_CRSM_SFT_RST, ADIN_CRSM_SFT_RST_EN);
    if (ret < 0)
    return ret;
    return phy_read_mmd_poll_timeout(phydev, MDIO_MMD_VEND1, ADIN_CRSM_STAT, ret,
    (ret & ADIN_CRSM_SYS_RDY),
    10000, 30000, true);
    }
#[no_mangle]
unsafe extern "C" fn adin_get_features(phydev: *mut phy_device) -> c_int {
    static int adin_get_features(struct phy_device *phydev)
    {
    struct adin_priv *priv = phydev.priv;
    struct device *dev = &phydev.mdio.dev;
    int ret;
    u8 val;
    ret = phy_read_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_PMA_10T1L_STAT);
    if (ret < 0)
    return ret;
// This depends on the voltage level from the power source
    priv.tx_level_2v4_able = !!(ret & MDIO_PMA_10T1L_STAT_2V4_ABLE);
    phydev_dbg(phydev, "PHY supports 2.4V TX level: %s\n",
    priv.tx_level_2v4_able ? "yes" : "no");
    priv.tx_level_prop_present = device_property_present(dev, "phy-10base-t1l-2.4vpp");
    if (priv.tx_level_prop_present) {
    ret = device_property_read_u8(dev, "phy-10base-t1l-2.4vpp", &val);
    if (ret < 0)
    return ret;
    priv.tx_level_2v4 = val;
    if (!priv.tx_level_2v4 && priv.tx_level_2v4_able)
    phydev_info(phydev,
    "PHY supports 2.4V TX level, but disabled via config\n");
    }
    linkmode_set_bit_array(phy_basic_ports_array, ARRAY_SIZE(phy_basic_ports_array),
    phydev.supported);
    return genphy_c45_pma_read_abilities(phydev);
    }
#[no_mangle]
unsafe extern "C" fn adin_get_sqi(phydev: *mut phy_device) -> c_int {
    static int adin_get_sqi(struct phy_device *phydev)
    {
    u16 mse_val;
    int sqi;
    int ret;
    ret = phy_read_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_STAT1);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(MDIO_STAT1_LSTATUS): !(ret &) -> else {
    else if (!(ret & MDIO_STAT1_LSTATUS))
    return 0;
    ret = phy_read_mmd(phydev, MDIO_STAT1, ADIN_MSE_VAL);
    if (ret < 0)
    return ret;
    mse_val = 0xFFFF & ret;
    for (sqi = 0; sqi < ARRAY_SIZE(adin_mse_sqi_map); sqi++) {
    if (mse_val >= adin_mse_sqi_map[sqi].start && mse_val <= adin_mse_sqi_map[sqi].end)
    return sqi;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn adin_get_sqi_max(phydev: *mut phy_device) -> c_int {
    static int adin_get_sqi_max(struct phy_device *phydev)
    {
    return ADIN_SQI_MAX;
    }
#[no_mangle]
unsafe extern "C" fn adin_probe(phydev: *mut phy_device) -> c_int {
    static int adin_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct adin_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    phydev.priv = priv;
    return 0;
    }
    static struct phy_driver adin_driver[] = {
    {
    .phy_id			= PHY_ID_ADIN1100,
    .phy_id_mask		= 0xffffffcf,
    .name			= "ADIN1100",
    .get_features		= adin_get_features,
    .soft_reset		= adin_soft_reset,
    .probe			= adin_probe,
    .config_aneg		= adin_config_aneg,
    .read_status		= adin_read_status,
    .config_intr		= adin_config_intr,
    .handle_interrupt	= adin_phy_handle_interrupt,
    .set_loopback		= adin_set_loopback,
    .suspend		= adin_suspend,
    .resume			= adin_resume,
    .get_sqi		= adin_get_sqi,
    .get_sqi_max		= adin_get_sqi_max,
    },
    };
    module_phy_driver(adin_driver);
    static const struct mdio_device_id __maybe_unused adin_tbl[] = {
    { PHY_ID_MATCH_MODEL(PHY_ID_ADIN1100) },
    { PHY_ID_MATCH_MODEL(PHY_ID_ADIN1110) },
    { PHY_ID_MATCH_MODEL(PHY_ID_ADIN2111) },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, adin_tbl);
    MODULE_DESCRIPTION("Analog Devices Industrial Ethernet T1L PHY driver");
    MODULE_LICENSE("Dual BSD/GPL");
