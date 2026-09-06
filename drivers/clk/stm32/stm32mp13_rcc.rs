//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/stm32/stm32mp13_rcc.h
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
//
// Copyright (C) 2020, STMicroelectronics - All Rights Reserved
//
// Configuration settings for the STM32MP13x CPU
//
// RCC registers
pub const RCC_SECCFGR: c_uint = 0x0;
pub const RCC_MP_SREQSETR: c_uint = 0x100;
pub const RCC_MP_SREQCLRR: c_uint = 0x104;
pub const RCC_MP_APRSTCR: c_uint = 0x108;
pub const RCC_MP_APRSTSR: c_uint = 0x10c;
pub const RCC_PWRLPDLYCR: c_uint = 0x110;
pub const RCC_MP_GRSTCSETR: c_uint = 0x114;
pub const RCC_BR_RSTSCLRR: c_uint = 0x118;
pub const RCC_MP_RSTSSETR: c_uint = 0x11c;
pub const RCC_MP_RSTSCLRR: c_uint = 0x120;
pub const RCC_MP_IWDGFZSETR: c_uint = 0x124;
pub const RCC_MP_IWDGFZCLRR: c_uint = 0x128;
pub const RCC_MP_CIER: c_uint = 0x200;
pub const RCC_MP_CIFR: c_uint = 0x204;
pub const RCC_BDCR: c_uint = 0x400;
pub const RCC_RDLSICR: c_uint = 0x404;
pub const RCC_OCENSETR: c_uint = 0x420;
pub const RCC_OCENCLRR: c_uint = 0x424;
pub const RCC_OCRDYR: c_uint = 0x428;
pub const RCC_HSICFGR: c_uint = 0x440;
pub const RCC_CSICFGR: c_uint = 0x444;
pub const RCC_MCO1CFGR: c_uint = 0x460;
pub const RCC_MCO2CFGR: c_uint = 0x464;
pub const RCC_DBGCFGR: c_uint = 0x468;
pub const RCC_RCK12SELR: c_uint = 0x480;
pub const RCC_RCK3SELR: c_uint = 0x484;
pub const RCC_RCK4SELR: c_uint = 0x488;
pub const RCC_PLL1CR: c_uint = 0x4a0;
pub const RCC_PLL1CFGR1: c_uint = 0x4a4;
pub const RCC_PLL1CFGR2: c_uint = 0x4a8;
pub const RCC_PLL1FRACR: c_uint = 0x4ac;
pub const RCC_PLL1CSGR: c_uint = 0x4b0;
pub const RCC_PLL2CR: c_uint = 0x4d0;
pub const RCC_PLL2CFGR1: c_uint = 0x4d4;
pub const RCC_PLL2CFGR2: c_uint = 0x4d8;
pub const RCC_PLL2FRACR: c_uint = 0x4dc;
pub const RCC_PLL2CSGR: c_uint = 0x4e0;
pub const RCC_PLL3CR: c_uint = 0x500;
pub const RCC_PLL3CFGR1: c_uint = 0x504;
pub const RCC_PLL3CFGR2: c_uint = 0x508;
pub const RCC_PLL3FRACR: c_uint = 0x50c;
pub const RCC_PLL3CSGR: c_uint = 0x510;
pub const RCC_PLL4CR: c_uint = 0x520;
pub const RCC_PLL4CFGR1: c_uint = 0x524;
pub const RCC_PLL4CFGR2: c_uint = 0x528;
pub const RCC_PLL4FRACR: c_uint = 0x52c;
pub const RCC_PLL4CSGR: c_uint = 0x530;
pub const RCC_MPCKSELR: c_uint = 0x540;
pub const RCC_ASSCKSELR: c_uint = 0x544;
pub const RCC_MSSCKSELR: c_uint = 0x548;
pub const RCC_CPERCKSELR: c_uint = 0x54c;
pub const RCC_RTCDIVR: c_uint = 0x560;
pub const RCC_MPCKDIVR: c_uint = 0x564;
pub const RCC_AXIDIVR: c_uint = 0x568;
pub const RCC_MLAHBDIVR: c_uint = 0x56c;
pub const RCC_APB1DIVR: c_uint = 0x570;
pub const RCC_APB2DIVR: c_uint = 0x574;
pub const RCC_APB3DIVR: c_uint = 0x578;
pub const RCC_APB4DIVR: c_uint = 0x57c;
pub const RCC_APB5DIVR: c_uint = 0x580;
pub const RCC_APB6DIVR: c_uint = 0x584;
pub const RCC_TIMG1PRER: c_uint = 0x5a0;
pub const RCC_TIMG2PRER: c_uint = 0x5a4;
pub const RCC_TIMG3PRER: c_uint = 0x5a8;
pub const RCC_DDRITFCR: c_uint = 0x5c0;
pub const RCC_I2C12CKSELR: c_uint = 0x600;
pub const RCC_I2C345CKSELR: c_uint = 0x604;
pub const RCC_SPI2S1CKSELR: c_uint = 0x608;
pub const RCC_SPI2S23CKSELR: c_uint = 0x60c;
pub const RCC_SPI45CKSELR: c_uint = 0x610;
pub const RCC_UART12CKSELR: c_uint = 0x614;
pub const RCC_UART35CKSELR: c_uint = 0x618;
pub const RCC_UART4CKSELR: c_uint = 0x61c;
pub const RCC_UART6CKSELR: c_uint = 0x620;
pub const RCC_UART78CKSELR: c_uint = 0x624;
pub const RCC_LPTIM1CKSELR: c_uint = 0x628;
pub const RCC_LPTIM23CKSELR: c_uint = 0x62c;
pub const RCC_LPTIM45CKSELR: c_uint = 0x630;
pub const RCC_SAI1CKSELR: c_uint = 0x634;
pub const RCC_SAI2CKSELR: c_uint = 0x638;
pub const RCC_FDCANCKSELR: c_uint = 0x63c;
pub const RCC_SPDIFCKSELR: c_uint = 0x640;
pub const RCC_ADC12CKSELR: c_uint = 0x644;
pub const RCC_SDMMC12CKSELR: c_uint = 0x648;
pub const RCC_ETH12CKSELR: c_uint = 0x64c;
pub const RCC_USBCKSELR: c_uint = 0x650;
pub const RCC_QSPICKSELR: c_uint = 0x654;
pub const RCC_FMCCKSELR: c_uint = 0x658;
pub const RCC_RNG1CKSELR: c_uint = 0x65c;
pub const RCC_STGENCKSELR: c_uint = 0x660;
pub const RCC_DCMIPPCKSELR: c_uint = 0x664;
pub const RCC_SAESCKSELR: c_uint = 0x668;
pub const RCC_APB1RSTSETR: c_uint = 0x6a0;
pub const RCC_APB1RSTCLRR: c_uint = 0x6a4;
pub const RCC_APB2RSTSETR: c_uint = 0x6a8;
pub const RCC_APB2RSTCLRR: c_uint = 0x6ac;
pub const RCC_APB3RSTSETR: c_uint = 0x6b0;
pub const RCC_APB3RSTCLRR: c_uint = 0x6b4;
pub const RCC_APB4RSTSETR: c_uint = 0x6b8;
pub const RCC_APB4RSTCLRR: c_uint = 0x6bc;
pub const RCC_APB5RSTSETR: c_uint = 0x6c0;
pub const RCC_APB5RSTCLRR: c_uint = 0x6c4;
pub const RCC_APB6RSTSETR: c_uint = 0x6c8;
pub const RCC_APB6RSTCLRR: c_uint = 0x6cc;
pub const RCC_AHB2RSTSETR: c_uint = 0x6d0;
pub const RCC_AHB2RSTCLRR: c_uint = 0x6d4;
pub const RCC_AHB4RSTSETR: c_uint = 0x6e0;
pub const RCC_AHB4RSTCLRR: c_uint = 0x6e4;
pub const RCC_AHB5RSTSETR: c_uint = 0x6e8;
pub const RCC_AHB5RSTCLRR: c_uint = 0x6ec;
pub const RCC_AHB6RSTSETR: c_uint = 0x6f0;
pub const RCC_AHB6RSTCLRR: c_uint = 0x6f4;
pub const RCC_MP_APB1ENSETR: c_uint = 0x700;
pub const RCC_MP_APB1ENCLRR: c_uint = 0x704;
pub const RCC_MP_APB2ENSETR: c_uint = 0x708;
pub const RCC_MP_APB2ENCLRR: c_uint = 0x70c;
pub const RCC_MP_APB3ENSETR: c_uint = 0x710;
pub const RCC_MP_APB3ENCLRR: c_uint = 0x714;
pub const RCC_MP_S_APB3ENSETR: c_uint = 0x718;
pub const RCC_MP_S_APB3ENCLRR: c_uint = 0x71c;
pub const RCC_MP_NS_APB3ENSETR: c_uint = 0x720;
pub const RCC_MP_NS_APB3ENCLRR: c_uint = 0x724;
pub const RCC_MP_APB4ENSETR: c_uint = 0x728;
pub const RCC_MP_APB4ENCLRR: c_uint = 0x72c;
pub const RCC_MP_S_APB4ENSETR: c_uint = 0x730;
pub const RCC_MP_S_APB4ENCLRR: c_uint = 0x734;
pub const RCC_MP_NS_APB4ENSETR: c_uint = 0x738;
pub const RCC_MP_NS_APB4ENCLRR: c_uint = 0x73c;
pub const RCC_MP_APB5ENSETR: c_uint = 0x740;
pub const RCC_MP_APB5ENCLRR: c_uint = 0x744;
pub const RCC_MP_APB6ENSETR: c_uint = 0x748;
pub const RCC_MP_APB6ENCLRR: c_uint = 0x74c;
pub const RCC_MP_AHB2ENSETR: c_uint = 0x750;
pub const RCC_MP_AHB2ENCLRR: c_uint = 0x754;
pub const RCC_MP_AHB4ENSETR: c_uint = 0x760;
pub const RCC_MP_AHB4ENCLRR: c_uint = 0x764;
pub const RCC_MP_S_AHB4ENSETR: c_uint = 0x768;
pub const RCC_MP_S_AHB4ENCLRR: c_uint = 0x76c;
pub const RCC_MP_NS_AHB4ENSETR: c_uint = 0x770;
pub const RCC_MP_NS_AHB4ENCLRR: c_uint = 0x774;
pub const RCC_MP_AHB5ENSETR: c_uint = 0x778;
pub const RCC_MP_AHB5ENCLRR: c_uint = 0x77c;
pub const RCC_MP_AHB6ENSETR: c_uint = 0x780;
pub const RCC_MP_AHB6ENCLRR: c_uint = 0x784;
pub const RCC_MP_S_AHB6ENSETR: c_uint = 0x788;
pub const RCC_MP_S_AHB6ENCLRR: c_uint = 0x78c;
pub const RCC_MP_NS_AHB6ENSETR: c_uint = 0x790;
pub const RCC_MP_NS_AHB6ENCLRR: c_uint = 0x794;
pub const RCC_MP_APB1LPENSETR: c_uint = 0x800;
pub const RCC_MP_APB1LPENCLRR: c_uint = 0x804;
pub const RCC_MP_APB2LPENSETR: c_uint = 0x808;
pub const RCC_MP_APB2LPENCLRR: c_uint = 0x80c;
pub const RCC_MP_APB3LPENSETR: c_uint = 0x810;
pub const RCC_MP_APB3LPENCLRR: c_uint = 0x814;
pub const RCC_MP_S_APB3LPENSETR: c_uint = 0x818;
pub const RCC_MP_S_APB3LPENCLRR: c_uint = 0x81c;
pub const RCC_MP_NS_APB3LPENSETR: c_uint = 0x820;
pub const RCC_MP_NS_APB3LPENCLRR: c_uint = 0x824;
pub const RCC_MP_APB4LPENSETR: c_uint = 0x828;
pub const RCC_MP_APB4LPENCLRR: c_uint = 0x82c;
pub const RCC_MP_S_APB4LPENSETR: c_uint = 0x830;
pub const RCC_MP_S_APB4LPENCLRR: c_uint = 0x834;
pub const RCC_MP_NS_APB4LPENSETR: c_uint = 0x838;
pub const RCC_MP_NS_APB4LPENCLRR: c_uint = 0x83c;
pub const RCC_MP_APB5LPENSETR: c_uint = 0x840;
pub const RCC_MP_APB5LPENCLRR: c_uint = 0x844;
pub const RCC_MP_APB6LPENSETR: c_uint = 0x848;
pub const RCC_MP_APB6LPENCLRR: c_uint = 0x84c;
pub const RCC_MP_AHB2LPENSETR: c_uint = 0x850;
pub const RCC_MP_AHB2LPENCLRR: c_uint = 0x854;
pub const RCC_MP_AHB4LPENSETR: c_uint = 0x858;
pub const RCC_MP_AHB4LPENCLRR: c_uint = 0x85c;
pub const RCC_MP_S_AHB4LPENSETR: c_uint = 0x868;
pub const RCC_MP_S_AHB4LPENCLRR: c_uint = 0x86c;
pub const RCC_MP_NS_AHB4LPENSETR: c_uint = 0x870;
pub const RCC_MP_NS_AHB4LPENCLRR: c_uint = 0x874;
pub const RCC_MP_AHB5LPENSETR: c_uint = 0x878;
pub const RCC_MP_AHB5LPENCLRR: c_uint = 0x87c;
pub const RCC_MP_AHB6LPENSETR: c_uint = 0x880;
pub const RCC_MP_AHB6LPENCLRR: c_uint = 0x884;
pub const RCC_MP_S_AHB6LPENSETR: c_uint = 0x888;
pub const RCC_MP_S_AHB6LPENCLRR: c_uint = 0x88c;
pub const RCC_MP_NS_AHB6LPENSETR: c_uint = 0x890;
pub const RCC_MP_NS_AHB6LPENCLRR: c_uint = 0x894;
pub const RCC_MP_S_AXIMLPENSETR: c_uint = 0x898;
pub const RCC_MP_S_AXIMLPENCLRR: c_uint = 0x89c;
pub const RCC_MP_NS_AXIMLPENSETR: c_uint = 0x8a0;
pub const RCC_MP_NS_AXIMLPENCLRR: c_uint = 0x8a4;
pub const RCC_MP_MLAHBLPENSETR: c_uint = 0x8a8;
pub const RCC_MP_MLAHBLPENCLRR: c_uint = 0x8ac;
pub const RCC_APB3SECSR: c_uint = 0x8c0;
pub const RCC_APB4SECSR: c_uint = 0x8c4;
pub const RCC_APB5SECSR: c_uint = 0x8c8;
pub const RCC_APB6SECSR: c_uint = 0x8cc;
pub const RCC_AHB2SECSR: c_uint = 0x8d0;
pub const RCC_AHB4SECSR: c_uint = 0x8d4;
pub const RCC_AHB5SECSR: c_uint = 0x8d8;
pub const RCC_AHB6SECSR: c_uint = 0x8dc;
pub const RCC_VERR: c_uint = 0xff4;
pub const RCC_IDR: c_uint = 0xff8;
pub const RCC_SIDR: c_uint = 0xffc;
// RCC_SECCFGR register fields
pub const RCC_SECCFGR_HSISEC: c_int = 0;
pub const RCC_SECCFGR_CSISEC: c_int = 1;
pub const RCC_SECCFGR_HSESEC: c_int = 2;
pub const RCC_SECCFGR_LSISEC: c_int = 3;
pub const RCC_SECCFGR_LSESEC: c_int = 4;
pub const RCC_SECCFGR_PLL12SEC: c_int = 8;
pub const RCC_SECCFGR_PLL3SEC: c_int = 9;
pub const RCC_SECCFGR_PLL4SEC: c_int = 10;
pub const RCC_SECCFGR_MPUSEC: c_int = 11;
pub const RCC_SECCFGR_AXISEC: c_int = 12;
pub const RCC_SECCFGR_MLAHBSEC: c_int = 13;
pub const RCC_SECCFGR_APB3DIVSEC: c_int = 16;
pub const RCC_SECCFGR_APB4DIVSEC: c_int = 17;
pub const RCC_SECCFGR_APB5DIVSEC: c_int = 18;
pub const RCC_SECCFGR_APB6DIVSEC: c_int = 19;
pub const RCC_SECCFGR_TIMG3SEC: c_int = 20;
pub const RCC_SECCFGR_CPERSEC: c_int = 21;
pub const RCC_SECCFGR_MCO1SEC: c_int = 22;
pub const RCC_SECCFGR_MCO2SEC: c_int = 23;
pub const RCC_SECCFGR_STPSEC: c_int = 24;
pub const RCC_SECCFGR_RSTSEC: c_int = 25;
pub const RCC_SECCFGR_PWRSEC: c_int = 31;
// RCC_MP_SREQSETR register fields

