//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/a100u2w.h
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
// Initio A100 device driver for Linux.
//
// Copyright (c) 1994-1998 Initio Corporation
// All rights reserved.
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
// Revision History:
// 06/18/98 HL, Initial production Version 1.02
// 12/19/98 bv, Use spinlocks for 2.1.95 and up
// 06/25/02 Doug Ledford <dledford@redhat.com>
// - This and the i60uscsi.h file are almost identical,
// merged them into a single header used by both .c files.
// 14/06/07 Alan Cox <alan@redhat.com>
// - Grand cleanup and Linuxisation
//

pub const ORC_MAXQUEUE: c_int = 245;
pub const ORC_MAXTAGS: c_int = 64;

pub const ORC_MAXQUEUE: c_int = 25;
pub const ORC_MAXTAGS: c_int = 8;

pub const TOTAL_SG_ENTRY: c_int = 32;
pub const MAX_TARGETS: c_int = 16;
pub const IMAX_CDB: c_int = 15;
pub const SENSE_SIZE: c_int = 14;
//
// Scatter-Gather Element Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_sgent {
    pub /: *mut *mut u32 base; / Data Pointer,
    pub /: *mut *mut u32 length; / Data Length,
}

// SCSI related definition
pub const DISC_NOT_ALLOW: c_uint = 0x80	/* Disconnect is not allowed    */;
pub const DISC_ALLOW: c_uint = 0xC0	/* Disconnect is allowed        */;
pub const ORC_OFFSET_SCB: c_int = 16;
pub const ORC_MAX_SCBS: c_int = 250;
pub const MAX_CHANNELS: c_int = 2;
pub const MAX_ESCB_ELE: c_int = 64;
pub const TCF_DRV_255_63: c_uint = 0x0400;
//
// Orchid Host Command Set
//
pub const ORC_CMD_NOP: c_uint = 0x00	/* Host command - NOP             */;
pub const ORC_CMD_VERSION: c_uint = 0x01	/* Host command - Get F/W version */;
pub const ORC_CMD_ECHO: c_uint = 0x02	/* Host command - ECHO            */;
pub const ORC_CMD_SET_NVM: c_uint = 0x03	/* Host command - Set NVRAM       */;
pub const ORC_CMD_GET_NVM: c_uint = 0x04	/* Host command - Get NVRAM       */;
pub const ORC_CMD_GET_BUS_STATUS: c_uint = 0x05	/* Host command - Get SCSI bus status */;
pub const ORC_CMD_ABORT_SCB: c_uint = 0x06	/* Host command - Abort SCB       */;
pub const ORC_CMD_ISSUE_SCB: c_uint = 0x07	/* Host command - Issue SCB       */;
//
// Orchid Register Set
//
pub const ORC_GINTS: c_uint = 0xA0	/* Global Interrupt Status        */;
pub const QINT: c_uint = 0x04	/* Reply Queue Interrupt  */;
pub const ORC_GIMSK: c_uint = 0xA1	/* Global Interrupt MASK  */;
pub const MQINT: c_uint = 0x04	/* Mask Reply Queue Interrupt     */;
pub const ORC_GCFG: c_uint = 0xA2	/* Global Configure               */;
pub const EEPRG: c_uint = 0x01	/* Enable EEPROM programming */;
pub const ORC_GSTAT: c_uint = 0xA3	/* Global status          */;
pub const WIDEBUS: c_uint = 0x10	/* Wide SCSI Devices connected    */;
pub const ORC_HDATA: c_uint = 0xA4	/* Host Data                      */;
pub const ORC_HCTRL: c_uint = 0xA5	/* Host Control                   */;
pub const SCSIRST: c_uint = 0x80	/* SCSI bus reset         */;
pub const HDO: c_uint = 0x40	/* Host data out          */;
pub const HOSTSTOP: c_uint = 0x02	/* Host stop RISC engine  */;
pub const DEVRST: c_uint = 0x01	/* Device reset                   */;
pub const ORC_HSTUS: c_uint = 0xA6	/* Host Status                    */;
pub const HDI: c_uint = 0x02	/* Host data in                   */;
pub const RREADY: c_uint = 0x01	/* RISC engine is ready to receive */;
pub const ORC_NVRAM: c_uint = 0xA7	/* Nvram port address             */;
pub const SE2CS: c_uint = 0x008;
pub const SE2CLK: c_uint = 0x004;
pub const SE2DO: c_uint = 0x002;
pub const SE2DI: c_uint = 0x001;
pub const ORC_PQUEUE: c_uint = 0xA8	/* Posting queue FIFO             */;
pub const ORC_PQCNT: c_uint = 0xA9	/* Posting queue FIFO Cnt */;
pub const ORC_RQUEUE: c_uint = 0xAA	/* Reply queue FIFO               */;
pub const ORC_RQUEUECNT: c_uint = 0xAB	/* Reply queue FIFO Cnt           */;
pub const ORC_FWBASEADR: c_uint = 0xAC	/* Firmware base address  */;
pub const ORC_EBIOSADR0: c_uint = 0xB0	/* External Bios address */;
pub const ORC_EBIOSADR1: c_uint = 0xB1	/* External Bios address */;
pub const ORC_EBIOSADR2: c_uint = 0xB2	/* External Bios address */;
pub const ORC_EBIOSDATA: c_uint = 0xB3	/* External Bios address */;
pub const ORC_SCBSIZE: c_uint = 0xB7	/* SCB size register              */;
pub const ORC_SCBBASE0: c_uint = 0xB8	/* SCB base address 0             */;
pub const ORC_SCBBASE1: c_uint = 0xBC	/* SCB base address 1             */;
pub const ORC_RISCCTL: c_uint = 0xE0	/* RISC Control                   */;
pub const PRGMRST: c_uint = 0x002;
pub const DOWNLOAD: c_uint = 0x001;
pub const ORC_PRGMCTR0: c_uint = 0xE2	/* RISC program counter           */;
pub const ORC_PRGMCTR1: c_uint = 0xE3	/* RISC program counter           */;
pub const ORC_RISCRAM: c_uint = 0xEC	/* RISC RAM data port 4 bytes     */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_extended_scb {
    pub /: *mut *mut orc_sgent sglist[TOTAL_SG_ENTRY]; /0 Start of SG list,
    pub /: *mut *mut *mut scsi_cmnd srb; /50 SRB Pointer,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_scb {
    pub /: *mut *mut u8 opcode; /00 SCB command code&residual,
    pub /: *mut *mut u8 flags; /01 SCB Flags,
    pub /: *mut *mut u8 target; /02 Target Id,
    pub /: *mut *mut u8 lun; /03 Lun,
    pub /: *mut *mut u32 reserved0; /04 Reserved for ORCHID must 0,
    pub /: *mut *mut u32 xferlen; /08 Data Transfer Length,
    pub /: *mut *mut u32 reserved1; /0C Reserved for ORCHID must 0,
    pub /: *mut *mut *mut u32 sg_len; /10 SG list #  8,
    pub /: *mut *mut u32 sg_addr; /14 SG List Buf physical Addr,
    pub /: *mut *mut u32 sg_addrhigh; /18 SG Buffer high physical Addr,
    pub /: *mut *mut u8 hastat; /1C Host Status,
    pub /: *mut *mut u8 tastat; /1D Target Status,
    pub /: *mut *mut u8 status; /1E SCB status,
    pub /: *mut *mut u8 link; /1F Link pointer, default 0xFF,
    pub /: *mut *mut u8 sense_len; /20 Sense Allocation Length,
    pub /: *mut *mut u8 cdb_len; /21 CDB Length,
    pub /: *mut *mut u8 ident; /22 Identify,
    pub /: *mut *mut u8 tag_msg; /23 Tag Message,
    pub /: *mut *mut u8 cdb[IMAX_CDB]; /24 SCSI CDBs,
    pub /: *mut *mut u8 scbidx; /3C Index for this ORCSCB,
    pub /: *mut *mut u32 sense_addr; /34 Sense Buffer physical Addr,
    pub /: *mut *mut *mut orc_extended_scb escb; /38 Extended SCB Pointer,
// 64bit pointer or 32bit pointer + reserved ?

    pub /: *mut *mut u8 reserved2[4]; /3E Reserved for Driver use,

}

// Opcodes of ORCSCB_Opcode
pub const ORC_EXECSCSI: c_uint = 0x00	/* SCSI initiator command with residual */;
pub const ORC_BUSDEVRST: c_uint = 0x01	/* SCSI Bus Device Reset  */;
// Status of ORCSCB_Status
pub const ORCSCB_COMPLETE: c_uint = 0x00	/* SCB request completed  */;
pub const ORCSCB_POST: c_uint = 0x01	/* SCB is posted by the HOST      */;
// Bit Definition for ORCSCB_Flags
pub const SCF_DISINT: c_uint = 0x01	/* Disable HOST interrupt */;
pub const SCF_DIR: c_uint = 0x18	/* Direction bits         */;
pub const SCF_NO_DCHK: c_uint = 0x00	/* Direction determined by SCSI   */;
pub const SCF_DIN: c_uint = 0x08	/* From Target to Initiator       */;
pub const SCF_DOUT: c_uint = 0x10	/* From Initiator to Target       */;
pub const SCF_NO_XF: c_uint = 0x18	/* No data transfer               */;
pub const SCF_POLL: c_uint = 0x40;
// Error Codes for ORCSCB_HaStat
pub const HOST_SEL_TOUT: c_uint = 0x11;
pub const HOST_DO_DU: c_uint = 0x12;
pub const HOST_BUS_FREE: c_uint = 0x13;
pub const HOST_BAD_PHAS: c_uint = 0x14;
pub const HOST_INV_CMD: c_uint = 0x16;
pub const HOST_SCSI_RST: c_uint = 0x1B;
pub const HOST_DEV_RST: c_uint = 0x1C;
// Error Codes for ORCSCB_TaStat
pub const TARGET_CHK_COND: c_uint = 0x02;
pub const TARGET_BUSY: c_uint = 0x08;
pub const TARGET_TAG_FULL: c_uint = 0x28;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_target {
    pub /: *mut *mut u8 TCS_DrvDASD; / 6,
    pub /: *mut *mut u8 TCS_DrvSCSI; / 7,
    pub /: *mut *mut u8 TCS_DrvHead; / 8,
    pub /: *mut *mut u16 TCS_DrvFlags; / 4,
    pub /: *mut *mut u8 TCS_DrvSector; / 7,
}

// Bit Definition for TCF_DrvFlags
pub const TCS_DF_NODASD_SUPT: c_uint = 0x20	/* Suppress OS/2 DASD Mgr support */;
pub const TCS_DF_NOSCSI_SUPT: c_uint = 0x40	/* Suppress OS/2 SCSI Mgr support */;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_host {
    pub /: *mut *mut unsigned long base; / Base address,
    pub (Channel)*/: *mut *mut u8 index; / Index,
    pub /: *mut *mut u8 scsi_id; / H/A SCSI ID,
    pub /: *mut *mut u8 BIOScfg; /BIOS configuration,
    pub flags: u8,
    pub /: *mut *mut u8 max_targets; / SCSI0MAXTags,
    pub /: *mut *mut *mut orc_scb scb_virt; / Virtual Pointer to SCB array,
    pub /: *mut *mut dma_addr_t scb_phys; / Scb Physical address,
    pub /: *mut *mut *mut orc_extended_scb escb_virt; / Virtual pointer to ESCB Scatter list,
    pub /: *mut *mut dma_addr_t escb_phys; / scatter list Physical address,
    pub /: *mut *mut u8 target_flag[16]; / target configuration, TCF_EN_TAG,
    pub /: *mut *mut u8 max_tags[16]; / ORC_MAX_SCBS,
    pub /: *mut *mut u32 allocation_map[MAX_CHANNELS][8]; / Max STB is 256, So 256/32,
    pub allocation_lock: spinlock_t,
    pub pdev: *mut pci_dev,
}

// Bit Definition for HCS_Flags
pub const HCF_SCSI_RESET: c_uint = 0x01	/* SCSI BUS RESET         */;
pub const HCF_PARITY: c_uint = 0x02	/* parity card                    */;
pub const HCF_LVDS: c_uint = 0x10	/* parity card                    */;
// Bit Definition for TargetFlag
pub const TCF_EN_255: c_uint = 0x08;
pub const TCF_EN_TAG: c_uint = 0x10;
pub const TCF_BUSY: c_uint = 0x20;
pub const TCF_DISCONNECT: c_uint = 0x40;
pub const TCF_SPIN_UP: c_uint = 0x80;
// Bit Definition for HCS_AFlags
pub const HCS_AF_IGNORE: c_uint = 0x01	/* Adapter ignore         */;
pub const HCS_AF_DISABLE_RESET: c_uint = 0x10	/* Adapter disable reset  */;
pub const HCS_AF_DISABLE_ADPT: c_uint = 0x80	/* Adapter disable                */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_nvram {
// ----------header ---------------
    pub /: *mut *mut u8 SubVendorID0; / 00 - Sub Vendor ID,
    pub /: *mut *mut u8 SubVendorID1; / 00 - Sub Vendor ID,
    pub /: *mut *mut u8 SubSysID0; / 02 - Sub System ID,
    pub /: *mut *mut u8 SubSysID1; / 02 - Sub System ID,
    pub /: *mut *mut u8 SubClass; / 04 - Sub Class,
    pub /: *mut *mut u8 VendorID0; / 05 - Vendor ID,
    pub /: *mut *mut u8 VendorID1; / 05 - Vendor ID,
    pub /: *mut *mut u8 DeviceID0; / 07 - Device ID,
    pub /: *mut *mut u8 DeviceID1; / 07 - Device ID,
    pub /: *mut *mut u8 Reserved0[2]; / 09 - Reserved,
    pub /: *mut *mut u8 revision; / 0B - revision of data structure,
// ----Host Adapter Structure ----
    pub /: *mut *mut u8 NumOfCh; / 0C - Number of SCSI channel,
    pub /: *mut *mut u8 BIOSConfig1; / 0D - BIOS configuration 1,
    pub /: *mut *mut u8 BIOSConfig2; / 0E - BIOS boot channel&target ID,
    pub /: *mut *mut u8 BIOSConfig3; / 0F - BIOS configuration 3,
// ----SCSI channel Structure ----
// from "CTRL-I SCSI Host Adapter SetUp menu "
    pub /: *mut *mut u8 scsi_id; / 10 - Channel 0 SCSI ID,
    pub /: *mut *mut u8 SCSI0Config; / 11 - Channel 0 SCSI configuration,
    pub /: *mut *mut u8 SCSI0MaxTags; / 12 - Channel 0 Maximum tags,
    pub /: *mut *mut u8 SCSI0ResetTime; / 13 - Channel 0 Reset recovering time,
    pub /: *mut *mut u8 ReservedforChannel0[2]; / 14 - Reserved,
// ----SCSI target Structure ----
// from "CTRL-I SCSI device SetUp menu "
    pub /: *mut *mut u8 Target00Config; / 16 - Channel 0 Target 0 config,
    pub /: *mut *mut u8 Target01Config; / 17 - Channel 0 Target 1 config,
    pub /: *mut *mut u8 Target02Config; / 18 - Channel 0 Target 2 config,
    pub /: *mut *mut u8 Target03Config; / 19 - Channel 0 Target 3 config,
    pub /: *mut *mut u8 Target04Config; / 1A - Channel 0 Target 4 config,
    pub /: *mut *mut u8 Target05Config; / 1B - Channel 0 Target 5 config,
    pub /: *mut *mut u8 Target06Config; / 1C - Channel 0 Target 6 config,
    pub /: *mut *mut u8 Target07Config; / 1D - Channel 0 Target 7 config,
    pub /: *mut *mut u8 Target08Config; / 1E - Channel 0 Target 8 config,
    pub /: *mut *mut u8 Target09Config; / 1F - Channel 0 Target 9 config,
    pub /: *mut *mut u8 Target0AConfig; / 20 - Channel 0 Target A config,
    pub /: *mut *mut u8 Target0BConfig; / 21 - Channel 0 Target B config,
    pub /: *mut *mut u8 Target0CConfig; / 22 - Channel 0 Target C config,
    pub /: *mut *mut u8 Target0DConfig; / 23 - Channel 0 Target D config,
    pub /: *mut *mut u8 Target0EConfig; / 24 - Channel 0 Target E config,
    pub /: *mut *mut u8 Target0FConfig; / 25 - Channel 0 Target F config,
    pub /: *mut *mut u8 SCSI1Id; / 26 - Channel 1 SCSI ID,
    pub /: *mut *mut u8 SCSI1Config; / 27 - Channel 1 SCSI configuration,
    pub /: *mut *mut u8 SCSI1MaxTags; / 28 - Channel 1 Maximum tags,
    pub /: *mut *mut u8 SCSI1ResetTime; / 29 - Channel 1 Reset recovering time,
    pub /: *mut *mut u8 ReservedforChannel1[2]; / 2A - Reserved,
// ----SCSI target Structure ----
// from "CTRL-I SCSI device SetUp menu "
    pub /: *mut *mut u8 Target10Config; / 2C - Channel 1 Target 0 config,
    pub /: *mut *mut u8 Target11Config; / 2D - Channel 1 Target 1 config,
    pub /: *mut *mut u8 Target12Config; / 2E - Channel 1 Target 2 config,
    pub /: *mut *mut u8 Target13Config; / 2F - Channel 1 Target 3 config,
    pub /: *mut *mut u8 Target14Config; / 30 - Channel 1 Target 4 config,
    pub /: *mut *mut u8 Target15Config; / 31 - Channel 1 Target 5 config,
    pub /: *mut *mut u8 Target16Config; / 32 - Channel 1 Target 6 config,
    pub /: *mut *mut u8 Target17Config; / 33 - Channel 1 Target 7 config,
    pub /: *mut *mut u8 Target18Config; / 34 - Channel 1 Target 8 config,
    pub /: *mut *mut u8 Target19Config; / 35 - Channel 1 Target 9 config,
    pub /: *mut *mut u8 Target1AConfig; / 36 - Channel 1 Target A config,
    pub /: *mut *mut u8 Target1BConfig; / 37 - Channel 1 Target B config,
    pub /: *mut *mut u8 Target1CConfig; / 38 - Channel 1 Target C config,
    pub /: *mut *mut u8 Target1DConfig; / 39 - Channel 1 Target D config,
    pub /: *mut *mut u8 Target1EConfig; / 3A - Channel 1 Target E config,
    pub /: *mut *mut u8 Target1FConfig; / 3B - Channel 1 Target F config,
    pub /: *mut *mut u8 reserved[3]; / 3C - Reserved,
// ---------- CheckSum ----------
    pub /: *mut *mut u8 CheckSum; / 3F - Checksum of NVRam,
}

// Bios Configuration for nvram->BIOSConfig1
pub const NBC_BIOSENABLE: c_uint = 0x01    /* BIOS enable                    */;
pub const NBC_CDROM: c_uint = 0x02    /* Support bootable CDROM */;
pub const NBC_REMOVABLE: c_uint = 0x04    /* Support removable drive        */;
// Bios Configuration for nvram->BIOSConfig2
pub const NBB_TARGET_MASK: c_uint = 0x0F    /* Boot SCSI target ID number     */;
pub const NBB_CHANL_MASK: c_uint = 0xF0    /* Boot SCSI channel number       */;
// Bit definition for nvram->SCSIConfig
pub const NCC_BUSRESET: c_uint = 0x01    /* Reset SCSI bus at power up     */;
pub const NCC_PARITYCHK: c_uint = 0x02    /* SCSI parity enable             */;
pub const NCC_LVDS: c_uint = 0x10    /* Enable LVDS                    */;
pub const NCC_ACTTERM1: c_uint = 0x20    /* Enable active terminator 1     */;
pub const NCC_ACTTERM2: c_uint = 0x40    /* Enable active terminator 2     */;
pub const NCC_AUTOTERM: c_uint = 0x80    /* Enable auto termination        */;
// Bit definition for nvram->TargetxConfig
pub const NTC_PERIOD: c_uint = 0x07    /* Maximum Sync. Speed            */;
pub const NTC_1GIGA: c_uint = 0x08    /* 255 head / 63 sectors (64/32) */;
pub const NTC_NO_SYNC: c_uint = 0x10    /* NO SYNC. NEGO          */;
pub const NTC_NO_WIDESYNC: c_uint = 0x20    /* NO WIDE SYNC. NEGO             */;
pub const NTC_DISC_ENABLE: c_uint = 0x40    /* Enable SCSI disconnect */;
pub const NTC_SPINUP: c_uint = 0x80    /* Start disk drive               */;
// Default NVRam values

pub const NCC_MAX_TAGS: c_uint = 0x20    /* Maximum tags per target        */;
pub const NCC_RESET_TIME: c_uint = 0x0A    /* SCSI RESET recovering time     */;
