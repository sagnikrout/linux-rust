//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/initio.h
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
// Initio 9100 device driver for Linux.
//
// Copyright (c) 1994-1998 Initio Corporation
// All rights reserved.
//
// Cleanups (c) Copyright 2007 Red Hat <alan@lxorguk.ukuu.org.uk>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, write to
// the Free Software Foundation, 675 Mass Ave, Cambridge, MA 02139, USA.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

pub const TOTAL_SG_ENTRY: c_int = 32;
pub const MAX_SUPPORTED_ADAPTERS: c_int = 8;
pub const MAX_OFFSET: c_int = 15;
pub const MAX_TARGETS: c_int = 16;
//
// Tulip Configuration Register Set
//
pub const TUL_PVID: c_uint = 0x00	/* Vendor ID                    */;
pub const TUL_PDID: c_uint = 0x02	/* Device ID                    */;
pub const TUL_PCMD: c_uint = 0x04	/* Command                      */;
pub const TUL_PSTUS: c_uint = 0x06	/* Status                       */;
pub const TUL_PRID: c_uint = 0x08	/* Revision number              */;
pub const TUL_PPI: c_uint = 0x09	/* Programming interface        */;
pub const TUL_PSC: c_uint = 0x0A	/* Sub Class                    */;
pub const TUL_PBC: c_uint = 0x0B	/* Base Class                   */;
pub const TUL_PCLS: c_uint = 0x0C	/* Cache line size              */;
pub const TUL_PLTR: c_uint = 0x0D	/* Latency timer                */;
pub const TUL_PHDT: c_uint = 0x0E	/* Header type                  */;
pub const TUL_PBIST: c_uint = 0x0F	/* BIST                         */;
pub const TUL_PBAD: c_uint = 0x10	/* Base address                 */;
pub const TUL_PBAD1: c_uint = 0x14	/* Base address                 */;
pub const TUL_PBAD2: c_uint = 0x18	/* Base address                 */;
pub const TUL_PBAD3: c_uint = 0x1C	/* Base address                 */;
pub const TUL_PBAD4: c_uint = 0x20	/* Base address                 */;
pub const TUL_PBAD5: c_uint = 0x24	/* Base address                 */;
pub const TUL_PRSVD: c_uint = 0x28	/* Reserved                     */;
pub const TUL_PRSVD1: c_uint = 0x2C	/* Reserved                     */;
pub const TUL_PRAD: c_uint = 0x30	/* Expansion ROM base address   */;
pub const TUL_PRSVD2: c_uint = 0x34	/* Reserved                     */;
pub const TUL_PRSVD3: c_uint = 0x38	/* Reserved                     */;
pub const TUL_PINTL: c_uint = 0x3C	/* Interrupt line               */;
pub const TUL_PINTP: c_uint = 0x3D	/* Interrupt pin                */;
pub const TUL_PIGNT: c_uint = 0x3E	/* MIN_GNT                      */;
pub const TUL_PMGNT: c_uint = 0x3F	/* MAX_GNT                      */;
//
// Jasmin Register Set
//
pub const TUL_HACFG0: c_uint = 0x40	/* H/A Configuration Register 0         */;
pub const TUL_HACFG1: c_uint = 0x41	/* H/A Configuration Register 1         */;
pub const TUL_HACFG2: c_uint = 0x42	/* H/A Configuration Register 2         */;
pub const TUL_SDCFG0: c_uint = 0x44	/* SCSI Device Configuration 0          */;
pub const TUL_SDCFG1: c_uint = 0x45	/* SCSI Device Configuration 1          */;
pub const TUL_SDCFG2: c_uint = 0x46	/* SCSI Device Configuration 2          */;
pub const TUL_SDCFG3: c_uint = 0x47	/* SCSI Device Configuration 3          */;
pub const TUL_GINTS: c_uint = 0x50	/* Global Interrupt Status Register     */;
pub const TUL_GIMSK: c_uint = 0x52	/* Global Interrupt MASK Register       */;
pub const TUL_GCTRL: c_uint = 0x54	/* Global Control Register              */;
pub const TUL_GCTRL_EEPROM_BIT: c_uint = 0x04;
pub const TUL_GCTRL1: c_uint = 0x55	/* Global Control Register              */;
pub const TUL_DMACFG: c_uint = 0x5B	/* DMA configuration                    */;
pub const TUL_NVRAM: c_uint = 0x5D	/* Non-volatile RAM port                */;
pub const TUL_SCnt0: c_uint = 0x80	/* 00 R/W Transfer Counter Low          */;
pub const TUL_SCnt1: c_uint = 0x81	/* 01 R/W Transfer Counter Mid          */;
pub const TUL_SCnt2: c_uint = 0x82	/* 02 R/W Transfer Count High           */;
pub const TUL_SFifoCnt: c_uint = 0x83	/* 03 R   FIFO counter                  */;
pub const TUL_SIntEnable: c_uint = 0x84	/* 03 W   Interrupt enble               */;
pub const TUL_SInt: c_uint = 0x84	/* 04 R   Interrupt Register            */;
pub const TUL_SCtrl0: c_uint = 0x85	/* 05 W   Control 0                     */;
pub const TUL_SStatus0: c_uint = 0x85	/* 05 R   Status 0                      */;
pub const TUL_SCtrl1: c_uint = 0x86	/* 06 W   Control 1                     */;
pub const TUL_SStatus1: c_uint = 0x86	/* 06 R   Status 1                      */;
pub const TUL_SConfig: c_uint = 0x87	/* 07 W   Configuration                 */;
pub const TUL_SStatus2: c_uint = 0x87	/* 07 R   Status 2                      */;
pub const TUL_SPeriod: c_uint = 0x88	/* 08 W   Sync. Transfer Period & Offset */;
pub const TUL_SOffset: c_uint = 0x88	/* 08 R   Offset                        */;
pub const TUL_SScsiId: c_uint = 0x89	/* 09 W   SCSI ID                       */;
pub const TUL_SBusId: c_uint = 0x89	/* 09 R   SCSI BUS ID                   */;
pub const TUL_STimeOut: c_uint = 0x8A	/* 0A W   Sel/Resel Time Out Register   */;
pub const TUL_SIdent: c_uint = 0x8A	/* 0A R   Identify Message Register     */;
pub const TUL_SAvail: c_uint = 0x8A	/* 0A R   Available Counter Register   */;
pub const TUL_SData: c_uint = 0x8B	/* 0B R/W SCSI data in/out              */;
pub const TUL_SFifo: c_uint = 0x8C	/* 0C R/W FIFO                          */;
pub const TUL_SSignal: c_uint = 0x90	/* 10 R/W SCSI signal in/out            */;
pub const TUL_SCmd: c_uint = 0x91	/* 11 R/W Command                       */;
pub const TUL_STest0: c_uint = 0x92	/* 12 R/W Test0                         */;
pub const TUL_STest1: c_uint = 0x93	/* 13 R/W Test1                         */;
pub const TUL_SCFG1: c_uint = 0x94	/* 14 R/W Configuration                 */;
pub const TUL_XAddH: c_uint = 0xC0	/*DMA Transfer Physical Address         */;
pub const TUL_XAddW: c_uint = 0xC8	/*DMA Current Transfer Physical Address */;
pub const TUL_XCntH: c_uint = 0xD0	/*DMA Transfer Counter                  */;
pub const TUL_XCntW: c_uint = 0xD4	/*DMA Current Transfer Counter          */;
pub const TUL_XCmd: c_uint = 0xD8	/*DMA Command Register                  */;
pub const TUL_Int: c_uint = 0xDC	/*Interrupt Register                    */;
pub const TUL_XStatus: c_uint = 0xDD	/*DMA status Register                   */;
pub const TUL_Mask: c_uint = 0xE0	/*Interrupt Mask Register               */;
pub const TUL_XCtrl: c_uint = 0xE4	/*DMA Control Register                  */;
pub const TUL_XCtrl1: c_uint = 0xE5	/*DMA Control Register 1                */;
pub const TUL_XFifo: c_uint = 0xE8	/*DMA FIFO                              */;
pub const TUL_WCtrl: c_uint = 0xF7	/*Bus master wait state control         */;
pub const TUL_DCtrl: c_uint = 0xFB	/*DMA delay control                     */;
// ----------------------------------------------------------------------
// bit definition for Command register of Configuration Space Header
// ----------------------------------------------------------------------
pub const BUSMS: c_uint = 0x04	/* BUS MASTER Enable                    */;
pub const IOSPA: c_uint = 0x01	/* IO Space Enable                      */;
// ----------------------------------------------------------------------
// Command Codes of Tulip SCSI Command register
// ----------------------------------------------------------------------
pub const TSC_EN_RESEL: c_uint = 0x80	/* Enable Reselection                   */;
pub const TSC_CMD_COMP: c_uint = 0x84	/* Command Complete Sequence            */;
pub const TSC_SEL: c_uint = 0x01	/* Select Without ATN Sequence          */;
pub const TSC_SEL_ATN: c_uint = 0x11	/* Select With ATN Sequence             */;
pub const TSC_SEL_ATN_DMA: c_uint = 0x51	/* Select With ATN Sequence with DMA    */;
pub const TSC_SEL_ATN3: c_uint = 0x31	/* Select With ATN3 Sequence            */;
pub const TSC_SEL_ATNSTOP: c_uint = 0x12	/* Select With ATN and Stop Sequence    */;
pub const TSC_SELATNSTOP: c_uint = 0x1E	/* Select With ATN and Stop Sequence    */;
pub const TSC_SEL_ATN_DIRECT_IN: c_uint = 0x95	/* Select With ATN Sequence     */;
pub const TSC_SEL_ATN_DIRECT_OUT: c_uint = 0x15	/* Select With ATN Sequence     */;
pub const TSC_SEL_ATN3_DIRECT_IN: c_uint = 0xB5	/* Select With ATN3 Sequence    */;
pub const TSC_SEL_ATN3_DIRECT_OUT: c_uint = 0x35	/* Select With ATN3 Sequence    */;
pub const TSC_XF_DMA_OUT_DIRECT: c_uint = 0x06	/* DMA Xfer Information out      */;
pub const TSC_XF_DMA_IN_DIRECT: c_uint = 0x86	/* DMA Xfer Information in       */;
pub const TSC_XF_DMA_OUT: c_uint = 0x43	/* DMA Xfer Information out              */;
pub const TSC_XF_DMA_IN: c_uint = 0xC3	/* DMA Xfer Information in               */;
pub const TSC_XF_FIFO_OUT: c_uint = 0x03	/* FIFO Xfer Information out             */;
pub const TSC_XF_FIFO_IN: c_uint = 0x83	/* FIFO Xfer Information in              */;
pub const TSC_MSG_ACCEPT: c_uint = 0x0F	/* Message Accept                       */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Control 0 Register
// ----------------------------------------------------------------------
pub const TSC_RST_SEQ: c_uint = 0x20	/* Reset sequence counter               */;
pub const TSC_FLUSH_FIFO: c_uint = 0x10	/* Flush FIFO                           */;
pub const TSC_ABT_CMD: c_uint = 0x04	/* Abort command (sequence)             */;
pub const TSC_RST_CHIP: c_uint = 0x02	/* Reset SCSI Chip                      */;
pub const TSC_RST_BUS: c_uint = 0x01	/* Reset SCSI Bus                       */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Control 1 Register
// ----------------------------------------------------------------------
pub const TSC_EN_SCAM: c_uint = 0x80	/* Enable SCAM                          */;
pub const TSC_TIMER: c_uint = 0x40	/* Select timeout unit                  */;
pub const TSC_EN_SCSI2: c_uint = 0x20	/* SCSI-2 mode                          */;
pub const TSC_PWDN: c_uint = 0x10	/* Power down mode                      */;
pub const TSC_WIDE_CPU: c_uint = 0x08	/* Wide CPU                             */;
pub const TSC_HW_RESELECT: c_uint = 0x04	/* Enable HW reselect                   */;
pub const TSC_EN_BUS_OUT: c_uint = 0x02	/* Enable SCSI data bus out latch       */;
pub const TSC_EN_BUS_IN: c_uint = 0x01	/* Enable SCSI data bus in latch        */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Configuration Register
// ----------------------------------------------------------------------
pub const TSC_EN_LATCH: c_uint = 0x80	/* Enable phase latch                   */;
pub const TSC_INITIATOR: c_uint = 0x40	/* Initiator mode                       */;
pub const TSC_EN_SCSI_PAR: c_uint = 0x20	/* Enable SCSI parity                   */;
pub const TSC_DMA_8BIT: c_uint = 0x10	/* Alternate dma 8-bits mode            */;
pub const TSC_DMA_16BIT: c_uint = 0x08	/* Alternate dma 16-bits mode           */;
pub const TSC_EN_WDACK: c_uint = 0x04	/* Enable DACK while wide SCSI xfer     */;
pub const TSC_ALT_PERIOD: c_uint = 0x02	/* Alternate sync period mode           */;
pub const TSC_DIS_SCSIRST: c_uint = 0x01	/* Disable SCSI bus reset us            */;

