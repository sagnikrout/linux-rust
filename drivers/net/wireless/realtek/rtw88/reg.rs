//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/reg.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const REG_SYS_FUNC_EN: c_uint = 0x0002;

pub const REG_SYS_PW_CTRL: c_uint = 0x0004;

pub const REG_APS_FSMCO: c_uint = 0x0004;

pub const REG_SYS_CLK_CTRL: c_uint = 0x0008;

pub const REG_SYS_CLKR: c_uint = 0x0008;

pub const REG_RSV_CTRL: c_uint = 0x001C;
pub const DISABLE_PI: c_uint = 0x3;
pub const ENABLE_PI: c_uint = 0x2;

pub const REG_RF_CTRL: c_uint = 0x001F;

pub const REG_RF_CTRL1: c_uint = 0x0020;
pub const REG_RF_CTRL2: c_uint = 0x0021;
pub const REG_AFE_CTRL1: c_uint = 0x0024;

pub const REG_EFUSE_CTRL: c_uint = 0x0030;

pub const BIT_SHIFT_EF_ADDR: c_int = 8;
pub const BIT_MASK_EF_ADDR: c_uint = 0x3ff;
pub const BIT_MASK_EF_DATA: c_uint = 0xff;

pub const BITS_PLL: c_uint = 0xf0;
pub const REG_AFE_XTAL_CTRL: c_uint = 0x24;
pub const REG_AFE_PLL_CTRL: c_uint = 0x28;
pub const REG_AFE_CTRL3: c_uint = 0x2c;
pub const BIT_MASK_XTAL: c_uint = 0x00FFF000;

pub const REG_LDO_EFUSE_CTRL: c_uint = 0x0034;

pub const BIT_LDO25_VOLTAGE_V25: c_uint = 0x03;

pub const BIT_SHIFT_LDO25_VOLTAGE: c_int = 4;

pub const REG_ACLK_MON: c_uint = 0x3e;
pub const REG_GPIO_MUXCFG: c_uint = 0x0040;

pub const REG_GPIO_PIN_CTRL: c_uint = 0x0044;
pub const REG_LED_CFG: c_uint = 0x004C;

pub const BIT_LED_MODE_SW_CTRL: c_int = 0;
pub const BIT_LED_MODE_RX: c_int = 6;
pub const BIT_LED_MODE_TX: c_int = 4;
pub const BIT_LED_MODE_TRX: c_int = 2;
pub const REG_LEDCFG2: c_uint = 0x004E;
pub const REG_GPIO_PIN_CTRL_2: c_uint = 0x0060;
pub const REG_PAD_CTRL1: c_uint = 0x0064;

pub const REG_WL_BT_PWR_CTRL: c_uint = 0x0068;

pub const REG_SYS_SDIO_CTRL: c_uint = 0x0070;

pub const REG_HCI_OPT_CTRL: c_uint = 0x0074;

pub const REG_RF_B_CTRL: c_uint = 0x76;
pub const REG_RF_CTRL3: c_uint = 0x0076;
pub const REG_AFE_CTRL_4: c_uint = 0x0078;

pub const REG_LDO_SWR_CTRL: c_uint = 0x007C;
pub const LDO_SEL: c_uint = 0xC3;
pub const SPS_SEL: c_uint = 0x83;

pub const REG_MCUFW_CTRL: c_uint = 0x0080;

pub const BIT_SHIFT_ROM_PGE: c_int = 16;

pub const REG_MCU_TST_CFG: c_uint = 0x84;
pub const VAL_FW_TRIGGER: c_uint = 0x1;
pub const REG_PMC_DBG_CTRL1: c_uint = 0xa8;

pub const REG_HIMR0: c_uint = 0xb0;
pub const REG_HISR0: c_uint = 0xb4;
pub const REG_HIMR1: c_uint = 0xb8;
pub const REG_HISR1: c_uint = 0xbc;
pub const REG_PAD_CTRL2: c_uint = 0x00C4;

pub const BIT_USB_MODE_U2: c_int = 1;
pub const BIT_USB_MODE_U3: c_int = 2;
pub const REG_EFUSE_ACCESS: c_uint = 0x00CF;
pub const EFUSE_ACCESS_ON: c_uint = 0x69;
pub const EFUSE_ACCESS_OFF: c_uint = 0x00;
pub const REG_WLRF1: c_uint = 0x00EC;
pub const REG_WIFI_BT_INFO: c_uint = 0x00AA;

pub const REG_SYS_CFG1: c_uint = 0x00F0;

pub const BIT_SHIFT_VENDOR_ID: c_int = 16;
pub const BIT_MASK_VENDOR_ID: c_uint = 0xf;

pub const BIT_SHIFT_CHIP_VER: c_int = 12;
pub const BIT_MASK_CHIP_VER: c_uint = 0xf;

pub const REG_SYS_STATUS1: c_uint = 0x00F4;
pub const REG_SYS_STATUS2: c_uint = 0x00F8;
pub const REG_SYS_CFG2: c_uint = 0x00FC;
pub const REG_WLRF1: c_uint = 0x00EC;

