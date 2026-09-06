//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192d/reg.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
// -----------------------------------------------------
// 0x0000h ~ 0x00FFh System Configuration
// -----------------------------------------------------
pub const REG_SYS_ISO_CTRL: c_uint = 0x0000;
pub const REG_SYS_FUNC_EN: c_uint = 0x0002;
pub const REG_APS_FSMCO: c_uint = 0x0004;
pub const REG_SYS_CLKR: c_uint = 0x0008;
pub const REG_9346CR: c_uint = 0x000A;
pub const REG_EE_VPD: c_uint = 0x000C;
pub const REG_AFE_MISC: c_uint = 0x0010;
pub const REG_SPS0_CTRL: c_uint = 0x0011;
pub const REG_POWER_OFF_IN_PROCESS: c_uint = 0x0017;
pub const REG_SPS_OCP_CFG: c_uint = 0x0018;
pub const REG_RSV_CTRL: c_uint = 0x001C;
pub const REG_RF_CTRL: c_uint = 0x001F;
pub const REG_LDOA15_CTRL: c_uint = 0x0020;
pub const REG_LDOV12D_CTRL: c_uint = 0x0021;
pub const REG_LDOHCI12_CTRL: c_uint = 0x0022;
pub const REG_LPLDO_CTRL: c_uint = 0x0023;
pub const REG_AFE_XTAL_CTRL: c_uint = 0x0024;
pub const REG_AFE_PLL_CTRL: c_uint = 0x0028;
// for 92d, DMDP,SMSP,DMSP contrl
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
pub const REG_MCUFWDL: c_uint = 0x0080;
pub const REG_HMEBOX_EXT_0: c_uint = 0x0088;
pub const REG_HMEBOX_EXT_1: c_uint = 0x008A;
pub const REG_HMEBOX_EXT_2: c_uint = 0x008C;
pub const REG_HMEBOX_EXT_3: c_uint = 0x008E;
pub const SIZE_OF_REG_HMEBOX_EXT: c_int = 2;
pub const REG_EFUSE_ACCESS: c_uint = 0x00CF;
pub const REG_BIST_SCAN: c_uint = 0x00D0;
pub const REG_BIST_RPT: c_uint = 0x00D4;
pub const REG_BIST_ROM_RPT: c_uint = 0x00D8;
pub const REG_USB_SIE_INTF: c_uint = 0x00E0;
pub const REG_PCIE_MIO_INTF: c_uint = 0x00E4;
pub const REG_PCIE_MIO_INTD: c_uint = 0x00E8;
pub const REG_HPON_FSM: c_uint = 0x00EC;
pub const REG_SYS_CFG: c_uint = 0x00F0;
pub const REG_MAC_PHY_CTRL_NORMAL: c_uint = 0x00f8;
pub const REG_MAC0: c_uint = 0x0081;
pub const REG_MAC1: c_uint = 0x0053;
pub const FW_MAC0_READY: c_uint = 0x18;
pub const FW_MAC1_READY: c_uint = 0x1A;

