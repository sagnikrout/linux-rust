//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-imx.c
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
//
// dwmac-imx.c - DWMAC Specific Glue layer for NXP imx8
//
// Copyright 2020 NXP
//

pub const DMA_BUS_MODE: c_uint = 0x00001000;

    struct imx_priv_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dwmac_ops {
    pub flags: u32,
    pub addr_width: u8,
    pub mac_rgmii_txclk_auto_adj: bool,
    pub priv): *mut *mut int (fix_soc_reset)(struct stmmac_priv,
    pub phy_intf_sel): *mut *mut *mut int (set_intf_mode)(struct imx_priv_data dwmac, u8,
    void (*fix_mac_speed)(void *priv, phy_interface_t interface,
    pub mode): int speed, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_priv_data {
    pub dev: *mut device,
    pub clk_tx: *mut clk,
    pub clk_mem: *mut clk,
    pub intf_regmap: *mut regmap,
    pub intf_reg_off: u32,
    pub rmii_refclk_ext: bool,
    pub base_addr: *mut void __iomem,
    pub ops: *const imx_dwmac_ops,
    pub plat_dat: *mut plat_stmmacenet_data,
}

#[no_mangle]
unsafe extern "C" fn imx8mp_set_intf_mode(dwmac: *mut imx_priv_data, phy_intf_sel: u8) -> c_int {
    static int imx8mp_set_intf_mode(struct imx_priv_data *dwmac, u8 phy_intf_sel)
    {
    unsigned int val;
    val = FIELD_PREP(GPR_ENET_QOS_INTF_SEL_MASK, phy_intf_sel) |
    GPR_ENET_QOS_CLK_GEN_EN;
    if (phy_intf_sel == PHY_INTF_SEL_RMII && !dwmac.rmii_refclk_ext)
    val |= GPR_ENET_QOS_CLK_TX_CLK_SEL;
#[no_mangle]
pub unsafe extern "C" fn if(PHY_INTF_SEL_RGMII: phy_intf_sel ==) -> else {
    else if (phy_intf_sel == PHY_INTF_SEL_RGMII)
    val |= GPR_ENET_QOS_RGMII_EN;
    return regmap_update_bits(dwmac.intf_regmap, dwmac.intf_reg_off,
    GPR_ENET_QOS_INTF_MODE_MASK, val);
    };
    static int
    imx8dxl_set_intf_mode(struct imx_priv_data *dwmac, u8 phy_intf_sel)
    {
// TBD: depends on imx8dxl scu interfaces to be upstreamed
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_set_intf_mode(dwmac: *mut imx_priv_data, phy_intf_sel: u8) -> c_int {
    static int imx93_set_intf_mode(struct imx_priv_data *dwmac, u8 phy_intf_sel)
    {
    unsigned int val;
    int ret;
    if (phy_intf_sel == PHY_INTF_SEL_RMII && dwmac.rmii_refclk_ext) {
    ret = regmap_clear_bits(dwmac.intf_regmap,
    dwmac.intf_reg_off +
    MX93_ENET_CLK_SEL_OFFSET,
    MX93_ENET_QOS_CLK_TX_SEL_MASK);
    if (ret)
    return ret;
    }
    val = FIELD_PREP(MX93_GPR_ENET_QOS_INTF_SEL_MASK, phy_intf_sel) |
    MX93_GPR_ENET_QOS_ENABLE;
    return regmap_update_bits(dwmac.intf_regmap, dwmac.intf_reg_off,
    MX93_GPR_ENET_QOS_INTF_SEL_MASK |
    MX93_GPR_ENET_QOS_ENABLE, val);
    };
#[no_mangle]
unsafe extern "C" fn imx_dwmac_clks_config(priv: *mut c_void, enabled: bool) -> c_int {
    static int imx_dwmac_clks_config(void *priv, bool enabled)
    {
    struct imx_priv_data *dwmac = priv;
    let mut ret: c_int = 0;
    if (enabled) {
    ret = clk_prepare_enable(dwmac.clk_mem);
    if (ret) {
    dev_err(dwmac.dev, "mem clock enable failed\n");
    return ret;
    }
    ret = clk_prepare_enable(dwmac.clk_tx);
    if (ret) {
    dev_err(dwmac.dev, "tx clock enable failed\n");
    clk_disable_unprepare(dwmac.clk_mem);
    return ret;
    }
    } else {
    clk_disable_unprepare(dwmac.clk_tx);
    clk_disable_unprepare(dwmac.clk_mem);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_set_phy_intf_sel(bsp_priv: *mut c_void, phy_intf_sel: u8) -> c_int {
    static int imx_set_phy_intf_sel(void *bsp_priv, u8 phy_intf_sel)
    {
    struct imx_priv_data *dwmac = bsp_priv;
    if (!dwmac.ops.set_intf_mode)
    return 0;
    if (phy_intf_sel != PHY_INTF_SEL_GMII_MII &&
    phy_intf_sel != PHY_INTF_SEL_RGMII &&
    phy_intf_sel != PHY_INTF_SEL_RMII)
    return -EINVAL;
    return dwmac.ops.set_intf_mode(dwmac, phy_intf_sel);
    }
    static int imx_dwmac_set_clk_tx_rate(void *bsp_priv, struct clk *clk_tx_i,
    phy_interface_t interface, int speed)
    {
    if (interface == PHY_INTERFACE_MODE_RMII ||
    interface == PHY_INTERFACE_MODE_MII)
    return 0;
    return stmmac_set_clk_tx_rate(bsp_priv, clk_tx_i, interface, speed);
    }
    static void imx_dwmac_fix_speed(void *priv, phy_interface_t interface,
    int speed, unsigned int mode)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct imx_priv_data *dwmac = priv;
    long rate;
    int err;
    plat_dat = dwmac.plat_dat;
    if (dwmac.ops.mac_rgmii_txclk_auto_adj ||
    (plat_dat.phy_interface == PHY_INTERFACE_MODE_RMII) ||
    (plat_dat.phy_interface == PHY_INTERFACE_MODE_MII))
    return;
    rate = rgmii_clock(speed);
    if (rate < 0) {
    dev_err(dwmac.dev, "invalid speed %d\n", speed);
    return;
    }
    err = clk_set_rate(dwmac.clk_tx, rate);
    if (err < 0)
    dev_err(dwmac.dev, "failed to set tx rate %lu\n", rate);
    }
    static void imx93_dwmac_fix_speed(void *priv, phy_interface_t interface,
    int speed, unsigned int mode)
    {
    struct imx_priv_data *dwmac = priv;
    unsigned int iface;
    int ctrl, old_ctrl;
    imx_dwmac_fix_speed(priv, interface, speed, mode);
    if (!dwmac || mode != MLO_AN_FIXED)
    return;
    if (regmap_read(dwmac.intf_regmap, dwmac.intf_reg_off, &iface))
    return;
    if (FIELD_GET(MX93_GPR_ENET_QOS_INTF_SEL_MASK, iface) !=
    PHY_INTF_SEL_RGMII)
    return;
    old_ctrl = readl(dwmac.base_addr + MAC_CTRL_REG);
    ctrl = old_ctrl & ~CTRL_SPEED_MASK;
    regmap_update_bits(dwmac.intf_regmap, dwmac.intf_reg_off,
    MX93_GPR_ENET_QOS_INTF_SEL_MASK |
    MX93_GPR_ENET_QOS_ENABLE, 0);
    writel(ctrl, dwmac.base_addr + MAC_CTRL_REG);
// Ensure the settings for CTRL are applied.
    readl(dwmac.base_addr + MAC_CTRL_REG);
    usleep_range(10, 20);
    iface &= MX93_GPR_ENET_QOS_INTF_SEL_MASK;
    iface |= MX93_GPR_ENET_QOS_ENABLE;
    regmap_update_bits(dwmac.intf_regmap, dwmac.intf_reg_off,
    MX93_GPR_ENET_QOS_INTF_SEL_MASK |
    MX93_GPR_ENET_QOS_ENABLE, iface);
    writel(old_ctrl, dwmac.base_addr + MAC_CTRL_REG);
    }
#[no_mangle]
unsafe extern "C" fn imx_dwmac_mx93_reset(priv: *mut stmmac_priv) -> c_int {
    static int imx_dwmac_mx93_reset(struct stmmac_priv *priv)
    {
    struct plat_stmmacenet_data *plat_dat = priv.plat;
    void __iomem *ioaddr = priv.ioaddr;
    u32 value;
// DMA SW reset
    value = readl(ioaddr + DMA_BUS_MODE);
    value |= DMA_BUS_MODE_SFT_RESET;
    writel(value, ioaddr + DMA_BUS_MODE);
    if (plat_dat.phy_interface == PHY_INTERFACE_MODE_RMII) {
    usleep_range(100, 200);
    writel(RMII_RESET_SPEED, ioaddr + MAC_CTRL_REG);
    }
    return readl_poll_timeout(ioaddr + DMA_BUS_MODE, value,
    !(value & DMA_BUS_MODE_SFT_RESET),
    10000, 1000000);
    }
    static int
    imx_dwmac_parse_dt(struct imx_priv_data *dwmac, struct device *dev)
    {
    struct device_node *np = dev.of_node;
    let mut err: c_int = 0;
    dwmac.rmii_refclk_ext = of_property_read_bool(np, "snps,rmii_refclk_ext");
    dwmac.clk_tx = devm_clk_get(dev, "tx");
    if (IS_ERR(dwmac.clk_tx)) {
    dev_err(dev, "failed to get tx clock\n");
    return PTR_ERR(dwmac.clk_tx);
    }
    dwmac.clk_mem = core::ptr::null_mut();
    if (of_machine_is_compatible("fsl,imx8dxl") ||
    of_machine_is_compatible("fsl,imx91") ||
    of_machine_is_compatible("fsl,imx93")) {
    dwmac.clk_mem = devm_clk_get(dev, "mem");
    if (IS_ERR(dwmac.clk_mem)) {
    dev_err(dev, "failed to get mem clock\n");
    return PTR_ERR(dwmac.clk_mem);
    }
    }
    if (of_machine_is_compatible("fsl,imx8mp") ||
    of_machine_is_compatible("fsl,imx91") ||
    of_machine_is_compatible("fsl,imx93")) {
// Binding doc describes the property:
// is required by i.MX8MP, i.MX91, i.MX93.
// is optional for i.MX8DXL.
//
    dwmac.intf_regmap =
    syscon_regmap_lookup_by_phandle_args(np, "intf_mode", 1,
    &dwmac.intf_reg_off);
    if (IS_ERR(dwmac.intf_regmap))
    return PTR_ERR(dwmac.intf_regmap);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn imx_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int imx_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct imx_priv_data *dwmac;
    const struct imx_dwmac_ops *data;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    dwmac = devm_kzalloc(&pdev.dev, sizeof(*dwmac), GFP_KERNEL);
    if (!dwmac)
    return -ENOMEM;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    data = of_device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "failed to get match data\n");
    return -EINVAL;
    }
    dwmac.ops = data;
    dwmac.dev = &pdev.dev;
    ret = imx_dwmac_parse_dt(dwmac, &pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "failed to parse OF data\n");
    return ret;
    }
    plat_dat.flags |= data.flags;
// Default TX Q0 to use TSO and rest TXQ for TBS
    for (int i = 1; i < plat_dat.tx_queues_to_use; i++)
    plat_dat.tx_queues_cfg[i].tbs_en = 1;
    plat_dat.host_dma_width = dwmac.ops.addr_width;
    plat_dat.set_phy_intf_sel = imx_set_phy_intf_sel;
    plat_dat.clks_config = imx_dwmac_clks_config;
    plat_dat.bsp_priv = dwmac;
    dwmac.plat_dat = plat_dat;
    dwmac.base_addr = stmmac_res.addr;
    ret = imx_dwmac_clks_config(dwmac, true);
    if (ret)
    return ret;
    if (dwmac.ops.fix_mac_speed) {
    plat_dat.fix_mac_speed = dwmac.ops.fix_mac_speed;
    } else if (!dwmac.ops.mac_rgmii_txclk_auto_adj) {
    plat_dat.clk_tx_i = dwmac.clk_tx;
    plat_dat.set_clk_tx_rate = imx_dwmac_set_clk_tx_rate;
    }
    dwmac.plat_dat.fix_soc_reset = dwmac.ops.fix_soc_reset;
    ret = stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    if (ret)
    imx_dwmac_clks_config(dwmac, false);
    return ret;
    }
    static struct imx_dwmac_ops imx8mp_dwmac_data = {
    .addr_width = 34,
    .mac_rgmii_txclk_auto_adj = false,
    .set_intf_mode = imx8mp_set_intf_mode,
    .flags = STMMAC_FLAG_EEE_DISABLE |
    STMMAC_FLAG_HWTSTAMP_CORRECT_LATENCY |
    STMMAC_FLAG_KEEP_PREAMBLE_BEFORE_SFD,
    };
    static struct imx_dwmac_ops imx8dxl_dwmac_data = {
    .addr_width = 32,
    .mac_rgmii_txclk_auto_adj = true,
    .set_intf_mode = imx8dxl_set_intf_mode,
    };
    static struct imx_dwmac_ops imx93_dwmac_data = {
    .addr_width = 32,
    .mac_rgmii_txclk_auto_adj = true,
    .set_intf_mode = imx93_set_intf_mode,
    .fix_soc_reset = imx_dwmac_mx93_reset,
    .fix_mac_speed = imx93_dwmac_fix_speed,
    };
    static const struct of_device_id imx_dwmac_match[] = {
    { .compatible = "nxp,imx8mp-dwmac-eqos", .data = &imx8mp_dwmac_data },
    { .compatible = "nxp,imx8dxl-dwmac-eqos", .data = &imx8dxl_dwmac_data },
    { .compatible = "nxp,imx93-dwmac-eqos", .data = &imx93_dwmac_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, imx_dwmac_match);
    static struct platform_driver imx_dwmac_driver = {
    .probe  = imx_dwmac_probe,
    .remove = stmmac_pltfr_remove,
    .driver = {
    .name           = "imx-dwmac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = imx_dwmac_match,
    },
    };
    module_platform_driver(imx_dwmac_driver);
    MODULE_AUTHOR("NXP");
    MODULE_DESCRIPTION("NXP imx8 DWMAC Specific Glue layer");
    MODULE_LICENSE("GPL v2");