pub const REG_CR: c_uint = 0x0100;

pub const REG_PBP: c_uint = 0x104;
pub const PBP_RX_MASK: c_uint = 0x0f;
pub const PBP_TX_MASK: c_uint = 0xf0;
pub const PBP_64: c_uint = 0x0;
pub const PBP_128: c_uint = 0x1;
pub const PBP_256: c_uint = 0x2;
pub const PBP_512: c_uint = 0x3;
pub const PBP_1024: c_uint = 0x4;
pub const BIT_SHIFT_TXDMA_VOQ_MAP: c_int = 4;
pub const BIT_MASK_TXDMA_VOQ_MAP: c_uint = 0x3;

pub const BIT_SHIFT_TXDMA_VIQ_MAP: c_int = 6;
pub const BIT_MASK_TXDMA_VIQ_MAP: c_uint = 0x3;

pub const REG_TXDMA_PQ_MAP: c_uint = 0x010C;

pub const BIT_SHIFT_TXDMA_BEQ_MAP: c_int = 8;
pub const BIT_MASK_TXDMA_BEQ_MAP: c_uint = 0x3;

pub const BIT_SHIFT_TXDMA_BKQ_MAP: c_int = 10;
pub const BIT_MASK_TXDMA_BKQ_MAP: c_uint = 0x3;

pub const BIT_SHIFT_TXDMA_MGQ_MAP: c_int = 12;
pub const BIT_MASK_TXDMA_MGQ_MAP: c_uint = 0x3;

pub const BIT_SHIFT_TXDMA_HIQ_MAP: c_int = 14;
pub const BIT_MASK_TXDMA_HIQ_MAP: c_uint = 0x3;

pub const BIT_SHIFT_TXSC_40M: c_int = 4;
pub const BIT_MASK_TXSC_40M: c_uint = 0xf;

pub const BIT_SHIFT_TXSC_20M: c_int = 0;
pub const BIT_MASK_TXSC_20M: c_uint = 0xf;

pub const BIT_SHIFT_MAC_CLK_SEL: c_int = 20;
pub const MAC_CLK_HW_DEF_80M: c_int = 0;
pub const MAC_CLK_HW_DEF_40M: c_int = 1;
pub const MAC_CLK_HW_DEF_20M: c_int = 2;
pub const MAC_CLK_SPEED: c_int = 80;
pub const REG_CR: c_uint = 0x0100;
pub const REG_TRXFF_BNDY: c_uint = 0x0114;
pub const REG_RXFF_BNDY: c_uint = 0x011C;
pub const REG_FE1IMR: c_uint = 0x0120;

pub const REG_CPWM: c_uint = 0x012C;
pub const REG_FWIMR: c_uint = 0x0130;

pub const REG_FWISR: c_uint = 0x0134;

pub const REG_PKTBUF_DBG_CTRL: c_uint = 0x0140;
pub const REG_C2HEVT: c_uint = 0x01A0;
pub const REG_MCUTST_1: c_uint = 0x01C0;
pub const REG_MCUTST_II: c_uint = 0x01C4;
pub const REG_WOWLAN_WAKE_REASON: c_uint = 0x01C7;
pub const REG_HMETFR: c_uint = 0x01CC;

pub const REG_HMEBOX0: c_uint = 0x01D0;
pub const REG_HMEBOX1: c_uint = 0x01D4;
pub const REG_HMEBOX2: c_uint = 0x01D8;
pub const REG_HMEBOX3: c_uint = 0x01DC;
pub const REG_LLT_INIT: c_uint = 0x01E0;

pub const REG_HMEBOX0_EX: c_uint = 0x01F0;
pub const REG_HMEBOX1_EX: c_uint = 0x01F4;
pub const REG_HMEBOX2_EX: c_uint = 0x01F8;
pub const REG_HMEBOX3_EX: c_uint = 0x01FC;
pub const REG_RQPN: c_uint = 0x0200;
pub const BIT_MASK_HPQ: c_uint = 0xff;
pub const BIT_SHIFT_HPQ: c_int = 0;

pub const BIT_MASK_LPQ: c_uint = 0xff;
pub const BIT_SHIFT_LPQ: c_int = 8;

pub const BIT_MASK_PUBQ: c_uint = 0xff;
pub const BIT_SHIFT_PUBQ: c_int = 16;

pub const REG_FIFOPAGE_CTRL_2: c_uint = 0x0204;

pub const BIT_MASK_BCN_HEAD_1_V1: c_uint = 0xfff;
pub const REG_AUTO_LLT_V1: c_uint = 0x0208;

pub const REG_DWBCN0_CTRL: c_uint = 0x0208;

pub const REG_TXDMA_OFFSET_CHK: c_uint = 0x020C;

pub const REG_TXDMA_STATUS: c_uint = 0x0210;

pub const REG_RQPN_NPQ: c_uint = 0x0214;
pub const BIT_MASK_NPQ: c_uint = 0xff;
pub const BIT_SHIFT_NPQ: c_int = 0;
pub const BIT_MASK_EPQ: c_uint = 0xff;
pub const BIT_SHIFT_EPQ: c_int = 16;

