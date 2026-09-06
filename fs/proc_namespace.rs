//! Automatically rewritten from C to Rust
//! Source: fs/proc_namespace.c
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
// fs/proc_namespace.c - handling of /proc/<pid>/{mounts,mountinfo,mountstats}
//
// In fact, that's a piece of procfs; it's *almost* isolated from
// the rest of fs/proc, but has rather close relationships with
// fs/namespace.c, thus here instead of fs/proc
//

#[no_mangle]
unsafe extern "C" fn mounts_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t mounts_poll(struct file *file, poll_table *wait)
    {
    struct seq_file *m = file.private_data;
    struct proc_mounts *p = m.private;
    struct mnt_namespace *ns = p.ns;
    let mut res: __poll_t = EPOLLIN | EPOLLRDNORM;
    int event;
    poll_wait(file, &p.ns.poll, wait);
    event = READ_ONCE(ns.event);
    if (m.poll_event != event) {
    m.poll_event = event;
    res |= EPOLLERR | EPOLLPRI;
    }
    return res;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_fs_opts {
    pub flag: c_int,
    pub str: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn show_sb_opts(m: *mut seq_file, sb: *mut super_block) -> c_int {
    static int show_sb_opts(struct seq_file *m, struct super_block *sb)
    {
    static const struct proc_fs_opts fs_opts[] = {
    { SB_SYNCHRONOUS, ",sync" },
    { SB_DIRSYNC, ",dirsync" },
    { SB_MANDLOCK, ",mand" },
    { SB_LAZYTIME, ",lazytime" },
    { 0, core::ptr::null_mut() }
    };
    const struct proc_fs_opts *fs_infop;
    for (fs_infop = fs_opts; fs_infop.flag; fs_infop++) {
    if (sb.s_flags & fs_infop.flag)
    seq_puts(m, fs_infop.str);
    }
    return security_sb_show_options(m, sb);
    }
#[no_mangle]
unsafe extern "C" fn show_vfsmnt_opts(m: *mut seq_file, mnt: *mut vfsmount) {
    static void show_vfsmnt_opts(struct seq_file *m, struct vfsmount *mnt)
    {
    static const struct proc_fs_opts mnt_opts[] = {
    { MNT_NOSUID, ",nosuid" },
    { MNT_NODEV, ",nodev" },
    { MNT_NOEXEC, ",noexec" },
    { MNT_NOATIME, ",noatime" },
    { MNT_NODIRATIME, ",nodiratime" },
    { MNT_RELATIME, ",relatime" },
    { MNT_NOSYMFOLLOW, ",nosymfollow" },
    { 0, core::ptr::null_mut() }
    };
    const struct proc_fs_opts *fs_infop;
    for (fs_infop = mnt_opts; fs_infop.flag; fs_infop++) {
    if (mnt.mnt_flags & fs_infop.flag)
    seq_puts(m, fs_infop.str);
    }
    if (is_idmapped_mnt(mnt))
    seq_puts(m, ",idmapped");
    }
#[no_mangle]
pub unsafe extern "C" fn mangle(m: *mut seq_file, s: *const c_char) {
    static inline void mangle(struct seq_file *m, const char *s)
    {
    seq_escape(m, s, " \t\n\\#");
    }
#[no_mangle]
unsafe extern "C" fn show_type(m: *mut seq_file, sb: *mut super_block) {
    static void show_type(struct seq_file *m, struct super_block *sb)
    {
    mangle(m, sb.s_type.name);
    if (sb.s_subtype) {
    seq_putc(m, '.');
    mangle(m, sb.s_subtype);
    }
    }
#[no_mangle]
unsafe extern "C" fn show_vfsmnt(m: *mut seq_file, mnt: *mut vfsmount) -> c_int {
    static int show_vfsmnt(struct seq_file *m, struct vfsmount *mnt)
    {
    struct proc_mounts *p = m.private;
    struct mount *r = real_mount(mnt);
    let mut mnt_path: path = { .dentry = mnt.mnt_root, .mnt = mnt };
    struct super_block *sb = mnt_path.dentry.d_sb;
    int err;
    if (sb.s_op.show_devname) {
    err = sb.s_op.show_devname(m, mnt_path.dentry);
    if (err)
    goto out;
    } else {
    mangle(m, r.mnt_devname);
    }
    seq_putc(m, ' ');
// mountpoints outside of chroot jail will give SEQ_SKIP on this
    err = seq_path_root(m, &mnt_path, &p.root, " \t\n\\");
    if (err)
    goto out;
    seq_putc(m, ' ');
    show_type(m, sb);
    seq_puts(m, __mnt_is_readonly(mnt) ? " ro" : " rw");
    err = show_sb_opts(m, sb);
    if (err)
    goto out;
    show_vfsmnt_opts(m, mnt);
    if (sb.s_op.show_options)
    err = sb.s_op.show_options(m, mnt_path.dentry);
    seq_puts(m, " 0 0\n");
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn show_mountinfo(m: *mut seq_file, mnt: *mut vfsmount) -> c_int {
    static int show_mountinfo(struct seq_file *m, struct vfsmount *mnt)
    {
    struct proc_mounts *p = m.private;
    struct mount *r = real_mount(mnt);
    struct super_block *sb = mnt.mnt_sb;
    let mut mnt_path: path = { .dentry = mnt.mnt_root, .mnt = mnt };
    int err;
    seq_printf(m, "%i %i %u:%u ", r.mnt_id, r.mnt_parent.mnt_id,
    MAJOR(sb.s_dev), MINOR(sb.s_dev));
    err = show_path(m, mnt.mnt_root);
    if (err)
    goto out;
    seq_putc(m, ' ');
// mountpoints outside of chroot jail will give SEQ_SKIP on this
    err = seq_path_root(m, &mnt_path, &p.root, " \t\n\\");
    if (err)
    goto out;
    seq_puts(m, mnt.mnt_flags & MNT_READONLY ? " ro" : " rw");
    show_vfsmnt_opts(m, mnt);
// Tagged fields ("foo:X" or "bar")
    if (IS_MNT_SHARED(r))
    seq_printf(m, " shared:%i", r.mnt_group_id);
    if (IS_MNT_SLAVE(r)) {
    let mut master: c_int = r.mnt_master.mnt_group_id;
    let mut dom: c_int = get_dominating_id(r, &p.root);
    seq_printf(m, " master:%i", master);
    if (dom && dom != master)
    seq_printf(m, " propagate_from:%i", dom);
    }
    if (IS_MNT_UNBINDABLE(r))
    seq_puts(m, " unbindable");
// Filesystem specific data
    seq_puts(m, " - ");
    show_type(m, sb);
    seq_putc(m, ' ');
    if (sb.s_op.show_devname) {
    err = sb.s_op.show_devname(m, mnt.mnt_root);
    if (err)
    goto out;
    } else {
    mangle(m, r.mnt_devname);
    }
    seq_puts(m, sb_rdonly(sb) ? " ro" : " rw");
    err = show_sb_opts(m, sb);
    if (err)
    goto out;
    if (sb.s_op.show_options)
    err = sb.s_op.show_options(m, mnt.mnt_root);
    seq_putc(m, '\n');
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn show_vfsstat(m: *mut seq_file, mnt: *mut vfsmount) -> c_int {
    static int show_vfsstat(struct seq_file *m, struct vfsmount *mnt)
    {
    struct proc_mounts *p = m.private;
    struct mount *r = real_mount(mnt);
    let mut mnt_path: path = { .dentry = mnt.mnt_root, .mnt = mnt };
    struct super_block *sb = mnt_path.dentry.d_sb;
    int err;
// device
    seq_puts(m, "device ");
    if (sb.s_op.show_devname) {
    err = sb.s_op.show_devname(m, mnt_path.dentry);
    if (err)
    goto out;
    } else {
    mangle(m, r.mnt_devname);
    }
// mount point
    seq_puts(m, " mounted on ");
// mountpoints outside of chroot jail will give SEQ_SKIP on this
    err = seq_path_root(m, &mnt_path, &p.root, " \t\n\\");
    if (err)
    goto out;
    seq_putc(m, ' ');
// file system type
    seq_puts(m, "with fstype ");
    show_type(m, sb);
// optional statistics
    if (sb.s_op.show_stats) {
    seq_putc(m, ' ');
    err = sb.s_op.show_stats(m, mnt_path.dentry);
    }
    seq_putc(m, '\n');
    out:
    return err;
    }
    static int mounts_open_common(struct inode *inode, struct file *file,
    int (*show)(struct seq_file *, struct vfsmount *))
    {
    struct task_struct *task = get_proc_task(inode);
    struct nsproxy *nsp;
    struct mnt_namespace *ns = core::ptr::null_mut();
    struct path root;
    struct proc_mounts *p;
    struct seq_file *m;
    let mut ret: c_int = -EINVAL;
    if (!task)
    goto err;
    task_lock(task);
    nsp = task.nsproxy;
    if (!nsp || !nsp.mnt_ns) {
    task_unlock(task);
    put_task_struct(task);
    goto err;
    }
    ns = nsp.mnt_ns;
    get_mnt_ns(ns);
    if (!task.real_fs) {
    task_unlock(task);
    put_task_struct(task);
    ret = -ENOENT;
    goto err_put_ns;
    }
    get_fs_root(task.real_fs, &root);
    task_unlock(task);
    put_task_struct(task);
    ret = seq_open_private(file, &mounts_op, sizeof(struct proc_mounts));
    if (ret)
    goto err_put_path;
    m = file.private_data;
    m.poll_event = ns.event;
    p = m.private;
    p.ns = ns;
    p.root = root;
    p.show = show;
    return 0;
    err_put_path:
    path_put(&root);
    err_put_ns:
    put_mnt_ns(ns);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mounts_release(inode: *mut inode, file: *mut file) -> c_int {
    static int mounts_release(struct inode *inode, struct file *file)
    {
    struct seq_file *m = file.private_data;
    struct proc_mounts *p = m.private;
    path_put(&p.root);
    put_mnt_ns(p.ns);
    return seq_release_private(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn mounts_open(inode: *mut inode, file: *mut file) -> c_int {
    static int mounts_open(struct inode *inode, struct file *file)
    {
    return mounts_open_common(inode, file, show_vfsmnt);
    }
#[no_mangle]
unsafe extern "C" fn mountinfo_open(inode: *mut inode, file: *mut file) -> c_int {
    static int mountinfo_open(struct inode *inode, struct file *file)
    {
    return mounts_open_common(inode, file, show_mountinfo);
    }
#[no_mangle]
unsafe extern "C" fn mountstats_open(inode: *mut inode, file: *mut file) -> c_int {
    static int mountstats_open(struct inode *inode, struct file *file)
    {
    return mounts_open_common(inode, file, show_vfsstat);
    }
    const struct file_operations proc_mounts_operations = {
    .open		= mounts_open,
    .read_iter	= seq_read_iter,
    .splice_read	= copy_splice_read,
    .llseek		= seq_lseek,
    .release	= mounts_release,
    .poll		= mounts_poll,
    };
    const struct file_operations proc_mountinfo_operations = {
    .open		= mountinfo_open,
    .read_iter	= seq_read_iter,
    .splice_read	= copy_splice_read,
    .llseek		= seq_lseek,
    .release	= mounts_release,
    .poll		= mounts_poll,
    };
    const struct file_operations proc_mountstats_operations = {
    .open		= mountstats_open,
    .read_iter	= seq_read_iter,
    .splice_read	= copy_splice_read,
    .llseek		= seq_lseek,
    .release	= mounts_release,
    };
