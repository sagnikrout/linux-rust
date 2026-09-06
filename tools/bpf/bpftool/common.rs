//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/common.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.

// Macro flag: #define _GNU_SOURCE

pub const BPF_FS_MAGIC: c_uint = 0xcafe4a11;

#[no_mangle]
pub unsafe extern "C" fn p_err(fmt: *const c_char, ...) {
    void p_err(const char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    if (json_output) {
    jsonw_start_object(json_wtr);
    jsonw_name(json_wtr, "error");
    jsonw_vprintf_enquote(json_wtr, fmt, ap);
    jsonw_end_object(json_wtr);
    } else {
    fprintf(stderr, "Error: ");
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    }
    va_end(ap);
    }
#[no_mangle]
pub unsafe extern "C" fn p_info(fmt: *const c_char, ...) {
    void p_info(const char *fmt, ...)
    {
    va_list ap;
    if (json_output)
    return;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    va_end(ap);
    }
#[no_mangle]
unsafe extern "C" fn is_bpffs(path: *const c_char) -> bool {
    static bool is_bpffs(const char *path)
    {
    struct statfs st_fs;
    if (statfs(path, &st_fs) < 0)
    return false;
    return (unsigned long)st_fs.f_type == BPF_FS_MAGIC;
    }
// Probe whether kernel switched from memlock-based (RLIMIT_MEMLOCK) to
// memcg-based memory accounting for BPF maps and programs. This was done in
// commit 97306be45fbe ("Merge branch 'switch to memcg-based memory
// accounting'"), in Linux 5.11.
//
// Libbpf also offers to probe for memcg-based accounting vs rlimit, but does
// so by checking for the availability of a given BPF helper and this has
// failed on some kernels with backports in the past, see commit 6b4384ff1088
// ("Revert "bpftool: Use libbpf 1.0 API mode instead of RLIMIT_MEMLOCK"").
// Instead, we can probe by lowering the process-based rlimit to 0, trying to
// load a BPF object, and resetting the rlimit. If the load succeeds then
// memcg-based accounting is supported.
//
// This would be too dangerous to do in the library, because multithreaded
// applications might attempt to load items while the rlimit is at 0. Given
// that bpftool is single-threaded, this is fine to do here.
//
#[no_mangle]
unsafe extern "C" fn known_to_need_rlimit() -> bool {
    static bool known_to_need_rlimit(void)
    {
    struct rlimit rlim_init, rlim_cur_zero = {};
    struct bpf_insn insns[] = {
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    let mut insn_cnt: usize = ARRAY_SIZE(insns);
    union bpf_attr attr;
    int prog_fd, err;
    memset(&attr, 0, sizeof(attr));
    attr.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    attr.insns = ptr_to_u64(insns);
    attr.insn_cnt = insn_cnt;
    attr.license = ptr_to_u64("GPL");
    if (getrlimit(RLIMIT_MEMLOCK, &rlim_init))
    return false;
// Drop the soft limit to zero. We maintain the hard limit to its
// current value, because lowering it would be a permanent operation
// for unprivileged users.
//
    rlim_cur_zero.rlim_max = rlim_init.rlim_max;
    if (setrlimit(RLIMIT_MEMLOCK, &rlim_cur_zero))
    return false;
// Do not use bpf_prog_load() from libbpf here, because it calls
// bump_rlimit_memlock(), interfering with the current probe.
//
    prog_fd = syscall(__NR_bpf, BPF_PROG_LOAD, &attr, sizeof(attr));
    err = errno;
// reset soft rlimit to its initial value
    setrlimit(RLIMIT_MEMLOCK, &rlim_init);
    if (prog_fd < 0)
    let mut err: return = = EPERM;
    close(prog_fd);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn set_max_rlimit() {
    void set_max_rlimit(void)
    {
    let mut rinf: rlimit = { RLIM_INFINITY, RLIM_INFINITY };
    if (known_to_need_rlimit())
    setrlimit(RLIMIT_MEMLOCK, &rinf);
    }
    static int
    mnt_fs(const char *target, const char *type, char *buff, size_t bufflen)
    {
    let mut bind_done: bool = false;
    while (mount("", target, "none", MS_PRIVATE | MS_REC, core::ptr::null_mut())) {
    if (errno != EINVAL || bind_done) {
    snprintf(buff, bufflen,
    "mount --make-private %s failed: %s",
    target, strerror(errno));
    return -1;
    }
    if (mount(target, target, "none", MS_BIND, core::ptr::null_mut())) {
    snprintf(buff, bufflen,
    "mount --bind %s %s failed: %s",
    target, target, strerror(errno));
    return -1;
    }
    bind_done = true;
    }
    if (mount(type, target, type, 0, "mode=0700")) {
    snprintf(buff, bufflen, "mount -t %s %s %s failed: %s",
    type, type, target, strerror(errno));
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mount_tracefs(target: *const c_char) -> c_int {
    int mount_tracefs(const char *target)
    {
    char err_str[ERR_MAX_LEN];
    int err;
    err = mnt_fs(target, "tracefs", err_str, ERR_MAX_LEN);
    if (err) {
    err_str[ERR_MAX_LEN - 1] = '\0';
    p_err("can't mount tracefs: %s", err_str);
    }
    return err;
    }
    int open_obj_pinned(const char *path, bool quiet,
    const struct bpf_obj_get_opts *opts)
    {
    char *pname;
    let mut fd: c_int = -1;
    pname = strdup(path);
    if (!pname) {
    if (!quiet)
    p_err("mem alloc failed");
    goto out_ret;
    }
    fd = bpf_obj_get_opts(pname, opts);
    if (fd < 0) {
    if (!quiet)
    p_err("bpf obj get (%s): %s", pname,
    errno == EACCES && !is_bpffs(dirname(pname)) ?
    "directory not in bpf file system (bpffs)" :
    strerror(errno));
    goto out_free;
    }
    out_free:
    free(pname);
    out_ret:
    return fd;
    }
    int open_obj_pinned_any(const char *path, enum bpf_obj_type exp_type,
    const struct bpf_obj_get_opts *opts)
    {
    enum bpf_obj_type type;
    int fd;
    fd = open_obj_pinned(path, false, opts);
    if (fd < 0)
    return -1;
    type = get_fd_type(fd);
    if (type < 0) {
    close(fd);
    return type;
    }
    if (type != exp_type) {
    p_err("incorrect object type: %s", get_fd_type_name(type));
    close(fd);
    return -1;
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn create_and_mount_bpffs_dir(dir_name: *const c_char) -> c_int {
    int create_and_mount_bpffs_dir(const char *dir_name)
    {
    char err_str[ERR_MAX_LEN];
    bool dir_exists;
    let mut err: c_int = 0;
    if (is_bpffs(dir_name))
    return err;
    dir_exists = access(dir_name, F_OK) == 0;
    if (!dir_exists) {
    char *temp_name;
    char *parent_name;
    temp_name = strdup(dir_name);
    if (!temp_name) {
    p_err("mem alloc failed");
    return -1;
    }
    parent_name = dirname(temp_name);
    if (is_bpffs(parent_name)) {
// nothing to do if already mounted
    free(temp_name);
    return err;
    }
    if (access(parent_name, F_OK) == -1) {
    p_err("can't create dir '%s' to pin BPF object: parent dir '%s' doesn't exist",
    dir_name, parent_name);
    free(temp_name);
    return -1;
    }
    free(temp_name);
    }
    if (block_mount) {
    p_err("no BPF file system found, not mounting it due to --nomount option");
    return -1;
    }
    if (!dir_exists) {
    err = mkdir(dir_name, S_IRWXU);
    if (err) {
    p_err("failed to create dir '%s': %s", dir_name, strerror(errno));
    return err;
    }
    }
    err = mnt_fs(dir_name, "bpf", err_str, ERR_MAX_LEN);
    if (err) {
    err_str[ERR_MAX_LEN - 1] = '\0';
    p_err("can't mount BPF file system on given dir '%s': %s",
    dir_name, err_str);
    if (!dir_exists)
    rmdir(dir_name);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mount_bpffs_for_file(file_name: *const c_char) -> c_int {
    int mount_bpffs_for_file(const char *file_name)
    {
    char err_str[ERR_MAX_LEN];
    char *temp_name;
    char *dir;
    let mut err: c_int = 0;
    if (access(file_name, F_OK) != -1) {
    p_err("can't pin BPF object: path '%s' already exists", file_name);
    return -1;
    }
    temp_name = strdup(file_name);
    if (!temp_name) {
    p_err("mem alloc failed");
    return -1;
    }
    dir = dirname(temp_name);
    if (is_bpffs(dir))
// nothing to do if already mounted
    goto out_free;
    if (access(dir, F_OK) == -1) {
    p_err("can't pin BPF object: dir '%s' doesn't exist", dir);
    err = -1;
    goto out_free;
    }
    if (block_mount) {
    p_err("no BPF file system found, not mounting it due to --nomount option");
    err = -1;
    goto out_free;
    }
    err = mnt_fs(dir, "bpf", err_str, ERR_MAX_LEN);
    if (err) {
    err_str[ERR_MAX_LEN - 1] = '\0';
    p_err("can't mount BPF file system to pin the object '%s': %s",
    file_name, err_str);
    }
    out_free:
    free(temp_name);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn do_pin_fd(fd: c_int, name: *const c_char) -> c_int {
    int do_pin_fd(int fd, const char *name)
    {
    int err;
    err = mount_bpffs_for_file(name);
    if (err)
    return err;
    err = bpf_obj_pin(fd, name);
    if (err)
    p_err("can't pin the object (%s): %s", name, strerror(errno));
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn do_pin_any(argc: c_int, argv: *mut c_char, : *mut *mut int (get_fd)(int, ): *mut c_char) -> c_int {
    int do_pin_any(int argc, char **argv, int (*get_fd)(int *, char ***))
    {
    int err;
    int fd;
    if (!REQ_ARGS(3))
    return -EINVAL;
    fd = get_fd(&argc, &argv);
    if (fd < 0)
    return fd;
    err = do_pin_fd(fd, *argv);
    close(fd);
    return err;
    }
    const char *get_fd_type_name(enum bpf_obj_type type)
    {
    static const char * const names[] = {
    [BPF_OBJ_UNKNOWN]	= "unknown",
    [BPF_OBJ_PROG]		= "prog",
    [BPF_OBJ_MAP]		= "map",
    [BPF_OBJ_LINK]		= "link",
    };
    if (type < 0 || type >= ARRAY_SIZE(names) || !names[type])
    return names[BPF_OBJ_UNKNOWN];
    return names[type];
    }
    void get_prog_full_name(const struct bpf_prog_info *prog_info, int prog_fd,
    char *name_buff, size_t buff_len)
    {
    const char *prog_name = prog_info.name;
    const struct btf_type *func_type;
    let mut finfo: bpf_func_info = {};
    let mut info: bpf_prog_info = {};
    let mut info_len: __u32 = sizeof(info);
    struct btf *prog_btf = core::ptr::null_mut();
    if (buff_len <= BPF_OBJ_NAME_LEN ||
    strlen(prog_info.name) < BPF_OBJ_NAME_LEN - 1)
    goto copy_name;
    if (!prog_info.btf_id || prog_info.nr_func_info == 0)
    goto copy_name;
    info.nr_func_info = 1;
    info.func_info_rec_size = prog_info.func_info_rec_size;
    if (info.func_info_rec_size > sizeof(finfo))
    info.func_info_rec_size = sizeof(finfo);
    info.func_info = ptr_to_u64(&finfo);
    if (bpf_prog_get_info_by_fd(prog_fd, &info, &info_len))
    goto copy_name;
    prog_btf = btf__load_from_kernel_by_id(info.btf_id);
    if (!prog_btf)
    goto copy_name;
    func_type = btf__type_by_id(prog_btf, finfo.type_id);
    if (!func_type || !btf_is_func(func_type))
    goto copy_name;
    prog_name = btf__name_by_offset(prog_btf, func_type.name_off);
    copy_name:
    snprintf(name_buff, buff_len, "%s", prog_name);
    if (prog_btf)
    btf__free(prog_btf);
    }
#[no_mangle]
pub unsafe extern "C" fn get_fd_type(fd: c_int) -> c_int {
    int get_fd_type(int fd)
    {
    char path[PATH_MAX];
    char buf[512];
    ssize_t n;
    snprintf(path, sizeof(path), "/proc/self/fd/%d", fd);
    n = readlink(path, buf, sizeof(buf));
    if (n < 0) {
    p_err("can't read link type: %s", strerror(errno));
    return -1;
    }
    if (n == sizeof(buf)) {
    p_err("can't read link type: path too long!");
    return -1;
    }
    buf[n] = '\0';
    if (strstr(buf, "bpf-map"))
    return BPF_OBJ_MAP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, _arg: "bpf-prog")) -> else {
    else if (strstr(buf, "bpf-prog"))
    return BPF_OBJ_PROG;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, _arg: "bpf-link")) -> else {
    else if (strstr(buf, "bpf-link"))
    return BPF_OBJ_LINK;
    return BPF_OBJ_UNKNOWN;
    }
    char *get_fdinfo(int fd, const char *key)
    {
    char path[PATH_MAX];
    char *line = core::ptr::null_mut();
    let mut line_n: usize = 0;
    ssize_t n;
    FILE *fdi;
    snprintf(path, sizeof(path), "/proc/self/fdinfo/%d", fd);
    fdi = fopen(path, "r");
    if (!fdi)
    return core::ptr::null_mut();
    while ((n = getline(&line, &line_n, fdi)) > 0) {
    char *value;
    int len;
    if (!strstr(line, key))
    continue;
    fclose(fdi);
    value = strchr(line, '\t');
    if (!value || !value[1]) {
    free(line);
    return core::ptr::null_mut();
    }
    value++;
    len = strlen(value);
    memmove(line, value, len);
    line[len - 1] = '\0';
    return line;
    }
    free(line);
    fclose(fdi);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn print_data_json(data: *mut u8, len: usize) {
    void print_data_json(uint8_t *data, size_t len)
    {
    unsigned int i;
    jsonw_start_array(json_wtr);
    for (i = 0; i < len; i++)
    jsonw_printf(json_wtr, "%d", data[i]);
    jsonw_end_array(json_wtr);
    }
#[no_mangle]
pub unsafe extern "C" fn print_hex_data_json(data: *mut u8, len: usize) {
    void print_hex_data_json(uint8_t *data, size_t len)
    {
    unsigned int i;
    jsonw_start_array(json_wtr);
    for (i = 0; i < len; i++)
    jsonw_printf(json_wtr, "\"0x%02hhx\"", data[i]);
    jsonw_end_array(json_wtr);
    }
// extra params for nftw cb
    static struct hashmap *build_fn_table;
    static enum bpf_obj_type build_fn_type;
    static int do_build_table_cb(const char *fpath, const struct stat *sb,
    int typeflag, struct FTW *ftwbuf)
    {
    struct bpf_prog_info pinned_info;
    let mut len: __u32 = sizeof(pinned_info);
    enum bpf_obj_type objtype;
    int fd, err = 0;
    char *path;
    if (typeflag != FTW_F)
    goto out_ret;
    fd = open_obj_pinned(fpath, true, core::ptr::null_mut());
    if (fd < 0)
    goto out_ret;
    objtype = get_fd_type(fd);
    if (objtype != build_fn_type)
    goto out_close;
    memset(&pinned_info, 0, sizeof(pinned_info));
    if (bpf_prog_get_info_by_fd(fd, &pinned_info, &len))
    goto out_close;
    path = strdup(fpath);
    if (!path) {
    err = -1;
    goto out_close;
    }
    err = hashmap__append(build_fn_table, pinned_info.id, path);
    if (err) {
    p_err("failed to append entry to hashmap for ID %u, path '%s': %s",
    pinned_info.id, path, strerror(errno));
    free(path);
    goto out_close;
    }
    out_close:
    close(fd);
    out_ret:
    return err;
    }
    int build_pinned_obj_table(struct hashmap *tab,
    enum bpf_obj_type type)
    {
    struct mntent *mntent = core::ptr::null_mut();
    FILE *mntfile = core::ptr::null_mut();
    let mut flags: c_int = FTW_PHYS;
    let mut nopenfd: c_int = 16;
    let mut err: c_int = 0;
    mntfile = setmntent("/proc/mounts", "r");
    if (!mntfile)
    return -1;
    build_fn_table = tab;
    build_fn_type = type;
    while ((mntent = getmntent(mntfile))) {
    char *path = mntent.mnt_dir;
    if (strncmp(mntent.mnt_type, "bpf", 3) != 0)
    continue;
    err = nftw(path, do_build_table_cb, nopenfd, flags);
    if (err)
    break;
    }
    fclose(mntfile);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn delete_pinned_obj_table(map: *mut hashmap) {
    void delete_pinned_obj_table(struct hashmap *map)
    {
    struct hashmap_entry *entry;
    size_t bkt;
    if (!map)
    return;
    hashmap__for_each_entry(map, entry, bkt)
    free(entry.pvalue);
    hashmap__free(map);
    }
#[no_mangle]
pub unsafe extern "C" fn get_page_size() -> c_uint {
    unsigned int get_page_size(void)
    {
    static int result;
    if (!result)
    result = getpagesize();
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn get_possible_cpus() -> c_uint {
    unsigned int get_possible_cpus(void)
    {
    let mut cpus: c_int = libbpf_num_possible_cpus();
    if (cpus < 0) {
    p_err("Can't get # of possible cpus: %s", strerror(-cpus));
    exit(-1);
    }
    return cpus;
    }
    static char *
    ifindex_to_name_ns(__u32 ifindex, __u32 ns_dev, __u32 ns_ino, char *buf)
    {
    struct stat st;
    int err;
    err = stat("/proc/self/ns/net", &st);
    if (err) {
    p_err("Can't stat /proc/self: %s", strerror(errno));
    return core::ptr::null_mut();
    }
    if (st.st_dev != ns_dev || st.st_ino != ns_ino)
    return core::ptr::null_mut();
    return if_indextoname(ifindex, buf);
    }
#[no_mangle]
unsafe extern "C" fn read_sysfs_hex_int(path: *mut c_char) -> c_int {
    static int read_sysfs_hex_int(char *path)
    {
    char vendor_id_buf[8];
    int len;
    int fd;
    fd = open(path, O_RDONLY);
    if (fd < 0) {
    p_err("Can't open %s: %s", path, strerror(errno));
    return -1;
    }
    len = read(fd, vendor_id_buf, sizeof(vendor_id_buf));
    close(fd);
    if (len < 0) {
    p_err("Can't read %s: %s", path, strerror(errno));
    return -1;
    }
    if (len >= (int)sizeof(vendor_id_buf)) {
    p_err("Value in %s too long", path);
    return -1;
    }
    vendor_id_buf[len] = 0;
    return strtol(vendor_id_buf, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn read_sysfs_netdev_hex_int(devname: *mut c_char, entry_name: *const c_char) -> c_int {
    static int read_sysfs_netdev_hex_int(char *devname, const char *entry_name)
    {
    char full_path[64];
    snprintf(full_path, sizeof(full_path), "/sys/class/net/%s/device/%s",
    devname, entry_name);
    return read_sysfs_hex_int(full_path);
    }
    const char *
    ifindex_to_arch(__u32 ifindex, __u64 ns_dev, __u64 ns_ino, const char **opt)
    {
    __maybe_unused int device_id;
    char devname[IF_NAMESIZE];
    int vendor_id;
    if (!ifindex_to_name_ns(ifindex, ns_dev, ns_ino, devname)) {
    p_err("Can't get net device name for ifindex %u: %s", ifindex,
    strerror(errno));
    return core::ptr::null_mut();
    }
    vendor_id = read_sysfs_netdev_hex_int(devname, "vendor");
    if (vendor_id < 0) {
    p_err("Can't get device vendor id for %s", devname);
    return core::ptr::null_mut();
    }
    switch (vendor_id) {

    case 0x19ee:
    device_id = read_sysfs_netdev_hex_int(devname, "device");
    if (device_id != 0x4000 &&
    device_id != 0x6000 &&
    device_id != 0x6003)
    p_info("Unknown NFP device ID, assuming it is NFP-6xxx arch");
// opt = "ctx4";
    return "NFP-6xxx";

// No NFP support in LLVM, we have no valid triple to return.
    default:
    p_err("Can't get arch name for device vendor id 0x%04x",
    (unsigned int)vendor_id);
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_dev_plain(ifindex: __u32, ns_dev: __u64, ns_inode: __u64) {
    void print_dev_plain(__u32 ifindex, __u64 ns_dev, __u64 ns_inode)
    {
    char name[IF_NAMESIZE];
    if (!ifindex)
    return;
    printf("  offloaded_to ");
    if (ifindex_to_name_ns(ifindex, ns_dev, ns_inode, name))
    printf("%s", name);
    else
    printf("ifindex %u ns_dev %llu ns_ino %llu",
    ifindex, ns_dev, ns_inode);
    }
#[no_mangle]
pub unsafe extern "C" fn print_dev_json(ifindex: __u32, ns_dev: __u64, ns_inode: __u64) {
    void print_dev_json(__u32 ifindex, __u64 ns_dev, __u64 ns_inode)
    {
    char name[IF_NAMESIZE];
    if (!ifindex)
    return;
    jsonw_name(json_wtr, "dev");
    jsonw_start_object(json_wtr);
    jsonw_uint_field(json_wtr, "ifindex", ifindex);
    jsonw_uint_field(json_wtr, "ns_dev", ns_dev);
    jsonw_uint_field(json_wtr, "ns_inode", ns_inode);
    if (ifindex_to_name_ns(ifindex, ns_dev, ns_inode, name))
    jsonw_string_field(json_wtr, "ifname", name);
    jsonw_end_object(json_wtr);
    }
#[no_mangle]
pub unsafe extern "C" fn parse_u32_arg(argc: *mut c_int, argv: *mut c_char, val: *mut __u32, what: *const c_char) -> c_int {
    int parse_u32_arg(int *argc, char ***argv, __u32 *val, const char *what)
    {
    char *endptr;
    NEXT_ARGP();
    if (*val) {
    p_err("%s already specified", what);
    return -1;
    }
// val = strtoul(**argv, &endptr, 0);
    if (*endptr) {
    p_err("can't parse %s as %s", **argv, what);
    return -1;
    }
    NEXT_ARGP();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __printf(_arg: 2, _arg: 0) -> c_int {
    int __printf(2, 0)
    print_all_levels(__maybe_unused enum libbpf_print_level level,
    const char *format, va_list args)
    {
    return vfprintf(stderr, format, args);
    }
#[no_mangle]
unsafe extern "C" fn prog_fd_by_nametag(nametag: *mut c_void, fds: *mut c_int, tag: bool) -> c_int {
    static int prog_fd_by_nametag(void *nametag, int **fds, bool tag)
    {
    char prog_name[MAX_PROG_FULL_NAME];
    let mut id: c_uint = 0;
    int fd, nb_fds = 0;
    void *tmp;
    int err;
    while (true) {
    let mut info: bpf_prog_info = {};
    let mut len: __u32 = sizeof(info);
    err = bpf_prog_get_next_id(id, &id);
    if (err) {
    if (errno != ENOENT) {
    p_err("%s", strerror(errno));
    goto err_close_fds;
    }
    return nb_fds;
    }
    fd = bpf_prog_get_fd_by_id(id);
    if (fd < 0) {
    if (errno == ENOENT)
    continue;
    p_err("can't get prog by id (%u): %s",
    id, strerror(errno));
    goto err_close_fds;
    }
    err = bpf_prog_get_info_by_fd(fd, &info, &len);
    if (err) {
    p_err("can't get prog info (%u): %s",
    id, strerror(errno));
    goto err_close_fd;
    }
    if (tag && memcmp(nametag, info.tag, BPF_TAG_SIZE)) {
    close(fd);
    continue;
    }
    if (!tag) {
    get_prog_full_name(&info, fd, prog_name,
    sizeof(prog_name));
    if (strncmp(nametag, prog_name, sizeof(prog_name))) {
    close(fd);
    continue;
    }
    }
    if (nb_fds > 0) {
    tmp = realloc(*fds, (nb_fds + 1) * sizeof(int));
    if (!tmp) {
    p_err("failed to realloc");
    goto err_close_fd;
    }
// fds = tmp;
    }
    (*fds)[nb_fds++] = fd;
    }
    err_close_fd:
    close(fd);
    err_close_fds:
    while (--nb_fds >= 0)
    close((*fds)[nb_fds]);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn prog_parse_fds(argc: *mut c_int, argv: *mut c_char, fds: *mut c_int) -> c_int {
    int prog_parse_fds(int *argc, char ***argv, int **fds)
    {
    if (is_prefix(**argv, "id")) {
    unsigned int id;
    char *endptr;
    NEXT_ARGP();
    id = strtoul(**argv, &endptr, 0);
    if (*endptr) {
    p_err("can't parse %s as ID", **argv);
    return -1;
    }
    NEXT_ARGP();
    (*fds)[0] = bpf_prog_get_fd_by_id(id);
    if ((*fds)[0] < 0) {
    p_err("get by id (%u): %s", id, strerror(errno));
    return -1;
    }
    return 1;
    } else if (is_prefix(**argv, "tag")) {
    unsigned char tag[BPF_TAG_SIZE];
    NEXT_ARGP();
    if (sscanf(**argv, BPF_TAG_FMT, tag, tag + 1, tag + 2,
    tag + 3, tag + 4, tag + 5, tag + 6, tag + 7)
    != BPF_TAG_SIZE) {
    p_err("can't parse tag");
    return -1;
    }
    NEXT_ARGP();
    return prog_fd_by_nametag(tag, fds, true);
    } else if (is_prefix(**argv, "name")) {
    char *name;
    NEXT_ARGP();
    name = **argv;
    if (strlen(name) > MAX_PROG_FULL_NAME - 1) {
    p_err("can't parse name");
    return -1;
    }
    NEXT_ARGP();
    return prog_fd_by_nametag(name, fds, false);
    } else if (is_prefix(**argv, "pinned")) {
    char *path;
    NEXT_ARGP();
    path = **argv;
    NEXT_ARGP();
    (*fds)[0] = open_obj_pinned_any(path, BPF_OBJ_PROG, core::ptr::null_mut());
    if ((*fds)[0] < 0)
    return -1;
    return 1;
    }
    p_err("expected 'id', 'tag', 'name' or 'pinned', got: '%s'?", **argv);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn prog_parse_fd(argc: *mut c_int, argv: *mut c_char) -> c_int {
    int prog_parse_fd(int *argc, char ***argv)
    {
    int *fds = core::ptr::null_mut();
    int nb_fds, fd;
    fds = malloc(sizeof(int));
    if (!fds) {
    p_err("mem alloc failed");
    return -1;
    }
    nb_fds = prog_parse_fds(argc, argv, &fds);
    if (nb_fds != 1) {
    if (nb_fds > 1) {
    p_err("several programs match this handle");
    while (nb_fds--)
    close(fds[nb_fds]);
    }
    fd = -1;
    goto exit_free;
    }
    fd = fds[0];
    exit_free:
    free(fds);
    return fd;
    }
    static int map_fd_by_name(char *name, int **fds,
    const struct bpf_get_fd_by_id_opts *opts)
    {
    let mut id: c_uint = 0;
    int fd, nb_fds = 0;
    void *tmp;
    int err;
    while (true) {
    LIBBPF_OPTS(bpf_get_fd_by_id_opts, opts_ro);
    let mut info: bpf_map_info = {};
    let mut len: __u32 = sizeof(info);
    err = bpf_map_get_next_id(id, &id);
    if (err) {
    if (errno != ENOENT) {
    p_err("%s", strerror(errno));
    goto err_close_fds;
    }
    return nb_fds;
    }
// Request a read-only fd to query the map info
    opts_ro.open_flags = BPF_F_RDONLY;
    fd = bpf_map_get_fd_by_id_opts(id, &opts_ro);
    if (fd < 0) {
    if (errno == ENOENT)
    continue;
    p_err("can't get map by id (%u): %s",
    id, strerror(errno));
    goto err_close_fds;
    }
    err = bpf_map_get_info_by_fd(fd, &info, &len);
    if (err) {
    p_err("can't get map info (%u): %s",
    id, strerror(errno));
    goto err_close_fd;
    }
    if (strncmp(name, info.name, BPF_OBJ_NAME_LEN)) {
    close(fd);
    continue;
    }
// Get an fd with the requested options, if they differ
// from the read-only options used to get the fd above.
//
    if (memcmp(opts, &opts_ro, sizeof(opts_ro))) {
    close(fd);
    fd = bpf_map_get_fd_by_id_opts(id, opts);
    if (fd < 0) {
    p_err("can't get map by id (%u): %s", id,
    strerror(errno));
    goto err_close_fds;
    }
    }
    if (nb_fds > 0) {
    tmp = realloc(*fds, (nb_fds + 1) * sizeof(int));
    if (!tmp) {
    p_err("failed to realloc");
    goto err_close_fd;
    }
// fds = tmp;
    }
    (*fds)[nb_fds++] = fd;
    }
    err_close_fd:
    close(fd);
    err_close_fds:
    while (--nb_fds >= 0)
    close((*fds)[nb_fds]);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn map_parse_fds(argc: *mut c_int, argv: *mut c_char, fds: *mut c_int, open_flags: __u32) -> c_int {
    int map_parse_fds(int *argc, char ***argv, int **fds, __u32 open_flags)
    {
    LIBBPF_OPTS(bpf_get_fd_by_id_opts, opts);
    assert((open_flags & ~BPF_F_RDONLY) == 0);
    opts.open_flags = open_flags;
    if (is_prefix(**argv, "id")) {
    unsigned int id;
    char *endptr;
    NEXT_ARGP();
    id = strtoul(**argv, &endptr, 0);
    if (*endptr) {
    p_err("can't parse %s as ID", **argv);
    return -1;
    }
    NEXT_ARGP();
    (*fds)[0] = bpf_map_get_fd_by_id_opts(id, &opts);
    if ((*fds)[0] < 0) {
    p_err("get map by id (%u): %s", id, strerror(errno));
    return -1;
    }
    return 1;
    } else if (is_prefix(**argv, "name")) {
    char *name;
    NEXT_ARGP();
    name = **argv;
    if (strlen(name) > BPF_OBJ_NAME_LEN - 1) {
    p_err("can't parse name");
    return -1;
    }
    NEXT_ARGP();
    return map_fd_by_name(name, fds, &opts);
    } else if (is_prefix(**argv, "pinned")) {
    char *path;
    LIBBPF_OPTS(bpf_obj_get_opts, get_opts);
    get_opts.file_flags = open_flags;
    NEXT_ARGP();
    path = **argv;
    NEXT_ARGP();
    (*fds)[0] = open_obj_pinned_any(path, BPF_OBJ_MAP, &get_opts);
    if ((*fds)[0] < 0)
    return -1;
    return 1;
    }
    p_err("expected 'id', 'name' or 'pinned', got: '%s'?", **argv);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn map_parse_fd(argc: *mut c_int, argv: *mut c_char, open_flags: __u32) -> c_int {
    int map_parse_fd(int *argc, char ***argv, __u32 open_flags)
    {
    int *fds = core::ptr::null_mut();
    int nb_fds, fd;
    fds = malloc(sizeof(int));
    if (!fds) {
    p_err("mem alloc failed");
    return -1;
    }
    nb_fds = map_parse_fds(argc, argv, &fds, open_flags);
    if (nb_fds != 1) {
    if (nb_fds > 1) {
    p_err("several maps match this handle");
    while (nb_fds--)
    close(fds[nb_fds]);
    }
    fd = -1;
    goto exit_free;
    }
    fd = fds[0];
    exit_free:
    free(fds);
    return fd;
    }
    int map_parse_fd_and_info(int *argc, char ***argv, struct bpf_map_info *info,
    __u32 *info_len, __u32 open_flags)
    {
    int err;
    int fd;
    fd = map_parse_fd(argc, argv, open_flags);
    if (fd < 0)
    return -1;
    err = bpf_map_get_info_by_fd(fd, info, info_len);
    if (err) {
    p_err("can't get map info: %s", strerror(errno));
    close(fd);
    return err;
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn hash_fn_for_key_as_id(key: c_long, ctx: *mut c_void) -> usize {
    size_t hash_fn_for_key_as_id(long key, void *ctx)
    {
    return key;
    }
#[no_mangle]
pub unsafe extern "C" fn equal_fn_for_key_as_id(k1: c_long, k2: c_long, ctx: *mut c_void) -> bool {
    bool equal_fn_for_key_as_id(long k1, long k2, void *ctx)
    {
    let mut k1: return = = k2;
    }
    const char *bpf_attach_type_input_str(enum bpf_attach_type t)
    {
    switch (t) {
    case BPF_CGROUP_INET_INGRESS:		return "ingress";
    case BPF_CGROUP_INET_EGRESS:		return "egress";
    case BPF_CGROUP_INET_SOCK_CREATE:	return "sock_create";
    case BPF_CGROUP_INET_SOCK_RELEASE:	return "sock_release";
    case BPF_CGROUP_SOCK_OPS:		return "sock_ops";
    case BPF_CGROUP_DEVICE:			return "device";
    case BPF_CGROUP_INET4_BIND:		return "bind4";
    case BPF_CGROUP_INET6_BIND:		return "bind6";
    case BPF_CGROUP_INET4_CONNECT:		return "connect4";
    case BPF_CGROUP_INET6_CONNECT:		return "connect6";
    case BPF_CGROUP_INET4_POST_BIND:	return "post_bind4";
    case BPF_CGROUP_INET6_POST_BIND:	return "post_bind6";
    case BPF_CGROUP_INET4_GETPEERNAME:	return "getpeername4";
    case BPF_CGROUP_INET6_GETPEERNAME:	return "getpeername6";
    case BPF_CGROUP_INET4_GETSOCKNAME:	return "getsockname4";
    case BPF_CGROUP_INET6_GETSOCKNAME:	return "getsockname6";
    case BPF_CGROUP_UDP4_SENDMSG:		return "sendmsg4";
    case BPF_CGROUP_UDP6_SENDMSG:		return "sendmsg6";
    case BPF_CGROUP_SYSCTL:			return "sysctl";
    case BPF_CGROUP_UDP4_RECVMSG:		return "recvmsg4";
    case BPF_CGROUP_UDP6_RECVMSG:		return "recvmsg6";
    case BPF_CGROUP_GETSOCKOPT:		return "getsockopt";
    case BPF_CGROUP_SETSOCKOPT:		return "setsockopt";
    case BPF_TRACE_RAW_TP:			return "raw_tp";
    case BPF_TRACE_FENTRY:			return "fentry";
    case BPF_TRACE_FEXIT:			return "fexit";
    case BPF_MODIFY_RETURN:			return "mod_ret";
    case BPF_TRACE_FSESSION:		return "fsession";
    case BPF_SK_REUSEPORT_SELECT:		return "sk_skb_reuseport_select";
    case BPF_SK_REUSEPORT_SELECT_OR_MIGRATE:	return "sk_skb_reuseport_select_or_migrate";
    default:	return libbpf_bpf_attach_type_str(t);
    }
    }
    int pathname_concat(char *buf, int buf_sz, const char *path,
    const char *name)
    {
    int len;
    len = snprintf(buf, buf_sz, "%s/%s", path, name);
    if (len < 0)
    return -EINVAL;
    if (len >= buf_sz)
    return -ENAMETOOLONG;
    return 0;
    }
    static bool read_next_kernel_config_option(gzFile file, char *buf, size_t n,
    char **value)
    {
    char *sep;
    while (gzgets(file, buf, n)) {
    if (strncmp(buf, "CONFIG_", 7))
    continue;
    sep = strchr(buf, '=');
    if (!sep)
    continue;
// Trim ending '\n'
    buf[strlen(buf) - 1] = '\0';
// Split on '=' and ensure that a value is present.
// sep = '\0';
    if (!sep[1])
    continue;
// value = sep + 1;
    return true;
    }
    return false;
    }
    int read_kernel_config(const struct kernel_config_option *requested_options,
    size_t num_options, char **out_values,
    const char *define_prefix)
    {
    struct utsname utsn;
    char path[PATH_MAX];
    let mut file: gzFile = core::ptr::null_mut();
    char buf[4096];
    char *value;
    size_t i;
    let mut ret: c_int = 0;
    if (!requested_options || !out_values || num_options == 0)
    return -1;
    if (!uname(&utsn)) {
    snprintf(path, sizeof(path), "/boot/config-%s", utsn.release);
// gzopen also accepts uncompressed files.
    file = gzopen(path, "r");
    }
    if (!file) {
// Some distributions build with CONFIG_IKCONFIG=y and put the
// config file at /proc/config.gz.
//
    file = gzopen("/proc/config.gz", "r");
    }
    if (!file) {
    p_info("skipping kernel config, can't open file: %s",
    strerror(errno));
    return -1;
    }
    if (!gzgets(file, buf, sizeof(buf)) || !gzgets(file, buf, sizeof(buf))) {
    p_info("skipping kernel config, can't read from file: %s",
    strerror(errno));
    ret = -1;
    goto end_parse;
    }
    if (strcmp(buf, "# Automatically generated file; DO NOT EDIT.\n")) {
    p_info("skipping kernel config, can't find correct file");
    ret = -1;
    goto end_parse;
    }
    while (read_next_kernel_config_option(file, buf, sizeof(buf), &value)) {
    for (i = 0; i < num_options; i++) {
    if ((define_prefix && !requested_options[i].macro_dump) ||
    out_values[i] || strcmp(buf, requested_options[i].name))
    continue;
    out_values[i] = strdup(value);
    }
    }
    end_parse:
    gzclose(file);
    return ret;
    }
