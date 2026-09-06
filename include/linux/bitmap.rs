//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bitmap.h
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

//
// bitmaps provide bit arrays that consume one or more unsigned
// longs.  The bitmap interface and available operations are listed
// here, in bitmap.h
//
// Function implementations generic to all architectures are in
// lib/bitmap.c.  Functions implementations that are architecture
// specific are in various arch/<arch>/include/asm/bitops.h headers
// and other arch/<arch> specific files.
//
// See lib/bitmap.c for more details.
//
// DOC: bitmap overview
//
// The available bitmap operations and their rough meaning in the
// case that the bitmap is a single unsigned long are thus:
//
// The generated code is more efficient when nbits is known at
// compile-time and at most BITS_PER_LONG.
//
// ::
//
// bitmap_zero(dst, nbits)                     *dst = 0UL
// bitmap_fill(dst, nbits)                     *dst = ~0UL
// bitmap_copy(dst, src, nbits)                *dst = *src
// bitmap_and(dst, src1, src2, nbits)          *dst = *src1 & *src2
// bitmap_or(dst, src1, src2, nbits)           *dst = *src1 | *src2
// bitmap_weighted_or(dst, src1, src2, nbits)	*dst = *src1 | *src2. Returns Hamming Weight of dst
// bitmap_weighted_xor(dst, src1, src2, nbits)	*dst = *src1 ^ *src2. Returns Hamming Weight of dst
// bitmap_xor(dst, src1, src2, nbits)          *dst = *src1 ^ *src2
// bitmap_andnot(dst, src1, src2, nbits)       *dst = *src1 & ~(*src2)
// bitmap_complement(dst, src, nbits)          *dst = ~(*src)
// bitmap_equal(src1, src2, nbits)             Are *src1 and *src2 equal?
// bitmap_intersects(src1, src2, nbits)        Do *src1 and *src2 overlap?
// bitmap_subset(src1, src2, nbits)            Is *src1 a subset of *src2?
// bitmap_empty(src, nbits)                    Are all bits zero in *src?
// bitmap_full(src, nbits)                     Are all bits set in *src?
// bitmap_weight(src, nbits)                   Hamming Weight: number set bits
// bitmap_weight_and(src1, src2, nbits)        Hamming Weight of and'ed bitmap
// bitmap_weight_andnot(src1, src2, nbits)     Hamming Weight of andnot'ed bitmap
// bitmap_weight_from(src, start, end)         Hamming Weight starting from @start
// bitmap_set(dst, pos, nbits)                 Set specified bit area
// bitmap_clear(dst, pos, nbits)               Clear specified bit area
// bitmap_find_next_zero_area(buf, len, pos, n, mask)  Find bit free area
// bitmap_find_next_zero_area_off(buf, len, pos, n, mask, mask_off)  as above
// bitmap_shift_right(dst, src, n, nbits)      *dst = *src >> n
// bitmap_shift_left(dst, src, n, nbits)       *dst = *src << n
// bitmap_cut(dst, src, first, n, nbits)       Cut n bits from first, copy rest
// bitmap_replace(dst, old, new, mask, nbits)  *dst = (*old & ~(*mask)) | (*new & *mask)
// bitmap_scatter(dst, src, mask, nbits)	*dst = map(dense, sparse)(src)
// bitmap_gather(dst, src, mask, nbits)	*dst = map(sparse, dense)(src)
// bitmap_remap(dst, src, old, new, nbits)     *dst = map(old, new)(src)
// bitmap_bitremap(oldbit, old, new, nbits)    newbit = map(old, new)(oldbit)
// bitmap_onto(dst, orig, relmap, nbits)       *dst = orig relative to relmap
// bitmap_fold(dst, orig, sz, nbits)           dst bits = orig bits mod sz
// bitmap_parse(buf, buflen, dst, nbits)       Parse bitmap dst from kernel buf
// bitmap_parse_user(ubuf, ulen, dst, nbits)   Parse bitmap dst from user buf
// bitmap_parselist(buf, dst, nbits)           Parse bitmap dst from kernel buf
// bitmap_parselist_user(buf, dst, nbits)      Parse bitmap dst from user buf
// bitmap_find_free_region(bitmap, bits, order)  Find and allocate bit region
// bitmap_release_region(bitmap, pos, order)   Free specified bit region
// bitmap_allocate_region(bitmap, pos, order)  Allocate specified bit region
// bitmap_from_arr32(dst, buf, nbits)          Copy nbits from u32[] buf to dst
// bitmap_from_arr64(dst, buf, nbits)          Copy nbits from u64[] buf to dst
// bitmap_to_arr32(buf, src, nbits)            Copy nbits from buf to u32[] dst
// bitmap_to_arr64(buf, src, nbits)            Copy nbits from buf to u64[] dst
// bitmap_get_value8(map, start)               Get 8bit value from map at start
// bitmap_set_value8(map, value, start)        Set 8bit value to map at start
// bitmap_read(map, start, nbits)              Read an nbits-sized value from
// map at start
// bitmap_write(map, value, start, nbits)      Write an nbits-sized value to
// map at start
//
// Note, bitmap_zero() and bitmap_fill() operate over the region of
// unsigned longs, that is, bits behind bitmap till the unsigned long
// boundary will be zeroed or filled as well. Consider to use
// bitmap_clear() or bitmap_set() to make explicit zeroing or filling
// respectively.
//
// DOC: bitmap bitops
//
// Also the following operations in asm/bitops.h apply to bitmaps.::
//
// set_bit(bit, addr)                  *addr |= bit
// clear_bit(bit, addr)                *addr &= ~bit
// change_bit(bit, addr)               *addr ^= bit
// test_bit(bit, addr)                 Is bit set in *addr?
// test_and_set_bit(bit, addr)         Set bit and return old value
// test_and_clear_bit(bit, addr)       Clear bit and return old value
// test_and_change_bit(bit, addr)      Change bit and return old value
// find_first_zero_bit(addr, nbits)    Position first zero bit in *addr
// find_first_bit(addr, nbits)         Position first set bit in *addr
// find_next_zero_bit(addr, nbits, bit)
// Position next zero bit in *addr >= bit
// find_next_bit(addr, nbits, bit)     Position next set bit in *addr >= bit
// find_next_and_bit(addr1, addr2, nbits, bit)
// Same as find_next_bit, but in
// (*addr1 & *addr2)
//
// DOC: declare bitmap
// The DECLARE_BITMAP(name,bits) macro, in linux/types.h, can be used
// to declare an array named 'name' of just enough unsigned longs to
// contain all bit positions from 0 to 'bits' - 1.
//
// Allocation and deallocation of bitmap.
// Provided in lib/bitmap.c to avoid circular dependency.
//
extern "C" {
    pub fn bitmap_free(bitmap: *const c_ulong);
}
// Managed variants of the above.
//
// lib/bitmap.c provides these functions:
//
extern "C" {
    pub fn __bitmap_weight(bitmap: *const c_ulong, nbits: c_uint) -> c_uint;
}
extern "C" {
    pub fn __bitmap_set(map: *mut c_ulong, start: c_uint, len: c_int);
}
extern "C" {
    pub fn __bitmap_clear(map: *mut c_ulong, start: c_uint, len: c_int);
}
//
// bitmap_find_next_zero_area - find a contiguous aligned zero area
// @map: The address to base the search on
// @size: The bitmap size in bits
// @start: The bitnumber to start searching at
// @nr: The number of zeroed bits we're looking for
// @align_mask: Alignment mask for zero area
//
// The @align_mask should be one less than a power of 2; the effect is that
// the bit offset of all zero areas this function finds is multiples of that
// power of 2. A @align_mask of 0 means no alignment is required.
//
// Return: The bit offset of the found area or a value >= @size
// if no area is found.
//

