//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tegra210-car.h
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
// This header provides constants for binding nvidia,tegra210-car.
//
// The first 224 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
// registers. These IDs often match those in the CAR's RST_DEVICES registers,
// but not in all cases. Some bits in CLK_OUT_ENB affect multiple clocks. In
// this case, those clocks are assigned IDs above 224 in order to highlight
// this issue. Implementations that interpret these clock IDs as bit values
// within the CLK_OUT_ENB or RST_DEVICES registers should be careful to
// explicitly handle these special cases.
//
// The balance of the clocks controlled by the CAR are assigned IDs of 224 and
// above.
//
// 0
// 1
// 2
pub const TEGRA210_CLK_ISPB: c_int = 3;
pub const TEGRA210_CLK_RTC: c_int = 4;
pub const TEGRA210_CLK_TIMER: c_int = 5;
pub const TEGRA210_CLK_UARTA: c_int = 6;
// 7 (register bit affects uartb and vfir)
pub const TEGRA210_CLK_GPIO: c_int = 8;
pub const TEGRA210_CLK_SDMMC2: c_int = 9;
// 10 (register bit affects spdif_in and spdif_out)
pub const TEGRA210_CLK_I2S1: c_int = 11;
pub const TEGRA210_CLK_I2C1: c_int = 12;
// 13
pub const TEGRA210_CLK_SDMMC1: c_int = 14;
pub const TEGRA210_CLK_SDMMC4: c_int = 15;
// 16
pub const TEGRA210_CLK_PWM: c_int = 17;
pub const TEGRA210_CLK_I2S2: c_int = 18;
// 19
// 20 (register bit affects vi and vi_sensor)
// 21
pub const TEGRA210_CLK_USBD: c_int = 22;
pub const TEGRA210_CLK_ISPA: c_int = 23;
// 24
// 25
pub const TEGRA210_CLK_DISP2: c_int = 26;
pub const TEGRA210_CLK_DISP1: c_int = 27;
pub const TEGRA210_CLK_HOST1X: c_int = 28;
// 29
pub const TEGRA210_CLK_I2S0: c_int = 30;
// 31
pub const TEGRA210_CLK_MC: c_int = 32;
pub const TEGRA210_CLK_AHBDMA: c_int = 33;
pub const TEGRA210_CLK_APBDMA: c_int = 34;
// 35
// 36
// 37
pub const TEGRA210_CLK_PMC: c_int = 38;
// 39 (register bit affects fuse and fuse_burn)
pub const TEGRA210_CLK_KFUSE: c_int = 40;
pub const TEGRA210_CLK_SBC1: c_int = 41;
// 42
// 43
pub const TEGRA210_CLK_SBC2: c_int = 44;
// 45
pub const TEGRA210_CLK_SBC3: c_int = 46;
pub const TEGRA210_CLK_I2C5: c_int = 47;
pub const TEGRA210_CLK_DSIA: c_int = 48;
// 49
// 50
// 51
pub const TEGRA210_CLK_CSI: c_int = 52;
// 53
pub const TEGRA210_CLK_I2C2: c_int = 54;
pub const TEGRA210_CLK_UARTC: c_int = 55;
pub const TEGRA210_CLK_MIPI_CAL: c_int = 56;
pub const TEGRA210_CLK_EMC: c_int = 57;
pub const TEGRA210_CLK_USB2: c_int = 58;
// 59
// 60
// 61
// 62
pub const TEGRA210_CLK_BSEV: c_int = 63;
// 64
pub const TEGRA210_CLK_UARTD: c_int = 65;
// 66
pub const TEGRA210_CLK_I2C3: c_int = 67;
pub const TEGRA210_CLK_SBC4: c_int = 68;
pub const TEGRA210_CLK_SDMMC3: c_int = 69;
pub const TEGRA210_CLK_PCIE: c_int = 70;
pub const TEGRA210_CLK_OWR: c_int = 71;
pub const TEGRA210_CLK_AFI: c_int = 72;
pub const TEGRA210_CLK_CSITE: c_int = 73;
// 74
// 75
pub const TEGRA210_CLK_LA: c_int = 76;
// 77
pub const TEGRA210_CLK_SOC_THERM: c_int = 78;
pub const TEGRA210_CLK_DTV: c_int = 79;
// 80
pub const TEGRA210_CLK_I2CSLOW: c_int = 81;
pub const TEGRA210_CLK_DSIB: c_int = 82;
pub const TEGRA210_CLK_TSEC: c_int = 83;
// 84
// 85
// 86
// 87
// 88
pub const TEGRA210_CLK_XUSB_HOST: c_int = 89;
// 90
// 91
pub const TEGRA210_CLK_CSUS: c_int = 92;
// 93
// 94
// 95 (bit affects xusb_dev and xusb_dev_src)
// 96
// 97
// 98
pub const TEGRA210_CLK_MSELECT: c_int = 99;
pub const TEGRA210_CLK_TSENSOR: c_int = 100;
pub const TEGRA210_CLK_I2S3: c_int = 101;
pub const TEGRA210_CLK_I2S4: c_int = 102;
pub const TEGRA210_CLK_I2C4: c_int = 103;
// 104
// 105
pub const TEGRA210_CLK_D_AUDIO: c_int = 106;
pub const TEGRA210_CLK_APB2APE: c_int = 107;
// 108
// 109
// 110
pub const TEGRA210_CLK_HDA2CODEC_2X: c_int = 111;
// 112
// 113
// 114
// 115
// 116
// 117
pub const TEGRA210_CLK_SPDIF_2X: c_int = 118;
pub const TEGRA210_CLK_ACTMON: c_int = 119;
pub const TEGRA210_CLK_EXTERN1: c_int = 120;
pub const TEGRA210_CLK_EXTERN2: c_int = 121;
pub const TEGRA210_CLK_EXTERN3: c_int = 122;
pub const TEGRA210_CLK_SATA_OOB: c_int = 123;
pub const TEGRA210_CLK_SATA: c_int = 124;
pub const TEGRA210_CLK_HDA: c_int = 125;
// 126
// 127
pub const TEGRA210_CLK_HDA2HDMI: c_int = 128;
// 129
// 130
// 131
// 132
// 133
// 134
// 135
pub const TEGRA210_CLK_CEC: c_int = 136;
// 137
// 138
// 139
// 140
// 141
// 142
// (bit affects xusb_falcon_src, xusb_fs_src, xusb_host_src and xusb_ss_src)
pub const TEGRA210_CLK_XUSB_GATE: c_int = 143;
pub const TEGRA210_CLK_CILAB: c_int = 144;
pub const TEGRA210_CLK_CILCD: c_int = 145;
pub const TEGRA210_CLK_CILE: c_int = 146;
pub const TEGRA210_CLK_DSIALP: c_int = 147;
pub const TEGRA210_CLK_DSIBLP: c_int = 148;
pub const TEGRA210_CLK_ENTROPY: c_int = 149;
// 150
// 151
pub const TEGRA210_CLK_DP2: c_int = 152;
// 153
// 154
// 155 (bit affects dfll_ref and dfll_soc)
pub const TEGRA210_CLK_XUSB_SS: c_int = 156;
// 157
// 158
// 159
// 160
pub const TEGRA210_CLK_DMIC1: c_int = 161;
pub const TEGRA210_CLK_DMIC2: c_int = 162;
// 163
// 164
// 165
pub const TEGRA210_CLK_I2C6: c_int = 166;
// 167
// 168
// 169
// 170
pub const TEGRA210_CLK_VIM2_CLK: c_int = 171;
// 172
pub const TEGRA210_CLK_MIPIBIF: c_int = 173;
// 174
// 175
// 176
pub const TEGRA210_CLK_CLK72MHZ: c_int = 177;
pub const TEGRA210_CLK_VIC03: c_int = 178;
// 179
// 180
pub const TEGRA210_CLK_DPAUX: c_int = 181;
pub const TEGRA210_CLK_SOR0: c_int = 182;
pub const TEGRA210_CLK_SOR1: c_int = 183;
pub const TEGRA210_CLK_GPU: c_int = 184;
pub const TEGRA210_CLK_DBGAPB: c_int = 185;
// 186
pub const TEGRA210_CLK_PLL_P_OUT_ADSP: c_int = 187;
// 188 ((bit affects pll_a_out_adsp and pll_a_out0_out_adsp)
pub const TEGRA210_CLK_PLL_G_REF: c_int = 189;
// 190
// 191
// 192
pub const TEGRA210_CLK_SDMMC_LEGACY: c_int = 193;
pub const TEGRA210_CLK_NVDEC: c_int = 194;
pub const TEGRA210_CLK_NVJPG: c_int = 195;
// 196
pub const TEGRA210_CLK_DMIC3: c_int = 197;
pub const TEGRA210_CLK_APE: c_int = 198;
pub const TEGRA210_CLK_ADSP: c_int = 199;
// 200
// 201
pub const TEGRA210_CLK_MAUD: c_int = 202;
// 203
// 204
// 205
pub const TEGRA210_CLK_TSECB: c_int = 206;
pub const TEGRA210_CLK_DPAUX1: c_int = 207;
pub const TEGRA210_CLK_VI_I2C: c_int = 208;
pub const TEGRA210_CLK_HSIC_TRK: c_int = 209;
pub const TEGRA210_CLK_USB2_TRK: c_int = 210;
pub const TEGRA210_CLK_QSPI: c_int = 211;
pub const TEGRA210_CLK_UARTAPE: c_int = 212;
// 213
// 214
// 215
// 216
// 217
pub const TEGRA210_CLK_ADSP_NEON: c_int = 218;
pub const TEGRA210_CLK_NVENC: c_int = 219;
pub const TEGRA210_CLK_IQC2: c_int = 220;
pub const TEGRA210_CLK_IQC1: c_int = 221;
pub const TEGRA210_CLK_SOR_SAFE: c_int = 222;
pub const TEGRA210_CLK_PLL_P_OUT_CPU: c_int = 223;
pub const TEGRA210_CLK_UARTB: c_int = 224;
pub const TEGRA210_CLK_VFIR: c_int = 225;
pub const TEGRA210_CLK_SPDIF_IN: c_int = 226;
pub const TEGRA210_CLK_SPDIF_OUT: c_int = 227;
pub const TEGRA210_CLK_VI: c_int = 228;
pub const TEGRA210_CLK_VI_SENSOR: c_int = 229;
pub const TEGRA210_CLK_FUSE: c_int = 230;
pub const TEGRA210_CLK_FUSE_BURN: c_int = 231;
pub const TEGRA210_CLK_CLK_32K: c_int = 232;
pub const TEGRA210_CLK_CLK_M: c_int = 233;
pub const TEGRA210_CLK_CLK_M_DIV2: c_int = 234;
pub const TEGRA210_CLK_CLK_M_DIV4: c_int = 235;
pub const TEGRA210_CLK_OSC_DIV2: c_int = 234;
pub const TEGRA210_CLK_OSC_DIV4: c_int = 235;
pub const TEGRA210_CLK_PLL_REF: c_int = 236;
pub const TEGRA210_CLK_PLL_C: c_int = 237;
pub const TEGRA210_CLK_PLL_C_OUT1: c_int = 238;
pub const TEGRA210_CLK_PLL_C2: c_int = 239;
pub const TEGRA210_CLK_PLL_C3: c_int = 240;
pub const TEGRA210_CLK_PLL_M: c_int = 241;
pub const TEGRA210_CLK_PLL_M_OUT1: c_int = 242;
pub const TEGRA210_CLK_PLL_P: c_int = 243;
pub const TEGRA210_CLK_PLL_P_OUT1: c_int = 244;
pub const TEGRA210_CLK_PLL_P_OUT2: c_int = 245;
pub const TEGRA210_CLK_PLL_P_OUT3: c_int = 246;
pub const TEGRA210_CLK_PLL_P_OUT4: c_int = 247;
pub const TEGRA210_CLK_PLL_A: c_int = 248;
pub const TEGRA210_CLK_PLL_A_OUT0: c_int = 249;
pub const TEGRA210_CLK_PLL_D: c_int = 250;
pub const TEGRA210_CLK_PLL_D_OUT0: c_int = 251;
pub const TEGRA210_CLK_PLL_D2: c_int = 252;
pub const TEGRA210_CLK_PLL_D2_OUT0: c_int = 253;
pub const TEGRA210_CLK_PLL_U: c_int = 254;
pub const TEGRA210_CLK_PLL_U_480M: c_int = 255;
pub const TEGRA210_CLK_PLL_U_60M: c_int = 256;
pub const TEGRA210_CLK_PLL_U_48M: c_int = 257;
// 258
pub const TEGRA210_CLK_PLL_X: c_int = 259;
pub const TEGRA210_CLK_PLL_X_OUT0: c_int = 260;
pub const TEGRA210_CLK_PLL_RE_VCO: c_int = 261;
pub const TEGRA210_CLK_PLL_RE_OUT: c_int = 262;
pub const TEGRA210_CLK_PLL_E: c_int = 263;
pub const TEGRA210_CLK_SPDIF_IN_SYNC: c_int = 264;
pub const TEGRA210_CLK_I2S0_SYNC: c_int = 265;
pub const TEGRA210_CLK_I2S1_SYNC: c_int = 266;
pub const TEGRA210_CLK_I2S2_SYNC: c_int = 267;
pub const TEGRA210_CLK_I2S3_SYNC: c_int = 268;
pub const TEGRA210_CLK_I2S4_SYNC: c_int = 269;
pub const TEGRA210_CLK_VIMCLK_SYNC: c_int = 270;
pub const TEGRA210_CLK_AUDIO0: c_int = 271;
pub const TEGRA210_CLK_AUDIO1: c_int = 272;
pub const TEGRA210_CLK_AUDIO2: c_int = 273;
pub const TEGRA210_CLK_AUDIO3: c_int = 274;
pub const TEGRA210_CLK_AUDIO4: c_int = 275;
pub const TEGRA210_CLK_SPDIF: c_int = 276;
// 277
pub const TEGRA210_CLK_QSPI_PM: c_int = 278;
// 279
// 280

