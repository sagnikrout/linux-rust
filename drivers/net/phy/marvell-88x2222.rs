//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/marvell-88x2222.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Marvell 88x2222 dual-port multi-speed ethernet transceiver.
//
// Supports:
// XAUI on the host side.
// 1000Base-X or 10GBase-R on the line side.
// SGMII over 1000Base-X.
//

// Port PCS Configuration
pub const MV_PCS_CONFIG: c_uint = 0xF002;
pub const MV_PCS_HOST_XAUI: c_uint = 0x73;

// Port Reset and Power Down
pub const MV_PORT_RST: c_uint = 0xF003;

// PMD Receive Signal Detect
pub const MV_RX_SIGNAL_DETECT: c_uint = 0x000A;

// 1000Base-X/SGMII Control Register

// 1000BASE-X/SGMII Status Register

// 1000Base-X Auto-Negotiation Advertisement Register

// 1000Base-X PHY Specific Status Register
pub const MV_1GBX_PHY_STAT: c_uint = 0xA003;

pub const AUTONEG_TIMEOUT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv2222_data {
    pub line_interface: phy_interface_t,
    pub sfp_link: bool,
}

// SFI PMA transmit enable
#[no_mangle]
unsafe extern "C" fn mv2222_tx_enable(phydev: *mut phy_device) -> c_int {
    static int mv2222_tx_enable(struct phy_device *phydev)
    {
    return phy_clear_bits_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_PMA_TXDIS,
    MDIO_PMD_TXDIS_GLOBAL);
    }
