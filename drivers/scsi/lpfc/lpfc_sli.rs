//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_sli.h
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
// Copyright (C) 2017-2024 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2016 Emulex.  All rights reserved.
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

// Macro flag: #define CONFIG_SCSI_LPFC_DEBUG_FS

// forward declaration for LPFC_IOCB_t's use
// Define the context types that SLI handles for abort and sums.
// Enumeration to describe the thread lock context.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_mbox_ctx {
    MBOX_THD_UNLOCKED,
    MBOX_THD_LOCKED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lpfc_vmid_tag {
    pub app_id: u32,
    pub cs_ctl_vmid: u8,
    pub /: *mut *mut *mut lpfc_vmid_context vmid_context; / UVEM context information,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cq_event {
    pub list: list_head,
    pub hdwq: u16,
    pub mcqe_cmpl: lpfc_mcqe,
    pub acqe_link: lpfc_acqe_link,
    pub acqe_fip: lpfc_acqe_fip,
    pub acqe_dcbx: lpfc_acqe_dcbx,
    pub acqe_grp5: lpfc_acqe_grp5,
    pub acqe_fc: lpfc_acqe_fc_la,
    pub acqe_sli: lpfc_acqe_sli,
    pub rcqe_cmpl: lpfc_rcqe,
    pub wcqe_axri: sli4_wcqe_xri_aborted,
    pub wcqe_cmpl: lpfc_wcqe_complete,
    pub cqe: },
}

// This structure is used to handle IOCB requests / responses
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_iocbq {
// lpfc_iocbqs are used in double linked lists
    pub list: list_head,
    pub clist: list_head,
    pub dlist: list_head,
    pub /: *mut *mut uint16_t iotag; / pre-assigned IO tag,
    pub /: *mut *mut uint16_t sli4_lxritag; / logical pre-assigned XRI.,
    pub /: *mut *mut uint16_t sli4_xritag; / pre-assigned XRI, (OXID) tag.,
    pub /: *mut *mut uint16_t hba_wqidx; / index to HBA work queue,
    pub cq_event: lpfc_cq_event,
    pub isr_timestamp: u64,
    pub /: *mut *mut lpfc_wqe128 wqe; / SLI-4,
    pub /: *mut *mut IOCB_t iocb; / SLI-3,
    pub /: *mut *mut lpfc_wcqe_complete wcqe_cmpl; / WQE cmpl,
    pub /: *mut *mut u32 unsol_rcv_len; / Receive len in usol path,
// Pack the u8's together and make them module-4.
    pub /: *mut *mut u8 num_bdes; / Number of BDEs,
    pub /: *mut *mut u8 abort_bls; / ABTS by initiator or responder,
    pub /: *mut *mut u8 abort_rctl; / ACC or RJT flag,
    pub /: *mut *mut u8 priority; / OAS priority,
    pub /: *mut *mut u8 retry; / retry counter for IOCB cmd - if needed,
    pub /: *mut *mut u8 rsvd1; / Pad for u32,
    pub /: *mut *mut u8 rsvd2; / Pad for u32,
    pub /: *mut *mut u8 rsvd3; / Pad for u32,
    pub cmd_flag: u32,

pub const LPFC_IO_FABRIC: c_uint = 0x10	/* Iocb send using fabric scheduler */;
pub const LPFC_DELAY_MEM_FREE: c_uint = 0x20    /* Defer free'ing of FC data */;
pub const LPFC_EXCHANGE_BUSY: c_uint = 0x40    /* SLI4 hba reported XB in response */;
pub const LPFC_USE_FCPWQIDX: c_uint = 0x80    /* Submit to specified FCPWQ index */;
pub const DSS_SECURITY_OP: c_uint = 0x100	/* security IO */;
pub const LPFC_IO_ON_TXCMPLQ: c_uint = 0x200	/* The IO is still on the TXCMPLQ */;
pub const LPFC_IO_DIF_PASS: c_uint = 0x400	/* T10 DIF IO pass-thru prot */;
pub const LPFC_IO_DIF_STRIP: c_uint = 0x800	/* T10 DIF IO strip prot */;
pub const LPFC_IO_DIF_INSERT: c_uint = 0x1000	/* T10 DIF IO insert prot */;
pub const LPFC_IO_CMD_OUTSTANDING: c_uint = 0x2000 /* timeout handler abort window */;
pub const LPFC_FIP_ELS_ID_MASK: c_uint = 0xc000	/* ELS_ID range 0-3, non-shifted mask */;
pub const LPFC_FIP_ELS_ID_SHIFT: c_int = 14;
pub const LPFC_IO_OAS: c_uint = 0x10000 /* OAS FCP IO */;
pub const LPFC_IO_FOF: c_uint = 0x20000 /* FOF FCP IO */;
pub const LPFC_IO_LOOPBACK: c_uint = 0x40000 /* Loopback IO */;
pub const LPFC_PRLI_NVME_REQ: c_uint = 0x80000 /* This is an NVME PRLI. */;
pub const LPFC_PRLI_FCP_REQ: c_uint = 0x100000 /* This is an NVME PRLI. */;
pub const LPFC_IO_NVME: c_uint = 0x200000 /* NVME FCP command */;
pub const LPFC_IO_NVME_LS: c_uint = 0x400000 /* NVME LS command */;
pub const LPFC_IO_NVMET: c_uint = 0x800000 /* NVMET command */;
pub const LPFC_IO_VMID: c_uint = 0x1000000 /* VMID tagged IO */;
pub const LPFC_IO_CMF: c_uint = 0x4000000 /* CMF command */;
    pub /: *mut *mut uint32_t drvrTimeout; / driver timeout in seconds,
    pub /: *mut *mut *mut lpfc_vport vport;/ virtual port pointer,
    pub cmd_dmabuf: *mut lpfc_dmabuf,
    pub rsp_dmabuf: *mut lpfc_dmabuf,
    pub bpl_dmabuf: *mut lpfc_dmabuf,
    pub /: *mut *mut uint32_t event_tag; / LA Event tag,
    pub wait_queue: *mut wait_queue_head_t,
    pub mbox: *mut lpfcMboxq,
    pub rrq: *mut lpfc_node_rrq,
    pub nvme_lsreq: *mut nvmefc_ls_req,
    pub axchg: *mut lpfc_async_xchg_ctx,
    pub dd_data: *mut bsg_job_data,
    pub context_un: },
    pub io_buf: *mut lpfc_io_buf,
    pub rsp_iocb: *mut lpfc_iocbq,
    pub ndlp: *mut lpfc_nodelist,
    pub vmid_tag: lpfc_vmid_tag,
    pub rsp): *mut lpfc_iocbq,
    pub rsp): *mut lpfc_iocbq,
    pub rsp): *mut lpfc_iocbq,
}

