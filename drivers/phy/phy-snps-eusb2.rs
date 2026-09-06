//! Automatically rewritten from C to Rust
//! Source: drivers/phy/phy-snps-eusb2.c
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
// Copyright (c) 2023, Linaro Limited
//

    static const char * const eusb2_hsphy_vreg_names[] = {
    "vdd", "vdda12",
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snps_eusb2_phy_drvdata {
    pub p): *mut *mut int (phy_init)(struct phy,
    pub clk_names: *const *const c_char,
    pub num_clks: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snps_eusb2_hsphy {
    pub phy: *mut phy,
    pub base: *mut void __iomem,
    pub ref_clk: *mut clk,
    pub clks: *mut clk_bulk_data,
    pub phy_reset: *mut reset_control,
    pub vregs: [regulator_bulk_data; EUSB2_NUM_VREGS],
    pub mode: enum phy_mode,
    pub repeater: *mut phy,
    pub data: *const snps_eusb2_phy_drvdata,
}

#[no_mangle]
unsafe extern "C" fn snps_eusb2_hsphy_set_mode(p: *mut phy, mode: enum phy_mode, submode: c_int) -> c_int {
    static int snps_eusb2_hsphy_set_mode(struct phy *p, enum phy_mode mode, int submode)
    {
    struct snps_eusb2_hsphy *phy = phy_get_drvdata(p);
    phy.mode = mode;
    return phy_set_mode_ext(phy.repeater, mode, submode);
    }
    static void snps_eusb2_hsphy_write_mask(void __iomem *base, u32 offset,
    u32 mask, u32 val)
    {
    u32 reg;
    reg = readl_relaxed(base + offset);
    reg &= ~mask;
    reg |= val & mask;
    writel_relaxed(reg, base + offset);
// Ensure above write is completed
    readl_relaxed(base + offset);
    }
#[no_mangle]
unsafe extern "C" fn qcom_eusb2_default_parameters(phy: *mut snps_eusb2_hsphy) {
    static void qcom_eusb2_default_parameters(struct snps_eusb2_hsphy *phy)
    {
// default parameters: tx pre-emphasis
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_9,
    PHY_CFG_TX_PREEMP_TUNE_MASK,
    FIELD_PREP(PHY_CFG_TX_PREEMP_TUNE_MASK, 0));
// tx rise/fall time
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_9,
    PHY_CFG_TX_RISE_TUNE_MASK,
    FIELD_PREP(PHY_CFG_TX_RISE_TUNE_MASK, 0x2));
// source impedance adjustment
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_9,
    PHY_CFG_TX_RES_TUNE_MASK,
    FIELD_PREP(PHY_CFG_TX_RES_TUNE_MASK, 0x1));
// dc voltage level adjustement
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_8,
    PHY_CFG_TX_HS_VREF_TUNE_MASK,
    FIELD_PREP(PHY_CFG_TX_HS_VREF_TUNE_MASK, 0x3));
// transmitter HS crossover adjustement
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_8,
    PHY_CFG_TX_HS_XV_TUNE_MASK,
    FIELD_PREP(PHY_CFG_TX_HS_XV_TUNE_MASK, 0x0));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snps_eusb2_ref_clk {
    pub freq: c_ulong,
    pub fsel_val: u32,
    pub div_7_0_val: u32,
    pub div_11_8_val: u32,
}

    static const struct snps_eusb2_ref_clk exynos_eusb2_ref_clk[] = {
    { 19200000, FSEL_19_2_MHZ_VAL, DIV_19_8_19_2_MHZ_VAL, EXYNOS_DIV_11_8_19_2_MHZ_VAL },
    { 20000000, FSEL_20_MHZ_VAL, DIV_19_8_20_MHZ_VAL, EXYNOS_DIV_11_8_20_MHZ_VAL },
    { 24000000, FSEL_24_MHZ_VAL, DIV_19_8_24_MHZ_VAL, EXYNOS_DIV_11_8_24_MHZ_VAL },
    { 26000000, FSEL_26_MHZ_VAL, DIV_19_8_26_MHZ_VAL, EXYNOS_DIV_11_8_26_MHZ_VAL },
    { 48000000, FSEL_48_MHZ_VAL, DIV_19_8_48_MHZ_VAL, EXYNOS_DIV_11_8_48_MHZ_VAL },
    };
