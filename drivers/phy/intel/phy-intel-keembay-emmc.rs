//! Automatically rewritten from C to Rust
//! Source: drivers/phy/intel/phy-intel-keembay-emmc.c
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
// Intel Keem Bay eMMC PHY driver
// Copyright (C) 2020 Intel Corporation
//

// eMMC/SD/SDIO core/phy configuration registers
pub const PHY_CFG_0: c_uint = 0x24;

pub const PHY_CFG_2: c_uint = 0x2c;

pub const PHY_STAT: c_uint = 0x40;

// From ACS_eMMC51_16nFFC_RO1100_Userguide_v1p0.pdf p17
pub const FREQSEL_200M_170M: c_uint = 0x0;
pub const FREQSEL_170M_140M: c_uint = 0x1;
pub const FREQSEL_140M_110M: c_uint = 0x2;
pub const FREQSEL_110M_80M: c_uint = 0x3;
pub const FREQSEL_80M_50M: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct keembay_emmc_phy {
    pub syscfg: *mut regmap,
    pub emmcclk: *mut clk,
}

    static const struct regmap_config keembay_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn keembay_emmc_phy_power(phy: *mut phy, on_off: bool) -> c_int {
    static int keembay_emmc_phy_power(struct phy *phy, bool on_off)
    {
    struct keembay_emmc_phy *priv = phy_get_drvdata(phy);
    unsigned int caldone;
    unsigned int dllrdy;
    unsigned int freqsel;
    unsigned int mhz;
    int ret;
//
// Keep phyctrl_pdb and phyctrl_endll low to allow
// initialization of CALIO state M/C DFFs
//
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, PWR_DOWN_MASK,
    FIELD_PREP(PWR_DOWN_MASK, 0));
    if (ret) {
    dev_err(&phy.dev, "CALIO power down bar failed: %d\n", ret);
    return ret;
    }
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, DLL_EN_MASK,
    FIELD_PREP(DLL_EN_MASK, 0));
    if (ret) {
    dev_err(&phy.dev, "turn off the dll failed: %d\n", ret);
    return ret;
    }
// Already finish power off above
    if (!on_off)
    return 0;
    mhz = DIV_ROUND_CLOSEST(clk_get_rate(priv.emmcclk), 1000000);
    if (mhz <= 200 && mhz >= 170)
    freqsel = FREQSEL_200M_170M;