pub const REG_AUTO_LLT: c_uint = 0x0224;

pub const REG_DWBCN1_CTRL: c_uint = 0x0228;
pub const REG_RQPN_CTRL_1: c_uint = 0x0228;
pub const REG_RQPN_CTRL_2: c_uint = 0x022C;

pub const REG_FIFOPAGE_INFO_1: c_uint = 0x0230;
pub const REG_FIFOPAGE_INFO_2: c_uint = 0x0234;
pub const REG_FIFOPAGE_INFO_3: c_uint = 0x0238;
pub const REG_FIFOPAGE_INFO_4: c_uint = 0x023C;
pub const REG_FIFOPAGE_INFO_5: c_uint = 0x0240;
pub const REG_H2C_HEAD: c_uint = 0x0244;
pub const REG_H2C_TAIL: c_uint = 0x0248;
pub const REG_H2C_READ_ADDR: c_uint = 0x024C;
pub const REG_H2C_INFO: c_uint = 0x0254;
pub const REG_RXDMA_AGG_PG_TH: c_uint = 0x0280;

pub const REG_RXPKT_NUM: c_uint = 0x0284;

pub const REG_RXDMA_STATUS: c_uint = 0x0288;
pub const REG_RXDMA_DPR: c_uint = 0x028C;
pub const REG_RXDMA_MODE: c_uint = 0x0290;

pub const BIT_DMA_BURST_SIZE_64: c_int = 2;
pub const BIT_DMA_BURST_SIZE_512: c_int = 1;
pub const BIT_DMA_BURST_SIZE_1024: c_int = 0;
pub const REG_RXPKTNUM: c_uint = 0x02B0;
pub const REG_EARLY_MODE_CONTROL: c_uint = 0x02BC;
pub const REG_INT_MIG: c_uint = 0x0304;
pub const REG_HCI_MIX_CFG: c_uint = 0x03FC;

pub const REG_BCNQ_INFO: c_uint = 0x0418;

pub const REG_TXPKT_EMPTY: c_uint = 0x041A;
pub const REG_FWHW_TXQ_CTRL: c_uint = 0x0420;

pub const REG_HWSEQ_CTRL: c_uint = 0x0423;
pub const REG_BCNQ_BDNY_V1: c_uint = 0x0424;
pub const REG_BCNQ_BDNY: c_uint = 0x0424;
pub const REG_MGQ_BDNY: c_uint = 0x0425;
pub const REG_LIFETIME_EN: c_uint = 0x0426;

pub const REG_SPEC_SIFS: c_uint = 0x0428;
pub const REG_RETRY_LIMIT: c_uint = 0x042a;
pub const REG_DARFRC: c_uint = 0x0430;
pub const REG_DARFRCH: c_uint = 0x0434;
pub const REG_RARFRCH: c_uint = 0x043C;
pub const REG_RRSR: c_uint = 0x0440;

pub const REG_ARFR0: c_uint = 0x0444;
pub const REG_ARFRH0: c_uint = 0x0448;
pub const REG_ARFR1_V1: c_uint = 0x044C;
pub const REG_ARFRH1_V1: c_uint = 0x0450;
pub const REG_CCK_CHECK: c_uint = 0x0454;

pub const REG_AMPDU_MAX_TIME_V1: c_uint = 0x0455;
pub const REG_BCNQ1_BDNY_V1: c_uint = 0x0456;
pub const REG_AMPDU_MAX_TIME: c_uint = 0x0456;
pub const REG_AMPDU_MAX_LENGTH: c_uint = 0x0458;
pub const REG_WMAC_LBK_BF_HD: c_uint = 0x045D;
pub const REG_TX_HANG_CTRL: c_uint = 0x045E;

pub const REG_FAST_EDCA_CTRL: c_uint = 0x0460;
pub const REG_DATA_SC: c_uint = 0x0483;
pub const REG_ARFR2_V1: c_uint = 0x048C;
pub const REG_ARFRH2_V1: c_uint = 0x0490;
pub const REG_ARFR3_V1: c_uint = 0x0494;

pub const REG_ARFRH3_V1: c_uint = 0x0498;
pub const REG_ARFR4: c_uint = 0x049C;

pub const REG_ARFRH4: c_uint = 0x04A0;
pub const REG_ARFR5: c_uint = 0x04A4;
pub const REG_ARFRH5: c_uint = 0x04A8;
pub const REG_SW_AMPDU_BURST_MODE_CTRL: c_uint = 0x04BC;

pub const REG_QUEUE_CTRL: c_uint = 0x04C6;

pub const REG_SINGLE_AMPDU_CTRL: c_uint = 0x04C7;

pub const REG_PROT_MODE_CTRL: c_uint = 0x04C8;
pub const REG_MAX_AGGR_NUM: c_uint = 0x04CA;
pub const REG_BAR_MODE_CTRL: c_uint = 0x04CC;
pub const REG_PRECNT_CTRL: c_uint = 0x04E5;

