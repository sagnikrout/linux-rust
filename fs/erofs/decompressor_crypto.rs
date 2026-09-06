//! Automatically rewritten from C to Rust
//! Source: fs/erofs/decompressor_crypto.c
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

    static int __z_erofs_crypto_decompress(struct z_erofs_decompress_req *rq,
    struct crypto_acomp *tfm)
    {
    struct sg_table st_src, st_dst;
    struct acomp_req *req;
    struct crypto_wait wait;
    const char *reason;
    u8 *headpage;
    int ret;
    headpage = kmap_local_page(*rq.in);
    reason = z_erofs_fixup_insize(rq, headpage + rq.pageofs_in,
    min_t(unsigned int, rq.inputsize,
    rq.sb.s_blocksize - rq.pageofs_in));
    kunmap_local(headpage);
    if (reason)
    return IS_ERR(reason) ? PTR_ERR(reason) : -EFSCORRUPTED;
    req = acomp_request_alloc(tfm);
    if (!req)
    return -ENOMEM;
    ret = sg_alloc_table_from_pages_segment(&st_src, rq.in, rq.inpages,
    rq.pageofs_in, rq.inputsize, UINT_MAX, GFP_KERNEL);
    if (ret < 0)
    goto failed_src_alloc;
    ret = sg_alloc_table_from_pages_segment(&st_dst, rq.out, rq.outpages,
    rq.pageofs_out, rq.outputsize, UINT_MAX, GFP_KERNEL);
    if (ret < 0)
    goto failed_dst_alloc;
    acomp_request_set_params(req, st_src.sgl,
    st_dst.sgl, rq.inputsize, rq.outputsize);
    crypto_init_wait(&wait);
    acomp_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG,
    crypto_req_done, &wait);
    ret = crypto_wait_req(crypto_acomp_decompress(req), &wait);
    if (ret) {
    erofs_err(rq.sb, "failed to decompress %d in[%u, %u] out[%u]",
    ret, rq.inputsize, rq.pageofs_in, rq.outputsize);
    ret = -EIO;
    }
    sg_free_table(&st_dst);
    failed_dst_alloc:
    sg_free_table(&st_src);
    failed_src_alloc:
    acomp_request_free(req);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_crypto_engine {
    pub crypto_name: *mut c_char,
    pub tfm: *mut crypto_acomp,
}

    static struct z_erofs_crypto_engine *z_erofs_crypto[Z_EROFS_COMPRESSION_MAX] = {
    [Z_EROFS_COMPRESSION_LZ4] = (struct z_erofs_crypto_engine[]) {
    {},
    },
    [Z_EROFS_COMPRESSION_LZMA] = (struct z_erofs_crypto_engine[]) {
    {},
    },
    [Z_EROFS_COMPRESSION_DEFLATE] = (struct z_erofs_crypto_engine[]) {
    { .crypto_name = "qat_deflate", },
    {},
    },
    [Z_EROFS_COMPRESSION_ZSTD] = (struct z_erofs_crypto_engine[]) {
    {},
    },
    };
    static DECLARE_RWSEM(z_erofs_crypto_rwsem);
    static struct crypto_acomp *z_erofs_crypto_get_engine(int alg)
    {
    struct z_erofs_crypto_engine *e;
    for (e = z_erofs_crypto[alg]; e.crypto_name; ++e)
    if (e.tfm)
    return e.tfm;
    return core::ptr::null_mut();
    }
    int z_erofs_crypto_decompress(struct z_erofs_decompress_req *rq,
    struct page **pgpl)
    {
    struct crypto_acomp *tfm;
    int i, err;
    down_read(&z_erofs_crypto_rwsem);
    tfm = z_erofs_crypto_get_engine(rq.alg);
    if (!tfm) {
    err = -EOPNOTSUPP;
    goto out;
    }
    for (i = 0; i < rq.outpages; i++) {
    let mut page: *mut page const = rq.out[i];
    struct page *victim;
    if (!page) {
    victim = __erofs_allocpage(pgpl, rq.gfp, true);
    if (!victim) {
    err = -ENOMEM;
    goto out;
    }
    set_page_private(victim, Z_EROFS_SHORTLIVED_PAGE);
    rq.out[i] = victim;
    }
    }
    err = __z_erofs_crypto_decompress(rq, tfm);
    out:
    up_read(&z_erofs_crypto_rwsem);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_enable_engine(name: *const c_char, len: c_int) -> c_int {
    int z_erofs_crypto_enable_engine(const char *name, int len)
    {
    struct z_erofs_crypto_engine *e;
    struct crypto_acomp *tfm;
    int alg;
    down_write(&z_erofs_crypto_rwsem);
    for (alg = 0; alg < Z_EROFS_COMPRESSION_MAX; ++alg) {
    for (e = z_erofs_crypto[alg]; e.crypto_name; ++e) {
    if (!strncmp(name, e.crypto_name, len)) {
    if (e.tfm)
    break;
    tfm = crypto_alloc_acomp(e.crypto_name, 0, 0);
    if (IS_ERR(tfm)) {
    up_write(&z_erofs_crypto_rwsem);
    return -EOPNOTSUPP;
    }
    e.tfm = tfm;
    break;
    }
    }
    }
    up_write(&z_erofs_crypto_rwsem);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_disable_all_engines() {
    void z_erofs_crypto_disable_all_engines(void)
    {
    struct z_erofs_crypto_engine *e;
    int alg;
    down_write(&z_erofs_crypto_rwsem);
    for (alg = 0; alg < Z_EROFS_COMPRESSION_MAX; ++alg) {
    for (e = z_erofs_crypto[alg]; e.crypto_name; ++e) {
    if (!e.tfm)
    continue;
    crypto_free_acomp(e.tfm);
    e.tfm = core::ptr::null_mut();
    }
    }
    up_write(&z_erofs_crypto_rwsem);
    }
#[no_mangle]
pub unsafe extern "C" fn z_erofs_crypto_show_engines(buf: *mut c_char, size: c_int, sep: c_char) -> c_int {
    int z_erofs_crypto_show_engines(char *buf, int size, char sep)
    {
    struct z_erofs_crypto_engine *e;
    int alg, len = 0;
    for (alg = 0; alg < Z_EROFS_COMPRESSION_MAX; ++alg) {
    for (e = z_erofs_crypto[alg]; e.crypto_name; ++e) {
    if (!e.tfm)
    continue;
    len += scnprintf(buf + len, size - len, "%s%c",
    e.crypto_name, sep);
    }
    }
    return len;
    }
