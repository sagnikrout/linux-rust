//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs46xx/cs46xx.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Cirrus Logic, Inc.
// Definitions for Cirrus Logic CS46xx chips
//

//
// Direct registers
//
// The following define the offsets of the registers accessed via base address
// register zero on the CS46xx part.
//
pub const BA0_HISR: c_uint = 0x00000000;
pub const BA0_HSR0: c_uint = 0x00000004;
pub const BA0_HICR: c_uint = 0x00000008;
pub const BA0_DMSR: c_uint = 0x00000100;
pub const BA0_HSAR: c_uint = 0x00000110;
pub const BA0_HDAR: c_uint = 0x00000114;
pub const BA0_HDMR: c_uint = 0x00000118;
pub const BA0_HDCR: c_uint = 0x0000011C;
pub const BA0_PFMC: c_uint = 0x00000200;
pub const BA0_PFCV1: c_uint = 0x00000204;
pub const BA0_PFCV2: c_uint = 0x00000208;
pub const BA0_PCICFG00: c_uint = 0x00000300;
pub const BA0_PCICFG04: c_uint = 0x00000304;
pub const BA0_PCICFG08: c_uint = 0x00000308;
pub const BA0_PCICFG0C: c_uint = 0x0000030C;
pub const BA0_PCICFG10: c_uint = 0x00000310;
pub const BA0_PCICFG14: c_uint = 0x00000314;
pub const BA0_PCICFG18: c_uint = 0x00000318;
pub const BA0_PCICFG1C: c_uint = 0x0000031C;
pub const BA0_PCICFG20: c_uint = 0x00000320;
pub const BA0_PCICFG24: c_uint = 0x00000324;
pub const BA0_PCICFG28: c_uint = 0x00000328;
pub const BA0_PCICFG2C: c_uint = 0x0000032C;
pub const BA0_PCICFG30: c_uint = 0x00000330;
pub const BA0_PCICFG34: c_uint = 0x00000334;
pub const BA0_PCICFG38: c_uint = 0x00000338;
pub const BA0_PCICFG3C: c_uint = 0x0000033C;
pub const BA0_CLKCR1: c_uint = 0x00000400;
pub const BA0_CLKCR2: c_uint = 0x00000404;
pub const BA0_PLLM: c_uint = 0x00000408;
pub const BA0_PLLCC: c_uint = 0x0000040C;
pub const BA0_FRR: c_uint = 0x00000410;
pub const BA0_CFL1: c_uint = 0x00000414;
pub const BA0_CFL2: c_uint = 0x00000418;
pub const BA0_SERMC1: c_uint = 0x00000420;
pub const BA0_SERMC2: c_uint = 0x00000424;
pub const BA0_SERC1: c_uint = 0x00000428;
pub const BA0_SERC2: c_uint = 0x0000042C;
pub const BA0_SERC3: c_uint = 0x00000430;
pub const BA0_SERC4: c_uint = 0x00000434;
pub const BA0_SERC5: c_uint = 0x00000438;
pub const BA0_SERBSP: c_uint = 0x0000043C;
pub const BA0_SERBST: c_uint = 0x00000440;
pub const BA0_SERBCM: c_uint = 0x00000444;
pub const BA0_SERBAD: c_uint = 0x00000448;
pub const BA0_SERBCF: c_uint = 0x0000044C;
pub const BA0_SERBWP: c_uint = 0x00000450;
pub const BA0_SERBRP: c_uint = 0x00000454;

pub const BA0_ASER_FADDR: c_uint = 0x00000458;

pub const BA0_ACCTL: c_uint = 0x00000460;
pub const BA0_ACSTS: c_uint = 0x00000464;
pub const BA0_ACOSV: c_uint = 0x00000468;
pub const BA0_ACCAD: c_uint = 0x0000046C;
pub const BA0_ACCDA: c_uint = 0x00000470;
pub const BA0_ACISV: c_uint = 0x00000474;
pub const BA0_ACSAD: c_uint = 0x00000478;
pub const BA0_ACSDA: c_uint = 0x0000047C;
pub const BA0_JSPT: c_uint = 0x00000480;
pub const BA0_JSCTL: c_uint = 0x00000484;
pub const BA0_JSC1: c_uint = 0x00000488;
pub const BA0_JSC2: c_uint = 0x0000048C;
pub const BA0_MIDCR: c_uint = 0x00000490;
pub const BA0_MIDSR: c_uint = 0x00000494;
pub const BA0_MIDWP: c_uint = 0x00000498;
pub const BA0_MIDRP: c_uint = 0x0000049C;
pub const BA0_JSIO: c_uint = 0x000004A0;

pub const BA0_ASER_MASTER: c_uint = 0x000004A4;

pub const BA0_CFGI: c_uint = 0x000004B0;
pub const BA0_SSVID: c_uint = 0x000004B4;
pub const BA0_GPIOR: c_uint = 0x000004B8;

pub const BA0_EGPIODR: c_uint = 0x000004BC;
pub const BA0_EGPIOPTR: c_uint = 0x000004C0;
pub const BA0_EGPIOTR: c_uint = 0x000004C4;
pub const BA0_EGPIOWR: c_uint = 0x000004C8;
pub const BA0_EGPIOSR: c_uint = 0x000004CC;
pub const BA0_SERC6: c_uint = 0x000004D0;
pub const BA0_SERC7: c_uint = 0x000004D4;
pub const BA0_SERACC: c_uint = 0x000004D8;
pub const BA0_ACCTL2: c_uint = 0x000004E0;
pub const BA0_ACSTS2: c_uint = 0x000004E4;
pub const BA0_ACOSV2: c_uint = 0x000004E8;
pub const BA0_ACCAD2: c_uint = 0x000004EC;
pub const BA0_ACCDA2: c_uint = 0x000004F0;
pub const BA0_ACISV2: c_uint = 0x000004F4;
pub const BA0_ACSAD2: c_uint = 0x000004F8;
pub const BA0_ACSDA2: c_uint = 0x000004FC;
pub const BA0_IOTAC0: c_uint = 0x00000500;
pub const BA0_IOTAC1: c_uint = 0x00000504;
pub const BA0_IOTAC2: c_uint = 0x00000508;
pub const BA0_IOTAC3: c_uint = 0x0000050C;
pub const BA0_IOTAC4: c_uint = 0x00000510;
pub const BA0_IOTAC5: c_uint = 0x00000514;
pub const BA0_IOTAC6: c_uint = 0x00000518;
pub const BA0_IOTAC7: c_uint = 0x0000051C;
pub const BA0_IOTAC8: c_uint = 0x00000520;
pub const BA0_IOTAC9: c_uint = 0x00000524;
pub const BA0_IOTAC10: c_uint = 0x00000528;
pub const BA0_IOTAC11: c_uint = 0x0000052C;
pub const BA0_IOTFR0: c_uint = 0x00000540;
pub const BA0_IOTFR1: c_uint = 0x00000544;
pub const BA0_IOTFR2: c_uint = 0x00000548;
pub const BA0_IOTFR3: c_uint = 0x0000054C;
pub const BA0_IOTFR4: c_uint = 0x00000550;
pub const BA0_IOTFR5: c_uint = 0x00000554;
pub const BA0_IOTFR6: c_uint = 0x00000558;
pub const BA0_IOTFR7: c_uint = 0x0000055C;
pub const BA0_IOTFIFO: c_uint = 0x00000580;
pub const BA0_IOTRRD: c_uint = 0x00000584;
pub const BA0_IOTFP: c_uint = 0x00000588;
pub const BA0_IOTCR: c_uint = 0x0000058C;
pub const BA0_DPCID: c_uint = 0x00000590;
pub const BA0_DPCIA: c_uint = 0x00000594;
pub const BA0_DPCIC: c_uint = 0x00000598;
pub const BA0_PCPCIR: c_uint = 0x00000600;
pub const BA0_PCPCIG: c_uint = 0x00000604;
pub const BA0_PCPCIEN: c_uint = 0x00000608;
pub const BA0_EPCIPMC: c_uint = 0x00000610;

//
// The following define the offsets of the registers and memories accessed via
// base address register one on the CS46xx part.
//
pub const BA1_SP_DMEM0: c_uint = 0x00000000;
pub const BA1_SP_DMEM1: c_uint = 0x00010000;
pub const BA1_SP_PMEM: c_uint = 0x00020000;
pub const BA1_SP_REG: c_uint = 0x00030000;
pub const BA1_SPCR: c_uint = 0x00030000;
pub const BA1_DREG: c_uint = 0x00030004;
pub const BA1_DSRWP: c_uint = 0x00030008;
pub const BA1_TWPR: c_uint = 0x0003000C;
pub const BA1_SPWR: c_uint = 0x00030010;
pub const BA1_SPIR: c_uint = 0x00030014;
pub const BA1_FGR1: c_uint = 0x00030020;
pub const BA1_SPCS: c_uint = 0x00030028;
pub const BA1_SDSR: c_uint = 0x0003002C;
pub const BA1_FRMT: c_uint = 0x00030030;
pub const BA1_FRCC: c_uint = 0x00030034;
pub const BA1_FRSC: c_uint = 0x00030038;
pub const BA1_OMNI_MEM: c_uint = 0x000E0000;
//
// The following defines are for the flags in the host interrupt status
// register.
//
pub const HISR_VC_MASK: c_uint = 0x0000FFFF;
pub const HISR_VC0: c_uint = 0x00000001;
pub const HISR_VC1: c_uint = 0x00000002;
pub const HISR_VC2: c_uint = 0x00000004;
pub const HISR_VC3: c_uint = 0x00000008;
pub const HISR_VC4: c_uint = 0x00000010;
pub const HISR_VC5: c_uint = 0x00000020;
pub const HISR_VC6: c_uint = 0x00000040;
pub const HISR_VC7: c_uint = 0x00000080;
pub const HISR_VC8: c_uint = 0x00000100;
pub const HISR_VC9: c_uint = 0x00000200;
pub const HISR_VC10: c_uint = 0x00000400;
pub const HISR_VC11: c_uint = 0x00000800;
pub const HISR_VC12: c_uint = 0x00001000;
pub const HISR_VC13: c_uint = 0x00002000;
pub const HISR_VC14: c_uint = 0x00004000;
pub const HISR_VC15: c_uint = 0x00008000;
pub const HISR_INT0: c_uint = 0x00010000;
pub const HISR_INT1: c_uint = 0x00020000;
pub const HISR_DMAI: c_uint = 0x00040000;
pub const HISR_FROVR: c_uint = 0x00080000;
pub const HISR_MIDI: c_uint = 0x00100000;

pub const HISR_RESERVED: c_uint = 0x0FE00000;

pub const HISR_SBINT: c_uint = 0x00200000;
pub const HISR_RESERVED: c_uint = 0x0FC00000;

