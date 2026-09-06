//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/hifn_795x.c
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
// 2007+ Copyright (c) Evgeniy Polyakov <johnpol@2ka.mipt.ru>
// All rights reserved.
//

    static char hifn_pll_ref[sizeof("extNNN")] = "ext";
    module_param_string(hifn_pll_ref, hifn_pll_ref, sizeof(hifn_pll_ref), 0444);
    MODULE_PARM_DESC(hifn_pll_ref,
    "PLL reference clock (pci[freq] or ext[freq], default ext)");
    static atomic_t hifn_dev_number;
pub const ACRYPTO_OP_DECRYPT: c_int = 0;
pub const ACRYPTO_OP_ENCRYPT: c_int = 1;
pub const ACRYPTO_OP_HMAC: c_int = 2;
pub const ACRYPTO_OP_RNG: c_int = 3;
pub const ACRYPTO_MODE_ECB: c_int = 0;
pub const ACRYPTO_MODE_CBC: c_int = 1;
pub const ACRYPTO_MODE_CFB: c_int = 2;
pub const ACRYPTO_MODE_OFB: c_int = 3;
pub const ACRYPTO_TYPE_AES_128: c_int = 0;
pub const ACRYPTO_TYPE_AES_192: c_int = 1;
pub const ACRYPTO_TYPE_AES_256: c_int = 2;
pub const ACRYPTO_TYPE_3DES: c_int = 3;
pub const ACRYPTO_TYPE_DES: c_int = 4;
pub const PCI_VENDOR_ID_HIFN: c_uint = 0x13A3;
pub const PCI_DEVICE_ID_HIFN_7955: c_uint = 0x0020;
pub const PCI_DEVICE_ID_HIFN_7956: c_uint = 0x001d;
// I/O region sizes
pub const HIFN_BAR0_SIZE: c_uint = 0x1000;
pub const HIFN_BAR1_SIZE: c_uint = 0x2000;
pub const HIFN_BAR2_SIZE: c_uint = 0x8000;
// DMA registres
pub const HIFN_DMA_CRA: c_uint = 0x0C	/* DMA Command Ring Address */;
pub const HIFN_DMA_SDRA: c_uint = 0x1C	/* DMA Source Data Ring Address */;
pub const HIFN_DMA_RRA: c_uint = 0x2C	/* DMA Result Ring Address */;
pub const HIFN_DMA_DDRA: c_uint = 0x3C	/* DMA Destination Data Ring Address */;
pub const HIFN_DMA_STCTL: c_uint = 0x40	/* DMA Status and Control */;
pub const HIFN_DMA_INTREN: c_uint = 0x44	/* DMA Interrupt Enable */;
pub const HIFN_DMA_CFG1: c_uint = 0x48	/* DMA Configuration #1 */;
pub const HIFN_DMA_CFG2: c_uint = 0x6C	/* DMA Configuration #2 */;
pub const HIFN_CHIP_ID: c_uint = 0x98	/* Chip ID */;
//
// Processing Unit Registers (offset from BASEREG0)
//
pub const HIFN_0_PUDATA: c_uint = 0x00	/* Processing Unit Data */;
pub const HIFN_0_PUCTRL: c_uint = 0x04	/* Processing Unit Control */;
pub const HIFN_0_PUISR: c_uint = 0x08	/* Processing Unit Interrupt Status */;
pub const HIFN_0_PUCNFG: c_uint = 0x0c	/* Processing Unit Configuration */;
pub const HIFN_0_PUIER: c_uint = 0x10	/* Processing Unit Interrupt Enable */;
pub const HIFN_0_PUSTAT: c_uint = 0x14	/* Processing Unit Status/Chip ID */;
pub const HIFN_0_FIFOSTAT: c_uint = 0x18	/* FIFO Status */;
pub const HIFN_0_FIFOCNFG: c_uint = 0x1c	/* FIFO Configuration */;
pub const HIFN_0_SPACESIZE: c_uint = 0x20	/* Register space size */;
// Processing Unit Control Register (HIFN_0_PUCTRL)
pub const HIFN_PUCTRL_CLRSRCFIFO: c_uint = 0x0010	/* clear source fifo */;
pub const HIFN_PUCTRL_STOP: c_uint = 0x0008	/* stop pu */;
pub const HIFN_PUCTRL_LOCKRAM: c_uint = 0x0004	/* lock ram */;
pub const HIFN_PUCTRL_DMAENA: c_uint = 0x0002	/* enable dma */;
pub const HIFN_PUCTRL_RESET: c_uint = 0x0001	/* Reset processing unit */;
// Processing Unit Interrupt Status Register (HIFN_0_PUISR)
pub const HIFN_PUISR_CMDINVAL: c_uint = 0x8000	/* Invalid command interrupt */;
pub const HIFN_PUISR_DATAERR: c_uint = 0x4000	/* Data error interrupt */;
pub const HIFN_PUISR_SRCFIFO: c_uint = 0x2000	/* Source FIFO ready interrupt */;
pub const HIFN_PUISR_DSTFIFO: c_uint = 0x1000	/* Destination FIFO ready interrupt */;
pub const HIFN_PUISR_DSTOVER: c_uint = 0x0200	/* Destination overrun interrupt */;
pub const HIFN_PUISR_SRCCMD: c_uint = 0x0080	/* Source command interrupt */;
pub const HIFN_PUISR_SRCCTX: c_uint = 0x0040	/* Source context interrupt */;
pub const HIFN_PUISR_SRCDATA: c_uint = 0x0020	/* Source data interrupt */;
pub const HIFN_PUISR_DSTDATA: c_uint = 0x0010	/* Destination data interrupt */;
pub const HIFN_PUISR_DSTRESULT: c_uint = 0x0004	/* Destination result interrupt */;
// Processing Unit Configuration Register (HIFN_0_PUCNFG)
pub const HIFN_PUCNFG_DRAMMASK: c_uint = 0xe000	/* DRAM size mask */;
pub const HIFN_PUCNFG_DSZ_256K: c_uint = 0x0000	/* 256k dram */;
pub const HIFN_PUCNFG_DSZ_512K: c_uint = 0x2000	/* 512k dram */;
pub const HIFN_PUCNFG_DSZ_1M: c_uint = 0x4000	/* 1m dram */;
pub const HIFN_PUCNFG_DSZ_2M: c_uint = 0x6000	/* 2m dram */;
pub const HIFN_PUCNFG_DSZ_4M: c_uint = 0x8000	/* 4m dram */;
pub const HIFN_PUCNFG_DSZ_8M: c_uint = 0xa000	/* 8m dram */;
pub const HIFN_PUNCFG_DSZ_16M: c_uint = 0xc000	/* 16m dram */;
pub const HIFN_PUCNFG_DSZ_32M: c_uint = 0xe000	/* 32m dram */;
pub const HIFN_PUCNFG_DRAMREFRESH: c_uint = 0x1800	/* DRAM refresh rate mask */;
pub const HIFN_PUCNFG_DRFR_512: c_uint = 0x0000	/* 512 divisor of ECLK */;
pub const HIFN_PUCNFG_DRFR_256: c_uint = 0x0800	/* 256 divisor of ECLK */;
pub const HIFN_PUCNFG_DRFR_128: c_uint = 0x1000	/* 128 divisor of ECLK */;
pub const HIFN_PUCNFG_TCALLPHASES: c_uint = 0x0200	/* your guess is as good as mine... */;
pub const HIFN_PUCNFG_TCDRVTOTEM: c_uint = 0x0100	/* your guess is as good as mine... */;
pub const HIFN_PUCNFG_BIGENDIAN: c_uint = 0x0080	/* DMA big endian mode */;
pub const HIFN_PUCNFG_BUS32: c_uint = 0x0040	/* Bus width 32bits */;
pub const HIFN_PUCNFG_BUS16: c_uint = 0x0000	/* Bus width 16 bits */;
pub const HIFN_PUCNFG_CHIPID: c_uint = 0x0020	/* Allow chipid from PUSTAT */;
pub const HIFN_PUCNFG_DRAM: c_uint = 0x0010	/* Context RAM is DRAM */;
pub const HIFN_PUCNFG_SRAM: c_uint = 0x0000	/* Context RAM is SRAM */;
pub const HIFN_PUCNFG_COMPSING: c_uint = 0x0004	/* Enable single compression context */;
pub const HIFN_PUCNFG_ENCCNFG: c_uint = 0x0002	/* Encryption configuration */;
// Processing Unit Interrupt Enable Register (HIFN_0_PUIER)
pub const HIFN_PUIER_CMDINVAL: c_uint = 0x8000	/* Invalid command interrupt */;
pub const HIFN_PUIER_DATAERR: c_uint = 0x4000	/* Data error interrupt */;
pub const HIFN_PUIER_SRCFIFO: c_uint = 0x2000	/* Source FIFO ready interrupt */;
pub const HIFN_PUIER_DSTFIFO: c_uint = 0x1000	/* Destination FIFO ready interrupt */;
pub const HIFN_PUIER_DSTOVER: c_uint = 0x0200	/* Destination overrun interrupt */;
pub const HIFN_PUIER_SRCCMD: c_uint = 0x0080	/* Source command interrupt */;
pub const HIFN_PUIER_SRCCTX: c_uint = 0x0040	/* Source context interrupt */;
pub const HIFN_PUIER_SRCDATA: c_uint = 0x0020	/* Source data interrupt */;
pub const HIFN_PUIER_DSTDATA: c_uint = 0x0010	/* Destination data interrupt */;
pub const HIFN_PUIER_DSTRESULT: c_uint = 0x0004	/* Destination result interrupt */;
// Processing Unit Status Register/Chip ID (HIFN_0_PUSTAT)
pub const HIFN_PUSTAT_CMDINVAL: c_uint = 0x8000	/* Invalid command interrupt */;
pub const HIFN_PUSTAT_DATAERR: c_uint = 0x4000	/* Data error interrupt */;
pub const HIFN_PUSTAT_SRCFIFO: c_uint = 0x2000	/* Source FIFO ready interrupt */;
pub const HIFN_PUSTAT_DSTFIFO: c_uint = 0x1000	/* Destination FIFO ready interrupt */;
pub const HIFN_PUSTAT_DSTOVER: c_uint = 0x0200	/* Destination overrun interrupt */;
pub const HIFN_PUSTAT_SRCCMD: c_uint = 0x0080	/* Source command interrupt */;
pub const HIFN_PUSTAT_SRCCTX: c_uint = 0x0040	/* Source context interrupt */;
pub const HIFN_PUSTAT_SRCDATA: c_uint = 0x0020	/* Source data interrupt */;
pub const HIFN_PUSTAT_DSTDATA: c_uint = 0x0010	/* Destination data interrupt */;
pub const HIFN_PUSTAT_DSTRESULT: c_uint = 0x0004	/* Destination result interrupt */;
pub const HIFN_PUSTAT_CHIPREV: c_uint = 0x00ff	/* Chip revision mask */;
pub const HIFN_PUSTAT_CHIPENA: c_uint = 0xff00	/* Chip enabled mask */;
pub const HIFN_PUSTAT_ENA_2: c_uint = 0x1100	/* Level 2 enabled */;
pub const HIFN_PUSTAT_ENA_1: c_uint = 0x1000	/* Level 1 enabled */;
pub const HIFN_PUSTAT_ENA_0: c_uint = 0x3000	/* Level 0 enabled */;
pub const HIFN_PUSTAT_REV_2: c_uint = 0x0020	/* 7751 PT6/2 */;
pub const HIFN_PUSTAT_REV_3: c_uint = 0x0030	/* 7751 PT6/3 */;
// FIFO Status Register (HIFN_0_FIFOSTAT)
pub const HIFN_FIFOSTAT_SRC: c_uint = 0x7f00	/* Source FIFO available */;
pub const HIFN_FIFOSTAT_DST: c_uint = 0x007f	/* Destination FIFO available */;
// FIFO Configuration Register (HIFN_0_FIFOCNFG)
pub const HIFN_FIFOCNFG_THRESHOLD: c_uint = 0x0400	/* must be written as 1 */;
//
// DMA Interface Registers (offset from BASEREG1)
//
pub const HIFN_1_DMA_CRAR: c_uint = 0x0c	/* DMA Command Ring Address */;
pub const HIFN_1_DMA_SRAR: c_uint = 0x1c	/* DMA Source Ring Address */;
pub const HIFN_1_DMA_RRAR: c_uint = 0x2c	/* DMA Result Ring Address */;
pub const HIFN_1_DMA_DRAR: c_uint = 0x3c	/* DMA Destination Ring Address */;
pub const HIFN_1_DMA_CSR: c_uint = 0x40	/* DMA Status and Control */;
pub const HIFN_1_DMA_IER: c_uint = 0x44	/* DMA Interrupt Enable */;
pub const HIFN_1_DMA_CNFG: c_uint = 0x48	/* DMA Configuration */;
pub const HIFN_1_PLL: c_uint = 0x4c	/* 795x: PLL config */;
pub const HIFN_1_7811_RNGENA: c_uint = 0x60	/* 7811: rng enable */;
pub const HIFN_1_7811_RNGCFG: c_uint = 0x64	/* 7811: rng config */;
pub const HIFN_1_7811_RNGDAT: c_uint = 0x68	/* 7811: rng data */;
pub const HIFN_1_7811_RNGSTS: c_uint = 0x6c	/* 7811: rng status */;
pub const HIFN_1_7811_MIPSRST: c_uint = 0x94	/* 7811: MIPS reset */;
pub const HIFN_1_REVID: c_uint = 0x98	/* Revision ID */;
pub const HIFN_1_UNLOCK_SECRET1: c_uint = 0xf4;
pub const HIFN_1_UNLOCK_SECRET2: c_uint = 0xfc;
pub const HIFN_1_PUB_RESET: c_uint = 0x204	/* Public/RNG Reset */;
pub const HIFN_1_PUB_BASE: c_uint = 0x300	/* Public Base Address */;
pub const HIFN_1_PUB_OPLEN: c_uint = 0x304	/* Public Operand Length */;
pub const HIFN_1_PUB_OP: c_uint = 0x308	/* Public Operand */;
pub const HIFN_1_PUB_STATUS: c_uint = 0x30c	/* Public Status */;
pub const HIFN_1_PUB_IEN: c_uint = 0x310	/* Public Interrupt enable */;
pub const HIFN_1_RNG_CONFIG: c_uint = 0x314	/* RNG config */;
pub const HIFN_1_RNG_DATA: c_uint = 0x318	/* RNG data */;
pub const HIFN_1_PUB_MEM: c_uint = 0x400	/* start of Public key memory */;
pub const HIFN_1_PUB_MEMEND: c_uint = 0xbff	/* end of Public key memory */;
// DMA Status and Control Register (HIFN_1_DMA_CSR)
pub const HIFN_DMACSR_D_CTRLMASK: c_uint = 0xc0000000	/* Destinition Ring Control */;
pub const HIFN_DMACSR_D_CTRL_NOP: c_uint = 0x00000000	/* Dest. Control: no-op */;
pub const HIFN_DMACSR_D_CTRL_DIS: c_uint = 0x40000000	/* Dest. Control: disable */;
pub const HIFN_DMACSR_D_CTRL_ENA: c_uint = 0x80000000	/* Dest. Control: enable */;
pub const HIFN_DMACSR_D_ABORT: c_uint = 0x20000000	/* Destinition Ring PCIAbort */;
pub const HIFN_DMACSR_D_DONE: c_uint = 0x10000000	/* Destinition Ring Done */;
pub const HIFN_DMACSR_D_LAST: c_uint = 0x08000000	/* Destinition Ring Last */;
pub const HIFN_DMACSR_D_WAIT: c_uint = 0x04000000	/* Destinition Ring Waiting */;
pub const HIFN_DMACSR_D_OVER: c_uint = 0x02000000	/* Destinition Ring Overflow */;
pub const HIFN_DMACSR_R_CTRL: c_uint = 0x00c00000	/* Result Ring Control */;
pub const HIFN_DMACSR_R_CTRL_NOP: c_uint = 0x00000000	/* Result Control: no-op */;
pub const HIFN_DMACSR_R_CTRL_DIS: c_uint = 0x00400000	/* Result Control: disable */;
pub const HIFN_DMACSR_R_CTRL_ENA: c_uint = 0x00800000	/* Result Control: enable */;
pub const HIFN_DMACSR_R_ABORT: c_uint = 0x00200000	/* Result Ring PCI Abort */;
pub const HIFN_DMACSR_R_DONE: c_uint = 0x00100000	/* Result Ring Done */;
pub const HIFN_DMACSR_R_LAST: c_uint = 0x00080000	/* Result Ring Last */;
pub const HIFN_DMACSR_R_WAIT: c_uint = 0x00040000	/* Result Ring Waiting */;
pub const HIFN_DMACSR_R_OVER: c_uint = 0x00020000	/* Result Ring Overflow */;
pub const HIFN_DMACSR_S_CTRL: c_uint = 0x0000c000	/* Source Ring Control */;
pub const HIFN_DMACSR_S_CTRL_NOP: c_uint = 0x00000000	/* Source Control: no-op */;
pub const HIFN_DMACSR_S_CTRL_DIS: c_uint = 0x00004000	/* Source Control: disable */;
pub const HIFN_DMACSR_S_CTRL_ENA: c_uint = 0x00008000	/* Source Control: enable */;
pub const HIFN_DMACSR_S_ABORT: c_uint = 0x00002000	/* Source Ring PCI Abort */;
pub const HIFN_DMACSR_S_DONE: c_uint = 0x00001000	/* Source Ring Done */;
pub const HIFN_DMACSR_S_LAST: c_uint = 0x00000800	/* Source Ring Last */;
pub const HIFN_DMACSR_S_WAIT: c_uint = 0x00000400	/* Source Ring Waiting */;
pub const HIFN_DMACSR_ILLW: c_uint = 0x00000200	/* Illegal write (7811 only) */;
pub const HIFN_DMACSR_ILLR: c_uint = 0x00000100	/* Illegal read (7811 only) */;
pub const HIFN_DMACSR_C_CTRL: c_uint = 0x000000c0	/* Command Ring Control */;
pub const HIFN_DMACSR_C_CTRL_NOP: c_uint = 0x00000000	/* Command Control: no-op */;
pub const HIFN_DMACSR_C_CTRL_DIS: c_uint = 0x00000040	/* Command Control: disable */;
pub const HIFN_DMACSR_C_CTRL_ENA: c_uint = 0x00000080	/* Command Control: enable */;
pub const HIFN_DMACSR_C_ABORT: c_uint = 0x00000020	/* Command Ring PCI Abort */;
pub const HIFN_DMACSR_C_DONE: c_uint = 0x00000010	/* Command Ring Done */;
pub const HIFN_DMACSR_C_LAST: c_uint = 0x00000008	/* Command Ring Last */;
pub const HIFN_DMACSR_C_WAIT: c_uint = 0x00000004	/* Command Ring Waiting */;
pub const HIFN_DMACSR_PUBDONE: c_uint = 0x00000002	/* Public op done (7951 only) */;
pub const HIFN_DMACSR_ENGINE: c_uint = 0x00000001	/* Command Ring Engine IRQ */;
// DMA Interrupt Enable Register (HIFN_1_DMA_IER)
pub const HIFN_DMAIER_D_ABORT: c_uint = 0x20000000	/* Destination Ring PCIAbort */;
pub const HIFN_DMAIER_D_DONE: c_uint = 0x10000000	/* Destination Ring Done */;
pub const HIFN_DMAIER_D_LAST: c_uint = 0x08000000	/* Destination Ring Last */;
pub const HIFN_DMAIER_D_WAIT: c_uint = 0x04000000	/* Destination Ring Waiting */;
pub const HIFN_DMAIER_D_OVER: c_uint = 0x02000000	/* Destination Ring Overflow */;
pub const HIFN_DMAIER_R_ABORT: c_uint = 0x00200000	/* Result Ring PCI Abort */;
pub const HIFN_DMAIER_R_DONE: c_uint = 0x00100000	/* Result Ring Done */;
pub const HIFN_DMAIER_R_LAST: c_uint = 0x00080000	/* Result Ring Last */;
pub const HIFN_DMAIER_R_WAIT: c_uint = 0x00040000	/* Result Ring Waiting */;
pub const HIFN_DMAIER_R_OVER: c_uint = 0x00020000	/* Result Ring Overflow */;
pub const HIFN_DMAIER_S_ABORT: c_uint = 0x00002000	/* Source Ring PCI Abort */;
pub const HIFN_DMAIER_S_DONE: c_uint = 0x00001000	/* Source Ring Done */;
pub const HIFN_DMAIER_S_LAST: c_uint = 0x00000800	/* Source Ring Last */;
pub const HIFN_DMAIER_S_WAIT: c_uint = 0x00000400	/* Source Ring Waiting */;
pub const HIFN_DMAIER_ILLW: c_uint = 0x00000200	/* Illegal write (7811 only) */;
pub const HIFN_DMAIER_ILLR: c_uint = 0x00000100	/* Illegal read (7811 only) */;
pub const HIFN_DMAIER_C_ABORT: c_uint = 0x00000020	/* Command Ring PCI Abort */;
pub const HIFN_DMAIER_C_DONE: c_uint = 0x00000010	/* Command Ring Done */;
pub const HIFN_DMAIER_C_LAST: c_uint = 0x00000008	/* Command Ring Last */;
pub const HIFN_DMAIER_C_WAIT: c_uint = 0x00000004	/* Command Ring Waiting */;
pub const HIFN_DMAIER_PUBDONE: c_uint = 0x00000002	/* public op done (7951 only) */;
pub const HIFN_DMAIER_ENGINE: c_uint = 0x00000001	/* Engine IRQ */;
// DMA Configuration Register (HIFN_1_DMA_CNFG)
pub const HIFN_DMACNFG_BIGENDIAN: c_uint = 0x10000000	/* big endian mode */;
pub const HIFN_DMACNFG_POLLFREQ: c_uint = 0x00ff0000	/* Poll frequency mask */;
pub const HIFN_DMACNFG_UNLOCK: c_uint = 0x00000800;
pub const HIFN_DMACNFG_POLLINVAL: c_uint = 0x00000700	/* Invalid Poll Scalar */;
pub const HIFN_DMACNFG_LAST: c_uint = 0x00000010	/* Host control LAST bit */;
pub const HIFN_DMACNFG_MODE: c_uint = 0x00000004	/* DMA mode */;
pub const HIFN_DMACNFG_DMARESET: c_uint = 0x00000002	/* DMA Reset # */;
pub const HIFN_DMACNFG_MSTRESET: c_uint = 0x00000001	/* Master Reset # */;
// PLL configuration register
pub const HIFN_PLL_REF_CLK_HBI: c_uint = 0x00000000	/* HBI reference clock */;
pub const HIFN_PLL_REF_CLK_PLL: c_uint = 0x00000001	/* PLL reference clock */;
pub const HIFN_PLL_BP: c_uint = 0x00000002	/* Reference clock bypass */;
pub const HIFN_PLL_PK_CLK_HBI: c_uint = 0x00000000	/* PK engine HBI clock */;
pub const HIFN_PLL_PK_CLK_PLL: c_uint = 0x00000008	/* PK engine PLL clock */;
pub const HIFN_PLL_PE_CLK_HBI: c_uint = 0x00000000	/* PE engine HBI clock */;
pub const HIFN_PLL_PE_CLK_PLL: c_uint = 0x00000010	/* PE engine PLL clock */;
pub const HIFN_PLL_RESERVED_1: c_uint = 0x00000400	/* Reserved bit, must be 1 */;

