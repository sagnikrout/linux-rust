//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-mt7621.c
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

pub const MT7621_GPIO_MODE_UART1: c_int = 1;
pub const MT7621_GPIO_MODE_I2C: c_int = 2;
pub const MT7621_GPIO_MODE_UART3_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_UART3_SHIFT: c_int = 3;
pub const MT7621_GPIO_MODE_UART3_GPIO: c_int = 1;
pub const MT7621_GPIO_MODE_UART2_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_UART2_SHIFT: c_int = 5;
pub const MT7621_GPIO_MODE_UART2_GPIO: c_int = 1;
pub const MT7621_GPIO_MODE_JTAG: c_int = 7;
pub const MT7621_GPIO_MODE_WDT_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_WDT_SHIFT: c_int = 8;
pub const MT7621_GPIO_MODE_WDT_GPIO: c_int = 1;
pub const MT7621_GPIO_MODE_PCIE_RST: c_int = 0;
pub const MT7621_GPIO_MODE_PCIE_REF: c_int = 2;
pub const MT7621_GPIO_MODE_PCIE_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_PCIE_SHIFT: c_int = 10;
pub const MT7621_GPIO_MODE_PCIE_GPIO: c_int = 1;
pub const MT7621_GPIO_MODE_MDIO_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_MDIO_SHIFT: c_int = 12;
pub const MT7621_GPIO_MODE_MDIO_GPIO: c_int = 1;
pub const MT7621_GPIO_MODE_RGMII1: c_int = 14;
pub const MT7621_GPIO_MODE_RGMII2: c_int = 15;
pub const MT7621_GPIO_MODE_SPI_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_SPI_SHIFT: c_int = 16;
pub const MT7621_GPIO_MODE_SPI_GPIO: c_int = 1;
pub const MT7621_GPIO_MODE_SDHCI_MASK: c_uint = 0x3;
pub const MT7621_GPIO_MODE_SDHCI_SHIFT: c_int = 18;
pub const MT7621_GPIO_MODE_SDHCI_GPIO: c_int = 1;
    static struct mtmips_pmx_func uart1_grp[] =  { FUNC("uart1", 0, 1, 2) };
    static struct mtmips_pmx_func i2c_grp[] =  { FUNC("i2c", 0, 3, 2) };
    static struct mtmips_pmx_func uart3_grp[] = {
    FUNC("uart3", 0, 5, 4),
    FUNC("i2s", 2, 5, 4),
    FUNC("spdif3", 3, 5, 4),
    };
    static struct mtmips_pmx_func uart2_grp[] = {
    FUNC("uart2", 0, 9, 4),
    FUNC("pcm", 2, 9, 4),
    FUNC("spdif2", 3, 9, 4),
    };
    static struct mtmips_pmx_func jtag_grp[] = { FUNC("jtag", 0, 13, 5) };
    static struct mtmips_pmx_func wdt_grp[] = {
    FUNC("wdt rst", 0, 18, 1),
    FUNC("wdt refclk", 2, 18, 1),
    };
    static struct mtmips_pmx_func pcie_rst_grp[] = {
    FUNC("pcie rst", MT7621_GPIO_MODE_PCIE_RST, 19, 1),
    FUNC("pcie refclk", MT7621_GPIO_MODE_PCIE_REF, 19, 1)
    };
    static struct mtmips_pmx_func mdio_grp[] = { FUNC("mdio", 0, 20, 2) };
    static struct mtmips_pmx_func rgmii2_grp[] = { FUNC("rgmii2", 0, 22, 12) };
    static struct mtmips_pmx_func spi_grp[] = {
    FUNC("spi", 0, 34, 7),
    FUNC("nand1", 2, 34, 7),
    };
    static struct mtmips_pmx_func sdhci_grp[] = {
    FUNC("sdhci", 0, 41, 8),
    FUNC("nand2", 2, 41, 8),
    };
    static struct mtmips_pmx_func rgmii1_grp[] = { FUNC("rgmii1", 0, 49, 12) };
    static struct mtmips_pmx_group mt7621_pinmux_data[] = {
    GRP("uart1", uart1_grp, 1, MT7621_GPIO_MODE_UART1),
    GRP("i2c", i2c_grp, 1, MT7621_GPIO_MODE_I2C),
    GRP_G("uart3", uart3_grp, MT7621_GPIO_MODE_UART3_MASK,
    MT7621_GPIO_MODE_UART3_GPIO, MT7621_GPIO_MODE_UART3_SHIFT),
    GRP_G("uart2", uart2_grp, MT7621_GPIO_MODE_UART2_MASK,
    MT7621_GPIO_MODE_UART2_GPIO, MT7621_GPIO_MODE_UART2_SHIFT),
    GRP("jtag", jtag_grp, 1, MT7621_GPIO_MODE_JTAG),
    GRP_G("wdt", wdt_grp, MT7621_GPIO_MODE_WDT_MASK,
    MT7621_GPIO_MODE_WDT_GPIO, MT7621_GPIO_MODE_WDT_SHIFT),
    GRP_G("pcie", pcie_rst_grp, MT7621_GPIO_MODE_PCIE_MASK,
    MT7621_GPIO_MODE_PCIE_GPIO, MT7621_GPIO_MODE_PCIE_SHIFT),
    GRP_G("mdio", mdio_grp, MT7621_GPIO_MODE_MDIO_MASK,
    MT7621_GPIO_MODE_MDIO_GPIO, MT7621_GPIO_MODE_MDIO_SHIFT),
    GRP("rgmii2", rgmii2_grp, 1, MT7621_GPIO_MODE_RGMII2),
    GRP_G("spi", spi_grp, MT7621_GPIO_MODE_SPI_MASK,
    MT7621_GPIO_MODE_SPI_GPIO, MT7621_GPIO_MODE_SPI_SHIFT),
    GRP_G("sdhci", sdhci_grp, MT7621_GPIO_MODE_SDHCI_MASK,
    MT7621_GPIO_MODE_SDHCI_GPIO, MT7621_GPIO_MODE_SDHCI_SHIFT),
    GRP("rgmii1", rgmii1_grp, 1, MT7621_GPIO_MODE_RGMII1),
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn mt7621_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int mt7621_pinctrl_probe(struct platform_device *pdev)
    {
    return mtmips_pinctrl_init(pdev, mt7621_pinmux_data);
    }
    static const struct of_device_id mt7621_pinctrl_match[] = {
    { .compatible = "ralink,mt7621-pinctrl" },
    { .compatible = "ralink,rt2880-pinmux" },
    {}
    };
    MODULE_DEVICE_TABLE(of, mt7621_pinctrl_match);
    static struct platform_driver mt7621_pinctrl_driver = {
    .probe = mt7621_pinctrl_probe,
    .driver = {
    .name = "mt7621-pinctrl",
    .of_match_table = mt7621_pinctrl_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn mt7621_pinctrl_init() -> int __init {
    static int __init mt7621_pinctrl_init(void)
    {
    return platform_driver_register(&mt7621_pinctrl_driver);
    }
    core_initcall_sync(mt7621_pinctrl_init);
    MODULE_DESCRIPTION("MediaTek MT7621 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MTK_PINCTRL");
