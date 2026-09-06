//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx5/core/alloc.c
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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

    (PAGE_SHIFT - MLX5_FRAG_BUF_POOL_MIN_BLOCK_SHIFT + 1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_dma_pool {
// Protects page_list and per-page allocation bitmaps.
    pub lock: mutex,
    pub page_list: list_head,
    pub dev: *mut mlx5_core_dev,
    pub node: c_int,
    pub block_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_dma_pool_page {
    pub pool: *mut mlx5_dma_pool,
    pub pool_link: list_head,
    pub bitmap: *mut c_ulong,
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_frag_buf_node_pools {
    pub pools: [*mut mlx5_dma_pool; MLX5_FRAG_BUF_POOLS_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_dma_pool_stats {
    pub node: c_int,
    pub block_size: usize,
    pub used_blocks: usize,
    pub allocated_blocks: usize,
}

// Handling for queue buffers -- we allocate a bunch of memory and
// register it in a memory region at HCA virtual address 0.
//
    static void *mlx5_dma_zalloc_coherent_node(struct mlx5_core_dev *dev,
    size_t size, dma_addr_t *dma_handle,
    int node)
    {
    struct device *device = mlx5_core_dma_dev(dev);
    struct mlx5_priv *priv = &dev.priv;
    int original_node;
    void *cpu_handle;
    mutex_lock(&priv.alloc_mutex);
    original_node = dev_to_node(device);
    set_dev_node(device, node);
    cpu_handle = dma_alloc_coherent(device, size, dma_handle,
    GFP_KERNEL);
    set_dev_node(device, original_node);
    mutex_unlock(&priv.alloc_mutex);
    return cpu_handle;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_dma_pool_destroy(pool: *mut mlx5_dma_pool) {
    static void mlx5_dma_pool_destroy(struct mlx5_dma_pool *pool)
    {
    mutex_destroy(&pool.lock);
    kfree(pool);
    }
    static struct mlx5_dma_pool *mlx5_dma_pool_create(struct mlx5_core_dev *dev,
    int node, u8 block_shift)
    {
    struct mlx5_dma_pool *pool;
    pool = kzalloc_obj(*pool);
    if (!pool)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&pool.page_list);
    mutex_init(&pool.lock);
    pool.dev = dev;
    pool.node = node;
    pool.block_shift = block_shift;
    return pool;
    }
    static struct mlx5_dma_pool_page *
    mlx5_dma_pool_page_alloc(struct mlx5_dma_pool *pool)
    {
    let mut blocks_per_page: c_int = BIT(PAGE_SHIFT - pool.block_shift);
    struct mlx5_dma_pool_page *page;
    page = kzalloc_obj(*page);
    if (!page)
    goto err_out;
    page.pool = pool;
    page.bitmap = bitmap_zalloc(blocks_per_page, GFP_KERNEL);
    if (!page.bitmap)
    goto err_free_page;
    bitmap_fill(page.bitmap, blocks_per_page);
    page.buf = mlx5_dma_zalloc_coherent_node(pool.dev, PAGE_SIZE,
    &page.dma, pool.node);
    if (!page.buf)
    goto err_free_bitmap;
    return page;
    err_free_bitmap:
    bitmap_free(page.bitmap);
    err_free_page:
    kfree(page);
    err_out:
    return core::ptr::null_mut();
    }
    static void mlx5_dma_pool_page_free(struct mlx5_core_dev *dev,
    struct mlx5_dma_pool_page *page)
    {
    dma_free_coherent(mlx5_core_dma_dev(dev), PAGE_SIZE, page.buf,
    page.dma);
    bitmap_free(page.bitmap);
    kfree(page);
    }
    static int mlx5_dma_pool_alloc_from_page(struct mlx5_dma_pool *pool,
    struct mlx5_dma_pool_page *page,
    unsigned long *idx_out)
    {
    let mut blocks_per_page: c_int = BIT(PAGE_SHIFT - pool.block_shift);
// idx_out = find_first_bit(page->bitmap, blocks_per_page);
    if (*idx_out >= blocks_per_page)
    return -ENOMEM;
    __clear_bit(*idx_out, page.bitmap);
    if (bitmap_empty(page.bitmap, blocks_per_page))
    list_move_tail(&page.pool_link, &pool.page_list);
    return 0;
    }
    static struct mlx5_dma_pool_page *
    mlx5_dma_pool_alloc(struct mlx5_dma_pool *pool, unsigned long *idx_out)
    {
    struct mlx5_dma_pool_page *page;
    mutex_lock(&pool.lock);
    page = list_first_entry_or_null(&pool.page_list,
    struct mlx5_dma_pool_page, pool_link);
    if (page && !mlx5_dma_pool_alloc_from_page(pool, page, idx_out))
    goto unlock; /* successfully allocated from existing page */
    page = mlx5_dma_pool_page_alloc(pool);
    if (!page)
    goto unlock;
    list_add(&page.pool_link, &pool.page_list);
    mlx5_dma_pool_alloc_from_page(pool, page, idx_out);
    unlock:
    mutex_unlock(&pool.lock);
    return page;
    }
    static void mlx5_dma_pool_free(struct mlx5_dma_pool *pool,
    struct mlx5_dma_pool_page *page,
    unsigned long idx)
    {
    let mut blocks_per_page: c_int = BIT(PAGE_SHIFT - pool.block_shift);
    bool was_full;
    mutex_lock(&pool.lock);
    was_full = bitmap_empty(page.bitmap, blocks_per_page);
    __set_bit(idx, page.bitmap);
    if (bitmap_full(page.bitmap, blocks_per_page)) {
    list_del(&page.pool_link);
    mlx5_dma_pool_page_free(pool.dev, page);
    } else {
    memset((u8 *)page.buf + (idx << pool.block_shift), 0,
    BIT(pool.block_shift));
    if (was_full)
    list_move(&page.pool_link, &pool.page_list);
    }
    mutex_unlock(&pool.lock);
    }
    static void mlx5_dma_pool_debugfs_get_stats(struct mlx5_dma_pool *pool,
    struct mlx5_dma_pool_stats *stats)
    {
    let mut blocks_per_page: c_int = BIT(PAGE_SHIFT - pool.block_shift);
    struct mlx5_dma_pool_page *page;
    let mut free_blocks: usize = 0;
    let mut pages: usize = 0;
    mutex_lock(&pool.lock);
    list_for_each_entry(page, &pool.page_list, pool_link) {
    pages++;
    free_blocks += bitmap_weight(page.bitmap, blocks_per_page);
    }
    mutex_unlock(&pool.lock);
    stats.node = pool.node;
    stats.block_size = BIT(pool.block_shift);
    stats.allocated_blocks = pages * blocks_per_page;
    stats.used_blocks = stats.allocated_blocks - free_blocks;
    }
    static void mlx5_dma_pool_debugfs_stats_print(struct seq_file *file,
    struct mlx5_dma_pool *pool)
    {
    let mut stats: mlx5_dma_pool_stats = {};
    mlx5_dma_pool_debugfs_get_stats(pool, &stats);
    seq_printf(file, "%4d       %5zu      %7zu           %7zu\n",
    stats.node, stats.block_size, stats.used_blocks,
    stats.allocated_blocks);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_dma_pools_debugfs_print_header(file: *mut seq_file) {
    static void mlx5_dma_pools_debugfs_print_header(struct seq_file *file)
    {
    seq_puts(file, "node  block_size  used_blocks  allocated_blocks\n");
    }
    static void
    mlx5_frag_buf_node_pools_destroy(struct mlx5_frag_buf_node_pools *node_pools)
    {
    for (int i = 0; i < MLX5_FRAG_BUF_POOLS_NUM; i++)
    if (node_pools.pools[i])
    mlx5_dma_pool_destroy(node_pools.pools[i]);
    kfree(node_pools);
    }
    static struct mlx5_frag_buf_node_pools *
    mlx5_frag_buf_node_pools_create(struct mlx5_core_dev *dev, int node)
    {
    struct mlx5_frag_buf_node_pools *node_pools;
    node_pools = kzalloc_obj(*node_pools);
    if (!node_pools)
    return core::ptr::null_mut();
    for (int i = 0; i < MLX5_FRAG_BUF_POOLS_NUM; i++) {
    let mut block_shift: u8 = MLX5_FRAG_BUF_POOL_MIN_BLOCK_SHIFT + i;
    node_pools.pools[i] = mlx5_dma_pool_create(dev, node,
    block_shift);
    if (!node_pools.pools[i]) {
    mlx5_frag_buf_node_pools_destroy(node_pools);
    return core::ptr::null_mut();
    }
    }
    return node_pools;
    }
    static int
    mlx5_frag_buf_dma_pools_debugfs_show(struct seq_file *file, void *priv)
    {
    struct mlx5_core_dev *dev = file.private;
    int node;
    mlx5_dma_pools_debugfs_print_header(file);
    if (!dev.priv.frag_buf_node_pools)
    return 0;
    for_each_node_state(node, N_POSSIBLE) {
    struct mlx5_frag_buf_node_pools *node_pools;
    node_pools = dev.priv.frag_buf_node_pools[node];
    if (!node_pools)
    continue;
    for (int i = 0; i < MLX5_FRAG_BUF_POOLS_NUM; i++) {
    struct mlx5_dma_pool *pool = node_pools.pools[i];
    if (!pool)
    continue;
    mlx5_dma_pool_debugfs_stats_print(file, pool);
    }
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(mlx5_frag_buf_dma_pools_debugfs);
#[no_mangle]
pub unsafe extern "C" fn mlx5_frag_buf_pools_cleanup(dev: *mut mlx5_core_dev) {
    void mlx5_frag_buf_pools_cleanup(struct mlx5_core_dev *dev)
    {
    struct mlx5_priv *priv = &dev.priv;
    int node;
    debugfs_remove(priv.dbg.frag_buf_dma_pools_debugfs);
    priv.dbg.frag_buf_dma_pools_debugfs = core::ptr::null_mut();
    for_each_node_state(node, N_POSSIBLE) {
    struct mlx5_frag_buf_node_pools *node_pools;
    node_pools = priv.frag_buf_node_pools[node];
    if (!node_pools)
    continue;
    mlx5_frag_buf_node_pools_destroy(node_pools);
    }
    kfree(priv.frag_buf_node_pools);
    priv.frag_buf_node_pools = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_frag_buf_pools_init(dev: *mut mlx5_core_dev) -> c_int {
    int mlx5_frag_buf_pools_init(struct mlx5_core_dev *dev)
    {
    struct mlx5_priv *priv = &dev.priv;
    int node;
    priv.frag_buf_node_pools = kzalloc_objs(*priv.frag_buf_node_pools,
    nr_node_ids);
    if (!priv.frag_buf_node_pools)
    return -ENOMEM;
    for_each_node_state(node, N_POSSIBLE) {
    struct mlx5_frag_buf_node_pools *node_pools;
    node_pools = mlx5_frag_buf_node_pools_create(dev, node);
    if (!node_pools) {
    mlx5_frag_buf_pools_cleanup(dev);
    return -ENOMEM;
    }
    priv.frag_buf_node_pools[node] = node_pools;
    }
    priv.dbg.frag_buf_dma_pools_debugfs =
    debugfs_create_file("frag_buf_dma_pools", 0444,
    priv.dbg.dbg_root, dev,
    &mlx5_frag_buf_dma_pools_debugfs_fops);
    return 0;
    }
    int mlx5_frag_buf_alloc_node(struct mlx5_core_dev *dev, int size,
    struct mlx5_frag_buf *buf, int node)
    {
    struct mlx5_dma_pool *pool;
    int pool_idx;
    node = node == NUMA_NO_NODE ? numa_mem_id() : node;
    buf.size = size;
    buf.npages = DIV_ROUND_UP(size, PAGE_SIZE);
    buf.page_shift = clamp_t(int, order_base_2(size),
    MLX5_FRAG_BUF_POOL_MIN_BLOCK_SHIFT,
    PAGE_SHIFT);
    buf.frags = kcalloc_node(buf.npages, sizeof(*buf.frags),
    GFP_KERNEL, node);
    if (!buf.frags)
    return -ENOMEM;
    pool_idx = buf.page_shift - MLX5_FRAG_BUF_POOL_MIN_BLOCK_SHIFT;
    pool = dev.priv.frag_buf_node_pools[node].pools[pool_idx];
    for (int i = 0; i < buf.npages; i++) {
    struct mlx5_buf_list *frag = &buf.frags[i];
    struct mlx5_dma_pool_page *page;
    unsigned long idx;
    page = mlx5_dma_pool_alloc(pool, &idx);
    if (!page) {
    mlx5_frag_buf_free(dev, buf);
    return -ENOMEM;
    }
    frag.buf = (u8 *)page.buf + (idx << pool.block_shift);
    frag.map = page.dma + (idx << pool.block_shift);
    frag.frag_page = page;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(mlx5_frag_buf_alloc_node);
#[no_mangle]
pub unsafe extern "C" fn mlx5_frag_buf_free(dev: *mut mlx5_core_dev, buf: *mut mlx5_frag_buf) {
    void mlx5_frag_buf_free(struct mlx5_core_dev *dev, struct mlx5_frag_buf *buf)
    {
    for (int i = 0; i < buf.npages; i++) {
    struct mlx5_buf_list *frag = &buf.frags[i];
    struct mlx5_dma_pool_page *page;
    struct mlx5_dma_pool *pool;
    unsigned long idx;
    if (!frag.buf)
    continue;
    page = frag.frag_page;
    pool = page.pool;
    idx = (frag.map - page.dma) >> pool.block_shift;
    mlx5_dma_pool_free(pool, page, idx);
    }
    kfree(buf.frags);
    }
    EXPORT_SYMBOL_GPL(mlx5_frag_buf_free);
#[no_mangle]
unsafe extern "C" fn mlx5_db_dma_pools_debugfs_show(file: *mut seq_file, priv: *mut c_void) -> c_int {
    static int mlx5_db_dma_pools_debugfs_show(struct seq_file *file, void *priv)
    {
    struct mlx5_core_dev *dev = file.private;
    int node;
    mlx5_dma_pools_debugfs_print_header(file);
    for_each_node_state(node, N_POSSIBLE) {
    struct mlx5_dma_pool *pool = dev.priv.db_node_pools[node];
    if (!pool)
    continue;
    mlx5_dma_pool_debugfs_stats_print(file, pool);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(mlx5_db_dma_pools_debugfs);
#[no_mangle]
pub unsafe extern "C" fn mlx5_db_pools_cleanup(dev: *mut mlx5_core_dev) {
    void mlx5_db_pools_cleanup(struct mlx5_core_dev *dev)
    {
    struct mlx5_priv *priv = &dev.priv;
    int node;
    debugfs_remove(priv.dbg.db_dma_pools_debugfs);
    priv.dbg.db_dma_pools_debugfs = core::ptr::null_mut();
    for_each_node_state(node, N_POSSIBLE)
    if (priv.db_node_pools[node])
    mlx5_dma_pool_destroy(priv.db_node_pools[node]);
    kfree(priv.db_node_pools);
    priv.db_node_pools = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_db_pools_init(dev: *mut mlx5_core_dev) -> c_int {
    int mlx5_db_pools_init(struct mlx5_core_dev *dev)
    {
    struct mlx5_priv *priv = &dev.priv;
    int node;
    priv.db_node_pools = kzalloc_objs(*priv.db_node_pools, nr_node_ids);
    if (!priv.db_node_pools)
    return -ENOMEM;
    for_each_node_state(node, N_POSSIBLE) {
    struct mlx5_dma_pool *pool;
    pool = mlx5_dma_pool_create(dev, node,
    order_base_2(cache_line_size()));
    if (!pool) {
    mlx5_db_pools_cleanup(dev);
    return -ENOMEM;
    }
    priv.db_node_pools[node] = pool;
    }
    priv.dbg.db_dma_pools_debugfs =
    debugfs_create_file("db_dma_pools", 0444, priv.dbg.dbg_root,
    dev, &mlx5_db_dma_pools_debugfs_fops);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_db_alloc_node(dev: *mut mlx5_core_dev, db: *mut mlx5_db, node: c_int) -> c_int {
    int mlx5_db_alloc_node(struct mlx5_core_dev *dev, struct mlx5_db *db, int node)
    {
    struct mlx5_dma_pool_page *page;
    struct mlx5_dma_pool *pool;
    unsigned long idx;
    int offset;
    node = node == NUMA_NO_NODE ? numa_mem_id() : node;
    pool = dev.priv.db_node_pools[node];
    page = mlx5_dma_pool_alloc(pool, &idx);
    if (!page)
    return -ENOMEM;
    offset = idx << pool.block_shift;
    db.u.pool_page = page;
    db.index = idx;
    db.db = (__be32 *)((u8 *)page.buf + offset);
    db.dma = page.dma + offset;
    return 0;
    }
    EXPORT_SYMBOL_GPL(mlx5_db_alloc_node);
#[no_mangle]
pub unsafe extern "C" fn mlx5_db_free(dev: *mut mlx5_core_dev, db: *mut mlx5_db) {
    void mlx5_db_free(struct mlx5_core_dev *dev, struct mlx5_db *db)
    {
    struct mlx5_dma_pool_page *page = db.u.pool_page;
    struct mlx5_dma_pool *pool = page.pool;
    mlx5_dma_pool_free(pool, page, db.index);
    }
    EXPORT_SYMBOL_GPL(mlx5_db_free);
#[no_mangle]
pub unsafe extern "C" fn mlx5_fill_page_frag_array_perm(buf: *mut mlx5_frag_buf, pas: *mut __be64, perm: u8) {
    void mlx5_fill_page_frag_array_perm(struct mlx5_frag_buf *buf, __be64 *pas, u8 perm)
    {
    int i;
    WARN_ON(perm & 0xfc);
    for (i = 0; i < buf.npages; i++)
    pas[i] = cpu_to_be64(buf.frags[i].map | perm);
    }
    EXPORT_SYMBOL_GPL(mlx5_fill_page_frag_array_perm);
#[no_mangle]
pub unsafe extern "C" fn mlx5_fill_page_frag_array(buf: *mut mlx5_frag_buf, pas: *mut __be64) {
    void mlx5_fill_page_frag_array(struct mlx5_frag_buf *buf, __be64 *pas)
    {
    mlx5_fill_page_frag_array_perm(buf, pas, 0);
    }
    EXPORT_SYMBOL_GPL(mlx5_fill_page_frag_array);
