//! Automatically rewritten from C to Rust
//! Source: lib/xz/xz_dec_lzma2.c
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


// SPDX-License-Identifier: 0BSD
//
// LZMA2 decoder
//
// Authors: Lasse Collin <lasse.collin@tukaani.org>
// Igor Pavlov <https://7-zip.org/>
//

//
// Range decoder initialization eats the first five bytes of each LZMA chunk.
//
pub const RC_INIT_BYTES: c_int = 5;
//
// Minimum number of usable input buffer to safely decode one LZMA symbol.
// The worst case is that we decode 22 bits using probabilities and 26
// direct bits. This may decode at maximum of 20 bytes of input. However,
// lzma_main() does an extra normalization before returning, thus we
// need to put 21 here.
//
pub const LZMA_IN_REQUIRED: c_int = 21;
//
// Dictionary (history buffer)
//
// These are always true:
// start <= pos <= full <= end
// pos <= limit <= end
//
// In multi-call mode, also these are true:
// end == size
// size <= size_max
// allocated <= size
//
// Most of these variables are size_t to support single-call mode,
// in which the dictionary variables address the actual output
// buffer directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dictionary {
// Beginning of the history buffer
    pub buf: *mut u8,
// Old position in buf (before decoding more data)
    pub start: usize,
// Position in buf
    pub pos: usize,
//
// How full dictionary is. This is used to detect corrupt input that
// would read beyond the beginning of the uncompressed stream.
//
    pub full: usize,
// Write limit; we don't write to buf[limit] or later bytes.
    pub limit: usize,
//
// End of the dictionary buffer. In multi-call mode, this is
// the same as the dictionary size. In single-call mode, this
// indicates the size of the output buffer.
//
    pub end: usize,
//
// Size of the dictionary as specified in Block Header. This is used
// together with "full" to detect corrupt input that would make us
// read beyond the beginning of the uncompressed stream.
//
    pub size: u32,
//
// Maximum allowed dictionary size in multi-call mode.
// This is ignored in single-call mode.
//
    pub size_max: u32,
//
// Amount of memory currently allocated for the dictionary.
// This is used only with XZ_DYNALLOC. (With XZ_PREALLOC,
// size_max is always the same as the allocated size.)
//
    pub allocated: u32,
// Operation mode
    pub mode: enum xz_mode,
}

// Range decoder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_dec {
    pub range: u32,
    pub code: u32,
//
// Number of initializing bytes remaining to be read
// by rc_read_init().
//
    pub init_bytes_left: u32,
//
// Buffer from which we read our input. It can be either
// temp.buf or the caller-provided input buffer.
//
    pub in: *const u8,
    pub in_pos: usize,
    pub in_limit: usize,
}

// Probabilities for a length decoder.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lzma_len_dec {
// Probability of match length being at least 10
    pub choice: u16,
// Probability of match length being at least 18
    pub choice2: u16,
// Probabilities for match lengths 2-9
    pub low: [u16; POS_STATES_MAX][LEN_LOW_SYMBOLS],
// Probabilities for match lengths 10-17
    pub mid: [u16; POS_STATES_MAX][LEN_MID_SYMBOLS],
