//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/genwqe/genwqe_card.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// IBM Accelerator Family 'GenWQE'
//
// (C) Copyright IBM Corp. 2013
//
// Author: Frank Haverkamp <haver@linux.vnet.ibm.com>
// Author: Joerg-Stephan Vogt <jsvogt@de.ibm.com>
// Author: Michael Jung <mijung@gmx.net>
// Author: Michael Ruettger <michael@ibmra.de>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License (version 2 only)
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// User-space API for the GenWQE card. For debugging and test purposes
// the register addresses are included here too.
//

// Basename of sysfs, debugfs and /dev interfaces

pub const GENWQE_TYPE_ALTERA_230: c_uint = 0x00 /* GenWQE4 Stratix-IV-230 */;
pub const GENWQE_TYPE_ALTERA_530: c_uint = 0x01 /* GenWQE4 Stratix-IV-530 */;
pub const GENWQE_TYPE_ALTERA_A4: c_uint = 0x02 /* GenWQE5 A4 Stratix-V-A4 */;
pub const GENWQE_TYPE_ALTERA_A7: c_uint = 0x03 /* GenWQE5 A7 Stratix-V-A7 */;
// MMIO Unit offsets: Each UnitID occupies a defined address range

pub const GENWQE_MAX_UNITS: c_int = 3;
// Common offsets per UnitID
pub const IO_EXTENDED_ERROR_POINTER: c_uint = 0x00000048;
pub const IO_ERROR_INJECT_SELECTOR: c_uint = 0x00000060;
pub const IO_EXTENDED_DIAG_SELECTOR: c_uint = 0x00000070;
pub const IO_EXTENDED_DIAG_READ_MBX: c_uint = 0x00000078;

// UnitID 0: Service Layer Unit (SLU)
// SLU: Unit Configuration Register
pub const IO_SLU_UNITCFG: c_uint = 0x00000000;
pub const IO_SLU_UNITCFG_TYPE_MASK: c_uint = 0x000000000ff00000 /* 27:20 */;
// SLU: Fault Isolation Register (FIR) (ac_slu_fir)
pub const IO_SLU_FIR: c_uint = 0x00000008 /* read only, wr direct */;
pub const IO_SLU_FIR_CLR: c_uint = 0x00000010 /* read and clear */;
// SLU: First Error Capture Register (FEC/WOF)
pub const IO_SLU_FEC: c_uint = 0x00000018;
pub const IO_SLU_ERR_ACT_MASK: c_uint = 0x00000020;
pub const IO_SLU_ERR_ATTN_MASK: c_uint = 0x00000028;
pub const IO_SLU_FIRX1_ACT_MASK: c_uint = 0x00000030;
pub const IO_SLU_FIRX0_ACT_MASK: c_uint = 0x00000038;
pub const IO_SLU_SEC_LEM_DEBUG_OVR: c_uint = 0x00000040;
pub const IO_SLU_EXTENDED_ERR_PTR: c_uint = 0x00000048;
pub const IO_SLU_COMMON_CONFIG: c_uint = 0x00000060;
pub const IO_SLU_FLASH_FIR: c_uint = 0x00000108;
pub const IO_SLU_SLC_FIR: c_uint = 0x00000110;
pub const IO_SLU_RIU_TRAP: c_uint = 0x00000280;
pub const IO_SLU_FLASH_FEC: c_uint = 0x00000308;
pub const IO_SLU_SLC_FEC: c_uint = 0x00000310;
//
// The  Virtual Function's Access is from offset 0x00010000
// The Physical Function's Access is from offset 0x00050000
// Single Shared Registers exists only at offset 0x00060000
//
// SLC: Queue Virtual Window Window for accessing into a specific VF
// queue. When accessing the 0x10000 space using the 0x50000 address
// segment, the value indicated here is used to specify which VF
// register is decoded. This register, and the 0x50000 register space
// can only be accessed by the PF. Example, if this register is set to
// 0x2, then a read from 0x50000 is the same as a read from 0x10000
// from VF=2.
//
// SLC: Queue Segment
pub const IO_SLC_QUEUE_SEGMENT: c_uint = 0x00010000;
pub const IO_SLC_VF_QUEUE_SEGMENT: c_uint = 0x00050000;
// SLC: Queue Offset
pub const IO_SLC_QUEUE_OFFSET: c_uint = 0x00010008;
pub const IO_SLC_VF_QUEUE_OFFSET: c_uint = 0x00050008;
// SLC: Queue Configuration
pub const IO_SLC_QUEUE_CONFIG: c_uint = 0x00010010;
pub const IO_SLC_VF_QUEUE_CONFIG: c_uint = 0x00050010;
// SLC: Job Timout/Only accessible for the PF
pub const IO_SLC_APPJOB_TIMEOUT: c_uint = 0x00010018;
pub const IO_SLC_VF_APPJOB_TIMEOUT: c_uint = 0x00050018;
pub const TIMEOUT_250MS: c_uint = 0x0000000f;
pub const HEARTBEAT_DISABLE: c_uint = 0x0000ff00;
// SLC: Queue InitSequence Register
pub const IO_SLC_QUEUE_INITSQN: c_uint = 0x00010020;
pub const IO_SLC_VF_QUEUE_INITSQN: c_uint = 0x00050020;
// SLC: Queue Wrap
pub const IO_SLC_QUEUE_WRAP: c_uint = 0x00010028;
pub const IO_SLC_VF_QUEUE_WRAP: c_uint = 0x00050028;
// SLC: Queue Status
pub const IO_SLC_QUEUE_STATUS: c_uint = 0x00010100;
pub const IO_SLC_VF_QUEUE_STATUS: c_uint = 0x00050100;
// SLC: Queue Working Time
pub const IO_SLC_QUEUE_WTIME: c_uint = 0x00010030;
pub const IO_SLC_VF_QUEUE_WTIME: c_uint = 0x00050030;
// SLC: Queue Error Counts
pub const IO_SLC_QUEUE_ERRCNTS: c_uint = 0x00010038;
pub const IO_SLC_VF_QUEUE_ERRCNTS: c_uint = 0x00050038;
// SLC: Queue Loast Response Word
pub const IO_SLC_QUEUE_LRW: c_uint = 0x00010040;
pub const IO_SLC_VF_QUEUE_LRW: c_uint = 0x00050040;
// SLC: Freerunning Timer
pub const IO_SLC_FREE_RUNNING_TIMER: c_uint = 0x00010108;
pub const IO_SLC_VF_FREE_RUNNING_TIMER: c_uint = 0x00050108;
// SLC: Queue Virtual Access Region
pub const IO_PF_SLC_VIRTUAL_REGION: c_uint = 0x00050000;
// SLC: Queue Virtual Window
pub const IO_PF_SLC_VIRTUAL_WINDOW: c_uint = 0x00060000;
// SLC: DDCB Application Job Pending [n] (n=0:63)

