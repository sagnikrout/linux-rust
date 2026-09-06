//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/tehuti/tn40_mdio.c
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
// Copyright (c) Tehuti Networks Ltd.

    (FIELD_PREP(TN40_MDIO_DEVAD_MASK, (device)) |	\
    (FIELD_PREP(TN40_MDIO_PRTAD_MASK, (port))))

#[no_mangle]
unsafe extern "C" fn tn40_mdio_set_speed(priv: *mut tn40_priv, speed: u32) {
    static void tn40_mdio_set_speed(struct tn40_priv *priv, u32 speed)
    {
    void __iomem *regs = priv.regs;
    int mdio_cfg;
    if (speed == TN40_MDIO_SPEED_1MHZ)
    mdio_cfg = (0x7d << 7) | 0x08;	/* 1MHz */
    else
    mdio_cfg = 0xA08;	/* 6MHz */
    mdio_cfg |= (1 << 6);
    writel(mdio_cfg, regs + TN40_REG_MDIO_CMD_STAT);
    msleep(100);
    }
#[no_mangle]
unsafe extern "C" fn tn40_mdio_stat(priv: *mut tn40_priv) -> u32 {
    static u32 tn40_mdio_stat(struct tn40_priv *priv)
    {
    void __iomem *regs = priv.regs;
    return readl(regs + TN40_REG_MDIO_CMD_STAT);
    }
#[no_mangle]
unsafe extern "C" fn tn40_mdio_wait_nobusy(priv: *mut tn40_priv, val: *mut u32) -> c_int {
    static int tn40_mdio_wait_nobusy(struct tn40_priv *priv, u32 *val)
    {
    u32 stat;
    int ret;
    ret = readx_poll_timeout_atomic(tn40_mdio_stat, priv, stat,
    TN40_GET_MDIO_BUSY(stat) == 0, 10,
    10000);
    if (val)
// val = stat;
    return ret;
    }
    static int tn40_mdio_read(struct tn40_priv *priv, int port, int device,
    u16 regnum)
    {
    void __iomem *regs = priv.regs;
    u32 i;
// wait until MDIO is not busy
    if (tn40_mdio_wait_nobusy(priv, core::ptr::null_mut()))
    return -EIO;
    i = TN40_MDIO_CMD_VAL(device, port);
    writel(i, regs + TN40_REG_MDIO_CMD);
    writel((u32)regnum, regs + TN40_REG_MDIO_ADDR);
    if (tn40_mdio_wait_nobusy(priv, core::ptr::null_mut()))
    return -EIO;
    writel(TN40_MDIO_CMD_READ | i, regs + TN40_REG_MDIO_CMD);
// read CMD_STAT until not busy
    if (tn40_mdio_wait_nobusy(priv, core::ptr::null_mut()))
    return -EIO;
    return lower_16_bits(readl(regs + TN40_REG_MDIO_DATA));
    }
    static int tn40_mdio_write(struct tn40_priv *priv, int port, int device,
    u16 regnum, u16 data)
    {
    void __iomem *regs = priv.regs;
    let mut tmp_reg: u32 = 0;
    int ret;
// wait until MDIO is not busy
    if (tn40_mdio_wait_nobusy(priv, core::ptr::null_mut()))
    return -EIO;
    writel(TN40_MDIO_CMD_VAL(device, port), regs + TN40_REG_MDIO_CMD);
    writel((u32)regnum, regs + TN40_REG_MDIO_ADDR);
    if (tn40_mdio_wait_nobusy(priv, core::ptr::null_mut()))
    return -EIO;
    writel((u32)data, regs + TN40_REG_MDIO_DATA);
// read CMD_STAT until not busy
    ret = tn40_mdio_wait_nobusy(priv, &tmp_reg);
    if (ret)
    return -EIO;
    if (TN40_GET_MDIO_RD_ERR(tmp_reg)) {
    dev_err(&priv.pdev.dev, "MDIO error after write command\n");
    return -EIO;
    }
    return 0;
    }
    static int tn40_mdio_read_c45(struct mii_bus *mii_bus, int addr, int devnum,
    int regnum)
    {
    return tn40_mdio_read(mii_bus.priv, addr, devnum, regnum);
    }
    static int tn40_mdio_write_c45(struct mii_bus *mii_bus, int addr, int devnum,
    int regnum, u16 val)
    {
    return  tn40_mdio_write(mii_bus.priv, addr, devnum, regnum, val);
    }
// registers an mdio node and an aqr105 PHY at address 1
// tn40_mdio-%id {
// ethernet-phy@1 {
// compatible = "ethernet-phy-id03a1.b4a3";
// reg = <1>;
// firmware-name = AQR105_FIRMWARE;
// };
//
#[no_mangle]
unsafe extern "C" fn tn40_swnodes_register(priv: *mut tn40_priv) -> c_int {
    static int tn40_swnodes_register(struct tn40_priv *priv)
    {
    struct tn40_nodes *nodes = &priv.nodes;
    struct pci_dev *pdev = priv.pdev;
    struct software_node *swnodes;
    u32 id;
    id = pci_dev_id(pdev);
    snprintf(nodes.phy_name, sizeof(nodes.phy_name), "ethernet-phy@1");
    snprintf(nodes.mdio_name, sizeof(nodes.mdio_name), "tn40_mdio-%x",
    id);
    swnodes = nodes.swnodes;
    swnodes[SWNODE_MDIO] = NODE_PROP(nodes.mdio_name, core::ptr::null_mut());
    nodes.phy_props[0] = PROPERTY_ENTRY_STRING("compatible",
    "ethernet-phy-id03a1.b4a3");
    nodes.phy_props[1] = PROPERTY_ENTRY_U32("reg", 1);
    nodes.phy_props[2] = PROPERTY_ENTRY_STRING("firmware-name",
    AQR105_FIRMWARE);
    swnodes[SWNODE_PHY] = NODE_PAR_PROP(nodes.phy_name,
    &swnodes[SWNODE_MDIO],
    nodes.phy_props);
    nodes.group[SWNODE_PHY] = &swnodes[SWNODE_PHY];
    nodes.group[SWNODE_MDIO] = &swnodes[SWNODE_MDIO];
    return software_node_register_node_group(nodes.group);
    }
#[no_mangle]
pub unsafe extern "C" fn tn40_swnodes_cleanup(priv: *mut tn40_priv) {
    void tn40_swnodes_cleanup(struct tn40_priv *priv)
    {
// cleanup of swnodes is only needed for AQR105-based cards
    if (priv.pdev.device == PCI_DEVICE_ID_TEHUTI_TN9510) {
    fwnode_handle_put(dev_fwnode(&priv.mdio.dev));
    device_remove_software_node(&priv.mdio.dev);
    software_node_unregister_node_group(priv.nodes.group);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tn40_mdiobus_init(priv: *mut tn40_priv) -> c_int {
    int tn40_mdiobus_init(struct tn40_priv *priv)
    {
    struct pci_dev *pdev = priv.pdev;
    struct mii_bus *bus;
    int ret;
    bus = devm_mdiobus_alloc(&pdev.dev);
    if (!bus)
    return -ENOMEM;
    bus.name = TN40_DRV_NAME;
    bus.parent = &pdev.dev;
    snprintf(bus.id, MII_BUS_ID_SIZE, "tn40xx-%x-%x",
    pci_domain_nr(pdev.bus), pci_dev_id(pdev));
    bus.priv = priv;
    bus.read_c45 = tn40_mdio_read_c45;
    bus.write_c45 = tn40_mdio_write_c45;
    priv.mdio = bus;
// provide swnodes for AQR105-based cards only
    if (pdev.device == PCI_DEVICE_ID_TEHUTI_TN9510) {
    ret = tn40_swnodes_register(priv);
    if (ret) {
    pr_err("swnodes failed\n");
    return ret;
    }
    ret = device_add_software_node(&bus.dev,
    priv.nodes.group[SWNODE_MDIO]);
    if (ret) {
    dev_err(&pdev.dev,
    "device_add_software_node failed: %d\n", ret);
    goto err_swnodes_unregister;
    }
    }
    tn40_mdio_set_speed(priv, TN40_MDIO_SPEED_6MHZ);
    ret = devm_mdiobus_register(&pdev.dev, bus);
    if (ret) {
    dev_err(&pdev.dev, "failed to register mdiobus %d %u %u\n",
    ret, bus.state, MDIOBUS_UNREGISTERED);
    goto err_swnodes_cleanup;
    }
    return 0;
    err_swnodes_unregister:
    software_node_unregister_node_group(priv.nodes.group);
    return ret;
    err_swnodes_cleanup:
    tn40_swnodes_cleanup(priv);
    return ret;
    }
    MODULE_FIRMWARE(AQR105_FIRMWARE);
