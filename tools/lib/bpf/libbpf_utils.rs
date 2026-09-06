//! Automatically rewritten from C to Rust
//! Source: tools/lib/bpf/libbpf_utils.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Copyright (C) 2013-2015 Alexei Starovoitov <ast@kernel.org>
// Copyright (C) 2015 Wang Nan <wangnan0@huawei.com>
// Copyright (C) 2015 Huawei Inc.
// Copyright (C) 2017 Nicira, Inc.
//

pub const ENOTSUPP: c_int = 524;

// make sure libbpf doesn't use kernel-only integer typedefs

    static const char *libbpf_strerror_table[NR_ERRNO] = {
    [ERRCODE_OFFSET(LIBELF)]	= "Something wrong in libelf",
    [ERRCODE_OFFSET(FORMAT)]	= "BPF object format invalid",
    [ERRCODE_OFFSET(KVERSION)]	= "'version' section incorrect or lost",
    [ERRCODE_OFFSET(ENDIAN)]	= "Endian mismatch",
    [ERRCODE_OFFSET(INTERNAL)]	= "Internal error in libbpf",
    [ERRCODE_OFFSET(RELOC)]		= "Relocation failed",
    [ERRCODE_OFFSET(VERIFY)]	= "Kernel verifier blocks program loading",
    [ERRCODE_OFFSET(PROG2BIG)]	= "Program too big",
    [ERRCODE_OFFSET(KVER)]		= "Incorrect kernel version",
    [ERRCODE_OFFSET(PROGTYPE)]	= "Kernel doesn't support this program type",
    [ERRCODE_OFFSET(WRNGPID)]	= "Wrong pid in netlink message",
    [ERRCODE_OFFSET(INVSEQ)]	= "Invalid netlink sequence",
    [ERRCODE_OFFSET(NLPARSE)]	= "Incorrect netlink message parsing",
    };