// SLC: Parser Trap RAM [n] (n=0:31)

// SLC: Dispatcher Trap RAM [n] (n=0:31)

// Global Fault Isolation Register (GFIR)
pub const IO_SLC_CFGREG_GFIR: c_uint = 0x00020000;
pub const GFIR_ERR_TRIGGER: c_uint = 0x0000ffff;
// SLU: Soft Reset Register
pub const IO_SLC_CFGREG_SOFTRESET: c_uint = 0x00020018;
// SLU: Misc Debug Register
pub const IO_SLC_MISC_DEBUG: c_uint = 0x00020060;
pub const IO_SLC_MISC_DEBUG_CLR: c_uint = 0x00020068;
pub const IO_SLC_MISC_DEBUG_SET: c_uint = 0x00020070;
// Temperature Sensor Reading
pub const IO_SLU_TEMPERATURE_SENSOR: c_uint = 0x00030000;
pub const IO_SLU_TEMPERATURE_CONFIG: c_uint = 0x00030008;
// Voltage Margining Control
pub const IO_SLU_VOLTAGE_CONTROL: c_uint = 0x00030080;
pub const IO_SLU_VOLTAGE_NOMINAL: c_uint = 0x00000000;
pub const IO_SLU_VOLTAGE_DOWN5: c_uint = 0x00000006;
pub const IO_SLU_VOLTAGE_UP5: c_uint = 0x00000007;
// Direct LED Control Register
pub const IO_SLU_LEDCONTROL: c_uint = 0x00030100;
// SLU: Flashbus Direct Access -A5
pub const IO_SLU_FLASH_DIRECTACCESS: c_uint = 0x00040010;
// SLU: Flashbus Direct Access2 -A5
pub const IO_SLU_FLASH_DIRECTACCESS2: c_uint = 0x00040020;
// SLU: Flashbus Command Interface -A5
pub const IO_SLU_FLASH_CMDINTF: c_uint = 0x00040030;
// SLU: BitStream Loaded
pub const IO_SLU_BITSTREAM: c_uint = 0x00040040;
// This Register has a switch which will change the CAs to UR
pub const IO_HSU_ERR_BEHAVIOR: c_uint = 0x01001010;
pub const IO_SLC2_SQB_TRAP: c_uint = 0x00062000;
pub const IO_SLC2_QUEUE_MANAGER_TRAP: c_uint = 0x00062008;
pub const IO_SLC2_FLS_MASTER_TRAP: c_uint = 0x00062010;
// UnitID 1: HSU Registers
pub const IO_HSU_UNITCFG: c_uint = 0x01000000;
pub const IO_HSU_FIR: c_uint = 0x01000008;
pub const IO_HSU_FIR_CLR: c_uint = 0x01000010;
pub const IO_HSU_FEC: c_uint = 0x01000018;
pub const IO_HSU_ERR_ACT_MASK: c_uint = 0x01000020;
pub const IO_HSU_ERR_ATTN_MASK: c_uint = 0x01000028;
pub const IO_HSU_FIRX1_ACT_MASK: c_uint = 0x01000030;
pub const IO_HSU_FIRX0_ACT_MASK: c_uint = 0x01000038;
pub const IO_HSU_SEC_LEM_DEBUG_OVR: c_uint = 0x01000040;
pub const IO_HSU_EXTENDED_ERR_PTR: c_uint = 0x01000048;
pub const IO_HSU_COMMON_CONFIG: c_uint = 0x01000060;
// UnitID 2: Application Unit (APP)
pub const IO_APP_UNITCFG: c_uint = 0x02000000;
pub const IO_APP_FIR: c_uint = 0x02000008;
pub const IO_APP_FIR_CLR: c_uint = 0x02000010;
pub const IO_APP_FEC: c_uint = 0x02000018;
pub const IO_APP_ERR_ACT_MASK: c_uint = 0x02000020;
pub const IO_APP_ERR_ATTN_MASK: c_uint = 0x02000028;
pub const IO_APP_FIRX1_ACT_MASK: c_uint = 0x02000030;
pub const IO_APP_FIRX0_ACT_MASK: c_uint = 0x02000038;
pub const IO_APP_SEC_LEM_DEBUG_OVR: c_uint = 0x02000040;
pub const IO_APP_EXTENDED_ERR_PTR: c_uint = 0x02000048;
pub const IO_APP_COMMON_CONFIG: c_uint = 0x02000060;
pub const IO_APP_DEBUG_REG_01: c_uint = 0x02010000;
pub const IO_APP_DEBUG_REG_02: c_uint = 0x02010008;
pub const IO_APP_DEBUG_REG_03: c_uint = 0x02010010;
pub const IO_APP_DEBUG_REG_04: c_uint = 0x02010018;
pub const IO_APP_DEBUG_REG_05: c_uint = 0x02010020;
pub const IO_APP_DEBUG_REG_06: c_uint = 0x02010028;
pub const IO_APP_DEBUG_REG_07: c_uint = 0x02010030;
pub const IO_APP_DEBUG_REG_08: c_uint = 0x02010038;
pub const IO_APP_DEBUG_REG_09: c_uint = 0x02010040;
pub const IO_APP_DEBUG_REG_10: c_uint = 0x02010048;
pub const IO_APP_DEBUG_REG_11: c_uint = 0x02010050;
pub const IO_APP_DEBUG_REG_12: c_uint = 0x02010058;
pub const IO_APP_DEBUG_REG_13: c_uint = 0x02010060;
pub const IO_APP_DEBUG_REG_14: c_uint = 0x02010068;
pub const IO_APP_DEBUG_REG_15: c_uint = 0x02010070;
pub const IO_APP_DEBUG_REG_16: c_uint = 0x02010078;
pub const IO_APP_DEBUG_REG_17: c_uint = 0x02010080;
pub const IO_APP_DEBUG_REG_18: c_uint = 0x02010088;
// Read/write from/to registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_reg_io {
    pub /: *mut *mut __u64 num; / register offset/address,
    pub val64: __u64,
}

