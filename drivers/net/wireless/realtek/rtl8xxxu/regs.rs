//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl8xxxu/regs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2014 - 2017 Jes Sorensen <Jes.Sorensen@gmail.com>
//
// Register definitions taken from original Realtek rtl8723au driver
//
// 0x0000 ~ 0x00FF	System Configuration
pub const REG_SYS_ISO_CTRL: c_uint = 0x0000;

pub const REG_SYS_FUNC: c_uint = 0x0002;

pub const REG_APS_FSMCO: c_uint = 0x0004;

pub const REG_SYS_CLKR: c_uint = 0x0008;

pub const REG_9346CR: c_uint = 0x000a;

pub const REG_EE_VPD: c_uint = 0x000c;
pub const REG_AFE_MISC: c_uint = 0x0010;

pub const REG_SPS0_CTRL: c_uint = 0x0011;
pub const REG_SPS_OCP_CFG: c_uint = 0x0018;
pub const REG_8192E_LDOV12_CTRL: c_uint = 0x0014;
pub const REG_SYS_SWR_CTRL2: c_uint = 0x0014;
pub const REG_RSV_CTRL: c_uint = 0x001c;

pub const REG_RF_CTRL: c_uint = 0x001f;

pub const REG_LDOA15_CTRL: c_uint = 0x0020;

pub const LDOA15_VOADJ_SHIFT: c_int = 4;
pub const REG_LDOV12D_CTRL: c_uint = 0x0021;

pub const LDOV12D_VADJ_SHIFT: c_int = 4;
pub const REG_LDOHCI12_CTRL: c_uint = 0x0022;
pub const REG_LPLDO_CTRL: c_uint = 0x0023;

pub const REG_AFE_XTAL_CTRL: c_uint = 0x0024;

//
// 0x0028 is also known as REG_AFE_CTRL2 on 8723bu/8192eu
//
pub const REG_AFE_PLL_CTRL: c_uint = 0x0028;

pub const REG_MAC_PHY_CTRL: c_uint = 0x002c;
pub const REG_EFUSE_CTRL: c_uint = 0x0030;
pub const REG_EFUSE_TEST: c_uint = 0x0034;

// 00: Wifi Efuse, 01: BT Efuse0, 10: BT Efuse1, 11: BT Efuse2

pub const EFUSE_SELECT_MASK: c_uint = 0x0300;
pub const EFUSE_WIFI_SELECT: c_uint = 0x0000;
pub const EFUSE_BT0_SELECT: c_uint = 0x0100;
pub const EFUSE_BT1_SELECT: c_uint = 0x0200;
pub const EFUSE_BT2_SELECT: c_uint = 0x0300;
pub const EFUSE_ACCESS_ENABLE: c_uint = 0x69	/* RTL8723 only */;
pub const EFUSE_ACCESS_DISABLE: c_uint = 0x00	/* RTL8723 only */;
pub const REG_PWR_DATA: c_uint = 0x0038;

pub const REG_CAL_TIMER: c_uint = 0x003c;
pub const REG_ACLK_MON: c_uint = 0x003e;
pub const REG_GPIO_MUXCFG: c_uint = 0x0040;

pub const REG_GPIO_IO_SEL: c_uint = 0x0042;
pub const REG_MAC_PINMUX_CFG: c_uint = 0x0043;
pub const REG_GPIO_PIN_CTRL: c_uint = 0x0044;
pub const REG_GPIO_INTM: c_uint = 0x0048;

pub const REG_LEDCFG0: c_uint = 0x004c;

pub const LED_MODE_SW_CTRL: c_uint = 0x0;
pub const LED_MODE_TX_OR_RX_EVENTS: c_uint = 0x3;

pub const LED_SW_OFF: c_uint = 0x0;
pub const LED_SW_ON: c_uint = 0x1;

pub const LED_IO_MODE_OUTPUT: c_uint = 0x0;
pub const LED_IO_MODE_INPUT: c_uint = 0x1;

pub const LED_GPIO_DISABLE: c_uint = 0x0;
pub const LED_GPIO_ENABLE: c_uint = 0x1;

pub const REG_LEDCFG1: c_uint = 0x004d;

pub const REG_LEDCFG2: c_uint = 0x004e;

pub const REG_LEDCFG3: c_uint = 0x004f;

pub const REG_FSIMR: c_uint = 0x0050;
pub const REG_FSISR: c_uint = 0x0054;
pub const REG_HSIMR: c_uint = 0x0058;
pub const REG_HSISR: c_uint = 0x005c;
// RTL8723 WIFI/BT/GPS Multi-Function GPIO Pin Control.
pub const REG_GPIO_PIN_CTRL_2: c_uint = 0x0060;
// RTL8723 WIFI/BT/GPS Multi-Function GPIO Select.
pub const REG_GPIO_IO_SEL_2: c_uint = 0x0062;

// RTL8723B
pub const REG_PAD_CTRL1: c_uint = 0x0064;

// RTL8723 only WIFI/BT/GPS Multi-Function control source.
pub const REG_MULTI_FUNC_CTRL: c_uint = 0x0068;

pub const REG_AFE_CTRL4: c_uint = 0x0078	/* 8192eu/8723bu */;
pub const REG_LDO_SW_CTRL: c_uint = 0x007c	/* 8192eu */;
pub const REG_MCU_FW_DL: c_uint = 0x0080;

pub const REG_HMBOX_EXT_0: c_uint = 0x0088;
pub const REG_HMBOX_EXT_1: c_uint = 0x008a;
pub const REG_HMBOX_EXT_2: c_uint = 0x008c;
pub const REG_HMBOX_EXT_3: c_uint = 0x008e;
pub const REG_RSVD_1: c_uint = 0x0097;
// Interrupt registers for 8192e/8723bu/8812
pub const REG_HIMR0: c_uint = 0x00b0;

pub const REG_HISR0: c_uint = 0x00b4;
pub const REG_HIMR1: c_uint = 0x00b8;

pub const REG_HISR1: c_uint = 0x00bc;
// Host suspend counter on FPGA platform
pub const REG_HOST_SUSP_CNT: c_uint = 0x00bc;
// Efuse access protection for RTL8723
pub const REG_EFUSE_ACCESS: c_uint = 0x00cf;
pub const REG_BIST_SCAN: c_uint = 0x00d0;
pub const REG_BIST_RPT: c_uint = 0x00d4;
pub const REG_BIST_ROM_RPT: c_uint = 0x00d8;
pub const REG_RSVD_4: c_uint = 0x00dc;
pub const REG_USB_SIE_INTF: c_uint = 0x00e0;
pub const REG_PCIE_MIO_INTF: c_uint = 0x00e4;
pub const REG_PCIE_MIO_INTD: c_uint = 0x00e8;
pub const REG_HPON_FSM: c_uint = 0x00ec;

pub const REG_SYS_CFG: c_uint = 0x00f0;

pub const SYS_CFG_VENDOR_ID_TSMC: c_int = 0;

pub const SYS_CFG_CHIP_VERSION_MASK: c_uint = 0xf000	/* Bit 12 - 15 */;
pub const REG_GPIO_OUTSTS: c_uint = 0x00f4	/*  For RTL8723 only. */;

pub const REG_SYS_CFG2: c_uint = 0x00fc	/* 8192eu */;
// 0x0100 ~ 0x01FF	MACTOP General Configuration
pub const REG_CR: c_uint = 0x0100;

