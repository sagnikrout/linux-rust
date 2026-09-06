//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/44x/pci.h
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


//
// PCI / PCI-X / PCI-Express support for 4xx parts
//
// Copyright 2007 Ben. Herrenschmidt <benh@kernel.crashing.org>, IBM Corp.
//
// Bits and pieces extracted from arch/ppc support by
//
// Matt Porter <mporter@kernel.crashing.org>
//
// Copyright 2002-2005 MontaVista Software Inc.
//
// 4xx PCI-X bridge register definitions
//
pub const PCIX0_VENDID: c_uint = 0x000;
pub const PCIX0_DEVID: c_uint = 0x002;
pub const PCIX0_COMMAND: c_uint = 0x004;
pub const PCIX0_STATUS: c_uint = 0x006;
pub const PCIX0_REVID: c_uint = 0x008;
pub const PCIX0_CLS: c_uint = 0x009;
pub const PCIX0_CACHELS: c_uint = 0x00c;
pub const PCIX0_LATTIM: c_uint = 0x00d;
pub const PCIX0_HDTYPE: c_uint = 0x00e;
pub const PCIX0_BIST: c_uint = 0x00f;
pub const PCIX0_BAR0L: c_uint = 0x010;
pub const PCIX0_BAR0H: c_uint = 0x014;
pub const PCIX0_BAR1: c_uint = 0x018;
pub const PCIX0_BAR2L: c_uint = 0x01c;
pub const PCIX0_BAR2H: c_uint = 0x020;
pub const PCIX0_BAR3: c_uint = 0x024;
pub const PCIX0_CISPTR: c_uint = 0x028;
pub const PCIX0_SBSYSVID: c_uint = 0x02c;
pub const PCIX0_SBSYSID: c_uint = 0x02e;
pub const PCIX0_EROMBA: c_uint = 0x030;
pub const PCIX0_CAP: c_uint = 0x034;
pub const PCIX0_RES0: c_uint = 0x035;
pub const PCIX0_RES1: c_uint = 0x036;
pub const PCIX0_RES2: c_uint = 0x038;
pub const PCIX0_INTLN: c_uint = 0x03c;
pub const PCIX0_INTPN: c_uint = 0x03d;
pub const PCIX0_MINGNT: c_uint = 0x03e;
pub const PCIX0_MAXLTNCY: c_uint = 0x03f;
pub const PCIX0_BRDGOPT1: c_uint = 0x040;
pub const PCIX0_BRDGOPT2: c_uint = 0x044;
pub const PCIX0_ERREN: c_uint = 0x050;
pub const PCIX0_ERRSTS: c_uint = 0x054;
pub const PCIX0_PLBBESR: c_uint = 0x058;
pub const PCIX0_PLBBEARL: c_uint = 0x05c;
pub const PCIX0_PLBBEARH: c_uint = 0x060;
pub const PCIX0_POM0LAL: c_uint = 0x068;
pub const PCIX0_POM0LAH: c_uint = 0x06c;
pub const PCIX0_POM0SA: c_uint = 0x070;
pub const PCIX0_POM0PCIAL: c_uint = 0x074;
pub const PCIX0_POM0PCIAH: c_uint = 0x078;
pub const PCIX0_POM1LAL: c_uint = 0x07c;
pub const PCIX0_POM1LAH: c_uint = 0x080;
pub const PCIX0_POM1SA: c_uint = 0x084;
pub const PCIX0_POM1PCIAL: c_uint = 0x088;
pub const PCIX0_POM1PCIAH: c_uint = 0x08c;
pub const PCIX0_POM2SA: c_uint = 0x090;
pub const PCIX0_PIM0SAL: c_uint = 0x098;

pub const PCIX0_PIM0LAL: c_uint = 0x09c;
pub const PCIX0_PIM0LAH: c_uint = 0x0a0;
pub const PCIX0_PIM1SA: c_uint = 0x0a4;
pub const PCIX0_PIM1LAL: c_uint = 0x0a8;
pub const PCIX0_PIM1LAH: c_uint = 0x0ac;
pub const PCIX0_PIM2SAL: c_uint = 0x0b0;

