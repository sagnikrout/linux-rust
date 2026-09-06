//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-stm32f4.c
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
// Author: Daniel Thompson <daniel.thompson@linaro.org>
//
// Inspired by clk-asm9260.c .
//

//
// Include list of clocks which are not derived from system clock (SYSCLOCK)
// The index of these clocks is the secondary index of DT bindings
//

pub const STM32F4_RCC_CR: c_uint = 0x00;
pub const STM32F4_RCC_PLLCFGR: c_uint = 0x04;
pub const STM32F4_RCC_CFGR: c_uint = 0x08;
pub const STM32F4_RCC_AHB1ENR: c_uint = 0x30;
pub const STM32F4_RCC_AHB2ENR: c_uint = 0x34;
pub const STM32F4_RCC_AHB3ENR: c_uint = 0x38;
pub const STM32F4_RCC_APB1ENR: c_uint = 0x40;
pub const STM32F4_RCC_APB2ENR: c_uint = 0x44;
pub const STM32F4_RCC_BDCR: c_uint = 0x70;
pub const STM32F4_RCC_CSR: c_uint = 0x74;
pub const STM32F4_RCC_SSCGR: c_uint = 0x80;
pub const STM32F4_RCC_PLLI2SCFGR: c_uint = 0x84;
pub const STM32F4_RCC_PLLSAICFGR: c_uint = 0x88;
pub const STM32F4_RCC_DCKCFGR: c_uint = 0x8c;
pub const STM32F7_RCC_DCKCFGR2: c_uint = 0x90;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_gate_data {
    pub offset: u8,
    pub bit_idx: u8,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
}

    static const struct stm32f4_gate_data stm32f429_gates[] __initconst = {
    { STM32F4_RCC_AHB1ENR,  0,	"gpioa",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  1,	"gpiob",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  2,	"gpioc",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  3,	"gpiod",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  4,	"gpioe",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  5,	"gpiof",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  6,	"gpiog",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  7,	"gpioh",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  8,	"gpioi",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  9,	"gpioj",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 10,	"gpiok",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 12,	"crc",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 18,	"bkpsra",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 20,	"ccmdatam",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 21,	"dma1",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 22,	"dma2",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 23,	"dma2d",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 25,	"ethmac",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 26,	"ethmactx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 27,	"ethmacrx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 28,	"ethmacptp",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 29,	"otghs",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 30,	"otghsulpi",	"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  0,	"dcmi",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  4,	"cryp",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  5,	"hash",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  6,	"rng",		"pll48" },
    { STM32F4_RCC_AHB2ENR,  7,	"otgfs",	"pll48" },
    { STM32F4_RCC_AHB3ENR,  0,	"fmc",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_APB1ENR,  0,	"tim2",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  1,	"tim3",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  2,	"tim4",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  3,	"tim5",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  4,	"tim6",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  5,	"tim7",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  6,	"tim12",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  7,	"tim13",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  8,	"tim14",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR, 11,	"wwdg",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 14,	"spi2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 15,	"spi3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 17,	"uart2",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 18,	"uart3",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 19,	"uart4",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 20,	"uart5",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 21,	"i2c1",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 22,	"i2c2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 23,	"i2c3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 25,	"can1",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 26,	"can2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 28,	"pwr",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 29,	"dac",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 30,	"uart7",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 31,	"uart8",	"apb1_div" },
    { STM32F4_RCC_APB2ENR,  0,	"tim1",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  1,	"tim8",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  4,	"usart1",	"apb2_div" },
    { STM32F4_RCC_APB2ENR,  5,	"usart6",	"apb2_div" },
    { STM32F4_RCC_APB2ENR,  8,	"adc1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR,  9,	"adc2",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 10,	"adc3",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 11,	"sdio",		"pll48" },
    { STM32F4_RCC_APB2ENR, 12,	"spi1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 13,	"spi4",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 14,	"syscfg",	"apb2_div" },
    { STM32F4_RCC_APB2ENR, 16,	"tim9",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 17,	"tim10",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 18,	"tim11",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 20,	"spi5",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 21,	"spi6",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 22,	"sai1",		"apb2_div" },
    };
    static const struct stm32f4_gate_data stm32f469_gates[] __initconst = {
    { STM32F4_RCC_AHB1ENR,  0,	"gpioa",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  1,	"gpiob",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  2,	"gpioc",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  3,	"gpiod",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  4,	"gpioe",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  5,	"gpiof",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  6,	"gpiog",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  7,	"gpioh",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  8,	"gpioi",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  9,	"gpioj",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 10,	"gpiok",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 12,	"crc",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 18,	"bkpsra",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 20,	"ccmdatam",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 21,	"dma1",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 22,	"dma2",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 23,	"dma2d",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 25,	"ethmac",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 26,	"ethmactx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 27,	"ethmacrx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 28,	"ethmacptp",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 29,	"otghs",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 30,	"otghsulpi",	"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  0,	"dcmi",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  4,	"cryp",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  5,	"hash",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  6,	"rng",		"pll48" },
    { STM32F4_RCC_AHB2ENR,  7,	"otgfs",	"pll48" },
    { STM32F4_RCC_AHB3ENR,  0,	"fmc",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_AHB3ENR,  1,	"qspi",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_APB1ENR,  0,	"tim2",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  1,	"tim3",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  2,	"tim4",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  3,	"tim5",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  4,	"tim6",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  5,	"tim7",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  6,	"tim12",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  7,	"tim13",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  8,	"tim14",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR, 11,	"wwdg",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 14,	"spi2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 15,	"spi3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 17,	"uart2",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 18,	"uart3",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 19,	"uart4",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 20,	"uart5",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 21,	"i2c1",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 22,	"i2c2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 23,	"i2c3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 25,	"can1",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 26,	"can2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 28,	"pwr",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 29,	"dac",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 30,	"uart7",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 31,	"uart8",	"apb1_div" },
    { STM32F4_RCC_APB2ENR,  0,	"tim1",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  1,	"tim8",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  4,	"usart1",	"apb2_div" },
    { STM32F4_RCC_APB2ENR,  5,	"usart6",	"apb2_div" },
    { STM32F4_RCC_APB2ENR,  8,	"adc1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR,  9,	"adc2",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 10,	"adc3",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 11,	"sdio",		"sdmux" },
    { STM32F4_RCC_APB2ENR, 12,	"spi1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 13,	"spi4",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 14,	"syscfg",	"apb2_div" },
    { STM32F4_RCC_APB2ENR, 16,	"tim9",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 17,	"tim10",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 18,	"tim11",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 20,	"spi5",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 21,	"spi6",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 22,	"sai1",		"apb2_div" },
    };
    static const struct stm32f4_gate_data stm32f746_gates[] __initconst = {
    { STM32F4_RCC_AHB1ENR,  0,	"gpioa",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  1,	"gpiob",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  2,	"gpioc",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  3,	"gpiod",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  4,	"gpioe",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  5,	"gpiof",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  6,	"gpiog",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  7,	"gpioh",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  8,	"gpioi",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  9,	"gpioj",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 10,	"gpiok",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 12,	"crc",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 18,	"bkpsra",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 20,	"dtcmram",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 21,	"dma1",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 22,	"dma2",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 23,	"dma2d",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 25,	"ethmac",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 26,	"ethmactx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 27,	"ethmacrx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 28,	"ethmacptp",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 29,	"otghs",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 30,	"otghsulpi",	"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  0,	"dcmi",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  4,	"cryp",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  5,	"hash",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  6,	"rng",		"pll48"   },
    { STM32F4_RCC_AHB2ENR,  7,	"otgfs",	"pll48"   },
    { STM32F4_RCC_AHB3ENR,  0,	"fmc",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_AHB3ENR,  1,	"qspi",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_APB1ENR,  0,	"tim2",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  1,	"tim3",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  2,	"tim4",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  3,	"tim5",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  4,	"tim6",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  5,	"tim7",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  6,	"tim12",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  7,	"tim13",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  8,	"tim14",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR, 11,	"wwdg",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 14,	"spi2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 15,	"spi3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 16,	"spdifrx",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 25,	"can1",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 26,	"can2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 27,	"cec",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 28,	"pwr",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 29,	"dac",		"apb1_div" },
    { STM32F4_RCC_APB2ENR,  0,	"tim1",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  1,	"tim8",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  7,	"sdmmc2",	"sdmux"    },
    { STM32F4_RCC_APB2ENR,  8,	"adc1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR,  9,	"adc2",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 10,	"adc3",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 11,	"sdmmc",	"sdmux"    },
    { STM32F4_RCC_APB2ENR, 12,	"spi1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 13,	"spi4",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 14,	"syscfg",	"apb2_div" },
    { STM32F4_RCC_APB2ENR, 16,	"tim9",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 17,	"tim10",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 18,	"tim11",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 20,	"spi5",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 21,	"spi6",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 22,	"sai1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 23,	"sai2",		"apb2_div" },
    };
    static const struct stm32f4_gate_data stm32f769_gates[] __initconst = {
    { STM32F4_RCC_AHB1ENR,  0,	"gpioa",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  1,	"gpiob",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  2,	"gpioc",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  3,	"gpiod",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  4,	"gpioe",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  5,	"gpiof",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  6,	"gpiog",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  7,	"gpioh",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  8,	"gpioi",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR,  9,	"gpioj",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 10,	"gpiok",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 12,	"crc",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 18,	"bkpsra",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 20,	"dtcmram",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 21,	"dma1",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 22,	"dma2",		"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 23,	"dma2d",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 25,	"ethmac",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 26,	"ethmactx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 27,	"ethmacrx",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 28,	"ethmacptp",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 29,	"otghs",	"ahb_div" },
    { STM32F4_RCC_AHB1ENR, 30,	"otghsulpi",	"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  0,	"dcmi",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  1,	"jpeg",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  4,	"cryp",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  5,	"hash",		"ahb_div" },
    { STM32F4_RCC_AHB2ENR,  6,	"rng",		"pll48"   },
    { STM32F4_RCC_AHB2ENR,  7,	"otgfs",	"pll48"   },
    { STM32F4_RCC_AHB3ENR,  0,	"fmc",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_AHB3ENR,  1,	"qspi",		"ahb_div",
    CLK_IGNORE_UNUSED },
    { STM32F4_RCC_APB1ENR,  0,	"tim2",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  1,	"tim3",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  2,	"tim4",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  3,	"tim5",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  4,	"tim6",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  5,	"tim7",		"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  6,	"tim12",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  7,	"tim13",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR,  8,	"tim14",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR, 10,	"rtcapb",	"apb1_mul" },
    { STM32F4_RCC_APB1ENR, 11,	"wwdg",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 13,	"can3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 14,	"spi2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 15,	"spi3",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 16,	"spdifrx",	"apb1_div" },
    { STM32F4_RCC_APB1ENR, 25,	"can1",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 26,	"can2",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 27,	"cec",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 28,	"pwr",		"apb1_div" },
    { STM32F4_RCC_APB1ENR, 29,	"dac",		"apb1_div" },
    { STM32F4_RCC_APB2ENR,  0,	"tim1",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  1,	"tim8",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR,  7,	"sdmmc2",	"sdmux2" },
    { STM32F4_RCC_APB2ENR,  8,	"adc1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR,  9,	"adc2",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 10,	"adc3",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 11,	"sdmmc1",	"sdmux1" },
    { STM32F4_RCC_APB2ENR, 12,	"spi1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 13,	"spi4",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 14,	"syscfg",	"apb2_div" },
    { STM32F4_RCC_APB2ENR, 16,	"tim9",		"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 17,	"tim10",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 18,	"tim11",	"apb2_mul" },
    { STM32F4_RCC_APB2ENR, 20,	"spi5",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 21,	"spi6",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 22,	"sai1",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 23,	"sai2",		"apb2_div" },
    { STM32F4_RCC_APB2ENR, 30,	"mdio",		"apb2_div" },
    };
    enum stm32f4_pll_ssc_mod_type {
    STM32F4_PLL_SSC_CENTER_SPREAD,
    STM32F4_PLL_SSC_DOWN_SPREAD,
    };
    static const char * const stm32f4_ssc_mod_methods[] __initconst = {
    [STM32F4_PLL_SSC_DOWN_SPREAD] = "down-spread",
    [STM32F4_PLL_SSC_CENTER_SPREAD] = "center-spread",
    };
//
// This bitmask tells us which bit offsets (0..192) on STM32F4[23]xxx
// have gate bits associated with them. Its combined hweight is 71.
//
pub const MAX_GATE_MAP: c_int = 3;
    static const u64 stm32f42xx_gate_map[MAX_GATE_MAP] = { 0x000000f17ef417ffull,
    0x0000000000000001ull,
    0x04777f33f6fec9ffull };
    static const u64 stm32f46xx_gate_map[MAX_GATE_MAP] = { 0x000000f17ef417ffull,
    0x0000000000000003ull,
    0x0c777f33f6fec9ffull };
    static const u64 stm32f746_gate_map[MAX_GATE_MAP] = { 0x000000f17ef417ffull,
    0x0000000000000003ull,
    0x04f77f833e01c9ffull };
    static const u64 stm32f769_gate_map[MAX_GATE_MAP] = { 0x000000f37ef417ffull,
    0x0000000000000003ull,
    0x44F77F833E01EDFFull };
    static const u64 *stm32f4_gate_map;
    static struct clk_hw **clks;
    static DEFINE_SPINLOCK(stm32f4_clk_lock);
    static void __iomem *base;
    static struct regmap *pdrm;
    static int stm32fx_end_primary_clk;
//
// "Multiplier" device for APBx clocks.
//
// The APBx dividers are power-of-two dividers and, if *not* running in 1:1
// mode, they also tap out the one of the low order state bits to run the
// timers. ST datasheets represent this feature as a (conditional) clock
// multiplier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_apb_mul {
    pub hw: clk_hw,
    pub bit_idx: u8,
}

    static unsigned long clk_apb_mul_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_apb_mul *am = to_clk_apb_mul(hw);
    if (readl(base + STM32F4_RCC_CFGR) & BIT(am.bit_idx))
    return parent_rate * 2;
    return parent_rate;
    }
    static int clk_apb_mul_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_apb_mul *am = to_clk_apb_mul(hw);
    let mut mult: c_ulong = 1;
    if (readl(base + STM32F4_RCC_CFGR) & BIT(am.bit_idx))
    mult = 2;
    if (clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT) {
    let mut best_parent: c_ulong = req.rate / mult;
    req.best_parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), best_parent);
    }
    req.rate = req.best_parent_rate * mult;
    return 0;
    }
    static int clk_apb_mul_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
//
// We must report success but we can do so unconditionally because
// clk_apb_mul_round_rate returns values that ensure this call is a
// nop.
//
    return 0;
    }
    static const struct clk_ops clk_apb_mul_factor_ops = {
    .determine_rate = clk_apb_mul_determine_rate,
    .set_rate = clk_apb_mul_set_rate,
    .recalc_rate = clk_apb_mul_recalc_rate,
    };
    static struct clk *clk_register_apb_mul(struct device *dev, const char *name,
    const char *parent_name,
    unsigned long flags, u8 bit_idx)
    {
    struct clk_apb_mul *am;
    struct clk_init_data init;
    struct clk *clk;
    am = kzalloc_obj(*am);
    if (!am)
    return ERR_PTR(-ENOMEM);
    am.bit_idx = bit_idx;
    am.hw.init = &init;
    init.name = name;
    init.ops = &clk_apb_mul_factor_ops;
    init.flags = flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    clk = clk_register(dev, &am.hw);
    if (IS_ERR(clk))
    kfree(am);
    return clk;
    }
    enum {
    PLL,
    PLL_I2S,
    PLL_SAI,
    };
    static const struct clk_div_table pll_divp_table[] = {
    { 0, 2 }, { 1, 4 }, { 2, 6 }, { 3, 8 }, { 0 }
    };
    static const struct clk_div_table pll_divq_table[] = {
    { 2, 2 }, { 3, 3 }, { 4, 4 }, { 5, 5 }, { 6, 6 }, { 7, 7 },
    { 8, 8 }, { 9, 9 }, { 10, 10 }, { 11, 11 }, { 12, 12 }, { 13, 13 },
    { 14, 14 }, { 15, 15 },
    { 0 }
    };
    static const struct clk_div_table pll_divr_table[] = {
    { 2, 2 }, { 3, 3 }, { 4, 4 }, { 5, 5 }, { 6, 6 }, { 7, 7 }, { 0 }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_pll_ssc {
    pub mod_freq: c_uint,
    pub mod_depth: c_uint,
    pub mod_type: enum stm32f4_pll_ssc_mod_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_pll {
    pub lock: *mut spinlock_t,
    pub gate: clk_gate,
    pub offset: u8,
    pub bit_rdy_idx: u8,
    pub status: u8,
    pub n_start: u8,
    pub ssc_enable: bool,
    pub ssc_conf: stm32f4_pll_ssc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_pll_post_div_data {
    pub idx: c_int,
    pub pll_idx: c_int,
    pub name: *const c_char,
    pub parent: *const c_char,
    pub flag: u8,
    pub offset: u8,
    pub shift: u8,
    pub width: u8,
    pub flag_div: u8,
    pub div_table: *const clk_div_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_vco_data {
    pub vco_name: *const c_char,
    pub offset: u8,
    pub bit_idx: u8,
    pub bit_rdy_idx: u8,
    pub sscg: bool,
}

    static const struct stm32f4_vco_data  vco_data[] = {
    { "vco",     STM32F4_RCC_PLLCFGR,    24, 25 },
    { "vco-i2s", STM32F4_RCC_PLLI2SCFGR, 26, 27 },
    { "vco-sai", STM32F4_RCC_PLLSAICFGR, 28, 29 },
    };
    static const struct clk_div_table post_divr_table[] = {
    { 0, 2 }, { 1, 4 }, { 2, 8 }, { 3, 16 }, { 0 }
    };
pub const MAX_POST_DIV: c_int = 3;
    static const struct stm32f4_pll_post_div_data  post_div_data[MAX_POST_DIV] = {
    { CLK_I2SQ_PDIV, PLL_VCO_I2S, "plli2s-q-div", "plli2s-q",
    CLK_SET_RATE_PARENT, STM32F4_RCC_DCKCFGR, 0, 5, 0, core::ptr::null_mut()},
    { CLK_SAIQ_PDIV, PLL_VCO_SAI, "pllsai-q-div", "pllsai-q",
    CLK_SET_RATE_PARENT, STM32F4_RCC_DCKCFGR, 8, 5, 0, core::ptr::null_mut() },
    { NO_IDX, PLL_VCO_SAI, "pllsai-r-div", "pllsai-r", CLK_SET_RATE_PARENT,
    STM32F4_RCC_DCKCFGR, 16, 2, 0, post_divr_table },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_div_data {
    pub shift: u8,
    pub width: u8,
    pub flag_div: u8,
    pub div_table: *const clk_div_table,
}

pub const MAX_PLL_DIV: c_int = 3;
    static const struct stm32f4_div_data  div_data[MAX_PLL_DIV] = {
    { 16, 2, 0, pll_divp_table },
    { 24, 4, 0, pll_divq_table },
    { 28, 3, 0, pll_divr_table },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_pll_data {
    pub pll_num: u8,
    pub n_start: u8,
    pub div_name: [*const c_char; MAX_PLL_DIV],
}

    static const struct stm32f4_pll_data stm32f429_pll[MAX_PLL_DIV] = {
    { PLL,	   192, { "pll", "pll48",    core::ptr::null_mut()	} },
    { PLL_I2S, 192, { core::ptr::null_mut(),  "plli2s-q", "plli2s-r" } },
    { PLL_SAI,  49, { core::ptr::null_mut(),  "pllsai-q", "pllsai-r" } },
    };
    static const struct stm32f4_pll_data stm32f469_pll[MAX_PLL_DIV] = {
    { PLL,	   50, { "pll",	     "pll-q",    "pll-r"    } },
    { PLL_I2S, 50, { "plli2s-p", "plli2s-q", "plli2s-r" } },
    { PLL_SAI, 50, { "pllsai-p", "pllsai-q", "pllsai-r" } },
    };
#[no_mangle]
unsafe extern "C" fn stm32f4_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int stm32f4_pll_is_enabled(struct clk_hw *hw)
    {
    return clk_gate_ops.is_enabled(hw);
    }
pub const PLL_TIMEOUT: c_int = 10000;
#[no_mangle]
unsafe extern "C" fn stm32f4_pll_enable(hw: *mut clk_hw) -> c_int {
    static int stm32f4_pll_enable(struct clk_hw *hw)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32f4_pll *pll = to_stm32f4_pll(gate);
    int bit_status;
    let mut timeout: c_uint = PLL_TIMEOUT;
    if (clk_gate_ops.is_enabled(hw))
    return 0;
    clk_gate_ops.enable(hw);
    do {
    bit_status = !(readl(gate.reg) & BIT(pll.bit_rdy_idx));
    } while (bit_status && --timeout);
    return bit_status;
    }
#[no_mangle]
unsafe extern "C" fn stm32f4_pll_disable(hw: *mut clk_hw) {
    static void stm32f4_pll_disable(struct clk_hw *hw)
    {
    clk_gate_ops.disable(hw);
    }
    static unsigned long stm32f4_pll_recalc(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32f4_pll *pll = to_stm32f4_pll(gate);
    unsigned long val;
    unsigned long n;
    val = readl(base + pll.offset);
    n = FIELD_GET(STM32F4_RCC_PLLCFGR_N_MASK, val);
    return parent_rate * n;
    }
    static int stm32f4_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32f4_pll *pll = to_stm32f4_pll(gate);
    unsigned long n;
    n = req.rate / req.best_parent_rate;
    if (n < pll.n_start)
    n = pll.n_start;
#[no_mangle]
pub unsafe extern "C" fn if(432: n >) -> else {
    else if (n > 432)
    n = 432;
    req.rate = req.best_parent_rate * n;
    return 0;
    }
    static void stm32f4_pll_set_ssc(struct clk_hw *hw, unsigned long parent_rate,
    unsigned int ndiv)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32f4_pll *pll = to_stm32f4_pll(gate);
    struct stm32f4_pll_ssc *ssc = &pll.ssc_conf;
    u32 modeper, incstep;
    u32 sscgr;
    sscgr = readl(base + STM32F4_RCC_SSCGR);
// reserved field must be kept at reset value
    sscgr &= STM32F4_RCC_SSCGR_RESERVED_MASK;
    modeper = DIV_ROUND_CLOSEST(parent_rate, 4 * ssc.mod_freq);
    incstep = DIV_ROUND_CLOSEST(((1 << 15) - 1) * ssc.mod_depth * ndiv,
    5 * 10000 * modeper);
    sscgr |= STM32F4_RCC_SSCGR_SSCGEN |
    FIELD_PREP(STM32F4_RCC_SSCGR_INCSTEP_MASK, incstep) |
    FIELD_PREP(STM32F4_RCC_SSCGR_MODPER_MASK, modeper);
    if (ssc.mod_type)
    sscgr |= STM32F4_RCC_SSCGR_SPREADSEL;
    writel(sscgr, base + STM32F4_RCC_SSCGR);
    }
    static int stm32f4_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32f4_pll *pll = to_stm32f4_pll(gate);
    unsigned long n;
    unsigned long val;
    int pll_state;
    pll_state = stm32f4_pll_is_enabled(hw);
    if (pll_state)
    stm32f4_pll_disable(hw);
    n = rate  / parent_rate;
    val = readl(base + pll.offset) & ~STM32F4_RCC_PLLCFGR_N_MASK;
    val |= FIELD_PREP(STM32F4_RCC_PLLCFGR_N_MASK, n);
    writel(val, base + pll.offset);
    if (pll.ssc_enable)
    stm32f4_pll_set_ssc(hw, parent_rate, n);
    if (pll_state)
    stm32f4_pll_enable(hw);
    return 0;
    }
    static const struct clk_ops stm32f4_pll_gate_ops = {
    .enable		= stm32f4_pll_enable,
    .disable	= stm32f4_pll_disable,
    .is_enabled	= stm32f4_pll_is_enabled,
    .recalc_rate	= stm32f4_pll_recalc,
    .determine_rate = stm32f4_pll_determine_rate,
    .set_rate	= stm32f4_pll_set_rate,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_pll_div {
    pub div: clk_divider,
    pub hw_pll: *mut clk_hw,
}

    static unsigned long stm32f4_pll_div_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return clk_divider_ops.recalc_rate(hw, parent_rate);
    }
    static int stm32f4_pll_div_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return clk_divider_ops.determine_rate(hw, req);
    }
    static int stm32f4_pll_div_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    int pll_state, ret;
    struct clk_divider *div = to_clk_divider(hw);
    struct stm32f4_pll_div *pll_div = to_pll_div_clk(div);
    pll_state = stm32f4_pll_is_enabled(pll_div.hw_pll);
    if (pll_state)
    stm32f4_pll_disable(pll_div.hw_pll);
    ret = clk_divider_ops.set_rate(hw, rate, parent_rate);
    if (pll_state)
    stm32f4_pll_enable(pll_div.hw_pll);
    return ret;
    }
    static const struct clk_ops stm32f4_pll_div_ops = {
    .recalc_rate = stm32f4_pll_div_recalc_rate,
    .determine_rate = stm32f4_pll_div_determine_rate,
    .set_rate = stm32f4_pll_div_set_rate,
    };
    static struct clk_hw *clk_register_pll_div(const char *name,
    const char *parent_name, unsigned long flags,
    void __iomem *reg, u8 shift, u8 width,
    u8 clk_divider_flags, const struct clk_div_table *table,
    struct clk_hw *pll_hw, spinlock_t *lock)
    {
    struct stm32f4_pll_div *pll_div;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
// allocate the divider
    pll_div = kzalloc_obj(*pll_div);
    if (!pll_div)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &stm32f4_pll_div_ops;
    init.flags = flags;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
// struct clk_divider assignments
    pll_div.div.reg = reg;
    pll_div.div.shift = shift;
    pll_div.div.width = width;
    pll_div.div.flags = clk_divider_flags;
    pll_div.div.lock = lock;
    pll_div.div.table = table;
    pll_div.div.hw.init = &init;
    pll_div.hw_pll = pll_hw;
// register the clock
    hw = &pll_div.div.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pll_div);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    static int __init stm32f4_pll_init_ssc(struct clk_hw *hw,
    const struct stm32f4_pll_ssc *conf)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32f4_pll *pll = to_stm32f4_pll(gate);
    struct clk_hw *parent;
    unsigned long parent_rate;
    int pll_state;
    unsigned long n, val;
    parent = clk_hw_get_parent(hw);
    if (!parent) {
    pr_err("%s: failed to get clock parent\n", __func__);
    return -ENODEV;
    }
    parent_rate = clk_hw_get_rate(parent);
    pll.ssc_enable = true;
    memcpy(&pll.ssc_conf, conf, sizeof(pll.ssc_conf));
    pll_state = stm32f4_pll_is_enabled(hw);
    if (pll_state)
    stm32f4_pll_disable(hw);
    val = readl(base + pll.offset);
    n = FIELD_GET(STM32F4_RCC_PLLCFGR_N_MASK, val);
    pr_debug("%s: pll: %s, parent: %s, parent-rate: %lu, n: %lu\n",
    __func__, clk_hw_get_name(hw), clk_hw_get_name(parent),
    parent_rate, n);
    stm32f4_pll_set_ssc(hw, parent_rate, n);
    if (pll_state)
    stm32f4_pll_enable(hw);
    return 0;
    }
    static int __init stm32f4_pll_ssc_parse_dt(struct device_node *np,
    struct stm32f4_pll_ssc *conf)
    {
    int ret;
    if (!conf)
    return -EINVAL;
    ret = of_property_read_u32(np, "st,ssc-modfreq-hz", &conf.mod_freq);
    if (ret)
    return ret;
    ret = of_property_read_u32(np, "st,ssc-moddepth-permyriad",
    &conf.mod_depth);
    if (ret) {
    pr_err("%pOF: missing st,ssc-moddepth-permyriad\n", np);
    return ret;
    }
    ret = fwnode_property_match_property_string(of_fwnode_handle(np),
    "st,ssc-modmethod",
    stm32f4_ssc_mod_methods,
    ARRAY_SIZE(stm32f4_ssc_mod_methods));
    if (ret < 0) {
    pr_err("%pOF: failed to get st,ssc-modmethod\n", np);
    return ret;
    }
    conf.mod_type = ret;
    pr_debug("%pOF: SSCG settings: mod_freq: %d, mod_depth: %d mod_method: %s [%d]\n",
    np, conf.mod_freq, conf.mod_depth,
    stm32f4_ssc_mod_methods[ret], conf.mod_type);
    return 0;
    }
    static struct clk_hw *stm32f4_rcc_register_pll(const char *pllsrc,
    const struct stm32f4_pll_data *data,  spinlock_t *lock)
    {
    struct stm32f4_pll *pll;
    let mut init: clk_init_data = { core::ptr::null_mut() };
    void __iomem *reg;
    struct clk_hw *pll_hw;
    int ret;
    int i;
    const struct stm32f4_vco_data *vco;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    vco = &vco_data[data.pll_num];
    init.name = vco.vco_name;
    init.ops = &stm32f4_pll_gate_ops;
    init.flags = CLK_SET_RATE_GATE;
    init.parent_names = &pllsrc;
    init.num_parents = 1;
    pll.gate.lock = lock;
    pll.gate.reg = base + STM32F4_RCC_CR;
    pll.gate.bit_idx = vco.bit_idx;
    pll.gate.hw.init = &init;
    pll.offset = vco.offset;
    pll.n_start = data.n_start;
    pll.bit_rdy_idx = vco.bit_rdy_idx;
    pll.status = (readl(base + STM32F4_RCC_CR) >> vco.bit_idx) & 0x1;
    reg = base + pll.offset;
    pll_hw = &pll.gate.hw;
    ret = clk_hw_register(core::ptr::null_mut(), pll_hw);
    if (ret) {
    kfree(pll);
    return ERR_PTR(ret);
    }
    for (i = 0; i < MAX_PLL_DIV; i++)
    if (data.div_name[i])
    clk_register_pll_div(data.div_name[i],
    vco.vco_name,
    0,
    reg,
    div_data[i].shift,
    div_data[i].width,
    div_data[i].flag_div,
    div_data[i].div_table,
    pll_hw,
    lock);
    return pll_hw;
    }
//
// Converts the primary and secondary indices (as they appear in DT) to an
// offset into our struct clock array.
//
#[no_mangle]
unsafe extern "C" fn stm32f4_rcc_lookup_clk_idx(primary: u8, secondary: u8) -> c_int {
    static int stm32f4_rcc_lookup_clk_idx(u8 primary, u8 secondary)
    {
    u64 table[MAX_GATE_MAP];
    if (primary == 1) {
    if (WARN_ON(secondary >= stm32fx_end_primary_clk))
    return -EINVAL;
    return secondary;
    }
    memcpy(table, stm32f4_gate_map, sizeof(table));
// only bits set in table can be used as indices
    if (WARN_ON(secondary >= BITS_PER_BYTE * sizeof(table) ||
    0 == (table[BIT_ULL_WORD(secondary)] &
    BIT_ULL_MASK(secondary))))
    return -EINVAL;
// mask out bits above our current index
    table[BIT_ULL_WORD(secondary)] &=
    GENMASK_ULL(secondary % BITS_PER_LONG_LONG, 0);
    return stm32fx_end_primary_clk - 1 + hweight64(table[0]) +
    (BIT_ULL_WORD(secondary) >= 1 ? hweight64(table[1]) : 0) +
    (BIT_ULL_WORD(secondary) >= 2 ? hweight64(table[2]) : 0);
    }
    static struct clk_hw *
    stm32f4_rcc_lookup_clk(struct of_phandle_args *clkspec, void *data)
    {
    let mut i: c_int = stm32f4_rcc_lookup_clk_idx(clkspec.args[0], clkspec.args[1]);
    if (i < 0)
    return ERR_PTR(-EINVAL);
    return clks[i];
    }

#[no_mangle]
pub unsafe extern "C" fn disable_power_domain_write_protection() {
    static inline void disable_power_domain_write_protection(void)
    {
    if (pdrm)
    regmap_update_bits(pdrm, 0x00, (1 << 8), (1 << 8));
    }
#[no_mangle]
pub unsafe extern "C" fn enable_power_domain_write_protection() {
    static inline void enable_power_domain_write_protection(void)
    {
    if (pdrm)
    regmap_update_bits(pdrm, 0x00, (1 << 8), (0 << 8));
    }
#[no_mangle]
pub unsafe extern "C" fn sofware_reset_backup_domain() {
    static inline void sofware_reset_backup_domain(void)
    {
    unsigned long val;
    val = readl(base + STM32F4_RCC_BDCR);
    writel(val | BIT(16), base + STM32F4_RCC_BDCR);
    writel(val & ~BIT(16), base + STM32F4_RCC_BDCR);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_rgate {
    pub gate: clk_gate,
    pub bit_rdy_idx: u8,
}

pub const RGATE_TIMEOUT: c_int = 50000;
#[no_mangle]
unsafe extern "C" fn rgclk_enable(hw: *mut clk_hw) -> c_int {
    static int rgclk_enable(struct clk_hw *hw)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct stm32_rgate *rgate = to_rgclk(gate);
    int bit_status;
    let mut timeout: c_uint = RGATE_TIMEOUT;
    if (clk_gate_ops.is_enabled(hw))
    return 0;
    disable_power_domain_write_protection();
    clk_gate_ops.enable(hw);
    do {
    bit_status = !(readl(gate.reg) & BIT(rgate.bit_rdy_idx));
    if (bit_status)
    udelay(100);
    } while (bit_status && --timeout);
    enable_power_domain_write_protection();
    return bit_status;
    }
#[no_mangle]
unsafe extern "C" fn rgclk_disable(hw: *mut clk_hw) {
    static void rgclk_disable(struct clk_hw *hw)
    {
    clk_gate_ops.disable(hw);
    }
#[no_mangle]
unsafe extern "C" fn rgclk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int rgclk_is_enabled(struct clk_hw *hw)
    {
    return clk_gate_ops.is_enabled(hw);
    }
    static const struct clk_ops rgclk_ops = {
    .enable = rgclk_enable,
    .disable = rgclk_disable,
    .is_enabled = rgclk_is_enabled,
    };
    static struct clk_hw *clk_register_rgate(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags,
    void __iomem *reg, u8 bit_idx, u8 bit_rdy_idx,
    u8 clk_gate_flags, spinlock_t *lock)
    {
    struct stm32_rgate *rgate;
    let mut init: clk_init_data = { core::ptr::null_mut() };
    struct clk_hw *hw;
    int ret;
    rgate = kzalloc_obj(*rgate);
    if (!rgate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &rgclk_ops;
    init.flags = flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    rgate.bit_rdy_idx = bit_rdy_idx;
    rgate.gate.lock = lock;
    rgate.gate.reg = reg;
    rgate.gate.bit_idx = bit_idx;
    rgate.gate.hw.init = &init;
    hw = &rgate.gate.hw;
    ret = clk_hw_register(dev, hw);
    if (ret) {
    kfree(rgate);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn cclk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int cclk_gate_enable(struct clk_hw *hw)
    {
    int ret;
    disable_power_domain_write_protection();
    ret = clk_gate_ops.enable(hw);
    enable_power_domain_write_protection();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cclk_gate_disable(hw: *mut clk_hw) {
    static void cclk_gate_disable(struct clk_hw *hw)
    {
    disable_power_domain_write_protection();
    clk_gate_ops.disable(hw);
    enable_power_domain_write_protection();
    }
#[no_mangle]
unsafe extern "C" fn cclk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int cclk_gate_is_enabled(struct clk_hw *hw)
    {
    return clk_gate_ops.is_enabled(hw);
    }
    static const struct clk_ops cclk_gate_ops = {
    .enable		= cclk_gate_enable,
    .disable	= cclk_gate_disable,
    .is_enabled	= cclk_gate_is_enabled,
    };
#[no_mangle]
unsafe extern "C" fn cclk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 cclk_mux_get_parent(struct clk_hw *hw)
    {
    return clk_mux_ops.get_parent(hw);
    }
#[no_mangle]
unsafe extern "C" fn cclk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int cclk_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    int ret;
    disable_power_domain_write_protection();
    sofware_reset_backup_domain();
    ret = clk_mux_ops.set_parent(hw, index);
    enable_power_domain_write_protection();
    return ret;
    }
    static const struct clk_ops cclk_mux_ops = {
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .get_parent = cclk_mux_get_parent,
    .set_parent = cclk_mux_set_parent,
    };
    static struct clk_hw *stm32_register_cclk(struct device *dev, const char *name,
    const char * const *parent_names, int num_parents,
    void __iomem *reg, u8 bit_idx, u8 shift, unsigned long flags,
    spinlock_t *lock)
    {
    struct clk_hw *hw;
    struct clk_gate *gate;
    struct clk_mux *mux;
    gate = kzalloc_obj(*gate);
    if (!gate) {
    hw = ERR_PTR(-EINVAL);
    goto fail;
    }
    mux = kzalloc_obj(*mux);
    if (!mux) {
    kfree(gate);
    hw = ERR_PTR(-EINVAL);
    goto fail;
    }
    gate.reg = reg;
    gate.bit_idx = bit_idx;
    gate.flags = 0;
    gate.lock = lock;
    mux.reg = reg;
    mux.shift = shift;
    mux.mask = 3;
    mux.flags = 0;
    hw = clk_hw_register_composite(dev, name, parent_names, num_parents,
    &mux.hw, &cclk_mux_ops,
    core::ptr::null_mut(), core::ptr::null_mut(),
    &gate.hw, &cclk_gate_ops,
    flags);
    if (IS_ERR(hw)) {
    kfree(gate);
    kfree(mux);
    }
    fail:
    return hw;
    }
    static const char *sys_parents[] __initdata =   { "hsi", core::ptr::null_mut(), "pll" };
    static const struct clk_div_table ahb_div_table[] = {
    { 0x0,   1 }, { 0x1,   1 }, { 0x2,   1 }, { 0x3,   1 },
    { 0x4,   1 }, { 0x5,   1 }, { 0x6,   1 }, { 0x7,   1 },
    { 0x8,   2 }, { 0x9,   4 }, { 0xa,   8 }, { 0xb,  16 },
    { 0xc,  64 }, { 0xd, 128 }, { 0xe, 256 }, { 0xf, 512 },
    { 0 },
    };
    static const struct clk_div_table apb_div_table[] = {
    { 0,  1 }, { 0,  1 }, { 0,  1 }, { 0,  1 },
    { 4,  2 }, { 5,  4 }, { 6,  8 }, { 7, 16 },
    { 0 },
    };
    static const char *rtc_parents[4] = {
    "no-clock", "lse", "lsi", "hse-rtc"
    };
    static const char *pll_src = "pll-src";
    static const char *pllsrc_parent[2] = { "hsi", core::ptr::null_mut() };
    static const char *dsi_parent[2] = { core::ptr::null_mut(), "pll-r" };
    static const char *lcd_parent[1] = { "pllsai-r-div" };
    static const char *i2s_parents[2] = { "plli2s-r", core::ptr::null_mut() };
    static const char *sai_parents[4] = { "pllsai-q-div", "plli2s-q-div", core::ptr::null_mut(),
    "no-clock" };
    static const char *pll48_parents[2] = { "pll-q", "pllsai-p" };
    static const char *sdmux_parents[2] = { "pll48", "sys" };
    static const char *hdmi_parents[2] = { "lse", "hsi_div488" };
    static const char *spdif_parent[1] = { "plli2s-p" };
    static const char *lptim_parent[4] = { "apb1_mul", "lsi", "hsi", "lse" };
    static const char *uart_parents1[4] = { "apb2_div", "sys", "hsi", "lse" };
    static const char *uart_parents2[4] = { "apb1_div", "sys", "hsi", "lse" };
    static const char *i2c_parents[4] = { "apb1_div", "sys", "hsi", "no-clock" };
    static const char * const dfsdm1_src[] = { "apb2_div", "sys" };
    static const char * const adsfdm1_parent[] = { "sai1_clk", "sai2_clk" };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_aux_clk {
    pub idx: c_int,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: c_int,
    pub offset_mux: c_int,
    pub shift: u8,
    pub mask: u8,
    pub offset_gate: c_int,
    pub bit_idx: u8,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32f4_clk_data {
    pub gates_data: *const stm32f4_gate_data,
    pub gates_map: *const u64,
    pub gates_num: c_int,
    pub pll_data: *const stm32f4_pll_data,
    pub aux_clk: *const stm32_aux_clk,
    pub aux_clk_num: c_int,
    pub end_primary: c_int,
}

    static const struct stm32_aux_clk stm32f429_aux_clk[] = {
    {
    CLK_LCD, "lcd-tft", lcd_parent, ARRAY_SIZE(lcd_parent),
    NO_MUX, 0, 0,
    STM32F4_RCC_APB2ENR, 26,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_I2S, "i2s", i2s_parents, ARRAY_SIZE(i2s_parents),
    STM32F4_RCC_CFGR, 23, 1,
    NO_GATE, 0,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI1, "sai1-a", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 20, 3,
    STM32F4_RCC_APB2ENR, 22,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI2, "sai1-b", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 22, 3,
    STM32F4_RCC_APB2ENR, 22,
    CLK_SET_RATE_PARENT
    },
    };
    static const struct stm32_aux_clk stm32f469_aux_clk[] = {
    {
    CLK_LCD, "lcd-tft", lcd_parent, ARRAY_SIZE(lcd_parent),
    NO_MUX, 0, 0,
    STM32F4_RCC_APB2ENR, 26,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_I2S, "i2s", i2s_parents, ARRAY_SIZE(i2s_parents),
    STM32F4_RCC_CFGR, 23, 1,
    NO_GATE, 0,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI1, "sai1-a", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 20, 3,
    STM32F4_RCC_APB2ENR, 22,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI2, "sai1-b", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 22, 3,
    STM32F4_RCC_APB2ENR, 22,
    CLK_SET_RATE_PARENT
    },
    {
    NO_IDX, "pll48", pll48_parents, ARRAY_SIZE(pll48_parents),
    STM32F4_RCC_DCKCFGR, 27, 1,
    NO_GATE, 0,
    0
    },
    {
    NO_IDX, "sdmux", sdmux_parents, ARRAY_SIZE(sdmux_parents),
    STM32F4_RCC_DCKCFGR, 28, 1,
    NO_GATE, 0,
    0
    },
    {
    CLK_F469_DSI, "dsi", dsi_parent, ARRAY_SIZE(dsi_parent),
    STM32F4_RCC_DCKCFGR, 29, 1,
    STM32F4_RCC_APB2ENR, 27,
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT
    },
    };
    static const struct stm32_aux_clk stm32f746_aux_clk[] = {
    {
    CLK_LCD, "lcd-tft", lcd_parent, ARRAY_SIZE(lcd_parent),
    NO_MUX, 0, 0,
    STM32F4_RCC_APB2ENR, 26,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_I2S, "i2s", i2s_parents, ARRAY_SIZE(i2s_parents),
    STM32F4_RCC_CFGR, 23, 1,
    NO_GATE, 0,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI1, "sai1_clk", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 20, 3,
    STM32F4_RCC_APB2ENR, 22,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI2, "sai2_clk", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 22, 3,
    STM32F4_RCC_APB2ENR, 23,
    CLK_SET_RATE_PARENT
    },
    {
    NO_IDX, "pll48", pll48_parents, ARRAY_SIZE(pll48_parents),
    STM32F7_RCC_DCKCFGR2, 27, 1,
    NO_GATE, 0,
    0
    },
    {
    NO_IDX, "sdmux", sdmux_parents, ARRAY_SIZE(sdmux_parents),
    STM32F7_RCC_DCKCFGR2, 28, 1,
    NO_GATE, 0,
    0
    },
    {
    CLK_HDMI_CEC, "hdmi-cec",
    hdmi_parents, ARRAY_SIZE(hdmi_parents),
    STM32F7_RCC_DCKCFGR2, 26, 1,
    NO_GATE, 0,
    0
    },
    {
    CLK_SPDIF, "spdif-rx",
    spdif_parent, ARRAY_SIZE(spdif_parent),
    STM32F7_RCC_DCKCFGR2, 22, 3,
    STM32F4_RCC_APB2ENR, 23,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_USART1, "usart1",
    uart_parents1, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 0, 3,
    STM32F4_RCC_APB2ENR, 4,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_USART2, "usart2",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 2, 3,
    STM32F4_RCC_APB1ENR, 17,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_USART3, "usart3",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 4, 3,
    STM32F4_RCC_APB1ENR, 18,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART4, "uart4",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 6, 3,
    STM32F4_RCC_APB1ENR, 19,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART5, "uart5",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 8, 3,
    STM32F4_RCC_APB1ENR, 20,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_USART6, "usart6",
    uart_parents1, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 10, 3,
    STM32F4_RCC_APB2ENR, 5,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART7, "uart7",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 12, 3,
    STM32F4_RCC_APB1ENR, 30,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART8, "uart8",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 14, 3,
    STM32F4_RCC_APB1ENR, 31,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C1, "i2c1",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 16, 3,
    STM32F4_RCC_APB1ENR, 21,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C2, "i2c2",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 18, 3,
    STM32F4_RCC_APB1ENR, 22,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C3, "i2c3",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 20, 3,
    STM32F4_RCC_APB1ENR, 23,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C4, "i2c4",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 22, 3,
    STM32F4_RCC_APB1ENR, 24,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_LPTIMER, "lptim1",
    lptim_parent, ARRAY_SIZE(lptim_parent),
    STM32F7_RCC_DCKCFGR2, 24, 3,
    STM32F4_RCC_APB1ENR, 9,
    CLK_SET_RATE_PARENT
    },
    };
    static const struct stm32_aux_clk stm32f769_aux_clk[] = {
    {
    CLK_LCD, "lcd-tft", lcd_parent, ARRAY_SIZE(lcd_parent),
    NO_MUX, 0, 0,
    STM32F4_RCC_APB2ENR, 26,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_I2S, "i2s", i2s_parents, ARRAY_SIZE(i2s_parents),
    STM32F4_RCC_CFGR, 23, 1,
    NO_GATE, 0,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI1, "sai1_clk", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 20, 3,
    STM32F4_RCC_APB2ENR, 22,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_SAI2, "sai2_clk", sai_parents, ARRAY_SIZE(sai_parents),
    STM32F4_RCC_DCKCFGR, 22, 3,
    STM32F4_RCC_APB2ENR, 23,
    CLK_SET_RATE_PARENT
    },
    {
    NO_IDX, "pll48", pll48_parents, ARRAY_SIZE(pll48_parents),
    STM32F7_RCC_DCKCFGR2, 27, 1,
    NO_GATE, 0,
    0
    },
    {
    NO_IDX, "sdmux1", sdmux_parents, ARRAY_SIZE(sdmux_parents),
    STM32F7_RCC_DCKCFGR2, 28, 1,
    NO_GATE, 0,
    0
    },
    {
    NO_IDX, "sdmux2", sdmux_parents, ARRAY_SIZE(sdmux_parents),
    STM32F7_RCC_DCKCFGR2, 29, 1,
    NO_GATE, 0,
    0
    },
    {
    CLK_HDMI_CEC, "hdmi-cec",
    hdmi_parents, ARRAY_SIZE(hdmi_parents),
    STM32F7_RCC_DCKCFGR2, 26, 1,
    NO_GATE, 0,
    0
    },
    {
    CLK_SPDIF, "spdif-rx",
    spdif_parent, ARRAY_SIZE(spdif_parent),
    STM32F7_RCC_DCKCFGR2, 22, 3,
    STM32F4_RCC_APB2ENR, 23,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_USART1, "usart1",
    uart_parents1, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 0, 3,
    STM32F4_RCC_APB2ENR, 4,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_USART2, "usart2",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 2, 3,
    STM32F4_RCC_APB1ENR, 17,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_USART3, "usart3",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 4, 3,
    STM32F4_RCC_APB1ENR, 18,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART4, "uart4",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 6, 3,
    STM32F4_RCC_APB1ENR, 19,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART5, "uart5",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 8, 3,
    STM32F4_RCC_APB1ENR, 20,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_USART6, "usart6",
    uart_parents1, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 10, 3,
    STM32F4_RCC_APB2ENR, 5,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART7, "uart7",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 12, 3,
    STM32F4_RCC_APB1ENR, 30,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_UART8, "uart8",
    uart_parents2, ARRAY_SIZE(uart_parents1),
    STM32F7_RCC_DCKCFGR2, 14, 3,
    STM32F4_RCC_APB1ENR, 31,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C1, "i2c1",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 16, 3,
    STM32F4_RCC_APB1ENR, 21,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C2, "i2c2",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 18, 3,
    STM32F4_RCC_APB1ENR, 22,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C3, "i2c3",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 20, 3,
    STM32F4_RCC_APB1ENR, 23,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_I2C4, "i2c4",
    i2c_parents, ARRAY_SIZE(i2c_parents),
    STM32F7_RCC_DCKCFGR2, 22, 3,
    STM32F4_RCC_APB1ENR, 24,
    CLK_SET_RATE_PARENT,
    },
    {
    CLK_LPTIMER, "lptim1",
    lptim_parent, ARRAY_SIZE(lptim_parent),
    STM32F7_RCC_DCKCFGR2, 24, 3,
    STM32F4_RCC_APB1ENR, 9,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_F769_DSI, "dsi",
    dsi_parent, ARRAY_SIZE(dsi_parent),
    STM32F7_RCC_DCKCFGR2, 0, 1,
    STM32F4_RCC_APB2ENR, 27,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_DFSDM1, "dfsdm1",
    dfsdm1_src, ARRAY_SIZE(dfsdm1_src),
    STM32F4_RCC_DCKCFGR, 25, 1,
    STM32F4_RCC_APB2ENR, 29,
    CLK_SET_RATE_PARENT
    },
    {
    CLK_ADFSDM1, "adfsdm1",
    adsfdm1_parent, ARRAY_SIZE(adsfdm1_parent),
    STM32F4_RCC_DCKCFGR, 26, 1,
    STM32F4_RCC_APB2ENR, 29,
    CLK_SET_RATE_PARENT
    },
    };
    static const struct stm32f4_clk_data stm32f429_clk_data = {
    .end_primary	= END_PRIMARY_CLK,
    .gates_data	= stm32f429_gates,
    .gates_map	= stm32f42xx_gate_map,
    .gates_num	= ARRAY_SIZE(stm32f429_gates),
    .pll_data	= stm32f429_pll,
    .aux_clk	= stm32f429_aux_clk,
    .aux_clk_num	= ARRAY_SIZE(stm32f429_aux_clk),
    };
    static const struct stm32f4_clk_data stm32f469_clk_data = {
    .end_primary	= END_PRIMARY_CLK,
    .gates_data	= stm32f469_gates,
    .gates_map	= stm32f46xx_gate_map,
    .gates_num	= ARRAY_SIZE(stm32f469_gates),
    .pll_data	= stm32f469_pll,
    .aux_clk	= stm32f469_aux_clk,
    .aux_clk_num	= ARRAY_SIZE(stm32f469_aux_clk),
    };
    static const struct stm32f4_clk_data stm32f746_clk_data = {
    .end_primary	= END_PRIMARY_CLK_F7,
    .gates_data	= stm32f746_gates,
    .gates_map	= stm32f746_gate_map,
    .gates_num	= ARRAY_SIZE(stm32f746_gates),
    .pll_data	= stm32f469_pll,
    .aux_clk	= stm32f746_aux_clk,
    .aux_clk_num	= ARRAY_SIZE(stm32f746_aux_clk),
    };
    static const struct stm32f4_clk_data stm32f769_clk_data = {
    .end_primary	= END_PRIMARY_CLK_F7,
    .gates_data	= stm32f769_gates,
    .gates_map	= stm32f769_gate_map,
    .gates_num	= ARRAY_SIZE(stm32f769_gates),
    .pll_data	= stm32f469_pll,
    .aux_clk	= stm32f769_aux_clk,
    .aux_clk_num	= ARRAY_SIZE(stm32f769_aux_clk),
    };
    static const struct of_device_id stm32f4_of_match[] = {
    {
    .compatible = "st,stm32f42xx-rcc",
    .data = &stm32f429_clk_data
    },
    {
    .compatible = "st,stm32f469-rcc",
    .data = &stm32f469_clk_data
    },
    {
    .compatible = "st,stm32f746-rcc",
    .data = &stm32f746_clk_data
    },
    {
    .compatible = "st,stm32f769-rcc",
    .data = &stm32f769_clk_data
    },
    {}
    };
    static struct clk_hw *stm32_register_aux_clk(const char *name,
    const char * const *parent_names, int num_parents,
    int offset_mux, u8 shift, u8 mask,
    int offset_gate, u8 bit_idx,
    unsigned long flags, spinlock_t *lock)
    {
    struct clk_hw *hw;
    struct clk_gate *gate = core::ptr::null_mut();
    struct clk_mux *mux = core::ptr::null_mut();
    struct clk_hw *mux_hw = core::ptr::null_mut(), *gate_hw = core::ptr::null_mut();
    const struct clk_ops *mux_ops = core::ptr::null_mut(), *gate_ops = core::ptr::null_mut();
    if (offset_gate != NO_GATE) {
    gate = kzalloc_obj(*gate);
    if (!gate) {
    hw = ERR_PTR(-EINVAL);
    goto fail;
    }
    gate.reg = base + offset_gate;
    gate.bit_idx = bit_idx;
    gate.flags = 0;
    gate.lock = lock;
    gate_hw = &gate.hw;
    gate_ops = &clk_gate_ops;
    }
    if (offset_mux != NO_MUX) {
    mux = kzalloc_obj(*mux);
    if (!mux) {
    hw = ERR_PTR(-EINVAL);
    goto fail;
    }
    mux.reg = base + offset_mux;
    mux.shift = shift;
    mux.mask = mask;
    mux.flags = 0;
    mux_hw = &mux.hw;
    mux_ops = &clk_mux_ops;
    }
    if (mux_hw == core::ptr::null_mut() && gate_hw == core::ptr::null_mut()) {
    hw = ERR_PTR(-EINVAL);
    goto fail;
    }
    hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
    mux_hw, mux_ops,
    core::ptr::null_mut(), core::ptr::null_mut(),
    gate_hw, gate_ops,
    flags);
    fail:
    if (IS_ERR(hw)) {
    kfree(gate);
    kfree(mux);
    }
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn stm32f4_rcc_init(np: *mut device_node) -> void __init {
    static void __init stm32f4_rcc_init(struct device_node *np)
    {
    const char *hse_clk, *i2s_in_clk;
    int n;
    const struct of_device_id *match;
    const struct stm32f4_clk_data *data;
    unsigned long pllm;
    struct clk_hw *pll_src_hw, *pll_vco_hw;
    struct stm32f4_pll_ssc ssc_conf;
    base = of_iomap(np, 0);
    if (!base) {
    pr_err("%pOFn: unable to map resource\n", np);
    return;
    }
    pdrm = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(pdrm)) {
    pdrm = core::ptr::null_mut();
    pr_warn("%s: Unable to get syscfg\n", __func__);
    }
    match = of_match_node(stm32f4_of_match, np);
    if (WARN_ON(!match))
    return;
    data = match.data;
    stm32fx_end_primary_clk = data.end_primary;
    clks = kmalloc_objs(*clks, data.gates_num + stm32fx_end_primary_clk);
    if (!clks)
    goto fail;
    stm32f4_gate_map = data.gates_map;
    hse_clk = of_clk_get_parent_name(np, 0);
    dsi_parent[0] = hse_clk;
    pllsrc_parent[1] = hse_clk;
    i2s_in_clk = of_clk_get_parent_name(np, 1);
    i2s_parents[1] = i2s_in_clk;
    sai_parents[2] = i2s_in_clk;
    if (of_device_is_compatible(np, "st,stm32f769-rcc")) {
    clk_hw_register_gate(core::ptr::null_mut(), "dfsdm1_apb", "apb2_div", 0,
    base + STM32F4_RCC_APB2ENR, 29,
    CLK_IGNORE_UNUSED, &stm32f4_clk_lock);
    dsi_parent[0] = pll_src;
    sai_parents[3] = pll_src;
    }
    clks[CLK_HSI] = clk_hw_register_fixed_rate_with_accuracy(core::ptr::null_mut(), "hsi",
    core::ptr::null_mut(), 0, 16000000, 160000);
    pll_src_hw = clk_hw_register_mux(core::ptr::null_mut(), pll_src, pllsrc_parent,
    ARRAY_SIZE(pllsrc_parent), 0,
    base + STM32F4_RCC_PLLCFGR, 22, 1, 0,
    &stm32f4_clk_lock);
    pllm = readl(base + STM32F4_RCC_PLLCFGR) & 0x3f;
    clk_hw_register_fixed_factor(core::ptr::null_mut(), "vco_in", pll_src,
    0, 1, pllm);
    pll_vco_hw = stm32f4_rcc_register_pll("vco_in", &data.pll_data[0],
    &stm32f4_clk_lock);
    clks[PLL_VCO_I2S] = stm32f4_rcc_register_pll("vco_in",
    &data.pll_data[1], &stm32f4_clk_lock);
    clks[PLL_VCO_SAI] = stm32f4_rcc_register_pll("vco_in",
    &data.pll_data[2], &stm32f4_clk_lock);
    for (n = 0; n < MAX_POST_DIV; n++) {
    const struct stm32f4_pll_post_div_data *post_div;
    struct clk_hw *hw;
    post_div = &post_div_data[n];
    hw = clk_register_pll_div(post_div.name,
    post_div.parent,
    post_div.flag,
    base + post_div.offset,
    post_div.shift,
    post_div.width,
    post_div.flag_div,
    post_div.div_table,
    clks[post_div.pll_idx],
    &stm32f4_clk_lock);
    if (post_div.idx != NO_IDX)
    clks[post_div.idx] = hw;
    }
    sys_parents[1] = hse_clk;
    clks[CLK_SYSCLK] = clk_hw_register_mux_table(
    core::ptr::null_mut(), "sys", sys_parents, ARRAY_SIZE(sys_parents), 0,
    base + STM32F4_RCC_CFGR, 0, 3, 0, core::ptr::null_mut(), &stm32f4_clk_lock);
    clk_register_divider_table(core::ptr::null_mut(), "ahb_div", "sys",
    CLK_SET_RATE_PARENT, base + STM32F4_RCC_CFGR,
    4, 4, 0, ahb_div_table, &stm32f4_clk_lock);
    clk_register_divider_table(core::ptr::null_mut(), "apb1_div", "ahb_div",
    CLK_SET_RATE_PARENT, base + STM32F4_RCC_CFGR,
    10, 3, 0, apb_div_table, &stm32f4_clk_lock);
    clk_register_apb_mul(core::ptr::null_mut(), "apb1_mul", "apb1_div",
    CLK_SET_RATE_PARENT, 12);
    clk_register_divider_table(core::ptr::null_mut(), "apb2_div", "ahb_div",
    CLK_SET_RATE_PARENT, base + STM32F4_RCC_CFGR,
    13, 3, 0, apb_div_table, &stm32f4_clk_lock);
    clk_register_apb_mul(core::ptr::null_mut(), "apb2_mul", "apb2_div",
    CLK_SET_RATE_PARENT, 15);
    clks[SYSTICK] = clk_hw_register_fixed_factor(core::ptr::null_mut(), "systick", "ahb_div",
    0, 1, 8);
    clks[FCLK] = clk_hw_register_fixed_factor(core::ptr::null_mut(), "fclk", "ahb_div",
    0, 1, 1);
    for (n = 0; n < data.gates_num; n++) {
    const struct stm32f4_gate_data *gd;
    unsigned int secondary;
    int idx;
    gd = &data.gates_data[n];
    secondary = 8 * (gd.offset - STM32F4_RCC_AHB1ENR) +
    gd.bit_idx;
    idx = stm32f4_rcc_lookup_clk_idx(0, secondary);
    if (idx < 0)
    goto fail;
    clks[idx] = clk_hw_register_gate(
    core::ptr::null_mut(), gd.name, gd.parent_name, gd.flags,
    base + gd.offset, gd.bit_idx, 0, &stm32f4_clk_lock);
    if (IS_ERR(clks[idx])) {
    pr_err("%pOF: Unable to register leaf clock %s\n",
    np, gd.name);
    goto fail;
    }
    }
    clks[CLK_LSI] = clk_register_rgate(core::ptr::null_mut(), "lsi", "clk-lsi", 0,
    base + STM32F4_RCC_CSR, 0, 1, 0, &stm32f4_clk_lock);
    if (IS_ERR(clks[CLK_LSI])) {
    pr_err("Unable to register lsi clock\n");
    goto fail;
    }
    clks[CLK_LSE] = clk_register_rgate(core::ptr::null_mut(), "lse", "clk-lse", 0,
    base + STM32F4_RCC_BDCR, 0, 1, 0, &stm32f4_clk_lock);
    if (IS_ERR(clks[CLK_LSE])) {
    pr_err("Unable to register lse clock\n");
    goto fail;
    }
    clks[CLK_HSE_RTC] = clk_hw_register_divider(core::ptr::null_mut(), "hse-rtc", "clk-hse",
    0, base + STM32F4_RCC_CFGR, 16, 5, 0,
    &stm32f4_clk_lock);
    if (IS_ERR(clks[CLK_HSE_RTC])) {
    pr_err("Unable to register hse-rtc clock\n");
    goto fail;
    }
    clks[CLK_RTC] = stm32_register_cclk(core::ptr::null_mut(), "rtc", rtc_parents, 4,
    base + STM32F4_RCC_BDCR, 15, 8, 0, &stm32f4_clk_lock);
    if (IS_ERR(clks[CLK_RTC])) {
    pr_err("Unable to register rtc clock\n");
    goto fail;
    }
    for (n = 0; n < data.aux_clk_num; n++) {
    const struct stm32_aux_clk *aux_clk;
    struct clk_hw *hw;
    aux_clk = &data.aux_clk[n];
    hw = stm32_register_aux_clk(aux_clk.name,
    aux_clk.parent_names, aux_clk.num_parents,
    aux_clk.offset_mux, aux_clk.shift,
    aux_clk.mask, aux_clk.offset_gate,
    aux_clk.bit_idx, aux_clk.flags,
    &stm32f4_clk_lock);
    if (IS_ERR(hw)) {
    pr_warn("Unable to register %s clk\n", aux_clk.name);
    continue;
    }
    if (aux_clk.idx != NO_IDX)
    clks[aux_clk.idx] = hw;
    }
    if (of_device_is_compatible(np, "st,stm32f746-rcc")) {
    clk_hw_register_fixed_factor(core::ptr::null_mut(), "hsi_div488", "hsi", 0,
    1, 488);
    clks[CLK_PLL_SRC] = pll_src_hw;
    }
    of_clk_add_hw_provider(np, stm32f4_rcc_lookup_clk, core::ptr::null_mut());
    if (!stm32f4_pll_ssc_parse_dt(np, &ssc_conf))
    stm32f4_pll_init_ssc(pll_vco_hw, &ssc_conf);
    return;
    fail:
    kfree(clks);
    iounmap(base);
    }
    CLK_OF_DECLARE_DRIVER(stm32f42xx_rcc, "st,stm32f42xx-rcc", stm32f4_rcc_init);
    CLK_OF_DECLARE_DRIVER(stm32f46xx_rcc, "st,stm32f469-rcc", stm32f4_rcc_init);
    CLK_OF_DECLARE_DRIVER(stm32f746_rcc, "st,stm32f746-rcc", stm32f4_rcc_init);
    CLK_OF_DECLARE_DRIVER(stm32f769_rcc, "st,stm32f769-rcc", stm32f4_rcc_init);
