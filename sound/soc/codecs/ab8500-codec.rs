//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ab8500-codec.h
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
// Copyright (C) ST-Ericsson SA 2012
//
// Author: Ola Lilja <ola.o.lilja@stericsson.com>,
// Kristoffer Karlsson <kristoffer.karlsson@stericsson.com>,
// Roger Nilsson <roger.xr.nilsson@stericsson.com>,
// for ST-Ericsson.
//
// Based on the early work done by:
// Mikko J. Lehto <mikko.lehto@symbio.com>,
// Mikko Sarmanne <mikko.sarmanne@symbio.com>,
// for ST-Ericsson.
//

// AB8500 interface slot offset definitions
pub const AB8500_AD_DATA0_OFFSET: c_int = 0;
pub const AB8500_DA_DATA0_OFFSET: c_int = 8;
pub const AB8500_AD_DATA1_OFFSET: c_int = 16;
pub const AB8500_DA_DATA1_OFFSET: c_int = 24;
// AB8500 audio bank (0x0d) register definitions
pub const AB8500_POWERUP: c_uint = 0x00;
pub const AB8500_AUDSWRESET: c_uint = 0x01;
pub const AB8500_ADPATHENA: c_uint = 0x02;
pub const AB8500_DAPATHENA: c_uint = 0x03;
pub const AB8500_ANACONF1: c_uint = 0x04;
pub const AB8500_ANACONF2: c_uint = 0x05;
pub const AB8500_DIGMICCONF: c_uint = 0x06;
pub const AB8500_ANACONF3: c_uint = 0x07;
pub const AB8500_ANACONF4: c_uint = 0x08;
pub const AB8500_DAPATHCONF: c_uint = 0x09;
pub const AB8500_MUTECONF: c_uint = 0x0A;
pub const AB8500_SHORTCIRCONF: c_uint = 0x0B;
pub const AB8500_ANACONF5: c_uint = 0x0C;
pub const AB8500_ENVCPCONF: c_uint = 0x0D;
pub const AB8500_SIGENVCONF: c_uint = 0x0E;
pub const AB8500_PWMGENCONF1: c_uint = 0x0F;
pub const AB8500_PWMGENCONF2: c_uint = 0x10;
pub const AB8500_PWMGENCONF3: c_uint = 0x11;
pub const AB8500_PWMGENCONF4: c_uint = 0x12;
pub const AB8500_PWMGENCONF5: c_uint = 0x13;
pub const AB8500_ANAGAIN1: c_uint = 0x14;
pub const AB8500_ANAGAIN2: c_uint = 0x15;
pub const AB8500_ANAGAIN3: c_uint = 0x16;
pub const AB8500_ANAGAIN4: c_uint = 0x17;
pub const AB8500_DIGLINHSLGAIN: c_uint = 0x18;
pub const AB8500_DIGLINHSRGAIN: c_uint = 0x19;
pub const AB8500_ADFILTCONF: c_uint = 0x1A;
pub const AB8500_DIGIFCONF1: c_uint = 0x1B;
pub const AB8500_DIGIFCONF2: c_uint = 0x1C;
pub const AB8500_DIGIFCONF3: c_uint = 0x1D;
pub const AB8500_DIGIFCONF4: c_uint = 0x1E;
pub const AB8500_ADSLOTSEL1: c_uint = 0x1F;
pub const AB8500_ADSLOTSEL2: c_uint = 0x20;
pub const AB8500_ADSLOTSEL3: c_uint = 0x21;
pub const AB8500_ADSLOTSEL4: c_uint = 0x22;
pub const AB8500_ADSLOTSEL5: c_uint = 0x23;
pub const AB8500_ADSLOTSEL6: c_uint = 0x24;
pub const AB8500_ADSLOTSEL7: c_uint = 0x25;
pub const AB8500_ADSLOTSEL8: c_uint = 0x26;
pub const AB8500_ADSLOTSEL9: c_uint = 0x27;
pub const AB8500_ADSLOTSEL10: c_uint = 0x28;
pub const AB8500_ADSLOTSEL11: c_uint = 0x29;
pub const AB8500_ADSLOTSEL12: c_uint = 0x2A;
pub const AB8500_ADSLOTSEL13: c_uint = 0x2B;
pub const AB8500_ADSLOTSEL14: c_uint = 0x2C;
pub const AB8500_ADSLOTSEL15: c_uint = 0x2D;
pub const AB8500_ADSLOTSEL16: c_uint = 0x2E;