pub const PCIX0_PIM2LAL: c_uint = 0x0b4;
pub const PCIX0_PIM2LAH: c_uint = 0x0b8;
pub const PCIX0_OMCAPID: c_uint = 0x0c0;
pub const PCIX0_OMNIPTR: c_uint = 0x0c1;
pub const PCIX0_OMMC: c_uint = 0x0c2;
pub const PCIX0_OMMA: c_uint = 0x0c4;
pub const PCIX0_OMMUA: c_uint = 0x0c8;
pub const PCIX0_OMMDATA: c_uint = 0x0cc;
pub const PCIX0_OMMEOI: c_uint = 0x0ce;
pub const PCIX0_PMCAPID: c_uint = 0x0d0;
pub const PCIX0_PMNIPTR: c_uint = 0x0d1;
pub const PCIX0_PMC: c_uint = 0x0d2;
pub const PCIX0_PMCSR: c_uint = 0x0d4;
pub const PCIX0_PMCSRBSE: c_uint = 0x0d6;
pub const PCIX0_PMDATA: c_uint = 0x0d7;
pub const PCIX0_PMSCRR: c_uint = 0x0d8;
pub const PCIX0_CAPID: c_uint = 0x0dc;
pub const PCIX0_NIPTR: c_uint = 0x0dd;
pub const PCIX0_CMD: c_uint = 0x0de;
pub const PCIX0_STS: c_uint = 0x0e0;
pub const PCIX0_IDR: c_uint = 0x0e4;
pub const PCIX0_CID: c_uint = 0x0e8;
pub const PCIX0_RID: c_uint = 0x0ec;
pub const PCIX0_PIM0SAH: c_uint = 0x0f8;
pub const PCIX0_PIM2SAH: c_uint = 0x0fc;
pub const PCIX0_MSGIL: c_uint = 0x100;
pub const PCIX0_MSGIH: c_uint = 0x104;
pub const PCIX0_MSGOL: c_uint = 0x108;
pub const PCIX0_MSGOH: c_uint = 0x10c;
pub const PCIX0_IM: c_uint = 0x1f8;
//
// 4xx PCI bridge register definitions
//
pub const PCIL0_PMM0LA: c_uint = 0x00;
pub const PCIL0_PMM0MA: c_uint = 0x04;
pub const PCIL0_PMM0PCILA: c_uint = 0x08;
pub const PCIL0_PMM0PCIHA: c_uint = 0x0c;
pub const PCIL0_PMM1LA: c_uint = 0x10;
pub const PCIL0_PMM1MA: c_uint = 0x14;
pub const PCIL0_PMM1PCILA: c_uint = 0x18;
pub const PCIL0_PMM1PCIHA: c_uint = 0x1c;
pub const PCIL0_PMM2LA: c_uint = 0x20;
pub const PCIL0_PMM2MA: c_uint = 0x24;
pub const PCIL0_PMM2PCILA: c_uint = 0x28;
pub const PCIL0_PMM2PCIHA: c_uint = 0x2c;
pub const PCIL0_PTM1MS: c_uint = 0x30;
pub const PCIL0_PTM1LA: c_uint = 0x34;
pub const PCIL0_PTM2MS: c_uint = 0x38;
pub const PCIL0_PTM2LA: c_uint = 0x3c;
//
// 4xx PCIe bridge register definitions
//
// DCR offsets
pub const DCRO_PEGPL_CFGBAH: c_uint = 0x00;
pub const DCRO_PEGPL_CFGBAL: c_uint = 0x01;
pub const DCRO_PEGPL_CFGMSK: c_uint = 0x02;
pub const DCRO_PEGPL_MSGBAH: c_uint = 0x03;
pub const DCRO_PEGPL_MSGBAL: c_uint = 0x04;
pub const DCRO_PEGPL_MSGMSK: c_uint = 0x05;
pub const DCRO_PEGPL_OMR1BAH: c_uint = 0x06;
pub const DCRO_PEGPL_OMR1BAL: c_uint = 0x07;
pub const DCRO_PEGPL_OMR1MSKH: c_uint = 0x08;
pub const DCRO_PEGPL_OMR1MSKL: c_uint = 0x09;
pub const DCRO_PEGPL_OMR2BAH: c_uint = 0x0a;
pub const DCRO_PEGPL_OMR2BAL: c_uint = 0x0b;
pub const DCRO_PEGPL_OMR2MSKH: c_uint = 0x0c;
pub const DCRO_PEGPL_OMR2MSKL: c_uint = 0x0d;
pub const DCRO_PEGPL_OMR3BAH: c_uint = 0x0e;
pub const DCRO_PEGPL_OMR3BAL: c_uint = 0x0f;
pub const DCRO_PEGPL_OMR3MSKH: c_uint = 0x10;
pub const DCRO_PEGPL_OMR3MSKL: c_uint = 0x11;
pub const DCRO_PEGPL_REGBAH: c_uint = 0x12;
pub const DCRO_PEGPL_REGBAL: c_uint = 0x13;
pub const DCRO_PEGPL_REGMSK: c_uint = 0x14;
pub const DCRO_PEGPL_SPECIAL: c_uint = 0x15;
pub const DCRO_PEGPL_CFG: c_uint = 0x16;
pub const DCRO_PEGPL_ESR: c_uint = 0x17;
pub const DCRO_PEGPL_EARH: c_uint = 0x18;
pub const DCRO_PEGPL_EARL: c_uint = 0x19;
pub const DCRO_PEGPL_EATR: c_uint = 0x1a;
// DMER mask
pub const GPL_DMER_MASK_DISA: c_uint = 0x02000000;
//
// System DCRs (SDRs)
//
pub const PESDR0_PLLLCT1: c_uint = 0x03a0;
pub const PESDR0_PLLLCT2: c_uint = 0x03a1;
pub const PESDR0_PLLLCT3: c_uint = 0x03a2;
//
// 440SPe additional DCRs
//
pub const PESDR0_440SPE_UTLSET1: c_uint = 0x0300;
pub const PESDR0_440SPE_UTLSET2: c_uint = 0x0301;
pub const PESDR0_440SPE_DLPSET: c_uint = 0x0302;
pub const PESDR0_440SPE_LOOP: c_uint = 0x0303;
pub const PESDR0_440SPE_RCSSET: c_uint = 0x0304;
pub const PESDR0_440SPE_RCSSTS: c_uint = 0x0305;
pub const PESDR0_440SPE_HSSL0SET1: c_uint = 0x0306;
pub const PESDR0_440SPE_HSSL0SET2: c_uint = 0x0307;
pub const PESDR0_440SPE_HSSL0STS: c_uint = 0x0308;
pub const PESDR0_440SPE_HSSL1SET1: c_uint = 0x0309;
pub const PESDR0_440SPE_HSSL1SET2: c_uint = 0x030a;
pub const PESDR0_440SPE_HSSL1STS: c_uint = 0x030b;
pub const PESDR0_440SPE_HSSL2SET1: c_uint = 0x030c;
pub const PESDR0_440SPE_HSSL2SET2: c_uint = 0x030d;
pub const PESDR0_440SPE_HSSL2STS: c_uint = 0x030e;
pub const PESDR0_440SPE_HSSL3SET1: c_uint = 0x030f;
pub const PESDR0_440SPE_HSSL3SET2: c_uint = 0x0310;
pub const PESDR0_440SPE_HSSL3STS: c_uint = 0x0311;
pub const PESDR0_440SPE_HSSL4SET1: c_uint = 0x0312;
pub const PESDR0_440SPE_HSSL4SET2: c_uint = 0x0313;
pub const PESDR0_440SPE_HSSL4STS: c_uint = 0x0314;
pub const PESDR0_440SPE_HSSL5SET1: c_uint = 0x0315;
pub const PESDR0_440SPE_HSSL5SET2: c_uint = 0x0316;
pub const PESDR0_440SPE_HSSL5STS: c_uint = 0x0317;
pub const PESDR0_440SPE_HSSL6SET1: c_uint = 0x0318;
pub const PESDR0_440SPE_HSSL6SET2: c_uint = 0x0319;
pub const PESDR0_440SPE_HSSL6STS: c_uint = 0x031a;
pub const PESDR0_440SPE_HSSL7SET1: c_uint = 0x031b;
pub const PESDR0_440SPE_HSSL7SET2: c_uint = 0x031c;
pub const PESDR0_440SPE_HSSL7STS: c_uint = 0x031d;
pub const PESDR0_440SPE_HSSCTLSET: c_uint = 0x031e;
pub const PESDR0_440SPE_LANE_ABCD: c_uint = 0x031f;
pub const PESDR0_440SPE_LANE_EFGH: c_uint = 0x0320;
pub const PESDR1_440SPE_UTLSET1: c_uint = 0x0340;
pub const PESDR1_440SPE_UTLSET2: c_uint = 0x0341;
pub const PESDR1_440SPE_DLPSET: c_uint = 0x0342;
pub const PESDR1_440SPE_LOOP: c_uint = 0x0343;
pub const PESDR1_440SPE_RCSSET: c_uint = 0x0344;
pub const PESDR1_440SPE_RCSSTS: c_uint = 0x0345;
pub const PESDR1_440SPE_HSSL0SET1: c_uint = 0x0346;
pub const PESDR1_440SPE_HSSL0SET2: c_uint = 0x0347;
pub const PESDR1_440SPE_HSSL0STS: c_uint = 0x0348;
pub const PESDR1_440SPE_HSSL1SET1: c_uint = 0x0349;
pub const PESDR1_440SPE_HSSL1SET2: c_uint = 0x034a;
pub const PESDR1_440SPE_HSSL1STS: c_uint = 0x034b;
pub const PESDR1_440SPE_HSSL2SET1: c_uint = 0x034c;
pub const PESDR1_440SPE_HSSL2SET2: c_uint = 0x034d;
pub const PESDR1_440SPE_HSSL2STS: c_uint = 0x034e;
pub const PESDR1_440SPE_HSSL3SET1: c_uint = 0x034f;
pub const PESDR1_440SPE_HSSL3SET2: c_uint = 0x0350;
pub const PESDR1_440SPE_HSSL3STS: c_uint = 0x0351;
pub const PESDR1_440SPE_HSSCTLSET: c_uint = 0x0352;
pub const PESDR1_440SPE_LANE_ABCD: c_uint = 0x0353;
pub const PESDR2_440SPE_UTLSET1: c_uint = 0x0370;
pub const PESDR2_440SPE_UTLSET2: c_uint = 0x0371;
pub const PESDR2_440SPE_DLPSET: c_uint = 0x0372;
pub const PESDR2_440SPE_LOOP: c_uint = 0x0373;
pub const PESDR2_440SPE_RCSSET: c_uint = 0x0374;
pub const PESDR2_440SPE_RCSSTS: c_uint = 0x0375;
pub const PESDR2_440SPE_HSSL0SET1: c_uint = 0x0376;
pub const PESDR2_440SPE_HSSL0SET2: c_uint = 0x0377;
pub const PESDR2_440SPE_HSSL0STS: c_uint = 0x0378;
pub const PESDR2_440SPE_HSSL1SET1: c_uint = 0x0379;
pub const PESDR2_440SPE_HSSL1SET2: c_uint = 0x037a;
pub const PESDR2_440SPE_HSSL1STS: c_uint = 0x037b;
pub const PESDR2_440SPE_HSSL2SET1: c_uint = 0x037c;
pub const PESDR2_440SPE_HSSL2SET2: c_uint = 0x037d;
pub const PESDR2_440SPE_HSSL2STS: c_uint = 0x037e;
pub const PESDR2_440SPE_HSSL3SET1: c_uint = 0x037f;
pub const PESDR2_440SPE_HSSL3SET2: c_uint = 0x0380;
pub const PESDR2_440SPE_HSSL3STS: c_uint = 0x0381;
pub const PESDR2_440SPE_HSSCTLSET: c_uint = 0x0382;
pub const PESDR2_440SPE_LANE_ABCD: c_uint = 0x0383;
//
// 405EX additional DCRs
//
pub const PESDR0_405EX_UTLSET1: c_uint = 0x0400;
pub const PESDR0_405EX_UTLSET2: c_uint = 0x0401;
pub const PESDR0_405EX_DLPSET: c_uint = 0x0402;
pub const PESDR0_405EX_LOOP: c_uint = 0x0403;
pub const PESDR0_405EX_RCSSET: c_uint = 0x0404;
pub const PESDR0_405EX_RCSSTS: c_uint = 0x0405;
pub const PESDR0_405EX_PHYSET1: c_uint = 0x0406;
pub const PESDR0_405EX_PHYSET2: c_uint = 0x0407;
pub const PESDR0_405EX_BIST: c_uint = 0x0408;
pub const PESDR0_405EX_LPB: c_uint = 0x040B;
pub const PESDR0_405EX_PHYSTA: c_uint = 0x040C;
pub const PESDR1_405EX_UTLSET1: c_uint = 0x0440;
pub const PESDR1_405EX_UTLSET2: c_uint = 0x0441;
pub const PESDR1_405EX_DLPSET: c_uint = 0x0442;
pub const PESDR1_405EX_LOOP: c_uint = 0x0443;
pub const PESDR1_405EX_RCSSET: c_uint = 0x0444;
pub const PESDR1_405EX_RCSSTS: c_uint = 0x0445;
pub const PESDR1_405EX_PHYSET1: c_uint = 0x0446;
pub const PESDR1_405EX_PHYSET2: c_uint = 0x0447;
pub const PESDR1_405EX_BIST: c_uint = 0x0448;
pub const PESDR1_405EX_LPB: c_uint = 0x044B;
pub const PESDR1_405EX_PHYSTA: c_uint = 0x044C;
//
// 460EX additional DCRs
//
pub const PESDR0_460EX_L0BIST: c_uint = 0x0308;
pub const PESDR0_460EX_L0BISTSTS: c_uint = 0x0309;
pub const PESDR0_460EX_L0CDRCTL: c_uint = 0x030A;
pub const PESDR0_460EX_L0DRV: c_uint = 0x030B;
pub const PESDR0_460EX_L0REC: c_uint = 0x030C;
pub const PESDR0_460EX_L0LPB: c_uint = 0x030D;
pub const PESDR0_460EX_L0CLK: c_uint = 0x030E;
pub const PESDR0_460EX_PHY_CTL_RST: c_uint = 0x030F;
pub const PESDR0_460EX_RSTSTA: c_uint = 0x0310;
pub const PESDR0_460EX_OBS: c_uint = 0x0311;
pub const PESDR0_460EX_L0ERRC: c_uint = 0x0320;
pub const PESDR1_460EX_L0BIST: c_uint = 0x0348;
pub const PESDR1_460EX_L1BIST: c_uint = 0x0349;
pub const PESDR1_460EX_L2BIST: c_uint = 0x034A;
pub const PESDR1_460EX_L3BIST: c_uint = 0x034B;
pub const PESDR1_460EX_L0BISTSTS: c_uint = 0x034C;
pub const PESDR1_460EX_L1BISTSTS: c_uint = 0x034D;
pub const PESDR1_460EX_L2BISTSTS: c_uint = 0x034E;
pub const PESDR1_460EX_L3BISTSTS: c_uint = 0x034F;
pub const PESDR1_460EX_L0CDRCTL: c_uint = 0x0350;
pub const PESDR1_460EX_L1CDRCTL: c_uint = 0x0351;
pub const PESDR1_460EX_L2CDRCTL: c_uint = 0x0352;
pub const PESDR1_460EX_L3CDRCTL: c_uint = 0x0353;
pub const PESDR1_460EX_L0DRV: c_uint = 0x0354;
pub const PESDR1_460EX_L1DRV: c_uint = 0x0355;
pub const PESDR1_460EX_L2DRV: c_uint = 0x0356;
pub const PESDR1_460EX_L3DRV: c_uint = 0x0357;
pub const PESDR1_460EX_L0REC: c_uint = 0x0358;
pub const PESDR1_460EX_L1REC: c_uint = 0x0359;
pub const PESDR1_460EX_L2REC: c_uint = 0x035A;
pub const PESDR1_460EX_L3REC: c_uint = 0x035B;
pub const PESDR1_460EX_L0LPB: c_uint = 0x035C;
pub const PESDR1_460EX_L1LPB: c_uint = 0x035D;
pub const PESDR1_460EX_L2LPB: c_uint = 0x035E;
pub const PESDR1_460EX_L3LPB: c_uint = 0x035F;
pub const PESDR1_460EX_L0CLK: c_uint = 0x0360;
pub const PESDR1_460EX_L1CLK: c_uint = 0x0361;
pub const PESDR1_460EX_L2CLK: c_uint = 0x0362;
pub const PESDR1_460EX_L3CLK: c_uint = 0x0363;
pub const PESDR1_460EX_PHY_CTL_RST: c_uint = 0x0364;
pub const PESDR1_460EX_RSTSTA: c_uint = 0x0365;
pub const PESDR1_460EX_OBS: c_uint = 0x0366;
pub const PESDR1_460EX_L0ERRC: c_uint = 0x0368;
pub const PESDR1_460EX_L1ERRC: c_uint = 0x0369;
pub const PESDR1_460EX_L2ERRC: c_uint = 0x036A;
pub const PESDR1_460EX_L3ERRC: c_uint = 0x036B;
pub const PESDR0_460EX_IHS1: c_uint = 0x036C;
pub const PESDR0_460EX_IHS2: c_uint = 0x036D;
//
// 460SX additional DCRs
//
pub const PESDRn_460SX_RCEI: c_uint = 0x02;
pub const PESDR0_460SX_HSSL0DAMP: c_uint = 0x320;
pub const PESDR0_460SX_HSSL1DAMP: c_uint = 0x321;
pub const PESDR0_460SX_HSSL2DAMP: c_uint = 0x322;
pub const PESDR0_460SX_HSSL3DAMP: c_uint = 0x323;
pub const PESDR0_460SX_HSSL4DAMP: c_uint = 0x324;
pub const PESDR0_460SX_HSSL5DAMP: c_uint = 0x325;
pub const PESDR0_460SX_HSSL6DAMP: c_uint = 0x326;
pub const PESDR0_460SX_HSSL7DAMP: c_uint = 0x327;
pub const PESDR1_460SX_HSSL0DAMP: c_uint = 0x354;
pub const PESDR1_460SX_HSSL1DAMP: c_uint = 0x355;
pub const PESDR1_460SX_HSSL2DAMP: c_uint = 0x356;
pub const PESDR1_460SX_HSSL3DAMP: c_uint = 0x357;
pub const PESDR2_460SX_HSSL0DAMP: c_uint = 0x384;
pub const PESDR2_460SX_HSSL1DAMP: c_uint = 0x385;
pub const PESDR2_460SX_HSSL2DAMP: c_uint = 0x386;
pub const PESDR2_460SX_HSSL3DAMP: c_uint = 0x387;
pub const PESDR0_460SX_HSSL0COEFA: c_uint = 0x328;
pub const PESDR0_460SX_HSSL1COEFA: c_uint = 0x329;
pub const PESDR0_460SX_HSSL2COEFA: c_uint = 0x32A;
pub const PESDR0_460SX_HSSL3COEFA: c_uint = 0x32B;
pub const PESDR0_460SX_HSSL4COEFA: c_uint = 0x32C;
pub const PESDR0_460SX_HSSL5COEFA: c_uint = 0x32D;
pub const PESDR0_460SX_HSSL6COEFA: c_uint = 0x32E;
pub const PESDR0_460SX_HSSL7COEFA: c_uint = 0x32F;
pub const PESDR1_460SX_HSSL0COEFA: c_uint = 0x358;
pub const PESDR1_460SX_HSSL1COEFA: c_uint = 0x359;
pub const PESDR1_460SX_HSSL2COEFA: c_uint = 0x35A;
pub const PESDR1_460SX_HSSL3COEFA: c_uint = 0x35B;
pub const PESDR2_460SX_HSSL0COEFA: c_uint = 0x388;
pub const PESDR2_460SX_HSSL1COEFA: c_uint = 0x389;
pub const PESDR2_460SX_HSSL2COEFA: c_uint = 0x38A;
pub const PESDR2_460SX_HSSL3COEFA: c_uint = 0x38B;
pub const PESDR0_460SX_HSSL1CALDRV: c_uint = 0x339;
pub const PESDR1_460SX_HSSL1CALDRV: c_uint = 0x361;
pub const PESDR2_460SX_HSSL1CALDRV: c_uint = 0x391;
pub const PESDR0_460SX_HSSSLEW: c_uint = 0x338;
pub const PESDR1_460SX_HSSSLEW: c_uint = 0x360;
pub const PESDR2_460SX_HSSSLEW: c_uint = 0x390;
pub const PESDR0_460SX_HSSCTLSET: c_uint = 0x31E;
pub const PESDR1_460SX_HSSCTLSET: c_uint = 0x352;
pub const PESDR2_460SX_HSSCTLSET: c_uint = 0x382;
pub const PESDR0_460SX_RCSSET: c_uint = 0x304;
pub const PESDR1_460SX_RCSSET: c_uint = 0x344;
pub const PESDR2_460SX_RCSSET: c_uint = 0x374;
//
// Of the above, some are common offsets from the base
//
pub const PESDRn_UTLSET1: c_uint = 0x00;
pub const PESDRn_UTLSET2: c_uint = 0x01;
pub const PESDRn_DLPSET: c_uint = 0x02;
pub const PESDRn_LOOP: c_uint = 0x03;
pub const PESDRn_RCSSET: c_uint = 0x04;
pub const PESDRn_RCSSTS: c_uint = 0x05;
// 440spe only
pub const PESDRn_440SPE_HSSL0SET1: c_uint = 0x06;
pub const PESDRn_440SPE_HSSL0SET2: c_uint = 0x07;
pub const PESDRn_440SPE_HSSL0STS: c_uint = 0x08;
pub const PESDRn_440SPE_HSSL1SET1: c_uint = 0x09;
pub const PESDRn_440SPE_HSSL1SET2: c_uint = 0x0a;
pub const PESDRn_440SPE_HSSL1STS: c_uint = 0x0b;
pub const PESDRn_440SPE_HSSL2SET1: c_uint = 0x0c;
pub const PESDRn_440SPE_HSSL2SET2: c_uint = 0x0d;
pub const PESDRn_440SPE_HSSL2STS: c_uint = 0x0e;
pub const PESDRn_440SPE_HSSL3SET1: c_uint = 0x0f;
pub const PESDRn_440SPE_HSSL3SET2: c_uint = 0x10;
pub const PESDRn_440SPE_HSSL3STS: c_uint = 0x11;
// 440spe port 0 only
pub const PESDRn_440SPE_HSSL4SET1: c_uint = 0x12;
pub const PESDRn_440SPE_HSSL4SET2: c_uint = 0x13;
pub const PESDRn_440SPE_HSSL4STS: c_uint = 0x14;
pub const PESDRn_440SPE_HSSL5SET1: c_uint = 0x15;
pub const PESDRn_440SPE_HSSL5SET2: c_uint = 0x16;
pub const PESDRn_440SPE_HSSL5STS: c_uint = 0x17;
pub const PESDRn_440SPE_HSSL6SET1: c_uint = 0x18;
pub const PESDRn_440SPE_HSSL6SET2: c_uint = 0x19;
pub const PESDRn_440SPE_HSSL6STS: c_uint = 0x1a;
pub const PESDRn_440SPE_HSSL7SET1: c_uint = 0x1b;
pub const PESDRn_440SPE_HSSL7SET2: c_uint = 0x1c;
pub const PESDRn_440SPE_HSSL7STS: c_uint = 0x1d;
// 405ex only
pub const PESDRn_405EX_PHYSET1: c_uint = 0x06;
pub const PESDRn_405EX_PHYSET2: c_uint = 0x07;
pub const PESDRn_405EX_PHYSTA: c_uint = 0x0c;
//
// UTL register offsets
//
pub const PEUTL_PBCTL: c_uint = 0x00;
pub const PEUTL_PBBSZ: c_uint = 0x20;
pub const PEUTL_OPDBSZ: c_uint = 0x68;
pub const PEUTL_IPHBSZ: c_uint = 0x70;
pub const PEUTL_IPDBSZ: c_uint = 0x78;
pub const PEUTL_OUTTR: c_uint = 0x90;
pub const PEUTL_INTR: c_uint = 0x98;
pub const PEUTL_PCTL: c_uint = 0xa0;
pub const PEUTL_RCSTA: c_uint = 0xB0;
pub const PEUTL_RCIRQEN: c_uint = 0xb8;
//
// Config space register offsets
//
pub const PECFG_ECRTCTL: c_uint = 0x074;
pub const PECFG_BAR0LMPA: c_uint = 0x210;
pub const PECFG_BAR0HMPA: c_uint = 0x214;
pub const PECFG_BAR1MPA: c_uint = 0x218;
pub const PECFG_BAR2LMPA: c_uint = 0x220;
pub const PECFG_BAR2HMPA: c_uint = 0x224;
pub const PECFG_PIMEN: c_uint = 0x33c;
pub const PECFG_PIM0LAL: c_uint = 0x340;
pub const PECFG_PIM0LAH: c_uint = 0x344;
pub const PECFG_PIM1LAL: c_uint = 0x348;
pub const PECFG_PIM1LAH: c_uint = 0x34c;
pub const PECFG_PIM01SAL: c_uint = 0x350;
pub const PECFG_PIM01SAH: c_uint = 0x354;
pub const PECFG_POM0LAL: c_uint = 0x380;
pub const PECFG_POM0LAH: c_uint = 0x384;
pub const PECFG_POM1LAL: c_uint = 0x388;
pub const PECFG_POM1LAH: c_uint = 0x38c;
pub const PECFG_POM2LAL: c_uint = 0x390;
pub const PECFG_POM2LAH: c_uint = 0x394;
// 460sx only
pub const PECFG_460SX_DLLSTA: c_uint = 0x3f8;
// 460sx Bit Mappings
pub const PECFG_460SX_DLLSTA_LINKUP: c_uint = 0x00000010;
pub const DCRO_PEGPL_460SX_OMR1MSKL_UOT: c_uint = 0x00000004;
// PEGPL Bit Mappings
pub const DCRO_PEGPL_OMRxMSKL_VAL: c_uint = 0x00000001;
pub const DCRO_PEGPL_OMR1MSKL_UOT: c_uint = 0x00000002;
pub const DCRO_PEGPL_OMR3MSKL_IO: c_uint = 0x00000002;
// 476FPE
pub const PCCFG_LCPA: c_uint = 0x270;
pub const PECFG_TLDLP: c_uint = 0x3F8;
pub const PECFG_TLDLP_LNKUP: c_uint = 0x00000008;
pub const PECFG_TLDLP_PRESENT: c_uint = 0x00000010;
pub const DCRO_PEGPL_476FPE_OMR1MSKL_UOT: c_uint = 0x00000004;
// SDR Bit Mappings
pub const PESDRx_RCSSET_HLDPLB: c_uint = 0x10000000;
pub const PESDRx_RCSSET_RSTGU: c_uint = 0x01000000;
pub const PESDRx_RCSSET_RDY: c_uint = 0x00100000;
pub const PESDRx_RCSSET_RSTDL: c_uint = 0x00010000;
pub const PESDRx_RCSSET_RSTPYN: c_uint = 0x00001000;
