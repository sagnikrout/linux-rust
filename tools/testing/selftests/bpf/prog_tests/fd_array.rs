//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fd_array.c
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

#[no_mangle]
pub unsafe extern "C" fn new_map() -> c_int {
    static inline int new_map(void)
    {
    const char *name = core::ptr::null_mut();
    let mut max_entries: __u32 = 1;
    let mut value_size: __u32 = 8;
    let mut key_size: __u32 = 4;
    return bpf_map_create(BPF_MAP_TYPE_ARRAY, name,
    key_size, value_size,
    max_entries, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn new_btf() -> c_int {
    static int new_btf(void)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_blob {
    pub btf_hdr: btf_header,
    pub types: [__u32; 8],
    pub str: __u32,
    } raw_btf = {
    .btf_hdr = {
    .magic = BTF_MAGIC,
    .version = BTF_VERSION,
    .hdr_len = sizeof(struct btf_header),
    .type_len = sizeof(raw_btf.types),
    .str_off = offsetof(struct btf_blob, str) - offsetof(struct btf_blob, types),
    .str_len = sizeof(raw_btf.str),
    },
    .types = {
// long
    BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 64, 8),  /* [1] */
// unsigned long
    BTF_TYPE_INT_ENC(0, 0, 0, 64, 8),  /* [2] */
    },
}

    return bpf_btf_load(&raw_btf, sizeof(raw_btf), core::ptr::null_mut());
    }

    if ((FD) >= 0) {	\
    close(FD);	\
    FD = -1;	\
    }			\
    } while(0)
#[no_mangle]
unsafe extern "C" fn map_exists(id: __u32) -> bool {
    static bool map_exists(__u32 id)
    {
    int fd;
    fd = bpf_map_get_fd_by_id(id);
    if (fd >= 0) {
    close(fd);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn btf_exists(id: __u32) -> bool {
    static bool btf_exists(__u32 id)
    {
    int fd;
    fd = bpf_btf_get_fd_by_id(id);
    if (fd >= 0) {
    close(fd);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_get_map_ids(prog_fd: c_int, nr_map_ids: *mut __u32, map_ids: *mut __u32) -> c_int {
    static inline int bpf_prog_get_map_ids(int prog_fd, __u32 *nr_map_ids, __u32 *map_ids)
    {
    let mut len: __u32 = sizeof(struct bpf_prog_info);
    struct bpf_prog_info info;
    int err;
    memset(&info, 0, len);
    info.nr_map_ids = *nr_map_ids;
    info.map_ids = ptr_to_u64(map_ids);
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &len);
    if (!ASSERT_OK(err, "bpf_prog_get_info_by_fd"))
    return -1;
// nr_map_ids = info.nr_map_ids;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __load_test_prog(map_fd: c_int, fd_array: *const c_int, fd_array_cnt: c_int) -> c_int {
    static int __load_test_prog(int map_fd, const int *fd_array, int fd_array_cnt)
    {
// A trivial program which uses one map
    struct bpf_insn insns[] = {
    BPF_LD_MAP_FD(BPF_REG_1, map_fd),
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
    BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
    BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    LIBBPF_OPTS(bpf_prog_load_opts, opts);
    opts.fd_array = fd_array;
    opts.fd_array_cnt = fd_array_cnt;
    return bpf_prog_load(BPF_PROG_TYPE_XDP, core::ptr::null_mut(), "GPL", insns, ARRAY_SIZE(insns), &opts);
    }
#[no_mangle]
unsafe extern "C" fn load_test_prog(fd_array: *const c_int, fd_array_cnt: c_int) -> c_int {
    static int load_test_prog(const int *fd_array, int fd_array_cnt)
    {
    int map_fd;
    int ret;
    map_fd = new_map();
    if (!ASSERT_GE(map_fd, 0, "new_map"))
    return map_fd;
    ret = __load_test_prog(map_fd, fd_array, fd_array_cnt);
    close(map_fd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_expected_map_ids(prog_fd: c_int, expected: c_int, map_ids: *mut __u32, nr_map_ids: *mut __u32) -> bool {
    static bool check_expected_map_ids(int prog_fd, int expected, __u32 *map_ids, __u32 *nr_map_ids)
    {
    int err;
    err = bpf_prog_get_map_ids(prog_fd, nr_map_ids, map_ids);
    if (!ASSERT_OK(err, "bpf_prog_get_map_ids"))
    return false;
    if (!ASSERT_EQ(*nr_map_ids, expected, "unexpected nr_map_ids"))
    return false;
    return true;
    }
//
// Load a program, which uses one map. No fd_array maps are present.
// On return only one map is expected to be bound to prog.
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__no_fd_array() {
    static void check_fd_array_cnt__no_fd_array(void)
    {
    __u32 map_ids[16];
    __u32 nr_map_ids;
    let mut prog_fd: c_int = -1;
    prog_fd = load_test_prog(core::ptr::null_mut(), 0);
    if (!ASSERT_GE(prog_fd, 0, "BPF_PROG_LOAD"))
    return;
    nr_map_ids = ARRAY_SIZE(map_ids);
    check_expected_map_ids(prog_fd, 1, map_ids, &nr_map_ids);
    close(prog_fd);
    }
//
// Load a program, which uses one map, and pass two extra, non-equal, maps in
// fd_array with fd_array_cnt=2. On return three maps are expected to be bound
// to the program.
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__fd_array_ok() {
    static void check_fd_array_cnt__fd_array_ok(void)
    {
    int extra_fds[2] = { -1, -1 };
    __u32 map_ids[16];
    __u32 nr_map_ids;
    let mut prog_fd: c_int = -1;
    extra_fds[0] = new_map();
    if (!ASSERT_GE(extra_fds[0], 0, "new_map"))
    goto cleanup;
    extra_fds[1] = new_map();
    if (!ASSERT_GE(extra_fds[1], 0, "new_map"))
    goto cleanup;
    prog_fd = load_test_prog(extra_fds, 2);
    if (!ASSERT_GE(prog_fd, 0, "BPF_PROG_LOAD"))
    goto cleanup;
    nr_map_ids = ARRAY_SIZE(map_ids);
    if (!check_expected_map_ids(prog_fd, 3, map_ids, &nr_map_ids))
    goto cleanup;
// maps should still exist when original file descriptors are closed
    Close(extra_fds[0]);
    Close(extra_fds[1]);
    if (!ASSERT_EQ(map_exists(map_ids[0]), true, "map_ids[0] should exist"))
    goto cleanup;
    if (!ASSERT_EQ(map_exists(map_ids[1]), true, "map_ids[1] should exist"))
    goto cleanup;
// some fds might be invalid, so ignore return codes
    cleanup:
    Close(extra_fds[1]);
    Close(extra_fds[0]);
    Close(prog_fd);
    }
//
// Load a program with a few extra maps duplicated in the fd_array.
// After the load maps should only be referenced once.
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__duplicated_maps() {
    static void check_fd_array_cnt__duplicated_maps(void)
    {
    int extra_fds[4] = { -1, -1, -1, -1 };
    __u32 map_ids[16];
    __u32 nr_map_ids;
    let mut prog_fd: c_int = -1;
    extra_fds[0] = extra_fds[2] = new_map();
    if (!ASSERT_GE(extra_fds[0], 0, "new_map"))
    goto cleanup;
    extra_fds[1] = extra_fds[3] = new_map();
    if (!ASSERT_GE(extra_fds[1], 0, "new_map"))
    goto cleanup;
    prog_fd = load_test_prog(extra_fds, 4);
    if (!ASSERT_GE(prog_fd, 0, "BPF_PROG_LOAD"))
    goto cleanup;
    nr_map_ids = ARRAY_SIZE(map_ids);
    if (!check_expected_map_ids(prog_fd, 3, map_ids, &nr_map_ids))
    goto cleanup;
// maps should still exist when original file descriptors are closed
    Close(extra_fds[0]);
    Close(extra_fds[1]);
    if (!ASSERT_EQ(map_exists(map_ids[0]), true, "map should exist"))
    goto cleanup;
    if (!ASSERT_EQ(map_exists(map_ids[1]), true, "map should exist"))
    goto cleanup;
// some fds might be invalid, so ignore return codes
    cleanup:
    Close(extra_fds[1]);
    Close(extra_fds[0]);
    Close(prog_fd);
    }
//
// Check that if maps which are referenced by a program are
// passed in fd_array, then they will be referenced only once
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__referenced_maps_in_fd_array() {
    static void check_fd_array_cnt__referenced_maps_in_fd_array(void)
    {
    int extra_fds[1] = { -1 };
    __u32 map_ids[16];
    __u32 nr_map_ids;
    let mut prog_fd: c_int = -1;
    extra_fds[0] = new_map();
    if (!ASSERT_GE(extra_fds[0], 0, "new_map"))
    goto cleanup;
    prog_fd = __load_test_prog(extra_fds[0], extra_fds, 1);
    if (!ASSERT_GE(prog_fd, 0, "BPF_PROG_LOAD"))
    goto cleanup;
    nr_map_ids = ARRAY_SIZE(map_ids);
    if (!check_expected_map_ids(prog_fd, 1, map_ids, &nr_map_ids))
    goto cleanup;
// map should still exist when original file descriptor is closed
    Close(extra_fds[0]);
    if (!ASSERT_EQ(map_exists(map_ids[0]), true, "map should exist"))
    goto cleanup;
// some fds might be invalid, so ignore return codes
    cleanup:
    Close(extra_fds[0]);
    Close(prog_fd);
    }
#[no_mangle]
unsafe extern "C" fn get_btf_id_by_fd(btf_fd: c_int, id: *mut __u32) -> c_int {
    static int get_btf_id_by_fd(int btf_fd, __u32 *id)
    {
    struct bpf_btf_info info;
    let mut info_len: __u32 = sizeof(info);
    int err;
    memset(&info, 0, info_len);
    err = bpf_btf_get_info_by_fd(btf_fd, &info, &info_len);
    if (err)
    return err;
    if (id)
// id = info.id;
    return 0;
    }
//
// Check that fd_array operates properly for btfs. Namely, to check that
// passing a btf fd in fd_array increases its reference count, do the
// following:
// 1) Create a new btf, it's referenced only by a file descriptor, so refcnt=1
// 2) Load a BPF prog with fd_array[0] = btf_fd; now btf's refcnt=2
// 3) Close the btf_fd, now refcnt=1
// Wait and check that BTF still exists.
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__referenced_btfs() {
    static void check_fd_array_cnt__referenced_btfs(void)
    {
    int extra_fds[1] = { -1 };
    let mut prog_fd: c_int = -1;
    __u32 btf_id;
    int tries;
    int err;
    extra_fds[0] = new_btf();
    if (!ASSERT_GE(extra_fds[0], 0, "new_btf"))
    goto cleanup;
    prog_fd = load_test_prog(extra_fds, 1);
    if (!ASSERT_GE(prog_fd, 0, "BPF_PROG_LOAD"))
    goto cleanup;
// btf should still exist when original file descriptor is closed
    err = get_btf_id_by_fd(extra_fds[0], &btf_id);
    if (!ASSERT_EQ(err, 0, "get_btf_id_by_fd"))
    goto cleanup;
    Close(extra_fds[0]);
    if (!ASSERT_GE(kern_sync_rcu(), 0, "kern_sync_rcu 1"))
    goto cleanup;
    if (!ASSERT_EQ(btf_exists(btf_id), true, "btf should exist"))
    goto cleanup;
    Close(prog_fd);
// The program is freed by a workqueue, so no reliable
// way to sync, so just wait a bit (max ~1 second).
    for (tries = 100; tries >= 0; tries--) {
    usleep(1000);
    if (!btf_exists(btf_id))
    break;
    if (tries)
    continue;
    PRINT_FAIL("btf should have been freed");
    }
// some fds might be invalid, so ignore return codes
    cleanup:
    Close(extra_fds[0]);
    Close(prog_fd);
    }
//
// Test that a program with trash in fd_array can't be loaded:
// only map and BTF file descriptors should be accepted.
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__fd_array_with_trash() {
    static void check_fd_array_cnt__fd_array_with_trash(void)
    {
    int extra_fds[3] = { -1, -1, -1 };
    let mut prog_fd: c_int = -1;
    extra_fds[0] = new_map();
    if (!ASSERT_GE(extra_fds[0], 0, "new_map"))
    goto cleanup;
    extra_fds[1] = new_btf();
    if (!ASSERT_GE(extra_fds[1], 0, "new_btf"))
    goto cleanup;
// trash 1: not a file descriptor
    extra_fds[2] = 0xbeef;
    prog_fd = load_test_prog(extra_fds, 3);
    if (!ASSERT_EQ(prog_fd, -EBADF, "prog should have been rejected with -EBADF"))
    goto cleanup;
// trash 2: not a map or btf
    extra_fds[2] = socket(AF_INET, SOCK_STREAM, 0);
    if (!ASSERT_GE(extra_fds[2], 0, "socket"))
    goto cleanup;
    prog_fd = load_test_prog(extra_fds, 3);
    if (!ASSERT_EQ(prog_fd, -EINVAL, "prog should have been rejected with -EINVAL"))
    goto cleanup;
// Validate that the prog is ok if trash is removed
    Close(extra_fds[2]);
    extra_fds[2] = new_btf();
    if (!ASSERT_GE(extra_fds[2], 0, "new_btf"))
    goto cleanup;
    prog_fd = load_test_prog(extra_fds, 3);
    if (!ASSERT_GE(prog_fd, 0, "prog should have been loaded"))
    goto cleanup;
// some fds might be invalid, so ignore return codes
    cleanup:
    Close(extra_fds[2]);
    Close(extra_fds[1]);
    Close(extra_fds[0]);
    }
//
// Test that a program with too big fd_array can't be loaded.
//
#[no_mangle]
unsafe extern "C" fn check_fd_array_cnt__fd_array_too_big() {
    static void check_fd_array_cnt__fd_array_too_big(void)
    {
    int extra_fds[65];
    let mut prog_fd: c_int = -1;
    int i;
    for (i = 0; i < 65; i++) {
    extra_fds[i] = new_map();
    if (!ASSERT_GE(extra_fds[i], 0, "new_map"))
    goto cleanup_fds;
    }
    prog_fd = load_test_prog(extra_fds, 65);
    ASSERT_EQ(prog_fd, -E2BIG, "prog should have been rejected with -E2BIG");
    cleanup_fds:
    while (i-- > 0)
    Close(extra_fds[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn test_fd_array_cnt() {
    void test_fd_array_cnt(void)
    {
    if (test__start_subtest("no-fd-array"))
    check_fd_array_cnt__no_fd_array();
    if (test__start_subtest("fd-array-ok"))
    check_fd_array_cnt__fd_array_ok();
    if (test__start_subtest("fd-array-dup-input"))
    check_fd_array_cnt__duplicated_maps();
    if (test__start_subtest("fd-array-ref-maps-in-array"))
    check_fd_array_cnt__referenced_maps_in_fd_array();
    if (test__start_subtest("fd-array-ref-btfs"))
    check_fd_array_cnt__referenced_btfs();
    if (test__start_subtest("fd-array-trash-input"))
    check_fd_array_cnt__fd_array_with_trash();
    if (test__start_subtest("fd-array-2big"))
    check_fd_array_cnt__fd_array_too_big();
    }
