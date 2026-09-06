//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_regs.h
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
// Some single registers are shared between many cores
// BCMA_CLKCTLST: ChipCommon (rev >= 20), PCIe, 80211
pub const BCMA_CLKCTLST: c_uint = 0x01E0 /* Clock control and status */;
pub const BCMA_CLKCTLST_FORCEALP: c_uint = 0x00000001 /* Force ALP request */;
pub const BCMA_CLKCTLST_FORCEHT: c_uint = 0x00000002 /* Force HT request */;
pub const BCMA_CLKCTLST_FORCEILP: c_uint = 0x00000004 /* Force ILP request */;
pub const BCMA_CLKCTLST_HAVEALPREQ: c_uint = 0x00000008 /* ALP available request */;
pub const BCMA_CLKCTLST_HAVEHTREQ: c_uint = 0x00000010 /* HT available request */;
pub const BCMA_CLKCTLST_HWCROFF: c_uint = 0x00000020 /* Force HW clock request off */;
pub const BCMA_CLKCTLST_HQCLKREQ: c_uint = 0x00000040 /* HQ Clock */;
pub const BCMA_CLKCTLST_EXTRESREQ: c_uint = 0x00000700 /* Mask of external resource requests */;
pub const BCMA_CLKCTLST_EXTRESREQ_SHIFT: c_int = 8;
pub const BCMA_CLKCTLST_HAVEALP: c_uint = 0x00010000 /* ALP available */;
pub const BCMA_CLKCTLST_HAVEHT: c_uint = 0x00020000 /* HT available */;
pub const BCMA_CLKCTLST_BP_ON_ALP: c_uint = 0x00040000 /* RO: running on ALP clock */;
pub const BCMA_CLKCTLST_BP_ON_HT: c_uint = 0x00080000 /* RO: running on HT clock */;
pub const BCMA_CLKCTLST_EXTRESST: c_uint = 0x07000000 /* Mask of external resource status */;
pub const BCMA_CLKCTLST_EXTRESST_SHIFT: c_int = 24;
// Is there any BCM4328 on BCMA bus?
pub const BCMA_CLKCTLST_4328A0_HAVEHT: c_uint = 0x00010000 /* 4328a0 has reversed bits */;
pub const BCMA_CLKCTLST_4328A0_HAVEALP: c_uint = 0x00020000 /* 4328a0 has reversed bits */;
// Agent registers (common for every core)
pub const BCMA_OOB_SEL_OUT_A30: c_uint = 0x0100;
pub const BCMA_IOCTL: c_uint = 0x0408 /* IO control */;
pub const BCMA_IOCTL_CLK: c_uint = 0x0001;
pub const BCMA_IOCTL_FGC: c_uint = 0x0002;
pub const BCMA_IOCTL_CORE_BITS: c_uint = 0x3FFC;
pub const BCMA_IOCTL_PME_EN: c_uint = 0x4000;
pub const BCMA_IOCTL_BIST_EN: c_uint = 0x8000;
pub const BCMA_IOST: c_uint = 0x0500 /* IO status */;
pub const BCMA_IOST_CORE_BITS: c_uint = 0x0FFF;
pub const BCMA_IOST_DMA64: c_uint = 0x1000;
pub const BCMA_IOST_GATED_CLK: c_uint = 0x2000;
pub const BCMA_IOST_BIST_ERROR: c_uint = 0x4000;
pub const BCMA_IOST_BIST_DONE: c_uint = 0x8000;
pub const BCMA_RESET_CTL: c_uint = 0x0800;
pub const BCMA_RESET_CTL_RESET: c_uint = 0x0001;
pub const BCMA_RESET_ST: c_uint = 0x0804;
pub const BCMA_NS_ROM_IOST_BOOT_DEV_MASK: c_uint = 0x0003;
pub const BCMA_NS_ROM_IOST_BOOT_DEV_NOR: c_uint = 0x0000;
pub const BCMA_NS_ROM_IOST_BOOT_DEV_NAND: c_uint = 0x0001;
pub const BCMA_NS_ROM_IOST_BOOT_DEV_ROM: c_uint = 0x0002;
// BCMA PCI config space registers.
pub const BCMA_PCI_PMCSR: c_uint = 0x44;
pub const BCMA_PCI_PE: c_uint = 0x100;
pub const BCMA_PCI_BAR0_WIN: c_uint = 0x80	/* Backplane address space 0 */;
pub const BCMA_PCI_BAR1_WIN: c_uint = 0x84	/* Backplane address space 1 */;
pub const BCMA_PCI_SPROMCTL: c_uint = 0x88	/* SPROM control */;
pub const BCMA_PCI_SPROMCTL_WE: c_uint = 0x10	/* SPROM write enable */;
pub const BCMA_PCI_BAR1_CONTROL: c_uint = 0x8c	/* Address space 1 burst control */;
pub const BCMA_PCI_IRQS: c_uint = 0x90	/* PCI interrupts */;
pub const BCMA_PCI_IRQMASK: c_uint = 0x94	/* PCI IRQ control and mask (pcirev >= 6 only) */;
pub const BCMA_PCI_BACKPLANE_IRQS: c_uint = 0x98	/* Backplane Interrupts */;
pub const BCMA_PCI_BAR0_WIN2: c_uint = 0xAC;
pub const BCMA_PCI_GPIO_IN: c_uint = 0xB0	/* GPIO Input (pcirev >= 3 only) */;
pub const BCMA_PCI_GPIO_OUT: c_uint = 0xB4	/* GPIO Output (pcirev >= 3 only) */;
pub const BCMA_PCI_GPIO_OUT_ENABLE: c_uint = 0xB8	/* GPIO Output Enable/Disable (pcirev >= 3 only) */;
pub const BCMA_PCI_GPIO_SCS: c_uint = 0x10	/* PCI config space bit 4 for 4306c0 slow clock source */;
pub const BCMA_PCI_GPIO_HWRAD: c_uint = 0x20	/* PCI config space GPIO 13 for hw radio disable */;
pub const BCMA_PCI_GPIO_XTAL: c_uint = 0x40	/* PCI config space GPIO 14 for Xtal powerup */;
pub const BCMA_PCI_GPIO_PLL: c_uint = 0x80	/* PCI config space GPIO 15 for PLL powerdown */;
pub const BCMA_PCIE2_BAR0_WIN2: c_uint = 0x70;
// SiliconBackplane Address Map.
// All regions may not exist on all chips.
//
pub const BCMA_SOC_SDRAM_BASE: c_uint = 0x00000000U	/* Physical SDRAM */;
pub const BCMA_SOC_PCI_MEM: c_uint = 0x08000000U	/* Host Mode sb2pcitranslation0 (64 MB) */;

