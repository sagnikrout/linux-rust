//! Automatically rewritten from C to Rust
//! Source: kernel/debug/kdb/kdb_io.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// Kernel Debugger Architecture Independent Console I/O handler
//
// Copyright (c) 1999-2006 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//

pub const CMD_BUFLEN: c_int = 256;
    char kdb_prompt_str[CMD_BUFLEN];
    let mut kdb_trap_printk = 0;
pub static mut kdb_printf_cpu: c_int = 0;
#[no_mangle]
unsafe extern "C" fn kgdb_transition_check(buffer: *mut c_char) -> c_int {
    if (buffer[0] != '+' && buffer[0] != '$') {
    KDB_STATE_SET(KGDB_TRANS);
    kdb_printf("%s", buffer);
    } else {
pub static mut slen: c_int = 0;
    if (slen > 3 && buffer[slen - 3] == '#') {
    kdb_gdb_state_pass(buffer);
    strcpy(buffer, "kgdb");
    KDB_STATE_SET(DOING_KGDB);
    return 1;
    }
    }
    return 0;
    }
//
// kdb_handle_escape() - validity check on an accumulated escape sequence.
// @buf:	Accumulated escape characters to be examined. Note that buf
// is not a string, it is an array of characters and need not be
// nil terminated.
// @sz:		Number of accumulated escape characters.
//
// Return: -1 if the escape sequence is unwanted, 0 if it is incomplete,
// otherwise it returns a mapped key value to pass to the upper layers.
//
#[no_mangle]
unsafe extern "C" fn kdb_handle_escape(buf: *mut c_char, sz: usize) -> c_int {
    let mut lastkey = buf + sz - 1;
    match (sz) {
    1 => {
    if (*lastkey == '\e') {
    return 0;
    }
    // break;
    }
    2 => {
    if (*lastkey == '[') {
    return 0;
    }
    // break;
    }
    3 => {
    match (*lastkey) {
    'A' => {
    return 16;
    }
    'B' => {
    return 14;
    }
    'C' => {
    return 6;
    }
    'D' => {
    return 2;
    }
    '1' => {
    }
    '3' => {
    }
    '4' => {
    return 0;
    }
    }
    break;
    case 4:
    if (*lastkey == '~') {
    match (buf[2]) {
    '1' => {
    return 1;
    }
    '3' => {
    return 4;
    }
    '4' => {
    return 5;
    }
    }
    }
    break;
    }
    return -1;
    }
//
// kdb_getchar() - Read a single character from a kdb console (or consoles).
//
// Other than polling the various consoles that are currently enabled,
// most of the work done in this function is dealing with escape sequences.
//
// An escape key could be the start of a vt100 control sequence such as \e[D
// (left arrow) or it could be a character in its own right.  The standard
// method for detecting the difference is to wait for 2 seconds to see if there
// are any other characters.  kdb is complicated by the lack of a timer service
// (interrupts are off), by multiple input sources. Escape sequence processing
// has to be done as states in the polling loop.
//
// Return: The key pressed or a control code derived from an escape sequence.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_getchar() -> c_char {
pub const ESCAPE_UDELAY: c_int = 1000;

    char buf[4];	/* longest vt100 escape sequence is 4 bytes */
    let mut pbuf = buf;
pub static mut escape_delay: c_int = 0;
    get_char_func *f, *f_prev = core::ptr::null_mut();
    let mut key = 0;
    static bool last_char_was_cr;
    while ( ) {
    if (*f == core::ptr::null_mut()) {
// Reset NMI watchdog once per poll loop
    touch_nmi_watchdog();
    f = &kdb_poll_funcs[0];
    }
    key = (*f)();
    if (key == -1) {
    if (escape_delay) {
    udelay(ESCAPE_UDELAY);
    if (--escape_delay == 0) {
    return '\e';
    }
    }
    continue;
    }
//
// The caller expects that newlines are either CR or LF. However
// some terminals send _both_ CR and LF. Avoid having to handle
// this in the caller by stripping the LF if we saw a CR right
// before.
//
    if (last_char_was_cr && key == '\n') {
    last_char_was_cr = false;
    continue;
    }
    last_char_was_cr = (key == '\r');
//
// When the first character is received (or we get a change
// input source) we set ourselves up to handle an escape
// sequences (just in case).
//
    if (f_prev != f) {
    f_prev = f;
    pbuf = buf;
    escape_delay = ESCAPE_DELAY;
    }
// pbuf++ = key;
    key = kdb_handle_escape(buf, pbuf - buf);
    if (key < 0) /* no escape sequence; return best character */ {
    return buf[pbuf - buf == 2 ? 1 : 0];
    }
    if (key > 0) {
    return key;
    }
    }
    unreachable();
    }
