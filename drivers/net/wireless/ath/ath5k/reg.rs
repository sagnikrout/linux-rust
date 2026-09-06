//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/reg.h
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
// Copyright (c) 2006-2008 Nick Kossifidis <mickflemm@gmail.com>
// Copyright (c) 2004-2008 Reyk Floeter <reyk@openbsd.org>
// Copyright (c) 2007-2008 Michael Taylor <mike.taylor@apprion.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Register values for Atheros 5210/5211/5212 cards from OpenBSD's ar5k
// maintained by Reyk Floeter
//
// I tried to document those registers by looking at ar5k code, some
// 802.11 (802.11e mostly) papers and by reading various public available
// Atheros presentations and papers like these:
//
// 5210 - http://nova.stanford.edu/~bbaas/ps/isscc2002_slides.pdf
//
// 5211 - http://www.hotchips.org/archives/hc14/3_Tue/16_mcfarland.pdf
//
// This file also contains register values found on a memory dump of
// Atheros's ART program (Atheros Radio Test), on ath9k, on legacy-hal
// released by Atheros and on various debug messages found on the net.
//

// ====MAC DMA REGISTERS====
//
// AR5210-Specific TXDP registers
// 5210 has only 2 transmit queues so no DCU/QCU, just
// 2 transmit descriptor pointers...
//
pub const AR5K_NOQCU_TXDP0: c_uint = 0x0000		/* Queue 0 - data */;
pub const AR5K_NOQCU_TXDP1: c_uint = 0x0004		/* Queue 1 - beacons */;
//
// Mac Control Register
//
pub const AR5K_CR: c_uint = 0x0008			/* Register Address */;
pub const AR5K_CR_TXE0: c_uint = 0x00000001	/* TX Enable for queue 0 on 5210 */;
pub const AR5K_CR_TXE1: c_uint = 0x00000002	/* TX Enable for queue 1 on 5210 */;
pub const AR5K_CR_RXE: c_uint = 0x00000004	/* RX Enable */;
pub const AR5K_CR_TXD0: c_uint = 0x00000008	/* TX Disable for queue 0 on 5210 */;
pub const AR5K_CR_TXD1: c_uint = 0x00000010	/* TX Disable for queue 1 on 5210 */;
pub const AR5K_CR_RXD: c_uint = 0x00000020	/* RX Disable */;
pub const AR5K_CR_SWI: c_uint = 0x00000040	/* Software Interrupt */;
//
// RX Descriptor Pointer register
//
pub const AR5K_RXDP: c_uint = 0x000c;
//
// Configuration and status register
//
pub const AR5K_CFG: c_uint = 0x0014			/* Register Address */;
pub const AR5K_CFG_SWTD: c_uint = 0x00000001	/* Byte-swap TX descriptor (for big endian archs) */;
pub const AR5K_CFG_SWTB: c_uint = 0x00000002	/* Byte-swap TX buffer */;
pub const AR5K_CFG_SWRD: c_uint = 0x00000004	/* Byte-swap RX descriptor */;
pub const AR5K_CFG_SWRB: c_uint = 0x00000008	/* Byte-swap RX buffer */;
pub const AR5K_CFG_SWRG: c_uint = 0x00000010	/* Byte-swap Register access */;
pub const AR5K_CFG_IBSS: c_uint = 0x00000020	/* 0-BSS, 1-IBSS [5211+] */;
pub const AR5K_CFG_PHY_OK: c_uint = 0x00000100	/* [5211+] */;
pub const AR5K_CFG_EEBS: c_uint = 0x00000200	/* EEPROM is busy */;
pub const AR5K_CFG_CLKGD: c_uint = 0x00000400	/* Clock gated (Disable dynamic clock) */;
pub const AR5K_CFG_TXCNT: c_uint = 0x00007800	/* Tx frame count (?) [5210] */;
pub const AR5K_CFG_TXCNT_S: c_int = 11;
pub const AR5K_CFG_TXFSTAT: c_uint = 0x00008000	/* Tx frame status (?) [5210] */;
pub const AR5K_CFG_TXFSTRT: c_uint = 0x00010000	/* [5210] */;
pub const AR5K_CFG_PCI_THRES: c_uint = 0x00060000	/* PCI Master req q threshold [5211+] */;
pub const AR5K_CFG_PCI_THRES_S: c_int = 17;
//
// Interrupt enable register
//
pub const AR5K_IER: c_uint = 0x0024		/* Register Address */;
pub const AR5K_IER_DISABLE: c_uint = 0x00000000	/* Disable card interrupts */;
pub const AR5K_IER_ENABLE: c_uint = 0x00000001	/* Enable card interrupts */;
//
// 0x0028 is Beacon Control Register on 5210
// and first RTS duration register on 5211
//
// Beacon control register [5210]
//
pub const AR5K_BCR: c_uint = 0x0028		/* Register Address */;
pub const AR5K_BCR_AP: c_uint = 0x00000000	/* AP mode */;
pub const AR5K_BCR_ADHOC: c_uint = 0x00000001	/* Ad-Hoc mode */;
pub const AR5K_BCR_BDMAE: c_uint = 0x00000002	/* DMA enable */;
pub const AR5K_BCR_TQ1FV: c_uint = 0x00000004	/* Use Queue1 for CAB traffic */;
pub const AR5K_BCR_TQ1V: c_uint = 0x00000008	/* Use Queue1 for Beacon traffic */;
pub const AR5K_BCR_BCGET: c_uint = 0x00000010;
//
// First RTS duration register [5211]
//
pub const AR5K_RTSD0: c_uint = 0x0028		/* Register Address */;
pub const AR5K_RTSD0_6: c_uint = 0x000000ff	/* 6Mb RTS duration mask (?) */;

pub const AR5K_RTSD0_9: c_uint = 0x0000ff00	/* 9Mb*/;
pub const AR5K_RTSD0_9_S: c_int = 8;
pub const AR5K_RTSD0_12: c_uint = 0x00ff0000	/* 12Mb*/;
pub const AR5K_RTSD0_12_S: c_int = 16;
pub const AR5K_RTSD0_18: c_uint = 0xff000000	/* 16Mb*/;
pub const AR5K_RTSD0_18_S: c_int = 24;
//
// 0x002c is Beacon Status Register on 5210
// and second RTS duration register on 5211
//
// Beacon status register [5210]
//
// As i can see in ar5k_ar5210_tx_start Reyk uses some of the values of BCR
// for this register, so i guess TQ1V,TQ1FV and BDMAE have the same meaning
// here and SNP/SNAP means "snapshot" (so this register gets synced with BCR).
// So SNAPPEDBCRVALID should also stand for "snapped BCR -values- valid", so i
// renamed it to SNAPSHOTSVALID to make more sense. I really have no idea what
// else can it be. I also renamed SNPBCMD to SNPADHOC to match BCR.
//
pub const AR5K_BSR: c_uint = 0x002c			/* Register Address */;
pub const AR5K_BSR_BDLYSW: c_uint = 0x00000001	/* SW Beacon delay (?) */;
pub const AR5K_BSR_BDLYDMA: c_uint = 0x00000002	/* DMA Beacon delay (?) */;
pub const AR5K_BSR_TXQ1F: c_uint = 0x00000004	/* Beacon queue (1) finished */;
pub const AR5K_BSR_ATIMDLY: c_uint = 0x00000008	/* ATIM delay (?) */;
pub const AR5K_BSR_SNPADHOC: c_uint = 0x00000100	/* Ad-hoc mode set (?) */;
pub const AR5K_BSR_SNPBDMAE: c_uint = 0x00000200	/* Beacon DMA enabled (?) */;
pub const AR5K_BSR_SNPTQ1FV: c_uint = 0x00000400	/* Queue1 is used for CAB traffic (?) */;
pub const AR5K_BSR_SNPTQ1V: c_uint = 0x00000800	/* Queue1 is used for Beacon traffic (?) */;
pub const AR5K_BSR_SNAPSHOTSVALID: c_uint = 0x00001000	/* BCR snapshots are valid (?) */;
pub const AR5K_BSR_SWBA_CNT: c_uint = 0x00ff0000;
//
// Second RTS duration register [5211]
//
pub const AR5K_RTSD1: c_uint = 0x002c			/* Register Address */;
pub const AR5K_RTSD1_24: c_uint = 0x000000ff	/* 24Mb */;
pub const AR5K_RTSD1_24_S: c_int = 0;
pub const AR5K_RTSD1_36: c_uint = 0x0000ff00	/* 36Mb */;
pub const AR5K_RTSD1_36_S: c_int = 8;
pub const AR5K_RTSD1_48: c_uint = 0x00ff0000	/* 48Mb */;
pub const AR5K_RTSD1_48_S: c_int = 16;
pub const AR5K_RTSD1_54: c_uint = 0xff000000	/* 54Mb */;
pub const AR5K_RTSD1_54_S: c_int = 24;
//
// Transmit configuration register
//
pub const AR5K_TXCFG: c_uint = 0x0030			/* Register Address */;
pub const AR5K_TXCFG_SDMAMR: c_uint = 0x00000007	/* DMA size (read) */;
pub const AR5K_TXCFG_SDMAMR_S: c_int = 0;
pub const AR5K_TXCFG_B_MODE: c_uint = 0x00000008	/* Set b mode for 5111 (enable 2111) */;
pub const AR5K_TXCFG_TXFSTP: c_uint = 0x00000008	/* TX DMA full Stop [5210] */;
pub const AR5K_TXCFG_TXFULL: c_uint = 0x000003f0	/* TX Trigger level mask */;
pub const AR5K_TXCFG_TXFULL_S: c_int = 4;
pub const AR5K_TXCFG_TXFULL_0B: c_uint = 0x00000000;
pub const AR5K_TXCFG_TXFULL_64B: c_uint = 0x00000010;
pub const AR5K_TXCFG_TXFULL_128B: c_uint = 0x00000020;
pub const AR5K_TXCFG_TXFULL_192B: c_uint = 0x00000030;
pub const AR5K_TXCFG_TXFULL_256B: c_uint = 0x00000040;
pub const AR5K_TXCFG_TXCONT_EN: c_uint = 0x00000080;
pub const AR5K_TXCFG_DMASIZE: c_uint = 0x00000100	/* Flag for passing DMA size [5210] */;
pub const AR5K_TXCFG_JUMBO_DESC_EN: c_uint = 0x00000400	/* Enable jumbo tx descriptors [5211+] */;
pub const AR5K_TXCFG_ADHOC_BCN_ATIM: c_uint = 0x00000800	/* Adhoc Beacon ATIM Policy */;
pub const AR5K_TXCFG_ATIM_WINDOW_DEF_DIS: c_uint = 0x00001000	/* Disable ATIM window defer [5211+] */;
pub const AR5K_TXCFG_RTSRND: c_uint = 0x00001000	/* [5211+] */;
pub const AR5K_TXCFG_FRMPAD_DIS: c_uint = 0x00002000	/* [5211+] */;
pub const AR5K_TXCFG_RDY_CBR_DIS: c_uint = 0x00004000	/* Ready time CBR disable [5211+] */;
pub const AR5K_TXCFG_JUMBO_FRM_MODE: c_uint = 0x00008000	/* Jumbo frame mode [5211+] */;
pub const AR5K_TXCFG_DCU_DBL_BUF_DIS: c_uint = 0x00008000	/* Disable double buffering on DCU */;
pub const AR5K_TXCFG_DCU_CACHING_DIS: c_uint = 0x00010000	/* Disable DCU caching */;
//
// Receive configuration register
//
pub const AR5K_RXCFG: c_uint = 0x0034			/* Register Address */;
pub const AR5K_RXCFG_SDMAMW: c_uint = 0x00000007	/* DMA size (write) */;
pub const AR5K_RXCFG_SDMAMW_S: c_int = 0;
pub const AR5K_RXCFG_ZLFDMA: c_uint = 0x00000008	/* Enable Zero-length frame DMA */;
pub const AR5K_RXCFG_DEF_ANTENNA: c_uint = 0x00000010	/* Default antenna (?) */;
pub const AR5K_RXCFG_JUMBO_RXE: c_uint = 0x00000020	/* Enable jumbo rx descriptors [5211+] */;
pub const AR5K_RXCFG_JUMBO_WRAP: c_uint = 0x00000040	/* Wrap jumbo frames [5211+] */;
pub const AR5K_RXCFG_SLE_ENTRY: c_uint = 0x00000080	/* Sleep entry policy */;
//
// Receive jumbo descriptor last address register
// Only found in 5211 (?)
//
pub const AR5K_RXJLA: c_uint = 0x0038;
//
// MIB control register
//
pub const AR5K_MIBC: c_uint = 0x0040			/* Register Address */;
pub const AR5K_MIBC_COW: c_uint = 0x00000001	/* Counter Overflow Warning */;
pub const AR5K_MIBC_FMC: c_uint = 0x00000002	/* Freeze MIB Counters  */;
pub const AR5K_MIBC_CMC: c_uint = 0x00000004	/* Clear MIB Counters  */;
pub const AR5K_MIBC_MCS: c_uint = 0x00000008	/* MIB counter strobe, increment all */;
//
// Timeout prescale register
//
pub const AR5K_TOPS: c_uint = 0x0044;
pub const AR5K_TOPS_M: c_uint = 0x0000ffff;
//
// Receive timeout register (no frame received)
//
pub const AR5K_RXNOFRM: c_uint = 0x0048;
pub const AR5K_RXNOFRM_M: c_uint = 0x000003ff;
//
// Transmit timeout register (no frame sent)
//
pub const AR5K_TXNOFRM: c_uint = 0x004c;
pub const AR5K_TXNOFRM_M: c_uint = 0x000003ff;
pub const AR5K_TXNOFRM_QCU: c_uint = 0x000ffc00;
pub const AR5K_TXNOFRM_QCU_S: c_int = 10;
//
// Receive frame gap timeout register
//
pub const AR5K_RPGTO: c_uint = 0x0050;
pub const AR5K_RPGTO_M: c_uint = 0x000003ff;
//
// Receive frame count limit register
//
pub const AR5K_RFCNT: c_uint = 0x0054;
pub const AR5K_RFCNT_M: c_uint = 0x0000001f	/* [5211+] (?) */;
pub const AR5K_RFCNT_RFCL: c_uint = 0x0000000f	/* [5210] */;
//
// Misc settings register
// (reserved0-3)
//
pub const AR5K_MISC: c_uint = 0x0058			/* Register Address */;
pub const AR5K_MISC_DMA_OBS_M: c_uint = 0x000001e0;
pub const AR5K_MISC_DMA_OBS_S: c_int = 5;
pub const AR5K_MISC_MISC_OBS_M: c_uint = 0x00000e00;
pub const AR5K_MISC_MISC_OBS_S: c_int = 9;
pub const AR5K_MISC_MAC_OBS_LSB_M: c_uint = 0x00007000;
pub const AR5K_MISC_MAC_OBS_LSB_S: c_int = 12;
pub const AR5K_MISC_MAC_OBS_MSB_M: c_uint = 0x00038000;
pub const AR5K_MISC_MAC_OBS_MSB_S: c_int = 15;
pub const AR5K_MISC_LED_DECAY: c_uint = 0x001c0000	/* [5210] */;
pub const AR5K_MISC_LED_BLINK: c_uint = 0x00e00000	/* [5210] */;
//
// QCU/DCU clock gating register (5311)
// (reserved4-5)
//
pub const AR5K_QCUDCU_CLKGT: c_uint = 0x005c			/* Register Address (?) */;
pub const AR5K_QCUDCU_CLKGT_QCU: c_uint = 0x0000ffff	/* Mask for QCU clock */;
pub const AR5K_QCUDCU_CLKGT_DCU: c_uint = 0x07ff0000	/* Mask for DCU clock */;
//
// Interrupt Status Registers
//
// For 5210 there is only one status register but for
// 5211/5212 we have one primary and 4 secondary registers.
// So we have AR5K_ISR for 5210 and AR5K_PISR /SISRx for 5211/5212.
// Most of these bits are common for all chipsets.
//
// NOTE: On 5211+ TXOK, TXDESC, TXERR, TXEOL and TXURN contain
// the logical OR from per-queue interrupt bits found on SISR registers
// (see below).
//
pub const AR5K_ISR: c_uint = 0x001c			/* Register Address [5210] */;
pub const AR5K_PISR: c_uint = 0x0080			/* Register Address [5211+] */;
pub const AR5K_ISR_RXOK: c_uint = 0x00000001	/* Frame successfully received */;
pub const AR5K_ISR_RXDESC: c_uint = 0x00000002	/* RX descriptor request */;
pub const AR5K_ISR_RXERR: c_uint = 0x00000004	/* Receive error */;
pub const AR5K_ISR_RXNOFRM: c_uint = 0x00000008	/* No frame received (receive timeout) */;
pub const AR5K_ISR_RXEOL: c_uint = 0x00000010	/* Empty RX descriptor */;
pub const AR5K_ISR_RXORN: c_uint = 0x00000020	/* Receive FIFO overrun */;
pub const AR5K_ISR_TXOK: c_uint = 0x00000040	/* Frame successfully transmitted */;
pub const AR5K_ISR_TXDESC: c_uint = 0x00000080	/* TX descriptor request */;
pub const AR5K_ISR_TXERR: c_uint = 0x00000100	/* Transmit error */;
pub const AR5K_ISR_TXNOFRM: c_uint = 0x00000200	/* No frame transmitted (transmit timeout);
// NOTE: We don't have per-queue info for this
// one, but we can enable it per-queue through
// TXNOFRM_QCU field on TXNOFRM register
pub const AR5K_ISR_TXEOL: c_uint = 0x00000400	/* Empty TX descriptor */;
pub const AR5K_ISR_TXURN: c_uint = 0x00000800	/* Transmit FIFO underrun */;
pub const AR5K_ISR_MIB: c_uint = 0x00001000	/* Update MIB counters */;
pub const AR5K_ISR_SWI: c_uint = 0x00002000	/* Software interrupt */;
pub const AR5K_ISR_RXPHY: c_uint = 0x00004000	/* PHY error */;
pub const AR5K_ISR_RXKCM: c_uint = 0x00008000	/* RX Key cache miss */;
pub const AR5K_ISR_SWBA: c_uint = 0x00010000	/* Software beacon alert */;
pub const AR5K_ISR_BRSSI: c_uint = 0x00020000	/* Beacon rssi below threshold (?) */;
pub const AR5K_ISR_BMISS: c_uint = 0x00040000	/* Beacon missed */;
pub const AR5K_ISR_HIUERR: c_uint = 0x00080000	/* Host Interface Unit error [5211+];
// 'or' of MCABT, SSERR, DPERR from SISR2
pub const AR5K_ISR_BNR: c_uint = 0x00100000	/* Beacon not ready [5211+] */;
pub const AR5K_ISR_MCABT: c_uint = 0x00100000	/* Master Cycle Abort [5210] */;
pub const AR5K_ISR_RXCHIRP: c_uint = 0x00200000	/* CHIRP Received [5212+] */;
pub const AR5K_ISR_SSERR: c_uint = 0x00200000	/* Signaled System Error [5210] */;
pub const AR5K_ISR_DPERR: c_uint = 0x00400000	/* Bus parity error [5210] */;
pub const AR5K_ISR_RXDOPPLER: c_uint = 0x00400000	/* Doppler chirp received [5212+] */;
pub const AR5K_ISR_TIM: c_uint = 0x00800000	/* [5211+] */;
pub const AR5K_ISR_BCNMISC: c_uint = 0x00800000	/* Misc beacon related interrupt;
// 'or' of TIM, CAB_END, DTIM_SYNC, BCN_TIMEOUT,
// CAB_TIMEOUT and DTIM bits from SISR2 [5212+]
pub const AR5K_ISR_GPIO: c_uint = 0x01000000	/* GPIO (rf kill) */;
pub const AR5K_ISR_QCBRORN: c_uint = 0x02000000	/* QCU CBR overrun [5211+] */;
pub const AR5K_ISR_QCBRURN: c_uint = 0x04000000	/* QCU CBR underrun [5211+] */;
pub const AR5K_ISR_QTRIG: c_uint = 0x08000000	/* QCU scheduling trigger [5211+] */;

