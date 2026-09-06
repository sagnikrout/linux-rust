//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/broadcom/genet/bcmgenet_wol.c
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
// Broadcom GENET (Gigabit Ethernet) Wake-on-LAN support
//
// Copyright (c) 2014-2025 Broadcom
//

// ethtool function - get WOL (Wake on LAN) settings, Only Magic Packet
// Detection is supported through ethtool
//
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_get_wol(dev: *mut net_device, wol: *mut ethtool_wolinfo) {
    void bcmgenet_get_wol(struct net_device *dev, struct ethtool_wolinfo *wol)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct device *kdev = &priv.pdev.dev;
    let mut phy_wolopts: u32 = 0;
    if (dev.phydev) {
    phy_ethtool_get_wol(dev.phydev, wol);
    phy_wolopts = wol.wolopts;
    }
// MAC is not wake-up capable, return what the PHY does
    if (!device_can_wakeup(kdev))
    return;
// Overlay MAC capabilities with that of the PHY queried before
    wol.supported |= WAKE_MAGIC | WAKE_MAGICSECURE | WAKE_FILTER;
    wol.wolopts |= priv.wolopts;
// Return the PHY configured magic password
    if (phy_wolopts & WAKE_MAGICSECURE)
    return;
// Otherwise the MAC one
    memset(wol.sopass, 0, sizeof(wol.sopass));
    if (wol.wolopts & WAKE_MAGICSECURE)
    memcpy(wol.sopass, priv.sopass, sizeof(priv.sopass));
    }
