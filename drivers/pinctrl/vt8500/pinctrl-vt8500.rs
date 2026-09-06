//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/vt8500/pinctrl-vt8500.c
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
// Pinctrl data for VIA VT8500 SoC
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
    static const struct wmt_pinctrl_bank_registers vt8500_banks[] = {
    WMT_PINCTRL_BANK(NO_REG, 0x3C, 0x5C, 0x7C, NO_REG, NO_REG),	/* 0 */
    WMT_PINCTRL_BANK(0x00, 0x20, 0x40, 0x60, NO_REG, NO_REG),	/* 1 */
    WMT_PINCTRL_BANK(0x04, 0x24, 0x44, 0x64, NO_REG, NO_REG),	/* 2 */
    WMT_PINCTRL_BANK(0x08, 0x28, 0x48, 0x68, NO_REG, NO_REG),	/* 3 */
    WMT_PINCTRL_BANK(0x0C, 0x2C, 0x4C, 0x6C, NO_REG, NO_REG),	/* 4 */
    WMT_PINCTRL_BANK(0x10, 0x30, 0x50, 0x70, NO_REG, NO_REG),	/* 5 */
    WMT_PINCTRL_BANK(0x14, 0x34, 0x54, 0x74, NO_REG, NO_REG),	/* 6 */
    };