pub const HIFN_PLL_ND_MULT_2: c_uint = 0x00000000	/* PLL clock multiplier 2 */;
pub const HIFN_PLL_ND_MULT_4: c_uint = 0x00000800	/* PLL clock multiplier 4 */;
pub const HIFN_PLL_ND_MULT_6: c_uint = 0x00001000	/* PLL clock multiplier 6 */;
pub const HIFN_PLL_ND_MULT_8: c_uint = 0x00001800	/* PLL clock multiplier 8 */;
pub const HIFN_PLL_ND_MULT_10: c_uint = 0x00002000	/* PLL clock multiplier 10 */;
pub const HIFN_PLL_ND_MULT_12: c_uint = 0x00002800	/* PLL clock multiplier 12 */;
pub const HIFN_PLL_IS_1_8: c_uint = 0x00000000	/* charge pump (mult. 1-8) */;
pub const HIFN_PLL_IS_9_12: c_uint = 0x00010000	/* charge pump (mult. 9-12) */;

// Public key reset register (HIFN_1_PUB_RESET)
pub const HIFN_PUBRST_RESET: c_uint = 0x00000001	/* reset public/rng unit */;
// Public base address register (HIFN_1_PUB_BASE)
pub const HIFN_PUBBASE_ADDR: c_uint = 0x00003fff	/* base address */;
// Public operand length register (HIFN_1_PUB_OPLEN)
pub const HIFN_PUBOPLEN_MOD_M: c_uint = 0x0000007f	/* modulus length mask */;

pub const HIFN_PUBOPLEN_EXP_M: c_uint = 0x0003ff80	/* exponent length mask */;

pub const HIFN_PUBOPLEN_RED_M: c_uint = 0x003c0000	/* reducend length mask */;

// Public operation register (HIFN_1_PUB_OP)
pub const HIFN_PUBOP_AOFFSET_M: c_uint = 0x0000007f	/* A offset mask */;

pub const HIFN_PUBOP_BOFFSET_M: c_uint = 0x00000f80	/* B offset mask */;

pub const HIFN_PUBOP_MOFFSET_M: c_uint = 0x0003f000	/* M offset mask */;

pub const HIFN_PUBOP_OP_MASK: c_uint = 0x003c0000	/* Opcode: */;
pub const HIFN_PUBOP_OP_NOP: c_uint = 0x00000000	/*  NOP */;
pub const HIFN_PUBOP_OP_ADD: c_uint = 0x00040000	/*  ADD */;
pub const HIFN_PUBOP_OP_ADDC: c_uint = 0x00080000	/*  ADD w/carry */;
pub const HIFN_PUBOP_OP_SUB: c_uint = 0x000c0000	/*  SUB */;
pub const HIFN_PUBOP_OP_SUBC: c_uint = 0x00100000	/*  SUB w/carry */;
pub const HIFN_PUBOP_OP_MODADD: c_uint = 0x00140000	/*  Modular ADD */;
pub const HIFN_PUBOP_OP_MODSUB: c_uint = 0x00180000	/*  Modular SUB */;
pub const HIFN_PUBOP_OP_INCA: c_uint = 0x001c0000	/*  INC A */;
pub const HIFN_PUBOP_OP_DECA: c_uint = 0x00200000	/*  DEC A */;
pub const HIFN_PUBOP_OP_MULT: c_uint = 0x00240000	/*  MULT */;
pub const HIFN_PUBOP_OP_MODMULT: c_uint = 0x00280000	/*  Modular MULT */;
pub const HIFN_PUBOP_OP_MODRED: c_uint = 0x002c0000	/*  Modular RED */;
pub const HIFN_PUBOP_OP_MODEXP: c_uint = 0x00300000	/*  Modular EXP */;
// Public status register (HIFN_1_PUB_STATUS)
pub const HIFN_PUBSTS_DONE: c_uint = 0x00000001	/* operation done */;
pub const HIFN_PUBSTS_CARRY: c_uint = 0x00000002	/* carry */;
// Public interrupt enable register (HIFN_1_PUB_IEN)
pub const HIFN_PUBIEN_DONE: c_uint = 0x00000001	/* operation done interrupt */;
// Random number generator config register (HIFN_1_RNG_CONFIG)
pub const HIFN_RNGCFG_ENA: c_uint = 0x00000001	/* enable rng */;
pub const HIFN_NAMESIZE: c_int = 32;
pub const HIFN_MAX_RESULT_ORDER: c_int = 5;

pub const HIFN_D_DST_DALIGN: c_int = 4;

pub const AES_MIN_KEY_SIZE: c_int = 16;
pub const AES_MAX_KEY_SIZE: c_int = 32;
pub const HIFN_DES_KEY_LENGTH: c_int = 8;
pub const HIFN_3DES_KEY_LENGTH: c_int = 24;

