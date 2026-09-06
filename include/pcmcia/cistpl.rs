//! Automatically rewritten from C Header to Rust Module
//! Source: include/pcmcia/cistpl.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// cistpl.h
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// (C) 1999             David A. Hinds
//
pub type cisdata_t = c_uchar;
pub const CISTPL_NULL: c_uint = 0x00;
pub const CISTPL_DEVICE: c_uint = 0x01;
pub const CISTPL_LONGLINK_CB: c_uint = 0x02;
pub const CISTPL_INDIRECT: c_uint = 0x03;
pub const CISTPL_CONFIG_CB: c_uint = 0x04;
pub const CISTPL_CFTABLE_ENTRY_CB: c_uint = 0x05;
pub const CISTPL_LONGLINK_MFC: c_uint = 0x06;
pub const CISTPL_BAR: c_uint = 0x07;
pub const CISTPL_PWR_MGMNT: c_uint = 0x08;
pub const CISTPL_EXTDEVICE: c_uint = 0x09;
pub const CISTPL_CHECKSUM: c_uint = 0x10;
pub const CISTPL_LONGLINK_A: c_uint = 0x11;
pub const CISTPL_LONGLINK_C: c_uint = 0x12;
pub const CISTPL_LINKTARGET: c_uint = 0x13;
pub const CISTPL_NO_LINK: c_uint = 0x14;
pub const CISTPL_VERS_1: c_uint = 0x15;
pub const CISTPL_ALTSTR: c_uint = 0x16;
pub const CISTPL_DEVICE_A: c_uint = 0x17;
pub const CISTPL_JEDEC_C: c_uint = 0x18;
pub const CISTPL_JEDEC_A: c_uint = 0x19;
pub const CISTPL_CONFIG: c_uint = 0x1a;
pub const CISTPL_CFTABLE_ENTRY: c_uint = 0x1b;
pub const CISTPL_DEVICE_OC: c_uint = 0x1c;
pub const CISTPL_DEVICE_OA: c_uint = 0x1d;
pub const CISTPL_DEVICE_GEO: c_uint = 0x1e;
pub const CISTPL_DEVICE_GEO_A: c_uint = 0x1f;
pub const CISTPL_MANFID: c_uint = 0x20;
pub const CISTPL_FUNCID: c_uint = 0x21;
pub const CISTPL_FUNCE: c_uint = 0x22;
pub const CISTPL_SWIL: c_uint = 0x23;
pub const CISTPL_END: c_uint = 0xff;
// Layer 2 tuples
pub const CISTPL_VERS_2: c_uint = 0x40;
pub const CISTPL_FORMAT: c_uint = 0x41;
pub const CISTPL_GEOMETRY: c_uint = 0x42;
pub const CISTPL_BYTEORDER: c_uint = 0x43;
pub const CISTPL_DATE: c_uint = 0x44;
pub const CISTPL_BATTERY: c_uint = 0x45;
pub const CISTPL_FORMAT_A: c_uint = 0x47;
// Layer 3 tuples
pub const CISTPL_ORG: c_uint = 0x46;
pub const CISTPL_SPCL: c_uint = 0x90;
pub const CISTPL_MAX_FUNCTIONS: c_int = 8;
pub const CISTPL_MFC_ATTR: c_uint = 0x00;
pub const CISTPL_MFC_COMMON: c_uint = 0x01;
pub const CISTPL_MAX_ALTSTR_STRINGS: c_int = 4;
pub const CISTPL_DTYPE_NULL: c_uint = 0x00;
pub const CISTPL_DTYPE_ROM: c_uint = 0x01;
pub const CISTPL_DTYPE_OTPROM: c_uint = 0x02;
pub const CISTPL_DTYPE_EPROM: c_uint = 0x03;
pub const CISTPL_DTYPE_EEPROM: c_uint = 0x04;
pub const CISTPL_DTYPE_FLASH: c_uint = 0x05;
pub const CISTPL_DTYPE_SRAM: c_uint = 0x06;
pub const CISTPL_DTYPE_DRAM: c_uint = 0x07;
pub const CISTPL_DTYPE_FUNCSPEC: c_uint = 0x0d;
pub const CISTPL_DTYPE_EXTEND: c_uint = 0x0e;
pub const CISTPL_MAX_DEVICES: c_int = 4;
pub const CISTPL_DEVICE_MWAIT: c_uint = 0x01;
pub const CISTPL_DEVICE_3VCC: c_uint = 0x02;
pub const CISTPL_VERS_1_MAX_PROD_STRINGS: c_int = 4;
pub const CISTPL_FUNCID_MULTI: c_uint = 0x00;
pub const CISTPL_FUNCID_MEMORY: c_uint = 0x01;
pub const CISTPL_FUNCID_SERIAL: c_uint = 0x02;
pub const CISTPL_FUNCID_PARALLEL: c_uint = 0x03;
pub const CISTPL_FUNCID_FIXED: c_uint = 0x04;
pub const CISTPL_FUNCID_VIDEO: c_uint = 0x05;
pub const CISTPL_FUNCID_NETWORK: c_uint = 0x06;
pub const CISTPL_FUNCID_AIMS: c_uint = 0x07;
pub const CISTPL_FUNCID_SCSI: c_uint = 0x08;
pub const CISTPL_SYSINIT_POST: c_uint = 0x01;
pub const CISTPL_SYSINIT_ROM: c_uint = 0x02;
// ======================================================================
pub const CISTPL_FUNCE_SERIAL_IF: c_uint = 0x00;
pub const CISTPL_FUNCE_SERIAL_CAP: c_uint = 0x01;
pub const CISTPL_FUNCE_SERIAL_SERV_DATA: c_uint = 0x02;
pub const CISTPL_FUNCE_SERIAL_SERV_FAX: c_uint = 0x03;
pub const CISTPL_FUNCE_SERIAL_SERV_VOICE: c_uint = 0x04;
pub const CISTPL_FUNCE_SERIAL_CAP_DATA: c_uint = 0x05;
pub const CISTPL_FUNCE_SERIAL_CAP_FAX: c_uint = 0x06;
pub const CISTPL_FUNCE_SERIAL_CAP_VOICE: c_uint = 0x07;
pub const CISTPL_FUNCE_SERIAL_IF_DATA: c_uint = 0x08;
pub const CISTPL_FUNCE_SERIAL_IF_FAX: c_uint = 0x09;
pub const CISTPL_FUNCE_SERIAL_IF_VOICE: c_uint = 0x0a;
// UART identification
pub const CISTPL_SERIAL_UART_8250: c_uint = 0x00;
pub const CISTPL_SERIAL_UART_16450: c_uint = 0x01;
pub const CISTPL_SERIAL_UART_16550: c_uint = 0x02;
pub const CISTPL_SERIAL_UART_8251: c_uint = 0x03;
pub const CISTPL_SERIAL_UART_8530: c_uint = 0x04;
pub const CISTPL_SERIAL_UART_85230: c_uint = 0x05;
// UART capabilities
pub const CISTPL_SERIAL_UART_SPACE: c_uint = 0x01;
pub const CISTPL_SERIAL_UART_MARK: c_uint = 0x02;
pub const CISTPL_SERIAL_UART_ODD: c_uint = 0x04;
pub const CISTPL_SERIAL_UART_EVEN: c_uint = 0x08;
pub const CISTPL_SERIAL_UART_5BIT: c_uint = 0x01;
pub const CISTPL_SERIAL_UART_6BIT: c_uint = 0x02;
pub const CISTPL_SERIAL_UART_7BIT: c_uint = 0x04;
pub const CISTPL_SERIAL_UART_8BIT: c_uint = 0x08;
pub const CISTPL_SERIAL_UART_1STOP: c_uint = 0x10;
pub const CISTPL_SERIAL_UART_MSTOP: c_uint = 0x20;
pub const CISTPL_SERIAL_UART_2STOP: c_uint = 0x40;
pub const CISTPL_SERIAL_MOD_103: c_uint = 0x01;
pub const CISTPL_SERIAL_MOD_V21: c_uint = 0x02;
pub const CISTPL_SERIAL_MOD_V23: c_uint = 0x04;
pub const CISTPL_SERIAL_MOD_V22: c_uint = 0x08;
pub const CISTPL_SERIAL_MOD_212A: c_uint = 0x10;
pub const CISTPL_SERIAL_MOD_V22BIS: c_uint = 0x20;
pub const CISTPL_SERIAL_MOD_V26: c_uint = 0x40;
pub const CISTPL_SERIAL_MOD_V26BIS: c_uint = 0x80;
pub const CISTPL_SERIAL_MOD_V27BIS: c_uint = 0x01;
pub const CISTPL_SERIAL_MOD_V29: c_uint = 0x02;
pub const CISTPL_SERIAL_MOD_V32: c_uint = 0x04;
pub const CISTPL_SERIAL_MOD_V32BIS: c_uint = 0x08;
pub const CISTPL_SERIAL_MOD_V34: c_uint = 0x10;
pub const CISTPL_SERIAL_ERR_MNP2_4: c_uint = 0x01;
pub const CISTPL_SERIAL_ERR_V42_LAPM: c_uint = 0x02;
pub const CISTPL_SERIAL_CMPR_V42BIS: c_uint = 0x01;
pub const CISTPL_SERIAL_CMPR_MNP5: c_uint = 0x02;
pub const CISTPL_SERIAL_CMD_AT1: c_uint = 0x01;
pub const CISTPL_SERIAL_CMD_AT2: c_uint = 0x02;
pub const CISTPL_SERIAL_CMD_AT3: c_uint = 0x04;
pub const CISTPL_SERIAL_CMD_MNP_AT: c_uint = 0x08;
pub const CISTPL_SERIAL_CMD_V25BIS: c_uint = 0x10;
pub const CISTPL_SERIAL_CMD_V25A: c_uint = 0x20;
pub const CISTPL_SERIAL_CMD_DMCL: c_uint = 0x40;
// ======================================================================
pub const CISTPL_FUNCE_LAN_TECH: c_uint = 0x01;
pub const CISTPL_FUNCE_LAN_SPEED: c_uint = 0x02;
pub const CISTPL_FUNCE_LAN_MEDIA: c_uint = 0x03;
pub const CISTPL_FUNCE_LAN_NODE_ID: c_uint = 0x04;
pub const CISTPL_FUNCE_LAN_CONNECTOR: c_uint = 0x05;
// LAN technologies
pub const CISTPL_LAN_TECH_ARCNET: c_uint = 0x01;
pub const CISTPL_LAN_TECH_ETHERNET: c_uint = 0x02;
pub const CISTPL_LAN_TECH_TOKENRING: c_uint = 0x03;
pub const CISTPL_LAN_TECH_LOCALTALK: c_uint = 0x04;
pub const CISTPL_LAN_TECH_FDDI: c_uint = 0x05;
pub const CISTPL_LAN_TECH_ATM: c_uint = 0x06;
pub const CISTPL_LAN_TECH_WIRELESS: c_uint = 0x07;
// LAN media definitions
pub const CISTPL_LAN_MEDIA_UTP: c_uint = 0x01;
pub const CISTPL_LAN_MEDIA_STP: c_uint = 0x02;
pub const CISTPL_LAN_MEDIA_THIN_COAX: c_uint = 0x03;
pub const CISTPL_LAN_MEDIA_THICK_COAX: c_uint = 0x04;
pub const CISTPL_LAN_MEDIA_FIBER: c_uint = 0x05;
pub const CISTPL_LAN_MEDIA_900MHZ: c_uint = 0x06;
pub const CISTPL_LAN_MEDIA_2GHZ: c_uint = 0x07;
pub const CISTPL_LAN_MEDIA_5GHZ: c_uint = 0x08;
pub const CISTPL_LAN_MEDIA_DIFF_IR: c_uint = 0x09;
pub const CISTPL_LAN_MEDIA_PTP_IR: c_uint = 0x0a;
// ======================================================================
pub const CISTPL_IDE_INTERFACE: c_uint = 0x01;
// First feature byte
pub const CISTPL_IDE_SILICON: c_uint = 0x04;
pub const CISTPL_IDE_UNIQUE: c_uint = 0x08;
pub const CISTPL_IDE_DUAL: c_uint = 0x10;
// Second feature byte
pub const CISTPL_IDE_HAS_SLEEP: c_uint = 0x01;
pub const CISTPL_IDE_HAS_STANDBY: c_uint = 0x02;
pub const CISTPL_IDE_HAS_IDLE: c_uint = 0x04;
pub const CISTPL_IDE_LOW_POWER: c_uint = 0x08;
pub const CISTPL_IDE_REG_INHIBIT: c_uint = 0x10;
pub const CISTPL_IDE_HAS_INDEX: c_uint = 0x20;
pub const CISTPL_IDE_IOIS16: c_uint = 0x40;
pub const CISTPL_FUNCE_IDE_IFACE: c_uint = 0x01;
pub const CISTPL_FUNCE_IDE_MASTER: c_uint = 0x02;
pub const CISTPL_FUNCE_IDE_SLAVE: c_uint = 0x03;
// ======================================================================
pub const CISTPL_BAR_SPACE: c_uint = 0x07;
pub const CISTPL_BAR_SPACE_IO: c_uint = 0x10;
pub const CISTPL_BAR_PREFETCH: c_uint = 0x20;
pub const CISTPL_BAR_CACHEABLE: c_uint = 0x40;
pub const CISTPL_BAR_1MEG_MAP: c_uint = 0x80;
// These are bits in the 'present' field, and indices in 'param'
pub const CISTPL_POWER_VNOM: c_int = 0;
pub const CISTPL_POWER_VMIN: c_int = 1;
pub const CISTPL_POWER_VMAX: c_int = 2;
pub const CISTPL_POWER_ISTATIC: c_int = 3;
pub const CISTPL_POWER_IAVG: c_int = 4;
pub const CISTPL_POWER_IPEAK: c_int = 5;
pub const CISTPL_POWER_IDOWN: c_int = 6;
pub const CISTPL_POWER_HIGHZ_OK: c_uint = 0x01;
pub const CISTPL_POWER_HIGHZ_REQ: c_uint = 0x02;
pub const CISTPL_IO_LINES_MASK: c_uint = 0x1f;
pub const CISTPL_IO_8BIT: c_uint = 0x20;
pub const CISTPL_IO_16BIT: c_uint = 0x40;
pub const CISTPL_IO_RANGE: c_uint = 0x80;
pub const CISTPL_IO_MAX_WIN: c_int = 16;
pub const CISTPL_MEM_MAX_WIN: c_int = 8;
pub const CISTPL_CFTABLE_DEFAULT: c_uint = 0x0001;
pub const CISTPL_CFTABLE_BVDS: c_uint = 0x0002;
pub const CISTPL_CFTABLE_WP: c_uint = 0x0004;
pub const CISTPL_CFTABLE_RDYBSY: c_uint = 0x0008;
pub const CISTPL_CFTABLE_MWAIT: c_uint = 0x0010;
pub const CISTPL_CFTABLE_AUDIO: c_uint = 0x0800;
pub const CISTPL_CFTABLE_READONLY: c_uint = 0x1000;
pub const CISTPL_CFTABLE_PWRDOWN: c_uint = 0x2000;
pub const CISTPL_CFTABLE_MASTER: c_uint = 0x000100;
pub const CISTPL_CFTABLE_INVALIDATE: c_uint = 0x000200;
pub const CISTPL_CFTABLE_VGA_PALETTE: c_uint = 0x000400;
pub const CISTPL_CFTABLE_PARITY: c_uint = 0x000800;
pub const CISTPL_CFTABLE_WAIT: c_uint = 0x001000;
pub const CISTPL_CFTABLE_SERR: c_uint = 0x002000;
pub const CISTPL_CFTABLE_FAST_BACK: c_uint = 0x004000;
pub const CISTPL_CFTABLE_BINARY_AUDIO: c_uint = 0x010000;
pub const CISTPL_CFTABLE_PWM_AUDIO: c_uint = 0x020000;
pub const CISTPL_ORG_FS: c_uint = 0x00;
pub const CISTPL_ORG_APPSPEC: c_uint = 0x01;
pub const CISTPL_ORG_XIP: c_uint = 0x02;
pub const CISTPL_FORMAT_DISK: c_uint = 0x00;
pub const CISTPL_FORMAT_MEM: c_uint = 0x01;
pub const CISTPL_EDC_NONE: c_uint = 0x00;
pub const CISTPL_EDC_CKSUM: c_uint = 0x01;
pub const CISTPL_EDC_CRC: c_uint = 0x02;
pub const CISTPL_EDC_PCC: c_uint = 0x03;
// Special cisdata_t value
pub const RETURN_FIRST_TUPLE: c_uint = 0xff;
// Attributes for tuple calls
pub const TUPLE_RETURN_LINK: c_uint = 0x01;
pub const TUPLE_RETURN_COMMON: c_uint = 0x02;
pub const CISTPL_MAX_CIS_SIZE: c_uint = 0x200;
