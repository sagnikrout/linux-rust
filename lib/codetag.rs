//! Automatically rewritten from C to Rust
//! Source: lib/codetag.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag_type {
    pub link: list_head,
    pub count: c_uint,
    pub mod_idr: idr,
//
// protects mod_idr, next_mod_seq,
// iter->mod_seq and cmod->mod_seq
//
    pub mod_lock: rw_semaphore,
    pub desc: codetag_type_desc,
// generates unique sequence number for module load
    pub next_mod_seq: c_ulong,
// bumped on every module load and unload
    pub content_id: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag_range {
    pub start: *mut codetag,
    pub stop: *mut codetag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag_module {
    pub mod: *mut module,
    pub range: codetag_range,
    pub mod_seq: c_ulong,
}

    static DEFINE_MUTEX(codetag_lock);
    static LIST_HEAD(codetag_types);
#[no_mangle]
pub unsafe extern "C" fn codetag_lock_module_list(cttype: *mut codetag_type) {
    void codetag_lock_module_list(struct codetag_type *cttype)
    {
    down_read(&cttype.mod_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_trylock_module_list(cttype: *mut codetag_type) -> bool {
    bool codetag_trylock_module_list(struct codetag_type *cttype)
    {
    return down_read_trylock(&cttype.mod_lock) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_unlock_module_list(cttype: *mut codetag_type) {
    void codetag_unlock_module_list(struct codetag_type *cttype)
    {
    up_read(&cttype.mod_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_get_content_id(cttype: *mut codetag_type) -> c_ulong {
    unsigned long codetag_get_content_id(struct codetag_type *cttype)
    {
    lockdep_assert_held(&cttype.mod_lock);
    return cttype.content_id;
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_get_count(cttype: *mut codetag_type) -> c_uint {
    unsigned int codetag_get_count(struct codetag_type *cttype)
    {
    lockdep_assert_held(&cttype.mod_lock);
    return cttype.count;
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_get_ct_iter(cttype: *mut codetag_type) -> codetag_iterator {
    struct codetag_iterator codetag_get_ct_iter(struct codetag_type *cttype)
    {
    struct codetag_iterator iter = {
    .cttype = cttype,
    .cmod = core::ptr::null_mut(),
    .mod_id = 0,
    .ct = core::ptr::null_mut(),
    .mod_seq = 0,
    };
    return iter;
    }
    static inline struct codetag *get_first_module_ct(struct codetag_module *cmod)
    {
    return cmod.range.start < cmod.range.stop ? cmod.range.start : core::ptr::null_mut();
    }
    static inline
    struct codetag *get_next_module_ct(struct codetag_iterator *iter)
    {
    struct codetag *res = (struct codetag *)
    ((char *)iter.ct + iter.cttype.desc.tag_size);
    return res < iter.cmod.range.stop ? res : core::ptr::null_mut();
    }
    struct codetag *codetag_next_ct(struct codetag_iterator *iter)
    {
    struct codetag_type *cttype = iter.cttype;
    struct codetag_module *cmod;
    struct codetag *ct;
    lockdep_assert_held(&cttype.mod_lock);
    if (unlikely(idr_is_empty(&cttype.mod_idr)))
    return core::ptr::null_mut();
    ct = core::ptr::null_mut();
    while (true) {
    cmod = idr_find(&cttype.mod_idr, iter.mod_id);
// If module was removed move to the next one
    if (!cmod)
    cmod = idr_get_next_ul(&cttype.mod_idr,
    &iter.mod_id);
// Exit if no more modules
    if (!cmod)
    break;
    if (!iter.cmod || iter.mod_seq != cmod.mod_seq) {
    iter.cmod = cmod;
    iter.mod_seq = cmod.mod_seq;
    ct = get_first_module_ct(cmod);
    } else {
    ct = get_next_module_ct(iter);
    }
    if (ct)
    break;
    iter.mod_id++;
    }
    iter.ct = ct;
    return ct;
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_to_text(out: *mut seq_buf, ct: *mut codetag) {
    void codetag_to_text(struct seq_buf *out, struct codetag *ct)
    {
    if (ct.modname)
    seq_buf_printf(out, "%s:%u [%s] func:%s",
    ct.filename, ct.lineno,
    ct.modname, ct.function);
    else
    seq_buf_printf(out, "%s:%u func:%s",
    ct.filename, ct.lineno, ct.function);
    }
    static inline size_t range_size(const struct codetag_type *cttype,
    const struct codetag_range *range)
    {
    return ((char *)range.stop - (char *)range.start) /
    cttype.desc.tag_size;
    }
    static void *get_symbol(struct module *mod, const char *prefix, const char *name)
    {
    DECLARE_SEQ_BUF(sb, KSYM_NAME_LEN);
    const char *buf;
    void *ret;
    seq_buf_printf(&sb, "%s%s", prefix, name);
    if (seq_buf_has_overflowed(&sb))
    return core::ptr::null_mut();
    buf = seq_buf_str(&sb);
    preempt_disable();
    ret = mod ?
    (void *)find_kallsyms_symbol_value(mod, buf) :
    (void *)kallsyms_lookup_name(buf);
    preempt_enable();
    return ret;
    }
    static struct codetag_range get_section_range(struct module *mod,
    const char *section)
    {
    return (struct codetag_range) {
    get_symbol(mod, CODETAG_SECTION_START_PREFIX, section),
    get_symbol(mod, CODETAG_SECTION_STOP_PREFIX, section),
    };
    }
    static const char *get_mod_name(__maybe_unused struct module *mod)
    {

    if (mod)
    return mod.name;

    return "(built-in)";
    }
#[no_mangle]
unsafe extern "C" fn codetag_module_init(cttype: *mut codetag_type, mod: *mut module) -> c_int {
    static int codetag_module_init(struct codetag_type *cttype, struct module *mod)
    {
    struct codetag_range range;
    struct codetag_module *cmod;
    int mod_id;
    int err;
    range = get_section_range(mod, cttype.desc.section);
    if (!range.start || !range.stop) {
    pr_warn("Failed to load code tags of type %s from the module %s\n",
    cttype.desc.section, get_mod_name(mod));
    return -EINVAL;
    }
// Ignore empty ranges
    if (range.start == range.stop)
    return 0;
    BUG_ON(range.start > range.stop);
    cmod = kmalloc_obj(*cmod);
    if (unlikely(!cmod))
    return -ENOMEM;
    cmod.mod = mod;
    cmod.range = range;
    down_write(&cttype.mod_lock);
    cmod.mod_seq = ++cttype.next_mod_seq;
    ++cttype.content_id;
    mod_id = idr_alloc(&cttype.mod_idr, cmod, 0, 0, GFP_KERNEL);
    if (mod_id >= 0) {
    if (cttype.desc.module_load) {
    err = cttype.desc.module_load(mod, range.start, range.stop);
    if (!err)
    cttype.count += range_size(cttype, &range);
    else
    idr_remove(&cttype.mod_idr, mod_id);
    } else {
    cttype.count += range_size(cttype, &range);
    err = 0;
    }
    } else {
    err = mod_id;
    }
    up_write(&cttype.mod_lock);
    if (err < 0) {
    kfree(cmod);
    return err;
    }
    return 0;
    }

// Some codetag types need a separate module section
    bool codetag_needs_module_section(struct module *mod, const char *name,
    unsigned long size)
    {
    const char *type_name;
    struct codetag_type *cttype;
    let mut ret: bool = false;
    if (strncmp(name, CODETAG_SECTION_PREFIX, strlen(CODETAG_SECTION_PREFIX)))
    return false;
    type_name = name + strlen(CODETAG_SECTION_PREFIX);
    mutex_lock(&codetag_lock);
    list_for_each_entry(cttype, &codetag_types, link) {
    if (strcmp(type_name, cttype.desc.section) == 0) {
    if (!cttype.desc.needs_section_mem)
    break;
    down_write(&cttype.mod_lock);
    ret = cttype.desc.needs_section_mem(mod, size);
    up_write(&cttype.mod_lock);
    break;
    }
    }
    mutex_unlock(&codetag_lock);
    return ret;
    }
    void *codetag_alloc_module_section(struct module *mod, const char *name,
    unsigned long size, unsigned int prepend,
    unsigned long align)
    {
    const char *type_name = name + strlen(CODETAG_SECTION_PREFIX);
    struct codetag_type *cttype;
    void *ret = ERR_PTR(-EINVAL);
    mutex_lock(&codetag_lock);
    list_for_each_entry(cttype, &codetag_types, link) {
    if (strcmp(type_name, cttype.desc.section) == 0) {
    if (WARN_ON(!cttype.desc.alloc_section_mem))
    break;
    down_write(&cttype.mod_lock);
    ret = cttype.desc.alloc_section_mem(mod, size, prepend, align);
    up_write(&cttype.mod_lock);
    break;
    }
    }
    mutex_unlock(&codetag_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_free_module_sections(mod: *mut module) {
    void codetag_free_module_sections(struct module *mod)
    {
    struct codetag_type *cttype;
    mutex_lock(&codetag_lock);
    list_for_each_entry(cttype, &codetag_types, link) {
    if (!cttype.desc.free_section_mem)
    continue;
    down_write(&cttype.mod_lock);
    cttype.desc.free_section_mem(mod, false);
    up_write(&cttype.mod_lock);
    }
    mutex_unlock(&codetag_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_module_replaced(mod: *mut module, new_mod: *mut module) {
    void codetag_module_replaced(struct module *mod, struct module *new_mod)
    {
    struct codetag_type *cttype;
    mutex_lock(&codetag_lock);
    list_for_each_entry(cttype, &codetag_types, link) {
    if (!cttype.desc.module_replaced)
    continue;
    down_write(&cttype.mod_lock);
    cttype.desc.module_replaced(mod, new_mod);
    up_write(&cttype.mod_lock);
    }
    mutex_unlock(&codetag_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_load_module(mod: *mut module) -> c_int {
    int codetag_load_module(struct module *mod)
    {
    struct codetag_type *cttype;
    let mut ret: c_int = 0;
    if (!mod)
    return 0;
    mutex_lock(&codetag_lock);
    list_for_each_entry(cttype, &codetag_types, link) {
    ret = codetag_module_init(cttype, mod);
    if (ret)
    break;
    }
    mutex_unlock(&codetag_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn codetag_unload_module(mod: *mut module) {
    void codetag_unload_module(struct module *mod)
    {
    struct codetag_type *cttype;
    if (!mod)
    return;
// await any module's kfree_rcu() operations to complete
    kvfree_rcu_barrier();
    mutex_lock(&codetag_lock);
    list_for_each_entry(cttype, &codetag_types, link) {
    struct codetag_module *found = core::ptr::null_mut();
    struct codetag_module *cmod;
    unsigned long mod_id, tmp;
    down_write(&cttype.mod_lock);
    idr_for_each_entry_ul(&cttype.mod_idr, cmod, tmp, mod_id) {
    if (cmod.mod && cmod.mod == mod) {
    found = cmod;
    break;
    }
    }
    if (found) {
    if (cttype.desc.module_unload)
    cttype.desc.module_unload(cmod.mod,
    cmod.range.start, cmod.range.stop);
    cttype.count -= range_size(cttype, &cmod.range);
    idr_remove(&cttype.mod_idr, mod_id);
    kfree(cmod);
    ++cttype.content_id;
    }
    up_write(&cttype.mod_lock);
    if (found && cttype.desc.free_section_mem)
    cttype.desc.free_section_mem(mod, true);
    }
    mutex_unlock(&codetag_lock);
    }

    struct codetag_type *
    codetag_register_type(const struct codetag_type_desc *desc)
    {
    struct codetag_type *cttype;
    int err;
    BUG_ON(desc.tag_size <= 0);
    cttype = kzalloc_obj(*cttype);
    if (unlikely(!cttype))
    return ERR_PTR(-ENOMEM);
    cttype.desc = *desc;
    idr_init(&cttype.mod_idr);
    init_rwsem(&cttype.mod_lock);
    err = codetag_module_init(cttype, core::ptr::null_mut());
    if (unlikely(err)) {
    kfree(cttype);
    return ERR_PTR(err);
    }
    mutex_lock(&codetag_lock);
    list_add_tail(&cttype.link, &codetag_types);
    mutex_unlock(&codetag_lock);
    return cttype;
    }