pub const BCMA_SOC_PCI_CFG: c_uint = 0x0c000000U	/* Host Mode sb2pcitranslation1 (64 MB) */;
pub const BCMA_SOC_SDRAM_SWAPPED: c_uint = 0x10000000U	/* Byteswapped Physical SDRAM */;
pub const BCMA_SOC_SDRAM_R2: c_uint = 0x80000000U	/* Region 2 for sdram (512 MB) */;
pub const BCMA_SOC_PCI_DMA: c_uint = 0x40000000U	/* Client Mode sb2pcitranslation2 (1 GB) */;
pub const BCMA_SOC_PCI_DMA2: c_uint = 0x80000000U	/* Client Mode sb2pcitranslation2 (1 GB) */;
pub const BCMA_SOC_PCI_DMA_SZ: c_uint = 0x40000000U	/* Client Mode sb2pcitranslation2 size in bytes */;
pub const BCMA_SOC_PCIE_DMA_L32: c_uint = 0x00000000U	/* PCIE Client Mode sb2pcitranslation2;
// (2 ZettaBytes), low 32 bits
//
pub const BCMA_SOC_PCIE_DMA_H32: c_uint = 0x80000000U	/* PCIE Client Mode sb2pcitranslation2;
// (2 ZettaBytes), high 32 bits
//
pub const BCMA_SOC_PCI1_MEM: c_uint = 0x40000000U	/* Host Mode sb2pcitranslation0 (64 MB) */;
pub const BCMA_SOC_PCI1_CFG: c_uint = 0x44000000U	/* Host Mode sb2pcitranslation1 (64 MB) */;
pub const BCMA_SOC_PCIE1_DMA_H32: c_uint = 0xc0000000U	/* PCIE Client Mode sb2pcitranslation2;
// (2 ZettaBytes), high 32 bits
//
pub const BCMA_SOC_FLASH1: c_uint = 0x1fc00000	/* MIPS Flash Region 1 */;
pub const BCMA_SOC_FLASH1_SZ: c_uint = 0x00400000	/* MIPS Size of Flash Region 1 */;
pub const BCMA_SOC_FLASH2: c_uint = 0x1c000000	/* Flash Region 2 (region 1 shadowed here) */;
pub const BCMA_SOC_FLASH2_SZ: c_uint = 0x02000000	/* Size of Flash Region 2 */;
