//! Automatically rewritten from C to Rust
//! Source: kernel/debug/gdbstub.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Kernel Debug Core
//
// Maintainer: Jason Wessel <jason.wessel@windriver.com>
//
// Copyright (C) 2000-2001 VERITAS Software Corporation.
// Copyright (C) 2002-2004 Timesys Corporation
// Copyright (C) 2003-2004 Amit S. Kale <amitkale@linsyssoft.com>
// Copyright (C) 2004 Pavel Machek <pavel@ucw.cz>
// Copyright (C) 2004-2006 Tom Rini <trini@kernel.crashing.org>
// Copyright (C) 2004-2006 LinSysSoft Technologies Pvt. Ltd.
// Copyright (C) 2005-2009 Wind River Systems, Inc.
// Copyright (C) 2007 MontaVista Software, Inc.
// Copyright (C) 2008 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//
// Contributors at various stages not listed above:
// Jason Wessel ( jason.wessel@windriver.com )
// George Anzinger <george@mvista.com>
// Anurekh Saxena (anurekh.saxena@timesys.com)
// Lake Stevens Instrument Division (Glenn Engel)
// Jim Kingdon, Cygnus Support.
//
// Original KGDB stub: David Grothe <dave@gcom.com>,
// Tigran Aivazian <tigran@sco.com>
//

pub const KGDB_MAX_THREAD_QUERY: c_int = 17;
// Our I/O buffers.
    static char			remcom_in_buffer[BUFMAX];
    static char			remcom_out_buffer[BUFMAX];
    static int			gdbstub_use_prev_in_buf;
    static int			gdbstub_prev_in_buf_pos;
// Storage for the registers, in GDB format.
    static unsigned long		gdb_regs[(NUMREGBYTES +
    sizeof!(unsigned long) - 1) /
    sizeof!(unsigned long)];
//
// GDB remote protocol parser:
//

