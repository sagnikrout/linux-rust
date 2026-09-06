//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/immap_qe.h
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
// QUICC Engine (QE) Internal Memory Map.
// The Internal Memory Map for devices with QE on them. This
// is the superset of all QE devices (8360, etc.).
// Copyright (C) 2006. Freescale Semiconductor, Inc. All rights reserved.
//
// Authors: 	Shlomi Gridish <gridish@freescale.com>
// Li Yang <leoli@freescale.com>
//

// QE I-RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_iram {
    pub /: *mut *mut __be32 iadd; / I-RAM Address Register,
    pub /: *mut *mut __be32 idata; / I-RAM Data Register,
    pub res0: [u8; 0x04],
    pub /: *mut *mut __be32 iready; / I-RAM Ready Register,
    pub res1: [u8; 0x70],
// C attribute field omitted
// QE Interrupt Controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_ic_regs {
    pub qicr: __be32,
    pub qivec: __be32,
    pub qripnr: __be32,
    pub qipnr: __be32,
    pub qipxcc: __be32,
    pub qipycc: __be32,
    pub qipwcc: __be32,
    pub qipzcc: __be32,
    pub qimr: __be32,
    pub qrimr: __be32,
    pub qicnr: __be32,
    pub res0: [u8; 0x4],
    pub qiprta: __be32,
    pub qiprtb: __be32,
    pub res1: [u8; 0x4],
    pub qricr: __be32,
    pub res2: [u8; 0x20],
    pub qhivec: __be32,
    pub res3: [u8; 0x1C],
// C attribute field omitted
// Communications Processor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp_qe {
    pub /: *mut *mut __be32 cecr; / QE command register,
    pub /: *mut *mut __be32 ceccr; / QE controller configuration register,
    pub /: *mut *mut __be32 cecdr; / QE command data register,
    pub res0: [u8; 0xA],
    pub /: *mut *mut __be16 ceter; / QE timer event register,
    pub res1: [u8; 0x2],
    pub /: *mut *mut __be16 cetmr; / QE timers mask register,
    pub /: *mut *mut __be32 cetscr; / QE time-stamp timer control register,
    pub /: *mut *mut __be32 cetsr1; / QE time-stamp register 1,
    pub /: *mut *mut __be32 cetsr2; / QE time-stamp register 2,
    pub res2: [u8; 0x8],
    pub /: *mut *mut __be32 cevter; / QE virtual tasks event register,
    pub /: *mut *mut __be32 cevtmr; / QE virtual tasks mask register,
    pub /: *mut *mut __be16 cercr; / QE RAM control register,
    pub res3: [u8; 0x2],
    pub res4: [u8; 0x24],
    pub /: *mut *mut __be16 ceexe1; / QE external request 1 event register,
    pub res5: [u8; 0x2],
    pub /: *mut *mut __be16 ceexm1; / QE external request 1 mask register,
    pub res6: [u8; 0x2],
    pub /: *mut *mut __be16 ceexe2; / QE external request 2 event register,
    pub res7: [u8; 0x2],
    pub /: *mut *mut __be16 ceexm2; / QE external request 2 mask register,
    pub res8: [u8; 0x2],
    pub /: *mut *mut __be16 ceexe3; / QE external request 3 event register,
    pub res9: [u8; 0x2],
    pub /: *mut *mut __be16 ceexm3; / QE external request 3 mask register,
    pub res10: [u8; 0x2],
    pub /: *mut *mut __be16 ceexe4; / QE external request 4 event register,
    pub res11: [u8; 0x2],
    pub /: *mut *mut __be16 ceexm4; / QE external request 4 mask register,
    pub res12: [u8; 0x3A],
    pub /: *mut *mut __be32 ceurnr; / QE microcode revision number register,
    pub res13: [u8; 0x244],
// C attribute field omitted
// QE Multiplexer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_mux {
    pub /: *mut *mut __be32 cmxgcr; / CMX general clock route register,
    pub /: *mut *mut __be32 cmxsi1cr_l; / CMX SI1 clock route low register,
    pub /: *mut *mut __be32 cmxsi1cr_h; / CMX SI1 clock route high register,
    pub /: *mut *mut __be32 cmxsi1syr; / CMX SI1 SYNC route register,
    pub /: *mut *mut __be32 cmxucr[4]; / CMX UCCx clock route registers,
    pub /: *mut *mut __be32 cmxupcr; / CMX UPC clock route register,
    pub res0: [u8; 0x1C],
