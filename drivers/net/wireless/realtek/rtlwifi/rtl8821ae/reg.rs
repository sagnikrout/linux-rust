//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8821ae/reg.h
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
// Copyright(c) 2009-2010  Realtek Corporation.
pub const TXPKT_BUF_SELECT: c_uint = 0x69;
pub const RXPKT_BUF_SELECT: c_uint = 0xA5;
pub const DISABLE_TRXPKT_BUF_ACCESS: c_uint = 0x0;
pub const REG_SYS_ISO_CTRL: c_uint = 0x0000;
pub const REG_SYS_FUNC_EN: c_uint = 0x0002;
pub const REG_APS_FSMCO: c_uint = 0x0004;
pub const REG_SYS_CLKR: c_uint = 0x0008;
pub const REG_9346CR: c_uint = 0x000A;
pub const REG_EE_VPD: c_uint = 0x000C;
pub const REG_AFE_MISC: c_uint = 0x0010;
pub const REG_SPS0_CTRL: c_uint = 0x0011;
pub const REG_SPS_OCP_CFG: c_uint = 0x0018;
pub const REG_RSV_CTRL: c_uint = 0x001C;
pub const REG_RF_CTRL: c_uint = 0x001F;
pub const REG_LDOA15_CTRL: c_uint = 0x0020;
pub const REG_LDOV12D_CTRL: c_uint = 0x0021;
pub const REG_LDOHCI12_CTRL: c_uint = 0x0022;
pub const REG_LPLDO_CTRL: c_uint = 0x0023;
pub const REG_AFE_XTAL_CTRL: c_uint = 0x0024;
// 1.5v for 8188EE test chip, 1.4v for MP chip
pub const REG_AFE_LDO_CTRL: c_uint = 0x0027;
pub const REG_AFE_PLL_CTRL: c_uint = 0x0028;
pub const REG_MAC_PHY_CTRL: c_uint = 0x002c;
pub const REG_EFUSE_CTRL: c_uint = 0x0030;
pub const REG_EFUSE_TEST: c_uint = 0x0034;
pub const REG_PWR_DATA: c_uint = 0x0038;
pub const REG_CAL_TIMER: c_uint = 0x003C;
pub const REG_ACLK_MON: c_uint = 0x003E;
pub const REG_GPIO_MUXCFG: c_uint = 0x0040;
pub const REG_GPIO_IO_SEL: c_uint = 0x0042;
pub const REG_MAC_PINMUX_CFG: c_uint = 0x0043;
pub const REG_GPIO_PIN_CTRL: c_uint = 0x0044;
pub const REG_GPIO_INTM: c_uint = 0x0048;
pub const REG_LEDCFG0: c_uint = 0x004C;
pub const REG_LEDCFG1: c_uint = 0x004D;
pub const REG_LEDCFG2: c_uint = 0x004E;
pub const REG_LEDCFG3: c_uint = 0x004F;
pub const REG_FSIMR: c_uint = 0x0050;
pub const REG_FSISR: c_uint = 0x0054;
pub const REG_HSIMR: c_uint = 0x0058;
pub const REG_HSISR: c_uint = 0x005c;
pub const REG_GPIO_PIN_CTRL_2: c_uint = 0x0060;
pub const REG_GPIO_IO_SEL_2: c_uint = 0x0062;
pub const REG_MULTI_FUNC_CTRL: c_uint = 0x0068;
pub const REG_GPIO_OUTPUT: c_uint = 0x006c;
pub const REG_OPT_CTRL: c_uint = 0x0074;
pub const REG_AFE_XTAL_CTRL_EXT: c_uint = 0x0078;
pub const REG_XCK_OUT_CTRL: c_uint = 0x007c;
pub const REG_MCUFWDL: c_uint = 0x0080;
pub const REG_WOL_EVENT: c_uint = 0x0081;
pub const REG_MCUTSTCFG: c_uint = 0x0084;
pub const REG_HIMR: c_uint = 0x00B0;
pub const REG_HISR: c_uint = 0x00B4;
pub const REG_HIMRE: c_uint = 0x00B8;
pub const REG_HISRE: c_uint = 0x00BC;
pub const REG_PMC_DBG_CTRL2: c_uint = 0x00CC;
pub const REG_EFUSE_ACCESS: c_uint = 0x00CF;
pub const REG_BIST_SCAN: c_uint = 0x00D0;
pub const REG_BIST_RPT: c_uint = 0x00D4;
pub const REG_BIST_ROM_RPT: c_uint = 0x00D8;
pub const REG_USB_SIE_INTF: c_uint = 0x00E0;
pub const REG_PCIE_MIO_INTF: c_uint = 0x00E4;
pub const REG_PCIE_MIO_INTD: c_uint = 0x00E8;
pub const REG_HPON_FSM: c_uint = 0x00EC;
pub const REG_SYS_CFG: c_uint = 0x00F0;
pub const REG_GPIO_OUTSTS: c_uint = 0x00F4;
pub const REG_MAC_PHY_CTRL_NORMAL: c_uint = 0x00F8;
pub const REG_SYS_CFG1: c_uint = 0x00FC;
pub const REG_ROM_VERSION: c_uint = 0x00FD;
pub const REG_CR: c_uint = 0x0100;
pub const REG_PBP: c_uint = 0x0104;
pub const REG_PKT_BUFF_ACCESS_CTRL: c_uint = 0x0106;
pub const REG_TRXDMA_CTRL: c_uint = 0x010C;
pub const REG_TRXFF_BNDY: c_uint = 0x0114;
pub const REG_TRXFF_STATUS: c_uint = 0x0118;
pub const REG_RXFF_PTR: c_uint = 0x011C;
pub const REG_CPWM: c_uint = 0x012F;
pub const REG_FWIMR: c_uint = 0x0130;
pub const REG_FWISR: c_uint = 0x0134;
pub const REG_FTISR: c_uint = 0x013C;
pub const REG_PKTBUF_DBG_CTRL: c_uint = 0x0140;
pub const REG_PKTBUF_DBG_DATA_L: c_uint = 0x0144;
pub const REG_PKTBUF_DBG_DATA_H: c_uint = 0x0148;