pub const TSC_WIDE_SCSI: c_uint = 0x80	/* Enable Wide SCSI                     */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI signal Register
// ----------------------------------------------------------------------
pub const TSC_RST_ACK: c_uint = 0x00	/* Release ACK signal                   */;
pub const TSC_RST_ATN: c_uint = 0x00	/* Release ATN signal                   */;
pub const TSC_RST_BSY: c_uint = 0x00	/* Release BSY signal                   */;
pub const TSC_SET_ACK: c_uint = 0x40	/* ACK signal                           */;
pub const TSC_SET_ATN: c_uint = 0x08	/* ATN signal                           */;
pub const TSC_REQI: c_uint = 0x80	/* REQ signal                           */;
pub const TSC_ACKI: c_uint = 0x40	/* ACK signal                           */;
pub const TSC_BSYI: c_uint = 0x20	/* BSY signal                           */;
pub const TSC_SELI: c_uint = 0x10	/* SEL signal                           */;
pub const TSC_ATNI: c_uint = 0x08	/* ATN signal                           */;
pub const TSC_MSGI: c_uint = 0x04	/* MSG signal                           */;
pub const TSC_CDI: c_uint = 0x02	/* C/D signal                           */;
pub const TSC_IOI: c_uint = 0x01	/* I/O signal                           */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Status 0 Register
// ----------------------------------------------------------------------
pub const TSS_INT_PENDING: c_uint = 0x80	/* Interrupt pending            */;
pub const TSS_SEQ_ACTIVE: c_uint = 0x40	/* Sequencer active             */;
pub const TSS_XFER_CNT: c_uint = 0x20	/* Transfer counter zero        */;
pub const TSS_FIFO_EMPTY: c_uint = 0x10	/* FIFO empty                   */;
pub const TSS_PAR_ERROR: c_uint = 0x08	/* SCSI parity error            */;
pub const TSS_PH_MASK: c_uint = 0x07	/* SCSI phase mask              */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Status 1 Register
// ----------------------------------------------------------------------
pub const TSS_STATUS_RCV: c_uint = 0x08	/* Status received              */;
pub const TSS_MSG_SEND: c_uint = 0x40	/* Message sent                 */;
pub const TSS_CMD_PH_CMP: c_uint = 0x20	/* command phase done              */;
pub const TSS_DATA_PH_CMP: c_uint = 0x10	/* Data phase done              */;
pub const TSS_STATUS_SEND: c_uint = 0x08	/* Status sent                  */;
pub const TSS_XFER_CMP: c_uint = 0x04	/* Transfer completed           */;
pub const TSS_SEL_CMP: c_uint = 0x02	/* Selection completed          */;
pub const TSS_ARB_CMP: c_uint = 0x01	/* Arbitration completed        */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Status 2 Register
// ----------------------------------------------------------------------
pub const TSS_CMD_ABTED: c_uint = 0x80	/* Command aborted              */;
pub const TSS_OFFSET_0: c_uint = 0x40	/* Offset counter zero          */;
pub const TSS_FIFO_FULL: c_uint = 0x20	/* FIFO full                    */;
pub const TSS_TIMEOUT_0: c_uint = 0x10	/* Timeout counter zero         */;
pub const TSS_BUSY_RLS: c_uint = 0x08	/* Busy release                 */;
pub const TSS_PH_MISMATCH: c_uint = 0x04	/* Phase mismatch               */;
pub const TSS_SCSI_BUS_EN: c_uint = 0x02	/* SCSI data bus enable         */;
pub const TSS_SCSIRST: c_uint = 0x01	/* SCSI bus reset in progress   */;
// ----------------------------------------------------------------------
// bit definition for Tulip SCSI Interrupt Register
// ----------------------------------------------------------------------
pub const TSS_RESEL_INT: c_uint = 0x80	/* Reselected interrupt         */;
pub const TSS_SEL_TIMEOUT: c_uint = 0x40	/* Selected/reselected timeout  */;
pub const TSS_BUS_SERV: c_uint = 0x20;
pub const TSS_SCSIRST_INT: c_uint = 0x10	/* SCSI bus reset detected      */;
pub const TSS_DISC_INT: c_uint = 0x08	/* Disconnected interrupt       */;
pub const TSS_SEL_INT: c_uint = 0x04	/* Select interrupt             */;
pub const TSS_SCAM_SEL: c_uint = 0x02	/* SCAM selected                */;
pub const TSS_FUNC_COMP: c_uint = 0x01;
// ----------------------------------------------------------------------
// SCSI Phase Codes.
// ----------------------------------------------------------------------
pub const DATA_OUT: c_int = 0;

