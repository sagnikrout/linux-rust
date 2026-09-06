//! Automatically rewritten from C to Rust
//! Source: fs/fuse/ioctl.c
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
//
// Copyright (C) 2017 Red Hat, Inc.
//

pub const FUSE_VERITY_ENABLE_ARG_MAX_PAGES: c_int = 256;
    static ssize_t fuse_send_ioctl(struct fuse_mount *fm, struct fuse_args *args,
    struct fuse_ioctl_out *outarg)
    {
    ssize_t ret;
    args.out_args[0].size = sizeof(*outarg);
    args.out_args[0].value = outarg;
    ret = fuse_simple_request(fm, args);
// Translate ENOSYS, which shouldn't be returned from fs
    if (ret == -ENOSYS)
    ret = -ENOTTY;
    if (ret >= 0 && outarg.result == -ENOSYS)
    outarg.result = -ENOTTY;
    return ret;
    }
//
// CUSE servers compiled on 32bit broke on 64bit kernels because the
// ABI was defined to be 'struct iovec' which is different on 32bit
// and 64bit.  Fortunately we can determine which structure the server
// used from the size of the reply.
//
    static int fuse_copy_ioctl_iovec_old(struct iovec *dst, void *src,
    size_t transferred, unsigned count,
    bool is_compat)
    {

    if (count * sizeof(struct compat_iovec) == transferred) {
    struct compat_iovec *ciov = src;
    unsigned i;
//
// With this interface a 32bit server cannot support
// non-compat (i.e. ones coming from 64bit apps) ioctl
// requests
//
    if (!is_compat)
    return -EINVAL;
    for (i = 0; i < count; i++) {
    dst[i].iov_base = compat_ptr(ciov[i].iov_base);
    dst[i].iov_len = ciov[i].iov_len;
    }
    return 0;
    }

    if (count * sizeof(struct iovec) != transferred)
    return -EIO;
    memcpy(dst, src, transferred);
    return 0;
    }
// Make sure iov_length() won't overflow
    static int fuse_verify_ioctl_iov(struct fuse_conn *fc, struct iovec *iov,
    size_t count)
    {
    size_t n;
    let mut max: u32 = fc.max_pages << PAGE_SHIFT;
    for (n = 0; n < count; n++, iov++) {
    if (iov.iov_len > (size_t) max)
    return -ENOMEM;
    max -= iov.iov_len;
    }
    return 0;
    }
    static int fuse_copy_ioctl_iovec(struct fuse_conn *fc, struct iovec *dst,
    void *src, size_t transferred, unsigned count,
    bool is_compat)
    {
    unsigned i;
    struct fuse_ioctl_iovec *fiov = src;
    if (fc.minor < 16) {
    return fuse_copy_ioctl_iovec_old(dst, src, transferred,
    count, is_compat);
    }
    if (count * sizeof(struct fuse_ioctl_iovec) != transferred)
    return -EIO;
    for (i = 0; i < count; i++) {
// Did the server supply an inappropriate value?
    if (fiov[i].base != (unsigned long) fiov[i].base ||
    fiov[i].len != (unsigned long) fiov[i].len)
    return -EIO;
    dst[i].iov_base = (void __user *) (unsigned long) fiov[i].base;
    dst[i].iov_len = (size_t) fiov[i].len;

    if (is_compat &&
    (ptr_to_compat(dst[i].iov_base) != fiov[i].base ||
    (compat_size_t) dst[i].iov_len != fiov[i].len))
    return -EIO;

    }
    return 0;
    }