// -----------------------------------------------------
// 0x0100h ~ 0x01FFh	MACTOP General Configuration
// -----------------------------------------------------
pub const REG_CR: c_uint = 0x0100;
pub const REG_PBP: c_uint = 0x0104;
pub const REG_TRXDMA_CTRL: c_uint = 0x010C;
pub const REG_TRXFF_BNDY: c_uint = 0x0114;
pub const REG_TRXFF_STATUS: c_uint = 0x0118;
pub const REG_RXFF_PTR: c_uint = 0x011C;
pub const REG_HIMR: c_uint = 0x0120;
pub const REG_HISR: c_uint = 0x0124;
pub const REG_HIMRE: c_uint = 0x0128;
pub const REG_HISRE: c_uint = 0x012C;
pub const REG_CPWM: c_uint = 0x012F;
pub const REG_FWIMR: c_uint = 0x0130;
pub const REG_FWISR: c_uint = 0x0134;
pub const REG_FTIMR: c_uint = 0x0138;
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
pub const REG_C2HEVT_MSG_NORMAL: c_uint = 0x01A0;
pub const REG_C2HEVT_MSG_TEST: c_uint = 0x01B8;
pub const REG_C2HEVT_CLEAR: c_uint = 0x01BF;
pub const REG_MCUTST_1: c_uint = 0x01c0;
pub const REG_FMETHR: c_uint = 0x01C8;
pub const REG_HMETFR: c_uint = 0x01CC;
pub const REG_HMEBOX_0: c_uint = 0x01D0;
pub const REG_HMEBOX_1: c_uint = 0x01D4;
pub const REG_HMEBOX_2: c_uint = 0x01D8;
pub const REG_HMEBOX_3: c_uint = 0x01DC;
pub const SIZE_OF_REG_HMEBOX: c_int = 4;
pub const REG_LLT_INIT: c_uint = 0x01E0;
pub const REG_BB_ACCEESS_CTRL: c_uint = 0x01E8;
pub const REG_BB_ACCESS_DATA: c_uint = 0x01EC;
// -----------------------------------------------------
// 0x0200h ~ 0x027Fh	TXDMA Configuration
// -----------------------------------------------------
pub const REG_RQPN: c_uint = 0x0200;
pub const REG_FIFOPAGE: c_uint = 0x0204;
pub const REG_TDECTRL: c_uint = 0x0208;
pub const REG_TXDMA_OFFSET_CHK: c_uint = 0x020C;
pub const REG_TXDMA_STATUS: c_uint = 0x0210;
pub const REG_RQPN_NPQ: c_uint = 0x0214;
// -----------------------------------------------------
// 0x0280h ~ 0x02FFh	RXDMA Configuration
// -----------------------------------------------------
pub const REG_RXDMA_AGG_PG_TH: c_uint = 0x0280;
pub const REG_RXPKT_NUM: c_uint = 0x0284;
pub const REG_RXDMA_STATUS: c_uint = 0x0288;
// -----------------------------------------------------
// 0x0300h ~ 0x03FFh	PCIe
// -----------------------------------------------------
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
pub const REG_DBI: c_uint = 0x0348;
pub const REG_DBI_WDATA: c_uint = 0x0348;
pub const REG_DBI_RDATA: c_uint = 0x034C;
pub const REG_DBI_CTRL: c_uint = 0x0350;
pub const REG_DBI_FLAG: c_uint = 0x0352;
pub const REG_MDIO: c_uint = 0x0354;
pub const REG_DBG_SEL: c_uint = 0x0360;
pub const REG_PCIE_HRPWM: c_uint = 0x0361;
pub const REG_PCIE_HCPWM: c_uint = 0x0363;
pub const REG_UART_CTRL: c_uint = 0x0364;
pub const REG_UART_TX_DESA: c_uint = 0x0370;
pub const REG_UART_RX_DESA: c_uint = 0x0378;
// -----------------------------------------------------
// 0x0400h ~ 0x047Fh	Protocol Configuration
// -----------------------------------------------------
pub const REG_VOQ_INFORMATION: c_uint = 0x0400;
pub const REG_VIQ_INFORMATION: c_uint = 0x0404;
pub const REG_BEQ_INFORMATION: c_uint = 0x0408;
pub const REG_BKQ_INFORMATION: c_uint = 0x040C;
pub const REG_MGQ_INFORMATION: c_uint = 0x0410;
pub const REG_HGQ_INFORMATION: c_uint = 0x0414;
pub const REG_BCNQ_INFORMATION: c_uint = 0x0418;
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
pub const REG_ARFR1: c_uint = 0x0448;
pub const REG_ARFR2: c_uint = 0x044C;
pub const REG_ARFR3: c_uint = 0x0450;
pub const REG_AGGLEN_LMT: c_uint = 0x0458;
pub const REG_AMPDU_MIN_SPACE: c_uint = 0x045C;
pub const REG_TXPKTBUF_WMAC_LBK_BF_HD: c_uint = 0x045D;
pub const REG_FAST_EDCA_CTRL: c_uint = 0x0460;
pub const REG_RD_RESP_PKT_TH: c_uint = 0x0463;
pub const REG_INIRTS_RATE_SEL: c_uint = 0x0480;
pub const REG_INIDATA_RATE_SEL: c_uint = 0x0484;
pub const REG_POWER_STATUS: c_uint = 0x04A4;
pub const REG_POWER_STAGE1: c_uint = 0x04B4;
pub const REG_POWER_STAGE2: c_uint = 0x04B8;
pub const REG_PKT_LIFE_TIME: c_uint = 0x04C0;
pub const REG_PKT_VO_VI_LIFE_TIME: c_uint = 0x04C0;
pub const REG_PKT_BE_BK_LIFE_TIME: c_uint = 0x04C2;
pub const REG_STBC_SETTING: c_uint = 0x04C4;
pub const REG_PROT_MODE_CTRL: c_uint = 0x04C8;
pub const REG_MAX_AGGR_NUM: c_uint = 0x04CA;
pub const REG_RTS_MAX_AGGR_NUM: c_uint = 0x04CB;
pub const REG_BAR_MODE_CTRL: c_uint = 0x04CC;
pub const REG_RA_TRY_RATE_AGG_LMT: c_uint = 0x04CF;
pub const REG_EARLY_MODE_CONTROL: c_uint = 0x4D0;
pub const REG_NQOS_SEQ: c_uint = 0x04DC;
pub const REG_QOS_SEQ: c_uint = 0x04DE;
pub const REG_NEED_CPU_HANDLE: c_uint = 0x04E0;
pub const REG_PKT_LOSE_RPT: c_uint = 0x04E1;
pub const REG_PTCL_ERR_STATUS: c_uint = 0x04E2;
pub const REG_DUMMY: c_uint = 0x04FC;
// -----------------------------------------------------
// 0x0500h ~ 0x05FFh	EDCA Configuration
// -----------------------------------------------------
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
pub const REG_BCN_CTRL_1: c_uint = 0x0551;
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
pub const REG_SCH_TXCMD: c_uint = 0x05D0;
// Dual MAC Co-Existence Register
pub const REG_DMC: c_uint = 0x05F0;
// -----------------------------------------------------
// 0x0600h ~ 0x07FFh	WMAC Configuration
// -----------------------------------------------------
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
// WMA, BA, CCX
pub const REG_NAV_CTRL: c_uint = 0x0650;
pub const REG_BACAMCMD: c_uint = 0x0654;
pub const REG_BACAMCONTENT: c_uint = 0x0658;
pub const REG_LBDLY: c_uint = 0x0660;
pub const REG_FWDLY: c_uint = 0x0661;
pub const REG_RXERR_RPT: c_uint = 0x0664;
pub const REG_WMAC_TRXPTCL_CTL: c_uint = 0x0668;
// Security
pub const REG_CAMCMD: c_uint = 0x0670;
pub const REG_CAMWRITE: c_uint = 0x0674;
pub const REG_CAMREAD: c_uint = 0x0678;
pub const REG_CAMDBG: c_uint = 0x067C;
pub const REG_SECCFG: c_uint = 0x0680;
// Power
pub const REG_WOW_CTRL: c_uint = 0x0690;
pub const REG_PSSTATUS: c_uint = 0x0691;
pub const REG_PS_RX_INFO: c_uint = 0x0692;
pub const REG_LPNAV_CTRL: c_uint = 0x0694;
pub const REG_WKFMCAM_CMD: c_uint = 0x0698;
pub const REG_WKFMCAM_RWD: c_uint = 0x069C;
pub const REG_RXFLTMAP0: c_uint = 0x06A0;
pub const REG_RXFLTMAP1: c_uint = 0x06A2;
pub const REG_RXFLTMAP2: c_uint = 0x06A4;
pub const REG_BCN_PSR_RPT: c_uint = 0x06A8;
pub const REG_CALB32K_CTRL: c_uint = 0x06AC;
pub const REG_PKT_MON_CTRL: c_uint = 0x06B4;
pub const REG_BT_COEX_TABLE: c_uint = 0x06C0;
pub const REG_WMAC_RESP_TXINFO: c_uint = 0x06D8;
pub const REG_USB_Queue_Select_MAC0: c_uint = 0xFE44;
pub const REG_USB_Queue_Select_MAC1: c_uint = 0xFE47;
// -----------------------------------------------------
// Redifine 8192C register definition for compatibility
// -----------------------------------------------------