pub const HIFN_IV_LENGTH: c_int = 8;
pub const HIFN_AES_IV_LENGTH: c_int = 16;

pub const HIFN_MAC_KEY_LENGTH: c_int = 64;
pub const HIFN_MD5_LENGTH: c_int = 16;
pub const HIFN_SHA1_LENGTH: c_int = 20;
pub const HIFN_MAC_TRUNC_LENGTH: c_int = 12;

pub const HIFN_USED_RESULT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_desc {
    pub l: volatile __le32,
    pub p: volatile __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_dma {
    pub 1]: hifn_desc cmdr[HIFN_D_CMD_RSIZE +,
    pub 1]: hifn_desc srcr[HIFN_D_SRC_RSIZE +,
    pub 1]: hifn_desc dstr[HIFN_D_DST_RSIZE +,
    pub 1]: hifn_desc resr[HIFN_D_RES_RSIZE +,
    pub command_bufs: [u8; HIFN_D_CMD_RSIZE][HIFN_MAX_COMMAND],
    pub result_bufs: [u8; HIFN_D_CMD_RSIZE][HIFN_MAX_RESULT],
//
// Our current positions for insertion and removal from the descriptor
// rings.
//
    pub resi: volatile int cmdi, srci, dsti,,
    pub resu: volatile int cmdu, srcu, dstu,,
    pub resk: int cmdk, srck, dstk,,
}

pub const HIFN_DEFAULT_ACTIVE_NUM: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_device {
    pub name: [c_char; HIFN_NAMESIZE],
    pub irq: c_int,
    pub pdev: *mut pci_dev,
    pub bar: [*mut void __iomem; 3],
    pub desc_virt: *mut c_void,
    pub desc_dma: dma_addr_t,
    pub dmareg: u32,
    pub sa: [*mut c_void; HIFN_D_RES_RSIZE],
    pub lock: spinlock_t,
    pub flags: u32,
    pub started: int active,,
    pub work: delayed_work,
    pub reset: c_ulong,
    pub success: c_ulong,
    pub prev_success: c_ulong,
    pub snum: u8,
    pub tasklet: tasklet_struct,
    pub queue: crypto_queue,
    pub alg_list: list_head,
    pub pk_clk_freq: c_uint,

    pub rng_wait_time: c_uint,
    pub rngtime: ktime_t,
    pub rng: hwrng,

}

pub const HIFN_D_LENGTH: c_uint = 0x0000ffff;
pub const HIFN_D_NOINVALID: c_uint = 0x01000000;
pub const HIFN_D_MASKDONEIRQ: c_uint = 0x02000000;
pub const HIFN_D_DESTOVER: c_uint = 0x04000000;
pub const HIFN_D_OVER: c_uint = 0x08000000;
pub const HIFN_D_LAST: c_uint = 0x20000000;
pub const HIFN_D_JUMP: c_uint = 0x40000000;
pub const HIFN_D_VALID: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_base_command {
    pub masks: volatile __le16,
    pub session_num: volatile __le16,
    pub total_source_count: volatile __le16,
    pub total_dest_count: volatile __le16,
}

pub const HIFN_BASE_CMD_COMP: c_uint = 0x0100	/* enable compression engine */;
pub const HIFN_BASE_CMD_PAD: c_uint = 0x0200	/* enable padding engine */;
pub const HIFN_BASE_CMD_MAC: c_uint = 0x0400	/* enable MAC engine */;
pub const HIFN_BASE_CMD_CRYPT: c_uint = 0x0800	/* enable crypt engine */;
pub const HIFN_BASE_CMD_DECODE: c_uint = 0x2000;
pub const HIFN_BASE_CMD_SRCLEN_M: c_uint = 0xc000;
pub const HIFN_BASE_CMD_SRCLEN_S: c_int = 14;
pub const HIFN_BASE_CMD_DSTLEN_M: c_uint = 0x3000;
pub const HIFN_BASE_CMD_DSTLEN_S: c_int = 12;
pub const HIFN_BASE_CMD_LENMASK_HI: c_uint = 0x30000;
pub const HIFN_BASE_CMD_LENMASK_LO: c_uint = 0x0ffff;
//
// Structure to help build up the command data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_crypt_command {
    pub masks: volatile __le16,
    pub header_skip: volatile __le16,
    pub source_count: volatile __le16,
    pub reserved: volatile __le16,
}

pub const HIFN_CRYPT_CMD_ALG_MASK: c_uint = 0x0003		/* algorithm: */;
pub const HIFN_CRYPT_CMD_ALG_DES: c_uint = 0x0000		/*   DES */;
pub const HIFN_CRYPT_CMD_ALG_3DES: c_uint = 0x0001		/*   3DES */;
pub const HIFN_CRYPT_CMD_ALG_RC4: c_uint = 0x0002		/*   RC4 */;
pub const HIFN_CRYPT_CMD_ALG_AES: c_uint = 0x0003		/*   AES */;
pub const HIFN_CRYPT_CMD_MODE_MASK: c_uint = 0x0018		/* Encrypt mode: */;
pub const HIFN_CRYPT_CMD_MODE_ECB: c_uint = 0x0000		/*   ECB */;
pub const HIFN_CRYPT_CMD_MODE_CBC: c_uint = 0x0008		/*   CBC */;
pub const HIFN_CRYPT_CMD_MODE_CFB: c_uint = 0x0010		/*   CFB */;
pub const HIFN_CRYPT_CMD_MODE_OFB: c_uint = 0x0018		/*   OFB */;
pub const HIFN_CRYPT_CMD_CLR_CTX: c_uint = 0x0040		/* clear context */;
pub const HIFN_CRYPT_CMD_KSZ_MASK: c_uint = 0x0600		/* AES key size: */;
pub const HIFN_CRYPT_CMD_KSZ_128: c_uint = 0x0000		/*  128 bit */;
pub const HIFN_CRYPT_CMD_KSZ_192: c_uint = 0x0200		/*  192 bit */;
pub const HIFN_CRYPT_CMD_KSZ_256: c_uint = 0x0400		/*  256 bit */;
pub const HIFN_CRYPT_CMD_NEW_KEY: c_uint = 0x0800		/* expect new key */;
pub const HIFN_CRYPT_CMD_NEW_IV: c_uint = 0x1000		/* expect new iv */;
pub const HIFN_CRYPT_CMD_SRCLEN_M: c_uint = 0xc000;
pub const HIFN_CRYPT_CMD_SRCLEN_S: c_int = 14;
pub const HIFN_MAC_CMD_ALG_MASK: c_uint = 0x0001;
pub const HIFN_MAC_CMD_ALG_SHA1: c_uint = 0x0000;
pub const HIFN_MAC_CMD_ALG_MD5: c_uint = 0x0001;
pub const HIFN_MAC_CMD_MODE_MASK: c_uint = 0x000c;
pub const HIFN_MAC_CMD_MODE_HMAC: c_uint = 0x0000;
pub const HIFN_MAC_CMD_MODE_SSL_MAC: c_uint = 0x0004;
pub const HIFN_MAC_CMD_MODE_HASH: c_uint = 0x0008;
pub const HIFN_MAC_CMD_MODE_FULL: c_uint = 0x0004;
pub const HIFN_MAC_CMD_TRUNC: c_uint = 0x0010;
pub const HIFN_MAC_CMD_RESULT: c_uint = 0x0020;
pub const HIFN_MAC_CMD_APPEND: c_uint = 0x0040;
pub const HIFN_MAC_CMD_SRCLEN_M: c_uint = 0xc000;
pub const HIFN_MAC_CMD_SRCLEN_S: c_int = 14;
//
// MAC POS IPsec initiates authentication after encryption on encodes
// and before decryption on decodes.
//
pub const HIFN_MAC_CMD_POS_IPSEC: c_uint = 0x0200;
pub const HIFN_MAC_CMD_NEW_KEY: c_uint = 0x0800;
pub const HIFN_COMP_CMD_SRCLEN_M: c_uint = 0xc000;
pub const HIFN_COMP_CMD_SRCLEN_S: c_int = 14;
pub const HIFN_COMP_CMD_ONE: c_uint = 0x0100	/* must be one */;
pub const HIFN_COMP_CMD_CLEARHIST: c_uint = 0x0010	/* clear history */;
pub const HIFN_COMP_CMD_UPDATEHIST: c_uint = 0x0008	/* update history */;
pub const HIFN_COMP_CMD_LZS_STRIP0: c_uint = 0x0004	/* LZS: strip zero */;
pub const HIFN_COMP_CMD_MPPC_RESTART: c_uint = 0x0004	/* MPPC: restart */;
pub const HIFN_COMP_CMD_ALG_MASK: c_uint = 0x0001	/* compression mode: */;
pub const HIFN_COMP_CMD_ALG_MPPC: c_uint = 0x0001	/*   MPPC */;
pub const HIFN_COMP_CMD_ALG_LZS: c_uint = 0x0000	/*   LZS */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_base_result {
    pub flags: volatile __le16,
    pub session: volatile __le16,
    pub /: *mut *mut volatile __le16 src_cnt; / 15:0 of source count,
    pub /: *mut *mut volatile __le16 dst_cnt; / 15:0 of dest count,
}

pub const HIFN_BASE_RES_DSTOVERRUN: c_uint = 0x0200	/* destination overrun */;
pub const HIFN_BASE_RES_SRCLEN_M: c_uint = 0xc000	/* 17:16 of source count */;
pub const HIFN_BASE_RES_SRCLEN_S: c_int = 14;
pub const HIFN_BASE_RES_DSTLEN_M: c_uint = 0x3000	/* 17:16 of dest count */;
pub const HIFN_BASE_RES_DSTLEN_S: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_comp_result {
    pub flags: volatile __le16,
    pub crc: volatile __le16,
}

pub const HIFN_COMP_RES_LCB_M: c_uint = 0xff00	/* longitudinal check byte */;
pub const HIFN_COMP_RES_LCB_S: c_int = 8;
pub const HIFN_COMP_RES_RESTART: c_uint = 0x0004	/* MPPC: restart */;
pub const HIFN_COMP_RES_ENDMARKER: c_uint = 0x0002	/* LZS: end marker seen */;
pub const HIFN_COMP_RES_SRC_NOTZERO: c_uint = 0x0001	/* source expired */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_mac_result {
    pub flags: volatile __le16,
    pub reserved: volatile __le16,
// followed by 0, 6, 8, or 10 u16's of the MAC, then crypt
}

pub const HIFN_MAC_RES_MISCOMPARE: c_uint = 0x0002	/* compare failed */;
pub const HIFN_MAC_RES_SRC_NOTZERO: c_uint = 0x0001	/* source expired */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_crypt_result {
    pub flags: volatile __le16,
    pub reserved: volatile __le16,
}

pub const HIFN_CRYPT_RES_SRC_NOTZERO: c_uint = 0x0001	/* source expired */;

pub const HIFN_POLL_FREQUENCY: c_uint = 0x1;

pub const HIFN_POLL_SCALAR: c_uint = 0x0;

pub const HIFN_MAX_SEGLEN: c_uint = 0xffff		/* maximum dma segment len */;
pub const HIFN_MAX_DMALEN: c_uint = 0x3ffff		/* maximum dma length */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_crypto_alg {
    pub entry: list_head,
    pub alg: skcipher_alg,
    pub dev: *mut hifn_device,
}

