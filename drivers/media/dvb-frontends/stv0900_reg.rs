//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0900_reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// stv0900_reg.h
//
// Driver for ST STV0900 satellite demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
//
extern "C" {
    pub fn shiftx(x: i32, demod: c_int, shift: i32) -> i32;
}

// MID
pub const R0900_MID: c_uint = 0xf100;
pub const F0900_MCHIP_IDENT: c_uint = 0xf10000f0;
pub const F0900_MRELEASE: c_uint = 0xf100000f;
// DACR1
pub const R0900_DACR1: c_uint = 0xf113;
pub const F0900_DAC_MODE: c_uint = 0xf11300e0;
pub const F0900_DAC_VALUE1: c_uint = 0xf113000f;
// DACR2
pub const R0900_DACR2: c_uint = 0xf114;
pub const F0900_DAC_VALUE0: c_uint = 0xf11400ff;
// OUTCFG
pub const R0900_OUTCFG: c_uint = 0xf11c;
pub const F0900_OUTSERRS1_HZ: c_uint = 0xf11c0040;
pub const F0900_OUTSERRS2_HZ: c_uint = 0xf11c0020;
pub const F0900_OUTSERRS3_HZ: c_uint = 0xf11c0010;
pub const F0900_OUTPARRS3_HZ: c_uint = 0xf11c0008;
// IRQSTATUS3
pub const R0900_IRQSTATUS3: c_uint = 0xf120;
pub const F0900_SPLL_LOCK: c_uint = 0xf1200020;
pub const F0900_SSTREAM_LCK_3: c_uint = 0xf1200010;
pub const F0900_SSTREAM_LCK_2: c_uint = 0xf1200008;
pub const F0900_SSTREAM_LCK_1: c_uint = 0xf1200004;
pub const F0900_SDVBS1_PRF_2: c_uint = 0xf1200002;
pub const F0900_SDVBS1_PRF_1: c_uint = 0xf1200001;
// IRQSTATUS2
pub const R0900_IRQSTATUS2: c_uint = 0xf121;
pub const F0900_SSPY_ENDSIM_3: c_uint = 0xf1210080;
pub const F0900_SSPY_ENDSIM_2: c_uint = 0xf1210040;
pub const F0900_SSPY_ENDSIM_1: c_uint = 0xf1210020;
pub const F0900_SPKTDEL_ERROR_2: c_uint = 0xf1210010;
pub const F0900_SPKTDEL_LOCKB_2: c_uint = 0xf1210008;
pub const F0900_SPKTDEL_LOCK_2: c_uint = 0xf1210004;
pub const F0900_SPKTDEL_ERROR_1: c_uint = 0xf1210002;
pub const F0900_SPKTDEL_LOCKB_1: c_uint = 0xf1210001;
// IRQSTATUS1
pub const R0900_IRQSTATUS1: c_uint = 0xf122;
pub const F0900_SPKTDEL_LOCK_1: c_uint = 0xf1220080;
pub const F0900_SDEMOD_LOCKB_2: c_uint = 0xf1220004;
pub const F0900_SDEMOD_LOCK_2: c_uint = 0xf1220002;
pub const F0900_SDEMOD_IRQ_2: c_uint = 0xf1220001;
// IRQSTATUS0
pub const R0900_IRQSTATUS0: c_uint = 0xf123;
pub const F0900_SDEMOD_LOCKB_1: c_uint = 0xf1230080;
pub const F0900_SDEMOD_LOCK_1: c_uint = 0xf1230040;
pub const F0900_SDEMOD_IRQ_1: c_uint = 0xf1230020;
pub const F0900_SBCH_ERRFLAG: c_uint = 0xf1230010;
pub const F0900_SDISEQC2RX_IRQ: c_uint = 0xf1230008;
pub const F0900_SDISEQC2TX_IRQ: c_uint = 0xf1230004;
pub const F0900_SDISEQC1RX_IRQ: c_uint = 0xf1230002;
pub const F0900_SDISEQC1TX_IRQ: c_uint = 0xf1230001;
// IRQMASK3
pub const R0900_IRQMASK3: c_uint = 0xf124;
pub const F0900_MPLL_LOCK: c_uint = 0xf1240020;
pub const F0900_MSTREAM_LCK_3: c_uint = 0xf1240010;
pub const F0900_MSTREAM_LCK_2: c_uint = 0xf1240008;
pub const F0900_MSTREAM_LCK_1: c_uint = 0xf1240004;
pub const F0900_MDVBS1_PRF_2: c_uint = 0xf1240002;
pub const F0900_MDVBS1_PRF_1: c_uint = 0xf1240001;
// IRQMASK2
pub const R0900_IRQMASK2: c_uint = 0xf125;
pub const F0900_MSPY_ENDSIM_3: c_uint = 0xf1250080;
pub const F0900_MSPY_ENDSIM_2: c_uint = 0xf1250040;
pub const F0900_MSPY_ENDSIM_1: c_uint = 0xf1250020;
pub const F0900_MPKTDEL_ERROR_2: c_uint = 0xf1250010;
pub const F0900_MPKTDEL_LOCKB_2: c_uint = 0xf1250008;
pub const F0900_MPKTDEL_LOCK_2: c_uint = 0xf1250004;
pub const F0900_MPKTDEL_ERROR_1: c_uint = 0xf1250002;
pub const F0900_MPKTDEL_LOCKB_1: c_uint = 0xf1250001;
// IRQMASK1
pub const R0900_IRQMASK1: c_uint = 0xf126;
pub const F0900_MPKTDEL_LOCK_1: c_uint = 0xf1260080;
pub const F0900_MEXTPINB2: c_uint = 0xf1260040;
pub const F0900_MEXTPIN2: c_uint = 0xf1260020;
pub const F0900_MEXTPINB1: c_uint = 0xf1260010;
pub const F0900_MEXTPIN1: c_uint = 0xf1260008;
pub const F0900_MDEMOD_LOCKB_2: c_uint = 0xf1260004;
pub const F0900_MDEMOD_LOCK_2: c_uint = 0xf1260002;
pub const F0900_MDEMOD_IRQ_2: c_uint = 0xf1260001;
// IRQMASK0
pub const R0900_IRQMASK0: c_uint = 0xf127;
pub const F0900_MDEMOD_LOCKB_1: c_uint = 0xf1270080;
pub const F0900_MDEMOD_LOCK_1: c_uint = 0xf1270040;
pub const F0900_MDEMOD_IRQ_1: c_uint = 0xf1270020;
pub const F0900_MBCH_ERRFLAG: c_uint = 0xf1270010;
pub const F0900_MDISEQC2RX_IRQ: c_uint = 0xf1270008;
pub const F0900_MDISEQC2TX_IRQ: c_uint = 0xf1270004;
pub const F0900_MDISEQC1RX_IRQ: c_uint = 0xf1270002;
pub const F0900_MDISEQC1TX_IRQ: c_uint = 0xf1270001;
// I2CCFG
pub const R0900_I2CCFG: c_uint = 0xf129;
pub const F0900_I2C_FASTMODE: c_uint = 0xf1290008;
pub const F0900_I2CADDR_INC: c_uint = 0xf1290003;
// P1_I2CRPT
pub const R0900_P1_I2CRPT: c_uint = 0xf12a;

pub const F0900_P1_I2CT_ON: c_uint = 0xf12a0080;

pub const F0900_P1_ENARPT_LEVEL: c_uint = 0xf12a0070;
pub const F0900_P1_SCLT_DELAY: c_uint = 0xf12a0008;
pub const F0900_P1_STOP_ENABLE: c_uint = 0xf12a0004;
pub const F0900_P1_STOP_SDAT2SDA: c_uint = 0xf12a0002;
// P2_I2CRPT
pub const R0900_P2_I2CRPT: c_uint = 0xf12b;
pub const F0900_P2_I2CT_ON: c_uint = 0xf12b0080;
pub const F0900_P2_ENARPT_LEVEL: c_uint = 0xf12b0070;
pub const F0900_P2_SCLT_DELAY: c_uint = 0xf12b0008;
pub const F0900_P2_STOP_ENABLE: c_uint = 0xf12b0004;
pub const F0900_P2_STOP_SDAT2SDA: c_uint = 0xf12b0002;
// IOPVALUE6
pub const R0900_IOPVALUE6: c_uint = 0xf138;
pub const F0900_VSCL: c_uint = 0xf1380004;
pub const F0900_VSDA: c_uint = 0xf1380002;
pub const F0900_VDATA3_0: c_uint = 0xf1380001;
// IOPVALUE5
pub const R0900_IOPVALUE5: c_uint = 0xf139;
pub const F0900_VDATA3_1: c_uint = 0xf1390080;
pub const F0900_VDATA3_2: c_uint = 0xf1390040;
pub const F0900_VDATA3_3: c_uint = 0xf1390020;
pub const F0900_VDATA3_4: c_uint = 0xf1390010;
pub const F0900_VDATA3_5: c_uint = 0xf1390008;
pub const F0900_VDATA3_6: c_uint = 0xf1390004;
pub const F0900_VDATA3_7: c_uint = 0xf1390002;
pub const F0900_VCLKOUT3: c_uint = 0xf1390001;
// IOPVALUE4
pub const R0900_IOPVALUE4: c_uint = 0xf13a;
pub const F0900_VSTROUT3: c_uint = 0xf13a0080;
pub const F0900_VDPN3: c_uint = 0xf13a0040;
pub const F0900_VERROR3: c_uint = 0xf13a0020;
pub const F0900_VDATA2_7: c_uint = 0xf13a0010;
pub const F0900_VCLKOUT2: c_uint = 0xf13a0008;
pub const F0900_VSTROUT2: c_uint = 0xf13a0004;
pub const F0900_VDPN2: c_uint = 0xf13a0002;
pub const F0900_VERROR2: c_uint = 0xf13a0001;
// IOPVALUE3
pub const R0900_IOPVALUE3: c_uint = 0xf13b;
pub const F0900_VDATA1_7: c_uint = 0xf13b0080;
pub const F0900_VCLKOUT1: c_uint = 0xf13b0040;
pub const F0900_VSTROUT1: c_uint = 0xf13b0020;
pub const F0900_VDPN1: c_uint = 0xf13b0010;
pub const F0900_VERROR1: c_uint = 0xf13b0008;
pub const F0900_VCLKOUT27: c_uint = 0xf13b0004;
pub const F0900_VDISEQCOUT2: c_uint = 0xf13b0002;
pub const F0900_VSCLT2: c_uint = 0xf13b0001;
// IOPVALUE2
pub const R0900_IOPVALUE2: c_uint = 0xf13c;
pub const F0900_VSDAT2: c_uint = 0xf13c0080;
pub const F0900_VAGCRF2: c_uint = 0xf13c0040;
pub const F0900_VDISEQCOUT1: c_uint = 0xf13c0020;
pub const F0900_VSCLT1: c_uint = 0xf13c0010;
pub const F0900_VSDAT1: c_uint = 0xf13c0008;
pub const F0900_VAGCRF1: c_uint = 0xf13c0004;
pub const F0900_VDIRCLK: c_uint = 0xf13c0002;
pub const F0900_VSTDBY: c_uint = 0xf13c0001;
// IOPVALUE1
pub const R0900_IOPVALUE1: c_uint = 0xf13d;
pub const F0900_VCS1: c_uint = 0xf13d0080;
pub const F0900_VCS0: c_uint = 0xf13d0040;
pub const F0900_VGPIO13: c_uint = 0xf13d0020;
pub const F0900_VGPIO12: c_uint = 0xf13d0010;
pub const F0900_VGPIO11: c_uint = 0xf13d0008;
pub const F0900_VGPIO10: c_uint = 0xf13d0004;
pub const F0900_VGPIO9: c_uint = 0xf13d0002;
pub const F0900_VGPIO8: c_uint = 0xf13d0001;
// IOPVALUE0
pub const R0900_IOPVALUE0: c_uint = 0xf13e;
pub const F0900_VGPIO7: c_uint = 0xf13e0080;
pub const F0900_VGPIO6: c_uint = 0xf13e0040;
pub const F0900_VGPIO5: c_uint = 0xf13e0020;
pub const F0900_VGPIO4: c_uint = 0xf13e0010;
pub const F0900_VGPIO3: c_uint = 0xf13e0008;
pub const F0900_VGPIO2: c_uint = 0xf13e0004;
pub const F0900_VGPIO1: c_uint = 0xf13e0002;
pub const F0900_VCLKI2: c_uint = 0xf13e0001;
// CLKI2CFG
pub const R0900_CLKI2CFG: c_uint = 0xf140;
pub const F0900_CLKI2_OPD: c_uint = 0xf1400080;
pub const F0900_CLKI2_CONFIG: c_uint = 0xf140007e;
pub const F0900_CLKI2_XOR: c_uint = 0xf1400001;
// GPIO1CFG
pub const R0900_GPIO1CFG: c_uint = 0xf141;
pub const F0900_GPIO1_OPD: c_uint = 0xf1410080;
pub const F0900_GPIO1_CONFIG: c_uint = 0xf141007e;
pub const F0900_GPIO1_XOR: c_uint = 0xf1410001;
// GPIO2CFG
pub const R0900_GPIO2CFG: c_uint = 0xf142;
pub const F0900_GPIO2_OPD: c_uint = 0xf1420080;
pub const F0900_GPIO2_CONFIG: c_uint = 0xf142007e;
pub const F0900_GPIO2_XOR: c_uint = 0xf1420001;
// GPIO3CFG
pub const R0900_GPIO3CFG: c_uint = 0xf143;
pub const F0900_GPIO3_OPD: c_uint = 0xf1430080;
pub const F0900_GPIO3_CONFIG: c_uint = 0xf143007e;
pub const F0900_GPIO3_XOR: c_uint = 0xf1430001;
// GPIO4CFG
pub const R0900_GPIO4CFG: c_uint = 0xf144;
pub const F0900_GPIO4_OPD: c_uint = 0xf1440080;
pub const F0900_GPIO4_CONFIG: c_uint = 0xf144007e;
pub const F0900_GPIO4_XOR: c_uint = 0xf1440001;
// GPIO5CFG
pub const R0900_GPIO5CFG: c_uint = 0xf145;
pub const F0900_GPIO5_OPD: c_uint = 0xf1450080;
pub const F0900_GPIO5_CONFIG: c_uint = 0xf145007e;
pub const F0900_GPIO5_XOR: c_uint = 0xf1450001;
// GPIO6CFG
pub const R0900_GPIO6CFG: c_uint = 0xf146;
pub const F0900_GPIO6_OPD: c_uint = 0xf1460080;
pub const F0900_GPIO6_CONFIG: c_uint = 0xf146007e;
pub const F0900_GPIO6_XOR: c_uint = 0xf1460001;
// GPIO7CFG
pub const R0900_GPIO7CFG: c_uint = 0xf147;
pub const F0900_GPIO7_OPD: c_uint = 0xf1470080;
pub const F0900_GPIO7_CONFIG: c_uint = 0xf147007e;
pub const F0900_GPIO7_XOR: c_uint = 0xf1470001;
// GPIO8CFG
pub const R0900_GPIO8CFG: c_uint = 0xf148;
pub const F0900_GPIO8_OPD: c_uint = 0xf1480080;
pub const F0900_GPIO8_CONFIG: c_uint = 0xf148007e;
pub const F0900_GPIO8_XOR: c_uint = 0xf1480001;
// GPIO9CFG
pub const R0900_GPIO9CFG: c_uint = 0xf149;
pub const F0900_GPIO9_OPD: c_uint = 0xf1490080;
pub const F0900_GPIO9_CONFIG: c_uint = 0xf149007e;
pub const F0900_GPIO9_XOR: c_uint = 0xf1490001;
// GPIO10CFG
pub const R0900_GPIO10CFG: c_uint = 0xf14a;
pub const F0900_GPIO10_OPD: c_uint = 0xf14a0080;
pub const F0900_GPIO10_CONFIG: c_uint = 0xf14a007e;
pub const F0900_GPIO10_XOR: c_uint = 0xf14a0001;
// GPIO11CFG
pub const R0900_GPIO11CFG: c_uint = 0xf14b;
pub const F0900_GPIO11_OPD: c_uint = 0xf14b0080;
pub const F0900_GPIO11_CONFIG: c_uint = 0xf14b007e;
pub const F0900_GPIO11_XOR: c_uint = 0xf14b0001;
// GPIO12CFG
pub const R0900_GPIO12CFG: c_uint = 0xf14c;
pub const F0900_GPIO12_OPD: c_uint = 0xf14c0080;
pub const F0900_GPIO12_CONFIG: c_uint = 0xf14c007e;
pub const F0900_GPIO12_XOR: c_uint = 0xf14c0001;
// GPIO13CFG
pub const R0900_GPIO13CFG: c_uint = 0xf14d;
pub const F0900_GPIO13_OPD: c_uint = 0xf14d0080;
pub const F0900_GPIO13_CONFIG: c_uint = 0xf14d007e;
pub const F0900_GPIO13_XOR: c_uint = 0xf14d0001;
// CS0CFG
pub const R0900_CS0CFG: c_uint = 0xf14e;
pub const F0900_CS0_OPD: c_uint = 0xf14e0080;
pub const F0900_CS0_CONFIG: c_uint = 0xf14e007e;
pub const F0900_CS0_XOR: c_uint = 0xf14e0001;
// CS1CFG
pub const R0900_CS1CFG: c_uint = 0xf14f;
pub const F0900_CS1_OPD: c_uint = 0xf14f0080;
pub const F0900_CS1_CONFIG: c_uint = 0xf14f007e;
pub const F0900_CS1_XOR: c_uint = 0xf14f0001;
// STDBYCFG
pub const R0900_STDBYCFG: c_uint = 0xf150;
pub const F0900_STDBY_OPD: c_uint = 0xf1500080;
pub const F0900_STDBY_CONFIG: c_uint = 0xf150007e;
pub const F0900_STBDY_XOR: c_uint = 0xf1500001;
// DIRCLKCFG
pub const R0900_DIRCLKCFG: c_uint = 0xf151;
pub const F0900_DIRCLK_OPD: c_uint = 0xf1510080;
pub const F0900_DIRCLK_CONFIG: c_uint = 0xf151007e;
pub const F0900_DIRCLK_XOR: c_uint = 0xf1510001;
// AGCRF1CFG
pub const R0900_AGCRF1CFG: c_uint = 0xf152;
pub const F0900_AGCRF1_OPD: c_uint = 0xf1520080;
pub const F0900_AGCRF1_CONFIG: c_uint = 0xf152007e;
pub const F0900_AGCRF1_XOR: c_uint = 0xf1520001;
// SDAT1CFG
pub const R0900_SDAT1CFG: c_uint = 0xf153;
pub const F0900_SDAT1_OPD: c_uint = 0xf1530080;
pub const F0900_SDAT1_CONFIG: c_uint = 0xf153007e;
pub const F0900_SDAT1_XOR: c_uint = 0xf1530001;
// SCLT1CFG
pub const R0900_SCLT1CFG: c_uint = 0xf154;
pub const F0900_SCLT1_OPD: c_uint = 0xf1540080;
pub const F0900_SCLT1_CONFIG: c_uint = 0xf154007e;
pub const F0900_SCLT1_XOR: c_uint = 0xf1540001;
// DISEQCO1CFG
pub const R0900_DISEQCO1CFG: c_uint = 0xf155;
pub const F0900_DISEQCO1_OPD: c_uint = 0xf1550080;
pub const F0900_DISEQCO1_CONFIG: c_uint = 0xf155007e;
pub const F0900_DISEQC1_XOR: c_uint = 0xf1550001;
// AGCRF2CFG
pub const R0900_AGCRF2CFG: c_uint = 0xf156;
pub const F0900_AGCRF2_OPD: c_uint = 0xf1560080;
pub const F0900_AGCRF2_CONFIG: c_uint = 0xf156007e;
pub const F0900_AGCRF2_XOR: c_uint = 0xf1560001;
// SDAT2CFG
pub const R0900_SDAT2CFG: c_uint = 0xf157;
pub const F0900_SDAT2_OPD: c_uint = 0xf1570080;
pub const F0900_SDAT2_CONFIG: c_uint = 0xf157007e;
pub const F0900_SDAT2_XOR: c_uint = 0xf1570001;
// SCLT2CFG
pub const R0900_SCLT2CFG: c_uint = 0xf158;
pub const F0900_SCLT2_OPD: c_uint = 0xf1580080;
pub const F0900_SCLT2_CONFIG: c_uint = 0xf158007e;
pub const F0900_SCLT2_XOR: c_uint = 0xf1580001;
// DISEQCO2CFG
pub const R0900_DISEQCO2CFG: c_uint = 0xf159;
pub const F0900_DISEQCO2_OPD: c_uint = 0xf1590080;
pub const F0900_DISEQCO2_CONFIG: c_uint = 0xf159007e;
pub const F0900_DISEQC2_XOR: c_uint = 0xf1590001;
// CLKOUT27CFG
pub const R0900_CLKOUT27CFG: c_uint = 0xf15a;
pub const F0900_CLKOUT27_OPD: c_uint = 0xf15a0080;
pub const F0900_CLKOUT27_CONFIG: c_uint = 0xf15a007e;
pub const F0900_CLKOUT27_XOR: c_uint = 0xf15a0001;
// ERROR1CFG
pub const R0900_ERROR1CFG: c_uint = 0xf15b;
pub const F0900_ERROR1_OPD: c_uint = 0xf15b0080;
pub const F0900_ERROR1_CONFIG: c_uint = 0xf15b007e;
pub const F0900_ERROR1_XOR: c_uint = 0xf15b0001;
// DPN1CFG
pub const R0900_DPN1CFG: c_uint = 0xf15c;
pub const F0900_DPN1_OPD: c_uint = 0xf15c0080;
pub const F0900_DPN1_CONFIG: c_uint = 0xf15c007e;
pub const F0900_DPN1_XOR: c_uint = 0xf15c0001;
// STROUT1CFG
pub const R0900_STROUT1CFG: c_uint = 0xf15d;
pub const F0900_STROUT1_OPD: c_uint = 0xf15d0080;
pub const F0900_STROUT1_CONFIG: c_uint = 0xf15d007e;
pub const F0900_STROUT1_XOR: c_uint = 0xf15d0001;
// CLKOUT1CFG
pub const R0900_CLKOUT1CFG: c_uint = 0xf15e;
pub const F0900_CLKOUT1_OPD: c_uint = 0xf15e0080;
pub const F0900_CLKOUT1_CONFIG: c_uint = 0xf15e007e;
pub const F0900_CLKOUT1_XOR: c_uint = 0xf15e0001;
// DATA71CFG
pub const R0900_DATA71CFG: c_uint = 0xf15f;
pub const F0900_DATA71_OPD: c_uint = 0xf15f0080;
pub const F0900_DATA71_CONFIG: c_uint = 0xf15f007e;
pub const F0900_DATA71_XOR: c_uint = 0xf15f0001;
// ERROR2CFG
pub const R0900_ERROR2CFG: c_uint = 0xf160;
pub const F0900_ERROR2_OPD: c_uint = 0xf1600080;
pub const F0900_ERROR2_CONFIG: c_uint = 0xf160007e;
pub const F0900_ERROR2_XOR: c_uint = 0xf1600001;
// DPN2CFG
pub const R0900_DPN2CFG: c_uint = 0xf161;
pub const F0900_DPN2_OPD: c_uint = 0xf1610080;
pub const F0900_DPN2_CONFIG: c_uint = 0xf161007e;
pub const F0900_DPN2_XOR: c_uint = 0xf1610001;
// STROUT2CFG
pub const R0900_STROUT2CFG: c_uint = 0xf162;
pub const F0900_STROUT2_OPD: c_uint = 0xf1620080;
pub const F0900_STROUT2_CONFIG: c_uint = 0xf162007e;
pub const F0900_STROUT2_XOR: c_uint = 0xf1620001;
// CLKOUT2CFG
pub const R0900_CLKOUT2CFG: c_uint = 0xf163;
pub const F0900_CLKOUT2_OPD: c_uint = 0xf1630080;
pub const F0900_CLKOUT2_CONFIG: c_uint = 0xf163007e;
pub const F0900_CLKOUT2_XOR: c_uint = 0xf1630001;
// DATA72CFG
pub const R0900_DATA72CFG: c_uint = 0xf164;
pub const F0900_DATA72_OPD: c_uint = 0xf1640080;
pub const F0900_DATA72_CONFIG: c_uint = 0xf164007e;
pub const F0900_DATA72_XOR: c_uint = 0xf1640001;
// ERROR3CFG
pub const R0900_ERROR3CFG: c_uint = 0xf165;
pub const F0900_ERROR3_OPD: c_uint = 0xf1650080;
pub const F0900_ERROR3_CONFIG: c_uint = 0xf165007e;
pub const F0900_ERROR3_XOR: c_uint = 0xf1650001;
// DPN3CFG
pub const R0900_DPN3CFG: c_uint = 0xf166;
pub const F0900_DPN3_OPD: c_uint = 0xf1660080;
pub const F0900_DPN3_CONFIG: c_uint = 0xf166007e;
pub const F0900_DPN3_XOR: c_uint = 0xf1660001;
// STROUT3CFG
pub const R0900_STROUT3CFG: c_uint = 0xf167;
pub const F0900_STROUT3_OPD: c_uint = 0xf1670080;
pub const F0900_STROUT3_CONFIG: c_uint = 0xf167007e;
pub const F0900_STROUT3_XOR: c_uint = 0xf1670001;
// CLKOUT3CFG
pub const R0900_CLKOUT3CFG: c_uint = 0xf168;
pub const F0900_CLKOUT3_OPD: c_uint = 0xf1680080;
pub const F0900_CLKOUT3_CONFIG: c_uint = 0xf168007e;
pub const F0900_CLKOUT3_XOR: c_uint = 0xf1680001;
// DATA73CFG
pub const R0900_DATA73CFG: c_uint = 0xf169;
pub const F0900_DATA73_OPD: c_uint = 0xf1690080;
pub const F0900_DATA73_CONFIG: c_uint = 0xf169007e;
pub const F0900_DATA73_XOR: c_uint = 0xf1690001;
// STRSTATUS1
pub const R0900_STRSTATUS1: c_uint = 0xf16a;
pub const F0900_STRSTATUS_SEL2: c_uint = 0xf16a00f0;
pub const F0900_STRSTATUS_SEL1: c_uint = 0xf16a000f;
// STRSTATUS2
pub const R0900_STRSTATUS2: c_uint = 0xf16b;
pub const F0900_STRSTATUS_SEL4: c_uint = 0xf16b00f0;
pub const F0900_STRSTATUS_SEL3: c_uint = 0xf16b000f;
// STRSTATUS3
pub const R0900_STRSTATUS3: c_uint = 0xf16c;
pub const F0900_STRSTATUS_SEL6: c_uint = 0xf16c00f0;
pub const F0900_STRSTATUS_SEL5: c_uint = 0xf16c000f;
// FSKTFC2
pub const R0900_FSKTFC2: c_uint = 0xf170;
pub const F0900_FSKT_KMOD: c_uint = 0xf17000fc;
pub const F0900_FSKT_CAR2: c_uint = 0xf1700003;
// FSKTFC1
pub const R0900_FSKTFC1: c_uint = 0xf171;
pub const F0900_FSKT_CAR1: c_uint = 0xf17100ff;
// FSKTFC0
pub const R0900_FSKTFC0: c_uint = 0xf172;
pub const F0900_FSKT_CAR0: c_uint = 0xf17200ff;
// FSKTDELTAF1
pub const R0900_FSKTDELTAF1: c_uint = 0xf173;
pub const F0900_FSKT_DELTAF1: c_uint = 0xf173000f;
// FSKTDELTAF0
pub const R0900_FSKTDELTAF0: c_uint = 0xf174;
pub const F0900_FSKT_DELTAF0: c_uint = 0xf17400ff;
// FSKTCTRL
pub const R0900_FSKTCTRL: c_uint = 0xf175;
pub const F0900_FSKT_EN_SGN: c_uint = 0xf1750040;
pub const F0900_FSKT_MOD_SGN: c_uint = 0xf1750020;
pub const F0900_FSKT_MOD_EN: c_uint = 0xf175001c;
pub const F0900_FSKT_DACMODE: c_uint = 0xf1750003;
// FSKRFC2
pub const R0900_FSKRFC2: c_uint = 0xf176;
pub const F0900_FSKR_DETSGN: c_uint = 0xf1760040;
pub const F0900_FSKR_OUTSGN: c_uint = 0xf1760020;
pub const F0900_FSKR_KAGC: c_uint = 0xf176001c;
pub const F0900_FSKR_CAR2: c_uint = 0xf1760003;
// FSKRFC1
pub const R0900_FSKRFC1: c_uint = 0xf177;
pub const F0900_FSKR_CAR1: c_uint = 0xf17700ff;
// FSKRFC0
pub const R0900_FSKRFC0: c_uint = 0xf178;
pub const F0900_FSKR_CAR0: c_uint = 0xf17800ff;
// FSKRK1
pub const R0900_FSKRK1: c_uint = 0xf179;
pub const F0900_FSKR_K1_EXP: c_uint = 0xf17900e0;
pub const F0900_FSKR_K1_MANT: c_uint = 0xf179001f;
// FSKRK2
pub const R0900_FSKRK2: c_uint = 0xf17a;
pub const F0900_FSKR_K2_EXP: c_uint = 0xf17a00e0;
pub const F0900_FSKR_K2_MANT: c_uint = 0xf17a001f;
// FSKRAGCR
pub const R0900_FSKRAGCR: c_uint = 0xf17b;
pub const F0900_FSKR_OUTCTL: c_uint = 0xf17b00c0;
pub const F0900_FSKR_AGC_REF: c_uint = 0xf17b003f;
// FSKRAGC
pub const R0900_FSKRAGC: c_uint = 0xf17c;
pub const F0900_FSKR_AGC_ACCU: c_uint = 0xf17c00ff;
// FSKRALPHA
pub const R0900_FSKRALPHA: c_uint = 0xf17d;
pub const F0900_FSKR_ALPHA_EXP: c_uint = 0xf17d001c;
pub const F0900_FSKR_ALPHA_M: c_uint = 0xf17d0003;
// FSKRPLTH1
pub const R0900_FSKRPLTH1: c_uint = 0xf17e;
pub const F0900_FSKR_BETA: c_uint = 0xf17e00f0;
pub const F0900_FSKR_PLL_TRESH1: c_uint = 0xf17e000f;
// FSKRPLTH0
pub const R0900_FSKRPLTH0: c_uint = 0xf17f;
pub const F0900_FSKR_PLL_TRESH0: c_uint = 0xf17f00ff;
// FSKRDF1
pub const R0900_FSKRDF1: c_uint = 0xf180;
pub const F0900_FSKR_OUT: c_uint = 0xf1800080;
pub const F0900_FSKR_DELTAF1: c_uint = 0xf180001f;
// FSKRDF0
pub const R0900_FSKRDF0: c_uint = 0xf181;
pub const F0900_FSKR_DELTAF0: c_uint = 0xf18100ff;
// FSKRSTEPP
pub const R0900_FSKRSTEPP: c_uint = 0xf182;
pub const F0900_FSKR_STEP_PLUS: c_uint = 0xf18200ff;
// FSKRSTEPM
pub const R0900_FSKRSTEPM: c_uint = 0xf183;
pub const F0900_FSKR_STEP_MINUS: c_uint = 0xf18300ff;
// FSKRDET1
pub const R0900_FSKRDET1: c_uint = 0xf184;
pub const F0900_FSKR_DETECT: c_uint = 0xf1840080;
pub const F0900_FSKR_CARDET_ACCU1: c_uint = 0xf184000f;
// FSKRDET0
pub const R0900_FSKRDET0: c_uint = 0xf185;
pub const F0900_FSKR_CARDET_ACCU0: c_uint = 0xf18500ff;
// FSKRDTH1
pub const R0900_FSKRDTH1: c_uint = 0xf186;
pub const F0900_FSKR_CARLOSS_THRESH1: c_uint = 0xf18600f0;
pub const F0900_FSKR_CARDET_THRESH1: c_uint = 0xf186000f;
// FSKRDTH0
pub const R0900_FSKRDTH0: c_uint = 0xf187;
pub const F0900_FSKR_CARDET_THRESH0: c_uint = 0xf18700ff;
// FSKRLOSS
pub const R0900_FSKRLOSS: c_uint = 0xf188;
pub const F0900_FSKR_CARLOSS_THRESH0: c_uint = 0xf18800ff;
// P2_DISTXCTL
pub const R0900_P2_DISTXCTL: c_uint = 0xf190;
pub const F0900_P2_TIM_OFF: c_uint = 0xf1900080;
pub const F0900_P2_DISEQC_RESET: c_uint = 0xf1900040;
pub const F0900_P2_TIM_CMD: c_uint = 0xf1900030;
pub const F0900_P2_DIS_PRECHARGE: c_uint = 0xf1900008;
pub const F0900_P2_DISTX_MODE: c_uint = 0xf1900007;
// P2_DISRXCTL
pub const R0900_P2_DISRXCTL: c_uint = 0xf191;
pub const F0900_P2_RECEIVER_ON: c_uint = 0xf1910080;
pub const F0900_P2_IGNO_SHORT22K: c_uint = 0xf1910040;
pub const F0900_P2_ONECHIP_TRX: c_uint = 0xf1910020;
pub const F0900_P2_EXT_ENVELOP: c_uint = 0xf1910010;
pub const F0900_P2_PIN_SELECT0: c_uint = 0xf191000c;
pub const F0900_P2_IRQ_RXEND: c_uint = 0xf1910002;
pub const F0900_P2_IRQ_4NBYTES: c_uint = 0xf1910001;
// P2_DISRX_ST0
pub const R0900_P2_DISRX_ST0: c_uint = 0xf194;
pub const F0900_P2_RX_END: c_uint = 0xf1940080;
pub const F0900_P2_RX_ACTIVE: c_uint = 0xf1940040;
pub const F0900_P2_SHORT_22KHZ: c_uint = 0xf1940020;
pub const F0900_P2_CONT_TONE: c_uint = 0xf1940010;
pub const F0900_P2_FIFO_4BREADY: c_uint = 0xf1940008;
pub const F0900_P2_FIFO_EMPTY: c_uint = 0xf1940004;
pub const F0900_P2_ABORT_DISRX: c_uint = 0xf1940001;
// P2_DISRX_ST1
pub const R0900_P2_DISRX_ST1: c_uint = 0xf195;
pub const F0900_P2_RX_FAIL: c_uint = 0xf1950080;
pub const F0900_P2_FIFO_PARITYFAIL: c_uint = 0xf1950040;
pub const F0900_P2_RX_NONBYTE: c_uint = 0xf1950020;
pub const F0900_P2_FIFO_OVERFLOW: c_uint = 0xf1950010;
pub const F0900_P2_FIFO_BYTENBR: c_uint = 0xf195000f;
// P2_DISRXDATA
pub const R0900_P2_DISRXDATA: c_uint = 0xf196;
pub const F0900_P2_DISRX_DATA: c_uint = 0xf19600ff;
// P2_DISTXDATA
pub const R0900_P2_DISTXDATA: c_uint = 0xf197;
pub const F0900_P2_DISEQC_FIFO: c_uint = 0xf19700ff;
// P2_DISTXSTATUS
pub const R0900_P2_DISTXSTATUS: c_uint = 0xf198;
pub const F0900_P2_TX_FAIL: c_uint = 0xf1980080;
pub const F0900_P2_FIFO_FULL: c_uint = 0xf1980040;
pub const F0900_P2_TX_IDLE: c_uint = 0xf1980020;
pub const F0900_P2_GAP_BURST: c_uint = 0xf1980010;
pub const F0900_P2_TXFIFO_BYTES: c_uint = 0xf198000f;
// P2_F22TX
pub const R0900_P2_F22TX: c_uint = 0xf199;
pub const F0900_P2_F22_REG: c_uint = 0xf19900ff;
// P2_F22RX
pub const R0900_P2_F22RX: c_uint = 0xf19a;
pub const F0900_P2_F22RX_REG: c_uint = 0xf19a00ff;
// P2_ACRPRESC
pub const R0900_P2_ACRPRESC: c_uint = 0xf19c;
pub const F0900_P2_ACR_PRESC: c_uint = 0xf19c0007;
// P2_ACRDIV
pub const R0900_P2_ACRDIV: c_uint = 0xf19d;
pub const F0900_P2_ACR_DIV: c_uint = 0xf19d00ff;
// P1_DISTXCTL
pub const R0900_P1_DISTXCTL: c_uint = 0xf1a0;

