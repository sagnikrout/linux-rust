//! Automatically rewritten from C to Rust
//! Source: drivers/accel/qaic/qaic_data.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2019-2021, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

pub const INBOUND_XFER: c_int = 1;
pub const OUTBOUND_XFER: c_int = 2;
pub const REQHP_OFF: c_uint = 0x0 /* we read this */;
pub const REQTP_OFF: c_uint = 0x4 /* we write this */;
pub const RSPHP_OFF: c_uint = 0x8 /* we write this */;
pub const RSPTP_OFF: c_uint = 0xc /* we read this */;

    ({							\
    FIELD_PREP(GENMASK(11, 0), (val)) |		\
    FIELD_PREP(GENMASK(20, 16), (index)) |		\
    FIELD_PREP(BIT(22), (sync)) |			\
    FIELD_PREP(GENMASK(26, 24), (cmd)) |		\
    FIELD_PREP(GENMASK(30, 29), (flags)) |		\
    FIELD_PREP(BIT(31), (cmd) ? 1 : 0);		\
    })
pub const NUM_EVENTS: c_int = 128;
pub const NUM_DELAYS: c_int = 10;

    static unsigned int wait_exec_default_timeout_ms = 5000; /* 5 sec default */
    module_param(wait_exec_default_timeout_ms, uint, 0600);
    MODULE_PARM_DESC(wait_exec_default_timeout_ms, "Default timeout for DRM_IOCTL_QAIC_WAIT_BO");
    static unsigned int datapath_poll_interval_us = 100; /* 100 usec default */
    module_param(datapath_poll_interval_us, uint, 0600);
    MODULE_PARM_DESC(datapath_poll_interval_us,
    "Amount of time to sleep between activity when datapath polling is enabled");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_req {
//
// A request ID is assigned to each memory handle going in DMA queue.
// As a single memory handle can enqueue multiple elements in DMA queue
// all of them will have the same request ID.
//
    pub req_id: __le16,
// Future use
    pub seq_id: __u8,
//
// Special encoded variable
// 7	0 - Do not force to generate MSI after DMA is completed
// 1 - Force to generate MSI after DMA is completed
// 6:5	Reserved
// 4	1 - Generate completion element in the response queue
// 0 - No Completion Code
// 3	0 - DMA request is a Link list transfer
// 1 - DMA request is a Bulk transfer
// 2	Reserved
// 1:0	00 - No DMA transfer involved
// 01 - DMA transfer is part of inbound transfer
// 10 - DMA transfer has outbound transfer
// 11 - NA
//
    pub cmd: __u8,
    pub resv: __le32,
// Source address for the transfer
    pub src_addr: __le64,
// Destination address for the transfer
    pub dest_addr: __le64,
// Length of transfer request
    pub len: __le32,
    pub resv2: __le32,
// Doorbell address
    pub db_addr: __le64,
//
// Special encoded variable
// 7	1 - Doorbell(db) write
// 0 - No doorbell write
// 6:2	Reserved
// 1:0	00 - 32 bit access, db address must be aligned to 32bit-boundary
// 01 - 16 bit access, db address must be aligned to 16bit-boundary
// 10 - 8 bit access, db address must be aligned to 8bit-boundary
// 11 - Reserved
//
    pub db_len: __u8,
    pub resv3: __u8,
    pub resv4: __le16,
// 32 bit data written to doorbell address
    pub db_data: __le32,
//
// Special encoded variable
// All the fields of sem_cmdX are passed from user and all are ORed
// together to form sem_cmd.
// 0:11		Semaphore value
// 15:12	Reserved
// 20:16	Semaphore index
// 21		Reserved
// 22		Semaphore Sync
// 23		Reserved
// 26:24	Semaphore command
// 28:27	Reserved
// 29		Semaphore DMA out bound sync fence
// 30		Semaphore DMA in bound sync fence
// 31		Enable semaphore command
//
    pub sem_cmd0: __le32,
    pub sem_cmd1: __le32,
    pub sem_cmd2: __le32,
    pub sem_cmd3: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbc_rsp {
// Request ID of the memory handle whose DMA transaction is completed
    pub req_id: __le16,
// Status of the DMA transaction. 0 : Success otherwise failure
    pub status: __le16,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn bo_queued(bo: *mut qaic_bo) -> bool {
    static inline bool bo_queued(struct qaic_bo *bo)
    {
    pub !list_empty(&bo->xfer_list): return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_dbc_req_elem_size() -> c_int {
    inline int get_dbc_req_elem_size(void)
    {
    pub dbc_req): return sizeof(struct,
    }
#[no_mangle]
pub unsafe extern "C" fn get_dbc_rsp_elem_size() -> c_int {
    inline int get_dbc_rsp_elem_size(void)
    {
    pub dbc_rsp): return sizeof(struct,
    }
#[no_mangle]
unsafe extern "C" fn free_slice(kref: *mut kref) {
    static void free_slice(struct kref *kref)
    {
    pub ref_count): *mut *mut bo_slice slice = container_of(kref, bo_slice,,
    pub slice->nents: slice->bo->total_slice_nents -=,
    }
    static int clone_range_of_sgt_for_slice(struct qaic_device *qdev, struct sg_table **sgt_out,
    struct sg_table *sgt_in, u64 size, u64 offset)
    {
    pub sgl: *mut *mut *mut *mut scatterlist sg, sgn, sgf,,
    pub offl: unsigned int len, nents, offf,,
    pub sgt: *mut sg_table,
    pub total_len: usize,
    pub j: int ret,,
// find out number of relevant nents needed for this mem
    pub 0: total_len =,
    pub NULL: sgf =,
    pub NULL: sgl =,
    pub 0: nents =,
    pub 0: offf =,
    pub 0: offl =,
    pub PAGE_SIZE: size = size ? size :,
    for_each_sgtable_dma_sg(sgt_in, sg, j) {
    pub sg_dma_len(sg): len =,
    if (!len)
    if (offset >= total_len && offset < total_len + len) {
    pub sg: sgf =,
    pub total_len: offf = offset -,
    }
    if (sgf)
    if (offset + size >= total_len &&
    offset + size <= total_len + len) {
    pub sg: sgl =,
    pub total_len: offl = offset + size -,
    }
    pub len: total_len +=,
    }
    if (!sgf || !sgl) {
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub kzalloc_obj(*sgt): *mut sgt =,
    if (!sgt) {
    pub -ENOMEM: ret =,
    pub out: goto,
    }
    pub GFP_KERNEL): ret = sg_alloc_table(sgt, nents,,
    if (ret)
    pub free_sgt: goto,
// copy relevant sg node and fix page and length
    pub sgf: sgn =,
    for_each_sgtable_dma_sg(sgt, sg, j) {
    pub sizeof(*sg)): *mut memcpy(sg, sgn,,
    if (sgn == sgf) {
    pub offf: sg_dma_address(sg) +=,
    pub offf: sg_dma_len(sg) -=,
    pub offf): sg_set_page(sg, sg_page(sgn), sg_dma_len(sg),,
    } else {
    pub 0: offf =,
    }
    if (sgn == sgl) {
    pub offf: sg_dma_len(sg) = offl -,
    pub offf): sg_set_page(sg, sg_page(sgn), offl - offf,,
    }
    pub sg_next(sgn): sgn =,
    }
// sgt_out = sgt;
    pub ret: return,
    free_sgt:
    out:
// sgt_out = NULL;
    pub ret: return,
    }
    static int encode_reqs(struct qaic_device *qdev, struct bo_slice *slice,
    struct qaic_attach_slice_entry *req)
    {
    pub cpu_to_le64(req->db_addr): __le64 db_addr =,
    pub cpu_to_le32(req->db_data): __le32 db_data =,
    pub sg: *mut scatterlist,
    pub BULK_XFER: __u8 cmd =,
    pub presync_sem: c_int,
    pub dev_addr: u64,
    pub db_len: __u8,
    pub i: c_int,
    if (!slice.no_xfer)
    pub OUTBOUND_XFER): cmd |= (slice->dir == DMA_TO_DEVICE ? INBOUND_XFER :,
    if (req.db_len && !IS_ALIGNED(req.db_addr, req.db_len / 8))
    pub -EINVAL: return,
    pub req->sem3.presync: presync_sem = req->sem0.presync + req->sem1.presync + req->sem2.presync +,
    if (presync_sem > 1)
    pub -EINVAL: return,
    presync_sem = req.sem0.presync << 0 | req.sem1.presync << 1 |
    pub 3: req->sem2.presync << 2 | req->sem3.presync <<,
    switch (req.db_len) {
    case 32:
    pub BIT(7): db_len =,
    case 16:
    pub 1: db_len = BIT(7) |,
    case 8:
    pub 2: db_len = BIT(7) |,
    case 0:
    pub /: *mut *mut db_len = 0; / doorbell is not active for this command,
    default:
    pub /: *mut *mut return -EINVAL; / should never hit this,
    }
//
// When we end up splitting up a single request (ie a buf slice) into
// multiple DMA requests, we have to manage the sync data carefully.
// There can only be one presync sem. That needs to be on every xfer
// so that the DMA engine doesn't transfer data before the receiver is
// ready. We only do the doorbell and postsync sems after the xfer.
// To guarantee previous xfers for the request are complete, we use a
// fence.
//
    pub req->dev_addr: dev_addr =,
    for_each_sgtable_dma_sg(slice.sgt, sg, i) {
    pub cmd: slice->reqs[i].cmd =,
    slice.reqs[i].src_addr = cpu_to_le64(slice.dir == DMA_TO_DEVICE ?
    pub dev_addr): sg_dma_address(sg) :,
    slice.reqs[i].dest_addr = cpu_to_le64(slice.dir == DMA_TO_DEVICE ?
    pub sg_dma_address(sg)): dev_addr :,
//
// sg_dma_len(sg) returns size of a DMA segment, maximum DMA
// segment size is set to UINT_MAX by qaic and hence return
// values of sg_dma_len(sg) can never exceed u32 range. So,
// by down sizing we are not corrupting the value.
//
    pub cpu_to_le32((u32)sg_dma_len(sg)): slice->reqs[i].len =,
    switch (presync_sem) {
    case BIT(0):
    slice.reqs[i].sem_cmd0 = cpu_to_le32(ENCODE_SEM(req.sem0.val,
    req.sem0.index,
    req.sem0.presync,
    req.sem0.cmd,
    case BIT(1):
    slice.reqs[i].sem_cmd1 = cpu_to_le32(ENCODE_SEM(req.sem1.val,
    req.sem1.index,
    req.sem1.presync,
    req.sem1.cmd,
    case BIT(2):
    slice.reqs[i].sem_cmd2 = cpu_to_le32(ENCODE_SEM(req.sem2.val,
    req.sem2.index,
    req.sem2.presync,
    req.sem2.cmd,
    case BIT(3):
    slice.reqs[i].sem_cmd3 = cpu_to_le32(ENCODE_SEM(req.sem3.val,
    req.sem3.index,
    req.sem3.presync,
    req.sem3.cmd,
    }
    pub sg_dma_len(sg): dev_addr +=,
    }
// add post transfer stuff to last segment
    pub GEN_COMPLETION: slice->reqs[i].cmd |=,
    pub db_addr: slice->reqs[i].db_addr =,
    pub db_len: slice->reqs[i].db_len =,
    pub db_data: slice->reqs[i].db_data =,
//
// Add a fence if we have more than one request going to the hardware
// representing the entirety of the user request, and the user request
// has no presync condition.
// Fences are expensive, so we try to avoid them. We rely on the
// hardware behavior to avoid needing one when there is a presync
// condition. When a presync exists, all requests for that same
// presync will be queued into a fifo. Thus, since we queue the
// post xfer activity only on the last request we queue, the hardware
// will ensure that the last queued request is processed last, thus
// making sure the post xfer activity happens at the right time without
// a fence.
//
    if (i && !presync_sem)
    req.sem0.flags |= (slice.dir == DMA_TO_DEVICE ?
    pub QAIC_SEM_OUTSYNCFENCE): QAIC_SEM_INSYNCFENCE :,
    slice.reqs[i].sem_cmd0 = cpu_to_le32(ENCODE_SEM(req.sem0.val, req.sem0.index,
    req.sem0.presync, req.sem0.cmd,
    slice.reqs[i].sem_cmd1 = cpu_to_le32(ENCODE_SEM(req.sem1.val, req.sem1.index,
    req.sem1.presync, req.sem1.cmd,
    slice.reqs[i].sem_cmd2 = cpu_to_le32(ENCODE_SEM(req.sem2.val, req.sem2.index,
    req.sem2.presync, req.sem2.cmd,
    slice.reqs[i].sem_cmd3 = cpu_to_le32(ENCODE_SEM(req.sem3.val, req.sem3.index,
    req.sem3.presync, req.sem3.cmd,
    pub 0: return,
    }
    static int qaic_map_one_slice(struct qaic_device *qdev, struct qaic_bo *bo,
    struct qaic_attach_slice_entry *slice_ent)
    {
    pub NULL: *mut *mut sg_table sgt =,
    pub slice: *mut bo_slice,
    pub ret: c_int,
    pub slice_ent->offset): ret = clone_range_of_sgt_for_slice(qdev, &sgt, bo->sgt, slice_ent->size,,
    if (ret)
    pub out: goto,
    pub kmalloc_obj(*slice): *mut slice =,
    if (!slice) {
    pub -ENOMEM: ret =,
    pub free_sgt: goto,
    }
    pub sgt->nents): *mut *mut slice->reqs = kvzalloc_objs(slice->reqs,,
    if (!slice.reqs) {
    pub -ENOMEM: ret =,
    pub free_slice: goto,
    }
    pub !slice_ent->size: slice->no_xfer =,
    pub sgt: slice->sgt =,
    pub sgt->nents: slice->nents =,
    pub bo->dir: slice->dir =,
    pub bo: slice->bo =,
    pub slice_ent->size: slice->size =,
    pub slice_ent->offset: slice->offset =,
    pub slice_ent): ret = encode_reqs(qdev, slice,,
    if (ret)
    pub free_req: goto,
    pub sgt->nents: bo->total_slice_nents +=,
    pub &bo->slices): list_add_tail(&slice->slice,,
    pub 0: return,
    free_req:
    free_slice:
    free_sgt:
    out:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn create_sgt(qdev: *mut qaic_device, sgt_out: *mut sg_table, size: u64) -> c_int {
    static int create_sgt(struct qaic_device *qdev, struct sg_table **sgt_out, u64 size)
    {
    pub sg: *mut scatterlist,
    pub sgt: *mut sg_table,
    pub pages: *mut page,
    pub pages_order: *mut c_int,
    pub buf_extra: c_int,
    pub max_order: c_int,
    pub nr_pages: c_int,
    pub 0: int ret =,
    pub k: int i, j,,
    pub order: c_int,
    if (size) {
    pub PAGE_SIZE): nr_pages = DIV_ROUND_UP(size,,
//
// calculate how much extra we are going to allocate, to remove
// later
//
    pub PAGE_SIZE: buf_extra = (PAGE_SIZE - size % PAGE_SIZE) %,
    pub get_order(size)): max_order = min(MAX_PAGE_ORDER,,
    } else {
// allocate a single page for book keeping
    pub 1: nr_pages =,
    pub 0: buf_extra =,
    pub 0: max_order =,
    }
    pub GFP_KERNEL): *mut *mut *mut pages = kvmalloc_array(nr_pages, sizeof(pages) + sizeof(pages_order),,
    if (!pages) {
    pub -ENOMEM: ret =,
    pub out: goto,
    }
    pub nr_pages: *mut *mut *mut *mut pages_order = (void )pages + sizeof(pages),
//
// Allocate requested memory using alloc_pages. It is possible to allocate
// the requested memory in multiple chunks by calling alloc_pages
// multiple times. Use SG table to handle multiple allocated pages.
//
    pub 0: i =,
    while (nr_pages > 0) {
    pub max_order): *mut *mut order = min(get_order(nr_pages  PAGE_SIZE),,
    while (1) {
    pages[i] = alloc_pages(GFP_KERNEL | GFP_HIGHUSER |
    __GFP_NOWARN | __GFP_ZERO |
    (order ? __GFP_NORETRY : __GFP_RETRY_MAYFAIL),
    if (pages[i])
    if (!order--) {
    pub -ENOMEM: ret =,
    pub free_partial_alloc: goto,
    }
    }
    pub order: max_order =,
    pub order: pages_order[i] =,
    pub order: nr_pages -= 1 <<,
    if (nr_pages <= 0)
// account for over allocation
    pub PAGE_SIZE: *mut *mut buf_extra += abs(nr_pages),
    }
    pub kmalloc_obj(*sgt): *mut sgt =,
    if (!sgt) {
    pub -ENOMEM: ret =,
    pub free_partial_alloc: goto,
    }
    if (sg_alloc_table(sgt, i, GFP_KERNEL)) {
    pub -ENOMEM: ret =,
    pub free_sgt: goto,
    }
// Populate the SG table with the allocated memory pages
    pub sgt->sgl: sg =,
    pub {: for (k = 0; k < i; k++, sg = sg_next(sg)),
// Last entry requires special handling
    if (k < i - 1) {
    pub 0): sg_set_page(sg, pages[k], PAGE_SIZE << pages_order[k],,
    } else {
    pub 0): sg_set_page(sg, pages[k], (PAGE_SIZE << pages_order[k]) - buf_extra,,
    }
    }
// sgt_out = sgt;
    pub ret: return,
    free_sgt:
    free_partial_alloc:
    pub j++): for (j = 0; j < i;,
    pub pages_order[j]): __free_pages(pages[j],,
    out:
// sgt_out = NULL;
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn invalid_sem(sem: *mut qaic_sem) -> bool {
    static bool invalid_sem(struct qaic_sem *sem)
    {
    if (sem.val & ~SEM_VAL_MASK || sem.index & ~SEM_INDEX_MASK ||
    !(sem.presync == 0 || sem.presync == 1) || sem.pad ||
    sem.flags & ~(QAIC_SEM_INSYNCFENCE | QAIC_SEM_OUTSYNCFENCE) ||
    sem.cmd > QAIC_SEM_WAIT_GT_0)
    pub true: return,
    pub false: return,
    }
    static int qaic_validate_req(struct qaic_device *qdev, struct qaic_attach_slice_entry *slice_ent,
    u32 count, u64 total_size)
    {
    pub total: u64,
    pub i: c_int,
    pub {: for (i = 0; i < count; i++),
    if (!(slice_ent[i].db_len == 32 || slice_ent[i].db_len == 16 ||
    slice_ent[i].db_len == 8 || slice_ent[i].db_len == 0) ||
    invalid_sem(&slice_ent[i].sem0) || invalid_sem(&slice_ent[i].sem1) ||
    invalid_sem(&slice_ent[i].sem2) || invalid_sem(&slice_ent[i].sem3))
    pub -EINVAL: return,
    if (check_add_overflow(slice_ent[i].offset, slice_ent[i].size, &total) ||
    total > total_size)
    pub -EINVAL: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn qaic_free_sgt(sgt: *mut sg_table) {
    static void qaic_free_sgt(struct sg_table *sgt)
    {
    pub sg: *mut scatterlist,
    if (!sgt)
    pub sg_next(sg)): for (sg = sgt->sgl; sg; sg =,
    if (sg_page(sg))
    pub get_order(sg->length)): __free_pages(sg_page(sg),,
    }
    static void qaic_gem_print_info(struct drm_printer *p, unsigned int indent,
    const struct drm_gem_object *obj)
    {
    pub to_qaic_bo(obj): *mut *mut qaic_bo bo =,
    pub bo->dir): drm_printf_indent(p, indent, "BO DMA direction %d\n",,
    }
    static const struct vm_operations_struct drm_vm_ops = {
    .open = drm_gem_vm_open,
    .close = drm_gem_vm_close,
}

#[no_mangle]
unsafe extern "C" fn qaic_gem_object_mmap(obj: *mut drm_gem_object, vma: *mut vm_area_struct) -> c_int {
    static int qaic_gem_object_mmap(struct drm_gem_object *obj, struct vm_area_struct *vma)
    {
    struct qaic_bo *bo = to_qaic_bo(obj);
    unsigned long remap_start;
    let mut offset: c_ulong = 0;
    unsigned long remap_end;
    struct scatterlist *sg;
    unsigned long length;
    let mut ret: c_int = 0;
    if (drm_gem_is_imported(obj))
    return -EINVAL;
    for (sg = bo.sgt.sgl; sg; sg = sg_next(sg)) {
    if (sg_page(sg)) {
// if sg is too large for the VMA, so truncate it to fit
    if (check_add_overflow(vma.vm_start, offset, &remap_start))
    return -EINVAL;
    if (check_add_overflow(remap_start, sg.length, &remap_end))
    return -EINVAL;
    if (remap_end > vma.vm_end) {
    if (check_sub_overflow(vma.vm_end, remap_start, &length))
    return -EINVAL;
    } else {
    length = sg.length;
    }
    if (length == 0)
    goto out;
    ret = remap_pfn_range(vma, vma.vm_start + offset, page_to_pfn(sg_page(sg)),
    length, vma.vm_page_prot);
    if (ret)
    goto out;
    offset += length;
    }
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qaic_free_object(obj: *mut drm_gem_object) {
    static void qaic_free_object(struct drm_gem_object *obj)
    {
    struct qaic_bo *bo = to_qaic_bo(obj);
    if (drm_gem_is_imported(obj)) {
// DMABUF/PRIME Path
    drm_prime_gem_destroy(obj, core::ptr::null_mut());
    } else {
// Private buffer allocation path
    qaic_free_sgt(bo.sgt);
    }
    mutex_destroy(&bo.lock);
    drm_gem_object_release(obj);
    kfree(bo);
    }
    static struct sg_table *qaic_get_sg_table(struct drm_gem_object *obj)
    {
    struct qaic_bo *bo = to_qaic_bo(obj);
    struct scatterlist *sg, *sg_in;
    struct sg_table *sgt, *sgt_in;
    int i;
    sgt_in = bo.sgt;
    sgt = kmalloc_obj(*sgt);
    if (!sgt)
    return ERR_PTR(-ENOMEM);
    if (sg_alloc_table(sgt, sgt_in.orig_nents, GFP_KERNEL)) {
    kfree(sgt);
    return ERR_PTR(-ENOMEM);
    }
    sg = sgt.sgl;
    for_each_sgtable_sg(sgt_in, sg_in, i) {
    memcpy(sg, sg_in, sizeof(*sg));
    sg = sg_next(sg);
    }
    return sgt;
    }
    static const struct drm_gem_object_funcs qaic_gem_funcs = {
    .free = qaic_free_object,
    .get_sg_table = qaic_get_sg_table,
    .print_info = qaic_gem_print_info,
    .mmap = qaic_gem_object_mmap,
    .vm_ops = &drm_vm_ops,
    };
#[no_mangle]
unsafe extern "C" fn qaic_init_bo(bo: *mut qaic_bo, reinit: bool) {
    static void qaic_init_bo(struct qaic_bo *bo, bool reinit)
    {
    if (reinit) {
    bo.sliced = false;
    reinit_completion(&bo.xfer_done);
    } else {
    mutex_init(&bo.lock);
    init_completion(&bo.xfer_done);
    }
    complete_all(&bo.xfer_done);
    INIT_LIST_HEAD(&bo.slices);
    INIT_LIST_HEAD(&bo.xfer_list);
    }
    static struct qaic_bo *qaic_alloc_init_bo(void)
    {
    struct qaic_bo *bo;
    bo = kzalloc_obj(*bo);
    if (!bo)
    return ERR_PTR(-ENOMEM);
    qaic_init_bo(bo, false);
    return bo;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_create_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_create_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_create_bo *args = data;
    int usr_rcu_id, qdev_rcu_id;
    struct drm_gem_object *obj;
    struct qaic_device *qdev;
    struct qaic_user *usr;
    struct qaic_bo *bo;
    size_t size;
    int ret;
    if (args.pad)
    return -EINVAL;
    size = PAGE_ALIGN(args.size);
    if (size == 0)
    return -EINVAL;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    bo = qaic_alloc_init_bo();
    if (IS_ERR(bo)) {
    ret = PTR_ERR(bo);
    goto unlock_dev_srcu;
    }
    obj = &bo.base;
    drm_gem_private_object_init(dev, obj, size);
    obj.funcs = &qaic_gem_funcs;
    ret = create_sgt(qdev, &bo.sgt, size);
    if (ret)
    goto free_bo;
    ret = drm_gem_create_mmap_offset(obj);
    if (ret)
    goto free_bo;
    ret = drm_gem_handle_create(file_priv, obj, &args.handle);
    if (ret)
    goto free_bo;
    drm_gem_object_put(obj);
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return 0;
    free_bo:
    drm_gem_object_put(obj);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_mmap_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_mmap_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_mmap_bo *args = data;
    int usr_rcu_id, qdev_rcu_id;
    struct drm_gem_object *obj;
    struct qaic_device *qdev;
    struct qaic_user *usr;
    let mut ret: c_int = 0;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    obj = drm_gem_object_lookup(file_priv, args.handle);
    if (!obj) {
    ret = -ENOENT;
    goto unlock_dev_srcu;
    }
    args.offset = drm_vma_node_offset_addr(&obj.vma_node);
    drm_gem_object_put(obj);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
    struct drm_gem_object *qaic_gem_prime_import(struct drm_device *dev, struct dma_buf *dma_buf)
    {
    struct dma_buf_attachment *attach;
    struct drm_gem_object *obj;
    struct qaic_bo *bo;
    int ret;
    bo = qaic_alloc_init_bo();
    if (IS_ERR(bo)) {
    ret = PTR_ERR(bo);
    goto out;
    }
    obj = &bo.base;
    get_dma_buf(dma_buf);
    attach = dma_buf_attach(dma_buf, dev.dev);
    if (IS_ERR(attach)) {
    ret = PTR_ERR(attach);
    goto attach_fail;
    }
    if (!attach.dmabuf.size) {
    ret = -EINVAL;
    goto size_align_fail;
    }
    drm_gem_private_object_init(dev, obj, attach.dmabuf.size);
//
// skipping dma_buf_map_attachment() as we do not know the direction
// just yet. Once the direction is known in the subsequent IOCTL to
// attach slicing, we can do it then.
//
    obj.funcs = &qaic_gem_funcs;
    obj.import_attach = attach;
    obj.resv = dma_buf.resv;
    return obj;
    size_align_fail:
    dma_buf_detach(dma_buf, attach);
    attach_fail:
    dma_buf_put(dma_buf);
    kfree(bo);
    out:
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn qaic_prepare_import_bo(bo: *mut qaic_bo, hdr: *mut qaic_attach_slice_hdr) -> c_int {
    static int qaic_prepare_import_bo(struct qaic_bo *bo, struct qaic_attach_slice_hdr *hdr)
    {
    struct drm_gem_object *obj = &bo.base;
    struct sg_table *sgt;
    int ret;
    sgt = dma_buf_map_attachment(obj.import_attach, hdr.dir);
    if (IS_ERR(sgt)) {
    ret = PTR_ERR(sgt);
    return ret;
    }
    bo.sgt = sgt;
    return 0;
    }
    static int qaic_prepare_export_bo(struct qaic_device *qdev, struct qaic_bo *bo,
    struct qaic_attach_slice_hdr *hdr)
    {
    int ret;
    ret = dma_map_sgtable(&qdev.pdev.dev, bo.sgt, hdr.dir, 0);
    if (ret)
    return -EFAULT;
    return 0;
    }
    static int qaic_prepare_bo(struct qaic_device *qdev, struct qaic_bo *bo,
    struct qaic_attach_slice_hdr *hdr)
    {
    int ret;
    if (drm_gem_is_imported(&bo.base))
    ret = qaic_prepare_import_bo(bo, hdr);
    else
    ret = qaic_prepare_export_bo(qdev, bo, hdr);
    bo.dir = hdr.dir;
    bo.dbc = &qdev.dbc[hdr.dbc_id];
    bo.nr_slice = hdr.count;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qaic_unprepare_import_bo(bo: *mut qaic_bo) {
    static void qaic_unprepare_import_bo(struct qaic_bo *bo)
    {
    dma_buf_unmap_attachment(bo.base.import_attach, bo.sgt, bo.dir);
    bo.sgt = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn qaic_unprepare_export_bo(qdev: *mut qaic_device, bo: *mut qaic_bo) {
    static void qaic_unprepare_export_bo(struct qaic_device *qdev, struct qaic_bo *bo)
    {
    dma_unmap_sgtable(&qdev.pdev.dev, bo.sgt, bo.dir, 0);
    }
#[no_mangle]
unsafe extern "C" fn qaic_unprepare_bo(qdev: *mut qaic_device, bo: *mut qaic_bo) {
    static void qaic_unprepare_bo(struct qaic_device *qdev, struct qaic_bo *bo)
    {
    if (drm_gem_is_imported(&bo.base))
    qaic_unprepare_import_bo(bo);
    else
    qaic_unprepare_export_bo(qdev, bo);
    bo.dir = 0;
    bo.dbc = core::ptr::null_mut();
    bo.nr_slice = 0;
    }
#[no_mangle]
unsafe extern "C" fn qaic_free_slices_bo(bo: *mut qaic_bo) {
    static void qaic_free_slices_bo(struct qaic_bo *bo)
    {
    struct bo_slice *slice, *temp;
    list_for_each_entry_safe(slice, temp, &bo.slices, slice)
    kref_put(&slice.ref_count, free_slice);
    if (WARN_ON_ONCE(bo.total_slice_nents != 0))
    bo.total_slice_nents = 0;
    bo.nr_slice = 0;
    }
    static int qaic_attach_slicing_bo(struct qaic_device *qdev, struct qaic_bo *bo,
    struct qaic_attach_slice_hdr *hdr,
    struct qaic_attach_slice_entry *slice_ent)
    {
    int ret, i;
    for (i = 0; i < hdr.count; i++) {
    ret = qaic_map_one_slice(qdev, bo, &slice_ent[i]);
    if (ret) {
    qaic_free_slices_bo(bo);
    return ret;
    }
    }
    if (bo.total_slice_nents > bo.dbc.nelem) {
    qaic_free_slices_bo(bo);
    return -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_attach_slice_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_attach_slice_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_attach_slice_entry *slice_ent;
    struct qaic_attach_slice *args = data;
    int rcu_id, usr_rcu_id, qdev_rcu_id;
    struct dma_bridge_chan	*dbc;
    struct drm_gem_object *obj;
    struct qaic_device *qdev;
    unsigned long arg_size;
    struct qaic_user *usr;
    u8 __user *user_data;
    struct qaic_bo *bo;
    int ret;
    if (args.hdr.count == 0)
    return -EINVAL;
    if (check_mul_overflow((unsigned long)args.hdr.count,
    (unsigned long)sizeof(*slice_ent),
    &arg_size))
    return -EINVAL;
    if (!(args.hdr.dir == DMA_TO_DEVICE || args.hdr.dir == DMA_FROM_DEVICE))
    return -EINVAL;
    if (args.data == 0)
    return -EINVAL;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    if (args.hdr.dbc_id >= qdev.num_dbc) {
    ret = -EINVAL;
    goto unlock_dev_srcu;
    }
    user_data = u64_to_user_ptr(args.data);
    slice_ent = memdup_user(user_data, arg_size);
    if (IS_ERR(slice_ent)) {
    ret = PTR_ERR(slice_ent);
    goto unlock_dev_srcu;
    }
    obj = drm_gem_object_lookup(file_priv, args.hdr.handle);
    if (!obj) {
    ret = -ENOENT;
    goto free_slice_ent;
    }
    ret = qaic_validate_req(qdev, slice_ent, args.hdr.count, obj.size);
    if (ret)
    goto put_bo;
    bo = to_qaic_bo(obj);
    ret = mutex_lock_interruptible(&bo.lock);
    if (ret)
    goto put_bo;
    if (bo.sliced) {
    ret = -EINVAL;
    goto unlock_bo;
    }
    dbc = &qdev.dbc[args.hdr.dbc_id];
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    if (dbc.usr != usr) {
    ret = -EINVAL;
    goto unlock_ch_srcu;
    }
    if (dbc.id == qdev.ssr_dbc) {
    ret = -EPIPE;
    goto unlock_ch_srcu;
    }
    ret = qaic_prepare_bo(qdev, bo, &args.hdr);
    if (ret)
    goto unlock_ch_srcu;
    ret = qaic_attach_slicing_bo(qdev, bo, &args.hdr, slice_ent);
    if (ret)
    goto unprepare_bo;
    if (args.hdr.dir == DMA_TO_DEVICE)
    dma_sync_sgtable_for_cpu(&qdev.pdev.dev, bo.sgt, args.hdr.dir);
    bo.sliced = true;
    list_add_tail(&bo.bo_list, &bo.dbc.bo_lists);
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    mutex_unlock(&bo.lock);
    kfree(slice_ent);
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return 0;
    unprepare_bo:
    qaic_unprepare_bo(qdev, bo);
    unlock_ch_srcu:
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    unlock_bo:
    mutex_unlock(&bo.lock);
    put_bo:
    drm_gem_object_put(obj);
    free_slice_ent:
    kfree(slice_ent);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn fifo_space_avail(head: u32, tail: u32, q_size: u32) -> u32 {
    static inline u32 fifo_space_avail(u32 head, u32 tail, u32 q_size)
    {
    let mut avail: u32 = head - tail - 1;
    if (head <= tail)
    avail += q_size;
    return avail;
    }
    static inline int copy_exec_reqs(struct qaic_device *qdev, struct bo_slice *slice, u32 dbc_id,
    u32 head, u32 *ptail)
    {
    struct dma_bridge_chan *dbc = &qdev.dbc[dbc_id];
    struct dbc_req *reqs = slice.reqs;
    let mut tail: u32 = *ptail;
    u32 avail;
    avail = fifo_space_avail(head, tail, dbc.nelem);
    if (avail < slice.nents)
    return -EAGAIN;
    if (tail + slice.nents > dbc.nelem) {
    avail = dbc.nelem - tail;
    avail = min_t(u32, avail, slice.nents);
    memcpy(fifo_at(dbc.req_q_base, tail), reqs, sizeof(*reqs) * avail);
    reqs += avail;
    avail = slice.nents - avail;
    if (avail)
    memcpy(dbc.req_q_base, reqs, sizeof(*reqs) * avail);
    } else {
    memcpy(fifo_at(dbc.req_q_base, tail), reqs, sizeof(*reqs) * slice.nents);
    }
// ptail = (tail + slice->nents) % dbc->nelem;
    return 0;
    }
    static inline int copy_partial_exec_reqs(struct qaic_device *qdev, struct bo_slice *slice,
    u64 resize, struct dma_bridge_chan *dbc, u32 head,
    u32 *ptail)
    {
    struct dbc_req *reqs = slice.reqs;
    struct dbc_req *last_req;
    let mut tail: u32 = *ptail;
    u64 last_bytes;
    u32 first_n;
    u32 avail;
    avail = fifo_space_avail(head, tail, dbc.nelem);
//
// After this for loop is complete, first_n represents the index
// of the last DMA request of this slice that needs to be
// transferred after resizing and last_bytes represents DMA size
// of that request.
//
    last_bytes = resize;
    for (first_n = 0; first_n < slice.nents; first_n++)
    if (last_bytes > le32_to_cpu(reqs[first_n].len))
    last_bytes -= le32_to_cpu(reqs[first_n].len);
    else
    break;
    if (avail < (first_n + 1))
    return -EAGAIN;
    if (first_n) {
    if (tail + first_n > dbc.nelem) {
    avail = dbc.nelem - tail;
    avail = min_t(u32, avail, first_n);
    memcpy(fifo_at(dbc.req_q_base, tail), reqs, sizeof(*reqs) * avail);
    last_req = reqs + avail;
    avail = first_n - avail;
    if (avail)
    memcpy(dbc.req_q_base, last_req, sizeof(*reqs) * avail);
    } else {
    memcpy(fifo_at(dbc.req_q_base, tail), reqs, sizeof(*reqs) * first_n);
    }
    }
//
// Copy over the last entry. Here we need to adjust len to the left over
// size, and set src and dst to the entry it is copied to.
//
    last_req = fifo_at(dbc.req_q_base, (tail + first_n) % dbc.nelem);
    memcpy(last_req, reqs + slice.nents - 1, sizeof(*reqs));
//
// last_bytes holds size of a DMA segment, maximum DMA segment size is
// set to UINT_MAX by qaic and hence last_bytes can never exceed u32
// range. So, by down sizing we are not corrupting the value.
//
    last_req.len = cpu_to_le32((u32)last_bytes);
    last_req.src_addr = reqs[first_n].src_addr;
    last_req.dest_addr = reqs[first_n].dest_addr;
    if (!last_bytes)
// Disable DMA transfer
    last_req.cmd = GENMASK(7, 2) & reqs[first_n].cmd;
// ptail = (tail + first_n + 1) % dbc->nelem;
    return 0;
    }
    static int send_bo_list_to_device(struct qaic_device *qdev, struct drm_file *file_priv,
    struct qaic_execute_entry *exec, unsigned int count,
    bool is_partial, struct dma_bridge_chan *dbc, u32 head,
    u32 *tail)
    {
    struct qaic_partial_execute_entry *pexec = (struct qaic_partial_execute_entry *)exec;
    struct drm_gem_object *obj;
    struct bo_slice *slice;
    unsigned long flags;
    struct qaic_bo *bo;
    int i, j;
    int ret;
    for (i = 0; i < count; i++) {
//
// ref count will be decremented when the transfer of this
// buffer is complete. It is inside dbc_irq_threaded_fn().
//
    obj = drm_gem_object_lookup(file_priv,
    is_partial ? pexec[i].handle : exec[i].handle);
    if (!obj) {
    ret = -ENOENT;
    goto failed_to_send_bo;
    }
    bo = to_qaic_bo(obj);
    ret = mutex_lock_interruptible(&bo.lock);
    if (ret)
    goto failed_to_send_bo;
    if (!bo.sliced) {
    ret = -EINVAL;
    goto unlock_bo;
    }
    if (is_partial && pexec[i].resize > bo.base.size) {
    ret = -EINVAL;
    goto unlock_bo;
    }
    spin_lock_irqsave(&dbc.xfer_lock, flags);
    if (bo_queued(bo)) {
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    ret = -EINVAL;
    goto unlock_bo;
    }
    bo.req_id = dbc.next_req_id++;
    list_for_each_entry(slice, &bo.slices, slice) {
    for (j = 0; j < slice.nents; j++)
    slice.reqs[j].req_id = cpu_to_le16(bo.req_id);
    if (is_partial && (!pexec[i].resize || pexec[i].resize <= slice.offset))
// Configure the slice for no DMA transfer
    ret = copy_partial_exec_reqs(qdev, slice, 0, dbc, head, tail);
#[no_mangle]
pub unsafe extern "C" fn if(slice->size: is_partial && pexec[i].resize < slice->offset +) -> else {
    else if (is_partial && pexec[i].resize < slice.offset + slice.size)
// Configure the slice to be partially DMA transferred
    ret = copy_partial_exec_reqs(qdev, slice,
    pexec[i].resize - slice.offset, dbc,
    head, tail);
    else
    ret = copy_exec_reqs(qdev, slice, dbc.id, head, tail);
    if (ret) {
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    goto unlock_bo;
    }
    }
    reinit_completion(&bo.xfer_done);
    list_add_tail(&bo.xfer_list, &dbc.xfer_list);
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    dma_sync_sgtable_for_device(&qdev.pdev.dev, bo.sgt, bo.dir);
    mutex_unlock(&bo.lock);
    }
    return 0;
    unlock_bo:
    mutex_unlock(&bo.lock);
    failed_to_send_bo:
    if (likely(obj))
    drm_gem_object_put(obj);
    for (j = 0; j < i; j++) {
    spin_lock_irqsave(&dbc.xfer_lock, flags);
    bo = list_last_entry(&dbc.xfer_list, struct qaic_bo, xfer_list);
    obj = &bo.base;
    list_del_init(&bo.xfer_list);
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    dma_sync_sgtable_for_cpu(&qdev.pdev.dev, bo.sgt, bo.dir);
    drm_gem_object_put(obj);
    }
    return ret;
    }
    static void update_profiling_data(struct drm_file *file_priv,
    struct qaic_execute_entry *exec, unsigned int count,
    bool is_partial, u64 received_ts, u64 submit_ts, u32 queue_level)
    {
    struct qaic_partial_execute_entry *pexec = (struct qaic_partial_execute_entry *)exec;
    struct drm_gem_object *obj;
    struct qaic_bo *bo;
    int i;
    for (i = 0; i < count; i++) {
//
// Since we already committed the BO to hardware, the only way
// this should fail is a pending signal. We can't cancel the
// submit to hardware, so we have to just skip the profiling
// data. In case the signal is not fatal to the process, we
// return success so that the user doesn't try to resubmit.
//
    obj = drm_gem_object_lookup(file_priv,
    is_partial ? pexec[i].handle : exec[i].handle);
    if (!obj)
    break;
    bo = to_qaic_bo(obj);
    bo.perf_stats.req_received_ts = received_ts;
    bo.perf_stats.req_submit_ts = submit_ts;
    bo.perf_stats.queue_level_before = queue_level;
    queue_level += bo.total_slice_nents;
    drm_gem_object_put(obj);
    }
    }
    static int __qaic_execute_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv,
    bool is_partial)
    {
    struct qaic_execute *args = data;
    struct qaic_execute_entry *exec;
    struct dma_bridge_chan *dbc;
    int usr_rcu_id, qdev_rcu_id;
    struct qaic_device *qdev;
    struct qaic_user *usr;
    u64 received_ts;
    u32 queue_level;
    u64 submit_ts;
    int rcu_id;
    u32 head;
    u32 tail;
    u64 size;
    int ret;
    received_ts = ktime_get_ns();
    size = is_partial ? sizeof(struct qaic_partial_execute_entry) : sizeof(*exec);
    if (args.hdr.count == 0)
    return -EINVAL;
    exec = memdup_array_user(u64_to_user_ptr(args.data), args.hdr.count, size);
    if (IS_ERR(exec))
    return PTR_ERR(exec);
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    if (args.hdr.dbc_id >= qdev.num_dbc) {
    ret = -EINVAL;
    goto unlock_dev_srcu;
    }
    dbc = &qdev.dbc[args.hdr.dbc_id];
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    if (!dbc.usr || dbc.usr.handle != usr.handle) {
    ret = -EPERM;
    goto release_ch_rcu;
    }
    if (dbc.id == qdev.ssr_dbc) {
    ret = -EPIPE;
    goto release_ch_rcu;
    }
    ret = mutex_lock_interruptible(&dbc.req_lock);
    if (ret)
    goto release_ch_rcu;
    head = readl(dbc.dbc_base + REQHP_OFF);
    tail = readl(dbc.dbc_base + REQTP_OFF);
    if (head == U32_MAX || tail == U32_MAX) {
// PCI link error
    ret = -ENODEV;
    goto unlock_req_lock;
    }
    queue_level = head <= tail ? tail - head : dbc.nelem - (head - tail);
    ret = send_bo_list_to_device(qdev, file_priv, exec, args.hdr.count, is_partial, dbc,
    head, &tail);
    if (ret)
    goto unlock_req_lock;
// Finalize commit to hardware
    submit_ts = ktime_get_ns();
    writel(tail, dbc.dbc_base + REQTP_OFF);
    mutex_unlock(&dbc.req_lock);
    update_profiling_data(file_priv, exec, args.hdr.count, is_partial, received_ts,
    submit_ts, queue_level);
    if (datapath_polling)
    schedule_work(&dbc.poll_work);
    unlock_req_lock:
    if (ret)
    mutex_unlock(&dbc.req_lock);
    release_ch_rcu:
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    kfree(exec);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_execute_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_execute_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    return __qaic_execute_bo_ioctl(dev, data, file_priv, false);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_partial_execute_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_partial_execute_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    return __qaic_execute_bo_ioctl(dev, data, file_priv, true);
    }
//
// Our interrupt handling is a bit more complicated than a simple ideal, but
// sadly necessary.
//
// Each dbc has a completion queue. Entries in the queue correspond to DMA
// requests which the device has processed. The hardware already has a built
// in irq mitigation. When the device puts an entry into the queue, it will
// only trigger an interrupt if the queue was empty. Therefore, when adding
// the Nth event to a non-empty queue, the hardware doesn't trigger an
// interrupt. This means the host doesn't get additional interrupts signaling
// the same thing - the queue has something to process.
// This behavior can be overridden in the DMA request.
// This means that when the host receives an interrupt, it is required to
// drain the queue.
//
// This behavior is what NAPI attempts to accomplish, although we can't use
// NAPI as we don't have a netdev. We use threaded irqs instead.
//
// However, there is a situation where the host drains the queue fast enough
// that every event causes an interrupt. Typically this is not a problem as
// the rate of events would be low. However, that is not the case with
// lprnet for example. On an Intel Xeon D-2191 where we run 8 instances of
// lprnet, the host receives roughly 80k interrupts per second from the device
// (per /proc/interrupts). While NAPI documentation indicates the host should
// just chug along, sadly that behavior causes instability in some hosts.
//
// Therefore, we implement an interrupt disable scheme similar to NAPI. The
// key difference is that we will delay after draining the queue for a small
// time to allow additional events to come in via polling. Using the above
// lprnet workload, this reduces the number of interrupts processed from
// ~80k/sec to about 64 in 5 minutes and appears to solve the system
// instability.
//
#[no_mangle]
pub unsafe extern "C" fn dbc_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    irqreturn_t dbc_irq_handler(int irq, void *data)
    {
    struct dma_bridge_chan *dbc = data;
    int rcu_id;
    u32 head;
    u32 tail;
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    if (datapath_polling) {
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
//
// Normally datapath_polling will not have irqs enabled, but
// when running with only one MSI the interrupt is shared with
// MHI so it cannot be disabled. Return ASAP instead.
//
    return IRQ_HANDLED;
    }
    if (!dbc.usr) {
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return IRQ_HANDLED;
    }
    head = readl(dbc.dbc_base + RSPHP_OFF);
    if (head == U32_MAX) { /* PCI link error */
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return IRQ_NONE;
    }
    tail = readl(dbc.dbc_base + RSPTP_OFF);
    if (tail == U32_MAX) { /* PCI link error */
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return IRQ_NONE;
    }
    if (head == tail) { /* queue empty */
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return IRQ_NONE;
    }
    if (!dbc.qdev.single_msi)
    disable_irq_nosync(irq);
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_irq_polling_work(work: *mut work_struct) {
    void qaic_irq_polling_work(struct work_struct *work)
    {
    struct dma_bridge_chan *dbc = container_of(work, struct dma_bridge_chan,  poll_work);
    unsigned long flags;
    int rcu_id;
    u32 head;
    u32 tail;
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    while (1) {
    if (dbc.qdev.dev_state != QAIC_ONLINE) {
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return;
    }
    if (!dbc.usr) {
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return;
    }
    spin_lock_irqsave(&dbc.xfer_lock, flags);
    if (list_empty(&dbc.xfer_list)) {
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return;
    }
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    head = readl(dbc.dbc_base + RSPHP_OFF);
    if (head == U32_MAX) { /* PCI link error */
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return;
    }
    tail = readl(dbc.dbc_base + RSPTP_OFF);
    if (tail == U32_MAX) { /* PCI link error */
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return;
    }
    if (head != tail) {
    irq_wake_thread(dbc.irq, dbc);
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return;
    }
    cond_resched();
    usleep_range(datapath_poll_interval_us, 2 * datapath_poll_interval_us);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dbc_irq_threaded_fn(irq: c_int, data: *mut c_void) -> irqreturn_t {
    irqreturn_t dbc_irq_threaded_fn(int irq, void *data)
    {
    struct dma_bridge_chan *dbc = data;
    let mut event_count: c_int = NUM_EVENTS;
    let mut delay_count: c_int = NUM_DELAYS;
    struct qaic_device *qdev;
    struct qaic_bo *bo, *i;
    struct dbc_rsp *rsp;
    unsigned long flags;
    int rcu_id;
    u16 status;
    u16 req_id;
    u32 head;
    u32 tail;
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    qdev = dbc.qdev;
    head = readl(dbc.dbc_base + RSPHP_OFF);
    if (head == U32_MAX) /* PCI link error */
    goto error_out;
    read_fifo:
    if (!event_count) {
    event_count = NUM_EVENTS;
    cond_resched();
    }
//
// if this channel isn't assigned or gets unassigned during processing
// we have nothing further to do
//
    if (!dbc.usr)
    goto error_out;
    tail = readl(dbc.dbc_base + RSPTP_OFF);
    if (tail == U32_MAX) /* PCI link error */
    goto error_out;
    if (head == tail) { /* queue empty */
    if (delay_count) {
    --delay_count;
    usleep_range(100, 200);
    goto read_fifo; /* check for a new event */
    }
    goto normal_out;
    }
    delay_count = NUM_DELAYS;
    while (head != tail) {
    if (!event_count)
    break;
    --event_count;
    rsp = dbc.rsp_q_base + head * sizeof(*rsp);
    req_id = le16_to_cpu(rsp.req_id);
    status = le16_to_cpu(rsp.status);
    if (status)
    pci_dbg(qdev.pdev, "req_id %d failed with status %d\n", req_id, status);
    spin_lock_irqsave(&dbc.xfer_lock, flags);
//
// A BO can receive multiple interrupts, since a BO can be
// divided into multiple slices and a buffer receives as many
// interrupts as slices. So until it receives interrupts for
// all the slices we cannot mark that buffer complete.
//
    list_for_each_entry_safe(bo, i, &dbc.xfer_list, xfer_list) {
    if (bo.req_id == req_id)
    bo.nr_slice_xfer_done++;
    else
    continue;
    if (bo.nr_slice_xfer_done < bo.nr_slice)
    break;
//
// At this point we have received all the interrupts for
// BO, which means BO execution is complete.
//
    dma_sync_sgtable_for_cpu(&qdev.pdev.dev, bo.sgt, bo.dir);
    bo.nr_slice_xfer_done = 0;
    list_del_init(&bo.xfer_list);
    bo.perf_stats.req_processed_ts = ktime_get_ns();
    complete_all(&bo.xfer_done);
    drm_gem_object_put(&bo.base);
    break;
    }
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    head = (head + 1) % dbc.nelem;
    }
//
// Update the head pointer of response queue and let the device know
// that we have consumed elements from the queue.
//
    writel(head, dbc.dbc_base + RSPHP_OFF);
// elements might have been put in the queue while we were processing
    goto read_fifo;
    normal_out:
    if (!qdev.single_msi && likely(!datapath_polling))
    enable_irq(irq);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: unlikely(datapath_polling)) -> else {
    else if (unlikely(datapath_polling))
    schedule_work(&dbc.poll_work);
// checking the fifo and enabling irqs is a race, missed event check
    tail = readl(dbc.dbc_base + RSPTP_OFF);
    if (tail != U32_MAX && head != tail) {
    if (!qdev.single_msi && likely(!datapath_polling))
    disable_irq_nosync(irq);
    goto read_fifo;
    }
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    return IRQ_HANDLED;
    error_out:
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    if (!qdev.single_msi && likely(!datapath_polling))
    enable_irq(irq);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: unlikely(datapath_polling)) -> else {
    else if (unlikely(datapath_polling))
    schedule_work(&dbc.poll_work);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_wait_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_wait_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_wait *args = data;
    int usr_rcu_id, qdev_rcu_id;
    struct dma_bridge_chan *dbc;
    struct drm_gem_object *obj;
    struct qaic_device *qdev;
    unsigned long timeout;
    struct qaic_user *usr;
    struct qaic_bo *bo;
    int rcu_id;
    int ret;
    if (args.pad != 0)
    return -EINVAL;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    if (args.dbc_id >= qdev.num_dbc) {
    ret = -EINVAL;
    goto unlock_dev_srcu;
    }
    dbc = &qdev.dbc[args.dbc_id];
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    if (dbc.usr != usr) {
    ret = -EPERM;
    goto unlock_ch_srcu;
    }
    if (dbc.id == qdev.ssr_dbc) {
    ret = -EPIPE;
    goto unlock_ch_srcu;
    }
    obj = drm_gem_object_lookup(file_priv, args.handle);
    if (!obj) {
    ret = -ENOENT;
    goto unlock_ch_srcu;
    }
    bo = to_qaic_bo(obj);
    timeout = args.timeout ? args.timeout : wait_exec_default_timeout_ms;
    timeout = msecs_to_jiffies(timeout);
    ret = wait_for_completion_interruptible_timeout(&bo.xfer_done, timeout);
    if (!ret) {
    ret = -ETIMEDOUT;
    goto put_obj;
    }
    if (ret > 0)
    ret = 0;
    if (!dbc.usr)
    ret = -EPERM;
    if (dbc.id == qdev.ssr_dbc)
    ret = -EPIPE;
    put_obj:
    drm_gem_object_put(obj);
    unlock_ch_srcu:
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_perf_stats_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_perf_stats_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_perf_stats_entry *ent = core::ptr::null_mut();
    struct qaic_perf_stats *args = data;
    int usr_rcu_id, qdev_rcu_id;
    struct drm_gem_object *obj;
    struct qaic_device *qdev;
    struct qaic_user *usr;
    struct qaic_bo *bo;
    let mut ret: c_int = 0;
    int i;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    if (args.hdr.dbc_id >= qdev.num_dbc) {
    ret = -EINVAL;
    goto unlock_dev_srcu;
    }
    ent = memdup_array_user(u64_to_user_ptr(args.data), args.hdr.count, sizeof(*ent));
    if (IS_ERR(ent)) {
    ret = PTR_ERR(ent);
    goto unlock_dev_srcu;
    }
    for (i = 0; i < args.hdr.count; i++) {
    obj = drm_gem_object_lookup(file_priv, ent[i].handle);
    if (!obj) {
    ret = -ENOENT;
    goto free_ent;
    }
    bo = to_qaic_bo(obj);
    if (!bo.sliced) {
    drm_gem_object_put(obj);
    ret = -EINVAL;
    goto free_ent;
    }
    if (bo.dbc.id != args.hdr.dbc_id) {
    drm_gem_object_put(obj);
    ret = -EINVAL;
    goto free_ent;
    }
//
// perf stats ioctl is called before wait ioctl is complete then
// the latency information is invalid.
//
    if (bo.perf_stats.req_processed_ts < bo.perf_stats.req_submit_ts) {
    ent[i].device_latency_us = 0;
    } else {
    ent[i].device_latency_us = div_u64((bo.perf_stats.req_processed_ts -
    bo.perf_stats.req_submit_ts), 1000);
    }
    ent[i].submit_latency_us = div_u64((bo.perf_stats.req_submit_ts -
    bo.perf_stats.req_received_ts), 1000);
    ent[i].queue_level_before = bo.perf_stats.queue_level_before;
    ent[i].num_queue_element = bo.total_slice_nents;
    drm_gem_object_put(obj);
    }
    if (copy_to_user(u64_to_user_ptr(args.data), ent, args.hdr.count * sizeof(*ent)))
    ret = -EFAULT;
    free_ent:
    kfree(ent);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn detach_slice_bo(qdev: *mut qaic_device, bo: *mut qaic_bo) {
    static void detach_slice_bo(struct qaic_device *qdev, struct qaic_bo *bo)
    {
    qaic_free_slices_bo(bo);
    qaic_unprepare_bo(qdev, bo);
    qaic_init_bo(bo, true);
    list_del(&bo.bo_list);
    drm_gem_object_put(&bo.base);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_detach_slice_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_detach_slice_bo_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_detach_slice *args = data;
    int rcu_id, usr_rcu_id, qdev_rcu_id;
    struct dma_bridge_chan *dbc;
    struct drm_gem_object *obj;
    struct qaic_device *qdev;
    struct qaic_user *usr;
    unsigned long flags;
    struct qaic_bo *bo;
    int ret;
    if (args.pad != 0)
    return -EINVAL;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    ret = -ENODEV;
    goto unlock_usr_srcu;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    ret = -ENODEV;
    goto unlock_dev_srcu;
    }
    obj = drm_gem_object_lookup(file_priv, args.handle);
    if (!obj) {
    ret = -ENOENT;
    goto unlock_dev_srcu;
    }
    bo = to_qaic_bo(obj);
    ret = mutex_lock_interruptible(&bo.lock);
    if (ret)
    goto put_bo;
    if (!bo.sliced) {
    ret = -EINVAL;
    goto unlock_bo;
    }
    dbc = bo.dbc;
    rcu_id = srcu_read_lock(&dbc.ch_lock);
    if (dbc.usr != usr) {
    ret = -EINVAL;
    goto unlock_ch_srcu;
    }
// Check if BO is committed to H/W for DMA
    spin_lock_irqsave(&dbc.xfer_lock, flags);
    if (bo_queued(bo)) {
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    ret = -EBUSY;
    goto unlock_ch_srcu;
    }
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    detach_slice_bo(qdev, bo);
    unlock_ch_srcu:
    srcu_read_unlock(&dbc.ch_lock, rcu_id);
    unlock_bo:
    mutex_unlock(&bo.lock);
    put_bo:
    drm_gem_object_put(obj);
    unlock_dev_srcu:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    unlock_usr_srcu:
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn empty_xfer_list(qdev: *mut qaic_device, dbc: *mut dma_bridge_chan) {
    static void empty_xfer_list(struct qaic_device *qdev, struct dma_bridge_chan *dbc)
    {
    unsigned long flags;
    struct qaic_bo *bo;
    spin_lock_irqsave(&dbc.xfer_lock, flags);
    while (!list_empty(&dbc.xfer_list)) {
    bo = list_first_entry(&dbc.xfer_list, typeof(*bo), xfer_list);
    list_del_init(&bo.xfer_list);
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    bo.nr_slice_xfer_done = 0;
    bo.req_id = 0;
    bo.perf_stats.req_received_ts = 0;
    bo.perf_stats.req_submit_ts = 0;
    bo.perf_stats.req_processed_ts = 0;
    bo.perf_stats.queue_level_before = 0;
    dma_sync_sgtable_for_cpu(&qdev.pdev.dev, bo.sgt, bo.dir);
    complete_all(&bo.xfer_done);
    drm_gem_object_put(&bo.base);
    spin_lock_irqsave(&dbc.xfer_lock, flags);
    }
    spin_unlock_irqrestore(&dbc.xfer_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sync_empty_xfer_list(qdev: *mut qaic_device, dbc: *mut dma_bridge_chan) {
    static void sync_empty_xfer_list(struct qaic_device *qdev, struct dma_bridge_chan *dbc)
    {
    empty_xfer_list(qdev, dbc);
    synchronize_srcu(&dbc.ch_lock);
//
// Threads holding channel lock, may add more elements in the xfer_list.
// Flush out these elements from xfer_list.
//
    empty_xfer_list(qdev, dbc);
    }
#[no_mangle]
pub unsafe extern "C" fn disable_dbc(qdev: *mut qaic_device, dbc_id: u32, usr: *mut qaic_user) -> c_int {
    int disable_dbc(struct qaic_device *qdev, u32 dbc_id, struct qaic_user *usr)
    {
    if (!qdev.dbc[dbc_id].usr || qdev.dbc[dbc_id].usr.handle != usr.handle)
    return -EPERM;
    qdev.dbc[dbc_id].usr = core::ptr::null_mut();
    synchronize_srcu(&qdev.dbc[dbc_id].ch_lock);
    return 0;
    }
//
// enable_dbc - Enable the DBC. DBCs are disabled by removing the context of
// user. Add user context back to DBC to enable it. This function trusts the
// DBC ID passed and expects the DBC to be disabled.
// @qdev: qaic device handle
// @dbc_id: ID of the DBC
// @usr: User context
//
#[no_mangle]
pub unsafe extern "C" fn enable_dbc(qdev: *mut qaic_device, dbc_id: u32, usr: *mut qaic_user) {
    void enable_dbc(struct qaic_device *qdev, u32 dbc_id, struct qaic_user *usr)
    {
    qdev.dbc[dbc_id].usr = usr;
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_dbc(qdev: *mut qaic_device, dbc_id: u32) {
    void wakeup_dbc(struct qaic_device *qdev, u32 dbc_id)
    {
    struct dma_bridge_chan *dbc = &qdev.dbc[dbc_id];
    dbc.usr = core::ptr::null_mut();
    sync_empty_xfer_list(qdev, dbc);
    }
#[no_mangle]
pub unsafe extern "C" fn release_dbc(qdev: *mut qaic_device, dbc_id: u32) {
    void release_dbc(struct qaic_device *qdev, u32 dbc_id)
    {
    struct qaic_bo *bo, *bo_temp;
    struct dma_bridge_chan *dbc;
    dbc = &qdev.dbc[dbc_id];
    if (!dbc.in_use)
    return;
    wakeup_dbc(qdev, dbc_id);
    dma_free_coherent(&qdev.pdev.dev, dbc.total_size, dbc.req_q_base, dbc.dma_addr);
    dbc.total_size = 0;
    dbc.req_q_base = core::ptr::null_mut();
    dbc.dma_addr = 0;
    dbc.nelem = 0;
    dbc.usr = core::ptr::null_mut();
    list_for_each_entry_safe(bo, bo_temp, &dbc.bo_lists, bo_list) {
    drm_gem_object_get(&bo.base);
    mutex_lock(&bo.lock);
    detach_slice_bo(qdev, bo);
    mutex_unlock(&bo.lock);
    drm_gem_object_put(&bo.base);
    }
    dbc.in_use = false;
    wake_up(&dbc.dbc_release);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_data_get_fifo_info(dbc: *mut dma_bridge_chan, head: *mut u32, tail: *mut u32) {
    void qaic_data_get_fifo_info(struct dma_bridge_chan *dbc, u32 *head, u32 *tail)
    {
    if (!dbc || !head || !tail)
    return;
// head = readl(dbc->dbc_base + REQHP_OFF);
// tail = readl(dbc->dbc_base + REQTP_OFF);
    }
//
// qaic_dbc_enter_ssr - Prepare to enter in sub system reset(SSR) for given DBC ID.
// @qdev: qaic device handle
// @dbc_id: ID of the DBC which will enter SSR
//
// The device will automatically deactivate the workload as not
// all errors can be silently recovered. The user will be
// notified and will need to decide the required recovery
// action to take.
//
#[no_mangle]
pub unsafe extern "C" fn qaic_dbc_enter_ssr(qdev: *mut qaic_device, dbc_id: u32) {
    void qaic_dbc_enter_ssr(struct qaic_device *qdev, u32 dbc_id)
    {
    qdev.ssr_dbc = dbc_id;
    release_dbc(qdev, dbc_id);
    }
//
// qaic_dbc_exit_ssr - Prepare to exit from sub system reset(SSR) for given DBC ID.
// @qdev: qaic device handle
//
// The DBC returns to an operational state and begins accepting work after exiting SSR.
//
#[no_mangle]
pub unsafe extern "C" fn qaic_dbc_exit_ssr(qdev: *mut qaic_device) {
    void qaic_dbc_exit_ssr(struct qaic_device *qdev)
    {
    qdev.ssr_dbc = QAIC_SSR_DBC_SENTINEL;
    }