// ethtool function - set WOL (Wake on LAN) settings.
// Only for magic packet detection mode.
//
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_set_wol(dev: *mut net_device, wol: *mut ethtool_wolinfo) -> c_int {
    int bcmgenet_set_wol(struct net_device *dev, struct ethtool_wolinfo *wol)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct device *kdev = &priv.pdev.dev;
    int ret;
// Try Wake-on-LAN from the PHY first
    if (dev.phydev) {
    ret = phy_ethtool_set_wol(dev.phydev, wol);
    if (ret != -EOPNOTSUPP && wol.wolopts)
    return ret;
    }
    if (!device_can_wakeup(kdev))
    return -ENOTSUPP;
    if (wol.wolopts & ~(WAKE_MAGIC | WAKE_MAGICSECURE | WAKE_FILTER))
    return -EINVAL;
    if (wol.wolopts & WAKE_MAGICSECURE)
    memcpy(priv.sopass, wol.sopass, sizeof(priv.sopass));
// Flag the device and relevant IRQ as wakeup capable
    if (wol.wolopts) {
    device_set_wakeup_enable(kdev, 1);
// Avoid unbalanced enable_irq_wake calls
    if (priv.wol_irq_disabled) {
    enable_irq_wake(priv.wol_irq);
    enable_irq_wake(priv.irq0);
    }
    priv.wol_irq_disabled = false;
    } else {
    device_set_wakeup_enable(kdev, 0);
// Avoid unbalanced disable_irq_wake calls
    if (!priv.wol_irq_disabled) {
    disable_irq_wake(priv.wol_irq);
    disable_irq_wake(priv.irq0);
    }
    priv.wol_irq_disabled = true;
    }
    priv.wolopts = wol.wolopts;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_poll_wol_status(priv: *mut bcmgenet_priv) -> c_int {
    static int bcmgenet_poll_wol_status(struct bcmgenet_priv *priv)
    {
    struct net_device *dev = priv.dev;
    let mut retries: c_int = 0;
    while (!(bcmgenet_rbuf_readl(priv, RBUF_STATUS)
    & RBUF_STATUS_WOL)) {
    retries++;
    if (retries > 50) {
    netdev_crit(dev, "polling wol mode timeout\n");
    return -ETIMEDOUT;
    }
    mdelay(1);
    }
    return retries;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_set_mpd_password(priv: *mut bcmgenet_priv) {
    static void bcmgenet_set_mpd_password(struct bcmgenet_priv *priv)
    {
    bcmgenet_umac_writel(priv, get_unaligned_be16(&priv.sopass[0]),
    UMAC_MPD_PW_MS);
    bcmgenet_umac_writel(priv, get_unaligned_be32(&priv.sopass[2]),
    UMAC_MPD_PW_LS);
    }
    int bcmgenet_wol_power_down_cfg(struct bcmgenet_priv *priv,
    enum bcmgenet_power_mode mode)
    {
    struct net_device *dev = priv.dev;
    u32 reg, hfb_ctrl_reg;
    let mut retries: c_int = 0;
    if (mode != GENET_POWER_WOL_MAGIC) {
    netif_err(priv, wol, dev, "unsupported mode: %d\n", mode);
    return -EINVAL;
    }
    if (priv.wolopts & (WAKE_MAGIC | WAKE_MAGICSECURE)) {
    reg = bcmgenet_umac_readl(priv, UMAC_MPD_CTRL);
    reg |= MPD_EN;
    if (priv.wolopts & WAKE_MAGICSECURE) {
    bcmgenet_set_mpd_password(priv);
    reg |= MPD_PW_EN;
    }
    bcmgenet_umac_writel(priv, reg, UMAC_MPD_CTRL);
    }
    hfb_ctrl_reg = bcmgenet_hfb_reg_readl(priv, HFB_CTRL);
    reg = hfb_ctrl_reg | RBUF_ACPI_EN;
    bcmgenet_hfb_reg_writel(priv, reg, HFB_CTRL);
// Do not leave UniMAC in MPD mode only
    retries = bcmgenet_poll_wol_status(priv);
    if (retries < 0) {
    reg = bcmgenet_umac_readl(priv, UMAC_MPD_CTRL);
    reg &= ~(MPD_EN | MPD_PW_EN);
    bcmgenet_umac_writel(priv, reg, UMAC_MPD_CTRL);
    bcmgenet_hfb_reg_writel(priv, hfb_ctrl_reg, HFB_CTRL);
    return retries;
    }
    netif_dbg(priv, wol, dev, "MPD WOL-ready status set after %d msec\n",
    retries);
// Disable phy status updates while suspending
    mutex_lock(&dev.phydev.lock);
    dev.phydev.state = PHY_READY;
    mutex_unlock(&dev.phydev.lock);
    clk_prepare_enable(priv.clk_wol);
// Enable CRC forward
    spin_lock_bh(&priv.reg_lock);
    reg = bcmgenet_umac_readl(priv, UMAC_CMD);
    priv.crc_fwd_en = 1;
    reg |= CMD_CRC_FWD;
// Can't suspend with WoL if MAC is still in reset
    if (reg & CMD_SW_RESET)
    reg &= ~CMD_SW_RESET;
// Receiver must be enabled for WOL MP detection
    reg |= CMD_RX_EN;
    bcmgenet_umac_writel(priv, reg, UMAC_CMD);
    spin_unlock_bh(&priv.reg_lock);
    reg = UMAC_IRQ_MPD_R;
    if (hfb_ctrl_reg & RBUF_HFB_EN)
    reg |=  UMAC_IRQ_HFB_SM | UMAC_IRQ_HFB_MM;
    bcmgenet_intrl2_0_writel(priv, reg, INTRL2_CPU_MASK_CLEAR);
    return 0;
    }
    int bcmgenet_wol_power_up_cfg(struct bcmgenet_priv *priv,
    enum bcmgenet_power_mode mode)
    {
    struct net_device *dev = priv.dev;
    u32 reg;
    if (mode != GENET_POWER_WOL_MAGIC) {
    netif_err(priv, wol, priv.dev, "invalid mode: %d\n", mode);
    return -EINVAL;
    }
    clk_disable_unprepare(priv.clk_wol);
    priv.crc_fwd_en = 0;
    bcmgenet_intrl2_0_writel(priv, UMAC_IRQ_WAKE_EVENT,
    INTRL2_CPU_MASK_SET);
    if (bcmgenet_has_mdio_intr(priv))
    bcmgenet_intrl2_0_writel(priv,
    UMAC_IRQ_MDIO_EVENT,
    INTRL2_CPU_MASK_CLEAR);
// Disable Magic Packet Detection
    if (priv.wolopts & (WAKE_MAGIC | WAKE_MAGICSECURE)) {
    reg = bcmgenet_umac_readl(priv, UMAC_MPD_CTRL);
    if (!(reg & MPD_EN))
    return -EPERM;	/* already reset so skip the rest */
    reg &= ~(MPD_EN | MPD_PW_EN);
    bcmgenet_umac_writel(priv, reg, UMAC_MPD_CTRL);
    }
// Disable ACPI mode
    reg = bcmgenet_hfb_reg_readl(priv, HFB_CTRL);
    if (!(reg & RBUF_ACPI_EN))
    return -EPERM;	/* already reset so skip the rest */
    reg &= ~RBUF_ACPI_EN;
    bcmgenet_hfb_reg_writel(priv, reg, HFB_CTRL);
// Disable CRC Forward
    spin_lock_bh(&priv.reg_lock);
    reg = bcmgenet_umac_readl(priv, UMAC_CMD);
    reg &= ~CMD_CRC_FWD;
    bcmgenet_umac_writel(priv, reg, UMAC_CMD);
    spin_unlock_bh(&priv.reg_lock);
// Resume link status tracking
    mutex_lock(&dev.phydev.lock);
    if (dev.phydev.link)
    dev.phydev.state = PHY_RUNNING;
    else
    dev.phydev.state = PHY_NOLINK;
    mutex_unlock(&dev.phydev.lock);
    return 0;
    }