pub const CMD_OUT: c_int = 2;

pub const MSG_IN: c_int = 7;
// ----------------------------------------------------------------------
// Command Codes of Tulip xfer Command register
// ----------------------------------------------------------------------
pub const TAX_X_FORC: c_uint = 0x02;
pub const TAX_X_ABT: c_uint = 0x04;
pub const TAX_X_CLR_FIFO: c_uint = 0x08;
pub const TAX_X_IN: c_uint = 0x21;
pub const TAX_X_OUT: c_uint = 0x01;
pub const TAX_SG_IN: c_uint = 0xA1;
pub const TAX_SG_OUT: c_uint = 0x81;
// ----------------------------------------------------------------------
// Tulip Interrupt Register
// ----------------------------------------------------------------------
pub const XCMP: c_uint = 0x01;
pub const FCMP: c_uint = 0x02;
pub const XABT: c_uint = 0x04;
pub const XERR: c_uint = 0x08;
pub const SCMP: c_uint = 0x10;
pub const IPEND: c_uint = 0x80;
// ----------------------------------------------------------------------
// Tulip DMA Status Register
// ----------------------------------------------------------------------
pub const XPEND: c_uint = 0x01	/* Transfer pending             */;
pub const FEMPTY: c_uint = 0x02	/* FIFO empty                   */;
// ----------------------------------------------------------------------
// bit definition for TUL_GCTRL
// ----------------------------------------------------------------------
pub const EXTSG: c_uint = 0x80;
pub const EXTAD: c_uint = 0x60;
pub const SEG4K: c_uint = 0x08;
pub const EEPRG: c_uint = 0x04;
pub const MRMUL: c_uint = 0x02;
// ----------------------------------------------------------------------
// bit definition for TUL_NVRAM
// ----------------------------------------------------------------------
pub const SE2CS: c_uint = 0x08;
pub const SE2CLK: c_uint = 0x04;
pub const SE2DO: c_uint = 0x02;
pub const SE2DI: c_uint = 0x01;
//
// Scatter-Gather Element Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_entry {
    pub /: *mut *mut u32 data; / Data Pointer,
    pub /: *mut *mut u32 len; / Data Length,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_ctrl_blk {
    pub next: *mut scsi_ctrl_blk,
    pub /: *mut *mut u8 status; /4,
    pub /: *mut *mut u8 next_state; /5,
    pub /: *mut *mut u8 mode; /6,
    pub /: *mut *mut u8 msgin; /7 SCB_Res0,
    pub /: *mut *mut u16 sgidx; /8,
    pub /: *mut *mut u16 sgmax; /A,

    pub /: *mut *mut u32 reserved[2]; /C,

    pub /: *mut *mut u32 reserved[3]; /C,

    pub /: *mut *mut u32 xferlen; /18 Current xfer len,
    pub /: *mut *mut u32 totxlen; /1C Total xfer len,
    pub /: *mut *mut u32 paddr; /20 SCB phy. Addr.,
    pub /: *mut *mut u8 opcode; /24 SCB command code,
    pub /: *mut *mut u8 flags; /25 SCB Flags,
    pub /: *mut *mut u8 target; /26 Target Id,
    pub /: *mut *mut u8 lun; /27 Lun,
    pub /: *mut *mut u32 bufptr; /28 Data Buffer Pointer,
    pub /: *mut *mut u32 buflen; /2C Data Allocation Length,
    pub /: *mut *mut u8 sglen; /30 SG list #,
    pub /: *mut *mut u8 senselen; /31 Sense Allocation Length,
    pub /: *mut *mut u8 hastat; /32,
    pub /: *mut *mut u8 tastat; /33,
    pub /: *mut *mut u8 cdblen; /34 CDB Length,
    pub /: *mut *mut u8 ident; /35 Identify,
    pub /: *mut *mut u8 tagmsg; /36 Tag Message,
    pub /: *mut *mut u8 tagid; /37 Queue Tag,
    pub /: *mut *mut u8 cdb[12]; /38,
    pub /: *mut *mut u32 sgpaddr; /44 SG List/Sense Buf phy. Addr.,
    pub /: *mut *mut u32 senseptr; /48 Sense data pointer,
    pub /: *mut *mut *mut *mut *mut void (post) (u8 , u8 ); /4C POST routine,
    pub /: *mut *mut *mut scsi_cmnd srb; /50 SRB Pointer,
    pub /: *mut *mut sg_entry sglist[TOTAL_SG_ENTRY]; /54 Start of SG list,
}

