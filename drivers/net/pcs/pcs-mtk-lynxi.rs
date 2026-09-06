//! Automatically rewritten from C to Rust
//! Source: drivers/net/pcs/pcs-mtk-lynxi.c
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
// Copyright (c) 2018-2019 MediaTek Inc.
// A library for MediaTek SGMII circuit
//
// Author: Sean Wang <sean.wang@mediatek.com>
// Author: Alexander Couzens <lynxis@fe80.eu>
// Author: Daniel Golle <daniel@makrotopia.org>
//

// SGMII subsystem config registers
// BMCR (low 16) BMSR (high 16)
pub const SGMSYS_PCS_CONTROL_1: c_uint = 0x0;

pub const SGMSYS_PCS_DEVICE_ID: c_uint = 0x4;
pub const SGMII_LYNXI_DEV_ID: c_uint = 0x4d544950;
pub const SGMSYS_PCS_ADVERTISE: c_uint = 0x8;

pub const SGMSYS_PCS_SCRATCH: c_uint = 0x14;

// Register to programmable link timer, the unit in 2 * 8ns
pub const SGMSYS_PCS_LINK_TIMER: c_uint = 0x18;

    ((ns) / 2 / 8))
// Register to control remote fault
pub const SGMSYS_SGMII_MODE: c_uint = 0x20;

// Register to reset SGMII design
pub const SGMSYS_RESERVED_0: c_uint = 0x34;

// Register to set SGMII speed, ANA RG_ Control Signals III

// Register to power up QPHY
pub const SGMSYS_QPHY_PWR_STATE_CTRL: c_uint = 0xe8;

// Register to QPHY wrapper control
pub const SGMSYS_QPHY_WRAP_CTRL: c_uint = 0xec;

