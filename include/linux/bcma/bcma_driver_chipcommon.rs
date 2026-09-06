//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_driver_chipcommon.h
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

// ChipCommon core registers.
pub const BCMA_CC_ID: c_uint = 0x0000;
pub const BCMA_CC_ID_ID: c_uint = 0x0000FFFF;
pub const BCMA_CC_ID_ID_SHIFT: c_int = 0;
pub const BCMA_CC_ID_REV: c_uint = 0x000F0000;
pub const BCMA_CC_ID_REV_SHIFT: c_int = 16;
pub const BCMA_CC_ID_PKG: c_uint = 0x00F00000;
pub const BCMA_CC_ID_PKG_SHIFT: c_int = 20;
pub const BCMA_CC_ID_NRCORES: c_uint = 0x0F000000;
pub const BCMA_CC_ID_NRCORES_SHIFT: c_int = 24;
pub const BCMA_CC_ID_TYPE: c_uint = 0xF0000000;
pub const BCMA_CC_ID_TYPE_SHIFT: c_int = 28;
pub const BCMA_CC_CAP: c_uint = 0x0004		/* Capabilities */;
pub const BCMA_CC_CAP_NRUART: c_uint = 0x00000003	/* # of UARTs */;
pub const BCMA_CC_CAP_MIPSEB: c_uint = 0x00000004	/* MIPS in BigEndian Mode */;
pub const BCMA_CC_CAP_UARTCLK: c_uint = 0x00000018	/* UART clock select */;
pub const BCMA_CC_CAP_UARTCLK_INT: c_uint = 0x00000008	/* UARTs are driven by internal divided clock */;
pub const BCMA_CC_CAP_UARTGPIO: c_uint = 0x00000020	/* UARTs on GPIO 15-12 */;
pub const BCMA_CC_CAP_EXTBUS: c_uint = 0x000000C0	/* External buses present */;
pub const BCMA_CC_CAP_FLASHT: c_uint = 0x00000700	/* Flash Type */;
pub const BCMA_CC_FLASHT_NONE: c_uint = 0x00000000	/* No flash */;
pub const BCMA_CC_FLASHT_STSER: c_uint = 0x00000100	/* ST serial flash */;
pub const BCMA_CC_FLASHT_ATSER: c_uint = 0x00000200	/* Atmel serial flash */;
pub const BCMA_CC_FLASHT_NAND: c_uint = 0x00000300	/* NAND flash */;
pub const BCMA_CC_FLASHT_PARA: c_uint = 0x00000700	/* Parallel flash */;
pub const BCMA_CC_CAP_PLLT: c_uint = 0x00038000	/* PLL Type */;
pub const BCMA_PLLTYPE_NONE: c_uint = 0x00000000;
pub const BCMA_PLLTYPE_1: c_uint = 0x00010000	/* 48Mhz base, 3 dividers */;
pub const BCMA_PLLTYPE_2: c_uint = 0x00020000	/* 48Mhz, 4 dividers */;
pub const BCMA_PLLTYPE_3: c_uint = 0x00030000	/* 25Mhz, 2 dividers */;
pub const BCMA_PLLTYPE_4: c_uint = 0x00008000	/* 48Mhz, 4 dividers */;
pub const BCMA_PLLTYPE_5: c_uint = 0x00018000	/* 25Mhz, 4 dividers */;
pub const BCMA_PLLTYPE_6: c_uint = 0x00028000	/* 100/200 or 120/240 only */;
pub const BCMA_PLLTYPE_7: c_uint = 0x00038000	/* 25Mhz, 4 dividers */;
pub const BCMA_CC_CAP_PCTL: c_uint = 0x00040000	/* Power Control */;
pub const BCMA_CC_CAP_OTPS: c_uint = 0x00380000	/* OTP size */;
pub const BCMA_CC_CAP_OTPS_SHIFT: c_int = 19;
pub const BCMA_CC_CAP_OTPS_BASE: c_int = 5;
pub const BCMA_CC_CAP_JTAGM: c_uint = 0x00400000	/* JTAG master present */;
pub const BCMA_CC_CAP_BROM: c_uint = 0x00800000	/* Internal boot ROM active */;
pub const BCMA_CC_CAP_64BIT: c_uint = 0x08000000	/* 64-bit Backplane */;
pub const BCMA_CC_CAP_PMU: c_uint = 0x10000000	/* PMU available (rev >= 20) */;
pub const BCMA_CC_CAP_ECI: c_uint = 0x20000000	/* ECI available (rev >= 20) */;
pub const BCMA_CC_CAP_SPROM: c_uint = 0x40000000	/* SPROM present */;
pub const BCMA_CC_CAP_NFLASH: c_uint = 0x80000000	/* NAND flash present (rev >= 35 or BCM4706?) */;
pub const BCMA_CC_CORECTL: c_uint = 0x0008;
pub const BCMA_CC_CORECTL_UARTCLK0: c_uint = 0x00000001	/* Drive UART with internal clock */;
pub const BCMA_CC_CORECTL_SE: c_uint = 0x00000002	/* sync clk out enable (corerev >= 3) */;
pub const BCMA_CC_CORECTL_UARTCLKEN: c_uint = 0x00000008	/* UART clock enable (rev >= 21) */;
pub const BCMA_CC_BIST: c_uint = 0x000C;
pub const BCMA_CC_OTPS: c_uint = 0x0010		/* OTP status */;
pub const BCMA_CC_OTPS_PROGFAIL: c_uint = 0x80000000;
pub const BCMA_CC_OTPS_PROTECT: c_uint = 0x00000007;
pub const BCMA_CC_OTPS_HW_PROTECT: c_uint = 0x00000001;
pub const BCMA_CC_OTPS_SW_PROTECT: c_uint = 0x00000002;
pub const BCMA_CC_OTPS_CID_PROTECT: c_uint = 0x00000004;
pub const BCMA_CC_OTPS_GU_PROG_IND: c_uint = 0x00000F00	/* General Use programmed indication */;
pub const BCMA_CC_OTPS_GU_PROG_IND_SHIFT: c_int = 8;
pub const BCMA_CC_OTPS_GU_PROG_HW: c_uint = 0x00000100	/* HW region programmed */;
pub const BCMA_CC_OTPC: c_uint = 0x0014		/* OTP control */;
pub const BCMA_CC_OTPC_RECWAIT: c_uint = 0xFF000000;
pub const BCMA_CC_OTPC_PROGWAIT: c_uint = 0x00FFFF00;
pub const BCMA_CC_OTPC_PRW_SHIFT: c_int = 8;
pub const BCMA_CC_OTPC_MAXFAIL: c_uint = 0x00000038;
pub const BCMA_CC_OTPC_VSEL: c_uint = 0x00000006;
pub const BCMA_CC_OTPC_SELVL: c_uint = 0x00000001;
pub const BCMA_CC_OTPP: c_uint = 0x0018		/* OTP prog */;
pub const BCMA_CC_OTPP_COL: c_uint = 0x000000FF;
pub const BCMA_CC_OTPP_ROW: c_uint = 0x0000FF00;
pub const BCMA_CC_OTPP_ROW_SHIFT: c_int = 8;
pub const BCMA_CC_OTPP_READERR: c_uint = 0x10000000;
pub const BCMA_CC_OTPP_VALUE: c_uint = 0x20000000;
pub const BCMA_CC_OTPP_READ: c_uint = 0x40000000;
pub const BCMA_CC_OTPP_START: c_uint = 0x80000000;
pub const BCMA_CC_OTPP_BUSY: c_uint = 0x80000000;
pub const BCMA_CC_OTPL: c_uint = 0x001C		/* OTP layout */;
pub const BCMA_CC_OTPL_GURGN_OFFSET: c_uint = 0x00000FFF	/* offset of general use region */;
pub const BCMA_CC_IRQSTAT: c_uint = 0x0020;
pub const BCMA_CC_IRQMASK: c_uint = 0x0024;
pub const BCMA_CC_IRQ_GPIO: c_uint = 0x00000001	/* gpio intr */;
pub const BCMA_CC_IRQ_EXT: c_uint = 0x00000002	/* ro: ext intr pin (corerev >= 3) */;
pub const BCMA_CC_IRQ_WDRESET: c_uint = 0x80000000	/* watchdog reset occurred */;
pub const BCMA_CC_CHIPCTL: c_uint = 0x0028		/* Rev >= 11 only */;
pub const BCMA_CC_CHIPSTAT: c_uint = 0x002C		/* Rev >= 11 only */;
pub const BCMA_CC_CHIPST_4313_SPROM_PRESENT: c_int = 1;
pub const BCMA_CC_CHIPST_4313_OTP_PRESENT: c_int = 2;
pub const BCMA_CC_CHIPST_4331_SPROM_PRESENT: c_int = 2;
pub const BCMA_CC_CHIPST_4331_OTP_PRESENT: c_int = 4;
pub const BCMA_CC_CHIPST_43228_ILP_DIV_EN: c_uint = 0x00000001;
pub const BCMA_CC_CHIPST_43228_OTP_PRESENT: c_uint = 0x00000002;
pub const BCMA_CC_CHIPST_43228_SERDES_REFCLK_PADSEL: c_uint = 0x00000004;
pub const BCMA_CC_CHIPST_43228_SDIO_MODE: c_uint = 0x00000008;
pub const BCMA_CC_CHIPST_43228_SDIO_OTP_PRESENT: c_uint = 0x00000010;
pub const BCMA_CC_CHIPST_43228_SDIO_RESET: c_uint = 0x00000020;