//
// Secondary status registers [5211+] (0 - 4)
//
// These give the status for each QCU, only QCUs 0-9 are
// represented.
//
pub const AR5K_SISR0: c_uint = 0x0084			/* Register Address [5211+] */;
pub const AR5K_SISR0_QCU_TXOK: c_uint = 0x000003ff	/* Mask for QCU_TXOK */;
pub const AR5K_SISR0_QCU_TXOK_S: c_int = 0;
pub const AR5K_SISR0_QCU_TXDESC: c_uint = 0x03ff0000	/* Mask for QCU_TXDESC */;
pub const AR5K_SISR0_QCU_TXDESC_S: c_int = 16;
pub const AR5K_SISR1: c_uint = 0x0088			/* Register Address [5211+] */;
pub const AR5K_SISR1_QCU_TXERR: c_uint = 0x000003ff	/* Mask for QCU_TXERR */;
pub const AR5K_SISR1_QCU_TXERR_S: c_int = 0;
pub const AR5K_SISR1_QCU_TXEOL: c_uint = 0x03ff0000	/* Mask for QCU_TXEOL */;
pub const AR5K_SISR1_QCU_TXEOL_S: c_int = 16;
pub const AR5K_SISR2: c_uint = 0x008c			/* Register Address [5211+] */;
pub const AR5K_SISR2_QCU_TXURN: c_uint = 0x000003ff	/* Mask for QCU_TXURN */;
pub const AR5K_SISR2_QCU_TXURN_S: c_int = 0;
pub const AR5K_SISR2_MCABT: c_uint = 0x00010000	/* Master Cycle Abort */;
pub const AR5K_SISR2_SSERR: c_uint = 0x00020000	/* Signaled System Error */;
pub const AR5K_SISR2_DPERR: c_uint = 0x00040000	/* Bus parity error */;
pub const AR5K_SISR2_TIM: c_uint = 0x01000000	/* [5212+] */;
pub const AR5K_SISR2_CAB_END: c_uint = 0x02000000	/* [5212+] */;
pub const AR5K_SISR2_DTIM_SYNC: c_uint = 0x04000000	/* DTIM sync lost [5212+] */;
pub const AR5K_SISR2_BCN_TIMEOUT: c_uint = 0x08000000	/* Beacon Timeout [5212+] */;
pub const AR5K_SISR2_CAB_TIMEOUT: c_uint = 0x10000000	/* CAB Timeout [5212+] */;
pub const AR5K_SISR2_DTIM: c_uint = 0x20000000	/* [5212+] */;
pub const AR5K_SISR2_TSFOOR: c_uint = 0x80000000	/* TSF Out of range */;
pub const AR5K_SISR3: c_uint = 0x0090			/* Register Address [5211+] */;
pub const AR5K_SISR3_QCBRORN: c_uint = 0x000003ff	/* Mask for QCBRORN */;
pub const AR5K_SISR3_QCBRORN_S: c_int = 0;
pub const AR5K_SISR3_QCBRURN: c_uint = 0x03ff0000	/* Mask for QCBRURN */;
pub const AR5K_SISR3_QCBRURN_S: c_int = 16;
pub const AR5K_SISR4: c_uint = 0x0094			/* Register Address [5211+] */;
pub const AR5K_SISR4_QTRIG: c_uint = 0x000003ff	/* Mask for QTRIG */;
pub const AR5K_SISR4_QTRIG_S: c_int = 0;
//
// Shadow read-and-clear interrupt status registers [5211+]
//
pub const AR5K_RAC_PISR: c_uint = 0x00c0		/* Read and clear PISR */;
pub const AR5K_RAC_SISR0: c_uint = 0x00c4		/* Read and clear SISR0 */;
pub const AR5K_RAC_SISR1: c_uint = 0x00c8		/* Read and clear SISR1 */;
pub const AR5K_RAC_SISR2: c_uint = 0x00cc		/* Read and clear SISR2 */;
pub const AR5K_RAC_SISR3: c_uint = 0x00d0		/* Read and clear SISR3 */;
pub const AR5K_RAC_SISR4: c_uint = 0x00d4		/* Read and clear SISR4 */;
//
// Interrupt Mask Registers
//
// As with ISRs 5210 has one IMR (AR5K_IMR) and 5211/5212 has one primary
// (AR5K_PIMR) and 4 secondary IMRs (AR5K_SIMRx). Note that ISR/IMR flags match.
//
pub const AR5K_IMR: c_uint = 0x0020			/* Register Address [5210] */;
pub const AR5K_PIMR: c_uint = 0x00a0			/* Register Address [5211+] */;
pub const AR5K_IMR_RXOK: c_uint = 0x00000001	/* Frame successfully received*/;
pub const AR5K_IMR_RXDESC: c_uint = 0x00000002	/* RX descriptor request*/;
pub const AR5K_IMR_RXERR: c_uint = 0x00000004	/* Receive error*/;
pub const AR5K_IMR_RXNOFRM: c_uint = 0x00000008	/* No frame received (receive timeout)*/;
pub const AR5K_IMR_RXEOL: c_uint = 0x00000010	/* Empty RX descriptor*/;
pub const AR5K_IMR_RXORN: c_uint = 0x00000020	/* Receive FIFO overrun*/;
pub const AR5K_IMR_TXOK: c_uint = 0x00000040	/* Frame successfully transmitted*/;
pub const AR5K_IMR_TXDESC: c_uint = 0x00000080	/* TX descriptor request*/;
pub const AR5K_IMR_TXERR: c_uint = 0x00000100	/* Transmit error*/;
pub const AR5K_IMR_TXNOFRM: c_uint = 0x00000200	/* No frame transmitted (transmit timeout)*/;
pub const AR5K_IMR_TXEOL: c_uint = 0x00000400	/* Empty TX descriptor*/;
pub const AR5K_IMR_TXURN: c_uint = 0x00000800	/* Transmit FIFO underrun*/;
pub const AR5K_IMR_MIB: c_uint = 0x00001000	/* Update MIB counters*/;
pub const AR5K_IMR_SWI: c_uint = 0x00002000	/* Software interrupt */;
pub const AR5K_IMR_RXPHY: c_uint = 0x00004000	/* PHY error*/;
pub const AR5K_IMR_RXKCM: c_uint = 0x00008000	/* RX Key cache miss */;
pub const AR5K_IMR_SWBA: c_uint = 0x00010000	/* Software beacon alert*/;
pub const AR5K_IMR_BRSSI: c_uint = 0x00020000	/* Beacon rssi below threshold (?) */;
pub const AR5K_IMR_BMISS: c_uint = 0x00040000	/* Beacon missed*/;
pub const AR5K_IMR_HIUERR: c_uint = 0x00080000	/* Host Interface Unit error [5211+] */;
pub const AR5K_IMR_BNR: c_uint = 0x00100000	/* Beacon not ready [5211+] */;
pub const AR5K_IMR_MCABT: c_uint = 0x00100000	/* Master Cycle Abort [5210] */;
pub const AR5K_IMR_RXCHIRP: c_uint = 0x00200000	/* CHIRP Received [5212+]*/;
pub const AR5K_IMR_SSERR: c_uint = 0x00200000	/* Signaled System Error [5210] */;
pub const AR5K_IMR_DPERR: c_uint = 0x00400000	/* Det par Error (?) [5210] */;
pub const AR5K_IMR_RXDOPPLER: c_uint = 0x00400000	/* Doppler chirp received [5212+] */;
pub const AR5K_IMR_TIM: c_uint = 0x00800000	/* [5211+] */;
pub const AR5K_IMR_BCNMISC: c_uint = 0x00800000	/* 'or' of TIM, CAB_END, DTIM_SYNC, BCN_TIMEOUT,;
pub const AR5K_IMR_GPIO: c_uint = 0x01000000	/* GPIO (rf kill)*/;
pub const AR5K_IMR_QCBRORN: c_uint = 0x02000000	/* QCU CBR overrun (?) [5211+] */;
pub const AR5K_IMR_QCBRURN: c_uint = 0x04000000	/* QCU CBR underrun (?) [5211+] */;
pub const AR5K_IMR_QTRIG: c_uint = 0x08000000	/* QCU scheduling trigger [5211+] */;
//
// Secondary interrupt mask registers [5211+] (0 - 4)
//
pub const AR5K_SIMR0: c_uint = 0x00a4			/* Register Address [5211+] */;
pub const AR5K_SIMR0_QCU_TXOK: c_uint = 0x000003ff	/* Mask for QCU_TXOK */;
pub const AR5K_SIMR0_QCU_TXOK_S: c_int = 0;
pub const AR5K_SIMR0_QCU_TXDESC: c_uint = 0x03ff0000	/* Mask for QCU_TXDESC */;
pub const AR5K_SIMR0_QCU_TXDESC_S: c_int = 16;
pub const AR5K_SIMR1: c_uint = 0x00a8			/* Register Address [5211+] */;
pub const AR5K_SIMR1_QCU_TXERR: c_uint = 0x000003ff	/* Mask for QCU_TXERR */;
pub const AR5K_SIMR1_QCU_TXERR_S: c_int = 0;
pub const AR5K_SIMR1_QCU_TXEOL: c_uint = 0x03ff0000	/* Mask for QCU_TXEOL */;
pub const AR5K_SIMR1_QCU_TXEOL_S: c_int = 16;
pub const AR5K_SIMR2: c_uint = 0x00ac			/* Register Address [5211+] */;
pub const AR5K_SIMR2_QCU_TXURN: c_uint = 0x000003ff	/* Mask for QCU_TXURN */;
pub const AR5K_SIMR2_QCU_TXURN_S: c_int = 0;
pub const AR5K_SIMR2_MCABT: c_uint = 0x00010000	/* Master Cycle Abort */;
pub const AR5K_SIMR2_SSERR: c_uint = 0x00020000	/* Signaled System Error */;
pub const AR5K_SIMR2_DPERR: c_uint = 0x00040000	/* Bus parity error */;
pub const AR5K_SIMR2_TIM: c_uint = 0x01000000	/* [5212+] */;
pub const AR5K_SIMR2_CAB_END: c_uint = 0x02000000	/* [5212+] */;
pub const AR5K_SIMR2_DTIM_SYNC: c_uint = 0x04000000	/* DTIM Sync lost [5212+] */;
pub const AR5K_SIMR2_BCN_TIMEOUT: c_uint = 0x08000000	/* Beacon Timeout [5212+] */;
pub const AR5K_SIMR2_CAB_TIMEOUT: c_uint = 0x10000000	/* CAB Timeout [5212+] */;
pub const AR5K_SIMR2_DTIM: c_uint = 0x20000000	/* [5212+] */;
pub const AR5K_SIMR2_TSFOOR: c_uint = 0x80000000	/* TSF OOR (?) */;
pub const AR5K_SIMR3: c_uint = 0x00b0			/* Register Address [5211+] */;
pub const AR5K_SIMR3_QCBRORN: c_uint = 0x000003ff	/* Mask for QCBRORN */;
pub const AR5K_SIMR3_QCBRORN_S: c_int = 0;
pub const AR5K_SIMR3_QCBRURN: c_uint = 0x03ff0000	/* Mask for QCBRURN */;
pub const AR5K_SIMR3_QCBRURN_S: c_int = 16;
pub const AR5K_SIMR4: c_uint = 0x00b4			/* Register Address [5211+] */;
pub const AR5K_SIMR4_QTRIG: c_uint = 0x000003ff	/* Mask for QTRIG */;
pub const AR5K_SIMR4_QTRIG_S: c_int = 0;
//
// DMA Debug registers 0-7
// 0xe0 - 0xfc
//
// Decompression mask registers [5212+]
//
pub const AR5K_DCM_ADDR: c_uint = 0x0400		/*Decompression mask address (index) */;
pub const AR5K_DCM_DATA: c_uint = 0x0404		/*Decompression mask data */;
//
// Wake On Wireless pattern control register [5212+]
//
pub const AR5K_WOW_PCFG: c_uint = 0x0410			/* Register Address */;
pub const AR5K_WOW_PCFG_PAT_MATCH_EN: c_uint = 0x00000001	/* Pattern match enable */;
pub const AR5K_WOW_PCFG_LONG_FRAME_POL: c_uint = 0x00000002	/* Long frame policy */;
pub const AR5K_WOW_PCFG_WOBMISS: c_uint = 0x00000004	/* Wake on bea(con) miss (?) */;
pub const AR5K_WOW_PCFG_PAT_0_EN: c_uint = 0x00000100	/* Enable pattern 0 */;
pub const AR5K_WOW_PCFG_PAT_1_EN: c_uint = 0x00000200	/* Enable pattern 1 */;
pub const AR5K_WOW_PCFG_PAT_2_EN: c_uint = 0x00000400	/* Enable pattern 2 */;
pub const AR5K_WOW_PCFG_PAT_3_EN: c_uint = 0x00000800	/* Enable pattern 3 */;
pub const AR5K_WOW_PCFG_PAT_4_EN: c_uint = 0x00001000	/* Enable pattern 4 */;
pub const AR5K_WOW_PCFG_PAT_5_EN: c_uint = 0x00002000	/* Enable pattern 5 */;
//
// Wake On Wireless pattern index register (?) [5212+]
//
pub const AR5K_WOW_PAT_IDX: c_uint = 0x0414;
//
// Wake On Wireless pattern data register [5212+]
//
pub const AR5K_WOW_PAT_DATA: c_uint = 0x0418			/* Register Address */;
pub const AR5K_WOW_PAT_DATA_0_3_V: c_uint = 0x00000001	/* Pattern 0, 3 value */;
pub const AR5K_WOW_PAT_DATA_1_4_V: c_uint = 0x00000100	/* Pattern 1, 4 value */;
pub const AR5K_WOW_PAT_DATA_2_5_V: c_uint = 0x00010000	/* Pattern 2, 5 value */;
pub const AR5K_WOW_PAT_DATA_0_3_M: c_uint = 0x01000000	/* Pattern 0, 3 mask */;
pub const AR5K_WOW_PAT_DATA_1_4_M: c_uint = 0x04000000	/* Pattern 1, 4 mask */;
pub const AR5K_WOW_PAT_DATA_2_5_M: c_uint = 0x10000000	/* Pattern 2, 5 mask */;
//
// Decompression configuration registers [5212+]
//
pub const AR5K_DCCFG: c_uint = 0x0420			/* Register Address */;
pub const AR5K_DCCFG_GLOBAL_EN: c_uint = 0x00000001	/* Enable decompression on all queues */;
pub const AR5K_DCCFG_BYPASS_EN: c_uint = 0x00000002	/* Bypass decompression */;
pub const AR5K_DCCFG_BCAST_EN: c_uint = 0x00000004	/* Enable decompression for bcast frames */;
pub const AR5K_DCCFG_MCAST_EN: c_uint = 0x00000008	/* Enable decompression for mcast frames */;
//
// Compression configuration registers [5212+]
//
pub const AR5K_CCFG: c_uint = 0x0600			/* Register Address */;
pub const AR5K_CCFG_WINDOW_SIZE: c_uint = 0x00000007	/* Compression window size */;
pub const AR5K_CCFG_CPC_EN: c_uint = 0x00000008	/* Enable performance counters */;
pub const AR5K_CCFG_CCU: c_uint = 0x0604			/* Register Address */;
pub const AR5K_CCFG_CCU_CUP_EN: c_uint = 0x00000001	/* CCU Catchup enable */;
pub const AR5K_CCFG_CCU_CREDIT: c_uint = 0x00000002	/* CCU Credit (field) */;
pub const AR5K_CCFG_CCU_CD_THRES: c_uint = 0x00000080	/* CCU Cyc(lic?) debt threshold (field) */;
pub const AR5K_CCFG_CCU_CUP_LCNT: c_uint = 0x00010000	/* CCU Catchup lit(?) count */;
pub const AR5K_CCFG_CCU_INIT: c_uint = 0x00100200	/* Initial value during reset */;
//
// Compression performance counter registers [5212+]
//
pub const AR5K_CPC0: c_uint = 0x0610		/* Compression performance counter 0 */;
pub const AR5K_CPC1: c_uint = 0x0614		/* Compression performance counter 1*/;
pub const AR5K_CPC2: c_uint = 0x0618		/* Compression performance counter 2 */;
pub const AR5K_CPC3: c_uint = 0x061c		/* Compression performance counter 3 */;
pub const AR5K_CPCOVF: c_uint = 0x0620		/* Compression performance overflow */;
//
// Queue control unit (QCU) registers [5211+]
//
// Card has 12 TX Queues but i see that only 0-9 are used (?)
// both in binary HAL (see ah.h) and ar5k. Each queue has its own
// TXDP at addresses 0x0800 - 0x082c, a CBR (Constant Bit Rate)
// configuration register (0x08c0 - 0x08ec), a ready time configuration
// register (0x0900 - 0x092c), a misc configuration register (0x09c0 -
// 0x09ec) and a status register (0x0a00 - 0x0a2c). We also have some
// global registers, QCU transmit enable/disable and "one shot arm (?)"
// set/clear, which contain status for all queues (we shift by 1 for each
// queue). To access these registers easily we define some macros here
// that are used inside HAL. For more infos check out *_tx_queue functs.
//
// Generic QCU Register access macros
//

//
// QCU Transmit descriptor pointer registers
//
pub const AR5K_QCU_TXDP_BASE: c_uint = 0x0800		/* Register Address - Queue0 TXDP */;

