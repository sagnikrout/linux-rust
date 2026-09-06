//! Automatically rewritten from C to Rust
//! Source: fs/quota/quota_v2.c
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
// vfsv0 quota IO operations on file
//

    MODULE_AUTHOR("Jan Kara");
    MODULE_DESCRIPTION("Quota format v2 support");
    MODULE_LICENSE("GPL");
    static void v2r0_mem2diskdqb(void *dp, struct dquot *dquot);
    static void v2r0_disk2memdqb(struct dquot *dquot, void *dp);
    static int v2r0_is_id(void *dp, struct dquot *dquot);
    static void v2r1_mem2diskdqb(void *dp, struct dquot *dquot);
    static void v2r1_disk2memdqb(struct dquot *dquot, void *dp);
    static int v2r1_is_id(void *dp, struct dquot *dquot);
    static const struct qtree_fmt_operations v2r0_qtree_ops = {
    .mem2disk_dqblk = v2r0_mem2diskdqb,
    .disk2mem_dqblk = v2r0_disk2memdqb,
    .is_id = v2r0_is_id,
    };
    static const struct qtree_fmt_operations v2r1_qtree_ops = {
    .mem2disk_dqblk = v2r1_mem2diskdqb,
    .disk2mem_dqblk = v2r1_disk2memdqb,
    .is_id = v2r1_is_id,
    };
pub const QUOTABLOCK_BITS: c_int = 10;