pub const HISR_H0P: c_uint = 0x40000000;
pub const HISR_INTENA: c_uint = 0x80000000;
//
// The following defines are for the flags in the host signal register 0.
//
pub const HSR0_VC_MASK: c_uint = 0xFFFFFFFF;
pub const HSR0_VC16: c_uint = 0x00000001;
pub const HSR0_VC17: c_uint = 0x00000002;
pub const HSR0_VC18: c_uint = 0x00000004;
pub const HSR0_VC19: c_uint = 0x00000008;
pub const HSR0_VC20: c_uint = 0x00000010;
pub const HSR0_VC21: c_uint = 0x00000020;
pub const HSR0_VC22: c_uint = 0x00000040;
pub const HSR0_VC23: c_uint = 0x00000080;
pub const HSR0_VC24: c_uint = 0x00000100;
pub const HSR0_VC25: c_uint = 0x00000200;
pub const HSR0_VC26: c_uint = 0x00000400;
pub const HSR0_VC27: c_uint = 0x00000800;
pub const HSR0_VC28: c_uint = 0x00001000;
pub const HSR0_VC29: c_uint = 0x00002000;
pub const HSR0_VC30: c_uint = 0x00004000;
pub const HSR0_VC31: c_uint = 0x00008000;
pub const HSR0_VC32: c_uint = 0x00010000;
pub const HSR0_VC33: c_uint = 0x00020000;
pub const HSR0_VC34: c_uint = 0x00040000;
pub const HSR0_VC35: c_uint = 0x00080000;
pub const HSR0_VC36: c_uint = 0x00100000;
pub const HSR0_VC37: c_uint = 0x00200000;
pub const HSR0_VC38: c_uint = 0x00400000;
pub const HSR0_VC39: c_uint = 0x00800000;
pub const HSR0_VC40: c_uint = 0x01000000;
pub const HSR0_VC41: c_uint = 0x02000000;
pub const HSR0_VC42: c_uint = 0x04000000;
pub const HSR0_VC43: c_uint = 0x08000000;
pub const HSR0_VC44: c_uint = 0x10000000;
pub const HSR0_VC45: c_uint = 0x20000000;
pub const HSR0_VC46: c_uint = 0x40000000;
pub const HSR0_VC47: c_uint = 0x80000000;
//
// The following defines are for the flags in the host interrupt control
// register.
//
pub const HICR_IEV: c_uint = 0x00000001;
pub const HICR_CHGM: c_uint = 0x00000002;
//
// The following defines are for the flags in the DMA status register.
//
pub const DMSR_HP: c_uint = 0x00000001;
pub const DMSR_HR: c_uint = 0x00000002;
pub const DMSR_SP: c_uint = 0x00000004;
pub const DMSR_SR: c_uint = 0x00000008;
//
// The following defines are for the flags in the host DMA source address
// register.
//
pub const HSAR_HOST_ADDR_MASK: c_uint = 0xFFFFFFFF;
pub const HSAR_DSP_ADDR_MASK: c_uint = 0x0000FFFF;
pub const HSAR_MEMID_MASK: c_uint = 0x000F0000;
pub const HSAR_MEMID_SP_DMEM0: c_uint = 0x00000000;
pub const HSAR_MEMID_SP_DMEM1: c_uint = 0x00010000;
pub const HSAR_MEMID_SP_PMEM: c_uint = 0x00020000;
pub const HSAR_MEMID_SP_DEBUG: c_uint = 0x00030000;
pub const HSAR_MEMID_OMNI_MEM: c_uint = 0x000E0000;
pub const HSAR_END: c_uint = 0x40000000;
pub const HSAR_ERR: c_uint = 0x80000000;
//
// The following defines are for the flags in the host DMA destination address
// register.
//
pub const HDAR_HOST_ADDR_MASK: c_uint = 0xFFFFFFFF;
pub const HDAR_DSP_ADDR_MASK: c_uint = 0x0000FFFF;
pub const HDAR_MEMID_MASK: c_uint = 0x000F0000;
pub const HDAR_MEMID_SP_DMEM0: c_uint = 0x00000000;
pub const HDAR_MEMID_SP_DMEM1: c_uint = 0x00010000;
pub const HDAR_MEMID_SP_PMEM: c_uint = 0x00020000;
pub const HDAR_MEMID_SP_DEBUG: c_uint = 0x00030000;
pub const HDAR_MEMID_OMNI_MEM: c_uint = 0x000E0000;
pub const HDAR_END: c_uint = 0x40000000;
pub const HDAR_ERR: c_uint = 0x80000000;
//
// The following defines are for the flags in the host DMA control register.
//
pub const HDMR_AC_MASK: c_uint = 0x0000F000;
pub const HDMR_AC_8_16: c_uint = 0x00001000;
pub const HDMR_AC_M_S: c_uint = 0x00002000;
pub const HDMR_AC_B_L: c_uint = 0x00004000;
pub const HDMR_AC_S_U: c_uint = 0x00008000;
//
// The following defines are for the flags in the host DMA control register.
//
pub const HDCR_COUNT_MASK: c_uint = 0x000003FF;
pub const HDCR_DONE: c_uint = 0x00004000;
pub const HDCR_OPT: c_uint = 0x00008000;
pub const HDCR_WBD: c_uint = 0x00400000;
pub const HDCR_WBS: c_uint = 0x00800000;
pub const HDCR_DMS_MASK: c_uint = 0x07000000;
pub const HDCR_DMS_LINEAR: c_uint = 0x00000000;
pub const HDCR_DMS_16_DWORDS: c_uint = 0x01000000;
pub const HDCR_DMS_32_DWORDS: c_uint = 0x02000000;
pub const HDCR_DMS_64_DWORDS: c_uint = 0x03000000;
pub const HDCR_DMS_128_DWORDS: c_uint = 0x04000000;
pub const HDCR_DMS_256_DWORDS: c_uint = 0x05000000;
pub const HDCR_DMS_512_DWORDS: c_uint = 0x06000000;
pub const HDCR_DMS_1024_DWORDS: c_uint = 0x07000000;
pub const HDCR_DH: c_uint = 0x08000000;
pub const HDCR_SMS_MASK: c_uint = 0x70000000;
pub const HDCR_SMS_LINEAR: c_uint = 0x00000000;
pub const HDCR_SMS_16_DWORDS: c_uint = 0x10000000;
pub const HDCR_SMS_32_DWORDS: c_uint = 0x20000000;
pub const HDCR_SMS_64_DWORDS: c_uint = 0x30000000;
pub const HDCR_SMS_128_DWORDS: c_uint = 0x40000000;
pub const HDCR_SMS_256_DWORDS: c_uint = 0x50000000;
pub const HDCR_SMS_512_DWORDS: c_uint = 0x60000000;
pub const HDCR_SMS_1024_DWORDS: c_uint = 0x70000000;
pub const HDCR_SH: c_uint = 0x80000000;
pub const HDCR_COUNT_SHIFT: c_int = 0;
//
// The following defines are for the flags in the performance monitor control
// register.
//
pub const PFMC_C1SS_MASK: c_uint = 0x0000001F;
pub const PFMC_C1EV: c_uint = 0x00000020;
pub const PFMC_C1RS: c_uint = 0x00008000;
pub const PFMC_C2SS_MASK: c_uint = 0x001F0000;
pub const PFMC_C2EV: c_uint = 0x00200000;
pub const PFMC_C2RS: c_uint = 0x80000000;
pub const PFMC_C1SS_SHIFT: c_int = 0;
pub const PFMC_C2SS_SHIFT: c_int = 16;
pub const PFMC_BUS_GRANT: c_int = 0;
pub const PFMC_GRANT_AFTER_REQ: c_int = 1;
pub const PFMC_TRANSACTION: c_int = 2;
pub const PFMC_DWORD_TRANSFER: c_int = 3;
pub const PFMC_SLAVE_READ: c_int = 4;
pub const PFMC_SLAVE_WRITE: c_int = 5;
pub const PFMC_PREEMPTION: c_int = 6;
pub const PFMC_DISCONNECT_RETRY: c_int = 7;
pub const PFMC_INTERRUPT: c_int = 8;
pub const PFMC_BUS_OWNERSHIP: c_int = 9;
pub const PFMC_TRANSACTION_LAG: c_int = 10;
pub const PFMC_PCI_CLOCK: c_int = 11;
pub const PFMC_SERIAL_CLOCK: c_int = 12;
pub const PFMC_SP_CLOCK: c_int = 13;
//
// The following defines are for the flags in the performance counter value 1
// register.
//
pub const PFCV1_PC1V_MASK: c_uint = 0xFFFFFFFF;
pub const PFCV1_PC1V_SHIFT: c_int = 0;
//
// The following defines are for the flags in the performance counter value 2
// register.
//
pub const PFCV2_PC2V_MASK: c_uint = 0xFFFFFFFF;
pub const PFCV2_PC2V_SHIFT: c_int = 0;
//
// The following defines are for the flags in the clock control register 1.
//
pub const CLKCR1_OSCS: c_uint = 0x00000001;
pub const CLKCR1_OSCP: c_uint = 0x00000002;
pub const CLKCR1_PLLSS_MASK: c_uint = 0x0000000C;
pub const CLKCR1_PLLSS_SERIAL: c_uint = 0x00000000;
pub const CLKCR1_PLLSS_CRYSTAL: c_uint = 0x00000004;
pub const CLKCR1_PLLSS_PCI: c_uint = 0x00000008;
pub const CLKCR1_PLLSS_RESERVED: c_uint = 0x0000000C;
pub const CLKCR1_PLLP: c_uint = 0x00000010;
pub const CLKCR1_SWCE: c_uint = 0x00000020;
pub const CLKCR1_PLLOS: c_uint = 0x00000040;
//
// The following defines are for the flags in the clock control register 2.
//
pub const CLKCR2_PDIVS_MASK: c_uint = 0x0000000F;
pub const CLKCR2_PDIVS_1: c_uint = 0x00000001;
pub const CLKCR2_PDIVS_2: c_uint = 0x00000002;
pub const CLKCR2_PDIVS_4: c_uint = 0x00000004;
pub const CLKCR2_PDIVS_7: c_uint = 0x00000007;
pub const CLKCR2_PDIVS_8: c_uint = 0x00000008;
pub const CLKCR2_PDIVS_16: c_uint = 0x00000000;
//
// The following defines are for the flags in the PLL multiplier register.
//
pub const PLLM_MASK: c_uint = 0x000000FF;
pub const PLLM_SHIFT: c_int = 0;
//
// The following defines are for the flags in the PLL capacitor coefficient
// register.
//
pub const PLLCC_CDR_MASK: c_uint = 0x00000007;