//
// kdb_position_cursor() - Place cursor in the correct horizontal position
// @prompt: Nil-terminated string containing the prompt string
// @buffer: Nil-terminated string containing the entire command line
// @cp: Cursor position, pointer the character in buffer where the cursor
// should be positioned.
//
// The cursor is positioned by sending a carriage-return and then printing
// the content of the line until we reach the correct cursor position.
//
// There is some additional fine detail here.
//
// Firstly, even though kdb_printf() will correctly format zero-width fields
// we want the second call to kdb_printf() to be conditional. That keeps things
// a little cleaner when LOGGING=1.
//
// Secondly, we can't combine everything into one call to kdb_printf() since
// that renders into a fixed length buffer and the combined print could result
// in unwanted truncation.
//
#[no_mangle]
unsafe extern "C" fn kdb_position_cursor(prompt: *mut c_char, buffer: *mut c_char, cp: *mut c_char) {
    kdb_printf("\r%s", prompt);
    if (cp > buffer) {
    kdb_printf("%.*s", (int)(cp - buffer), buffer);
    }
    }
//
// kdb_read
//
// This function reads a string of characters, terminated by
// a newline, or by reaching the end of the supplied buffer,
// from the current kernel debugger console device.
// Parameters:
// buffer	- Address of character buffer to receive input characters.
// bufsize - size, in bytes, of the character buffer
// Returns:
// Returns a pointer to the buffer containing the received
// character string.  This string will be terminated by a
// newline character.
// Locking:
// No locks are required to be held upon entry to this
// function.  It is not reentrant - it relies on the fact
// that while kdb is running on only one "master debug" cpu.
// Remarks:
// The buffer size must be >= 2.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_read(buffer: *mut c_char, bufsize: size_t) -> *mut c_void {
    let mut cp = buffer;
    let mut bufend = buffer+bufsize-2;	// Reserve space for newline
// and null byte
pub static mut lastchar: *mut c_void = core::ptr::null_mut();
pub static mut p_tmp: *mut c_void = core::ptr::null_mut();
    let mut tmp = 0;
    static char tmpbuffer[CMD_BUFLEN];
pub static mut len: c_int = 0;
    let mut len_tmp = 0;
pub static mut tab: c_int = 0;
    let mut count = 0;
    let mut i = 0;
    let mut diag = 0;
    let mut dtab_count = 0;
    let mut key = 0;
    let mut ret = 0;
    diag = kdbgetintenv("DTABCOUNT", &dtab_count);
    if (diag) {
    dtab_count = 30;
    }
    if (len > 0) {
    cp += len;
    if (*(buffer+len-1) == '\n') {
    cp -= 1;
    }
    }
    lastchar = cp;
// cp = '\0';
    kdb_printf("%s", buffer);
