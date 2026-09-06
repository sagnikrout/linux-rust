//! Automatically rewritten from C to Rust
//! Source: fs/jffs2/malloc.c
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
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

// These are initialised to NULL in the kernel startup code.
    If you're porting to other operating systems, beware */
    static struct kmem_cache *full_dnode_slab;
    static struct kmem_cache *raw_dirent_slab;
    static struct kmem_cache *raw_inode_slab;
    static struct kmem_cache *tmp_dnode_info_slab;
    static struct kmem_cache *raw_node_ref_slab;
    static struct kmem_cache *node_frag_slab;
    static struct kmem_cache *inode_cache_slab;

    static struct kmem_cache *xattr_datum_cache;
    static struct kmem_cache *xattr_ref_cache;

#[no_mangle]
pub unsafe extern "C" fn jffs2_create_slab_caches() -> int __init {
    int __init jffs2_create_slab_caches(void)
    {
    full_dnode_slab = KMEM_CACHE(jffs2_full_dnode, 0);
    if (!full_dnode_slab)
    goto err;
    raw_dirent_slab = KMEM_CACHE(jffs2_raw_dirent, SLAB_HWCACHE_ALIGN);
    if (!raw_dirent_slab)
    goto err;
    raw_inode_slab = KMEM_CACHE(jffs2_raw_inode, SLAB_HWCACHE_ALIGN);
    if (!raw_inode_slab)
    goto err;
    tmp_dnode_info_slab = KMEM_CACHE(jffs2_tmp_dnode_info, 0);
    if (!tmp_dnode_info_slab)
    goto err;
    raw_node_ref_slab = kmem_cache_create("jffs2_refblock",
    sizeof(struct jffs2_raw_node_ref) * (REFS_PER_BLOCK + 1),
    0, 0, core::ptr::null_mut());
    if (!raw_node_ref_slab)
    goto err;
    node_frag_slab = KMEM_CACHE(jffs2_node_frag, 0);
    if (!node_frag_slab)
    goto err;
    inode_cache_slab = KMEM_CACHE(jffs2_inode_cache, 0);
    if (!inode_cache_slab)
    goto err;

    xattr_datum_cache = KMEM_CACHE(jffs2_xattr_datum, 0);
    if (!xattr_datum_cache)
    goto err;
    xattr_ref_cache = KMEM_CACHE(jffs2_xattr_ref, 0);
    if (!xattr_ref_cache)
    goto err;

    return 0;
    err:
    jffs2_destroy_slab_caches();
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_destroy_slab_caches() {
    void jffs2_destroy_slab_caches(void)
    {
    kmem_cache_destroy(full_dnode_slab);
    kmem_cache_destroy(raw_dirent_slab);
    kmem_cache_destroy(raw_inode_slab);
    kmem_cache_destroy(tmp_dnode_info_slab);
    kmem_cache_destroy(raw_node_ref_slab);
    kmem_cache_destroy(node_frag_slab);
    kmem_cache_destroy(inode_cache_slab);

    kmem_cache_destroy(xattr_datum_cache);
    kmem_cache_destroy(xattr_ref_cache);

    }
    struct jffs2_full_dirent *jffs2_alloc_full_dirent(int namesize)
    {
    struct jffs2_full_dirent *ret;
    ret = kmalloc(sizeof(struct jffs2_full_dirent) + namesize, GFP_KERNEL);
    dbg_memalloc("%p\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_full_dirent(x: *mut jffs2_full_dirent) {
    void jffs2_free_full_dirent(struct jffs2_full_dirent *x)
    {
    dbg_memalloc("%p\n", x);
    kfree(x);
    }
    struct jffs2_full_dnode *jffs2_alloc_full_dnode(void)
    {
    struct jffs2_full_dnode *ret;
    ret = kmem_cache_alloc(full_dnode_slab, GFP_KERNEL);
    dbg_memalloc("%p\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_full_dnode(x: *mut jffs2_full_dnode) {
    void jffs2_free_full_dnode(struct jffs2_full_dnode *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(full_dnode_slab, x);
    }
    struct jffs2_raw_dirent *jffs2_alloc_raw_dirent(void)
    {
    struct jffs2_raw_dirent *ret;
    ret = kmem_cache_alloc(raw_dirent_slab, GFP_KERNEL);
    dbg_memalloc("%p\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_raw_dirent(x: *mut jffs2_raw_dirent) {
    void jffs2_free_raw_dirent(struct jffs2_raw_dirent *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(raw_dirent_slab, x);
    }
    struct jffs2_raw_inode *jffs2_alloc_raw_inode(void)
    {
    struct jffs2_raw_inode *ret;
    ret = kmem_cache_alloc(raw_inode_slab, GFP_KERNEL);
    dbg_memalloc("%p\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_raw_inode(x: *mut jffs2_raw_inode) {
    void jffs2_free_raw_inode(struct jffs2_raw_inode *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(raw_inode_slab, x);
    }
    struct jffs2_tmp_dnode_info *jffs2_alloc_tmp_dnode_info(void)
    {
    struct jffs2_tmp_dnode_info *ret;
    ret = kmem_cache_alloc(tmp_dnode_info_slab, GFP_KERNEL);
    dbg_memalloc("%p\n",
    ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_tmp_dnode_info(x: *mut jffs2_tmp_dnode_info) {
    void jffs2_free_tmp_dnode_info(struct jffs2_tmp_dnode_info *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(tmp_dnode_info_slab, x);
    }
    static struct jffs2_raw_node_ref *jffs2_alloc_refblock(void)
    {
    struct jffs2_raw_node_ref *ret;
    ret = kmem_cache_alloc(raw_node_ref_slab, GFP_KERNEL);
    if (ret) {
    let mut i: c_int = 0;
    for (i=0; i < REFS_PER_BLOCK; i++) {
    ret[i].flash_offset = REF_EMPTY_NODE;
    ret[i].next_in_ino = core::ptr::null_mut();
    }
    ret[i].flash_offset = REF_LINK_NODE;
    ret[i].next_in_ino = core::ptr::null_mut();
    }
    return ret;
    }
    int jffs2_prealloc_raw_node_refs(struct jffs2_sb_info *c,
    struct jffs2_eraseblock *jeb, int nr)
    {
    struct jffs2_raw_node_ref **p, *ref;
    let mut i: c_int = nr;
    dbg_memalloc("%d\n", nr);
    p = &jeb.last_node;
    ref = *p;
    dbg_memalloc("Reserving %d refs for block @0x%08x\n", nr, jeb.offset);
// If jeb->last_node is really a valid node then skip over it
    if (ref && ref.flash_offset != REF_EMPTY_NODE)
    ref++;
    while (i) {
    if (!ref) {
    dbg_memalloc("Allocating new refblock linked from %p\n", p);
    ref = *p = jffs2_alloc_refblock();
    if (!ref)
    return -ENOMEM;
    }
    if (ref.flash_offset == REF_LINK_NODE) {
    p = &ref.next_in_ino;
    ref = *p;
    continue;
    }
    i--;
    ref++;
    }
    jeb.allocated_refs = nr;
    dbg_memalloc("Reserved %d refs for block @0x%08x, last_node is %p (%08x,%p)\n",
    nr, jeb.offset, jeb.last_node, jeb.last_node.flash_offset,
    jeb.last_node.next_in_ino);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_refblock(x: *mut jffs2_raw_node_ref) {
    void jffs2_free_refblock(struct jffs2_raw_node_ref *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(raw_node_ref_slab, x);
    }
    struct jffs2_node_frag *jffs2_alloc_node_frag(void)
    {
    struct jffs2_node_frag *ret;
    ret = kmem_cache_alloc(node_frag_slab, GFP_KERNEL);
    dbg_memalloc("%p\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_node_frag(x: *mut jffs2_node_frag) {
    void jffs2_free_node_frag(struct jffs2_node_frag *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(node_frag_slab, x);
    }
    struct jffs2_inode_cache *jffs2_alloc_inode_cache(void)
    {
    struct jffs2_inode_cache *ret;
    ret = kmem_cache_alloc(inode_cache_slab, GFP_KERNEL);
    dbg_memalloc("%p\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_inode_cache(x: *mut jffs2_inode_cache) {
    void jffs2_free_inode_cache(struct jffs2_inode_cache *x)
    {
    dbg_memalloc("%p\n", x);
    kmem_cache_free(inode_cache_slab, x);
    }

    struct jffs2_xattr_datum *jffs2_alloc_xattr_datum(void)
    {
    struct jffs2_xattr_datum *xd;
    xd = kmem_cache_zalloc(xattr_datum_cache, GFP_KERNEL);
    dbg_memalloc("%p\n", xd);
    if (!xd)
    return core::ptr::null_mut();
    xd.class = RAWNODE_CLASS_XATTR_DATUM;
    xd.node = (void *)xd;
    INIT_LIST_HEAD(&xd.xindex);
    return xd;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_xattr_datum(xd: *mut jffs2_xattr_datum) {
    void jffs2_free_xattr_datum(struct jffs2_xattr_datum *xd)
    {
    dbg_memalloc("%p\n", xd);
    kmem_cache_free(xattr_datum_cache, xd);
    }
    struct jffs2_xattr_ref *jffs2_alloc_xattr_ref(void)
    {
    struct jffs2_xattr_ref *ref;
    ref = kmem_cache_zalloc(xattr_ref_cache, GFP_KERNEL);
    dbg_memalloc("%p\n", ref);
    if (!ref)
    return core::ptr::null_mut();
    ref.class = RAWNODE_CLASS_XATTR_REF;
    ref.node = (void *)ref;
    return ref;
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_free_xattr_ref(ref: *mut jffs2_xattr_ref) {
    void jffs2_free_xattr_ref(struct jffs2_xattr_ref *ref)
    {
    dbg_memalloc("%p\n", ref);
    kmem_cache_free(xattr_ref_cache, ref);
    }