#[no_mangle]
pub unsafe extern "C" fn if(140: mhz <= 170 && mhz >=) -> else {
    else if (mhz <= 170 && mhz >= 140)
    freqsel = FREQSEL_170M_140M;
#[no_mangle]
pub unsafe extern "C" fn if(110: mhz <= 140 && mhz >=) -> else {
    else if (mhz <= 140 && mhz >= 110)
    freqsel = FREQSEL_140M_110M;
#[no_mangle]
pub unsafe extern "C" fn if(80: mhz <= 110 && mhz >=) -> else {
    else if (mhz <= 110 && mhz >= 80)
    freqsel = FREQSEL_110M_80M;
#[no_mangle]
pub unsafe extern "C" fn if(50: mhz <= 80 && mhz >=) -> else {
    else if (mhz <= 80 && mhz >= 50)
    freqsel = FREQSEL_80M_50M;
    else
    freqsel = 0x0;
// Check for EMMC clock rate
    if (mhz > 175)
    dev_warn(&phy.dev, "Unsupported rate: %d MHz\n", mhz);
//
// According to the user manual, calpad calibration
// cycle takes more than 2us without the minimal recommended
// value, so we may need a little margin here
//
    udelay(5);
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, PWR_DOWN_MASK,
    FIELD_PREP(PWR_DOWN_MASK, 1));
    if (ret) {
    dev_err(&phy.dev, "CALIO power down bar failed: %d\n", ret);
    return ret;
    }
//
// According to the user manual, it asks driver to wait 5us for
// calpad busy trimming. However it is documented that this value is
// PVT(A.K.A. process, voltage and temperature) relevant, so some
// failure cases are found which indicates we should be more tolerant
// to calpad busy trimming.
//
    ret = regmap_read_poll_timeout(priv.syscfg, PHY_STAT,
    caldone, IS_CALDONE(caldone),
    0, 50);
    if (ret) {
    dev_err(&phy.dev, "caldone failed, ret=%d\n", ret);
    return ret;
    }
// Set the frequency of the DLL operation
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_2, SEL_FREQ_MASK,
    FIELD_PREP(SEL_FREQ_MASK, freqsel));
    if (ret) {
    dev_err(&phy.dev, "set the frequency of dll failed:%d\n", ret);
    return ret;
    }
// Turn on the DLL
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, DLL_EN_MASK,
    FIELD_PREP(DLL_EN_MASK, 1));
    if (ret) {
    dev_err(&phy.dev, "turn on the dll failed: %d\n", ret);
    return ret;
    }
//
// We turned on the DLL even though the rate was 0 because we the
// clock might be turned on later.  ...but we can't wait for the DLL
// to lock when the rate is 0 because it will never lock with no
// input clock.
//
// Technically we should be checking the lock later when the clock
// is turned on, but for now we won't.
//
    if (mhz == 0)
    return 0;
//
// After enabling analog DLL circuits docs say that we need 10.2 us if
// our source clock is at 50 MHz and that lock time scales linearly
// with clock speed. If we are powering on the PHY and the card clock
// is super slow (like 100kHz) this could take as long as 5.1 ms as
// per the math: 10.2 us * (50000000 Hz / 100000 Hz) => 5.1 ms
// hopefully we won't be running at 100 kHz, but we should still make
// sure we wait long enough.
//
// NOTE: There appear to be corner cases where the DLL seems to take
// extra long to lock for reasons that aren't understood. In some
// extreme cases we've seen it take up to over 10ms (!). We'll be
// generous and give it 50ms.
//
    ret = regmap_read_poll_timeout(priv.syscfg, PHY_STAT,
    dllrdy, IS_DLLRDY(dllrdy),
    0, 50 * USEC_PER_MSEC);
    if (ret)
    dev_err(&phy.dev, "dllrdy failed, ret=%d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn keembay_emmc_phy_init(phy: *mut phy) -> c_int {
    static int keembay_emmc_phy_init(struct phy *phy)
    {
    struct keembay_emmc_phy *priv = phy_get_drvdata(phy);
//
// We purposely get the clock here and not in probe to avoid the
// circular dependency problem. We expect:
// - PHY driver to probe
// - SDHCI driver to start probe
// - SDHCI driver to register it's clock
// - SDHCI driver to get the PHY
// - SDHCI driver to init the PHY
//
// The clock is optional, so upon any error just return it like
// any other error to user.
//
    priv.emmcclk = clk_get_optional(&phy.dev, "emmcclk");
    return PTR_ERR_OR_ZERO(priv.emmcclk);
    }
#[no_mangle]
unsafe extern "C" fn keembay_emmc_phy_exit(phy: *mut phy) -> c_int {
    static int keembay_emmc_phy_exit(struct phy *phy)
    {
    struct keembay_emmc_phy *priv = phy_get_drvdata(phy);
    clk_put(priv.emmcclk);
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn keembay_emmc_phy_power_on(phy: *mut phy) -> c_int {
    static int keembay_emmc_phy_power_on(struct phy *phy)
    {
    struct keembay_emmc_phy *priv = phy_get_drvdata(phy);
    int ret;
// Delay chain based txclk: enable
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, SEL_DLY_TXCLK_MASK,
    FIELD_PREP(SEL_DLY_TXCLK_MASK, 1));
    if (ret) {
    dev_err(&phy.dev, "ERROR: delay chain txclk set: %d\n", ret);
    return ret;
    }
// Output tap delay: enable
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, OTAP_DLY_ENA_MASK,
    FIELD_PREP(OTAP_DLY_ENA_MASK, 1));
    if (ret) {
    dev_err(&phy.dev, "ERROR: output tap delay set: %d\n", ret);
    return ret;
    }
// Output tap delay
    ret = regmap_update_bits(priv.syscfg, PHY_CFG_0, OTAP_DLY_SEL_MASK,
    FIELD_PREP(OTAP_DLY_SEL_MASK, 2));
    if (ret) {
    dev_err(&phy.dev, "ERROR: output tap delay select: %d\n", ret);
    return ret;
    }
// Power up eMMC phy analog blocks
    return keembay_emmc_phy_power(phy, true);
    }
#[no_mangle]
unsafe extern "C" fn keembay_emmc_phy_power_off(phy: *mut phy) -> c_int {
    static int keembay_emmc_phy_power_off(struct phy *phy)
    {
// Power down eMMC phy analog blocks
    return keembay_emmc_phy_power(phy, false);
    }
    static const struct phy_ops ops = {
    .init		= keembay_emmc_phy_init,
    .exit		= keembay_emmc_phy_exit,
    .power_on	= keembay_emmc_phy_power_on,
    .power_off	= keembay_emmc_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn keembay_emmc_phy_probe(pdev: *mut platform_device) -> c_int {
    static int keembay_emmc_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct keembay_emmc_phy *priv;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    void __iomem *base;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.syscfg = devm_regmap_init_mmio(dev, base, &keembay_regmap_config);
    if (IS_ERR(priv.syscfg))
    return PTR_ERR(priv.syscfg);
    generic_phy = devm_phy_create(dev, np, &ops);
    if (IS_ERR(generic_phy))
    return dev_err_probe(dev, PTR_ERR(generic_phy),
    "failed to create PHY\n");
    phy_set_drvdata(generic_phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id keembay_emmc_phy_dt_ids[] = {
    { .compatible = "intel,keembay-emmc-phy" },
    {}
    };
    MODULE_DEVICE_TABLE(of, keembay_emmc_phy_dt_ids);
    static struct platform_driver keembay_emmc_phy_driver = {
    .probe		= keembay_emmc_phy_probe,
    .driver		= {
    .name	= "keembay-emmc-phy",
    .of_match_table = keembay_emmc_phy_dt_ids,
    },
    };
    module_platform_driver(keembay_emmc_phy_driver);
    MODULE_AUTHOR("Wan Ahmad Zainie <wan.ahmad.zainie.wan.mohamad@intel.com>");
    MODULE_DESCRIPTION("Intel Keem Bay eMMC PHY driver");
    MODULE_LICENSE("GPL v2");
