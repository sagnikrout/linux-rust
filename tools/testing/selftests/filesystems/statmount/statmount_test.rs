//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/statmount/statmount_test.c
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
// Macro flag: #define _GNU_SOURCE

    static const char *const known_fs[] = {
    "9p", "adfs", "affs", "afs", "aio", "anon_inodefs", "apparmorfs",
    "autofs", "bcachefs", "bdev", "befs", "bfs", "binder", "binfmt_misc",
    "bpf", "btrfs", "btrfs_test_fs", "ceph", "cgroup", "cgroup2", "cifs",
    "coda", "configfs", "cpuset", "cramfs", "cxl", "dax", "debugfs",
    "devpts", "devtmpfs", "dmabuf", "drm", "ecryptfs", "efivarfs", "efs",
    "erofs", "exfat", "ext2", "ext3", "ext4", "f2fs", "functionfs",
    "fuse", "fuseblk", "fusectl", "gadgetfs", "gfs2", "gfs2meta", "hfs",
    "hfsplus", "hostfs", "hpfs", "hugetlbfs", "ibmasmfs", "iomem",
    "ipathfs", "iso9660", "jffs2", "jfs", "minix", "mqueue", "msdos",
    "nfs", "nfs4", "nfsd", "nilfs2", "nsfs", "ntfs", "ntfs3", "ocfs2",
    "ocfs2_dlmfs", "omfs", "openpromfs", "overlay", "pipefs", "proc",
    "pstore", "pvfs2", "qnx4", "qnx6", "ramfs", "resctrl", "romfs",
    "rootfs", "rpc_pipefs", "s390_hypfs", "secretmem", "securityfs",
    "selinuxfs", "smackfs", "smb3", "sockfs", "spufs", "squashfs", "sysfs",
    "sysv", "tmpfs", "tracefs", "ubifs", "udf", "ufs", "v7", "vboxsf",
    "vfat", "virtiofs", "vxfs", "xenfs", "xfs", "zonefs", core::ptr::null_mut() };
