//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mdio.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// linux/mdio.h: definitions for MDIO (clause 45) transceivers
// Copyright 2006-2009 Solarflare Communications Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

// MDIO Manageable Devices (MMDs).

// Physical Medium Dependent

// Generic MDIO registers.

pub const MDIO_DEVS2: c_int = 6;

pub const MDIO_PKGID2: c_int = 15;

// Media-dependent registers.

// Lanes B-D are numbered 134-136.

// LASI (Link Alarm Status Interrupt) registers, defined by XENPAK MSA.
pub const MDIO_PMA_LASI_RXCTRL: c_uint = 0x9000	/* RX_ALARM control */;
pub const MDIO_PMA_LASI_TXCTRL: c_uint = 0x9001	/* TX_ALARM control */;
pub const MDIO_PMA_LASI_CTRL: c_uint = 0x9002	/* LASI control */;
pub const MDIO_PMA_LASI_RXSTAT: c_uint = 0x9003	/* RX_ALARM status */;
pub const MDIO_PMA_LASI_TXSTAT: c_uint = 0x9004	/* TX_ALARM status */;
pub const MDIO_PMA_LASI_STAT: c_uint = 0x9005	/* LASI status */;
// Control register 1.
// Enable extended speed selection

// All speed selection bits

pub const MDIO_PMA_CTRL1_LOOPBACK: c_uint = 0x0001;

pub const MDIO_AN_CTRL1_XNP: c_uint = 0x2000	/* Enable extended next page */;
pub const MDIO_PCS_CTRL1_CLKSTOP_EN: c_uint = 0x400	/* Stop the clock during LPI */;
// 10 Gb/s

// 10PASS-TS/2BASE-TL

// Note: the MDIO_CTRL1_SPEED_XXX values for everything past 10PASS-TS/2BASE-TL
// do not match between the PCS and PMA values. Any additions past this point
// should be PMA or PCS specific. The following 2 defines are workarounds for
// values added before this was caught. They should be considered deprecated.
//

// 100 Gb/s

// 25 Gb/s

// 50 Gb/s

// 2.5 Gb/s

// 5 Gb/s

// Status register 1.
pub const MDIO_STAT1_LPOWERABLE: c_uint = 0x0002	/* Low-power ability */;

pub const MDIO_STAT1_FAULT: c_uint = 0x0080	/* Fault */;
pub const MDIO_PCS_STAT1_CLKSTOP_CAP: c_uint = 0x0040;
pub const MDIO_AN_STAT1_LPABLE: c_uint = 0x0001	/* Link partner AN ability */;

pub const MDIO_AN_STAT1_PAGE: c_uint = 0x0040	/* Page received */;
pub const MDIO_AN_STAT1_XNP: c_uint = 0x0080	/* Extended next page status */;
// Device Identifier 2
pub const MDIO_DEVID2_OUI: c_uint = 0xfc00	/* OUI Portion of PHY ID */;
pub const MDIO_DEVID2_MODEL_NUM: c_uint = 0x03f0	/* Manufacturer's Model Number */;
pub const MDIO_DEVID2_REV_NUM: c_uint = 0x000f	/* Revision Number */;
// Speed register.
pub const MDIO_SPEED_10G: c_uint = 0x0001	/* 10G capable */;
pub const MDIO_PMA_SPEED_2B: c_uint = 0x0002	/* 2BASE-TL capable */;
pub const MDIO_PMA_SPEED_10P: c_uint = 0x0004	/* 10PASS-TS capable */;
pub const MDIO_PMA_SPEED_1000: c_uint = 0x0010	/* 1000M capable */;
pub const MDIO_PMA_SPEED_100: c_uint = 0x0020	/* 100M capable */;
pub const MDIO_PMA_SPEED_10: c_uint = 0x0040	/* 10M capable */;
pub const MDIO_PMA_SPEED_2_5G: c_uint = 0x2000	/* 2.5G capable */;
pub const MDIO_PMA_SPEED_5G: c_uint = 0x4000	/* 5G capable */;
pub const MDIO_PCS_SPEED_10P2B: c_uint = 0x0002	/* 10PASS-TS/2BASE-TL capable */;
pub const MDIO_PCS_SPEED_2_5G: c_uint = 0x0040	/* 2.5G capable */;
pub const MDIO_PCS_SPEED_5G: c_uint = 0x0080	/* 5G capable */;
// Device present registers.