// Media Status Register
pub const REG_MSR: c_uint = 0x0102;
pub const MSR_LINKTYPE_MASK: c_uint = 0x3;
pub const MSR_LINKTYPE_NONE: c_uint = 0x0;
pub const MSR_LINKTYPE_ADHOC: c_uint = 0x1;
pub const MSR_LINKTYPE_STATION: c_uint = 0x2;
pub const MSR_LINKTYPE_AP: c_uint = 0x3;
pub const REG_PBP: c_uint = 0x0104;
pub const PBP_PAGE_SIZE_RX_SHIFT: c_int = 0;
pub const PBP_PAGE_SIZE_TX_SHIFT: c_int = 4;
pub const PBP_PAGE_SIZE_64: c_uint = 0x0;
pub const PBP_PAGE_SIZE_128: c_uint = 0x1;
pub const PBP_PAGE_SIZE_256: c_uint = 0x2;
pub const PBP_PAGE_SIZE_512: c_uint = 0x3;
pub const PBP_PAGE_SIZE_1024: c_uint = 0x4;
// 8188eu IOL magic
pub const REG_PKT_BUF_ACCESS_CTRL: c_uint = 0x0106;
pub const PKT_BUF_ACCESS_CTRL_TX: c_uint = 0x69;
pub const PKT_BUF_ACCESS_CTRL_RX: c_uint = 0xa5;
pub const REG_TRXDMA_CTRL: c_uint = 0x010c;

pub const TRXDMA_CTRL_VOQ_SHIFT: c_int = 4;
pub const TRXDMA_CTRL_VIQ_SHIFT: c_int = 6;
pub const TRXDMA_CTRL_BEQ_SHIFT: c_int = 8;
pub const TRXDMA_CTRL_BKQ_SHIFT: c_int = 10;
pub const TRXDMA_CTRL_MGQ_SHIFT: c_int = 12;
pub const TRXDMA_CTRL_HIQ_SHIFT: c_int = 14;
pub const TRXDMA_CTRL_VOQ_SHIFT_8192F: c_int = 4;
pub const TRXDMA_CTRL_VIQ_SHIFT_8192F: c_int = 7;
pub const TRXDMA_CTRL_BEQ_SHIFT_8192F: c_int = 10;
pub const TRXDMA_CTRL_BKQ_SHIFT_8192F: c_int = 13;
pub const TRXDMA_CTRL_MGQ_SHIFT_8192F: c_int = 16;
pub const TRXDMA_CTRL_HIQ_SHIFT_8192F: c_int = 19;
pub const TRXDMA_QUEUE_LOW: c_int = 1;
pub const TRXDMA_QUEUE_NORMAL: c_int = 2;
pub const TRXDMA_QUEUE_HIGH: c_int = 3;
pub const REG_TRXFF_BNDY: c_uint = 0x0114;
pub const REG_TRXFF_STATUS: c_uint = 0x0118;
pub const REG_RXFF_PTR: c_uint = 0x011c;
pub const REG_HIMR: c_uint = 0x0120;
pub const REG_HISR: c_uint = 0x0124;
pub const REG_HIMRE: c_uint = 0x0128;
pub const REG_HISRE: c_uint = 0x012c;
pub const REG_CPWM: c_uint = 0x012f;
pub const REG_FWIMR: c_uint = 0x0130;
pub const REG_FWISR: c_uint = 0x0134;
pub const REG_FTIMR: c_uint = 0x0138;
pub const REG_PKTBUF_DBG_CTRL: c_uint = 0x0140;
pub const REG_PKTBUF_DBG_DATA_L: c_uint = 0x0144;
pub const REG_PKTBUF_DBG_DATA_H: c_uint = 0x0148;
pub const REG_TC0_CTRL: c_uint = 0x0150;
pub const REG_TC1_CTRL: c_uint = 0x0154;
pub const REG_TC2_CTRL: c_uint = 0x0158;
pub const REG_TC3_CTRL: c_uint = 0x015c;
pub const REG_TC4_CTRL: c_uint = 0x0160;
pub const REG_TCUNIT_BASE: c_uint = 0x0164;
pub const REG_MBIST_START: c_uint = 0x0174;
pub const REG_MBIST_DONE: c_uint = 0x0178;
pub const REG_MBIST_FAIL: c_uint = 0x017c;
// 8188EU
pub const REG_32K_CTRL: c_uint = 0x0194;
pub const REG_C2HEVT_MSG_NORMAL: c_uint = 0x01a0;
pub const C2H_HW_FEATURE_REPORT: c_uint = 0x19;
pub const C2H_HW_FEATURE_DUMP: c_uint = 0xfd;
// 8192EU/8723BU/8812
pub const REG_C2HEVT_CMD_ID_8723B: c_uint = 0x01ae;
pub const REG_C2HEVT_CLEAR: c_uint = 0x01af;
pub const REG_C2HEVT_MSG_TEST: c_uint = 0x01b8;
pub const REG_MCUTST_1: c_uint = 0x01c0;
pub const REG_FMTHR: c_uint = 0x01c8;
pub const REG_HMTFR: c_uint = 0x01cc;
pub const REG_HMBOX_0: c_uint = 0x01d0;
pub const REG_HMBOX_1: c_uint = 0x01d4;
pub const REG_HMBOX_2: c_uint = 0x01d8;
pub const REG_HMBOX_3: c_uint = 0x01dc;
pub const REG_LLT_INIT: c_uint = 0x01e0;
pub const LLT_OP_INACTIVE: c_uint = 0x0;

pub const REG_BB_ACCESS_CTRL: c_uint = 0x01e8;
pub const REG_BB_ACCESS_DATA: c_uint = 0x01ec;
pub const REG_HMBOX_EXT0_8723B: c_uint = 0x01f0;
pub const REG_HMBOX_EXT1_8723B: c_uint = 0x01f4;
pub const REG_HMBOX_EXT2_8723B: c_uint = 0x01f8;
pub const REG_HMBOX_EXT3_8723B: c_uint = 0x01fc;
// 0x0200 ~ 0x027F	TXDMA Configuration
pub const REG_RQPN: c_uint = 0x0200;
pub const RQPN_HI_PQ_SHIFT: c_int = 0;
pub const RQPN_LO_PQ_SHIFT: c_int = 8;
pub const RQPN_PUB_PQ_SHIFT: c_int = 16;

pub const REG_FIFOPAGE: c_uint = 0x0204;
pub const REG_TDECTRL: c_uint = 0x0208;

pub const REG_TXDMA_OFFSET_CHK: c_uint = 0x020c;

pub const REG_TXDMA_STATUS: c_uint = 0x0210;
pub const REG_RQPN_NPQ: c_uint = 0x0214;
pub const RQPN_NPQ_SHIFT: c_int = 0;
pub const RQPN_EPQ_SHIFT: c_int = 16;
pub const REG_AUTO_LLT: c_uint = 0x0224;

pub const REG_DWBCN1_CTRL_8723B: c_uint = 0x0228;

