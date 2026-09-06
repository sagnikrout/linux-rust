//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ioat/hw.h
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
// Copyright(c) 2004 - 2009 Intel Corporation. All rights reserved.
//
// PCI Configuration Space Values
pub const IOAT_MMIO_BAR: c_int = 0;
// CB device ID's
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB0: c_uint = 0x0e20;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB1: c_uint = 0x0e21;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB2: c_uint = 0x0e22;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB3: c_uint = 0x0e23;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB4: c_uint = 0x0e24;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB5: c_uint = 0x0e25;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB6: c_uint = 0x0e26;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB7: c_uint = 0x0e27;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB8: c_uint = 0x0e2e;
pub const PCI_DEVICE_ID_INTEL_IOAT_IVB9: c_uint = 0x0e2f;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW0: c_uint = 0x2f20;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW1: c_uint = 0x2f21;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW2: c_uint = 0x2f22;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW3: c_uint = 0x2f23;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW4: c_uint = 0x2f24;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW5: c_uint = 0x2f25;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW6: c_uint = 0x2f26;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW7: c_uint = 0x2f27;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW8: c_uint = 0x2f2e;
pub const PCI_DEVICE_ID_INTEL_IOAT_HSW9: c_uint = 0x2f2f;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD0: c_uint = 0x0C50;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD1: c_uint = 0x0C51;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD2: c_uint = 0x0C52;
pub const PCI_DEVICE_ID_INTEL_IOAT_BWD3: c_uint = 0x0C53;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE0: c_uint = 0x6f50;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE1: c_uint = 0x6f51;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE2: c_uint = 0x6f52;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDXDE3: c_uint = 0x6f53;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX0: c_uint = 0x6f20;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX1: c_uint = 0x6f21;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX2: c_uint = 0x6f22;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX3: c_uint = 0x6f23;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX4: c_uint = 0x6f24;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX5: c_uint = 0x6f25;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX6: c_uint = 0x6f26;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX7: c_uint = 0x6f27;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX8: c_uint = 0x6f2e;
pub const PCI_DEVICE_ID_INTEL_IOAT_BDX9: c_uint = 0x6f2f;
pub const PCI_DEVICE_ID_INTEL_IOAT_SKX: c_uint = 0x2021;
pub const PCI_DEVICE_ID_INTEL_IOAT_ICX: c_uint = 0x0b00;
pub const IOAT_VER_1_2: c_uint = 0x12    /* Version 1.2 */;
pub const IOAT_VER_2_0: c_uint = 0x20    /* Version 2.0 */;
pub const IOAT_VER_3_0: c_uint = 0x30    /* Version 3.0 */;
pub const IOAT_VER_3_2: c_uint = 0x32    /* Version 3.2 */;
pub const IOAT_VER_3_3: c_uint = 0x33    /* Version 3.3 */;
pub const IOAT_VER_3_4: c_uint = 0x34	/* Version 3.4 */;
pub const IOAT_DESC_SZ: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_dma_descriptor {
    pub size: u32,
    pub ctl: u32,
    pub int_en:1: c_uint,
    pub src_snoop_dis:1: c_uint,
    pub dest_snoop_dis:1: c_uint,
    pub compl_write:1: c_uint,
    pub fence:1: c_uint,
    pub null:1: c_uint,
    pub src_brk:1: c_uint,
    pub dest_brk:1: c_uint,
    pub bundle:1: c_uint,
    pub dest_dca:1: c_uint,
    pub hint:1: c_uint,
    pub rsvd2:13: c_uint,
pub const IOAT_OP_COPY: c_uint = 0x00;
    pub op:8: c_uint,
    pub ctl_f: },
}

// store some driver data in an unused portion of the descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_xor_descriptor {
    pub size: u32,
    pub ctl: u32,
    pub int_en:1: c_uint,
    pub src_snoop_dis:1: c_uint,
    pub dest_snoop_dis:1: c_uint,
    pub compl_write:1: c_uint,
    pub fence:1: c_uint,
    pub src_cnt:3: c_uint,
    pub bundle:1: c_uint,
    pub dest_dca:1: c_uint,
    pub hint:1: c_uint,
    pub rsvd:13: c_uint,
pub const IOAT_OP_XOR: c_uint = 0x87;
pub const IOAT_OP_XOR_VAL: c_uint = 0x88;
    pub op:8: c_uint,
    pub ctl_f: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_xor_ext_descriptor {
    pub src_addr6: u64,
    pub src_addr7: u64,
    pub src_addr8: u64,
    pub next: u64,
    pub rsvd: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_pq_descriptor {
    pub size: u32,
    pub dwbes: u32,
    pub rsvd:25: c_uint,
    pub p_val_err:1: c_uint,
    pub q_val_err:1: c_uint,
    pub rsvd1:4: c_uint,
    pub wbes:1: c_uint,
    pub dwbes_f: },
}

pub const IOAT_OP_PQ: c_uint = 0x89;
pub const IOAT_OP_PQ_VAL: c_uint = 0x8a;
pub const IOAT_OP_PQ_16S: c_uint = 0xa0;
pub const IOAT_OP_PQ_VAL_16S: c_uint = 0xa1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_pq_ext_descriptor {
    pub src_addr4: u64,
    pub src_addr5: u64,
    pub src_addr6: u64,
    pub next: u64,
    pub src_addr7: u64,
    pub src_addr8: u64,
    pub rsvd: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_pq_update_descriptor {
    pub size: u32,
    pub ctl: u32,
    pub int_en:1: c_uint,
    pub src_snoop_dis:1: c_uint,
    pub dest_snoop_dis:1: c_uint,
    pub compl_write:1: c_uint,
    pub fence:1: c_uint,
    pub src_cnt:3: c_uint,
    pub bundle:1: c_uint,
    pub dest_dca:1: c_uint,
    pub hint:1: c_uint,
    pub p_disable:1: c_uint,
    pub q_disable:1: c_uint,
    pub rsvd:3: c_uint,
    pub coef:8: c_uint,
pub const IOAT_OP_PQ_UP: c_uint = 0x8b;
    pub op:8: c_uint,
    pub ctl_f: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_raw_descriptor {
    pub field: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_pq16a_descriptor {
    pub coef: [u8; 8],
    pub src_addr3: u64,
    pub src_addr4: u64,
    pub src_addr5: u64,
    pub src_addr6: u64,
    pub src_addr7: u64,
    pub src_addr8: u64,
    pub src_addr9: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_pq16b_descriptor {
    pub src_addr10: u64,
    pub src_addr11: u64,
    pub src_addr12: u64,
    pub src_addr13: u64,
    pub src_addr14: u64,
    pub src_addr15: u64,
    pub src_addr16: u64,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ioat_sed_pq_descriptor {
    pub a: ioat_pq16a_descriptor,
    pub b: ioat_pq16b_descriptor,
}

pub const SED_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_sed_raw_descriptor {
    pub a: [u64; 8],
    pub b: [u64; 8],
    pub c: [u64; 8],
}
