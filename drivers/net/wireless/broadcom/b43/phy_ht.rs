//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_ht.h
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

pub const B43_PHY_HT_BBCFG: c_uint = 0x001 /* BB config */;
pub const B43_PHY_HT_BBCFG_RSTCCA: c_uint = 0x4000 /* Reset CCA */;
pub const B43_PHY_HT_BBCFG_RSTRX: c_uint = 0x8000 /* Reset RX */;
pub const B43_PHY_HT_BANDCTL: c_uint = 0x009 /* Band control */;
pub const B43_PHY_HT_BANDCTL_5GHZ: c_uint = 0x0001 /* Use the 5GHz band */;
pub const B43_PHY_HT_TABLE_ADDR: c_uint = 0x072 /* Table address */;
pub const B43_PHY_HT_TABLE_DATALO: c_uint = 0x073 /* Table data low */;
pub const B43_PHY_HT_TABLE_DATAHI: c_uint = 0x074 /* Table data high */;
pub const B43_PHY_HT_CLASS_CTL: c_uint = 0x0B0 /* Classifier control */;
pub const B43_PHY_HT_CLASS_CTL_CCK_EN: c_uint = 0x0001 /* CCK enable */;
pub const B43_PHY_HT_CLASS_CTL_OFDM_EN: c_uint = 0x0002 /* OFDM enable */;
pub const B43_PHY_HT_CLASS_CTL_WAITED_EN: c_uint = 0x0004 /* Waited enable */;
pub const B43_PHY_HT_IQLOCAL_CMDGCTL: c_uint = 0x0C2	/* I/Q LO cal command G control */;
pub const B43_PHY_HT_SAMP_CMD: c_uint = 0x0C3	/* Sample command */;
pub const B43_PHY_HT_SAMP_CMD_STOP: c_uint = 0x0002	/* Stop */;
pub const B43_PHY_HT_SAMP_LOOP_CNT: c_uint = 0x0C4	/* Sample loop count */;
pub const B43_PHY_HT_SAMP_WAIT_CNT: c_uint = 0x0C5	/* Sample wait count */;
pub const B43_PHY_HT_SAMP_DEP_CNT: c_uint = 0x0C6	/* Sample depth count */;
pub const B43_PHY_HT_SAMP_STAT: c_uint = 0x0C7	/* Sample status */;
pub const B43_PHY_HT_EST_PWR_C1: c_uint = 0x118;
pub const B43_PHY_HT_EST_PWR_C2: c_uint = 0x119;
pub const B43_PHY_HT_EST_PWR_C3: c_uint = 0x11A;
pub const B43_PHY_HT_TSSIMODE: c_uint = 0x122	/* TSSI mode */;
pub const B43_PHY_HT_TSSIMODE_EN: c_uint = 0x0001	/* TSSI enable */;
pub const B43_PHY_HT_TSSIMODE_PDEN: c_uint = 0x0002	/* Power det enable */;
pub const B43_PHY_HT_BW1: c_uint = 0x1CE;
pub const B43_PHY_HT_BW2: c_uint = 0x1CF;
pub const B43_PHY_HT_BW3: c_uint = 0x1D0;
pub const B43_PHY_HT_BW4: c_uint = 0x1D1;
pub const B43_PHY_HT_BW5: c_uint = 0x1D2;
pub const B43_PHY_HT_BW6: c_uint = 0x1D3;
pub const B43_PHY_HT_TXPCTL_CMD_C1: c_uint = 0x1E7	/* TX power control command */;
pub const B43_PHY_HT_TXPCTL_CMD_C1_INIT: c_uint = 0x007F	/* Init */;
pub const B43_PHY_HT_TXPCTL_CMD_C1_COEFF: c_uint = 0x2000	/* Power control coefficients */;
pub const B43_PHY_HT_TXPCTL_CMD_C1_HWPCTLEN: c_uint = 0x4000	/* Hardware TX power control enable */;
pub const B43_PHY_HT_TXPCTL_CMD_C1_PCTLEN: c_uint = 0x8000	/* TX power control enable */;
pub const B43_PHY_HT_TXPCTL_N: c_uint = 0x1E8	/* TX power control N num */;
pub const B43_PHY_HT_TXPCTL_N_TSSID: c_uint = 0x00FF	/* N TSSI delay */;
pub const B43_PHY_HT_TXPCTL_N_TSSID_SHIFT: c_int = 0;
pub const B43_PHY_HT_TXPCTL_N_NPTIL2: c_uint = 0x0700	/* N PT integer log2 */;
pub const B43_PHY_HT_TXPCTL_N_NPTIL2_SHIFT: c_int = 8;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI: c_uint = 0x1E9	/* TX power control idle TSSI */;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI_C1: c_uint = 0x003F;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI_C1_SHIFT: c_int = 0;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI_C2: c_uint = 0x3F00;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI_C2_SHIFT: c_int = 8;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI_BINF: c_uint = 0x8000	/* Raw TSSI offset bin format */;
pub const B43_PHY_HT_TXPCTL_TARG_PWR: c_uint = 0x1EA	/* TX power control target power */;
pub const B43_PHY_HT_TXPCTL_TARG_PWR_C1: c_uint = 0x00FF	/* Power 0 */;
pub const B43_PHY_HT_TXPCTL_TARG_PWR_C1_SHIFT: c_int = 0;
pub const B43_PHY_HT_TXPCTL_TARG_PWR_C2: c_uint = 0xFF00	/* Power 1 */;
pub const B43_PHY_HT_TXPCTL_TARG_PWR_C2_SHIFT: c_int = 8;
pub const B43_PHY_HT_TX_PCTL_STATUS_C1: c_uint = 0x1ED;
pub const B43_PHY_HT_TX_PCTL_STATUS_C2: c_uint = 0x1EE;
pub const B43_PHY_HT_TXPCTL_CMD_C2: c_uint = 0x222;
pub const B43_PHY_HT_TXPCTL_CMD_C2_INIT: c_uint = 0x007F;
pub const B43_PHY_HT_RSSI_C1: c_uint = 0x219;
pub const B43_PHY_HT_RSSI_C2: c_uint = 0x21A;
pub const B43_PHY_HT_RSSI_C3: c_uint = 0x21B;