#[no_mangle]
unsafe extern "C" fn gdbstub_read_wait() -> c_int {
pub static mut ret: c_int = 0;
    let mut i = 0;
    if (unlikely(gdbstub_use_prev_in_buf)) {
    if (gdbstub_prev_in_buf_pos < gdbstub_use_prev_in_buf) {
    return remcom_in_buffer[gdbstub_prev_in_buf_pos++];
    }
    else {
    gdbstub_use_prev_in_buf = 0;
    }
    }
// poll any additional I/O interfaces that are defined
    while (ret < 0) {
    while (kdb_poll_funcs[i] != core::ptr::null_mut()) {
    }
    ret = kdb_poll_funcs[i]();
    if (ret > 0) {
    break;
    }
    }
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn gdbstub_read_wait() -> c_int {
pub static mut ret: c_int = 0;
    while (ret == NO_POLL_CHAR) {
    ret = dbg_io_ops.read_char();
    }
    return ret;
    }

// scan for the sequence $<data>#<checksum>
#[no_mangle]
unsafe extern "C" fn get_packet(buffer: *mut c_char) {
    let mut checksum = 0;
    let mut xmitcsum = 0;
    let mut count = 0;
    let mut ch = 0;
    do {
//
// Spin and wait around for the start character, ignore all
// other characters:
//
    while ((ch = (gdbstub_read_wait())) != '$') {
// nothing */;
    }
    kgdb_connected = 1;
    checksum = 0;
    xmitcsum = -1;
    count = 0;
//
// now, read until a # or end of buffer is found:
//
    while (count < (BUFMAX - 1)) {
    ch = gdbstub_read_wait();
    if (ch == '#') {
    break;
    }
    checksum = checksum + ch;
    buffer[count] = ch;
    count = count + 1;
    }
    if (ch == '#') {
    xmitcsum = hex_to_bin(gdbstub_read_wait()) << 4;
    xmitcsum += hex_to_bin(gdbstub_read_wait());
    if (checksum != xmitcsum) {
// failed checksum
    dbg_io_ops.write_char('-');
    }
    else {
// successful transfer
    dbg_io_ops.write_char('+');
    }
    if (dbg_io_ops.flush) {
    dbg_io_ops.flush();
    }
    }
    buffer[count] = 0;
    } while (checksum != xmitcsum);
    }
//
// Send the packet in buffer.
// Check for gdb connection if asked for.
//
#[no_mangle]
unsafe extern "C" fn put_packet(buffer: *mut c_char) {
    let mut checksum = 0;
    let mut count = 0;
    let mut ch = 0;
//
// $<packet info>#<checksum>.
//
    while (1) {
    dbg_io_ops.write_char('$');
    checksum = 0;
    count = 0;
    while ((ch = buffer[count])) {
    dbg_io_ops.write_char(ch);
    checksum += ch;
    count += 1;
    }
    dbg_io_ops.write_char('#');
    dbg_io_ops.write_char(hex_asc_hi(checksum));
    dbg_io_ops.write_char(hex_asc_lo(checksum));
    if (dbg_io_ops.flush) {
    dbg_io_ops.flush();
    }
// Now see what we get in reply.
    ch = gdbstub_read_wait();
    if (ch == 3) {
    ch = gdbstub_read_wait();
    }
// If we get an ACK, we are done.
    if (ch == '+') {
    return;
    }
//
// If we get the start of another packet, this means
// that GDB is attempting to reconnect.  We will NAK
// the packet being sent, and stop trying to send this
// packet.
//
    if (ch == '$') {
    dbg_io_ops.write_char('-');
    if (dbg_io_ops.flush) {
    dbg_io_ops.flush();
    }
    return;
    }
    }
    }
    static char gdbmsgbuf[BUFMAX + 1];
#[no_mangle]
pub unsafe extern "C" fn gdbstub_msg_write(s: *const c_char, len: c_int) {
pub static mut bufptr: *mut c_void = core::ptr::null_mut();
    let mut wcount = 0;
    let mut i = 0;
    if (len == 0) {
    len = strlen(s);
    }
// 'O'utput
    gdbmsgbuf[0] = 'O';
// Fill and send buffers...
    while (len > 0) {
    bufptr = gdbmsgbuf + 1;
// Calculate how many this time
    if ((len << 1) > (BUFMAX - 2)) {
    wcount = (BUFMAX - 2) >> 1;
    }
    else {
    wcount = len;
    }
// Pack in hex chars
    for (i = 0; i < wcount; i++) {
    bufptr = hex_byte_pack(bufptr, s[i]);
    }
// bufptr = '\0';
// Move up
    s += wcount;
    len -= wcount;
// Write packet
    put_packet(gdbmsgbuf);
    }
    }
//
// Convert the memory pointed to by mem into hex, placing result in
// buf.  Return a pointer to the last char put in buf (null). May
// return an error.
//
#[no_mangle]
pub unsafe extern "C" fn kgdb_mem2hex(mem: *mut c_char, buf: *mut c_char, count: c_int) -> *mut c_void {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
//
// We use the upper half of buf as an intermediate buffer for the
// raw memory copy.  Hex conversion will work against this one.
//
    tmp = buf + count;
    err = copy_from_kernel_nofault(tmp, mem, count);
    if (err) {
    return core::ptr::null_mut();
    }
    while (count > 0) {
    buf = hex_byte_pack(buf, *tmp);
    tmp += 1;
    count -= 1;
    }
// buf = 0;
    return buf;
    }
//
// Convert the hex array pointed to by buf into binary to be placed in
// mem.  Return a pointer to the character AFTER the last byte
// written.  May return an error.
//
#[no_mangle]
pub unsafe extern "C" fn kgdb_hex2mem(buf: *mut c_char, mem: *mut c_char, count: c_int) -> c_int {
pub static mut tmp_raw: *mut c_void = core::ptr::null_mut();
pub static mut tmp_hex: *mut c_void = core::ptr::null_mut();
//
// We use the upper half of buf as an intermediate buffer for the
// raw memory that is converted from hex.
//
    tmp_raw = buf + count * 2;
    tmp_hex = tmp_raw - 1;
    while (tmp_hex >= buf) {
    tmp_raw -= 1;
// tmp_raw = hex_to_bin(*tmp_hex--);
// tmp_raw |= hex_to_bin(*tmp_hex--) << 4;
    }
    return copy_to_kernel_nofault(mem, tmp_raw, count);
    }
//
// While we find nice hex chars, build a long_val.
// Return number of chars processed.
//
#[no_mangle]
pub unsafe extern "C" fn kgdb_hex2long(ptr: *mut c_char, long_val: *mut c_ulong) -> c_int {
    let mut hex_val = 0;
pub static mut num: c_int = 0;
pub static mut negate: c_int = 0;
// long_val = 0;
    if (**ptr == '-') {
    negate = 1;
    (*ptr)++;
    }
    while (**ptr) {
    hex_val = hex_to_bin(**ptr);
    if (hex_val < 0) {
    break;
    }
// long_val = (*long_val << 4) | hex_val;
    num += 1;
    (*ptr)++;
    }
    if (negate) {
// long_val = -*long_val;
    }
    return num;
    }
//
// Copy the binary array pointed to by buf into mem.  Fix $, #, and
// 0x7d escaped with 0x7d. Return -EFAULT on failure or 0 on success.
// The input buf is overwritten with the result to write to mem.
//
#[no_mangle]
unsafe extern "C" fn kgdb_ebin2mem(buf: *mut c_char, mem: *mut c_char, count: c_int) -> c_int {
pub static mut size: c_int = 0;
    let mut c = buf;
    while (count-- > 0) {
    c[size] = *buf += 1;
    if (c[size] == 0x7d) {
    c[size] = *buf++ ^ 0x20;
    }
    size += 1;
    }
    return copy_to_kernel_nofault(mem, c, size);
    }

#[no_mangle]
pub unsafe extern "C" fn pt_regs_to_gdb_regs(gdb_regs: *mut c_ulong, regs: *mut pt_regs) {
    let mut i = 0;
pub static mut idx: c_int = 0;
    let mut ptr = gdb_regs;
    while (i < DBG_MAX_REG_NUM) {
    dbg_get_reg(i, ptr + idx, regs);
    idx += dbg_reg_def[i].size;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn gdb_regs_to_pt_regs(gdb_regs: *mut c_ulong, regs: *mut pt_regs) {
    let mut i = 0;
pub static mut idx: c_int = 0;
    let mut ptr = gdb_regs;
    while (i < DBG_MAX_REG_NUM) {
    dbg_set_reg(i, ptr + idx, regs);
    idx += dbg_reg_def[i].size;
    }
    }

// Write memory due to an 'M' or 'X' packet.
#[no_mangle]
unsafe extern "C" fn write_mem_msg(binary: c_int) -> c_int {
    let mut ptr = &remcom_in_buffer[1];
    let mut addr = 0;
    let mut length = 0;
    let mut err = 0;
    if (kgdb_hex2long(&ptr, &addr) > 0 && *(ptr++) == ',' &&
    kgdb_hex2long(&ptr, &length) > 0 && *(ptr++) == ':') {
    if (binary) {
    err = kgdb_ebin2mem(ptr, addr, length);
    }
    else {
    err = kgdb_hex2mem(ptr, addr, length);
    }
    if (err) {
    return err;
    }
    if (CACHE_FLUSH_IS_SAFE) {
    flush_icache_range(addr, addr + length);
    }
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn error_packet(pkt: *mut c_char, error: c_int) {
    error = -error;
    pkt[0] = 'E';
    pkt[1] = hex_asc[(error / 10)];
    pkt[2] = hex_asc[(error % 10)];
    pkt[3] = '\0';
    }
//
// Thread ID accessors. We represent a flat TID space to GDB, where
// the per CPU idle threads (which under Linux all have PID 0) are
// remapped to negative TIDs.
//
pub const BUF_THREAD_ID_SIZE: c_int = 8;
#[no_mangle]
pub unsafe extern "C" fn pack_threadid(pkt: *mut c_char, id: *mut c_uchar) -> *mut c_void {
pub static mut limit: *mut c_void = core::ptr::null_mut();
pub static mut lzero: c_int = 1;
    limit = id + (BUF_THREAD_ID_SIZE / 2);
    while (id < limit) {
    if (!lzero || *id != 0) {
    pkt = hex_byte_pack(pkt, *id);
    lzero = 0;
    }
    id += 1;
    }
    if (lzero) {
    pkt = hex_byte_pack(pkt, 0);
    }
    return pkt;
    }
#[no_mangle]
unsafe extern "C" fn int_to_threadref(id: *mut c_uchar, value: c_int) {
    put_unaligned_be32(value, id);
    }
#[no_mangle]
pub unsafe extern "C" fn getthread(regs: *mut pt_regs, tid: c_int) -> *mut c_void {
//
// Non-positive TIDs are remapped to the cpu shadow information
//
    if (tid == 0 || tid == -1) {
    tid = -atomic_read(&kgdb_active) - 2;
    }
    if (tid < -1 && tid > -NR_CPUS - 2) {
    if (kgdb_info[-tid - 2].task) {
    return kgdb_info[-tid - 2].task;
    }
    else {
    return idle_task(-tid - 2);
    }
    }
    if (tid <= 0) {
    printk("KGDB: Internal thread select error\n");
    dump_stack();
    return core::ptr::null_mut();
    }
//
// find_task_by_pid_ns() does not take the tasklist lock anymore
// but is nicely RCU locked - hence is a pretty resilient
// thing to use:
//
    return find_task_by_pid_ns(tid, &init_pid_ns);
    }
//
// Remap normal tasks to their real PID,
// CPU shadow threads are mapped to -CPU - 2
//
#[no_mangle]
pub unsafe extern "C" fn shadow_pid(realpid: c_int) -> c_int {
    if (realpid) {
    return realpid;
    }
    return -raw_smp_processor_id() - 2;
    }
//
// All the functions that start with gdb_cmd are the various
// operations to implement the handlers for the gdbserial protocol
// where KGDB is communicating with an external debugger
//
// Handle the '?' status packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_status(ks: *mut kgdb_state) {
//
// We know that this packet is only sent
// during initial connect.  So to be safe,
// we clear out our breakpoints now in case
// GDB is reconnecting.
//
    dbg_remove_all_break();
    remcom_out_buffer[0] = 'S';
    hex_byte_pack(&remcom_out_buffer[1], ks.signo);
    }
#[no_mangle]
unsafe extern "C" fn gdb_get_regs_helper(ks: *mut kgdb_state) {
pub static mut thread: *mut c_void = core::ptr::null_mut();
pub static mut local_debuggerinfo: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    thread = kgdb_usethread;
    if (!thread) {
    thread = kgdb_info[ks.cpu].task;
    local_debuggerinfo = kgdb_info[ks.cpu].debuggerinfo;
    } else {
    local_debuggerinfo = core::ptr::null_mut();
    for_each_online_cpu(i) {
//
// Try to find the task on some other
// or possibly this node if we do not
// find the matching task then we try
// to approximate the results.
//
    if (thread == kgdb_info[i].task) {
    local_debuggerinfo = kgdb_info[i].debuggerinfo;
    }
    }
    }
//
// All threads that don't have debuggerinfo should be
// in schedule() sleeping, since all other CPUs
// are in kgdb_cpu_enter(), and thus have debuggerinfo.
//
    if (local_debuggerinfo) {
    pt_regs_to_gdb_regs(gdb_regs, local_debuggerinfo);
    } else {
//
// Pull stuff saved during switch_to; nothing
// else is accessible (or even particularly
// relevant).
//
// This should be enough for a stack trace.
//
    sleeping_thread_to_gdb_regs(gdb_regs, thread);
    }
    }
// Handle the 'g' get registers request
#[no_mangle]
unsafe extern "C" fn gdb_cmd_getregs(ks: *mut kgdb_state) {
    gdb_get_regs_helper(ks);
    kgdb_mem2hex(gdb_regs, remcom_out_buffer, NUMREGBYTES);
    }
// Handle the 'G' set registers request
#[no_mangle]
unsafe extern "C" fn gdb_cmd_setregs(ks: *mut kgdb_state) {
    kgdb_hex2mem(&remcom_in_buffer[1], gdb_regs, NUMREGBYTES);
    if (kgdb_usethread && kgdb_usethread != current) {
    error_packet(remcom_out_buffer, -EINVAL);
    } else {
    gdb_regs_to_pt_regs(gdb_regs, ks.linux_regs);
    strscpy(remcom_out_buffer, "OK");
    }
    }
// Handle the 'm' memory read bytes
#[no_mangle]
unsafe extern "C" fn gdb_cmd_memread(ks: *mut kgdb_state) {
    let mut ptr = &remcom_in_buffer[1];
    let mut length = 0;
    let mut addr = 0;
pub static mut err: *mut c_void = core::ptr::null_mut();
    if (kgdb_hex2long(&ptr, &addr) > 0 && *ptr++ == ',' &&
    kgdb_hex2long(&ptr, &length) > 0) {
    err = kgdb_mem2hex(addr, remcom_out_buffer, length);
    if (!err) {
    error_packet(remcom_out_buffer, -EINVAL);
    }
    } else {
    error_packet(remcom_out_buffer, -EINVAL);
    }
    }
// Handle the 'M' memory write bytes
#[no_mangle]
unsafe extern "C" fn gdb_cmd_memwrite(ks: *mut kgdb_state) {
pub static mut err: c_int = 0;
    if (err) {
    error_packet(remcom_out_buffer, err);
    }
    else {
    strscpy(remcom_out_buffer, "OK");
    }
    }

#[no_mangle]
pub unsafe extern "C" fn gdb_hex_reg_helper(regnum: c_int, out: *mut c_char) -> *mut c_void {
    let mut i = 0;
pub static mut offset: c_int = 0;
    for (i = 0; i < regnum; i++) {
    offset += dbg_reg_def[i].size;
    }
    return kgdb_mem2hex(gdb_regs + offset, out,
    dbg_reg_def[i].size);
    }
// Handle the 'p' individual register get
#[no_mangle]
unsafe extern "C" fn gdb_cmd_reg_get(ks: *mut kgdb_state) {
    let mut regnum = 0;
    let mut ptr = &remcom_in_buffer[1];
    kgdb_hex2long(&ptr, &regnum);
    if (regnum >= DBG_MAX_REG_NUM) {
    error_packet(remcom_out_buffer, -EINVAL);
    return;
    }
    gdb_get_regs_helper(ks);
    gdb_hex_reg_helper(regnum, remcom_out_buffer);
    }
// Handle the 'P' individual register set
#[no_mangle]
unsafe extern "C" fn gdb_cmd_reg_set(ks: *mut kgdb_state) {
    let mut regnum = 0;
    let mut ptr = &remcom_in_buffer[1];
pub static mut i: c_int = 0;
    kgdb_hex2long(&ptr, &regnum);
    if (*ptr++ != '=' ||
    !(!kgdb_usethread || kgdb_usethread == current) ||
    !dbg_get_reg(regnum, gdb_regs, ks.linux_regs)) {
    error_packet(remcom_out_buffer, -EINVAL);
    return;
    }
    memset(gdb_regs, 0, sizeof!(gdb_regs));
    while (i < sizeof!(gdb_regs) * 2) {
    if (hex_to_bin(ptr[i]) >= 0)
    i += 1;
    }
    else {
    break;
    }
    i = i / 2;
    kgdb_hex2mem(ptr, gdb_regs, i);
    dbg_set_reg(regnum, gdb_regs, ks.linux_regs);
    strscpy(remcom_out_buffer, "OK");
    }

// Handle the 'X' memory binary write bytes
#[no_mangle]
unsafe extern "C" fn gdb_cmd_binwrite(ks: *mut kgdb_state) {
pub static mut err: c_int = 0;
    if (err) {
    error_packet(remcom_out_buffer, err);
    }
    else {
    strscpy(remcom_out_buffer, "OK");
    }
    }
// Handle the 'D' or 'k', detach or kill packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_detachkill(ks: *mut kgdb_state) {
    let mut error = 0;
// The detach case
    if (remcom_in_buffer[0] == 'D') {
    error = dbg_remove_all_break();
    if (error < 0) {
    error_packet(remcom_out_buffer, error);
    } else {
    strscpy(remcom_out_buffer, "OK");
    kgdb_connected = 0;
    }
    put_packet(remcom_out_buffer);
    } else {
//
// Assume the kill case, with no exit code checking,
// trying to force detach the debugger:
//
    dbg_remove_all_break();
    kgdb_connected = 0;
    }
    }
// Handle the 'R' reboot packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_reboot(ks: *mut kgdb_state) -> c_int {
// For now, only honor R0
    if (strcmp(remcom_in_buffer, "R0") == 0) {
    printk("Executing emergency reboot\n");
    strscpy(remcom_out_buffer, "OK");
    put_packet(remcom_out_buffer);
//
// Execution should not return from
// machine_emergency_restart()
//
    machine_emergency_restart();
    kgdb_connected = 0;
    return 1;
    }
    return 0;
    }
// Handle the 'q' query packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_query(ks: *mut kgdb_state) {
pub static mut g: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    unsigned char thref[BUF_THREAD_ID_SIZE];
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut cpu = 0;
pub static mut finished: c_int = 0;
    match (remcom_in_buffer[1]) {
    's' => {
    }
    'f' => {
    if (memcmp(remcom_in_buffer + 2, "ThreadInfo", 10)) {
    // break;
    }
    i = 0;
    remcom_out_buffer[0] = 'm';
    ptr = remcom_out_buffer + 1;
    if (remcom_in_buffer[1] == 'f') {
// Each cpu is a shadow thread
    for_each_online_cpu(cpu) {
    ks.thr_query = 0;
// forward_decl: _to_threadref;
    ptr = pack_threadid(ptr, thref);
// (ptr++) = ',';
    i += 1;
    }
    }
    for_each_process_thread(g, p) {
    if (i >= ks.thr_query && !finished) {
// forward_decl: _to_threadref;
    ptr = pack_threadid(ptr, thref);
// (ptr++) = ',';
    ks.thr_query += 1;
    if (ks.thr_query % KGDB_MAX_THREAD_QUERY == 0) {
    finished = 1;
    }
    }
    i += 1;
    }
// (--ptr) = '\0';
    // break;
    }
    'C' => {
// Current thread id
    strscpy(remcom_out_buffer, "QC");
    ks.threadid = shadow_pid(current.pid);
// forward_decl: _to_threadref;
    pack_threadid(remcom_out_buffer + 2, thref);
    // break;
    }
    'T' => {
    if (memcmp(remcom_in_buffer + 1, "ThreadExtraInfo,", 16)) {
    // break;
    }
    ks.threadid = 0;
    ptr = remcom_in_buffer + 17;
    kgdb_hex2long(&ptr, &ks.threadid);
    if (!getthread(ks.linux_regs, ks.threadid)) {
    error_packet(remcom_out_buffer, -EINVAL);
    // break;
    }
    if ((int)ks.threadid > 0) {
    kgdb_mem2hex(getthread(ks.linux_regs,
    ks.threadid).comm,
    remcom_out_buffer, 16);
    } else {
    static char tmpstr[23 + BUF_THREAD_ID_SIZE];
    sprintf(tmpstr, "shadowCPU%d",
    (int)(-ks.threadid - 2));
    kgdb_mem2hex(tmpstr, remcom_out_buffer, strlen(tmpstr));
    }
    // break;

    }
    'R' => {
    if (strncmp(remcom_in_buffer, "qRcmd,", 6) == 0) {
pub static mut len: c_int = 0;
    if ((len % 2) != 0) {
    strscpy(remcom_out_buffer, "E01");
    // break;
    }
    kgdb_hex2mem(remcom_in_buffer + 6,
    remcom_out_buffer, len);
    len = len / 2;
    remcom_out_buffer[len++] = 0;
    kdb_common_init_state(ks);
    kdb_parse(remcom_out_buffer);
    kdb_common_deinit_state();
    strscpy(remcom_out_buffer, "OK");
    }
    // break;

    }
    'S' => {
    if (!strncmp(remcom_in_buffer, "qSupported:", 11)) {
    strscpy(remcom_out_buffer, kgdb_arch_gdb_stub_feature);
    }
    // break;
    }
    'X' => {
    if (!strncmp(remcom_in_buffer, "qXfer:", 6)) {
    kgdb_arch_handle_qxfer_pkt(remcom_in_buffer,
    remcom_out_buffer);
    }
    // break;

    }
    _ => {
    // break;
    }
    }
    }
// Handle the 'H' task query packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_task(ks: *mut kgdb_state) {
pub static mut thread: *mut c_void = core::ptr::null_mut();
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    match (remcom_in_buffer[1]) {
    'g' => {
    ptr = &remcom_in_buffer[2];
    kgdb_hex2long(&ptr, &ks.threadid);
    thread = getthread(ks.linux_regs, ks.threadid);
    if (!thread && ks.threadid > 0) {
    error_packet(remcom_out_buffer, -EINVAL);
    // break;
    }
    kgdb_usethread = thread;
    ks.kgdb_usethreadid = ks.threadid;
    strscpy(remcom_out_buffer, "OK");
    // break;
    }
    'c' => {
    ptr = &remcom_in_buffer[2];
    kgdb_hex2long(&ptr, &ks.threadid);
    if (!ks.threadid) {
    kgdb_contthread = core::ptr::null_mut();
    } else {
    thread = getthread(ks.linux_regs, ks.threadid);
    if (!thread && ks.threadid > 0) {
    error_packet(remcom_out_buffer, -EINVAL);
    // break;
    }
    kgdb_contthread = thread;
    }
    strscpy(remcom_out_buffer, "OK");
    // break;
    }
    }
    }
