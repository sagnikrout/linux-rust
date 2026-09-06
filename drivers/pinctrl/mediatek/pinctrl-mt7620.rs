//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-mt7620.c
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

pub const MT7620_GPIO_MODE_UART0_SHIFT: c_int = 2;
pub const MT7620_GPIO_MODE_UART0_MASK: c_uint = 0x7;

pub const MT7620_GPIO_MODE_UARTF: c_uint = 0x0;
pub const MT7620_GPIO_MODE_PCM_UARTF: c_uint = 0x1;
pub const MT7620_GPIO_MODE_PCM_I2S: c_uint = 0x2;
pub const MT7620_GPIO_MODE_I2S_UARTF: c_uint = 0x3;
pub const MT7620_GPIO_MODE_PCM_GPIO: c_uint = 0x4;
pub const MT7620_GPIO_MODE_GPIO_UARTF: c_uint = 0x5;
pub const MT7620_GPIO_MODE_GPIO_I2S: c_uint = 0x6;
pub const MT7620_GPIO_MODE_GPIO: c_uint = 0x7;
pub const MT7620_GPIO_MODE_NAND: c_int = 0;
pub const MT7620_GPIO_MODE_SD: c_int = 1;
pub const MT7620_GPIO_MODE_ND_SD_GPIO: c_int = 2;
pub const MT7620_GPIO_MODE_ND_SD_MASK: c_uint = 0x3;
pub const MT7620_GPIO_MODE_ND_SD_SHIFT: c_int = 18;
pub const MT7620_GPIO_MODE_PCIE_RST: c_int = 0;
pub const MT7620_GPIO_MODE_PCIE_REF: c_int = 1;
pub const MT7620_GPIO_MODE_PCIE_GPIO: c_int = 2;
pub const MT7620_GPIO_MODE_PCIE_MASK: c_uint = 0x3;
pub const MT7620_GPIO_MODE_PCIE_SHIFT: c_int = 16;
pub const MT7620_GPIO_MODE_WDT_RST: c_int = 0;
pub const MT7620_GPIO_MODE_WDT_REF: c_int = 1;
pub const MT7620_GPIO_MODE_WDT_GPIO: c_int = 2;
pub const MT7620_GPIO_MODE_WDT_MASK: c_uint = 0x3;
pub const MT7620_GPIO_MODE_WDT_SHIFT: c_int = 21;
pub const MT7620_GPIO_MODE_MDIO: c_int = 0;
pub const MT7620_GPIO_MODE_MDIO_REFCLK: c_int = 1;
pub const MT7620_GPIO_MODE_MDIO_GPIO: c_int = 2;
pub const MT7620_GPIO_MODE_MDIO_MASK: c_uint = 0x3;
pub const MT7620_GPIO_MODE_MDIO_SHIFT: c_int = 7;
pub const MT7620_GPIO_MODE_I2C: c_int = 0;
pub const MT7620_GPIO_MODE_UART1: c_int = 5;
pub const MT7620_GPIO_MODE_RGMII1: c_int = 9;
pub const MT7620_GPIO_MODE_RGMII2: c_int = 10;
pub const MT7620_GPIO_MODE_SPI: c_int = 11;
pub const MT7620_GPIO_MODE_SPI_REF_CLK: c_int = 12;
pub const MT7620_GPIO_MODE_WLED: c_int = 13;
pub const MT7620_GPIO_MODE_JTAG: c_int = 15;
pub const MT7620_GPIO_MODE_EPHY: c_int = 15;
pub const MT7620_GPIO_MODE_PA: c_int = 20;
    static struct mtmips_pmx_func i2c_grp[] =  { FUNC("i2c", 0, 1, 2) };
    static struct mtmips_pmx_func spi_grp[] = { FUNC("spi", 0, 3, 4) };
    static struct mtmips_pmx_func uartlite_grp[] = { FUNC("uartlite", 0, 15, 2) };
    static struct mtmips_pmx_func mdio_grp[] = {
    FUNC("mdio", MT7620_GPIO_MODE_MDIO, 22, 2),
    FUNC("refclk", MT7620_GPIO_MODE_MDIO_REFCLK, 22, 2),
    };
    static struct mtmips_pmx_func rgmii1_grp[] = { FUNC("rgmii1", 0, 24, 12) };
    static struct mtmips_pmx_func refclk_grp[] = { FUNC("spi refclk", 0, 37, 3) };
    static struct mtmips_pmx_func ephy_grp[] = { FUNC("ephy", 0, 40, 5) };
    static struct mtmips_pmx_func rgmii2_grp[] = { FUNC("rgmii2", 0, 60, 12) };
    static struct mtmips_pmx_func wled_grp[] = { FUNC("wled", 0, 72, 1) };
    static struct mtmips_pmx_func pa_grp[] = { FUNC("pa", 0, 18, 4) };
    static struct mtmips_pmx_func uartf_grp[] = {
    FUNC("uartf", MT7620_GPIO_MODE_UARTF, 7, 8),
    FUNC("pcm uartf", MT7620_GPIO_MODE_PCM_UARTF, 7, 8),
    FUNC("pcm i2s", MT7620_GPIO_MODE_PCM_I2S, 7, 8),
    FUNC("i2s uartf", MT7620_GPIO_MODE_I2S_UARTF, 7, 8),
    FUNC("pcm gpio", MT7620_GPIO_MODE_PCM_GPIO, 11, 4),
    FUNC("gpio uartf", MT7620_GPIO_MODE_GPIO_UARTF, 7, 4),
    FUNC("gpio i2s", MT7620_GPIO_MODE_GPIO_I2S, 7, 4),
    };
    static struct mtmips_pmx_func wdt_grp[] = {
    FUNC("wdt rst", 0, 17, 1),
    FUNC("wdt refclk", 0, 17, 1),
    };
    static struct mtmips_pmx_func pcie_rst_grp[] = {
    FUNC("pcie rst", MT7620_GPIO_MODE_PCIE_RST, 36, 1),
    FUNC("pcie refclk", MT7620_GPIO_MODE_PCIE_REF, 36, 1)
    };
    static struct mtmips_pmx_func nd_sd_grp[] = {
    FUNC("nand", MT7620_GPIO_MODE_NAND, 45, 15),
    FUNC("sd", MT7620_GPIO_MODE_SD, 47, 13)
    };
    static struct mtmips_pmx_group mt7620a_pinmux_data[] = {
    GRP("i2c", i2c_grp, 1, MT7620_GPIO_MODE_I2C),
    GRP("uartf", uartf_grp, MT7620_GPIO_MODE_UART0_MASK,
    MT7620_GPIO_MODE_UART0_SHIFT),
    GRP("spi", spi_grp, 1, MT7620_GPIO_MODE_SPI),
    GRP("uartlite", uartlite_grp, 1, MT7620_GPIO_MODE_UART1),
    GRP_G("wdt", wdt_grp, MT7620_GPIO_MODE_WDT_MASK,
    MT7620_GPIO_MODE_WDT_GPIO, MT7620_GPIO_MODE_WDT_SHIFT),
    GRP_G("mdio", mdio_grp, MT7620_GPIO_MODE_MDIO_MASK,
    MT7620_GPIO_MODE_MDIO_GPIO, MT7620_GPIO_MODE_MDIO_SHIFT),
    GRP("rgmii1", rgmii1_grp, 1, MT7620_GPIO_MODE_RGMII1),
    GRP("spi refclk", refclk_grp, 1, MT7620_GPIO_MODE_SPI_REF_CLK),
    GRP_G("pcie", pcie_rst_grp, MT7620_GPIO_MODE_PCIE_MASK,
    MT7620_GPIO_MODE_PCIE_GPIO, MT7620_GPIO_MODE_PCIE_SHIFT),
    GRP_G("nd_sd", nd_sd_grp, MT7620_GPIO_MODE_ND_SD_MASK,
    MT7620_GPIO_MODE_ND_SD_GPIO, MT7620_GPIO_MODE_ND_SD_SHIFT),
    GRP("rgmii2", rgmii2_grp, 1, MT7620_GPIO_MODE_RGMII2),
    GRP("wled", wled_grp, 1, MT7620_GPIO_MODE_WLED),
    GRP("ephy", ephy_grp, 1, MT7620_GPIO_MODE_EPHY),
    GRP("pa", pa_grp, 1, MT7620_GPIO_MODE_PA),
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn mt7620_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int mt7620_pinctrl_probe(struct platform_device *pdev)
    {
    return mtmips_pinctrl_init(pdev, mt7620a_pinmux_data);
    }
    static const struct of_device_id mt7620_pinctrl_match[] = {
    { .compatible = "ralink,mt7620-pinctrl" },
    { .compatible = "ralink,rt2880-pinmux" },
    {}
    };
    MODULE_DEVICE_TABLE(of, mt7620_pinctrl_match);
    static struct platform_driver mt7620_pinctrl_driver = {
    .probe = mt7620_pinctrl_probe,
    .driver = {
    .name = "mt7620-pinctrl",
    .of_match_table = mt7620_pinctrl_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn mt7620_pinctrl_init() -> int __init {
    static int __init mt7620_pinctrl_init(void)
    {
    return platform_driver_register(&mt7620_pinctrl_driver);
    }
    core_initcall_sync(mt7620_pinctrl_init);
    MODULE_DESCRIPTION("MediaTek MT7620 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MTK_PINCTRL");