pub const IOCB_SUCCESS: c_int = 0;
pub const IOCB_BUSY: c_int = 1;
pub const IOCB_ERROR: c_int = 2;
pub const IOCB_TIMEDOUT: c_int = 3;
pub const IOCB_ABORTED: c_int = 4;
pub const IOCB_ABORTING: c_int = 5;
pub const IOCB_NORESOURCE: c_int = 6;

pub const WQE_SUCCESS: c_int = 0;
pub const WQE_BUSY: c_int = 1;
pub const WQE_ERROR: c_int = 2;
pub const WQE_TIMEDOUT: c_int = 3;
pub const WQE_ABORTED: c_int = 4;
pub const WQE_ABORTING: c_int = 5;
pub const WQE_NORESOURCE: c_int = 6;
pub const LPFC_MBX_WAKE: c_int = 1;
pub const LPFC_MBX_IMED_UNREG: c_int = 2;
// MBOXQs are used in single linked lists
// cmds.  Not a generic pointer.
// Use for storing virtual address.
//
// Pointers that are seldom used during mbox execution, but require
// a saved context.
//
// and
// bsg_issue_mbox_ext_handle_job
//
// lpfc_mbx_cmpl_resume_rpi
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_ring_mask {
    pub /: *mut *mut uint8_t profile; / profile associated with ring,
    pub /: *mut *mut uint8_t rctl; / rctl / type pair configured for ring,
    pub /: *mut *mut uint8_t type; / rctl / type pair configured for ring,
    pub rsvd: u8,
// rcv'd unsol event
    pub ): *mut lpfc_iocbq,
}

