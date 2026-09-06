//! Automatically rewritten from C to Rust
//! Source: drivers/clk/davinci/psc-da850.c
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
// PSC clock descriptions for TI DA850/OMAP-L138/AM18XX
//
// Copyright (C) 2018 David Lechner <david@lechnology.com>
//

    LPSC_CLKDEV1(emifa_clkdev,	core::ptr::null_mut(),		"ti-aemif");
    LPSC_CLKDEV1(spi0_clkdev,	core::ptr::null_mut(),		"spi_davinci.0");
    LPSC_CLKDEV1(mmcsd0_clkdev,	core::ptr::null_mut(),		"da830-mmc.0");
    LPSC_CLKDEV1(uart0_clkdev,	core::ptr::null_mut(),		"serial8250.0");
// REVISIT: used dev_id instead of con_id
    LPSC_CLKDEV1(arm_clkdev,	"arm",		core::ptr::null_mut());
    LPSC_CLKDEV1(dsp_clkdev,	core::ptr::null_mut(),		"davinci-rproc.0");
    static const struct davinci_lpsc_clk_info da850_psc0_info[] = {
    LPSC(0,  0, tpcc0,   pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(1,  0, tptc0,   pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(2,  0, tptc1,   pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(3,  0, emifa,   async1,       emifa_clkdev,  0),
    LPSC(4,  0, spi0,    pll0_sysclk2, spi0_clkdev,   0),
    LPSC(5,  0, mmcsd0,  pll0_sysclk2, mmcsd0_clkdev, 0),
    LPSC(6,  0, aintc,   pll0_sysclk4, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(7,  0, arm_rom, pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(9,  0, uart0,   pll0_sysclk2, uart0_clkdev,  0),
    LPSC(13, 0, pruss,   pll0_sysclk2, core::ptr::null_mut(),          0),
    LPSC(14, 0, arm,     pll0_sysclk6, arm_clkdev,    LPSC_ALWAYS_ENABLED | LPSC_SET_RATE_PARENT),
    LPSC(15, 1, dsp,     pll0_sysclk1, dsp_clkdev,    LPSC_FORCE | LPSC_LOCAL_RESET),
    { }
    };
    LPSC_CLKDEV3(usb0_clkdev,	"fck",	"da830-usb-phy-clks",
    core::ptr::null_mut(),	"musb-da8xx",
    core::ptr::null_mut(),	"cppi41-dmaengine");
    LPSC_CLKDEV1(usb1_clkdev,	core::ptr::null_mut(),	"ohci-da8xx");
// REVISIT: gpio-davinci.c should be modified to drop con_id
    LPSC_CLKDEV1(gpio_clkdev,	"gpio",	core::ptr::null_mut());
    LPSC_CLKDEV2(emac_clkdev,	core::ptr::null_mut(),	"davinci_emac.1",
    "fck",	"davinci_mdio.0");
    LPSC_CLKDEV1(mcasp0_clkdev,	core::ptr::null_mut(),	"davinci-mcasp.0");
    LPSC_CLKDEV1(sata_clkdev,	"fck",	"ahci_da850");
    LPSC_CLKDEV1(vpif_clkdev,	core::ptr::null_mut(),	"vpif");
    LPSC_CLKDEV1(spi1_clkdev,	core::ptr::null_mut(),	"spi_davinci.1");
    LPSC_CLKDEV1(i2c1_clkdev,	core::ptr::null_mut(),	"i2c_davinci.2");
    LPSC_CLKDEV1(uart1_clkdev,	core::ptr::null_mut(),	"serial8250.1");
    LPSC_CLKDEV1(uart2_clkdev,	core::ptr::null_mut(),	"serial8250.2");
    LPSC_CLKDEV1(mcbsp0_clkdev,	core::ptr::null_mut(),	"davinci-mcbsp.0");
    LPSC_CLKDEV1(mcbsp1_clkdev,	core::ptr::null_mut(),	"davinci-mcbsp.1");
    LPSC_CLKDEV1(lcdc_clkdev,	"fck",	"da8xx_lcdc.0");
    LPSC_CLKDEV3(ehrpwm_clkdev,	"fck",	"ehrpwm.0",
    "fck",	"ehrpwm.1",
    core::ptr::null_mut(),	"da830-tbclksync");
    LPSC_CLKDEV1(mmcsd1_clkdev,	core::ptr::null_mut(),	"da830-mmc.1");
    LPSC_CLKDEV3(ecap_clkdev,	"fck",	"ecap.0",
    "fck",	"ecap.1",
    "fck",	"ecap.2");
#[no_mangle]
unsafe extern "C" fn da850_psc0_init(dev: *mut device, base: *mut void __iomem) -> c_int {
    static int da850_psc0_init(struct device *dev, void __iomem *base)
    {
    return davinci_psc_register_clocks(dev, da850_psc0_info, 16, base);
    }
#[no_mangle]
unsafe extern "C" fn of_da850_psc0_init(dev: *mut device, base: *mut void __iomem) -> c_int {
    static int of_da850_psc0_init(struct device *dev, void __iomem *base)
    {
    return of_davinci_psc_clk_init(dev, da850_psc0_info, 16, base);
    }
    static struct clk_bulk_data da850_psc0_parent_clks[] = {
    { .id = "pll0_sysclk1" },
    { .id = "pll0_sysclk2" },
    { .id = "pll0_sysclk4" },
    { .id = "pll0_sysclk6" },
    { .id = "async1"       },
    };
    const struct davinci_psc_init_data da850_psc0_init_data = {
    .parent_clks		= da850_psc0_parent_clks,
    .num_parent_clks	= ARRAY_SIZE(da850_psc0_parent_clks),
    .psc_init		= &da850_psc0_init,
    };
    const struct davinci_psc_init_data of_da850_psc0_init_data = {
    .parent_clks		= da850_psc0_parent_clks,
    .num_parent_clks	= ARRAY_SIZE(da850_psc0_parent_clks),
    .psc_init		= &of_da850_psc0_init,
    };
    static const struct davinci_lpsc_clk_info da850_psc1_info[] = {
    LPSC(0,  0, tpcc1,  pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(1,  0, usb0,   pll0_sysclk2, usb0_clkdev,   0),
    LPSC(2,  0, usb1,   pll0_sysclk4, usb1_clkdev,   0),
    LPSC(3,  0, gpio,   pll0_sysclk4, gpio_clkdev,   0),
    LPSC(5,  0, emac,   pll0_sysclk4, emac_clkdev,   0),
    LPSC(6,  0, ddr,    pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    LPSC(7,  0, mcasp0, async3,       mcasp0_clkdev, 0),
    LPSC(8,  0, sata,   pll0_sysclk2, sata_clkdev,   LPSC_FORCE),
    LPSC(9,  0, vpif,   pll0_sysclk2, vpif_clkdev,   0),
    LPSC(10, 0, spi1,   async3,       spi1_clkdev,   0),
    LPSC(11, 0, i2c1,   pll0_sysclk4, i2c1_clkdev,   0),
    LPSC(12, 0, uart1,  async3,       uart1_clkdev,  0),
    LPSC(13, 0, uart2,  async3,       uart2_clkdev,  0),
    LPSC(14, 0, mcbsp0, async3,       mcbsp0_clkdev, 0),
    LPSC(15, 0, mcbsp1, async3,       mcbsp1_clkdev, 0),
    LPSC(16, 0, lcdc,   pll0_sysclk2, lcdc_clkdev,   0),
    LPSC(17, 0, ehrpwm, async3,       ehrpwm_clkdev, 0),
    LPSC(18, 0, mmcsd1, pll0_sysclk2, mmcsd1_clkdev, 0),
    LPSC(20, 0, ecap,   async3,       ecap_clkdev,   0),
    LPSC(21, 0, tptc2,  pll0_sysclk2, core::ptr::null_mut(),          LPSC_ALWAYS_ENABLED),
    { }
    };
#[no_mangle]
unsafe extern "C" fn da850_psc1_init(dev: *mut device, base: *mut void __iomem) -> c_int {
    static int da850_psc1_init(struct device *dev, void __iomem *base)
    {
    return davinci_psc_register_clocks(dev, da850_psc1_info, 32, base);
    }
#[no_mangle]
unsafe extern "C" fn of_da850_psc1_init(dev: *mut device, base: *mut void __iomem) -> c_int {
    static int of_da850_psc1_init(struct device *dev, void __iomem *base)
    {
    return of_davinci_psc_clk_init(dev, da850_psc1_info, 32, base);
    }
    static struct clk_bulk_data da850_psc1_parent_clks[] = {
    { .id = "pll0_sysclk2" },
    { .id = "pll0_sysclk4" },
    { .id = "async3"       },
    };
    const struct davinci_psc_init_data da850_psc1_init_data = {
    .parent_clks		= da850_psc1_parent_clks,
    .num_parent_clks	= ARRAY_SIZE(da850_psc1_parent_clks),
    .psc_init		= &da850_psc1_init,
    };
    const struct davinci_psc_init_data of_da850_psc1_init_data = {
    .parent_clks		= da850_psc1_parent_clks,
    .num_parent_clks	= ARRAY_SIZE(da850_psc1_parent_clks),
    .psc_init		= &of_da850_psc1_init,
    };
