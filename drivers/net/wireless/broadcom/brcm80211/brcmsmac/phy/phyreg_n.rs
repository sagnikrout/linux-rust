//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/phy/phyreg_n.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//
pub const NPHY_TBL_ID_GAIN1: c_int = 0;
pub const NPHY_TBL_ID_GAIN2: c_int = 1;
pub const NPHY_TBL_ID_GAINBITS1: c_int = 2;
pub const NPHY_TBL_ID_GAINBITS2: c_int = 3;
pub const NPHY_TBL_ID_GAINLIMIT: c_int = 4;
pub const NPHY_TBL_ID_WRSSIGainLimit: c_int = 5;
pub const NPHY_TBL_ID_RFSEQ: c_int = 7;
pub const NPHY_TBL_ID_AFECTRL: c_int = 8;
pub const NPHY_TBL_ID_ANTSWCTRLLUT: c_int = 9;
pub const NPHY_TBL_ID_IQLOCAL: c_int = 15;
pub const NPHY_TBL_ID_NOISEVAR: c_int = 16;
pub const NPHY_TBL_ID_SAMPLEPLAY: c_int = 17;
pub const NPHY_TBL_ID_CORE1TXPWRCTL: c_int = 26;
pub const NPHY_TBL_ID_CORE2TXPWRCTL: c_int = 27;
pub const NPHY_TBL_ID_CMPMETRICDATAWEIGHTTBL: c_int = 30;
pub const NPHY_TBL_ID_EPSILONTBL0: c_int = 31;
pub const NPHY_TBL_ID_SCALARTBL0: c_int = 32;
pub const NPHY_TBL_ID_EPSILONTBL1: c_int = 33;
pub const NPHY_TBL_ID_SCALARTBL1: c_int = 34;
pub const NPHY_TO_BPHY_OFF: c_uint = 0xc00;
pub const NPHY_BandControl_currentBand: c_uint = 0x0001;
pub const RFCC_CHIP0_PU: c_uint = 0x0400;
pub const RFCC_POR_FORCE: c_uint = 0x0040;
pub const RFCC_OE_POR_FORCE: c_uint = 0x0080;
pub const NPHY_RfctrlIntc_override_OFF: c_int = 0;
pub const NPHY_RfctrlIntc_override_TRSW: c_int = 1;
pub const NPHY_RfctrlIntc_override_PA: c_int = 2;
pub const NPHY_RfctrlIntc_override_EXT_LNA_PU: c_int = 3;
pub const NPHY_RfctrlIntc_override_EXT_LNA_GAIN: c_int = 4;
pub const RIFS_ENABLE: c_uint = 0x80;
pub const BPHY_BAND_SEL_UP20: c_uint = 0x10;
pub const NPHY_MLenable: c_uint = 0x02;
pub const NPHY_RfseqMode_CoreActv_override: c_uint = 0x0001;
pub const NPHY_RfseqMode_Trigger_override: c_uint = 0x0002;

