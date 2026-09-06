//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/44x/fsp2.h
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


pub const DCRN_CMU_ADDR: c_uint = 0x00C	/* Chip management unic addr */;
pub const DCRN_CMU_DATA: c_uint = 0x00D	/* Chip management unic data */;
// PLB4 Arbiter
pub const DCRN_PLB4_PCBI: c_uint = 0x010	/* PLB Crossbar ID/Rev Register */;
pub const DCRN_PLB4_P0ACR: c_uint = 0x011	/* PLB0 Arbiter Control Register */;
pub const DCRN_PLB4_P0ESRL: c_uint = 0x012	/* PLB0 Error Status Register Low */;
pub const DCRN_PLB4_P0ESRH: c_uint = 0x013	/* PLB0 Error Status Register High */;
pub const DCRN_PLB4_P0EARL: c_uint = 0x014	/* PLB0 Error Address Register Low */;
pub const DCRN_PLB4_P0EARH: c_uint = 0x015	/* PLB0 Error Address Register High */;
pub const DCRN_PLB4_P0ESRLS: c_uint = 0x016	/* PLB0 Error Status Register Low Set*/;
pub const DCRN_PLB4_P0ESRHS: c_uint = 0x017	/* PLB0 Error Status Register High */;
pub const DCRN_PLB4_PCBC: c_uint = 0x018	/* PLB Crossbar Control Register */;
pub const DCRN_PLB4_P1ACR: c_uint = 0x019	/* PLB1 Arbiter Control Register */;
pub const DCRN_PLB4_P1ESRL: c_uint = 0x01A	/* PLB1 Error Status Register Low */;
pub const DCRN_PLB4_P1ESRH: c_uint = 0x01B	/* PLB1 Error Status Register High */;
pub const DCRN_PLB4_P1EARL: c_uint = 0x01C	/* PLB1 Error Address Register Low */;
pub const DCRN_PLB4_P1EARH: c_uint = 0x01D	/* PLB1 Error Address Register High */;
pub const DCRN_PLB4_P1ESRLS: c_uint = 0x01E	/* PLB1 Error Status Register Low Set*/;
pub const DCRN_PLB4_P1ESRHS: c_uint = 0x01F	/*PLB1 Error Status Register High Set*/;
// PLB4/OPB bridge 0, 1, 2, 3
pub const DCRN_PLB4OPB0_BASE: c_uint = 0x020;
pub const DCRN_PLB4OPB1_BASE: c_uint = 0x030;
pub const DCRN_PLB4OPB2_BASE: c_uint = 0x040;
pub const DCRN_PLB4OPB3_BASE: c_uint = 0x050;
pub const PLB4OPB_GESR0: c_uint = 0x0	/* Error status 0: Master Dev 0-3 */;
pub const PLB4OPB_GEAR: c_uint = 0x2	/* Error Address Register */;
pub const PLB4OPB_GEARU: c_uint = 0x3	/* Error Upper Address Register */;
pub const PLB4OPB_GESR1: c_uint = 0x4	/* Error Status 1: Master Dev 4-7 */;
pub const PLB4OPB_GESR2: c_uint = 0xC	/* Error Status 2: Master Dev 8-11 */;
// PLB4-to-AHB Bridge
pub const DCRN_PLB4AHB_BASE: c_uint = 0x400;

// PLB6 Controller
pub const DCRN_PLB6_BASE: c_uint = 0x11111300;

// PLB4-to-PLB6 Bridge
pub const DCRN_PLB4PLB6_BASE: c_uint = 0x11111320;

// PLB6-to-PLB4 Bridge
pub const DCRN_PLB6PLB4_BASE: c_uint = 0x11111350;

// PLB6-to-MCIF Bridge
pub const DCRN_PLB6MCIF_BASE: c_uint = 0x11111380;

// Configuration Logic Registers
pub const DCRN_CONF_BASE: c_uint = 0x11111400;