// 0x0280 ~ 0x02FF	RXDMA Configuration
pub const REG_RXDMA_AGG_PG_TH: c_uint = 0x0280	/* 0-7 : USB DMA size bits;

pub const REG_RXPKT_NUM: c_uint = 0x0284;

pub const REG_RXDMA_STATUS: c_uint = 0x0288;
// Presumably only found on newer chips such as 8723bu
pub const REG_RX_DMA_CTRL_8723B: c_uint = 0x0286;
pub const REG_RXDMA_PRO_8723B: c_uint = 0x0290;

pub const REG_EARLY_MODE_CONTROL_8710B: c_uint = 0x02bc;
pub const REG_RF_BB_CMD_ADDR: c_uint = 0x02c0;
pub const REG_RF_BB_CMD_DATA: c_uint = 0x02c4;
// spec version 11
// 0x0400 ~ 0x047F	Protocol Configuration
// 8192c, 8192d
pub const REG_VOQ_INFO: c_uint = 0x0400;
pub const REG_VIQ_INFO: c_uint = 0x0404;
pub const REG_BEQ_INFO: c_uint = 0x0408;
pub const REG_BKQ_INFO: c_uint = 0x040c;
// 8188e, 8723a, 8812a, 8821a, 8192e, 8723b
pub const REG_Q0_INFO: c_uint = 0x400;
pub const REG_Q1_INFO: c_uint = 0x404;
pub const REG_Q2_INFO: c_uint = 0x408;
pub const REG_Q3_INFO: c_uint = 0x40c;
pub const REG_MGQ_INFO: c_uint = 0x0410;
pub const REG_HGQ_INFO: c_uint = 0x0414;
pub const REG_BCNQ_INFO: c_uint = 0x0418;
pub const REG_CPU_MGQ_INFORMATION: c_uint = 0x041c;
pub const REG_FWHW_TXQ_CTRL: c_uint = 0x0420;

pub const REG_HWSEQ_CTRL: c_uint = 0x0423;
pub const REG_TXPKTBUF_BCNQ_BDNY: c_uint = 0x0424;
pub const REG_TXPKTBUF_MGQ_BDNY: c_uint = 0x0425;
pub const REG_LIFETIME_EN: c_uint = 0x0426;
pub const REG_MULTI_BCNQ_OFFSET: c_uint = 0x0427;
pub const REG_SPEC_SIFS: c_uint = 0x0428;
pub const SPEC_SIFS_CCK_MASK: c_uint = 0x00ff;
pub const SPEC_SIFS_CCK_SHIFT: c_int = 0;
pub const SPEC_SIFS_OFDM_MASK: c_uint = 0xff00;
pub const SPEC_SIFS_OFDM_SHIFT: c_int = 8;
pub const REG_RETRY_LIMIT: c_uint = 0x042a;
pub const RETRY_LIMIT_LONG_SHIFT: c_int = 0;
pub const RETRY_LIMIT_LONG_MASK: c_uint = 0x003f;
pub const RETRY_LIMIT_SHORT_SHIFT: c_int = 8;
pub const RETRY_LIMIT_SHORT_MASK: c_uint = 0x3f00;
pub const REG_DARFRC: c_uint = 0x0430;
pub const REG_RARFRC: c_uint = 0x0438;
pub const REG_RESPONSE_RATE_SET: c_uint = 0x0440;
pub const RESPONSE_RATE_BITMAP_ALL: c_uint = 0xfffff;
pub const RESPONSE_RATE_RRSR_CCK_ONLY_1M: c_uint = 0xffff1;
pub const RESPONSE_RATE_RRSR_INIT_2G: c_uint = 0x15f;
pub const RESPONSE_RATE_RRSR_INIT_5G: c_uint = 0x150;

pub const REG_ARFR0: c_uint = 0x0444;
pub const REG_ARFR1: c_uint = 0x0448;
pub const REG_ARFR2: c_uint = 0x044c;
pub const REG_ARFR3: c_uint = 0x0450;
pub const REG_CCK_CHECK: c_uint = 0x0454;

pub const REG_AMPDU_MAX_TIME_8723B: c_uint = 0x0456;
pub const REG_AGGLEN_LMT: c_uint = 0x0458;
pub const REG_AMPDU_MIN_SPACE: c_uint = 0x045c;
pub const REG_TXPKTBUF_WMAC_LBK_BF_HD: c_uint = 0x045d;
pub const REG_FAST_EDCA_CTRL: c_uint = 0x0460;
pub const REG_RD_RESP_PKT_TH: c_uint = 0x0463;
pub const REG_INIRTS_RATE_SEL: c_uint = 0x0480;
// 8723bu
pub const REG_DATA_SUBCHANNEL: c_uint = 0x0483;
// 8723au
pub const REG_INIDATA_RATE_SEL: c_uint = 0x0484;
// MACID_SLEEP_1/3 for 8723b, 8192e, 8812a, 8821a
pub const REG_MACID_SLEEP_3_8732B: c_uint = 0x0484;
pub const REG_MACID_SLEEP_1_8732B: c_uint = 0x0488;
pub const REG_POWER_STATUS: c_uint = 0x04a4;
pub const REG_POWER_STAGE1: c_uint = 0x04b4;
pub const REG_POWER_STAGE2: c_uint = 0x04b8;
pub const REG_AMPDU_BURST_MODE_8723B: c_uint = 0x04bc;
pub const REG_PKT_VO_VI_LIFE_TIME: c_uint = 0x04c0;
pub const REG_PKT_BE_BK_LIFE_TIME: c_uint = 0x04c2;
pub const REG_STBC_SETTING: c_uint = 0x04c4;
pub const REG_QUEUE_CTRL: c_uint = 0x04c6;
pub const REG_HT_SINGLE_AMPDU_8723B: c_uint = 0x04c7;

pub const REG_PROT_MODE_CTRL: c_uint = 0x04c8;
pub const REG_MAX_AGGR_NUM: c_uint = 0x04ca;
pub const REG_RTS_MAX_AGGR_NUM: c_uint = 0x04cb;
pub const REG_BAR_MODE_CTRL: c_uint = 0x04cc;
pub const REG_RA_TRY_RATE_AGG_LMT: c_uint = 0x04cf;
// MACID_DROP for 8723a
pub const REG_MACID_DROP_8732A: c_uint = 0x04d0;
// EARLY_MODE_CONTROL 8188e
pub const REG_EARLY_MODE_CONTROL_8188E: c_uint = 0x04d0;
// MACID_SLEEP_2 for 8723b, 8192e, 8812a, 8821a
pub const REG_MACID_SLEEP_2_8732B: c_uint = 0x04d0;
pub const REG_MACID_SLEEP: c_uint = 0x04d4;
pub const REG_NQOS_SEQ: c_uint = 0x04dc;
pub const REG_QOS_SEQ: c_uint = 0x04de;
pub const REG_NEED_CPU_HANDLE: c_uint = 0x04e0;
pub const REG_PKT_LOSE_RPT: c_uint = 0x04e1;
pub const REG_PTCL_ERR_STATUS: c_uint = 0x04e2;
pub const REG_TX_REPORT_CTRL: c_uint = 0x04ec;

pub const REG_TX_REPORT_TIME: c_uint = 0x04f0;
pub const REG_DUMMY: c_uint = 0x04fc;
// 0x0500 ~ 0x05FF	EDCA Configuration
pub const REG_EDCA_VO_PARAM: c_uint = 0x0500;
pub const REG_EDCA_VI_PARAM: c_uint = 0x0504;
pub const REG_EDCA_BE_PARAM: c_uint = 0x0508;
pub const REG_EDCA_BK_PARAM: c_uint = 0x050c;
pub const EDCA_PARAM_ECW_MIN_SHIFT: c_int = 8;
pub const EDCA_PARAM_ECW_MAX_SHIFT: c_int = 12;
pub const EDCA_PARAM_TXOP_SHIFT: c_int = 16;
pub const REG_BEACON_TCFG: c_uint = 0x0510;
pub const REG_PIFS: c_uint = 0x0512;
pub const REG_RDG_PIFS: c_uint = 0x0513;
pub const REG_SIFS_CCK: c_uint = 0x0514;
pub const REG_SIFS_OFDM: c_uint = 0x0516;
pub const REG_TSFTR_SYN_OFFSET: c_uint = 0x0518;
pub const REG_AGGR_BREAK_TIME: c_uint = 0x051a;
pub const REG_SLOT: c_uint = 0x051b;
pub const REG_TX_PTCL_CTRL: c_uint = 0x0520;
pub const REG_TXPAUSE: c_uint = 0x0522;
pub const REG_DIS_TXREQ_CLR: c_uint = 0x0523;
pub const REG_RD_CTRL: c_uint = 0x0524;
pub const REG_TBTT_PROHIBIT: c_uint = 0x0540;
pub const REG_RD_NAV_NXT: c_uint = 0x0544;
pub const REG_NAV_PROT_LEN: c_uint = 0x0546;
pub const REG_BEACON_CTRL: c_uint = 0x0550;
pub const REG_BEACON_CTRL_1: c_uint = 0x0551;

pub const REG_MBID_NUM: c_uint = 0x0552;
pub const REG_DUAL_TSF_RST: c_uint = 0x0553;

// The same as REG_MBSSID_BCN_SPACE
pub const REG_BCN_INTERVAL: c_uint = 0x0554;
pub const REG_MBSSID_BCN_SPACE: c_uint = 0x0554;
pub const REG_DRIVER_EARLY_INT: c_uint = 0x0558;
pub const DRIVER_EARLY_INT_TIME: c_int = 5;
pub const REG_BEACON_DMA_TIME: c_uint = 0x0559;
pub const BEACON_DMA_ATIME_INT_TIME: c_int = 2;
pub const REG_ATIMWND: c_uint = 0x055a;
pub const REG_USTIME_TSF_8723B: c_uint = 0x055c;
pub const REG_BCN_MAX_ERR: c_uint = 0x055d;
pub const REG_RXTSF_OFFSET_CCK: c_uint = 0x055e;
pub const REG_RXTSF_OFFSET_OFDM: c_uint = 0x055f;
pub const REG_TSFTR: c_uint = 0x0560;
pub const REG_TSFTR1: c_uint = 0x0568;
pub const REG_INIT_TSFTR: c_uint = 0x0564;
pub const REG_ATIMWND_1: c_uint = 0x0570;
pub const REG_PSTIMER: c_uint = 0x0580;
pub const REG_TIMER0: c_uint = 0x0584;
pub const REG_TIMER1: c_uint = 0x0588;
pub const REG_ACM_HW_CTRL: c_uint = 0x05c0;

pub const REG_ACM_RST_CTRL: c_uint = 0x05c1;
pub const REG_ACMAVG: c_uint = 0x05c2;
pub const REG_VO_ADMTIME: c_uint = 0x05c4;
pub const REG_VI_ADMTIME: c_uint = 0x05c6;
pub const REG_BE_ADMTIME: c_uint = 0x05c8;
pub const REG_EDCA_RANDOM_GEN: c_uint = 0x05cc;
pub const REG_SCH_TXCMD: c_uint = 0x05d0;
// define REG_FW_TSF_SYNC_CNT		0x04a0
pub const REG_SCH_TX_CMD: c_uint = 0x05f8;
pub const REG_FW_RESET_TSF_CNT_1: c_uint = 0x05fc;
pub const REG_FW_RESET_TSF_CNT_0: c_uint = 0x05fd;
pub const REG_FW_BCN_DIS_CNT: c_uint = 0x05fe;
// 0x0600 ~ 0x07FF  WMAC Configuration
pub const REG_APSD_CTRL: c_uint = 0x0600;

pub const REG_BW_OPMODE: c_uint = 0x0603;

pub const REG_TCR: c_uint = 0x0604;
// Receive Configuration Register
pub const REG_RCR: c_uint = 0x0608;

pub const REG_RX_PKT_LIMIT: c_uint = 0x060c;
pub const REG_RX_DLK_TIME: c_uint = 0x060d;
pub const REG_RX_DRVINFO_SZ: c_uint = 0x060f;
pub const REG_MACID: c_uint = 0x0610;
pub const REG_BSSID: c_uint = 0x0618;
pub const REG_MAR: c_uint = 0x0620;
pub const REG_MBIDCAMCFG: c_uint = 0x0628;
pub const REG_USTIME_EDCA: c_uint = 0x0638;
pub const REG_MAC_SPEC_SIFS: c_uint = 0x063a;
// 20100719 Joseph: Hardware register definition change. (HW datasheet v54)
// [15:8]SIFS_R2T_OFDM, [7:0]SIFS_R2T_CCK
pub const REG_R2T_SIFS: c_uint = 0x063c;
// [15:8]SIFS_T2T_OFDM, [7:0]SIFS_T2T_CCK
pub const REG_T2T_SIFS: c_uint = 0x063e;
pub const REG_ACKTO: c_uint = 0x0640;
pub const REG_CTS2TO: c_uint = 0x0641;
pub const REG_EIFS: c_uint = 0x0642;
// WMA, BA, CCX
pub const REG_NAV_CTRL: c_uint = 0x0650;
// In units of 128us
pub const REG_NAV_UPPER: c_uint = 0x0652;
pub const NAV_UPPER_UNIT: c_int = 128;
pub const REG_BACAMCMD: c_uint = 0x0654;
pub const REG_BACAMCONTENT: c_uint = 0x0658;
pub const REG_LBDLY: c_uint = 0x0660;
pub const REG_FWDLY: c_uint = 0x0661;
pub const REG_RXERR_RPT: c_uint = 0x0664;
pub const REG_WMAC_TRXPTCL_CTL: c_uint = 0x0668;

pub const WMAC_TRXPTCL_CTL_BW_20: c_int = 0;

// Security
pub const REG_CAM_CMD: c_uint = 0x0670;

pub const CAM_CMD_KEY_SHIFT: c_int = 3;
pub const REG_CAM_WRITE: c_uint = 0x0674;

pub const REG_CAM_READ: c_uint = 0x0678;
pub const REG_CAM_DEBUG: c_uint = 0x067c;
pub const REG_SECURITY_CFG: c_uint = 0x0680;

// Power
pub const REG_WOW_CTRL: c_uint = 0x0690;
pub const REG_PSSTATUS: c_uint = 0x0691;
pub const REG_PS_RX_INFO: c_uint = 0x0692;
pub const REG_LPNAV_CTRL: c_uint = 0x0694;
pub const REG_WKFMCAM_CMD: c_uint = 0x0698;
pub const REG_WKFMCAM_RWD: c_uint = 0x069c;
//
// RX Filters: each bit corresponds to the numerical value of the subtype.
// If it is set the subtype frame type is passed. The filter is only used when
// the RCR_ACCEPT_DATA_FRAME, RCR_ACCEPT_CTRL_FRAME, RCR_ACCEPT_MGMT_FRAME bit
// in the RCR are low.
//
// Example: Beacon subtype is binary 1000 which is decimal 8 so we have to set
// bit 8 (0x100) in REG_RXFLTMAP0 to enable reception.
//
pub const REG_RXFLTMAP0: c_uint = 0x06a0	/* Management frames */;
pub const REG_RXFLTMAP1: c_uint = 0x06a2	/* Control frames */;
pub const REG_RXFLTMAP2: c_uint = 0x06a4	/* Data frames */;
pub const REG_BCN_PSR_RPT: c_uint = 0x06a8;
pub const REG_CALB32K_CTRL: c_uint = 0x06ac;
pub const REG_PKT_MON_CTRL: c_uint = 0x06b4;
pub const REG_BT_COEX_TABLE1: c_uint = 0x06c0;
pub const REG_BT_COEX_TABLE2: c_uint = 0x06c4;
pub const REG_BT_COEX_TABLE3: c_uint = 0x06c8;
pub const REG_BT_COEX_TABLE4: c_uint = 0x06cc;
pub const REG_WMAC_RESP_TXINFO: c_uint = 0x06d8;
pub const REG_MACID1: c_uint = 0x0700;
pub const REG_BSSID1: c_uint = 0x0708;
//
// This seems to be 8723bu specific
//
pub const REG_BT_CONTROL_8723BU: c_uint = 0x0764;