// Control register 2.
pub const MDIO_PMA_CTRL2_TYPE: c_uint = 0x000f	/* PMA/PMD type selection */;
pub const MDIO_PMA_CTRL2_10GBCX4: c_uint = 0x0000	/* 10GBASE-CX4 type */;
pub const MDIO_PMA_CTRL2_10GBEW: c_uint = 0x0001	/* 10GBASE-EW type */;
pub const MDIO_PMA_CTRL2_10GBLW: c_uint = 0x0002	/* 10GBASE-LW type */;
pub const MDIO_PMA_CTRL2_10GBSW: c_uint = 0x0003	/* 10GBASE-SW type */;
pub const MDIO_PMA_CTRL2_10GBLX4: c_uint = 0x0004	/* 10GBASE-LX4 type */;
pub const MDIO_PMA_CTRL2_10GBER: c_uint = 0x0005	/* 10GBASE-ER type */;
pub const MDIO_PMA_CTRL2_10GBLR: c_uint = 0x0006	/* 10GBASE-LR type */;
pub const MDIO_PMA_CTRL2_10GBSR: c_uint = 0x0007	/* 10GBASE-SR type */;
pub const MDIO_PMA_CTRL2_10GBLRM: c_uint = 0x0008	/* 10GBASE-LRM type */;
pub const MDIO_PMA_CTRL2_10GBT: c_uint = 0x0009	/* 10GBASE-T type */;
pub const MDIO_PMA_CTRL2_10GBKX4: c_uint = 0x000a	/* 10GBASE-KX4 type */;
pub const MDIO_PMA_CTRL2_10GBKR: c_uint = 0x000b	/* 10GBASE-KR type */;
pub const MDIO_PMA_CTRL2_1000BT: c_uint = 0x000c	/* 1000BASE-T type */;
pub const MDIO_PMA_CTRL2_1000BKX: c_uint = 0x000d	/* 1000BASE-KX type */;
pub const MDIO_PMA_CTRL2_100BTX: c_uint = 0x000e	/* 100BASE-TX type */;
pub const MDIO_PMA_CTRL2_10BT: c_uint = 0x000f	/* 10BASE-T type */;
pub const MDIO_PMA_CTRL2_2_5GBT: c_uint = 0x0030  /* 2.5GBaseT type */;
pub const MDIO_PMA_CTRL2_5GBT: c_uint = 0x0031  /* 5GBaseT type */;
pub const MDIO_PMA_CTRL2_BASET1: c_uint = 0x003D  /* BASE-T1 type */;
pub const MDIO_PCS_CTRL2_TYPE: c_uint = 0x0003	/* PCS type selection */;
pub const MDIO_PCS_CTRL2_10GBR: c_uint = 0x0000	/* 10GBASE-R type */;
pub const MDIO_PCS_CTRL2_10GBX: c_uint = 0x0001	/* 10GBASE-X type */;
pub const MDIO_PCS_CTRL2_10GBW: c_uint = 0x0002	/* 10GBASE-W type */;
pub const MDIO_PCS_CTRL2_10GBT: c_uint = 0x0003	/* 10GBASE-T type */;
// Status register 2.
pub const MDIO_STAT2_RXFAULT: c_uint = 0x0400	/* Receive fault */;
pub const MDIO_STAT2_TXFAULT: c_uint = 0x0800	/* Transmit fault */;
pub const MDIO_STAT2_DEVPRST: c_uint = 0xc000	/* Device present */;
pub const MDIO_STAT2_DEVPRST_VAL: c_uint = 0x8000	/* Device present value */;
pub const MDIO_PMA_STAT2_LBABLE: c_uint = 0x0001	/* PMA loopback ability */;
pub const MDIO_PMA_STAT2_10GBEW: c_uint = 0x0002	/* 10GBASE-EW ability */;
pub const MDIO_PMA_STAT2_10GBLW: c_uint = 0x0004	/* 10GBASE-LW ability */;
pub const MDIO_PMA_STAT2_10GBSW: c_uint = 0x0008	/* 10GBASE-SW ability */;
pub const MDIO_PMA_STAT2_10GBLX4: c_uint = 0x0010	/* 10GBASE-LX4 ability */;
pub const MDIO_PMA_STAT2_10GBER: c_uint = 0x0020	/* 10GBASE-ER ability */;
pub const MDIO_PMA_STAT2_10GBLR: c_uint = 0x0040	/* 10GBASE-LR ability */;
pub const MDIO_PMA_STAT2_10GBSR: c_uint = 0x0080	/* 10GBASE-SR ability */;
pub const MDIO_PMD_STAT2_TXDISAB: c_uint = 0x0100	/* PMD TX disable ability */;
pub const MDIO_PMA_STAT2_EXTABLE: c_uint = 0x0200	/* Extended abilities */;
pub const MDIO_PMA_STAT2_RXFLTABLE: c_uint = 0x1000	/* Receive fault ability */;
pub const MDIO_PMA_STAT2_TXFLTABLE: c_uint = 0x2000	/* Transmit fault ability */;
pub const MDIO_PCS_STAT2_10GBR: c_uint = 0x0001	/* 10GBASE-R capable */;
pub const MDIO_PCS_STAT2_10GBX: c_uint = 0x0002	/* 10GBASE-X capable */;
pub const MDIO_PCS_STAT2_10GBW: c_uint = 0x0004	/* 10GBASE-W capable */;
pub const MDIO_PCS_STAT2_RXFLTABLE: c_uint = 0x1000	/* Receive fault ability */;
pub const MDIO_PCS_STAT2_TXFLTABLE: c_uint = 0x2000	/* Transmit fault ability */;
// Transmit disable register.
pub const MDIO_PMD_TXDIS_GLOBAL: c_uint = 0x0001	/* Global PMD TX disable */;
pub const MDIO_PMD_TXDIS_0: c_uint = 0x0002	/* PMD TX disable 0 */;
pub const MDIO_PMD_TXDIS_1: c_uint = 0x0004	/* PMD TX disable 1 */;
pub const MDIO_PMD_TXDIS_2: c_uint = 0x0008	/* PMD TX disable 2 */;
pub const MDIO_PMD_TXDIS_3: c_uint = 0x0010	/* PMD TX disable 3 */;
// Receive signal detect register.
pub const MDIO_PMD_RXDET_GLOBAL: c_uint = 0x0001	/* Global PMD RX signal detect */;
pub const MDIO_PMD_RXDET_0: c_uint = 0x0002	/* PMD RX signal detect 0 */;
pub const MDIO_PMD_RXDET_1: c_uint = 0x0004	/* PMD RX signal detect 1 */;
pub const MDIO_PMD_RXDET_2: c_uint = 0x0008	/* PMD RX signal detect 2 */;
pub const MDIO_PMD_RXDET_3: c_uint = 0x0010	/* PMD RX signal detect 3 */;
// Extended abilities register.
pub const MDIO_PMA_EXTABLE_10GCX4: c_uint = 0x0001	/* 10GBASE-CX4 ability */;
pub const MDIO_PMA_EXTABLE_10GBLRM: c_uint = 0x0002	/* 10GBASE-LRM ability */;
pub const MDIO_PMA_EXTABLE_10GBT: c_uint = 0x0004	/* 10GBASE-T ability */;
pub const MDIO_PMA_EXTABLE_10GBKX4: c_uint = 0x0008	/* 10GBASE-KX4 ability */;
pub const MDIO_PMA_EXTABLE_10GBKR: c_uint = 0x0010	/* 10GBASE-KR ability */;
pub const MDIO_PMA_EXTABLE_1000BT: c_uint = 0x0020	/* 1000BASE-T ability */;
pub const MDIO_PMA_EXTABLE_1000BKX: c_uint = 0x0040	/* 1000BASE-KX ability */;
pub const MDIO_PMA_EXTABLE_100BTX: c_uint = 0x0080	/* 100BASE-TX ability */;
pub const MDIO_PMA_EXTABLE_10BT: c_uint = 0x0100	/* 10BASE-T ability */;
pub const MDIO_PMA_EXTABLE_BT1: c_uint = 0x0800	/* BASE-T1 ability */;
pub const MDIO_PMA_EXTABLE_NBT: c_uint = 0x4000  /* 2.5/5GBASE-T ability */;
// AN Clause 73 linkword