// For fs-verity, determine iov lengths from input
#[no_mangle]
unsafe extern "C" fn fuse_setup_measure_verity(arg: c_ulong, iov: *mut iovec) -> c_int {
    static int fuse_setup_measure_verity(unsigned long arg, struct iovec *iov)
    {
    __u16 digest_size;
    struct fsverity_digest __user *uarg = (void __user *)arg;
    if (copy_from_user(&digest_size, &uarg.digest_size, sizeof(digest_size)))
    return -EFAULT;
    if (digest_size > SIZE_MAX - sizeof(struct fsverity_digest))
    return -EINVAL;
    iov.iov_len = sizeof(struct fsverity_digest) + digest_size;
    return 0;
    }
    static int fuse_setup_enable_verity(unsigned long arg, struct iovec *iov,
    unsigned int *in_iovs)
    {
    struct fsverity_enable_arg enable;
    struct fsverity_enable_arg __user *uarg = (void __user *)arg;
    let mut max_buffer_len: __u32 = FUSE_VERITY_ENABLE_ARG_MAX_PAGES * PAGE_SIZE;
    if (copy_from_user(&enable, uarg, sizeof(enable)))
    return -EFAULT;
    if (enable.salt_size > max_buffer_len || enable.sig_size > max_buffer_len)
    return -ENOMEM;
    if (enable.salt_size > 0) {
    iov++;
    (*in_iovs)++;
    iov.iov_base = u64_to_user_ptr(enable.salt_ptr);
    iov.iov_len = enable.salt_size;
    }
    if (enable.sig_size > 0) {
    iov++;
    (*in_iovs)++;
    iov.iov_base = u64_to_user_ptr(enable.sig_ptr);
    iov.iov_len = enable.sig_size;
    }
    return 0;
    }
//
// For ioctls, there is no generic way to determine how much memory
// needs to be read and/or written.  Furthermore, ioctls are allowed
// to dereference the passed pointer, so the parameter requires deep
// copying but FUSE has no idea whatsoever about what to copy in or
// out.
//
// This is solved by allowing FUSE server to retry ioctl with
// necessary in/out iovecs.  Let's assume the ioctl implementation
// needs to read in the following structure.
//
// struct a {
// char	*buf;
// size_t	buflen;
// }
//
// On the first callout to FUSE server, inarg->in_size and
// inarg->out_size will be NULL; then, the server completes the ioctl
// with FUSE_IOCTL_RETRY set in out->flags, out->in_iovs set to 1 and
// the actual iov array to
//
// { { .iov_base = inarg.arg,	.iov_len = sizeof(struct a) } }
//
// which tells FUSE to copy in the requested area and retry the ioctl.
// On the second round, the server has access to the structure and
// from that it can tell what to look for next, so on the invocation,
// it sets FUSE_IOCTL_RETRY, out->in_iovs to 2 and iov array to
//
// { { .iov_base = inarg.arg,	.iov_len = sizeof(struct a)	},
// { .iov_base = a.buf,	.iov_len = a.buflen		} }
//
// FUSE will copy both struct a and the pointed buffer from the
// process doing the ioctl and retry ioctl with both struct a and the
// buffer.
//
// This time, FUSE server has everything it needs and completes ioctl
// without FUSE_IOCTL_RETRY which finishes the ioctl call.
//
// Copying data out works the same way.
//
// Note that if FUSE_IOCTL_UNRESTRICTED is clear, the kernel
// automatically initializes in and out iovs by decoding @cmd with
// _IOC_* macros and the server is not allowed to request RETRY.  This
// limits ioctl data transfers to well-formed ioctls and is the forced
// behavior for all FUSE servers.
//
    long fuse_do_ioctl(struct file *file, unsigned int cmd, unsigned long arg,
    unsigned int flags)
    {
    struct fuse_file *ff = file.private_data;
    struct fuse_mount *fm = ff.fm;
    struct fuse_ioctl_in inarg = {
    .fh = ff.fh,
    .cmd = cmd,
    .arg = arg,
    .flags = flags
    };
    struct fuse_ioctl_out outarg;
    struct iovec *iov_page = core::ptr::null_mut();
    struct iovec *in_iov = core::ptr::null_mut(), *out_iov = core::ptr::null_mut();
    let mut in_iovs: c_uint = 0, out_iovs = 0, max_pages;
    size_t in_size, out_size, c;
    ssize_t transferred;
    int err, i;
    struct iov_iter ii;
    let mut ap: fuse_args_pages = {};

    inarg.flags |= FUSE_IOCTL_32BIT;

    if (flags & FUSE_IOCTL_COMPAT) {
    inarg.flags |= FUSE_IOCTL_32BIT;

    if (in_x32_syscall())
    inarg.flags |= FUSE_IOCTL_COMPAT_X32;

    }

// assume all the iovs returned by client always fits in a page
    BUILD_BUG_ON(sizeof(struct fuse_ioctl_iovec) * FUSE_IOCTL_MAX_IOV > PAGE_SIZE);
    err = -ENOMEM;
    ap.folios = fuse_folios_alloc(fm.fc.max_pages, GFP_KERNEL, &ap.descs);
    iov_page = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!ap.folios || !iov_page)
    goto out;
    fuse_folio_descs_length_init(ap.descs, 0, fm.fc.max_pages);
//
// If restricted, initialize IO parameters as encoded in @cmd.
// RETRY from server is not allowed.
//
    if (!(flags & FUSE_IOCTL_UNRESTRICTED)) {
    struct iovec *iov = iov_page;
    iov.iov_base = (void __user *)arg;
    iov.iov_len = _IOC_SIZE(cmd);
    if (_IOC_DIR(cmd) & _IOC_WRITE) {
    in_iov = iov;
    in_iovs = 1;
    }
    if (_IOC_DIR(cmd) & _IOC_READ) {
    out_iov = iov;
    out_iovs = 1;
    }
    err = 0;
    switch (cmd) {
    case FS_IOC_MEASURE_VERITY:
    err = fuse_setup_measure_verity(arg, iov);
    break;
    case FS_IOC_ENABLE_VERITY:
    err = fuse_setup_enable_verity(arg, iov, &in_iovs);
    break;
    }
    if (err)
    goto out;
    }
    retry:
    inarg.in_size = in_size = iov_length(in_iov, in_iovs);
    inarg.out_size = out_size = iov_length(out_iov, out_iovs);
//
// Out data can be used either for actual out data or iovs,
// make sure there always is at least one page.
//
    out_size = max_t(size_t, out_size, PAGE_SIZE);
    max_pages = DIV_ROUND_UP(max(in_size, out_size), PAGE_SIZE);
// make sure there are enough buffer pages and init request with them
    err = -ENOMEM;
    if (max_pages > fm.fc.max_pages)
    goto out;
    while (ap.num_folios < max_pages) {
    ap.folios[ap.num_folios] = folio_alloc(GFP_KERNEL | __GFP_HIGHMEM, 0);
    if (!ap.folios[ap.num_folios])
    goto out;
    ap.num_folios++;
    }
// okay, let's send it to the client
    ap.args.opcode = FUSE_IOCTL;
    ap.args.nodeid = ff.nodeid;
    ap.args.in_numargs = 1;
    ap.args.in_args[0].size = sizeof(inarg);
    ap.args.in_args[0].value = &inarg;
    if (in_size) {
    ap.args.in_numargs++;
    ap.args.in_args[1].size = in_size;
    ap.args.in_pages = true;
    err = -EFAULT;
    iov_iter_init(&ii, ITER_SOURCE, in_iov, in_iovs, in_size);
    for (i = 0; iov_iter_count(&ii) && !WARN_ON(i >= ap.num_folios); i++) {
    c = copy_folio_from_iter(ap.folios[i], 0, PAGE_SIZE, &ii);
    if (c != PAGE_SIZE && iov_iter_count(&ii))
    goto out;
    }
    }
    ap.args.out_numargs = 2;
    ap.args.out_args[1].size = out_size;
    ap.args.out_pages = true;
    ap.args.out_argvar = true;
    transferred = fuse_send_ioctl(fm, &ap.args, &outarg);
    err = transferred;
    if (transferred < 0)
    goto out;
// did it ask for retry?
    if (outarg.flags & FUSE_IOCTL_RETRY) {
    void *vaddr;
// no retry if in restricted mode
    err = -EIO;
    if (!(flags & FUSE_IOCTL_UNRESTRICTED))
    goto out;
    in_iovs = outarg.in_iovs;
    out_iovs = outarg.out_iovs;
//
// Make sure things are in boundary, separate checks
// are to protect against overflow.
//
    err = -ENOMEM;
    if (in_iovs > FUSE_IOCTL_MAX_IOV ||
    out_iovs > FUSE_IOCTL_MAX_IOV ||
    in_iovs + out_iovs > FUSE_IOCTL_MAX_IOV)
    goto out;
    vaddr = kmap_local_folio(ap.folios[0], 0);
    err = fuse_copy_ioctl_iovec(fm.fc, iov_page, vaddr,
    transferred, in_iovs + out_iovs,
    (flags & FUSE_IOCTL_COMPAT) != 0);
    kunmap_local(vaddr);
    if (err)
    goto out;
    in_iov = iov_page;
    out_iov = in_iov + in_iovs;
    err = fuse_verify_ioctl_iov(fm.fc, in_iov, in_iovs);
    if (err)
    goto out;
    err = fuse_verify_ioctl_iov(fm.fc, out_iov, out_iovs);
    if (err)
    goto out;
    goto retry;
    }
    err = -EIO;
    if (transferred > inarg.out_size)
    goto out;
    err = -EFAULT;
    iov_iter_init(&ii, ITER_DEST, out_iov, out_iovs, transferred);
    for (i = 0; iov_iter_count(&ii) && !WARN_ON(i >= ap.num_folios); i++) {
    c = copy_folio_to_iter(ap.folios[i], 0, PAGE_SIZE, &ii);
    if (c != PAGE_SIZE && iov_iter_count(&ii))
    goto out;
    }
    err = 0;
    out:
    kfree(iov_page);
    while (ap.num_folios)
    folio_put(ap.folios[--ap.num_folios]);
    kfree(ap.folios);
    return err ? err : outarg.result;
    }
    EXPORT_SYMBOL_GPL(fuse_do_ioctl);
    long fuse_ioctl_common(struct file *file, unsigned int cmd,
    unsigned long arg, unsigned int flags)
    {
    struct inode *inode = file_inode(file);
    struct fuse_conn *fc = get_fuse_conn(inode);
    if (!fuse_allow_current_process(fc))
    return -EACCES;
    if (fuse_is_bad(inode))
    return -EIO;
    return fuse_do_ioctl(file, cmd, arg, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn fuse_file_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    long fuse_file_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    return fuse_ioctl_common(file, cmd, arg, 0);
    }
    long fuse_file_compat_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    return fuse_ioctl_common(file, cmd, arg, FUSE_IOCTL_COMPAT);
    }
    static int fuse_priv_ioctl(struct inode *inode, struct fuse_file *ff,
    unsigned int cmd, void *ptr, size_t size)
    {
    struct fuse_mount *fm = ff.fm;
    struct fuse_ioctl_in inarg;
    struct fuse_ioctl_out outarg;
    FUSE_ARGS(args);
    int err;
    memset(&inarg, 0, sizeof(inarg));
    inarg.fh = ff.fh;
    inarg.cmd = cmd;

    inarg.flags |= FUSE_IOCTL_32BIT;

    if (S_ISDIR(inode.i_mode))
    inarg.flags |= FUSE_IOCTL_DIR;
    if (_IOC_DIR(cmd) & _IOC_READ)
    inarg.out_size = size;
    if (_IOC_DIR(cmd) & _IOC_WRITE)
    inarg.in_size = size;
    args.opcode = FUSE_IOCTL;
    args.nodeid = ff.nodeid;
    args.in_numargs = 2;
    args.in_args[0].size = sizeof(inarg);
    args.in_args[0].value = &inarg;
    args.in_args[1].size = inarg.in_size;
    args.in_args[1].value = ptr;
    args.out_numargs = 2;
    args.out_args[1].size = inarg.out_size;
    args.out_args[1].value = ptr;
    err = fuse_send_ioctl(fm, &args, &outarg);
    if (!err) {
    if (outarg.result < 0)
    err = outarg.result;
#[no_mangle]
pub unsafe extern "C" fn if(FUSE_IOCTL_RETRY: outarg.flags &) -> else {
    else if (outarg.flags & FUSE_IOCTL_RETRY)
    err = -EIO;
    }
    return err;
    }
    static struct fuse_file *fuse_priv_ioctl_prepare(struct inode *inode)
    {
    struct fuse_mount *fm = get_fuse_mount(inode);
    let mut isdir: bool = S_ISDIR(inode.i_mode);
    if (!fuse_allow_current_process(fm.fc))
    return ERR_PTR(-EACCES);
    if (fuse_is_bad(inode))
    return ERR_PTR(-EIO);
    if (!S_ISREG(inode.i_mode) && !isdir)
    return ERR_PTR(-ENOTTY);
    return fuse_file_open(fm, get_node_id(inode), O_RDONLY, isdir);
    }
#[no_mangle]
unsafe extern "C" fn fuse_priv_ioctl_cleanup(inode: *mut inode, ff: *mut fuse_file) {
    static void fuse_priv_ioctl_cleanup(struct inode *inode, struct fuse_file *ff)
    {
    fuse_file_release(inode, ff, O_RDONLY, core::ptr::null_mut(), S_ISDIR(inode.i_mode));
    }
#[no_mangle]
pub unsafe extern "C" fn fuse_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    int fuse_fileattr_get(struct dentry *dentry, struct file_kattr *fa)
    {
    struct inode *inode = d_inode(dentry);
    struct fuse_file *ff;
    unsigned int flags;
    struct fsxattr xfa;
    int err;
    ff = fuse_priv_ioctl_prepare(inode);
    if (IS_ERR(ff))
    return PTR_ERR(ff);
    if (fa.flags_valid) {
    err = fuse_priv_ioctl(inode, ff, FS_IOC_GETFLAGS,
    &flags, sizeof(flags));
    if (err)
    goto cleanup;
    fileattr_fill_flags(fa, flags);
    } else {
    err = fuse_priv_ioctl(inode, ff, FS_IOC_FSGETXATTR,
    &xfa, sizeof(xfa));
    if (err)
    goto cleanup;
    fileattr_fill_xflags(fa, xfa.fsx_xflags);
    fa.fsx_extsize = xfa.fsx_extsize;
    fa.fsx_nextents = xfa.fsx_nextents;
    fa.fsx_projid = xfa.fsx_projid;
    fa.fsx_cowextsize = xfa.fsx_cowextsize;
    }
    cleanup:
    fuse_priv_ioctl_cleanup(inode, ff);
    return err;
    }
    int fuse_fileattr_set(struct mnt_idmap *idmap,
    struct dentry *dentry, struct file_kattr *fa)
    {
    struct inode *inode = d_inode(dentry);
    struct fuse_file *ff;
    let mut flags: c_uint = fa.flags;
    struct fsxattr xfa;
    int err;
    ff = fuse_priv_ioctl_prepare(inode);
    if (IS_ERR(ff))
    return PTR_ERR(ff);
    if (fa.flags_valid) {
    err = fuse_priv_ioctl(inode, ff, FS_IOC_SETFLAGS,
    &flags, sizeof(flags));
    if (err)
    goto cleanup;
    } else {
    memset(&xfa, 0, sizeof(xfa));
    xfa.fsx_xflags = fa.fsx_xflags;
    xfa.fsx_extsize = fa.fsx_extsize;
    xfa.fsx_nextents = fa.fsx_nextents;
    xfa.fsx_projid = fa.fsx_projid;
    xfa.fsx_cowextsize = fa.fsx_cowextsize;
    err = fuse_priv_ioctl(inode, ff, FS_IOC_FSSETXATTR,
    &xfa, sizeof(xfa));
    }
    cleanup:
    fuse_priv_ioctl_cleanup(inode, ff);
    return err;
    }