// Handle the 'T' thread query packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_thread(ks: *mut kgdb_state) {
    let mut ptr = &remcom_in_buffer[1];
pub static mut thread: *mut c_void = core::ptr::null_mut();
    kgdb_hex2long(&ptr, &ks.threadid);
    thread = getthread(ks.linux_regs, ks.threadid);
    if (thread) {
    strscpy(remcom_out_buffer, "OK");
    }
    else {
    error_packet(remcom_out_buffer, -EINVAL);
    }
    }
// Handle the 'z' or 'Z' breakpoint remove or set packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_break(ks: *mut kgdb_state) {
//
// Since GDB-5.3, it's been drafted that '0' is a software
// breakpoint, '1' is a hardware breakpoint, so let's do that.
//
    let mut bpt_type = &remcom_in_buffer[1];
    let mut ptr = &remcom_in_buffer[2];
    let mut addr = 0;
    let mut length = 0;
pub static mut error: c_int = 0;
    if (arch_kgdb_ops.set_hw_breakpoint && *bpt_type >= '1') {
// Unsupported
    if (*bpt_type > '4') {
    return;
    }
    } else {
    if (*bpt_type != '0' && *bpt_type != '1') {
// Unsupported.
    return;
    }
    }
//
// Test if this is a hardware breakpoint, and
// if we support it:
//
    if (*bpt_type == '1' && !(arch_kgdb_ops.flags & KGDB_HW_BREAKPOINT)) {
// Unsupported.
    return;
    }
    if (*(ptr++) != ',') {
    error_packet(remcom_out_buffer, -EINVAL);
    return;
    }
    if (!kgdb_hex2long(&ptr, &addr)) {
    error_packet(remcom_out_buffer, -EINVAL);
    return;
    }
    if (*(ptr++) != ',' ||
    !kgdb_hex2long(&ptr, &length)) {
    error_packet(remcom_out_buffer, -EINVAL);
    return;
    }
    if (remcom_in_buffer[0] == 'Z' && *bpt_type == '0') {
    error = dbg_set_sw_break(addr);
    }

    else if (remcom_in_buffer[0] == 'z' && *bpt_type == '0') {
    error = dbg_remove_sw_break(addr);
    }

    else if (remcom_in_buffer[0] == 'Z') {
    error = arch_kgdb_ops.set_hw_breakpoint(addr,
    (int)length, *bpt_type - '0');
    }

    else if (remcom_in_buffer[0] == 'z') {
    error = arch_kgdb_ops.remove_hw_breakpoint(addr,
    (int) length, *bpt_type - '0');
    }
    if (error == 0) {
    strscpy(remcom_out_buffer, "OK");
    }
    else {
    error_packet(remcom_out_buffer, error);
    }
    }
