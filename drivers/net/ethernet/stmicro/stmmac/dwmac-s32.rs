//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-s32.c
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
// NXP S32G/R GMAC glue layer
//
// Copyright 2019-2026 NXP
//

// SoC PHY interface control register
pub const S32_PHY_INTF_SEL_MII: c_uint = 0x00;
pub const S32_PHY_INTF_SEL_SGMII: c_uint = 0x01;
pub const S32_PHY_INTF_SEL_RGMII: c_uint = 0x02;
pub const S32_PHY_INTF_SEL_RMII: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32_priv_data {
    pub ioaddr: *mut void __iomem,
    pub ctrl_sts: *mut void __iomem,
    pub sts_regmap: *mut regmap,
    pub sts_offset: c_uint,
    pub dev: *mut device,
    pub intf_mode: *mut phy_interface_t,
    pub tx_clk: *mut clk,
    pub rx_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn s32_gmac_write_phy_intf_select(gmac: *mut s32_priv_data) -> c_int {
    static int s32_gmac_write_phy_intf_select(struct s32_priv_data *gmac)
    {
    let mut ret: c_int = 0;
    if (gmac.ctrl_sts)
    writel(S32_PHY_INTF_SEL_RGMII, gmac.ctrl_sts);
    else
    ret = regmap_write(gmac.sts_regmap, gmac.sts_offset,
    S32_PHY_INTF_SEL_RGMII);
    dev_dbg(gmac.dev, "PHY mode set to %s\n", phy_modes(*gmac.intf_mode));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s32_gmac_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int s32_gmac_init(struct device *dev, void *priv)
    {
    struct s32_priv_data *gmac = priv;
    int ret;
// Set initial TX interface clock
    ret = clk_prepare_enable(gmac.tx_clk);
    if (ret) {
    dev_err(dev, "Can't enable tx clock\n");
    return ret;
    }
    ret = clk_set_rate(gmac.tx_clk, GMAC_INTF_RATE_125M);
    if (ret) {
    dev_err(dev, "Can't set tx clock\n");
    goto err_tx_disable;
    }
// Set initial RX interface clock
    ret = clk_prepare_enable(gmac.rx_clk);
    if (ret) {
    dev_err(dev, "Can't enable rx clock\n");
    goto err_tx_disable;
    }
    ret = clk_set_rate(gmac.rx_clk, GMAC_INTF_RATE_125M);
    if (ret) {
    dev_err(dev, "Can't set rx clock\n");
    goto err_txrx_disable;
    }
// Set interface mode
    ret = s32_gmac_write_phy_intf_select(gmac);
    if (ret) {
    dev_err(dev, "Can't set PHY interface mode\n");
    goto err_txrx_disable;
    }
    return 0;
    err_txrx_disable:
    clk_disable_unprepare(gmac.rx_clk);
    err_tx_disable:
    clk_disable_unprepare(gmac.tx_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s32_gmac_exit(dev: *mut device, priv: *mut c_void) {
    static void s32_gmac_exit(struct device *dev, void *priv)
    {
    struct s32_priv_data *gmac = priv;
    clk_disable_unprepare(gmac.tx_clk);
    clk_disable_unprepare(gmac.rx_clk);
    }
    static void s32_gmac_setup_multi_irq(struct device *dev,
    struct plat_stmmacenet_data *plat,
    struct stmmac_resources *res)
    {
    int i;
// RX IRQs
    for (i = 0; i < plat.rx_queues_to_use; i++) {
    if (res.rx_irq[i] <= 0) {
    dev_dbg(dev, "Missing RX queue %d interrupt\n", i);
    goto mac_irq_mode;
    }
    }
// TX IRQs
    for (i = 0; i < plat.tx_queues_to_use; i++) {
    if (res.tx_irq[i] <= 0) {
    dev_dbg(dev, "Missing TX queue %d interrupt\n", i);
    goto mac_irq_mode;
    }
    }
    plat.flags |= STMMAC_FLAG_MULTI_MSI_EN;
    dev_info(dev, "Multi-IRQ mode (per queue IRQs) selected\n");
    return;
    mac_irq_mode:
    plat.flags &= ~STMMAC_FLAG_MULTI_MSI_EN;
    dev_info(dev, "MAC IRQ mode selected\n");
    }
#[no_mangle]
unsafe extern "C" fn s32_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int s32_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat;
    struct device *dev = &pdev.dev;
    struct stmmac_resources res;
    struct s32_priv_data *gmac;
    int ret;
    gmac = devm_kzalloc(&pdev.dev, sizeof(*gmac), GFP_KERNEL);
    if (!gmac)
    return -ENOMEM;
    gmac.dev = &pdev.dev;
    ret = stmmac_get_platform_resources(pdev, &res);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to get platform resources\n");
    plat = devm_stmmac_probe_config_dt(pdev, res.mac);
    if (IS_ERR(plat))
    return dev_err_probe(dev, PTR_ERR(plat),
    "dt configuration failed\n");
// PHY interface mode control reg
    gmac.sts_regmap = syscon_regmap_lookup_by_phandle_args(dev.of_node,
    "nxp,phy-sel", 1, &gmac.sts_offset);
    if (gmac.sts_regmap == ERR_PTR(-EPROBE_DEFER))
    return PTR_ERR(gmac.sts_regmap);
    if (IS_ERR(gmac.sts_regmap)) {
    gmac.ctrl_sts = devm_platform_get_and_ioremap_resource(pdev, 1, core::ptr::null_mut());
    if (IS_ERR(gmac.ctrl_sts))
    return dev_err_probe(dev, PTR_ERR(gmac.ctrl_sts),
    "S32CC config region is missing\n");
    }
// tx clock
    gmac.tx_clk = devm_clk_get(&pdev.dev, "tx");
    if (IS_ERR(gmac.tx_clk))
    return dev_err_probe(dev, PTR_ERR(gmac.tx_clk),
    "tx clock not found\n");
// rx clock
    gmac.rx_clk = devm_clk_get(&pdev.dev, "rx");
    if (IS_ERR(gmac.rx_clk))
    return dev_err_probe(dev, PTR_ERR(gmac.rx_clk),
    "rx clock not found\n");
    gmac.intf_mode = &plat.phy_interface;
    gmac.ioaddr = res.addr;
// S32CC core feature set
    plat.core_type = DWMAC_CORE_GMAC4;
    plat.pmt = true;
    plat.flags |= STMMAC_FLAG_SPH_DISABLE;
    s32_gmac_setup_multi_irq(dev, plat, &res);
    plat.rx_fifo_size = 20480;
    plat.tx_fifo_size = 20480;
    plat.init = s32_gmac_init;
    plat.exit = s32_gmac_exit;
    plat.clk_tx_i = gmac.tx_clk;
    plat.set_clk_tx_rate = stmmac_set_clk_tx_rate;
    plat.bsp_priv = gmac;
    return stmmac_pltfr_probe(pdev, plat, &res);
    }
    static const struct of_device_id s32_dwmac_match[] = {
    { .compatible = "nxp,s32g2-dwmac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, s32_dwmac_match);
    static struct platform_driver s32_dwmac_driver = {
    .probe = s32_dwmac_probe,
    .remove = stmmac_pltfr_remove,
    .driver = {
    .name = "s32-dwmac",
    .pm = &stmmac_pltfr_pm_ops,
    .of_match_table = s32_dwmac_match,
    },
    };
    module_platform_driver(s32_dwmac_driver);
    MODULE_AUTHOR("Jan Petrous (OSS) <jan.petrous@oss.nxp.com>");
    MODULE_DESCRIPTION("NXP S32G/R common chassis GMAC driver");
    MODULE_LICENSE("GPL");