pub const F0900_P1_TIM_OFF: c_uint = 0xf1a00080;
pub const F0900_P1_DISEQC_RESET: c_uint = 0xf1a00040;

pub const F0900_P1_TIM_CMD: c_uint = 0xf1a00030;
pub const F0900_P1_DIS_PRECHARGE: c_uint = 0xf1a00008;

pub const F0900_P1_DISTX_MODE: c_uint = 0xf1a00007;

// P1_DISRXCTL
pub const R0900_P1_DISRXCTL: c_uint = 0xf1a1;

pub const F0900_P1_RECEIVER_ON: c_uint = 0xf1a10080;
pub const F0900_P1_IGNO_SHORT22K: c_uint = 0xf1a10040;
pub const F0900_P1_ONECHIP_TRX: c_uint = 0xf1a10020;
pub const F0900_P1_EXT_ENVELOP: c_uint = 0xf1a10010;
pub const F0900_P1_PIN_SELECT0: c_uint = 0xf1a1000c;
pub const F0900_P1_IRQ_RXEND: c_uint = 0xf1a10002;
pub const F0900_P1_IRQ_4NBYTES: c_uint = 0xf1a10001;
// P1_DISRX_ST0
pub const R0900_P1_DISRX_ST0: c_uint = 0xf1a4;

pub const F0900_P1_RX_END: c_uint = 0xf1a40080;

pub const F0900_P1_RX_ACTIVE: c_uint = 0xf1a40040;
pub const F0900_P1_SHORT_22KHZ: c_uint = 0xf1a40020;
pub const F0900_P1_CONT_TONE: c_uint = 0xf1a40010;
pub const F0900_P1_FIFO_4BREADY: c_uint = 0xf1a40008;
pub const F0900_P1_FIFO_EMPTY: c_uint = 0xf1a40004;
pub const F0900_P1_ABORT_DISRX: c_uint = 0xf1a40001;
// P1_DISRX_ST1
pub const R0900_P1_DISRX_ST1: c_uint = 0xf1a5;

pub const F0900_P1_RX_FAIL: c_uint = 0xf1a50080;
pub const F0900_P1_FIFO_PARITYFAIL: c_uint = 0xf1a50040;
pub const F0900_P1_RX_NONBYTE: c_uint = 0xf1a50020;
pub const F0900_P1_FIFO_OVERFLOW: c_uint = 0xf1a50010;
pub const F0900_P1_FIFO_BYTENBR: c_uint = 0xf1a5000f;

// P1_DISRXDATA
pub const R0900_P1_DISRXDATA: c_uint = 0xf1a6;

pub const F0900_P1_DISRX_DATA: c_uint = 0xf1a600ff;
// P1_DISTXDATA
pub const R0900_P1_DISTXDATA: c_uint = 0xf1a7;

pub const F0900_P1_DISEQC_FIFO: c_uint = 0xf1a700ff;
// P1_DISTXSTATUS
pub const R0900_P1_DISTXSTATUS: c_uint = 0xf1a8;
pub const F0900_P1_TX_FAIL: c_uint = 0xf1a80080;
pub const F0900_P1_FIFO_FULL: c_uint = 0xf1a80040;

pub const F0900_P1_TX_IDLE: c_uint = 0xf1a80020;

pub const F0900_P1_GAP_BURST: c_uint = 0xf1a80010;
pub const F0900_P1_TXFIFO_BYTES: c_uint = 0xf1a8000f;
// P1_F22TX
pub const R0900_P1_F22TX: c_uint = 0xf1a9;

pub const F0900_P1_F22_REG: c_uint = 0xf1a900ff;
// P1_F22RX
pub const R0900_P1_F22RX: c_uint = 0xf1aa;

pub const F0900_P1_F22RX_REG: c_uint = 0xf1aa00ff;
// P1_ACRPRESC
pub const R0900_P1_ACRPRESC: c_uint = 0xf1ac;

pub const F0900_P1_ACR_PRESC: c_uint = 0xf1ac0007;
// P1_ACRDIV
pub const R0900_P1_ACRDIV: c_uint = 0xf1ad;

pub const F0900_P1_ACR_DIV: c_uint = 0xf1ad00ff;
// NCOARSE
pub const R0900_NCOARSE: c_uint = 0xf1b3;
pub const F0900_M_DIV: c_uint = 0xf1b300ff;
// SYNTCTRL
pub const R0900_SYNTCTRL: c_uint = 0xf1b6;
pub const F0900_STANDBY: c_uint = 0xf1b60080;
pub const F0900_BYPASSPLLCORE: c_uint = 0xf1b60040;
pub const F0900_SELX1RATIO: c_uint = 0xf1b60020;
pub const F0900_STOP_PLL: c_uint = 0xf1b60008;
pub const F0900_BYPASSPLLFSK: c_uint = 0xf1b60004;
pub const F0900_SELOSCI: c_uint = 0xf1b60002;
pub const F0900_BYPASSPLLADC: c_uint = 0xf1b60001;
// FILTCTRL
pub const R0900_FILTCTRL: c_uint = 0xf1b7;
pub const F0900_INV_CLK135: c_uint = 0xf1b70080;
pub const F0900_SEL_FSKCKDIV: c_uint = 0xf1b70004;
pub const F0900_INV_CLKFSK: c_uint = 0xf1b70002;
pub const F0900_BYPASS_APPLI: c_uint = 0xf1b70001;
// PLLSTAT
pub const R0900_PLLSTAT: c_uint = 0xf1b8;
pub const F0900_PLLLOCK: c_uint = 0xf1b80001;
// STOPCLK1
pub const R0900_STOPCLK1: c_uint = 0xf1c2;
pub const F0900_STOP_CLKPKDT2: c_uint = 0xf1c20040;
pub const F0900_STOP_CLKPKDT1: c_uint = 0xf1c20020;
pub const F0900_STOP_CLKFEC: c_uint = 0xf1c20010;
pub const F0900_STOP_CLKADCI2: c_uint = 0xf1c20008;
pub const F0900_INV_CLKADCI2: c_uint = 0xf1c20004;
pub const F0900_STOP_CLKADCI1: c_uint = 0xf1c20002;
pub const F0900_INV_CLKADCI1: c_uint = 0xf1c20001;
// STOPCLK2
pub const R0900_STOPCLK2: c_uint = 0xf1c3;
pub const F0900_STOP_CLKSAMP2: c_uint = 0xf1c30010;
pub const F0900_STOP_CLKSAMP1: c_uint = 0xf1c30008;
pub const F0900_STOP_CLKVIT2: c_uint = 0xf1c30004;
pub const F0900_STOP_CLKVIT1: c_uint = 0xf1c30002;