pub const BCMA_CC_CHIPST_4360_XTAL_40MZ: c_uint = 0x00000001;
pub const BCMA_CC_JCMD: c_uint = 0x0030		/* Rev >= 10 only */;
pub const BCMA_CC_JCMD_START: c_uint = 0x80000000;
pub const BCMA_CC_JCMD_BUSY: c_uint = 0x80000000;
pub const BCMA_CC_JCMD_PAUSE: c_uint = 0x40000000;
pub const BCMA_CC_JCMD0_ACC_MASK: c_uint = 0x0000F000;
pub const BCMA_CC_JCMD0_ACC_IRDR: c_uint = 0x00000000;
pub const BCMA_CC_JCMD0_ACC_DR: c_uint = 0x00001000;
pub const BCMA_CC_JCMD0_ACC_IR: c_uint = 0x00002000;
pub const BCMA_CC_JCMD0_ACC_RESET: c_uint = 0x00003000;
pub const BCMA_CC_JCMD0_ACC_IRPDR: c_uint = 0x00004000;
pub const BCMA_CC_JCMD0_ACC_PDR: c_uint = 0x00005000;
pub const BCMA_CC_JCMD0_IRW_MASK: c_uint = 0x00000F00;
pub const BCMA_CC_JCMD_ACC_MASK: c_uint = 0x000F0000	/* Changes for corerev 11 */;
pub const BCMA_CC_JCMD_ACC_IRDR: c_uint = 0x00000000;
pub const BCMA_CC_JCMD_ACC_DR: c_uint = 0x00010000;
pub const BCMA_CC_JCMD_ACC_IR: c_uint = 0x00020000;
pub const BCMA_CC_JCMD_ACC_RESET: c_uint = 0x00030000;
pub const BCMA_CC_JCMD_ACC_IRPDR: c_uint = 0x00040000;
pub const BCMA_CC_JCMD_ACC_PDR: c_uint = 0x00050000;
pub const BCMA_CC_JCMD_IRW_MASK: c_uint = 0x00001F00;
pub const BCMA_CC_JCMD_IRW_SHIFT: c_int = 8;
pub const BCMA_CC_JCMD_DRW_MASK: c_uint = 0x0000003F;
pub const BCMA_CC_JIR: c_uint = 0x0034		/* Rev >= 10 only */;
pub const BCMA_CC_JDR: c_uint = 0x0038		/* Rev >= 10 only */;
pub const BCMA_CC_JCTL: c_uint = 0x003C		/* Rev >= 10 only */;

pub const BCMA_CC_FLASHCTL: c_uint = 0x0040;
// Start/busy bit in flashcontrol
pub const BCMA_CC_FLASHCTL_OPCODE: c_uint = 0x000000ff;
pub const BCMA_CC_FLASHCTL_ACTION: c_uint = 0x00000700;
pub const BCMA_CC_FLASHCTL_CS_ACTIVE: c_uint = 0x00001000	/* Chip Select Active, rev >= 20 */;
pub const BCMA_CC_FLASHCTL_START: c_uint = 0x80000000;

