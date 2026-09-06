//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/rzg2l-cru/rzg2l-cru-regs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// rzg2l-cru-regs.h--RZ/G2L (and alike SoCs) CRU Registers Definitions
//
// Copyright (C) 2024 Renesas Electronics Corp.
//
// HW CRU Registers Definition

// Memory Bank Base Address (Lower) Register for CRU Image Data

// Memory Bank Base Address (Higher) Register for CRU Image Data

pub const AMnMBS_MBSTS: c_uint = 0x7;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rzg2l_cru_common_regs {
    CRUnCTRL,	/* CRU Control */
    CRUnIE,		/* CRU Interrupt Enable */
    CRUnIE2,	/* CRU Interrupt Enable(2) */
    CRUnINTS,	/* CRU Interrupt Status */
    CRUnINTS2,	/* CRU Interrupt Status(2) */
    CRUnRST,	/* CRU Reset */
    AMnMB1ADDRL,	/* Bank 1 Address (Lower) for CRU Image Data */
    AMnMB1ADDRH,	/* Bank 1 Address (Higher) for CRU Image Data */
    AMnMB2ADDRL,    /* Bank 2 Address (Lower) for CRU Image Data */
    AMnMB2ADDRH,    /* Bank 2 Address (Higher) for CRU Image Data */
    AMnMB3ADDRL,    /* Bank 3 Address (Lower) for CRU Image Data */
    AMnMB3ADDRH,    /* Bank 3 Address (Higher) for CRU Image Data */
    AMnMB4ADDRL,    /* Bank 4 Address (Lower) for CRU Image Data */
    AMnMB4ADDRH,    /* Bank 4 Address (Higher) for CRU Image Data */
    AMnMB5ADDRL,    /* Bank 5 Address (Lower) for CRU Image Data */
    AMnMB5ADDRH,    /* Bank 5 Address (Higher) for CRU Image Data */
    AMnMB6ADDRL,    /* Bank 6 Address (Lower) for CRU Image Data */
    AMnMB6ADDRH,    /* Bank 6 Address (Higher) for CRU Image Data */
    AMnMB7ADDRL,    /* Bank 7 Address (Lower) for CRU Image Data */
    AMnMB7ADDRH,    /* Bank 7 Address (Higher) for CRU Image Data */
    AMnMB8ADDRL,    /* Bank 8 Address (Lower) for CRU Image Data */
    AMnMB8ADDRH,    /* Bank 8 Address (Higher) for CRU Image Data */
    AMnMBVALID,	/* Memory Bank Enable for CRU Image Data */
    AMnMBS,		/* Memory Bank Status for CRU Image Data */
    AMnMADRSL,	/* VD Memory Address Lower Status Register */
    AMnMADRSH,	/* VD Memory Address Higher Status Register */
    AMnAXIATTR,	/* AXI Master Transfer Setting Register for CRU Image Data */
    AMnFIFOPNTR,	/* AXI Master FIFO Pointer for CRU Image Data */
    AMnAXISTP,	/* AXI Master Transfer Stop for CRU Image Data */
    AMnAXISTPACK,	/* AXI Master Transfer Stop Status for CRU Image Data */
    AMnIS,		/* Image Stride Setting Register */
    ICnEN,		/* CRU Image Processing Enable */
    ICnSVCNUM,	/* CRU SVC Number Register */
    ICnSVC,		/* CRU VC Select Register */
    ICnMC,		/* CRU Image Processing Main Control */
    ICnIPMC_C0,	/* CRU Image Converter Main Control 0 */
    ICnMS,		/* CRU Module Status */
    ICnDMR,		/* CRU Data Output Mode */
    RZG2L_CRU_MAX_REG,
}
