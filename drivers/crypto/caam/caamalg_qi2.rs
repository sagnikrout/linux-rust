//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/caamalg_qi2.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2015-2016 Freescale Semiconductor Inc.
// Copyright 2017-2018 NXP
//

pub const DPAA2_CAAM_STORE_SIZE: c_int = 16;
// NAPI weight *must* be a multiple of the store size.
pub const DPAA2_CAAM_NAPI_WEIGHT: c_int = 512;
// The congestion entrance threshold was chosen so that on LS2088
// we support the maximum throughput for the available memory
//

//
// dpaa2_caam_priv - driver private data
// @dpseci_id: DPSECI object unique ID
// @major_ver: DPSECI major version
// @minor_ver: DPSECI minor version
// @dpseci_attr: DPSECI attributes
// @sec_attr: SEC engine attributes
// @rx_queue_attr: array of Rx queue attributes
// @tx_queue_attr: array of Tx queue attributes
// @cscn_mem: pointer to memory region containing the congestion SCN
// it's size is larger than to accommodate alignment
// @cscn_dma: dma address used by the QMAN to write CSCN messages
// @dev: device associated with the DPSECI object
// @mc_io: pointer to MC portal's I/O object
// @domain: IOMMU domain
// @ppriv: per CPU pointers to privata data
// @clean_mask: CPU mask of CPUs that have allocated netdevs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_caam_priv {
    pub dpsec_id: c_int,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub dpseci_attr: dpseci_attr,
    pub sec_attr: dpseci_sec_attr,
    pub rx_queue_attr: [dpseci_rx_queue_attr; DPSECI_MAX_QUEUE_NUM],
    pub tx_queue_attr: [dpseci_tx_queue_attr; DPSECI_MAX_QUEUE_NUM],
    pub num_pairs: c_int,
// congestion
    pub cscn_mem: *mut c_void,
    pub cscn_dma: dma_addr_t,
    pub dev: *mut device,
    pub mc_io: *mut fsl_mc_io,
    pub domain: *mut iommu_domain,
    pub ppriv: *mut dpaa2_caam_priv_per_cpu __percpu,
    pub dfs_root: *mut dentry,
    pub clean_mask: cpumask_var_t,
}

//
// dpaa2_caam_priv_per_cpu - per CPU private data
// @napi: napi structure
// @net_dev: netdev used by napi
// @req_fqid: (virtual) request (Tx / enqueue) FQID
// @rsp_fqid: (virtual) response (Rx / dequeue) FQID
// @prio: internal queue number - index for dpaa2_caam_priv.*_queue_attr
// @nctx: notification context of response FQ
// @store: where dequeued frames are stored
// @priv: backpointer to dpaa2_caam_priv
// @dpio: portal used for data path operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_caam_priv_per_cpu {
    pub napi: napi_struct,
    pub net_dev: *mut net_device,
    pub req_fqid: c_int,
    pub rsp_fqid: c_int,
    pub prio: c_int,
    pub nctx: dpaa2_io_notification_ctx,
    pub store: *mut dpaa2_io_store,
    pub priv: *mut dpaa2_caam_priv,
    pub dpio: *mut dpaa2_io,
}

// Length of a single buffer in the QI driver memory cache
pub const CAAM_QI_MEMCACHE_SIZE: c_int = 512;
//
// aead_edesc - s/w-extended aead descriptor
// @src_nents: number of segments in input scatterlist
// @dst_nents: number of segments in output scatterlist
// @iv_dma: dma address of iv for checking continuity and link table
// @qm_sg_bytes: length of dma mapped h/w link table
// @qm_sg_dma: bus physical mapped address of h/w link table
// @assoclen: associated data length, in CAAM endianness
// @assoclen_dma: bus physical mapped address of req->assoclen
// @sgt: the h/w link table, followed by IV
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_edesc {
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub iv_dma: dma_addr_t,
    pub qm_sg_bytes: c_int,
    pub qm_sg_dma: dma_addr_t,
    pub assoclen: c_uint,
    pub assoclen_dma: dma_addr_t,
    pub sgt: [dpaa2_sg_entry; ],
}

//
// skcipher_edesc - s/w-extended skcipher descriptor
// @src_nents: number of segments in input scatterlist
// @dst_nents: number of segments in output scatterlist
// @iv_dma: dma address of iv for checking continuity and link table
// @qm_sg_bytes: length of dma mapped qm_sg space
// @qm_sg_dma: I/O virtual address of h/w link table
// @sgt: the h/w link table, followed by IV
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skcipher_edesc {
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub iv_dma: dma_addr_t,
    pub qm_sg_bytes: c_int,
    pub qm_sg_dma: dma_addr_t,
    pub sgt: [dpaa2_sg_entry; ],
}

//
// ahash_edesc - s/w-extended ahash descriptor
// @qm_sg_dma: I/O virtual address of h/w link table
// @src_nents: number of segments in input scatterlist
// @qm_sg_bytes: length of dma mapped qm_sg space
// @sgt: pointer to h/w link table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahash_edesc {
    pub qm_sg_dma: dma_addr_t,
    pub src_nents: c_int,
    pub qm_sg_bytes: c_int,
    pub sgt: [dpaa2_sg_entry; ],
}

//
// caam_flc - Flow Context (FLC)
// @flc: Flow Context options
// @sh_desc: Shared Descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_flc {
    pub flc: [u32; 16],
    pub sh_desc: [u32; MAX_SDLEN],
    pub __aligned(CRYPTO_DMA_ALIGN): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum optype {
    ENCRYPT = 0,
    DECRYPT,
    NUM_OP
}

//
// caam_request - the request structure the driver application should fill while
// submitting a job to driver.
// @fd_flt: Frame list table defining input and output
// fd_flt[0] - FLE pointing to output buffer
// fd_flt[1] - FLE pointing to input buffer
// @fd_flt_dma: DMA address for the frame list table
// @flc: Flow Context
// @flc_dma: I/O virtual address of Flow Context
// @cbk: Callback function to invoke when job is completed
// @ctx: arbit context attached with request by the application
// @edesc: extended descriptor; points to one of {skcipher,aead}_edesc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_request {
    pub __aligned(CRYPTO_DMA_ALIGN): dpaa2_fl_entry fd_flt[2],
    pub fd_flt_dma: dma_addr_t,
    pub flc: *mut caam_flc,
    pub flc_dma: dma_addr_t,
    pub err): *mut *mut *mut void (cbk)(void ctx, u32,
    pub ctx: *mut c_void,
    pub edesc: *mut c_void,
    pub fallback_req: skcipher_request,
}

//
// dpaa2_caam_enqueue() - enqueue a crypto request
// @dev: device associated with the DPSECI object
// @req: pointer to caam_request
//
extern "C" {
    pub fn dpaa2_caam_enqueue(dev: *mut device, req: *mut caam_request) -> c_int;
}