pub const REG_TX_RPT_CTRL: c_uint = 0x04EC;
pub const REG_TX_RPT_TIME: c_uint = 0x04F0;
pub const REG_DUMMY_PAGE4_V1: c_uint = 0x04FC;
pub const REG_EDCA_VO_PARAM: c_uint = 0x0500;
pub const REG_EDCA_VI_PARAM: c_uint = 0x0504;
pub const REG_EDCA_BE_PARAM: c_uint = 0x0508;
pub const REG_EDCA_BK_PARAM: c_uint = 0x050C;

pub const REG_BCNTCFG: c_uint = 0x0510;
pub const REG_PIFS: c_uint = 0x0512;
pub const REG_SIFS: c_uint = 0x0514;
pub const BIT_SHIFT_SIFS_OFDM_CTX: c_int = 8;
pub const BIT_SHIFT_SIFS_CCK_TRX: c_int = 16;
pub const BIT_SHIFT_SIFS_OFDM_TRX: c_int = 24;
pub const REG_AGGR_BREAK_TIME: c_uint = 0x051A;
pub const REG_SLOT: c_uint = 0x051B;
pub const REG_TX_PTCL_CTRL: c_uint = 0x0520;

pub const REG_TXPAUSE: c_uint = 0x0522;

pub const REG_RD_CTRL: c_uint = 0x0524;

pub const REG_TBTT_PROHIBIT: c_uint = 0x0540;
pub const BIT_SHIFT_TBTT_HOLD_TIME_AP: c_int = 8;
pub const REG_RD_NAV_NXT: c_uint = 0x0544;
pub const REG_NAV_PROT_LEN: c_uint = 0x0546;
pub const REG_BCN_CTRL: c_uint = 0x0550;

pub const REG_BCN_CTRL_CLINT0: c_uint = 0x0551;
pub const REG_DRVERLYINT: c_uint = 0x0558;
pub const REG_BCNDMATIM: c_uint = 0x0559;
pub const REG_ATIMWND: c_uint = 0x055A;
pub const REG_USTIME_TSF: c_uint = 0x055C;
pub const REG_BCN_MAX_ERR: c_uint = 0x055D;
pub const REG_RXTSF_OFFSET_CCK: c_uint = 0x055E;
pub const REG_MISC_CTRL: c_uint = 0x0577;

pub const REG_HIQ_NO_LMT_EN: c_uint = 0x5A7;
pub const REG_DTIM_COUNTER_ROOT: c_uint = 0x5A8;

pub const REG_TIMER0_SRC_SEL: c_uint = 0x05B4;

pub const REG_TCR: c_uint = 0x0604;

pub const REG_RCR: c_uint = 0x0608;

pub const REG_RX_PKT_LIMIT: c_uint = 0x060C;
pub const REG_RX_DRVINFO_SZ: c_uint = 0x060F;

pub const REG_MAR: c_uint = 0x0620;
pub const REG_USTIME_EDCA: c_uint = 0x0638;
pub const REG_ACKTO_CCK: c_uint = 0x0639;
pub const REG_MAC_SPEC_SIFS: c_uint = 0x063A;
pub const REG_RESP_SIFS_CCK: c_uint = 0x063C;
pub const REG_RESP_SIFS_OFDM: c_uint = 0x063E;
pub const REG_ACKTO: c_uint = 0x0640;
pub const REG_EIFS: c_uint = 0x0642;
pub const REG_NAV_CTRL: c_uint = 0x0650;
pub const REG_WMAC_TRXPTCL_CTL: c_uint = 0x0668;

pub const REG_WMAC_TRXPTCL_CTL_H: c_uint = 0x066C;
pub const REG_WKFMCAM_CMD: c_uint = 0x0698;

pub const BIT_SHIFT_WKFCAM_ADDR_V2: c_int = 8;
pub const BIT_MASK_WKFCAM_ADDR_V2: c_uint = 0xff;

pub const REG_WKFMCAM_RWD: c_uint = 0x069C;

pub const REG_RXFLTMAP0: c_uint = 0x06A0;
pub const REG_RXFLTMAP1: c_uint = 0x06A2;
pub const REG_RXFLTMAP2: c_uint = 0x06A4;
pub const REG_RXFLTMAP4: c_uint = 0x068A;
pub const REG_BT_COEX_TABLE0: c_uint = 0x06C0;
pub const REG_BT_COEX_TABLE1: c_uint = 0x06C4;
pub const REG_BT_COEX_BRK_TABLE: c_uint = 0x06C8;
pub const REG_BT_COEX_TABLE_H: c_uint = 0x06CC;
pub const REG_BT_COEX_TABLE_H1: c_uint = 0x06CD;
pub const REG_BT_COEX_TABLE_H2: c_uint = 0x06CE;
pub const REG_BT_COEX_TABLE_H3: c_uint = 0x06CF;
pub const REG_BBPSF_CTRL: c_uint = 0x06DC;
pub const REG_BT_COEX_V2: c_uint = 0x0762;

pub const REG_GNT_BT: c_uint = 0x0765;

pub const REG_BT_COEX_ENH_INTR_CTRL: c_uint = 0x76E;

