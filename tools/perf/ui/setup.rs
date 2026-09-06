//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/setup.c
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

    struct mutex ui__lock;
    void *perf_gtk_handle;
    let mut use_browser: c_int = -1;

#[no_mangle]
unsafe extern "C" fn setup_gtk_browser() -> c_int {
    static int setup_gtk_browser(void)
    {
    int (*perf_ui_init)(void);
    if (perf_gtk_handle)
    return 0;
    perf_gtk_handle = dlopen(PERF_GTK_DSO, RTLD_LAZY);
    if (perf_gtk_handle == core::ptr::null_mut()) {
    char buf[PATH_MAX];
    scnprintf(buf, sizeof(buf), "%s/%s", LIBDIR, PERF_GTK_DSO);
    perf_gtk_handle = dlopen(buf, RTLD_LAZY);
    }
    if (perf_gtk_handle == core::ptr::null_mut())
    return -1;
    perf_ui_init = dlsym(perf_gtk_handle, "perf_gtk__init");
    if (perf_ui_init == core::ptr::null_mut())
    goto out_close;
    if (perf_ui_init() == 0)
    return 0;
    out_close:
    dlclose(perf_gtk_handle);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn exit_gtk_browser(wait_for_ok: bool) {
    static void exit_gtk_browser(bool wait_for_ok)
    {
    void (*perf_ui_exit)(bool);
    if (perf_gtk_handle == core::ptr::null_mut())
    return;
    perf_ui_exit = dlsym(perf_gtk_handle, "perf_gtk__exit");
    if (perf_ui_exit == core::ptr::null_mut())
    goto out_close;
    perf_ui_exit(wait_for_ok);
    out_close:
    dlclose(perf_gtk_handle);
    perf_gtk_handle = core::ptr::null_mut();
    }

    static inline int setup_gtk_browser(void) { return -1; }
    static inline void exit_gtk_browser(bool wait_for_ok __maybe_unused) {}

    int stdio__config_color(const struct option *opt __maybe_unused,
    const char *mode, int unset __maybe_unused)
    {
    perf_use_color_default = perf_config_colorbool("color.ui", mode, -1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn setup_browser(fallback_to_pager: bool) {
    void setup_browser(bool fallback_to_pager)
    {
    mutex_init(&ui__lock);
    if (use_browser < 2 && (!isatty(1) || dump_trace))
    use_browser = 0;
// default to TUI
    if (use_browser < 0)
    use_browser = 1;
    switch (use_browser) {
    case 2:
    if (setup_gtk_browser() == 0)
    break;
    printf("GTK browser requested but could not find %s\n",
    PERF_GTK_DSO);
    sleep(1);
    use_browser = 1;
// fall through
    case 1:
    if (ui__init() == 0)
    break;
// fall through
    default:
    use_browser = 0;
    if (fallback_to_pager)
    setup_pager();
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn exit_browser(wait_for_ok: bool) {
    void exit_browser(bool wait_for_ok)
    {
    switch (use_browser) {
    case 2:
    exit_gtk_browser(wait_for_ok);
    break;
    case 1:
    ui__exit(wait_for_ok);
    break;
    default:
    break;
    }
    mutex_destroy(&ui__lock);
    }
#[no_mangle]
pub unsafe extern "C" fn pthread__block_sigwinch() {
    void pthread__block_sigwinch(void)
    {
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGWINCH);
    pthread_sigmask(SIG_BLOCK, &set, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn pthread__unblock_sigwinch() {
    void pthread__unblock_sigwinch(void)
    {
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGWINCH);
    pthread_sigmask(SIG_UNBLOCK, &set, core::ptr::null_mut());
    }