// Structure used to hold SLI statistical counters and info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_ring_stat {
    pub /: *mut *mut uint64_t iocb_event; / IOCB event counters,
    pub /: *mut *mut uint64_t iocb_cmd; / IOCB cmd issued,
    pub /: *mut *mut uint64_t iocb_rsp; / IOCB rsp received,
    pub /: *mut *mut uint64_t iocb_cmd_delay; / IOCB cmd ring delay,
    pub /: *mut *mut uint64_t iocb_cmd_full; / IOCB cmd ring full,
    pub /: *mut *mut uint64_t iocb_cmd_empty; / IOCB cmd ring is now empty,
    pub /: *mut *mut uint64_t iocb_rsp_full; / IOCB rsp ring full,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli3_ring {
    pub /: *mut *mut uint32_t local_getidx; / last available cmd index (from cmdGetInx),
    pub /: *mut *mut uint32_t next_cmdidx; / next_cmd index,
    pub /: *mut *mut uint32_t rspidx; / current index in response ring,
    pub /: *mut *mut uint32_t cmdidx; / current index in command ring,
    pub /: *mut *mut uint16_t numCiocb; / number of command iocb's per ring,
    pub /: *mut *mut uint16_t numRiocb; / number of rsp iocb's per ring,
    pub /: *mut *mut uint16_t sizeCiocb; / Size of command iocb's in this ring,
    pub /: *mut *mut uint16_t sizeRiocb; / Size of response iocb's in this ring,
    pub /: *mut *mut *mut uint32_t cmdringaddr; / virtual address for cmd rings,
    pub /: *mut *mut *mut uint32_t rspringaddr; / virtual address for rsp rings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_ring {
    pub /: *mut *mut *mut lpfc_queue wqp; / Pointer to associated WQ,
}

// Structure used to hold SLI ring information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_ring {
    pub /: *mut *mut uint16_t flag; / ring flags,
pub const LPFC_DEFERRED_RING_EVENT: c_uint = 0x001	/* Deferred processing a ring event */;
pub const LPFC_CALL_RING_AVAILABLE: c_uint = 0x002	/* indicates cmd was full */;
pub const LPFC_STOP_IOCB_EVENT: c_uint = 0x020	/* Stop processing IOCB cmds event */;
    pub /: *mut *mut uint16_t abtsiotag; / tracks next iotag to use for ABTS,
    pub rsvd: u8,
    pub /: *mut *mut uint8_t ringno; / ring number,
    pub /: *mut *mut spinlock_t ring_lock; / lock for issuing commands,
    pub /: *mut *mut uint32_t fast_iotag; / max fastlookup based iotag,
    pub /: *mut *mut uint32_t iotag_ctr; / keeps track of the next iotag to use,
    pub /: *mut *mut uint32_t iotag_max; / max iotag value to use,
    pub txq: list_head,
    pub /: *mut *mut uint16_t txq_cnt; / current length of queue,
    pub /: *mut *mut uint16_t txq_max; / max length,
    pub txcmplq: list_head,
    pub /: *mut *mut uint16_t txcmplq_cnt; / current length of queue,
    pub /: *mut *mut uint16_t txcmplq_max; / max length,
    pub /: *mut *mut uint32_t missbufcnt; / keep track of buffers to post,
    pub postbufq: list_head,
    pub /: *mut *mut uint16_t postbufq_cnt; / current length of queue,
    pub /: *mut *mut uint16_t postbufq_max; / max length,
    pub iocb_continueq: list_head,
    pub /: *mut *mut uint16_t iocb_continueq_cnt; / current length of queue,
    pub /: *mut *mut uint16_t iocb_continueq_max; / max length,
    pub iocb_continue_saveq: list_head,
    pub prt: [lpfc_sli_ring_mask; LPFC_MAX_RING_MASK],
    pub /: *mut *mut uint32_t num_mask; / number of mask entries in prt array,
    pub ): *mut *mut lpfc_sli_ring , lpfc_iocbq,
    pub /: *mut *mut lpfc_sli_ring_stat stats; / SLI statistical info,
// cmd ring available
    pub ): *mut lpfc_sli_ring,
    pub sli3: lpfc_sli3_ring,
    pub sli4: lpfc_sli4_ring,
    pub sli: },
}

