//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_debugfs.h
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
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2025 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2007-2011 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//

// size of output line, for discovery_trace and slow_ring_trace
pub const LPFC_DEBUG_TRC_ENTRY_SIZE: c_int = 100;
// nodelist output buffer size
pub const LPFC_NODELIST_SIZE: c_int = 8192;
pub const LPFC_NODELIST_ENTRY_SIZE: c_int = 120;
// dumpHBASlim output buffer size
pub const LPFC_DUMPHBASLIM_SIZE: c_int = 4096;
// dumpHostSlim output buffer size
pub const LPFC_DUMPHOSTSLIM_SIZE: c_int = 4096;
// dumpSLIqinfo output buffer size
pub const LPFC_DUMPSLIQINFO_SIZE: c_int = 4096;
// hbqinfo output buffer size
pub const LPFC_HBQINFO_SIZE: c_int = 8192;
// hdwqinfo output buffer size
pub const LPFC_HDWQINFO_SIZE: c_int = 8192;
// nvmestat output buffer size
pub const LPFC_NVMESTAT_SIZE: c_int = 8192;
pub const LPFC_IOKTIME_SIZE: c_int = 8192;
pub const LPFC_NVMEIO_TRC_SIZE: c_int = 8192;
// scsistat output buffer size
pub const LPFC_SCSISTAT_SIZE: c_int = 8192;
// Congestion Info Buffer size
pub const LPFC_CGN_BUF_SIZE: c_int = 8192;
pub const LPFC_DEBUG_OUT_LINE_SZ: c_int = 80;
//
// For SLI4 iDiag debugfs diagnostics tool
//
// pciConf
pub const LPFC_PCI_CFG_BROWSE: c_uint = 0xffff;
pub const LPFC_PCI_CFG_RD_CMD_ARG: c_int = 2;
pub const LPFC_PCI_CFG_WR_CMD_ARG: c_int = 3;
pub const LPFC_PCI_CFG_SIZE: c_int = 4096;

pub const IDIAG_PCICFG_WHERE_INDX: c_int = 0;
pub const IDIAG_PCICFG_COUNT_INDX: c_int = 1;
pub const IDIAG_PCICFG_VALUE_INDX: c_int = 2;
// barAcc
pub const LPFC_PCI_BAR_BROWSE: c_uint = 0xffff;
pub const LPFC_PCI_BAR_RD_CMD_ARG: c_int = 3;
pub const LPFC_PCI_BAR_WR_CMD_ARG: c_int = 3;

pub const LPFC_PCI_BAR_RD_BUF_SIZE: c_int = 4096;

pub const IDIAG_BARACC_BAR_NUM_INDX: c_int = 0;
pub const IDIAG_BARACC_OFF_SET_INDX: c_int = 1;
pub const IDIAG_BARACC_ACC_MOD_INDX: c_int = 2;
pub const IDIAG_BARACC_REG_VAL_INDX: c_int = 2;
pub const IDIAG_BARACC_BAR_SZE_INDX: c_int = 3;
pub const IDIAG_BARACC_BAR_0: c_int = 0;
pub const IDIAG_BARACC_BAR_1: c_int = 1;
pub const IDIAG_BARACC_BAR_2: c_int = 2;
pub const SINGLE_WORD: c_int = 1;
// queue info
pub const LPFC_QUE_INFO_GET_BUF_SIZE: c_int = 4096;
// queue acc
pub const LPFC_QUE_ACC_BROWSE: c_uint = 0xffff;
pub const LPFC_QUE_ACC_RD_CMD_ARG: c_int = 4;
pub const LPFC_QUE_ACC_WR_CMD_ARG: c_int = 6;
pub const LPFC_QUE_ACC_BUF_SIZE: c_int = 4096;

