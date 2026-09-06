//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/symbols.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_info {
    pub host_env: perf_env,
    pub machine: *mut machine,
    pub thread: *mut thread,
}

#[no_mangle]
unsafe extern "C" fn init_test_info(ti: *mut test_info) -> c_int {
    static int init_test_info(struct test_info *ti)
    {
    perf_env__init(&ti.host_env);
    ti.machine = machine__new_host(&ti.host_env);
    if (!ti.machine) {
    pr_debug("machine__new_host() failed!\n");
    perf_env__exit(&ti.host_env);
    return TEST_FAIL;
    }
// Create a dummy thread
    ti.thread = machine__findnew_thread(ti.machine, 100, 100);
    if (!ti.thread) {
    pr_debug("machine__findnew_thread() failed!\n");
    perf_env__exit(&ti.host_env);
    return TEST_FAIL;
    }
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn exit_test_info(ti: *mut test_info) {
    static void exit_test_info(struct test_info *ti)
    {
    thread__put(ti.thread);
    machine__delete(ti.machine);
    perf_env__exit(&ti.host_env);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dso_map {
    pub dso: *mut dso,
    pub map: *mut map,
}

#[no_mangle]
unsafe extern "C" fn find_map_cb(map: *mut map, d: *mut c_void) -> c_int {
    static int find_map_cb(struct map *map, void *d)
    {
    struct dso_map *data = d;
    if (map__dso(map) != data.dso)
    return 0;
    data.map = map;
    return 1;
    }
    static struct map *find_module_map(struct machine *machine, struct dso *dso)
    {
    let mut data: dso_map = { .dso = dso };
    machine__for_each_kernel_map(machine, find_map_cb, &data);
    return data.map;
    }
#[no_mangle]
unsafe extern "C" fn get_test_dso_filename(filename: *mut c_char, max_sz: usize) {
    static void get_test_dso_filename(char *filename, size_t max_sz)
    {
    if (dso_to_test)
    strlcpy(filename, dso_to_test, max_sz);
    else
    perf_exe(filename, max_sz);
    }
#[no_mangle]
unsafe extern "C" fn create_map(ti: *mut test_info, filename: *mut c_char, map_p: *mut map) -> c_int {
    static int create_map(struct test_info *ti, char *filename, struct map **map_p)
    {
    struct dso *dso = machine__findnew_dso(ti.machine, filename);
//
// If 'filename' matches a current kernel module, must use a kernel
// map. Find the one that already exists.
//
    if (dso && dso__kernel(dso) != DSO_SPACE__USER) {
// map_p = find_module_map(ti->machine, dso);
    dso__put(dso);
    if (!*map_p) {
    pr_debug("Failed to find map for current kernel module %s",
    filename);
    return TEST_FAIL;
    }
    map__get(*map_p);
    return TEST_OK;
    }
    dso__put(dso);
// Create a dummy map at 0x100000
// map_p = map__new(ti->machine, 0x100000, 0xffffffff, 0, &dso_id_empty,
    PROT_EXEC, /*flags=*/0, filename, ti.thread);
    if (!*map_p) {
    pr_debug("Failed to create map!");
    return TEST_FAIL;
    }
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test_dso(dso: *mut dso) -> c_int {
    static int test_dso(struct dso *dso)
    {
    struct symbol *last_sym = core::ptr::null_mut();
    struct rb_node *nd;
    let mut ret: c_int = TEST_OK;
// dso__fprintf() prints all the symbols
    if (verbose > 1)
    dso__fprintf(dso, stderr);
    for (nd = rb_first_cached(dso__symbols(dso)); nd; nd = rb_next(nd)) {
    struct symbol *sym = rb_entry(nd, struct symbol, rb_node);
    if (symbol__type(sym) != STT_FUNC && symbol__type(sym) != STT_GNU_IFUNC)
    continue;
// Check for overlapping function symbols
    if (last_sym && sym.start < last_sym.end) {
    pr_debug("Overlapping symbols:\n");
    symbol__fprintf(last_sym, stderr);
    symbol__fprintf(sym, stderr);
    ret = TEST_FAIL;
    }
// Check for zero-length function symbol
    if (sym.start == sym.end) {
    pr_debug("Zero-length symbol:\n");
    symbol__fprintf(sym, stderr);
    ret = TEST_FAIL;
    }
    last_sym = sym;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn subdivided_dso_cb(dso: *mut dso, __maybe_unused: *mut *mut machine machine, d: *mut c_void) -> c_int {
    static int subdivided_dso_cb(struct dso *dso, struct machine *machine __maybe_unused, void *d)
    {
    struct dso *text_dso = d;
    if (dso != text_dso && strstarts(dso__short_name(dso), dso__short_name(text_dso)))
    if (test_dso(dso) != TEST_OK)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn process_subdivided_dso(machine: *mut machine, dso: *mut dso) -> c_int {
    static int process_subdivided_dso(struct machine *machine, struct dso *dso)
    {
    int ret;
    ret = machine__for_each_dso(machine, subdivided_dso_cb, dso);
    return ret < 0 ? TEST_FAIL : TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn test_file(ti: *mut test_info, filename: *mut c_char) -> c_int {
    static int test_file(struct test_info *ti, char *filename)
    {
    struct map *map = core::ptr::null_mut();
    int ret, nr;
    struct dso *dso;
    pr_debug("Testing %s\n", filename);
    ret = create_map(ti, filename, &map);
    if (ret != TEST_OK)
    return ret;
    dso = map__dso(map);
    nr = dso__load(dso, map);
    if (nr < 0) {
    pr_debug("dso__load() failed!\n");
    ret = TEST_FAIL;
    goto out_put;
    }
    if (nr == 0) {
    pr_debug("DSO has no symbols!\n");
    ret = TEST_SKIP;
    goto out_put;
    }
    ret = test_dso(dso);
// Module dso is split into many dsos by section
    if (ret == TEST_OK && dso__kernel(dso) != DSO_SPACE__USER)
    ret = process_subdivided_dso(ti.machine, dso);
    out_put:
    map__put(map);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test__symbols(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__symbols(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    char filename[PATH_MAX];
    struct test_info ti;
    int ret;
    ret = init_test_info(&ti);
    if (ret != TEST_OK)
    return ret;
    get_test_dso_filename(filename, sizeof(filename));
    ret = test_file(&ti, filename);
    exit_test_info(&ti);
    return ret;
    }
    DEFINE_SUITE("Symbols", symbols);