pub const NPHY_RfseqTrigger_rx2tx: c_uint = 0x0001;
pub const NPHY_RfseqTrigger_tx2rx: c_uint = 0x0002;
pub const NPHY_RfseqTrigger_updategainh: c_uint = 0x0004;
pub const NPHY_RfseqTrigger_updategainl: c_uint = 0x0008;
pub const NPHY_RfseqTrigger_updategainu: c_uint = 0x0010;
pub const NPHY_RfseqTrigger_reset2rx: c_uint = 0x0020;
pub const NPHY_RfseqStatus_rx2tx: c_uint = 0x0001;
pub const NPHY_RfseqStatus_tx2rx: c_uint = 0x0002;
pub const NPHY_RfseqStatus_updategainh: c_uint = 0x0004;
pub const NPHY_RfseqStatus_updategainl: c_uint = 0x0008;
pub const NPHY_RfseqStatus_updategainu: c_uint = 0x0010;
pub const NPHY_RfseqStatus_reset2rx: c_uint = 0x0020;
pub const NPHY_ClassifierCtrl_cck_en: c_uint = 0x1;
pub const NPHY_ClassifierCtrl_ofdm_en: c_uint = 0x2;
pub const NPHY_ClassifierCtrl_waited_en: c_uint = 0x4;
pub const NPHY_IQFlip_ADC1: c_uint = 0x0001;
pub const NPHY_IQFlip_ADC2: c_uint = 0x0010;
pub const NPHY_sampleCmd_STOP: c_uint = 0x0002;
pub const RX_GF_OR_MM: c_uint = 0x0004;
pub const RX_GF_MM_AUTO: c_uint = 0x0100;
pub const NPHY_iqloCalCmdGctl_IQLO_CAL_EN: c_uint = 0x8000;
pub const NPHY_IqestCmd_iqstart: c_uint = 0x1;
pub const NPHY_IqestCmd_iqMode: c_uint = 0x2;
pub const NPHY_TxPwrCtrlCmd_pwrIndex_init: c_uint = 0x40;
pub const NPHY_TxPwrCtrlCmd_pwrIndex_init_rev7: c_uint = 0x19;
pub const PRIM_SEL_UP20: c_uint = 0x8000;
pub const NPHY_RFSEQ_RX2TX: c_uint = 0x0;
pub const NPHY_RFSEQ_TX2RX: c_uint = 0x1;
pub const NPHY_RFSEQ_RESET2RX: c_uint = 0x2;
pub const NPHY_RFSEQ_UPDATEGAINH: c_uint = 0x3;
pub const NPHY_RFSEQ_UPDATEGAINL: c_uint = 0x4;
pub const NPHY_RFSEQ_UPDATEGAINU: c_uint = 0x5;
pub const NPHY_RFSEQ_CMD_NOP: c_uint = 0x0;
pub const NPHY_RFSEQ_CMD_RXG_FBW: c_uint = 0x1;
pub const NPHY_RFSEQ_CMD_TR_SWITCH: c_uint = 0x2;
pub const NPHY_RFSEQ_CMD_EXT_PA: c_uint = 0x3;
pub const NPHY_RFSEQ_CMD_RXPD_TXPD: c_uint = 0x4;
pub const NPHY_RFSEQ_CMD_TX_GAIN: c_uint = 0x5;
pub const NPHY_RFSEQ_CMD_RX_GAIN: c_uint = 0x6;
pub const NPHY_RFSEQ_CMD_SET_HPF_BW: c_uint = 0x7;
pub const NPHY_RFSEQ_CMD_CLR_HIQ_DIS: c_uint = 0x8;
pub const NPHY_RFSEQ_CMD_END: c_uint = 0xf;
pub const NPHY_REV3_RFSEQ_CMD_NOP: c_uint = 0x0;
pub const NPHY_REV3_RFSEQ_CMD_RXG_FBW: c_uint = 0x1;
pub const NPHY_REV3_RFSEQ_CMD_TR_SWITCH: c_uint = 0x2;
pub const NPHY_REV3_RFSEQ_CMD_INT_PA_PU: c_uint = 0x3;
pub const NPHY_REV3_RFSEQ_CMD_EXT_PA: c_uint = 0x4;
pub const NPHY_REV3_RFSEQ_CMD_RXPD_TXPD: c_uint = 0x5;
pub const NPHY_REV3_RFSEQ_CMD_TX_GAIN: c_uint = 0x6;
pub const NPHY_REV3_RFSEQ_CMD_RX_GAIN: c_uint = 0x7;
pub const NPHY_REV3_RFSEQ_CMD_CLR_HIQ_DIS: c_uint = 0x8;
pub const NPHY_REV3_RFSEQ_CMD_SET_HPF_H_HPC: c_uint = 0x9;
pub const NPHY_REV3_RFSEQ_CMD_SET_LPF_H_HPC: c_uint = 0xa;
pub const NPHY_REV3_RFSEQ_CMD_SET_HPF_M_HPC: c_uint = 0xb;
pub const NPHY_REV3_RFSEQ_CMD_SET_LPF_M_HPC: c_uint = 0xc;
pub const NPHY_REV3_RFSEQ_CMD_SET_HPF_L_HPC: c_uint = 0xd;
pub const NPHY_REV3_RFSEQ_CMD_SET_LPF_L_HPC: c_uint = 0xe;
pub const NPHY_REV3_RFSEQ_CMD_CLR_RXRX_BIAS: c_uint = 0xf;
pub const NPHY_REV3_RFSEQ_CMD_END: c_uint = 0x1f;
pub const NPHY_RSSI_SEL_W1: c_uint = 0x0;
pub const NPHY_RSSI_SEL_W2: c_uint = 0x1;
pub const NPHY_RSSI_SEL_NB: c_uint = 0x2;
pub const NPHY_RSSI_SEL_IQ: c_uint = 0x3;
pub const NPHY_RSSI_SEL_TSSI_2G: c_uint = 0x4;
pub const NPHY_RSSI_SEL_TSSI_5G: c_uint = 0x5;
pub const NPHY_RSSI_SEL_TBD: c_uint = 0x6;
pub const NPHY_RAIL_I: c_uint = 0x0;
pub const NPHY_RAIL_Q: c_uint = 0x1;
pub const NPHY_FORCESIG_DECODEGATEDCLKS: c_uint = 0x8;
pub const NPHY_REV7_RfctrlOverride_cmd_rxrf_pu: c_uint = 0x0;
pub const NPHY_REV7_RfctrlOverride_cmd_rx_pu: c_uint = 0x1;
pub const NPHY_REV7_RfctrlOverride_cmd_tx_pu: c_uint = 0x2;
pub const NPHY_REV7_RfctrlOverride_cmd_rxgain: c_uint = 0x3;
pub const NPHY_REV7_RfctrlOverride_cmd_txgain: c_uint = 0x4;
pub const NPHY_REV7_RXGAINCODE_RFMXGAIN_MASK: c_uint = 0x000ff;
pub const NPHY_REV7_RXGAINCODE_LPFGAIN_MASK: c_uint = 0x0ff00;
pub const NPHY_REV7_RXGAINCODE_DVGAGAIN_MASK: c_uint = 0xf0000;
pub const NPHY_REV7_TXGAINCODE_TGAIN_MASK: c_uint = 0x7fff;
pub const NPHY_REV7_TXGAINCODE_LPFGAIN_MASK: c_uint = 0x8000;
pub const NPHY_REV7_TXGAINCODE_BIQ0GAIN_SHIFT: c_int = 14;
pub const NPHY_REV7_RFCTRLOVERRIDE_ID0: c_uint = 0x0;
pub const NPHY_REV7_RFCTRLOVERRIDE_ID1: c_uint = 0x1;
pub const NPHY_REV7_RFCTRLOVERRIDE_ID2: c_uint = 0x2;

