//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hydra.h
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
// include/asm-ppc/hydra.h -- Mac I/O `Hydra' definitions
//
// Copyright (C) 1997 Geert Uytterhoeven
//
// This file is based on the following documentation:
//
// Macintosh Technology in the Common Hardware Reference Platform
// Apple Computer, Inc.
//
// © Copyright 1995 Apple Computer, Inc. All rights reserved.
//
// It's available online from https://www.cpu.lu/~mlan/ftp/MacTech.pdf
// You can obtain paper copies of this book from computer bookstores or by
// writing Morgan Kaufmann Publishers, Inc., 340 Pine Street, Sixth Floor, San
// Francisco, CA 94104. Reference ISBN 1-55860-393-X.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Hydra {
// DBDMA Controller Register Space
    pub Pad1: [c_char; 0x30],
    pub CachePD: u_int,
    pub IDs: u_int,
    pub Feature_Control: u_int,
    pub Pad2: [c_char; 0x7fc4],
// DBDMA Channel Register Space
    pub SCSI_DMA: [c_char; 0x100],
    pub Pad3: [c_char; 0x300],
    pub SCCA_Tx_DMA: [c_char; 0x100],
    pub SCCA_Rx_DMA: [c_char; 0x100],
    pub SCCB_Tx_DMA: [c_char; 0x100],
    pub SCCB_Rx_DMA: [c_char; 0x100],
    pub Pad4: [c_char; 0x7800],
// Device Register Space
    pub SCSI: [c_char; 0x1000],
    pub ADB: [c_char; 0x1000],
    pub SCC_Legacy: [c_char; 0x1000],
    pub SCC: [c_char; 0x1000],
    pub Pad9: [c_char; 0x2000],
    pub VIA: [c_char; 0x2000],
    pub Pad10: [c_char; 0x28000],
    pub OpenPIC: [c_char; 0x40000],
}

//
// Feature Control Register
//
pub const HYDRA_FC_SCC_CELL_EN: c_uint = 0x00000001	/* Enable SCC Clock */;
pub const HYDRA_FC_SCSI_CELL_EN: c_uint = 0x00000002	/* Enable SCSI Clock */;
pub const HYDRA_FC_SCCA_ENABLE: c_uint = 0x00000004	/* Enable SCC A Lines */;
pub const HYDRA_FC_SCCB_ENABLE: c_uint = 0x00000008	/* Enable SCC B Lines */;
pub const HYDRA_FC_ARB_BYPASS: c_uint = 0x00000010	/* Bypass Internal Arbiter */;
pub const HYDRA_FC_RESET_SCC: c_uint = 0x00000020	/* Reset SCC */;
pub const HYDRA_FC_MPIC_ENABLE: c_uint = 0x00000040	/* Enable OpenPIC */;
pub const HYDRA_FC_SLOW_SCC_PCLK: c_uint = 0x00000080	/* 1=15.6672, 0=25 MHz */;
pub const HYDRA_FC_MPIC_IS_MASTER: c_uint = 0x00000100	/* OpenPIC Master Mode */;
//
// OpenPIC Interrupt Sources
//
pub const HYDRA_INT_SIO: c_int = 0;
pub const HYDRA_INT_SCSI_DMA: c_int = 1;
pub const HYDRA_INT_SCCA_TX_DMA: c_int = 2;
pub const HYDRA_INT_SCCA_RX_DMA: c_int = 3;
pub const HYDRA_INT_SCCB_TX_DMA: c_int = 4;
pub const HYDRA_INT_SCCB_RX_DMA: c_int = 5;
pub const HYDRA_INT_SCSI: c_int = 6;
pub const HYDRA_INT_SCCA: c_int = 7;
pub const HYDRA_INT_SCCB: c_int = 8;
pub const HYDRA_INT_VIA: c_int = 9;
pub const HYDRA_INT_ADB: c_int = 10;
pub const HYDRA_INT_ADB_NMI: c_int = 11;

pub const HYDRA_INT_SPARE: c_int = 19;