// PHY XGXS lane state register.
pub const MDIO_PHYXS_LNSTAT_SYNC0: c_uint = 0x0001;
pub const MDIO_PHYXS_LNSTAT_SYNC1: c_uint = 0x0002;
pub const MDIO_PHYXS_LNSTAT_SYNC2: c_uint = 0x0004;
pub const MDIO_PHYXS_LNSTAT_SYNC3: c_uint = 0x0008;
pub const MDIO_PHYXS_LNSTAT_ALIGN: c_uint = 0x1000;
// PMA 10GBASE-T pair swap & polarity
pub const MDIO_PMA_10GBT_SWAPPOL_ABNX: c_uint = 0x0001	/* Pair A/B uncrossed */;
pub const MDIO_PMA_10GBT_SWAPPOL_CDNX: c_uint = 0x0002	/* Pair C/D uncrossed */;
pub const MDIO_PMA_10GBT_SWAPPOL_AREV: c_uint = 0x0100	/* Pair A polarity reversed */;
pub const MDIO_PMA_10GBT_SWAPPOL_BREV: c_uint = 0x0200	/* Pair B polarity reversed */;
pub const MDIO_PMA_10GBT_SWAPPOL_CREV: c_uint = 0x0400	/* Pair C polarity reversed */;
pub const MDIO_PMA_10GBT_SWAPPOL_DREV: c_uint = 0x0800	/* Pair D polarity reversed */;
// PMA 10GBASE-T TX power register.
pub const MDIO_PMA_10GBT_TXPWR_SHORT: c_uint = 0x0001	/* Short-reach mode */;
// PMA 10GBASE-T SNR registers.
// Value is SNR margin in dB, clamped to range [-127, 127], plus 0x8000.
pub const MDIO_PMA_10GBT_SNR_BIAS: c_uint = 0x8000;
pub const MDIO_PMA_10GBT_SNR_MAX: c_int = 127;
// PMA 10GBASE-R FEC ability register.
pub const MDIO_PMA_10GBR_FECABLE_ABLE: c_uint = 0x0001	/* FEC ability */;
pub const MDIO_PMA_10GBR_FECABLE_ERRABLE: c_uint = 0x0002	/* FEC error indic. ability */;
// PMA 10GBASE-R Fast Retrain status and control register.
pub const MDIO_PMA_10GBR_FSRT_ENABLE: c_uint = 0x0001	/* Fast retrain enable */;
// PCS 10GBASE-R/-T status register 1.
pub const MDIO_PCS_10GBRT_STAT1_BLKLK: c_uint = 0x0001	/* Block lock attained */;
// PCS 10GBASE-R/-T status register 2.
pub const MDIO_PCS_10GBRT_STAT2_ERR: c_uint = 0x00ff;
pub const MDIO_PCS_10GBRT_STAT2_BER: c_uint = 0x3f00;
// AN 10GBASE-T control register.
pub const MDIO_AN_10GBT_CTRL_ADVFSRT2_5G: c_uint = 0x0020	/* Advertise 2.5GBASE-T fast retrain */;
pub const MDIO_AN_10GBT_CTRL_ADV2_5G: c_uint = 0x0080	/* Advertise 2.5GBASE-T */;
pub const MDIO_AN_10GBT_CTRL_ADV5G: c_uint = 0x0100	/* Advertise 5GBASE-T */;
pub const MDIO_AN_10GBT_CTRL_ADV10G: c_uint = 0x1000	/* Advertise 10GBASE-T */;
pub const MDIO_AN_10GBT_CTRL_MS_ENABLE: c_uint = 0x8000	/* Master/slave manual config enable */;
pub const MDIO_AN_10GBT_CTRL_MS_VALUE: c_uint = 0x4000	/* Master/slave config value (1=Master) */;
pub const MDIO_AN_10GBT_CTRL_MS_PORT_TYPE: c_uint = 0x2000	/* Master Preferred Type */;
// AN 10GBASE-T status register.
pub const MDIO_AN_10GBT_STAT_MS_FAULT: c_uint = 0x8000	/* Master/slave fault */;
pub const MDIO_AN_10GBT_STAT_MS_RES: c_uint = 0x4000	/* Master/slave resolution (1=Master) */;
pub const MDIO_AN_10GBT_STAT_LP2_5G: c_uint = 0x0020  /* LP is 2.5GBT capable */;
pub const MDIO_AN_10GBT_STAT_LP5G: c_uint = 0x0040  /* LP is 5GBT capable */;
pub const MDIO_AN_10GBT_STAT_LPTRR: c_uint = 0x0200	/* LP training reset req. */;
pub const MDIO_AN_10GBT_STAT_LPLTABLE: c_uint = 0x0400	/* LP loop timing ability */;
pub const MDIO_AN_10GBT_STAT_LP10G: c_uint = 0x0800	/* LP is 10GBT capable */;
pub const MDIO_AN_10GBT_STAT_REMOK: c_uint = 0x1000	/* Remote OK */;
pub const MDIO_AN_10GBT_STAT_LOCOK: c_uint = 0x2000	/* Local OK */;
pub const MDIO_AN_10GBT_STAT_MS: c_uint = 0x4000	/* Master/slave config */;
pub const MDIO_AN_10GBT_STAT_MSFLT: c_uint = 0x8000	/* Master/slave config fault */;
// 10BASE-T1L PMA control
pub const MDIO_PMA_10T1L_CTRL_LB_EN: c_uint = 0x0001	/* Enable loopback mode */;
pub const MDIO_PMA_10T1L_CTRL_EEE_EN: c_uint = 0x0400	/* Enable EEE mode */;
pub const MDIO_PMA_10T1L_CTRL_LOW_POWER: c_uint = 0x0800	/* Low-power mode */;
pub const MDIO_PMA_10T1L_CTRL_2V4_EN: c_uint = 0x1000	/* Enable 2.4 Vpp operating mode */;
pub const MDIO_PMA_10T1L_CTRL_TX_DIS: c_uint = 0x4000	/* Transmit disable */;
pub const MDIO_PMA_10T1L_CTRL_PMA_RST: c_uint = 0x8000	/* MA reset */;
// 10BASE-T1L PMA status register.
pub const MDIO_PMA_10T1L_STAT_LINK: c_uint = 0x0001	/* PMA receive link up */;
pub const MDIO_PMA_10T1L_STAT_FAULT: c_uint = 0x0002	/* Fault condition detected */;
pub const MDIO_PMA_10T1L_STAT_POLARITY: c_uint = 0x0004	/* Receive polarity is reversed */;
pub const MDIO_PMA_10T1L_STAT_RECV_FAULT: c_uint = 0x0200	/* Able to detect fault on receive path */;
pub const MDIO_PMA_10T1L_STAT_EEE: c_uint = 0x0400	/* PHY has EEE ability */;
pub const MDIO_PMA_10T1L_STAT_LOW_POWER: c_uint = 0x0800	/* PMA has low-power ability */;
pub const MDIO_PMA_10T1L_STAT_2V4_ABLE: c_uint = 0x1000	/* PHY has 2.4 Vpp operating mode ability */;
pub const MDIO_PMA_10T1L_STAT_LB_ABLE: c_uint = 0x2000	/* PHY has loopback ability */;
// 10BASE-T1L PCS control register.
pub const MDIO_PCS_10T1L_CTRL_LB: c_uint = 0x4000	/* Enable PCS level loopback mode */;
pub const MDIO_PCS_10T1L_CTRL_RESET: c_uint = 0x8000	/* PCS reset */;
// BASE-T1 PMA/PMD extended ability register.
pub const MDIO_PMA_PMD_BT1_B100_ABLE: c_uint = 0x0001	/* 100BASE-T1 Ability */;
pub const MDIO_PMA_PMD_BT1_B1000_ABLE: c_uint = 0x0002	/* 1000BASE-T1 Ability */;
pub const MDIO_PMA_PMD_BT1_B10L_ABLE: c_uint = 0x0004	/* 10BASE-T1L Ability */;
// BASE-T1 auto-negotiation advertisement register [15:0]