pub const REG_PORT_CONTROL_8710B: c_uint = 0x076d;
pub const REG_WLAN_ACT_CONTROL_8723B: c_uint = 0x076e;
pub const REG_FPGA0_RF_MODE: c_uint = 0x0800;

pub const REG_FPGA0_TX_INFO: c_uint = 0x0804;

pub const REG_FPGA0_PSD_FUNC: c_uint = 0x0808;
pub const REG_FPGA0_TX_GAIN: c_uint = 0x080c;
pub const REG_FPGA0_RF_TIMING1: c_uint = 0x0810;
pub const REG_FPGA0_RF_TIMING2: c_uint = 0x0814;
pub const REG_FPGA0_POWER_SAVE: c_uint = 0x0818;

pub const REG_FPGA0_XA_HSSI_PARM1: c_uint = 0x0820	/* RF 3 wire register */;

pub const REG_FPGA0_XA_HSSI_PARM2: c_uint = 0x0824;
pub const REG_FPGA0_XB_HSSI_PARM1: c_uint = 0x0828;
pub const REG_FPGA0_XB_HSSI_PARM2: c_uint = 0x082c;
pub const FPGA0_HSSI_3WIRE_DATA_LEN: c_uint = 0x800;
pub const FPGA0_HSSI_3WIRE_ADDR_LEN: c_uint = 0x400;
pub const FPGA0_HSSI_PARM2_ADDR_SHIFT: c_int = 23;
pub const FPGA0_HSSI_PARM2_ADDR_MASK: c_uint = 0x7f800000	/* 0xff << 23 */;

