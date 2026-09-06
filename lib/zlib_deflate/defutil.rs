//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zlib_deflate/defutil.h
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


// Macro flag: #define Trace(dummy)
// Macro flag: #define Tracev(dummy)

// Macro flag: #define Tracevv(dummy)
pub const LENGTH_CODES: c_int = 29;
// number of length codes, not counting the special END_BLOCK code
pub const LITERALS: c_int = 256;
// number of literal bytes 0..255

// number of Literal or Length codes, including the END_BLOCK code
pub const D_CODES: c_int = 30;
// number of distance codes
pub const BL_CODES: c_int = 19;
// number of codes used to transfer the bit lengths

// maximum heap size
pub const MAX_BITS: c_int = 15;
// All codes must not exceed MAX_BITS bits
pub const INIT_STATE: c_int = 42;
pub const BUSY_STATE: c_int = 113;
pub const FINISH_STATE: c_int = 666;
// Stream status
// Data structure describing a single value and its code string.

pub type static_tree_desc = static_tree_desc_s;
pub type Pos = ush;
pub type IPos = unsigned;
// A Pos is an index in the character window. We use short instead of int to
// save space in the various tables. IPos is used only for parameter passing.
//
// used by deflate.c:
// Sliding window. Input bytes are read into the second half of the window,
// and move to the first half later to keep a dictionary of at least wSize
// bytes. With this organization, matches are limited to a distance of
// wSize-MAX_MATCH bytes, but this ensures that IO is always
// performed with a length multiple of the block size. Also, it limits
// the window size to 64K, which is quite useful on MSDOS.
// To do: use the user input buffer as sliding window.
//
// Actual size of window: 2*wSize, except when the user input buffer
// is directly used as sliding window.
//
// Link to older string with same hash index. To limit the size of this
// array to 64K, this link is maintained only for the last 32K strings.
// An index in this array is thus a window index modulo 32K.
//
// Number of bits by which ins_h must be shifted at each input
// step. It must be such that after MIN_MATCH steps, the oldest
// byte no longer takes part in the hash key, that is:
// hash_shift * MIN_MATCH >= hash_bits
//
// Window position at the beginning of the current output block. Gets
// negative when the window is moved backwards.
//
// Length of the best match at previous step. Matches not greater than this
// are discarded. This is used in the lazy match evaluation.
//
// To speed up deflation, hash chains are never searched beyond this
// length.  A higher limit improves compression ratio but degrades the
// speed.
//
// Attempt to find a better match only when the current match is strictly
// smaller than this value. This mechanism is used only for compression
// levels >= 4.
//

// Insert new strings in the hash table only if the match length is not
// greater than this length. This saves time but degrades compression.
// max_insert_length is used only for compression levels <= 3.
//
// Use a faster search when the previous match is longer than this
// used by trees.c:
// Didn't use ct_data typedef below to suppress compiler warning
// number of codes at each bit length for an optimal tree
// The sons of heap[n] are heap[2*n] and heap[2*n+1]. heap[0] is not used.
// The same heap array is used to build all trees.
//
// Depth of each subtree used as tie breaker for trees of equal frequency
//
// Size of match buffer for literals/lengths.  There are 4 reasons for
// limiting lit_bufsize to 64K:
// - frequencies can be kept in 16 bit counters
// - if compression is not successful for the first block, all input
// data is still in the window so we can still emit a stored block even
// when input comes from standard input.  (This can also be done for
// all blocks if lit_bufsize is not greater than 32K.)
// - if compression is not successful for a file smaller than 64K, we can
// even emit a stored file instead of a stored block (saving 5 bytes).
// This is applicable only for zip (not gzip or zlib).
// - creating new Huffman trees less frequently may not provide fast
// adaptation to changes in the input data statistics. (Take for
// example a binary file with poorly compressible code followed by
// a highly compressible string table.) Smaller buffer sizes give
// fast adaptation but have of course the overhead of transmitting
// trees more frequently.
// - I can't count above 4
//
// Buffer for distances. To simplify the code, d_buf and l_buf have
// the same number of elements. To use different lengths, an extra flag
// array would be necessary.
//

// Output buffer. bits are inserted starting at the bottom (least
// significant bits).
//
// Number of valid bits in bi_buf.  All bits above the last valid bit
// are always zero.
//

// Output a byte on the stream.
// IN assertion: there is enough room in pending_buf.
//

// Minimum amount of lookahead, except at the end of the input file.
// See deflate.c for comments about the MIN_MATCH+1.
//

// In order to simplify the code, particularly on 16 bit machines, match
// distances are limited to MAX_DIST instead of WSIZE.
//
// in trees.c
extern "C" {
    pub fn zlib_tr_init(s: *mut deflate_state);
}
extern "C" {
    pub fn zlib_tr_tally(s: *mut deflate_state, dist: unsigned, lc: unsigned) -> c_int;
}
extern "C" {
    pub fn zlib_tr_align(s: *mut deflate_state);
}
extern "C" {
    pub fn zlib_tr_stored_type_only(: *mut deflate_state);
}
// ===========================================================================
// Output a short LSB first on the stream.
// IN assertion: there is enough room in pendingBuf.
//

// ===========================================================================
// Reverse the first len bits of a code, using straightforward code (a faster
// method would use a table)
// IN assertion: 1 <= len <= 15
//
// ===========================================================================
// Flush the bit buffer, keeping at most 7 bits in it.
//
// ===========================================================================
// Flush the bit buffer and align the output on a byte boundary
//

// Number of bits used within bi_buf. (bi_buf might be implemented on
// more than 16 bits on some systems.)
//
// ===========================================================================
// Send a value on a given number of bits.
// IN assertion: length <= 16 and value fits in length bits.
//

extern "C" {
    pub fn send_bits(s: *mut deflate_state, value: c_int, length: c_int) -> static void;
}
// If not enough room in bi_buf, use (valid) bits from bi_buf and
// (16 - bi_valid) bits from value, leaving (width - (16-bi_valid))
// unused bits in value.
//

// =========================================================================
// Flush as much pending output as possible. All deflate() output goes
// through this function so some applications may wish to modify it
// to avoid allocating a large strm->next_out buffer and copying into it.
// (See also read_buf()).
//
