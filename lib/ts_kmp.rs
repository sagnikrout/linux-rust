//! Automatically rewritten from C to Rust
//! Source: lib/ts_kmp.c
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
// lib/ts_kmp.c		Knuth-Morris-Pratt text search implementation
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//
// ==========================================================================
//
// Implements a linear-time string-matching algorithm due to Knuth,
// Morris, and Pratt [1]. Their algorithm avoids the explicit
// computation of the transition function DELTA altogether. Its
// matching time is O(n), for n being length(text), using just an
// auxiliary function PI[1..m], for m being length(pattern),
// precomputed from the pattern in time O(m). The array PI allows
// the transition function DELTA to be computed efficiently
// "on the fly" as needed. Roughly speaking, for any state
// "q" = 0,1,...,m and any character "a" in SIGMA, the value
// PI["q"] contains the information that is independent of "a" and
// is needed to compute DELTA("q", "a") [2]. Since the array PI
// has only m entries, whereas DELTA has O(m|SIGMA|) entries, we
// save a factor of |SIGMA| in the preprocessing time by computing
// PI rather than DELTA.
//
// [1] Cormen, Leiserson, Rivest, Stein
// Introdcution to Algorithms, 2nd Edition, MIT Press
// [2] See finite automaton theory
//

    struct ts_kmp
    {
    u8 *		pattern;
    unsigned int	pattern_len;
    unsigned int	prefix_tbl[];
    };
#[no_mangle]
unsafe extern "C" fn kmp_find(conf: *mut ts_config, state: *mut ts_state) -> c_uint {
    static unsigned int kmp_find(struct ts_config *conf, struct ts_state *state)
    {
    struct ts_kmp *kmp = ts_config_priv(conf);
    unsigned int i, q = 0, text_len, consumed = state.offset;
    const u8 *text;
    let mut icase: c_int = conf.flags & TS_IGNORECASE;
    for (;;) {
    text_len = conf.get_next_block(consumed, &text, conf, state);
    if (unlikely(text_len == 0))
    break;
    for (i = 0; i < text_len; i++) {
    while (q > 0 && kmp.pattern[q]
    != (icase ? toupper(text[i]) : text[i]))
    q = kmp.prefix_tbl[q - 1];
    if (kmp.pattern[q]
    == (icase ? toupper(text[i]) : text[i]))
    q++;
    if (unlikely(q == kmp.pattern_len)) {
    state.offset = consumed + i + 1;
    return state.offset - kmp.pattern_len;
    }
    }
    consumed += text_len;
    }
    return UINT_MAX;
    }
    static inline void compute_prefix_tbl(const u8 *pattern, unsigned int len,
    unsigned int *prefix_tbl, int flags)
    {
    unsigned int k, q;
    let mut icase: u8 = flags & TS_IGNORECASE;
    for (k = 0, q = 1; q < len; q++) {
    while (k > 0 && (icase ? toupper(pattern[k]) : pattern[k])
    != (icase ? toupper(pattern[q]) : pattern[q]))
    k = prefix_tbl[k-1];
    if ((icase ? toupper(pattern[k]) : pattern[k])
    == (icase ? toupper(pattern[q]) : pattern[q]))
    k++;
    prefix_tbl[q] = k;
    }
    }
    static struct ts_config *kmp_init(const void *pattern, unsigned int len,
    gfp_t gfp_mask, int flags)
    {
    struct ts_config *conf;
    struct ts_kmp *kmp;
    int i;
    unsigned int prefix_tbl_len;
    size_t priv_size;
// Zero-length patterns would make kmp_find() read beyond kmp->pattern.
    if (unlikely(!len))
    return ERR_PTR(-EINVAL);
//
// kmp->pattern is stored immediately after the prefix_tbl[] table.
// Reject lengths that would wrap while sizing either region.
//
    if (unlikely(check_mul_overflow(len, sizeof(*kmp.prefix_tbl),
    &prefix_tbl_len) ||
    check_add_overflow(sizeof(*kmp), (size_t)len, &priv_size) ||
    check_add_overflow(priv_size, prefix_tbl_len, &priv_size)))
    return ERR_PTR(-EINVAL);
    conf = alloc_ts_config(priv_size, gfp_mask);
    if (IS_ERR(conf))
    return conf;
    conf.flags = flags;
    kmp = ts_config_priv(conf);
    kmp.pattern_len = len;
    compute_prefix_tbl(pattern, len, kmp.prefix_tbl, flags);
    kmp.pattern = (u8 *) kmp.prefix_tbl + prefix_tbl_len;
    if (flags & TS_IGNORECASE)
    for (i = 0; i < len; i++)
    kmp.pattern[i] = toupper(((u8 *)pattern)[i]);
    else
    memcpy(kmp.pattern, pattern, len);
    return conf;
    }
    static void *kmp_get_pattern(struct ts_config *conf)
    {
    struct ts_kmp *kmp = ts_config_priv(conf);
    return kmp.pattern;
    }
#[no_mangle]
unsafe extern "C" fn kmp_get_pattern_len(conf: *mut ts_config) -> c_uint {
    static unsigned int kmp_get_pattern_len(struct ts_config *conf)
    {
    struct ts_kmp *kmp = ts_config_priv(conf);
    return kmp.pattern_len;
    }
    static struct ts_ops kmp_ops = {
    .name		  = "kmp",
    .find		  = kmp_find,
    .init		  = kmp_init,
    .get_pattern	  = kmp_get_pattern,
    .get_pattern_len  = kmp_get_pattern_len,
    .owner		  = THIS_MODULE,
    .list		  = LIST_HEAD_INIT(kmp_ops.list)
    };
#[no_mangle]
unsafe extern "C" fn init_kmp() -> int __init {
    static int __init init_kmp(void)
    {
    return textsearch_register(&kmp_ops);
    }
#[no_mangle]
unsafe extern "C" fn exit_kmp() -> void __exit {
    static void __exit exit_kmp(void)
    {
    textsearch_unregister(&kmp_ops);
    }
    MODULE_DESCRIPTION("Knuth-Morris-Pratt text search implementation");
    MODULE_LICENSE("GPL");
    module_init(init_kmp);
    module_exit(exit_kmp);
