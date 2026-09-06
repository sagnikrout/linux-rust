//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-mt76x8.c
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

pub const MT76X8_GPIO_MODE_MASK: c_uint = 0x3;
pub const MT76X8_GPIO_MODE_P4LED_KN: c_int = 58;
pub const MT76X8_GPIO_MODE_P3LED_KN: c_int = 56;
pub const MT76X8_GPIO_MODE_P2LED_KN: c_int = 54;
pub const MT76X8_GPIO_MODE_P1LED_KN: c_int = 52;
pub const MT76X8_GPIO_MODE_P0LED_KN: c_int = 50;
pub const MT76X8_GPIO_MODE_WLED_KN: c_int = 48;
pub const MT76X8_GPIO_MODE_P4LED_AN: c_int = 42;
pub const MT76X8_GPIO_MODE_P3LED_AN: c_int = 40;
pub const MT76X8_GPIO_MODE_P2LED_AN: c_int = 38;
pub const MT76X8_GPIO_MODE_P1LED_AN: c_int = 36;
pub const MT76X8_GPIO_MODE_P0LED_AN: c_int = 34;
pub const MT76X8_GPIO_MODE_WLED_AN: c_int = 32;
pub const MT76X8_GPIO_MODE_PWM1: c_int = 30;
pub const MT76X8_GPIO_MODE_PWM0: c_int = 28;
pub const MT76X8_GPIO_MODE_UART2: c_int = 26;
pub const MT76X8_GPIO_MODE_UART1: c_int = 24;
pub const MT76X8_GPIO_MODE_I2C: c_int = 20;
pub const MT76X8_GPIO_MODE_REFCLK: c_int = 18;
pub const MT76X8_GPIO_MODE_PERST: c_int = 16;
pub const MT76X8_GPIO_MODE_WDT: c_int = 14;
pub const MT76X8_GPIO_MODE_SPI: c_int = 12;
pub const MT76X8_GPIO_MODE_SDMODE: c_int = 10;
pub const MT76X8_GPIO_MODE_UART0: c_int = 8;
pub const MT76X8_GPIO_MODE_I2S: c_int = 6;
pub const MT76X8_GPIO_MODE_CS1: c_int = 4;
pub const MT76X8_GPIO_MODE_SPIS: c_int = 2;
pub const MT76X8_GPIO_MODE_GPIO: c_int = 0;
    static struct mtmips_pmx_func pwm1_grp[] = {
    FUNC("sdxc d6", 3, 19, 1),
    FUNC("pwm1 utif", 2, 19, 1),
    FUNC("pwm1", 0, 19, 1),
    };
    static struct mtmips_pmx_func pwm0_grp[] = {
    FUNC("sdxc d7", 3, 18, 1),
    FUNC("pwm0 utif", 2, 18, 1),
    FUNC("pwm0", 0, 18, 1),
    };
    static struct mtmips_pmx_func uart2_grp[] = {
    FUNC("sdxc d5 d4", 3, 20, 2),
    FUNC("uart2 pwm", 2, 20, 2),
    FUNC("uart2", 0, 20, 2),
    };
    static struct mtmips_pmx_func uart1_grp[] = {
    FUNC("sw_r", 3, 45, 2),
    FUNC("uart1 pwm", 2, 45, 2),
    FUNC("uart1", 0, 45, 2),
    };
    static struct mtmips_pmx_func i2c_grp[] = {
    FUNC("debug", 2, 4, 2),
    FUNC("i2c", 0, 4, 2),
    };
    static struct mtmips_pmx_func refclk_grp[] = { FUNC("refclk", 0, 37, 1) };
    static struct mtmips_pmx_func perst_grp[] = { FUNC("perst", 0, 36, 1) };
    static struct mtmips_pmx_func wdt_grp[] = { FUNC("wdt", 0, 38, 1) };
    static struct mtmips_pmx_func spi_grp[] = { FUNC("spi", 0, 7, 4) };
    static struct mtmips_pmx_func sd_mode_grp[] = {
    FUNC("sdxc jtag", 3, 22, 8),
    FUNC("sdxc utif", 2, 22, 8),
    FUNC("sdxc", 0, 22, 8),
    };
    static struct mtmips_pmx_func uart0_grp[] = {
    FUNC("uart0", 0, 12, 2),
    };
    static struct mtmips_pmx_func i2s_grp[] = {
    FUNC("antenna", 3, 0, 4),
    FUNC("pcm", 2, 0, 4),
    FUNC("i2s", 0, 0, 4),
    };
    static struct mtmips_pmx_func spi_cs1_grp[] = {
    FUNC("spi refclk", 2, 6, 1),
    FUNC("spi cs1", 0, 6, 1),
    };
    static struct mtmips_pmx_func spis_grp[] = {
    FUNC("pwm_uart2", 3, 14, 4),
    FUNC("spis utif", 2, 14, 4),
    FUNC("spis", 0, 14, 4),
    };
    static struct mtmips_pmx_func gpio_grp[] = {
    FUNC("pcie", 3, 11, 1),
    FUNC("gpio refclk", 2, 11, 1),
    };
    static struct mtmips_pmx_func p4led_kn_grp[] = {
    FUNC("p4led_kn jtag", 3, 30, 1),
    FUNC("p4led_kn utif", 2, 30, 1),
    FUNC("p4led_kn", 0, 30, 1),
    };
    static struct mtmips_pmx_func p3led_kn_grp[] = {
    FUNC("p3led_kn jtag", 3, 31, 1),
    FUNC("p3led_kn utif", 2, 31, 1),
    FUNC("p3led_kn", 0, 31, 1),
    };
    static struct mtmips_pmx_func p2led_kn_grp[] = {
    FUNC("p2led_kn jtag", 3, 32, 1),
    FUNC("p2led_kn utif", 2, 32, 1),
    FUNC("p2led_kn", 0, 32, 1),
    };
    static struct mtmips_pmx_func p1led_kn_grp[] = {
    FUNC("p1led_kn jtag", 3, 33, 1),
    FUNC("p1led_kn utif", 2, 33, 1),
    FUNC("p1led_kn", 0, 33, 1),
    };
    static struct mtmips_pmx_func p0led_kn_grp[] = {
    FUNC("p0led_kn jtag", 3, 34, 1),
    FUNC("p0led_kn", 0, 34, 1),
    };
    static struct mtmips_pmx_func wled_kn_grp[] = {
    FUNC("wled_kn", 0, 35, 1),
    };
    static struct mtmips_pmx_func p4led_an_grp[] = {
    FUNC("p4led_an jtag", 3, 39, 1),
    FUNC("p4led_an utif", 2, 39, 1),
    FUNC("p4led_an", 0, 39, 1),
    };
    static struct mtmips_pmx_func p3led_an_grp[] = {
    FUNC("p3led_an jtag", 3, 40, 1),
    FUNC("p3led_an utif", 2, 40, 1),
    FUNC("p3led_an", 0, 40, 1),
    };
    static struct mtmips_pmx_func p2led_an_grp[] = {
    FUNC("p2led_an jtag", 3, 41, 1),
    FUNC("p2led_an utif", 2, 41, 1),
    FUNC("p2led_an", 0, 41, 1),
    };
    static struct mtmips_pmx_func p1led_an_grp[] = {
    FUNC("p1led_an jtag", 3, 42, 1),
    FUNC("p1led_an utif", 2, 42, 1),
    FUNC("p1led_an", 0, 42, 1),
    };
    static struct mtmips_pmx_func p0led_an_grp[] = {
    FUNC("p0led_an jtag", 3, 43, 1),
    FUNC("p0led_an", 0, 43, 1),
    };
    static struct mtmips_pmx_func wled_an_grp[] = {
    FUNC("wled_an", 0, 44, 1),
    };
    static struct mtmips_pmx_group mt76x8_pinmux_data[] = {
    GRP_G("pwm1", pwm1_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_PWM1),
    GRP_G("pwm0", pwm0_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_PWM0),
    GRP_G("uart2", uart2_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_UART2),
    GRP_G("uart1", uart1_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_UART1),
    GRP_G("i2c", i2c_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_I2C),
    GRP("refclk", refclk_grp, 1, MT76X8_GPIO_MODE_REFCLK),
    GRP("perst", perst_grp, 1, MT76X8_GPIO_MODE_PERST),
    GRP("wdt", wdt_grp, 1, MT76X8_GPIO_MODE_WDT),
    GRP("spi", spi_grp, 1, MT76X8_GPIO_MODE_SPI),
    GRP_G("sdmode", sd_mode_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_SDMODE),
    GRP_G("uart0", uart0_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_UART0),
    GRP_G("i2s", i2s_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_I2S),
    GRP_G("spi cs1", spi_cs1_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_CS1),
    GRP_G("spis", spis_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_SPIS),
    GRP_G("gpio", gpio_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_GPIO),
    GRP_G("wled_an", wled_an_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_WLED_AN),
    GRP_G("p0led_an", p0led_an_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P0LED_AN),
    GRP_G("p1led_an", p1led_an_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P1LED_AN),
    GRP_G("p2led_an", p2led_an_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P2LED_AN),
    GRP_G("p3led_an", p3led_an_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P3LED_AN),
    GRP_G("p4led_an", p4led_an_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P4LED_AN),
    GRP_G("wled_kn", wled_kn_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_WLED_KN),
    GRP_G("p0led_kn", p0led_kn_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P0LED_KN),
    GRP_G("p1led_kn", p1led_kn_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P1LED_KN),
    GRP_G("p2led_kn", p2led_kn_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P2LED_KN),
    GRP_G("p3led_kn", p3led_kn_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P3LED_KN),
    GRP_G("p4led_kn", p4led_kn_grp, MT76X8_GPIO_MODE_MASK,
    1, MT76X8_GPIO_MODE_P4LED_KN),
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn mt76x8_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int mt76x8_pinctrl_probe(struct platform_device *pdev)
    {
    return mtmips_pinctrl_init(pdev, mt76x8_pinmux_data);
    }
    static const struct of_device_id mt76x8_pinctrl_match[] = {
    { .compatible = "ralink,mt76x8-pinctrl" },
    { .compatible = "ralink,mt7620-pinctrl" },
    { .compatible = "ralink,rt2880-pinmux" },
    {}
    };
    MODULE_DEVICE_TABLE(of, mt76x8_pinctrl_match);
    static struct platform_driver mt76x8_pinctrl_driver = {
    .probe = mt76x8_pinctrl_probe,
    .driver = {
    .name = "mt76x8-pinctrl",
    .of_match_table = mt76x8_pinctrl_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn mt76x8_pinctrl_init() -> int __init {
    static int __init mt76x8_pinctrl_init(void)
    {
    return platform_driver_register(&mt76x8_pinctrl_driver);
    }
    core_initcall_sync(mt76x8_pinctrl_init);
    MODULE_DESCRIPTION("MediaTek MT76X8 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MTK_PINCTRL");