// struct mtk_pcs_lynxi -  This structure holds each sgmii regmap andassociated
// data
// @regmap:                The register map pointing at the range used to setup
// SGMII modes
// @dev:                   Pointer to device owning the PCS
// @ana_rgc3:              The offset of register ANA_RGC3 relative to regmap
// @interface:             Currently configured interface mode
// @pcs:                   Phylink PCS structure
// @flags:                 Flags indicating hardware properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pcs_lynxi {
    pub regmap: *mut regmap,
    pub ana_rgc3: u32,
    pub interface: phy_interface_t,
    pub pcs: phylink_pcs,
    pub flags: u32,
    pub fwnode: *mut fwnode_handle,
}

    static struct mtk_pcs_lynxi *pcs_to_mtk_pcs_lynxi(struct phylink_pcs *pcs)
    {
    return container_of(pcs, struct mtk_pcs_lynxi, pcs);
    }
    static unsigned int mtk_pcs_lynxi_inband_caps(struct phylink_pcs *pcs,
    phy_interface_t interface)
    {
    switch (interface) {
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_2500BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    return LINK_INBAND_DISABLE | LINK_INBAND_ENABLE;
    default:
    return 0;
    }
    }
    static void mtk_pcs_lynxi_get_state(struct phylink_pcs *pcs,
    unsigned int neg_mode,
    struct phylink_link_state *state)
    {
    struct mtk_pcs_lynxi *mpcs = pcs_to_mtk_pcs_lynxi(pcs);
    unsigned int bm, adv;
// Read the BMSR and LPA
    if (regmap_read(mpcs.regmap, SGMSYS_PCS_CONTROL_1, &bm) ||
    regmap_read(mpcs.regmap, SGMSYS_PCS_ADVERTISE, &adv)) {
    state.link = false;
    return;
    }
    phylink_mii_c22_pcs_decode_state(state, neg_mode,
    FIELD_GET(SGMII_BMSR, bm),
    FIELD_GET(SGMII_LPA, adv));
    }
    static int mtk_pcs_config_polarity(struct mtk_pcs_lynxi *mpcs,
    phy_interface_t interface)
    {
    struct fwnode_handle *fwnode = mpcs.fwnode, *pcs_fwnode;
    unsigned int pol, default_pol = PHY_POL_NORMAL;
    let mut val: c_uint = 0;
    int ret;
    if (!fwnode)
    return 0;
    if (fwnode_property_read_bool(fwnode, "mediatek,pnswap"))
    default_pol = PHY_POL_INVERT;
    pcs_fwnode = fwnode_get_named_child_node(fwnode, "pcs");
    ret = phy_get_rx_polarity(pcs_fwnode, phy_modes(interface),
    BIT(PHY_POL_NORMAL) | BIT(PHY_POL_INVERT),
    default_pol, &pol);
    if (ret) {
    fwnode_handle_put(pcs_fwnode);
    return ret;
    }
    if (pol == PHY_POL_INVERT)
    val |= SGMII_PN_SWAP_RX;
    ret = phy_get_tx_polarity(pcs_fwnode, phy_modes(interface),
    BIT(PHY_POL_NORMAL) | BIT(PHY_POL_INVERT),
    default_pol, &pol);
    fwnode_handle_put(pcs_fwnode);
    if (ret)
    return ret;
    if (pol == PHY_POL_INVERT)
    val |= SGMII_PN_SWAP_TX;
    return regmap_update_bits(mpcs.regmap, SGMSYS_QPHY_WRAP_CTRL,
    SGMII_PN_SWAP_RX | SGMII_PN_SWAP_TX, val);
    }
    static int mtk_pcs_lynxi_config(struct phylink_pcs *pcs, unsigned int neg_mode,
    phy_interface_t interface,
    const unsigned long *advertising,
    bool permit_pause_to_mac)
    {
    struct mtk_pcs_lynxi *mpcs = pcs_to_mtk_pcs_lynxi(pcs);
    let mut mode_changed: bool = false, changed;
    unsigned int rgc3, sgm_mode, bmcr;
    int advertise, link_timer;
    int ret;
    advertise = phylink_mii_c22_pcs_encode_advertisement(interface,
    advertising);
    if (advertise < 0)
    return advertise;
// Clearing IF_MODE_BIT0 switches the PCS to BASE-X mode, and
// we assume that fixes it's speed at bitrate = line rate (in
// other words, 1000Mbps or 2500Mbps).
//
    if (interface == PHY_INTERFACE_MODE_SGMII)
    sgm_mode = SGMII_IF_MODE_SGMII;
    else
    sgm_mode = 0;
    if (neg_mode & PHYLINK_PCS_NEG_INBAND)
    sgm_mode |= SGMII_REMOTE_FAULT_DIS;
    if (neg_mode == PHYLINK_PCS_NEG_INBAND_ENABLED) {
    if (interface == PHY_INTERFACE_MODE_SGMII)
    sgm_mode |= SGMII_SPEED_DUPLEX_AN;
    bmcr = BMCR_ANENABLE;
    } else {
    bmcr = 0;
    }
    if (mpcs.interface != interface) {
    link_timer = phylink_get_link_timer_ns(interface);
    if (link_timer < 0)
    return link_timer;
// PHYA power down
    regmap_set_bits(mpcs.regmap, SGMSYS_QPHY_PWR_STATE_CTRL,
    SGMII_PHYA_PWD);
// Reset SGMII PCS state
    regmap_set_bits(mpcs.regmap, SGMSYS_RESERVED_0,
    SGMII_SW_RESET);
    ret = mtk_pcs_config_polarity(mpcs, interface);
    if (ret)
    return ret;
    if (interface == PHY_INTERFACE_MODE_2500BASEX)
    rgc3 = SGMII_PHY_SPEED_3_125G;
    else
    rgc3 = SGMII_PHY_SPEED_1_25G;
// Configure the underlying interface speed
    regmap_update_bits(mpcs.regmap, mpcs.ana_rgc3,
    SGMII_PHY_SPEED_MASK, rgc3);
// Setup the link timer
    regmap_write(mpcs.regmap, SGMSYS_PCS_LINK_TIMER,
    SGMII_LINK_TIMER_VAL(link_timer));
    mpcs.interface = interface;
    mode_changed = true;
    }
// Update the advertisement, noting whether it has changed
    regmap_update_bits_check(mpcs.regmap, SGMSYS_PCS_ADVERTISE,
    SGMII_ADVERTISE, advertise, &changed);
// Update the sgmsys mode register
    regmap_update_bits(mpcs.regmap, SGMSYS_SGMII_MODE,
    SGMII_REMOTE_FAULT_DIS | SGMII_SPEED_DUPLEX_AN |
    SGMII_IF_MODE_SGMII, sgm_mode);
// Update the BMCR
    regmap_update_bits(mpcs.regmap, SGMSYS_PCS_CONTROL_1,
    BMCR_ANENABLE, bmcr);
// Release PHYA power down state
// Only removing bit SGMII_PHYA_PWD isn't enough.
// There are cases when the SGMII_PHYA_PWD register contains 0x9 which
// prevents SGMII from working. The SGMII still shows link but no traffic
// can flow. Writing 0x0 to the PHYA_PWD register fix the issue. 0x0 was
// taken from a good working state of the SGMII interface.
// Unknown how much the QPHY needs but it is racy without a sleep.
// Tested on mt7622 & mt7986.
//
    usleep_range(50, 100);
    regmap_write(mpcs.regmap, SGMSYS_QPHY_PWR_STATE_CTRL, 0);
    return changed || mode_changed;
    }
