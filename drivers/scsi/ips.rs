//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ips.h
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
// ips.h -- driver for the Adaptec / IBM ServeRAID controller
//
// Written By: Keith Mitchell, IBM Corporation
// Jack Hammer, Adaptec, Inc.
// David Jeffery, Adaptec, Inc.
//
// Copyright (C) 1999 IBM Corporation
// Copyright (C) 2003 Adaptec, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// NO WARRANTY
// THE PROGRAM IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED INCLUDING, WITHOUT
// LIMITATION, ANY WARRANTIES OR CONDITIONS OF TITLE, NON-INFRINGEMENT,
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. Each Recipient is
// solely responsible for determining the appropriateness of using and
// distributing the Program and assumes all risks associated with its
// exercise of rights under this Agreement, including but not limited to
// the risks and costs of program errors, damage to or loss of data,
// programs or equipment, and unavailability or interruption of operations.
//
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// Bugs/Comments/Suggestions should be mailed to:
// ipslinux@adaptec.com
//

//
// Some handy macros
//

// Macro flag: #define __iomem

//
// Adapter address map equates
//
pub const IPS_REG_HISR: c_uint = 0x08    /* Host Interrupt Status Reg   */;
pub const IPS_REG_CCSAR: c_uint = 0x10    /* Cmd Channel System Addr Reg */;
pub const IPS_REG_CCCR: c_uint = 0x14    /* Cmd Channel Control Reg     */;
pub const IPS_REG_SQHR: c_uint = 0x20    /* Status Q Head Reg           */;
pub const IPS_REG_SQTR: c_uint = 0x24    /* Status Q Tail Reg           */;
pub const IPS_REG_SQER: c_uint = 0x28    /* Status Q End Reg            */;
pub const IPS_REG_SQSR: c_uint = 0x2C    /* Status Q Start Reg          */;
pub const IPS_REG_SCPR: c_uint = 0x05    /* Subsystem control port reg  */;
pub const IPS_REG_ISPR: c_uint = 0x06    /* interrupt status port reg   */;
pub const IPS_REG_CBSP: c_uint = 0x07    /* CBSP register               */;
pub const IPS_REG_FLAP: c_uint = 0x18    /* Flash address port          */;
pub const IPS_REG_FLDP: c_uint = 0x1C    /* Flash data port             */;
pub const IPS_REG_NDAE: c_uint = 0x38    /* Anaconda 64 NDAE Register   */;
pub const IPS_REG_I2O_INMSGQ: c_uint = 0x40    /* I2O Inbound Message Queue   */;
pub const IPS_REG_I2O_OUTMSGQ: c_uint = 0x44    /* I2O Outbound Message Queue  */;
pub const IPS_REG_I2O_HIR: c_uint = 0x30    /* I2O Interrupt Status        */;
pub const IPS_REG_I960_IDR: c_uint = 0x20    /* i960 Inbound Doorbell       */;
pub const IPS_REG_I960_MSG0: c_uint = 0x18    /* i960 Outbound Reg 0         */;
pub const IPS_REG_I960_MSG1: c_uint = 0x1C    /* i960 Outbound Reg 1         */;
pub const IPS_REG_I960_OIMR: c_uint = 0x34    /* i960 Oubound Int Mask Reg   */;
//
// Adapter register bit equates
//
pub const IPS_BIT_GHI: c_uint = 0x04    /* HISR General Host Interrupt */;
pub const IPS_BIT_SQO: c_uint = 0x02    /* HISR Status Q Overflow      */;
pub const IPS_BIT_SCE: c_uint = 0x01    /* HISR Status Channel Enqueue */;
pub const IPS_BIT_SEM: c_uint = 0x08    /* CCCR Semaphore Bit          */;
pub const IPS_BIT_ILE: c_uint = 0x10    /* CCCR ILE Bit                */;
pub const IPS_BIT_START_CMD: c_uint = 0x101A  /* CCCR Start Command Channel  */;
pub const IPS_BIT_START_STOP: c_uint = 0x0002  /* CCCR Start/Stop Bit         */;
pub const IPS_BIT_RST: c_uint = 0x80    /* SCPR Reset Bit              */;
pub const IPS_BIT_EBM: c_uint = 0x02    /* SCPR Enable Bus Master      */;
pub const IPS_BIT_EI: c_uint = 0x80    /* HISR Enable Interrupts      */;
pub const IPS_BIT_OP: c_uint = 0x01    /* OP bit in CBSP              */;
pub const IPS_BIT_I2O_OPQI: c_uint = 0x08    /* General Host Interrupt      */;
pub const IPS_BIT_I960_MSG0I: c_uint = 0x01    /* Message Register 0 Interrupt*/;
pub const IPS_BIT_I960_MSG1I: c_uint = 0x02    /* Message Register 1 Interrupt*/;
//
// Adapter Command ID Equates
//
pub const IPS_CMD_GET_LD_INFO: c_uint = 0x19;
pub const IPS_CMD_GET_SUBSYS: c_uint = 0x40;
pub const IPS_CMD_READ_CONF: c_uint = 0x38;
pub const IPS_CMD_RW_NVRAM_PAGE: c_uint = 0xBC;
pub const IPS_CMD_READ: c_uint = 0x02;
pub const IPS_CMD_WRITE: c_uint = 0x03;
pub const IPS_CMD_FFDC: c_uint = 0xD7;
pub const IPS_CMD_ENQUIRY: c_uint = 0x05;
pub const IPS_CMD_FLUSH: c_uint = 0x0A;
pub const IPS_CMD_READ_SG: c_uint = 0x82;
pub const IPS_CMD_WRITE_SG: c_uint = 0x83;
pub const IPS_CMD_DCDB: c_uint = 0x04;
pub const IPS_CMD_DCDB_SG: c_uint = 0x84;
pub const IPS_CMD_EXTENDED_DCDB: c_uint = 0x95;
pub const IPS_CMD_EXTENDED_DCDB_SG: c_uint = 0x96;
pub const IPS_CMD_CONFIG_SYNC: c_uint = 0x58;
pub const IPS_CMD_ERROR_TABLE: c_uint = 0x17;
pub const IPS_CMD_DOWNLOAD: c_uint = 0x20;
pub const IPS_CMD_RW_BIOSFW: c_uint = 0x22;
pub const IPS_CMD_GET_VERSION_INFO: c_uint = 0xC6;
pub const IPS_CMD_RESET_CHANNEL: c_uint = 0x1A;
//
// Adapter Equates
//
pub const IPS_CSL: c_uint = 0xFF;
pub const IPS_POCL: c_uint = 0x30;
pub const IPS_NORM_STATE: c_uint = 0x00;
pub const IPS_MAX_ADAPTER_TYPES: c_int = 3;
pub const IPS_MAX_ADAPTERS: c_int = 16;
pub const IPS_MAX_IOCTL: c_int = 1;
pub const IPS_MAX_IOCTL_QUEUE: c_int = 8;
pub const IPS_MAX_QUEUE: c_int = 128;
pub const IPS_BLKSIZE: c_int = 512;
pub const IPS_MAX_SG: c_int = 17;
pub const IPS_MAX_LD: c_int = 8;
pub const IPS_MAX_CHANNELS: c_int = 4;
pub const IPS_MAX_TARGETS: c_int = 15;
pub const IPS_MAX_CHUNKS: c_int = 16;
pub const IPS_MAX_CMDS: c_int = 128;
pub const IPS_MAX_XFER: c_uint = 0x10000;
pub const IPS_NVRAM_P5_SIG: c_uint = 0xFFDDBB99;
pub const IPS_MAX_POST_BYTES: c_uint = 0x02;
pub const IPS_MAX_CONFIG_BYTES: c_uint = 0x02;
pub const IPS_GOOD_POST_STATUS: c_uint = 0x80;
pub const IPS_SEM_TIMEOUT: c_int = 2000;
pub const IPS_IOCTL_COMMAND: c_uint = 0x0D;
pub const IPS_INTR_ON: c_int = 0;
pub const IPS_INTR_IORL: c_int = 1;
pub const IPS_FFDC: c_int = 99;
pub const IPS_ADAPTER_ID: c_uint = 0xF;
pub const IPS_VENDORID_IBM: c_uint = 0x1014;
pub const IPS_VENDORID_ADAPTEC: c_uint = 0x9005;
pub const IPS_DEVICEID_COPPERHEAD: c_uint = 0x002E;
pub const IPS_DEVICEID_MORPHEUS: c_uint = 0x01BD;
pub const IPS_DEVICEID_MARCO: c_uint = 0x0250;
pub const IPS_SUBDEVICEID_4M: c_uint = 0x01BE;
pub const IPS_SUBDEVICEID_4L: c_uint = 0x01BF;
pub const IPS_SUBDEVICEID_4MX: c_uint = 0x0208;
pub const IPS_SUBDEVICEID_4LX: c_uint = 0x020E;
pub const IPS_SUBDEVICEID_5I2: c_uint = 0x0259;
pub const IPS_SUBDEVICEID_5I1: c_uint = 0x0258;
pub const IPS_SUBDEVICEID_6M: c_uint = 0x0279;
pub const IPS_SUBDEVICEID_6I: c_uint = 0x028C;
pub const IPS_SUBDEVICEID_7k: c_uint = 0x028E;
pub const IPS_SUBDEVICEID_7M: c_uint = 0x028F;
pub const IPS_IOCTL_SIZE: c_int = 8192;
pub const IPS_STATUS_SIZE: c_int = 4;

