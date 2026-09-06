//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_dma.h
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
// mpc8610-pcm.h - ALSA PCM interface for the Freescale MPC8610 SoC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccsr_dma {
    pub res0: [u8; 0x100],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccsr_dma_channel {
    pub /: *mut *mut __be32 mr; / Mode register,
    pub /: *mut *mut __be32 sr; / Status register,
    pub /: *mut *mut __be32 eclndar; / Current link descriptor extended addr reg,
    pub /: *mut *mut __be32 clndar; / Current link descriptor address register,
    pub /: *mut *mut __be32 satr; / Source attributes register,
    pub /: *mut *mut __be32 sar; / Source address register,
    pub /: *mut *mut __be32 datr; / Destination attributes register,
    pub /: *mut *mut __be32 dar; / Destination address register,
    pub /: *mut *mut __be32 bcr; / Byte count register,
    pub /: *mut *mut __be32 enlndar; / Next link descriptor extended address reg,
    pub /: *mut *mut __be32 nlndar; / Next link descriptor address register,
    pub res1: [u8; 4],
    pub /: *mut *mut __be32 eclsdar; / Current list descriptor extended addr reg,
    pub /: *mut *mut __be32 clsdar; / Current list descriptor address register,
    pub /: *mut *mut __be32 enlsdar; / Next list descriptor extended address reg,
    pub /: *mut *mut __be32 nlsdar; / Next list descriptor address register,
    pub /: *mut *mut __be32 ssr; / Source stride register,
    pub /: *mut *mut __be32 dsr; / Destination stride register,
    pub res2: [u8; 0x38],
    pub channel: [}; 4],
    pub dgsr: __be32,
}

pub const CCSR_DMA_MR_BWC_DISABLED: c_uint = 0x0F000000;
pub const CCSR_DMA_MR_BWC_SHIFT: c_int = 24;
pub const CCSR_DMA_MR_BWC_MASK: c_uint = 0x0F000000;

pub const CCSR_DMA_MR_EMP_EN: c_uint = 0x00200000;
pub const CCSR_DMA_MR_EMS_EN: c_uint = 0x00040000;
pub const CCSR_DMA_MR_DAHTS_MASK: c_uint = 0x00030000;
pub const CCSR_DMA_MR_DAHTS_1: c_uint = 0x00000000;
pub const CCSR_DMA_MR_DAHTS_2: c_uint = 0x00010000;
pub const CCSR_DMA_MR_DAHTS_4: c_uint = 0x00020000;
pub const CCSR_DMA_MR_DAHTS_8: c_uint = 0x00030000;
pub const CCSR_DMA_MR_SAHTS_MASK: c_uint = 0x0000C000;
pub const CCSR_DMA_MR_SAHTS_1: c_uint = 0x00000000;
pub const CCSR_DMA_MR_SAHTS_2: c_uint = 0x00004000;
pub const CCSR_DMA_MR_SAHTS_4: c_uint = 0x00008000;
pub const CCSR_DMA_MR_SAHTS_8: c_uint = 0x0000C000;
pub const CCSR_DMA_MR_DAHE: c_uint = 0x00002000;
pub const CCSR_DMA_MR_SAHE: c_uint = 0x00001000;
pub const CCSR_DMA_MR_SRW: c_uint = 0x00000400;
pub const CCSR_DMA_MR_EOSIE: c_uint = 0x00000200;
pub const CCSR_DMA_MR_EOLNIE: c_uint = 0x00000100;
pub const CCSR_DMA_MR_EOLSIE: c_uint = 0x00000080;
pub const CCSR_DMA_MR_EIE: c_uint = 0x00000040;
pub const CCSR_DMA_MR_XFE: c_uint = 0x00000020;
pub const CCSR_DMA_MR_CDSM_SWSM: c_uint = 0x00000010;
pub const CCSR_DMA_MR_CA: c_uint = 0x00000008;
pub const CCSR_DMA_MR_CTM: c_uint = 0x00000004;
pub const CCSR_DMA_MR_CC: c_uint = 0x00000002;
pub const CCSR_DMA_MR_CS: c_uint = 0x00000001;
pub const CCSR_DMA_SR_TE: c_uint = 0x00000080;
pub const CCSR_DMA_SR_CH: c_uint = 0x00000020;
pub const CCSR_DMA_SR_PE: c_uint = 0x00000010;
pub const CCSR_DMA_SR_EOLNI: c_uint = 0x00000008;
pub const CCSR_DMA_SR_CB: c_uint = 0x00000004;
pub const CCSR_DMA_SR_EOSI: c_uint = 0x00000002;
pub const CCSR_DMA_SR_EOLSI: c_uint = 0x00000001;
// ECLNDAR takes bits 32-36 of the CLNDAR register

pub const CCSR_DMA_CLNDAR_EOSIE: c_uint = 0x00000008;
// SATR and DATR, combined
pub const CCSR_DMA_ATR_PBATMU: c_uint = 0x20000000;
pub const CCSR_DMA_ATR_TFLOWLVL_0: c_uint = 0x00000000;
pub const CCSR_DMA_ATR_TFLOWLVL_1: c_uint = 0x06000000;
pub const CCSR_DMA_ATR_TFLOWLVL_2: c_uint = 0x08000000;
pub const CCSR_DMA_ATR_TFLOWLVL_3: c_uint = 0x0C000000;
pub const CCSR_DMA_ATR_PCIORDER: c_uint = 0x02000000;
pub const CCSR_DMA_ATR_SME: c_uint = 0x01000000;
pub const CCSR_DMA_ATR_NOSNOOP: c_uint = 0x00040000;
pub const CCSR_DMA_ATR_SNOOP: c_uint = 0x00050000;
pub const CCSR_DMA_ATR_ESAD_MASK: c_uint = 0x0000000F;
//
// List Descriptor for extended chaining mode DMA operations.
//
// The CLSDAR register points to the first (in a linked-list) List
// Descriptor.  Each object must be aligned on a 32-byte boundary. Each
// list descriptor points to a linked-list of link Descriptors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dma_list_descriptor {
    pub /: *mut *mut __be64 next; / Address of next list descriptor,
    pub /: *mut *mut __be64 first_link; / Address of first link descriptor,
    pub /: *mut *mut __be32 source; / Source stride,
    pub /: *mut *mut __be32 dest; / Destination stride,
    pub /: *mut *mut u8 res[8]; / Reserved,
// C attribute field omitted
//
// Link Descriptor for basic and extended chaining mode DMA operations.
//
// A Link Descriptor points to a single DMA buffer.  Each link descriptor
// must be aligned on a 32-byte boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dma_link_descriptor {
    pub /: *mut *mut __be32 source_attr; / Programmed into SATR register,
    pub /: *mut *mut __be32 source_addr; / Programmed into SAR register,
    pub /: *mut *mut __be32 dest_attr; / Programmed into DATR register,
    pub /: *mut *mut __be32 dest_addr; / Programmed into DAR register,
    pub /: *mut *mut __be64 next; / Address of next link descriptor,
    pub /: *mut *mut __be32 count; / Byte count,
    pub /: *mut *mut u8 res[4]; / Reserved,
// C attribute field omitted
