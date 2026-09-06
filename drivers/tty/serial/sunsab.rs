//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/sunsab.h
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
// sunsab.h: Register Definitions for the Siemens SAB82532 DUSCC
//
// Copyright (C) 1997  Eddie C. Dost  (ecd@skynet.be)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sab82532_async_rd_regs {
    pub /: *mut *mut u8 rfifo[0x20]; / Receive FIFO,
    pub /: *mut *mut u8 star; / Status Register,
    pub __pad1: u8,
    pub /: *mut *mut u8 mode; / Mode Register,
    pub /: *mut *mut u8 timr; / Timer Register,
    pub /: *mut *mut u8 xon; / XON Character,
    pub /: *mut *mut u8 xoff; / XOFF Character,
    pub /: *mut *mut u8 tcr; / Termination Character Register,
    pub /: *mut *mut u8 dafo; / Data Format,
    pub /: *mut *mut u8 rfc; / RFIFO Control Register,
    pub __pad2: u8,
    pub /: *mut *mut u8 rbcl; / Receive Byte Count Low,
    pub /: *mut *mut u8 rbch; / Receive Byte Count High,
    pub /: *mut *mut u8 ccr0; / Channel Configuration Register 0,
    pub /: *mut *mut u8 ccr1; / Channel Configuration Register 1,
    pub /: *mut *mut u8 ccr2; / Channel Configuration Register 2,
    pub /: *mut *mut u8 ccr3; / Channel Configuration Register 3,
    pub __pad3: [u8; 4],
    pub /: *mut *mut u8 vstr; / Version Status Register,
    pub __pad4: [u8; 3],
    pub /: *mut *mut u8 gis; / Global Interrupt Status,
    pub /: *mut *mut u8 ipc; / Interrupt Port Configuration,
    pub /: *mut *mut u8 isr0; / Interrupt Status 0,
    pub /: *mut *mut u8 isr1; / Interrupt Status 1,
    pub /: *mut *mut u8 pvr; / Port Value Register,
    pub /: *mut *mut u8 pis; / Port Interrupt Status,
    pub /: *mut *mut u8 pcr; / Port Configuration Register,
    pub /: *mut *mut u8 ccr4; / Channel Configuration Register 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sab82532_async_wr_regs {
    pub /: *mut *mut u8 xfifo[0x20]; / Transmit FIFO,
    pub /: *mut *mut u8 cmdr; / Command Register,
    pub __pad1: u8,
    pub mode: u8,
    pub timr: u8,
    pub xon: u8,
    pub xoff: u8,
    pub tcr: u8,
    pub dafo: u8,
    pub rfc: u8,
    pub __pad2: u8,
    pub /: *mut *mut u8 xbcl; / Transmit Byte Count Low,
    pub /: *mut *mut u8 xbch; / Transmit Byte Count High,
    pub ccr0: u8,
    pub ccr1: u8,
    pub ccr2: u8,
    pub ccr3: u8,
    pub /: *mut *mut u8 tsax; / Time-Slot Assignment Reg. Transmit,
    pub /: *mut *mut u8 tsar; / Time-Slot Assignment Reg. Receive,
    pub /: *mut *mut u8 xccr; / Transmit Channel Capacity Register,
    pub /: *mut *mut u8 rccr; / Receive Channel Capacity Register,
    pub /: *mut *mut u8 bgr; / Baud Rate Generator Register,
    pub /: *mut *mut u8 tic; / Transmit Immediate Character,
    pub /: *mut *mut u8 mxn; / Mask XON Character,
    pub /: *mut *mut u8 mxf; / Mask XOFF Character,
    pub /: *mut *mut u8 iva; / Interrupt Vector Address,
    pub ipc: u8,
    pub /: *mut *mut u8 imr0; / Interrupt Mask Register 0,
    pub /: *mut *mut u8 imr1; / Interrupt Mask Register 1,
    pub pvr: u8,
    pub /: *mut *mut u8 pim; / Port Interrupt Mask,
    pub pcr: u8,
    pub ccr4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sab82532_async_rw_regs {
    pub __pad1: [u8; 0x20],
    pub __pad2: u8,
    pub __pad3: u8,
    pub mode: u8,
    pub timr: u8,
    pub xon: u8,
    pub xoff: u8,
    pub tcr: u8,
    pub dafo: u8,
    pub rfc: u8,
    pub __pad4: u8,
    pub __pad5: u8,
    pub __pad6: u8,
    pub ccr0: u8,
    pub ccr1: u8,
    pub ccr2: u8,
    pub ccr3: u8,
    pub __pad7: u8,
    pub __pad8: u8,
    pub __pad9: u8,
    pub __pad10: u8,
    pub __pad11: u8,
    pub __pad12: u8,
    pub __pad13: u8,
    pub __pad14: u8,
    pub __pad15: u8,
    pub ipc: u8,
    pub __pad16: u8,
    pub __pad17: u8,
    pub pvr: u8,
    pub __pad18: u8,
    pub pcr: u8,
    pub ccr4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sab82532_async_regs {
    pub r: __volatile__ struct sab82532_async_rd_regs,
    pub w: __volatile__ struct sab82532_async_wr_regs,
    pub rw: __volatile__ struct sab82532_async_rw_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sab82532_irq_status {
    pub stat: c_ushort,
    pub isr0: c_uchar,
    pub isr1: c_uchar,
    pub sreg: },
}

// irqflags bits
pub const SAB82532_ALLS: c_uint = 0x00000001;
pub const SAB82532_XPR: c_uint = 0x00000002;
pub const SAB82532_REGS_PENDING: c_uint = 0x00000004;
// RFIFO Status Byte
pub const SAB82532_RSTAT_PE: c_uint = 0x80;
pub const SAB82532_RSTAT_FE: c_uint = 0x40;
pub const SAB82532_RSTAT_PARITY: c_uint = 0x01;
// Status Register (STAR)
pub const SAB82532_STAR_XDOV: c_uint = 0x80;
pub const SAB82532_STAR_XFW: c_uint = 0x40;
pub const SAB82532_STAR_RFNE: c_uint = 0x20;
pub const SAB82532_STAR_FCS: c_uint = 0x10;
pub const SAB82532_STAR_TEC: c_uint = 0x08;
pub const SAB82532_STAR_CEC: c_uint = 0x04;
pub const SAB82532_STAR_CTS: c_uint = 0x02;
// Command Register (CMDR)
pub const SAB82532_CMDR_RMC: c_uint = 0x80;
pub const SAB82532_CMDR_RRES: c_uint = 0x40;
pub const SAB82532_CMDR_RFRD: c_uint = 0x20;
pub const SAB82532_CMDR_STI: c_uint = 0x10;
pub const SAB82532_CMDR_XF: c_uint = 0x08;
pub const SAB82532_CMDR_XRES: c_uint = 0x01;
// Mode Register (MODE)
pub const SAB82532_MODE_FRTS: c_uint = 0x40;
pub const SAB82532_MODE_FCTS: c_uint = 0x20;
pub const SAB82532_MODE_FLON: c_uint = 0x10;
pub const SAB82532_MODE_RAC: c_uint = 0x08;
pub const SAB82532_MODE_RTS: c_uint = 0x04;
pub const SAB82532_MODE_TRS: c_uint = 0x02;
pub const SAB82532_MODE_TLP: c_uint = 0x01;
// Timer Register (TIMR)
pub const SAB82532_TIMR_CNT_MASK: c_uint = 0xe0;
pub const SAB82532_TIMR_VALUE_MASK: c_uint = 0x1f;
// Data Format (DAFO)
pub const SAB82532_DAFO_XBRK: c_uint = 0x40;
pub const SAB82532_DAFO_STOP: c_uint = 0x20;
pub const SAB82532_DAFO_PAR_SPACE: c_uint = 0x00;
pub const SAB82532_DAFO_PAR_ODD: c_uint = 0x08;
pub const SAB82532_DAFO_PAR_EVEN: c_uint = 0x10;
pub const SAB82532_DAFO_PAR_MARK: c_uint = 0x18;
pub const SAB82532_DAFO_PARE: c_uint = 0x04;
pub const SAB82532_DAFO_CHL8: c_uint = 0x00;
pub const SAB82532_DAFO_CHL7: c_uint = 0x01;
pub const SAB82532_DAFO_CHL6: c_uint = 0x02;
pub const SAB82532_DAFO_CHL5: c_uint = 0x03;
// RFIFO Control Register (RFC)
pub const SAB82532_RFC_DPS: c_uint = 0x40;
pub const SAB82532_RFC_DXS: c_uint = 0x20;
pub const SAB82532_RFC_RFDF: c_uint = 0x10;
pub const SAB82532_RFC_RFTH_1: c_uint = 0x00;
pub const SAB82532_RFC_RFTH_4: c_uint = 0x04;
pub const SAB82532_RFC_RFTH_16: c_uint = 0x08;
pub const SAB82532_RFC_RFTH_32: c_uint = 0x0c;
pub const SAB82532_RFC_TCDE: c_uint = 0x01;
// Received Byte Count High (RBCH)
pub const SAB82532_RBCH_DMA: c_uint = 0x80;
pub const SAB82532_RBCH_CAS: c_uint = 0x20;
// Transmit Byte Count High (XBCH)
pub const SAB82532_XBCH_DMA: c_uint = 0x80;
pub const SAB82532_XBCH_CAS: c_uint = 0x20;
pub const SAB82532_XBCH_XC: c_uint = 0x10;
// Channel Configuration Register 0 (CCR0)
pub const SAB82532_CCR0_PU: c_uint = 0x80;
pub const SAB82532_CCR0_MCE: c_uint = 0x40;
pub const SAB82532_CCR0_SC_NRZ: c_uint = 0x00;
pub const SAB82532_CCR0_SC_NRZI: c_uint = 0x08;
pub const SAB82532_CCR0_SC_FM0: c_uint = 0x10;
pub const SAB82532_CCR0_SC_FM1: c_uint = 0x14;
pub const SAB82532_CCR0_SC_MANCH: c_uint = 0x18;
pub const SAB82532_CCR0_SM_HDLC: c_uint = 0x00;
pub const SAB82532_CCR0_SM_SDLC_LOOP: c_uint = 0x01;
pub const SAB82532_CCR0_SM_BISYNC: c_uint = 0x02;
pub const SAB82532_CCR0_SM_ASYNC: c_uint = 0x03;
// Channel Configuration Register 1 (CCR1)
pub const SAB82532_CCR1_ODS: c_uint = 0x10;
pub const SAB82532_CCR1_BCR: c_uint = 0x08;
pub const SAB82532_CCR1_CM_MASK: c_uint = 0x07;
// Channel Configuration Register 2 (CCR2)
pub const SAB82532_CCR2_SOC1: c_uint = 0x80;
pub const SAB82532_CCR2_SOC0: c_uint = 0x40;
pub const SAB82532_CCR2_BR9: c_uint = 0x80;
pub const SAB82532_CCR2_BR8: c_uint = 0x40;
pub const SAB82532_CCR2_BDF: c_uint = 0x20;
pub const SAB82532_CCR2_SSEL: c_uint = 0x10;
pub const SAB82532_CCR2_XCS0: c_uint = 0x20;
pub const SAB82532_CCR2_RCS0: c_uint = 0x10;
pub const SAB82532_CCR2_TOE: c_uint = 0x08;
pub const SAB82532_CCR2_RWX: c_uint = 0x04;
pub const SAB82532_CCR2_DIV: c_uint = 0x01;
// Channel Configuration Register 3 (CCR3)
pub const SAB82532_CCR3_PSD: c_uint = 0x01;
// Time Slot Assignment Register Transmit (TSAX)
pub const SAB82532_TSAX_TSNX_MASK: c_uint = 0xfc;
pub const SAB82532_TSAX_XCS2: c_uint = 0x02	/* see also CCR2 */;
pub const SAB82532_TSAX_XCS1: c_uint = 0x01;
// Time Slot Assignment Register Receive (TSAR)
pub const SAB82532_TSAR_TSNR_MASK: c_uint = 0xfc;
pub const SAB82532_TSAR_RCS2: c_uint = 0x02	/* see also CCR2 */;
pub const SAB82532_TSAR_RCS1: c_uint = 0x01;
// Version Status Register (VSTR)
pub const SAB82532_VSTR_CD: c_uint = 0x80;
pub const SAB82532_VSTR_DPLA: c_uint = 0x40;
pub const SAB82532_VSTR_VN_MASK: c_uint = 0x0f;
pub const SAB82532_VSTR_VN_1: c_uint = 0x00;
pub const SAB82532_VSTR_VN_2: c_uint = 0x01;
pub const SAB82532_VSTR_VN_3_2: c_uint = 0x02;
// Global Interrupt Status Register (GIS)
pub const SAB82532_GIS_PI: c_uint = 0x80;
pub const SAB82532_GIS_ISA1: c_uint = 0x08;
pub const SAB82532_GIS_ISA0: c_uint = 0x04;
pub const SAB82532_GIS_ISB1: c_uint = 0x02;
pub const SAB82532_GIS_ISB0: c_uint = 0x01;
// Interrupt Vector Address (IVA)
pub const SAB82532_IVA_MASK: c_uint = 0xf1;
// Interrupt Port Configuration (IPC)
pub const SAB82532_IPC_VIS: c_uint = 0x80;
pub const SAB82532_IPC_SLA1: c_uint = 0x10;
pub const SAB82532_IPC_SLA0: c_uint = 0x08;
pub const SAB82532_IPC_CASM: c_uint = 0x04;
pub const SAB82532_IPC_IC_OPEN_DRAIN: c_uint = 0x00;
pub const SAB82532_IPC_IC_ACT_LOW: c_uint = 0x01;
pub const SAB82532_IPC_IC_ACT_HIGH: c_uint = 0x03;
// Interrupt Status Register 0 (ISR0)
pub const SAB82532_ISR0_TCD: c_uint = 0x80;
pub const SAB82532_ISR0_TIME: c_uint = 0x40;
pub const SAB82532_ISR0_PERR: c_uint = 0x20;
pub const SAB82532_ISR0_FERR: c_uint = 0x10;
pub const SAB82532_ISR0_PLLA: c_uint = 0x08;
pub const SAB82532_ISR0_CDSC: c_uint = 0x04;
pub const SAB82532_ISR0_RFO: c_uint = 0x02;
pub const SAB82532_ISR0_RPF: c_uint = 0x01;
// Interrupt Status Register 1 (ISR1)
pub const SAB82532_ISR1_BRK: c_uint = 0x80;
pub const SAB82532_ISR1_BRKT: c_uint = 0x40;
pub const SAB82532_ISR1_ALLS: c_uint = 0x20;
pub const SAB82532_ISR1_XOFF: c_uint = 0x10;
pub const SAB82532_ISR1_TIN: c_uint = 0x08;
pub const SAB82532_ISR1_CSC: c_uint = 0x04;
pub const SAB82532_ISR1_XON: c_uint = 0x02;
pub const SAB82532_ISR1_XPR: c_uint = 0x01;
// Interrupt Mask Register 0 (IMR0)
pub const SAB82532_IMR0_TCD: c_uint = 0x80;
pub const SAB82532_IMR0_TIME: c_uint = 0x40;
pub const SAB82532_IMR0_PERR: c_uint = 0x20;
pub const SAB82532_IMR0_FERR: c_uint = 0x10;
pub const SAB82532_IMR0_PLLA: c_uint = 0x08;
pub const SAB82532_IMR0_CDSC: c_uint = 0x04;
pub const SAB82532_IMR0_RFO: c_uint = 0x02;
pub const SAB82532_IMR0_RPF: c_uint = 0x01;
// Interrupt Mask Register 1 (IMR1)
pub const SAB82532_IMR1_BRK: c_uint = 0x80;
pub const SAB82532_IMR1_BRKT: c_uint = 0x40;
pub const SAB82532_IMR1_ALLS: c_uint = 0x20;
pub const SAB82532_IMR1_XOFF: c_uint = 0x10;
pub const SAB82532_IMR1_TIN: c_uint = 0x08;
pub const SAB82532_IMR1_CSC: c_uint = 0x04;
pub const SAB82532_IMR1_XON: c_uint = 0x02;
pub const SAB82532_IMR1_XPR: c_uint = 0x01;
// Port Interrupt Status Register (PIS)
pub const SAB82532_PIS_SYNC_B: c_uint = 0x08;
pub const SAB82532_PIS_DTR_B: c_uint = 0x04;
pub const SAB82532_PIS_DTR_A: c_uint = 0x02;
pub const SAB82532_PIS_SYNC_A: c_uint = 0x01;
// Channel Configuration Register 4 (CCR4)
pub const SAB82532_CCR4_MCK4: c_uint = 0x80;
pub const SAB82532_CCR4_EBRG: c_uint = 0x40;
pub const SAB82532_CCR4_TST1: c_uint = 0x20;
pub const SAB82532_CCR4_ICD: c_uint = 0x10;