// RCC_MP_SREQCLRR register fields

// RCC_MP_APRSTCR register fields

pub const RCC_MP_APRSTCR_RSTTO_SHIFT: c_int = 8;
// RCC_MP_APRSTSR register fields

pub const RCC_MP_APRSTSR_RSTTOV_SHIFT: c_int = 8;
// RCC_PWRLPDLYCR register fields

pub const RCC_PWRLPDLYCR_PWRLP_DLY_SHIFT: c_int = 0;
// RCC_MP_GRSTCSETR register fields

// RCC_BR_RSTSCLRR register fields

// RCC_MP_RSTSSETR register fields

// RCC_MP_RSTSCLRR register fields

// RCC_MP_IWDGFZSETR register fields

// RCC_MP_IWDGFZCLRR register fields

// RCC_MP_CIER register fields

// RCC_MP_CIFR register fields

// RCC_BDCR register fields

pub const RCC_BDCR_LSEDRV_SHIFT: c_int = 4;
pub const RCC_BDCR_RTCSRC_SHIFT: c_int = 16;
// RCC_RDLSICR register fields

pub const RCC_RDLSICR_MRD_SHIFT: c_int = 16;
pub const RCC_RDLSICR_EADLY_SHIFT: c_int = 24;
pub const RCC_RDLSICR_SPARE_SHIFT: c_int = 27;
// RCC_OCENSETR register fields

