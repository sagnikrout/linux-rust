//! Automatically rewritten from C to Rust
//! Source: drivers/devfreq/sun8i-a33-mbus.c
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
// Copyright (C) 2020-2021 Samuel Holland <samuel@sholland.org>
//

pub const MBUS_CR: c_uint = 0x0000;

pub const MBUS_CR_DRAM_TYPE_DDR2: c_int = 2;
pub const MBUS_CR_DRAM_TYPE_DDR3: c_int = 3;
pub const MBUS_CR_DRAM_TYPE_DDR4: c_int = 4;
pub const MBUS_CR_DRAM_TYPE_LPDDR2: c_int = 6;
pub const MBUS_CR_DRAM_TYPE_LPDDR3: c_int = 7;
pub const MBUS_TMR: c_uint = 0x000c;

pub const MBUS_PMU_CFG: c_uint = 0x009c;

pub const MBUS_MDFSCR: c_uint = 0x0100;

pub const MBUS_MDFSMRMR: c_uint = 0x0108;
pub const DRAM_PWRCTL: c_uint = 0x0004;

pub const DRAM_RFSHTMG: c_uint = 0x0090;

pub const DRAM_VTFCR: c_uint = 0x00b8;

pub const DRAM_ODTMAP: c_uint = 0x0120;
pub const DRAM_DX_MAX: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_a33_mbus_variant {
    pub min_dram_divider: u32,
    pub max_dram_divider: u32,
    pub odt_freq_mhz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun8i_a33_mbus {
    pub variant: *const sun8i_a33_mbus_variant,
    pub reg_dram: *mut void __iomem,
    pub reg_mbus: *mut void __iomem,
    pub clk_bus: *mut clk,
    pub clk_dram: *mut clk,
    pub clk_mbus: *mut clk,
    pub devfreq_dram: *mut devfreq,
    pub gov_data: devfreq_simple_ondemand_data,
    pub profile: devfreq_dev_profile,
    pub data_width: u32,
    pub nominal_bw: u32,
    pub odtmap: u32,
    pub tREFI_ns: u32,
    pub tRFC_ns: u32,
    pub freq_table: [c_ulong; ],
}

//
// The unit for this value is (MBUS clock cycles / MBUS_TMR_PERIOD). When
// MBUS_TMR_PERIOD is programmed to match the MBUS clock frequency in MHz, as
// it is during DRAM init and during probe, the resulting unit is microseconds.
//
    let mut pmu_period: static int = 50000;
    module_param(pmu_period, int, 0644);
    MODULE_PARM_DESC(pmu_period, "Bandwidth measurement period (microseconds)");
#[no_mangle]
unsafe extern "C" fn sun8i_a33_mbus_get_peak_bw(priv: *mut sun8i_a33_mbus) -> u32 {
    static u32 sun8i_a33_mbus_get_peak_bw(struct sun8i_a33_mbus *priv)
    {
// Returns the peak transfer (in KiB) during any single PMU period.
    return readl_relaxed(priv.reg_mbus + MBUS_TOTAL_BWCR);
    }
#[no_mangle]
unsafe extern "C" fn sun8i_a33_mbus_restart_pmu_counters(priv: *mut sun8i_a33_mbus) {
    static void sun8i_a33_mbus_restart_pmu_counters(struct sun8i_a33_mbus *priv)
    {
    let mut pmu_cfg: u32 = MBUS_PMU_CFG_PERIOD(pmu_period) | MBUS_PMU_CFG_UNIT_KB;
// All PMU counters are cleared on a disable->enable transition.
    writel_relaxed(pmu_cfg,
    priv.reg_mbus + MBUS_PMU_CFG);
    writel_relaxed(pmu_cfg | MBUS_PMU_CFG_ENABLE,
    priv.reg_mbus + MBUS_PMU_CFG);
    }
    static void sun8i_a33_mbus_update_nominal_bw(struct sun8i_a33_mbus *priv,
    u32 ddr_freq_mhz)
    {
//
// Nominal bandwidth (KiB per PMU period):
//
// DDR transfers   microseconds     KiB
// ------------- * ------------ * --------
// microsecond     PMU period    transfer
//
    priv.nominal_bw = ddr_freq_mhz * pmu_period * priv.data_width / 1024;
    }
    static int sun8i_a33_mbus_set_dram_freq(struct sun8i_a33_mbus *priv,
    unsigned long freq)
    {
    u32  ddr_freq_mhz = freq / USEC_PER_SEC; /* DDR */
    u32 dram_freq_mhz =    ddr_freq_mhz / 2; /* SDR */
    u32 mctl_freq_mhz =   dram_freq_mhz / 2; /* HDR */
    u32 dxodt, mdfscr, pwrctl, vtfcr;
    u32 i, tREFI_32ck, tRFC_ck;
    int ret;
// The rate change is not effective until the MDFS process runs.
    ret = clk_set_rate(priv.clk_dram, freq);
    if (ret)
    return ret;
// Disable automatic self-refesh and VTF before starting MDFS.
    pwrctl = readl_relaxed(priv.reg_dram + DRAM_PWRCTL) &
    ~DRAM_PWRCTL_SELFREF_EN;
    writel_relaxed(pwrctl, priv.reg_dram + DRAM_PWRCTL);
    vtfcr = readl_relaxed(priv.reg_dram + DRAM_VTFCR);
    writel_relaxed(vtfcr & ~DRAM_VTFCR_VTF_ENABLE,
    priv.reg_dram + DRAM_VTFCR);
// Set up MDFS and enable double buffering for timing registers.
    mdfscr = MBUS_MDFSCR_MODE_DFS |
    MBUS_MDFSCR_BYPASS |
    MBUS_MDFSCR_PAD_HOLD |
    MBUS_MDFSCR_BUFFER_TIMING;
    writel(mdfscr, priv.reg_mbus + MBUS_MDFSCR);
// Update the buffered copy of RFSHTMG.
    tREFI_32ck = priv.tREFI_ns * mctl_freq_mhz / 1000 / 32;
    tRFC_ck = DIV_ROUND_UP(priv.tRFC_ns * mctl_freq_mhz, 1000);
    writel(DRAM_RFSHTMG_TREFI(tREFI_32ck) | DRAM_RFSHTMG_TRFC(tRFC_ck),
    priv.reg_dram + DRAM_RFSHTMG);
// Enable ODT if needed, or disable it to save power.
    if (priv.odtmap && dram_freq_mhz > priv.variant.odt_freq_mhz) {
    dxodt = DRAM_DXnGCR0_DXODT_DYNAMIC;
    writel(priv.odtmap, priv.reg_dram + DRAM_ODTMAP);
    } else {
    dxodt = DRAM_DXnGCR0_DXODT_DISABLED;
    writel(0, priv.reg_dram + DRAM_ODTMAP);
    }
    for (i = 0; i < DRAM_DX_MAX; ++i) {
    void __iomem *reg = priv.reg_dram + DRAM_DXnGCR0(i);
    writel((readl(reg) & ~DRAM_DXnGCR0_DXODT) | dxodt, reg);
    }
    dev_dbg(priv.devfreq_dram.dev.parent,
    "Setting DRAM to %u MHz, tREFI=%u, tRFC=%u, ODT=%s\n",
    dram_freq_mhz, tREFI_32ck, tRFC_ck,
    dxodt == DRAM_DXnGCR0_DXODT_DYNAMIC ? "dynamic" : "disabled");
// Trigger hardware MDFS.
    writel(mdfscr | MBUS_MDFSCR_START, priv.reg_mbus + MBUS_MDFSCR);
    ret = readl_poll_timeout_atomic(priv.reg_mbus + MBUS_MDFSCR, mdfscr,
    !(mdfscr & MBUS_MDFSCR_START), 10, 1000);
    if (ret)
    return ret;
// Disable double buffering.
    writel(0, priv.reg_mbus + MBUS_MDFSCR);
// Restore VTF configuration.
    writel_relaxed(vtfcr, priv.reg_dram + DRAM_VTFCR);
// Enable automatic self-refresh at the lowest frequency only.
    if (freq == priv.freq_table[0])
    pwrctl |= DRAM_PWRCTL_SELFREF_EN;
    writel_relaxed(pwrctl, priv.reg_dram + DRAM_PWRCTL);
    sun8i_a33_mbus_restart_pmu_counters(priv);
    sun8i_a33_mbus_update_nominal_bw(priv, ddr_freq_mhz);
    return 0;
    }
    static int sun8i_a33_mbus_set_dram_target(struct device *dev,
    unsigned long *freq, u32 flags)
    {
    struct sun8i_a33_mbus *priv = dev_get_drvdata(dev);
    struct devfreq *devfreq = priv.devfreq_dram;
    struct dev_pm_opp *opp;
    int ret;
    opp = devfreq_recommended_opp(dev, freq, flags);
    if (IS_ERR(opp))
    return PTR_ERR(opp);
    dev_pm_opp_put(opp);
    if (*freq == devfreq.previous_freq)
    return 0;
    ret = sun8i_a33_mbus_set_dram_freq(priv, *freq);
    if (ret) {
    dev_warn(dev, "failed to set DRAM frequency: %d\n", ret);
// freq = devfreq->previous_freq;
    }
    return ret;
    }
    static int sun8i_a33_mbus_get_dram_status(struct device *dev,
    struct devfreq_dev_status *stat)
    {
    struct sun8i_a33_mbus *priv = dev_get_drvdata(dev);
    stat.busy_time		= sun8i_a33_mbus_get_peak_bw(priv);
    stat.total_time	= priv.nominal_bw;
    stat.current_frequency	= priv.devfreq_dram.previous_freq;
    sun8i_a33_mbus_restart_pmu_counters(priv);
    dev_dbg(dev, "Using %lu/%lu (%lu%%) at %lu MHz\n",
    stat.busy_time, stat.total_time,
    DIV_ROUND_CLOSEST(stat.busy_time * 100, stat.total_time),
    stat.current_frequency / USEC_PER_SEC);
    return 0;
    }
    static int sun8i_a33_mbus_hw_init(struct device *dev,
    struct sun8i_a33_mbus *priv,
    unsigned long ddr_freq)
    {
    u32 i, mbus_cr, mbus_freq_mhz;
// Choose tREFI and tRFC to match the configured DRAM type.
    mbus_cr = readl_relaxed(priv.reg_mbus + MBUS_CR);
    switch (MBUS_CR_GET_DRAM_TYPE(mbus_cr)) {
    case MBUS_CR_DRAM_TYPE_DDR2:
    case MBUS_CR_DRAM_TYPE_DDR3:
    case MBUS_CR_DRAM_TYPE_DDR4:
    priv.tREFI_ns = 7800;
    priv.tRFC_ns = 350;
    break;
    case MBUS_CR_DRAM_TYPE_LPDDR2:
    case MBUS_CR_DRAM_TYPE_LPDDR3:
    priv.tREFI_ns = 3900;
    priv.tRFC_ns = 210;
    break;
    default:
    return -EINVAL;
    }
// Save ODTMAP so it can be restored when raising the frequency.
    priv.odtmap = readl_relaxed(priv.reg_dram + DRAM_ODTMAP);
// Compute the DRAM data bus width by counting enabled DATx8 blocks.
    for (i = 0; i < DRAM_DX_MAX; ++i) {
    void __iomem *reg = priv.reg_dram + DRAM_DXnGCR0(i);
    if (!(readl_relaxed(reg) & DRAM_DXnGCR0_DXEN))
    break;
    }
    priv.data_width = i;
    dev_dbg(dev, "Detected %u-bit %sDDRx with%s ODT\n",
    priv.data_width * 8,
    MBUS_CR_GET_DRAM_TYPE(mbus_cr) > 4 ? "LP" : "",
    priv.odtmap ? "" : "out");
// Program MBUS_TMR such that the PMU period unit is microseconds.
    mbus_freq_mhz = clk_get_rate(priv.clk_mbus) / USEC_PER_SEC;
    writel_relaxed(MBUS_TMR_PERIOD(mbus_freq_mhz),
    priv.reg_mbus + MBUS_TMR);
// "Master Ready Mask Register" bits must be set or MDFS will block.
    writel_relaxed(0xffffffff, priv.reg_mbus + MBUS_MDFSMRMR);
    sun8i_a33_mbus_restart_pmu_counters(priv);
    sun8i_a33_mbus_update_nominal_bw(priv, ddr_freq / USEC_PER_SEC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun8i_a33_mbus_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sun8i_a33_mbus_suspend(struct device *dev)
    {
    struct sun8i_a33_mbus *priv = dev_get_drvdata(dev);
    clk_disable_unprepare(priv.clk_bus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun8i_a33_mbus_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sun8i_a33_mbus_resume(struct device *dev)
    {
    struct sun8i_a33_mbus *priv = dev_get_drvdata(dev);
    return clk_prepare_enable(priv.clk_bus);
    }
#[no_mangle]
unsafe extern "C" fn sun8i_a33_mbus_probe(pdev: *mut platform_device) -> c_int {
    static int sun8i_a33_mbus_probe(struct platform_device *pdev)
    {
    const struct sun8i_a33_mbus_variant *variant;
    struct device *dev = &pdev.dev;
    struct sun8i_a33_mbus *priv;
    unsigned long base_freq;
    unsigned int max_state;
    const char *err;
    int i, ret;
    variant = device_get_match_data(dev);
    if (!variant)
    return -EINVAL;
    max_state = variant.max_dram_divider - variant.min_dram_divider + 1;
    priv = devm_kzalloc(dev, struct_size(priv, freq_table, max_state), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.variant = variant;
    priv.reg_dram = devm_platform_ioremap_resource_byname(pdev, "dram");
    if (IS_ERR(priv.reg_dram))
    return PTR_ERR(priv.reg_dram);
    priv.reg_mbus = devm_platform_ioremap_resource_byname(pdev, "mbus");
    if (IS_ERR(priv.reg_mbus))
    return PTR_ERR(priv.reg_mbus);
    priv.clk_bus = devm_clk_get_enabled(dev, "bus");
    if (IS_ERR(priv.clk_bus))
    return dev_err_probe(dev, PTR_ERR(priv.clk_bus),
    "failed to get bus clock\n");
    priv.clk_dram = devm_clk_get(dev, "dram");
    if (IS_ERR(priv.clk_dram))
    return dev_err_probe(dev, PTR_ERR(priv.clk_dram),
    "failed to get dram clock\n");
    priv.clk_mbus = devm_clk_get(dev, "mbus");
    if (IS_ERR(priv.clk_mbus))
    return dev_err_probe(dev, PTR_ERR(priv.clk_mbus),
    "failed to get mbus clock\n");
// Lock the DRAM clock rate to keep priv->nominal_bw in sync.
    ret = devm_clk_rate_exclusive_get(dev, priv.clk_dram);
    if (ret)
    return dev_err_probe(dev, ret, "failed to lock dram clock rate\n");
// Lock the MBUS clock rate to keep MBUS_TMR_PERIOD in sync.
    ret = devm_clk_rate_exclusive_get(dev, priv.clk_mbus);
    if (ret)
    return dev_err_probe(dev, ret, "failed to lock mbus clock rate\n");
    priv.gov_data.upthreshold	= 10;
    priv.gov_data.downdifferential	=  5;
    priv.profile.initial_freq	= clk_get_rate(priv.clk_dram);
    priv.profile.polling_ms	= 1000;
    priv.profile.target		= sun8i_a33_mbus_set_dram_target;
    priv.profile.get_dev_status	= sun8i_a33_mbus_get_dram_status;
    priv.profile.freq_table	= priv.freq_table;
    priv.profile.max_state		= max_state;
    ret = devm_pm_opp_set_clkname(dev, "dram");
    if (ret)
    return dev_err_probe(dev, ret, "failed to add OPP table\n");
    base_freq = clk_get_rate(clk_get_parent(priv.clk_dram));
    for (i = 0; i < max_state; ++i) {
    let mut div: c_uint = variant.max_dram_divider - i;
    priv.freq_table[i] = base_freq / div;
    ret = dev_pm_opp_add(dev, priv.freq_table[i], 0);
    if (ret) {
    err = "failed to add OPPs\n";
    goto err_remove_opps;
    }
    }
    ret = sun8i_a33_mbus_hw_init(dev, priv, priv.profile.initial_freq);
    if (ret) {
    err = "failed to init hardware\n";
    goto err_remove_opps;
    }
    priv.devfreq_dram = devfreq_add_device(dev, &priv.profile,
    DEVFREQ_GOV_SIMPLE_ONDEMAND,
    &priv.gov_data);
    if (IS_ERR(priv.devfreq_dram)) {
    ret = PTR_ERR(priv.devfreq_dram);
    err = "failed to add devfreq device\n";
    goto err_remove_opps;
    }
//
// This must be set manually after registering the devfreq device,
// because there is no way to select a dynamic OPP as the suspend OPP.
//
    priv.devfreq_dram.suspend_freq = priv.freq_table[0];
    return 0;
    err_remove_opps:
    dev_pm_opp_remove_all_dynamic(dev);
    return dev_err_probe(dev, ret, err);
    }
#[no_mangle]
unsafe extern "C" fn sun8i_a33_mbus_remove(pdev: *mut platform_device) {
    static void sun8i_a33_mbus_remove(struct platform_device *pdev)
    {
    struct sun8i_a33_mbus *priv = platform_get_drvdata(pdev);
    let mut initial_freq: c_ulong = priv.profile.initial_freq;
    struct device *dev = &pdev.dev;
    int ret;
    devfreq_remove_device(priv.devfreq_dram);
    ret = sun8i_a33_mbus_set_dram_freq(priv, initial_freq);
    if (ret)
    dev_warn(dev, "failed to restore DRAM frequency: %d\n", ret);
    dev_pm_opp_remove_all_dynamic(dev);
    }
    static const struct sun8i_a33_mbus_variant sun50i_a64_mbus = {
    .min_dram_divider	= 1,
    .max_dram_divider	= 4,
    .odt_freq_mhz		= 400,
    };
    static const struct of_device_id sun8i_a33_mbus_of_match[] = {
    { .compatible = "allwinner,sun50i-a64-mbus", .data = &sun50i_a64_mbus },
    { .compatible = "allwinner,sun50i-h5-mbus", .data = &sun50i_a64_mbus },
    { },
    };
    MODULE_DEVICE_TABLE(of, sun8i_a33_mbus_of_match);
    static SIMPLE_DEV_PM_OPS(sun8i_a33_mbus_pm_ops,
    sun8i_a33_mbus_suspend, sun8i_a33_mbus_resume);
    static struct platform_driver sun8i_a33_mbus_driver = {
    .probe	= sun8i_a33_mbus_probe,
    .remove = sun8i_a33_mbus_remove,
    .driver	= {
    .name		= "sun8i-a33-mbus",
    .of_match_table	= sun8i_a33_mbus_of_match,
    .pm		= pm_ptr(&sun8i_a33_mbus_pm_ops),
    },
    };
    module_platform_driver(sun8i_a33_mbus_driver);
    MODULE_AUTHOR("Samuel Holland <samuel@sholland.org>");
    MODULE_DESCRIPTION("Allwinner sun8i/sun50i MBUS DEVFREQ Driver");
    MODULE_LICENSE("GPL v2");
