//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-socfpga.c
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
// Copyright Altera Corporation (C) 2014. All rights reserved.
//
// Adopted from dwmac-sti.c
//

pub const SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_GMII_MII: c_uint = 0x0;
pub const SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_RGMII: c_uint = 0x1;
pub const SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_RMII: c_uint = 0x2;
pub const SYSMGR_EMACGRP_CTRL_PHYSEL_WIDTH: c_int = 2;
pub const SYSMGR_EMACGRP_CTRL_PHYSEL_MASK: c_uint = 0x00000003;
pub const SYSMGR_EMACGRP_CTRL_PTP_REF_CLK_MASK: c_uint = 0x00000010;
pub const SYSMGR_GEN10_EMACGRP_CTRL_PTP_REF_CLK_MASK: c_uint = 0x00000100;
pub const SYSMGR_FPGAGRP_MODULE_REG: c_uint = 0x00000028;
pub const SYSMGR_FPGAGRP_MODULE_EMAC: c_uint = 0x00000004;
pub const SYSMGR_FPGAINTF_EMAC_REG: c_uint = 0x00000070;
pub const SYSMGR_FPGAINTF_EMAC_BIT: c_uint = 0x1;
pub const EMAC_SPLITTER_CTRL_REG: c_uint = 0x0;
pub const EMAC_SPLITTER_CTRL_SPEED_MASK: c_uint = 0x3;
pub const EMAC_SPLITTER_CTRL_SPEED_10: c_uint = 0x2;
pub const EMAC_SPLITTER_CTRL_SPEED_100: c_uint = 0x3;
pub const EMAC_SPLITTER_CTRL_SPEED_1000: c_uint = 0x0;
pub const SGMII_ADAPTER_CTRL_REG: c_uint = 0x00;
pub const SGMII_ADAPTER_ENABLE: c_uint = 0x0000;
pub const SGMII_ADAPTER_DISABLE: c_uint = 0x0001;
pub const SMTG_MDIO_ADDR: c_uint = 0x15;
pub const SMTG_TSC_WORD0: c_uint = 0xC;
pub const SMTG_TSC_WORD1: c_uint = 0xD;
pub const SMTG_TSC_WORD2: c_uint = 0xE;
pub const SMTG_TSC_WORD3: c_uint = 0xF;
pub const SMTG_TSC_SHIFT: c_int = 16;
    struct socfpga_dwmac;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_dwmac_ops {
    int (*set_phy_mode)(struct socfpga_dwmac *dwmac_priv,
    pub dev): *mut device,
    pub dwmac_priv): *mut *mut void (setup_plat_dat)(struct socfpga_dwmac,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_dwmac {
    pub reg_offset: u32,
    pub reg_shift: u32,
    pub plat_dat: *mut plat_stmmacenet_data,
    pub sys_mgr_base_addr: *mut regmap,
    pub stmmac_rst: *mut reset_control,
    pub stmmac_ocp_rst: *mut reset_control,
    pub splitter_base: *mut void __iomem,
    pub tse_pcs_base: *mut void __iomem,
    pub sgmii_adapter_base: *mut void __iomem,
    pub f2h_ptp_ref_clk: bool,
    pub ops: *const socfpga_dwmac_ops,
}

#[no_mangle]
unsafe extern "C" fn socfpga_get_plat_phymode(dwmac: *mut socfpga_dwmac) -> phy_interface_t {
    static phy_interface_t socfpga_get_plat_phymode(struct socfpga_dwmac *dwmac)
    {
    return dwmac.plat_dat.phy_interface;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_sgmii_config(dwmac: *mut socfpga_dwmac, enable: bool) {
    static void socfpga_sgmii_config(struct socfpga_dwmac *dwmac, bool enable)
    {
    let mut val: u16 = enable ? SGMII_ADAPTER_ENABLE : SGMII_ADAPTER_DISABLE;
    writew(val, dwmac.sgmii_adapter_base + SGMII_ADAPTER_CTRL_REG);
    }
    static void socfpga_dwmac_fix_mac_speed(void *bsp_priv,
    phy_interface_t interface, int speed,
    unsigned int mode)
    {
    struct socfpga_dwmac *dwmac = (struct socfpga_dwmac *)bsp_priv;
    void __iomem *sgmii_adapter_base = dwmac.sgmii_adapter_base;
    let mut phymode: phy_interface_t = socfpga_get_plat_phymode(dwmac);
    void __iomem *splitter_base = dwmac.splitter_base;
    u32 val;
    if (sgmii_adapter_base)
    socfpga_sgmii_config(dwmac, false);
    if (splitter_base) {
    val = readl(splitter_base + EMAC_SPLITTER_CTRL_REG);
    val &= ~EMAC_SPLITTER_CTRL_SPEED_MASK;
    switch (speed) {
    case 1000:
    val |= EMAC_SPLITTER_CTRL_SPEED_1000;
    break;
    case 100:
    val |= EMAC_SPLITTER_CTRL_SPEED_100;
    break;
    case 10:
    val |= EMAC_SPLITTER_CTRL_SPEED_10;
    break;
    default:
    return;
    }
    writel(val, splitter_base + EMAC_SPLITTER_CTRL_REG);
    }
    if ((phymode == PHY_INTERFACE_MODE_SGMII ||
    phymode == PHY_INTERFACE_MODE_1000BASEX) && sgmii_adapter_base)
    socfpga_sgmii_config(dwmac, true);
    }
#[no_mangle]
unsafe extern "C" fn socfpga_dwmac_parse_data(dwmac: *mut socfpga_dwmac, dev: *mut device) -> c_int {
    static int socfpga_dwmac_parse_data(struct socfpga_dwmac *dwmac, struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct regmap *sys_mgr_base_addr;
    u32 reg_offset, reg_shift;
    int ret, index;
    struct device_node *np_splitter = core::ptr::null_mut();
    struct device_node *np_sgmii_adapter = core::ptr::null_mut();
    struct resource res_splitter;
    struct resource res_tse_pcs;
    struct resource res_sgmii_adapter;
    sys_mgr_base_addr =
    altr_sysmgr_regmap_lookup_by_phandle(np, "altr,sysmgr-syscon");
    if (IS_ERR(sys_mgr_base_addr)) {
    dev_info(dev, "No sysmgr-syscon node found\n");
    return PTR_ERR(sys_mgr_base_addr);
    }
    ret = of_property_read_u32_index(np, "altr,sysmgr-syscon", 1, &reg_offset);
    if (ret) {
    dev_info(dev, "Could not read reg_offset from sysmgr-syscon!\n");
    return -EINVAL;
    }
    ret = of_property_read_u32_index(np, "altr,sysmgr-syscon", 2, &reg_shift);
    if (ret) {
    dev_info(dev, "Could not read reg_shift from sysmgr-syscon!\n");
    return -EINVAL;
    }
    dwmac.f2h_ptp_ref_clk = of_property_read_bool(np, "altr,f2h_ptp_ref_clk");
    np_splitter = of_parse_phandle(np, "altr,emac-splitter", 0);
    if (np_splitter) {
    ret = of_address_to_resource(np_splitter, 0, &res_splitter);
    of_node_put(np_splitter);
    if (ret) {
    dev_info(dev, "Missing emac splitter address\n");
    return -EINVAL;
    }
    dwmac.splitter_base = devm_ioremap_resource(dev, &res_splitter);
    if (IS_ERR(dwmac.splitter_base)) {
    dev_info(dev, "Failed to mapping emac splitter\n");
    return PTR_ERR(dwmac.splitter_base);
    }
    }
    np_sgmii_adapter = of_parse_phandle(np,
    "altr,gmii-to-sgmii-converter", 0);
    if (np_sgmii_adapter) {
    index = of_property_match_string(np_sgmii_adapter, "reg-names",
    "hps_emac_interface_splitter_avalon_slave");
    if (index >= 0) {
    if (of_address_to_resource(np_sgmii_adapter, index,
    &res_splitter)) {
    dev_err(dev,
    "%s: ERROR: missing emac splitter address\n",
    __func__);
    ret = -EINVAL;
    goto err_node_put;
    }
    dwmac.splitter_base =
    devm_ioremap_resource(dev, &res_splitter);
    if (IS_ERR(dwmac.splitter_base)) {
    ret = PTR_ERR(dwmac.splitter_base);
    goto err_node_put;
    }
    }
    index = of_property_match_string(np_sgmii_adapter, "reg-names",
    "gmii_to_sgmii_adapter_avalon_slave");
    if (index >= 0) {
    if (of_address_to_resource(np_sgmii_adapter, index,
    &res_sgmii_adapter)) {
    dev_err(dev,
    "%s: ERROR: failed mapping adapter\n",
    __func__);
    ret = -EINVAL;
    goto err_node_put;
    }
    dwmac.sgmii_adapter_base =
    devm_ioremap_resource(dev, &res_sgmii_adapter);
    if (IS_ERR(dwmac.sgmii_adapter_base)) {
    ret = PTR_ERR(dwmac.sgmii_adapter_base);
    goto err_node_put;
    }
    }
    index = of_property_match_string(np_sgmii_adapter, "reg-names",
    "eth_tse_control_port");
    if (index >= 0) {
    if (of_address_to_resource(np_sgmii_adapter, index,
    &res_tse_pcs)) {
    dev_err(dev,
    "%s: ERROR: failed mapping tse control port\n",
    __func__);
    ret = -EINVAL;
    goto err_node_put;
    }
    dwmac.tse_pcs_base =
    devm_ioremap_resource(dev, &res_tse_pcs);
    if (IS_ERR(dwmac.tse_pcs_base)) {
    ret = PTR_ERR(dwmac.tse_pcs_base);
    goto err_node_put;
    }
    }
    }
    dwmac.reg_offset = reg_offset;
    dwmac.reg_shift = reg_shift;
    dwmac.sys_mgr_base_addr = sys_mgr_base_addr;
    of_node_put(np_sgmii_adapter);
    return 0;
    err_node_put:
    of_node_put(np_sgmii_adapter);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_set_phy_mode_common(phymode: c_int, val: *mut u32) -> c_int {
    static int socfpga_set_phy_mode_common(int phymode, u32 *val)
    {
    switch (phymode) {
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
// val = SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_RGMII;
    break;
    case PHY_INTERFACE_MODE_MII:
    case PHY_INTERFACE_MODE_GMII:
    case PHY_INTERFACE_MODE_SGMII:
    case PHY_INTERFACE_MODE_1000BASEX:
// val = SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_GMII_MII;
    break;
    case PHY_INTERFACE_MODE_RMII:
// val = SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_RMII;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_smtgtime(mii: *mut mii_bus, smtg_addr: c_int, smtg_time: *mut u64) {
    static void get_smtgtime(struct mii_bus *mii, int smtg_addr, u64 *smtg_time)
    {
    u64 ns;
    ns = mdiobus_read(mii, smtg_addr, SMTG_TSC_WORD3);
    ns <<= SMTG_TSC_SHIFT;
    ns |= mdiobus_read(mii, smtg_addr, SMTG_TSC_WORD2);
    ns <<= SMTG_TSC_SHIFT;
    ns |= mdiobus_read(mii, smtg_addr, SMTG_TSC_WORD1);
    ns <<= SMTG_TSC_SHIFT;
    ns |= mdiobus_read(mii, smtg_addr, SMTG_TSC_WORD0);
// smtg_time = ns;
    }
    static int smtg_crosststamp(ktime_t *device, struct system_counterval_t *system,
    void *ctx)
    {
    struct stmmac_priv *priv = (struct stmmac_priv *)ctx;
    u32 num_snapshot, gpio_value, acr_value;
    void __iomem *ptpaddr = priv.ptpaddr;
    void __iomem *ioaddr = priv.hw.pcsr;
    unsigned long flags;
    let mut smtg_time: u64 = 0;
    let mut ptp_time: u64 = 0;
    int i, ret;
    u32 v;
// Both internal crosstimestamping and external triggered event
// timestamping cannot be run concurrently.
//
    if (priv.plat.flags & STMMAC_FLAG_EXT_SNAPSHOT_EN)
    return -EBUSY;
    mutex_lock(&priv.aux_ts_lock);
// Enable Internal snapshot trigger
    acr_value = readl(ptpaddr + PTP_ACR);
    acr_value &= ~PTP_ACR_MASK;
    switch (priv.plat.int_snapshot_num) {
    case AUX_SNAPSHOT0:
    acr_value |= PTP_ACR_ATSEN0;
    break;
    case AUX_SNAPSHOT1:
    acr_value |= PTP_ACR_ATSEN1;
    break;
    case AUX_SNAPSHOT2:
    acr_value |= PTP_ACR_ATSEN2;
    break;
    case AUX_SNAPSHOT3:
    acr_value |= PTP_ACR_ATSEN3;
    break;
    default:
    mutex_unlock(&priv.aux_ts_lock);
    return -EINVAL;
    }
    writel(acr_value, ptpaddr + PTP_ACR);
// Clear FIFO
    acr_value = readl(ptpaddr + PTP_ACR);
    acr_value |= PTP_ACR_ATSFC;
    writel(acr_value, ptpaddr + PTP_ACR);
// Release the mutex
    mutex_unlock(&priv.aux_ts_lock);
// Trigger Internal snapshot signal. Create a rising edge by just toggle
// the GPO0 to low and back to high.
//
    gpio_value = readl(ioaddr + XGMAC_GPIO_STATUS);
    gpio_value &= ~XGMAC_GPIO_GPO0;
    writel(gpio_value, ioaddr + XGMAC_GPIO_STATUS);
    gpio_value |= XGMAC_GPIO_GPO0;
    writel(gpio_value, ioaddr + XGMAC_GPIO_STATUS);
// Poll for time sync operation done
    ret = readl_poll_timeout(priv.ioaddr + XGMAC_INT_STATUS, v,
    (v & XGMAC_INT_TSIS), 100, 10000);
    if (ret) {
    netdev_err(priv.dev, "%s: Wait for time sync operation timeout\n",
    __func__);
    return ret;
    }
// system = (struct system_counterval_t) {
    .cycles = 0,
    .cs_id = CSID_ARM_ARCH_COUNTER,
    .use_nsecs = false,
    };
    num_snapshot = FIELD_GET(XGMAC_TIMESTAMP_ATSNS_MASK,
    readl(ioaddr + XGMAC_TIMESTAMP_STATUS));
// Repeat until the timestamps are from the FIFO last segment
    for (i = 0; i < num_snapshot; i++) {
    read_lock_irqsave(&priv.ptp_lock, flags);
    stmmac_get_ptptime(priv, ptpaddr, &ptp_time);
// device = ns_to_ktime(ptp_time);
    read_unlock_irqrestore(&priv.ptp_lock, flags);
    }
    get_smtgtime(priv.mii, SMTG_MDIO_ADDR, &smtg_time);
    system.cycles = smtg_time;
    return 0;
    }
    static int socfpga_gen5_set_phy_mode(struct socfpga_dwmac *dwmac,
    struct device *dev)
    {
    struct regmap *sys_mgr_base_addr = dwmac.sys_mgr_base_addr;
    let mut phymode: phy_interface_t = socfpga_get_plat_phymode(dwmac);
    let mut reg_offset: u32 = dwmac.reg_offset;
    let mut reg_shift: u32 = dwmac.reg_shift;
    u32 ctrl, val, module;
    if (socfpga_set_phy_mode_common(phymode, &val)) {
    dev_err(dev, "bad phy mode %d\n", phymode);
    return -EINVAL;
    }
// Overwrite val to GMII if splitter core is enabled. The phymode here
// is the actual phy mode on phy hardware, but phy interface from
// EMAC core is GMII.
//
    if (dwmac.splitter_base)
    val = SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_GMII_MII;
// Assert reset to the enet controller before changing the phy mode
    reset_control_assert(dwmac.stmmac_ocp_rst);
    reset_control_assert(dwmac.stmmac_rst);
    regmap_read(sys_mgr_base_addr, reg_offset, &ctrl);
    ctrl &= ~(SYSMGR_EMACGRP_CTRL_PHYSEL_MASK << reg_shift);
    ctrl |= val << reg_shift;
    if (dwmac.f2h_ptp_ref_clk ||
    phymode == PHY_INTERFACE_MODE_MII ||
    phymode == PHY_INTERFACE_MODE_GMII ||
    phymode == PHY_INTERFACE_MODE_SGMII) {
    regmap_read(sys_mgr_base_addr, SYSMGR_FPGAGRP_MODULE_REG,
    &module);
    module |= (SYSMGR_FPGAGRP_MODULE_EMAC << (reg_shift / 2));
    regmap_write(sys_mgr_base_addr, SYSMGR_FPGAGRP_MODULE_REG,
    module);
    }
    if (dwmac.f2h_ptp_ref_clk)
    ctrl |= SYSMGR_EMACGRP_CTRL_PTP_REF_CLK_MASK << (reg_shift / 2);
    else
    ctrl &= ~(SYSMGR_EMACGRP_CTRL_PTP_REF_CLK_MASK <<
    (reg_shift / 2));
    regmap_write(sys_mgr_base_addr, reg_offset, ctrl);
// Deassert reset for the phy configuration to be sampled by
// the enet controller, and operation to start in requested mode
//
    reset_control_deassert(dwmac.stmmac_ocp_rst);
    reset_control_deassert(dwmac.stmmac_rst);
    if (phymode == PHY_INTERFACE_MODE_SGMII)
    socfpga_sgmii_config(dwmac, true);
    return 0;
    }
    static int socfpga_gen10_set_phy_mode(struct socfpga_dwmac *dwmac,
    struct device *dev)
    {
    struct regmap *sys_mgr_base_addr = dwmac.sys_mgr_base_addr;
    let mut phymode: phy_interface_t = socfpga_get_plat_phymode(dwmac);
    let mut reg_offset: u32 = dwmac.reg_offset;
    let mut reg_shift: u32 = dwmac.reg_shift;
    u32 ctrl, val, module;
    if (socfpga_set_phy_mode_common(phymode, &val))
    return -EINVAL;
// Overwrite val to GMII if splitter core is enabled. The phymode here
// is the actual phy mode on phy hardware, but phy interface from
// EMAC core is GMII.
//
    if (dwmac.splitter_base)
    val = SYSMGR_EMACGRP_CTRL_PHYSEL_ENUM_GMII_MII;
// Assert reset to the enet controller before changing the phy mode
    reset_control_assert(dwmac.stmmac_ocp_rst);
    reset_control_assert(dwmac.stmmac_rst);
    regmap_read(sys_mgr_base_addr, reg_offset, &ctrl);
    ctrl &= ~(SYSMGR_EMACGRP_CTRL_PHYSEL_MASK);
    ctrl |= val;
    if (dwmac.f2h_ptp_ref_clk ||
    phymode == PHY_INTERFACE_MODE_MII ||
    phymode == PHY_INTERFACE_MODE_GMII ||
    phymode == PHY_INTERFACE_MODE_SGMII) {
    ctrl |= SYSMGR_GEN10_EMACGRP_CTRL_PTP_REF_CLK_MASK;
    regmap_read(sys_mgr_base_addr, SYSMGR_FPGAINTF_EMAC_REG,
    &module);
    module |= (SYSMGR_FPGAINTF_EMAC_BIT << reg_shift);
    regmap_write(sys_mgr_base_addr, SYSMGR_FPGAINTF_EMAC_REG,
    module);
    } else {
    ctrl &= ~SYSMGR_GEN10_EMACGRP_CTRL_PTP_REF_CLK_MASK;
    }
    regmap_write(sys_mgr_base_addr, reg_offset, ctrl);
// Deassert reset for the phy configuration to be sampled by
// the enet controller, and operation to start in requested mode
//
    reset_control_deassert(dwmac.stmmac_ocp_rst);
    reset_control_deassert(dwmac.stmmac_rst);
    if (phymode == PHY_INTERFACE_MODE_SGMII)
    socfpga_sgmii_config(dwmac, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_dwmac_pcs_init(priv: *mut stmmac_priv) -> c_int {
    static int socfpga_dwmac_pcs_init(struct stmmac_priv *priv)
    {
    struct socfpga_dwmac *dwmac = priv.plat.bsp_priv;
    struct regmap_config pcs_regmap_cfg = {
    .reg_bits = 16,
    .val_bits = 16,
    .reg_shift = REGMAP_UPSHIFT(1),
    };
    struct mdio_regmap_config mrc;
    struct regmap *pcs_regmap;
    struct phylink_pcs *pcs;
    struct mii_bus *pcs_bus;
    if (!dwmac.tse_pcs_base)
    return 0;
    pcs_regmap = devm_regmap_init_mmio(priv.device, dwmac.tse_pcs_base,
    &pcs_regmap_cfg);
    if (IS_ERR(pcs_regmap))
    return PTR_ERR(pcs_regmap);
    memset(&mrc, 0, sizeof(mrc));
    mrc.regmap = pcs_regmap;
    mrc.parent = priv.device;
    mrc.valid_addr = 0x0;
    mrc.autoscan = false;
// Can't use ndev->name here because it will not have been initialised,
// and in any case, the user can rename network interfaces at runtime.
//
    snprintf(mrc.name, MII_BUS_ID_SIZE, "%s-pcs-mii",
    dev_name(priv.device));
    pcs_bus = devm_mdio_regmap_register(priv.device, &mrc);
    if (IS_ERR(pcs_bus))
    return PTR_ERR(pcs_bus);
    pcs = lynx_pcs_create_mdiodev(pcs_bus, 0);
    if (IS_ERR(pcs))
    return PTR_ERR(pcs);
    priv.hw.phylink_pcs = pcs;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_dwmac_pcs_exit(priv: *mut stmmac_priv) {
    static void socfpga_dwmac_pcs_exit(struct stmmac_priv *priv)
    {
    if (priv.hw.phylink_pcs)
    lynx_pcs_destroy(priv.hw.phylink_pcs);
    }
    static struct phylink_pcs *socfpga_dwmac_select_pcs(struct stmmac_priv *priv,
    phy_interface_t interface)
    {
    return priv.hw.phylink_pcs;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_dwmac_init(dev: *mut device, bsp_priv: *mut c_void) -> c_int {
    static int socfpga_dwmac_init(struct device *dev, void *bsp_priv)
    {
    struct socfpga_dwmac *dwmac = bsp_priv;
    return dwmac.ops.set_phy_mode(dwmac, dev);
    }
#[no_mangle]
unsafe extern "C" fn socfpga_gen5_setup_plat_dat(dwmac: *mut socfpga_dwmac) {
    static void socfpga_gen5_setup_plat_dat(struct socfpga_dwmac *dwmac)
    {
    struct plat_stmmacenet_data *plat_dat = dwmac.plat_dat;
    plat_dat.core_type = DWMAC_CORE_GMAC;
// Rx watchdog timer in dwmac is buggy in this hw
    plat_dat.riwt_off = true;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_agilex5_setup_plat_dat(dwmac: *mut socfpga_dwmac) {
    static void socfpga_agilex5_setup_plat_dat(struct socfpga_dwmac *dwmac)
    {
    struct plat_stmmacenet_data *plat_dat = dwmac.plat_dat;
    plat_dat.core_type = DWMAC_CORE_XGMAC;
// Enable TSO
    plat_dat.flags |= STMMAC_FLAG_TSO_EN;
// Enable TBS
    switch (plat_dat.tx_queues_to_use) {
    case 8:
    plat_dat.tx_queues_cfg[7].tbs_en = true;
    fallthrough;
    case 7:
    plat_dat.tx_queues_cfg[6].tbs_en = true;
    break;
    default:
// Tx Queues 0 - 5 doesn't support TBS on Agilex5
    break;
    }
// Hw supported cross-timestamp
    plat_dat.int_snapshot_num = AUX_SNAPSHOT0;
    plat_dat.crosststamp = smtg_crosststamp;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int socfpga_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct device		*dev = &pdev.dev;
    int			ret;
    struct socfpga_dwmac	*dwmac;
    const struct socfpga_dwmac_ops *ops;
    ops = device_get_match_data(&pdev.dev);
    if (!ops) {
    dev_err(&pdev.dev, "no of match data provided\n");
    return -EINVAL;
    }
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    dwmac = devm_kzalloc(dev, sizeof(*dwmac), GFP_KERNEL);
    if (!dwmac)
    return -ENOMEM;
    dwmac.stmmac_ocp_rst = devm_reset_control_get_optional(dev, "stmmaceth-ocp");
    if (IS_ERR(dwmac.stmmac_ocp_rst)) {
    ret = PTR_ERR(dwmac.stmmac_ocp_rst);
    dev_err(dev, "error getting reset control of ocp %d\n", ret);
    return ret;
    }
    reset_control_deassert(dwmac.stmmac_ocp_rst);
    ret = socfpga_dwmac_parse_data(dwmac, dev);
    if (ret) {
    dev_err(dev, "Unable to parse OF data\n");
    return ret;
    }
// The socfpga driver needs to control the stmmac reset to set the phy
// mode. Create a copy of the core reset handle so it can be used by
// the driver later.
//
    dwmac.stmmac_rst = plat_dat.stmmac_rst;
    dwmac.ops = ops;
    dwmac.plat_dat = plat_dat;
    plat_dat.bsp_priv = dwmac;
    plat_dat.fix_mac_speed = socfpga_dwmac_fix_mac_speed;
    plat_dat.init = socfpga_dwmac_init;
    plat_dat.pcs_init = socfpga_dwmac_pcs_init;
    plat_dat.pcs_exit = socfpga_dwmac_pcs_exit;
    plat_dat.select_pcs = socfpga_dwmac_select_pcs;
    ops.setup_plat_dat(dwmac);
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct socfpga_dwmac_ops socfpga_gen5_ops = {
    .set_phy_mode = socfpga_gen5_set_phy_mode,
    .setup_plat_dat = socfpga_gen5_setup_plat_dat,
    };
    static const struct socfpga_dwmac_ops socfpga_gen10_ops = {
    .set_phy_mode = socfpga_gen10_set_phy_mode,
    .setup_plat_dat = socfpga_gen5_setup_plat_dat,
    };
    static const struct socfpga_dwmac_ops socfpga_agilex5_ops = {
    .set_phy_mode = socfpga_gen10_set_phy_mode,
    .setup_plat_dat = socfpga_agilex5_setup_plat_dat,
    };
    static const struct of_device_id socfpga_dwmac_match[] = {
    { .compatible = "altr,socfpga-stmmac", .data = &socfpga_gen5_ops },
    { .compatible = "altr,socfpga-stmmac-a10-s10", .data = &socfpga_gen10_ops },
    { .compatible = "altr,socfpga-stmmac-agilex5", .data = &socfpga_agilex5_ops },
    { }
    };
    MODULE_DEVICE_TABLE(of, socfpga_dwmac_match);
    static struct platform_driver socfpga_dwmac_driver = {
    .probe  = socfpga_dwmac_probe,
    .driver = {
    .name           = "socfpga-dwmac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = socfpga_dwmac_match,
    },
    };
    module_platform_driver(socfpga_dwmac_driver);
    MODULE_DESCRIPTION("Altera SOC DWMAC Specific Glue layer");
    MODULE_LICENSE("GPL v2");