// RCC_OCENCLRR register fields

// RCC_OCRDYR register fields

// RCC_HSICFGR register fields

pub const RCC_HSICFGR_HSIDIV_SHIFT: c_int = 0;
pub const RCC_HSICFGR_HSITRIM_SHIFT: c_int = 8;
pub const RCC_HSICFGR_HSICAL_SHIFT: c_int = 16;
// RCC_CSICFGR register fields

pub const RCC_CSICFGR_CSITRIM_SHIFT: c_int = 8;
pub const RCC_CSICFGR_CSICAL_SHIFT: c_int = 16;
// RCC_MCO1CFGR register fields

pub const RCC_MCO1CFGR_MCO1SEL_SHIFT: c_int = 0;
pub const RCC_MCO1CFGR_MCO1DIV_SHIFT: c_int = 4;
// RCC_MCO2CFGR register fields

pub const RCC_MCO2CFGR_MCO2SEL_SHIFT: c_int = 0;
pub const RCC_MCO2CFGR_MCO2DIV_SHIFT: c_int = 4;
// RCC_DBGCFGR register fields

pub const RCC_DBGCFGR_TRACEDIV_SHIFT: c_int = 0;
// RCC_RCK12SELR register fields

pub const RCC_RCK12SELR_PLL12SRC_SHIFT: c_int = 0;
// RCC_RCK3SELR register fields