pub const AB8500_ADSLOTHIZCTRL1: c_uint = 0x2F;
pub const AB8500_ADSLOTHIZCTRL2: c_uint = 0x30;
pub const AB8500_ADSLOTHIZCTRL3: c_uint = 0x31;
pub const AB8500_ADSLOTHIZCTRL4: c_uint = 0x32;
pub const AB8500_DASLOTCONF1: c_uint = 0x33;
pub const AB8500_DASLOTCONF2: c_uint = 0x34;
pub const AB8500_DASLOTCONF3: c_uint = 0x35;
pub const AB8500_DASLOTCONF4: c_uint = 0x36;
pub const AB8500_DASLOTCONF5: c_uint = 0x37;
pub const AB8500_DASLOTCONF6: c_uint = 0x38;
pub const AB8500_DASLOTCONF7: c_uint = 0x39;
pub const AB8500_DASLOTCONF8: c_uint = 0x3A;
pub const AB8500_CLASSDCONF1: c_uint = 0x3B;
pub const AB8500_CLASSDCONF2: c_uint = 0x3C;
pub const AB8500_CLASSDCONF3: c_uint = 0x3D;
pub const AB8500_DMICFILTCONF: c_uint = 0x3E;
pub const AB8500_DIGMULTCONF1: c_uint = 0x3F;
pub const AB8500_DIGMULTCONF2: c_uint = 0x40;
pub const AB8500_ADDIGGAIN1: c_uint = 0x41;
pub const AB8500_ADDIGGAIN2: c_uint = 0x42;
pub const AB8500_ADDIGGAIN3: c_uint = 0x43;
pub const AB8500_ADDIGGAIN4: c_uint = 0x44;
pub const AB8500_ADDIGGAIN5: c_uint = 0x45;
pub const AB8500_ADDIGGAIN6: c_uint = 0x46;
pub const AB8500_DADIGGAIN1: c_uint = 0x47;
pub const AB8500_DADIGGAIN2: c_uint = 0x48;
pub const AB8500_DADIGGAIN3: c_uint = 0x49;
pub const AB8500_DADIGGAIN4: c_uint = 0x4A;
pub const AB8500_DADIGGAIN5: c_uint = 0x4B;
pub const AB8500_DADIGGAIN6: c_uint = 0x4C;
pub const AB8500_ADDIGLOOPGAIN1: c_uint = 0x4D;
pub const AB8500_ADDIGLOOPGAIN2: c_uint = 0x4E;
pub const AB8500_HSLEARDIGGAIN: c_uint = 0x4F;
pub const AB8500_HSRDIGGAIN: c_uint = 0x50;
pub const AB8500_SIDFIRGAIN1: c_uint = 0x51;
pub const AB8500_SIDFIRGAIN2: c_uint = 0x52;
pub const AB8500_ANCCONF1: c_uint = 0x53;
pub const AB8500_ANCCONF2: c_uint = 0x54;
pub const AB8500_ANCCONF3: c_uint = 0x55;
pub const AB8500_ANCCONF4: c_uint = 0x56;
pub const AB8500_ANCCONF5: c_uint = 0x57;
pub const AB8500_ANCCONF6: c_uint = 0x58;
pub const AB8500_ANCCONF7: c_uint = 0x59;
pub const AB8500_ANCCONF8: c_uint = 0x5A;
pub const AB8500_ANCCONF9: c_uint = 0x5B;
pub const AB8500_ANCCONF10: c_uint = 0x5C;
pub const AB8500_ANCCONF11: c_uint = 0x5D;
pub const AB8500_ANCCONF12: c_uint = 0x5E;
pub const AB8500_ANCCONF13: c_uint = 0x5F;
pub const AB8500_ANCCONF14: c_uint = 0x60;
pub const AB8500_SIDFIRADR: c_uint = 0x61;
pub const AB8500_SIDFIRCOEF1: c_uint = 0x62;
pub const AB8500_SIDFIRCOEF2: c_uint = 0x63;
pub const AB8500_SIDFIRCONF: c_uint = 0x64;
pub const AB8500_AUDINTMASK1: c_uint = 0x65;
pub const AB8500_AUDINTSOURCE1: c_uint = 0x66;
pub const AB8500_AUDINTMASK2: c_uint = 0x67;
pub const AB8500_AUDINTSOURCE2: c_uint = 0x68;
pub const AB8500_FIFOCONF1: c_uint = 0x69;
pub const AB8500_FIFOCONF2: c_uint = 0x6A;
pub const AB8500_FIFOCONF3: c_uint = 0x6B;
pub const AB8500_FIFOCONF4: c_uint = 0x6C;
pub const AB8500_FIFOCONF5: c_uint = 0x6D;
pub const AB8500_FIFOCONF6: c_uint = 0x6E;
pub const AB8500_AUDREV: c_uint = 0x6F;