//
// QCU Transmit enable register
//
pub const AR5K_QCU_TXE: c_uint = 0x0840;

//
// QCU Transmit disable register
//
pub const AR5K_QCU_TXD: c_uint = 0x0880;

//
// QCU Constant Bit Rate configuration registers
//
pub const AR5K_QCU_CBRCFG_BASE: c_uint = 0x08c0	/* Register Address - Queue0 CBRCFG */;
pub const AR5K_QCU_CBRCFG_INTVAL: c_uint = 0x00ffffff	/* CBR Interval mask */;
pub const AR5K_QCU_CBRCFG_INTVAL_S: c_int = 0;
pub const AR5K_QCU_CBRCFG_ORN_THRES: c_uint = 0xff000000	/* CBR overrun threshold mask */;
pub const AR5K_QCU_CBRCFG_ORN_THRES_S: c_int = 24;

//
// QCU Ready time configuration registers
//
pub const AR5K_QCU_RDYTIMECFG_BASE: c_uint = 0x0900	/* Register Address - Queue0 RDYTIMECFG */;
pub const AR5K_QCU_RDYTIMECFG_INTVAL: c_uint = 0x00ffffff	/* Ready time interval mask */;
pub const AR5K_QCU_RDYTIMECFG_INTVAL_S: c_int = 0;
pub const AR5K_QCU_RDYTIMECFG_ENABLE: c_uint = 0x01000000	/* Ready time enable mask */;

//
// QCU one shot arm set registers
//
pub const AR5K_QCU_ONESHOTARM_SET: c_uint = 0x0940	/* Register Address -QCU "one shot arm set (?)" */;
pub const AR5K_QCU_ONESHOTARM_SET_M: c_uint = 0x0000ffff;
//
// QCU one shot arm clear registers
//
pub const AR5K_QCU_ONESHOTARM_CLEAR: c_uint = 0x0980	/* Register Address -QCU "one shot arm clear (?)" */;
pub const AR5K_QCU_ONESHOTARM_CLEAR_M: c_uint = 0x0000ffff;
//
// QCU misc registers
//
pub const AR5K_QCU_MISC_BASE: c_uint = 0x09c0			/* Register Address -Queue0 MISC */;
pub const AR5K_QCU_MISC_FRSHED_M: c_uint = 0x0000000f	/* Frame scheduling mask */;

pub const AR5K_QCU_MISC_ONESHOT_ENABLE: c_uint = 0x00000010	/* Oneshot enable */;
pub const AR5K_QCU_MISC_CBREXP_DIS: c_uint = 0x00000020	/* Disable CBR expired counter (normal queue) */;
pub const AR5K_QCU_MISC_CBREXP_BCN_DIS: c_uint = 0x00000040	/* Disable CBR expired counter (beacon queue) */;
pub const AR5K_QCU_MISC_BCN_ENABLE: c_uint = 0x00000080	/* Enable Beacon use */;
pub const AR5K_QCU_MISC_CBR_THRES_ENABLE: c_uint = 0x00000100	/* CBR expired threshold enabled */;
pub const AR5K_QCU_MISC_RDY_VEOL_POLICY: c_uint = 0x00000200	/* TXE reset when RDYTIME expired or VEOL */;
pub const AR5K_QCU_MISC_CBR_RESET_CNT: c_uint = 0x00000400	/* CBR threshold (counter) reset */;
pub const AR5K_QCU_MISC_DCU_EARLY: c_uint = 0x00000800	/* DCU early termination */;
pub const AR5K_QCU_MISC_DCU_CMP_EN: c_uint = 0x00001000	/* Enable frame compression */;

//
// QCU status registers
//
pub const AR5K_QCU_STS_BASE: c_uint = 0x0a00			/* Register Address - Queue0 STS */;
pub const AR5K_QCU_STS_FRMPENDCNT: c_uint = 0x00000003	/* Frames pending counter */;
pub const AR5K_QCU_STS_CBREXPCNT: c_uint = 0x0000ff00	/* CBR expired counter */;

//
// QCU ready time shutdown register
//
pub const AR5K_QCU_RDYTIMESHDN: c_uint = 0x0a40;
pub const AR5K_QCU_RDYTIMESHDN_M: c_uint = 0x000003ff;
//
// QCU compression buffer base registers [5212+]
//
pub const AR5K_QCU_CBB_SELECT: c_uint = 0x0b00;
pub const AR5K_QCU_CBB_ADDR: c_uint = 0x0b04;
pub const AR5K_QCU_CBB_ADDR_S: c_int = 9;
//
// QCU compression buffer configuration register [5212+]
// (buffer size)
//
pub const AR5K_QCU_CBCFG: c_uint = 0x0b08;
//
// Distributed Coordination Function (DCF) control unit (DCU)
// registers [5211+]
//
// These registers control the various characteristics of each queue
// for 802.11e (WME) compatibility so they go together with
// QCU registers in pairs. For each queue we have a QCU mask register,
// (0x1000 - 0x102c), a local-IFS settings register (0x1040 - 0x106c),
// a retry limit register (0x1080 - 0x10ac), a channel time register
// (0x10c0 - 0x10ec), a misc-settings register (0x1100 - 0x112c) and
// a sequence number register (0x1140 - 0x116c). It seems that "global"
// registers here affect all queues (see use of DCU_GBL_IFS_SLOT in ar5k).
// We use the same macros here for easier register access.
//
// DCU QCU mask registers
//
pub const AR5K_DCU_QCUMASK_BASE: c_uint = 0x1000		/* Register Address -Queue0 DCU_QCUMASK */;
pub const AR5K_DCU_QCUMASK_M: c_uint = 0x000003ff;

//
// DCU local Inter Frame Space settings register
//
pub const AR5K_DCU_LCL_IFS_BASE: c_uint = 0x1040			/* Register Address -Queue0 DCU_LCL_IFS */;
pub const AR5K_DCU_LCL_IFS_CW_MIN: c_uint = 0x000003ff	/* Minimum Contention Window */;
pub const AR5K_DCU_LCL_IFS_CW_MIN_S: c_int = 0;
pub const AR5K_DCU_LCL_IFS_CW_MAX: c_uint = 0x000ffc00	/* Maximum Contention Window */;
pub const AR5K_DCU_LCL_IFS_CW_MAX_S: c_int = 10;
pub const AR5K_DCU_LCL_IFS_AIFS: c_uint = 0x0ff00000	/* Arbitrated Interframe Space */;
pub const AR5K_DCU_LCL_IFS_AIFS_S: c_int = 20;
pub const AR5K_DCU_LCL_IFS_AIFS_MAX: c_uint = 0xfc		/* Anything above that can cause DCU to hang */;

//
// DCU retry limit registers
// all these fields don't allow zero values
//
pub const AR5K_DCU_RETRY_LMT_BASE: c_uint = 0x1080			/* Register Address -Queue0 DCU_RETRY_LMT */;
pub const AR5K_DCU_RETRY_LMT_RTS: c_uint = 0x0000000f	/* RTS failure limit. Transmission fails if no CTS is received for this number of times */;
pub const AR5K_DCU_RETRY_LMT_RTS_S: c_int = 0;
pub const AR5K_DCU_RETRY_LMT_STA_RTS: c_uint = 0x00003f00	/* STA RTS failure limit. If exceeded CW reset */;
pub const AR5K_DCU_RETRY_LMT_STA_RTS_S: c_int = 8;
pub const AR5K_DCU_RETRY_LMT_STA_DATA: c_uint = 0x000fc000	/* STA data failure limit. If exceeded CW reset. */;
pub const AR5K_DCU_RETRY_LMT_STA_DATA_S: c_int = 14;

//
// DCU channel time registers
//
pub const AR5K_DCU_CHAN_TIME_BASE: c_uint = 0x10c0			/* Register Address -Queue0 DCU_CHAN_TIME */;
pub const AR5K_DCU_CHAN_TIME_DUR: c_uint = 0x000fffff	/* Channel time duration */;
pub const AR5K_DCU_CHAN_TIME_DUR_S: c_int = 0;
pub const AR5K_DCU_CHAN_TIME_ENABLE: c_uint = 0x00100000	/* Enable channel time */;

//
// DCU misc registers [5211+]
//
// Note: Arbiter lockout control controls the
// behaviour on low priority queues when we have multiple queues
// with pending frames. Intra-frame lockout means we wait until
// the queue's current frame transmits (with post frame backoff and bursting)
// before we transmit anything else and global lockout means we
// wait for the whole queue to finish before higher priority queues
// can transmit (this is used on beacon and CAB queues).
// No lockout means there is no special handling.
//
pub const AR5K_DCU_MISC_BASE: c_uint = 0x1100			/* Register Address -Queue0 DCU_MISC */;
pub const AR5K_DCU_MISC_BACKOFF: c_uint = 0x0000003f	/* Mask for backoff threshold */;
pub const AR5K_DCU_MISC_ETS_RTS_POL: c_uint = 0x00000040	/* End of transmission series;
pub const AR5K_DCU_MISC_ETS_CW_POL: c_uint = 0x00000080	/* End of transmission series;
pub const AR5K_DCU_MISC_FRAG_WAIT: c_uint = 0x00000100	/* Wait for next fragment */;
pub const AR5K_DCU_MISC_BACKOFF_FRAG: c_uint = 0x00000200	/* Enable backoff while bursting */;
pub const AR5K_DCU_MISC_HCFPOLL_ENABLE: c_uint = 0x00000800	/* CF - Poll enable */;
pub const AR5K_DCU_MISC_BACKOFF_PERSIST: c_uint = 0x00001000	/* Persistent backoff */;
pub const AR5K_DCU_MISC_FRMPRFTCH_ENABLE: c_uint = 0x00002000	/* Enable frame pre-fetch */;
pub const AR5K_DCU_MISC_VIRTCOL: c_uint = 0x0000c000	/* Mask for Virtual Collision (?) */;
pub const AR5K_DCU_MISC_VIRTCOL_NORMAL: c_int = 0;
pub const AR5K_DCU_MISC_VIRTCOL_IGNORE: c_int = 1;
pub const AR5K_DCU_MISC_BCN_ENABLE: c_uint = 0x00010000	/* Enable Beacon use */;
pub const AR5K_DCU_MISC_ARBLOCK_CTL: c_uint = 0x00060000	/* Arbiter lockout control mask */;
pub const AR5K_DCU_MISC_ARBLOCK_CTL_S: c_int = 17;

pub const AR5K_DCU_MISC_ARBLOCK_IGNORE: c_uint = 0x00080000	/* Ignore Arbiter lockout */;
pub const AR5K_DCU_MISC_SEQ_NUM_INCR_DIS: c_uint = 0x00100000	/* Disable sequence number increment */;
pub const AR5K_DCU_MISC_POST_FR_BKOFF_DIS: c_uint = 0x00200000	/* Disable post-frame backoff */;
pub const AR5K_DCU_MISC_VIRT_COLL_POLICY: c_uint = 0x00400000	/* Virtual Collision cw policy */;
pub const AR5K_DCU_MISC_BLOWN_IFS_POLICY: c_uint = 0x00800000	/* Blown IFS policy (?) */;
pub const AR5K_DCU_MISC_SEQNUM_CTL: c_uint = 0x01000000	/* Sequence number control (?) */;

//
// DCU frame sequence number registers
//
pub const AR5K_DCU_SEQNUM_BASE: c_uint = 0x1140;
pub const AR5K_DCU_SEQNUM_M: c_uint = 0x00000fff;

//
// DCU global IFS SIFS register
//
pub const AR5K_DCU_GBL_IFS_SIFS: c_uint = 0x1030;
pub const AR5K_DCU_GBL_IFS_SIFS_M: c_uint = 0x0000ffff;
//
// DCU global IFS slot interval register
//
pub const AR5K_DCU_GBL_IFS_SLOT: c_uint = 0x1070;
pub const AR5K_DCU_GBL_IFS_SLOT_M: c_uint = 0x0000ffff;
//
// DCU global IFS EIFS register
//
pub const AR5K_DCU_GBL_IFS_EIFS: c_uint = 0x10b0;
pub const AR5K_DCU_GBL_IFS_EIFS_M: c_uint = 0x0000ffff;
//
// DCU global IFS misc register
//
// LFSR stands for Linear Feedback Shift Register
// and it's used for generating pseudo-random
// number sequences.
//
// (If i understand correctly, random numbers are
// used for idle sensing -multiplied with cwmin/max etc-)
//
pub const AR5K_DCU_GBL_IFS_MISC: c_uint = 0x10f0			/* Register Address */;
pub const AR5K_DCU_GBL_IFS_MISC_LFSR_SLICE: c_uint = 0x00000007	/* LFSR Slice Select */;
pub const AR5K_DCU_GBL_IFS_MISC_TURBO_MODE: c_uint = 0x00000008	/* Turbo mode */;
pub const AR5K_DCU_GBL_IFS_MISC_SIFS_DUR_USEC: c_uint = 0x000003f0	/* SIFS Duration mask */;
pub const AR5K_DCU_GBL_IFS_MISC_SIFS_DUR_USEC_S: c_int = 4;
pub const AR5K_DCU_GBL_IFS_MISC_USEC_DUR: c_uint = 0x000ffc00	/* USEC Duration mask */;
pub const AR5K_DCU_GBL_IFS_MISC_USEC_DUR_S: c_int = 10;
pub const AR5K_DCU_GBL_IFS_MISC_DCU_ARB_DELAY: c_uint = 0x00300000	/* DCU Arbiter delay mask */;
pub const AR5K_DCU_GBL_IFS_MISC_SIFS_CNT_RST: c_uint = 0x00400000	/* SIFS cnt reset policy (?) */;
pub const AR5K_DCU_GBL_IFS_MISC_AIFS_CNT_RST: c_uint = 0x00800000	/* AIFS cnt reset policy (?) */;
pub const AR5K_DCU_GBL_IFS_MISC_RND_LFSR_SL_DIS: c_uint = 0x01000000	/* Disable random LFSR slice */;
//
// DCU frame prefetch control register
//
pub const AR5K_DCU_FP: c_uint = 0x1230			/* Register Address */;
pub const AR5K_DCU_FP_NOBURST_DCU_EN: c_uint = 0x00000001	/* Enable non-burst prefetch on DCU (?) */;
pub const AR5K_DCU_FP_NOBURST_EN: c_uint = 0x00000010	/* Enable non-burst prefetch (?) */;
pub const AR5K_DCU_FP_BURST_DCU_EN: c_uint = 0x00000020	/* Enable burst prefetch on DCU (?) */;
//
// DCU transmit pause control/status register
//
pub const AR5K_DCU_TXP: c_uint = 0x1270			/* Register Address */;
pub const AR5K_DCU_TXP_M: c_uint = 0x000003ff	/* Tx pause mask */;
pub const AR5K_DCU_TXP_STATUS: c_uint = 0x00010000	/* Tx pause status */;
//
// DCU transmit filter table 0 (32 entries)
// each entry contains a 32bit slice of the
// 128bit tx filter for each DCU (4 slices per DCU)
//
pub const AR5K_DCU_TX_FILTER_0_BASE: c_uint = 0x1038;

//
// DCU transmit filter table 1 (16 entries)
//
pub const AR5K_DCU_TX_FILTER_1_BASE: c_uint = 0x103c;

//
// DCU clear transmit filter register
//
pub const AR5K_DCU_TX_FILTER_CLR: c_uint = 0x143c;
//
// DCU set transmit filter register
//
pub const AR5K_DCU_TX_FILTER_SET: c_uint = 0x147c;
//
// Reset control register
//
pub const AR5K_RESET_CTL: c_uint = 0x4000			/* Register Address */;
pub const AR5K_RESET_CTL_PCU: c_uint = 0x00000001	/* Protocol Control Unit reset */;
pub const AR5K_RESET_CTL_DMA: c_uint = 0x00000002	/* DMA (Rx/Tx) reset [5210] */;
pub const AR5K_RESET_CTL_BASEBAND: c_uint = 0x00000002	/* Baseband reset [5211+] */;
pub const AR5K_RESET_CTL_MAC: c_uint = 0x00000004	/* MAC reset (PCU+Baseband ?) [5210] */;
pub const AR5K_RESET_CTL_PHY: c_uint = 0x00000008	/* PHY reset [5210] */;
pub const AR5K_RESET_CTL_PCI: c_uint = 0x00000010	/* PCI Core reset (interrupts etc) */;
//
// Sleep control register
//
pub const AR5K_SLEEP_CTL: c_uint = 0x4004			/* Register Address */;
pub const AR5K_SLEEP_CTL_SLDUR: c_uint = 0x0000ffff	/* Sleep duration mask */;
pub const AR5K_SLEEP_CTL_SLDUR_S: c_int = 0;
pub const AR5K_SLEEP_CTL_SLE: c_uint = 0x00030000	/* Sleep enable mask */;
pub const AR5K_SLEEP_CTL_SLE_S: c_int = 16;
pub const AR5K_SLEEP_CTL_SLE_WAKE: c_uint = 0x00000000	/* Force chip awake */;
pub const AR5K_SLEEP_CTL_SLE_SLP: c_uint = 0x00010000	/* Force chip sleep */;
pub const AR5K_SLEEP_CTL_SLE_ALLOW: c_uint = 0x00020000	/* Normal sleep policy */;
pub const AR5K_SLEEP_CTL_SLE_UNITS: c_uint = 0x00000008	/* [5211+] */;
pub const AR5K_SLEEP_CTL_DUR_TIM_POL: c_uint = 0x00040000	/* Sleep duration timing policy */;
pub const AR5K_SLEEP_CTL_DUR_WRITE_POL: c_uint = 0x00080000	/* Sleep duration write policy */;
pub const AR5K_SLEEP_CTL_SLE_POL: c_uint = 0x00100000	/* Sleep policy mode */;
//
// Interrupt pending register
//
pub const AR5K_INTPEND: c_uint = 0x4008;
pub const AR5K_INTPEND_M: c_uint = 0x00000001;
//
// Sleep force register
//
pub const AR5K_SFR: c_uint = 0x400c;
pub const AR5K_SFR_EN: c_uint = 0x00000001;
//
// PCI configuration register
// TODO: Fix LED stuff
//
pub const AR5K_PCICFG: c_uint = 0x4010			/* Register Address */;
pub const AR5K_PCICFG_EEAE: c_uint = 0x00000001	/* Eeprom access enable [5210] */;
pub const AR5K_PCICFG_SLEEP_CLOCK_EN: c_uint = 0x00000002	/* Enable sleep clock */;
pub const AR5K_PCICFG_CLKRUNEN: c_uint = 0x00000004	/* CLKRUN enable [5211+] */;
pub const AR5K_PCICFG_EESIZE: c_uint = 0x00000018	/* Mask for EEPROM size [5211+] */;
pub const AR5K_PCICFG_EESIZE_S: c_int = 3;