pub const F0900_STOP_CLKTS: c_uint = 0xf1c30001;
// TSTTNR0
pub const R0900_TSTTNR0: c_uint = 0xf1df;
pub const F0900_SEL_FSK: c_uint = 0xf1df0080;
pub const F0900_FSK_PON: c_uint = 0xf1df0004;
// TSTTNR1
pub const R0900_TSTTNR1: c_uint = 0xf1e0;
pub const F0900_ADC1_PON: c_uint = 0xf1e00002;
pub const F0900_ADC1_INMODE: c_uint = 0xf1e00001;
// TSTTNR2
pub const R0900_TSTTNR2: c_uint = 0xf1e1;
pub const F0900_DISEQC1_PON: c_uint = 0xf1e10020;
// TSTTNR3
pub const R0900_TSTTNR3: c_uint = 0xf1e2;
pub const F0900_ADC2_PON: c_uint = 0xf1e20002;
pub const F0900_ADC2_INMODE: c_uint = 0xf1e20001;
// TSTTNR4
pub const R0900_TSTTNR4: c_uint = 0xf1e3;
pub const F0900_DISEQC2_PON: c_uint = 0xf1e30020;
// P2_IQCONST
pub const R0900_P2_IQCONST: c_uint = 0xf200;
pub const F0900_P2_CONSTEL_SELECT: c_uint = 0xf2000060;
pub const F0900_P2_IQSYMB_SEL: c_uint = 0xf200001f;
// P2_NOSCFG
pub const R0900_P2_NOSCFG: c_uint = 0xf201;
pub const F0900_P2_DUMMYPL_NOSDATA: c_uint = 0xf2010020;
pub const F0900_P2_NOSPLH_BETA: c_uint = 0xf2010018;
pub const F0900_P2_NOSDATA_BETA: c_uint = 0xf2010007;
// P2_ISYMB
pub const R0900_P2_ISYMB: c_uint = 0xf202;
pub const F0900_P2_I_SYMBOL: c_uint = 0xf20201ff;
// P2_QSYMB
pub const R0900_P2_QSYMB: c_uint = 0xf203;
pub const F0900_P2_Q_SYMBOL: c_uint = 0xf20301ff;
// P2_AGC1CFG
pub const R0900_P2_AGC1CFG: c_uint = 0xf204;
pub const F0900_P2_DC_FROZEN: c_uint = 0xf2040080;
pub const F0900_P2_DC_CORRECT: c_uint = 0xf2040040;
pub const F0900_P2_AMM_FROZEN: c_uint = 0xf2040020;
pub const F0900_P2_AMM_CORRECT: c_uint = 0xf2040010;
pub const F0900_P2_QUAD_FROZEN: c_uint = 0xf2040008;
pub const F0900_P2_QUAD_CORRECT: c_uint = 0xf2040004;
// P2_AGC1CN
pub const R0900_P2_AGC1CN: c_uint = 0xf206;
pub const F0900_P2_AGC1_LOCKED: c_uint = 0xf2060080;
pub const F0900_P2_AGC1_MINPOWER: c_uint = 0xf2060010;
pub const F0900_P2_AGCOUT_FAST: c_uint = 0xf2060008;
pub const F0900_P2_AGCIQ_BETA: c_uint = 0xf2060007;
// P2_AGC1REF
pub const R0900_P2_AGC1REF: c_uint = 0xf207;
pub const F0900_P2_AGCIQ_REF: c_uint = 0xf20700ff;
// P2_IDCCOMP
pub const R0900_P2_IDCCOMP: c_uint = 0xf208;
pub const F0900_P2_IAVERAGE_ADJ: c_uint = 0xf20801ff;
// P2_QDCCOMP
pub const R0900_P2_QDCCOMP: c_uint = 0xf209;
pub const F0900_P2_QAVERAGE_ADJ: c_uint = 0xf20901ff;
// P2_POWERI
pub const R0900_P2_POWERI: c_uint = 0xf20a;
pub const F0900_P2_POWER_I: c_uint = 0xf20a00ff;
// P2_POWERQ
pub const R0900_P2_POWERQ: c_uint = 0xf20b;
pub const F0900_P2_POWER_Q: c_uint = 0xf20b00ff;
// P2_AGC1AMM
pub const R0900_P2_AGC1AMM: c_uint = 0xf20c;
pub const F0900_P2_AMM_VALUE: c_uint = 0xf20c00ff;
// P2_AGC1QUAD
pub const R0900_P2_AGC1QUAD: c_uint = 0xf20d;
pub const F0900_P2_QUAD_VALUE: c_uint = 0xf20d01ff;
// P2_AGCIQIN1
pub const R0900_P2_AGCIQIN1: c_uint = 0xf20e;
pub const F0900_P2_AGCIQ_VALUE1: c_uint = 0xf20e00ff;
// P2_AGCIQIN0
pub const R0900_P2_AGCIQIN0: c_uint = 0xf20f;
pub const F0900_P2_AGCIQ_VALUE0: c_uint = 0xf20f00ff;
// P2_DEMOD
pub const R0900_P2_DEMOD: c_uint = 0xf210;
pub const F0900_P2_MANUALS2_ROLLOFF: c_uint = 0xf2100080;
pub const F0900_P2_SPECINV_CONTROL: c_uint = 0xf2100030;
pub const F0900_P2_FORCE_ENASAMP: c_uint = 0xf2100008;
pub const F0900_P2_MANUALSX_ROLLOFF: c_uint = 0xf2100004;
pub const F0900_P2_ROLLOFF_CONTROL: c_uint = 0xf2100003;
// P2_DMDMODCOD
pub const R0900_P2_DMDMODCOD: c_uint = 0xf211;
pub const F0900_P2_MANUAL_MODCOD: c_uint = 0xf2110080;
pub const F0900_P2_DEMOD_MODCOD: c_uint = 0xf211007c;
pub const F0900_P2_DEMOD_TYPE: c_uint = 0xf2110003;
// P2_DSTATUS
pub const R0900_P2_DSTATUS: c_uint = 0xf212;
pub const F0900_P2_CAR_LOCK: c_uint = 0xf2120080;
pub const F0900_P2_TMGLOCK_QUALITY: c_uint = 0xf2120060;
pub const F0900_P2_LOCK_DEFINITIF: c_uint = 0xf2120008;
pub const F0900_P2_OVADC_DETECT: c_uint = 0xf2120001;
// P2_DSTATUS2
pub const R0900_P2_DSTATUS2: c_uint = 0xf213;
pub const F0900_P2_DEMOD_DELOCK: c_uint = 0xf2130080;
pub const F0900_P2_AGC1_NOSIGNALACK: c_uint = 0xf2130008;
pub const F0900_P2_AGC2_OVERFLOW: c_uint = 0xf2130004;
pub const F0900_P2_CFR_OVERFLOW: c_uint = 0xf2130002;
pub const F0900_P2_GAMMA_OVERUNDER: c_uint = 0xf2130001;
// P2_DMDCFGMD
pub const R0900_P2_DMDCFGMD: c_uint = 0xf214;
pub const F0900_P2_DVBS2_ENABLE: c_uint = 0xf2140080;
pub const F0900_P2_DVBS1_ENABLE: c_uint = 0xf2140040;
pub const F0900_P2_SCAN_ENABLE: c_uint = 0xf2140010;
pub const F0900_P2_CFR_AUTOSCAN: c_uint = 0xf2140008;
pub const F0900_P2_TUN_RNG: c_uint = 0xf2140003;
// P2_DMDCFG2
pub const R0900_P2_DMDCFG2: c_uint = 0xf215;
pub const F0900_P2_S1S2_SEQUENTIAL: c_uint = 0xf2150040;
pub const F0900_P2_INFINITE_RELOCK: c_uint = 0xf2150010;
// P2_DMDISTATE
pub const R0900_P2_DMDISTATE: c_uint = 0xf216;
pub const F0900_P2_I2C_DEMOD_MODE: c_uint = 0xf216001f;
// P2_DMDT0M
pub const R0900_P2_DMDT0M: c_uint = 0xf217;
pub const F0900_P2_DMDT0_MIN: c_uint = 0xf21700ff;
// P2_DMDSTATE
pub const R0900_P2_DMDSTATE: c_uint = 0xf21b;
pub const F0900_P2_HEADER_MODE: c_uint = 0xf21b0060;
// P2_DMDFLYW
pub const R0900_P2_DMDFLYW: c_uint = 0xf21c;
pub const F0900_P2_I2C_IRQVAL: c_uint = 0xf21c00f0;
pub const F0900_P2_FLYWHEEL_CPT: c_uint = 0xf21c000f;
// P2_DSTATUS3
pub const R0900_P2_DSTATUS3: c_uint = 0xf21d;
pub const F0900_P2_DEMOD_CFGMODE: c_uint = 0xf21d0060;
// P2_DMDCFG3
pub const R0900_P2_DMDCFG3: c_uint = 0xf21e;
pub const F0900_P2_NOSTOP_FIFOFULL: c_uint = 0xf21e0008;
// P2_DMDCFG4
pub const R0900_P2_DMDCFG4: c_uint = 0xf21f;
pub const F0900_P2_TUNER_NRELAUNCH: c_uint = 0xf21f0008;
// P2_CORRELMANT
pub const R0900_P2_CORRELMANT: c_uint = 0xf220;
pub const F0900_P2_CORREL_MANT: c_uint = 0xf22000ff;
// P2_CORRELABS
pub const R0900_P2_CORRELABS: c_uint = 0xf221;
pub const F0900_P2_CORREL_ABS: c_uint = 0xf22100ff;
// P2_CORRELEXP
pub const R0900_P2_CORRELEXP: c_uint = 0xf222;
pub const F0900_P2_CORREL_ABSEXP: c_uint = 0xf22200f0;
pub const F0900_P2_CORREL_EXP: c_uint = 0xf222000f;
// P2_PLHMODCOD
pub const R0900_P2_PLHMODCOD: c_uint = 0xf224;
pub const F0900_P2_SPECINV_DEMOD: c_uint = 0xf2240080;
pub const F0900_P2_PLH_MODCOD: c_uint = 0xf224007c;
pub const F0900_P2_PLH_TYPE: c_uint = 0xf2240003;
// P2_DMDREG
pub const R0900_P2_DMDREG: c_uint = 0xf225;
pub const F0900_P2_DECIM_PLFRAMES: c_uint = 0xf2250001;
// P2_AGC2O
pub const R0900_P2_AGC2O: c_uint = 0xf22c;
pub const F0900_P2_AGC2_COEF: c_uint = 0xf22c0007;
// P2_AGC2REF
pub const R0900_P2_AGC2REF: c_uint = 0xf22d;
pub const F0900_P2_AGC2_REF: c_uint = 0xf22d00ff;
// P2_AGC1ADJ
pub const R0900_P2_AGC1ADJ: c_uint = 0xf22e;
pub const F0900_P2_AGC1_ADJUSTED: c_uint = 0xf22e007f;
// P2_AGC2I1
pub const R0900_P2_AGC2I1: c_uint = 0xf236;
pub const F0900_P2_AGC2_INTEGRATOR1: c_uint = 0xf23600ff;
// P2_AGC2I0
pub const R0900_P2_AGC2I0: c_uint = 0xf237;
pub const F0900_P2_AGC2_INTEGRATOR0: c_uint = 0xf23700ff;
// P2_CARCFG
pub const R0900_P2_CARCFG: c_uint = 0xf238;
pub const F0900_P2_CFRUPLOW_AUTO: c_uint = 0xf2380080;
pub const F0900_P2_CFRUPLOW_TEST: c_uint = 0xf2380040;
pub const F0900_P2_ROTAON: c_uint = 0xf2380004;
pub const F0900_P2_PH_DET_ALGO: c_uint = 0xf2380003;
// P2_ACLC
pub const R0900_P2_ACLC: c_uint = 0xf239;
pub const F0900_P2_CAR_ALPHA_MANT: c_uint = 0xf2390030;
pub const F0900_P2_CAR_ALPHA_EXP: c_uint = 0xf239000f;
// P2_BCLC
pub const R0900_P2_BCLC: c_uint = 0xf23a;
pub const F0900_P2_CAR_BETA_MANT: c_uint = 0xf23a0030;
pub const F0900_P2_CAR_BETA_EXP: c_uint = 0xf23a000f;
// P2_CARFREQ
pub const R0900_P2_CARFREQ: c_uint = 0xf23d;
pub const F0900_P2_KC_COARSE_EXP: c_uint = 0xf23d00f0;
pub const F0900_P2_BETA_FREQ: c_uint = 0xf23d000f;
// P2_CARHDR
pub const R0900_P2_CARHDR: c_uint = 0xf23e;
pub const F0900_P2_K_FREQ_HDR: c_uint = 0xf23e00ff;
// P2_LDT
pub const R0900_P2_LDT: c_uint = 0xf23f;
pub const F0900_P2_CARLOCK_THRES: c_uint = 0xf23f01ff;
// P2_LDT2
pub const R0900_P2_LDT2: c_uint = 0xf240;
pub const F0900_P2_CARLOCK_THRES2: c_uint = 0xf24001ff;
// P2_CFRICFG
pub const R0900_P2_CFRICFG: c_uint = 0xf241;
pub const F0900_P2_NEG_CFRSTEP: c_uint = 0xf2410001;
// P2_CFRUP1
pub const R0900_P2_CFRUP1: c_uint = 0xf242;
pub const F0900_P2_CFR_UP1: c_uint = 0xf24201ff;
// P2_CFRUP0
pub const R0900_P2_CFRUP0: c_uint = 0xf243;
pub const F0900_P2_CFR_UP0: c_uint = 0xf24300ff;
// P2_CFRLOW1
pub const R0900_P2_CFRLOW1: c_uint = 0xf246;
pub const F0900_P2_CFR_LOW1: c_uint = 0xf24601ff;
// P2_CFRLOW0
pub const R0900_P2_CFRLOW0: c_uint = 0xf247;
pub const F0900_P2_CFR_LOW0: c_uint = 0xf24700ff;
// P2_CFRINIT1
pub const R0900_P2_CFRINIT1: c_uint = 0xf248;
pub const F0900_P2_CFR_INIT1: c_uint = 0xf24801ff;
// P2_CFRINIT0
pub const R0900_P2_CFRINIT0: c_uint = 0xf249;
pub const F0900_P2_CFR_INIT0: c_uint = 0xf24900ff;
// P2_CFRINC1
pub const R0900_P2_CFRINC1: c_uint = 0xf24a;
pub const F0900_P2_MANUAL_CFRINC: c_uint = 0xf24a0080;
pub const F0900_P2_CFR_INC1: c_uint = 0xf24a003f;
// P2_CFRINC0
pub const R0900_P2_CFRINC0: c_uint = 0xf24b;
pub const F0900_P2_CFR_INC0: c_uint = 0xf24b00f8;
// P2_CFR2
pub const R0900_P2_CFR2: c_uint = 0xf24c;
pub const F0900_P2_CAR_FREQ2: c_uint = 0xf24c01ff;
// P2_CFR1
pub const R0900_P2_CFR1: c_uint = 0xf24d;
pub const F0900_P2_CAR_FREQ1: c_uint = 0xf24d00ff;
// P2_CFR0
pub const R0900_P2_CFR0: c_uint = 0xf24e;
pub const F0900_P2_CAR_FREQ0: c_uint = 0xf24e00ff;
// P2_LDI
pub const R0900_P2_LDI: c_uint = 0xf24f;
pub const F0900_P2_LOCK_DET_INTEGR: c_uint = 0xf24f01ff;
// P2_TMGCFG
pub const R0900_P2_TMGCFG: c_uint = 0xf250;
pub const F0900_P2_TMGLOCK_BETA: c_uint = 0xf25000c0;
pub const F0900_P2_DO_TIMING_CORR: c_uint = 0xf2500010;
pub const F0900_P2_TMG_MINFREQ: c_uint = 0xf2500003;
// P2_RTC
pub const R0900_P2_RTC: c_uint = 0xf251;
pub const F0900_P2_TMGALPHA_EXP: c_uint = 0xf25100f0;
pub const F0900_P2_TMGBETA_EXP: c_uint = 0xf251000f;
// P2_RTCS2
pub const R0900_P2_RTCS2: c_uint = 0xf252;
pub const F0900_P2_TMGALPHAS2_EXP: c_uint = 0xf25200f0;
pub const F0900_P2_TMGBETAS2_EXP: c_uint = 0xf252000f;
// P2_TMGTHRISE
pub const R0900_P2_TMGTHRISE: c_uint = 0xf253;
pub const F0900_P2_TMGLOCK_THRISE: c_uint = 0xf25300ff;
// P2_TMGTHFALL
pub const R0900_P2_TMGTHFALL: c_uint = 0xf254;
pub const F0900_P2_TMGLOCK_THFALL: c_uint = 0xf25400ff;
// P2_SFRUPRATIO
pub const R0900_P2_SFRUPRATIO: c_uint = 0xf255;
pub const F0900_P2_SFR_UPRATIO: c_uint = 0xf25500ff;
// P2_SFRLOWRATIO
pub const R0900_P2_SFRLOWRATIO: c_uint = 0xf256;
pub const F0900_P2_SFR_LOWRATIO: c_uint = 0xf25600ff;
// P2_KREFTMG
pub const R0900_P2_KREFTMG: c_uint = 0xf258;
pub const F0900_P2_KREF_TMG: c_uint = 0xf25800ff;
// P2_SFRSTEP
pub const R0900_P2_SFRSTEP: c_uint = 0xf259;
pub const F0900_P2_SFR_SCANSTEP: c_uint = 0xf25900f0;
pub const F0900_P2_SFR_CENTERSTEP: c_uint = 0xf259000f;
// P2_TMGCFG2
pub const R0900_P2_TMGCFG2: c_uint = 0xf25a;
pub const F0900_P2_SFRRATIO_FINE: c_uint = 0xf25a0001;
// P2_KREFTMG2
pub const R0900_P2_KREFTMG2: c_uint = 0xf25b;
pub const F0900_P2_KREF_TMG2: c_uint = 0xf25b00ff;
// P2_SFRINIT1
pub const R0900_P2_SFRINIT1: c_uint = 0xf25e;
pub const F0900_P2_SFR_INIT1: c_uint = 0xf25e007f;
// P2_SFRINIT0
pub const R0900_P2_SFRINIT0: c_uint = 0xf25f;
pub const F0900_P2_SFR_INIT0: c_uint = 0xf25f00ff;
// P2_SFRUP1
pub const R0900_P2_SFRUP1: c_uint = 0xf260;
pub const F0900_P2_AUTO_GUP: c_uint = 0xf2600080;
pub const F0900_P2_SYMB_FREQ_UP1: c_uint = 0xf260007f;
// P2_SFRUP0
pub const R0900_P2_SFRUP0: c_uint = 0xf261;
pub const F0900_P2_SYMB_FREQ_UP0: c_uint = 0xf26100ff;
// P2_SFRLOW1
pub const R0900_P2_SFRLOW1: c_uint = 0xf262;
pub const F0900_P2_AUTO_GLOW: c_uint = 0xf2620080;
pub const F0900_P2_SYMB_FREQ_LOW1: c_uint = 0xf262007f;
// P2_SFRLOW0
pub const R0900_P2_SFRLOW0: c_uint = 0xf263;
pub const F0900_P2_SYMB_FREQ_LOW0: c_uint = 0xf26300ff;
// P2_SFR3
pub const R0900_P2_SFR3: c_uint = 0xf264;
pub const F0900_P2_SYMB_FREQ3: c_uint = 0xf26400ff;
// P2_SFR2
pub const R0900_P2_SFR2: c_uint = 0xf265;
pub const F0900_P2_SYMB_FREQ2: c_uint = 0xf26500ff;
// P2_SFR1
pub const R0900_P2_SFR1: c_uint = 0xf266;
pub const F0900_P2_SYMB_FREQ1: c_uint = 0xf26600ff;
// P2_SFR0
pub const R0900_P2_SFR0: c_uint = 0xf267;
pub const F0900_P2_SYMB_FREQ0: c_uint = 0xf26700ff;
// P2_TMGREG2
pub const R0900_P2_TMGREG2: c_uint = 0xf268;
pub const F0900_P2_TMGREG2: c_uint = 0xf26800ff;
// P2_TMGREG1
pub const R0900_P2_TMGREG1: c_uint = 0xf269;
pub const F0900_P2_TMGREG1: c_uint = 0xf26900ff;
// P2_TMGREG0
pub const R0900_P2_TMGREG0: c_uint = 0xf26a;
pub const F0900_P2_TMGREG0: c_uint = 0xf26a00ff;
// P2_TMGLOCK1
pub const R0900_P2_TMGLOCK1: c_uint = 0xf26b;
pub const F0900_P2_TMGLOCK_LEVEL1: c_uint = 0xf26b01ff;
// P2_TMGLOCK0
pub const R0900_P2_TMGLOCK0: c_uint = 0xf26c;
pub const F0900_P2_TMGLOCK_LEVEL0: c_uint = 0xf26c00ff;
// P2_TMGOBS
pub const R0900_P2_TMGOBS: c_uint = 0xf26d;
pub const F0900_P2_ROLLOFF_STATUS: c_uint = 0xf26d00c0;
// P2_EQUALCFG
pub const R0900_P2_EQUALCFG: c_uint = 0xf26f;
pub const F0900_P2_EQUAL_ON: c_uint = 0xf26f0040;
pub const F0900_P2_MU_EQUALDFE: c_uint = 0xf26f0007;
// P2_EQUAI1
pub const R0900_P2_EQUAI1: c_uint = 0xf270;
pub const F0900_P2_EQUA_ACCI1: c_uint = 0xf27001ff;
// P2_EQUAQ1
pub const R0900_P2_EQUAQ1: c_uint = 0xf271;
pub const F0900_P2_EQUA_ACCQ1: c_uint = 0xf27101ff;
// P2_EQUAI2
pub const R0900_P2_EQUAI2: c_uint = 0xf272;
pub const F0900_P2_EQUA_ACCI2: c_uint = 0xf27201ff;
// P2_EQUAQ2
pub const R0900_P2_EQUAQ2: c_uint = 0xf273;
pub const F0900_P2_EQUA_ACCQ2: c_uint = 0xf27301ff;
// P2_EQUAI3
pub const R0900_P2_EQUAI3: c_uint = 0xf274;
pub const F0900_P2_EQUA_ACCI3: c_uint = 0xf27401ff;
// P2_EQUAQ3
pub const R0900_P2_EQUAQ3: c_uint = 0xf275;
pub const F0900_P2_EQUA_ACCQ3: c_uint = 0xf27501ff;
// P2_EQUAI4
pub const R0900_P2_EQUAI4: c_uint = 0xf276;
pub const F0900_P2_EQUA_ACCI4: c_uint = 0xf27601ff;
// P2_EQUAQ4
pub const R0900_P2_EQUAQ4: c_uint = 0xf277;
pub const F0900_P2_EQUA_ACCQ4: c_uint = 0xf27701ff;
// P2_EQUAI5
pub const R0900_P2_EQUAI5: c_uint = 0xf278;
pub const F0900_P2_EQUA_ACCI5: c_uint = 0xf27801ff;
// P2_EQUAQ5
pub const R0900_P2_EQUAQ5: c_uint = 0xf279;
pub const F0900_P2_EQUA_ACCQ5: c_uint = 0xf27901ff;
// P2_EQUAI6
pub const R0900_P2_EQUAI6: c_uint = 0xf27a;
pub const F0900_P2_EQUA_ACCI6: c_uint = 0xf27a01ff;
// P2_EQUAQ6
pub const R0900_P2_EQUAQ6: c_uint = 0xf27b;
pub const F0900_P2_EQUA_ACCQ6: c_uint = 0xf27b01ff;
// P2_EQUAI7
pub const R0900_P2_EQUAI7: c_uint = 0xf27c;
pub const F0900_P2_EQUA_ACCI7: c_uint = 0xf27c01ff;
// P2_EQUAQ7
pub const R0900_P2_EQUAQ7: c_uint = 0xf27d;
pub const F0900_P2_EQUA_ACCQ7: c_uint = 0xf27d01ff;
// P2_EQUAI8
pub const R0900_P2_EQUAI8: c_uint = 0xf27e;
pub const F0900_P2_EQUA_ACCI8: c_uint = 0xf27e01ff;
// P2_EQUAQ8
pub const R0900_P2_EQUAQ8: c_uint = 0xf27f;
pub const F0900_P2_EQUA_ACCQ8: c_uint = 0xf27f01ff;
// P2_NNOSDATAT1
pub const R0900_P2_NNOSDATAT1: c_uint = 0xf280;
pub const F0900_P2_NOSDATAT_NORMED1: c_uint = 0xf28000ff;
// P2_NNOSDATAT0
pub const R0900_P2_NNOSDATAT0: c_uint = 0xf281;
pub const F0900_P2_NOSDATAT_NORMED0: c_uint = 0xf28100ff;
// P2_NNOSDATA1
pub const R0900_P2_NNOSDATA1: c_uint = 0xf282;
pub const F0900_P2_NOSDATA_NORMED1: c_uint = 0xf28200ff;
// P2_NNOSDATA0
pub const R0900_P2_NNOSDATA0: c_uint = 0xf283;
pub const F0900_P2_NOSDATA_NORMED0: c_uint = 0xf28300ff;
// P2_NNOSPLHT1
pub const R0900_P2_NNOSPLHT1: c_uint = 0xf284;
pub const F0900_P2_NOSPLHT_NORMED1: c_uint = 0xf28400ff;
// P2_NNOSPLHT0
pub const R0900_P2_NNOSPLHT0: c_uint = 0xf285;
pub const F0900_P2_NOSPLHT_NORMED0: c_uint = 0xf28500ff;
// P2_NNOSPLH1
pub const R0900_P2_NNOSPLH1: c_uint = 0xf286;
pub const F0900_P2_NOSPLH_NORMED1: c_uint = 0xf28600ff;
// P2_NNOSPLH0
pub const R0900_P2_NNOSPLH0: c_uint = 0xf287;
pub const F0900_P2_NOSPLH_NORMED0: c_uint = 0xf28700ff;
// P2_NOSDATAT1
pub const R0900_P2_NOSDATAT1: c_uint = 0xf288;
pub const F0900_P2_NOSDATAT_UNNORMED1: c_uint = 0xf28800ff;
// P2_NOSDATAT0
pub const R0900_P2_NOSDATAT0: c_uint = 0xf289;
pub const F0900_P2_NOSDATAT_UNNORMED0: c_uint = 0xf28900ff;
// P2_NOSDATA1
pub const R0900_P2_NOSDATA1: c_uint = 0xf28a;
pub const F0900_P2_NOSDATA_UNNORMED1: c_uint = 0xf28a00ff;
// P2_NOSDATA0
pub const R0900_P2_NOSDATA0: c_uint = 0xf28b;
pub const F0900_P2_NOSDATA_UNNORMED0: c_uint = 0xf28b00ff;
// P2_NOSPLHT1
pub const R0900_P2_NOSPLHT1: c_uint = 0xf28c;
pub const F0900_P2_NOSPLHT_UNNORMED1: c_uint = 0xf28c00ff;
// P2_NOSPLHT0
pub const R0900_P2_NOSPLHT0: c_uint = 0xf28d;
pub const F0900_P2_NOSPLHT_UNNORMED0: c_uint = 0xf28d00ff;
// P2_NOSPLH1
pub const R0900_P2_NOSPLH1: c_uint = 0xf28e;
pub const F0900_P2_NOSPLH_UNNORMED1: c_uint = 0xf28e00ff;
// P2_NOSPLH0
pub const R0900_P2_NOSPLH0: c_uint = 0xf28f;
pub const F0900_P2_NOSPLH_UNNORMED0: c_uint = 0xf28f00ff;
// P2_CAR2CFG
pub const R0900_P2_CAR2CFG: c_uint = 0xf290;
pub const F0900_P2_CARRIER3_DISABLE: c_uint = 0xf2900040;
pub const F0900_P2_ROTA2ON: c_uint = 0xf2900004;
pub const F0900_P2_PH_DET_ALGO2: c_uint = 0xf2900003;
// P2_CFR2CFR1
pub const R0900_P2_CFR2CFR1: c_uint = 0xf291;
pub const F0900_P2_CFR2TOCFR1_DVBS1: c_uint = 0xf29100c0;
pub const F0900_P2_EN_S2CAR2CENTER: c_uint = 0xf2910020;
pub const F0900_P2_DIS_BCHERRCFR2: c_uint = 0xf2910010;
pub const F0900_P2_CFR2TOCFR1_BETA: c_uint = 0xf2910007;
// P2_CFR22
pub const R0900_P2_CFR22: c_uint = 0xf293;
pub const F0900_P2_CAR2_FREQ2: c_uint = 0xf29301ff;
// P2_CFR21
pub const R0900_P2_CFR21: c_uint = 0xf294;
pub const F0900_P2_CAR2_FREQ1: c_uint = 0xf29400ff;
// P2_CFR20
pub const R0900_P2_CFR20: c_uint = 0xf295;
pub const F0900_P2_CAR2_FREQ0: c_uint = 0xf29500ff;
// P2_ACLC2S2Q
pub const R0900_P2_ACLC2S2Q: c_uint = 0xf297;
pub const F0900_P2_ENAB_SPSKSYMB: c_uint = 0xf2970080;
pub const F0900_P2_CAR2S2_Q_ALPH_M: c_uint = 0xf2970030;
pub const F0900_P2_CAR2S2_Q_ALPH_E: c_uint = 0xf297000f;
// P2_ACLC2S28
pub const R0900_P2_ACLC2S28: c_uint = 0xf298;
pub const F0900_P2_OLDI3Q_MODE: c_uint = 0xf2980080;
pub const F0900_P2_CAR2S2_8_ALPH_M: c_uint = 0xf2980030;
pub const F0900_P2_CAR2S2_8_ALPH_E: c_uint = 0xf298000f;
// P2_ACLC2S216A
pub const R0900_P2_ACLC2S216A: c_uint = 0xf299;
pub const F0900_P2_DIS_C3STOPA2: c_uint = 0xf2990080;
pub const F0900_P2_CAR2S2_16ADERAT: c_uint = 0xf2990040;
pub const F0900_P2_CAR2S2_16A_ALPH_M: c_uint = 0xf2990030;
pub const F0900_P2_CAR2S2_16A_ALPH_E: c_uint = 0xf299000f;
// P2_ACLC2S232A
pub const R0900_P2_ACLC2S232A: c_uint = 0xf29a;
pub const F0900_P2_CAR2S2_32ADERAT: c_uint = 0xf29a0040;
pub const F0900_P2_CAR2S2_32A_ALPH_M: c_uint = 0xf29a0030;
pub const F0900_P2_CAR2S2_32A_ALPH_E: c_uint = 0xf29a000f;
// P2_BCLC2S2Q
pub const R0900_P2_BCLC2S2Q: c_uint = 0xf29c;
pub const F0900_P2_CAR2S2_Q_BETA_M: c_uint = 0xf29c0030;
pub const F0900_P2_CAR2S2_Q_BETA_E: c_uint = 0xf29c000f;
// P2_BCLC2S28
pub const R0900_P2_BCLC2S28: c_uint = 0xf29d;
pub const F0900_P2_CAR2S2_8_BETA_M: c_uint = 0xf29d0030;
pub const F0900_P2_CAR2S2_8_BETA_E: c_uint = 0xf29d000f;
// P2_BCLC2S216A
pub const R0900_P2_BCLC2S216A: c_uint = 0xf29e;
// P2_BCLC2S232A
pub const R0900_P2_BCLC2S232A: c_uint = 0xf29f;
// P2_PLROOT2
pub const R0900_P2_PLROOT2: c_uint = 0xf2ac;
pub const F0900_P2_PLSCRAMB_MODE: c_uint = 0xf2ac000c;
pub const F0900_P2_PLSCRAMB_ROOT2: c_uint = 0xf2ac0003;
// P2_PLROOT1
pub const R0900_P2_PLROOT1: c_uint = 0xf2ad;
pub const F0900_P2_PLSCRAMB_ROOT1: c_uint = 0xf2ad00ff;
// P2_PLROOT0
pub const R0900_P2_PLROOT0: c_uint = 0xf2ae;
pub const F0900_P2_PLSCRAMB_ROOT0: c_uint = 0xf2ae00ff;
// P2_MODCODLST0
pub const R0900_P2_MODCODLST0: c_uint = 0xf2b0;
// P2_MODCODLST1
pub const R0900_P2_MODCODLST1: c_uint = 0xf2b1;
pub const F0900_P2_DIS_MODCOD29: c_uint = 0xf2b100f0;
pub const F0900_P2_DIS_32PSK_9_10: c_uint = 0xf2b1000f;
// P2_MODCODLST2
pub const R0900_P2_MODCODLST2: c_uint = 0xf2b2;
pub const F0900_P2_DIS_32PSK_8_9: c_uint = 0xf2b200f0;
pub const F0900_P2_DIS_32PSK_5_6: c_uint = 0xf2b2000f;
// P2_MODCODLST3
pub const R0900_P2_MODCODLST3: c_uint = 0xf2b3;
pub const F0900_P2_DIS_32PSK_4_5: c_uint = 0xf2b300f0;
pub const F0900_P2_DIS_32PSK_3_4: c_uint = 0xf2b3000f;
// P2_MODCODLST4
pub const R0900_P2_MODCODLST4: c_uint = 0xf2b4;
pub const F0900_P2_DIS_16PSK_9_10: c_uint = 0xf2b400f0;
pub const F0900_P2_DIS_16PSK_8_9: c_uint = 0xf2b4000f;
// P2_MODCODLST5
pub const R0900_P2_MODCODLST5: c_uint = 0xf2b5;
pub const F0900_P2_DIS_16PSK_5_6: c_uint = 0xf2b500f0;
pub const F0900_P2_DIS_16PSK_4_5: c_uint = 0xf2b5000f;
// P2_MODCODLST6
pub const R0900_P2_MODCODLST6: c_uint = 0xf2b6;
pub const F0900_P2_DIS_16PSK_3_4: c_uint = 0xf2b600f0;
pub const F0900_P2_DIS_16PSK_2_3: c_uint = 0xf2b6000f;
// P2_MODCODLST7
pub const R0900_P2_MODCODLST7: c_uint = 0xf2b7;
pub const F0900_P2_DIS_8P_9_10: c_uint = 0xf2b700f0;
pub const F0900_P2_DIS_8P_8_9: c_uint = 0xf2b7000f;
// P2_MODCODLST8
pub const R0900_P2_MODCODLST8: c_uint = 0xf2b8;
pub const F0900_P2_DIS_8P_5_6: c_uint = 0xf2b800f0;
pub const F0900_P2_DIS_8P_3_4: c_uint = 0xf2b8000f;
// P2_MODCODLST9
pub const R0900_P2_MODCODLST9: c_uint = 0xf2b9;
pub const F0900_P2_DIS_8P_2_3: c_uint = 0xf2b900f0;
pub const F0900_P2_DIS_8P_3_5: c_uint = 0xf2b9000f;
// P2_MODCODLSTA
pub const R0900_P2_MODCODLSTA: c_uint = 0xf2ba;
pub const F0900_P2_DIS_QP_9_10: c_uint = 0xf2ba00f0;
pub const F0900_P2_DIS_QP_8_9: c_uint = 0xf2ba000f;
// P2_MODCODLSTB
pub const R0900_P2_MODCODLSTB: c_uint = 0xf2bb;
pub const F0900_P2_DIS_QP_5_6: c_uint = 0xf2bb00f0;
pub const F0900_P2_DIS_QP_4_5: c_uint = 0xf2bb000f;
// P2_MODCODLSTC
pub const R0900_P2_MODCODLSTC: c_uint = 0xf2bc;
pub const F0900_P2_DIS_QP_3_4: c_uint = 0xf2bc00f0;
pub const F0900_P2_DIS_QP_2_3: c_uint = 0xf2bc000f;
// P2_MODCODLSTD
pub const R0900_P2_MODCODLSTD: c_uint = 0xf2bd;
pub const F0900_P2_DIS_QP_3_5: c_uint = 0xf2bd00f0;
pub const F0900_P2_DIS_QP_1_2: c_uint = 0xf2bd000f;
// P2_MODCODLSTE
pub const R0900_P2_MODCODLSTE: c_uint = 0xf2be;
pub const F0900_P2_DIS_QP_2_5: c_uint = 0xf2be00f0;
pub const F0900_P2_DIS_QP_1_3: c_uint = 0xf2be000f;
// P2_MODCODLSTF
pub const R0900_P2_MODCODLSTF: c_uint = 0xf2bf;
pub const F0900_P2_DIS_QP_1_4: c_uint = 0xf2bf00f0;
// P2_GAUSSR0
pub const R0900_P2_GAUSSR0: c_uint = 0xf2c0;
pub const F0900_P2_EN_CCIMODE: c_uint = 0xf2c00080;
pub const F0900_P2_R0_GAUSSIEN: c_uint = 0xf2c0007f;
// P2_CCIR0
pub const R0900_P2_CCIR0: c_uint = 0xf2c1;
pub const F0900_P2_CCIDETECT_PLHONLY: c_uint = 0xf2c10080;
pub const F0900_P2_R0_CCI: c_uint = 0xf2c1007f;
// P2_CCIQUANT
pub const R0900_P2_CCIQUANT: c_uint = 0xf2c2;
pub const F0900_P2_CCI_BETA: c_uint = 0xf2c200e0;
pub const F0900_P2_CCI_QUANT: c_uint = 0xf2c2001f;
// P2_CCITHRES
pub const R0900_P2_CCITHRES: c_uint = 0xf2c3;
pub const F0900_P2_CCI_THRESHOLD: c_uint = 0xf2c300ff;
// P2_CCIACC
pub const R0900_P2_CCIACC: c_uint = 0xf2c4;
pub const F0900_P2_CCI_VALUE: c_uint = 0xf2c400ff;
// P2_DMDRESCFG
pub const R0900_P2_DMDRESCFG: c_uint = 0xf2c6;
pub const F0900_P2_DMDRES_RESET: c_uint = 0xf2c60080;
pub const F0900_P2_DMDRES_STRALL: c_uint = 0xf2c60008;
pub const F0900_P2_DMDRES_NEWONLY: c_uint = 0xf2c60004;
pub const F0900_P2_DMDRES_NOSTORE: c_uint = 0xf2c60002;
// P2_DMDRESADR
pub const R0900_P2_DMDRESADR: c_uint = 0xf2c7;
pub const F0900_P2_DMDRES_VALIDCFR: c_uint = 0xf2c70040;
pub const F0900_P2_DMDRES_MEMFULL: c_uint = 0xf2c70030;
pub const F0900_P2_DMDRES_RESNBR: c_uint = 0xf2c7000f;
// P2_DMDRESDATA7
pub const R0900_P2_DMDRESDATA7: c_uint = 0xf2c8;
pub const F0900_P2_DMDRES_DATA7: c_uint = 0xf2c800ff;
// P2_DMDRESDATA6
pub const R0900_P2_DMDRESDATA6: c_uint = 0xf2c9;
pub const F0900_P2_DMDRES_DATA6: c_uint = 0xf2c900ff;
// P2_DMDRESDATA5
pub const R0900_P2_DMDRESDATA5: c_uint = 0xf2ca;
pub const F0900_P2_DMDRES_DATA5: c_uint = 0xf2ca00ff;
// P2_DMDRESDATA4
pub const R0900_P2_DMDRESDATA4: c_uint = 0xf2cb;
pub const F0900_P2_DMDRES_DATA4: c_uint = 0xf2cb00ff;
// P2_DMDRESDATA3
pub const R0900_P2_DMDRESDATA3: c_uint = 0xf2cc;
pub const F0900_P2_DMDRES_DATA3: c_uint = 0xf2cc00ff;
// P2_DMDRESDATA2
pub const R0900_P2_DMDRESDATA2: c_uint = 0xf2cd;
pub const F0900_P2_DMDRES_DATA2: c_uint = 0xf2cd00ff;
// P2_DMDRESDATA1
pub const R0900_P2_DMDRESDATA1: c_uint = 0xf2ce;
pub const F0900_P2_DMDRES_DATA1: c_uint = 0xf2ce00ff;
// P2_DMDRESDATA0
pub const R0900_P2_DMDRESDATA0: c_uint = 0xf2cf;
pub const F0900_P2_DMDRES_DATA0: c_uint = 0xf2cf00ff;
// P2_FFEI1
pub const R0900_P2_FFEI1: c_uint = 0xf2d0;
pub const F0900_P2_FFE_ACCI1: c_uint = 0xf2d001ff;
// P2_FFEQ1
pub const R0900_P2_FFEQ1: c_uint = 0xf2d1;
pub const F0900_P2_FFE_ACCQ1: c_uint = 0xf2d101ff;
// P2_FFEI2
pub const R0900_P2_FFEI2: c_uint = 0xf2d2;
pub const F0900_P2_FFE_ACCI2: c_uint = 0xf2d201ff;
// P2_FFEQ2
pub const R0900_P2_FFEQ2: c_uint = 0xf2d3;
pub const F0900_P2_FFE_ACCQ2: c_uint = 0xf2d301ff;
// P2_FFEI3
pub const R0900_P2_FFEI3: c_uint = 0xf2d4;
pub const F0900_P2_FFE_ACCI3: c_uint = 0xf2d401ff;
// P2_FFEQ3
pub const R0900_P2_FFEQ3: c_uint = 0xf2d5;
pub const F0900_P2_FFE_ACCQ3: c_uint = 0xf2d501ff;
// P2_FFEI4
pub const R0900_P2_FFEI4: c_uint = 0xf2d6;
pub const F0900_P2_FFE_ACCI4: c_uint = 0xf2d601ff;
// P2_FFEQ4
pub const R0900_P2_FFEQ4: c_uint = 0xf2d7;
pub const F0900_P2_FFE_ACCQ4: c_uint = 0xf2d701ff;
// P2_FFECFG
pub const R0900_P2_FFECFG: c_uint = 0xf2d8;
pub const F0900_P2_EQUALFFE_ON: c_uint = 0xf2d80040;
pub const F0900_P2_MU_EQUALFFE: c_uint = 0xf2d80007;
// P2_TNRCFG
pub const R0900_P2_TNRCFG: c_uint = 0xf2e0;
pub const F0900_P2_TUN_ACKFAIL: c_uint = 0xf2e00080;
pub const F0900_P2_TUN_TYPE: c_uint = 0xf2e00070;
pub const F0900_P2_TUN_SECSTOP: c_uint = 0xf2e00008;
pub const F0900_P2_TUN_VCOSRCH: c_uint = 0xf2e00004;
pub const F0900_P2_TUN_MADDRESS: c_uint = 0xf2e00003;
// P2_TNRCFG2
pub const R0900_P2_TNRCFG2: c_uint = 0xf2e1;
pub const F0900_P2_TUN_IQSWAP: c_uint = 0xf2e10080;
pub const F0900_P2_DIS_BWCALC: c_uint = 0xf2e10004;
pub const F0900_P2_SHORT_WAITSTATES: c_uint = 0xf2e10002;
// P2_TNRXTAL
pub const R0900_P2_TNRXTAL: c_uint = 0xf2e4;
pub const F0900_P2_TUN_XTALFREQ: c_uint = 0xf2e4001f;
// P2_TNRSTEPS
pub const R0900_P2_TNRSTEPS: c_uint = 0xf2e7;
pub const F0900_P2_TUNER_BW0P125: c_uint = 0xf2e70080;
pub const F0900_P2_BWINC_OFFSET: c_uint = 0xf2e70170;
pub const F0900_P2_SOFTSTEP_RNG: c_uint = 0xf2e70008;
pub const F0900_P2_TUN_BWOFFSET: c_uint = 0xf2e70007;
// P2_TNRGAIN
pub const R0900_P2_TNRGAIN: c_uint = 0xf2e8;
pub const F0900_P2_TUN_KDIVEN: c_uint = 0xf2e800c0;
pub const F0900_P2_STB6X00_OCK: c_uint = 0xf2e80030;
pub const F0900_P2_TUN_GAIN: c_uint = 0xf2e8000f;
// P2_TNRRF1
pub const R0900_P2_TNRRF1: c_uint = 0xf2e9;
pub const F0900_P2_TUN_RFFREQ2: c_uint = 0xf2e900ff;
// P2_TNRRF0
pub const R0900_P2_TNRRF0: c_uint = 0xf2ea;
pub const F0900_P2_TUN_RFFREQ1: c_uint = 0xf2ea00ff;
// P2_TNRBW
pub const R0900_P2_TNRBW: c_uint = 0xf2eb;
pub const F0900_P2_TUN_RFFREQ0: c_uint = 0xf2eb00c0;
pub const F0900_P2_TUN_BW: c_uint = 0xf2eb003f;
// P2_TNRADJ
pub const R0900_P2_TNRADJ: c_uint = 0xf2ec;
pub const F0900_P2_STB61X0_CALTIME: c_uint = 0xf2ec0040;
// P2_TNRCTL2
pub const R0900_P2_TNRCTL2: c_uint = 0xf2ed;
pub const F0900_P2_STB61X0_RCCKOFF: c_uint = 0xf2ed0080;
pub const F0900_P2_STB61X0_ICP_SDOFF: c_uint = 0xf2ed0040;
pub const F0900_P2_STB61X0_DCLOOPOFF: c_uint = 0xf2ed0020;
pub const F0900_P2_STB61X0_REFOUTSEL: c_uint = 0xf2ed0010;
pub const F0900_P2_STB61X0_CALOFF: c_uint = 0xf2ed0008;
pub const F0900_P2_STB6XX0_LPT_BEN: c_uint = 0xf2ed0004;
pub const F0900_P2_STB6XX0_RX_OSCP: c_uint = 0xf2ed0002;
pub const F0900_P2_STB6XX0_SYN: c_uint = 0xf2ed0001;
// P2_TNRCFG3
pub const R0900_P2_TNRCFG3: c_uint = 0xf2ee;
pub const F0900_P2_TUN_PLLFREQ: c_uint = 0xf2ee001c;
pub const F0900_P2_TUN_I2CFREQ_MODE: c_uint = 0xf2ee0003;
// P2_TNRLAUNCH
pub const R0900_P2_TNRLAUNCH: c_uint = 0xf2f0;
// P2_TNRLD
pub const R0900_P2_TNRLD: c_uint = 0xf2f0;
pub const F0900_P2_TUNLD_VCOING: c_uint = 0xf2f00080;
pub const F0900_P2_TUN_REG1FAIL: c_uint = 0xf2f00040;
pub const F0900_P2_TUN_REG2FAIL: c_uint = 0xf2f00020;
pub const F0900_P2_TUN_REG3FAIL: c_uint = 0xf2f00010;
pub const F0900_P2_TUN_REG4FAIL: c_uint = 0xf2f00008;
pub const F0900_P2_TUN_REG5FAIL: c_uint = 0xf2f00004;
pub const F0900_P2_TUN_BWING: c_uint = 0xf2f00002;
pub const F0900_P2_TUN_LOCKED: c_uint = 0xf2f00001;
// P2_TNROBSL
pub const R0900_P2_TNROBSL: c_uint = 0xf2f6;
pub const F0900_P2_TUN_I2CABORTED: c_uint = 0xf2f60080;
pub const F0900_P2_TUN_LPEN: c_uint = 0xf2f60040;
pub const F0900_P2_TUN_FCCK: c_uint = 0xf2f60020;
pub const F0900_P2_TUN_I2CLOCKED: c_uint = 0xf2f60010;
pub const F0900_P2_TUN_PROGDONE: c_uint = 0xf2f6000c;
pub const F0900_P2_TUN_RFRESTE1: c_uint = 0xf2f60003;
// P2_TNRRESTE
pub const R0900_P2_TNRRESTE: c_uint = 0xf2f7;
pub const F0900_P2_TUN_RFRESTE0: c_uint = 0xf2f700ff;
// P2_SMAPCOEF7
pub const R0900_P2_SMAPCOEF7: c_uint = 0xf300;
pub const F0900_P2_DIS_QSCALE: c_uint = 0xf3000080;
pub const F0900_P2_SMAPCOEF_Q_LLR12: c_uint = 0xf300017f;
// P2_SMAPCOEF6
pub const R0900_P2_SMAPCOEF6: c_uint = 0xf301;
pub const F0900_P2_ADJ_8PSKLLR1: c_uint = 0xf3010004;
pub const F0900_P2_OLD_8PSKLLR1: c_uint = 0xf3010002;
pub const F0900_P2_DIS_AB8PSK: c_uint = 0xf3010001;
// P2_SMAPCOEF5
pub const R0900_P2_SMAPCOEF5: c_uint = 0xf302;
pub const F0900_P2_DIS_8SCALE: c_uint = 0xf3020080;
pub const F0900_P2_SMAPCOEF_8P_LLR23: c_uint = 0xf302017f;
// P2_NCO2MAX1
pub const R0900_P2_NCO2MAX1: c_uint = 0xf314;
pub const F0900_P2_TETA2_MAXVABS1: c_uint = 0xf31400ff;
// P2_NCO2MAX0
pub const R0900_P2_NCO2MAX0: c_uint = 0xf315;
pub const F0900_P2_TETA2_MAXVABS0: c_uint = 0xf31500ff;
// P2_NCO2FR1
pub const R0900_P2_NCO2FR1: c_uint = 0xf316;
pub const F0900_P2_NCO2FINAL_ANGLE1: c_uint = 0xf31600ff;
// P2_NCO2FR0
pub const R0900_P2_NCO2FR0: c_uint = 0xf317;
pub const F0900_P2_NCO2FINAL_ANGLE0: c_uint = 0xf31700ff;
// P2_CFR2AVRGE1
pub const R0900_P2_CFR2AVRGE1: c_uint = 0xf318;
pub const F0900_P2_I2C_CFR2AVERAGE1: c_uint = 0xf31800ff;
// P2_CFR2AVRGE0
pub const R0900_P2_CFR2AVRGE0: c_uint = 0xf319;
pub const F0900_P2_I2C_CFR2AVERAGE0: c_uint = 0xf31900ff;
// P2_DMDPLHSTAT
pub const R0900_P2_DMDPLHSTAT: c_uint = 0xf320;
pub const F0900_P2_PLH_STATISTIC: c_uint = 0xf32000ff;
// P2_LOCKTIME3
pub const R0900_P2_LOCKTIME3: c_uint = 0xf322;
pub const F0900_P2_DEMOD_LOCKTIME3: c_uint = 0xf32200ff;
// P2_LOCKTIME2
pub const R0900_P2_LOCKTIME2: c_uint = 0xf323;
pub const F0900_P2_DEMOD_LOCKTIME2: c_uint = 0xf32300ff;
// P2_LOCKTIME1
pub const R0900_P2_LOCKTIME1: c_uint = 0xf324;
pub const F0900_P2_DEMOD_LOCKTIME1: c_uint = 0xf32400ff;
// P2_LOCKTIME0
pub const R0900_P2_LOCKTIME0: c_uint = 0xf325;
pub const F0900_P2_DEMOD_LOCKTIME0: c_uint = 0xf32500ff;
// P2_VITSCALE
pub const R0900_P2_VITSCALE: c_uint = 0xf332;
pub const F0900_P2_NVTH_NOSRANGE: c_uint = 0xf3320080;
pub const F0900_P2_VERROR_MAXMODE: c_uint = 0xf3320040;
pub const F0900_P2_NSLOWSN_LOCKED: c_uint = 0xf3320008;
pub const F0900_P2_DIS_RSFLOCK: c_uint = 0xf3320002;
// P2_FECM
pub const R0900_P2_FECM: c_uint = 0xf333;
pub const F0900_P2_DSS_DVB: c_uint = 0xf3330080;
pub const F0900_P2_DSS_SRCH: c_uint = 0xf3330010;
pub const F0900_P2_SYNCVIT: c_uint = 0xf3330002;
pub const F0900_P2_IQINV: c_uint = 0xf3330001;
// P2_VTH12
pub const R0900_P2_VTH12: c_uint = 0xf334;
pub const F0900_P2_VTH12: c_uint = 0xf33400ff;
// P2_VTH23
pub const R0900_P2_VTH23: c_uint = 0xf335;
pub const F0900_P2_VTH23: c_uint = 0xf33500ff;
// P2_VTH34
pub const R0900_P2_VTH34: c_uint = 0xf336;
pub const F0900_P2_VTH34: c_uint = 0xf33600ff;
// P2_VTH56
pub const R0900_P2_VTH56: c_uint = 0xf337;
pub const F0900_P2_VTH56: c_uint = 0xf33700ff;
// P2_VTH67
pub const R0900_P2_VTH67: c_uint = 0xf338;
pub const F0900_P2_VTH67: c_uint = 0xf33800ff;
// P2_VTH78
pub const R0900_P2_VTH78: c_uint = 0xf339;
pub const F0900_P2_VTH78: c_uint = 0xf33900ff;
// P2_VITCURPUN
pub const R0900_P2_VITCURPUN: c_uint = 0xf33a;
pub const F0900_P2_VIT_CURPUN: c_uint = 0xf33a001f;
// P2_VERROR
pub const R0900_P2_VERROR: c_uint = 0xf33b;
pub const F0900_P2_REGERR_VIT: c_uint = 0xf33b00ff;
// P2_PRVIT
pub const R0900_P2_PRVIT: c_uint = 0xf33c;
pub const F0900_P2_DIS_VTHLOCK: c_uint = 0xf33c0040;
pub const F0900_P2_E7_8VIT: c_uint = 0xf33c0020;
pub const F0900_P2_E6_7VIT: c_uint = 0xf33c0010;
pub const F0900_P2_E5_6VIT: c_uint = 0xf33c0008;
pub const F0900_P2_E3_4VIT: c_uint = 0xf33c0004;
pub const F0900_P2_E2_3VIT: c_uint = 0xf33c0002;
pub const F0900_P2_E1_2VIT: c_uint = 0xf33c0001;
// P2_VAVSRVIT
pub const R0900_P2_VAVSRVIT: c_uint = 0xf33d;
pub const F0900_P2_AMVIT: c_uint = 0xf33d0080;
pub const F0900_P2_FROZENVIT: c_uint = 0xf33d0040;
pub const F0900_P2_SNVIT: c_uint = 0xf33d0030;
pub const F0900_P2_TOVVIT: c_uint = 0xf33d000c;
pub const F0900_P2_HYPVIT: c_uint = 0xf33d0003;
// P2_VSTATUSVIT
pub const R0900_P2_VSTATUSVIT: c_uint = 0xf33e;
pub const F0900_P2_PRFVIT: c_uint = 0xf33e0010;
pub const F0900_P2_LOCKEDVIT: c_uint = 0xf33e0008;
// P2_VTHINUSE
pub const R0900_P2_VTHINUSE: c_uint = 0xf33f;
pub const F0900_P2_VIT_INUSE: c_uint = 0xf33f00ff;
// P2_KDIV12
pub const R0900_P2_KDIV12: c_uint = 0xf340;
pub const F0900_P2_K_DIVIDER_12: c_uint = 0xf340007f;
// P2_KDIV23
pub const R0900_P2_KDIV23: c_uint = 0xf341;
pub const F0900_P2_K_DIVIDER_23: c_uint = 0xf341007f;
// P2_KDIV34
pub const R0900_P2_KDIV34: c_uint = 0xf342;
pub const F0900_P2_K_DIVIDER_34: c_uint = 0xf342007f;
// P2_KDIV56
pub const R0900_P2_KDIV56: c_uint = 0xf343;
pub const F0900_P2_K_DIVIDER_56: c_uint = 0xf343007f;
// P2_KDIV67
pub const R0900_P2_KDIV67: c_uint = 0xf344;
pub const F0900_P2_K_DIVIDER_67: c_uint = 0xf344007f;
// P2_KDIV78
pub const R0900_P2_KDIV78: c_uint = 0xf345;
pub const F0900_P2_K_DIVIDER_78: c_uint = 0xf345007f;
// P2_PDELCTRL1
pub const R0900_P2_PDELCTRL1: c_uint = 0xf350;
pub const F0900_P2_INV_MISMASK: c_uint = 0xf3500080;
pub const F0900_P2_FILTER_EN: c_uint = 0xf3500020;
pub const F0900_P2_EN_MIS00: c_uint = 0xf3500002;
pub const F0900_P2_ALGOSWRST: c_uint = 0xf3500001;
// P2_PDELCTRL2
pub const R0900_P2_PDELCTRL2: c_uint = 0xf351;
pub const F0900_P2_RESET_UPKO_COUNT: c_uint = 0xf3510040;
pub const F0900_P2_FRAME_MODE: c_uint = 0xf3510002;
pub const F0900_P2_NOBCHERRFLG_USE: c_uint = 0xf3510001;
// P2_HYSTTHRESH
pub const R0900_P2_HYSTTHRESH: c_uint = 0xf354;
pub const F0900_P2_UNLCK_THRESH: c_uint = 0xf35400f0;
pub const F0900_P2_DELIN_LCK_THRESH: c_uint = 0xf354000f;
// P2_ISIENTRY
pub const R0900_P2_ISIENTRY: c_uint = 0xf35e;
pub const F0900_P2_ISI_ENTRY: c_uint = 0xf35e00ff;
// P2_ISIBITENA
pub const R0900_P2_ISIBITENA: c_uint = 0xf35f;
pub const F0900_P2_ISI_BIT_EN: c_uint = 0xf35f00ff;
// P2_MATSTR1
pub const R0900_P2_MATSTR1: c_uint = 0xf360;
pub const F0900_P2_MATYPE_CURRENT1: c_uint = 0xf36000ff;
// P2_MATSTR0
pub const R0900_P2_MATSTR0: c_uint = 0xf361;
pub const F0900_P2_MATYPE_CURRENT0: c_uint = 0xf36100ff;
// P2_UPLSTR1
pub const R0900_P2_UPLSTR1: c_uint = 0xf362;
pub const F0900_P2_UPL_CURRENT1: c_uint = 0xf36200ff;
// P2_UPLSTR0
pub const R0900_P2_UPLSTR0: c_uint = 0xf363;
pub const F0900_P2_UPL_CURRENT0: c_uint = 0xf36300ff;
// P2_DFLSTR1
pub const R0900_P2_DFLSTR1: c_uint = 0xf364;
pub const F0900_P2_DFL_CURRENT1: c_uint = 0xf36400ff;
// P2_DFLSTR0
pub const R0900_P2_DFLSTR0: c_uint = 0xf365;
pub const F0900_P2_DFL_CURRENT0: c_uint = 0xf36500ff;
// P2_SYNCSTR
pub const R0900_P2_SYNCSTR: c_uint = 0xf366;
pub const F0900_P2_SYNC_CURRENT: c_uint = 0xf36600ff;
// P2_SYNCDSTR1
pub const R0900_P2_SYNCDSTR1: c_uint = 0xf367;
pub const F0900_P2_SYNCD_CURRENT1: c_uint = 0xf36700ff;
// P2_SYNCDSTR0
pub const R0900_P2_SYNCDSTR0: c_uint = 0xf368;
pub const F0900_P2_SYNCD_CURRENT0: c_uint = 0xf36800ff;
// P2_PDELSTATUS1
pub const R0900_P2_PDELSTATUS1: c_uint = 0xf369;
pub const F0900_P2_PKTDELIN_DELOCK: c_uint = 0xf3690080;
pub const F0900_P2_SYNCDUPDFL_BADDFL: c_uint = 0xf3690040;
pub const F0900_P2_CONTINUOUS_STREAM: c_uint = 0xf3690020;
pub const F0900_P2_UNACCEPTED_STREAM: c_uint = 0xf3690010;
pub const F0900_P2_BCH_ERROR_FLAG: c_uint = 0xf3690008;
pub const F0900_P2_PKTDELIN_LOCK: c_uint = 0xf3690002;
pub const F0900_P2_FIRST_LOCK: c_uint = 0xf3690001;
// P2_PDELSTATUS2
pub const R0900_P2_PDELSTATUS2: c_uint = 0xf36a;
pub const F0900_P2_FRAME_MODCOD: c_uint = 0xf36a007c;
pub const F0900_P2_FRAME_TYPE: c_uint = 0xf36a0003;
// P2_BBFCRCKO1
pub const R0900_P2_BBFCRCKO1: c_uint = 0xf36b;
pub const F0900_P2_BBHCRC_KOCNT1: c_uint = 0xf36b00ff;
// P2_BBFCRCKO0
pub const R0900_P2_BBFCRCKO0: c_uint = 0xf36c;
pub const F0900_P2_BBHCRC_KOCNT0: c_uint = 0xf36c00ff;
// P2_UPCRCKO1
pub const R0900_P2_UPCRCKO1: c_uint = 0xf36d;
pub const F0900_P2_PKTCRC_KOCNT1: c_uint = 0xf36d00ff;
// P2_UPCRCKO0
pub const R0900_P2_UPCRCKO0: c_uint = 0xf36e;
pub const F0900_P2_PKTCRC_KOCNT0: c_uint = 0xf36e00ff;
// P2_PDELCTRL3
pub const R0900_P2_PDELCTRL3: c_uint = 0xf36f;
pub const F0900_P2_PKTDEL_CONTFAIL: c_uint = 0xf36f0080;
pub const F0900_P2_NOFIFO_BCHERR: c_uint = 0xf36f0020;
// P2_TSSTATEM
pub const R0900_P2_TSSTATEM: c_uint = 0xf370;
pub const F0900_P2_TSDIL_ON: c_uint = 0xf3700080;
pub const F0900_P2_TSRS_ON: c_uint = 0xf3700020;
pub const F0900_P2_TSDESCRAMB_ON: c_uint = 0xf3700010;
pub const F0900_P2_TSFRAME_MODE: c_uint = 0xf3700008;
pub const F0900_P2_TS_DISABLE: c_uint = 0xf3700004;
pub const F0900_P2_TSOUT_NOSYNC: c_uint = 0xf3700001;
// P2_TSCFGH
pub const R0900_P2_TSCFGH: c_uint = 0xf372;
pub const F0900_P2_TSFIFO_DVBCI: c_uint = 0xf3720080;
pub const F0900_P2_TSFIFO_SERIAL: c_uint = 0xf3720040;
pub const F0900_P2_TSFIFO_TEIUPDATE: c_uint = 0xf3720020;
pub const F0900_P2_TSFIFO_DUTY50: c_uint = 0xf3720010;
pub const F0900_P2_TSFIFO_HSGNLOUT: c_uint = 0xf3720008;
pub const F0900_P2_TSFIFO_ERRMODE: c_uint = 0xf3720006;
pub const F0900_P2_RST_HWARE: c_uint = 0xf3720001;
// P2_TSCFGM
pub const R0900_P2_TSCFGM: c_uint = 0xf373;
pub const F0900_P2_TSFIFO_MANSPEED: c_uint = 0xf37300c0;
pub const F0900_P2_TSFIFO_PERMDATA: c_uint = 0xf3730020;
pub const F0900_P2_TSFIFO_DPUNACT: c_uint = 0xf3730002;
pub const F0900_P2_TSFIFO_INVDATA: c_uint = 0xf3730001;
// P2_TSCFGL
pub const R0900_P2_TSCFGL: c_uint = 0xf374;
pub const F0900_P2_TSFIFO_BCLKDEL1CK: c_uint = 0xf37400c0;
pub const F0900_P2_BCHERROR_MODE: c_uint = 0xf3740030;
pub const F0900_P2_TSFIFO_NSGNL2DATA: c_uint = 0xf3740008;
pub const F0900_P2_TSFIFO_EMBINDVB: c_uint = 0xf3740004;
pub const F0900_P2_TSFIFO_BITSPEED: c_uint = 0xf3740003;
// P2_TSINSDELH
pub const R0900_P2_TSINSDELH: c_uint = 0xf376;
pub const F0900_P2_TSDEL_SYNCBYTE: c_uint = 0xf3760080;
pub const F0900_P2_TSDEL_XXHEADER: c_uint = 0xf3760040;
pub const F0900_P2_TSDEL_BBHEADER: c_uint = 0xf3760020;
pub const F0900_P2_TSDEL_DATAFIELD: c_uint = 0xf3760010;
pub const F0900_P2_TSINSDEL_ISCR: c_uint = 0xf3760008;
pub const F0900_P2_TSINSDEL_NPD: c_uint = 0xf3760004;
pub const F0900_P2_TSINSDEL_RSPARITY: c_uint = 0xf3760002;
pub const F0900_P2_TSINSDEL_CRC8: c_uint = 0xf3760001;
// P2_TSDIVN
pub const R0900_P2_TSDIVN: c_uint = 0xf379;
pub const F0900_P2_TSFIFO_SPEEDMODE: c_uint = 0xf37900c0;
// P2_TSCFG4
pub const R0900_P2_TSCFG4: c_uint = 0xf37a;
pub const F0900_P2_TSFIFO_TSSPEEDMODE: c_uint = 0xf37a00c0;
// P2_TSSPEED
pub const R0900_P2_TSSPEED: c_uint = 0xf380;
pub const F0900_P2_TSFIFO_OUTSPEED: c_uint = 0xf38000ff;
// P2_TSSTATUS
pub const R0900_P2_TSSTATUS: c_uint = 0xf381;
pub const F0900_P2_TSFIFO_LINEOK: c_uint = 0xf3810080;
pub const F0900_P2_TSFIFO_ERROR: c_uint = 0xf3810040;
pub const F0900_P2_DIL_READY: c_uint = 0xf3810001;
// P2_TSSTATUS2
pub const R0900_P2_TSSTATUS2: c_uint = 0xf382;
pub const F0900_P2_TSFIFO_DEMODSEL: c_uint = 0xf3820080;
pub const F0900_P2_TSFIFOSPEED_STORE: c_uint = 0xf3820040;
pub const F0900_P2_DILXX_RESET: c_uint = 0xf3820020;
pub const F0900_P2_TSSERIAL_IMPOS: c_uint = 0xf3820010;
pub const F0900_P2_SCRAMBDETECT: c_uint = 0xf3820002;
// P2_TSBITRATE1
pub const R0900_P2_TSBITRATE1: c_uint = 0xf383;
pub const F0900_P2_TSFIFO_BITRATE1: c_uint = 0xf38300ff;
// P2_TSBITRATE0
pub const R0900_P2_TSBITRATE0: c_uint = 0xf384;
pub const F0900_P2_TSFIFO_BITRATE0: c_uint = 0xf38400ff;
// P2_ERRCTRL1
pub const R0900_P2_ERRCTRL1: c_uint = 0xf398;
pub const F0900_P2_ERR_SOURCE1: c_uint = 0xf39800f0;
pub const F0900_P2_NUM_EVENT1: c_uint = 0xf3980007;
// P2_ERRCNT12
pub const R0900_P2_ERRCNT12: c_uint = 0xf399;
pub const F0900_P2_ERRCNT1_OLDVALUE: c_uint = 0xf3990080;
pub const F0900_P2_ERR_CNT12: c_uint = 0xf399007f;
// P2_ERRCNT11
pub const R0900_P2_ERRCNT11: c_uint = 0xf39a;
pub const F0900_P2_ERR_CNT11: c_uint = 0xf39a00ff;
// P2_ERRCNT10
pub const R0900_P2_ERRCNT10: c_uint = 0xf39b;
pub const F0900_P2_ERR_CNT10: c_uint = 0xf39b00ff;
// P2_ERRCTRL2
pub const R0900_P2_ERRCTRL2: c_uint = 0xf39c;
pub const F0900_P2_ERR_SOURCE2: c_uint = 0xf39c00f0;
pub const F0900_P2_NUM_EVENT2: c_uint = 0xf39c0007;
// P2_ERRCNT22
pub const R0900_P2_ERRCNT22: c_uint = 0xf39d;
pub const F0900_P2_ERRCNT2_OLDVALUE: c_uint = 0xf39d0080;
pub const F0900_P2_ERR_CNT22: c_uint = 0xf39d007f;
// P2_ERRCNT21
pub const R0900_P2_ERRCNT21: c_uint = 0xf39e;
pub const F0900_P2_ERR_CNT21: c_uint = 0xf39e00ff;
// P2_ERRCNT20
pub const R0900_P2_ERRCNT20: c_uint = 0xf39f;
pub const F0900_P2_ERR_CNT20: c_uint = 0xf39f00ff;
// P2_FECSPY
pub const R0900_P2_FECSPY: c_uint = 0xf3a0;
pub const F0900_P2_SPY_ENABLE: c_uint = 0xf3a00080;
pub const F0900_P2_NO_SYNCBYTE: c_uint = 0xf3a00040;
pub const F0900_P2_SERIAL_MODE: c_uint = 0xf3a00020;
pub const F0900_P2_UNUSUAL_PACKET: c_uint = 0xf3a00010;
pub const F0900_P2_BERMETER_DATAMODE: c_uint = 0xf3a00008;
pub const F0900_P2_BERMETER_LMODE: c_uint = 0xf3a00002;
pub const F0900_P2_BERMETER_RESET: c_uint = 0xf3a00001;
// P2_FSPYCFG
pub const R0900_P2_FSPYCFG: c_uint = 0xf3a1;
pub const F0900_P2_FECSPY_INPUT: c_uint = 0xf3a100c0;
pub const F0900_P2_RST_ON_ERROR: c_uint = 0xf3a10020;
pub const F0900_P2_ONE_SHOT: c_uint = 0xf3a10010;
pub const F0900_P2_I2C_MODE: c_uint = 0xf3a1000c;
pub const F0900_P2_SPY_HYSTERESIS: c_uint = 0xf3a10003;
// P2_FSPYDATA
pub const R0900_P2_FSPYDATA: c_uint = 0xf3a2;
pub const F0900_P2_SPY_STUFFING: c_uint = 0xf3a20080;
pub const F0900_P2_SPY_CNULLPKT: c_uint = 0xf3a20020;
pub const F0900_P2_SPY_OUTDATA_MODE: c_uint = 0xf3a2001f;
// P2_FSPYOUT
pub const R0900_P2_FSPYOUT: c_uint = 0xf3a3;
pub const F0900_P2_FSPY_DIRECT: c_uint = 0xf3a30080;
pub const F0900_P2_STUFF_MODE: c_uint = 0xf3a30007;
// P2_FSTATUS
pub const R0900_P2_FSTATUS: c_uint = 0xf3a4;
pub const F0900_P2_SPY_ENDSIM: c_uint = 0xf3a40080;
pub const F0900_P2_VALID_SIM: c_uint = 0xf3a40040;
pub const F0900_P2_FOUND_SIGNAL: c_uint = 0xf3a40020;
pub const F0900_P2_DSS_SYNCBYTE: c_uint = 0xf3a40010;
pub const F0900_P2_RESULT_STATE: c_uint = 0xf3a4000f;
// P2_FBERCPT4
pub const R0900_P2_FBERCPT4: c_uint = 0xf3a8;
pub const F0900_P2_FBERMETER_CPT4: c_uint = 0xf3a800ff;
// P2_FBERCPT3
pub const R0900_P2_FBERCPT3: c_uint = 0xf3a9;
pub const F0900_P2_FBERMETER_CPT3: c_uint = 0xf3a900ff;
// P2_FBERCPT2
pub const R0900_P2_FBERCPT2: c_uint = 0xf3aa;
pub const F0900_P2_FBERMETER_CPT2: c_uint = 0xf3aa00ff;
// P2_FBERCPT1
pub const R0900_P2_FBERCPT1: c_uint = 0xf3ab;
pub const F0900_P2_FBERMETER_CPT1: c_uint = 0xf3ab00ff;
// P2_FBERCPT0
pub const R0900_P2_FBERCPT0: c_uint = 0xf3ac;
pub const F0900_P2_FBERMETER_CPT0: c_uint = 0xf3ac00ff;
// P2_FBERERR2
pub const R0900_P2_FBERERR2: c_uint = 0xf3ad;
pub const F0900_P2_FBERMETER_ERR2: c_uint = 0xf3ad00ff;
// P2_FBERERR1
pub const R0900_P2_FBERERR1: c_uint = 0xf3ae;
pub const F0900_P2_FBERMETER_ERR1: c_uint = 0xf3ae00ff;
// P2_FBERERR0
pub const R0900_P2_FBERERR0: c_uint = 0xf3af;
pub const F0900_P2_FBERMETER_ERR0: c_uint = 0xf3af00ff;
// P2_FSPYBER
pub const R0900_P2_FSPYBER: c_uint = 0xf3b2;
pub const F0900_P2_FSPYBER_SYNCBYTE: c_uint = 0xf3b20010;
pub const F0900_P2_FSPYBER_UNSYNC: c_uint = 0xf3b20008;
pub const F0900_P2_FSPYBER_CTIME: c_uint = 0xf3b20007;
// P1_IQCONST
pub const R0900_P1_IQCONST: c_uint = 0xf400;

