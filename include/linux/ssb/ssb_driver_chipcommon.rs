//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ssb/ssb_driver_chipcommon.h
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
// SonicsSiliconBackplane CHIPCOMMON core hardware definitions
//
// The chipcommon core provides chip identification, SB control,
// jtag, 0/1/2 uarts, clock frequency control, a watchdog interrupt timer,
// gpio interface, extbus, and support for serial and parallel flashes.
//
// Copyright 2005, Broadcom Corporation
// Copyright 2006, Michael Buesch <m@bues.ch>
//
// ChipCommon core registers.
pub const SSB_CHIPCO_CHIPID: c_uint = 0x0000;
pub const SSB_CHIPCO_IDMASK: c_uint = 0x0000FFFF;
pub const SSB_CHIPCO_REVMASK: c_uint = 0x000F0000;
pub const SSB_CHIPCO_REVSHIFT: c_int = 16;
pub const SSB_CHIPCO_PACKMASK: c_uint = 0x00F00000;
pub const SSB_CHIPCO_PACKSHIFT: c_int = 20;
pub const SSB_CHIPCO_NRCORESMASK: c_uint = 0x0F000000;
pub const SSB_CHIPCO_NRCORESSHIFT: c_int = 24;
pub const SSB_CHIPCO_CAP: c_uint = 0x0004		/* Capabilities */;
pub const SSB_CHIPCO_CAP_NRUART: c_uint = 0x00000003	/* # of UARTs */;
pub const SSB_CHIPCO_CAP_MIPSEB: c_uint = 0x00000004	/* MIPS in BigEndian Mode */;
pub const SSB_CHIPCO_CAP_UARTCLK: c_uint = 0x00000018	/* UART clock select */;
pub const SSB_CHIPCO_CAP_UARTCLK_INT: c_uint = 0x00000008	/* UARTs are driven by internal divided clock */;
pub const SSB_CHIPCO_CAP_UARTGPIO: c_uint = 0x00000020	/* UARTs on GPIO 15-12 */;
pub const SSB_CHIPCO_CAP_EXTBUS: c_uint = 0x000000C0	/* External buses present */;
pub const SSB_CHIPCO_CAP_FLASHT: c_uint = 0x00000700	/* Flash Type */;
pub const SSB_CHIPCO_FLASHT_NONE: c_uint = 0x00000000	/* No flash */;
pub const SSB_CHIPCO_FLASHT_STSER: c_uint = 0x00000100	/* ST serial flash */;
pub const SSB_CHIPCO_FLASHT_ATSER: c_uint = 0x00000200	/* Atmel serial flash */;
pub const SSB_CHIPCO_FLASHT_PARA: c_uint = 0x00000700	/* Parallel flash */;
pub const SSB_CHIPCO_CAP_PLLT: c_uint = 0x00038000	/* PLL Type */;
pub const SSB_PLLTYPE_NONE: c_uint = 0x00000000;
pub const SSB_PLLTYPE_1: c_uint = 0x00010000	/* 48Mhz base, 3 dividers */;
pub const SSB_PLLTYPE_2: c_uint = 0x00020000	/* 48Mhz, 4 dividers */;
pub const SSB_PLLTYPE_3: c_uint = 0x00030000	/* 25Mhz, 2 dividers */;
pub const SSB_PLLTYPE_4: c_uint = 0x00008000	/* 48Mhz, 4 dividers */;
pub const SSB_PLLTYPE_5: c_uint = 0x00018000	/* 25Mhz, 4 dividers */;
pub const SSB_PLLTYPE_6: c_uint = 0x00028000	/* 100/200 or 120/240 only */;
pub const SSB_PLLTYPE_7: c_uint = 0x00038000	/* 25Mhz, 4 dividers */;
pub const SSB_CHIPCO_CAP_PCTL: c_uint = 0x00040000	/* Power Control */;
pub const SSB_CHIPCO_CAP_OTPS: c_uint = 0x00380000	/* OTP size */;
pub const SSB_CHIPCO_CAP_OTPS_SHIFT: c_int = 19;
pub const SSB_CHIPCO_CAP_OTPS_BASE: c_int = 5;
pub const SSB_CHIPCO_CAP_JTAGM: c_uint = 0x00400000	/* JTAG master present */;
pub const SSB_CHIPCO_CAP_BROM: c_uint = 0x00800000	/* Internal boot ROM active */;
pub const SSB_CHIPCO_CAP_64BIT: c_uint = 0x08000000	/* 64-bit Backplane */;
pub const SSB_CHIPCO_CAP_PMU: c_uint = 0x10000000	/* PMU available (rev >= 20) */;
pub const SSB_CHIPCO_CAP_ECI: c_uint = 0x20000000	/* ECI available (rev >= 20) */;
pub const SSB_CHIPCO_CAP_SPROM: c_uint = 0x40000000	/* SPROM present */;
pub const SSB_CHIPCO_CORECTL: c_uint = 0x0008;
pub const SSB_CHIPCO_CORECTL_UARTCLK0: c_uint = 0x00000001	/* Drive UART with internal clock */;
pub const SSB_CHIPCO_CORECTL_SE: c_uint = 0x00000002	/* sync clk out enable (corerev >= 3) */;
pub const SSB_CHIPCO_CORECTL_UARTCLKEN: c_uint = 0x00000008	/* UART clock enable (rev >= 21) */;
pub const SSB_CHIPCO_BIST: c_uint = 0x000C;
pub const SSB_CHIPCO_OTPS: c_uint = 0x0010		/* OTP status */;
pub const SSB_CHIPCO_OTPS_PROGFAIL: c_uint = 0x80000000;
pub const SSB_CHIPCO_OTPS_PROTECT: c_uint = 0x00000007;
pub const SSB_CHIPCO_OTPS_HW_PROTECT: c_uint = 0x00000001;
pub const SSB_CHIPCO_OTPS_SW_PROTECT: c_uint = 0x00000002;
pub const SSB_CHIPCO_OTPS_CID_PROTECT: c_uint = 0x00000004;
pub const SSB_CHIPCO_OTPC: c_uint = 0x0014		/* OTP control */;
pub const SSB_CHIPCO_OTPC_RECWAIT: c_uint = 0xFF000000;
pub const SSB_CHIPCO_OTPC_PROGWAIT: c_uint = 0x00FFFF00;
pub const SSB_CHIPCO_OTPC_PRW_SHIFT: c_int = 8;
pub const SSB_CHIPCO_OTPC_MAXFAIL: c_uint = 0x00000038;
pub const SSB_CHIPCO_OTPC_VSEL: c_uint = 0x00000006;
pub const SSB_CHIPCO_OTPC_SELVL: c_uint = 0x00000001;
pub const SSB_CHIPCO_OTPP: c_uint = 0x0018		/* OTP prog */;
pub const SSB_CHIPCO_OTPP_COL: c_uint = 0x000000FF;
pub const SSB_CHIPCO_OTPP_ROW: c_uint = 0x0000FF00;
pub const SSB_CHIPCO_OTPP_ROW_SHIFT: c_int = 8;
pub const SSB_CHIPCO_OTPP_READERR: c_uint = 0x10000000;
pub const SSB_CHIPCO_OTPP_VALUE: c_uint = 0x20000000;
pub const SSB_CHIPCO_OTPP_READ: c_uint = 0x40000000;
pub const SSB_CHIPCO_OTPP_START: c_uint = 0x80000000;
pub const SSB_CHIPCO_OTPP_BUSY: c_uint = 0x80000000;
pub const SSB_CHIPCO_IRQSTAT: c_uint = 0x0020;
pub const SSB_CHIPCO_IRQMASK: c_uint = 0x0024;
pub const SSB_CHIPCO_IRQ_GPIO: c_uint = 0x00000001	/* gpio intr */;
pub const SSB_CHIPCO_IRQ_EXT: c_uint = 0x00000002	/* ro: ext intr pin (corerev >= 3) */;
pub const SSB_CHIPCO_IRQ_WDRESET: c_uint = 0x80000000	/* watchdog reset occurred */;
pub const SSB_CHIPCO_CHIPCTL: c_uint = 0x0028		/* Rev >= 11 only */;
pub const SSB_CHIPCO_CHIPSTAT: c_uint = 0x002C		/* Rev >= 11 only */;
pub const SSB_CHIPCO_JCMD: c_uint = 0x0030		/* Rev >= 10 only */;
pub const SSB_CHIPCO_JCMD_START: c_uint = 0x80000000;
pub const SSB_CHIPCO_JCMD_BUSY: c_uint = 0x80000000;
pub const SSB_CHIPCO_JCMD_PAUSE: c_uint = 0x40000000;
pub const SSB_CHIPCO_JCMD0_ACC_MASK: c_uint = 0x0000F000;
pub const SSB_CHIPCO_JCMD0_ACC_IRDR: c_uint = 0x00000000;
pub const SSB_CHIPCO_JCMD0_ACC_DR: c_uint = 0x00001000;
pub const SSB_CHIPCO_JCMD0_ACC_IR: c_uint = 0x00002000;
pub const SSB_CHIPCO_JCMD0_ACC_RESET: c_uint = 0x00003000;
pub const SSB_CHIPCO_JCMD0_ACC_IRPDR: c_uint = 0x00004000;
pub const SSB_CHIPCO_JCMD0_ACC_PDR: c_uint = 0x00005000;
pub const SSB_CHIPCO_JCMD0_IRW_MASK: c_uint = 0x00000F00;
pub const SSB_CHIPCO_JCMD_ACC_MASK: c_uint = 0x000F0000	/* Changes for corerev 11 */;
pub const SSB_CHIPCO_JCMD_ACC_IRDR: c_uint = 0x00000000;
pub const SSB_CHIPCO_JCMD_ACC_DR: c_uint = 0x00010000;
pub const SSB_CHIPCO_JCMD_ACC_IR: c_uint = 0x00020000;
pub const SSB_CHIPCO_JCMD_ACC_RESET: c_uint = 0x00030000;
pub const SSB_CHIPCO_JCMD_ACC_IRPDR: c_uint = 0x00040000;
pub const SSB_CHIPCO_JCMD_ACC_PDR: c_uint = 0x00050000;
pub const SSB_CHIPCO_JCMD_IRW_MASK: c_uint = 0x00001F00;
pub const SSB_CHIPCO_JCMD_IRW_SHIFT: c_int = 8;
pub const SSB_CHIPCO_JCMD_DRW_MASK: c_uint = 0x0000003F;
pub const SSB_CHIPCO_JIR: c_uint = 0x0034		/* Rev >= 10 only */;
pub const SSB_CHIPCO_JDR: c_uint = 0x0038		/* Rev >= 10 only */;
pub const SSB_CHIPCO_JCTL: c_uint = 0x003C		/* Rev >= 10 only */;

