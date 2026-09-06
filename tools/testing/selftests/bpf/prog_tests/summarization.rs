//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/summarization.c
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
unsafe extern "C" fn print_verifier_log(log: *const c_char) {
    static void print_verifier_log(const char *log)
    {
    if (env.verbosity >= VERBOSE_VERY)
    fprintf(stdout, "VERIFIER LOG:\n=============\n%s=============\n", log);
    }
    static void test_aux(const char *main_prog_name,
    const char *to_be_replaced,
    const char *replacement,
    bool expect_load,
    const char *err_msg)
    {
    struct summarization_freplace *freplace = core::ptr::null_mut();
    struct bpf_program *freplace_prog = core::ptr::null_mut();
    struct bpf_program *main_prog = core::ptr::null_mut();
    LIBBPF_OPTS(bpf_object_open_opts, opts);
    struct summarization *main = core::ptr::null_mut();
    char log[16*1024];
    int err;
    opts.kernel_log_buf = log;
    opts.kernel_log_size = sizeof(log);
    if (env.verbosity >= VERBOSE_SUPER)
    opts.kernel_log_level = 1 | 2 | 4;
    main = summarization__open_opts(&opts);
    if (!ASSERT_OK_PTR(main, "summarization__open"))
    goto out;
    main_prog = bpf_object__find_program_by_name(main.obj, main_prog_name);
    if (!ASSERT_OK_PTR(main_prog, "main_prog"))
    goto out;
    bpf_program__set_autoload(main_prog, true);
    err = summarization__load(main);
    print_verifier_log(log);
    if (!ASSERT_OK(err, "summarization__load"))
    goto out;
    freplace = summarization_freplace__open_opts(&opts);
    if (!ASSERT_OK_PTR(freplace, "summarization_freplace__open"))
    goto out;
    freplace_prog = bpf_object__find_program_by_name(freplace.obj, replacement);
    if (!ASSERT_OK_PTR(freplace_prog, "freplace_prog"))
    goto out;
    bpf_program__set_autoload(freplace_prog, true);
    bpf_program__set_autoattach(freplace_prog, true);
    bpf_program__set_attach_target(freplace_prog,
    bpf_program__fd(main_prog),
    to_be_replaced);
    err = summarization_freplace__load(freplace);
    print_verifier_log(log);
// The might_sleep extension doesn't work yet as sleepable calls are not
// allowed, but preserve the check in case it's supported later and then
// this particular combination can be enabled.
//
    if (!strcmp("might_sleep", replacement) && err) {
    ASSERT_HAS_SUBSTR(log, "sleepable helper bpf_copy_from_user#", "error log");
    ASSERT_EQ(err, -EINVAL, "err");
    test__skip();
    goto out;
    }
    if (expect_load) {
    ASSERT_OK(err, "summarization_freplace__load");
    } else {
    ASSERT_ERR(err, "summarization_freplace__load");
    ASSERT_HAS_SUBSTR(log, err_msg, "error log");
    }
    out:
    summarization_freplace__destroy(freplace);
    summarization__destroy(main);
    }
// There are two global subprograms in both summarization.skel.h:
// - one changes packet data;
// - another does not.
// It is ok to freplace subprograms that change packet data with those
// that either do or do not. It is only ok to freplace subprograms
// that do not change packet data with those that do not as well.
// The below tests check outcomes for each combination of such freplace.
// Also test a case when main subprogram itself is replaced and is a single
// subprogram in a program.
//
// This holds for might_sleep programs. It is ok to replace might_sleep with
// might_sleep and with does_not_sleep, but does_not_sleep cannot be replaced
// with might_sleep.
//
#[no_mangle]
pub unsafe extern "C" fn test_summarization_freplace() {
    void test_summarization_freplace(void)
    {
    struct {
    const char *main;
    const char *to_be_replaced;
    bool has_side_effect;
    } mains[2][4] = {
    {
    { "main_changes_with_subprogs",		"changes_pkt_data",	    true },
    { "main_changes_with_subprogs",		"does_not_change_pkt_data", false },
    { "main_changes",			"main_changes",             true },
    { "main_does_not_change",		"main_does_not_change",     false },
    },
    {
    { "main_might_sleep_with_subprogs",	"might_sleep",		    true },
    { "main_might_sleep_with_subprogs",	"does_not_sleep",	    false },
    { "main_might_sleep",			"main_might_sleep",	    true },
    { "main_does_not_sleep",		"main_does_not_sleep",	    false },
    },
    };
    const char *pkt_err = "Extension program changes packet data";
    const char *slp_err = "Extension program may sleep";
    struct {
    const char *func;
    bool has_side_effect;
    const char *err_msg;
    } replacements[2][2] = {
    {
    { "changes_pkt_data",	      true,	pkt_err },
    { "does_not_change_pkt_data", false,	pkt_err },
    },
    {
    { "might_sleep",	      true,	slp_err },
    { "does_not_sleep",	      false,	slp_err },
    },
    };
    char buf[64];
    for (int t = 0; t < 2; t++) {
    for (int i = 0; i < ARRAY_SIZE(mains); ++i) {
    for (int j = 0; j < ARRAY_SIZE(replacements); ++j) {
    snprintf(buf, sizeof(buf), "%s_with_%s",
    mains[t][i].to_be_replaced, replacements[t][j].func);
    if (!test__start_subtest(buf))
    continue;
    test_aux(mains[t][i].main, mains[t][i].to_be_replaced, replacements[t][j].func,
    mains[t][i].has_side_effect || !replacements[t][j].has_side_effect,
    replacements[t][j].err_msg);
    }
    }
    }
    }