pub const F0900_P1_CONSTEL_SELECT: c_uint = 0xf4000060;
pub const F0900_P1_IQSYMB_SEL: c_uint = 0xf400001f;
// P1_NOSCFG
pub const R0900_P1_NOSCFG: c_uint = 0xf401;

pub const F0900_P1_DUMMYPL_NOSDATA: c_uint = 0xf4010020;
pub const F0900_P1_NOSPLH_BETA: c_uint = 0xf4010018;
pub const F0900_P1_NOSDATA_BETA: c_uint = 0xf4010007;
// P1_ISYMB
pub const R0900_P1_ISYMB: c_uint = 0xf402;

pub const F0900_P1_I_SYMBOL: c_uint = 0xf40201ff;
// P1_QSYMB
pub const R0900_P1_QSYMB: c_uint = 0xf403;

pub const F0900_P1_Q_SYMBOL: c_uint = 0xf40301ff;
// P1_AGC1CFG
pub const R0900_P1_AGC1CFG: c_uint = 0xf404;

pub const F0900_P1_DC_FROZEN: c_uint = 0xf4040080;
pub const F0900_P1_DC_CORRECT: c_uint = 0xf4040040;
pub const F0900_P1_AMM_FROZEN: c_uint = 0xf4040020;
pub const F0900_P1_AMM_CORRECT: c_uint = 0xf4040010;
pub const F0900_P1_QUAD_FROZEN: c_uint = 0xf4040008;
pub const F0900_P1_QUAD_CORRECT: c_uint = 0xf4040004;
// P1_AGC1CN
pub const R0900_P1_AGC1CN: c_uint = 0xf406;