pub const IPS_MEMMAP_SIZE: c_int = 128;
pub const IPS_ONE_MSEC: c_int = 1;
pub const IPS_ONE_SEC: c_int = 1000;
//
// Geometry Settings
//
pub const IPS_COMP_HEADS: c_int = 128;
pub const IPS_COMP_SECTORS: c_int = 32;
pub const IPS_NORM_HEADS: c_int = 254;
pub const IPS_NORM_SECTORS: c_int = 63;
//
// Adapter Basic Status Codes
//
pub const IPS_BASIC_STATUS_MASK: c_uint = 0xFF;
pub const IPS_GSC_STATUS_MASK: c_uint = 0x0F;
pub const IPS_CMD_SUCCESS: c_uint = 0x00;
pub const IPS_CMD_RECOVERED_ERROR: c_uint = 0x01;
pub const IPS_INVAL_OPCO: c_uint = 0x03;
pub const IPS_INVAL_CMD_BLK: c_uint = 0x04;
pub const IPS_INVAL_PARM_BLK: c_uint = 0x05;
pub const IPS_BUSY: c_uint = 0x08;
pub const IPS_CMD_CMPLT_WERROR: c_uint = 0x0C;
pub const IPS_LD_ERROR: c_uint = 0x0D;
pub const IPS_CMD_TIMEOUT: c_uint = 0x0E;
pub const IPS_PHYS_DRV_ERROR: c_uint = 0x0F;
//
// Adapter Extended Status Equates
//
pub const IPS_ERR_SEL_TO: c_uint = 0xF0;
pub const IPS_ERR_OU_RUN: c_uint = 0xF2;
pub const IPS_ERR_HOST_RESET: c_uint = 0xF7;
pub const IPS_ERR_DEV_RESET: c_uint = 0xF8;
pub const IPS_ERR_RECOVERY: c_uint = 0xFC;
pub const IPS_ERR_CKCOND: c_uint = 0xFF;
//
// Operating System Defines
//
pub const IPS_OS_WINDOWS_NT: c_uint = 0x01;
pub const IPS_OS_NETWARE: c_uint = 0x02;
pub const IPS_OS_OPENSERVER: c_uint = 0x03;
pub const IPS_OS_UNIXWARE: c_uint = 0x04;
pub const IPS_OS_SOLARIS: c_uint = 0x05;
pub const IPS_OS_OS2: c_uint = 0x06;
pub const IPS_OS_LINUX: c_uint = 0x07;
pub const IPS_OS_FREEBSD: c_uint = 0x08;
//
// Adapter Revision ID's
//
pub const IPS_REVID_SERVERAID: c_uint = 0x02;
pub const IPS_REVID_NAVAJO: c_uint = 0x03;
pub const IPS_REVID_SERVERAID2: c_uint = 0x04;
pub const IPS_REVID_CLARINETP1: c_uint = 0x05;
pub const IPS_REVID_CLARINETP2: c_uint = 0x07;
pub const IPS_REVID_CLARINETP3: c_uint = 0x0D;
pub const IPS_REVID_TROMBONE32: c_uint = 0x0F;
pub const IPS_REVID_TROMBONE64: c_uint = 0x10;
//
// NVRAM Page 5 Adapter Defines
//
pub const IPS_ADTYPE_SERVERAID: c_uint = 0x01;
pub const IPS_ADTYPE_SERVERAID2: c_uint = 0x02;
pub const IPS_ADTYPE_NAVAJO: c_uint = 0x03;
pub const IPS_ADTYPE_KIOWA: c_uint = 0x04;
pub const IPS_ADTYPE_SERVERAID3: c_uint = 0x05;
pub const IPS_ADTYPE_SERVERAID3L: c_uint = 0x06;
pub const IPS_ADTYPE_SERVERAID4H: c_uint = 0x07;
pub const IPS_ADTYPE_SERVERAID4M: c_uint = 0x08;
pub const IPS_ADTYPE_SERVERAID4L: c_uint = 0x09;
pub const IPS_ADTYPE_SERVERAID4MX: c_uint = 0x0A;
pub const IPS_ADTYPE_SERVERAID4LX: c_uint = 0x0B;
pub const IPS_ADTYPE_SERVERAID5I2: c_uint = 0x0C;
pub const IPS_ADTYPE_SERVERAID5I1: c_uint = 0x0D;
pub const IPS_ADTYPE_SERVERAID6M: c_uint = 0x0E;
pub const IPS_ADTYPE_SERVERAID6I: c_uint = 0x0F;
pub const IPS_ADTYPE_SERVERAID7t: c_uint = 0x10;
pub const IPS_ADTYPE_SERVERAID7k: c_uint = 0x11;
pub const IPS_ADTYPE_SERVERAID7M: c_uint = 0x12;
//
// Adapter Command/Status Packet Definitions
//
pub const IPS_SUCCESS: c_uint = 0x01 /* Successfully completed       */;
pub const IPS_SUCCESS_IMM: c_uint = 0x02 /* Success - Immediately        */;
pub const IPS_FAILURE: c_uint = 0x04 /* Completed with Error         */;
//
// Logical Drive Equates
//
pub const IPS_LD_OFFLINE: c_uint = 0x02;
pub const IPS_LD_OKAY: c_uint = 0x03;
pub const IPS_LD_FREE: c_uint = 0x00;
pub const IPS_LD_SYS: c_uint = 0x06;
pub const IPS_LD_CRS: c_uint = 0x24;
//
// DCDB Table Equates
//
pub const IPS_NO_DISCONNECT: c_uint = 0x00;
pub const IPS_DISCONNECT_ALLOWED: c_uint = 0x80;
pub const IPS_NO_AUTO_REQSEN: c_uint = 0x40;
pub const IPS_DATA_NONE: c_uint = 0x00;
pub const IPS_DATA_UNK: c_uint = 0x00;
pub const IPS_DATA_IN: c_uint = 0x01;
pub const IPS_DATA_OUT: c_uint = 0x02;
pub const IPS_TRANSFER64K: c_uint = 0x08;
pub const IPS_NOTIMEOUT: c_uint = 0x00;
pub const IPS_TIMEOUT10: c_uint = 0x10;
pub const IPS_TIMEOUT60: c_uint = 0x20;
pub const IPS_TIMEOUT20M: c_uint = 0x30;
//
// SCSI Inquiry Data Flags
//
pub const IPS_SCSI_INQ_TYPE_DASD: c_uint = 0x00;
pub const IPS_SCSI_INQ_TYPE_PROCESSOR: c_uint = 0x03;
pub const IPS_SCSI_INQ_LU_CONNECTED: c_uint = 0x00;
pub const IPS_SCSI_INQ_RD_REV2: c_uint = 0x02;
pub const IPS_SCSI_INQ_REV2: c_uint = 0x02;
pub const IPS_SCSI_INQ_REV3: c_uint = 0x03;
pub const IPS_SCSI_INQ_Address16: c_uint = 0x01;
pub const IPS_SCSI_INQ_Address32: c_uint = 0x02;
pub const IPS_SCSI_INQ_MedChanger: c_uint = 0x08;
pub const IPS_SCSI_INQ_MultiPort: c_uint = 0x10;
pub const IPS_SCSI_INQ_EncServ: c_uint = 0x40;
pub const IPS_SCSI_INQ_SoftReset: c_uint = 0x01;
pub const IPS_SCSI_INQ_CmdQue: c_uint = 0x02;
pub const IPS_SCSI_INQ_Linked: c_uint = 0x08;
pub const IPS_SCSI_INQ_Sync: c_uint = 0x10;
pub const IPS_SCSI_INQ_WBus16: c_uint = 0x20;
pub const IPS_SCSI_INQ_WBus32: c_uint = 0x40;
pub const IPS_SCSI_INQ_RelAdr: c_uint = 0x80;
//
// SCSI Request Sense Data Flags
//
pub const IPS_SCSI_REQSEN_VALID: c_uint = 0x80;
pub const IPS_SCSI_REQSEN_CURRENT_ERR: c_uint = 0x70;
pub const IPS_SCSI_REQSEN_NO_SENSE: c_uint = 0x00;
//
// SCSI Mode Page Equates
//
pub const IPS_SCSI_MP3_SoftSector: c_uint = 0x01;
pub const IPS_SCSI_MP3_HardSector: c_uint = 0x02;
pub const IPS_SCSI_MP3_Removeable: c_uint = 0x04;
pub const IPS_SCSI_MP3_AllocateSurface: c_uint = 0x08;
//
// HA Flags
//
pub const IPS_HA_ENH_SG: c_uint = 0x1;
//
// SCB Flags
//
pub const IPS_SCB_MAP_SG: c_uint = 0x00008;