// dst = 0;
// dst = ~0UL;
// dst = *src;
//
// Copy bitmap and clear tail bits in last word.
//
// On 32-bit systems bitmaps are represented as u32 arrays internally. On LE64
// machines the order of hi and lo parts of numbers match the bitmap structure.
// In both cases conversion is not needed when copying data from/to arrays of
// u32. But in LE64 case, typecast in bitmap_copy_clear_tail() may lead
// to out-of-bound access. To avoid that, both LE and BE variants of 64-bit
// architectures are not using bitmap_copy_clear_tail().
//

//
// On 64-bit systems bitmaps are represented as u64 arrays internally. So,
// the conversion is not needed when copying data from/to arrays of u64.
//

extern "C" {
    pub fn bitmap_from_arr64(bitmap: *mut c_ulong, buf: *const u64, nbits: c_uint);
}
extern "C" {
    pub fn bitmap_to_arr64(buf: *mut u64, bitmap: *const c_ulong, nbits: c_uint);
}

extern "C" {
    pub fn __bitmap_and(_arg: dst, _arg: src1, _arg: src2, _arg: nbits) -> return;
}
// dst = *src1 | *src2;
extern "C" {
    pub fn hweight_long(BITMAP_LAST_WORD_MASK(nbits): *mut *mut dst &) -> return;
}
extern "C" {
    pub fn __bitmap_weighted_or(_arg: dst, _arg: src1, _arg: src2, _arg: nbits) -> return;
}
// dst = *src1 ^ *src2;
extern "C" {
    pub fn hweight_long(BITMAP_LAST_WORD_MASK(nbits): *mut *mut dst &) -> return;
}
extern "C" {
    pub fn __bitmap_weighted_xor(_arg: dst, _arg: src1, _arg: src2, _arg: nbits) -> return;
}
// dst = *src1 ^ *src2;
extern "C" {
    pub fn __bitmap_andnot(_arg: dst, _arg: src1, _arg: src2, _arg: nbits) -> return;
}
// dst = ~(*src);

