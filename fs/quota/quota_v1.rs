//! Automatically rewritten from C to Rust
//! Source: fs/quota/quota_v1.c
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

    MODULE_AUTHOR("Jan Kara");
    MODULE_DESCRIPTION("Old quota format support");
    MODULE_LICENSE("GPL");
pub const QUOTABLOCK_BITS: c_int = 10;

#[no_mangle]
pub unsafe extern "C" fn v1_stoqb(space: qsize_t) -> qsize_t {
    static inline qsize_t v1_stoqb(qsize_t space)
    {
    return (space + QUOTABLOCK_SIZE - 1) >> QUOTABLOCK_BITS;
    }
#[no_mangle]
pub unsafe extern "C" fn v1_qbtos(blocks: qsize_t) -> qsize_t {
    static inline qsize_t v1_qbtos(qsize_t blocks)
    {
    return blocks << QUOTABLOCK_BITS;
    }
#[no_mangle]
unsafe extern "C" fn v1_disk2mem_dqblk(m: *mut mem_dqblk, d: *mut v1_disk_dqblk) {
    static void v1_disk2mem_dqblk(struct mem_dqblk *m, struct v1_disk_dqblk *d)
    {
    m.dqb_ihardlimit = d.dqb_ihardlimit;
    m.dqb_isoftlimit = d.dqb_isoftlimit;
    m.dqb_curinodes = d.dqb_curinodes;
    m.dqb_bhardlimit = v1_qbtos(d.dqb_bhardlimit);
    m.dqb_bsoftlimit = v1_qbtos(d.dqb_bsoftlimit);
    m.dqb_curspace = v1_qbtos(d.dqb_curblocks);
    m.dqb_itime = d.dqb_itime;
    m.dqb_btime = d.dqb_btime;
    }
#[no_mangle]
unsafe extern "C" fn v1_mem2disk_dqblk(d: *mut v1_disk_dqblk, m: *mut mem_dqblk) {
    static void v1_mem2disk_dqblk(struct v1_disk_dqblk *d, struct mem_dqblk *m)
    {
    d.dqb_ihardlimit = m.dqb_ihardlimit;
    d.dqb_isoftlimit = m.dqb_isoftlimit;
    d.dqb_curinodes = m.dqb_curinodes;
    d.dqb_bhardlimit = v1_stoqb(m.dqb_bhardlimit);
    d.dqb_bsoftlimit = v1_stoqb(m.dqb_bsoftlimit);
    d.dqb_curblocks = v1_stoqb(m.dqb_curspace);
    d.dqb_itime = m.dqb_itime;
    d.dqb_btime = m.dqb_btime;
    }
#[no_mangle]
unsafe extern "C" fn v1_read_dqblk(dquot: *mut dquot) -> c_int {
    static int v1_read_dqblk(struct dquot *dquot)
    {
    let mut type: c_int = dquot.dq_id.type;
    struct v1_disk_dqblk dqblk;
    struct quota_info *dqopt = sb_dqopt(dquot.dq_sb);
    if (!dqopt.files[type])
    return -EINVAL;
// Set structure to 0s in case read fails/is after end of file
    memset(&dqblk, 0, sizeof(struct v1_disk_dqblk));
    dquot.dq_sb.s_op.quota_read(dquot.dq_sb, type, (char *)&dqblk,
    sizeof(struct v1_disk_dqblk),
    v1_dqoff(from_kqid(&init_user_ns, dquot.dq_id)));
    v1_disk2mem_dqblk(&dquot.dq_dqb, &dqblk);
    if (dquot.dq_dqb.dqb_bhardlimit == 0 &&
    dquot.dq_dqb.dqb_bsoftlimit == 0 &&
    dquot.dq_dqb.dqb_ihardlimit == 0 &&
    dquot.dq_dqb.dqb_isoftlimit == 0)
    set_bit(DQ_FAKE_B, &dquot.dq_flags);
    dqstats_inc(DQST_READS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn v1_commit_dqblk(dquot: *mut dquot) -> c_int {
    static int v1_commit_dqblk(struct dquot *dquot)
    {
    let mut type: c_short = dquot.dq_id.type;
    ssize_t ret;
    struct v1_disk_dqblk dqblk;
    v1_mem2disk_dqblk(&dqblk, &dquot.dq_dqb);
    if (((type == USRQUOTA) && uid_eq(dquot.dq_id.uid, GLOBAL_ROOT_UID)) ||
    ((type == GRPQUOTA) && gid_eq(dquot.dq_id.gid, GLOBAL_ROOT_GID))) {
    dqblk.dqb_btime =
    sb_dqopt(dquot.dq_sb).info[type].dqi_bgrace;
    dqblk.dqb_itime =
    sb_dqopt(dquot.dq_sb).info[type].dqi_igrace;
    }
    ret = 0;
    if (sb_dqopt(dquot.dq_sb).files[type])
    ret = dquot.dq_sb.s_op.quota_write(dquot.dq_sb, type,
    (char *)&dqblk, sizeof(struct v1_disk_dqblk),
    v1_dqoff(from_kqid(&init_user_ns, dquot.dq_id)));
    if (ret != sizeof(struct v1_disk_dqblk)) {
    quota_error(dquot.dq_sb, "dquota write failed");
    if (ret >= 0)
    ret = -EIO;
    goto out;
    }
    ret = 0;
    out:
    dqstats_inc(DQST_WRITES);
    return ret;
    }
// Magics of new quota format

    0xd9c01f11,     /* USRQUOTA */\
    0xd9c01927      /* GRPQUOTA */\
    }
// Header of new quota format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v2_disk_dqheader {
    pub /: *mut *mut __le32 dqh_magic; / Magic number identifying file,
    pub /: *mut *mut __le32 dqh_version; / File version,
}

#[no_mangle]
unsafe extern "C" fn v1_check_quota_file(sb: *mut super_block, type: c_int) -> c_int {
    static int v1_check_quota_file(struct super_block *sb, int type)
    {
    struct inode *inode = sb_dqopt(sb).files[type];
    ulong blocks;
    size_t off;
    struct v2_disk_dqheader dqhead;
    ssize_t size;
    loff_t isize;
    static const uint quota_magics[] = V2_INITQMAGICS;
    isize = i_size_read(inode);
    if (!isize)
    return 0;
    blocks = isize >> BLOCK_SIZE_BITS;
    off = isize & (BLOCK_SIZE - 1);
    if ((blocks % sizeof(struct v1_disk_dqblk) * BLOCK_SIZE + off) %
    sizeof(struct v1_disk_dqblk))
    return 0;
// Doublecheck whether we didn't get file with new format - with old
// quotactl() this could happen
    size = sb.s_op.quota_read(sb, type, (char *)&dqhead,
    sizeof(struct v2_disk_dqheader), 0);
    if (size != sizeof(struct v2_disk_dqheader))
    return 1;	/* Probably not new format */
    if (le32_to_cpu(dqhead.dqh_magic) != quota_magics[type])
    return 1;	/* Definitely not new format */
    printk(KERN_INFO
    "VFS: %s: Refusing to turn on old quota format on given file."
    " It probably contains newer quota format.\n", sb.s_id);
    return 0;		/* Seems like a new format file . refuse it */
    }
#[no_mangle]
unsafe extern "C" fn v1_read_file_info(sb: *mut super_block, type: c_int) -> c_int {
    static int v1_read_file_info(struct super_block *sb, int type)
    {
    struct quota_info *dqopt = sb_dqopt(sb);
    struct v1_disk_dqblk dqblk;
    unsigned int memalloc;
    int ret;
    down_read(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    ret = sb.s_op.quota_read(sb, type, (char *)&dqblk,
    sizeof(struct v1_disk_dqblk), v1_dqoff(0));
    if (ret != sizeof(struct v1_disk_dqblk)) {
    if (ret >= 0)
    ret = -EIO;
    goto out;
    }
    ret = 0;
// limits are stored as unsigned 32-bit data
    dqopt.info[type].dqi_max_spc_limit = 0xffffffffULL << QUOTABLOCK_BITS;
    dqopt.info[type].dqi_max_ino_limit = 0xffffffff;
    dqopt.info[type].dqi_igrace =
    dqblk.dqb_itime ? dqblk.dqb_itime : MAX_IQ_TIME;
    dqopt.info[type].dqi_bgrace =
    dqblk.dqb_btime ? dqblk.dqb_btime : MAX_DQ_TIME;
    out:
    memalloc_nofs_restore(memalloc);
    up_read(&dqopt.dqio_sem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn v1_write_file_info(sb: *mut super_block, type: c_int) -> c_int {
    static int v1_write_file_info(struct super_block *sb, int type)
    {
    struct quota_info *dqopt = sb_dqopt(sb);
    struct v1_disk_dqblk dqblk;
    unsigned int memalloc;
    int ret;
    down_write(&dqopt.dqio_sem);
    memalloc = memalloc_nofs_save();
    ret = sb.s_op.quota_read(sb, type, (char *)&dqblk,
    sizeof(struct v1_disk_dqblk), v1_dqoff(0));
    if (ret != sizeof(struct v1_disk_dqblk)) {
    if (ret >= 0)
    ret = -EIO;
    goto out;
    }
    spin_lock(&dq_data_lock);
    dqopt.info[type].dqi_flags &= ~DQF_INFO_DIRTY;
    dqblk.dqb_itime = dqopt.info[type].dqi_igrace;
    dqblk.dqb_btime = dqopt.info[type].dqi_bgrace;
    spin_unlock(&dq_data_lock);
    ret = sb.s_op.quota_write(sb, type, (char *)&dqblk,
    sizeof(struct v1_disk_dqblk), v1_dqoff(0));
    if (ret == sizeof(struct v1_disk_dqblk))
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    out:
    memalloc_nofs_restore(memalloc);
    up_write(&dqopt.dqio_sem);
    return ret;
    }
    static const struct quota_format_ops v1_format_ops = {
    .check_quota_file	= v1_check_quota_file,
    .read_file_info		= v1_read_file_info,
    .write_file_info	= v1_write_file_info,
    .read_dqblk		= v1_read_dqblk,
    .commit_dqblk		= v1_commit_dqblk,
    };
    static struct quota_format_type v1_quota_format = {
    .qf_fmt_id	= QFMT_VFS_OLD,
    .qf_ops		= &v1_format_ops,
    .qf_owner	= THIS_MODULE
    };
#[no_mangle]
unsafe extern "C" fn init_v1_quota_format() -> int __init {
    static int __init init_v1_quota_format(void)
    {
    register_quota_format(&v1_quota_format);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exit_v1_quota_format() -> void __exit {
    static void __exit exit_v1_quota_format(void)
    {
    unregister_quota_format(&v1_quota_format);
    }
    module_init(init_v1_quota_format);
    module_exit(exit_v1_quota_format);