pub const DCRN_L2CDCRAI: c_uint = 0x11111100;
pub const DCRN_L2CDCRDI: c_uint = 0x11111104;
// L2 indirect addresses
pub const L2MCK: c_uint = 0x120;
pub const L2MCKEN: c_uint = 0x130;
pub const L2INT: c_uint = 0x150;
pub const L2INTEN: c_uint = 0x160;
pub const L2LOG0: c_uint = 0x180;
pub const L2LOG1: c_uint = 0x184;
pub const L2LOG2: c_uint = 0x188;
pub const L2LOG3: c_uint = 0x18C;
pub const L2LOG4: c_uint = 0x190;
pub const L2LOG5: c_uint = 0x194;
pub const L2PLBSTAT0: c_uint = 0x300;
pub const L2PLBSTAT1: c_uint = 0x304;
pub const L2PLBMCKEN0: c_uint = 0x330;
pub const L2PLBMCKEN1: c_uint = 0x334;
pub const L2PLBINTEN0: c_uint = 0x360;
pub const L2PLBINTEN1: c_uint = 0x364;
pub const L2ARRSTAT0: c_uint = 0x500;
pub const L2ARRSTAT1: c_uint = 0x504;
pub const L2ARRSTAT2: c_uint = 0x508;
pub const L2ARRMCKEN0: c_uint = 0x530;
pub const L2ARRMCKEN1: c_uint = 0x534;
pub const L2ARRMCKEN2: c_uint = 0x538;
pub const L2ARRINTEN0: c_uint = 0x560;
pub const L2ARRINTEN1: c_uint = 0x564;
pub const L2ARRINTEN2: c_uint = 0x568;
pub const L2CPUSTAT: c_uint = 0x700;
pub const L2CPUMCKEN: c_uint = 0x730;
pub const L2CPUINTEN: c_uint = 0x760;
pub const L2RACSTAT0: c_uint = 0x900;
pub const L2RACMCKEN0: c_uint = 0x930;
pub const L2RACINTEN0: c_uint = 0x960;
pub const L2WACSTAT0: c_uint = 0xD00;
pub const L2WACSTAT1: c_uint = 0xD04;
pub const L2WACSTAT2: c_uint = 0xD08;
pub const L2WACMCKEN0: c_uint = 0xD30;
pub const L2WACMCKEN1: c_uint = 0xD34;
pub const L2WACMCKEN2: c_uint = 0xD38;
pub const L2WACINTEN0: c_uint = 0xD60;
pub const L2WACINTEN1: c_uint = 0xD64;
pub const L2WACINTEN2: c_uint = 0xD68;
pub const L2WDFSTAT: c_uint = 0xF00;
pub const L2WDFMCKEN: c_uint = 0xF30;
pub const L2WDFINTEN: c_uint = 0xF60;
// DDR3/4 Memory Controller
pub const DCRN_DDR34_BASE: c_uint = 0x11120000;
pub const DCRN_DDR34_MCSTAT: c_uint = 0x10;
pub const DCRN_DDR34_MCOPT1: c_uint = 0x20;
pub const DCRN_DDR34_MCOPT2: c_uint = 0x21;
pub const DCRN_DDR34_PHYSTAT: c_uint = 0x32;
pub const DCRN_DDR34_CFGR0: c_uint = 0x40;
pub const DCRN_DDR34_CFGR1: c_uint = 0x41;
pub const DCRN_DDR34_CFGR2: c_uint = 0x42;
pub const DCRN_DDR34_CFGR3: c_uint = 0x43;
pub const DCRN_DDR34_SCRUB_CNTL: c_uint = 0xAA;
pub const DCRN_DDR34_SCRUB_INT: c_uint = 0xAB;
pub const DCRN_DDR34_SCRUB_START_ADDR: c_uint = 0xB0;
pub const DCRN_DDR34_SCRUB_END_ADDR: c_uint = 0xD0;
pub const DCRN_DDR34_ECCERR_ADDR_PORT0: c_uint = 0xE0;
pub const DCRN_DDR34_ECCERR_ADDR_PORT1: c_uint = 0xE1;
pub const DCRN_DDR34_ECCERR_ADDR_PORT2: c_uint = 0xE2;
pub const DCRN_DDR34_ECCERR_ADDR_PORT3: c_uint = 0xE3;
pub const DCRN_DDR34_ECCERR_COUNT_PORT0: c_uint = 0xE4;
pub const DCRN_DDR34_ECCERR_COUNT_PORT1: c_uint = 0xE5;
pub const DCRN_DDR34_ECCERR_COUNT_PORT2: c_uint = 0xE6;
pub const DCRN_DDR34_ECCERR_COUNT_PORT3: c_uint = 0xE7;
pub const DCRN_DDR34_ECCERR_PORT0: c_uint = 0xF0;
pub const DCRN_DDR34_ECCERR_PORT1: c_uint = 0xF2;
pub const DCRN_DDR34_ECCERR_PORT2: c_uint = 0xF4;
pub const DCRN_DDR34_ECCERR_PORT3: c_uint = 0xF6;
pub const DCRN_DDR34_ECC_CHECK_PORT0: c_uint = 0xF8;
pub const DCRN_DDR34_ECC_CHECK_PORT1: c_uint = 0xF9;
pub const DCRN_DDR34_ECC_CHECK_PORT2: c_uint = 0xF9;
pub const DCRN_DDR34_ECC_CHECK_PORT3: c_uint = 0xFB;
pub const DDR34_SCRUB_CNTL_STOP: c_uint = 0x00000000;
pub const DDR34_SCRUB_CNTL_SCRUB: c_uint = 0x80000000;
pub const DDR34_SCRUB_CNTL_UE_STOP: c_uint = 0x20000000;
pub const DDR34_SCRUB_CNTL_CE_STOP: c_uint = 0x10000000;
pub const DDR34_SCRUB_CNTL_RANK_EN: c_uint = 0x00008000;
// PLB-Attached DDR3/4 Core Wrapper
pub const DCRN_CW_BASE: c_uint = 0x11111800;
pub const DCRN_CW_MCER0: c_uint = 0x00;
pub const DCRN_CW_MCER1: c_uint = 0x01;
pub const DCRN_CW_MCER_AND0: c_uint = 0x02;
pub const DCRN_CW_MCER_AND1: c_uint = 0x03;
pub const DCRN_CW_MCER_OR0: c_uint = 0x04;
pub const DCRN_CW_MCER_OR1: c_uint = 0x05;
pub const DCRN_CW_MCER_MASK0: c_uint = 0x06;
pub const DCRN_CW_MCER_MASK1: c_uint = 0x07;
pub const DCRN_CW_MCER_MASK_AND0: c_uint = 0x08;
pub const DCRN_CW_MCER_MASK_AND1: c_uint = 0x09;
pub const DCRN_CW_MCER_MASK_OR0: c_uint = 0x0A;
pub const DCRN_CW_MCER_MASK_OR1: c_uint = 0x0B;
pub const DCRN_CW_MCER_ACTION0: c_uint = 0x0C;
pub const DCRN_CW_MCER_ACTION1: c_uint = 0x0D;
pub const DCRN_CW_MCER_WOF0: c_uint = 0x0E;
pub const DCRN_CW_MCER_WOF1: c_uint = 0x0F;
pub const DCRN_CW_LFIR: c_uint = 0x10;
pub const DCRN_CW_LFIR_AND: c_uint = 0x11;
pub const DCRN_CW_LFIR_OR: c_uint = 0x12;
pub const DCRN_CW_LFIR_MASK: c_uint = 0x13;
pub const DCRN_CW_LFIR_MASK_AND: c_uint = 0x14;
pub const DCRN_CW_LFIR_MASK_OR: c_uint = 0x15;
pub const CW_MCER0_MEM_CE: c_uint = 0x00020000;
// CMU addresses
pub const CMUN_CRCS: c_uint = 0x00 /* Chip Reset Control/Status */;
pub const CMUN_CONFFIR0: c_uint = 0x20 /* Config Reg Parity FIR 0 */;
pub const CMUN_CONFFIR1: c_uint = 0x21 /* Config Reg Parity FIR 1 */;
pub const CMUN_CONFFIR2: c_uint = 0x22 /* Config Reg Parity FIR 2 */;
pub const CMUN_CONFFIR3: c_uint = 0x23 /* Config Reg Parity FIR 3 */;
pub const CMUN_URCR3_RS: c_uint = 0x24 /* Unit Reset Control Reg 3 Set */;
pub const CMUN_URCR3_C: c_uint = 0x25 /* Unit Reset Control Reg 3 Clear */;
pub const CMUN_URCR3_P: c_uint = 0x26 /* Unit Reset Control Reg 3 Pulse */;
pub const CMUN_PW0: c_uint = 0x2C /* Pulse Width Register */;
pub const CMUN_URCR0_P: c_uint = 0x2D /* Unit Reset Control Reg 0 Pulse */;
pub const CMUN_URCR1_P: c_uint = 0x2E /* Unit Reset Control Reg 1 Pulse */;
pub const CMUN_URCR2_P: c_uint = 0x2F /* Unit Reset Control Reg 2 Pulse */;
pub const CMUN_CLS_RW: c_uint = 0x30 /* Code Load Status (Read/Write) */;
pub const CMUN_CLS_S: c_uint = 0x31 /* Code Load Status (Set) */;
pub const CMUN_CLS_C: c_uint = 0x32 /* Code Load Status (Clear */;
pub const CMUN_URCR2_RS: c_uint = 0x33 /* Unit Reset Control Reg 2 Set */;
pub const CMUN_URCR2_C: c_uint = 0x34 /* Unit Reset Control Reg 2 Clear */;
pub const CMUN_CLKEN0: c_uint = 0x35 /* Clock Enable 0 */;
pub const CMUN_CLKEN1: c_uint = 0x36 /* Clock Enable 1 */;
pub const CMUN_PCD0: c_uint = 0x37 /* PSI clock divider 0 */;
pub const CMUN_PCD1: c_uint = 0x38 /* PSI clock divider 1 */;
pub const CMUN_TMR0: c_uint = 0x39 /* Reset Timer */;
pub const CMUN_TVS0: c_uint = 0x3A /* TV Sense Reg 0 */;
pub const CMUN_TVS1: c_uint = 0x3B /* TV Sense Reg 1 */;
pub const CMUN_MCCR: c_uint = 0x3C /* DRAM Configuration Reg */;
pub const CMUN_FIR0: c_uint = 0x3D /* Fault Isolation Reg 0 */;
pub const CMUN_FMR0: c_uint = 0x3E /* FIR Mask Reg 0 */;
pub const CMUN_ETDRB: c_uint = 0x3F /* ETDR Backdoor */;
// CRCS bit fields
pub const CRCS_STAT_MASK: c_uint = 0xF0000000;
pub const CRCS_STAT_POR: c_uint = 0x10000000;
pub const CRCS_STAT_PHR: c_uint = 0x20000000;
pub const CRCS_STAT_PCIE: c_uint = 0x30000000;
pub const CRCS_STAT_CRCS_SYS: c_uint = 0x40000000;
pub const CRCS_STAT_DBCR_SYS: c_uint = 0x50000000;
pub const CRCS_STAT_HOST_SYS: c_uint = 0x60000000;
pub const CRCS_STAT_CHIP_RST_B: c_uint = 0x70000000;
pub const CRCS_STAT_CRCS_CHIP: c_uint = 0x80000000;
pub const CRCS_STAT_DBCR_CHIP: c_uint = 0x90000000;
pub const CRCS_STAT_HOST_CHIP: c_uint = 0xA0000000;
pub const CRCS_STAT_PSI_CHIP: c_uint = 0xB0000000;
pub const CRCS_STAT_CRCS_CORE: c_uint = 0xC0000000;
pub const CRCS_STAT_DBCR_CORE: c_uint = 0xD0000000;
pub const CRCS_STAT_HOST_CORE: c_uint = 0xE0000000;
pub const CRCS_STAT_PCIE_HOT: c_uint = 0xF0000000;
pub const CRCS_STAT_SELF_CORE: c_uint = 0x40000000;
pub const CRCS_STAT_SELF_CHIP: c_uint = 0x50000000;
pub const CRCS_WATCHE: c_uint = 0x08000000;
pub const CRCS_CORE: c_uint = 0x04000000 /* Reset PPC440 core */;
pub const CRCS_CHIP: c_uint = 0x02000000 /* Chip Reset */;
pub const CRCS_SYS: c_uint = 0x01000000 /* System Reset */;
pub const CRCS_WRCR: c_uint = 0x00800000 /* Watchdog reset on core reset */;
pub const CRCS_EXTCR: c_uint = 0x00080000 /* CHIP_RST_B triggers chip reset */;
pub const CRCS_PLOCK: c_uint = 0x00000002 /* PLL Locked */;

// Macro flag: #define mfcmu(reg)\

