//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/vt8500/pinctrl-wm8505.c
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
// Pinctrl data for Wondermedia WM8505 SoC
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
    static const struct wmt_pinctrl_bank_registers wm8505_banks[] = {
    WMT_PINCTRL_BANK(0x64, 0x8C, 0xB4, 0xDC, NO_REG, NO_REG),	/* 0 */
    WMT_PINCTRL_BANK(0x40, 0x68, 0x90, 0xB8, NO_REG, NO_REG),	/* 1 */
    WMT_PINCTRL_BANK(0x44, 0x6C, 0x94, 0xBC, NO_REG, NO_REG),	/* 2 */
    WMT_PINCTRL_BANK(0x48, 0x70, 0x98, 0xC0, NO_REG, NO_REG),	/* 3 */
    WMT_PINCTRL_BANK(0x4C, 0x74, 0x9C, 0xC4, NO_REG, NO_REG),	/* 4 */
    WMT_PINCTRL_BANK(0x50, 0x78, 0xA0, 0xC8, NO_REG, NO_REG),	/* 5 */
    WMT_PINCTRL_BANK(0x54, 0x7C, 0xA4, 0xD0, NO_REG, NO_REG),	/* 6 */
    WMT_PINCTRL_BANK(0x58, 0x80, 0xA8, 0xD4, NO_REG, NO_REG),	/* 7 */
    WMT_PINCTRL_BANK(0x5C, 0x84, 0xAC, 0xD8, NO_REG, NO_REG),	/* 8 */
    WMT_PINCTRL_BANK(0x60, 0x88, 0xB0, 0xDC, NO_REG, NO_REG),	/* 9 */
    WMT_PINCTRL_BANK(0x500, 0x504, 0x508, 0x50C, NO_REG, NO_REG),	/* 10 */
    };
