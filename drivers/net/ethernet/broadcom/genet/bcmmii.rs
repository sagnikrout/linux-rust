//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/broadcom/genet/bcmmii.c
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
// Broadcom GENET MDIO routines
//
// Copyright (c) 2014-2025 Broadcom
//

#[no_mangle]
unsafe extern "C" fn bcmgenet_mac_config(dev: *mut net_device) {
    static void bcmgenet_mac_config(struct net_device *dev)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct phy_device *phydev = dev.phydev;
    u32 reg, cmd_bits = 0;
// speed
    if (phydev.speed == SPEED_1000)
    cmd_bits = CMD_SPEED_1000;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: phydev->speed ==) -> else {
    else if (phydev.speed == SPEED_100)
    cmd_bits = CMD_SPEED_100;
    else
    cmd_bits = CMD_SPEED_10;
    cmd_bits <<= CMD_SPEED_SHIFT;
// duplex
    if (phydev.duplex != DUPLEX_FULL) {
    cmd_bits |= CMD_HD_EN |
    CMD_RX_PAUSE_IGNORE | CMD_TX_PAUSE_IGNORE;
    } else {
// pause capability defaults to Symmetric
    if (priv.autoneg_pause) {
    let mut tx_pause: bool = 0, rx_pause = 0;
    if (phydev.autoneg)
    phy_get_pause(phydev, &tx_pause, &rx_pause);
    if (!tx_pause)
    cmd_bits |= CMD_TX_PAUSE_IGNORE;
    if (!rx_pause)
    cmd_bits |= CMD_RX_PAUSE_IGNORE;
    }
// Manual override
    if (!priv.rx_pause)
    cmd_bits |= CMD_RX_PAUSE_IGNORE;
    if (!priv.tx_pause)
    cmd_bits |= CMD_TX_PAUSE_IGNORE;
    }
// Program UMAC and RGMII block based on established
// link speed, duplex, and pause. The speed set in
// umac->cmd tell RGMII block which clock to use for
// transmit -- 25MHz(100Mbps) or 125MHz(1Gbps).
// Receive clock is provided by the PHY.
//
    reg = bcmgenet_ext_readl(priv, EXT_RGMII_OOB_CTRL);
    reg |= RGMII_LINK;
    bcmgenet_ext_writel(priv, reg, EXT_RGMII_OOB_CTRL);
    spin_lock_bh(&priv.reg_lock);
    reg = bcmgenet_umac_readl(priv, UMAC_CMD);
    reg &= ~((CMD_SPEED_MASK << CMD_SPEED_SHIFT) |
    CMD_HD_EN |
    CMD_RX_PAUSE_IGNORE | CMD_TX_PAUSE_IGNORE);
    reg |= cmd_bits;
    if (reg & CMD_SW_RESET) {
    reg &= ~CMD_SW_RESET;
    bcmgenet_umac_writel(priv, reg, UMAC_CMD);
    udelay(2);
    reg |= CMD_TX_EN | CMD_RX_EN;
    }
    bcmgenet_umac_writel(priv, reg, UMAC_CMD);
    spin_unlock_bh(&priv.reg_lock);
    }
