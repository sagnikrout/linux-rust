//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/major.h
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
// This file has definitions for major device numbers.
// For the device number assignments, see Documentation/admin-guide/devices.rst.
//
pub const UNNAMED_MAJOR: c_int = 0;
pub const MEM_MAJOR: c_int = 1;
pub const RAMDISK_MAJOR: c_int = 1;
pub const FLOPPY_MAJOR: c_int = 2;
pub const PTY_MASTER_MAJOR: c_int = 2;
pub const IDE0_MAJOR: c_int = 3;

pub const PTY_SLAVE_MAJOR: c_int = 3;
pub const TTY_MAJOR: c_int = 4;
pub const TTYAUX_MAJOR: c_int = 5;
pub const LP_MAJOR: c_int = 6;
pub const VCS_MAJOR: c_int = 7;
pub const LOOP_MAJOR: c_int = 7;
pub const SCSI_DISK0_MAJOR: c_int = 8;
pub const SCSI_TAPE_MAJOR: c_int = 9;
pub const MD_MAJOR: c_int = 9;
pub const MISC_MAJOR: c_int = 10;
pub const SCSI_CDROM_MAJOR: c_int = 11;

pub const XT_DISK_MAJOR: c_int = 13;
pub const INPUT_MAJOR: c_int = 13;
pub const SOUND_MAJOR: c_int = 14;
pub const CDU31A_CDROM_MAJOR: c_int = 15;
pub const JOYSTICK_MAJOR: c_int = 15;
pub const GOLDSTAR_CDROM_MAJOR: c_int = 16;
pub const OPTICS_CDROM_MAJOR: c_int = 17;
pub const SANYO_CDROM_MAJOR: c_int = 18;
pub const MITSUMI_X_CDROM_MAJOR: c_int = 20;

pub const SCSI_GENERIC_MAJOR: c_int = 21;
pub const IDE1_MAJOR: c_int = 22;
pub const DIGICU_MAJOR: c_int = 22;
pub const DIGI_MAJOR: c_int = 23;
pub const MITSUMI_CDROM_MAJOR: c_int = 23;
pub const CDU535_CDROM_MAJOR: c_int = 24;
pub const STL_SERIALMAJOR: c_int = 24;
pub const MATSUSHITA_CDROM_MAJOR: c_int = 25;
pub const STL_CALLOUTMAJOR: c_int = 25;
pub const MATSUSHITA_CDROM2_MAJOR: c_int = 26;
pub const QIC117_TAPE_MAJOR: c_int = 27;
pub const MATSUSHITA_CDROM3_MAJOR: c_int = 27;
pub const MATSUSHITA_CDROM4_MAJOR: c_int = 28;
pub const STL_SIOMEMMAJOR: c_int = 28;
pub const ACSI_MAJOR: c_int = 28;
pub const AZTECH_CDROM_MAJOR: c_int = 29;

pub const MTD_BLOCK_MAJOR: c_int = 31;
pub const CM206_CDROM_MAJOR: c_int = 32;
pub const IDE2_MAJOR: c_int = 33;
pub const IDE3_MAJOR: c_int = 34;
pub const Z8530_MAJOR: c_int = 34;

pub const NETLINK_MAJOR: c_int = 36;
pub const PS2ESDI_MAJOR: c_int = 36;
pub const IDETAPE_MAJOR: c_int = 37;
pub const Z2RAM_MAJOR: c_int = 37;

pub const RISCOM8_NORMAL_MAJOR: c_int = 48;

pub const RISCOM8_CALLOUT_MAJOR: c_int = 49;
pub const MKISS_MAJOR: c_int = 55;

