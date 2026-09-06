//! Automatically rewritten from C to Rust
//! Source: kernel/params.c
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



// SPDX-License-Identifier: GPL-2.0-or-later
//
// Helpers for initial module or kernel cmdline parsing
// Copyright (C) 2001 Rusty Russell.
//

// Protects all built-in parameters, modules use their own param_lock
// static DEFINE_MUTEX(param_lock);
// Use the module's mutex, or if built-in use the built-in mutex

#[no_mangle]
pub unsafe extern "C" fn check_kparam_locked(mod: *mut module) {
    BUG_ON(!mutex_is_locked(KPARAM_MUTEX(mod)));
    }

#[no_mangle]
pub unsafe extern "C" fn check_kparam_locked(mod: *mut module) {
    }

// This just allows us to keep track of which parameters are kmalloced.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmalloced_param {
    pub list: list_head,
    pub val: [c_char; ],
}
// static LIST_HEAD(kmalloced_params);
// static DEFINE_SPINLOCK(kmalloced_params_lock);
    static void *kmalloc_parameter(unsigned int size)
    {
    struct kmalloced_param *p;
    p = kmalloc(size_add(sizeof(*p), size), GFP_KERNEL);
    if (!p)
    return core::ptr::null_mut();
    spin_lock(&kmalloced_params_lock);
    list_add(&p.list, &kmalloced_params);
    spin_unlock(&kmalloced_params_lock);
    return p.val;
    }
// Does nothing if parameter wasn't kmalloced above.
#[no_mangle]
unsafe extern "C" fn maybe_kfree_parameter(param: *mut c_void) {
    struct kmalloced_param *p;
    spin_lock(&kmalloced_params_lock);
    list_for_each_entry(p, &kmalloced_params, list) {
    if (p.val == param) {
    list_del(&p.list);
    kfree(p);
    break;
    }
    }
    spin_unlock(&kmalloced_params_lock);
    }