pub const RCC_RCK3SELR_PLL3SRC_SHIFT: c_int = 0;
// RCC_RCK4SELR register fields

pub const RCC_RCK4SELR_PLL4SRC_SHIFT: c_int = 0;
// RCC_PLL1CR register fields

// RCC_PLL1CFGR1 register fields

pub const RCC_PLL1CFGR1_DIVN_SHIFT: c_int = 0;
pub const RCC_PLL1CFGR1_DIVM1_SHIFT: c_int = 16;
// RCC_PLL1CFGR2 register fields

pub const RCC_PLL1CFGR2_DIVP_SHIFT: c_int = 0;
pub const RCC_PLL1CFGR2_DIVQ_SHIFT: c_int = 8;
pub const RCC_PLL1CFGR2_DIVR_SHIFT: c_int = 16;
// RCC_PLL1FRACR register fields

pub const RCC_PLL1FRACR_FRACV_SHIFT: c_int = 3;
// RCC_PLL1CSGR register fields

pub const RCC_PLL1CSGR_MOD_PER_SHIFT: c_int = 0;
pub const RCC_PLL1CSGR_INC_STEP_SHIFT: c_int = 16;
// RCC_PLL2CR register fields

// RCC_PLL2CFGR1 register fields

pub const RCC_PLL2CFGR1_DIVN_SHIFT: c_int = 0;
pub const RCC_PLL2CFGR1_DIVM2_SHIFT: c_int = 16;
// RCC_PLL2CFGR2 register fields