#[no_mangle]
unsafe extern "C" fn exynos_eusb2_ref_clk_init(phy: *mut snps_eusb2_hsphy) -> c_int {
    static int exynos_eusb2_ref_clk_init(struct snps_eusb2_hsphy *phy)
    {
    const struct snps_eusb2_ref_clk *config = core::ptr::null_mut();
    let mut ref_clk_freq: c_ulong = clk_get_rate(phy.ref_clk);
    for (int i = 0; i < ARRAY_SIZE(exynos_eusb2_ref_clk); i++) {
    if (exynos_eusb2_ref_clk[i].freq == ref_clk_freq) {
    config = &exynos_eusb2_ref_clk[i];
    break;
    }
    }
    if (!config) {
    dev_err(&phy.phy.dev, "unsupported ref_clk_freq: %lu\n", ref_clk_freq);
    return -EINVAL;
    }
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_HS_PHY_CTRL_COMMON,
    FSEL_MASK,
    FIELD_PREP(FSEL_MASK, config.fsel_val));
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_CFG_PLLCFG0,
    PHY_CFG_PLL_FB_DIV_19_8_MASK,
    FIELD_PREP(PHY_CFG_PLL_FB_DIV_19_8_MASK,
    config.div_7_0_val));
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_CFG_PLLCFG1,
    EXYNOS_PHY_CFG_PLL_FB_DIV_11_8_MASK,
    config.div_11_8_val);
    return 0;
    }
    static const struct snps_eusb2_ref_clk qcom_eusb2_ref_clk[] = {
    { 19200000, FSEL_19_2_MHZ_VAL, DIV_7_0_19_2_MHZ_VAL, DIV_11_8_19_2_MHZ_VAL },
    { 38400000, FSEL_38_4_MHZ_VAL, DIV_7_0_38_4_MHZ_VAL, DIV_11_8_38_4_MHZ_VAL },
    };
#[no_mangle]
unsafe extern "C" fn qcom_eusb2_ref_clk_init(phy: *mut snps_eusb2_hsphy) -> c_int {
    static int qcom_eusb2_ref_clk_init(struct snps_eusb2_hsphy *phy)
    {
    const struct snps_eusb2_ref_clk *config = core::ptr::null_mut();
    let mut ref_clk_freq: c_ulong = clk_get_rate(phy.ref_clk);
    for (int i = 0; i < ARRAY_SIZE(qcom_eusb2_ref_clk); i++) {
    if (qcom_eusb2_ref_clk[i].freq == ref_clk_freq) {
    config = &qcom_eusb2_ref_clk[i];
    break;
    }
    }
    if (!config) {
    dev_err(&phy.phy.dev, "unsupported ref_clk_freq: %lu\n", ref_clk_freq);
    return -EINVAL;
    }
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL_COMMON0,
    FSEL_MASK,
    FIELD_PREP(FSEL_MASK, config.fsel_val));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_2,
    PHY_CFG_PLL_FB_DIV_7_0_MASK,
    config.div_7_0_val);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_3,
    PHY_CFG_PLL_FB_DIV_11_8_MASK,
    config.div_11_8_val);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_3,
    PHY_CFG_PLL_REF_DIV, PLL_REF_DIV_VAL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos_snps_eusb2_hsphy_init(p: *mut phy) -> c_int {
    static int exynos_snps_eusb2_hsphy_init(struct phy *p)
    {
    struct snps_eusb2_hsphy *phy = phy_get_drvdata(p);
    int ret;
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_HS_PHY_CTRL_RST,
    USB_PHY_RST_MASK | UTMI_PORT_RST_MASK,
    USB_PHY_RST_MASK | UTMI_PORT_RST_MASK);
    fsleep(50); /* required after holding phy in reset */
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_HS_PHY_CTRL_COMMON,
    RPTR_MODE, RPTR_MODE);
// update ref_clk related registers
    ret = exynos_eusb2_ref_clk_init(phy);
    if (ret)
    return ret;
// default parameter: tx fsls-vref
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_PHY_CFG_TX,
    EXYNOS_PHY_CFG_TX_FSLS_VREF_TUNE_MASK,
    FIELD_PREP(EXYNOS_PHY_CFG_TX_FSLS_VREF_TUNE_MASK, 0x0));
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_UTMI_TESTSE,
    TEST_IDDQ, 0);
    fsleep(10); /* required after releasing test_iddq */
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_HS_PHY_CTRL_RST,
    USB_PHY_RST_MASK, 0);
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_HS_PHY_CTRL_COMMON,
    PHY_ENABLE, PHY_ENABLE);
    snps_eusb2_hsphy_write_mask(phy.base, EXYNOS_USB_PHY_HS_PHY_CTRL_RST,
    UTMI_PORT_RST_MASK, 0);
    return 0;
    }
    static const char * const exynos_eusb2_hsphy_clock_names[] = {
    "ref", "bus", "ctrl",
    };
    static const struct snps_eusb2_phy_drvdata exynos2200_snps_eusb2_phy = {
    .phy_init	= exynos_snps_eusb2_hsphy_init,
    .clk_names	= exynos_eusb2_hsphy_clock_names,
    .num_clks	= ARRAY_SIZE(exynos_eusb2_hsphy_clock_names),
    };