pub const F0900_P1_AGC1_LOCKED: c_uint = 0xf4060080;
pub const F0900_P1_AGC1_MINPOWER: c_uint = 0xf4060010;
pub const F0900_P1_AGCOUT_FAST: c_uint = 0xf4060008;
pub const F0900_P1_AGCIQ_BETA: c_uint = 0xf4060007;
// P1_AGC1REF
pub const R0900_P1_AGC1REF: c_uint = 0xf407;

pub const F0900_P1_AGCIQ_REF: c_uint = 0xf40700ff;
// P1_IDCCOMP
pub const R0900_P1_IDCCOMP: c_uint = 0xf408;

pub const F0900_P1_IAVERAGE_ADJ: c_uint = 0xf40801ff;
// P1_QDCCOMP
pub const R0900_P1_QDCCOMP: c_uint = 0xf409;

pub const F0900_P1_QAVERAGE_ADJ: c_uint = 0xf40901ff;
// P1_POWERI
pub const R0900_P1_POWERI: c_uint = 0xf40a;

pub const F0900_P1_POWER_I: c_uint = 0xf40a00ff;

// P1_POWERQ
pub const R0900_P1_POWERQ: c_uint = 0xf40b;

pub const F0900_P1_POWER_Q: c_uint = 0xf40b00ff;

// P1_AGC1AMM
pub const R0900_P1_AGC1AMM: c_uint = 0xf40c;

pub const F0900_P1_AMM_VALUE: c_uint = 0xf40c00ff;
// P1_AGC1QUAD
pub const R0900_P1_AGC1QUAD: c_uint = 0xf40d;

pub const F0900_P1_QUAD_VALUE: c_uint = 0xf40d01ff;
// P1_AGCIQIN1
pub const R0900_P1_AGCIQIN1: c_uint = 0xf40e;

pub const F0900_P1_AGCIQ_VALUE1: c_uint = 0xf40e00ff;

// P1_AGCIQIN0
pub const R0900_P1_AGCIQIN0: c_uint = 0xf40f;

pub const F0900_P1_AGCIQ_VALUE0: c_uint = 0xf40f00ff;

// P1_DEMOD
pub const R0900_P1_DEMOD: c_uint = 0xf410;

pub const F0900_P1_MANUALS2_ROLLOFF: c_uint = 0xf4100080;

pub const F0900_P1_SPECINV_CONTROL: c_uint = 0xf4100030;

pub const F0900_P1_FORCE_ENASAMP: c_uint = 0xf4100008;
pub const F0900_P1_MANUALSX_ROLLOFF: c_uint = 0xf4100004;

pub const F0900_P1_ROLLOFF_CONTROL: c_uint = 0xf4100003;

// P1_DMDMODCOD
pub const R0900_P1_DMDMODCOD: c_uint = 0xf411;

pub const F0900_P1_MANUAL_MODCOD: c_uint = 0xf4110080;
pub const F0900_P1_DEMOD_MODCOD: c_uint = 0xf411007c;

pub const F0900_P1_DEMOD_TYPE: c_uint = 0xf4110003;

// P1_DSTATUS
pub const R0900_P1_DSTATUS: c_uint = 0xf412;

pub const F0900_P1_CAR_LOCK: c_uint = 0xf4120080;
pub const F0900_P1_TMGLOCK_QUALITY: c_uint = 0xf4120060;

pub const F0900_P1_LOCK_DEFINITIF: c_uint = 0xf4120008;

pub const F0900_P1_OVADC_DETECT: c_uint = 0xf4120001;
// P1_DSTATUS2
pub const R0900_P1_DSTATUS2: c_uint = 0xf413;

pub const F0900_P1_DEMOD_DELOCK: c_uint = 0xf4130080;
pub const F0900_P1_AGC1_NOSIGNALACK: c_uint = 0xf4130008;
pub const F0900_P1_AGC2_OVERFLOW: c_uint = 0xf4130004;
pub const F0900_P1_CFR_OVERFLOW: c_uint = 0xf4130002;
pub const F0900_P1_GAMMA_OVERUNDER: c_uint = 0xf4130001;
// P1_DMDCFGMD
pub const R0900_P1_DMDCFGMD: c_uint = 0xf414;

pub const F0900_P1_DVBS2_ENABLE: c_uint = 0xf4140080;

pub const F0900_P1_DVBS1_ENABLE: c_uint = 0xf4140040;

pub const F0900_P1_SCAN_ENABLE: c_uint = 0xf4140010;

pub const F0900_P1_CFR_AUTOSCAN: c_uint = 0xf4140008;

pub const F0900_P1_TUN_RNG: c_uint = 0xf4140003;
// P1_DMDCFG2
pub const R0900_P1_DMDCFG2: c_uint = 0xf415;

pub const F0900_P1_S1S2_SEQUENTIAL: c_uint = 0xf4150040;

pub const F0900_P1_INFINITE_RELOCK: c_uint = 0xf4150010;
// P1_DMDISTATE
pub const R0900_P1_DMDISTATE: c_uint = 0xf416;

pub const F0900_P1_I2C_DEMOD_MODE: c_uint = 0xf416001f;

// P1_DMDT0M
pub const R0900_P1_DMDT0M: c_uint = 0xf417;