pub const SSB_CHIPCO_FLASHCTL: c_uint = 0x0040;
pub const SSB_CHIPCO_FLASHCTL_START: c_uint = 0x80000000;

pub const SSB_CHIPCO_FLASHADDR: c_uint = 0x0044;
pub const SSB_CHIPCO_FLASHDATA: c_uint = 0x0048;
pub const SSB_CHIPCO_BCAST_ADDR: c_uint = 0x0050;
pub const SSB_CHIPCO_BCAST_DATA: c_uint = 0x0054;
pub const SSB_CHIPCO_GPIOPULLUP: c_uint = 0x0058		/* Rev >= 20 only */;
pub const SSB_CHIPCO_GPIOPULLDOWN: c_uint = 0x005C		/* Rev >= 20 only */;
pub const SSB_CHIPCO_GPIOIN: c_uint = 0x0060;
pub const SSB_CHIPCO_GPIOOUT: c_uint = 0x0064;
pub const SSB_CHIPCO_GPIOOUTEN: c_uint = 0x0068;
pub const SSB_CHIPCO_GPIOCTL: c_uint = 0x006C;
pub const SSB_CHIPCO_GPIOPOL: c_uint = 0x0070;
pub const SSB_CHIPCO_GPIOIRQ: c_uint = 0x0074;
pub const SSB_CHIPCO_WATCHDOG: c_uint = 0x0080;
pub const SSB_CHIPCO_GPIOTIMER: c_uint = 0x0088		/* LED powersave (corerev >= 16) */;
pub const SSB_CHIPCO_GPIOTIMER_OFFTIME: c_uint = 0x0000FFFF;
pub const SSB_CHIPCO_GPIOTIMER_OFFTIME_SHIFT: c_int = 0;
pub const SSB_CHIPCO_GPIOTIMER_ONTIME: c_uint = 0xFFFF0000;
pub const SSB_CHIPCO_GPIOTIMER_ONTIME_SHIFT: c_int = 16;
pub const SSB_CHIPCO_GPIOTOUTM: c_uint = 0x008C		/* LED powersave (corerev >= 16) */;
pub const SSB_CHIPCO_CLOCK_N: c_uint = 0x0090;
pub const SSB_CHIPCO_CLOCK_SB: c_uint = 0x0094;
pub const SSB_CHIPCO_CLOCK_PCI: c_uint = 0x0098;
pub const SSB_CHIPCO_CLOCK_M2: c_uint = 0x009C;
pub const SSB_CHIPCO_CLOCK_MIPS: c_uint = 0x00A0;
pub const SSB_CHIPCO_CLKDIV: c_uint = 0x00A4		/* Rev >= 3 only */;
pub const SSB_CHIPCO_CLKDIV_SFLASH: c_uint = 0x0F000000;
pub const SSB_CHIPCO_CLKDIV_SFLASH_SHIFT: c_int = 24;
pub const SSB_CHIPCO_CLKDIV_OTP: c_uint = 0x000F0000;
pub const SSB_CHIPCO_CLKDIV_OTP_SHIFT: c_int = 16;
pub const SSB_CHIPCO_CLKDIV_JTAG: c_uint = 0x00000F00;
pub const SSB_CHIPCO_CLKDIV_JTAG_SHIFT: c_int = 8;
pub const SSB_CHIPCO_CLKDIV_UART: c_uint = 0x000000FF;
pub const SSB_CHIPCO_PLLONDELAY: c_uint = 0x00B0		/* Rev >= 4 only */;
pub const SSB_CHIPCO_FREFSELDELAY: c_uint = 0x00B4		/* Rev >= 4 only */;
pub const SSB_CHIPCO_SLOWCLKCTL: c_uint = 0x00B8		/* 6 <= Rev <= 9 only */;
pub const SSB_CHIPCO_SLOWCLKCTL_SRC: c_uint = 0x00000007	/* slow clock source mask */;
pub const SSB_CHIPCO_SLOWCLKCTL_SRC_LPO: c_uint = 0x00000000	/* source of slow clock is LPO */;
pub const SSB_CHIPCO_SLOWCLKCTL_SRC_XTAL: c_uint = 0x00000001	/* source of slow clock is crystal */;
pub const SSB_CHIPCO_SLOECLKCTL_SRC_PCI: c_uint = 0x00000002	/* source of slow clock is PCI */;
pub const SSB_CHIPCO_SLOWCLKCTL_LPOFREQ: c_uint = 0x00000200	/* LPOFreqSel, 1: 160Khz, 0: 32KHz */;
pub const SSB_CHIPCO_SLOWCLKCTL_LPOPD: c_uint = 0x00000400	/* LPOPowerDown, 1: LPO is disabled, 0: LPO is enabled */;
pub const SSB_CHIPCO_SLOWCLKCTL_FSLOW: c_uint = 0x00000800	/* ForceSlowClk, 1: sb/cores running on slow clock, 0: power logic control */;
pub const SSB_CHIPCO_SLOWCLKCTL_IPLL: c_uint = 0x00001000	/* IgnorePllOffReq, 1/0: power logic ignores/honors PLL clock disable requests from core */;
pub const SSB_CHIPCO_SLOWCLKCTL_ENXTAL: c_uint = 0x00002000	/* XtalControlEn, 1/0: power logic does/doesn't disable crystal when appropriate */;
pub const SSB_CHIPCO_SLOWCLKCTL_XTALPU: c_uint = 0x00004000	/* XtalPU (RO), 1/0: crystal running/disabled */;
pub const SSB_CHIPCO_SLOWCLKCTL_CLKDIV: c_uint = 0xFFFF0000	/* ClockDivider (SlowClk = 1/(4+divisor)) */;
pub const SSB_CHIPCO_SLOWCLKCTL_CLKDIV_SHIFT: c_int = 16;
pub const SSB_CHIPCO_SYSCLKCTL: c_uint = 0x00C0		/* Rev >= 3 only */;
pub const SSB_CHIPCO_SYSCLKCTL_IDLPEN: c_uint = 0x00000001	/* ILPen: Enable Idle Low Power */;
pub const SSB_CHIPCO_SYSCLKCTL_ALPEN: c_uint = 0x00000002	/* ALPen: Enable Active Low Power */;
pub const SSB_CHIPCO_SYSCLKCTL_PLLEN: c_uint = 0x00000004	/* ForcePLLOn */;
pub const SSB_CHIPCO_SYSCLKCTL_FORCEALP: c_uint = 0x00000008	/* Force ALP (or HT if ALPen is not set */;
pub const SSB_CHIPCO_SYSCLKCTL_FORCEHT: c_uint = 0x00000010	/* Force HT */;
pub const SSB_CHIPCO_SYSCLKCTL_CLKDIV: c_uint = 0xFFFF0000	/* ClkDiv  (ILP = 1/(4+divisor)) */;
pub const SSB_CHIPCO_SYSCLKCTL_CLKDIV_SHIFT: c_int = 16;
pub const SSB_CHIPCO_CLKSTSTR: c_uint = 0x00C4		/* Rev >= 3 only */;
pub const SSB_CHIPCO_PCMCIA_CFG: c_uint = 0x0100;
pub const SSB_CHIPCO_PCMCIA_MEMWAIT: c_uint = 0x0104;
pub const SSB_CHIPCO_PCMCIA_ATTRWAIT: c_uint = 0x0108;
pub const SSB_CHIPCO_PCMCIA_IOWAIT: c_uint = 0x010C;
pub const SSB_CHIPCO_IDE_CFG: c_uint = 0x0110;
pub const SSB_CHIPCO_IDE_MEMWAIT: c_uint = 0x0114;
pub const SSB_CHIPCO_IDE_ATTRWAIT: c_uint = 0x0118;
pub const SSB_CHIPCO_IDE_IOWAIT: c_uint = 0x011C;
pub const SSB_CHIPCO_PROG_CFG: c_uint = 0x0120;
pub const SSB_CHIPCO_PROG_WAITCNT: c_uint = 0x0124;
pub const SSB_CHIPCO_FLASH_CFG: c_uint = 0x0128;
pub const SSB_CHIPCO_FLASH_WAITCNT: c_uint = 0x012C;
pub const SSB_CHIPCO_CLKCTLST: c_uint = 0x01E0 /* Clock control and status (rev >= 20) */;
pub const SSB_CHIPCO_CLKCTLST_FORCEALP: c_uint = 0x00000001 /* Force ALP request */;
pub const SSB_CHIPCO_CLKCTLST_FORCEHT: c_uint = 0x00000002 /* Force HT request */;
pub const SSB_CHIPCO_CLKCTLST_FORCEILP: c_uint = 0x00000004 /* Force ILP request */;
pub const SSB_CHIPCO_CLKCTLST_HAVEALPREQ: c_uint = 0x00000008 /* ALP available request */;
pub const SSB_CHIPCO_CLKCTLST_HAVEHTREQ: c_uint = 0x00000010 /* HT available request */;
pub const SSB_CHIPCO_CLKCTLST_HWCROFF: c_uint = 0x00000020 /* Force HW clock request off */;
pub const SSB_CHIPCO_CLKCTLST_HAVEALP: c_uint = 0x00010000 /* ALP available */;
pub const SSB_CHIPCO_CLKCTLST_HAVEHT: c_uint = 0x00020000 /* HT available */;
pub const SSB_CHIPCO_CLKCTLST_4328A0_HAVEHT: c_uint = 0x00010000 /* 4328a0 has reversed bits */;
pub const SSB_CHIPCO_CLKCTLST_4328A0_HAVEALP: c_uint = 0x00020000 /* 4328a0 has reversed bits */;
pub const SSB_CHIPCO_HW_WORKAROUND: c_uint = 0x01E4 /* Hardware workaround (rev >= 20) */;
pub const SSB_CHIPCO_UART0_DATA: c_uint = 0x0300;
pub const SSB_CHIPCO_UART0_IMR: c_uint = 0x0304;
pub const SSB_CHIPCO_UART0_FCR: c_uint = 0x0308;
pub const SSB_CHIPCO_UART0_LCR: c_uint = 0x030C;
pub const SSB_CHIPCO_UART0_MCR: c_uint = 0x0310;
pub const SSB_CHIPCO_UART0_LSR: c_uint = 0x0314;
pub const SSB_CHIPCO_UART0_MSR: c_uint = 0x0318;
pub const SSB_CHIPCO_UART0_SCRATCH: c_uint = 0x031C;
pub const SSB_CHIPCO_UART1_DATA: c_uint = 0x0400;
pub const SSB_CHIPCO_UART1_IMR: c_uint = 0x0404;
pub const SSB_CHIPCO_UART1_FCR: c_uint = 0x0408;
pub const SSB_CHIPCO_UART1_LCR: c_uint = 0x040C;
pub const SSB_CHIPCO_UART1_MCR: c_uint = 0x0410;
pub const SSB_CHIPCO_UART1_LSR: c_uint = 0x0414;
pub const SSB_CHIPCO_UART1_MSR: c_uint = 0x0418;
pub const SSB_CHIPCO_UART1_SCRATCH: c_uint = 0x041C;
// PMU registers (rev >= 20)
pub const SSB_CHIPCO_PMU_CTL: c_uint = 0x0600 /* PMU control */;
pub const SSB_CHIPCO_PMU_CTL_ILP_DIV: c_uint = 0xFFFF0000 /* ILP div mask */;
pub const SSB_CHIPCO_PMU_CTL_ILP_DIV_SHIFT: c_int = 16;
pub const SSB_CHIPCO_PMU_CTL_PLL_UPD: c_uint = 0x00000400;
pub const SSB_CHIPCO_PMU_CTL_NOILPONW: c_uint = 0x00000200 /* No ILP on wait */;
pub const SSB_CHIPCO_PMU_CTL_HTREQEN: c_uint = 0x00000100 /* HT req enable */;
pub const SSB_CHIPCO_PMU_CTL_ALPREQEN: c_uint = 0x00000080 /* ALP req enable */;
pub const SSB_CHIPCO_PMU_CTL_XTALFREQ: c_uint = 0x0000007C /* Crystal freq */;
pub const SSB_CHIPCO_PMU_CTL_XTALFREQ_SHIFT: c_int = 2;
pub const SSB_CHIPCO_PMU_CTL_ILPDIVEN: c_uint = 0x00000002 /* ILP div enable */;
pub const SSB_CHIPCO_PMU_CTL_LPOSEL: c_uint = 0x00000001 /* LPO sel */;
pub const SSB_CHIPCO_PMU_CAP: c_uint = 0x0604 /* PMU capabilities */;
pub const SSB_CHIPCO_PMU_CAP_REVISION: c_uint = 0x000000FF /* Revision mask */;
pub const SSB_CHIPCO_PMU_STAT: c_uint = 0x0608 /* PMU status */;
pub const SSB_CHIPCO_PMU_STAT_INTPEND: c_uint = 0x00000040 /* Interrupt pending */;
pub const SSB_CHIPCO_PMU_STAT_SBCLKST: c_uint = 0x00000030 /* Backplane clock status? */;
pub const SSB_CHIPCO_PMU_STAT_HAVEALP: c_uint = 0x00000008 /* ALP available */;
pub const SSB_CHIPCO_PMU_STAT_HAVEHT: c_uint = 0x00000004 /* HT available */;
pub const SSB_CHIPCO_PMU_STAT_RESINIT: c_uint = 0x00000003 /* Res init */;
pub const SSB_CHIPCO_PMU_RES_STAT: c_uint = 0x060C /* PMU res status */;
pub const SSB_CHIPCO_PMU_RES_PEND: c_uint = 0x0610 /* PMU res pending */;
pub const SSB_CHIPCO_PMU_TIMER: c_uint = 0x0614 /* PMU timer */;
pub const SSB_CHIPCO_PMU_MINRES_MSK: c_uint = 0x0618 /* PMU min res mask */;
pub const SSB_CHIPCO_PMU_MAXRES_MSK: c_uint = 0x061C /* PMU max res mask */;
pub const SSB_CHIPCO_PMU_RES_TABSEL: c_uint = 0x0620 /* PMU res table sel */;
pub const SSB_CHIPCO_PMU_RES_DEPMSK: c_uint = 0x0624 /* PMU res dep mask */;
pub const SSB_CHIPCO_PMU_RES_UPDNTM: c_uint = 0x0628 /* PMU res updown timer */;
pub const SSB_CHIPCO_PMU_RES_TIMER: c_uint = 0x062C /* PMU res timer */;
pub const SSB_CHIPCO_PMU_CLKSTRETCH: c_uint = 0x0630 /* PMU clockstretch */;
pub const SSB_CHIPCO_PMU_WATCHDOG: c_uint = 0x0634 /* PMU watchdog */;
pub const SSB_CHIPCO_PMU_RES_REQTS: c_uint = 0x0640 /* PMU res req timer sel */;
pub const SSB_CHIPCO_PMU_RES_REQT: c_uint = 0x0644 /* PMU res req timer */;
pub const SSB_CHIPCO_PMU_RES_REQM: c_uint = 0x0648 /* PMU res req mask */;
pub const SSB_CHIPCO_CHIPCTL_ADDR: c_uint = 0x0650;
pub const SSB_CHIPCO_CHIPCTL_DATA: c_uint = 0x0654;
pub const SSB_CHIPCO_REGCTL_ADDR: c_uint = 0x0658;
pub const SSB_CHIPCO_REGCTL_DATA: c_uint = 0x065C;
pub const SSB_CHIPCO_PLLCTL_ADDR: c_uint = 0x0660;
pub const SSB_CHIPCO_PLLCTL_DATA: c_uint = 0x0664;
// PMU PLL registers
// PMU rev 0 PLL registers
pub const SSB_PMU0_PLLCTL0: c_int = 0;
pub const SSB_PMU0_PLLCTL0_PDIV_MSK: c_uint = 0x00000001;

