//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tegra114-car.h
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
// This header provides constants for binding nvidia,tegra114-car.
//
// The first 160 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
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
// 0
// 1
// 2
// 3
pub const TEGRA114_CLK_RTC: c_int = 4;
pub const TEGRA114_CLK_TIMER: c_int = 5;
pub const TEGRA114_CLK_UARTA: c_int = 6;
// 7 (register bit affects uartb and vfir)
// 8
pub const TEGRA114_CLK_SDMMC2: c_int = 9;
// 10 (register bit affects spdif_in and spdif_out)
pub const TEGRA114_CLK_I2S1: c_int = 11;
pub const TEGRA114_CLK_I2C1: c_int = 12;
pub const TEGRA114_CLK_NDFLASH: c_int = 13;
pub const TEGRA114_CLK_SDMMC1: c_int = 14;
pub const TEGRA114_CLK_SDMMC4: c_int = 15;
// 16
pub const TEGRA114_CLK_PWM: c_int = 17;
pub const TEGRA114_CLK_I2S2: c_int = 18;
pub const TEGRA114_CLK_EPP: c_int = 19;
// 20 (register bit affects vi and vi_sensor)
pub const TEGRA114_CLK_GR2D: c_int = 21;
pub const TEGRA114_CLK_USBD: c_int = 22;
pub const TEGRA114_CLK_ISP: c_int = 23;
pub const TEGRA114_CLK_GR3D: c_int = 24;
// 25
pub const TEGRA114_CLK_DISP2: c_int = 26;
pub const TEGRA114_CLK_DISP1: c_int = 27;
pub const TEGRA114_CLK_HOST1X: c_int = 28;
pub const TEGRA114_CLK_VCP: c_int = 29;
pub const TEGRA114_CLK_I2S0: c_int = 30;
// 31
pub const TEGRA114_CLK_MC: c_int = 32;
// 33
pub const TEGRA114_CLK_APBDMA: c_int = 34;
// 35
pub const TEGRA114_CLK_KBC: c_int = 36;
// 37
// 38
// 39 (register bit affects fuse and fuse_burn)
pub const TEGRA114_CLK_KFUSE: c_int = 40;
pub const TEGRA114_CLK_SBC1: c_int = 41;
pub const TEGRA114_CLK_NOR: c_int = 42;
// 43
pub const TEGRA114_CLK_SBC2: c_int = 44;
// 45
pub const TEGRA114_CLK_SBC3: c_int = 46;
pub const TEGRA114_CLK_I2C5: c_int = 47;
pub const TEGRA114_CLK_DSIA: c_int = 48;
// 49
pub const TEGRA114_CLK_MIPI: c_int = 50;
pub const TEGRA114_CLK_HDMI: c_int = 51;
pub const TEGRA114_CLK_CSI: c_int = 52;
// 53
pub const TEGRA114_CLK_I2C2: c_int = 54;
pub const TEGRA114_CLK_UARTC: c_int = 55;
pub const TEGRA114_CLK_MIPI_CAL: c_int = 56;
pub const TEGRA114_CLK_EMC: c_int = 57;
pub const TEGRA114_CLK_USB2: c_int = 58;
pub const TEGRA114_CLK_USB3: c_int = 59;
// 60
pub const TEGRA114_CLK_VDE: c_int = 61;
pub const TEGRA114_CLK_BSEA: c_int = 62;
pub const TEGRA114_CLK_BSEV: c_int = 63;
// 64
pub const TEGRA114_CLK_UARTD: c_int = 65;
// 66
pub const TEGRA114_CLK_I2C3: c_int = 67;
pub const TEGRA114_CLK_SBC4: c_int = 68;
pub const TEGRA114_CLK_SDMMC3: c_int = 69;
// 70
pub const TEGRA114_CLK_OWR: c_int = 71;
// 72
pub const TEGRA114_CLK_CSITE: c_int = 73;
// 74
// 75
pub const TEGRA114_CLK_LA: c_int = 76;
pub const TEGRA114_CLK_TRACE: c_int = 77;
pub const TEGRA114_CLK_SOC_THERM: c_int = 78;
pub const TEGRA114_CLK_DTV: c_int = 79;
pub const TEGRA114_CLK_NDSPEED: c_int = 80;
pub const TEGRA114_CLK_I2CSLOW: c_int = 81;
pub const TEGRA114_CLK_DSIB: c_int = 82;
pub const TEGRA114_CLK_TSEC: c_int = 83;
// 84
// 85
// 86
// 87
// 88
pub const TEGRA114_CLK_XUSB_HOST: c_int = 89;
// 90
pub const TEGRA114_CLK_MSENC: c_int = 91;
pub const TEGRA114_CLK_CSUS: c_int = 92;
// 93
// 94
// 95 (bit affects xusb_dev and xusb_dev_src)
// 96
// 97
// 98
pub const TEGRA114_CLK_MSELECT: c_int = 99;
pub const TEGRA114_CLK_TSENSOR: c_int = 100;
pub const TEGRA114_CLK_I2S3: c_int = 101;
pub const TEGRA114_CLK_I2S4: c_int = 102;
pub const TEGRA114_CLK_I2C4: c_int = 103;
pub const TEGRA114_CLK_SBC5: c_int = 104;
pub const TEGRA114_CLK_SBC6: c_int = 105;
pub const TEGRA114_CLK_D_AUDIO: c_int = 106;
pub const TEGRA114_CLK_APBIF: c_int = 107;
pub const TEGRA114_CLK_DAM0: c_int = 108;
pub const TEGRA114_CLK_DAM1: c_int = 109;
pub const TEGRA114_CLK_DAM2: c_int = 110;
pub const TEGRA114_CLK_HDA2CODEC_2X: c_int = 111;
// 112
pub const TEGRA114_CLK_AUDIO0_2X: c_int = 113;
pub const TEGRA114_CLK_AUDIO1_2X: c_int = 114;
pub const TEGRA114_CLK_AUDIO2_2X: c_int = 115;
pub const TEGRA114_CLK_AUDIO3_2X: c_int = 116;
pub const TEGRA114_CLK_AUDIO4_2X: c_int = 117;
pub const TEGRA114_CLK_SPDIF_2X: c_int = 118;
pub const TEGRA114_CLK_ACTMON: c_int = 119;
pub const TEGRA114_CLK_EXTERN1: c_int = 120;
pub const TEGRA114_CLK_EXTERN2: c_int = 121;
pub const TEGRA114_CLK_EXTERN3: c_int = 122;
// 123
// 124
pub const TEGRA114_CLK_HDA: c_int = 125;
// 126
pub const TEGRA114_CLK_SE: c_int = 127;
pub const TEGRA114_CLK_HDA2HDMI: c_int = 128;
// 129
// 130
// 131
// 132
// 133
// 134
// 135
pub const TEGRA114_CLK_CEC: c_int = 136;
// 137
// 138
// 139
// 140
// 141
// 142
// 143 (bit affects xusb_falcon_src, xusb_fs_src,
// xusb_host_src and xusb_ss_src)
pub const TEGRA114_CLK_CILAB: c_int = 144;
pub const TEGRA114_CLK_CILCD: c_int = 145;
pub const TEGRA114_CLK_CILE: c_int = 146;
pub const TEGRA114_CLK_DSIALP: c_int = 147;
pub const TEGRA114_CLK_DSIBLP: c_int = 148;
// 149
pub const TEGRA114_CLK_DDS: c_int = 150;
// 151
pub const TEGRA114_CLK_DP2: c_int = 152;
pub const TEGRA114_CLK_AMX: c_int = 153;
pub const TEGRA114_CLK_ADX: c_int = 154;
// 155 (bit affects dfll_ref and dfll_soc)
pub const TEGRA114_CLK_XUSB_SS: c_int = 156;
// 157
// 158
// 159
// 160
// 161
// 162
// 163
// 164
// 165
// 166
// 167
// 168
// 169
// 170
// 171
// 172
// 173
// 174
// 175
// 176
// 177
// 178
// 179
// 180
// 181
// 182
// 183
// 184
// 185
// 186
// 187
// 188
// 189
// 190
// 191
pub const TEGRA114_CLK_UARTB: c_int = 192;
pub const TEGRA114_CLK_VFIR: c_int = 193;
pub const TEGRA114_CLK_SPDIF_IN: c_int = 194;
pub const TEGRA114_CLK_SPDIF_OUT: c_int = 195;
pub const TEGRA114_CLK_VI: c_int = 196;
pub const TEGRA114_CLK_VI_SENSOR: c_int = 197;
pub const TEGRA114_CLK_FUSE: c_int = 198;
pub const TEGRA114_CLK_FUSE_BURN: c_int = 199;
pub const TEGRA114_CLK_CLK_32K: c_int = 200;
pub const TEGRA114_CLK_CLK_M: c_int = 201;
pub const TEGRA114_CLK_CLK_M_DIV2: c_int = 202;
pub const TEGRA114_CLK_CLK_M_DIV4: c_int = 203;
pub const TEGRA114_CLK_OSC_DIV2: c_int = 202;
pub const TEGRA114_CLK_OSC_DIV4: c_int = 203;
pub const TEGRA114_CLK_PLL_REF: c_int = 204;
pub const TEGRA114_CLK_PLL_C: c_int = 205;
pub const TEGRA114_CLK_PLL_C_OUT1: c_int = 206;
pub const TEGRA114_CLK_PLL_C2: c_int = 207;
pub const TEGRA114_CLK_PLL_C3: c_int = 208;
pub const TEGRA114_CLK_PLL_M: c_int = 209;
pub const TEGRA114_CLK_PLL_M_OUT1: c_int = 210;
pub const TEGRA114_CLK_PLL_P: c_int = 211;
pub const TEGRA114_CLK_PLL_P_OUT1: c_int = 212;
pub const TEGRA114_CLK_PLL_P_OUT2: c_int = 213;
pub const TEGRA114_CLK_PLL_P_OUT3: c_int = 214;
pub const TEGRA114_CLK_PLL_P_OUT4: c_int = 215;
pub const TEGRA114_CLK_PLL_A: c_int = 216;
pub const TEGRA114_CLK_PLL_A_OUT0: c_int = 217;
pub const TEGRA114_CLK_PLL_D: c_int = 218;
pub const TEGRA114_CLK_PLL_D_OUT0: c_int = 219;
pub const TEGRA114_CLK_PLL_D2: c_int = 220;
pub const TEGRA114_CLK_PLL_D2_OUT0: c_int = 221;
pub const TEGRA114_CLK_PLL_U: c_int = 222;
pub const TEGRA114_CLK_PLL_U_480M: c_int = 223;
pub const TEGRA114_CLK_PLL_U_60M: c_int = 224;
pub const TEGRA114_CLK_PLL_U_48M: c_int = 225;
pub const TEGRA114_CLK_PLL_U_12M: c_int = 226;
pub const TEGRA114_CLK_PLL_X: c_int = 227;
pub const TEGRA114_CLK_PLL_X_OUT0: c_int = 228;
pub const TEGRA114_CLK_PLL_RE_VCO: c_int = 229;
pub const TEGRA114_CLK_PLL_RE_OUT: c_int = 230;
pub const TEGRA114_CLK_PLL_E_OUT0: c_int = 231;
pub const TEGRA114_CLK_SPDIF_IN_SYNC: c_int = 232;
pub const TEGRA114_CLK_I2S0_SYNC: c_int = 233;
pub const TEGRA114_CLK_I2S1_SYNC: c_int = 234;
pub const TEGRA114_CLK_I2S2_SYNC: c_int = 235;
pub const TEGRA114_CLK_I2S3_SYNC: c_int = 236;
pub const TEGRA114_CLK_I2S4_SYNC: c_int = 237;
pub const TEGRA114_CLK_VIMCLK_SYNC: c_int = 238;
pub const TEGRA114_CLK_AUDIO0: c_int = 239;
pub const TEGRA114_CLK_AUDIO1: c_int = 240;
pub const TEGRA114_CLK_AUDIO2: c_int = 241;
pub const TEGRA114_CLK_AUDIO3: c_int = 242;
pub const TEGRA114_CLK_AUDIO4: c_int = 243;
pub const TEGRA114_CLK_SPDIF: c_int = 244;
// 245
// 246
// 247
// 248
pub const TEGRA114_CLK_OSC: c_int = 249;
// 250
// 251
pub const TEGRA114_CLK_XUSB_HOST_SRC: c_int = 252;
pub const TEGRA114_CLK_XUSB_FALCON_SRC: c_int = 253;
pub const TEGRA114_CLK_XUSB_FS_SRC: c_int = 254;
pub const TEGRA114_CLK_XUSB_SS_SRC: c_int = 255;
pub const TEGRA114_CLK_XUSB_DEV_SRC: c_int = 256;
pub const TEGRA114_CLK_XUSB_DEV: c_int = 257;
pub const TEGRA114_CLK_XUSB_HS_SRC: c_int = 258;
pub const TEGRA114_CLK_SCLK: c_int = 259;
pub const TEGRA114_CLK_HCLK: c_int = 260;
pub const TEGRA114_CLK_PCLK: c_int = 261;
pub const TEGRA114_CLK_CCLK_G: c_int = 262;
pub const TEGRA114_CLK_CCLK_LP: c_int = 263;
pub const TEGRA114_CLK_DFLL_REF: c_int = 264;
pub const TEGRA114_CLK_DFLL_SOC: c_int = 265;
// 266
// 267
// 268
// 269
// 270
// 271
// 272
// 273
// 274
// 275
// 276
// 277
// 278
// 279
// 280
// 281
// 282
// 283
// 284
// 285
// 286
// 287
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
pub const TEGRA114_CLK_AUDIO0_MUX: c_int = 300;
pub const TEGRA114_CLK_AUDIO1_MUX: c_int = 301;
pub const TEGRA114_CLK_AUDIO2_MUX: c_int = 302;
pub const TEGRA114_CLK_AUDIO3_MUX: c_int = 303;
pub const TEGRA114_CLK_AUDIO4_MUX: c_int = 304;
pub const TEGRA114_CLK_SPDIF_MUX: c_int = 305;
// 306
// 307
// 308
pub const TEGRA114_CLK_DSIA_MUX: c_int = 309;
pub const TEGRA114_CLK_DSIB_MUX: c_int = 310;
pub const TEGRA114_CLK_XUSB_SS_DIV2: c_int = 311;
pub const TEGRA114_CLK_CLK_MAX: c_int = 312;
