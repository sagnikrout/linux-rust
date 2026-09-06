//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx25821/cx25821-reg.h
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
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
//
// Risc Instructions
pub const RISC_CNT_INC: c_uint = 0x00010000;
pub const RISC_CNT_RESET: c_uint = 0x00030000;
pub const RISC_IRQ1: c_uint = 0x01000000;
pub const RISC_IRQ2: c_uint = 0x02000000;
pub const RISC_EOL: c_uint = 0x04000000;
pub const RISC_SOL: c_uint = 0x08000000;
pub const RISC_WRITE: c_uint = 0x10000000;
pub const RISC_SKIP: c_uint = 0x20000000;
pub const RISC_JUMP: c_uint = 0x70000000;
pub const RISC_SYNC: c_uint = 0x80000000;
pub const RISC_RESYNC: c_uint = 0x80008000;
pub const RISC_READ: c_uint = 0x90000000;
pub const RISC_WRITERM: c_uint = 0xB0000000;
pub const RISC_WRITECM: c_uint = 0xC0000000;
pub const RISC_WRITECR: c_uint = 0xD0000000;
pub const RISC_WRITEC: c_uint = 0x50000000;
pub const RISC_READC: c_uint = 0xA0000000;
pub const RISC_SYNC_ODD: c_uint = 0x00000000;
pub const RISC_SYNC_EVEN: c_uint = 0x00000200;
pub const RISC_SYNC_ODD_VBI: c_uint = 0x00000006;
pub const RISC_SYNC_EVEN_VBI: c_uint = 0x00000207;
pub const RISC_NOOP: c_uint = 0xF0000000;
//
// ASB SRAM
//
pub const TX_SRAM: c_uint = 0x000000	/* Transmit SRAM */;
//
pub const RX_RAM: c_uint = 0x010000	/* Receive SRAM */;
//
// Application Layer (AL)
//
pub const DEV_CNTRL2: c_uint = 0x040000	/* Device control */;
pub const FLD_RUN_RISC: c_uint = 0x00000020;
// *****************************************************************************
pub const PCI_INT_MSK: c_uint = 0x040010	/* PCI interrupt mask */;
pub const PCI_INT_STAT: c_uint = 0x040014	/* PCI interrupt status */;
pub const PCI_INT_MSTAT: c_uint = 0x040018	/* PCI interrupt masked status */;

// *****************************************************************************
pub const VID_A_INT_MSK: c_uint = 0x040020	/* Video A interrupt mask */;
pub const VID_A_INT_STAT: c_uint = 0x040024	/* Video A interrupt status */;
pub const VID_A_INT_MSTAT: c_uint = 0x040028	/* Video A interrupt masked status */;
pub const VID_A_INT_SSTAT: c_uint = 0x04002C	/* Video A interrupt set status */;
// *****************************************************************************
pub const VID_B_INT_MSK: c_uint = 0x040030	/* Video B interrupt mask */;
pub const VID_B_INT_STAT: c_uint = 0x040034	/* Video B interrupt status */;
pub const VID_B_INT_MSTAT: c_uint = 0x040038	/* Video B interrupt masked status */;
pub const VID_B_INT_SSTAT: c_uint = 0x04003C	/* Video B interrupt set status */;
// *****************************************************************************
pub const VID_C_INT_MSK: c_uint = 0x040040	/* Video C interrupt mask */;
pub const VID_C_INT_STAT: c_uint = 0x040044	/* Video C interrupt status */;
pub const VID_C_INT_MSTAT: c_uint = 0x040048	/* Video C interrupt masked status */;
pub const VID_C_INT_SSTAT: c_uint = 0x04004C	/* Video C interrupt set status */;
// *****************************************************************************
pub const VID_D_INT_MSK: c_uint = 0x040050	/* Video D interrupt mask */;
pub const VID_D_INT_STAT: c_uint = 0x040054	/* Video D interrupt status */;
pub const VID_D_INT_MSTAT: c_uint = 0x040058	/* Video D interrupt masked status */;
pub const VID_D_INT_SSTAT: c_uint = 0x04005C	/* Video D interrupt set status */;
// *****************************************************************************
pub const VID_E_INT_MSK: c_uint = 0x040060	/* Video E interrupt mask */;
pub const VID_E_INT_STAT: c_uint = 0x040064	/* Video E interrupt status */;
pub const VID_E_INT_MSTAT: c_uint = 0x040068	/* Video E interrupt masked status */;
pub const VID_E_INT_SSTAT: c_uint = 0x04006C	/* Video E interrupt set status */;
// *****************************************************************************
pub const VID_F_INT_MSK: c_uint = 0x040070	/* Video F interrupt mask */;
pub const VID_F_INT_STAT: c_uint = 0x040074	/* Video F interrupt status */;
pub const VID_F_INT_MSTAT: c_uint = 0x040078	/* Video F interrupt masked status */;
pub const VID_F_INT_SSTAT: c_uint = 0x04007C	/* Video F interrupt set status */;
// *****************************************************************************
pub const VID_G_INT_MSK: c_uint = 0x040080	/* Video G interrupt mask */;
pub const VID_G_INT_STAT: c_uint = 0x040084	/* Video G interrupt status */;
pub const VID_G_INT_MSTAT: c_uint = 0x040088	/* Video G interrupt masked status */;
pub const VID_G_INT_SSTAT: c_uint = 0x04008C	/* Video G interrupt set status */;
// *****************************************************************************
pub const VID_H_INT_MSK: c_uint = 0x040090	/* Video H interrupt mask */;
pub const VID_H_INT_STAT: c_uint = 0x040094	/* Video H interrupt status */;
pub const VID_H_INT_MSTAT: c_uint = 0x040098	/* Video H interrupt masked status */;
pub const VID_H_INT_SSTAT: c_uint = 0x04009C	/* Video H interrupt set status */;
// *****************************************************************************
pub const VID_I_INT_MSK: c_uint = 0x0400A0	/* Video I interrupt mask */;
pub const VID_I_INT_STAT: c_uint = 0x0400A4	/* Video I interrupt status */;
pub const VID_I_INT_MSTAT: c_uint = 0x0400A8	/* Video I interrupt masked status */;
pub const VID_I_INT_SSTAT: c_uint = 0x0400AC	/* Video I interrupt set status */;
// *****************************************************************************
pub const VID_J_INT_MSK: c_uint = 0x0400B0	/* Video J interrupt mask */;
pub const VID_J_INT_STAT: c_uint = 0x0400B4	/* Video J interrupt status */;
pub const VID_J_INT_MSTAT: c_uint = 0x0400B8	/* Video J interrupt masked status */;
pub const VID_J_INT_SSTAT: c_uint = 0x0400BC	/* Video J interrupt set status */;
pub const FLD_VID_SRC_OPC_ERR: c_uint = 0x00020000;
pub const FLD_VID_DST_OPC_ERR: c_uint = 0x00010000;
pub const FLD_VID_SRC_SYNC: c_uint = 0x00002000;
pub const FLD_VID_DST_SYNC: c_uint = 0x00001000;
pub const FLD_VID_SRC_UF: c_uint = 0x00000200;
pub const FLD_VID_DST_OF: c_uint = 0x00000100;
pub const FLD_VID_SRC_RISC2: c_uint = 0x00000020;
pub const FLD_VID_DST_RISC2: c_uint = 0x00000010;
pub const FLD_VID_SRC_RISC1: c_uint = 0x00000002;
pub const FLD_VID_DST_RISC1: c_uint = 0x00000001;

