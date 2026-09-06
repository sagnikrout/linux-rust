//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/fsl/qbman/qman_priv.h
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


// Copyright 2008 - 2016 Freescale Semiconductor, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcr_querywq {
    pub verb: u8,
    pub result: u8,
    pub /: *mut *mut u16 channel_wq; / ignores wq (3 lsbits): _res[0-2],
    pub __reserved: [u8; 28],
    pub wq_len: [u32; 8],
    pub __packed: },
    pub 3: return wq->channel_wq >>,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __qm_mcr_querycongestion {
    pub state: [u32; 8],
}

// "Query Congestion Group State"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcr_querycongestion {
    pub verb: u8,
    pub result: u8,
    pub __reserved: [u8; 30],
// Access this struct using qman_cgrs_get()
    pub state: __qm_mcr_querycongestion,
    pub __packed: },
// "Query CGR"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcr_querycgr {
    pub verb: u8,
    pub result: u8,
    pub __reserved1: u16,
    pub /: *mut *mut __qm_mc_cgr cgr; / CGR fields,
    pub __reserved2: [u8; 6],
    pub /: *mut *mut u8 i_bcnt_hi; / high 8-bits of 40-bit "Instant",
    pub /: *mut *mut __be32 i_bcnt_lo; / low 32-bits of 40-bit,
    pub __reserved3: [u8; 3],
    pub /: *mut *mut u8 a_bcnt_hi; / high 8-bits of 40-bit "Average",
    pub /: *mut *mut __be32 a_bcnt_lo; / low 32-bits of 40-bit,
    pub cscn_targ_swp: [__be32; 4],
    pub __packed: },
    pub be32_to_cpu(q->i_bcnt_lo): return ((u64)q->i_bcnt_hi << 32) |,
    pub be32_to_cpu(q->a_bcnt_lo): return ((u64)q->a_bcnt_hi << 32) |,
// Congestion Groups
//
// This wrapper represents a bit-array for the state of the 256 QMan congestion
// groups. Is also used as a *mask* for congestion groups, eg. so we ignore
// those that don't concern us. We harness the structure and accessor details
// already used in the management command to query congestion groups.
//
pub const CGR_BITS_PER_WORD: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qman_cgrs {
    pub q: __qm_mcr_querycongestion,
}

// dest = *src;
// _d++ = *_a++ & *_b++;
// _d++ = *_a++ ^ *_b++;
extern "C" {
    pub fn qman_init_cgr_all();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_portal_config {
// Portal addresses
    pub addr_virt_ce: *mut c_void,
    pub addr_virt_ci: *mut void __iomem,
    pub dev: *mut device,
    pub iommu_domain: *mut iommu_domain,
// Allow these to be joined in lists
    pub list: list_head,
// User-visible portal configuration settings
// portal is affined to this cpu
    pub cpu: c_int,
// portal interrupt line
    pub irq: c_int,
//
// the portal's dedicated channel id, used initialising
// frame queues to target this portal when scheduled
//
    pub channel: u16,
//
// mask of pool channels this portal has dequeue access to
// (using QM_SDQCR_CHANNELS_POOL(n) for the bitmask)
//
    pub pools: u32,
}

// Revision info (for errata and feature handling)
pub const QMAN_REV11: c_uint = 0x0101;
pub const QMAN_REV12: c_uint = 0x0102;
pub const QMAN_REV20: c_uint = 0x0200;
pub const QMAN_REV30: c_uint = 0x0300;
pub const QMAN_REV31: c_uint = 0x0301;
pub const QMAN_REV32: c_uint = 0x0302;

extern "C" {
    pub fn qm_get_pools_sdqcr() -> u32;
}
extern "C" {
    pub fn qman_wq_alloc() -> c_int;
}

extern "C" {
    pub fn __qman_liodn_fixup(channel: u16);
}
extern "C" {
    pub fn qman_set_sdest(channel: u16, cpu_idx: c_uint);
}
//
// qman_query_fq - Queries FQD fields (via h/w query command)
// @fq: the frame queue object to be queried
// @fqd: storage for the queried FQD fields
//
extern "C" {
    pub fn qman_query_fq(fq: *mut qman_fq, fqd: *mut qm_fqd) -> c_int;
}
extern "C" {
    pub fn qman_alloc_fq_table(num_fqids: u32) -> c_int;
}
// QMan s/w corenet portal, low-level i/face
//
// For qm_dqrr_sdqcr_set(); Choose one SOURCE. Choose one COUNT. Choose one
// dequeue TYPE. Choose TOKEN (8-bit).
// If SOURCE == CHANNELS,
// Choose CHANNELS_DEDICATED and/or CHANNELS_POOL(n).
// You can choose DEDICATED_PRECEDENCE if the portal channel should have
// priority.
// If SOURCE == SPECIFICWQ,
// Either select the work-queue ID with SPECIFICWQ_WQ(), or select the
// channel (SPECIFICWQ_DEDICATED or SPECIFICWQ_POOL()) and specify the
// work-queue priority (0-7) with SPECIFICWQ_WQ() - either way, you get the
// same value.
//
pub const QM_SDQCR_SOURCE_CHANNELS: c_uint = 0x0;
pub const QM_SDQCR_SOURCE_SPECIFICWQ: c_uint = 0x40000000;
pub const QM_SDQCR_COUNT_EXACT1: c_uint = 0x0;
pub const QM_SDQCR_COUNT_UPTO3: c_uint = 0x20000000;
pub const QM_SDQCR_DEDICATED_PRECEDENCE: c_uint = 0x10000000;
pub const QM_SDQCR_TYPE_MASK: c_uint = 0x03000000;
pub const QM_SDQCR_TYPE_NULL: c_uint = 0x0;
pub const QM_SDQCR_TYPE_PRIO_QOS: c_uint = 0x01000000;
pub const QM_SDQCR_TYPE_ACTIVE_QOS: c_uint = 0x02000000;
pub const QM_SDQCR_TYPE_ACTIVE: c_uint = 0x03000000;
pub const QM_SDQCR_TOKEN_MASK: c_uint = 0x00ff0000;

pub const QM_SDQCR_CHANNELS_DEDICATED: c_uint = 0x00008000;
pub const QM_SDQCR_SPECIFICWQ_MASK: c_uint = 0x000000f7;
pub const QM_SDQCR_SPECIFICWQ_DEDICATED: c_uint = 0x00000000;

// For qm_dqrr_vdqcr_set(): use FQID(n) to fill in the frame queue ID
pub const QM_VDQCR_FQID_MASK: c_uint = 0x00ffffff;

//
// Used by all portal interrupt registers except 'inhibit'
// Channels with frame availability
//
pub const QM_PIRQ_DQAVAIL: c_uint = 0x0000ffff;
// The DQAVAIL interrupt fields break down into these bits;
pub const QM_DQAVAIL_PORTAL: c_uint = 0x8000		/* Portal channel */;

pub const QM_DQAVAIL_MASK: c_uint = 0xffff;
// This mask contains all the "irqsource" bits visible to API users

extern "C" {
    pub fn qm_get_fqid_maxcnt() -> c_uint;
}
extern "C" {
    pub fn qman_shutdown_fq(fqid: u32) -> c_int;
}
extern "C" {
    pub fn qman_requires_cleanup() -> c_int;
}
extern "C" {
    pub fn qman_done_cleanup();
}
extern "C" {
    pub fn qman_enable_irqs();
}