pub const AB8500_MASK_ALL: c_uint = 0xFF;

pub const AB8500_MASK_NONE: c_uint = 0x00;
// AB8500_POWERUP
pub const AB8500_POWERUP_POWERUP: c_int = 7;
pub const AB8500_POWERUP_ENANA: c_int = 3;
// AB8500_AUDSWRESET
pub const AB8500_AUDSWRESET_SWRESET: c_int = 7;
// AB8500_ADPATHENA
pub const AB8500_ADPATHENA_ENAD12: c_int = 7;
pub const AB8500_ADPATHENA_ENAD34: c_int = 5;
pub const AB8500_ADPATHENA_ENAD5768: c_int = 3;
// AB8500_DAPATHENA
pub const AB8500_DAPATHENA_ENDA1: c_int = 7;
pub const AB8500_DAPATHENA_ENDA2: c_int = 6;
pub const AB8500_DAPATHENA_ENDA3: c_int = 5;
pub const AB8500_DAPATHENA_ENDA4: c_int = 4;
pub const AB8500_DAPATHENA_ENDA5: c_int = 3;
pub const AB8500_DAPATHENA_ENDA6: c_int = 2;
// AB8500_ANACONF1
pub const AB8500_ANACONF1_HSLOWPOW: c_int = 7;
pub const AB8500_ANACONF1_DACLOWPOW1: c_int = 6;
pub const AB8500_ANACONF1_DACLOWPOW0: c_int = 5;
pub const AB8500_ANACONF1_EARDACLOWPOW: c_int = 4;
pub const AB8500_ANACONF1_EARSELCM: c_int = 2;
pub const AB8500_ANACONF1_HSHPEN: c_int = 1;
pub const AB8500_ANACONF1_EARDRVLOWPOW: c_int = 0;
// AB8500_ANACONF2
pub const AB8500_ANACONF2_ENMIC1: c_int = 7;
pub const AB8500_ANACONF2_ENMIC2: c_int = 6;
pub const AB8500_ANACONF2_ENLINL: c_int = 5;
pub const AB8500_ANACONF2_ENLINR: c_int = 4;
pub const AB8500_ANACONF2_MUTMIC1: c_int = 3;
pub const AB8500_ANACONF2_MUTMIC2: c_int = 2;
pub const AB8500_ANACONF2_MUTLINL: c_int = 1;
pub const AB8500_ANACONF2_MUTLINR: c_int = 0;
// AB8500_DIGMICCONF
pub const AB8500_DIGMICCONF_ENDMIC1: c_int = 7;
pub const AB8500_DIGMICCONF_ENDMIC2: c_int = 6;
pub const AB8500_DIGMICCONF_ENDMIC3: c_int = 5;
pub const AB8500_DIGMICCONF_ENDMIC4: c_int = 4;
pub const AB8500_DIGMICCONF_ENDMIC5: c_int = 3;
pub const AB8500_DIGMICCONF_ENDMIC6: c_int = 2;
pub const AB8500_DIGMICCONF_HSFADSPEED: c_int = 0;
// AB8500_ANACONF3
pub const AB8500_ANACONF3_MIC1SEL: c_int = 7;
pub const AB8500_ANACONF3_LINRSEL: c_int = 6;
pub const AB8500_ANACONF3_ENDRVHSL: c_int = 5;
pub const AB8500_ANACONF3_ENDRVHSR: c_int = 4;
pub const AB8500_ANACONF3_ENADCMIC: c_int = 2;
pub const AB8500_ANACONF3_ENADCLINL: c_int = 1;
pub const AB8500_ANACONF3_ENADCLINR: c_int = 0;
// AB8500_ANACONF4
pub const AB8500_ANACONF4_DISPDVSS: c_int = 7;
pub const AB8500_ANACONF4_ENEAR: c_int = 6;
pub const AB8500_ANACONF4_ENHSL: c_int = 5;
pub const AB8500_ANACONF4_ENHSR: c_int = 4;
pub const AB8500_ANACONF4_ENHFL: c_int = 3;
pub const AB8500_ANACONF4_ENHFR: c_int = 2;
pub const AB8500_ANACONF4_ENVIB1: c_int = 1;
pub const AB8500_ANACONF4_ENVIB2: c_int = 0;
// AB8500_DAPATHCONF
pub const AB8500_DAPATHCONF_ENDACEAR: c_int = 6;
pub const AB8500_DAPATHCONF_ENDACHSL: c_int = 5;
pub const AB8500_DAPATHCONF_ENDACHSR: c_int = 4;
pub const AB8500_DAPATHCONF_ENDACHFL: c_int = 3;
pub const AB8500_DAPATHCONF_ENDACHFR: c_int = 2;
pub const AB8500_DAPATHCONF_ENDACVIB1: c_int = 1;
pub const AB8500_DAPATHCONF_ENDACVIB2: c_int = 0;
// AB8500_MUTECONF
pub const AB8500_MUTECONF_MUTEAR: c_int = 6;
pub const AB8500_MUTECONF_MUTHSL: c_int = 5;
pub const AB8500_MUTECONF_MUTHSR: c_int = 4;
pub const AB8500_MUTECONF_MUTDACEAR: c_int = 2;
pub const AB8500_MUTECONF_MUTDACHSL: c_int = 1;
pub const AB8500_MUTECONF_MUTDACHSR: c_int = 0;
// AB8500_SHORTCIRCONF
pub const AB8500_SHORTCIRCONF_ENSHORTPWD: c_int = 7;
pub const AB8500_SHORTCIRCONF_EARSHORTDIS: c_int = 6;
pub const AB8500_SHORTCIRCONF_HSSHORTDIS: c_int = 5;
pub const AB8500_SHORTCIRCONF_HSPULLDEN: c_int = 4;
pub const AB8500_SHORTCIRCONF_HSOSCEN: c_int = 2;
pub const AB8500_SHORTCIRCONF_HSFADDIS: c_int = 1;
pub const AB8500_SHORTCIRCONF_HSZCDDIS: c_int = 0;
// Zero cross should be disabled
// AB8500_ANACONF5
pub const AB8500_ANACONF5_ENCPHS: c_int = 7;
pub const AB8500_ANACONF5_HSLDACTOLOL: c_int = 5;
pub const AB8500_ANACONF5_HSRDACTOLOR: c_int = 4;
pub const AB8500_ANACONF5_ENLOL: c_int = 3;
pub const AB8500_ANACONF5_ENLOR: c_int = 2;
pub const AB8500_ANACONF5_HSAUTOEN: c_int = 0;
// AB8500_ENVCPCONF
pub const AB8500_ENVCPCONF_ENVDETHTHRE: c_int = 4;
pub const AB8500_ENVCPCONF_ENVDETLTHRE: c_int = 0;
pub const AB8500_ENVCPCONF_ENVDETHTHRE_MAX: c_uint = 0x0F;
pub const AB8500_ENVCPCONF_ENVDETLTHRE_MAX: c_uint = 0x0F;
// AB8500_SIGENVCONF
pub const AB8500_SIGENVCONF_CPLVEN: c_int = 5;
pub const AB8500_SIGENVCONF_ENVDETCPEN: c_int = 4;
pub const AB8500_SIGENVCONF_ENVDETTIME: c_int = 0;
pub const AB8500_SIGENVCONF_ENVDETTIME_MAX: c_uint = 0x0F;
// AB8500_PWMGENCONF1
pub const AB8500_PWMGENCONF1_PWMTOVIB1: c_int = 7;
pub const AB8500_PWMGENCONF1_PWMTOVIB2: c_int = 6;
pub const AB8500_PWMGENCONF1_PWM1CTRL: c_int = 5;
pub const AB8500_PWMGENCONF1_PWM2CTRL: c_int = 4;
pub const AB8500_PWMGENCONF1_PWM1NCTRL: c_int = 3;
pub const AB8500_PWMGENCONF1_PWM1PCTRL: c_int = 2;
pub const AB8500_PWMGENCONF1_PWM2NCTRL: c_int = 1;
pub const AB8500_PWMGENCONF1_PWM2PCTRL: c_int = 0;
// AB8500_PWMGENCONF2
// AB8500_PWMGENCONF3
// AB8500_PWMGENCONF4
// AB8500_PWMGENCONF5
pub const AB8500_PWMGENCONFX_PWMVIBXPOL: c_int = 7;
pub const AB8500_PWMGENCONFX_PWMVIBXDUTCYC: c_int = 0;
pub const AB8500_PWMGENCONFX_PWMVIBXDUTCYC_MAX: c_uint = 0x64;
// AB8500_ANAGAIN1
// AB8500_ANAGAIN2
pub const AB8500_ANAGAINX_ENSEMICX: c_int = 7;
pub const AB8500_ANAGAINX_LOWPOWMICX: c_int = 6;
pub const AB8500_ANAGAINX_MICXGAIN: c_int = 0;
pub const AB8500_ANAGAINX_MICXGAIN_MAX: c_uint = 0x1F;
// AB8500_ANAGAIN3
pub const AB8500_ANAGAIN3_HSLGAIN: c_int = 4;
pub const AB8500_ANAGAIN3_HSRGAIN: c_int = 0;
pub const AB8500_ANAGAIN3_HSXGAIN_MAX: c_uint = 0x0F;
// AB8500_ANAGAIN4
pub const AB8500_ANAGAIN4_LINLGAIN: c_int = 4;
pub const AB8500_ANAGAIN4_LINRGAIN: c_int = 0;
pub const AB8500_ANAGAIN4_LINXGAIN_MAX: c_uint = 0x0F;
// AB8500_DIGLINHSLGAIN
// AB8500_DIGLINHSRGAIN
pub const AB8500_DIGLINHSXGAIN_LINTOHSXGAIN: c_int = 0;
pub const AB8500_DIGLINHSXGAIN_LINTOHSXGAIN_MAX: c_uint = 0x13;
// AB8500_ADFILTCONF
pub const AB8500_ADFILTCONF_AD1NH: c_int = 7;
pub const AB8500_ADFILTCONF_AD2NH: c_int = 6;
pub const AB8500_ADFILTCONF_AD3NH: c_int = 5;
pub const AB8500_ADFILTCONF_AD4NH: c_int = 4;
pub const AB8500_ADFILTCONF_AD1VOICE: c_int = 3;
pub const AB8500_ADFILTCONF_AD2VOICE: c_int = 2;
pub const AB8500_ADFILTCONF_AD3VOICE: c_int = 1;
pub const AB8500_ADFILTCONF_AD4VOICE: c_int = 0;
// AB8500_DIGIFCONF1
pub const AB8500_DIGIFCONF1_ENMASTGEN: c_int = 7;
pub const AB8500_DIGIFCONF1_IF1BITCLKOS1: c_int = 6;
pub const AB8500_DIGIFCONF1_IF1BITCLKOS0: c_int = 5;
pub const AB8500_DIGIFCONF1_ENFSBITCLK1: c_int = 4;
pub const AB8500_DIGIFCONF1_IF0BITCLKOS1: c_int = 2;
pub const AB8500_DIGIFCONF1_IF0BITCLKOS0: c_int = 1;
pub const AB8500_DIGIFCONF1_ENFSBITCLK0: c_int = 0;
// AB8500_DIGIFCONF2
pub const AB8500_DIGIFCONF2_FSYNC0P: c_int = 6;
pub const AB8500_DIGIFCONF2_BITCLK0P: c_int = 5;
pub const AB8500_DIGIFCONF2_IF0DEL: c_int = 4;
pub const AB8500_DIGIFCONF2_IF0FORMAT1: c_int = 3;
pub const AB8500_DIGIFCONF2_IF0FORMAT0: c_int = 2;
pub const AB8500_DIGIFCONF2_IF0WL1: c_int = 1;
pub const AB8500_DIGIFCONF2_IF0WL0: c_int = 0;
// AB8500_DIGIFCONF3
pub const AB8500_DIGIFCONF3_IF0DATOIF1AD: c_int = 7;
pub const AB8500_DIGIFCONF3_IF0CLKTOIF1CLK: c_int = 6;
pub const AB8500_DIGIFCONF3_IF1MASTER: c_int = 5;
pub const AB8500_DIGIFCONF3_IF1DATOIF0AD: c_int = 3;
pub const AB8500_DIGIFCONF3_IF1CLKTOIF0CLK: c_int = 2;
pub const AB8500_DIGIFCONF3_IF0MASTER: c_int = 1;
pub const AB8500_DIGIFCONF3_IF0BFIFOEN: c_int = 0;
// AB8500_DIGIFCONF4
pub const AB8500_DIGIFCONF4_FSYNC1P: c_int = 6;
pub const AB8500_DIGIFCONF4_BITCLK1P: c_int = 5;
pub const AB8500_DIGIFCONF4_IF1DEL: c_int = 4;
pub const AB8500_DIGIFCONF4_IF1FORMAT1: c_int = 3;
pub const AB8500_DIGIFCONF4_IF1FORMAT0: c_int = 2;
pub const AB8500_DIGIFCONF4_IF1WL1: c_int = 1;
pub const AB8500_DIGIFCONF4_IF1WL0: c_int = 0;
// AB8500_ADSLOTSELX
pub const AB8500_AD_OUT1: c_uint = 0x0;
pub const AB8500_AD_OUT2: c_uint = 0x1;
pub const AB8500_AD_OUT3: c_uint = 0x2;
pub const AB8500_AD_OUT4: c_uint = 0x3;
pub const AB8500_AD_OUT5: c_uint = 0x4;
pub const AB8500_AD_OUT6: c_uint = 0x5;
pub const AB8500_AD_OUT7: c_uint = 0x6;
pub const AB8500_AD_OUT8: c_uint = 0x7;
pub const AB8500_ZEROES: c_uint = 0x8;
pub const AB8500_TRISTATE: c_uint = 0xF;
pub const AB8500_ADSLOTSELX_EVEN_SHIFT: c_int = 0;
pub const AB8500_ADSLOTSELX_ODD_SHIFT: c_int = 4;