pub const AR5K_PCICFG_LED: c_uint = 0x00000060	/* Led status [5211+] */;
pub const AR5K_PCICFG_LED_NONE: c_uint = 0x00000000	/* Default [5211+] */;
pub const AR5K_PCICFG_LED_PEND: c_uint = 0x00000020	/* Scan / Auth pending */;
pub const AR5K_PCICFG_LED_ASSOC: c_uint = 0x00000040	/* Associated */;
pub const AR5K_PCICFG_BUS_SEL: c_uint = 0x00000380	/* Mask for "bus select" [5211+] (?) */;
pub const AR5K_PCICFG_CBEFIX_DIS: c_uint = 0x00000400	/* Disable CBE fix */;
pub const AR5K_PCICFG_SL_INTEN: c_uint = 0x00000800	/* Enable interrupts when asleep */;
pub const AR5K_PCICFG_LED_BCTL: c_uint = 0x00001000	/* Led blink (?) [5210] */;
pub const AR5K_PCICFG_RETRY_FIX: c_uint = 0x00001000	/* Enable pci core retry fix */;
pub const AR5K_PCICFG_SL_INPEN: c_uint = 0x00002000	/* Sleep even with pending interrupts*/;
pub const AR5K_PCICFG_SPWR_DN: c_uint = 0x00010000	/* Mask for power status */;
pub const AR5K_PCICFG_LEDMODE: c_uint = 0x000e0000	/* Ledmode [5211+] */;
pub const AR5K_PCICFG_LEDMODE_PROP: c_uint = 0x00000000	/* Blink on standard traffic [5211+] */;
pub const AR5K_PCICFG_LEDMODE_PROM: c_uint = 0x00020000	/* Default mode (blink on any traffic) [5211+] */;
pub const AR5K_PCICFG_LEDMODE_PWR: c_uint = 0x00040000	/* Some other blinking mode  (?) [5211+] */;
pub const AR5K_PCICFG_LEDMODE_RAND: c_uint = 0x00060000	/* Random blinking (?) [5211+] */;
pub const AR5K_PCICFG_LEDBLINK: c_uint = 0x00700000	/* Led blink rate */;
pub const AR5K_PCICFG_LEDBLINK_S: c_int = 20;
pub const AR5K_PCICFG_LEDSLOW: c_uint = 0x00800000	/* Slowest led blink rate [5211+] */;

pub const AR5K_PCICFG_SLEEP_CLOCK_RATE: c_uint = 0x03000000	/* Sleep clock rate */;
pub const AR5K_PCICFG_SLEEP_CLOCK_RATE_S: c_int = 24;
//
// "General Purpose Input/Output" (GPIO) control register
//
// I'm not sure about this but after looking at the code
// for all chipsets here is what i got.
//
// We have 6 GPIOs (pins), each GPIO has 4 modes (2 bits)
// Mode 0 -> always input
// Mode 1 -> output when GPIODO for this GPIO is set to 0
// Mode 2 -> output when GPIODO for this GPIO is set to 1
// Mode 3 -> always output
//
// For more infos check out get_gpio/set_gpio and
// set_gpio_input/set_gpio_output functs.
// For more infos on gpio interrupt check out set_gpio_intr.
//
pub const AR5K_NUM_GPIO: c_int = 6;
pub const AR5K_GPIOCR: c_uint = 0x4014				/* Register Address */;
pub const AR5K_GPIOCR_INT_ENA: c_uint = 0x00008000		/* Enable GPIO interrupt */;
pub const AR5K_GPIOCR_INT_SELL: c_uint = 0x00000000		/* Generate interrupt when pin is low */;
pub const AR5K_GPIOCR_INT_SELH: c_uint = 0x00010000		/* Generate interrupt when pin is high */;

//
// "General Purpose Input/Output" (GPIO) data output register
//
pub const AR5K_GPIODO: c_uint = 0x4018;
//
// "General Purpose Input/Output" (GPIO) data input register
//
pub const AR5K_GPIODI: c_uint = 0x401c;
pub const AR5K_GPIODI_M: c_uint = 0x0000002f;
//
// Silicon revision register
//
pub const AR5K_SREV: c_uint = 0x4020			/* Register Address */;
pub const AR5K_SREV_REV: c_uint = 0x0000000f	/* Mask for revision */;
pub const AR5K_SREV_REV_S: c_int = 0;
pub const AR5K_SREV_VER: c_uint = 0x000000ff	/* Mask for version */;
pub const AR5K_SREV_VER_S: c_int = 4;
//
// TXE write posting register
//
pub const AR5K_TXEPOST: c_uint = 0x4028;
//
// QCU sleep mask
//
pub const AR5K_QCU_SLEEP_MASK: c_uint = 0x402c;
// 0x4068 is compression buffer configuration
// register on 5414 and pm configuration register
// on 5424 and newer pci-e chips.
//
// Compression buffer configuration
// register (enable/disable) [5414]
//
pub const AR5K_5414_CBCFG: c_uint = 0x4068;
pub const AR5K_5414_CBCFG_BUF_DIS: c_uint = 0x10	/* Disable buffer */;
//
// PCI-E Power management configuration
// and status register [5424+]
//
pub const AR5K_PCIE_PM_CTL: c_uint = 0x4068			/* Register address */;
// Only 5424
pub const AR5K_PCIE_PM_CTL_L1_WHEN_D2: c_uint = 0x00000001	/* enable PCIe core enter L1;
pub const AR5K_PCIE_PM_CTL_L0_L0S_CLEAR: c_uint = 0x00000002	/* Clear L0 and L0S counters */;
pub const AR5K_PCIE_PM_CTL_L0_L0S_EN: c_uint = 0x00000004	/* Start L0 nd L0S counters */;
pub const AR5K_PCIE_PM_CTL_LDRESET_EN: c_uint = 0x00000008	/* Enable reset when link goes;
// Wake On Wireless
pub const AR5K_PCIE_PM_CTL_PME_EN: c_uint = 0x00000010	/* PME Enable */;
pub const AR5K_PCIE_PM_CTL_AUX_PWR_DET: c_uint = 0x00000020	/* Aux power detect */;
pub const AR5K_PCIE_PM_CTL_PME_CLEAR: c_uint = 0x00000040	/* Clear PME */;
pub const AR5K_PCIE_PM_CTL_PSM_D0: c_uint = 0x00000080;
pub const AR5K_PCIE_PM_CTL_PSM_D1: c_uint = 0x00000100;
pub const AR5K_PCIE_PM_CTL_PSM_D2: c_uint = 0x00000200;
pub const AR5K_PCIE_PM_CTL_PSM_D3: c_uint = 0x00000400;
//
// PCI-E Workaround enable register
//
pub const AR5K_PCIE_WAEN: c_uint = 0x407c;
//
// PCI-E Serializer/Deserializer
// registers
//
pub const AR5K_PCIE_SERDES: c_uint = 0x4080;
pub const AR5K_PCIE_SERDES_RESET: c_uint = 0x4084;
// ====EEPROM REGISTERS====
//
// EEPROM access registers
//
// Here we got a difference between 5210/5211-12
// read data register for 5210 is at 0x6800 and
// status register is at 0x6c00. There is also
// no eeprom command register on 5210 and the
// offsets are different.
//
// To read eeprom data for a specific offset:
// 5210 - enable eeprom access (AR5K_PCICFG_EEAE)
// read AR5K_EEPROM_BASE +(4 * offset)
// check the eeprom status register
// and read eeprom data register.
//
// 5211 - write offset to AR5K_EEPROM_BASE
// 5212   write AR5K_EEPROM_CMD_READ on AR5K_EEPROM_CMD
// check the eeprom status register
// and read eeprom data register.
//
// To write eeprom data for a specific offset:
// 5210 - enable eeprom access (AR5K_PCICFG_EEAE)
// write data to AR5K_EEPROM_BASE +(4 * offset)
// check the eeprom status register
// 5211 - write AR5K_EEPROM_CMD_RESET on AR5K_EEPROM_CMD
// 5212   write offset to AR5K_EEPROM_BASE
// write data to data register
// write AR5K_EEPROM_CMD_WRITE on AR5K_EEPROM_CMD
// check the eeprom status register
//
// For more infos check eeprom_* functs and the ar5k.c
// file posted in madwifi-devel mailing list.
// http://sourceforge.net/mailarchive/message.php?msg_id=8966525
//
pub const AR5K_EEPROM_BASE: c_uint = 0x6000;
//
// EEPROM data register
//
pub const AR5K_EEPROM_DATA_5211: c_uint = 0x6004;
pub const AR5K_EEPROM_DATA_5210: c_uint = 0x6800;

//
// EEPROM command register
//
pub const AR5K_EEPROM_CMD: c_uint = 0x6008			/* Register Address */;
pub const AR5K_EEPROM_CMD_READ: c_uint = 0x00000001	/* EEPROM read */;
pub const AR5K_EEPROM_CMD_WRITE: c_uint = 0x00000002	/* EEPROM write */;
pub const AR5K_EEPROM_CMD_RESET: c_uint = 0x00000004	/* EEPROM reset */;
//
// EEPROM status register
//
pub const AR5K_EEPROM_STAT_5210: c_uint = 0x6c00			/* Register Address [5210] */;
pub const AR5K_EEPROM_STAT_5211: c_uint = 0x600c			/* Register Address [5211+] */;

pub const AR5K_EEPROM_STAT_RDERR: c_uint = 0x00000001	/* EEPROM read failed */;
pub const AR5K_EEPROM_STAT_RDDONE: c_uint = 0x00000002	/* EEPROM read successful */;
pub const AR5K_EEPROM_STAT_WRERR: c_uint = 0x00000004	/* EEPROM write failed */;
pub const AR5K_EEPROM_STAT_WRDONE: c_uint = 0x00000008	/* EEPROM write successful */;
//
// EEPROM config register
//
pub const AR5K_EEPROM_CFG: c_uint = 0x6010			/* Register Address */;
pub const AR5K_EEPROM_CFG_SIZE: c_uint = 0x00000003		/* Size determination override */;
pub const AR5K_EEPROM_CFG_SIZE_AUTO: c_int = 0;
pub const AR5K_EEPROM_CFG_SIZE_4KBIT: c_int = 1;
pub const AR5K_EEPROM_CFG_SIZE_8KBIT: c_int = 2;
pub const AR5K_EEPROM_CFG_SIZE_16KBIT: c_int = 3;
pub const AR5K_EEPROM_CFG_WR_WAIT_DIS: c_uint = 0x00000004	/* Disable write wait */;
pub const AR5K_EEPROM_CFG_CLK_RATE: c_uint = 0x00000018	/* Clock rate */;
pub const AR5K_EEPROM_CFG_CLK_RATE_S: c_int = 3;
pub const AR5K_EEPROM_CFG_CLK_RATE_156KHZ: c_int = 0;
pub const AR5K_EEPROM_CFG_CLK_RATE_312KHZ: c_int = 1;
pub const AR5K_EEPROM_CFG_CLK_RATE_625KHZ: c_int = 2;
pub const AR5K_EEPROM_CFG_PROT_KEY: c_uint = 0x00ffff00      /* Protection key */;
pub const AR5K_EEPROM_CFG_PROT_KEY_S: c_int = 8;
pub const AR5K_EEPROM_CFG_LIND_EN: c_uint = 0x01000000	/* Enable length indicator (?) */;
//
// TODO: Wake On Wireless registers
// Range 0x7000 - 0x7ce0
//
// Protocol Control Unit (PCU) registers
//
// Used for checking initial register writes
// during channel reset (see reset func)
//
pub const AR5K_PCU_MIN: c_uint = 0x8000;
pub const AR5K_PCU_MAX: c_uint = 0x8fff;
//
// First station id register (Lower 32 bits of MAC address)
//
pub const AR5K_STA_ID0: c_uint = 0x8000;
pub const AR5K_STA_ID0_ARRD_L32: c_uint = 0xffffffff;
//
// Second station id register (Upper 16 bits of MAC address + PCU settings)
//
pub const AR5K_STA_ID1: c_uint = 0x8004			/* Register Address */;
pub const AR5K_STA_ID1_ADDR_U16: c_uint = 0x0000ffff	/* Upper 16 bits of MAC address */;
pub const AR5K_STA_ID1_AP: c_uint = 0x00010000	/* Set AP mode */;
pub const AR5K_STA_ID1_ADHOC: c_uint = 0x00020000	/* Set Ad-Hoc mode */;
pub const AR5K_STA_ID1_PWR_SV: c_uint = 0x00040000	/* Power save reporting */;
pub const AR5K_STA_ID1_NO_KEYSRCH: c_uint = 0x00080000	/* No key search */;
pub const AR5K_STA_ID1_NO_PSPOLL: c_uint = 0x00100000	/* No power save polling [5210] */;
pub const AR5K_STA_ID1_PCF_5211: c_uint = 0x00100000	/* Enable PCF on [5211+] */;
pub const AR5K_STA_ID1_PCF_5210: c_uint = 0x00200000	/* Enable PCF on [5210]*/;

pub const AR5K_STA_ID1_DEFAULT_ANTENNA: c_uint = 0x00200000	/* Use default antenna */;
pub const AR5K_STA_ID1_DESC_ANTENNA: c_uint = 0x00400000	/* Update antenna from descriptor */;
pub const AR5K_STA_ID1_RTS_DEF_ANTENNA: c_uint = 0x00800000	/* Use default antenna for RTS */;
pub const AR5K_STA_ID1_ACKCTS_6MB: c_uint = 0x01000000	/* Rate to use for ACK/CTS. 0: highest mandatory rate <= RX rate; 1: 1Mbps in B mode */;
pub const AR5K_STA_ID1_BASE_RATE_11B: c_uint = 0x02000000	/* 802.11b base rate. 0: 1, 2, 5.5 and 11Mbps; 1: 1 and 2Mbps. [5211+] */;
pub const AR5K_STA_ID1_SELFGEN_DEF_ANT: c_uint = 0x04000000	/* Use def. antenna for self generated frames */;
pub const AR5K_STA_ID1_CRYPT_MIC_EN: c_uint = 0x08000000	/* Enable MIC */;
pub const AR5K_STA_ID1_KEYSRCH_MODE: c_uint = 0x10000000	/* Look up key when key id != 0 */;
pub const AR5K_STA_ID1_PRESERVE_SEQ_NUM: c_uint = 0x20000000	/* Preserve sequence number */;
pub const AR5K_STA_ID1_CBCIV_ENDIAN: c_uint = 0x40000000	/* ??? */;
pub const AR5K_STA_ID1_KEYSRCH_MCAST: c_uint = 0x80000000	/* Do key cache search for mcast frames */;

//
// First BSSID register (MAC address, lower 32bits)
//
pub const AR5K_BSS_ID0: c_uint = 0x8008;
//
// Second BSSID register (MAC address in upper 16 bits)
//
// AID: Association ID
//
pub const AR5K_BSS_ID1: c_uint = 0x800c;
pub const AR5K_BSS_ID1_AID: c_uint = 0xffff0000;
pub const AR5K_BSS_ID1_AID_S: c_int = 16;
//
// Backoff slot time register
//
pub const AR5K_SLOT_TIME: c_uint = 0x8010;
//
// ACK/CTS timeout register
//
pub const AR5K_TIME_OUT: c_uint = 0x8014			/* Register Address */;
pub const AR5K_TIME_OUT_ACK: c_uint = 0x00001fff	/* ACK timeout mask */;
pub const AR5K_TIME_OUT_ACK_S: c_int = 0;
pub const AR5K_TIME_OUT_CTS: c_uint = 0x1fff0000	/* CTS timeout mask */;
pub const AR5K_TIME_OUT_CTS_S: c_int = 16;
//
// RSSI threshold register
//
pub const AR5K_RSSI_THR: c_uint = 0x8018		/* Register Address */;
pub const AR5K_RSSI_THR_M: c_uint = 0x000000ff	/* Mask for RSSI threshold [5211+] */;
pub const AR5K_RSSI_THR_BMISS_5210: c_uint = 0x00000700	/* Mask for Beacon Missed threshold [5210] */;
pub const AR5K_RSSI_THR_BMISS_5210_S: c_int = 8;
pub const AR5K_RSSI_THR_BMISS_5211: c_uint = 0x0000ff00	/* Mask for Beacon Missed threshold [5211+] */;
pub const AR5K_RSSI_THR_BMISS_5211_S: c_int = 8;

pub const AR5K_RSSI_THR_BMISS_S: c_int = 8;
//
// 5210 has more PCU registers because there is no QCU/DCU
// so queue parameters are set here, this way a lot common
// registers have different address for 5210. To make things
// easier we define a macro based on ah->ah_version for common
// registers with different addresses and common flags.
//
// Retry limit register
//
// Retry limit register for 5210 (no QCU/DCU so it's done in PCU)
//
pub const AR5K_NODCU_RETRY_LMT: c_uint = 0x801c			/* Register Address */;
pub const AR5K_NODCU_RETRY_LMT_SH_RETRY: c_uint = 0x0000000f	/* Short retry limit mask */;
pub const AR5K_NODCU_RETRY_LMT_SH_RETRY_S: c_int = 0;
pub const AR5K_NODCU_RETRY_LMT_LG_RETRY: c_uint = 0x000000f0	/* Long retry mask */;
pub const AR5K_NODCU_RETRY_LMT_LG_RETRY_S: c_int = 4;
pub const AR5K_NODCU_RETRY_LMT_SSH_RETRY: c_uint = 0x00003f00	/* Station short retry limit mask */;
pub const AR5K_NODCU_RETRY_LMT_SSH_RETRY_S: c_int = 8;
pub const AR5K_NODCU_RETRY_LMT_SLG_RETRY: c_uint = 0x000fc000	/* Station long retry limit mask */;
pub const AR5K_NODCU_RETRY_LMT_SLG_RETRY_S: c_int = 14;
pub const AR5K_NODCU_RETRY_LMT_CW_MIN: c_uint = 0x3ff00000	/* Minimum contention window mask */;
pub const AR5K_NODCU_RETRY_LMT_CW_MIN_S: c_int = 20;
//
// Transmit latency register
//
pub const AR5K_USEC_5210: c_uint = 0x8020			/* Register Address [5210] */;
pub const AR5K_USEC_5211: c_uint = 0x801c			/* Register Address [5211+] */;

