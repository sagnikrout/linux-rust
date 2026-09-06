//! Automatically rewritten from C to Rust
//! Source: fs/fuse/notify.c
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

    static int fuse_notify_poll(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_poll_wakeup_out outarg;
    int err;
    if (size != sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    fuse_copy_finish(cs);
    return fuse_notify_poll_wakeup(fc, &outarg);
    }
    static int fuse_notify_inval_inode(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_inval_inode_out outarg;
    int err;
    if (size != sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    fuse_copy_finish(cs);
    down_read(&fc.killsb);
    err = fuse_reverse_inval_inode(fc, outarg.ino,
    outarg.off, outarg.len);
    up_read(&fc.killsb);
    return err;
    }
    static int fuse_notify_inval_entry(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_inval_entry_out outarg;
    int err;
    char *buf;
    struct qstr name;
    if (size < sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    if (outarg.namelen > fc.name_max)
    return -ENAMETOOLONG;
    err = -EINVAL;
    if (size != sizeof(outarg) + outarg.namelen + 1)
    return -EINVAL;
    buf = kzalloc(outarg.namelen + 1, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    name.name = buf;
    name.len = outarg.namelen;
    err = fuse_copy_one(cs, buf, outarg.namelen + 1);
    if (err)
    goto err;
    fuse_copy_finish(cs);
    buf[outarg.namelen] = 0;
    down_read(&fc.killsb);
    err = fuse_reverse_inval_entry(fc, outarg.parent, 0, &name, outarg.flags);
    up_read(&fc.killsb);
    err:
    kfree(buf);
    return err;
    }
    static int fuse_notify_delete(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_delete_out outarg;
    int err;
    char *buf;
    struct qstr name;
    if (size < sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    if (outarg.namelen > fc.name_max)
    return -ENAMETOOLONG;
    if (size != sizeof(outarg) + outarg.namelen + 1)
    return -EINVAL;
    buf = kzalloc(outarg.namelen + 1, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    name.name = buf;
    name.len = outarg.namelen;
    err = fuse_copy_one(cs, buf, outarg.namelen + 1);
    if (err)
    goto err;
    fuse_copy_finish(cs);
    buf[outarg.namelen] = 0;
    down_read(&fc.killsb);
    err = fuse_reverse_inval_entry(fc, outarg.parent, outarg.child, &name, 0);
    up_read(&fc.killsb);
    err:
    kfree(buf);
    return err;
    }
    static int fuse_notify_store(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_store_out outarg;
    struct inode *inode;
    struct address_space *mapping;
    u64 nodeid;
    int err;
    unsigned int num;
    loff_t file_size;
    loff_t pos;
    loff_t end;
    if (size < sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    if (size - sizeof(outarg) != outarg.size)
    return -EINVAL;
    if (outarg.offset >= MAX_LFS_FILESIZE)
    return -EINVAL;
    nodeid = outarg.nodeid;
    pos = outarg.offset;
    num = min(outarg.size, MAX_LFS_FILESIZE - pos);
    down_read(&fc.killsb);
    err = -ENOENT;
    inode = fuse_ilookup(fc, nodeid,  core::ptr::null_mut());
    if (!inode)
    goto out_up_killsb;
    if (!S_ISREG(inode.i_mode)) {
    err = -EINVAL;
    goto out_iput;
    }
    mapping = inode.i_mapping;
    file_size = i_size_read(inode);
    end = pos + num;
    if (end > file_size) {
    file_size = end;
    fuse_write_update_attr(inode, file_size, num);
    }
    while (num) {
    struct folio *folio;
    unsigned int folio_offset;
    unsigned int nr_bytes;
    let mut index: pgoff_t = pos >> PAGE_SHIFT;
    folio = filemap_grab_folio(mapping, index);
    err = PTR_ERR(folio);
    if (IS_ERR(folio))
    goto out_iput;
    folio_offset = offset_in_folio(folio, pos);
    nr_bytes = min(num, folio_size(folio) - folio_offset);
    err = fuse_copy_folio(cs, &folio, folio_offset, nr_bytes, 0);
    if (!folio_test_uptodate(folio) && !err && folio_offset == 0 &&
    (nr_bytes == folio_size(folio) || file_size == end)) {
    folio_zero_segment(folio, nr_bytes, folio_size(folio));
    iomap_folio_mark_uptodate(folio);
    }
    folio_unlock(folio);
    folio_put(folio);
    if (err)
    goto out_iput;
    pos += nr_bytes;
    num -= nr_bytes;
    }
    err = 0;
    out_iput:
    iput(inode);
    out_up_killsb:
    up_read(&fc.killsb);
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuse_retrieve_args {
    pub ap: fuse_args_pages,
    pub inarg: fuse_notify_retrieve_in,
}

#[no_mangle]
unsafe extern "C" fn fuse_retrieve_end(args: *mut fuse_args, error: c_int) {
    static void fuse_retrieve_end(struct fuse_args *args, int error)
    {
    struct fuse_retrieve_args *ra =
    container_of(args, typeof(*ra), ap.args);
    release_pages(ra.ap.folios, ra.ap.num_folios);
    kfree(ra);
    }
    static int fuse_retrieve(struct fuse_mount *fm, struct inode *inode,
    struct fuse_notify_retrieve_out *outarg)
    {
    int err;
    struct address_space *mapping = inode.i_mapping;
    loff_t file_size;
    unsigned int num;
    unsigned int offset;
    let mut total_len: usize = 0;
    unsigned int num_pages;
    struct fuse_conn *fc = fm.fc;
    struct fuse_retrieve_args *ra;
    let mut args_size: usize = sizeof(*ra);
    struct fuse_args_pages *ap;
    struct fuse_args *args;
    let mut pos: loff_t = outarg.offset;
    offset = offset_in_page(pos);
    file_size = i_size_read(inode);
    num = min(outarg.size, fc.max_write);
    if (pos > file_size)
    num = 0;
#[no_mangle]
pub unsafe extern "C" fn if(pos: num > file_size -) -> else {
    else if (num > file_size - pos)
    num = file_size - pos;
    num_pages = DIV_ROUND_UP(num + offset, PAGE_SIZE);
    num_pages = min(num_pages, fc.max_pages);
    num = min(num, num_pages << PAGE_SHIFT);
    args_size += num_pages * (sizeof(ap.folios[0]) + sizeof(ap.descs[0]));
    ra = kzalloc(args_size, GFP_KERNEL);
    if (!ra)
    return -ENOMEM;
    ap = &ra.ap;
    ap.folios = (void *) (ra + 1);
    ap.descs = (void *) (ap.folios + num_pages);
    args = &ap.args;
    args.nodeid = outarg.nodeid;
    args.opcode = FUSE_NOTIFY_REPLY;
    args.in_numargs = 3;
    args.in_pages = true;
    args.end = fuse_retrieve_end;
    while (num && ap.num_folios < num_pages) {
    struct folio *folio;
    unsigned int folio_offset;
    unsigned int nr_bytes;
    let mut index: pgoff_t = pos >> PAGE_SHIFT;
    folio = filemap_get_folio(mapping, index);
    if (IS_ERR(folio))
    break;
    if (!folio_test_uptodate(folio)) {
    folio_put(folio);
    break;
    }
    folio_offset = offset_in_folio(folio, pos);
    nr_bytes = min(folio_size(folio) - folio_offset, num);
    ap.folios[ap.num_folios] = folio;
    ap.descs[ap.num_folios].offset = folio_offset;
    ap.descs[ap.num_folios].length = nr_bytes;
    ap.num_folios++;
    pos += nr_bytes;
    num -= nr_bytes;
    total_len += nr_bytes;
    }
    ra.inarg.offset = outarg.offset;
    ra.inarg.size = total_len;
    fuse_set_zero_arg0(args);
    args.in_args[1].size = sizeof(ra.inarg);
    args.in_args[1].value = &ra.inarg;
    args.in_args[2].size = total_len;
    err = fuse_simple_notify_reply(fm, args, outarg.notify_unique);
    if (err)
    fuse_retrieve_end(args, err);
    return err;
    }
    static int fuse_notify_retrieve(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_retrieve_out outarg;
    struct fuse_mount *fm;
    struct inode *inode;
    u64 nodeid;
    int err;
    if (size != sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    fuse_copy_finish(cs);
    if (outarg.offset >= MAX_LFS_FILESIZE)
    return -EINVAL;
    down_read(&fc.killsb);
    err = -ENOENT;
    nodeid = outarg.nodeid;
    inode = fuse_ilookup(fc, nodeid, &fm);
    if (inode) {
    err = -EINVAL;
    if (S_ISREG(inode.i_mode))
    err = fuse_retrieve(fm, inode, &outarg);
    iput(inode);
    }
    up_read(&fc.killsb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fuse_notify_resend(fc: *mut fuse_conn) -> c_int {
    static int fuse_notify_resend(struct fuse_conn *fc)
    {
    fuse_chan_resend(fc.chan);
    return 0;
    }
//
// Increments the fuse connection epoch.  This will cause dentries and
// readdir caches from previous epochs to be invalidated.  Additionally,
// if inval_wq is set, a work queue is scheduled to trigger the invalidation.
//
#[no_mangle]
unsafe extern "C" fn fuse_notify_inc_epoch(fc: *mut fuse_conn) -> c_int {
    static int fuse_notify_inc_epoch(struct fuse_conn *fc)
    {
    atomic_inc(&fc.epoch);
    if (inval_wq)
    schedule_work(&fc.epoch_work);
    return 0;
    }
    static int fuse_notify_prune(struct fuse_conn *fc, unsigned int size,
    struct fuse_copy_state *cs)
    {
    struct fuse_notify_prune_out outarg;
    let mut batch: c_uint = 512;
    u64 *nodeids __free(kfree) = kmalloc(sizeof(u64) * batch, GFP_KERNEL);
    unsigned int num, i;
    int err;
    if (!nodeids)
    return -ENOMEM;
    if (size < sizeof(outarg))
    return -EINVAL;
    err = fuse_copy_one(cs, &outarg, sizeof(outarg));
    if (err)
    return err;
    if (size - sizeof(outarg) != array_size(outarg.count, sizeof(u64)))
    return -EINVAL;
    for (; outarg.count; outarg.count -= num) {
    num = min(batch, outarg.count);
    err = fuse_copy_one(cs, nodeids, num * sizeof(u64));
    if (err)
    return err;
    scoped_guard(rwsem_read, &fc.killsb) {
    for (i = 0; i < num; i++)
    fuse_try_prune_one_inode(fc, nodeids[i]);
    }
    }
    return 0;
    }
    int fuse_notify(struct fuse_conn *fc, enum fuse_notify_code code,
    unsigned int size, struct fuse_copy_state *cs)
    {
    switch (code) {
    case FUSE_NOTIFY_POLL:
    return fuse_notify_poll(fc, size, cs);
    case FUSE_NOTIFY_INVAL_INODE:
    return fuse_notify_inval_inode(fc, size, cs);
    case FUSE_NOTIFY_INVAL_ENTRY:
    return fuse_notify_inval_entry(fc, size, cs);
    case FUSE_NOTIFY_STORE:
    return fuse_notify_store(fc, size, cs);
    case FUSE_NOTIFY_RETRIEVE:
    return fuse_notify_retrieve(fc, size, cs);
    case FUSE_NOTIFY_DELETE:
    return fuse_notify_delete(fc, size, cs);
    case FUSE_NOTIFY_RESEND:
    return fuse_notify_resend(fc);
    case FUSE_NOTIFY_INC_EPOCH:
    return fuse_notify_inc_epoch(fc);
    case FUSE_NOTIFY_PRUNE:
    return fuse_notify_prune(fc, size, cs);
    default:
    return -EINVAL;
    }
    }