pub const RCC_PLL2CFGR2_DIVP_SHIFT: c_int = 0;
pub const RCC_PLL2CFGR2_DIVQ_SHIFT: c_int = 8;
pub const RCC_PLL2CFGR2_DIVR_SHIFT: c_int = 16;
// RCC_PLL2FRACR register fields

pub const RCC_PLL2FRACR_FRACV_SHIFT: c_int = 3;
// RCC_PLL2CSGR register fields

pub const RCC_PLL2CSGR_MOD_PER_SHIFT: c_int = 0;
pub const RCC_PLL2CSGR_INC_STEP_SHIFT: c_int = 16;
// RCC_PLL3CR register fields

// RCC_PLL3CFGR1 register fields

pub const RCC_PLL3CFGR1_DIVN_SHIFT: c_int = 0;
pub const RCC_PLL3CFGR1_DIVM3_SHIFT: c_int = 16;
pub const RCC_PLL3CFGR1_IFRGE_SHIFT: c_int = 24;
// RCC_PLL3CFGR2 register fields

pub const RCC_PLL3CFGR2_DIVP_SHIFT: c_int = 0;
pub const RCC_PLL3CFGR2_DIVQ_SHIFT: c_int = 8;
pub const RCC_PLL3CFGR2_DIVR_SHIFT: c_int = 16;
// RCC_PLL3FRACR register fields

pub const RCC_PLL3FRACR_FRACV_SHIFT: c_int = 3;
// RCC_PLL3CSGR register fields

pub const RCC_PLL3CSGR_MOD_PER_SHIFT: c_int = 0;
pub const RCC_PLL3CSGR_INC_STEP_SHIFT: c_int = 16;
// RCC_PLL4CR register fields

// RCC_PLL4CFGR1 register fields

pub const RCC_PLL4CFGR1_DIVN_SHIFT: c_int = 0;
pub const RCC_PLL4CFGR1_DIVM4_SHIFT: c_int = 16;
pub const RCC_PLL4CFGR1_IFRGE_SHIFT: c_int = 24;
// RCC_PLL4CFGR2 register fields

pub const RCC_PLL4CFGR2_DIVP_SHIFT: c_int = 0;
pub const RCC_PLL4CFGR2_DIVQ_SHIFT: c_int = 8;
pub const RCC_PLL4CFGR2_DIVR_SHIFT: c_int = 16;
// RCC_PLL4FRACR register fields

pub const RCC_PLL4FRACR_FRACV_SHIFT: c_int = 3;
// RCC_PLL4CSGR register fields

pub const RCC_PLL4CSGR_MOD_PER_SHIFT: c_int = 0;
pub const RCC_PLL4CSGR_INC_STEP_SHIFT: c_int = 16;
// RCC_MPCKSELR register fields

pub const RCC_MPCKSELR_MPUSRC_SHIFT: c_int = 0;
// RCC_ASSCKSELR register fields

pub const RCC_ASSCKSELR_AXISSRC_SHIFT: c_int = 0;
// RCC_MSSCKSELR register fields

pub const RCC_MSSCKSELR_MLAHBSSRC_SHIFT: c_int = 0;
// RCC_CPERCKSELR register fields

pub const RCC_CPERCKSELR_CKPERSRC_SHIFT: c_int = 0;
// RCC_RTCDIVR register fields

pub const RCC_RTCDIVR_RTCDIV_SHIFT: c_int = 0;
// RCC_MPCKDIVR register fields

pub const RCC_MPCKDIVR_MPUDIV_SHIFT: c_int = 0;
// RCC_AXIDIVR register fields

pub const RCC_AXIDIVR_AXIDIV_SHIFT: c_int = 0;
// RCC_MLAHBDIVR register fields

pub const RCC_MLAHBDIVR_MLAHBDIV_SHIFT: c_int = 0;
// RCC_APB1DIVR register fields

pub const RCC_APB1DIVR_APB1DIV_SHIFT: c_int = 0;
// RCC_APB2DIVR register fields

pub const RCC_APB2DIVR_APB2DIV_SHIFT: c_int = 0;
// RCC_APB3DIVR register fields

pub const RCC_APB3DIVR_APB3DIV_SHIFT: c_int = 0;
// RCC_APB4DIVR register fields

pub const RCC_APB4DIVR_APB4DIV_SHIFT: c_int = 0;
// RCC_APB5DIVR register fields

pub const RCC_APB5DIVR_APB5DIV_SHIFT: c_int = 0;
// RCC_APB6DIVR register fields

pub const RCC_APB6DIVR_APB6DIV_SHIFT: c_int = 0;
// RCC_TIMG1PRER register fields

// RCC_TIMG2PRER register fields

// RCC_TIMG3PRER register fields

// RCC_DDRITFCR register fields