pub const SSB_PMU0_PLLCTL1: c_int = 1;
pub const SSB_PMU0_PLLCTL1_WILD_IMSK: c_uint = 0xF0000000 /* Wild int mask (low nibble) */;
pub const SSB_PMU0_PLLCTL1_WILD_IMSK_SHIFT: c_int = 28;
pub const SSB_PMU0_PLLCTL1_WILD_FMSK: c_uint = 0x0FFFFF00 /* Wild frac mask */;
pub const SSB_PMU0_PLLCTL1_WILD_FMSK_SHIFT: c_int = 8;
pub const SSB_PMU0_PLLCTL1_STOPMOD: c_uint = 0x00000040 /* Stop mod */;
pub const SSB_PMU0_PLLCTL2: c_int = 2;
pub const SSB_PMU0_PLLCTL2_WILD_IMSKHI: c_uint = 0x0000000F /* Wild int mask (high nibble) */;
pub const SSB_PMU0_PLLCTL2_WILD_IMSKHI_SHIFT: c_int = 0;
// PMU rev 1 PLL registers
pub const SSB_PMU1_PLLCTL0: c_int = 0;
pub const SSB_PMU1_PLLCTL0_P1DIV: c_uint = 0x00F00000 /* P1 div */;
pub const SSB_PMU1_PLLCTL0_P1DIV_SHIFT: c_int = 20;
pub const SSB_PMU1_PLLCTL0_P2DIV: c_uint = 0x0F000000 /* P2 div */;
pub const SSB_PMU1_PLLCTL0_P2DIV_SHIFT: c_int = 24;
pub const SSB_PMU1_PLLCTL1: c_int = 1;
pub const SSB_PMU1_PLLCTL1_M1DIV: c_uint = 0x000000FF /* M1 div */;
pub const SSB_PMU1_PLLCTL1_M1DIV_SHIFT: c_int = 0;
pub const SSB_PMU1_PLLCTL1_M2DIV: c_uint = 0x0000FF00 /* M2 div */;
pub const SSB_PMU1_PLLCTL1_M2DIV_SHIFT: c_int = 8;
pub const SSB_PMU1_PLLCTL1_M3DIV: c_uint = 0x00FF0000 /* M3 div */;
pub const SSB_PMU1_PLLCTL1_M3DIV_SHIFT: c_int = 16;
pub const SSB_PMU1_PLLCTL1_M4DIV: c_uint = 0xFF000000 /* M4 div */;
pub const SSB_PMU1_PLLCTL1_M4DIV_SHIFT: c_int = 24;
pub const SSB_PMU1_PLLCTL2: c_int = 2;
pub const SSB_PMU1_PLLCTL2_M5DIV: c_uint = 0x000000FF /* M5 div */;
pub const SSB_PMU1_PLLCTL2_M5DIV_SHIFT: c_int = 0;
pub const SSB_PMU1_PLLCTL2_M6DIV: c_uint = 0x0000FF00 /* M6 div */;
pub const SSB_PMU1_PLLCTL2_M6DIV_SHIFT: c_int = 8;
pub const SSB_PMU1_PLLCTL2_NDIVMODE: c_uint = 0x000E0000 /* NDIV mode */;
pub const SSB_PMU1_PLLCTL2_NDIVMODE_SHIFT: c_int = 17;
pub const SSB_PMU1_PLLCTL2_NDIVINT: c_uint = 0x1FF00000 /* NDIV int */;
pub const SSB_PMU1_PLLCTL2_NDIVINT_SHIFT: c_int = 20;
pub const SSB_PMU1_PLLCTL3: c_int = 3;
pub const SSB_PMU1_PLLCTL3_NDIVFRAC: c_uint = 0x00FFFFFF /* NDIV frac */;
pub const SSB_PMU1_PLLCTL3_NDIVFRAC_SHIFT: c_int = 0;
pub const SSB_PMU1_PLLCTL4: c_int = 4;
pub const SSB_PMU1_PLLCTL5: c_int = 5;
pub const SSB_PMU1_PLLCTL5_CLKDRV: c_uint = 0xFFFFFF00 /* clk drv */;
pub const SSB_PMU1_PLLCTL5_CLKDRV_SHIFT: c_int = 8;
// BCM4312 PLL resource numbers.
pub const SSB_PMURES_4312_SWITCHER_BURST: c_int = 0;
pub const SSB_PMURES_4312_SWITCHER_PWM: c_int = 1;
pub const SSB_PMURES_4312_PA_REF_LDO: c_int = 2;
pub const SSB_PMURES_4312_CORE_LDO_BURST: c_int = 3;
pub const SSB_PMURES_4312_CORE_LDO_PWM: c_int = 4;
pub const SSB_PMURES_4312_RADIO_LDO: c_int = 5;
pub const SSB_PMURES_4312_ILP_REQUEST: c_int = 6;
pub const SSB_PMURES_4312_BG_FILTBYP: c_int = 7;
pub const SSB_PMURES_4312_TX_FILTBYP: c_int = 8;
pub const SSB_PMURES_4312_RX_FILTBYP: c_int = 9;
pub const SSB_PMURES_4312_XTAL_PU: c_int = 10;
pub const SSB_PMURES_4312_ALP_AVAIL: c_int = 11;
pub const SSB_PMURES_4312_BB_PLL_FILTBYP: c_int = 12;
pub const SSB_PMURES_4312_RF_PLL_FILTBYP: c_int = 13;
pub const SSB_PMURES_4312_HT_AVAIL: c_int = 14;
// BCM4325 PLL resource numbers.
pub const SSB_PMURES_4325_BUCK_BOOST_BURST: c_int = 0;
pub const SSB_PMURES_4325_CBUCK_BURST: c_int = 1;
pub const SSB_PMURES_4325_CBUCK_PWM: c_int = 2;
pub const SSB_PMURES_4325_CLDO_CBUCK_BURST: c_int = 3;
pub const SSB_PMURES_4325_CLDO_CBUCK_PWM: c_int = 4;
pub const SSB_PMURES_4325_BUCK_BOOST_PWM: c_int = 5;
pub const SSB_PMURES_4325_ILP_REQUEST: c_int = 6;
pub const SSB_PMURES_4325_ABUCK_BURST: c_int = 7;
pub const SSB_PMURES_4325_ABUCK_PWM: c_int = 8;
pub const SSB_PMURES_4325_LNLDO1_PU: c_int = 9;
pub const SSB_PMURES_4325_LNLDO2_PU: c_int = 10;
pub const SSB_PMURES_4325_LNLDO3_PU: c_int = 11;
pub const SSB_PMURES_4325_LNLDO4_PU: c_int = 12;
pub const SSB_PMURES_4325_XTAL_PU: c_int = 13;
pub const SSB_PMURES_4325_ALP_AVAIL: c_int = 14;
pub const SSB_PMURES_4325_RX_PWRSW_PU: c_int = 15;
pub const SSB_PMURES_4325_TX_PWRSW_PU: c_int = 16;
pub const SSB_PMURES_4325_RFPLL_PWRSW_PU: c_int = 17;
pub const SSB_PMURES_4325_LOGEN_PWRSW_PU: c_int = 18;
pub const SSB_PMURES_4325_AFE_PWRSW_PU: c_int = 19;
pub const SSB_PMURES_4325_BBPLL_PWRSW_PU: c_int = 20;
pub const SSB_PMURES_4325_HT_AVAIL: c_int = 21;
// BCM4328 PLL resource numbers.
pub const SSB_PMURES_4328_EXT_SWITCHER_PWM: c_int = 0;
pub const SSB_PMURES_4328_BB_SWITCHER_PWM: c_int = 1;
pub const SSB_PMURES_4328_BB_SWITCHER_BURST: c_int = 2;
pub const SSB_PMURES_4328_BB_EXT_SWITCHER_BURST: c_int = 3;
pub const SSB_PMURES_4328_ILP_REQUEST: c_int = 4;
pub const SSB_PMURES_4328_RADIO_SWITCHER_PWM: c_int = 5;
pub const SSB_PMURES_4328_RADIO_SWITCHER_BURST: c_int = 6;
pub const SSB_PMURES_4328_ROM_SWITCH: c_int = 7;
pub const SSB_PMURES_4328_PA_REF_LDO: c_int = 8;
pub const SSB_PMURES_4328_RADIO_LDO: c_int = 9;
pub const SSB_PMURES_4328_AFE_LDO: c_int = 10;
pub const SSB_PMURES_4328_PLL_LDO: c_int = 11;
pub const SSB_PMURES_4328_BG_FILTBYP: c_int = 12;
pub const SSB_PMURES_4328_TX_FILTBYP: c_int = 13;
pub const SSB_PMURES_4328_RX_FILTBYP: c_int = 14;
pub const SSB_PMURES_4328_XTAL_PU: c_int = 15;
pub const SSB_PMURES_4328_XTAL_EN: c_int = 16;
pub const SSB_PMURES_4328_BB_PLL_FILTBYP: c_int = 17;
pub const SSB_PMURES_4328_RF_PLL_FILTBYP: c_int = 18;
pub const SSB_PMURES_4328_BB_PLL_PU: c_int = 19;
// BCM5354 PLL resource numbers.
pub const SSB_PMURES_5354_EXT_SWITCHER_PWM: c_int = 0;
pub const SSB_PMURES_5354_BB_SWITCHER_PWM: c_int = 1;
pub const SSB_PMURES_5354_BB_SWITCHER_BURST: c_int = 2;
pub const SSB_PMURES_5354_BB_EXT_SWITCHER_BURST: c_int = 3;
pub const SSB_PMURES_5354_ILP_REQUEST: c_int = 4;
pub const SSB_PMURES_5354_RADIO_SWITCHER_PWM: c_int = 5;
pub const SSB_PMURES_5354_RADIO_SWITCHER_BURST: c_int = 6;
pub const SSB_PMURES_5354_ROM_SWITCH: c_int = 7;
pub const SSB_PMURES_5354_PA_REF_LDO: c_int = 8;
pub const SSB_PMURES_5354_RADIO_LDO: c_int = 9;
pub const SSB_PMURES_5354_AFE_LDO: c_int = 10;
pub const SSB_PMURES_5354_PLL_LDO: c_int = 11;
pub const SSB_PMURES_5354_BG_FILTBYP: c_int = 12;
pub const SSB_PMURES_5354_TX_FILTBYP: c_int = 13;
pub const SSB_PMURES_5354_RX_FILTBYP: c_int = 14;
pub const SSB_PMURES_5354_XTAL_PU: c_int = 15;
pub const SSB_PMURES_5354_XTAL_EN: c_int = 16;
pub const SSB_PMURES_5354_BB_PLL_FILTBYP: c_int = 17;
pub const SSB_PMURES_5354_RF_PLL_FILTBYP: c_int = 18;
pub const SSB_PMURES_5354_BB_PLL_PU: c_int = 19;
// Chip specific Chip-Status register contents.
pub const SSB_CHIPCO_CHST_4322_SPROM_EXISTS: c_uint = 0x00000040 /* SPROM present */;
pub const SSB_CHIPCO_CHST_4325_SPROM_OTP_SEL: c_uint = 0x00000003;

