//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_wr.h
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
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2008-2012 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// SGE register field values.
//
pub const X_INGPCIEBOUNDARY_32B: c_int = 0;
pub const X_INGPCIEBOUNDARY_64B: c_int = 1;
pub const X_INGPCIEBOUNDARY_128B: c_int = 2;
pub const X_INGPCIEBOUNDARY_256B: c_int = 3;
pub const X_INGPCIEBOUNDARY_512B: c_int = 4;
pub const X_INGPCIEBOUNDARY_1024B: c_int = 5;
pub const X_INGPCIEBOUNDARY_2048B: c_int = 6;
pub const X_INGPCIEBOUNDARY_4096B: c_int = 7;
// GTS register
pub const X_TIMERREG_COUNTER0: c_int = 0;
pub const X_TIMERREG_COUNTER1: c_int = 1;
pub const X_TIMERREG_COUNTER2: c_int = 2;
pub const X_TIMERREG_COUNTER3: c_int = 3;
pub const X_TIMERREG_COUNTER4: c_int = 4;
pub const X_TIMERREG_COUNTER5: c_int = 5;
pub const X_TIMERREG_RESTART_COUNTER: c_int = 6;
pub const X_TIMERREG_UPDATE_CIDX: c_int = 7;
//
// Egress Context field values
//
pub const X_FETCHBURSTMIN_16B: c_int = 0;
pub const X_FETCHBURSTMIN_32B: c_int = 1;
pub const X_FETCHBURSTMIN_64B: c_int = 2;
pub const X_FETCHBURSTMIN_128B: c_int = 3;
pub const X_FETCHBURSTMAX_64B: c_int = 0;
pub const X_FETCHBURSTMAX_128B: c_int = 1;
pub const X_FETCHBURSTMAX_256B: c_int = 2;
pub const X_FETCHBURSTMAX_512B: c_int = 3;
pub const X_HOSTFCMODE_NONE: c_int = 0;
pub const X_HOSTFCMODE_INGRESS_QUEUE: c_int = 1;
pub const X_HOSTFCMODE_STATUS_PAGE: c_int = 2;
pub const X_HOSTFCMODE_BOTH: c_int = 3;
//
// Ingress Context field values
//
pub const X_UPDATESCHEDULING_TIMER: c_int = 0;
pub const X_UPDATESCHEDULING_COUNTER_OPTTIMER: c_int = 1;
pub const X_UPDATEDELIVERY_NONE: c_int = 0;
pub const X_UPDATEDELIVERY_INTERRUPT: c_int = 1;
pub const X_UPDATEDELIVERY_STATUS_PAGE: c_int = 2;
pub const X_UPDATEDELIVERY_BOTH: c_int = 3;
pub const X_INTERRUPTDESTINATION_PCIE: c_int = 0;
pub const X_INTERRUPTDESTINATION_IQ: c_int = 1;
pub const X_RSPD_TYPE_FLBUF: c_int = 0;
pub const X_RSPD_TYPE_CPL: c_int = 1;
pub const X_RSPD_TYPE_INTR: c_int = 2;
// WR status is at the same position as retval in a CMD header

// Ingress queue params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_iq_params {
    pub iq_start:1: u8,
    pub iq_stop:1: u8,
    pub pfn:3: u8,
    pub vfn: u8,
    pub physiqid: u16,
    pub iqid: u16,
    pub fl0id: u16,
    pub fl1id: u16,
    pub viid: u8,
    pub type: u8,
    pub iqasynch: u8,
    pub reserved4: u8,
    pub iqandst: u8,
    pub iqanus: u8,
    pub iqanud: u8,
    pub iqandstindex: u16,
    pub iqdroprss: u8,
    pub iqpciech: u8,
    pub iqdcaen: u8,
    pub iqdcacpu: u8,
    pub iqintcntthresh: u8,
    pub iqo: u8,
    pub iqcprio: u8,
    pub iqesize: u8,
    pub iqsize: u16,
    pub iqaddr: u64,
    pub iqflintiqhsen: u8,
    pub reserved5: u8,
    pub iqflintcongen: u8,
    pub iqflintcngchmap: u8,
    pub reserved6: u32,
    pub fl0hostfcmode: u8,
    pub fl0cprio: u8,
    pub fl0paden: u8,
    pub fl0packen: u8,
    pub fl0congen: u8,
    pub fl0dcaen: u8,
    pub fl0dcacpu: u8,
    pub fl0fbmin: u8,
    pub fl0fbmax: u8,
    pub fl0cidxfthresho: u8,
    pub fl0cidxfthresh: u8,
    pub fl0size: u16,
    pub fl0addr: u64,
    pub reserved7: u64,
    pub fl1hostfcmode: u8,
    pub fl1cprio: u8,
    pub fl1paden: u8,
    pub fl1packen: u8,
    pub fl1congen: u8,
    pub fl1dcaen: u8,
    pub fl1dcacpu: u8,
    pub fl1fbmin: u8,
    pub fl1fbmax: u8,
    pub fl1cidxfthresho: u8,
    pub fl1cidxfthresh: u8,
    pub fl1size: u16,
    pub fl1addr: u64,
}