pub const MDIO_AN_T1_ADV_L_FORCE_MS: c_uint = 0x1000	/* Force Master/slave Configuration */;

// BASE-T1 auto-negotiation advertisement register [31:16]
pub const MDIO_AN_T1_ADV_M_B10L: c_uint = 0x4000	/* device is compatible with 10BASE-T1L */;
pub const MDIO_AN_T1_ADV_M_1000BT1: c_uint = 0x0080	/* advertise 1000BASE-T1 */;
pub const MDIO_AN_T1_ADV_M_100BT1: c_uint = 0x0020	/* advertise 100BASE-T1 */;
pub const MDIO_AN_T1_ADV_M_MST: c_uint = 0x0010	/* advertise master preference */;
// BASE-T1 auto-negotiation advertisement register [47:32]
pub const MDIO_AN_T1_ADV_H_10L_TX_HI_REQ: c_uint = 0x1000	/* 10BASE-T1L High Level Transmit Request */;
pub const MDIO_AN_T1_ADV_H_10L_TX_HI: c_uint = 0x2000	/* 10BASE-T1L High Level Transmit Ability */;
// BASE-T1 AN LP Base Page ability register [15:0]

pub const MDIO_AN_T1_LP_L_FORCE_MS: c_uint = 0x1000	/* LP Force Master/slave Configuration */;

