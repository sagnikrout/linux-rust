//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/fsl_raid.h
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
// drivers/dma/fsl_raid.h
//
// Freescale RAID Engine device driver
//
// Author:
// Harninder Rai <harninder.rai@freescale.com>
// Naveen Burmi <naveenburmi@freescale.com>
//
// Rewrite:
// Xuelin Shi <xuelin.shi@freescale.com>
// Copyright (c) 2010-2012 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
pub const FSL_RE_MAX_CHANS: c_int = 4;

pub const FSL_RE_GFM_POLY: c_uint = 0x1d000000;

pub const FSL_RE_CFG1_CBSI: c_uint = 0x08000000;
pub const FSL_RE_CFG1_CBS0: c_uint = 0x00080000;
pub const FSL_RE_SLOT_FULL_SHIFT: c_int = 8;

pub const FSL_RE_SLOT_AVAIL_SHIFT: c_int = 8;

pub const FSL_RE_PQ_OPCODE: c_uint = 0x1B;
pub const FSL_RE_XOR_OPCODE: c_uint = 0x1A;
pub const FSL_RE_MOVE_OPCODE: c_uint = 0x8;
pub const FSL_RE_FRAME_ALIGN: c_int = 16;
pub const FSL_RE_BLOCK_SIZE: c_uint = 0x3 /* 4096 bytes */;
pub const FSL_RE_CACHEABLE_IO: c_uint = 0x0;
pub const FSL_RE_BUFFER_OUTPUT: c_uint = 0x0;
pub const FSL_RE_INTR_ON_ERROR: c_uint = 0x1;
pub const FSL_RE_DATA_DEP: c_uint = 0x1;
pub const FSL_RE_ENABLE_DPI: c_uint = 0x0;
pub const FSL_RE_RING_SIZE: c_uint = 0x400;

pub const FSL_RE_RING_SIZE_SHIFT: c_int = 8;
pub const FSL_RE_ADDR_BIT_SHIFT: c_int = 4;