// Bit Definition for status
pub const SCB_RENT: c_uint = 0x01;
pub const SCB_PEND: c_uint = 0x02;
pub const SCB_CONTIG: c_uint = 0x04	/* Contingent Allegiance */;
pub const SCB_SELECT: c_uint = 0x08;
pub const SCB_BUSY: c_uint = 0x10;
pub const SCB_DONE: c_uint = 0x20;
// Opcodes for opcode
pub const ExecSCSI: c_uint = 0x1;
pub const BusDevRst: c_uint = 0x2;
pub const AbortCmd: c_uint = 0x3;
// Bit Definition for mode
pub const SCM_RSENS: c_uint = 0x01	/* request sense mode */;
// Bit Definition for flags
pub const SCF_DONE: c_uint = 0x01;
pub const SCF_POST: c_uint = 0x02;
pub const SCF_SENSE: c_uint = 0x04;
pub const SCF_DIR: c_uint = 0x18;
pub const SCF_NO_DCHK: c_uint = 0x00;
pub const SCF_DIN: c_uint = 0x08;
pub const SCF_DOUT: c_uint = 0x10;
pub const SCF_NO_XF: c_uint = 0x18;
pub const SCF_WR_VF: c_uint = 0x20	/* Write verify turn on         */;
pub const SCF_POLL: c_uint = 0x40;
pub const SCF_SG: c_uint = 0x80;
// Error Codes for SCB_HaStat
pub const HOST_SEL_TOUT: c_uint = 0x11;
pub const HOST_DO_DU: c_uint = 0x12;
pub const HOST_BUS_FREE: c_uint = 0x13;
pub const HOST_BAD_PHAS: c_uint = 0x14;
pub const HOST_INV_CMD: c_uint = 0x16;
pub const HOST_ABORTED: c_uint = 0x1A	/* 07/21/98 */;
pub const HOST_SCSI_RST: c_uint = 0x1B;
pub const HOST_DEV_RST: c_uint = 0x1C;
// Error Codes for SCB_TaStat
pub const TARGET_CHKCOND: c_uint = 0x02;
pub const TARGET_BUSY: c_uint = 0x08;
pub const INI_QUEUE_FULL: c_uint = 0x28;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_control {
    pub flags: u16,
    pub js_period: u8,
    pub sconfig0: u8,
    pub drv_flags: u16,
    pub heads: u8,
    pub sectors: u8,
}