// Probabilities for match lengths 18-273
    pub high: [u16; LEN_HIGH_SYMBOLS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lzma_dec {
// Distances of latest four matches
    pub rep0: u32,
    pub rep1: u32,
    pub rep2: u32,
    pub rep3: u32,
//
// Length of a match. This is updated so that dict_repeat can
// be called again to finish repeating the whole match. This is
// size_t because a pointer to this is passed to dict_repeat,
// and there it's nicer to have size_t instead of uint32_t.
//
    pub len: usize,
// Types of the most recently seen LZMA symbols
    pub state: enum lzma_state,
//
// LZMA properties or related bit masks (number of literal
// context bits, a mask derived from the number of literal
// position bits, and a mask derived from the number
// position bits)
//
    pub lc: u32,
    pub /: *mut *mut uint32_t literal_pos_mask; / (1 << lp) - 1,
    pub /: *mut *mut uint32_t pos_mask; / (1 << pb) - 1,
// If 1, it's a match. Otherwise it's a single 8-bit literal.
    pub is_match: [u16; STATES][POS_STATES_MAX],
// If 1, it's a repeated match. The distance is one of rep0 .. rep3.
    pub is_rep: [u16; STATES],
//
// If 0, distance of a repeated match is rep0.
// Otherwise check is_rep1.
//
    pub is_rep0: [u16; STATES],
//
// If 0, distance of a repeated match is rep1.
// Otherwise check is_rep2.
//
    pub is_rep1: [u16; STATES],
// If 0, distance of a repeated match is rep2. Otherwise it is rep3.
    pub is_rep2: [u16; STATES],
//
// If 1, the repeated match has length of one byte. Otherwise
// the length is decoded from rep_len_decoder.
//
    pub is_rep0_long: [u16; STATES][POS_STATES_MAX],
//
// Probability tree for the highest two bits of the match
// distance. There is a separate probability tree for match
// lengths of 2 (i.e. MATCH_LEN_MIN), 3, 4, and [5, 273].
//
    pub dist_slot: [u16; DIST_STATES][DIST_SLOTS],
//
// Probility trees for additional bits for match distance
// when the distance is in the range [4, 127].
//
    pub DIST_MODEL_END]: uint16_t dist_special[FULL_DISTANCES -,
//
// Probability tree for the lowest four bits of a match
// distance that is equal to or greater than 128.
//
    pub dist_align: [u16; ALIGN_SIZE],
// Length of a normal match
    pub match_len_dec: lzma_len_dec,
// Length of a repeated match
    pub rep_len_dec: lzma_len_dec,
// Probabilities of literals
    pub literal: [u16; LITERAL_CODERS_MAX][LITERAL_CODER_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lzma2_dec {
// Position in xz_dec_lzma2_run().
    enum lzma2_seq {
    SEQ_CONTROL,
    SEQ_UNCOMPRESSED_1,
    SEQ_UNCOMPRESSED_2,
    SEQ_COMPRESSED_0,
    SEQ_COMPRESSED_1,
    SEQ_PROPERTIES,
    SEQ_LZMA_PREPARE,
    SEQ_LZMA_RUN,
    SEQ_COPY
    pub sequence: },
// Next position after decoding the compressed size of the chunk.
    pub next_sequence: enum lzma2_seq,
// Uncompressed size of LZMA chunk (2 MiB at maximum)
    pub uncompressed: usize,
//
// Compressed size of LZMA chunk or compressed/uncompressed
// size of uncompressed chunk (64 KiB at maximum)
//
    pub compressed: usize,
//
// True if dictionary reset is needed. This is false before
// the first chunk (LZMA or uncompressed).
//
    pub need_dict_reset: bool,
//
// True if new LZMA properties are needed. This is false
// before the first LZMA chunk.
//
    pub need_props: bool,

    pub pedantic_microlzma: bool,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xz_dec_lzma2 {
//
// The order below is important on x86 to reduce code size and
// it shouldn't hurt on other platforms. Everything up to and
// including lzma.pos_mask are in the first 128 bytes on x86-32,
// which allows using smaller instructions to access those
// variables. On x86-64, fewer variables fit into the first 128
// bytes, but this is still the best order without sacrificing
// the readability by splitting the structures.
//
    pub rc: rc_dec,
    pub dict: dictionary,
    pub lzma2: lzma2_dec,
    pub lzma: lzma_dec,
//
// Temporary buffer which holds a small number of input bytes between
// decoder calls. See lzma2_lzma() for details.
//
    struct {
    pub size: usize,
    pub LZMA_IN_REQUIRED]: *mut *mut uint8_t buf[3,
    pub temp: },
}

//
// Dictionary
//
// Reset the dictionary state. When in single-call mode, set up the beginning
// of the dictionary to point to the actual output buffer.
//
#[no_mangle]
unsafe extern "C" fn dict_reset(dict: *mut dictionary, b: *mut xz_buf) {
    static void dict_reset(struct dictionary *dict, struct xz_buf *b)
    {
    if (DEC_IS_SINGLE(dict.mode)) {
    dict.buf = b.out + b.out_pos;
    dict.end = b.out_size - b.out_pos;
    }
    dict.start = 0;
    dict.pos = 0;
    dict.limit = 0;
    dict.full = 0;
    }
// Set dictionary write limit
#[no_mangle]
unsafe extern "C" fn dict_limit(dict: *mut dictionary, out_max: usize) {
    static void dict_limit(struct dictionary *dict, size_t out_max)
    {
    if (dict.end - dict.pos <= out_max)
    dict.limit = dict.end;
    else
    dict.limit = dict.pos + out_max;
    }
// Return true if at least one byte can be written into the dictionary.
#[no_mangle]
pub unsafe extern "C" fn dict_has_space(dict: *const dictionary) -> bool {
    static inline bool dict_has_space(const struct dictionary *dict)
    {
    return dict.pos < dict.limit;
    }
//
// Get a byte from the dictionary at the given distance. The distance is
// assumed to valid, or as a special case, zero when the dictionary is
// still empty. This special case is needed for single-call decoding to
// avoid writing a '\0' to the end of the destination buffer.
//
#[no_mangle]
pub unsafe extern "C" fn dict_get(dict: *const dictionary, dist: usize) -> u32 {
    static inline uint32_t dict_get(const struct dictionary *dict, size_t dist)
    {
    let mut offset: usize = dict.pos - dist - 1;
    if (dist >= dict.pos)
    offset += dict.end;
    return dict.full > 0 ? dict.buf[offset] : 0;
    }
//
// Put one byte into the dictionary. It is assumed that there is space for it.
//
#[no_mangle]
pub unsafe extern "C" fn dict_put(dict: *mut dictionary, byte: u8) {
    static inline void dict_put(struct dictionary *dict, uint8_t byte)
    {
    dict.buf[dict.pos++] = byte;
    if (dict.full < dict.pos)
    dict.full = dict.pos;
    }
//
// Repeat given number of bytes from the given distance. If the distance is
// invalid, false is returned. On success, true is returned and *len is
// updated to indicate how many bytes were left to be repeated.
//
#[no_mangle]
unsafe extern "C" fn dict_repeat(dict: *mut dictionary, len: *mut usize, dist: usize) -> bool {
    static bool dict_repeat(struct dictionary *dict, size_t *len, size_t dist)
    {
    size_t back;
    size_t left;
    if (dist >= dict.full || dist >= dict.size)
    return false;
    left = min(dict.limit - dict.pos, *len);
// len -= left;
    back = dict.pos - dist - 1;
    if (dist >= dict.pos)
    back += dict.end;
    do {
    dict.buf[dict.pos++] = dict.buf[back++];
    if (back == dict.end)
    back = 0;
    } while (--left > 0);
    if (dict.full < dict.pos)
    dict.full = dict.pos;
    return true;
    }
// Copy uncompressed data as is from input to dictionary and output buffers.
    static void dict_uncompressed(struct dictionary *dict, struct xz_buf *b,
    size_t *left)
    {
    size_t copy_size;
    while (*left > 0 && b.in_pos < b.in_size
    && b.out_pos < b.out_size) {
    copy_size = min(b.in_size - b.in_pos,
    b.out_size - b.out_pos);
    if (copy_size > dict.end - dict.pos)
    copy_size = dict.end - dict.pos;
    if (copy_size > *left)
    copy_size = *left;
// left -= copy_size;
//
// If doing in-place decompression in single-call mode and the
// uncompressed size of the file is larger than the caller
// thought (i.e. it is invalid input!), the buffers below may
// overlap and cause undefined behavior with memcpy().
// With valid inputs memcpy() would be fine here.
//
    memmove(dict.buf + dict.pos, b.in + b.in_pos, copy_size);
    dict.pos += copy_size;
    if (dict.full < dict.pos)
    dict.full = dict.pos;
    if (DEC_IS_MULTI(dict.mode)) {
    if (dict.pos == dict.end)
    dict.pos = 0;
//
// Like above but for multi-call mode: use memmove()
// to avoid undefined behavior with invalid input.
//
    memmove(b.out + b.out_pos, b.in + b.in_pos,
    copy_size);
    }
    dict.start = dict.pos;
    b.out_pos += copy_size;
    b.in_pos += copy_size;
    }
    }

//
// Flush pending data from dictionary to b->out. It is assumed that there is
// enough space in b->out. This is guaranteed because caller uses dict_limit()
// before decoding data into the dictionary.
//
#[no_mangle]
unsafe extern "C" fn dict_flush(dict: *mut dictionary, b: *mut xz_buf) -> usize {
    static size_t dict_flush(struct dictionary *dict, struct xz_buf *b)
    {
    let mut copy_size: usize = dict.pos - dict.start;
    if (DEC_IS_MULTI(dict.mode)) {
    if (dict.pos == dict.end)
    dict.pos = 0;
//
// These buffers cannot overlap even if doing in-place
// decompression because in multi-call mode dict->buf
// has been allocated by us in this file; it's not
// provided by the caller like in single-call mode.
//
// With MicroLZMA, b->out can be NULL to skip bytes that
// the caller doesn't need. This cannot be done with XZ
// because it would break BCJ filters.
//
    if (!DICT_FLUSH_SUPPORTS_SKIPPING || b.out != core::ptr::null_mut())
    memcpy(b.out + b.out_pos, dict.buf + dict.start,
    copy_size);
    }
    dict.start = dict.pos;
    b.out_pos += copy_size;
    return copy_size;
    }
//
// Range decoder
//
// Reset the range decoder.
#[no_mangle]
unsafe extern "C" fn rc_reset(rc: *mut rc_dec) {
    static void rc_reset(struct rc_dec *rc)
    {
    rc.range = (uint32_t)-1;
    rc.code = 0;
    rc.init_bytes_left = RC_INIT_BYTES;
    }
//
// Read the first five initial bytes into rc->code if they haven't been
// read already. (Yes, the first byte gets completely ignored.)
//
#[no_mangle]
unsafe extern "C" fn rc_read_init(rc: *mut rc_dec, b: *mut xz_buf) -> bool {
    static bool rc_read_init(struct rc_dec *rc, struct xz_buf *b)
    {
    while (rc.init_bytes_left > 0) {
    if (b.in_pos == b.in_size)
    return false;
    rc.code = (rc.code << 8) + b.in[b.in_pos++];
    --rc.init_bytes_left;
    }
    return true;
    }
// Return true if there may not be enough input for the next decoding loop.
#[no_mangle]
pub unsafe extern "C" fn rc_limit_exceeded(rc: *const rc_dec) -> bool {
    static inline bool rc_limit_exceeded(const struct rc_dec *rc)
    {
    return rc.in_pos > rc.in_limit;
    }
//
// Return true if it is possible (from point of view of range decoder) that
// we have reached the end of the LZMA chunk.
//
#[no_mangle]
pub unsafe extern "C" fn rc_is_finished(rc: *const rc_dec) -> bool {
    static inline bool rc_is_finished(const struct rc_dec *rc)
    {
    return rc.code == 0;
    }
// Read the next input byte if needed.
#[no_mangle]
unsafe extern "C" fn rc_normalize(rc: *mut rc_dec) -> __always_inline void {
    static __always_inline void rc_normalize(struct rc_dec *rc)
    {
    if (rc.range < RC_TOP_VALUE) {
    rc.range <<= RC_SHIFT_BITS;
    rc.code = (rc.code << RC_SHIFT_BITS) + rc.in[rc.in_pos++];
    }
    }
//
// Decode one bit. In some versions, this function has been split in three
// functions so that the compiler is supposed to be able to more easily avoid
// an extra branch. In this particular version of the LZMA decoder, this
// doesn't seem to be a good idea (tested with GCC 3.3.6, 3.4.6, and 4.3.3
// on x86). Using a non-split version results in nicer looking code too.
//
// NOTE: This must return an int. Do not make it return a bool or the speed
// of the code generated by GCC 3.x decreases 10-15 %. (GCC 4.3 doesn't care,
// and it generates 10-20 % faster code than GCC 3.x from this file anyway.)
//
#[no_mangle]
unsafe extern "C" fn rc_bit(rc: *mut rc_dec, prob: *mut u16) -> __always_inline int {
    static __always_inline int rc_bit(struct rc_dec *rc, uint16_t *prob)
    {
    uint32_t bound;
    int bit;
    rc_normalize(rc);
    bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS) * *prob;
    if (rc.code < bound) {
    rc.range = bound;
// prob += (RC_BIT_MODEL_TOTAL - *prob) >> RC_MOVE_BITS;
    bit = 0;
    } else {
    rc.range -= bound;
    rc.code -= bound;
// prob -= *prob >> RC_MOVE_BITS;
    bit = 1;
    }
    return bit;
    }
// Decode a bittree starting from the most significant bit.
    static __always_inline uint32_t rc_bittree(struct rc_dec *rc,
    uint16_t *probs, uint32_t limit)
    {
    let mut symbol: u32 = 1;
    do {
    if (rc_bit(rc, &probs[symbol]))
    symbol = (symbol << 1) + 1;
    else
    symbol <<= 1;
    } while (symbol < limit);
    return symbol;
    }
// Decode a bittree starting from the least significant bit.
    static __always_inline void rc_bittree_reverse(struct rc_dec *rc,
    uint16_t *probs,
    uint32_t *dest, uint32_t limit)
    {
    let mut symbol: u32 = 1;
    let mut i: u32 = 0;
    do {
    if (rc_bit(rc, &probs[symbol])) {
    symbol = (symbol << 1) + 1;
// dest += 1 << i;
    } else {
    symbol <<= 1;
    }
    } while (++i < limit);
    }
// Decode direct bits (fixed fifty-fifty probability)
#[no_mangle]
pub unsafe extern "C" fn rc_direct(rc: *mut rc_dec, dest: *mut u32, limit: u32) {
    static inline void rc_direct(struct rc_dec *rc, uint32_t *dest, uint32_t limit)
    {
    uint32_t mask;
    do {
    rc_normalize(rc);
    rc.range >>= 1;
    rc.code -= rc.range;
    mask = (uint32_t)0 - (rc.code >> 31);
    rc.code += rc.range & mask;
// dest = (*dest << 1) + (mask + 1);
    } while (--limit > 0);
    }
//
// LZMA
//
// Get pointer to literal coder probability array.
    static uint16_t *lzma_literal_probs(struct xz_dec_lzma2 *s)
    {
    let mut prev_byte: u32 = dict_get(&s.dict, 0);
    let mut low: u32 = prev_byte >> (8 - s.lzma.lc);
    let mut high: u32 = (s.dict.pos & s.lzma.literal_pos_mask) << s.lzma.lc;
    return s.lzma.literal[low + high];
    }
// Decode a literal (one 8-bit byte)
#[no_mangle]
unsafe extern "C" fn lzma_literal(s: *mut xz_dec_lzma2) {
    static void lzma_literal(struct xz_dec_lzma2 *s)
    {
    uint16_t *probs;
    uint32_t symbol;
    uint32_t match_byte;
    uint32_t match_bit;
    uint32_t offset;
    uint32_t i;
    probs = lzma_literal_probs(s);
    if (lzma_state_is_literal(s.lzma.state)) {
    symbol = rc_bittree(&s.rc, probs, 0x100);
    } else {
    symbol = 1;
    match_byte = dict_get(&s.dict, s.lzma.rep0) << 1;
    offset = 0x100;
    do {
    match_bit = match_byte & offset;
    match_byte <<= 1;
    i = offset + match_bit + symbol;
    if (rc_bit(&s.rc, &probs[i])) {
    symbol = (symbol << 1) + 1;
    offset &= match_bit;
    } else {
    symbol <<= 1;
    offset &= ~match_bit;
    }
    } while (symbol < 0x100);
    }
    dict_put(&s.dict, (uint8_t)symbol);
    lzma_state_literal(&s.lzma.state);
    }
// Decode the length of the match into s->lzma.len.
    static void lzma_len(struct xz_dec_lzma2 *s, struct lzma_len_dec *l,
    uint32_t pos_state)
    {
    uint16_t *probs;
    uint32_t limit;
    if (!rc_bit(&s.rc, &l.choice)) {
    probs = l.low[pos_state];
    limit = LEN_LOW_SYMBOLS;
    s.lzma.len = MATCH_LEN_MIN;
    } else {
    if (!rc_bit(&s.rc, &l.choice2)) {
    probs = l.mid[pos_state];
    limit = LEN_MID_SYMBOLS;
    s.lzma.len = MATCH_LEN_MIN + LEN_LOW_SYMBOLS;
    } else {
    probs = l.high;
    limit = LEN_HIGH_SYMBOLS;
    s.lzma.len = MATCH_LEN_MIN + LEN_LOW_SYMBOLS
    + LEN_MID_SYMBOLS;
    }
    }
    s.lzma.len += rc_bittree(&s.rc, probs, limit) - limit;
    }
// Decode a match. The distance will be stored in s->lzma.rep0.
#[no_mangle]
unsafe extern "C" fn lzma_match(s: *mut xz_dec_lzma2, pos_state: u32) {
    static void lzma_match(struct xz_dec_lzma2 *s, uint32_t pos_state)
    {
    uint16_t *probs;
    uint32_t dist_slot;
    uint32_t limit;
    lzma_state_match(&s.lzma.state);
    s.lzma.rep3 = s.lzma.rep2;
    s.lzma.rep2 = s.lzma.rep1;
    s.lzma.rep1 = s.lzma.rep0;
    lzma_len(s, &s.lzma.match_len_dec, pos_state);
    probs = s.lzma.dist_slot[lzma_get_dist_state(s.lzma.len)];
    dist_slot = rc_bittree(&s.rc, probs, DIST_SLOTS) - DIST_SLOTS;
    if (dist_slot < DIST_MODEL_START) {
    s.lzma.rep0 = dist_slot;
    } else {
    limit = (dist_slot >> 1) - 1;
    s.lzma.rep0 = 2 + (dist_slot & 1);
    if (dist_slot < DIST_MODEL_END) {
    s.lzma.rep0 <<= limit;
    probs = s.lzma.dist_special + s.lzma.rep0
    - dist_slot - 1;
    rc_bittree_reverse(&s.rc, probs,
    &s.lzma.rep0, limit);
    } else {
    rc_direct(&s.rc, &s.lzma.rep0, limit - ALIGN_BITS);
    s.lzma.rep0 <<= ALIGN_BITS;
    rc_bittree_reverse(&s.rc, s.lzma.dist_align,
    &s.lzma.rep0, ALIGN_BITS);
    }
    }
    }
//
// Decode a repeated match. The distance is one of the four most recently
// seen matches. The distance will be stored in s->lzma.rep0.
//
#[no_mangle]
unsafe extern "C" fn lzma_rep_match(s: *mut xz_dec_lzma2, pos_state: u32) {
    static void lzma_rep_match(struct xz_dec_lzma2 *s, uint32_t pos_state)
    {
    uint32_t tmp;
    if (!rc_bit(&s.rc, &s.lzma.is_rep0[s.lzma.state])) {
    if (!rc_bit(&s.rc, &s.lzma.is_rep0_long[
    s.lzma.state][pos_state])) {
    lzma_state_short_rep(&s.lzma.state);
    s.lzma.len = 1;
    return;
    }
    } else {
    if (!rc_bit(&s.rc, &s.lzma.is_rep1[s.lzma.state])) {
    tmp = s.lzma.rep1;
    } else {
    if (!rc_bit(&s.rc, &s.lzma.is_rep2[s.lzma.state])) {
    tmp = s.lzma.rep2;
    } else {
    tmp = s.lzma.rep3;
    s.lzma.rep3 = s.lzma.rep2;
    }
    s.lzma.rep2 = s.lzma.rep1;
    }
    s.lzma.rep1 = s.lzma.rep0;
    s.lzma.rep0 = tmp;
    }
    lzma_state_long_rep(&s.lzma.state);
    lzma_len(s, &s.lzma.rep_len_dec, pos_state);
    }
// LZMA decoder core
#[no_mangle]
unsafe extern "C" fn lzma_main(s: *mut xz_dec_lzma2) -> bool {
    static bool lzma_main(struct xz_dec_lzma2 *s)
    {
    uint32_t pos_state;
//
// If the dictionary write limit was reached during the previous call,
// try to finish the possibly pending repeat in the dictionary.
//
    if (dict_has_space(&s.dict) && s.lzma.len > 0)
    dict_repeat(&s.dict, &s.lzma.len, s.lzma.rep0);
//
// Decode more LZMA symbols. One iteration may consume up to
// LZMA_IN_REQUIRED - 1 bytes.
//
    while (dict_has_space(&s.dict) && !rc_limit_exceeded(&s.rc)) {
    pos_state = s.dict.pos & s.lzma.pos_mask;
    if (!rc_bit(&s.rc, &s.lzma.is_match[
    s.lzma.state][pos_state])) {
    lzma_literal(s);
    } else {
    if (rc_bit(&s.rc, &s.lzma.is_rep[s.lzma.state]))
    lzma_rep_match(s, pos_state);
    else
    lzma_match(s, pos_state);
    if (!dict_repeat(&s.dict, &s.lzma.len, s.lzma.rep0))
    return false;
    }
    }
//
// Having the range decoder always normalized when we are outside
// this function makes it easier to correctly handle end of the chunk.
//
    rc_normalize(&s.rc);
    return true;
    }
//
// Reset the LZMA decoder and range decoder state. Dictionary is not reset
// here, because LZMA state may be reset without resetting the dictionary.
//
#[no_mangle]
unsafe extern "C" fn lzma_reset(s: *mut xz_dec_lzma2) {
    static void lzma_reset(struct xz_dec_lzma2 *s)
    {
    uint16_t *probs;
    size_t i;
    s.lzma.state = STATE_LIT_LIT;
    s.lzma.rep0 = 0;
    s.lzma.rep1 = 0;
    s.lzma.rep2 = 0;
    s.lzma.rep3 = 0;
    s.lzma.len = 0;
//
// All probabilities are initialized to the same value. This hack
// makes the code smaller by avoiding a separate loop for each
// probability array.
//
// This could be optimized so that only that part of literal
// probabilities that are actually required. In the common case
// we would write 12 KiB less.
//
    probs = s.lzma.is_match[0];
    for (i = 0; i < PROBS_TOTAL; ++i)
    probs[i] = RC_BIT_MODEL_TOTAL / 2;
    rc_reset(&s.rc);
    }
//
// Decode and validate LZMA properties (lc/lp/pb) and calculate the bit masks
// from the decoded lp and pb values. On success, the LZMA decoder state is
// reset and true is returned.
//
#[no_mangle]
unsafe extern "C" fn lzma_props(s: *mut xz_dec_lzma2, props: u8) -> bool {
    static bool lzma_props(struct xz_dec_lzma2 *s, uint8_t props)
    {
    if (props > (4 * 5 + 4) * 9 + 8)
    return false;
    s.lzma.pos_mask = 0;
    while (props >= 9 * 5) {
    props -= 9 * 5;
    ++s.lzma.pos_mask;
    }
    s.lzma.pos_mask = (1 << s.lzma.pos_mask) - 1;
    s.lzma.literal_pos_mask = 0;
    while (props >= 9) {
    props -= 9;
    ++s.lzma.literal_pos_mask;
    }
    s.lzma.lc = props;
    if (s.lzma.lc + s.lzma.literal_pos_mask > 4)
    return false;
    s.lzma.literal_pos_mask = (1 << s.lzma.literal_pos_mask) - 1;
    lzma_reset(s);
    return true;
    }
//
// LZMA2
//
// The LZMA decoder assumes that if the input limit (s->rc.in_limit) hasn't
// been exceeded, it is safe to read up to LZMA_IN_REQUIRED bytes. This
// wrapper function takes care of making the LZMA decoder's assumption safe.
//
// As long as there is plenty of input left to be decoded in the current LZMA
// chunk, we decode directly from the caller-supplied input buffer until
// there's LZMA_IN_REQUIRED bytes left. Those remaining bytes are copied into
// s->temp.buf, which (hopefully) gets filled on the next call to this
// function. We decode a few bytes from the temporary buffer so that we can
// continue decoding from the caller-supplied input buffer again.
//
#[no_mangle]
unsafe extern "C" fn lzma2_lzma(s: *mut xz_dec_lzma2, b: *mut xz_buf) -> bool {
    static bool lzma2_lzma(struct xz_dec_lzma2 *s, struct xz_buf *b)
    {
    size_t in_avail;
    size_t tmp;
    in_avail = b.in_size - b.in_pos;
    if (s.temp.size > 0 || s.lzma2.compressed == 0) {
    tmp = 2 * LZMA_IN_REQUIRED - s.temp.size;
    if (tmp > s.lzma2.compressed - s.temp.size)
    tmp = s.lzma2.compressed - s.temp.size;
    if (tmp > in_avail)
    tmp = in_avail;
    memcpy(s.temp.buf + s.temp.size, b.in + b.in_pos, tmp);
    if (s.temp.size + tmp == s.lzma2.compressed) {
    memzero(s.temp.buf + s.temp.size + tmp,
    sizeof(s.temp.buf)
    - s.temp.size - tmp);
    s.rc.in_limit = s.temp.size + tmp;
    } else if (s.temp.size + tmp < LZMA_IN_REQUIRED) {
    s.temp.size += tmp;
    b.in_pos += tmp;
    return true;
    } else {
    s.rc.in_limit = s.temp.size + tmp - LZMA_IN_REQUIRED;
    }
    s.rc.in = s.temp.buf;
    s.rc.in_pos = 0;
    if (!lzma_main(s) || s.rc.in_pos > s.temp.size + tmp)
    return false;
    s.lzma2.compressed -= s.rc.in_pos;
    if (s.rc.in_pos < s.temp.size) {
    s.temp.size -= s.rc.in_pos;
    memmove(s.temp.buf, s.temp.buf + s.rc.in_pos,
    s.temp.size);
    return true;
    }
    b.in_pos += s.rc.in_pos - s.temp.size;
    s.temp.size = 0;
    }
    in_avail = b.in_size - b.in_pos;
    if (in_avail >= LZMA_IN_REQUIRED) {
    s.rc.in = b.in;
    s.rc.in_pos = b.in_pos;
    if (in_avail >= s.lzma2.compressed + LZMA_IN_REQUIRED)
    s.rc.in_limit = b.in_pos + s.lzma2.compressed;
    else
    s.rc.in_limit = b.in_size - LZMA_IN_REQUIRED;
    if (!lzma_main(s))
    return false;
    in_avail = s.rc.in_pos - b.in_pos;
    if (in_avail > s.lzma2.compressed)
    return false;
    s.lzma2.compressed -= in_avail;
    b.in_pos = s.rc.in_pos;
    }
    in_avail = b.in_size - b.in_pos;
    if (in_avail < LZMA_IN_REQUIRED) {
    if (in_avail > s.lzma2.compressed)
    in_avail = s.lzma2.compressed;
    memcpy(s.temp.buf, b.in + b.in_pos, in_avail);
    s.temp.size = in_avail;
    b.in_pos += in_avail;
    }
    return true;
    }
//
// Take care of the LZMA2 control layer, and forward the job of actual LZMA
// decoding or copying of uncompressed chunks to other functions.
//
#[no_mangle]
pub unsafe extern "C" fn xz_dec_lzma2_run(s: *mut xz_dec_lzma2, b: *mut xz_buf) -> enum xz_ret {
    enum xz_ret xz_dec_lzma2_run(struct xz_dec_lzma2 *s, struct xz_buf *b)
    {
    uint32_t tmp;
    while (b.in_pos < b.in_size || s.lzma2.sequence == SEQ_LZMA_RUN) {
    switch (s.lzma2.sequence) {
    case SEQ_CONTROL:
//
// LZMA2 control byte
//
// Exact values:
// 0x00   End marker
// 0x01   Dictionary reset followed by
// an uncompressed chunk
// 0x02   Uncompressed chunk (no dictionary reset)
//
// Highest three bits (tmp & 0xE0):
// 0xE0   Dictionary reset, new properties and state
// reset, followed by LZMA compressed chunk
// 0xC0   New properties and state reset, followed
// by LZMA compressed chunk (no dictionary
// reset)
// 0xA0   State reset using old properties,
// followed by LZMA compressed chunk (no
// dictionary reset)
// 0x80   LZMA chunk (no dictionary or state reset)
//
// For LZMA compressed chunks, the lowest five bits
// (tmp & 1F) are the highest bits of the
// uncompressed size (bits 16-20).
//
// A new LZMA2 stream must begin with a dictionary
// reset. The first LZMA chunk must set new
// properties and reset the LZMA state.
//
// Values that don't match anything described above
// are invalid and we return XZ_DATA_ERROR.
//
    tmp = b.in[b.in_pos++];
    if (tmp == 0x00)
    return XZ_STREAM_END;
    if (tmp >= 0xE0 || tmp == 0x01) {
    s.lzma2.need_props = true;
    s.lzma2.need_dict_reset = false;
    dict_reset(&s.dict, b);
    } else if (s.lzma2.need_dict_reset) {
    return XZ_DATA_ERROR;
    }
    if (tmp >= 0x80) {
    s.lzma2.uncompressed = (tmp & 0x1F) << 16;
    s.lzma2.sequence = SEQ_UNCOMPRESSED_1;
    if (tmp >= 0xC0) {
//
// When there are new properties,
// state reset is done at
// SEQ_PROPERTIES.
//
    s.lzma2.need_props = false;
    s.lzma2.next_sequence
    = SEQ_PROPERTIES;
    } else if (s.lzma2.need_props) {
    return XZ_DATA_ERROR;
    } else {
    s.lzma2.next_sequence
    = SEQ_LZMA_PREPARE;
    if (tmp >= 0xA0)
    lzma_reset(s);
    }
    } else {
    if (tmp > 0x02)
    return XZ_DATA_ERROR;
    s.lzma2.sequence = SEQ_COMPRESSED_0;
    s.lzma2.next_sequence = SEQ_COPY;
    }
    break;
    case SEQ_UNCOMPRESSED_1:
    s.lzma2.uncompressed
    += (size_t)b.in[b.in_pos++] << 8;
    s.lzma2.sequence = SEQ_UNCOMPRESSED_2;
    break;
    case SEQ_UNCOMPRESSED_2:
    s.lzma2.uncompressed
    += (size_t)b.in[b.in_pos++] + 1;
    s.lzma2.sequence = SEQ_COMPRESSED_0;
    break;
    case SEQ_COMPRESSED_0:
    s.lzma2.compressed = (size_t)b.in[b.in_pos++] << 8;
    s.lzma2.sequence = SEQ_COMPRESSED_1;
    break;
    case SEQ_COMPRESSED_1:
    s.lzma2.compressed += (size_t)b.in[b.in_pos++] + 1;
    s.lzma2.sequence = s.lzma2.next_sequence;
    break;
    case SEQ_PROPERTIES:
    if (!lzma_props(s, b.in[b.in_pos++]))
    return XZ_DATA_ERROR;
    s.lzma2.sequence = SEQ_LZMA_PREPARE;
    fallthrough;
    case SEQ_LZMA_PREPARE:
    if (s.lzma2.compressed < RC_INIT_BYTES)
    return XZ_DATA_ERROR;
    if (!rc_read_init(&s.rc, b))
    return XZ_OK;
    s.lzma2.compressed -= RC_INIT_BYTES;
    s.lzma2.sequence = SEQ_LZMA_RUN;
    fallthrough;
    case SEQ_LZMA_RUN:
//
// Set dictionary limit to indicate how much we want
// to be decoded at maximum. Decode new data into the
// dictionary. Flush the new data from dictionary to
// b->out. Check if we finished decoding this chunk.
// In case the dictionary got full but we didn't fill
// the output buffer yet, we may run this loop
// multiple times without changing s->lzma2.sequence.
//
    dict_limit(&s.dict, min(b.out_size - b.out_pos,
    s.lzma2.uncompressed));
    if (!lzma2_lzma(s, b))
    return XZ_DATA_ERROR;
    s.lzma2.uncompressed -= dict_flush(&s.dict, b);
    if (s.lzma2.uncompressed == 0) {
    if (s.lzma2.compressed > 0 || s.lzma.len > 0
    || !rc_is_finished(&s.rc))
    return XZ_DATA_ERROR;
    rc_reset(&s.rc);
    s.lzma2.sequence = SEQ_CONTROL;
    } else if (b.out_pos == b.out_size
    || (b.in_pos == b.in_size
    && s.temp.size
    < s.lzma2.compressed)) {
    return XZ_OK;
    }
    break;
    case SEQ_COPY:
    dict_uncompressed(&s.dict, b, &s.lzma2.compressed);
    if (s.lzma2.compressed > 0)
    return XZ_OK;
    s.lzma2.sequence = SEQ_CONTROL;
    break;
    }
    }
    return XZ_OK;
    }
    struct xz_dec_lzma2 *xz_dec_lzma2_create(enum xz_mode mode, uint32_t dict_max)
    {
    struct xz_dec_lzma2 *s = kmalloc_obj(*s);
    if (s == core::ptr::null_mut())
    return core::ptr::null_mut();
    s.dict.mode = mode;
    s.dict.size_max = dict_max;
    if (DEC_IS_PREALLOC(mode)) {
    s.dict.buf = vmalloc(dict_max);
    if (s.dict.buf == core::ptr::null_mut()) {
    kfree(s);
    return core::ptr::null_mut();
    }
    } else if (DEC_IS_DYNALLOC(mode)) {
    s.dict.buf = core::ptr::null_mut();
    s.dict.allocated = 0;
    }
    return s;
    }
#[no_mangle]
pub unsafe extern "C" fn xz_dec_lzma2_reset(s: *mut xz_dec_lzma2, props: u8) -> enum xz_ret {
    enum xz_ret xz_dec_lzma2_reset(struct xz_dec_lzma2 *s, uint8_t props)
    {
// This limits dictionary size to 3 GiB to keep parsing simpler.
    if (props > 39)
    return XZ_OPTIONS_ERROR;
    s.dict.size = 2 + (props & 1);
    s.dict.size <<= (props >> 1) + 11;
    if (DEC_IS_MULTI(s.dict.mode)) {
    if (s.dict.size > s.dict.size_max)
    return XZ_MEMLIMIT_ERROR;
    s.dict.end = s.dict.size;
    if (DEC_IS_DYNALLOC(s.dict.mode)) {
    if (s.dict.allocated < s.dict.size) {
    s.dict.allocated = s.dict.size;
    vfree(s.dict.buf);
    s.dict.buf = vmalloc(s.dict.size);
    if (s.dict.buf == core::ptr::null_mut()) {
    s.dict.allocated = 0;
    return XZ_MEM_ERROR;
    }
    }
    }
    }
    s.lzma2.sequence = SEQ_CONTROL;
    s.lzma2.need_dict_reset = true;
    s.temp.size = 0;
    return XZ_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn xz_dec_lzma2_end(s: *mut xz_dec_lzma2) {
    void xz_dec_lzma2_end(struct xz_dec_lzma2 *s)
    {
    if (DEC_IS_MULTI(s.dict.mode))
    vfree(s.dict.buf);
    kfree(s);
    }

// This is a wrapper struct to have a nice struct name in the public API.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xz_dec_microlzma {
    pub s: xz_dec_lzma2,
}

    enum xz_ret xz_dec_microlzma_run(struct xz_dec_microlzma *s_ptr,
    struct xz_buf *b)
    {
    struct xz_dec_lzma2 *s = &s_ptr.s;
//
// sequence is SEQ_PROPERTIES before the first input byte,
// SEQ_LZMA_PREPARE until a total of five bytes have been read,
// and SEQ_LZMA_RUN for the rest of the input stream.
//
    if (s.lzma2.sequence != SEQ_LZMA_RUN) {
    if (s.lzma2.sequence == SEQ_PROPERTIES) {
// One byte is needed for the props.
    if (b.in_pos >= b.in_size)
    return XZ_OK;
//
// Don't increment b->in_pos here. The same byte is
// also passed to rc_read_init() which will ignore it.
//
    if (!lzma_props(s, ~b.in[b.in_pos]))
    return XZ_DATA_ERROR;
    s.lzma2.sequence = SEQ_LZMA_PREPARE;
    }
//
// xz_dec_microlzma_reset() doesn't validate the compressed
// size so we do it here. We have to limit the maximum size
// to avoid integer overflows in lzma2_lzma(). 3 GiB is a nice
// round number and much more than users of this code should
// ever need.
//
    if (s.lzma2.compressed < RC_INIT_BYTES
    || s.lzma2.compressed > (3U << 30))
    return XZ_DATA_ERROR;
    if (!rc_read_init(&s.rc, b))
    return XZ_OK;
    s.lzma2.compressed -= RC_INIT_BYTES;
    s.lzma2.sequence = SEQ_LZMA_RUN;
    dict_reset(&s.dict, b);
    }
// This is to allow increasing b->out_size between calls.
    if (DEC_IS_SINGLE(s.dict.mode))
    s.dict.end = b.out_size - b.out_pos;
    while (true) {
    dict_limit(&s.dict, min(b.out_size - b.out_pos,
    s.lzma2.uncompressed));
    if (!lzma2_lzma(s, b))
    return XZ_DATA_ERROR;
    s.lzma2.uncompressed -= dict_flush(&s.dict, b);
    if (s.lzma2.uncompressed == 0) {
    if (s.lzma2.pedantic_microlzma) {
    if (s.lzma2.compressed > 0 || s.lzma.len > 0
    || !rc_is_finished(&s.rc))
    return XZ_DATA_ERROR;
    }
    return XZ_STREAM_END;
    }
    if (b.out_pos == b.out_size)
    return XZ_OK;
    if (b.in_pos == b.in_size
    && s.temp.size < s.lzma2.compressed)
    return XZ_OK;
    }
    }
    struct xz_dec_microlzma *xz_dec_microlzma_alloc(enum xz_mode mode,
    uint32_t dict_size)
    {
    struct xz_dec_microlzma *s;
// Restrict dict_size to the same range as in the LZMA2 code.
    if (dict_size < 4096 || dict_size > (3U << 30))
    return core::ptr::null_mut();
    s = kmalloc_obj(*s);
    if (s == core::ptr::null_mut())
    return core::ptr::null_mut();
    s.s.dict.mode = mode;
    s.s.dict.size = dict_size;
    if (DEC_IS_MULTI(mode)) {
    s.s.dict.end = dict_size;
    s.s.dict.buf = vmalloc(dict_size);
    if (s.s.dict.buf == core::ptr::null_mut()) {
    kfree(s);
    return core::ptr::null_mut();
    }
    }
    return s;
    }
    void xz_dec_microlzma_reset(struct xz_dec_microlzma *s, uint32_t comp_size,
    uint32_t uncomp_size, int uncomp_size_is_exact)
    {
//
// comp_size is validated in xz_dec_microlzma_run().
// uncomp_size can safely be anything.
//
    s.s.lzma2.compressed = comp_size;
    s.s.lzma2.uncompressed = uncomp_size;
    s.s.lzma2.pedantic_microlzma = uncomp_size_is_exact;
    s.s.lzma2.sequence = SEQ_PROPERTIES;
    s.s.temp.size = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn xz_dec_microlzma_end(s: *mut xz_dec_microlzma) {
    void xz_dec_microlzma_end(struct xz_dec_microlzma *s)
    {
    if (DEC_IS_MULTI(s.s.dict.mode))
    vfree(s.s.dict.buf);
    kfree(s);
    }
