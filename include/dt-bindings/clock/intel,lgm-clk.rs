//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/intel,lgm-clk.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2020 Intel Corporation.
// Lei Chuanhua <Chuanhua.lei@intel.com>
// Zhu Yixin <Yixin.zhu@intel.com>
//
// PLL clocks
pub const LGM_CLK_OSC: c_int = 1;
pub const LGM_CLK_PLLPP: c_int = 2;
pub const LGM_CLK_PLL2: c_int = 3;
pub const LGM_CLK_PLL0CZ: c_int = 4;
pub const LGM_CLK_PLL0B: c_int = 5;
pub const LGM_CLK_PLL1: c_int = 6;
pub const LGM_CLK_LJPLL3: c_int = 7;
pub const LGM_CLK_LJPLL4: c_int = 8;
pub const LGM_CLK_PLL0CM0: c_int = 9;
pub const LGM_CLK_PLL0CM1: c_int = 10;
// clocks from PLLs
// ROPLL clocks
pub const LGM_CLK_PP_HW: c_int = 15;
pub const LGM_CLK_PP_UC: c_int = 16;
pub const LGM_CLK_PP_FXD: c_int = 17;
pub const LGM_CLK_PP_TBM: c_int = 18;
// PLL2 clocks
pub const LGM_CLK_DDR: c_int = 20;
// PLL0CZ
pub const LGM_CLK_CM: c_int = 25;
pub const LGM_CLK_IC: c_int = 26;
pub const LGM_CLK_SDXC3: c_int = 27;
// PLL0B
pub const LGM_CLK_NGI: c_int = 30;
pub const LGM_CLK_NOC4: c_int = 31;
pub const LGM_CLK_SW: c_int = 32;
pub const LGM_CLK_QSPI: c_int = 33;

// PLL1
pub const LGM_CLK_CT: c_int = 35;
pub const LGM_CLK_DSP: c_int = 36;
pub const LGM_CLK_VIF: c_int = 37;
// LJPLL3
pub const LGM_CLK_CML: c_int = 40;
pub const LGM_CLK_SERDES: c_int = 41;
pub const LGM_CLK_POOL: c_int = 42;
pub const LGM_CLK_PTP: c_int = 43;
// LJPLL4
pub const LGM_CLK_PCIE: c_int = 45;