// BASE-T1 AN LP Base Page ability register [31:16]
pub const MDIO_AN_T1_LP_M_MST: c_uint = 0x0010	/* LP master preference */;
pub const MDIO_AN_T1_LP_M_B10L: c_uint = 0x4000	/* LP is compatible with 10BASE-T1L */;
// BASE-T1 AN LP Base Page ability register [47:32]
pub const MDIO_AN_T1_LP_H_10L_TX_HI_REQ: c_uint = 0x1000	/* 10BASE-T1L High Level LP Transmit Request */;
pub const MDIO_AN_T1_LP_H_10L_TX_HI: c_uint = 0x2000	/* 10BASE-T1L High Level LP Transmit Ability */;
// 10BASE-T1 AN control register
pub const MDIO_AN_10BT1_AN_CTRL_ADV_EEE_T1L: c_uint = 0x4000 /* 10BASE-T1L EEE ability advertisement */;
// 10BASE-T1 AN status register
pub const MDIO_AN_10BT1_AN_STAT_LPA_EEE_T1L: c_uint = 0x4000 /* 10BASE-T1L LP EEE ability advertisement */;
// BASE-T1 PMA/PMD control register
pub const MDIO_PMA_PMD_BT1_CTRL_STRAP: c_uint = 0x000F /* Type selection (Strap) */;
pub const MDIO_PMA_PMD_BT1_CTRL_STRAP_B1000: c_uint = 0x0001 /* Select 1000BASE-T1 */;
pub const MDIO_PMA_PMD_BT1_CTRL_CFG_MST: c_uint = 0x4000 /* MASTER-SLAVE config value */;
// 1000BASE-T1 PCS control register
pub const MDIO_PCS_1000BT1_CTRL_LOW_POWER: c_uint = 0x0800 /* Low power mode */;
pub const MDIO_PCS_1000BT1_CTRL_DISABLE_TX: c_uint = 0x4000 /* Global PMA transmit disable */;
pub const MDIO_PCS_1000BT1_CTRL_RESET: c_uint = 0x8000 /* Software reset value */;
// 1000BASE-T1 PCS status register
pub const MDIO_PCS_1000BT1_STAT_LINK: c_uint = 0x0004 /* PCS Link is up */;
pub const MDIO_PCS_1000BT1_STAT_FAULT: c_uint = 0x0080 /* There is a fault condition */;
// EEE Supported/Advertisement/LP Advertisement registers.
//
// EEE capability Register (3.20), Advertisement (7.60) and
// Link partner ability (7.61) registers have and can use the same identical
// bit masks.
//
pub const MDIO_AN_EEE_ADV_100TX: c_uint = 0x0002	/* Advertise 100TX EEE cap */;
pub const MDIO_AN_EEE_ADV_1000T: c_uint = 0x0004	/* Advertise 1000T EEE cap */;
// Note: the two defines above can be potentially used by the user-land
// and cannot remove them now.
// So, we define the new generic MDIO_EEE_100TX and MDIO_EEE_1000T macros
// using the previous ones (that can be considered obsolete).
//