pub const PLLCC_CDR_240_350_MHZ: c_uint = 0x00000000;
pub const PLLCC_CDR_184_265_MHZ: c_uint = 0x00000001;
pub const PLLCC_CDR_144_205_MHZ: c_uint = 0x00000002;
pub const PLLCC_CDR_111_160_MHZ: c_uint = 0x00000003;
pub const PLLCC_CDR_87_123_MHZ: c_uint = 0x00000004;
pub const PLLCC_CDR_67_96_MHZ: c_uint = 0x00000005;
pub const PLLCC_CDR_52_74_MHZ: c_uint = 0x00000006;
pub const PLLCC_CDR_45_58_MHZ: c_uint = 0x00000007;

pub const PLLCC_CDR_271_398_MHZ: c_uint = 0x00000000;
pub const PLLCC_CDR_227_330_MHZ: c_uint = 0x00000001;
pub const PLLCC_CDR_167_239_MHZ: c_uint = 0x00000002;
pub const PLLCC_CDR_150_215_MHZ: c_uint = 0x00000003;
pub const PLLCC_CDR_107_154_MHZ: c_uint = 0x00000004;
pub const PLLCC_CDR_98_140_MHZ: c_uint = 0x00000005;
pub const PLLCC_CDR_73_104_MHZ: c_uint = 0x00000006;
pub const PLLCC_CDR_63_90_MHZ: c_uint = 0x00000007;

pub const PLLCC_LPF_MASK: c_uint = 0x000000F8;

pub const PLLCC_LPF_23850_60000_KHZ: c_uint = 0x00000000;
pub const PLLCC_LPF_7960_26290_KHZ: c_uint = 0x00000008;
pub const PLLCC_LPF_4160_10980_KHZ: c_uint = 0x00000018;
pub const PLLCC_LPF_1740_4580_KHZ: c_uint = 0x00000038;
pub const PLLCC_LPF_724_1910_KHZ: c_uint = 0x00000078;
pub const PLLCC_LPF_317_798_KHZ: c_uint = 0x000000F8;

pub const PLLCC_LPF_25580_64530_KHZ: c_uint = 0x00000000;
pub const PLLCC_LPF_14360_37270_KHZ: c_uint = 0x00000008;
pub const PLLCC_LPF_6100_16020_KHZ: c_uint = 0x00000018;
pub const PLLCC_LPF_2540_6690_KHZ: c_uint = 0x00000038;
pub const PLLCC_LPF_1050_2780_KHZ: c_uint = 0x00000078;
pub const PLLCC_LPF_450_1160_KHZ: c_uint = 0x000000F8;

//
// The following defines are for the flags in the feature reporting register.
//
pub const FRR_FAB_MASK: c_uint = 0x00000003;
pub const FRR_MASK_MASK: c_uint = 0x0000001C;

pub const FRR_CFOP_MASK: c_uint = 0x000000E0;

pub const FRR_CFOP_MASK: c_uint = 0x00000FE0;

pub const FRR_CFOP_NOT_DVD: c_uint = 0x00000020;
pub const FRR_CFOP_A3D: c_uint = 0x00000040;
pub const FRR_CFOP_128_PIN: c_uint = 0x00000080;

pub const FRR_CFOP_CS4280: c_uint = 0x00000800;

pub const FRR_FAB_SHIFT: c_int = 0;
pub const FRR_MASK_SHIFT: c_int = 2;
pub const FRR_CFOP_SHIFT: c_int = 5;
//
// The following defines are for the flags in the configuration load 1
// register.
//
pub const CFL1_CLOCK_SOURCE_MASK: c_uint = 0x00000003;
pub const CFL1_CLOCK_SOURCE_CS423X: c_uint = 0x00000000;
pub const CFL1_CLOCK_SOURCE_AC97: c_uint = 0x00000001;
pub const CFL1_CLOCK_SOURCE_CRYSTAL: c_uint = 0x00000002;
pub const CFL1_CLOCK_SOURCE_DUAL_AC97: c_uint = 0x00000003;
pub const CFL1_VALID_DATA_MASK: c_uint = 0x000000FF;
//
// The following defines are for the flags in the configuration load 2
// register.
//
pub const CFL2_VALID_DATA_MASK: c_uint = 0x000000FF;
//
// The following defines are for the flags in the serial port master control
// register 1.
//
pub const SERMC1_MSPE: c_uint = 0x00000001;
pub const SERMC1_PTC_MASK: c_uint = 0x0000000E;
pub const SERMC1_PTC_CS423X: c_uint = 0x00000000;
pub const SERMC1_PTC_AC97: c_uint = 0x00000002;
pub const SERMC1_PTC_DAC: c_uint = 0x00000004;
pub const SERMC1_PLB: c_uint = 0x00000010;
pub const SERMC1_XLB: c_uint = 0x00000020;
//
// The following defines are for the flags in the serial port master control
// register 2.
//
pub const SERMC2_LROE: c_uint = 0x00000001;
pub const SERMC2_MCOE: c_uint = 0x00000002;
pub const SERMC2_MCDIV: c_uint = 0x00000004;
//
// The following defines are for the flags in the serial port 1 configuration
// register.
//
pub const SERC1_SO1EN: c_uint = 0x00000001;
pub const SERC1_SO1F_MASK: c_uint = 0x0000000E;
pub const SERC1_SO1F_CS423X: c_uint = 0x00000000;
pub const SERC1_SO1F_AC97: c_uint = 0x00000002;
pub const SERC1_SO1F_DAC: c_uint = 0x00000004;
pub const SERC1_SO1F_SPDIF: c_uint = 0x00000006;
//
// The following defines are for the flags in the serial port 2 configuration
// register.
//
pub const SERC2_SI1EN: c_uint = 0x00000001;
pub const SERC2_SI1F_MASK: c_uint = 0x0000000E;
pub const SERC2_SI1F_CS423X: c_uint = 0x00000000;
pub const SERC2_SI1F_AC97: c_uint = 0x00000002;
pub const SERC2_SI1F_ADC: c_uint = 0x00000004;
pub const SERC2_SI1F_SPDIF: c_uint = 0x00000006;
//
// The following defines are for the flags in the serial port 3 configuration
// register.
//
pub const SERC3_SO2EN: c_uint = 0x00000001;
pub const SERC3_SO2F_MASK: c_uint = 0x00000006;
pub const SERC3_SO2F_DAC: c_uint = 0x00000000;
pub const SERC3_SO2F_SPDIF: c_uint = 0x00000002;
//
// The following defines are for the flags in the serial port 4 configuration
// register.
//
pub const SERC4_SO3EN: c_uint = 0x00000001;
pub const SERC4_SO3F_MASK: c_uint = 0x00000006;
pub const SERC4_SO3F_DAC: c_uint = 0x00000000;
pub const SERC4_SO3F_SPDIF: c_uint = 0x00000002;
//
// The following defines are for the flags in the serial port 5 configuration
// register.
//
pub const SERC5_SI2EN: c_uint = 0x00000001;
pub const SERC5_SI2F_MASK: c_uint = 0x00000006;
pub const SERC5_SI2F_ADC: c_uint = 0x00000000;
pub const SERC5_SI2F_SPDIF: c_uint = 0x00000002;
//
// The following defines are for the flags in the serial port backdoor sample
// pointer register.
//
pub const SERBSP_FSP_MASK: c_uint = 0x0000000F;
pub const SERBSP_FSP_SHIFT: c_int = 0;
//
// The following defines are for the flags in the serial port backdoor status
// register.
//
pub const SERBST_RRDY: c_uint = 0x00000001;
pub const SERBST_WBSY: c_uint = 0x00000002;
//
// The following defines are for the flags in the serial port backdoor command
// register.
//
pub const SERBCM_RDC: c_uint = 0x00000001;
pub const SERBCM_WRC: c_uint = 0x00000002;
//
// The following defines are for the flags in the serial port backdoor address
// register.
//

pub const SERBAD_FAD_MASK: c_uint = 0x000000FF;

pub const SERBAD_FAD_MASK: c_uint = 0x000001FF;

pub const SERBAD_FAD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the serial port backdoor
// configuration register.
//
pub const SERBCF_HBP: c_uint = 0x00000001;
//
// The following defines are for the flags in the serial port backdoor write
// port register.
//
pub const SERBWP_FWD_MASK: c_uint = 0x000FFFFF;
pub const SERBWP_FWD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the serial port backdoor read
// port register.
//
pub const SERBRP_FRD_MASK: c_uint = 0x000FFFFF;
pub const SERBRP_FRD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the async FIFO address register.
//

pub const ASER_FADDR_A1_MASK: c_uint = 0x000001FF;
pub const ASER_FADDR_EN1: c_uint = 0x00008000;
pub const ASER_FADDR_A2_MASK: c_uint = 0x01FF0000;
pub const ASER_FADDR_EN2: c_uint = 0x80000000;
pub const ASER_FADDR_A1_SHIFT: c_int = 0;
pub const ASER_FADDR_A2_SHIFT: c_int = 16;

//
// The following defines are for the flags in the AC97 control register.
//
pub const ACCTL_RSTN: c_uint = 0x00000001;
pub const ACCTL_ESYN: c_uint = 0x00000002;
pub const ACCTL_VFRM: c_uint = 0x00000004;
pub const ACCTL_DCV: c_uint = 0x00000008;
pub const ACCTL_CRW: c_uint = 0x00000010;
pub const ACCTL_ASYN: c_uint = 0x00000020;

pub const ACCTL_TC: c_uint = 0x00000040;

//
// The following defines are for the flags in the AC97 status register.
//
pub const ACSTS_CRDY: c_uint = 0x00000001;
pub const ACSTS_VSTS: c_uint = 0x00000002;

pub const ACSTS_WKUP: c_uint = 0x00000004;