// *****************************************************************************
pub const AUD_A_INT_MSK: c_uint = 0x0400C0	/* Audio Int interrupt mask */;
pub const AUD_A_INT_STAT: c_uint = 0x0400C4	/* Audio Int interrupt status */;
pub const AUD_A_INT_MSTAT: c_uint = 0x0400C8	/* Audio Int interrupt masked status */;
pub const AUD_A_INT_SSTAT: c_uint = 0x0400CC	/* Audio Int interrupt set status */;
// *****************************************************************************
pub const AUD_B_INT_MSK: c_uint = 0x0400D0	/* Audio Int interrupt mask */;
pub const AUD_B_INT_STAT: c_uint = 0x0400D4	/* Audio Int interrupt status */;
pub const AUD_B_INT_MSTAT: c_uint = 0x0400D8	/* Audio Int interrupt masked status */;
pub const AUD_B_INT_SSTAT: c_uint = 0x0400DC	/* Audio Int interrupt set status */;
// *****************************************************************************
pub const AUD_C_INT_MSK: c_uint = 0x0400E0	/* Audio Int interrupt mask */;
pub const AUD_C_INT_STAT: c_uint = 0x0400E4	/* Audio Int interrupt status */;
pub const AUD_C_INT_MSTAT: c_uint = 0x0400E8	/* Audio Int interrupt masked status */;
pub const AUD_C_INT_SSTAT: c_uint = 0x0400EC	/* Audio Int interrupt set status */;
// *****************************************************************************
pub const AUD_D_INT_MSK: c_uint = 0x0400F0	/* Audio Int interrupt mask */;
pub const AUD_D_INT_STAT: c_uint = 0x0400F4	/* Audio Int interrupt status */;
pub const AUD_D_INT_MSTAT: c_uint = 0x0400F8	/* Audio Int interrupt masked status */;
pub const AUD_D_INT_SSTAT: c_uint = 0x0400FC	/* Audio Int interrupt set status */;
// *****************************************************************************
pub const AUD_E_INT_MSK: c_uint = 0x040100	/* Audio Int interrupt mask */;
pub const AUD_E_INT_STAT: c_uint = 0x040104	/* Audio Int interrupt status */;
pub const AUD_E_INT_MSTAT: c_uint = 0x040108	/* Audio Int interrupt masked status */;
pub const AUD_E_INT_SSTAT: c_uint = 0x04010C	/* Audio Int interrupt set status */;
pub const FLD_AUD_SRC_OPC_ERR: c_uint = 0x00020000;
pub const FLD_AUD_DST_OPC_ERR: c_uint = 0x00010000;
pub const FLD_AUD_SRC_SYNC: c_uint = 0x00002000;
pub const FLD_AUD_DST_SYNC: c_uint = 0x00001000;
pub const FLD_AUD_SRC_OF: c_uint = 0x00000200;
pub const FLD_AUD_DST_OF: c_uint = 0x00000100;
pub const FLD_AUD_SRC_RISCI2: c_uint = 0x00000020;
pub const FLD_AUD_DST_RISCI2: c_uint = 0x00000010;
pub const FLD_AUD_SRC_RISCI1: c_uint = 0x00000002;
pub const FLD_AUD_DST_RISCI1: c_uint = 0x00000001;
// *****************************************************************************
pub const MBIF_A_INT_MSK: c_uint = 0x040110	/* MBIF Int interrupt mask */;
pub const MBIF_A_INT_STAT: c_uint = 0x040114	/* MBIF Int interrupt status */;
pub const MBIF_A_INT_MSTAT: c_uint = 0x040118	/* MBIF Int interrupt masked status */;
pub const MBIF_A_INT_SSTAT: c_uint = 0x04011C	/* MBIF Int interrupt set status */;
// *****************************************************************************
pub const MBIF_B_INT_MSK: c_uint = 0x040120	/* MBIF Int interrupt mask */;
pub const MBIF_B_INT_STAT: c_uint = 0x040124	/* MBIF Int interrupt status */;
pub const MBIF_B_INT_MSTAT: c_uint = 0x040128	/* MBIF Int interrupt masked status */;
pub const MBIF_B_INT_SSTAT: c_uint = 0x04012C	/* MBIF Int interrupt set status */;
pub const FLD_MBIF_DST_OPC_ERR: c_uint = 0x00010000;
pub const FLD_MBIF_DST_SYNC: c_uint = 0x00001000;
pub const FLD_MBIF_DST_OF: c_uint = 0x00000100;
pub const FLD_MBIF_DST_RISCI2: c_uint = 0x00000010;
pub const FLD_MBIF_DST_RISCI1: c_uint = 0x00000001;
// *****************************************************************************
pub const AUD_EXT_INT_MSK: c_uint = 0x040060	/* Audio Ext interrupt mask */;
pub const AUD_EXT_INT_STAT: c_uint = 0x040064	/* Audio Ext interrupt status */;
pub const AUD_EXT_INT_MSTAT: c_uint = 0x040068	/* Audio Ext interrupt masked status */;
pub const AUD_EXT_INT_SSTAT: c_uint = 0x04006C	/* Audio Ext interrupt set status */;
pub const FLD_AUD_EXT_OPC_ERR: c_uint = 0x00010000;
pub const FLD_AUD_EXT_SYNC: c_uint = 0x00001000;
pub const FLD_AUD_EXT_OF: c_uint = 0x00000100;
pub const FLD_AUD_EXT_RISCI2: c_uint = 0x00000010;
pub const FLD_AUD_EXT_RISCI1: c_uint = 0x00000001;
// *****************************************************************************
pub const GPIO_LO: c_uint = 0x110010	/* Lower  of GPIO pins [31:0] */;
pub const GPIO_HI: c_uint = 0x110014	/* Upper WORD  of GPIO pins [47:31] */;
pub const GPIO_LO_OE: c_uint = 0x110018	/* Lower  of GPIO output enable [31:0] */;
pub const GPIO_HI_OE: c_uint = 0x11001C	/* Upper word  of GPIO output enable [47:32] */;
pub const GPIO_LO_INT_MSK: c_uint = 0x11003C	/* GPIO interrupt mask */;
pub const GPIO_LO_INT_STAT: c_uint = 0x110044	/* GPIO interrupt status */;
pub const GPIO_LO_INT_MSTAT: c_uint = 0x11004C	/* GPIO interrupt masked status */;
pub const GPIO_LO_ISM_SNS: c_uint = 0x110054	/* GPIO interrupt sensitivity */;
pub const GPIO_LO_ISM_POL: c_uint = 0x11005C	/* GPIO interrupt polarity */;
pub const GPIO_HI_INT_MSK: c_uint = 0x110040	/* GPIO interrupt mask */;
pub const GPIO_HI_INT_STAT: c_uint = 0x110048	/* GPIO interrupt status */;
pub const GPIO_HI_INT_MSTAT: c_uint = 0x110050	/* GPIO interrupt masked status */;
pub const GPIO_HI_ISM_SNS: c_uint = 0x110058	/* GPIO interrupt sensitivity */;
pub const GPIO_HI_ISM_POL: c_uint = 0x110060	/* GPIO interrupt polarity */;

