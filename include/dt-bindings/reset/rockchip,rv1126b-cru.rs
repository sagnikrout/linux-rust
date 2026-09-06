//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/rockchip,rv1126b-cru.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (c) 2025 Rockchip Electronics Co., Ltd.
// Author: Elaine Zhang <zhangqing@rock-chips.com>
//
// ==========================list all of reset fields id===========================
// TOPCRU-->SOFTRST_CON00
// TOPCRU-->SOFTRST_CON15
pub const SRST_P_CRU: c_int = 0;
pub const SRST_P_CRU_BIU: c_int = 1;
// BUSCRU-->SOFTRST_CON00
pub const SRST_A_TOP_BIU: c_int = 2;
pub const SRST_A_RKCE_BIU: c_int = 3;
pub const SRST_A_BUS_BIU: c_int = 4;
pub const SRST_H_BUS_BIU: c_int = 5;
pub const SRST_P_BUS_BIU: c_int = 6;
pub const SRST_P_CRU_BUS: c_int = 7;
pub const SRST_P_SYS_GRF: c_int = 8;
pub const SRST_H_BOOTROM: c_int = 9;
pub const SRST_A_GIC400: c_int = 10;
pub const SRST_A_SPINLOCK: c_int = 11;
pub const SRST_P_WDT_NS: c_int = 12;
pub const SRST_T_WDT_NS: c_int = 13;
// BUSCRU-->SOFTRST_CON01
pub const SRST_P_WDT_HPMCU: c_int = 14;
pub const SRST_T_WDT_HPMCU: c_int = 15;
pub const SRST_H_CACHE: c_int = 16;
pub const SRST_P_HPMCU_MAILBOX: c_int = 17;
pub const SRST_P_HPMCU_INTMUX: c_int = 18;
pub const SRST_HPMCU_FULL_CLUSTER: c_int = 19;
pub const SRST_HPMCU_PWUP: c_int = 20;
pub const SRST_HPMCU_ONLY_CORE: c_int = 21;
pub const SRST_T_HPMCU_JTAG: c_int = 22;
pub const SRST_P_RKDMA: c_int = 23;
pub const SRST_A_RKDMA: c_int = 24;
// BUSCRU-->SOFTRST_CON02
pub const SRST_P_DCF: c_int = 25;
pub const SRST_A_DCF: c_int = 26;
pub const SRST_H_RGA: c_int = 27;
pub const SRST_A_RGA: c_int = 28;
pub const SRST_CORE_RGA: c_int = 29;
pub const SRST_P_TIMER: c_int = 30;
pub const SRST_TIMER0: c_int = 31;
pub const SRST_TIMER1: c_int = 32;
pub const SRST_TIMER2: c_int = 33;
pub const SRST_TIMER3: c_int = 34;
pub const SRST_TIMER4: c_int = 35;
pub const SRST_TIMER5: c_int = 36;
pub const SRST_A_RKCE: c_int = 37;
pub const SRST_PKA_RKCE: c_int = 38;
pub const SRST_H_RKRNG_S: c_int = 39;
pub const SRST_H_RKRNG_NS: c_int = 40;
// BUSCRU-->SOFTRST_CON03
pub const SRST_P_I2C0: c_int = 41;
pub const SRST_I2C0: c_int = 42;
pub const SRST_P_I2C1: c_int = 43;
pub const SRST_I2C1: c_int = 44;
pub const SRST_P_I2C3: c_int = 45;
pub const SRST_I2C3: c_int = 46;
pub const SRST_P_I2C4: c_int = 47;
pub const SRST_I2C4: c_int = 48;
pub const SRST_P_I2C5: c_int = 49;
pub const SRST_I2C5: c_int = 50;
pub const SRST_P_SPI0: c_int = 51;
pub const SRST_SPI0: c_int = 52;
pub const SRST_P_SPI1: c_int = 53;
pub const SRST_SPI1: c_int = 54;
// BUSCRU-->SOFTRST_CON04
pub const SRST_P_PWM0: c_int = 55;
pub const SRST_PWM0: c_int = 56;
pub const SRST_P_PWM2: c_int = 57;
pub const SRST_PWM2: c_int = 58;
pub const SRST_P_PWM3: c_int = 59;
pub const SRST_PWM3: c_int = 60;
// BUSCRU-->SOFTRST_CON05
pub const SRST_P_UART1: c_int = 61;
pub const SRST_S_UART1: c_int = 62;
pub const SRST_P_UART2: c_int = 63;
pub const SRST_S_UART2: c_int = 64;
pub const SRST_P_UART3: c_int = 65;
pub const SRST_S_UART3: c_int = 66;
pub const SRST_P_UART4: c_int = 67;
pub const SRST_S_UART4: c_int = 68;
pub const SRST_P_UART5: c_int = 69;
pub const SRST_S_UART5: c_int = 70;
pub const SRST_P_UART6: c_int = 71;
pub const SRST_S_UART6: c_int = 72;
pub const SRST_P_UART7: c_int = 73;
pub const SRST_S_UART7: c_int = 74;
// BUSCRU-->SOFTRST_CON06
pub const SRST_P_TSADC: c_int = 75;
pub const SRST_TSADC: c_int = 76;
pub const SRST_H_SAI0: c_int = 77;
pub const SRST_M_SAI0: c_int = 78;
pub const SRST_H_SAI1: c_int = 79;
pub const SRST_M_SAI1: c_int = 80;
pub const SRST_H_SAI2: c_int = 81;
pub const SRST_M_SAI2: c_int = 82;
pub const SRST_H_RKDSM: c_int = 83;
pub const SRST_M_RKDSM: c_int = 84;
pub const SRST_H_PDM: c_int = 85;
pub const SRST_M_PDM: c_int = 86;
pub const SRST_PDM: c_int = 87;
// BUSCRU-->SOFTRST_CON07
pub const SRST_H_ASRC0: c_int = 88;
pub const SRST_ASRC0: c_int = 89;
pub const SRST_H_ASRC1: c_int = 90;
pub const SRST_ASRC1: c_int = 91;
pub const SRST_P_AUDIO_ADC_BUS: c_int = 92;
pub const SRST_M_AUDIO_ADC_BUS: c_int = 93;
pub const SRST_P_RKCE: c_int = 94;
pub const SRST_H_NS_RKCE: c_int = 95;
pub const SRST_P_OTPC_NS: c_int = 96;
pub const SRST_SBPI_OTPC_NS: c_int = 97;
pub const SRST_USER_OTPC_NS: c_int = 98;
pub const SRST_OTPC_ARB: c_int = 99;
pub const SRST_P_OTP_MASK: c_int = 100;
// PERICRU-->SOFTRST_CON00
pub const SRST_A_PERI_BIU: c_int = 101;
pub const SRST_P_PERI_BIU: c_int = 102;
pub const SRST_P_RTC_BIU: c_int = 103;
pub const SRST_P_CRU_PERI: c_int = 104;
pub const SRST_P_PERI_GRF: c_int = 105;
pub const SRST_P_GPIO1: c_int = 106;
pub const SRST_DB_GPIO1: c_int = 107;
pub const SRST_P_IOC_VCCIO1: c_int = 108;
pub const SRST_A_USB3OTG: c_int = 109;
pub const SRST_H_USB2HOST: c_int = 110;
pub const SRST_H_ARB_USB2HOST: c_int = 111;
pub const SRST_P_RTC_TEST: c_int = 112;
// PERICRU-->SOFTRST_CON01
pub const SRST_H_EMMC: c_int = 113;
pub const SRST_H_FSPI0: c_int = 114;
pub const SRST_H_XIP_FSPI0: c_int = 115;
pub const SRST_S_2X_FSPI0: c_int = 116;
pub const SRST_UTMI_USB2HOST: c_int = 117;
pub const SRST_REF_PIPEPHY: c_int = 118;
pub const SRST_P_PIPEPHY: c_int = 119;
pub const SRST_P_PIPEPHY_GRF: c_int = 120;
pub const SRST_P_USB2PHY: c_int = 121;
pub const SRST_POR_USB2PHY: c_int = 122;
pub const SRST_OTG_USB2PHY: c_int = 123;
pub const SRST_HOST_USB2PHY: c_int = 124;
// CORECRU-->SOFTRST_CON00
pub const SRST_REF_PVTPLL_CORE: c_int = 125;
pub const SRST_NCOREPORESET0: c_int = 126;
pub const SRST_NCORESET0: c_int = 127;
pub const SRST_NCOREPORESET1: c_int = 128;
pub const SRST_NCORESET1: c_int = 129;
pub const SRST_NCOREPORESET2: c_int = 130;
pub const SRST_NCORESET2: c_int = 131;
pub const SRST_NCOREPORESET3: c_int = 132;
pub const SRST_NCORESET3: c_int = 133;
pub const SRST_NDBGRESET: c_int = 134;
pub const SRST_NL2RESET: c_int = 135;
// CORECRU-->SOFTRST_CON01
pub const SRST_A_CORE_BIU: c_int = 136;
pub const SRST_P_CORE_BIU: c_int = 137;
pub const SRST_H_CORE_BIU: c_int = 138;
pub const SRST_P_DBG: c_int = 139;
pub const SRST_POT_DBG: c_int = 140;
pub const SRST_NT_DBG: c_int = 141;
pub const SRST_P_CORE_PVTPLL: c_int = 142;
pub const SRST_P_CRU_CORE: c_int = 143;
pub const SRST_P_CORE_GRF: c_int = 144;
pub const SRST_P_DFT2APB: c_int = 145;
// PMUCRU-->SOFTRST_CON00
pub const SRST_H_PMU_BIU: c_int = 146;
pub const SRST_P_PMU_GPIO0: c_int = 147;
pub const SRST_DB_PMU_GPIO0: c_int = 148;
pub const SRST_P_PMU_HP_TIMER: c_int = 149;
pub const SRST_PMU_HP_TIMER: c_int = 150;
pub const SRST_PMU_32K_HP_TIMER: c_int = 151;
// PMUCRU-->SOFTRST_CON01
pub const SRST_P_PWM1: c_int = 152;
pub const SRST_PWM1: c_int = 153;
pub const SRST_P_I2C2: c_int = 154;
pub const SRST_I2C2: c_int = 155;
pub const SRST_P_UART0: c_int = 156;
pub const SRST_S_UART0: c_int = 157;
// PMUCRU-->SOFTRST_CON02
pub const SRST_P_RCOSC_CTRL: c_int = 158;
pub const SRST_REF_RCOSC_CTRL: c_int = 159;
pub const SRST_P_IOC_PMUIO0: c_int = 160;
pub const SRST_P_CRU_PMU: c_int = 161;
pub const SRST_P_PMU_GRF: c_int = 162;
pub const SRST_PREROLL: c_int = 163;
pub const SRST_PREROLL_32K: c_int = 164;
pub const SRST_H_PMU_SRAM: c_int = 165;
// PMUCRU-->SOFTRST_CON03
pub const SRST_P_WDT_LPMCU: c_int = 166;
pub const SRST_T_WDT_LPMCU: c_int = 167;
pub const SRST_LPMCU_FULL_CLUSTER: c_int = 168;
pub const SRST_LPMCU_PWUP: c_int = 169;
pub const SRST_LPMCU_ONLY_CORE: c_int = 170;
pub const SRST_T_LPMCU_JTAG: c_int = 171;
pub const SRST_P_LPMCU_MAILBOX: c_int = 172;
// PMU1CRU-->SOFTRST_CON00
pub const SRST_P_SPI2AHB: c_int = 173;
pub const SRST_H_SPI2AHB: c_int = 174;
pub const SRST_H_FSPI1: c_int = 175;
pub const SRST_H_XIP_FSPI1: c_int = 176;
pub const SRST_S_1X_FSPI1: c_int = 177;
pub const SRST_P_IOC_PMUIO1: c_int = 178;
pub const SRST_P_CRU_PMU1: c_int = 179;
pub const SRST_P_AUDIO_ADC_PMU: c_int = 180;
pub const SRST_M_AUDIO_ADC_PMU: c_int = 181;
pub const SRST_H_PMU1_BIU: c_int = 182;
// PMU1CRU-->SOFTRST_CON01
pub const SRST_P_LPDMA: c_int = 183;
pub const SRST_A_LPDMA: c_int = 184;
pub const SRST_H_LPSAI: c_int = 185;
pub const SRST_M_LPSAI: c_int = 186;
pub const SRST_P_AOA_TDD: c_int = 187;
pub const SRST_P_AOA_FE: c_int = 188;
pub const SRST_P_AOA_AAD: c_int = 189;
pub const SRST_P_AOA_APB: c_int = 190;
pub const SRST_P_AOA_SRAM: c_int = 191;
// DDRCRU-->SOFTRST_CON00
pub const SRST_P_DDR_BIU: c_int = 192;
pub const SRST_P_DDRC: c_int = 193;
pub const SRST_P_DDRMON: c_int = 194;
pub const SRST_TIMER_DDRMON: c_int = 195;
pub const SRST_P_DFICTRL: c_int = 196;
pub const SRST_P_DDR_GRF: c_int = 197;
pub const SRST_P_CRU_DDR: c_int = 198;
pub const SRST_P_DDRPHY: c_int = 199;
pub const SRST_P_DMA2DDR: c_int = 200;
// SUBDDRCRU-->SOFTRST_CON00
pub const SRST_A_SYSMEM_BIU: c_int = 201;
pub const SRST_A_SYSMEM: c_int = 202;
pub const SRST_A_DDR_BIU: c_int = 203;
pub const SRST_A_DDRSCH0_CPU: c_int = 204;
pub const SRST_A_DDRSCH1_NPU: c_int = 205;
pub const SRST_A_DDRSCH2_POE: c_int = 206;
pub const SRST_A_DDRSCH3_VI: c_int = 207;
pub const SRST_CORE_DDRC: c_int = 208;
pub const SRST_DDRMON: c_int = 209;
pub const SRST_DFICTRL: c_int = 210;
pub const SRST_RS: c_int = 211;
pub const SRST_A_DMA2DDR: c_int = 212;
pub const SRST_DDRPHY: c_int = 213;
// VICRU-->SOFTRST_CON00
pub const SRST_REF_PVTPLL_ISP: c_int = 214;
pub const SRST_A_GMAC_BIU: c_int = 215;
pub const SRST_A_VI_BIU: c_int = 216;
pub const SRST_H_VI_BIU: c_int = 217;
pub const SRST_P_VI_BIU: c_int = 218;
pub const SRST_P_CRU_VI: c_int = 219;
pub const SRST_P_VI_GRF: c_int = 220;
pub const SRST_P_VI_PVTPLL: c_int = 221;
pub const SRST_P_DSMC: c_int = 222;
pub const SRST_A_DSMC: c_int = 223;
pub const SRST_H_CAN0: c_int = 224;
pub const SRST_CAN0: c_int = 225;
pub const SRST_H_CAN1: c_int = 226;
pub const SRST_CAN1: c_int = 227;
// VICRU-->SOFTRST_CON01
pub const SRST_P_GPIO2: c_int = 228;
pub const SRST_DB_GPIO2: c_int = 229;
pub const SRST_P_GPIO4: c_int = 230;
pub const SRST_DB_GPIO4: c_int = 231;
pub const SRST_P_GPIO5: c_int = 232;
pub const SRST_DB_GPIO5: c_int = 233;
pub const SRST_P_GPIO6: c_int = 234;
pub const SRST_DB_GPIO6: c_int = 235;
pub const SRST_P_GPIO7: c_int = 236;
pub const SRST_DB_GPIO7: c_int = 237;
pub const SRST_P_IOC_VCCIO2: c_int = 238;
pub const SRST_P_IOC_VCCIO4: c_int = 239;
pub const SRST_P_IOC_VCCIO5: c_int = 240;
pub const SRST_P_IOC_VCCIO6: c_int = 241;
pub const SRST_P_IOC_VCCIO7: c_int = 242;
// VICRU-->SOFTRST_CON02
pub const SRST_CORE_ISP: c_int = 243;
pub const SRST_H_VICAP: c_int = 244;
pub const SRST_A_VICAP: c_int = 245;
pub const SRST_D_VICAP: c_int = 246;
pub const SRST_ISP0_VICAP: c_int = 247;
pub const SRST_CORE_VPSS: c_int = 248;
pub const SRST_CORE_VPSL: c_int = 249;
pub const SRST_P_CSI2HOST0: c_int = 250;
pub const SRST_P_CSI2HOST1: c_int = 251;
pub const SRST_P_CSI2HOST2: c_int = 252;
pub const SRST_P_CSI2HOST3: c_int = 253;
pub const SRST_H_SDMMC0: c_int = 254;
pub const SRST_A_GMAC: c_int = 255;
pub const SRST_P_CSIPHY0: c_int = 256;
pub const SRST_P_CSIPHY1: c_int = 257;
// VICRU-->SOFTRST_CON03
pub const SRST_P_MACPHY: c_int = 258;
pub const SRST_MACPHY: c_int = 259;
pub const SRST_P_SARADC1: c_int = 260;
pub const SRST_SARADC1: c_int = 261;
pub const SRST_P_SARADC2: c_int = 262;
pub const SRST_SARADC2: c_int = 263;
// VEPUCRU-->SOFTRST_CON00
pub const SRST_REF_PVTPLL_VEPU: c_int = 264;
pub const SRST_A_VEPU_BIU: c_int = 265;
pub const SRST_H_VEPU_BIU: c_int = 266;
pub const SRST_P_VEPU_BIU: c_int = 267;
pub const SRST_P_CRU_VEPU: c_int = 268;
pub const SRST_P_VEPU_GRF: c_int = 269;
pub const SRST_P_GPIO3: c_int = 270;
pub const SRST_DB_GPIO3: c_int = 271;
pub const SRST_P_IOC_VCCIO3: c_int = 272;
pub const SRST_P_SARADC0: c_int = 273;
pub const SRST_SARADC0: c_int = 274;
pub const SRST_H_SDMMC1: c_int = 275;
// VEPUCRU-->SOFTRST_CON01
pub const SRST_P_VEPU_PVTPLL: c_int = 276;
pub const SRST_H_VEPU: c_int = 277;
pub const SRST_A_VEPU: c_int = 278;
pub const SRST_CORE_VEPU: c_int = 279;
// NPUCRU-->SOFTRST_CON00
pub const SRST_REF_PVTPLL_NPU: c_int = 280;
pub const SRST_A_NPU_BIU: c_int = 281;
pub const SRST_H_NPU_BIU: c_int = 282;
pub const SRST_P_NPU_BIU: c_int = 283;
pub const SRST_P_CRU_NPU: c_int = 284;
pub const SRST_P_NPU_GRF: c_int = 285;
pub const SRST_P_NPU_PVTPLL: c_int = 286;
pub const SRST_H_RKNN: c_int = 287;
pub const SRST_A_RKNN: c_int = 288;
// VDOCRU-->SOFTRST_CON00
pub const SRST_A_RKVDEC_BIU: c_int = 289;
pub const SRST_A_VDO_BIU: c_int = 290;
pub const SRST_H_VDO_BIU: c_int = 291;
pub const SRST_P_VDO_BIU: c_int = 292;
pub const SRST_P_CRU_VDO: c_int = 293;
pub const SRST_P_VDO_GRF: c_int = 294;
pub const SRST_A_RKVDEC: c_int = 295;
pub const SRST_H_RKVDEC: c_int = 296;
pub const SRST_HEVC_CA_RKVDEC: c_int = 297;
pub const SRST_A_VOP: c_int = 298;
pub const SRST_H_VOP: c_int = 299;
pub const SRST_D_VOP: c_int = 300;
pub const SRST_A_OOC: c_int = 301;
pub const SRST_H_OOC: c_int = 302;
pub const SRST_D_OOC: c_int = 303;
// VDOCRU-->SOFTRST_CON01
pub const SRST_H_RKJPEG: c_int = 304;
pub const SRST_A_RKJPEG: c_int = 305;
pub const SRST_A_RKMMU_DECOM: c_int = 306;
pub const SRST_H_RKMMU_DECOM: c_int = 307;
pub const SRST_D_DECOM: c_int = 308;
pub const SRST_A_DECOM: c_int = 309;
pub const SRST_P_DECOM: c_int = 310;
pub const SRST_P_MIPI_DSI: c_int = 311;
pub const SRST_P_DSIPHY: c_int = 312;
// VCPCRU-->SOFTRST_CON00
pub const SRST_REF_PVTPLL_VCP: c_int = 313;
pub const SRST_A_VCP_BIU: c_int = 314;
pub const SRST_H_VCP_BIU: c_int = 315;
pub const SRST_P_VCP_BIU: c_int = 316;
pub const SRST_P_CRU_VCP: c_int = 317;
pub const SRST_P_VCP_GRF: c_int = 318;
pub const SRST_P_VCP_PVTPLL: c_int = 319;
pub const SRST_A_AISP_BIU: c_int = 320;
pub const SRST_H_AISP_BIU: c_int = 321;
pub const SRST_CORE_AISP: c_int = 322;
// VCPCRU-->SOFTRST_CON01
pub const SRST_H_FEC: c_int = 323;
pub const SRST_A_FEC: c_int = 324;
pub const SRST_CORE_FEC: c_int = 325;
pub const SRST_H_AVSP: c_int = 326;
pub const SRST_A_AVSP: c_int = 327;
