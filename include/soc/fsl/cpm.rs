//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/cpm.h
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
// SPI Parameter RAM common to QE and CPM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_pram {
    pub /: *mut *mut __be16 rbase; / Rx Buffer descriptor base address,
    pub /: *mut *mut __be16 tbase; / Tx Buffer descriptor base address,
    pub /: *mut *mut u8 rfcr; / Rx function code,
    pub /: *mut *mut u8 tfcr; / Tx function code,
    pub /: *mut *mut __be16 mrblr; / Max receive buffer length,
    pub /: *mut *mut __be32 rstate; / Internal,
    pub /: *mut *mut __be32 rdp; / Internal,
    pub /: *mut *mut __be16 rbptr; / Internal,
    pub /: *mut *mut __be16 rbc; / Internal,
    pub /: *mut *mut __be32 rxtmp; / Internal,
    pub /: *mut *mut __be32 tstate; / Internal,
    pub /: *mut *mut __be32 tdp; / Internal,
    pub /: *mut *mut __be16 tbptr; / Internal,
    pub /: *mut *mut __be16 tbc; / Internal,
    pub /: *mut *mut __be32 txtmp; / Internal,
    pub /: *mut *mut __be32 res; / Tx temp.,
    pub /: *mut *mut __be16 rpbase; / Relocation pointer (CPM1 only),
    pub /: *mut *mut __be16 res1; / Reserved,
}

//
// USB Controller pram common to QE and CPM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ctlr {
    pub usb_usmod: u8,
    pub usb_usadr: u8,
    pub usb_uscom: u8,
    pub res1: [u8; 1],
    pub usb_usep: [__be16; 4],
    pub res2: [u8; 4],
    pub usb_usber: __be16,
    pub res3: [u8; 2],
    pub usb_usbmr: __be16,
    pub res4: [u8; 1],
    pub usb_usbs: u8,
// Fields down below are QE-only
    pub usb_ussft: __be16,
    pub res5: [u8; 2],
    pub usb_usfrn: __be16,
    pub res6: [u8; 0x22],
// C attribute field omitted
//
// Function code bits, usually generic to devices.
//

// Opcodes common to CPM1 and CPM2
//

// Buffer descriptors used by many of the CPM protocols.
    pub /: *mut *mut ushort cbd_sc; / Status and Control,
    pub /: *mut *mut ushort cbd_datlen; / Data length in buffer,
    pub /: *mut *mut uint cbd_bufaddr; / Buffer address in host memory,
    pub cbd_t: },
// Buffer descriptor control/status used by serial
//

// Buffer descriptor control/status used by Ethernet receive.
// Common to SCC and FCC.
//

// Buffer descriptor control/status used by Ethernet transmit.
// Common to SCC and FCC.
//

// Buffer descriptor control/status used by Transparent mode SCC.
//

// Buffer descriptor control/status used by I2C.
//

    pub opcode): int cpm_command(u32 command, u8,

    pub -ENOSYS: return,

    pub dev): *mut int cpm2_gpiochip_add32(struct device,
