//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-rt2880.c
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

    static struct mtmips_pmx_func i2c_grp[] = { FUNC("i2c", 0, 1, 2) };
    static struct mtmips_pmx_func spi_grp[] = { FUNC("spi", 0, 3, 4) };
    static struct mtmips_pmx_func uartlite_grp[] = { FUNC("uartlite", 0, 7, 8) };
    static struct mtmips_pmx_func jtag_grp[] = { FUNC("jtag", 0, 17, 5) };
    static struct mtmips_pmx_func mdio_grp[] = { FUNC("mdio", 0, 22, 2) };
    static struct mtmips_pmx_func sdram_grp[] = { FUNC("sdram", 0, 24, 16) };
    static struct mtmips_pmx_func pci_grp[] = { FUNC("pci", 0, 40, 32) };
    static struct mtmips_pmx_group rt2880_pinmux_data_act[] = {
    GRP("i2c", i2c_grp, 1, RT2880_GPIO_MODE_I2C),
    GRP("spi", spi_grp, 1, RT2880_GPIO_MODE_SPI),
    GRP("uartlite", uartlite_grp, 1, RT2880_GPIO_MODE_UART0),
    GRP("jtag", jtag_grp, 1, RT2880_GPIO_MODE_JTAG),
    GRP("mdio", mdio_grp, 1, RT2880_GPIO_MODE_MDIO),
    GRP("sdram", sdram_grp, 1, RT2880_GPIO_MODE_SDRAM),
    GRP("pci", pci_grp, 1, RT2880_GPIO_MODE_PCI),
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn rt2880_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int rt2880_pinctrl_probe(struct platform_device *pdev)
    {
    return mtmips_pinctrl_init(pdev, rt2880_pinmux_data_act);
    }
    static const struct of_device_id rt2880_pinctrl_match[] = {
    { .compatible = "ralink,rt2880-pinctrl" },
    { .compatible = "ralink,rt2880-pinmux" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt2880_pinctrl_match);
    static struct platform_driver rt2880_pinctrl_driver = {
    .probe = rt2880_pinctrl_probe,
    .driver = {
    .name = "rt2880-pinctrl",
    .of_match_table = rt2880_pinctrl_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn rt2880_pinctrl_init() -> int __init {
    static int __init rt2880_pinctrl_init(void)
    {
    return platform_driver_register(&rt2880_pinctrl_driver);
    }
    core_initcall_sync(rt2880_pinctrl_init);
    MODULE_DESCRIPTION("MediaTek RT2880 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MTK_PINCTRL");
