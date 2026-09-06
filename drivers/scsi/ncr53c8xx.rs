//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ncr53c8xx.h
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
// Device driver for the PCI-SCSI NCR538XX controller family.
//
// Copyright (C) 1994  Wolfgang Stanglmeier
// Copyright (C) 1998-2001  Gerard Roudier <groudier@free.fr>
//
// -----------------------------------------------------------------------------
//
// This driver has been ported to Linux from the FreeBSD NCR53C8XX driver
// and is currently maintained by
//
// Gerard Roudier              <groudier@free.fr>
//
// Being given that this driver originates from the FreeBSD version, and
// in order to keep synergy on both, any suggested enhancements and corrections
// received on Linux are automatically a potential candidate for the FreeBSD
// version.
//
// The original driver has been written for 386bsd and FreeBSD by
// Wolfgang Stanglmeier        <wolf@cologne.de>
// Stefan Esser                <se@mi.Uni-Koeln.de>
//
// And has been ported to NetBSD by
// Charles M. Hannum           <mycroft@gnu.ai.mit.edu>
//
// NVRAM detection and reading.
// Copyright (C) 1997 Richard Waltham <dormouse@farsrobt.demon.co.uk>
//
// Added support for MIPS big endian systems.
// Carsten Langgaard, carstenl@mips.com
// Copyright (C) 2000 MIPS Technologies, Inc.  All rights reserved.
//
// Added support for HP PARISC big endian systems.
// Copyright (C) 2000 MIPS Technologies, Inc.  All rights reserved.
//

//
// If you want a driver as small as possible, donnot define the
// following options.
//
// Macro flag: #define SCSI_NCR_BOOT_COMMAND_LINE_SUPPORT
// Macro flag: #define SCSI_NCR_DEBUG_INFO_SUPPORT
//
// To disable integrity checking, do not define the
// following option.
//

// ---------------------------------------------------------------------
// Take into account kernel configured parameters.
// Most of these options can be overridden at startup by a command line.
// ---------------------------------------------------------------------
//
// For Ultra2 and Ultra3 SCSI support option, use special features.
//
// Value (default) means:
// bit 0 : all features enabled, except:
// bit 1 : PCI Write And Invalidate.
// bit 2 : Data Phase Mismatch handling from SCRIPTS.
//
// Use boot options ncr53c8xx=specf:1 if you want all chip features to be
// enabled by the driver.
//

//
// Allow tags from 2 to 256, default 8
//

//
// Allow tagged command queuing support if configured with default number
// of tags set to max (see above).
//

//
// Immediate arbitration
//

// Macro flag: #define SCSI_NCR_IARB_SUPPORT

//
// Sync transfer frequency at startup.
// Allow from 5Mhz to 80Mhz default 20 Mhz.
//

//
// Disallow disconnections at boot-up
//

//
// Force synchronous negotiation for all targets
//

//
// Disable master parity checking (flawed hardwares need that)
//

//
// Disable scsi parity checking (flawed devices may need that)
//

//
// Settle time after reset at boot-up
//

//
// Bridge quirks work-around option defaulted to 1.
//

pub const SCSI_NCR_PCIQ_WORK_AROUND_OPT: c_int = 1;

//
// Work-around common bridge misbehaviour.
//
// - Do not flush posted writes in the opposite
// direction on read.
// - May reorder DMA writes to memory.
//
// This option should not affect performances
// significantly, so it is the default.
//

// Macro flag: #define	SCSI_NCR_PCIQ_MAY_NOT_FLUSH_PW_UPSTREAM
// Macro flag: #define	SCSI_NCR_PCIQ_MAY_REORDER_WRITES
// Macro flag: #define	SCSI_NCR_PCIQ_MAY_MISS_COMPLETIONS
//
// Same as option 1, but also deal with
// misconfigured interrupts.
//
// - Edge triggered instead of level sensitive.
// - No interrupt line connected.
// - IRQ number misconfigured.
//
// If no interrupt is delivered, the driver will
// catch the interrupt conditions 10 times per
// second. No need to say that this option is
// not recommended.
//