pub const B43_PHY_HT_RF_SEQ_MODE_CA_OVER: c_uint = 0x0001	/* Core active override */;
pub const B43_PHY_HT_RF_SEQ_MODE_TR_OVER: c_uint = 0x0002	/* Trigger override */;

pub const B43_PHY_HT_RF_SEQ_TRIG_RX2TX: c_uint = 0x0001 /* RX2TX */;
pub const B43_PHY_HT_RF_SEQ_TRIG_TX2RX: c_uint = 0x0002 /* TX2RX */;
pub const B43_PHY_HT_RF_SEQ_TRIG_UPGH: c_uint = 0x0004 /* Update gain H */;
pub const B43_PHY_HT_RF_SEQ_TRIG_UPGL: c_uint = 0x0008 /* Update gain L */;
pub const B43_PHY_HT_RF_SEQ_TRIG_UPGU: c_uint = 0x0010 /* Update gain U */;
pub const B43_PHY_HT_RF_SEQ_TRIG_RST2RX: c_uint = 0x0020 /* Reset to RX */;

// Values for the status are the same as for the trigger
pub const B43_PHY_HT_RF_CTL_CMD: c_uint = 0x810;
pub const B43_PHY_HT_RF_CTL_CMD_FORCE: c_uint = 0x0001;
pub const B43_PHY_HT_RF_CTL_CMD_CHIP0_PU: c_uint = 0x0002;

pub const B43_PHY_HT_TXPCTL_CMD_C3_INIT: c_uint = 0x007F;

pub const B43_PHY_HT_TXPCTL_IDLE_TSSI2_C3: c_uint = 0x003F;
pub const B43_PHY_HT_TXPCTL_IDLE_TSSI2_C3_SHIFT: c_int = 0;

pub const B43_PHY_HT_TXPCTL_TARG_PWR2_C3: c_uint = 0x00FF;
pub const B43_PHY_HT_TXPCTL_TARG_PWR2_C3_SHIFT: c_int = 0;

pub const B43_PHY_B_BBCFG_RSTCCA: c_uint = 0x4000 /* Reset CCA */;
pub const B43_PHY_B_BBCFG_RSTRX: c_uint = 0x8000 /* Reset RX */;

// Values for PHY registers used on channel switching
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_ht_channeltab_e_phy {
    pub bw1: u16,
    pub bw2: u16,
    pub bw3: u16,
    pub bw4: u16,
    pub bw5: u16,
    pub bw6: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_ht {
    pub rf_ctl_int_save: [u16; 3],
    pub tx_pwr_ctl: bool,
    pub tx_pwr_idx: [u8; 3],
    pub bb_mult_save: [i32; 3],
    pub idle_tssi: [u8; 3],
}