// label;
    key = kdb_getchar();
    if (key != 9) {
    tab = 0;
    }
    match (key) {
    8 => {
    if (cp > buffer) {
    memmove(cp-1, cp, lastchar - cp + 1);
    lastchar -= 1;
    cp -= 1;
    kdb_printf("\b%s ", cp);
    kdb_position_cursor(kdb_prompt_str, buffer, cp);
    }
    // break;
    }
    10 => {
    }
    13 => {
// lastchar++ = '\n';
// lastchar++ = '\0';
    if (!KDB_STATE(KGDB_TRANS)) {
    KDB_STATE_SET(KGDB_TRANS);
    kdb_printf("%s", buffer);
    }
    kdb_printf("\n");
    return buffer;
    }
    4 => {
    if (cp < lastchar) {
    memmove(cp, cp+1, lastchar - cp);
    lastchar -= 1;
    kdb_printf("%s ", cp);
    kdb_position_cursor(kdb_prompt_str, buffer, cp);
    }
    // break;
    }
    1 => {
    if (cp > buffer) {
    cp = buffer;
    kdb_position_cursor(kdb_prompt_str, buffer, cp);
    }
    // break;
    }
    5 => {
    if (cp < lastchar) {
    kdb_printf("%s", cp);
    cp = lastchar;
    }
    // break;
    }
    2 => {
    if (cp > buffer) {
    kdb_printf("\b");
    cp -= 1;
    }
    // break;
    }
    14 => {
    }
    16 => {
    kdb_printf("\r%*c\r",
    (int)(strlen(kdb_prompt_str) + (lastchar - buffer)),
    ' ');
// lastchar = (char)key;
// (lastchar+1) = '\0';
    return lastchar;
    }
    6 => {
    if (cp < lastchar) {
    kdb_printf("%c", *cp);
    cp += 1;
    }
    // break;
    }
    9 => {
    if (tab < 2) {
    tab += 1;
    }
    tmp = *cp;
// cp = '\0';
    p_tmp = strrchr(buffer, ' ');
    p_tmp = (p_tmp ? p_tmp + 1 : buffer);
    strscpy(tmpbuffer, p_tmp);
// cp = tmp;
    len = strlen(tmpbuffer);
    count = kallsyms_symbol_complete(tmpbuffer, sizeof!(tmpbuffer));
    if (tab == 2 && count > 0) {
    kdb_printf("\n%d symbols are found.", count);
    if (count > dtab_count) {
    count = dtab_count;
    kdb_printf(" But only first %d symbols will"
    " be printed.\nYou can change the"
    " environment variable DTABCOUNT.",
    count);
    }
    kdb_printf("\n");
    while (i < count) {
    ret = kallsyms_symbol_next(tmpbuffer, i, sizeof!(tmpbuffer));
    if (WARN_ON!(!ret)) {
    // break;
    }
    if (ret != -E2BIG) {
    kdb_printf("%s ", tmpbuffer);
    }
    else {
    kdb_printf("%s... ", tmpbuffer);
    }
    tmpbuffer[len] = '\0';
    }
    if (i >= dtab_count) {
    kdb_printf("...");
    }
    kdb_printf("\n");
    kdb_printf("%s",  kdb_prompt_str);
    kdb_printf("%s", buffer);
    if (cp != lastchar) {
    kdb_position_cursor(kdb_prompt_str, buffer, cp);
    }
    } else if (tab != 2 && count > 0) {
// How many new characters do we want from tmpbuffer?
    len_tmp = strlen(tmpbuffer) - len;
    if (lastchar + len_tmp >= bufend) {
    len_tmp = bufend - lastchar;
    }
    if (len_tmp) {
// + 1 ensures the '\0' is memmove'd
    memmove(cp+len_tmp, cp, (lastchar-cp) + 1);
    memcpy(cp, tmpbuffer+len, len_tmp);
    kdb_printf("%s", cp);
    cp += len_tmp;
    lastchar += len_tmp;
    if (cp != lastchar) {
    kdb_position_cursor(kdb_prompt_str,
    buffer, cp);
    }
    }
    }
    kdb_nextline = 1; /* reset output line number */
    // break;
    }
    _ => {
    if (key >= 32 && lastchar < bufend) {
    if (cp < lastchar) {
    memmove(cp+1, cp, lastchar - cp + 1);
    lastchar += 1;
// cp = key;
    kdb_printf("%s", cp);
    cp += 1;
    kdb_position_cursor(kdb_prompt_str, buffer, cp);
    } else {
// ++lastchar = '\0';
// cp++ = key;
// The kgdb transition check will hide
// printed characters if we think that
// kgdb is connecting, until the check
// fails
    if (!KDB_STATE(KGDB_TRANS)) {
    if (kgdb_transition_check(buffer)) {
    return buffer;
    }
    } else {
    kdb_printf("%c", key);
    }
    }
// Special escape to kgdb
    if (lastchar - buffer >= 5 &&
    strcmp(lastchar - 5, "$?#3f") == 0) {
    kdb_gdb_state_pass(lastchar - 5);
    strcpy(buffer, "kgdb");
    KDB_STATE_SET(DOING_KGDB);
    return buffer;
    }
    if (lastchar - buffer >= 11 &&
    strcmp(lastchar - 11, "$qSupported") == 0) {
    kdb_gdb_state_pass(lastchar - 11);
    strcpy(buffer, "kgdb");
    KDB_STATE_SET(DOING_KGDB);
    return buffer;
    }
    }
    // break;
    }
    }
