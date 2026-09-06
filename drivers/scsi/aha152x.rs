//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aha152x.h
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
//
// $Id: aha152x.h,v 2.7 2004/01/24 11:39:03 fischer Exp $
//
// number of queueable commands
pub const AHA152X_MAXQUEUE: c_int = 7;

// port addresses

pub const IO_RANGE: c_uint = 0x20;
// used in aha152x_porttest
pub const O_PORTA: c_uint = 0x1a               /* PORT A */;
pub const O_PORTB: c_uint = 0x1b               /* PORT B */;
pub const O_DMACNTRL1: c_uint = 0x13               /* DMA control 1 */;
pub const O_STACK: c_uint = 0x1d               /* stack */;
// used in tc1550_porttest
pub const O_TC_PORTA: c_uint = 0x0a               /* PORT A */;
pub const O_TC_PORTB: c_uint = 0x0b               /* PORT B */;
pub const O_TC_DMACNTRL1: c_uint = 0x03               /* DMA control 1 */;
pub const O_TC_STACK: c_uint = 0x0d               /* stack */;
// bits and bitmasks to ports
// SCSI sequence control
pub const TEMODEO: c_uint = 0x80;
pub const ENSELO: c_uint = 0x40;
pub const ENSELI: c_uint = 0x20;
pub const ENRESELI: c_uint = 0x10;
pub const ENAUTOATNO: c_uint = 0x08;
pub const ENAUTOATNI: c_uint = 0x04;
pub const ENAUTOATNP: c_uint = 0x02;
pub const SCSIRSTO: c_uint = 0x01;
// SCSI transfer control 0
pub const SCSIEN: c_uint = 0x80;
pub const DMAEN: c_uint = 0x40;
pub const CH1: c_uint = 0x20;
pub const CLRSTCNT: c_uint = 0x10;
pub const SPIOEN: c_uint = 0x08;
pub const CLRCH1: c_uint = 0x02;
// SCSI transfer control 1
pub const BITBUCKET: c_uint = 0x80;
pub const SWRAPEN: c_uint = 0x40;
pub const ENSPCHK: c_uint = 0x20;
pub const STIMESEL: c_uint = 0x18    /* mask */;
pub const STIMESEL_: c_int = 3;
pub const ENSTIMER: c_uint = 0x04;
pub const BYTEALIGN: c_uint = 0x02;
// SCSI signal IN
pub const SIG_CDI: c_uint = 0x80;
pub const SIG_IOI: c_uint = 0x40;
pub const SIG_MSGI: c_uint = 0x20;
pub const SIG_ATNI: c_uint = 0x10;
pub const SIG_SELI: c_uint = 0x08;
pub const SIG_BSYI: c_uint = 0x04;
pub const SIG_REQI: c_uint = 0x02;
pub const SIG_ACKI: c_uint = 0x01;
// SCSI Phases

// SCSI signal OUT
pub const SIG_CDO: c_uint = 0x80;
pub const SIG_IOO: c_uint = 0x40;
pub const SIG_MSGO: c_uint = 0x20;
pub const SIG_ATNO: c_uint = 0x10;
pub const SIG_SELO: c_uint = 0x08;
pub const SIG_BSYO: c_uint = 0x04;
pub const SIG_REQO: c_uint = 0x02;
pub const SIG_ACKO: c_uint = 0x01;
// SCSI rate control
pub const SXFR: c_uint = 0x70    /* mask */;
pub const SXFR_: c_int = 4;
pub const SOFS: c_uint = 0x0f    /* mask */;
// SCSI ID
pub const OID: c_uint = 0x70;
pub const OID_: c_int = 4;
pub const TID: c_uint = 0x07;
// SCSI transfer count