pub const FSL_RE_ERROR: c_uint = 0x40000000;
pub const FSL_RE_INTR: c_uint = 0x80000000;
pub const FSL_RE_CLR_INTR: c_uint = 0x80000000;
pub const FSL_RE_PAUSE: c_uint = 0x80000000;
pub const FSL_RE_ENABLE: c_uint = 0x80000000;
pub const FSL_RE_REG_LIODN_MASK: c_uint = 0x00000FFF;
pub const FSL_RE_CDB_OPCODE_MASK: c_uint = 0xF8000000;
pub const FSL_RE_CDB_OPCODE_SHIFT: c_int = 27;
pub const FSL_RE_CDB_EXCLEN_MASK: c_uint = 0x03000000;
pub const FSL_RE_CDB_EXCLEN_SHIFT: c_int = 24;
pub const FSL_RE_CDB_EXCLQ1_MASK: c_uint = 0x00F00000;
pub const FSL_RE_CDB_EXCLQ1_SHIFT: c_int = 20;
pub const FSL_RE_CDB_EXCLQ2_MASK: c_uint = 0x000F0000;
pub const FSL_RE_CDB_EXCLQ2_SHIFT: c_int = 16;
pub const FSL_RE_CDB_BLKSIZE_MASK: c_uint = 0x0000C000;
pub const FSL_RE_CDB_BLKSIZE_SHIFT: c_int = 14;
pub const FSL_RE_CDB_CACHE_MASK: c_uint = 0x00003000;
pub const FSL_RE_CDB_CACHE_SHIFT: c_int = 12;
pub const FSL_RE_CDB_BUFFER_MASK: c_uint = 0x00000800;
pub const FSL_RE_CDB_BUFFER_SHIFT: c_int = 11;
pub const FSL_RE_CDB_ERROR_MASK: c_uint = 0x00000400;
pub const FSL_RE_CDB_ERROR_SHIFT: c_int = 10;
pub const FSL_RE_CDB_NRCS_MASK: c_uint = 0x0000003C;
pub const FSL_RE_CDB_NRCS_SHIFT: c_int = 6;
pub const FSL_RE_CDB_DEPEND_MASK: c_uint = 0x00000008;
pub const FSL_RE_CDB_DEPEND_SHIFT: c_int = 3;
pub const FSL_RE_CDB_DPI_MASK: c_uint = 0x00000004;
pub const FSL_RE_CDB_DPI_SHIFT: c_int = 2;
//
// the largest cf block is 19*sizeof(struct cmpnd_frame), which is 304 bytes.
// here 19 = 1(cdb)+2(dest)+16(src), align to 64bytes, that is 320 bytes.
// the largest cdb block: struct pq_cdb which is 180 bytes, adding to cf block
// 320+180=500, align to 64bytes, that is 512 bytes.
//
pub const FSL_RE_CF_DESC_SIZE: c_int = 320;
pub const FSL_RE_CF_CDB_SIZE: c_int = 512;
pub const FSL_RE_CF_CDB_ALIGN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_ctrl {
// General Configuration Registers
    pub /: *mut *mut __be32 global_config; / Global Configuration Register,
    pub rsvd1: [u8; 4],
    pub /: *mut *mut __be32 galois_field_config; / Galois Field Configuration Register,
    pub rsvd2: [u8; 4],
    pub /: *mut *mut __be32 jq_wrr_config; / WRR Configuration register,
    pub rsvd3: [u8; 4],
    pub /: *mut *mut __be32 crc_config; / CRC Configuration register,
    pub rsvd4: [u8; 228],
    pub /: *mut *mut __be32 system_reset; / System Reset Register,
    pub rsvd5: [u8; 252],
    pub /: *mut *mut __be32 global_status; / Global Status Register,
    pub rsvd6: [u8; 832],
    pub /: *mut *mut __be32 re_liodn_base; / LIODN Base Register,
    pub rsvd7: [u8; 1712],
    pub /: *mut *mut __be32 re_version_id; / Version ID register of RE,
    pub /: *mut *mut __be32 re_version_id_2; / Version ID 2 register of RE,
    pub rsvd8: [u8; 512],
    pub /: *mut *mut __be32 host_config; / Host I/F Configuration Register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_chan_cfg {
// Registers for JR interface
    pub /: *mut *mut __be32 jr_config_0; / Job Queue Configuration 0 Register,
    pub /: *mut *mut __be32 jr_config_1; / Job Queue Configuration 1 Register,
    pub /: *mut *mut __be32 jr_interrupt_status; / Job Queue Interrupt Status Register,
    pub rsvd1: [u8; 4],
    pub /: *mut *mut __be32 jr_command; / Job Queue Command Register,
    pub rsvd2: [u8; 4],
    pub /: *mut *mut __be32 jr_status; / Job Queue Status Register,
    pub rsvd3: [u8; 228],
// Input Ring
    pub /: *mut *mut __be32 inbring_base_h; / Inbound Ring Base Address Register - High,
    pub /: *mut *mut __be32 inbring_base_l; / Inbound Ring Base Address Register - Low,
    pub /: *mut *mut __be32 inbring_size; / Inbound Ring Size Register,
    pub rsvd4: [u8; 4],
    pub /: *mut *mut __be32 inbring_slot_avail; / Inbound Ring Slot Available Register,
    pub rsvd5: [u8; 4],
    pub /: *mut *mut __be32 inbring_add_job; / Inbound Ring Add Job Register,
    pub rsvd6: [u8; 4],
    pub /: *mut *mut __be32 inbring_cnsmr_indx; / Inbound Ring Consumer Index Register,
    pub rsvd7: [u8; 220],
// Output Ring
    pub /: *mut *mut __be32 oubring_base_h; / Outbound Ring Base Address Register - High,
    pub /: *mut *mut __be32 oubring_base_l; / Outbound Ring Base Address Register - Low,
    pub /: *mut *mut __be32 oubring_size; / Outbound Ring Size Register,
    pub rsvd8: [u8; 4],
    pub /: *mut *mut __be32 oubring_job_rmvd; / Outbound Ring Job Removed Register,
    pub rsvd9: [u8; 4],
    pub /: *mut *mut __be32 oubring_slot_full; / Outbound Ring Slot Full Register,
    pub rsvd10: [u8; 4],
    pub /: *mut *mut __be32 oubring_prdcr_indx; / Outbound Ring Producer Index,
}

//
// Command Descriptor Block (CDB) for unicast move command.
// In RAID Engine terms, memcpy is done through move command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_move_cdb {
    pub cdb32: __be32,
}

// Data protection/integrity related fields
pub const FSL_RE_DPI_APPS_MASK: c_uint = 0xC0000000;
pub const FSL_RE_DPI_APPS_SHIFT: c_int = 30;
pub const FSL_RE_DPI_REF_MASK: c_uint = 0x30000000;
pub const FSL_RE_DPI_REF_SHIFT: c_int = 28;
pub const FSL_RE_DPI_GUARD_MASK: c_uint = 0x0C000000;
pub const FSL_RE_DPI_GUARD_SHIFT: c_int = 26;
pub const FSL_RE_DPI_ATTR_MASK: c_uint = 0x03000000;
pub const FSL_RE_DPI_ATTR_SHIFT: c_int = 24;
pub const FSL_RE_DPI_META_MASK: c_uint = 0x0000FFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_dpi {
    pub dpi32: __be32,
    pub ref: __be32,
}