#[no_mangle]
unsafe extern "C" fn qcom_snps_eusb2_hsphy_init(p: *mut phy) -> c_int {
    static int qcom_snps_eusb2_hsphy_init(struct phy *p)
    {
    struct snps_eusb2_hsphy *phy = phy_get_drvdata(p);
    int ret;
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG0,
    CMN_CTRL_OVERRIDE_EN, CMN_CTRL_OVERRIDE_EN);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_UTMI_CTRL5, POR, POR);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL_COMMON0,
    PHY_ENABLE | RETENABLEN, PHY_ENABLE | RETENABLEN);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_APB_ACCESS_CMD,
    APB_LOGIC_RESET, APB_LOGIC_RESET);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_UTMI_PHY_CMN_CTRL0, TESTBURNIN, 0);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_FSEL_SEL,
    FSEL_SEL, FSEL_SEL);
// update ref_clk related registers
    ret = qcom_eusb2_ref_clk_init(phy);
    if (ret)
    return ret;
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_1,
    PHY_CFG_PLL_CPBIAS_CNTRL_MASK,
    FIELD_PREP(PHY_CFG_PLL_CPBIAS_CNTRL_MASK, 0x0));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_4,
    PHY_CFG_PLL_INT_CNTRL_MASK,
    FIELD_PREP(PHY_CFG_PLL_INT_CNTRL_MASK, 0x8));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_4,
    PHY_CFG_PLL_GMP_CNTRL_MASK,
    FIELD_PREP(PHY_CFG_PLL_GMP_CNTRL_MASK, 0x1));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_5,
    PHY_CFG_PLL_PROP_CNTRL_MASK,
    FIELD_PREP(PHY_CFG_PLL_PROP_CNTRL_MASK, 0x10));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_6,
    PHY_CFG_PLL_VCO_CNTRL_MASK,
    FIELD_PREP(PHY_CFG_PLL_VCO_CNTRL_MASK, 0x0));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG_CTRL_5,
    PHY_CFG_PLL_VREF_TUNE_MASK,
    FIELD_PREP(PHY_CFG_PLL_VREF_TUNE_MASK, 0x1));
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL2,
    VBUS_DET_EXT_SEL, VBUS_DET_EXT_SEL);
// set default parameters
    qcom_eusb2_default_parameters(phy);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL2,
    USB2_SUSPEND_N_SEL | USB2_SUSPEND_N,
    USB2_SUSPEND_N_SEL | USB2_SUSPEND_N);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_UTMI_CTRL0, SLEEPM, SLEEPM);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL_COMMON0,
    SIDDQ_SEL, SIDDQ_SEL);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL_COMMON0,
    SIDDQ, 0);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_UTMI_CTRL5, POR, 0);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_HS_PHY_CTRL2,
    USB2_SUSPEND_N_SEL, 0);
    snps_eusb2_hsphy_write_mask(phy.base, QCOM_USB_PHY_CFG0,
    CMN_CTRL_OVERRIDE_EN, 0);
    return 0;
    }
    static const char * const qcom_eusb2_hsphy_clock_names[] = {
    "ref",
    };
    static const struct snps_eusb2_phy_drvdata sm8550_snps_eusb2_phy = {
    .phy_init	= qcom_snps_eusb2_hsphy_init,
    .clk_names      = qcom_eusb2_hsphy_clock_names,
    .num_clks       = ARRAY_SIZE(qcom_eusb2_hsphy_clock_names),
    };
