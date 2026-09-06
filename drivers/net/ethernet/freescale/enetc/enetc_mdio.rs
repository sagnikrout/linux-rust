//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/enetc/enetc_mdio.c
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
// Copyright 2019 NXP

pub const ENETC_MDIO_CFG: c_uint = 0x0	/* MDIO configuration and status */;
pub const ENETC_MDIO_CTL: c_uint = 0x4	/* MDIO control */;
pub const ENETC_MDIO_DATA: c_uint = 0x8	/* MDIO data */;
pub const ENETC_MDIO_ADDR: c_uint = 0xc	/* MDIO address */;

// external MDIO only - driven on neg MDC edge

    (MDIO_CFG_HOLD(2) | \
    MDIO_CFG_CLKDIV(258) | \
    MDIO_CFG_NEG)

#[no_mangle]
pub unsafe extern "C" fn enetc_mdio_rd(mdio_priv: *mut enetc_mdio_priv, off: c_int) -> u32 {
    static inline u32 enetc_mdio_rd(struct enetc_mdio_priv *mdio_priv, int off)
    {
    return enetc_port_rd_mdio(mdio_priv.hw, mdio_priv.mdio_base + off);
    }
    static inline void enetc_mdio_wr(struct enetc_mdio_priv *mdio_priv, int off,
    u32 val)
    {
    enetc_port_wr_mdio(mdio_priv.hw, mdio_priv.mdio_base + off, val);
    }
#[no_mangle]
unsafe extern "C" fn enetc_mdio_is_busy(mdio_priv: *mut enetc_mdio_priv) -> bool {
    static bool enetc_mdio_is_busy(struct enetc_mdio_priv *mdio_priv)
    {
    return enetc_mdio_rd(mdio_priv, ENETC_MDIO_CFG) & MDIO_CFG_BSY;
    }
#[no_mangle]
unsafe extern "C" fn enetc_mdio_wait_complete(mdio_priv: *mut enetc_mdio_priv) -> c_int {
    static int enetc_mdio_wait_complete(struct enetc_mdio_priv *mdio_priv)
    {
    bool is_busy;
    return readx_poll_timeout(enetc_mdio_is_busy, mdio_priv,
    is_busy, !is_busy, 10, 10 * 1000);
    }
    int enetc_mdio_write_c22(struct mii_bus *bus, int phy_id, int regnum,
    u16 value)
    {
    struct enetc_mdio_priv *mdio_priv = bus.priv;
    u32 mdio_ctl, mdio_cfg;
    u16 dev_addr;
    int ret;
    mdio_cfg = ENETC_EMDIO_CFG;
    dev_addr = regnum & 0x1f;
    mdio_cfg &= ~MDIO_CFG_ENC45;
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CFG, mdio_cfg);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// set port and dev addr
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CTL, mdio_ctl);
// write the value
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_DATA, value);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL_GPL(enetc_mdio_write_c22);
    int enetc_mdio_write_c45(struct mii_bus *bus, int phy_id, int dev_addr,
    int regnum, u16 value)
    {
    struct enetc_mdio_priv *mdio_priv = bus.priv;
    u32 mdio_ctl, mdio_cfg;
    int ret;
    mdio_cfg = ENETC_EMDIO_CFG;
    mdio_cfg |= MDIO_CFG_ENC45;
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CFG, mdio_cfg);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// set port and dev addr
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CTL, mdio_ctl);
// set the register address
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_ADDR, regnum & 0xffff);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// write the value
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_DATA, value);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL_GPL(enetc_mdio_write_c45);
#[no_mangle]
pub unsafe extern "C" fn enetc_mdio_read_c22(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int {
    int enetc_mdio_read_c22(struct mii_bus *bus, int phy_id, int regnum)
    {
    struct enetc_mdio_priv *mdio_priv = bus.priv;
    u32 mdio_ctl, mdio_cfg;
    u16 dev_addr, value;
    int ret;
    mdio_cfg = ENETC_EMDIO_CFG;
    dev_addr = regnum & 0x1f;
    mdio_cfg &= ~MDIO_CFG_ENC45;
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CFG, mdio_cfg);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// set port and device addr
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CTL, mdio_ctl);
// initiate the read
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CTL, mdio_ctl | MDIO_CTL_READ);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// return all Fs if nothing was there
    if (enetc_mdio_rd(mdio_priv, ENETC_MDIO_CFG) & MDIO_CFG_RD_ER) {
    dev_dbg(&bus.dev,
    "Error while reading PHY%d reg at %d.%d\n",
    phy_id, dev_addr, regnum);
    return 0xffff;
    }
    value = enetc_mdio_rd(mdio_priv, ENETC_MDIO_DATA) & 0xffff;
    return value;
    }
    EXPORT_SYMBOL_GPL(enetc_mdio_read_c22);
    int enetc_mdio_read_c45(struct mii_bus *bus, int phy_id, int dev_addr,
    int regnum)
    {
    struct enetc_mdio_priv *mdio_priv = bus.priv;
    u32 mdio_ctl, mdio_cfg;
    u16 value;
    int ret;
    mdio_cfg = ENETC_EMDIO_CFG;
    mdio_cfg |= MDIO_CFG_ENC45;
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CFG, mdio_cfg);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// set port and device addr
    mdio_ctl = MDIO_CTL_PORT_ADDR(phy_id) | MDIO_CTL_DEV_ADDR(dev_addr);
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CTL, mdio_ctl);
// set the register address
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_ADDR, regnum & 0xffff);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// initiate the read
    enetc_mdio_wr(mdio_priv, ENETC_MDIO_CTL, mdio_ctl | MDIO_CTL_READ);
    ret = enetc_mdio_wait_complete(mdio_priv);
    if (ret)
    return ret;
// return all Fs if nothing was there
    if (enetc_mdio_rd(mdio_priv, ENETC_MDIO_CFG) & MDIO_CFG_RD_ER) {
    dev_dbg(&bus.dev,
    "Error while reading PHY%d reg at %d.%d\n",
    phy_id, dev_addr, regnum);
    return 0xffff;
    }
    value = enetc_mdio_rd(mdio_priv, ENETC_MDIO_DATA) & 0xffff;
    return value;
    }
    EXPORT_SYMBOL_GPL(enetc_mdio_read_c45);
    struct enetc_hw *enetc_hw_alloc(struct device *dev, void __iomem *port_regs)
    {
    struct enetc_hw *hw;
    hw = devm_kzalloc(dev, sizeof(*hw), GFP_KERNEL);
    if (!hw)
    return ERR_PTR(-ENOMEM);
    hw.port = port_regs;
    return hw;
    }
    EXPORT_SYMBOL_GPL(enetc_hw_alloc);
// Lock for MDIO access errata on LS1028A
    DEFINE_RWLOCK(enetc_mdio_lock);
    EXPORT_SYMBOL_GPL(enetc_mdio_lock);
