//! Automatically rewritten from C to Rust
//! Source: kernel/module/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Module sysfs support
//
// Copyright (C) 2008 Rusty Russell
//

//
// /sys/module/foo/sections stuff
// J. Corbet <corbet@lwn.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_sect_attrs {
    pub grp: attribute_group,
    pub attrs: [bin_attribute; ],
}

    static ssize_t module_sect_read(struct file *file, struct kobject *kobj,
    const struct bin_attribute *battr,
    char *buf, loff_t pos, size_t count)
    {
    char bounce[MODULE_SECT_READ_SIZE + 1];
    size_t wrote;
    if (pos != 0)
    return -EINVAL;
//
// Since we're a binary read handler, we must account for the
// trailing NUL byte that sprintf will write: if "buf" is
// too small to hold the NUL, or the NUL is exactly the last
// byte, the read will look like it got truncated by one byte.
// Since there is no way to ask sprintf nicely to not write
// the NUL, we have to use a bounce buffer.
//
    wrote = scnprintf(bounce, sizeof(bounce), "0x%px\n",
    kallsyms_show_value(file.f_cred)
    ? battr.private : core::ptr::null_mut());
    count = min(count, wrote);
    memcpy(buf, bounce, count);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn free_sect_attrs(sect_attrs: *mut module_sect_attrs) {
    static void free_sect_attrs(struct module_sect_attrs *sect_attrs)
    {
    const struct bin_attribute *const *bin_attr;
    for (bin_attr = sect_attrs.grp.bin_attrs; *bin_attr; bin_attr++)
    kfree((*bin_attr).attr.name);
    kfree(sect_attrs.grp.bin_attrs);
    kfree(sect_attrs);
    }
#[no_mangle]
unsafe extern "C" fn add_sect_attrs(mod: *mut module, info: *const load_info) -> c_int {
    static int add_sect_attrs(struct module *mod, const struct load_info *info)
    {
    struct module_sect_attrs *sect_attrs;
    const struct bin_attribute **gattr;
    struct bin_attribute *sattr;
    let mut nloaded: c_uint = 0, i;
    int ret;
// Count loaded sections and allocate structures
    for (i = 0; i < info.hdr.e_shnum; i++)
    if (!sect_empty(&info.sechdrs[i]))
    nloaded++;
    sect_attrs = kzalloc_flex(*sect_attrs, attrs, nloaded);
    if (!sect_attrs)
    return -ENOMEM;
    gattr = kzalloc_objs(*gattr, nloaded + 1);
    if (!gattr) {
    kfree(sect_attrs);
    return -ENOMEM;
    }
// Setup section attributes.
    sect_attrs.grp.name = "sections";
    sect_attrs.grp.bin_attrs = gattr;
    sattr = &sect_attrs.attrs[0];
    for (i = 0; i < info.hdr.e_shnum; i++) {
    Elf_Shdr *sec = &info.sechdrs[i];
    if (sect_empty(sec))
    continue;
    sysfs_bin_attr_init(sattr);
    sattr.attr.name =
    kstrdup(info.secstrings + sec.sh_name, GFP_KERNEL);
    if (!sattr.attr.name) {
    ret = -ENOMEM;
    goto out;
    }
    sattr.read = module_sect_read;
    sattr.private = (void *)sec.sh_addr;
    sattr.size = MODULE_SECT_READ_SIZE;
    sattr.attr.mode = 0400;
// (gattr++) = sattr++;
    }
    ret = sysfs_create_group(&mod.mkobj.kobj, &sect_attrs.grp);
    if (ret)
    goto out;
    mod.sect_attrs = sect_attrs;
    return 0;
    out:
    free_sect_attrs(sect_attrs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remove_sect_attrs(mod: *mut module) {
    static void remove_sect_attrs(struct module *mod)
    {
    if (mod.sect_attrs) {
    sysfs_remove_group(&mod.mkobj.kobj,
    &mod.sect_attrs.grp);
//
// We are positive that no one is using any sect attrs
// at this point.  Deallocate immediately.
//
    free_sect_attrs(mod.sect_attrs);
    mod.sect_attrs = core::ptr::null_mut();
    }
    }
//
// /sys/module/foo/notes/.section.name gives contents of SHT_NOTE sections.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_notes_attrs {
    pub grp: attribute_group,
    pub attrs: [bin_attribute; ],
}

#[no_mangle]
unsafe extern "C" fn free_notes_attrs(notes_attrs: *mut module_notes_attrs) {
    static void free_notes_attrs(struct module_notes_attrs *notes_attrs)
    {
    kfree(notes_attrs.grp.bin_attrs);
    kfree(notes_attrs);
    }
#[no_mangle]
unsafe extern "C" fn add_notes_attrs(mod: *mut module, info: *const load_info) -> c_int {
    static int add_notes_attrs(struct module *mod, const struct load_info *info)
    {
    unsigned int notes, loaded, i;
    struct module_notes_attrs *notes_attrs;
    const struct bin_attribute **gattr;
    struct bin_attribute *nattr;
    int ret;
// Count notes sections and allocate structures.
    notes = 0;
    for (i = 0; i < info.hdr.e_shnum; i++)
    if (!sect_empty(&info.sechdrs[i]) &&
    info.sechdrs[i].sh_type == SHT_NOTE)
    ++notes;
    if (notes == 0)
    return 0;
    notes_attrs = kzalloc_flex(*notes_attrs, attrs, notes);
    if (!notes_attrs)
    return -ENOMEM;
    gattr = kzalloc_objs(*gattr, notes + 1);
    if (!gattr) {
    kfree(notes_attrs);
    return -ENOMEM;
    }
    notes_attrs.grp.name = "notes";
    notes_attrs.grp.bin_attrs = gattr;
    nattr = &notes_attrs.attrs[0];
    for (loaded = i = 0; i < info.hdr.e_shnum; ++i) {
    if (sect_empty(&info.sechdrs[i]))
    continue;
    if (info.sechdrs[i].sh_type == SHT_NOTE) {
    sysfs_bin_attr_init(nattr);
    nattr.attr.name = mod.sect_attrs.attrs[loaded].attr.name;
    nattr.attr.mode = 0444;
    nattr.size = info.sechdrs[i].sh_size;
    nattr.private = (void *)info.sechdrs[i].sh_addr;
    nattr.read = sysfs_bin_attr_simple_read;
// (gattr++) = nattr++;
    }
    ++loaded;
    }
    ret = sysfs_create_group(&mod.mkobj.kobj, &notes_attrs.grp);
    if (ret)
    goto out;
    mod.notes_attrs = notes_attrs;
    return 0;
    out:
    free_notes_attrs(notes_attrs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remove_notes_attrs(mod: *mut module) {
    static void remove_notes_attrs(struct module *mod)
    {
    if (mod.notes_attrs) {
    sysfs_remove_group(&mod.mkobj.kobj,
    &mod.notes_attrs.grp);
//
// We are positive that no one is using any notes attrs
// at this point.  Deallocate immediately.
//
    free_notes_attrs(mod.notes_attrs);
    mod.notes_attrs = core::ptr::null_mut();
    }
    }

#[no_mangle]
pub unsafe extern "C" fn add_sect_attrs(mod: *mut module, info: *const load_info) -> c_int {
    static inline int add_sect_attrs(struct module *mod, const struct load_info *info)
    {
    return 0;
    }
    static inline void remove_sect_attrs(struct module *mod) { }
#[no_mangle]
pub unsafe extern "C" fn add_notes_attrs(mod: *mut module, info: *const load_info) -> c_int {
    static inline int add_notes_attrs(struct module *mod, const struct load_info *info)
    {
    return 0;
    }
    static inline void remove_notes_attrs(struct module *mod) { }

#[no_mangle]
unsafe extern "C" fn del_usage_links(mod: *mut module) {
    static void del_usage_links(struct module *mod)
    {

    struct module_use *use;
    mutex_lock(&module_mutex);
    list_for_each_entry(use, &mod.target_list, target_list)
    sysfs_remove_link(use.target.holders_dir, mod.name);
    mutex_unlock(&module_mutex);

    }
#[no_mangle]
unsafe extern "C" fn add_usage_links(mod: *mut module) -> c_int {
    static int add_usage_links(struct module *mod)
    {
    let mut ret: c_int = 0;

    struct module_use *use;
    mutex_lock(&module_mutex);
    list_for_each_entry(use, &mod.target_list, target_list) {
    ret = sysfs_create_link(use.target.holders_dir,
    &mod.mkobj.kobj, mod.name);
    if (ret)
    break;
    }
    mutex_unlock(&module_mutex);
    if (ret)
    del_usage_links(mod);

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn module_remove_modinfo_attrs(mod: *mut module, end: c_int) {
    static void module_remove_modinfo_attrs(struct module *mod, int end)
    {
    const struct module_attribute *attr;
    int i;
    for (i = 0; (attr = &mod.modinfo_attrs[i]); i++) {
    if (end >= 0 && i > end)
    break;
// pick a field to test for end of list
    if (!attr.attr.name)
    break;
    sysfs_remove_file(&mod.mkobj.kobj, &attr.attr);
    if (attr.free)
    attr.free(mod);
    }
    kfree(mod.modinfo_attrs);
    }
#[no_mangle]
unsafe extern "C" fn module_add_modinfo_attrs(mod: *mut module) -> c_int {
    static int module_add_modinfo_attrs(struct module *mod)
    {
    const struct module_attribute *attr;
    struct module_attribute *temp_attr;
    let mut error: c_int = 0;
    int i;
    mod.modinfo_attrs = kzalloc((sizeof(struct module_attribute) *
    (modinfo_attrs_count + 1)),
    GFP_KERNEL);
    if (!mod.modinfo_attrs)
    return -ENOMEM;
    temp_attr = mod.modinfo_attrs;
    for (i = 0; (attr = modinfo_attrs[i]); i++) {
    if (!attr.test || attr.test(mod)) {
    memcpy(temp_attr, attr, sizeof(*temp_attr));
    sysfs_attr_init(&temp_attr.attr);
    error = sysfs_create_file(&mod.mkobj.kobj,
    &temp_attr.attr);
    if (error)
    goto error_out;
    ++temp_attr;
    }
    }
    return 0;
    error_out:
    if (i > 0)
    module_remove_modinfo_attrs(mod, --i);
    else
    kfree(mod.modinfo_attrs);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn mod_kobject_put(mod: *mut module) {
    static void mod_kobject_put(struct module *mod)
    {
    DECLARE_COMPLETION_ONSTACK(c);
    mod.mkobj.kobj_completion = &c;
    kobject_put(&mod.mkobj.kobj);
    wait_for_completion(&c);
    }
#[no_mangle]
unsafe extern "C" fn mod_sysfs_init(mod: *mut module) -> c_int {
    static int mod_sysfs_init(struct module *mod)
    {
    int err;
    struct kobject *kobj;
    if (!module_kset) {
    pr_err("%s: module sysfs not initialized\n", mod.name);
    err = -EINVAL;
    goto out;
    }
    kobj = kset_find_obj(module_kset, mod.name);
    if (kobj) {
    pr_err("%s: module is already loaded\n", mod.name);
    kobject_put(kobj);
    err = -EINVAL;
    goto out;
    }
    mod.mkobj.mod = mod;
    memset(&mod.mkobj.kobj, 0, sizeof(mod.mkobj.kobj));
    mod.mkobj.kobj.kset = module_kset;
    err = kobject_init_and_add(&mod.mkobj.kobj, &module_ktype, core::ptr::null_mut(),
    "%s", mod.name);
    if (err)
    mod_kobject_put(mod);
    out:
    return err;
    }
    int mod_sysfs_setup(struct module *mod,
    const struct load_info *info,
    struct kernel_param *kparam,
    unsigned int num_params)
    {
    int err;
    err = mod_sysfs_init(mod);
    if (err)
    goto out;
    mod.holders_dir = kobject_create_and_add("holders", &mod.mkobj.kobj);
    if (!mod.holders_dir) {
    err = -ENOMEM;
    goto out_unreg;
    }
    err = module_param_sysfs_setup(mod, kparam, num_params);
    if (err)
    goto out_unreg_holders;
    err = module_add_modinfo_attrs(mod);
    if (err)
    goto out_unreg_param;
    err = add_usage_links(mod);
    if (err)
    goto out_unreg_modinfo_attrs;
    err = add_sect_attrs(mod, info);
    if (err)
    goto out_del_usage_links;
    err = add_notes_attrs(mod, info);
    if (err)
    goto out_unreg_sect_attrs;
    return 0;
    out_unreg_sect_attrs:
    remove_sect_attrs(mod);
    out_del_usage_links:
    del_usage_links(mod);
    out_unreg_modinfo_attrs:
    module_remove_modinfo_attrs(mod, -1);
    out_unreg_param:
    module_param_sysfs_remove(mod);
    out_unreg_holders:
    kobject_put(mod.holders_dir);
    out_unreg:
    mod_kobject_put(mod);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mod_sysfs_fini(mod: *mut module) {
    static void mod_sysfs_fini(struct module *mod)
    {
    remove_notes_attrs(mod);
    remove_sect_attrs(mod);
    mod_kobject_put(mod);
    }
#[no_mangle]
pub unsafe extern "C" fn mod_sysfs_teardown(mod: *mut module) {
    void mod_sysfs_teardown(struct module *mod)
    {
    del_usage_links(mod);
    module_remove_modinfo_attrs(mod, -1);
    module_param_sysfs_remove(mod);
    kobject_put(mod.mkobj.drivers_dir);
    kobject_put(mod.holders_dir);
    mod_sysfs_fini(mod);
    }
#[no_mangle]
pub unsafe extern "C" fn init_param_lock(mod: *mut module) {
    void init_param_lock(struct module *mod)
    {
    mutex_init(&mod.param_lock);
    }