pub const AR5K_USEC_1: c_uint = 0x0000007f	/* clock cycles for 1us */;
pub const AR5K_USEC_1_S: c_int = 0;
pub const AR5K_USEC_32: c_uint = 0x00003f80	/* clock cycles for 1us while on 32MHz clock */;
pub const AR5K_USEC_32_S: c_int = 7;
pub const AR5K_USEC_TX_LATENCY_5211: c_uint = 0x007fc000;
pub const AR5K_USEC_TX_LATENCY_5211_S: c_int = 14;
pub const AR5K_USEC_RX_LATENCY_5211: c_uint = 0x1f800000;
pub const AR5K_USEC_RX_LATENCY_5211_S: c_int = 23;
pub const AR5K_USEC_TX_LATENCY_5210: c_uint = 0x000fc000	/* also for 5311 */;
pub const AR5K_USEC_TX_LATENCY_5210_S: c_int = 14;
pub const AR5K_USEC_RX_LATENCY_5210: c_uint = 0x03f00000	/* also for 5311 */;
pub const AR5K_USEC_RX_LATENCY_5210_S: c_int = 20;
//
// PCU beacon control register
//
pub const AR5K_BEACON_5210: c_uint = 0x8024			/*Register Address [5210] */;
pub const AR5K_BEACON_5211: c_uint = 0x8020			/*Register Address [5211+] */;

pub const AR5K_BEACON_PERIOD: c_uint = 0x0000ffff	/* Mask for beacon period */;
pub const AR5K_BEACON_PERIOD_S: c_int = 0;
pub const AR5K_BEACON_TIM: c_uint = 0x007f0000	/* Mask for TIM offset */;
pub const AR5K_BEACON_TIM_S: c_int = 16;
pub const AR5K_BEACON_ENABLE: c_uint = 0x00800000	/* Enable beacons */;
pub const AR5K_BEACON_RESET_TSF: c_uint = 0x01000000	/* Force TSF reset */;
//
// CFP period register
//
pub const AR5K_CFP_PERIOD_5210: c_uint = 0x8028;
pub const AR5K_CFP_PERIOD_5211: c_uint = 0x8024;

//
// Next beacon time register
//
pub const AR5K_TIMER0_5210: c_uint = 0x802c;
pub const AR5K_TIMER0_5211: c_uint = 0x8028;

//
// Next DMA beacon alert register
//
pub const AR5K_TIMER1_5210: c_uint = 0x8030;
pub const AR5K_TIMER1_5211: c_uint = 0x802c;

//
// Next software beacon alert register
//
pub const AR5K_TIMER2_5210: c_uint = 0x8034;
pub const AR5K_TIMER2_5211: c_uint = 0x8030;

//
// Next ATIM window time register
//
pub const AR5K_TIMER3_5210: c_uint = 0x8038;
pub const AR5K_TIMER3_5211: c_uint = 0x8034;

//
// 5210 First inter frame spacing register (IFS)
//
pub const AR5K_IFS0: c_uint = 0x8040;
pub const AR5K_IFS0_SIFS: c_uint = 0x000007ff;
pub const AR5K_IFS0_SIFS_S: c_int = 0;
pub const AR5K_IFS0_DIFS: c_uint = 0x007ff800;
pub const AR5K_IFS0_DIFS_S: c_int = 11;
//
// 5210 Second inter frame spacing register (IFS)
//
pub const AR5K_IFS1: c_uint = 0x8044;
pub const AR5K_IFS1_PIFS: c_uint = 0x00000fff;
pub const AR5K_IFS1_PIFS_S: c_int = 0;
pub const AR5K_IFS1_EIFS: c_uint = 0x03fff000;
pub const AR5K_IFS1_EIFS_S: c_int = 12;
pub const AR5K_IFS1_CS_EN: c_uint = 0x04000000;
pub const AR5K_IFS1_CS_EN_S: c_int = 26;
//
// CFP duration register
//
pub const AR5K_CFP_DUR_5210: c_uint = 0x8048;
pub const AR5K_CFP_DUR_5211: c_uint = 0x8038;

//
// Receive filter register
//
pub const AR5K_RX_FILTER_5210: c_uint = 0x804c			/* Register Address [5210] */;
pub const AR5K_RX_FILTER_5211: c_uint = 0x803c			/* Register Address [5211+] */;

pub const AR5K_RX_FILTER_UCAST: c_uint = 0x00000001	/* Don't filter unicast frames */;
pub const AR5K_RX_FILTER_MCAST: c_uint = 0x00000002	/* Don't filter multicast frames */;
pub const AR5K_RX_FILTER_BCAST: c_uint = 0x00000004	/* Don't filter broadcast frames */;
pub const AR5K_RX_FILTER_CONTROL: c_uint = 0x00000008	/* Don't filter control frames */;
pub const AR5K_RX_FILTER_BEACON: c_uint = 0x00000010	/* Don't filter beacon frames */;
pub const AR5K_RX_FILTER_PROM: c_uint = 0x00000020	/* Set promiscuous mode */;
pub const AR5K_RX_FILTER_XRPOLL: c_uint = 0x00000040	/* Don't filter XR poll frame [5212+] */;
pub const AR5K_RX_FILTER_PROBEREQ: c_uint = 0x00000080	/* Don't filter probe requests [5212+] */;
pub const AR5K_RX_FILTER_PHYERR_5212: c_uint = 0x00000100	/* Don't filter phy errors [5212+] */;
pub const AR5K_RX_FILTER_RADARERR_5212: c_uint = 0x00000200	/* Don't filter phy radar errors [5212+] */;
pub const AR5K_RX_FILTER_PHYERR_5211: c_uint = 0x00000040	/* [5211] */;
pub const AR5K_RX_FILTER_RADARERR_5211: c_uint = 0x00000080	/* [5211] */;

//
// Multicast filter register (lower 32 bits)
//
pub const AR5K_MCAST_FILTER0_5210: c_uint = 0x8050;
pub const AR5K_MCAST_FILTER0_5211: c_uint = 0x8040;

//
// Multicast filter register (higher 16 bits)
//
pub const AR5K_MCAST_FILTER1_5210: c_uint = 0x8054;
pub const AR5K_MCAST_FILTER1_5211: c_uint = 0x8044;

//
// Transmit mask register (lower 32 bits) [5210]
//
pub const AR5K_TX_MASK0: c_uint = 0x8058;
//
// Transmit mask register (higher 16 bits) [5210]
//
pub const AR5K_TX_MASK1: c_uint = 0x805c;
//
// Clear transmit mask [5210]
//
pub const AR5K_CLR_TMASK: c_uint = 0x8060;
//
// Trigger level register (before transmission) [5210]
//
pub const AR5K_TRIG_LVL: c_uint = 0x8064;
//
// PCU Diagnostic register
//
// Used for tweaking/diagnostics.
//
pub const AR5K_DIAG_SW_5210: c_uint = 0x8068			/* Register Address [5210] */;
pub const AR5K_DIAG_SW_5211: c_uint = 0x8048			/* Register Address [5211+] */;

pub const AR5K_DIAG_SW_DIS_WEP_ACK: c_uint = 0x00000001	/* Disable ACKs if WEP key is invalid */;
pub const AR5K_DIAG_SW_DIS_ACK: c_uint = 0x00000002	/* Disable ACKs */;
pub const AR5K_DIAG_SW_DIS_CTS: c_uint = 0x00000004	/* Disable CTSs */;
pub const AR5K_DIAG_SW_DIS_ENC: c_uint = 0x00000008	/* Disable HW encryption */;
pub const AR5K_DIAG_SW_DIS_DEC: c_uint = 0x00000010	/* Disable HW decryption */;
pub const AR5K_DIAG_SW_DIS_TX_5210: c_uint = 0x00000020	/* Disable transmit [5210] */;
pub const AR5K_DIAG_SW_DIS_RX_5210: c_uint = 0x00000040	/* Disable receive */;
pub const AR5K_DIAG_SW_DIS_RX_5211: c_uint = 0x00000020;

pub const AR5K_DIAG_SW_LOOP_BACK_5210: c_uint = 0x00000080	/* TX Data Loopback (i guess it goes with DIS_TX) [5210] */;
pub const AR5K_DIAG_SW_LOOP_BACK_5211: c_uint = 0x00000040;

pub const AR5K_DIAG_SW_CORR_FCS_5210: c_uint = 0x00000100	/* Generate invalid TX FCS */;
pub const AR5K_DIAG_SW_CORR_FCS_5211: c_uint = 0x00000080;

pub const AR5K_DIAG_SW_CHAN_INFO_5210: c_uint = 0x00000200	/* Add 56 bytes of channel info before the frame data in the RX buffer */;
pub const AR5K_DIAG_SW_CHAN_INFO_5211: c_uint = 0x00000100;

pub const AR5K_DIAG_SW_EN_SCRAM_SEED_5210: c_uint = 0x00000400	/* Enable fixed scrambler seed */;
pub const AR5K_DIAG_SW_EN_SCRAM_SEED_5211: c_uint = 0x00000200;

pub const AR5K_DIAG_SW_ECO_ENABLE: c_uint = 0x00000400	/* [5211+] */;
pub const AR5K_DIAG_SW_SCVRAM_SEED: c_uint = 0x0003f800	/* [5210] */;
pub const AR5K_DIAG_SW_SCRAM_SEED_M: c_uint = 0x0001fc00	/* Scrambler seed mask */;
pub const AR5K_DIAG_SW_SCRAM_SEED_S: c_int = 10;
pub const AR5K_DIAG_SW_DIS_SEQ_INC_5210: c_uint = 0x00040000	/* Disable seqnum increment (?)[5210] */;
pub const AR5K_DIAG_SW_FRAME_NV0_5210: c_uint = 0x00080000;
pub const AR5K_DIAG_SW_FRAME_NV0_5211: c_uint = 0x00020000	/* Accept frames of non-zero protocol number */;

pub const AR5K_DIAG_SW_OBSPT_M: c_uint = 0x000c0000	/* Observation point select (?) */;
pub const AR5K_DIAG_SW_OBSPT_S: c_int = 18;
pub const AR5K_DIAG_SW_RX_CLEAR_HIGH: c_uint = 0x00100000	/* Ignore carrier sense */;
pub const AR5K_DIAG_SW_IGNORE_CARR_SENSE: c_uint = 0x00200000	/* Ignore virtual carrier sense */;
pub const AR5K_DIAG_SW_CHANNEL_IDLE_HIGH: c_uint = 0x00400000	/* Force channel idle high */;
pub const AR5K_DIAG_SW_PHEAR_ME: c_uint = 0x00800000	/* ??? */;
//
// TSF (clock) register (lower 32 bits)
//
pub const AR5K_TSF_L32_5210: c_uint = 0x806c;
pub const AR5K_TSF_L32_5211: c_uint = 0x804c;

//
// TSF (clock) register (higher 32 bits)
//
pub const AR5K_TSF_U32_5210: c_uint = 0x8070;
pub const AR5K_TSF_U32_5211: c_uint = 0x8050;

//
// Last beacon timestamp register (Read Only)
//
pub const AR5K_LAST_TSTP: c_uint = 0x8080;
//
// ADDAC test register [5211+]
//
pub const AR5K_ADDAC_TEST: c_uint = 0x8054			/* Register Address */;
pub const AR5K_ADDAC_TEST_TXCONT: c_uint = 0x00000001	/* Test continuous tx */;
pub const AR5K_ADDAC_TEST_TST_MODE: c_uint = 0x00000002	/* Test mode */;
pub const AR5K_ADDAC_TEST_LOOP_EN: c_uint = 0x00000004	/* Enable loop */;
pub const AR5K_ADDAC_TEST_LOOP_LEN: c_uint = 0x00000008	/* Loop length (field) */;
pub const AR5K_ADDAC_TEST_USE_U8: c_uint = 0x00004000	/* Use upper 8 bits */;
pub const AR5K_ADDAC_TEST_MSB: c_uint = 0x00008000	/* State of MSB */;
pub const AR5K_ADDAC_TEST_TRIG_SEL: c_uint = 0x00010000	/* Trigger select */;
pub const AR5K_ADDAC_TEST_TRIG_PTY: c_uint = 0x00020000	/* Trigger polarity */;
pub const AR5K_ADDAC_TEST_RXCONT: c_uint = 0x00040000	/* Continuous capture */;
pub const AR5K_ADDAC_TEST_CAPTURE: c_uint = 0x00080000	/* Begin capture */;
pub const AR5K_ADDAC_TEST_TST_ARM: c_uint = 0x00100000	/* ARM rx buffer for capture */;
//
// Default antenna register [5211+]
//
pub const AR5K_DEFAULT_ANTENNA: c_uint = 0x8058;
//
// Frame control QoS mask register (?) [5211+]
// (FC_QOS_MASK)
//
pub const AR5K_FRAME_CTL_QOSM: c_uint = 0x805c;
//
// Seq mask register (?) [5211+]
//
pub const AR5K_SEQ_MASK: c_uint = 0x8060;
//
// Retry count register [5210]
//
pub const AR5K_RETRY_CNT: c_uint = 0x8084			/* Register Address [5210] */;
pub const AR5K_RETRY_CNT_SSH: c_uint = 0x0000003f	/* Station short retry count (?) */;
pub const AR5K_RETRY_CNT_SLG: c_uint = 0x00000fc0	/* Station long retry count (?) */;
//
// Back-off status register [5210]
//
pub const AR5K_BACKOFF: c_uint = 0x8088			/* Register Address [5210] */;
pub const AR5K_BACKOFF_CW: c_uint = 0x000003ff	/* Backoff Contention Window (?) */;
pub const AR5K_BACKOFF_CNT: c_uint = 0x03ff0000	/* Backoff count (?) */;
//
// NAV register (current)
//
pub const AR5K_NAV_5210: c_uint = 0x808c;
pub const AR5K_NAV_5211: c_uint = 0x8084;

//
// MIB counters:
//
// max value is 0xc000, if this is reached we get a MIB interrupt.
// they can be controlled via AR5K_MIBC and are cleared on read.
//
// RTS success (MIB counter)
//
pub const AR5K_RTS_OK_5210: c_uint = 0x8090;
pub const AR5K_RTS_OK_5211: c_uint = 0x8088;

//
// RTS failure (MIB counter)
//
pub const AR5K_RTS_FAIL_5210: c_uint = 0x8094;
pub const AR5K_RTS_FAIL_5211: c_uint = 0x808c;

//
// ACK failure (MIB counter)
//
pub const AR5K_ACK_FAIL_5210: c_uint = 0x8098;
pub const AR5K_ACK_FAIL_5211: c_uint = 0x8090;

//
// FCS failure (MIB counter)
//
pub const AR5K_FCS_FAIL_5210: c_uint = 0x809c;
pub const AR5K_FCS_FAIL_5211: c_uint = 0x8094;

//
// Beacon count register
//
pub const AR5K_BEACON_CNT_5210: c_uint = 0x80a0;
pub const AR5K_BEACON_CNT_5211: c_uint = 0x8098;

