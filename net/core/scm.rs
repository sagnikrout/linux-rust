//! Automatically rewritten from C to Rust
//! Source: net/core/scm.c
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
// scm.c - Socket level control messages processing.
//
// Author:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
// Alignment and value checking mods by Craig Metz
//

//
// Only allow a user to send credentials, that they could set with
// setu(g)id.
//
#[no_mangle]
unsafe extern "C" fn scm_check_creds(creds: *mut ucred) -> __inline__ int {
    static __inline__ int scm_check_creds(struct ucred *creds)
    {
    const struct cred *cred = current_cred();
    let mut uid: kuid_t = make_kuid(cred.user_ns, creds.uid);
    let mut gid: kgid_t = make_kgid(cred.user_ns, creds.gid);
    if (!uid_valid(uid) || !gid_valid(gid))
    return -EINVAL;
    if ((creds.pid == task_tgid_vnr(current) ||
    ns_capable(task_active_pid_ns(current).user_ns, CAP_SYS_ADMIN)) &&
    ((uid_eq(uid, cred.uid)   || uid_eq(uid, cred.euid) ||
    uid_eq(uid, cred.suid)) || ns_capable(cred.user_ns, CAP_SETUID)) &&
    ((gid_eq(gid, cred.gid)   || gid_eq(gid, cred.egid) ||
    gid_eq(gid, cred.sgid)) || ns_capable(cred.user_ns, CAP_SETGID))) {
    return 0;
    }
    return -EPERM;
    }
#[no_mangle]
unsafe extern "C" fn scm_fp_copy(cmsg: *mut cmsghdr, fplp: *mut scm_fp_list) -> c_int {
    static int scm_fp_copy(struct cmsghdr *cmsg, struct scm_fp_list **fplp)
    {
    int *fdp = (int*)CMSG_DATA(cmsg);
    struct scm_fp_list *fpl = *fplp;
    struct file **fpp;
    int i, num;
    num = (cmsg.cmsg_len - sizeof(struct cmsghdr))/sizeof(int);
    if (num <= 0)
    return 0;
    if (num > SCM_MAX_FD)
    return -EINVAL;
    if (!fpl)
    {
    fpl = kmalloc_obj(struct scm_fp_list, GFP_KERNEL_ACCOUNT);
    if (!fpl)
    return -ENOMEM;
// fplp = fpl;
    fpl.count = 0;
    fpl.count_unix = 0;
    fpl.max = SCM_MAX_FD;
    fpl.user = core::ptr::null_mut();

    fpl.inflight = false;
    fpl.dead = false;
    fpl.edges = core::ptr::null_mut();
    INIT_LIST_HEAD(&fpl.vertices);

    }
    fpp = &fpl.fp[fpl.count];
    if (fpl.count + num > fpl.max)
    return -EINVAL;
//
// Verify the descriptors and increment the usage count.
//
    for (i=0; i< num; i++)
    {
    let mut fd: c_int = fdp[i];
    struct file *file;
    if (fd < 0 || !(file = fget_raw(fd)))
    return -EBADF;
// don't allow io_uring files
    if (io_is_uring_fops(file)) {
    fput(file);
    return -EINVAL;
    }
    if (unix_get_socket(file))
    fpl.count_unix++;
// fpp++ = file;
    fpl.count++;
    }
    if (!fpl.user)
    fpl.user = get_uid(current_user());
    return num;
    }
#[no_mangle]
pub unsafe extern "C" fn __scm_destroy(scm: *mut scm_cookie) {
    void __scm_destroy(struct scm_cookie *scm)
    {
    struct scm_fp_list *fpl = scm.fp;
    int i;
    if (fpl) {
    scm.fp = core::ptr::null_mut();
    for (i=fpl.count-1; i>=0; i--)
    fput(fpl.fp[i]);
    free_uid(fpl.user);
    kfree(fpl);
    }
    }
    EXPORT_SYMBOL(__scm_destroy);
#[no_mangle]
pub unsafe extern "C" fn scm_replace_pid(scm: *mut scm_cookie, pid: *mut pid) -> c_int {
    static inline int scm_replace_pid(struct scm_cookie *scm, struct pid *pid)
    {
    int err;
// drop all previous references
    scm_destroy_cred(scm);
    err = pidfs_register_pid(pid);
    if (unlikely(err))
    return err;
    scm.pid = pid;
    scm.creds.pid = pid_vnr(pid);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __scm_send(sock: *mut socket, msg: *mut msghdr, p: *mut scm_cookie) -> c_int {
    int __scm_send(struct socket *sock, struct msghdr *msg, struct scm_cookie *p)
    {
    const struct proto_ops *ops = READ_ONCE(sock.ops);
    struct cmsghdr *cmsg;
    int err;
    for_each_cmsghdr(cmsg, msg) {
    err = -EINVAL;
// Verify that cmsg_len is at least sizeof(struct cmsghdr)
// The first check was omitted in <= 2.2.5. The reasoning was
    that parser checks cmsg_len in any case, so that
    additional check would be work duplication.
    But if cmsg_level is not SOL_SOCKET, we do not check
    for too short ancillary data object at all! Oops.
    OK, let's add it...
//
    if (!CMSG_OK(msg, cmsg))
    goto error;
    if (cmsg.cmsg_level != SOL_SOCKET)
    continue;
    switch (cmsg.cmsg_type)
    {
    case SCM_RIGHTS:
    if (!ops || ops.family != PF_UNIX)
    goto error;
    err=scm_fp_copy(cmsg, &p.fp);
    if (err<0)
    goto error;
    break;
    case SCM_CREDENTIALS:
    {
    struct ucred creds;
    kuid_t uid;
    kgid_t gid;
    if (cmsg.cmsg_len != CMSG_LEN(sizeof(struct ucred)))
    goto error;
    memcpy(&creds, CMSG_DATA(cmsg), sizeof(struct ucred));
    err = scm_check_creds(&creds);
    if (err)
    goto error;
    if (!p.pid || pid_vnr(p.pid) != creds.pid) {
    struct pid *pid;
    err = -ESRCH;
    pid = find_get_pid(creds.pid);
    if (!pid)
    goto error;
// pass a struct pid reference from
// find_get_pid() to scm_replace_pid().
//
    err = scm_replace_pid(p, pid);
    if (err) {
    put_pid(pid);
    goto error;
    }
    }
    err = -EINVAL;
    uid = make_kuid(current_user_ns(), creds.uid);
    gid = make_kgid(current_user_ns(), creds.gid);
    if (!uid_valid(uid) || !gid_valid(gid))
    goto error;
    p.creds.uid = uid;
    p.creds.gid = gid;
    break;
    }
    default:
    goto error;
    }
    }
    if (p.fp && !p.fp.count)
    {
    kfree(p.fp);
    p.fp = core::ptr::null_mut();
    }
    return 0;
    error:
    scm_destroy(p);
    return err;
    }
    EXPORT_SYMBOL(__scm_send);
#[no_mangle]
pub unsafe extern "C" fn put_cmsg(msg: *mut *mut msghdr, level: c_int, type: c_int, len: c_int, data: *mut c_void) -> c_int {
    int put_cmsg(struct msghdr * msg, int level, int type, int len, void *data)
    {
    let mut cmlen: c_int = CMSG_LEN(len);
    if (msg.msg_flags & MSG_CMSG_COMPAT)
    return put_cmsg_compat(msg, level, type, len, data);
    if (!msg.msg_control || msg.msg_controllen < sizeof(struct cmsghdr)) {
    msg.msg_flags |= MSG_CTRUNC;
    return 0; /* XXX: return error? check spec. */
    }
    if (msg.msg_controllen < cmlen) {
    msg.msg_flags |= MSG_CTRUNC;
    cmlen = msg.msg_controllen;
    }
    if (msg.msg_control_is_user) {
    struct cmsghdr __user *cm = msg.msg_control_user;
    check_object_size(data, cmlen - sizeof(*cm), true);
    scoped_user_write_access_size(cm, cmlen, efault) {
    unsafe_put_user(cmlen, &cm.cmsg_len, efault);
    unsafe_put_user(level, &cm.cmsg_level, efault);
    unsafe_put_user(type, &cm.cmsg_type, efault);
    unsafe_copy_to_user(CMSG_USER_DATA(cm), data,
    cmlen - sizeof(*cm), efault);
    }
    } else {
    struct cmsghdr *cm = msg.msg_control;
    cm.cmsg_level = level;
    cm.cmsg_type = type;
    cm.cmsg_len = cmlen;
    memcpy(CMSG_DATA(cm), data, cmlen - sizeof(*cm));
    }
    cmlen = min(CMSG_SPACE(len), msg.msg_controllen);
    if (msg.msg_control_is_user)
    msg.msg_control_user += cmlen;
    else
    msg.msg_control += cmlen;
    msg.msg_controllen -= cmlen;
    return 0;
    efault:
    return -EFAULT;
    }
    EXPORT_SYMBOL(put_cmsg);
    int put_cmsg_notrunc(struct msghdr *msg, int level, int type, int len,
    void *data)
    {
// Don't produce truncated CMSGs
    if (!msg.msg_control || msg.msg_controllen < CMSG_LEN(len))
    return -ETOOSMALL;
    return put_cmsg(msg, level, type, len, data);
    }
#[no_mangle]
pub unsafe extern "C" fn put_cmsg_scm_timestamping64(msg: *mut msghdr, tss_internal: *mut scm_timestamping_internal) {
    void put_cmsg_scm_timestamping64(struct msghdr *msg, struct scm_timestamping_internal *tss_internal)
    {
    struct scm_timestamping64 tss;
    int i;
    for (i = 0; i < ARRAY_SIZE(tss.ts); i++) {
    let mut tv: timespec64 = ktime_to_timespec64(tss_internal.ts[i]);
    tss.ts[i].tv_sec = tv.tv_sec;
    tss.ts[i].tv_nsec = tv.tv_nsec;
    }
    put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMPING_NEW, sizeof(tss), &tss);
    }
    EXPORT_SYMBOL(put_cmsg_scm_timestamping64);
#[no_mangle]
pub unsafe extern "C" fn put_cmsg_scm_timestamping(msg: *mut msghdr, tss_internal: *mut scm_timestamping_internal) {
    void put_cmsg_scm_timestamping(struct msghdr *msg, struct scm_timestamping_internal *tss_internal)
    {
    struct scm_timestamping tss;
    int i;
    for (i = 0; i < ARRAY_SIZE(tss.ts); i++) {
    let mut tv: timespec64 = ktime_to_timespec64(tss_internal.ts[i]);
    tss.ts[i].tv_sec = tv.tv_sec;
    tss.ts[i].tv_nsec = tv.tv_nsec;
    }
    put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMPING_OLD, sizeof(tss), &tss);
    }
    EXPORT_SYMBOL(put_cmsg_scm_timestamping);
#[no_mangle]
unsafe extern "C" fn scm_max_fds(msg: *mut msghdr) -> c_int {
    static int scm_max_fds(struct msghdr *msg)
    {
    if (msg.msg_controllen <= sizeof(struct cmsghdr))
    return 0;
    return (msg.msg_controllen - sizeof(struct cmsghdr)) / sizeof(int);
    }
    int scm_recv_one_fd(struct file *f, int __user *ufd, unsigned int flags,
    bool notrunc)
    {
    int error;
    if (!ufd)
    return -EFAULT;
    error = security_file_receive(f);
    if (error)
    return notrunc ? put_user(error, ufd) : error;
    FD_PREPARE(fdf, flags, get_file(f));
    if (fdf.err)
    return fdf.err;
    error = put_user(fd_prepare_fd(fdf), ufd);
    if (error)
    return error;
    __receive_sock(fd_prepare_file(fdf));
    return fd_publish(fdf);
    }
#[no_mangle]
pub unsafe extern "C" fn scm_detach_fds(msg: *mut msghdr, scm: *mut scm_cookie, notrunc: bool) {
    void scm_detach_fds(struct msghdr *msg, struct scm_cookie *scm, bool notrunc)
    {
    struct cmsghdr __user *cm =
    ( struct cmsghdr __user *)msg.msg_control_user;
    let mut o_flags: c_uint = (msg.msg_flags & MSG_CMSG_CLOEXEC) ? O_CLOEXEC : 0;
    let mut fdmax: c_int = min_t(int, scm_max_fds(msg), scm.fp.count);
    int __user *cmsg_data = CMSG_USER_DATA(cm);
    let mut err: c_int = 0, i;
// no use for FD passing from kernel space callers
    if (WARN_ON_ONCE(!msg.msg_control_is_user))
    return;
    if (msg.msg_flags & MSG_CMSG_COMPAT) {
    scm_detach_fds_compat(msg, scm, notrunc);
    return;
    }
    for (i = 0; i < fdmax; i++) {
    err = scm_recv_one_fd(scm.fp.fp[i], cmsg_data + i, o_flags, notrunc);
    if (err < 0)
    break;
    }
    if (i > 0) {
    let mut cmlen: c_int = CMSG_LEN(i * sizeof(int));
    err = put_user(SOL_SOCKET, &cm.cmsg_level);
    if (!err)
    err = put_user(SCM_RIGHTS, &cm.cmsg_type);
    if (!err)
    err = put_user(cmlen, &cm.cmsg_len);
    if (!err) {
    cmlen = CMSG_SPACE(i * sizeof(int));
    if (msg.msg_controllen < cmlen)
    cmlen = msg.msg_controllen;
    msg.msg_control_user += cmlen;
    msg.msg_controllen -= cmlen;
    }
    }
    if (i < scm.fp.count || (scm.fp.count && fdmax <= 0))
    msg.msg_flags |= MSG_CTRUNC;
//
// All of the files that fit in the message have had their usage counts
// incremented, so we just free the list.
//
    __scm_destroy(scm);
    }
    EXPORT_SYMBOL(scm_detach_fds);
    struct scm_fp_list *scm_fp_dup(struct scm_fp_list *fpl)
    {
    struct scm_fp_list *new_fpl;
    int i;
    if (!fpl)
    return core::ptr::null_mut();
    new_fpl = kmemdup(fpl, offsetof(struct scm_fp_list, fp[fpl.count]),
    GFP_KERNEL_ACCOUNT);
    if (new_fpl) {
    for (i = 0; i < fpl.count; i++)
    get_file(fpl.fp[i]);
    new_fpl.max = new_fpl.count;
    new_fpl.user = get_uid(fpl.user);

    new_fpl.inflight = false;
    new_fpl.edges = core::ptr::null_mut();
    INIT_LIST_HEAD(&new_fpl.vertices);

    }
    return new_fpl;
    }
    EXPORT_SYMBOL(scm_fp_dup);

#[no_mangle]
unsafe extern "C" fn scm_passec(sk: *mut sock, msg: *mut msghdr, scm: *mut scm_cookie) {
    static void scm_passec(struct sock *sk, struct msghdr *msg, struct scm_cookie *scm)
    {
    struct lsm_context ctx;
    int err;
    if (sk.sk_scm_security) {
    err = security_secid_to_secctx(scm.secid, &ctx);
    if (err >= 0) {
    put_cmsg(msg, SOL_SOCKET, SCM_SECURITY, ctx.len,
    ctx.context);
    security_release_secctx(&ctx);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn scm_has_secdata(sk: *mut sock) -> bool {
    static bool scm_has_secdata(struct sock *sk)
    {
    return sk.sk_scm_security;
    }

#[no_mangle]
unsafe extern "C" fn scm_passec(sk: *mut sock, msg: *mut msghdr, scm: *mut scm_cookie) {
    static void scm_passec(struct sock *sk, struct msghdr *msg, struct scm_cookie *scm)
    {
    }
#[no_mangle]
unsafe extern "C" fn scm_has_secdata(sk: *mut sock) -> bool {
    static bool scm_has_secdata(struct sock *sk)
    {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn scm_pidfd_recv(msg: *mut msghdr, scm: *mut scm_cookie) {
    static void scm_pidfd_recv(struct msghdr *msg, struct scm_cookie *scm)
    {
    struct file *pidfd_file = core::ptr::null_mut();
    int len, pidfd;
// put_cmsg() doesn't return an error if CMSG is truncated,
// that's why we need to opencode these checks here.
//
    if (msg.msg_flags & MSG_CMSG_COMPAT)
    len = sizeof(struct compat_cmsghdr) + sizeof(int);
    else
    len = sizeof(struct cmsghdr) + sizeof(int);
    if (msg.msg_controllen < len) {
    msg.msg_flags |= MSG_CTRUNC;
    return;
    }
    if (!scm.pid)
    return;
    pidfd = pidfd_prepare(scm.pid, PIDFD_STALE, &pidfd_file);
    if (put_cmsg(msg, SOL_SOCKET, SCM_PIDFD, sizeof(int), &pidfd)) {
    if (pidfd_file) {
    put_unused_fd(pidfd);
    fput(pidfd_file);
    }
    return;
    }
    if (pidfd_file)
    fd_install(pidfd, pidfd_file);
    }
    static bool __scm_recv_common(struct sock *sk, struct msghdr *msg,
    struct scm_cookie *scm, int flags)
    {
    if (!msg.msg_control) {
    if (sk.sk_scm_credentials || sk.sk_scm_pidfd ||
    scm.fp || scm_has_secdata(sk))
    msg.msg_flags |= MSG_CTRUNC;
    scm_destroy(scm);
    return false;
    }
    if (sk.sk_scm_credentials) {
    struct user_namespace *current_ns = current_user_ns();
    struct ucred ucreds = {
    .pid = scm.creds.pid,
    .uid = from_kuid_munged(current_ns, scm.creds.uid),
    .gid = from_kgid_munged(current_ns, scm.creds.gid),
    };
    put_cmsg(msg, SOL_SOCKET, SCM_CREDENTIALS, sizeof(ucreds), &ucreds);
    }
    scm_passec(sk, msg, scm);
    return true;
    }
    void scm_recv(struct socket *sock, struct msghdr *msg,
    struct scm_cookie *scm, int flags)
    {
    if (!__scm_recv_common(sock.sk, msg, scm, flags))
    return;
    scm_destroy_cred(scm);
    }
    EXPORT_SYMBOL(scm_recv);
    void scm_recv_unix(struct socket *sock, struct msghdr *msg,
    struct scm_cookie *scm, int flags)
    {
    if (!__scm_recv_common(sock.sk, msg, scm, flags))
    return;
    if (scm.fp) {
    struct unix_sock *u;
    u = unix_sk(sock.sk);
    scm_detach_fds(msg, scm, READ_ONCE(u.scm_rights_notrunc));
    }
    if (sock.sk.sk_scm_pidfd)
    scm_pidfd_recv(msg, scm);
    scm_destroy_cred(scm);
    }