// -----------------------------------------------------
// 8192C (MSR) Media Status Register(Offset 0x4C, 8 bits)
// -----------------------------------------------------
pub const MSR_NOLINK: c_uint = 0x00;
pub const MSR_ADHOC: c_uint = 0x01;
pub const MSR_INFRA: c_uint = 0x02;
pub const MSR_AP: c_uint = 0x03;
pub const MSR_MASK: c_uint = 0x03;
// 6. Adaptive Control Registers  (Offset: 0x0160 - 0x01CF)
// -----------------------------------------------------
// 8192C Response Rate Set Register(offset 0x181, 24bits)
// -----------------------------------------------------
pub const RRSR_RSC_OFFSET: c_int = 21;
pub const RRSR_SHORT_OFFSET: c_int = 23;
pub const RRSR_RSC_BW_40M: c_uint = 0x600000;
pub const RRSR_RSC_UPSUBCHNL: c_uint = 0x400000;
pub const RRSR_RSC_LOWSUBCHNL: c_uint = 0x200000;
pub const RRSR_SHORT: c_uint = 0x800000;

// -----------------------------------------------------
// 8192C Rate Definition
// -----------------------------------------------------
// CCK
pub const RATR_1M: c_uint = 0x00000001;
pub const RATR_2M: c_uint = 0x00000002;
pub const RATR_55M: c_uint = 0x00000004;
pub const RATR_11M: c_uint = 0x00000008;
// OFDM
pub const RATR_6M: c_uint = 0x00000010;
pub const RATR_9M: c_uint = 0x00000020;
pub const RATR_12M: c_uint = 0x00000040;
pub const RATR_18M: c_uint = 0x00000080;
pub const RATR_24M: c_uint = 0x00000100;
pub const RATR_36M: c_uint = 0x00000200;
pub const RATR_48M: c_uint = 0x00000400;
pub const RATR_54M: c_uint = 0x00000800;
// MCS 1 Spatial Stream
pub const RATR_MCS0: c_uint = 0x00001000;
pub const RATR_MCS1: c_uint = 0x00002000;
pub const RATR_MCS2: c_uint = 0x00004000;
pub const RATR_MCS3: c_uint = 0x00008000;
pub const RATR_MCS4: c_uint = 0x00010000;
pub const RATR_MCS5: c_uint = 0x00020000;
pub const RATR_MCS6: c_uint = 0x00040000;
pub const RATR_MCS7: c_uint = 0x00080000;
// MCS 2 Spatial Stream
pub const RATR_MCS8: c_uint = 0x00100000;
pub const RATR_MCS9: c_uint = 0x00200000;
pub const RATR_MCS10: c_uint = 0x00400000;
pub const RATR_MCS11: c_uint = 0x00800000;
pub const RATR_MCS12: c_uint = 0x01000000;
pub const RATR_MCS13: c_uint = 0x02000000;
pub const RATR_MCS14: c_uint = 0x04000000;
pub const RATR_MCS15: c_uint = 0x08000000;
// CCK