pub const F0900_P1_DMDT0_MIN: c_uint = 0xf41700ff;
// P1_DMDSTATE
pub const R0900_P1_DMDSTATE: c_uint = 0xf41b;

pub const F0900_P1_HEADER_MODE: c_uint = 0xf41b0060;

// P1_DMDFLYW
pub const R0900_P1_DMDFLYW: c_uint = 0xf41c;

pub const F0900_P1_I2C_IRQVAL: c_uint = 0xf41c00f0;
pub const F0900_P1_FLYWHEEL_CPT: c_uint = 0xf41c000f;

// P1_DSTATUS3
pub const R0900_P1_DSTATUS3: c_uint = 0xf41d;

pub const F0900_P1_DEMOD_CFGMODE: c_uint = 0xf41d0060;
// P1_DMDCFG3
pub const R0900_P1_DMDCFG3: c_uint = 0xf41e;

pub const F0900_P1_NOSTOP_FIFOFULL: c_uint = 0xf41e0008;
// P1_DMDCFG4
pub const R0900_P1_DMDCFG4: c_uint = 0xf41f;

pub const F0900_P1_TUNER_NRELAUNCH: c_uint = 0xf41f0008;
// P1_CORRELMANT
pub const R0900_P1_CORRELMANT: c_uint = 0xf420;

pub const F0900_P1_CORREL_MANT: c_uint = 0xf42000ff;
// P1_CORRELABS
pub const R0900_P1_CORRELABS: c_uint = 0xf421;

pub const F0900_P1_CORREL_ABS: c_uint = 0xf42100ff;
// P1_CORRELEXP
pub const R0900_P1_CORRELEXP: c_uint = 0xf422;

pub const F0900_P1_CORREL_ABSEXP: c_uint = 0xf42200f0;
pub const F0900_P1_CORREL_EXP: c_uint = 0xf422000f;
// P1_PLHMODCOD
pub const R0900_P1_PLHMODCOD: c_uint = 0xf424;

pub const F0900_P1_SPECINV_DEMOD: c_uint = 0xf4240080;

pub const F0900_P1_PLH_MODCOD: c_uint = 0xf424007c;
pub const F0900_P1_PLH_TYPE: c_uint = 0xf4240003;
// P1_DMDREG
pub const R0900_P1_DMDREG: c_uint = 0xf425;

pub const F0900_P1_DECIM_PLFRAMES: c_uint = 0xf4250001;
// P1_AGC2O
pub const R0900_P1_AGC2O: c_uint = 0xf42c;

pub const F0900_P1_AGC2_COEF: c_uint = 0xf42c0007;
// P1_AGC2REF
pub const R0900_P1_AGC2REF: c_uint = 0xf42d;

pub const F0900_P1_AGC2_REF: c_uint = 0xf42d00ff;
// P1_AGC1ADJ
pub const R0900_P1_AGC1ADJ: c_uint = 0xf42e;

pub const F0900_P1_AGC1_ADJUSTED: c_uint = 0xf42e007f;
// P1_AGC2I1
pub const R0900_P1_AGC2I1: c_uint = 0xf436;

pub const F0900_P1_AGC2_INTEGRATOR1: c_uint = 0xf43600ff;
// P1_AGC2I0
pub const R0900_P1_AGC2I0: c_uint = 0xf437;

pub const F0900_P1_AGC2_INTEGRATOR0: c_uint = 0xf43700ff;
// P1_CARCFG
pub const R0900_P1_CARCFG: c_uint = 0xf438;

pub const F0900_P1_CFRUPLOW_AUTO: c_uint = 0xf4380080;
pub const F0900_P1_CFRUPLOW_TEST: c_uint = 0xf4380040;
pub const F0900_P1_ROTAON: c_uint = 0xf4380004;
pub const F0900_P1_PH_DET_ALGO: c_uint = 0xf4380003;
// P1_ACLC
pub const R0900_P1_ACLC: c_uint = 0xf439;

pub const F0900_P1_CAR_ALPHA_MANT: c_uint = 0xf4390030;
pub const F0900_P1_CAR_ALPHA_EXP: c_uint = 0xf439000f;
// P1_BCLC
pub const R0900_P1_BCLC: c_uint = 0xf43a;

pub const F0900_P1_CAR_BETA_MANT: c_uint = 0xf43a0030;
pub const F0900_P1_CAR_BETA_EXP: c_uint = 0xf43a000f;
// P1_CARFREQ
pub const R0900_P1_CARFREQ: c_uint = 0xf43d;

pub const F0900_P1_KC_COARSE_EXP: c_uint = 0xf43d00f0;
pub const F0900_P1_BETA_FREQ: c_uint = 0xf43d000f;
// P1_CARHDR
pub const R0900_P1_CARHDR: c_uint = 0xf43e;

pub const F0900_P1_K_FREQ_HDR: c_uint = 0xf43e00ff;
// P1_LDT
pub const R0900_P1_LDT: c_uint = 0xf43f;

pub const F0900_P1_CARLOCK_THRES: c_uint = 0xf43f01ff;
// P1_LDT2
pub const R0900_P1_LDT2: c_uint = 0xf440;

pub const F0900_P1_CARLOCK_THRES2: c_uint = 0xf44001ff;
// P1_CFRICFG
pub const R0900_P1_CFRICFG: c_uint = 0xf441;

pub const F0900_P1_NEG_CFRSTEP: c_uint = 0xf4410001;
// P1_CFRUP1
pub const R0900_P1_CFRUP1: c_uint = 0xf442;

pub const F0900_P1_CFR_UP1: c_uint = 0xf44201ff;

// P1_CFRUP0
pub const R0900_P1_CFRUP0: c_uint = 0xf443;

pub const F0900_P1_CFR_UP0: c_uint = 0xf44300ff;

// P1_CFRLOW1
pub const R0900_P1_CFRLOW1: c_uint = 0xf446;

pub const F0900_P1_CFR_LOW1: c_uint = 0xf44601ff;

// P1_CFRLOW0
pub const R0900_P1_CFRLOW0: c_uint = 0xf447;

pub const F0900_P1_CFR_LOW0: c_uint = 0xf44700ff;

// P1_CFRINIT1
pub const R0900_P1_CFRINIT1: c_uint = 0xf448;

pub const F0900_P1_CFR_INIT1: c_uint = 0xf44801ff;

// P1_CFRINIT0
pub const R0900_P1_CFRINIT0: c_uint = 0xf449;

pub const F0900_P1_CFR_INIT0: c_uint = 0xf44900ff;

// P1_CFRINC1
pub const R0900_P1_CFRINC1: c_uint = 0xf44a;

pub const F0900_P1_MANUAL_CFRINC: c_uint = 0xf44a0080;
pub const F0900_P1_CFR_INC1: c_uint = 0xf44a003f;
// P1_CFRINC0
pub const R0900_P1_CFRINC0: c_uint = 0xf44b;

pub const F0900_P1_CFR_INC0: c_uint = 0xf44b00f8;
// P1_CFR2
pub const R0900_P1_CFR2: c_uint = 0xf44c;

pub const F0900_P1_CAR_FREQ2: c_uint = 0xf44c01ff;

// P1_CFR1
pub const R0900_P1_CFR1: c_uint = 0xf44d;

pub const F0900_P1_CAR_FREQ1: c_uint = 0xf44d00ff;

// P1_CFR0
pub const R0900_P1_CFR0: c_uint = 0xf44e;

pub const F0900_P1_CAR_FREQ0: c_uint = 0xf44e00ff;

// P1_LDI
pub const R0900_P1_LDI: c_uint = 0xf44f;

pub const F0900_P1_LOCK_DET_INTEGR: c_uint = 0xf44f01ff;
// P1_TMGCFG
pub const R0900_P1_TMGCFG: c_uint = 0xf450;

pub const F0900_P1_TMGLOCK_BETA: c_uint = 0xf45000c0;
pub const F0900_P1_DO_TIMING_CORR: c_uint = 0xf4500010;
pub const F0900_P1_TMG_MINFREQ: c_uint = 0xf4500003;
// P1_RTC
pub const R0900_P1_RTC: c_uint = 0xf451;

pub const F0900_P1_TMGALPHA_EXP: c_uint = 0xf45100f0;
pub const F0900_P1_TMGBETA_EXP: c_uint = 0xf451000f;
// P1_RTCS2
pub const R0900_P1_RTCS2: c_uint = 0xf452;

pub const F0900_P1_TMGALPHAS2_EXP: c_uint = 0xf45200f0;
pub const F0900_P1_TMGBETAS2_EXP: c_uint = 0xf452000f;
// P1_TMGTHRISE
pub const R0900_P1_TMGTHRISE: c_uint = 0xf453;

pub const F0900_P1_TMGLOCK_THRISE: c_uint = 0xf45300ff;
// P1_TMGTHFALL
pub const R0900_P1_TMGTHFALL: c_uint = 0xf454;

pub const F0900_P1_TMGLOCK_THFALL: c_uint = 0xf45400ff;
// P1_SFRUPRATIO
pub const R0900_P1_SFRUPRATIO: c_uint = 0xf455;

pub const F0900_P1_SFR_UPRATIO: c_uint = 0xf45500ff;
// P1_SFRLOWRATIO
pub const R0900_P1_SFRLOWRATIO: c_uint = 0xf456;
pub const F0900_P1_SFR_LOWRATIO: c_uint = 0xf45600ff;
// P1_KREFTMG
pub const R0900_P1_KREFTMG: c_uint = 0xf458;

pub const F0900_P1_KREF_TMG: c_uint = 0xf45800ff;
// P1_SFRSTEP
pub const R0900_P1_SFRSTEP: c_uint = 0xf459;

pub const F0900_P1_SFR_SCANSTEP: c_uint = 0xf45900f0;
pub const F0900_P1_SFR_CENTERSTEP: c_uint = 0xf459000f;
// P1_TMGCFG2
pub const R0900_P1_TMGCFG2: c_uint = 0xf45a;

pub const F0900_P1_SFRRATIO_FINE: c_uint = 0xf45a0001;
// P1_KREFTMG2
pub const R0900_P1_KREFTMG2: c_uint = 0xf45b;

pub const F0900_P1_KREF_TMG2: c_uint = 0xf45b00ff;
// P1_SFRINIT1
pub const R0900_P1_SFRINIT1: c_uint = 0xf45e;

pub const F0900_P1_SFR_INIT1: c_uint = 0xf45e007f;
// P1_SFRINIT0
pub const R0900_P1_SFRINIT0: c_uint = 0xf45f;

pub const F0900_P1_SFR_INIT0: c_uint = 0xf45f00ff;
// P1_SFRUP1
pub const R0900_P1_SFRUP1: c_uint = 0xf460;

pub const F0900_P1_AUTO_GUP: c_uint = 0xf4600080;

pub const F0900_P1_SYMB_FREQ_UP1: c_uint = 0xf460007f;
// P1_SFRUP0
pub const R0900_P1_SFRUP0: c_uint = 0xf461;

pub const F0900_P1_SYMB_FREQ_UP0: c_uint = 0xf46100ff;
// P1_SFRLOW1
pub const R0900_P1_SFRLOW1: c_uint = 0xf462;

pub const F0900_P1_AUTO_GLOW: c_uint = 0xf4620080;

pub const F0900_P1_SYMB_FREQ_LOW1: c_uint = 0xf462007f;
// P1_SFRLOW0
pub const R0900_P1_SFRLOW0: c_uint = 0xf463;

pub const F0900_P1_SYMB_FREQ_LOW0: c_uint = 0xf46300ff;
// P1_SFR3
pub const R0900_P1_SFR3: c_uint = 0xf464;

pub const F0900_P1_SYMB_FREQ3: c_uint = 0xf46400ff;

// P1_SFR2
pub const R0900_P1_SFR2: c_uint = 0xf465;

pub const F0900_P1_SYMB_FREQ2: c_uint = 0xf46500ff;

// P1_SFR1
pub const R0900_P1_SFR1: c_uint = 0xf466;

pub const F0900_P1_SYMB_FREQ1: c_uint = 0xf46600ff;

// P1_SFR0
pub const R0900_P1_SFR0: c_uint = 0xf467;

pub const F0900_P1_SYMB_FREQ0: c_uint = 0xf46700ff;

// P1_TMGREG2
pub const R0900_P1_TMGREG2: c_uint = 0xf468;

pub const F0900_P1_TMGREG2: c_uint = 0xf46800ff;
// P1_TMGREG1
pub const R0900_P1_TMGREG1: c_uint = 0xf469;

pub const F0900_P1_TMGREG1: c_uint = 0xf46900ff;
// P1_TMGREG0
pub const R0900_P1_TMGREG0: c_uint = 0xf46a;

pub const F0900_P1_TMGREG0: c_uint = 0xf46a00ff;
// P1_TMGLOCK1
pub const R0900_P1_TMGLOCK1: c_uint = 0xf46b;

pub const F0900_P1_TMGLOCK_LEVEL1: c_uint = 0xf46b01ff;
// P1_TMGLOCK0
pub const R0900_P1_TMGLOCK0: c_uint = 0xf46c;

pub const F0900_P1_TMGLOCK_LEVEL0: c_uint = 0xf46c00ff;
// P1_TMGOBS
pub const R0900_P1_TMGOBS: c_uint = 0xf46d;

pub const F0900_P1_ROLLOFF_STATUS: c_uint = 0xf46d00c0;

// P1_EQUALCFG
pub const R0900_P1_EQUALCFG: c_uint = 0xf46f;

pub const F0900_P1_EQUAL_ON: c_uint = 0xf46f0040;
pub const F0900_P1_MU_EQUALDFE: c_uint = 0xf46f0007;
// P1_EQUAI1
pub const R0900_P1_EQUAI1: c_uint = 0xf470;

pub const F0900_P1_EQUA_ACCI1: c_uint = 0xf47001ff;
// P1_EQUAQ1
pub const R0900_P1_EQUAQ1: c_uint = 0xf471;

pub const F0900_P1_EQUA_ACCQ1: c_uint = 0xf47101ff;
// P1_EQUAI2
pub const R0900_P1_EQUAI2: c_uint = 0xf472;

pub const F0900_P1_EQUA_ACCI2: c_uint = 0xf47201ff;
// P1_EQUAQ2
pub const R0900_P1_EQUAQ2: c_uint = 0xf473;

pub const F0900_P1_EQUA_ACCQ2: c_uint = 0xf47301ff;
// P1_EQUAI3
pub const R0900_P1_EQUAI3: c_uint = 0xf474;

pub const F0900_P1_EQUA_ACCI3: c_uint = 0xf47401ff;
// P1_EQUAQ3
pub const R0900_P1_EQUAQ3: c_uint = 0xf475;

pub const F0900_P1_EQUA_ACCQ3: c_uint = 0xf47501ff;
// P1_EQUAI4
pub const R0900_P1_EQUAI4: c_uint = 0xf476;

pub const F0900_P1_EQUA_ACCI4: c_uint = 0xf47601ff;
// P1_EQUAQ4
pub const R0900_P1_EQUAQ4: c_uint = 0xf477;

pub const F0900_P1_EQUA_ACCQ4: c_uint = 0xf47701ff;
// P1_EQUAI5
pub const R0900_P1_EQUAI5: c_uint = 0xf478;

pub const F0900_P1_EQUA_ACCI5: c_uint = 0xf47801ff;
// P1_EQUAQ5
pub const R0900_P1_EQUAQ5: c_uint = 0xf479;

pub const F0900_P1_EQUA_ACCQ5: c_uint = 0xf47901ff;
// P1_EQUAI6
pub const R0900_P1_EQUAI6: c_uint = 0xf47a;

pub const F0900_P1_EQUA_ACCI6: c_uint = 0xf47a01ff;
// P1_EQUAQ6
pub const R0900_P1_EQUAQ6: c_uint = 0xf47b;

pub const F0900_P1_EQUA_ACCQ6: c_uint = 0xf47b01ff;
// P1_EQUAI7
pub const R0900_P1_EQUAI7: c_uint = 0xf47c;

pub const F0900_P1_EQUA_ACCI7: c_uint = 0xf47c01ff;
// P1_EQUAQ7
pub const R0900_P1_EQUAQ7: c_uint = 0xf47d;

pub const F0900_P1_EQUA_ACCQ7: c_uint = 0xf47d01ff;
// P1_EQUAI8
pub const R0900_P1_EQUAI8: c_uint = 0xf47e;

pub const F0900_P1_EQUA_ACCI8: c_uint = 0xf47e01ff;
// P1_EQUAQ8
pub const R0900_P1_EQUAQ8: c_uint = 0xf47f;

pub const F0900_P1_EQUA_ACCQ8: c_uint = 0xf47f01ff;
// P1_NNOSDATAT1
pub const R0900_P1_NNOSDATAT1: c_uint = 0xf480;

pub const F0900_P1_NOSDATAT_NORMED1: c_uint = 0xf48000ff;

// P1_NNOSDATAT0
pub const R0900_P1_NNOSDATAT0: c_uint = 0xf481;

pub const F0900_P1_NOSDATAT_NORMED0: c_uint = 0xf48100ff;

// P1_NNOSDATA1
pub const R0900_P1_NNOSDATA1: c_uint = 0xf482;

pub const F0900_P1_NOSDATA_NORMED1: c_uint = 0xf48200ff;
// P1_NNOSDATA0
pub const R0900_P1_NNOSDATA0: c_uint = 0xf483;

pub const F0900_P1_NOSDATA_NORMED0: c_uint = 0xf48300ff;
// P1_NNOSPLHT1
pub const R0900_P1_NNOSPLHT1: c_uint = 0xf484;

pub const F0900_P1_NOSPLHT_NORMED1: c_uint = 0xf48400ff;

// P1_NNOSPLHT0
pub const R0900_P1_NNOSPLHT0: c_uint = 0xf485;

pub const F0900_P1_NOSPLHT_NORMED0: c_uint = 0xf48500ff;

// P1_NNOSPLH1
pub const R0900_P1_NNOSPLH1: c_uint = 0xf486;

pub const F0900_P1_NOSPLH_NORMED1: c_uint = 0xf48600ff;
// P1_NNOSPLH0
pub const R0900_P1_NNOSPLH0: c_uint = 0xf487;

pub const F0900_P1_NOSPLH_NORMED0: c_uint = 0xf48700ff;
// P1_NOSDATAT1
pub const R0900_P1_NOSDATAT1: c_uint = 0xf488;

pub const F0900_P1_NOSDATAT_UNNORMED1: c_uint = 0xf48800ff;
// P1_NOSDATAT0
pub const R0900_P1_NOSDATAT0: c_uint = 0xf489;

pub const F0900_P1_NOSDATAT_UNNORMED0: c_uint = 0xf48900ff;
// P1_NOSDATA1
pub const R0900_P1_NOSDATA1: c_uint = 0xf48a;

pub const F0900_P1_NOSDATA_UNNORMED1: c_uint = 0xf48a00ff;
// P1_NOSDATA0
pub const R0900_P1_NOSDATA0: c_uint = 0xf48b;

pub const F0900_P1_NOSDATA_UNNORMED0: c_uint = 0xf48b00ff;
// P1_NOSPLHT1
pub const R0900_P1_NOSPLHT1: c_uint = 0xf48c;

pub const F0900_P1_NOSPLHT_UNNORMED1: c_uint = 0xf48c00ff;
// P1_NOSPLHT0
pub const R0900_P1_NOSPLHT0: c_uint = 0xf48d;

pub const F0900_P1_NOSPLHT_UNNORMED0: c_uint = 0xf48d00ff;
// P1_NOSPLH1
pub const R0900_P1_NOSPLH1: c_uint = 0xf48e;

pub const F0900_P1_NOSPLH_UNNORMED1: c_uint = 0xf48e00ff;
// P1_NOSPLH0
pub const R0900_P1_NOSPLH0: c_uint = 0xf48f;

pub const F0900_P1_NOSPLH_UNNORMED0: c_uint = 0xf48f00ff;
// P1_CAR2CFG
pub const R0900_P1_CAR2CFG: c_uint = 0xf490;

pub const F0900_P1_CARRIER3_DISABLE: c_uint = 0xf4900040;
pub const F0900_P1_ROTA2ON: c_uint = 0xf4900004;
pub const F0900_P1_PH_DET_ALGO2: c_uint = 0xf4900003;
// P1_CFR2CFR1
pub const R0900_P1_CFR2CFR1: c_uint = 0xf491;

pub const F0900_P1_CFR2TOCFR1_DVBS1: c_uint = 0xf49100c0;
pub const F0900_P1_EN_S2CAR2CENTER: c_uint = 0xf4910020;
pub const F0900_P1_DIS_BCHERRCFR2: c_uint = 0xf4910010;
pub const F0900_P1_CFR2TOCFR1_BETA: c_uint = 0xf4910007;
// P1_CFR22
pub const R0900_P1_CFR22: c_uint = 0xf493;

pub const F0900_P1_CAR2_FREQ2: c_uint = 0xf49301ff;
// P1_CFR21
pub const R0900_P1_CFR21: c_uint = 0xf494;

pub const F0900_P1_CAR2_FREQ1: c_uint = 0xf49400ff;
// P1_CFR20
pub const R0900_P1_CFR20: c_uint = 0xf495;

pub const F0900_P1_CAR2_FREQ0: c_uint = 0xf49500ff;
// P1_ACLC2S2Q
pub const R0900_P1_ACLC2S2Q: c_uint = 0xf497;

pub const F0900_P1_ENAB_SPSKSYMB: c_uint = 0xf4970080;
pub const F0900_P1_CAR2S2_Q_ALPH_M: c_uint = 0xf4970030;
pub const F0900_P1_CAR2S2_Q_ALPH_E: c_uint = 0xf497000f;
// P1_ACLC2S28
pub const R0900_P1_ACLC2S28: c_uint = 0xf498;

pub const F0900_P1_OLDI3Q_MODE: c_uint = 0xf4980080;
pub const F0900_P1_CAR2S2_8_ALPH_M: c_uint = 0xf4980030;
pub const F0900_P1_CAR2S2_8_ALPH_E: c_uint = 0xf498000f;
// P1_ACLC2S216A
pub const R0900_P1_ACLC2S216A: c_uint = 0xf499;

pub const F0900_P1_DIS_C3STOPA2: c_uint = 0xf4990080;
pub const F0900_P1_CAR2S2_16ADERAT: c_uint = 0xf4990040;
pub const F0900_P1_CAR2S2_16A_ALPH_M: c_uint = 0xf4990030;
pub const F0900_P1_CAR2S2_16A_ALPH_E: c_uint = 0xf499000f;
// P1_ACLC2S232A
pub const R0900_P1_ACLC2S232A: c_uint = 0xf49a;

pub const F0900_P1_CAR2S2_32ADERAT: c_uint = 0xf49a0040;
pub const F0900_P1_CAR2S2_32A_ALPH_M: c_uint = 0xf49a0030;
pub const F0900_P1_CAR2S2_32A_ALPH_E: c_uint = 0xf49a000f;
// P1_BCLC2S2Q
pub const R0900_P1_BCLC2S2Q: c_uint = 0xf49c;

pub const F0900_P1_CAR2S2_Q_BETA_M: c_uint = 0xf49c0030;
pub const F0900_P1_CAR2S2_Q_BETA_E: c_uint = 0xf49c000f;
// P1_BCLC2S28
pub const R0900_P1_BCLC2S28: c_uint = 0xf49d;

pub const F0900_P1_CAR2S2_8_BETA_M: c_uint = 0xf49d0030;
pub const F0900_P1_CAR2S2_8_BETA_E: c_uint = 0xf49d000f;
// P1_BCLC2S216A
pub const R0900_P1_BCLC2S216A: c_uint = 0xf49e;

// P1_BCLC2S232A
pub const R0900_P1_BCLC2S232A: c_uint = 0xf49f;

// P1_PLROOT2
pub const R0900_P1_PLROOT2: c_uint = 0xf4ac;

pub const F0900_P1_PLSCRAMB_MODE: c_uint = 0xf4ac000c;
pub const F0900_P1_PLSCRAMB_ROOT2: c_uint = 0xf4ac0003;
// P1_PLROOT1
pub const R0900_P1_PLROOT1: c_uint = 0xf4ad;

pub const F0900_P1_PLSCRAMB_ROOT1: c_uint = 0xf4ad00ff;
// P1_PLROOT0
pub const R0900_P1_PLROOT0: c_uint = 0xf4ae;

pub const F0900_P1_PLSCRAMB_ROOT0: c_uint = 0xf4ae00ff;
// P1_MODCODLST0
pub const R0900_P1_MODCODLST0: c_uint = 0xf4b0;