pub const REG_BT_ACT_STATISTICS: c_uint = 0x0770;
pub const REG_BT_ACT_STATISTICS_1: c_uint = 0x0774;
pub const REG_BT_STAT_CTRL: c_uint = 0x0778;
pub const REG_BT_TDMA_TIME: c_uint = 0x0790;

pub const REG_LTR_IDLE_LATENCY: c_uint = 0x0798;
pub const REG_LTR_ACTIVE_LATENCY: c_uint = 0x079C;
pub const REG_LTR_CTRL_BASIC: c_uint = 0x07A4;
pub const REG_WMAC_OPTION_FUNCTION: c_uint = 0x07D0;
pub const REG_WMAC_OPTION_FUNCTION_1: c_uint = 0x07D4;
pub const REG_FPGA0_RFMOD: c_uint = 0x0800;

pub const REG_CCK_RPT_FORMAT: c_uint = 0x0804;

pub const REG_RXPSEL: c_uint = 0x0808;

pub const REG_TXPSEL: c_uint = 0x080C;
pub const REG_RX_GAIN_EN: c_uint = 0x081c;
pub const REG_CCASEL: c_uint = 0x082C;
pub const REG_PDMFTH: c_uint = 0x0830;
pub const REG_BWINDICATION: c_uint = 0x0834;
pub const REG_CCA2ND: c_uint = 0x0838;
pub const REG_L1PKTH: c_uint = 0x0848;
pub const REG_CLKTRK: c_uint = 0x0860;
pub const REG_CSI_MASK_SETTING1: c_uint = 0x0874;
pub const REG_NBI_SETTING: c_uint = 0x087c;

pub const REG_CSI_FIX_MASK0: c_uint = 0x0880;
pub const REG_CSI_FIX_MASK1: c_uint = 0x0884;
pub const REG_CSI_FIX_MASK6: c_uint = 0x0898;
pub const REG_CSI_FIX_MASK7: c_uint = 0x089c;
pub const REG_ADCCLK: c_uint = 0x08AC;
pub const REG_HSSI_READ: c_uint = 0x08B0;
pub const REG_FPGA0_XCD_RF_PARA: c_uint = 0x08B4;
pub const REG_RX_MCS_LIMIT: c_uint = 0x08BC;
pub const REG_ADC160: c_uint = 0x08C4;
pub const REG_DBGSEL: c_uint = 0x08fc;
pub const REG_ANTSEL_SW: c_uint = 0x0900;
pub const REG_DAC_RSTB: c_uint = 0x090c;
pub const REG_PSD: c_uint = 0x0910;

pub const REG_SINGLE_TONE_CONT_TX: c_uint = 0x0914;
pub const REG_AGC_TABLE: c_uint = 0x0958;
pub const REG_RFE_CTRL_E: c_uint = 0x0974;
pub const REG_2ND_CCA_CTRL: c_uint = 0x0976;
pub const REG_IQK_COM00: c_uint = 0x0978;
pub const REG_IQK_COM32: c_uint = 0x097c;
pub const REG_IQK_COM64: c_uint = 0x0980;
pub const REG_IQK_COM96: c_uint = 0x0984;
pub const REG_FAS: c_uint = 0x09a4;
pub const REG_RXSB: c_uint = 0x0a00;

pub const REG_CCK_RX: c_uint = 0x0a04;
pub const REG_CCK_PD_TH: c_uint = 0x0a0a;
pub const REG_PRECTRL: c_uint = 0x0a14;

pub const REG_CCA_MF: c_uint = 0x0a20;

pub const REG_CCK0_TX_FILTER1: c_uint = 0x0a20;
pub const REG_CCK0_TX_FILTER2: c_uint = 0x0a24;
pub const REG_CCK0_DEBUG_PORT: c_uint = 0x0a28;
pub const REG_CCK0_FAREPORT: c_uint = 0x0a2c;

pub const REG_FA_CCK: c_uint = 0x0a5c;
pub const REG_DIS_DPD: c_uint = 0x0a70;

pub const REG_CCA: c_uint = 0x0a70;

pub const REG_ANTSEL: c_uint = 0x0a74;

pub const REG_CCKTX: c_uint = 0x0a84;

pub const REG_CNTRST: c_uint = 0x0b58;
pub const REG_3WIRE_SWA: c_uint = 0x0c00;
pub const REG_RX_IQC_AB_A: c_uint = 0x0c10;
pub const REG_RX_IQC_CD_A: c_uint = 0x0c14;
pub const REG_TXSCALE_A: c_uint = 0x0c1c;