// AB8500_ADSLOTHIZCTRL1
// AB8500_ADSLOTHIZCTRL2
// AB8500_ADSLOTHIZCTRL3
// AB8500_ADSLOTHIZCTRL4
// AB8500_DASLOTCONF1
pub const AB8500_DASLOTCONF1_DA12VOICE: c_int = 7;
pub const AB8500_DASLOTCONF1_SWAPDA12_34: c_int = 6;
pub const AB8500_DASLOTCONF1_DAI7TOADO1: c_int = 5;
// AB8500_DASLOTCONF2
pub const AB8500_DASLOTCONF2_DAI8TOADO2: c_int = 5;
// AB8500_DASLOTCONF3
pub const AB8500_DASLOTCONF3_DA34VOICE: c_int = 7;
pub const AB8500_DASLOTCONF3_DAI7TOADO3: c_int = 5;
// AB8500_DASLOTCONF4
pub const AB8500_DASLOTCONF4_DAI8TOADO4: c_int = 5;
// AB8500_DASLOTCONF5
pub const AB8500_DASLOTCONF5_DA56VOICE: c_int = 7;
pub const AB8500_DASLOTCONF5_DAI7TOADO5: c_int = 5;
// AB8500_DASLOTCONF6
pub const AB8500_DASLOTCONF6_DAI8TOADO6: c_int = 5;
// AB8500_DASLOTCONF7
pub const AB8500_DASLOTCONF7_DAI8TOADO7: c_int = 5;
// AB8500_DASLOTCONF8
pub const AB8500_DASLOTCONF8_DAI7TOADO8: c_int = 5;
pub const AB8500_DASLOTCONFX_SLTODAX_SHIFT: c_int = 0;
pub const AB8500_DASLOTCONFX_SLTODAX_MASK: c_uint = 0x1F;
// AB8500_CLASSDCONF1
pub const AB8500_CLASSDCONF1_PARLHF: c_int = 7;
pub const AB8500_CLASSDCONF1_PARLVIB: c_int = 6;
pub const AB8500_CLASSDCONF1_VIB1SWAPEN: c_int = 3;
pub const AB8500_CLASSDCONF1_VIB2SWAPEN: c_int = 2;
pub const AB8500_CLASSDCONF1_HFLSWAPEN: c_int = 1;
pub const AB8500_CLASSDCONF1_HFRSWAPEN: c_int = 0;
// AB8500_CLASSDCONF2
pub const AB8500_CLASSDCONF2_FIRBYP3: c_int = 7;
pub const AB8500_CLASSDCONF2_FIRBYP2: c_int = 6;
pub const AB8500_CLASSDCONF2_FIRBYP1: c_int = 5;
pub const AB8500_CLASSDCONF2_FIRBYP0: c_int = 4;
pub const AB8500_CLASSDCONF2_HIGHVOLEN3: c_int = 3;
pub const AB8500_CLASSDCONF2_HIGHVOLEN2: c_int = 2;
pub const AB8500_CLASSDCONF2_HIGHVOLEN1: c_int = 1;
pub const AB8500_CLASSDCONF2_HIGHVOLEN0: c_int = 0;
// AB8500_CLASSDCONF3
pub const AB8500_CLASSDCONF3_DITHHPGAIN: c_int = 4;
pub const AB8500_CLASSDCONF3_DITHHPGAIN_MAX: c_uint = 0x0A;
pub const AB8500_CLASSDCONF3_DITHWGAIN: c_int = 0;
pub const AB8500_CLASSDCONF3_DITHWGAIN_MAX: c_uint = 0x0A;
// AB8500_DMICFILTCONF
pub const AB8500_DMICFILTCONF_ANCINSEL: c_int = 7;
pub const AB8500_DMICFILTCONF_DA3TOEAR: c_int = 6;
pub const AB8500_DMICFILTCONF_DMIC1SINC3: c_int = 5;
pub const AB8500_DMICFILTCONF_DMIC2SINC3: c_int = 4;
pub const AB8500_DMICFILTCONF_DMIC3SINC3: c_int = 3;
pub const AB8500_DMICFILTCONF_DMIC4SINC3: c_int = 2;
pub const AB8500_DMICFILTCONF_DMIC5SINC3: c_int = 1;
pub const AB8500_DMICFILTCONF_DMIC6SINC3: c_int = 0;
// AB8500_DIGMULTCONF1
pub const AB8500_DIGMULTCONF1_DATOHSLEN: c_int = 7;
pub const AB8500_DIGMULTCONF1_DATOHSREN: c_int = 6;
pub const AB8500_DIGMULTCONF1_AD1SEL: c_int = 5;
pub const AB8500_DIGMULTCONF1_AD2SEL: c_int = 4;
pub const AB8500_DIGMULTCONF1_AD3SEL: c_int = 3;
pub const AB8500_DIGMULTCONF1_AD5SEL: c_int = 2;
pub const AB8500_DIGMULTCONF1_AD6SEL: c_int = 1;
pub const AB8500_DIGMULTCONF1_ANCSEL: c_int = 0;
// AB8500_DIGMULTCONF2
pub const AB8500_DIGMULTCONF2_DATOHFREN: c_int = 7;
pub const AB8500_DIGMULTCONF2_DATOHFLEN: c_int = 6;
pub const AB8500_DIGMULTCONF2_HFRSEL: c_int = 5;
pub const AB8500_DIGMULTCONF2_HFLSEL: c_int = 4;
pub const AB8500_DIGMULTCONF2_FIRSID1SEL: c_int = 2;
pub const AB8500_DIGMULTCONF2_FIRSID2SEL: c_int = 0;
// AB8500_ADDIGGAIN1
// AB8500_ADDIGGAIN2
// AB8500_ADDIGGAIN3
// AB8500_ADDIGGAIN4
// AB8500_ADDIGGAIN5
// AB8500_ADDIGGAIN6
pub const AB8500_ADDIGGAINX_FADEDISADX: c_int = 6;
pub const AB8500_ADDIGGAINX_ADXGAIN_MAX: c_uint = 0x3F;
// AB8500_DADIGGAIN1
// AB8500_DADIGGAIN2
// AB8500_DADIGGAIN3
// AB8500_DADIGGAIN4
// AB8500_DADIGGAIN5
// AB8500_DADIGGAIN6
pub const AB8500_DADIGGAINX_FADEDISDAX: c_int = 6;
pub const AB8500_DADIGGAINX_DAXGAIN_MAX: c_uint = 0x3F;
// AB8500_ADDIGLOOPGAIN1
// AB8500_ADDIGLOOPGAIN2
pub const AB8500_ADDIGLOOPGAINX_FADEDISADXL: c_int = 6;
pub const AB8500_ADDIGLOOPGAINX_ADXLBGAIN_MAX: c_uint = 0x3F;
// AB8500_HSLEARDIGGAIN
pub const AB8500_HSLEARDIGGAIN_HSSINC1: c_int = 7;
pub const AB8500_HSLEARDIGGAIN_FADEDISHSL: c_int = 4;
pub const AB8500_HSLEARDIGGAIN_HSLDGAIN_MAX: c_uint = 0x09;
// AB8500_HSRDIGGAIN
pub const AB8500_HSRDIGGAIN_FADESPEED: c_int = 6;
pub const AB8500_HSRDIGGAIN_FADEDISHSR: c_int = 4;
pub const AB8500_HSRDIGGAIN_HSRDGAIN_MAX: c_uint = 0x09;
// AB8500_SIDFIRGAIN1
// AB8500_SIDFIRGAIN2
pub const AB8500_SIDFIRGAINX_FIRSIDXGAIN_MAX: c_uint = 0x1F;
// AB8500_ANCCONF1
pub const AB8500_ANCCONF1_ANCIIRUPDATE: c_int = 3;
pub const AB8500_ANCCONF1_ENANC: c_int = 2;
pub const AB8500_ANCCONF1_ANCIIRINIT: c_int = 1;
pub const AB8500_ANCCONF1_ANCFIRUPDATE: c_int = 0;
// AB8500_ANCCONF2
pub const AB8500_ANCCONF2_SHIFT: c_int = 5;

