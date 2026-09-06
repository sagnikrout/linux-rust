//! Automatically rewritten from C to Rust
//! Source: fs/devpts/inode.c
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
// -*- linux-c -*- ---------------------------------------------------------
//
// linux/fs/devpts/inode.c
//
// Copyright 1998-2004 H. Peter Anvin -- All Rights Reserved
//
// -------------------------------------------------------------------------

pub const DEVPTS_DEFAULT_MODE: c_int = 0600;
//
// ptmx is a new node in /dev/pts and will be unused in legacy (single-
// instance) mode. To prevent surprises in user space, set permissions of
// ptmx to 0. Use 'chmod' or remount with '-o ptmxmode' to set meaningful
// permissions.
//
pub const DEVPTS_DEFAULT_PTMX_MODE: c_int = 0000;
pub const PTMX_MINOR: c_int = 2;
//
// sysctl support for setting limits on the number of Unix98 ptys allocated.
// Otherwise one can eat up all kernel memory by opening /dev/ptmx repeatedly.
//
    let mut pty_limit: static int = NR_UNIX98_PTY_DEFAULT;
    let mut pty_reserve: static int = NR_UNIX98_PTY_RESERVE;
    static int pty_limit_min;
    let mut pty_limit_max: static int = INT_MAX;
    let mut pty_count: static atomic_t = ATOMIC_INIT(0);
    static const struct ctl_table pty_table[] = {
    {
    .procname	= "max",
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .data		= &pty_limit,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &pty_limit_min,
    .extra2		= &pty_limit_max,
    }, {
    .procname	= "reserve",
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .data		= &pty_reserve,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= &pty_limit_min,
    .extra2		= &pty_limit_max,
    }, {
    .procname	= "nr",
    .maxlen		= sizeof(int),
    .mode		= 0444,
    .data		= &pty_count,
    .proc_handler	= proc_dointvec,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pts_mount_opts {
    pub setuid: c_int,
    pub setgid: c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub mode: umode_t,
    pub ptmxmode: umode_t,
    pub reserve: c_int,
    pub max: c_int,
}

    enum {
    Opt_uid, Opt_gid, Opt_mode, Opt_ptmxmode, Opt_newinstance,  Opt_max,
    Opt_err
    };
    static const struct fs_parameter_spec devpts_param_specs[] = {
    fsparam_gid	("gid",		Opt_gid),
    fsparam_s32	("max",		Opt_max),
    fsparam_u32oct	("mode",	Opt_mode),
    fsparam_flag	("newinstance",	Opt_newinstance),
    fsparam_u32oct	("ptmxmode",	Opt_ptmxmode),
    fsparam_uid	("uid",		Opt_uid),
    {}
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pts_fs_info {
    pub allocated_ptys: ida,
    pub mount_opts: pts_mount_opts,
    pub sb: *mut super_block,
    pub borrowed: *mut *mut inode ptmx_inode; //,
}

    static inline struct pts_fs_info *DEVPTS_SB(struct super_block *sb)
    {
    return sb.s_fs_info;
    }
#[no_mangle]
unsafe extern "C" fn devpts_ptmx_path(path: *mut path) -> c_int {
    static int devpts_ptmx_path(struct path *path)
    {
    struct super_block *sb;
    int err;
// Is a devpts filesystem at "pts" in the same directory?
    err = path_pts(path);
    if (err)
    return err;
// Is the path the root of a devpts filesystem?
    sb = path.mnt.mnt_sb;
    if ((sb.s_magic != DEVPTS_SUPER_MAGIC) ||
    (path.mnt.mnt_root != sb.s_root))
    return -ENODEV;
    return 0;
    }
//
// Try to find a suitable devpts filesystem. We support the following
// scenarios:
// - The ptmx device node is located in the same directory as the devpts
// mount where the pts device nodes are located.
// This is e.g. the case when calling open on the /dev/pts/ptmx device
// node when the devpts filesystem is mounted at /dev/pts.
// - The ptmx device node is located outside the devpts filesystem mount
// where the pts device nodes are located. For example, the ptmx device
// is a symlink, separate device node, or bind-mount.
// A supported scenario is bind-mounting /dev/pts/ptmx to /dev/ptmx and
// then calling open on /dev/ptmx. In this case a suitable pts
// subdirectory can be found in the common parent directory /dev of the
// devpts mount and the ptmx bind-mount, after resolving the /dev/ptmx
// bind-mount.
// If no suitable pts subdirectory can be found this function will fail.
// This is e.g. the case when bind-mounting /dev/pts/ptmx to /ptmx.
//
    struct vfsmount *devpts_mntget(struct file *filp, struct pts_fs_info *fsi)
    {
    struct path path;
    let mut err: c_int = 0;
    path = filp.f_path;
    path_get(&path);
// Walk upward while the start point is a bind mount of
// a single file.
//
    while (path.mnt.mnt_root == path.dentry)
    if (follow_up(&path) == 0)
    break;
// devpts_ptmx_path() finds a devpts fs or returns an error.
    if ((path.mnt.mnt_sb.s_magic != DEVPTS_SUPER_MAGIC) ||
    (DEVPTS_SB(path.mnt.mnt_sb) != fsi))
    err = devpts_ptmx_path(&path);
    dput(path.dentry);
    if (!err) {
    if (DEVPTS_SB(path.mnt.mnt_sb) == fsi)
    return path.mnt;
    err = -ENODEV;
    }
    mntput(path.mnt);
    return ERR_PTR(err);
    }
    struct pts_fs_info *devpts_acquire(struct file *filp)
    {
    struct pts_fs_info *result;
    struct path path;
    struct super_block *sb;
    path = filp.f_path;
    path_get(&path);
// Has the devpts filesystem already been found?
    if (path.mnt.mnt_sb.s_magic != DEVPTS_SUPER_MAGIC) {
    int err;
    err = devpts_ptmx_path(&path);
    if (err) {
    result = ERR_PTR(err);
    goto out;
    }
    }
//
// pty code needs to hold extra references in case of last /dev/tty close
//
    sb = path.mnt.mnt_sb;
    atomic_inc(&sb.s_active);
    result = DEVPTS_SB(sb);
    out:
    path_put(&path);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn devpts_release(fsi: *mut pts_fs_info) {
    void devpts_release(struct pts_fs_info *fsi)
    {
    deactivate_super(fsi.sb);
    }
//
// devpts_parse_param - Parse mount parameters
//
#[no_mangle]
unsafe extern "C" fn devpts_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    static int devpts_parse_param(struct fs_context *fc, struct fs_parameter *param)
    {
    struct pts_fs_info *fsi = fc.s_fs_info;
    struct pts_mount_opts *opts = &fsi.mount_opts;
    struct fs_parse_result result;
    int opt;
    opt = fs_parse(fc, devpts_param_specs, param, &result);
    if (opt < 0)
    return opt;
    switch (opt) {
    case Opt_uid:
    opts.uid = result.uid;
    opts.setuid = 1;
    break;
    case Opt_gid:
    opts.gid = result.gid;
    opts.setgid = 1;
    break;
    case Opt_mode:
    opts.mode = result.uint_32 & S_IALLUGO;
    break;
    case Opt_ptmxmode:
    opts.ptmxmode = result.uint_32 & S_IALLUGO;
    break;
    case Opt_newinstance:
    break;
    case Opt_max:
    if (result.uint_32 > NR_UNIX98_PTY_MAX)
    return invalf(fc, "max out of range");
    opts.max = result.uint_32;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mknod_ptmx(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    static int mknod_ptmx(struct super_block *sb, struct fs_context *fc)
    {
    int mode;
    struct dentry *dentry;
    struct inode *inode;
    struct dentry *root = sb.s_root;
    struct pts_fs_info *fsi = DEVPTS_SB(sb);
    struct pts_mount_opts *opts = &fsi.mount_opts;
    let mut ptmx_uid: kuid_t = current_fsuid();
    let mut ptmx_gid: kgid_t = current_fsgid();
    dentry = simple_start_creating(root, "ptmx");
    if (IS_ERR(dentry)) {
    pr_err("Unable to alloc dentry for ptmx node\n");
    return PTR_ERR(dentry);
    }
//
// Create a new 'ptmx' node in this mount of devpts.
//
    inode = new_inode(sb);
    if (!inode) {
    simple_done_creating(dentry);
    pr_err("Unable to alloc inode for ptmx node\n");
    return -ENOMEM;
    }
    inode.i_ino = 2;
    simple_inode_init_ts(inode);
    mode = S_IFCHR|opts.ptmxmode;
    init_special_inode(inode, mode, MKDEV(TTYAUX_MAJOR, 2));
    inode.i_uid = ptmx_uid;
    inode.i_gid = ptmx_gid;
    fsi.ptmx_inode = inode;
    d_make_persistent(dentry, inode);
    simple_done_creating(dentry);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn update_ptmx_mode(fsi: *mut pts_fs_info) {
    static void update_ptmx_mode(struct pts_fs_info *fsi)
    {
    fsi.ptmx_inode.i_mode = S_IFCHR|fsi.mount_opts.ptmxmode;
    }
#[no_mangle]
unsafe extern "C" fn devpts_reconfigure(fc: *mut fs_context) -> c_int {
    static int devpts_reconfigure(struct fs_context *fc)
    {
    struct pts_fs_info *fsi = DEVPTS_SB(fc.root.d_sb);
    struct pts_fs_info *new = fc.s_fs_info;
// Apply the revised options.  We don't want to change ->reserve.
// Ideally, we'd update each option conditionally on it having been
// explicitly changed, but the default is to reset everything so that
// would break UAPI...
//
    fsi.mount_opts.setuid		= new.mount_opts.setuid;
    fsi.mount_opts.setgid		= new.mount_opts.setgid;
    fsi.mount_opts.uid		= new.mount_opts.uid;
    fsi.mount_opts.gid		= new.mount_opts.gid;
    fsi.mount_opts.mode		= new.mount_opts.mode;
    fsi.mount_opts.ptmxmode	= new.mount_opts.ptmxmode;
    fsi.mount_opts.max		= new.mount_opts.max;
//
// parse_mount_options() restores options to default values
// before parsing and may have changed ptmxmode. So, update the
// mode in the inode too. Bogus options don't fail the remount,
// so do this even on error return.
//
    update_ptmx_mode(fsi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn devpts_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int {
    static int devpts_show_options(struct seq_file *seq, struct dentry *root)
    {
    struct pts_fs_info *fsi = DEVPTS_SB(root.d_sb);
    struct pts_mount_opts *opts = &fsi.mount_opts;
    if (opts.setuid)
    seq_printf(seq, ",uid=%u",
    from_kuid_munged(&init_user_ns, opts.uid));
    if (opts.setgid)
    seq_printf(seq, ",gid=%u",
    from_kgid_munged(&init_user_ns, opts.gid));
    seq_printf(seq, ",mode=%03o", opts.mode);
    seq_printf(seq, ",ptmxmode=%03o", opts.ptmxmode);
    if (opts.max < NR_UNIX98_PTY_MAX)
    seq_printf(seq, ",max=%d", opts.max);
    return 0;
    }
    static const struct super_operations devpts_sops = {
    .statfs		= simple_statfs,
    .show_options	= devpts_show_options,
    };
#[no_mangle]
unsafe extern "C" fn devpts_fill_super(s: *mut super_block, fc: *mut fs_context) -> c_int {
    static int devpts_fill_super(struct super_block *s, struct fs_context *fc)
    {
    struct pts_fs_info *fsi = DEVPTS_SB(s);
    struct inode *inode;
    s.s_iflags &= ~SB_I_NODEV;
    s.s_blocksize = 1024;
    s.s_blocksize_bits = 10;
    s.s_magic = DEVPTS_SUPER_MAGIC;
    s.s_op = &devpts_sops;
    s.s_d_flags = DCACHE_DONTCACHE;
    s.s_time_gran = 1;
    fsi.sb = s;
    inode = new_inode(s);
    if (!inode)
    return -ENOMEM;
    inode.i_ino = 1;
    simple_inode_init_ts(inode);
    inode.i_mode = S_IFDIR | S_IRUGO | S_IXUGO | S_IWUSR;
    inode.i_op = &simple_dir_inode_operations;
    inode.i_fop = &simple_dir_operations;
    set_nlink(inode, 2);
    s.s_root = d_make_root(inode);
    if (!s.s_root) {
    pr_err("get root dentry failed\n");
    return -ENOMEM;
    }
    return mknod_ptmx(s, fc);
    }
//
// devpts_get_tree()
//
// Mount a new (private) instance of devpts.  PTYs created in this
// instance are independent of the PTYs in other devpts instances.
//
#[no_mangle]
unsafe extern "C" fn devpts_get_tree(fc: *mut fs_context) -> c_int {
    static int devpts_get_tree(struct fs_context *fc)
    {
    return get_tree_nodev(fc, devpts_fill_super);
    }
#[no_mangle]
unsafe extern "C" fn devpts_free_fc(fc: *mut fs_context) {
    static void devpts_free_fc(struct fs_context *fc)
    {
    kfree(fc.s_fs_info);
    }
    static const struct fs_context_operations devpts_context_ops = {
    .free		= devpts_free_fc,
    .parse_param	= devpts_parse_param,
    .get_tree	= devpts_get_tree,
    .reconfigure	= devpts_reconfigure,
    };
//
// Set up the filesystem mount context.
//
#[no_mangle]
unsafe extern "C" fn devpts_init_fs_context(fc: *mut fs_context) -> c_int {
    static int devpts_init_fs_context(struct fs_context *fc)
    {
    struct pts_fs_info *fsi;
    fsi = kzalloc_obj(struct pts_fs_info);
    if (!fsi)
    return -ENOMEM;
    ida_init(&fsi.allocated_ptys);
    fsi.mount_opts.uid     = GLOBAL_ROOT_UID;
    fsi.mount_opts.gid     = GLOBAL_ROOT_GID;
    fsi.mount_opts.mode    = DEVPTS_DEFAULT_MODE;
    fsi.mount_opts.ptmxmode = DEVPTS_DEFAULT_PTMX_MODE;
    fsi.mount_opts.max     = NR_UNIX98_PTY_MAX;
    if (fc.purpose == FS_CONTEXT_FOR_MOUNT &&
    current.nsproxy.mnt_ns == init_task.nsproxy.mnt_ns)
    fsi.mount_opts.reserve = true;
    fc.s_fs_info = fsi;
    fc.ops = &devpts_context_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn devpts_kill_sb(sb: *mut super_block) {
    static void devpts_kill_sb(struct super_block *sb)
    {
    struct pts_fs_info *fsi = DEVPTS_SB(sb);
    if (fsi)
    ida_destroy(&fsi.allocated_ptys);
    kfree(fsi);
    kill_anon_super(sb);
    }
    static struct file_system_type devpts_fs_type = {
    .name		= "devpts",
    .init_fs_context = devpts_init_fs_context,
    .parameters	= devpts_param_specs,
    .kill_sb	= devpts_kill_sb,
    .fs_flags	= FS_USERNS_MOUNT,
    };
//
// The normal naming convention is simply /dev/pts/<number>; this conforms
// to the System V naming convention
//
#[no_mangle]
pub unsafe extern "C" fn devpts_new_index(fsi: *mut pts_fs_info) -> c_int {
    int devpts_new_index(struct pts_fs_info *fsi)
    {
    let mut index: c_int = -ENOSPC;
    if (atomic_inc_return(&pty_count) >= (pty_limit -
    (fsi.mount_opts.reserve ? 0 : pty_reserve)))
    goto out;
    index = ida_alloc_max(&fsi.allocated_ptys, fsi.mount_opts.max - 1,
    GFP_KERNEL);
    out:
    if (index < 0)
    atomic_dec(&pty_count);
    return index;
    }
#[no_mangle]
pub unsafe extern "C" fn devpts_kill_index(fsi: *mut pts_fs_info, idx: c_int) {
    void devpts_kill_index(struct pts_fs_info *fsi, int idx)
    {
    ida_free(&fsi.allocated_ptys, idx);
    atomic_dec(&pty_count);
    }
//
// devpts_pty_new -- create a new inode in /dev/pts
// @fsi: Filesystem info for this instance.
// @index: used as a name of the node
// @priv: what's given back by devpts_get_priv
//
// The dentry for the created inode is returned.
// Remove it from /dev/pts/ with devpts_pty_kill().
//
    struct dentry *devpts_pty_new(struct pts_fs_info *fsi, int index, void *priv)
    {
    struct dentry *dentry;
    struct super_block *sb = fsi.sb;
    struct inode *inode;
    struct dentry *root;
    struct pts_mount_opts *opts;
    char s[12];
    root = sb.s_root;
    opts = &fsi.mount_opts;
    inode = new_inode(sb);
    if (!inode)
    return ERR_PTR(-ENOMEM);
    inode.i_ino = index + 3;
    inode.i_uid = opts.setuid ? opts.uid : current_fsuid();
    inode.i_gid = opts.setgid ? opts.gid : current_fsgid();
    simple_inode_init_ts(inode);
    init_special_inode(inode, S_IFCHR|opts.mode, MKDEV(UNIX98_PTY_SLAVE_MAJOR, index));
    sprintf(s, "%d", index);
    dentry = d_alloc_name(root, s);
    if (!dentry) {
    iput(inode);
    return ERR_PTR(-ENOMEM);
    }
    dentry.d_fsdata = priv;
    d_make_persistent(dentry, inode);
    fsnotify_create(d_inode(root), dentry);
    dput(dentry);
    return dentry; // borrowed
    }
//
// devpts_get_priv -- get private data for a slave
// @dentry: dentry of the slave
//
// Returns whatever was passed as priv in devpts_pty_new for a given inode.
//
    void *devpts_get_priv(struct dentry *dentry)
    {
    if (dentry.d_sb.s_magic != DEVPTS_SUPER_MAGIC)
    return core::ptr::null_mut();
    return dentry.d_fsdata;
    }
//
// devpts_pty_kill -- remove inode form /dev/pts
// @dentry: dentry of the slave to be removed
//
// This is an inverse operation of devpts_pty_new.
//
#[no_mangle]
pub unsafe extern "C" fn devpts_pty_kill(dentry: *mut dentry) {
    void devpts_pty_kill(struct dentry *dentry)
    {
    WARN_ON_ONCE(dentry.d_sb.s_magic != DEVPTS_SUPER_MAGIC);
    dentry.d_fsdata = core::ptr::null_mut();
    drop_nlink(dentry.d_inode);
    d_drop(dentry);
    fsnotify_unlink(d_inode(dentry.d_parent), dentry);
    d_make_discardable(dentry);
    }
#[no_mangle]
unsafe extern "C" fn init_devpts_fs() -> int __init {
    static int __init init_devpts_fs(void)
    {
    let mut err: c_int = register_filesystem(&devpts_fs_type);
    if (!err) {
    register_sysctl("kernel/pty", pty_table);
    }
    return err;
    }
    module_init(init_devpts_fs)