pub const SSB_CHIPCO_CHST_4325_SDIO_USB_MODE: c_uint = 0x00000004;
pub const SSB_CHIPCO_CHST_4325_SDIO_USB_MODE_SHIFT: c_int = 2;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALID: c_uint = 0x00000008;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALID_SHIFT: c_int = 3;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALUE: c_uint = 0x000001F0;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALUE_SHIFT: c_int = 4;
pub const SSB_CHIPCO_CHST_4325_PMUTOP_2B: c_uint = 0x00000200 /* 1 for 2b, 0 for to 2a */;
// Macros to determine SPROM presence based on Chip-Status register.

// Clockcontrol masks and values
// SSB_CHIPCO_CLOCK_N
pub const SSB_CHIPCO_CLK_N1: c_uint = 0x0000003F	/* n1 control */;
pub const SSB_CHIPCO_CLK_N2: c_uint = 0x00003F00	/* n2 control */;
pub const SSB_CHIPCO_CLK_N2_SHIFT: c_int = 8;
pub const SSB_CHIPCO_CLK_PLLC: c_uint = 0x000F0000	/* pll control */;
pub const SSB_CHIPCO_CLK_PLLC_SHIFT: c_int = 16;
// SSB_CHIPCO_CLOCK_SB/PCI/UART
pub const SSB_CHIPCO_CLK_M1: c_uint = 0x0000003F	/* m1 control */;
pub const SSB_CHIPCO_CLK_M2: c_uint = 0x00003F00	/* m2 control */;
pub const SSB_CHIPCO_CLK_M2_SHIFT: c_int = 8;
pub const SSB_CHIPCO_CLK_M3: c_uint = 0x003F0000	/* m3 control */;
pub const SSB_CHIPCO_CLK_M3_SHIFT: c_int = 16;
pub const SSB_CHIPCO_CLK_MC: c_uint = 0x1F000000	/* mux control */;
pub const SSB_CHIPCO_CLK_MC_SHIFT: c_int = 24;
// N3M Clock control magic field values
pub const SSB_CHIPCO_CLK_F6_2: c_uint = 0x02		/* A factor of 2 in */;
pub const SSB_CHIPCO_CLK_F6_3: c_uint = 0x03		/* 6-bit fields like */;
pub const SSB_CHIPCO_CLK_F6_4: c_uint = 0x05		/* N1, M1 or M3 */;
pub const SSB_CHIPCO_CLK_F6_5: c_uint = 0x09;
pub const SSB_CHIPCO_CLK_F6_6: c_uint = 0x11;
pub const SSB_CHIPCO_CLK_F6_7: c_uint = 0x21;