pub const REG_TX_AGC_A_CCK_11_CCK_1: c_uint = 0xc20;
pub const REG_TX_AGC_A_OFDM18_OFDM6: c_uint = 0xc24;
pub const REG_TX_AGC_A_OFDM54_OFDM24: c_uint = 0xc28;
pub const REG_TX_AGC_A_MCS3_MCS0: c_uint = 0xc2c;
pub const REG_TX_AGC_A_MCS7_MCS4: c_uint = 0xc30;
pub const REG_TX_AGC_A_MCS11_MCS8: c_uint = 0xc34;
pub const REG_TX_AGC_A_MCS15_MCS12: c_uint = 0xc38;
pub const REG_TX_AGC_A_NSS1_INDEX3_NSS1_INDEX0: c_uint = 0xc3c;
pub const REG_TX_AGC_A_NSS1_INDEX7_NSS1_INDEX4: c_uint = 0xc40;
pub const REG_TX_AGC_A_NSS2_INDEX1_NSS1_INDEX8: c_uint = 0xc44;
pub const REG_TX_AGC_A_NSS2_INDEX5_NSS2_INDEX2: c_uint = 0xc48;
pub const REG_TX_AGC_A_NSS2_INDEX9_NSS2_INDEX6: c_uint = 0xc4c;
pub const REG_RXIGI_A: c_uint = 0x0c50;
pub const REG_TX_PWR_TRAINING_A: c_uint = 0x0c54;
pub const REG_CK_MONHA: c_uint = 0x0c5c;
pub const REG_AFE_PWR1_A: c_uint = 0x0c60;
pub const REG_AFE_PWR2_A: c_uint = 0x0c64;
pub const REG_RX_WAIT_CCA_TX_CCK_RFON_A: c_uint = 0x0c68;
pub const REG_OFDM0_XA_TX_IQ_IMBALANCE: c_uint = 0x0c80;
pub const REG_OFDM0_A_TX_AFE: c_uint = 0x0c84;
pub const REG_OFDM0_XB_TX_IQ_IMBALANCE: c_uint = 0x0c88;
pub const REG_TSSI_TRK_SW: c_uint = 0x0c8c;
pub const REG_LSSI_WRITE_A: c_uint = 0x0c90;
pub const REG_PREDISTA: c_uint = 0x0c90;
pub const REG_TXAGCIDX: c_uint = 0x0c94;
pub const REG_TX_AGC_A: c_uint = 0x0c94;
pub const REG_RFE_PINMUX_A: c_uint = 0x0cb0;
pub const REG_RFE_INV_A: c_uint = 0x0cb4;
pub const REG_RFE_CTRL8: c_uint = 0x0cb4;

pub const PTA_CTRL_PIN: c_uint = 0x66;
pub const DPDT_CTRL_PIN: c_uint = 0x77;
pub const RFE_INV_MASK: c_uint = 0x3ff00000;
pub const REG_RFECTL_A: c_uint = 0x0cb8;
pub const REG_RFE_INV0: c_uint = 0x0cbc;
pub const REG_RFE_INV8: c_uint = 0x0cbd;

pub const REG_RFE_INV16: c_uint = 0x0cbe;

pub const REG_IQK_DPD_CFG: c_uint = 0x0cc4;
pub const REG_CFG_PMPD: c_uint = 0x0cc8;
pub const REG_IQC_Y: c_uint = 0x0ccc;
pub const REG_IQC_X: c_uint = 0x0cd4;
pub const REG_INTPO_SETA: c_uint = 0x0ce8;
pub const REG_IQKA_END: c_uint = 0x0d00;
pub const REG_PI_READ_A: c_uint = 0x0d04;
pub const REG_SI_READ_A: c_uint = 0x0d08;
pub const REG_IQKB_END: c_uint = 0x0d40;
pub const REG_PI_READ_B: c_uint = 0x0d44;
pub const REG_SI_READ_B: c_uint = 0x0d48;
pub const REG_3WIRE_SWB: c_uint = 0x0e00;
pub const REG_RX_IQC_AB_B: c_uint = 0x0e10;
pub const REG_RX_IQC_CD_B: c_uint = 0x0e14;
pub const REG_TXSCALE_B: c_uint = 0x0e1c;
pub const REG_TX_AGC_B_CCK_11_CCK_1: c_uint = 0xe20;
pub const REG_TX_AGC_B_OFDM18_OFDM6: c_uint = 0xe24;
pub const REG_TX_AGC_B_OFDM54_OFDM24: c_uint = 0xe28;
pub const REG_TX_AGC_B_MCS3_MCS0: c_uint = 0xe2c;
pub const REG_TX_AGC_B_MCS7_MCS4: c_uint = 0xe30;
pub const REG_TX_AGC_B_MCS11_MCS8: c_uint = 0xe34;
pub const REG_TX_AGC_B_MCS15_MCS12: c_uint = 0xe38;
pub const REG_TX_AGC_B_NSS1_INDEX3_NSS1_INDEX0: c_uint = 0xe3c;
pub const REG_TX_AGC_B_NSS1_INDEX7_NSS1_INDEX4: c_uint = 0xe40;
pub const REG_TX_AGC_B_NSS2_INDEX1_NSS1_INDEX8: c_uint = 0xe44;
pub const REG_TX_AGC_B_NSS2_INDEX5_NSS2_INDEX2: c_uint = 0xe48;
pub const REG_TX_AGC_B_NSS2_INDEX9_NSS2_INDEX6: c_uint = 0xe4c;
pub const REG_RXIGI_B: c_uint = 0x0e50;
pub const REG_TX_PWR_TRAINING_B: c_uint = 0x0e54;
pub const REG_CK_MONHB: c_uint = 0x0e5c;
pub const REG_AFE_PWR1_B: c_uint = 0x0e60;
pub const REG_AFE_PWR2_B: c_uint = 0x0e64;
pub const REG_RX_WAIT_CCA_TX_CCK_RFON_B: c_uint = 0x0e68;
pub const REG_TXTONEB: c_uint = 0x0e80;
pub const REG_RXTONEB: c_uint = 0x0e84;
pub const REG_TXPITMB: c_uint = 0x0e88;
pub const REG_RXPITMB: c_uint = 0x0e8c;
pub const REG_LSSI_WRITE_B: c_uint = 0x0e90;
pub const REG_PREDISTB: c_uint = 0x0e90;
pub const REG_INIDLYB: c_uint = 0x0e94;
pub const REG_TX_AGC_B: c_uint = 0x0e94;
pub const REG_RFE_PINMUX_B: c_uint = 0x0eb0;
pub const REG_RFE_INV_B: c_uint = 0x0eb4;
pub const REG_RFECTL_B: c_uint = 0x0eb8;
pub const REG_BPBDB: c_uint = 0x0ec4;
pub const REG_PHYTXONB: c_uint = 0x0ec8;
pub const REG_IQKYB: c_uint = 0x0ecc;
pub const REG_IQKXB: c_uint = 0x0ed4;
pub const REG_INTPO_SETB: c_uint = 0x0ee8;
pub const REG_CRC_CCK: c_uint = 0x0f04;
pub const REG_CCA_OFDM: c_uint = 0x0f08;
pub const REG_CRC_VHT: c_uint = 0x0f0c;
pub const REG_CRC_HT: c_uint = 0x0f10;
pub const REG_CRC_OFDM: c_uint = 0x0f14;
pub const REG_FA_OFDM: c_uint = 0x0f48;
pub const REG_DBGRPT: c_uint = 0x0fa0;
pub const REG_CCA_CCK: c_uint = 0x0fcc;
pub const REG_SYS_CFG3_8814A: c_uint = 0x1000;
pub const REG_ANAPARSW_MAC_0: c_uint = 0x1010;