// goto;
    }
//
// kdb_getstr
//
// Print the prompt string and read a command from the
// input device.
//
// Parameters:
// buffer	Address of buffer to receive command
// bufsize Size of buffer in bytes
// prompt	Pointer to string to use as prompt string
// Returns:
// Pointer to command buffer.
// Locking:
// None.
// Remarks:
// For SMP kernels, the processor number will be
// substituted for %d, %x or %o in the prompt.
//
#[no_mangle]
pub unsafe extern "C" fn kdb_getstr(buffer: *mut c_char, bufsize: size_t, prompt: *mut c_char) -> *mut c_void {
    if (prompt && kdb_prompt_str != prompt) {
    strscpy(kdb_prompt_str, prompt);
    }
    kdb_printf("%s", kdb_prompt_str);
    kdb_nextline = 1;	/* Prompt and input resets line number */
    return kdb_read(buffer, bufsize);
    }
//
// kdb_input_flush
//
// Get rid of any buffered console input.
//
// Parameters:
// none
// Returns:
// nothing
// Locking:
// none
// Remarks:
// Call this function whenever you want to flush input.  If there is any
// outstanding input, it ignores all characters until there has been no
// data for approximately 1ms.
//
#[no_mangle]
unsafe extern "C" fn kdb_input_flush() {
pub static mut f: *mut c_void = core::ptr::null_mut();
    let mut res = 0;
pub static mut flush_delay: c_int = 1;
    while (flush_delay) {
    flush_delay -= 1;
// label;
    touch_nmi_watchdog();
    while (*f) {
    res = (*f)();
    if (res != -1) {
    flush_delay = 1;
// goto;
    }
    }
    if (flush_delay) {
    mdelay(1);
    }
    }
    }
//
// kdb_printf
//
// Print a string to the output device(s).
//
// Parameters:
// printf-like format and optional args.
// Returns:
// 0
// Locking:
// None.
// Remarks:
// use 'kdbcons->write()' to avoid polluting 'log_buf' with
// kdb output.
//
// If the user is doing a cmd args | grep srch
// then kdb_grepping_flag is set.
// In that case we need to accumulate full lines (ending in \n) before
// searching for the pattern.
//
    static char kdb_buffer[256];	/* A bit too big to go on stack */
    static char *next_avail = kdb_buffer;
    static int  size_avail;
    static int  suspend_grep;