//
// CDB for GenQ command. In RAID Engine terminology, XOR is
// done through this command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_xor_cdb {
    pub cdb32: __be32,
    pub gfm: [u8; 16],
    pub dpi_dest_spec: fsl_re_dpi,
    pub dpi_src_spec: [fsl_re_dpi; 16],
}

// CDB for no-op command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_noop_cdb {
    pub cdb32: __be32,
}

//
// CDB for GenQQ command. In RAID Engine terminology, P/Q is
// done through this command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_pq_cdb {
    pub cdb32: __be32,
    pub gfm_q1: [u8; 16],
    pub gfm_q2: [u8; 16],
    pub dpi_dest_spec: [fsl_re_dpi; 2],
    pub dpi_src_spec: [fsl_re_dpi; 16],
}

// Compound frame
pub const FSL_RE_CF_ADDR_HIGH_MASK: c_uint = 0x000000FF;
pub const FSL_RE_CF_EXT_MASK: c_uint = 0x80000000;
pub const FSL_RE_CF_EXT_SHIFT: c_int = 31;
pub const FSL_RE_CF_FINAL_MASK: c_uint = 0x40000000;
pub const FSL_RE_CF_FINAL_SHIFT: c_int = 30;
pub const FSL_RE_CF_LENGTH_MASK: c_uint = 0x000FFFFF;
pub const FSL_RE_CF_BPID_MASK: c_uint = 0x00FF0000;
pub const FSL_RE_CF_BPID_SHIFT: c_int = 16;
pub const FSL_RE_CF_OFFSET_MASK: c_uint = 0x00001FFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_cmpnd_frame {
    pub addr_high: __be32,
    pub addr_low: __be32,
    pub efrl32: __be32,
    pub rbro32: __be32,
}

// Frame descriptor
pub const FSL_RE_HWDESC_LIODN_MASK: c_uint = 0x3F000000;
pub const FSL_RE_HWDESC_LIODN_SHIFT: c_int = 24;
pub const FSL_RE_HWDESC_BPID_MASK: c_uint = 0x00FF0000;
pub const FSL_RE_HWDESC_BPID_SHIFT: c_int = 16;
pub const FSL_RE_HWDESC_ELIODN_MASK: c_uint = 0x0000F000;
pub const FSL_RE_HWDESC_ELIODN_SHIFT: c_int = 12;
pub const FSL_RE_HWDESC_FMT_SHIFT: c_int = 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_hw_desc {
    pub lbea32: __be32,
    pub addr_low: __be32,
    pub fmt32: __be32,
    pub status: __be32,
}

// Raid Engine device private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_drv_private {
    pub total_chans: u8,
    pub dma_dev: dma_device,
    pub base: *mut void __iomem,
    pub re_jrs: [*mut fsl_re_chan; FSL_RE_MAX_CHANS],
    pub cf_desc_pool: *mut dma_pool,
    pub hw_desc_pool: *mut dma_pool,
}

// Per job ring data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_chan {
    pub name: [c_char; 16],
    pub /: *mut *mut spinlock_t desc_lock; / queue lock,
    pub /: *mut *mut list_head ack_q; / wait to acked queue,
    pub /: *mut *mut list_head active_q; / already issued on hw, not completed,
    pub submit_q: list_head,
    pub /: *mut *mut list_head free_q; / alloc available queue,
    pub dev: *mut device,
    pub re_dev: *mut fsl_re_drv_private,
    pub chan: dma_chan,
    pub jrregs: *mut fsl_re_chan_cfg __iomem,
    pub irq: c_int,
    pub irqtask: tasklet_struct,
    pub alloc_count: u32,
// hw descriptor ring for inbound queue
    pub inb_phys_addr: dma_addr_t,
    pub inb_ring_virt_addr: *mut fsl_re_hw_desc,
    pub inb_count: u32,
// hw descriptor ring for outbound queue
    pub oub_phys_addr: dma_addr_t,
    pub oub_ring_virt_addr: *mut fsl_re_hw_desc,
    pub oub_count: u32,
}

// Async transaction descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_re_desc {
    pub async_tx: dma_async_tx_descriptor,
    pub node: list_head,
    pub hwdesc: fsl_re_hw_desc,
    pub re_chan: *mut fsl_re_chan,
// hwdesc will point to cf_addr
    pub cf_addr: *mut c_void,
    pub cf_paddr: dma_addr_t,
    pub cdb_addr: *mut c_void,
    pub cdb_paddr: dma_addr_t,
    pub status: c_int,
}