pub const ASYNC_SCATTERLIST_CACHE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_cipher_walk {
    pub cache: [scatterlist; ASYNC_SCATTERLIST_CACHE],
    pub flags: u32,
    pub num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_context {
    pub key: [u8; HIFN_MAX_CRYPT_KEY_LENGTH],
    pub dev: *mut hifn_device,
    pub keysize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_request_context {
    pub iv: *mut u8,
    pub ivsize: c_uint,
    pub unused: u8 op, type, mode,,
    pub walk: hifn_cipher_walk,
}

#[no_mangle]
pub unsafe extern "C" fn hifn_read_0(dev: *mut hifn_device, reg: u32) -> u32 {
    static inline u32 hifn_read_0(struct hifn_device *dev, u32 reg)
    {
    return readl(dev.bar[0] + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_read_1(dev: *mut hifn_device, reg: u32) -> u32 {
    static inline u32 hifn_read_1(struct hifn_device *dev, u32 reg)
    {
    return readl(dev.bar[1] + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_write_0(dev: *mut hifn_device, reg: u32, val: u32) {
    static inline void hifn_write_0(struct hifn_device *dev, u32 reg, u32 val)
    {
    writel(( u32)cpu_to_le32(val), dev.bar[0] + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_write_1(dev: *mut hifn_device, reg: u32, val: u32) {
    static inline void hifn_write_1(struct hifn_device *dev, u32 reg, u32 val)
    {
    writel(( u32)cpu_to_le32(val), dev.bar[1] + reg);
    }
#[no_mangle]
unsafe extern "C" fn hifn_wait_puc(dev: *mut hifn_device) {
    static void hifn_wait_puc(struct hifn_device *dev)
    {
    int i;
    u32 ret;
    for (i = 10000; i > 0; --i) {
    ret = hifn_read_0(dev, HIFN_0_PUCTRL);
    if (!(ret & HIFN_PUCTRL_RESET))
    break;
    udelay(1);
    }
    if (!i)
    dev_err(&dev.pdev.dev, "Failed to reset PUC unit.\n");
    }
#[no_mangle]
unsafe extern "C" fn hifn_reset_puc(dev: *mut hifn_device) {
    static void hifn_reset_puc(struct hifn_device *dev)
    {
    hifn_write_0(dev, HIFN_0_PUCTRL, HIFN_PUCTRL_DMAENA);
    hifn_wait_puc(dev);
    }
#[no_mangle]
unsafe extern "C" fn hifn_stop_device(dev: *mut hifn_device) {
    static void hifn_stop_device(struct hifn_device *dev)
    {
    hifn_write_1(dev, HIFN_1_DMA_CSR,
    HIFN_DMACSR_D_CTRL_DIS | HIFN_DMACSR_R_CTRL_DIS |
    HIFN_DMACSR_S_CTRL_DIS | HIFN_DMACSR_C_CTRL_DIS);
    hifn_write_0(dev, HIFN_0_PUIER, 0);
    hifn_write_1(dev, HIFN_1_DMA_IER, 0);
    }
#[no_mangle]
unsafe extern "C" fn hifn_reset_dma(dev: *mut hifn_device, full: c_int) {
    static void hifn_reset_dma(struct hifn_device *dev, int full)
    {
    hifn_stop_device(dev);
//
// Setting poll frequency and others to 0.
//
    hifn_write_1(dev, HIFN_1_DMA_CNFG, HIFN_DMACNFG_MSTRESET |
    HIFN_DMACNFG_DMARESET | HIFN_DMACNFG_MODE);
    mdelay(1);
//
// Reset DMA.
//
    if (full) {
    hifn_write_1(dev, HIFN_1_DMA_CNFG, HIFN_DMACNFG_MODE);
    mdelay(1);
    } else {
    hifn_write_1(dev, HIFN_1_DMA_CNFG, HIFN_DMACNFG_MODE |
    HIFN_DMACNFG_MSTRESET);
    hifn_reset_puc(dev);
    }
    hifn_write_1(dev, HIFN_1_DMA_CNFG, HIFN_DMACNFG_MSTRESET |
    HIFN_DMACNFG_DMARESET | HIFN_DMACNFG_MODE);
    hifn_reset_puc(dev);
    }
#[no_mangle]
unsafe extern "C" fn hifn_next_signature(a: u32, cnt: u_int) -> u32 {
    static u32 hifn_next_signature(u32 a, u_int cnt)
    {
    int i;
    u32 v;
    for (i = 0; i < cnt; i++) {
// get the parity
    v = a & 0x80080125;
    v ^= v >> 16;
    v ^= v >> 8;
    v ^= v >> 4;
    v ^= v >> 2;
    v ^= v >> 1;
    a = (v & 1) ^ (a << 1);
    }
    return a;
    }
    static struct pci2id {
    u_short		pci_vendor;
    u_short		pci_prod;
    char		card_id[13];
    } pci2id[] = {
    {
    PCI_VENDOR_ID_HIFN,
    PCI_DEVICE_ID_HIFN_7955,
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00 }
    },
    {
    PCI_VENDOR_ID_HIFN,
    PCI_DEVICE_ID_HIFN_7956,
    { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00 }
    }
    };

#[no_mangle]
unsafe extern "C" fn hifn_rng_data_present(rng: *mut hwrng, wait: c_int) -> c_int {
    static int hifn_rng_data_present(struct hwrng *rng, int wait)
    {
    struct hifn_device *dev = (struct hifn_device *)rng.priv;
    s64 nsec;
    nsec = ktime_to_ns(ktime_sub(ktime_get(), dev.rngtime));
    nsec -= dev.rng_wait_time;
    if (nsec <= 0)
    return 1;
    if (!wait)
    return 0;
    ndelay(nsec);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hifn_rng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int hifn_rng_data_read(struct hwrng *rng, u32 *data)
    {
    struct hifn_device *dev = (struct hifn_device *)rng.priv;
// data = hifn_read_1(dev, HIFN_1_RNG_DATA);
    dev.rngtime = ktime_get();
    return 4;
    }
#[no_mangle]
unsafe extern "C" fn hifn_register_rng(dev: *mut hifn_device) -> c_int {
    static int hifn_register_rng(struct hifn_device *dev)
    {
//
// We must wait at least 256 Pk_clk cycles between two reads of the rng.
//
    dev.rng_wait_time	= DIV_ROUND_UP_ULL(NSEC_PER_SEC,
    dev.pk_clk_freq) * 256;
    dev.rng.name		= dev.name;
    dev.rng.data_present	= hifn_rng_data_present;
    dev.rng.data_read	= hifn_rng_data_read;
    dev.rng.priv		= (unsigned long)dev;
    return hwrng_register(&dev.rng);
    }
#[no_mangle]
unsafe extern "C" fn hifn_unregister_rng(dev: *mut hifn_device) {
    static void hifn_unregister_rng(struct hifn_device *dev)
    {
    hwrng_unregister(&dev.rng);
    }

pub const hifn_register_rng(dev): c_int = 0;
// Macro flag: #define hifn_unregister_rng(dev)

#[no_mangle]
unsafe extern "C" fn hifn_init_pubrng(dev: *mut hifn_device) -> c_int {
    static int hifn_init_pubrng(struct hifn_device *dev)
    {
    int i;
    hifn_write_1(dev, HIFN_1_PUB_RESET, hifn_read_1(dev, HIFN_1_PUB_RESET) |
    HIFN_PUBRST_RESET);
    for (i = 100; i > 0; --i) {
    mdelay(1);
    if ((hifn_read_1(dev, HIFN_1_PUB_RESET) & HIFN_PUBRST_RESET) == 0)
    break;
    }
    if (!i) {
    dev_err(&dev.pdev.dev, "Failed to initialise public key engine.\n");
    } else {
    hifn_write_1(dev, HIFN_1_PUB_IEN, HIFN_PUBIEN_DONE);
    dev.dmareg |= HIFN_DMAIER_PUBDONE;
    hifn_write_1(dev, HIFN_1_DMA_IER, dev.dmareg);
    dev_dbg(&dev.pdev.dev, "Public key engine has been successfully initialised.\n");
    }
// Enable RNG engine.
    hifn_write_1(dev, HIFN_1_RNG_CONFIG,
    hifn_read_1(dev, HIFN_1_RNG_CONFIG) | HIFN_RNGCFG_ENA);
    dev_dbg(&dev.pdev.dev, "RNG engine has been successfully initialised.\n");

// First value must be discarded
    hifn_read_1(dev, HIFN_1_RNG_DATA);
    dev.rngtime = ktime_get();

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hifn_enable_crypto(dev: *mut hifn_device) -> c_int {
    static int hifn_enable_crypto(struct hifn_device *dev)
    {
    u32 dmacfg, addr;
    char *offtbl = core::ptr::null_mut();
    int i;
    for (i = 0; i < ARRAY_SIZE(pci2id); i++) {
    if (pci2id[i].pci_vendor == dev.pdev.vendor &&
    pci2id[i].pci_prod == dev.pdev.device) {
    offtbl = pci2id[i].card_id;
    break;
    }
    }
    if (!offtbl) {
    dev_err(&dev.pdev.dev, "Unknown card!\n");
    return -ENODEV;
    }
    dmacfg = hifn_read_1(dev, HIFN_1_DMA_CNFG);
    hifn_write_1(dev, HIFN_1_DMA_CNFG,
    HIFN_DMACNFG_UNLOCK | HIFN_DMACNFG_MSTRESET |
    HIFN_DMACNFG_DMARESET | HIFN_DMACNFG_MODE);
    mdelay(1);
    addr = hifn_read_1(dev, HIFN_1_UNLOCK_SECRET1);
    mdelay(1);
    hifn_write_1(dev, HIFN_1_UNLOCK_SECRET2, 0);
    mdelay(1);
    for (i = 0; i < 12; ++i) {
    addr = hifn_next_signature(addr, offtbl[i] + 0x101);
    hifn_write_1(dev, HIFN_1_UNLOCK_SECRET2, addr);
    mdelay(1);
    }
    hifn_write_1(dev, HIFN_1_DMA_CNFG, dmacfg);
    dev_dbg(&dev.pdev.dev, "%s %s.\n", dev.name, pci_name(dev.pdev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hifn_init_dma(dev: *mut hifn_device) {
    static void hifn_init_dma(struct hifn_device *dev)
    {
    struct hifn_dma *dma = dev.desc_virt;
    let mut dptr: u32 = dev.desc_dma;
    int i;
    for (i = 0; i < HIFN_D_CMD_RSIZE; ++i)
    dma.cmdr[i].p = __cpu_to_le32(dptr +
    offsetof(struct hifn_dma, command_bufs[i][0]));
    for (i = 0; i < HIFN_D_RES_RSIZE; ++i)
    dma.resr[i].p = __cpu_to_le32(dptr +
    offsetof(struct hifn_dma, result_bufs[i][0]));
// Setup LAST descriptors.
    dma.cmdr[HIFN_D_CMD_RSIZE].p = __cpu_to_le32(dptr +
    offsetof(struct hifn_dma, cmdr[0]));
    dma.srcr[HIFN_D_SRC_RSIZE].p = __cpu_to_le32(dptr +
    offsetof(struct hifn_dma, srcr[0]));
    dma.dstr[HIFN_D_DST_RSIZE].p = __cpu_to_le32(dptr +
    offsetof(struct hifn_dma, dstr[0]));
    dma.resr[HIFN_D_RES_RSIZE].p = __cpu_to_le32(dptr +
    offsetof(struct hifn_dma, resr[0]));
    dma.cmdu = dma.srcu = dma.dstu = dma.resu = 0;
    dma.cmdi = dma.srci = dma.dsti = dma.resi = 0;
    dma.cmdk = dma.srck = dma.dstk = dma.resk = 0;
    }
//
// Initialize the PLL. We need to know the frequency of the reference clock
// to calculate the optimal multiplier. For PCI we assume 66MHz, since that
// allows us to operate without the risk of overclocking the chip. If it
// actually uses 33MHz, the chip will operate at half the speed, this can be
// overridden by specifying the frequency as module parameter (pci33).
//
// Unfortunately the PCI clock is not very suitable since the HIFN needs a
// stable clock and the PCI clock frequency may vary, so the default is the
// external clock. There is no way to find out its frequency, we default to
// 66MHz since according to Mike Ham of HiFn, almost every board in existence
// has an external crystal populated at 66MHz.
//
#[no_mangle]
unsafe extern "C" fn hifn_init_pll(dev: *mut hifn_device) {
    static void hifn_init_pll(struct hifn_device *dev)
    {
    unsigned int freq, m;
    u32 pllcfg;
    pllcfg = HIFN_1_PLL | HIFN_PLL_RESERVED_1;
    if (strncmp(hifn_pll_ref, "ext", 3) == 0)
    pllcfg |= HIFN_PLL_REF_CLK_PLL;
    else
    pllcfg |= HIFN_PLL_REF_CLK_HBI;
    if (hifn_pll_ref[3] == '\0' ||
    kstrtouint(hifn_pll_ref + 3, 10, &freq)) {
    freq = 66;
    dev_info(&dev.pdev.dev, "assuming %u MHz clock speed, override with hifn_pll_ref=%.3s<frequency>\n",
    freq, hifn_pll_ref);
    }
    m = HIFN_PLL_FCK_MAX / freq;
    pllcfg |= (m / 2 - 1) << HIFN_PLL_ND_SHIFT;
    if (m <= 8)
    pllcfg |= HIFN_PLL_IS_1_8;
    else
    pllcfg |= HIFN_PLL_IS_9_12;
// Select clock source and enable clock bypass
    hifn_write_1(dev, HIFN_1_PLL, pllcfg |
    HIFN_PLL_PK_CLK_HBI | HIFN_PLL_PE_CLK_HBI | HIFN_PLL_BP);
// Let the chip lock to the input clock
    mdelay(10);
// Disable clock bypass
    hifn_write_1(dev, HIFN_1_PLL, pllcfg |
    HIFN_PLL_PK_CLK_HBI | HIFN_PLL_PE_CLK_HBI);
// Switch the engines to the PLL
    hifn_write_1(dev, HIFN_1_PLL, pllcfg |
    HIFN_PLL_PK_CLK_PLL | HIFN_PLL_PE_CLK_PLL);
//
// The Fpk_clk runs at half the total speed. Its frequency is needed to
// calculate the minimum time between two reads of the rng. Since 33MHz
// is actually 33.333... we overestimate the frequency here, resulting
// in slightly larger intervals.
//
    dev.pk_clk_freq = 1000000 * (freq + 1) * m / 2;
    }
#[no_mangle]
unsafe extern "C" fn hifn_init_registers(dev: *mut hifn_device) {
    static void hifn_init_registers(struct hifn_device *dev)
    {
    let mut dptr: u32 = dev.desc_dma;
// Initialization magic...
    hifn_write_0(dev, HIFN_0_PUCTRL, HIFN_PUCTRL_DMAENA);
    hifn_write_0(dev, HIFN_0_FIFOCNFG, HIFN_FIFOCNFG_THRESHOLD);
    hifn_write_0(dev, HIFN_0_PUIER, HIFN_PUIER_DSTOVER);
// write all 4 ring address registers
    hifn_write_1(dev, HIFN_1_DMA_CRAR, dptr +
    offsetof(struct hifn_dma, cmdr[0]));
    hifn_write_1(dev, HIFN_1_DMA_SRAR, dptr +
    offsetof(struct hifn_dma, srcr[0]));
    hifn_write_1(dev, HIFN_1_DMA_DRAR, dptr +
    offsetof(struct hifn_dma, dstr[0]));
    hifn_write_1(dev, HIFN_1_DMA_RRAR, dptr +
    offsetof(struct hifn_dma, resr[0]));
    mdelay(2);

    hifn_write_1(dev, HIFN_1_DMA_CSR,
    HIFN_DMACSR_D_CTRL_DIS | HIFN_DMACSR_R_CTRL_DIS |
    HIFN_DMACSR_S_CTRL_DIS | HIFN_DMACSR_C_CTRL_DIS |
    HIFN_DMACSR_D_ABORT | HIFN_DMACSR_D_DONE | HIFN_DMACSR_D_LAST |
    HIFN_DMACSR_D_WAIT | HIFN_DMACSR_D_OVER |
    HIFN_DMACSR_R_ABORT | HIFN_DMACSR_R_DONE | HIFN_DMACSR_R_LAST |
    HIFN_DMACSR_R_WAIT | HIFN_DMACSR_R_OVER |
    HIFN_DMACSR_S_ABORT | HIFN_DMACSR_S_DONE | HIFN_DMACSR_S_LAST |
    HIFN_DMACSR_S_WAIT |
    HIFN_DMACSR_C_ABORT | HIFN_DMACSR_C_DONE | HIFN_DMACSR_C_LAST |
    HIFN_DMACSR_C_WAIT |
    HIFN_DMACSR_ENGINE |
    HIFN_DMACSR_PUBDONE);

    hifn_write_1(dev, HIFN_1_DMA_CSR,
    HIFN_DMACSR_C_CTRL_ENA | HIFN_DMACSR_S_CTRL_ENA |
    HIFN_DMACSR_D_CTRL_ENA | HIFN_DMACSR_R_CTRL_ENA |
    HIFN_DMACSR_D_ABORT | HIFN_DMACSR_D_DONE | HIFN_DMACSR_D_LAST |
    HIFN_DMACSR_D_WAIT | HIFN_DMACSR_D_OVER |
    HIFN_DMACSR_R_ABORT | HIFN_DMACSR_R_DONE | HIFN_DMACSR_R_LAST |
    HIFN_DMACSR_R_WAIT | HIFN_DMACSR_R_OVER |
    HIFN_DMACSR_S_ABORT | HIFN_DMACSR_S_DONE | HIFN_DMACSR_S_LAST |
    HIFN_DMACSR_S_WAIT |
    HIFN_DMACSR_C_ABORT | HIFN_DMACSR_C_DONE | HIFN_DMACSR_C_LAST |
    HIFN_DMACSR_C_WAIT |
    HIFN_DMACSR_ENGINE |
    HIFN_DMACSR_PUBDONE);

    hifn_read_1(dev, HIFN_1_DMA_CSR);
    dev.dmareg |= HIFN_DMAIER_R_DONE | HIFN_DMAIER_C_ABORT |
    HIFN_DMAIER_D_OVER | HIFN_DMAIER_R_OVER |
    HIFN_DMAIER_S_ABORT | HIFN_DMAIER_D_ABORT | HIFN_DMAIER_R_ABORT |
    HIFN_DMAIER_ENGINE;
    dev.dmareg &= ~HIFN_DMAIER_C_WAIT;
    hifn_write_1(dev, HIFN_1_DMA_IER, dev.dmareg);
    hifn_read_1(dev, HIFN_1_DMA_IER);

    hifn_write_0(dev, HIFN_0_PUCNFG, HIFN_PUCNFG_ENCCNFG |
    HIFN_PUCNFG_DRFR_128 | HIFN_PUCNFG_TCALLPHASES |
    HIFN_PUCNFG_TCDRVTOTEM | HIFN_PUCNFG_BUS32 |
    HIFN_PUCNFG_DRAM);

    hifn_write_0(dev, HIFN_0_PUCNFG, 0x10342);

    hifn_init_pll(dev);
    hifn_write_0(dev, HIFN_0_PUISR, HIFN_PUISR_DSTOVER);
    hifn_write_1(dev, HIFN_1_DMA_CNFG, HIFN_DMACNFG_MSTRESET |
    HIFN_DMACNFG_DMARESET | HIFN_DMACNFG_MODE | HIFN_DMACNFG_LAST |
    ((HIFN_POLL_FREQUENCY << 16 ) & HIFN_DMACNFG_POLLFREQ) |
    ((HIFN_POLL_SCALAR << 8) & HIFN_DMACNFG_POLLINVAL));
    }
    static int hifn_setup_base_command(struct hifn_device *dev, u8 *buf,
    unsigned dlen, unsigned slen, u16 mask, u8 snum)
    {
    struct hifn_base_command *base_cmd;
    u8 *buf_pos = buf;
    base_cmd = (struct hifn_base_command *)buf_pos;
    base_cmd.masks = __cpu_to_le16(mask);
    base_cmd.total_source_count =
    __cpu_to_le16(slen & HIFN_BASE_CMD_LENMASK_LO);
    base_cmd.total_dest_count =
    __cpu_to_le16(dlen & HIFN_BASE_CMD_LENMASK_LO);
    dlen >>= 16;
    slen >>= 16;
    base_cmd.session_num = __cpu_to_le16(snum |
    ((slen << HIFN_BASE_CMD_SRCLEN_S) & HIFN_BASE_CMD_SRCLEN_M) |
    ((dlen << HIFN_BASE_CMD_DSTLEN_S) & HIFN_BASE_CMD_DSTLEN_M));
    return sizeof(struct hifn_base_command);
    }
    static int hifn_setup_crypto_command(struct hifn_device *dev,
    u8 *buf, unsigned dlen, unsigned slen,
    u8 *key, int keylen, u8 *iv, int ivsize, u16 mode)
    {
    struct hifn_dma *dma = dev.desc_virt;
    struct hifn_crypt_command *cry_cmd;
    u8 *buf_pos = buf;
    u16 cmd_len;
    cry_cmd = (struct hifn_crypt_command *)buf_pos;
    cry_cmd.source_count = __cpu_to_le16(dlen & 0xffff);
    dlen >>= 16;
    cry_cmd.masks = __cpu_to_le16(mode |
    ((dlen << HIFN_CRYPT_CMD_SRCLEN_S) &
    HIFN_CRYPT_CMD_SRCLEN_M));
    cry_cmd.header_skip = 0;
    cry_cmd.reserved = 0;
    buf_pos += sizeof(struct hifn_crypt_command);
    dma.cmdu++;
    if (dma.cmdu > 1) {
    dev.dmareg |= HIFN_DMAIER_C_WAIT;
    hifn_write_1(dev, HIFN_1_DMA_IER, dev.dmareg);
    }
    if (keylen) {
    memcpy(buf_pos, key, keylen);
    buf_pos += keylen;
    }
    if (ivsize) {
    memcpy(buf_pos, iv, ivsize);
    buf_pos += ivsize;
    }
    cmd_len = buf_pos - buf;
    return cmd_len;
    }
    static int hifn_setup_cmd_desc(struct hifn_device *dev,
    struct hifn_context *ctx, struct hifn_request_context *rctx,
    void *priv, unsigned int nbytes)
    {
    struct hifn_dma *dma = dev.desc_virt;
    int cmd_len, sa_idx;
    u8 *buf, *buf_pos;
    u16 mask;
    sa_idx = dma.cmdi;
    buf_pos = buf = dma.command_bufs[dma.cmdi];
    mask = 0;
    switch (rctx.op) {
    case ACRYPTO_OP_DECRYPT:
    mask = HIFN_BASE_CMD_CRYPT | HIFN_BASE_CMD_DECODE;
    break;
    case ACRYPTO_OP_ENCRYPT:
    mask = HIFN_BASE_CMD_CRYPT;
    break;
    case ACRYPTO_OP_HMAC:
    mask = HIFN_BASE_CMD_MAC;
    break;
    default:
    goto err_out;
    }
    buf_pos += hifn_setup_base_command(dev, buf_pos, nbytes,
    nbytes, mask, dev.snum);
    if (rctx.op == ACRYPTO_OP_ENCRYPT || rctx.op == ACRYPTO_OP_DECRYPT) {
    let mut md: u16 = 0;
    if (ctx.keysize)
    md |= HIFN_CRYPT_CMD_NEW_KEY;
    if (rctx.iv && rctx.mode != ACRYPTO_MODE_ECB)
    md |= HIFN_CRYPT_CMD_NEW_IV;
    switch (rctx.mode) {
    case ACRYPTO_MODE_ECB:
    md |= HIFN_CRYPT_CMD_MODE_ECB;
    break;
    case ACRYPTO_MODE_CBC:
    md |= HIFN_CRYPT_CMD_MODE_CBC;
    break;
    case ACRYPTO_MODE_CFB:
    md |= HIFN_CRYPT_CMD_MODE_CFB;
    break;
    case ACRYPTO_MODE_OFB:
    md |= HIFN_CRYPT_CMD_MODE_OFB;
    break;
    default:
    goto err_out;
    }
    switch (rctx.type) {
    case ACRYPTO_TYPE_AES_128:
    if (ctx.keysize != 16)
    goto err_out;
    md |= HIFN_CRYPT_CMD_KSZ_128 |
    HIFN_CRYPT_CMD_ALG_AES;
    break;
    case ACRYPTO_TYPE_AES_192:
    if (ctx.keysize != 24)
    goto err_out;
    md |= HIFN_CRYPT_CMD_KSZ_192 |
    HIFN_CRYPT_CMD_ALG_AES;
    break;
    case ACRYPTO_TYPE_AES_256:
    if (ctx.keysize != 32)
    goto err_out;
    md |= HIFN_CRYPT_CMD_KSZ_256 |
    HIFN_CRYPT_CMD_ALG_AES;
    break;
    case ACRYPTO_TYPE_3DES:
    if (ctx.keysize != 24)
    goto err_out;
    md |= HIFN_CRYPT_CMD_ALG_3DES;
    break;
    case ACRYPTO_TYPE_DES:
    if (ctx.keysize != 8)
    goto err_out;
    md |= HIFN_CRYPT_CMD_ALG_DES;
    break;
    default:
    goto err_out;
    }
    buf_pos += hifn_setup_crypto_command(dev, buf_pos,
    nbytes, nbytes, ctx.key, ctx.keysize,
    rctx.iv, rctx.ivsize, md);
    }
    dev.sa[sa_idx] = priv;
    dev.started++;
    cmd_len = buf_pos - buf;
    dma.cmdr[dma.cmdi].l = __cpu_to_le32(cmd_len | HIFN_D_VALID |
    HIFN_D_LAST | HIFN_D_MASKDONEIRQ);
    if (++dma.cmdi == HIFN_D_CMD_RSIZE) {
    dma.cmdr[dma.cmdi].l = __cpu_to_le32(
    HIFN_D_VALID | HIFN_D_LAST |
    HIFN_D_MASKDONEIRQ | HIFN_D_JUMP);
    dma.cmdi = 0;
    } else {
    dma.cmdr[dma.cmdi - 1].l |= __cpu_to_le32(HIFN_D_VALID);
    }
    if (!(dev.flags & HIFN_FLAG_CMD_BUSY)) {
    hifn_write_1(dev, HIFN_1_DMA_CSR, HIFN_DMACSR_C_CTRL_ENA);
    dev.flags |= HIFN_FLAG_CMD_BUSY;
    }
    return 0;
    err_out:
    return -EINVAL;
    }
    static int hifn_setup_src_desc(struct hifn_device *dev, struct page *page,
    unsigned int offset, unsigned int size, int last)
    {
    struct hifn_dma *dma = dev.desc_virt;
    int idx;
    dma_addr_t addr;
    addr = dma_map_page(&dev.pdev.dev, page, offset, size,
    DMA_TO_DEVICE);
    idx = dma.srci;
    dma.srcr[idx].p = __cpu_to_le32(addr);
    dma.srcr[idx].l = __cpu_to_le32(size | HIFN_D_VALID |
    HIFN_D_MASKDONEIRQ | (last ? HIFN_D_LAST : 0));
    if (++idx == HIFN_D_SRC_RSIZE) {
    dma.srcr[idx].l = __cpu_to_le32(HIFN_D_VALID |
    HIFN_D_JUMP | HIFN_D_MASKDONEIRQ |
    (last ? HIFN_D_LAST : 0));
    idx = 0;
    }
    dma.srci = idx;
    dma.srcu++;
    if (!(dev.flags & HIFN_FLAG_SRC_BUSY)) {
    hifn_write_1(dev, HIFN_1_DMA_CSR, HIFN_DMACSR_S_CTRL_ENA);
    dev.flags |= HIFN_FLAG_SRC_BUSY;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn hifn_setup_res_desc(dev: *mut hifn_device) {
    static void hifn_setup_res_desc(struct hifn_device *dev)
    {
    struct hifn_dma *dma = dev.desc_virt;
    dma.resr[dma.resi].l = __cpu_to_le32(HIFN_USED_RESULT |
    HIFN_D_VALID | HIFN_D_LAST);
//
// dma->resr[dma->resi].l = __cpu_to_le32(HIFN_MAX_RESULT | HIFN_D_VALID |
// HIFN_D_LAST);
//
    if (++dma.resi == HIFN_D_RES_RSIZE) {
    dma.resr[HIFN_D_RES_RSIZE].l = __cpu_to_le32(HIFN_D_VALID |
    HIFN_D_JUMP | HIFN_D_MASKDONEIRQ | HIFN_D_LAST);
    dma.resi = 0;
    }
    dma.resu++;
    if (!(dev.flags & HIFN_FLAG_RES_BUSY)) {
    hifn_write_1(dev, HIFN_1_DMA_CSR, HIFN_DMACSR_R_CTRL_ENA);
    dev.flags |= HIFN_FLAG_RES_BUSY;
    }
    }
    static void hifn_setup_dst_desc(struct hifn_device *dev, struct page *page,
    unsigned offset, unsigned size, int last)
    {
    struct hifn_dma *dma = dev.desc_virt;
    int idx;
    dma_addr_t addr;
    addr = dma_map_page(&dev.pdev.dev, page, offset, size,
    DMA_FROM_DEVICE);
    idx = dma.dsti;
    dma.dstr[idx].p = __cpu_to_le32(addr);
    dma.dstr[idx].l = __cpu_to_le32(size |	HIFN_D_VALID |
    HIFN_D_MASKDONEIRQ | (last ? HIFN_D_LAST : 0));
    if (++idx == HIFN_D_DST_RSIZE) {
    dma.dstr[idx].l = __cpu_to_le32(HIFN_D_VALID |
    HIFN_D_JUMP | HIFN_D_MASKDONEIRQ |
    (last ? HIFN_D_LAST : 0));
    idx = 0;
    }
    dma.dsti = idx;
    dma.dstu++;
    if (!(dev.flags & HIFN_FLAG_DST_BUSY)) {
    hifn_write_1(dev, HIFN_1_DMA_CSR, HIFN_DMACSR_D_CTRL_ENA);
    dev.flags |= HIFN_FLAG_DST_BUSY;
    }
    }
    static int hifn_setup_dma(struct hifn_device *dev,
    struct hifn_context *ctx, struct hifn_request_context *rctx,
    struct scatterlist *src, struct scatterlist *dst,
    unsigned int nbytes, void *priv)
    {
    struct scatterlist *t;
    struct page *spage, *dpage;
    unsigned int soff, doff;
    unsigned int n, len;
    n = nbytes;
    while (n) {
    spage = sg_page(src);
    soff = src.offset;
    len = min(src.length, n);
    hifn_setup_src_desc(dev, spage, soff, len, n - len == 0);
    src++;
    n -= len;
    }
    t = &rctx.walk.cache[0];
    n = nbytes;
    while (n) {
    if (t.length && rctx.walk.flags & ASYNC_FLAGS_MISALIGNED) {
    BUG_ON(!sg_page(t));
    dpage = sg_page(t);
    doff = 0;
    len = t.length;
    } else {
    BUG_ON(!sg_page(dst));
    dpage = sg_page(dst);
    doff = dst.offset;
    len = dst.length;
    }
    len = min(len, n);
    hifn_setup_dst_desc(dev, dpage, doff, len, n - len == 0);
    dst++;
    t++;
    n -= len;
    }
    hifn_setup_cmd_desc(dev, ctx, rctx, priv, nbytes);
    hifn_setup_res_desc(dev);
    return 0;
    }
    static int hifn_cipher_walk_init(struct hifn_cipher_walk *w,
    int num, gfp_t gfp_flags)
    {
    int i;
    num = min(ASYNC_SCATTERLIST_CACHE, num);
    sg_init_table(w.cache, num);
    w.num = 0;
    for (i = 0; i < num; ++i) {
    struct page *page = alloc_page(gfp_flags);
    struct scatterlist *s;
    if (!page)
    break;
    s = &w.cache[i];
    sg_set_page(s, page, PAGE_SIZE, 0);
    w.num++;
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn hifn_cipher_walk_exit(w: *mut hifn_cipher_walk) {
    static void hifn_cipher_walk_exit(struct hifn_cipher_walk *w)
    {
    int i;
    for (i = 0; i < w.num; ++i) {
    struct scatterlist *s = &w.cache[i];
    __free_page(sg_page(s));
    s.length = 0;
    }
    w.num = 0;
    }
    static int skcipher_add(unsigned int *drestp, struct scatterlist *dst,
    unsigned int size, unsigned int *nbytesp)
    {
    unsigned int copy, drest = *drestp, nbytes = *nbytesp;
    let mut idx: c_int = 0;
    if (drest < size || size > nbytes)
    return -EINVAL;
    while (size) {
    copy = min3(drest, size, dst.length);
    size -= copy;
    drest -= copy;
    nbytes -= copy;
    pr_debug("%s: copy: %u, size: %u, drest: %u, nbytes: %u.\n",
    __func__, copy, size, drest, nbytes);
    dst++;
    idx++;
    }
// nbytesp = nbytes;
// drestp = drest;
    return idx;
    }
    static int hifn_cipher_walk(struct skcipher_request *req,
    struct hifn_cipher_walk *w)
    {
    struct scatterlist *dst, *t;
    let mut nbytes: c_uint = req.cryptlen, offset, copy, diff;
    int idx, tidx, err;
    tidx = idx = 0;
    offset = 0;
    while (nbytes) {
    if (idx >= w.num && (w.flags & ASYNC_FLAGS_MISALIGNED))
    return -EINVAL;
    dst = &req.dst[idx];
    pr_debug("\n%s: dlen: %u, doff: %u, offset: %u, nbytes: %u.\n",
    __func__, dst.length, dst.offset, offset, nbytes);
    if (!IS_ALIGNED(dst.offset, HIFN_D_DST_DALIGN) ||
    !IS_ALIGNED(dst.length, HIFN_D_DST_DALIGN) ||
    offset) {
    let mut slen: unsigned = min(dst.length - offset, nbytes);
    let mut dlen: unsigned = PAGE_SIZE;
    t = &w.cache[idx];
    err = skcipher_add(&dlen, dst, slen, &nbytes);
    if (err < 0)
    return err;
    idx += err;
    copy = slen & ~(HIFN_D_DST_DALIGN - 1);
    diff = slen & (HIFN_D_DST_DALIGN - 1);
    if (dlen < nbytes) {
//
// Destination page does not have enough space
// to put there additional blocksized chunk,
// so we mark that page as containing only
// blocksize aligned chunks:
// t->length = (slen & ~(HIFN_D_DST_DALIGN - 1));
// and increase number of bytes to be processed
// in next chunk:
// nbytes += diff;
//
    nbytes += diff;
//
// Temporary of course...
// Kick author if you will catch this one.
//
    pr_err("%s: dlen: %u, nbytes: %u, slen: %u, offset: %u.\n",
    __func__, dlen, nbytes, slen, offset);
    pr_err("%s: please contact author to fix this "
    "issue, generally you should not catch "
    "this path under any condition but who "
    "knows how did you use crypto code.\n"
    "Thank you.\n",	__func__);
    BUG();
    } else {
    copy += diff + nbytes;
    dst = &req.dst[idx];
    err = skcipher_add(&dlen, dst, nbytes, &nbytes);
    if (err < 0)
    return err;
    idx += err;
    }
    t.length = copy;
    t.offset = offset;
    } else {
    nbytes -= min(dst.length, nbytes);
    idx++;
    }
    tidx++;
    }
    return tidx;
    }
#[no_mangle]
unsafe extern "C" fn hifn_setup_session(req: *mut skcipher_request) -> c_int {
    static int hifn_setup_session(struct skcipher_request *req)
    {
    struct hifn_context *ctx = crypto_tfm_ctx(req.base.tfm);
    struct hifn_request_context *rctx = skcipher_request_ctx(req);
    struct hifn_device *dev = ctx.dev;
    unsigned long dlen, flags;
    let mut nbytes: c_uint = req.cryptlen, idx = 0;
    let mut err: c_int = -EINVAL, sg_num;
    struct scatterlist *dst;
    if (rctx.iv && !rctx.ivsize && rctx.mode != ACRYPTO_MODE_ECB)
    goto err_out_exit;
    rctx.walk.flags = 0;
    while (nbytes) {
    dst = &req.dst[idx];
    dlen = min(dst.length, nbytes);
    if (!IS_ALIGNED(dst.offset, HIFN_D_DST_DALIGN) ||
    !IS_ALIGNED(dlen, HIFN_D_DST_DALIGN))
    rctx.walk.flags |= ASYNC_FLAGS_MISALIGNED;
    nbytes -= dlen;
    idx++;
    }
    if (rctx.walk.flags & ASYNC_FLAGS_MISALIGNED) {
    err = hifn_cipher_walk_init(&rctx.walk, idx, GFP_ATOMIC);
    if (err < 0)
    return err;
    }
    sg_num = hifn_cipher_walk(req, &rctx.walk);
    if (sg_num < 0) {
    err = sg_num;
    goto err_out_exit;
    }
    spin_lock_irqsave(&dev.lock, flags);
    if (dev.started + sg_num > HIFN_QUEUE_LENGTH) {
    err = -EAGAIN;
    goto err_out;
    }
    err = hifn_setup_dma(dev, ctx, rctx, req.src, req.dst, req.cryptlen, req);
    if (err)
    goto err_out;
    dev.snum++;
    dev.active = HIFN_DEFAULT_ACTIVE_NUM;
    spin_unlock_irqrestore(&dev.lock, flags);
    return 0;
    err_out:
    spin_unlock_irqrestore(&dev.lock, flags);
    err_out_exit:
    if (err) {
    dev_info(&dev.pdev.dev, "iv: %p [%d], key: %p [%d], mode: %u, op: %u, "
    "type: %u, err: %d.\n",
    rctx.iv, rctx.ivsize,
    ctx.key, ctx.keysize,
    rctx.mode, rctx.op, rctx.type, err);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hifn_start_device(dev: *mut hifn_device) -> c_int {
    static int hifn_start_device(struct hifn_device *dev)
    {
    int err;
    dev.started = dev.active = 0;
    hifn_reset_dma(dev, 1);
    err = hifn_enable_crypto(dev);
    if (err)
    return err;
    hifn_reset_puc(dev);
    hifn_init_dma(dev);
    hifn_init_registers(dev);
    hifn_init_pubrng(dev);
    return 0;
    }
    static int skcipher_get(void *saddr, unsigned int *srestp, unsigned int offset,
    struct scatterlist *dst, unsigned int size, unsigned int *nbytesp)
    {
    let mut srest: c_uint = *srestp, nbytes = *nbytesp, copy;
    void *daddr;
    let mut idx: c_int = 0;
    if (srest < size || size > nbytes)
    return -EINVAL;
    while (size) {
    copy = min3(srest, dst.length, size);
    daddr = kmap_atomic(sg_page(dst));
    memcpy(daddr + dst.offset + offset, saddr, copy);
    kunmap_atomic(daddr);
    nbytes -= copy;
    size -= copy;
    srest -= copy;
    saddr += copy;
    offset = 0;
    pr_debug("%s: copy: %u, size: %u, srest: %u, nbytes: %u.\n",
    __func__, copy, size, srest, nbytes);
    dst++;
    idx++;
    }
// nbytesp = nbytes;
// srestp = srest;
    return idx;
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_complete_sa(dev: *mut hifn_device, i: c_int) {
    static inline void hifn_complete_sa(struct hifn_device *dev, int i)
    {
    unsigned long flags;
    spin_lock_irqsave(&dev.lock, flags);
    dev.sa[i] = core::ptr::null_mut();
    dev.started--;
    if (dev.started < 0)
    dev_info(&dev.pdev.dev, "%s: started: %d.\n", __func__,
    dev.started);
    spin_unlock_irqrestore(&dev.lock, flags);
    BUG_ON(dev.started < 0);
    }
#[no_mangle]
unsafe extern "C" fn hifn_process_ready(req: *mut skcipher_request, error: c_int) {
    static void hifn_process_ready(struct skcipher_request *req, int error)
    {
    struct hifn_request_context *rctx = skcipher_request_ctx(req);
    if (rctx.walk.flags & ASYNC_FLAGS_MISALIGNED) {
    let mut nbytes: c_uint = req.cryptlen;
    let mut idx: c_int = 0, err;
    struct scatterlist *dst, *t;
    void *saddr;
    while (nbytes) {
    t = &rctx.walk.cache[idx];
    dst = &req.dst[idx];
    pr_debug("\n%s: sg_page(t): %p, t.length: %u, "
    "sg_page(dst): %p, dst.length: %u, "
    "nbytes: %u.\n",
    __func__, sg_page(t), t.length,
    sg_page(dst), dst.length, nbytes);
    if (!t.length) {
    nbytes -= min(dst.length, nbytes);
    idx++;
    continue;
    }
    saddr = kmap_atomic(sg_page(t));
    err = skcipher_get(saddr, &t.length, t.offset,
    dst, nbytes, &nbytes);
    if (err < 0) {
    kunmap_atomic(saddr);
    break;
    }
    idx += err;
    kunmap_atomic(saddr);
    }
    hifn_cipher_walk_exit(&rctx.walk);
    }
    skcipher_request_complete(req, error);
    }
#[no_mangle]
unsafe extern "C" fn hifn_clear_rings(dev: *mut hifn_device, error: c_int) {
    static void hifn_clear_rings(struct hifn_device *dev, int error)
    {
    struct hifn_dma *dma = dev.desc_virt;
    int i, u;
    dev_dbg(&dev.pdev.dev, "ring cleanup 1: i: %d.%d.%d.%d, u: %d.%d.%d.%d, "
    "k: %d.%d.%d.%d.\n",
    dma.cmdi, dma.srci, dma.dsti, dma.resi,
    dma.cmdu, dma.srcu, dma.dstu, dma.resu,
    dma.cmdk, dma.srck, dma.dstk, dma.resk);
    i = dma.resk; u = dma.resu;
    while (u != 0) {
    if (dma.resr[i].l & __cpu_to_le32(HIFN_D_VALID))
    break;
    if (dev.sa[i]) {
    dev.success++;
    dev.reset = 0;
    hifn_process_ready(dev.sa[i], error);
    hifn_complete_sa(dev, i);
    }
    if (++i == HIFN_D_RES_RSIZE)
    i = 0;
    u--;
    }
    dma.resk = i; dma.resu = u;
    i = dma.srck; u = dma.srcu;
    while (u != 0) {
    if (dma.srcr[i].l & __cpu_to_le32(HIFN_D_VALID))
    break;
    if (++i == HIFN_D_SRC_RSIZE)
    i = 0;
    u--;
    }
    dma.srck = i; dma.srcu = u;
    i = dma.cmdk; u = dma.cmdu;
    while (u != 0) {
    if (dma.cmdr[i].l & __cpu_to_le32(HIFN_D_VALID))
    break;
    if (++i == HIFN_D_CMD_RSIZE)
    i = 0;
    u--;
    }
    dma.cmdk = i; dma.cmdu = u;
    i = dma.dstk; u = dma.dstu;
    while (u != 0) {
    if (dma.dstr[i].l & __cpu_to_le32(HIFN_D_VALID))
    break;
    if (++i == HIFN_D_DST_RSIZE)
    i = 0;
    u--;
    }
    dma.dstk = i; dma.dstu = u;
    dev_dbg(&dev.pdev.dev, "ring cleanup 2: i: %d.%d.%d.%d, u: %d.%d.%d.%d, "
    "k: %d.%d.%d.%d.\n",
    dma.cmdi, dma.srci, dma.dsti, dma.resi,
    dma.cmdu, dma.srcu, dma.dstu, dma.resu,
    dma.cmdk, dma.srck, dma.dstk, dma.resk);
    }
#[no_mangle]
unsafe extern "C" fn hifn_work(work: *mut work_struct) {
    static void hifn_work(struct work_struct *work)
    {
    struct delayed_work *dw = to_delayed_work(work);
    struct hifn_device *dev = container_of(dw, struct hifn_device, work);
    unsigned long flags;
    let mut reset: c_int = 0;
    let mut r: u32 = 0;
    spin_lock_irqsave(&dev.lock, flags);
    if (dev.active == 0) {
    struct hifn_dma *dma = dev.desc_virt;
    if (dma.cmdu == 0 && (dev.flags & HIFN_FLAG_CMD_BUSY)) {
    dev.flags &= ~HIFN_FLAG_CMD_BUSY;
    r |= HIFN_DMACSR_C_CTRL_DIS;
    }
    if (dma.srcu == 0 && (dev.flags & HIFN_FLAG_SRC_BUSY)) {
    dev.flags &= ~HIFN_FLAG_SRC_BUSY;
    r |= HIFN_DMACSR_S_CTRL_DIS;
    }
    if (dma.dstu == 0 && (dev.flags & HIFN_FLAG_DST_BUSY)) {
    dev.flags &= ~HIFN_FLAG_DST_BUSY;
    r |= HIFN_DMACSR_D_CTRL_DIS;
    }
    if (dma.resu == 0 && (dev.flags & HIFN_FLAG_RES_BUSY)) {
    dev.flags &= ~HIFN_FLAG_RES_BUSY;
    r |= HIFN_DMACSR_R_CTRL_DIS;
    }
    if (r)
    hifn_write_1(dev, HIFN_1_DMA_CSR, r);
    } else
    dev.active--;
    if ((dev.prev_success == dev.success) && dev.started)
    reset = 1;
    dev.prev_success = dev.success;
    spin_unlock_irqrestore(&dev.lock, flags);
    if (reset) {
    if (++dev.reset >= 5) {
    int i;
    struct hifn_dma *dma = dev.desc_virt;
    dev_info(&dev.pdev.dev,
    "r: %08x, active: %d, started: %d, "
    "success: %lu: qlen: %u/%u, reset: %d.\n",
    r, dev.active, dev.started,
    dev.success, dev.queue.qlen, dev.queue.max_qlen,
    reset);
    dev_info(&dev.pdev.dev, "%s: res: ", __func__);
    for (i = 0; i < HIFN_D_RES_RSIZE; ++i) {
    pr_info("%x.%p ", dma.resr[i].l, dev.sa[i]);
    if (dev.sa[i]) {
    hifn_process_ready(dev.sa[i], -ENODEV);
    hifn_complete_sa(dev, i);
    }
    }
    pr_info("\n");
    hifn_reset_dma(dev, 1);
    hifn_stop_device(dev);
    hifn_start_device(dev);
    dev.reset = 0;
    }
    tasklet_schedule(&dev.tasklet);
    }
    schedule_delayed_work(&dev.work, HZ);
    }
#[no_mangle]
unsafe extern "C" fn hifn_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t hifn_interrupt(int irq, void *data)
    {
    struct hifn_device *dev = data;
    struct hifn_dma *dma = dev.desc_virt;
    u32 dmacsr, restart;
    dmacsr = hifn_read_1(dev, HIFN_1_DMA_CSR);
    dev_dbg(&dev.pdev.dev, "1 dmacsr: %08x, dmareg: %08x, res: %08x [%d], "
    "i: %d.%d.%d.%d, u: %d.%d.%d.%d.\n",
    dmacsr, dev.dmareg, dmacsr & dev.dmareg, dma.cmdi,
    dma.cmdi, dma.srci, dma.dsti, dma.resi,
    dma.cmdu, dma.srcu, dma.dstu, dma.resu);
    if ((dmacsr & dev.dmareg) == 0)
    return IRQ_NONE;
    hifn_write_1(dev, HIFN_1_DMA_CSR, dmacsr & dev.dmareg);
    if (dmacsr & HIFN_DMACSR_ENGINE)
    hifn_write_0(dev, HIFN_0_PUISR, hifn_read_0(dev, HIFN_0_PUISR));
    if (dmacsr & HIFN_DMACSR_PUBDONE)
    hifn_write_1(dev, HIFN_1_PUB_STATUS,
    hifn_read_1(dev, HIFN_1_PUB_STATUS) | HIFN_PUBSTS_DONE);
    restart = dmacsr & (HIFN_DMACSR_R_OVER | HIFN_DMACSR_D_OVER);
    if (restart) {
    let mut puisr: u32 = hifn_read_0(dev, HIFN_0_PUISR);
    dev_warn(&dev.pdev.dev, "overflow: r: %d, d: %d, puisr: %08x, d: %u.\n",
    !!(dmacsr & HIFN_DMACSR_R_OVER),
    !!(dmacsr & HIFN_DMACSR_D_OVER),
    puisr, !!(puisr & HIFN_PUISR_DSTOVER));
    if (!!(puisr & HIFN_PUISR_DSTOVER))
    hifn_write_0(dev, HIFN_0_PUISR, HIFN_PUISR_DSTOVER);
    hifn_write_1(dev, HIFN_1_DMA_CSR, dmacsr & (HIFN_DMACSR_R_OVER |
    HIFN_DMACSR_D_OVER));
    }
    restart = dmacsr & (HIFN_DMACSR_C_ABORT | HIFN_DMACSR_S_ABORT |
    HIFN_DMACSR_D_ABORT | HIFN_DMACSR_R_ABORT);
    if (restart) {
    dev_warn(&dev.pdev.dev, "abort: c: %d, s: %d, d: %d, r: %d.\n",
    !!(dmacsr & HIFN_DMACSR_C_ABORT),
    !!(dmacsr & HIFN_DMACSR_S_ABORT),
    !!(dmacsr & HIFN_DMACSR_D_ABORT),
    !!(dmacsr & HIFN_DMACSR_R_ABORT));
    hifn_reset_dma(dev, 1);
    hifn_init_dma(dev);
    hifn_init_registers(dev);
    }
    if ((dmacsr & HIFN_DMACSR_C_WAIT) && (dma.cmdu == 0)) {
    dev_dbg(&dev.pdev.dev, "wait on command.\n");
    dev.dmareg &= ~(HIFN_DMAIER_C_WAIT);
    hifn_write_1(dev, HIFN_1_DMA_IER, dev.dmareg);
    }
    tasklet_schedule(&dev.tasklet);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hifn_flush(dev: *mut hifn_device) {
    static void hifn_flush(struct hifn_device *dev)
    {
    unsigned long flags;
    struct crypto_async_request *async_req;
    struct skcipher_request *req;
    struct hifn_dma *dma = dev.desc_virt;
    int i;
    for (i = 0; i < HIFN_D_RES_RSIZE; ++i) {
    struct hifn_desc *d = &dma.resr[i];
    if (dev.sa[i]) {
    hifn_process_ready(dev.sa[i],
    (d.l & __cpu_to_le32(HIFN_D_VALID)) ? -ENODEV : 0);
    hifn_complete_sa(dev, i);
    }
    }
    spin_lock_irqsave(&dev.lock, flags);
    while ((async_req = crypto_dequeue_request(&dev.queue))) {
    req = skcipher_request_cast(async_req);
    spin_unlock_irqrestore(&dev.lock, flags);
    hifn_process_ready(req, -ENODEV);
    spin_lock_irqsave(&dev.lock, flags);
    }
    spin_unlock_irqrestore(&dev.lock, flags);
    }
    static int hifn_setkey(struct crypto_skcipher *cipher, const u8 *key,
    unsigned int len)
    {
    struct hifn_context *ctx = crypto_skcipher_ctx(cipher);
    struct hifn_device *dev = ctx.dev;
    int err;
    err = verify_skcipher_des_key(cipher, key);
    if (err)
    return err;
    dev.flags &= ~HIFN_FLAG_OLD_KEY;
    memcpy(ctx.key, key, len);
    ctx.keysize = len;
    return 0;
    }
    static int hifn_des3_setkey(struct crypto_skcipher *cipher, const u8 *key,
    unsigned int len)
    {
    struct hifn_context *ctx = crypto_skcipher_ctx(cipher);
    struct hifn_device *dev = ctx.dev;
    int err;
    err = verify_skcipher_des3_key(cipher, key);
    if (err)
    return err;
    dev.flags &= ~HIFN_FLAG_OLD_KEY;
    memcpy(ctx.key, key, len);
    ctx.keysize = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hifn_handle_req(req: *mut skcipher_request) -> c_int {
    static int hifn_handle_req(struct skcipher_request *req)
    {
    struct hifn_context *ctx = crypto_tfm_ctx(req.base.tfm);
    struct hifn_device *dev = ctx.dev;
    let mut err: c_int = -EAGAIN;
    if (dev.started + DIV_ROUND_UP(req.cryptlen, PAGE_SIZE) <= HIFN_QUEUE_LENGTH)
    err = hifn_setup_session(req);
    if (err == -EAGAIN) {
    unsigned long flags;
    spin_lock_irqsave(&dev.lock, flags);
    err = crypto_enqueue_request(&dev.queue, &req.base);
    spin_unlock_irqrestore(&dev.lock, flags);
    }
    return err;
    }
    static int hifn_setup_crypto_req(struct skcipher_request *req, u8 op,
    u8 type, u8 mode)
    {
    struct hifn_context *ctx = crypto_tfm_ctx(req.base.tfm);
    struct hifn_request_context *rctx = skcipher_request_ctx(req);
    unsigned ivsize;
    ivsize = crypto_skcipher_ivsize(crypto_skcipher_reqtfm(req));
    if (req.iv && mode != ACRYPTO_MODE_ECB) {
    if (type == ACRYPTO_TYPE_AES_128)
    ivsize = HIFN_AES_IV_LENGTH;
#[no_mangle]
pub unsafe extern "C" fn if(ACRYPTO_TYPE_DES: type ==) -> else {
    else if (type == ACRYPTO_TYPE_DES)
    ivsize = HIFN_DES_KEY_LENGTH;
#[no_mangle]
pub unsafe extern "C" fn if(ACRYPTO_TYPE_3DES: type ==) -> else {
    else if (type == ACRYPTO_TYPE_3DES)
    ivsize = HIFN_3DES_KEY_LENGTH;
    }
    if (ctx.keysize != 16 && type == ACRYPTO_TYPE_AES_128) {
    if (ctx.keysize == 24)
    type = ACRYPTO_TYPE_AES_192;
#[no_mangle]
pub unsafe extern "C" fn if(32: ctx->keysize ==) -> else {
    else if (ctx.keysize == 32)
    type = ACRYPTO_TYPE_AES_256;
    }
    rctx.op = op;
    rctx.mode = mode;
    rctx.type = type;
    rctx.iv = req.iv;
    rctx.ivsize = ivsize;
//
// HEAVY TODO: needs to kick Herbert XU to write documentation.
//
    return hifn_handle_req(req);
    }
#[no_mangle]
unsafe extern "C" fn hifn_process_queue(dev: *mut hifn_device) -> c_int {
    static int hifn_process_queue(struct hifn_device *dev)
    {
    struct crypto_async_request *async_req, *backlog;
    struct skcipher_request *req;
    unsigned long flags;
    let mut err: c_int = 0;
    while (dev.started < HIFN_QUEUE_LENGTH) {
    spin_lock_irqsave(&dev.lock, flags);
    backlog = crypto_get_backlog(&dev.queue);
    async_req = crypto_dequeue_request(&dev.queue);
    spin_unlock_irqrestore(&dev.lock, flags);
    if (!async_req)
    break;
    if (backlog)
    crypto_request_complete(backlog, -EINPROGRESS);
    req = skcipher_request_cast(async_req);
    err = hifn_handle_req(req);
    if (err)
    break;
    }
    return err;
    }
    static int hifn_setup_crypto(struct skcipher_request *req, u8 op,
    u8 type, u8 mode)
    {
    int err;
    struct hifn_context *ctx = crypto_tfm_ctx(req.base.tfm);
    struct hifn_device *dev = ctx.dev;
    err = hifn_setup_crypto_req(req, op, type, mode);
    if (err)
    return err;
    if (dev.started < HIFN_QUEUE_LENGTH &&	dev.queue.qlen)
    hifn_process_queue(dev);
    return -EINPROGRESS;
    }
//
// AES ecryption functions.
//
#[no_mangle]
pub unsafe extern "C" fn hifn_encrypt_aes_ecb(req: *mut skcipher_request) -> c_int {
    static inline int hifn_encrypt_aes_ecb(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_ENCRYPT,
    ACRYPTO_TYPE_AES_128, ACRYPTO_MODE_ECB);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_encrypt_aes_cbc(req: *mut skcipher_request) -> c_int {
    static inline int hifn_encrypt_aes_cbc(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_ENCRYPT,
    ACRYPTO_TYPE_AES_128, ACRYPTO_MODE_CBC);
    }
//
// AES decryption functions.
//
#[no_mangle]
pub unsafe extern "C" fn hifn_decrypt_aes_ecb(req: *mut skcipher_request) -> c_int {
    static inline int hifn_decrypt_aes_ecb(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_DECRYPT,
    ACRYPTO_TYPE_AES_128, ACRYPTO_MODE_ECB);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_decrypt_aes_cbc(req: *mut skcipher_request) -> c_int {
    static inline int hifn_decrypt_aes_cbc(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_DECRYPT,
    ACRYPTO_TYPE_AES_128, ACRYPTO_MODE_CBC);
    }
//
// DES ecryption functions.
//
#[no_mangle]
pub unsafe extern "C" fn hifn_encrypt_des_ecb(req: *mut skcipher_request) -> c_int {
    static inline int hifn_encrypt_des_ecb(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_ENCRYPT,
    ACRYPTO_TYPE_DES, ACRYPTO_MODE_ECB);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_encrypt_des_cbc(req: *mut skcipher_request) -> c_int {
    static inline int hifn_encrypt_des_cbc(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_ENCRYPT,
    ACRYPTO_TYPE_DES, ACRYPTO_MODE_CBC);
    }
//
// DES decryption functions.
//
#[no_mangle]
pub unsafe extern "C" fn hifn_decrypt_des_ecb(req: *mut skcipher_request) -> c_int {
    static inline int hifn_decrypt_des_ecb(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_DECRYPT,
    ACRYPTO_TYPE_DES, ACRYPTO_MODE_ECB);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_decrypt_des_cbc(req: *mut skcipher_request) -> c_int {
    static inline int hifn_decrypt_des_cbc(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_DECRYPT,
    ACRYPTO_TYPE_DES, ACRYPTO_MODE_CBC);
    }
//
// 3DES ecryption functions.
//
#[no_mangle]
pub unsafe extern "C" fn hifn_encrypt_3des_ecb(req: *mut skcipher_request) -> c_int {
    static inline int hifn_encrypt_3des_ecb(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_ENCRYPT,
    ACRYPTO_TYPE_3DES, ACRYPTO_MODE_ECB);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_encrypt_3des_cbc(req: *mut skcipher_request) -> c_int {
    static inline int hifn_encrypt_3des_cbc(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_ENCRYPT,
    ACRYPTO_TYPE_3DES, ACRYPTO_MODE_CBC);
    }
// 3DES decryption functions.
#[no_mangle]
pub unsafe extern "C" fn hifn_decrypt_3des_ecb(req: *mut skcipher_request) -> c_int {
    static inline int hifn_decrypt_3des_ecb(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_DECRYPT,
    ACRYPTO_TYPE_3DES, ACRYPTO_MODE_ECB);
    }
#[no_mangle]
pub unsafe extern "C" fn hifn_decrypt_3des_cbc(req: *mut skcipher_request) -> c_int {
    static inline int hifn_decrypt_3des_cbc(struct skcipher_request *req)
    {
    return hifn_setup_crypto(req, ACRYPTO_OP_DECRYPT,
    ACRYPTO_TYPE_3DES, ACRYPTO_MODE_CBC);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifn_alg_template {
    pub name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub drv_name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub bsize: c_uint,
    pub skcipher: skcipher_alg,
}

    static const struct hifn_alg_template hifn_alg_templates[] = {
//
// 3DES ECB and CBC modes.
//
    {
    .name = "cbc(des3_ede)", .drv_name = "cbc-3des", .bsize = 8,
    .skcipher = {
    .ivsize		=	HIFN_IV_LENGTH,
    .min_keysize	=	HIFN_3DES_KEY_LENGTH,
    .max_keysize	=	HIFN_3DES_KEY_LENGTH,
    .setkey		=	hifn_des3_setkey,
    .encrypt	=	hifn_encrypt_3des_cbc,
    .decrypt	=	hifn_decrypt_3des_cbc,
    },
    },
    {
    .name = "ecb(des3_ede)", .drv_name = "ecb-3des", .bsize = 8,
    .skcipher = {
    .min_keysize	=	HIFN_3DES_KEY_LENGTH,
    .max_keysize	=	HIFN_3DES_KEY_LENGTH,
    .setkey		=	hifn_des3_setkey,
    .encrypt	=	hifn_encrypt_3des_ecb,
    .decrypt	=	hifn_decrypt_3des_ecb,
    },
    },
//
// DES ECB and CBC modes.
//
    {
    .name = "cbc(des)", .drv_name = "cbc-des", .bsize = 8,
    .skcipher = {
    .ivsize		=	HIFN_IV_LENGTH,
    .min_keysize	=	HIFN_DES_KEY_LENGTH,
    .max_keysize	=	HIFN_DES_KEY_LENGTH,
    .setkey		=	hifn_setkey,
    .encrypt	=	hifn_encrypt_des_cbc,
    .decrypt	=	hifn_decrypt_des_cbc,
    },
    },
    {
    .name = "ecb(des)", .drv_name = "ecb-des", .bsize = 8,
    .skcipher = {
    .min_keysize	=	HIFN_DES_KEY_LENGTH,
    .max_keysize	=	HIFN_DES_KEY_LENGTH,
    .setkey		=	hifn_setkey,
    .encrypt	=	hifn_encrypt_des_ecb,
    .decrypt	=	hifn_decrypt_des_ecb,
    },
    },
//
// AES ECB and CBC modes.
//
    {
    .name = "ecb(aes)", .drv_name = "ecb-aes", .bsize = 16,
    .skcipher = {
    .min_keysize	=	AES_MIN_KEY_SIZE,
    .max_keysize	=	AES_MAX_KEY_SIZE,
    .setkey		=	hifn_setkey,
    .encrypt	=	hifn_encrypt_aes_ecb,
    .decrypt	=	hifn_decrypt_aes_ecb,
    },
    },
    {
    .name = "cbc(aes)", .drv_name = "cbc-aes", .bsize = 16,
    .skcipher = {
    .ivsize		=	HIFN_AES_IV_LENGTH,
    .min_keysize	=	AES_MIN_KEY_SIZE,
    .max_keysize	=	AES_MAX_KEY_SIZE,
    .setkey		=	hifn_setkey,
    .encrypt	=	hifn_encrypt_aes_cbc,
    .decrypt	=	hifn_decrypt_aes_cbc,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn hifn_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int hifn_init_tfm(struct crypto_skcipher *tfm)
    {
    struct skcipher_alg *alg = crypto_skcipher_alg(tfm);
    struct hifn_crypto_alg *ha = crypto_alg_to_hifn(alg);
    struct hifn_context *ctx = crypto_skcipher_ctx(tfm);
    ctx.dev = ha.dev;
    crypto_skcipher_set_reqsize(tfm, sizeof(struct hifn_request_context));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hifn_alg_alloc(dev: *mut hifn_device, t: *const hifn_alg_template) -> c_int {
    static int hifn_alg_alloc(struct hifn_device *dev, const struct hifn_alg_template *t)
    {
    struct hifn_crypto_alg *alg;
    int err;
    alg = kzalloc_obj(*alg);
    if (!alg)
    return -ENOMEM;
    alg.alg = t.skcipher;
    alg.alg.init = hifn_init_tfm;
    err = -EINVAL;
    if (strscpy(alg.alg.base.cra_name, t.name) < 0)
    goto out_free_alg;
    if (snprintf(alg.alg.base.cra_driver_name, CRYPTO_MAX_ALG_NAME,
    "%s-%s", t.drv_name, dev.name) >= CRYPTO_MAX_ALG_NAME)
    goto out_free_alg;
    alg.alg.base.cra_priority = 300;
    alg.alg.base.cra_flags = CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_ASYNC;
    alg.alg.base.cra_blocksize = t.bsize;
    alg.alg.base.cra_ctxsize = sizeof(struct hifn_context);
    alg.alg.base.cra_alignmask = 0;
    alg.alg.base.cra_module = THIS_MODULE;
    alg.dev = dev;
    list_add_tail(&alg.entry, &dev.alg_list);
    err = crypto_register_skcipher(&alg.alg);
    if (err) {
    list_del(&alg.entry);
    out_free_alg:
    kfree(alg);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hifn_unregister_alg(dev: *mut hifn_device) {
    static void hifn_unregister_alg(struct hifn_device *dev)
    {
    struct hifn_crypto_alg *a, *n;
    list_for_each_entry_safe(a, n, &dev.alg_list, entry) {
    list_del(&a.entry);
    crypto_unregister_skcipher(&a.alg);
    kfree(a);
    }
    }
#[no_mangle]
unsafe extern "C" fn hifn_register_alg(dev: *mut hifn_device) -> c_int {
    static int hifn_register_alg(struct hifn_device *dev)
    {
    int i, err;
    for (i = 0; i < ARRAY_SIZE(hifn_alg_templates); ++i) {
    err = hifn_alg_alloc(dev, &hifn_alg_templates[i]);
    if (err)
    goto err_out_exit;
    }
    return 0;
    err_out_exit:
    hifn_unregister_alg(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hifn_tasklet_callback(data: c_ulong) {
    static void hifn_tasklet_callback(unsigned long data)
    {
    struct hifn_device *dev = (struct hifn_device *)data;
//
// This is ok to call this without lock being held,
// althogh it modifies some parameters used in parallel,
// (like dev->success), but they are used in process
// context or update is atomic (like setting dev->sa[i] to NULL).
//
    hifn_clear_rings(dev, 0);
    if (dev.started < HIFN_QUEUE_LENGTH &&	dev.queue.qlen)
    hifn_process_queue(dev);
    }
#[no_mangle]
unsafe extern "C" fn hifn_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int hifn_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    int err, i;
    struct hifn_device *dev;
    char name[8];
    err = pci_enable_device(pdev);
    if (err)
    return err;
    pci_set_master(pdev);
    err = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (err)
    goto err_out_disable_pci_device;
    snprintf(name, sizeof(name), "hifn%d",
    atomic_inc_return(&hifn_dev_number) - 1);
    err = pci_request_regions(pdev, name);
    if (err)
    goto err_out_disable_pci_device;
    if (pci_resource_len(pdev, 0) < HIFN_BAR0_SIZE ||
    pci_resource_len(pdev, 1) < HIFN_BAR1_SIZE ||
    pci_resource_len(pdev, 2) < HIFN_BAR2_SIZE) {
    dev_err(&pdev.dev, "Broken hardware - I/O regions are too small.\n");
    err = -ENODEV;
    goto err_out_free_regions;
    }
    dev = kzalloc(sizeof(struct hifn_device) + sizeof(struct crypto_alg),
    GFP_KERNEL);
    if (!dev) {
    err = -ENOMEM;
    goto err_out_free_regions;
    }
    INIT_LIST_HEAD(&dev.alg_list);
    strscpy(dev.name, name);
    spin_lock_init(&dev.lock);
    for (i = 0; i < 3; ++i) {
    unsigned long addr, size;
    addr = pci_resource_start(pdev, i);
    size = pci_resource_len(pdev, i);
    dev.bar[i] = ioremap(addr, size);
    if (!dev.bar[i]) {
    err = -ENOMEM;
    goto err_out_unmap_bars;
    }
    }
    dev.desc_virt = dma_alloc_coherent(&pdev.dev,
    sizeof(struct hifn_dma),
    &dev.desc_dma, GFP_KERNEL);
    if (!dev.desc_virt) {
    dev_err(&pdev.dev, "Failed to allocate descriptor rings.\n");
    err = -ENOMEM;
    goto err_out_unmap_bars;
    }
    dev.pdev = pdev;
    dev.irq = pdev.irq;
    for (i = 0; i < HIFN_D_RES_RSIZE; ++i)
    dev.sa[i] = core::ptr::null_mut();
    pci_set_drvdata(pdev, dev);
    tasklet_init(&dev.tasklet, hifn_tasklet_callback, (unsigned long)dev);
    crypto_init_queue(&dev.queue, 1);
    err = request_irq(dev.irq, hifn_interrupt, IRQF_SHARED, dev.name, dev);
    if (err) {
    dev_err(&pdev.dev, "Failed to request IRQ%d: err: %d.\n",
    dev.irq, err);
    dev.irq = 0;
    goto err_out_free_desc;
    }
    err = hifn_start_device(dev);
    if (err)
    goto err_out_free_irq;
    err = hifn_register_rng(dev);
    if (err)
    goto err_out_stop_device;
    err = hifn_register_alg(dev);
    if (err)
    goto err_out_unregister_rng;
    INIT_DELAYED_WORK(&dev.work, hifn_work);
    schedule_delayed_work(&dev.work, HZ);
    dev_dbg(&pdev.dev, "HIFN crypto accelerator card at %s has been "
    "successfully registered as %s.\n",
    pci_name(pdev), dev.name);
    return 0;
    err_out_unregister_rng:
    hifn_unregister_rng(dev);
    err_out_stop_device:
    hifn_reset_dma(dev, 1);
    hifn_stop_device(dev);
    err_out_free_irq:
    free_irq(dev.irq, dev);
    tasklet_kill(&dev.tasklet);
    err_out_free_desc:
    dma_free_coherent(&pdev.dev, sizeof(struct hifn_dma), dev.desc_virt,
    dev.desc_dma);
    err_out_unmap_bars:
    for (i = 0; i < 3; ++i)
    if (dev.bar[i])
    iounmap(dev.bar[i]);
    kfree(dev);
    err_out_free_regions:
    pci_release_regions(pdev);
    err_out_disable_pci_device:
    pci_disable_device(pdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hifn_remove(pdev: *mut pci_dev) {
    static void hifn_remove(struct pci_dev *pdev)
    {
    int i;
    struct hifn_device *dev;
    dev = pci_get_drvdata(pdev);
    if (dev) {
    cancel_delayed_work_sync(&dev.work);
    hifn_unregister_rng(dev);
    hifn_unregister_alg(dev);
    hifn_reset_dma(dev, 1);
    hifn_stop_device(dev);
    free_irq(dev.irq, dev);
    tasklet_kill(&dev.tasklet);
    hifn_flush(dev);
    dma_free_coherent(&pdev.dev, sizeof(struct hifn_dma),
    dev.desc_virt, dev.desc_dma);
    for (i = 0; i < 3; ++i)
    if (dev.bar[i])
    iounmap(dev.bar[i]);
    kfree(dev);
    }
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static struct pci_device_id hifn_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_HIFN, PCI_DEVICE_ID_HIFN_7955) },
    { PCI_DEVICE(PCI_VENDOR_ID_HIFN, PCI_DEVICE_ID_HIFN_7956) },
    { 0 }
    };
    MODULE_DEVICE_TABLE(pci, hifn_pci_tbl);
    static struct pci_driver hifn_pci_driver = {
    .name     = "hifn795x",
    .id_table = hifn_pci_tbl,
    .probe    = hifn_probe,
    .remove   = hifn_remove,
    };
#[no_mangle]
unsafe extern "C" fn hifn_init() -> int __init {
    static int __init hifn_init(void)
    {
    unsigned int freq;
    int err;
    if (strncmp(hifn_pll_ref, "ext", 3) &&
    strncmp(hifn_pll_ref, "pci", 3)) {
    pr_err("hifn795x: invalid hifn_pll_ref clock, must be pci or ext");
    return -EINVAL;
    }
//
// For the 7955/7956 the reference clock frequency must be in the
// range of 20MHz-100MHz. For the 7954 the upper bound is 66.67MHz,
// but this chip is currently not supported.
//
    if (hifn_pll_ref[3] != '\0') {
    freq = simple_strtoul(hifn_pll_ref + 3, core::ptr::null_mut(), 10);
    if (freq < 20 || freq > 100) {
    pr_err("hifn795x: invalid hifn_pll_ref frequency, must"
    "be in the range of 20-100");
    return -EINVAL;
    }
    }
    err = pci_register_driver(&hifn_pci_driver);
    if (err < 0) {
    pr_err("Failed to register PCI driver for %s device.\n",
    hifn_pci_driver.name);
    return -ENODEV;
    }
    pr_info("Driver for HIFN 795x crypto accelerator chip "
    "has been successfully registered.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hifn_fini() -> void __exit {
    static void __exit hifn_fini(void)
    {
    pci_unregister_driver(&hifn_pci_driver);
    pr_info("Driver for HIFN 795x crypto accelerator chip "
    "has been successfully unregistered.\n");
    }
    module_init(hifn_init);
    module_exit(hifn_fini);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Evgeniy Polyakov <johnpol@2ka.mipt.ru>");
    MODULE_DESCRIPTION("Driver for HIFN 795x crypto accelerator chip.");
