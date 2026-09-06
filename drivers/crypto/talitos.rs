//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/talitos.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Freescale SEC (talitos) device register and descriptor header defines
//
// Copyright (c) 2006-2011 Freescale Semiconductor, Inc.
//
pub const TALITOS_TIMEOUT: c_int = 100000;
pub const TALITOS1_MAX_DATA_LEN: c_int = 32768;
pub const TALITOS2_MAX_DATA_LEN: c_int = 65535;

// descriptor pointer entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct talitos_ptr {
    pub /: *mut *mut __be16 len; / length,
    pub extent*/: *mut *mut u8 j_extent; / jump to sg link table and/or,
    pub /: *mut *mut u8 eptr; / extended address,
}

// descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct talitos_desc {
    pub /: *mut *mut __be32 hdr; / header high bits,
    pub /: *mut *mut __be32 hdr_lo; / header low bits,
    pub /: *mut *mut __be32 hdr1; / header for SEC1,
}

//
// talitos_edesc - s/w-extended descriptor
// @bufsl: scatterlist buffer
// @src: pointer to input scatterlist
// @first: first descriptor of a chain
// @last: last descriptor of a chain
//
// @src_nents: number of segments in input scatterlist
// @dst_nents: number of segments in output scatterlist
// @iv_dma: dma address of iv for checking continuity and link table
// @dma_len: length of dma mapped link_tbl space
// @dma_link_tbl: bus physical address of link_tbl/buf
// @next_desc: next descriptor
// @desc: h/w descriptor
// @link_tbl: input and output h/w link tables (if {src,dst}_nents > 1) (SEC2)
// @buf: input and output buffeur (if {src,dst}_nents > 1) (SEC1)
//
// if decrypting (with authcheck), or either one of src_nents or dst_nents
// is greater than 1, an integrity check value is concatenated to the end
// of link_tbl data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct talitos_edesc {
    pub bufsl: [scatterlist; 2],
    pub src: *mut scatterlist,
    pub first: c_int,
    pub last: c_int,
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub iv_dma: dma_addr_t,
    pub dma_len: c_int,
    pub dma_link_tbl: dma_addr_t,
    pub next_desc: *mut talitos_edesc,
    pub desc: talitos_desc,
    pub link_tbl): DECLARE_FLEX_ARRAY(struct talitos_ptr,,
    pub buf): DECLARE_FLEX_ARRAY(u8,,
}

//
// talitos_request - descriptor submission request
// @desc: descriptor pointer (kernel virtual)
// @dma_desc: descriptor's physical bus address
// @callback: whom to call when descriptor processing is done
// @context: caller context (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct talitos_request {
    pub desc: *mut talitos_desc,
    pub dma_desc: dma_addr_t,
    pub error): *mut *mut void context, int,
    pub context: *mut c_void,
}

// per-channel fifo management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct talitos_channel {
    pub reg: *mut void __iomem,
// request fifo
    pub fifo: *mut talitos_request,
// number of requests pending in channel h/w fifo
    pub ____cacheline_aligned: atomic_t submit_count,
// request submission (head) lock
    pub ____cacheline_aligned: spinlock_t head_lock,
// index to next free descriptor request
    pub head: c_int,
// request release (tail) lock
    pub ____cacheline_aligned: spinlock_t tail_lock,
// index to next in-progress/done descriptor request
    pub tail: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct talitos_private {
    pub dev: *mut device,
    pub ofdev: *mut platform_device,
    pub reg: *mut void __iomem,
    pub reg_deu: *mut void __iomem,
    pub reg_aesu: *mut void __iomem,
    pub reg_mdeu: *mut void __iomem,
    pub reg_afeu: *mut void __iomem,
    pub reg_rngu: *mut void __iomem,
    pub reg_pkeu: *mut void __iomem,
    pub reg_keu: *mut void __iomem,
    pub reg_crcu: *mut void __iomem,
    pub irq: [c_int; 2],
// SEC global registers lock
    pub ____cacheline_aligned: spinlock_t reg_lock,
// SEC version geometry (from device tree node)
    pub num_channels: c_uint,
    pub chfifo_len: c_uint,
    pub exec_units: c_uint,
    pub desc_types: c_uint,
// SEC Compatibility info
    pub features: c_ulong,
//
// length of the request fifo
// fifo_len is chfifo_len rounded up to next power of 2
// so we can use bitwise ops to wrap
//
    pub fifo_len: c_uint,
// next channel to be assigned next incoming descriptor
    pub ____cacheline_aligned: atomic_t last_chan,
// request callback tasklet
    pub done_task: [tasklet_struct; 2],
// list of registered algorithms
    pub alg_list: list_head,
// hwrng device
    pub rng: hwrng,
    pub rng_registered: bool,
    pub __counted_by(num_channels): talitos_channel chan[],
}