// Macro flag: #define	SCSI_NCR_PCIQ_MAY_NOT_FLUSH_PW_UPSTREAM
// Macro flag: #define	SCSI_NCR_PCIQ_MAY_REORDER_WRITES
// Macro flag: #define	SCSI_NCR_PCIQ_MAY_MISS_COMPLETIONS
// Macro flag: #define	SCSI_NCR_PCIQ_BROKEN_INTR
//
// Some bridge designers decided to flush
// everything prior to deliver the interrupt.
// This option tries to deal with such a
// behaviour.
//

// Macro flag: #define	SCSI_NCR_PCIQ_SYNC_ON_INTR

//
// Other parameters not configurable with "make config"
// Avoid to change these constants, unless you know what you are doing.
//
// Macro flag: #define SCSI_NCR_ALWAYS_SIMPLE_TAG

//
// Compute some desirable value for CAN_QUEUE
// and CMD_PER_LUN.
// The driver will use lower values if these
// ones appear to be too large.
//

//
// IO functions definition for big/little endian CPU support.
// For now, the NCR is only supported in little endian addressing mode,
//

//
// If the NCR uses big endian addressing mode over the
// PCI, actual io register addresses for byte and word
// accesses must be changed according to lane routing.
// Btw, ncr_offb() and ncr_offw() macros only apply to
// constants and so donnot generate bloated code.
//

//
// If the CPU and the NCR use same endian-ness addressing,
// no byte reordering is needed for script patching.
// Macro cpu_to_scr() is to be used for script patching.
// Macro scr_to_cpu() is to be used for getting a DWORD
// from the script.
//

//
// Access to the controller chip.
//
// If the CPU and the NCR use same endian-ness addressing,
// no byte reordering is needed for accessing chip io
// registers. Functions suffixed by '_raw' are assumed
// to access the chip over the PCI without doing byte
// reordering. Functions suffixed by '_l2b' are
// assumed to perform little-endian to big-endian byte
// reordering, those suffixed by '_b2l' blah, blah,
// blah, ...
//
// MEMORY mapped IO input / output
//

// Only 8 or 32 bit transfers allowed

// Only 8 or 32 bit transfers allowed

//
// Set bit field ON, OFF
//

//
// We normally want the chip to have a consistent view
// of driver internal data structures when we restart it.
// Thus these macros.
//

//
// NCR53C8XX devices features table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncr_chip {
    pub revision_id: c_ushort,
    pub /: *mut *mut unsigned char burst_max; / log-base-2 of max burst,
    pub offset_max: c_uchar,
    pub nr_divisor: c_uchar,
    pub features: c_uint,

}

//
// Driver setup structure.
//
// This structure is initialized from linux config options.
// It can be overridden at boot-up by the boot command line.
//
pub const SCSI_NCR_MAX_EXCLUDES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncr_driver_setup {
    pub master_parity: u8,
    pub scsi_parity: u8,
    pub disconnection: u8,
    pub special_features: u8,
    pub force_sync_nego: u8,
    pub reverse_probe: u8,
    pub pci_fix_up: u8,
    pub use_nvram: u8,
    pub verbose: u8,
    pub default_tags: u8,
    pub default_sync: u16,
    pub debug: u16,
    pub burst_max: u8,
    pub led_pin: u8,
    pub max_wide: u8,
    pub settle_delay: u8,
    pub diff_support: u8,
    pub irqm: u8,
    pub bus_check: u8,
    pub optimize: u8,
    pub recovery: u8,
    pub host_id: u8,
    pub iarb: u16,
    pub excludes: [u32; SCSI_NCR_MAX_EXCLUDES],
    pub tag_ctrl: [c_char; 100],
}

//
// Initial setup.
// Can be overridden at startup by a command line.
//

//
// Boot fail safe setup.
// Override initial setup from boot command line:
// ncr53c8xx=safe:y
//

