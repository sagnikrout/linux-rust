//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/mkregtable.c
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


// SPDX-License-Identifier: MIT
// utility to create the register check tables
// this includes inlined list.h safe for userspace.
//
// Copyright 2009 Jerome Glisse
// Copyright 2009 Red Hat Inc.
//
// Authors:
// Jerome Glisse
// Dave Airlie
//

//
// container_of - cast a member of a structure out to the containing structure
// @ptr:    the pointer to the member.
// @type:   the type of the container struct this is embedded in.
// @member: the name of the member within the struct.
//

    const typeof(((type *)0).member)*__mptr = (ptr);    \
    (type *)((char *)__mptr - offsetof(type, member)); })
//
// Simple doubly linked list implementation.
//
// Some of the internal functions ("__xxx") are useful when
// manipulating whole lists rather than single entries, as
// sometimes we already know the next/prev entries and we can
// generate better code by using them directly rather than
// using the generic single-entry routines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub prev: *mut *mut list_head next,,
}

#[no_mangle]
pub unsafe extern "C" fn INIT_LIST_HEAD(list: *mut list_head) {
    static inline void INIT_LIST_HEAD(struct list_head *list)
    {
    list.next = list;
    list.prev = list;
    }
//
// Insert a new entry between two known consecutive entries.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//

    static inline void __list_add(struct list_head *new,
    struct list_head *prev, struct list_head *next)
    {
    next.prev = new;
    new.next = next;
    new.prev = prev;
    prev.next = new;
    }

    extern void __list_add(struct list_head *new,
    struct list_head *prev, struct list_head *next);

//
// list_add_tail - add a new entry
// @new: new entry to be added
// @head: list head to add it before
//
// Insert a new entry before the specified head.
// This is useful for implementing queues.
//
#[no_mangle]
pub unsafe extern "C" fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    static inline void list_add_tail(struct list_head *new, struct list_head *head)
    {
    __list_add(new, head.prev, head);
    }
//
// list_entry - get the struct for this entry
// @ptr:	the &struct list_head pointer.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//

    container_of(ptr, type, member)
//
// list_for_each_entry	-	iterate over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

    for (pos = list_entry((head).next, typeof(*pos), member);	\
    &pos.member != (head); 	\
    pos = list_entry(pos.member.next, typeof(*pos), member))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct offset {
    pub list: list_head,
    pub offset: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct table {
    pub offsets: list_head,
    pub offset_max: unsigned,
    pub nentry: unsigned,
    pub table: *mut unsigned,
    pub gpu_prefix: *mut c_char,
}

    static struct offset *offset_new(unsigned o)
    {
    struct offset *offset;
    offset = (struct offset *)malloc(sizeof(struct offset));
    if (offset) {
    INIT_LIST_HEAD(&offset.list);
    offset.offset = o;
    }
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn table_offset_add(t: *mut table, offset: *mut offset) {
    static void table_offset_add(struct table *t, struct offset *offset)
    {
    list_add_tail(&offset.list, &t.offsets);
    }
#[no_mangle]
unsafe extern "C" fn table_init(t: *mut table) {
    static void table_init(struct table *t)
    {
    INIT_LIST_HEAD(&t.offsets);
    t.offset_max = 0;
    t.nentry = 0;
    t.table = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn table_print(t: *mut table) {
    static void table_print(struct table *t)
    {
    unsigned nlloop, i, j, n, c, id;
    nlloop = (t.nentry + 3) / 4;
    c = t.nentry;
    printf("static const unsigned %s_reg_safe_bm[%d] = {\n", t.gpu_prefix,
    t.nentry);
    for (i = 0, id = 0; i < nlloop; i++) {
    n = 4;
    if (n > c)
    n = c;
    c -= n;
    for (j = 0; j < n; j++) {
    if (j == 0)
    printf("\t");
    else
    printf(" ");
    printf("0x%08X,", t.table[id++]);
    }
    printf("\n");
    }
    printf("};\n");
    }
#[no_mangle]
unsafe extern "C" fn table_build(t: *mut table) -> c_int {
    static int table_build(struct table *t)
    {
    struct offset *offset;
    unsigned i, m;
    t.nentry = ((t.offset_max >> 2) + 31) / 32;
    t.table = (unsigned *)malloc(sizeof(unsigned) * t.nentry);
    if (t.table == core::ptr::null_mut())
    return -1;
    memset(t.table, 0xff, sizeof(unsigned) * t.nentry);
    list_for_each_entry(offset, &t.offsets, list) {
    i = (offset.offset >> 2) / 32;
    m = (offset.offset >> 2) & 31;
    m = 1 << m;
    t.table[i] ^= m;
    }
    return 0;
    }
    static char gpu_name[10];
#[no_mangle]
unsafe extern "C" fn parser_auth(t: *mut table, filename: *const c_char) -> c_int {
    static int parser_auth(struct table *t, const char *filename)
    {
    FILE *file;
    regex_t mask_rex;
    regmatch_t match[4];
    char buf[1024];
    size_t end;
    int len;
    let mut done: c_int = 0;
    int r;
    unsigned o;
    struct offset *offset;
    char last_reg_s[10];
    int last_reg;
    if (regcomp
    (&mask_rex, "(0x[0-9a-fA-F]*) *([_a-zA-Z0-9]*)", REG_EXTENDED)) {
    fprintf(stderr, "Failed to compile regular expression\n");
    return -1;
    }
    file = fopen(filename, "r");
    if (file == core::ptr::null_mut()) {
    fprintf(stderr, "Failed to open: %s\n", filename);
    return -1;
    }
    fseek(file, 0, SEEK_END);
    end = ftell(file);
    fseek(file, 0, SEEK_SET);
// get header
    if (fgets(buf, 1024, file) == core::ptr::null_mut()) {
    fclose(file);
    return -1;
    }
// first line will contain the last register
// and gpu name
    sscanf(buf, "%9s %9s", gpu_name, last_reg_s);
    t.gpu_prefix = gpu_name;
    last_reg = strtol(last_reg_s, core::ptr::null_mut(), 16);
    do {
    if (fgets(buf, 1024, file) == core::ptr::null_mut()) {
    fclose(file);
    return -1;
    }
    len = strlen(buf);
    if (ftell(file) == end)
    done = 1;
    if (len) {
    r = regexec(&mask_rex, buf, 4, match, 0);
    if (r == REG_NOMATCH) {
    } else if (r) {
    fprintf(stderr,
    "Error matching regular expression %d in %s\n",
    r, filename);
    fclose(file);
    return -1;
    } else {
    buf[match[0].rm_eo] = 0;
    buf[match[1].rm_eo] = 0;
    buf[match[2].rm_eo] = 0;
    o = strtol(&buf[match[1].rm_so], core::ptr::null_mut(), 16);
    offset = offset_new(o);
    table_offset_add(t, offset);
    if (o > t.offset_max)
    t.offset_max = o;
    }
    }
    } while (!done);
    fclose(file);
    if (t.offset_max < last_reg)
    t.offset_max = last_reg;
    return table_build(t);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct table t;
    if (argc != 2) {
    fprintf(stderr, "Usage: %s <authfile>\n", argv[0]);
    exit(1);
    }
    table_init(&t);
    if (parser_auth(&t, argv[1])) {
    fprintf(stderr, "Failed to parse file %s\n", argv[1]);
    return -1;
    }
    table_print(&t);
    return 0;
    }