// Flashcontrol action + opcodes for ST flashes
pub const BCMA_CC_FLASHCTL_ST_WREN: c_uint = 0x0006		/* Write Enable */;
pub const BCMA_CC_FLASHCTL_ST_WRDIS: c_uint = 0x0004		/* Write Disable */;
pub const BCMA_CC_FLASHCTL_ST_RDSR: c_uint = 0x0105		/* Read Status Register */;
pub const BCMA_CC_FLASHCTL_ST_WRSR: c_uint = 0x0101		/* Write Status Register */;
pub const BCMA_CC_FLASHCTL_ST_READ: c_uint = 0x0303		/* Read Data Bytes */;
pub const BCMA_CC_FLASHCTL_ST_PP: c_uint = 0x0302		/* Page Program */;
pub const BCMA_CC_FLASHCTL_ST_SE: c_uint = 0x02d8		/* Sector Erase */;
pub const BCMA_CC_FLASHCTL_ST_BE: c_uint = 0x00c7		/* Bulk Erase */;
pub const BCMA_CC_FLASHCTL_ST_DP: c_uint = 0x00b9		/* Deep Power-down */;
pub const BCMA_CC_FLASHCTL_ST_RES: c_uint = 0x03ab		/* Read Electronic Signature */;
pub const BCMA_CC_FLASHCTL_ST_CSA: c_uint = 0x1000		/* Keep chip select asserted */;
pub const BCMA_CC_FLASHCTL_ST_SSE: c_uint = 0x0220		/* Sub-sector Erase */;
// Flashcontrol action + opcodes for Atmel flashes
pub const BCMA_CC_FLASHCTL_AT_READ: c_uint = 0x07e8;
pub const BCMA_CC_FLASHCTL_AT_PAGE_READ: c_uint = 0x07d2;
pub const BCMA_CC_FLASHCTL_AT_STATUS: c_uint = 0x01d7;
pub const BCMA_CC_FLASHCTL_AT_BUF1_WRITE: c_uint = 0x0384;
pub const BCMA_CC_FLASHCTL_AT_BUF2_WRITE: c_uint = 0x0387;
pub const BCMA_CC_FLASHCTL_AT_BUF1_ERASE_PROGRAM: c_uint = 0x0283;
pub const BCMA_CC_FLASHCTL_AT_BUF2_ERASE_PROGRAM: c_uint = 0x0286;
pub const BCMA_CC_FLASHCTL_AT_BUF1_PROGRAM: c_uint = 0x0288;
pub const BCMA_CC_FLASHCTL_AT_BUF2_PROGRAM: c_uint = 0x0289;
pub const BCMA_CC_FLASHCTL_AT_PAGE_ERASE: c_uint = 0x0281;
pub const BCMA_CC_FLASHCTL_AT_BLOCK_ERASE: c_uint = 0x0250;
pub const BCMA_CC_FLASHCTL_AT_BUF1_WRITE_ERASE_PROGRAM: c_uint = 0x0382;
pub const BCMA_CC_FLASHCTL_AT_BUF2_WRITE_ERASE_PROGRAM: c_uint = 0x0385;
pub const BCMA_CC_FLASHCTL_AT_BUF1_LOAD: c_uint = 0x0253;
pub const BCMA_CC_FLASHCTL_AT_BUF2_LOAD: c_uint = 0x0255;
pub const BCMA_CC_FLASHCTL_AT_BUF1_COMPARE: c_uint = 0x0260;
pub const BCMA_CC_FLASHCTL_AT_BUF2_COMPARE: c_uint = 0x0261;
pub const BCMA_CC_FLASHCTL_AT_BUF1_REPROGRAM: c_uint = 0x0258;
pub const BCMA_CC_FLASHCTL_AT_BUF2_REPROGRAM: c_uint = 0x0259;
pub const BCMA_CC_FLASHADDR: c_uint = 0x0044;
pub const BCMA_CC_FLASHDATA: c_uint = 0x0048;
// Status register bits for ST flashes
pub const BCMA_CC_FLASHDATA_ST_WIP: c_uint = 0x01		/* Write In Progress */;
pub const BCMA_CC_FLASHDATA_ST_WEL: c_uint = 0x02		/* Write Enable Latch */;
pub const BCMA_CC_FLASHDATA_ST_BP_MASK: c_uint = 0x1c		/* Block Protect */;
pub const BCMA_CC_FLASHDATA_ST_BP_SHIFT: c_int = 2;
pub const BCMA_CC_FLASHDATA_ST_SRWD: c_uint = 0x80		/* Status Register Write Disable */;
// Status register bits for Atmel flashes
pub const BCMA_CC_FLASHDATA_AT_READY: c_uint = 0x80;
pub const BCMA_CC_FLASHDATA_AT_MISMATCH: c_uint = 0x40;
pub const BCMA_CC_FLASHDATA_AT_ID_MASK: c_uint = 0x38;
pub const BCMA_CC_FLASHDATA_AT_ID_SHIFT: c_int = 3;
pub const BCMA_CC_BCAST_ADDR: c_uint = 0x0050;
pub const BCMA_CC_BCAST_DATA: c_uint = 0x0054;
pub const BCMA_CC_GPIOPULLUP: c_uint = 0x0058		/* Rev >= 20 only */;
pub const BCMA_CC_GPIOPULLDOWN: c_uint = 0x005C		/* Rev >= 20 only */;
pub const BCMA_CC_GPIOIN: c_uint = 0x0060;
pub const BCMA_CC_GPIOOUT: c_uint = 0x0064;
pub const BCMA_CC_GPIOOUTEN: c_uint = 0x0068;
pub const BCMA_CC_GPIOCTL: c_uint = 0x006C;
pub const BCMA_CC_GPIOPOL: c_uint = 0x0070;
pub const BCMA_CC_GPIOIRQ: c_uint = 0x0074;
pub const BCMA_CC_WATCHDOG: c_uint = 0x0080;
pub const BCMA_CC_GPIOTIMER: c_uint = 0x0088		/* LED powersave (corerev >= 16) */;
pub const BCMA_CC_GPIOTIMER_OFFTIME: c_uint = 0x0000FFFF;
pub const BCMA_CC_GPIOTIMER_OFFTIME_SHIFT: c_int = 0;
pub const BCMA_CC_GPIOTIMER_ONTIME: c_uint = 0xFFFF0000;
pub const BCMA_CC_GPIOTIMER_ONTIME_SHIFT: c_int = 16;
pub const BCMA_CC_GPIOTOUTM: c_uint = 0x008C		/* LED powersave (corerev >= 16) */;
pub const BCMA_CC_CLOCK_N: c_uint = 0x0090;
pub const BCMA_CC_CLOCK_SB: c_uint = 0x0094;
pub const BCMA_CC_CLOCK_PCI: c_uint = 0x0098;
pub const BCMA_CC_CLOCK_M2: c_uint = 0x009C;
pub const BCMA_CC_CLOCK_MIPS: c_uint = 0x00A0;
pub const BCMA_CC_CLKDIV: c_uint = 0x00A4		/* Rev >= 3 only */;
pub const BCMA_CC_CLKDIV_SFLASH: c_uint = 0x0F000000;
pub const BCMA_CC_CLKDIV_SFLASH_SHIFT: c_int = 24;
pub const BCMA_CC_CLKDIV_OTP: c_uint = 0x000F0000;
pub const BCMA_CC_CLKDIV_OTP_SHIFT: c_int = 16;
pub const BCMA_CC_CLKDIV_JTAG: c_uint = 0x00000F00;
pub const BCMA_CC_CLKDIV_JTAG_SHIFT: c_int = 8;
pub const BCMA_CC_CLKDIV_UART: c_uint = 0x000000FF;
pub const BCMA_CC_CAP_EXT: c_uint = 0x00AC		/* Capabilities */;
pub const BCMA_CC_CAP_EXT_SECI_PRESENT: c_uint = 0x00000001;
pub const BCMA_CC_CAP_EXT_GSIO_PRESENT: c_uint = 0x00000002;
pub const BCMA_CC_CAP_EXT_GCI_PRESENT: c_uint = 0x00000004;
pub const BCMA_CC_CAP_EXT_SECI_PUART_PRESENT: c_uint = 0x00000008    /* UART present */;
pub const BCMA_CC_CAP_EXT_AOB_PRESENT: c_uint = 0x00000040;
pub const BCMA_CC_PLLONDELAY: c_uint = 0x00B0		/* Rev >= 4 only */;
pub const BCMA_CC_FREFSELDELAY: c_uint = 0x00B4		/* Rev >= 4 only */;
pub const BCMA_CC_SLOWCLKCTL: c_uint = 0x00B8		/* 6 <= Rev <= 9 only */;
pub const BCMA_CC_SLOWCLKCTL_SRC: c_uint = 0x00000007	/* slow clock source mask */;
pub const BCMA_CC_SLOWCLKCTL_SRC_LPO: c_uint = 0x00000000	/* source of slow clock is LPO */;
pub const BCMA_CC_SLOWCLKCTL_SRC_XTAL: c_uint = 0x00000001	/* source of slow clock is crystal */;
pub const BCMA_CC_SLOECLKCTL_SRC_PCI: c_uint = 0x00000002	/* source of slow clock is PCI */;
pub const BCMA_CC_SLOWCLKCTL_LPOFREQ: c_uint = 0x00000200	/* LPOFreqSel, 1: 160Khz, 0: 32KHz */;
pub const BCMA_CC_SLOWCLKCTL_LPOPD: c_uint = 0x00000400	/* LPOPowerDown, 1: LPO is disabled, 0: LPO is enabled */;
pub const BCMA_CC_SLOWCLKCTL_FSLOW: c_uint = 0x00000800	/* ForceSlowClk, 1: sb/cores running on slow clock, 0: power logic control */;
pub const BCMA_CC_SLOWCLKCTL_IPLL: c_uint = 0x00001000	/* IgnorePllOffReq, 1/0: power logic ignores/honors PLL clock disable requests from core */;
pub const BCMA_CC_SLOWCLKCTL_ENXTAL: c_uint = 0x00002000	/* XtalControlEn, 1/0: power logic does/doesn't disable crystal when appropriate */;
pub const BCMA_CC_SLOWCLKCTL_XTALPU: c_uint = 0x00004000	/* XtalPU (RO), 1/0: crystal running/disabled */;
pub const BCMA_CC_SLOWCLKCTL_CLKDIV: c_uint = 0xFFFF0000	/* ClockDivider (SlowClk = 1/(4+divisor)) */;
pub const BCMA_CC_SLOWCLKCTL_CLKDIV_SHIFT: c_int = 16;
pub const BCMA_CC_SYSCLKCTL: c_uint = 0x00C0		/* Rev >= 3 only */;
pub const BCMA_CC_SYSCLKCTL_IDLPEN: c_uint = 0x00000001	/* ILPen: Enable Idle Low Power */;
pub const BCMA_CC_SYSCLKCTL_ALPEN: c_uint = 0x00000002	/* ALPen: Enable Active Low Power */;
pub const BCMA_CC_SYSCLKCTL_PLLEN: c_uint = 0x00000004	/* ForcePLLOn */;
pub const BCMA_CC_SYSCLKCTL_FORCEALP: c_uint = 0x00000008	/* Force ALP (or HT if ALPen is not set */;
pub const BCMA_CC_SYSCLKCTL_FORCEHT: c_uint = 0x00000010	/* Force HT */;
pub const BCMA_CC_SYSCLKCTL_CLKDIV: c_uint = 0xFFFF0000	/* ClkDiv  (ILP = 1/(4+divisor)) */;
pub const BCMA_CC_SYSCLKCTL_CLKDIV_SHIFT: c_int = 16;
pub const BCMA_CC_CLKSTSTR: c_uint = 0x00C4		/* Rev >= 3 only */;
pub const BCMA_CC_EROM: c_uint = 0x00FC;
pub const BCMA_CC_PCMCIA_CFG: c_uint = 0x0100;
pub const BCMA_CC_PCMCIA_MEMWAIT: c_uint = 0x0104;
pub const BCMA_CC_PCMCIA_ATTRWAIT: c_uint = 0x0108;
pub const BCMA_CC_PCMCIA_IOWAIT: c_uint = 0x010C;
pub const BCMA_CC_IDE_CFG: c_uint = 0x0110;
pub const BCMA_CC_IDE_MEMWAIT: c_uint = 0x0114;
pub const BCMA_CC_IDE_ATTRWAIT: c_uint = 0x0118;
pub const BCMA_CC_IDE_IOWAIT: c_uint = 0x011C;
pub const BCMA_CC_PROG_CFG: c_uint = 0x0120;
pub const BCMA_CC_PROG_WAITCNT: c_uint = 0x0124;
pub const BCMA_CC_FLASH_CFG: c_uint = 0x0128;
pub const BCMA_CC_FLASH_CFG_DS: c_uint = 0x0010	/* Data size, 0=8bit, 1=16bit */;
pub const BCMA_CC_FLASH_WAITCNT: c_uint = 0x012C;
pub const BCMA_CC_SROM_CONTROL: c_uint = 0x0190;
pub const BCMA_CC_SROM_CONTROL_START: c_uint = 0x80000000;
pub const BCMA_CC_SROM_CONTROL_BUSY: c_uint = 0x80000000;
pub const BCMA_CC_SROM_CONTROL_OPCODE: c_uint = 0x60000000;
pub const BCMA_CC_SROM_CONTROL_OP_READ: c_uint = 0x00000000;
pub const BCMA_CC_SROM_CONTROL_OP_WRITE: c_uint = 0x20000000;
pub const BCMA_CC_SROM_CONTROL_OP_WRDIS: c_uint = 0x40000000;
pub const BCMA_CC_SROM_CONTROL_OP_WREN: c_uint = 0x60000000;
pub const BCMA_CC_SROM_CONTROL_OTPSEL: c_uint = 0x00000010;
pub const BCMA_CC_SROM_CONTROL_OTP_PRESENT: c_uint = 0x00000020;
pub const BCMA_CC_SROM_CONTROL_LOCK: c_uint = 0x00000008;
pub const BCMA_CC_SROM_CONTROL_SIZE_MASK: c_uint = 0x00000006;
pub const BCMA_CC_SROM_CONTROL_SIZE_1K: c_uint = 0x00000000;
pub const BCMA_CC_SROM_CONTROL_SIZE_4K: c_uint = 0x00000002;
pub const BCMA_CC_SROM_CONTROL_SIZE_16K: c_uint = 0x00000004;
pub const BCMA_CC_SROM_CONTROL_SIZE_SHIFT: c_int = 1;
pub const BCMA_CC_SROM_CONTROL_PRESENT: c_uint = 0x00000001;
// Block 0x140 - 0x190 registers are chipset specific
pub const BCMA_CC_4706_FLASHSCFG: c_uint = 0x18C		/* Flash struct configuration */;
pub const BCMA_CC_4706_FLASHSCFG_MASK: c_uint = 0x000000ff;
pub const BCMA_CC_4706_FLASHSCFG_SF1: c_uint = 0x00000001	/* 2nd serial flash present */;
pub const BCMA_CC_4706_FLASHSCFG_PF1: c_uint = 0x00000002	/* 2nd parallel flash present */;
pub const BCMA_CC_4706_FLASHSCFG_SF1_TYPE: c_uint = 0x00000004	/* 2nd serial flash type : 0 : ST, 1 : Atmel */;
pub const BCMA_CC_4706_FLASHSCFG_NF1: c_uint = 0x00000008	/* 2nd NAND flash present */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_MASK: c_uint = 0x000000f0;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_4MB: c_uint = 0x00000010	/* 4MB */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_8MB: c_uint = 0x00000020	/* 8MB */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_16MB: c_uint = 0x00000030	/* 16MB */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_32MB: c_uint = 0x00000040	/* 32MB */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_64MB: c_uint = 0x00000050	/* 64MB */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_128MB: c_uint = 0x00000060	/* 128MB */;
pub const BCMA_CC_4706_FLASHSCFG_1ST_MADDR_SEG_256MB: c_uint = 0x00000070	/* 256MB */;
// NAND flash registers for BCM4706 (corerev = 31)
pub const BCMA_CC_NFLASH_CTL: c_uint = 0x01A0;
pub const BCMA_CC_NFLASH_CTL_ERR: c_uint = 0x08000000;
pub const BCMA_CC_NFLASH_CONF: c_uint = 0x01A4;
pub const BCMA_CC_NFLASH_COL_ADDR: c_uint = 0x01A8;
pub const BCMA_CC_NFLASH_ROW_ADDR: c_uint = 0x01AC;
pub const BCMA_CC_NFLASH_DATA: c_uint = 0x01B0;
pub const BCMA_CC_NFLASH_WAITCNT0: c_uint = 0x01B4;
// 0x1E0 is defined as shared BCMA_CLKCTLST
pub const BCMA_CC_HW_WORKAROUND: c_uint = 0x01E4 /* Hardware workaround (rev >= 20) */;
pub const BCMA_CC_UART0_DATA: c_uint = 0x0300;
pub const BCMA_CC_UART0_IMR: c_uint = 0x0304;
pub const BCMA_CC_UART0_FCR: c_uint = 0x0308;
pub const BCMA_CC_UART0_LCR: c_uint = 0x030C;
pub const BCMA_CC_UART0_MCR: c_uint = 0x0310;
pub const BCMA_CC_UART0_LSR: c_uint = 0x0314;
pub const BCMA_CC_UART0_MSR: c_uint = 0x0318;
pub const BCMA_CC_UART0_SCRATCH: c_uint = 0x031C;
pub const BCMA_CC_UART1_DATA: c_uint = 0x0400;
pub const BCMA_CC_UART1_IMR: c_uint = 0x0404;
pub const BCMA_CC_UART1_FCR: c_uint = 0x0408;
pub const BCMA_CC_UART1_LCR: c_uint = 0x040C;
pub const BCMA_CC_UART1_MCR: c_uint = 0x0410;
pub const BCMA_CC_UART1_LSR: c_uint = 0x0414;
pub const BCMA_CC_UART1_MSR: c_uint = 0x0418;
pub const BCMA_CC_UART1_SCRATCH: c_uint = 0x041C;
// PMU registers (rev >= 20)
pub const BCMA_CC_PMU_CTL: c_uint = 0x0600 /* PMU control */;
pub const BCMA_CC_PMU_CTL_ILP_DIV: c_uint = 0xFFFF0000 /* ILP div mask */;
pub const BCMA_CC_PMU_CTL_ILP_DIV_SHIFT: c_int = 16;
pub const BCMA_CC_PMU_CTL_RES: c_uint = 0x00006000 /* reset control mask */;
pub const BCMA_CC_PMU_CTL_RES_SHIFT: c_int = 13;
pub const BCMA_CC_PMU_CTL_RES_RELOAD: c_uint = 0x2	/* reload POR values */;
pub const BCMA_CC_PMU_CTL_PLL_UPD: c_uint = 0x00000400;
pub const BCMA_CC_PMU_CTL_NOILPONW: c_uint = 0x00000200 /* No ILP on wait */;
pub const BCMA_CC_PMU_CTL_HTREQEN: c_uint = 0x00000100 /* HT req enable */;
pub const BCMA_CC_PMU_CTL_ALPREQEN: c_uint = 0x00000080 /* ALP req enable */;
pub const BCMA_CC_PMU_CTL_XTALFREQ: c_uint = 0x0000007C /* Crystal freq */;
pub const BCMA_CC_PMU_CTL_XTALFREQ_SHIFT: c_int = 2;
pub const BCMA_CC_PMU_CTL_ILPDIVEN: c_uint = 0x00000002 /* ILP div enable */;
pub const BCMA_CC_PMU_CTL_LPOSEL: c_uint = 0x00000001 /* LPO sel */;
pub const BCMA_CC_PMU_CAP: c_uint = 0x0604 /* PMU capabilities */;
pub const BCMA_CC_PMU_CAP_REVISION: c_uint = 0x000000FF /* Revision mask */;
pub const BCMA_CC_PMU_STAT: c_uint = 0x0608 /* PMU status */;
pub const BCMA_CC_PMU_STAT_EXT_LPO_AVAIL: c_uint = 0x00000100;
pub const BCMA_CC_PMU_STAT_WDRESET: c_uint = 0x00000080;
pub const BCMA_CC_PMU_STAT_INTPEND: c_uint = 0x00000040 /* Interrupt pending */;
pub const BCMA_CC_PMU_STAT_SBCLKST: c_uint = 0x00000030 /* Backplane clock status? */;
pub const BCMA_CC_PMU_STAT_HAVEALP: c_uint = 0x00000008 /* ALP available */;
pub const BCMA_CC_PMU_STAT_HAVEHT: c_uint = 0x00000004 /* HT available */;
pub const BCMA_CC_PMU_STAT_RESINIT: c_uint = 0x00000003 /* Res init */;
pub const BCMA_CC_PMU_RES_STAT: c_uint = 0x060C /* PMU res status */;
pub const BCMA_CC_PMU_RES_PEND: c_uint = 0x0610 /* PMU res pending */;
pub const BCMA_CC_PMU_TIMER: c_uint = 0x0614 /* PMU timer */;
pub const BCMA_CC_PMU_MINRES_MSK: c_uint = 0x0618 /* PMU min res mask */;
pub const BCMA_CC_PMU_MAXRES_MSK: c_uint = 0x061C /* PMU max res mask */;
pub const BCMA_CC_PMU_RES_TABSEL: c_uint = 0x0620 /* PMU res table sel */;
pub const BCMA_CC_PMU_RES_DEPMSK: c_uint = 0x0624 /* PMU res dep mask */;
pub const BCMA_CC_PMU_RES_UPDNTM: c_uint = 0x0628 /* PMU res updown timer */;
pub const BCMA_CC_PMU_RES_TIMER: c_uint = 0x062C /* PMU res timer */;
pub const BCMA_CC_PMU_CLKSTRETCH: c_uint = 0x0630 /* PMU clockstretch */;
pub const BCMA_CC_PMU_WATCHDOG: c_uint = 0x0634 /* PMU watchdog */;
pub const BCMA_CC_PMU_RES_REQTS: c_uint = 0x0640 /* PMU res req timer sel */;
pub const BCMA_CC_PMU_RES_REQT: c_uint = 0x0644 /* PMU res req timer */;
pub const BCMA_CC_PMU_RES_REQM: c_uint = 0x0648 /* PMU res req mask */;
pub const BCMA_CC_PMU_CHIPCTL_ADDR: c_uint = 0x0650;
pub const BCMA_CC_PMU_CHIPCTL_DATA: c_uint = 0x0654;
pub const BCMA_CC_PMU_REGCTL_ADDR: c_uint = 0x0658;
pub const BCMA_CC_PMU_REGCTL_DATA: c_uint = 0x065C;
pub const BCMA_CC_PMU_PLLCTL_ADDR: c_uint = 0x0660;
pub const BCMA_CC_PMU_PLLCTL_DATA: c_uint = 0x0664;
pub const BCMA_CC_PMU_STRAPOPT: c_uint = 0x0668 /* (corerev >= 28) */;
pub const BCMA_CC_PMU_XTAL_FREQ: c_uint = 0x066C /* (pmurev >= 10) */;
pub const BCMA_CC_PMU_XTAL_FREQ_ILPCTL_MASK: c_uint = 0x00001FFF;
pub const BCMA_CC_PMU_XTAL_FREQ_MEASURE_MASK: c_uint = 0x80000000;
pub const BCMA_CC_PMU_XTAL_FREQ_MEASURE_SHIFT: c_int = 31;
pub const BCMA_CC_SPROM: c_uint = 0x0800 /* SPROM beginning */;
// NAND flash MLC controller registers (corerev >= 38)
pub const BCMA_CC_NAND_REVISION: c_uint = 0x0C00;
pub const BCMA_CC_NAND_CMD_START: c_uint = 0x0C04;
pub const BCMA_CC_NAND_CMD_ADDR_X: c_uint = 0x0C08;
pub const BCMA_CC_NAND_CMD_ADDR: c_uint = 0x0C0C;
pub const BCMA_CC_NAND_CMD_END_ADDR: c_uint = 0x0C10;
pub const BCMA_CC_NAND_CS_NAND_SELECT: c_uint = 0x0C14;
pub const BCMA_CC_NAND_CS_NAND_XOR: c_uint = 0x0C18;
pub const BCMA_CC_NAND_SPARE_RD0: c_uint = 0x0C20;
pub const BCMA_CC_NAND_SPARE_RD4: c_uint = 0x0C24;
pub const BCMA_CC_NAND_SPARE_RD8: c_uint = 0x0C28;
pub const BCMA_CC_NAND_SPARE_RD12: c_uint = 0x0C2C;
pub const BCMA_CC_NAND_SPARE_WR0: c_uint = 0x0C30;
pub const BCMA_CC_NAND_SPARE_WR4: c_uint = 0x0C34;
pub const BCMA_CC_NAND_SPARE_WR8: c_uint = 0x0C38;
pub const BCMA_CC_NAND_SPARE_WR12: c_uint = 0x0C3C;
pub const BCMA_CC_NAND_ACC_CONTROL: c_uint = 0x0C40;
pub const BCMA_CC_NAND_CONFIG: c_uint = 0x0C48;
pub const BCMA_CC_NAND_TIMING_1: c_uint = 0x0C50;
pub const BCMA_CC_NAND_TIMING_2: c_uint = 0x0C54;
pub const BCMA_CC_NAND_SEMAPHORE: c_uint = 0x0C58;
pub const BCMA_CC_NAND_DEVID: c_uint = 0x0C60;
pub const BCMA_CC_NAND_DEVID_X: c_uint = 0x0C64;
pub const BCMA_CC_NAND_BLOCK_LOCK_STATUS: c_uint = 0x0C68;
pub const BCMA_CC_NAND_INTFC_STATUS: c_uint = 0x0C6C;
pub const BCMA_CC_NAND_ECC_CORR_ADDR_X: c_uint = 0x0C70;
pub const BCMA_CC_NAND_ECC_CORR_ADDR: c_uint = 0x0C74;
pub const BCMA_CC_NAND_ECC_UNC_ADDR_X: c_uint = 0x0C78;
pub const BCMA_CC_NAND_ECC_UNC_ADDR: c_uint = 0x0C7C;
pub const BCMA_CC_NAND_READ_ERROR_COUNT: c_uint = 0x0C80;
pub const BCMA_CC_NAND_CORR_STAT_THRESHOLD: c_uint = 0x0C84;
pub const BCMA_CC_NAND_READ_ADDR_X: c_uint = 0x0C90;
pub const BCMA_CC_NAND_READ_ADDR: c_uint = 0x0C94;
pub const BCMA_CC_NAND_PAGE_PROGRAM_ADDR_X: c_uint = 0x0C98;
pub const BCMA_CC_NAND_PAGE_PROGRAM_ADDR: c_uint = 0x0C9C;
pub const BCMA_CC_NAND_COPY_BACK_ADDR_X: c_uint = 0x0CA0;
pub const BCMA_CC_NAND_COPY_BACK_ADDR: c_uint = 0x0CA4;
pub const BCMA_CC_NAND_BLOCK_ERASE_ADDR_X: c_uint = 0x0CA8;
pub const BCMA_CC_NAND_BLOCK_ERASE_ADDR: c_uint = 0x0CAC;
pub const BCMA_CC_NAND_INV_READ_ADDR_X: c_uint = 0x0CB0;
pub const BCMA_CC_NAND_INV_READ_ADDR: c_uint = 0x0CB4;
pub const BCMA_CC_NAND_BLK_WR_PROTECT: c_uint = 0x0CC0;
pub const BCMA_CC_NAND_ACC_CONTROL_CS1: c_uint = 0x0CD0;
pub const BCMA_CC_NAND_CONFIG_CS1: c_uint = 0x0CD4;
pub const BCMA_CC_NAND_TIMING_1_CS1: c_uint = 0x0CD8;
pub const BCMA_CC_NAND_TIMING_2_CS1: c_uint = 0x0CDC;
pub const BCMA_CC_NAND_SPARE_RD16: c_uint = 0x0D30;
pub const BCMA_CC_NAND_SPARE_RD20: c_uint = 0x0D34;
pub const BCMA_CC_NAND_SPARE_RD24: c_uint = 0x0D38;
pub const BCMA_CC_NAND_SPARE_RD28: c_uint = 0x0D3C;
pub const BCMA_CC_NAND_CACHE_ADDR: c_uint = 0x0D40;
pub const BCMA_CC_NAND_CACHE_DATA: c_uint = 0x0D44;
pub const BCMA_CC_NAND_CTRL_CONFIG: c_uint = 0x0D48;
pub const BCMA_CC_NAND_CTRL_STATUS: c_uint = 0x0D4C;
// Divider allocation in 4716/47162/5356
pub const BCMA_CC_PMU5_MAINPLL_CPU: c_int = 1;
pub const BCMA_CC_PMU5_MAINPLL_MEM: c_int = 2;
pub const BCMA_CC_PMU5_MAINPLL_SSB: c_int = 3;
// PLL usage in 4716/47162
pub const BCMA_CC_PMU4716_MAINPLL_PLL0: c_int = 12;
// PLL usage in 5356/5357
pub const BCMA_CC_PMU5356_MAINPLL_PLL0: c_int = 0;
pub const BCMA_CC_PMU5357_MAINPLL_PLL0: c_int = 0;
// 4706 PMU
pub const BCMA_CC_PMU4706_MAINPLL_PLL0: c_int = 0;