// OFDM

// MCS 1 Spatial Stream

// MCS 2 Spatial Stream

// ALL CCK Rate

// -----------------------------------------------------
// 8192C BW_OPMODE bits		(Offset 0x203, 8bit)
// -----------------------------------------------------

// -----------------------------------------------------
// 8192C CAM Config Setting (offset 0x250, 1 byte)
// -----------------------------------------------------

pub const CAM_NOTVALID: c_uint = 0x0000;

pub const CAM_NONE: c_uint = 0x0;
pub const CAM_WEP40: c_uint = 0x01;
pub const CAM_TKIP: c_uint = 0x02;
pub const CAM_AES: c_uint = 0x04;
pub const CAM_WEP104: c_uint = 0x05;
pub const CAM_SMS4: c_uint = 0x6;
pub const TOTAL_CAM_ENTRY: c_int = 32;
pub const HALF_CAM_ENTRY: c_int = 16;

pub const CAM_READ: c_uint = 0x00000000;

// 10. Power Save Control Registers	 (Offset: 0x0260 - 0x02DF)

// 12. Host Interrupt Status Registers	 (Offset: 0x0300 - 0x030F)
// -----------------------------------------------------
// 8190 IMR/ISR bits	(offset 0xfd,  8bits)
// -----------------------------------------------------
pub const IMR8190_DISABLED: c_uint = 0x0;

