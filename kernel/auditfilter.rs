//! Automatically rewritten from C to Rust
//! Source: kernel/auditfilter.c
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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-or-later
// auditfilter.c -- filtering of audit events
//
// Copyright 2003-2004 Red Hat, Inc.
// Copyright 2005 Hewlett-Packard Development Company, L.P.
// Copyright 2005 IBM Corporation
//

//
// Locking model:
//
// audit_filter_mutex:
// Synchronizes writes and blocking reads of audit's filterlist
// data.  Rcu is used to traverse the filterlist and access
// contents of structs audit_entry, audit_watch and opaque
// LSM rules during filtering.  If modified, these structures
// must be copied and replace their counterparts in the filterlist.
// An audit_parent struct is not accessed during filtering, so may
// be written directly provided audit_filter_mutex is held.
//
// Audit filter lists, defined in <linux/audit.h>
    struct list_head audit_filter_list[AUDIT_NR_FILTERS] = {
    LIST_HEAD_INIT(audit_filter_list[0]),
    LIST_HEAD_INIT(audit_filter_list[1]),
    LIST_HEAD_INIT(audit_filter_list[2]),
    LIST_HEAD_INIT(audit_filter_list[3]),
    LIST_HEAD_INIT(audit_filter_list[4]),
    LIST_HEAD_INIT(audit_filter_list[5]),
    LIST_HEAD_INIT(audit_filter_list[6]),
    LIST_HEAD_INIT(audit_filter_list[7]),

    };
