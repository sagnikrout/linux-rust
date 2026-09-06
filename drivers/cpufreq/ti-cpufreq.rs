//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/ti-cpufreq.c
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
// TI CPUFreq/OPP hw-supported driver
//
// Copyright (C) 2016-2017 Texas Instruments, Inc.
// Dave Gerlach <d-gerlach@ti.com>
//

pub const REVISION_MASK: c_uint = 0xF;
pub const REVISION_SHIFT: c_int = 28;
pub const AM33XX_800M_ARM_MPU_MAX_FREQ: c_uint = 0x1E2F;
pub const AM43XX_600M_ARM_MPU_MAX_FREQ: c_uint = 0xFFA;
pub const DRA7_EFUSE_HAS_OD_MPU_OPP: c_int = 11;
pub const DRA7_EFUSE_HAS_HIGH_MPU_OPP: c_int = 15;
pub const DRA76_EFUSE_HAS_PLUS_MPU_OPP: c_int = 18;
pub const DRA7_EFUSE_HAS_ALL_MPU_OPP: c_int = 23;
pub const DRA76_EFUSE_HAS_ALL_MPU_OPP: c_int = 24;

pub const OMAP3_CONTROL_DEVICE_STATUS: c_uint = 0x4800244C;
pub const OMAP3_CONTROL_IDCODE: c_uint = 0x4830A204;
pub const OMAP34xx_ProdID_SKUID: c_uint = 0x4830A20C;

pub const AM625_EFUSE_K_MPU_OPP: c_int = 11;
pub const AM625_EFUSE_S_MPU_OPP: c_int = 19;
pub const AM625_EFUSE_T_MPU_OPP: c_int = 20;

    enum {
    AM62A7_EFUSE_M_MPU_OPP =		13,
    AM62A7_EFUSE_N_MPU_OPP,
    AM62A7_EFUSE_O_MPU_OPP,
    AM62A7_EFUSE_P_MPU_OPP,
    AM62A7_EFUSE_Q_MPU_OPP,
    AM62A7_EFUSE_R_MPU_OPP,
    AM62A7_EFUSE_S_MPU_OPP,
//
// The V, U, and T speed grade numbering is out of order
// to align with the AM625 more uniformly. I promise I know
// my ABCs ;)
//
    AM62A7_EFUSE_V_MPU_OPP,
    AM62A7_EFUSE_U_MPU_OPP,
    AM62A7_EFUSE_T_MPU_OPP,
    };

pub const AM62L3_EFUSE_E_MPU_OPP: c_int = 5;
pub const AM62L3_EFUSE_O_MPU_OPP: c_int = 15;

pub const AM62P5_EFUSE_O_MPU_OPP: c_int = 15;
pub const AM62P5_EFUSE_S_MPU_OPP: c_int = 19;
pub const AM62P5_EFUSE_T_MPU_OPP: c_int = 20;
pub const AM62P5_EFUSE_U_MPU_OPP: c_int = 21;
pub const AM62P5_EFUSE_V_MPU_OPP: c_int = 22;

pub const VERSION_COUNT: c_int = 2;
    struct ti_cpufreq_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_cpufreq_soc_data {
    pub reg_names: *const *const c_char,
    unsigned long (*efuse_xlate)(struct ti_cpufreq_data *opp_data,
    pub efuse): c_ulong,
    pub efuse_fallback: c_ulong,
    pub efuse_offset: c_ulong,
    pub efuse_mask: c_ulong,
    pub efuse_shift: c_ulong,
    pub rev_offset: c_ulong,
    pub multi_regulator: bool,
    pub needs_k3_socinfo: bool,
// Backward compatibility hack: Might have missing syscon
pub const TI_QUIRK_SYSCON_MAY_BE_MISSING: c_uint = 0x1;
// Backward compatibility hack: new syscon size is 1 register wide
pub const TI_QUIRK_SYSCON_IS_SINGLE_REG: c_uint = 0x2;
    pub quirks: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_cpufreq_data {
    pub cpu_dev: *mut device,
    pub opp_node: *mut device_node,
    pub syscon: *mut regmap,
    pub soc_data: *const ti_cpufreq_soc_data,
}

    static unsigned long amx3_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
    if (!efuse)
    efuse = opp_data.soc_data.efuse_fallback;
// AM335x and AM437x use "OPP disable" bits, so invert
    return ~efuse;
    }
    static unsigned long dra7_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
    let mut calculated_efuse: c_ulong = DRA7_EFUSE_NOM_MPU_OPP;