//
// The following defines are for the flags in the AC97 output slot valid
// register.
//
pub const ACOSV_SLV3: c_uint = 0x00000001;
pub const ACOSV_SLV4: c_uint = 0x00000002;
pub const ACOSV_SLV5: c_uint = 0x00000004;
pub const ACOSV_SLV6: c_uint = 0x00000008;
pub const ACOSV_SLV7: c_uint = 0x00000010;
pub const ACOSV_SLV8: c_uint = 0x00000020;
pub const ACOSV_SLV9: c_uint = 0x00000040;
pub const ACOSV_SLV10: c_uint = 0x00000080;
pub const ACOSV_SLV11: c_uint = 0x00000100;
pub const ACOSV_SLV12: c_uint = 0x00000200;
//
// The following defines are for the flags in the AC97 command address
// register.
//
pub const ACCAD_CI_MASK: c_uint = 0x0000007F;
pub const ACCAD_CI_SHIFT: c_int = 0;
//
// The following defines are for the flags in the AC97 command data register.
//
pub const ACCDA_CD_MASK: c_uint = 0x0000FFFF;
pub const ACCDA_CD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the AC97 input slot valid
// register.
//
pub const ACISV_ISV3: c_uint = 0x00000001;
pub const ACISV_ISV4: c_uint = 0x00000002;
pub const ACISV_ISV5: c_uint = 0x00000004;
pub const ACISV_ISV6: c_uint = 0x00000008;
pub const ACISV_ISV7: c_uint = 0x00000010;
pub const ACISV_ISV8: c_uint = 0x00000020;
pub const ACISV_ISV9: c_uint = 0x00000040;
pub const ACISV_ISV10: c_uint = 0x00000080;
pub const ACISV_ISV11: c_uint = 0x00000100;
pub const ACISV_ISV12: c_uint = 0x00000200;
//
// The following defines are for the flags in the AC97 status address
// register.
//
pub const ACSAD_SI_MASK: c_uint = 0x0000007F;
pub const ACSAD_SI_SHIFT: c_int = 0;
//
// The following defines are for the flags in the AC97 status data register.
//
pub const ACSDA_SD_MASK: c_uint = 0x0000FFFF;
pub const ACSDA_SD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the joystick poll/trigger
// register.
//
pub const JSPT_CAX: c_uint = 0x00000001;
pub const JSPT_CAY: c_uint = 0x00000002;
pub const JSPT_CBX: c_uint = 0x00000004;
pub const JSPT_CBY: c_uint = 0x00000008;
pub const JSPT_BA1: c_uint = 0x00000010;
pub const JSPT_BA2: c_uint = 0x00000020;
pub const JSPT_BB1: c_uint = 0x00000040;
pub const JSPT_BB2: c_uint = 0x00000080;
//
// The following defines are for the flags in the joystick control register.
//
pub const JSCTL_SP_MASK: c_uint = 0x00000003;
pub const JSCTL_SP_SLOW: c_uint = 0x00000000;
pub const JSCTL_SP_MEDIUM_SLOW: c_uint = 0x00000001;
pub const JSCTL_SP_MEDIUM_FAST: c_uint = 0x00000002;
pub const JSCTL_SP_FAST: c_uint = 0x00000003;
pub const JSCTL_ARE: c_uint = 0x00000004;
//
// The following defines are for the flags in the joystick coordinate pair 1
// readback register.
//
pub const JSC1_Y1V_MASK: c_uint = 0x0000FFFF;
pub const JSC1_X1V_MASK: c_uint = 0xFFFF0000;
pub const JSC1_Y1V_SHIFT: c_int = 0;
pub const JSC1_X1V_SHIFT: c_int = 16;
//
// The following defines are for the flags in the joystick coordinate pair 2
// readback register.
//
pub const JSC2_Y2V_MASK: c_uint = 0x0000FFFF;
pub const JSC2_X2V_MASK: c_uint = 0xFFFF0000;
pub const JSC2_Y2V_SHIFT: c_int = 0;
pub const JSC2_X2V_SHIFT: c_int = 16;
//
// The following defines are for the flags in the MIDI control register.
//
pub const MIDCR_TXE: c_uint = 0x00000001	/* Enable transmitting. */;
pub const MIDCR_RXE: c_uint = 0x00000002	/* Enable receiving. */;
pub const MIDCR_RIE: c_uint = 0x00000004	/* Interrupt upon tx ready. */;
pub const MIDCR_TIE: c_uint = 0x00000008	/* Interrupt upon rx ready. */;
pub const MIDCR_MLB: c_uint = 0x00000010	/* Enable midi loopback. */;
pub const MIDCR_MRST: c_uint = 0x00000020	/* Reset interface. */;
//
// The following defines are for the flags in the MIDI status register.
//
pub const MIDSR_TBF: c_uint = 0x00000001	/* Tx FIFO is full. */;
pub const MIDSR_RBE: c_uint = 0x00000002	/* Rx FIFO is empty. */;
//
// The following defines are for the flags in the MIDI write port register.
//
pub const MIDWP_MWD_MASK: c_uint = 0x000000FF;
pub const MIDWP_MWD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the MIDI read port register.
//
pub const MIDRP_MRD_MASK: c_uint = 0x000000FF;
pub const MIDRP_MRD_SHIFT: c_int = 0;
//
// The following defines are for the flags in the joystick GPIO register.
//
pub const JSIO_DAX: c_uint = 0x00000001;
pub const JSIO_DAY: c_uint = 0x00000002;
pub const JSIO_DBX: c_uint = 0x00000004;
pub const JSIO_DBY: c_uint = 0x00000008;
pub const JSIO_AXOE: c_uint = 0x00000010;
pub const JSIO_AYOE: c_uint = 0x00000020;
pub const JSIO_BXOE: c_uint = 0x00000040;
pub const JSIO_BYOE: c_uint = 0x00000080;
//
// The following defines are for the flags in the master async/sync serial
// port enable register.
//

pub const ASER_MASTER_ME: c_uint = 0x00000001;

//
// The following defines are for the flags in the configuration interface
// register.
//
pub const CFGI_CLK: c_uint = 0x00000001;
pub const CFGI_DOUT: c_uint = 0x00000002;
pub const CFGI_DIN_EEN: c_uint = 0x00000004;
pub const CFGI_EELD: c_uint = 0x00000008;
//
// The following defines are for the flags in the subsystem ID and vendor ID
// register.
//
pub const SSVID_VID_MASK: c_uint = 0x0000FFFF;
pub const SSVID_SID_MASK: c_uint = 0xFFFF0000;
pub const SSVID_VID_SHIFT: c_int = 0;
pub const SSVID_SID_SHIFT: c_int = 16;
//
// The following defines are for the flags in the GPIO pin interface register.
//
pub const GPIOR_VOLDN: c_uint = 0x00000001;
pub const GPIOR_VOLUP: c_uint = 0x00000002;
pub const GPIOR_SI2D: c_uint = 0x00000004;
pub const GPIOR_SI2OE: c_uint = 0x00000008;
//
// The following defines are for the flags in the extended GPIO pin direction
// register.
//

pub const EGPIODR_GPOE0: c_uint = 0x00000001;
pub const EGPIODR_GPOE1: c_uint = 0x00000002;
pub const EGPIODR_GPOE2: c_uint = 0x00000004;
pub const EGPIODR_GPOE3: c_uint = 0x00000008;
pub const EGPIODR_GPOE4: c_uint = 0x00000010;
pub const EGPIODR_GPOE5: c_uint = 0x00000020;
pub const EGPIODR_GPOE6: c_uint = 0x00000040;
pub const EGPIODR_GPOE7: c_uint = 0x00000080;
pub const EGPIODR_GPOE8: c_uint = 0x00000100;

//
// The following defines are for the flags in the extended GPIO pin polarity
// type register.
//

pub const EGPIOPTR_GPPT0: c_uint = 0x00000001;
pub const EGPIOPTR_GPPT1: c_uint = 0x00000002;
pub const EGPIOPTR_GPPT2: c_uint = 0x00000004;
pub const EGPIOPTR_GPPT3: c_uint = 0x00000008;
pub const EGPIOPTR_GPPT4: c_uint = 0x00000010;
pub const EGPIOPTR_GPPT5: c_uint = 0x00000020;
pub const EGPIOPTR_GPPT6: c_uint = 0x00000040;
pub const EGPIOPTR_GPPT7: c_uint = 0x00000080;
pub const EGPIOPTR_GPPT8: c_uint = 0x00000100;

//
// The following defines are for the flags in the extended GPIO pin sticky
// register.
//

pub const EGPIOTR_GPS0: c_uint = 0x00000001;
pub const EGPIOTR_GPS1: c_uint = 0x00000002;
pub const EGPIOTR_GPS2: c_uint = 0x00000004;
pub const EGPIOTR_GPS3: c_uint = 0x00000008;
pub const EGPIOTR_GPS4: c_uint = 0x00000010;
pub const EGPIOTR_GPS5: c_uint = 0x00000020;
pub const EGPIOTR_GPS6: c_uint = 0x00000040;
pub const EGPIOTR_GPS7: c_uint = 0x00000080;
pub const EGPIOTR_GPS8: c_uint = 0x00000100;

//
// The following defines are for the flags in the extended GPIO ping wakeup
// register.
//

pub const EGPIOWR_GPW0: c_uint = 0x00000001;
pub const EGPIOWR_GPW1: c_uint = 0x00000002;
pub const EGPIOWR_GPW2: c_uint = 0x00000004;
pub const EGPIOWR_GPW3: c_uint = 0x00000008;
pub const EGPIOWR_GPW4: c_uint = 0x00000010;
pub const EGPIOWR_GPW5: c_uint = 0x00000020;
pub const EGPIOWR_GPW6: c_uint = 0x00000040;
pub const EGPIOWR_GPW7: c_uint = 0x00000080;
pub const EGPIOWR_GPW8: c_uint = 0x00000100;

//
// The following defines are for the flags in the extended GPIO pin status
// register.
//

pub const EGPIOSR_GPS0: c_uint = 0x00000001;
pub const EGPIOSR_GPS1: c_uint = 0x00000002;
pub const EGPIOSR_GPS2: c_uint = 0x00000004;
pub const EGPIOSR_GPS3: c_uint = 0x00000008;
pub const EGPIOSR_GPS4: c_uint = 0x00000010;
pub const EGPIOSR_GPS5: c_uint = 0x00000020;
pub const EGPIOSR_GPS6: c_uint = 0x00000040;
pub const EGPIOSR_GPS7: c_uint = 0x00000080;
pub const EGPIOSR_GPS8: c_uint = 0x00000100;

//
// The following defines are for the flags in the serial port 6 configuration
// register.
//

pub const SERC6_ASDO2EN: c_uint = 0x00000001;

//
// The following defines are for the flags in the serial port 7 configuration
// register.
//

pub const SERC7_ASDI2EN: c_uint = 0x00000001;
pub const SERC7_POSILB: c_uint = 0x00000002;
pub const SERC7_SIPOLB: c_uint = 0x00000004;
pub const SERC7_SOSILB: c_uint = 0x00000008;
pub const SERC7_SISOLB: c_uint = 0x00000010;

//
// The following defines are for the flags in the serial port AC link
// configuration register.
//

pub const SERACC_CHIP_TYPE_MASK: c_uint = 0x00000001;
pub const SERACC_CHIP_TYPE_1_03: c_uint = 0x00000000;
pub const SERACC_CHIP_TYPE_2_0: c_uint = 0x00000001;
pub const SERACC_TWO_CODECS: c_uint = 0x00000002;
pub const SERACC_MDM: c_uint = 0x00000004;
pub const SERACC_HSP: c_uint = 0x00000008;
pub const SERACC_ODT: c_uint = 0x00000010 /* only CS4630 */;

//
// The following defines are for the flags in the AC97 control register 2.
//