pub static mut list_head: usize = 0;
// DEFINE_MUTEX;
#[no_mangle]
unsafe extern "C" fn audit_free_lsm_field(f: *mut audit_field) {
    match (f.type) {
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    AUDIT_OBJ_USER => {
    AUDIT_OBJ_ROLE => {
    AUDIT_OBJ_TYPE => {
    AUDIT_OBJ_LEV_LOW => {
    AUDIT_OBJ_LEV_HIGH => {
    kfree(f.lsm_str);
    security_audit_rule_free(f.lsm_rule);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn audit_free_rule(e: *mut audit_entry) {
    let mut i = 0;
    struct audit_krule *erule = &e.rule;
// some rules don't have associated watches
    if (erule.watch) {
    audit_put_watch(erule.watch);
    }
    if (erule.fields) {
    for (i = 0; i < erule.field_count; i++)
    }
    audit_free_lsm_field(&erule.fields[i]);
    kfree(erule.fields);
    kfree(erule.filterkey);
    kfree(e);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_free_rule_rcu(head: *mut rcu_head) {
    struct audit_entry *e = container_of(head, struct audit_entry, rcu);
    audit_free_rule(e);
    }
// Initialize an audit filterlist entry.
#[no_mangle]
pub unsafe extern "C" fn audit_init_entry() {
    let mut entry = core::ptr::null_mut();
    let mut fields = core::ptr::null_mut();
    entry = kzalloc_obj(*entry);
    if (unlikely(!entry)) {
    return core::ptr::null_mut();
    }
    fields = kzalloc_objs(*fields, field_count);
    if (unlikely(!fields)) {
    kfree(entry);
    return core::ptr::null_mut();
    }
    entry.rule.fields = fields;
    return entry;
    }
// Unpack a filter field's string representation from user-space
// buffer.
#[no_mangle]
pub unsafe extern "C" fn audit_unpack_string() {
    let mut str = core::ptr::null_mut();
    if (!*bufp || (len == 0) || (len > *remain)) {
    return ERR_PTR(-EINVAL);
    }
// Of the currently implemented string fields, PATH_MAX
// defines the longest valid length.
//
    if (len > PATH_MAX) {
    return ERR_PTR(-ENAMETOOLONG);
    }
    str = kmalloc(len + 1, GFP_KERNEL);
    if (unlikely(!str)) {
    return ERR_PTR(-ENOMEM);
    }
    memcpy(str, *bufp, len);
    str[len] = 0;
// bufp += len;
// remain -= len;
    return str;
    }
// Translate an inode field to kernel representation.
#[no_mangle]
pub unsafe extern "C" fn audit_to_inode() {
    if ((krule.listnr != AUDIT_FILTER_EXIT &&
    krule.listnr != AUDIT_FILTER_URING_EXIT) ||
    krule.inode_f || krule.watch || krule.tree ||
    (f.op != Audit_equal && f.op != Audit_not_equal))
    return -EINVAL;
    krule.inode_f = f;
    return 0;
    }
    static __u32 *classes[AUDIT_SYSCALL_CLASSES];
#[no_mangle]
pub unsafe extern "C" fn audit_register_class(class: c_int, list: *mut c_uint) -> c_int {
    __u32 *p = kcalloc(AUDIT_BITMASK_SIZE, sizeof(__u32), GFP_KERNEL);
    if (!p) {
    return -ENOMEM;
    }
    while (*list != ~0U) {
pub static mut n: c_uint = *list++;
    if (n >= AUDIT_BITMASK_SIZE * 32 - AUDIT_SYSCALL_CLASSES) {
    kfree(p);
    return -EINVAL;
    }
    p[AUDIT_WORD(n)] |= AUDIT_BIT(n);
    }
    if (class >= AUDIT_SYSCALL_CLASSES || classes[class]) {
    kfree(p);
    return -EINVAL;
    }
    classes[class] = p;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_match_class(class: c_int, syscall: c_uint) -> c_int {
    if (unlikely(syscall >= AUDIT_BITMASK_SIZE * 32)) {
    return 0;
    }
    if (unlikely(class >= AUDIT_SYSCALL_CLASSES || !classes[class])) {
    return 0;
    }
    return classes[class][AUDIT_WORD(syscall)] & AUDIT_BIT(syscall);
    }

#[no_mangle]
pub unsafe extern "C" fn audit_match_class_bits(class: c_int, mask: *mut u32) -> c_int {
    let mut i = 0;
    if (classes[class]) {
    for (i = 0; i < AUDIT_BITMASK_SIZE; i++)
    if (mask[i] & classes[class][i]) {
    return 0;
    }
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn audit_match_signal(entry: *mut audit_entry) -> c_int {
    struct audit_field *arch = entry.rule.arch_f;
    if (!arch) {
// When arch is unspecified, we must check both masks on biarch
// as syscall number alone is ambiguous.
    return (audit_match_class_bits(AUDIT_CLASS_SIGNAL,
    entry.rule.mask) &&
    audit_match_class_bits(AUDIT_CLASS_SIGNAL_32,
    entry.rule.mask));
    }
    switch (audit_classify_arch(arch.val)) {
    0 => { /* native */
    return (audit_match_class_bits(AUDIT_CLASS_SIGNAL,
    entry.rule.mask));
    1 => { /* 32bit on biarch */
    return (audit_match_class_bits(AUDIT_CLASS_SIGNAL_32,
    entry.rule.mask));
    _ => {
    return 1;
    }
    }

// Common user-space to kernel rule translation.
#[no_mangle]
pub unsafe extern "C" fn audit_to_entry_common() {
    let mut listnr = 0;
    let mut entry = core::ptr::null_mut();
    int i, err;
    err = -EINVAL;
    listnr = rule.flags & ~AUDIT_FILTER_PREPEND;
    match (listnr) {
    _ => {
    goto exit_err;

    AUDIT_FILTER_ENTRY => {
    pr_err("AUDIT_FILTER_ENTRY is deprecated\n");
    goto exit_err;
    AUDIT_FILTER_EXIT => {
    AUDIT_FILTER_URING_EXIT => {
    AUDIT_FILTER_TASK => {

    AUDIT_FILTER_USER => {
    AUDIT_FILTER_EXCLUDE => {
    AUDIT_FILTER_FS => {
    ;
    }
    if (unlikely(rule.action == AUDIT_POSSIBLE)) {
    pr_err("AUDIT_POSSIBLE is deprecated\n");
    goto exit_err;
    }
    if (rule.action != AUDIT_NEVER && rule.action != AUDIT_ALWAYS) {
    goto exit_err;
    }
    if (rule.field_count > AUDIT_MAX_FIELDS) {
    goto exit_err;
    }
    err = -ENOMEM;
    entry = audit_init_entry(rule.field_count);
    if (!entry) {
    goto exit_err;
    }
    entry.rule.flags = rule.flags & AUDIT_FILTER_PREPEND;
    entry.rule.listnr = listnr;
    entry.rule.action = rule.action;
    entry.rule.field_count = rule.field_count;
    for (i = 0; i < AUDIT_BITMASK_SIZE; i++)
    entry.rule.mask[i] = rule.mask[i];
    for (i = 0; i < AUDIT_SYSCALL_CLASSES; i++) {
pub static mut bit: c_int = AUDIT_BITMASK_SIZE * 32 - i - 1;
    __u32 *p = &entry.rule.mask[AUDIT_WORD(bit)];
    let mut class = core::ptr::null_mut();
    if (!(*p & AUDIT_BIT(bit))) {
    continue;
    }
// p &= ~AUDIT_BIT(bit);
    class = classes[i];
    if (class) {
    let mut j = 0;
    for (j = 0; j < AUDIT_BITMASK_SIZE; j++)
    entry.rule.mask[j] |= class[j];
    }
    }
    return entry;
    exit_err:
    return ERR_PTR(err);
    }
    static u32 audit_ops[] = {
    [Audit_equal] = AUDIT_EQUAL,
    [Audit_not_equal] = AUDIT_NOT_EQUAL,
    [Audit_bitmask] = AUDIT_BIT_MASK,
    [Audit_bittest] = AUDIT_BIT_TEST,
    [Audit_lt] = AUDIT_LESS_THAN,
    [Audit_gt] = AUDIT_GREATER_THAN,
    [Audit_le] = AUDIT_LESS_THAN_OR_EQUAL,
    [Audit_ge] = AUDIT_GREATER_THAN_OR_EQUAL,
    };
#[no_mangle]
unsafe extern "C" fn audit_to_op(op: u32) -> u32 {
    let mut n = 0;
    for (n = Audit_equal; n < Audit_bad && audit_ops[n] != op; n++)
    ;
    return n;
    }
// check if an audit field is valid
#[no_mangle]
unsafe extern "C" fn audit_field_valid(entry: *mut audit_entry, f: *mut audit_field) -> c_int {
    match (f.type) {
    AUDIT_MSGTYPE => {
    if (entry.rule.listnr != AUDIT_FILTER_EXCLUDE &&
    entry.rule.listnr != AUDIT_FILTER_USER)
    return -EINVAL;
    break;
    AUDIT_FSTYPE => {
    if (entry.rule.listnr != AUDIT_FILTER_FS) {
    return -EINVAL;
    }
    break;
    AUDIT_PERM => {
    if (entry.rule.listnr == AUDIT_FILTER_URING_EXIT) {
    return -EINVAL;
    }
    break;
    }
    match (entry.rule.listnr) {
    AUDIT_FILTER_FS => {
    match (f.type) {
    AUDIT_FSTYPE => {
    AUDIT_FILTERKEY => {
    break;
    _ => {
    return -EINVAL;
    }
    }
// Check for valid field type and op
    match (f.type) {
    AUDIT_ARG0 => {
    AUDIT_ARG1 => {
    AUDIT_ARG2 => {
    AUDIT_ARG3 => {
    AUDIT_PERS => { /* <uapi/linux/personality.h> */
    AUDIT_DEVMINOR => {
// all ops are valid
    break;
    AUDIT_UID => {
    AUDIT_EUID => {
    AUDIT_SUID => {
    AUDIT_FSUID => {
    AUDIT_LOGINUID => {
    AUDIT_OBJ_UID => {
    AUDIT_GID => {
    AUDIT_EGID => {
    AUDIT_SGID => {
    AUDIT_FSGID => {
    AUDIT_OBJ_GID => {
    AUDIT_PID => {
    AUDIT_MSGTYPE => {
    AUDIT_PPID => {
    AUDIT_DEVMAJOR => {
    AUDIT_EXIT => {
    AUDIT_SUCCESS => {
    AUDIT_INODE => {
    AUDIT_SESSIONID => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    AUDIT_OBJ_LEV_LOW => {
    AUDIT_OBJ_LEV_HIGH => {
    AUDIT_SADDR_FAM => {
// bit ops are only useful on syscall args
    if (f.op == Audit_bitmask || f.op == Audit_bittest) {
    return -EINVAL;
    }
    break;
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_OBJ_USER => {
    AUDIT_OBJ_ROLE => {
    AUDIT_OBJ_TYPE => {
    AUDIT_WATCH => {
    AUDIT_DIR => {
    AUDIT_FILTERKEY => {
    AUDIT_LOGINUID_SET => {
    AUDIT_ARCH => {
    AUDIT_FSTYPE => {
    AUDIT_PERM => {
    AUDIT_FILETYPE => {
    AUDIT_FIELD_COMPARE => {
    AUDIT_EXE => {
// only equal and not equal valid ops
    if (f.op != Audit_not_equal && f.op != Audit_equal) {
    return -EINVAL;
    }
    break;
    _ => {
// field not recognized
    return -EINVAL;
    }
// Check for select valid field values
    match (f.type) {
    AUDIT_LOGINUID_SET => {
    if ((f.val != 0) && (f.val != 1)) {
    return -EINVAL;
    }
    break;
    AUDIT_PERM => {
    if (f.val & ~15) {
    return -EINVAL;
    }
    break;
    AUDIT_FILETYPE => {
    if (f.val & ~S_IFMT) {
    return -EINVAL;
    }
    break;
    AUDIT_FIELD_COMPARE => {
    if (f.val > AUDIT_MAX_FIELD_COMPARE) {
    return -EINVAL;
    }
    break;
    AUDIT_SADDR_FAM => {
    if (f.val >= AF_MAX) {
    return -EINVAL;
    }
    break;
    _ => {
    break;
    }
    return 0;
    }
// Translate struct audit_rule_data to kernel's rule representation.
#[no_mangle]
pub unsafe extern "C" fn audit_data_to_entry() {
pub static mut err: c_int = 0;
    let mut entry = core::ptr::null_mut();
    let mut bufp = core::ptr::null_mut();
pub static mut remain: usize = datasz - sizeof(struct audit_rule_data);
    let mut i = 0;
    let mut str = core::ptr::null_mut();
    let mut audit_mark = core::ptr::null_mut();
    entry = audit_to_entry_common(data);
    if (IS_ERR(entry)) {
    goto exit_nofree;
    }
    bufp = data.buf;
    for (i = 0; i < data.field_count; i++) {
    struct audit_field *f = &entry.rule.fields[i];
    let mut f_val = 0;
    err = -EINVAL;
    f.op = audit_to_op(data.fieldflags[i]);
    if (f.op == Audit_bad) {
    goto exit_free;
    }
    f.type = data.fields[i];
    f_val = data.values[i];
// Support legacy tests for a valid loginuid
    if ((f.type == AUDIT_LOGINUID) && (f_val == AUDIT_UID_UNSET)) {
    f.type = AUDIT_LOGINUID_SET;
    f_val = 0;
    entry.rule.pflags |= AUDIT_LOGINUID_LEGACY;
    }
    err = audit_field_valid(entry, f);
    if (err) {
    goto exit_free;
    }
    err = -EINVAL;
    match (f.type) {
    AUDIT_LOGINUID => {
    AUDIT_UID => {
    AUDIT_EUID => {
    AUDIT_SUID => {
    AUDIT_FSUID => {
    AUDIT_OBJ_UID => {
    f.uid = make_kuid(current_user_ns(), f_val);
    if (!uid_valid(f.uid)) {
    goto exit_free;
    }
    break;
    AUDIT_GID => {
    AUDIT_EGID => {
    AUDIT_SGID => {
    AUDIT_FSGID => {
    AUDIT_OBJ_GID => {
    f.gid = make_kgid(current_user_ns(), f_val);
    if (!gid_valid(f.gid)) {
    goto exit_free;
    }
    break;
    AUDIT_ARCH => {
    f.val = f_val;
    entry.rule.arch_f = f;
    break;
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    AUDIT_OBJ_USER => {
    AUDIT_OBJ_ROLE => {
    AUDIT_OBJ_TYPE => {
    AUDIT_OBJ_LEV_LOW => {
    AUDIT_OBJ_LEV_HIGH => {
    str = audit_unpack_string(&bufp, &remain, f_val);
    if (IS_ERR(str)) {
    err = PTR_ERR(str);
    goto exit_free;
    }
    entry.rule.buflen += f_val;
    f.lsm_str = str;
    err = security_audit_rule_init(f.type, f.op, str,
    &f.lsm_rule,
    GFP_KERNEL);
// Keep currently invalid fields around in case they
// become valid after a policy reload.
    if (err == -EINVAL) {
    pr_warn("audit rule for LSM \'%s\' is invalid\n",
    str);
    err = 0;
    } else if (err)
    goto exit_free;
    break;
    AUDIT_WATCH => {
    str = audit_unpack_string(&bufp, &remain, f_val);
    if (IS_ERR(str)) {
    err = PTR_ERR(str);
    goto exit_free;
    }
    err = audit_to_watch(&entry.rule, str, f_val, f.op);
    if (err) {
    kfree(str);
    goto exit_free;
    }
    entry.rule.buflen += f_val;
    break;
    AUDIT_DIR => {
    str = audit_unpack_string(&bufp, &remain, f_val);
    if (IS_ERR(str)) {
    err = PTR_ERR(str);
    goto exit_free;
    }
    err = audit_make_tree(&entry.rule, str, f.op);
    kfree(str);
    if (err) {
    goto exit_free;
    }
    entry.rule.buflen += f_val;
    break;
    AUDIT_INODE => {
    f.val = f_val;
    err = audit_to_inode(&entry.rule, f);
    if (err) {
    goto exit_free;
    }
    break;
    AUDIT_FILTERKEY => {
    if (entry.rule.filterkey || f_val > AUDIT_MAX_KEY_LEN) {
    goto exit_free;
    }
    str = audit_unpack_string(&bufp, &remain, f_val);
    if (IS_ERR(str)) {
    err = PTR_ERR(str);
    goto exit_free;
    }
    entry.rule.buflen += f_val;
    entry.rule.filterkey = str;
    break;
    AUDIT_EXE => {
    if (entry.rule.exe || f_val > PATH_MAX) {
    goto exit_free;
    }
    str = audit_unpack_string(&bufp, &remain, f_val);
    if (IS_ERR(str)) {
    err = PTR_ERR(str);
    goto exit_free;
    }
    audit_mark = audit_alloc_mark(&entry.rule, str, f_val, core::ptr::null_mut());
    if (IS_ERR(audit_mark)) {
    kfree(str);
    err = PTR_ERR(audit_mark);
    goto exit_free;
    }
    entry.rule.buflen += f_val;
    entry.rule.exe = audit_mark;
    break;
    _ => {
    f.val = f_val;
    break;
    }
    }
    if (entry.rule.inode_f && entry.rule.inode_f.op == Audit_not_equal) {
    entry.rule.inode_f = core::ptr::null_mut();
    }
    exit_nofree:
    return entry;
    exit_free:
    if (entry.rule.tree) {
    audit_put_tree(entry.rule.tree); /* that's the temporary one */
    }
    if (entry.rule.exe) {
    audit_remove_mark(entry.rule.exe); /* that's the template one */
    }
    audit_free_rule(entry);
    return ERR_PTR(err);
    }
// Pack a filter field's string representation into data block.
#[no_mangle]
pub unsafe extern "C" fn audit_pack_string(bufp: *mut c_void, str: *const c_char) -> usize {
pub static mut len: usize = strlen(str);
    memcpy(*bufp, str, len);
// bufp += len;
    return len;
    }
// Translate kernel rule representation to struct audit_rule_data.
#[no_mangle]
pub unsafe extern "C" fn audit_krule_to_data() {
    let mut data = core::ptr::null_mut();
    let mut bufp = core::ptr::null_mut();
    let mut i = 0;
    data = kzalloc_flex(*data, buf, krule.buflen);
    if (unlikely(!data)) {
    return core::ptr::null_mut();
    }
    data.flags = krule.flags | krule.listnr;
    data.action = krule.action;
    data.field_count = krule.field_count;
    bufp = data.buf;
    for (i = 0; i < data.field_count; i++) {
    struct audit_field *f = &krule.fields[i];
    data.fields[i] = f.type;
    data.fieldflags[i] = audit_ops[f.op];
    match (f.type) {
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    AUDIT_OBJ_USER => {
    AUDIT_OBJ_ROLE => {
    AUDIT_OBJ_TYPE => {
    AUDIT_OBJ_LEV_LOW => {
    AUDIT_OBJ_LEV_HIGH => {
    data.buflen += data.values[i] =
    audit_pack_string(&bufp, f.lsm_str);
    break;
    AUDIT_WATCH => {
    data.buflen += data.values[i] =
    audit_pack_string(&bufp,
    audit_watch_path(krule.watch));
    break;
    AUDIT_DIR => {
    data.buflen += data.values[i] =
    audit_pack_string(&bufp,
    audit_tree_path(krule.tree));
    break;
    AUDIT_FILTERKEY => {
    data.buflen += data.values[i] =
    audit_pack_string(&bufp, krule.filterkey);
    break;
    AUDIT_EXE => {
    data.buflen += data.values[i] =
    audit_pack_string(&bufp, audit_mark_path(krule.exe));
    break;
    AUDIT_LOGINUID_SET => {
    if (krule.pflags & AUDIT_LOGINUID_LEGACY && !f.val) {
    data.fields[i] = AUDIT_LOGINUID;
    data.values[i] = AUDIT_UID_UNSET;
    break;
    }
    fallthrough;	/* if set */
    _ => {
    data.values[i] = f.val;
    }
    }
    for (i = 0; i < AUDIT_BITMASK_SIZE; i++)
    data.mask[i] = krule.mask[i];
    return data;
    }
// Compare two rules in kernel format.  Considered success if rules
// don't match.
#[no_mangle]
unsafe extern "C" fn audit_compare_rule(a: *mut audit_krule, b: *mut audit_krule) -> c_int {
    let mut i = 0;
    if (a.flags != b.flags ||
    a.pflags != b.pflags ||
    a.listnr != b.listnr ||
    a.action != b.action ||
    a.field_count != b.field_count)
    return 1;
    for (i = 0; i < a.field_count; i++) {
    if (a.fields[i].type != b.fields[i].type ||
    a.fields[i].op != b.fields[i].op)
    return 1;
    match (a.fields[i].type) {
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    AUDIT_OBJ_USER => {
    AUDIT_OBJ_ROLE => {
    AUDIT_OBJ_TYPE => {
    AUDIT_OBJ_LEV_LOW => {
    AUDIT_OBJ_LEV_HIGH => {
    if (strcmp(a.fields[i].lsm_str, b.fields[i].lsm_str)) {
    return 1;
    }
    break;
    AUDIT_WATCH => {
    if (strcmp(audit_watch_path(a.watch),
    audit_watch_path(b.watch)))
    return 1;
    break;
    AUDIT_DIR => {
    if (strcmp(audit_tree_path(a.tree),
    audit_tree_path(b.tree)))
    return 1;
    break;
    AUDIT_FILTERKEY => {
// both filterkeys exist based on above type compare
    if (strcmp(a.filterkey, b.filterkey)) {
    return 1;
    }
    break;
    AUDIT_EXE => {
// both paths exist based on above type compare
    if (strcmp(audit_mark_path(a.exe),
    audit_mark_path(b.exe)))
    return 1;
    break;
    AUDIT_UID => {
    AUDIT_EUID => {
    AUDIT_SUID => {
    AUDIT_FSUID => {
    AUDIT_LOGINUID => {
    AUDIT_OBJ_UID => {
    if (!uid_eq(a.fields[i].uid, b.fields[i].uid)) {
    return 1;
    }
    break;
    AUDIT_GID => {
    AUDIT_EGID => {
    AUDIT_SGID => {
    AUDIT_FSGID => {
    AUDIT_OBJ_GID => {
    if (!gid_eq(a.fields[i].gid, b.fields[i].gid)) {
    return 1;
    }
    break;
    _ => {
    if (a.fields[i].val != b.fields[i].val) {
    return 1;
    }
    }
    }
    for (i = 0; i < AUDIT_BITMASK_SIZE; i++)
    if (a.mask[i] != b.mask[i]) {
    return 1;
    }
    return 0;
    }
// Duplicate LSM field information.  The lsm_rule is opaque, so must be
// re-initialized.
#[no_mangle]
pub unsafe extern "C" fn audit_dupe_lsm_field() {
    let mut ret = 0;
    let mut lsm_str = core::ptr::null_mut();
// our own copy of lsm_str
    lsm_str = kstrdup(sf.lsm_str, GFP_KERNEL);
    if (unlikely(!lsm_str)) {
    return -ENOMEM;
    }
    df.lsm_str = lsm_str;
// our own (refreshed) copy of lsm_rule
    ret = security_audit_rule_init(df.type, df.op, df.lsm_str,
    &df.lsm_rule, GFP_KERNEL);
// Keep currently invalid fields around in case they
// become valid after a policy reload.
    if (ret == -EINVAL) {
    pr_warn("audit rule for LSM \'%s\' is invalid\n",
    df.lsm_str);
    ret = 0;
    }
    return ret;
    }
// Duplicate an audit rule.  This will be a deep copy with the exception
// of the watch - that pointer is carried over.  The LSM specific fields
// will be updated in the copy.  The point is to be able to replace the old
// rule with the new rule in the filterlist, then free the old rule.
// The rlist element is undefined; list manipulations are handled apart from
// the initial copy.
#[no_mangle]
pub unsafe extern "C" fn audit_dupe_rule() {
pub static mut fcount: u32 = old.field_count;
    let mut entry = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    let mut fk = core::ptr::null_mut();
    int i, err = 0;
    entry = audit_init_entry(fcount);
    if (unlikely(!entry)) {
    return ERR_PTR(-ENOMEM);
    }
    new = &entry.rule;
    new.flags = old.flags;
    new.pflags = old.pflags;
    new.listnr = old.listnr;
    new.action = old.action;
    for (i = 0; i < AUDIT_BITMASK_SIZE; i++)
    new.mask[i] = old.mask[i];
    new.prio = old.prio;
    new.buflen = old.buflen;
    new.inode_f = old.inode_f;
    new.field_count = old.field_count;
//
// note that we are OK with not refcounting here; audit_match_tree()
// never dereferences tree and we can't get false positives there
// since we'd have to have rule gone from the list *and* removed
// before the chunks found by lookup had been allocated, i.e. before
// the beginning of list scan.
//
    new.tree = old.tree;
    memcpy(new.fields, old.fields, sizeof(struct audit_field) * fcount);
// deep copy this information, updating the lsm_rule fields, because
// the originals will all be freed when the old rule is freed.
    for (i = 0; i < fcount; i++) {
    match (new.fields[i].type) {
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    AUDIT_OBJ_USER => {
    AUDIT_OBJ_ROLE => {
    AUDIT_OBJ_TYPE => {
    AUDIT_OBJ_LEV_LOW => {
    AUDIT_OBJ_LEV_HIGH => {
    err = audit_dupe_lsm_field(&new.fields[i],
    &old.fields[i]);
    break;
    AUDIT_FILTERKEY => {
    fk = kstrdup(old.filterkey, GFP_KERNEL);
    if (unlikely(!fk)) {
    err = -ENOMEM;
    }
    else {
    new.filterkey = fk;
    }
    break;
    AUDIT_EXE => {
    err = audit_dupe_exe(new, old, ctx);
    break;
    }
    if (err) {
    if (new.exe) {
    audit_remove_mark(new.exe);
    }
    audit_free_rule(entry);
    return ERR_PTR(err);
    }
    }
    if (old.watch) {
    audit_get_watch(old.watch);
    new.watch = old.watch;
    }
    return entry;
    }
// Find an existing audit rule.
// Caller must hold audit_filter_mutex to prevent stale rule data.
#[no_mangle]
pub unsafe extern "C" fn audit_find_rule() {
    struct audit_entry *e, *found = core::ptr::null_mut();
    let mut list = core::ptr::null_mut();
    let mut h = 0;
    if (entry.rule.inode_f) {
    h = audit_hash_ino(entry.rule.inode_f.val);
// p = list = &audit_inode_hash[h];
    } else if (entry.rule.watch) {
// we don't know the inode number, so must walk entire hash
    for (h = 0; h < AUDIT_INODE_BUCKETS; h++) {
    list = &audit_inode_hash[h];
    list_for_each_entry(e, list, list)
    if (!audit_compare_rule(&entry.rule, &e.rule)) {
    found = e;
    goto out;
    }
    }
    goto out;
    } else {
// p = list = &audit_filter_list[entry->rule.listnr];
    }
    list_for_each_entry(e, list, list)
    if (!audit_compare_rule(&entry.rule, &e.rule)) {
    found = e;
    goto out;
    }
    out:
    return found;
    }
pub static mut prio_low: u64 = ~0ULL/2;
pub static mut prio_high: u64 = ~0ULL/2 - 1;
// Add rule to given filterlist if not a duplicate.
#[no_mangle]
pub unsafe extern "C" fn audit_add_rule(entry: *mut audit_entry) -> c_int {
    let mut e = core::ptr::null_mut();
    struct audit_watch *watch = entry.rule.watch;
    struct audit_tree *tree = entry.rule.tree;
    let mut list = core::ptr::null_mut();
pub static mut err: c_int = 0;

pub static mut dont_count: c_int = 0;
// If any of these, don't count towards total
    match (entry.rule.listnr) {
    AUDIT_FILTER_USER => {
    AUDIT_FILTER_EXCLUDE => {
    AUDIT_FILTER_FS => {
    dont_count = 1;
    }

    mutex_lock(&audit_filter_mutex);
    e = audit_find_rule(entry, &list);
    if (e) {
    mutex_unlock(&audit_filter_mutex);
    err = -EEXIST;
// normally audit_add_tree_rule() will free it on failure
    if (tree) {
    audit_put_tree(tree);
    }
    return err;
    }
    if (watch) {
// audit_filter_mutex is dropped and re-taken during this call
    err = audit_add_watch(&entry.rule, &list);
    if (err) {
    mutex_unlock(&audit_filter_mutex);
//
// normally audit_add_tree_rule() will free it
// on failure
//
    if (tree) {
    audit_put_tree(tree);
    }
    return err;
    }
    }
    if (tree) {
    err = audit_add_tree_rule(&entry.rule);
    if (err) {
    mutex_unlock(&audit_filter_mutex);
    return err;
    }
    }
    entry.rule.prio = ~0ULL;
    if (entry.rule.listnr == AUDIT_FILTER_EXIT ||
    entry.rule.listnr == AUDIT_FILTER_URING_EXIT) {
    if (entry.rule.flags & AUDIT_FILTER_PREPEND) {
    entry.rule.prio = ++prio_high;
    }
    else {
    entry.rule.prio = --prio_low;
    }
    }
    if (entry.rule.flags & AUDIT_FILTER_PREPEND) {
    list_add(&entry.rule.list,
    &audit_rules_list[entry.rule.listnr]);
    list_add_rcu(&entry.list, list);
    entry.rule.flags &= ~AUDIT_FILTER_PREPEND;
    } else {
    list_add_tail(&entry.rule.list,
    &audit_rules_list[entry.rule.listnr]);
    list_add_tail_rcu(&entry.list, list);
    }

    if (!dont_count) {
    audit_n_rules++;
    }
    if (!audit_match_signal(entry)) {
    audit_signals++;
    }

    mutex_unlock(&audit_filter_mutex);
    return err;
    }
// Remove an existing rule from filterlist.
#[no_mangle]
pub unsafe extern "C" fn audit_del_rule(entry: *mut audit_entry) -> c_int {
    let mut e = core::ptr::null_mut();
    let mut list = core::ptr::null_mut();
pub static mut ret: c_int = 0;

pub static mut dont_count: c_int = 0;
// If any of these, don't count towards total
    match (entry.rule.listnr) {
    AUDIT_FILTER_USER => {
    AUDIT_FILTER_EXCLUDE => {
    AUDIT_FILTER_FS => {
    dont_count = 1;
    }

    mutex_lock(&audit_filter_mutex);
    e = audit_find_rule(entry, &list);
    if (!e) {
    ret = -ENOENT;
    goto out;
    }
    list_del_rcu(&e.list);
    list_del(&e.rule.list);
    synchronize_rcu();
    if (e.rule.watch) {
    audit_remove_watch_rule(&e.rule);
    }
    if (e.rule.tree) {
    audit_remove_tree_rule(&e.rule);
    }
    if (e.rule.exe) {
    audit_remove_mark_rule(&e.rule);
    }

    if (!dont_count) {
    audit_n_rules--;
    }
    if (!audit_match_signal(entry)) {
    audit_signals--;
    }

    call_rcu(&e.rcu, audit_free_rule_rcu);
    out:
    mutex_unlock(&audit_filter_mutex);
    return ret;
    }
// List rules using struct audit_rule_data.
#[no_mangle]
unsafe extern "C" fn audit_list_rules(seq: c_int, q: *mut sk_buff_head) {
    let mut skb = core::ptr::null_mut();
    let mut r = core::ptr::null_mut();
    let mut i = 0;
// This is a blocking read, so use audit_filter_mutex instead of rcu
// iterator to sync with list writers.
    for (i = 0; i < AUDIT_NR_FILTERS; i++) {
    list_for_each_entry(r, &audit_rules_list[i], list) {
    let mut data = core::ptr::null_mut();
    data = audit_krule_to_data(r);
    if (unlikely(!data)) {
    break;
    }
    skb = audit_make_reply(seq, AUDIT_LIST_RULES, 0, 1,
    data,
    struct_size(data, buf, data.buflen));
    if (skb) {
    skb_queue_tail(q, skb);
    }
    kfree(data);
    }
    }
    skb = audit_make_reply(seq, AUDIT_LIST_RULES, 1, 1, core::ptr::null_mut(), 0);
    if (skb) {
    skb_queue_tail(q, skb);
    }
    }
// Log rule additions and removals
#[no_mangle]
unsafe extern "C" fn audit_log_rule_change(action: *mut c_char, rule: *mut audit_krule, res: c_int) {
    let mut ab = core::ptr::null_mut();
    if (!audit_enabled) {
    return;
    }
    ab = audit_log_start(audit_context(), GFP_KERNEL, AUDIT_CONFIG_CHANGE);
    if (!ab) {
    return;
    }
    audit_log_session_info(ab);
    audit_log_task_context(ab);
    audit_log_format(ab, " op=%s", action);
    audit_log_key(ab, rule.filterkey);
    audit_log_format(ab, " list=%d res=%d", rule.listnr, res);
    audit_log_end(ab);
    }
//
// audit_rule_change - apply all rules to the specified message type
// @type: audit message type
// @seq: netlink audit message sequence (serial) number
// @data: payload data
// @datasz: size of payload data
//
#[no_mangle]
pub unsafe extern "C" fn audit_rule_change(type: c_int, seq: c_int, data: *mut c_void, datasz: usize) -> c_int {
pub static mut err: c_int = 0;
    let mut entry = core::ptr::null_mut();
    match (type) {
    AUDIT_ADD_RULE => {
    entry = audit_data_to_entry(data, datasz);
    if (IS_ERR(entry)) {
    return PTR_ERR(entry);
    }
    err = audit_add_rule(entry);
    audit_log_rule_change("add_rule", &entry.rule, !err);
    break;
    AUDIT_DEL_RULE => {
    entry = audit_data_to_entry(data, datasz);
    if (IS_ERR(entry)) {
    return PTR_ERR(entry);
    }
    err = audit_del_rule(entry);
    audit_log_rule_change("remove_rule", &entry.rule, !err);
    break;
    _ => {
// WARN_ON;
    return -EINVAL;
    }
    if (err || type == AUDIT_DEL_RULE) {
    if (type == AUDIT_DEL_RULE && entry.rule.tree) {
    audit_put_tree(entry.rule.tree);
    }
    if (entry.rule.exe) {
    audit_remove_mark(entry.rule.exe);
    }
    audit_free_rule(entry);
    }
    return err;
    }
//
// audit_list_rules_send - list the audit rules
// @request_skb: skb of request we are replying to (used to target the reply)
// @seq: netlink audit message sequence (serial) number
//
#[no_mangle]
pub unsafe extern "C" fn audit_list_rules_send(request_skb: *mut sk_buff, seq: c_int) -> c_int {
    let mut tsk = core::ptr::null_mut();
    let mut dest = core::ptr::null_mut();
// We can't just spew out the rules here because we might fill
// the available socket buffer space and deadlock waiting for
// auditctl to read from it... which isn't ever going to
// happen if we're actually running in the context of auditctl
// trying to _send_ the stuff
    dest = kmalloc_obj(*dest);
    if (!dest) {
    return -ENOMEM;
    }
    dest.net = get_net(sock_net(NETLINK_CB(request_skb).sk));
    dest.portid = NETLINK_CB(request_skb).portid;
    skb_queue_head_init(&dest.q);
    mutex_lock(&audit_filter_mutex);
    audit_list_rules(seq, &dest.q);
    mutex_unlock(&audit_filter_mutex);
    tsk = kthread_run(audit_send_list_thread, dest, "audit_send_list");
    if (IS_ERR(tsk)) {
    skb_queue_purge(&dest.q);
    put_net(dest.net);
    kfree(dest);
    return PTR_ERR(tsk);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_comparator(left: u32, op: u32, right: u32) -> c_int {
    match (op) {
    Audit_equal => {
    return (left == right);
    Audit_not_equal => {
    return (left != right);
    Audit_lt => {
    return (left < right);
    Audit_le => {
    return (left <= right);
    Audit_gt => {
    return (left > right);
    Audit_ge => {
    return (left >= right);
    Audit_bitmask => {
    return (left & right);
    Audit_bittest => {
    return ((left & right) == right);
    _ => {
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn audit_uid_comparator(left: kuid_t, op: u32, right: kuid_t) -> c_int {
    match (op) {
    Audit_equal => {
    return uid_eq(left, right);
    Audit_not_equal => {
    return !uid_eq(left, right);
    Audit_lt => {
    return uid_lt(left, right);
    Audit_le => {
    return uid_lte(left, right);
    Audit_gt => {
    return uid_gt(left, right);
    Audit_ge => {
    return uid_gte(left, right);
    Audit_bitmask => {
    Audit_bittest => {
    _ => {
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn audit_gid_comparator(left: kgid_t, op: u32, right: kgid_t) -> c_int {
    match (op) {
    Audit_equal => {
    return gid_eq(left, right);
    Audit_not_equal => {
    return !gid_eq(left, right);
    Audit_lt => {
    return gid_lt(left, right);
    Audit_le => {
    return gid_lte(left, right);
    Audit_gt => {
    return gid_gt(left, right);
    Audit_ge => {
    return gid_gte(left, right);
    Audit_bitmask => {
    Audit_bittest => {
    _ => {
    return 0;
    }
    }
//
// parent_len - find the length of the parent portion of a pathname
// @path: pathname of which to determine length
//
#[no_mangle]
pub unsafe extern "C" fn parent_len(path: *const c_char) -> c_int {
    let mut plen = 0;
    let mut p = core::ptr::null_mut();
    plen = strlen(path);
    if (plen == 0) {
    return plen;
    }
// disregard trailing slashes
    p = path + plen - 1;
    while ((*p == '/') && (p > path))
    p--;
// walk backward until we find the next slash or hit beginning
    while ((*p != '/') && (p > path))
    p--;
// did we find a slash? Then increment to include it in path
    if (*p == '/') {
    p++;
    }
    return p - path;
    }
//
// audit_compare_dname_path - compare given dentry name with last component in
// given path. Return of 0 indicates a match.
// @dname:	dentry name that we're comparing
// @path:	full pathname that we're comparing
// @parentlen:	length of the parent if known. Passing in AUDIT_NAME_FULL
// here indicates that we must compute this value.
//
#[no_mangle]
pub unsafe extern "C" fn audit_compare_dname_path(dname: *const qstr, path: *const c_char, parentlen: c_int) -> c_int {
    int dlen, pathlen;
    let mut p = core::ptr::null_mut();
    dlen = dname.len;
    pathlen = strlen(path);
    if (pathlen < dlen) {
    return 1;
    }
    if (parentlen == AUDIT_NAME_FULL) {
    parentlen = parent_len(path);
    }
    p = path + parentlen;
// handle trailing slashes
    pathlen -= parentlen;
    while (pathlen > 0 && p[pathlen - 1] == '/')
    pathlen--;
    if (pathlen != dlen) {
    return 1;
    }
    return memcmp(p, dname.name, dlen);
    }
#[no_mangle]
pub unsafe extern "C" fn audit_filter(msgtype: c_int, listtype: c_uint) -> c_int {
    let mut e = core::ptr::null_mut();
    int ret = 1; /* Audit by default */
    rcu_read_lock();
    list_for_each_entry_rcu(e, &audit_filter_list[listtype], list) {
    int i, result = 0;
    for (i = 0; i < e.rule.field_count; i++) {
    struct audit_field *f = &e.rule.fields[i];
pub static mut prop: lsm_prop = { };
    let mut pid = 0;
    match (f.type) {
    AUDIT_PID => {
    pid = task_tgid_nr(current);
    result = audit_comparator(pid, f.op, f.val);
    break;
    AUDIT_UID => {
    result = audit_uid_comparator(current_uid(), f.op, f.uid);
    break;
    AUDIT_GID => {
    result = audit_gid_comparator(current_gid(), f.op, f.gid);
    break;
    AUDIT_LOGINUID => {
    result = audit_uid_comparator(audit_get_loginuid(current),
    f.op, f.uid);
    break;
    AUDIT_LOGINUID_SET => {
    result = audit_comparator(audit_loginuid_set(current),
    f.op, f.val);
    break;
    AUDIT_MSGTYPE => {
    result = audit_comparator(msgtype, f.op, f.val);
    break;
    AUDIT_SUBJ_USER => {
    AUDIT_SUBJ_ROLE => {
    AUDIT_SUBJ_TYPE => {
    AUDIT_SUBJ_SEN => {
    AUDIT_SUBJ_CLR => {
    if (f.lsm_rule) {
    security_current_getlsmprop_subj(&prop);
    result = security_audit_rule_match(
    &prop, f.type, f.op,
    f.lsm_rule);
    }
    break;
    AUDIT_EXE => {
    result = audit_exe_compare(current, e.rule.exe);
    if (f.op == Audit_not_equal) {
    result = !result;
    }
    break;
    _ => {
    goto unlock_and_return;
    }
    if (result < 0) /* error */
    goto unlock_and_return;
    if (!result) {
    break;
    }
    }
    if (result > 0) {
    if (e.rule.action == AUDIT_NEVER || listtype == AUDIT_FILTER_EXCLUDE) {
    ret = 0;
    }
    break;
    }
    }
    unlock_and_return:
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn update_lsm_rule(r: *mut audit_krule) -> c_int {
    struct audit_entry *entry = container_of(r, struct audit_entry, rule);
    let mut nentry = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (!security_audit_rule_known(r)) {
    return 0;
    }
    nentry = audit_dupe_rule(r, core::ptr::null_mut());
    if (entry.rule.exe) {
    audit_remove_mark(entry.rule.exe);
    }
    if (IS_ERR(nentry)) {
// save the first error encountered for the
// return value
    err = PTR_ERR(nentry);
    audit_panic("error updating LSM filters");
    if (r.watch) {
    list_del(&r.rlist);
    }
    list_del_rcu(&entry.list);
    list_del(&r.list);
    } else {
    if (r.watch || r.tree) {
    list_replace_init(&r.rlist, &nentry.rule.rlist);
    }
    list_replace_rcu(&entry.list, &nentry.list);
    list_replace(&r.list, &nentry.rule.list);
    }
    call_rcu(&entry.rcu, audit_free_rule_rcu);
    return err;
    }
// This function will re-initialize the lsm_rule field of all applicable rules.
// It will traverse the filter lists searching for rules that contain LSM
// specific filter fields.  When such a rule is found, it is copied, the
// LSM field is re-initialized, and the old rule is replaced with the
// updated rule.
#[no_mangle]
pub unsafe extern "C" fn audit_update_lsm_rules() -> c_int {
    struct audit_krule *r, *n;
    int i, err = 0;
// audit_filter_mutex synchronizes the writers
    mutex_lock(&audit_filter_mutex);
    for (i = 0; i < AUDIT_NR_FILTERS; i++) {
    list_for_each_entry_safe(r, n, &audit_rules_list[i], list) {
pub static mut res: c_int = update_lsm_rule(r);
    if (!err) {
    err = res;
    }
    }
    }
    mutex_unlock(&audit_filter_mutex);
    return err;
    }
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}