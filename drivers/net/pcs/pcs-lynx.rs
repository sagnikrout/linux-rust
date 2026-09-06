//! Automatically rewritten from C to Rust
//! Source: drivers/net/pcs/pcs-lynx.c
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
// Copyright 2020 NXP
// Lynx PCS MDIO helpers
//

pub const LINK_TIMER_LO: c_uint = 0x12;
pub const LINK_TIMER_HI: c_uint = 0x13;
pub const IF_MODE: c_uint = 0x14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lynx_pcs {
    pub pcs: phylink_pcs,
    pub mdio: *mut mdio_device,
}

    enum sgmii_speed {
    SGMII_SPEED_10		= 0,
    SGMII_SPEED_100		= 1,
    SGMII_SPEED_1000	= 2,
    SGMII_SPEED_2500	= 2,
    };

    static unsigned int lynx_pcs_inband_caps(struct phylink_pcs *pcs,
    phy_interface_t interface)
    {
    switch (interface) {
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_2500BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_QSGMII:
    return LINK_INBAND_DISABLE | LINK_INBAND_ENABLE;
    case PHY_INTERFACE_MODE_10GBASER:
    return LINK_INBAND_DISABLE;
    case PHY_INTERFACE_MODE_USXGMII:
    case PHY_INTERFACE_MODE_10G_QXGMII:
    return LINK_INBAND_ENABLE;
    default:
    return 0;
    }
    }
    static void lynx_pcs_get_state_usxgmii(struct mdio_device *pcs,
    struct phylink_link_state *state)
    {
    struct mii_bus *bus = pcs.bus;
    let mut addr: c_int = pcs.addr;
    int status, lpa;
    status = mdiobus_c45_read(bus, addr, MDIO_MMD_VEND2, MII_BMSR);
    if (status < 0)
    return;
    state.link = !!(status & MDIO_STAT1_LSTATUS);
    state.an_complete = !!(status & MDIO_AN_STAT1_COMPLETE);
    if (!state.link || !state.an_complete)
    return;
    lpa = mdiobus_c45_read(bus, addr, MDIO_MMD_VEND2, MII_LPA);
    if (lpa < 0)
    return;
    phylink_decode_usxgmii_word(state, lpa);
    }
    static void lynx_pcs_get_state(struct phylink_pcs *pcs, unsigned int neg_mode,
    struct phylink_link_state *state)
    {
    struct lynx_pcs *lynx = phylink_pcs_to_lynx(pcs);
    switch (state.interface) {
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_2500BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_QSGMII:
    phylink_mii_c22_pcs_get_state(lynx.mdio, neg_mode, state);
    break;
    case PHY_INTERFACE_MODE_USXGMII:
    case PHY_INTERFACE_MODE_10G_QXGMII:
    lynx_pcs_get_state_usxgmii(lynx.mdio, state);
    break;
    case PHY_INTERFACE_MODE_10GBASER:
    phylink_mii_c45_pcs_get_state(lynx.mdio, state);
    break;
    default:
    break;
    }
    dev_dbg(&lynx.mdio.dev,
    "mode=%s/%s/%s link=%u an_complete=%u\n",
    phy_modes(state.interface),
    phy_speed_to_str(state.speed),
    phy_duplex_to_str(state.duplex),
    state.link, state.an_complete);
    }
    static int lynx_pcs_config_giga(struct mdio_device *pcs,
    phy_interface_t interface,
    const unsigned long *advertising,
    unsigned int neg_mode)
    {
    int link_timer_ns;
    u32 link_timer;
    u16 if_mode;
    int err;
    link_timer_ns = phylink_get_link_timer_ns(interface);
    if (link_timer_ns > 0) {
    link_timer = LINK_TIMER_VAL(link_timer_ns);
    mdiodev_write(pcs, LINK_TIMER_LO, link_timer & 0xffff);
    mdiodev_write(pcs, LINK_TIMER_HI, link_timer >> 16);
    }
    if (interface == PHY_INTERFACE_MODE_1000BASEX ||
    interface == PHY_INTERFACE_MODE_2500BASEX) {
    if_mode = 0;
    } else {
// SGMII and QSGMII
    if_mode = IF_MODE_SGMII_EN;
    if (neg_mode == PHYLINK_PCS_NEG_INBAND_ENABLED)
    if_mode |= IF_MODE_USE_SGMII_AN;
    }
    err = mdiodev_modify(pcs, IF_MODE,
    IF_MODE_SGMII_EN | IF_MODE_USE_SGMII_AN,
    if_mode);
    if (err)
    return err;
    return phylink_mii_c22_pcs_config(pcs, interface, advertising,
    neg_mode);
    }
    static int lynx_pcs_config_usxgmii(struct mdio_device *pcs,
    phy_interface_t interface,
    const unsigned long *advertising,
    unsigned int neg_mode)
    {
    struct mii_bus *bus = pcs.bus;
    let mut addr: c_int = pcs.addr;
    if (neg_mode != PHYLINK_PCS_NEG_INBAND_ENABLED) {
    dev_err(&pcs.dev, "%s only supports in-band AN for now\n",
    phy_modes(interface));
    return -EOPNOTSUPP;
    }
// Configure device ability for the USXGMII Replicator
    return mdiobus_c45_write(bus, addr, MDIO_MMD_VEND2, MII_ADVERTISE,
    MDIO_USXGMII_10G | MDIO_USXGMII_LINK |
    MDIO_USXGMII_FULL_DUPLEX |
    ADVERTISE_SGMII | ADVERTISE_LPACK);
    }
    static int lynx_pcs_config(struct phylink_pcs *pcs, unsigned int neg_mode,
    phy_interface_t ifmode,
    const unsigned long *advertising, bool permit)
    {
    struct lynx_pcs *lynx = phylink_pcs_to_lynx(pcs);
    switch (ifmode) {
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_QSGMII:
    case PHY_INTERFACE_MODE_2500BASEX:
    return lynx_pcs_config_giga(lynx.mdio, ifmode, advertising,
    neg_mode);
    case PHY_INTERFACE_MODE_USXGMII:
    case PHY_INTERFACE_MODE_10G_QXGMII:
    return lynx_pcs_config_usxgmii(lynx.mdio, ifmode, advertising,
    neg_mode);
    case PHY_INTERFACE_MODE_10GBASER:
// Nothing to do here for 10GBASER
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lynx_pcs_an_restart(pcs: *mut phylink_pcs) {
    static void lynx_pcs_an_restart(struct phylink_pcs *pcs)
    {
    struct lynx_pcs *lynx = phylink_pcs_to_lynx(pcs);
    phylink_mii_c22_pcs_an_restart(lynx.mdio);
    }
    static void lynx_pcs_link_up_sgmii(struct mdio_device *pcs,
    unsigned int neg_mode,
    int speed, int duplex)
    {
    let mut if_mode: u16 = 0, sgmii_speed;
// The PCS needs to be configured manually only
// when not operating on in-band mode
//
    if (neg_mode == PHYLINK_PCS_NEG_INBAND_ENABLED)
    return;
    if (duplex == DUPLEX_HALF)
    if_mode |= IF_MODE_HALF_DUPLEX;
    switch (speed) {
    case SPEED_1000:
    sgmii_speed = SGMII_SPEED_1000;
    break;
    case SPEED_100:
    sgmii_speed = SGMII_SPEED_100;
    break;
    case SPEED_10:
    sgmii_speed = SGMII_SPEED_10;
    break;
    case SPEED_UNKNOWN:
// Silently don't do anything
    return;
    default:
    dev_err(&pcs.dev, "Invalid PCS speed %d\n", speed);
    return;
    }
    if_mode |= IF_MODE_SPEED(sgmii_speed);
    mdiodev_modify(pcs, IF_MODE,
    IF_MODE_HALF_DUPLEX | IF_MODE_SPEED_MSK,
    if_mode);
    }
    static void lynx_pcs_link_up(struct phylink_pcs *pcs, unsigned int neg_mode,
    phy_interface_t interface,
    int speed, int duplex)
    {
    struct lynx_pcs *lynx = phylink_pcs_to_lynx(pcs);
    switch (interface) {
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_QSGMII:
    lynx_pcs_link_up_sgmii(lynx.mdio, neg_mode, speed, duplex);
    break;
    case PHY_INTERFACE_MODE_USXGMII:
    case PHY_INTERFACE_MODE_10G_QXGMII:
// At the moment, only in-band AN is supported for USXGMII
// so nothing to do in link_up
//
    break;
    default:
    break;
    }
    }
    static const struct phylink_pcs_ops lynx_pcs_phylink_ops = {
    .pcs_inband_caps = lynx_pcs_inband_caps,
    .pcs_get_state = lynx_pcs_get_state,
    .pcs_config = lynx_pcs_config,
    .pcs_an_restart = lynx_pcs_an_restart,
    .pcs_link_up = lynx_pcs_link_up,
    };
    static const phy_interface_t lynx_interfaces[] = {
    PHY_INTERFACE_MODE_SGMII,
    PHY_INTERFACE_MODE_QSGMII,
    PHY_INTERFACE_MODE_1000BASEX,
    PHY_INTERFACE_MODE_2500BASEX,
    PHY_INTERFACE_MODE_10GBASER,
    PHY_INTERFACE_MODE_USXGMII,
    PHY_INTERFACE_MODE_10G_QXGMII,
    };
    static struct phylink_pcs *lynx_pcs_create(struct mdio_device *mdio)
    {
    struct lynx_pcs *lynx;
    int i;
    lynx = kzalloc_obj(*lynx);
    if (!lynx)
    return ERR_PTR(-ENOMEM);
    mdio_device_get(mdio);
    lynx.mdio = mdio;
    lynx.pcs.ops = &lynx_pcs_phylink_ops;
    lynx.pcs.poll = true;
    for (i = 0; i < ARRAY_SIZE(lynx_interfaces); i++)
    __set_bit(lynx_interfaces[i], lynx.pcs.supported_interfaces);
    return lynx_to_phylink_pcs(lynx);
    }
    struct phylink_pcs *lynx_pcs_create_mdiodev(struct mii_bus *bus, int addr)
    {
    struct mdio_device *mdio;
    struct phylink_pcs *pcs;
    mdio = mdio_device_create(bus, addr);
    if (IS_ERR(mdio))
    return ERR_CAST(mdio);
    pcs = lynx_pcs_create(mdio);
// lynx_create() has taken a refcount on the mdiodev if it was
// successful. If lynx_create() fails, this will free the mdio
// device here. In any case, we don't need to hold our reference
// anymore, and putting it here will allow mdio_device_put() in
// lynx_destroy() to automatically free the mdio device.
//
    mdio_device_put(mdio);
    return pcs;
    }
    EXPORT_SYMBOL(lynx_pcs_create_mdiodev);
//
// lynx_pcs_create_fwnode() creates a lynx PCS instance from the fwnode
// device indicated by node.
//
// Returns:
// -ENODEV if the fwnode is marked unavailable
// -EPROBE_DEFER if we fail to find the device
// -ENOMEM if we fail to allocate memory
// pointer to a phylink_pcs on success
//
    struct phylink_pcs *lynx_pcs_create_fwnode(struct fwnode_handle *node)
    {
    struct mdio_device *mdio;
    struct phylink_pcs *pcs;
    if (!fwnode_device_is_available(node))
    return ERR_PTR(-ENODEV);
    mdio = fwnode_mdio_find_device(node);
    if (!mdio)
    return ERR_PTR(-EPROBE_DEFER);
    pcs = lynx_pcs_create(mdio);
// lynx_create() has taken a refcount on the mdiodev if it was
// successful. If lynx_create() fails, this will free the mdio
// device here. In any case, we don't need to hold our reference
// anymore, and putting it here will allow mdio_device_put() in
// lynx_destroy() to automatically free the mdio device.
//
    mdio_device_put(mdio);
    return pcs;
    }
    EXPORT_SYMBOL_GPL(lynx_pcs_create_fwnode);
#[no_mangle]
pub unsafe extern "C" fn lynx_pcs_destroy(pcs: *mut phylink_pcs) {
    void lynx_pcs_destroy(struct phylink_pcs *pcs)
    {
    struct lynx_pcs *lynx = phylink_pcs_to_lynx(pcs);
    mdio_device_put(lynx.mdio);
    kfree(lynx);
    }
    EXPORT_SYMBOL(lynx_pcs_destroy);
    MODULE_DESCRIPTION("NXP Lynx PCS phylink library");
    MODULE_LICENSE("Dual BSD/GPL");
