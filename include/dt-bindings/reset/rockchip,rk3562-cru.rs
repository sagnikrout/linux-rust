//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/rockchip,rk3562-cru.h
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
// Copyright (c) 2024-2025 Rockchip Electronics Co. Ltd.
//
// Author: Elaine Zhang <zhangqing@rock-chips.com>
//
// Name=SOFTRST_CON01,Offset=0x404
pub const SRST_A_TOP_BIU: c_int = 0;
pub const SRST_A_TOP_VIO_BIU: c_int = 1;
pub const SRST_REF_PVTPLL_LOGIC: c_int = 2;
// Name=SOFTRST_CON03,Offset=0x40C
pub const SRST_NCOREPORESET0: c_int = 3;
pub const SRST_NCOREPORESET1: c_int = 4;
pub const SRST_NCOREPORESET2: c_int = 5;
pub const SRST_NCOREPORESET3: c_int = 6;
pub const SRST_NCORESET0: c_int = 7;
pub const SRST_NCORESET1: c_int = 8;
pub const SRST_NCORESET2: c_int = 9;
pub const SRST_NCORESET3: c_int = 10;
pub const SRST_NL2RESET: c_int = 11;
// Name=SOFTRST_CON04,Offset=0x410
pub const SRST_DAP: c_int = 12;
pub const SRST_P_DBG_DAPLITE: c_int = 13;
pub const SRST_REF_PVTPLL_CORE: c_int = 14;
// Name=SOFTRST_CON05,Offset=0x414
pub const SRST_A_CORE_BIU: c_int = 15;
pub const SRST_P_CORE_BIU: c_int = 16;
pub const SRST_H_CORE_BIU: c_int = 17;
// Name=SOFTRST_CON06,Offset=0x418
pub const SRST_A_NPU_BIU: c_int = 18;
pub const SRST_H_NPU_BIU: c_int = 19;
pub const SRST_A_RKNN: c_int = 20;
pub const SRST_H_RKNN: c_int = 21;
pub const SRST_REF_PVTPLL_NPU: c_int = 22;
// Name=SOFTRST_CON08,Offset=0x420
pub const SRST_A_GPU_BIU: c_int = 23;
pub const SRST_GPU: c_int = 24;
pub const SRST_REF_PVTPLL_GPU: c_int = 25;
pub const SRST_GPU_BRG_BIU: c_int = 26;
// Name=SOFTRST_CON09,Offset=0x424
pub const SRST_RKVENC_CORE: c_int = 27;
pub const SRST_A_VEPU_BIU: c_int = 28;
pub const SRST_H_VEPU_BIU: c_int = 29;
pub const SRST_A_RKVENC: c_int = 30;
pub const SRST_H_RKVENC: c_int = 31;
// Name=SOFTRST_CON10,Offset=0x428
pub const SRST_RKVDEC_HEVC_CA: c_int = 32;
pub const SRST_A_VDPU_BIU: c_int = 33;
pub const SRST_H_VDPU_BIU: c_int = 34;
pub const SRST_A_RKVDEC: c_int = 35;
pub const SRST_H_RKVDEC: c_int = 36;
// Name=SOFTRST_CON11,Offset=0x42C
pub const SRST_A_VI_BIU: c_int = 37;
pub const SRST_H_VI_BIU: c_int = 38;
pub const SRST_P_VI_BIU: c_int = 39;
pub const SRST_ISP: c_int = 40;
pub const SRST_A_VICAP: c_int = 41;
pub const SRST_H_VICAP: c_int = 42;
pub const SRST_D_VICAP: c_int = 43;
pub const SRST_I0_VICAP: c_int = 44;
pub const SRST_I1_VICAP: c_int = 45;
pub const SRST_I2_VICAP: c_int = 46;
pub const SRST_I3_VICAP: c_int = 47;
// Name=SOFTRST_CON12,Offset=0x430
pub const SRST_P_CSIHOST0: c_int = 48;
pub const SRST_P_CSIHOST1: c_int = 49;
pub const SRST_P_CSIHOST2: c_int = 50;
pub const SRST_P_CSIHOST3: c_int = 51;
pub const SRST_P_CSIPHY0: c_int = 52;
pub const SRST_P_CSIPHY1: c_int = 53;
// Name=SOFTRST_CON13,Offset=0x434
pub const SRST_A_VO_BIU: c_int = 54;
pub const SRST_H_VO_BIU: c_int = 55;
pub const SRST_A_VOP: c_int = 56;
pub const SRST_H_VOP: c_int = 57;
pub const SRST_D_VOP: c_int = 58;
pub const SRST_D_VOP1: c_int = 59;
// Name=SOFTRST_CON14,Offset=0x438
pub const SRST_A_RGA_BIU: c_int = 60;
pub const SRST_H_RGA_BIU: c_int = 61;
pub const SRST_A_RGA: c_int = 62;
pub const SRST_H_RGA: c_int = 63;
pub const SRST_RGA_CORE: c_int = 64;
pub const SRST_A_JDEC: c_int = 65;
pub const SRST_H_JDEC: c_int = 66;
// Name=SOFTRST_CON15,Offset=0x43C
pub const SRST_B_EBK_BIU: c_int = 67;
pub const SRST_P_EBK_BIU: c_int = 68;
pub const SRST_AHB2AXI_EBC: c_int = 69;
pub const SRST_H_EBC: c_int = 70;
pub const SRST_D_EBC: c_int = 71;
pub const SRST_H_EINK: c_int = 72;
pub const SRST_P_EINK: c_int = 73;
// Name=SOFTRST_CON16,Offset=0x440
pub const SRST_P_PHP_BIU: c_int = 74;
pub const SRST_A_PHP_BIU: c_int = 75;
pub const SRST_P_PCIE20: c_int = 76;
pub const SRST_PCIE20_POWERUP: c_int = 77;
pub const SRST_USB3OTG: c_int = 78;
// Name=SOFTRST_CON17,Offset=0x444
pub const SRST_PIPEPHY: c_int = 79;
// Name=SOFTRST_CON18,Offset=0x448
pub const SRST_A_BUS_BIU: c_int = 80;
pub const SRST_H_BUS_BIU: c_int = 81;
pub const SRST_P_BUS_BIU: c_int = 82;
// Name=SOFTRST_CON19,Offset=0x44C
pub const SRST_P_I2C1: c_int = 83;
pub const SRST_P_I2C2: c_int = 84;
pub const SRST_P_I2C3: c_int = 85;
pub const SRST_P_I2C4: c_int = 86;
pub const SRST_P_I2C5: c_int = 87;
pub const SRST_I2C1: c_int = 88;
pub const SRST_I2C2: c_int = 89;
pub const SRST_I2C3: c_int = 90;
pub const SRST_I2C4: c_int = 91;
pub const SRST_I2C5: c_int = 92;
// Name=SOFTRST_CON20,Offset=0x450
pub const SRST_BUS_GPIO3: c_int = 93;
pub const SRST_BUS_GPIO4: c_int = 94;
// Name=SOFTRST_CON21,Offset=0x454
pub const SRST_P_TIMER: c_int = 95;
pub const SRST_TIMER0: c_int = 96;
pub const SRST_TIMER1: c_int = 97;
pub const SRST_TIMER2: c_int = 98;
pub const SRST_TIMER3: c_int = 99;
pub const SRST_TIMER4: c_int = 100;
pub const SRST_TIMER5: c_int = 101;
pub const SRST_P_STIMER: c_int = 102;
pub const SRST_STIMER0: c_int = 103;
pub const SRST_STIMER1: c_int = 104;
// Name=SOFTRST_CON22,Offset=0x458
pub const SRST_P_WDTNS: c_int = 105;
pub const SRST_WDTNS: c_int = 106;
pub const SRST_P_GRF: c_int = 107;
pub const SRST_P_SGRF: c_int = 108;
pub const SRST_P_MAILBOX: c_int = 109;
pub const SRST_P_INTC: c_int = 110;
pub const SRST_A_BUS_GIC400: c_int = 111;
pub const SRST_A_BUS_GIC400_DEBUG: c_int = 112;
// Name=SOFTRST_CON23,Offset=0x45C
pub const SRST_A_BUS_SPINLOCK: c_int = 113;
pub const SRST_A_DCF: c_int = 114;
pub const SRST_P_DCF: c_int = 115;
pub const SRST_F_BUS_CM0_CORE: c_int = 116;
pub const SRST_T_BUS_CM0_JTAG: c_int = 117;
pub const SRST_H_ICACHE: c_int = 118;
pub const SRST_H_DCACHE: c_int = 119;
// Name=SOFTRST_CON24,Offset=0x460
pub const SRST_P_TSADC: c_int = 120;
pub const SRST_TSADC: c_int = 121;
pub const SRST_TSADCPHY: c_int = 122;
pub const SRST_P_DFT2APB: c_int = 123;
// Name=SOFTRST_CON25,Offset=0x464
pub const SRST_A_GMAC: c_int = 124;
pub const SRST_P_APB2ASB_VCCIO156: c_int = 125;
pub const SRST_P_DSIPHY: c_int = 126;
pub const SRST_P_DSITX: c_int = 127;
pub const SRST_P_CPU_EMA_DET: c_int = 128;
pub const SRST_P_HASH: c_int = 129;
pub const SRST_P_TOPCRU: c_int = 130;
// Name=SOFTRST_CON26,Offset=0x468
pub const SRST_P_ASB2APB_VCCIO156: c_int = 131;
pub const SRST_P_IOC_VCCIO156: c_int = 132;
pub const SRST_P_GPIO3_VCCIO156: c_int = 133;
pub const SRST_P_GPIO4_VCCIO156: c_int = 134;
pub const SRST_P_SARADC_VCCIO156: c_int = 135;
pub const SRST_SARADC_VCCIO156: c_int = 136;
pub const SRST_SARADC_VCCIO156_PHY: c_int = 137;
// Name=SOFTRST_CON27,Offset=0x46c
pub const SRST_A_MAC100: c_int = 138;
// Name=PMU0SOFTRST_CON00,Offset=0x10200
pub const SRST_P_PMU0_CRU: c_int = 139;
pub const SRST_P_PMU0_PMU: c_int = 140;
pub const SRST_PMU0_PMU: c_int = 141;
pub const SRST_P_PMU0_HP_TIMER: c_int = 142;
pub const SRST_PMU0_HP_TIMER: c_int = 143;
pub const SRST_PMU0_32K_HP_TIMER: c_int = 144;
pub const SRST_P_PMU0_PVTM: c_int = 145;
pub const SRST_PMU0_PVTM: c_int = 146;
pub const SRST_P_IOC_PMUIO: c_int = 147;
pub const SRST_P_PMU0_GPIO0: c_int = 148;
pub const SRST_PMU0_GPIO0: c_int = 149;
pub const SRST_P_PMU0_GRF: c_int = 150;
pub const SRST_P_PMU0_SGRF: c_int = 151;
// Name=PMU0SOFTRST_CON01,Offset=0x10204
pub const SRST_DDR_FAIL_SAFE: c_int = 152;
pub const SRST_P_PMU0_SCRKEYGEN: c_int = 153;
// Name=PMU0SOFTRST_CON02,Offset=0x10208
pub const SRST_P_PMU0_I2C0: c_int = 154;
pub const SRST_PMU0_I2C0: c_int = 155;
// Name=PMU1SOFTRST_CON00,Offset=0x18200
pub const SRST_P_PMU1_CRU: c_int = 156;
pub const SRST_H_PMU1_MEM: c_int = 157;
pub const SRST_H_PMU1_BIU: c_int = 158;
pub const SRST_P_PMU1_BIU: c_int = 159;
pub const SRST_P_PMU1_UART0: c_int = 160;
pub const SRST_S_PMU1_UART0: c_int = 161;
// Name=PMU1SOFTRST_CON01,Offset=0x18204
pub const SRST_P_PMU1_SPI0: c_int = 162;
pub const SRST_PMU1_SPI0: c_int = 163;
pub const SRST_P_PMU1_PWM0: c_int = 164;
pub const SRST_PMU1_PWM0: c_int = 165;
// Name=PMU1SOFTRST_CON02,Offset=0x18208
pub const SRST_F_PMU1_CM0_CORE: c_int = 166;
pub const SRST_T_PMU1_CM0_JTAG: c_int = 167;
pub const SRST_P_PMU1_WDTNS: c_int = 168;
pub const SRST_PMU1_WDTNS: c_int = 169;
pub const SRST_PMU1_MAILBOX: c_int = 170;
// Name=DDRSOFTRST_CON00,Offset=0x20200
pub const SRST_MSCH_BRG_BIU: c_int = 171;
pub const SRST_P_MSCH_BIU: c_int = 172;
pub const SRST_P_DDR_HWLP: c_int = 173;
pub const SRST_P_DDR_PHY: c_int = 290;
pub const SRST_P_DDR_DFICTL: c_int = 174;
pub const SRST_P_DDR_DMA2DDR: c_int = 175;
// Name=DDRSOFTRST_CON01,Offset=0x20204
pub const SRST_P_DDR_MON: c_int = 176;
pub const SRST_TM_DDR_MON: c_int = 177;
pub const SRST_P_DDR_GRF: c_int = 178;
pub const SRST_P_DDR_CRU: c_int = 179;
pub const SRST_P_SUBDDR_CRU: c_int = 180;
// Name=SUBDDRSOFTRST_CON00,Offset=0x28200
pub const SRST_MSCH_BIU: c_int = 181;
pub const SRST_DDR_PHY: c_int = 182;
pub const SRST_DDR_DFICTL: c_int = 183;
pub const SRST_DDR_SCRAMBLE: c_int = 184;
pub const SRST_DDR_MON: c_int = 185;
pub const SRST_A_DDR_SPLIT: c_int = 186;
pub const SRST_DDR_DMA2DDR: c_int = 187;
// Name=PERISOFTRST_CON01,Offset=0x30404
pub const SRST_A_PERI_BIU: c_int = 188;
pub const SRST_H_PERI_BIU: c_int = 189;
pub const SRST_P_PERI_BIU: c_int = 190;
pub const SRST_P_PERICRU: c_int = 191;
// Name=PERISOFTRST_CON02,Offset=0x30408
pub const SRST_H_SAI0_8CH: c_int = 192;
pub const SRST_M_SAI0_8CH: c_int = 193;
pub const SRST_H_SAI1_8CH: c_int = 194;
pub const SRST_M_SAI1_8CH: c_int = 195;
pub const SRST_H_SAI2_2CH: c_int = 196;
pub const SRST_M_SAI2_2CH: c_int = 197;
// Name=PERISOFTRST_CON03,Offset=0x3040C
pub const SRST_H_DSM: c_int = 198;
pub const SRST_DSM: c_int = 199;
pub const SRST_H_PDM: c_int = 200;
pub const SRST_M_PDM: c_int = 201;
pub const SRST_H_SPDIF: c_int = 202;
pub const SRST_M_SPDIF: c_int = 203;
// Name=PERISOFTRST_CON04,Offset=0x30410
pub const SRST_H_SDMMC0: c_int = 204;
pub const SRST_H_SDMMC1: c_int = 205;
pub const SRST_H_EMMC: c_int = 206;
pub const SRST_A_EMMC: c_int = 207;
pub const SRST_C_EMMC: c_int = 208;
pub const SRST_B_EMMC: c_int = 209;
pub const SRST_T_EMMC: c_int = 210;
pub const SRST_S_SFC: c_int = 211;
pub const SRST_H_SFC: c_int = 212;
// Name=PERISOFTRST_CON05,Offset=0x30414
pub const SRST_H_USB2HOST: c_int = 213;
pub const SRST_H_USB2HOST_ARB: c_int = 214;
pub const SRST_USB2HOST_UTMI: c_int = 215;
// Name=PERISOFTRST_CON06,Offset=0x30418
pub const SRST_P_SPI1: c_int = 216;
pub const SRST_SPI1: c_int = 217;
pub const SRST_P_SPI2: c_int = 218;
pub const SRST_SPI2: c_int = 219;
// Name=PERISOFTRST_CON07,Offset=0x3041C
pub const SRST_P_UART1: c_int = 220;
pub const SRST_P_UART2: c_int = 221;
pub const SRST_P_UART3: c_int = 222;
pub const SRST_P_UART4: c_int = 223;
pub const SRST_P_UART5: c_int = 224;
pub const SRST_P_UART6: c_int = 225;
pub const SRST_P_UART7: c_int = 226;
pub const SRST_P_UART8: c_int = 227;
pub const SRST_P_UART9: c_int = 228;
pub const SRST_S_UART1: c_int = 229;
pub const SRST_S_UART2: c_int = 230;
// Name=PERISOFTRST_CON08,Offset=0x30420
pub const SRST_S_UART3: c_int = 231;
pub const SRST_S_UART4: c_int = 232;
pub const SRST_S_UART5: c_int = 233;
pub const SRST_S_UART6: c_int = 234;
pub const SRST_S_UART7: c_int = 235;
// Name=PERISOFTRST_CON09,Offset=0x30424
pub const SRST_S_UART8: c_int = 236;
pub const SRST_S_UART9: c_int = 237;
// Name=PERISOFTRST_CON10,Offset=0x30428
pub const SRST_P_PWM1_PERI: c_int = 238;
pub const SRST_PWM1_PERI: c_int = 239;
pub const SRST_P_PWM2_PERI: c_int = 240;
pub const SRST_PWM2_PERI: c_int = 241;
pub const SRST_P_PWM3_PERI: c_int = 242;
pub const SRST_PWM3_PERI: c_int = 243;
// Name=PERISOFTRST_CON11,Offset=0x3042C
pub const SRST_P_CAN0: c_int = 244;
pub const SRST_CAN0: c_int = 245;
pub const SRST_P_CAN1: c_int = 246;
pub const SRST_CAN1: c_int = 247;
// Name=PERISOFTRST_CON12,Offset=0x30430
pub const SRST_A_CRYPTO: c_int = 248;
pub const SRST_H_CRYPTO: c_int = 249;
pub const SRST_P_CRYPTO: c_int = 250;
pub const SRST_CORE_CRYPTO: c_int = 251;
pub const SRST_PKA_CRYPTO: c_int = 252;
pub const SRST_H_KLAD: c_int = 253;
pub const SRST_P_KEY_READER: c_int = 254;
pub const SRST_H_RK_RNG_NS: c_int = 255;
pub const SRST_H_RK_RNG_S: c_int = 256;
pub const SRST_H_TRNG_NS: c_int = 257;
pub const SRST_H_TRNG_S: c_int = 258;
pub const SRST_H_CRYPTO_S: c_int = 259;
// Name=PERISOFTRST_CON13,Offset=0x30434
pub const SRST_P_PERI_WDT: c_int = 260;
pub const SRST_T_PERI_WDT: c_int = 261;
pub const SRST_A_SYSMEM: c_int = 262;
pub const SRST_H_BOOTROM: c_int = 263;
pub const SRST_P_PERI_GRF: c_int = 264;
pub const SRST_A_DMAC: c_int = 265;
pub const SRST_A_RKDMAC: c_int = 267;
// Name=PERISOFTRST_CON14,Offset=0x30438
pub const SRST_P_OTPC_NS: c_int = 268;
pub const SRST_SBPI_OTPC_NS: c_int = 269;
pub const SRST_USER_OTPC_NS: c_int = 270;
pub const SRST_P_OTPC_S: c_int = 271;
pub const SRST_SBPI_OTPC_S: c_int = 272;
pub const SRST_USER_OTPC_S: c_int = 273;
pub const SRST_OTPC_ARB: c_int = 274;
pub const SRST_P_OTPPHY: c_int = 275;
pub const SRST_OTP_NPOR: c_int = 276;
// Name=PERISOFTRST_CON15,Offset=0x3043C
pub const SRST_P_USB2PHY: c_int = 277;
pub const SRST_USB2PHY_POR: c_int = 278;
pub const SRST_USB2PHY_OTG: c_int = 279;
pub const SRST_USB2PHY_HOST: c_int = 280;
pub const SRST_P_PIPEPHY: c_int = 281;
// Name=PERISOFTRST_CON16,Offset=0x30440
pub const SRST_P_SARADC: c_int = 282;
pub const SRST_SARADC: c_int = 283;
pub const SRST_SARADC_PHY: c_int = 284;
pub const SRST_P_IOC_VCCIO234: c_int = 285;
// Name=PERISOFTRST_CON17,Offset=0x30444
pub const SRST_P_PERI_GPIO1: c_int = 286;
pub const SRST_P_PERI_GPIO2: c_int = 287;
pub const SRST_PERI_GPIO1: c_int = 288;
pub const SRST_PERI_GPIO2: c_int = 289;