//
// All registers of our card will return values not equal this values.
// If we see IO_ILLEGAL_VALUE on any of our MMIO register reads, the
// card can be considered as unusable. It will need recovery.
//
pub const IO_ILLEGAL_VALUE: c_uint = 0xffffffffffffffffull;
//
// Generic DDCB execution interface.
//
// This interface is a first prototype resulting from discussions we
// had with other teams which wanted to use the Genwqe card. It allows
// to issue a DDCB request in a generic way. The request will block
// until it finishes or time out with error.
//
// Some DDCBs require DMA addresses to be specified in the ASIV
// block. The interface provies the capability to let the kernel
// driver know where those addresses are by specifying the ATS field,
// such that it can replace the user-space addresses with appropriate
// DMA addresses or DMA addresses of a scatter gather list which is
// dynamically created.
//
// Our hardware will refuse DDCB execution if the ATS field is not as
// expected. That means the DDCB execution engine in the chip knows
// where it expects DMA addresses within the ASIV part of the DDCB and
// will check that against the ATS field definition. Any invalid or
// unknown ATS content will lead to DDCB refusal.
//
// Genwqe chip Units
pub const DDCB_ACFUNC_SLU: c_uint = 0x00  /* chip service layer unit */;
pub const DDCB_ACFUNC_APP: c_uint = 0x01  /* chip application */;
// DDCB return codes (RETC)
pub const DDCB_RETC_IDLE: c_uint = 0x0000 /* Unexecuted/DDCB created */;
pub const DDCB_RETC_PENDING: c_uint = 0x0101 /* Pending Execution */;
pub const DDCB_RETC_COMPLETE: c_uint = 0x0102 /* Cmd complete. No error */;
pub const DDCB_RETC_FAULT: c_uint = 0x0104 /* App Err, recoverable */;
pub const DDCB_RETC_ERROR: c_uint = 0x0108 /* App Err, non-recoverable */;
pub const DDCB_RETC_FORCED_ERROR: c_uint = 0x01ff /* overwritten by driver  */;
pub const DDCB_RETC_UNEXEC: c_uint = 0x0110 /* Unexe/Removed from queue */;
pub const DDCB_RETC_TERM: c_uint = 0x0120 /* Terminated */;
pub const DDCB_RETC_RES0: c_uint = 0x0140 /* Reserved */;
pub const DDCB_RETC_RES1: c_uint = 0x0180 /* Reserved */;
// DDCB Command Options (CMDOPT)
pub const DDCB_OPT_ECHO_FORCE_NO: c_uint = 0x0000 /* ECHO DDCB */;
pub const DDCB_OPT_ECHO_FORCE_102: c_uint = 0x0001 /* force return code */;
pub const DDCB_OPT_ECHO_FORCE_104: c_uint = 0x0002;
pub const DDCB_OPT_ECHO_FORCE_108: c_uint = 0x0003;
pub const DDCB_OPT_ECHO_FORCE_110: c_uint = 0x0004 /* only on PF ! */;
pub const DDCB_OPT_ECHO_FORCE_120: c_uint = 0x0005;
pub const DDCB_OPT_ECHO_FORCE_140: c_uint = 0x0006;
pub const DDCB_OPT_ECHO_FORCE_180: c_uint = 0x0007;