// -----------------------------------------------------
// 8192C EFUSE
// -----------------------------------------------------
pub const HWSET_MAX_SIZE: c_int = 256;
pub const EFUSE_MAX_SECTION: c_int = 32;
pub const EFUSE_REAL_CONTENT_LEN: c_int = 512;
// -----------------------------------------------------
// 8192C EEPROM/EFUSE share register definition.
// -----------------------------------------------------
pub const EEPROM_DEFAULT_TSSI: c_uint = 0x0;
pub const EEPROM_DEFAULT_CRYSTALCAP: c_uint = 0x0;
pub const EEPROM_DEFAULT_THERMALMETER: c_uint = 0x12;
pub const EEPROM_DEFAULT_TXPOWERLEVEL_2G: c_uint = 0x2C;
pub const EEPROM_DEFAULT_TXPOWERLEVEL_5G: c_uint = 0x22;
pub const EEPROM_DEFAULT_HT40_2SDIFF: c_uint = 0x0;
// HT20<->40 default Tx Power Index Difference
pub const EEPROM_DEFAULT_HT20_DIFF: c_int = 2;
// OFDM Tx Power index diff
pub const EEPROM_DEFAULT_LEGACYHTTXPOWERDIFF: c_uint = 0x4;
pub const EEPROM_DEFAULT_HT40_PWRMAXOFFSET: c_int = 0;
pub const EEPROM_DEFAULT_HT20_PWRMAXOFFSET: c_int = 0;
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
pub const EEPROM_CHANNEL_PLAN_NCC: c_uint = 0xB;
pub const EEPROM_CHANNEL_PLAN_BY_HW_MASK: c_uint = 0x80;
pub const EEPROM_CID_DEFAULT: c_uint = 0x0;
pub const EEPROM_CID_TOSHIBA: c_uint = 0x4;
pub const EEPROM_CID_CCX: c_uint = 0x10;
pub const EEPROM_CID_QMI: c_uint = 0x0D;
pub const EEPROM_CID_WHQL: c_uint = 0xFE;
pub const RTL8192_EEPROM_ID: c_uint = 0x8129;
pub const EEPROM_WAPI_SUPPORT: c_uint = 0x78;
pub const RTL8190_EEPROM_ID: c_uint = 0x8129	/* 0-1 */;
pub const EEPROM_HPON: c_uint = 0x02 /* LDO settings.2-5 */;
pub const EEPROM_CLK: c_uint = 0x06 /* Clock settings.6-7 */;
pub const EEPROM_MAC_FUNCTION: c_uint = 0x08 /* SE Test mode.8 */;
pub const EEPROM_VID: c_uint = 0x28 /* SE Vendor ID.A-B */;
pub const EEPROM_DID: c_uint = 0x2A /* SE Device ID. C-D */;
pub const EEPROM_SVID: c_uint = 0x2C /* SE Vendor ID.E-F */;
pub const EEPROM_SMID: c_uint = 0x2E /* SE PCI Subsystem ID. 10-11 */;
pub const EEPROM_VID_USB: c_uint = 0xC;
pub const EEPROM_PID_USB: c_uint = 0xE;
pub const EEPROM_ENDPOINT_SETTING: c_uint = 0x10;
pub const EEPROM_MAC_ADDR: c_uint = 0x16 /* SEMAC Address. 12-17 */;
pub const EEPROM_MAC_ADDR_MAC0_92DU: c_uint = 0x19;
pub const EEPROM_MAC_ADDR_MAC0_92D: c_uint = 0x55;
pub const EEPROM_MAC_ADDR_MAC1_92D: c_uint = 0x5B;
// 2.4G band Tx power index setting
pub const EEPROM_CCK_TX_PWR_INX_2G: c_uint = 0x61;
pub const EEPROM_HT40_1S_TX_PWR_INX_2G: c_uint = 0x67;
pub const EEPROM_HT40_2S_TX_PWR_INX_DIFF_2G: c_uint = 0x6D;
pub const EEPROM_HT20_TX_PWR_INX_DIFF_2G: c_uint = 0x70;
pub const EEPROM_OFDM_TX_PWR_INX_DIFF_2G: c_uint = 0x73;
pub const EEPROM_HT40_MAX_PWR_OFFSET_2G: c_uint = 0x76;
pub const EEPROM_HT20_MAX_PWR_OFFSET_2G: c_uint = 0x79;
// 5GL channel 32-64
pub const EEPROM_HT40_1S_TX_PWR_INX_5GL: c_uint = 0x7C;
pub const EEPROM_HT40_2S_TX_PWR_INX_DIFF_5GL: c_uint = 0x82;
pub const EEPROM_HT20_TX_PWR_INX_DIFF_5GL: c_uint = 0x85;
pub const EEPROM_OFDM_TX_PWR_INX_DIFF_5GL: c_uint = 0x88;
pub const EEPROM_HT40_MAX_PWR_OFFSET_5GL: c_uint = 0x8B;
pub const EEPROM_HT20_MAX_PWR_OFFSET_5GL: c_uint = 0x8E;
// 5GM channel 100-140
pub const EEPROM_HT40_1S_TX_PWR_INX_5GM: c_uint = 0x91;
pub const EEPROM_HT40_2S_TX_PWR_INX_DIFF_5GM: c_uint = 0x97;
pub const EEPROM_HT20_TX_PWR_INX_DIFF_5GM: c_uint = 0x9A;
pub const EEPROM_OFDM_TX_PWR_INX_DIFF_5GM: c_uint = 0x9D;
pub const EEPROM_HT40_MAX_PWR_OFFSET_5GM: c_uint = 0xA0;
pub const EEPROM_HT20_MAX_PWR_OFFSET_5GM: c_uint = 0xA3;
// 5GH channel 149-165
pub const EEPROM_HT40_1S_TX_PWR_INX_5GH: c_uint = 0xA6;
pub const EEPROM_HT40_2S_TX_PWR_INX_DIFF_5GH: c_uint = 0xAC;
pub const EEPROM_HT20_TX_PWR_INX_DIFF_5GH: c_uint = 0xAF;
pub const EEPROM_OFDM_TX_PWR_INX_DIFF_5GH: c_uint = 0xB2;
pub const EEPROM_HT40_MAX_PWR_OFFSET_5GH: c_uint = 0xB5;
pub const EEPROM_HT20_MAX_PWR_OFFSET_5GH: c_uint = 0xB8;
// Map of supported channels.
pub const EEPROM_CHANNEL_PLAN: c_uint = 0xBB;
pub const EEPROM_IQK_DELTA: c_uint = 0xBC;
pub const EEPROM_LCK_DELTA: c_uint = 0xBC;
pub const EEPROM_XTAL_K: c_uint = 0xBD	/* [7:5] */;
pub const EEPROM_TSSI_A_5G: c_uint = 0xBE;
pub const EEPROM_TSSI_B_5G: c_uint = 0xBF;
pub const EEPROM_TSSI_AB_5G: c_uint = 0xC0;
pub const EEPROM_THERMAL_METER: c_uint = 0xC3	/* [4:0] */;
pub const EEPROM_RF_OPT1: c_uint = 0xC4;
pub const EEPROM_RF_OPT2: c_uint = 0xC5;
pub const EEPROM_RF_OPT3: c_uint = 0xC6;
pub const EEPROM_RF_OPT4: c_uint = 0xC7;
pub const EEPROM_RF_OPT5: c_uint = 0xC8;
pub const EEPROM_RF_OPT6: c_uint = 0xC9;
pub const EEPROM_VERSION: c_uint = 0xCA;
pub const EEPROM_CUSTOMER_ID: c_uint = 0xCB;
pub const EEPROM_RF_OPT7: c_uint = 0xCC;
pub const EEPROM_DEF_PART_NO: c_uint = 0x3FD    /* Byte */;
pub const EEPROME_CHIP_VERSION_L: c_uint = 0x3FF;
pub const EEPROME_CHIP_VERSION_H: c_uint = 0x3FE;
//
// Current IOREG MAP
// 0x0000h ~ 0x00FFh   System Configuration (256 Bytes)
// 0x0100h ~ 0x01FFh   MACTOP General Configuration (256 Bytes)
// 0x0200h ~ 0x027Fh   TXDMA Configuration (128 Bytes)
// 0x0280h ~ 0x02FFh   RXDMA Configuration (128 Bytes)
// 0x0300h ~ 0x03FFh   PCIE EMAC Reserved Region (256 Bytes)
// 0x0400h ~ 0x04FFh   Protocol Configuration (256 Bytes)
// 0x0500h ~ 0x05FFh   EDCA Configuration (256 Bytes)
// 0x0600h ~ 0x07FFh   WMAC Configuration (512 Bytes)
// 0x2000h ~ 0x3FFFh   8051 FW Download Region (8196 Bytes)
//
// -----------------------------------------------------
// 8192C (RCR)	(Offset 0x608, 32 bits)
// -----------------------------------------------------