pub const SSB_CHIPCO_CLK_MC_BYPASS: c_uint = 0x08;
pub const SSB_CHIPCO_CLK_MC_M1: c_uint = 0x04;
pub const SSB_CHIPCO_CLK_MC_M1M2: c_uint = 0x02;
pub const SSB_CHIPCO_CLK_MC_M1M2M3: c_uint = 0x01;
pub const SSB_CHIPCO_CLK_MC_M1M3: c_uint = 0x11;
// Type 2 Clock control magic field values

pub const SSB_CHIPCO_CLK_T2MC_M1BYP: c_int = 1;
pub const SSB_CHIPCO_CLK_T2MC_M2BYP: c_int = 2;
pub const SSB_CHIPCO_CLK_T2MC_M3BYP: c_int = 4;
// Type 6 Clock control magic field values

// Common clock base

// Clock control values for 200Mhz in 5350
pub const SSB_CHIPCO_CLK_5350_N: c_uint = 0x0311;
pub const SSB_CHIPCO_CLK_5350_M: c_uint = 0x04020009;
// Bits in the config registers
pub const SSB_CHIPCO_CFG_EN: c_uint = 0x0001		/* Enable */;
pub const SSB_CHIPCO_CFG_EXTM: c_uint = 0x000E		/* Extif Mode */;
pub const SSB_CHIPCO_CFG_EXTM_ASYNC: c_uint = 0x0002		/* Async/Parallel flash */;
pub const SSB_CHIPCO_CFG_EXTM_SYNC: c_uint = 0x0004		/* Synchronous */;
pub const SSB_CHIPCO_CFG_EXTM_PCMCIA: c_uint = 0x0008		/* PCMCIA */;
pub const SSB_CHIPCO_CFG_EXTM_IDE: c_uint = 0x000A		/* IDE */;
pub const SSB_CHIPCO_CFG_DS16: c_uint = 0x0010		/* Data size, 0=8bit, 1=16bit */;
pub const SSB_CHIPCO_CFG_CLKDIV: c_uint = 0x0060		/* Sync: Clock divisor */;
pub const SSB_CHIPCO_CFG_CLKEN: c_uint = 0x0080		/* Sync: Clock enable */;
pub const SSB_CHIPCO_CFG_BSTRO: c_uint = 0x0100		/* Sync: Size/Bytestrobe */;
// Flash-specific control/status values
// flashcontrol opcodes for ST flashes
pub const SSB_CHIPCO_FLASHCTL_ST_WREN: c_uint = 0x0006		/* Write Enable */;
pub const SSB_CHIPCO_FLASHCTL_ST_WRDIS: c_uint = 0x0004		/* Write Disable */;
pub const SSB_CHIPCO_FLASHCTL_ST_RDSR: c_uint = 0x0105		/* Read Status Register */;
pub const SSB_CHIPCO_FLASHCTL_ST_WRSR: c_uint = 0x0101		/* Write Status Register */;
pub const SSB_CHIPCO_FLASHCTL_ST_READ: c_uint = 0x0303		/* Read Data Bytes */;
pub const SSB_CHIPCO_FLASHCTL_ST_PP: c_uint = 0x0302		/* Page Program */;
pub const SSB_CHIPCO_FLASHCTL_ST_SE: c_uint = 0x02D8		/* Sector Erase */;
pub const SSB_CHIPCO_FLASHCTL_ST_BE: c_uint = 0x00C7		/* Bulk Erase */;
pub const SSB_CHIPCO_FLASHCTL_ST_DP: c_uint = 0x00B9		/* Deep Power-down */;
pub const SSB_CHIPCO_FLASHCTL_ST_RES: c_uint = 0x03AB		/* Read Electronic Signature */;
pub const SSB_CHIPCO_FLASHCTL_ST_CSA: c_uint = 0x1000		/* Keep chip select asserted */;
pub const SSB_CHIPCO_FLASHCTL_ST_SSE: c_uint = 0x0220		/* Sub-sector Erase */;
// Status register bits for ST flashes
pub const SSB_CHIPCO_FLASHSTA_ST_WIP: c_uint = 0x01		/* Write In Progress */;
pub const SSB_CHIPCO_FLASHSTA_ST_WEL: c_uint = 0x02		/* Write Enable Latch */;
pub const SSB_CHIPCO_FLASHSTA_ST_BP: c_uint = 0x1C		/* Block Protect */;
pub const SSB_CHIPCO_FLASHSTA_ST_BP_SHIFT: c_int = 2;
pub const SSB_CHIPCO_FLASHSTA_ST_SRWD: c_uint = 0x80		/* Status Register Write Disable */;
// flashcontrol opcodes for Atmel flashes
pub const SSB_CHIPCO_FLASHCTL_AT_READ: c_uint = 0x07E8;
pub const SSB_CHIPCO_FLASHCTL_AT_PAGE_READ: c_uint = 0x07D2;

