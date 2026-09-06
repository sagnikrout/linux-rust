//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tegra30-car.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This header provides constants for binding nvidia,tegra30-car.
//
// The first 130 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
// registers. These IDs often match those in the CAR's RST_DEVICES registers,
// but not in all cases. Some bits in CLK_OUT_ENB affect multiple clocks. In
// this case, those clocks are assigned IDs above 160 in order to highlight
// this issue. Implementations that interpret these clock IDs as bit values
// within the CLK_OUT_ENB or RST_DEVICES registers should be careful to
// explicitly handle these special cases.
//
// The balance of the clocks controlled by the CAR are assigned IDs of 160 and
// above.
//
pub const TEGRA30_CLK_CPU: c_int = 0;
// 1
// 2
// 3
pub const TEGRA30_CLK_RTC: c_int = 4;
pub const TEGRA30_CLK_TIMER: c_int = 5;
pub const TEGRA30_CLK_UARTA: c_int = 6;
// 7 (register bit affects uartb and vfir)
pub const TEGRA30_CLK_GPIO: c_int = 8;
pub const TEGRA30_CLK_SDMMC2: c_int = 9;
// 10 (register bit affects spdif_in and spdif_out)
pub const TEGRA30_CLK_I2S1: c_int = 11;
pub const TEGRA30_CLK_I2C1: c_int = 12;
pub const TEGRA30_CLK_NDFLASH: c_int = 13;
pub const TEGRA30_CLK_SDMMC1: c_int = 14;
pub const TEGRA30_CLK_SDMMC4: c_int = 15;
// 16
pub const TEGRA30_CLK_PWM: c_int = 17;
pub const TEGRA30_CLK_I2S2: c_int = 18;
pub const TEGRA30_CLK_EPP: c_int = 19;
// 20 (register bit affects vi and vi_sensor)
pub const TEGRA30_CLK_GR2D: c_int = 21;
pub const TEGRA30_CLK_USBD: c_int = 22;
pub const TEGRA30_CLK_ISP: c_int = 23;
pub const TEGRA30_CLK_GR3D: c_int = 24;
// 25
pub const TEGRA30_CLK_DISP2: c_int = 26;
pub const TEGRA30_CLK_DISP1: c_int = 27;
pub const TEGRA30_CLK_HOST1X: c_int = 28;
pub const TEGRA30_CLK_VCP: c_int = 29;
pub const TEGRA30_CLK_I2S0: c_int = 30;
pub const TEGRA30_CLK_COP_CACHE: c_int = 31;
pub const TEGRA30_CLK_MC: c_int = 32;
pub const TEGRA30_CLK_AHBDMA: c_int = 33;
pub const TEGRA30_CLK_APBDMA: c_int = 34;
// 35
pub const TEGRA30_CLK_KBC: c_int = 36;
pub const TEGRA30_CLK_STATMON: c_int = 37;
pub const TEGRA30_CLK_PMC: c_int = 38;
// 39 (register bit affects fuse and fuse_burn)
pub const TEGRA30_CLK_KFUSE: c_int = 40;
pub const TEGRA30_CLK_SBC1: c_int = 41;
pub const TEGRA30_CLK_NOR: c_int = 42;
// 43
pub const TEGRA30_CLK_SBC2: c_int = 44;
// 45
pub const TEGRA30_CLK_SBC3: c_int = 46;
pub const TEGRA30_CLK_I2C5: c_int = 47;
pub const TEGRA30_CLK_DSIA: c_int = 48;
// 49 (register bit affects cve and tvo)
pub const TEGRA30_CLK_MIPI: c_int = 50;
pub const TEGRA30_CLK_HDMI: c_int = 51;
pub const TEGRA30_CLK_CSI: c_int = 52;
pub const TEGRA30_CLK_TVDAC: c_int = 53;
pub const TEGRA30_CLK_I2C2: c_int = 54;
pub const TEGRA30_CLK_UARTC: c_int = 55;
// 56
pub const TEGRA30_CLK_EMC: c_int = 57;
pub const TEGRA30_CLK_USB2: c_int = 58;
pub const TEGRA30_CLK_USB3: c_int = 59;
pub const TEGRA30_CLK_MPE: c_int = 60;
pub const TEGRA30_CLK_VDE: c_int = 61;
pub const TEGRA30_CLK_BSEA: c_int = 62;
pub const TEGRA30_CLK_BSEV: c_int = 63;
pub const TEGRA30_CLK_SPEEDO: c_int = 64;
pub const TEGRA30_CLK_UARTD: c_int = 65;
pub const TEGRA30_CLK_UARTE: c_int = 66;
pub const TEGRA30_CLK_I2C3: c_int = 67;
pub const TEGRA30_CLK_SBC4: c_int = 68;
pub const TEGRA30_CLK_SDMMC3: c_int = 69;
pub const TEGRA30_CLK_PCIE: c_int = 70;
pub const TEGRA30_CLK_OWR: c_int = 71;
pub const TEGRA30_CLK_AFI: c_int = 72;
pub const TEGRA30_CLK_CSITE: c_int = 73;
// 74
pub const TEGRA30_CLK_AVPUCQ: c_int = 75;
pub const TEGRA30_CLK_LA: c_int = 76;
// 77
// 78
pub const TEGRA30_CLK_DTV: c_int = 79;
pub const TEGRA30_CLK_NDSPEED: c_int = 80;
pub const TEGRA30_CLK_I2CSLOW: c_int = 81;
pub const TEGRA30_CLK_DSIB: c_int = 82;
// 83
pub const TEGRA30_CLK_IRAMA: c_int = 84;
pub const TEGRA30_CLK_IRAMB: c_int = 85;
pub const TEGRA30_CLK_IRAMC: c_int = 86;
pub const TEGRA30_CLK_IRAMD: c_int = 87;
pub const TEGRA30_CLK_CRAM2: c_int = 88;
// 89