// ===5212 Specific PCU registers===
//
// Transmit power control register
//
pub const AR5K_TPC: c_uint = 0x80e8;
pub const AR5K_TPC_ACK: c_uint = 0x0000003f	/* ack frames */;
pub const AR5K_TPC_ACK_S: c_int = 0;
pub const AR5K_TPC_CTS: c_uint = 0x00003f00	/* cts frames */;
pub const AR5K_TPC_CTS_S: c_int = 8;
pub const AR5K_TPC_CHIRP: c_uint = 0x003f0000	/* chirp frames */;
pub const AR5K_TPC_CHIRP_S: c_int = 16;
pub const AR5K_TPC_DOPPLER: c_uint = 0x0f000000	/* doppler chirp span */;
pub const AR5K_TPC_DOPPLER_S: c_int = 24;
//
// XR (eXtended Range) mode register
//
pub const AR5K_XRMODE: c_uint = 0x80c0			/* Register Address */;
pub const AR5K_XRMODE_POLL_TYPE_M: c_uint = 0x0000003f	/* Mask for Poll type (?) */;
pub const AR5K_XRMODE_POLL_TYPE_S: c_int = 0;
pub const AR5K_XRMODE_POLL_SUBTYPE_M: c_uint = 0x0000003c	/* Mask for Poll subtype (?) */;
pub const AR5K_XRMODE_POLL_SUBTYPE_S: c_int = 2;
pub const AR5K_XRMODE_POLL_WAIT_ALL: c_uint = 0x00000080	/* Wait for poll */;
pub const AR5K_XRMODE_SIFS_DELAY: c_uint = 0x000fff00	/* Mask for SIFS delay */;
pub const AR5K_XRMODE_FRAME_HOLD_M: c_uint = 0xfff00000	/* Mask for frame hold (?) */;
pub const AR5K_XRMODE_FRAME_HOLD_S: c_int = 20;
//
// XR delay register
//
pub const AR5K_XRDELAY: c_uint = 0x80c4			/* Register Address */;
pub const AR5K_XRDELAY_SLOT_DELAY_M: c_uint = 0x0000ffff	/* Mask for slot delay */;
pub const AR5K_XRDELAY_SLOT_DELAY_S: c_int = 0;
pub const AR5K_XRDELAY_CHIRP_DELAY_M: c_uint = 0xffff0000	/* Mask for CHIRP data delay */;
pub const AR5K_XRDELAY_CHIRP_DELAY_S: c_int = 16;
//
// XR timeout register
//
pub const AR5K_XRTIMEOUT: c_uint = 0x80c8			/* Register Address */;
pub const AR5K_XRTIMEOUT_CHIRP_M: c_uint = 0x0000ffff	/* Mask for CHIRP timeout */;
pub const AR5K_XRTIMEOUT_CHIRP_S: c_int = 0;
pub const AR5K_XRTIMEOUT_POLL_M: c_uint = 0xffff0000	/* Mask for Poll timeout */;
pub const AR5K_XRTIMEOUT_POLL_S: c_int = 16;
//
// XR chirp register
//
pub const AR5K_XRCHIRP: c_uint = 0x80cc			/* Register Address */;
pub const AR5K_XRCHIRP_SEND: c_uint = 0x00000001	/* Send CHIRP */;
pub const AR5K_XRCHIRP_GAP: c_uint = 0xffff0000	/* Mask for CHIRP gap (?) */;
//
// XR stomp register
//
pub const AR5K_XRSTOMP: c_uint = 0x80d0			/* Register Address */;
pub const AR5K_XRSTOMP_TX: c_uint = 0x00000001	/* Stomp Tx (?) */;
pub const AR5K_XRSTOMP_RX: c_uint = 0x00000002	/* Stomp Rx (?) */;
pub const AR5K_XRSTOMP_TX_RSSI: c_uint = 0x00000004	/* Stomp Tx RSSI (?) */;
pub const AR5K_XRSTOMP_TX_BSSID: c_uint = 0x00000008	/* Stomp Tx BSSID (?) */;
pub const AR5K_XRSTOMP_DATA: c_uint = 0x00000010	/* Stomp data (?)*/;
pub const AR5K_XRSTOMP_RSSI_THRES: c_uint = 0x0000ff00	/* Mask for XR RSSI threshold */;
//
// First enhanced sleep register
//
pub const AR5K_SLEEP0: c_uint = 0x80d4			/* Register Address */;
pub const AR5K_SLEEP0_NEXT_DTIM: c_uint = 0x0007ffff	/* Mask for next DTIM (?) */;
pub const AR5K_SLEEP0_NEXT_DTIM_S: c_int = 0;
pub const AR5K_SLEEP0_ASSUME_DTIM: c_uint = 0x00080000	/* Assume DTIM */;
pub const AR5K_SLEEP0_ENH_SLEEP_EN: c_uint = 0x00100000	/* Enable enhanced sleep control */;
pub const AR5K_SLEEP0_CABTO: c_uint = 0xff000000	/* Mask for CAB Time Out */;
pub const AR5K_SLEEP0_CABTO_S: c_int = 24;
//
// Second enhanced sleep register
//
pub const AR5K_SLEEP1: c_uint = 0x80d8			/* Register Address */;
pub const AR5K_SLEEP1_NEXT_TIM: c_uint = 0x0007ffff	/* Mask for next TIM (?) */;
pub const AR5K_SLEEP1_NEXT_TIM_S: c_int = 0;
pub const AR5K_SLEEP1_BEACON_TO: c_uint = 0xff000000	/* Mask for Beacon Time Out */;
pub const AR5K_SLEEP1_BEACON_TO_S: c_int = 24;
//
// Third enhanced sleep register
//
pub const AR5K_SLEEP2: c_uint = 0x80dc			/* Register Address */;
pub const AR5K_SLEEP2_TIM_PER: c_uint = 0x0000ffff	/* Mask for TIM period (?) */;
pub const AR5K_SLEEP2_TIM_PER_S: c_int = 0;
pub const AR5K_SLEEP2_DTIM_PER: c_uint = 0xffff0000	/* Mask for DTIM period (?) */;
pub const AR5K_SLEEP2_DTIM_PER_S: c_int = 16;
//
// TX power control (TPC) register
//
// XXX: PCDAC steps (0.5dBm) or dBm ?
//
pub const AR5K_TXPC: c_uint = 0x80e8			/* Register Address */;
pub const AR5K_TXPC_ACK_M: c_uint = 0x0000003f	/* ACK tx power */;
pub const AR5K_TXPC_ACK_S: c_int = 0;
pub const AR5K_TXPC_CTS_M: c_uint = 0x00003f00	/* CTS tx power */;
pub const AR5K_TXPC_CTS_S: c_int = 8;
pub const AR5K_TXPC_CHIRP_M: c_uint = 0x003f0000	/* CHIRP tx power */;
pub const AR5K_TXPC_CHIRP_S: c_int = 16;
pub const AR5K_TXPC_DOPPLER: c_uint = 0x0f000000	/* Doppler chirp span (?) */;
pub const AR5K_TXPC_DOPPLER_S: c_int = 24;
//
// Profile count registers
//
// These registers can be cleared and frozen with ATH5K_MIBC, but they do not
// generate a MIB interrupt.
// Instead of overflowing, they shift by one bit to the right. All registers
// shift together, i.e. when one reaches the max, all shift at the same time by
// one bit to the right. This way we should always get consistent values.
//
pub const AR5K_PROFCNT_TX: c_uint = 0x80ec	/* Tx count */;
pub const AR5K_PROFCNT_RX: c_uint = 0x80f0	/* Rx count */;
pub const AR5K_PROFCNT_RXCLR: c_uint = 0x80f4	/* Busy count */;
pub const AR5K_PROFCNT_CYCLE: c_uint = 0x80f8	/* Cycle counter */;
//
// Quiet period control registers
//
pub const AR5K_QUIET_CTL1: c_uint = 0x80fc			/* Register Address */;
pub const AR5K_QUIET_CTL1_NEXT_QT_TSF: c_uint = 0x0000ffff	/* Next quiet period TSF (TU) */;
pub const AR5K_QUIET_CTL1_NEXT_QT_TSF_S: c_int = 0;
pub const AR5K_QUIET_CTL1_QT_EN: c_uint = 0x00010000	/* Enable quiet period */;
pub const AR5K_QUIET_CTL1_ACK_CTS_EN: c_uint = 0x00020000	/* Send ACK/CTS during quiet period */;
pub const AR5K_QUIET_CTL2: c_uint = 0x8100			/* Register Address */;
pub const AR5K_QUIET_CTL2_QT_PER: c_uint = 0x0000ffff	/* Mask for quiet period periodicity */;
pub const AR5K_QUIET_CTL2_QT_PER_S: c_int = 0;
pub const AR5K_QUIET_CTL2_QT_DUR: c_uint = 0xffff0000	/* Mask for quiet period duration */;
pub const AR5K_QUIET_CTL2_QT_DUR_S: c_int = 16;
//
// TSF parameter register
//
pub const AR5K_TSF_PARM: c_uint = 0x8104			/* Register Address */;
pub const AR5K_TSF_PARM_INC: c_uint = 0x000000ff	/* Mask for TSF increment */;
pub const AR5K_TSF_PARM_INC_S: c_int = 0;
//
// QoS NOACK policy
//
pub const AR5K_QOS_NOACK: c_uint = 0x8108			/* Register Address */;
pub const AR5K_QOS_NOACK_2BIT_VALUES: c_uint = 0x0000000f	/* ??? */;
pub const AR5K_QOS_NOACK_2BIT_VALUES_S: c_int = 0;
pub const AR5K_QOS_NOACK_BIT_OFFSET: c_uint = 0x00000070	/* ??? */;
pub const AR5K_QOS_NOACK_BIT_OFFSET_S: c_int = 4;
pub const AR5K_QOS_NOACK_BYTE_OFFSET: c_uint = 0x00000180	/* ??? */;
pub const AR5K_QOS_NOACK_BYTE_OFFSET_S: c_int = 7;
//
// PHY error filter register
//
pub const AR5K_PHY_ERR_FIL: c_uint = 0x810c;
pub const AR5K_PHY_ERR_FIL_RADAR: c_uint = 0x00000020	/* Radar signal */;
pub const AR5K_PHY_ERR_FIL_OFDM: c_uint = 0x00020000	/* OFDM false detect (ANI) */;
pub const AR5K_PHY_ERR_FIL_CCK: c_uint = 0x02000000	/* CCK false detect (ANI) */;
//
// XR latency register
//
pub const AR5K_XRLAT_TX: c_uint = 0x8110;
//
// ACK SIFS register
//
pub const AR5K_ACKSIFS: c_uint = 0x8114			/* Register Address */;
pub const AR5K_ACKSIFS_INC: c_uint = 0x00000000	/* ACK SIFS Increment (field) */;
//
// MIC QoS control register (?)
//
pub const AR5K_MIC_QOS_CTL: c_uint = 0x8118			/* Register Address */;

pub const AR5K_MIC_QOS_CTL_MQ_EN: c_uint = 0x00010000	/* Enable MIC QoS */;
//
// MIC QoS select register (?)
//
pub const AR5K_MIC_QOS_SEL: c_uint = 0x811c;

//
// Misc mode control register (?)
//
pub const AR5K_MISC_MODE: c_uint = 0x8120			/* Register Address */;
pub const AR5K_MISC_MODE_FBSSID_MATCH: c_uint = 0x00000001	/* Force BSSID match */;
pub const AR5K_MISC_MODE_ACKSIFS_MEM: c_uint = 0x00000002	/* ACK SIFS memory (?) */;
pub const AR5K_MISC_MODE_COMBINED_MIC: c_uint = 0x00000004	/* use rx/tx MIC key */;
// more bits
//
// OFDM Filter counter
//
pub const AR5K_OFDM_FIL_CNT: c_uint = 0x8124;
//
// CCK Filter counter
//
pub const AR5K_CCK_FIL_CNT: c_uint = 0x8128;
//
// PHY Error Counters (same masks as AR5K_PHY_ERR_FIL)
//
pub const AR5K_PHYERR_CNT1: c_uint = 0x812c;
pub const AR5K_PHYERR_CNT1_MASK: c_uint = 0x8130;
pub const AR5K_PHYERR_CNT2: c_uint = 0x8134;
pub const AR5K_PHYERR_CNT2_MASK: c_uint = 0x8138;
// if the PHY Error Counters reach this maximum, we get MIB interrupts
pub const ATH5K_PHYERR_CNT_MAX: c_uint = 0x00c00000;
//
// TSF Threshold register (?)
//
pub const AR5K_TSF_THRES: c_uint = 0x813c;
//
// TODO: Wake On Wireless registers
// Range: 0x8147 - 0x818c
//
// Rate -> ACK SIFS mapping table (32 entries)
//
pub const AR5K_RATE_ACKSIFS_BASE: c_uint = 0x8680			/* Register Address */;

pub const AR5K_RATE_ACKSIFS_NORMAL: c_uint = 0x00000001	/* Normal SIFS (field) */;
pub const AR5K_RATE_ACKSIFS_TURBO: c_uint = 0x00000400	/* Turbo SIFS (field) */;
//
// Rate -> duration mapping table (32 entries)
//
pub const AR5K_RATE_DUR_BASE: c_uint = 0x8700;

//
// Rate -> db mapping table
// (8 entries, each one has 4 8bit fields)
//
pub const AR5K_RATE2DB_BASE: c_uint = 0x87c0;

//
// db -> Rate mapping table
// (8 entries, each one has 4 8bit fields)
//
pub const AR5K_DB2RATE_BASE: c_uint = 0x87e0;

// ===5212 end===
pub const AR5K_KEYTABLE_SIZE_5210: c_int = 64;
pub const AR5K_KEYTABLE_SIZE_5211: c_int = 128;
// ===PHY REGISTERS===
//
// PHY registers start
//
pub const AR5K_PHY_BASE: c_uint = 0x9800;

//
// TST_2 (Misc config parameters)
//
pub const AR5K_PHY_TST2: c_uint = 0x9800			/* Register Address */;
pub const AR5K_PHY_TST2_TRIG_SEL: c_uint = 0x00000007	/* Trigger select (?)*/;
pub const AR5K_PHY_TST2_TRIG: c_uint = 0x00000010	/* Trigger (?) */;
pub const AR5K_PHY_TST2_CBUS_MODE: c_uint = 0x00000060	/* Cardbus mode (?) */;
pub const AR5K_PHY_TST2_CLK32: c_uint = 0x00000400	/* CLK_OUT is CLK32 (32kHz external) */;
pub const AR5K_PHY_TST2_CHANCOR_DUMP_EN: c_uint = 0x00000800	/* Enable Chancor dump (?) */;
pub const AR5K_PHY_TST2_EVEN_CHANCOR_DUMP: c_uint = 0x00001000	/* Even Chancor dump (?) */;
pub const AR5K_PHY_TST2_RFSILENT_EN: c_uint = 0x00002000	/* Enable RFSILENT */;
pub const AR5K_PHY_TST2_ALT_RFDATA: c_uint = 0x00004000	/* Alternate RFDATA (5-2GHz switch ?) */;
pub const AR5K_PHY_TST2_MINI_OBS_EN: c_uint = 0x00008000	/* Enable mini OBS (?) */;
pub const AR5K_PHY_TST2_RX2_IS_RX5_INV: c_uint = 0x00010000	/* 2GHz rx path is the 5GHz path inverted (?) */;
pub const AR5K_PHY_TST2_SLOW_CLK160: c_uint = 0x00020000	/* Slow CLK160 (?) */;
pub const AR5K_PHY_TST2_AGC_OBS_SEL_3: c_uint = 0x00040000	/* AGC OBS Select 3 (?) */;
pub const AR5K_PHY_TST2_BBB_OBS_SEL: c_uint = 0x00080000	/* BB OBS Select (field ?) */;
pub const AR5K_PHY_TST2_ADC_OBS_SEL: c_uint = 0x00800000	/* ADC OBS Select (field ?) */;
pub const AR5K_PHY_TST2_RX_CLR_SEL: c_uint = 0x08000000	/* RX Clear Select (?) */;
pub const AR5K_PHY_TST2_FORCE_AGC_CLR: c_uint = 0x10000000	/* Force AGC clear (?) */;
pub const AR5K_PHY_SHIFT_2GHZ: c_uint = 0x00004007	/* Used to access 2GHz radios */;
pub const AR5K_PHY_SHIFT_5GHZ: c_uint = 0x00000007	/* Used to access 5GHz radios (default) */;
//
// PHY frame control register [5110] /turbo mode register [5111+]
//
// There is another frame control register for [5111+]
// at address 0x9944 (see below) but the 2 first flags
// are common here between 5110 frame control register
// and [5111+] turbo mode register, so this also works as
// a "turbo mode register" for 5110. We treat this one as
// a frame control register for 5110 below.
//
pub const AR5K_PHY_TURBO: c_uint = 0x9804			/* Register Address */;
pub const AR5K_PHY_TURBO_MODE: c_uint = 0x00000001	/* Enable turbo mode */;
pub const AR5K_PHY_TURBO_SHORT: c_uint = 0x00000002	/* Set short symbols to turbo mode */;
pub const AR5K_PHY_TURBO_MIMO: c_uint = 0x00000004	/* Set turbo for mimo */;
//
// PHY agility command register
// (aka TST_1)
//
pub const AR5K_PHY_AGC: c_uint = 0x9808			/* Register Address */;
pub const AR5K_PHY_TST1: c_uint = 0x9808;
pub const AR5K_PHY_AGC_DISABLE: c_uint = 0x08000000	/* Disable AGC to A2 (?)*/;
pub const AR5K_PHY_TST1_TXHOLD: c_uint = 0x00003800	/* Set tx hold (?) */;
pub const AR5K_PHY_TST1_TXSRC_SRC: c_uint = 0x00000002	/* Used with bit 7 (?) */;
pub const AR5K_PHY_TST1_TXSRC_SRC_S: c_int = 1;
pub const AR5K_PHY_TST1_TXSRC_ALT: c_uint = 0x00000080	/* Set input to tsdac (?) */;
pub const AR5K_PHY_TST1_TXSRC_ALT_S: c_int = 7;
//
// PHY timing register 3 [5112+]
//
pub const AR5K_PHY_TIMING_3: c_uint = 0x9814;
pub const AR5K_PHY_TIMING_3_DSC_MAN: c_uint = 0xfffe0000;
pub const AR5K_PHY_TIMING_3_DSC_MAN_S: c_int = 17;
pub const AR5K_PHY_TIMING_3_DSC_EXP: c_uint = 0x0001e000;
pub const AR5K_PHY_TIMING_3_DSC_EXP_S: c_int = 13;
//
// PHY chip revision register
//
pub const AR5K_PHY_CHIP_ID: c_uint = 0x9818;
//
// PHY activation register
//
pub const AR5K_PHY_ACT: c_uint = 0x981c			/* Register Address */;
pub const AR5K_PHY_ACT_ENABLE: c_uint = 0x00000001	/* Activate PHY */;
pub const AR5K_PHY_ACT_DISABLE: c_uint = 0x00000002	/* Deactivate PHY */;
//
// PHY RF control registers
//
pub const AR5K_PHY_RF_CTL2: c_uint = 0x9824			/* Register Address */;
pub const AR5K_PHY_RF_CTL2_TXF2TXD_START: c_uint = 0x0000000f	/* TX frame to TX data start */;
pub const AR5K_PHY_RF_CTL2_TXF2TXD_START_S: c_int = 0;
pub const AR5K_PHY_RF_CTL3: c_uint = 0x9828			/* Register Address */;
pub const AR5K_PHY_RF_CTL3_TXE2XLNA_ON: c_uint = 0x0000ff00	/* TX end to XLNA on */;
pub const AR5K_PHY_RF_CTL3_TXE2XLNA_ON_S: c_int = 8;
pub const AR5K_PHY_ADC_CTL: c_uint = 0x982c;
pub const AR5K_PHY_ADC_CTL_INBUFGAIN_OFF: c_uint = 0x00000003;
pub const AR5K_PHY_ADC_CTL_INBUFGAIN_OFF_S: c_int = 0;
pub const AR5K_PHY_ADC_CTL_PWD_DAC_OFF: c_uint = 0x00002000;
pub const AR5K_PHY_ADC_CTL_PWD_BAND_GAP_OFF: c_uint = 0x00004000;
pub const AR5K_PHY_ADC_CTL_PWD_ADC_OFF: c_uint = 0x00008000;
pub const AR5K_PHY_ADC_CTL_INBUFGAIN_ON: c_uint = 0x00030000;
pub const AR5K_PHY_ADC_CTL_INBUFGAIN_ON_S: c_int = 16;
pub const AR5K_PHY_RF_CTL4: c_uint = 0x9834			/* Register Address */;
pub const AR5K_PHY_RF_CTL4_TXF2XPA_A_ON: c_uint = 0x00000001	/* TX frame to XPA A on (field) */;
pub const AR5K_PHY_RF_CTL4_TXF2XPA_B_ON: c_uint = 0x00000100	/* TX frame to XPA B on (field) */;
pub const AR5K_PHY_RF_CTL4_TXE2XPA_A_OFF: c_uint = 0x00010000	/* TX end to XPA A off (field) */;
pub const AR5K_PHY_RF_CTL4_TXE2XPA_B_OFF: c_uint = 0x01000000	/* TX end to XPA B off (field) */;
//
// Pre-Amplifier control register
// (XPA -> external pre-amplifier)
//
pub const AR5K_PHY_PA_CTL: c_uint = 0x9838			/* Register Address */;
pub const AR5K_PHY_PA_CTL_XPA_A_HI: c_uint = 0x00000001	/* XPA A high (?) */;
pub const AR5K_PHY_PA_CTL_XPA_B_HI: c_uint = 0x00000002	/* XPA B high (?) */;
pub const AR5K_PHY_PA_CTL_XPA_A_EN: c_uint = 0x00000004	/* Enable XPA A */;
pub const AR5K_PHY_PA_CTL_XPA_B_EN: c_uint = 0x00000008	/* Enable XPA B */;
//
// PHY settling register
//
pub const AR5K_PHY_SETTLING: c_uint = 0x9844			/* Register Address */;
pub const AR5K_PHY_SETTLING_AGC: c_uint = 0x0000007f	/* AGC settling time */;
pub const AR5K_PHY_SETTLING_AGC_S: c_int = 0;
pub const AR5K_PHY_SETTLING_SWITCH: c_uint = 0x00003f80	/* Switch settling time */;
pub const AR5K_PHY_SETTLING_SWITCH_S: c_int = 7;
//
// PHY Gain registers
//
pub const AR5K_PHY_GAIN: c_uint = 0x9848			/* Register Address */;
pub const AR5K_PHY_GAIN_TXRX_ATTEN: c_uint = 0x0003f000	/* TX-RX Attenuation */;
pub const AR5K_PHY_GAIN_TXRX_ATTEN_S: c_int = 12;
pub const AR5K_PHY_GAIN_TXRX_RF_MAX: c_uint = 0x007c0000;
pub const AR5K_PHY_GAIN_TXRX_RF_MAX_S: c_int = 18;
pub const AR5K_PHY_GAIN_OFFSET: c_uint = 0x984c			/* Register Address */;
pub const AR5K_PHY_GAIN_OFFSET_RXTX_FLAG: c_uint = 0x00020000	/* RX-TX flag (?) */;
//
// Desired ADC/PGA size register
// (for more infos read ANI patent)
//
pub const AR5K_PHY_DESIRED_SIZE: c_uint = 0x9850			/* Register Address */;
pub const AR5K_PHY_DESIRED_SIZE_ADC: c_uint = 0x000000ff	/* ADC desired size */;
pub const AR5K_PHY_DESIRED_SIZE_ADC_S: c_int = 0;
pub const AR5K_PHY_DESIRED_SIZE_PGA: c_uint = 0x0000ff00	/* PGA desired size */;
pub const AR5K_PHY_DESIRED_SIZE_PGA_S: c_int = 8;
pub const AR5K_PHY_DESIRED_SIZE_TOT: c_uint = 0x0ff00000	/* Total desired size */;
pub const AR5K_PHY_DESIRED_SIZE_TOT_S: c_int = 20;
//
// PHY signal register
// (for more infos read ANI patent)
//
pub const AR5K_PHY_SIG: c_uint = 0x9858			/* Register Address */;
pub const AR5K_PHY_SIG_FIRSTEP: c_uint = 0x0003f000	/* FIRSTEP */;
pub const AR5K_PHY_SIG_FIRSTEP_S: c_int = 12;
pub const AR5K_PHY_SIG_FIRPWR: c_uint = 0x03fc0000	/* FIPWR */;
pub const AR5K_PHY_SIG_FIRPWR_S: c_int = 18;
//
// PHY coarse agility control register
// (for more infos read ANI patent)
//
pub const AR5K_PHY_AGCCOARSE: c_uint = 0x985c			/* Register Address */;
pub const AR5K_PHY_AGCCOARSE_LO: c_uint = 0x00007f80	/* AGC Coarse low */;
pub const AR5K_PHY_AGCCOARSE_LO_S: c_int = 7;
pub const AR5K_PHY_AGCCOARSE_HI: c_uint = 0x003f8000	/* AGC Coarse high */;
pub const AR5K_PHY_AGCCOARSE_HI_S: c_int = 15;
//
// PHY agility control register
//
pub const AR5K_PHY_AGCCTL: c_uint = 0x9860			/* Register address */;
pub const AR5K_PHY_AGCCTL_CAL: c_uint = 0x00000001	/* Enable PHY calibration */;
pub const AR5K_PHY_AGCCTL_NF: c_uint = 0x00000002	/* Enable Noise Floor calibration */;
pub const AR5K_PHY_AGCCTL_OFDM_DIV_DIS: c_uint = 0x00000008	/* Disable antenna diversity on OFDM modes */;
pub const AR5K_PHY_AGCCTL_NF_EN: c_uint = 0x00008000	/* Enable nf calibration to happen (?) */;
pub const AR5K_PHY_AGCTL_FLTR_CAL: c_uint = 0x00010000	/* Allow filter calibration (?) */;
pub const AR5K_PHY_AGCCTL_NF_NOUPDATE: c_uint = 0x00020000	/* Don't update nf automatically */;
//
// PHY noise floor status register (CCA = Clear Channel Assessment)
//
pub const AR5K_PHY_NF: c_uint = 0x9864			/* Register address */;
pub const AR5K_PHY_NF_M: c_uint = 0x000001ff	/* Noise floor, written to hardware in 1/2 dBm units */;

