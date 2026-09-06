//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mediatek,mt8365-clk.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// Copyright (c) 2022 MediaTek Inc.
//
// TOPCKGEN
pub const CLK_TOP_CLK_NULL: c_int = 0;
pub const CLK_TOP_I2S0_BCK: c_int = 1;
pub const CLK_TOP_DSI0_LNTC_DSICK: c_int = 2;
pub const CLK_TOP_VPLL_DPIX: c_int = 3;
pub const CLK_TOP_LVDSTX_CLKDIG_CTS: c_int = 4;
pub const CLK_TOP_MFGPLL: c_int = 5;
pub const CLK_TOP_SYSPLL_D2: c_int = 6;
pub const CLK_TOP_SYSPLL1_D2: c_int = 7;
pub const CLK_TOP_SYSPLL1_D4: c_int = 8;
pub const CLK_TOP_SYSPLL1_D8: c_int = 9;
pub const CLK_TOP_SYSPLL1_D16: c_int = 10;
pub const CLK_TOP_SYSPLL_D3: c_int = 11;
pub const CLK_TOP_SYSPLL2_D2: c_int = 12;
pub const CLK_TOP_SYSPLL2_D4: c_int = 13;
pub const CLK_TOP_SYSPLL2_D8: c_int = 14;
pub const CLK_TOP_SYSPLL_D5: c_int = 15;
pub const CLK_TOP_SYSPLL3_D2: c_int = 16;
pub const CLK_TOP_SYSPLL3_D4: c_int = 17;
pub const CLK_TOP_SYSPLL_D7: c_int = 18;
pub const CLK_TOP_SYSPLL4_D2: c_int = 19;
pub const CLK_TOP_SYSPLL4_D4: c_int = 20;
pub const CLK_TOP_UNIVPLL: c_int = 21;
pub const CLK_TOP_UNIVPLL_D2: c_int = 22;
pub const CLK_TOP_UNIVPLL1_D2: c_int = 23;
pub const CLK_TOP_UNIVPLL1_D4: c_int = 24;
pub const CLK_TOP_UNIVPLL_D3: c_int = 25;
pub const CLK_TOP_UNIVPLL2_D2: c_int = 26;
pub const CLK_TOP_UNIVPLL2_D4: c_int = 27;
pub const CLK_TOP_UNIVPLL2_D8: c_int = 28;
pub const CLK_TOP_UNIVPLL2_D32: c_int = 29;
pub const CLK_TOP_UNIVPLL_D5: c_int = 30;
pub const CLK_TOP_UNIVPLL3_D2: c_int = 31;
pub const CLK_TOP_UNIVPLL3_D4: c_int = 32;
pub const CLK_TOP_MMPLL: c_int = 33;
pub const CLK_TOP_MMPLL_D2: c_int = 34;
pub const CLK_TOP_LVDSPLL_D2: c_int = 35;
pub const CLK_TOP_LVDSPLL_D4: c_int = 36;
pub const CLK_TOP_LVDSPLL_D8: c_int = 37;
pub const CLK_TOP_LVDSPLL_D16: c_int = 38;
pub const CLK_TOP_USB20_192M: c_int = 39;
pub const CLK_TOP_USB20_192M_D4: c_int = 40;
pub const CLK_TOP_USB20_192M_D8: c_int = 41;
pub const CLK_TOP_USB20_192M_D16: c_int = 42;
pub const CLK_TOP_USB20_192M_D32: c_int = 43;
pub const CLK_TOP_APLL1: c_int = 44;
pub const CLK_TOP_APLL1_D2: c_int = 45;
pub const CLK_TOP_APLL1_D4: c_int = 46;
pub const CLK_TOP_APLL1_D8: c_int = 47;
pub const CLK_TOP_APLL2: c_int = 48;
pub const CLK_TOP_APLL2_D2: c_int = 49;
pub const CLK_TOP_APLL2_D4: c_int = 50;
pub const CLK_TOP_APLL2_D8: c_int = 51;
pub const CLK_TOP_SYS_26M_D2: c_int = 52;
pub const CLK_TOP_MSDCPLL: c_int = 53;
pub const CLK_TOP_MSDCPLL_D2: c_int = 54;
pub const CLK_TOP_DSPPLL: c_int = 55;
pub const CLK_TOP_DSPPLL_D2: c_int = 56;
pub const CLK_TOP_DSPPLL_D4: c_int = 57;
pub const CLK_TOP_DSPPLL_D8: c_int = 58;
pub const CLK_TOP_APUPLL: c_int = 59;
pub const CLK_TOP_CLK26M_D52: c_int = 60;
pub const CLK_TOP_AXI_SEL: c_int = 61;
pub const CLK_TOP_MEM_SEL: c_int = 62;
pub const CLK_TOP_MM_SEL: c_int = 63;
pub const CLK_TOP_SCP_SEL: c_int = 64;
pub const CLK_TOP_MFG_SEL: c_int = 65;
pub const CLK_TOP_ATB_SEL: c_int = 66;
pub const CLK_TOP_CAMTG_SEL: c_int = 67;
pub const CLK_TOP_CAMTG1_SEL: c_int = 68;
pub const CLK_TOP_UART_SEL: c_int = 69;
pub const CLK_TOP_SPI_SEL: c_int = 70;
pub const CLK_TOP_MSDC50_0_HC_SEL: c_int = 71;
pub const CLK_TOP_MSDC2_2_HC_SEL: c_int = 72;
pub const CLK_TOP_MSDC50_0_SEL: c_int = 73;
pub const CLK_TOP_MSDC50_2_SEL: c_int = 74;
pub const CLK_TOP_MSDC30_1_SEL: c_int = 75;
pub const CLK_TOP_AUDIO_SEL: c_int = 76;
pub const CLK_TOP_AUD_INTBUS_SEL: c_int = 77;
pub const CLK_TOP_AUD_1_SEL: c_int = 78;
pub const CLK_TOP_AUD_2_SEL: c_int = 79;
pub const CLK_TOP_AUD_ENGEN1_SEL: c_int = 80;
pub const CLK_TOP_AUD_ENGEN2_SEL: c_int = 81;
pub const CLK_TOP_AUD_SPDIF_SEL: c_int = 82;
pub const CLK_TOP_DISP_PWM_SEL: c_int = 83;
pub const CLK_TOP_DXCC_SEL: c_int = 84;
pub const CLK_TOP_SSUSB_SYS_SEL: c_int = 85;
pub const CLK_TOP_SSUSB_XHCI_SEL: c_int = 86;
pub const CLK_TOP_SPM_SEL: c_int = 87;
pub const CLK_TOP_I2C_SEL: c_int = 88;
pub const CLK_TOP_PWM_SEL: c_int = 89;
pub const CLK_TOP_SENIF_SEL: c_int = 90;
pub const CLK_TOP_AES_FDE_SEL: c_int = 91;
pub const CLK_TOP_CAMTM_SEL: c_int = 92;
pub const CLK_TOP_DPI0_SEL: c_int = 93;
pub const CLK_TOP_DPI1_SEL: c_int = 94;
pub const CLK_TOP_DSP_SEL: c_int = 95;
pub const CLK_TOP_NFI2X_SEL: c_int = 96;
pub const CLK_TOP_NFIECC_SEL: c_int = 97;
pub const CLK_TOP_ECC_SEL: c_int = 98;
pub const CLK_TOP_ETH_SEL: c_int = 99;
pub const CLK_TOP_GCPU_SEL: c_int = 100;
pub const CLK_TOP_GCPU_CPM_SEL: c_int = 101;
pub const CLK_TOP_APU_SEL: c_int = 102;
pub const CLK_TOP_APU_IF_SEL: c_int = 103;
pub const CLK_TOP_MBIST_DIAG_SEL: c_int = 104;
pub const CLK_TOP_APLL_I2S0_SEL: c_int = 105;
pub const CLK_TOP_APLL_I2S1_SEL: c_int = 106;
pub const CLK_TOP_APLL_I2S2_SEL: c_int = 107;
pub const CLK_TOP_APLL_I2S3_SEL: c_int = 108;
pub const CLK_TOP_APLL_TDMOUT_SEL: c_int = 109;
pub const CLK_TOP_APLL_TDMIN_SEL: c_int = 110;
pub const CLK_TOP_APLL_SPDIF_SEL: c_int = 111;
pub const CLK_TOP_APLL12_CK_DIV0: c_int = 112;
pub const CLK_TOP_APLL12_CK_DIV1: c_int = 113;
pub const CLK_TOP_APLL12_CK_DIV2: c_int = 114;
pub const CLK_TOP_APLL12_CK_DIV3: c_int = 115;
pub const CLK_TOP_APLL12_CK_DIV4: c_int = 116;
pub const CLK_TOP_APLL12_CK_DIV4B: c_int = 117;
pub const CLK_TOP_APLL12_CK_DIV5: c_int = 118;
pub const CLK_TOP_APLL12_CK_DIV5B: c_int = 119;
pub const CLK_TOP_APLL12_CK_DIV6: c_int = 120;
pub const CLK_TOP_AUD_I2S0_M: c_int = 121;
pub const CLK_TOP_AUD_I2S1_M: c_int = 122;
pub const CLK_TOP_AUD_I2S2_M: c_int = 123;
pub const CLK_TOP_AUD_I2S3_M: c_int = 124;
pub const CLK_TOP_AUD_TDMOUT_M: c_int = 125;
pub const CLK_TOP_AUD_TDMOUT_B: c_int = 126;
pub const CLK_TOP_AUD_TDMIN_M: c_int = 127;
pub const CLK_TOP_AUD_TDMIN_B: c_int = 128;
pub const CLK_TOP_AUD_SPDIF_M: c_int = 129;
pub const CLK_TOP_USB20_48M_EN: c_int = 130;
pub const CLK_TOP_UNIVPLL_48M_EN: c_int = 131;
pub const CLK_TOP_LVDSTX_CLKDIG_EN: c_int = 132;
pub const CLK_TOP_VPLL_DPIX_EN: c_int = 133;
pub const CLK_TOP_SSUSB_TOP_CK_EN: c_int = 134;
pub const CLK_TOP_SSUSB_PHY_CK_EN: c_int = 135;
pub const CLK_TOP_CONN_32K: c_int = 136;
pub const CLK_TOP_CONN_26M: c_int = 137;
pub const CLK_TOP_DSP_32K: c_int = 138;
pub const CLK_TOP_DSP_26M: c_int = 139;
pub const CLK_TOP_NR_CLK: c_int = 140;
// INFRACFG
pub const CLK_IFR_PMIC_TMR: c_int = 0;
pub const CLK_IFR_PMIC_AP: c_int = 1;
pub const CLK_IFR_PMIC_MD: c_int = 2;
pub const CLK_IFR_PMIC_CONN: c_int = 3;
pub const CLK_IFR_ICUSB: c_int = 4;
pub const CLK_IFR_GCE: c_int = 5;
pub const CLK_IFR_THERM: c_int = 6;
pub const CLK_IFR_PWM_HCLK: c_int = 7;
pub const CLK_IFR_PWM1: c_int = 8;
pub const CLK_IFR_PWM2: c_int = 9;
pub const CLK_IFR_PWM3: c_int = 10;
pub const CLK_IFR_PWM4: c_int = 11;
pub const CLK_IFR_PWM5: c_int = 12;
pub const CLK_IFR_PWM: c_int = 13;
pub const CLK_IFR_UART0: c_int = 14;
pub const CLK_IFR_UART1: c_int = 15;
pub const CLK_IFR_UART2: c_int = 16;
pub const CLK_IFR_DSP_UART: c_int = 17;
pub const CLK_IFR_GCE_26M: c_int = 18;
pub const CLK_IFR_CQ_DMA_FPC: c_int = 19;
pub const CLK_IFR_BTIF: c_int = 20;
pub const CLK_IFR_SPI0: c_int = 21;
pub const CLK_IFR_MSDC0_HCLK: c_int = 22;
pub const CLK_IFR_MSDC2_HCLK: c_int = 23;
pub const CLK_IFR_MSDC1_HCLK: c_int = 24;
pub const CLK_IFR_DVFSRC: c_int = 25;
pub const CLK_IFR_GCPU: c_int = 26;
pub const CLK_IFR_TRNG: c_int = 27;
pub const CLK_IFR_AUXADC: c_int = 28;
pub const CLK_IFR_CPUM: c_int = 29;
pub const CLK_IFR_AUXADC_MD: c_int = 30;
pub const CLK_IFR_AP_DMA: c_int = 31;
pub const CLK_IFR_DEBUGSYS: c_int = 32;
pub const CLK_IFR_AUDIO: c_int = 33;
pub const CLK_IFR_PWM_FBCLK6: c_int = 34;
pub const CLK_IFR_DISP_PWM: c_int = 35;
pub const CLK_IFR_AUD_26M_BK: c_int = 36;
pub const CLK_IFR_CQ_DMA: c_int = 37;
pub const CLK_IFR_MSDC0_SF: c_int = 38;
pub const CLK_IFR_MSDC1_SF: c_int = 39;
pub const CLK_IFR_MSDC2_SF: c_int = 40;
pub const CLK_IFR_AP_MSDC0: c_int = 41;
pub const CLK_IFR_MD_MSDC0: c_int = 42;
pub const CLK_IFR_MSDC0_SRC: c_int = 43;
pub const CLK_IFR_MSDC1_SRC: c_int = 44;
pub const CLK_IFR_MSDC2_SRC: c_int = 45;
pub const CLK_IFR_PWRAP_TMR: c_int = 46;
pub const CLK_IFR_PWRAP_SPI: c_int = 47;
pub const CLK_IFR_PWRAP_SYS: c_int = 48;
pub const CLK_IFR_MCU_PM_BK: c_int = 49;
pub const CLK_IFR_IRRX_26M: c_int = 50;
pub const CLK_IFR_IRRX_32K: c_int = 51;
pub const CLK_IFR_I2C0_AXI: c_int = 52;
pub const CLK_IFR_I2C1_AXI: c_int = 53;
pub const CLK_IFR_I2C2_AXI: c_int = 54;
pub const CLK_IFR_I2C3_AXI: c_int = 55;
pub const CLK_IFR_NIC_AXI: c_int = 56;
pub const CLK_IFR_NIC_SLV_AXI: c_int = 57;
pub const CLK_IFR_APU_AXI: c_int = 58;
pub const CLK_IFR_NFIECC: c_int = 59;
pub const CLK_IFR_NFIECC_BK: c_int = 60;
pub const CLK_IFR_NFI1X_BK: c_int = 61;
pub const CLK_IFR_NFI_BK: c_int = 62;
pub const CLK_IFR_MSDC2_AP_BK: c_int = 63;
pub const CLK_IFR_MSDC2_MD_BK: c_int = 64;
pub const CLK_IFR_MSDC2_BK: c_int = 65;
pub const CLK_IFR_SUSB_133_BK: c_int = 66;
pub const CLK_IFR_SUSB_66_BK: c_int = 67;
pub const CLK_IFR_SSUSB_SYS: c_int = 68;
pub const CLK_IFR_SSUSB_REF: c_int = 69;
pub const CLK_IFR_SSUSB_XHCI: c_int = 70;
pub const CLK_IFR_NR_CLK: c_int = 71;
// PERICFG
pub const CLK_PERIAXI: c_int = 0;
pub const CLK_PERI_NR_CLK: c_int = 1;
// APMIXEDSYS
pub const CLK_APMIXED_ARMPLL: c_int = 0;
pub const CLK_APMIXED_MAINPLL: c_int = 1;
pub const CLK_APMIXED_UNIVPLL: c_int = 2;
pub const CLK_APMIXED_MFGPLL: c_int = 3;
pub const CLK_APMIXED_MSDCPLL: c_int = 4;
pub const CLK_APMIXED_MMPLL: c_int = 5;
pub const CLK_APMIXED_APLL1: c_int = 6;
pub const CLK_APMIXED_APLL2: c_int = 7;
pub const CLK_APMIXED_LVDSPLL: c_int = 8;
pub const CLK_APMIXED_DSPPLL: c_int = 9;
pub const CLK_APMIXED_APUPLL: c_int = 10;
pub const CLK_APMIXED_UNIV_EN: c_int = 11;
pub const CLK_APMIXED_USB20_EN: c_int = 12;
pub const CLK_APMIXED_NR_CLK: c_int = 13;
// GCE
pub const CLK_GCE_FAXI: c_int = 0;
pub const CLK_GCE_NR_CLK: c_int = 1;
// AUDIOTOP
pub const CLK_AUD_AFE: c_int = 0;
pub const CLK_AUD_I2S: c_int = 1;
pub const CLK_AUD_22M: c_int = 2;
pub const CLK_AUD_24M: c_int = 3;
pub const CLK_AUD_INTDIR: c_int = 4;
pub const CLK_AUD_APLL2_TUNER: c_int = 5;
pub const CLK_AUD_APLL_TUNER: c_int = 6;
pub const CLK_AUD_SPDF: c_int = 7;
pub const CLK_AUD_HDMI: c_int = 8;
pub const CLK_AUD_HDMI_IN: c_int = 9;
pub const CLK_AUD_ADC: c_int = 10;
pub const CLK_AUD_DAC: c_int = 11;
pub const CLK_AUD_DAC_PREDIS: c_int = 12;
pub const CLK_AUD_TML: c_int = 13;
pub const CLK_AUD_I2S1_BK: c_int = 14;
pub const CLK_AUD_I2S2_BK: c_int = 15;
pub const CLK_AUD_I2S3_BK: c_int = 16;
pub const CLK_AUD_I2S4_BK: c_int = 17;
pub const CLK_AUD_NR_CLK: c_int = 18;
// MIPI_CSI0A
pub const CLK_MIPI0A_CSR_CSI_EN_0A: c_int = 0;
pub const CLK_MIPI_RX_ANA_CSI0A_NR_CLK: c_int = 1;
// MIPI_CSI0B
pub const CLK_MIPI0B_CSR_CSI_EN_0B: c_int = 0;
pub const CLK_MIPI_RX_ANA_CSI0B_NR_CLK: c_int = 1;
// MIPI_CSI1A
pub const CLK_MIPI1A_CSR_CSI_EN_1A: c_int = 0;
pub const CLK_MIPI_RX_ANA_CSI1A_NR_CLK: c_int = 1;
// MIPI_CSI1B
pub const CLK_MIPI1B_CSR_CSI_EN_1B: c_int = 0;
pub const CLK_MIPI_RX_ANA_CSI1B_NR_CLK: c_int = 1;
// MIPI_CSI2A
pub const CLK_MIPI2A_CSR_CSI_EN_2A: c_int = 0;
pub const CLK_MIPI_RX_ANA_CSI2A_NR_CLK: c_int = 1;
// MIPI_CSI2B
pub const CLK_MIPI2B_CSR_CSI_EN_2B: c_int = 0;
pub const CLK_MIPI_RX_ANA_CSI2B_NR_CLK: c_int = 1;
// MCUCFG
pub const CLK_MCU_BUS_SEL: c_int = 0;
pub const CLK_MCU_NR_CLK: c_int = 1;
// MFGCFG
pub const CLK_MFG_BG3D: c_int = 0;
pub const CLK_MFG_MBIST_DIAG: c_int = 1;
pub const CLK_MFG_NR_CLK: c_int = 2;
// MMSYS
pub const CLK_MM_MM_MDP_RDMA0: c_int = 0;
pub const CLK_MM_MM_MDP_CCORR0: c_int = 1;
pub const CLK_MM_MM_MDP_RSZ0: c_int = 2;
pub const CLK_MM_MM_MDP_RSZ1: c_int = 3;
pub const CLK_MM_MM_MDP_TDSHP0: c_int = 4;
pub const CLK_MM_MM_MDP_WROT0: c_int = 5;
pub const CLK_MM_MM_MDP_WDMA0: c_int = 6;
pub const CLK_MM_MM_DISP_OVL0: c_int = 7;
pub const CLK_MM_MM_DISP_OVL0_2L: c_int = 8;
pub const CLK_MM_MM_DISP_RSZ0: c_int = 9;
pub const CLK_MM_MM_DISP_RDMA0: c_int = 10;
pub const CLK_MM_MM_DISP_WDMA0: c_int = 11;
pub const CLK_MM_MM_DISP_COLOR0: c_int = 12;
pub const CLK_MM_MM_DISP_CCORR0: c_int = 13;
pub const CLK_MM_MM_DISP_AAL0: c_int = 14;
pub const CLK_MM_MM_DISP_GAMMA0: c_int = 15;
pub const CLK_MM_MM_DISP_DITHER0: c_int = 16;
pub const CLK_MM_MM_DSI0: c_int = 17;
pub const CLK_MM_MM_DISP_RDMA1: c_int = 18;
pub const CLK_MM_MM_MDP_RDMA1: c_int = 19;
pub const CLK_MM_DPI0_DPI0: c_int = 20;
pub const CLK_MM_MM_FAKE: c_int = 21;
pub const CLK_MM_MM_SMI_COMMON: c_int = 22;
pub const CLK_MM_MM_SMI_LARB0: c_int = 23;
pub const CLK_MM_MM_SMI_COMM0: c_int = 24;
pub const CLK_MM_MM_SMI_COMM1: c_int = 25;
pub const CLK_MM_MM_CAM_MDP: c_int = 26;
pub const CLK_MM_MM_SMI_IMG: c_int = 27;
pub const CLK_MM_MM_SMI_CAM: c_int = 28;
pub const CLK_MM_IMG_IMG_DL_RELAY: c_int = 29;
pub const CLK_MM_IMG_IMG_DL_ASYNC_TOP: c_int = 30;
pub const CLK_MM_DSI0_DIG_DSI: c_int = 31;
pub const CLK_MM_26M_HRTWT: c_int = 32;
pub const CLK_MM_MM_DPI0: c_int = 33;
pub const CLK_MM_LVDSTX_PXL: c_int = 34;
pub const CLK_MM_LVDSTX_CTS: c_int = 35;
pub const CLK_MM_NR_CLK: c_int = 36;
// IMGSYS
pub const CLK_CAM_LARB2: c_int = 0;
pub const CLK_CAM: c_int = 1;
pub const CLK_CAMTG: c_int = 2;
pub const CLK_CAM_SENIF: c_int = 3;
pub const CLK_CAMSV0: c_int = 4;
pub const CLK_CAMSV1: c_int = 5;
pub const CLK_CAM_FDVT: c_int = 6;
pub const CLK_CAM_WPE: c_int = 7;
pub const CLK_CAM_NR_CLK: c_int = 8;
// VDECSYS
pub const CLK_VDEC_VDEC: c_int = 0;
pub const CLK_VDEC_LARB1: c_int = 1;
pub const CLK_VDEC_NR_CLK: c_int = 2;
// VENCSYS
pub const CLK_VENC: c_int = 0;
pub const CLK_VENC_JPGENC: c_int = 1;
pub const CLK_VENC_NR_CLK: c_int = 2;
// APUSYS
pub const CLK_APU_IPU_CK: c_int = 0;
pub const CLK_APU_AXI: c_int = 1;
pub const CLK_APU_JTAG: c_int = 2;
pub const CLK_APU_IF_CK: c_int = 3;
pub const CLK_APU_EDMA: c_int = 4;
pub const CLK_APU_AHB: c_int = 5;
pub const CLK_APU_NR_CLK: c_int = 6;
