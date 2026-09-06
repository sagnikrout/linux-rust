//! Automatically rewritten from C to Rust
//! Source: fs/ceph/locks.c
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

    static u64 lock_secret;
    static int ceph_lock_wait_for_completion(struct ceph_mds_client *mdsc,
    struct ceph_mds_request *req);
#[no_mangle]
pub unsafe extern "C" fn secure_addr(addr: *mut c_void) -> u64 {
    static inline u64 secure_addr(void *addr)
    {
    let mut v: u64 = lock_secret ^ (u64)(unsigned long)addr;
//
// Set the most significant bit, so that MDS knows the 'owner'
// is sufficient to identify the owner of lock. (old code uses
// both 'owner' and 'pid')
//
    v |= (1ULL << 63);
    return v;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_flock_init() -> void __init {
    void __init ceph_flock_init(void)
    {
    get_random_bytes(&lock_secret, sizeof(lock_secret));
    }
#[no_mangle]
unsafe extern "C" fn ceph_fl_copy_lock(dst: *mut file_lock, src: *mut file_lock) {
    static void ceph_fl_copy_lock(struct file_lock *dst, struct file_lock *src)
    {
    struct inode *inode = file_inode(dst.c.flc_file);
    atomic_inc(&ceph_inode(inode).i_filelock_ref);
    dst.fl_u.ceph.inode = igrab(inode);
    }
//
// Do not use the 'fl->fl_file' in release function, which
// is possibly already released by another thread.
//
#[no_mangle]
unsafe extern "C" fn ceph_fl_release_lock(fl: *mut file_lock) {
    static void ceph_fl_release_lock(struct file_lock *fl)
    {
    struct inode *inode = fl.fl_u.ceph.inode;
    struct ceph_inode_info *ci;
//
// If inode is NULL it should be a request file_lock,
// nothing we can do.
//
    if (!inode)
    return;
    ci = ceph_inode(inode);
    if (atomic_dec_and_test(&ci.i_filelock_ref)) {
// clear error when all locks are released
    clear_bit(CEPH_I_ERROR_FILELOCK_BIT, &ci.i_ceph_flags);
    }
    fl.fl_u.ceph.inode = core::ptr::null_mut();
    iput(inode);
    }
    static const struct file_lock_operations ceph_fl_lock_ops = {
    .fl_copy_lock = ceph_fl_copy_lock,
    .fl_release_private = ceph_fl_release_lock,
    };
//
// Implement fcntl and flock locking functions.
//
    static int ceph_lock_message(u8 lock_type, u16 operation, struct inode *inode,
    int cmd, u8 wait, struct file_lock *fl)
    {
    struct ceph_mds_client *mdsc = ceph_sb_to_mdsc(inode.i_sb);
    struct ceph_client *cl = mdsc.fsc.client;
    struct ceph_mds_request *req;
    int err;
    let mut length: u64 = 0;
    u64 owner;
    if (operation == CEPH_MDS_OP_SETFILELOCK) {
//
// increasing i_filelock_ref closes race window between
// handling request reply and adding file_lock struct to
// inode. Otherwise, auth caps may get trimmed in the
// window. Caller function will decrease the counter.
//
    fl.fl_ops = &ceph_fl_lock_ops;
    fl.fl_ops.fl_copy_lock(fl, core::ptr::null_mut());
    }
    if (operation != CEPH_MDS_OP_SETFILELOCK || cmd == CEPH_LOCK_UNLOCK)
    wait = 0;
    req = ceph_mdsc_create_request(mdsc, operation, USE_AUTH_MDS);
    if (IS_ERR(req))
    return PTR_ERR(req);
    req.r_inode = inode;
    ihold(inode);
    req.r_num_caps = 1;
// mds requires start and length rather than start and end
    if (LLONG_MAX == fl.fl_end)
    length = 0;
    else
    length = fl.fl_end - fl.fl_start + 1;
    owner = secure_addr(fl.c.flc_owner);
    doutc(cl, "rule: %d, op: %d, owner: %llx, pid: %llu, "
    "start: %llu, length: %llu, wait: %d, type: %d\n",
    (int)lock_type, (int)operation, owner,
    (u64) fl.c.flc_pid,
    fl.fl_start, length, wait, fl.c.flc_type);
    req.r_args.filelock_change.rule = lock_type;
    req.r_args.filelock_change.type = cmd;
    req.r_args.filelock_change.owner = cpu_to_le64(owner);
    req.r_args.filelock_change.pid = cpu_to_le64((u64) fl.c.flc_pid);
    req.r_args.filelock_change.start = cpu_to_le64(fl.fl_start);
    req.r_args.filelock_change.length = cpu_to_le64(length);
    req.r_args.filelock_change.wait = wait;
    err = ceph_mdsc_submit_request(mdsc, inode, req);
    if (!err)
    err = ceph_mdsc_wait_request(mdsc, req, wait ?
    ceph_lock_wait_for_completion : core::ptr::null_mut());
    if (!err && operation == CEPH_MDS_OP_GETFILELOCK) {
    fl.c.flc_pid = -le64_to_cpu(req.r_reply_info.filelock_reply.pid);
    if (CEPH_LOCK_SHARED == req.r_reply_info.filelock_reply.type)
    fl.c.flc_type = F_RDLCK;
#[no_mangle]
pub unsafe extern "C" fn if(req->r_reply_info.filelock_reply->type: CEPH_LOCK_EXCL ==) -> else {
    else if (CEPH_LOCK_EXCL == req.r_reply_info.filelock_reply.type)
    fl.c.flc_type = F_WRLCK;
    else
    fl.c.flc_type = F_UNLCK;
    fl.fl_start = le64_to_cpu(req.r_reply_info.filelock_reply.start);
    length = le64_to_cpu(req.r_reply_info.filelock_reply.start) +
    le64_to_cpu(req.r_reply_info.filelock_reply.length);
    if (length >= 1)
    fl.fl_end = length -1;
    else
    fl.fl_end = 0;
    }
    ceph_mdsc_put_request(req);
    doutc(cl, "rule: %d, op: %d, pid: %llu, start: %llu, "
    "length: %llu, wait: %d, type: %d, err code %d\n",
    (int)lock_type, (int)operation, (u64) fl.c.flc_pid,
    fl.fl_start, length, wait, fl.c.flc_type, err);
    return err;
    }
    static int ceph_lock_wait_for_completion(struct ceph_mds_client *mdsc,
    struct ceph_mds_request *req)
    {
    struct ceph_client *cl = mdsc.fsc.client;
    struct ceph_mds_request *intr_req;
    struct inode *inode = req.r_inode;
    int err, lock_type;
    BUG_ON(req.r_op != CEPH_MDS_OP_SETFILELOCK);
    if (req.r_args.filelock_change.rule == CEPH_LOCK_FCNTL)
    lock_type = CEPH_LOCK_FCNTL_INTR;
#[no_mangle]
pub unsafe extern "C" fn if(CEPH_LOCK_FLOCK: req->r_args.filelock_change.rule ==) -> else {
    else if (req.r_args.filelock_change.rule == CEPH_LOCK_FLOCK)
    lock_type = CEPH_LOCK_FLOCK_INTR;
    else
    BUG_ON(1);
    BUG_ON(req.r_args.filelock_change.type == CEPH_LOCK_UNLOCK);
    err = wait_for_completion_interruptible(&req.r_completion);
    if (!err)
    return 0;
    doutc(cl, "request %llu was interrupted\n", req.r_tid);
    mutex_lock(&mdsc.mutex);
    if (test_bit(CEPH_MDS_R_GOT_RESULT, &req.r_req_flags)) {
    err = 0;
    } else {
//
// ensure we aren't running concurrently with
// ceph_fill_trace or ceph_readdir_prepopulate, which
// rely on locks (dir mutex) held by our caller.
//
    mutex_lock(&req.r_fill_mutex);
    req.r_err = err;
    set_bit(CEPH_MDS_R_ABORTED, &req.r_req_flags);
    mutex_unlock(&req.r_fill_mutex);
    if (!req.r_session) {
// haven't sent the request
    err = 0;
    }
    }
    mutex_unlock(&mdsc.mutex);
    if (!err)
    return 0;
    intr_req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_SETFILELOCK,
    USE_AUTH_MDS);
    if (IS_ERR(intr_req))
    return PTR_ERR(intr_req);
    intr_req.r_inode = inode;
    ihold(inode);
    intr_req.r_num_caps = 1;
    intr_req.r_args.filelock_change = req.r_args.filelock_change;
    intr_req.r_args.filelock_change.rule = lock_type;
    intr_req.r_args.filelock_change.type = CEPH_LOCK_UNLOCK;
    err = ceph_mdsc_do_request(mdsc, inode, intr_req);
    ceph_mdsc_put_request(intr_req);
    if (err && err != -ERESTARTSYS)
    return err;
    err = wait_for_completion_killable(&req.r_safe_completion);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn try_unlock_file(file: *mut file, fl: *mut file_lock) -> c_int {
    static int try_unlock_file(struct file *file, struct file_lock *fl)
    {
    int err;
    let mut orig_flags: c_uint = fl.c.flc_flags;
    fl.c.flc_flags |= FL_EXISTS;
    err = locks_lock_file_wait(file, fl);
    fl.c.flc_flags = orig_flags;
    if (err == -ENOENT) {
    if (!(orig_flags & FL_EXISTS))
    err = 0;
    return err;
    }
    return 1;
    }
//
// Attempt to set an fcntl lock.
// For now, this just goes away to the server. Later it may be more awesome.
//
#[no_mangle]
pub unsafe extern "C" fn ceph_lock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    int ceph_lock(struct file *file, int cmd, struct file_lock *fl)
    {
    struct inode *inode = file_inode(file);
    struct ceph_inode_info *ci = ceph_inode(inode);
    struct ceph_mds_client *mdsc = ceph_sb_to_mdsc(inode.i_sb);
    struct ceph_client *cl = ceph_inode_to_client(inode);
    let mut err: c_int = 0;
    let mut op: u16 = CEPH_MDS_OP_SETFILELOCK;
    let mut wait: u8 = 0;
    u8 lock_cmd;
    if (!(fl.c.flc_flags & FL_POSIX))
    return -ENOLCK;
    if (ceph_inode_is_shutdown(inode))
    return -ESTALE;
    doutc(cl, "fl_owner: %p\n", fl.c.flc_owner);
// set wait bit as appropriate, then make command as Ceph expects it
    if (IS_GETLK(cmd))
    op = CEPH_MDS_OP_GETFILELOCK;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_SETLKW(cmd)) -> else {
    else if (IS_SETLKW(cmd))
    wait = 1;
    if (test_bit(CEPH_I_ERROR_FILELOCK_BIT, &ci.i_ceph_flags)) {
    if (op == CEPH_MDS_OP_SETFILELOCK && lock_is_unlock(fl))
    posix_lock_file(file, fl, core::ptr::null_mut());
    return -EIO;
    }
// Wait for reset to complete before acquiring new locks
    if (op == CEPH_MDS_OP_SETFILELOCK && !lock_is_unlock(fl)) {
    err = ceph_mdsc_wait_for_reset(mdsc);
    if (err)
    return err;
    }
    if (lock_is_read(fl))
    lock_cmd = CEPH_LOCK_SHARED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: lock_is_write(fl)) -> else {
    else if (lock_is_write(fl))
    lock_cmd = CEPH_LOCK_EXCL;
    else
    lock_cmd = CEPH_LOCK_UNLOCK;
    if (op == CEPH_MDS_OP_SETFILELOCK && lock_is_unlock(fl)) {
    err = try_unlock_file(file, fl);
    if (err <= 0)
    return err;
    }
    err = ceph_lock_message(CEPH_LOCK_FCNTL, op, inode, lock_cmd, wait, fl);
    if (!err) {
    if (op == CEPH_MDS_OP_SETFILELOCK && F_UNLCK != fl.c.flc_type) {
    doutc(cl, "locking locally\n");
    err = posix_lock_file(file, fl, core::ptr::null_mut());
    if (err) {
// undo! This should only happen if
// the kernel detects local
// deadlock.
    ceph_lock_message(CEPH_LOCK_FCNTL, op, inode,
    CEPH_LOCK_UNLOCK, 0, fl);
    doutc(cl, "got %d on posix_lock_file, undid lock\n",
    err);
    }
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_flock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    int ceph_flock(struct file *file, int cmd, struct file_lock *fl)
    {
    struct inode *inode = file_inode(file);
    struct ceph_inode_info *ci = ceph_inode(inode);
    struct ceph_mds_client *mdsc = ceph_sb_to_mdsc(inode.i_sb);
    struct ceph_client *cl = ceph_inode_to_client(inode);
    let mut err: c_int = 0;
    let mut wait: u8 = 0;
    u8 lock_cmd;
    if (!(fl.c.flc_flags & FL_FLOCK))
    return -ENOLCK;
    if (ceph_inode_is_shutdown(inode))
    return -ESTALE;
    doutc(cl, "fl_file: %p\n", fl.c.flc_file);
    if (test_bit(CEPH_I_ERROR_FILELOCK_BIT, &ci.i_ceph_flags)) {
    if (lock_is_unlock(fl))
    locks_lock_file_wait(file, fl);
    return -EIO;
    }
// Wait for reset to complete before acquiring new locks
    if (!lock_is_unlock(fl)) {
    err = ceph_mdsc_wait_for_reset(mdsc);
    if (err)
    return err;
    }
    if (IS_SETLKW(cmd))
    wait = 1;
    if (lock_is_read(fl))
    lock_cmd = CEPH_LOCK_SHARED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: lock_is_write(fl)) -> else {
    else if (lock_is_write(fl))
    lock_cmd = CEPH_LOCK_EXCL;
    else
    lock_cmd = CEPH_LOCK_UNLOCK;
    if (lock_is_unlock(fl)) {
    err = try_unlock_file(file, fl);
    if (err <= 0)
    return err;
    }
    err = ceph_lock_message(CEPH_LOCK_FLOCK, CEPH_MDS_OP_SETFILELOCK,
    inode, lock_cmd, wait, fl);
    if (!err && F_UNLCK != fl.c.flc_type) {
    err = locks_lock_file_wait(file, fl);
    if (err) {
    ceph_lock_message(CEPH_LOCK_FLOCK,
    CEPH_MDS_OP_SETFILELOCK,
    inode, CEPH_LOCK_UNLOCK, 0, fl);
    doutc(cl, "got %d on locks_lock_file_wait, undid lock\n",
    err);
    }
    }
    return err;
    }
//
// Fills in the passed counter variables, so you can prepare pagelist metadata
// before calling ceph_encode_locks.
//
#[no_mangle]
pub unsafe extern "C" fn ceph_count_locks(inode: *mut inode, fcntl_count: *mut c_int, flock_count: *mut c_int) {
    void ceph_count_locks(struct inode *inode, int *fcntl_count, int *flock_count)
    {
    struct ceph_client *cl = ceph_inode_to_client(inode);
    struct file_lock *lock;
    struct file_lock_context *ctx;
// fcntl_count = 0;
// flock_count = 0;
    ctx = locks_inode_context(inode);
    if (ctx) {
    spin_lock(&ctx.flc_lock);
    for_each_file_lock(lock, &ctx.flc_posix)
    ++(*fcntl_count);
    for_each_file_lock(lock, &ctx.flc_flock)
    ++(*flock_count);
    spin_unlock(&ctx.flc_lock);
    }
    doutc(cl, "counted %d flock locks and %d fcntl locks\n",
// flock_count, *fcntl_count);
    }
//
// Given a pointer to a lock, convert it to a ceph filelock
//
    static int lock_to_ceph_filelock(struct inode *inode,
    struct file_lock *lock,
    struct ceph_filelock *cephlock)
    {
    struct ceph_client *cl = ceph_inode_to_client(inode);
    let mut err: c_int = 0;
    cephlock.start = cpu_to_le64(lock.fl_start);
    cephlock.length = cpu_to_le64(lock.fl_end - lock.fl_start + 1);
    cephlock.client = cpu_to_le64(0);
    cephlock.pid = cpu_to_le64((u64) lock.c.flc_pid);
    cephlock.owner = cpu_to_le64(secure_addr(lock.c.flc_owner));
    switch (lock.c.flc_type) {
    case F_RDLCK:
    cephlock.type = CEPH_LOCK_SHARED;
    break;
    case F_WRLCK:
    cephlock.type = CEPH_LOCK_EXCL;
    break;
    case F_UNLCK:
    cephlock.type = CEPH_LOCK_UNLOCK;
    break;
    default:
    doutc(cl, "Have unknown lock type %d\n",
    lock.c.flc_type);
    err = -EINVAL;
    }
    return err;
    }
//
// Encode the flock and fcntl locks for the given inode into the ceph_filelock
// array. Must be called with inode->i_lock already held.
// If we encounter more of a specific lock type than expected, return -ENOSPC.
//
    int ceph_encode_locks_to_buffer(struct inode *inode,
    struct ceph_filelock *flocks,
    int num_fcntl_locks, int num_flock_locks)
    {
    struct file_lock *lock;
    struct file_lock_context *ctx = locks_inode_context(inode);
    struct ceph_client *cl = ceph_inode_to_client(inode);
    let mut err: c_int = 0;
    let mut seen_fcntl: c_int = 0;
    let mut seen_flock: c_int = 0;
    let mut l: c_int = 0;
    doutc(cl, "encoding %d flock and %d fcntl locks\n", num_flock_locks,
    num_fcntl_locks);
    if (!ctx)
    return 0;
    spin_lock(&ctx.flc_lock);
    for_each_file_lock(lock, &ctx.flc_posix) {
    ++seen_fcntl;
    if (seen_fcntl > num_fcntl_locks) {
    err = -ENOSPC;
    goto fail;
    }
    err = lock_to_ceph_filelock(inode, lock, &flocks[l]);
    if (err)
    goto fail;
    ++l;
    }
    for_each_file_lock(lock, &ctx.flc_flock) {
    ++seen_flock;
    if (seen_flock > num_flock_locks) {
    err = -ENOSPC;
    goto fail;
    }
    err = lock_to_ceph_filelock(inode, lock, &flocks[l]);
    if (err)
    goto fail;
    ++l;
    }
    fail:
    spin_unlock(&ctx.flc_lock);
    return err;
    }
//
// Copy the encoded flock and fcntl locks into the pagelist.
// Format is: #fcntl locks, sequential fcntl locks, #flock locks,
// sequential flock locks.
// Returns zero on success.
//
    int ceph_locks_to_pagelist(struct ceph_filelock *flocks,
    struct ceph_pagelist *pagelist,
    int num_fcntl_locks, int num_flock_locks)
    {
    let mut err: c_int = 0;
    __le32 nlocks;
    nlocks = cpu_to_le32(num_fcntl_locks);
    err = ceph_pagelist_append(pagelist, &nlocks, sizeof(nlocks));
    if (err)
    goto out_fail;
    if (num_fcntl_locks > 0) {
    err = ceph_pagelist_append(pagelist, flocks,
    num_fcntl_locks * sizeof(*flocks));
    if (err)
    goto out_fail;
    }
    nlocks = cpu_to_le32(num_flock_locks);
    err = ceph_pagelist_append(pagelist, &nlocks, sizeof(nlocks));
    if (err)
    goto out_fail;
    if (num_flock_locks > 0) {
    err = ceph_pagelist_append(pagelist, &flocks[num_fcntl_locks],
    num_flock_locks * sizeof(*flocks));
    }
    out_fail:
    return err;
    }