// *****************************************************************************
pub const TC_REQ: c_uint = 0x040090	/* Rider PCI Express traFFic class request */;
// *****************************************************************************
pub const TC_REQ_SET: c_uint = 0x040094	/* Rider PCI Express traFFic class request set */;
// *****************************************************************************
// Rider
// *****************************************************************************
// PCI Compatible Header
// *****************************************************************************
pub const RDR_CFG0: c_uint = 0x050000;
pub const RDR_VENDOR_DEVICE_ID_CFG: c_uint = 0x050000;
// *****************************************************************************
pub const RDR_CFG1: c_uint = 0x050004;
// *****************************************************************************
pub const RDR_CFG2: c_uint = 0x050008;
// *****************************************************************************
pub const RDR_CFG3: c_uint = 0x05000C;
// *****************************************************************************
pub const RDR_CFG4: c_uint = 0x050010;
// *****************************************************************************
pub const RDR_CFG5: c_uint = 0x050014;
// *****************************************************************************
pub const RDR_CFG6: c_uint = 0x050018;
// *****************************************************************************
pub const RDR_CFG7: c_uint = 0x05001C;
// *****************************************************************************
pub const RDR_CFG8: c_uint = 0x050020;
// *****************************************************************************
pub const RDR_CFG9: c_uint = 0x050024;
// *****************************************************************************
pub const RDR_CFGA: c_uint = 0x050028;
// *****************************************************************************
pub const RDR_CFGB: c_uint = 0x05002C;
pub const RDR_SUSSYSTEM_ID_CFG: c_uint = 0x05002C;
// *****************************************************************************
pub const RDR_CFGC: c_uint = 0x050030;
// *****************************************************************************
pub const RDR_CFGD: c_uint = 0x050034;
// *****************************************************************************
pub const RDR_CFGE: c_uint = 0x050038;
// *****************************************************************************
pub const RDR_CFGF: c_uint = 0x05003C;
// *****************************************************************************
// PCI-Express Capabilities
// *****************************************************************************
pub const RDR_PECAP: c_uint = 0x050040;
// *****************************************************************************
pub const RDR_PEDEVCAP: c_uint = 0x050044;
// *****************************************************************************
pub const RDR_PEDEVSC: c_uint = 0x050048;
// *****************************************************************************
pub const RDR_PELINKCAP: c_uint = 0x05004C;
// *****************************************************************************
pub const RDR_PELINKSC: c_uint = 0x050050;
// *****************************************************************************
pub const RDR_PMICAP: c_uint = 0x050080;
// *****************************************************************************
pub const RDR_PMCSR: c_uint = 0x050084;
// *****************************************************************************
pub const RDR_VPDCAP: c_uint = 0x050090;
// *****************************************************************************
pub const RDR_VPDDATA: c_uint = 0x050094;
// *****************************************************************************
pub const RDR_MSICAP: c_uint = 0x0500A0;
// *****************************************************************************
pub const RDR_MSIARL: c_uint = 0x0500A4;
// *****************************************************************************
pub const RDR_MSIARU: c_uint = 0x0500A8;
// *****************************************************************************
pub const RDR_MSIDATA: c_uint = 0x0500AC;
// *****************************************************************************
// PCI Express Extended Capabilities
// *****************************************************************************
pub const RDR_AERXCAP: c_uint = 0x050100;
// *****************************************************************************
pub const RDR_AERUESTA: c_uint = 0x050104;
// *****************************************************************************
pub const RDR_AERUEMSK: c_uint = 0x050108;
// *****************************************************************************
pub const RDR_AERUESEV: c_uint = 0x05010C;
// *****************************************************************************
pub const RDR_AERCESTA: c_uint = 0x050110;
// *****************************************************************************
pub const RDR_AERCEMSK: c_uint = 0x050114;
// *****************************************************************************
pub const RDR_AERCC: c_uint = 0x050118;
// *****************************************************************************
pub const RDR_AERHL0: c_uint = 0x05011C;
// *****************************************************************************
pub const RDR_AERHL1: c_uint = 0x050120;
// *****************************************************************************
pub const RDR_AERHL2: c_uint = 0x050124;
// *****************************************************************************
pub const RDR_AERHL3: c_uint = 0x050128;
// *****************************************************************************
pub const RDR_VCXCAP: c_uint = 0x050200;
// *****************************************************************************
pub const RDR_VCCAP1: c_uint = 0x050204;
// *****************************************************************************
pub const RDR_VCCAP2: c_uint = 0x050208;
// *****************************************************************************
pub const RDR_VCSC: c_uint = 0x05020C;
// *****************************************************************************
pub const RDR_VCR0_CAP: c_uint = 0x050210;
// *****************************************************************************
pub const RDR_VCR0_CTRL: c_uint = 0x050214;
// *****************************************************************************
pub const RDR_VCR0_STAT: c_uint = 0x050218;
// *****************************************************************************
pub const RDR_VCR1_CAP: c_uint = 0x05021C;
// *****************************************************************************
pub const RDR_VCR1_CTRL: c_uint = 0x050220;
// *****************************************************************************
pub const RDR_VCR1_STAT: c_uint = 0x050224;
// *****************************************************************************
pub const RDR_VCR2_CAP: c_uint = 0x050228;
// *****************************************************************************
pub const RDR_VCR2_CTRL: c_uint = 0x05022C;
// *****************************************************************************
pub const RDR_VCR2_STAT: c_uint = 0x050230;
// *****************************************************************************
pub const RDR_VCR3_CAP: c_uint = 0x050234;
// *****************************************************************************
pub const RDR_VCR3_CTRL: c_uint = 0x050238;
// *****************************************************************************
pub const RDR_VCR3_STAT: c_uint = 0x05023C;
// *****************************************************************************
pub const RDR_VCARB0: c_uint = 0x050240;
// *****************************************************************************
pub const RDR_VCARB1: c_uint = 0x050244;
// *****************************************************************************
pub const RDR_VCARB2: c_uint = 0x050248;
// *****************************************************************************
pub const RDR_VCARB3: c_uint = 0x05024C;
// *****************************************************************************
pub const RDR_VCARB4: c_uint = 0x050250;
// *****************************************************************************
pub const RDR_VCARB5: c_uint = 0x050254;
// *****************************************************************************
pub const RDR_VCARB6: c_uint = 0x050258;
// *****************************************************************************
pub const RDR_VCARB7: c_uint = 0x05025C;
// *****************************************************************************
pub const RDR_RDRSTAT0: c_uint = 0x050300;
// *****************************************************************************
pub const RDR_RDRSTAT1: c_uint = 0x050304;
// *****************************************************************************
pub const RDR_RDRCTL0: c_uint = 0x050308;
// *****************************************************************************
pub const RDR_RDRCTL1: c_uint = 0x05030C;
// *****************************************************************************
// Transaction Layer Registers
// *****************************************************************************
pub const RDR_TLSTAT0: c_uint = 0x050310;
// *****************************************************************************
pub const RDR_TLSTAT1: c_uint = 0x050314;
// *****************************************************************************
pub const RDR_TLCTL0: c_uint = 0x050318;
pub const FLD_CFG_UR_CPL_MODE: c_uint = 0x00000040;
pub const FLD_CFG_CORR_ERR_QUITE: c_uint = 0x00000020;
pub const FLD_CFG_RCB_CK_EN: c_uint = 0x00000010;
pub const FLD_CFG_BNDRY_CK_EN: c_uint = 0x00000008;
pub const FLD_CFG_BYTE_EN_CK_EN: c_uint = 0x00000004;
pub const FLD_CFG_RELAX_ORDER_MSK: c_uint = 0x00000002;
pub const FLD_CFG_TAG_ORDER_EN: c_uint = 0x00000001;
// *****************************************************************************
pub const RDR_TLCTL1: c_uint = 0x05031C;
// *****************************************************************************
pub const RDR_REQRCAL: c_uint = 0x050320;
// *****************************************************************************
pub const RDR_REQRCAU: c_uint = 0x050324;
// *****************************************************************************
pub const RDR_REQEPA: c_uint = 0x050328;
// *****************************************************************************
pub const RDR_REQCTRL: c_uint = 0x05032C;
// *****************************************************************************
pub const RDR_REQSTAT: c_uint = 0x050330;
// *****************************************************************************
pub const RDR_TL_TEST: c_uint = 0x050334;
// *****************************************************************************
pub const RDR_VCR01_CTL: c_uint = 0x050348;
// *****************************************************************************
pub const RDR_VCR23_CTL: c_uint = 0x05034C;
// *****************************************************************************
pub const RDR_RX_VCR0_FC: c_uint = 0x050350;
// *****************************************************************************
pub const RDR_RX_VCR1_FC: c_uint = 0x050354;
// *****************************************************************************
pub const RDR_RX_VCR2_FC: c_uint = 0x050358;
// *****************************************************************************
pub const RDR_RX_VCR3_FC: c_uint = 0x05035C;
// *****************************************************************************
// Data Link Layer Registers
// *****************************************************************************
pub const RDR_DLLSTAT: c_uint = 0x050360;
// *****************************************************************************
pub const RDR_DLLCTRL: c_uint = 0x050364;
// *****************************************************************************
pub const RDR_REPLAYTO: c_uint = 0x050368;
// *****************************************************************************
pub const RDR_ACKLATTO: c_uint = 0x05036C;
// *****************************************************************************
// MAC Layer Registers
// *****************************************************************************
pub const RDR_MACSTAT0: c_uint = 0x050380;
// *****************************************************************************
pub const RDR_MACSTAT1: c_uint = 0x050384;
// *****************************************************************************
pub const RDR_MACCTRL0: c_uint = 0x050388;
// *****************************************************************************
pub const RDR_MACCTRL1: c_uint = 0x05038C;
// *****************************************************************************
pub const RDR_MACCTRL2: c_uint = 0x050390;
// *****************************************************************************
pub const RDR_MAC_LB_DATA: c_uint = 0x050394;
// *****************************************************************************
pub const RDR_L0S_EXIT_LAT: c_uint = 0x050398;
// *****************************************************************************
// DMAC
// *****************************************************************************
pub const DMA1_PTR1: c_uint = 0x100000	/* DMA Current Ptr : Ch#1 */;
// *****************************************************************************
pub const DMA2_PTR1: c_uint = 0x100004	/* DMA Current Ptr : Ch#2 */;
// *****************************************************************************
pub const DMA3_PTR1: c_uint = 0x100008	/* DMA Current Ptr : Ch#3 */;
// *****************************************************************************
pub const DMA4_PTR1: c_uint = 0x10000C	/* DMA Current Ptr : Ch#4 */;
// *****************************************************************************
pub const DMA5_PTR1: c_uint = 0x100010	/* DMA Current Ptr : Ch#5 */;
// *****************************************************************************
pub const DMA6_PTR1: c_uint = 0x100014	/* DMA Current Ptr : Ch#6 */;
// *****************************************************************************
pub const DMA7_PTR1: c_uint = 0x100018	/* DMA Current Ptr : Ch#7 */;
// *****************************************************************************
pub const DMA8_PTR1: c_uint = 0x10001C	/* DMA Current Ptr : Ch#8 */;
// *****************************************************************************
pub const DMA9_PTR1: c_uint = 0x100020	/* DMA Current Ptr : Ch#9 */;
// *****************************************************************************
pub const DMA10_PTR1: c_uint = 0x100024	/* DMA Current Ptr : Ch#10 */;
// *****************************************************************************
pub const DMA11_PTR1: c_uint = 0x100028	/* DMA Current Ptr : Ch#11 */;
// *****************************************************************************
pub const DMA12_PTR1: c_uint = 0x10002C	/* DMA Current Ptr : Ch#12 */;
// *****************************************************************************
pub const DMA13_PTR1: c_uint = 0x100030	/* DMA Current Ptr : Ch#13 */;
// *****************************************************************************
pub const DMA14_PTR1: c_uint = 0x100034	/* DMA Current Ptr : Ch#14 */;
// *****************************************************************************
pub const DMA15_PTR1: c_uint = 0x100038	/* DMA Current Ptr : Ch#15 */;
// *****************************************************************************
pub const DMA16_PTR1: c_uint = 0x10003C	/* DMA Current Ptr : Ch#16 */;
// *****************************************************************************
pub const DMA17_PTR1: c_uint = 0x100040	/* DMA Current Ptr : Ch#17 */;
// *****************************************************************************
pub const DMA18_PTR1: c_uint = 0x100044	/* DMA Current Ptr : Ch#18 */;
// *****************************************************************************
pub const DMA19_PTR1: c_uint = 0x100048	/* DMA Current Ptr : Ch#19 */;
// *****************************************************************************
pub const DMA20_PTR1: c_uint = 0x10004C	/* DMA Current Ptr : Ch#20 */;
// *****************************************************************************
pub const DMA21_PTR1: c_uint = 0x100050	/* DMA Current Ptr : Ch#21 */;
// *****************************************************************************
pub const DMA22_PTR1: c_uint = 0x100054	/* DMA Current Ptr : Ch#22 */;
// *****************************************************************************
pub const DMA23_PTR1: c_uint = 0x100058	/* DMA Current Ptr : Ch#23 */;
// *****************************************************************************
pub const DMA24_PTR1: c_uint = 0x10005C	/* DMA Current Ptr : Ch#24 */;
// *****************************************************************************
pub const DMA25_PTR1: c_uint = 0x100060	/* DMA Current Ptr : Ch#25 */;
// *****************************************************************************
pub const DMA26_PTR1: c_uint = 0x100064	/* DMA Current Ptr : Ch#26 */;
// *****************************************************************************
pub const DMA1_PTR2: c_uint = 0x100080	/* DMA Tab Ptr : Ch#1 */;
// *****************************************************************************
pub const DMA2_PTR2: c_uint = 0x100084	/* DMA Tab Ptr : Ch#2 */;
// *****************************************************************************
pub const DMA3_PTR2: c_uint = 0x100088	/* DMA Tab Ptr : Ch#3 */;
// *****************************************************************************
pub const DMA4_PTR2: c_uint = 0x10008C	/* DMA Tab Ptr : Ch#4 */;
// *****************************************************************************
pub const DMA5_PTR2: c_uint = 0x100090	/* DMA Tab Ptr : Ch#5 */;
// *****************************************************************************
pub const DMA6_PTR2: c_uint = 0x100094	/* DMA Tab Ptr : Ch#6 */;
// *****************************************************************************
pub const DMA7_PTR2: c_uint = 0x100098	/* DMA Tab Ptr : Ch#7 */;
// *****************************************************************************
pub const DMA8_PTR2: c_uint = 0x10009C	/* DMA Tab Ptr : Ch#8 */;
// *****************************************************************************
pub const DMA9_PTR2: c_uint = 0x1000A0	/* DMA Tab Ptr : Ch#9 */;
// *****************************************************************************
pub const DMA10_PTR2: c_uint = 0x1000A4	/* DMA Tab Ptr : Ch#10 */;
// *****************************************************************************
pub const DMA11_PTR2: c_uint = 0x1000A8	/* DMA Tab Ptr : Ch#11 */;
// *****************************************************************************
pub const DMA12_PTR2: c_uint = 0x1000AC	/* DMA Tab Ptr : Ch#12 */;
// *****************************************************************************
pub const DMA13_PTR2: c_uint = 0x1000B0	/* DMA Tab Ptr : Ch#13 */;
// *****************************************************************************
pub const DMA14_PTR2: c_uint = 0x1000B4	/* DMA Tab Ptr : Ch#14 */;
// *****************************************************************************
pub const DMA15_PTR2: c_uint = 0x1000B8	/* DMA Tab Ptr : Ch#15 */;
// *****************************************************************************
pub const DMA16_PTR2: c_uint = 0x1000BC	/* DMA Tab Ptr : Ch#16 */;
// *****************************************************************************
pub const DMA17_PTR2: c_uint = 0x1000C0	/* DMA Tab Ptr : Ch#17 */;
// *****************************************************************************
pub const DMA18_PTR2: c_uint = 0x1000C4	/* DMA Tab Ptr : Ch#18 */;
// *****************************************************************************
pub const DMA19_PTR2: c_uint = 0x1000C8	/* DMA Tab Ptr : Ch#19 */;
// *****************************************************************************
pub const DMA20_PTR2: c_uint = 0x1000CC	/* DMA Tab Ptr : Ch#20 */;
// *****************************************************************************
pub const DMA21_PTR2: c_uint = 0x1000D0	/* DMA Tab Ptr : Ch#21 */;
// *****************************************************************************
pub const DMA22_PTR2: c_uint = 0x1000D4	/* DMA Tab Ptr : Ch#22 */;
// *****************************************************************************
pub const DMA23_PTR2: c_uint = 0x1000D8	/* DMA Tab Ptr : Ch#23 */;
// *****************************************************************************
pub const DMA24_PTR2: c_uint = 0x1000DC	/* DMA Tab Ptr : Ch#24 */;
// *****************************************************************************
pub const DMA25_PTR2: c_uint = 0x1000E0	/* DMA Tab Ptr : Ch#25 */;
// *****************************************************************************
pub const DMA26_PTR2: c_uint = 0x1000E4	/* DMA Tab Ptr : Ch#26 */;
// *****************************************************************************
pub const DMA1_CNT1: c_uint = 0x100100	/* DMA BuFFer Size : Ch#1 */;
// *****************************************************************************
pub const DMA2_CNT1: c_uint = 0x100104	/* DMA BuFFer Size : Ch#2 */;
// *****************************************************************************
pub const DMA3_CNT1: c_uint = 0x100108	/* DMA BuFFer Size : Ch#3 */;
// *****************************************************************************
pub const DMA4_CNT1: c_uint = 0x10010C	/* DMA BuFFer Size : Ch#4 */;
// *****************************************************************************
pub const DMA5_CNT1: c_uint = 0x100110	/* DMA BuFFer Size : Ch#5 */;
// *****************************************************************************
pub const DMA6_CNT1: c_uint = 0x100114	/* DMA BuFFer Size : Ch#6 */;
// *****************************************************************************
pub const DMA7_CNT1: c_uint = 0x100118	/* DMA BuFFer Size : Ch#7 */;
// *****************************************************************************
pub const DMA8_CNT1: c_uint = 0x10011C	/* DMA BuFFer Size : Ch#8 */;
// *****************************************************************************
pub const DMA9_CNT1: c_uint = 0x100120	/* DMA BuFFer Size : Ch#9 */;
// *****************************************************************************
pub const DMA10_CNT1: c_uint = 0x100124	/* DMA BuFFer Size : Ch#10 */;
// *****************************************************************************
pub const DMA11_CNT1: c_uint = 0x100128	/* DMA BuFFer Size : Ch#11 */;
// *****************************************************************************
pub const DMA12_CNT1: c_uint = 0x10012C	/* DMA BuFFer Size : Ch#12 */;
// *****************************************************************************
pub const DMA13_CNT1: c_uint = 0x100130	/* DMA BuFFer Size : Ch#13 */;
// *****************************************************************************
pub const DMA14_CNT1: c_uint = 0x100134	/* DMA BuFFer Size : Ch#14 */;
// *****************************************************************************
pub const DMA15_CNT1: c_uint = 0x100138	/* DMA BuFFer Size : Ch#15 */;
// *****************************************************************************
pub const DMA16_CNT1: c_uint = 0x10013C	/* DMA BuFFer Size : Ch#16 */;
// *****************************************************************************
pub const DMA17_CNT1: c_uint = 0x100140	/* DMA BuFFer Size : Ch#17 */;
// *****************************************************************************
pub const DMA18_CNT1: c_uint = 0x100144	/* DMA BuFFer Size : Ch#18 */;
// *****************************************************************************
pub const DMA19_CNT1: c_uint = 0x100148	/* DMA BuFFer Size : Ch#19 */;
// *****************************************************************************
pub const DMA20_CNT1: c_uint = 0x10014C	/* DMA BuFFer Size : Ch#20 */;
// *****************************************************************************
pub const DMA21_CNT1: c_uint = 0x100150	/* DMA BuFFer Size : Ch#21 */;
// *****************************************************************************
pub const DMA22_CNT1: c_uint = 0x100154	/* DMA BuFFer Size : Ch#22 */;
// *****************************************************************************
pub const DMA23_CNT1: c_uint = 0x100158	/* DMA BuFFer Size : Ch#23 */;
// *****************************************************************************
pub const DMA24_CNT1: c_uint = 0x10015C	/* DMA BuFFer Size : Ch#24 */;
// *****************************************************************************
pub const DMA25_CNT1: c_uint = 0x100160	/* DMA BuFFer Size : Ch#25 */;
// *****************************************************************************
pub const DMA26_CNT1: c_uint = 0x100164	/* DMA BuFFer Size : Ch#26 */;
// *****************************************************************************
pub const DMA1_CNT2: c_uint = 0x100180	/* DMA Table Size : Ch#1 */;
// *****************************************************************************
pub const DMA2_CNT2: c_uint = 0x100184	/* DMA Table Size : Ch#2 */;
// *****************************************************************************
pub const DMA3_CNT2: c_uint = 0x100188	/* DMA Table Size : Ch#3 */;
// *****************************************************************************
pub const DMA4_CNT2: c_uint = 0x10018C	/* DMA Table Size : Ch#4 */;
// *****************************************************************************
pub const DMA5_CNT2: c_uint = 0x100190	/* DMA Table Size : Ch#5 */;
// *****************************************************************************
pub const DMA6_CNT2: c_uint = 0x100194	/* DMA Table Size : Ch#6 */;
// *****************************************************************************
pub const DMA7_CNT2: c_uint = 0x100198	/* DMA Table Size : Ch#7 */;
// *****************************************************************************
pub const DMA8_CNT2: c_uint = 0x10019C	/* DMA Table Size : Ch#8 */;
// *****************************************************************************
pub const DMA9_CNT2: c_uint = 0x1001A0	/* DMA Table Size : Ch#9 */;
// *****************************************************************************
pub const DMA10_CNT2: c_uint = 0x1001A4	/* DMA Table Size : Ch#10 */;
// *****************************************************************************
pub const DMA11_CNT2: c_uint = 0x1001A8	/* DMA Table Size : Ch#11 */;
// *****************************************************************************
pub const DMA12_CNT2: c_uint = 0x1001AC	/* DMA Table Size : Ch#12 */;
// *****************************************************************************
pub const DMA13_CNT2: c_uint = 0x1001B0	/* DMA Table Size : Ch#13 */;
// *****************************************************************************
pub const DMA14_CNT2: c_uint = 0x1001B4	/* DMA Table Size : Ch#14 */;
// *****************************************************************************
pub const DMA15_CNT2: c_uint = 0x1001B8	/* DMA Table Size : Ch#15 */;
// *****************************************************************************
pub const DMA16_CNT2: c_uint = 0x1001BC	/* DMA Table Size : Ch#16 */;
// *****************************************************************************
pub const DMA17_CNT2: c_uint = 0x1001C0	/* DMA Table Size : Ch#17 */;
// *****************************************************************************
pub const DMA18_CNT2: c_uint = 0x1001C4	/* DMA Table Size : Ch#18 */;
// *****************************************************************************
pub const DMA19_CNT2: c_uint = 0x1001C8	/* DMA Table Size : Ch#19 */;
// *****************************************************************************
pub const DMA20_CNT2: c_uint = 0x1001CC	/* DMA Table Size : Ch#20 */;
// *****************************************************************************
pub const DMA21_CNT2: c_uint = 0x1001D0	/* DMA Table Size : Ch#21 */;
// *****************************************************************************
pub const DMA22_CNT2: c_uint = 0x1001D4	/* DMA Table Size : Ch#22 */;
// *****************************************************************************
pub const DMA23_CNT2: c_uint = 0x1001D8	/* DMA Table Size : Ch#23 */;
// *****************************************************************************
pub const DMA24_CNT2: c_uint = 0x1001DC	/* DMA Table Size : Ch#24 */;
// *****************************************************************************
pub const DMA25_CNT2: c_uint = 0x1001E0	/* DMA Table Size : Ch#25 */;
// *****************************************************************************
pub const DMA26_CNT2: c_uint = 0x1001E4	/* DMA Table Size : Ch#26 */;
// *****************************************************************************
// ITG
// *****************************************************************************
pub const TM_CNT_LDW: c_uint = 0x110000	/* Timer : Counter low */;
// *****************************************************************************
pub const TM_CNT_UW: c_uint = 0x110004	/* Timer : Counter high word */;
// *****************************************************************************
pub const TM_LMT_LDW: c_uint = 0x110008	/* Timer : Limit low */;
// *****************************************************************************
pub const TM_LMT_UW: c_uint = 0x11000C	/* Timer : Limit high word */;
// *****************************************************************************
pub const GP0_IO: c_uint = 0x110010	/* GPIO output enables data I/O */;
pub const FLD_GP_OE: c_uint = 0x00FF0000	/* GPIO: GP_OE output enable */;
pub const FLD_GP_IN: c_uint = 0x0000FF00	/* GPIO: GP_IN status */;
pub const FLD_GP_OUT: c_uint = 0x000000FF	/* GPIO: GP_OUT control */;
// *****************************************************************************
pub const GPIO_ISM: c_uint = 0x110014	/* GPIO interrupt sensitivity mode */;
pub const FLD_GP_ISM_SNS: c_uint = 0x00000070;
pub const FLD_GP_ISM_POL: c_uint = 0x00000007;
// *****************************************************************************
pub const SOFT_RESET: c_uint = 0x11001C	/* Output system reset reg */;
pub const FLD_PECOS_SOFT_RESET: c_uint = 0x00000001;
// *****************************************************************************
pub const MC416_RWD: c_uint = 0x110020	/* MC416 GPIO[18:3] pin */;
pub const MC416_OEN: c_uint = 0x110024	/* Output enable of GPIO[18:3] */;
pub const MC416_CTL: c_uint = 0x110028;
// *****************************************************************************
pub const ALT_PIN_OUT_SEL: c_uint = 0x11002C	/* Alternate GPIO output select */;
pub const FLD_ALT_GPIO_OUT_SEL: c_uint = 0xF0000000;
// 0          Disabled <-- default
// 1          GPIO[0]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
// 8          ATT_IF
pub const FLD_AUX_PLL_CLK_ALT_SEL: c_uint = 0x0F000000;
// 0          AUX_PLL_CLK<-- default
// 1          GPIO[2]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const FLD_IR_TX_ALT_SEL: c_uint = 0x00F00000;
// 0          IR_TX <-- default
// 1          GPIO[1]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const FLD_IR_RX_ALT_SEL: c_uint = 0x000F0000;
// 0          IR_RX <-- default
// 1          GPIO[0]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const FLD_GPIO10_ALT_SEL: c_uint = 0x0000F000;
// 0          GPIO[10] <-- default
// 1          GPIO[0]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const FLD_GPIO2_ALT_SEL: c_uint = 0x00000F00;
// 0          GPIO[2] <-- default
// 1          GPIO[1]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const FLD_GPIO1_ALT_SEL: c_uint = 0x000000F0;
// 0          GPIO[1] <-- default
// 1          GPIO[0]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const FLD_GPIO0_ALT_SEL: c_uint = 0x0000000F;
// 0          GPIO[0] <-- default
// 1          GPIO[1]
// 2          GPIO[10]
// 3          VIP_656_DATA_VAL
// 4          VIP_656_DATA[0]
// 5          VIP_656_CLK
// 6          VIP_656_DATA_EXT[1]
// 7          VIP_656_DATA_EXT[0]
pub const ALT_PIN_IN_SEL: c_uint = 0x110030	/* Alternate GPIO input select */;
pub const FLD_GPIO10_ALT_IN_SEL: c_uint = 0x0000F000;
// 0          GPIO[10] <-- default
// 1          IR_RX
// 2          IR_TX
// 3          AUX_PLL_CLK
// 4          IF_ATT_SEL
// 5          GPIO[0]
// 6          GPIO[1]
// 7          GPIO[2]
pub const FLD_GPIO2_ALT_IN_SEL: c_uint = 0x00000F00;
// 0          GPIO[2] <-- default
// 1          IR_RX
// 2          IR_TX
// 3          AUX_PLL_CLK
// 4          IF_ATT_SEL
pub const FLD_GPIO1_ALT_IN_SEL: c_uint = 0x000000F0;
// 0          GPIO[1] <-- default
// 1          IR_RX
// 2          IR_TX
// 3          AUX_PLL_CLK
// 4          IF_ATT_SEL
pub const FLD_GPIO0_ALT_IN_SEL: c_uint = 0x0000000F;
// 0          GPIO[0] <-- default
// 1          IR_RX
// 2          IR_TX
// 3          AUX_PLL_CLK
// 4          IF_ATT_SEL
// *****************************************************************************
pub const TEST_BUS_CTL1: c_uint = 0x110040	/* Test bus control register #1 */;
// *****************************************************************************
pub const TEST_BUS_CTL2: c_uint = 0x110044	/* Test bus control register #2 */;
// *****************************************************************************
pub const CLK_DELAY: c_uint = 0x110048	/* Clock delay */;
pub const FLD_MOE_CLK_DIS: c_uint = 0x80000000	/* Disable MoE clock */;
// *****************************************************************************
pub const PAD_CTRL: c_uint = 0x110068	/* Pad drive strength control */;
// *****************************************************************************
pub const MBIST_CTRL: c_uint = 0x110050	/* SRAM memory built-in self test control */;
// *****************************************************************************
pub const MBIST_STAT: c_uint = 0x110054	/* SRAM memory built-in self test status */;
// *****************************************************************************
// PLL registers
// *****************************************************************************
pub const PLL_A_INT_FRAC: c_uint = 0x110088;
pub const PLL_A_POST_STAT_BIST: c_uint = 0x11008C;
pub const PLL_B_INT_FRAC: c_uint = 0x110090;
pub const PLL_B_POST_STAT_BIST: c_uint = 0x110094;
pub const PLL_C_INT_FRAC: c_uint = 0x110098;
pub const PLL_C_POST_STAT_BIST: c_uint = 0x11009C;
pub const PLL_D_INT_FRAC: c_uint = 0x1100A0;
pub const PLL_D_POST_STAT_BIST: c_uint = 0x1100A4;
pub const CLK_RST: c_uint = 0x11002C;
pub const FLD_VID_I_CLK_NOE: c_uint = 0x00001000;
pub const FLD_VID_J_CLK_NOE: c_uint = 0x00002000;
pub const FLD_USE_ALT_PLL_REF: c_uint = 0x00004000;
pub const VID_CH_MODE_SEL: c_uint = 0x110078;
pub const VID_CH_CLK_SEL: c_uint = 0x11007C;
// *****************************************************************************
pub const VBI_A_DMA: c_uint = 0x130008	/* VBI A DMA data port */;
// *****************************************************************************
pub const VID_A_VIP_CTL: c_uint = 0x130080	/* Video A VIP format control */;
pub const FLD_VIP_MODE: c_uint = 0x00000001;
// *****************************************************************************
pub const VID_A_PIXEL_FRMT: c_uint = 0x130084	/* Video A pixel format */;
pub const FLD_VID_A_GAMMA_DIS: c_uint = 0x00000008;
pub const FLD_VID_A_FORMAT: c_uint = 0x00000007;
pub const FLD_VID_A_GAMMA_FACTOR: c_uint = 0x00000010;
// *****************************************************************************
pub const VID_A_VBI_CTL: c_uint = 0x130088	/* Video A VBI miscellaneous control */;
pub const FLD_VID_A_VIP_EXT: c_uint = 0x00000003;
// *****************************************************************************
pub const VID_B_DMA: c_uint = 0x130100	/* Video B DMA data port */;
// *****************************************************************************
pub const VBI_B_DMA: c_uint = 0x130108	/* VBI B DMA data port */;
// *****************************************************************************
pub const VID_B_SRC_SEL: c_uint = 0x130144	/* Video B source select */;
pub const FLD_VID_B_SRC_SEL: c_uint = 0x00000000;
// *****************************************************************************
pub const VID_B_LNGTH: c_uint = 0x130150	/* Video B line length */;
pub const FLD_VID_B_LN_LNGTH: c_uint = 0x00000FFF;
// *****************************************************************************
pub const VID_B_VIP_CTL: c_uint = 0x130180	/* Video B VIP format control */;
// *****************************************************************************
pub const VID_B_PIXEL_FRMT: c_uint = 0x130184	/* Video B pixel format */;
pub const FLD_VID_B_GAMMA_DIS: c_uint = 0x00000008;
pub const FLD_VID_B_FORMAT: c_uint = 0x00000007;
pub const FLD_VID_B_GAMMA_FACTOR: c_uint = 0x00000010;
// *****************************************************************************
pub const VID_C_DMA: c_uint = 0x130200	/* Video C DMA data port */;
// *****************************************************************************
pub const VID_C_LNGTH: c_uint = 0x130250	/* Video C line length */;
pub const FLD_VID_C_LN_LNGTH: c_uint = 0x00000FFF;
// *****************************************************************************
// Video Destination Channels
// *****************************************************************************
pub const VID_DST_A_GPCNT: c_uint = 0x130020	/* Video A general purpose counter */;
pub const VID_DST_B_GPCNT: c_uint = 0x130120	/* Video B general purpose counter */;
pub const VID_DST_C_GPCNT: c_uint = 0x130220	/* Video C general purpose counter */;
pub const VID_DST_D_GPCNT: c_uint = 0x130320	/* Video D general purpose counter */;
pub const VID_DST_E_GPCNT: c_uint = 0x130420	/* Video E general purpose counter */;
pub const VID_DST_F_GPCNT: c_uint = 0x130520	/* Video F general purpose counter */;
pub const VID_DST_G_GPCNT: c_uint = 0x130620	/* Video G general purpose counter */;
pub const VID_DST_H_GPCNT: c_uint = 0x130720	/* Video H general purpose counter */;
// *****************************************************************************
pub const VID_DST_A_GPCNT_CTL: c_uint = 0x130030	/* Video A general purpose control */;
pub const VID_DST_B_GPCNT_CTL: c_uint = 0x130130	/* Video B general purpose control */;
pub const VID_DST_C_GPCNT_CTL: c_uint = 0x130230	/* Video C general purpose control */;
pub const VID_DST_D_GPCNT_CTL: c_uint = 0x130330	/* Video D general purpose control */;
pub const VID_DST_E_GPCNT_CTL: c_uint = 0x130430	/* Video E general purpose control */;
pub const VID_DST_F_GPCNT_CTL: c_uint = 0x130530	/* Video F general purpose control */;
pub const VID_DST_G_GPCNT_CTL: c_uint = 0x130630	/* Video G general purpose control */;
pub const VID_DST_H_GPCNT_CTL: c_uint = 0x130730	/* Video H general purpose control */;
// *****************************************************************************
pub const VID_DST_A_DMA_CTL: c_uint = 0x130040	/* Video A DMA control */;
pub const VID_DST_B_DMA_CTL: c_uint = 0x130140	/* Video B DMA control */;
pub const VID_DST_C_DMA_CTL: c_uint = 0x130240	/* Video C DMA control */;
pub const VID_DST_D_DMA_CTL: c_uint = 0x130340	/* Video D DMA control */;
pub const VID_DST_E_DMA_CTL: c_uint = 0x130440	/* Video E DMA control */;
pub const VID_DST_F_DMA_CTL: c_uint = 0x130540	/* Video F DMA control */;
pub const VID_DST_G_DMA_CTL: c_uint = 0x130640	/* Video G DMA control */;
pub const VID_DST_H_DMA_CTL: c_uint = 0x130740	/* Video H DMA control */;
pub const FLD_VID_RISC_EN: c_uint = 0x00000010;
pub const FLD_VID_FIFO_EN: c_uint = 0x00000001;
// *****************************************************************************
pub const VID_DST_A_VIP_CTL: c_uint = 0x130080	/* Video A VIP control */;
pub const VID_DST_B_VIP_CTL: c_uint = 0x130180	/* Video B VIP control */;
pub const VID_DST_C_VIP_CTL: c_uint = 0x130280	/* Video C VIP control */;
pub const VID_DST_D_VIP_CTL: c_uint = 0x130380	/* Video D VIP control */;
pub const VID_DST_E_VIP_CTL: c_uint = 0x130480	/* Video E VIP control */;
pub const VID_DST_F_VIP_CTL: c_uint = 0x130580	/* Video F VIP control */;
pub const VID_DST_G_VIP_CTL: c_uint = 0x130680	/* Video G VIP control */;
pub const VID_DST_H_VIP_CTL: c_uint = 0x130780	/* Video H VIP control */;
// *****************************************************************************
pub const VID_DST_A_PIX_FRMT: c_uint = 0x130084	/* Video A Pixel format */;
pub const VID_DST_B_PIX_FRMT: c_uint = 0x130184	/* Video B Pixel format */;
pub const VID_DST_C_PIX_FRMT: c_uint = 0x130284	/* Video C Pixel format */;
pub const VID_DST_D_PIX_FRMT: c_uint = 0x130384	/* Video D Pixel format */;
pub const VID_DST_E_PIX_FRMT: c_uint = 0x130484	/* Video E Pixel format */;
pub const VID_DST_F_PIX_FRMT: c_uint = 0x130584	/* Video F Pixel format */;
pub const VID_DST_G_PIX_FRMT: c_uint = 0x130684	/* Video G Pixel format */;
pub const VID_DST_H_PIX_FRMT: c_uint = 0x130784	/* Video H Pixel format */;
// *****************************************************************************
// Video Source Channels
// *****************************************************************************
pub const VID_SRC_A_GPCNT_CTL: c_uint = 0x130804	/* Video A general purpose control */;
pub const VID_SRC_B_GPCNT_CTL: c_uint = 0x130904	/* Video B general purpose control */;
pub const VID_SRC_C_GPCNT_CTL: c_uint = 0x130A04	/* Video C general purpose control */;
pub const VID_SRC_D_GPCNT_CTL: c_uint = 0x130B04	/* Video D general purpose control */;
pub const VID_SRC_E_GPCNT_CTL: c_uint = 0x130C04	/* Video E general purpose control */;
pub const VID_SRC_F_GPCNT_CTL: c_uint = 0x130D04	/* Video F general purpose control */;
pub const VID_SRC_I_GPCNT_CTL: c_uint = 0x130E04	/* Video I general purpose control */;
pub const VID_SRC_J_GPCNT_CTL: c_uint = 0x130F04	/* Video J general purpose control */;
// *****************************************************************************
pub const VID_SRC_A_GPCNT: c_uint = 0x130808	/* Video A general purpose counter */;
pub const VID_SRC_B_GPCNT: c_uint = 0x130908	/* Video B general purpose counter */;
pub const VID_SRC_C_GPCNT: c_uint = 0x130A08	/* Video C general purpose counter */;
pub const VID_SRC_D_GPCNT: c_uint = 0x130B08	/* Video D general purpose counter */;
pub const VID_SRC_E_GPCNT: c_uint = 0x130C08	/* Video E general purpose counter */;
pub const VID_SRC_F_GPCNT: c_uint = 0x130D08	/* Video F general purpose counter */;
pub const VID_SRC_I_GPCNT: c_uint = 0x130E08	/* Video I general purpose counter */;
pub const VID_SRC_J_GPCNT: c_uint = 0x130F08	/* Video J general purpose counter */;
// *****************************************************************************
pub const VID_SRC_A_DMA_CTL: c_uint = 0x13080C	/* Video A DMA control */;
pub const VID_SRC_B_DMA_CTL: c_uint = 0x13090C	/* Video B DMA control */;
pub const VID_SRC_C_DMA_CTL: c_uint = 0x130A0C	/* Video C DMA control */;
pub const VID_SRC_D_DMA_CTL: c_uint = 0x130B0C	/* Video D DMA control */;
pub const VID_SRC_E_DMA_CTL: c_uint = 0x130C0C	/* Video E DMA control */;
pub const VID_SRC_F_DMA_CTL: c_uint = 0x130D0C	/* Video F DMA control */;
pub const VID_SRC_I_DMA_CTL: c_uint = 0x130E0C	/* Video I DMA control */;
pub const VID_SRC_J_DMA_CTL: c_uint = 0x130F0C	/* Video J DMA control */;
pub const FLD_APB_RISC_EN: c_uint = 0x00000010;
pub const FLD_APB_FIFO_EN: c_uint = 0x00000001;
// *****************************************************************************
pub const VID_SRC_A_FMT_CTL: c_uint = 0x130810	/* Video A format control */;
pub const VID_SRC_B_FMT_CTL: c_uint = 0x130910	/* Video B format control */;
pub const VID_SRC_C_FMT_CTL: c_uint = 0x130A10	/* Video C format control */;
pub const VID_SRC_D_FMT_CTL: c_uint = 0x130B10	/* Video D format control */;
pub const VID_SRC_E_FMT_CTL: c_uint = 0x130C10	/* Video E format control */;
pub const VID_SRC_F_FMT_CTL: c_uint = 0x130D10	/* Video F format control */;
pub const VID_SRC_I_FMT_CTL: c_uint = 0x130E10	/* Video I format control */;
pub const VID_SRC_J_FMT_CTL: c_uint = 0x130F10	/* Video J format control */;
// *****************************************************************************
pub const VID_SRC_A_ACTIVE_CTL1: c_uint = 0x130814	/* Video A active control      1 */;
pub const VID_SRC_B_ACTIVE_CTL1: c_uint = 0x130914	/* Video B active control      1 */;
pub const VID_SRC_C_ACTIVE_CTL1: c_uint = 0x130A14	/* Video C active control      1 */;
pub const VID_SRC_D_ACTIVE_CTL1: c_uint = 0x130B14	/* Video D active control      1 */;
pub const VID_SRC_E_ACTIVE_CTL1: c_uint = 0x130C14	/* Video E active control      1 */;
pub const VID_SRC_F_ACTIVE_CTL1: c_uint = 0x130D14	/* Video F active control      1 */;
pub const VID_SRC_I_ACTIVE_CTL1: c_uint = 0x130E14	/* Video I active control      1 */;
pub const VID_SRC_J_ACTIVE_CTL1: c_uint = 0x130F14	/* Video J active control      1 */;
// *****************************************************************************
pub const VID_SRC_A_ACTIVE_CTL2: c_uint = 0x130818	/* Video A active control      2 */;
pub const VID_SRC_B_ACTIVE_CTL2: c_uint = 0x130918	/* Video B active control      2 */;
pub const VID_SRC_C_ACTIVE_CTL2: c_uint = 0x130A18	/* Video C active control      2 */;
pub const VID_SRC_D_ACTIVE_CTL2: c_uint = 0x130B18	/* Video D active control      2 */;
pub const VID_SRC_E_ACTIVE_CTL2: c_uint = 0x130C18	/* Video E active control      2 */;
pub const VID_SRC_F_ACTIVE_CTL2: c_uint = 0x130D18	/* Video F active control      2 */;
pub const VID_SRC_I_ACTIVE_CTL2: c_uint = 0x130E18	/* Video I active control      2 */;
pub const VID_SRC_J_ACTIVE_CTL2: c_uint = 0x130F18	/* Video J active control      2 */;
// *****************************************************************************
pub const VID_SRC_A_CDT_SZ: c_uint = 0x13081C	/* Video A CDT size */;
pub const VID_SRC_B_CDT_SZ: c_uint = 0x13091C	/* Video B CDT size */;
pub const VID_SRC_C_CDT_SZ: c_uint = 0x130A1C	/* Video C CDT size */;
pub const VID_SRC_D_CDT_SZ: c_uint = 0x130B1C	/* Video D CDT size */;
pub const VID_SRC_E_CDT_SZ: c_uint = 0x130C1C	/* Video E CDT size */;
pub const VID_SRC_F_CDT_SZ: c_uint = 0x130D1C	/* Video F CDT size */;
pub const VID_SRC_I_CDT_SZ: c_uint = 0x130E1C	/* Video I CDT size */;
pub const VID_SRC_J_CDT_SZ: c_uint = 0x130F1C	/* Video J CDT size */;
// *****************************************************************************
// Audio I/F
// *****************************************************************************
pub const AUD_DST_A_DMA: c_uint = 0x140000	/* Audio Int A DMA data port */;
pub const AUD_SRC_A_DMA: c_uint = 0x140008	/* Audio Int A DMA data port */;
pub const AUD_A_GPCNT: c_uint = 0x140010	/* Audio Int A gp counter */;
pub const FLD_AUD_A_GP_CNT: c_uint = 0x0000FFFF;
pub const AUD_A_GPCNT_CTL: c_uint = 0x140014	/* Audio Int A gp control */;
pub const AUD_A_LNGTH: c_uint = 0x140018	/* Audio Int A line length */;
pub const AUD_A_CFG: c_uint = 0x14001C	/* Audio Int A configuration */;
// *****************************************************************************
pub const AUD_DST_B_DMA: c_uint = 0x140100	/* Audio Int B DMA data port */;
pub const AUD_SRC_B_DMA: c_uint = 0x140108	/* Audio Int B DMA data port */;
pub const AUD_B_GPCNT: c_uint = 0x140110	/* Audio Int B gp counter */;
pub const FLD_AUD_B_GP_CNT: c_uint = 0x0000FFFF;
pub const AUD_B_GPCNT_CTL: c_uint = 0x140114	/* Audio Int B gp control */;
pub const AUD_B_LNGTH: c_uint = 0x140118	/* Audio Int B line length */;
pub const AUD_B_CFG: c_uint = 0x14011C	/* Audio Int B configuration */;
// *****************************************************************************
pub const AUD_DST_C_DMA: c_uint = 0x140200	/* Audio Int C DMA data port */;
pub const AUD_SRC_C_DMA: c_uint = 0x140208	/* Audio Int C DMA data port */;
pub const AUD_C_GPCNT: c_uint = 0x140210	/* Audio Int C gp counter */;
pub const FLD_AUD_C_GP_CNT: c_uint = 0x0000FFFF;
pub const AUD_C_GPCNT_CTL: c_uint = 0x140214	/* Audio Int C gp control */;
pub const AUD_C_LNGTH: c_uint = 0x140218	/* Audio Int C line length */;
pub const AUD_C_CFG: c_uint = 0x14021C	/* Audio Int C configuration */;
// *****************************************************************************
pub const AUD_DST_D_DMA: c_uint = 0x140300	/* Audio Int D DMA data port */;
pub const AUD_SRC_D_DMA: c_uint = 0x140308	/* Audio Int D DMA data port */;
pub const AUD_D_GPCNT: c_uint = 0x140310	/* Audio Int D gp counter */;
pub const FLD_AUD_D_GP_CNT: c_uint = 0x0000FFFF;
pub const AUD_D_GPCNT_CTL: c_uint = 0x140314	/* Audio Int D gp control */;
pub const AUD_D_LNGTH: c_uint = 0x140318	/* Audio Int D line length */;
pub const AUD_D_CFG: c_uint = 0x14031C	/* Audio Int D configuration */;
// *****************************************************************************
pub const AUD_SRC_E_DMA: c_uint = 0x140400	/* Audio Int E DMA data port */;
pub const AUD_E_GPCNT: c_uint = 0x140410	/* Audio Int E gp counter */;
pub const FLD_AUD_E_GP_CNT: c_uint = 0x0000FFFF;
pub const AUD_E_GPCNT_CTL: c_uint = 0x140414	/* Audio Int E gp control */;
pub const AUD_E_CFG: c_uint = 0x14041C	/* Audio Int E configuration */;
// *****************************************************************************
pub const FLD_AUD_DST_LN_LNGTH: c_uint = 0x00000FFF;
pub const FLD_AUD_DST_PK_MODE: c_uint = 0x00004000;
pub const FLD_AUD_CLK_ENABLE: c_uint = 0x00000200;
pub const FLD_AUD_MASTER_MODE: c_uint = 0x00000002;
pub const FLD_AUD_SONY_MODE: c_uint = 0x00000001;
pub const FLD_AUD_CLK_SELECT_PLL_D: c_uint = 0x00001800;
pub const FLD_AUD_DST_ENABLE: c_uint = 0x00020000;
pub const FLD_AUD_SRC_ENABLE: c_uint = 0x00010000;
// *****************************************************************************
pub const AUD_INT_DMA_CTL: c_uint = 0x140500	/* Audio Int DMA control */;
pub const FLD_AUD_SRC_E_RISC_EN: c_uint = 0x00008000;
pub const FLD_AUD_SRC_C_RISC_EN: c_uint = 0x00004000;
pub const FLD_AUD_SRC_B_RISC_EN: c_uint = 0x00002000;
pub const FLD_AUD_SRC_A_RISC_EN: c_uint = 0x00001000;
pub const FLD_AUD_DST_D_RISC_EN: c_uint = 0x00000800;
pub const FLD_AUD_DST_C_RISC_EN: c_uint = 0x00000400;
pub const FLD_AUD_DST_B_RISC_EN: c_uint = 0x00000200;
pub const FLD_AUD_DST_A_RISC_EN: c_uint = 0x00000100;
pub const FLD_AUD_SRC_E_FIFO_EN: c_uint = 0x00000080;
pub const FLD_AUD_SRC_C_FIFO_EN: c_uint = 0x00000040;
pub const FLD_AUD_SRC_B_FIFO_EN: c_uint = 0x00000020;
pub const FLD_AUD_SRC_A_FIFO_EN: c_uint = 0x00000010;
pub const FLD_AUD_DST_D_FIFO_EN: c_uint = 0x00000008;
pub const FLD_AUD_DST_C_FIFO_EN: c_uint = 0x00000004;
pub const FLD_AUD_DST_B_FIFO_EN: c_uint = 0x00000002;
pub const FLD_AUD_DST_A_FIFO_EN: c_uint = 0x00000001;
// *****************************************************************************
//
// Mobilygen Interface Registers
//
// *****************************************************************************
// Mobilygen Interface A
// *****************************************************************************
pub const MB_IF_A_DMA: c_uint = 0x150000	/* MBIF A DMA data port */;
pub const MB_IF_A_GPCN: c_uint = 0x150008	/* MBIF A GP counter */;
pub const MB_IF_A_GPCN_CTRL: c_uint = 0x15000C;
pub const MB_IF_A_DMA_CTRL: c_uint = 0x150010;
pub const MB_IF_A_LENGTH: c_uint = 0x150014;
pub const MB_IF_A_HDMA_XFER_SZ: c_uint = 0x150018;
pub const MB_IF_A_HCMD: c_uint = 0x15001C;
pub const MB_IF_A_HCONFIG: c_uint = 0x150020;
pub const MB_IF_A_DATA_STRUCT_0: c_uint = 0x150024;
pub const MB_IF_A_DATA_STRUCT_1: c_uint = 0x150028;
pub const MB_IF_A_DATA_STRUCT_2: c_uint = 0x15002C;
pub const MB_IF_A_DATA_STRUCT_3: c_uint = 0x150030;
pub const MB_IF_A_DATA_STRUCT_4: c_uint = 0x150034;
pub const MB_IF_A_DATA_STRUCT_5: c_uint = 0x150038;
pub const MB_IF_A_DATA_STRUCT_6: c_uint = 0x15003C;
pub const MB_IF_A_DATA_STRUCT_7: c_uint = 0x150040;
pub const MB_IF_A_DATA_STRUCT_8: c_uint = 0x150044;
pub const MB_IF_A_DATA_STRUCT_9: c_uint = 0x150048;
pub const MB_IF_A_DATA_STRUCT_A: c_uint = 0x15004C;
pub const MB_IF_A_DATA_STRUCT_B: c_uint = 0x150050;
pub const MB_IF_A_DATA_STRUCT_C: c_uint = 0x150054;
pub const MB_IF_A_DATA_STRUCT_D: c_uint = 0x150058;
pub const MB_IF_A_DATA_STRUCT_E: c_uint = 0x15005C;
pub const MB_IF_A_DATA_STRUCT_F: c_uint = 0x150060;
// *****************************************************************************
// Mobilygen Interface B
// *****************************************************************************
pub const MB_IF_B_DMA: c_uint = 0x160000	/* MBIF A DMA data port */;
pub const MB_IF_B_GPCN: c_uint = 0x160008	/* MBIF A GP counter */;
pub const MB_IF_B_GPCN_CTRL: c_uint = 0x16000C;
pub const MB_IF_B_DMA_CTRL: c_uint = 0x160010;
pub const MB_IF_B_LENGTH: c_uint = 0x160014;
pub const MB_IF_B_HDMA_XFER_SZ: c_uint = 0x160018;
pub const MB_IF_B_HCMD: c_uint = 0x16001C;
pub const MB_IF_B_HCONFIG: c_uint = 0x160020;
pub const MB_IF_B_DATA_STRUCT_0: c_uint = 0x160024;
pub const MB_IF_B_DATA_STRUCT_1: c_uint = 0x160028;
pub const MB_IF_B_DATA_STRUCT_2: c_uint = 0x16002C;
pub const MB_IF_B_DATA_STRUCT_3: c_uint = 0x160030;
pub const MB_IF_B_DATA_STRUCT_4: c_uint = 0x160034;
pub const MB_IF_B_DATA_STRUCT_5: c_uint = 0x160038;
pub const MB_IF_B_DATA_STRUCT_6: c_uint = 0x16003C;
pub const MB_IF_B_DATA_STRUCT_7: c_uint = 0x160040;
pub const MB_IF_B_DATA_STRUCT_8: c_uint = 0x160044;
pub const MB_IF_B_DATA_STRUCT_9: c_uint = 0x160048;
pub const MB_IF_B_DATA_STRUCT_A: c_uint = 0x16004C;
pub const MB_IF_B_DATA_STRUCT_B: c_uint = 0x160050;
pub const MB_IF_B_DATA_STRUCT_C: c_uint = 0x160054;
pub const MB_IF_B_DATA_STRUCT_D: c_uint = 0x160058;
pub const MB_IF_B_DATA_STRUCT_E: c_uint = 0x16005C;
pub const MB_IF_B_DATA_STRUCT_F: c_uint = 0x160060;
// MB_DMA_CTRL
pub const FLD_MB_IF_RISC_EN: c_uint = 0x00000010;
pub const FLD_MB_IF_FIFO_EN: c_uint = 0x00000001;
// MB_LENGTH
pub const FLD_MB_IF_LN_LNGTH: c_uint = 0x00000FFF;
// MB_HCMD register
pub const FLD_MB_HCMD_H_GO: c_uint = 0x80000000;
pub const FLD_MB_HCMD_H_BUSY: c_uint = 0x40000000;
pub const FLD_MB_HCMD_H_DMA_HOLD: c_uint = 0x10000000;
pub const FLD_MB_HCMD_H_DMA_BUSY: c_uint = 0x08000000;
pub const FLD_MB_HCMD_H_DMA_TYPE: c_uint = 0x04000000;
pub const FLD_MB_HCMD_H_DMA_XACT: c_uint = 0x02000000;
pub const FLD_MB_HCMD_H_RW_N: c_uint = 0x01000000;
pub const FLD_MB_HCMD_H_ADDR: c_uint = 0x00FF0000;
pub const FLD_MB_HCMD_H_DATA: c_uint = 0x0000FFFF;
// *****************************************************************************
// I2C #1
// *****************************************************************************
pub const I2C1_ADDR: c_uint = 0x180000	/* I2C #1 address */;
pub const FLD_I2C_DADDR: c_uint = 0xfe000000	/* RW [31:25] I2C Device Address */;
// RO [24] reserved
// *****************************************************************************
pub const FLD_I2C_SADDR: c_uint = 0x00FFFFFF	/* RW [23:0]  I2C Sub-address */;
// *****************************************************************************
pub const I2C1_WDATA: c_uint = 0x180004	/* I2C #1 write data */;
pub const FLD_I2C_WDATA: c_uint = 0xFFFFFFFF	/* RW [31:0] */;
// *****************************************************************************
pub const I2C1_CTRL: c_uint = 0x180008	/* I2C #1 control */;
pub const FLD_I2C_PERIOD: c_uint = 0xFF000000	/* RW [31:24] */;
pub const FLD_I2C_SCL_IN: c_uint = 0x00200000	/* RW [21] */;
pub const FLD_I2C_SDA_IN: c_uint = 0x00100000	/* RW [20] */;
// RO [19:18] reserved
pub const FLD_I2C_SCL_OUT: c_uint = 0x00020000	/* RW [17] */;
pub const FLD_I2C_SDA_OUT: c_uint = 0x00010000	/* RW [16] */;
// RO [15] reserved
pub const FLD_I2C_DATA_LEN: c_uint = 0x00007000	/* RW [14:12] */;
pub const FLD_I2C_SADDR_INC: c_uint = 0x00000800	/* RW [11] */;
// RO [10:9] reserved
pub const FLD_I2C_SADDR_LEN: c_uint = 0x00000300	/* RW [9:8] */;
// RO [7:6] reserved
pub const FLD_I2C_SOFT: c_uint = 0x00000020	/* RW [5] */;
pub const FLD_I2C_NOSTOP: c_uint = 0x00000010	/* RW [4] */;
pub const FLD_I2C_EXTEND: c_uint = 0x00000008	/* RW [3] */;
pub const FLD_I2C_SYNC: c_uint = 0x00000004	/* RW [2] */;
pub const FLD_I2C_READ_SA: c_uint = 0x00000002	/* RW [1] */;
pub const FLD_I2C_READ_WRN: c_uint = 0x00000001	/* RW [0] */;
// *****************************************************************************
pub const I2C1_RDATA: c_uint = 0x18000C	/* I2C #1 read data */;
pub const FLD_I2C_RDATA: c_uint = 0xFFFFFFFF	/* RO [31:0] */;
// *****************************************************************************
pub const I2C1_STAT: c_uint = 0x180010	/* I2C #1 status */;
pub const FLD_I2C_XFER_IN_PROG: c_uint = 0x00000002	/* RO [1] */;
pub const FLD_I2C_RACK: c_uint = 0x00000001	/* RO [0] */;
// *****************************************************************************
// I2C #2
// *****************************************************************************
pub const I2C2_ADDR: c_uint = 0x190000	/* I2C #2 address */;
// *****************************************************************************
pub const I2C2_WDATA: c_uint = 0x190004	/* I2C #2 write data */;
// *****************************************************************************
pub const I2C2_CTRL: c_uint = 0x190008	/* I2C #2 control */;
// *****************************************************************************
pub const I2C2_RDATA: c_uint = 0x19000C	/* I2C #2 read data */;
// *****************************************************************************
pub const I2C2_STAT: c_uint = 0x190010	/* I2C #2 status */;
// *****************************************************************************
// I2C #3
// *****************************************************************************
pub const I2C3_ADDR: c_uint = 0x1A0000	/* I2C #3 address */;
// *****************************************************************************
pub const I2C3_WDATA: c_uint = 0x1A0004	/* I2C #3 write data */;
// *****************************************************************************
pub const I2C3_CTRL: c_uint = 0x1A0008	/* I2C #3 control */;
// *****************************************************************************
pub const I2C3_RDATA: c_uint = 0x1A000C	/* I2C #3 read data */;
// *****************************************************************************
pub const I2C3_STAT: c_uint = 0x1A0010	/* I2C #3 status */;
// *****************************************************************************
// UART
// *****************************************************************************
pub const UART_CTL: c_uint = 0x1B0000	/* UART Control Register */;