// C attribute field omitted
// QE Timers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_timers {
    pub register*/: *mut *mut u8 gtcfr1; / Timer 1 and Timer 2 global config,
    pub res0: [u8; 0x3],
    pub register*/: *mut *mut u8 gtcfr2; / Timer 3 and timer 4 global config,
    pub res1: [u8; 0xB],
    pub /: *mut *mut __be16 gtmdr1; / Timer 1 mode register,
    pub /: *mut *mut __be16 gtmdr2; / Timer 2 mode register,
    pub /: *mut *mut __be16 gtrfr1; / Timer 1 reference register,
    pub /: *mut *mut __be16 gtrfr2; / Timer 2 reference register,
    pub /: *mut *mut __be16 gtcpr1; / Timer 1 capture register,
    pub /: *mut *mut __be16 gtcpr2; / Timer 2 capture register,
    pub /: *mut *mut __be16 gtcnr1; / Timer 1 counter,
    pub /: *mut *mut __be16 gtcnr2; / Timer 2 counter,
    pub /: *mut *mut __be16 gtmdr3; / Timer 3 mode register,
    pub /: *mut *mut __be16 gtmdr4; / Timer 4 mode register,
    pub /: *mut *mut __be16 gtrfr3; / Timer 3 reference register,
    pub /: *mut *mut __be16 gtrfr4; / Timer 4 reference register,
    pub /: *mut *mut __be16 gtcpr3; / Timer 3 capture register,
    pub /: *mut *mut __be16 gtcpr4; / Timer 4 capture register,
    pub /: *mut *mut __be16 gtcnr3; / Timer 3 counter,
    pub /: *mut *mut __be16 gtcnr4; / Timer 4 counter,
    pub /: *mut *mut __be16 gtevr1; / Timer 1 event register,
    pub /: *mut *mut __be16 gtevr2; / Timer 2 event register,
    pub /: *mut *mut __be16 gtevr3; / Timer 3 event register,
    pub /: *mut *mut __be16 gtevr4; / Timer 4 event register,
    pub /: *mut *mut __be16 gtps; / Timer 1 prescale register,
    pub res2: [u8; 0x46],
// C attribute field omitted
// BRG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_brg {
    pub /: *mut *mut __be32 brgc[16]; / BRG configuration registers,
    pub res0: [u8; 0x40],
// C attribute field omitted
// SPI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi {
    pub res0: [u8; 0x20],
    pub /: *mut *mut __be32 spmode; / SPI mode register,
    pub res1: [u8; 0x2],
    pub /: *mut *mut u8 spie; / SPI event register,
    pub res2: [u8; 0x1],
    pub res3: [u8; 0x2],
    pub /: *mut *mut u8 spim; / SPI mask register,
    pub res4: [u8; 0x1],
    pub res5: [u8; 0x1],
    pub /: *mut *mut u8 spcom; / SPI command register,
    pub res6: [u8; 0x2],
    pub /: *mut *mut __be32 spitd; / SPI transmit data register (cpu mode),
    pub /: *mut *mut __be32 spird; / SPI receive data register (cpu mode),
    pub res7: [u8; 0x8],
