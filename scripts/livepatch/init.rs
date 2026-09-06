//! Automatically rewritten from C to Rust
//! Source: scripts/livepatch/init.c
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
//
// Init code for a livepatch kernel module
//

    static struct klp_patch *patch;
#[no_mangle]
unsafe extern "C" fn livepatch_mod_init() -> int __init {
    static int __init livepatch_mod_init(void)
    {
    struct klp_object_ext *obj_exts;
    size_t obj_exts_sec_size;
    struct klp_object *objs;
    unsigned int nr_objs;
    int ret;
    obj_exts = klp_find_section_by_name(THIS_MODULE, ".init.klp_objects",
    &obj_exts_sec_size);
    nr_objs = obj_exts_sec_size / sizeof(*obj_exts);
    if (!nr_objs) {
    pr_err("nothing to patch!\n");
    ret = -EINVAL;
    goto err;
    }
    patch = kzalloc_obj(*patch);
    if (!patch) {
    ret = -ENOMEM;
    goto err;
    }
    objs = kzalloc(sizeof(struct klp_object) * (nr_objs + 1),  GFP_KERNEL);
    if (!objs) {
    ret = -ENOMEM;
    goto err_free_patch;
    }
    for (int i = 0; i < nr_objs; i++) {
    struct klp_object_ext *obj_ext = obj_exts + i;
    struct klp_func_ext *funcs_ext = obj_ext.funcs;
    let mut nr_funcs: c_uint = obj_ext.nr_funcs;
    struct klp_func *funcs = objs[i].funcs;
    struct klp_object *obj = objs + i;
    funcs = kzalloc(sizeof(struct klp_func) * (nr_funcs + 1), GFP_KERNEL);
    if (!funcs) {
    ret = -ENOMEM;
    for (int j = 0; j < i; j++)
    kfree(objs[i].funcs);
    goto err_free_objs;
    }
    for (int j = 0; j < nr_funcs; j++) {
    funcs[j].old_name   = funcs_ext[j].old_name;
    funcs[j].new_func   = funcs_ext[j].new_func;
    funcs[j].old_sympos = funcs_ext[j].sympos;
    }
    obj.name = obj_ext.name;
    obj.funcs = funcs;
    memcpy(&obj.callbacks, &obj_ext.callbacks, sizeof(struct klp_callbacks));
    }
    patch.mod = THIS_MODULE;
    patch.objs = objs;
// TODO patch->states

    patch.replace = false;

    patch.replace = true;

    return klp_enable_patch(patch);
    err_free_objs:
    kfree(objs);
    err_free_patch:
    kfree(patch);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn livepatch_mod_exit() -> void __exit {
    static void __exit livepatch_mod_exit(void)
    {
    struct klp_object *obj;
    klp_for_each_object_static(patch, obj)
    kfree(obj.funcs);
    kfree(patch.objs);
    kfree(patch);
    }
    module_init(livepatch_mod_init);
    module_exit(livepatch_mod_exit);
    MODULE_LICENSE("GPL");
    MODULE_INFO(livepatch, "Y");
    MODULE_DESCRIPTION("Livepatch module");
