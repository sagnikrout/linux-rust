//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/hisilicon/sgl.c
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
// Copyright (c) 2019 HiSilicon Limited.

pub const HISI_ACC_SGL_SGE_NR_MIN: c_int = 1;
pub const HISI_ACC_SGL_NR_MAX: c_int = 256;
pub const HISI_ACC_SGL_ALIGN_SIZE: c_int = 64;
pub const HISI_ACC_MEM_BLOCK_NR: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acc_hw_sge {
    pub buf: dma_addr_t,
    pub page_ctrl: *mut c_void,
    pub len: __le32,
    pub pad: __le32,
    pub pad0: __le32,
    pub pad1: __le32,
}

// use default sgl head size 64B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_acc_hw_sgl {
    pub next_dma: dma_addr_t,
    pub entry_sum_in_chain: __le16,
    pub entry_sum_in_sgl: __le16,
    pub entry_length_in_sgl: __le16,
    pub pad0: __le16,
    pub pad1: [__le64; 5],
    pub next: *mut hisi_acc_hw_sgl,
    pub sge_entries: [acc_hw_sge; ],
    pub __aligned(1): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_acc_sgl_pool {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_block {
    pub sgl: *mut hisi_acc_hw_sgl,
    pub sgl_dma: dma_addr_t,
    pub size: usize,
    pub mem_block: [}; HISI_ACC_MEM_BLOCK_NR],
    pub sgl_num_per_block: u32,
    pub block_num: u32,
    pub count: u32,
    pub sge_nr: u32,
    pub sgl_size: usize,
}

//
// hisi_acc_create_sgl_pool() - Create a hw sgl pool.
// @dev: The device which hw sgl pool belongs to.
// @count: Count of hisi_acc_hw_sgl in pool.
// @sge_nr: The count of sge in hw_sgl
//
// This function creates a hw sgl pool, after this user can get hw sgl memory
// from it.
//
    struct hisi_acc_sgl_pool *hisi_acc_create_sgl_pool(struct device *dev,
    u32 count, u32 sge_nr)
    {
    u32 sgl_size, block_size, sgl_num_per_block, block_num, remain_sgl;
    struct hisi_acc_sgl_pool *pool;
    struct mem_block *block;
    u32 i, j;
    if (!dev || !count || !sge_nr || sge_nr > HISI_ACC_SGL_SGE_NR_MAX)
    return ERR_PTR(-EINVAL);
    sgl_size = ALIGN(sizeof(struct acc_hw_sge) * sge_nr +
    sizeof(struct hisi_acc_hw_sgl),
    HISI_ACC_SGL_ALIGN_SIZE);
//
// the pool may allocate a block of memory of size PAGE_SIZE * 2^MAX_PAGE_ORDER,
// block size may exceed 2^31 on ia64, so the max of block size is 2^31
//
    block_size = 1 << (PAGE_SHIFT + MAX_PAGE_ORDER < 32 ?
    PAGE_SHIFT + MAX_PAGE_ORDER : 31);
    sgl_num_per_block = block_size / sgl_size;
    block_num = count / sgl_num_per_block;
    remain_sgl = count % sgl_num_per_block;
    if ((!remain_sgl && block_num > HISI_ACC_MEM_BLOCK_NR) ||
    (remain_sgl > 0 && block_num > HISI_ACC_MEM_BLOCK_NR - 1))
    return ERR_PTR(-EINVAL);
    pool = kzalloc_obj(*pool);
    if (!pool)
    return ERR_PTR(-ENOMEM);
    block = pool.mem_block;
    for (i = 0; i < block_num; i++) {
    block[i].sgl = dma_alloc_coherent(dev, block_size,
    &block[i].sgl_dma,
    GFP_KERNEL);
    if (!block[i].sgl) {
    dev_err(dev, "Fail to allocate hw SG buffer!\n");
    goto err_free_mem;
    }
    block[i].size = block_size;
    }
    if (remain_sgl > 0) {
    block[i].sgl = dma_alloc_coherent(dev, remain_sgl * sgl_size,
    &block[i].sgl_dma,
    GFP_KERNEL);
    if (!block[i].sgl) {
    dev_err(dev, "Fail to allocate remained hw SG buffer!\n");
    goto err_free_mem;
    }
    block[i].size = remain_sgl * sgl_size;
    }
    pool.sgl_num_per_block = sgl_num_per_block;
    pool.block_num = remain_sgl ? block_num + 1 : block_num;
    pool.count = count;
    pool.sgl_size = sgl_size;
    pool.sge_nr = sge_nr;
    return pool;
    err_free_mem:
    for (j = 0; j < i; j++)
    dma_free_coherent(dev, block_size, block[j].sgl,
    block[j].sgl_dma);
    kfree_sensitive(pool);
    return ERR_PTR(-ENOMEM);
    }
    EXPORT_SYMBOL_GPL(hisi_acc_create_sgl_pool);