// P1_MODCODLST1
pub const R0900_P1_MODCODLST1: c_uint = 0xf4b1;

pub const F0900_P1_DIS_MODCOD29: c_uint = 0xf4b100f0;
pub const F0900_P1_DIS_32PSK_9_10: c_uint = 0xf4b1000f;
// P1_MODCODLST2
pub const R0900_P1_MODCODLST2: c_uint = 0xf4b2;

pub const F0900_P1_DIS_32PSK_8_9: c_uint = 0xf4b200f0;
pub const F0900_P1_DIS_32PSK_5_6: c_uint = 0xf4b2000f;
// P1_MODCODLST3
pub const R0900_P1_MODCODLST3: c_uint = 0xf4b3;

pub const F0900_P1_DIS_32PSK_4_5: c_uint = 0xf4b300f0;
pub const F0900_P1_DIS_32PSK_3_4: c_uint = 0xf4b3000f;
// P1_MODCODLST4
pub const R0900_P1_MODCODLST4: c_uint = 0xf4b4;

pub const F0900_P1_DIS_16PSK_9_10: c_uint = 0xf4b400f0;
pub const F0900_P1_DIS_16PSK_8_9: c_uint = 0xf4b4000f;
// P1_MODCODLST5
pub const R0900_P1_MODCODLST5: c_uint = 0xf4b5;

pub const F0900_P1_DIS_16PSK_5_6: c_uint = 0xf4b500f0;
pub const F0900_P1_DIS_16PSK_4_5: c_uint = 0xf4b5000f;
// P1_MODCODLST6
pub const R0900_P1_MODCODLST6: c_uint = 0xf4b6;

pub const F0900_P1_DIS_16PSK_3_4: c_uint = 0xf4b600f0;
pub const F0900_P1_DIS_16PSK_2_3: c_uint = 0xf4b6000f;
// P1_MODCODLST7
pub const R0900_P1_MODCODLST7: c_uint = 0xf4b7;

pub const F0900_P1_DIS_8P_9_10: c_uint = 0xf4b700f0;
pub const F0900_P1_DIS_8P_8_9: c_uint = 0xf4b7000f;
// P1_MODCODLST8
pub const R0900_P1_MODCODLST8: c_uint = 0xf4b8;

pub const F0900_P1_DIS_8P_5_6: c_uint = 0xf4b800f0;
pub const F0900_P1_DIS_8P_3_4: c_uint = 0xf4b8000f;
// P1_MODCODLST9
pub const R0900_P1_MODCODLST9: c_uint = 0xf4b9;

pub const F0900_P1_DIS_8P_2_3: c_uint = 0xf4b900f0;
pub const F0900_P1_DIS_8P_3_5: c_uint = 0xf4b9000f;
// P1_MODCODLSTA
pub const R0900_P1_MODCODLSTA: c_uint = 0xf4ba;

pub const F0900_P1_DIS_QP_9_10: c_uint = 0xf4ba00f0;
pub const F0900_P1_DIS_QP_8_9: c_uint = 0xf4ba000f;
// P1_MODCODLSTB
pub const R0900_P1_MODCODLSTB: c_uint = 0xf4bb;

pub const F0900_P1_DIS_QP_5_6: c_uint = 0xf4bb00f0;
pub const F0900_P1_DIS_QP_4_5: c_uint = 0xf4bb000f;
// P1_MODCODLSTC
pub const R0900_P1_MODCODLSTC: c_uint = 0xf4bc;

pub const F0900_P1_DIS_QP_3_4: c_uint = 0xf4bc00f0;
pub const F0900_P1_DIS_QP_2_3: c_uint = 0xf4bc000f;
// P1_MODCODLSTD
pub const R0900_P1_MODCODLSTD: c_uint = 0xf4bd;

pub const F0900_P1_DIS_QP_3_5: c_uint = 0xf4bd00f0;
pub const F0900_P1_DIS_QP_1_2: c_uint = 0xf4bd000f;
// P1_MODCODLSTE
pub const R0900_P1_MODCODLSTE: c_uint = 0xf4be;

pub const F0900_P1_DIS_QP_2_5: c_uint = 0xf4be00f0;
pub const F0900_P1_DIS_QP_1_3: c_uint = 0xf4be000f;
// P1_MODCODLSTF
pub const R0900_P1_MODCODLSTF: c_uint = 0xf4bf;

pub const F0900_P1_DIS_QP_1_4: c_uint = 0xf4bf00f0;
// P1_GAUSSR0
pub const R0900_P1_GAUSSR0: c_uint = 0xf4c0;

pub const F0900_P1_EN_CCIMODE: c_uint = 0xf4c00080;
pub const F0900_P1_R0_GAUSSIEN: c_uint = 0xf4c0007f;
// P1_CCIR0
pub const R0900_P1_CCIR0: c_uint = 0xf4c1;

pub const F0900_P1_CCIDETECT_PLHONLY: c_uint = 0xf4c10080;
pub const F0900_P1_R0_CCI: c_uint = 0xf4c1007f;
// P1_CCIQUANT
pub const R0900_P1_CCIQUANT: c_uint = 0xf4c2;

pub const F0900_P1_CCI_BETA: c_uint = 0xf4c200e0;
pub const F0900_P1_CCI_QUANT: c_uint = 0xf4c2001f;
// P1_CCITHRES
pub const R0900_P1_CCITHRES: c_uint = 0xf4c3;

pub const F0900_P1_CCI_THRESHOLD: c_uint = 0xf4c300ff;
// P1_CCIACC
pub const R0900_P1_CCIACC: c_uint = 0xf4c4;

pub const F0900_P1_CCI_VALUE: c_uint = 0xf4c400ff;
// P1_DMDRESCFG
pub const R0900_P1_DMDRESCFG: c_uint = 0xf4c6;

pub const F0900_P1_DMDRES_RESET: c_uint = 0xf4c60080;
pub const F0900_P1_DMDRES_STRALL: c_uint = 0xf4c60008;
pub const F0900_P1_DMDRES_NEWONLY: c_uint = 0xf4c60004;
pub const F0900_P1_DMDRES_NOSTORE: c_uint = 0xf4c60002;
// P1_DMDRESADR
pub const R0900_P1_DMDRESADR: c_uint = 0xf4c7;

pub const F0900_P1_DMDRES_VALIDCFR: c_uint = 0xf4c70040;
pub const F0900_P1_DMDRES_MEMFULL: c_uint = 0xf4c70030;
pub const F0900_P1_DMDRES_RESNBR: c_uint = 0xf4c7000f;
// P1_DMDRESDATA7
pub const R0900_P1_DMDRESDATA7: c_uint = 0xf4c8;
pub const F0900_P1_DMDRES_DATA7: c_uint = 0xf4c800ff;
// P1_DMDRESDATA6
pub const R0900_P1_DMDRESDATA6: c_uint = 0xf4c9;
pub const F0900_P1_DMDRES_DATA6: c_uint = 0xf4c900ff;
// P1_DMDRESDATA5
pub const R0900_P1_DMDRESDATA5: c_uint = 0xf4ca;
pub const F0900_P1_DMDRES_DATA5: c_uint = 0xf4ca00ff;
// P1_DMDRESDATA4
pub const R0900_P1_DMDRESDATA4: c_uint = 0xf4cb;
pub const F0900_P1_DMDRES_DATA4: c_uint = 0xf4cb00ff;
// P1_DMDRESDATA3
pub const R0900_P1_DMDRESDATA3: c_uint = 0xf4cc;
pub const F0900_P1_DMDRES_DATA3: c_uint = 0xf4cc00ff;
// P1_DMDRESDATA2
pub const R0900_P1_DMDRESDATA2: c_uint = 0xf4cd;
pub const F0900_P1_DMDRES_DATA2: c_uint = 0xf4cd00ff;
// P1_DMDRESDATA1
pub const R0900_P1_DMDRESDATA1: c_uint = 0xf4ce;
pub const F0900_P1_DMDRES_DATA1: c_uint = 0xf4ce00ff;
// P1_DMDRESDATA0
pub const R0900_P1_DMDRESDATA0: c_uint = 0xf4cf;
pub const F0900_P1_DMDRES_DATA0: c_uint = 0xf4cf00ff;
// P1_FFEI1
pub const R0900_P1_FFEI1: c_uint = 0xf4d0;

pub const F0900_P1_FFE_ACCI1: c_uint = 0xf4d001ff;
// P1_FFEQ1
pub const R0900_P1_FFEQ1: c_uint = 0xf4d1;

pub const F0900_P1_FFE_ACCQ1: c_uint = 0xf4d101ff;
// P1_FFEI2
pub const R0900_P1_FFEI2: c_uint = 0xf4d2;

pub const F0900_P1_FFE_ACCI2: c_uint = 0xf4d201ff;
// P1_FFEQ2
pub const R0900_P1_FFEQ2: c_uint = 0xf4d3;

pub const F0900_P1_FFE_ACCQ2: c_uint = 0xf4d301ff;
// P1_FFEI3
pub const R0900_P1_FFEI3: c_uint = 0xf4d4;

pub const F0900_P1_FFE_ACCI3: c_uint = 0xf4d401ff;
// P1_FFEQ3
pub const R0900_P1_FFEQ3: c_uint = 0xf4d5;

pub const F0900_P1_FFE_ACCQ3: c_uint = 0xf4d501ff;
// P1_FFEI4
pub const R0900_P1_FFEI4: c_uint = 0xf4d6;

pub const F0900_P1_FFE_ACCI4: c_uint = 0xf4d601ff;
// P1_FFEQ4
pub const R0900_P1_FFEQ4: c_uint = 0xf4d7;

pub const F0900_P1_FFE_ACCQ4: c_uint = 0xf4d701ff;
// P1_FFECFG
pub const R0900_P1_FFECFG: c_uint = 0xf4d8;

pub const F0900_P1_EQUALFFE_ON: c_uint = 0xf4d80040;
pub const F0900_P1_MU_EQUALFFE: c_uint = 0xf4d80007;
// P1_TNRCFG
pub const R0900_P1_TNRCFG: c_uint = 0xf4e0;

pub const F0900_P1_TUN_ACKFAIL: c_uint = 0xf4e00080;
pub const F0900_P1_TUN_TYPE: c_uint = 0xf4e00070;
pub const F0900_P1_TUN_SECSTOP: c_uint = 0xf4e00008;
pub const F0900_P1_TUN_VCOSRCH: c_uint = 0xf4e00004;
pub const F0900_P1_TUN_MADDRESS: c_uint = 0xf4e00003;
// P1_TNRCFG2
pub const R0900_P1_TNRCFG2: c_uint = 0xf4e1;

pub const F0900_P1_TUN_IQSWAP: c_uint = 0xf4e10080;
pub const F0900_P1_DIS_BWCALC: c_uint = 0xf4e10004;
pub const F0900_P1_SHORT_WAITSTATES: c_uint = 0xf4e10002;
// P1_TNRXTAL
pub const R0900_P1_TNRXTAL: c_uint = 0xf4e4;

pub const F0900_P1_TUN_XTALFREQ: c_uint = 0xf4e4001f;
// P1_TNRSTEPS
pub const R0900_P1_TNRSTEPS: c_uint = 0xf4e7;

pub const F0900_P1_TUNER_BW0P125: c_uint = 0xf4e70080;
pub const F0900_P1_BWINC_OFFSET: c_uint = 0xf4e70170;
pub const F0900_P1_SOFTSTEP_RNG: c_uint = 0xf4e70008;
pub const F0900_P1_TUN_BWOFFSET: c_uint = 0xf4e70007;
// P1_TNRGAIN
pub const R0900_P1_TNRGAIN: c_uint = 0xf4e8;

pub const F0900_P1_TUN_KDIVEN: c_uint = 0xf4e800c0;
pub const F0900_P1_STB6X00_OCK: c_uint = 0xf4e80030;
pub const F0900_P1_TUN_GAIN: c_uint = 0xf4e8000f;
// P1_TNRRF1
pub const R0900_P1_TNRRF1: c_uint = 0xf4e9;

pub const F0900_P1_TUN_RFFREQ2: c_uint = 0xf4e900ff;

// P1_TNRRF0
pub const R0900_P1_TNRRF0: c_uint = 0xf4ea;

pub const F0900_P1_TUN_RFFREQ1: c_uint = 0xf4ea00ff;

// P1_TNRBW
pub const R0900_P1_TNRBW: c_uint = 0xf4eb;

pub const F0900_P1_TUN_RFFREQ0: c_uint = 0xf4eb00c0;

pub const F0900_P1_TUN_BW: c_uint = 0xf4eb003f;

// P1_TNRADJ
pub const R0900_P1_TNRADJ: c_uint = 0xf4ec;

pub const F0900_P1_STB61X0_CALTIME: c_uint = 0xf4ec0040;
// P1_TNRCTL2
pub const R0900_P1_TNRCTL2: c_uint = 0xf4ed;

pub const F0900_P1_STB61X0_RCCKOFF: c_uint = 0xf4ed0080;
pub const F0900_P1_STB61X0_ICP_SDOFF: c_uint = 0xf4ed0040;
pub const F0900_P1_STB61X0_DCLOOPOFF: c_uint = 0xf4ed0020;
pub const F0900_P1_STB61X0_REFOUTSEL: c_uint = 0xf4ed0010;
pub const F0900_P1_STB61X0_CALOFF: c_uint = 0xf4ed0008;
pub const F0900_P1_STB6XX0_LPT_BEN: c_uint = 0xf4ed0004;
pub const F0900_P1_STB6XX0_RX_OSCP: c_uint = 0xf4ed0002;
pub const F0900_P1_STB6XX0_SYN: c_uint = 0xf4ed0001;
// P1_TNRCFG3
pub const R0900_P1_TNRCFG3: c_uint = 0xf4ee;

pub const F0900_P1_TUN_PLLFREQ: c_uint = 0xf4ee001c;
pub const F0900_P1_TUN_I2CFREQ_MODE: c_uint = 0xf4ee0003;
// P1_TNRLAUNCH
pub const R0900_P1_TNRLAUNCH: c_uint = 0xf4f0;

// P1_TNRLD
pub const R0900_P1_TNRLD: c_uint = 0xf4f0;

pub const F0900_P1_TUNLD_VCOING: c_uint = 0xf4f00080;
pub const F0900_P1_TUN_REG1FAIL: c_uint = 0xf4f00040;
pub const F0900_P1_TUN_REG2FAIL: c_uint = 0xf4f00020;
pub const F0900_P1_TUN_REG3FAIL: c_uint = 0xf4f00010;
pub const F0900_P1_TUN_REG4FAIL: c_uint = 0xf4f00008;
pub const F0900_P1_TUN_REG5FAIL: c_uint = 0xf4f00004;
pub const F0900_P1_TUN_BWING: c_uint = 0xf4f00002;
pub const F0900_P1_TUN_LOCKED: c_uint = 0xf4f00001;
// P1_TNROBSL
pub const R0900_P1_TNROBSL: c_uint = 0xf4f6;

pub const F0900_P1_TUN_I2CABORTED: c_uint = 0xf4f60080;
pub const F0900_P1_TUN_LPEN: c_uint = 0xf4f60040;
pub const F0900_P1_TUN_FCCK: c_uint = 0xf4f60020;
pub const F0900_P1_TUN_I2CLOCKED: c_uint = 0xf4f60010;
pub const F0900_P1_TUN_PROGDONE: c_uint = 0xf4f6000c;
pub const F0900_P1_TUN_RFRESTE1: c_uint = 0xf4f60003;

// P1_TNRRESTE
pub const R0900_P1_TNRRESTE: c_uint = 0xf4f7;

pub const F0900_P1_TUN_RFRESTE0: c_uint = 0xf4f700ff;

// P1_SMAPCOEF7
pub const R0900_P1_SMAPCOEF7: c_uint = 0xf500;

pub const F0900_P1_DIS_QSCALE: c_uint = 0xf5000080;
pub const F0900_P1_SMAPCOEF_Q_LLR12: c_uint = 0xf500017f;
// P1_SMAPCOEF6
pub const R0900_P1_SMAPCOEF6: c_uint = 0xf501;

pub const F0900_P1_ADJ_8PSKLLR1: c_uint = 0xf5010004;
pub const F0900_P1_OLD_8PSKLLR1: c_uint = 0xf5010002;
pub const F0900_P1_DIS_AB8PSK: c_uint = 0xf5010001;
// P1_SMAPCOEF5
pub const R0900_P1_SMAPCOEF5: c_uint = 0xf502;

pub const F0900_P1_DIS_8SCALE: c_uint = 0xf5020080;
pub const F0900_P1_SMAPCOEF_8P_LLR23: c_uint = 0xf502017f;
// P1_NCO2MAX1
pub const R0900_P1_NCO2MAX1: c_uint = 0xf514;

pub const F0900_P1_TETA2_MAXVABS1: c_uint = 0xf51400ff;
// P1_NCO2MAX0
pub const R0900_P1_NCO2MAX0: c_uint = 0xf515;

pub const F0900_P1_TETA2_MAXVABS0: c_uint = 0xf51500ff;
// P1_NCO2FR1
pub const R0900_P1_NCO2FR1: c_uint = 0xf516;

pub const F0900_P1_NCO2FINAL_ANGLE1: c_uint = 0xf51600ff;
// P1_NCO2FR0
pub const R0900_P1_NCO2FR0: c_uint = 0xf517;

pub const F0900_P1_NCO2FINAL_ANGLE0: c_uint = 0xf51700ff;
// P1_CFR2AVRGE1
pub const R0900_P1_CFR2AVRGE1: c_uint = 0xf518;

pub const F0900_P1_I2C_CFR2AVERAGE1: c_uint = 0xf51800ff;
// P1_CFR2AVRGE0
pub const R0900_P1_CFR2AVRGE0: c_uint = 0xf519;

pub const F0900_P1_I2C_CFR2AVERAGE0: c_uint = 0xf51900ff;
// P1_DMDPLHSTAT
pub const R0900_P1_DMDPLHSTAT: c_uint = 0xf520;

pub const F0900_P1_PLH_STATISTIC: c_uint = 0xf52000ff;
// P1_LOCKTIME3
pub const R0900_P1_LOCKTIME3: c_uint = 0xf522;

pub const F0900_P1_DEMOD_LOCKTIME3: c_uint = 0xf52200ff;
// P1_LOCKTIME2
pub const R0900_P1_LOCKTIME2: c_uint = 0xf523;

pub const F0900_P1_DEMOD_LOCKTIME2: c_uint = 0xf52300ff;
// P1_LOCKTIME1
pub const R0900_P1_LOCKTIME1: c_uint = 0xf524;

pub const F0900_P1_DEMOD_LOCKTIME1: c_uint = 0xf52400ff;
// P1_LOCKTIME0
pub const R0900_P1_LOCKTIME0: c_uint = 0xf525;

pub const F0900_P1_DEMOD_LOCKTIME0: c_uint = 0xf52500ff;
// P1_VITSCALE
pub const R0900_P1_VITSCALE: c_uint = 0xf532;

pub const F0900_P1_NVTH_NOSRANGE: c_uint = 0xf5320080;
pub const F0900_P1_VERROR_MAXMODE: c_uint = 0xf5320040;
pub const F0900_P1_NSLOWSN_LOCKED: c_uint = 0xf5320008;
pub const F0900_P1_DIS_RSFLOCK: c_uint = 0xf5320002;
// P1_FECM
pub const R0900_P1_FECM: c_uint = 0xf533;

pub const F0900_P1_DSS_DVB: c_uint = 0xf5330080;

pub const F0900_P1_DSS_SRCH: c_uint = 0xf5330010;
pub const F0900_P1_SYNCVIT: c_uint = 0xf5330002;
pub const F0900_P1_IQINV: c_uint = 0xf5330001;

// P1_VTH12
pub const R0900_P1_VTH12: c_uint = 0xf534;

pub const F0900_P1_VTH12: c_uint = 0xf53400ff;
// P1_VTH23
pub const R0900_P1_VTH23: c_uint = 0xf535;

pub const F0900_P1_VTH23: c_uint = 0xf53500ff;
// P1_VTH34
pub const R0900_P1_VTH34: c_uint = 0xf536;

pub const F0900_P1_VTH34: c_uint = 0xf53600ff;
// P1_VTH56
pub const R0900_P1_VTH56: c_uint = 0xf537;

pub const F0900_P1_VTH56: c_uint = 0xf53700ff;
// P1_VTH67
pub const R0900_P1_VTH67: c_uint = 0xf538;

pub const F0900_P1_VTH67: c_uint = 0xf53800ff;
// P1_VTH78
pub const R0900_P1_VTH78: c_uint = 0xf539;

pub const F0900_P1_VTH78: c_uint = 0xf53900ff;
// P1_VITCURPUN
pub const R0900_P1_VITCURPUN: c_uint = 0xf53a;

pub const F0900_P1_VIT_CURPUN: c_uint = 0xf53a001f;

// P1_VERROR
pub const R0900_P1_VERROR: c_uint = 0xf53b;

pub const F0900_P1_REGERR_VIT: c_uint = 0xf53b00ff;
// P1_PRVIT
pub const R0900_P1_PRVIT: c_uint = 0xf53c;

pub const F0900_P1_DIS_VTHLOCK: c_uint = 0xf53c0040;
pub const F0900_P1_E7_8VIT: c_uint = 0xf53c0020;
pub const F0900_P1_E6_7VIT: c_uint = 0xf53c0010;
pub const F0900_P1_E5_6VIT: c_uint = 0xf53c0008;
pub const F0900_P1_E3_4VIT: c_uint = 0xf53c0004;
pub const F0900_P1_E2_3VIT: c_uint = 0xf53c0002;
pub const F0900_P1_E1_2VIT: c_uint = 0xf53c0001;
// P1_VAVSRVIT
pub const R0900_P1_VAVSRVIT: c_uint = 0xf53d;

pub const F0900_P1_AMVIT: c_uint = 0xf53d0080;
pub const F0900_P1_FROZENVIT: c_uint = 0xf53d0040;
pub const F0900_P1_SNVIT: c_uint = 0xf53d0030;
pub const F0900_P1_TOVVIT: c_uint = 0xf53d000c;
pub const F0900_P1_HYPVIT: c_uint = 0xf53d0003;
// P1_VSTATUSVIT
pub const R0900_P1_VSTATUSVIT: c_uint = 0xf53e;

pub const F0900_P1_PRFVIT: c_uint = 0xf53e0010;

pub const F0900_P1_LOCKEDVIT: c_uint = 0xf53e0008;

// P1_VTHINUSE
pub const R0900_P1_VTHINUSE: c_uint = 0xf53f;

pub const F0900_P1_VIT_INUSE: c_uint = 0xf53f00ff;
// P1_KDIV12
pub const R0900_P1_KDIV12: c_uint = 0xf540;

pub const F0900_P1_K_DIVIDER_12: c_uint = 0xf540007f;
// P1_KDIV23
pub const R0900_P1_KDIV23: c_uint = 0xf541;

pub const F0900_P1_K_DIVIDER_23: c_uint = 0xf541007f;
// P1_KDIV34
pub const R0900_P1_KDIV34: c_uint = 0xf542;

pub const F0900_P1_K_DIVIDER_34: c_uint = 0xf542007f;
// P1_KDIV56
pub const R0900_P1_KDIV56: c_uint = 0xf543;

pub const F0900_P1_K_DIVIDER_56: c_uint = 0xf543007f;
// P1_KDIV67
pub const R0900_P1_KDIV67: c_uint = 0xf544;

pub const F0900_P1_K_DIVIDER_67: c_uint = 0xf544007f;
// P1_KDIV78
pub const R0900_P1_KDIV78: c_uint = 0xf545;

pub const F0900_P1_K_DIVIDER_78: c_uint = 0xf545007f;
// P1_PDELCTRL1
pub const R0900_P1_PDELCTRL1: c_uint = 0xf550;

pub const F0900_P1_INV_MISMASK: c_uint = 0xf5500080;

pub const F0900_P1_FILTER_EN: c_uint = 0xf5500020;

pub const F0900_P1_EN_MIS00: c_uint = 0xf5500002;

pub const F0900_P1_ALGOSWRST: c_uint = 0xf5500001;

// P1_PDELCTRL2
pub const R0900_P1_PDELCTRL2: c_uint = 0xf551;

pub const F0900_P1_RESET_UPKO_COUNT: c_uint = 0xf5510040;

pub const F0900_P1_FRAME_MODE: c_uint = 0xf5510002;
pub const F0900_P1_NOBCHERRFLG_USE: c_uint = 0xf5510001;
// P1_HYSTTHRESH
pub const R0900_P1_HYSTTHRESH: c_uint = 0xf554;

