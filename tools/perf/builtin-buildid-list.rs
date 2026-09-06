//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-buildid-list.c
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


//
// builtin-buildid-list.c
//
// Builtin buildid-list command: list buildids in perf.data, in the running
// kernel and in ELF files.
//
// Copyright (C) 2009, Red Hat Inc.
// Copyright (C) 2009, Arnaldo Carvalho de Melo <acme@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn buildid__map_cb(map: *mut map, __maybe_unused: *mut *mut void arg) -> c_int {
    static int buildid__map_cb(struct map *map, void *arg __maybe_unused)
    {
    const struct dso *dso = map__dso(map);
    char bid_buf[SBUILD_ID_SIZE];
    const char *dso_long_name = dso__long_name(dso);
    const char *dso_short_name = dso__short_name(dso);
    memset(bid_buf, 0, sizeof(bid_buf));
    if (dso__has_build_id(dso))
    build_id__snprintf(dso__bid(dso), bid_buf, sizeof(bid_buf));
    printf("%s %16" PRIx64 " %16" PRIx64, bid_buf, map__start(map), map__end(map));
    if (dso_long_name != core::ptr::null_mut())
    printf(" %s", dso_long_name);
#[no_mangle]
pub unsafe extern "C" fn if(NULL: dso_short_name !=) -> else {
    else if (dso_short_name != core::ptr::null_mut())
    printf(" %s", dso_short_name);
    printf("\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn buildid__show_kernel_maps() {
    static void buildid__show_kernel_maps(void)
    {
    struct perf_env host_env;
    struct machine *machine;
    perf_env__init(&host_env);
    machine = machine__new_host(&host_env);
    machine__for_each_kernel_map(machine, buildid__map_cb, core::ptr::null_mut());
    machine__delete(machine);
    perf_env__exit(&host_env);
    }
#[no_mangle]
unsafe extern "C" fn sysfs__fprintf_build_id(fp: *mut FILE) -> c_int {
    static int sysfs__fprintf_build_id(FILE *fp)
    {
    char sbuild_id[SBUILD_ID_SIZE];
    int ret;
    ret = sysfs__snprintf_build_id("/", sbuild_id, sizeof(sbuild_id));
    if (ret + 1 != sizeof(sbuild_id))
    return ret < 0 ? ret : -EINVAL;
    return fprintf(fp, "%s\n", sbuild_id);
    }
#[no_mangle]
unsafe extern "C" fn filename__fprintf_build_id(name: *const c_char, fp: *mut FILE) -> c_int {
    static int filename__fprintf_build_id(const char *name, FILE *fp)
    {
    char sbuild_id[SBUILD_ID_SIZE];
    int ret;
    ret = filename__snprintf_build_id(name, sbuild_id, sizeof(sbuild_id));
    if (ret + 1 != sizeof(sbuild_id))
    return ret < 0 ? ret : -EINVAL;
    return fprintf(fp, "%s\n", sbuild_id);
    }
#[no_mangle]
unsafe extern "C" fn dso__skip_buildid(dso: *mut dso, with_hits: c_int) -> bool {
    static bool dso__skip_buildid(struct dso *dso, int with_hits)
    {
    return with_hits && !dso__hit(dso);
    }
#[no_mangle]
unsafe extern "C" fn perf_session__list_build_ids(force: bool, with_hits: bool) -> c_int {
    static int perf_session__list_build_ids(bool force, bool with_hits)
    {
    struct perf_session *session;
    struct perf_data data = {
    .path  = input_name,
    .mode  = PERF_DATA_MODE_READ,
    .force = force,
    };
    struct perf_tool build_id__mark_dso_hit_ops;
    symbol__elf_init();
//
// See if this is an ELF file first:
//
    if (filename__fprintf_build_id(input_name, stdout) > 0)
    goto out;
    perf_tool__init(&build_id__mark_dso_hit_ops, /*ordered_events=*/true);
    build_id__mark_dso_hit_ops.sample	= build_id__mark_dso_hit;
    build_id__mark_dso_hit_ops.mmap		= perf_event__process_mmap;
    build_id__mark_dso_hit_ops.mmap2	= perf_event__process_mmap2;
    build_id__mark_dso_hit_ops.fork		= perf_event__process_fork;
    build_id__mark_dso_hit_ops.exit		= perf_event__exit_del_thread;
    build_id__mark_dso_hit_ops.attr		= perf_event__process_attr;
    build_id__mark_dso_hit_ops.build_id	= perf_event__process_build_id;
    session = perf_session__new(&data, &build_id__mark_dso_hit_ops);
    if (IS_ERR(session))
    return PTR_ERR(session);
//
// We take all buildids when the file contains AUX area tracing data
// because we do not decode the trace because it would take too long.
//
    if (!perf_data__is_pipe(&data) &&
    perf_header__has_feat(&session.header, HEADER_AUXTRACE))
    with_hits = false;
    if (!perf_header__has_feat(&session.header, HEADER_BUILD_ID))
    with_hits = true;
    if (zstd_init(&(session.zstd_data), 0) < 0)
    pr_warning("Decompression initialization failed. Reported data may be incomplete.\n");
//
// in pipe-mode, the only way to get the buildids is to parse
// the record stream. Buildids are stored as RECORD_HEADER_BUILD_ID
//
    if (with_hits || perf_data__is_pipe(&data))
    perf_session__process_events(session);
    perf_session__fprintf_dsos_buildid(session, stdout, dso__skip_buildid, with_hits);
    perf_session__delete(session);
    out:
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_buildid_list(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_buildid_list(int argc, const char **argv)
    {
    let mut show_kernel: bool = false;
    let mut show_kernel_maps: bool = false;
    let mut with_hits: bool = false;
    let mut force: bool = false;
    const struct option options[] = {
    OPT_BOOLEAN('H', "with-hits", &with_hits, "Show only DSOs with hits"),
    OPT_STRING('i', "input", &input_name, "file", "input file name"),
    OPT_BOOLEAN('f', "force", &force, "don't complain, do it"),
    OPT_BOOLEAN('k', "kernel", &show_kernel, "Show current kernel build id"),
    OPT_BOOLEAN('m', "kernel-maps", &show_kernel_maps,
    "Show build id of current kernel + modules"),
    OPT_INCR('v', "verbose", &verbose, "be more verbose"),
    OPT_END()
    };
    const char * const buildid_list_usage[] = {
    "perf buildid-list [<options>]",
    core::ptr::null_mut()
    };
    argc = parse_options(argc, argv, options, buildid_list_usage, 0);
    setup_pager();
    if (show_kernel) {
    return !(sysfs__fprintf_build_id(stdout) > 0);
    } else if (show_kernel_maps) {
    buildid__show_kernel_maps();
    return 0;
    }
    return perf_session__list_build_ids(force, with_hits);
    }