// Please keep sorted by bank/bit

    static const struct pinctrl_pin_desc wm8505_pins[] = {
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
    PINCTRL_PIN(WMT_PIN_WAKEUP2, "wakeup2"),
    PINCTRL_PIN(WMT_PIN_WAKEUP3, "wakeup3"),
    PINCTRL_PIN(WMT_PIN_SUSGPIO0, "susgpio0"),
    PINCTRL_PIN(WMT_PIN_SDDATA0, "sd_data0"),
    PINCTRL_PIN(WMT_PIN_SDDATA1, "sd_data1"),
    PINCTRL_PIN(WMT_PIN_SDDATA2, "sd_data2"),
    PINCTRL_PIN(WMT_PIN_SDDATA3, "sd_data3"),
    PINCTRL_PIN(WMT_PIN_MMCDATA0, "mmc_data0"),
    PINCTRL_PIN(WMT_PIN_MMCDATA1, "mmc_data1"),
    PINCTRL_PIN(WMT_PIN_MMCDATA2, "mmc_data2"),
    PINCTRL_PIN(WMT_PIN_MMCDATA3, "mmc_data3"),
    PINCTRL_PIN(WMT_PIN_VDIN0, "vdin0"),
    PINCTRL_PIN(WMT_PIN_VDIN1, "vdin1"),
    PINCTRL_PIN(WMT_PIN_VDIN2, "vdin2"),
    PINCTRL_PIN(WMT_PIN_VDIN3, "vdin3"),
    PINCTRL_PIN(WMT_PIN_VDIN4, "vdin4"),
    PINCTRL_PIN(WMT_PIN_VDIN5, "vdin5"),
    PINCTRL_PIN(WMT_PIN_VDIN6, "vdin6"),
    PINCTRL_PIN(WMT_PIN_VDIN7, "vdin7"),
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
    PINCTRL_PIN(WMT_PIN_VHSYNC, "v_hsync"),
    PINCTRL_PIN(WMT_PIN_VVSYNC, "v_vsync"),
    PINCTRL_PIN(WMT_PIN_VGAHSYNC, "vga_hsync"),
    PINCTRL_PIN(WMT_PIN_VGAVSYNC, "vga_vsync"),
    PINCTRL_PIN(WMT_PIN_VDHSYNC, "vd_hsync"),
    PINCTRL_PIN(WMT_PIN_VDVSYNC, "vd_vsync"),
    PINCTRL_PIN(WMT_PIN_NORD0, "nor_d0"),
    PINCTRL_PIN(WMT_PIN_NORD1, "nor_d1"),
    PINCTRL_PIN(WMT_PIN_NORD2, "nor_d2"),
    PINCTRL_PIN(WMT_PIN_NORD3, "nor_d3"),
    PINCTRL_PIN(WMT_PIN_NORD4, "nor_d4"),
    PINCTRL_PIN(WMT_PIN_NORD5, "nor_d5"),
    PINCTRL_PIN(WMT_PIN_NORD6, "nor_d6"),
    PINCTRL_PIN(WMT_PIN_NORD7, "nor_d7"),
    PINCTRL_PIN(WMT_PIN_NORD8, "nor_d8"),
    PINCTRL_PIN(WMT_PIN_NORD9, "nor_d9"),
    PINCTRL_PIN(WMT_PIN_NORD10, "nor_d10"),
    PINCTRL_PIN(WMT_PIN_NORD11, "nor_d11"),
    PINCTRL_PIN(WMT_PIN_NORD12, "nor_d12"),
    PINCTRL_PIN(WMT_PIN_NORD13, "nor_d13"),
    PINCTRL_PIN(WMT_PIN_NORD14, "nor_d14"),
    PINCTRL_PIN(WMT_PIN_NORD15, "nor_d15"),
    PINCTRL_PIN(WMT_PIN_NORA0, "nor_a0"),
    PINCTRL_PIN(WMT_PIN_NORA1, "nor_a1"),
    PINCTRL_PIN(WMT_PIN_NORA2, "nor_a2"),
    PINCTRL_PIN(WMT_PIN_NORA3, "nor_a3"),
    PINCTRL_PIN(WMT_PIN_NORA4, "nor_a4"),
    PINCTRL_PIN(WMT_PIN_NORA5, "nor_a5"),
    PINCTRL_PIN(WMT_PIN_NORA6, "nor_a6"),
    PINCTRL_PIN(WMT_PIN_NORA7, "nor_a7"),
    PINCTRL_PIN(WMT_PIN_NORA8, "nor_a8"),
    PINCTRL_PIN(WMT_PIN_NORA9, "nor_a9"),
    PINCTRL_PIN(WMT_PIN_NORA10, "nor_a10"),
    PINCTRL_PIN(WMT_PIN_NORA11, "nor_a11"),
    PINCTRL_PIN(WMT_PIN_NORA12, "nor_a12"),
    PINCTRL_PIN(WMT_PIN_NORA13, "nor_a13"),
    PINCTRL_PIN(WMT_PIN_NORA14, "nor_a14"),
    PINCTRL_PIN(WMT_PIN_NORA15, "nor_a15"),
    PINCTRL_PIN(WMT_PIN_NORA16, "nor_a16"),
    PINCTRL_PIN(WMT_PIN_NORA17, "nor_a17"),
    PINCTRL_PIN(WMT_PIN_NORA18, "nor_a18"),
    PINCTRL_PIN(WMT_PIN_NORA19, "nor_a19"),
    PINCTRL_PIN(WMT_PIN_NORA20, "nor_a20"),
    PINCTRL_PIN(WMT_PIN_NORA21, "nor_a21"),
    PINCTRL_PIN(WMT_PIN_NORA22, "nor_a22"),
    PINCTRL_PIN(WMT_PIN_NORA23, "nor_a23"),
    PINCTRL_PIN(WMT_PIN_NORA24, "nor_a24"),
    PINCTRL_PIN(WMT_PIN_AC97SDI, "ac97_sdi"),
    PINCTRL_PIN(WMT_PIN_AC97SYNC, "ac97_sync"),
    PINCTRL_PIN(WMT_PIN_AC97SDO, "ac97_sdo"),
    PINCTRL_PIN(WMT_PIN_AC97BCLK, "ac97_bclk"),
    PINCTRL_PIN(WMT_PIN_AC97RST, "ac97_rst"),
    PINCTRL_PIN(WMT_PIN_SFDO, "sf_do"),
    PINCTRL_PIN(WMT_PIN_SFCS0, "sf_cs0"),
    PINCTRL_PIN(WMT_PIN_SFCS1, "sf_cs1"),
    PINCTRL_PIN(WMT_PIN_SFCLK, "sf_clk"),
    PINCTRL_PIN(WMT_PIN_SFDI, "sf_di"),
    PINCTRL_PIN(WMT_PIN_SPI0CLK, "spi0_clk"),
    PINCTRL_PIN(WMT_PIN_SPI0MISO, "spi0_miso"),
    PINCTRL_PIN(WMT_PIN_SPI0MOSI, "spi0_mosi"),
    PINCTRL_PIN(WMT_PIN_SPI0SS, "spi0_ss"),
    PINCTRL_PIN(WMT_PIN_SPI1CLK, "spi1_clk"),
    PINCTRL_PIN(WMT_PIN_SPI1MISO, "spi1_miso"),
    PINCTRL_PIN(WMT_PIN_SPI1MOSI, "spi1_mosi"),
    PINCTRL_PIN(WMT_PIN_SPI1SS, "spi1_ss"),
    PINCTRL_PIN(WMT_PIN_SPI2CLK, "spi2_clk"),
    PINCTRL_PIN(WMT_PIN_SPI2MISO, "spi2_miso"),
    PINCTRL_PIN(WMT_PIN_SPI2MOSI, "spi2_mosi"),
    PINCTRL_PIN(WMT_PIN_SPI2SS, "spi2_ss"),
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
    PINCTRL_PIN(WMT_PIN_I2C0SCL, "i2c0_scl"),
    PINCTRL_PIN(WMT_PIN_I2C0SDA, "i2c0_sda"),
    PINCTRL_PIN(WMT_PIN_I2C1SCL, "i2c1_scl"),
    PINCTRL_PIN(WMT_PIN_I2C1SDA, "i2c1_sda"),
    PINCTRL_PIN(WMT_PIN_I2C2SCL, "i2c2_scl"),
    PINCTRL_PIN(WMT_PIN_I2C2SDA, "i2c2_sda"),
    };