pub const REG_TC0_CTRL: c_uint = 0x0150;
pub const REG_TC1_CTRL: c_uint = 0x0154;
pub const REG_TC2_CTRL: c_uint = 0x0158;
pub const REG_TC3_CTRL: c_uint = 0x015C;
pub const REG_TC4_CTRL: c_uint = 0x0160;
pub const REG_TCUNIT_BASE: c_uint = 0x0164;
pub const REG_MBIST_START: c_uint = 0x0174;
pub const REG_MBIST_DONE: c_uint = 0x0178;
pub const REG_MBIST_FAIL: c_uint = 0x017C;
pub const REG_32K_CTRL: c_uint = 0x0194;
pub const REG_C2HEVT_MSG_NORMAL: c_uint = 0x01A0;
pub const REG_C2HEVT_CLEAR: c_uint = 0x01AF;
pub const REG_C2HEVT_MSG_TEST: c_uint = 0x01B8;
pub const REG_MCUTST_1: c_uint = 0x01c0;
pub const REG_MCUTST_WOWLAN: c_uint = 0x01C7;
pub const REG_FMETHR: c_uint = 0x01C8;
pub const REG_HMETFR: c_uint = 0x01CC;
pub const REG_HMEBOX_0: c_uint = 0x01D0;
pub const REG_HMEBOX_1: c_uint = 0x01D4;
pub const REG_HMEBOX_2: c_uint = 0x01D8;
pub const REG_HMEBOX_3: c_uint = 0x01DC;
pub const REG_LLT_INIT: c_uint = 0x01E0;
pub const REG_BB_ACCEESS_CTRL: c_uint = 0x01E8;
pub const REG_BB_ACCESS_DATA: c_uint = 0x01EC;
pub const REG_HMEBOX_EXT_0: c_uint = 0x01F0;
pub const REG_HMEBOX_EXT_1: c_uint = 0x01F4;
pub const REG_HMEBOX_EXT_2: c_uint = 0x01F8;
pub const REG_HMEBOX_EXT_3: c_uint = 0x01FC;
pub const REG_RQPN: c_uint = 0x0200;
pub const REG_FIFOPAGE: c_uint = 0x0204;
pub const REG_TDECTRL: c_uint = 0x0208;
pub const REG_TXDMA_OFFSET_CHK: c_uint = 0x020C;
pub const REG_TXDMA_STATUS: c_uint = 0x0210;
pub const REG_RQPN_NPQ: c_uint = 0x0214;
pub const REG_RXDMA_AGG_PG_TH: c_uint = 0x0280;
// FW shall update this register before FW write RXPKT_RELEASE_POLL to 1
pub const REG_FW_UPD_RDPTR: c_uint = 0x0284;
// Control the RX DMA.
pub const REG_RXDMA_CONTROL: c_uint = 0x0286;
// The number of packets in RXPKTBUF.
pub const REG_RXPKT_NUM: c_uint = 0x0287;
pub const REG_PCIE_CTRL_REG: c_uint = 0x0300;
pub const REG_INT_MIG: c_uint = 0x0304;
pub const REG_BCNQ_DESA: c_uint = 0x0308;
pub const REG_HQ_DESA: c_uint = 0x0310;
pub const REG_MGQ_DESA: c_uint = 0x0318;
pub const REG_VOQ_DESA: c_uint = 0x0320;
pub const REG_VIQ_DESA: c_uint = 0x0328;
pub const REG_BEQ_DESA: c_uint = 0x0330;
pub const REG_BKQ_DESA: c_uint = 0x0338;
pub const REG_RX_DESA: c_uint = 0x0340;
pub const REG_DBI_WDATA: c_uint = 0x0348;
pub const REG_DBI_RDATA: c_uint = 0x034C;
pub const REG_DBI_CTRL: c_uint = 0x0350;
pub const REG_DBI_ADDR: c_uint = 0x0350;
pub const REG_DBI_FLAG: c_uint = 0x0352;
pub const REG_MDIO_WDATA: c_uint = 0x0354;
pub const REG_MDIO_RDATA: c_uint = 0x0356;
pub const REG_MDIO_CTL: c_uint = 0x0358;
pub const REG_DBG_SEL: c_uint = 0x0360;
pub const REG_PCIE_HRPWM: c_uint = 0x0361;
pub const REG_PCIE_HCPWM: c_uint = 0x0363;
pub const REG_UART_CTRL: c_uint = 0x0364;
pub const REG_WATCH_DOG: c_uint = 0x0368;
pub const REG_UART_TX_DESA: c_uint = 0x0370;
pub const REG_UART_RX_DESA: c_uint = 0x0378;
pub const REG_HDAQ_DESA_NODEF: c_uint = 0x0000;
pub const REG_CMDQ_DESA_NODEF: c_uint = 0x0000;
pub const REG_VOQ_INFORMATION: c_uint = 0x0400;
pub const REG_VIQ_INFORMATION: c_uint = 0x0404;
pub const REG_BEQ_INFORMATION: c_uint = 0x0408;
pub const REG_BKQ_INFORMATION: c_uint = 0x040C;
pub const REG_MGQ_INFORMATION: c_uint = 0x0410;
pub const REG_HGQ_INFORMATION: c_uint = 0x0414;
pub const REG_BCNQ_INFORMATION: c_uint = 0x0418;
pub const REG_TXPKT_EMPTY: c_uint = 0x041A;
pub const REG_CPU_MGQ_INFORMATION: c_uint = 0x041C;
pub const REG_FWHW_TXQ_CTRL: c_uint = 0x0420;
pub const REG_HWSEQ_CTRL: c_uint = 0x0423;
pub const REG_TXPKTBUF_BCNQ_BDNY: c_uint = 0x0424;
pub const REG_TXPKTBUF_MGQ_BDNY: c_uint = 0x0425;
pub const REG_MULTI_BCNQ_EN: c_uint = 0x0426;
pub const REG_MULTI_BCNQ_OFFSET: c_uint = 0x0427;
pub const REG_SPEC_SIFS: c_uint = 0x0428;
pub const REG_RL: c_uint = 0x042A;
pub const REG_DARFRC: c_uint = 0x0430;
pub const REG_RARFRC: c_uint = 0x0438;
pub const REG_RRSR: c_uint = 0x0440;
pub const REG_ARFR0: c_uint = 0x0444;
pub const REG_ARFR1: c_uint = 0x044C;
pub const REG_CCK_CHECK: c_uint = 0x0454;
pub const REG_AMPDU_MAX_TIME: c_uint = 0x0456;
pub const REG_AGGLEN_LMT: c_uint = 0x0458;
pub const REG_AMPDU_MIN_SPACE: c_uint = 0x045C;
pub const REG_TXPKTBUF_WMAC_LBK_BF_HD: c_uint = 0x045D;
pub const REG_FAST_EDCA_CTRL: c_uint = 0x0460;
pub const REG_RD_RESP_PKT_TH: c_uint = 0x0463;
pub const REG_INIRTS_RATE_SEL: c_uint = 0x0480;
pub const REG_INIDATA_RATE_SEL: c_uint = 0x0484;
pub const REG_ARFR2: c_uint = 0x048C;
pub const REG_ARFR3: c_uint = 0x0494;
pub const REG_POWER_STATUS: c_uint = 0x04A4;
pub const REG_POWER_STAGE1: c_uint = 0x04B4;
pub const REG_POWER_STAGE2: c_uint = 0x04B8;
pub const REG_PKT_LIFE_TIME: c_uint = 0x04C0;
pub const REG_STBC_SETTING: c_uint = 0x04C4;
pub const REG_HT_SINGLE_AMPDU: c_uint = 0x04C7;
pub const REG_PROT_MODE_CTRL: c_uint = 0x04C8;
pub const REG_MAX_AGGR_NUM: c_uint = 0x04CA;
pub const REG_BAR_MODE_CTRL: c_uint = 0x04CC;
pub const REG_RA_TRY_RATE_AGG_LMT: c_uint = 0x04CF;
pub const REG_EARLY_MODE_CONTROL: c_uint = 0x04D0;
pub const REG_NQOS_SEQ: c_uint = 0x04DC;
pub const REG_QOS_SEQ: c_uint = 0x04DE;
pub const REG_NEED_CPU_HANDLE: c_uint = 0x04E0;
pub const REG_PKT_LOSE_RPT: c_uint = 0x04E1;
pub const REG_PTCL_ERR_STATUS: c_uint = 0x04E2;
pub const REG_TX_RPT_CTRL: c_uint = 0x04EC;
pub const REG_TX_RPT_TIME: c_uint = 0x04F0;
pub const REG_DUMMY: c_uint = 0x04FC;
pub const REG_EDCA_VO_PARAM: c_uint = 0x0500;
pub const REG_EDCA_VI_PARAM: c_uint = 0x0504;
pub const REG_EDCA_BE_PARAM: c_uint = 0x0508;
pub const REG_EDCA_BK_PARAM: c_uint = 0x050C;
pub const REG_BCNTCFG: c_uint = 0x0510;
pub const REG_PIFS: c_uint = 0x0512;
pub const REG_RDG_PIFS: c_uint = 0x0513;
pub const REG_SIFS_CTX: c_uint = 0x0514;
pub const REG_SIFS_TRX: c_uint = 0x0516;
pub const REG_AGGR_BREAK_TIME: c_uint = 0x051A;
pub const REG_SLOT: c_uint = 0x051B;
pub const REG_TX_PTCL_CTRL: c_uint = 0x0520;
pub const REG_TXPAUSE: c_uint = 0x0522;
pub const REG_DIS_TXREQ_CLR: c_uint = 0x0523;
pub const REG_RD_CTRL: c_uint = 0x0524;
pub const REG_TBTT_PROHIBIT: c_uint = 0x0540;
pub const REG_RD_NAV_NXT: c_uint = 0x0544;
pub const REG_NAV_PROT_LEN: c_uint = 0x0546;
pub const REG_BCN_CTRL: c_uint = 0x0550;
pub const REG_MBID_NUM: c_uint = 0x0552;
pub const REG_DUAL_TSF_RST: c_uint = 0x0553;
pub const REG_BCN_INTERVAL: c_uint = 0x0554;
pub const REG_MBSSID_BCN_SPACE: c_uint = 0x0554;
pub const REG_DRVERLYINT: c_uint = 0x0558;
pub const REG_BCNDMATIM: c_uint = 0x0559;
pub const REG_ATIMWND: c_uint = 0x055A;
pub const REG_USTIME_TSF: c_uint = 0x055C;
pub const REG_BCN_MAX_ERR: c_uint = 0x055D;
pub const REG_RXTSF_OFFSET_CCK: c_uint = 0x055E;
pub const REG_RXTSF_OFFSET_OFDM: c_uint = 0x055F;
pub const REG_TSFTR: c_uint = 0x0560;
pub const REG_INIT_TSFTR: c_uint = 0x0564;
pub const REG_SECONDARY_CCA_CTRL: c_uint = 0x0577;
pub const REG_PSTIMER: c_uint = 0x0580;
pub const REG_TIMER0: c_uint = 0x0584;
pub const REG_TIMER1: c_uint = 0x0588;
pub const REG_ACMHWCTRL: c_uint = 0x05C0;
pub const REG_ACMRSTCTRL: c_uint = 0x05C1;
pub const REG_ACMAVG: c_uint = 0x05C2;
pub const REG_VO_ADMTIME: c_uint = 0x05C4;
pub const REG_VI_ADMTIME: c_uint = 0x05C6;
pub const REG_BE_ADMTIME: c_uint = 0x05C8;
pub const REG_EDCA_RANDOM_GEN: c_uint = 0x05CC;
pub const REG_NOA_DESC_SEL: c_uint = 0x05CF;
pub const REG_NOA_DESC_DURATION: c_uint = 0x05E0;
pub const REG_NOA_DESC_INTERVAL: c_uint = 0x05E4;
pub const REG_NOA_DESC_START: c_uint = 0x05E8;
pub const REG_NOA_DESC_COUNT: c_uint = 0x05EC;
pub const REG_SCH_TXCMD: c_uint = 0x05F8;
pub const REG_APSD_CTRL: c_uint = 0x0600;
pub const REG_BWOPMODE: c_uint = 0x0603;
pub const REG_TCR: c_uint = 0x0604;
pub const REG_RCR: c_uint = 0x0608;
pub const REG_RX_PKT_LIMIT: c_uint = 0x060C;
pub const REG_RX_DLK_TIME: c_uint = 0x060D;
pub const REG_RX_DRVINFO_SZ: c_uint = 0x060F;
pub const REG_MACID: c_uint = 0x0610;
pub const REG_BSSID: c_uint = 0x0618;
pub const REG_MAR: c_uint = 0x0620;
pub const REG_MBIDCAMCFG: c_uint = 0x0628;
pub const REG_USTIME_EDCA: c_uint = 0x0638;
pub const REG_MAC_SPEC_SIFS: c_uint = 0x063A;
pub const REG_RESP_SIFS_CCK: c_uint = 0x063C;
pub const REG_RESP_SIFS_OFDM: c_uint = 0x063E;
pub const REG_ACKTO: c_uint = 0x0640;
pub const REG_CTS2TO: c_uint = 0x0641;
pub const REG_EIFS: c_uint = 0x0642;
pub const REG_NAV_CTRL: c_uint = 0x0650;
pub const REG_NAV_UPPER: c_uint = 0x0652;
pub const REG_BACAMCMD: c_uint = 0x0654;
pub const REG_BACAMCONTENT: c_uint = 0x0658;
pub const REG_LBDLY: c_uint = 0x0660;
pub const REG_FWDLY: c_uint = 0x0661;
pub const REG_RXERR_RPT: c_uint = 0x0664;
pub const REG_TRXPTCL_CTL: c_uint = 0x0668;
pub const REG_CAMCMD: c_uint = 0x0670;
pub const REG_CAMWRITE: c_uint = 0x0674;
pub const REG_CAMREAD: c_uint = 0x0678;
pub const REG_CAMDBG: c_uint = 0x067C;
pub const REG_SECCFG: c_uint = 0x0680;
pub const REG_WOW_CTRL: c_uint = 0x0690;
pub const REG_PSSTATUS: c_uint = 0x0691;
pub const REG_PS_RX_INFO: c_uint = 0x0692;
pub const REG_UAPSD_TID: c_uint = 0x0693;
pub const REG_LPNAV_CTRL: c_uint = 0x0694;
pub const REG_WKFMCAM_NUM: c_uint = 0x0698;
pub const REG_WKFMCAM_RWD: c_uint = 0x069C;
pub const REG_RXFLTMAP0: c_uint = 0x06A0;
pub const REG_RXFLTMAP1: c_uint = 0x06A2;
pub const REG_RXFLTMAP2: c_uint = 0x06A4;
pub const REG_BCN_PSR_RPT: c_uint = 0x06A8;
pub const REG_CALB32K_CTRL: c_uint = 0x06AC;
pub const REG_PKT_MON_CTRL: c_uint = 0x06B4;
pub const REG_BT_COEX_TABLE: c_uint = 0x06C0;
pub const REG_WMAC_RESP_TXINFO: c_uint = 0x06D8;
pub const REG_USB_INFO: c_uint = 0xFE17;
pub const REG_USB_SPECIAL_OPTION: c_uint = 0xFE55;
pub const REG_USB_DMA_AGG_TO: c_uint = 0xFE5B;
pub const REG_USB_AGG_TO: c_uint = 0xFE5C;
pub const REG_USB_AGG_TH: c_uint = 0xFE5D;
pub const REG_TEST_USB_TXQS: c_uint = 0xFE48;
pub const REG_TEST_SIE_VID: c_uint = 0xFE60;
pub const REG_TEST_SIE_PID: c_uint = 0xFE62;
pub const REG_TEST_SIE_OPTIONAL: c_uint = 0xFE64;
pub const REG_TEST_SIE_CHIRP_K: c_uint = 0xFE65;
pub const REG_TEST_SIE_PHY: c_uint = 0xFE66;
pub const REG_TEST_SIE_MAC_ADDR: c_uint = 0xFE70;
pub const REG_TEST_SIE_STRING: c_uint = 0xFE80;
pub const REG_NORMAL_SIE_VID: c_uint = 0xFE60;
pub const REG_NORMAL_SIE_PID: c_uint = 0xFE62;
pub const REG_NORMAL_SIE_OPTIONAL: c_uint = 0xFE64;
pub const REG_NORMAL_SIE_EP: c_uint = 0xFE65;
pub const REG_NORMAL_SIE_PHY: c_uint = 0xFE68;
pub const REG_NORMAL_SIE_MAC_ADDR: c_uint = 0xFE70;
pub const REG_NORMAL_SIE_STRING: c_uint = 0xFE80;

pub const UNUSED_REGISTER: c_uint = 0x1BF;

pub const INVALID_BBRF_VALUE: c_uint = 0x12345678;
pub const MAX_MSS_DENSITY_2T: c_uint = 0x13;
pub const MAX_MSS_DENSITY_1T: c_uint = 0x0A;

pub const GPIOSEL_GPIO: c_int = 0;

// 8723/8188E Host System Interrupt Mask Register (offset 0x58, 32 byte)

// 8723/8188E Host System Interrupt Status Register (offset 0x5C, 32 byte)

pub const MSR_NOLINK: c_uint = 0x00;
pub const MSR_ADHOC: c_uint = 0x01;
pub const MSR_INFRA: c_uint = 0x02;
pub const MSR_AP: c_uint = 0x03;
pub const MSR_MASK: c_uint = 0x03;
pub const RRSR_RSC_OFFSET: c_int = 21;
pub const RRSR_SHORT_OFFSET: c_int = 23;
pub const RRSR_RSC_BW_40M: c_uint = 0x600000;
pub const RRSR_RSC_UPSUBCHNL: c_uint = 0x400000;
pub const RRSR_RSC_LOWSUBCHNL: c_uint = 0x200000;
pub const RRSR_SHORT: c_uint = 0x800000;

pub const RATR_1M: c_uint = 0x00000001;
pub const RATR_2M: c_uint = 0x00000002;
pub const RATR_55M: c_uint = 0x00000004;
pub const RATR_11M: c_uint = 0x00000008;
pub const RATR_6M: c_uint = 0x00000010;
pub const RATR_9M: c_uint = 0x00000020;
pub const RATR_12M: c_uint = 0x00000040;
pub const RATR_18M: c_uint = 0x00000080;
pub const RATR_24M: c_uint = 0x00000100;
pub const RATR_36M: c_uint = 0x00000200;
pub const RATR_48M: c_uint = 0x00000400;
pub const RATR_54M: c_uint = 0x00000800;
pub const RATR_MCS0: c_uint = 0x00001000;
pub const RATR_MCS1: c_uint = 0x00002000;
pub const RATR_MCS2: c_uint = 0x00004000;
pub const RATR_MCS3: c_uint = 0x00008000;
pub const RATR_MCS4: c_uint = 0x00010000;
pub const RATR_MCS5: c_uint = 0x00020000;
pub const RATR_MCS6: c_uint = 0x00040000;
pub const RATR_MCS7: c_uint = 0x00080000;
pub const RATR_MCS8: c_uint = 0x00100000;
pub const RATR_MCS9: c_uint = 0x00200000;
pub const RATR_MCS10: c_uint = 0x00400000;
pub const RATR_MCS11: c_uint = 0x00800000;
pub const RATR_MCS12: c_uint = 0x01000000;
pub const RATR_MCS13: c_uint = 0x02000000;
pub const RATR_MCS14: c_uint = 0x04000000;
pub const RATR_MCS15: c_uint = 0x08000000;

pub const CAM_NOTVALID: c_uint = 0x0000;

pub const CAM_NONE: c_uint = 0x0;
pub const CAM_WEP40: c_uint = 0x01;
pub const CAM_TKIP: c_uint = 0x02;
pub const CAM_AES: c_uint = 0x04;
pub const CAM_WEP104: c_uint = 0x05;
pub const TOTAL_CAM_ENTRY: c_int = 32;
pub const HALF_CAM_ENTRY: c_int = 16;

pub const CAM_READ: c_uint = 0x00000000;

pub const SCR_USEDK: c_uint = 0x01;
pub const SCR_TXSEC_ENABLE: c_uint = 0x02;
pub const SCR_RXSEC_ENABLE: c_uint = 0x04;

//
// 8188 IMR/ISR bits
//
pub const IMR_DISABLED: c_uint = 0x0;
// IMR DW0(0x0060-0063) Bit 0-31
// TXRPT interrupt when CCX bit of the packet is set

// Power Save Time Out Interrupt

// When GTIMER4 expires, this bit is set to 1

// When GTIMER3 expires, this bit is set to 1

// Transmit Beacon0 Error

// Transmit Beacon0 OK

// TSF Timer BIT32 toggle indication interrupt

// Beacon DMA Interrupt 0

// Beacon Queue DMA OK0

// HSISR Indicator (HSIMR & HSISR is true, this bit is set to 1)

// Beacon DMA Interrupt Extension for Win7

// CTWidnow End or ATIM Window End

// HISR1 Indicator (HISR1 & HIMR1 is true, this bit is set to 1)

// CPU to Host Command INT Status, Write 1 clear

// CPU power Mode exchange INT Status, Write 1 clear

// CPU power Mode exchange INT Status, Write 1 clear

// High Queue DMA OK

