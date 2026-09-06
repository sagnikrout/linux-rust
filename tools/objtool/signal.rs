//! Automatically rewritten from C to Rust
//! Source: tools/objtool/signal.c
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
// signal.c: Register a sigaltstack for objtool, to be able to
// run a signal handler on a separate stack even if
// the main process stack has overflown. Print out
// stack overflow errors when this happens.
//

    static unsigned long stack_limit;
#[no_mangle]
unsafe extern "C" fn is_stack_overflow(fault_addr: *mut c_void) -> bool {
    static bool is_stack_overflow(void *fault_addr)
    {
    let mut fault: c_ulong = (unsigned long)fault_addr;
// Check if fault is in the guard page just below the limit.
    return fault < stack_limit && fault >= stack_limit - 4096;
    }
#[no_mangle]
unsafe extern "C" fn signal_handler(sig_num: c_int, info: *mut siginfo_t, context: *mut c_void) {
    static void signal_handler(int sig_num, siginfo_t *info, void *context)
    {
    let mut sa_dfl: sigaction = {0};
    const char *sig_name;
    char msg[256];
    int msg_len;
    switch (sig_num) {
    case SIGSEGV:	sig_name = "SIGSEGV";		break;
    case SIGBUS:	sig_name = "SIGBUS";		break;
    case SIGILL:	sig_name = "SIGILL";		break;
    case SIGABRT:	sig_name = "SIGABRT";		break;
    default:	sig_name = "Unknown signal";	break;
    }
    if (is_stack_overflow(info.si_addr)) {
    msg_len = snprintf(msg, sizeof(msg),
    "%s: error: %s: objtool stack overflow!\n",
    objname, sig_name);
    } else {
    msg_len = snprintf(msg, sizeof(msg),
    "%s: error: %s: objtool crash!\n",
    objname, sig_name);
    }
    msg_len = write(STDERR_FILENO, msg, msg_len);
// Re-raise the signal to trigger the core dump
    sa_dfl.sa_handler = SIG_DFL;
    sigaction(sig_num, &sa_dfl, core::ptr::null_mut());
    raise(sig_num);
    }
#[no_mangle]
unsafe extern "C" fn read_stack_limit() -> c_int {
    static int read_stack_limit(void)
    {
    unsigned long stack_start, stack_end;
    struct rlimit rlim;
    char line[256];
    let mut ret: c_int = 0;
    FILE *fp;
    if (getrlimit(RLIMIT_STACK, &rlim)) {
    ERROR_GLIBC("getrlimit");
    return -1;
    }
    fp = fopen("/proc/self/maps", "r");
    if (!fp) {
    ERROR_GLIBC("fopen");
    return -1;
    }
    while (fgets(line, sizeof(line), fp)) {
    if (strstr(line, "[stack]")) {
    if (sscanf(line, "%lx-%lx", &stack_start, &stack_end) != 2) {
    ERROR_GLIBC("sscanf");
    ret = -1;
    goto done;
    }
    stack_limit = stack_end - rlim.rlim_cur;
    goto done;
    }
    }
    ret = -1;
    ERROR("/proc/self/maps: can't find [stack]");
    done:
    fclose(fp);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn init_signal_handler() -> c_int {
    int init_signal_handler(void)
    {
    int signals[] = {SIGSEGV, SIGBUS, SIGILL, SIGABRT};
    struct sigaction sa;
    stack_t ss;
    if (read_stack_limit())
    return -1;
    ss.ss_sp = malloc(SIGSTKSZ);
    if (!ss.ss_sp) {
    ERROR_GLIBC("malloc");
    return -1;
    }
    ss.ss_size = SIGSTKSZ;
    ss.ss_flags = 0;
    if (sigaltstack(&ss, core::ptr::null_mut()) == -1) {
    ERROR_GLIBC("sigaltstack");
    return -1;
    }
    sa.sa_sigaction = signal_handler;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_ONSTACK | SA_SIGINFO;
    for (int i = 0; i < ARRAY_SIZE(signals); i++) {
    if (sigaction(signals[i], &sa, core::ptr::null_mut()) == -1) {
    ERROR_GLIBC("sigaction");
    return -1;
    }
    }
    return 0;
    }