// Order of these names must match the above list
    static const char * const wm8505_groups[] = {
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
    "wakeup2",
    "wakeup3",
    "susgpio0",
    "sd_data0",
    "sd_data1",
    "sd_data2",
    "sd_data3",
    "mmc_data0",
    "mmc_data1",
    "mmc_data2",
    "mmc_data3",
    "vdin0",
    "vdin1",
    "vdin2",
    "vdin3",
    "vdin4",
    "vdin5",
    "vdin6",
    "vdin7",
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
    "v_hsync",
    "v_vsync",
    "vga_hsync",
    "vga_vsync",
    "vd_hsync",
    "vd_vsync",
    "nor_d0",
    "nor_d1",
    "nor_d2",
    "nor_d3",
    "nor_d4",
    "nor_d5",
    "nor_d6",
    "nor_d7",
    "nor_d8",
    "nor_d9",
    "nor_d10",
    "nor_d11",
    "nor_d12",
    "nor_d13",
    "nor_d14",
    "nor_d15",
    "nor_a0",
    "nor_a1",
    "nor_a2",
    "nor_a3",
    "nor_a4",
    "nor_a5",
    "nor_a6",
    "nor_a7",
    "nor_a8",
    "nor_a9",
    "nor_a10",
    "nor_a11",
    "nor_a12",
    "nor_a13",
    "nor_a14",
    "nor_a15",
    "nor_a16",
    "nor_a17",
    "nor_a18",
    "nor_a19",
    "nor_a20",
    "nor_a21",
    "nor_a22",
    "nor_a23",
    "nor_a24",
    "ac97_sdi",
    "ac97_sync",
    "ac97_sdo",
    "ac97_bclk",
    "ac97_rst",
    "sf_do",
    "sf_cs0",
    "sf_cs1",
    "sf_clk",
    "sf_di",
    "spi0_clk",
    "spi0_miso",
    "spi0_mosi",
    "spi0_ss",
    "spi1_clk",
    "spi1_miso",
    "spi1_mosi",
    "spi1_ss",
    "spi2_clk",
    "spi2_miso",
    "spi2_mosi",
    "spi2_ss",
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
    "i2c0_scl",
    "i2c0_sda",
    "i2c1_scl",
    "i2c1_sda",
    "i2c2_scl",
    "i2c2_sda",
    };
#[no_mangle]
unsafe extern "C" fn wm8505_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int wm8505_pinctrl_probe(struct platform_device *pdev)
    {
    struct wmt_pinctrl_data *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.banks = wm8505_banks;
    data.nbanks = ARRAY_SIZE(wm8505_banks);
    data.pins = wm8505_pins;
    data.npins = ARRAY_SIZE(wm8505_pins);
    data.groups = wm8505_groups;
    data.ngroups = ARRAY_SIZE(wm8505_groups);
    return wmt_pinctrl_probe(pdev, data);
    }
    static const struct of_device_id wmt_pinctrl_of_match[] = {
    { .compatible = "wm,wm8505-pinctrl" },
    { /* sentinel */ },
    };
    static struct platform_driver wmt_pinctrl_driver = {
    .probe	= wm8505_pinctrl_probe,
    .driver = {
    .name	= "pinctrl-wm8505",
    .of_match_table	= wmt_pinctrl_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(wmt_pinctrl_driver);