pub const MDIO_EEE_10GT: c_uint = 0x0008	/* 10GT EEE cap */;
pub const MDIO_EEE_1000KX: c_uint = 0x0010	/* 1000KX EEE cap */;
pub const MDIO_EEE_10GKX4: c_uint = 0x0020	/* 10G KX4 EEE cap */;
pub const MDIO_EEE_10GKR: c_uint = 0x0040	/* 10G KR EEE cap */;
pub const MDIO_EEE_40GR_FW: c_uint = 0x0100	/* 40G R fast wake */;
pub const MDIO_EEE_40GR_DS: c_uint = 0x0200	/* 40G R deep sleep */;
pub const MDIO_EEE_100GR_FW: c_uint = 0x1000	/* 100G R fast wake */;
pub const MDIO_EEE_100GR_DS: c_uint = 0x2000	/* 100G R deep sleep */;
pub const MDIO_EEE_2_5GT: c_uint = 0x0001	/* 2.5GT EEE cap */;
pub const MDIO_EEE_5GT: c_uint = 0x0002	/* 5GT EEE cap */;
// AN MultiGBASE-T AN control 2
pub const MDIO_AN_THP_BP2_5GT: c_uint = 0x0008	/* 2.5GT THP bypass request */;
// 2.5G/5G Extended abilities register.
pub const MDIO_PMA_NG_EXTABLE_2_5GBT: c_uint = 0x0001	/* 2.5GBASET ability */;
pub const MDIO_PMA_NG_EXTABLE_5GBT: c_uint = 0x0002	/* 5GBASET ability */;
// LASI RX_ALARM control/status registers.
pub const MDIO_PMA_LASI_RX_PHYXSLFLT: c_uint = 0x0001	/* PHY XS RX local fault */;
pub const MDIO_PMA_LASI_RX_PCSLFLT: c_uint = 0x0008	/* PCS RX local fault */;
pub const MDIO_PMA_LASI_RX_PMALFLT: c_uint = 0x0010	/* PMA/PMD RX local fault */;
pub const MDIO_PMA_LASI_RX_OPTICPOWERFLT: c_uint = 0x0020	/* RX optical power fault */;
pub const MDIO_PMA_LASI_RX_WISLFLT: c_uint = 0x0200	/* WIS local fault */;
// LASI TX_ALARM control/status registers.
pub const MDIO_PMA_LASI_TX_PHYXSLFLT: c_uint = 0x0001	/* PHY XS TX local fault */;
pub const MDIO_PMA_LASI_TX_PCSLFLT: c_uint = 0x0008	/* PCS TX local fault */;
pub const MDIO_PMA_LASI_TX_PMALFLT: c_uint = 0x0010	/* PMA/PMD TX local fault */;
pub const MDIO_PMA_LASI_TX_LASERPOWERFLT: c_uint = 0x0080	/* Laser output power fault */;
pub const MDIO_PMA_LASI_TX_LASERTEMPFLT: c_uint = 0x0100	/* Laser temperature fault */;
pub const MDIO_PMA_LASI_TX_LASERBICURRFLT: c_uint = 0x0200	/* Laser bias current fault */;
// LASI control/status registers.
pub const MDIO_PMA_LASI_LSALARM: c_uint = 0x0001	/* LS_ALARM enable/status */;
pub const MDIO_PMA_LASI_TXALARM: c_uint = 0x0002	/* TX_ALARM enable/status */;
pub const MDIO_PMA_LASI_RXALARM: c_uint = 0x0004	/* RX_ALARM enable/status */;
// Mapping between MDIO PRTAD/DEVAD and mii_ioctl_data::phy_id
pub const MDIO_PHY_ID_C45: c_uint = 0x8000;
pub const MDIO_PHY_ID_PRTAD: c_uint = 0x03e0;
pub const MDIO_PHY_ID_DEVAD: c_uint = 0x001f;

