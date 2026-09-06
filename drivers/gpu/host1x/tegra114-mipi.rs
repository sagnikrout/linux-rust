//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/host1x/tegra114-mipi.c
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


//
// Copyright (C) 2013 NVIDIA Corporation
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

pub const MIPI_CAL_CTRL: c_uint = 0x00;

pub const MIPI_CAL_AUTOCAL_CTRL: c_uint = 0x01;
pub const MIPI_CAL_STATUS: c_uint = 0x02;

pub const MIPI_CAL_CONFIG_CSIA: c_uint = 0x05;
pub const MIPI_CAL_CONFIG_CSIB: c_uint = 0x06;
pub const MIPI_CAL_CONFIG_CSIC: c_uint = 0x07;
pub const MIPI_CAL_CONFIG_CSID: c_uint = 0x08;
pub const MIPI_CAL_CONFIG_CSIE: c_uint = 0x09;
pub const MIPI_CAL_CONFIG_CSIF: c_uint = 0x0a;
pub const MIPI_CAL_CONFIG_DSIA: c_uint = 0x0e;
pub const MIPI_CAL_CONFIG_DSIB: c_uint = 0x0f;
pub const MIPI_CAL_CONFIG_DSIC: c_uint = 0x10;
pub const MIPI_CAL_CONFIG_DSID: c_uint = 0x11;
pub const MIPI_CAL_CONFIG_DSIA_CLK: c_uint = 0x19;
pub const MIPI_CAL_CONFIG_DSIB_CLK: c_uint = 0x1a;
pub const MIPI_CAL_CONFIG_CSIAB_CLK: c_uint = 0x1b;
pub const MIPI_CAL_CONFIG_DSIC_CLK: c_uint = 0x1c;
pub const MIPI_CAL_CONFIG_CSICD_CLK: c_uint = 0x1c;
pub const MIPI_CAL_CONFIG_DSID_CLK: c_uint = 0x1d;
pub const MIPI_CAL_CONFIG_CSIE_CLK: c_uint = 0x1d;
// for data and clock lanes

// for data lanes

// for clock lanes

pub const MIPI_CAL_BIAS_PAD_CFG0: c_uint = 0x16;

pub const MIPI_CAL_BIAS_PAD_CFG1: c_uint = 0x17;