// C attribute field omitted
// SI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si1 {
    pub /: *mut *mut __be16 sixmr1[4]; / SI1 TDMx (x = A B C D) mode register,
    pub /: *mut *mut u8 siglmr1_h; / SI1 global mode register high,
    pub res0: [u8; 0x1],
    pub /: *mut *mut u8 sicmdr1_h; / SI1 command register high,
    pub res2: [u8; 0x1],
    pub /: *mut *mut u8 sistr1_h; / SI1 status register high,
    pub res3: [u8; 0x1],
    pub /: *mut *mut __be16 sirsr1_h; / SI1 RAM shadow address register high,
    pub /: *mut *mut u8 sitarc1; / SI1 RAM counter Tx TDMA,
    pub /: *mut *mut u8 sitbrc1; / SI1 RAM counter Tx TDMB,
    pub /: *mut *mut u8 sitcrc1; / SI1 RAM counter Tx TDMC,
    pub /: *mut *mut u8 sitdrc1; / SI1 RAM counter Tx TDMD,
    pub /: *mut *mut u8 sirarc1; / SI1 RAM counter Rx TDMA,
    pub /: *mut *mut u8 sirbrc1; / SI1 RAM counter Rx TDMB,
    pub /: *mut *mut u8 sircrc1; / SI1 RAM counter Rx TDMC,
    pub /: *mut *mut u8 sirdrc1; / SI1 RAM counter Rx TDMD,
    pub res4: [u8; 0x8],
    pub /: *mut *mut __be16 siemr1; / SI1 TDME mode register 16 bits,
    pub /: *mut *mut __be16 sifmr1; / SI1 TDMF mode register 16 bits,
    pub /: *mut *mut __be16 sigmr1; / SI1 TDMG mode register 16 bits,
    pub /: *mut *mut __be16 sihmr1; / SI1 TDMH mode register 16 bits,
    pub /: *mut *mut u8 siglmg1_l; / SI1 global mode register low 8 bits,
    pub res5: [u8; 0x1],
    pub /: *mut *mut u8 sicmdr1_l; / SI1 command register low 8 bits,
    pub res6: [u8; 0x1],
    pub /: *mut *mut u8 sistr1_l; / SI1 status register low 8 bits,
    pub res7: [u8; 0x1],
    pub bits*/: *mut *mut __be16 sirsr1_l; / SI1 RAM shadow address register low 16,
    pub /: *mut *mut u8 siterc1; / SI1 RAM counter Tx TDME 8 bits,
    pub /: *mut *mut u8 sitfrc1; / SI1 RAM counter Tx TDMF 8 bits,
    pub /: *mut *mut u8 sitgrc1; / SI1 RAM counter Tx TDMG 8 bits,
    pub /: *mut *mut u8 sithrc1; / SI1 RAM counter Tx TDMH 8 bits,
    pub /: *mut *mut u8 sirerc1; / SI1 RAM counter Rx TDME 8 bits,
    pub /: *mut *mut u8 sirfrc1; / SI1 RAM counter Rx TDMF 8 bits,
    pub /: *mut *mut u8 sirgrc1; / SI1 RAM counter Rx TDMG 8 bits,
    pub /: *mut *mut u8 sirhrc1; / SI1 RAM counter Rx TDMH 8 bits,
    pub res8: [u8; 0x8],
    pub /: *mut *mut __be32 siml1; / SI1 multiframe limit register,
    pub /: *mut *mut u8 siedm1; / SI1 extended diagnostic mode register,
    pub res9: [u8; 0xBB],
// C attribute field omitted
// SI Routing Tables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sir {
    pub tx: [u8; 0x400],
    pub rx: [u8; 0x400],
    pub res0: [u8; 0x800],
// C attribute field omitted
// USB Controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_usb_ctlr {
    pub usb_usmod: u8,
    pub usb_usadr: u8,
    pub usb_uscom: u8,
    pub res1: [u8; 1],
    pub usb_usep: [__be16; 4],
    pub res2: [u8; 4],
    pub usb_usber: __be16,
    pub res3: [u8; 2],
    pub usb_usbmr: __be16,
    pub res4: [u8; 1],
    pub usb_usbs: u8,
    pub usb_ussft: __be16,
    pub res5: [u8; 2],
    pub usb_usfrn: __be16,
    pub res6: [u8; 0x22],
// C attribute field omitted
// MCC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_mcc {
    pub /: *mut *mut __be32 mcce; / MCC event register,
    pub /: *mut *mut __be32 mccm; / MCC mask register,
    pub /: *mut *mut __be32 mccf; / MCC configuration register,
    pub /: *mut *mut __be32 merl; / MCC emergency request level register,
    pub res0: [u8; 0xF0],