pub const BCMA_CC_PMU6_4706_PROC_P2DIV_MASK: c_uint = 0x000f0000;
pub const BCMA_CC_PMU6_4706_PROC_P2DIV_SHIFT: c_int = 16;
pub const BCMA_CC_PMU6_4706_PROC_P1DIV_MASK: c_uint = 0x0000f000;
pub const BCMA_CC_PMU6_4706_PROC_P1DIV_SHIFT: c_int = 12;
pub const BCMA_CC_PMU6_4706_PROC_NDIV_INT_MASK: c_uint = 0x00000ff8;
pub const BCMA_CC_PMU6_4706_PROC_NDIV_INT_SHIFT: c_int = 3;
pub const BCMA_CC_PMU6_4706_PROC_NDIV_MODE_MASK: c_uint = 0x00000007;
pub const BCMA_CC_PMU6_4706_PROC_NDIV_MODE_SHIFT: c_int = 0;
// PMU rev 15
pub const BCMA_CC_PMU15_PLL_PLLCTL0: c_int = 0;
pub const BCMA_CC_PMU15_PLL_PC0_CLKSEL_MASK: c_uint = 0x00000003;
pub const BCMA_CC_PMU15_PLL_PC0_CLKSEL_SHIFT: c_int = 0;
pub const BCMA_CC_PMU15_PLL_PC0_FREQTGT_MASK: c_uint = 0x003FFFFC;
pub const BCMA_CC_PMU15_PLL_PC0_FREQTGT_SHIFT: c_int = 2;
pub const BCMA_CC_PMU15_PLL_PC0_PRESCALE_MASK: c_uint = 0x00C00000;
pub const BCMA_CC_PMU15_PLL_PC0_PRESCALE_SHIFT: c_int = 22;
pub const BCMA_CC_PMU15_PLL_PC0_KPCTRL_MASK: c_uint = 0x07000000;
pub const BCMA_CC_PMU15_PLL_PC0_KPCTRL_SHIFT: c_int = 24;
pub const BCMA_CC_PMU15_PLL_PC0_FCNTCTRL_MASK: c_uint = 0x38000000;
pub const BCMA_CC_PMU15_PLL_PC0_FCNTCTRL_SHIFT: c_int = 27;
pub const BCMA_CC_PMU15_PLL_PC0_FDCMODE_MASK: c_uint = 0x40000000;
pub const BCMA_CC_PMU15_PLL_PC0_FDCMODE_SHIFT: c_int = 30;
pub const BCMA_CC_PMU15_PLL_PC0_CTRLBIAS_MASK: c_uint = 0x80000000;
pub const BCMA_CC_PMU15_PLL_PC0_CTRLBIAS_SHIFT: c_int = 31;
// ALP clock on pre-PMU chips
pub const BCMA_CC_PMU_ALP_CLOCK: c_int = 20000000;
// HT clock for systems with PMU-enabled chipcommon
pub const BCMA_CC_PMU_HT_CLOCK: c_int = 80000000;
// PMU rev 5 (& 6)
pub const BCMA_CC_PPL_P1P2_OFF: c_int = 0;
pub const BCMA_CC_PPL_P1_MASK: c_uint = 0x0f000000;
pub const BCMA_CC_PPL_P1_SHIFT: c_int = 24;
pub const BCMA_CC_PPL_P2_MASK: c_uint = 0x00f00000;
pub const BCMA_CC_PPL_P2_SHIFT: c_int = 20;
pub const BCMA_CC_PPL_M14_OFF: c_int = 1;
pub const BCMA_CC_PPL_MDIV_MASK: c_uint = 0x000000ff;
pub const BCMA_CC_PPL_MDIV_WIDTH: c_int = 8;
pub const BCMA_CC_PPL_NM5_OFF: c_int = 2;
pub const BCMA_CC_PPL_NDIV_MASK: c_uint = 0xfff00000;
pub const BCMA_CC_PPL_NDIV_SHIFT: c_int = 20;
pub const BCMA_CC_PPL_FMAB_OFF: c_int = 3;
pub const BCMA_CC_PPL_MRAT_MASK: c_uint = 0xf0000000;
pub const BCMA_CC_PPL_MRAT_SHIFT: c_int = 28;
pub const BCMA_CC_PPL_ABRAT_MASK: c_uint = 0x08000000;
pub const BCMA_CC_PPL_ABRAT_SHIFT: c_int = 27;
pub const BCMA_CC_PPL_FDIV_MASK: c_uint = 0x07ffffff;
pub const BCMA_CC_PPL_PLLCTL_OFF: c_int = 4;
pub const BCMA_CC_PPL_PCHI_OFF: c_int = 5;
pub const BCMA_CC_PPL_PCHI_MASK: c_uint = 0x0000003f;
pub const BCMA_CC_PMU_PLL_CTL0: c_int = 0;
pub const BCMA_CC_PMU_PLL_CTL1: c_int = 1;
pub const BCMA_CC_PMU_PLL_CTL2: c_int = 2;
pub const BCMA_CC_PMU_PLL_CTL3: c_int = 3;
pub const BCMA_CC_PMU_PLL_CTL4: c_int = 4;
pub const BCMA_CC_PMU_PLL_CTL5: c_int = 5;
pub const BCMA_CC_PMU1_PLL0_PC0_P1DIV_MASK: c_uint = 0x00f00000;
pub const BCMA_CC_PMU1_PLL0_PC0_P1DIV_SHIFT: c_int = 20;
pub const BCMA_CC_PMU1_PLL0_PC2_NDIV_INT_MASK: c_uint = 0x1ff00000;
pub const BCMA_CC_PMU1_PLL0_PC2_NDIV_INT_SHIFT: c_int = 20;
pub const BCMA_CCB_MII_MNG_CTL: c_uint = 0x0000;
pub const BCMA_CCB_MII_MNG_CMD_DATA: c_uint = 0x0004;
// BCM4331 ChipControl numbers.