pub const RCC_DDRITFCR_KERDCG_DLY_SHIFT: c_int = 11;
pub const RCC_DDRITFCR_DDRCKMOD_SHIFT: c_int = 20;
pub const RCC_DDRITFCR_DFILP_WIDTH_SHIFT: c_int = 25;
pub const RCC_DDRITFCR_GSKP_DUR_SHIFT: c_int = 28;
// RCC_I2C12CKSELR register fields

pub const RCC_I2C12CKSELR_I2C12SRC_SHIFT: c_int = 0;
// RCC_I2C345CKSELR register fields

pub const RCC_I2C345CKSELR_I2C3SRC_SHIFT: c_int = 0;
pub const RCC_I2C345CKSELR_I2C4SRC_SHIFT: c_int = 3;
pub const RCC_I2C345CKSELR_I2C5SRC_SHIFT: c_int = 6;
// RCC_SPI2S1CKSELR register fields

pub const RCC_SPI2S1CKSELR_SPI1SRC_SHIFT: c_int = 0;
// RCC_SPI2S23CKSELR register fields

pub const RCC_SPI2S23CKSELR_SPI23SRC_SHIFT: c_int = 0;
// RCC_SPI45CKSELR register fields

pub const RCC_SPI45CKSELR_SPI4SRC_SHIFT: c_int = 0;
pub const RCC_SPI45CKSELR_SPI5SRC_SHIFT: c_int = 3;
// RCC_UART12CKSELR register fields

pub const RCC_UART12CKSELR_UART1SRC_SHIFT: c_int = 0;
pub const RCC_UART12CKSELR_UART2SRC_SHIFT: c_int = 3;
// RCC_UART35CKSELR register fields

pub const RCC_UART35CKSELR_UART35SRC_SHIFT: c_int = 0;
// RCC_UART4CKSELR register fields

pub const RCC_UART4CKSELR_UART4SRC_SHIFT: c_int = 0;
// RCC_UART6CKSELR register fields

pub const RCC_UART6CKSELR_UART6SRC_SHIFT: c_int = 0;
// RCC_UART78CKSELR register fields

pub const RCC_UART78CKSELR_UART78SRC_SHIFT: c_int = 0;
// RCC_LPTIM1CKSELR register fields

pub const RCC_LPTIM1CKSELR_LPTIM1SRC_SHIFT: c_int = 0;
// RCC_LPTIM23CKSELR register fields

pub const RCC_LPTIM23CKSELR_LPTIM2SRC_SHIFT: c_int = 0;
pub const RCC_LPTIM23CKSELR_LPTIM3SRC_SHIFT: c_int = 3;
// RCC_LPTIM45CKSELR register fields

pub const RCC_LPTIM45CKSELR_LPTIM45SRC_SHIFT: c_int = 0;
// RCC_SAI1CKSELR register fields

pub const RCC_SAI1CKSELR_SAI1SRC_SHIFT: c_int = 0;
// RCC_SAI2CKSELR register fields

pub const RCC_SAI2CKSELR_SAI2SRC_SHIFT: c_int = 0;
// RCC_FDCANCKSELR register fields

pub const RCC_FDCANCKSELR_FDCANSRC_SHIFT: c_int = 0;
// RCC_SPDIFCKSELR register fields

pub const RCC_SPDIFCKSELR_SPDIFSRC_SHIFT: c_int = 0;
// RCC_ADC12CKSELR register fields

pub const RCC_ADC12CKSELR_ADC1SRC_SHIFT: c_int = 0;
pub const RCC_ADC12CKSELR_ADC2SRC_SHIFT: c_int = 2;
// RCC_SDMMC12CKSELR register fields

pub const RCC_SDMMC12CKSELR_SDMMC1SRC_SHIFT: c_int = 0;
pub const RCC_SDMMC12CKSELR_SDMMC2SRC_SHIFT: c_int = 3;
// RCC_ETH12CKSELR register fields

pub const RCC_ETH12CKSELR_ETH1SRC_SHIFT: c_int = 0;
pub const RCC_ETH12CKSELR_ETH1PTPDIV_SHIFT: c_int = 4;
pub const RCC_ETH12CKSELR_ETH2SRC_SHIFT: c_int = 8;
pub const RCC_ETH12CKSELR_ETH2PTPDIV_SHIFT: c_int = 12;
// RCC_USBCKSELR register fields

pub const RCC_USBCKSELR_USBPHYSRC_SHIFT: c_int = 0;
// RCC_QSPICKSELR register fields

pub const RCC_QSPICKSELR_QSPISRC_SHIFT: c_int = 0;
// RCC_FMCCKSELR register fields

pub const RCC_FMCCKSELR_FMCSRC_SHIFT: c_int = 0;
// RCC_RNG1CKSELR register fields

pub const RCC_RNG1CKSELR_RNG1SRC_SHIFT: c_int = 0;
// RCC_STGENCKSELR register fields

pub const RCC_STGENCKSELR_STGENSRC_SHIFT: c_int = 0;
// RCC_DCMIPPCKSELR register fields

pub const RCC_DCMIPPCKSELR_DCMIPPSRC_SHIFT: c_int = 0;
// RCC_SAESCKSELR register fields

pub const RCC_SAESCKSELR_SAESSRC_SHIFT: c_int = 0;
// RCC_APB1RSTSETR register fields