// C attribute field omitted
// QE UCC Slow
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_slow {
    pub /: *mut *mut __be32 gumr_l; / UCCx general mode register (low),
    pub /: *mut *mut __be32 gumr_h; / UCCx general mode register (high),
    pub /: *mut *mut __be16 upsmr; / UCCx protocol-specific mode register,
    pub res0: [u8; 0x2],
    pub /: *mut *mut __be16 utodr; / UCCx transmit on demand register,
    pub /: *mut *mut __be16 udsr; / UCCx data synchronization register,
    pub /: *mut *mut __be16 ucce; / UCCx event register,
    pub res1: [u8; 0x2],
    pub /: *mut *mut __be16 uccm; / UCCx mask register,
    pub res2: [u8; 0x1],
    pub /: *mut *mut u8 uccs; / UCCx status register,
    pub res3: [u8; 0x24],
    pub utpt: __be16,
    pub res4: [u8; 0x52],
    pub /: *mut *mut u8 guemr; / UCC general extended mode register,
// C attribute field omitted
// QE UCC Fast
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_fast {
    pub /: *mut *mut __be32 gumr; / UCCx general mode register,
    pub /: *mut *mut __be32 upsmr; / UCCx protocol-specific mode register,
    pub /: *mut *mut __be16 utodr; / UCCx transmit on demand register,
    pub res0: [u8; 0x2],
    pub /: *mut *mut __be16 udsr; / UCCx data synchronization register,
    pub res1: [u8; 0x2],
    pub /: *mut *mut __be32 ucce; / UCCx event register,
    pub /: *mut *mut __be32 uccm; / UCCx mask register,
    pub /: *mut *mut u8 uccs; / UCCx status register,
    pub res2: [u8; 0x7],
    pub /: *mut *mut __be32 urfb; / UCC receive FIFO base,
    pub /: *mut *mut __be16 urfs; / UCC receive FIFO size,
    pub res3: [u8; 0x2],
    pub /: *mut *mut __be16 urfet; / UCC receive FIFO emergency threshold,
    pub emergency: *mut *mut __be16 urfset; / UCC receive FIFO special,
    pub /: *mut *mut __be32 utfb; / UCC transmit FIFO base,
    pub /: *mut *mut __be16 utfs; / UCC transmit FIFO size,
    pub res4: [u8; 0x2],
    pub /: *mut *mut __be16 utfet; / UCC transmit FIFO emergency threshold,
    pub res5: [u8; 0x2],
    pub /: *mut *mut __be16 utftt; / UCC transmit FIFO transmit threshold,
    pub res6: [u8; 0x2],
    pub /: *mut *mut __be16 utpt; / UCC transmit polling timer,
    pub res7: [u8; 0x2],
    pub /: *mut *mut __be32 urtry; / UCC retry counter register,
    pub res8: [u8; 0x4C],
    pub /: *mut *mut u8 guemr; / UCC general extended mode register,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc {
    pub slow: ucc_slow,
    pub fast: ucc_fast,
    pub /: *mut *mut u8 res[0x200]; / UCC blocks are 512 bytes each,
}

// MultiPHY UTOPIA POS Controllers (UPC)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upc {
    pub /: *mut *mut __be32 upgcr; / UTOPIA/POS general configuration register,
    pub /: *mut *mut __be32 uplpa; / UTOPIA/POS last PHY address,
    pub /: *mut *mut __be32 uphec; / ATM HEC register,
    pub /: *mut *mut __be32 upuc; / UTOPIA/POS UCC configuration,
    pub /: *mut *mut __be32 updc1; / UTOPIA/POS device 1 configuration,
    pub /: *mut *mut __be32 updc2; / UTOPIA/POS device 2 configuration,
    pub /: *mut *mut __be32 updc3; / UTOPIA/POS device 3 configuration,
    pub /: *mut *mut __be32 updc4; / UTOPIA/POS device 4 configuration,
    pub /: *mut *mut __be32 upstpa; / UTOPIA/POS STPA threshold,
    pub res0: [u8; 0xC],
    pub /: *mut *mut __be32 updrs1_h; / UTOPIA/POS device 1 rate select,
    pub /: *mut *mut __be32 updrs1_l; / UTOPIA/POS device 1 rate select,
    pub /: *mut *mut __be32 updrs2_h; / UTOPIA/POS device 2 rate select,
    pub /: *mut *mut __be32 updrs2_l; / UTOPIA/POS device 2 rate select,
    pub /: *mut *mut __be32 updrs3_h; / UTOPIA/POS device 3 rate select,
    pub /: *mut *mut __be32 updrs3_l; / UTOPIA/POS device 3 rate select,
    pub /: *mut *mut __be32 updrs4_h; / UTOPIA/POS device 4 rate select,
    pub /: *mut *mut __be32 updrs4_l; / UTOPIA/POS device 4 rate select,
    pub /: *mut *mut __be32 updrp1; / UTOPIA/POS device 1 receive priority low,
    pub /: *mut *mut __be32 updrp2; / UTOPIA/POS device 2 receive priority low,
    pub /: *mut *mut __be32 updrp3; / UTOPIA/POS device 3 receive priority low,
    pub /: *mut *mut __be32 updrp4; / UTOPIA/POS device 4 receive priority low,
    pub /: *mut *mut __be32 upde1; / UTOPIA/POS device 1 event,
    pub /: *mut *mut __be32 upde2; / UTOPIA/POS device 2 event,
    pub /: *mut *mut __be32 upde3; / UTOPIA/POS device 3 event,
    pub /: *mut *mut __be32 upde4; / UTOPIA/POS device 4 event,
    pub uprp1: __be16,
    pub uprp2: __be16,
    pub uprp3: __be16,
    pub uprp4: __be16,
    pub res1: [u8; 0x8],
    pub /: *mut *mut __be16 uptirr1_0; / Device 1 transmit internal rate 0,
    pub /: *mut *mut __be16 uptirr1_1; / Device 1 transmit internal rate 1,
    pub /: *mut *mut __be16 uptirr1_2; / Device 1 transmit internal rate 2,
    pub /: *mut *mut __be16 uptirr1_3; / Device 1 transmit internal rate 3,
    pub /: *mut *mut __be16 uptirr2_0; / Device 2 transmit internal rate 0,
    pub /: *mut *mut __be16 uptirr2_1; / Device 2 transmit internal rate 1,
    pub /: *mut *mut __be16 uptirr2_2; / Device 2 transmit internal rate 2,
    pub /: *mut *mut __be16 uptirr2_3; / Device 2 transmit internal rate 3,
    pub /: *mut *mut __be16 uptirr3_0; / Device 3 transmit internal rate 0,
    pub /: *mut *mut __be16 uptirr3_1; / Device 3 transmit internal rate 1,
    pub /: *mut *mut __be16 uptirr3_2; / Device 3 transmit internal rate 2,
    pub /: *mut *mut __be16 uptirr3_3; / Device 3 transmit internal rate 3,
    pub /: *mut *mut __be16 uptirr4_0; / Device 4 transmit internal rate 0,
    pub /: *mut *mut __be16 uptirr4_1; / Device 4 transmit internal rate 1,
    pub /: *mut *mut __be16 uptirr4_2; / Device 4 transmit internal rate 2,
    pub /: *mut *mut __be16 uptirr4_3; / Device 4 transmit internal rate 3,
    pub /: *mut *mut __be32 uper1; / Device 1 port enable register,
    pub /: *mut *mut __be32 uper2; / Device 2 port enable register,
    pub /: *mut *mut __be32 uper3; / Device 3 port enable register,
    pub /: *mut *mut __be32 uper4; / Device 4 port enable register,
    pub res2: [u8; 0x150],
// C attribute field omitted
// SDMA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma {
    pub /: *mut *mut __be32 sdsr; / Serial DMA status register,
    pub /: *mut *mut __be32 sdmr; / Serial DMA mode register,
    pub /: *mut *mut __be32 sdtr1; / SDMA system bus threshold register,
    pub /: *mut *mut __be32 sdtr2; / SDMA secondary bus threshold register,
    pub /: *mut *mut __be32 sdhy1; / SDMA system bus hysteresis register,
    pub /: *mut *mut __be32 sdhy2; / SDMA secondary bus hysteresis register,
    pub /: *mut *mut __be32 sdta1; / SDMA system bus address register,
    pub /: *mut *mut __be32 sdta2; / SDMA secondary bus address register,
    pub /: *mut *mut __be32 sdtm1; / SDMA system bus MSNUM register,
    pub /: *mut *mut __be32 sdtm2; / SDMA secondary bus MSNUM register,
    pub res0: [u8; 0x10],
    pub /: *mut *mut __be32 sdaqr; / SDMA address bus qualify register,
    pub /: *mut *mut __be32 sdaqmr; / SDMA address bus qualify mask register,
    pub res1: [u8; 0x4],
    pub /: *mut *mut __be32 sdebcr; / SDMA CAM entries base register,
    pub res2: [u8; 0x38],
// C attribute field omitted
// Debug Space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg {
    pub /: *mut *mut __be32 bpdcr; / Breakpoint debug command register,
    pub /: *mut *mut __be32 bpdsr; / Breakpoint debug status register,
    pub /: *mut *mut __be32 bpdmr; / Breakpoint debug mask register,
    pub /: *mut *mut __be32 bprmrr0; / Breakpoint request mode risc register 0,
    pub /: *mut *mut __be32 bprmrr1; / Breakpoint request mode risc register 1,
    pub res0: [u8; 0x8],
    pub /: *mut *mut __be32 bprmtr0; / Breakpoint request mode trb register 0,
    pub /: *mut *mut __be32 bprmtr1; / Breakpoint request mode trb register 1,
    pub res1: [u8; 0x8],
    pub /: *mut *mut __be32 bprmir; / Breakpoint request mode immediate register,
    pub /: *mut *mut __be32 bprmsr; / Breakpoint request mode serial register,
    pub /: *mut *mut __be32 bpemr; / Breakpoint exit mode register,
    pub res2: [u8; 0x48],
// C attribute field omitted
//
// RISC Special Registers (Trap and Breakpoint).  These are described in
// the QE Developer's Handbook.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsp {
    pub /: *mut *mut __be32 tibcr[16]; / Trap/instruction breakpoint control regs,
    pub res0: [u8; 64],
    pub ibcr0: __be32,
    pub ibs0: __be32,
    pub ibcnr0: __be32,
    pub res1: [u8; 4],
    pub ibcr1: __be32,
    pub ibs1: __be32,
    pub ibcnr1: __be32,
    pub npcr: __be32,
    pub dbcr: __be32,
    pub dbar: __be32,
    pub dbamr: __be32,
    pub dbsr: __be32,
    pub dbcnr: __be32,
    pub res2: [u8; 12],
    pub dbdr_h: __be32,
    pub dbdr_l: __be32,
    pub dbdmr_h: __be32,
    pub dbdmr_l: __be32,
    pub bsr: __be32,
    pub bor: __be32,
    pub bior: __be32,
    pub res3: [u8; 4],
    pub iatr: [__be32; 4],
    pub /: *mut *mut __be32 eccr; / Exception control configuration register,
    pub eicr: __be32,
    pub res4: [u8; 0x100-0xf8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_immap {
    pub /: *mut *mut qe_iram iram; / I-RAM,
    pub /: *mut *mut qe_ic_regs ic; / Interrupt Controller,
    pub /: *mut *mut cp_qe cp; / Communications Processor,
    pub /: *mut *mut qe_mux qmx; / QE Multiplexer,
    pub /: *mut *mut qe_timers qet; / QE Timers,
    pub /: *mut *mut spi spi[0x2]; / spi,
    pub /: *mut *mut qe_mcc mcc; / mcc,
    pub /: *mut *mut qe_brg brg; / brg,
    pub /: *mut *mut qe_usb_ctlr usb; / USB,
    pub /: *mut *mut si1 si1; / SI,
    pub res11: [u8; 0x800],
    pub /: *mut *mut sir sir; / SI Routing Tables,
    pub /: *mut *mut ucc ucc1; / ucc1,
    pub /: *mut *mut ucc ucc3; / ucc3,
    pub /: *mut *mut ucc ucc5; / ucc5,
    pub /: *mut *mut ucc ucc7; / ucc7,
    pub res12: [u8; 0x600],
    pub 1*/: *mut *mut upc upc1; / MultiPHY UTOPIA POS Ctrlr,
    pub /: *mut *mut ucc ucc2; / ucc2,
    pub /: *mut *mut ucc ucc4; / ucc4,
    pub /: *mut *mut ucc ucc6; / ucc6,
    pub /: *mut *mut ucc ucc8; / ucc8,
    pub res13: [u8; 0x600],
    pub 2*/: *mut *mut upc upc2; / MultiPHY UTOPIA POS Ctrlr,
    pub /: *mut *mut sdma sdma; / SDMA,
    pub 0x1040FF: *mut *mut dbg dbg; / 0x104080 -,
    pub 0x1042FF: *mut *mut rsp rsp[0x2]; / 0x104100 -,
    pub /: *mut *mut u8 res14[0x300]; / 0x104300 - 0x1045FF,
    pub /: *mut *mut u8 res15[0x3A00]; / 0x104600 - 0x107FFF,
    pub /: *mut *mut u8 res16[0x8000]; / 0x108000 - 0x110000,
    pub 0x11C000: *mut *mut u8 muram[0xC000]; / 0x110000 -,
    pub /: *mut *mut u8 res17[0x24000]; / 0x11C000 - 0x140000,
    pub /: *mut *mut u8 res18[0xC0000]; / 0x140000 - 0x200000,
// C attribute field omitted
    pub qe_immr: *mut extern struct qe_immap __iomem,