#[no_mangle]
pub unsafe extern "C" fn v2_stoqb(space: qsize_t) -> qsize_t {
    static inline qsize_t v2_stoqb(qsize_t space)
    {
    return (space + QUOTABLOCK_SIZE - 1) >> QUOTABLOCK_BITS;
    }
#[no_mangle]
pub unsafe extern "C" fn v2_qbtos(blocks: qsize_t) -> qsize_t {
    static inline qsize_t v2_qbtos(qsize_t blocks)
    {
    return blocks << QUOTABLOCK_BITS;
    }
    static int v2_read_header(struct super_block *sb, int type,
    struct v2_disk_dqheader *dqhead)
    {
    ssize_t size;
    size = sb.s_op.quota_read(sb, type, (char *)dqhead,
    sizeof(struct v2_disk_dqheader), 0);
    if (size != sizeof(struct v2_disk_dqheader)) {
    quota_error(sb, "Failed header read: expected=%zd got=%zd",
    sizeof(struct v2_disk_dqheader), size);
    if (size < 0)
    return size;
    return -EIO;
    }
    return 0;
    }
// Check whether given file is really vfsv0 quotafile
#[no_mangle]
unsafe extern "C" fn v2_check_quota_file(sb: *mut super_block, type: c_int) -> c_int {
    static int v2_check_quota_file(struct super_block *sb, int type)
    {
    struct v2_disk_dqheader dqhead;
    static const uint quota_magics[] = V2_INITQMAGICS;
    static const uint quota_versions[] = V2_INITQVERSIONS;
    if (v2_read_header(sb, type, &dqhead))
    return 0;
    if (le32_to_cpu(dqhead.dqh_magic) != quota_magics[type] ||
    le32_to_cpu(dqhead.dqh_version) > quota_versions[type])
    return 0;
    return 1;
    }
// Read information header from quota file
#[no_mangle]
unsafe extern "C" fn v2_read_file_info(sb: *mut super_block, type: c_int) -> c_int {
    static int v2_read_file_info(struct super_block *sb, int type)
    {
    struct v2_disk_dqinfo dinfo;
    struct v2_disk_dqheader dqhead;
    struct quota_info *dqopt = sb_dqopt(sb);
    struct mem_dqinfo *info = &dqopt.info[type];
    struct qtree_mem_dqinfo *qinfo;
    ssize_t size;
    unsigned int version;
    unsigned int memalloc;
    int ret;
    down_read(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    ret = v2_read_header(sb, type, &dqhead);
    if (ret < 0)
    goto out;
    version = le32_to_cpu(dqhead.dqh_version);
    if ((info.dqi_fmt_id == QFMT_VFS_V0 && version != 0) ||
    (info.dqi_fmt_id == QFMT_VFS_V1 && version != 1)) {
    ret = -EINVAL;
    goto out;
    }
    size = sb.s_op.quota_read(sb, type, (char *)&dinfo,
    sizeof(struct v2_disk_dqinfo), V2_DQINFOOFF);
    if (size != sizeof(struct v2_disk_dqinfo)) {
    quota_error(sb, "Can't read info structure");
    if (size < 0)
    ret = size;
    else
    ret = -EIO;
    goto out;
    }
    info.dqi_priv = kmalloc_obj(struct qtree_mem_dqinfo);
    if (!info.dqi_priv) {
    ret = -ENOMEM;
    goto out;
    }
    qinfo = info.dqi_priv;
    if (version == 0) {
// limits are stored as unsigned 32-bit data
    info.dqi_max_spc_limit = 0xffffffffLL << QUOTABLOCK_BITS;
    info.dqi_max_ino_limit = 0xffffffff;
    } else {
//
// Used space is stored as unsigned 64-bit value in bytes but
// quota core supports only signed 64-bit values so use that
// as a limit
//
    info.dqi_max_spc_limit = 0x7fffffffffffffffLL; /* 2^63-1 */
    info.dqi_max_ino_limit = 0x7fffffffffffffffLL;
    }
    info.dqi_bgrace = le32_to_cpu(dinfo.dqi_bgrace);
    info.dqi_igrace = le32_to_cpu(dinfo.dqi_igrace);
// No flags currently supported
    info.dqi_flags = 0;
    qinfo.dqi_sb = sb;
    qinfo.dqi_type = type;
    qinfo.dqi_blocks = le32_to_cpu(dinfo.dqi_blocks);
    qinfo.dqi_free_blk = le32_to_cpu(dinfo.dqi_free_blk);
    qinfo.dqi_free_entry = le32_to_cpu(dinfo.dqi_free_entry);
    qinfo.dqi_blocksize_bits = V2_DQBLKSIZE_BITS;
    qinfo.dqi_usable_bs = 1 << V2_DQBLKSIZE_BITS;
    qinfo.dqi_qtree_depth = qtree_depth(qinfo);
    if (version == 0) {
    qinfo.dqi_entry_size = sizeof(struct v2r0_disk_dqblk);
    qinfo.dqi_ops = &v2r0_qtree_ops;
    } else {
    qinfo.dqi_entry_size = sizeof(struct v2r1_disk_dqblk);
    qinfo.dqi_ops = &v2r1_qtree_ops;
    }
    ret = -EUCLEAN;
// Some sanity checks of the read headers...
    if ((loff_t)qinfo.dqi_blocks << qinfo.dqi_blocksize_bits >
    i_size_read(sb_dqopt(sb).files[type])) {
    quota_error(sb, "Number of blocks too big for quota file size (%llu > %llu).",
    (loff_t)qinfo.dqi_blocks << qinfo.dqi_blocksize_bits,
    i_size_read(sb_dqopt(sb).files[type]));
    goto out_free;
    }
    if (qinfo.dqi_free_blk && (qinfo.dqi_free_blk <= QT_TREEOFF ||
    qinfo.dqi_free_blk >= qinfo.dqi_blocks)) {
    quota_error(sb, "Free block number %u out of range (%u, %u).",
    qinfo.dqi_free_blk, QT_TREEOFF, qinfo.dqi_blocks);
    goto out_free;
    }
    if (qinfo.dqi_free_entry && (qinfo.dqi_free_entry <= QT_TREEOFF ||
    qinfo.dqi_free_entry >= qinfo.dqi_blocks)) {
    quota_error(sb, "Block with free entry %u out of range (%u, %u).",
    qinfo.dqi_free_entry, QT_TREEOFF,
    qinfo.dqi_blocks);
    goto out_free;
    }
    ret = 0;
    out_free:
    if (ret) {
    kfree(info.dqi_priv);
    info.dqi_priv = core::ptr::null_mut();
    }
    out:
    memalloc_nofs_restore(memalloc);
    up_read(&dqopt.dqio_sem);
    return ret;
    }
// Write information header to quota file
#[no_mangle]
unsafe extern "C" fn v2_write_file_info(sb: *mut super_block, type: c_int) -> c_int {
    static int v2_write_file_info(struct super_block *sb, int type)
    {
    struct v2_disk_dqinfo dinfo;
    struct quota_info *dqopt = sb_dqopt(sb);
    struct mem_dqinfo *info = &dqopt.info[type];
    struct qtree_mem_dqinfo *qinfo = info.dqi_priv;
    ssize_t size;
    unsigned int memalloc;
    down_write(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    spin_lock(&dq_data_lock);
    info.dqi_flags &= ~DQF_INFO_DIRTY;
    dinfo.dqi_bgrace = cpu_to_le32(info.dqi_bgrace);
    dinfo.dqi_igrace = cpu_to_le32(info.dqi_igrace);
// No flags currently supported
    dinfo.dqi_flags = cpu_to_le32(0);
    spin_unlock(&dq_data_lock);
    dinfo.dqi_blocks = cpu_to_le32(qinfo.dqi_blocks);
    dinfo.dqi_free_blk = cpu_to_le32(qinfo.dqi_free_blk);
    dinfo.dqi_free_entry = cpu_to_le32(qinfo.dqi_free_entry);
    size = sb.s_op.quota_write(sb, type, (char *)&dinfo,
    sizeof(struct v2_disk_dqinfo), V2_DQINFOOFF);
    memalloc_nofs_restore(memalloc);
    up_write(&dqopt.dqio_sem);
    if (size != sizeof(struct v2_disk_dqinfo)) {
    quota_error(sb, "Can't write info structure");
    return size < 0 ? size : -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn v2r0_disk2memdqb(dquot: *mut dquot, dp: *mut c_void) {
    static void v2r0_disk2memdqb(struct dquot *dquot, void *dp)
    {
    struct v2r0_disk_dqblk *d = dp, empty;
    struct mem_dqblk *m = &dquot.dq_dqb;
    m.dqb_ihardlimit = le32_to_cpu(d.dqb_ihardlimit);
    m.dqb_isoftlimit = le32_to_cpu(d.dqb_isoftlimit);
    m.dqb_curinodes = le32_to_cpu(d.dqb_curinodes);
    m.dqb_itime = le64_to_cpu(d.dqb_itime);
    m.dqb_bhardlimit = v2_qbtos(le32_to_cpu(d.dqb_bhardlimit));
    m.dqb_bsoftlimit = v2_qbtos(le32_to_cpu(d.dqb_bsoftlimit));
    m.dqb_curspace = le64_to_cpu(d.dqb_curspace);
    m.dqb_btime = le64_to_cpu(d.dqb_btime);
// We need to escape back all-zero structure
    memset(&empty, 0, sizeof(struct v2r0_disk_dqblk));
    empty.dqb_itime = cpu_to_le64(1);
    if (!memcmp(&empty, dp, sizeof(struct v2r0_disk_dqblk)))
    m.dqb_itime = 0;
    }
#[no_mangle]
unsafe extern "C" fn v2r0_mem2diskdqb(dp: *mut c_void, dquot: *mut dquot) {
    static void v2r0_mem2diskdqb(void *dp, struct dquot *dquot)
    {
    struct v2r0_disk_dqblk *d = dp;
    struct mem_dqblk *m = &dquot.dq_dqb;
    struct qtree_mem_dqinfo *info =
    sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv;
    d.dqb_ihardlimit = cpu_to_le32(m.dqb_ihardlimit);
    d.dqb_isoftlimit = cpu_to_le32(m.dqb_isoftlimit);
    d.dqb_curinodes = cpu_to_le32(m.dqb_curinodes);
    d.dqb_itime = cpu_to_le64(m.dqb_itime);
    d.dqb_bhardlimit = cpu_to_le32(v2_stoqb(m.dqb_bhardlimit));
    d.dqb_bsoftlimit = cpu_to_le32(v2_stoqb(m.dqb_bsoftlimit));
    d.dqb_curspace = cpu_to_le64(m.dqb_curspace);
    d.dqb_btime = cpu_to_le64(m.dqb_btime);
    d.dqb_id = cpu_to_le32(from_kqid(&init_user_ns, dquot.dq_id));
    if (qtree_entry_unused(info, dp))
    d.dqb_itime = cpu_to_le64(1);
    }
#[no_mangle]
unsafe extern "C" fn v2r0_is_id(dp: *mut c_void, dquot: *mut dquot) -> c_int {
    static int v2r0_is_id(void *dp, struct dquot *dquot)
    {
    struct v2r0_disk_dqblk *d = dp;
    struct qtree_mem_dqinfo *info =
    sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv;
    if (qtree_entry_unused(info, dp))
    return 0;
    return qid_eq(make_kqid(&init_user_ns, dquot.dq_id.type,
    le32_to_cpu(d.dqb_id)),
    dquot.dq_id);
    }
#[no_mangle]
unsafe extern "C" fn v2r1_disk2memdqb(dquot: *mut dquot, dp: *mut c_void) {
    static void v2r1_disk2memdqb(struct dquot *dquot, void *dp)
    {
    struct v2r1_disk_dqblk *d = dp, empty;
    struct mem_dqblk *m = &dquot.dq_dqb;
    m.dqb_ihardlimit = le64_to_cpu(d.dqb_ihardlimit);
    m.dqb_isoftlimit = le64_to_cpu(d.dqb_isoftlimit);
    m.dqb_curinodes = le64_to_cpu(d.dqb_curinodes);
    m.dqb_itime = le64_to_cpu(d.dqb_itime);
    m.dqb_bhardlimit = v2_qbtos(le64_to_cpu(d.dqb_bhardlimit));
    m.dqb_bsoftlimit = v2_qbtos(le64_to_cpu(d.dqb_bsoftlimit));
    m.dqb_curspace = le64_to_cpu(d.dqb_curspace);
    m.dqb_btime = le64_to_cpu(d.dqb_btime);
// We need to escape back all-zero structure
    memset(&empty, 0, sizeof(struct v2r1_disk_dqblk));
    empty.dqb_itime = cpu_to_le64(1);
    if (!memcmp(&empty, dp, sizeof(struct v2r1_disk_dqblk)))
    m.dqb_itime = 0;
    }
#[no_mangle]
unsafe extern "C" fn v2r1_mem2diskdqb(dp: *mut c_void, dquot: *mut dquot) {
    static void v2r1_mem2diskdqb(void *dp, struct dquot *dquot)
    {
    struct v2r1_disk_dqblk *d = dp;
    struct mem_dqblk *m = &dquot.dq_dqb;
    struct qtree_mem_dqinfo *info =
    sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv;
    d.dqb_ihardlimit = cpu_to_le64(m.dqb_ihardlimit);
    d.dqb_isoftlimit = cpu_to_le64(m.dqb_isoftlimit);
    d.dqb_curinodes = cpu_to_le64(m.dqb_curinodes);
    d.dqb_itime = cpu_to_le64(m.dqb_itime);
    d.dqb_bhardlimit = cpu_to_le64(v2_stoqb(m.dqb_bhardlimit));
    d.dqb_bsoftlimit = cpu_to_le64(v2_stoqb(m.dqb_bsoftlimit));
    d.dqb_curspace = cpu_to_le64(m.dqb_curspace);
    d.dqb_btime = cpu_to_le64(m.dqb_btime);
    d.dqb_id = cpu_to_le32(from_kqid(&init_user_ns, dquot.dq_id));
    d.dqb_pad = 0;
    if (qtree_entry_unused(info, dp))
    d.dqb_itime = cpu_to_le64(1);
    }
#[no_mangle]
unsafe extern "C" fn v2r1_is_id(dp: *mut c_void, dquot: *mut dquot) -> c_int {
    static int v2r1_is_id(void *dp, struct dquot *dquot)
    {
    struct v2r1_disk_dqblk *d = dp;
    struct qtree_mem_dqinfo *info =
    sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv;
    if (qtree_entry_unused(info, dp))
    return 0;
    return qid_eq(make_kqid(&init_user_ns, dquot.dq_id.type,
    le32_to_cpu(d.dqb_id)),
    dquot.dq_id);
    }
#[no_mangle]
unsafe extern "C" fn v2_read_dquot(dquot: *mut dquot) -> c_int {
    static int v2_read_dquot(struct dquot *dquot)
    {
    struct quota_info *dqopt = sb_dqopt(dquot.dq_sb);
    int ret;
    unsigned int memalloc;
    down_read(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    ret = qtree_read_dquot(
    sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv,
    dquot);
    memalloc_nofs_restore(memalloc);
    up_read(&dqopt.dqio_sem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn v2_write_dquot(dquot: *mut dquot) -> c_int {
    static int v2_write_dquot(struct dquot *dquot)
    {
    struct quota_info *dqopt = sb_dqopt(dquot.dq_sb);
    int ret;
    let mut alloc: bool = false;
    unsigned int memalloc;
//
// If space for dquot is already allocated, we don't need any
// protection as we'll only overwrite the place of dquot. We are
// still protected by concurrent writes of the same dquot by
// dquot->dq_lock.
//
    if (!dquot.dq_off) {
    alloc = true;
    down_write(&dqopt.dqio_sem);
    } else {
    down_read(&dqopt.dqio_sem);
    }
    memalloc = memalloc_nofs_save();
    ret = qtree_write_dquot(
    sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv,
    dquot);
    memalloc_nofs_restore(memalloc);
    if (alloc)
    up_write(&dqopt.dqio_sem);
    else
    up_read(&dqopt.dqio_sem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn v2_release_dquot(dquot: *mut dquot) -> c_int {
    static int v2_release_dquot(struct dquot *dquot)
    {
    struct quota_info *dqopt = sb_dqopt(dquot.dq_sb);
    unsigned int memalloc;
    int ret;
    down_write(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    ret = qtree_release_dquot(sb_dqinfo(dquot.dq_sb, dquot.dq_id.type).dqi_priv, dquot);
    memalloc_nofs_restore(memalloc);
    up_write(&dqopt.dqio_sem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn v2_free_file_info(sb: *mut super_block, type: c_int) -> c_int {
    static int v2_free_file_info(struct super_block *sb, int type)
    {
    kfree(sb_dqinfo(sb, type).dqi_priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn v2_get_next_id(sb: *mut super_block, qid: *mut kqid) -> c_int {
    static int v2_get_next_id(struct super_block *sb, struct kqid *qid)
    {
    struct quota_info *dqopt = sb_dqopt(sb);
    unsigned int memalloc;
    int ret;
    down_read(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    ret = qtree_get_next_id(sb_dqinfo(sb, qid.type).dqi_priv, qid);
    memalloc_nofs_restore(memalloc);
    up_read(&dqopt.dqio_sem);
    return ret;
    }
    static const struct quota_format_ops v2_format_ops = {
    .check_quota_file	= v2_check_quota_file,
    .read_file_info		= v2_read_file_info,
    .write_file_info	= v2_write_file_info,
    .free_file_info		= v2_free_file_info,
    .read_dqblk		= v2_read_dquot,
    .commit_dqblk		= v2_write_dquot,
    .release_dqblk		= v2_release_dquot,
    .get_next_id		= v2_get_next_id,
    };
    static struct quota_format_type v2r0_quota_format = {
    .qf_fmt_id	= QFMT_VFS_V0,
    .qf_ops		= &v2_format_ops,
    .qf_owner	= THIS_MODULE
    };
    static struct quota_format_type v2r1_quota_format = {
    .qf_fmt_id	= QFMT_VFS_V1,
    .qf_ops		= &v2_format_ops,
    .qf_owner	= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn init_v2_quota_format() -> int __init {
    static int __init init_v2_quota_format(void)
    {
    register_quota_format(&v2r0_quota_format);
    register_quota_format(&v2r1_quota_format);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exit_v2_quota_format() -> void __exit {
    static void __exit exit_v2_quota_format(void)
    {
    unregister_quota_format(&v2r0_quota_format);
    unregister_quota_format(&v2r1_quota_format);
    }
    module_init(init_v2_quota_format);
    module_exit(exit_v2_quota_format);