// *****************************************************************************
pub const UART_BRD: c_uint = 0x1B0004	/* UART Baud Rate Divisor */;
pub const FLD_BRD: c_uint = 0x0000FFFF	/* RW field - default 0x197 */;
// *****************************************************************************
pub const UART_DBUF: c_uint = 0x1B0008	/* UART Tx/Rx Data BuFFer */;
pub const FLD_DB: c_uint = 0xFFFFFFFF	/* RW field - default 0 */;
// *****************************************************************************
pub const UART_ISR: c_uint = 0x1B000C	/* UART Interrupt Status */;

// *****************************************************************************
pub const UART_CNT: c_uint = 0x1B0010	/* UART Tx/Rx FIFO Byte Count */;

// *****************************************************************************
// Motion Detection
pub const MD_CH0_GRID_BLOCK_YCNT: c_uint = 0x170014;
pub const MD_CH1_GRID_BLOCK_YCNT: c_uint = 0x170094;
pub const MD_CH2_GRID_BLOCK_YCNT: c_uint = 0x170114;
pub const MD_CH3_GRID_BLOCK_YCNT: c_uint = 0x170194;
pub const MD_CH4_GRID_BLOCK_YCNT: c_uint = 0x170214;
pub const MD_CH5_GRID_BLOCK_YCNT: c_uint = 0x170294;
pub const MD_CH6_GRID_BLOCK_YCNT: c_uint = 0x170314;
pub const MD_CH7_GRID_BLOCK_YCNT: c_uint = 0x170394;
pub const PIXEL_FRMT_422: c_int = 4;
pub const PIXEL_FRMT_411: c_int = 5;
pub const PIXEL_FRMT_Y8: c_int = 6;
pub const PIXEL_ENGINE_VIP1: c_int = 0;
pub const PIXEL_ENGINE_VIP2: c_int = 1;
