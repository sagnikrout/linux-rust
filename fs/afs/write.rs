//! Automatically rewritten from C to Rust
//! Source: fs/afs/write.c
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
// handling of writes to regular files and writing back to the server
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// completion of write to server
//
#[no_mangle]
unsafe extern "C" fn afs_pages_written_back(vnode: *mut afs_vnode, start: loff_t, len: c_uint) {
    static void afs_pages_written_back(struct afs_vnode *vnode, loff_t start, unsigned int len)
    {
    _enter("{%llx:%llu},{%x @%llx}",
    vnode.fid.vid, vnode.fid.vnode, len, start);
    afs_prune_wb_keys(vnode);
    _leave("");
    }
//
// Find a key to use for the writeback.  We cached the keys used to author the
// writes on the vnode.  wreq->netfs_priv2 will contain the last writeback key
// record used or NULL and we need to start from there if it's set.
// wreq->netfs_priv will be set to the key itself or NULL.
//
#[no_mangle]
unsafe extern "C" fn afs_get_writeback_key(wreq: *mut netfs_io_request) {
    static void afs_get_writeback_key(struct netfs_io_request *wreq)
    {
    struct afs_wb_key *wbk, *old = wreq.netfs_priv2;
    struct afs_vnode *vnode = AFS_FS_I(wreq.inode);
    key_put(wreq.netfs_priv);
    wreq.netfs_priv = core::ptr::null_mut();
    wreq.netfs_priv2 = core::ptr::null_mut();
    spin_lock(&vnode.wb_lock);
    if (old)
    wbk = list_next_entry(old, vnode_link);
    else
    wbk = list_first_entry(&vnode.wb_keys, struct afs_wb_key, vnode_link);
    list_for_each_entry_from(wbk, &vnode.wb_keys, vnode_link) {
    _debug("wbk %u", key_serial(wbk.key));
    if (key_validate(wbk.key) == 0) {
    refcount_inc(&wbk.usage);
    wreq.netfs_priv = key_get(wbk.key);
    wreq.netfs_priv2 = wbk;
    _debug("USE WB KEY %u", key_serial(wbk.key));
    break;
    }
    }
    spin_unlock(&vnode.wb_lock);
    afs_put_wb_key(old);
    }
#[no_mangle]
unsafe extern "C" fn afs_store_data_success(op: *mut afs_operation) {
    static void afs_store_data_success(struct afs_operation *op)
    {
    struct afs_vnode *vnode = op.file[0].vnode;
    op.ctime = op.file[0].scb.status.mtime_client;
    afs_vnode_commit_status(op, &op.file[0]);
    if (!afs_op_error(op)) {
    afs_pages_written_back(vnode, op.store.pos, op.store.size);
    afs_stat_v(vnode, n_stores);
    atomic_long_add(op.store.size, &afs_v2net(vnode).n_store_bytes);
    }
    }
    static const struct afs_operation_ops afs_store_data_operation = {
    .issue_afs_rpc	= afs_fs_store_data,
    .issue_yfs_rpc	= yfs_fs_store_data,
    .success	= afs_store_data_success,
    };
//
// Prepare a subrequest to write to the server.  This sets the max_len
// parameter.
//
#[no_mangle]
pub unsafe extern "C" fn afs_prepare_write(subreq: *mut netfs_io_subrequest) {
    void afs_prepare_write(struct netfs_io_subrequest *subreq)
    {
    struct netfs_io_stream *stream = &subreq.rreq.io_streams[subreq.stream_nr];
// if (test_bit(NETFS_SREQ_RETRYING, &subreq->flags))
// subreq->max_len = 512 * 1024;
// else
    stream.sreq_max_len = 256 * 1024 * 1024;
    }
//
// Issue a subrequest to write to the server.
//
#[no_mangle]
unsafe extern "C" fn afs_issue_write_worker(work: *mut work_struct) {
    static void afs_issue_write_worker(struct work_struct *work)
    {
    struct netfs_io_subrequest *subreq = container_of(work, struct netfs_io_subrequest, work);
    struct netfs_io_request *wreq = subreq.rreq;
    struct afs_operation *op;
    struct afs_vnode *vnode = AFS_FS_I(wreq.inode);
    let mut pos: c_ulonglong = subreq.start + subreq.transferred;
    let mut len: usize = subreq.len - subreq.transferred;
    let mut ret: c_int = -ENOKEY;
    _enter("R=%x[%x],%s{%llx:%llu.%u},%llx,%zx",
    wreq.debug_id, subreq.debug_index,
    vnode.volume.name,
    vnode.fid.vid,
    vnode.fid.vnode,
    vnode.fid.unique,
    pos, len);

    if (subreq.debug_index == 3)
    return netfs_write_subrequest_terminated(subreq, -ENOANO);
    if (!subreq.retry_count) {
    set_bit(NETFS_SREQ_NEED_RETRY, &subreq.flags);
    return netfs_write_subrequest_terminated(subreq, -EAGAIN);
    }

    op = afs_alloc_operation(wreq.netfs_priv, vnode.volume);
    if (IS_ERR(op))
    return netfs_write_subrequest_terminated(subreq, -EAGAIN);
    afs_op_set_vnode(op, 0, vnode);
    op.file[0].dv_delta	= 1;
    op.file[0].modification = true;
    op.store.pos		= pos;
    op.store.size		= len;
    op.flags		|= AFS_OPERATION_UNINTR;
    op.ops			= &afs_store_data_operation;
    afs_begin_vnode_operation(op);
    op.store.write_iter	= &subreq.io_iter;
    op.store.i_size	= umax(pos + len, netfs_read_remote_i_size(&vnode.netfs.inode));
    op.mtime		= inode_get_mtime(&vnode.netfs.inode);
    afs_wait_for_operation(op);
    ret = afs_put_operation(op);
    switch (ret) {
    case 0:
    __set_bit(NETFS_SREQ_MADE_PROGRESS, &subreq.flags);
    break;
    case -EACCES:
    case -EPERM:
    case -ENOKEY:
    case -EKEYEXPIRED:
    case -EKEYREJECTED:
    case -EKEYREVOKED:
// If there are more keys we can try, use the retry algorithm
// to rotate the keys.
//
    if (wreq.netfs_priv2)
    set_bit(NETFS_SREQ_NEED_RETRY, &subreq.flags);
    break;
    }
    netfs_write_subrequest_terminated(subreq, ret < 0 ? ret : subreq.len);
    }
#[no_mangle]
pub unsafe extern "C" fn afs_issue_write(subreq: *mut netfs_io_subrequest) {
    void afs_issue_write(struct netfs_io_subrequest *subreq)
    {
    subreq.work.func = afs_issue_write_worker;
    if (!queue_work(system_dfl_wq, &subreq.work))
    WARN_ON_ONCE(1);
    }
//
// Writeback calls this when it finds a folio that needs uploading.  This isn't
// called if writeback only has copy-to-cache to deal with.
//
#[no_mangle]
pub unsafe extern "C" fn afs_begin_writeback(wreq: *mut netfs_io_request) {
    void afs_begin_writeback(struct netfs_io_request *wreq)
    {
    if (S_ISREG(wreq.inode.i_mode))
    afs_get_writeback_key(wreq);
    }
//
// Prepare to retry the writes in request.  Use this to try rotating the
// available writeback keys.
//
#[no_mangle]
pub unsafe extern "C" fn afs_retry_request(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream) {
    void afs_retry_request(struct netfs_io_request *wreq, struct netfs_io_stream *stream)
    {
    struct netfs_io_subrequest *subreq =
    list_first_entry(&stream.subrequests,
    struct netfs_io_subrequest, rreq_link);
    switch (wreq.origin) {
    case NETFS_READAHEAD:
    case NETFS_READPAGE:
    case NETFS_READ_GAPS:
    case NETFS_READ_SINGLE:
    case NETFS_READ_FOR_WRITE:
    case NETFS_UNBUFFERED_READ:
    case NETFS_DIO_READ:
    return;
    default:
    break;
    }
    switch (subreq.error) {
    case -EACCES:
    case -EPERM:
    case -ENOKEY:
    case -EKEYEXPIRED:
    case -EKEYREJECTED:
    case -EKEYREVOKED:
    afs_get_writeback_key(wreq);
    if (!wreq.netfs_priv)
    stream.failed = true;
    break;
    }
    }
//
// write some of the pending data back to the server
//
#[no_mangle]
pub unsafe extern "C" fn afs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int {
    int afs_writepages(struct address_space *mapping, struct writeback_control *wbc)
    {
    struct afs_vnode *vnode = AFS_FS_I(mapping.host);
    int ret;
// We have to be careful as we can end up racing with setattr()
// truncating the pagecache since the caller doesn't take a lock here
// to prevent it.
//
    if (wbc.sync_mode == WB_SYNC_ALL)
    down_read(&vnode.validate_lock);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !down_read_trylock(&vnode->validate_lock)) -> else {
    else if (!down_read_trylock(&vnode.validate_lock))
    return 0;
    ret = netfs_writepages(mapping, wbc);
    up_read(&vnode.validate_lock);
    return ret;
    }
//
// flush any dirty pages for this process, and check for write errors.
// - the return status from this call provides a reliable indication of
// whether any write errors occurred for this process.
//
#[no_mangle]
pub unsafe extern "C" fn afs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    int afs_fsync(struct file *file, loff_t start, loff_t end, int datasync)
    {
    struct afs_vnode *vnode = AFS_FS_I(file_inode(file));
    struct afs_file *af = file.private_data;
    int ret;
    _enter("{%llx:%llu},{n=%pD},%d",
    vnode.fid.vid, vnode.fid.vnode, file,
    datasync);
    ret = afs_validate(vnode, af.key);
    if (ret < 0)
    return ret;
    return file_write_and_wait_range(file, start, end);
    }
//
// notification that a previously read-only page is about to become writable
// - if it returns an error, the caller will deliver a bus error signal
//
#[no_mangle]
pub unsafe extern "C" fn afs_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    vm_fault_t afs_page_mkwrite(struct vm_fault *vmf)
    {
    struct file *file = vmf.vma.vm_file;
    if (afs_validate(AFS_FS_I(file_inode(file)), afs_file_key(file)) < 0)
    return VM_FAULT_SIGBUS;
    return netfs_page_mkwrite(vmf, core::ptr::null_mut());
    }
//
// Prune the keys cached for writeback.  The caller must hold vnode->wb_lock.
//
#[no_mangle]
pub unsafe extern "C" fn afs_prune_wb_keys(vnode: *mut afs_vnode) {
    void afs_prune_wb_keys(struct afs_vnode *vnode)
    {
    LIST_HEAD(graveyard);
    struct afs_wb_key *wbk, *tmp;
// Discard unused keys
    spin_lock(&vnode.wb_lock);
    if (!mapping_tagged(&vnode.netfs.inode.i_data, PAGECACHE_TAG_WRITEBACK) &&
    !mapping_tagged(&vnode.netfs.inode.i_data, PAGECACHE_TAG_DIRTY)) {
    list_for_each_entry_safe(wbk, tmp, &vnode.wb_keys, vnode_link) {
    if (refcount_read(&wbk.usage) == 1)
    list_move(&wbk.vnode_link, &graveyard);
    }
    }
    spin_unlock(&vnode.wb_lock);
    while (!list_empty(&graveyard)) {
    wbk = list_entry(graveyard.next, struct afs_wb_key, vnode_link);
    list_del(&wbk.vnode_link);
    afs_put_wb_key(wbk);
    }
    }
