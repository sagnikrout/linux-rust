//! Automatically rewritten from C to Rust
//! Source: fs/fat/cache.c
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
// linux/fs/fat/cache.c
//
// Written 1992,1993 by Werner Almesberger
//
// Mar 1999. AV. Changed cache, so that it uses the starting cluster instead
// of inode number.
// May 1999. AV. Fixed the bogosity with FAT32 (read "FAT28"). Fscking lusers.
//

// this must be > 0.
pub const FAT_MAX_CACHE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_cache {
    pub cache_list: list_head,
    pub /: *mut *mut int nr_contig; / number of contiguous clusters,
    pub /: *mut *mut int fcluster; / cluster number in the file.,
    pub /: *mut *mut int dcluster; / cluster number on disk.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_cache_id {
    pub id: c_uint,
    pub nr_contig: c_int,
    pub fcluster: c_int,
    pub dcluster: c_int,
}

    static struct kmem_cache *fat_cache_cachep;
#[no_mangle]
unsafe extern "C" fn init_once(foo: *mut c_void) {
    static void init_once(void *foo)
    {
    struct fat_cache *cache = (struct fat_cache *)foo;
    INIT_LIST_HEAD(&cache.cache_list);
    }
#[no_mangle]
pub unsafe extern "C" fn fat_cache_init() -> int __init {
    int __init fat_cache_init(void)
    {
    fat_cache_cachep = kmem_cache_create("fat_cache",
    sizeof(struct fat_cache),
    0, SLAB_RECLAIM_ACCOUNT,
    init_once);
    if (fat_cache_cachep == core::ptr::null_mut())
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fat_cache_destroy() {
    void fat_cache_destroy(void)
    {
    kmem_cache_destroy(fat_cache_cachep);
    }
    static inline struct fat_cache *fat_cache_alloc(void)
    {
    return kmem_cache_alloc(fat_cache_cachep, GFP_NOFS);
    }
#[no_mangle]
pub unsafe extern "C" fn fat_cache_free(cache: *mut fat_cache) {
    static inline void fat_cache_free(struct fat_cache *cache)
    {
    BUG_ON(!list_empty(&cache.cache_list));
    kmem_cache_free(fat_cache_cachep, cache);
    }
    static inline void fat_cache_update_lru(struct inode *inode,
    struct fat_cache *cache)
    {
    if (MSDOS_I(inode).cache_lru.next != &cache.cache_list)
    list_move(&cache.cache_list, &MSDOS_I(inode).cache_lru);
    }
    static int fat_cache_lookup(struct inode *inode, int fclus,
    struct fat_cache_id *cid,
    int *cached_fclus, int *cached_dclus)
    {
    let mut nohit: static struct fat_cache = { .fcluster = 0, };
    struct fat_cache *hit = &nohit, *p;
    let mut offset: c_int = -1;
    spin_lock(&MSDOS_I(inode).cache_lru_lock);
    list_for_each_entry(p, &MSDOS_I(inode).cache_lru, cache_list) {
// Find the cache of "fclus" or nearest cache.
    if (p.fcluster <= fclus && hit.fcluster < p.fcluster) {
    hit = p;
    if ((hit.fcluster + hit.nr_contig) < fclus) {
    offset = hit.nr_contig;
    } else {
    offset = fclus - hit.fcluster;
    break;
    }
    }
    }
    if (hit != &nohit) {
    fat_cache_update_lru(inode, hit);
    cid.id = MSDOS_I(inode).cache_valid_id;
    cid.nr_contig = hit.nr_contig;
    cid.fcluster = hit.fcluster;
    cid.dcluster = hit.dcluster;
// cached_fclus = cid->fcluster + offset;
// cached_dclus = cid->dcluster + offset;
    }
    spin_unlock(&MSDOS_I(inode).cache_lru_lock);
    return offset;
    }
    static struct fat_cache *fat_cache_merge(struct inode *inode,
    struct fat_cache_id *new)
    {
    struct fat_cache *p;
    list_for_each_entry(p, &MSDOS_I(inode).cache_lru, cache_list) {
// Find the same part as "new" in cluster-chain.
    if (p.fcluster == new.fcluster) {
    BUG_ON(p.dcluster != new.dcluster);
    if (new.nr_contig > p.nr_contig)
    p.nr_contig = new.nr_contig;
    return p;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fat_cache_add(inode: *mut inode, new: *mut fat_cache_id) {
    static void fat_cache_add(struct inode *inode, struct fat_cache_id *new)
    {
    struct fat_cache *cache, *tmp;
    if (new.fcluster == -1) /* dummy cache */
    return;
    spin_lock(&MSDOS_I(inode).cache_lru_lock);
    if (new.id != FAT_CACHE_VALID &&
    new.id != MSDOS_I(inode).cache_valid_id)
    goto out;	/* this cache was invalidated */
    cache = fat_cache_merge(inode, new);
    if (cache == core::ptr::null_mut()) {
    if (MSDOS_I(inode).nr_caches < FAT_MAX_CACHE) {
    MSDOS_I(inode).nr_caches++;
    spin_unlock(&MSDOS_I(inode).cache_lru_lock);
    tmp = fat_cache_alloc();
    if (!tmp) {
    spin_lock(&MSDOS_I(inode).cache_lru_lock);
    MSDOS_I(inode).nr_caches--;
    spin_unlock(&MSDOS_I(inode).cache_lru_lock);
    return;
    }
    spin_lock(&MSDOS_I(inode).cache_lru_lock);
    cache = fat_cache_merge(inode, new);
    if (cache != core::ptr::null_mut()) {
    MSDOS_I(inode).nr_caches--;
    fat_cache_free(tmp);
    goto out_update_lru;
    }
    cache = tmp;
    } else {
    struct list_head *p = MSDOS_I(inode).cache_lru.prev;
    cache = list_entry(p, struct fat_cache, cache_list);
    }
    cache.fcluster = new.fcluster;
    cache.dcluster = new.dcluster;
    cache.nr_contig = new.nr_contig;
    }
    out_update_lru:
    fat_cache_update_lru(inode, cache);
    out:
    spin_unlock(&MSDOS_I(inode).cache_lru_lock);
    }
//
// Cache invalidation occurs rarely, thus the LRU chain is not updated. It
// fixes itself after a while.
//
#[no_mangle]
unsafe extern "C" fn __fat_cache_inval_inode(inode: *mut inode) {
    static void __fat_cache_inval_inode(struct inode *inode)
    {
    struct msdos_inode_info *i = MSDOS_I(inode);
    struct fat_cache *cache;
    while (!list_empty(&i.cache_lru)) {
    cache = list_entry(i.cache_lru.next,
    struct fat_cache, cache_list);
    list_del_init(&cache.cache_list);
    i.nr_caches--;
    fat_cache_free(cache);
    }
// Update. The copy of caches before this id is discarded.
    i.cache_valid_id++;
    if (i.cache_valid_id == FAT_CACHE_VALID)
    i.cache_valid_id++;
    }
#[no_mangle]
pub unsafe extern "C" fn fat_cache_inval_inode(inode: *mut inode) {
    void fat_cache_inval_inode(struct inode *inode)
    {
    spin_lock(&MSDOS_I(inode).cache_lru_lock);
    __fat_cache_inval_inode(inode);
    spin_unlock(&MSDOS_I(inode).cache_lru_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_contiguous(cid: *mut fat_cache_id, dclus: c_int) -> c_int {
    static inline int cache_contiguous(struct fat_cache_id *cid, int dclus)
    {
    cid.nr_contig++;
    return ((cid.dcluster + cid.nr_contig) == dclus);
    }
#[no_mangle]
pub unsafe extern "C" fn cache_init(cid: *mut fat_cache_id, fclus: c_int, dclus: c_int) {
    static inline void cache_init(struct fat_cache_id *cid, int fclus, int dclus)
    {
    cid.id = FAT_CACHE_VALID;
    cid.fcluster = fclus;
    cid.dcluster = dclus;
    cid.nr_contig = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fat_get_cluster(inode: *mut inode, cluster: c_int, fclus: *mut c_int, dclus: *mut c_int) -> c_int {
    int fat_get_cluster(struct inode *inode, int cluster, int *fclus, int *dclus)
    {
    struct super_block *sb = inode.i_sb;
    struct msdos_sb_info *sbi = MSDOS_SB(sb);
    let mut limit: c_int = sb.s_maxbytes >> sbi.cluster_bits;
    struct fat_entry fatent;
    struct fat_cache_id cid;
    int nr;
    BUG_ON(MSDOS_I(inode).i_start == 0);
// fclus = 0;
// dclus = MSDOS_I(inode)->i_start;
    if (!fat_valid_entry(sbi, *dclus)) {
    fat_fs_error_ratelimit(sb,
    "%s: invalid start cluster (i_pos %lld, start %08x)",
    __func__, MSDOS_I(inode).i_pos, *dclus);
    return -EIO;
    }
    if (cluster == 0)
    return 0;
    if (fat_cache_lookup(inode, cluster, &cid, fclus, dclus) < 0) {
//
// dummy, always not contiguous
// This is reinitialized by cache_init(), later.
//
    cache_init(&cid, -1, -1);
    }
    fatent_init(&fatent);
    while (*fclus < cluster) {
// prevent the infinite loop of cluster chain
    if (*fclus > limit) {
    fat_fs_error_ratelimit(sb,
    "%s: detected the cluster chain loop (i_pos %lld)",
    __func__, MSDOS_I(inode).i_pos);
    nr = -EIO;
    goto out;
    }
    nr = fat_ent_read(inode, &fatent, *dclus);
    if (nr < 0)
    goto out;
#[no_mangle]
pub unsafe extern "C" fn if(FAT_ENT_FREE: nr ==) -> else {
    fat_fs_error_ratelimit(sb,
    "%s: invalid cluster chain (i_pos %lld)",
    __func__, MSDOS_I(inode).i_pos);
    nr = -EIO;
    goto out;
    } else if (nr == FAT_ENT_EOF) {
    fat_cache_add(inode, &cid);
    goto out;
    }
    (*fclus)++;
// dclus = nr;
    if (!cache_contiguous(&cid, *dclus))
    cache_init(&cid, *fclus, *dclus);
    }
    nr = 0;
    fat_cache_add(inode, &cid);
    out:
    fatent_brelse(&fatent);
    return nr;
    }
#[no_mangle]
unsafe extern "C" fn fat_bmap_cluster(inode: *mut inode, cluster: c_int) -> c_int {
    static int fat_bmap_cluster(struct inode *inode, int cluster)
    {
    struct super_block *sb = inode.i_sb;
    int ret, fclus, dclus;
    if (MSDOS_I(inode).i_start == 0)
    return 0;
    ret = fat_get_cluster(inode, cluster, &fclus, &dclus);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(FAT_ENT_EOF: ret ==) -> else {
    fat_fs_error(sb, "%s: request beyond EOF (i_pos %lld)",
    __func__, MSDOS_I(inode).i_pos);
    return -EIO;
    }
    return dclus;
    }
    int fat_get_mapped_cluster(struct inode *inode, sector_t sector,
    sector_t last_block,
    unsigned long *mapped_blocks, sector_t *bmap)
    {
    struct super_block *sb = inode.i_sb;
    struct msdos_sb_info *sbi = MSDOS_SB(sb);
    int cluster, offset;
    cluster = sector >> (sbi.cluster_bits - sb.s_blocksize_bits);
    offset  = sector & (sbi.sec_per_clus - 1);
    cluster = fat_bmap_cluster(inode, cluster);
    if (cluster < 0)
    return cluster;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cluster) -> else {
// bmap = fat_clus_to_blknr(sbi, cluster) + offset;
// mapped_blocks = sbi->sec_per_clus - offset;
    if (*mapped_blocks > last_block - sector)
// mapped_blocks = last_block - sector;
    }
    return 0;
    }
    static int is_exceed_eof(struct inode *inode, sector_t sector,
    sector_t *last_block, int create)
    {
    struct super_block *sb = inode.i_sb;
    let mut blocksize: c_ulong = sb.s_blocksize;
    let mut blocksize_bits: c_uchar = sb.s_blocksize_bits;
// last_block = (i_size_read(inode) + (blocksize - 1)) >> blocksize_bits;
    if (sector >= *last_block) {
    if (!create)
    return 1;
//
// ->mmu_private can access on only allocation path.
// (caller must hold ->i_mutex)
//
// last_block = (MSDOS_I(inode)->mmu_private + (blocksize - 1))
    >> blocksize_bits;
    if (sector >= *last_block)
    return 1;
    }
    return 0;
    }
    int fat_bmap(struct inode *inode, sector_t sector, sector_t *phys,
    unsigned long *mapped_blocks, int create, bool from_bmap)
    {
    struct msdos_sb_info *sbi = MSDOS_SB(inode.i_sb);
    sector_t last_block;
// phys = 0;
// mapped_blocks = 0;
    if (!is_fat32(sbi) && (inode.i_ino == MSDOS_ROOT_INO)) {
    if (sector < (sbi.dir_entries >> sbi.dir_per_block_bits)) {
// phys = sector + sbi->dir_start;
// mapped_blocks = 1;
    }
    return 0;
    }
    if (!from_bmap) {
    if (is_exceed_eof(inode, sector, &last_block, create))
    return 0;
    } else {
    last_block = inode.i_blocks >>
    (inode.i_sb.s_blocksize_bits - 9);
    if (sector >= last_block)
    return 0;
    }
    return fat_get_mapped_cluster(inode, sector, last_block, mapped_blocks,
    phys);
    }