pub const AR5K_PHY_NF_THRESH62: c_uint = 0x0007f000	/* Thresh62 -check ANI patent- (field) */;
pub const AR5K_PHY_NF_THRESH62_S: c_int = 12;
pub const AR5K_PHY_NF_MINCCA_PWR: c_uint = 0x0ff80000	/* Minimum measured noise level, read from hardware in 1 dBm units */;
pub const AR5K_PHY_NF_MINCCA_PWR_S: c_int = 19;
//
// PHY ADC saturation register [5110]
//
pub const AR5K_PHY_ADCSAT: c_uint = 0x9868;
pub const AR5K_PHY_ADCSAT_ICNT: c_uint = 0x0001f800;
pub const AR5K_PHY_ADCSAT_ICNT_S: c_int = 11;
pub const AR5K_PHY_ADCSAT_THR: c_uint = 0x000007e0;
pub const AR5K_PHY_ADCSAT_THR_S: c_int = 5;
//
// PHY Weak ofdm signal detection threshold registers (ANI) [5212+]
//
// High thresholds
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR: c_uint = 0x9868;
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR_M2_COUNT: c_uint = 0x0000001f;
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR_M2_COUNT_S: c_int = 0;
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR_M1: c_uint = 0x00fe0000;
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR_M1_S: c_int = 17;
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR_M2: c_uint = 0x7f000000;
pub const AR5K_PHY_WEAK_OFDM_HIGH_THR_M2_S: c_int = 24;
// Low thresholds
pub const AR5K_PHY_WEAK_OFDM_LOW_THR: c_uint = 0x986c;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_SELFCOR_EN: c_uint = 0x00000001;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_M2_COUNT: c_uint = 0x00003f00;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_M2_COUNT_S: c_int = 8;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_M1: c_uint = 0x001fc000;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_M1_S: c_int = 14;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_M2: c_uint = 0x0fe00000;
pub const AR5K_PHY_WEAK_OFDM_LOW_THR_M2_S: c_int = 21;
//
// PHY sleep registers [5112+]
//
pub const AR5K_PHY_SCR: c_uint = 0x9870;
pub const AR5K_PHY_SLMT: c_uint = 0x9874;
pub const AR5K_PHY_SLMT_32MHZ: c_uint = 0x0000007f;
pub const AR5K_PHY_SCAL: c_uint = 0x9878;
pub const AR5K_PHY_SCAL_32MHZ: c_uint = 0x0000000e;
pub const AR5K_PHY_SCAL_32MHZ_5311: c_uint = 0x00000008;
pub const AR5K_PHY_SCAL_32MHZ_2417: c_uint = 0x0000000a;
pub const AR5K_PHY_SCAL_32MHZ_HB63: c_uint = 0x00000032;
//
// PHY PLL (Phase Locked Loop) control register
//
pub const AR5K_PHY_PLL: c_uint = 0x987c;
pub const AR5K_PHY_PLL_20MHZ: c_uint = 0x00000013	/* For half rate (?) */;
// 40MHz -> 5GHz band
pub const AR5K_PHY_PLL_40MHZ_5211: c_uint = 0x00000018;
pub const AR5K_PHY_PLL_40MHZ_5212: c_uint = 0x000000aa;
pub const AR5K_PHY_PLL_40MHZ_5413: c_uint = 0x00000004;

// 44MHz -> 2.4GHz band
pub const AR5K_PHY_PLL_44MHZ_5211: c_uint = 0x00000019;
pub const AR5K_PHY_PLL_44MHZ_5212: c_uint = 0x000000ab;

pub const AR5K_PHY_PLL_RF5111: c_uint = 0x00000000;
pub const AR5K_PHY_PLL_RF5112: c_uint = 0x00000040;
pub const AR5K_PHY_PLL_HALF_RATE: c_uint = 0x00000100;
pub const AR5K_PHY_PLL_QUARTER_RATE: c_uint = 0x00000200;
//
// RF Buffer register
//
// It's obvious from the code that 0x989c is the buffer register but
// for the other special registers that we write to after sending each
// packet, i have no idea. So I'll name them BUFFER_CONTROL_X registers
// for now. It's interesting that they are also used for some other operations.
//
pub const AR5K_RF_BUFFER: c_uint = 0x989c;
pub const AR5K_RF_BUFFER_CONTROL_0: c_uint = 0x98c0	/* Channel on 5110 */;
pub const AR5K_RF_BUFFER_CONTROL_1: c_uint = 0x98c4	/* Bank 7 on 5112 */;
pub const AR5K_RF_BUFFER_CONTROL_2: c_uint = 0x98cc	/* Bank 7 on 5111 */;
pub const AR5K_RF_BUFFER_CONTROL_3: c_uint = 0x98d0	/* Bank 2 on 5112 */;
// Channel set on 5111
// Used to read radio revision
pub const AR5K_RF_BUFFER_CONTROL_4: c_uint = 0x98d4  /* RF Stage register on 5110 */;
// Bank 0,1,2,6 on 5111
// Bank 1 on 5112
// Used during activation on 5111
pub const AR5K_RF_BUFFER_CONTROL_5: c_uint = 0x98d8	/* Bank 3 on 5111 */;
// Used during activation on 5111
// Channel on 5112
// Bank 6 on 5112
pub const AR5K_RF_BUFFER_CONTROL_6: c_uint = 0x98dc	/* Bank 3 on 5112 */;
//
// PHY RF stage register [5210]
//
pub const AR5K_PHY_RFSTG: c_uint = 0x98d4;
pub const AR5K_PHY_RFSTG_DISABLE: c_uint = 0x00000021;
//
// BIN masks (?)
//
pub const AR5K_PHY_BIN_MASK_1: c_uint = 0x9900;
pub const AR5K_PHY_BIN_MASK_2: c_uint = 0x9904;
pub const AR5K_PHY_BIN_MASK_3: c_uint = 0x9908;
pub const AR5K_PHY_BIN_MASK_CTL: c_uint = 0x990c;
pub const AR5K_PHY_BIN_MASK_CTL_MASK_4: c_uint = 0x00003fff;
pub const AR5K_PHY_BIN_MASK_CTL_MASK_4_S: c_int = 0;
pub const AR5K_PHY_BIN_MASK_CTL_RATE: c_uint = 0xff000000;
pub const AR5K_PHY_BIN_MASK_CTL_RATE_S: c_int = 24;
//
// PHY Antenna control register
//
pub const AR5K_PHY_ANT_CTL: c_uint = 0x9910			/* Register Address */;
pub const AR5K_PHY_ANT_CTL_TXRX_EN: c_uint = 0x00000001	/* Enable TX/RX (?) */;
pub const AR5K_PHY_ANT_CTL_SECTORED_ANT: c_uint = 0x00000004	/* Sectored Antenna */;
pub const AR5K_PHY_ANT_CTL_HITUNE5: c_uint = 0x00000008	/* Hitune5 (?) */;
pub const AR5K_PHY_ANT_CTL_SWTABLE_IDLE: c_uint = 0x000003f0	/* Switch table idle (?) */;
pub const AR5K_PHY_ANT_CTL_SWTABLE_IDLE_S: c_int = 4;
//
// PHY receiver delay register [5111+]
//
pub const AR5K_PHY_RX_DELAY: c_uint = 0x9914			/* Register Address */;
pub const AR5K_PHY_RX_DELAY_M: c_uint = 0x00003fff	/* Mask for RX activate to receive delay (/100ns) */;
//
// PHY max rx length register (?) [5111]
//
pub const AR5K_PHY_MAX_RX_LEN: c_uint = 0x991c;
//
// PHY timing register 4
// I(nphase)/Q(adrature) calibration register [5111+]
//
pub const AR5K_PHY_IQ: c_uint = 0x9920			/* Register Address */;
pub const AR5K_PHY_IQ_CORR_Q_Q_COFF: c_uint = 0x0000001f	/* Mask for q correction info */;
pub const AR5K_PHY_IQ_CORR_Q_Q_COFF_S: c_int = 0;
pub const AR5K_PHY_IQ_CORR_Q_I_COFF: c_uint = 0x000007e0	/* Mask for i correction info */;
pub const AR5K_PHY_IQ_CORR_Q_I_COFF_S: c_int = 5;
pub const AR5K_PHY_IQ_CORR_ENABLE: c_uint = 0x00000800	/* Enable i/q correction */;
pub const AR5K_PHY_IQ_CAL_NUM_LOG_MAX: c_uint = 0x0000f000	/* Mask for max number of samples in log scale */;
pub const AR5K_PHY_IQ_CAL_NUM_LOG_MAX_S: c_int = 12;
pub const AR5K_PHY_IQ_RUN: c_uint = 0x00010000	/* Run i/q calibration */;
pub const AR5K_PHY_IQ_USE_PT_DF: c_uint = 0x00020000	/* Use pilot track df (?) */;
pub const AR5K_PHY_IQ_EARLY_TRIG_THR: c_uint = 0x00200000	/* Early trigger threshold (?) (field) */;
pub const AR5K_PHY_IQ_PILOT_MASK_EN: c_uint = 0x10000000	/* Enable pilot mask (?) */;
pub const AR5K_PHY_IQ_CHAN_MASK_EN: c_uint = 0x20000000	/* Enable channel mask (?) */;
pub const AR5K_PHY_IQ_SPUR_FILT_EN: c_uint = 0x40000000	/* Enable spur filter */;
pub const AR5K_PHY_IQ_SPUR_RSSI_EN: c_uint = 0x80000000	/* Enable spur rssi */;
//
// PHY timing register 5
// OFDM Self-correlator Cyclic RSSI threshold params
// (Check out bb_cycpwr_thr1 on ANI patent)
//
pub const AR5K_PHY_OFDM_SELFCORR: c_uint = 0x9924			/* Register Address */;
pub const AR5K_PHY_OFDM_SELFCORR_CYPWR_THR1_EN: c_uint = 0x00000001	/* Enable cyclic RSSI thr 1 */;
pub const AR5K_PHY_OFDM_SELFCORR_CYPWR_THR1: c_uint = 0x000000fe	/* Mask for Cyclic RSSI threshold 1 */;
pub const AR5K_PHY_OFDM_SELFCORR_CYPWR_THR1_S: c_int = 1;
pub const AR5K_PHY_OFDM_SELFCORR_CYPWR_THR3: c_uint = 0x00000100	/* Cyclic RSSI threshold 3 (field) (?) */;
pub const AR5K_PHY_OFDM_SELFCORR_RSSI_1ATHR_EN: c_uint = 0x00008000	/* Enable 1A RSSI threshold (?) */;
pub const AR5K_PHY_OFDM_SELFCORR_RSSI_1ATHR: c_uint = 0x00010000	/* 1A RSSI threshold (field) (?) */;
pub const AR5K_PHY_OFDM_SELFCORR_LSCTHR_HIRSSI: c_uint = 0x00800000	/* Long sc threshold hi rssi (?) */;
//
// PHY-only warm reset register
//
pub const AR5K_PHY_WARM_RESET: c_uint = 0x9928;
//
// PHY-only control register
//
pub const AR5K_PHY_CTL: c_uint = 0x992c			/* Register Address */;
pub const AR5K_PHY_CTL_RX_DRAIN_RATE: c_uint = 0x00000001	/* RX drain rate (?) */;
pub const AR5K_PHY_CTL_LATE_TX_SIG_SYM: c_uint = 0x00000002	/* Late tx signal symbol (?) */;
pub const AR5K_PHY_CTL_GEN_SCRAMBLER: c_uint = 0x00000004	/* Generate scrambler */;
pub const AR5K_PHY_CTL_TX_ANT_SEL: c_uint = 0x00000008	/* TX antenna select */;
pub const AR5K_PHY_CTL_TX_ANT_STATIC: c_uint = 0x00000010	/* Static TX antenna */;
pub const AR5K_PHY_CTL_RX_ANT_SEL: c_uint = 0x00000020	/* RX antenna select */;
pub const AR5K_PHY_CTL_RX_ANT_STATIC: c_uint = 0x00000040	/* Static RX antenna */;
pub const AR5K_PHY_CTL_LOW_FREQ_SLE_EN: c_uint = 0x00000080	/* Enable low freq sleep */;
//
// PHY PAPD probe register [5111+]
//
pub const AR5K_PHY_PAPD_PROBE: c_uint = 0x9930;
pub const AR5K_PHY_PAPD_PROBE_SH_HI_PAR: c_uint = 0x00000001;
pub const AR5K_PHY_PAPD_PROBE_PCDAC_BIAS: c_uint = 0x00000002;
pub const AR5K_PHY_PAPD_PROBE_COMP_GAIN: c_uint = 0x00000040;
pub const AR5K_PHY_PAPD_PROBE_TXPOWER: c_uint = 0x00007e00;
pub const AR5K_PHY_PAPD_PROBE_TXPOWER_S: c_int = 9;
pub const AR5K_PHY_PAPD_PROBE_TX_NEXT: c_uint = 0x00008000;
pub const AR5K_PHY_PAPD_PROBE_PREDIST_EN: c_uint = 0x00010000;
pub const AR5K_PHY_PAPD_PROBE_TYPE: c_uint = 0x01800000	/* [5112+] */;
pub const AR5K_PHY_PAPD_PROBE_TYPE_S: c_int = 23;
pub const AR5K_PHY_PAPD_PROBE_TYPE_OFDM: c_int = 0;
pub const AR5K_PHY_PAPD_PROBE_TYPE_XR: c_int = 1;
pub const AR5K_PHY_PAPD_PROBE_TYPE_CCK: c_int = 2;
pub const AR5K_PHY_PAPD_PROBE_GAINF: c_uint = 0xfe000000;
pub const AR5K_PHY_PAPD_PROBE_GAINF_S: c_int = 25;
pub const AR5K_PHY_PAPD_PROBE_INI_5111: c_uint = 0x00004883	/* [5212+] */;
pub const AR5K_PHY_PAPD_PROBE_INI_5112: c_uint = 0x00004882	/* [5212+] */;
//
// PHY TX rate power registers [5112+]
//
pub const AR5K_PHY_TXPOWER_RATE1: c_uint = 0x9934;
pub const AR5K_PHY_TXPOWER_RATE2: c_uint = 0x9938;
pub const AR5K_PHY_TXPOWER_RATE_MAX: c_uint = 0x993c;
pub const AR5K_PHY_TXPOWER_RATE_MAX_TPC_ENABLE: c_uint = 0x00000040;
pub const AR5K_PHY_TXPOWER_RATE3: c_uint = 0xa234;
pub const AR5K_PHY_TXPOWER_RATE4: c_uint = 0xa238;
//
// PHY frame control register [5111+]
//
pub const AR5K_PHY_FRAME_CTL_5210: c_uint = 0x9804;
pub const AR5K_PHY_FRAME_CTL_5211: c_uint = 0x9944;

