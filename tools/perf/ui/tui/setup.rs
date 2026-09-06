//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/tui/setup.c
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


    static volatile int ui__need_resize;
    extern struct perf_error_ops perf_tui_eops;
    extern bool tui_helpline__set;
    extern void hist_browser__init_hpp(void);
#[no_mangle]
pub unsafe extern "C" fn ui__refresh_dimensions(force: bool) {
    void ui__refresh_dimensions(bool force)
    {
    if (force || ui__need_resize) {
    ui__need_resize = 0;
    mutex_lock(&ui__lock);
    SLtt_get_screen_size();
    SLsmg_reinit_smg();
    mutex_unlock(&ui__lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn ui__sigwinch(__maybe_unused: int sig) {
    static void ui__sigwinch(int sig __maybe_unused)
    {
    ui__need_resize = 1;
    }
#[no_mangle]
unsafe extern "C" fn ui__setup_sigwinch() {
    static void ui__setup_sigwinch(void)
    {
    static bool done;
    if (done)
    return;
    done = true;
    pthread__unblock_sigwinch();
    signal(SIGWINCH, ui__sigwinch);
    }
#[no_mangle]
pub unsafe extern "C" fn ui__getch(delay_secs: c_int) -> c_int {
    int ui__getch(int delay_secs)
    {
    struct timeval timeout, *ptimeout = delay_secs ? &timeout : core::ptr::null_mut();
    fd_set read_set;
    int err, key;
    ui__setup_sigwinch();
    FD_ZERO(&read_set);
    FD_SET(0, &read_set);
    if (delay_secs) {
    timeout.tv_sec = delay_secs;
    timeout.tv_usec = 0;
    }
    err = select(1, &read_set, core::ptr::null_mut(), core::ptr::null_mut(), ptimeout);
    if (err == 0)
    return K_TIMER;
    if (err == -1) {
    if (errno == EINTR)
    return K_RESIZE;
    return K_ERROR;
    }
    key = SLang_getkey();
    if (key != K_ESC)
    return key;
    FD_ZERO(&read_set);
    FD_SET(0, &read_set);
    timeout.tv_sec = 0;
    timeout.tv_usec = 20;
    err = select(1, &read_set, core::ptr::null_mut(), core::ptr::null_mut(), &timeout);
    if (err == 0)
    return K_ESC;
    SLang_ungetkey(key);
    return SLkp_getkey();
    }

#[no_mangle]
unsafe extern "C" fn ui__signal_backtrace(sig: c_int) {
    static void ui__signal_backtrace(int sig)
    {
    void *stackdump[32];
    size_t size;
    ui__exit(false);
    psignal(sig, "perf");
    printf("-------- backtrace --------\n");
    size = backtrace(stackdump, ARRAY_SIZE(stackdump));
    __dump_stack(stdout, stackdump, size);
    exit(0);
    }

#[no_mangle]
unsafe extern "C" fn ui__signal(sig: c_int) {
    static void ui__signal(int sig)
    {
    ui__exit(false);
    psignal(sig, "perf");
    exit(0);
    }
#[no_mangle]
unsafe extern "C" fn ui__sigcont(sig: c_int) {
    static void ui__sigcont(int sig)
    {
    static struct termios tty;
    if (sig == SIGTSTP) {
    while (tcgetattr(SLang_TT_Read_FD, &tty) == -1 && errno == EINTR)
    ;
    while (write(SLang_TT_Read_FD, PERF_COLOR_RESET, sizeof(PERF_COLOR_RESET) - 1) == -1 && errno == EINTR)
    ;
    raise(SIGSTOP);
    } else {
    while (tcsetattr(SLang_TT_Read_FD, TCSADRAIN, &tty) == -1 && errno == EINTR)
    ;
    raise(SIGWINCH);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ui__init() -> c_int {
    int ui__init(void)
    {
    int err;
    SLutf8_enable(-1);
    SLtt_get_terminfo();
    SLtt_get_screen_size();
    err = SLsmg_init_smg();
    if (err < 0)
    goto out;
    err = SLang_init_tty(-1, 0, 0);
    if (err < 0)
    goto out;
    SLtty_set_suspend_state(true);
    err = SLkp_init();
    if (err < 0) {
    pr_err("TUI initialization failed.\n");
    goto out;
    }
    SLkp_define_keysym("^(kB)", SL_KEY_UNTAB);
    signal(SIGSEGV, ui__signal_backtrace);
    signal(SIGFPE, ui__signal_backtrace);
    signal(SIGINT, ui__signal);
    signal(SIGQUIT, ui__signal);
    signal(SIGTERM, ui__signal);
    signal(SIGTSTP, ui__sigcont);
    signal(SIGCONT, ui__sigcont);
    perf_error__register(&perf_tui_eops);
    ui_helpline__init();
    ui_browser__init();
    tui_progress__init();
    hist_browser__init_hpp();
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn ui__exit(wait_for_ok: bool) {
    void ui__exit(bool wait_for_ok)
    {
    if (wait_for_ok && tui_helpline__set)
    ui__question_window("Fatal Error",
    ui_helpline__last_msg,
    "Press any key...", 0);
    SLtt_set_cursor_visibility(1);
    if (mutex_trylock(&ui__lock)) {
    SLsmg_refresh();
    SLsmg_reset_smg();
    mutex_unlock(&ui__lock);
    }
    SLang_reset_tty();
    perf_error__unregister(&perf_tui_eops);
    }