// SFI PMA transmit disable
#[no_mangle]
unsafe extern "C" fn mv2222_tx_disable(phydev: *mut phy_device) -> c_int {
    static int mv2222_tx_disable(struct phy_device *phydev)
    {
    return phy_set_bits_mmd(phydev, MDIO_MMD_PMAPMD, MDIO_PMA_TXDIS,
    MDIO_PMD_TXDIS_GLOBAL);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_soft_reset(phydev: *mut phy_device) -> c_int {
    static int mv2222_soft_reset(struct phy_device *phydev)
    {
    int val, ret;
    ret = phy_write_mmd(phydev, MDIO_MMD_VEND2, MV_PORT_RST,
    MV_PORT_RST_SW);
    if (ret < 0)
    return ret;
    return phy_read_mmd_poll_timeout(phydev, MDIO_MMD_VEND2, MV_PORT_RST,
    val, !(val & MV_PORT_RST_SW),
    5000, 1000000, true);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_disable_aneg(phydev: *mut phy_device) -> c_int {
    static int mv2222_disable_aneg(struct phy_device *phydev)
    {
    int ret = phy_clear_bits_mmd(phydev, MDIO_MMD_PCS, MV_1GBX_CTRL,
    BMCR_ANENABLE | BMCR_ANRESTART);
    if (ret < 0)
    return ret;
    return mv2222_soft_reset(phydev);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_enable_aneg(phydev: *mut phy_device) -> c_int {
    static int mv2222_enable_aneg(struct phy_device *phydev)
    {
    int ret = phy_set_bits_mmd(phydev, MDIO_MMD_PCS, MV_1GBX_CTRL,
    BMCR_ANENABLE | BMCR_RESET);
    if (ret < 0)
    return ret;
    return mv2222_soft_reset(phydev);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_set_sgmii_speed(phydev: *mut phy_device) -> c_int {
    static int mv2222_set_sgmii_speed(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    switch (phydev.speed) {
    default:
    case SPEED_1000:
    if ((linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT,
    priv.supported)))
    return phy_modify_mmd(phydev, MDIO_MMD_PCS,
    MV_1GBX_CTRL,
    BMCR_SPEED1000 | BMCR_SPEED100,
    BMCR_SPEED1000);
    fallthrough;
    case SPEED_100:
    if ((linkmode_test_bit(ETHTOOL_LINK_MODE_100baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_100baseT_Half_BIT,
    priv.supported)))
    return phy_modify_mmd(phydev, MDIO_MMD_PCS,
    MV_1GBX_CTRL,
    BMCR_SPEED1000 | BMCR_SPEED100,
    BMCR_SPEED100);
    fallthrough;
    case SPEED_10:
    if ((linkmode_test_bit(ETHTOOL_LINK_MODE_10baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10baseT_Half_BIT,
    priv.supported)))
    return phy_modify_mmd(phydev, MDIO_MMD_PCS,
    MV_1GBX_CTRL,
    BMCR_SPEED1000 | BMCR_SPEED100,
    BMCR_SPEED10);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn mv2222_is_10g_capable(phydev: *mut phy_device) -> bool {
    static bool mv2222_is_10g_capable(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    return (linkmode_test_bit(ETHTOOL_LINK_MODE_10000baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10000baseCR_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10000baseSR_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10000baseLR_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10000baseLRM_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10000baseER_Full_BIT,
    priv.supported));
    }
#[no_mangle]
unsafe extern "C" fn mv2222_is_1gbx_capable(phydev: *mut phy_device) -> bool {
    static bool mv2222_is_1gbx_capable(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    return linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseX_Full_BIT,
    priv.supported);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_is_sgmii_capable(phydev: *mut phy_device) -> bool {
    static bool mv2222_is_sgmii_capable(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    return (linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_100baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_100baseT_Half_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10baseT_Full_BIT,
    priv.supported) ||
    linkmode_test_bit(ETHTOOL_LINK_MODE_10baseT_Half_BIT,
    priv.supported));
    }
#[no_mangle]
unsafe extern "C" fn mv2222_config_line(phydev: *mut phy_device) -> c_int {
    static int mv2222_config_line(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    switch (priv.line_interface) {
    case PHY_INTERFACE_MODE_10GBASER:
    return phy_write_mmd(phydev, MDIO_MMD_VEND2, MV_PCS_CONFIG,
    MV_PCS_HOST_XAUI | MV_PCS_LINE_10GBR);
    case PHY_INTERFACE_MODE_1000BASEX:
    return phy_write_mmd(phydev, MDIO_MMD_VEND2, MV_PCS_CONFIG,
    MV_PCS_HOST_XAUI | MV_PCS_LINE_1GBX_AN);
    case PHY_INTERFACE_MODE_SGMII:
    return phy_write_mmd(phydev, MDIO_MMD_VEND2, MV_PCS_CONFIG,
    MV_PCS_HOST_XAUI | MV_PCS_LINE_SGMII_AN);
    default:
    return -EINVAL;
    }
    }
// Switch between 1G (1000Base-X/SGMII) and 10G (10GBase-R) modes
#[no_mangle]
unsafe extern "C" fn mv2222_swap_line_type(phydev: *mut phy_device) -> c_int {
    static int mv2222_swap_line_type(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    let mut changed: bool = false;
    int ret;
    switch (priv.line_interface) {
    case PHY_INTERFACE_MODE_10GBASER:
    if (mv2222_is_1gbx_capable(phydev)) {
    priv.line_interface = PHY_INTERFACE_MODE_1000BASEX;
    changed = true;
    }
    if (mv2222_is_sgmii_capable(phydev)) {
    priv.line_interface = PHY_INTERFACE_MODE_SGMII;
    changed = true;
    }
    break;
    case PHY_INTERFACE_MODE_1000BASEX:
    case PHY_INTERFACE_MODE_SGMII:
    if (mv2222_is_10g_capable(phydev)) {
    priv.line_interface = PHY_INTERFACE_MODE_10GBASER;
    changed = true;
    }
    break;
    default:
    return -EINVAL;
    }
    if (changed) {
    ret = mv2222_config_line(phydev);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_setup_forced(phydev: *mut phy_device) -> c_int {
    static int mv2222_setup_forced(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    int ret;
    if (priv.line_interface == PHY_INTERFACE_MODE_10GBASER) {
    if (phydev.speed < SPEED_10000 &&
    phydev.speed != SPEED_UNKNOWN) {
    ret = mv2222_swap_line_type(phydev);
    if (ret < 0)
    return ret;
    }
    }
    if (priv.line_interface == PHY_INTERFACE_MODE_SGMII) {
    ret = mv2222_set_sgmii_speed(phydev);
    if (ret < 0)
    return ret;
    }
    return mv2222_disable_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_config_aneg(phydev: *mut phy_device) -> c_int {
    static int mv2222_config_aneg(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    int ret, adv;
// SFP is not present, do nothing
    if (priv.line_interface == PHY_INTERFACE_MODE_NA)
    return 0;
    if (phydev.autoneg == AUTONEG_DISABLE ||
    priv.line_interface == PHY_INTERFACE_MODE_10GBASER)
    return mv2222_setup_forced(phydev);
    adv = linkmode_adv_to_mii_adv_x(priv.supported,
    ETHTOOL_LINK_MODE_1000baseX_Full_BIT);
    ret = phy_modify_mmd(phydev, MDIO_MMD_PCS, MV_1GBX_ADVERTISE,
    ADVERTISE_1000XFULL |
    ADVERTISE_1000XPAUSE | ADVERTISE_1000XPSE_ASYM,
    adv);
    if (ret < 0)
    return ret;
    return mv2222_enable_aneg(phydev);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_aneg_done(phydev: *mut phy_device) -> c_int {
    static int mv2222_aneg_done(struct phy_device *phydev)
    {
    int ret;
    if (mv2222_is_10g_capable(phydev)) {
    ret = phy_read_mmd(phydev, MDIO_MMD_PCS, MDIO_STAT1);
    if (ret < 0)
    return ret;
    if (ret & MDIO_STAT1_LSTATUS)
    return 1;
    }
    ret = phy_read_mmd(phydev, MDIO_MMD_PCS, MV_1GBX_STAT);
    if (ret < 0)
    return ret;
    return (ret & BMSR_ANEGCOMPLETE);
    }
// Returns negative on error, 0 if link is down, 1 if link is up
#[no_mangle]
unsafe extern "C" fn mv2222_read_status_10g(phydev: *mut phy_device) -> c_int {
    static int mv2222_read_status_10g(struct phy_device *phydev)
    {
    static int timeout;
    int val, link = 0;
    val = phy_read_mmd(phydev, MDIO_MMD_PCS, MDIO_STAT1);
    if (val < 0)
    return val;
    if (val & MDIO_STAT1_LSTATUS) {
    link = 1;
// 10GBASE-R do not support auto-negotiation
    phydev.autoneg = AUTONEG_DISABLE;
    phydev.speed = SPEED_10000;
    phydev.duplex = DUPLEX_FULL;
    } else {
    if (phydev.autoneg == AUTONEG_ENABLE) {
    timeout++;
    if (timeout > AUTONEG_TIMEOUT) {
    timeout = 0;
    val = mv2222_swap_line_type(phydev);
    if (val < 0)
    return val;
    return mv2222_config_aneg(phydev);
    }
    }
    }
    return link;
    }
// Returns negative on error, 0 if link is down, 1 if link is up
#[no_mangle]
unsafe extern "C" fn mv2222_read_status_1g(phydev: *mut phy_device) -> c_int {
    static int mv2222_read_status_1g(struct phy_device *phydev)
    {
    static int timeout;
    int val, link = 0;
    val = phy_read_mmd(phydev, MDIO_MMD_PCS, MV_1GBX_STAT);
    if (val < 0)
    return val;
    if (phydev.autoneg == AUTONEG_ENABLE &&
    !(val & BMSR_ANEGCOMPLETE)) {
    timeout++;
    if (timeout > AUTONEG_TIMEOUT) {
    timeout = 0;
    val = mv2222_swap_line_type(phydev);
    if (val < 0)
    return val;
    return mv2222_config_aneg(phydev);
    }
    return 0;
    }
    if (!(val & BMSR_LSTATUS))
    return 0;
    link = 1;
    val = phy_read_mmd(phydev, MDIO_MMD_PCS, MV_1GBX_PHY_STAT);
    if (val < 0)
    return val;
    if (val & MV_1GBX_PHY_STAT_AN_RESOLVED) {
    if (val & MV_1GBX_PHY_STAT_DUPLEX)
    phydev.duplex = DUPLEX_FULL;
    else
    phydev.duplex = DUPLEX_HALF;
    if (val & MV_1GBX_PHY_STAT_SPEED1000)
    phydev.speed = SPEED_1000;
#[no_mangle]
pub unsafe extern "C" fn if(MV_1GBX_PHY_STAT_SPEED100: val &) -> else {
    else if (val & MV_1GBX_PHY_STAT_SPEED100)
    phydev.speed = SPEED_100;
    else
    phydev.speed = SPEED_10;
    }
    return link;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_link_is_operational(phydev: *mut phy_device) -> bool {
    static bool mv2222_link_is_operational(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    int val;
    val = phy_read_mmd(phydev, MDIO_MMD_PMAPMD, MV_RX_SIGNAL_DETECT);
    if (val < 0 || !(val & MV_RX_SIGNAL_DETECT_GLOBAL))
    return false;
    if (phydev.sfp_bus && !priv.sfp_link)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_read_status(phydev: *mut phy_device) -> c_int {
    static int mv2222_read_status(struct phy_device *phydev)
    {
    struct mv2222_data *priv = phydev.priv;
    int link;
    phydev.link = 0;
    phydev.speed = SPEED_UNKNOWN;
    phydev.duplex = DUPLEX_UNKNOWN;
    if (!mv2222_link_is_operational(phydev))
    return 0;
    if (priv.line_interface == PHY_INTERFACE_MODE_10GBASER)
    link = mv2222_read_status_10g(phydev);
    else
    link = mv2222_read_status_1g(phydev);
    if (link < 0)
    return link;
    phydev.link = link;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_resume(phydev: *mut phy_device) -> c_int {
    static int mv2222_resume(struct phy_device *phydev)
    {
    return mv2222_tx_enable(phydev);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_suspend(phydev: *mut phy_device) -> c_int {
    static int mv2222_suspend(struct phy_device *phydev)
    {
    return mv2222_tx_disable(phydev);
    }
#[no_mangle]
unsafe extern "C" fn mv2222_get_features(phydev: *mut phy_device) -> c_int {
    static int mv2222_get_features(struct phy_device *phydev)
    {
// All supported linkmodes are set at probe
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_config_init(phydev: *mut phy_device) -> c_int {
    static int mv2222_config_init(struct phy_device *phydev)
    {
    if (phydev.interface != PHY_INTERFACE_MODE_XAUI)
    return -EINVAL;
    return 0;
    }
    static int mv2222_configure_serdes(struct phy_port *port, bool enable,
    phy_interface_t interface)
    {
    struct phy_device *phydev = port_phydev(port);
    struct mv2222_data *priv;
    let mut ret: c_int = 0;
    priv = phydev.priv;
    priv.line_interface = interface;
    if (enable) {
    linkmode_and(priv.supported, phydev.supported, port.supported);
    ret = mv2222_config_line(phydev);
    if (ret < 0)
    return ret;
    if (mutex_trylock(&phydev.lock)) {
    ret = mv2222_config_aneg(phydev);
    mutex_unlock(&phydev.lock);
    }
    } else {
    linkmode_zero(priv.supported);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_port_link_up(port: *mut phy_port) {
    static void mv2222_port_link_up(struct phy_port *port)
    {
    struct phy_device *phydev = port_phydev(port);
    struct mv2222_data *priv;
    priv = phydev.priv;
    priv.sfp_link = true;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_port_link_down(port: *mut phy_port) {
    static void mv2222_port_link_down(struct phy_port *port)
    {
    struct phy_device *phydev = port_phydev(port);
    struct mv2222_data *priv;
    priv = phydev.priv;
    priv.sfp_link = false;
    }
    static const struct phy_port_ops mv2222_port_ops = {
    .link_up = mv2222_port_link_up,
    .link_down = mv2222_port_link_down,
    .configure_mii = mv2222_configure_serdes,
    };
#[no_mangle]
unsafe extern "C" fn mv2222_attach_mii_port(phydev: *mut phy_device, port: *mut phy_port) -> c_int {
    static int mv2222_attach_mii_port(struct phy_device *phydev, struct phy_port *port)
    {
    port.ops = &mv2222_port_ops;
    __set_bit(PHY_INTERFACE_MODE_10GBASER, port.interfaces);
    __set_bit(PHY_INTERFACE_MODE_1000BASEX, port.interfaces);
    __set_bit(PHY_INTERFACE_MODE_SGMII, port.interfaces);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv2222_probe(phydev: *mut phy_device) -> c_int {
    static int mv2222_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct mv2222_data *priv = core::ptr::null_mut();
    __ETHTOOL_DECLARE_LINK_MODE_MASK(supported) = { 0, };
    linkmode_set_bit(ETHTOOL_LINK_MODE_Autoneg_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_Pause_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_FIBRE_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_TP_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10baseT_Half_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10baseT_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_100baseT_Half_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_100baseT_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_1000baseX_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseT_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseCR_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseSR_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseLR_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseLRM_Full_BIT, supported);
    linkmode_set_bit(ETHTOOL_LINK_MODE_10000baseER_Full_BIT, supported);
    linkmode_copy(phydev.supported, supported);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.line_interface = PHY_INTERFACE_MODE_NA;
    phydev.priv = priv;
    return 0;
    }
    static struct phy_driver mv2222_drivers[] = {
    {
    .phy_id = MARVELL_PHY_ID_88X2222,
    .phy_id_mask = MARVELL_PHY_ID_MASK,
    .name = "Marvell 88X2222",
    .get_features = mv2222_get_features,
    .soft_reset = mv2222_soft_reset,
    .config_init = mv2222_config_init,
    .config_aneg = mv2222_config_aneg,
    .aneg_done = mv2222_aneg_done,
    .probe = mv2222_probe,
    .suspend = mv2222_suspend,
    .resume = mv2222_resume,
    .read_status = mv2222_read_status,
    .attach_mii_port = mv2222_attach_mii_port,
    },
    };
    module_phy_driver(mv2222_drivers);
    static const struct mdio_device_id __maybe_unused mv2222_tbl[] = {
    { MARVELL_PHY_ID_88X2222, MARVELL_PHY_ID_MASK },
    { }
    };
    MODULE_DEVICE_TABLE(mdio, mv2222_tbl);
    MODULE_DESCRIPTION("Marvell 88x2222 ethernet transceiver driver");
    MODULE_LICENSE("GPL");