// Please keep sorted by bank/bit

    static const struct pinctrl_pin_desc vt8500_pins[] = {
    PINCTRL_PIN(WMT_PIN_EXTGPIO0, "extgpio0"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO1, "extgpio1"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO2, "extgpio2"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO3, "extgpio3"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO4, "extgpio4"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO5, "extgpio5"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO6, "extgpio6"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO7, "extgpio7"),
    PINCTRL_PIN(WMT_PIN_EXTGPIO8, "extgpio8"),
    PINCTRL_PIN(WMT_PIN_UART0RTS, "uart0_rts"),
    PINCTRL_PIN(WMT_PIN_UART0TXD, "uart0_txd"),
    PINCTRL_PIN(WMT_PIN_UART0CTS, "uart0_cts"),
    PINCTRL_PIN(WMT_PIN_UART0RXD, "uart0_rxd"),
    PINCTRL_PIN(WMT_PIN_UART1RTS, "uart1_rts"),
    PINCTRL_PIN(WMT_PIN_UART1TXD, "uart1_txd"),
    PINCTRL_PIN(WMT_PIN_UART1CTS, "uart1_cts"),
    PINCTRL_PIN(WMT_PIN_UART1RXD, "uart1_rxd"),
    PINCTRL_PIN(WMT_PIN_SPI0CLK, "spi0_clk"),
    PINCTRL_PIN(WMT_PIN_SPI0SS, "spi0_ss"),
    PINCTRL_PIN(WMT_PIN_SPI0MISO, "spi0_miso"),
    PINCTRL_PIN(WMT_PIN_SPI0MOSI, "spi0_mosi"),
    PINCTRL_PIN(WMT_PIN_SPI1CLK, "spi1_clk"),
    PINCTRL_PIN(WMT_PIN_SPI1SS, "spi1_ss"),
    PINCTRL_PIN(WMT_PIN_SPI1MISO, "spi1_miso"),
    PINCTRL_PIN(WMT_PIN_SPI1MOSI, "spi1_mosi"),
    PINCTRL_PIN(WMT_PIN_SPI2CLK, "spi2_clk"),
    PINCTRL_PIN(WMT_PIN_SPI2SS, "spi2_ss"),
    PINCTRL_PIN(WMT_PIN_SPI2MISO, "spi2_miso"),
    PINCTRL_PIN(WMT_PIN_SPI2MOSI, "spi2_mosi"),
    PINCTRL_PIN(WMT_PIN_SDDATA0, "sd_data0"),
    PINCTRL_PIN(WMT_PIN_SDDATA1, "sd_data1"),
    PINCTRL_PIN(WMT_PIN_SDDATA2, "sd_data2"),
    PINCTRL_PIN(WMT_PIN_SDDATA3, "sd_data3"),
    PINCTRL_PIN(WMT_PIN_MMCDATA0, "mmc_data0"),
    PINCTRL_PIN(WMT_PIN_MMCDATA1, "mmc_data1"),
    PINCTRL_PIN(WMT_PIN_MMCDATA2, "mmc_data2"),
    PINCTRL_PIN(WMT_PIN_MMCDATA3, "mmc_data3"),
    PINCTRL_PIN(WMT_PIN_SDCLK, "sd_clk"),
    PINCTRL_PIN(WMT_PIN_SDWP, "sd_wp"),
    PINCTRL_PIN(WMT_PIN_SDCMD, "sd_cmd"),
    PINCTRL_PIN(WMT_PIN_MSDATA0, "ms_data0"),
    PINCTRL_PIN(WMT_PIN_MSDATA1, "ms_data1"),
    PINCTRL_PIN(WMT_PIN_MSDATA2, "ms_data2"),
    PINCTRL_PIN(WMT_PIN_MSDATA3, "ms_data3"),
    PINCTRL_PIN(WMT_PIN_MSCLK, "ms_clk"),
    PINCTRL_PIN(WMT_PIN_MSBS, "ms_bs"),
    PINCTRL_PIN(WMT_PIN_MSINS, "ms_ins"),
    PINCTRL_PIN(WMT_PIN_I2C0SCL, "i2c0_scl"),
    PINCTRL_PIN(WMT_PIN_I2C0SDA, "i2c0_sda"),
    PINCTRL_PIN(WMT_PIN_I2C1SCL, "i2c1_scl"),
    PINCTRL_PIN(WMT_PIN_I2C1SDA, "i2c1_sda"),
    PINCTRL_PIN(WMT_PIN_MII0RXD0, "mii0_rxd0"),
    PINCTRL_PIN(WMT_PIN_MII0RXD1, "mii0_rxd1"),
    PINCTRL_PIN(WMT_PIN_MII0RXD2, "mii0_rxd2"),
    PINCTRL_PIN(WMT_PIN_MII0RXD3, "mii0_rxd3"),
    PINCTRL_PIN(WMT_PIN_MII0RXCLK, "mii0_rxclk"),
    PINCTRL_PIN(WMT_PIN_MII0RXDV, "mii0_rxdv"),
    PINCTRL_PIN(WMT_PIN_MII0RXERR, "mii0_rxerr"),
    PINCTRL_PIN(WMT_PIN_MII0PHYRST, "mii0_phyrst"),
    PINCTRL_PIN(WMT_PIN_MII0TXD0, "mii0_txd0"),
    PINCTRL_PIN(WMT_PIN_MII0TXD1, "mii0_txd1"),
    PINCTRL_PIN(WMT_PIN_MII0TXD2, "mii0_txd2"),
    PINCTRL_PIN(WMT_PIN_MII0TXD3, "mii0_txd3"),
    PINCTRL_PIN(WMT_PIN_MII0TXCLK, "mii0_txclk"),
    PINCTRL_PIN(WMT_PIN_MII0TXEN, "mii0_txen"),
    PINCTRL_PIN(WMT_PIN_MII0TXERR, "mii0_txerr"),
    PINCTRL_PIN(WMT_PIN_MII0PHYPD, "mii0_phypd"),
    PINCTRL_PIN(WMT_PIN_MII0COL, "mii0_col"),
    PINCTRL_PIN(WMT_PIN_MII0CRS, "mii0_crs"),
    PINCTRL_PIN(WMT_PIN_MII0MDIO, "mii0_mdio"),
    PINCTRL_PIN(WMT_PIN_MII0MDC, "mii0_mdc"),
    PINCTRL_PIN(WMT_PIN_SEECS, "see_cs"),
    PINCTRL_PIN(WMT_PIN_SEECK, "see_ck"),
    PINCTRL_PIN(WMT_PIN_SEEDI, "see_di"),
    PINCTRL_PIN(WMT_PIN_SEEDO, "see_do"),
    PINCTRL_PIN(WMT_PIN_IDEDREQ0, "ide_dreq0"),
    PINCTRL_PIN(WMT_PIN_IDEDREQ1, "ide_dreq1"),
    PINCTRL_PIN(WMT_PIN_IDEIOW, "ide_iow"),
    PINCTRL_PIN(WMT_PIN_IDEIOR, "ide_ior"),
    PINCTRL_PIN(WMT_PIN_IDEDACK, "ide_dack"),
    PINCTRL_PIN(WMT_PIN_IDEIORDY, "ide_iordy"),
    PINCTRL_PIN(WMT_PIN_IDEINTRQ, "ide_intrq"),
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
    PINCTRL_PIN(WMT_PIN_NANDCLE0, "nand_cle0"),
    PINCTRL_PIN(WMT_PIN_NANDCLE1, "nand_cle1"),
    PINCTRL_PIN(WMT_PIN_VDOUT6_7, "vdout6_7"),
    PINCTRL_PIN(WMT_PIN_VHSYNC, "vhsync"),
    PINCTRL_PIN(WMT_PIN_VVSYNC, "vvsync"),
    PINCTRL_PIN(WMT_PIN_TSDIN0, "tsdin0"),
    PINCTRL_PIN(WMT_PIN_TSDIN1, "tsdin1"),
    PINCTRL_PIN(WMT_PIN_TSDIN2, "tsdin2"),
    PINCTRL_PIN(WMT_PIN_TSDIN3, "tsdin3"),
    PINCTRL_PIN(WMT_PIN_TSDIN4, "tsdin4"),
    PINCTRL_PIN(WMT_PIN_TSDIN5, "tsdin5"),
    PINCTRL_PIN(WMT_PIN_TSDIN6, "tsdin6"),
    PINCTRL_PIN(WMT_PIN_TSDIN7, "tsdin7"),
    PINCTRL_PIN(WMT_PIN_TSSYNC, "tssync"),
    PINCTRL_PIN(WMT_PIN_TSVALID, "tsvalid"),
    PINCTRL_PIN(WMT_PIN_TSCLK, "tsclk"),
    PINCTRL_PIN(WMT_PIN_LCDD0, "lcd_d0"),
    PINCTRL_PIN(WMT_PIN_LCDD1, "lcd_d1"),
    PINCTRL_PIN(WMT_PIN_LCDD2, "lcd_d2"),
    PINCTRL_PIN(WMT_PIN_LCDD3, "lcd_d3"),
    PINCTRL_PIN(WMT_PIN_LCDD4, "lcd_d4"),
    PINCTRL_PIN(WMT_PIN_LCDD5, "lcd_d5"),
    PINCTRL_PIN(WMT_PIN_LCDD6, "lcd_d6"),
    PINCTRL_PIN(WMT_PIN_LCDD7, "lcd_d7"),
    PINCTRL_PIN(WMT_PIN_LCDD8, "lcd_d8"),
    PINCTRL_PIN(WMT_PIN_LCDD9, "lcd_d9"),
    PINCTRL_PIN(WMT_PIN_LCDD10, "lcd_d10"),
    PINCTRL_PIN(WMT_PIN_LCDD11, "lcd_d11"),
    PINCTRL_PIN(WMT_PIN_LCDD12, "lcd_d12"),
    PINCTRL_PIN(WMT_PIN_LCDD13, "lcd_d13"),
    PINCTRL_PIN(WMT_PIN_LCDD14, "lcd_d14"),
    PINCTRL_PIN(WMT_PIN_LCDD15, "lcd_d15"),
    PINCTRL_PIN(WMT_PIN_LCDD16, "lcd_d16"),
    PINCTRL_PIN(WMT_PIN_LCDD17, "lcd_d17"),
    PINCTRL_PIN(WMT_PIN_LCDCLK, "lcd_clk"),
    PINCTRL_PIN(WMT_PIN_LCDDEN, "lcd_den"),
    PINCTRL_PIN(WMT_PIN_LCDLINE, "lcd_line"),
    PINCTRL_PIN(WMT_PIN_LCDFRM, "lcd_frm"),
    PINCTRL_PIN(WMT_PIN_LCDBIAS, "lcd_bias"),
    };