// Management Queue DMA OK

// AC_BK DMA OK

// AC_BE DMA OK

// AC_VI DMA OK

// AC_VO DMA OK

// Rx Descriptor Unavailable

// IMR DW1(0x00B4-00B7) Bit 0-31
// Beacon DMA Interrupt 7

// Beacon DMA Interrupt 6

// Beacon DMA Interrupt 5

// Beacon DMA Interrupt 4

// Beacon DMA Interrupt 3

// Beacon DMA Interrupt 2

// Beacon DMA Interrupt 1

// Beacon Queue DMA OK Interrup 7

// Beacon Queue DMA OK Interrup 6

// Beacon Queue DMA OK Interrup 5

// Beacon Queue DMA OK Interrup 4

// Beacon Queue DMA OK Interrup 3

// Beacon Queue DMA OK Interrup 2

// Beacon Queue DMA OK Interrup 1

// ATIM Window End Extension for Win7

// Tx Error Flag Interrupt Status, write 1 clear.

// Rx Error Flag INT Status, Write 1 clear

// Transmit FIFO Overflow

// Receive FIFO Overflow

pub const HWSET_MAX_SIZE: c_int = 512;
pub const EFUSE_MAX_SECTION: c_int = 64;
pub const EFUSE_REAL_CONTENT_LEN: c_int = 256;
// PG data exclude header, dummy 7 bytes frome CP test and reserved 1byte.
pub const EFUSE_OOB_PROTECT_BYTES: c_int = 18;
pub const EEPROM_DEFAULT_TSSI: c_uint = 0x0;
pub const EEPROM_DEFAULT_TXPOWERDIFF: c_uint = 0x0;
pub const EEPROM_DEFAULT_CRYSTALCAP: c_uint = 0x5;
pub const EEPROM_DEFAULT_BOARDTYPE: c_uint = 0x02;
pub const EEPROM_DEFAULT_TXPOWER: c_uint = 0x1010;
pub const EEPROM_DEFAULT_HT2T_TXPWR: c_uint = 0x10;
pub const EEPROM_DEFAULT_LEGACYHTTXPOWERDIFF: c_uint = 0x3;
pub const EEPROM_DEFAULT_THERMALMETER: c_uint = 0x18;
pub const EEPROM_DEFAULT_ANTTXPOWERDIFF: c_uint = 0x0;
pub const EEPROM_DEFAULT_TXPWDIFF_CRYSTALCAP: c_uint = 0x5;
pub const EEPROM_DEFAULT_TXPOWERLEVEL: c_uint = 0x22;
pub const EEPROM_DEFAULT_HT40_2SDIFF: c_uint = 0x0;
pub const EEPROM_DEFAULT_HT20_DIFF: c_int = 2;
pub const EEPROM_DEFAULT_LEGACYHTTXPOWERDIFF: c_uint = 0x3;
pub const EEPROM_DEFAULT_HT40_PWRMAXOFFSET: c_int = 0;
pub const EEPROM_DEFAULT_HT20_PWRMAXOFFSET: c_int = 0;
pub const RF_OPTION1: c_uint = 0x79;
pub const RF_OPTION2: c_uint = 0x7A;
pub const RF_OPTION3: c_uint = 0x7B;
pub const RF_OPTION4: c_uint = 0xC3;
pub const EEPROM_DEFAULT_PID: c_uint = 0x1234;
pub const EEPROM_DEFAULT_VID: c_uint = 0x5678;
pub const EEPROM_DEFAULT_CUSTOMERID: c_uint = 0xAB;
pub const EEPROM_DEFAULT_SUBCUSTOMERID: c_uint = 0xCD;
pub const EEPROM_DEFAULT_VERSION: c_int = 0;
pub const EEPROM_CHANNEL_PLAN_FCC: c_uint = 0x0;
pub const EEPROM_CHANNEL_PLAN_IC: c_uint = 0x1;
pub const EEPROM_CHANNEL_PLAN_ETSI: c_uint = 0x2;
pub const EEPROM_CHANNEL_PLAN_SPAIN: c_uint = 0x3;
pub const EEPROM_CHANNEL_PLAN_FRANCE: c_uint = 0x4;
pub const EEPROM_CHANNEL_PLAN_MKK: c_uint = 0x5;
pub const EEPROM_CHANNEL_PLAN_MKK1: c_uint = 0x6;
pub const EEPROM_CHANNEL_PLAN_ISRAEL: c_uint = 0x7;
pub const EEPROM_CHANNEL_PLAN_TELEC: c_uint = 0x8;
pub const EEPROM_CHANNEL_PLAN_GLOBAL_DOMAIN: c_uint = 0x9;
pub const EEPROM_CHANNEL_PLAN_WORLD_WIDE_13: c_uint = 0xA;

pub const EEPROM_CHANNEL_PLAN_BY_HW_MASK: c_uint = 0x80;
pub const EEPROM_CID_DEFAULT: c_uint = 0x0;
pub const EEPROM_CID_TOSHIBA: c_uint = 0x4;
pub const EEPROM_CID_CCX: c_uint = 0x10;
pub const EEPROM_CID_QMI: c_uint = 0x0D;
pub const EEPROM_CID_WHQL: c_uint = 0xFE;
pub const RTL_EEPROM_ID: c_uint = 0x8129;
pub const EEPROM_HPON: c_uint = 0x02;
pub const EEPROM_CLK: c_uint = 0x06;
pub const EEPROM_TESTR: c_uint = 0x08;
pub const EEPROM_TXPOWERCCK: c_uint = 0x10;
pub const EEPROM_TXPOWERHT40_1S: c_uint = 0x16;
pub const EEPROM_TXPOWERHT20DIFF: c_uint = 0x1B;
pub const EEPROM_TXPOWER_OFDMDIFF: c_uint = 0x1B;
pub const EEPROM_TX_PWR_INX: c_uint = 0x10;

pub const EEPROM_RF_BOARD_OPTION: c_uint = 0xC1;
pub const EEPROM_RF_FEATURE_OPTION_88E: c_uint = 0xC2;
pub const EEPROM_RF_BT_SETTING: c_uint = 0xC3;
pub const EEPROM_VERSION: c_uint = 0xC4;
pub const EEPROM_CUSTOMER_ID: c_uint = 0xC5;
pub const EEPROM_RF_ANTENNA_OPT_88E: c_uint = 0xC9;
pub const EEPROM_RFE_OPTION: c_uint = 0xCA;
pub const EEPROM_MAC_ADDR: c_uint = 0xD0;
pub const EEPROM_VID: c_uint = 0xD6;
pub const EEPROM_DID: c_uint = 0xD8;
pub const EEPROM_SVID: c_uint = 0xDA;
pub const EEPROM_SMID: c_uint = 0xDC;

pub const RCR_MXDMA_OFFSET: c_int = 8;
pub const RCR_FIFO_OFFSET: c_int = 13;
pub const RSV_CTRL: c_uint = 0x001C;
pub const RD_CTRL: c_uint = 0x0524;
pub const REG_USB_INFO: c_uint = 0xFE17;
pub const REG_USB_SPECIAL_OPTION: c_uint = 0xFE55;
pub const REG_USB_DMA_AGG_TO: c_uint = 0xFE5B;
pub const REG_USB_AGG_TO: c_uint = 0xFE5C;
pub const REG_USB_AGG_TH: c_uint = 0xFE5D;
pub const REG_USB_VID: c_uint = 0xFE60;
pub const REG_USB_PID: c_uint = 0xFE62;
pub const REG_USB_OPTIONAL: c_uint = 0xFE64;
pub const REG_USB_CHIRP_K: c_uint = 0xFE65;
pub const REG_USB_PHY: c_uint = 0xFE66;
pub const REG_USB_MAC_ADDR: c_uint = 0xFE70;
pub const REG_USB_HRPWM: c_uint = 0xFE58;
pub const REG_USB_HCPWM: c_uint = 0xFE57;

pub const APLL_REF_CLK_13MHZ: c_uint = 0x1;
pub const APLL_REF_CLK_19_2MHZ: c_uint = 0x2;
pub const APLL_REF_CLK_20MHZ: c_uint = 0x3;
pub const APLL_REF_CLK_25MHZ: c_uint = 0x4;
pub const APLL_REF_CLK_26MHZ: c_uint = 0x5;
pub const APLL_REF_CLK_38_4MHZ: c_uint = 0x6;
pub const APLL_REF_CLK_40MHZ: c_uint = 0x7;

pub const CHIP_VER_RTL_MASK: c_uint = 0xF000;
pub const CHIP_VER_RTL_SHIFT: c_int = 12;

pub const MASK_NETTYPE: c_uint = 0x30000;
pub const NT_NO_LINK: c_uint = 0x0;
pub const NT_LINK_AD_HOC: c_uint = 0x1;
pub const NT_LINK_AP: c_uint = 0x2;
pub const NT_AS_AP: c_uint = 0x3;

pub const MASK_LBMODE: c_uint = 0xF000000;
pub const LOOPBACK_NORMAL: c_uint = 0x0;

pub const LOOPBACK_MAC_DELAY: c_uint = 0x3;
pub const LOOPBACK_PHY: c_uint = 0x1;
pub const LOOPBACK_DMA: c_uint = 0x7;

pub const _PSRX_MASK: c_uint = 0xF;
pub const _PSTX_MASK: c_uint = 0xF0;

pub const PBP_64: c_uint = 0x0;
pub const PBP_128: c_uint = 0x1;
pub const PBP_256: c_uint = 0x2;
pub const PBP_512: c_uint = 0x3;
pub const PBP_1024: c_uint = 0x4;

pub const QUEUE_LOW: c_int = 1;
pub const QUEUE_NORMAL: c_int = 2;
pub const QUEUE_HIGH: c_int = 3;
pub const _LLT_NO_ACTIVE: c_uint = 0x0;
pub const _LLT_WRITE_ACCESS: c_uint = 0x1;
pub const _LLT_READ_ACCESS: c_uint = 0x2;

pub const BCN_HEAD_MASK: c_uint = 0xFF00;
pub const BLK_DESC_NUM_SHIFT: c_int = 4;
pub const BLK_DESC_NUM_MASK: c_uint = 0xF;

pub const RATE_REG_BITMAP_ALL: c_uint = 0xFFFFF;

pub const RRSR_RSC_RESERVED: c_uint = 0x0;
pub const RRSR_RSC_UPPER_SUBCHANNEL: c_uint = 0x1;
pub const RRSR_RSC_LOWER_SUBCHANNEL: c_uint = 0x2;
pub const RRSR_RSC_DUPLICATE_MODE: c_uint = 0x3;

pub const RETRY_LIMIT_SHORT_SHIFT: c_int = 8;
pub const RETRY_LIMIT_LONG_SHIFT: c_int = 0;

pub const AC_PARAM_TXOP_LIMIT_OFFSET: c_int = 16;
pub const AC_PARAM_ECW_MAX_OFFSET: c_int = 12;
pub const AC_PARAM_ECW_MIN_OFFSET: c_int = 8;
pub const AC_PARAM_AIFS_OFFSET: c_int = 0;

pub const RATE_BITMAP_ALL: c_uint = 0xFFFFF;
pub const RATE_RRSR_CCK_ONLY_1M: c_uint = 0xFFFF1;

pub const RXERR_TYPE_OFDM_PPDU: c_int = 0;
pub const RXERR_TYPE_OFDM_FALSE_ALARM: c_int = 1;
pub const RXERR_TYPE_OFDM_MPDU_OK: c_int = 2;
pub const RXERR_TYPE_OFDM_MPDU_FAIL: c_int = 3;
pub const RXERR_TYPE_CCK_PPDU: c_int = 4;
pub const RXERR_TYPE_CCK_FALSE_ALARM: c_int = 5;
pub const RXERR_TYPE_CCK_MPDU_OK: c_int = 6;
pub const RXERR_TYPE_CCK_MPDU_FAIL: c_int = 7;
pub const RXERR_TYPE_HT_PPDU: c_int = 8;
pub const RXERR_TYPE_HT_FALSE_ALARM: c_int = 9;
pub const RXERR_TYPE_HT_MPDU_TOTAL: c_int = 10;
pub const RXERR_TYPE_HT_MPDU_OK: c_int = 11;
pub const RXERR_TYPE_HT_MPDU_FAIL: c_int = 12;
pub const RXERR_TYPE_RX_FULL_DROP: c_int = 15;
pub const RXERR_COUNTER_MASK: c_uint = 0xFFFFF;

pub const USB_IS_HIGH_SPEED: c_int = 0;
pub const USB_IS_FULL_SPEED: c_int = 1;

pub const USB_NORMAL_SIE_EP_MASK: c_uint = 0xF;
pub const USB_NORMAL_SIE_EP_SHIFT: c_int = 4;
pub const USB_TEST_EP_MASK: c_uint = 0x30;
pub const USB_TEST_EP_SHIFT: c_int = 4;

pub const MAC_ADDR_LEN: c_int = 6;
pub const LAST_ENTRY_OF_TX_PKT_BUFFER: c_int = 255;
pub const POLLING_LLT_THRESHOLD: c_int = 20;
pub const POLLING_READY_TIMEOUT_COUNT: c_int = 3000;
pub const MAX_MSS_DENSITY_2T: c_uint = 0x13;
pub const MAX_MSS_DENSITY_1T: c_uint = 0x0A;

pub const EPROM_CMD_CONFIG: c_uint = 0x3;
pub const EPROM_CMD_LOAD: c_int = 1;