// 91
pub const TEGRA30_CLK_CSUS: c_int = 92;
pub const TEGRA30_CLK_CDEV2: c_int = 93;
pub const TEGRA30_CLK_CDEV1: c_int = 94;
// 95
pub const TEGRA30_CLK_CPU_G: c_int = 96;
pub const TEGRA30_CLK_CPU_LP: c_int = 97;
pub const TEGRA30_CLK_GR3D2: c_int = 98;
pub const TEGRA30_CLK_MSELECT: c_int = 99;
pub const TEGRA30_CLK_TSENSOR: c_int = 100;
pub const TEGRA30_CLK_I2S3: c_int = 101;
pub const TEGRA30_CLK_I2S4: c_int = 102;
pub const TEGRA30_CLK_I2C4: c_int = 103;
pub const TEGRA30_CLK_SBC5: c_int = 104;
pub const TEGRA30_CLK_SBC6: c_int = 105;
pub const TEGRA30_CLK_D_AUDIO: c_int = 106;
pub const TEGRA30_CLK_APBIF: c_int = 107;
pub const TEGRA30_CLK_DAM0: c_int = 108;
pub const TEGRA30_CLK_DAM1: c_int = 109;
pub const TEGRA30_CLK_DAM2: c_int = 110;
pub const TEGRA30_CLK_HDA2CODEC_2X: c_int = 111;
pub const TEGRA30_CLK_ATOMICS: c_int = 112;
pub const TEGRA30_CLK_AUDIO0_2X: c_int = 113;
pub const TEGRA30_CLK_AUDIO1_2X: c_int = 114;
pub const TEGRA30_CLK_AUDIO2_2X: c_int = 115;
pub const TEGRA30_CLK_AUDIO3_2X: c_int = 116;
pub const TEGRA30_CLK_AUDIO4_2X: c_int = 117;
pub const TEGRA30_CLK_SPDIF_2X: c_int = 118;
pub const TEGRA30_CLK_ACTMON: c_int = 119;
pub const TEGRA30_CLK_EXTERN1: c_int = 120;
pub const TEGRA30_CLK_EXTERN2: c_int = 121;
pub const TEGRA30_CLK_EXTERN3: c_int = 122;
pub const TEGRA30_CLK_SATA_OOB: c_int = 123;
pub const TEGRA30_CLK_SATA: c_int = 124;
pub const TEGRA30_CLK_HDA: c_int = 125;
// 126
pub const TEGRA30_CLK_SE: c_int = 127;
pub const TEGRA30_CLK_HDA2HDMI: c_int = 128;
pub const TEGRA30_CLK_SATA_COLD: c_int = 129;
// 130
// 131
// 132
// 133
// 134
// 135
pub const TEGRA30_CLK_CEC: c_int = 136;
// 137
// 138
// 139
// 140
// 141
// 142
// 143
// 144
// 145
// 146
// 147
// 148
// 149
// 150
// 151
// 152
// 153
// 154
// 155
// 156
// 157
// 158
// 159
pub const TEGRA30_CLK_UARTB: c_int = 160;
pub const TEGRA30_CLK_VFIR: c_int = 161;
pub const TEGRA30_CLK_SPDIF_IN: c_int = 162;
pub const TEGRA30_CLK_SPDIF_OUT: c_int = 163;
pub const TEGRA30_CLK_VI: c_int = 164;
pub const TEGRA30_CLK_VI_SENSOR: c_int = 165;
pub const TEGRA30_CLK_FUSE: c_int = 166;
pub const TEGRA30_CLK_FUSE_BURN: c_int = 167;
pub const TEGRA30_CLK_CVE: c_int = 168;
pub const TEGRA30_CLK_TVO: c_int = 169;
pub const TEGRA30_CLK_CLK_32K: c_int = 170;
pub const TEGRA30_CLK_CLK_M: c_int = 171;
pub const TEGRA30_CLK_CLK_M_DIV2: c_int = 172;
pub const TEGRA30_CLK_CLK_M_DIV4: c_int = 173;
pub const TEGRA30_CLK_OSC_DIV2: c_int = 172;
pub const TEGRA30_CLK_OSC_DIV4: c_int = 173;
pub const TEGRA30_CLK_PLL_REF: c_int = 174;
pub const TEGRA30_CLK_PLL_C: c_int = 175;
pub const TEGRA30_CLK_PLL_C_OUT1: c_int = 176;
pub const TEGRA30_CLK_PLL_M: c_int = 177;
pub const TEGRA30_CLK_PLL_M_OUT1: c_int = 178;
pub const TEGRA30_CLK_PLL_P: c_int = 179;
pub const TEGRA30_CLK_PLL_P_OUT1: c_int = 180;
pub const TEGRA30_CLK_PLL_P_OUT2: c_int = 181;
pub const TEGRA30_CLK_PLL_P_OUT3: c_int = 182;
pub const TEGRA30_CLK_PLL_P_OUT4: c_int = 183;
pub const TEGRA30_CLK_PLL_A: c_int = 184;
pub const TEGRA30_CLK_PLL_A_OUT0: c_int = 185;
pub const TEGRA30_CLK_PLL_D: c_int = 186;
pub const TEGRA30_CLK_PLL_D_OUT0: c_int = 187;
pub const TEGRA30_CLK_PLL_D2: c_int = 188;
pub const TEGRA30_CLK_PLL_D2_OUT0: c_int = 189;
pub const TEGRA30_CLK_PLL_U: c_int = 190;
pub const TEGRA30_CLK_PLL_X: c_int = 191;
pub const TEGRA30_CLK_PLL_X_OUT0: c_int = 192;
pub const TEGRA30_CLK_PLL_E: c_int = 193;
pub const TEGRA30_CLK_SPDIF_IN_SYNC: c_int = 194;
pub const TEGRA30_CLK_I2S0_SYNC: c_int = 195;
pub const TEGRA30_CLK_I2S1_SYNC: c_int = 196;
pub const TEGRA30_CLK_I2S2_SYNC: c_int = 197;
pub const TEGRA30_CLK_I2S3_SYNC: c_int = 198;
pub const TEGRA30_CLK_I2S4_SYNC: c_int = 199;
pub const TEGRA30_CLK_VIMCLK_SYNC: c_int = 200;
pub const TEGRA30_CLK_AUDIO0: c_int = 201;
pub const TEGRA30_CLK_AUDIO1: c_int = 202;
pub const TEGRA30_CLK_AUDIO2: c_int = 203;
pub const TEGRA30_CLK_AUDIO3: c_int = 204;
pub const TEGRA30_CLK_AUDIO4: c_int = 205;
pub const TEGRA30_CLK_SPDIF: c_int = 206;
// 207
// 208
// 209
pub const TEGRA30_CLK_SCLK: c_int = 210;
// 211
pub const TEGRA30_CLK_CCLK_G: c_int = 212;
pub const TEGRA30_CLK_CCLK_LP: c_int = 213;
pub const TEGRA30_CLK_TWD: c_int = 214;
pub const TEGRA30_CLK_CML0: c_int = 215;
pub const TEGRA30_CLK_CML1: c_int = 216;
pub const TEGRA30_CLK_HCLK: c_int = 217;
pub const TEGRA30_CLK_PCLK: c_int = 218;
// 219
pub const TEGRA30_CLK_OSC: c_int = 220;
// 221
// 222
// 223
// 288
// 289
// 290
// 291
// 292
// 293
// 294
// 295
// 296
// 297
// 298
// 299
// 300
// 301
// 302
pub const TEGRA30_CLK_AUDIO0_MUX: c_int = 303;
pub const TEGRA30_CLK_AUDIO1_MUX: c_int = 304;
pub const TEGRA30_CLK_AUDIO2_MUX: c_int = 305;
pub const TEGRA30_CLK_AUDIO3_MUX: c_int = 306;
pub const TEGRA30_CLK_AUDIO4_MUX: c_int = 307;
pub const TEGRA30_CLK_SPDIF_MUX: c_int = 308;
pub const TEGRA30_CLK_CSIA_PAD: c_int = 309;
pub const TEGRA30_CLK_CSIB_PAD: c_int = 310;