//
// Bit Definition for TCF_Flags
pub const TCF_SCSI_RATE: c_uint = 0x0007;
pub const TCF_EN_DISC: c_uint = 0x0008;
pub const TCF_NO_SYNC_NEGO: c_uint = 0x0010;
pub const TCF_NO_WDTR: c_uint = 0x0020;
pub const TCF_EN_255: c_uint = 0x0040;
pub const TCF_EN_START: c_uint = 0x0080;
pub const TCF_WDTR_DONE: c_uint = 0x0100;
pub const TCF_SYNC_DONE: c_uint = 0x0200;
pub const TCF_BUSY: c_uint = 0x0400;
// Bit Definition for TCF_DrvFlags
pub const TCF_DRV_BUSY: c_uint = 0x01	/* Indicate target busy(driver) */;
pub const TCF_DRV_EN_TAG: c_uint = 0x0800;
pub const TCF_DRV_255_63: c_uint = 0x0400;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initio_host {
    pub /: *mut *mut u16 addr; / 00,
    pub /: *mut *mut u16 bios_addr; / 02,
    pub /: *mut *mut u8 irq; / 04,
    pub /: *mut *mut u8 scsi_id; / 05,
    pub /: *mut *mut u8 max_tar; / 06,
    pub /: *mut *mut u8 num_scbs; / 07,
    pub /: *mut *mut u8 flags; / 08,
    pub /: *mut *mut u8 index; / 09,
    pub /: *mut *mut u8 ha_id; / 0A,
    pub /: *mut *mut u8 config; / 0B,
    pub /: *mut *mut u16 idmask; / 0C,
    pub /: *mut *mut u8 semaph; / 0E,
    pub /: *mut *mut u8 phase; / 0F,
    pub /: *mut *mut u8 jsstatus0; / 10,
    pub /: *mut *mut u8 jsint; / 11,
    pub /: *mut *mut u8 jsstatus1; / 12,
    pub /: *mut *mut u8 sconf1; / 13,
    pub /: *mut *mut u8 msg[8]; / 14,
    pub /: *mut *mut *mut scsi_ctrl_blk next_avail; / 1C,
    pub /: *mut *mut *mut scsi_ctrl_blk scb; / 20,
    pub /*UNUSED*/: *mut *mut *mut *mut scsi_ctrl_blk scb_end; / 24 /,
    pub /: *mut *mut *mut scsi_ctrl_blk next_pending; / 28,
    pub /*UNUSED*/: *mut *mut *mut *mut scsi_ctrl_blk next_contig; / 2C /,
    pub /: *mut *mut *mut scsi_ctrl_blk active; / 30,
    pub /: *mut *mut *mut target_control active_tc; / 34,
    pub /: *mut *mut *mut scsi_ctrl_blk first_avail; / 38,
    pub /: *mut *mut *mut scsi_ctrl_blk last_avail; / 3C,
    pub /: *mut *mut *mut scsi_ctrl_blk first_pending; / 40,
    pub /: *mut *mut *mut scsi_ctrl_blk last_pending; / 44,
    pub /: *mut *mut *mut scsi_ctrl_blk first_busy; / 48,
    pub /: *mut *mut *mut scsi_ctrl_blk last_busy; / 4C,
    pub /: *mut *mut *mut scsi_ctrl_blk first_done; / 50,
    pub /: *mut *mut *mut scsi_ctrl_blk last_done; / 54,
    pub /: *mut *mut u8 max_tags[16]; / 58,
    pub /: *mut *mut u8 act_tags[16]; / 68,
    pub /: *mut *mut target_control targets[MAX_TARGETS]; / 78,
    pub avail_lock: spinlock_t,
    pub semaph_lock: spinlock_t,
    pub pci_dev: *mut pci_dev,
}