//
// The efuse on dra7 and am57 parts contains a specific
// value indicating the highest available OPP.
//
    switch (efuse) {
    case DRA76_EFUSE_HAS_PLUS_MPU_OPP:
    case DRA76_EFUSE_HAS_ALL_MPU_OPP:
    calculated_efuse |= DRA76_EFUSE_PLUS_MPU_OPP;
    fallthrough;
    case DRA7_EFUSE_HAS_ALL_MPU_OPP:
    case DRA7_EFUSE_HAS_HIGH_MPU_OPP:
    calculated_efuse |= DRA7_EFUSE_HIGH_MPU_OPP;
    fallthrough;
    case DRA7_EFUSE_HAS_OD_MPU_OPP:
    calculated_efuse |= DRA7_EFUSE_OD_MPU_OPP;
    }
    return calculated_efuse;
    }
    static unsigned long omap3_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
// OPP enable bit ("Speed Binned")
    return BIT(efuse);
    }
    static unsigned long am62p5_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
    let mut calculated_efuse: c_ulong = AM62P5_SUPPORT_O_MPU_OPP;
    switch (efuse) {
    case AM62P5_EFUSE_V_MPU_OPP:
    case AM62P5_EFUSE_U_MPU_OPP:
    case AM62P5_EFUSE_T_MPU_OPP:
    case AM62P5_EFUSE_S_MPU_OPP:
    calculated_efuse |= AM62P5_SUPPORT_U_MPU_OPP;
    fallthrough;
    case AM62P5_EFUSE_O_MPU_OPP:
    calculated_efuse |= AM62P5_SUPPORT_O_MPU_OPP;
    }
    return calculated_efuse;
    }
    static unsigned long am62a7_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
    let mut calculated_efuse: c_ulong = AM62A7_SUPPORT_N_MPU_OPP;
    switch (efuse) {
    case AM62A7_EFUSE_V_MPU_OPP:
    case AM62A7_EFUSE_U_MPU_OPP:
    case AM62A7_EFUSE_T_MPU_OPP:
    case AM62A7_EFUSE_S_MPU_OPP:
    calculated_efuse |= AM62A7_SUPPORT_V_MPU_OPP;
    fallthrough;
    case AM62A7_EFUSE_R_MPU_OPP:
    case AM62A7_EFUSE_Q_MPU_OPP:
    case AM62A7_EFUSE_P_MPU_OPP:
    case AM62A7_EFUSE_O_MPU_OPP:
    calculated_efuse |= AM62A7_SUPPORT_R_MPU_OPP;
    fallthrough;
    case AM62A7_EFUSE_N_MPU_OPP:
    case AM62A7_EFUSE_M_MPU_OPP:
    calculated_efuse |= AM62A7_SUPPORT_N_MPU_OPP;
    }
    return calculated_efuse;
    }
    static unsigned long am625_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
    let mut calculated_efuse: c_ulong = AM625_SUPPORT_K_MPU_OPP;
    switch (efuse) {
    case AM625_EFUSE_T_MPU_OPP:
    calculated_efuse |= AM625_SUPPORT_T_MPU_OPP;
    fallthrough;
    case AM625_EFUSE_S_MPU_OPP:
    calculated_efuse |= AM625_SUPPORT_S_MPU_OPP;
    fallthrough;
    case AM625_EFUSE_K_MPU_OPP:
    calculated_efuse |= AM625_SUPPORT_K_MPU_OPP;
    }
    return calculated_efuse;
    }
    static unsigned long am62l3_efuse_xlate(struct ti_cpufreq_data *opp_data,
    unsigned long efuse)
    {
    let mut calculated_efuse: c_ulong = AM62L3_SUPPORT_E_MPU_OPP;
    switch (efuse) {
    case AM62L3_EFUSE_O_MPU_OPP:
    calculated_efuse |= AM62L3_SUPPORT_O_MPU_OPP;
    fallthrough;
    case AM62L3_EFUSE_E_MPU_OPP:
    calculated_efuse |= AM62L3_SUPPORT_E_MPU_OPP;
    }
    return calculated_efuse;
    }
    static struct ti_cpufreq_soc_data am3x_soc_data = {
    .efuse_xlate = amx3_efuse_xlate,
    .efuse_fallback = AM33XX_800M_ARM_MPU_MAX_FREQ,
    .efuse_offset = 0x07fc,
    .efuse_mask = 0x1fff,
    .rev_offset = 0x600,
    .multi_regulator = false,
    };
    static struct ti_cpufreq_soc_data am4x_soc_data = {
    .efuse_xlate = amx3_efuse_xlate,
    .efuse_fallback = AM43XX_600M_ARM_MPU_MAX_FREQ,
    .efuse_offset = 0x0610,
    .efuse_mask = 0x3f,
    .rev_offset = 0x600,
    .multi_regulator = false,
    };
    static struct ti_cpufreq_soc_data dra7_soc_data = {
    .efuse_xlate = dra7_efuse_xlate,
    .efuse_offset = 0x020c,
    .efuse_mask = 0xf80000,
    .efuse_shift = 19,
    .rev_offset = 0x204,
    .multi_regulator = true,
    };