pub const RCR_MXDMA_OFFSET: c_int = 8;
pub const RCR_FIFO_OFFSET: c_int = 13;
// -----------------------------------------------------
// 8192C Regsiter Bit and Content definition
// -----------------------------------------------------
// 0x0000h ~ 0x00FFh	System Configuration
// -----------------------------------------------------
// SPS0_CTRL

// SYS_ISO_CTRL

// SYS_FUNC_EN

// APS_FSMCO

// SYS_CLKR

// 9346CR

// AFE_MISC

// RSV_CTRL

// RF_CTRL

// LDOA15_CTRL

// LDOV12D_CTRL

// AFE_XTAL_CTRL

// AFE_PLL_CTRL

pub const APLL_REF_CLK_13MHZ: c_uint = 0x1;
pub const APLL_REF_CLK_19_2MHZ: c_uint = 0x2;
pub const APLL_REF_CLK_20MHZ: c_uint = 0x3;
pub const APLL_REF_CLK_25MHZ: c_uint = 0x4;
pub const APLL_REF_CLK_26MHZ: c_uint = 0x5;
pub const APLL_REF_CLK_38_4MHZ: c_uint = 0x6;
pub const APLL_REF_CLK_40MHZ: c_uint = 0x7;

// EFUSE_CTRL

// EFUSE_TEST

// MCUFWDL

// REG_SYS_CFG

pub const QUEUE_LOW: c_int = 1;
pub const QUEUE_NORMAL: c_int = 2;
pub const QUEUE_HIGH: c_int = 3;

// LLT_INIT
pub const _LLT_NO_ACTIVE: c_uint = 0x0;
pub const _LLT_WRITE_ACCESS: c_uint = 0x1;
pub const _LLT_READ_ACCESS: c_uint = 0x2;

// -----------------------------------------------------
// 0x0400h ~ 0x047Fh	Protocol Configuration
// -----------------------------------------------------
// FWHW_TXQ_CTRL

pub const RETRY_LIMIT_SHORT_SHIFT: c_int = 8;
pub const RETRY_LIMIT_LONG_SHIFT: c_int = 0;
// -----------------------------------------------------
// 0x0500h ~ 0x05FFh	EDCA Configuration
// -----------------------------------------------------
// EDCA setting
pub const AC_PARAM_TXOP_LIMIT_OFFSET: c_int = 16;
pub const AC_PARAM_ECW_MAX_OFFSET: c_int = 12;
pub const AC_PARAM_ECW_MIN_OFFSET: c_int = 8;
pub const AC_PARAM_AIFS_OFFSET: c_int = 0;
// REG_RD_CTRL

// REG_BCN_CTRL

// ACMHWCTRL

// -----------------------------------------------------
// 0x0600h ~ 0x07FFh	WMAC Configuration
// -----------------------------------------------------
// TCR

// SECCFG