// Bit Definition for HCB_Config
pub const HCC_SCSI_RESET: c_uint = 0x01;
pub const HCC_EN_PAR: c_uint = 0x02;
pub const HCC_ACT_TERM1: c_uint = 0x04;
pub const HCC_ACT_TERM2: c_uint = 0x08;
pub const HCC_AUTO_TERM: c_uint = 0x10;
pub const HCC_EN_PWR: c_uint = 0x80;
// Bit Definition for HCB_Flags
pub const HCF_EXPECT_DISC: c_uint = 0x01;
pub const HCF_EXPECT_SELECT: c_uint = 0x02;
pub const HCF_EXPECT_RESET: c_uint = 0x10;
pub const HCF_EXPECT_DONE_DISC: c_uint = 0x20;
//
// SCSI target configuration
// ----------header ---------------
// ----Host Adapter Structure ----
// ---------- CheckSum ----------
// Bios Configuration for nvram->BIOSConfig1
pub const NBC1_ENABLE: c_uint = 0x01	/* BIOS enable                  */;
pub const NBC1_8DRIVE: c_uint = 0x02	/* Support more than 2 drives   */;
pub const NBC1_REMOVABLE: c_uint = 0x04	/* Support removable drive      */;
pub const NBC1_INT19: c_uint = 0x08	/* Intercept int 19h            */;
pub const NBC1_BIOSSCAN: c_uint = 0x10	/* Dynamic BIOS scan            */;
pub const NBC1_LUNSUPPORT: c_uint = 0x40	/* Support LUN                  */;
// HA Configuration Byte 1
pub const NHC1_BOOTIDMASK: c_uint = 0x0F	/* Boot ID number               */;
pub const NHC1_LUNMASK: c_uint = 0x70	/* Boot LUN number              */;
pub const NHC1_CHANMASK: c_uint = 0x80	/* Boot Channel number          */;
// Bit definition for nvram->SCSIconfig1
pub const NCC1_BUSRESET: c_uint = 0x01	/* Reset SCSI bus at power up   */;
pub const NCC1_PARITYCHK: c_uint = 0x02	/* SCSI parity enable           */;
pub const NCC1_ACTTERM1: c_uint = 0x04	/* Enable active terminator 1   */;
pub const NCC1_ACTTERM2: c_uint = 0x08	/* Enable active terminator 2   */;
pub const NCC1_AUTOTERM: c_uint = 0x10	/* Enable auto terminator       */;
pub const NCC1_PWRMGR: c_uint = 0x80	/* Enable power management      */;
// Bit definition for SCSI Target configuration byte
pub const NTC_DISCONNECT: c_uint = 0x08	/* Enable SCSI disconnect       */;
pub const NTC_SYNC: c_uint = 0x10	/* SYNC_NEGO                    */;
pub const NTC_NO_WDTR: c_uint = 0x20	/* SYNC_NEGO                    */;
pub const NTC_1GIGA: c_uint = 0x40	/* 255 head / 63 sectors (64/32) */;
pub const NTC_SPINUP: c_uint = 0x80	/* Start disk drive             */;
// Default NVRam values
pub const INI_SIGNATURE: c_uint = 0xC925;