pub const LPFC_IDIAG_EQ: c_int = 1;
pub const LPFC_IDIAG_CQ: c_int = 2;
pub const LPFC_IDIAG_MQ: c_int = 3;
pub const LPFC_IDIAG_WQ: c_int = 4;
pub const LPFC_IDIAG_RQ: c_int = 5;
pub const IDIAG_QUEACC_QUETP_INDX: c_int = 0;
pub const IDIAG_QUEACC_QUEID_INDX: c_int = 1;
pub const IDIAG_QUEACC_INDEX_INDX: c_int = 2;
pub const IDIAG_QUEACC_COUNT_INDX: c_int = 3;
pub const IDIAG_QUEACC_OFFST_INDX: c_int = 4;
pub const IDIAG_QUEACC_VALUE_INDX: c_int = 5;
// doorbell register acc
pub const LPFC_DRB_ACC_ALL: c_uint = 0xffff;
pub const LPFC_DRB_ACC_RD_CMD_ARG: c_int = 1;
pub const LPFC_DRB_ACC_WR_CMD_ARG: c_int = 2;
pub const LPFC_DRB_ACC_BUF_SIZE: c_int = 256;
pub const LPFC_DRB_EQ: c_int = 1;
pub const LPFC_DRB_CQ: c_int = 2;
pub const LPFC_DRB_MQ: c_int = 3;
pub const LPFC_DRB_WQ: c_int = 4;
pub const LPFC_DRB_RQ: c_int = 5;
pub const LPFC_DRB_MAX: c_int = 5;
pub const IDIAG_DRBACC_REGID_INDX: c_int = 0;
pub const IDIAG_DRBACC_VALUE_INDX: c_int = 1;
// control register acc
pub const LPFC_CTL_ACC_ALL: c_uint = 0xffff;
pub const LPFC_CTL_ACC_RD_CMD_ARG: c_int = 1;
pub const LPFC_CTL_ACC_WR_CMD_ARG: c_int = 2;
pub const LPFC_CTL_ACC_BUF_SIZE: c_int = 256;
pub const LPFC_CTL_PORT_SEM: c_int = 1;
pub const LPFC_CTL_PORT_STA: c_int = 2;
pub const LPFC_CTL_PORT_CTL: c_int = 3;
pub const LPFC_CTL_PORT_ER1: c_int = 4;
pub const LPFC_CTL_PORT_ER2: c_int = 5;
pub const LPFC_CTL_PDEV_CTL: c_int = 6;
pub const LPFC_CTL_MAX: c_int = 6;
pub const IDIAG_CTLACC_REGID_INDX: c_int = 0;
pub const IDIAG_CTLACC_VALUE_INDX: c_int = 1;
// mailbox access
pub const LPFC_MBX_DMP_ARG: c_int = 4;
pub const LPFC_MBX_ACC_BUF_SIZE: c_int = 512;
pub const LPFC_MBX_ACC_LBUF_SZ: c_int = 128;
pub const LPFC_MBX_DMP_MBX_WORD: c_uint = 0x00000001;
pub const LPFC_MBX_DMP_MBX_BYTE: c_uint = 0x00000002;

pub const LPFC_BSG_DMP_MBX_RD_MBX: c_uint = 0x00000001;
pub const LPFC_BSG_DMP_MBX_RD_BUF: c_uint = 0x00000002;
pub const LPFC_BSG_DMP_MBX_WR_MBX: c_uint = 0x00000004;
pub const LPFC_BSG_DMP_MBX_WR_BUF: c_uint = 0x00000008;

pub const LPFC_MBX_DMP_ALL: c_uint = 0xffff;
pub const LPFC_MBX_ALL_CMD: c_uint = 0xff;
pub const IDIAG_MBXACC_MBCMD_INDX: c_int = 0;
pub const IDIAG_MBXACC_DPMAP_INDX: c_int = 1;
pub const IDIAG_MBXACC_DPCNT_INDX: c_int = 2;
pub const IDIAG_MBXACC_WDCNT_INDX: c_int = 3;
// extents access
pub const LPFC_EXT_ACC_CMD_ARG: c_int = 1;
pub const LPFC_EXT_ACC_BUF_SIZE: c_int = 4096;
pub const LPFC_EXT_ACC_AVAIL: c_uint = 0x1;
pub const LPFC_EXT_ACC_ALLOC: c_uint = 0x2;
pub const LPFC_EXT_ACC_DRIVR: c_uint = 0x4;