pub const ACCTL2_RSTN: c_uint = 0x00000001;
pub const ACCTL2_ESYN: c_uint = 0x00000002;
pub const ACCTL2_VFRM: c_uint = 0x00000004;
pub const ACCTL2_DCV: c_uint = 0x00000008;
pub const ACCTL2_CRW: c_uint = 0x00000010;
pub const ACCTL2_ASYN: c_uint = 0x00000020;

//
// The following defines are for the flags in the AC97 status register 2.
//

pub const ACSTS2_CRDY: c_uint = 0x00000001;
pub const ACSTS2_VSTS: c_uint = 0x00000002;

//
// The following defines are for the flags in the AC97 output slot valid
// register 2.
//

pub const ACOSV2_SLV3: c_uint = 0x00000001;
pub const ACOSV2_SLV4: c_uint = 0x00000002;
pub const ACOSV2_SLV5: c_uint = 0x00000004;
pub const ACOSV2_SLV6: c_uint = 0x00000008;
pub const ACOSV2_SLV7: c_uint = 0x00000010;
pub const ACOSV2_SLV8: c_uint = 0x00000020;
pub const ACOSV2_SLV9: c_uint = 0x00000040;
pub const ACOSV2_SLV10: c_uint = 0x00000080;
pub const ACOSV2_SLV11: c_uint = 0x00000100;
pub const ACOSV2_SLV12: c_uint = 0x00000200;

//
// The following defines are for the flags in the AC97 command address
// register 2.
//

pub const ACCAD2_CI_MASK: c_uint = 0x0000007F;
pub const ACCAD2_CI_SHIFT: c_int = 0;

//
// The following defines are for the flags in the AC97 command data register
// 2.
//

pub const ACCDA2_CD_MASK: c_uint = 0x0000FFFF;
pub const ACCDA2_CD_SHIFT: c_int = 0;

//
// The following defines are for the flags in the AC97 input slot valid
// register 2.
//

pub const ACISV2_ISV3: c_uint = 0x00000001;
pub const ACISV2_ISV4: c_uint = 0x00000002;
pub const ACISV2_ISV5: c_uint = 0x00000004;
pub const ACISV2_ISV6: c_uint = 0x00000008;
pub const ACISV2_ISV7: c_uint = 0x00000010;
pub const ACISV2_ISV8: c_uint = 0x00000020;
pub const ACISV2_ISV9: c_uint = 0x00000040;
pub const ACISV2_ISV10: c_uint = 0x00000080;
pub const ACISV2_ISV11: c_uint = 0x00000100;
pub const ACISV2_ISV12: c_uint = 0x00000200;

//
// The following defines are for the flags in the AC97 status address
// register 2.
//

pub const ACSAD2_SI_MASK: c_uint = 0x0000007F;
pub const ACSAD2_SI_SHIFT: c_int = 0;

//
// The following defines are for the flags in the AC97 status data register 2.
//

pub const ACSDA2_SD_MASK: c_uint = 0x0000FFFF;
pub const ACSDA2_SD_SHIFT: c_int = 0;

//
// The following defines are for the flags in the I/O trap address and control
// registers (all 12).
//

pub const IOTAC_SA_MASK: c_uint = 0x0000FFFF;
pub const IOTAC_MSK_MASK: c_uint = 0x000F0000;
pub const IOTAC_IODC_MASK: c_uint = 0x06000000;
pub const IOTAC_IODC_16_BIT: c_uint = 0x00000000;
pub const IOTAC_IODC_10_BIT: c_uint = 0x02000000;
pub const IOTAC_IODC_12_BIT: c_uint = 0x04000000;
pub const IOTAC_WSPI: c_uint = 0x08000000;
pub const IOTAC_RSPI: c_uint = 0x10000000;
pub const IOTAC_WSE: c_uint = 0x20000000;
pub const IOTAC_WE: c_uint = 0x40000000;
pub const IOTAC_RE: c_uint = 0x80000000;
pub const IOTAC_SA_SHIFT: c_int = 0;
pub const IOTAC_MSK_SHIFT: c_int = 16;

//
// The following defines are for the flags in the I/O trap fast read registers
// (all 8).
//

pub const IOTFR_D_MASK: c_uint = 0x0000FFFF;
pub const IOTFR_A_MASK: c_uint = 0x000F0000;
pub const IOTFR_R_MASK: c_uint = 0x0F000000;
pub const IOTFR_ALL: c_uint = 0x40000000;
pub const IOTFR_VL: c_uint = 0x80000000;
pub const IOTFR_D_SHIFT: c_int = 0;
pub const IOTFR_A_SHIFT: c_int = 16;
pub const IOTFR_R_SHIFT: c_int = 24;

//
// The following defines are for the flags in the I/O trap FIFO register.
//

pub const IOTFIFO_BA_MASK: c_uint = 0x00003FFF;
pub const IOTFIFO_S_MASK: c_uint = 0x00FF0000;
pub const IOTFIFO_OF: c_uint = 0x40000000;
pub const IOTFIFO_SPIOF: c_uint = 0x80000000;
pub const IOTFIFO_BA_SHIFT: c_int = 0;
pub const IOTFIFO_S_SHIFT: c_int = 16;

//
// The following defines are for the flags in the I/O trap retry read data
// register.
//

pub const IOTRRD_D_MASK: c_uint = 0x0000FFFF;
pub const IOTRRD_RDV: c_uint = 0x80000000;
pub const IOTRRD_D_SHIFT: c_int = 0;

//
// The following defines are for the flags in the I/O trap FIFO pointer
// register.
//

pub const IOTFP_CA_MASK: c_uint = 0x00003FFF;
pub const IOTFP_PA_MASK: c_uint = 0x3FFF0000;
pub const IOTFP_CA_SHIFT: c_int = 0;
pub const IOTFP_PA_SHIFT: c_int = 16;

//
// The following defines are for the flags in the I/O trap control register.
//

pub const IOTCR_ITD: c_uint = 0x00000001;
pub const IOTCR_HRV: c_uint = 0x00000002;
pub const IOTCR_SRV: c_uint = 0x00000004;
pub const IOTCR_DTI: c_uint = 0x00000008;
pub const IOTCR_DFI: c_uint = 0x00000010;
pub const IOTCR_DDP: c_uint = 0x00000020;
pub const IOTCR_JTE: c_uint = 0x00000040;
pub const IOTCR_PPE: c_uint = 0x00000080;

//
// The following defines are for the flags in the direct PCI data register.
//

pub const DPCID_D_MASK: c_uint = 0xFFFFFFFF;
pub const DPCID_D_SHIFT: c_int = 0;

//
// The following defines are for the flags in the direct PCI address register.
//

pub const DPCIA_A_MASK: c_uint = 0xFFFFFFFF;
pub const DPCIA_A_SHIFT: c_int = 0;

//
// The following defines are for the flags in the direct PCI command register.
//

pub const DPCIC_C_MASK: c_uint = 0x0000000F;
pub const DPCIC_C_IOREAD: c_uint = 0x00000002;
pub const DPCIC_C_IOWRITE: c_uint = 0x00000003;
pub const DPCIC_BE_MASK: c_uint = 0x000000F0;

//
// The following defines are for the flags in the PC/PCI request register.
//

pub const PCPCIR_RDC_MASK: c_uint = 0x00000007;
pub const PCPCIR_C_MASK: c_uint = 0x00007000;
pub const PCPCIR_REQ: c_uint = 0x00008000;
pub const PCPCIR_RDC_SHIFT: c_int = 0;
pub const PCPCIR_C_SHIFT: c_int = 12;

//
// The following defines are for the flags in the PC/PCI grant register.
//

pub const PCPCIG_GDC_MASK: c_uint = 0x00000007;
pub const PCPCIG_VL: c_uint = 0x00008000;
pub const PCPCIG_GDC_SHIFT: c_int = 0;

//
// The following defines are for the flags in the PC/PCI master enable
// register.
//

pub const PCPCIEN_EN: c_uint = 0x00000001;

//
// The following defines are for the flags in the extended PCI power
// management control register.
//

pub const EPCIPMC_GWU: c_uint = 0x00000001;
pub const EPCIPMC_FSPC: c_uint = 0x00000002;

//
// The following defines are for the flags in the SP control register.
//
pub const SPCR_RUN: c_uint = 0x00000001;
pub const SPCR_STPFR: c_uint = 0x00000002;
pub const SPCR_RUNFR: c_uint = 0x00000004;
pub const SPCR_TICK: c_uint = 0x00000008;
pub const SPCR_DRQEN: c_uint = 0x00000020;
pub const SPCR_RSTSP: c_uint = 0x00000040;
pub const SPCR_OREN: c_uint = 0x00000080;

pub const SPCR_PCIINT: c_uint = 0x00000100;
pub const SPCR_OINTD: c_uint = 0x00000200;
pub const SPCR_CRE: c_uint = 0x00008000;

//
// The following defines are for the flags in the debug index register.
//
pub const DREG_REGID_MASK: c_uint = 0x0000007F;
pub const DREG_DEBUG: c_uint = 0x00000080;
pub const DREG_RGBK_MASK: c_uint = 0x00000700;
pub const DREG_TRAP: c_uint = 0x00000800;

pub const DREG_TRAPX: c_uint = 0x00001000;

pub const DREG_REGID_SHIFT: c_int = 0;
pub const DREG_RGBK_SHIFT: c_int = 8;
pub const DREG_RGBK_REGID_MASK: c_uint = 0x0000077F;
pub const DREG_REGID_R0: c_uint = 0x00000010;
pub const DREG_REGID_R1: c_uint = 0x00000011;
pub const DREG_REGID_R2: c_uint = 0x00000012;
pub const DREG_REGID_R3: c_uint = 0x00000013;
pub const DREG_REGID_R4: c_uint = 0x00000014;
pub const DREG_REGID_R5: c_uint = 0x00000015;
pub const DREG_REGID_R6: c_uint = 0x00000016;
pub const DREG_REGID_R7: c_uint = 0x00000017;
pub const DREG_REGID_R8: c_uint = 0x00000018;
pub const DREG_REGID_R9: c_uint = 0x00000019;
pub const DREG_REGID_RA: c_uint = 0x0000001A;
pub const DREG_REGID_RB: c_uint = 0x0000001B;
pub const DREG_REGID_RC: c_uint = 0x0000001C;
pub const DREG_REGID_RD: c_uint = 0x0000001D;
pub const DREG_REGID_RE: c_uint = 0x0000001E;
pub const DREG_REGID_RF: c_uint = 0x0000001F;
pub const DREG_REGID_RA_BUS_LOW: c_uint = 0x00000020;
pub const DREG_REGID_RA_BUS_HIGH: c_uint = 0x00000038;
pub const DREG_REGID_YBUS_LOW: c_uint = 0x00000050;
pub const DREG_REGID_YBUS_HIGH: c_uint = 0x00000058;
pub const DREG_REGID_TRAP_0: c_uint = 0x00000100;
pub const DREG_REGID_TRAP_1: c_uint = 0x00000101;
pub const DREG_REGID_TRAP_2: c_uint = 0x00000102;
pub const DREG_REGID_TRAP_3: c_uint = 0x00000103;
pub const DREG_REGID_TRAP_4: c_uint = 0x00000104;
pub const DREG_REGID_TRAP_5: c_uint = 0x00000105;
pub const DREG_REGID_TRAP_6: c_uint = 0x00000106;
pub const DREG_REGID_TRAP_7: c_uint = 0x00000107;
pub const DREG_REGID_INDIRECT_ADDRESS: c_uint = 0x0000010E;
pub const DREG_REGID_TOP_OF_STACK: c_uint = 0x0000010F;