//
// search arg1 to see if it contains arg2
// (kdmain.c provides flags for ^pat and pat$)
//
// return 1 for found, 0 for not found
//
#[no_mangle]
unsafe extern "C" fn kdb_search_string(searched: *mut c_char, searchfor: *mut c_char) -> c_int {
    char firstchar, *cp;
    let mut len1 = 0;
    let mut len2 = 0;
// not counting the newline at the end of "searched"
    len1 = strlen(searched)-1;
    len2 = strlen(searchfor);
    if (len1 < len2) {
    return 0;
    }
    if (kdb_grep_leading && kdb_grep_trailing && len1 != len2) {
    return 0;
    }
    if (kdb_grep_leading) {
    if (!strncmp(searched, searchfor, len2)) {
    return 1;
    }
    } else if (kdb_grep_trailing) {
    if (!strncmp(searched+len1-len2, searchfor, len2)) {
    return 1;
    }
    } else {
    firstchar = *searchfor;
    cp = searched;
    while ((cp = strchr(cp, firstchar))) {
    if (!strncmp(cp, searchfor, len2)) {
    return 1;
    }
    cp += 1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kdb_msg_write(msg: *const c_char, msg_len: c_int) {
pub static mut c: *mut c_void = core::ptr::null_mut();
pub static mut cp: *mut c_void = core::ptr::null_mut();
    let mut cookie = 0;
    let mut len = 0;
    if (msg_len == 0) {
    return;
    }
    cp = msg;
    len = msg_len;
    while (len--) {
    dbg_io_ops.write_char(*cp);
    cp += 1;
    }
//
// The console_srcu_read_lock() only provides safe console list
// traversal. The use of the ->write() callback relies on all other
// CPUs being stopped at the moment and console drivers being able to
// handle reentrance when @oops_in_progress is set.
//
// There is no guarantee that every console driver can handle
// reentrance in this way; the developer deploying the debugger
// is responsible for ensuring that the console drivers they
// have selected handle reentrance appropriately.
//
    cookie = console_srcu_read_lock();
    for_each_console_srcu(c) {
pub static mut flags: c_short = 0;
    if (!console_is_usable(c, flags, true)) {
    continue;
    }
    if (c == dbg_io_ops.cons) {
    continue;
    }
    if (flags & CON_NBCON) {
pub static mut wctxt: nbcon_write_context = 0;
//
// Do not continue if the console is NBCON and the context
// can't be acquired.
//
    if (!nbcon_kdb_try_acquire(c, &wctxt)) {
    continue;
    }
    nbcon_write_context_set_buf(&wctxt, msg, msg_len);
    c.write_atomic(c, &wctxt);
    nbcon_kdb_release(&wctxt);
    } else {
//
// Set oops_in_progress to encourage the console drivers to
// disregard their internal spin locks: in the current calling
// context the risk of deadlock is a bigger problem than risks
// due to re-entering the console driver. We operate directly on
// oops_in_progress rather than using bust_spinlocks() because
// the calls bust_spinlocks() makes on exit are not appropriate
// for this calling context.
//
    oops_in_progress += 1;
    c.write(c, msg, msg_len);
    oops_in_progress -= 1;
    }
    touch_nmi_watchdog();
    }
    console_srcu_read_unlock(cookie);
    }
#[no_mangle]
pub unsafe extern "C" fn vkdb_printf(src: kdb_msgsrc, fmt: *const c_char, ap: va_list) -> c_int {
    let mut diag = 0;
    let mut linecount = 0;
    let mut colcount = 0;
    int logging, saved_loglevel = 0;
pub static mut retlen: c_int = 0;
    let mut fnd = 0;
    let mut len = 0;
    let mut this_cpu = 0;
    let mut old_cpu = 0;
    char *cp, *cp2, *cphold = core::ptr::null_mut(), replaced_byte = ' ';
    let mut moreprompt = "more> ";
    let mut flags = 0;
// Serialize kdb_printf if multiple cpus try to write at once.
// But if any cpu goes recursive in kdb, just print the output,
// even if it is interleaved with any other text.
//
    local_irq_save(flags);
    this_cpu = smp_processor_id();
    for (;;) {
    old_cpu = cmpxchg(&kdb_printf_cpu, -1, this_cpu);
    if (old_cpu == -1 || old_cpu == this_cpu) {
    break;
    }
    cpu_relax();
    }
    diag = kdbgetintenv("LINES", &linecount);
    if (diag || linecount <= 1) {
    linecount = 24;
    }
    diag = kdbgetintenv("COLUMNS", &colcount);
    if (diag || colcount <= 1) {
    colcount = 80;
    }
    diag = kdbgetintenv("LOGGING", &logging);
    if (diag) {
    logging = 0;
    }
    if (!kdb_grepping_flag || suspend_grep) {
// normally, every vsnprintf starts a new buffer
    next_avail = kdb_buffer;
    size_avail = sizeof!(kdb_buffer);
    }
    vsnprintf(next_avail, size_avail, fmt, ap);
//
// If kdb_parse() found that the command was cmd xxx | grep yyy
// then kdb_grepping_flag is set, and kdb_grep_string contains yyy
//
// Accumulate the print data up to a newline before searching it.
// (vsnprintf does null-terminate the string that it generates)
//
// skip the search if prints are temporarily unconditional
    if (!suspend_grep && kdb_grepping_flag) {
    cp = strchr(kdb_buffer, '\n');
    if (!cp) {
//
// Special cases that don't end with newlines
// but should be written without one:
// The "[nn]kdb> " prompt should
// appear at the front of the buffer.
//
// The "[nn]more " prompt should also be
// (MOREPROMPT -> moreprompt)
// written *   but we print that ourselves,
// we set the suspend_grep flag to make
// it unconditional.
//
    if (next_avail == kdb_buffer) {
//
// these should occur after a newline,
// so they will be at the front of the
// buffer
//
    cp2 = kdb_buffer;
    len = strlen(kdb_prompt_str);
    if (!strncmp(cp2, kdb_prompt_str, len)) {
//
// We're about to start a new
// command, so we can go back
// to normal mode.
//
    kdb_grepping_flag = 0;
// goto;
    }
    }
// no newline; don't search/write the buffer
    until one is there */
    len = strlen(kdb_buffer);
    next_avail = kdb_buffer + len;
    size_avail = sizeof!(kdb_buffer) - len;
// goto;
    }
//
// The newline is present; print through it or discard
// it, depending on the results of the search.
//
    cp += 1;	 	     /* to byte after the newline */
    replaced_byte = *cp; /* remember what it was */
    cphold = cp;	     /* remember where it was */
// cp = '\0';	     // end the string for our search
//
// We now have a newline at the end of the string
// Only continue with this output if it contains the
// search string.
//
    fnd = kdb_search_string(kdb_buffer, kdb_grep_string);
    if (!fnd) {
//
// At this point the complete line at the start
// of kdb_buffer can be discarded, as it does
// not contain what the user is looking for.
// Shift the buffer left.
//
// cphold = replaced_byte;
    len = strlen(cphold);
// Use memmove() because the buffers overlap
    memmove(kdb_buffer, cphold, len + 1);
    next_avail = kdb_buffer + len;
    size_avail = sizeof!(kdb_buffer) - len;
// goto;
    }
    if (kdb_grepping_flag >= KDB_GREPPING_FLAG_SEARCH) {
//
// This was a interactive search (using '/' at more
// prompt) and it has completed. Replace the \0 with
// its original value to ensure multi-line strings
// are handled properly, and return to normal mode.
//
// cphold = replaced_byte;
    kdb_grepping_flag = 0;
    }
//
// at this point the string is a full line and
// should be printed, up to the null.
//
    }
// label;
//
// Write to all consoles.
//
    retlen = strlen(kdb_buffer);
    cp =  printk_skip_headers(kdb_buffer);
    if (!dbg_kdb_mode && kgdb_connected) {
    gdbstub_msg_write(cp, retlen - (cp - kdb_buffer));
    }
    else {
    kdb_msg_write(cp, retlen - (cp - kdb_buffer));
    }
    if (logging) {
    saved_loglevel = console_loglevel;
    console_loglevel = CONSOLE_LOGLEVEL_SILENT;
    if (printk_get_level(kdb_buffer) || src == KDB_MSGSRC_PRINTK) {
    printk("%s", kdb_buffer);
    }
    else {
    pr_info!("%s", kdb_buffer);
    }
    }
    if (KDB_STATE(PAGER)) {
//
// Check printed string to decide how to bump the
// kdb_nextline to control when the more prompt should
// show up.
//
pub static mut got: c_int = 0;
    len = retlen;
    while (len--) {
    if (kdb_buffer[len] == '\n') {
    kdb_nextline += 1;
    got = 0;
    } else if (kdb_buffer[len] == '\r') {
    got = 0;
    } else {
    got += 1;
    }
    }
    kdb_nextline += got / (colcount + 1);
    }
// check for having reached the LINES number of printed lines
    if (kdb_nextline >= linecount) {
    let mut ch = 0;
// Watch out for recursion here.  Any routine that calls
// kdb_printf will come back through here.  And kdb_read
// uses kdb_printf to echo on serial consoles ...
//
    kdb_nextline = 1;	/* In case of recursion */
//
// Pause until cr.
//
    moreprompt = kdbgetenv("MOREPROMPT");
    if (moreprompt == core::ptr::null_mut()) {
    moreprompt = "more> ";
    }
    kdb_input_flush();
    kdb_msg_write(moreprompt, strlen(moreprompt));
    if (logging) {
    printk("%s", moreprompt);
    }
    ch = kdb_getchar();
    kdb_nextline = 1;	/* Really set output line 1 */
// empty and reset the buffer:
    kdb_buffer[0] = '\0';
    next_avail = kdb_buffer;
    size_avail = sizeof!(kdb_buffer);
    if ((ch == 'q') || (ch == 'Q')) {
// user hit q or Q
    KDB_FLAG_SET(CMD_INTERRUPT); /* command interrupted */
    KDB_STATE_CLEAR(PAGER);
// end of command output; back to normal mode
    kdb_grepping_flag = 0;
    kdb_printf("\n");
    } else if (ch == ' ') {
    kdb_printf("\r");
    suspend_grep = 1; /* for this recursion */
    } else if (ch == '\n' || ch == '\r') {
    kdb_nextline = linecount - 1;
    kdb_printf("\r");
    suspend_grep = 1; /* for this recursion */
    } else if (ch == '/' && !kdb_grepping_flag) {
    kdb_printf("\r");
    kdb_getstr(kdb_grep_string, KDB_GREP_STRLEN,
    kdbgetenv("SEARCHPROMPT") ?: "search> ");
// strchrnul(kdb_grep_string, '\n') = '\0';
    kdb_grepping_flag += KDB_GREPPING_FLAG_SEARCH;
    suspend_grep = 1; /* for this recursion */
    } else if (ch) {
// user hit something unexpected
    suspend_grep = 1; /* for this recursion */
    if (ch != '/') {
    kdb_printf(
    "\nOnly 'q', 'Q' or '/' are processed at "
    "more prompt, input ignored\n");
    }
    else {
    kdb_printf("\n'/' cannot be used during | "
    "grep filtering, input ignored\n");
    }
    } else if (kdb_grepping_flag) {
// user hit enter
    suspend_grep = 1; /* for this recursion */
    kdb_printf("\n");
    }
    kdb_input_flush();
    }
//
// For grep searches, shift the printed string left.
// replaced_byte contains the character that was overwritten with
// the terminating null, and cphold points to the null.
// Then adjust the notion of available space in the buffer.
//
    if (kdb_grepping_flag && !suspend_grep) {
// cphold = replaced_byte;
    len = strlen(cphold);
// Use memmove() because the buffers overlap
    memmove(kdb_buffer, cphold, len + 1);
    next_avail = kdb_buffer + len;
    size_avail = sizeof!(kdb_buffer) - len;
    }
// label;
    suspend_grep = 0; /* end of what may have been a recursive call */
    if (logging) {
    console_loglevel = saved_loglevel;
    }
// kdb_printf_cpu locked the code above.
    smp_store_release(&kdb_printf_cpu, old_cpu);
    local_irq_restore(flags);
    return retlen;
    }
#[no_mangle]
pub unsafe extern "C" fn kdb_printf(fmt: *const c_char, ...) -> c_int {
    let mut ap;
    let mut r = 0;
    va_start(ap, fmt);
    r = vkdb_printf(KDB_MSGSRC_INTERNAL, fmt, ap);
    va_end(ap);
    return r;
    }
    EXPORT_SYMBOL_GPL(kdb_printf);
}