//
// OMAP35x TRM (SPRUF98K):
// CONTROL_IDCODE (0x4830 A204) describes Silicon revisions.
// Control OMAP Status Register 15:0 (Address 0x4800 244C)
// to separate between omap3503, omap3515, omap3525, omap3530
// and feature presence.
// There are encodings for versions limited to 400/266MHz
// but we ignore.
// Not clear if this also holds for omap34xx.
// some eFuse values e.g. CONTROL_FUSE_OPP1_VDD1
// are stored in the SYSCON register range
// Register 0x4830A20C [ProdID.SKUID] [0:3]
// 0x0 for normal 600/430MHz device.
// 0x8 for 720/520MHz device.
// Not clear what omap34xx value is.
//
    static struct ti_cpufreq_soc_data omap34xx_soc_data = {
    .efuse_xlate = omap3_efuse_xlate,
    .efuse_offset = OMAP34xx_ProdID_SKUID - OMAP3_SYSCON_BASE,
    .efuse_shift = 3,
    .efuse_mask = BIT(3),
    .rev_offset = OMAP3_CONTROL_IDCODE - OMAP3_SYSCON_BASE,
    .multi_regulator = false,
    .quirks = TI_QUIRK_SYSCON_MAY_BE_MISSING,
    };
//
// AM/DM37x TRM (SPRUGN4M)
// CONTROL_IDCODE (0x4830 A204) describes Silicon revisions.
// Control Device Status Register 15:0 (Address 0x4800 244C)
// to separate between am3703, am3715, dm3725, dm3730
// and feature presence.
// Speed Binned = Bit 9
// 0 800/600 MHz
// 1 1000/800 MHz
// some eFuse values e.g. CONTROL_FUSE_OPP 1G_VDD1
// are stored in the SYSCON register range.
// There is no 0x4830A20C [ProdID.SKUID] register (exists but
// seems to always read as 0).
//
    static const char * const omap3_reg_names[] = {"cpu0", "vbb", core::ptr::null_mut()};
    static struct ti_cpufreq_soc_data omap36xx_soc_data = {
    .reg_names = omap3_reg_names,
    .efuse_xlate = omap3_efuse_xlate,
    .efuse_offset = OMAP3_CONTROL_DEVICE_STATUS - OMAP3_SYSCON_BASE,
    .efuse_shift = 9,
    .efuse_mask = BIT(9),
    .rev_offset = OMAP3_CONTROL_IDCODE - OMAP3_SYSCON_BASE,
    .multi_regulator = true,
    .quirks = TI_QUIRK_SYSCON_MAY_BE_MISSING,
    };
