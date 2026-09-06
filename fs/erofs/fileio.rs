//! Automatically rewritten from C to Rust
//! Source: fs/erofs/fileio.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2024, Alibaba Cloud
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_fileio_rq {
    pub bvecs: [bio_vec; 16],
    pub bio: bio,
    pub iocb: kiocb,
    pub sb: *mut super_block,
    pub ref: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_fileio {
    pub map: erofs_map_blocks,
    pub dev: erofs_map_dev,
    pub rq: *mut erofs_fileio_rq,
}

#[no_mangle]
unsafe extern "C" fn erofs_fileio_ki_complete(iocb: *mut kiocb, ret: c_long) {
    static void erofs_fileio_ki_complete(struct kiocb *iocb, long ret)
    {
    struct erofs_fileio_rq *rq =
    container_of(iocb, struct erofs_fileio_rq, iocb);
    struct folio_iter fi;
    if (ret >= 0 && ret != rq.bio.bi_iter.bi_size)
    ret = -EIO;
    if (!rq.bio.bi_end_io) {
    bio_for_each_folio_all(fi, &rq.bio) {
    DBG_BUGON(folio_test_uptodate(fi.folio));
    erofs_onlinefolio_end(fi.folio, ret < 0, false);
    }
    } else if (ret < 0 && !rq.bio.bi_status) {
    rq.bio.bi_status = errno_to_blk_status(ret);
    }
    bio_endio(&rq.bio);
    bio_uninit(&rq.bio);
    if (refcount_dec_and_test(&rq.ref))
    kfree(rq);
    }
#[no_mangle]
unsafe extern "C" fn erofs_fileio_rq_submit(rq: *mut erofs_fileio_rq) {
    static void erofs_fileio_rq_submit(struct erofs_fileio_rq *rq)
    {
    struct iov_iter iter;
    ssize_t ret;
    if (!rq)
    return;
    rq.iocb.ki_pos = rq.bio.bi_iter.bi_sector << SECTOR_SHIFT;
    rq.iocb.ki_ioprio = get_current_ioprio();
    rq.iocb.ki_complete = erofs_fileio_ki_complete;
    if (test_opt(&EROFS_SB(rq.sb).opt, DIRECT_IO) &&
    rq.iocb.ki_filp.f_mode & FMODE_CAN_ODIRECT)
    rq.iocb.ki_flags = IOCB_DIRECT;
    iov_iter_bvec(&iter, ITER_DEST, rq.bvecs, rq.bio.bi_vcnt,
    rq.bio.bi_iter.bi_size);
    scoped_with_creds(rq.iocb.ki_filp.f_cred)
    ret = vfs_iocb_iter_read(rq.iocb.ki_filp, &rq.iocb, &iter);
    if (ret != -EIOCBQUEUED)
    erofs_fileio_ki_complete(&rq.iocb, ret);
    if (refcount_dec_and_test(&rq.ref))
    kfree(rq);
    }
    static struct erofs_fileio_rq *erofs_fileio_rq_alloc(struct erofs_map_dev *mdev)
    {
    struct erofs_fileio_rq *rq = kzalloc_obj(*rq, GFP_KERNEL | __GFP_NOFAIL);
    bio_init(&rq.bio, core::ptr::null_mut(), rq.bvecs, ARRAY_SIZE(rq.bvecs), REQ_OP_READ);
    rq.iocb.ki_filp = mdev.m_dif.file;
    rq.sb = mdev.m_sb;
    refcount_set(&rq.ref, 2);
    return rq;
    }
    struct bio *erofs_fileio_bio_alloc(struct erofs_map_dev *mdev)
    {
    return &erofs_fileio_rq_alloc(mdev).bio;
    }
#[no_mangle]
pub unsafe extern "C" fn erofs_fileio_submit_bio(bio: *mut bio) {
    void erofs_fileio_submit_bio(struct bio *bio)
    {
    return erofs_fileio_rq_submit(container_of(bio, struct erofs_fileio_rq,
    bio));
    }
    static int erofs_fileio_scan_folio(struct erofs_fileio *io,
    struct inode *inode, struct folio *folio)
    {
    struct erofs_map_blocks *map = &io.map;
    let mut cur: c_uint = 0, end = folio_size(folio), len, attached = 0;
    let mut pos: loff_t = folio_pos(folio), ofs;
    let mut err: c_int = 0;
    erofs_onlinefolio_init(folio);
    while (cur < end) {
    if (!in_range(pos + cur, map.m_la, map.m_llen)) {
    map.m_la = pos + cur;
    map.m_llen = end - cur;
    err = erofs_map_blocks(inode, map);
    if (err)
    break;
    }
    ofs = folio_pos(folio) + cur - map.m_la;
    len = min_t(loff_t, map.m_llen - ofs, end - cur);
    if (map.m_flags & EROFS_MAP_META) {
    let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
    void *src;
    src = erofs_read_metabuf(&buf, inode.i_sb,
    map.m_pa + ofs, erofs_inode_in_metabox(inode));
    if (IS_ERR(src)) {
    err = PTR_ERR(src);
    break;
    }
    memcpy_to_folio(folio, cur, src, len);
    erofs_put_metabuf(&buf);
    } else if (!(map.m_flags & EROFS_MAP_MAPPED)) {
    folio_zero_segment(folio, cur, cur + len);
    attached = 0;
    } else {
    if (io.rq && (map.m_pa + ofs != io.dev.m_pa ||
    map.m_deviceid != io.dev.m_deviceid)) {
    io_retry:
    erofs_fileio_rq_submit(io.rq);
    io.rq = core::ptr::null_mut();
    }
    if (!io.rq) {
    io.dev = (struct erofs_map_dev) {
    .m_pa = io.map.m_pa + ofs,
    .m_deviceid = io.map.m_deviceid,
    };
    err = erofs_map_dev(inode.i_sb, &io.dev);
    if (err)
    break;
    io.rq = erofs_fileio_rq_alloc(&io.dev);
    io.rq.bio.bi_iter.bi_sector =
    (io.dev.m_dif.fsoff + io.dev.m_pa) >> 9;
    attached = 0;
    }
    if (!bio_add_folio(&io.rq.bio, folio, len, cur))
    goto io_retry;
    if (!attached++)
    erofs_onlinefolio_split(folio);
    io.dev.m_pa += len;
    }
    cur += len;
    }
    erofs_onlinefolio_end(folio, err, false);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn erofs_fileio_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    static int erofs_fileio_read_folio(struct file *file, struct folio *folio)
    {
    bool need_iput;
    struct inode *realinode = erofs_real_inode(folio_inode(folio), &need_iput);
    let mut io: erofs_fileio = {};
    int err;
    trace_erofs_read_folio(realinode, folio, true);
    err = erofs_fileio_scan_folio(&io, realinode, folio);
    erofs_fileio_rq_submit(io.rq);
    if (need_iput)
    iput(realinode);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn erofs_fileio_readahead(rac: *mut readahead_control) {
    static void erofs_fileio_readahead(struct readahead_control *rac)
    {
    bool need_iput;
    struct inode *realinode = erofs_real_inode(rac.mapping.host, &need_iput);
    let mut io: erofs_fileio = {};
    struct folio *folio;
    int err;
    trace_erofs_readahead(realinode, readahead_index(rac),
    readahead_count(rac), true);
    while ((folio = readahead_folio(rac))) {
    err = erofs_fileio_scan_folio(&io, realinode, folio);
    if (err && err != -EINTR)
    erofs_err(realinode.i_sb, "readahead error at folio %lu @ nid %llu",
    folio.index, EROFS_I(realinode).nid);
    }
    erofs_fileio_rq_submit(io.rq);
    if (need_iput)
    iput(realinode);
    }
    const struct address_space_operations erofs_fileio_aops = {
    .read_folio = erofs_fileio_read_folio,
    .readahead = erofs_fileio_readahead,
    };
#[no_mangle]
pub unsafe extern "C" fn erofs_read_meta_folio(file: *mut file, folio: *mut folio) -> c_int {
    int erofs_read_meta_folio(struct file *file, struct folio *folio)
    {
    struct erofs_fileio io = {
    .dev = { .m_pa = folio_pos(folio), },
    };
    struct inode *inode = folio_inode(folio);
    int err;
    err = erofs_map_dev(inode.i_sb, &io.dev);
    if (err)
    return err;
    io.rq = erofs_fileio_rq_alloc(&io.dev);
    io.rq.bio.bi_iter.bi_sector =
    (io.dev.m_dif.fsoff + io.dev.m_pa) >> 9;
    erofs_onlinefolio_init(folio);
    bio_add_folio_nofail(&io.rq.bio, folio, folio_size(folio), 0);
    erofs_fileio_rq_submit(io.rq);
    return 0;
    }