#[no_mangle]
unsafe extern "C" fn write_file(path: *const c_char, val: *const c_char) {
    static void write_file(const char *path, const char *val)
    {
    let mut fd: c_int = open(path, O_WRONLY);
    let mut len: usize = strlen(val);
    int ret;
    if (fd == -1)
    ksft_exit_fail_msg("opening %s for write: %s\n", path, strerror(errno));
    ret = write(fd, val, len);
    if (ret == -1)
    ksft_exit_fail_msg("writing to %s: %s\n", path, strerror(errno));
    if (ret != len)
    ksft_exit_fail_msg("short write to %s\n", path);
    ret = close(fd);
    if (ret == -1)
    ksft_exit_fail_msg("closing %s\n", path);
    }
#[no_mangle]
unsafe extern "C" fn get_mnt_id(name: *const c_char, path: *const c_char, mask: u64) -> u64 {
    static uint64_t get_mnt_id(const char *name, const char *path, uint64_t mask)
    {
    struct statx sx;
    int ret;
    ret = statx(AT_FDCWD, path, 0, mask, &sx);
    if (ret == -1)
    ksft_exit_fail_msg("retrieving %s mount ID for %s: %s\n",
    mask & STATX_MNT_ID_UNIQUE ? "unique" : "old",
    name, strerror(errno));
    if (!(sx.stx_mask & mask))
    ksft_exit_fail_msg("no %s mount ID available for %s\n",
    mask & STATX_MNT_ID_UNIQUE ? "unique" : "old",
    name);
    return sx.stx_mnt_id;
    }
    static char root_mntpoint[] = "/tmp/statmount_test_root.XXXXXX";
    static int orig_root;
    static uint64_t root_id, parent_id;
    static uint32_t old_root_id, old_parent_id;
    static FILE *f_mountinfo;
#[no_mangle]
unsafe extern "C" fn cleanup_namespace() {
    static void cleanup_namespace(void)
    {
    int ret;
    if (f_mountinfo)
    fclose(f_mountinfo);
    ret = fchdir(orig_root);
    if (ret == -1)
    ksft_perror("fchdir to original root");
    ret = chroot(".");
    if (ret == -1)
    ksft_perror("chroot to original root");
    umount2(root_mntpoint, MNT_DETACH);
    rmdir(root_mntpoint);
    }
#[no_mangle]
unsafe extern "C" fn setup_namespace() {
    static void setup_namespace(void)
    {
    int ret;
    char buf[32];
    let mut uid: uid_t = getuid();
    let mut gid: gid_t = getgid();
    ret = unshare(CLONE_NEWNS|CLONE_NEWUSER|CLONE_NEWPID);
    if (ret == -1)
    ksft_exit_fail_msg("unsharing mountns and userns: %s\n",
    strerror(errno));
    sprintf(buf, "0 %d 1", uid);
    write_file("/proc/self/uid_map", buf);
    write_file("/proc/self/setgroups", "deny");
    sprintf(buf, "0 %d 1", gid);
    write_file("/proc/self/gid_map", buf);
    f_mountinfo = fopen("/proc/self/mountinfo", "re");
    if (!f_mountinfo)
    ksft_exit_fail_msg("failed to open mountinfo: %s\n",
    strerror(errno));
    ret = mount("", "/", core::ptr::null_mut(), MS_REC|MS_PRIVATE, core::ptr::null_mut());
    if (ret == -1)
    ksft_exit_fail_msg("making mount tree private: %s\n",
    strerror(errno));
    if (!mkdtemp(root_mntpoint))
    ksft_exit_fail_msg("creating temporary directory %s: %s\n",
    root_mntpoint, strerror(errno));
    old_parent_id = get_mnt_id("parent", root_mntpoint, STATX_MNT_ID);
    parent_id = get_mnt_id("parent", root_mntpoint, STATX_MNT_ID_UNIQUE);
    orig_root = open("/", O_PATH);
    if (orig_root == -1)
    ksft_exit_fail_msg("opening root directory: %s",
    strerror(errno));
    atexit(cleanup_namespace);
    ret = mount(root_mntpoint, root_mntpoint, core::ptr::null_mut(), MS_BIND, core::ptr::null_mut());
    if (ret == -1)
    ksft_exit_fail_msg("mounting temp root %s: %s\n",
    root_mntpoint, strerror(errno));
    ret = chroot(root_mntpoint);
    if (ret == -1)
    ksft_exit_fail_msg("chroot to temp root %s: %s\n",
    root_mntpoint, strerror(errno));
    ret = chdir("/");
    if (ret == -1)
    ksft_exit_fail_msg("chdir to root: %s\n", strerror(errno));
    old_root_id = get_mnt_id("root", "/", STATX_MNT_ID);
    root_id = get_mnt_id("root", "/", STATX_MNT_ID_UNIQUE);
    }
#[no_mangle]
unsafe extern "C" fn setup_mount_tree(log2_num: c_int) -> c_int {
    static int setup_mount_tree(int log2_num)
    {
    int ret, i;
    ret = mount("", "/", core::ptr::null_mut(), MS_REC|MS_SHARED, core::ptr::null_mut());
    if (ret == -1) {
    ksft_test_result_fail("making mount tree shared: %s\n",
    strerror(errno));
    return -1;
    }
    for (i = 0; i < log2_num; i++) {
    ret = mount("/", "/", core::ptr::null_mut(), MS_BIND, core::ptr::null_mut());
    if (ret == -1) {
    ksft_test_result_fail("mounting submount %s: %s\n",
    root_mntpoint, strerror(errno));
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_listmount_empty_root() {
    static void test_listmount_empty_root(void)
    {
    ssize_t res;
    let mut size: c_uint = 32;
    uint64_t list[size];
    res = listmount(LSMT_ROOT, 0, 0, list, size, 0);
    if (res == -1) {
    ksft_test_result_fail("listmount: %s\n", strerror(errno));
    return;
    }
    if (res != 1) {
    ksft_test_result_fail("listmount result is %zi != 1\n", res);
    return;
    }
    if (list[0] != root_id) {
    ksft_test_result_fail("listmount ID doesn't match 0x%llx != 0x%llx\n",
    (unsigned long long) list[0],
    (unsigned long long) root_id);
    return;
    }
    ksft_test_result_pass("listmount empty root\n");
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_zero_mask() {
    static void test_statmount_zero_mask(void)
    {
    struct statmount sm;
    int ret;
    ret = statmount(root_id, 0, 0, 0, &sm, sizeof(sm), 0);
    if (ret == -1) {
    ksft_test_result_fail("statmount zero mask: %s\n",
    strerror(errno));
    return;
    }
    if (sm.size != sizeof(sm)) {
    ksft_test_result_fail("unexpected size: %u != %u\n",
    sm.size, (uint32_t) sizeof(sm));
    return;
    }
    if (sm.mask != 0) {
    ksft_test_result_fail("unexpected mask: 0x%llx != 0x0\n",
    (unsigned long long) sm.mask);
    return;
    }
    ksft_test_result_pass("statmount zero mask\n");
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_mnt_basic() {
    static void test_statmount_mnt_basic(void)
    {
    struct statmount sm;
    int ret;
    let mut mask: u64 = STATMOUNT_MNT_BASIC;
    ret = statmount(root_id, 0, 0, mask, &sm, sizeof(sm), 0);
    if (ret == -1) {
    ksft_test_result_fail("statmount mnt basic: %s\n",
    strerror(errno));
    return;
    }
    if (sm.size != sizeof(sm)) {
    ksft_test_result_fail("unexpected size: %u != %u\n",
    sm.size, (uint32_t) sizeof(sm));
    return;
    }
    if (sm.mask != mask) {
    ksft_test_result_skip("statmount mnt basic unavailable\n");
    return;
    }
    if (sm.mnt_id != root_id) {
    ksft_test_result_fail("unexpected root ID: 0x%llx != 0x%llx\n",
    (unsigned long long) sm.mnt_id,
    (unsigned long long) root_id);
    return;
    }
    if (sm.mnt_id_old != old_root_id) {
    ksft_test_result_fail("unexpected old root ID: %u != %u\n",
    sm.mnt_id_old, old_root_id);
    return;
    }
    if (sm.mnt_parent_id != parent_id) {
    ksft_test_result_fail("unexpected parent ID: 0x%llx != 0x%llx\n",
    (unsigned long long) sm.mnt_parent_id,
    (unsigned long long) parent_id);
    return;
    }
    if (sm.mnt_parent_id_old != old_parent_id) {
    ksft_test_result_fail("unexpected old parent ID: %u != %u\n",
    sm.mnt_parent_id_old, old_parent_id);
    return;
    }
    if (sm.mnt_propagation != MS_PRIVATE) {
    ksft_test_result_fail("unexpected propagation: 0x%llx\n",
    (unsigned long long) sm.mnt_propagation);
    return;
    }
    ksft_test_result_pass("statmount mnt basic\n");
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_sb_basic() {
    static void test_statmount_sb_basic(void)
    {
    struct statmount sm;
    int ret;
    let mut mask: u64 = STATMOUNT_SB_BASIC;
    struct statx sx;
    struct statfs sf;
    ret = statmount(root_id, 0, 0, mask, &sm, sizeof(sm), 0);
    if (ret == -1) {
    ksft_test_result_fail("statmount sb basic: %s\n",
    strerror(errno));
    return;
    }
    if (sm.size != sizeof(sm)) {
    ksft_test_result_fail("unexpected size: %u != %u\n",
    sm.size, (uint32_t) sizeof(sm));
    return;
    }
    if (sm.mask != mask) {
    ksft_test_result_skip("statmount sb basic unavailable\n");
    return;
    }
    ret = statx(AT_FDCWD, "/", 0, 0, &sx);
    if (ret == -1) {
    ksft_test_result_fail("stat root failed: %s\n",
    strerror(errno));
    return;
    }
    if (sm.sb_dev_major != sx.stx_dev_major ||
    sm.sb_dev_minor != sx.stx_dev_minor) {
    ksft_test_result_fail("unexpected sb dev %u:%u != %u:%u\n",
    sm.sb_dev_major, sm.sb_dev_minor,
    sx.stx_dev_major, sx.stx_dev_minor);
    return;
    }
    ret = statfs("/", &sf);
    if (ret == -1) {
    ksft_test_result_fail("statfs root failed: %s\n",
    strerror(errno));
    return;
    }
    if (sm.sb_magic != sf.f_type) {
    ksft_test_result_fail("unexpected sb magic: 0x%llx != 0x%lx\n",
    (unsigned long long) sm.sb_magic,
    sf.f_type);
    return;
    }
    ksft_test_result_pass("statmount sb basic\n");
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_mnt_point() {
    static void test_statmount_mnt_point(void)
    {
    struct statmount *sm;
    sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_POINT, 0);
    if (!sm) {
    ksft_test_result_fail("statmount mount point: %s\n",
    strerror(errno));
    return;
    }
    if (!(sm.mask & STATMOUNT_MNT_POINT)) {
    ksft_test_result_fail("missing STATMOUNT_MNT_POINT in mask\n");
    return;
    }
    if (strcmp(sm.str + sm.mnt_point, "/") != 0) {
    ksft_test_result_fail("unexpected mount point: '%s' != '/'\n",
    sm.str + sm.mnt_point);
    goto out;
    }
    ksft_test_result_pass("statmount mount point\n");
    out:
    free(sm);
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_mnt_root() {
    static void test_statmount_mnt_root(void)
    {
    struct statmount *sm;
    const char *mnt_root, *last_dir, *last_root;
    last_dir = strrchr(root_mntpoint, '/');
    assert(last_dir);
    last_dir++;
    sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_ROOT, 0);
    if (!sm) {
    ksft_test_result_fail("statmount mount root: %s\n",
    strerror(errno));
    return;
    }
    if (!(sm.mask & STATMOUNT_MNT_ROOT)) {
    ksft_test_result_fail("missing STATMOUNT_MNT_ROOT in mask\n");
    return;
    }
    mnt_root = sm.str + sm.mnt_root;
    last_root = strrchr(mnt_root, '/');
    if (last_root)
    last_root++;
    else
    last_root = mnt_root;
    if (strcmp(last_dir, last_root) != 0) {
    ksft_test_result_fail("unexpected mount root last component: '%s' != '%s'\n",
    last_root, last_dir);
    goto out;
    }
    ksft_test_result_pass("statmount mount root\n");
    out:
    free(sm);
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_fs_type() {
    static void test_statmount_fs_type(void)
    {
    struct statmount *sm;
    const char *fs_type;
    const char *const *s;
    sm = statmount_alloc(root_id, 0, STATMOUNT_FS_TYPE, 0);
    if (!sm) {
    ksft_test_result_fail("statmount fs type: %s\n",
    strerror(errno));
    return;
    }
    if (!(sm.mask & STATMOUNT_FS_TYPE)) {
    ksft_test_result_fail("missing STATMOUNT_FS_TYPE in mask\n");
    return;
    }
    fs_type = sm.str + sm.fs_type;
    for (s = known_fs; s != core::ptr::null_mut(); s++) {
    if (strcmp(fs_type, *s) == 0)
    break;
    }
    if (!s)
    ksft_print_msg("unknown filesystem type: %s\n", fs_type);
    ksft_test_result_pass("statmount fs type\n");
    free(sm);
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_mnt_opts() {
    static void test_statmount_mnt_opts(void)
    {
    struct statmount *sm;
    const char *statmount_opts;
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_BASIC | STATMOUNT_MNT_OPTS,
    0);
    if (!sm) {
    ksft_test_result_fail("statmount mnt opts: %s\n",
    strerror(errno));
    return;
    }
    if (!(sm.mask & STATMOUNT_MNT_BASIC)) {
    ksft_test_result_fail("missing STATMOUNT_MNT_BASIC in mask\n");
    return;
    }
    while (getline(&line, &len, f_mountinfo) != -1) {
    int i;
    char *p, *p2;
    unsigned int old_mnt_id;
    old_mnt_id = atoi(line);
    if (old_mnt_id != sm.mnt_id_old)
    continue;
    for (p = line, i = 0; p && i < 5; i++)
    p = strchr(p + 1, ' ');
    if (!p)
    continue;
    p2 = strchr(p + 1, ' ');
    if (!p2)
    continue;
// p2 = '\0';
    p = strchr(p2 + 1, '-');
    if (!p)
    continue;
    for (p++, i = 0; p && i < 2; i++)
    p = strchr(p + 1, ' ');
    if (!p)
    continue;
    p++;
// skip generic superblock options
    if (strncmp(p, "ro", 2) == 0)
    p += 2;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strncmp(p, _arg: "rw", 0: 2) ==) -> else {
    else if (strncmp(p, "rw", 2) == 0)
    p += 2;
    if (*p == ',')
    p++;
    if (strncmp(p, "sync", 4) == 0)
    p += 4;
    if (*p == ',')
    p++;
    if (strncmp(p, "dirsync", 7) == 0)
    p += 7;
    if (*p == ',')
    p++;
    if (strncmp(p, "lazytime", 8) == 0)
    p += 8;
    if (*p == ',')
    p++;
    p2 = strrchr(p, '\n');
    if (p2)
// p2 = '\0';
    if (sm.mask & STATMOUNT_MNT_OPTS)
    statmount_opts = sm.str + sm.mnt_opts;
    else
    statmount_opts = "";
    if (strcmp(statmount_opts, p) != 0)
    ksft_test_result_fail(
    "unexpected mount options: '%s' != '%s'\n",
    statmount_opts, p);
    else
    ksft_test_result_pass("statmount mount options\n");
    free(sm);
    free(line);
    return;
    }
    ksft_test_result_fail("didn't find mount entry\n");
    free(sm);
    free(line);
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_string(mask: u64, off: usize, name: *const c_char) {
    static void test_statmount_string(uint64_t mask, size_t off, const char *name)
    {
    struct statmount *sm;
    size_t len, shortsize, exactsize;
    uint32_t start, i;
    int ret;
    sm = statmount_alloc(root_id, 0, mask, 0);
    if (!sm) {
    ksft_test_result_fail("statmount %s: %s\n", name,
    strerror(errno));
    goto out;
    }
    if (sm.size < sizeof(*sm)) {
    ksft_test_result_fail("unexpected size: %u < %u\n",
    sm.size, (uint32_t) sizeof(*sm));
    goto out;
    }
    if (sm.mask != mask) {
    ksft_test_result_skip("statmount %s unavailable\n", name);
    goto out;
    }
    len = sm.size - sizeof(*sm);
    start = ((uint32_t *) sm)[off];
    for (i = start;; i++) {
    if (i >= len) {
    ksft_test_result_fail("string out of bounds\n");
    goto out;
    }
    if (!sm.str[i])
    break;
    }
    exactsize = sm.size;
    shortsize = sizeof(*sm) + i;
    ret = statmount(root_id, 0, 0, mask, sm, exactsize, 0);
    if (ret == -1) {
    ksft_test_result_fail("statmount exact size: %s\n",
    strerror(errno));
    goto out;
    }
    errno = 0;
    ret = statmount(root_id, 0, 0, mask, sm, shortsize, 0);
    if (ret != -1 || errno != EOVERFLOW) {
    ksft_test_result_fail("should have failed with EOVERFLOW: %s\n",
    strerror(errno));
    goto out;
    }
    ksft_test_result_pass("statmount string %s\n", name);
    out:
    free(sm);
    }
#[no_mangle]
unsafe extern "C" fn test_listmount_tree() {
    static void test_listmount_tree(void)
    {
    ssize_t res;
    let mut log2_num: c_uint = 4;
    let mut step: c_uint = 3;
    let mut size: c_uint = (1 << log2_num) + step + 1;
    size_t num, expect = 1 << log2_num;
    uint64_t list[size];
    uint64_t list2[size];
    size_t i;
    res = setup_mount_tree(log2_num);
    if (res == -1)
    return;
    num = res = listmount(LSMT_ROOT, 0, 0, list, size, 0);
    if (res == -1) {
    ksft_test_result_fail("listmount: %s\n", strerror(errno));
    return;
    }
    if (num != expect) {
    ksft_test_result_fail("listmount result is %zi != %zi\n",
    res, expect);
    return;
    }
    for (i = 0; i < size - step;) {
    res = listmount(LSMT_ROOT, 0, i ? list2[i - 1] : 0, list2 + i, step, 0);
    if (res == -1)
    ksft_test_result_fail("short listmount: %s\n",
    strerror(errno));
    i += res;
    if (res < step)
    break;
    }
    if (i != num) {
    ksft_test_result_fail("different number of entries: %zu != %zu\n",
    i, num);
    return;
    }
    for (i = 0; i < num; i++) {
    if (list2[i] != list[i]) {
    ksft_test_result_fail("different value for entry %zu: 0x%llx != 0x%llx\n",
    i,
    (unsigned long long) list2[i],
    (unsigned long long) list[i]);
    }
    }
    ksft_test_result_pass("listmount tree\n");
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_by_fd() {
    static void test_statmount_by_fd(void)
    {
    struct statmount *sm = core::ptr::null_mut();
    char tmpdir[] = "/statmount.fd.XXXXXX";
    const char root[] = "/test";
    char subdir[PATH_MAX], tmproot[PATH_MAX];
    int fd;
    if (!mkdtemp(tmpdir)) {
    ksft_perror("mkdtemp");
    return;
    }
    if (mount("statmount.test", tmpdir, "tmpfs", 0, core::ptr::null_mut())) {
    ksft_perror("mount");
    rmdir(tmpdir);
    return;
    }
    snprintf(subdir, PATH_MAX, "%s%s", tmpdir, root);
    snprintf(tmproot, PATH_MAX, "%s/%s", tmpdir, "chroot");
    if (mkdir(subdir, 0755)) {
    ksft_perror("mkdir");
    goto err_tmpdir;
    }
    if (mount(subdir, subdir, core::ptr::null_mut(), MS_BIND, 0)) {
    ksft_perror("mount");
    goto err_subdir;
    }
    if (mkdir(tmproot, 0755)) {
    ksft_perror("mkdir");
    goto err_subdir;
    }
    fd = open(subdir, O_PATH);
    if (fd < 0) {
    ksft_perror("open");
    goto err_tmproot;
    }
    if (chroot(tmproot)) {
    ksft_perror("chroot");
    goto err_fd;
    }
    sm = statmount_alloc_by_fd(fd, STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT);
    if (!sm) {
    ksft_test_result_fail("statmount by fd failed: %s\n", strerror(errno));
    goto err_chroot;
    }
    if (sm.size < sizeof(*sm)) {
    ksft_test_result_fail("unexpected size: %u < %u\n",
    sm.size, (uint32_t) sizeof(*sm));
    goto err_chroot;
    }
    if (sm.mask & STATMOUNT_MNT_POINT) {
    ksft_test_result_fail("STATMOUNT_MNT_POINT unexpectedly set in statmount\n");
    goto err_chroot;
    }
    if (!(sm.mask & STATMOUNT_MNT_ROOT)) {
    ksft_test_result_fail("STATMOUNT_MNT_ROOT not set in statmount\n");
    goto err_chroot;
    }
    if (strcmp(root, sm.str + sm.mnt_root) != 0) {
    ksft_test_result_fail("statmount returned incorrect mnt_root,"
    "statmount mnt_root: %s != %s\n",
    sm.str + sm.mnt_root, root);
    goto err_chroot;
    }
    if (chroot(".")) {
    ksft_perror("chroot");
    goto out;
    }
    free(sm);
    sm = statmount_alloc_by_fd(fd, STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT);
    if (!sm) {
    ksft_test_result_fail("statmount by fd failed: %s\n", strerror(errno));
    goto err_fd;
    }
    if (sm.size < sizeof(*sm)) {
    ksft_test_result_fail("unexpected size: %u < %u\n",
    sm.size, (uint32_t) sizeof(*sm));
    goto out;
    }
    if (!(sm.mask & STATMOUNT_MNT_POINT)) {
    ksft_test_result_fail("STATMOUNT_MNT_POINT not set in statmount\n");
    goto out;
    }
    if (!(sm.mask & STATMOUNT_MNT_ROOT)) {
    ksft_test_result_fail("STATMOUNT_MNT_ROOT not set in statmount\n");
    goto out;
    }
    if (strcmp(subdir, sm.str + sm.mnt_point) != 0) {
    ksft_test_result_fail("statmount returned incorrect mnt_point,"
    "statmount mnt_point: %s != %s\n", sm.str + sm.mnt_point, subdir);
    goto out;
    }
    if (strcmp(root, sm.str + sm.mnt_root) != 0) {
    ksft_test_result_fail("statmount returned incorrect mnt_root,"
    "statmount mnt_root: %s != %s\n", sm.str + sm.mnt_root, root);
    goto out;
    }
    ksft_test_result_pass("statmount by fd\n");
    goto out;
    err_chroot:
    chroot(".");
    out:
    free(sm);
    err_fd:
    close(fd);
    err_tmproot:
    rmdir(tmproot);
    err_subdir:
    umount2(subdir, MNT_DETACH);
    rmdir(subdir);
    err_tmpdir:
    umount2(tmpdir, MNT_DETACH);
    rmdir(tmpdir);
    }
#[no_mangle]
unsafe extern "C" fn test_statmount_by_fd_unmounted() {
    static void test_statmount_by_fd_unmounted(void)
    {
    const char root[] = "/test.unmounted";
    char tmpdir[] = "/statmount.fd.XXXXXX";
    char subdir[PATH_MAX];
    int fd;
    struct statmount *sm = core::ptr::null_mut();
    if (!mkdtemp(tmpdir)) {
    ksft_perror("mkdtemp");
    return;
    }
    if (mount("statmount.test", tmpdir, "tmpfs", 0, core::ptr::null_mut())) {
    ksft_perror("mount");
    rmdir(tmpdir);
    return;
    }
    snprintf(subdir, PATH_MAX, "%s%s", tmpdir, root);
    if (mkdir(subdir, 0755)) {
    ksft_perror("mkdir");
    goto err_tmpdir;
    }
    if (mount(subdir, subdir, 0, MS_BIND, core::ptr::null_mut())) {
    ksft_perror("mount");
    goto err_subdir;
    }
    fd = open(subdir, O_PATH);
    if (fd < 0) {
    ksft_perror("open");
    goto err_subdir;
    }
    if (umount2(tmpdir, MNT_DETACH)) {
    ksft_perror("umount2");
    goto err_fd;
    }
    sm = statmount_alloc_by_fd(fd, STATMOUNT_MNT_POINT | STATMOUNT_MNT_ROOT);
    if (!sm) {
    ksft_test_result_fail("statmount by fd unmounted: %s\n",
    strerror(errno));
    goto err_sm;
    }
    if (sm.size < sizeof(*sm)) {
    ksft_test_result_fail("unexpected size: %u < %u\n",
    sm.size, (uint32_t) sizeof(*sm));
    goto err_sm;
    }
    if (sm.mask & STATMOUNT_MNT_POINT) {
    ksft_test_result_fail("STATMOUNT_MNT_POINT unexpectedly set in mask\n");
    goto err_sm;
    }
    if (!(sm.mask & STATMOUNT_MNT_ROOT)) {
    ksft_test_result_fail("STATMOUNT_MNT_ROOT not set in mask\n");
    goto err_sm;
    }
    if (strcmp(sm.str + sm.mnt_root, root) != 0) {
    ksft_test_result_fail("statmount returned incorrect mnt_root,"
    "statmount mnt_root: %s != %s\n",
    sm.str + sm.mnt_root, root);
    goto err_sm;
    }
    ksft_test_result_pass("statmount by fd on unmounted mount\n");
    err_sm:
    free(sm);
    err_fd:
    close(fd);
    err_subdir:
    umount2(subdir, MNT_DETACH);
    rmdir(subdir);
    err_tmpdir:
    umount2(tmpdir, MNT_DETACH);
    rmdir(tmpdir);
    }

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int ret;
    uint64_t all_mask = STATMOUNT_SB_BASIC | STATMOUNT_MNT_BASIC |
    STATMOUNT_PROPAGATE_FROM | STATMOUNT_MNT_ROOT |
    STATMOUNT_MNT_POINT | STATMOUNT_FS_TYPE | STATMOUNT_MNT_NS_ID;
    ksft_print_header();
    ret = statmount(0, 0, 0, 0, core::ptr::null_mut(), 0, 0);
    assert(ret == -1);
    if (errno == ENOSYS)
    ksft_exit_skip("statmount() syscall not supported\n");
    setup_namespace();
    ksft_set_plan(17);
    test_listmount_empty_root();
    test_statmount_zero_mask();
    test_statmount_mnt_basic();
    test_statmount_sb_basic();
    test_statmount_mnt_root();
    test_statmount_mnt_point();
    test_statmount_fs_type();
    test_statmount_mnt_opts();
    test_statmount_string(STATMOUNT_MNT_ROOT, str_off(mnt_root), "mount root");
    test_statmount_string(STATMOUNT_MNT_POINT, str_off(mnt_point), "mount point");
    test_statmount_string(STATMOUNT_FS_TYPE, str_off(fs_type), "fs type");
    test_statmount_string(all_mask, str_off(mnt_root), "mount root & all");
    test_statmount_string(all_mask, str_off(mnt_point), "mount point & all");
    test_statmount_string(all_mask, str_off(fs_type), "fs type & all");
    test_listmount_tree();
    test_statmount_by_fd_unmounted();
    test_statmount_by_fd();
    if (ksft_get_fail_cnt() + ksft_get_error_cnt() > 0)
    ksft_exit_fail();
    else
    ksft_exit_pass();
    }