pub const REG_TX_AGC_B_RATE18_06: c_uint = 0x0830;
pub const REG_TX_AGC_B_RATE54_24: c_uint = 0x0834;
pub const REG_TX_AGC_B_CCK1_55_MCS32: c_uint = 0x0838;
pub const REG_TX_AGC_B_MCS03_MCS00: c_uint = 0x083c;
pub const REG_FPGA0_XA_LSSI_PARM: c_uint = 0x0840;
pub const REG_FPGA0_XB_LSSI_PARM: c_uint = 0x0844;
pub const FPGA0_LSSI_PARM_ADDR_SHIFT: c_int = 20;
pub const FPGA0_LSSI_PARM_ADDR_MASK: c_uint = 0x0ff00000;
pub const FPGA0_LSSI_PARM_DATA_MASK: c_uint = 0x000fffff;
pub const REG_TX_AGC_B_MCS07_MCS04: c_uint = 0x0848;
pub const REG_TX_AGC_B_MCS11_MCS08: c_uint = 0x084c;
pub const REG_FPGA0_XCD_SWITCH_CTRL: c_uint = 0x085c;
pub const REG_FPGA0_XA_RF_INT_OE: c_uint = 0x0860	/* RF Channel switch */;
pub const REG_FPGA0_XB_RF_INT_OE: c_uint = 0x0864;
pub const FPGA0_INT_OE_ANTENNA_AB_OPEN: c_uint = 0x000;

pub const REG_TX_AGC_B_MCS15_MCS12: c_uint = 0x0868;
pub const REG_TX_AGC_B_CCK11_A_CCK2_11: c_uint = 0x086c;
pub const REG_FPGA0_XAB_RF_SW_CTRL: c_uint = 0x0870;
pub const REG_FPGA0_XA_RF_SW_CTRL: c_uint = 0x0870	/* 16 bit */;
pub const REG_FPGA0_XB_RF_SW_CTRL: c_uint = 0x0872	/* 16 bit */;
pub const REG_FPGA0_XCD_RF_SW_CTRL: c_uint = 0x0874;
pub const REG_FPGA0_XC_RF_SW_CTRL: c_uint = 0x0874	/* 16 bit */;
pub const REG_FPGA0_XD_RF_SW_CTRL: c_uint = 0x0876	/* 16 bit */;

pub const FPGA0_RF_3WIRE_MASK: c_uint = 0xf;

pub const FPGA0_RF_BD_CTRL_SHIFT: c_int = 16;
pub const REG_FPGA0_XAB_RF_PARM: c_uint = 0x0878	/* Antenna select path in ODM */;
pub const REG_FPGA0_XA_RF_PARM: c_uint = 0x0878	/* 16 bit */;
pub const REG_FPGA0_XB_RF_PARM: c_uint = 0x087a	/* 16 bit */;
pub const REG_FPGA0_XCD_RF_PARM: c_uint = 0x087c;
pub const REG_FPGA0_XC_RF_PARM: c_uint = 0x087c	/* 16 bit */;
pub const REG_FPGA0_XD_RF_PARM: c_uint = 0x087e	/* 16 bit */;

pub const REG_FPGA0_ANALOG1: c_uint = 0x0880;
pub const REG_FPGA0_ANALOG2: c_uint = 0x0884;

