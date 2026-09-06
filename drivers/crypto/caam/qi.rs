//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/qi.h
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
// Public definitions for the CAAM/QI (Queue Interface) backend.
//
// Copyright 2013-2016 Freescale Semiconductor, Inc.
// Copyright 2016-2017, 2020 NXP
//

// Length of a single buffer in the QI driver memory cache
pub const CAAM_QI_MEMCACHE_SIZE: c_int = 768;
//
// This is the request structure the driver application should fill while
// submitting a job to driver.
//
// caam_qi_cbk - application's callback function invoked by the driver when the
// request has been successfully processed.
// @drv_req: original request that was submitted
// @status: completion status of request (0 - success, non-zero - error code)
//
extern "C" {
    pub fn void(drv_req: *mut *mut caam_qi_cbk)(struct caam_drv_req, status: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum optype {
    ENCRYPT,
    DECRYPT,
    NUM_OP
}

//
// caam_drv_ctx - CAAM/QI backend driver context
//
// The jobs are processed by the driver against a driver context.
// With every cryptographic context, a driver context is attached.
// The driver context contains data for private use by driver.
// For the applications, this is an opaque structure.
//
// @prehdr: preheader placed before shrd desc
// @sh_desc: shared descriptor
// @context_a: shared descriptor dma address
// @req_fq: to-CAAM request frame queue
// @rsp_fq: from-CAAM response frame queue
// @refcnt: reference counter incremented for each frame enqueued in to-CAAM FQ
// @cpu: cpu on which to receive CAAM response
// @op_type: operation type
// @qidev: device pointer for CAAM/QI backend
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_drv_ctx {
    pub prehdr: [u32; 2],
    pub sh_desc: [u32; MAX_SDLEN],
    pub __aligned(CRYPTO_DMA_ALIGN): },
    pub context_a: dma_addr_t,
    pub req_fq: *mut qman_fq,
    pub rsp_fq: *mut qman_fq,
    pub refcnt: refcount_t,
    pub cpu: c_int,
    pub op_type: optype,
    pub qidev: *mut device,
}

//
// caam_drv_req - The request structure the driver application should fill while
// submitting a job to driver.
// @fd_sgt: QMan S/G pointing to output (fd_sgt[0]) and input (fd_sgt[1])
// buffers.
// @cbk: callback function to invoke when job is completed
// @app_ctx: arbitrary context attached with request by the application
//
// The fields mentioned below should not be used by application.
// These are for private use by driver.
//
// @hdr__: linked list header to maintain list of outstanding requests to CAAM
// @hwaddr: DMA address for the S/G table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_drv_req {
    pub fd_sgt: [qm_sg_entry; 2],
    pub drv_ctx: *mut caam_drv_ctx,
    pub cbk: caam_qi_cbk,
    pub app_ctx: *mut c_void,
    pub __aligned(CRYPTO_DMA_ALIGN): },
//
// caam_drv_ctx_init - Initialise a CAAM/QI driver context
//
// A CAAM/QI driver context must be attached with each cryptographic context.
// This function allocates memory for CAAM/QI context and returns a handle to
// the application. This handle must be submitted along with each enqueue
// request to the driver by the application.
//
// @cpu: CPU where the application prefers to the driver to receive CAAM
// responses. The request completion callback would be issued from this
// CPU.
// @sh_desc: shared descriptor pointer to be attached with CAAM/QI driver
// context.
//
// Returns a driver context on success or negative error code on failure.
//
    pub sh_desc): *mut u32,
//
// caam_qi_enqueue - Submit a request to QI backend driver.
//
// The request structure must be properly filled as described above.
//
// @qidev: device pointer for QI backend
// @req: CAAM QI request structure
//
// Returns 0 on success or negative error code on failure.
//
    pub req): *mut *mut int caam_qi_enqueue(struct device qidev, struct caam_drv_req,
//
// caam_drv_ctx_busy - Check if there are too many jobs pending with CAAM
// or too many CAAM responses are pending to be processed.
// @drv_ctx: driver context for which job is to be submitted
//
// Returns caam congestion status 'true/false'
//
    pub drv_ctx): *mut bool caam_drv_ctx_busy(struct caam_drv_ctx,
//
// caam_drv_ctx_update - Update QI driver context
//
// Invoked when shared descriptor is required to be change in driver context.
//
// @drv_ctx: driver context to be updated
// @sh_desc: new shared descriptor pointer to be updated in QI driver context
//
// Returns 0 on success or negative error code on failure.
//
    pub sh_desc): *mut *mut int caam_drv_ctx_update(struct caam_drv_ctx drv_ctx, u32,
//
// caam_drv_ctx_rel - Release a QI driver context
// @drv_ctx: context to be released
//
    pub drv_ctx): *mut void caam_drv_ctx_rel(struct caam_drv_ctx,
    pub pdev): *mut int caam_qi_init(struct platform_device,
//
// qi_cache_alloc - Allocate buffers from CAAM-QI cache
//
// Invoked when a user of the CAAM-QI (i.e. caamalg-qi) needs data which has
// to be allocated on the hotpath. Instead of using malloc, one can use the
// services of the CAAM QI memory cache (backed by kmem_cache). The buffers
// will have a size of 256B, which is sufficient for hosting 16 SG entries.
//
// @flags: flags that would be used for the equivalent malloc(..) call
//
// Returns a pointer to a retrieved buffer on success or NULL on failure.
//
    pub flags): *mut *mut void qi_cache_alloc(gfp_t,
//
// qi_cache_free - Frees buffers allocated from CAAM-QI cache
//
// Invoked when a user of the CAAM-QI (i.e. caamalg-qi) no longer needs
// the buffer previously allocated by a qi_cache_alloc call.
// No checking is being done, the call is a passthrough call to
// kmem_cache_free(...)
//
// @obj: object previously allocated using qi_cache_alloc()
//
    pub obj): *mut void qi_cache_free(void,