pub const F0900_P1_UNLCK_THRESH: c_uint = 0xf55400f0;
pub const F0900_P1_DELIN_LCK_THRESH: c_uint = 0xf554000f;
// P1_ISIENTRY
pub const R0900_P1_ISIENTRY: c_uint = 0xf55e;

pub const F0900_P1_ISI_ENTRY: c_uint = 0xf55e00ff;
// P1_ISIBITENA
pub const R0900_P1_ISIBITENA: c_uint = 0xf55f;

pub const F0900_P1_ISI_BIT_EN: c_uint = 0xf55f00ff;
// P1_MATSTR1
pub const R0900_P1_MATSTR1: c_uint = 0xf560;

pub const F0900_P1_MATYPE_CURRENT1: c_uint = 0xf56000ff;
// P1_MATSTR0
pub const R0900_P1_MATSTR0: c_uint = 0xf561;

pub const F0900_P1_MATYPE_CURRENT0: c_uint = 0xf56100ff;
// P1_UPLSTR1
pub const R0900_P1_UPLSTR1: c_uint = 0xf562;

pub const F0900_P1_UPL_CURRENT1: c_uint = 0xf56200ff;
// P1_UPLSTR0
pub const R0900_P1_UPLSTR0: c_uint = 0xf563;

pub const F0900_P1_UPL_CURRENT0: c_uint = 0xf56300ff;
// P1_DFLSTR1
pub const R0900_P1_DFLSTR1: c_uint = 0xf564;

pub const F0900_P1_DFL_CURRENT1: c_uint = 0xf56400ff;
// P1_DFLSTR0
pub const R0900_P1_DFLSTR0: c_uint = 0xf565;

pub const F0900_P1_DFL_CURRENT0: c_uint = 0xf56500ff;
// P1_SYNCSTR
pub const R0900_P1_SYNCSTR: c_uint = 0xf566;

pub const F0900_P1_SYNC_CURRENT: c_uint = 0xf56600ff;
// P1_SYNCDSTR1
pub const R0900_P1_SYNCDSTR1: c_uint = 0xf567;

pub const F0900_P1_SYNCD_CURRENT1: c_uint = 0xf56700ff;
// P1_SYNCDSTR0
pub const R0900_P1_SYNCDSTR0: c_uint = 0xf568;

pub const F0900_P1_SYNCD_CURRENT0: c_uint = 0xf56800ff;
// P1_PDELSTATUS1
pub const R0900_P1_PDELSTATUS1: c_uint = 0xf569;
pub const F0900_P1_PKTDELIN_DELOCK: c_uint = 0xf5690080;
pub const F0900_P1_SYNCDUPDFL_BADDFL: c_uint = 0xf5690040;
pub const F0900_P1_CONTINUOUS_STREAM: c_uint = 0xf5690020;
pub const F0900_P1_UNACCEPTED_STREAM: c_uint = 0xf5690010;
pub const F0900_P1_BCH_ERROR_FLAG: c_uint = 0xf5690008;
pub const F0900_P1_PKTDELIN_LOCK: c_uint = 0xf5690002;

pub const F0900_P1_FIRST_LOCK: c_uint = 0xf5690001;
// P1_PDELSTATUS2
pub const R0900_P1_PDELSTATUS2: c_uint = 0xf56a;
pub const F0900_P1_FRAME_MODCOD: c_uint = 0xf56a007c;
pub const F0900_P1_FRAME_TYPE: c_uint = 0xf56a0003;
// P1_BBFCRCKO1
pub const R0900_P1_BBFCRCKO1: c_uint = 0xf56b;

pub const F0900_P1_BBHCRC_KOCNT1: c_uint = 0xf56b00ff;
// P1_BBFCRCKO0
pub const R0900_P1_BBFCRCKO0: c_uint = 0xf56c;

pub const F0900_P1_BBHCRC_KOCNT0: c_uint = 0xf56c00ff;
// P1_UPCRCKO1
pub const R0900_P1_UPCRCKO1: c_uint = 0xf56d;

pub const F0900_P1_PKTCRC_KOCNT1: c_uint = 0xf56d00ff;
// P1_UPCRCKO0
pub const R0900_P1_UPCRCKO0: c_uint = 0xf56e;

pub const F0900_P1_PKTCRC_KOCNT0: c_uint = 0xf56e00ff;
// P1_PDELCTRL3
pub const R0900_P1_PDELCTRL3: c_uint = 0xf56f;

pub const F0900_P1_PKTDEL_CONTFAIL: c_uint = 0xf56f0080;
pub const F0900_P1_NOFIFO_BCHERR: c_uint = 0xf56f0020;
// P1_TSSTATEM
pub const R0900_P1_TSSTATEM: c_uint = 0xf570;

pub const F0900_P1_TSDIL_ON: c_uint = 0xf5700080;
pub const F0900_P1_TSRS_ON: c_uint = 0xf5700020;
pub const F0900_P1_TSDESCRAMB_ON: c_uint = 0xf5700010;
pub const F0900_P1_TSFRAME_MODE: c_uint = 0xf5700008;
pub const F0900_P1_TS_DISABLE: c_uint = 0xf5700004;
pub const F0900_P1_TSOUT_NOSYNC: c_uint = 0xf5700001;
// P1_TSCFGH
pub const R0900_P1_TSCFGH: c_uint = 0xf572;

pub const F0900_P1_TSFIFO_DVBCI: c_uint = 0xf5720080;
pub const F0900_P1_TSFIFO_SERIAL: c_uint = 0xf5720040;
pub const F0900_P1_TSFIFO_TEIUPDATE: c_uint = 0xf5720020;
pub const F0900_P1_TSFIFO_DUTY50: c_uint = 0xf5720010;
pub const F0900_P1_TSFIFO_HSGNLOUT: c_uint = 0xf5720008;
pub const F0900_P1_TSFIFO_ERRMODE: c_uint = 0xf5720006;
pub const F0900_P1_RST_HWARE: c_uint = 0xf5720001;

// P1_TSCFGM
pub const R0900_P1_TSCFGM: c_uint = 0xf573;

pub const F0900_P1_TSFIFO_MANSPEED: c_uint = 0xf57300c0;
pub const F0900_P1_TSFIFO_PERMDATA: c_uint = 0xf5730020;
pub const F0900_P1_TSFIFO_DPUNACT: c_uint = 0xf5730002;
pub const F0900_P1_TSFIFO_INVDATA: c_uint = 0xf5730001;
// P1_TSCFGL
pub const R0900_P1_TSCFGL: c_uint = 0xf574;

pub const F0900_P1_TSFIFO_BCLKDEL1CK: c_uint = 0xf57400c0;
pub const F0900_P1_BCHERROR_MODE: c_uint = 0xf5740030;
pub const F0900_P1_TSFIFO_NSGNL2DATA: c_uint = 0xf5740008;
pub const F0900_P1_TSFIFO_EMBINDVB: c_uint = 0xf5740004;
pub const F0900_P1_TSFIFO_BITSPEED: c_uint = 0xf5740003;
// P1_TSINSDELH
pub const R0900_P1_TSINSDELH: c_uint = 0xf576;

pub const F0900_P1_TSDEL_SYNCBYTE: c_uint = 0xf5760080;
pub const F0900_P1_TSDEL_XXHEADER: c_uint = 0xf5760040;
pub const F0900_P1_TSDEL_BBHEADER: c_uint = 0xf5760020;
pub const F0900_P1_TSDEL_DATAFIELD: c_uint = 0xf5760010;
pub const F0900_P1_TSINSDEL_ISCR: c_uint = 0xf5760008;
pub const F0900_P1_TSINSDEL_NPD: c_uint = 0xf5760004;
pub const F0900_P1_TSINSDEL_RSPARITY: c_uint = 0xf5760002;
pub const F0900_P1_TSINSDEL_CRC8: c_uint = 0xf5760001;
// P1_TSDIVN
pub const R0900_P1_TSDIVN: c_uint = 0xf579;

pub const F0900_P1_TSFIFO_SPEEDMODE: c_uint = 0xf57900c0;
// P1_TSCFG4
pub const R0900_P1_TSCFG4: c_uint = 0xf57a;

pub const F0900_P1_TSFIFO_TSSPEEDMODE: c_uint = 0xf57a00c0;
// P1_TSSPEED
pub const R0900_P1_TSSPEED: c_uint = 0xf580;

pub const F0900_P1_TSFIFO_OUTSPEED: c_uint = 0xf58000ff;
// P1_TSSTATUS
pub const R0900_P1_TSSTATUS: c_uint = 0xf581;

pub const F0900_P1_TSFIFO_LINEOK: c_uint = 0xf5810080;

pub const F0900_P1_TSFIFO_ERROR: c_uint = 0xf5810040;
pub const F0900_P1_DIL_READY: c_uint = 0xf5810001;
// P1_TSSTATUS2
pub const R0900_P1_TSSTATUS2: c_uint = 0xf582;

pub const F0900_P1_TSFIFO_DEMODSEL: c_uint = 0xf5820080;
pub const F0900_P1_TSFIFOSPEED_STORE: c_uint = 0xf5820040;
pub const F0900_P1_DILXX_RESET: c_uint = 0xf5820020;
pub const F0900_P1_TSSERIAL_IMPOS: c_uint = 0xf5820010;
pub const F0900_P1_SCRAMBDETECT: c_uint = 0xf5820002;
// P1_TSBITRATE1
pub const R0900_P1_TSBITRATE1: c_uint = 0xf583;

pub const F0900_P1_TSFIFO_BITRATE1: c_uint = 0xf58300ff;
// P1_TSBITRATE0
pub const R0900_P1_TSBITRATE0: c_uint = 0xf584;

pub const F0900_P1_TSFIFO_BITRATE0: c_uint = 0xf58400ff;
// P1_ERRCTRL1
pub const R0900_P1_ERRCTRL1: c_uint = 0xf598;

pub const F0900_P1_ERR_SOURCE1: c_uint = 0xf59800f0;
pub const F0900_P1_NUM_EVENT1: c_uint = 0xf5980007;
// P1_ERRCNT12
pub const R0900_P1_ERRCNT12: c_uint = 0xf599;

pub const F0900_P1_ERRCNT1_OLDVALUE: c_uint = 0xf5990080;
pub const F0900_P1_ERR_CNT12: c_uint = 0xf599007f;

// P1_ERRCNT11
pub const R0900_P1_ERRCNT11: c_uint = 0xf59a;

pub const F0900_P1_ERR_CNT11: c_uint = 0xf59a00ff;

// P1_ERRCNT10
pub const R0900_P1_ERRCNT10: c_uint = 0xf59b;

pub const F0900_P1_ERR_CNT10: c_uint = 0xf59b00ff;

// P1_ERRCTRL2
pub const R0900_P1_ERRCTRL2: c_uint = 0xf59c;

pub const F0900_P1_ERR_SOURCE2: c_uint = 0xf59c00f0;
pub const F0900_P1_NUM_EVENT2: c_uint = 0xf59c0007;
// P1_ERRCNT22
pub const R0900_P1_ERRCNT22: c_uint = 0xf59d;

pub const F0900_P1_ERRCNT2_OLDVALUE: c_uint = 0xf59d0080;
pub const F0900_P1_ERR_CNT22: c_uint = 0xf59d007f;

// P1_ERRCNT21
pub const R0900_P1_ERRCNT21: c_uint = 0xf59e;

pub const F0900_P1_ERR_CNT21: c_uint = 0xf59e00ff;

// P1_ERRCNT20
pub const R0900_P1_ERRCNT20: c_uint = 0xf59f;

pub const F0900_P1_ERR_CNT20: c_uint = 0xf59f00ff;

// P1_FECSPY
pub const R0900_P1_FECSPY: c_uint = 0xf5a0;

pub const F0900_P1_SPY_ENABLE: c_uint = 0xf5a00080;
pub const F0900_P1_NO_SYNCBYTE: c_uint = 0xf5a00040;
pub const F0900_P1_SERIAL_MODE: c_uint = 0xf5a00020;
pub const F0900_P1_UNUSUAL_PACKET: c_uint = 0xf5a00010;
pub const F0900_P1_BERMETER_DATAMODE: c_uint = 0xf5a00008;
pub const F0900_P1_BERMETER_LMODE: c_uint = 0xf5a00002;
pub const F0900_P1_BERMETER_RESET: c_uint = 0xf5a00001;
// P1_FSPYCFG
pub const R0900_P1_FSPYCFG: c_uint = 0xf5a1;

pub const F0900_P1_FECSPY_INPUT: c_uint = 0xf5a100c0;
pub const F0900_P1_RST_ON_ERROR: c_uint = 0xf5a10020;
pub const F0900_P1_ONE_SHOT: c_uint = 0xf5a10010;
pub const F0900_P1_I2C_MODE: c_uint = 0xf5a1000c;
pub const F0900_P1_SPY_HYSTERESIS: c_uint = 0xf5a10003;
// P1_FSPYDATA
pub const R0900_P1_FSPYDATA: c_uint = 0xf5a2;

pub const F0900_P1_SPY_STUFFING: c_uint = 0xf5a20080;
pub const F0900_P1_SPY_CNULLPKT: c_uint = 0xf5a20020;
pub const F0900_P1_SPY_OUTDATA_MODE: c_uint = 0xf5a2001f;
// P1_FSPYOUT
pub const R0900_P1_FSPYOUT: c_uint = 0xf5a3;

pub const F0900_P1_FSPY_DIRECT: c_uint = 0xf5a30080;
pub const F0900_P1_STUFF_MODE: c_uint = 0xf5a30007;
// P1_FSTATUS
pub const R0900_P1_FSTATUS: c_uint = 0xf5a4;

pub const F0900_P1_SPY_ENDSIM: c_uint = 0xf5a40080;
pub const F0900_P1_VALID_SIM: c_uint = 0xf5a40040;
pub const F0900_P1_FOUND_SIGNAL: c_uint = 0xf5a40020;
pub const F0900_P1_DSS_SYNCBYTE: c_uint = 0xf5a40010;
pub const F0900_P1_RESULT_STATE: c_uint = 0xf5a4000f;
// P1_FBERCPT4
pub const R0900_P1_FBERCPT4: c_uint = 0xf5a8;

pub const F0900_P1_FBERMETER_CPT4: c_uint = 0xf5a800ff;
// P1_FBERCPT3
pub const R0900_P1_FBERCPT3: c_uint = 0xf5a9;

pub const F0900_P1_FBERMETER_CPT3: c_uint = 0xf5a900ff;
// P1_FBERCPT2
pub const R0900_P1_FBERCPT2: c_uint = 0xf5aa;

pub const F0900_P1_FBERMETER_CPT2: c_uint = 0xf5aa00ff;
// P1_FBERCPT1
pub const R0900_P1_FBERCPT1: c_uint = 0xf5ab;

pub const F0900_P1_FBERMETER_CPT1: c_uint = 0xf5ab00ff;
// P1_FBERCPT0
pub const R0900_P1_FBERCPT0: c_uint = 0xf5ac;

pub const F0900_P1_FBERMETER_CPT0: c_uint = 0xf5ac00ff;
// P1_FBERERR2
pub const R0900_P1_FBERERR2: c_uint = 0xf5ad;

pub const F0900_P1_FBERMETER_ERR2: c_uint = 0xf5ad00ff;
// P1_FBERERR1
pub const R0900_P1_FBERERR1: c_uint = 0xf5ae;

pub const F0900_P1_FBERMETER_ERR1: c_uint = 0xf5ae00ff;
// P1_FBERERR0
pub const R0900_P1_FBERERR0: c_uint = 0xf5af;

pub const F0900_P1_FBERMETER_ERR0: c_uint = 0xf5af00ff;
// P1_FSPYBER
pub const R0900_P1_FSPYBER: c_uint = 0xf5b2;

pub const F0900_P1_FSPYBER_SYNCBYTE: c_uint = 0xf5b20010;
pub const F0900_P1_FSPYBER_UNSYNC: c_uint = 0xf5b20008;
pub const F0900_P1_FSPYBER_CTIME: c_uint = 0xf5b20007;
// RCCFG2
pub const R0900_RCCFG2: c_uint = 0xf600;
// TSGENERAL
pub const R0900_TSGENERAL: c_uint = 0xf630;
pub const F0900_TSFIFO_DISTS2PAR: c_uint = 0xf6300040;
pub const F0900_MUXSTREAM_OUTMODE: c_uint = 0xf6300008;
pub const F0900_TSFIFO_PERMPARAL: c_uint = 0xf6300006;
// TSGENERAL1X
pub const R0900_TSGENERAL1X: c_uint = 0xf670;
// NBITER_NF4
pub const R0900_NBITER_NF4: c_uint = 0xfa03;
pub const F0900_NBITER_NF_QP_1_2: c_uint = 0xfa0300ff;
// NBITER_NF5
pub const R0900_NBITER_NF5: c_uint = 0xfa04;
pub const F0900_NBITER_NF_QP_3_5: c_uint = 0xfa0400ff;
// NBITER_NF6
pub const R0900_NBITER_NF6: c_uint = 0xfa05;
pub const F0900_NBITER_NF_QP_2_3: c_uint = 0xfa0500ff;
// NBITER_NF7
pub const R0900_NBITER_NF7: c_uint = 0xfa06;
pub const F0900_NBITER_NF_QP_3_4: c_uint = 0xfa0600ff;
// NBITER_NF8
pub const R0900_NBITER_NF8: c_uint = 0xfa07;
pub const F0900_NBITER_NF_QP_4_5: c_uint = 0xfa0700ff;
// NBITER_NF9
pub const R0900_NBITER_NF9: c_uint = 0xfa08;
pub const F0900_NBITER_NF_QP_5_6: c_uint = 0xfa0800ff;
// NBITER_NF10
pub const R0900_NBITER_NF10: c_uint = 0xfa09;
pub const F0900_NBITER_NF_QP_8_9: c_uint = 0xfa0900ff;
// NBITER_NF11
pub const R0900_NBITER_NF11: c_uint = 0xfa0a;
pub const F0900_NBITER_NF_QP_9_10: c_uint = 0xfa0a00ff;
// NBITER_NF12
pub const R0900_NBITER_NF12: c_uint = 0xfa0b;
pub const F0900_NBITER_NF_8P_3_5: c_uint = 0xfa0b00ff;
// NBITER_NF13
pub const R0900_NBITER_NF13: c_uint = 0xfa0c;
pub const F0900_NBITER_NF_8P_2_3: c_uint = 0xfa0c00ff;
// NBITER_NF14
pub const R0900_NBITER_NF14: c_uint = 0xfa0d;
pub const F0900_NBITER_NF_8P_3_4: c_uint = 0xfa0d00ff;
// NBITER_NF15
pub const R0900_NBITER_NF15: c_uint = 0xfa0e;
pub const F0900_NBITER_NF_8P_5_6: c_uint = 0xfa0e00ff;
// NBITER_NF16
pub const R0900_NBITER_NF16: c_uint = 0xfa0f;
pub const F0900_NBITER_NF_8P_8_9: c_uint = 0xfa0f00ff;
// NBITER_NF17
pub const R0900_NBITER_NF17: c_uint = 0xfa10;
pub const F0900_NBITER_NF_8P_9_10: c_uint = 0xfa1000ff;
// NBITERNOERR
pub const R0900_NBITERNOERR: c_uint = 0xfa3f;
pub const F0900_NBITER_STOP_CRIT: c_uint = 0xfa3f000f;
// GAINLLR_NF4
pub const R0900_GAINLLR_NF4: c_uint = 0xfa43;
pub const F0900_GAINLLR_NF_QP_1_2: c_uint = 0xfa43007f;
// GAINLLR_NF5
pub const R0900_GAINLLR_NF5: c_uint = 0xfa44;
pub const F0900_GAINLLR_NF_QP_3_5: c_uint = 0xfa44007f;
// GAINLLR_NF6
pub const R0900_GAINLLR_NF6: c_uint = 0xfa45;
pub const F0900_GAINLLR_NF_QP_2_3: c_uint = 0xfa45007f;
// GAINLLR_NF7
pub const R0900_GAINLLR_NF7: c_uint = 0xfa46;
pub const F0900_GAINLLR_NF_QP_3_4: c_uint = 0xfa46007f;
// GAINLLR_NF8
pub const R0900_GAINLLR_NF8: c_uint = 0xfa47;
pub const F0900_GAINLLR_NF_QP_4_5: c_uint = 0xfa47007f;
// GAINLLR_NF9
pub const R0900_GAINLLR_NF9: c_uint = 0xfa48;
pub const F0900_GAINLLR_NF_QP_5_6: c_uint = 0xfa48007f;
// GAINLLR_NF10
pub const R0900_GAINLLR_NF10: c_uint = 0xfa49;
pub const F0900_GAINLLR_NF_QP_8_9: c_uint = 0xfa49007f;
// GAINLLR_NF11
pub const R0900_GAINLLR_NF11: c_uint = 0xfa4a;
pub const F0900_GAINLLR_NF_QP_9_10: c_uint = 0xfa4a007f;
// GAINLLR_NF12
pub const R0900_GAINLLR_NF12: c_uint = 0xfa4b;
pub const F0900_GAINLLR_NF_8P_3_5: c_uint = 0xfa4b007f;
// GAINLLR_NF13
pub const R0900_GAINLLR_NF13: c_uint = 0xfa4c;
pub const F0900_GAINLLR_NF_8P_2_3: c_uint = 0xfa4c007f;
// GAINLLR_NF14
pub const R0900_GAINLLR_NF14: c_uint = 0xfa4d;
pub const F0900_GAINLLR_NF_8P_3_4: c_uint = 0xfa4d007f;
// GAINLLR_NF15
pub const R0900_GAINLLR_NF15: c_uint = 0xfa4e;
pub const F0900_GAINLLR_NF_8P_5_6: c_uint = 0xfa4e007f;
// GAINLLR_NF16
pub const R0900_GAINLLR_NF16: c_uint = 0xfa4f;
pub const F0900_GAINLLR_NF_8P_8_9: c_uint = 0xfa4f007f;
// GAINLLR_NF17
pub const R0900_GAINLLR_NF17: c_uint = 0xfa50;
pub const F0900_GAINLLR_NF_8P_9_10: c_uint = 0xfa50007f;
// CFGEXT
pub const R0900_CFGEXT: c_uint = 0xfa80;
pub const F0900_STAGMODE: c_uint = 0xfa800080;
pub const F0900_BYPBCH: c_uint = 0xfa800040;
pub const F0900_BYPLDPC: c_uint = 0xfa800020;
pub const F0900_LDPCMODE: c_uint = 0xfa800010;
pub const F0900_INVLLRSIGN: c_uint = 0xfa800008;
pub const F0900_SHORTMULT: c_uint = 0xfa800004;
pub const F0900_EXTERNTX: c_uint = 0xfa800001;
// GENCFG
pub const R0900_GENCFG: c_uint = 0xfa86;
pub const F0900_BROADCAST: c_uint = 0xfa860010;
pub const F0900_PRIORITY: c_uint = 0xfa860002;
pub const F0900_DDEMOD: c_uint = 0xfa860001;
// LDPCERR1
pub const R0900_LDPCERR1: c_uint = 0xfa96;
pub const F0900_LDPC_ERRORS_COUNTER1: c_uint = 0xfa9600ff;
// LDPCERR0
pub const R0900_LDPCERR0: c_uint = 0xfa97;
pub const F0900_LDPC_ERRORS_COUNTER0: c_uint = 0xfa9700ff;
// BCHERR
pub const R0900_BCHERR: c_uint = 0xfa98;
pub const F0900_ERRORFLAG: c_uint = 0xfa980010;
pub const F0900_BCH_ERRORS_COUNTER: c_uint = 0xfa98000f;
// TSTRES0
pub const R0900_TSTRES0: c_uint = 0xff11;
pub const F0900_FRESFEC: c_uint = 0xff110080;
// P2_TCTL4
pub const R0900_P2_TCTL4: c_uint = 0xff28;
pub const F0900_P2_PN4_SELECT: c_uint = 0xff280020;
// P1_TCTL4
pub const R0900_P1_TCTL4: c_uint = 0xff48;

pub const F0900_P1_PN4_SELECT: c_uint = 0xff480020;
// P2_TSTDISRX
pub const R0900_P2_TSTDISRX: c_uint = 0xff65;
pub const F0900_P2_PIN_SELECT1: c_uint = 0xff650008;
// P1_TSTDISRX
pub const R0900_P1_TSTDISRX: c_uint = 0xff67;

pub const F0900_P1_PIN_SELECT1: c_uint = 0xff670008;

pub const STV0900_NBREGS: c_int = 723;
pub const STV0900_NBFIELDS: c_int = 1420;