pub const DREG_REGID_TRAP_8: c_uint = 0x00000110;
pub const DREG_REGID_TRAP_9: c_uint = 0x00000111;
pub const DREG_REGID_TRAP_10: c_uint = 0x00000112;
pub const DREG_REGID_TRAP_11: c_uint = 0x00000113;
pub const DREG_REGID_TRAP_12: c_uint = 0x00000114;
pub const DREG_REGID_TRAP_13: c_uint = 0x00000115;
pub const DREG_REGID_TRAP_14: c_uint = 0x00000116;
pub const DREG_REGID_TRAP_15: c_uint = 0x00000117;
pub const DREG_REGID_TRAP_16: c_uint = 0x00000118;
pub const DREG_REGID_TRAP_17: c_uint = 0x00000119;
pub const DREG_REGID_TRAP_18: c_uint = 0x0000011A;
pub const DREG_REGID_TRAP_19: c_uint = 0x0000011B;
pub const DREG_REGID_TRAP_20: c_uint = 0x0000011C;
pub const DREG_REGID_TRAP_21: c_uint = 0x0000011D;
pub const DREG_REGID_TRAP_22: c_uint = 0x0000011E;
pub const DREG_REGID_TRAP_23: c_uint = 0x0000011F;

pub const DREG_REGID_RSA0_LOW: c_uint = 0x00000200;
pub const DREG_REGID_RSA0_HIGH: c_uint = 0x00000201;
pub const DREG_REGID_RSA1_LOW: c_uint = 0x00000202;
pub const DREG_REGID_RSA1_HIGH: c_uint = 0x00000203;
pub const DREG_REGID_RSA2: c_uint = 0x00000204;
pub const DREG_REGID_RSA3: c_uint = 0x00000205;
pub const DREG_REGID_RSI0_LOW: c_uint = 0x00000206;
pub const DREG_REGID_RSI0_HIGH: c_uint = 0x00000207;
pub const DREG_REGID_RSI1: c_uint = 0x00000208;
pub const DREG_REGID_RSI2: c_uint = 0x00000209;
pub const DREG_REGID_SAGUSTATUS: c_uint = 0x0000020A;
pub const DREG_REGID_RSCONFIG01_LOW: c_uint = 0x0000020B;
pub const DREG_REGID_RSCONFIG01_HIGH: c_uint = 0x0000020C;
pub const DREG_REGID_RSCONFIG23_LOW: c_uint = 0x0000020D;
pub const DREG_REGID_RSCONFIG23_HIGH: c_uint = 0x0000020E;
pub const DREG_REGID_RSDMA01E: c_uint = 0x0000020F;
pub const DREG_REGID_RSDMA23E: c_uint = 0x00000210;
pub const DREG_REGID_RSD0_LOW: c_uint = 0x00000211;
pub const DREG_REGID_RSD0_HIGH: c_uint = 0x00000212;
pub const DREG_REGID_RSD1_LOW: c_uint = 0x00000213;
pub const DREG_REGID_RSD1_HIGH: c_uint = 0x00000214;
pub const DREG_REGID_RSD2_LOW: c_uint = 0x00000215;
pub const DREG_REGID_RSD2_HIGH: c_uint = 0x00000216;
pub const DREG_REGID_RSD3_LOW: c_uint = 0x00000217;
pub const DREG_REGID_RSD3_HIGH: c_uint = 0x00000218;
pub const DREG_REGID_SRAR_HIGH: c_uint = 0x0000021A;
pub const DREG_REGID_SRAR_LOW: c_uint = 0x0000021B;
pub const DREG_REGID_DMA_STATE: c_uint = 0x0000021C;
pub const DREG_REGID_CURRENT_DMA_STREAM: c_uint = 0x0000021D;
pub const DREG_REGID_NEXT_DMA_STREAM: c_uint = 0x0000021E;
pub const DREG_REGID_CPU_STATUS: c_uint = 0x00000300;
pub const DREG_REGID_MAC_MODE: c_uint = 0x00000301;
pub const DREG_REGID_STACK_AND_REPEAT: c_uint = 0x00000302;
pub const DREG_REGID_INDEX0: c_uint = 0x00000304;
pub const DREG_REGID_INDEX1: c_uint = 0x00000305;
pub const DREG_REGID_DMA_STATE_0_3: c_uint = 0x00000400;
pub const DREG_REGID_DMA_STATE_4_7: c_uint = 0x00000404;
pub const DREG_REGID_DMA_STATE_8_11: c_uint = 0x00000408;
pub const DREG_REGID_DMA_STATE_12_15: c_uint = 0x0000040C;
pub const DREG_REGID_DMA_STATE_16_19: c_uint = 0x00000410;
pub const DREG_REGID_DMA_STATE_20_23: c_uint = 0x00000414;
pub const DREG_REGID_DMA_STATE_24_27: c_uint = 0x00000418;
pub const DREG_REGID_DMA_STATE_28_31: c_uint = 0x0000041C;
pub const DREG_REGID_DMA_STATE_32_35: c_uint = 0x00000420;
pub const DREG_REGID_DMA_STATE_36_39: c_uint = 0x00000424;
pub const DREG_REGID_DMA_STATE_40_43: c_uint = 0x00000428;
pub const DREG_REGID_DMA_STATE_44_47: c_uint = 0x0000042C;
pub const DREG_REGID_DMA_STATE_48_51: c_uint = 0x00000430;
pub const DREG_REGID_DMA_STATE_52_55: c_uint = 0x00000434;
pub const DREG_REGID_DMA_STATE_56_59: c_uint = 0x00000438;
pub const DREG_REGID_DMA_STATE_60_63: c_uint = 0x0000043C;
pub const DREG_REGID_DMA_STATE_64_67: c_uint = 0x00000440;
pub const DREG_REGID_DMA_STATE_68_71: c_uint = 0x00000444;
pub const DREG_REGID_DMA_STATE_72_75: c_uint = 0x00000448;
pub const DREG_REGID_DMA_STATE_76_79: c_uint = 0x0000044C;
pub const DREG_REGID_DMA_STATE_80_83: c_uint = 0x00000450;
pub const DREG_REGID_DMA_STATE_84_87: c_uint = 0x00000454;
pub const DREG_REGID_DMA_STATE_88_91: c_uint = 0x00000458;
pub const DREG_REGID_DMA_STATE_92_95: c_uint = 0x0000045C;
pub const DREG_REGID_TRAP_SELECT: c_uint = 0x00000500;
pub const DREG_REGID_TRAP_WRITE_0: c_uint = 0x00000500;
pub const DREG_REGID_TRAP_WRITE_1: c_uint = 0x00000501;
pub const DREG_REGID_TRAP_WRITE_2: c_uint = 0x00000502;
pub const DREG_REGID_TRAP_WRITE_3: c_uint = 0x00000503;
pub const DREG_REGID_TRAP_WRITE_4: c_uint = 0x00000504;
pub const DREG_REGID_TRAP_WRITE_5: c_uint = 0x00000505;
pub const DREG_REGID_TRAP_WRITE_6: c_uint = 0x00000506;
pub const DREG_REGID_TRAP_WRITE_7: c_uint = 0x00000507;

pub const DREG_REGID_TRAP_WRITE_8: c_uint = 0x00000510;
pub const DREG_REGID_TRAP_WRITE_9: c_uint = 0x00000511;
pub const DREG_REGID_TRAP_WRITE_10: c_uint = 0x00000512;
pub const DREG_REGID_TRAP_WRITE_11: c_uint = 0x00000513;
pub const DREG_REGID_TRAP_WRITE_12: c_uint = 0x00000514;
pub const DREG_REGID_TRAP_WRITE_13: c_uint = 0x00000515;
pub const DREG_REGID_TRAP_WRITE_14: c_uint = 0x00000516;
pub const DREG_REGID_TRAP_WRITE_15: c_uint = 0x00000517;
pub const DREG_REGID_TRAP_WRITE_16: c_uint = 0x00000518;
pub const DREG_REGID_TRAP_WRITE_17: c_uint = 0x00000519;
pub const DREG_REGID_TRAP_WRITE_18: c_uint = 0x0000051A;
pub const DREG_REGID_TRAP_WRITE_19: c_uint = 0x0000051B;
pub const DREG_REGID_TRAP_WRITE_20: c_uint = 0x0000051C;
pub const DREG_REGID_TRAP_WRITE_21: c_uint = 0x0000051D;
pub const DREG_REGID_TRAP_WRITE_22: c_uint = 0x0000051E;
pub const DREG_REGID_TRAP_WRITE_23: c_uint = 0x0000051F;