pub const IDE4_MAJOR: c_int = 56;
pub const IDE5_MAJOR: c_int = 57;
pub const SCSI_DISK1_MAJOR: c_int = 65;
pub const SCSI_DISK2_MAJOR: c_int = 66;
pub const SCSI_DISK3_MAJOR: c_int = 67;
pub const SCSI_DISK4_MAJOR: c_int = 68;
pub const SCSI_DISK5_MAJOR: c_int = 69;
pub const SCSI_DISK6_MAJOR: c_int = 70;
pub const SCSI_DISK7_MAJOR: c_int = 71;
pub const COMPAQ_SMART2_MAJOR: c_int = 72;
pub const COMPAQ_SMART2_MAJOR1: c_int = 73;
pub const COMPAQ_SMART2_MAJOR2: c_int = 74;
pub const COMPAQ_SMART2_MAJOR3: c_int = 75;
pub const COMPAQ_SMART2_MAJOR4: c_int = 76;
pub const COMPAQ_SMART2_MAJOR5: c_int = 77;
pub const COMPAQ_SMART2_MAJOR6: c_int = 78;
pub const COMPAQ_SMART2_MAJOR7: c_int = 79;
pub const SPECIALIX_NORMAL_MAJOR: c_int = 75;
pub const SPECIALIX_CALLOUT_MAJOR: c_int = 76;
pub const AURORA_MAJOR: c_int = 79;

pub const SCSI_CHANGER_MAJOR: c_int = 86;
pub const IDE6_MAJOR: c_int = 88;
pub const IDE7_MAJOR: c_int = 89;
pub const IDE8_MAJOR: c_int = 90;
pub const MTD_CHAR_MAJOR: c_int = 90;
pub const IDE9_MAJOR: c_int = 91;
pub const DASD_MAJOR: c_int = 94;
pub const MDISK_MAJOR: c_int = 95;
pub const UBD_MAJOR: c_int = 98;
pub const PP_MAJOR: c_int = 99;
pub const JSFD_MAJOR: c_int = 99;
pub const PHONE_MAJOR: c_int = 100;
pub const COMPAQ_CISS_MAJOR: c_int = 104;
pub const COMPAQ_CISS_MAJOR1: c_int = 105;
pub const COMPAQ_CISS_MAJOR2: c_int = 106;
pub const COMPAQ_CISS_MAJOR3: c_int = 107;
pub const COMPAQ_CISS_MAJOR4: c_int = 108;
pub const COMPAQ_CISS_MAJOR5: c_int = 109;
pub const COMPAQ_CISS_MAJOR6: c_int = 110;
pub const COMPAQ_CISS_MAJOR7: c_int = 111;
pub const VIODASD_MAJOR: c_int = 112;
pub const VIOCD_MAJOR: c_int = 113;
pub const ATARAID_MAJOR: c_int = 114;
pub const SCSI_DISK8_MAJOR: c_int = 128;
pub const SCSI_DISK9_MAJOR: c_int = 129;
pub const SCSI_DISK10_MAJOR: c_int = 130;
pub const SCSI_DISK11_MAJOR: c_int = 131;
pub const SCSI_DISK12_MAJOR: c_int = 132;
pub const SCSI_DISK13_MAJOR: c_int = 133;
pub const SCSI_DISK14_MAJOR: c_int = 134;
pub const SCSI_DISK15_MAJOR: c_int = 135;
pub const UNIX98_PTY_MASTER_MAJOR: c_int = 128;
pub const UNIX98_PTY_MAJOR_COUNT: c_int = 8;

pub const DRBD_MAJOR: c_int = 147;
pub const RTF_MAJOR: c_int = 150;
pub const RAW_MAJOR: c_int = 162;
pub const USB_ACM_MAJOR: c_int = 166;
pub const USB_ACM_AUX_MAJOR: c_int = 167;
pub const USB_CHAR_MAJOR: c_int = 180;
pub const MMC_BLOCK_MAJOR: c_int = 179;

pub const MSR_MAJOR: c_int = 202;
pub const CPUID_MAJOR: c_int = 203;

pub const IBM_TTY3270_MAJOR: c_int = 227;
pub const IBM_FS3270_MAJOR: c_int = 228;
pub const VIOTAPE_MAJOR: c_int = 230;
pub const BLOCK_EXT_MAJOR: c_int = 259;

