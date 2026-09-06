//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_h323_asn1.c
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
//
// BER and PER decoding library for H.323 conntrack/NAT module.
//
// Copyright (c) 2006 by Jing Min Zhao <zhaojingmin@users.sourceforge.net>
//
// See nf_conntrack_helper_h323_asn1.h for details.
//

// Trace Flag

pub const H323_TRACE: c_int = 0;

pub const TAB_SIZE: c_int = 4;

// Macro flag: #define FNAME(name)

// ASN.1 Types
pub const NUL: c_int = 0;
pub const BOOL: c_int = 1;
pub const OID: c_int = 2;
pub const INT: c_int = 3;
pub const ENUM: c_int = 4;
pub const BITSTR: c_int = 5;
pub const NUMSTR: c_int = 6;
pub const NUMDGT: c_int = 6;
pub const TBCDSTR: c_int = 6;
pub const OCTSTR: c_int = 7;
pub const PRTSTR: c_int = 7;
pub const IA5STR: c_int = 7;
pub const GENSTR: c_int = 7;
pub const BMPSTR: c_int = 8;
pub const SEQ: c_int = 9;
pub const SET: c_int = 9;
pub const SEQOF: c_int = 10;
pub const SETOF: c_int = 10;
pub const CHOICE: c_int = 11;
// Constraint Types
pub const FIXD: c_int = 0;
// #define BITS 1-8
pub const BYTE: c_int = 9;
pub const WORD: c_int = 10;
pub const CONS: c_int = 11;
pub const SEMI: c_int = 12;
pub const UNCO: c_int = 13;
// ASN.1 Type Attributes
pub const SKIP: c_int = 0;
pub const STOP: c_int = 1;
pub const DECODE: c_int = 2;
pub const EXT: c_int = 4;
pub const OPEN: c_int = 8;
pub const OPT: c_int = 16;
// ASN.1 Field Structure
    typedef struct field_t {

    char *name;

    unsigned char type;
    unsigned char sz;
    unsigned char lb;
    unsigned char ub;
    unsigned short attr;
    unsigned short offset;
    const struct field_t *fields;
    } field_t;
// Bit Stream
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitstr {
    pub buf: *mut c_uchar,
    pub beg: *mut c_uchar,
    pub end: *mut c_uchar,
    pub cur: *mut c_uchar,
    pub bit: c_uint,
}

// Tool Functions

    static unsigned int get_len(struct bitstr *bs);
    static unsigned int get_bit(struct bitstr *bs);
    static unsigned int get_bits(struct bitstr *bs, unsigned int b);
    static unsigned int get_bitmap(struct bitstr *bs, unsigned int b);
    static unsigned int get_uint(struct bitstr *bs, int b);