#[no_mangle]
pub unsafe extern "C" fn libbpf_strerror(err: c_int, buf: *mut c_char, size: usize) -> c_int {
    int libbpf_strerror(int err, char *buf, size_t size)
    {
    int ret;
    if (!buf || !size)
    return libbpf_err(-EINVAL);
    err = err > 0 ? err : -err;
    if (err < __LIBBPF_ERRNO__START) {
    ret = strerror_r(err, buf, size);
    buf[size - 1] = '\0';
    return libbpf_err_errno(ret);
    }
    if (err < __LIBBPF_ERRNO__END) {
    const char *msg;
    msg = libbpf_strerror_table[ERRNO_OFFSET(err)];
    ret = snprintf(buf, size, "%s", msg);
    buf[size - 1] = '\0';
// The length of the buf and msg is positive.
// A negative number may be returned only when the
// size exceeds INT_MAX. Not likely to appear.
//
    if (ret >= size)
    return libbpf_err(-ERANGE);
    return 0;
    }
    ret = snprintf(buf, size, "Unknown libbpf error %d", err);
    buf[size - 1] = '\0';
    if (ret >= size)
    return libbpf_err(-ERANGE);
    return libbpf_err(-ENOENT);
    }
    const char *libbpf_errstr(int err)
    {
    static __thread char buf[12];
    if (err > 0)
    err = -err;
    switch (err) {
    case -E2BIG:		return "-E2BIG";
    case -EACCES:		return "-EACCES";
    case -EADDRINUSE:	return "-EADDRINUSE";
    case -EADDRNOTAVAIL:	return "-EADDRNOTAVAIL";
    case -EAGAIN:		return "-EAGAIN";
    case -EALREADY:		return "-EALREADY";
    case -EBADF:		return "-EBADF";
    case -EBADFD:		return "-EBADFD";
    case -EBUSY:		return "-EBUSY";
    case -ECANCELED:	return "-ECANCELED";
    case -ECHILD:		return "-ECHILD";
    case -EDEADLK:		return "-EDEADLK";
    case -EDOM:		return "-EDOM";
    case -EEXIST:		return "-EEXIST";
    case -EFAULT:		return "-EFAULT";
    case -EFBIG:		return "-EFBIG";
    case -EILSEQ:		return "-EILSEQ";
    case -EINPROGRESS:	return "-EINPROGRESS";
    case -EINTR:		return "-EINTR";
    case -EINVAL:		return "-EINVAL";
    case -EIO:		return "-EIO";
    case -EISDIR:		return "-EISDIR";
    case -ELOOP:		return "-ELOOP";
    case -EMFILE:		return "-EMFILE";
    case -EMLINK:		return "-EMLINK";
    case -EMSGSIZE:		return "-EMSGSIZE";
    case -ENAMETOOLONG:	return "-ENAMETOOLONG";
    case -ENFILE:		return "-ENFILE";
    case -ENODATA:		return "-ENODATA";
    case -ENODEV:		return "-ENODEV";
    case -ENOENT:		return "-ENOENT";
    case -ENOEXEC:		return "-ENOEXEC";
    case -ENOLINK:		return "-ENOLINK";
    case -ENOMEM:		return "-ENOMEM";
    case -ENOSPC:		return "-ENOSPC";
    case -ENOTBLK:		return "-ENOTBLK";
    case -ENOTDIR:		return "-ENOTDIR";
    case -ENOTSUPP:		return "-ENOTSUPP";
    case -ENOTTY:		return "-ENOTTY";
    case -ENXIO:		return "-ENXIO";
    case -EOPNOTSUPP:	return "-EOPNOTSUPP";
    case -EOVERFLOW:	return "-EOVERFLOW";
    case -EPERM:		return "-EPERM";
    case -EPIPE:		return "-EPIPE";
    case -EPROTO:		return "-EPROTO";
    case -EPROTONOSUPPORT:	return "-EPROTONOSUPPORT";
    case -ERANGE:		return "-ERANGE";
    case -EROFS:		return "-EROFS";
    case -ESPIPE:		return "-ESPIPE";
    case -ESRCH:		return "-ESRCH";
    case -ETXTBSY:		return "-ETXTBSY";
    case -EUCLEAN:		return "-EUCLEAN";
    case -EXDEV:		return "-EXDEV";
    default:
    snprintf(buf, sizeof(buf), "%d", err);
    return buf;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_unaligned_be32(p: *const c_void) -> __u32 {
    static inline __u32 get_unaligned_be32(const void *p)
    {
    __be32 val;
    memcpy(&val, p, sizeof(val));
    return be32_to_cpu(val);
    }
#[no_mangle]
pub unsafe extern "C" fn put_unaligned_be32(val: __u32, p: *mut c_void) {
    static inline void put_unaligned_be32(__u32 val, void *p)
    {
    let mut be_val: __be32 = cpu_to_be32(val);
    memcpy(p, &be_val, sizeof(be_val));
    }
pub const SHA256_BLOCK_LENGTH: c_int = 64;

    static const __u32 sha256_K[64] = {
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    };

    {                                                                      \
    __u32 tmp = h + Sigma_1(e) + Ch(e, f, g) + sha256_K[i] + w[i]; \
    d += tmp;                                                      \
    h = tmp + Sigma_0(a) + Maj(a, b, c);                           \
    }
#[no_mangle]
unsafe extern "C" fn sha256_blocks(state[8]: __u32, data: *const __u8, nblocks: usize) {
    static void sha256_blocks(__u32 state[8], const __u8 *data, size_t nblocks)
    {
    while (nblocks--) {
    let mut a: __u32 = state[0];
    let mut b: __u32 = state[1];
    let mut c: __u32 = state[2];
    let mut d: __u32 = state[3];
    let mut e: __u32 = state[4];
    let mut f: __u32 = state[5];
    let mut g: __u32 = state[6];
    let mut h: __u32 = state[7];
    __u32 w[64];
    int i;
    for (i = 0; i < 16; i++)
    w[i] = get_unaligned_be32(&data[4 * i]);
    for (; i < ARRAY_SIZE(w); i++)
    w[i] = sigma_1(w[i - 2]) + w[i - 7] +
    sigma_0(w[i - 15]) + w[i - 16];
    for (i = 0; i < ARRAY_SIZE(w); i += 8) {
    SHA256_ROUND(i + 0, a, b, c, d, e, f, g, h);
    SHA256_ROUND(i + 1, h, a, b, c, d, e, f, g);
    SHA256_ROUND(i + 2, g, h, a, b, c, d, e, f);
    SHA256_ROUND(i + 3, f, g, h, a, b, c, d, e);
    SHA256_ROUND(i + 4, e, f, g, h, a, b, c, d);
    SHA256_ROUND(i + 5, d, e, f, g, h, a, b, c);
    SHA256_ROUND(i + 6, c, d, e, f, g, h, a, b);
    SHA256_ROUND(i + 7, b, c, d, e, f, g, h, a);
    }
    state[0] += a;
    state[1] += b;
    state[2] += c;
    state[3] += d;
    state[4] += e;
    state[5] += f;
    state[6] += g;
    state[7] += h;
    data += SHA256_BLOCK_LENGTH;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn libbpf_sha256(data: *const c_void, len: usize, out[SHA256_DIGEST_LENGTH]: __u8) {
    void libbpf_sha256(const void *data, size_t len, __u8 out[SHA256_DIGEST_LENGTH])
    {
    __u32 state[8] = { 0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19 };
    let mut bitcount: __be64 = cpu_to_be64((__u64)len * 8);
    __u8 final_data[2 * SHA256_BLOCK_LENGTH] = { 0 };
    let mut final_len: usize = len % SHA256_BLOCK_LENGTH;
    int i;
    sha256_blocks(state, data, len / SHA256_BLOCK_LENGTH);
    memcpy(final_data, data + len - final_len, final_len);
    final_data[final_len] = 0x80;
    final_len = roundup(final_len + 9, SHA256_BLOCK_LENGTH);
    memcpy(&final_data[final_len - 8], &bitcount, 8);
    sha256_blocks(state, final_data, final_len / SHA256_BLOCK_LENGTH);
    for (i = 0; i < ARRAY_SIZE(state); i++)
    put_unaligned_be32(state[i], &out[4 * i]);
    }