// SCSI interrupt status
pub const TARGET: c_uint = 0x80;
pub const SELDO: c_uint = 0x40;
pub const SELDI: c_uint = 0x20;
pub const SELINGO: c_uint = 0x10;
pub const SWRAP: c_uint = 0x08;
pub const SDONE: c_uint = 0x04;
pub const SPIORDY: c_uint = 0x02;
pub const DMADONE: c_uint = 0x01;
pub const SETSDONE: c_uint = 0x80;
pub const CLRSELDO: c_uint = 0x40;
pub const CLRSELDI: c_uint = 0x20;
pub const CLRSELINGO: c_uint = 0x10;
pub const CLRSWRAP: c_uint = 0x08;
pub const CLRSDONE: c_uint = 0x04;
pub const CLRSPIORDY: c_uint = 0x02;
pub const CLRDMADONE: c_uint = 0x01;
// SCSI status 1
pub const SELTO: c_uint = 0x80;
pub const ATNTARG: c_uint = 0x40;
pub const SCSIRSTI: c_uint = 0x20;
pub const PHASEMIS: c_uint = 0x10;
pub const BUSFREE: c_uint = 0x08;
pub const SCSIPERR: c_uint = 0x04;
pub const PHASECHG: c_uint = 0x02;
pub const REQINIT: c_uint = 0x01;
pub const CLRSELTIMO: c_uint = 0x80;
pub const CLRATNO: c_uint = 0x40;
pub const CLRSCSIRSTI: c_uint = 0x20;
pub const CLRBUSFREE: c_uint = 0x08;
pub const CLRSCSIPERR: c_uint = 0x04;
pub const CLRPHASECHG: c_uint = 0x02;
pub const CLRREQINIT: c_uint = 0x01;
// SCSI status 2
pub const SOFFSET: c_uint = 0x20;
pub const SEMPTY: c_uint = 0x10;
pub const SFULL: c_uint = 0x08;
pub const SFCNT: c_uint = 0x07    /* mask */;
// SCSI status 3
pub const SCSICNT: c_uint = 0xf0    /* mask */;
pub const SCSICNT_: c_int = 4;
pub const OFFCNT: c_uint = 0x0f    /* mask */;
// SCSI TEST control
pub const SCTESTU: c_uint = 0x08;
pub const SCTESTD: c_uint = 0x04;
pub const STCTEST: c_uint = 0x01;
// SCSI status 4
pub const SYNCERR: c_uint = 0x04;
pub const FWERR: c_uint = 0x02;
pub const FRERR: c_uint = 0x01;
pub const CLRSYNCERR: c_uint = 0x04;
pub const CLRFWERR: c_uint = 0x02;
pub const CLRFRERR: c_uint = 0x01;
// SCSI interrupt mode 0
pub const ENSELDO: c_uint = 0x40;
pub const ENSELDI: c_uint = 0x20;
pub const ENSELINGO: c_uint = 0x10;
pub const ENSWRAP: c_uint = 0x08;
pub const ENSDONE: c_uint = 0x04;
pub const ENSPIORDY: c_uint = 0x02;
pub const ENDMADONE: c_uint = 0x01;
// SCSI interrupt mode 1
pub const ENSELTIMO: c_uint = 0x80;
pub const ENATNTARG: c_uint = 0x40;
pub const ENSCSIRST: c_uint = 0x20;
pub const ENPHASEMIS: c_uint = 0x10;
pub const ENBUSFREE: c_uint = 0x08;
pub const ENSCSIPERR: c_uint = 0x04;
pub const ENPHASECHG: c_uint = 0x02;
pub const ENREQINIT: c_uint = 0x01;
// DMA control 0
pub const ENDMA: c_uint = 0x80;
pub const _8BIT: c_uint = 0x40;
pub const DMA: c_uint = 0x20;
pub const WRITE_READ: c_uint = 0x08;
pub const INTEN: c_uint = 0x04;
pub const RSTFIFO: c_uint = 0x02;
pub const SWINT: c_uint = 0x01;
// DMA control 1
pub const PWRDWN: c_uint = 0x80;
pub const STK: c_uint = 0x07    /* mask */;
// DMA status
pub const ATDONE: c_uint = 0x80;
pub const WORDRDY: c_uint = 0x40;
pub const INTSTAT: c_uint = 0x20;
pub const DFIFOFULL: c_uint = 0x10;
pub const DFIFOEMP: c_uint = 0x08;
// BURST control
pub const BON: c_uint = 0xf0;
pub const BOFF: c_uint = 0x0f;
// TEST REGISTER
pub const BOFFTMR: c_uint = 0x40;
pub const BONTMR: c_uint = 0x20;
pub const STCNTH: c_uint = 0x10;
pub const STCNTM: c_uint = 0x08;
pub const STCNTL: c_uint = 0x04;
pub const SCSIBLK: c_uint = 0x02;
pub const DMABLK: c_uint = 0x01;
// On the AHA-152x board PORTA and PORTB contain
//

// Some macros to manipulate ports and their bits

// for the pcmcia stub
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aha152x_setup {
    pub io_port: c_int,
    pub irq: c_int,
    pub scsiid: c_int,
    pub reconnect: c_int,
    pub parity: c_int,
    pub synchronous: c_int,
    pub delay: c_int,
    pub ext_trans: c_int,
    pub tc1550: c_int,

    pub debug: c_int,

    pub conf: *mut c_char,
}

extern "C" {
    pub fn aha152x_release(: *mut Scsi_Host);
}
extern "C" {
    pub fn aha152x_host_reset_host(: *mut Scsi_Host) -> c_int;
}