// Order of these names must match the above list
    static const char * const vt8500_groups[] = {
    "extgpio0",
    "extgpio1",
    "extgpio2",
    "extgpio3",
    "extgpio4",
    "extgpio5",
    "extgpio6",
    "extgpio7",
    "extgpio8",
    "uart0_rts",
    "uart0_txd",
    "uart0_cts",
    "uart0_rxd",
    "uart1_rts",
    "uart1_txd",
    "uart1_cts",
    "uart1_rxd",
    "spi0_clk",
    "spi0_ss",
    "spi0_miso",
    "spi0_mosi",
    "spi1_clk",
    "spi1_ss",
    "spi1_miso",
    "spi1_mosi",
    "spi2_clk",
    "spi2_ss",
    "spi2_miso",
    "spi2_mosi",
    "sd_data0",
    "sd_data1",
    "sd_data2",
    "sd_data3",
    "mmc_data0",
    "mmc_data1",
    "mmc_data2",
    "mmc_data3",
    "sd_clk",
    "sd_wp",
    "sd_cmd",
    "ms_data0",
    "ms_data1",
    "ms_data2",
    "ms_data3",
    "ms_clk",
    "ms_bs",
    "ms_ins",
    "i2c0_scl",
    "i2c0_sda",
    "i2c1_scl",
    "i2c1_sda",
    "mii0_rxd0",
    "mii0_rxd1",
    "mii0_rxd2",
    "mii0_rxd3",
    "mii0_rxclk",
    "mii0_rxdv",
    "mii0_rxerr",
    "mii0_phyrst",
    "mii0_txd0",
    "mii0_txd1",
    "mii0_txd2",
    "mii0_txd3",
    "mii0_txclk",
    "mii0_txen",
    "mii0_txerr",
    "mii0_phypd",
    "mii0_col",
    "mii0_crs",
    "mii0_mdio",
    "mii0_mdc",
    "see_cs",
    "see_ck",
    "see_di",
    "see_do",
    "ide_dreq0",
    "ide_dreq1",
    "ide_iow",
    "ide_ior",
    "ide_dack",
    "ide_iordy",
    "ide_intrq",
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
    "nand_cle0",
    "nand_cle1",
    "vdout6_7",
    "vhsync",
    "vvsync",
    "tsdin0",
    "tsdin1",
    "tsdin2",
    "tsdin3",
    "tsdin4",
    "tsdin5",
    "tsdin6",
    "tsdin7",
    "tssync",
    "tsvalid",
    "tsclk",
    "lcd_d0",
    "lcd_d1",
    "lcd_d2",
    "lcd_d3",
    "lcd_d4",
    "lcd_d5",
    "lcd_d6",
    "lcd_d7",
    "lcd_d8",
    "lcd_d9",
    "lcd_d10",
    "lcd_d11",
    "lcd_d12",
    "lcd_d13",
    "lcd_d14",
    "lcd_d15",
    "lcd_d16",
    "lcd_d17",
    "lcd_clk",
    "lcd_den",
    "lcd_line",
    "lcd_frm",
    "lcd_bias",
    };
#[no_mangle]
unsafe extern "C" fn vt8500_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int vt8500_pinctrl_probe(struct platform_device *pdev)
    {
    struct wmt_pinctrl_data *data;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.banks = vt8500_banks;
    data.nbanks = ARRAY_SIZE(vt8500_banks);
    data.pins = vt8500_pins;
    data.npins = ARRAY_SIZE(vt8500_pins);
    data.groups = vt8500_groups;
    data.ngroups = ARRAY_SIZE(vt8500_groups);
    return wmt_pinctrl_probe(pdev, data);
    }
    static const struct of_device_id wmt_pinctrl_of_match[] = {
    { .compatible = "via,vt8500-pinctrl" },
    { /* sentinel */ },
    };
    static struct platform_driver wmt_pinctrl_driver = {
    .probe	= vt8500_pinctrl_probe,
    .driver = {
    .name	= "pinctrl-vt8500",
    .of_match_table	= wmt_pinctrl_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(wmt_pinctrl_driver);