pub const AB8500_ANCCONF2_MAX: c_uint = 0xF;
// AB8500_ANCCONF3
pub const AB8500_ANCCONF3_SHIFT: c_int = 5;

pub const AB8500_ANCCONF3_MAX: c_uint = 0xF;
// AB8500_ANCCONF4
pub const AB8500_ANCCONF4_SHIFT: c_int = 5;

pub const AB8500_ANCCONF4_MAX: c_uint = 0xF;
// AB8500_ANC_FIR_COEFFS

pub const AB8500_ANC_FIR_COEFF_MAX: c_uint = 0x7FFF;
pub const AB8500_ANC_FIR_COEFFS: c_int = 15;
// AB8500_ANC_IIR_COEFFS

pub const AB8500_ANC_IIR_COEFF_MAX: c_uint = 0x7FFFFF;
pub const AB8500_ANC_IIR_COEFFS: c_int = 24;
// AB8500_ANC_WARP_DELAY
pub const AB8500_ANC_WARP_DELAY_SHIFT: c_int = 16;
pub const AB8500_ANC_WARP_DELAY_MIN: c_uint = 0x0000;
pub const AB8500_ANC_WARP_DELAY_MAX: c_uint = 0xFFFF;
// AB8500_ANCCONF11
// AB8500_ANCCONF12
// AB8500_ANCCONF13
// AB8500_ANCCONF14
// AB8500_SIDFIRADR
pub const AB8500_SIDFIRADR_FIRSIDSET: c_int = 7;
pub const AB8500_SIDFIRADR_ADDRESS_SHIFT: c_int = 0;
pub const AB8500_SIDFIRADR_ADDRESS_MAX: c_uint = 0x7F;
// AB8500_SIDFIRCOEF1
// AB8500_SIDFIRCOEF2
pub const AB8500_SID_FIR_COEFF_MIN: c_int = 0;
pub const AB8500_SID_FIR_COEFF_MAX: c_uint = 0xFFFF;
pub const AB8500_SID_FIR_COEFFS: c_int = 128;
// AB8500_SIDFIRCONF
pub const AB8500_SIDFIRCONF_ENFIRSIDS: c_int = 2;
pub const AB8500_SIDFIRCONF_FIRSIDSTOIF1: c_int = 1;
pub const AB8500_SIDFIRCONF_FIRSIDBUSY: c_int = 0;
// AB8500_AUDINTMASK1
// AB8500_AUDINTSOURCE1
// AB8500_AUDINTMASK2
// AB8500_AUDINTSOURCE2
// AB8500_FIFOCONF1
pub const AB8500_FIFOCONF1_BFIFOMASK: c_uint = 0x80;
pub const AB8500_FIFOCONF1_BFIFO19M2: c_uint = 0x40;
pub const AB8500_FIFOCONF1_BFIFOINT_SHIFT: c_int = 0;
pub const AB8500_FIFOCONF1_BFIFOINT_MAX: c_uint = 0x3F;
// AB8500_FIFOCONF2
pub const AB8500_FIFOCONF2_BFIFOTX_SHIFT: c_int = 0;
pub const AB8500_FIFOCONF2_BFIFOTX_MAX: c_uint = 0xFF;
// AB8500_FIFOCONF3
pub const AB8500_FIFOCONF3_BFIFOEXSL_SHIFT: c_int = 5;
pub const AB8500_FIFOCONF3_BFIFOEXSL_MAX: c_uint = 0x5;
pub const AB8500_FIFOCONF3_PREBITCLK0_SHIFT: c_int = 2;
pub const AB8500_FIFOCONF3_PREBITCLK0_MAX: c_uint = 0x7;
pub const AB8500_FIFOCONF3_BFIFOMAST_SHIFT: c_int = 1;
pub const AB8500_FIFOCONF3_BFIFORUN_SHIFT: c_int = 0;
// AB8500_FIFOCONF4
pub const AB8500_FIFOCONF4_BFIFOFRAMSW_SHIFT: c_int = 0;
pub const AB8500_FIFOCONF4_BFIFOFRAMSW_MAX: c_uint = 0xFF;
// AB8500_FIFOCONF5
pub const AB8500_FIFOCONF5_BFIFOWAKEUP_SHIFT: c_int = 0;
pub const AB8500_FIFOCONF5_BFIFOWAKEUP_MAX: c_uint = 0xFF;
// AB8500_FIFOCONF6
pub const AB8500_FIFOCONF6_BFIFOSAMPLE_SHIFT: c_int = 0;
pub const AB8500_FIFOCONF6_BFIFOSAMPLE_MAX: c_uint = 0xFF;
// AB8500_AUDREV
