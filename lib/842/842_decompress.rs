//! Automatically rewritten from C to Rust
//! Source: lib/842/842_decompress.c
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
// 842 Software Decompression
//
// Copyright (C) 2015 Dan Streetman, IBM Corp
//
// See 842.h for details of the 842 compressed format.
//

// rolling fifo sizes

    static u8 decomp_ops[OPS_MAX][4] = {
    { D8, N0, N0, N0 },
    { D4, D2, I2, N0 },
    { D4, I2, D2, N0 },
    { D4, I2, I2, N0 },
    { D4, I4, N0, N0 },
    { D2, I2, D4, N0 },
    { D2, I2, D2, I2 },
    { D2, I2, I2, D2 },
    { D2, I2, I2, I2 },
    { D2, I2, I4, N0 },
    { I2, D2, D4, N0 },
    { I2, D4, I2, N0 },
    { I2, D2, I2, D2 },
    { I2, D2, I2, I2 },
    { I2, D2, I4, N0 },
    { I2, I2, D4, N0 },
    { I2, I2, D2, I2 },
    { I2, I2, I2, D2 },
    { I2, I2, I2, I2 },
    { I2, I2, I4, N0 },
    { I4, D4, N0, N0 },
    { I4, D2, I2, N0 },
    { I4, I2, D2, N0 },
    { I4, I2, I2, N0 },
    { I4, I4, N0, N0 },
    { I8, N0, N0, N0 }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw842_param {
    pub in: *mut u8,
    pub bit: u8,
    pub ilen: u64,
    pub out: *mut u8,
    pub ostart: *mut u8,
    pub olen: u64,
}

    ((s) == 2 ? be16_to_cpu(get_unaligned((__be16 *)d)) :	\
    (s) == 4 ? be32_to_cpu(get_unaligned((__be32 *)d)) :	\
    (s) == 8 ? be64_to_cpu(get_unaligned((__be64 *)d)) :	\
    0)
    static int next_bits(struct sw842_param *p, u64 *d, u8 n);
#[no_mangle]
unsafe extern "C" fn __split_next_bits(p: *mut sw842_param, d: *mut u64, n: u8, s: u8) -> c_int {
    static int __split_next_bits(struct sw842_param *p, u64 *d, u8 n, u8 s)
    {
    let mut tmp: u64 = 0;
    int ret;
    if (n <= s) {
    pr_debug("split_next_bits invalid n %u s %u\n", n, s);
    return -EINVAL;
    }
    ret = next_bits(p, &tmp, n - s);
    if (ret)
    return ret;
    ret = next_bits(p, d, s);
    if (ret)
    return ret;
// d |= tmp << s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn next_bits(p: *mut sw842_param, d: *mut u64, n: u8) -> c_int {
    static int next_bits(struct sw842_param *p, u64 *d, u8 n)
    {
    u8 *in = p.in, b = p.bit, bits = b + n;
    if (n > 64) {
    pr_debug("next_bits invalid n %u\n", n);
    return -EINVAL;
    }
// split this up if reading > 8 bytes, or if we're at the end of
// the input buffer and would read past the end
//
    if (bits > 64)
    return __split_next_bits(p, d, n, 32);
#[no_mangle]
pub unsafe extern "C" fn if(56: p->ilen < 8 && bits > 32 && bits <=) -> else {
    else if (p.ilen < 8 && bits > 32 && bits <= 56)
    return __split_next_bits(p, d, n, 16);
#[no_mangle]
pub unsafe extern "C" fn if(24: p->ilen < 4 && bits > 16 && bits <=) -> else {
    else if (p.ilen < 4 && bits > 16 && bits <= 24)
    return __split_next_bits(p, d, n, 8);
    if (DIV_ROUND_UP(bits, 8) > p.ilen)
    return -EOVERFLOW;
    if (bits <= 8)
// d = *in >> (8 - bits);
#[no_mangle]
pub unsafe extern "C" fn if(16: bits <=) -> else {
    else if (bits <= 16)
// d = be16_to_cpu(get_unaligned((__be16 *)in)) >> (16 - bits);
#[no_mangle]
pub unsafe extern "C" fn if(32: bits <=) -> else {
    else if (bits <= 32)
// d = be32_to_cpu(get_unaligned((__be32 *)in)) >> (32 - bits);
    else
// d = be64_to_cpu(get_unaligned((__be64 *)in)) >> (64 - bits);
// d &= GENMASK_ULL(n - 1, 0);
    p.bit += n;
    if (p.bit > 7) {
    p.in += p.bit / 8;
    p.ilen -= p.bit / 8;
    p.bit %= 8;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_data(p: *mut sw842_param, n: u8) -> c_int {
    static int do_data(struct sw842_param *p, u8 n)
    {
    u64 v;
    int ret;
    if (n > p.olen)
    return -ENOSPC;
    ret = next_bits(p, &v, n * 8);
    if (ret)
    return ret;
    switch (n) {
    case 2:
    put_unaligned(cpu_to_be16((u16)v), (__be16 *)p.out);
    break;
    case 4:
    put_unaligned(cpu_to_be32((u32)v), (__be32 *)p.out);
    break;
    case 8:
    put_unaligned(cpu_to_be64((u64)v), (__be64 *)p.out);
    break;
    default:
    return -EINVAL;
    }
    p.out += n;
    p.olen -= n;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __do_index(p: *mut sw842_param, size: u8, bits: u8, fsize: u64) -> c_int {
    static int __do_index(struct sw842_param *p, u8 size, u8 bits, u64 fsize)
    {
    u64 index, offset, total = round_down(p.out - p.ostart, 8);
    int ret;
    ret = next_bits(p, &index, bits);
    if (ret)
    return ret;
    offset = index * size;
// a ring buffer of fsize is used; correct the offset
    if (total > fsize) {
// this is where the current fifo is
    let mut section: u64 = round_down(total, fsize);
// the current pos in the fifo
    let mut pos: u64 = total - section;
// if the offset is past/at the pos, we need to
// go back to the last fifo section
//
    if (offset >= pos)
    section -= fsize;
    offset += section;
    }
    if (offset + size > total) {
    pr_debug("index%x %lx points past end %lx\n", size,
    (unsigned long)offset, (unsigned long)total);
    return -EINVAL;
    }
    if (size != 2 && size != 4 && size != 8)
    WARN(1, "__do_index invalid size %x\n", size);
    else
    pr_debug("index%x to %lx off %lx adjoff %lx tot %lx data %lx\n",
    size, (unsigned long)index,
    (unsigned long)(index * size), (unsigned long)offset,
    (unsigned long)total,
    (unsigned long)beN_to_cpu(&p.ostart[offset], size));
    memcpy(p.out, &p.ostart[offset], size);
    p.out += size;
    p.olen -= size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_index(p: *mut sw842_param, n: u8) -> c_int {
    static int do_index(struct sw842_param *p, u8 n)
    {
    switch (n) {
    case 2:
    return __do_index(p, 2, I2_BITS, I2_FIFO_SIZE);
    case 4:
    return __do_index(p, 4, I4_BITS, I4_FIFO_SIZE);
    case 8:
    return __do_index(p, 8, I8_BITS, I8_FIFO_SIZE);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_op(p: *mut sw842_param, o: u8) -> c_int {
    static int do_op(struct sw842_param *p, u8 o)
    {
    int i, ret = 0;
    if (o >= OPS_MAX)
    return -EINVAL;
    for (i = 0; i < 4; i++) {
    let mut op: u8 = decomp_ops[o][i];
    pr_debug("op is %x\n", op);
    switch (op & OP_ACTION) {
    case OP_ACTION_DATA:
    ret = do_data(p, op & OP_AMOUNT);
    break;
    case OP_ACTION_INDEX:
    ret = do_index(p, op & OP_AMOUNT);
    break;
    case OP_ACTION_NOOP:
    break;
    default:
    pr_err("Internal error, invalid op %x\n", op);
    return -EINVAL;
    }
    if (ret)
    return ret;
    }
    if (sw842_template_counts)
    atomic_inc(&template_count[o]);
    return 0;
    }
//
// sw842_decompress
//
// Decompress the 842-compressed buffer of length @ilen at @in
// to the output buffer @out, using no more than @olen bytes.
//
// The compressed buffer must be only a single 842-compressed buffer,
// with the standard format described in the comments in 842.h
// Processing will stop when the 842 "END" template is detected,
// not the end of the buffer.
//
// Returns: 0 on success, error on failure.  The @olen parameter
// will contain the number of output bytes written on success, or
// 0 on error.
//
    int sw842_decompress(const u8 *in, unsigned int ilen,
    u8 *out, unsigned int *olen)
    {
    struct sw842_param p;
    int ret;
    u64 op, rep, tmp, bytes, total;
    u64 crc;
    p.in = (u8 *)in;
    p.bit = 0;
    p.ilen = ilen;
    p.out = out;
    p.ostart = out;
    p.olen = *olen;
    total = p.olen;
// olen = 0;
    do {
    ret = next_bits(&p, &op, OP_BITS);
    if (ret)
    return ret;
    pr_debug("template is %lx\n", (unsigned long)op);
    switch (op) {
    case OP_REPEAT:
    ret = next_bits(&p, &rep, REPEAT_BITS);
    if (ret)
    return ret;
    if (p.out == out) /* no previous bytes */
    return -EINVAL;
// copy rep + 1
    rep++;
    if (rep * 8 > p.olen)
    return -ENOSPC;
    while (rep-- > 0) {
    memcpy(p.out, p.out - 8, 8);
    p.out += 8;
    p.olen -= 8;
    }
    if (sw842_template_counts)
    atomic_inc(&template_repeat_count);
    break;
    case OP_ZEROS:
    if (8 > p.olen)
    return -ENOSPC;
    memset(p.out, 0, 8);
    p.out += 8;
    p.olen -= 8;
    if (sw842_template_counts)
    atomic_inc(&template_zeros_count);
    break;
    case OP_SHORT_DATA:
    ret = next_bits(&p, &bytes, SHORT_DATA_BITS);
    if (ret)
    return ret;
    if (!bytes || bytes > SHORT_DATA_BITS_MAX)
    return -EINVAL;
    while (bytes-- > 0) {
    ret = next_bits(&p, &tmp, 8);
    if (ret)
    return ret;
// p.out = (u8)tmp;
    p.out++;
    p.olen--;
    }
    if (sw842_template_counts)
    atomic_inc(&template_short_data_count);
    break;
    case OP_END:
    if (sw842_template_counts)
    atomic_inc(&template_end_count);
    break;
    default: /* use template */
    ret = do_op(&p, op);
    if (ret)
    return ret;
    break;
    }
    } while (op != OP_END);
//
// crc(0:31) is saved in compressed data starting with the
// next bit after End of stream template.
//
    ret = next_bits(&p, &crc, CRC_BITS);
    if (ret)
    return ret;
//
// Validate CRC saved in compressed data.
//
    if (crc != (u64)crc32_be(0, out, total - p.olen)) {
    pr_debug("CRC mismatch for decompression\n");
    return -EINVAL;
    }
    if (unlikely((total - p.olen) > UINT_MAX))
    return -ENOSPC;
// olen = total - p.olen;
    return 0;
    }
    EXPORT_SYMBOL_GPL(sw842_decompress);
#[no_mangle]
unsafe extern "C" fn sw842_init() -> int __init {
    static int __init sw842_init(void)
    {
    if (sw842_template_counts)
    sw842_debugfs_create();
    return 0;
    }
    module_init(sw842_init);
#[no_mangle]
unsafe extern "C" fn sw842_exit() -> void __exit {
    static void __exit sw842_exit(void)
    {
    if (sw842_template_counts)
    sw842_debugfs_remove();
    }
    module_exit(sw842_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Software 842 Decompressor");
    MODULE_AUTHOR("Dan Streetman <ddstreet@ieee.org>");