//
// Passthru stuff
//

// flashing defines
pub const IPS_FW_IMAGE: c_uint = 0x00;
pub const IPS_BIOS_IMAGE: c_uint = 0x01;
pub const IPS_WRITE_FW: c_uint = 0x01;
pub const IPS_WRITE_BIOS: c_uint = 0x02;
pub const IPS_ERASE_BIOS: c_uint = 0x03;
pub const IPS_BIOS_HEADER: c_uint = 0xC0;
// time oriented stuff
pub const IPS_SECS_8HOURS: c_int = 28800;
//
// Scsi_Host Template
//
// Raid Command Formats
//
// --------------------------------------------------------------------------
// Data returned from a GetVersion Command
// --------------------------------------------------------------------------
// SubSystem Parameter[4]
pub const IPS_GET_VERSION_SUPPORT: c_uint = 0x00018000  /* Mask for Versioning Support */;
//
// SCSI Structures
//
// Inquiry Data Format
//
// Read Capacity Data Format
//
// Request Sense Data Format
//
// Sense Data Format - Page 3
//
// Sense Data Format - Page 4
//
// Sense Data Format - Page 8
//
// Sense Data Format - Block Descriptor (DASD)
//
// Sense Data Format - Mode Page Header
//
// Scatter Gather list format
//
// Status Info
//
// SCB Queue Format
//
// Wait queue_format
//
// forward decl for host structure
extern "C" {
    pub fn void(: *mut *mut ips_scb_callback) (ips_ha_t, : *mut ips_scb) -> typedef;
}
//
// SCB Format
//
// Passthru Command Format
//

// The Version Information below gets created by SED during the build process.
// Do not modify the next line; it's what SED is looking for to do the insert.
// Version Info
//
// VERSION.H -- version numbers and copyright notices in various formats
//
pub const IPS_VER_MAJOR: c_int = 7;

pub const IPS_VER_MINOR: c_int = 12;

pub const IPS_VER_BUILD: c_int = 05;

pub const IPS_RELEASE_ID: c_uint = 0x00020000;
pub const IPS_BUILD_IDENT: c_int = 761;

// Version numbers for various adapters

// Compatibility IDs for various adapters

pub const IPS_COMPAT_MAX_ADAPTER_TYPE: c_int = 18;
pub const IPS_COMPAT_ID_LENGTH: c_int = 8;
