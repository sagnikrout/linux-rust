//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-tegra.c
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

    static const char *const mgbe_clks[] = {
    "rx-pcs", "tx", "tx-pcs", "mac-divider", "mac", "mgbe", "ptp_ref", "mac"
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mgbe {
    pub dev: *mut device,
    pub clks: *mut clk_bulk_data,
    pub rst_mac: *mut reset_control,
    pub rst_pcs: *mut reset_control,
    pub iommu_sid: u32,
    pub hv: *mut void __iomem,
    pub regs: *mut void __iomem,
    pub xpcs: *mut void __iomem,
    pub mii: *mut mii_bus,
}

pub const XPCS_WRAP_UPHY_RX_CONTROL: c_uint = 0x801c;

pub const XPCS_WRAP_UPHY_HW_INIT_CTRL: c_uint = 0x8020;

pub const XPCS_WRAP_UPHY_STATUS: c_uint = 0x8044;

pub const XPCS_WRAP_IRQ_STATUS: c_uint = 0x8050;

pub const XPCS_REG_ADDR_SHIFT: c_int = 10;
pub const XPCS_REG_ADDR_MASK: c_uint = 0x1fff;
pub const XPCS_ADDR: c_uint = 0x3fc;
pub const MGBE_WRAP_COMMON_INTR_ENABLE: c_uint = 0x8704;

pub const MGBE_WRAP_AXI_ASID0_CTRL: c_uint = 0x8400;
#[no_mangle]
unsafe extern "C" fn tegra_mgbe_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_mgbe_suspend(struct device *dev)
    {
    struct tegra_mgbe *mgbe = get_stmmac_bsp_priv(dev);
    int err;
    err = stmmac_suspend(dev);
    if (err)
    return err;
    clk_bulk_disable_unprepare(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    return reset_control_assert(mgbe.rst_mac);
    }
#[no_mangle]
unsafe extern "C" fn tegra_mgbe_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_mgbe_resume(struct device *dev)
    {
    struct tegra_mgbe *mgbe = get_stmmac_bsp_priv(dev);
    u32 value;
    int err;
    err = clk_bulk_prepare_enable(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    if (err < 0)
    return err;
    err = reset_control_deassert(mgbe.rst_mac);
    if (err < 0)
    return err;
// Enable common interrupt at wrapper level
    writel(MAC_SBD_INTR, mgbe.regs + MGBE_WRAP_COMMON_INTR_ENABLE);
// Program SID
    writel(mgbe.iommu_sid, mgbe.hv + MGBE_WRAP_AXI_ASID0_CTRL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_STATUS);
    if ((value & XPCS_WRAP_UPHY_STATUS_TX_P_UP) == 0) {
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_HW_INIT_CTRL);
    value |= XPCS_WRAP_UPHY_HW_INIT_CTRL_TX_EN;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_HW_INIT_CTRL);
    }
    err = readl_poll_timeout(mgbe.xpcs + XPCS_WRAP_UPHY_HW_INIT_CTRL, value,
    (value & XPCS_WRAP_UPHY_HW_INIT_CTRL_TX_EN) == 0,
    500, 500 * 2000);
    if (err < 0) {
    dev_err(mgbe.dev, "timeout waiting for TX lane to become enabled\n");
    clk_bulk_disable_unprepare(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    return err;
    }
    err = stmmac_resume(dev);
    if (err < 0)
    clk_bulk_disable_unprepare(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mgbe_uphy_lane_bringup_serdes_up(ndev: *mut net_device, mgbe_data: *mut c_void) -> c_int {
    static int mgbe_uphy_lane_bringup_serdes_up(struct net_device *ndev, void *mgbe_data)
    {
    struct tegra_mgbe *mgbe = (struct tegra_mgbe *)mgbe_data;
    u32 value;
    int err;
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_SW_OVRD;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value &= ~XPCS_WRAP_UPHY_RX_CONTROL_RX_IDDQ;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value &= ~XPCS_WRAP_UPHY_RX_CONTROL_AUX_RX_IDDQ;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    usleep_range(10, 20);  /* 50ns min delay needed as per HW design */
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value &= ~XPCS_WRAP_UPHY_RX_CONTROL_RX_SLEEP;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    usleep_range(10, 20);  /* 500ns min delay needed as per HW design */
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_CAL_EN;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    err = readl_poll_timeout(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL, value,
    (value & XPCS_WRAP_UPHY_RX_CONTROL_RX_CAL_EN) == 0,
    1000, 1000 * 2000);
    if (err < 0) {
    dev_err(mgbe.dev, "timeout waiting for RX calibration to become enabled\n");
    return err;
    }
    usleep_range(10, 20);  /* 50ns min delay needed as per HW design */
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_DATA_EN;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value &= ~XPCS_WRAP_UPHY_RX_CONTROL_RX_PCS_PHY_RDY;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    usleep_range(10, 20);  /* 50ns min delay needed as per HW design */
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_CDR_RESET;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    usleep_range(10, 20);  /* 50ns min delay needed as per HW design */
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_PCS_PHY_RDY;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    msleep(30);  /* 30ms delay needed as per HW design */
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value &= ~XPCS_WRAP_UPHY_RX_CONTROL_RX_CDR_RESET;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    err = readl_poll_timeout(mgbe.xpcs + XPCS_WRAP_IRQ_STATUS, value,
    value & XPCS_WRAP_IRQ_STATUS_PCS_LINK_STS,
    500, 500 * 2000);
    if (err < 0) {
    dev_err(mgbe.dev, "timeout waiting for link to become ready\n");
    return err;
    }
// clear status
    writel(value, mgbe.xpcs + XPCS_WRAP_IRQ_STATUS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mgbe_uphy_lane_bringup_serdes_down(ndev: *mut net_device, mgbe_data: *mut c_void) {
    static void mgbe_uphy_lane_bringup_serdes_down(struct net_device *ndev, void *mgbe_data)
    {
    struct tegra_mgbe *mgbe = (struct tegra_mgbe *)mgbe_data;
    u32 value;
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_SW_OVRD;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value &= ~XPCS_WRAP_UPHY_RX_CONTROL_RX_DATA_EN;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_SLEEP;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_AUX_RX_IDDQ;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    value |= XPCS_WRAP_UPHY_RX_CONTROL_RX_IDDQ;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_RX_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn tegra_mgbe_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_mgbe_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat;
    struct stmmac_resources res;
    let mut use_legacy_ptp: bool = false;
    struct tegra_mgbe *mgbe;
    int irq, err, i;
    u32 value;
    mgbe = devm_kzalloc(&pdev.dev, sizeof(*mgbe), GFP_KERNEL);
    if (!mgbe)
    return -ENOMEM;
    mgbe.dev = &pdev.dev;
    memset(&res, 0, sizeof(res));
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    mgbe.hv = devm_platform_ioremap_resource_byname(pdev, "hypervisor");
    if (IS_ERR(mgbe.hv))
    return PTR_ERR(mgbe.hv);
    mgbe.regs = devm_platform_ioremap_resource_byname(pdev, "mac");
    if (IS_ERR(mgbe.regs))
    return PTR_ERR(mgbe.regs);
    mgbe.xpcs = devm_platform_ioremap_resource_byname(pdev, "xpcs");
    if (IS_ERR(mgbe.xpcs))
    return PTR_ERR(mgbe.xpcs);
// get controller's stream id from iommu property in device tree
    if (!tegra_dev_iommu_get_stream_id(mgbe.dev, &mgbe.iommu_sid)) {
    dev_err(mgbe.dev, "failed to get iommu stream id\n");
    return -EINVAL;
    }
    res.addr = mgbe.regs;
    res.irq = irq;
    mgbe.clks = devm_kcalloc(&pdev.dev, ARRAY_SIZE(mgbe_clks),
    sizeof(*mgbe.clks), GFP_KERNEL);
    if (!mgbe.clks)
    return -ENOMEM;
// Older device-trees use 'ptp-ref' rather than 'ptp_ref'.
// Fall back when the legacy name is present.
//
    if (of_property_match_string(pdev.dev.of_node, "clock-names",
    "ptp-ref") >= 0)
    use_legacy_ptp = true;
    for (i = 0; i < ARRAY_SIZE(mgbe_clks); i++) {
    mgbe.clks[i].id = mgbe_clks[i];
    if (use_legacy_ptp && !strcmp(mgbe_clks[i], "ptp_ref")) {
    dev_warn(mgbe.dev,
    "Device-tree update needed for PTP clock!\n");
    mgbe.clks[i].id = "ptp-ref";
    }
    }
    err = devm_clk_bulk_get(mgbe.dev, ARRAY_SIZE(mgbe_clks), mgbe.clks);
    if (err < 0)
    return err;
    err = clk_bulk_prepare_enable(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    if (err < 0)
    return err;
// Perform MAC reset
    mgbe.rst_mac = devm_reset_control_get(&pdev.dev, "mac");
    if (IS_ERR(mgbe.rst_mac)) {
    err = PTR_ERR(mgbe.rst_mac);
    goto disable_clks;
    }
    err = reset_control_assert(mgbe.rst_mac);
    if (err < 0)
    goto disable_clks;
    usleep_range(2000, 4000);
    err = reset_control_deassert(mgbe.rst_mac);
    if (err < 0)
    goto disable_clks;
// Perform PCS reset
    mgbe.rst_pcs = devm_reset_control_get(&pdev.dev, "pcs");
    if (IS_ERR(mgbe.rst_pcs)) {
    err = PTR_ERR(mgbe.rst_pcs);
    goto disable_clks;
    }
    err = reset_control_assert(mgbe.rst_pcs);
    if (err < 0)
    goto disable_clks;
    usleep_range(2000, 4000);
    err = reset_control_deassert(mgbe.rst_pcs);
    if (err < 0)
    goto disable_clks;
    plat = devm_stmmac_probe_config_dt(pdev, res.mac);
    if (IS_ERR(plat)) {
    err = PTR_ERR(plat);
    goto disable_clks;
    }
    plat.core_type = DWMAC_CORE_XGMAC;
    plat.flags |= STMMAC_FLAG_TSO_EN;
    plat.pmt = true;
    plat.bsp_priv = mgbe;
    if (!plat.mdio_node)
    plat.mdio_node = of_get_child_by_name(pdev.dev.of_node, "mdio");
    if (!plat.mdio_bus_data) {
    plat.mdio_bus_data = devm_kzalloc(&pdev.dev, sizeof(*plat.mdio_bus_data),
    GFP_KERNEL);
    if (!plat.mdio_bus_data) {
    err = -ENOMEM;
    goto disable_clks;
    }
    }
    plat.mdio_bus_data.needs_reset = true;
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_STATUS);
    if ((value & XPCS_WRAP_UPHY_STATUS_TX_P_UP) == 0) {
    value = readl(mgbe.xpcs + XPCS_WRAP_UPHY_HW_INIT_CTRL);
    value |= XPCS_WRAP_UPHY_HW_INIT_CTRL_TX_EN;
    writel(value, mgbe.xpcs + XPCS_WRAP_UPHY_HW_INIT_CTRL);
    }
    err = readl_poll_timeout(mgbe.xpcs + XPCS_WRAP_UPHY_HW_INIT_CTRL, value,
    (value & XPCS_WRAP_UPHY_HW_INIT_CTRL_TX_EN) == 0,
    500, 500 * 2000);
    if (err < 0) {
    dev_err(mgbe.dev, "timeout waiting for TX lane to become enabled\n");
    goto disable_clks;
    }
    plat.serdes_powerup = mgbe_uphy_lane_bringup_serdes_up;
    plat.serdes_powerdown = mgbe_uphy_lane_bringup_serdes_down;
// Tx FIFO Size - 128KB
    plat.tx_fifo_size = 131072;
// Rx FIFO Size - 192KB
    plat.rx_fifo_size = 196608;
// Enable common interrupt at wrapper level
    writel(MAC_SBD_INTR, mgbe.regs + MGBE_WRAP_COMMON_INTR_ENABLE);
// Program SID
    writel(mgbe.iommu_sid, mgbe.hv + MGBE_WRAP_AXI_ASID0_CTRL);
    plat.flags |= STMMAC_FLAG_SERDES_UP_AFTER_PHY_LINKUP;
    err = stmmac_dvr_probe(&pdev.dev, plat, &res);
    if (err < 0)
    goto disable_clks;
    return 0;
    disable_clks:
    clk_bulk_disable_unprepare(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_mgbe_remove(pdev: *mut platform_device) {
    static void tegra_mgbe_remove(struct platform_device *pdev)
    {
    struct tegra_mgbe *mgbe = get_stmmac_bsp_priv(&pdev.dev);
    clk_bulk_disable_unprepare(ARRAY_SIZE(mgbe_clks), mgbe.clks);
    stmmac_pltfr_remove(pdev);
    }
    static const struct of_device_id tegra_mgbe_match[] = {
    { .compatible = "nvidia,tegra234-mgbe", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tegra_mgbe_match);
    static SIMPLE_DEV_PM_OPS(tegra_mgbe_pm_ops, tegra_mgbe_suspend, tegra_mgbe_resume);
    static struct platform_driver tegra_mgbe_driver = {
    .probe = tegra_mgbe_probe,
    .remove = tegra_mgbe_remove,
    .driver = {
    .name = "tegra-mgbe",
    .pm		= &tegra_mgbe_pm_ops,
    .of_match_table = tegra_mgbe_match,
    },
    };
    module_platform_driver(tegra_mgbe_driver);
    MODULE_AUTHOR("Thierry Reding <treding@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra MGBE driver");
    MODULE_LICENSE("GPL");