pub const REG_FPGA0_ANALOG3: c_uint = 0x0888;
pub const REG_FPGA0_ANALOG4: c_uint = 0x088c;
pub const REG_NHM_TH9_TH10_8723B: c_uint = 0x0890;
pub const REG_NHM_TIMER_8723B: c_uint = 0x0894;
pub const REG_NHM_TH3_TO_TH0_8723B: c_uint = 0x0898;
pub const REG_NHM_TH7_TO_TH4_8723B: c_uint = 0x089c;
pub const REG_FPGA0_XA_LSSI_READBACK: c_uint = 0x08a0	/* Tranceiver LSSI Readback */;
pub const REG_FPGA0_XB_LSSI_READBACK: c_uint = 0x08a4;
pub const REG_FPGA0_PSD_REPORT: c_uint = 0x08b4;
pub const REG_HSPI_XA_READBACK: c_uint = 0x08b8	/* Transceiver A HSPI read */;
pub const REG_HSPI_XB_READBACK: c_uint = 0x08bc	/* Transceiver B HSPI read */;
pub const REG_FPGA1_RF_MODE: c_uint = 0x0900;
pub const REG_FPGA1_TX_INFO: c_uint = 0x090c;
pub const FPGA1_TX_ANT_MASK: c_uint = 0x0000000f;
pub const FPGA1_TX_ANT_L_MASK: c_uint = 0x000000f0;
pub const FPGA1_TX_ANT_NON_HT_MASK: c_uint = 0x00000f00;
pub const FPGA1_TX_ANT_HT1_MASK: c_uint = 0x0000f000;
pub const FPGA1_TX_ANT_HT2_MASK: c_uint = 0x000f0000;
pub const FPGA1_TX_ANT_HT_S1_MASK: c_uint = 0x00f00000;
pub const FPGA1_TX_ANT_NON_HT_S1_MASK: c_uint = 0x0f000000;
pub const FPGA1_TX_OFDM_TXSC_MASK: c_uint = 0x30000000;
pub const REG_ANT_MAPPING1: c_uint = 0x0914;
pub const REG_RFE_OPT: c_uint = 0x0920;
pub const REG_DPDT_CTRL: c_uint = 0x092c	/* 8723BU */;
pub const REG_RFE_CTRL_ANTA_SRC: c_uint = 0x0930	/* 8723BU */;
pub const REG_RFE_CTRL_ANT_SRC1: c_uint = 0x0934;
pub const REG_RFE_CTRL_ANT_SRC2: c_uint = 0x0938;
pub const REG_RFE_CTRL_ANT_SRC3: c_uint = 0x093c;
pub const REG_RFE_PATH_SELECT: c_uint = 0x0940	/* 8723BU */;
pub const REG_RFE_BUFFER: c_uint = 0x0944	/* 8723BU */;
pub const REG_S0S1_PATH_SWITCH: c_uint = 0x0948	/* 8723BU */;
pub const REG_RX_DFIR_MOD_97F: c_uint = 0x0948;
pub const REG_OFDM_RX_DFIR: c_uint = 0x954;
pub const REG_RFE_OPT62: c_uint = 0x0968;
pub const REG_CCK0_SYSTEM: c_uint = 0x0a00;

pub const REG_CCK0_AFE_SETTING: c_uint = 0x0a04;
pub const CCK0_AFE_RX_MASK: c_uint = 0x0f000000;
pub const CCK0_AFE_TX_MASK: c_uint = 0xf0000000;
pub const CCK0_AFE_RX_ANT_A: c_int = 0;

pub const CCK0_AFE_RX_ANT_OPTION_A: c_int = 0;

pub const REG_CCK_ANTDIV_PARA2: c_uint = 0x0a04;
pub const REG_BB_POWER_SAVE4: c_uint = 0x0a74;
// 8188eu
pub const REG_LNA_SWITCH: c_uint = 0x0b2c;

pub const REG_CCK_PD_THRESH: c_uint = 0x0a0a;
pub const CCK_PD_TYPE1_LV0_TH: c_uint = 0x40;
pub const CCK_PD_TYPE1_LV1_TH: c_uint = 0x83;
pub const CCK_PD_TYPE1_LV2_TH: c_uint = 0xcd;
pub const CCK_PD_TYPE1_LV3_TH: c_uint = 0xdd;
pub const CCK_PD_TYPE1_LV4_TH: c_uint = 0xed;
pub const REG_CCK0_TX_FILTER1: c_uint = 0x0a20;
pub const REG_CCK0_TX_FILTER2: c_uint = 0x0a24;
pub const REG_CCK0_DEBUG_PORT: c_uint = 0x0a28	/* debug port and Tx filter3 */;
pub const REG_AGC_RPT: c_uint = 0xa80;

pub const REG_CCK0_TX_FILTER3: c_uint = 0x0aac;
pub const REG_CONFIG_ANT_A: c_uint = 0x0b68;
pub const REG_CONFIG_ANT_B: c_uint = 0x0b6c;
pub const REG_OFDM0_TRX_PATH_ENABLE: c_uint = 0x0c04;
pub const OFDM_RF_PATH_RX_MASK: c_uint = 0x0f;

pub const OFDM_RF_PATH_TX_MASK: c_uint = 0xf0;

pub const REG_OFDM0_TR_MUX_PAR: c_uint = 0x0c08;
pub const REG_OFDM0_FA_RSTC: c_uint = 0x0c0c;
pub const REG_DOWNSAM_FACTOR: c_uint = 0x0c10;
pub const REG_OFDM0_XA_RX_AFE: c_uint = 0x0c10;
pub const REG_OFDM0_XA_RX_IQ_IMBALANCE: c_uint = 0x0c14;
pub const REG_OFDM0_XB_RX_IQ_IMBALANCE: c_uint = 0x0c1c;
pub const REG_OFDM0_ENERGY_CCA_THRES: c_uint = 0x0c4c;
pub const REG_OFDM0_RX_D_SYNC_PATH: c_uint = 0x0c40;

pub const REG_OFDM0_XA_AGC_CORE1: c_uint = 0x0c50;
pub const REG_OFDM0_XA_AGC_CORE2: c_uint = 0x0c54;
pub const REG_OFDM0_XB_AGC_CORE1: c_uint = 0x0c58;
pub const REG_OFDM0_XB_AGC_CORE2: c_uint = 0x0c5c;
pub const REG_OFDM0_XC_AGC_CORE1: c_uint = 0x0c60;
pub const REG_OFDM0_XC_AGC_CORE2: c_uint = 0x0c64;
pub const REG_OFDM0_XD_AGC_CORE1: c_uint = 0x0c68;
pub const REG_OFDM0_XD_AGC_CORE2: c_uint = 0x0c6c;
pub const OFDM0_X_AGC_CORE1_IGI_MASK: c_uint = 0x0000007F;
pub const REG_OFDM0_AGC_PARM1: c_uint = 0x0c70;
pub const REG_OFDM0_AGC_RSSI_TABLE: c_uint = 0x0c78;
pub const REG_OFDM0_XA_TX_IQ_IMBALANCE: c_uint = 0x0c80;
pub const REG_OFDM0_XB_TX_IQ_IMBALANCE: c_uint = 0x0c88;
pub const REG_OFDM0_XC_TX_IQ_IMBALANCE: c_uint = 0x0c90;
pub const REG_OFDM0_XD_TX_IQ_IMBALANCE: c_uint = 0x0c98;
pub const REG_OFDM0_XC_TX_AFE: c_uint = 0x0c94;
pub const REG_OFDM0_XD_TX_AFE: c_uint = 0x0c9c;
pub const REG_OFDM0_RX_IQ_EXT_ANTA: c_uint = 0x0ca0;
// 8188eu
pub const REG_ANTDIV_PARA1: c_uint = 0x0ca4;
pub const REG_RXIQB_EXT: c_uint = 0x0ca8;
// 8723bu
pub const REG_OFDM0_TX_PSDO_NOISE_WEIGHT: c_uint = 0x0ce4;
pub const REG_OFDM1_LSTF: c_uint = 0x0d00;

pub const OFDM_LSTF_MASK: c_uint = 0x70000000;
pub const REG_OFDM1_TRX_PATH_ENABLE: c_uint = 0x0d04;
pub const REG_OFDM1_CFO_TRACKING: c_uint = 0x0d2c;