// Egress queue params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_eq_params {
    pub pfn: u8,
    pub vfn: u8,
    pub eqstart:1: u8,
    pub eqstop:1: u8,
    pub physeqid: u16,
    pub eqid: u32,
    pub hostfcmode:2: u8,
    pub cprio:1: u8,
    pub pciechn:3: u8,
    pub iqid: u16,
    pub dcaen:1: u8,
    pub dcacpu:5: u8,
    pub fbmin:3: u8,
    pub fbmax:3: u8,
    pub cidxfthresho:1: u8,
    pub cidxfthresh:3: u8,
    pub eqsize: u16,
    pub eqaddr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_dma_buf {
    pub list: list_head,
    pub /: *mut *mut *mut void vaddr; / Virtual address,
    pub /: *mut *mut dma_addr_t paddr; / Physical address,
    pub /: *mut *mut uint32_t len; / Buffer size,
}

// Generic I/O request structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_ioreq {
    pub List: *mut *mut csio_sm sm; / SM,,
// should be the first member
//
    pub /: *mut *mut int iq_idx; / Ingress queue index,
    pub /: *mut *mut int eq_idx; / Egress queue index,
    pub /: *mut *mut uint32_t nsge; / Number of SG elements,
    pub /: *mut *mut uint32_t tmo; / Driver timeout,
    pub /: *mut *mut uint32_t datadir; / Data direction,
    pub /: *mut *mut csio_dma_buf dma_buf; / Req/resp DMA buffers,
    pub /: *mut *mut uint16_t wr_status; / WR completion status,
    pub /: *mut *mut int16_t drv_status; / Driver internal status,
    pub /: *mut *mut *mut csio_lnode lnode; / Owner lnode,
    pub /: *mut *mut *mut csio_rnode rnode; / Src/destination rnode,
    pub ): *mut *mut *mut void (io_cbfn) (struct csio_hw , struct csio_ioreq,
// completion callback
    pub 1.: *mut *mut *mut void scratch1; / Scratch area,
//
    pub /: *mut *mut *mut void scratch2; / Scratch area 2.,
    pub with: *mut *mut list_head gen_list; / Any list associated,
// this ioreq.
//
    pub passed: *mut *mut uint64_t fw_handle; / Unique handle,
// to FW
//
    pub /: *mut *mut uint8_t dcopy; / Data copy required,
    pub reserved1: u8,
    pub reserved2: u16,
    pub /: *mut *mut completion cmplobj; / ioreq completion object,
    pub ____cacheline_aligned_in_smp: },
//
// Egress status page for egress cidx updates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_qstatus_page {
    pub qid: __be32,
    pub cidx: __be16,
    pub pidx: __be16,
}

// in bytes
//
// Defines for type
//
// Structure for footer (last 2 flits) of Ingress Queue Entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_iqwr_footer {
    pub hdrbuflen_pidx: __be32,
    pub pldbuflen_qid: __be32,
    pub type_gen: u8,
    pub last_flit: __be64,
    pub u: },
}

pub const IQWRF_GEN_SHIFT: c_int = 7;

//
// WR pair:
// ========
// A WR can start towards the end of a queue, and then continue at the
// beginning, since the queue is considered to be circular. This will
// require a pair of address/len to be passed back to the caller -
// hence the Work request pair structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_wr_pair {
    pub addr1: *mut c_void,
    pub size1: u32,
    pub addr2: *mut c_void,
    pub size2: u32,
}

//
// The following structure is used by ingress processing to return the
// free list buffers to consumers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_fl_dma_buf {
    pub flbufs: [csio_dma_buf; CSIO_MAX_FLBUF_PER_IQWR],
// Freelist DMA buffers
    pub the: *mut *mut int offset; / Offset within,
// first FL buf.
//
    pub /: *mut *mut uint32_t totlen; / Total length,
    pub can: *mut *mut uint8_t defer_free; / Free of buffer,
// deferred
//
}

// Data-types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_iq {
    pub /: *mut *mut uint16_t iqid; / Queue ID,
    pub /: *mut *mut uint16_t physiqid; / Physical Queue ID,
    pub bit,: *mut *mut uint16_t genbit; / Generation,
