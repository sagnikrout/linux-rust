//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/lpc18xx-ccu.h
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


//
// Copyright (c) 2015 Joachim Eastwood <manabian@gmail.com>
//
// This code is released using a dual license strategy: BSD/GPL
// You can choose the licence that better fits your requirements.
//
// Released under the terms of 3-clause BSD License
// Released under the terms of GNU General Public License Version 2.0
//
// Clock Control Unit 1 (CCU1) clock offsets
pub const CLK_APB3_BUS: c_uint = 0x100;
pub const CLK_APB3_I2C1: c_uint = 0x108;
pub const CLK_APB3_DAC: c_uint = 0x110;
pub const CLK_APB3_ADC0: c_uint = 0x118;
pub const CLK_APB3_ADC1: c_uint = 0x120;
pub const CLK_APB3_CAN0: c_uint = 0x128;
pub const CLK_APB1_BUS: c_uint = 0x200;
pub const CLK_APB1_MOTOCON_PWM: c_uint = 0x208;
pub const CLK_APB1_I2C0: c_uint = 0x210;
pub const CLK_APB1_I2S: c_uint = 0x218;
pub const CLK_APB1_CAN1: c_uint = 0x220;
pub const CLK_SPIFI: c_uint = 0x300;
pub const CLK_CPU_BUS: c_uint = 0x400;
pub const CLK_CPU_SPIFI: c_uint = 0x408;
pub const CLK_CPU_GPIO: c_uint = 0x410;
pub const CLK_CPU_LCD: c_uint = 0x418;
pub const CLK_CPU_ETHERNET: c_uint = 0x420;
pub const CLK_CPU_USB0: c_uint = 0x428;
pub const CLK_CPU_EMC: c_uint = 0x430;
pub const CLK_CPU_SDIO: c_uint = 0x438;
pub const CLK_CPU_DMA: c_uint = 0x440;
pub const CLK_CPU_CORE: c_uint = 0x448;
pub const CLK_CPU_SCT: c_uint = 0x468;
pub const CLK_CPU_USB1: c_uint = 0x470;
pub const CLK_CPU_EMCDIV: c_uint = 0x478;
pub const CLK_CPU_FLASHA: c_uint = 0x480;
pub const CLK_CPU_FLASHB: c_uint = 0x488;
pub const CLK_CPU_M0APP: c_uint = 0x490;
pub const CLK_CPU_ADCHS: c_uint = 0x498;
pub const CLK_CPU_EEPROM: c_uint = 0x4a0;
pub const CLK_CPU_WWDT: c_uint = 0x500;
pub const CLK_CPU_UART0: c_uint = 0x508;
pub const CLK_CPU_UART1: c_uint = 0x510;
pub const CLK_CPU_SSP0: c_uint = 0x518;
pub const CLK_CPU_TIMER0: c_uint = 0x520;
pub const CLK_CPU_TIMER1: c_uint = 0x528;
pub const CLK_CPU_SCU: c_uint = 0x530;
pub const CLK_CPU_CREG: c_uint = 0x538;
pub const CLK_CPU_RITIMER: c_uint = 0x600;
pub const CLK_CPU_UART2: c_uint = 0x608;
pub const CLK_CPU_UART3: c_uint = 0x610;
pub const CLK_CPU_TIMER2: c_uint = 0x618;
pub const CLK_CPU_TIMER3: c_uint = 0x620;
pub const CLK_CPU_SSP1: c_uint = 0x628;
pub const CLK_CPU_QEI: c_uint = 0x630;
pub const CLK_PERIPH_BUS: c_uint = 0x700;
pub const CLK_PERIPH_CORE: c_uint = 0x710;
pub const CLK_PERIPH_SGPIO: c_uint = 0x718;
pub const CLK_USB0: c_uint = 0x800;
pub const CLK_USB1: c_uint = 0x900;
pub const CLK_SPI: c_uint = 0xA00;
pub const CLK_ADCHS: c_uint = 0xB00;
// Clock Control Unit 2 (CCU2) clock offsets
pub const CLK_AUDIO: c_uint = 0x100;
pub const CLK_APB2_UART3: c_uint = 0x200;
pub const CLK_APB2_UART2: c_uint = 0x300;
pub const CLK_APB0_UART1: c_uint = 0x400;
pub const CLK_APB0_UART0: c_uint = 0x500;
pub const CLK_APB2_SSP1: c_uint = 0x600;
pub const CLK_APB0_SSP0: c_uint = 0x700;
pub const CLK_SDIO: c_uint = 0x800;
