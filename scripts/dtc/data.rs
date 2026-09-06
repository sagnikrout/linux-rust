//! Automatically rewritten from C to Rust
//! Source: scripts/dtc/data.c
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
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
//

#[no_mangle]
pub unsafe extern "C" fn data_free(d: data) {
    void data_free(struct data d)
    {
    struct marker *m, *nm;
    m = d.markers;
    while (m) {
    nm = m.next;
    free(m.ref);
    free(m);
    m = nm;
    }
    if (d.val)
    free(d.val);
    }
#[no_mangle]
pub unsafe extern "C" fn data_grow_for(d: data, xlen: c_uint) -> data {
    struct data data_grow_for(struct data d, unsigned int xlen)
    {
    struct data nd;
    unsigned int newsize;
    if (xlen == 0)
    return d;
    nd = d;
    newsize = xlen;
    while ((d.len + xlen) > newsize)
    newsize *= 2;
    nd.val = xrealloc(d.val, newsize);
    return nd;
    }
#[no_mangle]
pub unsafe extern "C" fn data_copy_mem(mem: *const c_char, len: c_int) -> data {
    struct data data_copy_mem(const char *mem, int len)
    {
    struct data d;
    d = data_grow_for(empty_data, len);
    d.len = len;
    memcpy(d.val, mem, len);
    return d;
    }
#[no_mangle]
pub unsafe extern "C" fn data_copy_escape_string(s: *const c_char, len: c_int) -> data {
    struct data data_copy_escape_string(const char *s, int len)
    {
    let mut i: c_int = 0;
    struct data d;
    char *q;
    d = data_add_marker(empty_data, TYPE_STRING, core::ptr::null_mut());
    d = data_grow_for(d, len + 1);
    q = d.val;
    while (i < len) {
    let mut c: c_char = s[i++];
    if (c == '\\')
    c = get_escape_char(s, &i);
    q[d.len++] = c;
    }
    q[d.len++] = '\0';
    return d;
    }
#[no_mangle]
pub unsafe extern "C" fn data_copy_file(f: *mut FILE, maxlen: usize) -> data {
    struct data data_copy_file(FILE *f, size_t maxlen)
    {
    let mut d: data = empty_data;
    d = data_add_marker(d, TYPE_NONE, core::ptr::null_mut());
    while (!feof(f) && (d.len < maxlen)) {
    size_t chunksize, ret;
    if (maxlen == (size_t)-1)
    chunksize = 4096;
    else
    chunksize = maxlen - d.len;
    d = data_grow_for(d, chunksize);
    ret = fread(d.val + d.len, 1, chunksize, f);
    if (ferror(f))
    die("Error reading file into data: %s", strerror(errno));
    if (d.len + ret < d.len)
    die("Overflow reading file into data\n");
    d.len += ret;
    }
    return d;
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_data(d: data, p: *const c_void, len: c_int) -> data {
    struct data data_append_data(struct data d, const void *p, int len)
    {
    d = data_grow_for(d, len);
    memcpy(d.val + d.len, p, len);
    d.len += len;
    return d;
    }
    struct data data_insert_at_marker(struct data d, struct marker *m,
    const void *p, int len)
    {
    d = data_grow_for(d, len);
    memmove(d.val + m.offset + len, d.val + m.offset, d.len - m.offset);
    memcpy(d.val + m.offset, p, len);
    d.len += len;
// Adjust all markers after the one we're inserting at
    m = m.next;
    for_each_marker(m)
    m.offset += len;
    return d;
    }
#[no_mangle]
unsafe extern "C" fn data_append_markers(d: data, m: *mut marker) -> data {
    static struct data data_append_markers(struct data d, struct marker *m)
    {
    struct marker **mp = &d.markers;
// Find the end of the markerlist
    while (*mp)
    mp = &((*mp).next);
// mp = m;
    return d;
    }
#[no_mangle]
pub unsafe extern "C" fn data_merge(d1: data, d2: data) -> data {
    struct data data_merge(struct data d1, struct data d2)
    {
    struct data d;
    struct marker *m2 = d2.markers;
    d = data_append_markers(data_append_data(d1, d2.val, d2.len), m2);
// Adjust for the length of d1
    for_each_marker(m2)
    m2.offset += d1.len;
    d2.markers = core::ptr::null_mut(); /* So data_free() doesn't clobber them */
    data_free(d2);
    return d;
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_integer(d: data, value: u64, bits: c_int) -> data {
    struct data data_append_integer(struct data d, uint64_t value, int bits)
    {
    uint8_t value_8;
    fdt16_t value_16;
    fdt32_t value_32;
    fdt64_t value_64;
    switch (bits) {
    case 8:
    value_8 = value;
    return data_append_data(d, &value_8, 1);
    case 16:
    value_16 = cpu_to_fdt16(value);
    return data_append_data(d, &value_16, 2);
    case 32:
    value_32 = cpu_to_fdt32(value);
    return data_append_data(d, &value_32, 4);
    case 64:
    value_64 = cpu_to_fdt64(value);
    return data_append_data(d, &value_64, 8);
    default:
    die("Invalid literal size (%d)\n", bits);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_re(d: data, address: u64, size: u64) -> data {
    struct data data_append_re(struct data d, uint64_t address, uint64_t size)
    {
    struct fdt_reserve_entry re;
    re.address = cpu_to_fdt64(address);
    re.size = cpu_to_fdt64(size);
    return data_append_data(d, &re, sizeof(re));
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_cell(d: data, word: cell_t) -> data {
    struct data data_append_cell(struct data d, cell_t word)
    {
    return data_append_integer(d, word, sizeof(word) * 8);
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_addr(d: data, addr: u64) -> data {
    struct data data_append_addr(struct data d, uint64_t addr)
    {
    return data_append_integer(d, addr, sizeof(addr) * 8);
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_byte(d: data, byte: u8) -> data {
    struct data data_append_byte(struct data d, uint8_t byte)
    {
    return data_append_data(d, &byte, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_zeroes(d: data, len: c_int) -> data {
    struct data data_append_zeroes(struct data d, int len)
    {
    d = data_grow_for(d, len);
    memset(d.val + d.len, 0, len);
    d.len += len;
    return d;
    }
#[no_mangle]
pub unsafe extern "C" fn data_append_align(d: data, align: c_int) -> data {
    struct data data_append_align(struct data d, int align)
    {
    let mut newlen: c_int = ALIGN(d.len, align);
    return data_append_zeroes(d, newlen - d.len);
    }
#[no_mangle]
pub unsafe extern "C" fn data_add_marker(d: data, type: enum markertype, ref: *mut c_char) -> data {
    struct data data_add_marker(struct data d, enum markertype type, char *ref)
    {
    struct marker *m;
    m = alloc_marker(d.len, type, ref);
    return data_append_markers(d, m);
    }
#[no_mangle]
pub unsafe extern "C" fn data_is_one_string(d: data) -> bool {
    bool data_is_one_string(struct data d)
    {
    int i;
    let mut len: c_int = d.len;
    if (len == 0)
    return false;
    for (i = 0; i < len-1; i++)
    if (d.val[i] == '\0')
    return false;
    if (d.val[len-1] != '\0')
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn data_insert_data(d: data, m: *mut marker, old: data) -> data {
    struct data data_insert_data(struct data d, struct marker *m, struct data old)
    {
    let mut offset: c_uint = m.offset;
    struct marker *next = m.next;
    struct marker *marker;
    struct data new_data;
    char *ref;
    new_data = data_insert_at_marker(d, m, old.val, old.len);
// Copy all markers from old value
    marker = old.markers;
    for_each_marker(marker) {
    ref = core::ptr::null_mut();
    if (marker.ref)
    ref = xstrdup(marker.ref);
    m.next = alloc_marker(marker.offset + offset, marker.type,
    ref);
    m = m.next;
    }
    m.next = next;
    return new_data;
    }
    struct marker *alloc_marker(unsigned int offset, enum markertype type,
    char *ref)
    {
    struct marker *m;
    m = xmalloc(sizeof(*m));
    m.offset = offset;
    m.type = type;
    m.ref = ref;
    m.next = core::ptr::null_mut();
    return m;
    }
