//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/enetc/netc_blk_ctrl.c
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
// NXP NETC Blocks Control Driver
//
// Copyright 2024 NXP
//
// This driver is used for pre-initialization of NETC, such as PCS and MII
// protocols, LDID, warm reset, etc. Therefore, all NETC device drivers can
// only be probed after the netc-blk-crtl driver has completed initialization.
// In addition, when the system enters suspend mode, IERB, PRB, and NETCMIX
// will be powered off, except for WOL. Therefore, when the system resumes,
// these blocks need to be reinitialized.
//

// NETCMIX registers
pub const IMX95_CFG_LINK_IO_VAR: c_uint = 0x0;
pub const IO_VAR_16FF_16G_SERDES: c_uint = 0x1;

pub const IMX95_CFG_LINK_MII_PROT: c_uint = 0x4;

pub const MII_PROT_MII: c_uint = 0x0;
pub const MII_PROT_RMII: c_uint = 0x1;
pub const MII_PROT_RGMII: c_uint = 0x2;
pub const MII_PROT_SERIAL: c_uint = 0x3;

pub const IMX94_EXT_PIN_CONTROL: c_uint = 0x10;

// NETC privileged register block register
pub const PRB_NETCRR: c_uint = 0x100;

pub const PRB_NETCSR: c_uint = 0x104;

// NETC integrated endpoint register block register
pub const IERB_EMDIOFAUXR: c_uint = 0x344;
pub const IERB_T0FAUXR: c_uint = 0x444;

// Platform information
pub const IMX95_ENETC0_BUS_DEVFN: c_uint = 0x0;
pub const IMX95_ENETC1_BUS_DEVFN: c_uint = 0x40;
pub const IMX95_ENETC2_BUS_DEVFN: c_uint = 0x80;
pub const IMX94_ENETC0_BUS_DEVFN: c_uint = 0x100;
pub const IMX94_ENETC1_BUS_DEVFN: c_uint = 0x140;
pub const IMX94_ENETC2_BUS_DEVFN: c_uint = 0x180;
pub const IMX94_TIMER0_BUS_DEVFN: c_uint = 0x1;
pub const IMX94_TIMER1_BUS_DEVFN: c_uint = 0x101;
pub const IMX94_TIMER2_BUS_DEVFN: c_uint = 0x181;
pub const IMX94_ENETC0_LINK: c_int = 3;
pub const IMX94_ENETC1_LINK: c_int = 4;
pub const IMX94_ENETC2_LINK: c_int = 5;

