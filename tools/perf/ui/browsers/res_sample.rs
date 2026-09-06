//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/browsers/res_sample.c
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
// Display a menu with individual samples to browse with perf script

    let mut context_len: static u64 = 10 * NSEC_PER_MSEC;
#[no_mangle]
unsafe extern "C" fn res_sample_config(var: *const c_char, value: *const c_char, __maybe_unused: *mut *mut void data) -> c_int {
    static int res_sample_config(const char *var, const char *value, void *data __maybe_unused)
    {
    if (!strcmp(var, "samples.context"))
    return perf_config_u64(&context_len, var, value);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn res_sample_init() {
    void res_sample_init(void)
    {
    perf_config(res_sample_config, core::ptr::null_mut());
    }
    int res_sample_browse(struct res_sample *res_samples, int num_res,
    struct evsel *evsel, enum rstype rstype)
    {
    char **names;
    int i, n;
    int choice;
    char *cmd;
    char pbuf[256], tidbuf[32], cpubuf[32];
    const char *perf = perf_exe(pbuf, sizeof pbuf);
    char trange[128], tsample[64];
    struct res_sample *r;
    char extra_format[256];
    names = calloc(num_res, sizeof(char *));
    if (!names)
    return -1;
    for (i = 0; i < num_res; i++) {
    char tbuf[64];
    timestamp__scnprintf_nsec(res_samples[i].time, tbuf, sizeof tbuf);
    if (asprintf(&names[i], "%s: CPU %d tid %d", tbuf,
    res_samples[i].cpu, res_samples[i].tid) < 0) {
    while (--i >= 0)
    zfree(&names[i]);
    free(names);
    return -1;
    }
    }
    choice = ui__popup_menu(num_res, names, core::ptr::null_mut());
    for (i = 0; i < num_res; i++)
    zfree(&names[i]);
    free(names);
    if (choice < 0 || choice >= num_res)
    return -1;
    r = &res_samples[choice];
    n = timestamp__scnprintf_nsec(r.time - context_len, trange, sizeof trange);
    trange[n++] = ',';
    timestamp__scnprintf_nsec(r.time + context_len, trange + n, sizeof trange - n);
    timestamp__scnprintf_nsec(r.time, tsample, sizeof tsample);
    attr_to_script(extra_format, &evsel.core.attr);
    if (asprintf(&cmd, "%s script %s%s --time %s %s%s %s%s --ns %s %s %s %s %s | less +/%s",
    perf,
    input_name ? "-i " : "",
    input_name ? input_name : "",
    trange,
    r.cpu >= 0 ? "--cpu " : "",
    r.cpu >= 0 ? (sprintf(cpubuf, "%d", r.cpu), cpubuf) : "",
    r.tid ? "--tid " : "",
    r.tid ? (sprintf(tidbuf, "%d", r.tid), tidbuf) : "",
    extra_format,
    rstype == A_ASM ? "-F +disasm" :
    rstype == A_SOURCE ? "-F +srcline,+srccode" : "",
    symbol_conf.inline_name ? "--inline" : "",
    "--show-lost-events ",
    r.tid ? "--show-switch-events --show-task-events " : "",
    tsample) < 0)
    return -1;
    run_script(cmd);
    free(cmd);
    return 0;
    }