pub const REG_OFDM1_CSI_FIX_MASK1: c_uint = 0x0d40;
pub const REG_OFDM1_CSI_FIX_MASK2: c_uint = 0x0d44;
pub const REG_ANAPWR1: c_uint = 0x0d94;
pub const REG_TX_AGC_A_RATE18_06: c_uint = 0x0e00;
pub const REG_TX_AGC_A_RATE54_24: c_uint = 0x0e04;
pub const REG_TX_AGC_A_CCK1_MCS32: c_uint = 0x0e08;
pub const REG_TX_AGC_A_MCS03_MCS00: c_uint = 0x0e10;
pub const REG_TX_AGC_A_MCS07_MCS04: c_uint = 0x0e14;
pub const REG_TX_AGC_A_MCS11_MCS08: c_uint = 0x0e18;
pub const REG_TX_AGC_A_MCS15_MCS12: c_uint = 0x0e1c;
pub const REG_NP_ANTA: c_uint = 0x0e20;
pub const REG_TAP_UPD_97F: c_uint = 0x0e24;
pub const REG_FPGA0_IQK: c_uint = 0x0e28;
pub const REG_TX_IQK_TONE_A: c_uint = 0x0e30;
pub const REG_RX_IQK_TONE_A: c_uint = 0x0e34;
pub const REG_TX_IQK_PI_A: c_uint = 0x0e38;
pub const REG_RX_IQK_PI_A: c_uint = 0x0e3c;
pub const REG_TX_IQK: c_uint = 0x0e40;
pub const REG_RX_IQK: c_uint = 0x0e44;
pub const REG_IQK_AGC_PTS: c_uint = 0x0e48;
pub const REG_IQK_AGC_RSP: c_uint = 0x0e4c;
pub const REG_TX_IQK_TONE_B: c_uint = 0x0e50;
pub const REG_RX_IQK_TONE_B: c_uint = 0x0e54;
pub const REG_TX_IQK_PI_B: c_uint = 0x0e58;
pub const REG_RX_IQK_PI_B: c_uint = 0x0e5c;
pub const REG_IQK_AGC_CONT: c_uint = 0x0e60;
pub const REG_BLUETOOTH: c_uint = 0x0e6c;
pub const REG_RX_WAIT_CCA: c_uint = 0x0e70;
pub const REG_TX_CCK_RFON: c_uint = 0x0e74;
pub const REG_TX_CCK_BBON: c_uint = 0x0e78;
pub const REG_TX_OFDM_RFON: c_uint = 0x0e7c;
pub const REG_TX_OFDM_BBON: c_uint = 0x0e80;
pub const REG_TX_TO_RX: c_uint = 0x0e84;
pub const REG_TX_TO_TX: c_uint = 0x0e88;
pub const REG_RX_CCK: c_uint = 0x0e8c;
pub const REG_TX_POWER_BEFORE_IQK_A: c_uint = 0x0e94;
pub const REG_IQK_RPT_TXA: c_uint = 0x0e98;
pub const REG_TX_POWER_AFTER_IQK_A: c_uint = 0x0e9c;
pub const REG_RX_POWER_BEFORE_IQK_A: c_uint = 0x0ea0;
pub const REG_RX_POWER_BEFORE_IQK_A_2: c_uint = 0x0ea4;
pub const REG_RX_POWER_AFTER_IQK_A: c_uint = 0x0ea8;
pub const REG_IQK_RPT_RXA: c_uint = 0x0ea8;
pub const REG_RX_POWER_AFTER_IQK_A_2: c_uint = 0x0eac;
pub const REG_TX_POWER_BEFORE_IQK_B: c_uint = 0x0eb4;
pub const REG_IQK_RPT_TXB: c_uint = 0x0eb8;
pub const REG_TX_POWER_AFTER_IQK_B: c_uint = 0x0ebc;
pub const REG_RX_POWER_BEFORE_IQK_B: c_uint = 0x0ec0;
pub const REG_RX_POWER_BEFORE_IQK_B_2: c_uint = 0x0ec4;
pub const REG_RX_POWER_AFTER_IQK_B: c_uint = 0x0ec8;
pub const REG_IQK_RPT_RXB: c_uint = 0x0ec8;
pub const REG_RX_POWER_AFTER_IQK_B_2: c_uint = 0x0ecc;
pub const REG_RX_OFDM: c_uint = 0x0ed0;
pub const REG_RX_WAIT_RIFS: c_uint = 0x0ed4;
pub const REG_RX_TO_RX: c_uint = 0x0ed8;
pub const REG_STANDBY: c_uint = 0x0edc;
pub const REG_SLEEP: c_uint = 0x0ee0;
pub const REG_PMPD_ANAEN: c_uint = 0x0eec;
pub const REG_FW_START_ADDRESS: c_uint = 0x1000;
pub const REG_FW_START_ADDRESS_8192F: c_uint = 0x4000;
pub const REG_SW_GPIO_SHARE_CTRL_0: c_uint = 0x1038;
pub const REG_SW_GPIO_SHARE_CTRL_1: c_uint = 0x103c;
pub const REG_GPIO_A0: c_uint = 0x1050;
pub const REG_GPIO_B0: c_uint = 0x105b;
pub const REG_USB_INFO: c_uint = 0xfe17;
pub const REG_USB_HIMR: c_uint = 0xfe38;

// RSVD	BIT(13)

// RSVD	BIT(11)

pub const REG_USB_ACCESS_TIMEOUT: c_uint = 0xfe4c;
pub const REG_USB_SPECIAL_OPTION: c_uint = 0xfe55;

