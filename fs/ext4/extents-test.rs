//! Automatically rewritten from C to Rust
//! Source: fs/ext4/extents-test.c
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
// Written by Ojaswin Mujoo <ojaswin@linux.ibm.com> (IBM)
//
// These Kunit tests are designed to test the functionality of
// extent split and conversion in ext4.
//
// Currently, ext4 can split extents in 2 ways:
// 1. By splitting the extents in the extent tree and optionally converting them
// to written or unwritten based on flags passed.
// 2. In case 1 encounters an error, ext4 instead zerooes out the unwritten
// areas of the extent and marks the complete extent written.
//
// The primary function that handles this is ext4_split_convert_extents().
//
// We test both of the methods of split. The behavior we try to enforce is:
// 1. When passing EXT4_GET_BLOCKS_CONVERT flag to ext4_split_convert_extents(),
// the split extent should be converted to initialized.
// 2. When passing EXT4_GET_BLOCKS_CONVERT_UNWRITTEN flag to
// ext4_split_convert_extents(), the split extent should be converted to
// uninitialized.
// 3. In case we use the zeroout method, then we should correctly write zeroes
// to the unwritten areas of the extent and we should not corrupt/leak any
// data.
//
// Enforcing 1 and 2 is straight forward, we just setup a minimal inode with
// extent tree, call ext4_split_convert_extents() and check the final state of
// the extent tree.
//
// For zeroout testing, we maintain a separate buffer which represents the disk
// data corresponding to the extents. We then override ext4's zeroout functions
// to instead write zeroes to our buffer. Then, we override
// ext4_ext_insert_extent() to return -ENOSPC, which triggers the zeroout.
// Finally, we check the state of the extent tree and zeroout buffer to confirm
// everything went well.
//

pub const EXT_DATA_PBLK: c_int = 100;
pub const EXT_DATA_LBLK: c_int = 10;
pub const EXT_DATA_LEN: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_ctx {
//
// Ext4 inode which has only 1 unwrit extent
//
    pub k_ei: *mut ext4_inode_info,
//
// Represents the underlying data area (used for zeroout testing)
//
    pub k_data: *mut c_char,
    pub k_ctx: },
//
// describes the state of an expected extent in extent tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_ext_state {
    pub ex_lblk: ext4_lblk_t,
    pub ex_len: ext4_lblk_t,
    pub is_unwrit: bool,
}