// setup netdev link state when PHY link status change and
// update UMAC and RGMII block when link up
//
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_mii_setup(dev: *mut net_device) {
    void bcmgenet_mii_setup(struct net_device *dev)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct phy_device *phydev = dev.phydev;
    u32 reg;
    if (phydev.link) {
    bcmgenet_mac_config(dev);
    } else {
    reg = bcmgenet_ext_readl(priv, EXT_RGMII_OOB_CTRL);
    reg &= ~RGMII_LINK;
    bcmgenet_ext_writel(priv, reg, EXT_RGMII_OOB_CTRL);
    }
    bcmgenet_eee_enable_set(dev, phydev.enable_tx_lpi);
    phy_print_status(phydev);
    }
    static int bcmgenet_fixed_phy_link_update(struct net_device *dev,
    struct fixed_phy_status *status)
    {
    struct bcmgenet_priv *priv;
    u32 reg;
    if (dev && dev.phydev && status) {
    priv = netdev_priv(dev);
    reg = bcmgenet_umac_readl(priv, UMAC_MODE);
    status.link = !!(reg & MODE_LINK_STATUS);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_phy_pause_set(dev: *mut net_device, rx: bool, tx: bool) {
    void bcmgenet_phy_pause_set(struct net_device *dev, bool rx, bool tx)
    {
    struct phy_device *phydev = dev.phydev;
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Pause_BIT, phydev.advertising, rx);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, phydev.advertising,
    rx | tx);
    phy_start_aneg(phydev);
    mutex_lock(&phydev.lock);
    if (phydev.link)
    bcmgenet_mac_config(dev);
    mutex_unlock(&phydev.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_phy_power_set(dev: *mut net_device, enable: bool) {
    void bcmgenet_phy_power_set(struct net_device *dev, bool enable)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    let mut reg: u32 = 0;
// EXT_GPHY_CTRL is only valid for GENETv4 and onward
    if (GENET_IS_V4(priv) || bcmgenet_has_ephy_16nm(priv)) {
    reg = bcmgenet_ext_readl(priv, EXT_GPHY_CTRL);
    if (enable) {
    reg &= ~EXT_CK25_DIS;
    bcmgenet_ext_writel(priv, reg, EXT_GPHY_CTRL);
    mdelay(1);
    reg &= ~(EXT_CFG_IDDQ_BIAS | EXT_CFG_PWR_DOWN |
    EXT_CFG_IDDQ_GLOBAL_PWR);
    reg |= EXT_GPHY_RESET;
    bcmgenet_ext_writel(priv, reg, EXT_GPHY_CTRL);
    mdelay(1);
    reg &= ~EXT_GPHY_RESET;
    } else {
    reg |= EXT_GPHY_RESET;
    bcmgenet_ext_writel(priv, reg, EXT_GPHY_CTRL);
    mdelay(1);
    reg |= EXT_CFG_IDDQ_BIAS | EXT_CFG_PWR_DOWN |
    EXT_CFG_IDDQ_GLOBAL_PWR;
    bcmgenet_ext_writel(priv, reg, EXT_GPHY_CTRL);
    mdelay(1);
    reg |= EXT_CK25_DIS;
    }
    bcmgenet_ext_writel(priv, reg, EXT_GPHY_CTRL);
    udelay(60);
    } else {
    mdelay(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_moca_phy_setup(priv: *mut bcmgenet_priv) {
    static void bcmgenet_moca_phy_setup(struct bcmgenet_priv *priv)
    {
    if (bcmgenet_has_moca_link_det(priv))
    fixed_phy_set_link_update(priv.dev.phydev,
    bcmgenet_fixed_phy_link_update);
    }
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_mii_config(dev: *mut net_device, init: bool) -> c_int {
    int bcmgenet_mii_config(struct net_device *dev, bool init)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct phy_device *phydev = dev.phydev;
    struct device *kdev = &priv.pdev.dev;
    const char *phy_name = core::ptr::null_mut();
    let mut id_mode_dis: u32 = 0;
    u32 port_ctrl;
    u32 reg;
    switch (priv.phy_interface) {
    case PHY_INTERFACE_MODE_INTERNAL:
    phy_name = "internal PHY";
    fallthrough;
    case PHY_INTERFACE_MODE_MOCA:
// Irrespective of the actually configured PHY speed (100 or
// 1000) GENETv4 only has an internal GPHY so we will just end
// up masking the Gigabit features from what we support, not
// switching to the EPHY
//
    if (GENET_IS_V4(priv))
    port_ctrl = PORT_MODE_INT_GPHY;
    else
    port_ctrl = PORT_MODE_INT_EPHY;
    if (!phy_name) {
    phy_name = "MoCA";
    if (!GENET_IS_V5(priv))
    port_ctrl |= LED_ACT_SOURCE_MAC;
    bcmgenet_moca_phy_setup(priv);
    }
    break;
    case PHY_INTERFACE_MODE_MII:
    phy_name = "external MII";
    phy_set_max_speed(phydev, SPEED_100);
    port_ctrl = PORT_MODE_EXT_EPHY;
    break;
    case PHY_INTERFACE_MODE_REVMII:
    phy_name = "external RvMII";
// of_mdiobus_register took care of reading the 'max-speed'
// PHY property for us, effectively limiting the PHY supported
// capabilities, use that knowledge to also configure the
// Reverse MII interface correctly.
//
    if (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    dev.phydev.supported))
    port_ctrl = PORT_MODE_EXT_RVMII_50;
    else
    port_ctrl = PORT_MODE_EXT_RVMII_25;
    break;
    case PHY_INTERFACE_MODE_RGMII:
// RGMII_NO_ID: TXC transitions at the same time as TXD
// (requires PCB or receiver-side delay)
//
// ID is implicitly disabled for 100Mbps (RG)MII operation.
//
    phy_name = "external RGMII (no delay)";
    id_mode_dis = BIT(16);
    port_ctrl = PORT_MODE_EXT_GPHY;
    break;
    case PHY_INTERFACE_MODE_RGMII_TXID:
// RGMII_TXID:	Add 2ns delay on TXC (90 degree shift)
    phy_name = "external RGMII (TX delay)";
    port_ctrl = PORT_MODE_EXT_GPHY;
    break;
    case PHY_INTERFACE_MODE_RGMII_RXID:
    phy_name = "external RGMII (RX delay)";
    port_ctrl = PORT_MODE_EXT_GPHY;
    break;
    default:
    dev_err(kdev, "unknown phy mode: %d\n", priv.phy_interface);
    return -EINVAL;
    }
    bcmgenet_sys_writel(priv, port_ctrl, SYS_PORT_CTRL);
    priv.ext_phy = !priv.internal_phy &&
    (priv.phy_interface != PHY_INTERFACE_MODE_MOCA);
// This is an external PHY (xMII), so we need to enable the RGMII
// block for the interface to work, unconditionally clear the
// Out-of-band disable since we do not need it.
//
    mutex_lock(&phydev.lock);
    reg = bcmgenet_ext_readl(priv, EXT_RGMII_OOB_CTRL);
    reg &= ~OOB_DISABLE;
    if (priv.ext_phy) {
    reg &= ~ID_MODE_DIS;
    reg |= id_mode_dis;
    if (GENET_IS_V1(priv) || GENET_IS_V2(priv) || GENET_IS_V3(priv))
    reg |= RGMII_MODE_EN_V123;
    else
    reg |= RGMII_MODE_EN;
    }
    bcmgenet_ext_writel(priv, reg, EXT_RGMII_OOB_CTRL);
    mutex_unlock(&phydev.lock);
    if (init)
    dev_info(kdev, "configuring instance for %s\n", phy_name);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_mii_probe(dev: *mut net_device) -> c_int {
    int bcmgenet_mii_probe(struct net_device *dev)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct device *kdev = &priv.pdev.dev;
    struct device_node *dn = kdev.of_node;
    let mut phy_iface: phy_interface_t = priv.phy_interface;
    struct phy_device *phydev;
    u32 phy_flags = PHY_BRCM_AUTO_PWRDWN_ENABLE |
    PHY_BRCM_DIS_TXCRXC_NOENRGY |
    PHY_BRCM_IDDQ_SUSPEND;
    int ret;
// Communicate the integrated PHY revision
    if (priv.internal_phy)
    phy_flags = priv.gphy_rev;
// This is an ugly quirk but we have not been correctly interpreting
// the phy_interface values and we have done that across different
// drivers, so at least we are consistent in our mistakes.
//
// When the Generic PHY driver is in use either the PHY has been
// strapped or programmed correctly by the boot loader so we should
// stick to our incorrect interpretation since we have validated it.
//
// Now when a dedicated PHY driver is in use, we need to reverse the
// meaning of the phy_interface_mode values to something that the PHY
// driver will interpret and act on such that we have two mistakes
// canceling themselves so to speak. We only do this for the two
// modes that GENET driver officially supports on Broadcom STB chips:
// PHY_INTERFACE_MODE_RGMII and PHY_INTERFACE_MODE_RGMII_TXID. Other
// modes are not *officially* supported with the boot loader and the
// scripted environment generating Device Tree blobs for those
// platforms.
//
// Note that internal PHY, MoCA and fixed-link configurations are not
// affected because they use different phy_interface_t values or the
// Generic PHY driver.
//
    switch (priv.phy_interface) {
    case PHY_INTERFACE_MODE_RGMII:
    phy_iface = PHY_INTERFACE_MODE_RGMII_ID;
    break;
    case PHY_INTERFACE_MODE_RGMII_TXID:
    phy_iface = PHY_INTERFACE_MODE_RGMII_RXID;
    break;
    default:
    break;
    }
    if (dn) {
    phydev = of_phy_connect(dev, priv.phy_dn, bcmgenet_mii_setup,
    phy_flags, phy_iface);
    if (!phydev) {
    pr_err("could not attach to PHY\n");
    return -ENODEV;
    }
    } else {
    if (has_acpi_companion(kdev)) {
    char mdio_bus_id[MII_BUS_ID_SIZE];
    struct mii_bus *unimacbus;
    snprintf(mdio_bus_id, MII_BUS_ID_SIZE, "%s-%d",
    UNIMAC_MDIO_DRV_NAME, priv.pdev.id);
    unimacbus = mdio_find_bus(mdio_bus_id);
    if (!unimacbus) {
    pr_err("Unable to find mii\n");
    return -ENODEV;
    }
    phydev = phy_find_first(unimacbus);
    put_device(&unimacbus.dev);
    if (!phydev) {
    pr_err("Unable to find PHY\n");
    return -ENODEV;
    }
    } else {
    phydev = dev.phydev;
    }
    phydev.dev_flags = phy_flags;
    ret = phy_connect_direct(dev, phydev, bcmgenet_mii_setup,
    phy_iface);
    if (ret) {
    pr_err("could not attach to PHY\n");
    return -ENODEV;
    }
    }
// Configure port multiplexer based on what the probed PHY device since
// reading the 'max-speed' property determines the maximum supported
// PHY speed which is needed for bcmgenet_mii_config() to configure
// things appropriately.
//
    ret = bcmgenet_mii_config(dev, true);
    if (ret) {
    phy_disconnect(dev.phydev);
    return ret;
    }
// The internal PHY has its link interrupts routed to the
// Ethernet MAC ISRs. On GENETv5 there is a hardware issue
// that prevents the signaling of link UP interrupts when
// the link operates at 10Mbps, so fallback to polling for
// those versions of GENET.
//
    if (priv.internal_phy && !GENET_IS_V5(priv))
    dev.phydev.irq = PHY_MAC_INTERRUPT;
// Indicate that the MAC is responsible for PHY PM
    dev.phydev.mac_managed_pm = true;
    if (!GENET_IS_V1(priv))
    phy_support_eee(dev.phydev);
    return 0;
    }
    static struct device_node *bcmgenet_mii_of_find_mdio(struct bcmgenet_priv *priv)
    {
    struct device_node *dn = priv.pdev.dev.of_node;
    struct device *kdev = &priv.pdev.dev;
    char *compat;
    compat = kasprintf(GFP_KERNEL, "brcm,genet-mdio-v%d", priv.version);
    if (!compat)
    return core::ptr::null_mut();
    priv.mdio_dn = of_get_compatible_child(dn, compat);
    kfree(compat);
    if (!priv.mdio_dn) {
    dev_err(kdev, "unable to find MDIO bus node\n");
    return core::ptr::null_mut();
    }
    return priv.mdio_dn;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_mii_wait(wait_func_data: *mut c_void) -> c_int {
    static int bcmgenet_mii_wait(void *wait_func_data)
    {
    struct bcmgenet_priv *priv = wait_func_data;
    wait_event_timeout(priv.wq,
    !(bcmgenet_umac_readl(priv, UMAC_MDIO_CMD)
    & MDIO_START_BUSY),
    HZ / 100);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_mii_register(priv: *mut bcmgenet_priv) -> c_int {
    static int bcmgenet_mii_register(struct bcmgenet_priv *priv)
    {
    struct platform_device *pdev = priv.pdev;
    struct device_node *dn = pdev.dev.of_node;
    struct unimac_mdio_pdata ppd;
    struct platform_device *ppdev;
    struct resource *pres, res;
    int id, ret;
    pres = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!pres) {
    dev_err(&pdev.dev, "Invalid resource\n");
    return -EINVAL;
    }
    memset(&res, 0, sizeof(res));
    memset(&ppd, 0, sizeof(ppd));
    ppd.wait_func = bcmgenet_mii_wait;
    ppd.wait_func_data = priv;
    ppd.bus_name = "bcmgenet MII bus";
// Pass a reference to our "main" clock which is used for MDIO
// transfers
//
    ppd.clk = priv.clk;
// Unimac MDIO bus controller starts at UniMAC offset + MDIO_CMD
// and is 2 * 32-bits word long, 8 bytes total.
//
    res.start = pres.start + GENET_UMAC_OFF + UMAC_MDIO_CMD;
    res.end = res.start + 8;
    res.flags = IORESOURCE_MEM;
    if (dn)
    id = of_alias_get_id(dn, "eth");
    else
    id = pdev.id;
    ppdev = platform_device_alloc(UNIMAC_MDIO_DRV_NAME, id);
    if (!ppdev)
    return -ENOMEM;
// Retain this platform_device pointer for later cleanup
    priv.mii_pdev = ppdev;
    ppdev.dev.parent = &pdev.dev;
    if (dn)
    platform_device_set_of_node(ppdev, bcmgenet_mii_of_find_mdio(priv));
    else
    ppd.phy_mask = ~0;
    ret = platform_device_add_resources(ppdev, &res, 1);
    if (ret)
    goto out;
    ret = platform_device_add_data(ppdev, &ppd, sizeof(ppd));
    if (ret)
    goto out;
    ret = platform_device_add(ppdev);
    if (ret)
    goto out;
    return 0;
    out:
    platform_device_put(ppdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_phy_interface_init(priv: *mut bcmgenet_priv) -> c_int {
    static int bcmgenet_phy_interface_init(struct bcmgenet_priv *priv)
    {
    struct device *kdev = &priv.pdev.dev;
    let mut phy_mode: c_int = device_get_phy_mode(kdev);
    if (phy_mode < 0) {
    dev_err(kdev, "invalid PHY mode property\n");
    return phy_mode;
    }
    priv.phy_interface = phy_mode;
// We need to specifically look up whether this PHY interface is
// internal or not *before* we even try to probe the PHY driver
// over MDIO as we may have shut down the internal PHY for power
// saving purposes.
//
    if (priv.phy_interface == PHY_INTERFACE_MODE_INTERNAL)
    priv.internal_phy = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_mii_of_init(priv: *mut bcmgenet_priv) -> c_int {
    static int bcmgenet_mii_of_init(struct bcmgenet_priv *priv)
    {
    struct device_node *dn = priv.pdev.dev.of_node;
    struct phy_device *phydev;
    int ret;
// Fetch the PHY phandle
    priv.phy_dn = of_parse_phandle(dn, "phy-handle", 0);
// In the case of a fixed PHY, the DT node associated
// to the PHY is the Ethernet MAC DT node.
//
    if (!priv.phy_dn && of_phy_is_fixed_link(dn)) {
    ret = of_phy_register_fixed_link(dn);
    if (ret)
    return ret;
    priv.phy_dn = of_node_get(dn);
    }
// Get the link mode
    ret = bcmgenet_phy_interface_init(priv);
    if (ret)
    return ret;
// Make sure we initialize MoCA PHYs with a link down
    if (priv.phy_interface == PHY_INTERFACE_MODE_MOCA) {
    phydev = of_phy_find_device(dn);
    if (phydev) {
    phydev.link = 0;
    put_device(&phydev.mdio.dev);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcmgenet_mii_bus_init(priv: *mut bcmgenet_priv) -> c_int {
    static int bcmgenet_mii_bus_init(struct bcmgenet_priv *priv)
    {
    struct device *kdev = &priv.pdev.dev;
    struct device_node *dn = kdev.of_node;
    if (dn)
    return bcmgenet_mii_of_init(priv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: has_acpi_companion(kdev)) -> else {
    else if (has_acpi_companion(kdev))
    return bcmgenet_phy_interface_init(priv);
    else
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_mii_init(dev: *mut net_device) -> c_int {
    int bcmgenet_mii_init(struct net_device *dev)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    int ret;
    ret = bcmgenet_mii_register(priv);
    if (ret)
    return ret;
    ret = bcmgenet_mii_bus_init(priv);
    if (ret)
    goto out;
    return 0;
    out:
    bcmgenet_mii_exit(dev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bcmgenet_mii_exit(dev: *mut net_device) {
    void bcmgenet_mii_exit(struct net_device *dev)
    {
    struct bcmgenet_priv *priv = netdev_priv(dev);
    struct device_node *dn = priv.pdev.dev.of_node;
    if (of_phy_is_fixed_link(dn))
    of_phy_deregister_fixed_link(dn);
    of_node_put(priv.phy_dn);
    platform_device_unregister(priv.mii_pdev);
    }
