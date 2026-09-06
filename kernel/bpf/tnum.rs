//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/tnum.c
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
// tnum: tracked (or tristate) numbers
//
// A tnum tracks knowledge about the bits of a value.  Each bit can be either
// known (0 or 1), or unknown (x).  Arithmetic operations on tnums will
// propagate the unknown bits such that the tnum result represents all the
// possible results for possible values of the operands.
//

// A completely unknown value
    let mut tnum_unknown: tnum = { .value = 0, .mask = -1 };
#[no_mangle]
pub unsafe extern "C" fn tnum_const(value: u64) -> tnum {
    struct tnum tnum_const(u64 value)
    {
    return TNUM(value, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_range(min: u64, max: u64) -> tnum {
    struct tnum tnum_range(u64 min, u64 max)
    {
    let mut chi: u64 = min ^ max, delta;
    let mut bits: u8 = fls64(chi);
// special case, needed because 1ULL << 64 is undefined
    if (bits > 63)
    return tnum_unknown;
// e.g. if chi = 4, bits = 3, delta = (1<<3) - 1 = 7.
// if chi = 0, bits = 0, delta = (1<<0) - 1 = 0, so we return
// constant min (since min == max).
//
    delta = (1ULL << bits) - 1;
    return TNUM(min & ~delta, delta);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_lshift(a: tnum, shift: u8) -> tnum {
    struct tnum tnum_lshift(struct tnum a, u8 shift)
    {
    return TNUM(a.value << shift, a.mask << shift);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_rshift(a: tnum, shift: u8) -> tnum {
    struct tnum tnum_rshift(struct tnum a, u8 shift)
    {
    return TNUM(a.value >> shift, a.mask >> shift);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_arshift(a: tnum, min_shift: u8, insn_bitness: u8) -> tnum {
    struct tnum tnum_arshift(struct tnum a, u8 min_shift, u8 insn_bitness)
    {
// if a.value is negative, arithmetic shifting by minimum shift
// will have larger negative offset compared to more shifting.
// If a.value is nonnegative, arithmetic shifting by minimum shift
// will have larger positive offset compare to more shifting.
//
    if (insn_bitness == 32)
    return TNUM((u32)(((s32)a.value) >> min_shift),
    (u32)(((s32)a.mask)  >> min_shift));
    else
    return TNUM((s64)a.value >> min_shift,
    (s64)a.mask  >> min_shift);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_add(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_add(struct tnum a, struct tnum b)
    {
    u64 sm, sv, sigma, chi, mu;
    sm = a.mask + b.mask;
    sv = a.value + b.value;
    sigma = sm + sv;
    chi = sigma ^ sv;
    mu = chi | a.mask | b.mask;
    return TNUM(sv & ~mu, mu);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_sub(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_sub(struct tnum a, struct tnum b)
    {
    u64 dv, alpha, beta, chi, mu;
    dv = a.value - b.value;
    alpha = dv + a.mask;
    beta = dv - b.mask;
    chi = alpha ^ beta;
    mu = chi | a.mask | b.mask;
    return TNUM(dv & ~mu, mu);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_neg(a: tnum) -> tnum {
    struct tnum tnum_neg(struct tnum a)
    {
    return tnum_sub(TNUM(0, 0), a);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_and(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_and(struct tnum a, struct tnum b)
    {
    u64 alpha, beta, v;
    alpha = a.value | a.mask;
    beta = b.value | b.mask;
    v = a.value & b.value;
    return TNUM(v, alpha & beta & ~v);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_or(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_or(struct tnum a, struct tnum b)
    {
    u64 v, mu;
    v = a.value | b.value;
    mu = a.mask | b.mask;
    return TNUM(v, mu & ~v);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_xor(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_xor(struct tnum a, struct tnum b)
    {
    u64 v, mu;
    v = a.value ^ b.value;
    mu = a.mask | b.mask;
    return TNUM(v & ~mu, mu);
    }
// Perform long multiplication, iterating through the bits in a using rshift:
// - if LSB(a) is a known 0, keep current accumulator
// - if LSB(a) is a known 1, add b to current accumulator
// - if LSB(a) is unknown, take a union of the above cases.
//
// For example:
//
// acc_0:        acc_1:
//
// 11 *  ->      11 *  ->      11 *  -> union(0011, 1001) == x0x1
// x1            01            11
// ------        ------        ------
// 11            11            11
// xx            00            11
// ------        ------        ------
// ????          0011          1001
//
#[no_mangle]
pub unsafe extern "C" fn tnum_mul(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_mul(struct tnum a, struct tnum b)
    {
    let mut acc: tnum = TNUM(0, 0);
    while (a.value || a.mask) {
// LSB of tnum a is a certain 1
    if (a.value & 1)
    acc = tnum_add(acc, b);
// LSB of tnum a is uncertain
#[no_mangle]
pub unsafe extern "C" fn if(1: a.mask &) -> else {
// acc = tnum_union(acc_0, acc_1), where acc_0 and
// acc_1 are partial accumulators for cases
// LSB(a) = certain 0 and LSB(a) = certain 1.
// acc_0 = acc + 0 * b = acc.
// acc_1 = acc + 1 * b = tnum_add(acc, b).
//
    acc = tnum_union(acc, tnum_add(acc, b));
    }
// Note: no case for LSB is certain 0
    a = tnum_rshift(a, 1);
    b = tnum_lshift(b, 1);
    }
    return acc;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_overlap(a: tnum, b: tnum) -> bool {
    bool tnum_overlap(struct tnum a, struct tnum b)
    {
    u64 mu;
    mu = ~a.mask & ~b.mask;
    return (a.value & mu) == (b.value & mu);
    }
// Note that if a and b disagree - i.e. one has a 'known 1' where the other has
// a 'known 0' - this will return a 'known 1' for that bit.
//
#[no_mangle]
pub unsafe extern "C" fn tnum_intersect(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_intersect(struct tnum a, struct tnum b)
    {
    u64 v, mu;
    v = a.value | b.value;
    mu = a.mask & b.mask;
    return TNUM(v & ~mu, mu);
    }
// Returns a tnum with the uncertainty from both a and b, and in addition, new
// uncertainty at any position that a and b disagree. This represents a
// superset of the union of the concrete sets of both a and b. Despite the
// overapproximation, it is optimal.
//
#[no_mangle]
pub unsafe extern "C" fn tnum_union(a: tnum, b: tnum) -> tnum {
    struct tnum tnum_union(struct tnum a, struct tnum b)
    {
    let mut v: u64 = a.value & b.value;
    let mut mu: u64 = (a.value ^ b.value) | a.mask | b.mask;
    return TNUM(v & ~mu, mu);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_cast(a: tnum, size: u8) -> tnum {
    struct tnum tnum_cast(struct tnum a, u8 size)
    {
    a.value &= (1ULL << (size * 8)) - 1;
    a.mask &= (1ULL << (size * 8)) - 1;
    return a;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_is_aligned(a: tnum, size: u64) -> bool {
    bool tnum_is_aligned(struct tnum a, u64 size)
    {
    if (!size)
    return true;
    return !((a.value | a.mask) & (size - 1));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_in(a: tnum, b: tnum) -> bool {
    bool tnum_in(struct tnum a, struct tnum b)
    {
    if (b.mask & ~a.mask)
    return false;
    b.value &= ~a.mask;
    return a.value == b.value;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_sbin(str: *mut c_char, size: usize, a: tnum) -> c_int {
    int tnum_sbin(char *str, size_t size, struct tnum a)
    {
    size_t n;
    for (n = 64; n; n--) {
    if (n < size) {
    if (a.mask & 1)
    str[n - 1] = 'x';
#[no_mangle]
pub unsafe extern "C" fn if(1: a.value &) -> else {
    else if (a.value & 1)
    str[n - 1] = '1';
    else
    str[n - 1] = '0';
    }
    a.mask >>= 1;
    a.value >>= 1;
    }
    str[min(size - 1, (size_t)64)] = 0;
    return 64;
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_subreg(a: tnum) -> tnum {
    struct tnum tnum_subreg(struct tnum a)
    {
    return tnum_cast(a, 4);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_clear_subreg(a: tnum) -> tnum {
    struct tnum tnum_clear_subreg(struct tnum a)
    {
    return tnum_lshift(tnum_rshift(a, 32), 32);
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_with_subreg(reg: tnum, subreg: tnum) -> tnum {
    struct tnum tnum_with_subreg(struct tnum reg, struct tnum subreg)
    {
    return tnum_or(tnum_clear_subreg(reg), tnum_subreg(subreg));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_const_subreg(a: tnum, value: u32) -> tnum {
    struct tnum tnum_const_subreg(struct tnum a, u32 value)
    {
    return tnum_with_subreg(a, tnum_const(value));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_bswap16(a: tnum) -> tnum {
    struct tnum tnum_bswap16(struct tnum a)
    {
    return TNUM(swab16(a.value & 0xFFFF), swab16(a.mask & 0xFFFF));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_bswap32(a: tnum) -> tnum {
    struct tnum tnum_bswap32(struct tnum a)
    {
    return TNUM(swab32(a.value & 0xFFFFFFFF), swab32(a.mask & 0xFFFFFFFF));
    }
#[no_mangle]
pub unsafe extern "C" fn tnum_bswap64(a: tnum) -> tnum {
    struct tnum tnum_bswap64(struct tnum a)
    {
    return TNUM(swab64(a.value), swab64(a.mask));
    }
// Given tnum t, and a number z such that tmin <= z < tmax, where tmin
// is the smallest member of the t (= t.value) and tmax is the largest
// member of t (= t.value | t.mask), returns the smallest member of t
// larger than z.
//
// For example,
// t      = x11100x0
// z      = 11110001 (241)
// result = 11110010 (242)
//
// Note: if this function is called with z >= tmax, it just returns
// early with tmax; if this function is called with z < tmin, the
// algorithm already returns tmin.
//
#[no_mangle]
pub unsafe extern "C" fn tnum_step(t: tnum, z: u64) -> u64 {
    u64 tnum_step(struct tnum t, u64 z)
    {
    u64 tmax, d, carry_mask, filled, inc;
    tmax = t.value | t.mask;
// if z >= largest member of t, return largest member of t
    if (z >= tmax)
    return tmax;
// if z < smallest member of t, return smallest member of t
    if (z < t.value)
    return t.value;
//
// Let r be the result tnum member, z = t.value + d.
// Every tnum member is t.value | s for some submask s of t.mask,
// and since t.value & t.mask == 0, t.value | s == t.value + s.
// So r > z becomes s > d where d = z - t.value.
//
// Find the smallest submask s of t.mask greater than d by
// "incrementing d within the mask": fill every non-mask
// position with 1 (`filled`) so +1 ripples through the gaps,
// then keep only mask bits. `carry_mask` additionally fills
// positions below the highest non-mask 1 in d, preventing
// it from trapping the carry.
//
    d = z - t.value;
    carry_mask = (1ULL << fls64(d & ~t.mask)) - 1;
    filled = d | carry_mask | ~t.mask;
    inc = (filled + 1) & t.mask;
    return t.value | inc;
    }