pub const MIPI_CAL_BIAS_PAD_CFG2: c_uint = 0x18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mipi_pad {
    pub data: c_ulong,
    pub clk: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mipi_soc {
    pub has_clk_lane: bool,
    pub pads: *const tegra_mipi_pad,
    pub num_pads: c_uint,
    pub clock_enable_override: bool,
    pub needs_vclamp_ref: bool,
// bias pad configuration settings
    pub pad_drive_down_ref: u8,
    pub pad_drive_up_ref: u8,
    pub pad_vclamp_level: u8,
    pub pad_vauxp_level: u8,
// calibration settings for data lanes
    pub hspdos: u8,
    pub hspuos: u8,
    pub termos: u8,
// calibration settings for clock lanes
    pub hsclkpdos: u8,
    pub hsclkpuos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mipi {
    pub soc: *const tegra_mipi_soc,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub /: *mut *mut mutex lock; / for register access,
    pub clk: *mut clk,
    pub usage_count: c_ulong,
}

    static inline u32 tegra_mipi_readl(struct tegra_mipi *mipi,
    unsigned long offset)
    {
    return readl(mipi.regs + (offset << 2));
    }
    static inline void tegra_mipi_writel(struct tegra_mipi *mipi, u32 value,
    unsigned long offset)
    {
    writel(value, mipi.regs + (offset << 2));
    }
#[no_mangle]
unsafe extern "C" fn tegra114_mipi_power_up(mipi: *mut tegra_mipi) -> c_int {
    static int tegra114_mipi_power_up(struct tegra_mipi *mipi)
    {
    u32 value;
    int err;
    err = clk_enable(mipi.clk);
    if (err < 0)
    return err;
    value = tegra_mipi_readl(mipi, MIPI_CAL_BIAS_PAD_CFG0);
    value &= ~MIPI_CAL_BIAS_PAD_PDVCLAMP;
    if (mipi.soc.needs_vclamp_ref)
    value |= MIPI_CAL_BIAS_PAD_E_VCLAMP_REF;
    tegra_mipi_writel(mipi, value, MIPI_CAL_BIAS_PAD_CFG0);
    value = tegra_mipi_readl(mipi, MIPI_CAL_BIAS_PAD_CFG2);
    value &= ~MIPI_CAL_BIAS_PAD_PDVREG;
    tegra_mipi_writel(mipi, value, MIPI_CAL_BIAS_PAD_CFG2);
    clk_disable(mipi.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra114_mipi_power_down(mipi: *mut tegra_mipi) -> c_int {
    static int tegra114_mipi_power_down(struct tegra_mipi *mipi)
    {
    u32 value;
    int err;
    err = clk_enable(mipi.clk);
    if (err < 0)
    return err;
//
// The MIPI_CAL_BIAS_PAD_PDVREG controls a voltage regulator that
// supplies the DSI pads. This must be kept enabled until none of the
// DSI lanes are used anymore.
//
    value = tegra_mipi_readl(mipi, MIPI_CAL_BIAS_PAD_CFG2);
    value |= MIPI_CAL_BIAS_PAD_PDVREG;
    tegra_mipi_writel(mipi, value, MIPI_CAL_BIAS_PAD_CFG2);
//
// MIPI_CAL_BIAS_PAD_PDVCLAMP and MIPI_CAL_BIAS_PAD_E_VCLAMP_REF
// control a regulator that supplies current to the pre-driver logic.
// Powering down this regulator causes DSI to fail, so it must remain
// powered on until none of the DSI lanes are used anymore.
//
    value = tegra_mipi_readl(mipi, MIPI_CAL_BIAS_PAD_CFG0);
    if (mipi.soc.needs_vclamp_ref)
    value &= ~MIPI_CAL_BIAS_PAD_E_VCLAMP_REF;
    value |= MIPI_CAL_BIAS_PAD_PDVCLAMP;
    tegra_mipi_writel(mipi, value, MIPI_CAL_BIAS_PAD_CFG0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra114_mipi_enable(mipidev: *mut tegra_mipi_device) -> c_int {
    static int tegra114_mipi_enable(struct tegra_mipi_device *mipidev)
    {
    struct tegra_mipi *mipi = platform_get_drvdata(mipidev.pdev);
    let mut err: c_int = 0;
    mutex_lock(&mipi.lock);
    if (mipi.usage_count++ == 0)
    err = tegra114_mipi_power_up(mipi);
    mutex_unlock(&mipi.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra114_mipi_disable(mipidev: *mut tegra_mipi_device) -> c_int {
    static int tegra114_mipi_disable(struct tegra_mipi_device *mipidev)
    {
    struct tegra_mipi *mipi = platform_get_drvdata(mipidev.pdev);
    let mut err: c_int = 0;
    mutex_lock(&mipi.lock);
    if (--mipi.usage_count == 0)
    err = tegra114_mipi_power_down(mipi);
    mutex_unlock(&mipi.lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra114_mipi_finish_calibration(mipidev: *mut tegra_mipi_device) -> c_int {
    static int tegra114_mipi_finish_calibration(struct tegra_mipi_device *mipidev)
    {
    struct tegra_mipi *mipi = platform_get_drvdata(mipidev.pdev);
    void __iomem *status_reg = mipi.regs + (MIPI_CAL_STATUS << 2);
    u32 value;
    int err;
    err = readl_relaxed_poll_timeout(status_reg, value,
    !(value & MIPI_CAL_STATUS_ACTIVE) &&
    (value & MIPI_CAL_STATUS_DONE), 50,
    250000);
    mutex_unlock(&mipi.lock);
    clk_disable(mipi.clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra114_mipi_start_calibration(mipidev: *mut tegra_mipi_device) -> c_int {
    static int tegra114_mipi_start_calibration(struct tegra_mipi_device *mipidev)
    {
    struct tegra_mipi *mipi = platform_get_drvdata(mipidev.pdev);
    const struct tegra_mipi_soc *soc = mipi.soc;
    unsigned int i;
    u32 value;
    int err;
    err = clk_enable(mipi.clk);
    if (err < 0)
    return err;
    mutex_lock(&mipi.lock);
    value = MIPI_CAL_BIAS_PAD_DRV_DN_REF(soc.pad_drive_down_ref) |
    MIPI_CAL_BIAS_PAD_DRV_UP_REF(soc.pad_drive_up_ref);
    tegra_mipi_writel(mipi, value, MIPI_CAL_BIAS_PAD_CFG1);
    value = tegra_mipi_readl(mipi, MIPI_CAL_BIAS_PAD_CFG2);
    value &= ~MIPI_CAL_BIAS_PAD_VCLAMP(0x7);
    value &= ~MIPI_CAL_BIAS_PAD_VAUXP(0x7);
    value |= MIPI_CAL_BIAS_PAD_VCLAMP(soc.pad_vclamp_level);
    value |= MIPI_CAL_BIAS_PAD_VAUXP(soc.pad_vauxp_level);
    tegra_mipi_writel(mipi, value, MIPI_CAL_BIAS_PAD_CFG2);
    for (i = 0; i < soc.num_pads; i++) {
    let mut clk: u32 = 0, data = 0;
    if (mipidev.pads & BIT(i)) {
    data = MIPI_CAL_CONFIG_SELECT |
    MIPI_CAL_CONFIG_HSPDOS(soc.hspdos) |
    MIPI_CAL_CONFIG_HSPUOS(soc.hspuos) |
    MIPI_CAL_CONFIG_TERMOS(soc.termos);
    clk = MIPI_CAL_CONFIG_SELECT |
    MIPI_CAL_CONFIG_HSCLKPDOSD(soc.hsclkpdos) |
    MIPI_CAL_CONFIG_HSCLKPUOSD(soc.hsclkpuos);
    }
    tegra_mipi_writel(mipi, data, soc.pads[i].data);
    if (soc.has_clk_lane && soc.pads[i].clk != 0)
    tegra_mipi_writel(mipi, clk, soc.pads[i].clk);
    }
    value = tegra_mipi_readl(mipi, MIPI_CAL_CTRL);
    value &= ~MIPI_CAL_CTRL_NOISE_FILTER(0xf);
    value &= ~MIPI_CAL_CTRL_PRESCALE(0x3);
    value |= MIPI_CAL_CTRL_NOISE_FILTER(0xa);
    value |= MIPI_CAL_CTRL_PRESCALE(0x2);
    if (!soc.clock_enable_override)
    value &= ~MIPI_CAL_CTRL_CLKEN_OVR;
    else
    value |= MIPI_CAL_CTRL_CLKEN_OVR;
    tegra_mipi_writel(mipi, value, MIPI_CAL_CTRL);
// clear any pending status bits
    value = tegra_mipi_readl(mipi, MIPI_CAL_STATUS);
    tegra_mipi_writel(mipi, value, MIPI_CAL_STATUS);
    value = tegra_mipi_readl(mipi, MIPI_CAL_CTRL);
    value |= MIPI_CAL_CTRL_START;
    tegra_mipi_writel(mipi, value, MIPI_CAL_CTRL);
//
// Wait for min 72uS to let calibration logic finish calibration
// sequence codes before waiting for pads idle state to apply the
// results.
//
    usleep_range(75, 80);
    return 0;
    }
    static const struct tegra_mipi_ops tegra114_mipi_ops = {
    .enable = tegra114_mipi_enable,
    .disable = tegra114_mipi_disable,
    .start_calibration = tegra114_mipi_start_calibration,
    .finish_calibration = tegra114_mipi_finish_calibration,
    };
    static const struct tegra_mipi_pad tegra114_mipi_pads[] = {
    { .data = MIPI_CAL_CONFIG_CSIA },
    { .data = MIPI_CAL_CONFIG_CSIB },
    { .data = MIPI_CAL_CONFIG_CSIC },
    { .data = MIPI_CAL_CONFIG_CSID },
    { .data = MIPI_CAL_CONFIG_CSIE },
    { .data = MIPI_CAL_CONFIG_DSIA },
    { .data = MIPI_CAL_CONFIG_DSIB },
    { .data = MIPI_CAL_CONFIG_DSIC },
    { .data = MIPI_CAL_CONFIG_DSID },
    };
    static const struct tegra_mipi_soc tegra114_mipi_soc = {
    .has_clk_lane = false,
    .pads = tegra114_mipi_pads,
    .num_pads = ARRAY_SIZE(tegra114_mipi_pads),
    .clock_enable_override = true,
    .needs_vclamp_ref = true,
    .pad_drive_down_ref = 0x2,
    .pad_drive_up_ref = 0x0,
    .pad_vclamp_level = 0x0,
    .pad_vauxp_level = 0x0,
    .hspdos = 0x0,
    .hspuos = 0x4,
    .termos = 0x5,
    .hsclkpdos = 0x0,
    .hsclkpuos = 0x4,
    };
    static const struct tegra_mipi_pad tegra124_mipi_pads[] = {
    { .data = MIPI_CAL_CONFIG_CSIA, .clk = MIPI_CAL_CONFIG_CSIAB_CLK },
    { .data = MIPI_CAL_CONFIG_CSIB, .clk = MIPI_CAL_CONFIG_CSIAB_CLK },
    { .data = MIPI_CAL_CONFIG_CSIC, .clk = MIPI_CAL_CONFIG_CSICD_CLK },
    { .data = MIPI_CAL_CONFIG_CSID, .clk = MIPI_CAL_CONFIG_CSICD_CLK },
    { .data = MIPI_CAL_CONFIG_CSIE, .clk = MIPI_CAL_CONFIG_CSIE_CLK  },
    { .data = MIPI_CAL_CONFIG_DSIA, .clk = MIPI_CAL_CONFIG_DSIA_CLK  },
    { .data = MIPI_CAL_CONFIG_DSIB, .clk = MIPI_CAL_CONFIG_DSIB_CLK  },
    };
    static const struct tegra_mipi_soc tegra124_mipi_soc = {
    .has_clk_lane = true,
    .pads = tegra124_mipi_pads,
    .num_pads = ARRAY_SIZE(tegra124_mipi_pads),
    .clock_enable_override = true,
    .needs_vclamp_ref = true,
    .pad_drive_down_ref = 0x2,
    .pad_drive_up_ref = 0x0,
    .pad_vclamp_level = 0x0,
    .pad_vauxp_level = 0x0,
    .hspdos = 0x0,
    .hspuos = 0x0,
    .termos = 0x0,
    .hsclkpdos = 0x1,
    .hsclkpuos = 0x2,
    };
    static const struct tegra_mipi_soc tegra132_mipi_soc = {
    .has_clk_lane = true,
    .pads = tegra124_mipi_pads,
    .num_pads = ARRAY_SIZE(tegra124_mipi_pads),
    .clock_enable_override = false,
    .needs_vclamp_ref = false,
    .pad_drive_down_ref = 0x0,
    .pad_drive_up_ref = 0x3,
    .pad_vclamp_level = 0x0,
    .pad_vauxp_level = 0x0,
    .hspdos = 0x0,
    .hspuos = 0x0,
    .termos = 0x0,
    .hsclkpdos = 0x3,
    .hsclkpuos = 0x2,
    };
    static const struct tegra_mipi_pad tegra210_mipi_pads[] = {
    { .data = MIPI_CAL_CONFIG_CSIA, .clk = 0 },
    { .data = MIPI_CAL_CONFIG_CSIB, .clk = 0 },
    { .data = MIPI_CAL_CONFIG_CSIC, .clk = 0 },
    { .data = MIPI_CAL_CONFIG_CSID, .clk = 0 },
    { .data = MIPI_CAL_CONFIG_CSIE, .clk = 0 },
    { .data = MIPI_CAL_CONFIG_CSIF, .clk = 0 },
    { .data = MIPI_CAL_CONFIG_DSIA, .clk = MIPI_CAL_CONFIG_DSIA_CLK },
    { .data = MIPI_CAL_CONFIG_DSIB, .clk = MIPI_CAL_CONFIG_DSIB_CLK },
    { .data = MIPI_CAL_CONFIG_DSIC, .clk = MIPI_CAL_CONFIG_DSIC_CLK },
    { .data = MIPI_CAL_CONFIG_DSID, .clk = MIPI_CAL_CONFIG_DSID_CLK },
    };
    static const struct tegra_mipi_soc tegra210_mipi_soc = {
    .has_clk_lane = true,
    .pads = tegra210_mipi_pads,
    .num_pads = ARRAY_SIZE(tegra210_mipi_pads),
    .clock_enable_override = true,
    .needs_vclamp_ref = false,
    .pad_drive_down_ref = 0x0,
    .pad_drive_up_ref = 0x3,
    .pad_vclamp_level = 0x1,
    .pad_vauxp_level = 0x1,
    .hspdos = 0x0,
    .hspuos = 0x2,
    .termos = 0x0,
    .hsclkpdos = 0x0,
    .hsclkpuos = 0x2,
    };
    static const struct of_device_id tegra_mipi_of_match[] = {
    { .compatible = "nvidia,tegra114-mipi", .data = &tegra114_mipi_soc },
    { .compatible = "nvidia,tegra124-mipi", .data = &tegra124_mipi_soc },
    { .compatible = "nvidia,tegra132-mipi", .data = &tegra132_mipi_soc },
    { .compatible = "nvidia,tegra210-mipi", .data = &tegra210_mipi_soc },
    { },
    };
#[no_mangle]
unsafe extern "C" fn tegra_mipi_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_mipi_probe(struct platform_device *pdev)
    {
    const struct of_device_id *match;
    struct tegra_mipi *mipi;
    match = of_match_node(tegra_mipi_of_match, pdev.dev.of_node);
    if (!match)
    return -ENODEV;
    mipi = devm_kzalloc(&pdev.dev, sizeof(*mipi), GFP_KERNEL);
    if (!mipi)
    return -ENOMEM;
    mipi.soc = match.data;
    mipi.dev = &pdev.dev;
    mipi.regs = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(mipi.regs))
    return PTR_ERR(mipi.regs);
    mutex_init(&mipi.lock);
    mipi.clk = devm_clk_get_prepared(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mipi.clk)) {
    dev_err(&pdev.dev, "failed to get clock\n");
    return PTR_ERR(mipi.clk);
    }
    platform_set_drvdata(pdev, mipi);
    return devm_tegra_mipi_add_provider(&pdev.dev, pdev.dev.of_node,
    &tegra114_mipi_ops);
    }
    struct platform_driver tegra_mipi_driver = {
    .driver = {
    .name = "tegra-mipi",
    .of_match_table = tegra_mipi_of_match,
    },
    .probe = tegra_mipi_probe,
    };