//
// hisi_acc_free_sgl_pool() - Free a hw sgl pool.
// @dev: The device which hw sgl pool belongs to.
// @pool: Pointer of pool.
//
// This function frees memory of a hw sgl pool.
//
#[no_mangle]
pub unsafe extern "C" fn hisi_acc_free_sgl_pool(dev: *mut device, pool: *mut hisi_acc_sgl_pool) {
    void hisi_acc_free_sgl_pool(struct device *dev, struct hisi_acc_sgl_pool *pool)
    {
    struct mem_block *block;
    u32 i;
    if (!dev || !pool)
    return;
    block = pool.mem_block;
    for (i = 0; i < pool.block_num; i++)
    dma_free_coherent(dev, block[i].size, block[i].sgl,
    block[i].sgl_dma);
    kfree(pool);
    }
    EXPORT_SYMBOL_GPL(hisi_acc_free_sgl_pool);
    static struct hisi_acc_hw_sgl *acc_get_sgl(struct hisi_acc_sgl_pool *pool,
    u32 index, dma_addr_t *hw_sgl_dma)
    {
    struct mem_block *block;
    u32 block_index, offset;
    block = pool.mem_block;
    block_index = index / pool.sgl_num_per_block;
    offset = index % pool.sgl_num_per_block;
// hw_sgl_dma = block[block_index].sgl_dma + pool->sgl_size * offset;
    return (void *)block[block_index].sgl + pool.sgl_size * offset;
    }
    static void sg_map_to_hw_sg(struct scatterlist *sgl,
    struct acc_hw_sge *hw_sge)
    {
    hw_sge.buf = sg_dma_address(sgl);
    hw_sge.len = cpu_to_le32(sg_dma_len(sgl));
    hw_sge.page_ctrl = sg_virt(sgl);
    }
#[no_mangle]
unsafe extern "C" fn inc_hw_sgl_sge(hw_sgl: *mut hisi_acc_hw_sgl) {
    static void inc_hw_sgl_sge(struct hisi_acc_hw_sgl *hw_sgl)
    {
    let mut var: u16 = le16_to_cpu(hw_sgl.entry_sum_in_sgl);
    var++;
    hw_sgl.entry_sum_in_sgl = cpu_to_le16(var);
    }
#[no_mangle]
unsafe extern "C" fn update_hw_sgl_sum_sge(hw_sgl: *mut hisi_acc_hw_sgl, sum: u16) {
    static void update_hw_sgl_sum_sge(struct hisi_acc_hw_sgl *hw_sgl, u16 sum)
    {
    hw_sgl.entry_sum_in_chain = cpu_to_le16(sum);
    }
#[no_mangle]
unsafe extern "C" fn clear_hw_sgl_sge(hw_sgl: *mut hisi_acc_hw_sgl) {
    static void clear_hw_sgl_sge(struct hisi_acc_hw_sgl *hw_sgl)
    {
    struct acc_hw_sge *hw_sge = hw_sgl.sge_entries;
    let mut entry_sum: u16 = le16_to_cpu(hw_sgl.entry_sum_in_sgl);
    int i;
    for (i = 0; i < entry_sum; i++) {
    hw_sge[i].page_ctrl = core::ptr::null_mut();
    hw_sge[i].buf = 0;
    hw_sge[i].len = 0;
    }
    }
//
// hisi_acc_sg_buf_map_to_hw_sgl - Map a scatterlist to a hw sgl.
// @dev: The device which hw sgl belongs to.
// @sgl: Scatterlist which will be mapped to hw sgl.
// @pool: Pool which hw sgl memory will be allocated in.
// @index: Index of hisi_acc_hw_sgl in pool.
// @hw_sgl_dma: The dma address of allocated hw sgl.
// @dir: DMA direction.
//
// This function builds hw sgl according input sgl, user can use hw_sgl_dma
// as src/dst in its BD. Only support single hw sgl currently.
//
    struct hisi_acc_hw_sgl *
    hisi_acc_sg_buf_map_to_hw_sgl(struct device *dev, struct scatterlist *sgl,
    struct hisi_acc_sgl_pool *pool, u32 index,
    dma_addr_t *hw_sgl_dma, enum dma_data_direction dir)
    {
    struct hisi_acc_hw_sgl *curr_hw_sgl;
    unsigned int i, sg_n_mapped;
    let mut curr_sgl_dma: dma_addr_t = 0;
    struct acc_hw_sge *curr_hw_sge;
    struct scatterlist *sg;
    int sg_n, ret;
    if (!dev || !sgl || !pool || !hw_sgl_dma || index >= pool.count)
    return ERR_PTR(-EINVAL);
    sg_n = sg_nents(sgl);
    sg_n_mapped = dma_map_sg(dev, sgl, sg_n, dir);
    if (!sg_n_mapped) {
    dev_err(dev, "DMA mapping for SG error!\n");
    return ERR_PTR(-EINVAL);
    }
    if (sg_n_mapped > pool.sge_nr) {
    dev_err(dev, "the number of entries in input scatterlist is bigger than SGL pool setting.\n");
    ret = -EINVAL;
    goto err_unmap;
    }
    curr_hw_sgl = acc_get_sgl(pool, index, &curr_sgl_dma);
    curr_hw_sgl.entry_length_in_sgl = cpu_to_le16(pool.sge_nr);
    curr_hw_sge = curr_hw_sgl.sge_entries;
    for_each_sg(sgl, sg, sg_n_mapped, i) {
    sg_map_to_hw_sg(sg, curr_hw_sge);
    inc_hw_sgl_sge(curr_hw_sgl);
    curr_hw_sge++;
    }
    update_hw_sgl_sum_sge(curr_hw_sgl, pool.sge_nr);
// hw_sgl_dma = curr_sgl_dma;
    return curr_hw_sgl;
    err_unmap:
    dma_unmap_sg(dev, sgl, sg_n, dir);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(hisi_acc_sg_buf_map_to_hw_sgl);
//
// hisi_acc_sg_buf_unmap() - Unmap allocated hw sgl.
// @dev: The device which hw sgl belongs to.
// @sgl: Related scatterlist.
// @hw_sgl: Virtual address of hw sgl.
// @dir: DMA direction.
//
// This function unmaps allocated hw sgl.
//
    void hisi_acc_sg_buf_unmap(struct device *dev, struct scatterlist *sgl,
    struct hisi_acc_hw_sgl *hw_sgl, enum dma_data_direction dir)
    {
    if (!dev || !sgl || !hw_sgl)
    return;
    dma_unmap_sg(dev, sgl, sg_nents(sgl), dir);
    clear_hw_sgl_sge(hw_sgl);
    hw_sgl.entry_sum_in_chain = 0;
    hw_sgl.entry_sum_in_sgl = 0;
    hw_sgl.entry_length_in_sgl = 0;
    }
    EXPORT_SYMBOL_GPL(hisi_acc_sg_buf_unmap);