#[no_mangle]
unsafe extern "C" fn dash2underscore(c: c_char) -> c_char {
    if (c == '-')
    return '_';
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn parameqn(a: *const c_char, b: *const c_char, n: usize) -> bool {
    size_t i;
    for (i = 0; i < n; i++) {
    if (dash2underscore(a[i]) != dash2underscore(b[i]))
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn parameq(a: *const c_char, b: *const c_char) -> bool {
    return parameqn(a, b, strlen(a)+1);
    }
#[no_mangle]
unsafe extern "C" fn param_check_unsafe(kp: *const kernel_param) -> bool {
    if (kp.flags & KERNEL_PARAM_FL_HWPARAM &&
    security_locked_down(LOCKDOWN_MODULE_PARAMETERS))
    return false;
    if (kp.flags & KERNEL_PARAM_FL_UNSAFE) {
    pr_notice("Setting dangerous option %s - tainting kernel\n",
    kp.name);
    add_taint(TAINT_USER, LOCKDEP_STILL_OK);
    }
    return true;
    }
    static int parse_one(char *param,
    char *val,
    const char *doing,
    const struct kernel_param *params,
    unsigned num_params,
    s16 min_level,
    s16 max_level,
    void *arg, parse_unknown_fn handle_unknown)
    {
    unsigned int i;
    int err;
// Find parameter
    for (i = 0; i < num_params; i++) {
    if (parameq(param, params[i].name)) {
    if (params[i].level < min_level
    || params[i].level > max_level)
    return 0;
// No one handled NULL, so do it here.
    if (!val &&
    !(params[i].ops.flags & KERNEL_PARAM_OPS_FL_NOARG))
    return -EINVAL;
    pr_debug("handling %s with value '%s'\n", param,
    val ? val : "no-arg");
    kernel_param_lock(params[i].mod);
    if (param_check_unsafe(&params[i]))
    err = params[i].ops.set(val, &params[i]);
    else
    err = -EPERM;
    kernel_param_unlock(params[i].mod);
    return err;
    }
    }
    if (handle_unknown) {
    pr_debug("doing %s: %s='%s'\n", doing, param, val);
    return handle_unknown(param, val, doing, arg);
    }
    pr_debug("Unknown argument '%s'\n", param);
    return -ENOENT;
    }
// Args looks like "foo=bar,bar2 baz=fuz wiz".
    char *parse_args(const char *doing,
    char *args,
    const struct kernel_param *params,
    unsigned int num,
    s16 min_level,
    s16 max_level,
    void *arg, parse_unknown_fn unknown)
    {
    char *param, *val, *err = core::ptr::null_mut();
// Chew leading spaces
    args = skip_spaces(args);
    if (*args)
    pr_debug("doing %s, parsing ARGS: '%s'\n", doing, args);
    while (*args) {
    int ret;
    int irq_was_disabled;
    args = next_arg(args, &param, &val);
// Stop at --
    if (!val && strcmp(param, "--") == 0)
    return err ?: args;
    irq_was_disabled = irqs_disabled();
    ret = parse_one(param, val, doing, params, num,
    min_level, max_level, arg, unknown);
    if (irq_was_disabled && !irqs_disabled())
    pr_warn("%s: option '%s' enabled irq's!\n",
    doing, param);
    switch (ret) {
    case 0:
    continue;
    case -ENOENT:
    pr_err("%s: Unknown parameter `%s'\n", doing, param);
    break;
    case -ENOSPC:
    pr_err("%s: `%s' too large for parameter `%s'\n",
    doing, val ?: "", param);
    break;
    default:
    pr_err("%s: `%s' invalid for parameter `%s'\n",
    doing, val ?: "", param);
    break;
    }
    err = ERR_PTR(ret);
    }
    return err;
    }
// Lazy bastard, eh?

    int param_set_##name(const char *val, const struct kernel_param *kp) \
    {								\
    return strtolfn(val, 0, (type *)kp.arg);		\
    }								\
    int param_get_##name(char *buffer, const struct kernel_param *kp) \
    {								\
    return scnprintf(buffer, PAGE_SIZE, format "\n",	\
// ((type *)kp->arg));			\
    }								\
    const struct kernel_param_ops param_ops_##name = {			\
    .set = param_set_##name,				\
    .get = param_get_##name,				\
    };								\
    EXPORT_SYMBOL(param_set_##name);				\
    EXPORT_SYMBOL(param_get_##name);				\
    EXPORT_SYMBOL(param_ops_##name)
    STANDARD_PARAM_DEF(byte,	unsigned char,		"%hhu",		kstrtou8);
    STANDARD_PARAM_DEF(short,	short,			"%hi",		kstrtos16);
    STANDARD_PARAM_DEF(ushort,	unsigned short,		"%hu",		kstrtou16);
    STANDARD_PARAM_DEF(int,		int,			"%i",		kstrtoint);
    STANDARD_PARAM_DEF(uint,	unsigned int,		"%u",		kstrtouint);
    STANDARD_PARAM_DEF(long,	long,			"%li",		kstrtol);
    STANDARD_PARAM_DEF(ulong,	unsigned long,		"%lu",		kstrtoul);
    STANDARD_PARAM_DEF(ullong,	unsigned long long,	"%llu",		kstrtoull);
    STANDARD_PARAM_DEF(hexint,	unsigned int,		"%#08x", 	kstrtouint);
    int param_set_uint_minmax(const char *val, const struct kernel_param *kp,
    unsigned int min, unsigned int max)
    {
    unsigned int num;
    int ret;
    if (!val)
    return -EINVAL;
    ret = kstrtouint(val, 0, &num);
    if (ret)
    return ret;
    if (num < min || num > max)
    return -EINVAL;
// ((unsigned int *)kp->arg) = num;
    return 0;
    }
    EXPORT_SYMBOL_GPL(param_set_uint_minmax);
#[no_mangle]
pub unsafe extern "C" fn param_set_charp(val: *const c_char, kp: *const kernel_param) -> c_int {
    char *tmp;
    size_t len, maxlen = 1024;
    len = strnlen(val, maxlen + 1);
    if (len == maxlen + 1) {
    pr_err("%s: string parameter too long\n", kp.name);
    return -ENOSPC;
    }
//
// This is a hack. We can't kmalloc() in early boot, and we
// don't need to; this mangled commandline is preserved.
//
    if (slab_is_available()) {
    tmp = kmalloc_parameter(len + 1);
    if (!tmp)
    return -ENOMEM;
    memcpy(tmp, val, len + 1);
    } else
    tmp = (char *)val;
    maybe_kfree_parameter(*(char **)kp.arg);
// (char **)kp->arg = tmp;
    return 0;
    }
    EXPORT_SYMBOL(param_set_charp);
#[no_mangle]
pub unsafe extern "C" fn param_get_charp(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    return scnprintf(buffer, PAGE_SIZE, "%s\n", *((char **)kp.arg));
    }
    EXPORT_SYMBOL(param_get_charp);
#[no_mangle]
pub unsafe extern "C" fn param_free_charp(arg: *mut c_void) {
    maybe_kfree_parameter(*((char **)arg));
    }
    EXPORT_SYMBOL(param_free_charp);
    const struct kernel_param_ops param_ops_charp = {
    .set = param_set_charp,
    .get = param_get_charp,
    .free = param_free_charp,
    };
    EXPORT_SYMBOL(param_ops_charp);
// Actually could be a bool or an int, for historical reasons.
#[no_mangle]
pub unsafe extern "C" fn param_set_bool(val: *const c_char, kp: *const kernel_param) -> c_int {
// No equals means "set"...
    if (!val) val = "1";
// One of =[yYnN01]
    return kstrtobool(val, kp.arg);
    }
    EXPORT_SYMBOL(param_set_bool);
#[no_mangle]
pub unsafe extern "C" fn param_get_bool(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
// Y and N chosen as being relatively non-coder friendly
    return sprintf(buffer, "%c\n", *(bool *)kp.arg ? 'Y' : 'N');
    }
    EXPORT_SYMBOL(param_get_bool);
    const struct kernel_param_ops param_ops_bool = {
    .flags = KERNEL_PARAM_OPS_FL_NOARG,
    .set = param_set_bool,
    .get = param_get_bool,
    };
    EXPORT_SYMBOL(param_ops_bool);
#[no_mangle]
pub unsafe extern "C" fn param_set_bool_enable_only(val: *const c_char, kp: *const kernel_param) -> c_int {
    int err;
    bool new_value;
    let mut orig_value: bool = *(bool *)kp.arg;
    let mut dummy_kp: kernel_param = *kp;
    dummy_kp.arg = &new_value;
    err = param_set_bool(val, &dummy_kp);
    if (err)
    return err;
// Don't let them unset it once it's set!
    if (!new_value && orig_value)
    return -EROFS;
    if (new_value)
    err = param_set_bool(val, kp);
    return err;
    }
    EXPORT_SYMBOL_GPL(param_set_bool_enable_only);
    const struct kernel_param_ops param_ops_bool_enable_only = {
    .flags = KERNEL_PARAM_OPS_FL_NOARG,
    .set = param_set_bool_enable_only,
    .get = param_get_bool,
    };
    EXPORT_SYMBOL_GPL(param_ops_bool_enable_only);
// This one must be bool.
#[no_mangle]
pub unsafe extern "C" fn param_set_invbool(val: *const c_char, kp: *const kernel_param) -> c_int {
    int ret;
    bool boolval;
    struct kernel_param dummy;
    dummy.arg = &boolval;
    ret = param_set_bool(val, &dummy);
    if (ret == 0)
// (bool *)kp->arg = !boolval;
    return ret;
    }
    EXPORT_SYMBOL(param_set_invbool);
#[no_mangle]
pub unsafe extern "C" fn param_get_invbool(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    return sprintf(buffer, "%c\n", (*(bool *)kp.arg) ? 'N' : 'Y');
    }
    EXPORT_SYMBOL(param_get_invbool);
    const struct kernel_param_ops param_ops_invbool = {
    .set = param_set_invbool,
    .get = param_get_invbool,
    };
    EXPORT_SYMBOL(param_ops_invbool);
#[no_mangle]
pub unsafe extern "C" fn param_set_bint(val: *const c_char, kp: *const kernel_param) -> c_int {
// Match bool exactly, by re-using it.
    let mut boolkp: kernel_param = *kp;
    bool v;
    int ret;
    boolkp.arg = &v;
    ret = param_set_bool(val, &boolkp);
    if (ret == 0)
// (int *)kp->arg = v;
    return ret;
    }
    EXPORT_SYMBOL(param_set_bint);
    const struct kernel_param_ops param_ops_bint = {
    .flags = KERNEL_PARAM_OPS_FL_NOARG,
    .set = param_set_bint,
    .get = param_get_int,
    };
    EXPORT_SYMBOL(param_ops_bint);
// We break the rule and mangle the string.
    static int param_array(struct module *mod,
    const char *name,
    const char *val,
    unsigned int min, unsigned int max,
    void *elem, int elemsize,
    int (*set)(const char *, const struct kernel_param *kp),
    s16 level,
    unsigned int *num)
    {
    int ret;
    struct kernel_param kp;
    char save;
// Get the name right for errors.
    kp.name = name;
    kp.arg = elem;
    kp.level = level;
// num = 0;
// We expect a comma-separated list of values.
    do {
    int len;
    if (*num == max) {
    pr_err("%s: can only take %i arguments\n", name, max);
    return -EINVAL;
    }
    len = strcspn(val, ",");
// nul-terminate and parse
    save = val[len];
    ((char *)val)[len] = '\0';
    check_kparam_locked(mod);
    ret = set(val, &kp);
    if (ret != 0)
    return ret;
    kp.arg += elemsize;
    val += len+1;
    (*num)++;
    } while (save == ',');
    if (*num < min) {
    pr_err("%s: needs at least %i arguments\n", name, min);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn param_array_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    const struct kparam_array *arr = kp.arr;
    unsigned int temp_num;
    return param_array(kp.mod, kp.name, val, 1, arr.max, arr.elem,
    arr.elemsize, arr.ops.set, kp.level,
    arr.num ?: &temp_num);
    }
#[no_mangle]
unsafe extern "C" fn param_array_get(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    int i, off, ret;
    const struct kparam_array *arr = kp.arr;
    let mut p: kernel_param = *kp;
    for (i = off = 0; i < (arr.num ? *arr.num : arr.max); i++) {
// Replace \n with comma
    if (i)
    buffer[off - 1] = ',';
    p.arg = arr.elem + arr.elemsize * i;
    check_kparam_locked(p.mod);
    ret = arr.ops.get(buffer + off, &p);
    if (ret < 0)
    return ret;
    off += ret;
    }
    buffer[off] = '\0';
    return off;
    }
#[no_mangle]
unsafe extern "C" fn param_array_free(arg: *mut c_void) {
    unsigned int i;
    const struct kparam_array *arr = arg;
    if (arr.ops.free)
    for (i = 0; i < (arr.num ? *arr.num : arr.max); i++)
    arr.ops.free(arr.elem + arr.elemsize * i);
    }
    const struct kernel_param_ops param_array_ops = {
    .set = param_array_set,
    .get = param_array_get,
    .free = param_array_free,
    };
    EXPORT_SYMBOL(param_array_ops);
#[no_mangle]
pub unsafe extern "C" fn param_set_copystring(val: *const c_char, kp: *const kernel_param) -> c_int {
    const struct kparam_string *kps = kp.str;
    let mut len: usize = strnlen(val, kps.maxlen);
    if (len == kps.maxlen) {
    pr_err("%s: string doesn't fit in %u chars.\n",
    kp.name, kps.maxlen-1);
    return -ENOSPC;
    }
    memcpy(kps.string, val, len + 1);
    return 0;
    }
    EXPORT_SYMBOL(param_set_copystring);
#[no_mangle]
pub unsafe extern "C" fn param_get_string(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    const struct kparam_string *kps = kp.str;
    return scnprintf(buffer, PAGE_SIZE, "%s\n", kps.string);
    }
    EXPORT_SYMBOL(param_get_string);
    const struct kernel_param_ops param_ops_string = {
    .set = param_set_copystring,
    .get = param_get_string,
    };
    EXPORT_SYMBOL(param_ops_string);
// sysfs output in /sys/module/XYZ/parameters/

    struct param_attribute
    {
    struct module_attribute mattr;
    const struct kernel_param *param;
    };
    struct module_param_attrs
    {
    unsigned int num;
    struct attribute_group grp;
    struct param_attribute attrs[] __counted_by(num);
    };

    static ssize_t param_attr_show(const struct module_attribute *mattr,
    struct module_kobject *mk, char *buf)
    {
    int count;
    const struct param_attribute *attribute = to_param_attr(mattr);
    if (!attribute.param.ops.get)
    return -EPERM;
    kernel_param_lock(mk.mod);
    count = attribute.param.ops.get(buf, attribute.param);
    kernel_param_unlock(mk.mod);
    return count;
    }
// sysfs always hands a nul-terminated string in buf.  We rely on that.
    static ssize_t param_attr_store(const struct module_attribute *mattr,
    struct module_kobject *mk,
    const char *buf, size_t len)
    {
    int err;
    const struct param_attribute *attribute = to_param_attr(mattr);
    if (!attribute.param.ops.set)
    return -EPERM;
    kernel_param_lock(mk.mod);
    if (param_check_unsafe(attribute.param))
    err = attribute.param.ops.set(buf, attribute.param);
    else
    err = -EPERM;
    kernel_param_unlock(mk.mod);
    if (!err)
    return len;
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn kernel_param_lock(mod: *mut module) {
    mutex_lock(KPARAM_MUTEX(mod));
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_param_unlock(mod: *mut module) {
    mutex_unlock(KPARAM_MUTEX(mod));
    }
    EXPORT_SYMBOL(kernel_param_lock);
    EXPORT_SYMBOL(kernel_param_unlock);
//
// add_sysfs_param - add a parameter to sysfs
// @mk: struct module_kobject
// @kp: the actual parameter definition to add to sysfs
// @name: name of parameter
//
// Create a kobject if for a (per-module) parameter if mp NULL, and
// create file in sysfs.  Returns an error on out of memory.  Always cleans up
// if there's an error.
//
    static __init_or_module int add_sysfs_param(struct module_kobject *mk,
    const struct kernel_param *kp,
    const char *name)
    {
    struct module_param_attrs *new_mp;
    struct attribute **new_attrs;
    unsigned int i;
// We don't bother calling this with invisible parameters.
    BUG_ON(!kp.perm);
    if (!mk.mp) {
// First allocation.
    mk.mp = kzalloc_obj(*mk.mp);
    if (!mk.mp)
    return -ENOMEM;
    mk.mp.grp.name = "parameters";
// NULL-terminated attribute array.
    mk.mp.grp.attrs = kzalloc_obj(mk.mp.grp.attrs[0]);
// Caller will cleanup via free_module_param_attrs
    if (!mk.mp.grp.attrs)
    return -ENOMEM;
    }
// Enlarge allocations.
    new_mp = krealloc(mk.mp, struct_size(mk.mp, attrs, mk.mp.num + 1),
    GFP_KERNEL);
    if (!new_mp)
    return -ENOMEM;
    mk.mp = new_mp;
    mk.mp.num++;
// Extra pointer for NULL terminator
    new_attrs = krealloc_array(mk.mp.grp.attrs, mk.mp.num + 1,
    sizeof(mk.mp.grp.attrs[0]), GFP_KERNEL);
    if (!new_attrs)
    return -ENOMEM;
    mk.mp.grp.attrs = new_attrs;
// Tack new one on the end.
    memset(&mk.mp.attrs[mk.mp.num - 1], 0, sizeof(mk.mp.attrs[0]));
    sysfs_attr_init(&mk.mp.attrs[mk.mp.num - 1].mattr.attr);
    mk.mp.attrs[mk.mp.num - 1].param = kp;
    mk.mp.attrs[mk.mp.num - 1].mattr.show = param_attr_show;
// Do not allow runtime DAC changes to make param writable.
    if ((kp.perm & (S_IWUSR | S_IWGRP | S_IWOTH)) != 0)
    mk.mp.attrs[mk.mp.num - 1].mattr.store = param_attr_store;
    else
    mk.mp.attrs[mk.mp.num - 1].mattr.store = core::ptr::null_mut();
    mk.mp.attrs[mk.mp.num - 1].mattr.attr.name = (char *)name;
    mk.mp.attrs[mk.mp.num - 1].mattr.attr.mode = kp.perm;
// Fix up all the pointers, since krealloc can move us
    for (i = 0; i < mk.mp.num; i++)
    mk.mp.grp.attrs[i] = &mk.mp.attrs[i].mattr.attr;
    mk.mp.grp.attrs[mk.mp.num] = core::ptr::null_mut();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn free_module_param_attrs(mk: *mut module_kobject) {
    if (mk.mp)
    kfree(mk.mp.grp.attrs);
    kfree(mk.mp);
    mk.mp = core::ptr::null_mut();
    }
//
// module_param_sysfs_setup - setup sysfs support for one module
// @mod: module
// @kparam: module parameters (array)
// @num_params: number of module parameters
//
// Adds sysfs entries for module parameters under
// /sys/module/[mod->name]/parameters
//
    int module_param_sysfs_setup(struct module *mod,
    const struct kernel_param *kparam,
    unsigned int num_params)
    {
    int i, err;
    let mut params: bool = false;
    for (i = 0; i < num_params; i++) {
    if (kparam[i].perm == 0)
    continue;
    err = add_sysfs_param(&mod.mkobj, &kparam[i], kparam[i].name);
    if (err) {
    free_module_param_attrs(&mod.mkobj);
    return err;
    }
    params = true;
    }
    if (!params)
    return 0;
// Create the param group.
    err = sysfs_create_group(&mod.mkobj.kobj, &mod.mkobj.mp.grp);
    if (err)
    free_module_param_attrs(&mod.mkobj);
    return err;
    }
//
// module_param_sysfs_remove - remove sysfs support for one module
// @mod: module
//
// Remove sysfs entries for module parameters and the corresponding
// kobject.
//
#[no_mangle]
pub unsafe extern "C" fn module_param_sysfs_remove(mod: *mut module) {
    if (mod.mkobj.mp) {
    sysfs_remove_group(&mod.mkobj.kobj, &mod.mkobj.mp.grp);
//
// We are positive that no one is using any param
// attrs at this point. Deallocate immediately.
//
    free_module_param_attrs(&mod.mkobj);
    }
    }

    struct module_kobject * __init_or_module
    lookup_or_create_module_kobject(const char *name)
    {
    struct module_kobject *mk;
    struct kobject *kobj;
    int err;
    kobj = kset_find_obj(module_kset, name);
    if (kobj)
    return to_module_kobject(kobj);
    mk = kzalloc_obj(struct module_kobject);
    if (!mk)
    return core::ptr::null_mut();
    mk.mod = THIS_MODULE;
    mk.kobj.kset = module_kset;
    err = kobject_init_and_add(&mk.kobj, &module_ktype, core::ptr::null_mut(), "%s", name);
    if (IS_ENABLED(CONFIG_MODULES) && !err)
    err = sysfs_create_file(&mk.kobj, &module_uevent.attr);
    if (err) {
    kobject_put(&mk.kobj);
    pr_crit("Adding module '%s' to sysfs failed (%d), the system may be unstable.\n",
    name, err);
    return core::ptr::null_mut();
    }
// So that we hold reference in both cases.
    kobject_get(&mk.kobj);
    return mk;
    }
    static void __init kernel_add_sysfs_param(const char *name,
    const struct kernel_param *kparam,
    unsigned int name_skip)
    {
    struct module_kobject *mk;
    int err;
    mk = lookup_or_create_module_kobject(name);
    if (!mk)
    return;
// We need to remove old parameters before adding more.
    if (mk.mp)
    sysfs_remove_group(&mk.kobj, &mk.mp.grp);
// These should not fail at boot.
    err = add_sysfs_param(mk, kparam, kparam.name + name_skip);
    BUG_ON(err);
    err = sysfs_create_group(&mk.kobj, &mk.mp.grp);
    BUG_ON(err);
    kobject_uevent(&mk.kobj, KOBJ_ADD);
    kobject_put(&mk.kobj);
    }
//
// param_sysfs_builtin - add sysfs parameters for built-in modules
//
// Add module_parameters to sysfs for "modules" built into the kernel.
//
// The "module" name (KBUILD_MODNAME) is stored before a dot, the
// "parameter" name is stored behind a dot in kernel_param->name. So,
// extract the "module" name for all built-in kernel_param-eters,
// and for all who have the same, call kernel_add_sysfs_param.
//
#[no_mangle]
unsafe extern "C" fn param_sysfs_builtin() -> c_int {
    const struct kernel_param *kp;
    unsigned int name_len;
    char modname[MODULE_NAME_LEN];
    for (kp = __start___param; kp < __stop___param; kp++) {
    char *dot;
    if (kp.perm == 0)
    continue;
    dot = strchr(kp.name, '.');
    if (!dot) {
// This happens for core_param()
    strscpy(modname, "kernel");
    name_len = 0;
    } else {
    name_len = dot - kp.name + 1;
    strscpy(modname, kp.name, name_len);
    }
    kernel_add_sysfs_param(modname, kp, name_len);
    }
    }
    ssize_t __modver_version_show(const struct module_attribute *mattr,
    struct module_kobject *mk, char *buf)
    {
    const struct module_version_attribute *vattr =
    container_of_const(mattr, struct module_version_attribute, mattr);
    return scnprintf(buf, PAGE_SIZE, "%s\n", vattr.version);
    }
    extern const struct module_version_attribute __start___modver[];
    extern const struct module_version_attribute __stop___modver[];
#[no_mangle]
unsafe extern "C" fn version_sysfs_builtin() -> c_int {
    const struct module_version_attribute *vattr;
    struct module_kobject *mk;
    int err;
    for (vattr = __start___modver; vattr < __stop___modver; vattr++) {
    mk = lookup_or_create_module_kobject(vattr.module_name);
    if (mk) {
    err = sysfs_create_file(&mk.kobj, &vattr.mattr.attr);
    WARN_ON_ONCE(err);
    kobject_uevent(&mk.kobj, KOBJ_ADD);
    kobject_put(&mk.kobj);
    }
    }
    }
// module-related sysfs stuff
    static ssize_t module_attr_show(struct kobject *kobj,
    struct attribute *attr,
    char *buf)
    {
    const struct module_attribute *attribute;
    struct module_kobject *mk;
    int ret;
    attribute = to_module_attr(attr);
    mk = to_module_kobject(kobj);
    if (!attribute.show)
    return -EIO;
    ret = attribute.show(attribute, mk, buf);
    return ret;
    }
    static ssize_t module_attr_store(struct kobject *kobj,
    struct attribute *attr,
    const char *buf, size_t len)
    {
    const struct module_attribute *attribute;
    struct module_kobject *mk;
    int ret;
    attribute = to_module_attr(attr);
    mk = to_module_kobject(kobj);
    if (!attribute.store)
    return -EIO;
    ret = attribute.store(attribute, mk, buf, len);
    return ret;
    }
    static const struct sysfs_ops module_sysfs_ops = {
    .show = module_attr_show,
    .store = module_attr_store,
    };
#[no_mangle]
unsafe extern "C" fn uevent_filter(kobj: *const kobject) -> c_int {
    const struct kobj_type *ktype = get_ktype(kobj);
    if (ktype == &module_ktype)
    return 1;
    return 0;
    }
    static const struct kset_uevent_ops module_uevent_ops = {
    .filter = uevent_filter,
    };
    struct kset *module_kset;
#[no_mangle]
unsafe extern "C" fn module_kobj_release(kobj: *mut kobject) {
    struct module_kobject *mk = to_module_kobject(kobj);
    if (mk.kobj_completion)
    complete(mk.kobj_completion);
    }
    const struct kobj_type module_ktype = {
    .release   =	module_kobj_release,
    .sysfs_ops =	&module_sysfs_ops,
    };
//
// param_sysfs_init - create "module" kset
//
// This must be done before any driver registration so that when a driver comes
// from a built-in module, the driver core can add the module under /sys/module
// and create the associated driver symlinks.
//
#[no_mangle]
unsafe extern "C" fn param_sysfs_init() -> c_int {
    module_kset = kset_create_and_add("module", &module_uevent_ops, core::ptr::null_mut());
    if (!module_kset) {
    printk(KERN_WARNING "%s (%d): error creating kset\n",
    __FILE__, __LINE__);
    return -ENOMEM;
    }
    return 0;
    }
    pure_initcall(param_sysfs_init);
//
// param_sysfs_builtin_init - add sysfs version and parameter
// attributes for built-in modules
//
#[no_mangle]
unsafe extern "C" fn param_sysfs_builtin_init() -> c_int {
    if (!module_kset)
    return -ENOMEM;
    version_sysfs_builtin();
    param_sysfs_builtin();
    return 0;
    }
    late_initcall(param_sysfs_builtin_init);

//
// module_destroy_params - free all parameters for one module
// @params: module parameters (array)
// @num: number of module parameters
//
#[no_mangle]
pub unsafe extern "C" fn module_destroy_params(params: *const kernel_param, num: c_uint) {
    unsigned int i;
    for (i = 0; i < num; i++)
    if (params[i].ops.free)
    params[i].ops.free(params[i].arg);
    }