pub const REG_ANAPAR_XTAL_0: c_uint = 0x1040;

pub const REG_CPU_DMEM_CON: c_uint = 0x1080;

pub const REG_SW_MDIO: c_uint = 0x10C0;
pub const REG_H2C_PKT_READADDR: c_uint = 0x10D0;
pub const REG_H2C_PKT_WRITEADDR: c_uint = 0x10D4;
pub const REG_FW_DBG6: c_uint = 0x10F8;
pub const REG_FW_DBG7: c_uint = 0x10FC;
pub const FW_KEY_MASK: c_uint = 0xffffff00;
pub const REG_CR_EXT: c_uint = 0x1100;
pub const REG_FT1IMR: c_uint = 0x1138;

pub const REG_FT1ISR: c_uint = 0x113c;

pub const REG_DDMA_CH0SA: c_uint = 0x1200;
pub const REG_DDMA_CH0DA: c_uint = 0x1204;
pub const REG_DDMA_CH0CTRL: c_uint = 0x1208;

pub const BIT_MASK_DDMACH0_DLEN: c_uint = 0x3ffff;
pub const REG_H2CQ_CSR: c_uint = 0x1330;

pub const REG_FAST_EDCA_VOVI_SETTING: c_uint = 0x1448;
pub const REG_FAST_EDCA_BEBK_SETTING: c_uint = 0x144C;
pub const REG_RXPSF_CTRL: c_uint = 0x1610;

pub const BIT_SHIFT_RXGCK_VHT_FIFOTHR: c_int = 26;
pub const BIT_MASK_RXGCK_VHT_FIFOTHR: c_uint = 0x3;

pub const BIT_SHIFT_RXGCK_HT_FIFOTHR: c_int = 24;
pub const BIT_MASK_RXGCK_HT_FIFOTHR: c_uint = 0x3;

pub const BIT_SHIFT_RXGCK_OFDM_FIFOTHR: c_int = 22;
pub const BIT_MASK_RXGCK_OFDM_FIFOTHR: c_uint = 0x3;

pub const BIT_SHIFT_RXGCK_CCK_FIFOTHR: c_int = 20;
pub const BIT_MASK_RXGCK_CCK_FIFOTHR: c_uint = 0x3;

pub const BIT_SHIFT_RXPSF_PKTLENTHR: c_int = 13;
pub const BIT_MASK_RXPSF_PKTLENTHR: c_uint = 0x7;

pub const BIT_SHIFT_RXPSF_ERRTHR: c_int = 0;
pub const BIT_MASK_RXPSF_ERRTHR: c_uint = 0x7;

pub const REG_RXPSF_TYPE_CTRL: c_uint = 0x1614;
pub const REG_GENERAL_OPTION: c_uint = 0x1664;

pub const REG_WL2LTECOEX_INDIRECT_ACCESS_CTRL_V1: c_uint = 0x1700;
pub const REG_WL2LTECOEX_INDIRECT_ACCESS_WRITE_DATA_V1: c_uint = 0x1704;
pub const REG_WL2LTECOEX_INDIRECT_ACCESS_READ_DATA_V1: c_uint = 0x1708;

