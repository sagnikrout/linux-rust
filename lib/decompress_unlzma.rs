//! Automatically rewritten from C to Rust
//! Source: lib/decompress_unlzma.c
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


// Lzma decompressor for Linux kernel. Shamelessly snarfed
// from busybox 1.1.1
//
// Linux kernel adaptation
// Copyright (C) 2006  Alain < alain@knaff.lu >
//
// Based on small lzma deflate implementation/Small range coder
// implementation for lzma.
// Copyright (C) 2006  Aurelien Jacobs < aurel@gnuage.org >
//
// Based on LzmaDecode.c from the LZMA SDK 4.22 (https://www.7-zip.org/)
// Copyright (C) 1999-2005  Igor Pavlov
//
// Copyrights of the parts, see headers below.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//

// Macro flag: #define PREBOOT

#[no_mangle]
unsafe extern "C" fn read_int(ptr: *mut c_uchar, size: c_int) -> long long INIT {
    static long long INIT read_int(unsigned char *ptr, int size)
    {
    int i;
    let mut ret: c_longlong = 0;
    for (i = 0; i < size; i++)
    ret = (ret << 8) | ptr[size-i-1];
    return ret;
    }

    x = (typeof(x))read_int((unsigned char *)&x, sizeof(x))
// Small range coder implementation for lzma.
// Copyright (C) 2006  Aurelien Jacobs < aurel@gnuage.org >
//
// Based on LzmaDecode.c from the LZMA SDK 4.22 (https://www.7-zip.org/)
// Copyright (c) 1999-2005  Igor Pavlov
//

pub const LZMA_IOBUF_SIZE: c_uint = 0x10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc {
    pub long): *mut *mut *mut long (fill)(void, unsigned,
    pub ptr: *mut u8,
    pub buffer: *mut u8,
    pub buffer_end: *mut u8,
    pub buffer_size: c_long,
    pub code: u32,
    pub range: u32,
    pub bound: u32,
    pub ): *mut *mut void (error)(char,
}

pub const RC_TOP_BITS: c_int = 24;
pub const RC_MOVE_BITS: c_int = 5;
pub const RC_MODEL_TOTAL_BITS: c_int = 11;
#[no_mangle]
unsafe extern "C" fn nofill(buffer: *mut c_void, len: c_ulong) -> long INIT {
    static long INIT nofill(void *buffer, unsigned long len)
    {
    return -1;
    }
// Called twice: once at startup and once in rc_normalize()
#[no_mangle]
unsafe extern "C" fn rc_read(rc: *mut rc) -> void INIT {
    static void INIT rc_read(struct rc *rc)
    {
    rc.buffer_size = rc.fill((char *)rc.buffer, LZMA_IOBUF_SIZE);
    if (rc.buffer_size <= 0)
    rc.error("unexpected EOF");
    rc.ptr = rc.buffer;
    rc.buffer_end = rc.buffer + rc.buffer_size;
    }
// Called once
    static inline void INIT rc_init(struct rc *rc,
    long (*fill)(void*, unsigned long),
    char *buffer, long buffer_size)
    {
    if (fill)
    rc.fill = fill;
    else
    rc.fill = nofill;
    rc.buffer = (uint8_t *)buffer;
    rc.buffer_size = buffer_size;
    rc.buffer_end = rc.buffer + rc.buffer_size;
    rc.ptr = rc.buffer;
    rc.code = 0;
    rc.range = 0xFFFFFFFF;
    }
#[no_mangle]
pub unsafe extern "C" fn rc_init_code(rc: *mut rc) -> void INIT {
    static inline void INIT rc_init_code(struct rc *rc)
    {
    int i;
    for (i = 0; i < 5; i++) {
    if (rc.ptr >= rc.buffer_end)
    rc_read(rc);
    rc.code = (rc.code << 8) | *rc.ptr++;
    }
    }
// Called twice, but one callsite is in inline'd rc_is_bit_0_helper()
#[no_mangle]
unsafe extern "C" fn rc_do_normalize(rc: *mut rc) -> void INIT {
    static void INIT rc_do_normalize(struct rc *rc)
    {
    if (rc.ptr >= rc.buffer_end)
    rc_read(rc);
    rc.range <<= 8;
    rc.code = (rc.code << 8) | *rc.ptr++;
    }
#[no_mangle]
pub unsafe extern "C" fn rc_normalize(rc: *mut rc) -> void INIT {
    static inline void INIT rc_normalize(struct rc *rc)
    {
    if (rc.range < (1 << RC_TOP_BITS))
    rc_do_normalize(rc);
    }
// Called 9 times
// Why rc_is_bit_0_helper exists?
// Because we want to always expose (rc->code < rc->bound) to optimizer
//
#[no_mangle]
pub unsafe extern "C" fn rc_is_bit_0_helper(rc: *mut rc, p: *mut u16) -> uint32_t INIT {
    static inline uint32_t INIT rc_is_bit_0_helper(struct rc *rc, uint16_t *p)
    {
    rc_normalize(rc);
    rc.bound = *p * (rc.range >> RC_MODEL_TOTAL_BITS);
    return rc.bound;
    }
#[no_mangle]
pub unsafe extern "C" fn rc_is_bit_0(rc: *mut rc, p: *mut u16) -> int INIT {
    static inline int INIT rc_is_bit_0(struct rc *rc, uint16_t *p)
    {
    let mut t: u32 = rc_is_bit_0_helper(rc, p);
    return rc.code < t;
    }
// Called ~10 times, but very small, thus inlined
#[no_mangle]
pub unsafe extern "C" fn rc_update_bit_0(rc: *mut rc, p: *mut u16) -> void INIT {
    static inline void INIT rc_update_bit_0(struct rc *rc, uint16_t *p)
    {
    rc.range = rc.bound;
// p += ((1 << RC_MODEL_TOTAL_BITS) - *p) >> RC_MOVE_BITS;
    }
#[no_mangle]
pub unsafe extern "C" fn rc_update_bit_1(rc: *mut rc, p: *mut u16) -> void INIT {
    static inline void INIT rc_update_bit_1(struct rc *rc, uint16_t *p)
    {
    rc.range -= rc.bound;
    rc.code -= rc.bound;
// p -= *p >> RC_MOVE_BITS;
    }
// Called 4 times in unlzma loop
#[no_mangle]
unsafe extern "C" fn rc_get_bit(rc: *mut rc, p: *mut u16, symbol: *mut c_int) -> int INIT {
    static int INIT rc_get_bit(struct rc *rc, uint16_t *p, int *symbol)
    {
    if (rc_is_bit_0(rc, p)) {
    rc_update_bit_0(rc, p);
// symbol *= 2;
    return 0;
    } else {
    rc_update_bit_1(rc, p);
// symbol = *symbol * 2 + 1;
    return 1;
    }
    }
// Called once
#[no_mangle]
pub unsafe extern "C" fn rc_direct_bit(rc: *mut rc) -> int INIT {
    static inline int INIT rc_direct_bit(struct rc *rc)
    {
    rc_normalize(rc);
    rc.range >>= 1;
    if (rc.code >= rc.range) {
    rc.code -= rc.range;
    return 1;
    }
    return 0;
    }
// Called twice
    static inline void INIT
    rc_bit_tree_decode(struct rc *rc, uint16_t *p, int num_levels, int *symbol)
    {
    let mut i: c_int = num_levels;
// symbol = 1;
    while (i--)
    rc_get_bit(rc, p + *symbol, symbol);
// symbol -= 1 << num_levels;
    }
//
// Small lzma deflate implementation.
// Copyright (C) 2006  Aurelien Jacobs < aurel@gnuage.org >
//
// Based on LzmaDecode.c from the LZMA SDK 4.22 (https://www.7-zip.org/)
// Copyright (C) 1999-2005  Igor Pavlov
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lzma_header {
    pub pos: u8,
    pub dict_size: u32,
    pub dst_size: u64,
// C attribute field omitted
pub const LZMA_BASE_SIZE: c_int = 1846;
pub const LZMA_LIT_SIZE: c_int = 768;
pub const LZMA_NUM_POS_BITS_MAX: c_int = 4;
pub const LZMA_LEN_NUM_LOW_BITS: c_int = 3;
pub const LZMA_LEN_NUM_MID_BITS: c_int = 3;
pub const LZMA_LEN_NUM_HIGH_BITS: c_int = 8;
pub const LZMA_LEN_CHOICE: c_int = 0;

    + (1 << (LZMA_NUM_POS_BITS_MAX + LZMA_LEN_NUM_LOW_BITS)))

    +(1 << (LZMA_NUM_POS_BITS_MAX + LZMA_LEN_NUM_MID_BITS)))

pub const LZMA_NUM_STATES: c_int = 12;
pub const LZMA_NUM_LIT_STATES: c_int = 7;
pub const LZMA_START_POS_MODEL_INDEX: c_int = 4;
pub const LZMA_END_POS_MODEL_INDEX: c_int = 14;

pub const LZMA_NUM_POS_SLOT_BITS: c_int = 6;
pub const LZMA_NUM_LEN_TO_POS_STATES: c_int = 4;
pub const LZMA_NUM_ALIGN_BITS: c_int = 4;
pub const LZMA_MATCH_MIN_LEN: c_int = 2;
pub const LZMA_IS_MATCH: c_int = 0;

    + (LZMA_NUM_STATES << LZMA_NUM_POS_BITS_MAX))

    +(LZMA_NUM_LEN_TO_POS_STATES << LZMA_NUM_POS_SLOT_BITS))

    + LZMA_NUM_FULL_DISTANCES - LZMA_END_POS_MODEL_INDEX)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct writer {
    pub buffer: *mut u8,
    pub previous_byte: u8,
    pub buffer_pos: usize,
    pub bufsize: c_int,
    pub global_pos: usize,
    pub long): *mut *mut *mut long (flush)(void, unsigned,
    pub header: *mut lzma_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate {
    pub state: c_int,
    pub rep3: uint32_t rep0, rep1, rep2,,
}

#[no_mangle]
pub unsafe extern "C" fn get_pos(wr: *mut writer) -> size_t INIT {
    static inline size_t INIT get_pos(struct writer *wr)
    {
    return
    wr.global_pos + wr.buffer_pos;
    }
    static inline uint8_t INIT peek_old_byte(struct writer *wr,
    uint32_t offs)
    {
    if (!wr.flush) {
    int32_t pos;
    while (offs > wr.header.dict_size)
    offs -= wr.header.dict_size;
    pos = wr.buffer_pos - offs;
    return wr.buffer[pos];
    } else {
    let mut pos: u32 = wr.buffer_pos - offs;
    while (pos >= wr.header.dict_size)
    pos += wr.header.dict_size;
    return wr.buffer[pos];
    }
    }
#[no_mangle]
pub unsafe extern "C" fn write_byte(wr: *mut writer, byte: u8) -> int INIT {
    static inline int INIT write_byte(struct writer *wr, uint8_t byte)
    {
    wr.buffer[wr.buffer_pos++] = wr.previous_byte = byte;
    if (wr.flush && wr.buffer_pos == wr.header.dict_size) {
    wr.buffer_pos = 0;
    wr.global_pos += wr.header.dict_size;
    if (wr.flush((char *)wr.buffer, wr.header.dict_size)
    != wr.header.dict_size)
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn copy_byte(wr: *mut writer, offs: u32) -> int INIT {
    static inline int INIT copy_byte(struct writer *wr, uint32_t offs)
    {
    return write_byte(wr, peek_old_byte(wr, offs));
    }
    static inline int INIT copy_bytes(struct writer *wr,
    uint32_t rep0, int len)
    {
    do {
    if (copy_byte(wr, rep0))
    return -1;
    len--;
    } while (len != 0 && wr.buffer_pos < wr.header.dst_size);
    return len;
    }
    static inline int INIT process_bit0(struct writer *wr, struct rc *rc,
    struct cstate *cst, uint16_t *p,
    int pos_state, uint16_t *prob,
    int lc, uint32_t literal_pos_mask) {
    let mut mi: c_int = 1;
    rc_update_bit_0(rc, prob);
    prob = (p + LZMA_LITERAL +
    (LZMA_LIT_SIZE
// (((get_pos(wr) & literal_pos_mask) << lc)
    + (wr.previous_byte >> (8 - lc))))
    );
    if (cst.state >= LZMA_NUM_LIT_STATES) {
    let mut match_byte: c_int = peek_old_byte(wr, cst.rep0);
    do {
    int bit;
    uint16_t *prob_lit;
    match_byte <<= 1;
    bit = match_byte & 0x100;
    prob_lit = prob + 0x100 + bit + mi;
    if (rc_get_bit(rc, prob_lit, &mi)) {
    if (!bit)
    break;
    } else {
    if (bit)
    break;
    }
    } while (mi < 0x100);
    }
    while (mi < 0x100) {
    uint16_t *prob_lit = prob + mi;
    rc_get_bit(rc, prob_lit, &mi);
    }
    if (cst.state < 4)
    cst.state = 0;
#[no_mangle]
pub unsafe extern "C" fn if(10: cst->state <) -> else {
    else if (cst.state < 10)
    cst.state -= 3;
    else
    cst.state -= 6;
    return write_byte(wr, mi);
    }
    static inline int INIT process_bit1(struct writer *wr, struct rc *rc,
    struct cstate *cst, uint16_t *p,
    int pos_state, uint16_t *prob) {
    int offset;
    uint16_t *prob_len;
    int num_bits;
    int len;
    rc_update_bit_1(rc, prob);
    prob = p + LZMA_IS_REP + cst.state;
    if (rc_is_bit_0(rc, prob)) {
    rc_update_bit_0(rc, prob);
    cst.rep3 = cst.rep2;
    cst.rep2 = cst.rep1;
    cst.rep1 = cst.rep0;
    cst.state = cst.state < LZMA_NUM_LIT_STATES ? 0 : 3;
    prob = p + LZMA_LEN_CODER;
    } else {
    rc_update_bit_1(rc, prob);
    prob = p + LZMA_IS_REP_G0 + cst.state;
    if (rc_is_bit_0(rc, prob)) {
    rc_update_bit_0(rc, prob);
    prob = (p + LZMA_IS_REP_0_LONG
    + (cst.state <<
    LZMA_NUM_POS_BITS_MAX) +
    pos_state);
    if (rc_is_bit_0(rc, prob)) {
    rc_update_bit_0(rc, prob);
    cst.state = cst.state < LZMA_NUM_LIT_STATES ?
    9 : 11;
    return copy_byte(wr, cst.rep0);
    } else {
    rc_update_bit_1(rc, prob);
    }
    } else {
    uint32_t distance;
    rc_update_bit_1(rc, prob);
    prob = p + LZMA_IS_REP_G1 + cst.state;
    if (rc_is_bit_0(rc, prob)) {
    rc_update_bit_0(rc, prob);
    distance = cst.rep1;
    } else {
    rc_update_bit_1(rc, prob);
    prob = p + LZMA_IS_REP_G2 + cst.state;
    if (rc_is_bit_0(rc, prob)) {
    rc_update_bit_0(rc, prob);
    distance = cst.rep2;
    } else {
    rc_update_bit_1(rc, prob);
    distance = cst.rep3;
    cst.rep3 = cst.rep2;
    }
    cst.rep2 = cst.rep1;
    }
    cst.rep1 = cst.rep0;
    cst.rep0 = distance;
    }
    cst.state = cst.state < LZMA_NUM_LIT_STATES ? 8 : 11;
    prob = p + LZMA_REP_LEN_CODER;
    }
    prob_len = prob + LZMA_LEN_CHOICE;
    if (rc_is_bit_0(rc, prob_len)) {
    rc_update_bit_0(rc, prob_len);
    prob_len = (prob + LZMA_LEN_LOW
    + (pos_state <<
    LZMA_LEN_NUM_LOW_BITS));
    offset = 0;
    num_bits = LZMA_LEN_NUM_LOW_BITS;
    } else {
    rc_update_bit_1(rc, prob_len);
    prob_len = prob + LZMA_LEN_CHOICE_2;
    if (rc_is_bit_0(rc, prob_len)) {
    rc_update_bit_0(rc, prob_len);
    prob_len = (prob + LZMA_LEN_MID
    + (pos_state <<
    LZMA_LEN_NUM_MID_BITS));
    offset = 1 << LZMA_LEN_NUM_LOW_BITS;
    num_bits = LZMA_LEN_NUM_MID_BITS;
    } else {
    rc_update_bit_1(rc, prob_len);
    prob_len = prob + LZMA_LEN_HIGH;
    offset = ((1 << LZMA_LEN_NUM_LOW_BITS)
    + (1 << LZMA_LEN_NUM_MID_BITS));
    num_bits = LZMA_LEN_NUM_HIGH_BITS;
    }
    }
    rc_bit_tree_decode(rc, prob_len, num_bits, &len);
    len += offset;
    if (cst.state < 4) {
    int pos_slot;
    cst.state += LZMA_NUM_LIT_STATES;
    prob =
    p + LZMA_POS_SLOT +
    ((len <
    LZMA_NUM_LEN_TO_POS_STATES ? len :
    LZMA_NUM_LEN_TO_POS_STATES - 1)
    << LZMA_NUM_POS_SLOT_BITS);
    rc_bit_tree_decode(rc, prob,
    LZMA_NUM_POS_SLOT_BITS,
    &pos_slot);
    if (pos_slot >= LZMA_START_POS_MODEL_INDEX) {
    int i, mi;
    num_bits = (pos_slot >> 1) - 1;
    cst.rep0 = 2 | (pos_slot & 1);
    if (pos_slot < LZMA_END_POS_MODEL_INDEX) {
    cst.rep0 <<= num_bits;
    prob = p + LZMA_SPEC_POS +
    cst.rep0 - pos_slot - 1;
    } else {
    num_bits -= LZMA_NUM_ALIGN_BITS;
    while (num_bits--)
    cst.rep0 = (cst.rep0 << 1) |
    rc_direct_bit(rc);
    prob = p + LZMA_ALIGN;
    cst.rep0 <<= LZMA_NUM_ALIGN_BITS;
    num_bits = LZMA_NUM_ALIGN_BITS;
    }
    i = 1;
    mi = 1;
    while (num_bits--) {
    if (rc_get_bit(rc, prob + mi, &mi))
    cst.rep0 |= i;
    i <<= 1;
    }
    } else
    cst.rep0 = pos_slot;
    if (++(cst.rep0) == 0)
    return 0;
    if (cst.rep0 > wr.header.dict_size
    || cst.rep0 > get_pos(wr))
    return -1;
    }
    len += LZMA_MATCH_MIN_LEN;
    return copy_bytes(wr, cst.rep0, len);
    }
    STATIC inline int INIT unlzma(unsigned char *buf, long in_len,
    long (*fill)(void*, unsigned long),
    long (*flush)(void*, unsigned long),
    unsigned char *output,
    long *posp,
    void(*error)(char *x)
    )
    {
    struct lzma_header header;
    int lc, pb, lp;
    uint32_t pos_state_mask;
    uint32_t literal_pos_mask;
    uint16_t *p;
    int num_probs;
    struct rc rc;
    int i, mi;
    struct writer wr;
    struct cstate cst;
    unsigned char *inbuf;
    let mut ret: c_int = -1;
    rc.error = error;
    if (buf)
    inbuf = buf;
    else
    inbuf = malloc(LZMA_IOBUF_SIZE);
    if (!inbuf) {
    error("Could not allocate input buffer");
    goto exit_0;
    }
    cst.state = 0;
    cst.rep0 = cst.rep1 = cst.rep2 = cst.rep3 = 1;
    wr.header = &header;
    wr.flush = flush;
    wr.global_pos = 0;
    wr.previous_byte = 0;
    wr.buffer_pos = 0;
    rc_init(&rc, fill, inbuf, in_len);
    for (i = 0; i < sizeof(header); i++) {
    if (rc.ptr >= rc.buffer_end)
    rc_read(&rc);
    ((unsigned char *)&header)[i] = *rc.ptr++;
    }
    if (header.pos >= (9 * 5 * 5)) {
    error("bad header");
    goto exit_1;
    }
    mi = 0;
    lc = header.pos;
    while (lc >= 9) {
    mi++;
    lc -= 9;
    }
    pb = 0;
    lp = mi;
    while (lp >= 5) {
    pb++;
    lp -= 5;
    }
    pos_state_mask = (1 << pb) - 1;
    literal_pos_mask = (1 << lp) - 1;
    ENDIAN_CONVERT(header.dict_size);
    ENDIAN_CONVERT(header.dst_size);
    if (header.dict_size == 0)
    header.dict_size = 1;
    if (output)
    wr.buffer = output;
    else {
    wr.bufsize = MIN(header.dst_size, header.dict_size);
    wr.buffer = large_malloc(wr.bufsize);
    }
    if (wr.buffer == core::ptr::null_mut())
    goto exit_1;
    num_probs = LZMA_BASE_SIZE + (LZMA_LIT_SIZE << (lc + lp));
    p = (uint16_t *) large_malloc(num_probs * sizeof(*p));
    if (p == core::ptr::null_mut())
    goto exit_2;
    num_probs = LZMA_LITERAL + (LZMA_LIT_SIZE << (lc + lp));
    for (i = 0; i < num_probs; i++)
    p[i] = (1 << RC_MODEL_TOTAL_BITS) >> 1;
    rc_init_code(&rc);
    while (get_pos(&wr) < header.dst_size) {
    let mut pos_state: c_int = get_pos(&wr) & pos_state_mask;
    uint16_t *prob = p + LZMA_IS_MATCH +
    (cst.state << LZMA_NUM_POS_BITS_MAX) + pos_state;
    if (rc_is_bit_0(&rc, prob)) {
    if (process_bit0(&wr, &rc, &cst, p, pos_state, prob,
    lc, literal_pos_mask)) {
    error("LZMA data is corrupt");
    goto exit_3;
    }
    } else {
    if (process_bit1(&wr, &rc, &cst, p, pos_state, prob)) {
    error("LZMA data is corrupt");
    goto exit_3;
    }
    if (cst.rep0 == 0)
    break;
    }
    if (rc.buffer_size <= 0)
    goto exit_3;
    }
    if (posp)
// posp = rc.ptr-rc.buffer;
    if (!wr.flush || wr.flush(wr.buffer, wr.buffer_pos) == wr.buffer_pos)
    ret = 0;
    exit_3:
    large_free(p);
    exit_2:
    if (!output)
    large_free(wr.buffer);
    exit_1:
    if (!buf)
    free(inbuf);
    exit_0:
    return ret;
    }

    STATIC int INIT __decompress(unsigned char *buf, long in_len,
    long (*fill)(void*, unsigned long),
    long (*flush)(void*, unsigned long),
    unsigned char *output, long out_len,
    long *posp,
    void (*error)(char *x))
    {
    return unlzma(buf, in_len - 4, fill, flush, output, posp, error);
    }