pub const RA_LSSIWRITE_8821A: c_uint = 0xc90;
pub const RB_LSSIWRITE_8821A: c_uint = 0xe90;
pub const RA_PIREAD_8821A: c_uint = 0xd04;
pub const RB_PIREAD_8821A: c_uint = 0xd44;
pub const RA_SIREAD_8821A: c_uint = 0xd08;
pub const RB_SIREAD_8821A: c_uint = 0xd48;
pub const RPMAC_RESET: c_uint = 0x100;
pub const RPMAC_TXSTART: c_uint = 0x104;
pub const RPMAC_TXLEGACYSIG: c_uint = 0x108;
pub const RPMAC_TXHTSIG1: c_uint = 0x10c;
pub const RPMAC_TXHTSIG2: c_uint = 0x110;
pub const RPMAC_PHYDEBUG: c_uint = 0x114;
pub const RPMAC_TXPACKETNUM: c_uint = 0x118;
pub const RPMAC_TXIDLE: c_uint = 0x11c;
pub const RPMAC_TXMACHEADER0: c_uint = 0x120;
pub const RPMAC_TXMACHEADER1: c_uint = 0x124;
pub const RPMAC_TXMACHEADER2: c_uint = 0x128;
pub const RPMAC_TXMACHEADER3: c_uint = 0x12c;
pub const RPMAC_TXMACHEADER4: c_uint = 0x130;
pub const RPMAC_TXMACHEADER5: c_uint = 0x134;
pub const RPMAC_TXDADATYPE: c_uint = 0x138;
pub const RPMAC_TXRANDOMSEED: c_uint = 0x13c;
pub const RPMAC_CCKPLCPPREAMBLE: c_uint = 0x140;
pub const RPMAC_CCKPLCPHEADER: c_uint = 0x144;
pub const RPMAC_CCKCRC16: c_uint = 0x148;
pub const RPMAC_OFDMRXCRC32OK: c_uint = 0x170;
pub const RPMAC_OFDMRXCRC32ER: c_uint = 0x174;
pub const RPMAC_OFDMRXPARITYER: c_uint = 0x178;
pub const RPMAC_OFDMRXCRC8ER: c_uint = 0x17c;
pub const RPMAC_CCKCRXRC16ER: c_uint = 0x180;
pub const RPMAC_CCKCRXRC32ER: c_uint = 0x184;
pub const RPMAC_CCKCRXRC32OK: c_uint = 0x188;
pub const RPMAC_TXSTATUS: c_uint = 0x18c;
pub const RFPGA0_RFMOD: c_uint = 0x800;
pub const RFPGA0_TXINFO: c_uint = 0x804;
pub const RFPGA0_PSDFUNCTION: c_uint = 0x808;
pub const RFPGA0_TXGAINSTAGE: c_uint = 0x80c;
pub const RFPGA0_RFTIMING1: c_uint = 0x810;
pub const RFPGA0_RFTIMING2: c_uint = 0x814;
pub const RFPGA0_XA_HSSIPARAMETER1: c_uint = 0x820;
pub const RFPGA0_XA_HSSIPARAMETER2: c_uint = 0x824;
pub const RFPGA0_XB_HSSIPARAMETER1: c_uint = 0x828;
pub const RFPGA0_XB_HSSIPARAMETER2: c_uint = 0x82c;
pub const RCCAONSEC: c_uint = 0x838;
pub const RFPGA0_XA_LSSIPARAMETER: c_uint = 0x840;
pub const RFPGA0_XB_LSSIPARAMETER: c_uint = 0x844;
pub const RL1PEAKTH: c_uint = 0x848;
pub const RFPGA0_RFWAKEUPPARAMETER: c_uint = 0x850;
pub const RFPGA0_RFSLEEPUPPARAMETER: c_uint = 0x854;
pub const RFPGA0_XAB_SWITCHCONTROL: c_uint = 0x858;
pub const RFPGA0_XCD_SWITCHCONTROL: c_uint = 0x85c;
pub const RFPGA0_XA_RFINTERFACEOE: c_uint = 0x860;
pub const RFC_AREA: c_uint = 0x860;
pub const RFPGA0_XB_RFINTERFACEOE: c_uint = 0x864;
pub const RFPGA0_XAB_RFINTERFACESW: c_uint = 0x870;
pub const RFPGA0_XCD_RFINTERFACESW: c_uint = 0x874;
pub const RFPGA0_XAB_RFPARAMETER: c_uint = 0x878;
pub const RFPGA0_XCD_RFPARAMETER: c_uint = 0x87c;
pub const RFPGA0_ANALOGPARAMETER1: c_uint = 0x880;
pub const RFPGA0_ANALOGPARAMETER2: c_uint = 0x884;
pub const RFPGA0_ANALOGPARAMETER3: c_uint = 0x888;
pub const RFPGA0_ANALOGPARAMETER4: c_uint = 0x88c;
pub const RFPGA0_XA_LSSIREADBACK: c_uint = 0x8a0;
pub const RFPGA0_XB_LSSIREADBACK: c_uint = 0x8a4;
pub const RFPGA0_XC_LSSIREADBACK: c_uint = 0x8a8;
pub const RRFMOD: c_uint = 0x8ac;
pub const RHSSIREAD_8821AE: c_uint = 0x8b0;
pub const RFPGA0_PSDREPORT: c_uint = 0x8b4;
pub const TRANSCEIVEA_HSPI_READBACK: c_uint = 0x8b8;
pub const TRANSCEIVEB_HSPI_READBACK: c_uint = 0x8bc;
pub const RADC_BUF_CLK: c_uint = 0x8c4;
pub const RFPGA0_XAB_RFINTERFACERB: c_uint = 0x8e0;
pub const RFPGA0_XCD_RFINTERFACERB: c_uint = 0x8e4;
pub const RFPGA1_RFMOD: c_uint = 0x900;
pub const RFPGA1_TXBLOCK: c_uint = 0x904;
pub const RFPGA1_DEBUGSELECT: c_uint = 0x908;
pub const RFPGA1_TXINFO: c_uint = 0x90c;
pub const RCCK_SYSTEM: c_uint = 0xa00;
pub const BCCK_SYSTEM: c_uint = 0x10;
pub const RCCK0_AFESETTING: c_uint = 0xa04;
pub const RCCK0_CCA: c_uint = 0xa08;
pub const RCCK0_RXAGC1: c_uint = 0xa0c;
pub const RCCK0_RXAGC2: c_uint = 0xa10;
pub const RCCK0_RXHP: c_uint = 0xa14;
pub const RCCK0_DSPPARAMETER1: c_uint = 0xa18;
pub const RCCK0_DSPPARAMETER2: c_uint = 0xa1c;
pub const RCCK0_TXFILTER1: c_uint = 0xa20;
pub const RCCK0_TXFILTER2: c_uint = 0xa24;
pub const RCCK0_DEBUGPORT: c_uint = 0xa28;
pub const RCCK0_FALSEALARMREPORT: c_uint = 0xa2c;
pub const RCCK0_TRSSIREPORT: c_uint = 0xa50;
pub const RCCK0_RXREPORT: c_uint = 0xa54;
pub const RCCK0_FACOUNTERLOWER: c_uint = 0xa5c;
pub const RCCK0_FACOUNTERUPPER: c_uint = 0xa58;
pub const RCCK0_CCA_CNT: c_uint = 0xa60;
// PageB(0XB00)
pub const RPDP_ANTA: c_uint = 0xb00;
pub const RPDP_ANTA_4: c_uint = 0xb04;
pub const RPDP_ANTA_8: c_uint = 0xb08;
pub const RPDP_ANTA_C: c_uint = 0xb0c;
pub const RPDP_ANTA_10: c_uint = 0xb10;
pub const RPDP_ANTA_14: c_uint = 0xb14;
pub const RPDP_ANTA_18: c_uint = 0xb18;
pub const RPDP_ANTA_1C: c_uint = 0xb1c;
pub const RPDP_ANTA_20: c_uint = 0xb20;
pub const RPDP_ANTA_24: c_uint = 0xb24;
pub const RCONFIG_PMPD_ANTA: c_uint = 0xb28;
pub const RCONFIG_RAM64x16: c_uint = 0xb2c;
pub const RBNDA: c_uint = 0xb30;
pub const RHSSIPAR: c_uint = 0xb34;
pub const RCONFIG_ANTA: c_uint = 0xb68;
pub const RCONFIG_ANTB: c_uint = 0xb6c;
pub const RPDP_ANTB: c_uint = 0xb70;
pub const RPDP_ANTB_4: c_uint = 0xb74;
pub const RPDP_ANTB_8: c_uint = 0xb78;
pub const RPDP_ANTB_C: c_uint = 0xb7c;
pub const RPDP_ANTB_10: c_uint = 0xb80;
pub const RPDP_ANTB_14: c_uint = 0xb84;
pub const RPDP_ANTB_18: c_uint = 0xb88;
pub const RPDP_ANTB_1C: c_uint = 0xb8c;
pub const RPDP_ANTB_20: c_uint = 0xb90;
pub const RPDP_ANTB_24: c_uint = 0xb94;
pub const RCONFIG_PMPD_ANTB: c_uint = 0xb98;
pub const RBNDB: c_uint = 0xba0;
pub const RAPK: c_uint = 0xbd8;
pub const RPM_RX0_ANTA: c_uint = 0xbdc;
pub const RPM_RX1_ANTA: c_uint = 0xbe0;
pub const RPM_RX2_ANTA: c_uint = 0xbe4;
pub const RPM_RX3_ANTA: c_uint = 0xbe8;
pub const RPM_RX0_ANTB: c_uint = 0xbec;
pub const RPM_RX1_ANTB: c_uint = 0xbf0;
pub const RPM_RX2_ANTB: c_uint = 0xbf4;
pub const RPM_RX3_ANTB: c_uint = 0xbf8;
// RSSI Dump

