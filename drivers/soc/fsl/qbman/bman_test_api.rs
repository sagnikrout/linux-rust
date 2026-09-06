//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qbman/bman_test_api.c
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


// Copyright 2008 - 2016 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

pub const NUM_BUFS: c_int = 93;
pub const LOOPS: c_int = 3;
pub const BMAN_TOKEN_MASK: c_uint = 0x00FFFFFFFFFFLLU;
    static struct bman_pool *pool;
    static struct bm_buffer bufs_in[NUM_BUFS] ____cacheline_aligned;
    static struct bm_buffer bufs_out[NUM_BUFS] ____cacheline_aligned;
    static int bufs_received;
#[no_mangle]
unsafe extern "C" fn bufs_init() {
    static void bufs_init(void)
    {
    int i;
    for (i = 0; i < NUM_BUFS; i++)
    bm_buffer_set64(&bufs_in[i], 0xfedc01234567LLU * i);
    bufs_received = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bufs_cmp(a: *const bm_buffer, b: *const bm_buffer) -> c_int {
    static inline int bufs_cmp(const struct bm_buffer *a, const struct bm_buffer *b)
    {
    if (bman_ip_rev == BMAN_REV20 || bman_ip_rev == BMAN_REV21) {
//
// On SoCs with BMan revison 2.0, BMan only respects the 40
// LS-bits of buffer addresses, masking off the upper 8-bits on
// release commands. The API provides for 48-bit addresses
// because some SoCs support all 48-bits. When generating
// garbage addresses for testing, we either need to zero the
// upper 8-bits when releasing to BMan (otherwise we'll be
// disappointed when the buffers we acquire back from BMan
// don't match), or we need to mask the upper 8-bits off when
// comparing. We do the latter.
//
    if ((bm_buffer_get64(a) & BMAN_TOKEN_MASK) <
    (bm_buffer_get64(b) & BMAN_TOKEN_MASK))
    return -1;
    if ((bm_buffer_get64(a) & BMAN_TOKEN_MASK) >
    (bm_buffer_get64(b) & BMAN_TOKEN_MASK))
    return 1;
    } else {
    if (bm_buffer_get64(a) < bm_buffer_get64(b))
    return -1;
    if (bm_buffer_get64(a) > bm_buffer_get64(b))
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bufs_confirm() {
    static void bufs_confirm(void)
    {
    int i, j;
    for (i = 0; i < NUM_BUFS; i++) {
    let mut matches: c_int = 0;
    for (j = 0; j < NUM_BUFS; j++)
    if (!bufs_cmp(&bufs_in[i], &bufs_out[j]))
    matches++;
    WARN_ON(matches != 1);
    }
    }
// test
#[no_mangle]
pub unsafe extern "C" fn bman_test_api() {
    void bman_test_api(void)
    {
    int i, loops = LOOPS;
    bufs_init();
    pr_info("%s(): Starting\n", __func__);
    pool = bman_new_pool();
    if (!pool) {
    pr_crit("bman_new_pool() failed\n");
    goto failed;
    }
// Release buffers
    do_loop:
    i = 0;
    while (i < NUM_BUFS) {
    let mut num: c_int = 8;
    if (i + num > NUM_BUFS)
    num = NUM_BUFS - i;
    if (bman_release(pool, bufs_in + i, num)) {
    pr_crit("bman_release() failed\n");
    goto failed;
    }
    i += num;
    }
// Acquire buffers
    while (i > 0) {
    int tmp, num = 8;
    if (num > i)
    num = i;
    tmp = bman_acquire(pool, bufs_out + i - num, num);
    WARN_ON(tmp != num);
    i -= num;
    }
    i = bman_acquire(pool, core::ptr::null_mut(), 1);
    WARN_ON(i > 0);
    bufs_confirm();
    if (--loops)
    goto do_loop;
// Clean up
    bman_free_pool(pool);
    pr_info("%s(): Finished\n", __func__);
    return;
    failed:
    WARN_ON(1);
    }