pub const DREG_REGID_MAC0_ACC0_LOW: c_uint = 0x00000600;
pub const DREG_REGID_MAC0_ACC1_LOW: c_uint = 0x00000601;
pub const DREG_REGID_MAC0_ACC2_LOW: c_uint = 0x00000602;
pub const DREG_REGID_MAC0_ACC3_LOW: c_uint = 0x00000603;
pub const DREG_REGID_MAC1_ACC0_LOW: c_uint = 0x00000604;
pub const DREG_REGID_MAC1_ACC1_LOW: c_uint = 0x00000605;
pub const DREG_REGID_MAC1_ACC2_LOW: c_uint = 0x00000606;
pub const DREG_REGID_MAC1_ACC3_LOW: c_uint = 0x00000607;
pub const DREG_REGID_MAC0_ACC0_MID: c_uint = 0x00000608;
pub const DREG_REGID_MAC0_ACC1_MID: c_uint = 0x00000609;
pub const DREG_REGID_MAC0_ACC2_MID: c_uint = 0x0000060A;
pub const DREG_REGID_MAC0_ACC3_MID: c_uint = 0x0000060B;
pub const DREG_REGID_MAC1_ACC0_MID: c_uint = 0x0000060C;
pub const DREG_REGID_MAC1_ACC1_MID: c_uint = 0x0000060D;
pub const DREG_REGID_MAC1_ACC2_MID: c_uint = 0x0000060E;
pub const DREG_REGID_MAC1_ACC3_MID: c_uint = 0x0000060F;
pub const DREG_REGID_MAC0_ACC0_HIGH: c_uint = 0x00000610;
pub const DREG_REGID_MAC0_ACC1_HIGH: c_uint = 0x00000611;
pub const DREG_REGID_MAC0_ACC2_HIGH: c_uint = 0x00000612;
pub const DREG_REGID_MAC0_ACC3_HIGH: c_uint = 0x00000613;
pub const DREG_REGID_MAC1_ACC0_HIGH: c_uint = 0x00000614;
pub const DREG_REGID_MAC1_ACC1_HIGH: c_uint = 0x00000615;
pub const DREG_REGID_MAC1_ACC2_HIGH: c_uint = 0x00000616;
pub const DREG_REGID_MAC1_ACC3_HIGH: c_uint = 0x00000617;
pub const DREG_REGID_RSHOUT_LOW: c_uint = 0x00000620;
pub const DREG_REGID_RSHOUT_MID: c_uint = 0x00000628;
pub const DREG_REGID_RSHOUT_HIGH: c_uint = 0x00000630;
//
// The following defines are for the flags in the DMA stream requestor write
//
pub const DSRWP_DSR_MASK: c_uint = 0x0000000F;
pub const DSRWP_DSR_BG_RQ: c_uint = 0x00000001;
pub const DSRWP_DSR_PRIORITY_MASK: c_uint = 0x00000006;
pub const DSRWP_DSR_PRIORITY_0: c_uint = 0x00000000;
pub const DSRWP_DSR_PRIORITY_1: c_uint = 0x00000002;
pub const DSRWP_DSR_PRIORITY_2: c_uint = 0x00000004;
pub const DSRWP_DSR_PRIORITY_3: c_uint = 0x00000006;
pub const DSRWP_DSR_RQ_PENDING: c_uint = 0x00000008;
//
// The following defines are for the flags in the trap write port register.
//
pub const TWPR_TW_MASK: c_uint = 0x0000FFFF;
pub const TWPR_TW_SHIFT: c_int = 0;
//
// The following defines are for the flags in the stack pointer write
// register.
//
pub const SPWR_STKP_MASK: c_uint = 0x0000000F;
pub const SPWR_STKP_SHIFT: c_int = 0;
//
// The following defines are for the flags in the SP interrupt register.
//
pub const SPIR_FRI: c_uint = 0x00000001;
pub const SPIR_DOI: c_uint = 0x00000002;
pub const SPIR_GPI2: c_uint = 0x00000004;
pub const SPIR_GPI3: c_uint = 0x00000008;
pub const SPIR_IP0: c_uint = 0x00000010;
pub const SPIR_IP1: c_uint = 0x00000020;
pub const SPIR_IP2: c_uint = 0x00000040;
pub const SPIR_IP3: c_uint = 0x00000080;
//
// The following defines are for the flags in the functional group 1 register.
//
pub const FGR1_F1S_MASK: c_uint = 0x0000FFFF;
pub const FGR1_F1S_SHIFT: c_int = 0;
//
// The following defines are for the flags in the SP clock status register.
//
pub const SPCS_FRI: c_uint = 0x00000001;
pub const SPCS_DOI: c_uint = 0x00000002;
pub const SPCS_GPI2: c_uint = 0x00000004;
pub const SPCS_GPI3: c_uint = 0x00000008;
pub const SPCS_IP0: c_uint = 0x00000010;
pub const SPCS_IP1: c_uint = 0x00000020;
pub const SPCS_IP2: c_uint = 0x00000040;
pub const SPCS_IP3: c_uint = 0x00000080;
pub const SPCS_SPRUN: c_uint = 0x00000100;
pub const SPCS_SLEEP: c_uint = 0x00000200;
pub const SPCS_FG: c_uint = 0x00000400;
pub const SPCS_ORUN: c_uint = 0x00000800;
pub const SPCS_IRQ: c_uint = 0x00001000;
pub const SPCS_FGN_MASK: c_uint = 0x0000E000;
pub const SPCS_FGN_SHIFT: c_int = 13;
//
// The following defines are for the flags in the SP DMA requestor status
// register.
//
pub const SDSR_DCS_MASK: c_uint = 0x000000FF;
pub const SDSR_DCS_SHIFT: c_int = 0;
pub const SDSR_DCS_NONE: c_uint = 0x00000007;
//
// The following defines are for the flags in the frame timer register.
//
pub const FRMT_FTV_MASK: c_uint = 0x0000FFFF;
pub const FRMT_FTV_SHIFT: c_int = 0;
//
// The following defines are for the flags in the frame timer current count
// register.
//
pub const FRCC_FCC_MASK: c_uint = 0x0000FFFF;
pub const FRCC_FCC_SHIFT: c_int = 0;
//
// The following defines are for the flags in the frame timer save count
// register.
//
pub const FRSC_FCS_MASK: c_uint = 0x0000FFFF;
pub const FRSC_FCS_SHIFT: c_int = 0;
//
// The following define the various flags stored in the scatter/gather
// descriptors.
//
pub const DMA_SG_NEXT_ENTRY_MASK: c_uint = 0x00000FF8;
pub const DMA_SG_SAMPLE_END_MASK: c_uint = 0x0FFF0000;
pub const DMA_SG_SAMPLE_END_FLAG: c_uint = 0x10000000;
pub const DMA_SG_LOOP_END_FLAG: c_uint = 0x20000000;
pub const DMA_SG_SIGNAL_END_FLAG: c_uint = 0x40000000;
pub const DMA_SG_SIGNAL_PAGE_FLAG: c_uint = 0x80000000;
pub const DMA_SG_NEXT_ENTRY_SHIFT: c_int = 3;
pub const DMA_SG_SAMPLE_END_SHIFT: c_int = 16;
//
// The following define the offsets of the fields within the on-chip generic
// DMA requestor.
//
pub const DMA_RQ_CONTROL1: c_uint = 0x00000000;
pub const DMA_RQ_CONTROL2: c_uint = 0x00000004;
pub const DMA_RQ_SOURCE_ADDR: c_uint = 0x00000008;
pub const DMA_RQ_DESTINATION_ADDR: c_uint = 0x0000000C;
pub const DMA_RQ_NEXT_PAGE_ADDR: c_uint = 0x00000010;
pub const DMA_RQ_NEXT_PAGE_SGDESC: c_uint = 0x00000014;
pub const DMA_RQ_LOOP_START_ADDR: c_uint = 0x00000018;
pub const DMA_RQ_POST_LOOP_ADDR: c_uint = 0x0000001C;
pub const DMA_RQ_PAGE_MAP_ADDR: c_uint = 0x00000020;
//
// The following defines are for the flags in the first control word of the
// on-chip generic DMA requestor.
//
pub const DMA_RQ_C1_COUNT_MASK: c_uint = 0x000003FF;
pub const DMA_RQ_C1_DESTINATION_SCATTER: c_uint = 0x00001000;
pub const DMA_RQ_C1_SOURCE_GATHER: c_uint = 0x00002000;
pub const DMA_RQ_C1_DONE_FLAG: c_uint = 0x00004000;
pub const DMA_RQ_C1_OPTIMIZE_STATE: c_uint = 0x00008000;
pub const DMA_RQ_C1_SAMPLE_END_STATE_MASK: c_uint = 0x00030000;
pub const DMA_RQ_C1_FULL_PAGE: c_uint = 0x00000000;
pub const DMA_RQ_C1_BEFORE_SAMPLE_END: c_uint = 0x00010000;
pub const DMA_RQ_C1_PAGE_MAP_ERROR: c_uint = 0x00020000;
pub const DMA_RQ_C1_AT_SAMPLE_END: c_uint = 0x00030000;
pub const DMA_RQ_C1_LOOP_END_STATE_MASK: c_uint = 0x000C0000;
pub const DMA_RQ_C1_NOT_LOOP_END: c_uint = 0x00000000;
pub const DMA_RQ_C1_BEFORE_LOOP_END: c_uint = 0x00040000;
pub const DMA_RQ_C1_2PAGE_LOOP_BEGIN: c_uint = 0x00080000;
pub const DMA_RQ_C1_LOOP_BEGIN: c_uint = 0x000C0000;
pub const DMA_RQ_C1_PAGE_MAP_MASK: c_uint = 0x00300000;
pub const DMA_RQ_C1_PM_NONE_PENDING: c_uint = 0x00000000;
pub const DMA_RQ_C1_PM_NEXT_PENDING: c_uint = 0x00100000;
pub const DMA_RQ_C1_PM_RESERVED: c_uint = 0x00200000;
pub const DMA_RQ_C1_PM_LOOP_NEXT_PENDING: c_uint = 0x00300000;
pub const DMA_RQ_C1_WRITEBACK_DEST_FLAG: c_uint = 0x00400000;
pub const DMA_RQ_C1_WRITEBACK_SRC_FLAG: c_uint = 0x00800000;
pub const DMA_RQ_C1_DEST_SIZE_MASK: c_uint = 0x07000000;
pub const DMA_RQ_C1_DEST_LINEAR: c_uint = 0x00000000;
pub const DMA_RQ_C1_DEST_MOD16: c_uint = 0x01000000;
pub const DMA_RQ_C1_DEST_MOD32: c_uint = 0x02000000;
pub const DMA_RQ_C1_DEST_MOD64: c_uint = 0x03000000;
pub const DMA_RQ_C1_DEST_MOD128: c_uint = 0x04000000;
pub const DMA_RQ_C1_DEST_MOD256: c_uint = 0x05000000;
pub const DMA_RQ_C1_DEST_MOD512: c_uint = 0x06000000;
pub const DMA_RQ_C1_DEST_MOD1024: c_uint = 0x07000000;
pub const DMA_RQ_C1_DEST_ON_HOST: c_uint = 0x08000000;
pub const DMA_RQ_C1_SOURCE_SIZE_MASK: c_uint = 0x70000000;
pub const DMA_RQ_C1_SOURCE_LINEAR: c_uint = 0x00000000;
pub const DMA_RQ_C1_SOURCE_MOD16: c_uint = 0x10000000;
pub const DMA_RQ_C1_SOURCE_MOD32: c_uint = 0x20000000;
pub const DMA_RQ_C1_SOURCE_MOD64: c_uint = 0x30000000;
pub const DMA_RQ_C1_SOURCE_MOD128: c_uint = 0x40000000;
pub const DMA_RQ_C1_SOURCE_MOD256: c_uint = 0x50000000;
pub const DMA_RQ_C1_SOURCE_MOD512: c_uint = 0x60000000;
pub const DMA_RQ_C1_SOURCE_MOD1024: c_uint = 0x70000000;
pub const DMA_RQ_C1_SOURCE_ON_HOST: c_uint = 0x80000000;
pub const DMA_RQ_C1_COUNT_SHIFT: c_int = 0;
//
// The following defines are for the flags in the second control word of the
// on-chip generic DMA requestor.
//
pub const DMA_RQ_C2_VIRTUAL_CHANNEL_MASK: c_uint = 0x0000003F;
pub const DMA_RQ_C2_VIRTUAL_SIGNAL_MASK: c_uint = 0x00000300;
pub const DMA_RQ_C2_NO_VIRTUAL_SIGNAL: c_uint = 0x00000000;
pub const DMA_RQ_C2_SIGNAL_EVERY_DMA: c_uint = 0x00000100;
pub const DMA_RQ_C2_SIGNAL_SOURCE_PINGPONG: c_uint = 0x00000200;
pub const DMA_RQ_C2_SIGNAL_DEST_PINGPONG: c_uint = 0x00000300;
pub const DMA_RQ_C2_AUDIO_CONVERT_MASK: c_uint = 0x0000F000;
pub const DMA_RQ_C2_AC_NONE: c_uint = 0x00000000;
pub const DMA_RQ_C2_AC_8_TO_16_BIT: c_uint = 0x00001000;
pub const DMA_RQ_C2_AC_MONO_TO_STEREO: c_uint = 0x00002000;
pub const DMA_RQ_C2_AC_ENDIAN_CONVERT: c_uint = 0x00004000;
pub const DMA_RQ_C2_AC_SIGNED_CONVERT: c_uint = 0x00008000;
pub const DMA_RQ_C2_LOOP_END_MASK: c_uint = 0x0FFF0000;
pub const DMA_RQ_C2_LOOP_MASK: c_uint = 0x30000000;
pub const DMA_RQ_C2_NO_LOOP: c_uint = 0x00000000;
pub const DMA_RQ_C2_ONE_PAGE_LOOP: c_uint = 0x10000000;
pub const DMA_RQ_C2_TWO_PAGE_LOOP: c_uint = 0x20000000;
pub const DMA_RQ_C2_MULTI_PAGE_LOOP: c_uint = 0x30000000;
pub const DMA_RQ_C2_SIGNAL_LOOP_BACK: c_uint = 0x40000000;
pub const DMA_RQ_C2_SIGNAL_POST_BEGIN_PAGE: c_uint = 0x80000000;
pub const DMA_RQ_C2_VIRTUAL_CHANNEL_SHIFT: c_int = 0;
pub const DMA_RQ_C2_LOOP_END_SHIFT: c_int = 16;
//
// The following defines are for the flags in the source and destination words
// of the on-chip generic DMA requestor.
//
pub const DMA_RQ_SD_ADDRESS_MASK: c_uint = 0x0000FFFF;
pub const DMA_RQ_SD_MEMORY_ID_MASK: c_uint = 0x000F0000;
pub const DMA_RQ_SD_SP_PARAM_ADDR: c_uint = 0x00000000;
pub const DMA_RQ_SD_SP_SAMPLE_ADDR: c_uint = 0x00010000;
pub const DMA_RQ_SD_SP_PROGRAM_ADDR: c_uint = 0x00020000;
pub const DMA_RQ_SD_SP_DEBUG_ADDR: c_uint = 0x00030000;
pub const DMA_RQ_SD_OMNIMEM_ADDR: c_uint = 0x000E0000;
pub const DMA_RQ_SD_END_FLAG: c_uint = 0x40000000;
pub const DMA_RQ_SD_ERROR_FLAG: c_uint = 0x80000000;
pub const DMA_RQ_SD_ADDRESS_SHIFT: c_int = 0;
//
// The following defines are for the flags in the page map address word of the
// on-chip generic DMA requestor.
//
pub const DMA_RQ_PMA_LOOP_THIRD_PAGE_ENTRY_MASK: c_uint = 0x00000FF8;
pub const DMA_RQ_PMA_PAGE_TABLE_MASK: c_uint = 0xFFFFF000;
pub const DMA_RQ_PMA_LOOP_THIRD_PAGE_ENTRY_SHIFT: c_int = 3;
pub const DMA_RQ_PMA_PAGE_TABLE_SHIFT: c_int = 12;
pub const BA1_VARIDEC_BUF_1: c_uint = 0x000;
pub const BA1_PDTC: c_uint = 0x0c0    /* BA1_PLAY_DMA_TRANSACTION_COUNT_REG */;
pub const BA1_PFIE: c_uint = 0x0c4    /* BA1_PLAY_FORMAT_&_INTERRUPT_ENABLE_REG */;
pub const BA1_PBA: c_uint = 0x0c8    /* BA1_PLAY_BUFFER_ADDRESS */;
pub const BA1_PVOL: c_uint = 0x0f8    /* BA1_PLAY_VOLUME_REG */;
pub const BA1_PSRC: c_uint = 0x288    /* BA1_PLAY_SAMPLE_RATE_CORRECTION_REG */;
pub const BA1_PCTL: c_uint = 0x2a4    /* BA1_PLAY_CONTROL_REG */;
pub const BA1_PPI: c_uint = 0x2b4    /* BA1_PLAY_PHASE_INCREMENT_REG */;
pub const BA1_CCTL: c_uint = 0x064    /* BA1_CAPTURE_CONTROL_REG */;
pub const BA1_CIE: c_uint = 0x104    /* BA1_CAPTURE_INTERRUPT_ENABLE_REG */;
pub const BA1_CBA: c_uint = 0x10c    /* BA1_CAPTURE_BUFFER_ADDRESS */;
pub const BA1_CSRC: c_uint = 0x2c8    /* BA1_CAPTURE_SAMPLE_RATE_CORRECTION_REG */;
pub const BA1_CCI: c_uint = 0x2d8    /* BA1_CAPTURE_COEFFICIENT_INCREMENT_REG */;
pub const BA1_CD: c_uint = 0x2e0    /* BA1_CAPTURE_DELAY_REG */;
pub const BA1_CPI: c_uint = 0x2f4    /* BA1_CAPTURE_PHASE_INCREMENT_REG */;
pub const BA1_CVOL: c_uint = 0x2f8    /* BA1_CAPTURE_VOLUME_REG */;
pub const BA1_CFG1: c_uint = 0x134    /* BA1_CAPTURE_FRAME_GROUP_1_REG */;
pub const BA1_CFG2: c_uint = 0x138    /* BA1_CAPTURE_FRAME_GROUP_2_REG */;
pub const BA1_CCST: c_uint = 0x13c    /* BA1_CAPTURE_CONSTANT_REG */;
pub const BA1_CSPB: c_uint = 0x340    /* BA1_CAPTURE_SPB_ADDRESS */;
//

