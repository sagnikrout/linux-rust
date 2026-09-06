//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/vt8500/pinctrl-wm8750.c
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
// Pinctrl data for Wondermedia WM8750 SoC
//
// Copyright (c) 2013 Tony Prisk <linux@prisktech.co.nz>
//

//
// Describe the register offsets within the GPIO memory space
// The dedicated external GPIO's should always be listed in bank 0
// so they are exported in the 0..31 range which is what users
// expect.
//
// Do not reorder these banks as it will change the pin numbering
//
    static const struct wmt_pinctrl_bank_registers wm8750_banks[] = {
    WMT_PINCTRL_BANK(0x40, 0x80, 0xC0, 0x00, 0x480, 0x4C0),	/* 0 */
    WMT_PINCTRL_BANK(0x44, 0x84, 0xC4, 0x04, 0x484, 0x4C4),	/* 1 */
    WMT_PINCTRL_BANK(0x48, 0x88, 0xC8, 0x08, 0x488, 0x4C8),	/* 2 */
    WMT_PINCTRL_BANK(0x4C, 0x8C, 0xCC, 0x0C, 0x48C, 0x4CC),	/* 3 */
    WMT_PINCTRL_BANK(0x50, 0x90, 0xD0, 0x10, 0x490, 0x4D0),	/* 4 */
    WMT_PINCTRL_BANK(0x54, 0x94, 0xD4, 0x14, 0x494, 0x4D4),	/* 5 */
    WMT_PINCTRL_BANK(0x58, 0x98, 0xD8, 0x18, 0x498, 0x4D8),	/* 6 */
    WMT_PINCTRL_BANK(0x5C, 0x9C, 0xDC, 0x1C, 0x49C, 0x4DC),	/* 7 */
    WMT_PINCTRL_BANK(0x60, 0xA0, 0xE0, 0x20, 0x4A0, 0x4E0),	/* 8 */
    WMT_PINCTRL_BANK(0x70, 0xB0, 0xF0, 0x30, 0x4B0, 0x4F0),	/* 9 */
    WMT_PINCTRL_BANK(0x7C, 0xBC, 0xDC, 0x3C, 0x4BC, 0x4FC),	/* 10 */
    };