//
// describes the state of the data area of a writ extent. Used for testing
// correctness of zeroout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_ext_data_state {
    pub exp_char: c_char,
    pub off_blk: ext4_lblk_t,
    pub len_blk: ext4_lblk_t,
}

    enum kunit_test_types {
    TEST_SPLIT_CONVERT,
    TEST_CREATE_BLOCKS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_ext_test_param {
// description of test
    pub desc: *mut c_char,
// determines which function will be tested
    pub type: c_int,
// is extent unwrit at beginning of test
    pub is_unwrit_at_start: bool,
// flags to pass while splitting
    pub split_flags: c_int,
// map describing range to split
    pub split_map: ext4_map_blocks,
// disable zeroout
    pub disable_zeroout: bool,
// no of extents expected after split
    pub nr_exp_ext: c_int,
//
// expected state of extents after split. We will never split into more
// than 3 extents
//
    pub exp_ext_state: [kunit_ext_state; 3],
// Below fields used for zeroout tests
    pub is_zeroout_test: bool,
//
// no of expected data segments (zeroout tests). Example, if we expect
// data to be 4kb 0s, followed by 8kb non-zero, then nr_exp_data_segs==2
//
    pub nr_exp_data_segs: c_int,
//
// expected state of data area after zeroout.
//
    pub exp_data_state: [kunit_ext_data_state; 3],
}

#[no_mangle]
unsafe extern "C" fn ext_init_fs_context(fc: *mut fs_context) -> c_int {
    static int ext_init_fs_context(struct fs_context *fc)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ext_set(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    static int ext_set(struct super_block *sb, struct fs_context *fc)
    {
    return set_anon_super_fc(sb, fc);
    }
    static struct file_system_type ext_fs_type = {
    .name		 = "extents test",
    .init_fs_context = ext_init_fs_context,
    .kill_sb	 = kill_anon_super,
    };
#[no_mangle]
unsafe extern "C" fn extents_kunit_exit(test: *mut kunit) {
    static void extents_kunit_exit(struct kunit *test)
    {
    struct ext4_sb_info *sbi;
    if (!k_ctx.k_ei)
    return;
    sbi = k_ctx.k_ei.vfs_inode.i_sb.s_fs_info;
    ext4_es_unregister_shrinker(sbi);
    deactivate_super(sbi.s_sb);
    kfree(sbi);
    kfree(k_ctx.k_ei);
    kfree(k_ctx.k_data);
    }
    static int __ext4_ext_dirty_stub(const char *where, unsigned int line,
    handle_t *handle, struct inode *inode,
    struct ext4_ext_path *path)
    {
    return 0;
    }
    static struct ext4_ext_path *
    ext4_ext_insert_extent_stub(handle_t *handle, struct inode *inode,
    struct ext4_ext_path *path,
    struct ext4_extent *newext, int gb_flags)
    {
    return ERR_PTR(-ENOSPC);
    }
//
// We will zeroout the equivalent range in the data area
//
#[no_mangle]
unsafe extern "C" fn ext4_ext_zeroout_stub(inode: *mut inode, ex: *mut ext4_extent) -> c_int {
    static int ext4_ext_zeroout_stub(struct inode *inode, struct ext4_extent *ex)
    {
    ext4_lblk_t ee_block, off_blk;
    loff_t ee_len;
    loff_t off_bytes;
    struct kunit *test = kunit_get_current_test();
    ee_block = le32_to_cpu(ex.ee_block);
    ee_len = ext4_ext_get_actual_len(ex);
    KUNIT_EXPECT_EQ_MSG(test, 1, ee_block >= EXT_DATA_LBLK, "ee_block=%d",
    ee_block);
    KUNIT_EXPECT_EQ(test, 1,
    ee_block + ee_len <= EXT_DATA_LBLK + EXT_DATA_LEN);
    off_blk = ee_block - EXT_DATA_LBLK;
    off_bytes = off_blk << inode.i_sb.s_blocksize_bits;
    memset(k_ctx.k_data + off_bytes, 0,
    ee_len << inode.i_sb.s_blocksize_bits);
    return 0;
    }
    static int ext4_issue_zeroout_stub(struct inode *inode, ext4_lblk_t lblk,
    ext4_fsblk_t pblk, ext4_lblk_t len)
    {
    ext4_lblk_t off_blk;
    loff_t off_bytes;
    struct kunit *test = kunit_get_current_test();
    kunit_log(KERN_ALERT, test,
    "%s: lblk=%u pblk=%llu len=%u", __func__, lblk, pblk, len);
    KUNIT_EXPECT_EQ(test, 1, lblk >= EXT_DATA_LBLK);
    KUNIT_EXPECT_EQ(test, 1, lblk + len <= EXT_DATA_LBLK + EXT_DATA_LEN);
    KUNIT_EXPECT_EQ(test, 1, lblk - EXT_DATA_LBLK == pblk - EXT_DATA_PBLK);
    off_blk = lblk - EXT_DATA_LBLK;
    off_bytes = off_blk << inode.i_sb.s_blocksize_bits;
    memset(k_ctx.k_data + off_bytes, 0,
    len << inode.i_sb.s_blocksize_bits);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn extents_kunit_init(test: *mut kunit) -> c_int {
    static int extents_kunit_init(struct kunit *test)
    {
    struct ext4_extent_header *eh = core::ptr::null_mut();
    struct ext4_inode_info *ei;
    struct inode *inode;
    struct super_block *sb;
    struct fs_context *fc;
    struct ext4_sb_info *sbi = core::ptr::null_mut();
    struct kunit_ext_test_param *param =
    (struct kunit_ext_test_param *)(test.param_value);
    int err;
    sbi = kzalloc_obj(struct ext4_sb_info);
    if (sbi == core::ptr::null_mut())
    return -ENOMEM;
    fc = fs_context_for_mount(&ext_fs_type, 0);
    if (IS_ERR(fc)) {
    kfree(sbi);
    return PTR_ERR(fc);
    }
    sb = sget_fc(fc, core::ptr::null_mut(), ext_set);
    put_fs_context(fc);
    if (IS_ERR(sb)) {
    kfree(sbi);
    return PTR_ERR(sb);
    }
    sbi.s_sb = sb;
    sb.s_fs_info = sbi;
    sb.s_blocksize = 4096;
    sb.s_blocksize_bits = 12;
    if (!param || !param.disable_zeroout)
    sbi.s_extent_max_zeroout_kb = 32;
    err = ext4_es_register_shrinker(sbi);
    if (err)
    goto out_deactivate;
// setup the mock inode
    k_ctx.k_ei = kzalloc_obj(struct ext4_inode_info);
    if (k_ctx.k_ei == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out;
    }
    ei = k_ctx.k_ei;
    inode = &ei.vfs_inode;
    ext4_es_init_tree(&ei.i_es_tree);
    rwlock_init(&ei.i_es_lock);
    INIT_LIST_HEAD(&ei.i_es_list);
    ei.i_es_all_nr = 0;
    ei.i_es_shk_nr = 0;
    ei.i_es_shrink_lblk = 0;
    ei.i_disksize = (EXT_DATA_LBLK + EXT_DATA_LEN + 10)
    << sb.s_blocksize_bits;
    ei.i_flags = 0;
    ext4_set_inode_flag(inode, EXT4_INODE_EXTENTS);
    inode.i_sb = sb;
    k_ctx.k_data = kzalloc(EXT_DATA_LEN * 4096, GFP_KERNEL);
    if (k_ctx.k_data == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out;
    }
//
// set the data area to a junk value
//
    memset(k_ctx.k_data, 'X', EXT_DATA_LEN * 4096);
// create a tree with depth 0
    eh = (struct ext4_extent_header *)k_ctx.k_ei.i_data;
// Fill extent header
    eh = ext_inode_hdr(&k_ctx.k_ei.vfs_inode);
    eh.eh_depth = 0;
    eh.eh_entries = cpu_to_le16(1);
    eh.eh_magic = EXT4_EXT_MAGIC;
    eh.eh_max = cpu_to_le16(ext4_ext_space_root_idx_test(
    &k_ctx.k_ei.vfs_inode, 0));
    eh.eh_generation = 0;
//
// add 1 extent in leaf node covering:
// - lblks: [EXT_DATA_LBLK, EXT_DATA_LBLK * + EXT_DATA_LEN)
// - pblks: [EXT_DATA_PBLK, EXT_DATA_PBLK + EXT_DATA_LEN)
//
    EXT_FIRST_EXTENT(eh).ee_block = cpu_to_le32(EXT_DATA_LBLK);
    EXT_FIRST_EXTENT(eh).ee_len = cpu_to_le16(EXT_DATA_LEN);
    ext4_ext_store_pblock(EXT_FIRST_EXTENT(eh), EXT_DATA_PBLK);
    if (!param || param.is_unwrit_at_start)
    ext4_ext_mark_unwritten(EXT_FIRST_EXTENT(eh));
    ext4_es_insert_extent(inode, EXT_DATA_LBLK, EXT_DATA_LEN, EXT_DATA_PBLK,
    ext4_ext_is_unwritten(EXT_FIRST_EXTENT(eh)) ?
    EXTENT_STATUS_UNWRITTEN :
    EXTENT_STATUS_WRITTEN,
    0);
// Add stubs
    kunit_activate_static_stub(test, __ext4_ext_dirty,
    __ext4_ext_dirty_stub);
    kunit_activate_static_stub(test, ext4_ext_zeroout, ext4_ext_zeroout_stub);
    kunit_activate_static_stub(test, ext4_issue_zeroout,
    ext4_issue_zeroout_stub);
    up_write(&sb.s_umount);
    return 0;
    out:
    kfree(k_ctx.k_ei);
    k_ctx.k_ei = core::ptr::null_mut();
    kfree(k_ctx.k_data);
    k_ctx.k_data = core::ptr::null_mut();
    ext4_es_unregister_shrinker(sbi);
    out_deactivate:
    deactivate_locked_super(sb);
    kfree(sbi);
    return err;
    }
//
// Return 1 if all bytes in the buf equal to c, else return the offset of first mismatch
//
#[no_mangle]
unsafe extern "C" fn check_buffer(buf: *mut c_char, c: c_int, size: c_int) -> c_int {
    static int check_buffer(char *buf, int c, int size)
    {
    void *ret = core::ptr::null_mut();
    ret = memchr_inv(buf, c, size);
    if (ret  == core::ptr::null_mut())
    return 0;
    kunit_log(KERN_ALERT, kunit_get_current_test(),
    "# %s: wrong char found at offset %u (expected:%d got:%d)", __func__,
    (u32)((char *)ret - buf), c, *((char *)ret));
    return 1;
    }
//
// Simulate a map block call by first calling ext4_map_query_blocks() to
// correctly populate map flags and pblk and then call the
// ext4_map_create_blocks() to do actual split and conversion. This is easier
// than calling ext4_map_blocks() because that needs mocking a lot of unrelated
// functions.
//
    static void ext4_map_create_blocks_helper(struct kunit *test,
    struct inode *inode,
    struct ext4_map_blocks *map,
    int flags)
    {
    let mut retval: c_int = 0;
    retval = ext4_map_query_blocks(core::ptr::null_mut(), inode, map, flags);
    if (retval < 0) {
    KUNIT_FAIL(test,
    "ext4_map_query_blocks() failed. Cannot proceed\n");
    return;
    }
    ext4_map_create_blocks(core::ptr::null_mut(), inode, map, flags);
    }
#[no_mangle]
unsafe extern "C" fn test_split_convert(test: *mut kunit) {
    static void test_split_convert(struct kunit *test)
    {
    struct ext4_ext_path *path;
    struct inode *inode = &k_ctx.k_ei.vfs_inode;
    struct ext4_extent *ex;
    struct ext4_map_blocks map;
    const struct kunit_ext_test_param *param =
    (const struct kunit_ext_test_param *)(test.param_value);
    let mut blkbits: c_int = inode.i_sb.s_blocksize_bits;
    if (param.is_zeroout_test)
//
// Force zeroout by making ext4_ext_insert_extent return ENOSPC
//
    kunit_activate_static_stub(test, ext4_ext_insert_extent,
    ext4_ext_insert_extent_stub);
    path = ext4_find_extent(inode, EXT_DATA_LBLK, core::ptr::null_mut(), EXT4_EX_NOCACHE);
    ex = path.p_ext;
    KUNIT_EXPECT_EQ(test, EXT_DATA_LBLK, le32_to_cpu(ex.ee_block));
    KUNIT_EXPECT_EQ(test, EXT_DATA_LEN, ext4_ext_get_actual_len(ex));
    KUNIT_EXPECT_EQ(test, param.is_unwrit_at_start,
    ext4_ext_is_unwritten(ex));
    if (param.is_zeroout_test)
    KUNIT_EXPECT_EQ(test, 0,
    check_buffer(k_ctx.k_data, 'X',
    EXT_DATA_LEN << blkbits));
    map.m_lblk = param.split_map.m_lblk;
    map.m_len = param.split_map.m_len;
    switch (param.type) {
    case TEST_SPLIT_CONVERT:
    path = ext4_split_convert_extents_test(core::ptr::null_mut(), inode, &map,
    path, param.split_flags, core::ptr::null_mut());
    break;
    case TEST_CREATE_BLOCKS:
    ext4_map_create_blocks_helper(test, inode, &map, param.split_flags);
    break;
    default:
    KUNIT_FAIL(test, "param.type %d not support.", param.type);
    }
    path = ext4_find_extent(inode, EXT_DATA_LBLK, core::ptr::null_mut(), EXT4_EX_NOCACHE);
    ex = path.p_ext;
    for (int i = 0; i < param.nr_exp_ext; i++) {
    let mut exp_ext: kunit_ext_state = param.exp_ext_state[i];
    let mut es_check_needed: bool = param.type != TEST_SPLIT_CONVERT;
    struct extent_status es;
    int contains_ex, ex_end, es_end, es_pblk;
    KUNIT_EXPECT_EQ(test, exp_ext.ex_lblk,
    le32_to_cpu(ex.ee_block));
    KUNIT_EXPECT_EQ(test, exp_ext.ex_len,
    ext4_ext_get_actual_len(ex));
    KUNIT_EXPECT_EQ(test, exp_ext.is_unwrit,
    ext4_ext_is_unwritten(ex));
//
// Confirm extent cache is in sync. Note that es cache can be
// merged even when on-disk extents are not so take that into
// account.
//
// Also, ext4_split_convert_extents() forces EXT4_EX_NOCACHE hence
// es status are ignored for that case.
//
    if (es_check_needed) {
    ext4_es_lookup_extent(inode, le32_to_cpu(ex.ee_block),
    core::ptr::null_mut(), &es, core::ptr::null_mut());
    ex_end = exp_ext.ex_lblk + exp_ext.ex_len;
    es_end = es.es_lblk + es.es_len;
    contains_ex = es.es_lblk <= exp_ext.ex_lblk &&
    es_end >= ex_end;
    es_pblk = ext4_es_pblock(&es) +
    (exp_ext.ex_lblk - es.es_lblk);
    KUNIT_EXPECT_EQ(test, contains_ex, 1);
    KUNIT_EXPECT_EQ(test, ext4_ext_pblock(ex), es_pblk);
    KUNIT_EXPECT_EQ(test, 1,
    (exp_ext.is_unwrit &&
    ext4_es_is_unwritten(&es)) ||
    (!exp_ext.is_unwrit &&
    ext4_es_is_written(&es)));
    }
// Only printed on failure
    kunit_log(KERN_INFO, test,
    "# [extent %d] exp: lblk:%d len:%d unwrit:%d \n", i,
    exp_ext.ex_lblk, exp_ext.ex_len, exp_ext.is_unwrit);
    kunit_log(KERN_INFO, test,
    "# [extent %d] got: lblk:%d len:%d unwrit:%d\n", i,
    le32_to_cpu(ex.ee_block),
    ext4_ext_get_actual_len(ex),
    ext4_ext_is_unwritten(ex));
    if (es_check_needed)
    kunit_log(
    KERN_INFO, test,
    "# [extent %d] es: lblk:%d len:%d pblk:%lld type:0x%x\n",
    i, es.es_lblk, es.es_len, ext4_es_pblock(&es),
    ext4_es_type(&es));
    kunit_log(KERN_INFO, test, "------------------\n");
    ex = ex + 1;
    }
    if (!param.is_zeroout_test)
    return;
//
// Check that then data area has been zeroed out correctly
//
    for (int i = 0; i < param.nr_exp_data_segs; i++) {
    loff_t off, len;
    let mut exp_data_seg: kunit_ext_data_state = param.exp_data_state[i];
    off = exp_data_seg.off_blk << blkbits;
    len = exp_data_seg.len_blk << blkbits;
    KUNIT_EXPECT_EQ_MSG(test, 0,
    check_buffer(k_ctx.k_data + off,
    exp_data_seg.exp_char, len),
    "# corruption in byte range [%lld, %lld)",
    off, len);
    }
    return;
    }
    static const struct kunit_ext_test_param test_split_convert_params[] = {
// unwrit to writ splits
    { .desc = "split unwrit extent to 2 extents and convert 1st half writ",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
    { .desc = "split unwrit extent to 2 extents and convert 2nd half writ",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
    { .desc = "split unwrit extent to 3 extents and convert 2nd half to writ",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 3,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 2,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1 + (EXT_DATA_LEN - 2),
    .ex_len = 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
// writ to unwrit splits
    { .desc = "split writ extent to 2 extents and convert 1st half unwrit",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
    { .desc = "split writ extent to 2 extents and convert 2nd half unwrit",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
    { .desc = "split writ extent to 3 extents and convert 2nd half to unwrit",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 3,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 2,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1 + (EXT_DATA_LEN - 2),
    .ex_len = 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
//
// ***** zeroout tests
//
// unwrit to writ splits
    { .desc = "split unwrit extent to 2 extents and convert 1st half writ (zeroout)",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split unwrit extent to 2 extents and convert 2nd half writ (zeroout)",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split unwrit extent to 3 extents and convert 2nd half writ (zeroout)",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 3,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X', .off_blk = 1, .len_blk = EXT_DATA_LEN - 2 },
    { .exp_char = 0, .off_blk = EXT_DATA_LEN - 1, .len_blk = 1 } } },
// writ to unwrit splits
    { .desc = "split writ extent to 2 extents and convert 1st half unwrit (zeroout)",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split writ extent to 2 extents and convert 2nd half unwrit (zeroout)",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split writ extent to 3 extents and convert 2nd half unwrit (zeroout)",
    .type = TEST_SPLIT_CONVERT,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 3,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 2 },
    { .exp_char = 'X',
    .off_blk = EXT_DATA_LEN - 1,
    .len_blk = 1 } } },
    };
// Tests to trigger ext4_ext_map_blocks() -> convert_initialized_extent()
    static const struct kunit_ext_test_param test_convert_initialized_params[] = {
// writ to unwrit splits
    { .desc = "split writ extent to 2 extents and convert 1st half unwrit",
    .type = TEST_CREATE_BLOCKS,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .is_unwrit_at_start = 0,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
    { .desc = "split writ extent to 2 extents and convert 2nd half unwrit",
    .type = TEST_CREATE_BLOCKS,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .is_unwrit_at_start = 0,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
    { .desc = "split writ extent to 3 extents and convert 2nd half to unwrit",
    .type = TEST_CREATE_BLOCKS,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .is_unwrit_at_start = 0,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 3,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 2,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1 + (EXT_DATA_LEN - 2),
    .ex_len = 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
// writ to unwrit splits (zeroout)
    { .desc = "split writ extent to 2 extents and convert 1st half unwrit (zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split writ extent to 2 extents and convert 2nd half unwrit (zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split writ extent to 3 extents and convert 2nd half unwrit (zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 0,
    .split_flags = EXT4_GET_BLOCKS_CONVERT_UNWRITTEN,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 3,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 2 },
    { .exp_char = 'X',
    .off_blk = EXT_DATA_LEN - 1,
    .len_blk = 1 } } },
    };
// Tests to trigger ext4_ext_map_blocks() -> ext4_ext_handle_unwritten_exntents()
    static const struct kunit_ext_test_param test_handle_unwritten_params[] = {
// unwrit to writ splits via endio path
    { .desc = "split unwrit extent to 2 extents and convert 1st half writ (endio)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
    { .desc = "split unwrit extent to 2 extents and convert 2nd half writ (endio)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 2,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
    { .desc = "split unwrit extent to 3 extents and convert 2nd half to writ (endio)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 3,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 2,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1 + (EXT_DATA_LEN - 2),
    .ex_len = 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
// unwrit to writ splits via non-endio path
    { .desc = "split unwrit extent to 2 extents and convert 1st half writ (non endio)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CREATE,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 2,
    .disable_zeroout = true,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
    { .desc = "split unwrit extent to 2 extents and convert 2nd half writ (non endio)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CREATE,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 2,
    .disable_zeroout = true,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 1,
    .is_unwrit = 0 } },
    .is_zeroout_test = 0 },
    { .desc = "split unwrit extent to 3 extents and convert 2nd half to writ (non endio)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CREATE,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 3,
    .disable_zeroout = true,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = 1,
    .is_unwrit = 1 },
    { .ex_lblk = EXT_DATA_LBLK + 1,
    .ex_len = EXT_DATA_LEN - 2,
    .is_unwrit = 0 },
    { .ex_lblk = EXT_DATA_LBLK + 1 + (EXT_DATA_LEN - 2),
    .ex_len = 1,
    .is_unwrit = 1 } },
    .is_zeroout_test = 0 },
//
// ***** zeroout tests
//
// unwrit to writ splits (endio)
    { .desc = "split unwrit extent to 2 extents and convert 1st half writ (endio, zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split unwrit extent to 2 extents and convert 2nd half writ (endio, zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split unwrit extent to 3 extents and convert 2nd half writ (endio, zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CONVERT,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 3,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 2 },
    { .exp_char = 0,
    .off_blk = EXT_DATA_LEN - 1,
    .len_blk = 1 } } },
// unwrit to writ splits (non-endio)
    { .desc = "split unwrit extent to 2 extents and convert 1st half writ (non-endio, zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CREATE,
    .split_map = { .m_lblk = EXT_DATA_LBLK, .m_len = 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 'X', .off_blk = 0, .len_blk = 1 },
    { .exp_char = 0,
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split unwrit extent to 2 extents and convert 2nd half writ (non-endio, zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CREATE,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 1 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 2,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 1 } } },
    { .desc = "split unwrit extent to 3 extents and convert 2nd half writ (non-endio, zeroout)",
    .type = TEST_CREATE_BLOCKS,
    .is_unwrit_at_start = 1,
    .split_flags = EXT4_GET_BLOCKS_CREATE,
    .split_map = { .m_lblk = EXT_DATA_LBLK + 1, .m_len = EXT_DATA_LEN - 2 },
    .nr_exp_ext = 1,
    .exp_ext_state = { { .ex_lblk = EXT_DATA_LBLK,
    .ex_len = EXT_DATA_LEN,
    .is_unwrit = 0 } },
    .is_zeroout_test = 1,
    .nr_exp_data_segs = 3,
    .exp_data_state = { { .exp_char = 0, .off_blk = 0, .len_blk = 1 },
    { .exp_char = 'X',
    .off_blk = 1,
    .len_blk = EXT_DATA_LEN - 2 },
    { .exp_char = 0,
    .off_blk = EXT_DATA_LEN - 1,
    .len_blk = 1 } } },
    };
#[no_mangle]
unsafe extern "C" fn ext_get_desc(test: *mut kunit, p: *const c_void, desc: *mut c_char) {
    static void ext_get_desc(struct kunit *test, const void *p, char *desc)
    {
    struct kunit_ext_test_param *param = (struct kunit_ext_test_param *)p;
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "%s %s\n", param.desc,
    (param.type & TEST_CREATE_BLOCKS) ? "(highlevel)" : "");
    }
#[no_mangle]
unsafe extern "C" fn test_split_convert_param_init(test: *mut kunit) -> c_int {
    static int test_split_convert_param_init(struct kunit *test)
    {
    let mut arr_size: usize = ARRAY_SIZE(test_split_convert_params);
    kunit_register_params_array(test, test_split_convert_params, arr_size,
    ext_get_desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_convert_initialized_param_init(test: *mut kunit) -> c_int {
    static int test_convert_initialized_param_init(struct kunit *test)
    {
    let mut arr_size: usize = ARRAY_SIZE(test_convert_initialized_params);
    kunit_register_params_array(test, test_convert_initialized_params,
    arr_size, ext_get_desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_handle_unwritten_init(test: *mut kunit) -> c_int {
    static int test_handle_unwritten_init(struct kunit *test)
    {
    let mut arr_size: usize = ARRAY_SIZE(test_handle_unwritten_params);
    kunit_register_params_array(test, test_handle_unwritten_params,
    arr_size, ext_get_desc);
    return 0;
    }
//
// Note that we use KUNIT_CASE_PARAM_WITH_INIT() instead of the more compact
// KUNIT_ARRAY_PARAM() because the later currently has a limitation causing the
// output parsing to be prone to error. For more context:
//
// https://lore.kernel.org/linux-kselftest/aULJpTvJDw9ctUDe@li-dc0c254c-257c-11b2-a85c-98b6c1322444.ibm.com
//
    static struct kunit_case extents_test_cases[] = {
    KUNIT_CASE_PARAM_WITH_INIT(test_split_convert, kunit_array_gen_params,
    test_split_convert_param_init, core::ptr::null_mut()),
    KUNIT_CASE_PARAM_WITH_INIT(test_split_convert, kunit_array_gen_params,
    test_convert_initialized_param_init, core::ptr::null_mut()),
    KUNIT_CASE_PARAM_WITH_INIT(test_split_convert, kunit_array_gen_params,
    test_handle_unwritten_init, core::ptr::null_mut()),
    {}
    };
    static struct kunit_suite extents_test_suite = {
    .name = "ext4_extents_test",
    .init = extents_kunit_init,
    .exit = extents_kunit_exit,
    .test_cases = extents_test_cases,
    };
    kunit_test_suites(&extents_test_suite);
    MODULE_LICENSE("GPL");