//
pub const SAVE_REG_MAX: c_uint = 0x10;
pub const POWER_DOWN_ALL: c_uint = 0x7f0f;
// maxinum number of AC97 codecs connected, AC97 2.0 defined 4
pub const MAX_NR_AC97: c_int = 4;
pub const CS46XX_PRIMARY_CODEC_INDEX: c_int = 0;
pub const CS46XX_SECONDARY_CODEC_INDEX: c_int = 1;
pub const CS46XX_SECONDARY_CODEC_OFFSET: c_uint = 0x80;
pub const CS46XX_DSP_CAPTURE_CHANNEL: c_int = 1;
// capture
pub const CS46XX_DSP_CAPTURE_CHANNEL: c_int = 1;
// mixer
pub const CS46XX_MIXER_SPDIF_INPUT_ELEMENT: c_int = 1;
pub const CS46XX_MIXER_SPDIF_OUTPUT_ELEMENT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_cs46xx_pcm {
    pub hw_buf: snd_dma_buffer,
    pub ctl: c_uint,
    pub /: *mut *mut unsigned int shift; / Shift count to trasform frames in bytes,
    pub pcm_rec: snd_pcm_indirect,
    pub substream: *mut snd_pcm_substream,
    pub pcm_channel: *mut *mut dsp_pcm_channel_descriptor,
    pub /: *mut *mut int pcm_channel_id; / Fron Rear, Center Lfe ...,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_cs46xx_region {
    pub name: [c_char; 24],
    pub base: c_ulong,
    pub remap_addr: *mut void __iomem,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_cs46xx {
    pub irq: c_int,
    pub ba0_addr: c_ulong,
    pub ba1_addr: c_ulong,
    pub ba0: snd_cs46xx_region,
    pub data0: snd_cs46xx_region,
    pub data1: snd_cs46xx_region,
    pub pmem: snd_cs46xx_region,
    pub reg: snd_cs46xx_region,
    pub name: },
    pub idx: [snd_cs46xx_region; 5],
    pub region: },
    pub mode: c_uint,
    pub hw_buf: snd_dma_buffer,
    pub ctl: c_uint,
    pub /: *mut *mut unsigned int shift; / Shift count to trasform frames in bytes,
    pub pcm_rec: snd_pcm_indirect,
    pub substream: *mut snd_pcm_substream,
    pub capt: },
    pub nr_ac97_codecs: c_int,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: [*mut snd_ac97; MAX_NR_AC97],
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub rmidi: *mut snd_rawmidi,
    pub midi_input: *mut snd_rawmidi_substream,
    pub midi_output: *mut snd_rawmidi_substream,
    pub reg_lock: spinlock_t,
    pub midcr: c_uint,
    pub uartm: c_uint,
    pub amplifier: c_int,
    pub int): *mut *mut *mut void (amplifier_ctrl)(struct snd_cs46xx ,,
    pub int): *mut *mut *mut void (active_ctrl)(struct snd_cs46xx ,,
    pub ): *mut *mut void (mixer_init)(struct snd_cs46xx,
    pub acpi_port: c_int,
    pub /: *mut *mut *mut snd_kcontrol eapd_switch; / for amplifier hack,
    pub /: *mut *mut int accept_valid; / accept mmap valid (for OSS),
    pub in_suspend: c_int,
    pub gameport: *mut gameport,

    pub spos_mutex: mutex,
    pub dsp_spos_instance: *mut *mut dsp_spos_instance,
    pub pcm_rear: *mut snd_pcm,
    pub pcm_center_lfe: *mut snd_pcm,
    pub pcm_iec958: *mut snd_pcm,
pub const CS46XX_DSP_MODULES: c_int = 5;
    pub modules: [*mut dsp_module_desc; CS46XX_DSP_MODULES],
    pub playback_pcm: *mut snd_cs46xx_pcm,
    pub play_ctl: c_uint,
    pub ba1: *mut ba1_struct,

    pub saved_regs: *mut u32,

}

extern "C" {
    pub fn snd_cs46xx_pcm(chip: *mut snd_cs46xx, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_pcm_rear(chip: *mut snd_cs46xx, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_pcm_iec958(chip: *mut snd_cs46xx, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_pcm_center_lfe(chip: *mut snd_cs46xx, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_mixer(chip: *mut snd_cs46xx, spdif_device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_midi(chip: *mut snd_cs46xx, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_start_dsp(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_gameport(chip: *mut snd_cs46xx) -> c_int;
}