// Page C
pub const ROFDM0_LSTF: c_uint = 0xc00;
pub const ROFDM0_TRXPATHENABLE: c_uint = 0xc04;
pub const ROFDM0_TRMUXPAR: c_uint = 0xc08;
pub const ROFDM0_TRSWISOLATION: c_uint = 0xc0c;
pub const ROFDM0_XARXAFE: c_uint = 0xc10;
pub const ROFDM0_XARXIQIMBALANCE: c_uint = 0xc14;
pub const ROFDM0_XBRXAFE: c_uint = 0xc18;
pub const ROFDM0_XBRXIQIMBALANCE: c_uint = 0xc1c;
pub const ROFDM0_XCRXAFE: c_uint = 0xc20;
pub const ROFDM0_XCRXIQIMBANLANCE: c_uint = 0xc24;
pub const ROFDM0_XDRXAFE: c_uint = 0xc28;
pub const ROFDM0_XDRXIQIMBALANCE: c_uint = 0xc2c;
pub const ROFDM0_RXDETECTOR1: c_uint = 0xc30;
pub const ROFDM0_RXDETECTOR2: c_uint = 0xc34;
pub const ROFDM0_RXDETECTOR3: c_uint = 0xc38;
pub const ROFDM0_RXDETECTOR4: c_uint = 0xc3c;
pub const ROFDM0_RXDSP: c_uint = 0xc40;
pub const ROFDM0_CFOANDDAGC: c_uint = 0xc44;
pub const ROFDM0_CCADROPTHRESHOLD: c_uint = 0xc48;
pub const ROFDM0_ECCATHRESHOLD: c_uint = 0xc4c;
pub const ROFDM0_XAAGCCORE1: c_uint = 0xc50;
pub const ROFDM0_XAAGCCORE2: c_uint = 0xc54;
pub const ROFDM0_XBAGCCORE1: c_uint = 0xc58;
pub const ROFDM0_XBAGCCORE2: c_uint = 0xc5c;
pub const ROFDM0_XCAGCCORE1: c_uint = 0xc60;
pub const ROFDM0_XCAGCCORE2: c_uint = 0xc64;
pub const ROFDM0_XDAGCCORE1: c_uint = 0xc68;
pub const ROFDM0_XDAGCCORE2: c_uint = 0xc6c;
pub const ROFDM0_AGCPARAMETER1: c_uint = 0xc70;
pub const ROFDM0_AGCPARAMETER2: c_uint = 0xc74;
pub const ROFDM0_AGCRSSITABLE: c_uint = 0xc78;
pub const ROFDM0_HTSTFAGC: c_uint = 0xc7c;
pub const ROFDM0_XATXIQIMBALANCE: c_uint = 0xc80;
pub const ROFDM0_XATXAFE: c_uint = 0xc84;
pub const ROFDM0_XBTXIQIMBALANCE: c_uint = 0xc88;
pub const ROFDM0_XBTXAFE: c_uint = 0xc8c;
pub const ROFDM0_XCTXIQIMBALANCE: c_uint = 0xc90;
pub const ROFDM0_XCTXAFE: c_uint = 0xc94;
pub const ROFDM0_XDTXIQIMBALANCE: c_uint = 0xc98;
pub const ROFDM0_XDTXAFE: c_uint = 0xc9c;
pub const ROFDM0_RXIQEXTANTA: c_uint = 0xca0;
pub const ROFDM0_TXCOEFF1: c_uint = 0xca4;
pub const ROFDM0_TXCOEFF2: c_uint = 0xca8;
pub const ROFDM0_TXCOEFF3: c_uint = 0xcac;
pub const ROFDM0_TXCOEFF4: c_uint = 0xcb0;
pub const ROFDM0_TXCOEFF5: c_uint = 0xcb4;
pub const ROFDM0_TXCOEFF6: c_uint = 0xcb8;
// Path_A RFE cotrol
pub const RA_RFE_CTRL_8812: c_uint = 0xcb8;
// Path_B RFE control
pub const RB_RFE_CTRL_8812: c_uint = 0xeb8;
pub const ROFDM0_RXHPPARAMETER: c_uint = 0xce0;
pub const ROFDM0_TXPSEUDONOISEWGT: c_uint = 0xce4;
pub const ROFDM0_FRAMESYNC: c_uint = 0xcf0;
pub const ROFDM0_DFSREPORT: c_uint = 0xcf4;
pub const ROFDM1_LSTF: c_uint = 0xd00;
pub const ROFDM1_TRXPATHENABLE: c_uint = 0xd04;
pub const ROFDM1_CF0: c_uint = 0xd08;
pub const ROFDM1_CSI1: c_uint = 0xd10;
pub const ROFDM1_SBD: c_uint = 0xd14;
pub const ROFDM1_CSI2: c_uint = 0xd18;
pub const ROFDM1_CFOTRACKING: c_uint = 0xd2c;
pub const ROFDM1_TRXMESAURE1: c_uint = 0xd34;
pub const ROFDM1_INTFDET: c_uint = 0xd3c;
pub const ROFDM1_PSEUDONOISESTATEAB: c_uint = 0xd50;
pub const ROFDM1_PSEUDONOISESTATECD: c_uint = 0xd54;
pub const ROFDM1_RXPSEUDONOISEWGT: c_uint = 0xd58;
pub const ROFDM_PHYCOUNTER1: c_uint = 0xda0;
pub const ROFDM_PHYCOUNTER2: c_uint = 0xda4;
pub const ROFDM_PHYCOUNTER3: c_uint = 0xda8;
pub const ROFDM_SHORTCFOAB: c_uint = 0xdac;
pub const ROFDM_SHORTCFOCD: c_uint = 0xdb0;
pub const ROFDM_LONGCFOAB: c_uint = 0xdb4;
pub const ROFDM_LONGCFOCD: c_uint = 0xdb8;
pub const ROFDM_TAILCF0AB: c_uint = 0xdbc;
pub const ROFDM_TAILCF0CD: c_uint = 0xdc0;
pub const ROFDM_PWMEASURE1: c_uint = 0xdc4;
pub const ROFDM_PWMEASURE2: c_uint = 0xdc8;
pub const ROFDM_BWREPORT: c_uint = 0xdcc;
pub const ROFDM_AGCREPORT: c_uint = 0xdd0;
pub const ROFDM_RXSNR: c_uint = 0xdd4;
pub const ROFDM_RXEVMCSI: c_uint = 0xdd8;
pub const ROFDM_SIGREPORT: c_uint = 0xddc;
pub const RTXAGC_A_CCK11_CCK1: c_uint = 0xc20;
pub const RTXAGC_A_OFDM18_OFDM6: c_uint = 0xc24;
pub const RTXAGC_A_OFDM54_OFDM24: c_uint = 0xc28;
pub const RTXAGC_A_MCS03_MCS00: c_uint = 0xc2c;
pub const RTXAGC_A_MCS07_MCS04: c_uint = 0xc30;
pub const RTXAGC_A_MCS11_MCS08: c_uint = 0xc34;
pub const RTXAGC_A_MCS15_MCS12: c_uint = 0xc38;
pub const RTXAGC_A_NSS1INDEX3_NSS1INDEX0: c_uint = 0xc3c;
pub const RTXAGC_A_NSS1INDEX7_NSS1INDEX4: c_uint = 0xc40;
pub const RTXAGC_A_NSS2INDEX1_NSS1INDEX8: c_uint = 0xc44;
pub const RTXAGC_A_NSS2INDEX5_NSS2INDEX2: c_uint = 0xc48;
pub const RTXAGC_A_NSS2INDEX9_NSS2INDEX6: c_uint = 0xc4c;
pub const RTXAGC_B_CCK11_CCK1: c_uint = 0xe20;
pub const RTXAGC_B_OFDM18_OFDM6: c_uint = 0xe24;
pub const RTXAGC_B_OFDM54_OFDM24: c_uint = 0xe28;
pub const RTXAGC_B_MCS03_MCS00: c_uint = 0xe2c;
pub const RTXAGC_B_MCS07_MCS04: c_uint = 0xe30;
pub const RTXAGC_B_MCS11_MCS08: c_uint = 0xe34;
pub const RTXAGC_B_MCS15_MCS12: c_uint = 0xe38;
pub const RTXAGC_B_NSS1INDEX3_NSS1INDEX0: c_uint = 0xe3c;
pub const RTXAGC_B_NSS1INDEX7_NSS1INDEX4: c_uint = 0xe40;
pub const RTXAGC_B_NSS2INDEX1_NSS1INDEX8: c_uint = 0xe44;
pub const RTXAGC_B_NSS2INDEX5_NSS2INDEX2: c_uint = 0xe48;
pub const RTXAGC_B_NSS2INDEX9_NSS2INDEX6: c_uint = 0xe4c;
pub const RA_TXPWRTRAING: c_uint = 0xc54;
pub const RB_TXPWRTRAING: c_uint = 0xe54;
pub const RFPGA0_IQK: c_uint = 0xe28;
pub const RTX_IQK_TONE_A: c_uint = 0xe30;
pub const RRX_IQK_TONE_A: c_uint = 0xe34;
pub const RTX_IQK_PI_A: c_uint = 0xe38;
pub const RRX_IQK_PI_A: c_uint = 0xe3c;
pub const RTX_IQK: c_uint = 0xe40;
pub const RRX_IQK: c_uint = 0xe44;
pub const RIQK_AGC_PTS: c_uint = 0xe48;
pub const RIQK_AGC_RSP: c_uint = 0xe4c;
pub const RTX_IQK_TONE_B: c_uint = 0xe50;
pub const RRX_IQK_TONE_B: c_uint = 0xe54;
pub const RTX_IQK_PI_B: c_uint = 0xe58;
pub const RRX_IQK_PI_B: c_uint = 0xe5c;
pub const RIQK_AGC_CONT: c_uint = 0xe60;
pub const RBLUE_TOOTH: c_uint = 0xe6c;
pub const RRX_WAIT_CCA: c_uint = 0xe70;
pub const RTX_CCK_RFON: c_uint = 0xe74;
pub const RTX_CCK_BBON: c_uint = 0xe78;
pub const RTX_OFDM_RFON: c_uint = 0xe7c;
pub const RTX_OFDM_BBON: c_uint = 0xe80;
pub const RTX_TO_RX: c_uint = 0xe84;
pub const RTX_TO_TX: c_uint = 0xe88;
pub const RRX_CCK: c_uint = 0xe8c;
pub const RTX_POWER_BEFORE_IQK_A: c_uint = 0xe94;
pub const RTX_POWER_AFTER_IQK_A: c_uint = 0xe9c;
pub const RRX_POWER_BEFORE_IQK_A: c_uint = 0xea0;
pub const RRX_POWER_BEFORE_IQK_A_2: c_uint = 0xea4;
pub const RRX_POWER_AFTER_IQK_A: c_uint = 0xea8;
pub const RRX_POWER_AFTER_IQK_A_2: c_uint = 0xeac;
pub const RTX_POWER_BEFORE_IQK_B: c_uint = 0xeb4;
pub const RTX_POWER_AFTER_IQK_B: c_uint = 0xebc;
pub const RRX_POER_BEFORE_IQK_B: c_uint = 0xec0;
pub const RRX_POER_BEFORE_IQK_B_2: c_uint = 0xec4;
pub const RRX_POWER_AFTER_IQK_B: c_uint = 0xec8;
pub const RRX_POWER_AFTER_IQK_B_2: c_uint = 0xecc;
pub const RRX_OFDM: c_uint = 0xed0;
pub const RRX_WAIT_RIFS: c_uint = 0xed4;
pub const RRX_TO_RX: c_uint = 0xed8;
pub const RSTANDBY: c_uint = 0xedc;
pub const RSLEEP: c_uint = 0xee0;
pub const RPMPD_ANAEN: c_uint = 0xeec;
pub const RZEBRA1_HSSIENABLE: c_uint = 0x0;
pub const RZEBRA1_TRXENABLE1: c_uint = 0x1;
pub const RZEBRA1_TRXENABLE2: c_uint = 0x2;
pub const RZEBRA1_AGC: c_uint = 0x4;
pub const RZEBRA1_CHARGEPUMP: c_uint = 0x5;
pub const RZEBRA1_CHANNEL: c_uint = 0x7;
pub const RZEBRA1_TXGAIN: c_uint = 0x8;
pub const RZEBRA1_TXLPF: c_uint = 0x9;
pub const RZEBRA1_RXLPF: c_uint = 0xb;
pub const RZEBRA1_RXHPFCORNER: c_uint = 0xc;
pub const RGLOBALCTRL: c_int = 0;
pub const RRTL8256_TXLPF: c_int = 19;
pub const RRTL8256_RXLPF: c_int = 11;
pub const RRTL8258_TXLPF: c_uint = 0x11;
pub const RRTL8258_RXLPF: c_uint = 0x13;
pub const RRTL8258_RSSILPF: c_uint = 0xa;
pub const RF_AC: c_uint = 0x00;
pub const RF_IQADJ_G1: c_uint = 0x01;
pub const RF_IQADJ_G2: c_uint = 0x02;
pub const RF_POW_TRSW: c_uint = 0x05;
pub const RF_GAIN_RX: c_uint = 0x06;
pub const RF_GAIN_TX: c_uint = 0x07;
pub const RF_TXM_IDAC: c_uint = 0x08;
pub const RF_BS_IQGEN: c_uint = 0x0F;
pub const RF_MODE1: c_uint = 0x10;
pub const RF_MODE2: c_uint = 0x11;
pub const RF_RX_AGC_HP: c_uint = 0x12;
pub const RF_TX_AGC: c_uint = 0x13;
pub const RF_BIAS: c_uint = 0x14;
pub const RF_IPA: c_uint = 0x15;
pub const RF_POW_ABILITY: c_uint = 0x17;
pub const RF_MODE_AG: c_uint = 0x18;
pub const RRFCHANNEL: c_uint = 0x18;
pub const RF_CHNLBW: c_uint = 0x18;
pub const RF_TOP: c_uint = 0x19;
pub const RF_RX_G1: c_uint = 0x1A;
pub const RF_RX_G2: c_uint = 0x1B;
pub const RF_RX_BB2: c_uint = 0x1C;
pub const RF_RX_BB1: c_uint = 0x1D;
pub const RF_RCK1: c_uint = 0x1E;
pub const RF_RCK2: c_uint = 0x1F;
pub const RF_TX_G1: c_uint = 0x20;
pub const RF_TX_G2: c_uint = 0x21;
pub const RF_TX_G3: c_uint = 0x22;
pub const RF_TX_BB1: c_uint = 0x23;
pub const RF_T_METER: c_uint = 0x24;
pub const RF_T_METER_88E: c_uint = 0x42;
pub const RF_T_METER_8812A: c_uint = 0x42;
pub const RF_SYN_G1: c_uint = 0x25;
pub const RF_SYN_G2: c_uint = 0x26;
pub const RF_SYN_G3: c_uint = 0x27;
pub const RF_SYN_G4: c_uint = 0x28;
pub const RF_SYN_G5: c_uint = 0x29;
pub const RF_SYN_G6: c_uint = 0x2A;
pub const RF_SYN_G7: c_uint = 0x2B;
pub const RF_SYN_G8: c_uint = 0x2C;
pub const RF_RCK_OS: c_uint = 0x30;
pub const RF_TXPA_G1: c_uint = 0x31;
pub const RF_TXPA_G2: c_uint = 0x32;
pub const RF_TXPA_G3: c_uint = 0x33;
pub const RF_TX_BIAS_A: c_uint = 0x35;
pub const RF_TX_BIAS_D: c_uint = 0x36;
pub const RF_LOBF_9: c_uint = 0x38;
pub const RF_RXRF_A3: c_uint = 0x3C;
pub const RF_TRSW: c_uint = 0x3F;
pub const RF_TXRF_A2: c_uint = 0x41;
pub const RF_TXPA_G4: c_uint = 0x46;
pub const RF_TXPA_A4: c_uint = 0x4B;
pub const RF_APK: c_uint = 0x63;
pub const RF_WE_LUT: c_uint = 0xEF;
pub const BBBRESETB: c_uint = 0x100;
pub const BGLOBALRESETB: c_uint = 0x200;
pub const BOFDMTXSTART: c_uint = 0x4;
pub const BCCKTXSTART: c_uint = 0x8;
pub const BCRC32DEBUG: c_uint = 0x100;
pub const BPMACLOOPBACK: c_uint = 0x10;
pub const BTXLSIG: c_uint = 0xffffff;
pub const BOFDMTXRATE: c_uint = 0xf;
pub const BOFDMTXRESERVED: c_uint = 0x10;
pub const BOFDMTXLENGTH: c_uint = 0x1ffe0;
pub const BOFDMTXPARITY: c_uint = 0x20000;
pub const BTXHTSIG1: c_uint = 0xffffff;
pub const BTXHTMCSRATE: c_uint = 0x7f;
pub const BTXHTBW: c_uint = 0x80;
pub const BTXHTLENGTH: c_uint = 0xffff00;
pub const BTXHTSIG2: c_uint = 0xffffff;
pub const BTXHTSMOOTHING: c_uint = 0x1;
pub const BTXHTSOUNDING: c_uint = 0x2;
pub const BTXHTRESERVED: c_uint = 0x4;
pub const BTXHTAGGREATION: c_uint = 0x8;
pub const BTXHTSTBC: c_uint = 0x30;
pub const BTXHTADVANCECODING: c_uint = 0x40;
pub const BTXHTSHORTGI: c_uint = 0x80;
pub const BTXHTNUMBERHT_LTF: c_uint = 0x300;
pub const BTXHTCRC8: c_uint = 0x3fc00;
pub const BCOUNTERRESET: c_uint = 0x10000;
pub const BNUMOFOFDMTX: c_uint = 0xffff;
pub const BNUMOFCCKTX: c_uint = 0xffff0000;
pub const BTXIDLEINTERVAL: c_uint = 0xffff;
pub const BOFDMSERVICE: c_uint = 0xffff0000;
pub const BTXMACHEADER: c_uint = 0xffffffff;
pub const BTXDATAINIT: c_uint = 0xff;
pub const BTXHTMODE: c_uint = 0x100;
pub const BTXDATATYPE: c_uint = 0x30000;
pub const BTXRANDOMSEED: c_uint = 0xffffffff;
pub const BCCKTXPREAMBLE: c_uint = 0x1;
pub const BCCKTXSFD: c_uint = 0xffff0000;
pub const BCCKTXSIG: c_uint = 0xff;
pub const BCCKTXSERVICE: c_uint = 0xff00;
pub const BCCKLENGTHEXT: c_uint = 0x8000;
pub const BCCKTXLENGHT: c_uint = 0xffff0000;
pub const BCCKTXCRC16: c_uint = 0xffff;
pub const BCCKTXSTATUS: c_uint = 0x1;
pub const BOFDMTXSTATUS: c_uint = 0x2;

