//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/trident.h
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

pub const TRIDENTFB_DEBUG: c_int = 0;

// PCI IDS of supported cards temporarily here
pub const CYBER9320: c_uint = 0x9320;
pub const CYBER9388: c_uint = 0x9388;
pub const CYBER9382: c_uint = 0x9382		/* the real PCI id for this is 9660 */;
pub const CYBER9385: c_uint = 0x9385		/* ditto */;
pub const CYBER9397: c_uint = 0x9397;
pub const CYBER9397DVD: c_uint = 0x939A;
pub const CYBER9520: c_uint = 0x9520;
pub const CYBER9525DVD: c_uint = 0x9525;
pub const TGUI9440: c_uint = 0x9440;
pub const TGUI9660: c_uint = 0x9660;
pub const PROVIDIA9685: c_uint = 0x9685;
pub const IMAGE975: c_uint = 0x9750;
pub const IMAGE985: c_uint = 0x9850;
pub const BLADE3D: c_uint = 0x9880;
pub const CYBERBLADEE4: c_uint = 0x9540;
pub const CYBERBLADEi7: c_uint = 0x8400;
pub const CYBERBLADEi7D: c_uint = 0x8420;
pub const CYBERBLADEi1: c_uint = 0x8500;
pub const CYBERBLADEi1D: c_uint = 0x8520;
pub const CYBERBLADEAi1: c_uint = 0x8600;
pub const CYBERBLADEAi1D: c_uint = 0x8620;
pub const CYBERBLADEXPAi1: c_uint = 0x8820;
pub const CYBERBLADEXPm8: c_uint = 0x9910;
pub const CYBERBLADEXPm16: c_uint = 0x9930;
// these defines are for 'lcd' variable
pub const LCD_STRETCH: c_int = 0;
pub const LCD_CENTER: c_int = 1;
pub const LCD_BIOS: c_int = 2;
// General Registers
pub const SPR: c_uint = 0x1F		/* Software Programming Register (videoram) */;
// 3C4
pub const RevisionID: c_uint = 0x09;
pub const OldOrNew: c_uint = 0x0B;
pub const ConfPort1: c_uint = 0x0C;
pub const ConfPort2: c_uint = 0x0C;
pub const NewMode2: c_uint = 0x0D;
pub const NewMode1: c_uint = 0x0E;
pub const Protection: c_uint = 0x11;
pub const MCLKLow: c_uint = 0x16;
pub const MCLKHigh: c_uint = 0x17;
pub const ClockLow: c_uint = 0x18;
pub const ClockHigh: c_uint = 0x19;
pub const SSetup: c_uint = 0x20;
pub const SKey: c_uint = 0x37;
pub const SPKey: c_uint = 0x57;
// 3x4
pub const CRTCModuleTest: c_uint = 0x1E;
pub const FIFOControl: c_uint = 0x20;
pub const LinearAddReg: c_uint = 0x21;
pub const DRAMTiming: c_uint = 0x23;
pub const New32: c_uint = 0x23;
pub const RAMDACTiming: c_uint = 0x25;
pub const CRTHiOrd: c_uint = 0x27;
pub const AddColReg: c_uint = 0x29;
pub const InterfaceSel: c_uint = 0x2A;
pub const HorizOverflow: c_uint = 0x2B;
pub const GETest: c_uint = 0x2D;
pub const Performance: c_uint = 0x2F;
pub const GraphEngReg: c_uint = 0x36;
pub const I2C: c_uint = 0x37;
pub const PixelBusReg: c_uint = 0x38;
pub const PCIReg: c_uint = 0x39;
pub const DRAMControl: c_uint = 0x3A;
pub const MiscContReg: c_uint = 0x3C;
pub const CursorXLow: c_uint = 0x40;
pub const CursorXHigh: c_uint = 0x41;
pub const CursorYLow: c_uint = 0x42;
pub const CursorYHigh: c_uint = 0x43;
pub const CursorLocLow: c_uint = 0x44;
pub const CursorLocHigh: c_uint = 0x45;
pub const CursorXOffset: c_uint = 0x46;
pub const CursorYOffset: c_uint = 0x47;
pub const CursorFG1: c_uint = 0x48;
pub const CursorFG2: c_uint = 0x49;
pub const CursorFG3: c_uint = 0x4A;
pub const CursorFG4: c_uint = 0x4B;
pub const CursorBG1: c_uint = 0x4C;
pub const CursorBG2: c_uint = 0x4D;
pub const CursorBG3: c_uint = 0x4E;
pub const CursorBG4: c_uint = 0x4F;
pub const CursorControl: c_uint = 0x50;
pub const PCIRetry: c_uint = 0x55;
pub const PreEndControl: c_uint = 0x56;
pub const PreEndFetch: c_uint = 0x57;
pub const PCIMaster: c_uint = 0x60;
pub const Enhancement0: c_uint = 0x62;
pub const NewEDO: c_uint = 0x64;
pub const TVinterface: c_uint = 0xC0;
pub const TVMode: c_uint = 0xC1;
pub const ClockControl: c_uint = 0xCF;
// 3CE
pub const MiscExtFunc: c_uint = 0x0F;
pub const PowerStatus: c_uint = 0x23;
pub const MiscIntContReg: c_uint = 0x2F;
pub const CyberControl: c_uint = 0x30;
pub const CyberEnhance: c_uint = 0x31;
pub const FPConfig: c_uint = 0x33;
pub const VertStretch: c_uint = 0x52;
pub const HorStretch: c_uint = 0x53;
pub const BiosMode: c_uint = 0x5c;
pub const BiosReg: c_uint = 0x5d;
// Graphics Engine
pub const STATUS: c_uint = 0x2120;
pub const OLDCMD: c_uint = 0x2124;
pub const DRAWFL: c_uint = 0x2128;
pub const OLDCLR: c_uint = 0x212C;
pub const OLDDST: c_uint = 0x2138;
pub const OLDSRC: c_uint = 0x213C;
pub const OLDDIM: c_uint = 0x2140;
pub const CMD: c_uint = 0x2144;
pub const ROP: c_uint = 0x2148;
pub const COLOR: c_uint = 0x2160;
pub const BGCOLOR: c_uint = 0x2164;
pub const SRC1: c_uint = 0x2100;
pub const SRC2: c_uint = 0x2104;
pub const DST1: c_uint = 0x2108;
pub const DST2: c_uint = 0x210C;
pub const ROP_S: c_uint = 0xCC;
pub const ROP_P: c_uint = 0xF0;
pub const ROP_X: c_uint = 0x66;