pub const SSB_CHIPCO_FLASHCTL_AT_STATUS: c_uint = 0x01D7;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_WRITE: c_uint = 0x0384;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_WRITE: c_uint = 0x0387;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_ERASE_PRGM: c_uint = 0x0283	/* Erase program */;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_ERASE_PRGM: c_uint = 0x0286	/* Erase program */;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_PROGRAM: c_uint = 0x0288;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_PROGRAM: c_uint = 0x0289;
pub const SSB_CHIPCO_FLASHCTL_AT_PAGE_ERASE: c_uint = 0x0281;
pub const SSB_CHIPCO_FLASHCTL_AT_BLOCK_ERASE: c_uint = 0x0250;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_WRER_PRGM: c_uint = 0x0382	/* Write erase program */;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_WRER_PRGM: c_uint = 0x0385	/* Write erase program */;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_LOAD: c_uint = 0x0253;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_LOAD: c_uint = 0x0255;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_COMPARE: c_uint = 0x0260;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_COMPARE: c_uint = 0x0261;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_REPROGRAM: c_uint = 0x0258;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_REPROGRAM: c_uint = 0x0259;
// Status register bits for Atmel flashes
pub const SSB_CHIPCO_FLASHSTA_AT_READY: c_uint = 0x80;
pub const SSB_CHIPCO_FLASHSTA_AT_MISMATCH: c_uint = 0x40;
pub const SSB_CHIPCO_FLASHSTA_AT_ID: c_uint = 0x38;
pub const SSB_CHIPCO_FLASHSTA_AT_ID_SHIFT: c_int = 3;
// OTP
// OTP regions