pub const REG_USB_HRPWM: c_uint = 0xfe58;
pub const REG_USB_DMA_AGG_TO: c_uint = 0xfe5b;
pub const REG_USB_AGG_TIMEOUT: c_uint = 0xfe5c;
pub const REG_USB_AGG_THRESH: c_uint = 0xfe5d;
pub const REG_NORMAL_SIE_VID: c_uint = 0xfe60	/* 0xfe60 - 0xfe61 */;
pub const REG_NORMAL_SIE_PID: c_uint = 0xfe62	/* 0xfe62 - 0xfe63 */;
pub const REG_NORMAL_SIE_OPTIONAL: c_uint = 0xfe64;
pub const REG_NORMAL_SIE_EP: c_uint = 0xfe65	/* 0xfe65 - 0xfe67 */;
pub const REG_NORMAL_SIE_EP_TX: c_uint = 0xfe66;
pub const NORMAL_SIE_EP_TX_HIGH_MASK: c_uint = 0x000f;
pub const NORMAL_SIE_EP_TX_NORMAL_MASK: c_uint = 0x00f0;
pub const NORMAL_SIE_EP_TX_LOW_MASK: c_uint = 0x0f00;
pub const REG_NORMAL_SIE_PHY: c_uint = 0xfe68	/* 0xfe68 - 0xfe6b */;
pub const REG_NORMAL_SIE_OPTIONAL2: c_uint = 0xfe6c;
pub const REG_NORMAL_SIE_GPS_EP: c_uint = 0xfe6d	/* RTL8723 only */;
pub const REG_NORMAL_SIE_MAC_ADDR: c_uint = 0xfe70	/* 0xfe70 - 0xfe75 */;
pub const REG_NORMAL_SIE_STRING: c_uint = 0xfe80	/* 0xfe80 - 0xfedf */;
//
// 8710B register addresses between 0x00 and 0xff must have 0x8000
// added to them. We take care of that in the rtl8xxxu_read{8,16,32}
// and rtl8xxxu_write{8,16,32} functions.
//
pub const REG_SYS_FUNC_8710B: c_uint = 0x0004;
pub const REG_AFE_CTRL_8710B: c_uint = 0x0050;
pub const REG_WL_RF_PSS_8710B: c_uint = 0x005c;
pub const REG_EFUSE_INDIRECT_CTRL_8710B: c_uint = 0x006c;
pub const NORMAL_REG_READ_OFFSET: c_uint = 0x83000000;
pub const NORMAL_REG_WRITE_OFFSET: c_uint = 0x84000000;
pub const EFUSE_READ_OFFSET: c_uint = 0x85000000;
pub const EFUSE_WRITE_OFFSET: c_uint = 0x86000000;
pub const REG_HIMR0_8710B: c_uint = 0x0080;
pub const REG_HISR0_8710B: c_uint = 0x0084;
//
// 8710B uses this instead of REG_MCU_FW_DL, but at least bits
// 0-7 have the same meaning.
//
pub const REG_8051FW_CTRL_V1_8710B: c_uint = 0x0090;
pub const REG_USB_HOST_INDIRECT_DATA_8710B: c_uint = 0x009c;
pub const REG_WL_STATUS_8710B: c_uint = 0x00f0;
pub const REG_USB_HOST_INDIRECT_ADDR_8710B: c_uint = 0x00f8;
//
// 8710B registers which must be accessed through rtl8710b_read_syson_reg
// and rtl8710b_write_syson_reg.
//
pub const SYSON_REG_BASE_ADDR_8710B: c_uint = 0x40000000;
pub const REG_SYS_XTAL_CTRL0_8710B: c_uint = 0x060;
pub const REG_SYS_EEPROM_CTRL0_8710B: c_uint = 0x0e0;
pub const REG_SYS_SYSTEM_CFG0_8710B: c_uint = 0x1f0;
pub const REG_SYS_SYSTEM_CFG1_8710B: c_uint = 0x1f4;
pub const REG_SYS_SYSTEM_CFG2_8710B: c_uint = 0x1f8;
// RF6052 registers
pub const RF6052_REG_AC: c_uint = 0x00;
pub const RF6052_REG_IQADJ_G1: c_uint = 0x01;
pub const RF6052_REG_IQADJ_G2: c_uint = 0x02;
pub const RF6052_REG_BS_PA_APSET_G1_G4: c_uint = 0x03;
pub const RF6052_REG_BS_PA_APSET_G5_G8: c_uint = 0x04;
pub const RF6052_REG_POW_TRSW: c_uint = 0x05;
pub const RF6052_REG_GAIN_RX: c_uint = 0x06;
pub const RF6052_REG_GAIN_TX: c_uint = 0x07;
pub const RF6052_REG_TXM_IDAC: c_uint = 0x08;
pub const RF6052_REG_IPA_G: c_uint = 0x09;
pub const RF6052_REG_TXBIAS_G: c_uint = 0x0a;
pub const RF6052_REG_TXPA_AG: c_uint = 0x0b;
pub const RF6052_REG_IPA_A: c_uint = 0x0c;
pub const RF6052_REG_TXBIAS_A: c_uint = 0x0d;
pub const RF6052_REG_BS_PA_APSET_G9_G11: c_uint = 0x0e;
pub const RF6052_REG_BS_IQGEN: c_uint = 0x0f;
pub const RF6052_REG_MODE1: c_uint = 0x10;
pub const RF6052_REG_MODE2: c_uint = 0x11;
pub const RF6052_REG_RX_AGC_HP: c_uint = 0x12;
pub const RF6052_REG_TX_AGC: c_uint = 0x13;
pub const RF6052_REG_BIAS: c_uint = 0x14;
pub const RF6052_REG_IPA: c_uint = 0x15;
pub const RF6052_REG_TXBIAS: c_uint = 0x16;
pub const RF6052_REG_POW_ABILITY: c_uint = 0x17;
pub const RF6052_REG_MODE_AG: c_uint = 0x18	/* RF channel and BW switch */;
pub const MODE_AG_CHANNEL_MASK: c_uint = 0x3ff;

pub const MODE_AG_BW_80MHZ_8723B: c_int = 0;
pub const RF6052_REG_TOP: c_uint = 0x19;
pub const RF6052_REG_RX_G1: c_uint = 0x1a;
pub const RF6052_REG_RX_G2: c_uint = 0x1b;
pub const RF6052_REG_RX_BB2: c_uint = 0x1c;
pub const RF6052_REG_RX_BB1: c_uint = 0x1d;
pub const RF6052_REG_RCK1: c_uint = 0x1e;
pub const RF6052_REG_RCK2: c_uint = 0x1f;
pub const RF6052_REG_TX_G1: c_uint = 0x20;
pub const RF6052_REG_TX_G2: c_uint = 0x21;
pub const RF6052_REG_TX_G3: c_uint = 0x22;
pub const RF6052_REG_TX_BB1: c_uint = 0x23;
pub const RF6052_REG_T_METER: c_uint = 0x24;
pub const RF6052_REG_SYN_G1: c_uint = 0x25	/* RF TX Power control */;
pub const RF6052_REG_SYN_G2: c_uint = 0x26	/* RF TX Power control */;
pub const RF6052_REG_SYN_G3: c_uint = 0x27	/* RF TX Power control */;
pub const RF6052_REG_SYN_G4: c_uint = 0x28	/* RF TX Power control */;
pub const RF6052_REG_SYN_G5: c_uint = 0x29	/* RF TX Power control */;
pub const RF6052_REG_SYN_G6: c_uint = 0x2a	/* RF TX Power control */;
pub const RF6052_REG_SYN_G7: c_uint = 0x2b	/* RF TX Power control */;
pub const RF6052_REG_SYN_G8: c_uint = 0x2c	/* RF TX Power control */;
pub const RF6052_REG_RCK_OS: c_uint = 0x30	/* RF TX PA control */;
pub const RF6052_REG_TXPA_G1: c_uint = 0x31	/* RF TX PA control */;
pub const RF6052_REG_TXPA_G2: c_uint = 0x32	/* RF TX PA control */;
pub const RF6052_REG_TXPA_G3: c_uint = 0x33	/* RF TX PA control */;
//
// NextGen regs: 8723BU
//
pub const RF6052_REG_GAIN_P1: c_uint = 0x35;
pub const RF6052_REG_T_METER_8723B: c_uint = 0x42;
pub const RF6052_REG_UNKNOWN_43: c_uint = 0x43;
pub const RF6052_REG_UNKNOWN_55: c_uint = 0x55;
pub const RF6052_REG_PAD_TXG: c_uint = 0x56;
pub const RF6052_REG_TXMOD: c_uint = 0x58;
pub const RF6052_REG_RXG_MIX_SWBW: c_uint = 0x87;
pub const RF6052_REG_S0S1: c_uint = 0xb0;
pub const RF6052_REG_GAIN_CCA: c_uint = 0xdf;
pub const RF6052_REG_UNKNOWN_ED: c_uint = 0xed;
pub const RF6052_REG_WE_LUT: c_uint = 0xef;
pub const RF6052_REG_GAIN_CTRL: c_uint = 0xf5;
