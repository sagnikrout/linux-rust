//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/sw/rxe/rxe_mr.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//

// Return a random 8 bit key value that is
// different than the last_key. Set last_key to -1
// if this is the first key for an MR or MW
//
#[no_mangle]
pub unsafe extern "C" fn rxe_get_next_key(last_key: u32) -> u8 {
    u8 rxe_get_next_key(u32 last_key)
    {
    u8 key;
    do {
    get_random_bytes(&key, 1);
    } while (key == last_key);
    return key;
    }
#[no_mangle]
pub unsafe extern "C" fn mr_check_range(mr: *mut rxe_mr, iova: u64, length: usize) -> c_int {
    int mr_check_range(struct rxe_mr *mr, u64 iova, size_t length)
    {
    switch (mr.ibmr.type) {
    case IB_MR_TYPE_DMA:
    return 0;
    case IB_MR_TYPE_USER:
    case IB_MR_TYPE_MEM_REG:
    if (iova < mr.ibmr.iova ||
    iova + length > mr.ibmr.iova + mr.ibmr.length) {
    rxe_dbg_mr(mr, "iova/length out of range\n");
    return -EINVAL;
    }
    return 0;
    default:
    rxe_dbg_mr(mr, "mr type not supported\n");
    return -EINVAL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_mr_init(access: c_int, mr: *mut rxe_mr) {
    void rxe_mr_init(int access, struct rxe_mr *mr)
    {
    let mut key: u32 = mr.elem.index << 8 | rxe_get_next_key(-1);
// set ibmr->l/rkey and also copy into private l/rkey
// for user MRs these will always be the same
// for cases where caller 'owns' the key portion
// they may be different until REG_MR WQE is executed.
//
    mr.lkey = mr.ibmr.lkey = key;
    mr.rkey = mr.ibmr.rkey = key;
    mr.access = access;
    mr.ibmr.page_size = PAGE_SIZE;
    mr.page_mask = PAGE_MASK;
    mr.page_shift = PAGE_SHIFT;
    mr.state = RXE_MR_STATE_INVALID;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_mr_init_dma(access: c_int, mr: *mut rxe_mr) {
    void rxe_mr_init_dma(int access, struct rxe_mr *mr)
    {
    rxe_mr_init(access, mr);
    mr.state = RXE_MR_STATE_VALID;
    mr.ibmr.type = IB_MR_TYPE_DMA;
    }
//
// Convert iova to page_info index. The page_info stores pages of size
// PAGE_SIZE, but MRs can have different page sizes. This function
// handles the conversion for all cases:
//
// 1. mr->page_size > PAGE_SIZE:
// The MR's iova may not be aligned to mr->page_size. We use the
// aligned base (iova & page_mask) as reference, then calculate
// which PAGE_SIZE sub-page the iova falls into.
//
// 2. mr->page_size <= PAGE_SIZE:
// Use simple shift arithmetic since each page_info entry corresponds
// to one or more MR pages.
//
#[no_mangle]
unsafe extern "C" fn rxe_mr_iova_to_index(mr: *mut rxe_mr, iova: u64) -> c_ulong {
    static unsigned long rxe_mr_iova_to_index(struct rxe_mr *mr, u64 iova)
    {
    int idx;
    if (mr_page_size(mr) > PAGE_SIZE)
    idx = (iova - (mr.ibmr.iova & mr.page_mask)) >> PAGE_SHIFT;
    else
    idx = (iova >> mr.page_shift) -
    (mr.ibmr.iova >> mr.page_shift);
    WARN_ON(idx >= mr.nbuf);
    return idx;
    }
//
// Convert iova to offset within the page_info entry.
//
// For mr_page_size > PAGE_SIZE, the offset is within the system page.
// For mr_page_size <= PAGE_SIZE, the offset is within the MR page size.
//
#[no_mangle]
unsafe extern "C" fn rxe_mr_iova_to_page_offset(mr: *mut rxe_mr, iova: u64) -> c_ulong {
    static unsigned long rxe_mr_iova_to_page_offset(struct rxe_mr *mr, u64 iova)
    {
    if (mr_page_size(mr) > PAGE_SIZE)
    return iova & (PAGE_SIZE - 1);
    else
    return iova & (mr_page_size(mr) - 1);
    }
#[no_mangle]
unsafe extern "C" fn is_pmem_page(pg: *mut page) -> bool {
    static bool is_pmem_page(struct page *pg)
    {
    let mut paddr: c_ulong = page_to_phys(pg);
    return REGION_INTERSECTS ==
    region_intersects(paddr, PAGE_SIZE, IORESOURCE_MEM,
    IORES_DESC_PERSISTENT_MEMORY);
    }
#[no_mangle]
unsafe extern "C" fn rxe_mr_fill_pages_from_sgt(mr: *mut rxe_mr, sgt: *mut sg_table) -> c_int {
    static int rxe_mr_fill_pages_from_sgt(struct rxe_mr *mr, struct sg_table *sgt)
    {
    struct sg_page_iter sg_iter;
    struct page *page;
    let mut persistent: bool = !!(mr.access & IB_ACCESS_FLUSH_PERSISTENT);
    WARN_ON(mr_page_size(mr) != PAGE_SIZE);
    __sg_page_iter_start(&sg_iter, sgt.sgl, sgt.orig_nents, 0);
    if (!__sg_page_iter_next(&sg_iter))
    return 0;
    while (true) {
    page = sg_page_iter_page(&sg_iter);
    if (persistent && !is_pmem_page(page)) {
    rxe_dbg_mr(mr, "Page can't be persistent\n");
    return -EINVAL;
    }
    mr.page_info[mr.nbuf].page = page;
    mr.page_info[mr.nbuf].offset = 0;
    mr.nbuf++;
    if (!__sg_page_iter_next(&sg_iter))
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __alloc_mr_page_info(mr: *mut rxe_mr, num_pages: c_int) -> c_int {
    static int __alloc_mr_page_info(struct rxe_mr *mr, int num_pages)
    {
    mr.page_info = kzalloc_objs(struct rxe_mr_page, num_pages);
    if (!mr.page_info)
    return -ENOMEM;
    mr.max_allowed_buf = num_pages;
    mr.nbuf = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alloc_mr_page_info(mr: *mut rxe_mr, num_pages: c_int) -> c_int {
    static int alloc_mr_page_info(struct rxe_mr *mr, int num_pages)
    {
    int ret;
    WARN_ON(mr.num_buf);
    ret = __alloc_mr_page_info(mr, num_pages);
    if (ret)
    return ret;
    mr.num_buf = num_pages;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_mr_page_info(mr: *mut rxe_mr) {
    static void free_mr_page_info(struct rxe_mr *mr)
    {
    if (!mr.page_info)
    return;
    kfree(mr.page_info);
    mr.page_info = core::ptr::null_mut();
    }
    int rxe_mr_init_user(struct rxe_dev *rxe, u64 start, u64 length,
    int access, struct rxe_mr *mr)
    {
    struct ib_umem *umem;
    int err;
    rxe_mr_init(access, mr);
    umem = ib_umem_get_va(&rxe.ib_dev, start, length, access);
    if (IS_ERR(umem)) {
    rxe_dbg_mr(mr, "Unable to pin memory region err = %d\n",
    (int)PTR_ERR(umem));
    return PTR_ERR(umem);
    }
    err = alloc_mr_page_info(mr, ib_umem_num_pages(umem));
    if (err)
    goto err2;
    err = rxe_mr_fill_pages_from_sgt(mr, &umem.sgt_append.sgt);
    if (err)
    goto err1;
    mr.umem = umem;
    mr.ibmr.type = IB_MR_TYPE_USER;
    mr.state = RXE_MR_STATE_VALID;
    return 0;
    err1:
    free_mr_page_info(mr);
    err2:
    ib_umem_release(umem);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_mr_init_fast(max_pages: c_int, mr: *mut rxe_mr) -> c_int {
    int rxe_mr_init_fast(int max_pages, struct rxe_mr *mr)
    {
    int err;
// always allow remote access for FMRs
    rxe_mr_init(RXE_ACCESS_REMOTE, mr);
    err = alloc_mr_page_info(mr, max_pages);
    if (err)
    goto err1;
    mr.state = RXE_MR_STATE_FREE;
    mr.ibmr.type = IB_MR_TYPE_MEM_REG;
    return 0;
    err1:
    return err;
    }
//
// I) MRs with page_size >= PAGE_SIZE,
// Split a large MR page (mr->page_size) into multiple PAGE_SIZE
// sub-pages and store them in page_info, offset is always 0.
//
// Called when mr->page_size > PAGE_SIZE. Each call to rxe_set_page()
// represents one mr->page_size region, which we must split into
// (mr->page_size >> PAGE_SHIFT) individual pages.
//
// II) MRs with page_size < PAGE_SIZE,
// Save each PAGE_SIZE page and its offset within the system page in page_info.
//
#[no_mangle]
unsafe extern "C" fn rxe_set_page(ibmr: *mut ib_mr, dma_addr: u64) -> c_int {
    static int rxe_set_page(struct ib_mr *ibmr, u64 dma_addr)
    {
    struct rxe_mr *mr = to_rmr(ibmr);
    let mut persistent: bool = !!(mr.access & IB_ACCESS_FLUSH_PERSISTENT);
    u32 i, pages_per_mr = mr_page_size(mr) >> PAGE_SHIFT;
    pages_per_mr = MAX(1, pages_per_mr);
    for (i = 0; i < pages_per_mr; i++) {
    let mut addr: u64 = dma_addr + i * PAGE_SIZE;
    struct page *sub_page = ib_virt_dma_to_page(addr);
    if (unlikely(mr.nbuf >= mr.max_allowed_buf))
    return -ENOMEM;
    if (persistent && !is_pmem_page(sub_page)) {
    rxe_dbg_mr(mr, "Page cannot be persistent\n");
    return -EINVAL;
    }
    mr.page_info[mr.nbuf].page = sub_page;
    mr.page_info[mr.nbuf].offset = addr & (PAGE_SIZE - 1);
    mr.nbuf++;
    }
    return 0;
    }
    int rxe_map_mr_sg(struct ib_mr *ibmr, struct scatterlist *sgl,
    int sg_nents, unsigned int *sg_offset)
    {
    struct rxe_mr *mr = to_rmr(ibmr);
    let mut page_size: c_uint = mr_page_size(mr);
//
// Ensure page_size and PAGE_SIZE are compatible for mapping.
// We require one to be a multiple of the other for correct
// iova-to-page conversion.
//
    if (!IS_ALIGNED(page_size, PAGE_SIZE) &&
    !IS_ALIGNED(PAGE_SIZE, page_size)) {
    rxe_dbg_mr(mr, "MR page size %u must be compatible with PAGE_SIZE %lu\n",
    page_size, PAGE_SIZE);
    return -EINVAL;
    }
    if (mr_page_size(mr) > PAGE_SIZE) {
// resize page_info if needed
    let mut map_mr_pages: u32 = (page_size >> PAGE_SHIFT) * mr.num_buf;
    if (map_mr_pages > mr.max_allowed_buf) {
    rxe_dbg_mr(mr, "requested pages %u exceed max %u\n",
    map_mr_pages, mr.max_allowed_buf);
    free_mr_page_info(mr);
    if (__alloc_mr_page_info(mr, map_mr_pages))
    return -ENOMEM;
    }
    }
    mr.nbuf = 0;
    mr.page_shift = ilog2(page_size);
    mr.page_mask = ~((u64)page_size - 1);
    return ib_sg_to_pages(ibmr, sgl, sg_nents, sg_offset, rxe_set_page);
    }
    static int rxe_mr_copy_xarray(struct rxe_mr *mr, u64 iova, void *addr,
    unsigned int length, enum rxe_mr_copy_dir dir)
    {
    unsigned int bytes;
    u8 *va;
    while (length) {
    let mut index: c_ulong = rxe_mr_iova_to_index(mr, iova);
    struct rxe_mr_page *info = &mr.page_info[index];
    let mut page_offset: c_uint = rxe_mr_iova_to_page_offset(mr, iova);
    if (!info.page)
    return -EFAULT;
    page_offset += info.offset;
    bytes = min_t(unsigned int, length, PAGE_SIZE - page_offset);
    va = kmap_local_page(info.page);
    if (dir == RXE_FROM_MR_OBJ)
    memcpy(addr, va + page_offset, bytes);
    else
    memcpy(va + page_offset, addr, bytes);
    kunmap_local(va);
    addr += bytes;
    iova += bytes;
    length -= bytes;
    }
    return 0;
    }
    static void rxe_mr_copy_dma(struct rxe_mr *mr, u64 dma_addr, void *addr,
    unsigned int length, enum rxe_mr_copy_dir dir)
    {
    let mut page_offset: c_uint = dma_addr & (PAGE_SIZE - 1);
    unsigned int bytes;
    struct page *page;
    u8 *va;
    while (length) {
    page = ib_virt_dma_to_page(dma_addr);
    bytes = min_t(unsigned int, length,
    PAGE_SIZE - page_offset);
    va = kmap_local_page(page);
    if (dir == RXE_TO_MR_OBJ)
    memcpy(va + page_offset, addr, bytes);
    else
    memcpy(addr, va + page_offset, bytes);
    kunmap_local(va);
    page_offset = 0;
    dma_addr += bytes;
    addr += bytes;
    length -= bytes;
    }
    }
    int rxe_mr_copy(struct rxe_mr *mr, u64 iova, void *addr,
    unsigned int length, enum rxe_mr_copy_dir dir)
    {
    int err;
    if (length == 0)
    return 0;
    if (WARN_ON(!mr))
    return -EINVAL;
    if (mr.ibmr.type == IB_MR_TYPE_DMA) {
    rxe_mr_copy_dma(mr, iova, addr, length, dir);
    return 0;
    }
    err = mr_check_range(mr, iova, length);
    if (unlikely(err)) {
    rxe_dbg_mr(mr, "iova out of range\n");
    return err;
    }
    if (is_odp_mr(mr))
    return rxe_odp_mr_copy(mr, iova, addr, length, dir);
    else
    return rxe_mr_copy_xarray(mr, iova, addr, length, dir);
    }
// copy data in or out of a wqe, i.e. sg list
// under the control of a dma descriptor
//
    int copy_data(
    struct rxe_pd		*pd,
    int			access,
    struct rxe_dma_info	*dma,
    void			*addr,
    int			length,
    enum rxe_mr_copy_dir	dir)
    {
    int			bytes;
    struct rxe_sge		*sge	= &dma.sge[dma.cur_sge];
    let mut offset: c_int = dma.sge_offset;
    let mut resid: c_int = dma.resid;
    struct rxe_mr		*mr	= core::ptr::null_mut();
    u64			iova;
    int			err;
    if (length == 0)
    return 0;
    if (length > resid) {
    err = -EINVAL;
    goto err2;
    }
    if (sge.length && (offset < sge.length)) {
    mr = lookup_mr(pd, access, sge.lkey, RXE_LOOKUP_LOCAL);
    if (!mr) {
    err = -EINVAL;
    goto err1;
    }
    }
    while (length > 0) {
    bytes = length;
    if (offset >= sge.length) {
    if (mr) {
    rxe_put(mr);
    mr = core::ptr::null_mut();
    }
    sge++;
    dma.cur_sge++;
    offset = 0;
    if (dma.cur_sge >= dma.num_sge) {
    err = -ENOSPC;
    goto err2;
    }
    if (sge.length) {
    mr = lookup_mr(pd, access, sge.lkey,
    RXE_LOOKUP_LOCAL);
    if (!mr) {
    err = -EINVAL;
    goto err1;
    }
    } else {
    continue;
    }
    }
    if (bytes > sge.length - offset)
    bytes = sge.length - offset;
    if (bytes > 0) {
    iova = sge.addr + offset;
    err = rxe_mr_copy(mr, iova, addr, bytes, dir);
    if (err)
    goto err2;
    offset	+= bytes;
    resid	-= bytes;
    length	-= bytes;
    addr	+= bytes;
    }
    }
    dma.sge_offset = offset;
    dma.resid	= resid;
    if (mr)
    rxe_put(mr);
    return 0;
    err2:
    if (mr)
    rxe_put(mr);
    err1:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rxe_mr_flush_pmem_iova(mr: *mut rxe_mr, iova: u64, length: c_uint) -> c_int {
    static int rxe_mr_flush_pmem_iova(struct rxe_mr *mr, u64 iova, unsigned int length)
    {
    unsigned int bytes;
    int err;
    u8 *va;
    err = mr_check_range(mr, iova, length);
    if (err)
    return err;
    while (length > 0) {
    let mut index: c_ulong = rxe_mr_iova_to_index(mr, iova);
    struct rxe_mr_page *info = &mr.page_info[index];
    let mut page_offset: c_uint = rxe_mr_iova_to_page_offset(mr, iova);
    if (!info.page)
    return -EFAULT;
    page_offset += info.offset;
    bytes = min_t(unsigned int, length, PAGE_SIZE - page_offset);
    va = kmap_local_page(info.page);
    arch_wb_cache_pmem(va + page_offset, bytes);
    kunmap_local(va);
    length -= bytes;
    iova += bytes;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_flush_pmem_iova(mr: *mut rxe_mr, start: u64, length: c_uint) -> c_int {
    int rxe_flush_pmem_iova(struct rxe_mr *mr, u64 start, unsigned int length)
    {
    int err;
// mr must be valid even if length is zero
    if (WARN_ON(!mr))
    return -EINVAL;
    if (length == 0)
    return 0;
    if (mr.ibmr.type == IB_MR_TYPE_DMA)
    return -EFAULT;
    if (is_odp_mr(mr))
    err = rxe_odp_flush_pmem_iova(mr, start, length);
    else
    err = rxe_mr_flush_pmem_iova(mr, start, length);
    return err;
    }
// Guarantee atomicity of atomic operations at the machine level.
    DEFINE_SPINLOCK(atomic_ops_lock);
    enum resp_states rxe_mr_do_atomic_op(struct rxe_mr *mr, u64 iova, int opcode,
    u64 compare, u64 swap_add, u64 *orig_val)
    {
    unsigned int page_offset;
    struct page *page;
    u64 value;
    u64 *va;
    if (unlikely(mr.state != RXE_MR_STATE_VALID)) {
    rxe_dbg_mr(mr, "mr not in valid state\n");
    return RESPST_ERR_RKEY_VIOLATION;
    }
    if (mr.ibmr.type == IB_MR_TYPE_DMA) {
    page_offset = iova & (PAGE_SIZE - 1);
    page = ib_virt_dma_to_page(iova);
    } else {
    unsigned long index;
    int err;
    struct rxe_mr_page *info;
    err = mr_check_range(mr, iova, sizeof(value));
    if (err) {
    rxe_dbg_mr(mr, "iova out of range\n");
    return RESPST_ERR_RKEY_VIOLATION;
    }
    page_offset = rxe_mr_iova_to_page_offset(mr, iova);
    index = rxe_mr_iova_to_index(mr, iova);
    info = &mr.page_info[index];
    if (!info.page)
    return RESPST_ERR_RKEY_VIOLATION;
    page_offset += info.offset;
    page = info.page;
    }
    if (unlikely(page_offset & 0x7)) {
    rxe_dbg_mr(mr, "iova not aligned\n");
    return RESPST_ERR_MISALIGNED_ATOMIC;
    }
    va = kmap_local_page(page);
    spin_lock_bh(&atomic_ops_lock);
    value = *orig_val = va[page_offset >> 3];
    if (opcode == IB_OPCODE_RC_COMPARE_SWAP) {
    if (value == compare)
    va[page_offset >> 3] = swap_add;
    } else {
    value += swap_add;
    va[page_offset >> 3] = value;
    }
    spin_unlock_bh(&atomic_ops_lock);
    kunmap_local(va);
    return RESPST_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_mr_do_atomic_write(mr: *mut rxe_mr, iova: u64, value: u64) -> enum resp_states {
    enum resp_states rxe_mr_do_atomic_write(struct rxe_mr *mr, u64 iova, u64 value)
    {
    unsigned int page_offset;
    struct page *page;
    u64 *va;
    if (mr.ibmr.type == IB_MR_TYPE_DMA) {
    page_offset = iova & (PAGE_SIZE - 1);
    page = ib_virt_dma_to_page(iova);
    } else {
    unsigned long index;
    int err;
    struct rxe_mr_page *info;
// See IBA oA19-28
    err = mr_check_range(mr, iova, sizeof(value));
    if (unlikely(err)) {
    rxe_dbg_mr(mr, "iova out of range\n");
    return RESPST_ERR_RKEY_VIOLATION;
    }
    page_offset = rxe_mr_iova_to_page_offset(mr, iova);
    index = rxe_mr_iova_to_index(mr, iova);
    info = &mr.page_info[index];
    if (!info.page)
    return RESPST_ERR_RKEY_VIOLATION;
    page_offset += info.offset;
    page = info.page;
    }
// See IBA A19.4.2
    if (unlikely(page_offset & 0x7)) {
    rxe_dbg_mr(mr, "misaligned address\n");
    return RESPST_ERR_MISALIGNED_ATOMIC;
    }
    va = kmap_local_page(page);
// Do atomic write after all prior operations have completed
    smp_store_release(&va[page_offset >> 3], value);
    kunmap_local(va);
    return RESPST_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn advance_dma_data(dma: *mut rxe_dma_info, length: c_uint) -> c_int {
    int advance_dma_data(struct rxe_dma_info *dma, unsigned int length)
    {
    struct rxe_sge		*sge	= &dma.sge[dma.cur_sge];
    let mut offset: c_int = dma.sge_offset;
    let mut resid: c_int = dma.resid;
    while (length) {
    unsigned int bytes;
    if (offset >= sge.length) {
    sge++;
    dma.cur_sge++;
    offset = 0;
    if (dma.cur_sge >= dma.num_sge)
    return -ENOSPC;
    }
    bytes = length;
    if (bytes > sge.length - offset)
    bytes = sge.length - offset;
    offset	+= bytes;
    resid	-= bytes;
    length	-= bytes;
    }
    dma.sge_offset = offset;
    dma.resid	= resid;
    return 0;
    }
    struct rxe_mr *lookup_mr(struct rxe_pd *pd, int access, u32 key,
    enum rxe_mr_lookup_type type)
    {
    struct rxe_mr *mr;
    struct rxe_dev *rxe = to_rdev(pd.ibpd.device);
    let mut index: c_int = key >> 8;
    mr = rxe_pool_get_index(&rxe.mr_pool, index);
    if (!mr)
    return core::ptr::null_mut();
    if (unlikely((type == RXE_LOOKUP_LOCAL && mr.lkey != key) ||
    (type == RXE_LOOKUP_REMOTE && mr.rkey != key) ||
    mr_pd(mr) != pd || ((access & mr.access) != access) ||
    mr.state != RXE_MR_STATE_VALID)) {
    rxe_put(mr);
    mr = core::ptr::null_mut();
    }
    return mr;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_invalidate_mr(qp: *mut rxe_qp, key: u32) -> c_int {
    int rxe_invalidate_mr(struct rxe_qp *qp, u32 key)
    {
    struct rxe_dev *rxe = to_rdev(qp.ibqp.device);
    struct rxe_mr *mr;
    int remote;
    int ret;
    mr = rxe_pool_get_index(&rxe.mr_pool, key >> 8);
    if (!mr) {
    rxe_dbg_qp(qp, "No MR for key %#x\n", key);
    ret = -EINVAL;
    goto err;
    }
    remote = mr.access & RXE_ACCESS_REMOTE;
    if (remote ? (key != mr.rkey) : (key != mr.lkey)) {
    rxe_dbg_mr(mr, "wr key (%#x) doesn't match mr key (%#x)\n",
    key, (remote ? mr.rkey : mr.lkey));
    ret = -EINVAL;
    goto err_drop_ref;
    }
    if (atomic_read(&mr.num_mw) > 0) {
    rxe_dbg_mr(mr, "Attempt to invalidate an MR while bound to MWs\n");
    ret = -EINVAL;
    goto err_drop_ref;
    }
    if (unlikely(mr.ibmr.type != IB_MR_TYPE_MEM_REG)) {
    rxe_dbg_mr(mr, "Type (%d) is wrong\n", mr.ibmr.type);
    ret = -EINVAL;
    goto err_drop_ref;
    }
    mr.state = RXE_MR_STATE_FREE;
    ret = 0;
    err_drop_ref:
    rxe_put(mr);
    err:
    return ret;
    }
// user can (re)register fast MR by executing a REG_MR WQE.
// user is expected to hold a reference on the ib mr until the
// WQE completes.
// Once a fast MR is created this is the only way to change the
// private keys. It is the responsibility of the user to maintain
// the ib mr keys in sync with rxe mr keys.
//
#[no_mangle]
pub unsafe extern "C" fn rxe_reg_fast_mr(qp: *mut rxe_qp, wqe: *mut rxe_send_wqe) -> c_int {
    int rxe_reg_fast_mr(struct rxe_qp *qp, struct rxe_send_wqe *wqe)
    {
    struct rxe_mr *mr = to_rmr(wqe.wr.wr.reg.mr);
    let mut key: u32 = wqe.wr.wr.reg.key;
    let mut access: u32 = wqe.wr.wr.reg.access;
// user can only register MR in free state
    if (unlikely(mr.state != RXE_MR_STATE_FREE)) {
    rxe_dbg_mr(mr, "mr.lkey = 0x%x not free\n", mr.lkey);
    return -EINVAL;
    }
// user can only register mr with qp in same protection domain
    if (unlikely(qp.ibqp.pd != mr.ibmr.pd)) {
    rxe_dbg_mr(mr, "qp.pd and mr.pd don't match\n");
    return -EINVAL;
    }
// user is only allowed to change key portion of l/rkey
    if (unlikely((mr.lkey & ~0xff) != (key & ~0xff))) {
    rxe_dbg_mr(mr, "key = 0x%x has wrong index mr.lkey = 0x%x\n",
    key, mr.lkey);
    return -EINVAL;
    }
    mr.access = access;
    mr.lkey = key;
    mr.rkey = key;
    mr.ibmr.iova = wqe.wr.wr.reg.mr.iova;
    mr.state = RXE_MR_STATE_VALID;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rxe_mr_cleanup(elem: *mut rxe_pool_elem) {
    void rxe_mr_cleanup(struct rxe_pool_elem *elem)
    {
    struct rxe_mr *mr = container_of(elem, typeof(*mr), elem);
    rxe_put(mr_pd(mr));
    ib_umem_release(mr.umem);
    if (mr.ibmr.type != IB_MR_TYPE_DMA)
    free_mr_page_info(mr);
    }