// OTP regions (Byte offsets from otp size)

pub const SSB_CHIPCO_OTP_CIDBASE_OFF: c_int = 0;
pub const SSB_CHIPCO_OTP_CIDLIM_OFF: c_int = 8;
// Predefined OTP words (Word offset from otp size)

pub const SSB_CHIPCO_OTP_CID_OFF: c_int = 0;
pub const SSB_CHIPCO_OTP_PKG_OFF: c_int = 1;
pub const SSB_CHIPCO_OTP_FID_OFF: c_int = 2;
pub const SSB_CHIPCO_OTP_RSV_OFF: c_int = 3;
pub const SSB_CHIPCO_OTP_LIM_OFF: c_int = 4;
pub const SSB_CHIPCO_OTP_SIGNATURE: c_uint = 0x578A;
pub const SSB_CHIPCO_OTP_MAGIC: c_uint = 0x4E56;
// Data for the PMU, if available.
// Check availability with ((struct ssb_chipcommon)->capabilities & SSB_CHIPCO_CAP_PMU)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_chipcommon_pmu {
    pub /: *mut *mut u8 rev; / PMU revision,
    pub /: *mut *mut u32 crystalfreq; / The active crystal frequency (in kHz),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_chipcommon {
    pub dev: *mut ssb_device,
    pub capabilities: u32,
    pub status: u32,
// Fast Powerup Delay constant
    pub fast_pwrup_delay: u16,
    pub gpio_lock: spinlock_t,
    pub pmu: ssb_chipcommon_pmu,
    pub ticks_per_ms: u32,
    pub max_timer_ms: u32,
}