// Handle the 'C' signal / exception passing packets
#[no_mangle]
unsafe extern "C" fn gdb_cmd_exception_pass(ks: *mut kgdb_state) -> c_int {
// C09 == pass exception
// C15 == detach kgdb, pass exception
//
    if (remcom_in_buffer[1] == '0' && remcom_in_buffer[2] == '9') {
    ks.pass_exception = 1;
    remcom_in_buffer[0] = 'c';
    } else if (remcom_in_buffer[1] == '1' && remcom_in_buffer[2] == '5') {
    ks.pass_exception = 1;
    remcom_in_buffer[0] = 'D';
    dbg_remove_all_break();
    kgdb_connected = 0;
    return 1;
    } else {
    gdbstub_msg_write("KGDB only knows signal 9 (pass)"
    " and 15 (pass and disconnect)\n"
    "Executing a continue without signal passing\n", 0);
    remcom_in_buffer[0] = 'c';
    }
// Indicate fall through
    return -1;
    }
//
// This function performs all gdbserial command processing
//
#[no_mangle]
pub unsafe extern "C" fn gdb_serial_stub(ks: *mut kgdb_state) -> c_int {
pub static mut error: c_int = 0;
    let mut tmp = 0;
// Initialize comm buffer and globals.
    memset(remcom_out_buffer, 0, sizeof!(remcom_out_buffer));
    kgdb_usethread = kgdb_info[ks.cpu].task;
    ks.kgdb_usethreadid = shadow_pid(kgdb_info[ks.cpu].task.pid);
    ks.pass_exception = 0;
    if (kgdb_connected) {
    unsigned char thref[BUF_THREAD_ID_SIZE];
pub static mut ptr: *mut c_void = core::ptr::null_mut();
// Reply to host that an exception has occurred
    ptr = remcom_out_buffer;
// ptr++ = 'T';
    ptr = hex_byte_pack(ptr, ks.signo);
    ptr += strlen(strcpy(ptr, "thread:"));
// forward_decl: _to_threadref;
    ptr = pack_threadid(ptr, thref);
// ptr++ = ';';
    put_packet(remcom_out_buffer);
    }
    while (1) {
    error = 0;
// Clear the out buffer.
    memset(remcom_out_buffer, 0, sizeof!(remcom_out_buffer));
    get_packet(remcom_in_buffer);
    match (remcom_in_buffer[0]) {
    '?' => {
    gdb_cmd_status(ks);
    // break;
    }
    'g' => {
    gdb_cmd_getregs(ks);
    // break;
    }
    'G' => {
    gdb_cmd_setregs(ks);
    // break;
    }
    'm' => {
    gdb_cmd_memread(ks);
    // break;
    }
    'M' => {
    gdb_cmd_memwrite(ks);
    // break;

    }
    'p' => {
    gdb_cmd_reg_get(ks);
    // break;
    }
    'P' => {
    gdb_cmd_reg_set(ks);
    // break;

    }
    'X' => {
    gdb_cmd_binwrite(ks);
    // break;
// kill or detach. KGDB should treat this like a
// continue.
//
    }
    'D' => {
    }
    'k' => {
    gdb_cmd_detachkill(ks);
// goto;
    }
    'R' => {
    if (gdb_cmd_reboot(ks)) {
// goto;
    }
    // break;
    }
    'q' => {
    gdb_cmd_query(ks);
    // break;
    }
    'H' => {
    gdb_cmd_task(ks);
    // break;
    }
    'T' => {
    gdb_cmd_thread(ks);
    // break;
    }
    'z' => {
    }
    'Z' => {
    gdb_cmd_break(ks);
    // break;

    }
    '3' => {
    if (remcom_in_buffer[1] == '\0') {
    gdb_cmd_detachkill(ks);
    return DBG_PASS_EVENT;
    }
    fallthrough;

    }
    'C' => {
    tmp = gdb_cmd_exception_pass(ks);
    if (tmp > 0) {
// goto;
    }
    if (tmp == 0) {
    // break;
    }
    fallthrough;	/* on tmp < 0 */
    }
    'c' => {
    }
    's' => {
    if (kgdb_contthread && kgdb_contthread != current) {
// Can't switch threads in kgdb
    error_packet(remcom_out_buffer, -EINVAL);
    // break;
    }
    fallthrough;	/* to default processing */
    }
    _ => {
// label;
    error = kgdb_arch_handle_exception(ks.ex_vector,
    ks.signo,
    ks.err_code,
    remcom_in_buffer,
    remcom_out_buffer,
    ks.linux_regs);
//
// Leave cmd processing on error, detach,
// kill, continue, or single step.
//
    if (error >= 0 || remcom_in_buffer[0] == 'D' ||
    remcom_in_buffer[0] == 'k') {
    error = 0;
// goto;
    }
    }
    }
// reply to the request
    put_packet(remcom_out_buffer);
    }
// label;
    if (ks.pass_exception) {
    error = 1;
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn gdbstub_state(ks: *mut kgdb_state, cmd: *mut c_char) -> c_int {
    let mut error = 0;
    match (cmd[0]) {
    'e' => {
    error = kgdb_arch_handle_exception(ks.ex_vector,
    ks.signo,
    ks.err_code,
    remcom_in_buffer,
    remcom_out_buffer,
    ks.linux_regs);
    return error;
    }
    's' => {
    }
    'c' => {
    strscpy(remcom_in_buffer, cmd, sizeof!(remcom_in_buffer));
    return 0;
    }
    '$' => {
    strscpy(remcom_in_buffer, cmd, sizeof!(remcom_in_buffer));
    gdbstub_use_prev_in_buf = strlen(remcom_in_buffer);
    gdbstub_prev_in_buf_pos = 0;
    return 0;
    }
    }
    dbg_io_ops.write_char('+');
    put_packet(remcom_out_buffer);
    return 0;
    }
//
// gdbstub_exit - Send an exit message to GDB
// @status: The exit code to report.
//
#[no_mangle]
pub unsafe extern "C" fn gdbstub_exit(status: c_int) {
    unsigned char checksum, ch, buffer[3];
    let mut loop = 0;
    if (!kgdb_connected) {
    return;
    }
    kgdb_connected = 0;
    if (!dbg_io_ops || dbg_kdb_mode) {
    return;
    }
    buffer[0] = 'W';
    buffer[1] = hex_asc_hi(status);
    buffer[2] = hex_asc_lo(status);
    dbg_io_ops.write_char('$');
    checksum = 0;
    while (loop < 3) {
    ch = buffer[loop];
    checksum += ch;
    dbg_io_ops.write_char(ch);
    }
    dbg_io_ops.write_char('#');
    dbg_io_ops.write_char(hex_asc_hi(checksum));
    dbg_io_ops.write_char(hex_asc_lo(checksum));
// make sure the output is flushed, lest the bootloader clobber it
    if (dbg_io_ops.flush) {
    dbg_io_ops.flush();
    }
    }