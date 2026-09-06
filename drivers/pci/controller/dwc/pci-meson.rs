//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pci-meson.c
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
// PCIe host controller driver for Amlogic MESON SoCs
//
// Copyright (c) 2018 Amlogic, inc.
// Author: Yue Wang <yue.wang@amlogic.com>
//

// PCIe specific config registers
pub const PCIE_CFG0: c_uint = 0x0;

pub const PCIE_CFG_STATUS12: c_uint = 0x30;

pub const PCIE_CFG_STATUS17: c_uint = 0x44;

pub const MAX_PAYLOAD_SIZE: c_int = 256;
pub const MAX_READ_REQ_SIZE: c_int = 256;
pub const PCIE_RESET_DELAY: c_int = 500;
pub const PCIE_SHARED_RESET: c_int = 1;
pub const PCIE_NORMAL_RESET: c_int = 0;
    enum pcie_data_rate {
    PCIE_GEN1,
    PCIE_GEN2,
    PCIE_GEN3,
    PCIE_GEN4
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pcie_clk_res {
    pub clk: *mut clk,
    pub port_clk: *mut clk,
    pub general_clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pcie_rc_reset {
    pub port: *mut reset_control,
    pub apb: *mut reset_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pcie {
    pub pci: dw_pcie,
    pub cfg_base: *mut void __iomem,
    pub clk_res: meson_pcie_clk_res,
    pub mrst: meson_pcie_rc_reset,
    pub reset_gpio: *mut gpio_desc,
    pub phy: *mut phy,
}

    static struct reset_control *meson_pcie_get_reset(struct meson_pcie *mp,
    const char *id,
    u32 reset_type)
    {
    struct device *dev = mp.pci.dev;
    struct reset_control *reset;
    if (reset_type == PCIE_SHARED_RESET)
    reset = devm_reset_control_get_shared(dev, id);
    else
    reset = devm_reset_control_get(dev, id);
    return reset;
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_get_resets(mp: *mut meson_pcie) -> c_int {
    static int meson_pcie_get_resets(struct meson_pcie *mp)
    {
    struct meson_pcie_rc_reset *mrst = &mp.mrst;
    mrst.port = meson_pcie_get_reset(mp, "port", PCIE_NORMAL_RESET);
    if (IS_ERR(mrst.port))
    return PTR_ERR(mrst.port);
    reset_control_deassert(mrst.port);
    mrst.apb = meson_pcie_get_reset(mp, "apb", PCIE_SHARED_RESET);
    if (IS_ERR(mrst.apb))
    return PTR_ERR(mrst.apb);
    reset_control_deassert(mrst.apb);
    return 0;
    }
    static int meson_pcie_get_mems(struct platform_device *pdev,
    struct meson_pcie *mp)
    {
    struct dw_pcie *pci = &mp.pci;
    struct resource *res;
//
// For the broken DTs that supply 'dbi' as 'elbi', parse the 'elbi'
// region and assign it to both 'pci->elbi_base' and 'pci->dbi_space' so
// that the DWC core can skip parsing both regions.
//
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "elbi");
    if (res) {
    pci.elbi_base = devm_pci_remap_cfg_resource(pci.dev, res);
    if (IS_ERR(pci.elbi_base))
    return PTR_ERR(pci.elbi_base);
    pci.dbi_base = pci.elbi_base;
    pci.dbi_phys_addr = res.start;
    }
    mp.cfg_base = devm_platform_ioremap_resource_byname(pdev, "cfg");
    if (IS_ERR(mp.cfg_base))
    return PTR_ERR(mp.cfg_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_power_on(mp: *mut meson_pcie) -> c_int {
    static int meson_pcie_power_on(struct meson_pcie *mp)
    {
    let mut ret: c_int = 0;
    ret = phy_init(mp.phy);
    if (ret)
    return ret;
    ret = phy_power_on(mp.phy);
    if (ret) {
    phy_exit(mp.phy);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_power_off(mp: *mut meson_pcie) {
    static void meson_pcie_power_off(struct meson_pcie *mp)
    {
    phy_power_off(mp.phy);
    phy_exit(mp.phy);
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_reset(mp: *mut meson_pcie) -> c_int {
    static int meson_pcie_reset(struct meson_pcie *mp)
    {
    struct meson_pcie_rc_reset *mrst = &mp.mrst;
    let mut ret: c_int = 0;
    ret = phy_reset(mp.phy);
    if (ret)
    return ret;
    reset_control_assert(mrst.port);
    reset_control_assert(mrst.apb);
    udelay(PCIE_RESET_DELAY);
    reset_control_deassert(mrst.port);
    reset_control_deassert(mrst.apb);
    udelay(PCIE_RESET_DELAY);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn meson_pcie_disable_clock(data: *mut c_void) {
    static inline void meson_pcie_disable_clock(void *data)
    {
    struct clk *clk = data;
    clk_disable_unprepare(clk);
    }
    static inline struct clk *meson_pcie_probe_clock(struct device *dev,
    const char *id, u64 rate)
    {
    struct clk *clk;
    int ret;
    clk = devm_clk_get(dev, id);
    if (IS_ERR(clk))
    return clk;
    if (rate) {
    ret = clk_set_rate(clk, rate);
    if (ret) {
    dev_err(dev, "set clk rate failed, ret = %d\n", ret);
    return ERR_PTR(ret);
    }
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    dev_err(dev, "couldn't enable clk\n");
    return ERR_PTR(ret);
    }
    ret = devm_add_action_or_reset(dev, meson_pcie_disable_clock, clk);
    if (ret)
    return ERR_PTR(ret);
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_probe_clocks(mp: *mut meson_pcie) -> c_int {
    static int meson_pcie_probe_clocks(struct meson_pcie *mp)
    {
    struct device *dev = mp.pci.dev;
    struct meson_pcie_clk_res *res = &mp.clk_res;
    res.port_clk = meson_pcie_probe_clock(dev, "port", PORT_CLK_RATE);
    if (IS_ERR(res.port_clk))
    return PTR_ERR(res.port_clk);
    res.general_clk = meson_pcie_probe_clock(dev, "general", 0);
    if (IS_ERR(res.general_clk))
    return PTR_ERR(res.general_clk);
    res.clk = meson_pcie_probe_clock(dev, "pclk", 0);
    if (IS_ERR(res.clk))
    return PTR_ERR(res.clk);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn meson_cfg_readl(mp: *mut meson_pcie, reg: u32) -> u32 {
    static inline u32 meson_cfg_readl(struct meson_pcie *mp, u32 reg)
    {
    return readl(mp.cfg_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn meson_cfg_writel(mp: *mut meson_pcie, val: u32, reg: u32) {
    static inline void meson_cfg_writel(struct meson_pcie *mp, u32 val, u32 reg)
    {
    writel(val, mp.cfg_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_assert_reset(mp: *mut meson_pcie) {
    static void meson_pcie_assert_reset(struct meson_pcie *mp)
    {
    gpiod_set_value_cansleep(mp.reset_gpio, 1);
    udelay(500);
    gpiod_set_value_cansleep(mp.reset_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_ltssm_enable(mp: *mut meson_pcie) {
    static void meson_pcie_ltssm_enable(struct meson_pcie *mp)
    {
    u32 val;
    val = meson_cfg_readl(mp, PCIE_CFG0);
    val |= APP_LTSSM_ENABLE;
    meson_cfg_writel(mp, val, PCIE_CFG0);
    }
#[no_mangle]
unsafe extern "C" fn meson_size_to_payload(mp: *mut meson_pcie, size: c_int) -> c_int {
    static int meson_size_to_payload(struct meson_pcie *mp, int size)
    {
    struct device *dev = mp.pci.dev;
//
// dwc supports 2^(val+7) payload size, which val is 0~5 default to 1.
// So if input size is not 2^order alignment or less than 2^7 or bigger
// than 2^12, just set to default size 2^(1+7).
//
    if (!is_power_of_2(size) || size < 128 || size > 4096) {
    dev_warn(dev, "payload size %d, set to default 256\n", size);
    return 1;
    }
    return fls(size) - 8;
    }
#[no_mangle]
unsafe extern "C" fn meson_set_max_payload(mp: *mut meson_pcie, size: c_int) {
    static void meson_set_max_payload(struct meson_pcie *mp, int size)
    {
    struct dw_pcie *pci = &mp.pci;
    u32 val;
    let mut offset: u16 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    let mut max_payload_size: c_int = meson_size_to_payload(mp, size);
    val = dw_pcie_readl_dbi(pci, offset + PCI_EXP_DEVCTL);
    val &= ~PCI_EXP_DEVCTL_PAYLOAD;
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_DEVCTL, val);
    val = dw_pcie_readl_dbi(pci, offset + PCI_EXP_DEVCTL);
    val |= PCIE_CAP_MAX_PAYLOAD_SIZE(max_payload_size);
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_DEVCTL, val);
    }
#[no_mangle]
unsafe extern "C" fn meson_set_max_rd_req_size(mp: *mut meson_pcie, size: c_int) {
    static void meson_set_max_rd_req_size(struct meson_pcie *mp, int size)
    {
    struct dw_pcie *pci = &mp.pci;
    u32 val;
    let mut offset: u16 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    let mut max_rd_req_size: c_int = meson_size_to_payload(mp, size);
    val = dw_pcie_readl_dbi(pci, offset + PCI_EXP_DEVCTL);
    val &= ~PCI_EXP_DEVCTL_READRQ;
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_DEVCTL, val);
    val = dw_pcie_readl_dbi(pci, offset + PCI_EXP_DEVCTL);
    val |= PCIE_CAP_MAX_READ_REQ_SIZE(max_rd_req_size);
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_DEVCTL, val);
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int meson_pcie_start_link(struct dw_pcie *pci)
    {
    struct meson_pcie *mp = to_meson_pcie(pci);
    meson_pcie_ltssm_enable(mp);
    meson_pcie_assert_reset(mp);
    return 0;
    }
    static int meson_pcie_rd_own_conf(struct pci_bus *bus, u32 devfn,
    int where, int size, u32 *val)
    {
    int ret;
    ret = pci_generic_config_read(bus, devfn, where, size, val);
    if (ret != PCIBIOS_SUCCESSFUL)
    return ret;
//
// There is a bug in the MESON AXG PCIe controller whereby software
// cannot program the PCI_CLASS_DEVICE register, so we must fabricate
// the return value in the config accessors.
//
    if ((where & ~3) == PCI_CLASS_REVISION) {
    if (size <= 2)
// val = (*val & ((1 << (size * 8)) - 1)) << (8 * (where & 3));
// val &= ~0xffffff00;
// val |= PCI_CLASS_BRIDGE_PCI_NORMAL << 8;
    if (size <= 2)
// val = (*val >> (8 * (where & 3))) & ((1 << (size * 8)) - 1);
    }
    return PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops meson_pci_ops = {
    .map_bus = dw_pcie_own_conf_map_bus,
    .read = meson_pcie_rd_own_conf,
    .write = pci_generic_config_write,
    };
#[no_mangle]
unsafe extern "C" fn meson_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool meson_pcie_link_up(struct dw_pcie *pci)
    {
    struct meson_pcie *mp = to_meson_pcie(pci);
    u32 state12;
    state12 = meson_cfg_readl(mp, PCIE_CFG_STATUS12);
    return IS_SMLH_LINK_UP(state12) && IS_RDLH_LINK_UP(state12);
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int meson_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct meson_pcie *mp = to_meson_pcie(pci);
    pp.bridge.ops = &meson_pci_ops;
    meson_set_max_payload(mp, MAX_PAYLOAD_SIZE);
    meson_set_max_rd_req_size(mp, MAX_READ_REQ_SIZE);
    return 0;
    }
    static const struct dw_pcie_host_ops meson_pcie_host_ops = {
    .init = meson_pcie_host_init,
    };
    static const struct dw_pcie_ops dw_pcie_ops = {
    .link_up = meson_pcie_link_up,
    .start_link = meson_pcie_start_link,
    };
#[no_mangle]
unsafe extern "C" fn meson_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int meson_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dw_pcie *pci;
    struct meson_pcie *mp;
    int ret;
    mp = devm_kzalloc(dev, sizeof(*mp), GFP_KERNEL);
    if (!mp)
    return -ENOMEM;
    pci = &mp.pci;
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
    pci.pp.ops = &meson_pcie_host_ops;
    pci.num_lanes = 1;
    mp.phy = devm_phy_get(dev, "pcie");
    if (IS_ERR(mp.phy)) {
    dev_err(dev, "get phy failed, %pe\n", mp.phy);
    return PTR_ERR(mp.phy);
    }
    mp.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(mp.reset_gpio)) {
    dev_err(dev, "get reset gpio failed\n");
    return PTR_ERR(mp.reset_gpio);
    }
    ret = meson_pcie_get_resets(mp);
    if (ret) {
    dev_err(dev, "get reset resource failed, %d\n", ret);
    return ret;
    }
    ret = meson_pcie_get_mems(pdev, mp);
    if (ret) {
    dev_err(dev, "get memory resource failed, %d\n", ret);
    return ret;
    }
    ret = meson_pcie_power_on(mp);
    if (ret) {
    dev_err(dev, "phy power on failed, %d\n", ret);
    return ret;
    }
    ret = meson_pcie_reset(mp);
    if (ret) {
    dev_err(dev, "reset failed, %d\n", ret);
    goto err_phy;
    }
    ret = meson_pcie_probe_clocks(mp);
    if (ret) {
    dev_err(dev, "init clock resources failed, %d\n", ret);
    goto err_phy;
    }
    platform_set_drvdata(pdev, mp);
    ret = dw_pcie_host_init(&pci.pp);
    if (ret < 0) {
    dev_err(dev, "Add PCIe port failed, %d\n", ret);
    goto err_phy;
    }
    return 0;
    err_phy:
    meson_pcie_power_off(mp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_pcie_remove(pdev: *mut platform_device) {
    static void meson_pcie_remove(struct platform_device *pdev)
    {
    struct meson_pcie *mp = platform_get_drvdata(pdev);
    dw_pcie_host_deinit(&mp.pci.pp);
    meson_pcie_power_off(mp);
    }
    static const struct of_device_id meson_pcie_of_match[] = {
    {
    .compatible = "amlogic,axg-pcie",
    },
    {
    .compatible = "amlogic,g12a-pcie",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, meson_pcie_of_match);
    static struct platform_driver meson_pcie_driver = {
    .probe = meson_pcie_probe,
    .remove = meson_pcie_remove,
    .driver = {
    .name = "meson-pcie",
    .of_match_table = meson_pcie_of_match,
    },
    };
    module_platform_driver(meson_pcie_driver);
    MODULE_AUTHOR("Yue Wang <yue.wang@amlogic.com>");
    MODULE_DESCRIPTION("Amlogic PCIe Controller driver");
    MODULE_LICENSE("GPL v2");