// Please keep sorted by bank/bit

    static const struct pinctrl_pin_desc wm8750_pins[] = {
    PINCTRL_PIN(WMT_PIN_EXTGPIO0, "extgpio0"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO1, "extgpio1"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO2, "extgpio2"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO3, "extgpio3"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO4, "extgpio4"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO5, "extgpio5"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO6, "extgpio6"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO7, "extgpio7"),
    PINCTRL_PIN(WMT_PIN_WAKEUP0, "wakeup0"),
    PINCTRL_PIN(WMT_PIN_WAKEUP1, "wakeup1"),
    PINCTRL_PIN(WMT_PIN_SD0CD, "sd0_cd"),
    PINCTRL_PIN(WMT_PIN_VDOUT0, "vdout0"),
    PINCTRL_PIN(WMT_PIN_VDOUT1, "vdout1"),
    PINCTRL_PIN(WMT_PIN_VDOUT2, "vdout2"),
    PINCTRL_PIN(WMT_PIN_VDOUT3, "vdout3"),
    PINCTRL_PIN(WMT_PIN_VDOUT4, "vdout4"),
    PINCTRL_PIN(WMT_PIN_VDOUT5, "vdout5"),
    PINCTRL_PIN(WMT_PIN_VDOUT6, "vdout6"),
    PINCTRL_PIN(WMT_PIN_VDOUT7, "vdout7"),
    PINCTRL_PIN(WMT_PIN_VDOUT8, "vdout8"),
    PINCTRL_PIN(WMT_PIN_VDOUT9, "vdout9"),
    PINCTRL_PIN(WMT_PIN_VDOUT10, "vdout10"),
    PINCTRL_PIN(WMT_PIN_VDOUT11, "vdout11"),
    PINCTRL_PIN(WMT_PIN_VDOUT12, "vdout12"),
    PINCTRL_PIN(WMT_PIN_VDOUT13, "vdout13"),
    PINCTRL_PIN(WMT_PIN_VDOUT14, "vdout14"),
    PINCTRL_PIN(WMT_PIN_VDOUT15, "vdout15"),
    PINCTRL_PIN(WMT_PIN_VDOUT16, "vdout16"),
    PINCTRL_PIN(WMT_PIN_VDOUT17, "vdout17"),
    PINCTRL_PIN(WMT_PIN_VDOUT18, "vdout18"),
    PINCTRL_PIN(WMT_PIN_VDOUT19, "vdout19"),
    PINCTRL_PIN(WMT_PIN_VDOUT20, "vdout20"),
    PINCTRL_PIN(WMT_PIN_VDOUT21, "vdout21"),
    PINCTRL_PIN(WMT_PIN_VDOUT22, "vdout22"),
    PINCTRL_PIN(WMT_PIN_VDOUT23, "vdout23"),
    PINCTRL_PIN(WMT_PIN_VDIN0, "vdin0"),
    PINCTRL_PIN(WMT_PIN_VDIN1, "vdin1"),
    PINCTRL_PIN(WMT_PIN_VDIN2, "vdin2"),
    PINCTRL_PIN(WMT_PIN_VDIN3, "vdin3"),
    PINCTRL_PIN(WMT_PIN_VDIN4, "vdin4"),
    PINCTRL_PIN(WMT_PIN_VDIN5, "vdin5"),
    PINCTRL_PIN(WMT_PIN_VDIN6, "vdin6"),
    PINCTRL_PIN(WMT_PIN_VDIN7, "vdin7"),
    PINCTRL_PIN(WMT_PIN_SPI0_MOSI, "spi0_mosi"),
    PINCTRL_PIN(WMT_PIN_SPI0_MISO, "spi0_miso"),
    PINCTRL_PIN(WMT_PIN_SPI0_SS, "spi0_ss"),
    PINCTRL_PIN(WMT_PIN_SPI0_CLK, "spi0_clk"),
    PINCTRL_PIN(WMT_PIN_SPI0_SSB, "spi0_ssb"),
    PINCTRL_PIN(WMT_PIN_SD0CLK, "sd0_clk"),
    PINCTRL_PIN(WMT_PIN_SD0CMD, "sd0_cmd"),
    PINCTRL_PIN(WMT_PIN_SD0WP, "sd0_wp"),
    PINCTRL_PIN(WMT_PIN_SD0DATA0, "sd0_data0"),
    PINCTRL_PIN(WMT_PIN_SD0DATA1, "sd0_data1"),
    PINCTRL_PIN(WMT_PIN_SD0DATA2, "sd0_data2"),
    PINCTRL_PIN(WMT_PIN_SD0DATA3, "sd0_data3"),
    PINCTRL_PIN(WMT_PIN_SD1DATA0, "sd1_data0"),
    PINCTRL_PIN(WMT_PIN_SD1DATA1, "sd1_data1"),
    PINCTRL_PIN(WMT_PIN_SD1DATA2, "sd1_data2"),
    PINCTRL_PIN(WMT_PIN_SD1DATA3, "sd1_data3"),
    PINCTRL_PIN(WMT_PIN_SD1DATA4, "sd1_data4"),
    PINCTRL_PIN(WMT_PIN_SD1DATA5, "sd1_data5"),
    PINCTRL_PIN(WMT_PIN_SD1DATA6, "sd1_data6"),
    PINCTRL_PIN(WMT_PIN_SD1DATA7, "sd1_data7"),
    PINCTRL_PIN(WMT_PIN_I2C0_SCL, "i2c0_scl"),
    PINCTRL_PIN(WMT_PIN_I2C0_SDA, "i2c0_sda"),
    PINCTRL_PIN(WMT_PIN_I2C1_SCL, "i2c1_scl"),
    PINCTRL_PIN(WMT_PIN_I2C1_SDA, "i2c1_sda"),
    PINCTRL_PIN(WMT_PIN_I2C2_SCL, "i2c2_scl"),
    PINCTRL_PIN(WMT_PIN_I2C2_SDA, "i2c2_sda"),
    PINCTRL_PIN(WMT_PIN_UART0_RTS, "uart0_rts"),
    PINCTRL_PIN(WMT_PIN_UART0_TXD, "uart0_txd"),
    PINCTRL_PIN(WMT_PIN_UART0_CTS, "uart0_cts"),
    PINCTRL_PIN(WMT_PIN_UART0_RXD, "uart0_rxd"),
    PINCTRL_PIN(WMT_PIN_UART1_RTS, "uart1_rts"),
    PINCTRL_PIN(WMT_PIN_UART1_TXD, "uart1_txd"),
    PINCTRL_PIN(WMT_PIN_UART1_CTS, "uart1_cts"),
    PINCTRL_PIN(WMT_PIN_UART1_RXD, "uart1_rxd"),
    PINCTRL_PIN(WMT_PIN_UART2_RTS, "uart2_rts"),
    PINCTRL_PIN(WMT_PIN_UART2_TXD, "uart2_txd"),
    PINCTRL_PIN(WMT_PIN_UART2_CTS, "uart2_cts"),
    PINCTRL_PIN(WMT_PIN_UART2_RXD, "uart2_rxd"),
    PINCTRL_PIN(WMT_PIN_UART3_RTS, "uart3_rts"),
    PINCTRL_PIN(WMT_PIN_UART3_TXD, "uart3_txd"),
    PINCTRL_PIN(WMT_PIN_UART3_CTS, "uart3_cts"),
    PINCTRL_PIN(WMT_PIN_UART3_RXD, "uart3_rxd"),
    PINCTRL_PIN(WMT_PIN_SD2CD, "sd2_cd"),
    PINCTRL_PIN(WMT_PIN_SD2DATA3, "sd2_data3"),
    PINCTRL_PIN(WMT_PIN_SD2DATA0, "sd2_data0"),
    PINCTRL_PIN(WMT_PIN_SD2WP, "sd2_wp"),
    PINCTRL_PIN(WMT_PIN_SD2DATA1, "sd2_data1"),
    PINCTRL_PIN(WMT_PIN_SD2DATA2, "sd2_data2"),
    PINCTRL_PIN(WMT_PIN_SD2CMD, "sd2_cmd"),
    PINCTRL_PIN(WMT_PIN_SD2CLK, "sd2_clk"),
    PINCTRL_PIN(WMT_PIN_SD2PWR, "sd2_pwr"),
    PINCTRL_PIN(WMT_PIN_SD1CLK, "sd1_clk"),
    PINCTRL_PIN(WMT_PIN_SD1CMD, "sd1_cmd"),
    PINCTRL_PIN(WMT_PIN_SD1PWR, "sd1_pwr"),
    PINCTRL_PIN(WMT_PIN_SD1WP, "sd1_wp"),
    PINCTRL_PIN(WMT_PIN_SD1CD, "sd1_cd"),
    PINCTRL_PIN(WMT_PIN_SPI0SS3, "spi0_ss3"),
    PINCTRL_PIN(WMT_PIN_SPI0SS2, "spi0_ss2"),
    PINCTRL_PIN(WMT_PIN_PWMOUT1, "pwmout1"),
    PINCTRL_PIN(WMT_PIN_PWMOUT0, "pwmout0"),
    };