// Structure used for configuring rings to a specific profile or rctl / type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_hbq_init {
    pub /: *mut *mut uint32_t rn; / Receive buffer notification,
    pub /: *mut *mut uint32_t entry_count; / max # of entries in HBQ,
    pub /: *mut *mut uint32_t headerLen; / 0 if not profile 4 or 5,
    pub /: *mut *mut uint32_t logEntry; / Set to 1 if this HBQ used for LogEntry,
    pub /: *mut *mut uint32_t profile; / Selection profile 0=all, 7=logentry,
    pub Ring0=b0001,: *mut *mut uint32_t ring_mask; / Binds HBQ to a ring e.g.,
// ring2=b0100
    pub /: *mut *mut uint32_t hbq_index; / index of this hbq in ring .HBQs[],
    pub seqlenoff: u32,
    pub maxlen: u32,
    pub seqlenbcnt: u32,
    pub cmdcodeoff: u32,
    pub cmdmatch: [u32; 8],
    pub /: *mut *mut uint32_t mask_count; / number of mask entries in prt array,
    pub hbqMasks: [hbq_mask; 6],
// Non-config rings fields to keep track of buffer allocations
    pub /: *mut *mut uint32_t buffer_count; / number of buffers allocated,
    pub /: *mut *mut uint32_t init_count; / number to allocate when initialized,
    pub /: *mut *mut uint32_t add_count; / number to allocate when starved,
// Structure used to hold SLI statistical counters and info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_stat {
    pub /: *mut *mut uint64_t mbox_stat_err; / Mbox cmds completed status error,
    pub /: *mut *mut uint64_t mbox_cmd; / Mailbox commands issued,
    pub /: *mut *mut uint64_t sli_intr; / Count of Host Attention interrupts,
    pub /: *mut *mut uint64_t sli_prev_intr; / Previous cnt of Host Attention interrupts,
    pub /: *mut *mut uint64_t sli_ips; / Host Attention interrupts per sec,
    pub /: *mut *mut uint32_t err_attn_event; / Error Attn event counters,
    pub /: *mut *mut uint32_t link_event; / Link event counters,
    pub /: *mut *mut uint32_t mbox_event; / Mailbox event counters,
    pub /: *mut *mut uint32_t mbox_busy; / Mailbox cmd busy,
}

// Structure to store link status values when port stats are reset
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_lnk_stat {
    pub link_failure_count: u32,
    pub loss_of_sync_count: u32,
    pub loss_of_signal_count: u32,
    pub prim_seq_protocol_err_count: u32,
    pub invalid_tx_word_count: u32,
    pub invalid_crc_count: u32,
    pub error_frames: u32,
    pub link_events: u32,
}

// Structure used to hold SLI information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli {
    pub num_rings: u32,
    pub sli_flag: u32,