// Flags for different platforms

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_devinfo {
    pub flags: u32,
    pub pdev): *mut *mut int (netcmix_init)(struct platform_device,
    pub pdev): *mut *mut int (ierb_init)(struct platform_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_blk_ctrl {
    pub prb: *mut void __iomem,
    pub ierb: *mut void __iomem,
    pub netcmix: *mut void __iomem,
    pub devinfo: *const netc_devinfo,
    pub pdev: *mut platform_device,
    pub debugfs_root: *mut dentry,
}

#[no_mangle]
unsafe extern "C" fn netc_reg_write(base: *mut void __iomem, offset: u32, val: u32) {
    static void netc_reg_write(void __iomem *base, u32 offset, u32 val)
    {
    netc_write(base + offset, val);
    }
#[no_mangle]
unsafe extern "C" fn netc_reg_read(base: *mut void __iomem, offset: u32) -> u32 {
    static u32 netc_reg_read(void __iomem *base, u32 offset)
    {
    return netc_read(base + offset);
    }
#[no_mangle]
unsafe extern "C" fn netc_of_pci_get_bus_devfn(np: *mut device_node) -> c_int {
    static int netc_of_pci_get_bus_devfn(struct device_node *np)
    {
    u32 reg[5];
    int error;
    error = of_property_read_u32_array(np, "reg", reg, ARRAY_SIZE(reg));
    if (error)
    return error;
    return (reg[0] >> 8) & 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn netc_get_link_mii_protocol(interface: phy_interface_t) -> c_int {
    static int netc_get_link_mii_protocol(phy_interface_t interface)
    {
    switch (interface) {
    case PHY_INTERFACE_MODE_MII:
    return MII_PROT_MII;
    case PHY_INTERFACE_MODE_RMII:
    return MII_PROT_RMII;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    return MII_PROT_RGMII;
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_2500BASEX:
    case PHY_INTERFACE_MODE_10GBASER:
    case PHY_INTERFACE_MODE_XGMII:
    case PHY_INTERFACE_MODE_USXGMII:
    return MII_PROT_SERIAL;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn imx95_netcmix_init(pdev: *mut platform_device) -> c_int {
    static int imx95_netcmix_init(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    phy_interface_t interface;
    int bus_devfn, mii_proto;
    u32 val;
    int err;
// Default setting of MII protocol
    val = MII_PROT(0, MII_PROT_RGMII) | MII_PROT(1, MII_PROT_RGMII) |
    MII_PROT(2, MII_PROT_SERIAL);
// Update the link MII protocol through parsing phy-mode
    for_each_available_child_of_node_scoped(np, child) {
    for_each_available_child_of_node_scoped(child, gchild) {
    if (!of_device_is_compatible(gchild, "pci1131,e101"))
    continue;
    bus_devfn = netc_of_pci_get_bus_devfn(gchild);
    if (bus_devfn < 0)
    return -EINVAL;
    if (bus_devfn == IMX95_ENETC2_BUS_DEVFN)
    continue;
    err = of_get_phy_mode(gchild, &interface);
    if (err)
    continue;
    mii_proto = netc_get_link_mii_protocol(interface);
    if (mii_proto < 0)
    return -EINVAL;
    switch (bus_devfn) {
    case IMX95_ENETC0_BUS_DEVFN:
    val = u32_replace_bits(val, mii_proto,
    CFG_LINK_MII_PORT_0);
    break;
    case IMX95_ENETC1_BUS_DEVFN:
    val = u32_replace_bits(val, mii_proto,
    CFG_LINK_MII_PORT_1);
    break;
    default:
    return -EINVAL;
    }
    }
    }
// Configure Link I/O variant
    netc_reg_write(priv.netcmix, IMX95_CFG_LINK_IO_VAR,
    IO_VAR(2, IO_VAR_16FF_16G_SERDES));
// Configure Link 2 PCS protocol
    netc_reg_write(priv.netcmix, IMX95_CFG_LINK_PCS_PROT(2),
    PCS_PROT_10G_SXGMII);
    netc_reg_write(priv.netcmix, IMX95_CFG_LINK_MII_PROT, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx94_enetc_get_link_id(np: *mut device_node) -> c_int {
    static int imx94_enetc_get_link_id(struct device_node *np)
    {
    let mut bus_devfn: c_int = netc_of_pci_get_bus_devfn(np);
// Parse ENETC link number
    switch (bus_devfn) {
    case IMX94_ENETC0_BUS_DEVFN:
    return IMX94_ENETC0_LINK;
    case IMX94_ENETC1_BUS_DEVFN:
    return IMX94_ENETC1_LINK;
    case IMX94_ENETC2_BUS_DEVFN:
    return IMX94_ENETC2_LINK;
    default:
    return -EINVAL;
    }
    }
    static int imx94_link_config(struct netc_blk_ctrl *priv,
    struct device_node *np, int link_id)
    {
    phy_interface_t interface;
    int mii_proto;
    u32 val;
// The node may be disabled and does not have a 'phy-mode'
// or 'phy-connection-type' property.
//
    if (of_get_phy_mode(np, &interface))
    return 0;
    mii_proto = netc_get_link_mii_protocol(interface);
    if (mii_proto < 0)
    return mii_proto;
    val = mii_proto & NETC_LINK_CFG_MII_PROT;
    if (val == MII_PROT_SERIAL)
    val = u32_replace_bits(val, IO_VAR_16FF_16G_SERDES,
    NETC_LINK_CFG_IO_VAR);
    netc_reg_write(priv.netcmix, IMX94_NETC_LINK_CFG(link_id), val);
    return 0;
    }
    static int imx94_enetc_link_config(struct netc_blk_ctrl *priv,
    struct device_node *np,
    bool *enetc0_en)
    {
    let mut link_id: c_int = imx94_enetc_get_link_id(np);
    if (link_id < 0)
    return link_id;
    if (link_id == IMX94_ENETC0_LINK && of_device_is_available(np))
// enetc0_en = true;
    return imx94_link_config(priv, np, link_id);
    }
    static int imx94_switch_link_config(struct netc_blk_ctrl *priv,
    struct device_node *np,
    bool *swp2_en)
    {
    struct device_node *ports;
    u32 port_id;
    let mut err: c_int = 0;
    ports = of_get_child_by_name(np, "ethernet-ports");
    if (!ports)
    return -ENODEV;
// The switch may be owned by a guest OS, in this case, the switch
// node in the host OS will be disabled, but we still hope that the
// host OS could do some configurations for the switch, as the
// netc_blk_ctrl is owned by host OS. So of_device_is_available()
// is not needed here.
//
    for_each_available_child_of_node_scoped(ports, child) {
    if (of_property_read_u32(child, "reg", &port_id) < 0) {
    err = -ENODEV;
    goto end;
    }
    switch (port_id) {
    case 0 ... 2: /* External ports */
    err = imx94_link_config(priv, child, port_id);
    if (err)
    goto end;
    if (port_id == 2)
// swp2_en = true;
    break;
    case 3: /* CPU port */
    break;
    default:
    err = -EINVAL;
    goto end;
    }
    }
    end:
    of_node_put(ports);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn imx94_netcmix_init(pdev: *mut platform_device) -> c_int {
    static int imx94_netcmix_init(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    let mut enetc0_en: bool = false, swp2_en = false;
    u32 val;
    int err;
    for_each_child_of_node_scoped(np, child) {
    for_each_child_of_node_scoped(child, gchild) {
    if (of_device_is_compatible(gchild, "pci1131,e101")) {
    err = imx94_enetc_link_config(priv, gchild,
    &enetc0_en);
    if (err)
    return err;
    } else if (of_device_is_compatible(gchild,
    "pci1131,eef2")) {
    err = imx94_switch_link_config(priv, gchild,
    &swp2_en);
    if (err)
    return err;
    }
    }
    }
    if (enetc0_en && swp2_en) {
    dev_err(&pdev.dev,
    "Cannot enable swp2 and enetc0 at the same time\n");
    return -EINVAL;
    }
// ENETC 0 and switch port 2 share the same parallel interface, they
// cannot be enabled at the same time. The interface is set for the
// ENETC 0 by default unless the switch port 2 is enabled in the DTS.
//
    val = netc_reg_read(priv.netcmix, IMX94_EXT_PIN_CONTROL);
    if (!swp2_en)
    val |= MAC2_MAC3_SEL;
    else
    val &= ~MAC2_MAC3_SEL;
    netc_reg_write(priv.netcmix, IMX94_EXT_PIN_CONTROL, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_ierb_is_locked(priv: *mut netc_blk_ctrl) -> bool {
    static bool netc_ierb_is_locked(struct netc_blk_ctrl *priv)
    {
    return !!(netc_reg_read(priv.prb, PRB_NETCRR) & NETCRR_LOCK);
    }
#[no_mangle]
unsafe extern "C" fn netc_lock_ierb(priv: *mut netc_blk_ctrl) -> c_int {
    static int netc_lock_ierb(struct netc_blk_ctrl *priv)
    {
    u32 val;
    netc_reg_write(priv.prb, PRB_NETCRR, NETCRR_LOCK);
    return read_poll_timeout(netc_reg_read, val, !(val & NETCSR_STATE),
    100, 2000, false, priv.prb, PRB_NETCSR);
    }
#[no_mangle]
unsafe extern "C" fn netc_unlock_ierb_with_warm_reset(priv: *mut netc_blk_ctrl) -> c_int {
    static int netc_unlock_ierb_with_warm_reset(struct netc_blk_ctrl *priv)
    {
    u32 val;
    netc_reg_write(priv.prb, PRB_NETCRR, 0);
    return read_poll_timeout(netc_reg_read, val, !(val & NETCRR_LOCK),
    1000, 100000, true, priv.prb, PRB_NETCRR);
    }
#[no_mangle]
unsafe extern "C" fn netc_get_phy_addr(np: *mut device_node) -> c_int {
    static int netc_get_phy_addr(struct device_node *np)
    {
    struct device_node *mdio_node, *phy_node;
    let mut addr: u32 = 0;
    let mut err: c_int = 0;
    mdio_node = of_get_child_by_name(np, "mdio");
    if (!mdio_node)
    return -ENODEV;
    phy_node = of_get_next_child(mdio_node, core::ptr::null_mut());
    if (!phy_node) {
    err = -ENODEV;
    goto of_put_mdio_node;
    }
    err = of_property_read_u32(phy_node, "reg", &addr);
    if (err)
    goto of_put_phy_node;
    if (addr >= PHY_MAX_ADDR)
    err = -EINVAL;
    of_put_phy_node:
    of_node_put(phy_node);
    of_put_mdio_node:
    of_node_put(mdio_node);
    return err ? err : addr;
    }
#[no_mangle]
unsafe extern "C" fn netc_parse_emdio_phy_mask(np: *mut device_node, phy_mask: *mut u32) -> c_int {
    static int netc_parse_emdio_phy_mask(struct device_node *np, u32 *phy_mask)
    {
    let mut mask: u32 = 0;
    for_each_child_of_node_scoped(np, child) {
    u32 addr;
    int err;
    err = of_property_read_u32(child, "reg", &addr);
    if (err)
    return err;
    if (addr >= PHY_MAX_ADDR)
    return -EINVAL;
    mask |= BIT(addr);
    }
// phy_mask = mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_get_emdio_phy_mask(np: *mut device_node, phy_mask: *mut u32) -> c_int {
    static int netc_get_emdio_phy_mask(struct device_node *np, u32 *phy_mask)
    {
    for_each_child_of_node_scoped(np, child) {
    for_each_child_of_node_scoped(child, gchild) {
    if (!of_device_is_compatible(gchild, "pci1131,ee00"))
    continue;
    return netc_parse_emdio_phy_mask(gchild, phy_mask);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx95_enetc_mdio_phyaddr_config(pdev: *mut platform_device) -> c_int {
    static int imx95_enetc_mdio_phyaddr_config(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    int bus_devfn, addr, err;
    let mut phy_mask: u32 = 0;
    err = netc_get_emdio_phy_mask(np, &phy_mask);
    if (err) {
    dev_err(dev, "Failed to get PHY address mask\n");
    return err;
    }
// Update the port EMDIO PHY address through parsing phy properties.
// This is needed when using the port EMDIO but it's harmless when
// using the central EMDIO. So apply it on all cases.
//
    for_each_child_of_node_scoped(np, child) {
    for_each_child_of_node_scoped(child, gchild) {
    if (!of_device_is_compatible(gchild, "pci1131,e101"))
    continue;
    bus_devfn = netc_of_pci_get_bus_devfn(gchild);
    if (bus_devfn < 0) {
    dev_err(dev, "Failed to get BDF number\n");
    return bus_devfn;
    }
    addr = netc_get_phy_addr(gchild);
    if (addr < 0) {
    if (addr == -ENODEV)
    continue;
    dev_err(dev, "Failed to get PHY address\n");
    return addr;
    }
    if (phy_mask & BIT(addr)) {
    dev_err(dev,
    "Find same PHY address in EMDIO and ENETC node\n");
    return -EINVAL;
    }
    switch (bus_devfn) {
    case IMX95_ENETC0_BUS_DEVFN:
    netc_reg_write(priv.ierb, IERB_LBCR(0),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    break;
    case IMX95_ENETC1_BUS_DEVFN:
    netc_reg_write(priv.ierb, IERB_LBCR(1),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    break;
    case IMX95_ENETC2_BUS_DEVFN:
    netc_reg_write(priv.ierb, IERB_LBCR(2),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    break;
    default:
    break;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx95_ierb_init(pdev: *mut platform_device) -> c_int {
    static int imx95_ierb_init(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
// EMDIO : No MSI-X intterupt
    netc_reg_write(priv.ierb, IERB_EMDIOFAUXR, 0);
// ENETC0 PF
    netc_reg_write(priv.ierb, IERB_EFAUXR(0), 0);
// ENETC0 VF0
    netc_reg_write(priv.ierb, IERB_VFAUXR(0), 1);
// ENETC0 VF1
    netc_reg_write(priv.ierb, IERB_VFAUXR(1), 2);
// ENETC1 PF
    netc_reg_write(priv.ierb, IERB_EFAUXR(1), 3);
// ENETC1 VF0
    netc_reg_write(priv.ierb, IERB_VFAUXR(2), 5);
// ENETC1 VF1
    netc_reg_write(priv.ierb, IERB_VFAUXR(3), 6);
// ENETC2 PF
    netc_reg_write(priv.ierb, IERB_EFAUXR(2), 4);
// ENETC2 VF0
    netc_reg_write(priv.ierb, IERB_VFAUXR(4), 5);
// ENETC2 VF1
    netc_reg_write(priv.ierb, IERB_VFAUXR(5), 6);
// NETC TIMER
    netc_reg_write(priv.ierb, IERB_T0FAUXR, 7);
    return imx95_enetc_mdio_phyaddr_config(pdev);
    }
#[no_mangle]
unsafe extern "C" fn imx94_get_enetc_id(np: *mut device_node) -> c_int {
    static int imx94_get_enetc_id(struct device_node *np)
    {
    let mut bus_devfn: c_int = netc_of_pci_get_bus_devfn(np);
// Parse ENETC offset
    switch (bus_devfn) {
    case IMX94_ENETC0_BUS_DEVFN:
    return NETC_ENETC_ID(0);
    case IMX94_ENETC1_BUS_DEVFN:
    return NETC_ENETC_ID(1);
    case IMX94_ENETC2_BUS_DEVFN:
    return NETC_ENETC_ID(2);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn imx94_get_timer_id(np: *mut device_node) -> c_int {
    static int imx94_get_timer_id(struct device_node *np)
    {
    let mut bus_devfn: c_int = netc_of_pci_get_bus_devfn(np);
// Parse NETC PTP timer ID, the timer0 is on bus 0,
// the timer 1 and timer2 is on bus 1.
//
    switch (bus_devfn) {
    case IMX94_TIMER0_BUS_DEVFN:
    return NETC_TIMER_ID(0);
    case IMX94_TIMER1_BUS_DEVFN:
    return NETC_TIMER_ID(1);
    case IMX94_TIMER2_BUS_DEVFN:
    return NETC_TIMER_ID(2);
    default:
    return -EINVAL;
    }
    }
    static int imx94_enetc_update_tid(struct netc_blk_ctrl *priv,
    struct device_node *np)
    {
    struct device *dev = &priv.pdev.dev;
    struct device_node *timer_np;
    int eid, tid;
    eid = imx94_get_enetc_id(np);
    if (eid < 0) {
    dev_err(dev, "Failed to get ENETC ID\n");
    return eid;
    }
    timer_np = of_parse_phandle(np, "ptp-timer", 0);
    if (!timer_np) {
// If 'ptp-timer' is not present, the timer1 is the default
// timer of all standalone ENETCs, which is on the same PCIe
// bus as these ENETCs.
//
    tid = NETC_TIMER_ID(1);
    goto end;
    }
    tid = imx94_get_timer_id(timer_np);
    of_node_put(timer_np);
    if (tid < 0) {
    dev_err(dev, "Failed to get NETC Timer ID\n");
    return tid;
    }
    end:
    netc_reg_write(priv.ierb, IERB_ETBCR(eid), tid);
    return 0;
    }
    static int imx94_enetc_mdio_phyaddr_config(struct netc_blk_ctrl *priv,
    struct device_node *np,
    u32 phy_mask)
    {
    struct device *dev = &priv.pdev.dev;
    int bus_devfn, addr;
    bus_devfn = netc_of_pci_get_bus_devfn(np);
    if (bus_devfn < 0) {
    dev_err(dev, "Failed to get BDF number\n");
    return bus_devfn;
    }
    addr = netc_get_phy_addr(np);
    if (addr < 0) {
    if (addr == -ENODEV)
    return 0;
    dev_err(dev, "Failed to get PHY address\n");
    return addr;
    }
    if (phy_mask & BIT(addr)) {
    dev_err(dev,
    "Find same PHY address in EMDIO and ENETC node\n");
    return -EINVAL;
    }
    switch (bus_devfn) {
    case IMX94_ENETC0_BUS_DEVFN:
    netc_reg_write(priv.ierb, IERB_LBCR(IMX94_ENETC0_LINK),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    break;
    case IMX94_ENETC1_BUS_DEVFN:
    netc_reg_write(priv.ierb, IERB_LBCR(IMX94_ENETC1_LINK),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    break;
    case IMX94_ENETC2_BUS_DEVFN:
    netc_reg_write(priv.ierb, IERB_LBCR(IMX94_ENETC2_LINK),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    break;
    default:
    break;
    }
    return 0;
    }
    static int imx94_ierb_enetc_init(struct netc_blk_ctrl *priv,
    struct device_node *np,
    u32 phy_mask)
    {
    int err;
    err = imx94_enetc_update_tid(priv, np);
    if (err)
    return err;
    return imx94_enetc_mdio_phyaddr_config(priv, np, phy_mask);
    }
    static int imx94_switch_mdio_phyaddr_config(struct netc_blk_ctrl *priv,
    struct device_node *np,
    u32 port_id, u32 phy_mask)
    {
    int addr;
// The switch has 3 external ports at most
    if (port_id > 2)
    return 0;
    addr = netc_get_phy_addr(np);
    if (addr < 0) {
    if (addr == -ENODEV)
    return 0;
    return addr;
    }
    if (phy_mask & BIT(addr)) {
    dev_err(&priv.pdev.dev,
    "Found same PHY address in EMDIO and switch node\n");
    return -EINVAL;
    }
    netc_reg_write(priv.ierb, IERB_LBCR(port_id),
    LBCR_MDIO_PHYAD_PRTAD(addr));
    return 0;
    }
    static int imx94_ierb_switch_init(struct netc_blk_ctrl *priv,
    struct device_node *np,
    u32 phy_mask)
    {
    struct device_node *ports;
    u32 port_id;
    let mut err: c_int = 0;
    ports = of_get_child_by_name(np, "ethernet-ports");
    if (!ports)
    return -ENODEV;
    for_each_available_child_of_node_scoped(ports, child) {
    err = of_property_read_u32(child, "reg", &port_id);
    if (err)
    goto end;
    err = imx94_switch_mdio_phyaddr_config(priv, child,
    port_id, phy_mask);
    if (err)
    goto end;
    }
    end:
    of_node_put(ports);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn imx94_ierb_init(pdev: *mut platform_device) -> c_int {
    static int imx94_ierb_init(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    let mut phy_mask: u32 = 0;
    int err;
    err = netc_get_emdio_phy_mask(np, &phy_mask);
    if (err) {
    dev_err(&pdev.dev, "Failed to get PHY address mask\n");
    return err;
    }
    for_each_child_of_node_scoped(np, child) {
    for_each_child_of_node_scoped(child, gchild) {
    if (of_device_is_compatible(gchild, "pci1131,e101")) {
    err = imx94_ierb_enetc_init(priv, gchild,
    phy_mask);
    if (err)
    return err;
    } else if (of_device_is_compatible(gchild,
    "pci1131,eef2")) {
    err = imx94_ierb_switch_init(priv, gchild,
    phy_mask);
    if (err)
    return err;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_ierb_init(pdev: *mut platform_device) -> c_int {
    static int netc_ierb_init(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
    const struct netc_devinfo *devinfo = priv.devinfo;
    int err;
    if (netc_ierb_is_locked(priv)) {
    err = netc_unlock_ierb_with_warm_reset(priv);
    if (err) {
    dev_err(&pdev.dev, "Unlock IERB failed.\n");
    return err;
    }
    }
    if (devinfo.ierb_init) {
    err = devinfo.ierb_init(pdev);
    if (err)
    return err;
    }
    err = netc_lock_ierb(priv);
    if (err) {
    dev_err(&pdev.dev, "Lock IERB failed.\n");
    return err;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn netc_prb_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int netc_prb_show(struct seq_file *s, void *data)
    {
    struct netc_blk_ctrl *priv = s.private;
    u32 val;
    val = netc_reg_read(priv.prb, PRB_NETCRR);
    seq_printf(s, "[PRB NETCRR] Lock:%d SR:%d\n",
    (val & NETCRR_LOCK) ? 1 : 0,
    (val & NETCRR_SR) ? 1 : 0);
    val = netc_reg_read(priv.prb, PRB_NETCSR);
    seq_printf(s, "[PRB NETCSR] State:%d Error:%d\n",
    (val & NETCSR_STATE) ? 1 : 0,
    (val & NETCSR_ERROR) ? 1 : 0);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(netc_prb);
#[no_mangle]
unsafe extern "C" fn netc_blk_ctrl_create_debugfs(priv: *mut netc_blk_ctrl) {
    static void netc_blk_ctrl_create_debugfs(struct netc_blk_ctrl *priv)
    {
    struct dentry *root;
    root = debugfs_create_dir("netc_blk_ctrl", core::ptr::null_mut());
    if (IS_ERR(root))
    return;
    priv.debugfs_root = root;
    debugfs_create_file("prb", 0444, root, priv, &netc_prb_fops);
    }
#[no_mangle]
unsafe extern "C" fn netc_blk_ctrl_remove_debugfs(priv: *mut netc_blk_ctrl) {
    static void netc_blk_ctrl_remove_debugfs(struct netc_blk_ctrl *priv)
    {
    debugfs_remove_recursive(priv.debugfs_root);
    priv.debugfs_root = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn netc_blk_ctrl_create_debugfs(priv: *mut netc_blk_ctrl) {
    static void netc_blk_ctrl_create_debugfs(struct netc_blk_ctrl *priv)
    {
    }
#[no_mangle]
unsafe extern "C" fn netc_blk_ctrl_remove_debugfs(priv: *mut netc_blk_ctrl) {
    static void netc_blk_ctrl_remove_debugfs(struct netc_blk_ctrl *priv)
    {
    }

#[no_mangle]
unsafe extern "C" fn netc_prb_check_error(priv: *mut netc_blk_ctrl) -> c_int {
    static int netc_prb_check_error(struct netc_blk_ctrl *priv)
    {
    if (netc_reg_read(priv.prb, PRB_NETCSR) & NETCSR_ERROR)
    return -1;
    return 0;
    }
    static const struct netc_devinfo imx95_devinfo = {
    .flags = NETC_HAS_NETCMIX,
    .netcmix_init = imx95_netcmix_init,
    .ierb_init = imx95_ierb_init,
    };
    static const struct netc_devinfo imx94_devinfo = {
    .flags = NETC_HAS_NETCMIX,
    .netcmix_init = imx94_netcmix_init,
    .ierb_init = imx94_ierb_init,
    };
    static const struct of_device_id netc_blk_ctrl_match[] = {
    { .compatible = "nxp,imx95-netc-blk-ctrl", .data = &imx95_devinfo },
    { .compatible = "nxp,imx94-netc-blk-ctrl", .data = &imx94_devinfo },
    {},
    };
    MODULE_DEVICE_TABLE(of, netc_blk_ctrl_match);
#[no_mangle]
unsafe extern "C" fn netc_blk_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int netc_blk_ctrl_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    const struct netc_devinfo *devinfo;
    struct device *dev = &pdev.dev;
    const struct of_device_id *id;
    struct netc_blk_ctrl *priv;
    struct clk *ipg_clk;
    void __iomem *regs;
    int err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.pdev = pdev;
    ipg_clk = devm_clk_get_optional_enabled(dev, "ipg");
    if (IS_ERR(ipg_clk))
    return dev_err_probe(dev, PTR_ERR(ipg_clk),
    "Set ipg clock failed\n");
    id = of_match_device(netc_blk_ctrl_match, dev);
    if (!id)
    return dev_err_probe(dev, -EINVAL, "Cannot match device\n");
    devinfo = (struct netc_devinfo *)id.data;
    if (!devinfo)
    return dev_err_probe(dev, -EINVAL, "No device information\n");
    priv.devinfo = devinfo;
    regs = devm_platform_ioremap_resource_byname(pdev, "ierb");
    if (IS_ERR(regs))
    return dev_err_probe(dev, PTR_ERR(regs),
    "Missing IERB resource\n");
    priv.ierb = regs;
    regs = devm_platform_ioremap_resource_byname(pdev, "prb");
    if (IS_ERR(regs))
    return dev_err_probe(dev, PTR_ERR(regs),
    "Missing PRB resource\n");
    priv.prb = regs;
    if (devinfo.flags & NETC_HAS_NETCMIX) {
    regs = devm_platform_ioremap_resource_byname(pdev, "netcmix");
    if (IS_ERR(regs))
    return dev_err_probe(dev, PTR_ERR(regs),
    "Missing NETCMIX resource\n");
    priv.netcmix = regs;
    }
    platform_set_drvdata(pdev, priv);
    if (devinfo.netcmix_init) {
    err = devinfo.netcmix_init(pdev);
    if (err)
    return dev_err_probe(dev, err,
    "Initializing NETCMIX failed\n");
    }
    err = netc_ierb_init(pdev);
    if (err)
    return dev_err_probe(dev, err, "Initializing IERB failed\n");
    if (netc_prb_check_error(priv) < 0)
    dev_warn(dev, "The current IERB configuration is invalid\n");
    netc_blk_ctrl_create_debugfs(priv);
    err = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (err) {
    netc_blk_ctrl_remove_debugfs(priv);
    return dev_err_probe(dev, err, "of_platform_populate failed\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netc_blk_ctrl_remove(pdev: *mut platform_device) {
    static void netc_blk_ctrl_remove(struct platform_device *pdev)
    {
    struct netc_blk_ctrl *priv = platform_get_drvdata(pdev);
    of_platform_depopulate(&pdev.dev);
    netc_blk_ctrl_remove_debugfs(priv);
    }
    static struct platform_driver netc_blk_ctrl_driver = {
    .driver = {
    .name = "nxp-netc-blk-ctrl",
    .of_match_table = netc_blk_ctrl_match,
    },
    .probe = netc_blk_ctrl_probe,
    .remove = netc_blk_ctrl_remove,
    };
    module_platform_driver(netc_blk_ctrl_driver);
    MODULE_DESCRIPTION("NXP NETC Blocks Control Driver");
    MODULE_LICENSE("Dual BSD/GPL");
