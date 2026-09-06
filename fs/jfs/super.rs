//! Automatically rewritten from C to Rust
//! Source: fs/jfs/super.c
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
// Copyright (C) International Business Machines Corp., 2000-2004
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

    MODULE_DESCRIPTION("The Journaled Filesystem (JFS)");
    MODULE_AUTHOR("Steve Best/Dave Kleikamp/Barry Arndt, IBM");
    MODULE_LICENSE("GPL");
    static struct kmem_cache *jfs_inode_cachep;
    static const struct super_operations jfs_super_operations;
    static const struct export_operations jfs_export_operations;
    static struct file_system_type jfs_fs_type;
pub const MAX_COMMIT_THREADS: c_int = 64;
    static int commit_threads;
    module_param(commit_threads, int, 0);
    MODULE_PARM_DESC(commit_threads, "Number of commit threads");
    static struct task_struct *jfsCommitThread[MAX_COMMIT_THREADS];
    struct task_struct *jfsIOthread;
    struct task_struct *jfsSyncThread;

    let mut jfsloglevel: c_int = JFS_LOGLEVEL_WARN;
    module_param(jfsloglevel, int, 0644);
    MODULE_PARM_DESC(jfsloglevel, "Specify JFS loglevel (0, 1 or 2)");

#[no_mangle]
unsafe extern "C" fn jfs_handle_error(sb: *mut super_block) {
    static void jfs_handle_error(struct super_block *sb)
    {
    struct jfs_sb_info *sbi = JFS_SBI(sb);
    if (sb_rdonly(sb))
    return;
    updateSuper(sb, FM_DIRTY);
    if (sbi.flag & JFS_ERR_PANIC)
    panic("JFS (device %s): panic forced after error\n",
    sb.s_id);
#[no_mangle]
pub unsafe extern "C" fn if(JFS_ERR_REMOUNT_RO: sbi->flag &) -> else {
    jfs_err("ERROR: (device %s): remounting filesystem as read-only",
    sb.s_id);
    sb.s_flags |= SB_RDONLY;
    }
// nothing is done for continue beyond marking the superblock dirty
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_error(sb: *mut super_block, fmt: *const c_char, ...) {
    void jfs_error(struct super_block *sb, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    pr_err("ERROR: (device %s): %ps: %pV\n",
    sb.s_id, __builtin_return_address(0), &vaf);
    va_end(args);
    jfs_handle_error(sb);
    }
    static struct inode *jfs_alloc_inode(struct super_block *sb)
    {
    struct jfs_inode_info *jfs_inode;
    jfs_inode = alloc_inode_sb(sb, jfs_inode_cachep, GFP_NOFS);
    if (!jfs_inode)
    return core::ptr::null_mut();

    memset(&jfs_inode.i_dquot, 0, sizeof(jfs_inode.i_dquot));

    return &jfs_inode.vfs_inode;
    }
#[no_mangle]
unsafe extern "C" fn jfs_free_inode(inode: *mut inode) {
    static void jfs_free_inode(struct inode *inode)
    {
    kmem_cache_free(jfs_inode_cachep, JFS_IP(inode));
    }
#[no_mangle]
unsafe extern "C" fn jfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    static int jfs_statfs(struct dentry *dentry, struct kstatfs *buf)
    {
    struct jfs_sb_info *sbi = JFS_SBI(dentry.d_sb);
    s64 maxinodes;
    struct inomap *imap = JFS_IP(sbi.ipimap).i_imap;
    jfs_info("In jfs_statfs");
    buf.f_type = JFS_SUPER_MAGIC;
    buf.f_bsize = sbi.bsize;
    buf.f_blocks = sbi.bmap.db_mapsize;
    buf.f_bfree = sbi.bmap.db_nfree;
    buf.f_bavail = sbi.bmap.db_nfree;
//
// If we really return the number of allocated & free inodes, some
// applications will fail because they won't see enough free inodes.
// We'll try to calculate some guess as to how many inodes we can
// really allocate
//
// buf->f_files = atomic_read(&imap->im_numinos);
// buf->f_ffree = atomic_read(&imap->im_numfree);
//
    maxinodes = min((s64) atomic_read(&imap.im_numinos) +
    ((sbi.bmap.db_nfree >> imap.im_l2nbperiext)
    << L2INOSPEREXT), (s64) 0xffffffffLL);
    buf.f_files = maxinodes;
    buf.f_ffree = maxinodes - (atomic_read(&imap.im_numinos) -
    atomic_read(&imap.im_numfree));
    buf.f_fsid.val[0] = crc32_le(0, (char *)&sbi.uuid,
    sizeof(sbi.uuid)/2);
    buf.f_fsid.val[1] = crc32_le(0,
    (char *)&sbi.uuid + sizeof(sbi.uuid)/2,
    sizeof(sbi.uuid)/2);
    buf.f_namelen = JFS_NAME_MAX;
    return 0;
    }

    static int jfs_quota_off(struct super_block *sb, int type);
    static int jfs_quota_on(struct super_block *sb, int type, int format_id,
    const struct path *path);
#[no_mangle]
unsafe extern "C" fn jfs_quota_off_umount(sb: *mut super_block) {
    static void jfs_quota_off_umount(struct super_block *sb)
    {
    int type;
    for (type = 0; type < MAXQUOTAS; type++)
    jfs_quota_off(sb, type);
    }
    static const struct quotactl_ops jfs_quotactl_ops = {
    .quota_on	= jfs_quota_on,
    .quota_off	= jfs_quota_off,
    .quota_sync	= dquot_quota_sync,
    .get_state	= dquot_get_state,
    .set_info	= dquot_set_dqinfo,
    .get_dqblk	= dquot_get_dqblk,
    .set_dqblk	= dquot_set_dqblk,
    .get_nextdqblk	= dquot_get_next_dqblk,
    };

#[no_mangle]
pub unsafe extern "C" fn jfs_quota_off_umount(sb: *mut super_block) {
    static inline void jfs_quota_off_umount(struct super_block *sb)
    {
    }

#[no_mangle]
unsafe extern "C" fn jfs_put_super(sb: *mut super_block) {
    static void jfs_put_super(struct super_block *sb)
    {
    struct jfs_sb_info *sbi = JFS_SBI(sb);
    int rc;
    jfs_info("In jfs_put_super");
    jfs_quota_off_umount(sb);
    rc = jfs_umount(sb);
    if (rc)
    jfs_err("jfs_umount failed with return code %d", rc);
    unload_nls(sbi.nls_tab);
    truncate_inode_pages(sbi.direct_inode.i_mapping, 0);
    iput(sbi.direct_inode);
    kfree(sbi);
    }
    enum {
    Opt_integrity, Opt_nointegrity, Opt_iocharset, Opt_resize,
    Opt_resize_nosize, Opt_errors, Opt_ignore, Opt_err, Opt_quota,
    Opt_usrquota, Opt_grpquota, Opt_uid, Opt_gid, Opt_umask,
    Opt_discard, Opt_nodiscard, Opt_discard_minblk
    };
    static const struct constant_table jfs_param_errors[] = {
    {"continue",	JFS_ERR_CONTINUE},
    {"remount-ro",	JFS_ERR_REMOUNT_RO},
    {"panic",	JFS_ERR_PANIC},
    {}
    };
    static const struct fs_parameter_spec jfs_param_spec[] = {
    fsparam_flag_no	("integrity",	Opt_integrity),
    fsparam_string	("iocharset",	Opt_iocharset),
    fsparam_u64	("resize",	Opt_resize),
    fsparam_flag	("resize",	Opt_resize_nosize),
    fsparam_enum	("errors",	Opt_errors,	jfs_param_errors),
    fsparam_flag	("quota",	Opt_quota),
    fsparam_flag	("noquota",	Opt_ignore),
    fsparam_flag	("usrquota",	Opt_usrquota),
    fsparam_flag	("grpquota",	Opt_grpquota),
    fsparam_uid	("uid",		Opt_uid),
    fsparam_gid	("gid",		Opt_gid),
    fsparam_u32oct	("umask",	Opt_umask),
    fsparam_flag	("discard",	Opt_discard),
    fsparam_u32	("discard",	Opt_discard_minblk),
    fsparam_flag	("nodiscard",	Opt_nodiscard),
    {}
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jfs_context {
    pub flag: c_int,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub umask: c_uint,
    pub minblks_trim: c_uint,
    pub nls_map: *mut c_void,
    pub resize: bool,
    pub newLVSize: i64,
}

#[no_mangle]
unsafe extern "C" fn jfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    static int jfs_parse_param(struct fs_context *fc, struct fs_parameter *param)
    {
    struct jfs_context *ctx = fc.fs_private;
    let mut reconfigure: c_int = (fc.purpose == FS_CONTEXT_FOR_RECONFIGURE);
    struct fs_parse_result result;
    struct nls_table *nls_map;
    int opt;
    opt = fs_parse(fc, jfs_param_spec, param, &result);
    if (opt < 0)
    return opt;
    switch (opt) {
    case Opt_integrity:
    if (result.negated)
    ctx.flag |= JFS_NOINTEGRITY;
    else
    ctx.flag &= ~JFS_NOINTEGRITY;
    break;
    case Opt_ignore:
// Silently ignore the quota options
// Don't do anything ;-)
    break;
    case Opt_iocharset:
    if (ctx.nls_map && ctx.nls_map != (void *) -1) {
    unload_nls(ctx.nls_map);
    ctx.nls_map = core::ptr::null_mut();
    }
    if (!strcmp(param.string, "none"))
    ctx.nls_map = core::ptr::null_mut();
    else {
    nls_map = load_nls(param.string);
    if (!nls_map) {
    pr_err("JFS: charset not found\n");
    return -EINVAL;
    }
    ctx.nls_map = nls_map;
    }
    break;
    case Opt_resize:
    if (!reconfigure)
    return -EINVAL;
    ctx.resize = true;
    ctx.newLVSize = result.uint_64;
    break;
    case Opt_resize_nosize:
    if (!reconfigure)
    return -EINVAL;
    ctx.resize = true;
    break;
    case Opt_errors:
    ctx.flag &= ~JFS_ERR_MASK;
    ctx.flag |= result.uint_32;
    break;

    case Opt_quota:
    case Opt_usrquota:
    ctx.flag |= JFS_USRQUOTA;
    break;
    case Opt_grpquota:
    ctx.flag |= JFS_GRPQUOTA;
    break;

    case Opt_usrquota:
    case Opt_grpquota:
    case Opt_quota:
    pr_err("JFS: quota operations not supported\n");
    break;

    case Opt_uid:
    ctx.uid = result.uid;
    break;
    case Opt_gid:
    ctx.gid = result.gid;
    break;
    case Opt_umask:
    if (result.uint_32 & ~0777) {
    pr_err("JFS: Invalid value of umask\n");
    return -EINVAL;
    }
    ctx.umask = result.uint_32;
    break;
    case Opt_discard:
// if set to 1, even copying files will cause
// trimming :O
// -> user has more control over the online trimming
//
    ctx.minblks_trim = 64;
    ctx.flag |= JFS_DISCARD;
    break;
    case Opt_nodiscard:
    ctx.flag &= ~JFS_DISCARD;
    break;
    case Opt_discard_minblk:
    ctx.minblks_trim = result.uint_32;
    ctx.flag |= JFS_DISCARD;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_reconfigure(fc: *mut fs_context) -> c_int {
    static int jfs_reconfigure(struct fs_context *fc)
    {
    struct jfs_context *ctx = fc.fs_private;
    struct super_block *sb = fc.root.d_sb;
    let mut readonly: c_int = fc.sb_flags & SB_RDONLY;
    let mut rc: c_int = 0;
    let mut flag: c_int = ctx.flag;
    int ret;
    sync_filesystem(sb);
// Transfer results of parsing to the sbi
    JFS_SBI(sb).flag = ctx.flag;
    JFS_SBI(sb).uid = ctx.uid;
    JFS_SBI(sb).gid = ctx.gid;
    JFS_SBI(sb).umask = ctx.umask;
    JFS_SBI(sb).minblks_trim = ctx.minblks_trim;
    if (ctx.nls_map != (void *) -1) {
    unload_nls(JFS_SBI(sb).nls_tab);
    JFS_SBI(sb).nls_tab = ctx.nls_map;
    }
    ctx.nls_map = core::ptr::null_mut();
    if (ctx.resize) {
    if (sb_rdonly(sb)) {
    pr_err("JFS: resize requires volume to be mounted read-write\n");
    return -EROFS;
    }
    if (!ctx.newLVSize) {
    ctx.newLVSize = sb_bdev_nr_blocks(sb);
    if (ctx.newLVSize == 0)
    pr_err("JFS: Cannot determine volume size\n");
    }
    rc = jfs_extendfs(sb, ctx.newLVSize, 0);
    if (rc)
    return rc;
    }
    if (sb_rdonly(sb) && !readonly) {
//
// Invalidate any previously read metadata.  fsck may have
// changed the on-disk data since we mounted r/o
//
    truncate_inode_pages(JFS_SBI(sb).direct_inode.i_mapping, 0);
    JFS_SBI(sb).flag = flag;
    ret = jfs_mount_rw(sb, 1);
// mark the fs r/w for quota activity
    sb.s_flags &= ~SB_RDONLY;
    dquot_resume(sb, -1);
    return ret;
    }
    if (!sb_rdonly(sb) && readonly) {
    rc = dquot_suspend(sb, -1);
    if (rc < 0)
    return rc;
    rc = jfs_umount_rw(sb);
    JFS_SBI(sb).flag = flag;
    return rc;
    }
    if ((JFS_SBI(sb).flag & JFS_NOINTEGRITY) != (flag & JFS_NOINTEGRITY)) {
    if (!sb_rdonly(sb)) {
    rc = jfs_umount_rw(sb);
    if (rc)
    return rc;
    JFS_SBI(sb).flag = flag;
    ret = jfs_mount_rw(sb, 1);
    return ret;
    }
    }
    JFS_SBI(sb).flag = flag;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    static int jfs_fill_super(struct super_block *sb, struct fs_context *fc)
    {
    struct jfs_context *ctx = fc.fs_private;
    let mut silent: c_int = fc.sb_flags & SB_SILENT;
    struct jfs_sb_info *sbi;
    struct inode *inode;
    int rc;
    let mut ret: c_int = -EINVAL;
    jfs_info("In jfs_read_super: s_flags=0x%lx", sb.s_flags);
    sbi = kzalloc_obj(struct jfs_sb_info);
    if (!sbi)
    return -ENOMEM;
    sb.s_fs_info = sbi;
    sb.s_max_links = JFS_LINK_MAX;
    sb.s_time_min = 0;
    sb.s_time_max = U32_MAX;
    sbi.sb = sb;
// Transfer results of parsing to the sbi
    sbi.flag = ctx.flag;
    sbi.uid = ctx.uid;
    sbi.gid = ctx.gid;
    sbi.umask = ctx.umask;
    if (ctx.nls_map != (void *) -1) {
    unload_nls(sbi.nls_tab);
    sbi.nls_tab = ctx.nls_map;
    }
    ctx.nls_map = core::ptr::null_mut();
    if (sbi.flag & JFS_DISCARD) {
    if (!bdev_max_discard_sectors(sb.s_bdev)) {
    pr_err("JFS: discard option not supported on device\n");
    sbi.flag &= ~JFS_DISCARD;
    } else {
    sbi.minblks_trim = ctx.minblks_trim;
    }
    }

    sb.s_flags |= SB_POSIXACL;

    if (ctx.resize) {
    pr_err("resize option for remount only\n");
    goto out_unload;
    }
//
// Initialize blocksize to 4K.
//
    if (!sb_set_blocksize(sb, PSIZE))
    goto out_unload;
//
// Set method vectors.
//
    sb.s_op = &jfs_super_operations;
    sb.s_export_op = &jfs_export_operations;
    sb.s_xattr = jfs_xattr_handlers;

    sb.dq_op = &dquot_operations;
    sb.s_qcop = &jfs_quotactl_ops;
    sb.s_quota_types = QTYPE_MASK_USR | QTYPE_MASK_GRP;

//
// Initialize direct-mapping inode/address-space
//
    inode = new_inode(sb);
    if (inode == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto out_unload;
    }
    inode.i_size = bdev_nr_bytes(sb.s_bdev);
    inode.i_mapping.a_ops = &jfs_metapage_aops;
    inode_fake_hash(inode);
    mapping_set_gfp_mask(inode.i_mapping, GFP_NOFS);
    sbi.direct_inode = inode;
    rc = jfs_mount(sb);
    if (rc) {
    if (!silent)
    jfs_err("jfs_mount failed w/return code = %d", rc);
    goto out_mount_failed;
    }
    if (sb_rdonly(sb))
    sbi.log = core::ptr::null_mut();
    else {
    rc = jfs_mount_rw(sb, 0);
    if (rc) {
    if (!silent) {
    jfs_err("jfs_mount_rw failed, return code = %d",
    rc);
    }
    goto out_no_rw;
    }
    }
    sb.s_magic = JFS_SUPER_MAGIC;
    if (sbi.mntflag & JFS_OS2)
    set_default_d_op(sb, &jfs_ci_dentry_operations);
    inode = jfs_iget(sb, ROOT_I);
    if (IS_ERR(inode)) {
    ret = PTR_ERR(inode);
    goto out_no_rw;
    }
    sb.s_root = d_make_root(inode);
    if (!sb.s_root)
    goto out_no_root;
// logical blocks are represented by 40 bits in pxd_t, etc.
// and page cache is indexed by long
//
    sb.s_maxbytes = min(((loff_t)sb.s_blocksize) << 40, MAX_LFS_FILESIZE);
    sb.s_time_gran = 1;
    return 0;
    out_no_root:
    jfs_err("jfs_read_super: get root dentry failed");
    out_no_rw:
    rc = jfs_umount(sb);
    if (rc)
    jfs_err("jfs_umount failed with return code %d", rc);
    out_mount_failed:
    filemap_write_and_wait(sbi.direct_inode.i_mapping);
    truncate_inode_pages(sbi.direct_inode.i_mapping, 0);
    make_bad_inode(sbi.direct_inode);
    iput(sbi.direct_inode);
    sbi.direct_inode = core::ptr::null_mut();
    out_unload:
    unload_nls(sbi.nls_tab);
    kfree(sbi);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jfs_freeze(sb: *mut super_block) -> c_int {
    static int jfs_freeze(struct super_block *sb)
    {
    struct jfs_sb_info *sbi = JFS_SBI(sb);
    struct jfs_log *log = sbi.log;
    let mut rc: c_int = 0;
    if (!sb_rdonly(sb)) {
    txQuiesce(sb);
    rc = lmLogShutdown(log);
    if (rc) {
    jfs_error(sb, "lmLogShutdown failed\n");
// let operations fail rather than hang
    txResume(sb);
    return rc;
    }
    rc = updateSuper(sb, FM_CLEAN);
    if (rc) {
    jfs_err("jfs_freeze: updateSuper failed");
//
// Don't fail here. Everything succeeded except
// marking the superblock clean, so there's really
// no harm in leaving it frozen for now.
//
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_unfreeze(sb: *mut super_block) -> c_int {
    static int jfs_unfreeze(struct super_block *sb)
    {
    struct jfs_sb_info *sbi = JFS_SBI(sb);
    struct jfs_log *log = sbi.log;
    let mut rc: c_int = 0;
    if (!sb_rdonly(sb)) {
    rc = updateSuper(sb, FM_MOUNT);
    if (rc) {
    jfs_error(sb, "updateSuper failed\n");
    goto out;
    }
    rc = lmLogInit(log);
    if (rc)
    jfs_error(sb, "lmLogInit failed\n");
    out:
    txResume(sb);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn jfs_get_tree(fc: *mut fs_context) -> c_int {
    static int jfs_get_tree(struct fs_context *fc)
    {
    return get_tree_bdev(fc, jfs_fill_super);
    }
#[no_mangle]
unsafe extern "C" fn jfs_sync_fs(sb: *mut super_block, wait: c_int) -> c_int {
    static int jfs_sync_fs(struct super_block *sb, int wait)
    {
    struct jfs_log *log = JFS_SBI(sb).log;
// log == NULL indicates read-only mount
    if (log) {
//
// Write quota structures to quota file, sync_blockdev() will
// write them to disk later
//
    dquot_writeback_dquots(sb, -1);
    jfs_flush_journal(log, wait);
    jfs_syncpt(log, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int {
    static int jfs_show_options(struct seq_file *seq, struct dentry *root)
    {
    struct jfs_sb_info *sbi = JFS_SBI(root.d_sb);
    if (uid_valid(sbi.uid))
    seq_printf(seq, ",uid=%d", from_kuid(&init_user_ns, sbi.uid));
    if (gid_valid(sbi.gid))
    seq_printf(seq, ",gid=%d", from_kgid(&init_user_ns, sbi.gid));
    if (sbi.umask != -1)
    seq_printf(seq, ",umask=%03o", sbi.umask);
    if (sbi.flag & JFS_NOINTEGRITY)
    seq_puts(seq, ",nointegrity");
    if (sbi.flag & JFS_DISCARD)
    seq_printf(seq, ",discard=%u", sbi.minblks_trim);
    if (sbi.nls_tab)
    seq_printf(seq, ",iocharset=%s", sbi.nls_tab.charset);
    if (sbi.flag & JFS_ERR_CONTINUE)
    seq_printf(seq, ",errors=continue");
    if (sbi.flag & JFS_ERR_PANIC)
    seq_printf(seq, ",errors=panic");

    if (sbi.flag & JFS_USRQUOTA)
    seq_puts(seq, ",usrquota");
    if (sbi.flag & JFS_GRPQUOTA)
    seq_puts(seq, ",grpquota");

    return 0;
    }

// Read data from quotafile - avoid pagecache and such because we cannot afford
// acquiring the locks... As quota files are never truncated and quota code
// itself serializes the operations (and no one else should touch the files)
// we don't have to be afraid of races
    static ssize_t jfs_quota_read(struct super_block *sb, int type, char *data,
    size_t len, loff_t off)
    {
    struct inode *inode = sb_dqopt(sb).files[type];
    let mut blk: sector_t = off >> sb.s_blocksize_bits;
    let mut err: c_int = 0;
    let mut offset: c_int = off & (sb.s_blocksize - 1);
    int tocopy;
    size_t toread;
    struct buffer_head tmp_bh;
    struct buffer_head *bh;
    let mut i_size: loff_t = i_size_read(inode);
    if (off > i_size)
    return 0;
    if (off+len > i_size)
    len = i_size-off;
    toread = len;
    while (toread > 0) {
    tocopy = min_t(size_t, sb.s_blocksize - offset, toread);
    tmp_bh.b_state = 0;
    tmp_bh.b_size = i_blocksize(inode);
    err = jfs_get_block(inode, blk, &tmp_bh, 0);
    if (err)
    return err;
    if (!buffer_mapped(&tmp_bh))	/* A hole? */
    memset(data, 0, tocopy);
    else {
    bh = sb_bread(sb, tmp_bh.b_blocknr);
    if (!bh)
    return -EIO;
    memcpy(data, bh.b_data+offset, tocopy);
    brelse(bh);
    }
    offset = 0;
    toread -= tocopy;
    data += tocopy;
    blk++;
    }
    return len;
    }
// Write to quotafile
    static ssize_t jfs_quota_write(struct super_block *sb, int type,
    const char *data, size_t len, loff_t off)
    {
    struct inode *inode = sb_dqopt(sb).files[type];
    let mut blk: sector_t = off >> sb.s_blocksize_bits;
    let mut err: c_int = 0;
    let mut offset: c_int = off & (sb.s_blocksize - 1);
    int tocopy;
    let mut towrite: usize = len;
    struct buffer_head tmp_bh;
    struct buffer_head *bh;
    inode_lock(inode);
    while (towrite > 0) {
    tocopy = min_t(size_t, sb.s_blocksize - offset, towrite);
    tmp_bh.b_state = 0;
    tmp_bh.b_size = i_blocksize(inode);
    err = jfs_get_block(inode, blk, &tmp_bh, 1);
    if (err)
    goto out;
    if (offset || tocopy != sb.s_blocksize)
    bh = sb_bread(sb, tmp_bh.b_blocknr);
    else
    bh = sb_getblk(sb, tmp_bh.b_blocknr);
    if (!bh) {
    err = -EIO;
    goto out;
    }
    lock_buffer(bh);
    memcpy(bh.b_data+offset, data, tocopy);
    flush_dcache_folio(bh.b_folio);
    set_buffer_uptodate(bh);
    mark_buffer_dirty(bh);
    unlock_buffer(bh);
    brelse(bh);
    offset = 0;
    towrite -= tocopy;
    data += tocopy;
    blk++;
    }
    out:
    if (len == towrite) {
    inode_unlock(inode);
    return err;
    }
    if (inode.i_size < off+len-towrite)
    i_size_write(inode, off+len-towrite);
    inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    mark_inode_dirty(inode);
    inode_unlock(inode);
    return len - towrite;
    }
    static struct dquot __rcu **jfs_get_dquots(struct inode *inode)
    {
    return JFS_IP(inode).i_dquot;
    }
    static int jfs_quota_on(struct super_block *sb, int type, int format_id,
    const struct path *path)
    {
    int err;
    struct inode *inode;
    err = dquot_quota_on(sb, type, format_id, path);
    if (err)
    return err;
    inode = d_inode(path.dentry);
    inode_lock(inode);
    JFS_IP(inode).mode2 |= JFS_NOATIME_FL | JFS_IMMUTABLE_FL;
    inode_set_flags(inode, S_NOATIME | S_IMMUTABLE,
    S_NOATIME | S_IMMUTABLE);
    inode_unlock(inode);
    mark_inode_dirty(inode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_quota_off(sb: *mut super_block, type: c_int) -> c_int {
    static int jfs_quota_off(struct super_block *sb, int type)
    {
    struct inode *inode = sb_dqopt(sb).files[type];
    int err;
    if (!inode || !igrab(inode))
    goto out;
    err = dquot_quota_off(sb, type);
    if (err)
    goto out_put;
    inode_lock(inode);
    JFS_IP(inode).mode2 &= ~(JFS_NOATIME_FL | JFS_IMMUTABLE_FL);
    inode_set_flags(inode, 0, S_NOATIME | S_IMMUTABLE);
    inode_unlock(inode);
    mark_inode_dirty(inode);
    out_put:
    iput(inode);
    return err;
    out:
    return dquot_quota_off(sb, type);
    }

    static const struct super_operations jfs_super_operations = {
    .alloc_inode	= jfs_alloc_inode,
    .free_inode	= jfs_free_inode,
    .dirty_inode	= jfs_dirty_inode,
    .write_inode	= jfs_write_inode,
    .evict_inode	= jfs_evict_inode,
    .put_super	= jfs_put_super,
    .sync_fs	= jfs_sync_fs,
    .freeze_fs	= jfs_freeze,
    .unfreeze_fs	= jfs_unfreeze,
    .statfs		= jfs_statfs,
    .show_options	= jfs_show_options,

    .quota_read	= jfs_quota_read,
    .quota_write	= jfs_quota_write,
    .get_dquots	= jfs_get_dquots,

    };
    static const struct export_operations jfs_export_operations = {
    .encode_fh	= generic_encode_ino32_fh,
    .fh_to_dentry	= jfs_fh_to_dentry,
    .fh_to_parent	= jfs_fh_to_parent,
    .get_parent	= jfs_get_parent,
    };
#[no_mangle]
unsafe extern "C" fn jfs_init_options(fc: *mut fs_context, ctx: *mut jfs_context) {
    static void jfs_init_options(struct fs_context *fc, struct jfs_context *ctx)
    {
    if (fc.purpose == FS_CONTEXT_FOR_RECONFIGURE) {
    struct super_block *sb = fc.root.d_sb;
// Copy over current option values and mount flags
    ctx.uid = JFS_SBI(sb).uid;
    ctx.gid = JFS_SBI(sb).gid;
    ctx.umask = JFS_SBI(sb).umask;
    ctx.nls_map = (void *)-1;
    ctx.minblks_trim = JFS_SBI(sb).minblks_trim;
    ctx.flag = JFS_SBI(sb).flag;
    } else {
//
// Initialize the mount flag and determine the default
// error handler
//
    ctx.flag = JFS_ERR_REMOUNT_RO;
    ctx.uid = INVALID_UID;
    ctx.gid = INVALID_GID;
    ctx.umask = -1;
    ctx.nls_map = (void *)-1;
    }
    }
#[no_mangle]
unsafe extern "C" fn jfs_free_fc(fc: *mut fs_context) {
    static void jfs_free_fc(struct fs_context *fc)
    {
    struct jfs_context *ctx = fc.fs_private;
    if (ctx.nls_map != (void *) -1)
    unload_nls(ctx.nls_map);
    kfree(ctx);
    }
    static const struct fs_context_operations jfs_context_ops = {
    .parse_param	= jfs_parse_param,
    .get_tree	= jfs_get_tree,
    .reconfigure	= jfs_reconfigure,
    .free		= jfs_free_fc,
    };
#[no_mangle]
unsafe extern "C" fn jfs_init_fs_context(fc: *mut fs_context) -> c_int {
    static int jfs_init_fs_context(struct fs_context *fc)
    {
    struct jfs_context *ctx;
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    return -ENOMEM;
    jfs_init_options(fc, ctx);
    fc.fs_private = ctx;
    fc.ops = &jfs_context_ops;
    return 0;
    }
    static struct file_system_type jfs_fs_type = {
    .owner		= THIS_MODULE,
    .name		= "jfs",
    .kill_sb	= kill_block_super,
    .fs_flags	= FS_REQUIRES_DEV,
    .init_fs_context = jfs_init_fs_context,
    .parameters	= jfs_param_spec,
    };
    MODULE_ALIAS_FS("jfs");
#[no_mangle]
unsafe extern "C" fn init_once(foo: *mut c_void) {
    static void init_once(void *foo)
    {
    struct jfs_inode_info *jfs_ip = (struct jfs_inode_info *) foo;
    memset(jfs_ip, 0, sizeof(struct jfs_inode_info));
    INIT_LIST_HEAD(&jfs_ip.anon_inode_list);
    init_rwsem(&jfs_ip.rdwrlock);
    mutex_init(&jfs_ip.commit_mutex);
    init_rwsem(&jfs_ip.xattr_sem);
    spin_lock_init(&jfs_ip.ag_lock);
    jfs_ip.active_ag = -1;
    inode_init_once(&jfs_ip.vfs_inode);
    }
#[no_mangle]
unsafe extern "C" fn init_jfs_fs() -> int __init {
    static int __init init_jfs_fs(void)
    {
    int i;
    int rc;
    jfs_inode_cachep =
    kmem_cache_create_usercopy("jfs_ip", sizeof(struct jfs_inode_info),
    0, SLAB_RECLAIM_ACCOUNT|SLAB_ACCOUNT,
    offsetof(struct jfs_inode_info, i_inline_all),
    sizeof_field(struct jfs_inode_info, i_inline_all),
    init_once);
    if (jfs_inode_cachep == core::ptr::null_mut())
    return -ENOMEM;
//
// Metapage initialization
//
    rc = metapage_init();
    if (rc) {
    jfs_err("metapage_init failed w/rc = %d", rc);
    goto free_slab;
    }
//
// Transaction Manager initialization
//
    rc = txInit();
    if (rc) {
    jfs_err("txInit failed w/rc = %d", rc);
    goto free_metapage;
    }
//
// I/O completion thread (endio)
//
    jfsIOthread = kthread_run(jfsIOWait, core::ptr::null_mut(), "jfsIO");
    if (IS_ERR(jfsIOthread)) {
    rc = PTR_ERR(jfsIOthread);
    jfs_err("init_jfs_fs: fork failed w/rc = %d", rc);
    goto end_txmngr;
    }
    if (commit_threads < 1)
    commit_threads = num_online_cpus();
    if (commit_threads > MAX_COMMIT_THREADS)
    commit_threads = MAX_COMMIT_THREADS;
    for (i = 0; i < commit_threads; i++) {
    jfsCommitThread[i] = kthread_run(jfs_lazycommit, core::ptr::null_mut(),
    "jfsCommit");
    if (IS_ERR(jfsCommitThread[i])) {
    rc = PTR_ERR(jfsCommitThread[i]);
    jfs_err("init_jfs_fs: fork failed w/rc = %d", rc);
    commit_threads = i;
    goto kill_committask;
    }
    }
    jfsSyncThread = kthread_run(jfs_sync, core::ptr::null_mut(), "jfsSync");
    if (IS_ERR(jfsSyncThread)) {
    rc = PTR_ERR(jfsSyncThread);
    jfs_err("init_jfs_fs: fork failed w/rc = %d", rc);
    goto kill_committask;
    }

    jfs_proc_init();

    rc = register_filesystem(&jfs_fs_type);
    if (!rc)
    return 0;

    jfs_proc_clean();

    kthread_stop(jfsSyncThread);
    kill_committask:
    for (i = 0; i < commit_threads; i++)
    kthread_stop(jfsCommitThread[i]);
    kthread_stop(jfsIOthread);
    end_txmngr:
    txExit();
    free_metapage:
    metapage_exit();
    free_slab:
    kmem_cache_destroy(jfs_inode_cachep);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn exit_jfs_fs() -> void __exit {
    static void __exit exit_jfs_fs(void)
    {
    int i;
    jfs_info("exit_jfs_fs called");
    txExit();
    metapage_exit();
    kthread_stop(jfsIOthread);
    for (i = 0; i < commit_threads; i++)
    kthread_stop(jfsCommitThread[i]);
    kthread_stop(jfsSyncThread);

    jfs_proc_clean();

    unregister_filesystem(&jfs_fs_type);
//
// Make sure all delayed rcu free inodes are flushed before we
// destroy cache.
//
    rcu_barrier();
    kmem_cache_destroy(jfs_inode_cachep);
    }
    module_init(init_jfs_fs)
    module_exit(exit_jfs_fs)
