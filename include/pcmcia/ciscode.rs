//! Automatically rewritten from C Header to Rust Module
//! Source: include/pcmcia/ciscode.h
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
// ciscode.h
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// (C) 1999		David A. Hinds
//
// Manufacturer and Product ID codes
pub const MANFID_3COM: c_uint = 0x0101;
pub const PRODID_3COM_3CXEM556: c_uint = 0x0035;
pub const PRODID_3COM_3CCFEM556: c_uint = 0x0556;
pub const PRODID_3COM_3C562: c_uint = 0x0562;
pub const MANFID_ACCTON: c_uint = 0x01bf;
pub const PRODID_ACCTON_EN2226: c_uint = 0x010a;
pub const MANFID_ADAPTEC: c_uint = 0x012f;
pub const PRODID_ADAPTEC_SCSI: c_uint = 0x0001;
pub const MANFID_ATT: c_uint = 0xffff;
pub const PRODID_ATT_KIT: c_uint = 0x0100;
pub const MANFID_CONTEC: c_uint = 0xc001;
pub const MANFID_FUJITSU: c_uint = 0x0004;
pub const PRODID_FUJITSU_MBH10302: c_uint = 0x0004;
pub const PRODID_FUJITSU_MBH10304: c_uint = 0x1003;
pub const PRODID_FUJITSU_LA501: c_uint = 0x2000;
pub const MANFID_IBM: c_uint = 0x00a4;
pub const PRODID_IBM_HOME_AND_AWAY: c_uint = 0x002e;
pub const MANFID_INTEL: c_uint = 0x0089;
pub const PRODID_INTEL_DUAL_RS232: c_uint = 0x0301;
pub const PRODID_INTEL_2PLUS: c_uint = 0x8422;
pub const MANFID_KME: c_uint = 0x0032;
pub const PRODID_KME_KXLC005_A: c_uint = 0x0704;
pub const PRODID_KME_KXLC005_B: c_uint = 0x2904;
pub const MANFID_LINKSYS: c_uint = 0x0143;
pub const PRODID_LINKSYS_PCMLM28: c_uint = 0xc0ab;
pub const PRODID_LINKSYS_3400: c_uint = 0x3341;
pub const MANFID_MEGAHERTZ: c_uint = 0x0102;
pub const PRODID_MEGAHERTZ_VARIOUS: c_uint = 0x0000;
pub const PRODID_MEGAHERTZ_EM3288: c_uint = 0x0006;
pub const MANFID_MACNICA: c_uint = 0xc00b;
pub const MANFID_MOTOROLA: c_uint = 0x0109;
pub const PRODID_MOTOROLA_MARINER: c_uint = 0x0501;
pub const MANFID_NATINST: c_uint = 0x010b;
pub const PRODID_NATINST_QUAD_RS232: c_uint = 0xd180;
pub const MANFID_NEW_MEDIA: c_uint = 0x0057;
pub const MANFID_NOKIA: c_uint = 0x0124;
pub const PRODID_NOKIA_CARDPHONE: c_uint = 0x0900;
pub const MANFID_OLICOM: c_uint = 0x0121;
pub const PRODID_OLICOM_OC2231: c_uint = 0x3122;
pub const PRODID_OLICOM_OC2232: c_uint = 0x3222;
pub const MANFID_OMEGA: c_uint = 0x0137;
pub const PRODID_OMEGA_QSP_100: c_uint = 0x0025;
pub const MANFID_OSITECH: c_uint = 0x0140;
pub const PRODID_OSITECH_JACK_144: c_uint = 0x0001;
pub const PRODID_OSITECH_JACK_288: c_uint = 0x0002;
pub const PRODID_OSITECH_JACK_336: c_uint = 0x0007;
pub const PRODID_OSITECH_SEVEN: c_uint = 0x0008;
pub const MANFID_OXSEMI: c_uint = 0x0279;
pub const MANFID_PIONEER: c_uint = 0x000b;
pub const MANFID_PSION: c_uint = 0x016c;
pub const PRODID_PSION_NET100: c_uint = 0x0023;
pub const MANFID_QUATECH: c_uint = 0x0137;
pub const PRODID_QUATECH_SPP100: c_uint = 0x0003;
pub const PRODID_QUATECH_DUAL_RS232: c_uint = 0x0012;
pub const PRODID_QUATECH_DUAL_RS232_D1: c_uint = 0x0007;
pub const PRODID_QUATECH_DUAL_RS232_D2: c_uint = 0x0052;
pub const PRODID_QUATECH_DUAL_RS232_G: c_uint = 0x004d;
pub const PRODID_QUATECH_QUAD_RS232: c_uint = 0x001b;
pub const PRODID_QUATECH_DUAL_RS422: c_uint = 0x000e;
pub const PRODID_QUATECH_QUAD_RS422: c_uint = 0x0045;
pub const MANFID_SMC: c_uint = 0x0108;
pub const PRODID_SMC_ETHER: c_uint = 0x0105;
pub const MANFID_SOCKET: c_uint = 0x0104;
pub const PRODID_SOCKET_DUAL_RS232: c_uint = 0x0006;
pub const PRODID_SOCKET_EIO: c_uint = 0x000a;
pub const PRODID_SOCKET_LPE: c_uint = 0x000d;
pub const PRODID_SOCKET_LPE_CF: c_uint = 0x0075;
pub const MANFID_SUNDISK: c_uint = 0x0045;
pub const MANFID_TDK: c_uint = 0x0105;
pub const PRODID_TDK_CF010: c_uint = 0x0900;
pub const PRODID_TDK_NP9610: c_uint = 0x0d0a;
pub const PRODID_TDK_MN3200: c_uint = 0x0e0a;
pub const PRODID_TDK_GN3410: c_uint = 0x4815;
pub const MANFID_TOSHIBA: c_uint = 0x0098;
pub const MANFID_UNGERMANN: c_uint = 0x02c0;
pub const MANFID_XIRCOM: c_uint = 0x0105;
pub const MANFID_POSSIO: c_uint = 0x030c;
pub const PRODID_POSSIO_GCC: c_uint = 0x0003;
pub const MANFID_NEC: c_uint = 0x0010;
