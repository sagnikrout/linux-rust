//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-rt3883.c
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

pub const RT3883_GPIO_MODE_UART0_SHIFT: c_int = 2;
pub const RT3883_GPIO_MODE_UART0_MASK: c_uint = 0x7;

pub const RT3883_GPIO_MODE_UARTF: c_uint = 0x0;
pub const RT3883_GPIO_MODE_PCM_UARTF: c_uint = 0x1;
pub const RT3883_GPIO_MODE_PCM_I2S: c_uint = 0x2;
pub const RT3883_GPIO_MODE_I2S_UARTF: c_uint = 0x3;
pub const RT3883_GPIO_MODE_PCM_GPIO: c_uint = 0x4;
pub const RT3883_GPIO_MODE_GPIO_UARTF: c_uint = 0x5;
pub const RT3883_GPIO_MODE_GPIO_I2S: c_uint = 0x6;
pub const RT3883_GPIO_MODE_GPIO: c_uint = 0x7;
pub const RT3883_GPIO_MODE_I2C: c_int = 0;
pub const RT3883_GPIO_MODE_SPI: c_int = 1;
pub const RT3883_GPIO_MODE_UART1: c_int = 5;
pub const RT3883_GPIO_MODE_JTAG: c_int = 6;
pub const RT3883_GPIO_MODE_MDIO: c_int = 7;
pub const RT3883_GPIO_MODE_GE1: c_int = 9;
pub const RT3883_GPIO_MODE_GE2: c_int = 10;
pub const RT3883_GPIO_MODE_PCI_SHIFT: c_int = 11;
pub const RT3883_GPIO_MODE_PCI_MASK: c_uint = 0x7;

pub const RT3883_GPIO_MODE_LNA_A_SHIFT: c_int = 16;
pub const RT3883_GPIO_MODE_LNA_A_MASK: c_uint = 0x3;

pub const RT3883_GPIO_MODE_LNA_A_GPIO: c_uint = 0x3;

pub const RT3883_GPIO_MODE_LNA_G_SHIFT: c_int = 18;
pub const RT3883_GPIO_MODE_LNA_G_MASK: c_uint = 0x3;

pub const RT3883_GPIO_MODE_LNA_G_GPIO: c_uint = 0x3;

    static struct mtmips_pmx_func i2c_grp[] =  { FUNC("i2c", 0, 1, 2) };
    static struct mtmips_pmx_func spi_grp[] = { FUNC("spi", 0, 3, 4) };
    static struct mtmips_pmx_func uartf_grp[] = {
    FUNC("uartf", RT3883_GPIO_MODE_UARTF, 7, 8),
    FUNC("pcm uartf", RT3883_GPIO_MODE_PCM_UARTF, 7, 8),
    FUNC("pcm i2s", RT3883_GPIO_MODE_PCM_I2S, 7, 8),
    FUNC("i2s uartf", RT3883_GPIO_MODE_I2S_UARTF, 7, 8),
    FUNC("pcm gpio", RT3883_GPIO_MODE_PCM_GPIO, 11, 4),
    FUNC("gpio uartf", RT3883_GPIO_MODE_GPIO_UARTF, 7, 4),
    FUNC("gpio i2s", RT3883_GPIO_MODE_GPIO_I2S, 7, 4),
    };
    static struct mtmips_pmx_func uartlite_grp[] = { FUNC("uartlite", 0, 15, 2) };
    static struct mtmips_pmx_func jtag_grp[] = { FUNC("jtag", 0, 17, 5) };
    static struct mtmips_pmx_func mdio_grp[] = { FUNC("mdio", 0, 22, 2) };
    static struct mtmips_pmx_func lna_a_grp[] = { FUNC("lna a", 0, 32, 3) };
    static struct mtmips_pmx_func lna_g_grp[] = { FUNC("lna g", 0, 35, 3) };
    static struct mtmips_pmx_func pci_grp[] = {
    FUNC("pci-dev", 0, 40, 32),
    FUNC("pci-host2", 1, 40, 32),
    FUNC("pci-host1", 2, 40, 32),
    FUNC("pci-fnc", 3, 40, 32)
    };
    static struct mtmips_pmx_func ge1_grp[] = { FUNC("ge1", 0, 72, 12) };
    static struct mtmips_pmx_func ge2_grp[] = { FUNC("ge2", 0, 84, 12) };
    static struct mtmips_pmx_group rt3883_pinmux_data[] = {
    GRP("i2c", i2c_grp, 1, RT3883_GPIO_MODE_I2C),
    GRP("spi", spi_grp, 1, RT3883_GPIO_MODE_SPI),
    GRP("uartf", uartf_grp, RT3883_GPIO_MODE_UART0_MASK,
    RT3883_GPIO_MODE_UART0_SHIFT),
    GRP("uartlite", uartlite_grp, 1, RT3883_GPIO_MODE_UART1),
    GRP("jtag", jtag_grp, 1, RT3883_GPIO_MODE_JTAG),
    GRP("mdio", mdio_grp, 1, RT3883_GPIO_MODE_MDIO),
    GRP("lna a", lna_a_grp, 1, RT3883_GPIO_MODE_LNA_A),
    GRP("lna g", lna_g_grp, 1, RT3883_GPIO_MODE_LNA_G),
    GRP("pci", pci_grp, RT3883_GPIO_MODE_PCI_MASK,
    RT3883_GPIO_MODE_PCI_SHIFT),
    GRP("ge1", ge1_grp, 1, RT3883_GPIO_MODE_GE1),
    GRP("ge2", ge2_grp, 1, RT3883_GPIO_MODE_GE2),
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn rt3883_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int rt3883_pinctrl_probe(struct platform_device *pdev)
    {
    return mtmips_pinctrl_init(pdev, rt3883_pinmux_data);
    }
    static const struct of_device_id rt3883_pinctrl_match[] = {
    { .compatible = "ralink,rt3883-pinctrl" },
    { .compatible = "ralink,rt2880-pinmux" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt3883_pinctrl_match);
    static struct platform_driver rt3883_pinctrl_driver = {
    .probe = rt3883_pinctrl_probe,
    .driver = {
    .name = "rt3883-pinctrl",
    .of_match_table = rt3883_pinctrl_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn rt3883_pinctrl_init() -> int __init {
    static int __init rt3883_pinctrl_init(void)
    {
    return platform_driver_register(&rt3883_pinctrl_driver);
    }
    core_initcall_sync(rt3883_pinctrl_init);
    MODULE_DESCRIPTION("MediaTek RT3883 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MTK_PINCTRL");
