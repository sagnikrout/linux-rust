//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mpspec_def.h
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
// Structure definitions for SMP machines following the
// Intel Multiprocessing Specification 1.1 and 1.4.
//
// This tag identifies where the SMP configuration
// information is.
//

// Intel MP Floating Pointer Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpf_intel {
    pub /: *mut *mut char signature[4]; / "_MP_",
    pub /: *mut *mut unsigned int physptr; / Configuration table address,
    pub /: *mut *mut unsigned char length; / Our length (paragraphs),
    pub /: *mut *mut unsigned char specification; / Specification version,
    pub /: *mut *mut unsigned char checksum; / Checksum (makes sum 0),
    pub /: *mut *mut unsigned char feature1; / Standard or configuration ?,
    pub /: *mut *mut unsigned char feature2; / Bit7 set for IMCR|PIC,
    pub /: *mut *mut unsigned char feature3; / Unused (0),
    pub /: *mut *mut unsigned char feature4; / Unused (0),
    pub /: *mut *mut unsigned char feature5; / Unused (0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_table {
    pub signature: [c_char; 4],
    pub /: *mut *mut unsigned short length; / Size of table,
    pub /: *mut *mut char spec; / 0x01,
    pub checksum: c_char,
    pub oem: [c_char; 8],
    pub productid: [c_char; 12],
    pub /: *mut *mut unsigned int oemptr; / 0 if not present,
    pub /: *mut *mut unsigned short oemsize; / 0 if not present,
    pub oemcount: c_ushort,
    pub /: *mut *mut unsigned int lapic; / APIC address,
    pub reserved: c_uint,
}

// Followed by entries
pub const MP_PROCESSOR: c_int = 0;
pub const MP_BUS: c_int = 1;
pub const MP_IOAPIC: c_int = 2;
pub const MP_INTSRC: c_int = 3;
pub const MP_LINTSRC: c_int = 4;
// Used by IBM NUMA-Q to describe node locality
pub const MP_TRANSLATION: c_int = 192;

pub const CPU_STEPPING_MASK: c_uint = 0x000F;
pub const CPU_MODEL_MASK: c_uint = 0x00F0;
pub const CPU_FAMILY_MASK: c_uint = 0x0F00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_cpu {
    pub type: c_uchar,
    pub /: *mut *mut unsigned char apicid; / Local APIC number,
    pub /: *mut *mut unsigned char apicver; / Its versions,
    pub cpuflag: c_uchar,
    pub cpufeature: c_uint,
    pub /: *mut *mut unsigned int featureflag; / CPUID feature value,
    pub reserved: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_bus {
    pub type: c_uchar,
    pub busid: c_uchar,
    pub bustype: [c_uchar; 6],
}

// List of Bus Type string values, Intel MP Spec.

pub const MPC_APIC_USABLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_ioapic {
    pub type: c_uchar,
    pub apicid: c_uchar,
    pub apicver: c_uchar,
    pub flags: c_uchar,
    pub apicaddr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_intsrc {
    pub type: c_uchar,
    pub irqtype: c_uchar,
    pub irqflag: c_ushort,
    pub srcbus: c_uchar,
    pub srcbusirq: c_uchar,
    pub dstapic: c_uchar,
    pub dstirq: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mp_irq_source_types {
    mp_INT = 0,
    mp_NMI = 1,
    mp_SMI = 2,
    mp_ExtINT = 3
}

pub const MP_IRQPOL_DEFAULT: c_uint = 0x0;
pub const MP_IRQPOL_ACTIVE_HIGH: c_uint = 0x1;
pub const MP_IRQPOL_RESERVED: c_uint = 0x2;
pub const MP_IRQPOL_ACTIVE_LOW: c_uint = 0x3;
pub const MP_IRQPOL_MASK: c_uint = 0x3;
pub const MP_IRQTRIG_DEFAULT: c_uint = 0x0;
pub const MP_IRQTRIG_EDGE: c_uint = 0x4;
pub const MP_IRQTRIG_RESERVED: c_uint = 0x8;
pub const MP_IRQTRIG_LEVEL: c_uint = 0xc;
pub const MP_IRQTRIG_MASK: c_uint = 0xc;
pub const MP_APIC_ALL: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_lintsrc {
    pub type: c_uchar,
    pub irqtype: c_uchar,
    pub irqflag: c_ushort,
    pub srcbusid: c_uchar,
    pub srcbusirq: c_uchar,
    pub destapic: c_uchar,
    pub destapiclint: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_oemtable {
    pub signature: [c_char; 4],
    pub /: *mut *mut unsigned short length; / Size of table,
    pub /: *mut *mut char rev; / 0x01,
    pub checksum: c_char,
    pub mpc: [c_char; 8],
}

//
// Default configurations
//
// 1	2 CPU ISA 82489DX
// 2	2 CPU EISA 82489DX neither IRQ 0 timer nor IRQ 13 DMA chaining
// 3	2 CPU EISA 82489DX
// 4	2 CPU MCA 82489DX
// 5	2 CPU ISA+PCI
// 6	2 CPU EISA+PCI
// 7	2 CPU MCA+PCI
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mp_bustype {
    MP_BUS_ISA = 1,
    MP_BUS_EISA,
    MP_BUS_PCI,
}