// Decoder Functions
    static int decode_nul(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_bool(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_oid(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_int(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_enum(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_bitstr(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_numstr(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_octstr(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_bmpstr(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_seq(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_seqof(struct bitstr *bs, const struct field_t *f, char *base, int level);
    static int decode_choice(struct bitstr *bs, const struct field_t *f, char *base, int level);
// Decoder Functions Vector
    typedef int (*decoder_t)(struct bitstr *, const struct field_t *, char *, int);
    static const decoder_t Decoders[] = {
    decode_nul,
    decode_bool,
    decode_oid,
    decode_int,
    decode_enum,
    decode_bitstr,
    decode_numstr,
    decode_octstr,
    decode_bmpstr,
    decode_seq,
    decode_seqof,
    decode_choice,
    };
//
// H.323 Types
//

//
// Functions
//
// Assume bs is aligned && v < 16384
#[no_mangle]
unsafe extern "C" fn get_len(bs: *mut bitstr) -> c_uint {
    static unsigned int get_len(struct bitstr *bs)
    {
    unsigned int v;
    v = *bs.cur++;
    if (v & 0x80) {
    v &= 0x3f;
    v <<= 8;
    v += *bs.cur++;
    }
    return v;
    }
#[no_mangle]
unsafe extern "C" fn nf_h323_error_boundary(bs: *mut bitstr, bytes: usize, bits: usize) -> c_int {
    static int nf_h323_error_boundary(struct bitstr *bs, size_t bytes, size_t bits)
    {
    bits += bs.bit;
    bytes += bits / BITS_PER_BYTE;
    if (bits % BITS_PER_BYTE > 0)
    bytes++;
    if (bs.cur + bytes > bs.end)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_bit(bs: *mut bitstr) -> c_uint {
    static unsigned int get_bit(struct bitstr *bs)
    {
    let mut b: c_uint = (*bs.cur) & (0x80 >> bs.bit);
    INC_BIT(bs);
    return b;
    }
// Assume b <= 8
#[no_mangle]
unsafe extern "C" fn get_bits(bs: *mut bitstr, b: c_uint) -> c_uint {
    static unsigned int get_bits(struct bitstr *bs, unsigned int b)
    {
    unsigned int v, l;
    v = (*bs.cur) & (0xffU >> bs.bit);
    l = b + bs.bit;
    if (l < 8) {
    v >>= 8 - l;
    bs.bit = l;
    } else if (l == 8) {
    bs.cur++;
    bs.bit = 0;
    } else {		/* l > 8 */
    v <<= 8;
    v += *(++bs.cur);
    v >>= 16 - l;
    bs.bit = l - 8;
    }
    return v;
    }
// Assume b <= 32
#[no_mangle]
unsafe extern "C" fn get_bitmap(bs: *mut bitstr, b: c_uint) -> c_uint {
    static unsigned int get_bitmap(struct bitstr *bs, unsigned int b)
    {
    unsigned int v, l, shift, bytes;
    if (!b)
    return 0;
    l = bs.bit + b;
    if (l < 8) {
    v = (unsigned int)(*bs.cur) << (bs.bit + 24);
    bs.bit = l;
    } else if (l == 8) {
    v = (unsigned int)(*bs.cur++) << (bs.bit + 24);
    bs.bit = 0;
    } else {
    for (bytes = l >> 3, shift = 24, v = 0; bytes;
    bytes--, shift -= 8)
    v |= (unsigned int)(*bs.cur++) << shift;
    if (l < 32) {
    v |= (unsigned int)(*bs.cur) << shift;
    v <<= bs.bit;
    } else if (l > 32) {
    v <<= bs.bit;
    v |= (*bs.cur) >> (8 - bs.bit);
    }
    bs.bit = l & 0x7;
    }
    v &= 0xffffffff << (32 - b);
    return v;
    }
//
// Assume bs is aligned and sizeof(unsigned int) == 4
//
#[no_mangle]
unsafe extern "C" fn get_uint(bs: *mut bitstr, b: c_int) -> c_uint {
    static unsigned int get_uint(struct bitstr *bs, int b)
    {
    let mut v: c_uint = 0;
    switch (b) {
    case 4:
    v |= *bs.cur++;
    v <<= 8;
    fallthrough;
    case 3:
    v |= *bs.cur++;
    v <<= 8;
    fallthrough;
    case 2:
    v |= *bs.cur++;
    v <<= 8;
    fallthrough;
    case 1:
    v |= *bs.cur++;
    break;
    }
    return v;
    }
    static int decode_nul(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
    return H323_ERROR_NONE;
    }
    static int decode_bool(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
    INC_BIT(bs);
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_oid(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    int len;
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 1, 0))
    return H323_ERROR_BOUND;
    len = *bs.cur++;
    bs.cur += len;
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_int(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int len;
    PRINT("%*s%s", level * TAB_SIZE, " ", f.name);
    switch (f.sz) {
    case BYTE:		/* Range == 256 */
    BYTE_ALIGN(bs);
    bs.cur++;
    break;
    case WORD:		/* 257 <= Range <= 64K */
    BYTE_ALIGN(bs);
    bs.cur += 2;
    break;
    case CONS:		/* 64K < Range < 4G */
    if (nf_h323_error_boundary(bs, 0, 2))
    return H323_ERROR_BOUND;
    len = get_bits(bs, 2) + 1;
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    BYTE_ALIGN(bs);
    if (base && (f.attr & DECODE)) {	/* timeToLive */
    let mut v: c_uint = get_uint(bs, len) + f.lb;
    PRINT(" = %u", v);
// ((unsigned int *)(base + f->offset)) = v;
    }
    bs.cur += len;
    break;
    case UNCO:
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    bs.cur += len;
    break;
    default:		/* 2 <= Range <= 255 */
    INC_BITS(bs, f.sz);
    break;
    }
    PRINT("\n");
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_enum(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
    if ((f.attr & EXT) && get_bit(bs)) {
    INC_BITS(bs, 7);
    } else {
    INC_BITS(bs, f.sz);
    }
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_bitstr(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int len;
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
    BYTE_ALIGN(bs);
    switch (f.sz) {
    case FIXD:		/* fixed length > 16 */
    len = f.lb;
    break;
    case WORD:		/* 2-byte length */
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = (*bs.cur++) << 8;
    len += (*bs.cur++) + f.lb;
    break;
    case SEMI:
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    break;
    default:
    len = 0;
    break;
    }
    bs.cur += len >> 3;
    bs.bit = len & 7;
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_numstr(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int len;
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
// 2 <= Range <= 255
    if (nf_h323_error_boundary(bs, 0, f.sz))
    return H323_ERROR_BOUND;
    len = get_bits(bs, f.sz) + f.lb;
    BYTE_ALIGN(bs);
    INC_BITS(bs, (len << 2));
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_octstr(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int len;
    PRINT("%*s%s", level * TAB_SIZE, " ", f.name);
    switch (f.sz) {
    case FIXD:		/* Range == 1 */
    if (f.lb > 2) {
    BYTE_ALIGN(bs);
    if (base && (f.attr & DECODE)) {
// The IP Address
// ((unsigned int *)(base + f->offset)) =
    bs.cur - bs.buf;
    }
    }
    len = f.lb;
    break;
    case BYTE:		/* Range == 256 */
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 1, 0))
    return H323_ERROR_BOUND;
    len = (*bs.cur++) + f.lb;
    break;
    case SEMI:
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs) + f.lb;
    break;
    default:		/* 2 <= Range <= 255 */
    if (nf_h323_error_boundary(bs, 0, f.sz))
    return H323_ERROR_BOUND;
    len = get_bits(bs, f.sz) + f.lb;
    BYTE_ALIGN(bs);
    break;
    }
    bs.cur += len;
    PRINT("\n");
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_bmpstr(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int len;
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
    switch (f.sz) {
    case BYTE:		/* Range == 256 */
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 1, 0))
    return H323_ERROR_BOUND;
    len = (*bs.cur++) + f.lb;
    break;
    default:		/* 2 <= Range <= 255 */
    if (nf_h323_error_boundary(bs, 0, f.sz))
    return H323_ERROR_BOUND;
    len = get_bits(bs, f.sz) + f.lb;
    BYTE_ALIGN(bs);
    break;
    }
    bs.cur += len << 1;
    if (nf_h323_error_boundary(bs, 0, 0))
    return H323_ERROR_BOUND;
    return H323_ERROR_NONE;
    }
    static int decode_seq(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int ext, bmp, i, opt, len = 0, bmp2, bmp2_len;
    int err;
    const struct field_t *son;
    unsigned char *beg = core::ptr::null_mut();
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
// Decode?
    base = (base && (f.attr & DECODE)) ? base + f.offset : core::ptr::null_mut();
// Extensible?
    if (nf_h323_error_boundary(bs, 0, 1))
    return H323_ERROR_BOUND;
    ext = (f.attr & EXT) ? get_bit(bs) : 0;
// Get fields bitmap
    if (nf_h323_error_boundary(bs, 0, f.sz))
    return H323_ERROR_BOUND;
    if (f.sz > 32)
    return H323_ERROR_RANGE;
    bmp = get_bitmap(bs, f.sz);
    if (base)
// (unsigned int *)base = bmp;
// Decode the root components
    for (i = opt = 0, son = f.fields; i < f.lb; i++, son++) {
    if (son.attr & STOP) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE, " ",
    son.name);
    return H323_ERROR_STOP;
    }
    if (son.attr & OPT) {	/* Optional component */
    if (!((0x80000000U >> (opt++)) & bmp))	/* Not exist */
    continue;
    }
// Decode
    if (son.attr & OPEN) {	/* Open field */
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    if (!base || !(son.attr & DECODE)) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE,
    " ", son.name);
    bs.cur += len;
    continue;
    }
    beg = bs.cur;
// Decode
    if ((err = (Decoders[son.type]) (bs, son, base,
    level + 1)) <
    H323_ERROR_NONE)
    return err;
    bs.cur = beg + len;
    bs.bit = 0;
    } else if ((err = (Decoders[son.type]) (bs, son, base,
    level + 1)) <
    H323_ERROR_NONE)
    return err;
    }
// No extension?
    if (!ext)
    return H323_ERROR_NONE;
// Get the extension bitmap
    if (nf_h323_error_boundary(bs, 0, 7))
    return H323_ERROR_BOUND;
    bmp2_len = get_bits(bs, 7) + 1;
    if (nf_h323_error_boundary(bs, 0, bmp2_len))
    return H323_ERROR_BOUND;
    if (bmp2_len > 32)
    return H323_ERROR_RANGE;
    bmp2 = get_bitmap(bs, bmp2_len);
    bmp |= bmp2 >> f.sz;
    if (base)
// (unsigned int *)base = bmp;
    BYTE_ALIGN(bs);
// Decode the extension components
    for (opt = 0; opt < bmp2_len; opt++, i++, son++) {
// Check Range
    if (i >= f.ub) {	/* Newer Version? */
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    bs.cur += len;
    continue;
    }
    if (son.attr & STOP) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE, " ",
    son.name);
    return H323_ERROR_STOP;
    }
    if (!((0x80000000 >> opt) & bmp2))	/* Not present */
    continue;
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    if (!base || !(son.attr & DECODE)) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE, " ",
    son.name);
    bs.cur += len;
    continue;
    }
    beg = bs.cur;
    if ((err = (Decoders[son.type]) (bs, son, base,
    level + 1)) <
    H323_ERROR_NONE)
    return err;
    bs.cur = beg + len;
    bs.bit = 0;
    }
    return H323_ERROR_NONE;
    }
    static int decode_seqof(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int count, effective_count = 0, i, len = 0;
    int err;
    const struct field_t *son;
    unsigned char *beg = core::ptr::null_mut();
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
// Decode?
    base = (base && (f.attr & DECODE)) ? base + f.offset : core::ptr::null_mut();
// Decode item count
    switch (f.sz) {
    case BYTE:
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 1, 0))
    return H323_ERROR_BOUND;
    count = *bs.cur++;
    break;
    case WORD:
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    count = *bs.cur++;
    count <<= 8;
    count += *bs.cur++;
    break;
    case SEMI:
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    count = get_len(bs);
    break;
    default:
    if (nf_h323_error_boundary(bs, 0, f.sz))
    return H323_ERROR_BOUND;
    count = get_bits(bs, f.sz);
    break;
    }
    count += f.lb;
// Write Count
    if (base) {
    effective_count = count > f.ub ? f.ub : count;
// (unsigned int *)base = effective_count;
    base += sizeof(unsigned int);
    }
// Decode nested field
    son = f.fields;
    if (base)
    base -= son.offset;
    for (i = 0; i < count; i++) {
    if (son.attr & OPEN) {
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    if (!base || !(son.attr & DECODE)) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE,
    " ", son.name);
    bs.cur += len;
    continue;
    }
    beg = bs.cur;
    if ((err = (Decoders[son.type]) (bs, son,
    i <
    effective_count ?
    base : core::ptr::null_mut(),
    level + 1)) <
    H323_ERROR_NONE)
    return err;
    bs.cur = beg + len;
    bs.bit = 0;
    } else
    if ((err = (Decoders[son.type]) (bs, son,
    i <
    effective_count ?
    base : core::ptr::null_mut(),
    level + 1)) <
    H323_ERROR_NONE)
    return err;
    if (base)
    base += son.offset;
    }
    return H323_ERROR_NONE;
    }
    static int decode_choice(struct bitstr *bs, const struct field_t *f,
    char *base, int level)
    {
    unsigned int type, ext, len = 0;
    int err;
    const struct field_t *son;
    unsigned char *beg = core::ptr::null_mut();
    PRINT("%*s%s\n", level * TAB_SIZE, " ", f.name);
// Decode?
    base = (base && (f.attr & DECODE)) ? base + f.offset : core::ptr::null_mut();
// Decode the choice index number
    if (nf_h323_error_boundary(bs, 0, 1))
    return H323_ERROR_BOUND;
    if ((f.attr & EXT) && get_bit(bs)) {
    ext = 1;
    if (nf_h323_error_boundary(bs, 0, 7))
    return H323_ERROR_BOUND;
    type = get_bits(bs, 7) + f.lb;
    } else {
    ext = 0;
    if (nf_h323_error_boundary(bs, 0, f.sz))
    return H323_ERROR_BOUND;
    type = get_bits(bs, f.sz);
    if (type >= f.lb)
    return H323_ERROR_RANGE;
    }
// Write Type
    if (base)
// (unsigned int *)base = type;
// Check Range
    if (type >= f.ub) {	/* Newer version? */
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    bs.cur += len;
    return H323_ERROR_NONE;
    }
// Transfer to son level
    son = &f.fields[type];
    if (son.attr & STOP) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE, " ", son.name);
    return H323_ERROR_STOP;
    }
    if (ext || (son.attr & OPEN)) {
    BYTE_ALIGN(bs);
    if (nf_h323_error_boundary(bs, 2, 0))
    return H323_ERROR_BOUND;
    len = get_len(bs);
    if (nf_h323_error_boundary(bs, len, 0))
    return H323_ERROR_BOUND;
    if (!base || !(son.attr & DECODE)) {
    PRINT("%*s%s\n", (level + 1) * TAB_SIZE, " ",
    son.name);
    bs.cur += len;
    return H323_ERROR_NONE;
    }
    beg = bs.cur;
    if ((err = (Decoders[son.type]) (bs, son, base, level + 1)) <
    H323_ERROR_NONE)
    return err;
    bs.cur = beg + len;
    bs.bit = 0;
    } else if ((err = (Decoders[son.type]) (bs, son, base, level + 1)) <
    H323_ERROR_NONE)
    return err;
    return H323_ERROR_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn DecodeRasMessage(buf: *mut c_uchar, sz: usize, ras: *mut RasMessage) -> c_int {
    int DecodeRasMessage(unsigned char *buf, size_t sz, RasMessage *ras)
    {
    static const struct field_t ras_message = {
    FNAME("RasMessage") CHOICE, 5, 24, 32, DECODE | EXT,
    0, _RasMessage
    };
    struct bitstr bs;
    bs.buf = bs.beg = bs.cur = buf;
    bs.end = buf + sz;
    bs.bit = 0;
    return decode_choice(&bs, &ras_message, (char *) ras, 0);
    }
    static int DecodeH323_UserInformation(unsigned char *buf, unsigned char *beg,
    size_t sz, H323_UserInformation *uuie)
    {
    static const struct field_t h323_userinformation = {
    FNAME("H323-UserInformation") SEQ, 1, 2, 2, DECODE | EXT,
    0, _H323_UserInformation
    };
    struct bitstr bs;
    bs.buf = buf;
    bs.beg = bs.cur = beg;
    bs.end = beg + sz;
    bs.bit = 0;
    return decode_seq(&bs, &h323_userinformation, (char *) uuie, 0);
    }
    int DecodeMultimediaSystemControlMessage(unsigned char *buf, size_t sz,
    MultimediaSystemControlMessage *
    mscm)
    {
    static const struct field_t multimediasystemcontrolmessage = {
    FNAME("MultimediaSystemControlMessage") CHOICE, 2, 4, 4,
    DECODE | EXT, 0, _MultimediaSystemControlMessage
    };
    struct bitstr bs;
    bs.buf = bs.beg = bs.cur = buf;
    bs.end = buf + sz;
    bs.bit = 0;
    return decode_choice(&bs, &multimediasystemcontrolmessage,
    (char *) mscm, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn DecodeQ931(buf: *mut c_uchar, sz: usize, q931: *mut Q931) -> c_int {
    int DecodeQ931(unsigned char *buf, size_t sz, Q931 *q931)
    {
    unsigned char *p = buf;
    int len;
    if (!p || sz < 1)
    return H323_ERROR_BOUND;
// Protocol Discriminator
    if (*p != 0x08) {
    PRINT("Unknown Protocol Discriminator\n");
    return H323_ERROR_RANGE;
    }
    p++;
    sz--;
// CallReferenceValue
    if (sz < 1)
    return H323_ERROR_BOUND;
    len = *p++;
    sz--;
    if (sz < len)
    return H323_ERROR_BOUND;
    p += len;
    sz -= len;
// Message Type
    if (sz < 2)
    return H323_ERROR_BOUND;
    q931.MessageType = *p++;
    sz--;
    PRINT("MessageType = %02X\n", q931.MessageType);
    if (*p & 0x80) {
    p++;
    sz--;
    }
// Decode Information Elements
    while (sz > 0) {
    if (*p == 0x7e) {	/* UserUserIE */
    if (sz < 3)
    break;
    p++;
    len = *p++ << 8;
    len |= *p++;
    sz -= 3;
    if (sz < len)
    break;
    p++;
    len--;
    if (len <= 0)
    break;
    return DecodeH323_UserInformation(buf, p, len,
    &q931.UUIE);
    }
    p++;
    sz--;
    if (sz < 1)
    break;
    len = *p++;
    sz--;
    if (sz < len)
    break;
    p += len;
    sz -= len;
    }
    PRINT("Q.931 UUIE not found\n");
    return H323_ERROR_BOUND;
    }