// General definitions
pub const LAST_ENTRY_OF_TX_PKT_BUFFER: c_int = 255;
pub const LAST_ENTRY_OF_TX_PKT_BUFFER_DUAL_MAC: c_int = 127;
pub const POLLING_LLT_THRESHOLD: c_int = 20;
pub const POLLING_READY_TIMEOUT_COUNT: c_int = 1000;
// Min Spacing related settings.
pub const MAX_MSS_DENSITY_2T: c_uint = 0x13;
pub const MAX_MSS_DENSITY_1T: c_uint = 0x0A;
// BB-PHY register PMAC 0x100 PHY 0x800 - 0xEFF
// 1. PMAC duplicate register due to connection:
// RF_Mode, TRxRN, NumOf L-STF
// 2. 0x800/0x900/0xA00/0xC00/0xD00/0xE00
// 3. RF register 0x00-2E
// 4. Bit Mask for BB/RF register
// 5. Other defintion for BB/RF R/W
// 3. Page8(0x800)
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
pub const RFPGA0_XA_LSSIPARAMETER: c_uint = 0x840;
pub const RFPGA0_XB_LSSIPARAMETER: c_uint = 0x844;
pub const RFPGA0_RFWAKEUPPARAMETER: c_uint = 0x850;
pub const RFPGA0_RFSLEEPUPPARAMETER: c_uint = 0x854;
pub const RFPGA0_XAB_SWITCHCONTROL: c_uint = 0x858;
pub const RFPGA0_XCD_SWITCHCONTROL: c_uint = 0x85c;
pub const RFPGA0_XA_RFINTERFACEOE: c_uint = 0x860;
pub const RFPGA0_XB_RFINTERFACEOE: c_uint = 0x864;
pub const RFPGA0_XAB_RFINTERFACESW: c_uint = 0x870;
pub const RFPGA0_XCD_RFINTERFACESW: c_uint = 0x874;
pub const RFPGA0_XAB_RFPARAMETER: c_uint = 0x878;
pub const RFPGA0_XCD_RFPARAMETER: c_uint = 0x87c;
pub const RFPGA0_ANALOGPARAMETER1: c_uint = 0x880;
pub const RFPGA0_ANALOGPARAMETER2: c_uint = 0x884;
pub const RFPGA0_ANALOGPARAMETER3: c_uint = 0x888;
pub const RFPGA0_ADDALLOCKEN: c_uint = 0x888;
pub const RFPGA0_ANALOGPARAMETER4: c_uint = 0x88c;
pub const RFPGA0_XA_LSSIREADBACK: c_uint = 0x8a0;
pub const RFPGA0_XB_LSSIREADBACK: c_uint = 0x8a4;
pub const RFPGA0_XC_LSSIREADBACK: c_uint = 0x8a8;
pub const RFPGA0_XD_LSSIREADBACK: c_uint = 0x8ac;
pub const RFPGA0_PSDREPORT: c_uint = 0x8b4;
pub const TRANSCEIVERA_HSPI_READBACK: c_uint = 0x8b8;
pub const TRANSCEIVERB_HSPI_READBACK: c_uint = 0x8bc;
pub const RFPGA0_XAB_RFINTERFACERB: c_uint = 0x8e0;
pub const RFPGA0_XCD_RFINTERFACERB: c_uint = 0x8e4;
// 4. Page9(0x900)
pub const RFPGA1_RFMOD: c_uint = 0x900;
pub const RFPGA1_TXBLOCK: c_uint = 0x904;
pub const RFPGA1_DEBUGSELECT: c_uint = 0x908;
pub const RFPGA1_TXINFO: c_uint = 0x90c;
// 5. PageA(0xA00)
pub const RCCK0_SYSTEM: c_uint = 0xa00;
pub const RCCK0_AFESSTTING: c_uint = 0xa04;
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
pub const RPDP_ANTA: c_uint = 0xb00;
pub const RCONFIG_ANTA: c_uint = 0xb68;
pub const RCONFIG_ANTB: c_uint = 0xb6c;
pub const RPDP_ANTB: c_uint = 0xb70;
// 6. PageC(0xC00)
pub const ROFDM0_LSTF: c_uint = 0xc00;
pub const ROFDM0_TRXPATHENABLE: c_uint = 0xc04;
pub const ROFDM0_TRMUXPAR: c_uint = 0xc08;
pub const ROFDM0_TRSWISOLATION: c_uint = 0xc0c;
pub const ROFDM0_XARXAFE: c_uint = 0xc10;
pub const ROFDM0_XARXIQIMBALANCE: c_uint = 0xc14;
pub const ROFDM0_XBRXAFE: c_uint = 0xc18;
pub const ROFDM0_XBRXIQIMBALANCE: c_uint = 0xc1c;
pub const ROFDM0_XCRXAFE: c_uint = 0xc20;
pub const ROFDM0_XCRXIQIMBALANCE: c_uint = 0xc24;
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
pub const ROFDM0_RXHPPARAMETER: c_uint = 0xce0;
pub const ROFDM0_TXPSEUDONOISEWGT: c_uint = 0xce4;
pub const ROFDM0_FRAMESYNC: c_uint = 0xcf0;
pub const ROFDM0_DFSREPORT: c_uint = 0xcf4;
pub const ROFDM0_RXIQEXTANTA: c_uint = 0xca0;
pub const ROFDM0_TXCOEFF1: c_uint = 0xca4;
pub const ROFDM0_TXCOEFF2: c_uint = 0xca8;
pub const ROFDM0_TXCOEFF3: c_uint = 0xcac;
pub const ROFDM0_TXCOEFF4: c_uint = 0xcb0;
pub const ROFDM0_TXCOEFF5: c_uint = 0xcb4;
pub const ROFDM0_TXCOEFF6: c_uint = 0xcb8;
// 7. PageD(0xD00)
pub const ROFDM1_LSTF: c_uint = 0xd00;
pub const ROFDM1_TRXPATHENABLE: c_uint = 0xd04;
pub const ROFDM1_CFO: c_uint = 0xd08;
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
pub const ROFDM_TAILCFOAB: c_uint = 0xdbc;
pub const ROFDM_TAILCFOCD: c_uint = 0xdc0;
pub const ROFDM_PWMEASURE1: c_uint = 0xdc4;
pub const ROFDM_PWMEASURE2: c_uint = 0xdc8;
pub const ROFDM_BWREPORT: c_uint = 0xdcc;
pub const ROFDM_AGCREPORT: c_uint = 0xdd0;
pub const ROFDM_RXSNR: c_uint = 0xdd4;
pub const ROFDM_RXEVMCSI: c_uint = 0xdd8;
pub const ROFDM_SIGREPORT: c_uint = 0xddc;
// 8. PageE(0xE00)
pub const RTXAGC_A_RATE18_06: c_uint = 0xe00;
pub const RTXAGC_A_RATE54_24: c_uint = 0xe04;
pub const RTXAGC_A_CCK1_MCS32: c_uint = 0xe08;
pub const RTXAGC_A_MCS03_MCS00: c_uint = 0xe10;
pub const RTXAGC_A_MCS07_MCS04: c_uint = 0xe14;
pub const RTXAGC_A_MCS11_MCS08: c_uint = 0xe18;
pub const RTXAGC_A_MCS15_MCS12: c_uint = 0xe1c;
pub const RTXAGC_B_RATE18_06: c_uint = 0x830;
pub const RTXAGC_B_RATE54_24: c_uint = 0x834;
pub const RTXAGC_B_CCK1_55_MCS32: c_uint = 0x838;
pub const RTXAGC_B_MCS03_MCS00: c_uint = 0x83c;
pub const RTXAGC_B_MCS07_MCS04: c_uint = 0x848;
pub const RTXAGC_B_MCS11_MCS08: c_uint = 0x84c;
pub const RTXAGC_B_MCS15_MCS12: c_uint = 0x868;
pub const RTXAGC_B_CCK11_A_CCK2_11: c_uint = 0x86c;
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
pub const RRX_POWER_BEFORE_IQK_B: c_uint = 0xec0;
pub const RRX_POWER_BEFORE_IQK_B_2: c_uint = 0xec4;
pub const RRX_POWER_AFTER_IQK_B: c_uint = 0xec8;
pub const RRX_POWER_AFTER_IQK_B_2: c_uint = 0xecc;
pub const MASK_IQK_RESULT: c_uint = 0x03ff0000;
pub const RRX_OFDM: c_uint = 0xed0;
pub const RRX_WAIT_RIFS: c_uint = 0xed4;
pub const RRX_TO_RX: c_uint = 0xed8;
pub const RSTANDBY: c_uint = 0xedc;
pub const RSLEEP: c_uint = 0xee0;
pub const RPMPD_ANAEN: c_uint = 0xeec;
// RL6052 Register definition
pub const RF_AC: c_uint = 0x00;
pub const RF_IQADJ_G1: c_uint = 0x01;
pub const RF_IQADJ_G2: c_uint = 0x02;
pub const RF_BS_PA_APSET_G1_G4: c_uint = 0x03;
pub const RF_POW_TRSW: c_uint = 0x05;
pub const RF_GAIN_RX: c_uint = 0x06;
pub const RF_GAIN_TX: c_uint = 0x07;
pub const RF_TXM_IDAC: c_uint = 0x08;
pub const RF_TXPA_AG: c_uint = 0x0B;
pub const RF_BS_IQGEN: c_uint = 0x0F;
pub const RF_MODE1: c_uint = 0x10;
pub const RF_MODE2: c_uint = 0x11;
pub const RF_RX_AGC_HP: c_uint = 0x12;
pub const RF_TX_AGC: c_uint = 0x13;
pub const RF_BIAS: c_uint = 0x14;
pub const RF_IPA: c_uint = 0x15;
pub const RF_POW_ABILITY: c_uint = 0x17;
pub const RF_MODE_AG: c_uint = 0x18;
pub const rfchannel: c_uint = 0x18;
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
pub const RF_T_METER: c_uint = 0x42;
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
// Bit Mask
// 2. Page8(0x800)
pub const BRFMOD: c_uint = 0x1;
pub const BCCKTXSC: c_uint = 0x30;
pub const BCCKEN: c_uint = 0x1000000;
pub const BOFDMEN: c_uint = 0x2000000;
pub const B3WIREDATALENGTH: c_uint = 0x800;
pub const B3WIREADDRESSLENGTH: c_uint = 0x400;
pub const BRFSI_RFENV: c_uint = 0x10;
pub const BLSSIREADADDRESS: c_uint = 0x7f800000;
pub const BLSSIREADEDGE: c_uint = 0x80000000;
pub const BLSSIREADBACKDATA: c_uint = 0xfffff;
// 4. PageA(0xA00)
pub const BCCKSIDEBAND: c_uint = 0x10;
// Other Definition
pub const BBYTE0: c_uint = 0x1;
pub const BBYTE1: c_uint = 0x2;
pub const BBYTE2: c_uint = 0x4;
pub const BBYTE3: c_uint = 0x8;
pub const BWORD0: c_uint = 0x3;
pub const BWORD1: c_uint = 0xc;
pub const BDWORD: c_uint = 0xf;