#[no_mangle]
unsafe extern "C" fn snps_eusb2_hsphy_init(p: *mut phy) -> c_int {
    static int snps_eusb2_hsphy_init(struct phy *p)
    {
    struct snps_eusb2_hsphy *phy = phy_get_drvdata(p);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(phy.vregs), phy.vregs);
    if (ret)
    return ret;
    ret = phy_init(phy.repeater);
    if (ret) {
    dev_err(&p.dev, "repeater init failed: %d\n", ret);
    goto disable_vreg;
    }
    ret = clk_bulk_prepare_enable(phy.data.num_clks, phy.clks);
    if (ret) {
    dev_err(&p.dev, "failed to enable ref clock: %d\n", ret);
    goto exit_repeater;
    }
    ret = reset_control_assert(phy.phy_reset);
    if (ret) {
    dev_err(&p.dev, "failed to assert phy_reset: %d\n", ret);
    goto disable_clks;
    }
    usleep_range(100, 150);
    ret = reset_control_deassert(phy.phy_reset);
    if (ret) {
    dev_err(&p.dev, "failed to de-assert phy_reset: %d\n", ret);
    goto disable_clks;
    }
    ret = phy.data.phy_init(p);
    if (ret)
    goto disable_clks;
    return 0;
    disable_clks:
    clk_bulk_disable_unprepare(phy.data.num_clks, phy.clks);
    exit_repeater:
    phy_exit(phy.repeater);
    disable_vreg:
    regulator_bulk_disable(ARRAY_SIZE(phy.vregs), phy.vregs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snps_eusb2_hsphy_exit(p: *mut phy) -> c_int {
    static int snps_eusb2_hsphy_exit(struct phy *p)
    {
    struct snps_eusb2_hsphy *phy = phy_get_drvdata(p);
    clk_bulk_disable_unprepare(phy.data.num_clks, phy.clks);
    regulator_bulk_disable(ARRAY_SIZE(phy.vregs), phy.vregs);
    phy_exit(phy.repeater);
    return 0;
    }
    static const struct phy_ops snps_eusb2_hsphy_ops = {
    .init		= snps_eusb2_hsphy_init,
    .exit		= snps_eusb2_hsphy_exit,
    .set_mode	= snps_eusb2_hsphy_set_mode,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn snps_eusb2_hsphy_probe(pdev: *mut platform_device) -> c_int {
    static int snps_eusb2_hsphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct snps_eusb2_hsphy *phy;
    struct phy_provider *phy_provider;
    struct phy *generic_phy;
    int ret, i;
    int num;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.data = device_get_match_data(dev);
    if (!phy.data)
    return -EINVAL;
    phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.base))
    return PTR_ERR(phy.base);
    phy.phy_reset = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(phy.phy_reset))
    return PTR_ERR(phy.phy_reset);
    phy.clks = devm_kcalloc(dev, phy.data.num_clks, sizeof(*phy.clks),
    GFP_KERNEL);
    if (!phy.clks)
    return -ENOMEM;
    for (i = 0; i < phy.data.num_clks; ++i)
    phy.clks[i].id = phy.data.clk_names[i];
    ret = devm_clk_bulk_get(dev, phy.data.num_clks, phy.clks);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get phy clock(s)\n");
    phy.ref_clk = core::ptr::null_mut();
    for (i = 0; i < phy.data.num_clks; ++i) {
    if (!strcmp(phy.clks[i].id, "ref")) {
    phy.ref_clk = phy.clks[i].clk;
    break;
    }
    }
    if (IS_ERR_OR_NULL(phy.ref_clk)) {
    ret = phy.ref_clk ? PTR_ERR(phy.ref_clk) : -ENOENT;
    return dev_err_probe(dev, ret,
    "failed to get ref clk\n");
    }
    num = ARRAY_SIZE(phy.vregs);
    for (i = 0; i < num; i++)
    phy.vregs[i].supply = eusb2_hsphy_vreg_names[i];
    ret = devm_regulator_bulk_get(dev, num, phy.vregs);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to get regulator supplies\n");
    phy.repeater = devm_of_phy_optional_get(dev, np, core::ptr::null_mut());
    if (IS_ERR(phy.repeater))
    return dev_err_probe(dev, PTR_ERR(phy.repeater),
    "failed to get repeater\n");
    generic_phy = devm_phy_create(dev, core::ptr::null_mut(), &snps_eusb2_hsphy_ops);
    if (IS_ERR(generic_phy)) {
    dev_err(dev, "failed to create phy: %d\n", ret);
    return PTR_ERR(generic_phy);
    }
    dev_set_drvdata(dev, phy);
    phy_set_drvdata(generic_phy, phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    return 0;
    }
    static const struct of_device_id snps_eusb2_hsphy_of_match_table[] = {
    {
    .compatible = "qcom,sm8550-snps-eusb2-phy",
    .data = &sm8550_snps_eusb2_phy,
    }, {
    .compatible = "samsung,exynos2200-eusb2-phy",
    .data = &exynos2200_snps_eusb2_phy,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, snps_eusb2_hsphy_of_match_table);
    static struct platform_driver snps_eusb2_hsphy_driver = {
    .probe		= snps_eusb2_hsphy_probe,
    .driver = {
    .name	= "snps-eusb2-hsphy",
    .of_match_table = snps_eusb2_hsphy_of_match_table,
    },
    };
    module_platform_driver(snps_eusb2_hsphy_driver);
    MODULE_DESCRIPTION("Synopsys eUSB2 HS PHY driver");
    MODULE_LICENSE("GPL");