pub const BRFMOD: c_uint = 0x1;
pub const BJAPANMODE: c_uint = 0x2;
pub const BCCKTXSC: c_uint = 0x30;
// Block & Path enable
pub const ROFDMCCKEN: c_uint = 0x808;
pub const BCCKEN: c_uint = 0x10000000;
pub const BOFDMEN: c_uint = 0x20000000;
// Rx antenna
pub const RRXPATH: c_uint = 0x808;
pub const BRXPATH: c_uint = 0xff;
// Tx antenna
pub const RTXPATH: c_uint = 0x80c;
pub const BTXPATH: c_uint = 0x0fffffff;
// for cck rx path selection
pub const RCCK_RX: c_uint = 0xa04;
pub const BCCK_RX: c_uint = 0x0c000000;
// Use LSIG for VHT length
pub const RVHTLEN_USE_LSIG: c_uint = 0x8c3;
pub const BOFDMRXADCPHASE: c_uint = 0x10000;
pub const BOFDMTXDACPHASE: c_uint = 0x40000;
pub const BXATXAGC: c_uint = 0x3f;
pub const BXBTXAGC: c_uint = 0xf00;
pub const BXCTXAGC: c_uint = 0xf000;
pub const BXDTXAGC: c_uint = 0xf0000;
pub const BPASTART: c_uint = 0xf0000000;
pub const BTRSTART: c_uint = 0x00f00000;
pub const BRFSTART: c_uint = 0x0000f000;
pub const BBBSTART: c_uint = 0x000000f0;
pub const BBBCCKSTART: c_uint = 0x0000000f;
pub const BPAEND: c_uint = 0xf;
pub const BTREND: c_uint = 0x0f000000;
pub const BRFEND: c_uint = 0x000f0000;
pub const BCCAMASK: c_uint = 0x000000f0;
pub const BR2RCCAMASK: c_uint = 0x00000f00;
pub const BHSSI_R2TDELAY: c_uint = 0xf8000000;
pub const BHSSI_T2RDELAY: c_uint = 0xf80000;
pub const BCONTXHSSI: c_uint = 0x400;
pub const BIGFROMCCK: c_uint = 0x200;
pub const BAGCADDRESS: c_uint = 0x3f;
pub const BRXHPTX: c_uint = 0x7000;
pub const BRXHP2RX: c_uint = 0x38000;
pub const BRXHPCCKINI: c_uint = 0xc0000;
pub const BAGCTXCODE: c_uint = 0xc00000;
pub const BAGCRXCODE: c_uint = 0x300000;
pub const B3WIREDATALENGTH: c_uint = 0x800;
pub const B3WIREADDREAALENGTH: c_uint = 0x400;
pub const B3WIRERFPOWERDOWN: c_uint = 0x1;
pub const B5GPAPEPOLARITY: c_uint = 0x40000000;
pub const B2GPAPEPOLARITY: c_uint = 0x80000000;
pub const BRFSW_TXDEFAULTANT: c_uint = 0x3;
pub const BRFSW_TXOPTIONANT: c_uint = 0x30;
pub const BRFSW_RXDEFAULTANT: c_uint = 0x300;
pub const BRFSW_RXOPTIONANT: c_uint = 0x3000;
pub const BRFSI_3WIREDATA: c_uint = 0x1;
pub const BRFSI_3WIRECLOCK: c_uint = 0x2;
pub const BRFSI_3WIRELOAD: c_uint = 0x4;
pub const BRFSI_3WIRERW: c_uint = 0x8;
pub const BRFSI_3WIRE: c_uint = 0xf;
pub const BRFSI_RFENV: c_uint = 0x10;
pub const BRFSI_TRSW: c_uint = 0x20;
pub const BRFSI_TRSWB: c_uint = 0x40;
pub const BRFSI_ANTSW: c_uint = 0x100;
pub const BRFSI_ANTSWB: c_uint = 0x200;
pub const BRFSI_PAPE: c_uint = 0x400;
pub const BRFSI_PAPE5G: c_uint = 0x800;
pub const BBANDSELECT: c_uint = 0x1;
pub const BHTSIG2_GI: c_uint = 0x80;
pub const BHTSIG2_SMOOTHING: c_uint = 0x01;
pub const BHTSIG2_SOUNDING: c_uint = 0x02;
pub const BHTSIG2_AGGREATON: c_uint = 0x08;
pub const BHTSIG2_STBC: c_uint = 0x30;
pub const BHTSIG2_ADVCODING: c_uint = 0x40;
pub const BHTSIG2_NUMOFHTLTF: c_uint = 0x300;
pub const BHTSIG2_CRC8: c_uint = 0x3fc;
pub const BHTSIG1_MCS: c_uint = 0x7f;
pub const BHTSIG1_BANDWIDTH: c_uint = 0x80;
pub const BHTSIG1_HTLENGTH: c_uint = 0xffff;
pub const BLSIG_RATE: c_uint = 0xf;
pub const BLSIG_RESERVED: c_uint = 0x10;
pub const BLSIG_LENGTH: c_uint = 0x1fffe;
pub const BLSIG_PARITY: c_uint = 0x20;
pub const BCCKRXPHASE: c_uint = 0x4;
pub const BLSSIREADADDRESS: c_uint = 0x7f800000;
pub const BLSSIREADEDGE: c_uint = 0x80000000;
pub const BLSSIREADBACKDATA: c_uint = 0xfffff;
pub const BLSSIREADOKFLAG: c_uint = 0x1000;
pub const BCCKSAMPLERATE: c_uint = 0x8;
pub const BREGULATOR0STANDBY: c_uint = 0x1;
pub const BREGULATORPLLSTANDBY: c_uint = 0x2;
pub const BREGULATOR1STANDBY: c_uint = 0x4;
pub const BPLLPOWERUP: c_uint = 0x8;
pub const BDPLLPOWERUP: c_uint = 0x10;
pub const BDA10POWERUP: c_uint = 0x20;
pub const BAD7POWERUP: c_uint = 0x200;
pub const BDA6POWERUP: c_uint = 0x2000;
pub const BXTALPOWERUP: c_uint = 0x4000;
pub const B40MDCLKPOWERUP: c_uint = 0x8000;
pub const BDA6DEBUGMODE: c_uint = 0x20000;
pub const BDA6SWING: c_uint = 0x380000;
pub const BADCLKPHASE: c_uint = 0x4000000;
pub const B80MCLKDELAY: c_uint = 0x18000000;
pub const BAFEWATCHDOGENABLE: c_uint = 0x20000000;
pub const BXTALCAP01: c_uint = 0xc0000000;
pub const BXTALCAP23: c_uint = 0x3;
pub const BXTALCAP92X: c_uint = 0x0f000000;
pub const BXTALCAP: c_uint = 0x0f000000;
pub const BINTDIFCLKENABLE: c_uint = 0x400;
pub const BEXTSIGCLKENABLE: c_uint = 0x800;
pub const BBANDGAP_MBIAS_POWERUP: c_uint = 0x10000;
pub const BAD11SH_GAIN: c_uint = 0xc0000;
pub const BAD11NPUT_RANGE: c_uint = 0x700000;
pub const BAD110P_CURRENT: c_uint = 0x3800000;
pub const BLPATH_LOOPBACK: c_uint = 0x4000000;
pub const BQPATH_LOOPBACK: c_uint = 0x8000000;
pub const BAFE_LOOPBACK: c_uint = 0x10000000;
pub const BDA10_SWING: c_uint = 0x7e0;
pub const BDA10_REVERSE: c_uint = 0x800;
pub const BDA_CLK_SOURCE: c_uint = 0x1000;
pub const BDA7INPUT_RANGE: c_uint = 0x6000;
pub const BDA7_GAIN: c_uint = 0x38000;
pub const BDA7OUTPUT_CM_MODE: c_uint = 0x40000;
pub const BDA7INPUT_CM_MODE: c_uint = 0x380000;
pub const BDA7CURRENT: c_uint = 0xc00000;
pub const BREGULATOR_ADJUST: c_uint = 0x7000000;
pub const BAD11POWERUP_ATTX: c_uint = 0x1;
pub const BDA10PS_ATTX: c_uint = 0x10;
pub const BAD11POWERUP_ATRX: c_uint = 0x100;
pub const BDA10PS_ATRX: c_uint = 0x1000;
pub const BCCKRX_AGC_FORMAT: c_uint = 0x200;
pub const BPSDFFT_SAMPLE_POINT: c_uint = 0xc000;
pub const BPSD_AVERAGE_NUM: c_uint = 0x3000;
pub const BIQPATH_CONTROL: c_uint = 0xc00;
pub const BPSD_FREQ: c_uint = 0x3ff;
pub const BPSD_ANTENNA_PATH: c_uint = 0x30;
pub const BPSD_IQ_SWITCH: c_uint = 0x40;
pub const BPSD_RX_TRIGGER: c_uint = 0x400000;
pub const BPSD_TX_TRIGGER: c_uint = 0x80000000;
pub const BPSD_SINE_TONE_SCALE: c_uint = 0x7f000000;
pub const BPSD_REPORT: c_uint = 0xffff;
pub const BOFDM_TXSC: c_uint = 0x30000000;
pub const BCCK_TXON: c_uint = 0x1;
pub const BOFDM_TXON: c_uint = 0x2;
pub const BDEBUG_PAGE: c_uint = 0xfff;
pub const BDEBUG_ITEM: c_uint = 0xff;
pub const BANTL: c_uint = 0x10;
pub const BANT_NONHT: c_uint = 0x100;
pub const BANT_HT1: c_uint = 0x1000;
pub const BANT_HT2: c_uint = 0x10000;
pub const BANT_HT1S1: c_uint = 0x100000;
pub const BANT_NONHTS1: c_uint = 0x1000000;
pub const BCCK_BBMODE: c_uint = 0x3;
pub const BCCK_TXPOWERSAVING: c_uint = 0x80;
pub const BCCK_RXPOWERSAVING: c_uint = 0x40;
pub const BCCK_SIDEBAND: c_uint = 0x10;
pub const BCCK_SCRAMBLE: c_uint = 0x8;
pub const BCCK_ANTDIVERSITY: c_uint = 0x8000;
pub const BCCK_CARRIER_RECOVERY: c_uint = 0x4000;
pub const BCCK_TXRATE: c_uint = 0x3000;
pub const BCCK_DCCANCEL: c_uint = 0x0800;
pub const BCCK_ISICANCEL: c_uint = 0x0400;
pub const BCCK_MATCH_FILTER: c_uint = 0x0200;
pub const BCCK_EQUALIZER: c_uint = 0x0100;
pub const BCCK_PREAMBLE_DETECT: c_uint = 0x800000;
pub const BCCK_FAST_FALSECCA: c_uint = 0x400000;
pub const BCCK_CH_ESTSTART: c_uint = 0x300000;
pub const BCCK_CCA_COUNT: c_uint = 0x080000;
pub const BCCK_CS_LIM: c_uint = 0x070000;
pub const BCCK_BIST_MODE: c_uint = 0x80000000;
pub const BCCK_CCAMASK: c_uint = 0x40000000;
pub const BCCK_TX_DAC_PHASE: c_uint = 0x4;
pub const BCCK_RX_ADC_PHASE: c_uint = 0x20000000;
pub const BCCKR_CP_MODE: c_uint = 0x0100;
pub const BCCK_TXDC_OFFSET: c_uint = 0xf0;
pub const BCCK_RXDC_OFFSET: c_uint = 0xf;
pub const BCCK_CCA_MODE: c_uint = 0xc000;
pub const BCCK_FALSECS_LIM: c_uint = 0x3f00;
pub const BCCK_CS_RATIO: c_uint = 0xc00000;
pub const BCCK_CORGBIT_SEL: c_uint = 0x300000;
pub const BCCK_PD_LIM: c_uint = 0x0f0000;
pub const BCCK_NEWCCA: c_uint = 0x80000000;
pub const BCCK_RXHP_OF_IG: c_uint = 0x8000;
pub const BCCK_RXIG: c_uint = 0x7f00;
pub const BCCK_LNA_POLARITY: c_uint = 0x800000;
pub const BCCK_RX1ST_BAIN: c_uint = 0x7f0000;
pub const BCCK_RF_EXTEND: c_uint = 0x20000000;
pub const BCCK_RXAGC_SATLEVEL: c_uint = 0x1f000000;
pub const BCCK_RXAGC_SATCOUNT: c_uint = 0xe0;
pub const BCCKRXRFSETTLE: c_uint = 0x1f;
pub const BCCK_FIXED_RXAGC: c_uint = 0x8000;
pub const BCCK_ANTENNA_POLARITY: c_uint = 0x2000;
pub const BCCK_TXFILTER_TYPE: c_uint = 0x0c00;
pub const BCCK_RXAGC_REPORTTYPE: c_uint = 0x0300;
pub const BCCK_RXDAGC_EN: c_uint = 0x80000000;
pub const BCCK_RXDAGC_PERIOD: c_uint = 0x20000000;
pub const BCCK_RXDAGC_SATLEVEL: c_uint = 0x1f000000;
pub const BCCK_TIMING_RECOVERY: c_uint = 0x800000;
pub const BCCK_TXC0: c_uint = 0x3f0000;
pub const BCCK_TXC1: c_uint = 0x3f000000;
pub const BCCK_TXC2: c_uint = 0x3f;
pub const BCCK_TXC3: c_uint = 0x3f00;
pub const BCCK_TXC4: c_uint = 0x3f0000;
pub const BCCK_TXC5: c_uint = 0x3f000000;
pub const BCCK_TXC6: c_uint = 0x3f;
pub const BCCK_TXC7: c_uint = 0x3f00;
pub const BCCK_DEBUGPORT: c_uint = 0xff0000;
pub const BCCK_DAC_DEBUG: c_uint = 0x0f000000;
pub const BCCK_FALSEALARM_ENABLE: c_uint = 0x8000;
pub const BCCK_FALSEALARM_READ: c_uint = 0x4000;
pub const BCCK_TRSSI: c_uint = 0x7f;
pub const BCCK_RXAGC_REPORT: c_uint = 0xfe;
pub const BCCK_RXREPORT_ANTSEL: c_uint = 0x80000000;
pub const BCCK_RXREPORT_MFOFF: c_uint = 0x40000000;
pub const BCCK_RXREPORT_SQLOSS: c_uint = 0x20000000;
pub const BCCK_RXREPORT_PKTLOSS: c_uint = 0x10000000;
pub const BCCK_RXREPORT_LOCKEDBIT: c_uint = 0x08000000;
pub const BCCK_RXREPORT_RATEERROR: c_uint = 0x04000000;
pub const BCCK_RXREPORT_RXRATE: c_uint = 0x03000000;
pub const BCCK_RXFA_COUNTER_LOWER: c_uint = 0xff;
pub const BCCK_RXFA_COUNTER_UPPER: c_uint = 0xff000000;
pub const BCCK_RXHPAGC_START: c_uint = 0xe000;
pub const BCCK_RXHPAGC_FINAL: c_uint = 0x1c00;
pub const BCCK_RXFALSEALARM_ENABLE: c_uint = 0x8000;
pub const BCCK_FACOUNTER_FREEZE: c_uint = 0x4000;
pub const BCCK_TXPATH_SEL: c_uint = 0x10000000;
pub const BCCK_DEFAULT_RXPATH: c_uint = 0xc000000;
pub const BCCK_OPTION_RXPATH: c_uint = 0x3000000;
pub const BNUM_OFSTF: c_uint = 0x3;
pub const BSHIFT_L: c_uint = 0xc0;
pub const BGI_TH: c_uint = 0xc;
pub const BRXPATH_A: c_uint = 0x1;
pub const BRXPATH_B: c_uint = 0x2;
pub const BRXPATH_C: c_uint = 0x4;
pub const BRXPATH_D: c_uint = 0x8;
pub const BTXPATH_A: c_uint = 0x1;
pub const BTXPATH_B: c_uint = 0x2;
pub const BTXPATH_C: c_uint = 0x4;
pub const BTXPATH_D: c_uint = 0x8;
pub const BTRSSI_FREQ: c_uint = 0x200;
pub const BADC_BACKOFF: c_uint = 0x3000;
pub const BDFIR_BACKOFF: c_uint = 0xc000;
pub const BTRSSI_LATCH_PHASE: c_uint = 0x10000;
pub const BRX_LDC_OFFSET: c_uint = 0xff;
pub const BRX_QDC_OFFSET: c_uint = 0xff00;
pub const BRX_DFIR_MODE: c_uint = 0x1800000;
pub const BRX_DCNF_TYPE: c_uint = 0xe000000;
pub const BRXIQIMB_A: c_uint = 0x3ff;
pub const BRXIQIMB_B: c_uint = 0xfc00;
pub const BRXIQIMB_C: c_uint = 0x3f0000;
pub const BRXIQIMB_D: c_uint = 0xffc00000;
pub const BDC_DC_NOTCH: c_uint = 0x60000;
pub const BRXNB_NOTCH: c_uint = 0x1f000000;
pub const BPD_TH: c_uint = 0xf;
pub const BPD_TH_OPT2: c_uint = 0xc000;
pub const BPWED_TH: c_uint = 0x700;
pub const BIFMF_WIN_L: c_uint = 0x800;
pub const BPD_OPTION: c_uint = 0x1000;
pub const BMF_WIN_L: c_uint = 0xe000;
pub const BBW_SEARCH_L: c_uint = 0x30000;
pub const BWIN_ENH_L: c_uint = 0xc0000;
pub const BBW_TH: c_uint = 0x700000;
pub const BED_TH2: c_uint = 0x3800000;
pub const BBW_OPTION: c_uint = 0x4000000;
pub const BRADIO_TH: c_uint = 0x18000000;
pub const BWINDOW_L: c_uint = 0xe0000000;
pub const BSBD_OPTION: c_uint = 0x1;
pub const BFRAME_TH: c_uint = 0x1c;
pub const BFS_OPTION: c_uint = 0x60;
pub const BDC_SLOPE_CHECK: c_uint = 0x80;
pub const BFGUARD_COUNTER_DC_L: c_uint = 0xe00;
pub const BFRAME_WEIGHT_SHORT: c_uint = 0x7000;
pub const BSUB_TUNE: c_uint = 0xe00000;
pub const BFRAME_DC_LENGTH: c_uint = 0xe000000;
pub const BSBD_START_OFFSET: c_uint = 0x30000000;
pub const BFRAME_TH_2: c_uint = 0x7;
pub const BFRAME_GI2_TH: c_uint = 0x38;
pub const BGI2_SYNC_EN: c_uint = 0x40;
pub const BSARCH_SHORT_EARLY: c_uint = 0x300;
pub const BSARCH_SHORT_LATE: c_uint = 0xc00;
pub const BSARCH_GI2_LATE: c_uint = 0x70000;
pub const BCFOANTSUM: c_uint = 0x1;
pub const BCFOACC: c_uint = 0x2;
pub const BCFOSTARTOFFSET: c_uint = 0xc;
pub const BCFOLOOPBACK: c_uint = 0x70;
pub const BCFOSUMWEIGHT: c_uint = 0x80;
pub const BDAGCENABLE: c_uint = 0x10000;
pub const BTXIQIMB_A: c_uint = 0x3ff;
pub const BTXIQIMB_b: c_uint = 0xfc00;
pub const BTXIQIMB_C: c_uint = 0x3f0000;
pub const BTXIQIMB_D: c_uint = 0xffc00000;
pub const BTXIDCOFFSET: c_uint = 0xff;
pub const BTXIQDCOFFSET: c_uint = 0xff00;
pub const BTXDFIRMODE: c_uint = 0x10000;
pub const BTXPESUDO_NOISEON: c_uint = 0x4000000;
pub const BTXPESUDO_NOISE_A: c_uint = 0xff;
pub const BTXPESUDO_NOISE_B: c_uint = 0xff00;
pub const BTXPESUDO_NOISE_C: c_uint = 0xff0000;
pub const BTXPESUDO_NOISE_D: c_uint = 0xff000000;
pub const BCCA_DROPOPTION: c_uint = 0x20000;
pub const BCCA_DROPTHRES: c_uint = 0xfff00000;
pub const BEDCCA_H: c_uint = 0xf;
pub const BEDCCA_L: c_uint = 0xf0;
pub const BLAMBDA_ED: c_uint = 0x300;
pub const BRX_INITIALGAIN: c_uint = 0x7f;
pub const BRX_ANTDIV_EN: c_uint = 0x80;
pub const BRX_AGC_ADDRESS_FOR_LNA: c_uint = 0x7f00;
pub const BRX_HIGHPOWER_FLOW: c_uint = 0x8000;
pub const BRX_AGC_FREEZE_THRES: c_uint = 0xc0000;
pub const BRX_FREEZESTEP_AGC1: c_uint = 0x300000;
pub const BRX_FREEZESTEP_AGC2: c_uint = 0xc00000;
pub const BRX_FREEZESTEP_AGC3: c_uint = 0x3000000;
pub const BRX_FREEZESTEP_AGC0: c_uint = 0xc000000;
pub const BRXRSSI_CMP_EN: c_uint = 0x10000000;
pub const BRXQUICK_AGCEN: c_uint = 0x20000000;
pub const BRXAGC_FREEZE_THRES_MODE: c_uint = 0x40000000;
pub const BRX_OVERFLOW_CHECKTYPE: c_uint = 0x80000000;
pub const BRX_AGCSHIFT: c_uint = 0x7f;
pub const BTRSW_TRI_ONLY: c_uint = 0x80;
pub const BPOWER_THRES: c_uint = 0x300;
pub const BRXAGC_EN: c_uint = 0x1;
pub const BRXAGC_TOGETHER_EN: c_uint = 0x2;
pub const BRXAGC_MIN: c_uint = 0x4;
pub const BRXHP_INI: c_uint = 0x7;
pub const BRXHP_TRLNA: c_uint = 0x70;
pub const BRXHP_RSSI: c_uint = 0x700;
pub const BRXHP_BBP1: c_uint = 0x7000;
pub const BRXHP_BBP2: c_uint = 0x70000;
pub const BRXHP_BBP3: c_uint = 0x700000;
pub const BRSSI_H: c_uint = 0x7f0000;
pub const BRSSI_GEN: c_uint = 0x7f000000;
pub const BRXSETTLE_TRSW: c_uint = 0x7;
pub const BRXSETTLE_LNA: c_uint = 0x38;
pub const BRXSETTLE_RSSI: c_uint = 0x1c0;
pub const BRXSETTLE_BBP: c_uint = 0xe00;
pub const BRXSETTLE_RXHP: c_uint = 0x7000;
pub const BRXSETTLE_ANTSW_RSSI: c_uint = 0x38000;
pub const BRXSETTLE_ANTSW: c_uint = 0xc0000;
pub const BRXPROCESS_TIME_DAGC: c_uint = 0x300000;
pub const BRXSETTLE_HSSI: c_uint = 0x400000;
pub const BRXPROCESS_TIME_BBPPW: c_uint = 0x800000;
pub const BRXANTENNA_POWER_SHIFT: c_uint = 0x3000000;
pub const BRSSI_TABLE_SELECT: c_uint = 0xc000000;
pub const BRXHP_FINAL: c_uint = 0x7000000;
pub const BRXHPSETTLE_BBP: c_uint = 0x7;
pub const BRXHTSETTLE_HSSI: c_uint = 0x8;
pub const BRXHTSETTLE_RXHP: c_uint = 0x70;
pub const BRXHTSETTLE_BBPPW: c_uint = 0x80;
pub const BRXHTSETTLE_IDLE: c_uint = 0x300;
pub const BRXHTSETTLE_RESERVED: c_uint = 0x1c00;
pub const BRXHT_RXHP_EN: c_uint = 0x8000;
pub const BRXAGC_FREEZE_THRES: c_uint = 0x30000;
pub const BRXAGC_TOGETHEREN: c_uint = 0x40000;
pub const BRXHTAGC_MIN: c_uint = 0x80000;
pub const BRXHTAGC_EN: c_uint = 0x100000;
pub const BRXHTDAGC_EN: c_uint = 0x200000;
pub const BRXHT_RXHP_BBP: c_uint = 0x1c00000;
pub const BRXHT_RXHP_FINAL: c_uint = 0xe0000000;
pub const BRXPW_RADIO_TH: c_uint = 0x3;
pub const BRXPW_RADIO_EN: c_uint = 0x4;
pub const BRXMF_HOLD: c_uint = 0x3800;
pub const BRXPD_DELAY_TH1: c_uint = 0x38;
pub const BRXPD_DELAY_TH2: c_uint = 0x1c0;
pub const BRXPD_DC_COUNT_MAX: c_uint = 0x600;
pub const BRXPD_DELAY_TH: c_uint = 0x8000;
pub const BRXPROCESS_DELAY: c_uint = 0xf0000;
pub const BRXSEARCHRANGE_GI2_EARLY: c_uint = 0x700000;
pub const BRXFRAME_FUARD_COUNTER_L: c_uint = 0x3800000;
pub const BRXSGI_GUARD_L: c_uint = 0xc000000;
pub const BRXSGI_SEARCH_L: c_uint = 0x30000000;
pub const BRXSGI_TH: c_uint = 0xc0000000;
pub const BDFSCNT0: c_uint = 0xff;
pub const BDFSCNT1: c_uint = 0xff00;
pub const BDFSFLAG: c_uint = 0xf0000;
pub const BMF_WEIGHT_SUM: c_uint = 0x300000;
pub const BMINIDX_TH: c_uint = 0x7f000000;
pub const BDAFORMAT: c_uint = 0x40000;
pub const BTXCH_EMU_ENABLE: c_uint = 0x01000000;
pub const BTRSW_ISOLATION_A: c_uint = 0x7f;
pub const BTRSW_ISOLATION_B: c_uint = 0x7f00;
pub const BTRSW_ISOLATION_C: c_uint = 0x7f0000;
pub const BTRSW_ISOLATION_D: c_uint = 0x7f000000;
pub const BEXT_LNA_GAIN: c_uint = 0x7c00;
pub const BSTBC_EN: c_uint = 0x4;
pub const BANTENNA_MAPPING: c_uint = 0x10;
pub const BNSS: c_uint = 0x20;
pub const BCFO_ANTSUM_ID: c_uint = 0x200;
pub const BPHY_COUNTER_RESET: c_uint = 0x8000000;
pub const BCFO_REPORT_GET: c_uint = 0x4000000;
pub const BOFDM_CONTINUE_TX: c_uint = 0x10000000;
pub const BOFDM_SINGLE_CARRIER: c_uint = 0x20000000;
pub const BOFDM_SINGLE_TONE: c_uint = 0x40000000;
pub const BHT_DETECT: c_uint = 0x100;
pub const BCFOEN: c_uint = 0x10000;
pub const BCFOVALUE: c_uint = 0xfff00000;
pub const BSIGTONE_RE: c_uint = 0x3f;
pub const BSIGTONE_IM: c_uint = 0x7f00;
pub const BCOUNTER_CCA: c_uint = 0xffff;
pub const BCOUNTER_PARITYFAIL: c_uint = 0xffff0000;
pub const BCOUNTER_RATEILLEGAL: c_uint = 0xffff;
pub const BCOUNTER_CRC8FAIL: c_uint = 0xffff0000;
pub const BCOUNTER_MCSNOSUPPORT: c_uint = 0xffff;
pub const BCOUNTER_FASTSYNC: c_uint = 0xffff;
pub const BSHORTCFO: c_uint = 0xfff;
pub const BSHORTCFOT_LENGTH: c_int = 12;
pub const BSHORTCFOF_LENGTH: c_int = 11;
pub const BLONGCFO: c_uint = 0x7ff;
pub const BLONGCFOT_LENGTH: c_int = 11;
pub const BLONGCFOF_LENGTH: c_int = 11;
pub const BTAILCFO: c_uint = 0x1fff;
pub const BTAILCFOT_LENGTH: c_int = 13;
pub const BTAILCFOF_LENGTH: c_int = 12;
pub const BNOISE_EN_PWDB: c_uint = 0xffff;
pub const BCC_POWER_DB: c_uint = 0xffff0000;
pub const BMOISE_PWDB: c_uint = 0xffff;
pub const BPOWERMEAST_LENGTH: c_int = 10;
pub const BPOWERMEASF_LENGTH: c_int = 3;
pub const BRX_HT_BW: c_uint = 0x1;
pub const BRXSC: c_uint = 0x6;
pub const BRX_HT: c_uint = 0x8;
pub const BNB_INTF_DET_ON: c_uint = 0x1;
pub const BINTF_WIN_LEN_CFG: c_uint = 0x30;
pub const BNB_INTF_TH_CFG: c_uint = 0x1c0;
pub const BRFGAIN: c_uint = 0x3f;
pub const BTABLESEL: c_uint = 0x40;
pub const BTRSW: c_uint = 0x80;
pub const BRXSNR_A: c_uint = 0xff;
pub const BRXSNR_B: c_uint = 0xff00;
pub const BRXSNR_C: c_uint = 0xff0000;
pub const BRXSNR_D: c_uint = 0xff000000;
pub const BSNR_EVMT_LENGTH: c_int = 8;
pub const BSNR_EVMF_LENGTH: c_int = 1;
pub const BCSI1ST: c_uint = 0xff;
pub const BCSI2ND: c_uint = 0xff00;
pub const BRXEVM1ST: c_uint = 0xff0000;
pub const BRXEVM2ND: c_uint = 0xff000000;
pub const BSIGEVM: c_uint = 0xff;
pub const BPWDB: c_uint = 0xff00;
pub const BSGIEN: c_uint = 0x10000;
pub const BSFACTOR_QMA1: c_uint = 0xf;
pub const BSFACTOR_QMA2: c_uint = 0xf0;
pub const BSFACTOR_QMA3: c_uint = 0xf00;
pub const BSFACTOR_QMA4: c_uint = 0xf000;
pub const BSFACTOR_QMA5: c_uint = 0xf0000;
pub const BSFACTOR_QMA6: c_uint = 0xf0000;
pub const BSFACTOR_QMA7: c_uint = 0xf00000;
pub const BSFACTOR_QMA8: c_uint = 0xf000000;
pub const BSFACTOR_QMA9: c_uint = 0xf0000000;
pub const BCSI_SCHEME: c_uint = 0x100000;
pub const BNOISE_LVL_TOP_SET: c_uint = 0x3;
pub const BCHSMOOTH: c_uint = 0x4;
pub const BCHSMOOTH_CFG1: c_uint = 0x38;
pub const BCHSMOOTH_CFG2: c_uint = 0x1c0;
pub const BCHSMOOTH_CFG3: c_uint = 0xe00;
pub const BCHSMOOTH_CFG4: c_uint = 0x7000;
pub const BMRCMODE: c_uint = 0x800000;
pub const BTHEVMCFG: c_uint = 0x7000000;
pub const BLOOP_FIT_TYPE: c_uint = 0x1;
pub const BUPD_CFO: c_uint = 0x40;
pub const BUPD_CFO_OFFDATA: c_uint = 0x80;
pub const BADV_UPD_CFO: c_uint = 0x100;
pub const BADV_TIME_CTRL: c_uint = 0x800;
pub const BUPD_CLKO: c_uint = 0x1000;
pub const BFC: c_uint = 0x6000;
pub const BTRACKING_MODE: c_uint = 0x8000;
pub const BPHCMP_ENABLE: c_uint = 0x10000;
pub const BUPD_CLKO_LTF: c_uint = 0x20000;
pub const BCOM_CH_CFO: c_uint = 0x40000;
pub const BCSI_ESTI_MODE: c_uint = 0x80000;
pub const BADV_UPD_EQZ: c_uint = 0x100000;
pub const BUCHCFG: c_uint = 0x7000000;
pub const BUPDEQZ: c_uint = 0x8000000;
pub const BRX_PESUDO_NOISE_ON: c_uint = 0x20000000;
pub const BRX_PESUDO_NOISE_A: c_uint = 0xff;
pub const BRX_PESUDO_NOISE_B: c_uint = 0xff00;
pub const BRX_PESUDO_NOISE_C: c_uint = 0xff0000;
pub const BRX_PESUDO_NOISE_D: c_uint = 0xff000000;
pub const BRX_PESUDO_NOISESTATE_A: c_uint = 0xffff;
pub const BRX_PESUDO_NOISESTATE_B: c_uint = 0xffff0000;
pub const BRX_PESUDO_NOISESTATE_C: c_uint = 0xffff;
pub const BRX_PESUDO_NOISESTATE_D: c_uint = 0xffff0000;
pub const BZEBRA1_HSSIENABLE: c_uint = 0x8;
pub const BZEBRA1_TRXCONTROL: c_uint = 0xc00;
pub const BZEBRA1_TRXGAINSETTING: c_uint = 0x07f;
pub const BZEBRA1_RXCOUNTER: c_uint = 0xc00;
pub const BZEBRA1_TXCHANGEPUMP: c_uint = 0x38;
pub const BZEBRA1_RXCHANGEPUMP: c_uint = 0x7;
pub const BZEBRA1_CHANNEL_NUM: c_uint = 0xf80;
pub const BZEBRA1_TXLPFBW: c_uint = 0x400;
pub const BZEBRA1_RXLPFBW: c_uint = 0x600;
pub const BRTL8256REG_MODE_CTRL1: c_uint = 0x100;
pub const BRTL8256REG_MODE_CTRL0: c_uint = 0x40;
pub const BRTL8256REG_TXLPFBW: c_uint = 0x18;
pub const BRTL8256REG_RXLPFBW: c_uint = 0x600;
pub const BRTL8258_TXLPFBW: c_uint = 0xc;
pub const BRTL8258_RXLPFBW: c_uint = 0xc00;
pub const BRTL8258_RSSILPFBW: c_uint = 0xc0;
pub const BBYTE0: c_uint = 0x1;
pub const BBYTE1: c_uint = 0x2;
pub const BBYTE2: c_uint = 0x4;
pub const BBYTE3: c_uint = 0x8;
pub const BWORD0: c_uint = 0x3;
pub const BWORD1: c_uint = 0xc;
pub const BWORD: c_uint = 0xf;
pub const MASKBYTE0: c_uint = 0xff;
pub const MASKBYTE1: c_uint = 0xff00;
pub const MASKBYTE2: c_uint = 0xff0000;
pub const MASKBYTE3: c_uint = 0xff000000;
pub const MASKHWORD: c_uint = 0xffff0000;
pub const MASKLWORD: c_uint = 0x0000ffff;
pub const MASKDWORD: c_uint = 0xffffffff;
pub const MASK12BITS: c_uint = 0xfff;
pub const MASKH4BITS: c_uint = 0xf0000000;
pub const MASKOFDM_D: c_uint = 0xffc00000;
pub const MASKCCK: c_uint = 0x3f3f3f3f;
pub const MASK4BITS: c_uint = 0x0f;
pub const MASK20BITS: c_uint = 0xfffff;
pub const RFREG_OFFSET_MASK: c_uint = 0xfffff;
pub const BENABLE: c_uint = 0x1;
pub const BDISABLE: c_uint = 0x0;
pub const LEFT_ANTENNA: c_uint = 0x0;
pub const RIGHT_ANTENNA: c_uint = 0x1;
pub const TCHECK_TXSTATUS: c_int = 500;
pub const TUPDATE_RXCOUNTER: c_int = 100;
pub const REG_UN_used_register: c_uint = 0x01bf;
// Path_A RFE cotrol pinmux
pub const RA_RFE_PINMUX: c_uint = 0xcb0;
// Path_B RFE control pinmux
pub const RB_RFE_PINMUX: c_uint = 0xeb0;
pub const RA_RFE_INV: c_uint = 0xcb4;
pub const RB_RFE_INV: c_uint = 0xeb4;
// RXIQC
// RxIQ imblance matrix coeff. A & B
pub const RA_RXIQC_AB: c_uint = 0xc10;
// RxIQ imblance matrix coeff. C & D
pub const RA_RXIQC_CD: c_uint = 0xc14;
// Pah_A TX scaling factor
pub const RA_TXSCALE: c_uint = 0xc1c;
// Path_B TX scaling factor
pub const RB_TXSCALE: c_uint = 0xe1c;
// RxIQ imblance matrix coeff. A & B
pub const RB_RXIQC_AB: c_uint = 0xe10;
// RxIQ imblance matrix coeff. C & D
pub const RB_RXIQC_CD: c_uint = 0xe14;
// bit mask for IQC matrix element A & C
pub const RXIQC_AC: c_uint = 0x02ff;
// bit mask for IQC matrix element A & C
pub const RXIQC_BD: c_uint = 0x02ff0000;
// 2 EFUSE_TEST (For RTL8723 partially)