// .features flag
pub const TALITOS_FTR_SRC_LINK_TBL_LEN_INCLUDES_EXTENT: c_uint = 0x00000001;
pub const TALITOS_FTR_HW_AUTH_CHECK: c_uint = 0x00000002;
pub const TALITOS_FTR_SHA224_HWINIT: c_uint = 0x00000004;
pub const TALITOS_FTR_HMAC_OK: c_uint = 0x00000008;
pub const TALITOS_FTR_SEC1: c_uint = 0x00000010;
//
// If both CONFIG_CRYPTO_DEV_TALITOS1 and CONFIG_CRYPTO_DEV_TALITOS2 are
// defined, we check the features which are set according to the device tree.
// Otherwise, we answer true or false directly
//
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_CRYPTO_DEV_TALITOS1) -> return;
}
//
// TALITOS_xxx_LO addresses point to the low data bits (32-63) of the register
//

// global register offset addresses
pub const TALITOS_MCR: c_uint = 0x1030  /* master control register */;

pub const TALITOS1_MCR_SWR: c_uint = 0x1000000     /* s/w reset */;
pub const TALITOS2_MCR_SWR: c_uint = 0x1     /* s/w reset */;
pub const TALITOS_MCR_LO: c_uint = 0x1034;
pub const TALITOS_IMR: c_uint = 0x1008  /* interrupt mask register */;
// enable channel IRQs

// enable channel IRQs

pub const TALITOS_IMR_LO: c_uint = 0x100C;
pub const TALITOS1_IMR_LO_INIT: c_uint = 0x2000000 /* allow RNGU error IRQs */;
pub const TALITOS2_IMR_LO_INIT: c_uint = 0x20000 /* allow RNGU error IRQs */;
pub const TALITOS_ISR: c_uint = 0x1010  /* interrupt status register */;

pub const TALITOS1_ISR_TEA_ERR: c_uint = 0x00000040;