//
// AM3517 is quite similar to AM/DM37x except that it has no
// high speed grade eFuse and no abb ldo
//
    static struct ti_cpufreq_soc_data am3517_soc_data = {
    .efuse_xlate = omap3_efuse_xlate,
    .efuse_offset = OMAP3_CONTROL_DEVICE_STATUS - OMAP3_SYSCON_BASE,
    .efuse_shift = 0,
    .efuse_mask = 0,
    .rev_offset = OMAP3_CONTROL_IDCODE - OMAP3_SYSCON_BASE,
    .multi_regulator = false,
    .quirks = TI_QUIRK_SYSCON_MAY_BE_MISSING,
    };
    static const struct soc_device_attribute k3_cpufreq_soc[] = {
    { .family = "AM62X", },
    { .family = "AM62AX", },
    { .family = "AM62DX", },
    { .family = "AM62LX", },
    { .family = "AM62PX", },
    { /* sentinel */ }
    };
    static struct ti_cpufreq_soc_data am625_soc_data = {
    .efuse_xlate = am625_efuse_xlate,
    .efuse_offset = 0x0018,
    .efuse_mask = 0x07c0,
    .efuse_shift = 0x6,
    .multi_regulator = false,
    .needs_k3_socinfo = true,
    .quirks = TI_QUIRK_SYSCON_IS_SINGLE_REG,
    };
    static struct ti_cpufreq_soc_data am62a7_soc_data = {
    .efuse_xlate = am62a7_efuse_xlate,
    .efuse_offset = 0x0,
    .efuse_mask = 0x07c0,
    .efuse_shift = 0x6,
    .multi_regulator = false,
    .needs_k3_socinfo = true,
    };
    static struct ti_cpufreq_soc_data am62l3_soc_data = {
    .efuse_xlate = am62l3_efuse_xlate,
    .efuse_offset = 0x0,
    .efuse_mask = 0x07c0,
    .efuse_shift = 0x6,
    .multi_regulator = false,
    .needs_k3_socinfo = true,
    };
    static struct ti_cpufreq_soc_data am62p5_soc_data = {
    .efuse_xlate = am62p5_efuse_xlate,
    .efuse_offset = 0x0,
    .efuse_mask = 0x07c0,
    .efuse_shift = 0x6,
    .multi_regulator = false,
    .needs_k3_socinfo = true,
    };
//
// ti_cpufreq_get_efuse() - Parse and return efuse value present on SoC
// @opp_data: pointer to ti_cpufreq_data context
// @efuse_value: Set to the value parsed from efuse
//
// Returns error code if efuse not read properly.
//
    static int ti_cpufreq_get_efuse(struct ti_cpufreq_data *opp_data,
    u32 *efuse_value)
    {
    struct device *dev = opp_data.cpu_dev;
    u32 efuse;
    int ret;
    ret = regmap_read(opp_data.syscon, opp_data.soc_data.efuse_offset,
    &efuse);
    if (opp_data.soc_data.quirks & TI_QUIRK_SYSCON_IS_SINGLE_REG && ret == -EIO)
    ret = regmap_read(opp_data.syscon, 0x0, &efuse);
    if (opp_data.soc_data.quirks & TI_QUIRK_SYSCON_MAY_BE_MISSING && ret == -EIO) {
// not a syscon register!
    void __iomem *regs = ioremap(OMAP3_SYSCON_BASE +
    opp_data.soc_data.efuse_offset, 4);
    if (!regs)
    return -ENOMEM;
    efuse = readl(regs);
    iounmap(regs);
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    dev_err(dev,
    "Failed to read the efuse value from syscon: %d\n",
    ret);
    return ret;
    }
    efuse = (efuse & opp_data.soc_data.efuse_mask);
    efuse >>= opp_data.soc_data.efuse_shift;
// efuse_value = opp_data->soc_data->efuse_xlate(opp_data, efuse);
    return 0;
    }