// Definitions of Service Layer Commands
pub const SLCMD_ECHO_SYNC: c_uint = 0x00 /* PF/VF */;
pub const SLCMD_MOVE_FLASH: c_uint = 0x06 /* PF only */;
pub const SLCMD_MOVE_FLASH_FLAGS_MODE: c_uint = 0x03 /* bit 0 and 1 used for mode */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum genwqe_card_state {
    GENWQE_CARD_UNUSED = 0,
    GENWQE_CARD_USED = 1,
    GENWQE_CARD_FATAL_ERROR = 2,
    GENWQE_CARD_RELOAD_BITSTREAM = 3,
    GENWQE_CARD_STATE_MAX,
}

// common struct for chip image exchange
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_bitstream {
    pub /: *mut *mut __u64 data_addr; / pointer to image data,
    pub /: *mut *mut __u32 size; / size of image file,
    pub /: *mut *mut __u32 crc; / crc of this image,
    pub /: *mut *mut __u64 target_addr; / starting address in Flash,
    pub /: *mut *mut __u32 partition; / '0', '1', or 'v',
    pub /: *mut *mut __u32 uid; / 1=host/x=dram,
    pub /: *mut *mut __u64 slu_id; / informational/sim: SluID,
    pub /: *mut *mut __u64 app_id; / informational/sim: AppID,
    pub /: *mut *mut __u16 retc; / returned from processing,
    pub /: *mut *mut __u16 attn; / attention code from processing,
    pub /: *mut *mut __u32 progress; / progress code from processing,
}