// 43224 chip-specific ChipControl register bits
pub const BCMA_CCTRL_43224_GPIO_TOGGLE: c_uint = 0x8000		/* gpio[3:0] pins as btcoex or s/w gpio */;
pub const BCMA_CCTRL_43224A0_12MA_LED_DRIVE: c_uint = 0x00F000F0	/* 12 mA drive strength */;
pub const BCMA_CCTRL_43224B0_12MA_LED_DRIVE: c_uint = 0xF0		/* 12 mA drive strength for later 43224s */;
// 4313 Chip specific ChipControl register bits
pub const BCMA_CCTRL_4313_12MA_LED_DRIVE: c_uint = 0x00000007	/* 12 mA drive strengh for later 4313 */;
// BCM5357 ChipControl register bits

// Data for the PMU, if available.
// Check availability with ((struct bcma_chipcommon)->capabilities & BCMA_CC_CAP_PMU)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_chipcommon_pmu {
    pub /: *mut *mut *mut bcma_device core; / Can be separated core or just ChipCommon one,
    pub /: *mut *mut u8 rev; / PMU revision,
    pub /: *mut *mut u32 crystalfreq; / The active crystal frequency (in kHz),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_pflash {
    pub present: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_sflash {
    pub present: bool,
    pub blocksize: u32,
    pub numblocks: u16,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_nflash {
// Must be the fist member for the brcmnand driver to
// de-reference that structure.
//
    pub brcmnand_info: brcmnand_platform_data,
    pub present: bool,
    pub /: *mut *mut bool boot; / This is the flash the SoC boots from,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_serial_port {
    pub regs: *mut c_void,
    pub clockspeed: c_ulong,
    pub irq: c_uint,
    pub baud_base: c_uint,
    pub reg_shift: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_cc {
    pub core: *mut bcma_device,
    pub status: u32,
    pub capabilities: u32,
    pub capabilities_ext: u32,
    pub setup_done:1: u8,
    pub early_setup_done:1: u8,
// Fast Powerup Delay constant
    pub fast_pwrup_delay: u16,
    pub pmu: bcma_chipcommon_pmu,

    pub pflash: bcma_pflash,

    pub sflash: bcma_sflash,

    pub nflash: bcma_nflash,

    pub nr_serial_ports: c_int,
    pub serial_ports: [bcma_serial_port; 4],
    pub ticks_per_ms: u32,
    pub watchdog: *mut platform_device,
// Lock for GPIO register access.
    pub gpio_lock: spinlock_t,

    pub gpio: gpio_chip,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_cc_b {
    pub core: *mut bcma_device,
    pub setup_done:1: u8,
    pub mii: *mut void __iomem,
}

// Register access

// PMU registers access

extern "C" {
    pub fn bcma_chipco_watchdog_timer_set(cc: *mut bcma_drv_cc, ticks: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_get_alp_clock(cc: *mut bcma_drv_cc) -> u32;
}
extern "C" {
    pub fn bcma_chipco_irq_mask(cc: *mut bcma_drv_cc, mask: u32, value: u32);
}
extern "C" {
    pub fn bcma_chipco_irq_status(cc: *mut bcma_drv_cc, mask: u32) -> u32;
}
// Chipcommon GPIO pin access.
extern "C" {
    pub fn bcma_chipco_gpio_in(cc: *mut bcma_drv_cc, mask: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_out(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_outen(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_control(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_intmask(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_polarity(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_pullup(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn bcma_chipco_gpio_pulldown(cc: *mut bcma_drv_cc, mask: u32, value: u32) -> u32;
}
// PMU support
extern "C" {
    pub fn bcma_pmu_spuravoid_pllupdate(cc: *mut bcma_drv_cc, spuravoid: c_int);
}
extern "C" {
    pub fn bcma_pmu_get_bus_clock(cc: *mut bcma_drv_cc) -> u32;
}
extern "C" {
    pub fn bcma_chipco_b_mii_write(ccb: *mut bcma_drv_cc_b, offset: u32, value: u32);
}