pub const EFUSE_SEL_MASK: c_uint = 0x300;
pub const EFUSE_WIFI_SEL_0: c_uint = 0x0;
// REG_MULTI_FUNC_CTRL(For RTL8723 Only)
// Enable GPIO[9] as WiFi HW PDn source

// WiFi HW PDn polarity control

// WiFi function enable

// Enable GPIO[9] as WiFi RF HW PDn source

// Enable GPIO[11] as BT HW PDn source

// BT HW PDn polarity control

// BT function enable

// Enable GPIO[11] as BT/GPS RF HW PDn source

// Enable GPIO[10] as GPS HW PDn source

// GPS HW PDn polarity control

// GPS function enable

pub const BMASKBYTE0: c_uint = 0xff;
pub const BMASKBYTE1: c_uint = 0xff00;
pub const BMASKBYTE2: c_uint = 0xff0000;
pub const BMASKBYTE3: c_uint = 0xff000000;
pub const BMASKHWORD: c_uint = 0xffff0000;
pub const BMASKLWORD: c_uint = 0x0000ffff;
pub const BMASKDWORD: c_uint = 0xffffffff;
pub const BMASK12BITS: c_uint = 0xfff;
pub const BMASKH4BITS: c_uint = 0xf0000000;
pub const BMASKOFDM_D: c_uint = 0xffc00000;
pub const BMASKCCK: c_uint = 0x3f3f3f3f;
pub const BMASKRFEINV: c_uint = 0x3ff00000;
pub const BRFREGOFFSETMASK: c_uint = 0xfffff;
pub const ODM_REG_CCK_RPT_FORMAT_11AC: c_uint = 0x804;
pub const ODM_REG_BB_RX_PATH_11AC: c_uint = 0x808;
// PAGE 9
pub const ODM_REG_OFDM_FA_RST_11AC: c_uint = 0x9A4;
// PAGE A
pub const ODM_REG_CCK_CCA_11AC: c_uint = 0xA0A;
pub const ODM_REG_CCK_FA_RST_11AC: c_uint = 0xA2C;
pub const ODM_REG_CCK_FA_11AC: c_uint = 0xA5C;
// PAGE C
pub const ODM_REG_IGI_A_11AC: c_uint = 0xC50;
// PAGE E
pub const ODM_REG_IGI_B_11AC: c_uint = 0xE50;
// PAGE F
pub const ODM_REG_OFDM_FA_11AC: c_uint = 0xF48;
// 2 MAC REG LIST
// DIG Related
pub const ODM_BIT_IGI_11AC: c_uint = 0xFFFFFFFF;

pub const ODM_BIT_BB_RX_PATH_11AC: c_uint = 0xF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AGGRE_SIZE {
    HT_AGG_SIZE_8K = 0,
    HT_AGG_SIZE_16K = 1,
    HT_AGG_SIZE_32K = 2,
    HT_AGG_SIZE_64K = 3,
    VHT_AGG_SIZE_128K = 4,
    VHT_AGG_SIZE_256K = 5,
    VHT_AGG_SIZE_512K = 6,
    VHT_AGG_SIZE_1024K = 7,
}

pub const REG_AMPDU_MAX_LENGTH_8812: c_uint = 0x0458;