// RCC_APB1RSTCLRR register fields

// RCC_APB2RSTSETR register fields

// RCC_APB2RSTCLRR register fields

// RCC_APB3RSTSETR register fields

// RCC_APB3RSTCLRR register fields

// RCC_APB4RSTSETR register fields

// RCC_APB4RSTCLRR register fields

// RCC_APB5RSTSETR register fields

// RCC_APB5RSTCLRR register fields

// RCC_APB6RSTSETR register fields

// RCC_APB6RSTCLRR register fields

// RCC_AHB2RSTSETR register fields

// RCC_AHB2RSTCLRR register fields

// RCC_AHB4RSTSETR register fields

// RCC_AHB4RSTCLRR register fields

// RCC_AHB5RSTSETR register fields

// RCC_AHB5RSTCLRR register fields

// RCC_AHB6RSTSETR register fields

// RCC_AHB6RSTCLRR register fields

// RCC_MP_APB1ENSETR register fields

// RCC_MP_APB1ENCLRR register fields

// RCC_MP_APB2ENSETR register fields

// RCC_MP_APB2ENCLRR register fields

// RCC_MP_APB3ENSETR register fields

// RCC_MP_APB3ENCLRR register fields

// RCC_MP_S_APB3ENSETR register fields

// RCC_MP_S_APB3ENCLRR register fields

// RCC_MP_NS_APB3ENSETR register fields

// RCC_MP_NS_APB3ENCLRR register fields

// RCC_MP_APB4ENSETR register fields

// RCC_MP_APB4ENCLRR register fields

// RCC_MP_S_APB4ENSETR register fields

// RCC_MP_S_APB4ENCLRR register fields

// RCC_MP_NS_APB4ENSETR register fields

// RCC_MP_NS_APB4ENCLRR register fields

// RCC_MP_APB5ENSETR register fields

// RCC_MP_APB5ENCLRR register fields

// RCC_MP_APB6ENSETR register fields

// RCC_MP_APB6ENCLRR register fields

// RCC_MP_AHB2ENSETR register fields

// RCC_MP_AHB2ENCLRR register fields

// RCC_MP_AHB4ENSETR register fields

// RCC_MP_AHB4ENCLRR register fields

// RCC_MP_S_AHB4ENSETR register fields

// RCC_MP_S_AHB4ENCLRR register fields

// RCC_MP_NS_AHB4ENSETR register fields

// RCC_MP_NS_AHB4ENCLRR register fields

// RCC_MP_AHB5ENSETR register fields

// RCC_MP_AHB5ENCLRR register fields

// RCC_MP_AHB6ENSETR register fields

// RCC_MP_AHB6ENCLRR register fields

// RCC_MP_S_AHB6ENSETR register fields

// RCC_MP_S_AHB6ENCLRR register fields

// RCC_MP_NS_AHB6ENSETR register fields

// RCC_MP_NS_AHB6ENCLRR register fields

// RCC_MP_APB1LPENSETR register fields

// RCC_MP_APB1LPENCLRR register fields

// RCC_MP_APB2LPENSETR register fields

// RCC_MP_APB2LPENCLRR register fields

// RCC_MP_APB3LPENSETR register fields

// RCC_MP_APB3LPENCLRR register fields

// RCC_MP_S_APB3LPENSETR register fields

// RCC_MP_S_APB3LPENCLRR register fields

// RCC_MP_NS_APB3LPENSETR register fields

// RCC_MP_NS_APB3LPENCLRR register fields

// RCC_MP_APB4LPENSETR register fields

// RCC_MP_APB4LPENCLRR register fields

// RCC_MP_S_APB4LPENSETR register fields

// RCC_MP_S_APB4LPENCLRR register fields

// RCC_MP_NS_APB4LPENSETR register fields

// RCC_MP_NS_APB4LPENCLRR register fields

// RCC_MP_APB5LPENSETR register fields

// RCC_MP_APB5LPENCLRR register fields

// RCC_MP_APB6LPENSETR register fields

// RCC_MP_APB6LPENCLRR register fields

// RCC_MP_AHB2LPENSETR register fields

// RCC_MP_AHB2LPENCLRR register fields

// RCC_MP_AHB4LPENSETR register fields

// RCC_MP_AHB4LPENCLRR register fields

// RCC_MP_S_AHB4LPENSETR register fields

// RCC_MP_S_AHB4LPENCLRR register fields

// RCC_MP_NS_AHB4LPENSETR register fields

// RCC_MP_NS_AHB4LPENCLRR register fields

// RCC_MP_AHB5LPENSETR register fields

// RCC_MP_AHB5LPENCLRR register fields

// RCC_MP_AHB6LPENSETR register fields

// RCC_MP_AHB6LPENCLRR register fields

// RCC_MP_S_AHB6LPENSETR register fields

// RCC_MP_S_AHB6LPENCLRR register fields

// RCC_MP_NS_AHB6LPENSETR register fields

// RCC_MP_NS_AHB6LPENCLRR register fields

// RCC_MP_S_AXIMLPENSETR register fields

// RCC_MP_S_AXIMLPENCLRR register fields

// RCC_MP_NS_AXIMLPENSETR register fields