pub const TEGRA210_CLK_SOR0_OUT: c_int = 281;
pub const TEGRA210_CLK_SOR1_OUT: c_int = 282;
// 283
pub const TEGRA210_CLK_XUSB_HOST_SRC: c_int = 284;
pub const TEGRA210_CLK_XUSB_FALCON_SRC: c_int = 285;
pub const TEGRA210_CLK_XUSB_FS_SRC: c_int = 286;
pub const TEGRA210_CLK_XUSB_SS_SRC: c_int = 287;
pub const TEGRA210_CLK_XUSB_DEV_SRC: c_int = 288;
pub const TEGRA210_CLK_XUSB_DEV: c_int = 289;
pub const TEGRA210_CLK_XUSB_HS_SRC: c_int = 290;
pub const TEGRA210_CLK_SCLK: c_int = 291;
pub const TEGRA210_CLK_HCLK: c_int = 292;
pub const TEGRA210_CLK_PCLK: c_int = 293;
pub const TEGRA210_CLK_CCLK_G: c_int = 294;
pub const TEGRA210_CLK_CCLK_LP: c_int = 295;
pub const TEGRA210_CLK_DFLL_REF: c_int = 296;
pub const TEGRA210_CLK_DFLL_SOC: c_int = 297;
pub const TEGRA210_CLK_VI_SENSOR2: c_int = 298;
pub const TEGRA210_CLK_PLL_P_OUT5: c_int = 299;
pub const TEGRA210_CLK_CML0: c_int = 300;
pub const TEGRA210_CLK_CML1: c_int = 301;
pub const TEGRA210_CLK_PLL_C4: c_int = 302;
pub const TEGRA210_CLK_PLL_DP: c_int = 303;
pub const TEGRA210_CLK_PLL_E_MUX: c_int = 304;
pub const TEGRA210_CLK_PLL_MB: c_int = 305;
pub const TEGRA210_CLK_PLL_A1: c_int = 306;
pub const TEGRA210_CLK_PLL_D_DSI_OUT: c_int = 307;
pub const TEGRA210_CLK_PLL_C4_OUT0: c_int = 308;
pub const TEGRA210_CLK_PLL_C4_OUT1: c_int = 309;
pub const TEGRA210_CLK_PLL_C4_OUT2: c_int = 310;
pub const TEGRA210_CLK_PLL_C4_OUT3: c_int = 311;
pub const TEGRA210_CLK_PLL_U_OUT: c_int = 312;
pub const TEGRA210_CLK_PLL_U_OUT1: c_int = 313;
pub const TEGRA210_CLK_PLL_U_OUT2: c_int = 314;
pub const TEGRA210_CLK_USB2_HSIC_TRK: c_int = 315;
pub const TEGRA210_CLK_PLL_P_OUT_HSIO: c_int = 316;
pub const TEGRA210_CLK_PLL_P_OUT_XUSB: c_int = 317;
pub const TEGRA210_CLK_XUSB_SSP_SRC: c_int = 318;
pub const TEGRA210_CLK_PLL_RE_OUT1: c_int = 319;
pub const TEGRA210_CLK_PLL_MB_UD: c_int = 320;
pub const TEGRA210_CLK_PLL_P_UD: c_int = 321;
pub const TEGRA210_CLK_ISP: c_int = 322;
pub const TEGRA210_CLK_PLL_A_OUT_ADSP: c_int = 323;
pub const TEGRA210_CLK_PLL_A_OUT0_OUT_ADSP: c_int = 324;
// 325
pub const TEGRA210_CLK_OSC: c_int = 326;
pub const TEGRA210_CLK_CSI_TPG: c_int = 327;
// 328
// 329
// 330
// 331
// 332
// 333
// 334
// 335
// 336
// 337
// 338
// 339
// 340
// 341
// 342
// 343
// 344
// 345
// 346
// 347
// 348
// 349
pub const TEGRA210_CLK_AUDIO0_MUX: c_int = 350;
pub const TEGRA210_CLK_AUDIO1_MUX: c_int = 351;
pub const TEGRA210_CLK_AUDIO2_MUX: c_int = 352;
pub const TEGRA210_CLK_AUDIO3_MUX: c_int = 353;
pub const TEGRA210_CLK_AUDIO4_MUX: c_int = 354;
pub const TEGRA210_CLK_SPDIF_MUX: c_int = 355;
// 356
// 357
// 358
pub const TEGRA210_CLK_DSIA_MUX: c_int = 359;
pub const TEGRA210_CLK_DSIB_MUX: c_int = 360;
// 361
pub const TEGRA210_CLK_XUSB_SS_DIV2: c_int = 362;
pub const TEGRA210_CLK_PLL_M_UD: c_int = 363;
pub const TEGRA210_CLK_PLL_C_UD: c_int = 364;
pub const TEGRA210_CLK_SCLK_MUX: c_int = 365;
pub const TEGRA210_CLK_ACLK: c_int = 370;
pub const TEGRA210_CLK_DMIC1_SYNC_CLK: c_int = 388;
pub const TEGRA210_CLK_DMIC1_SYNC_CLK_MUX: c_int = 389;
pub const TEGRA210_CLK_DMIC2_SYNC_CLK: c_int = 390;
pub const TEGRA210_CLK_DMIC2_SYNC_CLK_MUX: c_int = 391;
pub const TEGRA210_CLK_DMIC3_SYNC_CLK: c_int = 392;
pub const TEGRA210_CLK_DMIC3_SYNC_CLK_MUX: c_int = 393;
pub const TEGRA210_CLK_CLK_MAX: c_int = 394;