// Order of these names must match the above list
    static const char * const wm8750_groups[] = {
    "extgpio0",
    "extgpio1",
    "extgpio2",
    "extgpio3",
    "extgpio4",
    "extgpio5",
    "extgpio6",
    "extgpio7",
    "wakeup0",
    "wakeup1",
    "sd0_cd",
    "vdout0",
    "vdout1",
    "vdout2",
    "vdout3",
    "vdout4",
    "vdout5",
    "vdout6",
    "vdout7",
    "vdout8",
    "vdout9",
    "vdout10",
    "vdout11",
    "vdout12",
    "vdout13",
    "vdout14",
    "vdout15",
    "vdout16",
    "vdout17",
    "vdout18",
    "vdout19",
    "vdout20",
    "vdout21",
    "vdout22",
    "vdout23",
    "vdin0",
    "vdin1",
    "vdin2",
    "vdin3",
    "vdin4",
    "vdin5",
    "vdin6",
    "vdin7",
    "spi0_mosi",
    "spi0_miso",
    "spi0_ss",
    "spi0_clk",
    "spi0_ssb",
    "sd0_clk",
    "sd0_cmd",
    "sd0_wp",
    "sd0_data0",
    "sd0_data1",
    "sd0_data2",
    "sd0_data3",
    "sd1_data0",
    "sd1_data1",
    "sd1_data2",
    "sd1_data3",
    "sd1_data4",
    "sd1_data5",
    "sd1_data6",
    "sd1_data7",
    "i2c0_scl",
    "i2c0_sda",
    "i2c1_scl",
    "i2c1_sda",
    "i2c2_scl",
    "i2c2_sda",
    "uart0_rts",
    "uart0_txd",
    "uart0_cts",
    "uart0_rxd",
    "uart1_rts",
    "uart1_txd",
    "uart1_cts",
    "uart1_rxd",
    "uart2_rts",
    "uart2_txd",
    "uart2_cts",
    "uart2_rxd",
    "uart3_rts",
    "uart3_txd",
    "uart3_cts",
    "uart3_rxd",
    "sd2_cd",
    "sd2_data3",
    "sd2_data0",
    "sd2_wp",
    "sd2_data1",
    "sd2_data2",
    "sd2_cmd",
    "sd2_clk",
    "sd2_pwr",
    "sd1_clk",
    "sd1_cmd",
    "sd1_pwr",
    "sd1_wp",
    "sd1_cd",
    "spi0_ss3",
    "spi0_ss2",
    "pwmout1",
    "pwmout0",
    };
#[no_mangle]
unsafe extern "C" fn wm8750_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int wm8750_pinctrl_probe(struct platform_device *pdev)
    {
    struct wmt_pinctrl_data *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.banks = wm8750_banks;
    data.nbanks = ARRAY_SIZE(wm8750_banks);
    data.pins = wm8750_pins;
    data.npins = ARRAY_SIZE(wm8750_pins);
    data.groups = wm8750_groups;
    data.ngroups = ARRAY_SIZE(wm8750_groups);
    return wmt_pinctrl_probe(pdev, data);
    }
    static const struct of_device_id wmt_pinctrl_of_match[] = {
    { .compatible = "wm,wm8750-pinctrl" },
    { /* sentinel */ },
    };
    static struct platform_driver wmt_pinctrl_driver = {
    .probe	= wm8750_pinctrl_probe,
    .driver = {
    .name	= "pinctrl-wm8750",
    .of_match_table	= wmt_pinctrl_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(wmt_pinctrl_driver);