// ---[5111+]---
pub const AR5K_PHY_FRAME_CTL_WIN_LEN: c_uint = 0x00000003	/* Force window length (?) */;
pub const AR5K_PHY_FRAME_CTL_WIN_LEN_S: c_int = 0;
pub const AR5K_PHY_FRAME_CTL_TX_CLIP: c_uint = 0x00000038	/* Mask for tx clip (?) */;
pub const AR5K_PHY_FRAME_CTL_TX_CLIP_S: c_int = 3;
pub const AR5K_PHY_FRAME_CTL_PREP_CHINFO: c_uint = 0x00010000	/* Prepend chan info */;
pub const AR5K_PHY_FRAME_CTL_EMU: c_uint = 0x80000000;
pub const AR5K_PHY_FRAME_CTL_EMU_S: c_int = 31;
// ---[5110/5111]---
pub const AR5K_PHY_FRAME_CTL_TIMING_ERR: c_uint = 0x01000000	/* PHY timing error */;
pub const AR5K_PHY_FRAME_CTL_PARITY_ERR: c_uint = 0x02000000	/* Parity error */;
pub const AR5K_PHY_FRAME_CTL_ILLRATE_ERR: c_uint = 0x04000000	/* Illegal rate */;
pub const AR5K_PHY_FRAME_CTL_ILLLEN_ERR: c_uint = 0x08000000	/* Illegal length */;
pub const AR5K_PHY_FRAME_CTL_SERVICE_ERR: c_uint = 0x20000000;
pub const AR5K_PHY_FRAME_CTL_TXURN_ERR: c_uint = 0x40000000	/* TX underrun */;

//
// PHY Tx Power adjustment register [5212A+]
//
pub const AR5K_PHY_TX_PWR_ADJ: c_uint = 0x994c;
pub const AR5K_PHY_TX_PWR_ADJ_CCK_GAIN_DELTA: c_uint = 0x00000fc0;
pub const AR5K_PHY_TX_PWR_ADJ_CCK_GAIN_DELTA_S: c_int = 6;
pub const AR5K_PHY_TX_PWR_ADJ_CCK_PCDAC_INDEX: c_uint = 0x00fc0000;
pub const AR5K_PHY_TX_PWR_ADJ_CCK_PCDAC_INDEX_S: c_int = 18;
//
// PHY radar detection register [5111+]
//
pub const AR5K_PHY_RADAR: c_uint = 0x9954;
pub const AR5K_PHY_RADAR_ENABLE: c_uint = 0x00000001;
pub const AR5K_PHY_RADAR_DISABLE: c_uint = 0x00000000;
pub const AR5K_PHY_RADAR_INBANDTHR: c_uint = 0x0000003e	/* Inband threshold;
pub const AR5K_PHY_RADAR_INBANDTHR_S: c_int = 1;
pub const AR5K_PHY_RADAR_PRSSI_THR: c_uint = 0x00000fc0	/* Pulse RSSI/SNR threshold;
pub const AR5K_PHY_RADAR_PRSSI_THR_S: c_int = 6;
pub const AR5K_PHY_RADAR_PHEIGHT_THR: c_uint = 0x0003f000	/* Pulse height threshold;
pub const AR5K_PHY_RADAR_PHEIGHT_THR_S: c_int = 12;
pub const AR5K_PHY_RADAR_RSSI_THR: c_uint = 0x00fc0000	/* Radar RSSI/SNR threshold.;
pub const AR5K_PHY_RADAR_RSSI_THR_S: c_int = 18;
pub const AR5K_PHY_RADAR_FIRPWR_THR: c_uint = 0x7f000000	/* Finite Impulse Response;
pub const AR5K_PHY_RADAR_FIRPWR_THRS: c_int = 24;
//
// PHY antenna switch table registers
//
pub const AR5K_PHY_ANT_SWITCH_TABLE_0: c_uint = 0x9960;
pub const AR5K_PHY_ANT_SWITCH_TABLE_1: c_uint = 0x9964;
//
// PHY Noise floor threshold
//
pub const AR5K_PHY_NFTHRES: c_uint = 0x9968;
//
// Sigma Delta register (?) [5213]
//
pub const AR5K_PHY_SIGMA_DELTA: c_uint = 0x996C;
pub const AR5K_PHY_SIGMA_DELTA_ADC_SEL: c_uint = 0x00000003;
pub const AR5K_PHY_SIGMA_DELTA_ADC_SEL_S: c_int = 0;
pub const AR5K_PHY_SIGMA_DELTA_FILT2: c_uint = 0x000000f8;
pub const AR5K_PHY_SIGMA_DELTA_FILT2_S: c_int = 3;
pub const AR5K_PHY_SIGMA_DELTA_FILT1: c_uint = 0x00001f00;
pub const AR5K_PHY_SIGMA_DELTA_FILT1_S: c_int = 8;
pub const AR5K_PHY_SIGMA_DELTA_ADC_CLIP: c_uint = 0x01ffe000;
pub const AR5K_PHY_SIGMA_DELTA_ADC_CLIP_S: c_int = 13;
//
// RF restart register [5112+] (?)
//
pub const AR5K_PHY_RESTART: c_uint = 0x9970		/* restart */;
pub const AR5K_PHY_RESTART_DIV_GC: c_uint = 0x001c0000	/* Fast diversity gc_limit (?) */;
pub const AR5K_PHY_RESTART_DIV_GC_S: c_int = 18;
//
// RF Bus access request register (for synth-only channel switching)
//
pub const AR5K_PHY_RFBUS_REQ: c_uint = 0x997C;
pub const AR5K_PHY_RFBUS_REQ_REQUEST: c_uint = 0x00000001;
//
// Spur mitigation masks (?)
//
pub const AR5K_PHY_TIMING_7: c_uint = 0x9980;
pub const AR5K_PHY_TIMING_8: c_uint = 0x9984;
pub const AR5K_PHY_TIMING_8_PILOT_MASK_2: c_uint = 0x000fffff;
pub const AR5K_PHY_TIMING_8_PILOT_MASK_2_S: c_int = 0;
pub const AR5K_PHY_BIN_MASK2_1: c_uint = 0x9988;
pub const AR5K_PHY_BIN_MASK2_2: c_uint = 0x998c;
pub const AR5K_PHY_BIN_MASK2_3: c_uint = 0x9990;
pub const AR5K_PHY_BIN_MASK2_4: c_uint = 0x9994;
pub const AR5K_PHY_BIN_MASK2_4_MASK_4: c_uint = 0x00003fff;
pub const AR5K_PHY_BIN_MASK2_4_MASK_4_S: c_int = 0;
pub const AR5K_PHY_TIMING_9: c_uint = 0x9998;
pub const AR5K_PHY_TIMING_10: c_uint = 0x999c;
pub const AR5K_PHY_TIMING_10_PILOT_MASK_2: c_uint = 0x000fffff;
pub const AR5K_PHY_TIMING_10_PILOT_MASK_2_S: c_int = 0;
//
// Spur mitigation control
//
pub const AR5K_PHY_TIMING_11: c_uint = 0x99a0		/* Register address */;
pub const AR5K_PHY_TIMING_11_SPUR_DELTA_PHASE: c_uint = 0x000fffff	/* Spur delta phase */;
pub const AR5K_PHY_TIMING_11_SPUR_DELTA_PHASE_S: c_int = 0;
pub const AR5K_PHY_TIMING_11_SPUR_FREQ_SD: c_uint = 0x3ff00000	/* Freq sigma delta */;
pub const AR5K_PHY_TIMING_11_SPUR_FREQ_SD_S: c_int = 20;
pub const AR5K_PHY_TIMING_11_USE_SPUR_IN_AGC: c_uint = 0x40000000	/* Spur filter in AGC detector */;
pub const AR5K_PHY_TIMING_11_USE_SPUR_IN_SELFCOR: c_uint = 0x80000000	/* Spur filter in OFDM self correlator */;
//
// Gain tables
//
pub const AR5K_BB_GAIN_BASE: c_uint = 0x9b00	/* BaseBand Amplifier Gain table base address */;

pub const AR5K_RF_GAIN_BASE: c_uint = 0x9a00	/* RF Amplifier Gain table base address */;

//
// PHY timing IQ calibration result register [5111+]
//
pub const AR5K_PHY_IQRES_CAL_PWR_I: c_uint = 0x9c10	/* I (Inphase) power value */;
pub const AR5K_PHY_IQRES_CAL_PWR_Q: c_uint = 0x9c14	/* Q (Quadrature) power value */;
pub const AR5K_PHY_IQRES_CAL_CORR: c_uint = 0x9c18	/* I/Q Correlation */;
//
// PHY current RSSI register [5111+]
//
pub const AR5K_PHY_CURRENT_RSSI: c_uint = 0x9c1c;
//
// PHY RF Bus grant register
//
pub const AR5K_PHY_RFBUS_GRANT: c_uint = 0x9c20;
pub const AR5K_PHY_RFBUS_GRANT_OK: c_uint = 0x00000001;
//
// PHY ADC test register
//
pub const AR5K_PHY_ADC_TEST: c_uint = 0x9c24;
pub const AR5K_PHY_ADC_TEST_I: c_uint = 0x00000001;
pub const AR5K_PHY_ADC_TEST_Q: c_uint = 0x00000200;
//
// PHY DAC test register
//
pub const AR5K_PHY_DAC_TEST: c_uint = 0x9c28;
pub const AR5K_PHY_DAC_TEST_I: c_uint = 0x00000001;
pub const AR5K_PHY_DAC_TEST_Q: c_uint = 0x00000200;
//
// PHY PTAT register (?)
//
pub const AR5K_PHY_PTAT: c_uint = 0x9c2c;
//
// PHY Illegal TX rate register [5112+]
//
pub const AR5K_PHY_BAD_TX_RATE: c_uint = 0x9c30;
//
// PHY SPUR Power register [5112+]
//
pub const AR5K_PHY_SPUR_PWR: c_uint = 0x9c34			/* Register Address */;
pub const AR5K_PHY_SPUR_PWR_I: c_uint = 0x00000001	/* SPUR Power estimate for I (field) */;
pub const AR5K_PHY_SPUR_PWR_Q: c_uint = 0x00000100	/* SPUR Power estimate for Q (field) */;
pub const AR5K_PHY_SPUR_PWR_FILT: c_uint = 0x00010000	/* Power with SPUR removed (field) */;
//
// PHY Channel status register [5112+] (?)
//
pub const AR5K_PHY_CHAN_STATUS: c_uint = 0x9c38;
pub const AR5K_PHY_CHAN_STATUS_BT_ACT: c_uint = 0x00000001;
pub const AR5K_PHY_CHAN_STATUS_RX_CLR_RAW: c_uint = 0x00000002;
pub const AR5K_PHY_CHAN_STATUS_RX_CLR_MAC: c_uint = 0x00000004;
pub const AR5K_PHY_CHAN_STATUS_RX_CLR_PAP: c_uint = 0x00000008;
//
// Heavy clip enable register
//
pub const AR5K_PHY_HEAVY_CLIP_ENABLE: c_uint = 0x99e0;
//
// PHY clock sleep registers [5112+]
//
pub const AR5K_PHY_SCLOCK: c_uint = 0x99f0;
pub const AR5K_PHY_SCLOCK_32MHZ: c_uint = 0x0000000c;
pub const AR5K_PHY_SDELAY: c_uint = 0x99f4;
pub const AR5K_PHY_SDELAY_32MHZ: c_uint = 0x000000ff;
pub const AR5K_PHY_SPENDING: c_uint = 0x99f8;
//
// PHY PAPD I (power?) table (?)
// (92! entries)
//
pub const AR5K_PHY_PAPD_I_BASE: c_uint = 0xa000;

//
// PHY PCDAC TX power table
//
pub const AR5K_PHY_PCDAC_TXPOWER_BASE: c_uint = 0xa180;

//
// PHY mode register [5111+]
//
pub const AR5K_PHY_MODE: c_uint = 0x0a200			/* Register Address */;
pub const AR5K_PHY_MODE_MOD: c_uint = 0x00000001	/* PHY Modulation bit */;
pub const AR5K_PHY_MODE_MOD_OFDM: c_int = 0;
pub const AR5K_PHY_MODE_MOD_CCK: c_int = 1;
pub const AR5K_PHY_MODE_FREQ: c_uint = 0x00000002	/* Freq mode bit */;
pub const AR5K_PHY_MODE_FREQ_5GHZ: c_int = 0;
pub const AR5K_PHY_MODE_FREQ_2GHZ: c_int = 2;
pub const AR5K_PHY_MODE_MOD_DYN: c_uint = 0x00000004	/* Enable Dynamic OFDM/CCK mode [5112+] */;
pub const AR5K_PHY_MODE_RAD: c_uint = 0x00000008	/* [5212+] */;
pub const AR5K_PHY_MODE_RAD_RF5111: c_int = 0;
pub const AR5K_PHY_MODE_RAD_RF5112: c_int = 8;
pub const AR5K_PHY_MODE_XR: c_uint = 0x00000010	/* Enable XR mode [5112+] */;
pub const AR5K_PHY_MODE_HALF_RATE: c_uint = 0x00000020	/* Enable Half rate (test) */;
pub const AR5K_PHY_MODE_QUARTER_RATE: c_uint = 0x00000040	/* Enable Quarter rat (test) */;
//
// PHY CCK transmit control register [5111+ (?)]
//
pub const AR5K_PHY_CCKTXCTL: c_uint = 0xa204;
pub const AR5K_PHY_CCKTXCTL_WORLD: c_uint = 0x00000000;
pub const AR5K_PHY_CCKTXCTL_JAPAN: c_uint = 0x00000010;
pub const AR5K_PHY_CCKTXCTL_SCRAMBLER_DIS: c_uint = 0x00000001;
pub const AR5K_PHY_CCKTXCTK_DAC_SCALE: c_uint = 0x00000004;
//
// PHY CCK Cross-correlator Barker RSSI threshold register [5212+]
//
pub const AR5K_PHY_CCK_CROSSCORR: c_uint = 0xa208;
pub const AR5K_PHY_CCK_CROSSCORR_WEAK_SIG_THR: c_uint = 0x0000003f;
pub const AR5K_PHY_CCK_CROSSCORR_WEAK_SIG_THR_S: c_int = 0;
// Same address is used for antenna diversity activation
pub const AR5K_PHY_FAST_ANT_DIV: c_uint = 0xa208;
pub const AR5K_PHY_FAST_ANT_DIV_EN: c_uint = 0x00002000;
//
// PHY 2GHz gain register [5111+]
//
pub const AR5K_PHY_GAIN_2GHZ: c_uint = 0xa20c;
pub const AR5K_PHY_GAIN_2GHZ_MARGIN_TXRX: c_uint = 0x00fc0000;
pub const AR5K_PHY_GAIN_2GHZ_MARGIN_TXRX_S: c_int = 18;
pub const AR5K_PHY_GAIN_2GHZ_INI_5111: c_uint = 0x6480416c;
pub const AR5K_PHY_CCK_RX_CTL_4: c_uint = 0xa21c;
pub const AR5K_PHY_CCK_RX_CTL_4_FREQ_EST_SHORT: c_uint = 0x01f80000;
pub const AR5K_PHY_CCK_RX_CTL_4_FREQ_EST_SHORT_S: c_int = 19;
pub const AR5K_PHY_DAG_CCK_CTL: c_uint = 0xa228;
pub const AR5K_PHY_DAG_CCK_CTL_EN_RSSI_THR: c_uint = 0x00000200;
pub const AR5K_PHY_DAG_CCK_CTL_RSSI_THR: c_uint = 0x0001fc00;
pub const AR5K_PHY_DAG_CCK_CTL_RSSI_THR_S: c_int = 10;
pub const AR5K_PHY_FAST_ADC: c_uint = 0xa24c;
pub const AR5K_PHY_BLUETOOTH: c_uint = 0xa254;
//
// Transmit Power Control register
// [2413+]
//
pub const AR5K_PHY_TPC_RG1: c_uint = 0xa258;
pub const AR5K_PHY_TPC_RG1_NUM_PD_GAIN: c_uint = 0x0000c000;
pub const AR5K_PHY_TPC_RG1_NUM_PD_GAIN_S: c_int = 14;
pub const AR5K_PHY_TPC_RG1_PDGAIN_1: c_uint = 0x00030000;
pub const AR5K_PHY_TPC_RG1_PDGAIN_1_S: c_int = 16;
pub const AR5K_PHY_TPC_RG1_PDGAIN_2: c_uint = 0x000c0000;
pub const AR5K_PHY_TPC_RG1_PDGAIN_2_S: c_int = 18;
pub const AR5K_PHY_TPC_RG1_PDGAIN_3: c_uint = 0x00300000;
pub const AR5K_PHY_TPC_RG1_PDGAIN_3_S: c_int = 20;
pub const AR5K_PHY_TPC_RG5: c_uint = 0xa26C;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_OVERLAP: c_uint = 0x0000000F;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_OVERLAP_S: c_int = 0;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_1: c_uint = 0x000003F0;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_1_S: c_int = 4;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_2: c_uint = 0x0000FC00;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_2_S: c_int = 10;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_3: c_uint = 0x003F0000;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_3_S: c_int = 16;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_4: c_uint = 0x0FC00000;
pub const AR5K_PHY_TPC_RG5_PD_GAIN_BOUNDARY_4_S: c_int = 22;
//
// PHY PDADC Tx power table
//
pub const AR5K_PHY_PDADC_TXPOWER_BASE: c_uint = 0xa280;

//
// Platform registers for WiSoC
//
pub const AR5K_AR5312_RESET: c_uint = 0xbc003020;
pub const AR5K_AR5312_RESET_BB0_COLD: c_uint = 0x00000004;
pub const AR5K_AR5312_RESET_BB1_COLD: c_uint = 0x00000200;
pub const AR5K_AR5312_RESET_WMAC0: c_uint = 0x00002000;
pub const AR5K_AR5312_RESET_BB0_WARM: c_uint = 0x00004000;
pub const AR5K_AR5312_RESET_WMAC1: c_uint = 0x00020000;
pub const AR5K_AR5312_RESET_BB1_WARM: c_uint = 0x00040000;
pub const AR5K_AR5312_ENABLE: c_uint = 0xbc003080;
pub const AR5K_AR5312_ENABLE_WLAN0: c_uint = 0x00000001;
pub const AR5K_AR5312_ENABLE_WLAN1: c_uint = 0x00000008;
pub const AR5K_AR2315_RESET: c_uint = 0xb1000004;
pub const AR5K_AR2315_RESET_WMAC: c_uint = 0x00000001;
pub const AR5K_AR2315_RESET_BB_WARM: c_uint = 0x00000002;
pub const AR5K_AR2315_AHB_ARB_CTL: c_uint = 0xb1000008;
pub const AR5K_AR2315_AHB_ARB_CTL_WLAN: c_uint = 0x00000002;
pub const AR5K_AR2315_BYTESWAP: c_uint = 0xb100000c;
pub const AR5K_AR2315_BYTESWAP_WMAC: c_uint = 0x00000002;