// Register access

extern "C" {
    pub fn ssb_chipcommon_init(cc: *mut ssb_chipcommon);
}
extern "C" {
    pub fn ssb_chipco_suspend(cc: *mut ssb_chipcommon);
}
extern "C" {
    pub fn ssb_chipco_resume(cc: *mut ssb_chipcommon);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssb_clkmode {
    SSB_CLKMODE_SLOW,
    SSB_CLKMODE_FAST,
    SSB_CLKMODE_DYNAMIC,
}

extern "C" {
    pub fn ssb_chipco_watchdog_timer_set(cc: *mut ssb_chipcommon, ticks: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_irq_mask(cc: *mut ssb_chipcommon, mask: u32, value: u32);
}
extern "C" {
    pub fn ssb_chipco_irq_status(cc: *mut ssb_chipcommon, mask: u32) -> u32;
}
// Chipcommon GPIO pin access.
extern "C" {
    pub fn ssb_chipco_gpio_in(cc: *mut ssb_chipcommon, mask: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_out(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_outen(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_control(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_intmask(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_polarity(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_pullup(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn ssb_chipco_gpio_pulldown(cc: *mut ssb_chipcommon, mask: u32, value: u32) -> u32;
}

// PMU support
extern "C" {
    pub fn ssb_pmu_init(cc: *mut ssb_chipcommon);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssb_pmu_ldo_volt_id {
    LDO_PAREF = 0,
    LDO_VOLT1,
    LDO_VOLT2,
    LDO_VOLT3,
}

extern "C" {
    pub fn ssb_pmu_set_ldo_paref(cc: *mut ssb_chipcommon, on: bool);
}
extern "C" {
    pub fn ssb_pmu_spuravoid_pllupdate(cc: *mut ssb_chipcommon, spuravoid: c_int);
}
