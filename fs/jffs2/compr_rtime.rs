//! Automatically rewritten from C to Rust
//! Source: fs/jffs2/compr_rtime.c
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by Arjan van de Ven <arjanv@redhat.com>
//
// For licensing information, see the file 'LICENCE' in this directory.
//
// Very simple lz77-ish encoder.
//
// Theory of operation: Both encoder and decoder have a list of "last
// occurrences" for every possible source-value; after sending the
// first source-byte, the second byte indicated the "run" length of
// matches
//
// The algorithm is intended to only send "whole bytes", no bit-messing.
//

// _compress returns the compressed size, -1 if bigger
    static int jffs2_rtime_compress(unsigned char *data_in,
    unsigned char *cpage_out,
    uint32_t *sourcelen, uint32_t *dstlen)
    {
    unsigned short positions[256];
    let mut outpos: c_int = 0;
    let mut pos: c_int = 0;
    if (*dstlen <= 3)
    return -1;
    memset(positions,0,sizeof(positions));
    while (pos < (*sourcelen) && outpos <= (*dstlen)-2) {
    int backpos, runlen=0;
    unsigned char value;
    value = data_in[pos];
    cpage_out[outpos++] = data_in[pos++];
    backpos = positions[value];
    positions[value]=pos;
    while ((backpos < pos) && (pos < (*sourcelen)) &&
    (data_in[pos]==data_in[backpos++]) && (runlen<255)) {
    pos++;
    runlen++;
    }
    cpage_out[outpos++] = runlen;
    }
    if (outpos >= pos) {
// We failed
    return -1;
    }
// Tell the caller how much we managed to compress, and how much space it took
// sourcelen = pos;
// dstlen = outpos;
    return 0;
    }
    static int jffs2_rtime_decompress(unsigned char *data_in,
    unsigned char *cpage_out,
    uint32_t srclen, uint32_t destlen)
    {
    unsigned short positions[256];
    let mut outpos: c_int = 0;
    let mut pos: c_int = 0;
    memset(positions,0,sizeof(positions));
    while (outpos<destlen) {
    unsigned char value;
    int backoffs;
    int repeat;
    value = data_in[pos++];
    cpage_out[outpos++] = value; /* first the verbatim copied byte */
    repeat = data_in[pos++];
    backoffs = positions[value];
    positions[value]=outpos;
    if (repeat) {
    if ((outpos + repeat) > destlen) {
    return 1;
    }
    if (backoffs + repeat >= outpos) {
    while(repeat) {
    cpage_out[outpos++] = cpage_out[backoffs++];
    repeat--;
    }
    } else {
    memcpy(&cpage_out[outpos],&cpage_out[backoffs],repeat);
    outpos+=repeat;
    }
    }
    }
    return 0;
    }
    static struct jffs2_compressor jffs2_rtime_comp = {
    .priority = JFFS2_RTIME_PRIORITY,
    .name = "rtime",
    .compr = JFFS2_COMPR_RTIME,
    .compress = &jffs2_rtime_compress,
    .decompress = &jffs2_rtime_decompress,

    .disabled = 1,

    .disabled = 0,

    };
#[no_mangle]
pub unsafe extern "C" fn jffs2_rtime_init() -> c_int {
    int jffs2_rtime_init(void)
    {
    return jffs2_register_compressor(&jffs2_rtime_comp);
    }
#[no_mangle]
pub unsafe extern "C" fn jffs2_rtime_exit() {
    void jffs2_rtime_exit(void)
    {
    jffs2_unregister_compressor(&jffs2_rtime_comp);
    }
