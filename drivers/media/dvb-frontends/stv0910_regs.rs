//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0910_regs.h
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
//
// @DVB-S/DVB-S2 STMicroelectronics STV0900 register definitions
// Author Manfred Voelkel, August 2013
// (c) 2013 Digital Devices GmbH Germany.  All rights reserved
//
// =======================================================================
// Registers Declaration (Internal ST, All Applications )
// -------------------------
// Each register (RSTV0910__XXXXX) is defined by its address (2 bytes).
// Each field (FSTV0910__XXXXX) is defined as follow:
// [register address -- 2bytes][field offset -- 4 bits][unused -- 3 bits]
// [field sign -- 1 bit][field mask -- 1byte]
// =======================================================================
//
// MID
pub const RSTV0910_MID: c_uint = 0xf100;
pub const FSTV0910_MCHIP_IDENT: c_uint = 0xf10040f0;
pub const FSTV0910_MRELEASE: c_uint = 0xf100000f;
// DID
pub const RSTV0910_DID: c_uint = 0xf101;
pub const FSTV0910_DEVICE_ID: c_uint = 0xf10100ff;
// DACR1
pub const RSTV0910_DACR1: c_uint = 0xf113;
pub const FSTV0910_DAC_MODE: c_uint = 0xf11350e0;
pub const FSTV0910_DAC_VALUE1: c_uint = 0xf113000f;
// DACR2
pub const RSTV0910_DACR2: c_uint = 0xf114;
pub const FSTV0910_DAC_VALUE0: c_uint = 0xf11400ff;
// PADCFG
pub const RSTV0910_PADCFG: c_uint = 0xf11a;
pub const FSTV0910_AGCRF2_OPD: c_uint = 0xf11a3008;
pub const FSTV0910_AGCRF2_XOR: c_uint = 0xf11a2004;
pub const FSTV0910_AGCRF1_OPD: c_uint = 0xf11a1002;
pub const FSTV0910_AGCRF1_XOR: c_uint = 0xf11a0001;
// OUTCFG2
pub const RSTV0910_OUTCFG2: c_uint = 0xf11b;
pub const FSTV0910_TS2_ERROR_XOR: c_uint = 0xf11b7080;
pub const FSTV0910_TS2_DPN_XOR: c_uint = 0xf11b6040;
pub const FSTV0910_TS2_STROUT_XOR: c_uint = 0xf11b5020;
pub const FSTV0910_TS2_CLOCKOUT_XOR: c_uint = 0xf11b4010;
pub const FSTV0910_TS1_ERROR_XOR: c_uint = 0xf11b3008;
pub const FSTV0910_TS1_DPN_XOR: c_uint = 0xf11b2004;
pub const FSTV0910_TS1_STROUT_XOR: c_uint = 0xf11b1002;
pub const FSTV0910_TS1_CLOCKOUT_XOR: c_uint = 0xf11b0001;
// OUTCFG
pub const RSTV0910_OUTCFG: c_uint = 0xf11c;
pub const FSTV0910_TS2_OUTSER_HZ: c_uint = 0xf11c5020;
pub const FSTV0910_TS1_OUTSER_HZ: c_uint = 0xf11c4010;
pub const FSTV0910_TS2_OUTPAR_HZ: c_uint = 0xf11c3008;
pub const FSTV0910_TS1_OUTPAR_HZ: c_uint = 0xf11c2004;
pub const FSTV0910_TS_SERDATA0: c_uint = 0xf11c1002;
// IRQSTATUS3
pub const RSTV0910_IRQSTATUS3: c_uint = 0xf120;
pub const FSTV0910_SPLL_LOCK: c_uint = 0xf1205020;
pub const FSTV0910_SSTREAM_LCK_1: c_uint = 0xf1204010;
pub const FSTV0910_SSTREAM_LCK_2: c_uint = 0xf1203008;
pub const FSTV0910_SDVBS1_PRF_2: c_uint = 0xf1201002;
pub const FSTV0910_SDVBS1_PRF_1: c_uint = 0xf1200001;
// IRQSTATUS2
pub const RSTV0910_IRQSTATUS2: c_uint = 0xf121;
pub const FSTV0910_SSPY_ENDSIM_1: c_uint = 0xf1217080;
pub const FSTV0910_SSPY_ENDSIM_2: c_uint = 0xf1216040;
pub const FSTV0910_SPKTDEL_ERROR_2: c_uint = 0xf1214010;
pub const FSTV0910_SPKTDEL_LOCKB_2: c_uint = 0xf1213008;
pub const FSTV0910_SPKTDEL_LOCK_2: c_uint = 0xf1212004;
pub const FSTV0910_SPKTDEL_ERROR_1: c_uint = 0xf1211002;
pub const FSTV0910_SPKTDEL_LOCKB_1: c_uint = 0xf1210001;
// IRQSTATUS1
pub const RSTV0910_IRQSTATUS1: c_uint = 0xf122;
pub const FSTV0910_SPKTDEL_LOCK_1: c_uint = 0xf1227080;
pub const FSTV0910_SFEC_LOCKB_2: c_uint = 0xf1226040;
pub const FSTV0910_SFEC_LOCK_2: c_uint = 0xf1225020;
pub const FSTV0910_SFEC_LOCKB_1: c_uint = 0xf1224010;
pub const FSTV0910_SFEC_LOCK_1: c_uint = 0xf1223008;
pub const FSTV0910_SDEMOD_LOCKB_2: c_uint = 0xf1222004;
pub const FSTV0910_SDEMOD_LOCK_2: c_uint = 0xf1221002;
pub const FSTV0910_SDEMOD_IRQ_2: c_uint = 0xf1220001;
// IRQSTATUS0
pub const RSTV0910_IRQSTATUS0: c_uint = 0xf123;
pub const FSTV0910_SDEMOD_LOCKB_1: c_uint = 0xf1237080;
pub const FSTV0910_SDEMOD_LOCK_1: c_uint = 0xf1236040;
pub const FSTV0910_SDEMOD_IRQ_1: c_uint = 0xf1235020;
pub const FSTV0910_SBCH_ERRFLAG: c_uint = 0xf1234010;
pub const FSTV0910_SDISEQC2_IRQ: c_uint = 0xf1232004;
pub const FSTV0910_SDISEQC1_IRQ: c_uint = 0xf1230001;
// IRQMASK3
pub const RSTV0910_IRQMASK3: c_uint = 0xf124;
pub const FSTV0910_MPLL_LOCK: c_uint = 0xf1245020;
pub const FSTV0910_MSTREAM_LCK_1: c_uint = 0xf1244010;
pub const FSTV0910_MSTREAM_LCK_2: c_uint = 0xf1243008;
pub const FSTV0910_MDVBS1_PRF_2: c_uint = 0xf1241002;
pub const FSTV0910_MDVBS1_PRF_1: c_uint = 0xf1240001;
// IRQMASK2
pub const RSTV0910_IRQMASK2: c_uint = 0xf125;
pub const FSTV0910_MSPY_ENDSIM_1: c_uint = 0xf1257080;
pub const FSTV0910_MSPY_ENDSIM_2: c_uint = 0xf1256040;
pub const FSTV0910_MPKTDEL_ERROR_2: c_uint = 0xf1254010;
pub const FSTV0910_MPKTDEL_LOCKB_2: c_uint = 0xf1253008;
pub const FSTV0910_MPKTDEL_LOCK_2: c_uint = 0xf1252004;
pub const FSTV0910_MPKTDEL_ERROR_1: c_uint = 0xf1251002;
pub const FSTV0910_MPKTDEL_LOCKB_1: c_uint = 0xf1250001;
// IRQMASK1
pub const RSTV0910_IRQMASK1: c_uint = 0xf126;
pub const FSTV0910_MPKTDEL_LOCK_1: c_uint = 0xf1267080;
pub const FSTV0910_MFEC_LOCKB_2: c_uint = 0xf1266040;
pub const FSTV0910_MFEC_LOCK_2: c_uint = 0xf1265020;
pub const FSTV0910_MFEC_LOCKB_1: c_uint = 0xf1264010;
pub const FSTV0910_MFEC_LOCK_1: c_uint = 0xf1263008;
pub const FSTV0910_MDEMOD_LOCKB_2: c_uint = 0xf1262004;
pub const FSTV0910_MDEMOD_LOCK_2: c_uint = 0xf1261002;
pub const FSTV0910_MDEMOD_IRQ_2: c_uint = 0xf1260001;
// IRQMASK0
pub const RSTV0910_IRQMASK0: c_uint = 0xf127;
pub const FSTV0910_MDEMOD_LOCKB_1: c_uint = 0xf1277080;
pub const FSTV0910_MDEMOD_LOCK_1: c_uint = 0xf1276040;
pub const FSTV0910_MDEMOD_IRQ_1: c_uint = 0xf1275020;
pub const FSTV0910_MBCH_ERRFLAG: c_uint = 0xf1274010;
pub const FSTV0910_MDISEQC2_IRQ: c_uint = 0xf1272004;
pub const FSTV0910_MDISEQC1_IRQ: c_uint = 0xf1270001;
// I2CCFG
pub const RSTV0910_I2CCFG: c_uint = 0xf129;
pub const FSTV0910_I2C_FASTMODE: c_uint = 0xf1293008;
pub const FSTV0910_I2CADDR_INC: c_uint = 0xf1290003;
// P1_I2CRPT
pub const RSTV0910_P1_I2CRPT: c_uint = 0xf12a;
pub const FSTV0910_P1_I2CT_ON: c_uint = 0xf12a7080;
pub const FSTV0910_P1_ENARPT_LEVEL: c_uint = 0xf12a4070;
pub const FSTV0910_P1_SCLT_DELAY: c_uint = 0xf12a3008;
pub const FSTV0910_P1_STOP_ENABLE: c_uint = 0xf12a2004;
pub const FSTV0910_P1_STOP_SDAT2SDA: c_uint = 0xf12a1002;
// P2_I2CRPT
pub const RSTV0910_P2_I2CRPT: c_uint = 0xf12b;
pub const FSTV0910_P2_I2CT_ON: c_uint = 0xf12b7080;
pub const FSTV0910_P2_ENARPT_LEVEL: c_uint = 0xf12b4070;
pub const FSTV0910_P2_SCLT_DELAY: c_uint = 0xf12b3008;
pub const FSTV0910_P2_STOP_ENABLE: c_uint = 0xf12b2004;
pub const FSTV0910_P2_STOP_SDAT2SDA: c_uint = 0xf12b1002;
// GPIO0CFG
pub const RSTV0910_GPIO0CFG: c_uint = 0xf140;
pub const FSTV0910_GPIO0_OPD: c_uint = 0xf1407080;
pub const FSTV0910_GPIO0_CONFIG: c_uint = 0xf140107e;
pub const FSTV0910_GPIO0_XOR: c_uint = 0xf1400001;
// GPIO1CFG
pub const RSTV0910_GPIO1CFG: c_uint = 0xf141;
pub const FSTV0910_GPIO1_OPD: c_uint = 0xf1417080;
pub const FSTV0910_GPIO1_CONFIG: c_uint = 0xf141107e;
pub const FSTV0910_GPIO1_XOR: c_uint = 0xf1410001;
// GPIO2CFG
pub const RSTV0910_GPIO2CFG: c_uint = 0xf142;
pub const FSTV0910_GPIO2_OPD: c_uint = 0xf1427080;
pub const FSTV0910_GPIO2_CONFIG: c_uint = 0xf142107e;
pub const FSTV0910_GPIO2_XOR: c_uint = 0xf1420001;
// GPIO3CFG
pub const RSTV0910_GPIO3CFG: c_uint = 0xf143;
pub const FSTV0910_GPIO3_OPD: c_uint = 0xf1437080;
pub const FSTV0910_GPIO3_CONFIG: c_uint = 0xf143107e;
pub const FSTV0910_GPIO3_XOR: c_uint = 0xf1430001;
// GPIO4CFG
pub const RSTV0910_GPIO4CFG: c_uint = 0xf144;
pub const FSTV0910_GPIO4_OPD: c_uint = 0xf1447080;
pub const FSTV0910_GPIO4_CONFIG: c_uint = 0xf144107e;
pub const FSTV0910_GPIO4_XOR: c_uint = 0xf1440001;
// GPIO5CFG
pub const RSTV0910_GPIO5CFG: c_uint = 0xf145;
pub const FSTV0910_GPIO5_OPD: c_uint = 0xf1457080;
pub const FSTV0910_GPIO5_CONFIG: c_uint = 0xf145107e;
pub const FSTV0910_GPIO5_XOR: c_uint = 0xf1450001;
// GPIO6CFG
pub const RSTV0910_GPIO6CFG: c_uint = 0xf146;
pub const FSTV0910_GPIO6_OPD: c_uint = 0xf1467080;
pub const FSTV0910_GPIO6_CONFIG: c_uint = 0xf146107e;
pub const FSTV0910_GPIO6_XOR: c_uint = 0xf1460001;
// GPIO7CFG
pub const RSTV0910_GPIO7CFG: c_uint = 0xf147;
pub const FSTV0910_GPIO7_OPD: c_uint = 0xf1477080;
pub const FSTV0910_GPIO7_CONFIG: c_uint = 0xf147107e;
pub const FSTV0910_GPIO7_XOR: c_uint = 0xf1470001;
// GPIO8CFG
pub const RSTV0910_GPIO8CFG: c_uint = 0xf148;
pub const FSTV0910_GPIO8_OPD: c_uint = 0xf1487080;
pub const FSTV0910_GPIO8_CONFIG: c_uint = 0xf148107e;
pub const FSTV0910_GPIO8_XOR: c_uint = 0xf1480001;
// GPIO9CFG
pub const RSTV0910_GPIO9CFG: c_uint = 0xf149;
pub const FSTV0910_GPIO9_OPD: c_uint = 0xf1497080;
pub const FSTV0910_GPIO9_CONFIG: c_uint = 0xf149107e;
pub const FSTV0910_GPIO9_XOR: c_uint = 0xf1490001;
// GPIO10CFG
pub const RSTV0910_GPIO10CFG: c_uint = 0xf14a;
pub const FSTV0910_GPIO10_OPD: c_uint = 0xf14a7080;
pub const FSTV0910_GPIO10_CONFIG: c_uint = 0xf14a107e;
pub const FSTV0910_GPIO10_XOR: c_uint = 0xf14a0001;
// GPIO11CFG
pub const RSTV0910_GPIO11CFG: c_uint = 0xf14b;
pub const FSTV0910_GPIO11_OPD: c_uint = 0xf14b7080;
pub const FSTV0910_GPIO11_CONFIG: c_uint = 0xf14b107e;
pub const FSTV0910_GPIO11_XOR: c_uint = 0xf14b0001;
// GPIO12CFG
pub const RSTV0910_GPIO12CFG: c_uint = 0xf14c;
pub const FSTV0910_GPIO12_OPD: c_uint = 0xf14c7080;
pub const FSTV0910_GPIO12_CONFIG: c_uint = 0xf14c107e;
pub const FSTV0910_GPIO12_XOR: c_uint = 0xf14c0001;
// GPIO13CFG
pub const RSTV0910_GPIO13CFG: c_uint = 0xf14d;
pub const FSTV0910_GPIO13_OPD: c_uint = 0xf14d7080;
pub const FSTV0910_GPIO13_CONFIG: c_uint = 0xf14d107e;
pub const FSTV0910_GPIO13_XOR: c_uint = 0xf14d0001;
// GPIO14CFG
pub const RSTV0910_GPIO14CFG: c_uint = 0xf14e;
pub const FSTV0910_GPIO14_OPD: c_uint = 0xf14e7080;
pub const FSTV0910_GPIO14_CONFIG: c_uint = 0xf14e107e;
pub const FSTV0910_GPIO14_XOR: c_uint = 0xf14e0001;
// GPIO15CFG
pub const RSTV0910_GPIO15CFG: c_uint = 0xf14f;
pub const FSTV0910_GPIO15_OPD: c_uint = 0xf14f7080;
pub const FSTV0910_GPIO15_CONFIG: c_uint = 0xf14f107e;
pub const FSTV0910_GPIO15_XOR: c_uint = 0xf14f0001;
// GPIO16CFG
pub const RSTV0910_GPIO16CFG: c_uint = 0xf150;
pub const FSTV0910_GPIO16_OPD: c_uint = 0xf1507080;
pub const FSTV0910_GPIO16_CONFIG: c_uint = 0xf150107e;
pub const FSTV0910_GPIO16_XOR: c_uint = 0xf1500001;
// GPIO17CFG
pub const RSTV0910_GPIO17CFG: c_uint = 0xf151;
pub const FSTV0910_GPIO17_OPD: c_uint = 0xf1517080;
pub const FSTV0910_GPIO17_CONFIG: c_uint = 0xf151107e;
pub const FSTV0910_GPIO17_XOR: c_uint = 0xf1510001;
// GPIO18CFG
pub const RSTV0910_GPIO18CFG: c_uint = 0xf152;
pub const FSTV0910_GPIO18_OPD: c_uint = 0xf1527080;
pub const FSTV0910_GPIO18_CONFIG: c_uint = 0xf152107e;
pub const FSTV0910_GPIO18_XOR: c_uint = 0xf1520001;
// GPIO19CFG
pub const RSTV0910_GPIO19CFG: c_uint = 0xf153;
pub const FSTV0910_GPIO19_OPD: c_uint = 0xf1537080;
pub const FSTV0910_GPIO19_CONFIG: c_uint = 0xf153107e;
pub const FSTV0910_GPIO19_XOR: c_uint = 0xf1530001;
// GPIO20CFG
pub const RSTV0910_GPIO20CFG: c_uint = 0xf154;
pub const FSTV0910_GPIO20_OPD: c_uint = 0xf1547080;
pub const FSTV0910_GPIO20_CONFIG: c_uint = 0xf154107e;
pub const FSTV0910_GPIO20_XOR: c_uint = 0xf1540001;
// GPIO21CFG
pub const RSTV0910_GPIO21CFG: c_uint = 0xf155;
pub const FSTV0910_GPIO21_OPD: c_uint = 0xf1557080;
pub const FSTV0910_GPIO21_CONFIG: c_uint = 0xf155107e;
pub const FSTV0910_GPIO21_XOR: c_uint = 0xf1550001;
// GPIO22CFG
pub const RSTV0910_GPIO22CFG: c_uint = 0xf156;
pub const FSTV0910_GPIO22_OPD: c_uint = 0xf1567080;
pub const FSTV0910_GPIO22_CONFIG: c_uint = 0xf156107e;
pub const FSTV0910_GPIO22_XOR: c_uint = 0xf1560001;
// STRSTATUS1
pub const RSTV0910_STRSTATUS1: c_uint = 0xf16a;
pub const FSTV0910_STRSTATUS_SEL2: c_uint = 0xf16a40f0;
pub const FSTV0910_STRSTATUS_SEL1: c_uint = 0xf16a000f;
// STRSTATUS2
pub const RSTV0910_STRSTATUS2: c_uint = 0xf16b;
pub const FSTV0910_STRSTATUS_SEL4: c_uint = 0xf16b40f0;
pub const FSTV0910_STRSTATUS_SEL3: c_uint = 0xf16b000f;
// STRSTATUS3
pub const RSTV0910_STRSTATUS3: c_uint = 0xf16c;
pub const FSTV0910_STRSTATUS_SEL6: c_uint = 0xf16c40f0;
pub const FSTV0910_STRSTATUS_SEL5: c_uint = 0xf16c000f;
// FSKTFC2
pub const RSTV0910_FSKTFC2: c_uint = 0xf170;
pub const FSTV0910_FSKT_KMOD: c_uint = 0xf17020fc;
pub const FSTV0910_FSKT_CAR2: c_uint = 0xf1700003;
// FSKTFC1
pub const RSTV0910_FSKTFC1: c_uint = 0xf171;
pub const FSTV0910_FSKT_CAR1: c_uint = 0xf17100ff;
// FSKTFC0
pub const RSTV0910_FSKTFC0: c_uint = 0xf172;
pub const FSTV0910_FSKT_CAR0: c_uint = 0xf17200ff;
// FSKTDELTAF1
pub const RSTV0910_FSKTDELTAF1: c_uint = 0xf173;
pub const FSTV0910_FSKT_DELTAF1: c_uint = 0xf173000f;
// FSKTDELTAF0
pub const RSTV0910_FSKTDELTAF0: c_uint = 0xf174;
pub const FSTV0910_FSKT_DELTAF0: c_uint = 0xf17400ff;
// FSKTCTRL
pub const RSTV0910_FSKTCTRL: c_uint = 0xf175;
pub const FSTV0910_FSKT_PINSEL: c_uint = 0xf1757080;
pub const FSTV0910_FSKT_EN_SGN: c_uint = 0xf1756040;
pub const FSTV0910_FSKT_MOD_SGN: c_uint = 0xf1755020;
pub const FSTV0910_FSKT_MOD_EN: c_uint = 0xf175201c;
pub const FSTV0910_FSKT_DACMODE: c_uint = 0xf1750003;
// FSKRFC2
pub const RSTV0910_FSKRFC2: c_uint = 0xf176;
pub const FSTV0910_FSKR_DETSGN: c_uint = 0xf1766040;
pub const FSTV0910_FSKR_OUTSGN: c_uint = 0xf1765020;
pub const FSTV0910_FSKR_KAGC: c_uint = 0xf176201c;
pub const FSTV0910_FSKR_CAR2: c_uint = 0xf1760003;
// FSKRFC1
pub const RSTV0910_FSKRFC1: c_uint = 0xf177;
pub const FSTV0910_FSKR_CAR1: c_uint = 0xf17700ff;
// FSKRFC0
pub const RSTV0910_FSKRFC0: c_uint = 0xf178;
pub const FSTV0910_FSKR_CAR0: c_uint = 0xf17800ff;
// FSKRK1
pub const RSTV0910_FSKRK1: c_uint = 0xf179;
pub const FSTV0910_FSKR_K1_EXP: c_uint = 0xf17950e0;
pub const FSTV0910_FSKR_K1_MANT: c_uint = 0xf179001f;
// FSKRK2
pub const RSTV0910_FSKRK2: c_uint = 0xf17a;
pub const FSTV0910_FSKR_K2_EXP: c_uint = 0xf17a50e0;
pub const FSTV0910_FSKR_K2_MANT: c_uint = 0xf17a001f;
// FSKRAGCR
pub const RSTV0910_FSKRAGCR: c_uint = 0xf17b;
pub const FSTV0910_FSKR_OUTCTL: c_uint = 0xf17b60c0;
pub const FSTV0910_FSKR_AGC_REF: c_uint = 0xf17b003f;
// FSKRAGC
pub const RSTV0910_FSKRAGC: c_uint = 0xf17c;
pub const FSTV0910_FSKR_AGC_ACCU: c_uint = 0xf17c00ff;
// FSKRALPHA
pub const RSTV0910_FSKRALPHA: c_uint = 0xf17d;
pub const FSTV0910_FSKR_ALPHA_EXP: c_uint = 0xf17d201c;
pub const FSTV0910_FSKR_ALPHA_M: c_uint = 0xf17d0003;
// FSKRPLTH1
pub const RSTV0910_FSKRPLTH1: c_uint = 0xf17e;
pub const FSTV0910_FSKR_BETA: c_uint = 0xf17e40f0;
pub const FSTV0910_FSKR_PLL_TRESH1: c_uint = 0xf17e000f;
// FSKRPLTH0
pub const RSTV0910_FSKRPLTH0: c_uint = 0xf17f;
pub const FSTV0910_FSKR_PLL_TRESH0: c_uint = 0xf17f00ff;
// FSKRDF1
pub const RSTV0910_FSKRDF1: c_uint = 0xf180;
pub const FSTV0910_FSKR_OUT: c_uint = 0xf1807080;
pub const FSTV0910_FSKR_STATE: c_uint = 0xf1805060;
pub const FSTV0910_FSKR_DELTAF1: c_uint = 0xf180001f;
// FSKRDF0
pub const RSTV0910_FSKRDF0: c_uint = 0xf181;
pub const FSTV0910_FSKR_DELTAF0: c_uint = 0xf18100ff;
// FSKRSTEPP
pub const RSTV0910_FSKRSTEPP: c_uint = 0xf182;
pub const FSTV0910_FSKR_STEP_PLUS: c_uint = 0xf18200ff;
// FSKRSTEPM
pub const RSTV0910_FSKRSTEPM: c_uint = 0xf183;
pub const FSTV0910_FSKR_STEP_MINUS: c_uint = 0xf18300ff;
// FSKRDET1
pub const RSTV0910_FSKRDET1: c_uint = 0xf184;
pub const FSTV0910_FSKR_DETECT: c_uint = 0xf1847080;
pub const FSTV0910_FSKR_CARDET_ACCU1: c_uint = 0xf184000f;
// FSKRDET0
pub const RSTV0910_FSKRDET0: c_uint = 0xf185;
pub const FSTV0910_FSKR_CARDET_ACCU0: c_uint = 0xf18500ff;
// FSKRDTH1
pub const RSTV0910_FSKRDTH1: c_uint = 0xf186;
pub const FSTV0910_FSKR_CARLOSS_THRESH1: c_uint = 0xf18640f0;
pub const FSTV0910_FSKR_CARDET_THRESH1: c_uint = 0xf186000f;
// FSKRDTH0
pub const RSTV0910_FSKRDTH0: c_uint = 0xf187;
pub const FSTV0910_FSKR_CARDET_THRESH0: c_uint = 0xf18700ff;
// FSKRLOSS
pub const RSTV0910_FSKRLOSS: c_uint = 0xf188;
pub const FSTV0910_FSKR_CARLOSS_THRESH0: c_uint = 0xf18800ff;
// NCOARSE
pub const RSTV0910_NCOARSE: c_uint = 0xf1b3;
pub const FSTV0910_CP: c_uint = 0xf1b330f8;
pub const FSTV0910_IDF: c_uint = 0xf1b30007;
// NCOARSE1
pub const RSTV0910_NCOARSE1: c_uint = 0xf1b4;
pub const FSTV0910_N_DIV: c_uint = 0xf1b400ff;
// NCOARSE2
pub const RSTV0910_NCOARSE2: c_uint = 0xf1b5;
pub const FSTV0910_ODF: c_uint = 0xf1b5003f;
// SYNTCTRL
pub const RSTV0910_SYNTCTRL: c_uint = 0xf1b6;
pub const FSTV0910_STANDBY: c_uint = 0xf1b67080;
pub const FSTV0910_BYPASSPLLCORE: c_uint = 0xf1b66040;
pub const FSTV0910_STOP_PLL: c_uint = 0xf1b63008;
pub const FSTV0910_OSCI_E: c_uint = 0xf1b61002;
// FILTCTRL
pub const RSTV0910_FILTCTRL: c_uint = 0xf1b7;
pub const FSTV0910_INV_CLKFSK: c_uint = 0xf1b71002;
pub const FSTV0910_BYPASS_APPLI: c_uint = 0xf1b70001;
// PLLSTAT
pub const RSTV0910_PLLSTAT: c_uint = 0xf1b8;
pub const FSTV0910_PLLLOCK: c_uint = 0xf1b80001;
// STOPCLK1
pub const RSTV0910_STOPCLK1: c_uint = 0xf1c2;
pub const FSTV0910_INV_CLKADCI2: c_uint = 0xf1c22004;
pub const FSTV0910_INV_CLKADCI1: c_uint = 0xf1c20001;
// STOPCLK2
pub const RSTV0910_STOPCLK2: c_uint = 0xf1c3;
pub const FSTV0910_STOP_DVBS2FEC2: c_uint = 0xf1c35020;
pub const FSTV0910_STOP_DVBS2FEC: c_uint = 0xf1c34010;
pub const FSTV0910_STOP_DVBS1FEC2: c_uint = 0xf1c33008;
pub const FSTV0910_STOP_DVBS1FEC: c_uint = 0xf1c32004;
pub const FSTV0910_STOP_DEMOD2: c_uint = 0xf1c31002;
pub const FSTV0910_STOP_DEMOD: c_uint = 0xf1c30001;
// PREGCTL
pub const RSTV0910_PREGCTL: c_uint = 0xf1c8;
pub const FSTV0910_REG3V3TO2V5_POFF: c_uint = 0xf1c87080;
// TSTTNR0
pub const RSTV0910_TSTTNR0: c_uint = 0xf1df;
pub const FSTV0910_FSK_PON: c_uint = 0xf1df2004;
// TSTTNR1
pub const RSTV0910_TSTTNR1: c_uint = 0xf1e0;
pub const FSTV0910_ADC1_PON: c_uint = 0xf1e01002;
// TSTTNR2
pub const RSTV0910_TSTTNR2: c_uint = 0xf1e1;
pub const FSTV0910_I2C_DISEQC_PON: c_uint = 0xf1e15020;
pub const FSTV0910_DISEQC_CLKDIV: c_uint = 0xf1e1000f;
// TSTTNR3
pub const RSTV0910_TSTTNR3: c_uint = 0xf1e2;
pub const FSTV0910_ADC2_PON: c_uint = 0xf1e21002;
// P2_IQCONST
pub const RSTV0910_P2_IQCONST: c_uint = 0xf200;
pub const FSTV0910_P2_CONSTEL_SELECT: c_uint = 0xf2005060;
pub const FSTV0910_P2_IQSYMB_SEL: c_uint = 0xf200001f;
// P2_NOSCFG
pub const RSTV0910_P2_NOSCFG: c_uint = 0xf201;
pub const FSTV0910_P2_DUMMYPL_NOSDATA: c_uint = 0xf2015020;
pub const FSTV0910_P2_NOSPLH_BETA: c_uint = 0xf2013018;
pub const FSTV0910_P2_NOSDATA_BETA: c_uint = 0xf2010007;
// P2_ISYMB
pub const RSTV0910_P2_ISYMB: c_uint = 0xf202;
pub const FSTV0910_P2_I_SYMBOL: c_uint = 0xf20201ff;
// P2_QSYMB
pub const RSTV0910_P2_QSYMB: c_uint = 0xf203;
pub const FSTV0910_P2_Q_SYMBOL: c_uint = 0xf20301ff;
// P2_AGC1CFG
pub const RSTV0910_P2_AGC1CFG: c_uint = 0xf204;
pub const FSTV0910_P2_DC_FROZEN: c_uint = 0xf2047080;
pub const FSTV0910_P2_DC_CORRECT: c_uint = 0xf2046040;
pub const FSTV0910_P2_AMM_FROZEN: c_uint = 0xf2045020;
pub const FSTV0910_P2_AMM_CORRECT: c_uint = 0xf2044010;
pub const FSTV0910_P2_QUAD_FROZEN: c_uint = 0xf2043008;
pub const FSTV0910_P2_QUAD_CORRECT: c_uint = 0xf2042004;
// P2_AGC1CN
pub const RSTV0910_P2_AGC1CN: c_uint = 0xf206;
pub const FSTV0910_P2_AGC1_LOCKED: c_uint = 0xf2067080;
pub const FSTV0910_P2_AGC1_MINPOWER: c_uint = 0xf2064010;
pub const FSTV0910_P2_AGCOUT_FAST: c_uint = 0xf2063008;
pub const FSTV0910_P2_AGCIQ_BETA: c_uint = 0xf2060007;
// P2_AGC1REF
pub const RSTV0910_P2_AGC1REF: c_uint = 0xf207;
pub const FSTV0910_P2_AGCIQ_REF: c_uint = 0xf20700ff;
// P2_IDCCOMP
pub const RSTV0910_P2_IDCCOMP: c_uint = 0xf208;
pub const FSTV0910_P2_IAVERAGE_ADJ: c_uint = 0xf20801ff;
// P2_QDCCOMP
pub const RSTV0910_P2_QDCCOMP: c_uint = 0xf209;
pub const FSTV0910_P2_QAVERAGE_ADJ: c_uint = 0xf20901ff;
// P2_POWERI
pub const RSTV0910_P2_POWERI: c_uint = 0xf20a;
pub const FSTV0910_P2_POWER_I: c_uint = 0xf20a00ff;
// P2_POWERQ
pub const RSTV0910_P2_POWERQ: c_uint = 0xf20b;
pub const FSTV0910_P2_POWER_Q: c_uint = 0xf20b00ff;
// P2_AGC1AMM
pub const RSTV0910_P2_AGC1AMM: c_uint = 0xf20c;
pub const FSTV0910_P2_AMM_VALUE: c_uint = 0xf20c00ff;
// P2_AGC1QUAD
pub const RSTV0910_P2_AGC1QUAD: c_uint = 0xf20d;
pub const FSTV0910_P2_QUAD_VALUE: c_uint = 0xf20d01ff;
// P2_AGCIQIN1
pub const RSTV0910_P2_AGCIQIN1: c_uint = 0xf20e;
pub const FSTV0910_P2_AGCIQ_VALUE1: c_uint = 0xf20e00ff;
// P2_AGCIQIN0
pub const RSTV0910_P2_AGCIQIN0: c_uint = 0xf20f;
pub const FSTV0910_P2_AGCIQ_VALUE0: c_uint = 0xf20f00ff;
// P2_DEMOD
pub const RSTV0910_P2_DEMOD: c_uint = 0xf210;
pub const FSTV0910_P2_MANUALS2_ROLLOFF: c_uint = 0xf2107080;
pub const FSTV0910_P2_SPECINV_CONTROL: c_uint = 0xf2104030;
pub const FSTV0910_P2_MANUALSX_ROLLOFF: c_uint = 0xf2102004;
pub const FSTV0910_P2_ROLLOFF_CONTROL: c_uint = 0xf2100003;
// P2_DMDMODCOD
pub const RSTV0910_P2_DMDMODCOD: c_uint = 0xf211;
pub const FSTV0910_P2_MANUAL_MODCOD: c_uint = 0xf2117080;
pub const FSTV0910_P2_DEMOD_MODCOD: c_uint = 0xf211207c;
pub const FSTV0910_P2_DEMOD_TYPE: c_uint = 0xf2110003;
// P2_DSTATUS
pub const RSTV0910_P2_DSTATUS: c_uint = 0xf212;
pub const FSTV0910_P2_CAR_LOCK: c_uint = 0xf2127080;
pub const FSTV0910_P2_TMGLOCK_QUALITY: c_uint = 0xf2125060;
pub const FSTV0910_P2_LOCK_DEFINITIF: c_uint = 0xf2123008;
pub const FSTV0910_P2_OVADC_DETECT: c_uint = 0xf2120001;
// P2_DSTATUS2
pub const RSTV0910_P2_DSTATUS2: c_uint = 0xf213;
pub const FSTV0910_P2_DEMOD_DELOCK: c_uint = 0xf2137080;
pub const FSTV0910_P2_MODCODRQ_SYNCTAG: c_uint = 0xf2135020;
pub const FSTV0910_P2_POLYPH_SATEVENT: c_uint = 0xf2134010;
pub const FSTV0910_P2_AGC1_NOSIGNALACK: c_uint = 0xf2133008;
pub const FSTV0910_P2_AGC2_OVERFLOW: c_uint = 0xf2132004;
pub const FSTV0910_P2_CFR_OVERFLOW: c_uint = 0xf2131002;
pub const FSTV0910_P2_GAMMA_OVERUNDER: c_uint = 0xf2130001;
// P2_DMDCFGMD
pub const RSTV0910_P2_DMDCFGMD: c_uint = 0xf214;
pub const FSTV0910_P2_DVBS2_ENABLE: c_uint = 0xf2147080;
pub const FSTV0910_P2_DVBS1_ENABLE: c_uint = 0xf2146040;
pub const FSTV0910_P2_SCAN_ENABLE: c_uint = 0xf2144010;
pub const FSTV0910_P2_CFR_AUTOSCAN: c_uint = 0xf2143008;
pub const FSTV0910_P2_TUN_RNG: c_uint = 0xf2140003;
// P2_DMDCFG2
pub const RSTV0910_P2_DMDCFG2: c_uint = 0xf215;
pub const FSTV0910_P2_S1S2_SEQUENTIAL: c_uint = 0xf2156040;
pub const FSTV0910_P2_INFINITE_RELOCK: c_uint = 0xf2154010;
// P2_DMDISTATE
pub const RSTV0910_P2_DMDISTATE: c_uint = 0xf216;
pub const FSTV0910_P2_I2C_NORESETDMODE: c_uint = 0xf2167080;
pub const FSTV0910_P2_I2C_DEMOD_MODE: c_uint = 0xf216001f;
// P2_DMDT0M
pub const RSTV0910_P2_DMDT0M: c_uint = 0xf217;
pub const FSTV0910_P2_DMDT0_MIN: c_uint = 0xf21700ff;
// P2_DMDSTATE
pub const RSTV0910_P2_DMDSTATE: c_uint = 0xf21b;
pub const FSTV0910_P2_HEADER_MODE: c_uint = 0xf21b5060;
// P2_DMDFLYW
pub const RSTV0910_P2_DMDFLYW: c_uint = 0xf21c;
pub const FSTV0910_P2_I2C_IRQVAL: c_uint = 0xf21c40f0;
pub const FSTV0910_P2_FLYWHEEL_CPT: c_uint = 0xf21c000f;
// P2_DSTATUS3
pub const RSTV0910_P2_DSTATUS3: c_uint = 0xf21d;
pub const FSTV0910_P2_CFR_ZIGZAG: c_uint = 0xf21d7080;
pub const FSTV0910_P2_DEMOD_CFGMODE: c_uint = 0xf21d5060;
pub const FSTV0910_P2_GAMMA_LOWBAUDRATE: c_uint = 0xf21d4010;
// P2_DMDCFG3
pub const RSTV0910_P2_DMDCFG3: c_uint = 0xf21e;
pub const FSTV0910_P2_NOSTOP_FIFOFULL: c_uint = 0xf21e3008;
// P2_DMDCFG4
pub const RSTV0910_P2_DMDCFG4: c_uint = 0xf21f;
pub const FSTV0910_P2_DIS_VITLOCK: c_uint = 0xf21f7080;
pub const FSTV0910_P2_DIS_CLKENABLE: c_uint = 0xf21f2004;
// P2_CORRELMANT
pub const RSTV0910_P2_CORRELMANT: c_uint = 0xf220;
pub const FSTV0910_P2_CORREL_MANT: c_uint = 0xf22000ff;
// P2_CORRELABS
pub const RSTV0910_P2_CORRELABS: c_uint = 0xf221;
pub const FSTV0910_P2_CORREL_ABS: c_uint = 0xf22100ff;
// P2_CORRELEXP
pub const RSTV0910_P2_CORRELEXP: c_uint = 0xf222;
pub const FSTV0910_P2_CORREL_ABSEXP: c_uint = 0xf22240f0;
pub const FSTV0910_P2_CORREL_EXP: c_uint = 0xf222000f;
// P2_PLHMODCOD
pub const RSTV0910_P2_PLHMODCOD: c_uint = 0xf224;
pub const FSTV0910_P2_SPECINV_DEMOD: c_uint = 0xf2247080;
pub const FSTV0910_P2_PLH_MODCOD: c_uint = 0xf224207c;
pub const FSTV0910_P2_PLH_TYPE: c_uint = 0xf2240003;
// P2_DMDREG
pub const RSTV0910_P2_DMDREG: c_uint = 0xf225;
pub const FSTV0910_P2_DECIM_PLFRAMES: c_uint = 0xf2250001;
// P2_AGCNADJ
pub const RSTV0910_P2_AGCNADJ: c_uint = 0xf226;
pub const FSTV0910_P2_RADJOFF_AGC2: c_uint = 0xf2267080;
pub const FSTV0910_P2_RADJOFF_AGC1: c_uint = 0xf2266040;
pub const FSTV0910_P2_AGC_NADJ: c_uint = 0xf226013f;
// P2_AGCKS
pub const RSTV0910_P2_AGCKS: c_uint = 0xf227;
pub const FSTV0910_P2_RSADJ_MANUALCFG: c_uint = 0xf2277080;
pub const FSTV0910_P2_RSADJ_CCMMODE: c_uint = 0xf2276040;
pub const FSTV0910_P2_RADJ_SPSK: c_uint = 0xf227013f;
// P2_AGCKQ
pub const RSTV0910_P2_AGCKQ: c_uint = 0xf228;
pub const FSTV0910_P2_RADJON_DVBS1: c_uint = 0xf2286040;
pub const FSTV0910_P2_RADJ_QPSK: c_uint = 0xf228013f;
// P2_AGCK8
pub const RSTV0910_P2_AGCK8: c_uint = 0xf229;
pub const FSTV0910_P2_RADJ_8PSK: c_uint = 0xf229013f;
// P2_AGCK16
pub const RSTV0910_P2_AGCK16: c_uint = 0xf22a;
pub const FSTV0910_P2_R2ADJOFF_16APSK: c_uint = 0xf22a6040;
pub const FSTV0910_P2_R1ADJOFF_16APSK: c_uint = 0xf22a5020;
pub const FSTV0910_P2_RADJ_16APSK: c_uint = 0xf22a011f;
// P2_AGCK32
pub const RSTV0910_P2_AGCK32: c_uint = 0xf22b;
pub const FSTV0910_P2_R3ADJOFF_32APSK: c_uint = 0xf22b7080;
pub const FSTV0910_P2_R2ADJOFF_32APSK: c_uint = 0xf22b6040;
pub const FSTV0910_P2_R1ADJOFF_32APSK: c_uint = 0xf22b5020;
pub const FSTV0910_P2_RADJ_32APSK: c_uint = 0xf22b011f;
// P2_AGC2O
pub const RSTV0910_P2_AGC2O: c_uint = 0xf22c;
pub const FSTV0910_P2_CSTENV_MODE: c_uint = 0xf22c60c0;
pub const FSTV0910_P2_AGC2_COEF: c_uint = 0xf22c0007;
// P2_AGC2REF
pub const RSTV0910_P2_AGC2REF: c_uint = 0xf22d;
pub const FSTV0910_P2_AGC2_REF: c_uint = 0xf22d00ff;
// P2_AGC1ADJ
pub const RSTV0910_P2_AGC1ADJ: c_uint = 0xf22e;
pub const FSTV0910_P2_AGC1_ADJUSTED: c_uint = 0xf22e007f;
// P2_AGCRSADJ
pub const RSTV0910_P2_AGCRSADJ: c_uint = 0xf22f;
pub const FSTV0910_P2_RS_ADJUSTED: c_uint = 0xf22f007f;
// P2_AGCRQADJ
pub const RSTV0910_P2_AGCRQADJ: c_uint = 0xf230;
pub const FSTV0910_P2_RQ_ADJUSTED: c_uint = 0xf230007f;
// P2_AGCR8ADJ
pub const RSTV0910_P2_AGCR8ADJ: c_uint = 0xf231;
pub const FSTV0910_P2_R8_ADJUSTED: c_uint = 0xf231007f;
// P2_AGCR1ADJ
pub const RSTV0910_P2_AGCR1ADJ: c_uint = 0xf232;
pub const FSTV0910_P2_R1_ADJUSTED: c_uint = 0xf232007f;
// P2_AGCR2ADJ
pub const RSTV0910_P2_AGCR2ADJ: c_uint = 0xf233;
pub const FSTV0910_P2_R2_ADJUSTED: c_uint = 0xf233007f;
// P2_AGCR3ADJ
pub const RSTV0910_P2_AGCR3ADJ: c_uint = 0xf234;
pub const FSTV0910_P2_R3_ADJUSTED: c_uint = 0xf234007f;
// P2_AGCREFADJ
pub const RSTV0910_P2_AGCREFADJ: c_uint = 0xf235;
pub const FSTV0910_P2_AGC2REF_ADJUSTED: c_uint = 0xf235007f;
// P2_AGC2I1
pub const RSTV0910_P2_AGC2I1: c_uint = 0xf236;
pub const FSTV0910_P2_AGC2_INTEGRATOR1: c_uint = 0xf23600ff;
// P2_AGC2I0
pub const RSTV0910_P2_AGC2I0: c_uint = 0xf237;
pub const FSTV0910_P2_AGC2_INTEGRATOR0: c_uint = 0xf23700ff;
// P2_CARCFG
pub const RSTV0910_P2_CARCFG: c_uint = 0xf238;
pub const FSTV0910_P2_ROTAON: c_uint = 0xf2382004;
pub const FSTV0910_P2_PH_DET_ALGO: c_uint = 0xf2380003;
// P2_ACLC
pub const RSTV0910_P2_ACLC: c_uint = 0xf239;
pub const FSTV0910_P2_CAR_ALPHA_MANT: c_uint = 0xf2394030;
pub const FSTV0910_P2_CAR_ALPHA_EXP: c_uint = 0xf239000f;
// P2_BCLC
pub const RSTV0910_P2_BCLC: c_uint = 0xf23a;
pub const FSTV0910_P2_CAR_BETA_MANT: c_uint = 0xf23a4030;
pub const FSTV0910_P2_CAR_BETA_EXP: c_uint = 0xf23a000f;
// P2_ACLCS2
pub const RSTV0910_P2_ACLCS2: c_uint = 0xf23b;
pub const FSTV0910_P2_CARS2_APLHA_MANTISSE: c_uint = 0xf23b4030;
pub const FSTV0910_P2_CARS2_ALPHA_EXP: c_uint = 0xf23b000f;
// P2_BCLCS2
pub const RSTV0910_P2_BCLCS2: c_uint = 0xf23c;
pub const FSTV0910_P2_CARS2_BETA_MANTISSE: c_uint = 0xf23c4030;
pub const FSTV0910_P2_CARS2_BETA_EXP: c_uint = 0xf23c000f;
// P2_CARFREQ
pub const RSTV0910_P2_CARFREQ: c_uint = 0xf23d;
pub const FSTV0910_P2_KC_COARSE_EXP: c_uint = 0xf23d40f0;
pub const FSTV0910_P2_BETA_FREQ: c_uint = 0xf23d000f;
// P2_CARHDR
pub const RSTV0910_P2_CARHDR: c_uint = 0xf23e;
pub const FSTV0910_P2_K_FREQ_HDR: c_uint = 0xf23e00ff;
// P2_LDT
pub const RSTV0910_P2_LDT: c_uint = 0xf23f;
pub const FSTV0910_P2_CARLOCK_THRES: c_uint = 0xf23f01ff;
// P2_LDT2
pub const RSTV0910_P2_LDT2: c_uint = 0xf240;
pub const FSTV0910_P2_CARLOCK_THRES2: c_uint = 0xf24001ff;
// P2_CFRICFG
pub const RSTV0910_P2_CFRICFG: c_uint = 0xf241;
pub const FSTV0910_P2_NEG_CFRSTEP: c_uint = 0xf2410001;
// P2_CFRUP1
pub const RSTV0910_P2_CFRUP1: c_uint = 0xf242;
pub const FSTV0910_P2_CFR_UP1: c_uint = 0xf24201ff;
// P2_CFRUP0
pub const RSTV0910_P2_CFRUP0: c_uint = 0xf243;
pub const FSTV0910_P2_CFR_UP0: c_uint = 0xf24300ff;
// P2_CFRIBASE1
pub const RSTV0910_P2_CFRIBASE1: c_uint = 0xf244;
pub const FSTV0910_P2_CFRINIT_BASE1: c_uint = 0xf24400ff;
// P2_CFRIBASE0
pub const RSTV0910_P2_CFRIBASE0: c_uint = 0xf245;
pub const FSTV0910_P2_CFRINIT_BASE0: c_uint = 0xf24500ff;
// P2_CFRLOW1
pub const RSTV0910_P2_CFRLOW1: c_uint = 0xf246;
pub const FSTV0910_P2_CFR_LOW1: c_uint = 0xf24601ff;
// P2_CFRLOW0
pub const RSTV0910_P2_CFRLOW0: c_uint = 0xf247;
pub const FSTV0910_P2_CFR_LOW0: c_uint = 0xf24700ff;
// P2_CFRINIT1
pub const RSTV0910_P2_CFRINIT1: c_uint = 0xf248;
pub const FSTV0910_P2_CFR_INIT1: c_uint = 0xf24801ff;
// P2_CFRINIT0
pub const RSTV0910_P2_CFRINIT0: c_uint = 0xf249;
pub const FSTV0910_P2_CFR_INIT0: c_uint = 0xf24900ff;
// P2_CFRINC1
pub const RSTV0910_P2_CFRINC1: c_uint = 0xf24a;
pub const FSTV0910_P2_MANUAL_CFRINC: c_uint = 0xf24a7080;
pub const FSTV0910_P2_CFR_INC1: c_uint = 0xf24a003f;
// P2_CFRINC0
pub const RSTV0910_P2_CFRINC0: c_uint = 0xf24b;
pub const FSTV0910_P2_CFR_INC0: c_uint = 0xf24b00ff;
// P2_CFR2
pub const RSTV0910_P2_CFR2: c_uint = 0xf24c;
pub const FSTV0910_P2_CAR_FREQ2: c_uint = 0xf24c01ff;
// P2_CFR1
pub const RSTV0910_P2_CFR1: c_uint = 0xf24d;
pub const FSTV0910_P2_CAR_FREQ1: c_uint = 0xf24d00ff;
// P2_CFR0
pub const RSTV0910_P2_CFR0: c_uint = 0xf24e;
pub const FSTV0910_P2_CAR_FREQ0: c_uint = 0xf24e00ff;
// P2_LDI
pub const RSTV0910_P2_LDI: c_uint = 0xf24f;
pub const FSTV0910_P2_LOCK_DET_INTEGR: c_uint = 0xf24f01ff;
// P2_TMGCFG
pub const RSTV0910_P2_TMGCFG: c_uint = 0xf250;
pub const FSTV0910_P2_TMGLOCK_BETA: c_uint = 0xf25060c0;
pub const FSTV0910_P2_DO_TIMING_CORR: c_uint = 0xf2504010;
pub const FSTV0910_P2_TMG_MINFREQ: c_uint = 0xf2500003;
// P2_RTC
pub const RSTV0910_P2_RTC: c_uint = 0xf251;
pub const FSTV0910_P2_TMGALPHA_EXP: c_uint = 0xf25140f0;
pub const FSTV0910_P2_TMGBETA_EXP: c_uint = 0xf251000f;
// P2_RTCS2
pub const RSTV0910_P2_RTCS2: c_uint = 0xf252;
pub const FSTV0910_P2_TMGALPHAS2_EXP: c_uint = 0xf25240f0;
pub const FSTV0910_P2_TMGBETAS2_EXP: c_uint = 0xf252000f;
// P2_TMGTHRISE
pub const RSTV0910_P2_TMGTHRISE: c_uint = 0xf253;
pub const FSTV0910_P2_TMGLOCK_THRISE: c_uint = 0xf25300ff;
// P2_TMGTHFALL
pub const RSTV0910_P2_TMGTHFALL: c_uint = 0xf254;
pub const FSTV0910_P2_TMGLOCK_THFALL: c_uint = 0xf25400ff;
// P2_SFRUPRATIO
pub const RSTV0910_P2_SFRUPRATIO: c_uint = 0xf255;
pub const FSTV0910_P2_SFR_UPRATIO: c_uint = 0xf25500ff;
// P2_SFRLOWRATIO
pub const RSTV0910_P2_SFRLOWRATIO: c_uint = 0xf256;
pub const FSTV0910_P2_SFR_LOWRATIO: c_uint = 0xf25600ff;
// P2_KTTMG
pub const RSTV0910_P2_KTTMG: c_uint = 0xf257;
pub const FSTV0910_P2_KT_TMG_EXP: c_uint = 0xf25740f0;
// P2_KREFTMG
pub const RSTV0910_P2_KREFTMG: c_uint = 0xf258;
pub const FSTV0910_P2_KREF_TMG: c_uint = 0xf25800ff;
// P2_SFRSTEP
pub const RSTV0910_P2_SFRSTEP: c_uint = 0xf259;
pub const FSTV0910_P2_SFR_SCANSTEP: c_uint = 0xf25940f0;
pub const FSTV0910_P2_SFR_CENTERSTEP: c_uint = 0xf259000f;
// P2_TMGCFG2
pub const RSTV0910_P2_TMGCFG2: c_uint = 0xf25a;
pub const FSTV0910_P2_DIS_AUTOSAMP: c_uint = 0xf25a3008;
pub const FSTV0910_P2_SFRRATIO_FINE: c_uint = 0xf25a0001;
// P2_KREFTMG2
pub const RSTV0910_P2_KREFTMG2: c_uint = 0xf25b;
pub const FSTV0910_P2_KREF_TMG2: c_uint = 0xf25b00ff;
// P2_TMGCFG3
pub const RSTV0910_P2_TMGCFG3: c_uint = 0xf25d;
pub const FSTV0910_P2_CONT_TMGCENTER: c_uint = 0xf25d3008;
pub const FSTV0910_P2_AUTO_GUP: c_uint = 0xf25d2004;
pub const FSTV0910_P2_AUTO_GLOW: c_uint = 0xf25d1002;
// P2_SFRINIT1
pub const RSTV0910_P2_SFRINIT1: c_uint = 0xf25e;
pub const FSTV0910_P2_SFR_INIT1: c_uint = 0xf25e00ff;
// P2_SFRINIT0
pub const RSTV0910_P2_SFRINIT0: c_uint = 0xf25f;
pub const FSTV0910_P2_SFR_INIT0: c_uint = 0xf25f00ff;
// P2_SFRUP1
pub const RSTV0910_P2_SFRUP1: c_uint = 0xf260;
pub const FSTV0910_P2_SYMB_FREQ_UP1: c_uint = 0xf26000ff;
// P2_SFRUP0
pub const RSTV0910_P2_SFRUP0: c_uint = 0xf261;
pub const FSTV0910_P2_SYMB_FREQ_UP0: c_uint = 0xf26100ff;
// P2_SFRLOW1
pub const RSTV0910_P2_SFRLOW1: c_uint = 0xf262;
pub const FSTV0910_P2_SYMB_FREQ_LOW1: c_uint = 0xf26200ff;
// P2_SFRLOW0
pub const RSTV0910_P2_SFRLOW0: c_uint = 0xf263;
pub const FSTV0910_P2_SYMB_FREQ_LOW0: c_uint = 0xf26300ff;
// P2_SFR3
pub const RSTV0910_P2_SFR3: c_uint = 0xf264;
pub const FSTV0910_P2_SYMB_FREQ3: c_uint = 0xf26400ff;
// P2_SFR2
pub const RSTV0910_P2_SFR2: c_uint = 0xf265;
pub const FSTV0910_P2_SYMB_FREQ2: c_uint = 0xf26500ff;
// P2_SFR1
pub const RSTV0910_P2_SFR1: c_uint = 0xf266;
pub const FSTV0910_P2_SYMB_FREQ1: c_uint = 0xf26600ff;
// P2_SFR0
pub const RSTV0910_P2_SFR0: c_uint = 0xf267;
pub const FSTV0910_P2_SYMB_FREQ0: c_uint = 0xf26700ff;
// P2_TMGREG2
pub const RSTV0910_P2_TMGREG2: c_uint = 0xf268;
pub const FSTV0910_P2_TMGREG2: c_uint = 0xf26800ff;
// P2_TMGREG1
pub const RSTV0910_P2_TMGREG1: c_uint = 0xf269;
pub const FSTV0910_P2_TMGREG1: c_uint = 0xf26900ff;
// P2_TMGREG0
pub const RSTV0910_P2_TMGREG0: c_uint = 0xf26a;
pub const FSTV0910_P2_TMGREG0: c_uint = 0xf26a00ff;
// P2_TMGLOCK1
pub const RSTV0910_P2_TMGLOCK1: c_uint = 0xf26b;
pub const FSTV0910_P2_TMGLOCK_LEVEL1: c_uint = 0xf26b01ff;
// P2_TMGLOCK0
pub const RSTV0910_P2_TMGLOCK0: c_uint = 0xf26c;
pub const FSTV0910_P2_TMGLOCK_LEVEL0: c_uint = 0xf26c00ff;
// P2_TMGOBS
pub const RSTV0910_P2_TMGOBS: c_uint = 0xf26d;
pub const FSTV0910_P2_ROLLOFF_STATUS: c_uint = 0xf26d60c0;
// P2_EQUALCFG
pub const RSTV0910_P2_EQUALCFG: c_uint = 0xf26f;
pub const FSTV0910_P2_EQUAL_ON: c_uint = 0xf26f6040;
pub const FSTV0910_P2_MU_EQUALDFE: c_uint = 0xf26f0007;
// P2_EQUAI1
pub const RSTV0910_P2_EQUAI1: c_uint = 0xf270;
pub const FSTV0910_P2_EQUA_ACCI1: c_uint = 0xf27001ff;
// P2_EQUAQ1
pub const RSTV0910_P2_EQUAQ1: c_uint = 0xf271;
pub const FSTV0910_P2_EQUA_ACCQ1: c_uint = 0xf27101ff;
// P2_EQUAI2
pub const RSTV0910_P2_EQUAI2: c_uint = 0xf272;
pub const FSTV0910_P2_EQUA_ACCI2: c_uint = 0xf27201ff;
// P2_EQUAQ2
pub const RSTV0910_P2_EQUAQ2: c_uint = 0xf273;
pub const FSTV0910_P2_EQUA_ACCQ2: c_uint = 0xf27301ff;
// P2_EQUAI3
pub const RSTV0910_P2_EQUAI3: c_uint = 0xf274;
pub const FSTV0910_P2_EQUA_ACCI3: c_uint = 0xf27401ff;
// P2_EQUAQ3
pub const RSTV0910_P2_EQUAQ3: c_uint = 0xf275;
pub const FSTV0910_P2_EQUA_ACCQ3: c_uint = 0xf27501ff;
// P2_EQUAI4
pub const RSTV0910_P2_EQUAI4: c_uint = 0xf276;
pub const FSTV0910_P2_EQUA_ACCI4: c_uint = 0xf27601ff;
// P2_EQUAQ4
pub const RSTV0910_P2_EQUAQ4: c_uint = 0xf277;
pub const FSTV0910_P2_EQUA_ACCQ4: c_uint = 0xf27701ff;
// P2_EQUAI5
pub const RSTV0910_P2_EQUAI5: c_uint = 0xf278;
pub const FSTV0910_P2_EQUA_ACCI5: c_uint = 0xf27801ff;
// P2_EQUAQ5
pub const RSTV0910_P2_EQUAQ5: c_uint = 0xf279;
pub const FSTV0910_P2_EQUA_ACCQ5: c_uint = 0xf27901ff;
// P2_EQUAI6
pub const RSTV0910_P2_EQUAI6: c_uint = 0xf27a;
pub const FSTV0910_P2_EQUA_ACCI6: c_uint = 0xf27a01ff;
// P2_EQUAQ6
pub const RSTV0910_P2_EQUAQ6: c_uint = 0xf27b;
pub const FSTV0910_P2_EQUA_ACCQ6: c_uint = 0xf27b01ff;
// P2_EQUAI7
pub const RSTV0910_P2_EQUAI7: c_uint = 0xf27c;
pub const FSTV0910_P2_EQUA_ACCI7: c_uint = 0xf27c01ff;
// P2_EQUAQ7
pub const RSTV0910_P2_EQUAQ7: c_uint = 0xf27d;
pub const FSTV0910_P2_EQUA_ACCQ7: c_uint = 0xf27d01ff;
// P2_EQUAI8
pub const RSTV0910_P2_EQUAI8: c_uint = 0xf27e;
pub const FSTV0910_P2_EQUA_ACCI8: c_uint = 0xf27e01ff;
// P2_EQUAQ8
pub const RSTV0910_P2_EQUAQ8: c_uint = 0xf27f;
pub const FSTV0910_P2_EQUA_ACCQ8: c_uint = 0xf27f01ff;
// P2_NNOSDATAT1
pub const RSTV0910_P2_NNOSDATAT1: c_uint = 0xf280;
pub const FSTV0910_P2_NOSDATAT_NORMED1: c_uint = 0xf28000ff;
// P2_NNOSDATAT0
pub const RSTV0910_P2_NNOSDATAT0: c_uint = 0xf281;
pub const FSTV0910_P2_NOSDATAT_NORMED0: c_uint = 0xf28100ff;
// P2_NNOSDATA1
pub const RSTV0910_P2_NNOSDATA1: c_uint = 0xf282;
pub const FSTV0910_P2_NOSDATA_NORMED1: c_uint = 0xf28200ff;
// P2_NNOSDATA0
pub const RSTV0910_P2_NNOSDATA0: c_uint = 0xf283;
pub const FSTV0910_P2_NOSDATA_NORMED0: c_uint = 0xf28300ff;
// P2_NNOSPLHT1
pub const RSTV0910_P2_NNOSPLHT1: c_uint = 0xf284;
pub const FSTV0910_P2_NOSPLHT_NORMED1: c_uint = 0xf28400ff;
// P2_NNOSPLHT0
pub const RSTV0910_P2_NNOSPLHT0: c_uint = 0xf285;
pub const FSTV0910_P2_NOSPLHT_NORMED0: c_uint = 0xf28500ff;
// P2_NNOSPLH1
pub const RSTV0910_P2_NNOSPLH1: c_uint = 0xf286;
pub const FSTV0910_P2_NOSPLH_NORMED1: c_uint = 0xf28600ff;
// P2_NNOSPLH0
pub const RSTV0910_P2_NNOSPLH0: c_uint = 0xf287;
pub const FSTV0910_P2_NOSPLH_NORMED0: c_uint = 0xf28700ff;
// P2_NOSDATAT1
pub const RSTV0910_P2_NOSDATAT1: c_uint = 0xf288;
pub const FSTV0910_P2_NOSDATAT_UNNORMED1: c_uint = 0xf28800ff;
// P2_NOSDATAT0
pub const RSTV0910_P2_NOSDATAT0: c_uint = 0xf289;
pub const FSTV0910_P2_NOSDATAT_UNNORMED0: c_uint = 0xf28900ff;
// P2_NNOSFRAME1
pub const RSTV0910_P2_NNOSFRAME1: c_uint = 0xf28a;
pub const FSTV0910_P2_NOSFRAME_NORMED1: c_uint = 0xf28a00ff;
// P2_NNOSFRAME0
pub const RSTV0910_P2_NNOSFRAME0: c_uint = 0xf28b;
pub const FSTV0910_P2_NOSFRAME_NORMED0: c_uint = 0xf28b00ff;
// P2_NNOSRAD1
pub const RSTV0910_P2_NNOSRAD1: c_uint = 0xf28c;
pub const FSTV0910_P2_NOSRADIAL_NORMED1: c_uint = 0xf28c00ff;
// P2_NNOSRAD0
pub const RSTV0910_P2_NNOSRAD0: c_uint = 0xf28d;
pub const FSTV0910_P2_NOSRADIAL_NORMED0: c_uint = 0xf28d00ff;
// P2_NOSCFGF1
pub const RSTV0910_P2_NOSCFGF1: c_uint = 0xf28e;
pub const FSTV0910_P2_LOWNOISE_MESURE: c_uint = 0xf28e7080;
pub const FSTV0910_P2_NOS_DELFRAME: c_uint = 0xf28e6040;
pub const FSTV0910_P2_NOSDATA_MODE: c_uint = 0xf28e4030;
pub const FSTV0910_P2_FRAMESEL_TYPESEL: c_uint = 0xf28e200c;
pub const FSTV0910_P2_FRAMESEL_TYPE: c_uint = 0xf28e0003;
// P2_NOSCFGF2
pub const RSTV0910_P2_NOSCFGF2: c_uint = 0xf28f;
pub const FSTV0910_P2_DIS_NOSPILOTS: c_uint = 0xf28f7080;
pub const FSTV0910_P2_FRAMESEL_MODCODSEL: c_uint = 0xf28f5060;
pub const FSTV0910_P2_FRAMESEL_MODCOD: c_uint = 0xf28f001f;
// P2_CAR2CFG
pub const RSTV0910_P2_CAR2CFG: c_uint = 0xf290;
pub const FSTV0910_P2_ROTA2ON: c_uint = 0xf2902004;
pub const FSTV0910_P2_PH_DET_ALGO2: c_uint = 0xf2900003;
// P2_CFR2CFR1
pub const RSTV0910_P2_CFR2CFR1: c_uint = 0xf291;
pub const FSTV0910_P2_EN_S2CAR2CENTER: c_uint = 0xf2915020;
pub const FSTV0910_P2_CFR2TOCFR1_BETA: c_uint = 0xf2910007;
// P2_CAR3CFG
pub const RSTV0910_P2_CAR3CFG: c_uint = 0xf292;
pub const FSTV0910_P2_CARRIER23_MODE: c_uint = 0xf29260c0;
pub const FSTV0910_P2_CAR3INTERM_DVBS1: c_uint = 0xf2925020;
pub const FSTV0910_P2_ABAMPLIF_MODE: c_uint = 0xf2923018;
pub const FSTV0910_P2_CARRIER3_ALPHA3DL: c_uint = 0xf2920007;
// P2_CFR22
pub const RSTV0910_P2_CFR22: c_uint = 0xf293;
pub const FSTV0910_P2_CAR2_FREQ2: c_uint = 0xf29301ff;
// P2_CFR21
pub const RSTV0910_P2_CFR21: c_uint = 0xf294;
pub const FSTV0910_P2_CAR2_FREQ1: c_uint = 0xf29400ff;
// P2_CFR20
pub const RSTV0910_P2_CFR20: c_uint = 0xf295;
pub const FSTV0910_P2_CAR2_FREQ0: c_uint = 0xf29500ff;
// P2_ACLC2S2Q
pub const RSTV0910_P2_ACLC2S2Q: c_uint = 0xf297;
pub const FSTV0910_P2_ENAB_SPSKSYMB: c_uint = 0xf2977080;
pub const FSTV0910_P2_CAR2S2_Q_ALPH_M: c_uint = 0xf2974030;
pub const FSTV0910_P2_CAR2S2_Q_ALPH_E: c_uint = 0xf297000f;
// P2_ACLC2S28
pub const RSTV0910_P2_ACLC2S28: c_uint = 0xf298;
pub const FSTV0910_P2_CAR2S2_8_ALPH_M: c_uint = 0xf2984030;
pub const FSTV0910_P2_CAR2S2_8_ALPH_E: c_uint = 0xf298000f;
// P2_ACLC2S216A
pub const RSTV0910_P2_ACLC2S216A: c_uint = 0xf299;
pub const FSTV0910_P2_CAR2S2_16A_ALPH_M: c_uint = 0xf2994030;
pub const FSTV0910_P2_CAR2S2_16A_ALPH_E: c_uint = 0xf299000f;
// P2_ACLC2S232A
pub const RSTV0910_P2_ACLC2S232A: c_uint = 0xf29a;
pub const FSTV0910_P2_CAR2S2_32A_ALPH_M: c_uint = 0xf29a4030;
pub const FSTV0910_P2_CAR2S2_32A_ALPH_E: c_uint = 0xf29a000f;
// P2_BCLC2S2Q
pub const RSTV0910_P2_BCLC2S2Q: c_uint = 0xf29c;
pub const FSTV0910_P2_CAR2S2_Q_BETA_M: c_uint = 0xf29c4030;
pub const FSTV0910_P2_CAR2S2_Q_BETA_E: c_uint = 0xf29c000f;
// P2_BCLC2S28
pub const RSTV0910_P2_BCLC2S28: c_uint = 0xf29d;
pub const FSTV0910_P2_CAR2S2_8_BETA_M: c_uint = 0xf29d4030;
pub const FSTV0910_P2_CAR2S2_8_BETA_E: c_uint = 0xf29d000f;
// P2_BCLC2S216A
pub const RSTV0910_P2_BCLC2S216A: c_uint = 0xf29e;
pub const FSTV0910_P2_DVBS2S216A_NIP: c_uint = 0xf29e7080;
pub const FSTV0910_P2_CAR2S2_16A_BETA_M: c_uint = 0xf29e4030;
pub const FSTV0910_P2_CAR2S2_16A_BETA_E: c_uint = 0xf29e000f;
// P2_BCLC2S232A
pub const RSTV0910_P2_BCLC2S232A: c_uint = 0xf29f;
pub const FSTV0910_P2_DVBS2S232A_NIP: c_uint = 0xf29f7080;
pub const FSTV0910_P2_CAR2S2_32A_BETA_M: c_uint = 0xf29f4030;
pub const FSTV0910_P2_CAR2S2_32A_BETA_E: c_uint = 0xf29f000f;
// P2_PLROOT2
pub const RSTV0910_P2_PLROOT2: c_uint = 0xf2ac;
pub const FSTV0910_P2_PLSCRAMB_MODE: c_uint = 0xf2ac200c;
pub const FSTV0910_P2_PLSCRAMB_ROOT2: c_uint = 0xf2ac0003;
// P2_PLROOT1
pub const RSTV0910_P2_PLROOT1: c_uint = 0xf2ad;
pub const FSTV0910_P2_PLSCRAMB_ROOT1: c_uint = 0xf2ad00ff;
// P2_PLROOT0
pub const RSTV0910_P2_PLROOT0: c_uint = 0xf2ae;
pub const FSTV0910_P2_PLSCRAMB_ROOT0: c_uint = 0xf2ae00ff;
// P2_MODCODLST0
pub const RSTV0910_P2_MODCODLST0: c_uint = 0xf2b0;
pub const FSTV0910_P2_NACCES_MODCODCH: c_uint = 0xf2b00001;
// P2_MODCODLST1
pub const RSTV0910_P2_MODCODLST1: c_uint = 0xf2b1;
pub const FSTV0910_P2_SYMBRATE_FILTER: c_uint = 0xf2b13008;
pub const FSTV0910_P2_NRESET_MODCODLST: c_uint = 0xf2b12004;
pub const FSTV0910_P2_DIS_32PSK_9_10: c_uint = 0xf2b10003;
// P2_MODCODLST2
pub const RSTV0910_P2_MODCODLST2: c_uint = 0xf2b2;
pub const FSTV0910_P2_DIS_32PSK_8_9: c_uint = 0xf2b240f0;
pub const FSTV0910_P2_DIS_32PSK_5_6: c_uint = 0xf2b2000f;
// P2_MODCODLST3
pub const RSTV0910_P2_MODCODLST3: c_uint = 0xf2b3;
pub const FSTV0910_P2_DIS_32PSK_4_5: c_uint = 0xf2b340f0;
pub const FSTV0910_P2_DIS_32PSK_3_4: c_uint = 0xf2b3000f;
// P2_MODCODLST4
pub const RSTV0910_P2_MODCODLST4: c_uint = 0xf2b4;
pub const FSTV0910_P2_DUMMYPL_PILOT: c_uint = 0xf2b47080;
pub const FSTV0910_P2_DUMMYPL_NOPILOT: c_uint = 0xf2b46040;
pub const FSTV0910_P2_DIS_16PSK_9_10: c_uint = 0xf2b44030;
pub const FSTV0910_P2_DIS_16PSK_8_9: c_uint = 0xf2b4000f;
// P2_MODCODLST5
pub const RSTV0910_P2_MODCODLST5: c_uint = 0xf2b5;
pub const FSTV0910_P2_DIS_16PSK_5_6: c_uint = 0xf2b540f0;
pub const FSTV0910_P2_DIS_16PSK_4_5: c_uint = 0xf2b5000f;
// P2_MODCODLST6
pub const RSTV0910_P2_MODCODLST6: c_uint = 0xf2b6;
pub const FSTV0910_P2_DIS_16PSK_3_4: c_uint = 0xf2b640f0;
pub const FSTV0910_P2_DIS_16PSK_2_3: c_uint = 0xf2b6000f;
// P2_MODCODLST7
pub const RSTV0910_P2_MODCODLST7: c_uint = 0xf2b7;
pub const FSTV0910_P2_MODCOD_NNOSFILTER: c_uint = 0xf2b77080;
pub const FSTV0910_P2_DIS_8PSK_9_10: c_uint = 0xf2b74030;
pub const FSTV0910_P2_DIS_8PSK_8_9: c_uint = 0xf2b7000f;
// P2_MODCODLST8
pub const RSTV0910_P2_MODCODLST8: c_uint = 0xf2b8;
pub const FSTV0910_P2_DIS_8PSK_5_6: c_uint = 0xf2b840f0;
pub const FSTV0910_P2_DIS_8PSK_3_4: c_uint = 0xf2b8000f;
// P2_MODCODLST9
pub const RSTV0910_P2_MODCODLST9: c_uint = 0xf2b9;
pub const FSTV0910_P2_DIS_8PSK_2_3: c_uint = 0xf2b940f0;
pub const FSTV0910_P2_DIS_8PSK_3_5: c_uint = 0xf2b9000f;
// P2_MODCODLSTA
pub const RSTV0910_P2_MODCODLSTA: c_uint = 0xf2ba;
pub const FSTV0910_P2_NOSFILTER_LIMITE: c_uint = 0xf2ba7080;
pub const FSTV0910_P2_DIS_QPSK_9_10: c_uint = 0xf2ba4030;
pub const FSTV0910_P2_DIS_QPSK_8_9: c_uint = 0xf2ba000f;
// P2_MODCODLSTB
pub const RSTV0910_P2_MODCODLSTB: c_uint = 0xf2bb;
pub const FSTV0910_P2_DIS_QPSK_5_6: c_uint = 0xf2bb40f0;
pub const FSTV0910_P2_DIS_QPSK_4_5: c_uint = 0xf2bb000f;
// P2_MODCODLSTC
pub const RSTV0910_P2_MODCODLSTC: c_uint = 0xf2bc;
pub const FSTV0910_P2_DIS_QPSK_3_4: c_uint = 0xf2bc40f0;
pub const FSTV0910_P2_DIS_QPSK_2_3: c_uint = 0xf2bc000f;
// P2_MODCODLSTD
pub const RSTV0910_P2_MODCODLSTD: c_uint = 0xf2bd;
pub const FSTV0910_P2_DIS_QPSK_3_5: c_uint = 0xf2bd40f0;
pub const FSTV0910_P2_DIS_QPSK_1_2: c_uint = 0xf2bd000f;
// P2_MODCODLSTE
pub const RSTV0910_P2_MODCODLSTE: c_uint = 0xf2be;
pub const FSTV0910_P2_DIS_QPSK_2_5: c_uint = 0xf2be40f0;
pub const FSTV0910_P2_DIS_QPSK_1_3: c_uint = 0xf2be000f;
// P2_MODCODLSTF
pub const RSTV0910_P2_MODCODLSTF: c_uint = 0xf2bf;
pub const FSTV0910_P2_DIS_QPSK_1_4: c_uint = 0xf2bf40f0;
pub const FSTV0910_P2_DEMOD_INVMODLST: c_uint = 0xf2bf3008;
pub const FSTV0910_P2_DEMODOUT_ENABLE: c_uint = 0xf2bf2004;
pub const FSTV0910_P2_DDEMOD_NSET: c_uint = 0xf2bf1002;
pub const FSTV0910_P2_MODCOD_NSTOCK: c_uint = 0xf2bf0001;
// P2_GAUSSR0
pub const RSTV0910_P2_GAUSSR0: c_uint = 0xf2c0;
pub const FSTV0910_P2_EN_CCIMODE: c_uint = 0xf2c07080;
pub const FSTV0910_P2_R0_GAUSSIEN: c_uint = 0xf2c0007f;
// P2_CCIR0
pub const RSTV0910_P2_CCIR0: c_uint = 0xf2c1;
pub const FSTV0910_P2_CCIDETECT_PLHONLY: c_uint = 0xf2c17080;
pub const FSTV0910_P2_R0_CCI: c_uint = 0xf2c1007f;
// P2_CCIQUANT
pub const RSTV0910_P2_CCIQUANT: c_uint = 0xf2c2;
pub const FSTV0910_P2_CCI_BETA: c_uint = 0xf2c250e0;
pub const FSTV0910_P2_CCI_QUANT: c_uint = 0xf2c2001f;
// P2_CCITHRES
pub const RSTV0910_P2_CCITHRES: c_uint = 0xf2c3;
pub const FSTV0910_P2_CCI_THRESHOLD: c_uint = 0xf2c300ff;
// P2_CCIACC
pub const RSTV0910_P2_CCIACC: c_uint = 0xf2c4;
pub const FSTV0910_P2_CCI_VALUE: c_uint = 0xf2c400ff;
// P2_DSTATUS4
pub const RSTV0910_P2_DSTATUS4: c_uint = 0xf2c5;
pub const FSTV0910_P2_RAINFADE_DETECT: c_uint = 0xf2c57080;
pub const FSTV0910_P2_NOTHRES2_FAIL: c_uint = 0xf2c56040;
pub const FSTV0910_P2_NOTHRES1_FAIL: c_uint = 0xf2c55020;
pub const FSTV0910_P2_DMDPROG_ERROR: c_uint = 0xf2c52004;
pub const FSTV0910_P2_CSTENV_DETECT: c_uint = 0xf2c51002;
pub const FSTV0910_P2_DETECTION_TRIAX: c_uint = 0xf2c50001;
// P2_DMDRESCFG
pub const RSTV0910_P2_DMDRESCFG: c_uint = 0xf2c6;
pub const FSTV0910_P2_DMDRES_RESET: c_uint = 0xf2c67080;
pub const FSTV0910_P2_DMDRES_STRALL: c_uint = 0xf2c63008;
pub const FSTV0910_P2_DMDRES_NEWONLY: c_uint = 0xf2c62004;
pub const FSTV0910_P2_DMDRES_NOSTORE: c_uint = 0xf2c61002;
// P2_DMDRESADR
pub const RSTV0910_P2_DMDRESADR: c_uint = 0xf2c7;
pub const FSTV0910_P2_DMDRES_VALIDCFR: c_uint = 0xf2c76040;
pub const FSTV0910_P2_DMDRES_MEMFULL: c_uint = 0xf2c74030;
pub const FSTV0910_P2_DMDRES_RESNBR: c_uint = 0xf2c7000f;
// P2_DMDRESDATA7
pub const RSTV0910_P2_DMDRESDATA7: c_uint = 0xf2c8;
pub const FSTV0910_P2_DMDRES_DATA7: c_uint = 0xf2c800ff;
// P2_DMDRESDATA6
pub const RSTV0910_P2_DMDRESDATA6: c_uint = 0xf2c9;
pub const FSTV0910_P2_DMDRES_DATA6: c_uint = 0xf2c900ff;
// P2_DMDRESDATA5
pub const RSTV0910_P2_DMDRESDATA5: c_uint = 0xf2ca;
pub const FSTV0910_P2_DMDRES_DATA5: c_uint = 0xf2ca00ff;
// P2_DMDRESDATA4
pub const RSTV0910_P2_DMDRESDATA4: c_uint = 0xf2cb;
pub const FSTV0910_P2_DMDRES_DATA4: c_uint = 0xf2cb00ff;
// P2_DMDRESDATA3
pub const RSTV0910_P2_DMDRESDATA3: c_uint = 0xf2cc;
pub const FSTV0910_P2_DMDRES_DATA3: c_uint = 0xf2cc00ff;
// P2_DMDRESDATA2
pub const RSTV0910_P2_DMDRESDATA2: c_uint = 0xf2cd;
pub const FSTV0910_P2_DMDRES_DATA2: c_uint = 0xf2cd00ff;
// P2_DMDRESDATA1
pub const RSTV0910_P2_DMDRESDATA1: c_uint = 0xf2ce;
pub const FSTV0910_P2_DMDRES_DATA1: c_uint = 0xf2ce00ff;
// P2_DMDRESDATA0
pub const RSTV0910_P2_DMDRESDATA0: c_uint = 0xf2cf;
pub const FSTV0910_P2_DMDRES_DATA0: c_uint = 0xf2cf00ff;
// P2_FFEI1
pub const RSTV0910_P2_FFEI1: c_uint = 0xf2d0;
pub const FSTV0910_P2_FFE_ACCI1: c_uint = 0xf2d001ff;
// P2_FFEQ1
pub const RSTV0910_P2_FFEQ1: c_uint = 0xf2d1;
pub const FSTV0910_P2_FFE_ACCQ1: c_uint = 0xf2d101ff;
// P2_FFEI2
pub const RSTV0910_P2_FFEI2: c_uint = 0xf2d2;
pub const FSTV0910_P2_FFE_ACCI2: c_uint = 0xf2d201ff;
// P2_FFEQ2
pub const RSTV0910_P2_FFEQ2: c_uint = 0xf2d3;
pub const FSTV0910_P2_FFE_ACCQ2: c_uint = 0xf2d301ff;
// P2_FFEI3
pub const RSTV0910_P2_FFEI3: c_uint = 0xf2d4;
pub const FSTV0910_P2_FFE_ACCI3: c_uint = 0xf2d401ff;
// P2_FFEQ3
pub const RSTV0910_P2_FFEQ3: c_uint = 0xf2d5;
pub const FSTV0910_P2_FFE_ACCQ3: c_uint = 0xf2d501ff;
// P2_FFEI4
pub const RSTV0910_P2_FFEI4: c_uint = 0xf2d6;
pub const FSTV0910_P2_FFE_ACCI4: c_uint = 0xf2d601ff;
// P2_FFEQ4
pub const RSTV0910_P2_FFEQ4: c_uint = 0xf2d7;
pub const FSTV0910_P2_FFE_ACCQ4: c_uint = 0xf2d701ff;
// P2_FFECFG
pub const RSTV0910_P2_FFECFG: c_uint = 0xf2d8;
pub const FSTV0910_P2_EQUALFFE_ON: c_uint = 0xf2d86040;
pub const FSTV0910_P2_EQUAL_USEDSYMB: c_uint = 0xf2d84030;
pub const FSTV0910_P2_MU_EQUALFFE: c_uint = 0xf2d80007;
// P2_TNRCFG2
pub const RSTV0910_P2_TNRCFG2: c_uint = 0xf2e1;
pub const FSTV0910_P2_TUN_IQSWAP: c_uint = 0xf2e17080;
// P2_SMAPCOEF7
pub const RSTV0910_P2_SMAPCOEF7: c_uint = 0xf300;
pub const FSTV0910_P2_DIS_QSCALE: c_uint = 0xf3007080;
pub const FSTV0910_P2_SMAPCOEF_Q_LLR12: c_uint = 0xf300017f;
// P2_SMAPCOEF6
pub const RSTV0910_P2_SMAPCOEF6: c_uint = 0xf301;
pub const FSTV0910_P2_DIS_AGC2SCALE: c_uint = 0xf3017080;
pub const FSTV0910_P2_ADJ_8PSKLLR1: c_uint = 0xf3012004;
pub const FSTV0910_P2_OLD_8PSKLLR1: c_uint = 0xf3011002;
pub const FSTV0910_P2_DIS_AB8PSK: c_uint = 0xf3010001;
// P2_SMAPCOEF5
pub const RSTV0910_P2_SMAPCOEF5: c_uint = 0xf302;
pub const FSTV0910_P2_DIS_8SCALE: c_uint = 0xf3027080;
pub const FSTV0910_P2_SMAPCOEF_8P_LLR23: c_uint = 0xf302017f;
// P2_SMAPCOEF4
pub const RSTV0910_P2_SMAPCOEF4: c_uint = 0xf303;
pub const FSTV0910_P2_SMAPCOEF_16APSK_LLR12: c_uint = 0xf303017f;
// P2_SMAPCOEF3
pub const RSTV0910_P2_SMAPCOEF3: c_uint = 0xf304;
pub const FSTV0910_P2_SMAPCOEF_16APSK_LLR34: c_uint = 0xf304017f;
// P2_SMAPCOEF2
pub const RSTV0910_P2_SMAPCOEF2: c_uint = 0xf305;
pub const FSTV0910_P2_SMAPCOEF_32APSK_R2R3: c_uint = 0xf30541f0;
pub const FSTV0910_P2_SMAPCOEF_32APSK_LLR2: c_uint = 0xf305010f;
// P2_SMAPCOEF1
pub const RSTV0910_P2_SMAPCOEF1: c_uint = 0xf306;
pub const FSTV0910_P2_DIS_16SCALE: c_uint = 0xf3067080;
pub const FSTV0910_P2_SMAPCOEF_32_LLR34: c_uint = 0xf306017f;
// P2_SMAPCOEF0
pub const RSTV0910_P2_SMAPCOEF0: c_uint = 0xf307;
pub const FSTV0910_P2_DIS_32SCALE: c_uint = 0xf3077080;
pub const FSTV0910_P2_SMAPCOEF_32_LLR15: c_uint = 0xf307017f;
// P2_NOSTHRES1
pub const RSTV0910_P2_NOSTHRES1: c_uint = 0xf309;
pub const FSTV0910_P2_NOS_THRESHOLD1: c_uint = 0xf30900ff;
// P2_NOSTHRES2
pub const RSTV0910_P2_NOSTHRES2: c_uint = 0xf30a;
pub const FSTV0910_P2_NOS_THRESHOLD2: c_uint = 0xf30a00ff;
// P2_NOSDIFF1
pub const RSTV0910_P2_NOSDIFF1: c_uint = 0xf30b;
pub const FSTV0910_P2_NOSTHRES1_DIFF: c_uint = 0xf30b00ff;
// P2_RAINFADE
pub const RSTV0910_P2_RAINFADE: c_uint = 0xf30c;
pub const FSTV0910_P2_NOSTHRES_DATAT: c_uint = 0xf30c7080;
pub const FSTV0910_P2_RAINFADE_CNLIMIT: c_uint = 0xf30c4070;
pub const FSTV0910_P2_RAINFADE_TIMEOUT: c_uint = 0xf30c0007;
// P2_NOSRAMCFG
pub const RSTV0910_P2_NOSRAMCFG: c_uint = 0xf30d;
pub const FSTV0910_P2_NOSRAM_ACTIVATION: c_uint = 0xf30d4030;
pub const FSTV0910_P2_NOSRAM_CNRONLY: c_uint = 0xf30d3008;
pub const FSTV0910_P2_NOSRAM_LGNCNR1: c_uint = 0xf30d0007;
// P2_NOSRAMPOS
pub const RSTV0910_P2_NOSRAMPOS: c_uint = 0xf30e;
pub const FSTV0910_P2_NOSRAM_LGNCNR0: c_uint = 0xf30e40f0;
pub const FSTV0910_P2_NOSRAM_VALIDE: c_uint = 0xf30e2004;
pub const FSTV0910_P2_NOSRAM_CNRVAL1: c_uint = 0xf30e0003;
// P2_NOSRAMVAL
pub const RSTV0910_P2_NOSRAMVAL: c_uint = 0xf30f;
pub const FSTV0910_P2_NOSRAM_CNRVAL0: c_uint = 0xf30f00ff;
// P2_DMDPLHSTAT
pub const RSTV0910_P2_DMDPLHSTAT: c_uint = 0xf320;
pub const FSTV0910_P2_PLH_STATISTIC: c_uint = 0xf32000ff;
// P2_LOCKTIME3
pub const RSTV0910_P2_LOCKTIME3: c_uint = 0xf322;
pub const FSTV0910_P2_DEMOD_LOCKTIME3: c_uint = 0xf32200ff;
// P2_LOCKTIME2
pub const RSTV0910_P2_LOCKTIME2: c_uint = 0xf323;
pub const FSTV0910_P2_DEMOD_LOCKTIME2: c_uint = 0xf32300ff;
// P2_LOCKTIME1
pub const RSTV0910_P2_LOCKTIME1: c_uint = 0xf324;
pub const FSTV0910_P2_DEMOD_LOCKTIME1: c_uint = 0xf32400ff;
// P2_LOCKTIME0
pub const RSTV0910_P2_LOCKTIME0: c_uint = 0xf325;
pub const FSTV0910_P2_DEMOD_LOCKTIME0: c_uint = 0xf32500ff;
// P2_VITSCALE
pub const RSTV0910_P2_VITSCALE: c_uint = 0xf332;
pub const FSTV0910_P2_NVTH_NOSRANGE: c_uint = 0xf3327080;
pub const FSTV0910_P2_VERROR_MAXMODE: c_uint = 0xf3326040;
pub const FSTV0910_P2_NSLOWSN_LOCKED: c_uint = 0xf3323008;
pub const FSTV0910_P2_DIS_RSFLOCK: c_uint = 0xf3321002;
// P2_FECM
pub const RSTV0910_P2_FECM: c_uint = 0xf333;
pub const FSTV0910_P2_DSS_DVB: c_uint = 0xf3337080;
pub const FSTV0910_P2_DSS_SRCH: c_uint = 0xf3334010;
pub const FSTV0910_P2_SYNCVIT: c_uint = 0xf3331002;
pub const FSTV0910_P2_IQINV: c_uint = 0xf3330001;
// P2_VTH12
pub const RSTV0910_P2_VTH12: c_uint = 0xf334;
pub const FSTV0910_P2_VTH12: c_uint = 0xf33400ff;
// P2_VTH23
pub const RSTV0910_P2_VTH23: c_uint = 0xf335;
pub const FSTV0910_P2_VTH23: c_uint = 0xf33500ff;
// P2_VTH34
pub const RSTV0910_P2_VTH34: c_uint = 0xf336;
pub const FSTV0910_P2_VTH34: c_uint = 0xf33600ff;
// P2_VTH56
pub const RSTV0910_P2_VTH56: c_uint = 0xf337;
pub const FSTV0910_P2_VTH56: c_uint = 0xf33700ff;
// P2_VTH67
pub const RSTV0910_P2_VTH67: c_uint = 0xf338;
pub const FSTV0910_P2_VTH67: c_uint = 0xf33800ff;
// P2_VTH78
pub const RSTV0910_P2_VTH78: c_uint = 0xf339;
pub const FSTV0910_P2_VTH78: c_uint = 0xf33900ff;
// P2_VITCURPUN
pub const RSTV0910_P2_VITCURPUN: c_uint = 0xf33a;
pub const FSTV0910_P2_VIT_CURPUN: c_uint = 0xf33a001f;
// P2_VERROR
pub const RSTV0910_P2_VERROR: c_uint = 0xf33b;
pub const FSTV0910_P2_REGERR_VIT: c_uint = 0xf33b00ff;
// P2_PRVIT
pub const RSTV0910_P2_PRVIT: c_uint = 0xf33c;
pub const FSTV0910_P2_DIS_VTHLOCK: c_uint = 0xf33c6040;
pub const FSTV0910_P2_E7_8VIT: c_uint = 0xf33c5020;
pub const FSTV0910_P2_E6_7VIT: c_uint = 0xf33c4010;
pub const FSTV0910_P2_E5_6VIT: c_uint = 0xf33c3008;
pub const FSTV0910_P2_E3_4VIT: c_uint = 0xf33c2004;
pub const FSTV0910_P2_E2_3VIT: c_uint = 0xf33c1002;
pub const FSTV0910_P2_E1_2VIT: c_uint = 0xf33c0001;
// P2_VAVSRVIT
pub const RSTV0910_P2_VAVSRVIT: c_uint = 0xf33d;
pub const FSTV0910_P2_AMVIT: c_uint = 0xf33d7080;
pub const FSTV0910_P2_FROZENVIT: c_uint = 0xf33d6040;
pub const FSTV0910_P2_SNVIT: c_uint = 0xf33d4030;
pub const FSTV0910_P2_TOVVIT: c_uint = 0xf33d200c;
pub const FSTV0910_P2_HYPVIT: c_uint = 0xf33d0003;
// P2_VSTATUSVIT
pub const RSTV0910_P2_VSTATUSVIT: c_uint = 0xf33e;
pub const FSTV0910_P2_PRFVIT: c_uint = 0xf33e4010;
pub const FSTV0910_P2_LOCKEDVIT: c_uint = 0xf33e3008;
// P2_VTHINUSE
pub const RSTV0910_P2_VTHINUSE: c_uint = 0xf33f;
pub const FSTV0910_P2_VIT_INUSE: c_uint = 0xf33f00ff;
// P2_KDIV12
pub const RSTV0910_P2_KDIV12: c_uint = 0xf340;
pub const FSTV0910_P2_K_DIVIDER_12: c_uint = 0xf340007f;
// P2_KDIV23
pub const RSTV0910_P2_KDIV23: c_uint = 0xf341;
pub const FSTV0910_P2_K_DIVIDER_23: c_uint = 0xf341007f;
// P2_KDIV34
pub const RSTV0910_P2_KDIV34: c_uint = 0xf342;
pub const FSTV0910_P2_K_DIVIDER_34: c_uint = 0xf342007f;
// P2_KDIV56
pub const RSTV0910_P2_KDIV56: c_uint = 0xf343;
pub const FSTV0910_P2_K_DIVIDER_56: c_uint = 0xf343007f;
// P2_KDIV67
pub const RSTV0910_P2_KDIV67: c_uint = 0xf344;
pub const FSTV0910_P2_K_DIVIDER_67: c_uint = 0xf344007f;
// P2_KDIV78
pub const RSTV0910_P2_KDIV78: c_uint = 0xf345;
pub const FSTV0910_P2_K_DIVIDER_78: c_uint = 0xf345007f;
// P2_TSPIDFLT1
pub const RSTV0910_P2_TSPIDFLT1: c_uint = 0xf346;
pub const FSTV0910_P2_PIDFLT_ADDR: c_uint = 0xf34600ff;
// P2_TSPIDFLT0
pub const RSTV0910_P2_TSPIDFLT0: c_uint = 0xf347;
pub const FSTV0910_P2_PIDFLT_DATA: c_uint = 0xf34700ff;
// P2_PDELCTRL0
pub const RSTV0910_P2_PDELCTRL0: c_uint = 0xf34f;
pub const FSTV0910_P2_ISIOBS_MODE: c_uint = 0xf34f4030;
// P2_PDELCTRL1
pub const RSTV0910_P2_PDELCTRL1: c_uint = 0xf350;
pub const FSTV0910_P2_INV_MISMASK: c_uint = 0xf3507080;
pub const FSTV0910_P2_FILTER_EN: c_uint = 0xf3505020;
pub const FSTV0910_P2_HYSTEN: c_uint = 0xf3503008;
pub const FSTV0910_P2_HYSTSWRST: c_uint = 0xf3502004;
pub const FSTV0910_P2_EN_MIS00: c_uint = 0xf3501002;
pub const FSTV0910_P2_ALGOSWRST: c_uint = 0xf3500001;
// P2_PDELCTRL2
pub const RSTV0910_P2_PDELCTRL2: c_uint = 0xf351;
pub const FSTV0910_P2_FORCE_CONTINUOUS: c_uint = 0xf3517080;
pub const FSTV0910_P2_RESET_UPKO_COUNT: c_uint = 0xf3516040;
pub const FSTV0910_P2_USER_PKTDELIN_NB: c_uint = 0xf3515020;
pub const FSTV0910_P2_FRAME_MODE: c_uint = 0xf3511002;
// P2_HYSTTHRESH
pub const RSTV0910_P2_HYSTTHRESH: c_uint = 0xf354;
pub const FSTV0910_P2_DELIN_LOCKTHRES: c_uint = 0xf35440f0;
pub const FSTV0910_P2_DELIN_UNLOCKTHRES: c_uint = 0xf354000f;
// P2_UPLCCST0
pub const RSTV0910_P2_UPLCCST0: c_uint = 0xf358;
pub const FSTV0910_P2_UPL_CST0: c_uint = 0xf35830f8;
pub const FSTV0910_P2_UPL_MODE: c_uint = 0xf3580007;
// P2_ISIENTRY
pub const RSTV0910_P2_ISIENTRY: c_uint = 0xf35e;
pub const FSTV0910_P2_ISI_ENTRY: c_uint = 0xf35e00ff;
// P2_ISIBITENA
pub const RSTV0910_P2_ISIBITENA: c_uint = 0xf35f;
pub const FSTV0910_P2_ISI_BIT_EN: c_uint = 0xf35f00ff;
// P2_MATSTR1
pub const RSTV0910_P2_MATSTR1: c_uint = 0xf360;
pub const FSTV0910_P2_MATYPE_CURRENT1: c_uint = 0xf36000ff;
// P2_MATSTR0
pub const RSTV0910_P2_MATSTR0: c_uint = 0xf361;
pub const FSTV0910_P2_MATYPE_CURRENT0: c_uint = 0xf36100ff;
// P2_UPLSTR1
pub const RSTV0910_P2_UPLSTR1: c_uint = 0xf362;
pub const FSTV0910_P2_UPL_CURRENT1: c_uint = 0xf36200ff;
// P2_UPLSTR0
pub const RSTV0910_P2_UPLSTR0: c_uint = 0xf363;
pub const FSTV0910_P2_UPL_CURRENT0: c_uint = 0xf36300ff;
// P2_DFLSTR1
pub const RSTV0910_P2_DFLSTR1: c_uint = 0xf364;
pub const FSTV0910_P2_DFL_CURRENT1: c_uint = 0xf36400ff;
// P2_DFLSTR0
pub const RSTV0910_P2_DFLSTR0: c_uint = 0xf365;
pub const FSTV0910_P2_DFL_CURRENT0: c_uint = 0xf36500ff;
// P2_SYNCSTR
pub const RSTV0910_P2_SYNCSTR: c_uint = 0xf366;
pub const FSTV0910_P2_SYNC_CURRENT: c_uint = 0xf36600ff;
// P2_SYNCDSTR1
pub const RSTV0910_P2_SYNCDSTR1: c_uint = 0xf367;
pub const FSTV0910_P2_SYNCD_CURRENT1: c_uint = 0xf36700ff;
// P2_SYNCDSTR0
pub const RSTV0910_P2_SYNCDSTR0: c_uint = 0xf368;
pub const FSTV0910_P2_SYNCD_CURRENT0: c_uint = 0xf36800ff;
// P2_PDELSTATUS1
pub const RSTV0910_P2_PDELSTATUS1: c_uint = 0xf369;
pub const FSTV0910_P2_PKTDELIN_DELOCK: c_uint = 0xf3697080;
pub const FSTV0910_P2_SYNCDUPDFL_BADDFL: c_uint = 0xf3696040;
pub const FSTV0910_P2_UNACCEPTED_STREAM: c_uint = 0xf3694010;
pub const FSTV0910_P2_BCH_ERROR_FLAG: c_uint = 0xf3693008;
pub const FSTV0910_P2_PKTDELIN_LOCK: c_uint = 0xf3691002;
pub const FSTV0910_P2_FIRST_LOCK: c_uint = 0xf3690001;
// P2_PDELSTATUS2
pub const RSTV0910_P2_PDELSTATUS2: c_uint = 0xf36a;
pub const FSTV0910_P2_FRAME_MODCOD: c_uint = 0xf36a207c;
pub const FSTV0910_P2_FRAME_TYPE: c_uint = 0xf36a0003;
// P2_BBFCRCKO1
pub const RSTV0910_P2_BBFCRCKO1: c_uint = 0xf36b;
pub const FSTV0910_P2_BBHCRC_KOCNT1: c_uint = 0xf36b00ff;
// P2_BBFCRCKO0
pub const RSTV0910_P2_BBFCRCKO0: c_uint = 0xf36c;
pub const FSTV0910_P2_BBHCRC_KOCNT0: c_uint = 0xf36c00ff;
// P2_UPCRCKO1
pub const RSTV0910_P2_UPCRCKO1: c_uint = 0xf36d;
pub const FSTV0910_P2_PKTCRC_KOCNT1: c_uint = 0xf36d00ff;
// P2_UPCRCKO0
pub const RSTV0910_P2_UPCRCKO0: c_uint = 0xf36e;
pub const FSTV0910_P2_PKTCRC_KOCNT0: c_uint = 0xf36e00ff;
// P2_PDELCTRL3
pub const RSTV0910_P2_PDELCTRL3: c_uint = 0xf36f;
pub const FSTV0910_P2_NOFIFO_BCHERR: c_uint = 0xf36f5020;
pub const FSTV0910_P2_PKTDELIN_DELACMERR: c_uint = 0xf36f4010;
// P2_TSSTATEM
pub const RSTV0910_P2_TSSTATEM: c_uint = 0xf370;
pub const FSTV0910_P2_TSDIL_ON: c_uint = 0xf3707080;
pub const FSTV0910_P2_TSRS_ON: c_uint = 0xf3705020;
pub const FSTV0910_P2_TSDESCRAMB_ON: c_uint = 0xf3704010;
pub const FSTV0910_P2_TSFRAME_MODE: c_uint = 0xf3703008;
pub const FSTV0910_P2_TS_DISABLE: c_uint = 0xf3702004;
pub const FSTV0910_P2_TSACM_MODE: c_uint = 0xf3701002;
pub const FSTV0910_P2_TSOUT_NOSYNC: c_uint = 0xf3700001;
// P2_TSSTATEL
pub const RSTV0910_P2_TSSTATEL: c_uint = 0xf371;
pub const FSTV0910_P2_TSNOSYNCBYTE: c_uint = 0xf3717080;
pub const FSTV0910_P2_TSPARITY_ON: c_uint = 0xf3716040;
pub const FSTV0910_P2_TSISSYI_ON: c_uint = 0xf3713008;
pub const FSTV0910_P2_TSNPD_ON: c_uint = 0xf3712004;
pub const FSTV0910_P2_TSCRC8_ON: c_uint = 0xf3711002;
pub const FSTV0910_P2_TSDSS_PACKET: c_uint = 0xf3710001;
// P2_TSCFGH
pub const RSTV0910_P2_TSCFGH: c_uint = 0xf372;
pub const FSTV0910_P2_TSFIFO_DVBCI: c_uint = 0xf3727080;
pub const FSTV0910_P2_TSFIFO_SERIAL: c_uint = 0xf3726040;
pub const FSTV0910_P2_TSFIFO_TEIUPDATE: c_uint = 0xf3725020;
pub const FSTV0910_P2_TSFIFO_DUTY50: c_uint = 0xf3724010;
pub const FSTV0910_P2_TSFIFO_HSGNLOUT: c_uint = 0xf3723008;
pub const FSTV0910_P2_TSFIFO_ERRMODE: c_uint = 0xf3721006;
pub const FSTV0910_P2_RST_HWARE: c_uint = 0xf3720001;
// P2_TSCFGM
pub const RSTV0910_P2_TSCFGM: c_uint = 0xf373;
pub const FSTV0910_P2_TSFIFO_MANSPEED: c_uint = 0xf37360c0;
pub const FSTV0910_P2_TSFIFO_PERMDATA: c_uint = 0xf3735020;
pub const FSTV0910_P2_TSFIFO_NONEWSGNL: c_uint = 0xf3734010;
pub const FSTV0910_P2_TSFIFO_INVDATA: c_uint = 0xf3730001;
// P2_TSCFGL
pub const RSTV0910_P2_TSCFGL: c_uint = 0xf374;
pub const FSTV0910_P2_TSFIFO_BCLKDEL1CK: c_uint = 0xf37460c0;
pub const FSTV0910_P2_BCHERROR_MODE: c_uint = 0xf3744030;
pub const FSTV0910_P2_TSFIFO_NSGNL2DATA: c_uint = 0xf3743008;
pub const FSTV0910_P2_TSFIFO_EMBINDVB: c_uint = 0xf3742004;
pub const FSTV0910_P2_TSFIFO_BITSPEED: c_uint = 0xf3740003;
// P2_TSSYNC
pub const RSTV0910_P2_TSSYNC: c_uint = 0xf375;
pub const FSTV0910_P2_TSFIFO_SYNCMODE: c_uint = 0xf3753018;
// P2_TSINSDELH
pub const RSTV0910_P2_TSINSDELH: c_uint = 0xf376;
pub const FSTV0910_P2_TSDEL_SYNCBYTE: c_uint = 0xf3767080;
pub const FSTV0910_P2_TSDEL_XXHEADER: c_uint = 0xf3766040;
pub const FSTV0910_P2_TSDEL_DATAFIELD: c_uint = 0xf3764010;
pub const FSTV0910_P2_TSINSDEL_RSPARITY: c_uint = 0xf3761002;
pub const FSTV0910_P2_TSINSDEL_CRC8: c_uint = 0xf3760001;
// P2_TSINSDELM
pub const RSTV0910_P2_TSINSDELM: c_uint = 0xf377;
pub const FSTV0910_P2_TSINS_EMODCOD: c_uint = 0xf3774010;
pub const FSTV0910_P2_TSINS_TOKEN: c_uint = 0xf3773008;
pub const FSTV0910_P2_TSINS_XXXERR: c_uint = 0xf3772004;
pub const FSTV0910_P2_TSINS_MATYPE: c_uint = 0xf3771002;
pub const FSTV0910_P2_TSINS_UPL: c_uint = 0xf3770001;
// P2_TSINSDELL
pub const RSTV0910_P2_TSINSDELL: c_uint = 0xf378;
pub const FSTV0910_P2_TSINS_DFL: c_uint = 0xf3787080;
pub const FSTV0910_P2_TSINS_SYNCD: c_uint = 0xf3786040;
pub const FSTV0910_P2_TSINS_BLOCLEN: c_uint = 0xf3785020;
pub const FSTV0910_P2_TSINS_SIGPCOUNT: c_uint = 0xf3784010;
pub const FSTV0910_P2_TSINS_FIFO: c_uint = 0xf3783008;
pub const FSTV0910_P2_TSINS_REALPACK: c_uint = 0xf3782004;
pub const FSTV0910_P2_TSINS_TSCONFIG: c_uint = 0xf3781002;
pub const FSTV0910_P2_TSINS_LATENCY: c_uint = 0xf3780001;
// P2_TSDIVN
pub const RSTV0910_P2_TSDIVN: c_uint = 0xf379;
pub const FSTV0910_P2_TSFIFO_SPEEDMODE: c_uint = 0xf37960c0;
pub const FSTV0910_P2_TSFIFO_RISEOK: c_uint = 0xf3790007;
// P2_TSCFG4
pub const RSTV0910_P2_TSCFG4: c_uint = 0xf37a;
pub const FSTV0910_P2_TSFIFO_TSSPEEDMODE: c_uint = 0xf37a60c0;
// P2_TSSPEED
pub const RSTV0910_P2_TSSPEED: c_uint = 0xf380;
pub const FSTV0910_P2_TSFIFO_OUTSPEED: c_uint = 0xf38000ff;
// P2_TSSTATUS
pub const RSTV0910_P2_TSSTATUS: c_uint = 0xf381;
pub const FSTV0910_P2_TSFIFO_LINEOK: c_uint = 0xf3817080;
pub const FSTV0910_P2_TSFIFO_ERROR: c_uint = 0xf3816040;
pub const FSTV0910_P2_TSFIFO_NOSYNC: c_uint = 0xf3814010;
pub const FSTV0910_P2_TSREGUL_ERROR: c_uint = 0xf3812004;
pub const FSTV0910_P2_DIL_READY: c_uint = 0xf3810001;
// P2_TSSTATUS2
pub const RSTV0910_P2_TSSTATUS2: c_uint = 0xf382;
pub const FSTV0910_P2_TSFIFO_DEMODSEL: c_uint = 0xf3827080;
pub const FSTV0910_P2_TSFIFOSPEED_STORE: c_uint = 0xf3826040;
pub const FSTV0910_P2_DILXX_RESET: c_uint = 0xf3825020;
pub const FSTV0910_P2_SCRAMBDETECT: c_uint = 0xf3821002;
// P2_TSBITRATE1
pub const RSTV0910_P2_TSBITRATE1: c_uint = 0xf383;
pub const FSTV0910_P2_TSFIFO_BITRATE1: c_uint = 0xf38300ff;
// P2_TSBITRATE0
pub const RSTV0910_P2_TSBITRATE0: c_uint = 0xf384;
pub const FSTV0910_P2_TSFIFO_BITRATE0: c_uint = 0xf38400ff;
// P2_TSPACKLEN1
pub const RSTV0910_P2_TSPACKLEN1: c_uint = 0xf385;
pub const FSTV0910_P2_TSFIFO_PACKCPT: c_uint = 0xf38550e0;
// P2_TSDLY2
pub const RSTV0910_P2_TSDLY2: c_uint = 0xf389;
pub const FSTV0910_P2_SOFFIFO_LATENCY2: c_uint = 0xf389000f;
// P2_TSDLY1
pub const RSTV0910_P2_TSDLY1: c_uint = 0xf38a;
pub const FSTV0910_P2_SOFFIFO_LATENCY1: c_uint = 0xf38a00ff;
// P2_TSDLY0
pub const RSTV0910_P2_TSDLY0: c_uint = 0xf38b;
pub const FSTV0910_P2_SOFFIFO_LATENCY0: c_uint = 0xf38b00ff;
// P2_TSNPDAV
pub const RSTV0910_P2_TSNPDAV: c_uint = 0xf38c;
pub const FSTV0910_P2_TSNPD_AVERAGE: c_uint = 0xf38c00ff;
// P2_TSBUFSTAT2
pub const RSTV0910_P2_TSBUFSTAT2: c_uint = 0xf38d;
pub const FSTV0910_P2_TSISCR_3BYTES: c_uint = 0xf38d7080;
pub const FSTV0910_P2_TSISCR_NEWDATA: c_uint = 0xf38d6040;
pub const FSTV0910_P2_TSISCR_BUFSTAT2: c_uint = 0xf38d003f;
// P2_TSBUFSTAT1
pub const RSTV0910_P2_TSBUFSTAT1: c_uint = 0xf38e;
pub const FSTV0910_P2_TSISCR_BUFSTAT1: c_uint = 0xf38e00ff;
// P2_TSBUFSTAT0
pub const RSTV0910_P2_TSBUFSTAT0: c_uint = 0xf38f;
pub const FSTV0910_P2_TSISCR_BUFSTAT0: c_uint = 0xf38f00ff;
// P2_TSDEBUGL
pub const RSTV0910_P2_TSDEBUGL: c_uint = 0xf391;
pub const FSTV0910_P2_TSFIFO_ERROR_EVNT: c_uint = 0xf3912004;
pub const FSTV0910_P2_TSFIFO_OVERFLOWM: c_uint = 0xf3910001;
// P2_TSDLYSET2
pub const RSTV0910_P2_TSDLYSET2: c_uint = 0xf392;
pub const FSTV0910_P2_SOFFIFO_OFFSET: c_uint = 0xf39260c0;
pub const FSTV0910_P2_HYSTERESIS_THRESHOLD: c_uint = 0xf3924030;
pub const FSTV0910_P2_SOFFIFO_SYMBOFFS2: c_uint = 0xf392000f;
// P2_TSDLYSET1
pub const RSTV0910_P2_TSDLYSET1: c_uint = 0xf393;
pub const FSTV0910_P2_SOFFIFO_SYMBOFFS1: c_uint = 0xf39300ff;
// P2_TSDLYSET0
pub const RSTV0910_P2_TSDLYSET0: c_uint = 0xf394;
pub const FSTV0910_P2_SOFFIFO_SYMBOFFS0: c_uint = 0xf39400ff;
// P2_ERRCTRL1
pub const RSTV0910_P2_ERRCTRL1: c_uint = 0xf398;
pub const FSTV0910_P2_ERR_SOURCE1: c_uint = 0xf39840f0;
pub const FSTV0910_P2_NUM_EVENT1: c_uint = 0xf3980007;
// P2_ERRCNT12
pub const RSTV0910_P2_ERRCNT12: c_uint = 0xf399;
pub const FSTV0910_P2_ERRCNT1_OLDVALUE: c_uint = 0xf3997080;
pub const FSTV0910_P2_ERR_CNT12: c_uint = 0xf399007f;
// P2_ERRCNT11
pub const RSTV0910_P2_ERRCNT11: c_uint = 0xf39a;
pub const FSTV0910_P2_ERR_CNT11: c_uint = 0xf39a00ff;
// P2_ERRCNT10
pub const RSTV0910_P2_ERRCNT10: c_uint = 0xf39b;
pub const FSTV0910_P2_ERR_CNT10: c_uint = 0xf39b00ff;
// P2_ERRCTRL2
pub const RSTV0910_P2_ERRCTRL2: c_uint = 0xf39c;
pub const FSTV0910_P2_ERR_SOURCE2: c_uint = 0xf39c40f0;
pub const FSTV0910_P2_NUM_EVENT2: c_uint = 0xf39c0007;
// P2_ERRCNT22
pub const RSTV0910_P2_ERRCNT22: c_uint = 0xf39d;
pub const FSTV0910_P2_ERRCNT2_OLDVALUE: c_uint = 0xf39d7080;
pub const FSTV0910_P2_ERR_CNT22: c_uint = 0xf39d007f;
// P2_ERRCNT21
pub const RSTV0910_P2_ERRCNT21: c_uint = 0xf39e;
pub const FSTV0910_P2_ERR_CNT21: c_uint = 0xf39e00ff;
// P2_ERRCNT20
pub const RSTV0910_P2_ERRCNT20: c_uint = 0xf39f;
pub const FSTV0910_P2_ERR_CNT20: c_uint = 0xf39f00ff;
// P2_FECSPY
pub const RSTV0910_P2_FECSPY: c_uint = 0xf3a0;
pub const FSTV0910_P2_SPY_ENABLE: c_uint = 0xf3a07080;
pub const FSTV0910_P2_NO_SYNCBYTE: c_uint = 0xf3a06040;
pub const FSTV0910_P2_SERIAL_MODE: c_uint = 0xf3a05020;
pub const FSTV0910_P2_UNUSUAL_PACKET: c_uint = 0xf3a04010;
pub const FSTV0910_P2_BERMETER_DATAMODE: c_uint = 0xf3a0200c;
pub const FSTV0910_P2_BERMETER_LMODE: c_uint = 0xf3a01002;
pub const FSTV0910_P2_BERMETER_RESET: c_uint = 0xf3a00001;
// P2_FSPYCFG
pub const RSTV0910_P2_FSPYCFG: c_uint = 0xf3a1;
pub const FSTV0910_P2_FECSPY_INPUT: c_uint = 0xf3a160c0;
pub const FSTV0910_P2_RST_ON_ERROR: c_uint = 0xf3a15020;
pub const FSTV0910_P2_ONE_SHOT: c_uint = 0xf3a14010;
pub const FSTV0910_P2_I2C_MODE: c_uint = 0xf3a1200c;
pub const FSTV0910_P2_SPY_HYSTERESIS: c_uint = 0xf3a10003;
// P2_FSPYDATA
pub const RSTV0910_P2_FSPYDATA: c_uint = 0xf3a2;
pub const FSTV0910_P2_SPY_STUFFING: c_uint = 0xf3a27080;
pub const FSTV0910_P2_SPY_CNULLPKT: c_uint = 0xf3a25020;
pub const FSTV0910_P2_SPY_OUTDATA_MODE: c_uint = 0xf3a2001f;
// P2_FSPYOUT
pub const RSTV0910_P2_FSPYOUT: c_uint = 0xf3a3;
pub const FSTV0910_P2_FSPY_DIRECT: c_uint = 0xf3a37080;
pub const FSTV0910_P2_STUFF_MODE: c_uint = 0xf3a30007;
// P2_FSTATUS
pub const RSTV0910_P2_FSTATUS: c_uint = 0xf3a4;
pub const FSTV0910_P2_SPY_ENDSIM: c_uint = 0xf3a47080;
pub const FSTV0910_P2_VALID_SIM: c_uint = 0xf3a46040;
pub const FSTV0910_P2_FOUND_SIGNAL: c_uint = 0xf3a45020;
pub const FSTV0910_P2_DSS_SYNCBYTE: c_uint = 0xf3a44010;
pub const FSTV0910_P2_RESULT_STATE: c_uint = 0xf3a4000f;
// P2_FBERCPT4
pub const RSTV0910_P2_FBERCPT4: c_uint = 0xf3a8;
pub const FSTV0910_P2_FBERMETER_CPT4: c_uint = 0xf3a800ff;
// P2_FBERCPT3
pub const RSTV0910_P2_FBERCPT3: c_uint = 0xf3a9;
pub const FSTV0910_P2_FBERMETER_CPT3: c_uint = 0xf3a900ff;
// P2_FBERCPT2
pub const RSTV0910_P2_FBERCPT2: c_uint = 0xf3aa;
pub const FSTV0910_P2_FBERMETER_CPT2: c_uint = 0xf3aa00ff;
// P2_FBERCPT1
pub const RSTV0910_P2_FBERCPT1: c_uint = 0xf3ab;
pub const FSTV0910_P2_FBERMETER_CPT1: c_uint = 0xf3ab00ff;
// P2_FBERCPT0
pub const RSTV0910_P2_FBERCPT0: c_uint = 0xf3ac;
pub const FSTV0910_P2_FBERMETER_CPT0: c_uint = 0xf3ac00ff;
// P2_FBERERR2
pub const RSTV0910_P2_FBERERR2: c_uint = 0xf3ad;
pub const FSTV0910_P2_FBERMETER_ERR2: c_uint = 0xf3ad00ff;
// P2_FBERERR1
pub const RSTV0910_P2_FBERERR1: c_uint = 0xf3ae;
pub const FSTV0910_P2_FBERMETER_ERR1: c_uint = 0xf3ae00ff;
// P2_FBERERR0
pub const RSTV0910_P2_FBERERR0: c_uint = 0xf3af;
pub const FSTV0910_P2_FBERMETER_ERR0: c_uint = 0xf3af00ff;
// P2_FSPYBER
pub const RSTV0910_P2_FSPYBER: c_uint = 0xf3b2;
pub const FSTV0910_P2_FSPYBER_SYNCBYTE: c_uint = 0xf3b24010;
pub const FSTV0910_P2_FSPYBER_UNSYNC: c_uint = 0xf3b23008;
pub const FSTV0910_P2_FSPYBER_CTIME: c_uint = 0xf3b20007;
// P2_SFERROR
pub const RSTV0910_P2_SFERROR: c_uint = 0xf3c1;
pub const FSTV0910_P2_SFEC_REGERR_VIT: c_uint = 0xf3c100ff;
// P2_SFECSTATUS
pub const RSTV0910_P2_SFECSTATUS: c_uint = 0xf3c3;
pub const FSTV0910_P2_SFEC_ON: c_uint = 0xf3c37080;
pub const FSTV0910_P2_SFEC_OFF: c_uint = 0xf3c36040;
pub const FSTV0910_P2_LOCKEDSFEC: c_uint = 0xf3c33008;
pub const FSTV0910_P2_SFEC_DELOCK: c_uint = 0xf3c32004;
pub const FSTV0910_P2_SFEC_DEMODSEL: c_uint = 0xf3c31002;
pub const FSTV0910_P2_SFEC_OVFON: c_uint = 0xf3c30001;
// P2_SFKDIV12
pub const RSTV0910_P2_SFKDIV12: c_uint = 0xf3c4;
pub const FSTV0910_P2_SFECKDIV12_MAN: c_uint = 0xf3c47080;
// P2_SFKDIV23
pub const RSTV0910_P2_SFKDIV23: c_uint = 0xf3c5;
pub const FSTV0910_P2_SFECKDIV23_MAN: c_uint = 0xf3c57080;
// P2_SFKDIV34
pub const RSTV0910_P2_SFKDIV34: c_uint = 0xf3c6;
pub const FSTV0910_P2_SFECKDIV34_MAN: c_uint = 0xf3c67080;
// P2_SFKDIV56
pub const RSTV0910_P2_SFKDIV56: c_uint = 0xf3c7;
pub const FSTV0910_P2_SFECKDIV56_MAN: c_uint = 0xf3c77080;
// P2_SFKDIV67
pub const RSTV0910_P2_SFKDIV67: c_uint = 0xf3c8;
pub const FSTV0910_P2_SFECKDIV67_MAN: c_uint = 0xf3c87080;
// P2_SFKDIV78
pub const RSTV0910_P2_SFKDIV78: c_uint = 0xf3c9;
pub const FSTV0910_P2_SFECKDIV78_MAN: c_uint = 0xf3c97080;
// P2_SFSTATUS
pub const RSTV0910_P2_SFSTATUS: c_uint = 0xf3cc;
pub const FSTV0910_P2_SFEC_LINEOK: c_uint = 0xf3cc7080;
pub const FSTV0910_P2_SFEC_ERROR: c_uint = 0xf3cc6040;
pub const FSTV0910_P2_SFEC_DATA7: c_uint = 0xf3cc5020;
pub const FSTV0910_P2_SFEC_PKTDNBRFAIL: c_uint = 0xf3cc4010;
pub const FSTV0910_P2_TSSFEC_DEMODSEL: c_uint = 0xf3cc3008;
pub const FSTV0910_P2_SFEC_NOSYNC: c_uint = 0xf3cc2004;
pub const FSTV0910_P2_SFEC_UNREGULA: c_uint = 0xf3cc1002;
pub const FSTV0910_P2_SFEC_READY: c_uint = 0xf3cc0001;
// P2_SFDLYSET2
pub const RSTV0910_P2_SFDLYSET2: c_uint = 0xf3d0;
pub const FSTV0910_P2_SFEC_DISABLE: c_uint = 0xf3d01002;
// P2_SFERRCTRL
pub const RSTV0910_P2_SFERRCTRL: c_uint = 0xf3d8;
pub const FSTV0910_P2_SFEC_ERR_SOURCE: c_uint = 0xf3d840f0;
pub const FSTV0910_P2_SFEC_NUM_EVENT: c_uint = 0xf3d80007;
// P2_SFERRCNT2
pub const RSTV0910_P2_SFERRCNT2: c_uint = 0xf3d9;
pub const FSTV0910_P2_SFERRC_OLDVALUE: c_uint = 0xf3d97080;
pub const FSTV0910_P2_SFEC_ERR_CNT2: c_uint = 0xf3d9007f;
// P2_SFERRCNT1
pub const RSTV0910_P2_SFERRCNT1: c_uint = 0xf3da;
pub const FSTV0910_P2_SFEC_ERR_CNT1: c_uint = 0xf3da00ff;
// P2_SFERRCNT0
pub const RSTV0910_P2_SFERRCNT0: c_uint = 0xf3db;
pub const FSTV0910_P2_SFEC_ERR_CNT0: c_uint = 0xf3db00ff;
// P1_IQCONST
pub const RSTV0910_P1_IQCONST: c_uint = 0xf400;
pub const FSTV0910_P1_CONSTEL_SELECT: c_uint = 0xf4005060;
pub const FSTV0910_P1_IQSYMB_SEL: c_uint = 0xf400001f;
// P1_NOSCFG
pub const RSTV0910_P1_NOSCFG: c_uint = 0xf401;
pub const FSTV0910_P1_DUMMYPL_NOSDATA: c_uint = 0xf4015020;
pub const FSTV0910_P1_NOSPLH_BETA: c_uint = 0xf4013018;
pub const FSTV0910_P1_NOSDATA_BETA: c_uint = 0xf4010007;
// P1_ISYMB
pub const RSTV0910_P1_ISYMB: c_uint = 0xf402;
pub const FSTV0910_P1_I_SYMBOL: c_uint = 0xf40201ff;
// P1_QSYMB
pub const RSTV0910_P1_QSYMB: c_uint = 0xf403;
pub const FSTV0910_P1_Q_SYMBOL: c_uint = 0xf40301ff;
// P1_AGC1CFG
pub const RSTV0910_P1_AGC1CFG: c_uint = 0xf404;
pub const FSTV0910_P1_DC_FROZEN: c_uint = 0xf4047080;
pub const FSTV0910_P1_DC_CORRECT: c_uint = 0xf4046040;
pub const FSTV0910_P1_AMM_FROZEN: c_uint = 0xf4045020;
pub const FSTV0910_P1_AMM_CORRECT: c_uint = 0xf4044010;
pub const FSTV0910_P1_QUAD_FROZEN: c_uint = 0xf4043008;
pub const FSTV0910_P1_QUAD_CORRECT: c_uint = 0xf4042004;
// P1_AGC1CN
pub const RSTV0910_P1_AGC1CN: c_uint = 0xf406;
pub const FSTV0910_P1_AGC1_LOCKED: c_uint = 0xf4067080;
pub const FSTV0910_P1_AGC1_MINPOWER: c_uint = 0xf4064010;
pub const FSTV0910_P1_AGCOUT_FAST: c_uint = 0xf4063008;
pub const FSTV0910_P1_AGCIQ_BETA: c_uint = 0xf4060007;
// P1_AGC1REF
pub const RSTV0910_P1_AGC1REF: c_uint = 0xf407;
pub const FSTV0910_P1_AGCIQ_REF: c_uint = 0xf40700ff;
// P1_IDCCOMP
pub const RSTV0910_P1_IDCCOMP: c_uint = 0xf408;
pub const FSTV0910_P1_IAVERAGE_ADJ: c_uint = 0xf40801ff;
// P1_QDCCOMP
pub const RSTV0910_P1_QDCCOMP: c_uint = 0xf409;
pub const FSTV0910_P1_QAVERAGE_ADJ: c_uint = 0xf40901ff;
// P1_POWERI
pub const RSTV0910_P1_POWERI: c_uint = 0xf40a;
pub const FSTV0910_P1_POWER_I: c_uint = 0xf40a00ff;
// P1_POWERQ
pub const RSTV0910_P1_POWERQ: c_uint = 0xf40b;
pub const FSTV0910_P1_POWER_Q: c_uint = 0xf40b00ff;
// P1_AGC1AMM
pub const RSTV0910_P1_AGC1AMM: c_uint = 0xf40c;
pub const FSTV0910_P1_AMM_VALUE: c_uint = 0xf40c00ff;
// P1_AGC1QUAD
pub const RSTV0910_P1_AGC1QUAD: c_uint = 0xf40d;
pub const FSTV0910_P1_QUAD_VALUE: c_uint = 0xf40d01ff;
// P1_AGCIQIN1
pub const RSTV0910_P1_AGCIQIN1: c_uint = 0xf40e;
pub const FSTV0910_P1_AGCIQ_VALUE1: c_uint = 0xf40e00ff;
// P1_AGCIQIN0
pub const RSTV0910_P1_AGCIQIN0: c_uint = 0xf40f;
pub const FSTV0910_P1_AGCIQ_VALUE0: c_uint = 0xf40f00ff;
// P1_DEMOD
pub const RSTV0910_P1_DEMOD: c_uint = 0xf410;
pub const FSTV0910_P1_MANUALS2_ROLLOFF: c_uint = 0xf4107080;
pub const FSTV0910_P1_SPECINV_CONTROL: c_uint = 0xf4104030;
pub const FSTV0910_P1_MANUALSX_ROLLOFF: c_uint = 0xf4102004;
pub const FSTV0910_P1_ROLLOFF_CONTROL: c_uint = 0xf4100003;
// P1_DMDMODCOD
pub const RSTV0910_P1_DMDMODCOD: c_uint = 0xf411;
pub const FSTV0910_P1_MANUAL_MODCOD: c_uint = 0xf4117080;
pub const FSTV0910_P1_DEMOD_MODCOD: c_uint = 0xf411207c;
pub const FSTV0910_P1_DEMOD_TYPE: c_uint = 0xf4110003;
// P1_DSTATUS
pub const RSTV0910_P1_DSTATUS: c_uint = 0xf412;
pub const FSTV0910_P1_CAR_LOCK: c_uint = 0xf4127080;
pub const FSTV0910_P1_TMGLOCK_QUALITY: c_uint = 0xf4125060;
pub const FSTV0910_P1_LOCK_DEFINITIF: c_uint = 0xf4123008;
pub const FSTV0910_P1_OVADC_DETECT: c_uint = 0xf4120001;
// P1_DSTATUS2
pub const RSTV0910_P1_DSTATUS2: c_uint = 0xf413;
pub const FSTV0910_P1_DEMOD_DELOCK: c_uint = 0xf4137080;
pub const FSTV0910_P1_MODCODRQ_SYNCTAG: c_uint = 0xf4135020;
pub const FSTV0910_P1_POLYPH_SATEVENT: c_uint = 0xf4134010;
pub const FSTV0910_P1_AGC1_NOSIGNALACK: c_uint = 0xf4133008;
pub const FSTV0910_P1_AGC2_OVERFLOW: c_uint = 0xf4132004;
pub const FSTV0910_P1_CFR_OVERFLOW: c_uint = 0xf4131002;
pub const FSTV0910_P1_GAMMA_OVERUNDER: c_uint = 0xf4130001;
// P1_DMDCFGMD
pub const RSTV0910_P1_DMDCFGMD: c_uint = 0xf414;
pub const FSTV0910_P1_DVBS2_ENABLE: c_uint = 0xf4147080;
pub const FSTV0910_P1_DVBS1_ENABLE: c_uint = 0xf4146040;
pub const FSTV0910_P1_SCAN_ENABLE: c_uint = 0xf4144010;
pub const FSTV0910_P1_CFR_AUTOSCAN: c_uint = 0xf4143008;
pub const FSTV0910_P1_TUN_RNG: c_uint = 0xf4140003;
// P1_DMDCFG2
pub const RSTV0910_P1_DMDCFG2: c_uint = 0xf415;
pub const FSTV0910_P1_S1S2_SEQUENTIAL: c_uint = 0xf4156040;
pub const FSTV0910_P1_INFINITE_RELOCK: c_uint = 0xf4154010;
// P1_DMDISTATE
pub const RSTV0910_P1_DMDISTATE: c_uint = 0xf416;
pub const FSTV0910_P1_I2C_NORESETDMODE: c_uint = 0xf4167080;
pub const FSTV0910_P1_I2C_DEMOD_MODE: c_uint = 0xf416001f;
// P1_DMDT0M
pub const RSTV0910_P1_DMDT0M: c_uint = 0xf417;
pub const FSTV0910_P1_DMDT0_MIN: c_uint = 0xf41700ff;
// P1_DMDSTATE
pub const RSTV0910_P1_DMDSTATE: c_uint = 0xf41b;
pub const FSTV0910_P1_HEADER_MODE: c_uint = 0xf41b5060;
// P1_DMDFLYW
pub const RSTV0910_P1_DMDFLYW: c_uint = 0xf41c;
pub const FSTV0910_P1_I2C_IRQVAL: c_uint = 0xf41c40f0;
pub const FSTV0910_P1_FLYWHEEL_CPT: c_uint = 0xf41c000f;
// P1_DSTATUS3
pub const RSTV0910_P1_DSTATUS3: c_uint = 0xf41d;
pub const FSTV0910_P1_CFR_ZIGZAG: c_uint = 0xf41d7080;
pub const FSTV0910_P1_DEMOD_CFGMODE: c_uint = 0xf41d5060;
pub const FSTV0910_P1_GAMMA_LOWBAUDRATE: c_uint = 0xf41d4010;
// P1_DMDCFG3
pub const RSTV0910_P1_DMDCFG3: c_uint = 0xf41e;
pub const FSTV0910_P1_NOSTOP_FIFOFULL: c_uint = 0xf41e3008;
// P1_DMDCFG4
pub const RSTV0910_P1_DMDCFG4: c_uint = 0xf41f;
pub const FSTV0910_P1_DIS_VITLOCK: c_uint = 0xf41f7080;
pub const FSTV0910_P1_DIS_CLKENABLE: c_uint = 0xf41f2004;
// P1_CORRELMANT
pub const RSTV0910_P1_CORRELMANT: c_uint = 0xf420;
pub const FSTV0910_P1_CORREL_MANT: c_uint = 0xf42000ff;
// P1_CORRELABS
pub const RSTV0910_P1_CORRELABS: c_uint = 0xf421;
pub const FSTV0910_P1_CORREL_ABS: c_uint = 0xf42100ff;
// P1_CORRELEXP
pub const RSTV0910_P1_CORRELEXP: c_uint = 0xf422;
pub const FSTV0910_P1_CORREL_ABSEXP: c_uint = 0xf42240f0;
pub const FSTV0910_P1_CORREL_EXP: c_uint = 0xf422000f;
// P1_PLHMODCOD
pub const RSTV0910_P1_PLHMODCOD: c_uint = 0xf424;
pub const FSTV0910_P1_SPECINV_DEMOD: c_uint = 0xf4247080;
pub const FSTV0910_P1_PLH_MODCOD: c_uint = 0xf424207c;
pub const FSTV0910_P1_PLH_TYPE: c_uint = 0xf4240003;
// P1_DMDREG
pub const RSTV0910_P1_DMDREG: c_uint = 0xf425;
pub const FSTV0910_P1_DECIM_PLFRAMES: c_uint = 0xf4250001;
// P1_AGCNADJ
pub const RSTV0910_P1_AGCNADJ: c_uint = 0xf426;
pub const FSTV0910_P1_RADJOFF_AGC2: c_uint = 0xf4267080;
pub const FSTV0910_P1_RADJOFF_AGC1: c_uint = 0xf4266040;
pub const FSTV0910_P1_AGC_NADJ: c_uint = 0xf426013f;
// P1_AGCKS
pub const RSTV0910_P1_AGCKS: c_uint = 0xf427;
pub const FSTV0910_P1_RSADJ_MANUALCFG: c_uint = 0xf4277080;
pub const FSTV0910_P1_RSADJ_CCMMODE: c_uint = 0xf4276040;
pub const FSTV0910_P1_RADJ_SPSK: c_uint = 0xf427013f;
// P1_AGCKQ
pub const RSTV0910_P1_AGCKQ: c_uint = 0xf428;
pub const FSTV0910_P1_RADJON_DVBS1: c_uint = 0xf4286040;
pub const FSTV0910_P1_RADJ_QPSK: c_uint = 0xf428013f;
// P1_AGCK8
pub const RSTV0910_P1_AGCK8: c_uint = 0xf429;
pub const FSTV0910_P1_RADJ_8PSK: c_uint = 0xf429013f;
// P1_AGCK16
pub const RSTV0910_P1_AGCK16: c_uint = 0xf42a;
pub const FSTV0910_P1_R2ADJOFF_16APSK: c_uint = 0xf42a6040;
pub const FSTV0910_P1_R1ADJOFF_16APSK: c_uint = 0xf42a5020;
pub const FSTV0910_P1_RADJ_16APSK: c_uint = 0xf42a011f;
// P1_AGCK32
pub const RSTV0910_P1_AGCK32: c_uint = 0xf42b;
pub const FSTV0910_P1_R3ADJOFF_32APSK: c_uint = 0xf42b7080;
pub const FSTV0910_P1_R2ADJOFF_32APSK: c_uint = 0xf42b6040;
pub const FSTV0910_P1_R1ADJOFF_32APSK: c_uint = 0xf42b5020;
pub const FSTV0910_P1_RADJ_32APSK: c_uint = 0xf42b011f;
// P1_AGC2O
pub const RSTV0910_P1_AGC2O: c_uint = 0xf42c;
pub const FSTV0910_P1_CSTENV_MODE: c_uint = 0xf42c60c0;
pub const FSTV0910_P1_AGC2_COEF: c_uint = 0xf42c0007;
// P1_AGC2REF
pub const RSTV0910_P1_AGC2REF: c_uint = 0xf42d;
pub const FSTV0910_P1_AGC2_REF: c_uint = 0xf42d00ff;
// P1_AGC1ADJ
pub const RSTV0910_P1_AGC1ADJ: c_uint = 0xf42e;
pub const FSTV0910_P1_AGC1_ADJUSTED: c_uint = 0xf42e007f;
// P1_AGCRSADJ
pub const RSTV0910_P1_AGCRSADJ: c_uint = 0xf42f;
pub const FSTV0910_P1_RS_ADJUSTED: c_uint = 0xf42f007f;
// P1_AGCRQADJ
pub const RSTV0910_P1_AGCRQADJ: c_uint = 0xf430;
pub const FSTV0910_P1_RQ_ADJUSTED: c_uint = 0xf430007f;
// P1_AGCR8ADJ
pub const RSTV0910_P1_AGCR8ADJ: c_uint = 0xf431;
pub const FSTV0910_P1_R8_ADJUSTED: c_uint = 0xf431007f;
// P1_AGCR1ADJ
pub const RSTV0910_P1_AGCR1ADJ: c_uint = 0xf432;
pub const FSTV0910_P1_R1_ADJUSTED: c_uint = 0xf432007f;
// P1_AGCR2ADJ
pub const RSTV0910_P1_AGCR2ADJ: c_uint = 0xf433;
pub const FSTV0910_P1_R2_ADJUSTED: c_uint = 0xf433007f;
// P1_AGCR3ADJ
pub const RSTV0910_P1_AGCR3ADJ: c_uint = 0xf434;
pub const FSTV0910_P1_R3_ADJUSTED: c_uint = 0xf434007f;
// P1_AGCREFADJ
pub const RSTV0910_P1_AGCREFADJ: c_uint = 0xf435;
pub const FSTV0910_P1_AGC2REF_ADJUSTED: c_uint = 0xf435007f;
// P1_AGC2I1
pub const RSTV0910_P1_AGC2I1: c_uint = 0xf436;
pub const FSTV0910_P1_AGC2_INTEGRATOR1: c_uint = 0xf43600ff;
// P1_AGC2I0
pub const RSTV0910_P1_AGC2I0: c_uint = 0xf437;
pub const FSTV0910_P1_AGC2_INTEGRATOR0: c_uint = 0xf43700ff;
// P1_CARCFG
pub const RSTV0910_P1_CARCFG: c_uint = 0xf438;
pub const FSTV0910_P1_ROTAON: c_uint = 0xf4382004;
pub const FSTV0910_P1_PH_DET_ALGO: c_uint = 0xf4380003;
// P1_ACLC
pub const RSTV0910_P1_ACLC: c_uint = 0xf439;
pub const FSTV0910_P1_CAR_ALPHA_MANT: c_uint = 0xf4394030;
pub const FSTV0910_P1_CAR_ALPHA_EXP: c_uint = 0xf439000f;
// P1_BCLC
pub const RSTV0910_P1_BCLC: c_uint = 0xf43a;
pub const FSTV0910_P1_CAR_BETA_MANT: c_uint = 0xf43a4030;
pub const FSTV0910_P1_CAR_BETA_EXP: c_uint = 0xf43a000f;
// P1_ACLCS2
pub const RSTV0910_P1_ACLCS2: c_uint = 0xf43b;
pub const FSTV0910_P1_CARS2_APLHA_MANTISSE: c_uint = 0xf43b4030;
pub const FSTV0910_P1_CARS2_ALPHA_EXP: c_uint = 0xf43b000f;
// P1_BCLCS2
pub const RSTV0910_P1_BCLCS2: c_uint = 0xf43c;
pub const FSTV0910_P1_CARS2_BETA_MANTISSE: c_uint = 0xf43c4030;
pub const FSTV0910_P1_CARS2_BETA_EXP: c_uint = 0xf43c000f;
// P1_CARFREQ
pub const RSTV0910_P1_CARFREQ: c_uint = 0xf43d;
pub const FSTV0910_P1_KC_COARSE_EXP: c_uint = 0xf43d40f0;
pub const FSTV0910_P1_BETA_FREQ: c_uint = 0xf43d000f;
// P1_CARHDR
pub const RSTV0910_P1_CARHDR: c_uint = 0xf43e;
pub const FSTV0910_P1_K_FREQ_HDR: c_uint = 0xf43e00ff;
// P1_LDT
pub const RSTV0910_P1_LDT: c_uint = 0xf43f;
pub const FSTV0910_P1_CARLOCK_THRES: c_uint = 0xf43f01ff;
// P1_LDT2
pub const RSTV0910_P1_LDT2: c_uint = 0xf440;
pub const FSTV0910_P1_CARLOCK_THRES2: c_uint = 0xf44001ff;
// P1_CFRICFG
pub const RSTV0910_P1_CFRICFG: c_uint = 0xf441;
pub const FSTV0910_P1_NEG_CFRSTEP: c_uint = 0xf4410001;
// P1_CFRUP1
pub const RSTV0910_P1_CFRUP1: c_uint = 0xf442;
pub const FSTV0910_P1_CFR_UP1: c_uint = 0xf44201ff;
// P1_CFRUP0
pub const RSTV0910_P1_CFRUP0: c_uint = 0xf443;
pub const FSTV0910_P1_CFR_UP0: c_uint = 0xf44300ff;
// P1_CFRIBASE1
pub const RSTV0910_P1_CFRIBASE1: c_uint = 0xf444;
pub const FSTV0910_P1_CFRINIT_BASE1: c_uint = 0xf44400ff;
// P1_CFRIBASE0
pub const RSTV0910_P1_CFRIBASE0: c_uint = 0xf445;
pub const FSTV0910_P1_CFRINIT_BASE0: c_uint = 0xf44500ff;
// P1_CFRLOW1
pub const RSTV0910_P1_CFRLOW1: c_uint = 0xf446;
pub const FSTV0910_P1_CFR_LOW1: c_uint = 0xf44601ff;
// P1_CFRLOW0
pub const RSTV0910_P1_CFRLOW0: c_uint = 0xf447;
pub const FSTV0910_P1_CFR_LOW0: c_uint = 0xf44700ff;
// P1_CFRINIT1
pub const RSTV0910_P1_CFRINIT1: c_uint = 0xf448;
pub const FSTV0910_P1_CFR_INIT1: c_uint = 0xf44801ff;
// P1_CFRINIT0
pub const RSTV0910_P1_CFRINIT0: c_uint = 0xf449;
pub const FSTV0910_P1_CFR_INIT0: c_uint = 0xf44900ff;
// P1_CFRINC1
pub const RSTV0910_P1_CFRINC1: c_uint = 0xf44a;
pub const FSTV0910_P1_MANUAL_CFRINC: c_uint = 0xf44a7080;
pub const FSTV0910_P1_CFR_INC1: c_uint = 0xf44a003f;
// P1_CFRINC0
pub const RSTV0910_P1_CFRINC0: c_uint = 0xf44b;
pub const FSTV0910_P1_CFR_INC0: c_uint = 0xf44b00ff;
// P1_CFR2
pub const RSTV0910_P1_CFR2: c_uint = 0xf44c;
pub const FSTV0910_P1_CAR_FREQ2: c_uint = 0xf44c01ff;
// P1_CFR1
pub const RSTV0910_P1_CFR1: c_uint = 0xf44d;
pub const FSTV0910_P1_CAR_FREQ1: c_uint = 0xf44d00ff;
// P1_CFR0
pub const RSTV0910_P1_CFR0: c_uint = 0xf44e;
pub const FSTV0910_P1_CAR_FREQ0: c_uint = 0xf44e00ff;
// P1_LDI
pub const RSTV0910_P1_LDI: c_uint = 0xf44f;
pub const FSTV0910_P1_LOCK_DET_INTEGR: c_uint = 0xf44f01ff;
// P1_TMGCFG
pub const RSTV0910_P1_TMGCFG: c_uint = 0xf450;
pub const FSTV0910_P1_TMGLOCK_BETA: c_uint = 0xf45060c0;
pub const FSTV0910_P1_DO_TIMING_CORR: c_uint = 0xf4504010;
pub const FSTV0910_P1_TMG_MINFREQ: c_uint = 0xf4500003;
// P1_RTC
pub const RSTV0910_P1_RTC: c_uint = 0xf451;
pub const FSTV0910_P1_TMGALPHA_EXP: c_uint = 0xf45140f0;
pub const FSTV0910_P1_TMGBETA_EXP: c_uint = 0xf451000f;
// P1_RTCS2
pub const RSTV0910_P1_RTCS2: c_uint = 0xf452;
pub const FSTV0910_P1_TMGALPHAS2_EXP: c_uint = 0xf45240f0;
pub const FSTV0910_P1_TMGBETAS2_EXP: c_uint = 0xf452000f;
// P1_TMGTHRISE
pub const RSTV0910_P1_TMGTHRISE: c_uint = 0xf453;
pub const FSTV0910_P1_TMGLOCK_THRISE: c_uint = 0xf45300ff;
// P1_TMGTHFALL
pub const RSTV0910_P1_TMGTHFALL: c_uint = 0xf454;
pub const FSTV0910_P1_TMGLOCK_THFALL: c_uint = 0xf45400ff;
// P1_SFRUPRATIO
pub const RSTV0910_P1_SFRUPRATIO: c_uint = 0xf455;
pub const FSTV0910_P1_SFR_UPRATIO: c_uint = 0xf45500ff;
// P1_SFRLOWRATIO
pub const RSTV0910_P1_SFRLOWRATIO: c_uint = 0xf456;
pub const FSTV0910_P1_SFR_LOWRATIO: c_uint = 0xf45600ff;
// P1_KTTMG
pub const RSTV0910_P1_KTTMG: c_uint = 0xf457;
pub const FSTV0910_P1_KT_TMG_EXP: c_uint = 0xf45740f0;
// P1_KREFTMG
pub const RSTV0910_P1_KREFTMG: c_uint = 0xf458;
pub const FSTV0910_P1_KREF_TMG: c_uint = 0xf45800ff;
// P1_SFRSTEP
pub const RSTV0910_P1_SFRSTEP: c_uint = 0xf459;
pub const FSTV0910_P1_SFR_SCANSTEP: c_uint = 0xf45940f0;
pub const FSTV0910_P1_SFR_CENTERSTEP: c_uint = 0xf459000f;
// P1_TMGCFG2
pub const RSTV0910_P1_TMGCFG2: c_uint = 0xf45a;
pub const FSTV0910_P1_DIS_AUTOSAMP: c_uint = 0xf45a3008;
pub const FSTV0910_P1_SFRRATIO_FINE: c_uint = 0xf45a0001;
// P1_KREFTMG2
pub const RSTV0910_P1_KREFTMG2: c_uint = 0xf45b;
pub const FSTV0910_P1_KREF_TMG2: c_uint = 0xf45b00ff;
// P1_TMGCFG3
pub const RSTV0910_P1_TMGCFG3: c_uint = 0xf45d;
pub const FSTV0910_P1_CONT_TMGCENTER: c_uint = 0xf45d3008;
pub const FSTV0910_P1_AUTO_GUP: c_uint = 0xf45d2004;
pub const FSTV0910_P1_AUTO_GLOW: c_uint = 0xf45d1002;
// P1_SFRINIT1
pub const RSTV0910_P1_SFRINIT1: c_uint = 0xf45e;
pub const FSTV0910_P1_SFR_INIT1: c_uint = 0xf45e00ff;
// P1_SFRINIT0
pub const RSTV0910_P1_SFRINIT0: c_uint = 0xf45f;
pub const FSTV0910_P1_SFR_INIT0: c_uint = 0xf45f00ff;
// P1_SFRUP1
pub const RSTV0910_P1_SFRUP1: c_uint = 0xf460;
pub const FSTV0910_P1_SYMB_FREQ_UP1: c_uint = 0xf46000ff;
// P1_SFRUP0
pub const RSTV0910_P1_SFRUP0: c_uint = 0xf461;
pub const FSTV0910_P1_SYMB_FREQ_UP0: c_uint = 0xf46100ff;
// P1_SFRLOW1
pub const RSTV0910_P1_SFRLOW1: c_uint = 0xf462;
pub const FSTV0910_P1_SYMB_FREQ_LOW1: c_uint = 0xf46200ff;
// P1_SFRLOW0
pub const RSTV0910_P1_SFRLOW0: c_uint = 0xf463;
pub const FSTV0910_P1_SYMB_FREQ_LOW0: c_uint = 0xf46300ff;
// P1_SFR3
pub const RSTV0910_P1_SFR3: c_uint = 0xf464;
pub const FSTV0910_P1_SYMB_FREQ3: c_uint = 0xf46400ff;
// P1_SFR2
pub const RSTV0910_P1_SFR2: c_uint = 0xf465;
pub const FSTV0910_P1_SYMB_FREQ2: c_uint = 0xf46500ff;
// P1_SFR1
pub const RSTV0910_P1_SFR1: c_uint = 0xf466;
pub const FSTV0910_P1_SYMB_FREQ1: c_uint = 0xf46600ff;
// P1_SFR0
pub const RSTV0910_P1_SFR0: c_uint = 0xf467;
pub const FSTV0910_P1_SYMB_FREQ0: c_uint = 0xf46700ff;
// P1_TMGREG2
pub const RSTV0910_P1_TMGREG2: c_uint = 0xf468;
pub const FSTV0910_P1_TMGREG2: c_uint = 0xf46800ff;
// P1_TMGREG1
pub const RSTV0910_P1_TMGREG1: c_uint = 0xf469;
pub const FSTV0910_P1_TMGREG1: c_uint = 0xf46900ff;
// P1_TMGREG0
pub const RSTV0910_P1_TMGREG0: c_uint = 0xf46a;
pub const FSTV0910_P1_TMGREG0: c_uint = 0xf46a00ff;
// P1_TMGLOCK1
pub const RSTV0910_P1_TMGLOCK1: c_uint = 0xf46b;
pub const FSTV0910_P1_TMGLOCK_LEVEL1: c_uint = 0xf46b01ff;
// P1_TMGLOCK0
pub const RSTV0910_P1_TMGLOCK0: c_uint = 0xf46c;
pub const FSTV0910_P1_TMGLOCK_LEVEL0: c_uint = 0xf46c00ff;
// P1_TMGOBS
pub const RSTV0910_P1_TMGOBS: c_uint = 0xf46d;
pub const FSTV0910_P1_ROLLOFF_STATUS: c_uint = 0xf46d60c0;
// P1_EQUALCFG
pub const RSTV0910_P1_EQUALCFG: c_uint = 0xf46f;
pub const FSTV0910_P1_EQUAL_ON: c_uint = 0xf46f6040;
pub const FSTV0910_P1_MU_EQUALDFE: c_uint = 0xf46f0007;
// P1_EQUAI1
pub const RSTV0910_P1_EQUAI1: c_uint = 0xf470;
pub const FSTV0910_P1_EQUA_ACCI1: c_uint = 0xf47001ff;
// P1_EQUAQ1
pub const RSTV0910_P1_EQUAQ1: c_uint = 0xf471;
pub const FSTV0910_P1_EQUA_ACCQ1: c_uint = 0xf47101ff;
// P1_EQUAI2
pub const RSTV0910_P1_EQUAI2: c_uint = 0xf472;
pub const FSTV0910_P1_EQUA_ACCI2: c_uint = 0xf47201ff;
// P1_EQUAQ2
pub const RSTV0910_P1_EQUAQ2: c_uint = 0xf473;
pub const FSTV0910_P1_EQUA_ACCQ2: c_uint = 0xf47301ff;
// P1_EQUAI3
pub const RSTV0910_P1_EQUAI3: c_uint = 0xf474;
pub const FSTV0910_P1_EQUA_ACCI3: c_uint = 0xf47401ff;
// P1_EQUAQ3
pub const RSTV0910_P1_EQUAQ3: c_uint = 0xf475;
pub const FSTV0910_P1_EQUA_ACCQ3: c_uint = 0xf47501ff;
// P1_EQUAI4
pub const RSTV0910_P1_EQUAI4: c_uint = 0xf476;
pub const FSTV0910_P1_EQUA_ACCI4: c_uint = 0xf47601ff;
// P1_EQUAQ4
pub const RSTV0910_P1_EQUAQ4: c_uint = 0xf477;
pub const FSTV0910_P1_EQUA_ACCQ4: c_uint = 0xf47701ff;
// P1_EQUAI5
pub const RSTV0910_P1_EQUAI5: c_uint = 0xf478;
pub const FSTV0910_P1_EQUA_ACCI5: c_uint = 0xf47801ff;
// P1_EQUAQ5
pub const RSTV0910_P1_EQUAQ5: c_uint = 0xf479;
pub const FSTV0910_P1_EQUA_ACCQ5: c_uint = 0xf47901ff;
// P1_EQUAI6
pub const RSTV0910_P1_EQUAI6: c_uint = 0xf47a;
pub const FSTV0910_P1_EQUA_ACCI6: c_uint = 0xf47a01ff;
// P1_EQUAQ6
pub const RSTV0910_P1_EQUAQ6: c_uint = 0xf47b;
pub const FSTV0910_P1_EQUA_ACCQ6: c_uint = 0xf47b01ff;
// P1_EQUAI7
pub const RSTV0910_P1_EQUAI7: c_uint = 0xf47c;
pub const FSTV0910_P1_EQUA_ACCI7: c_uint = 0xf47c01ff;
// P1_EQUAQ7
pub const RSTV0910_P1_EQUAQ7: c_uint = 0xf47d;
pub const FSTV0910_P1_EQUA_ACCQ7: c_uint = 0xf47d01ff;
// P1_EQUAI8
pub const RSTV0910_P1_EQUAI8: c_uint = 0xf47e;
pub const FSTV0910_P1_EQUA_ACCI8: c_uint = 0xf47e01ff;
// P1_EQUAQ8
pub const RSTV0910_P1_EQUAQ8: c_uint = 0xf47f;
pub const FSTV0910_P1_EQUA_ACCQ8: c_uint = 0xf47f01ff;
// P1_NNOSDATAT1
pub const RSTV0910_P1_NNOSDATAT1: c_uint = 0xf480;
pub const FSTV0910_P1_NOSDATAT_NORMED1: c_uint = 0xf48000ff;
// P1_NNOSDATAT0
pub const RSTV0910_P1_NNOSDATAT0: c_uint = 0xf481;
pub const FSTV0910_P1_NOSDATAT_NORMED0: c_uint = 0xf48100ff;
// P1_NNOSDATA1
pub const RSTV0910_P1_NNOSDATA1: c_uint = 0xf482;
pub const FSTV0910_P1_NOSDATA_NORMED1: c_uint = 0xf48200ff;
// P1_NNOSDATA0
pub const RSTV0910_P1_NNOSDATA0: c_uint = 0xf483;
pub const FSTV0910_P1_NOSDATA_NORMED0: c_uint = 0xf48300ff;
// P1_NNOSPLHT1
pub const RSTV0910_P1_NNOSPLHT1: c_uint = 0xf484;
pub const FSTV0910_P1_NOSPLHT_NORMED1: c_uint = 0xf48400ff;
// P1_NNOSPLHT0
pub const RSTV0910_P1_NNOSPLHT0: c_uint = 0xf485;
pub const FSTV0910_P1_NOSPLHT_NORMED0: c_uint = 0xf48500ff;
// P1_NNOSPLH1
pub const RSTV0910_P1_NNOSPLH1: c_uint = 0xf486;
pub const FSTV0910_P1_NOSPLH_NORMED1: c_uint = 0xf48600ff;
// P1_NNOSPLH0
pub const RSTV0910_P1_NNOSPLH0: c_uint = 0xf487;
pub const FSTV0910_P1_NOSPLH_NORMED0: c_uint = 0xf48700ff;
// P1_NOSDATAT1
pub const RSTV0910_P1_NOSDATAT1: c_uint = 0xf488;
pub const FSTV0910_P1_NOSDATAT_UNNORMED1: c_uint = 0xf48800ff;
// P1_NOSDATAT0
pub const RSTV0910_P1_NOSDATAT0: c_uint = 0xf489;
pub const FSTV0910_P1_NOSDATAT_UNNORMED0: c_uint = 0xf48900ff;
// P1_NNOSFRAME1
pub const RSTV0910_P1_NNOSFRAME1: c_uint = 0xf48a;
pub const FSTV0910_P1_NOSFRAME_NORMED1: c_uint = 0xf48a00ff;
// P1_NNOSFRAME0
pub const RSTV0910_P1_NNOSFRAME0: c_uint = 0xf48b;
pub const FSTV0910_P1_NOSFRAME_NORMED0: c_uint = 0xf48b00ff;
// P1_NNOSRAD1
pub const RSTV0910_P1_NNOSRAD1: c_uint = 0xf48c;
pub const FSTV0910_P1_NOSRADIAL_NORMED1: c_uint = 0xf48c00ff;
// P1_NNOSRAD0
pub const RSTV0910_P1_NNOSRAD0: c_uint = 0xf48d;
pub const FSTV0910_P1_NOSRADIAL_NORMED0: c_uint = 0xf48d00ff;
// P1_NOSCFGF1
pub const RSTV0910_P1_NOSCFGF1: c_uint = 0xf48e;
pub const FSTV0910_P1_LOWNOISE_MESURE: c_uint = 0xf48e7080;
pub const FSTV0910_P1_NOS_DELFRAME: c_uint = 0xf48e6040;
pub const FSTV0910_P1_NOSDATA_MODE: c_uint = 0xf48e4030;
pub const FSTV0910_P1_FRAMESEL_TYPESEL: c_uint = 0xf48e200c;
pub const FSTV0910_P1_FRAMESEL_TYPE: c_uint = 0xf48e0003;
// P1_NOSCFGF2
pub const RSTV0910_P1_NOSCFGF2: c_uint = 0xf48f;
pub const FSTV0910_P1_DIS_NOSPILOTS: c_uint = 0xf48f7080;
pub const FSTV0910_P1_FRAMESEL_MODCODSEL: c_uint = 0xf48f5060;
pub const FSTV0910_P1_FRAMESEL_MODCOD: c_uint = 0xf48f001f;
// P1_CAR2CFG
pub const RSTV0910_P1_CAR2CFG: c_uint = 0xf490;
pub const FSTV0910_P1_ROTA2ON: c_uint = 0xf4902004;
pub const FSTV0910_P1_PH_DET_ALGO2: c_uint = 0xf4900003;
// P1_CFR2CFR1
pub const RSTV0910_P1_CFR2CFR1: c_uint = 0xf491;
pub const FSTV0910_P1_EN_S2CAR2CENTER: c_uint = 0xf4915020;
pub const FSTV0910_P1_CFR2TOCFR1_BETA: c_uint = 0xf4910007;
// P1_CAR3CFG
pub const RSTV0910_P1_CAR3CFG: c_uint = 0xf492;
pub const FSTV0910_P1_CARRIER23_MODE: c_uint = 0xf49260c0;
pub const FSTV0910_P1_CAR3INTERM_DVBS1: c_uint = 0xf4925020;
pub const FSTV0910_P1_ABAMPLIF_MODE: c_uint = 0xf4923018;
pub const FSTV0910_P1_CARRIER3_ALPHA3DL: c_uint = 0xf4920007;
// P1_CFR22
pub const RSTV0910_P1_CFR22: c_uint = 0xf493;
pub const FSTV0910_P1_CAR2_FREQ2: c_uint = 0xf49301ff;
// P1_CFR21
pub const RSTV0910_P1_CFR21: c_uint = 0xf494;
pub const FSTV0910_P1_CAR2_FREQ1: c_uint = 0xf49400ff;
// P1_CFR20
pub const RSTV0910_P1_CFR20: c_uint = 0xf495;
pub const FSTV0910_P1_CAR2_FREQ0: c_uint = 0xf49500ff;
// P1_ACLC2S2Q
pub const RSTV0910_P1_ACLC2S2Q: c_uint = 0xf497;
pub const FSTV0910_P1_ENAB_SPSKSYMB: c_uint = 0xf4977080;
pub const FSTV0910_P1_CAR2S2_Q_ALPH_M: c_uint = 0xf4974030;
pub const FSTV0910_P1_CAR2S2_Q_ALPH_E: c_uint = 0xf497000f;
// P1_ACLC2S28
pub const RSTV0910_P1_ACLC2S28: c_uint = 0xf498;
pub const FSTV0910_P1_CAR2S2_8_ALPH_M: c_uint = 0xf4984030;
pub const FSTV0910_P1_CAR2S2_8_ALPH_E: c_uint = 0xf498000f;
// P1_ACLC2S216A
pub const RSTV0910_P1_ACLC2S216A: c_uint = 0xf499;
pub const FSTV0910_P1_CAR2S2_16A_ALPH_M: c_uint = 0xf4994030;
pub const FSTV0910_P1_CAR2S2_16A_ALPH_E: c_uint = 0xf499000f;
// P1_ACLC2S232A
pub const RSTV0910_P1_ACLC2S232A: c_uint = 0xf49a;
pub const FSTV0910_P1_CAR2S2_32A_ALPH_M: c_uint = 0xf49a4030;
pub const FSTV0910_P1_CAR2S2_32A_ALPH_E: c_uint = 0xf49a000f;
// P1_BCLC2S2Q
pub const RSTV0910_P1_BCLC2S2Q: c_uint = 0xf49c;
pub const FSTV0910_P1_CAR2S2_Q_BETA_M: c_uint = 0xf49c4030;
pub const FSTV0910_P1_CAR2S2_Q_BETA_E: c_uint = 0xf49c000f;
// P1_BCLC2S28
pub const RSTV0910_P1_BCLC2S28: c_uint = 0xf49d;
pub const FSTV0910_P1_CAR2S2_8_BETA_M: c_uint = 0xf49d4030;
pub const FSTV0910_P1_CAR2S2_8_BETA_E: c_uint = 0xf49d000f;
// P1_BCLC2S216A
pub const RSTV0910_P1_BCLC2S216A: c_uint = 0xf49e;
pub const FSTV0910_P1_DVBS2S216A_NIP: c_uint = 0xf49e7080;
pub const FSTV0910_P1_CAR2S2_16A_BETA_M: c_uint = 0xf49e4030;
pub const FSTV0910_P1_CAR2S2_16A_BETA_E: c_uint = 0xf49e000f;
// P1_BCLC2S232A
pub const RSTV0910_P1_BCLC2S232A: c_uint = 0xf49f;
pub const FSTV0910_P1_DVBS2S232A_NIP: c_uint = 0xf49f7080;
pub const FSTV0910_P1_CAR2S2_32A_BETA_M: c_uint = 0xf49f4030;
pub const FSTV0910_P1_CAR2S2_32A_BETA_E: c_uint = 0xf49f000f;
// P1_PLROOT2
pub const RSTV0910_P1_PLROOT2: c_uint = 0xf4ac;
pub const FSTV0910_P1_PLSCRAMB_MODE: c_uint = 0xf4ac200c;
pub const FSTV0910_P1_PLSCRAMB_ROOT2: c_uint = 0xf4ac0003;
// P1_PLROOT1
pub const RSTV0910_P1_PLROOT1: c_uint = 0xf4ad;
pub const FSTV0910_P1_PLSCRAMB_ROOT1: c_uint = 0xf4ad00ff;
// P1_PLROOT0
pub const RSTV0910_P1_PLROOT0: c_uint = 0xf4ae;
pub const FSTV0910_P1_PLSCRAMB_ROOT0: c_uint = 0xf4ae00ff;
// P1_MODCODLST0
pub const RSTV0910_P1_MODCODLST0: c_uint = 0xf4b0;
pub const FSTV0910_P1_NACCES_MODCODCH: c_uint = 0xf4b00001;
// P1_MODCODLST1
pub const RSTV0910_P1_MODCODLST1: c_uint = 0xf4b1;
pub const FSTV0910_P1_SYMBRATE_FILTER: c_uint = 0xf4b13008;
pub const FSTV0910_P1_NRESET_MODCODLST: c_uint = 0xf4b12004;
pub const FSTV0910_P1_DIS_32PSK_9_10: c_uint = 0xf4b10003;
// P1_MODCODLST2
pub const RSTV0910_P1_MODCODLST2: c_uint = 0xf4b2;
pub const FSTV0910_P1_DIS_32PSK_8_9: c_uint = 0xf4b240f0;
pub const FSTV0910_P1_DIS_32PSK_5_6: c_uint = 0xf4b2000f;
// P1_MODCODLST3
pub const RSTV0910_P1_MODCODLST3: c_uint = 0xf4b3;
pub const FSTV0910_P1_DIS_32PSK_4_5: c_uint = 0xf4b340f0;
pub const FSTV0910_P1_DIS_32PSK_3_4: c_uint = 0xf4b3000f;
// P1_MODCODLST4
pub const RSTV0910_P1_MODCODLST4: c_uint = 0xf4b4;
pub const FSTV0910_P1_DUMMYPL_PILOT: c_uint = 0xf4b47080;
pub const FSTV0910_P1_DUMMYPL_NOPILOT: c_uint = 0xf4b46040;
pub const FSTV0910_P1_DIS_16PSK_9_10: c_uint = 0xf4b44030;
pub const FSTV0910_P1_DIS_16PSK_8_9: c_uint = 0xf4b4000f;
// P1_MODCODLST5
pub const RSTV0910_P1_MODCODLST5: c_uint = 0xf4b5;
pub const FSTV0910_P1_DIS_16PSK_5_6: c_uint = 0xf4b540f0;
pub const FSTV0910_P1_DIS_16PSK_4_5: c_uint = 0xf4b5000f;
// P1_MODCODLST6
pub const RSTV0910_P1_MODCODLST6: c_uint = 0xf4b6;
pub const FSTV0910_P1_DIS_16PSK_3_4: c_uint = 0xf4b640f0;
pub const FSTV0910_P1_DIS_16PSK_2_3: c_uint = 0xf4b6000f;
// P1_MODCODLST7
pub const RSTV0910_P1_MODCODLST7: c_uint = 0xf4b7;
pub const FSTV0910_P1_MODCOD_NNOSFILTER: c_uint = 0xf4b77080;
pub const FSTV0910_P1_DIS_8PSK_9_10: c_uint = 0xf4b74030;
pub const FSTV0910_P1_DIS_8PSK_8_9: c_uint = 0xf4b7000f;
// P1_MODCODLST8
pub const RSTV0910_P1_MODCODLST8: c_uint = 0xf4b8;
pub const FSTV0910_P1_DIS_8PSK_5_6: c_uint = 0xf4b840f0;
pub const FSTV0910_P1_DIS_8PSK_3_4: c_uint = 0xf4b8000f;
// P1_MODCODLST9
pub const RSTV0910_P1_MODCODLST9: c_uint = 0xf4b9;
pub const FSTV0910_P1_DIS_8PSK_2_3: c_uint = 0xf4b940f0;
pub const FSTV0910_P1_DIS_8PSK_3_5: c_uint = 0xf4b9000f;
// P1_MODCODLSTA
pub const RSTV0910_P1_MODCODLSTA: c_uint = 0xf4ba;
pub const FSTV0910_P1_NOSFILTER_LIMITE: c_uint = 0xf4ba7080;
pub const FSTV0910_P1_DIS_QPSK_9_10: c_uint = 0xf4ba4030;
pub const FSTV0910_P1_DIS_QPSK_8_9: c_uint = 0xf4ba000f;
// P1_MODCODLSTB
pub const RSTV0910_P1_MODCODLSTB: c_uint = 0xf4bb;
pub const FSTV0910_P1_DIS_QPSK_5_6: c_uint = 0xf4bb40f0;
pub const FSTV0910_P1_DIS_QPSK_4_5: c_uint = 0xf4bb000f;
// P1_MODCODLSTC
pub const RSTV0910_P1_MODCODLSTC: c_uint = 0xf4bc;
pub const FSTV0910_P1_DIS_QPSK_3_4: c_uint = 0xf4bc40f0;
pub const FSTV0910_P1_DIS_QPSK_2_3: c_uint = 0xf4bc000f;
// P1_MODCODLSTD
pub const RSTV0910_P1_MODCODLSTD: c_uint = 0xf4bd;
pub const FSTV0910_P1_DIS_QPSK_3_5: c_uint = 0xf4bd40f0;
pub const FSTV0910_P1_DIS_QPSK_1_2: c_uint = 0xf4bd000f;
// P1_MODCODLSTE
pub const RSTV0910_P1_MODCODLSTE: c_uint = 0xf4be;
pub const FSTV0910_P1_DIS_QPSK_2_5: c_uint = 0xf4be40f0;
pub const FSTV0910_P1_DIS_QPSK_1_3: c_uint = 0xf4be000f;
// P1_MODCODLSTF
pub const RSTV0910_P1_MODCODLSTF: c_uint = 0xf4bf;
pub const FSTV0910_P1_DIS_QPSK_1_4: c_uint = 0xf4bf40f0;
pub const FSTV0910_P1_DEMOD_INVMODLST: c_uint = 0xf4bf3008;
pub const FSTV0910_P1_DEMODOUT_ENABLE: c_uint = 0xf4bf2004;
pub const FSTV0910_P1_DDEMOD_NSET: c_uint = 0xf4bf1002;
pub const FSTV0910_P1_MODCOD_NSTOCK: c_uint = 0xf4bf0001;
// P1_GAUSSR0
pub const RSTV0910_P1_GAUSSR0: c_uint = 0xf4c0;
pub const FSTV0910_P1_EN_CCIMODE: c_uint = 0xf4c07080;
pub const FSTV0910_P1_R0_GAUSSIEN: c_uint = 0xf4c0007f;
// P1_CCIR0
pub const RSTV0910_P1_CCIR0: c_uint = 0xf4c1;
pub const FSTV0910_P1_CCIDETECT_PLHONLY: c_uint = 0xf4c17080;
pub const FSTV0910_P1_R0_CCI: c_uint = 0xf4c1007f;
// P1_CCIQUANT
pub const RSTV0910_P1_CCIQUANT: c_uint = 0xf4c2;
pub const FSTV0910_P1_CCI_BETA: c_uint = 0xf4c250e0;
pub const FSTV0910_P1_CCI_QUANT: c_uint = 0xf4c2001f;
// P1_CCITHRES
pub const RSTV0910_P1_CCITHRES: c_uint = 0xf4c3;
pub const FSTV0910_P1_CCI_THRESHOLD: c_uint = 0xf4c300ff;
// P1_CCIACC
pub const RSTV0910_P1_CCIACC: c_uint = 0xf4c4;
pub const FSTV0910_P1_CCI_VALUE: c_uint = 0xf4c400ff;
// P1_DSTATUS4
pub const RSTV0910_P1_DSTATUS4: c_uint = 0xf4c5;
pub const FSTV0910_P1_RAINFADE_DETECT: c_uint = 0xf4c57080;
pub const FSTV0910_P1_NOTHRES2_FAIL: c_uint = 0xf4c56040;
pub const FSTV0910_P1_NOTHRES1_FAIL: c_uint = 0xf4c55020;
pub const FSTV0910_P1_DMDPROG_ERROR: c_uint = 0xf4c52004;
pub const FSTV0910_P1_CSTENV_DETECT: c_uint = 0xf4c51002;
pub const FSTV0910_P1_DETECTION_TRIAX: c_uint = 0xf4c50001;
// P1_DMDRESCFG
pub const RSTV0910_P1_DMDRESCFG: c_uint = 0xf4c6;
pub const FSTV0910_P1_DMDRES_RESET: c_uint = 0xf4c67080;
pub const FSTV0910_P1_DMDRES_STRALL: c_uint = 0xf4c63008;
pub const FSTV0910_P1_DMDRES_NEWONLY: c_uint = 0xf4c62004;
pub const FSTV0910_P1_DMDRES_NOSTORE: c_uint = 0xf4c61002;
// P1_DMDRESADR
pub const RSTV0910_P1_DMDRESADR: c_uint = 0xf4c7;
pub const FSTV0910_P1_DMDRES_VALIDCFR: c_uint = 0xf4c76040;
pub const FSTV0910_P1_DMDRES_MEMFULL: c_uint = 0xf4c74030;
pub const FSTV0910_P1_DMDRES_RESNBR: c_uint = 0xf4c7000f;
// P1_DMDRESDATA7
pub const RSTV0910_P1_DMDRESDATA7: c_uint = 0xf4c8;
pub const FSTV0910_P1_DMDRES_DATA7: c_uint = 0xf4c800ff;
// P1_DMDRESDATA6
pub const RSTV0910_P1_DMDRESDATA6: c_uint = 0xf4c9;
pub const FSTV0910_P1_DMDRES_DATA6: c_uint = 0xf4c900ff;
// P1_DMDRESDATA5
pub const RSTV0910_P1_DMDRESDATA5: c_uint = 0xf4ca;
pub const FSTV0910_P1_DMDRES_DATA5: c_uint = 0xf4ca00ff;
// P1_DMDRESDATA4
pub const RSTV0910_P1_DMDRESDATA4: c_uint = 0xf4cb;
pub const FSTV0910_P1_DMDRES_DATA4: c_uint = 0xf4cb00ff;
// P1_DMDRESDATA3
pub const RSTV0910_P1_DMDRESDATA3: c_uint = 0xf4cc;
pub const FSTV0910_P1_DMDRES_DATA3: c_uint = 0xf4cc00ff;
// P1_DMDRESDATA2
pub const RSTV0910_P1_DMDRESDATA2: c_uint = 0xf4cd;
pub const FSTV0910_P1_DMDRES_DATA2: c_uint = 0xf4cd00ff;
// P1_DMDRESDATA1
pub const RSTV0910_P1_DMDRESDATA1: c_uint = 0xf4ce;
pub const FSTV0910_P1_DMDRES_DATA1: c_uint = 0xf4ce00ff;
// P1_DMDRESDATA0
pub const RSTV0910_P1_DMDRESDATA0: c_uint = 0xf4cf;
pub const FSTV0910_P1_DMDRES_DATA0: c_uint = 0xf4cf00ff;
// P1_FFEI1
pub const RSTV0910_P1_FFEI1: c_uint = 0xf4d0;
pub const FSTV0910_P1_FFE_ACCI1: c_uint = 0xf4d001ff;
// P1_FFEQ1
pub const RSTV0910_P1_FFEQ1: c_uint = 0xf4d1;
pub const FSTV0910_P1_FFE_ACCQ1: c_uint = 0xf4d101ff;
// P1_FFEI2
pub const RSTV0910_P1_FFEI2: c_uint = 0xf4d2;
pub const FSTV0910_P1_FFE_ACCI2: c_uint = 0xf4d201ff;
// P1_FFEQ2
pub const RSTV0910_P1_FFEQ2: c_uint = 0xf4d3;
pub const FSTV0910_P1_FFE_ACCQ2: c_uint = 0xf4d301ff;
// P1_FFEI3
pub const RSTV0910_P1_FFEI3: c_uint = 0xf4d4;
pub const FSTV0910_P1_FFE_ACCI3: c_uint = 0xf4d401ff;
// P1_FFEQ3
pub const RSTV0910_P1_FFEQ3: c_uint = 0xf4d5;
pub const FSTV0910_P1_FFE_ACCQ3: c_uint = 0xf4d501ff;
// P1_FFEI4
pub const RSTV0910_P1_FFEI4: c_uint = 0xf4d6;
pub const FSTV0910_P1_FFE_ACCI4: c_uint = 0xf4d601ff;
// P1_FFEQ4
pub const RSTV0910_P1_FFEQ4: c_uint = 0xf4d7;
pub const FSTV0910_P1_FFE_ACCQ4: c_uint = 0xf4d701ff;
// P1_FFECFG
pub const RSTV0910_P1_FFECFG: c_uint = 0xf4d8;
pub const FSTV0910_P1_EQUALFFE_ON: c_uint = 0xf4d86040;
pub const FSTV0910_P1_EQUAL_USEDSYMB: c_uint = 0xf4d84030;
pub const FSTV0910_P1_MU_EQUALFFE: c_uint = 0xf4d80007;
// P1_TNRCFG2
pub const RSTV0910_P1_TNRCFG2: c_uint = 0xf4e1;
pub const FSTV0910_P1_TUN_IQSWAP: c_uint = 0xf4e17080;
// P1_SMAPCOEF7
pub const RSTV0910_P1_SMAPCOEF7: c_uint = 0xf500;
pub const FSTV0910_P1_DIS_QSCALE: c_uint = 0xf5007080;
pub const FSTV0910_P1_SMAPCOEF_Q_LLR12: c_uint = 0xf500017f;
// P1_SMAPCOEF6
pub const RSTV0910_P1_SMAPCOEF6: c_uint = 0xf501;
pub const FSTV0910_P1_DIS_AGC2SCALE: c_uint = 0xf5017080;
pub const FSTV0910_P1_ADJ_8PSKLLR1: c_uint = 0xf5012004;
pub const FSTV0910_P1_OLD_8PSKLLR1: c_uint = 0xf5011002;
pub const FSTV0910_P1_DIS_AB8PSK: c_uint = 0xf5010001;
// P1_SMAPCOEF5
pub const RSTV0910_P1_SMAPCOEF5: c_uint = 0xf502;
pub const FSTV0910_P1_DIS_8SCALE: c_uint = 0xf5027080;
pub const FSTV0910_P1_SMAPCOEF_8P_LLR23: c_uint = 0xf502017f;
// P1_SMAPCOEF4
pub const RSTV0910_P1_SMAPCOEF4: c_uint = 0xf503;
pub const FSTV0910_P1_SMAPCOEF_16APSK_LLR12: c_uint = 0xf503017f;
// P1_SMAPCOEF3
pub const RSTV0910_P1_SMAPCOEF3: c_uint = 0xf504;
pub const FSTV0910_P1_SMAPCOEF_16APSK_LLR34: c_uint = 0xf504017f;
// P1_SMAPCOEF2
pub const RSTV0910_P1_SMAPCOEF2: c_uint = 0xf505;
pub const FSTV0910_P1_SMAPCOEF_32APSK_R2R3: c_uint = 0xf50541f0;
pub const FSTV0910_P1_SMAPCOEF_32APSK_LLR2: c_uint = 0xf505010f;
// P1_SMAPCOEF1
pub const RSTV0910_P1_SMAPCOEF1: c_uint = 0xf506;
pub const FSTV0910_P1_DIS_16SCALE: c_uint = 0xf5067080;
pub const FSTV0910_P1_SMAPCOEF_32_LLR34: c_uint = 0xf506017f;
// P1_SMAPCOEF0
pub const RSTV0910_P1_SMAPCOEF0: c_uint = 0xf507;
pub const FSTV0910_P1_DIS_32SCALE: c_uint = 0xf5077080;
pub const FSTV0910_P1_SMAPCOEF_32_LLR15: c_uint = 0xf507017f;
// P1_NOSTHRES1
pub const RSTV0910_P1_NOSTHRES1: c_uint = 0xf509;
pub const FSTV0910_P1_NOS_THRESHOLD1: c_uint = 0xf50900ff;
// P1_NOSTHRES2
pub const RSTV0910_P1_NOSTHRES2: c_uint = 0xf50a;
pub const FSTV0910_P1_NOS_THRESHOLD2: c_uint = 0xf50a00ff;
// P1_NOSDIFF1
pub const RSTV0910_P1_NOSDIFF1: c_uint = 0xf50b;
pub const FSTV0910_P1_NOSTHRES1_DIFF: c_uint = 0xf50b00ff;
// P1_RAINFADE
pub const RSTV0910_P1_RAINFADE: c_uint = 0xf50c;
pub const FSTV0910_P1_NOSTHRES_DATAT: c_uint = 0xf50c7080;
pub const FSTV0910_P1_RAINFADE_CNLIMIT: c_uint = 0xf50c4070;
pub const FSTV0910_P1_RAINFADE_TIMEOUT: c_uint = 0xf50c0007;
// P1_NOSRAMCFG
pub const RSTV0910_P1_NOSRAMCFG: c_uint = 0xf50d;
pub const FSTV0910_P1_NOSRAM_ACTIVATION: c_uint = 0xf50d4030;
pub const FSTV0910_P1_NOSRAM_CNRONLY: c_uint = 0xf50d3008;
pub const FSTV0910_P1_NOSRAM_LGNCNR1: c_uint = 0xf50d0007;
// P1_NOSRAMPOS
pub const RSTV0910_P1_NOSRAMPOS: c_uint = 0xf50e;
pub const FSTV0910_P1_NOSRAM_LGNCNR0: c_uint = 0xf50e40f0;
pub const FSTV0910_P1_NOSRAM_VALIDE: c_uint = 0xf50e2004;
pub const FSTV0910_P1_NOSRAM_CNRVAL1: c_uint = 0xf50e0003;
// P1_NOSRAMVAL
pub const RSTV0910_P1_NOSRAMVAL: c_uint = 0xf50f;
pub const FSTV0910_P1_NOSRAM_CNRVAL0: c_uint = 0xf50f00ff;
// P1_DMDPLHSTAT
pub const RSTV0910_P1_DMDPLHSTAT: c_uint = 0xf520;
pub const FSTV0910_P1_PLH_STATISTIC: c_uint = 0xf52000ff;
// P1_LOCKTIME3
pub const RSTV0910_P1_LOCKTIME3: c_uint = 0xf522;
pub const FSTV0910_P1_DEMOD_LOCKTIME3: c_uint = 0xf52200ff;
// P1_LOCKTIME2
pub const RSTV0910_P1_LOCKTIME2: c_uint = 0xf523;
pub const FSTV0910_P1_DEMOD_LOCKTIME2: c_uint = 0xf52300ff;
// P1_LOCKTIME1
pub const RSTV0910_P1_LOCKTIME1: c_uint = 0xf524;
pub const FSTV0910_P1_DEMOD_LOCKTIME1: c_uint = 0xf52400ff;
// P1_LOCKTIME0
pub const RSTV0910_P1_LOCKTIME0: c_uint = 0xf525;
pub const FSTV0910_P1_DEMOD_LOCKTIME0: c_uint = 0xf52500ff;
// P1_VITSCALE
pub const RSTV0910_P1_VITSCALE: c_uint = 0xf532;
pub const FSTV0910_P1_NVTH_NOSRANGE: c_uint = 0xf5327080;
pub const FSTV0910_P1_VERROR_MAXMODE: c_uint = 0xf5326040;
pub const FSTV0910_P1_NSLOWSN_LOCKED: c_uint = 0xf5323008;
pub const FSTV0910_P1_DIS_RSFLOCK: c_uint = 0xf5321002;
// P1_FECM
pub const RSTV0910_P1_FECM: c_uint = 0xf533;
pub const FSTV0910_P1_DSS_DVB: c_uint = 0xf5337080;
pub const FSTV0910_P1_DSS_SRCH: c_uint = 0xf5334010;
pub const FSTV0910_P1_SYNCVIT: c_uint = 0xf5331002;
pub const FSTV0910_P1_IQINV: c_uint = 0xf5330001;
// P1_VTH12
pub const RSTV0910_P1_VTH12: c_uint = 0xf534;
pub const FSTV0910_P1_VTH12: c_uint = 0xf53400ff;
// P1_VTH23
pub const RSTV0910_P1_VTH23: c_uint = 0xf535;
pub const FSTV0910_P1_VTH23: c_uint = 0xf53500ff;
// P1_VTH34
pub const RSTV0910_P1_VTH34: c_uint = 0xf536;
pub const FSTV0910_P1_VTH34: c_uint = 0xf53600ff;
// P1_VTH56
pub const RSTV0910_P1_VTH56: c_uint = 0xf537;
pub const FSTV0910_P1_VTH56: c_uint = 0xf53700ff;
// P1_VTH67
pub const RSTV0910_P1_VTH67: c_uint = 0xf538;
pub const FSTV0910_P1_VTH67: c_uint = 0xf53800ff;
// P1_VTH78
pub const RSTV0910_P1_VTH78: c_uint = 0xf539;
pub const FSTV0910_P1_VTH78: c_uint = 0xf53900ff;
// P1_VITCURPUN
pub const RSTV0910_P1_VITCURPUN: c_uint = 0xf53a;
pub const FSTV0910_P1_VIT_CURPUN: c_uint = 0xf53a001f;
// P1_VERROR
pub const RSTV0910_P1_VERROR: c_uint = 0xf53b;
pub const FSTV0910_P1_REGERR_VIT: c_uint = 0xf53b00ff;
// P1_PRVIT
pub const RSTV0910_P1_PRVIT: c_uint = 0xf53c;
pub const FSTV0910_P1_DIS_VTHLOCK: c_uint = 0xf53c6040;
pub const FSTV0910_P1_E7_8VIT: c_uint = 0xf53c5020;
pub const FSTV0910_P1_E6_7VIT: c_uint = 0xf53c4010;
pub const FSTV0910_P1_E5_6VIT: c_uint = 0xf53c3008;
pub const FSTV0910_P1_E3_4VIT: c_uint = 0xf53c2004;
pub const FSTV0910_P1_E2_3VIT: c_uint = 0xf53c1002;
pub const FSTV0910_P1_E1_2VIT: c_uint = 0xf53c0001;
// P1_VAVSRVIT
pub const RSTV0910_P1_VAVSRVIT: c_uint = 0xf53d;
pub const FSTV0910_P1_AMVIT: c_uint = 0xf53d7080;
pub const FSTV0910_P1_FROZENVIT: c_uint = 0xf53d6040;
pub const FSTV0910_P1_SNVIT: c_uint = 0xf53d4030;
pub const FSTV0910_P1_TOVVIT: c_uint = 0xf53d200c;
pub const FSTV0910_P1_HYPVIT: c_uint = 0xf53d0003;
// P1_VSTATUSVIT
pub const RSTV0910_P1_VSTATUSVIT: c_uint = 0xf53e;
pub const FSTV0910_P1_PRFVIT: c_uint = 0xf53e4010;
pub const FSTV0910_P1_LOCKEDVIT: c_uint = 0xf53e3008;
// P1_VTHINUSE
pub const RSTV0910_P1_VTHINUSE: c_uint = 0xf53f;
pub const FSTV0910_P1_VIT_INUSE: c_uint = 0xf53f00ff;
// P1_KDIV12
pub const RSTV0910_P1_KDIV12: c_uint = 0xf540;
pub const FSTV0910_P1_K_DIVIDER_12: c_uint = 0xf540007f;
// P1_KDIV23
pub const RSTV0910_P1_KDIV23: c_uint = 0xf541;
pub const FSTV0910_P1_K_DIVIDER_23: c_uint = 0xf541007f;
// P1_KDIV34
pub const RSTV0910_P1_KDIV34: c_uint = 0xf542;
pub const FSTV0910_P1_K_DIVIDER_34: c_uint = 0xf542007f;
// P1_KDIV56
pub const RSTV0910_P1_KDIV56: c_uint = 0xf543;
pub const FSTV0910_P1_K_DIVIDER_56: c_uint = 0xf543007f;
// P1_KDIV67
pub const RSTV0910_P1_KDIV67: c_uint = 0xf544;
pub const FSTV0910_P1_K_DIVIDER_67: c_uint = 0xf544007f;
// P1_KDIV78
pub const RSTV0910_P1_KDIV78: c_uint = 0xf545;
pub const FSTV0910_P1_K_DIVIDER_78: c_uint = 0xf545007f;
// P1_TSPIDFLT1
pub const RSTV0910_P1_TSPIDFLT1: c_uint = 0xf546;
pub const FSTV0910_P1_PIDFLT_ADDR: c_uint = 0xf54600ff;
// P1_TSPIDFLT0
pub const RSTV0910_P1_TSPIDFLT0: c_uint = 0xf547;
pub const FSTV0910_P1_PIDFLT_DATA: c_uint = 0xf54700ff;
// P1_PDELCTRL0
pub const RSTV0910_P1_PDELCTRL0: c_uint = 0xf54f;
pub const FSTV0910_P1_ISIOBS_MODE: c_uint = 0xf54f4030;
// P1_PDELCTRL1
pub const RSTV0910_P1_PDELCTRL1: c_uint = 0xf550;
pub const FSTV0910_P1_INV_MISMASK: c_uint = 0xf5507080;
pub const FSTV0910_P1_FILTER_EN: c_uint = 0xf5505020;
pub const FSTV0910_P1_HYSTEN: c_uint = 0xf5503008;
pub const FSTV0910_P1_HYSTSWRST: c_uint = 0xf5502004;
pub const FSTV0910_P1_EN_MIS00: c_uint = 0xf5501002;
pub const FSTV0910_P1_ALGOSWRST: c_uint = 0xf5500001;
// P1_PDELCTRL2
pub const RSTV0910_P1_PDELCTRL2: c_uint = 0xf551;
pub const FSTV0910_P1_FORCE_CONTINUOUS: c_uint = 0xf5517080;
pub const FSTV0910_P1_RESET_UPKO_COUNT: c_uint = 0xf5516040;
pub const FSTV0910_P1_USER_PKTDELIN_NB: c_uint = 0xf5515020;
pub const FSTV0910_P1_FRAME_MODE: c_uint = 0xf5511002;
// P1_HYSTTHRESH
pub const RSTV0910_P1_HYSTTHRESH: c_uint = 0xf554;
pub const FSTV0910_P1_DELIN_LOCKTHRES: c_uint = 0xf55440f0;
pub const FSTV0910_P1_DELIN_UNLOCKTHRES: c_uint = 0xf554000f;
// P1_UPLCCST0
pub const RSTV0910_P1_UPLCCST0: c_uint = 0xf558;
pub const FSTV0910_P1_UPL_CST0: c_uint = 0xf55830f8;
pub const FSTV0910_P1_UPL_MODE: c_uint = 0xf5580007;
// P1_ISIENTRY
pub const RSTV0910_P1_ISIENTRY: c_uint = 0xf55e;
pub const FSTV0910_P1_ISI_ENTRY: c_uint = 0xf55e00ff;
// P1_ISIBITENA
pub const RSTV0910_P1_ISIBITENA: c_uint = 0xf55f;
pub const FSTV0910_P1_ISI_BIT_EN: c_uint = 0xf55f00ff;
// P1_MATSTR1
pub const RSTV0910_P1_MATSTR1: c_uint = 0xf560;
pub const FSTV0910_P1_MATYPE_CURRENT1: c_uint = 0xf56000ff;
// P1_MATSTR0
pub const RSTV0910_P1_MATSTR0: c_uint = 0xf561;
pub const FSTV0910_P1_MATYPE_CURRENT0: c_uint = 0xf56100ff;
// P1_UPLSTR1
pub const RSTV0910_P1_UPLSTR1: c_uint = 0xf562;
pub const FSTV0910_P1_UPL_CURRENT1: c_uint = 0xf56200ff;
// P1_UPLSTR0
pub const RSTV0910_P1_UPLSTR0: c_uint = 0xf563;
pub const FSTV0910_P1_UPL_CURRENT0: c_uint = 0xf56300ff;
// P1_DFLSTR1
pub const RSTV0910_P1_DFLSTR1: c_uint = 0xf564;
pub const FSTV0910_P1_DFL_CURRENT1: c_uint = 0xf56400ff;
// P1_DFLSTR0
pub const RSTV0910_P1_DFLSTR0: c_uint = 0xf565;
pub const FSTV0910_P1_DFL_CURRENT0: c_uint = 0xf56500ff;
// P1_SYNCSTR
pub const RSTV0910_P1_SYNCSTR: c_uint = 0xf566;
pub const FSTV0910_P1_SYNC_CURRENT: c_uint = 0xf56600ff;
// P1_SYNCDSTR1
pub const RSTV0910_P1_SYNCDSTR1: c_uint = 0xf567;
pub const FSTV0910_P1_SYNCD_CURRENT1: c_uint = 0xf56700ff;
// P1_SYNCDSTR0
pub const RSTV0910_P1_SYNCDSTR0: c_uint = 0xf568;
pub const FSTV0910_P1_SYNCD_CURRENT0: c_uint = 0xf56800ff;
// P1_PDELSTATUS1
pub const RSTV0910_P1_PDELSTATUS1: c_uint = 0xf569;
pub const FSTV0910_P1_PKTDELIN_DELOCK: c_uint = 0xf5697080;
pub const FSTV0910_P1_SYNCDUPDFL_BADDFL: c_uint = 0xf5696040;
pub const FSTV0910_P1_UNACCEPTED_STREAM: c_uint = 0xf5694010;
pub const FSTV0910_P1_BCH_ERROR_FLAG: c_uint = 0xf5693008;
pub const FSTV0910_P1_PKTDELIN_LOCK: c_uint = 0xf5691002;
pub const FSTV0910_P1_FIRST_LOCK: c_uint = 0xf5690001;
// P1_PDELSTATUS2
pub const RSTV0910_P1_PDELSTATUS2: c_uint = 0xf56a;
pub const FSTV0910_P1_FRAME_MODCOD: c_uint = 0xf56a207c;
pub const FSTV0910_P1_FRAME_TYPE: c_uint = 0xf56a0003;
// P1_BBFCRCKO1
pub const RSTV0910_P1_BBFCRCKO1: c_uint = 0xf56b;
pub const FSTV0910_P1_BBHCRC_KOCNT1: c_uint = 0xf56b00ff;
// P1_BBFCRCKO0
pub const RSTV0910_P1_BBFCRCKO0: c_uint = 0xf56c;
pub const FSTV0910_P1_BBHCRC_KOCNT0: c_uint = 0xf56c00ff;
// P1_UPCRCKO1
pub const RSTV0910_P1_UPCRCKO1: c_uint = 0xf56d;
pub const FSTV0910_P1_PKTCRC_KOCNT1: c_uint = 0xf56d00ff;
// P1_UPCRCKO0
pub const RSTV0910_P1_UPCRCKO0: c_uint = 0xf56e;
pub const FSTV0910_P1_PKTCRC_KOCNT0: c_uint = 0xf56e00ff;
// P1_PDELCTRL3
pub const RSTV0910_P1_PDELCTRL3: c_uint = 0xf56f;
pub const FSTV0910_P1_NOFIFO_BCHERR: c_uint = 0xf56f5020;
pub const FSTV0910_P1_PKTDELIN_DELACMERR: c_uint = 0xf56f4010;
// P1_TSSTATEM
pub const RSTV0910_P1_TSSTATEM: c_uint = 0xf570;
pub const FSTV0910_P1_TSDIL_ON: c_uint = 0xf5707080;
pub const FSTV0910_P1_TSRS_ON: c_uint = 0xf5705020;
pub const FSTV0910_P1_TSDESCRAMB_ON: c_uint = 0xf5704010;
pub const FSTV0910_P1_TSFRAME_MODE: c_uint = 0xf5703008;
pub const FSTV0910_P1_TS_DISABLE: c_uint = 0xf5702004;
pub const FSTV0910_P1_TSACM_MODE: c_uint = 0xf5701002;
pub const FSTV0910_P1_TSOUT_NOSYNC: c_uint = 0xf5700001;
// P1_TSSTATEL
pub const RSTV0910_P1_TSSTATEL: c_uint = 0xf571;
pub const FSTV0910_P1_TSNOSYNCBYTE: c_uint = 0xf5717080;
pub const FSTV0910_P1_TSPARITY_ON: c_uint = 0xf5716040;
pub const FSTV0910_P1_TSISSYI_ON: c_uint = 0xf5713008;
pub const FSTV0910_P1_TSNPD_ON: c_uint = 0xf5712004;
pub const FSTV0910_P1_TSCRC8_ON: c_uint = 0xf5711002;
pub const FSTV0910_P1_TSDSS_PACKET: c_uint = 0xf5710001;
// P1_TSCFGH
pub const RSTV0910_P1_TSCFGH: c_uint = 0xf572;
pub const FSTV0910_P1_TSFIFO_DVBCI: c_uint = 0xf5727080;
pub const FSTV0910_P1_TSFIFO_SERIAL: c_uint = 0xf5726040;
pub const FSTV0910_P1_TSFIFO_TEIUPDATE: c_uint = 0xf5725020;
pub const FSTV0910_P1_TSFIFO_DUTY50: c_uint = 0xf5724010;
pub const FSTV0910_P1_TSFIFO_HSGNLOUT: c_uint = 0xf5723008;
pub const FSTV0910_P1_TSFIFO_ERRMODE: c_uint = 0xf5721006;
pub const FSTV0910_P1_RST_HWARE: c_uint = 0xf5720001;
// P1_TSCFGM
pub const RSTV0910_P1_TSCFGM: c_uint = 0xf573;
pub const FSTV0910_P1_TSFIFO_MANSPEED: c_uint = 0xf57360c0;
pub const FSTV0910_P1_TSFIFO_PERMDATA: c_uint = 0xf5735020;
pub const FSTV0910_P1_TSFIFO_NONEWSGNL: c_uint = 0xf5734010;
pub const FSTV0910_P1_TSFIFO_INVDATA: c_uint = 0xf5730001;
// P1_TSCFGL
pub const RSTV0910_P1_TSCFGL: c_uint = 0xf574;
pub const FSTV0910_P1_TSFIFO_BCLKDEL1CK: c_uint = 0xf57460c0;
pub const FSTV0910_P1_BCHERROR_MODE: c_uint = 0xf5744030;
pub const FSTV0910_P1_TSFIFO_NSGNL2DATA: c_uint = 0xf5743008;
pub const FSTV0910_P1_TSFIFO_EMBINDVB: c_uint = 0xf5742004;
pub const FSTV0910_P1_TSFIFO_BITSPEED: c_uint = 0xf5740003;
// P1_TSSYNC
pub const RSTV0910_P1_TSSYNC: c_uint = 0xf575;
pub const FSTV0910_P1_TSFIFO_SYNCMODE: c_uint = 0xf5753018;
// P1_TSINSDELH
pub const RSTV0910_P1_TSINSDELH: c_uint = 0xf576;
pub const FSTV0910_P1_TSDEL_SYNCBYTE: c_uint = 0xf5767080;
pub const FSTV0910_P1_TSDEL_XXHEADER: c_uint = 0xf5766040;
pub const FSTV0910_P1_TSDEL_DATAFIELD: c_uint = 0xf5764010;
pub const FSTV0910_P1_TSINSDEL_RSPARITY: c_uint = 0xf5761002;
pub const FSTV0910_P1_TSINSDEL_CRC8: c_uint = 0xf5760001;
// P1_TSINSDELM
pub const RSTV0910_P1_TSINSDELM: c_uint = 0xf577;
pub const FSTV0910_P1_TSINS_EMODCOD: c_uint = 0xf5774010;
pub const FSTV0910_P1_TSINS_TOKEN: c_uint = 0xf5773008;
pub const FSTV0910_P1_TSINS_XXXERR: c_uint = 0xf5772004;
pub const FSTV0910_P1_TSINS_MATYPE: c_uint = 0xf5771002;
pub const FSTV0910_P1_TSINS_UPL: c_uint = 0xf5770001;
// P1_TSINSDELL
pub const RSTV0910_P1_TSINSDELL: c_uint = 0xf578;
pub const FSTV0910_P1_TSINS_DFL: c_uint = 0xf5787080;
pub const FSTV0910_P1_TSINS_SYNCD: c_uint = 0xf5786040;
pub const FSTV0910_P1_TSINS_BLOCLEN: c_uint = 0xf5785020;
pub const FSTV0910_P1_TSINS_SIGPCOUNT: c_uint = 0xf5784010;
pub const FSTV0910_P1_TSINS_FIFO: c_uint = 0xf5783008;
pub const FSTV0910_P1_TSINS_REALPACK: c_uint = 0xf5782004;
pub const FSTV0910_P1_TSINS_TSCONFIG: c_uint = 0xf5781002;
pub const FSTV0910_P1_TSINS_LATENCY: c_uint = 0xf5780001;
// P1_TSDIVN
pub const RSTV0910_P1_TSDIVN: c_uint = 0xf579;
pub const FSTV0910_P1_TSFIFO_SPEEDMODE: c_uint = 0xf57960c0;
pub const FSTV0910_P1_TSFIFO_RISEOK: c_uint = 0xf5790007;
// P1_TSCFG4
pub const RSTV0910_P1_TSCFG4: c_uint = 0xf57a;
pub const FSTV0910_P1_TSFIFO_TSSPEEDMODE: c_uint = 0xf57a60c0;
// P1_TSSPEED
pub const RSTV0910_P1_TSSPEED: c_uint = 0xf580;
pub const FSTV0910_P1_TSFIFO_OUTSPEED: c_uint = 0xf58000ff;
// P1_TSSTATUS
pub const RSTV0910_P1_TSSTATUS: c_uint = 0xf581;
pub const FSTV0910_P1_TSFIFO_LINEOK: c_uint = 0xf5817080;
pub const FSTV0910_P1_TSFIFO_ERROR: c_uint = 0xf5816040;
pub const FSTV0910_P1_TSFIFO_NOSYNC: c_uint = 0xf5814010;
pub const FSTV0910_P1_TSREGUL_ERROR: c_uint = 0xf5812004;
pub const FSTV0910_P1_DIL_READY: c_uint = 0xf5810001;
// P1_TSSTATUS2
pub const RSTV0910_P1_TSSTATUS2: c_uint = 0xf582;
pub const FSTV0910_P1_TSFIFO_DEMODSEL: c_uint = 0xf5827080;
pub const FSTV0910_P1_TSFIFOSPEED_STORE: c_uint = 0xf5826040;
pub const FSTV0910_P1_DILXX_RESET: c_uint = 0xf5825020;
pub const FSTV0910_P1_SCRAMBDETECT: c_uint = 0xf5821002;
// P1_TSBITRATE1
pub const RSTV0910_P1_TSBITRATE1: c_uint = 0xf583;
pub const FSTV0910_P1_TSFIFO_BITRATE1: c_uint = 0xf58300ff;
// P1_TSBITRATE0
pub const RSTV0910_P1_TSBITRATE0: c_uint = 0xf584;
pub const FSTV0910_P1_TSFIFO_BITRATE0: c_uint = 0xf58400ff;
// P1_TSPACKLEN1
pub const RSTV0910_P1_TSPACKLEN1: c_uint = 0xf585;
pub const FSTV0910_P1_TSFIFO_PACKCPT: c_uint = 0xf58550e0;
// P1_TSDLY2
pub const RSTV0910_P1_TSDLY2: c_uint = 0xf589;
pub const FSTV0910_P1_SOFFIFO_LATENCY2: c_uint = 0xf589000f;
// P1_TSDLY1
pub const RSTV0910_P1_TSDLY1: c_uint = 0xf58a;
pub const FSTV0910_P1_SOFFIFO_LATENCY1: c_uint = 0xf58a00ff;
// P1_TSDLY0
pub const RSTV0910_P1_TSDLY0: c_uint = 0xf58b;
pub const FSTV0910_P1_SOFFIFO_LATENCY0: c_uint = 0xf58b00ff;
// P1_TSNPDAV
pub const RSTV0910_P1_TSNPDAV: c_uint = 0xf58c;
pub const FSTV0910_P1_TSNPD_AVERAGE: c_uint = 0xf58c00ff;
// P1_TSBUFSTAT2
pub const RSTV0910_P1_TSBUFSTAT2: c_uint = 0xf58d;
pub const FSTV0910_P1_TSISCR_3BYTES: c_uint = 0xf58d7080;
pub const FSTV0910_P1_TSISCR_NEWDATA: c_uint = 0xf58d6040;
pub const FSTV0910_P1_TSISCR_BUFSTAT2: c_uint = 0xf58d003f;
// P1_TSBUFSTAT1
pub const RSTV0910_P1_TSBUFSTAT1: c_uint = 0xf58e;
pub const FSTV0910_P1_TSISCR_BUFSTAT1: c_uint = 0xf58e00ff;
// P1_TSBUFSTAT0
pub const RSTV0910_P1_TSBUFSTAT0: c_uint = 0xf58f;
pub const FSTV0910_P1_TSISCR_BUFSTAT0: c_uint = 0xf58f00ff;
// P1_TSDEBUGL
pub const RSTV0910_P1_TSDEBUGL: c_uint = 0xf591;
pub const FSTV0910_P1_TSFIFO_ERROR_EVNT: c_uint = 0xf5912004;
pub const FSTV0910_P1_TSFIFO_OVERFLOWM: c_uint = 0xf5910001;
// P1_TSDLYSET2
pub const RSTV0910_P1_TSDLYSET2: c_uint = 0xf592;
pub const FSTV0910_P1_SOFFIFO_OFFSET: c_uint = 0xf59260c0;
pub const FSTV0910_P1_HYSTERESIS_THRESHOLD: c_uint = 0xf5924030;
pub const FSTV0910_P1_SOFFIFO_SYMBOFFS2: c_uint = 0xf592000f;
// P1_TSDLYSET1
pub const RSTV0910_P1_TSDLYSET1: c_uint = 0xf593;
pub const FSTV0910_P1_SOFFIFO_SYMBOFFS1: c_uint = 0xf59300ff;
// P1_TSDLYSET0
pub const RSTV0910_P1_TSDLYSET0: c_uint = 0xf594;
pub const FSTV0910_P1_SOFFIFO_SYMBOFFS0: c_uint = 0xf59400ff;
// P1_ERRCTRL1
pub const RSTV0910_P1_ERRCTRL1: c_uint = 0xf598;
pub const FSTV0910_P1_ERR_SOURCE1: c_uint = 0xf59840f0;
pub const FSTV0910_P1_NUM_EVENT1: c_uint = 0xf5980007;
// P1_ERRCNT12
pub const RSTV0910_P1_ERRCNT12: c_uint = 0xf599;
pub const FSTV0910_P1_ERRCNT1_OLDVALUE: c_uint = 0xf5997080;
pub const FSTV0910_P1_ERR_CNT12: c_uint = 0xf599007f;
// P1_ERRCNT11
pub const RSTV0910_P1_ERRCNT11: c_uint = 0xf59a;
pub const FSTV0910_P1_ERR_CNT11: c_uint = 0xf59a00ff;
// P1_ERRCNT10
pub const RSTV0910_P1_ERRCNT10: c_uint = 0xf59b;
pub const FSTV0910_P1_ERR_CNT10: c_uint = 0xf59b00ff;
// P1_ERRCTRL2
pub const RSTV0910_P1_ERRCTRL2: c_uint = 0xf59c;
pub const FSTV0910_P1_ERR_SOURCE2: c_uint = 0xf59c40f0;
pub const FSTV0910_P1_NUM_EVENT2: c_uint = 0xf59c0007;
// P1_ERRCNT22
pub const RSTV0910_P1_ERRCNT22: c_uint = 0xf59d;
pub const FSTV0910_P1_ERRCNT2_OLDVALUE: c_uint = 0xf59d7080;
pub const FSTV0910_P1_ERR_CNT22: c_uint = 0xf59d007f;
// P1_ERRCNT21
pub const RSTV0910_P1_ERRCNT21: c_uint = 0xf59e;
pub const FSTV0910_P1_ERR_CNT21: c_uint = 0xf59e00ff;
// P1_ERRCNT20
pub const RSTV0910_P1_ERRCNT20: c_uint = 0xf59f;
pub const FSTV0910_P1_ERR_CNT20: c_uint = 0xf59f00ff;
// P1_FECSPY
pub const RSTV0910_P1_FECSPY: c_uint = 0xf5a0;
pub const FSTV0910_P1_SPY_ENABLE: c_uint = 0xf5a07080;
pub const FSTV0910_P1_NO_SYNCBYTE: c_uint = 0xf5a06040;
pub const FSTV0910_P1_SERIAL_MODE: c_uint = 0xf5a05020;
pub const FSTV0910_P1_UNUSUAL_PACKET: c_uint = 0xf5a04010;
pub const FSTV0910_P1_BERMETER_DATAMODE: c_uint = 0xf5a0200c;
pub const FSTV0910_P1_BERMETER_LMODE: c_uint = 0xf5a01002;
pub const FSTV0910_P1_BERMETER_RESET: c_uint = 0xf5a00001;
// P1_FSPYCFG
pub const RSTV0910_P1_FSPYCFG: c_uint = 0xf5a1;
pub const FSTV0910_P1_FECSPY_INPUT: c_uint = 0xf5a160c0;
pub const FSTV0910_P1_RST_ON_ERROR: c_uint = 0xf5a15020;
pub const FSTV0910_P1_ONE_SHOT: c_uint = 0xf5a14010;
pub const FSTV0910_P1_I2C_MODE: c_uint = 0xf5a1200c;
pub const FSTV0910_P1_SPY_HYSTERESIS: c_uint = 0xf5a10003;
// P1_FSPYDATA
pub const RSTV0910_P1_FSPYDATA: c_uint = 0xf5a2;
pub const FSTV0910_P1_SPY_STUFFING: c_uint = 0xf5a27080;
pub const FSTV0910_P1_SPY_CNULLPKT: c_uint = 0xf5a25020;
pub const FSTV0910_P1_SPY_OUTDATA_MODE: c_uint = 0xf5a2001f;
// P1_FSPYOUT
pub const RSTV0910_P1_FSPYOUT: c_uint = 0xf5a3;
pub const FSTV0910_P1_FSPY_DIRECT: c_uint = 0xf5a37080;
pub const FSTV0910_P1_STUFF_MODE: c_uint = 0xf5a30007;
// P1_FSTATUS
pub const RSTV0910_P1_FSTATUS: c_uint = 0xf5a4;
pub const FSTV0910_P1_SPY_ENDSIM: c_uint = 0xf5a47080;
pub const FSTV0910_P1_VALID_SIM: c_uint = 0xf5a46040;
pub const FSTV0910_P1_FOUND_SIGNAL: c_uint = 0xf5a45020;
pub const FSTV0910_P1_DSS_SYNCBYTE: c_uint = 0xf5a44010;
pub const FSTV0910_P1_RESULT_STATE: c_uint = 0xf5a4000f;
// P1_FBERCPT4
pub const RSTV0910_P1_FBERCPT4: c_uint = 0xf5a8;
pub const FSTV0910_P1_FBERMETER_CPT4: c_uint = 0xf5a800ff;
// P1_FBERCPT3
pub const RSTV0910_P1_FBERCPT3: c_uint = 0xf5a9;
pub const FSTV0910_P1_FBERMETER_CPT3: c_uint = 0xf5a900ff;
// P1_FBERCPT2
pub const RSTV0910_P1_FBERCPT2: c_uint = 0xf5aa;
pub const FSTV0910_P1_FBERMETER_CPT2: c_uint = 0xf5aa00ff;
// P1_FBERCPT1
pub const RSTV0910_P1_FBERCPT1: c_uint = 0xf5ab;
pub const FSTV0910_P1_FBERMETER_CPT1: c_uint = 0xf5ab00ff;
// P1_FBERCPT0
pub const RSTV0910_P1_FBERCPT0: c_uint = 0xf5ac;
pub const FSTV0910_P1_FBERMETER_CPT0: c_uint = 0xf5ac00ff;
// P1_FBERERR2
pub const RSTV0910_P1_FBERERR2: c_uint = 0xf5ad;
pub const FSTV0910_P1_FBERMETER_ERR2: c_uint = 0xf5ad00ff;
// P1_FBERERR1
pub const RSTV0910_P1_FBERERR1: c_uint = 0xf5ae;
pub const FSTV0910_P1_FBERMETER_ERR1: c_uint = 0xf5ae00ff;
// P1_FBERERR0
pub const RSTV0910_P1_FBERERR0: c_uint = 0xf5af;
pub const FSTV0910_P1_FBERMETER_ERR0: c_uint = 0xf5af00ff;
// P1_FSPYBER
pub const RSTV0910_P1_FSPYBER: c_uint = 0xf5b2;
pub const FSTV0910_P1_FSPYBER_SYNCBYTE: c_uint = 0xf5b24010;
pub const FSTV0910_P1_FSPYBER_UNSYNC: c_uint = 0xf5b23008;
pub const FSTV0910_P1_FSPYBER_CTIME: c_uint = 0xf5b20007;
// P1_SFERROR
pub const RSTV0910_P1_SFERROR: c_uint = 0xf5c1;
pub const FSTV0910_P1_SFEC_REGERR_VIT: c_uint = 0xf5c100ff;
// P1_SFECSTATUS
pub const RSTV0910_P1_SFECSTATUS: c_uint = 0xf5c3;
pub const FSTV0910_P1_SFEC_ON: c_uint = 0xf5c37080;
pub const FSTV0910_P1_SFEC_OFF: c_uint = 0xf5c36040;
pub const FSTV0910_P1_LOCKEDSFEC: c_uint = 0xf5c33008;
pub const FSTV0910_P1_SFEC_DELOCK: c_uint = 0xf5c32004;
pub const FSTV0910_P1_SFEC_DEMODSEL: c_uint = 0xf5c31002;
pub const FSTV0910_P1_SFEC_OVFON: c_uint = 0xf5c30001;
// P1_SFKDIV12
pub const RSTV0910_P1_SFKDIV12: c_uint = 0xf5c4;
pub const FSTV0910_P1_SFECKDIV12_MAN: c_uint = 0xf5c47080;
// P1_SFKDIV23
pub const RSTV0910_P1_SFKDIV23: c_uint = 0xf5c5;
pub const FSTV0910_P1_SFECKDIV23_MAN: c_uint = 0xf5c57080;
// P1_SFKDIV34
pub const RSTV0910_P1_SFKDIV34: c_uint = 0xf5c6;
pub const FSTV0910_P1_SFECKDIV34_MAN: c_uint = 0xf5c67080;
// P1_SFKDIV56
pub const RSTV0910_P1_SFKDIV56: c_uint = 0xf5c7;
pub const FSTV0910_P1_SFECKDIV56_MAN: c_uint = 0xf5c77080;
// P1_SFKDIV67
pub const RSTV0910_P1_SFKDIV67: c_uint = 0xf5c8;
pub const FSTV0910_P1_SFECKDIV67_MAN: c_uint = 0xf5c87080;
// P1_SFKDIV78
pub const RSTV0910_P1_SFKDIV78: c_uint = 0xf5c9;
pub const FSTV0910_P1_SFECKDIV78_MAN: c_uint = 0xf5c97080;
// P1_SFSTATUS
pub const RSTV0910_P1_SFSTATUS: c_uint = 0xf5cc;
pub const FSTV0910_P1_SFEC_LINEOK: c_uint = 0xf5cc7080;
pub const FSTV0910_P1_SFEC_ERROR: c_uint = 0xf5cc6040;
pub const FSTV0910_P1_SFEC_DATA7: c_uint = 0xf5cc5020;
pub const FSTV0910_P1_SFEC_PKTDNBRFAIL: c_uint = 0xf5cc4010;
pub const FSTV0910_P1_TSSFEC_DEMODSEL: c_uint = 0xf5cc3008;
pub const FSTV0910_P1_SFEC_NOSYNC: c_uint = 0xf5cc2004;
pub const FSTV0910_P1_SFEC_UNREGULA: c_uint = 0xf5cc1002;
pub const FSTV0910_P1_SFEC_READY: c_uint = 0xf5cc0001;
// P1_SFDLYSET2
pub const RSTV0910_P1_SFDLYSET2: c_uint = 0xf5d0;
pub const FSTV0910_P1_SFEC_DISABLE: c_uint = 0xf5d01002;
// P1_SFERRCTRL
pub const RSTV0910_P1_SFERRCTRL: c_uint = 0xf5d8;
pub const FSTV0910_P1_SFEC_ERR_SOURCE: c_uint = 0xf5d840f0;
pub const FSTV0910_P1_SFEC_NUM_EVENT: c_uint = 0xf5d80007;
// P1_SFERRCNT2
pub const RSTV0910_P1_SFERRCNT2: c_uint = 0xf5d9;
pub const FSTV0910_P1_SFERRC_OLDVALUE: c_uint = 0xf5d97080;
pub const FSTV0910_P1_SFEC_ERR_CNT2: c_uint = 0xf5d9007f;
// P1_SFERRCNT1
pub const RSTV0910_P1_SFERRCNT1: c_uint = 0xf5da;
pub const FSTV0910_P1_SFEC_ERR_CNT1: c_uint = 0xf5da00ff;
// P1_SFERRCNT0
pub const RSTV0910_P1_SFERRCNT0: c_uint = 0xf5db;
pub const FSTV0910_P1_SFEC_ERR_CNT0: c_uint = 0xf5db00ff;
// RCCFG2
pub const RSTV0910_RCCFG2: c_uint = 0xf600;
pub const FSTV0910_TSRCFIFO_DVBCI: c_uint = 0xf6007080;
pub const FSTV0910_TSRCFIFO_SERIAL: c_uint = 0xf6006040;
pub const FSTV0910_TSRCFIFO_DISABLE: c_uint = 0xf6005020;
pub const FSTV0910_TSFIFO_2TORC: c_uint = 0xf6004010;
pub const FSTV0910_TSRCFIFO_HSGNLOUT: c_uint = 0xf6003008;
pub const FSTV0910_TSRCFIFO_ERRMODE: c_uint = 0xf6001006;
// RCCFG1
pub const RSTV0910_RCCFG1: c_uint = 0xf601;
pub const FSTV0910_TSRCFIFO_MANSPEED: c_uint = 0xf60160c0;
pub const FSTV0910_TSRCFIFO_PERMDATA: c_uint = 0xf6015020;
pub const FSTV0910_TSRCFIFO_NONEWSGNL: c_uint = 0xf6014010;
pub const FSTV0910_TSRCFIFO_INVDATA: c_uint = 0xf6010001;
// RCCFG0
pub const RSTV0910_RCCFG0: c_uint = 0xf602;
pub const FSTV0910_TSRCFIFO_BCLKDEL1CK: c_uint = 0xf60260c0;
pub const FSTV0910_TSRCFIFO_DUTY50: c_uint = 0xf6024010;
pub const FSTV0910_TSRCFIFO_NSGNL2DATA: c_uint = 0xf6023008;
pub const FSTV0910_TSRCFIFO_NPDSGNL: c_uint = 0xf6022004;
// RCINSDEL2
pub const RSTV0910_RCINSDEL2: c_uint = 0xf603;
pub const FSTV0910_TSRCDEL_SYNCBYTE: c_uint = 0xf6037080;
pub const FSTV0910_TSRCDEL_XXHEADER: c_uint = 0xf6036040;
pub const FSTV0910_TSRCDEL_BBHEADER: c_uint = 0xf6035020;
pub const FSTV0910_TSRCDEL_DATAFIELD: c_uint = 0xf6034010;
pub const FSTV0910_TSRCINSDEL_ISCR: c_uint = 0xf6033008;
pub const FSTV0910_TSRCINSDEL_NPD: c_uint = 0xf6032004;
pub const FSTV0910_TSRCINSDEL_RSPARITY: c_uint = 0xf6031002;
pub const FSTV0910_TSRCINSDEL_CRC8: c_uint = 0xf6030001;
// RCINSDEL1
pub const RSTV0910_RCINSDEL1: c_uint = 0xf604;
pub const FSTV0910_TSRCINS_BBPADDING: c_uint = 0xf6047080;
pub const FSTV0910_TSRCINS_BCHFEC: c_uint = 0xf6046040;
pub const FSTV0910_TSRCINS_EMODCOD: c_uint = 0xf6044010;
pub const FSTV0910_TSRCINS_TOKEN: c_uint = 0xf6043008;
pub const FSTV0910_TSRCINS_XXXERR: c_uint = 0xf6042004;
pub const FSTV0910_TSRCINS_MATYPE: c_uint = 0xf6041002;
pub const FSTV0910_TSRCINS_UPL: c_uint = 0xf6040001;
// RCINSDEL0
pub const RSTV0910_RCINSDEL0: c_uint = 0xf605;
pub const FSTV0910_TSRCINS_DFL: c_uint = 0xf6057080;
pub const FSTV0910_TSRCINS_SYNCD: c_uint = 0xf6056040;
pub const FSTV0910_TSRCINS_BLOCLEN: c_uint = 0xf6055020;
pub const FSTV0910_TSRCINS_SIGPCOUNT: c_uint = 0xf6054010;
pub const FSTV0910_TSRCINS_FIFO: c_uint = 0xf6053008;
pub const FSTV0910_TSRCINS_REALPACK: c_uint = 0xf6052004;
pub const FSTV0910_TSRCINS_TSCONFIG: c_uint = 0xf6051002;
pub const FSTV0910_TSRCINS_LATENCY: c_uint = 0xf6050001;
// RCSTATUS
pub const RSTV0910_RCSTATUS: c_uint = 0xf606;
pub const FSTV0910_TSRCFIFO_LINEOK: c_uint = 0xf6067080;
pub const FSTV0910_TSRCFIFO_ERROR: c_uint = 0xf6066040;
pub const FSTV0910_TSRCREGUL_ERROR: c_uint = 0xf6064010;
pub const FSTV0910_TSRCFIFO_DEMODSEL: c_uint = 0xf6063008;
pub const FSTV0910_TSRCFIFOSPEED_STORE: c_uint = 0xf6062004;
pub const FSTV0910_TSRCSPEED_IMPOSSIBLE: c_uint = 0xf6060001;
// RCSPEED
pub const RSTV0910_RCSPEED: c_uint = 0xf607;
pub const FSTV0910_TSRCFIFO_OUTSPEED: c_uint = 0xf60700ff;
// TSGENERAL
pub const RSTV0910_TSGENERAL: c_uint = 0xf630;
pub const FSTV0910_TSFIFO_DISTS2PAR: c_uint = 0xf6306040;
pub const FSTV0910_MUXSTREAM_OUTMODE: c_uint = 0xf6303008;
pub const FSTV0910_TSFIFO_PERMPARAL: c_uint = 0xf6301006;
// P1_DISIRQCFG
pub const RSTV0910_P1_DISIRQCFG: c_uint = 0xf700;
pub const FSTV0910_P1_ENRXEND: c_uint = 0xf7006040;
pub const FSTV0910_P1_ENRXFIFO8B: c_uint = 0xf7005020;
pub const FSTV0910_P1_ENTRFINISH: c_uint = 0xf7004010;
pub const FSTV0910_P1_ENTIMEOUT: c_uint = 0xf7003008;
pub const FSTV0910_P1_ENTXEND: c_uint = 0xf7002004;
pub const FSTV0910_P1_ENTXFIFO64B: c_uint = 0xf7001002;
pub const FSTV0910_P1_ENGAPBURST: c_uint = 0xf7000001;
// P1_DISIRQSTAT
pub const RSTV0910_P1_DISIRQSTAT: c_uint = 0xf701;
pub const FSTV0910_P1_IRQRXEND: c_uint = 0xf7016040;
pub const FSTV0910_P1_IRQRXFIFO8B: c_uint = 0xf7015020;
pub const FSTV0910_P1_IRQTRFINISH: c_uint = 0xf7014010;
pub const FSTV0910_P1_IRQTIMEOUT: c_uint = 0xf7013008;
pub const FSTV0910_P1_IRQTXEND: c_uint = 0xf7012004;
pub const FSTV0910_P1_IRQTXFIFO64B: c_uint = 0xf7011002;
pub const FSTV0910_P1_IRQGAPBURST: c_uint = 0xf7010001;
// P1_DISTXCFG
pub const RSTV0910_P1_DISTXCFG: c_uint = 0xf702;
pub const FSTV0910_P1_DISTX_RESET: c_uint = 0xf7027080;
pub const FSTV0910_P1_TIM_OFF: c_uint = 0xf7026040;
pub const FSTV0910_P1_TIM_CMD: c_uint = 0xf7024030;
pub const FSTV0910_P1_ENVELOP: c_uint = 0xf7023008;
pub const FSTV0910_P1_DIS_PRECHARGE: c_uint = 0xf7022004;
pub const FSTV0910_P1_DISEQC_MODE: c_uint = 0xf7020003;
// P1_DISTXSTATUS
pub const RSTV0910_P1_DISTXSTATUS: c_uint = 0xf703;
pub const FSTV0910_P1_TX_FIFO_FULL: c_uint = 0xf7036040;
pub const FSTV0910_P1_TX_IDLE: c_uint = 0xf7035020;
pub const FSTV0910_P1_GAP_BURST: c_uint = 0xf7034010;
pub const FSTV0910_P1_TX_FIFO64B: c_uint = 0xf7033008;
pub const FSTV0910_P1_TX_END: c_uint = 0xf7032004;
pub const FSTV0910_P1_TR_TIMEOUT: c_uint = 0xf7031002;
pub const FSTV0910_P1_TR_FINISH: c_uint = 0xf7030001;
// P1_DISTXBYTES
pub const RSTV0910_P1_DISTXBYTES: c_uint = 0xf704;
pub const FSTV0910_P1_TXFIFO_BYTES: c_uint = 0xf70400ff;
// P1_DISTXFIFO
pub const RSTV0910_P1_DISTXFIFO: c_uint = 0xf705;
pub const FSTV0910_P1_DISEQC_TX_FIFO: c_uint = 0xf70500ff;
// P1_DISTXF22
pub const RSTV0910_P1_DISTXF22: c_uint = 0xf706;
pub const FSTV0910_P1_F22TX: c_uint = 0xf70600ff;
// P1_DISTIMEOCFG
pub const RSTV0910_P1_DISTIMEOCFG: c_uint = 0xf708;
pub const FSTV0910_P1_RXCHOICE: c_uint = 0xf7081006;
pub const FSTV0910_P1_TIMEOUT_OFF: c_uint = 0xf7080001;
// P1_DISTIMEOUT
pub const RSTV0910_P1_DISTIMEOUT: c_uint = 0xf709;
pub const FSTV0910_P1_TIMEOUT_COUNT: c_uint = 0xf70900ff;
// P1_DISRXCFG
pub const RSTV0910_P1_DISRXCFG: c_uint = 0xf70a;
pub const FSTV0910_P1_DISRX_RESET: c_uint = 0xf70a7080;
pub const FSTV0910_P1_EXTENVELOP: c_uint = 0xf70a6040;
pub const FSTV0910_P1_PINSELECT: c_uint = 0xf70a3038;
pub const FSTV0910_P1_IGNORE_SHORT22K: c_uint = 0xf70a2004;
pub const FSTV0910_P1_SIGNED_RXIN: c_uint = 0xf70a1002;
pub const FSTV0910_P1_DISRX_ON: c_uint = 0xf70a0001;
// P1_DISRXSTAT1
pub const RSTV0910_P1_DISRXSTAT1: c_uint = 0xf70b;
pub const FSTV0910_P1_RXEND: c_uint = 0xf70b7080;
pub const FSTV0910_P1_RXACTIVE: c_uint = 0xf70b6040;
pub const FSTV0910_P1_RXDETECT: c_uint = 0xf70b5020;
pub const FSTV0910_P1_CONTTONE: c_uint = 0xf70b4010;
pub const FSTV0910_P1_8BFIFOREADY: c_uint = 0xf70b3008;
pub const FSTV0910_P1_FIFOEMPTY: c_uint = 0xf70b2004;
// P1_DISRXSTAT0
pub const RSTV0910_P1_DISRXSTAT0: c_uint = 0xf70c;
pub const FSTV0910_P1_RXFAIL: c_uint = 0xf70c7080;
pub const FSTV0910_P1_FIFOPFAIL: c_uint = 0xf70c6040;
pub const FSTV0910_P1_RXNONBYTE: c_uint = 0xf70c5020;
pub const FSTV0910_P1_FIFOOVF: c_uint = 0xf70c4010;
pub const FSTV0910_P1_SHORT22K: c_uint = 0xf70c3008;
pub const FSTV0910_P1_RXMSGLOST: c_uint = 0xf70c2004;
// P1_DISRXBYTES
pub const RSTV0910_P1_DISRXBYTES: c_uint = 0xf70d;
pub const FSTV0910_P1_RXFIFO_BYTES: c_uint = 0xf70d001f;
// P1_DISRXPARITY1
pub const RSTV0910_P1_DISRXPARITY1: c_uint = 0xf70e;
pub const FSTV0910_P1_DISRX_PARITY1: c_uint = 0xf70e00ff;
// P1_DISRXPARITY0
pub const RSTV0910_P1_DISRXPARITY0: c_uint = 0xf70f;
pub const FSTV0910_P1_DISRX_PARITY0: c_uint = 0xf70f00ff;
// P1_DISRXFIFO
pub const RSTV0910_P1_DISRXFIFO: c_uint = 0xf710;
pub const FSTV0910_P1_DISEQC_RX_FIFO: c_uint = 0xf71000ff;
// P1_DISRXDC1
pub const RSTV0910_P1_DISRXDC1: c_uint = 0xf711;
pub const FSTV0910_P1_DC_VALUE1: c_uint = 0xf7110103;
// P1_DISRXDC0
pub const RSTV0910_P1_DISRXDC0: c_uint = 0xf712;
pub const FSTV0910_P1_DC_VALUE0: c_uint = 0xf71200ff;
// P1_DISRXF221
pub const RSTV0910_P1_DISRXF221: c_uint = 0xf714;
pub const FSTV0910_P1_F22RX1: c_uint = 0xf714000f;
// P1_DISRXF220
pub const RSTV0910_P1_DISRXF220: c_uint = 0xf715;
pub const FSTV0910_P1_F22RX0: c_uint = 0xf71500ff;
// P1_DISRXF100
pub const RSTV0910_P1_DISRXF100: c_uint = 0xf716;
pub const FSTV0910_P1_F100RX: c_uint = 0xf71600ff;
// P1_DISRXSHORT22K
pub const RSTV0910_P1_DISRXSHORT22K: c_uint = 0xf71c;
pub const FSTV0910_P1_SHORT22K_LENGTH: c_uint = 0xf71c001f;
// P1_ACRPRESC
pub const RSTV0910_P1_ACRPRESC: c_uint = 0xf71e;
pub const FSTV0910_P1_ACR_PRESC: c_uint = 0xf71e0007;
// P1_ACRDIV
pub const RSTV0910_P1_ACRDIV: c_uint = 0xf71f;
pub const FSTV0910_P1_ACR_DIV: c_uint = 0xf71f00ff;
// P2_DISIRQCFG
pub const RSTV0910_P2_DISIRQCFG: c_uint = 0xf740;
pub const FSTV0910_P2_ENRXEND: c_uint = 0xf7406040;
pub const FSTV0910_P2_ENRXFIFO8B: c_uint = 0xf7405020;
pub const FSTV0910_P2_ENTRFINISH: c_uint = 0xf7404010;
pub const FSTV0910_P2_ENTIMEOUT: c_uint = 0xf7403008;
pub const FSTV0910_P2_ENTXEND: c_uint = 0xf7402004;
pub const FSTV0910_P2_ENTXFIFO64B: c_uint = 0xf7401002;
pub const FSTV0910_P2_ENGAPBURST: c_uint = 0xf7400001;
// P2_DISIRQSTAT
pub const RSTV0910_P2_DISIRQSTAT: c_uint = 0xf741;
pub const FSTV0910_P2_IRQRXEND: c_uint = 0xf7416040;
pub const FSTV0910_P2_IRQRXFIFO8B: c_uint = 0xf7415020;
pub const FSTV0910_P2_IRQTRFINISH: c_uint = 0xf7414010;
pub const FSTV0910_P2_IRQTIMEOUT: c_uint = 0xf7413008;
pub const FSTV0910_P2_IRQTXEND: c_uint = 0xf7412004;
pub const FSTV0910_P2_IRQTXFIFO64B: c_uint = 0xf7411002;
pub const FSTV0910_P2_IRQGAPBURST: c_uint = 0xf7410001;
// P2_DISTXCFG
pub const RSTV0910_P2_DISTXCFG: c_uint = 0xf742;
pub const FSTV0910_P2_DISTX_RESET: c_uint = 0xf7427080;
pub const FSTV0910_P2_TIM_OFF: c_uint = 0xf7426040;
pub const FSTV0910_P2_TIM_CMD: c_uint = 0xf7424030;
pub const FSTV0910_P2_ENVELOP: c_uint = 0xf7423008;
pub const FSTV0910_P2_DIS_PRECHARGE: c_uint = 0xf7422004;
pub const FSTV0910_P2_DISEQC_MODE: c_uint = 0xf7420003;
// P2_DISTXSTATUS
pub const RSTV0910_P2_DISTXSTATUS: c_uint = 0xf743;
pub const FSTV0910_P2_TX_FIFO_FULL: c_uint = 0xf7436040;
pub const FSTV0910_P2_TX_IDLE: c_uint = 0xf7435020;
pub const FSTV0910_P2_GAP_BURST: c_uint = 0xf7434010;
pub const FSTV0910_P2_TX_FIFO64B: c_uint = 0xf7433008;
pub const FSTV0910_P2_TX_END: c_uint = 0xf7432004;
pub const FSTV0910_P2_TR_TIMEOUT: c_uint = 0xf7431002;
pub const FSTV0910_P2_TR_FINISH: c_uint = 0xf7430001;
// P2_DISTXBYTES
pub const RSTV0910_P2_DISTXBYTES: c_uint = 0xf744;
pub const FSTV0910_P2_TXFIFO_BYTES: c_uint = 0xf74400ff;
// P2_DISTXFIFO
pub const RSTV0910_P2_DISTXFIFO: c_uint = 0xf745;
pub const FSTV0910_P2_DISEQC_TX_FIFO: c_uint = 0xf74500ff;
// P2_DISTXF22
pub const RSTV0910_P2_DISTXF22: c_uint = 0xf746;
pub const FSTV0910_P2_F22TX: c_uint = 0xf74600ff;
// P2_DISTIMEOCFG
pub const RSTV0910_P2_DISTIMEOCFG: c_uint = 0xf748;
pub const FSTV0910_P2_RXCHOICE: c_uint = 0xf7481006;
pub const FSTV0910_P2_TIMEOUT_OFF: c_uint = 0xf7480001;
// P2_DISTIMEOUT
pub const RSTV0910_P2_DISTIMEOUT: c_uint = 0xf749;
pub const FSTV0910_P2_TIMEOUT_COUNT: c_uint = 0xf74900ff;
// P2_DISRXCFG
pub const RSTV0910_P2_DISRXCFG: c_uint = 0xf74a;
pub const FSTV0910_P2_DISRX_RESET: c_uint = 0xf74a7080;
pub const FSTV0910_P2_EXTENVELOP: c_uint = 0xf74a6040;
pub const FSTV0910_P2_PINSELECT: c_uint = 0xf74a3038;
pub const FSTV0910_P2_IGNORE_SHORT22K: c_uint = 0xf74a2004;
pub const FSTV0910_P2_SIGNED_RXIN: c_uint = 0xf74a1002;
pub const FSTV0910_P2_DISRX_ON: c_uint = 0xf74a0001;
// P2_DISRXSTAT1
pub const RSTV0910_P2_DISRXSTAT1: c_uint = 0xf74b;
pub const FSTV0910_P2_RXEND: c_uint = 0xf74b7080;
pub const FSTV0910_P2_RXACTIVE: c_uint = 0xf74b6040;
pub const FSTV0910_P2_RXDETECT: c_uint = 0xf74b5020;
pub const FSTV0910_P2_CONTTONE: c_uint = 0xf74b4010;
pub const FSTV0910_P2_8BFIFOREADY: c_uint = 0xf74b3008;
pub const FSTV0910_P2_FIFOEMPTY: c_uint = 0xf74b2004;
// P2_DISRXSTAT0
pub const RSTV0910_P2_DISRXSTAT0: c_uint = 0xf74c;
pub const FSTV0910_P2_RXFAIL: c_uint = 0xf74c7080;
pub const FSTV0910_P2_FIFOPFAIL: c_uint = 0xf74c6040;
pub const FSTV0910_P2_RXNONBYTE: c_uint = 0xf74c5020;
pub const FSTV0910_P2_FIFOOVF: c_uint = 0xf74c4010;
pub const FSTV0910_P2_SHORT22K: c_uint = 0xf74c3008;
pub const FSTV0910_P2_RXMSGLOST: c_uint = 0xf74c2004;
// P2_DISRXBYTES
pub const RSTV0910_P2_DISRXBYTES: c_uint = 0xf74d;
pub const FSTV0910_P2_RXFIFO_BYTES: c_uint = 0xf74d001f;
// P2_DISRXPARITY1
pub const RSTV0910_P2_DISRXPARITY1: c_uint = 0xf74e;
pub const FSTV0910_P2_DISRX_PARITY1: c_uint = 0xf74e00ff;
// P2_DISRXPARITY0
pub const RSTV0910_P2_DISRXPARITY0: c_uint = 0xf74f;
pub const FSTV0910_P2_DISRX_PARITY0: c_uint = 0xf74f00ff;
// P2_DISRXFIFO
pub const RSTV0910_P2_DISRXFIFO: c_uint = 0xf750;
pub const FSTV0910_P2_DISEQC_RX_FIFO: c_uint = 0xf75000ff;
// P2_DISRXDC1
pub const RSTV0910_P2_DISRXDC1: c_uint = 0xf751;
pub const FSTV0910_P2_DC_VALUE1: c_uint = 0xf7510103;
// P2_DISRXDC0
pub const RSTV0910_P2_DISRXDC0: c_uint = 0xf752;
pub const FSTV0910_P2_DC_VALUE0: c_uint = 0xf75200ff;
// P2_DISRXF221
pub const RSTV0910_P2_DISRXF221: c_uint = 0xf754;
pub const FSTV0910_P2_F22RX1: c_uint = 0xf754000f;
// P2_DISRXF220
pub const RSTV0910_P2_DISRXF220: c_uint = 0xf755;
pub const FSTV0910_P2_F22RX0: c_uint = 0xf75500ff;
// P2_DISRXF100
pub const RSTV0910_P2_DISRXF100: c_uint = 0xf756;
pub const FSTV0910_P2_F100RX: c_uint = 0xf75600ff;
// P2_DISRXSHORT22K
pub const RSTV0910_P2_DISRXSHORT22K: c_uint = 0xf75c;
pub const FSTV0910_P2_SHORT22K_LENGTH: c_uint = 0xf75c001f;
// P2_ACRPRESC
pub const RSTV0910_P2_ACRPRESC: c_uint = 0xf75e;
pub const FSTV0910_P2_ACR_PRESC: c_uint = 0xf75e0007;
// P2_ACRDIV
pub const RSTV0910_P2_ACRDIV: c_uint = 0xf75f;
pub const FSTV0910_P2_ACR_DIV: c_uint = 0xf75f00ff;
// P1_NBITER_NF1
pub const RSTV0910_P1_NBITER_NF1: c_uint = 0xfa00;
pub const FSTV0910_P1_NBITER_NF_QPSK_1_4: c_uint = 0xfa0000ff;
// P1_NBITER_NF2
pub const RSTV0910_P1_NBITER_NF2: c_uint = 0xfa01;
pub const FSTV0910_P1_NBITER_NF_QPSK_1_3: c_uint = 0xfa0100ff;
// P1_NBITER_NF3
pub const RSTV0910_P1_NBITER_NF3: c_uint = 0xfa02;
pub const FSTV0910_P1_NBITER_NF_QPSK_2_5: c_uint = 0xfa0200ff;
// P1_NBITER_NF4
pub const RSTV0910_P1_NBITER_NF4: c_uint = 0xfa03;
pub const FSTV0910_P1_NBITER_NF_QPSK_1_2: c_uint = 0xfa0300ff;
// P1_NBITER_NF5
pub const RSTV0910_P1_NBITER_NF5: c_uint = 0xfa04;
pub const FSTV0910_P1_NBITER_NF_QPSK_3_5: c_uint = 0xfa0400ff;
// P1_NBITER_NF6
pub const RSTV0910_P1_NBITER_NF6: c_uint = 0xfa05;
pub const FSTV0910_P1_NBITER_NF_QPSK_2_3: c_uint = 0xfa0500ff;
// P1_NBITER_NF7
pub const RSTV0910_P1_NBITER_NF7: c_uint = 0xfa06;
pub const FSTV0910_P1_NBITER_NF_QPSK_3_4: c_uint = 0xfa0600ff;
// P1_NBITER_NF8
pub const RSTV0910_P1_NBITER_NF8: c_uint = 0xfa07;
pub const FSTV0910_P1_NBITER_NF_QPSK_4_5: c_uint = 0xfa0700ff;
// P1_NBITER_NF9
pub const RSTV0910_P1_NBITER_NF9: c_uint = 0xfa08;
pub const FSTV0910_P1_NBITER_NF_QPSK_5_6: c_uint = 0xfa0800ff;
// P1_NBITER_NF10
pub const RSTV0910_P1_NBITER_NF10: c_uint = 0xfa09;
pub const FSTV0910_P1_NBITER_NF_QPSK_8_9: c_uint = 0xfa0900ff;
// P1_NBITER_NF11
pub const RSTV0910_P1_NBITER_NF11: c_uint = 0xfa0a;
pub const FSTV0910_P1_NBITER_NF_QPSK_9_10: c_uint = 0xfa0a00ff;
// P1_NBITER_NF12
pub const RSTV0910_P1_NBITER_NF12: c_uint = 0xfa0b;
pub const FSTV0910_P1_NBITER_NF_8PSK_3_5: c_uint = 0xfa0b00ff;
// P1_NBITER_NF13
pub const RSTV0910_P1_NBITER_NF13: c_uint = 0xfa0c;
pub const FSTV0910_P1_NBITER_NF_8PSK_2_3: c_uint = 0xfa0c00ff;
// P1_NBITER_NF14
pub const RSTV0910_P1_NBITER_NF14: c_uint = 0xfa0d;
pub const FSTV0910_P1_NBITER_NF_8PSK_3_4: c_uint = 0xfa0d00ff;
// P1_NBITER_NF15
pub const RSTV0910_P1_NBITER_NF15: c_uint = 0xfa0e;
pub const FSTV0910_P1_NBITER_NF_8PSK_5_6: c_uint = 0xfa0e00ff;
// P1_NBITER_NF16
pub const RSTV0910_P1_NBITER_NF16: c_uint = 0xfa0f;
pub const FSTV0910_P1_NBITER_NF_8PSK_8_9: c_uint = 0xfa0f00ff;
// P1_NBITER_NF17
pub const RSTV0910_P1_NBITER_NF17: c_uint = 0xfa10;
pub const FSTV0910_P1_NBITER_NF_8PSK_9_10: c_uint = 0xfa1000ff;
// P1_NBITER_NF18
pub const RSTV0910_P1_NBITER_NF18: c_uint = 0xfa11;
pub const FSTV0910_P1_NBITER_NF_16APSK_2_3: c_uint = 0xfa1100ff;
// P1_NBITER_NF19
pub const RSTV0910_P1_NBITER_NF19: c_uint = 0xfa12;
pub const FSTV0910_P1_NBITER_NF_16APSK_3_4: c_uint = 0xfa1200ff;
// P1_NBITER_NF20
pub const RSTV0910_P1_NBITER_NF20: c_uint = 0xfa13;
pub const FSTV0910_P1_NBITER_NF_16APSK_4_5: c_uint = 0xfa1300ff;
// P1_NBITER_NF21
pub const RSTV0910_P1_NBITER_NF21: c_uint = 0xfa14;
pub const FSTV0910_P1_NBITER_NF_16APSK_5_6: c_uint = 0xfa1400ff;
// P1_NBITER_NF22
pub const RSTV0910_P1_NBITER_NF22: c_uint = 0xfa15;
pub const FSTV0910_P1_NBITER_NF_16APSK_8_9: c_uint = 0xfa1500ff;
// P1_NBITER_NF23
pub const RSTV0910_P1_NBITER_NF23: c_uint = 0xfa16;
pub const FSTV0910_P1_NBITER_NF_16APSK_9_10: c_uint = 0xfa1600ff;
// P1_NBITER_NF24
pub const RSTV0910_P1_NBITER_NF24: c_uint = 0xfa17;
pub const FSTV0910_P1_NBITER_NF_32APSK_3_4: c_uint = 0xfa1700ff;
// P1_NBITER_NF25
pub const RSTV0910_P1_NBITER_NF25: c_uint = 0xfa18;
pub const FSTV0910_P1_NBITER_NF_32APSK_4_5: c_uint = 0xfa1800ff;
// P1_NBITER_NF26
pub const RSTV0910_P1_NBITER_NF26: c_uint = 0xfa19;
pub const FSTV0910_P1_NBITER_NF_32APSK_5_6: c_uint = 0xfa1900ff;
// P1_NBITER_NF27
pub const RSTV0910_P1_NBITER_NF27: c_uint = 0xfa1a;
pub const FSTV0910_P1_NBITER_NF_32APSK_8_9: c_uint = 0xfa1a00ff;
// P1_NBITER_NF28
pub const RSTV0910_P1_NBITER_NF28: c_uint = 0xfa1b;
pub const FSTV0910_P1_NBITER_NF_32APSK_9_10: c_uint = 0xfa1b00ff;
// P1_NBITER_SF1
pub const RSTV0910_P1_NBITER_SF1: c_uint = 0xfa1c;
pub const FSTV0910_P1_NBITER_SF_QPSK_1_4: c_uint = 0xfa1c00ff;
// P1_NBITER_SF2
pub const RSTV0910_P1_NBITER_SF2: c_uint = 0xfa1d;
pub const FSTV0910_P1_NBITER_SF_QPSK_1_3: c_uint = 0xfa1d00ff;
// P1_NBITER_SF3
pub const RSTV0910_P1_NBITER_SF3: c_uint = 0xfa1e;
pub const FSTV0910_P1_NBITER_SF_QPSK_2_5: c_uint = 0xfa1e00ff;
// P1_NBITER_SF4
pub const RSTV0910_P1_NBITER_SF4: c_uint = 0xfa1f;
pub const FSTV0910_P1_NBITER_SF_QPSK_1_2: c_uint = 0xfa1f00ff;
// P1_NBITER_SF5
pub const RSTV0910_P1_NBITER_SF5: c_uint = 0xfa20;
pub const FSTV0910_P1_NBITER_SF_QPSK_3_5: c_uint = 0xfa2000ff;
// P1_NBITER_SF6
pub const RSTV0910_P1_NBITER_SF6: c_uint = 0xfa21;
pub const FSTV0910_P1_NBITER_SF_QPSK_2_3: c_uint = 0xfa2100ff;
// P1_NBITER_SF7
pub const RSTV0910_P1_NBITER_SF7: c_uint = 0xfa22;
pub const FSTV0910_P1_NBITER_SF_QPSK_3_4: c_uint = 0xfa2200ff;
// P1_NBITER_SF8
pub const RSTV0910_P1_NBITER_SF8: c_uint = 0xfa23;
pub const FSTV0910_P1_NBITER_SF_QPSK_4_5: c_uint = 0xfa2300ff;
// P1_NBITER_SF9
pub const RSTV0910_P1_NBITER_SF9: c_uint = 0xfa24;
pub const FSTV0910_P1_NBITER_SF_QPSK_5_6: c_uint = 0xfa2400ff;
// P1_NBITER_SF10
pub const RSTV0910_P1_NBITER_SF10: c_uint = 0xfa25;
pub const FSTV0910_P1_NBITER_SF_QPSK_8_9: c_uint = 0xfa2500ff;
// P1_NBITER_SF12
pub const RSTV0910_P1_NBITER_SF12: c_uint = 0xfa26;
pub const FSTV0910_P1_NBITER_SF_8PSK_3_5: c_uint = 0xfa2600ff;
// P1_NBITER_SF13
pub const RSTV0910_P1_NBITER_SF13: c_uint = 0xfa27;
pub const FSTV0910_P1_NBITER_SF_8PSK_2_3: c_uint = 0xfa2700ff;
// P1_NBITER_SF14
pub const RSTV0910_P1_NBITER_SF14: c_uint = 0xfa28;
pub const FSTV0910_P1_NBITER_SF_8PSK_3_4: c_uint = 0xfa2800ff;
// P1_NBITER_SF15
pub const RSTV0910_P1_NBITER_SF15: c_uint = 0xfa29;
pub const FSTV0910_P1_NBITER_SF_8PSK_5_6: c_uint = 0xfa2900ff;
// P1_NBITER_SF16
pub const RSTV0910_P1_NBITER_SF16: c_uint = 0xfa2a;
pub const FSTV0910_P1_NBITER_SF_8PSK_8_9: c_uint = 0xfa2a00ff;
// P1_NBITER_SF18
pub const RSTV0910_P1_NBITER_SF18: c_uint = 0xfa2b;
pub const FSTV0910_P1_NBITER_SF_16APSK_2_3: c_uint = 0xfa2b00ff;
// P1_NBITER_SF19
pub const RSTV0910_P1_NBITER_SF19: c_uint = 0xfa2c;
pub const FSTV0910_P1_NBITER_SF_16APSK_3_4: c_uint = 0xfa2c00ff;
// P1_NBITER_SF20
pub const RSTV0910_P1_NBITER_SF20: c_uint = 0xfa2d;
pub const FSTV0910_P1_NBITER_SF_16APSK_4_5: c_uint = 0xfa2d00ff;
// P1_NBITER_SF21
pub const RSTV0910_P1_NBITER_SF21: c_uint = 0xfa2e;
pub const FSTV0910_P1_NBITER_SF_16APSK_5_6: c_uint = 0xfa2e00ff;
// P1_NBITER_SF22
pub const RSTV0910_P1_NBITER_SF22: c_uint = 0xfa2f;
pub const FSTV0910_P1_NBITER_SF_16APSK_8_9: c_uint = 0xfa2f00ff;
// P1_NBITER_SF24
pub const RSTV0910_P1_NBITER_SF24: c_uint = 0xfa30;
pub const FSTV0910_P1_NBITER_SF_32APSK_3_4: c_uint = 0xfa3000ff;
// P1_NBITER_SF25
pub const RSTV0910_P1_NBITER_SF25: c_uint = 0xfa31;
pub const FSTV0910_P1_NBITER_SF_32APSK_4_5: c_uint = 0xfa3100ff;
// P1_NBITER_SF26
pub const RSTV0910_P1_NBITER_SF26: c_uint = 0xfa32;
pub const FSTV0910_P1_NBITER_SF_32APSK_5_6: c_uint = 0xfa3200ff;
// P1_NBITER_SF27
pub const RSTV0910_P1_NBITER_SF27: c_uint = 0xfa33;
pub const FSTV0910_P1_NBITER_SF_32APSK_8_9: c_uint = 0xfa3300ff;
// SELSATUR6
pub const RSTV0910_SELSATUR6: c_uint = 0xfa34;
pub const FSTV0910_SSAT_SF27: c_uint = 0xfa343008;
pub const FSTV0910_SSAT_SF26: c_uint = 0xfa342004;
pub const FSTV0910_SSAT_SF25: c_uint = 0xfa341002;
pub const FSTV0910_SSAT_SF24: c_uint = 0xfa340001;
// SELSATUR5
pub const RSTV0910_SELSATUR5: c_uint = 0xfa35;
pub const FSTV0910_SSAT_SF22: c_uint = 0xfa357080;
pub const FSTV0910_SSAT_SF21: c_uint = 0xfa356040;
pub const FSTV0910_SSAT_SF20: c_uint = 0xfa355020;
pub const FSTV0910_SSAT_SF19: c_uint = 0xfa354010;
pub const FSTV0910_SSAT_SF18: c_uint = 0xfa353008;
pub const FSTV0910_SSAT_SF16: c_uint = 0xfa352004;
pub const FSTV0910_SSAT_SF15: c_uint = 0xfa351002;
pub const FSTV0910_SSAT_SF14: c_uint = 0xfa350001;
// SELSATUR4
pub const RSTV0910_SELSATUR4: c_uint = 0xfa36;
pub const FSTV0910_SSAT_SF13: c_uint = 0xfa367080;
pub const FSTV0910_SSAT_SF12: c_uint = 0xfa366040;
pub const FSTV0910_SSAT_SF10: c_uint = 0xfa365020;
pub const FSTV0910_SSAT_SF9: c_uint = 0xfa364010;
pub const FSTV0910_SSAT_SF8: c_uint = 0xfa363008;
pub const FSTV0910_SSAT_SF7: c_uint = 0xfa362004;
pub const FSTV0910_SSAT_SF6: c_uint = 0xfa361002;
pub const FSTV0910_SSAT_SF5: c_uint = 0xfa360001;
// SELSATUR3
pub const RSTV0910_SELSATUR3: c_uint = 0xfa37;
pub const FSTV0910_SSAT_SF4: c_uint = 0xfa377080;
pub const FSTV0910_SSAT_SF3: c_uint = 0xfa376040;
pub const FSTV0910_SSAT_SF2: c_uint = 0xfa375020;
pub const FSTV0910_SSAT_SF1: c_uint = 0xfa374010;
pub const FSTV0910_SSAT_NF28: c_uint = 0xfa373008;
pub const FSTV0910_SSAT_NF27: c_uint = 0xfa372004;
pub const FSTV0910_SSAT_NF26: c_uint = 0xfa371002;
pub const FSTV0910_SSAT_NF25: c_uint = 0xfa370001;
// SELSATUR2
pub const RSTV0910_SELSATUR2: c_uint = 0xfa38;
pub const FSTV0910_SSAT_NF24: c_uint = 0xfa387080;
pub const FSTV0910_SSAT_NF23: c_uint = 0xfa386040;
pub const FSTV0910_SSAT_NF22: c_uint = 0xfa385020;
pub const FSTV0910_SSAT_NF21: c_uint = 0xfa384010;
pub const FSTV0910_SSAT_NF20: c_uint = 0xfa383008;
pub const FSTV0910_SSAT_NF19: c_uint = 0xfa382004;
pub const FSTV0910_SSAT_NF18: c_uint = 0xfa381002;
pub const FSTV0910_SSAT_NF17: c_uint = 0xfa380001;
// SELSATUR1
pub const RSTV0910_SELSATUR1: c_uint = 0xfa39;
pub const FSTV0910_SSAT_NF16: c_uint = 0xfa397080;
pub const FSTV0910_SSAT_NF15: c_uint = 0xfa396040;
pub const FSTV0910_SSAT_NF14: c_uint = 0xfa395020;
pub const FSTV0910_SSAT_NF13: c_uint = 0xfa394010;
pub const FSTV0910_SSAT_NF12: c_uint = 0xfa393008;
pub const FSTV0910_SSAT_NF11: c_uint = 0xfa392004;
pub const FSTV0910_SSAT_NF10: c_uint = 0xfa391002;
pub const FSTV0910_SSAT_NF9: c_uint = 0xfa390001;
// SELSATUR0
pub const RSTV0910_SELSATUR0: c_uint = 0xfa3a;
pub const FSTV0910_SSAT_NF8: c_uint = 0xfa3a7080;
pub const FSTV0910_SSAT_NF7: c_uint = 0xfa3a6040;
pub const FSTV0910_SSAT_NF6: c_uint = 0xfa3a5020;
pub const FSTV0910_SSAT_NF5: c_uint = 0xfa3a4010;
pub const FSTV0910_SSAT_NF4: c_uint = 0xfa3a3008;
pub const FSTV0910_SSAT_NF3: c_uint = 0xfa3a2004;
pub const FSTV0910_SSAT_NF2: c_uint = 0xfa3a1002;
pub const FSTV0910_SSAT_NF1: c_uint = 0xfa3a0001;
// GAINLLR_NF1
pub const RSTV0910_GAINLLR_NF1: c_uint = 0xfa40;
pub const FSTV0910_GAINLLR_NF_QPSK_1_4: c_uint = 0xfa40007f;
// GAINLLR_NF2
pub const RSTV0910_GAINLLR_NF2: c_uint = 0xfa41;
pub const FSTV0910_GAINLLR_NF_QPSK_1_3: c_uint = 0xfa41007f;
// GAINLLR_NF3
pub const RSTV0910_GAINLLR_NF3: c_uint = 0xfa42;
pub const FSTV0910_GAINLLR_NF_QPSK_2_5: c_uint = 0xfa42007f;
// GAINLLR_NF4
pub const RSTV0910_GAINLLR_NF4: c_uint = 0xfa43;
pub const FSTV0910_GAINLLR_NF_QPSK_1_2: c_uint = 0xfa43007f;
// GAINLLR_NF5
pub const RSTV0910_GAINLLR_NF5: c_uint = 0xfa44;
pub const FSTV0910_GAINLLR_NF_QPSK_3_5: c_uint = 0xfa44007f;
// GAINLLR_NF6
pub const RSTV0910_GAINLLR_NF6: c_uint = 0xfa45;
pub const FSTV0910_GAINLLR_NF_QPSK_2_3: c_uint = 0xfa45007f;
// GAINLLR_NF7
pub const RSTV0910_GAINLLR_NF7: c_uint = 0xfa46;
pub const FSTV0910_GAINLLR_NF_QPSK_3_4: c_uint = 0xfa46007f;
// GAINLLR_NF8
pub const RSTV0910_GAINLLR_NF8: c_uint = 0xfa47;
pub const FSTV0910_GAINLLR_NF_QPSK_4_5: c_uint = 0xfa47007f;
// GAINLLR_NF9
pub const RSTV0910_GAINLLR_NF9: c_uint = 0xfa48;
pub const FSTV0910_GAINLLR_NF_QPSK_5_6: c_uint = 0xfa48007f;
// GAINLLR_NF10
pub const RSTV0910_GAINLLR_NF10: c_uint = 0xfa49;
pub const FSTV0910_GAINLLR_NF_QPSK_8_9: c_uint = 0xfa49007f;
// GAINLLR_NF11
pub const RSTV0910_GAINLLR_NF11: c_uint = 0xfa4a;
pub const FSTV0910_GAINLLR_NF_QPSK_9_10: c_uint = 0xfa4a007f;
// GAINLLR_NF12
pub const RSTV0910_GAINLLR_NF12: c_uint = 0xfa4b;
pub const FSTV0910_GAINLLR_NF_8PSK_3_5: c_uint = 0xfa4b007f;
// GAINLLR_NF13
pub const RSTV0910_GAINLLR_NF13: c_uint = 0xfa4c;
pub const FSTV0910_GAINLLR_NF_8PSK_2_3: c_uint = 0xfa4c007f;
// GAINLLR_NF14
pub const RSTV0910_GAINLLR_NF14: c_uint = 0xfa4d;
pub const FSTV0910_GAINLLR_NF_8PSK_3_4: c_uint = 0xfa4d007f;
// GAINLLR_NF15
pub const RSTV0910_GAINLLR_NF15: c_uint = 0xfa4e;
pub const FSTV0910_GAINLLR_NF_8PSK_5_6: c_uint = 0xfa4e007f;
// GAINLLR_NF16
pub const RSTV0910_GAINLLR_NF16: c_uint = 0xfa4f;
pub const FSTV0910_GAINLLR_NF_8PSK_8_9: c_uint = 0xfa4f007f;
// GAINLLR_NF17
pub const RSTV0910_GAINLLR_NF17: c_uint = 0xfa50;
pub const FSTV0910_GAINLLR_NF_8PSK_9_10: c_uint = 0xfa50007f;
// GAINLLR_NF18
pub const RSTV0910_GAINLLR_NF18: c_uint = 0xfa51;
pub const FSTV0910_GAINLLR_NF_16APSK_2_3: c_uint = 0xfa51007f;
// GAINLLR_NF19
pub const RSTV0910_GAINLLR_NF19: c_uint = 0xfa52;
pub const FSTV0910_GAINLLR_NF_16APSK_3_4: c_uint = 0xfa52007f;
// GAINLLR_NF20
pub const RSTV0910_GAINLLR_NF20: c_uint = 0xfa53;
pub const FSTV0910_GAINLLR_NF_16APSK_4_5: c_uint = 0xfa53007f;
// GAINLLR_NF21
pub const RSTV0910_GAINLLR_NF21: c_uint = 0xfa54;
pub const FSTV0910_GAINLLR_NF_16APSK_5_6: c_uint = 0xfa54007f;
// GAINLLR_NF22
pub const RSTV0910_GAINLLR_NF22: c_uint = 0xfa55;
pub const FSTV0910_GAINLLR_NF_16APSK_8_9: c_uint = 0xfa55007f;
// GAINLLR_NF23
pub const RSTV0910_GAINLLR_NF23: c_uint = 0xfa56;
pub const FSTV0910_GAINLLR_NF_16APSK_9_10: c_uint = 0xfa56007f;
// GAINLLR_NF24
pub const RSTV0910_GAINLLR_NF24: c_uint = 0xfa57;
pub const FSTV0910_GAINLLR_NF_32APSK_3_4: c_uint = 0xfa57007f;
// GAINLLR_NF25
pub const RSTV0910_GAINLLR_NF25: c_uint = 0xfa58;
pub const FSTV0910_GAINLLR_NF_32APSK_4_5: c_uint = 0xfa58007f;
// GAINLLR_NF26
pub const RSTV0910_GAINLLR_NF26: c_uint = 0xfa59;
pub const FSTV0910_GAINLLR_NF_32APSK_5_6: c_uint = 0xfa59007f;
// GAINLLR_NF27
pub const RSTV0910_GAINLLR_NF27: c_uint = 0xfa5a;
pub const FSTV0910_GAINLLR_NF_32APSK_8_9: c_uint = 0xfa5a007f;
// GAINLLR_NF28
pub const RSTV0910_GAINLLR_NF28: c_uint = 0xfa5b;
pub const FSTV0910_GAINLLR_NF_32APSK_9_10: c_uint = 0xfa5b007f;
// GAINLLR_SF1
pub const RSTV0910_GAINLLR_SF1: c_uint = 0xfa5c;
pub const FSTV0910_GAINLLR_SF_QPSK_1_4: c_uint = 0xfa5c007f;
// GAINLLR_SF2
pub const RSTV0910_GAINLLR_SF2: c_uint = 0xfa5d;
pub const FSTV0910_GAINLLR_SF_QPSK_1_3: c_uint = 0xfa5d007f;
// GAINLLR_SF3
pub const RSTV0910_GAINLLR_SF3: c_uint = 0xfa5e;
pub const FSTV0910_GAINLLR_SF_QPSK_2_5: c_uint = 0xfa5e007f;
// GAINLLR_SF4
pub const RSTV0910_GAINLLR_SF4: c_uint = 0xfa5f;
pub const FSTV0910_GAINLLR_SF_QPSK_1_2: c_uint = 0xfa5f007f;
// GAINLLR_SF5
pub const RSTV0910_GAINLLR_SF5: c_uint = 0xfa60;
pub const FSTV0910_GAINLLR_SF_QPSK_3_5: c_uint = 0xfa60007f;
// GAINLLR_SF6
pub const RSTV0910_GAINLLR_SF6: c_uint = 0xfa61;
pub const FSTV0910_GAINLLR_SF_QPSK_2_3: c_uint = 0xfa61007f;
// GAINLLR_SF7
pub const RSTV0910_GAINLLR_SF7: c_uint = 0xfa62;
pub const FSTV0910_GAINLLR_SF_QPSK_3_4: c_uint = 0xfa62007f;
// GAINLLR_SF8
pub const RSTV0910_GAINLLR_SF8: c_uint = 0xfa63;
pub const FSTV0910_GAINLLR_SF_QPSK_4_5: c_uint = 0xfa63007f;
// GAINLLR_SF9
pub const RSTV0910_GAINLLR_SF9: c_uint = 0xfa64;
pub const FSTV0910_GAINLLR_SF_QPSK_5_6: c_uint = 0xfa64007f;
// GAINLLR_SF10
pub const RSTV0910_GAINLLR_SF10: c_uint = 0xfa65;
pub const FSTV0910_GAINLLR_SF_QPSK_8_9: c_uint = 0xfa65007f;
// GAINLLR_SF12
pub const RSTV0910_GAINLLR_SF12: c_uint = 0xfa66;
pub const FSTV0910_GAINLLR_SF_8PSK_3_5: c_uint = 0xfa66007f;
// GAINLLR_SF13
pub const RSTV0910_GAINLLR_SF13: c_uint = 0xfa67;
pub const FSTV0910_GAINLLR_SF_8PSK_2_3: c_uint = 0xfa67007f;
// GAINLLR_SF14
pub const RSTV0910_GAINLLR_SF14: c_uint = 0xfa68;
pub const FSTV0910_GAINLLR_SF_8PSK_3_4: c_uint = 0xfa68007f;
// GAINLLR_SF15
pub const RSTV0910_GAINLLR_SF15: c_uint = 0xfa69;
pub const FSTV0910_GAINLLR_SF_8PSK_5_6: c_uint = 0xfa69007f;
// GAINLLR_SF16
pub const RSTV0910_GAINLLR_SF16: c_uint = 0xfa6a;
pub const FSTV0910_GAINLLR_SF_8PSK_8_9: c_uint = 0xfa6a007f;
// GAINLLR_SF18
pub const RSTV0910_GAINLLR_SF18: c_uint = 0xfa6b;
pub const FSTV0910_GAINLLR_SF_16APSK_2_3: c_uint = 0xfa6b007f;
// GAINLLR_SF19
pub const RSTV0910_GAINLLR_SF19: c_uint = 0xfa6c;
pub const FSTV0910_GAINLLR_SF_16APSK_3_4: c_uint = 0xfa6c007f;
// GAINLLR_SF20
pub const RSTV0910_GAINLLR_SF20: c_uint = 0xfa6d;
pub const FSTV0910_GAINLLR_SF_16APSK_4_5: c_uint = 0xfa6d007f;
// GAINLLR_SF21
pub const RSTV0910_GAINLLR_SF21: c_uint = 0xfa6e;
pub const FSTV0910_GAINLLR_SF_16APSK_5_6: c_uint = 0xfa6e007f;
// GAINLLR_SF22
pub const RSTV0910_GAINLLR_SF22: c_uint = 0xfa6f;
pub const FSTV0910_GAINLLR_SF_16APSK_8_9: c_uint = 0xfa6f007f;
// GAINLLR_SF24
pub const RSTV0910_GAINLLR_SF24: c_uint = 0xfa70;
pub const FSTV0910_GAINLLR_SF_32APSK_3_4: c_uint = 0xfa70007f;
// GAINLLR_SF25
pub const RSTV0910_GAINLLR_SF25: c_uint = 0xfa71;
pub const FSTV0910_GAINLLR_SF_32APSK_4_5: c_uint = 0xfa71007f;
// GAINLLR_SF26
pub const RSTV0910_GAINLLR_SF26: c_uint = 0xfa72;
pub const FSTV0910_GAINLLR_SF_32APSK_5_6: c_uint = 0xfa72007f;
// GAINLLR_SF27
pub const RSTV0910_GAINLLR_SF27: c_uint = 0xfa73;
pub const FSTV0910_GAINLLR_SF_32APSK_8_9: c_uint = 0xfa73007f;
// CFGEXT
pub const RSTV0910_CFGEXT: c_uint = 0xfa80;
pub const FSTV0910_BYPBCH: c_uint = 0xfa806040;
pub const FSTV0910_BYPLDPC: c_uint = 0xfa805020;
pub const FSTV0910_SHORTMULT: c_uint = 0xfa802004;
// GENCFG
pub const RSTV0910_GENCFG: c_uint = 0xfa86;
pub const FSTV0910_BROADCAST: c_uint = 0xfa864010;
pub const FSTV0910_CROSSINPUT: c_uint = 0xfa861002;
pub const FSTV0910_DDEMOD: c_uint = 0xfa860001;
// LDPCERR1
pub const RSTV0910_LDPCERR1: c_uint = 0xfa96;
pub const FSTV0910_LDPC_ERRORS1: c_uint = 0xfa9600ff;
// LDPCERR0
pub const RSTV0910_LDPCERR0: c_uint = 0xfa97;
pub const FSTV0910_LDPC_ERRORS0: c_uint = 0xfa9700ff;
// BCHERR
pub const RSTV0910_BCHERR: c_uint = 0xfa98;
pub const FSTV0910_ERRORFLAG: c_uint = 0xfa984010;
pub const FSTV0910_BCH_ERRORS_COUNTER: c_uint = 0xfa98000f;
// P1_MAXEXTRAITER
pub const RSTV0910_P1_MAXEXTRAITER: c_uint = 0xfab1;
pub const FSTV0910_P1_MAX_EXTRA_ITER: c_uint = 0xfab100ff;
// P2_MAXEXTRAITER
pub const RSTV0910_P2_MAXEXTRAITER: c_uint = 0xfab6;
pub const FSTV0910_P2_MAX_EXTRA_ITER: c_uint = 0xfab600ff;
// P1_STATUSITER
pub const RSTV0910_P1_STATUSITER: c_uint = 0xfabc;
pub const FSTV0910_P1_STATUS_ITER: c_uint = 0xfabc00ff;
// P1_STATUSMAXITER
pub const RSTV0910_P1_STATUSMAXITER: c_uint = 0xfabd;
pub const FSTV0910_P1_STATUS_MAX_ITER: c_uint = 0xfabd00ff;
// P2_STATUSITER
pub const RSTV0910_P2_STATUSITER: c_uint = 0xfabe;
pub const FSTV0910_P2_STATUS_ITER: c_uint = 0xfabe00ff;
// P2_STATUSMAXITER
pub const RSTV0910_P2_STATUSMAXITER: c_uint = 0xfabf;
pub const FSTV0910_P2_STATUS_MAX_ITER: c_uint = 0xfabf00ff;
// P2_NBITER_NF1
pub const RSTV0910_P2_NBITER_NF1: c_uint = 0xfac0;
pub const FSTV0910_P2_NBITER_NF_QPSK_1_4: c_uint = 0xfac000ff;
// P2_NBITER_NF2
pub const RSTV0910_P2_NBITER_NF2: c_uint = 0xfac1;
pub const FSTV0910_P2_NBITER_NF_QPSK_1_3: c_uint = 0xfac100ff;
// P2_NBITER_NF3
pub const RSTV0910_P2_NBITER_NF3: c_uint = 0xfac2;
pub const FSTV0910_P2_NBITER_NF_QPSK_2_5: c_uint = 0xfac200ff;
// P2_NBITER_NF4
pub const RSTV0910_P2_NBITER_NF4: c_uint = 0xfac3;
pub const FSTV0910_P2_NBITER_NF_QPSK_1_2: c_uint = 0xfac300ff;
// P2_NBITER_NF5
pub const RSTV0910_P2_NBITER_NF5: c_uint = 0xfac4;
pub const FSTV0910_P2_NBITER_NF_QPSK_3_5: c_uint = 0xfac400ff;
// P2_NBITER_NF6
pub const RSTV0910_P2_NBITER_NF6: c_uint = 0xfac5;
pub const FSTV0910_P2_NBITER_NF_QPSK_2_3: c_uint = 0xfac500ff;
// P2_NBITER_NF7
pub const RSTV0910_P2_NBITER_NF7: c_uint = 0xfac6;
pub const FSTV0910_P2_NBITER_NF_QPSK_3_4: c_uint = 0xfac600ff;
// P2_NBITER_NF8
pub const RSTV0910_P2_NBITER_NF8: c_uint = 0xfac7;
pub const FSTV0910_P2_NBITER_NF_QPSK_4_5: c_uint = 0xfac700ff;
// P2_NBITER_NF9
pub const RSTV0910_P2_NBITER_NF9: c_uint = 0xfac8;
pub const FSTV0910_P2_NBITER_NF_QPSK_5_6: c_uint = 0xfac800ff;
// P2_NBITER_NF10
pub const RSTV0910_P2_NBITER_NF10: c_uint = 0xfac9;
pub const FSTV0910_P2_NBITER_NF_QPSK_8_9: c_uint = 0xfac900ff;
// P2_NBITER_NF11
pub const RSTV0910_P2_NBITER_NF11: c_uint = 0xfaca;
pub const FSTV0910_P2_NBITER_NF_QPSK_9_10: c_uint = 0xfaca00ff;
// P2_NBITER_NF12
pub const RSTV0910_P2_NBITER_NF12: c_uint = 0xfacb;
pub const FSTV0910_P2_NBITER_NF_8PSK_3_5: c_uint = 0xfacb00ff;
// P2_NBITER_NF13
pub const RSTV0910_P2_NBITER_NF13: c_uint = 0xfacc;
pub const FSTV0910_P2_NBITER_NF_8PSK_2_3: c_uint = 0xfacc00ff;
// P2_NBITER_NF14
pub const RSTV0910_P2_NBITER_NF14: c_uint = 0xfacd;
pub const FSTV0910_P2_NBITER_NF_8PSK_3_4: c_uint = 0xfacd00ff;
// P2_NBITER_NF15
pub const RSTV0910_P2_NBITER_NF15: c_uint = 0xface;
pub const FSTV0910_P2_NBITER_NF_8PSK_5_6: c_uint = 0xface00ff;
// P2_NBITER_NF16
pub const RSTV0910_P2_NBITER_NF16: c_uint = 0xfacf;
pub const FSTV0910_P2_NBITER_NF_8PSK_8_9: c_uint = 0xfacf00ff;
// P2_NBITER_NF17
pub const RSTV0910_P2_NBITER_NF17: c_uint = 0xfad0;
pub const FSTV0910_P2_NBITER_NF_8PSK_9_10: c_uint = 0xfad000ff;
// P2_NBITER_NF18
pub const RSTV0910_P2_NBITER_NF18: c_uint = 0xfad1;
pub const FSTV0910_P2_NBITER_NF_16APSK_2_3: c_uint = 0xfad100ff;
// P2_NBITER_NF19
pub const RSTV0910_P2_NBITER_NF19: c_uint = 0xfad2;
pub const FSTV0910_P2_NBITER_NF_16APSK_3_4: c_uint = 0xfad200ff;
// P2_NBITER_NF20
pub const RSTV0910_P2_NBITER_NF20: c_uint = 0xfad3;
pub const FSTV0910_P2_NBITER_NF_16APSK_4_5: c_uint = 0xfad300ff;
// P2_NBITER_NF21
pub const RSTV0910_P2_NBITER_NF21: c_uint = 0xfad4;
pub const FSTV0910_P2_NBITER_NF_16APSK_5_6: c_uint = 0xfad400ff;
// P2_NBITER_NF22
pub const RSTV0910_P2_NBITER_NF22: c_uint = 0xfad5;
pub const FSTV0910_P2_NBITER_NF_16APSK_8_9: c_uint = 0xfad500ff;
// P2_NBITER_NF23
pub const RSTV0910_P2_NBITER_NF23: c_uint = 0xfad6;
pub const FSTV0910_P2_NBITER_NF_16APSK_9_10: c_uint = 0xfad600ff;
// P2_NBITER_NF24
pub const RSTV0910_P2_NBITER_NF24: c_uint = 0xfad7;
pub const FSTV0910_P2_NBITER_NF_32APSK_3_4: c_uint = 0xfad700ff;
// P2_NBITER_NF25
pub const RSTV0910_P2_NBITER_NF25: c_uint = 0xfad8;
pub const FSTV0910_P2_NBITER_NF_32APSK_4_5: c_uint = 0xfad800ff;
// P2_NBITER_NF26
pub const RSTV0910_P2_NBITER_NF26: c_uint = 0xfad9;
pub const FSTV0910_P2_NBITER_NF_32APSK_5_6: c_uint = 0xfad900ff;
// P2_NBITER_NF27
pub const RSTV0910_P2_NBITER_NF27: c_uint = 0xfada;
pub const FSTV0910_P2_NBITER_NF_32APSK_8_9: c_uint = 0xfada00ff;
// P2_NBITER_NF28
pub const RSTV0910_P2_NBITER_NF28: c_uint = 0xfadb;
pub const FSTV0910_P2_NBITER_NF_32APSK_9_10: c_uint = 0xfadb00ff;
// P2_NBITER_SF1
pub const RSTV0910_P2_NBITER_SF1: c_uint = 0xfadc;
pub const FSTV0910_P2_NBITER_SF_QPSK_1_4: c_uint = 0xfadc00ff;
// P2_NBITER_SF2
pub const RSTV0910_P2_NBITER_SF2: c_uint = 0xfadd;
pub const FSTV0910_P2_NBITER_SF_QPSK_1_3: c_uint = 0xfadd00ff;
// P2_NBITER_SF3
pub const RSTV0910_P2_NBITER_SF3: c_uint = 0xfade;
pub const FSTV0910_P2_NBITER_SF_QPSK_2_5: c_uint = 0xfade00ff;
// P2_NBITER_SF4
pub const RSTV0910_P2_NBITER_SF4: c_uint = 0xfadf;
pub const FSTV0910_P2_NBITER_SF_QPSK_1_2: c_uint = 0xfadf00ff;
// P2_NBITER_SF5
pub const RSTV0910_P2_NBITER_SF5: c_uint = 0xfae0;
pub const FSTV0910_P2_NBITER_SF_QPSK_3_5: c_uint = 0xfae000ff;
// P2_NBITER_SF6
pub const RSTV0910_P2_NBITER_SF6: c_uint = 0xfae1;
pub const FSTV0910_P2_NBITER_SF_QPSK_2_3: c_uint = 0xfae100ff;
// P2_NBITER_SF7
pub const RSTV0910_P2_NBITER_SF7: c_uint = 0xfae2;
pub const FSTV0910_P2_NBITER_SF_QPSK_3_4: c_uint = 0xfae200ff;
// P2_NBITER_SF8
pub const RSTV0910_P2_NBITER_SF8: c_uint = 0xfae3;
pub const FSTV0910_P2_NBITER_SF_QPSK_4_5: c_uint = 0xfae300ff;
// P2_NBITER_SF9
pub const RSTV0910_P2_NBITER_SF9: c_uint = 0xfae4;
pub const FSTV0910_P2_NBITER_SF_QPSK_5_6: c_uint = 0xfae400ff;
// P2_NBITER_SF10
pub const RSTV0910_P2_NBITER_SF10: c_uint = 0xfae5;
pub const FSTV0910_P2_NBITER_SF_QPSK_8_9: c_uint = 0xfae500ff;
// P2_NBITER_SF12
pub const RSTV0910_P2_NBITER_SF12: c_uint = 0xfae6;
pub const FSTV0910_P2_NBITER_SF_8PSK_3_5: c_uint = 0xfae600ff;
// P2_NBITER_SF13
pub const RSTV0910_P2_NBITER_SF13: c_uint = 0xfae7;
pub const FSTV0910_P2_NBITER_SF_8PSK_2_3: c_uint = 0xfae700ff;
// P2_NBITER_SF14
pub const RSTV0910_P2_NBITER_SF14: c_uint = 0xfae8;
pub const FSTV0910_P2_NBITER_SF_8PSK_3_4: c_uint = 0xfae800ff;
// P2_NBITER_SF15
pub const RSTV0910_P2_NBITER_SF15: c_uint = 0xfae9;
pub const FSTV0910_P2_NBITER_SF_8PSK_5_6: c_uint = 0xfae900ff;
// P2_NBITER_SF16
pub const RSTV0910_P2_NBITER_SF16: c_uint = 0xfaea;
pub const FSTV0910_P2_NBITER_SF_8PSK_8_9: c_uint = 0xfaea00ff;
// P2_NBITER_SF18
pub const RSTV0910_P2_NBITER_SF18: c_uint = 0xfaeb;
pub const FSTV0910_P2_NBITER_SF_16APSK_2_3: c_uint = 0xfaeb00ff;
// P2_NBITER_SF19
pub const RSTV0910_P2_NBITER_SF19: c_uint = 0xfaec;
pub const FSTV0910_P2_NBITER_SF_16APSK_3_4: c_uint = 0xfaec00ff;
// P2_NBITER_SF20
pub const RSTV0910_P2_NBITER_SF20: c_uint = 0xfaed;
pub const FSTV0910_P2_NBITER_SF_16APSK_4_5: c_uint = 0xfaed00ff;
// P2_NBITER_SF21
pub const RSTV0910_P2_NBITER_SF21: c_uint = 0xfaee;
pub const FSTV0910_P2_NBITER_SF_16APSK_5_6: c_uint = 0xfaee00ff;
// P2_NBITER_SF22
pub const RSTV0910_P2_NBITER_SF22: c_uint = 0xfaef;
pub const FSTV0910_P2_NBITER_SF_16APSK_8_9: c_uint = 0xfaef00ff;
// P2_NBITER_SF24
pub const RSTV0910_P2_NBITER_SF24: c_uint = 0xfaf0;
pub const FSTV0910_P2_NBITER_SF_32APSK_3_4: c_uint = 0xfaf000ff;
// P2_NBITER_SF25
pub const RSTV0910_P2_NBITER_SF25: c_uint = 0xfaf1;
pub const FSTV0910_P2_NBITER_SF_32APSK_4_5: c_uint = 0xfaf100ff;
// P2_NBITER_SF26
pub const RSTV0910_P2_NBITER_SF26: c_uint = 0xfaf2;
pub const FSTV0910_P2_NBITER_SF_32APSK_5_6: c_uint = 0xfaf200ff;
// P2_NBITER_SF27
pub const RSTV0910_P2_NBITER_SF27: c_uint = 0xfaf3;
pub const FSTV0910_P2_NBITER_SF_32APSK_8_9: c_uint = 0xfaf300ff;
// TSTRES0
pub const RSTV0910_TSTRES0: c_uint = 0xff11;
pub const FSTV0910_FRESFEC: c_uint = 0xff117080;
pub const FSTV0910_FRESSYM1: c_uint = 0xff113008;
pub const FSTV0910_FRESSYM2: c_uint = 0xff112004;
// TSTOUT
pub const RSTV0910_TSTOUT: c_uint = 0xff12;
pub const FSTV0910_TS: c_uint = 0xff12103e;
pub const FSTV0910_TEST_OUT: c_uint = 0xff120001;
// TSTIN
pub const RSTV0910_TSTIN: c_uint = 0xff13;
pub const FSTV0910_TEST_IN: c_uint = 0xff137080;
// P2_TSTDMD
pub const RSTV0910_P2_TSTDMD: c_uint = 0xff20;
pub const FSTV0910_P2_CFRINIT_INVZIGZAG: c_uint = 0xff203008;
// P2_TCTL1
pub const RSTV0910_P2_TCTL1: c_uint = 0xff24;
pub const FSTV0910_P2_TST_IQSYMBSEL: c_uint = 0xff24001f;
// P2_TCTL4
pub const RSTV0910_P2_TCTL4: c_uint = 0xff28;
pub const FSTV0910_P2_CFR2TOCFR1_DVBS1: c_uint = 0xff2860c0;
// P2_TPKTDELIN
pub const RSTV0910_P2_TPKTDELIN: c_uint = 0xff37;
pub const FSTV0910_P2_CFG_RSPARITYON: c_uint = 0xff377080;
// P1_TSTDMD
pub const RSTV0910_P1_TSTDMD: c_uint = 0xff40;
pub const FSTV0910_P1_CFRINIT_INVZIGZAG: c_uint = 0xff403008;
// P1_TCTL1
pub const RSTV0910_P1_TCTL1: c_uint = 0xff44;
pub const FSTV0910_P1_TST_IQSYMBSEL: c_uint = 0xff44001f;
// P1_TCTL4
pub const RSTV0910_P1_TCTL4: c_uint = 0xff48;
pub const FSTV0910_P1_CFR2TOCFR1_DVBS1: c_uint = 0xff4860c0;
// P1_TPKTDELIN
pub const RSTV0910_P1_TPKTDELIN: c_uint = 0xff57;
pub const FSTV0910_P1_CFG_RSPARITYON: c_uint = 0xff577080;
// TSTTSRS
pub const RSTV0910_TSTTSRS: c_uint = 0xff6d;
pub const FSTV0910_TSTRS_DISRS2: c_uint = 0xff6d1002;
pub const FSTV0910_TSTRS_DISRS1: c_uint = 0xff6d0001;
pub const STV0910_NBREGS: c_int = 975;
pub const STV0910_NBFIELDS: c_int = 1818;