//
// ti_cpufreq_get_rev() - Parse and return rev value present on SoC
// @opp_data: pointer to ti_cpufreq_data context
// @revision_value: Set to the value parsed from revision register
//
// Returns error code if revision not read properly.
//
    static int ti_cpufreq_get_rev(struct ti_cpufreq_data *opp_data,
    u32 *revision_value)
    {
    struct device *dev = opp_data.cpu_dev;
    u32 revision;
    int ret;
    if (soc_device_match(k3_cpufreq_soc)) {
//
// Since the SR is 1.0, hard code the revision_value as
// 0x1 here. This way we avoid re using the same register
// that is giving us required information inside socinfo
// anyway.
//
// revision_value = 0x1;
    goto done;
    }
// Defer if k3-socinfo hasn't registered the SoC device yet
    if (opp_data.soc_data.needs_k3_socinfo)
    return dev_err_probe(opp_data.cpu_dev, -EPROBE_DEFER,
    "SoC device not registered by k3-socinfo\n");
    ret = regmap_read(opp_data.syscon, opp_data.soc_data.rev_offset,
    &revision);
    if (opp_data.soc_data.quirks & TI_QUIRK_SYSCON_MAY_BE_MISSING && ret == -EIO) {
// not a syscon register!
    void __iomem *regs = ioremap(OMAP3_SYSCON_BASE +
    opp_data.soc_data.rev_offset, 4);
    if (!regs)
    return -ENOMEM;
    revision = readl(regs);
    iounmap(regs);
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    dev_err(dev,
    "Failed to read the revision number from syscon: %d\n",
    ret);
    return ret;
    }
// revision_value = BIT((revision >> REVISION_SHIFT) & REVISION_MASK);
    done:
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_cpufreq_setup_syscon_register(opp_data: *mut ti_cpufreq_data) -> c_int {
    static int ti_cpufreq_setup_syscon_register(struct ti_cpufreq_data *opp_data)
    {
    struct device *dev = opp_data.cpu_dev;
    struct device_node *np = opp_data.opp_node;
    opp_data.syscon = syscon_regmap_lookup_by_phandle(np,
    "syscon");
    if (IS_ERR(opp_data.syscon)) {
    dev_err(dev,
    "\"syscon\" is missing, cannot use OPPv2 table.\n");
    return PTR_ERR(opp_data.syscon);
    }
    return 0;
    }
    static const struct of_device_id ti_cpufreq_of_match[]  __maybe_unused = {
    { .compatible = "ti,am33xx", .data = &am3x_soc_data, },
    { .compatible = "ti,am3517", .data = &am3517_soc_data, },
    { .compatible = "ti,am43", .data = &am4x_soc_data, },
    { .compatible = "ti,dra7", .data = &dra7_soc_data },
    { .compatible = "ti,omap34xx", .data = &omap34xx_soc_data, },
    { .compatible = "ti,omap36xx", .data = &omap36xx_soc_data, },
    { .compatible = "ti,am625", .data = &am625_soc_data, },
    { .compatible = "ti,am62a7", .data = &am62a7_soc_data, },
    { .compatible = "ti,am62d2", .data = &am62a7_soc_data, },
    { .compatible = "ti,am62l3", .data = &am62l3_soc_data, },
    { .compatible = "ti,am62p5", .data = &am62p5_soc_data, },
// legacy
    { .compatible = "ti,omap3430", .data = &omap34xx_soc_data, },
    { .compatible = "ti,omap3630", .data = &omap36xx_soc_data, },
    {},
    };
#[no_mangle]
unsafe extern "C" fn ti_cpufreq_probe(pdev: *mut platform_device) -> c_int {
    static int ti_cpufreq_probe(struct platform_device *pdev)
    {
    u32 version[VERSION_COUNT];
    const struct of_device_id *match;
    struct ti_cpufreq_data *opp_data;
    const char * const default_reg_names[] = {"vdd", "vbb", core::ptr::null_mut()};
    int ret;
    struct dev_pm_opp_config config = {
    .supported_hw = version,
    .supported_hw_count = ARRAY_SIZE(version),
    };
    match = dev_get_platdata(&pdev.dev);
    if (!match)
    return -ENODEV;
    opp_data = devm_kzalloc(&pdev.dev, sizeof(*opp_data), GFP_KERNEL);
    if (!opp_data)
    return -ENOMEM;
    opp_data.soc_data = match.data;
    opp_data.cpu_dev = get_cpu_device(0);
    if (!opp_data.cpu_dev) {
    pr_err("%s: Failed to get device for CPU0\n", __func__);
    return -ENODEV;
    }
    opp_data.opp_node = dev_pm_opp_of_get_opp_desc_node(opp_data.cpu_dev);
    if (!opp_data.opp_node) {
    dev_info(opp_data.cpu_dev,
    "OPP-v2 not supported, cpufreq-dt will attempt to use legacy tables.\n");
    goto register_cpufreq_dt;
    }
    ret = ti_cpufreq_setup_syscon_register(opp_data);
    if (ret)
    goto fail_put_node;
//
// OPPs determine whether or not they are supported based on
// two metrics:
// 0 - SoC Revision
// 1 - eFuse value
//
    ret = ti_cpufreq_get_rev(opp_data, &version[0]);
    if (ret)
    goto fail_put_node;
    ret = ti_cpufreq_get_efuse(opp_data, &version[1]);
    if (ret)
    goto fail_put_node;
    if (opp_data.soc_data.multi_regulator) {
    if (opp_data.soc_data.reg_names)
    config.regulator_names = opp_data.soc_data.reg_names;
    else
    config.regulator_names = default_reg_names;
    }
    ret = dev_pm_opp_set_config(opp_data.cpu_dev, &config);
    if (ret < 0) {
    dev_err_probe(opp_data.cpu_dev, ret, "Failed to set OPP config\n");
    goto fail_put_node;
    }
    of_node_put(opp_data.opp_node);
    register_cpufreq_dt:
    platform_device_register_simple("cpufreq-dt", -1, core::ptr::null_mut(), 0);
    return 0;
    fail_put_node:
    of_node_put(opp_data.opp_node);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ti_cpufreq_init() -> int __init {
    static int __init ti_cpufreq_init(void)
    {
    const struct of_device_id *match;
// Check to ensure we are on a compatible platform
    match = of_machine_get_match(ti_cpufreq_of_match);
    if (match)
    platform_device_register_data(core::ptr::null_mut(), "ti-cpufreq", -1, match,
    sizeof(*match));
    return 0;
    }
    module_init(ti_cpufreq_init);
    static struct platform_driver ti_cpufreq_driver = {
    .probe = ti_cpufreq_probe,
    .driver = {
    .name = "ti-cpufreq",
    },
    };
    builtin_platform_driver(ti_cpufreq_driver);
    MODULE_DESCRIPTION("TI CPUFreq/OPP hw-supported driver");
    MODULE_AUTHOR("Dave Gerlach <d-gerlach@ti.com>");
    MODULE_LICENSE("GPL v2");