pub const TALITOS_ISR_LO: c_uint = 0x1014;
pub const TALITOS_ICR: c_uint = 0x1018  /* interrupt clear register */;
pub const TALITOS_ICR_LO: c_uint = 0x101C;
// channel register address stride
pub const TALITOS_CH_BASE_OFFSET: c_uint = 0x1000	/* default channel map base */;
pub const TALITOS1_CH_STRIDE: c_uint = 0x1000;
pub const TALITOS2_CH_STRIDE: c_uint = 0x100;
// channel configuration register
pub const TALITOS_CCCR: c_uint = 0x8;
pub const TALITOS2_CCCR_CONT: c_uint = 0x2    /* channel continue on SEC2 */;
pub const TALITOS2_CCCR_RESET: c_uint = 0x1    /* channel reset on SEC2 */;
pub const TALITOS_CCCR_LO: c_uint = 0xc;
pub const TALITOS_CCCR_LO_IWSE: c_uint = 0x80   /* chan. ICCR writeback enab. */;
pub const TALITOS_CCCR_LO_EAE: c_uint = 0x20   /* extended address enable */;
pub const TALITOS_CCCR_LO_CDWE: c_uint = 0x10   /* chan. done writeback enab. */;
pub const TALITOS_CCCR_LO_NE: c_uint = 0x8    /* fetch next descriptor enab. */;
pub const TALITOS_CCCR_LO_NT: c_uint = 0x4    /* notification type */;
pub const TALITOS_CCCR_LO_CDIE: c_uint = 0x2    /* channel done IRQ enable */;
pub const TALITOS1_CCCR_LO_RESET: c_uint = 0x1    /* channel reset on SEC1 */;
// CCPSR: channel pointer status register
pub const TALITOS_CCPSR: c_uint = 0x10;
pub const TALITOS_CCPSR_LO: c_uint = 0x14;
pub const TALITOS_CCPSR_LO_DOF: c_uint = 0x8000 /* double FF write oflow error */;
pub const TALITOS_CCPSR_LO_SOF: c_uint = 0x4000 /* single FF write oflow error */;
pub const TALITOS_CCPSR_LO_MDTE: c_uint = 0x2000 /* master data transfer error */;
pub const TALITOS_CCPSR_LO_SGDLZ: c_uint = 0x1000 /* s/g data len zero error */;
pub const TALITOS_CCPSR_LO_FPZ: c_uint = 0x0800 /* fetch ptr zero error */;
pub const TALITOS_CCPSR_LO_IDH: c_uint = 0x0400 /* illegal desc hdr error */;
pub const TALITOS_CCPSR_LO_IEU: c_uint = 0x0200 /* invalid EU error */;
pub const TALITOS_CCPSR_LO_EU: c_uint = 0x0100 /* EU error detected */;
pub const TALITOS_CCPSR_LO_GB: c_uint = 0x0080 /* gather boundary error */;
pub const TALITOS_CCPSR_LO_GRL: c_uint = 0x0040 /* gather return/length error */;
pub const TALITOS_CCPSR_LO_SB: c_uint = 0x0020 /* scatter boundary error */;
pub const TALITOS_CCPSR_LO_SRL: c_uint = 0x0010 /* scatter return/length error */;
// channel fetch fifo register
pub const TALITOS_FF: c_uint = 0x48;
pub const TALITOS_FF_LO: c_uint = 0x4c;
// current descriptor pointer register
pub const TALITOS_CDPR: c_uint = 0x40;
pub const TALITOS_CDPR_LO: c_uint = 0x44;
// descriptor buffer register
pub const TALITOS_DESCBUF: c_uint = 0x80;
pub const TALITOS_DESCBUF_LO: c_uint = 0x84;
// gather link table
pub const TALITOS_GATHER: c_uint = 0xc0;
pub const TALITOS_GATHER_LO: c_uint = 0xc4;
// scatter link table
pub const TALITOS_SCATTER: c_uint = 0xe0;
pub const TALITOS_SCATTER_LO: c_uint = 0xe4;
// execution unit registers base
pub const TALITOS2_DEU: c_uint = 0x2000;
pub const TALITOS2_AESU: c_uint = 0x4000;
pub const TALITOS2_MDEU: c_uint = 0x6000;
pub const TALITOS2_AFEU: c_uint = 0x8000;
pub const TALITOS2_RNGU: c_uint = 0xa000;
pub const TALITOS2_PKEU: c_uint = 0xc000;
pub const TALITOS2_KEU: c_uint = 0xe000;
pub const TALITOS2_CRCU: c_uint = 0xf000;
pub const TALITOS12_AESU: c_uint = 0x4000;
pub const TALITOS12_DEU: c_uint = 0x5000;
pub const TALITOS12_MDEU: c_uint = 0x6000;
pub const TALITOS10_AFEU: c_uint = 0x8000;
pub const TALITOS10_DEU: c_uint = 0xa000;
pub const TALITOS10_MDEU: c_uint = 0xc000;
pub const TALITOS10_RNGU: c_uint = 0xe000;
pub const TALITOS10_PKEU: c_uint = 0x10000;
pub const TALITOS10_AESU: c_uint = 0x12000;
// execution unit interrupt status registers
pub const TALITOS_EUDSR: c_uint = 0x10	/* data size */;
pub const TALITOS_EUDSR_LO: c_uint = 0x14;
pub const TALITOS_EURCR: c_uint = 0x18 /* reset control*/;
pub const TALITOS_EURCR_LO: c_uint = 0x1c;
pub const TALITOS_EUSR: c_uint = 0x28 /* rng status */;
pub const TALITOS_EUSR_LO: c_uint = 0x2c;
pub const TALITOS_EUISR: c_uint = 0x30;
pub const TALITOS_EUISR_LO: c_uint = 0x34;
pub const TALITOS_EUICR: c_uint = 0x38 /* int. control */;
pub const TALITOS_EUICR_LO: c_uint = 0x3c;
pub const TALITOS_EU_FIFO: c_uint = 0x800 /* output FIFO */;
pub const TALITOS_EU_FIFO_LO: c_uint = 0x804 /* output FIFO */;
// DES unit
pub const TALITOS1_DEUICR_KPE: c_uint = 0x00200000 /* Key Parity Error */;
// message digest unit
pub const TALITOS_MDEUICR_LO_ICE: c_uint = 0x4000 /* integrity check IRQ enable */;
// random number unit
pub const TALITOS_RNGUSR_LO_RD: c_uint = 0x1	/* reset done */;
pub const TALITOS_RNGUSR_LO_OFL: c_uint = 0xff0000/* output FIFO length */;
pub const TALITOS_RNGURCR_LO_SR: c_uint = 0x1	/* software reset */;
pub const TALITOS_MDEU_CONTEXT_SIZE_MD5_SHA1_SHA256: c_uint = 0x28;
pub const TALITOS_MDEU_CONTEXT_SIZE_SHA384_SHA512: c_uint = 0x48;
//
// talitos descriptor header (hdr) bits
//
// written back when done

// primary execution unit select

// primary execution unit mode (MODE0) and derivatives

// secondary execution unit select (SEL1)

// secondary execution unit mode (MODE1) and derivatives

// direction of overall data flow (DIR)

// request done notification (DN)

// descriptor types

// link table extent field bits
pub const DESC_PTR_LNKTBL_JUMP: c_uint = 0x80;
pub const DESC_PTR_LNKTBL_RET: c_uint = 0x02;
pub const DESC_PTR_LNKTBL_NEXT: c_uint = 0x01;