// ORIGINAL CONTENT of ncrreg.h from FreeBSD
// -----------------------------------------------------------------
//
// The ncr 53c810 register structure.
//
// -----------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncr_reg {
// 00*/  u8	nc_scntl0;    /* full arb., ena parity, par->ATN
// 01*/  u8	nc_scntl1;    /* no reset
pub const ISCON: c_uint = 0x10  /* connected to scsi		    */;
pub const CRST: c_uint = 0x08  /* force reset                      */;
pub const IARB: c_uint = 0x02  /* immediate arbitration            */;
// 02*/  u8	nc_scntl2;    /* no disconnect expected
pub const SDU: c_uint = 0x80  /* cmd: disconnect will raise error */;
pub const CHM: c_uint = 0x40  /* sta: chained mode                */;
pub const WSS: c_uint = 0x08  /* sta: wide scsi send           [W]*/;
pub const WSR: c_uint = 0x01  /* sta: wide scsi received       [W]*/;
// 03*/  u8	nc_scntl3;    /* cnf system clock dependent
pub const EWS: c_uint = 0x08  /* cmd: enable wide scsi         [W]*/;
pub const ULTRA: c_uint = 0x80  /* cmd: ULTRA enable                */;
// bits 0-2, 7 rsvd for C1010
// 04*/  u8	nc_scid;	/* cnf host adapter scsi address
pub const RRE: c_uint = 0x40  /* r/w:e enable response to resel.  */;
pub const SRE: c_uint = 0x20  /* r/w:e enable response to select  */;
// 05*/  u8	nc_sxfer;	/* ### Sync speed and count
// bits 6-7 rsvd for C1010
// 06*/  u8	nc_sdid;	/* ### Destination-ID
// 07*/  u8	nc_gpreg;	/* ??? IO-Pins
// 08*/  u8	nc_sfbr;	/* ### First byte in phase
// 09*/  u8	nc_socl;
pub const CREQ: c_uint = 0x80	/* r/w: SCSI-REQ                    */;
pub const CACK: c_uint = 0x40	/* r/w: SCSI-ACK                    */;
pub const CBSY: c_uint = 0x20	/* r/w: SCSI-BSY                    */;
pub const CSEL: c_uint = 0x10	/* r/w: SCSI-SEL                    */;
pub const CATN: c_uint = 0x08	/* r/w: SCSI-ATN                    */;
pub const CMSG: c_uint = 0x04	/* r/w: SCSI-MSG                    */;
pub const CC_D: c_uint = 0x02	/* r/w: SCSI-C_D                    */;
pub const CI_O: c_uint = 0x01	/* r/w: SCSI-I_O                    */;
// 0a*/  u8	nc_ssid;
// 0b*/  u8	nc_sbcl;
// 0c*/  u8	nc_dstat;
pub const DFE: c_uint = 0x80  /* sta: dma fifo empty              */;
pub const MDPE: c_uint = 0x40  /* int: master data parity error    */;
pub const BF: c_uint = 0x20  /* int: script: bus fault           */;
pub const ABRT: c_uint = 0x10  /* int: script: command aborted     */;
pub const SSI: c_uint = 0x08  /* int: script: single step         */;
pub const SIR: c_uint = 0x04  /* int: script: interrupt instruct. */;
pub const IID: c_uint = 0x01  /* int: script: illegal instruct.   */;
// 0d*/  u8	nc_sstat0;
pub const ILF: c_uint = 0x80  /* sta: data in SIDL register lsb   */;
pub const ORF: c_uint = 0x40  /* sta: data in SODR register lsb   */;
pub const OLF: c_uint = 0x20  /* sta: data in SODL register lsb   */;
pub const AIP: c_uint = 0x10  /* sta: arbitration in progress     */;
pub const LOA: c_uint = 0x08  /* sta: arbitration lost            */;
pub const WOA: c_uint = 0x04  /* sta: arbitration won             */;
pub const IRST: c_uint = 0x02  /* sta: scsi reset signal           */;
pub const SDP: c_uint = 0x01  /* sta: scsi parity signal          */;
// 0e*/  u8	nc_sstat1;
pub const FF3210: c_uint = 0xf0	/* sta: bytes in the scsi fifo      */;
// 0f*/  u8	nc_sstat2;
pub const ILF1: c_uint = 0x80  /* sta: data in SIDL register msb[W]*/;
pub const ORF1: c_uint = 0x40  /* sta: data in SODR register msb[W]*/;
pub const OLF1: c_uint = 0x20  /* sta: data in SODL register msb[W]*/;
pub const DM: c_uint = 0x04  /* sta: DIFFSENS mismatch (895/6 only) */;
pub const LDSC: c_uint = 0x02  /* sta: disconnect & reconnect      */;
// 10*/  u8	nc_dsa;	/* --> Base page
// 11*/  u8	nc_dsa1;
// 12*/  u8	nc_dsa2;
// 13*/  u8	nc_dsa3;
// 14*/  u8	nc_istat;	/* --> Main Command and status
pub const CABRT: c_uint = 0x80  /* cmd: abort current operation     */;
pub const SRST: c_uint = 0x40  /* mod: reset chip                  */;
pub const SIGP: c_uint = 0x20  /* r/w: message from host to ncr    */;
pub const SEM: c_uint = 0x10  /* r/w: message between host + ncr  */;
pub const CON: c_uint = 0x08  /* sta: connected to scsi           */;
pub const INTF: c_uint = 0x04  /* sta: int on the fly (reset by wr)*/;
pub const SIP: c_uint = 0x02  /* sta: scsi-interrupt              */;
pub const DIP: c_uint = 0x01  /* sta: host/script interrupt       */;
// 15*/  u8	nc_istat1;	/* 896 and later cores only
pub const FLSH: c_uint = 0x04  /* sta: chip is flushing            */;
pub const SRUN: c_uint = 0x02  /* sta: scripts are running         */;
pub const SIRQD: c_uint = 0x01  /* r/w: disable INT pin             */;
// 16*/  u8	nc_mbox0;	/* 896 and later cores only
// 17*/  u8	nc_mbox1;	/* 896 and later cores only
// 18*/	u8	nc_ctest0;
pub const EHP: c_uint = 0x04	/* 720 even host parity             */;
// 19*/  u8	nc_ctest1;
// 1a*/  u8	nc_ctest2;
pub const CSIGP: c_uint = 0x40;
// bits 0-2,7 rsvd for C1010
// 1b*/  u8	nc_ctest3;
pub const FLF: c_uint = 0x08  /* cmd: flush dma fifo              */;
pub const CLF: c_uint = 0x04	/* cmd: clear dma fifo		    */;
pub const FM: c_uint = 0x02  /* mod: fetch pin mode              */;
pub const WRIE: c_uint = 0x01  /* mod: write and invalidate enable */;
// bits 4-7 rsvd for C1010
// 1c*/  u32    nc_temp;	/* ### Temporary stack
// 20*/	u8	nc_dfifo;
// 21*/  u8	nc_ctest4;
pub const MUX: c_uint = 0x80  /* 720 host bus multiplex mode      */;
pub const BDIS: c_uint = 0x80  /* mod: burst disable               */;
pub const MPEE: c_uint = 0x08  /* mod: master parity error enable  */;
// 22*/  u8	nc_ctest5;
pub const DFS: c_uint = 0x20  /* mod: dma fifo size               */;
// bits 0-1, 3-7 rsvd for C1010
// 23*/  u8	nc_ctest6;
// 24*/  u32    nc_dbc;	/* ### Byte count and command
// 28*/  u32    nc_dnad;	/* ### Next command register
// 2c*/  u32    nc_dsp;	/* --> Script Pointer
// 30*/  u32    nc_dsps;	/* --> Script pointer save/opcode#2
// 34*/  u8	nc_scratcha;  /* Temporary register a
// 35*/  u8	nc_scratcha1;
// 36*/  u8	nc_scratcha2;
// 37*/  u8	nc_scratcha3;
// 38*/  u8	nc_dmode;
pub const BL_2: c_uint = 0x80  /* mod: burst length shift value +2 */;
pub const BL_1: c_uint = 0x40  /* mod: burst length shift value +1 */;
pub const ERL: c_uint = 0x08  /* mod: enable read line            */;
pub const ERMP: c_uint = 0x04  /* mod: enable read multiple        */;
pub const BOF: c_uint = 0x02  /* mod: burst op code fetch         */;
// 39*/  u8	nc_dien;
// 3a*/  u8	nc_sbr;
// 3b*/  u8	nc_dcntl;	/* --> Script execution control
pub const CLSE: c_uint = 0x80  /* mod: cache line size enable      */;
pub const PFF: c_uint = 0x40  /* cmd: pre-fetch flush             */;
pub const PFEN: c_uint = 0x20  /* mod: pre-fetch enable            */;
pub const EA: c_uint = 0x20  /* mod: 720 enable-ack              */;
pub const SSM: c_uint = 0x10  /* mod: single step mode            */;
pub const IRQM: c_uint = 0x08  /* mod: irq mode (1 = totem pole !) */;
pub const STD: c_uint = 0x04  /* cmd: start dma mode              */;
pub const IRQD: c_uint = 0x02  /* mod: irq disable                 */;
pub const NOCOM: c_uint = 0x01	/* cmd: protect sfbr while reselect */;
// bits 0-1 rsvd for C1010
// 3c*/  u32	nc_adder;
// 40*/  u16	nc_sien;	/* -->: interrupt enable
// 42*/  u16	nc_sist;	/* <--: interrupt status
pub const SBMC: c_uint = 0x1000/* sta: SCSI Bus Mode Change (895/6 only) */;
pub const STO: c_uint = 0x0400/* sta: timeout (select)            */;
pub const GEN: c_uint = 0x0200/* sta: timeout (general)           */;
pub const HTH: c_uint = 0x0100/* sta: timeout (handshake)         */;
pub const MA: c_uint = 0x80  /* sta: phase mismatch              */;
pub const CMP: c_uint = 0x40  /* sta: arbitration complete        */;
pub const SEL: c_uint = 0x20  /* sta: selected by another device  */;
pub const RSL: c_uint = 0x10  /* sta: reselected by another device*/;
pub const SGE: c_uint = 0x08  /* sta: gross error (over/underflow)*/;
pub const UDC: c_uint = 0x04  /* sta: unexpected disconnect       */;
pub const RST: c_uint = 0x02  /* sta: scsi bus reset detected     */;
pub const PAR: c_uint = 0x01  /* sta: scsi parity error           */;
// 44*/  u8	nc_slpar;
// 45*/  u8	nc_swide;
// 46*/  u8	nc_macntl;
// 47*/  u8	nc_gpcntl;
// 48*/  u8	nc_stime0;    /* cmd: timeout for select&handshake
// 49*/  u8	nc_stime1;    /* cmd: timeout user defined
// 4a*/  u16   nc_respid;    /* sta: Reselect-IDs
// 4c*/  u8	nc_stest0;
// 4d*/  u8	nc_stest1;
pub const SCLK: c_uint = 0x80	/* Use the PCI clock as SCSI clock	*/;
pub const DBLEN: c_uint = 0x08	/* clock doubler running		*/;
pub const DBLSEL: c_uint = 0x04	/* clock doubler selected		*/;
// 4e*/  u8	nc_stest2;
pub const ROF: c_uint = 0x40	/* reset scsi offset (after gross error!) */;
pub const DIF: c_uint = 0x20  /* 720 SCSI differential mode             */;
pub const EXT: c_uint = 0x02  /* extended filtering                     */;
// 4f*/  u8	nc_stest3;
pub const TE: c_uint = 0x80	/* c: tolerAnt enable */;
pub const HSC: c_uint = 0x20	/* c: Halt SCSI Clock */;
pub const CSF: c_uint = 0x02	/* c: clear scsi fifo */;
// 50*/  u16   nc_sidl;	/* Lowlevel: latched from scsi data
// 52*/  u8	nc_stest4;
pub const SMODE: c_uint = 0xc0	/* SCSI bus mode      (895/6 only) */;
pub const SMODE_HVD: c_uint = 0x40	/* High Voltage Differential       */;
pub const SMODE_SE: c_uint = 0x80	/* Single Ended                    */;
pub const SMODE_LVD: c_uint = 0xc0	/* Low Voltage Differential        */;
pub const LCKFRQ: c_uint = 0x20	/* Frequency Lock (895/6 only)     */;
// bits 0-5 rsvd for C1010
// 53*/  u8	nc_53_;
// 54*/  u16	nc_sodl;	/* Lowlevel: data out to scsi data
// 56*/	u8	nc_ccntl0;	/* Chip Control 0 (896)
pub const ENPMJ: c_uint = 0x80	/* Enable Phase Mismatch Jump       */;
pub const PMJCTL: c_uint = 0x40	/* Phase Mismatch Jump Control      */;
pub const ENNDJ: c_uint = 0x20	/* Enable Non Data PM Jump          */;
pub const DISFC: c_uint = 0x10	/* Disable Auto FIFO Clear          */;
pub const DILS: c_uint = 0x02	/* Disable Internal Load/Store      */;
pub const DPR: c_uint = 0x01	/* Disable Pipe Req                 */;
// 57*/	u8	nc_ccntl1;	/* Chip Control 1 (896)
pub const ZMOD: c_uint = 0x80	/* High Impedance Mode              */;
pub const DIC: c_uint = 0x10	/* Disable Internal Cycles	    */;
pub const DDAC: c_uint = 0x08	/* Disable Dual Address Cycle       */;
pub const XTIMOD: c_uint = 0x04	/* 64-bit Table Ind. Indexing Mode  */;
pub const EXTIBMV: c_uint = 0x02	/* Enable 64-bit Table Ind. BMOV    */;
pub const EXDBMV: c_uint = 0x01	/* Enable 64-bit Direct BMOV        */;
// 58*/  u16	nc_sbdl;	/* Lowlevel: data from scsi data
// 5a*/  u16	nc_5a_;
// 5c*/  u8	nc_scr0;	/* Working register B
// 5d*/  u8	nc_scr1;	/*
// 5e*/  u8	nc_scr2;	/*
// 5f*/  u8	nc_scr3;	/*
// 60*/  u8	nc_scrx[64];	/* Working register C-R
// a0*/	u32	nc_mmrs;	/* Memory Move Read Selector
// a4*/	u32	nc_mmws;	/* Memory Move Write Selector
// a8*/	u32	nc_sfs;		/* Script Fetch Selector
// ac*/	u32	nc_drs;		/* DSA Relative Selector
// b0*/	u32	nc_sbms;	/* Static Block Move Selector
// b4*/	u32	nc_dbms;	/* Dynamic Block Move Selector
// b8*/	u32	nc_dnad64;	/* DMA Next Address 64
// bc*/	u16	nc_scntl4;	/* C1010 only
pub const U3EN: c_uint = 0x80	/* Enable Ultra 3                   */;
pub const AIPEN: c_uint = 0x40   /* Allow check upper byte lanes     */;
pub const XCLKH_DT: c_uint = 0x08 /* Extra clock of data hold on DT;
pub const XCLKH_ST: c_uint = 0x04 /* Extra clock of data hold on ST;
// be*/  u8	nc_aipcntl0;	/* Epat Control 1 C1010 only
// bf*/  u8	nc_aipcntl1;	/* AIP Control C1010_66 Only
// c0*/	u32	nc_pmjad1;	/* Phase Mismatch Jump Address 1
// c4*/	u32	nc_pmjad2;	/* Phase Mismatch Jump Address 2
// c8*/	u8	nc_rbc;		/* Remaining Byte Count
// c9*/	u8	nc_rbc1;	/*
// ca*/	u8	nc_rbc2;	/*
// cb*/	u8	nc_rbc3;	/*
// cc*/	u8	nc_ua;		/* Updated Address
// cd*/	u8	nc_ua1;		/*
// ce*/	u8	nc_ua2;		/*
// cf*/	u8	nc_ua3;		/*
// d0*/	u32	nc_esa;		/* Entry Storage Address
// d4*/	u8	nc_ia;		/* Instruction Address
// d5*/	u8	nc_ia1;
// d6*/	u8	nc_ia2;
// d7*/	u8	nc_ia3;
// d8*/	u32	nc_sbc;		/* SCSI Byte Count (3 bytes only)
// dc*/	u32	nc_csbc;	/* Cumulative SCSI Byte Count
// Following for C1010 only
// e0*/  u16	nc_crcpad;	/* CRC Value
// e2*/  u8	nc_crccntl0;	/* CRC control register
pub const SNDCRC: c_uint = 0x10	/* Send CRC Request                 */;
// e3*/  u8	nc_crccntl1;	/* CRC control register
// e4*/  u32	nc_crcdata;	/* CRC data register
// e8*/  u32	nc_e8_;		/* rsvd
// ec*/  u32	nc_ec_;		/* rsvd
// f0*/  u16	nc_dfbc;	/* DMA FIFO byte count
}

