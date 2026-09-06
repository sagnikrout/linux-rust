//! Automatically rewritten from C to Rust
//! Source: crypto/proc.c
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
// Scatterlist Cryptographic API.
//
// Procfs information.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
//

    static void *c_start(struct seq_file *m, loff_t *pos)
    __acquires_shared(&crypto_alg_sem)
    {
    down_read(&crypto_alg_sem);
    return seq_list_start(&crypto_alg_list, *pos);
    }
    static void *c_next(struct seq_file *m, void *p, loff_t *pos)
    __must_hold_shared(&crypto_alg_sem)
    {
    return seq_list_next(p, &crypto_alg_list, pos);
    }
#[no_mangle]
unsafe extern "C" fn c_stop(m: *mut seq_file, p: *mut c_void) {
    static void c_stop(struct seq_file *m, void *p)
    __releases_shared(&crypto_alg_sem)
    {
    up_read(&crypto_alg_sem);
    }
#[no_mangle]
unsafe extern "C" fn c_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int c_show(struct seq_file *m, void *p)
    {
    struct crypto_alg *alg = list_entry(p, struct crypto_alg, cra_list);
    seq_printf(m, "name         : %s\n", alg.cra_name);
    seq_printf(m, "driver       : %s\n", alg.cra_driver_name);
    seq_printf(m, "module       : %s\n", module_name(alg.cra_module));
    seq_printf(m, "priority     : %d\n", alg.cra_priority);
    seq_printf(m, "refcnt       : %u\n", refcount_read(&alg.cra_refcnt));
    seq_printf(m, "selftest     : %s\n",
    (alg.cra_flags & CRYPTO_ALG_TESTED) ?
    "passed" : "unknown");
    seq_printf(m, "internal     : %s\n",
    str_yes_no(alg.cra_flags & CRYPTO_ALG_INTERNAL));
    if (fips_enabled)
    seq_printf(m, "fips         : %s\n",
    str_no_yes(alg.cra_flags & CRYPTO_ALG_FIPS_INTERNAL));
    if (alg.cra_flags & CRYPTO_ALG_LARVAL) {
    seq_printf(m, "type         : larval\n");
    seq_printf(m, "flags        : 0x%x\n", alg.cra_flags);
    goto out;
    }
    if (alg.cra_type && alg.cra_type.show) {
    alg.cra_type.show(m, alg);
    goto out;
    }
    switch (alg.cra_flags & CRYPTO_ALG_TYPE_MASK) {
    case CRYPTO_ALG_TYPE_CIPHER:
    seq_printf(m, "type         : cipher\n");
    seq_printf(m, "blocksize    : %u\n", alg.cra_blocksize);
    seq_printf(m, "min keysize  : %u\n",
    alg.cra_cipher.cia_min_keysize);
    seq_printf(m, "max keysize  : %u\n",
    alg.cra_cipher.cia_max_keysize);
    break;
    default:
    seq_printf(m, "type         : unknown\n");
    break;
    }
    out:
    seq_putc(m, '\n');
    return 0;
    }
    static const struct seq_operations crypto_seq_ops = {
    .start		= c_start,
    .next		= c_next,
    .stop		= c_stop,
    .show		= c_show
    };
#[no_mangle]
pub unsafe extern "C" fn crypto_init_proc() -> void __init {
    void __init crypto_init_proc(void)
    {
    proc_create_seq("crypto", 0, core::ptr::null_mut(), &crypto_seq_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_exit_proc() -> void __exit {
    void __exit crypto_exit_proc(void)
    {
    remove_proc_entry("crypto", core::ptr::null_mut());
    }