// SCSI related definition
pub const DISC_NOT_ALLOW: c_uint = 0x80	/* Disconnect is not allowed    */;
pub const DISC_ALLOW: c_uint = 0xC0	/* Disconnect is allowed        */;
pub const SCSICMD_RequestSense: c_uint = 0x03;
pub const SCSI_ABORT_SNOOZE: c_int = 0;
pub const SCSI_ABORT_SUCCESS: c_int = 1;
pub const SCSI_ABORT_PENDING: c_int = 2;
pub const SCSI_ABORT_BUSY: c_int = 3;
pub const SCSI_ABORT_NOT_RUNNING: c_int = 4;
pub const SCSI_ABORT_ERROR: c_int = 5;
pub const SCSI_RESET_SNOOZE: c_int = 0;
pub const SCSI_RESET_PUNT: c_int = 1;
pub const SCSI_RESET_SUCCESS: c_int = 2;
pub const SCSI_RESET_PENDING: c_int = 3;
pub const SCSI_RESET_WAKEUP: c_int = 4;
pub const SCSI_RESET_NOT_RUNNING: c_int = 5;
pub const SCSI_RESET_ERROR: c_int = 6;
pub const SCSI_RESET_SYNCHRONOUS: c_uint = 0x01;
pub const SCSI_RESET_ASYNCHRONOUS: c_uint = 0x02;
pub const SCSI_RESET_SUGGEST_BUS_RESET: c_uint = 0x04;
pub const SCSI_RESET_SUGGEST_HOST_RESET: c_uint = 0x08;
pub const SCSI_RESET_BUS_RESET: c_uint = 0x100;
pub const SCSI_RESET_HOST_RESET: c_uint = 0x200;
pub const SCSI_RESET_ACTION: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initio_cmd_priv {
    pub sense_dma_addr: dma_addr_t,
    pub sglist_dma_addr: dma_addr_t,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
