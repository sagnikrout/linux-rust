//! Automatically rewritten from C to Rust
//! Source: tools/objtool/builtin-check.c
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
// Copyright (C) 2015-2017 Josh Poimboeuf <jpoimboe@redhat.com>
//

    int orig_argc;
    static char **orig_argv;
    const char *objname;
    struct opts opts;
    static const char * const check_usage[] = {
    "objtool <actions> [<options>] file.o",
    core::ptr::null_mut(),
    };
    static const char * const env_usage[] = {
    "OBJTOOL_ARGS=\"<options>\"",
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn parse_dump(opt: *const option, str: *const c_char, unset: c_int) -> c_int {
    static int parse_dump(const struct option *opt, const char *str, int unset)
    {
    if (!str || !strcmp(str, "orc")) {
    opts.dump_orc = true;
    return 0;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn parse_hacks(opt: *const option, str: *const c_char, unset: c_int) -> c_int {
    static int parse_hacks(const struct option *opt, const char *str, int unset)
    {
    let mut found: bool = false;
//
// Use strstr() as a lazy method of checking for comma-separated
// options.
//
// No string provided == enable all options.
//
    if (!str || strstr(str, "jump_label")) {
    opts.hack_jump_label = true;
    found = true;
    }
    if (!str || strstr(str, "noinstr")) {
    opts.hack_noinstr = true;
    found = true;
    }
    if (!str || strstr(str, "skylake")) {
    opts.hack_skylake = true;
    found = true;
    }
    return found ? 0 : -1;
    }
    static const struct option check_options[] = {
    OPT_GROUP("Actions:"),
    OPT_STRING_OPTARG('d',	 "disas", &opts.disas, "function-pattern", "disassemble functions", "*"),
    OPT_CALLBACK_OPTARG('h', "hacks", core::ptr::null_mut(), core::ptr::null_mut(), "jump_label,noinstr,skylake", "patch toolchain bugs/limitations", parse_hacks),
    OPT_BOOLEAN('i',	 "ibt", &opts.ibt, "validate and annotate IBT"),
    OPT_BOOLEAN(0,		 "klp-symids", &opts.klp_symids, "generate .klp.symids for duplicate symbol disambiguation"),
    OPT_BOOLEAN('m',	 "mcount", &opts.mcount, "annotate mcount/fentry calls for ftrace"),
    OPT_BOOLEAN(0,		 "noabs", &opts.noabs, "reject absolute references in allocatable sections"),
    OPT_BOOLEAN('n',	 "noinstr", &opts.noinstr, "validate noinstr rules"),
    OPT_BOOLEAN(0,		 "orc", &opts.orc, "generate ORC metadata"),
    OPT_BOOLEAN('r',	 "retpoline", &opts.retpoline, "validate and annotate retpoline usage"),
    OPT_BOOLEAN(0,		 "rethunk", &opts.rethunk, "validate and annotate rethunk usage"),
    OPT_BOOLEAN(0,		 "unret", &opts.unret, "validate entry unret placement"),
    OPT_INTEGER(0,		 "prefix", &opts.prefix, "generate or grow prefix symbols for N-byte function padding"),
    OPT_BOOLEAN('l',	 "sls", &opts.sls, "validate straight-line-speculation mitigations"),
    OPT_BOOLEAN('s',	 "stackval", &opts.stackval, "validate frame pointer rules"),
    OPT_BOOLEAN('t',	 "static-call", &opts.static_call, "annotate static calls"),
    OPT_BOOLEAN('u',	 "uaccess", &opts.uaccess, "validate uaccess rules for SMAP"),
    OPT_CALLBACK_OPTARG(0,	 "dump", core::ptr::null_mut(), core::ptr::null_mut(), "orc", "dump metadata", parse_dump),
    OPT_GROUP("Options:"),
    OPT_BOOLEAN(0,		 "cfi", &opts.cfi, "grow kCFI preamble symbols (use with --prefix)"),
    OPT_BOOLEAN(0,		 "fineibt", &opts.fineibt, "create .cfi_sites section for FineIBT"),
    OPT_BOOLEAN(0,		 "backtrace", &opts.backtrace, "unwind on error"),
    OPT_BOOLEAN(0,		 "backup", &opts.backup, "create backup (.orig) file on warning/error"),
    OPT_BOOLEAN(0,		 "dry-run", &opts.dryrun, "don't write modifications"),
    OPT_BOOLEAN(0,		 "link", &opts.link, "object is a linked object"),
    OPT_BOOLEAN(0,		 "module", &opts.module, "object is part of a kernel module"),
    OPT_BOOLEAN(0,		 "mnop", &opts.mnop, "nop out mcount call sites"),
    OPT_BOOLEAN(0,		 "no-unreachable", &opts.no_unreachable, "skip 'unreachable instruction' warnings"),
    OPT_STRING('o',		 "output", &opts.output, "file", "output file name"),
    OPT_BOOLEAN(0,		 "sec-address", &opts.sec_address, "print section addresses in warnings"),
    OPT_BOOLEAN(0,		 "stats", &opts.stats, "print statistics"),
    OPT_STRING(0,		 "trace", &opts.trace, "func", "trace function validation"),
    OPT_BOOLEAN('v',	 "verbose", &opts.verbose, "verbose warnings"),
    OPT_BOOLEAN(0,		 "werror", &opts.werror, "return error on warnings"),
    OPT_BOOLEAN(0,		 "wide", &opts.wide, "wide output"),
    OPT_END(),
    };
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_options(argc: c_int, argv: *const c_char, usage[]: *const *const c_char) -> c_int {
    int cmd_parse_options(int argc, const char **argv, const char * const usage[])
    {
    const char *envv[16] = { };
    char *env;
    int envc;
    env = getenv("OBJTOOL_ARGS");
    if (env) {
    envv[0] = "OBJTOOL_ARGS";
    for (envc = 1; envc < ARRAY_SIZE(envv); ) {
    envv[envc++] = env;
    env = strchr(env, ' ');
    if (!env)
    break;
// env = '\0';
    env++;
    }
    parse_options(envc, envv, check_options, env_usage, 0);
    }
    env = getenv("OBJTOOL_VERBOSE");
    if (env && !strcmp(env, "1"))
    opts.verbose = true;
    argc = parse_options(argc, argv, check_options, usage, 0);
    if (argc != 1)
    usage_with_options(usage, check_options);
    return argc;
    }
#[no_mangle]
unsafe extern "C" fn opts_valid() -> bool {
    static bool opts_valid(void)
    {
    if (opts.mnop && !opts.mcount) {
    ERROR("--mnop requires --mcount");
    return false;
    }
    if (opts.noinstr && !opts.link) {
    ERROR("--noinstr requires --link");
    return false;
    }
    if (opts.ibt && !opts.link) {
    ERROR("--ibt requires --link");
    return false;
    }
    if (opts.unret && !opts.link) {
    ERROR("--unret requires --link");
    return false;
    }
    if (opts.cfi && !opts.prefix) {
    ERROR("--cfi requires --prefix");
    return false;
    }
    if (opts.fineibt && !opts.cfi) {
    ERROR("--fineibt requires --cfi");
    return false;
    }
    if (opts.klp_symids && !opts.link) {
    ERROR("--klp-symids requires --link");
    return false;
    }
    if (opts.disas			||
    opts.hack_jump_label	||
    opts.hack_noinstr		||
    opts.ibt			||
    opts.klp_symids		||
    opts.mcount			||
    opts.noabs			||
    opts.noinstr		||
    opts.orc			||
    opts.retpoline		||
    opts.rethunk		||
    opts.sls			||
    opts.stackval		||
    opts.static_call		||
    opts.uaccess) {
    if (opts.dump_orc) {
    ERROR("--dump can't be combined with other actions");
    return false;
    }
    return true;
    }
    if (opts.dump_orc)
    return true;
    ERROR("At least one action required");
    return false;
    }
#[no_mangle]
unsafe extern "C" fn copy_file(src: *const c_char, dst: *const c_char) -> c_int {
    static int copy_file(const char *src, const char *dst)
    {
    size_t to_copy, copied;
    int dst_fd, src_fd;
    struct stat stat;
    let mut offset: off_t = 0;
    src_fd = open(src, O_RDONLY);
    if (src_fd == -1) {
    ERROR("can't open %s for reading: %s", src, strerror(errno));
    return 1;
    }
    dst_fd = open(dst, O_WRONLY | O_CREAT | O_TRUNC, 0400);
    if (dst_fd == -1) {
    ERROR("can't open %s for writing: %s", dst, strerror(errno));
    return 1;
    }
    if (fstat(src_fd, &stat) == -1) {
    ERROR_GLIBC("fstat");
    return 1;
    }
    if (fchmod(dst_fd, stat.st_mode) == -1) {
    ERROR_GLIBC("fchmod");
    return 1;
    }
    for (to_copy = stat.st_size; to_copy > 0; to_copy -= copied) {
    copied = sendfile(dst_fd, src_fd, &offset, to_copy);
    if (copied == -1) {
    ERROR_GLIBC("sendfile");
    return 1;
    }
    }
    close(dst_fd);
    close(src_fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn save_argv(argc: c_int, argv: *const c_char) {
    static void save_argv(int argc, const char **argv)
    {
    orig_argv = calloc(argc, sizeof(char *));
    if (!orig_argv) {
    ERROR_GLIBC("calloc");
    exit(1);
    }
    for (int i = 0; i < argc; i++) {
    orig_argv[i] = strdup(argv[i]);
    if (!orig_argv[i]) {
    ERROR_GLIBC("strdup(%s)", argv[i]);
    exit(1);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn make_backup() -> c_int {
    int make_backup(void)
    {
    char *backup;
//
// Make a backup before kbuild deletes the file so the error
// can be recreated without recompiling or relinking.
//
    backup = malloc(strlen(objname) + strlen(ORIG_SUFFIX) + 1);
    if (!backup) {
    ERROR_GLIBC("malloc");
    return 1;
    }
    strcpy(backup, objname);
    strcat(backup, ORIG_SUFFIX);
    if (copy_file(objname, backup))
    return 1;
//
// Print the cmdline args to make it easier to recreate.
//
    fprintf(stderr, "%s", orig_argv[0]);
    for (int i = 1; i < orig_argc; i++) {
    char *arg = orig_argv[i];
// Modify the printed args to use the backup
    if (!opts.output && !strcmp(arg, objname))
    fprintf(stderr, " %s -o %s", backup, objname);
    else
    fprintf(stderr, " %s", arg);
    }
    fprintf(stderr, "\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn objtool_run(argc: c_int, argv: *const c_char) -> c_int {
    int objtool_run(int argc, const char **argv)
    {
    struct objtool_file *file;
    let mut ret: c_int = 0;
    orig_argc = argc;
    save_argv(argc, argv);
    cmd_parse_options(argc, argv, check_usage);
    if (!opts_valid())
    return 1;
    objname = argv[0];
    if (opts.dump_orc)
    return orc_dump(objname);
    if (!opts.dryrun && opts.output) {
// copy original .o file to output file
    if (copy_file(objname, opts.output))
    return 1;
// from here on, work directly on the output file
    objname = opts.output;
    }
    file = objtool_open_read(objname);
    if (!file)
    return 1;
    if (!opts.link && has_multiple_files(file.elf)) {
    ERROR("Linked object requires --link");
    return 1;
    }
    ret = check(file);
    if (ret)
    return ret;
    if (!opts.dryrun && file.elf.changed && elf_write(file.elf))
    return 1;
    return elf_close(file.elf);
    }
