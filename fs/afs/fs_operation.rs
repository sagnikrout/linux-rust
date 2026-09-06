//! Automatically rewritten from C to Rust
//! Source: fs/afs/fs_operation.c
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
// Fileserver-directed operation handling.
//
// Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static atomic_t afs_operation_debug_counter;
//
// Create an operation against a volume.
//
    struct afs_operation *afs_alloc_operation(struct key *key, struct afs_volume *volume)
    {
    struct afs_operation *op;
    _enter("");
    op = kzalloc_obj(*op);
    if (!op)
    return ERR_PTR(-ENOMEM);
    if (!key) {
    key = afs_request_key(volume.cell);
    if (IS_ERR(key)) {
    kfree(op);
    return ERR_CAST(key);
    }
    } else {
    key_get(key);
    }
    op.key			= key;
    op.volume		= afs_get_volume(volume, afs_volume_trace_get_new_op);
    op.net			= volume.cell.net;
    op.cb_v_break		= atomic_read(&volume.cb_v_break);
    op.pre_volsync.creation = volume.creation_time;
    op.pre_volsync.update	= volume.update_time;
    op.debug_id		= atomic_inc_return(&afs_operation_debug_counter);
    op.nr_iterations	= -1;
    afs_op_set_error(op, -EDESTADDRREQ);
    _leave(" = [op=%08x]", op.debug_id);
    return op;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct afs_io_locker {
    pub link: list_head,
    pub task: *mut task_struct,
    pub have_lock: c_ulong,
}

//
// Unlock the I/O lock on a vnode.
//
#[no_mangle]
unsafe extern "C" fn afs_unlock_for_io(vnode: *mut afs_vnode) {
    static void afs_unlock_for_io(struct afs_vnode *vnode)
    {
    struct afs_io_locker *locker;
    spin_lock(&vnode.lock);
    locker = list_first_entry_or_null(&vnode.io_lock_waiters,
    struct afs_io_locker, link);
    if (locker) {
    list_del(&locker.link);
    smp_store_release(&locker.have_lock, 1); /* The unlock barrier. */
    smp_mb__after_atomic(); /* Store have_lock before task state */
    wake_up_process(locker.task);
    } else {
    clear_bit(AFS_VNODE_IO_LOCK, &vnode.flags);
    }
    spin_unlock(&vnode.lock);
    }
//
// Lock the I/O lock on a vnode uninterruptibly.  We can't use an ordinary
// mutex as lockdep will complain if we unlock it in the wrong thread.
//
#[no_mangle]
unsafe extern "C" fn afs_lock_for_io(vnode: *mut afs_vnode) {
    static void afs_lock_for_io(struct afs_vnode *vnode)
    {
    let mut myself: afs_io_locker = { .task = current, };
    spin_lock(&vnode.lock);
    if (!test_and_set_bit(AFS_VNODE_IO_LOCK, &vnode.flags)) {
    spin_unlock(&vnode.lock);
    return;
    }
    list_add_tail(&myself.link, &vnode.io_lock_waiters);
    spin_unlock(&vnode.lock);
    for (;;) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    if (smp_load_acquire(&myself.have_lock)) /* The lock barrier */
    break;
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    }
//
// Lock the I/O lock on a vnode interruptibly.  We can't use an ordinary mutex
// as lockdep will complain if we unlock it in the wrong thread.
//
#[no_mangle]
unsafe extern "C" fn afs_lock_for_io_interruptible(vnode: *mut afs_vnode) -> c_int {
    static int afs_lock_for_io_interruptible(struct afs_vnode *vnode)
    {
    let mut myself: afs_io_locker = { .task = current, };
    let mut ret: c_int = 0;
    spin_lock(&vnode.lock);
    if (!test_and_set_bit(AFS_VNODE_IO_LOCK, &vnode.flags)) {
    spin_unlock(&vnode.lock);
    return 0;
    }
    list_add_tail(&myself.link, &vnode.io_lock_waiters);
    spin_unlock(&vnode.lock);
    for (;;) {
    set_current_state(TASK_INTERRUPTIBLE);
    if (smp_load_acquire(&myself.have_lock) || /* The lock barrier */
    signal_pending(current))
    break;
    schedule();
    }
    __set_current_state(TASK_RUNNING);
// If we got a signal, try to transfer the lock onto the next
// waiter.
//
    if (unlikely(signal_pending(current))) {
    spin_lock(&vnode.lock);
    if (myself.have_lock) {
    spin_unlock(&vnode.lock);
    afs_unlock_for_io(vnode);
    } else {
    list_del(&myself.link);
    spin_unlock(&vnode.lock);
    }
    ret = -ERESTARTSYS;
    }
    return ret;
    }
//
// Lock the vnode(s) being operated upon.
//
#[no_mangle]
unsafe extern "C" fn afs_get_io_locks(op: *mut afs_operation) -> bool {
    static bool afs_get_io_locks(struct afs_operation *op)
    {
    struct afs_vnode *vnode = op.file[0].vnode;
    struct afs_vnode *vnode2 = op.file[1].vnode;
    _enter("");
    if (op.flags & AFS_OPERATION_UNINTR) {
    afs_lock_for_io(vnode);
    op.flags |= AFS_OPERATION_LOCK_0;
    _leave(" = t [1]");
    return true;
    }
    if (!vnode2 || !op.file[1].need_io_lock || vnode == vnode2)
    vnode2 = core::ptr::null_mut();
    if (vnode2 > vnode)
    swap(vnode, vnode2);
    if (afs_lock_for_io_interruptible(vnode) < 0) {
    afs_op_set_error(op, -ERESTARTSYS);
    op.flags |= AFS_OPERATION_STOP;
    _leave(" = f [I 0]");
    return false;
    }
    op.flags |= AFS_OPERATION_LOCK_0;
    if (vnode2) {
    if (afs_lock_for_io_interruptible(vnode2) < 0) {
    afs_op_set_error(op, -ERESTARTSYS);
    op.flags |= AFS_OPERATION_STOP;
    afs_unlock_for_io(vnode);
    op.flags &= ~AFS_OPERATION_LOCK_0;
    _leave(" = f [I 1]");
    return false;
    }
    op.flags |= AFS_OPERATION_LOCK_1;
    }
    _leave(" = t [2]");
    return true;
    }
#[no_mangle]
unsafe extern "C" fn afs_drop_io_locks(op: *mut afs_operation) {
    static void afs_drop_io_locks(struct afs_operation *op)
    {
    struct afs_vnode *vnode = op.file[0].vnode;
    struct afs_vnode *vnode2 = op.file[1].vnode;
    _enter("");
    if (op.flags & AFS_OPERATION_LOCK_1)
    afs_unlock_for_io(vnode2);
    if (op.flags & AFS_OPERATION_LOCK_0)
    afs_unlock_for_io(vnode);
    }
    static void afs_prepare_vnode(struct afs_operation *op, struct afs_vnode_param *vp,
    unsigned int index)
    {
    struct afs_vnode *vnode = vp.vnode;
    if (vnode) {
    vp.fid			= vnode.fid;
    vp.dv_before		= vnode.status.data_version;
    vp.cb_break_before	= afs_calc_vnode_cb_break(vnode);
    if (vnode.lock_state != AFS_VNODE_LOCK_NONE)
    op.flags	|= AFS_OPERATION_CUR_ONLY;
    if (vp.modification)
    set_bit(AFS_VNODE_MODIFYING, &vnode.flags);
    }
    if (vp.fid.vnode)
    _debug("PREP[%u] {%llx:%llu.%u}",
    index, vp.fid.vid, vp.fid.vnode, vp.fid.unique);
    }
//
// Begin an operation on the fileserver.
//
// Fileserver operations are serialised on the server by vnode, so we serialise
// them here also using the io_lock.
//
#[no_mangle]
pub unsafe extern "C" fn afs_begin_vnode_operation(op: *mut afs_operation) -> bool {
    bool afs_begin_vnode_operation(struct afs_operation *op)
    {
    struct afs_vnode *vnode = op.file[0].vnode;
    ASSERT(vnode);
    _enter("");
    if (op.file[0].need_io_lock)
    if (!afs_get_io_locks(op))
    return false;
    afs_prepare_vnode(op, &op.file[0], 0);
    afs_prepare_vnode(op, &op.file[1], 1);
    op.cb_v_break = atomic_read(&op.volume.cb_v_break);
    _leave(" = true");
    return true;
    }
//
// Tidy up a filesystem cursor and unlock the vnode.
//
#[no_mangle]
pub unsafe extern "C" fn afs_end_vnode_operation(op: *mut afs_operation) {
    void afs_end_vnode_operation(struct afs_operation *op)
    {
    _enter("");
    switch (afs_op_error(op)) {
    case -EDESTADDRREQ:
    case -EADDRNOTAVAIL:
    case -ENETUNREACH:
    case -EHOSTUNREACH:
    afs_dump_edestaddrreq(op);
    break;
    }
    afs_drop_io_locks(op);
    }
//
// Wait for an in-progress operation to complete.
//
#[no_mangle]
pub unsafe extern "C" fn afs_wait_for_operation(op: *mut afs_operation) {
    void afs_wait_for_operation(struct afs_operation *op)
    {
    _enter("");
    while (afs_select_fileserver(op)) {
    op.call_responded = false;
    op.call_error = 0;
    op.call_abort_code = 0;
    if (test_bit(AFS_SERVER_FL_IS_YFS, &op.server.flags) &&
    op.ops.issue_yfs_rpc)
    op.ops.issue_yfs_rpc(op);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: op->ops->issue_afs_rpc) -> else {
    else if (op.ops.issue_afs_rpc)
    op.ops.issue_afs_rpc(op);
    else
    op.call_error = -ENOTSUPP;
    if (op.call) {
    afs_wait_for_call_to_complete(op.call);
    op.call_abort_code = op.call.abort_code;
    op.call_error = op.call.error;
    op.call_responded = op.call.responded;
    afs_put_call(op.call);
    }
    }
    if (op.call_responded && op.server)
    set_bit(AFS_SERVER_FL_RESPONDING, &op.server.flags);
    if (!afs_op_error(op)) {
    _debug("success");
    op.ops.success(op);
    } else if (op.cumul_error.aborted) {
    if (op.ops.aborted)
    op.ops.aborted(op);
    } else {
    if (op.ops.failed)
    op.ops.failed(op);
    }
    afs_end_vnode_operation(op);
    if (!afs_op_error(op) && op.ops.edit_dir) {
    _debug("edit_dir");
    op.ops.edit_dir(op);
    }
    _leave("");
    }
//
// Dispose of an operation.
//
#[no_mangle]
pub unsafe extern "C" fn afs_put_operation(op: *mut afs_operation) -> c_int {
    int afs_put_operation(struct afs_operation *op)
    {
    struct afs_addr_list *alist;
    int i, ret = afs_op_error(op);
    _enter("op=%08x,%d", op.debug_id, ret);
    if (op.ops && op.ops.put)
    op.ops.put(op);
    if (op.file[0].modification)
    clear_bit(AFS_VNODE_MODIFYING, &op.file[0].vnode.flags);
    if (op.file[1].modification && op.file[1].vnode != op.file[0].vnode)
    clear_bit(AFS_VNODE_MODIFYING, &op.file[1].vnode.flags);
    if (op.file[0].put_vnode)
    iput(&op.file[0].vnode.netfs.inode);
    if (op.file[1].put_vnode)
    iput(&op.file[1].vnode.netfs.inode);
    if (op.more_files) {
    for (i = 0; i < op.nr_files - 2; i++)
    if (op.more_files[i].put_vnode)
    iput(&op.more_files[i].vnode.netfs.inode);
    kvfree(op.more_files);
    }
    if (op.estate) {
    alist = op.estate.addresses;
    if (alist) {
    if (op.call_responded &&
    op.addr_index != alist.preferred &&
    test_bit(alist.preferred, &op.addr_tried))
    WRITE_ONCE(alist.preferred, op.addr_index);
    }
    }
    afs_clear_server_states(op);
    afs_put_serverlist(op.net, op.server_list);
    afs_put_volume(op.volume, afs_volume_trace_put_put_op);
    key_put(op.key);
    kfree(op);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn afs_do_sync_operation(op: *mut afs_operation) -> c_int {
    int afs_do_sync_operation(struct afs_operation *op)
    {
    afs_begin_vnode_operation(op);
    afs_wait_for_operation(op);
    return afs_put_operation(op);
    }