// -----------------------------------------------------------
//
// Utility macros for the script.
//
// -----------------------------------------------------------
//

pub type ncrcmd = u32;
// -----------------------------------------------------------
//
// SCSI phases
//
// DT phases illegal for ncr driver.
//
// -----------------------------------------------------------
//
pub const SCR_DATA_OUT: c_uint = 0x00000000;
pub const SCR_DATA_IN: c_uint = 0x01000000;
pub const SCR_COMMAND: c_uint = 0x02000000;
pub const SCR_STATUS: c_uint = 0x03000000;
pub const SCR_DT_DATA_OUT: c_uint = 0x04000000;
pub const SCR_DT_DATA_IN: c_uint = 0x05000000;
pub const SCR_MSG_OUT: c_uint = 0x06000000;
pub const SCR_MSG_IN: c_uint = 0x07000000;
pub const SCR_ILG_OUT: c_uint = 0x04000000;
pub const SCR_ILG_IN: c_uint = 0x05000000;
// -----------------------------------------------------------
//
// Data transfer via SCSI.
//
// -----------------------------------------------------------
//
// MOVE_ABS (LEN)
// <<start address>>
//
// MOVE_IND (LEN)
// <<dnad_offset>>
//
// MOVE_TBL
// <<dnad_offset>>
//
// -----------------------------------------------------------
//
pub const OPC_MOVE: c_uint = 0x08000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scr_tblmove {
    pub size: u32,
    pub addr: u32,
}