pub const BITMAP_MEM_ALIGNMENT: c_int = 8;

extern "C" {
    pub fn __bitmap_equal(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
//
// bitmap_or_equal - Check whether the or of two bitmaps is equal to a third
// @src1:	Pointer to bitmap 1
// @src2:	Pointer to bitmap 2 will be or'ed with bitmap 1
// @src3:	Pointer to bitmap 3. Compare to the result of *@src1 | *@src2
// @nbits:	number of bits in each of these bitmaps
//
// Returns: True if (*@src1 | *@src2) == *@src3, false otherwise
//
extern "C" {
    pub fn __bitmap_or_equal(_arg: src1, _arg: src2, _arg: src3, _arg: nbits) -> return;
}
extern "C" {
    pub fn __bitmap_intersects(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
extern "C" {
    pub fn __bitmap_subset(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
extern "C" {
    pub fn hweight_long(BITMAP_LAST_WORD_MASK(nbits): *mut *mut src &) -> return;
}
extern "C" {
    pub fn __bitmap_weight(_arg: src, _arg: nbits) -> return;
}
extern "C" {
    pub fn hweight_long(BITMAP_LAST_WORD_MASK(nbits): *mut *mut *mut src1 & src2 &) -> return;
}
extern "C" {
    pub fn __bitmap_weight_and(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
extern "C" {
    pub fn hweight_long(BITMAP_LAST_WORD_MASK(nbits): *mut *mut *mut src1 & ~(src2) &) -> return;
}
extern "C" {
    pub fn __bitmap_weight_andnot(_arg: src1, _arg: src2, _arg: nbits) -> return;
}
//
// bitmap_weight_from - Hamming weight for a memory region
// @bitmap: The base address
// @start: The bitnumber to starts weighting
// @end: the bitmap size in bits
//
// Returns the number of set bits in the region. If @start >= @end,
// return >= end.
//
extern "C" {
    pub fn hweight_long(1: *mut *mut bitmap & GENMASK(end -, _arg: start)) -> return;
}
// Opencode round_down() to not include math.h
// map |= GENMASK(start + nbits - 1, start);
// map &= ~GENMASK(start + nbits - 1, start);
// dst = (*src & BITMAP_LAST_WORD_MASK(nbits)) >> shift;
// dst = (*src << shift) & BITMAP_LAST_WORD_MASK(nbits);
// dst = (*old & ~(*mask)) | (*new & *mask);
//
// bitmap_scatter - Scatter a bitmap according to the given mask
// @dst: scattered bitmap
// @src: gathered bitmap
// @mask: mask representing bits to assign to in the scattered bitmap
// @nbits: number of bits in each of these bitmaps
//
// Scatters bitmap with sequential bits according to the given @mask.
//
// Example:
// If @src bitmap = 0x005a, with @mask = 0x1313, @dst will be 0x0302.
//
// Or in binary form
// @src			@mask			@dst
// 0000000001011010	0001001100010011	0000001100000010
//
// (Bits 0, 1, 2, 3, 4, 5 are copied to the bits 0, 1, 4, 8, 9, 12)
//
// A more 'visual' description of the operation::
//
// src:  0000000001011010
// ||||||
// +------+|||||
// |  +----+||||
// |  |+----+|||
// |  ||   +-+||
// |  ||   |  ||
// mask: ...v..vv...v..vv
// ...0..11...0..10
// dst:  0000001100000010
//
// A relationship exists between bitmap_scatter() and bitmap_gather(). See
// bitmap_gather() for the bitmap gather detailed operations. TL;DR:
// bitmap_gather() can be seen as the 'reverse' bitmap_scatter() operation.
//
// bitmap_gather - Gather a bitmap according to given mask
// @dst: gathered bitmap
// @src: scattered bitmap
// @mask: mask representing bits to extract from in the scattered bitmap
// @nbits: number of bits in each of these bitmaps
//
// Gathers bitmap with sparse bits according to the given @mask.
//
// Example:
// If @src bitmap = 0x0302, with @mask = 0x1313, @dst will be 0x001a.
//
// Or in binary form
// @src			@mask			@dst
// 0000001100000010	0001001100010011	0000000000011010
//
// (Bits 0, 1, 4, 8, 9, 12 are copied to the bits 0, 1, 2, 3, 4, 5)
//
// A more 'visual' description of the operation::
//
// mask: ...v..vv...v..vv
// src:  0000001100000010
// ^  ^^   ^   0
// |  ||   |  10
// |  ||   > 010
// |  |+--> 1010
// |  +--> 11010
// +----> 011010
// dst:  0000000000011010
//
// A relationship exists between bitmap_gather() and bitmap_scatter(). See
// bitmap_scatter() for the bitmap scatter detailed operations. TL;DR:
// bitmap_scatter() can be seen as the 'reverse' bitmap_gather() operation.
//
// Suppose scattered computed using bitmap_scatter(scattered, src, mask, n).
// The operation bitmap_gather(result, scattered, mask, n) leads to a result
// equal or equivalent to src.
//
// The result can be 'equivalent' because bitmap_scatter() and bitmap_gather()
// are not bijective.
// The result and src values are equivalent in that sense that a call to
// bitmap_scatter(res, src, mask, n) and a call to
// bitmap_scatter(res, result, mask, n) will lead to the same res value.
//
// bitmap_release_region - release allocated bitmap region
// @bitmap: array of unsigned longs corresponding to the bitmap
// @pos: beginning of bit region to release
// @order: region size (log base 2 of number of bits) to release
//
// This is the complement to __bitmap_find_free_region() and releases
// the found region (by clearing it in the bitmap).
//
// bitmap_allocate_region - allocate bitmap region
// @bitmap: array of unsigned longs corresponding to the bitmap
// @pos: beginning of bit region to allocate
// @order: region size (log base 2 of number of bits) to allocate
//
// Allocate (set bits in) a specified region of a bitmap.
//
// Returns: 0 on success, or %-EBUSY if specified region wasn't
// free (not all bits were zero).
//
// bitmap_find_free_region - find a contiguous aligned mem region
// @bitmap: array of unsigned longs corresponding to the bitmap
// @bits: number of bits in the bitmap
// @order: region size (log base 2 of number of bits) to find
//
// Find a region of free (zero) bits in a @bitmap of @bits bits and
// allocate them (set them to one).  Only consider regions of length
// a power (@order) of two, aligned to that power of two, which
// makes the search algorithm much faster.
//
// Returns: the bit offset in bitmap of the allocated region,
// or -errno on failure.
//
// BITMAP_FROM_U64() - Represent u64 value in the format suitable for bitmap.
// @n: u64 value
//
// Linux bitmaps are internally arrays of unsigned longs, i.e. 32-bit
// integers in 32-bit environment, and 64-bit integers in 64-bit one.
//
// There are four combinations of endianness and length of the word in linux
// ABIs: LE64, BE64, LE32 and BE32.
//
// On 64-bit kernels 64-bit LE and BE numbers are naturally ordered in
// bitmaps and therefore don't require any special handling.
//
// On 32-bit kernels 32-bit LE ABI orders lo word of 64-bit number in memory
// prior to hi, and 32-bit BE orders hi word prior to lo. The bitmap on the
// other hand is represented as an array of 32-bit words and the position of
// bit N may therefore be calculated as: word #(N/32) and bit #(N%32) in that
// word.  For example, bit #42 is located at 10th position of 2nd word.
// It matches 32-bit LE ABI, and we can simply let the compiler store 64-bit
// values in memory as it usually does. But for BE we need to swap hi and lo
// words manually.
//
// With all that, the macro BITMAP_FROM_U64() does explicit reordering of hi and
// lo parts of u64.  For LE32 it does nothing, and for BE environment it swaps
// hi and lo words, as is expected by bitmap.
//

//
// bitmap_from_u64 - Check and swap words within u64.
// @mask: source bitmap
// @dst:  destination bitmap
//
// In 32-bit Big Endian kernel, when using ``(u32 *)(&val)[*]``
// to read u64 mask, we will get the wrong word.
// That is ``(u32 *)(&val)[0]`` gets the upper 32 bits,
// but we expect the lower 32-bits of u64.
//
// bitmap_read - read a value of n-bits from the memory region
// @map: address to the bitmap memory region
// @start: bit offset of the n-bit value
// @nbits: size of value in bits, nonzero, up to BITS_PER_LONG
//
// Returns: value of @nbits bits located at the @start bit offset within the
// @map memory region. For @nbits = 0 and @nbits > BITS_PER_LONG the return
// value is undefined.
//
// bitmap_write - write n-bit value within a memory region
// @map: address to the bitmap memory region
// @value: value to write, clamped to nbits
// @start: bit offset of the n-bit value
// @nbits: size of value in bits, nonzero, up to BITS_PER_LONG.
//
// bitmap_write() behaves as-if implemented as @nbits calls of __assign_bit(),
// i.e. bits beyond @nbits are ignored:
//
// for (bit = 0; bit < nbits; bit++)
// __assign_bit(start + bit, bitmap, val & BIT(bit));
//
// For @nbits == 0 and @nbits > BITS_PER_LONG no writes are performed.
//