// Additional sli_flags
pub const LPFC_SLI_MBOX_ACTIVE: c_uint = 0x100	/* HBA mailbox is currently active */;
pub const LPFC_SLI_ACTIVE: c_uint = 0x200	/* SLI in firmware is active */;
pub const LPFC_PROCESS_LA: c_uint = 0x400	/* Able to process link attention */;
pub const LPFC_BLOCK_MGMT_IO: c_uint = 0x800	/* Don't allow mgmt mbx or iocb cmds */;
pub const LPFC_SLI_ASYNC_MBX_BLK: c_uint = 0x2000 /* Async mailbox is blocked */;
pub const LPFC_SLI_SUPPRESS_RSP: c_uint = 0x4000 /* Suppress RSP feature is supported */;
pub const LPFC_SLI_USE_EQDR: c_uint = 0x8000 /* EQ Delay Register is supported */;
pub const LPFC_QUEUE_FREE_INIT: c_uint = 0x10000 /* Queue freeing is in progress */;
pub const LPFC_QUEUE_FREE_WAIT: c_uint = 0x20000 /* Hold Queue free as it is being;
// used outside worker thread
//
    pub sli3_ring: *mut lpfc_sli_ring,
    pub /: *mut *mut lpfc_sli_stat slistat; / SLI statistical info,
    pub mboxq: list_head,
    pub /: *mut *mut uint16_t mboxq_cnt; / current length of queue,
    pub /: *mut *mut uint16_t mboxq_max; / max length,
    pub /: *mut *mut *mut LPFC_MBOXQ_t mbox_active; / active mboxq information,
    pub mboxq_cmpl: list_head,
    pub mbox: *mut *mut timer_list mbox_tmo; / Hold clk to timeout active,
pub const LPFC_IOCBQ_LOOKUP_INCREMENT: c_int = 1024;
    pub /: *mut *mut *mut *mut lpfc_iocbq  iocbq_lookup; / array to lookup IOCB by IOTAG,
    pub /: *mut *mut size_t iocbq_lookup_len; / current lengs of the array,
    pub /: *mut *mut uint16_t last_iotag; / last allocated IOTAG,
    pub /: *mut *mut time64_t stats_start; / in seconds,
    pub lnk_stat_offsets: lpfc_lnk_stat,
}

// Timeout for normal outstanding mbox command (Seconds)
pub const LPFC_MBOX_TMO: c_int = 30;
// Timeout for non-flash-based outstanding sli_config mbox command (Seconds)
pub const LPFC_MBOX_SLI4_CONFIG_TMO: c_int = 60;
// Timeout for flash-based outstanding sli_config mbox command (Seconds)
pub const LPFC_MBOX_SLI4_CONFIG_EXTENDED_TMO: c_int = 300;
// Timeout for other flash-based outstanding mbox command (Seconds)
pub const LPFC_MBOX_TMO_FLASH_CMD: c_int = 300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_io_buf {
// Common fields
    pub list: list_head,
    pub data: *mut c_void,
    pub dma_handle: dma_addr_t,
    pub dma_phys_sgl: dma_addr_t,
    pub /: *mut *mut *mut sli4_sge dma_sgl; / initial segment chunk,
// linked list of extra sli4_hybrid_sge
    pub dma_sgl_xtra_list: list_head,
// list head for fcp_cmd_rsp buf
    pub dma_cmd_rsp_list: list_head,
    pub cur_iocbq: lpfc_iocbq,
    pub hdwq: *mut lpfc_sli4_hdw_queue,
    pub hdwq_no: u16,
    pub cpu: u16,
    pub ndlp: *mut lpfc_nodelist,
    pub timeout: u32,
    pub flags: u16,
pub const LPFC_SBUF_XBUSY: c_uint = 0x1	/* SLI4 hba reported XB on WCQE cmpl */;
pub const LPFC_SBUF_BUMP_QDEPTH: c_uint = 0x2	/* bumped queue depth counter */;
// External DIF device IO conversions
pub const LPFC_SBUF_NORMAL_DIF: c_uint = 0x4	/* normal mode to insert/strip */;
pub const LPFC_SBUF_PASS_DIF: c_uint = 0x8	/* insert/strip mode to passthru */;
pub const LPFC_SBUF_NOT_POSTED: c_uint = 0x10    /* SGL failed post to FW. */;
    pub /: *mut *mut uint16_t status; / From IOCB Word 7- ulpStatus,
    pub /: *mut *mut uint32_t result; / From IOCB Word 4.,
    pub by: *mut *mut uint32_t seg_cnt; / Number of scatter-gather segments returned,
// dma_map_sg.  The driver needs this for calls
// to dma_unmap_sg.
//
    pub start_time: c_ulong,
    pub /: *mut *mut spinlock_t buf_lock; / lock used in case of simultaneous abort,
    pub /: *mut *mut bool expedite; / this is an expedite io_buf,
// SCSI specific fields
    pub pCmd: *mut scsi_cmnd,
    pub rdata: *mut lpfc_rport_data,
    pub for: *mut *mut uint32_t prot_seg_cnt; / seg_cnt's counterpart,
// protection data
//
// data and dma_handle are the kernel virtual and bus
// address of the dma-able buffer containing the
// fcp_cmd, fcp_rsp and a scatter gather bde list that
// supports the sg_tablesize value.
//
    pub fcp_cmnd: *mut fcp_cmnd,
    pub fcp_rsp: *mut fcp_rsp,
    pub waitq: *mut wait_queue_head_t,

// Used to restore any changes to protection data for
// error injection
//
    pub prot_data_segment: *mut c_void,
    pub prot_data: u32,
    pub prot_data_type: u32,
pub const LPFC_INJERR_REFTAG: c_int = 1;
pub const LPFC_INJERR_APPTAG: c_int = 2;
pub const LPFC_INJERR_GUARD: c_int = 3;

}

// NVME specific fields