// Issuing a specific DDCB command

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_debug_data {
    pub driver_version: [c_char; 64],
    pub slu_unitcfg: __u64,
    pub app_unitcfg: __u64,
    pub ddcb_before: [__u8; DDCB_LENGTH],
    pub ddcb_prev: [__u8; DDCB_LENGTH],
    pub ddcb_finished: [__u8; DDCB_LENGTH],
}

//
// Address Translation Specification (ATS) definitions
//
// Each 4 bit within the ATS 64-bit word specify the required address
// translation at the defined offset.
//
// 63 LSB
// 6666.5555.5555.5544.4444.4443.3333.3333 ... 11
// 3210.9876.5432.1098.7654.3210.9876.5432 ... 1098.7654.3210
//
// offset: 0x00 0x08 0x10 0x18 0x20 0x28 0x30 0x38 ... 0x68 0x70 0x78
// res  res  res  res  ASIV ...
// The first 4 entries in the ATS word are reserved. The following nibbles
// each describe at an 8 byte offset the format of the required data.
//
pub const ATS_TYPE_DATA: c_uint = 0x0ull /* data  */;
pub const ATS_TYPE_FLAT_RD: c_uint = 0x4ull /* flat buffer read only */;
pub const ATS_TYPE_FLAT_RDWR: c_uint = 0x5ull /* flat buffer read/write */;
pub const ATS_TYPE_SGL_RD: c_uint = 0x6ull /* sgl read only */;
pub const ATS_TYPE_SGL_RDWR: c_uint = 0x7ull /* sgl read/write */;

//
// struct genwqe_ddcb_cmd - User parameter for generic DDCB commands
//
// On the way into the kernel the driver will read the whole data
// structure. On the way out the driver will not copy the ASIV data
// back to user-space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_ddcb_cmd {
// START of data copied to/from driver
    pub /: *mut *mut __u64 next_addr; / chaining genwqe_ddcb_cmd,
    pub /: *mut *mut __u64 flags; / reserved,
    pub /: *mut *mut __u8 acfunc; / accelerators functional unit,
    pub /: *mut *mut __u8 cmd; / command to execute,
    pub /: *mut *mut __u8 asiv_length; / used parameter length,
    pub /: *mut *mut __u8 asv_length; / length of valid return values,
    pub /: *mut *mut __u16 cmdopts; / command options,
    pub /: *mut *mut __u16 retc; / return code from processing,
    pub /: *mut *mut __u16 attn; / attention code from processing,
    pub /: *mut *mut __u16 vcrc; / variant crc16,
    pub /: *mut *mut __u32 progress; / progress code from processing,
    pub /: *mut *mut __u64 deque_ts; / dequeue time stamp,
    pub /: *mut *mut __u64 cmplt_ts; / completion time stamp,
    pub /: *mut *mut __u64 disp_ts; / SW processing start,
// move to end and avoid copy-back
    pub /: *mut *mut __u64 ddata_addr; / collect debug data,
// command specific values
    pub asv: [__u8; DDCB_ASV_LENGTH],
// END of data copied from driver
    pub ats: __u64,
    pub asiv: [__u8; DDCB_ASIV_LENGTH_ATS],
}

// used for flash update to keep it backward compatible
// END of data copied to driver
pub const GENWQE_IOC_CODE: c_uint = 0xa5;
// Access functions

//
// struct genwqe_mem - Memory pinning/unpinning information
// @addr:          virtual user space address
// @size:          size of the area pin/dma-map/unmap
// direction:      0: read/1: read and write
//
// Avoid pinning and unpinning of memory pages dynamically. Instead
// the idea is to pin the whole buffer space required for DDCB
// opertionas in advance. The driver will reuse this pinning and the
// memory associated with it to setup the sglists for the DDCB
// requests without the need to allocate and free memory or map and
// unmap to get the DMA addresses.
//
// The inverse operation needs to be called after the pinning is not
// needed anymore. The pinnings else the pinnings will get removed
// after the device is closed. Note that pinnings will required
// memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_mem {
    pub addr: __u64,
    pub size: __u64,
    pub direction: __u64,
    pub flags: __u64,
}

//
// Generic synchronous DDCB execution interface.
// Synchronously execute a DDCB.
//
// Return: 0 on success or negative error code.
// -EINVAL: Invalid parameters (ASIV_LEN, ASV_LEN, illegal fixups
// no mappings found/could not create mappings
// -EFAULT: illegal addresses in fixups, purging failed
// -EBADMSG: enqueing failed, retc != DDCB_RETC_COMPLETE
//

// Service Layer functions (PF only)