pub const REG_RX_IQC_AB_C: c_uint = 0x1810;
pub const REG_RX_IQC_CD_C: c_uint = 0x1814;
pub const REG_TXSCALE_C: c_uint = 0x181c;
pub const REG_CK_MONHC: c_uint = 0x185c;
pub const REG_AFE_PWR1_C: c_uint = 0x1860;
pub const REG_IGN_GNT_BT1: c_uint = 0x1860;
pub const REG_TX_AGC_C: c_uint = 0x1894;
pub const REG_RFE_PINMUX_C: c_uint = 0x18b4;
pub const REG_RFESEL_CTRL: c_uint = 0x1990;
pub const REG_AGC_TBL: c_uint = 0x1998;
pub const REG_RX_IQC_AB_D: c_uint = 0x1a10;
pub const REG_RX_IQC_CD_D: c_uint = 0x1a14;
pub const REG_TXSCALE_D: c_uint = 0x1a1c;
pub const REG_CK_MONHD: c_uint = 0x1a5c;
pub const REG_AFE_PWR1_D: c_uint = 0x1a60;
pub const REG_TX_AGC_D: c_uint = 0x1a94;
pub const REG_RFE_PINMUX_D: c_uint = 0x1ab4;
pub const REG_RFE_INVSEL_D: c_uint = 0x1abc;

pub const REG_NOMASK_TXBT: c_uint = 0x1ca7;
pub const REG_ANAPAR: c_uint = 0x1c30;

pub const REG_RSTB_SEL: c_uint = 0x1c38;

pub const REG_HRCV_MSG: c_uint = 0x1cf;
pub const REG_EDCCA_REPORT: c_uint = 0x2d38;

pub const REG_IGN_GNTBT4: c_uint = 0x4160;
pub const REG_USB_MOD: c_uint = 0xf008;
pub const REG_USB3_RXITV: c_uint = 0xf050;
pub const REG_USB2_PHY_ADR: c_uint = 0xfe40;
pub const REG_USB2_PHY_DAT: c_uint = 0xfe41;
pub const REG_USB2_PHY_CMD: c_uint = 0xfe42;
pub const BIT_USB2_PHY_CMD_TRG: c_uint = 0x81;
pub const REG_USB_HRPWM: c_uint = 0xfe58;
pub const REG_USB3_PHY_ADR: c_uint = 0xff0c;
pub const REG_USB3_PHY_DAT_L: c_uint = 0xff0d;
pub const REG_USB3_PHY_DAT_H: c_uint = 0xff0e;

pub const RF_MODE: c_uint = 0x00;
pub const RF_MODOPT: c_uint = 0x01;
pub const RF_WLINT: c_uint = 0x01;
pub const RF_WLSEL: c_uint = 0x02;
pub const RF_DTXLOK: c_uint = 0x08;
pub const RF_CFGCH: c_uint = 0x18;

pub const RF_RCK1_V1: c_uint = 0x1c;
pub const RF_RCK: c_uint = 0x1d;
pub const RF_MODE_TABLE_ADDR: c_uint = 0x30;
pub const RF_MODE_TABLE_DATA0: c_uint = 0x31;
pub const RF_MODE_TABLE_DATA1: c_uint = 0x32;
pub const RF_LUTWA: c_uint = 0x33;
pub const RF_LUTWD1: c_uint = 0x3e;
pub const RF_LUTWD0: c_uint = 0x3f;

pub const RF_T_METER: c_uint = 0x42;
pub const RF_BSPAD: c_uint = 0x54;
pub const RF_GAINTX: c_uint = 0x56;
pub const RF_TXMOD: c_uint = 0x58;
pub const RF_TXATANK: c_uint = 0x64;
pub const RF_TXA_PREPAD: c_uint = 0x65;
pub const RF_TRXIQ: c_uint = 0x66;
pub const RF_RXIQGEN: c_uint = 0x8d;
pub const RF_RXBB2: c_uint = 0x8f;
pub const RF_SYN_PFD: c_uint = 0xb0;
pub const RF_LCK: c_uint = 0xb4;
pub const RF_XTALX2: c_uint = 0xb8;
pub const RF_SYN_CTRL: c_uint = 0xbb;
pub const RF_MALSEL: c_uint = 0xbe;
pub const RF_SYN_AAC: c_uint = 0xc9;
pub const RF_AAC_CTRL: c_uint = 0xca;
pub const RF_FAST_LCK: c_uint = 0xcc;
pub const RF_RCKD: c_uint = 0xde;
pub const RF_TXADBG: c_uint = 0xde;
pub const RF_LUTDBG: c_uint = 0xdf;

pub const RF_LUTWE2: c_uint = 0xee;
pub const RF_LUTWE: c_uint = 0xef;
pub const LTE_COEX_CTRL: c_uint = 0x38;
pub const LTE_WL_TRX_CTRL: c_uint = 0xa0;
pub const LTE_BT_TRX_CTRL: c_uint = 0xa4;