pub const IDIAG_EXTACC_EXMAP_INDX: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_debug {
    pub i_private: *mut c_char,
    pub op: c_char,
pub const LPFC_IDIAG_OP_RD: c_int = 1;
pub const LPFC_IDIAG_OP_WR: c_int = 2;
    pub buffer: *mut c_char,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_debugfs_trc {
    pub fmt: *mut c_char,
    pub data1: u32,
    pub data2: u32,
    pub data3: u32,
    pub seq_cnt: u32,
    pub jif: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_debugfs_nvmeio_trc {
    pub fmt: *mut c_char,
    pub data1: u16,
    pub data2: u16,
    pub data3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_idiag_offset {
    pub last_rd: u32,
}

pub const LPFC_IDIAG_CMD_DATA_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_idiag_cmd {
    pub opcode: u32,
pub const LPFC_IDIAG_CMD_PCICFG_RD: c_uint = 0x00000001;
pub const LPFC_IDIAG_CMD_PCICFG_WR: c_uint = 0x00000002;
pub const LPFC_IDIAG_CMD_PCICFG_ST: c_uint = 0x00000003;
pub const LPFC_IDIAG_CMD_PCICFG_CL: c_uint = 0x00000004;
pub const LPFC_IDIAG_CMD_BARACC_RD: c_uint = 0x00000008;
pub const LPFC_IDIAG_CMD_BARACC_WR: c_uint = 0x00000009;
pub const LPFC_IDIAG_CMD_BARACC_ST: c_uint = 0x0000000a;
pub const LPFC_IDIAG_CMD_BARACC_CL: c_uint = 0x0000000b;
pub const LPFC_IDIAG_CMD_QUEACC_RD: c_uint = 0x00000011;
pub const LPFC_IDIAG_CMD_QUEACC_WR: c_uint = 0x00000012;
pub const LPFC_IDIAG_CMD_QUEACC_ST: c_uint = 0x00000013;
pub const LPFC_IDIAG_CMD_QUEACC_CL: c_uint = 0x00000014;
pub const LPFC_IDIAG_CMD_DRBACC_RD: c_uint = 0x00000021;
pub const LPFC_IDIAG_CMD_DRBACC_WR: c_uint = 0x00000022;
pub const LPFC_IDIAG_CMD_DRBACC_ST: c_uint = 0x00000023;
pub const LPFC_IDIAG_CMD_DRBACC_CL: c_uint = 0x00000024;
pub const LPFC_IDIAG_CMD_CTLACC_RD: c_uint = 0x00000031;
pub const LPFC_IDIAG_CMD_CTLACC_WR: c_uint = 0x00000032;
pub const LPFC_IDIAG_CMD_CTLACC_ST: c_uint = 0x00000033;
pub const LPFC_IDIAG_CMD_CTLACC_CL: c_uint = 0x00000034;
pub const LPFC_IDIAG_CMD_MBXACC_DP: c_uint = 0x00000041;
pub const LPFC_IDIAG_BSG_MBXACC_DP: c_uint = 0x00000042;
pub const LPFC_IDIAG_CMD_EXTACC_RD: c_uint = 0x00000051;
    pub data: [u32; LPFC_IDIAG_CMD_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_idiag {
    pub active: u32,
    pub cmd: lpfc_idiag_cmd,
    pub offset: lpfc_idiag_offset,
    pub ptr_private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rx_monitor_debug {
    pub i_private: *mut c_char,
    pub buffer: *mut c_char,
}

// multixripool output buffer size
pub const LPFC_DUMP_MULTIXRIPOOL_SIZE: c_int = 8192;
// Mask for discovery_trace
pub const LPFC_DISC_TRC_ELS_CMD: c_uint = 0x1	/* Trace ELS commands */;
pub const LPFC_DISC_TRC_ELS_RSP: c_uint = 0x2	/* Trace ELS response */;
pub const LPFC_DISC_TRC_ELS_UNSOL: c_uint = 0x4	/* Trace ELS rcv'ed   */;
pub const LPFC_DISC_TRC_ELS_ALL: c_uint = 0x7	/* Trace ELS */;
pub const LPFC_DISC_TRC_MBOX_VPORT: c_uint = 0x8	/* Trace vport MBOXs */;
pub const LPFC_DISC_TRC_MBOX: c_uint = 0x10	/* Trace other MBOXs */;
pub const LPFC_DISC_TRC_MBOX_ALL: c_uint = 0x18	/* Trace all MBOXs */;
pub const LPFC_DISC_TRC_CT: c_uint = 0x20	/* Trace disc CT requests */;
pub const LPFC_DISC_TRC_DSM: c_uint = 0x40    /* Trace DSM events */;
pub const LPFC_DISC_TRC_RPORT: c_uint = 0x80    /* Trace rport events */;
pub const LPFC_DISC_TRC_NODE: c_uint = 0x100   /* Trace ndlp state changes */;
pub const LPFC_DISC_TRC_DISCOVERY: c_uint = 0xef    /* common mask for general;
// discovery

//
// Driver debug utility routines outside of debugfs. The debug utility
// routines implemented here is intended to be used in the instrumented
// debug driver for debugging host or port issues.
//
// lpfc_debug_dump_qe - dump an specific entry from a queue
// @q: Pointer to the queue descriptor.
// @idx: Index to the entry on the queue.
//
// This function dumps an entry indexed by @idx from a queue specified by the
// queue descriptor @q.
//
// sanity checks
//
// lpfc_debug_dump_q - dump all entries from an specific queue
// @q: Pointer to the queue descriptor.
//
// This function dumps all entries from a queue specified by the queue
// descriptor @q.
//
// sanity check
//
// lpfc_debug_dump_wq - dump all entries from the fcp or nvme work queue
// @phba: Pointer to HBA context object.
// @wqidx: Index to a FCP or NVME work queue.
//
// This function dumps all entries from a FCP or NVME work queue specified
// by the wqidx.
//
// lpfc_debug_dump_cq - dump all entries from a fcp or nvme work queue's
// cmpl queue
// @phba: Pointer to HBA context object.
// @wqidx: Index to a FCP work queue.
//
// This function dumps all entries from a FCP or NVME completion queue
// which is associated to the work queue specified by the @wqidx.
//
// io wq and cq are 1:1, thus same indexes
//
// lpfc_debug_dump_hba_eq - dump all entries from a fcp work queue's evt queue
// @phba: Pointer to HBA context object.
// @fcp_wqidx: Index to a FCP work queue.
//
// This function dumps all entries from a FCP event queue which is
// associated to the FCP work queue specified by the @fcp_wqidx.
//
// lpfc_debug_dump_dat_rq - dump all entries from the receive data queue
// @phba: Pointer to HBA context object.
//
// This function dumps all entries from the receive data queue.
//
// lpfc_debug_dump_hdr_rq - dump all entries from the receive header queue
// @phba: Pointer to HBA context object.
//
// This function dumps all entries from the receive header queue.
//
// lpfc_debug_dump_wq_by_id - dump all entries from a work queue by queue id
// @phba: Pointer to HBA context object.
// @qid: Work queue identifier.
//
// This function dumps all entries from a work queue identified by the queue
// identifier.
//
// lpfc_debug_dump_mq_by_id - dump all entries from a mbox queue by queue id
// @phba: Pointer to HBA context object.
// @qid: Mbox work queue identifier.
//
// This function dumps all entries from a mbox work queue identified by the
// queue identifier.
//
// lpfc_debug_dump_rq_by_id - dump all entries from a receive queue by queue id
// @phba: Pointer to HBA context object.
// @qid: Receive queue identifier.
//
// This function dumps all entries from a receive queue identified by the
// queue identifier.
//
// lpfc_debug_dump_cq_by_id - dump all entries from a cmpl queue by queue id
// @phba: Pointer to HBA context object.
// @qid: Complete queue identifier.
//
// This function dumps all entries from a complete queue identified by the
// queue identifier.
//
// lpfc_debug_dump_eq_by_id - dump all entries from an event queue by queue id
// @phba: Pointer to HBA context object.
// @qid: Complete queue identifier.
//
// This function dumps all entries from an event queue identified by the
// queue identifier.
//
extern "C" {
    pub fn lpfc_debug_dump_all_queues(: *mut lpfc_hba);
}