// RCC_MP_NS_AXIMLPENCLRR register fields

// RCC_MP_MLAHBLPENSETR register fields

// RCC_MP_MLAHBLPENCLRR register fields

// RCC_APB3SECSR register fields
pub const RCC_APB3SECSR_LPTIM2SECF: c_int = 0;
pub const RCC_APB3SECSR_LPTIM3SECF: c_int = 1;
pub const RCC_APB3SECSR_VREFSECF: c_int = 13;
// RCC_APB4SECSR register fields
pub const RCC_APB4SECSR_DCMIPPSECF: c_int = 1;
pub const RCC_APB4SECSR_USBPHYSECF: c_int = 16;
// RCC_APB5SECSR register fields
pub const RCC_APB5SECSR_RTCSECF: c_int = 8;
pub const RCC_APB5SECSR_TZCSECF: c_int = 11;
pub const RCC_APB5SECSR_ETZPCSECF: c_int = 13;
pub const RCC_APB5SECSR_IWDG1SECF: c_int = 15;
pub const RCC_APB5SECSR_BSECSECF: c_int = 16;

pub const RCC_APB5SECSR_STGENCSECF: c_int = 20;
pub const RCC_APB5SECSR_STGENROSECF: c_int = 21;
// RCC_APB6SECSR register fields
pub const RCC_APB6SECSR_USART1SECF: c_int = 0;
pub const RCC_APB6SECSR_USART2SECF: c_int = 1;
pub const RCC_APB6SECSR_SPI4SECF: c_int = 2;
pub const RCC_APB6SECSR_SPI5SECF: c_int = 3;
pub const RCC_APB6SECSR_I2C3SECF: c_int = 4;
pub const RCC_APB6SECSR_I2C4SECF: c_int = 5;
pub const RCC_APB6SECSR_I2C5SECF: c_int = 6;
pub const RCC_APB6SECSR_TIM12SECF: c_int = 7;
pub const RCC_APB6SECSR_TIM13SECF: c_int = 8;
pub const RCC_APB6SECSR_TIM14SECF: c_int = 9;
pub const RCC_APB6SECSR_TIM15SECF: c_int = 10;
pub const RCC_APB6SECSR_TIM16SECF: c_int = 11;
pub const RCC_APB6SECSR_TIM17SECF: c_int = 12;
// RCC_AHB2SECSR register fields
pub const RCC_AHB2SECSR_DMA3SECF: c_int = 3;
pub const RCC_AHB2SECSR_DMAMUX2SECF: c_int = 4;
pub const RCC_AHB2SECSR_ADC1SECF: c_int = 5;
pub const RCC_AHB2SECSR_ADC2SECF: c_int = 6;
pub const RCC_AHB2SECSR_USBOSECF: c_int = 8;
// RCC_AHB4SECSR register fields
pub const RCC_AHB4SECSR_TSCSECF: c_int = 15;
// RCC_AHB5SECSR register fields
pub const RCC_AHB5SECSR_PKASECF: c_int = 2;
pub const RCC_AHB5SECSR_SAESSECF: c_int = 3;
pub const RCC_AHB5SECSR_CRYP1SECF: c_int = 4;
pub const RCC_AHB5SECSR_HASH1SECF: c_int = 5;
pub const RCC_AHB5SECSR_RNG1SECF: c_int = 6;
pub const RCC_AHB5SECSR_BKPSRAMSECF: c_int = 8;
// RCC_AHB6SECSR register fields
pub const RCC_AHB6SECSR_MCESECF: c_int = 1;
pub const RCC_AHB6SECSR_FMCSECF: c_int = 12;
pub const RCC_AHB6SECSR_QSPISECF: c_int = 14;
pub const RCC_AHB6SECSR_SDMMC1SECF: c_int = 16;
pub const RCC_AHB6SECSR_SDMMC2SECF: c_int = 17;

pub const RCC_AHB6SECSR_ETH1SECF_SHIFT: c_int = 7;
pub const RCC_AHB6SECSR_ETH2SECF_SHIFT: c_int = 27;
pub const RCC_AHB6SECSR_ETH1CKSECF: c_int = 7;
pub const RCC_AHB6SECSR_ETH1TXSECF: c_int = 8;
pub const RCC_AHB6SECSR_ETH1RXSECF: c_int = 9;
pub const RCC_AHB6SECSR_ETH1MACSECF: c_int = 10;
pub const RCC_AHB6SECSR_ETH1STPSECF: c_int = 11;
pub const RCC_AHB6SECSR_ETH2CKSECF: c_int = 27;
pub const RCC_AHB6SECSR_ETH2TXSECF: c_int = 28;
pub const RCC_AHB6SECSR_ETH2RXSECF: c_int = 29;
pub const RCC_AHB6SECSR_ETH2MACSECF: c_int = 30;
pub const RCC_AHB6SECSR_ETH2STPSECF: c_int = 31;
// RCC_VERR register fields

pub const RCC_VERR_MINREV_SHIFT: c_int = 0;
pub const RCC_VERR_MAJREV_SHIFT: c_int = 4;
// RCC_IDR register fields

pub const RCC_IDR_ID_SHIFT: c_int = 0;
// RCC_SIDR register fields

pub const RCC_SIDR_SID_SHIFT: c_int = 0;