// PLL0CM0
pub const LGM_CLK_CPU0: c_int = 50;
// PLL0CM1
pub const LGM_CLK_CPU1: c_int = 55;
// Miscellaneous clocks
pub const LGM_CLK_EMMC4: c_int = 60;
pub const LGM_CLK_SDXC2: c_int = 61;
pub const LGM_CLK_EMMC: c_int = 62;
pub const LGM_CLK_SDXC: c_int = 63;
pub const LGM_CLK_SLIC: c_int = 64;
pub const LGM_CLK_DCL: c_int = 65;
pub const LGM_CLK_DOCSIS: c_int = 66;
pub const LGM_CLK_PCM: c_int = 67;
pub const LGM_CLK_DDR_PHY: c_int = 68;
pub const LGM_CLK_PONDEF: c_int = 69;
pub const LGM_CLK_PL25M: c_int = 70;
pub const LGM_CLK_PL10M: c_int = 71;
pub const LGM_CLK_PL1544K: c_int = 72;
pub const LGM_CLK_PL2048K: c_int = 73;
pub const LGM_CLK_PL8K: c_int = 74;
pub const LGM_CLK_PON_NTR: c_int = 75;
pub const LGM_CLK_SYNC0: c_int = 76;
pub const LGM_CLK_SYNC1: c_int = 77;
pub const LGM_CLK_PROGDIV: c_int = 78;
pub const LGM_CLK_OD0: c_int = 79;
pub const LGM_CLK_OD1: c_int = 80;
pub const LGM_CLK_CBPHY0: c_int = 81;
pub const LGM_CLK_CBPHY1: c_int = 82;
pub const LGM_CLK_CBPHY2: c_int = 83;
pub const LGM_CLK_CBPHY3: c_int = 84;
// Gate clocks
// Gate CLK0
pub const LGM_GCLK_C55: c_int = 100;
pub const LGM_GCLK_QSPI: c_int = 101;
pub const LGM_GCLK_EIP197: c_int = 102;
pub const LGM_GCLK_VAULT: c_int = 103;
pub const LGM_GCLK_TOE: c_int = 104;
pub const LGM_GCLK_SDXC: c_int = 105;
pub const LGM_GCLK_EMMC: c_int = 106;
pub const LGM_GCLK_SPI_DBG: c_int = 107;
pub const LGM_GCLK_DMA3: c_int = 108;
// Gate CLK1
pub const LGM_GCLK_DMA0: c_int = 120;
pub const LGM_GCLK_LEDC0: c_int = 121;
pub const LGM_GCLK_LEDC1: c_int = 122;
pub const LGM_GCLK_I2S0: c_int = 123;
pub const LGM_GCLK_I2S1: c_int = 124;
pub const LGM_GCLK_EBU: c_int = 125;
pub const LGM_GCLK_PWM: c_int = 126;
pub const LGM_GCLK_I2C0: c_int = 127;
pub const LGM_GCLK_I2C1: c_int = 128;
pub const LGM_GCLK_I2C2: c_int = 129;
pub const LGM_GCLK_I2C3: c_int = 130;
pub const LGM_GCLK_SSC0: c_int = 131;
pub const LGM_GCLK_SSC1: c_int = 132;
pub const LGM_GCLK_SSC2: c_int = 133;
pub const LGM_GCLK_SSC3: c_int = 134;
pub const LGM_GCLK_GPTC0: c_int = 135;
pub const LGM_GCLK_GPTC1: c_int = 136;
pub const LGM_GCLK_GPTC2: c_int = 137;
pub const LGM_GCLK_GPTC3: c_int = 138;
pub const LGM_GCLK_ASC0: c_int = 139;
pub const LGM_GCLK_ASC1: c_int = 140;
pub const LGM_GCLK_ASC2: c_int = 141;
pub const LGM_GCLK_ASC3: c_int = 142;
pub const LGM_GCLK_PCM0: c_int = 143;
pub const LGM_GCLK_PCM1: c_int = 144;
pub const LGM_GCLK_PCM2: c_int = 145;
// Gate CLK2
pub const LGM_GCLK_PCIE10: c_int = 150;
pub const LGM_GCLK_PCIE11: c_int = 151;
pub const LGM_GCLK_PCIE30: c_int = 152;
pub const LGM_GCLK_PCIE31: c_int = 153;
pub const LGM_GCLK_PCIE20: c_int = 154;
pub const LGM_GCLK_PCIE21: c_int = 155;
pub const LGM_GCLK_PCIE40: c_int = 156;
pub const LGM_GCLK_PCIE41: c_int = 157;
pub const LGM_GCLK_XPCS0: c_int = 158;
pub const LGM_GCLK_XPCS1: c_int = 159;
pub const LGM_GCLK_XPCS2: c_int = 160;
pub const LGM_GCLK_XPCS3: c_int = 161;
pub const LGM_GCLK_SATA0: c_int = 162;
pub const LGM_GCLK_SATA1: c_int = 163;
pub const LGM_GCLK_SATA2: c_int = 164;
pub const LGM_GCLK_SATA3: c_int = 165;
// Gate CLK3
pub const LGM_GCLK_ARCEM4: c_int = 170;
pub const LGM_GCLK_IDMAR1: c_int = 171;
pub const LGM_GCLK_IDMAT0: c_int = 172;
pub const LGM_GCLK_IDMAT1: c_int = 173;
pub const LGM_GCLK_IDMAT2: c_int = 174;
pub const LGM_GCLK_PPV4: c_int = 175;
pub const LGM_GCLK_GSWIPO: c_int = 176;
pub const LGM_GCLK_CQEM: c_int = 177;
pub const LGM_GCLK_XPCS5: c_int = 178;
pub const LGM_GCLK_USB1: c_int = 179;
pub const LGM_GCLK_USB2: c_int = 180;