#[no_mangle]
unsafe extern "C" fn mtk_pcs_lynxi_restart_an(pcs: *mut phylink_pcs) {
    static void mtk_pcs_lynxi_restart_an(struct phylink_pcs *pcs)
    {
    struct mtk_pcs_lynxi *mpcs = pcs_to_mtk_pcs_lynxi(pcs);
    regmap_set_bits(mpcs.regmap, SGMSYS_PCS_CONTROL_1, BMCR_ANRESTART);
    }
    static void mtk_pcs_lynxi_link_up(struct phylink_pcs *pcs,
    unsigned int neg_mode,
    phy_interface_t interface, int speed,
    int duplex)
    {
    struct mtk_pcs_lynxi *mpcs = pcs_to_mtk_pcs_lynxi(pcs);
    unsigned int sgm_mode;
    if (neg_mode != PHYLINK_PCS_NEG_INBAND_ENABLED) {
// Force the speed and duplex setting
    if (speed == SPEED_10)
    sgm_mode = SGMII_SPEED_10;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: speed ==) -> else {
    else if (speed == SPEED_100)
    sgm_mode = SGMII_SPEED_100;
    else
    sgm_mode = SGMII_SPEED_1000;
    if (duplex != DUPLEX_FULL)
    sgm_mode |= SGMII_DUPLEX_HALF;
    regmap_update_bits(mpcs.regmap, SGMSYS_SGMII_MODE,
    SGMII_DUPLEX_HALF | SGMII_SPEED_MASK,
    sgm_mode);
    }
    }
#[no_mangle]
unsafe extern "C" fn mtk_pcs_lynxi_disable(pcs: *mut phylink_pcs) {
    static void mtk_pcs_lynxi_disable(struct phylink_pcs *pcs)
    {
    struct mtk_pcs_lynxi *mpcs = pcs_to_mtk_pcs_lynxi(pcs);
    mpcs.interface = PHY_INTERFACE_MODE_NA;
    }
    static const struct phylink_pcs_ops mtk_pcs_lynxi_ops = {
    .pcs_inband_caps = mtk_pcs_lynxi_inband_caps,
    .pcs_get_state = mtk_pcs_lynxi_get_state,
    .pcs_config = mtk_pcs_lynxi_config,
    .pcs_an_restart = mtk_pcs_lynxi_restart_an,
    .pcs_link_up = mtk_pcs_lynxi_link_up,
    .pcs_disable = mtk_pcs_lynxi_disable,
    };
    struct phylink_pcs *mtk_pcs_lynxi_create(struct device *dev,
    struct fwnode_handle *fwnode,
    struct regmap *regmap, u32 ana_rgc3)
    {
    struct mtk_pcs_lynxi *mpcs;
    u32 id, ver;
    int ret;
    ret = regmap_read(regmap, SGMSYS_PCS_DEVICE_ID, &id);
    if (ret < 0)
    return core::ptr::null_mut();
    if (id != SGMII_LYNXI_DEV_ID) {
    dev_err(dev, "unknown PCS device id %08x\n", id);
    return core::ptr::null_mut();
    }
    ret = regmap_read(regmap, SGMSYS_PCS_SCRATCH, &ver);
    if (ret < 0)
    return core::ptr::null_mut();
    ver = FIELD_GET(SGMII_DEV_VERSION, ver);
    if (ver != 0x1) {
    dev_err(dev, "unknown PCS device version %04x\n", ver);
    return core::ptr::null_mut();
    }
    dev_dbg(dev, "MediaTek LynxI SGMII PCS (id 0x%08x, ver 0x%04x)\n", id,
    ver);
    mpcs = kzalloc_obj(*mpcs);
    if (!mpcs)
    return core::ptr::null_mut();
    mpcs.ana_rgc3 = ana_rgc3;
    mpcs.regmap = regmap;
    mpcs.pcs.ops = &mtk_pcs_lynxi_ops;
    mpcs.pcs.poll = true;
    mpcs.interface = PHY_INTERFACE_MODE_NA;
    mpcs.fwnode = fwnode_handle_get(fwnode);
    __set_bit(PHY_INTERFACE_MODE_SGMII, mpcs.pcs.supported_interfaces);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, mpcs.pcs.supported_interfaces);
    __set_bit(PHY_INTERFACE_MODE_2500BASEX, mpcs.pcs.supported_interfaces);
    return &mpcs.pcs;
    }
    EXPORT_SYMBOL(mtk_pcs_lynxi_create);
#[no_mangle]
pub unsafe extern "C" fn mtk_pcs_lynxi_destroy(pcs: *mut phylink_pcs) {
    void mtk_pcs_lynxi_destroy(struct phylink_pcs *pcs)
    {
    struct mtk_pcs_lynxi *mpcs;
    if (!pcs)
    return;
    mpcs = pcs_to_mtk_pcs_lynxi(pcs);
    fwnode_handle_put(mpcs.fwnode);
    kfree(mpcs);
    }
    EXPORT_SYMBOL(mtk_pcs_lynxi_destroy);
    MODULE_DESCRIPTION("MediaTek SGMII library for LynxI");
    MODULE_LICENSE("GPL");