// -----------------------------------------------------------
//
// Selection
//
// -----------------------------------------------------------
//
// SEL_ABS | SCR_ID (0..15)    [ | REL_JMP]
// <<alternate_address>>
//
// SEL_TBL | << dnad_offset>>  [ | REL_JMP]
// <<alternate_address>>
//
// -----------------------------------------------------------
//
pub const SCR_SEL_ABS: c_uint = 0x40000000;
pub const SCR_SEL_ABS_ATN: c_uint = 0x41000000;
pub const SCR_SEL_TBL: c_uint = 0x42000000;
pub const SCR_SEL_TBL_ATN: c_uint = 0x43000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scr_tblsel {
    pub sel_scntl3: u8,
    pub sel_id: u8,
    pub sel_sxfer: u8,
    pub sel_scntl4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scr_tblsel {
    pub sel_scntl4: u8,
    pub sel_sxfer: u8,
    pub sel_id: u8,
    pub sel_scntl3: u8,
}

pub const SCR_JMP_REL: c_uint = 0x04000000;

// -----------------------------------------------------------
//
// Waiting for Disconnect or Reselect
//
// -----------------------------------------------------------
//
// WAIT_DISC
// dummy: <<alternate_address>>
//
// WAIT_RESEL
// <<alternate_address>>
//
// -----------------------------------------------------------
//
pub const SCR_WAIT_DISC: c_uint = 0x48000000;
pub const SCR_WAIT_RESEL: c_uint = 0x50000000;
// -----------------------------------------------------------
//
// Bit Set / Reset
//
// -----------------------------------------------------------
//
// SET (flags {|.. })
//
// CLR (flags {|.. })
//
// -----------------------------------------------------------
//

pub const SCR_CARRY: c_uint = 0x00000400;
pub const SCR_TRG: c_uint = 0x00000200;
pub const SCR_ACK: c_uint = 0x00000040;
pub const SCR_ATN: c_uint = 0x00000008;
// -----------------------------------------------------------
//
// Memory to memory move
//
// -----------------------------------------------------------
//
// COPY (bytecount)
// << source_address >>
// << destination_address >>
//
// SCR_COPY   sets the NO FLUSH option by default.
// SCR_COPY_F does not set this option.
//
// For chips which do not support this option,
// ncr_copy_and_bind() will remove this bit.
// -----------------------------------------------------------
//
pub const SCR_NO_FLUSH: c_uint = 0x01000000;

// -----------------------------------------------------------
//
// Register move and binary operations
//
// -----------------------------------------------------------
//
// SFBR_REG (reg, op, data)        reg  = SFBR op data
// << 0 >>
//
// REG_SFBR (reg, op, data)        SFBR = reg op data
// << 0 >>
//
// REG_REG  (reg, op, data)        reg  = reg op data
// << 0 >>
//
// -----------------------------------------------------------
// On 810A, 860, 825A, 875, 895 and 896 chips the content
// of SFBR register can be used as data (SCR_SFBR_DATA).
// The 896 has additional IO registers starting at
// offset 0x80. Bit 7 of register offset is stored in
// bit 7 of the SCRIPTS instruction first DWORD.
// -----------------------------------------------------------
//

pub const SCR_LOAD: c_uint = 0x00000000;
pub const SCR_SHL: c_uint = 0x01000000;
pub const SCR_OR: c_uint = 0x02000000;
pub const SCR_XOR: c_uint = 0x03000000;
pub const SCR_AND: c_uint = 0x04000000;
pub const SCR_SHR: c_uint = 0x05000000;
pub const SCR_ADD: c_uint = 0x06000000;
pub const SCR_ADDC: c_uint = 0x07000000;

// -----------------------------------------------------------
//
// FROM_REG (reg)		  SFBR = reg
// << 0 >>
//
// TO_REG	 (reg)		  reg  = SFBR
// << 0 >>
//
// LOAD_REG (reg, data)	  reg  = <data>
// << 0 >>
//
// LOAD_SFBR(data) 	  SFBR = <data>
// << 0 >>
//
// -----------------------------------------------------------
//

// -----------------------------------------------------------
//
// LOAD  from memory   to register.
// STORE from register to memory.
//
// Only supported by 810A, 860, 825A, 875, 895 and 896.
//
// -----------------------------------------------------------
//
// LOAD_ABS (LEN)
// <<start address>>
//
// LOAD_REL (LEN)        (DSA relative)
// <<dsa_offset>>
//
// -----------------------------------------------------------
//

pub const SCR_NO_FLUSH2: c_uint = 0x02000000;
pub const SCR_DSA_REL2: c_uint = 0x10000000;

// -----------------------------------------------------------
//
// Waiting for Disconnect or Reselect
//
// -----------------------------------------------------------
//
// JUMP            [ | IFTRUE/IFFALSE ( ... ) ]
// <<address>>
//
// JUMPR           [ | IFTRUE/IFFALSE ( ... ) ]
// <<distance>>
//
// CALL            [ | IFTRUE/IFFALSE ( ... ) ]
// <<address>>
//
// CALLR           [ | IFTRUE/IFFALSE ( ... ) ]
// <<distance>>
//
// RETURN          [ | IFTRUE/IFFALSE ( ... ) ]
// <<dummy>>
//
// INT             [ | IFTRUE/IFFALSE ( ... ) ]
// <<ident>>
//
// INT_FLY         [ | IFTRUE/IFFALSE ( ... ) ]
// <<ident>>
//
// Conditions:
// WHEN (phase)
// IF   (phase)
// CARRYSET
// DATA (data, mask)
//
// -----------------------------------------------------------
//
pub const SCR_NO_OP: c_uint = 0x80000000;
pub const SCR_JUMP: c_uint = 0x80080000;
pub const SCR_JUMP64: c_uint = 0x80480000;
pub const SCR_JUMPR: c_uint = 0x80880000;
pub const SCR_CALL: c_uint = 0x88080000;
pub const SCR_CALLR: c_uint = 0x88880000;
pub const SCR_RETURN: c_uint = 0x90080000;
pub const SCR_INT: c_uint = 0x98080000;
pub const SCR_INT_FLY: c_uint = 0x98180000;

// -----------------------------------------------------------
//
// SCSI  constants.
//
// -----------------------------------------------------------
//
// End of ncrreg from FreeBSD
//

// ==========================================================
//
// Structures used by the detection routine to transmit
// device configuration to the attach function.
//
// ==========================================================
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncr_slot {
    pub base: u_long,
    pub base_2: u_long,
    pub base_c: u_long,
    pub base_2_c: u_long,
    pub base_v: *mut void __iomem,
    pub base_2_v: *mut void __iomem,
    pub irq: c_int,
// port and reg fields to use INB, OUTB macros
    pub reg: *mut volatile struct ncr_reg __iomem,
}

// ==========================================================
//
// Structure used by detection routine to save data on
// each detected board for attach.
//
// ==========================================================
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncr_device {
    pub dev: *mut device,
    pub slot: ncr_slot,
    pub chip: ncr_chip,
    pub host_id: u_char,
    pub differential: u8,
}

// To keep track of the dma mapping (sg/single) that has been set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncr_cmd_priv {
    pub data_mapped: c_int,
    pub data_mapping: c_int,
}

extern "C" {
    pub fn ncr53c8xx_release(host: *mut Scsi_Host);
}
extern "C" {
    pub fn ncr53c8xx_intr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ncr53c8xx_init() -> c_int;
}
extern "C" {
    pub fn ncr53c8xx_exit();
}
