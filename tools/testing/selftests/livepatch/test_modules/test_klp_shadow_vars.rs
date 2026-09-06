//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/livepatch/test_modules/test_klp_shadow_vars.c
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
// Copyright (C) 2018 Joe Lawrence <joe.lawrence@redhat.com>

//
// Keep a small list of pointers so that we can print address-agnostic
// pointer values.  Use a rolling integer count to differentiate the values.
// Ironically we could have used the shadow variable API to do this, but
// let's not lean too heavily on the very code we're testing.
//
    static LIST_HEAD(ptr_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_ptr {
    pub ptr: *mut c_void,
    pub id: c_int,
    pub list: list_head,
}

#[no_mangle]
unsafe extern "C" fn free_ptr_list() {
    static void free_ptr_list(void)
    {
    struct shadow_ptr *sp, *tmp_sp;
    list_for_each_entry_safe(sp, tmp_sp, &ptr_list, list) {
    list_del(&sp.list);
    kfree(sp);
    }
    }
#[no_mangle]
unsafe extern "C" fn ptr_id(ptr: *mut c_void) -> c_int {
    static int ptr_id(void *ptr)
    {
    struct shadow_ptr *sp;
    static int count;
    list_for_each_entry(sp, &ptr_list, list) {
    if (sp.ptr == ptr)
    return sp.id;
    }
    sp = kmalloc(sizeof(*sp), GFP_ATOMIC);
    if (!sp)
    return -ENOMEM;
    sp.ptr = ptr;
    sp.id = count++;
    list_add(&sp.list, &ptr_list);
    return sp.id;
    }
//
// Shadow variable wrapper functions that echo the function and arguments
// to the kernel log for testing verification.  Don't display raw pointers,
// but use the ptr_id() value instead.
//
    static void *shadow_get(void *obj, unsigned long id)
    {
    int **sv;
    sv = klp_shadow_get(obj, id);
    pr_info("klp_%s(obj=PTR%d, id=0x%lx) = PTR%d\n",
    __func__, ptr_id(obj), id, ptr_id(sv));
    return sv;
    }
    static void *shadow_alloc(void *obj, unsigned long id, size_t size,
    gfp_t gfp_flags, klp_shadow_ctor_t ctor,
    void *ctor_data)
    {
    int **var = ctor_data;
    int **sv;
    sv = klp_shadow_alloc(obj, id, size, gfp_flags, ctor, var);
    pr_info("klp_%s(obj=PTR%d, id=0x%lx, size=%zx, gfp_flags=%pGg), ctor=PTR%d, ctor_data=PTR%d = PTR%d\n",
    __func__, ptr_id(obj), id, size, &gfp_flags, ptr_id(ctor),
    ptr_id(*var), ptr_id(sv));
    return sv;
    }
    static void *shadow_get_or_alloc(void *obj, unsigned long id, size_t size,
    gfp_t gfp_flags, klp_shadow_ctor_t ctor,
    void *ctor_data)
    {
    int **var = ctor_data;
    int **sv;
    sv = klp_shadow_get_or_alloc(obj, id, size, gfp_flags, ctor, var);
    pr_info("klp_%s(obj=PTR%d, id=0x%lx, size=%zx, gfp_flags=%pGg), ctor=PTR%d, ctor_data=PTR%d = PTR%d\n",
    __func__, ptr_id(obj), id, size, &gfp_flags, ptr_id(ctor),
    ptr_id(*var), ptr_id(sv));
    return sv;
    }
#[no_mangle]
unsafe extern "C" fn shadow_free(obj: *mut c_void, id: c_ulong, dtor: klp_shadow_dtor_t) {
    static void shadow_free(void *obj, unsigned long id, klp_shadow_dtor_t dtor)
    {
    klp_shadow_free(obj, id, dtor);
    pr_info("klp_%s(obj=PTR%d, id=0x%lx, dtor=PTR%d)\n",
    __func__, ptr_id(obj), id, ptr_id(dtor));
    }
#[no_mangle]
unsafe extern "C" fn shadow_free_all(id: c_ulong, dtor: klp_shadow_dtor_t) {
    static void shadow_free_all(unsigned long id, klp_shadow_dtor_t dtor)
    {
    klp_shadow_free_all(id, dtor);
    pr_info("klp_%s(id=0x%lx, dtor=PTR%d)\n", __func__, id, ptr_id(dtor));
    }
// Shadow variable constructor - remember simple pointer data
#[no_mangle]
unsafe extern "C" fn shadow_ctor(obj: *mut c_void, shadow_data: *mut c_void, ctor_data: *mut c_void) -> c_int {
    static int shadow_ctor(void *obj, void *shadow_data, void *ctor_data)
    {
    int **sv = shadow_data;
    int **var = ctor_data;
    if (!var)
    return -EINVAL;
// sv = *var;
    pr_info("%s: PTR%d . PTR%d\n", __func__, ptr_id(sv), ptr_id(*var));
    return 0;
    }
//
// With more than one item to free in the list, order is not determined and
// shadow_dtor will not be passed to shadow_free_all() which would make the
// test fail. (see pass 6)
//
#[no_mangle]
unsafe extern "C" fn shadow_dtor(obj: *mut c_void, shadow_data: *mut c_void) {
    static void shadow_dtor(void *obj, void *shadow_data)
    {
    int **sv = shadow_data;
    pr_info("%s(obj=PTR%d, shadow_data=PTR%d)\n",
    __func__, ptr_id(obj), ptr_id(sv));
    }
// number of objects we simulate that need shadow vars
pub const NUM_OBJS: c_int = 3;
// dynamically created obj fields have the following shadow var id values
pub const SV_ID1: c_uint = 0x1234;
pub const SV_ID2: c_uint = 0x1235;
//
// The main test case adds/removes new fields (shadow var) to each of these
// test structure instances. The last group of fields in the struct represent
// the idea that shadow variables may be added and removed to and from the
// struct during execution.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_object {
// add anything here below and avoid to define an empty struct
    pub sp: shadow_ptr,
// these represent shadow vars added and removed with SV_ID{1,2}
// char nfield1;
// int  nfield2;
}

#[no_mangle]
unsafe extern "C" fn test_klp_shadow_vars_init() -> c_int {
    static int test_klp_shadow_vars_init(void)
    {
    struct test_object objs[NUM_OBJS];
    char nfields1[NUM_OBJS], *pnfields1[NUM_OBJS], **sv1[NUM_OBJS];
    char *pndup[NUM_OBJS];
    int nfields2[NUM_OBJS], *pnfields2[NUM_OBJS], **sv2[NUM_OBJS];
    void **sv;
    int ret;
    int i;
    ptr_id(core::ptr::null_mut());
//
// With an empty shadow variable hash table, expect not to find
// any matches.
//
    sv = shadow_get(&objs[0], SV_ID1);
    if (!sv)
    pr_info("  got expected core::ptr::null_mut() result\n");
// pass 1: init & alloc a char+int pair of svars for each objs
    for (i = 0; i < NUM_OBJS; i++) {
    pnfields1[i] = &nfields1[i];
    ptr_id(pnfields1[i]);
    if (i % 2) {
    sv1[i] = shadow_alloc(&objs[i], SV_ID1,
    sizeof(pnfields1[i]), GFP_KERNEL,
    shadow_ctor, &pnfields1[i]);
    } else {
    sv1[i] = shadow_get_or_alloc(&objs[i], SV_ID1,
    sizeof(pnfields1[i]), GFP_KERNEL,
    shadow_ctor, &pnfields1[i]);
    }
    if (!sv1[i]) {
    ret = -ENOMEM;
    goto out;
    }
    pnfields2[i] = &nfields2[i];
    ptr_id(pnfields2[i]);
    sv2[i] = shadow_alloc(&objs[i], SV_ID2, sizeof(pnfields2[i]),
    GFP_KERNEL, shadow_ctor, &pnfields2[i]);
    if (!sv2[i]) {
    ret = -ENOMEM;
    goto out;
    }
    }
// pass 2: verify we find allocated svars and where they point to
    for (i = 0; i < NUM_OBJS; i++) {
// check the "char" svar for all objects
    sv = shadow_get(&objs[i], SV_ID1);
    if (!sv) {
    ret = -EINVAL;
    goto out;
    }
    if ((char **)sv == sv1[i] && *sv1[i] == pnfields1[i])
    pr_info("  got expected PTR%d . PTR%d result\n",
    ptr_id(sv1[i]), ptr_id(*sv1[i]));
// check the "int" svar for all objects
    sv = shadow_get(&objs[i], SV_ID2);
    if (!sv) {
    ret = -EINVAL;
    goto out;
    }
    if ((int **)sv == sv2[i] && *sv2[i] == pnfields2[i])
    pr_info("  got expected PTR%d . PTR%d result\n",
    ptr_id(sv2[i]), ptr_id(*sv2[i]));
    }
// pass 3: verify that 'get_or_alloc' returns already allocated svars
    for (i = 0; i < NUM_OBJS; i++) {
    pndup[i] = &nfields1[i];
    ptr_id(pndup[i]);
    sv = shadow_get_or_alloc(&objs[i], SV_ID1, sizeof(pndup[i]),
    GFP_KERNEL, shadow_ctor, &pndup[i]);
    if (!sv) {
    ret = -EINVAL;
    goto out;
    }
    if ((char **)sv == sv1[i] && *sv1[i] == pnfields1[i])
    pr_info("  got expected PTR%d . PTR%d result\n",
    ptr_id(sv1[i]), ptr_id(*sv1[i]));
    }
// pass 4: free <objs[*], SV_ID1> pairs of svars, verify removal
    for (i = 0; i < NUM_OBJS; i++) {
    shadow_free(&objs[i], SV_ID1, shadow_dtor); /* 'char' pairs */
    sv = shadow_get(&objs[i], SV_ID1);
    if (!sv)
    pr_info("  got expected core::ptr::null_mut() result\n");
    }
// pass 5: check we still find <objs[*], SV_ID2> svar pairs
    for (i = 0; i < NUM_OBJS; i++) {
    sv = shadow_get(&objs[i], SV_ID2);	/* 'int' pairs */
    if (!sv) {
    ret = -EINVAL;
    goto out;
    }
    if ((int **)sv == sv2[i] && *sv2[i] == pnfields2[i])
    pr_info("  got expected PTR%d . PTR%d result\n",
    ptr_id(sv2[i]), ptr_id(*sv2[i]));
    }
// pass 6: free all the <objs[*], SV_ID2> svar pairs too.
    shadow_free_all(SV_ID2, core::ptr::null_mut());		/* 'int' pairs */
    for (i = 0; i < NUM_OBJS; i++) {
    sv = shadow_get(&objs[i], SV_ID2);
    if (!sv)
    pr_info("  got expected core::ptr::null_mut() result\n");
    }
    free_ptr_list();
    return 0;
    out:
    shadow_free_all(SV_ID1, core::ptr::null_mut());		/* 'char' pairs */
    shadow_free_all(SV_ID2, core::ptr::null_mut());		/* 'int' pairs */
    free_ptr_list();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_klp_shadow_vars_exit() {
    static void test_klp_shadow_vars_exit(void)
    {
    }
    module_init(test_klp_shadow_vars_init);
    module_exit(test_klp_shadow_vars_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joe Lawrence <joe.lawrence@redhat.com>");
    MODULE_DESCRIPTION("Livepatch test: shadow variables");