// UsxgmiiChannelInfo[15:0] for USXGMII in-band auto-negotiation.
pub const MDIO_USXGMII_EEE_CLK_STP: c_uint = 0x0080	/* EEE clock stop supported */;
pub const MDIO_USXGMII_EEE: c_uint = 0x0100	/* EEE supported */;
pub const MDIO_USXGMII_SPD_MASK: c_uint = 0x0e00	/* USXGMII speed mask */;
pub const MDIO_USXGMII_FULL_DUPLEX: c_uint = 0x1000	/* USXGMII full duplex */;
pub const MDIO_USXGMII_DPX_SPD_MASK: c_uint = 0x1e00	/* USXGMII duplex and speed bits */;
pub const MDIO_USXGMII_10: c_uint = 0x0000	/* 10Mbps */;
pub const MDIO_USXGMII_10HALF: c_uint = 0x0000	/* 10Mbps half-duplex */;
pub const MDIO_USXGMII_10FULL: c_uint = 0x1000	/* 10Mbps full-duplex */;
pub const MDIO_USXGMII_100: c_uint = 0x0200	/* 100Mbps */;
pub const MDIO_USXGMII_100HALF: c_uint = 0x0200	/* 100Mbps half-duplex */;
pub const MDIO_USXGMII_100FULL: c_uint = 0x1200	/* 100Mbps full-duplex */;
pub const MDIO_USXGMII_1000: c_uint = 0x0400	/* 1000Mbps */;
pub const MDIO_USXGMII_1000HALF: c_uint = 0x0400	/* 1000Mbps half-duplex */;
pub const MDIO_USXGMII_1000FULL: c_uint = 0x1400	/* 1000Mbps full-duplex */;
pub const MDIO_USXGMII_10G: c_uint = 0x0600	/* 10Gbps */;
pub const MDIO_USXGMII_10GHALF: c_uint = 0x0600	/* 10Gbps half-duplex */;
pub const MDIO_USXGMII_10GFULL: c_uint = 0x1600	/* 10Gbps full-duplex */;
pub const MDIO_USXGMII_2500: c_uint = 0x0800	/* 2500Mbps */;
pub const MDIO_USXGMII_2500HALF: c_uint = 0x0800	/* 2500Mbps half-duplex */;
pub const MDIO_USXGMII_2500FULL: c_uint = 0x1800	/* 2500Mbps full-duplex */;
pub const MDIO_USXGMII_5000: c_uint = 0x0a00	/* 5000Mbps */;
pub const MDIO_USXGMII_5000HALF: c_uint = 0x0a00	/* 5000Mbps half-duplex */;
pub const MDIO_USXGMII_5000FULL: c_uint = 0x1a00	/* 5000Mbps full-duplex */;
pub const MDIO_USXGMII_LINK: c_uint = 0x8000	/* PHY link with copper-side partner */;