// initially set to 1
//
    pub /: *mut *mut int flq_idx; / Freelist queue index,
    pub /: *mut *mut iq_handler_t iq_intx_handler; / IQ INTx handler routine,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_eq {
    pub /: *mut *mut uint16_t eqid; / Qid,
    pub /: *mut *mut uint16_t physeqid; / Physical Queue ID,
    pub around*/: *mut *mut uint8_t wrap[512]; / Temp area for q-wrap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_fl {
    pub /: *mut *mut uint16_t flid; / Qid,
    pub /: *mut *mut uint16_t packen; / Packing enabled?,
    pub /: *mut *mut int offset; / Offset within FL buf,
    pub /: *mut *mut int sreg; / Size register,
    pub array: *mut *mut *mut csio_dma_buf bufs; / Free list buffer ptr,
// indexed using flq->cidx/pidx
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_qstats {
    pub /: *mut *mut uint32_t n_tot_reqs; / Total no. of Requests,
    pub /: *mut *mut uint32_t n_tot_rsps; / Total no. of responses,
    pub /: *mut *mut uint32_t n_qwrap; / Queue wraps,
    pub /: *mut *mut uint32_t n_eq_wr_split; / Number of split EQ WRs,
    pub /: *mut *mut uint32_t n_qentry; / Queue entry,
    pub /: *mut *mut uint32_t n_qempty; / Queue empty,
    pub /: *mut *mut uint32_t n_qfull; / Queue fulls,
    pub /: *mut *mut uint32_t n_rsp_unknown; / Unknown response type,
    pub /: *mut *mut uint32_t n_stray_comp; / Stray completion intr,
    pub /: *mut *mut uint32_t n_flq_refill; / Number of FL refills,
}

// Queue metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_q {
    pub /: *mut *mut uint16_t type; / Type: Ingress/Egress/FL,
    pub /: *mut *mut uint16_t pidx; / producer index,
    pub /: *mut *mut uint16_t cidx; / consumer index,
    pub /: *mut *mut uint16_t inc_idx; / Incremental index,
    pub q: *mut *mut uint32_t wr_sz; / Size of all WRs in this,
// if fixed
//
    pub address: *mut *mut *mut void vstart; / Base virtual,
// of queue
//
    pub to: *mut *mut *mut void vwrap; / Virtual end address,
// wrap around at
//
    pub /: *mut *mut uint32_t credits; / Size of queue in credits,
    pub /: *mut *mut *mut void owner; / Owner,
    pub iq: csio_iq,
    pub eq: csio_eq,
    pub fl: csio_fl,
    pub un: },
    pub of: *mut *mut dma_addr_t pstart; / Base physical address,
// queue
//
    pub /: *mut *mut uint32_t portid; / PCIE Channel,
    pub /: *mut *mut uint32_t size; / Size of queue in bytes,
    pub /: *mut *mut csio_qstats stats; / Statistics,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_sge {
    pub cached: *mut *mut uint32_t csio_fl_align; / Calculated and,
// for fast path
//
    pub boundaries,: *mut *mut uint32_t sge_control; / padding,,
// lengths, etc.
//
    pub /: *mut *mut uint32_t sge_host_page_size; / Host page size,
    pub sge_fl_buf_size: [u32; CSIO_SGE_FL_SIZE_REGS],
// free list buffer sizes
    pub timer_val: [u16; CSIO_SGE_NTIMERS],
    pub counter_val: [u8; CSIO_SGE_NCOUNTERS],
}

// Work request module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_wrm {
    pub /: *mut *mut int num_q; / Number of queues,
    pub pointers: *mut *mut *mut *mut csio_q q_arr; / Array of queue,
// allocated dynamically
// based on configured values
//
    pub fn*/: *mut *mut uint32_t fw_iq_start; / Start ID of IQ for this,
    pub fn*/: *mut *mut uint32_t fw_eq_start; / Start ID of EQ for this,
    pub intr_map: [*mut csio_q; CSIO_MAX_IQ],
// IQ-id to IQ map table.
    pub /: *mut *mut int free_qidx; / queue idx of free queue,
    pub /: *mut *mut csio_sge sge; / SGE params,
}

extern "C" {
    pub fn csio_wr_destroy_queues(: *mut csio_hw, cmd: bool) -> c_int;
}
extern "C" {
    pub fn csio_wr_copy_to_wrp(: *mut c_void, : *mut csio_wr_pair, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn csio_wr_issue(: *mut csio_hw, _arg: c_int, _arg: bool) -> c_int;
}
extern "C" {
    pub fn csio_wr_sge_init(: *mut csio_hw);
}
extern "C" {
    pub fn csio_wrm_init(: *mut csio_wrm, : *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_wrm_exit(: *mut csio_wrm, : *mut csio_hw);
}
