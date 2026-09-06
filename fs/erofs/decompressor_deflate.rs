//! Automatically rewritten from C to Rust
//! Source: fs/erofs/decompressor_deflate.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_erofs_deflate {
    pub next: *mut z_erofs_deflate,
    pub z: z_stream_s,
    pub bounce: [u8; PAGE_SIZE],
}

    static DEFINE_SPINLOCK(z_erofs_deflate_lock);
    static unsigned int z_erofs_deflate_nstrms, z_erofs_deflate_avail_strms;
    static struct z_erofs_deflate *z_erofs_deflate_head;
    static DECLARE_WAIT_QUEUE_HEAD(z_erofs_deflate_wq);
    module_param_named(deflate_streams, z_erofs_deflate_nstrms, uint, 0444);
#[no_mangle]
unsafe extern "C" fn z_erofs_deflate_exit() {
    static void z_erofs_deflate_exit(void)
    {
// there should be no running fs instance
    while (z_erofs_deflate_avail_strms) {
    struct z_erofs_deflate *strm;
    spin_lock(&z_erofs_deflate_lock);
    strm = z_erofs_deflate_head;
    if (!strm) {
    spin_unlock(&z_erofs_deflate_lock);
    continue;
    }
    z_erofs_deflate_head = core::ptr::null_mut();
    spin_unlock(&z_erofs_deflate_lock);
    while (strm) {
    struct z_erofs_deflate *n = strm.next;
    vfree(strm.z.workspace);
    kfree(strm);
    --z_erofs_deflate_avail_strms;
    strm = n;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn z_erofs_deflate_init() -> int __init {
    static int __init z_erofs_deflate_init(void)
    {
// by default, use # of possible CPUs instead
    if (!z_erofs_deflate_nstrms)
    z_erofs_deflate_nstrms = num_possible_cpus();
    return 0;
    }
    static int z_erofs_load_deflate_config(struct super_block *sb,
    struct erofs_super_block *dsb, void *data, int size)
    {
    struct z_erofs_deflate_cfgs *dfl = data;
    static DEFINE_MUTEX(deflate_resize_mutex);
    static bool inited;
    if (!dfl || size < sizeof(struct z_erofs_deflate_cfgs)) {
    erofs_err(sb, "invalid deflate cfgs, size=%u", size);
    return -EINVAL;
    }
    if (dfl.windowbits > MAX_WBITS) {
    erofs_err(sb, "unsupported windowbits %u", dfl.windowbits);
    return -EOPNOTSUPP;
    }
    mutex_lock(&deflate_resize_mutex);
    if (!inited) {
    for (; z_erofs_deflate_avail_strms < z_erofs_deflate_nstrms;
    ++z_erofs_deflate_avail_strms) {
    struct z_erofs_deflate *strm;
    strm = kzalloc_obj(*strm);
    if (!strm)
    goto failed;
// XXX: in-kernel zlib cannot customize windowbits
    strm.z.workspace = vmalloc(zlib_inflate_workspacesize());
    if (!strm.z.workspace) {
    kfree(strm);
    goto failed;
    }
    spin_lock(&z_erofs_deflate_lock);
    strm.next = z_erofs_deflate_head;
    z_erofs_deflate_head = strm;
    spin_unlock(&z_erofs_deflate_lock);
    }
    inited = true;
    }
    mutex_unlock(&deflate_resize_mutex);
    return 0;
    failed:
    mutex_unlock(&deflate_resize_mutex);
    z_erofs_deflate_exit();
    return -ENOMEM;
    }
    static const char *__z_erofs_deflate_decompress(struct z_erofs_decompress_req *rq,
    struct page **pgpl)
    {
    struct super_block *sb = rq.sb;
    let mut dctx: z_erofs_stream_dctx = { .rq = rq, .no = -1, .ni = 0 };
    struct z_erofs_deflate *strm;
    const char *reason;
    int zerr;
// 1. get the exact DEFLATE compressed size
    dctx.kin = kmap_local_page(*rq.in);
    reason = z_erofs_fixup_insize(rq, dctx.kin + rq.pageofs_in,
    min(rq.inputsize, sb.s_blocksize - rq.pageofs_in));
    if (reason) {
    kunmap_local(dctx.kin);
    return reason;
    }
// 2. get an available DEFLATE context
    again:
    spin_lock(&z_erofs_deflate_lock);
    strm = z_erofs_deflate_head;
    if (!strm) {
    spin_unlock(&z_erofs_deflate_lock);
    wait_event(z_erofs_deflate_wq, READ_ONCE(z_erofs_deflate_head));
    goto again;
    }
    z_erofs_deflate_head = strm.next;
    spin_unlock(&z_erofs_deflate_lock);
// 3. multi-call decompress
    zerr = zlib_inflateInit2(&strm.z, -MAX_WBITS);
    if (zerr != Z_OK) {
    reason = ERR_PTR(-EINVAL);
    goto failed_zinit;
    }
    rq.fillgaps = true;	/* DEFLATE doesn't support core::ptr::null_mut() output buffer */
    strm.z.avail_in = min(rq.inputsize, PAGE_SIZE - rq.pageofs_in);
    rq.inputsize -= strm.z.avail_in;
    strm.z.next_in = dctx.kin + rq.pageofs_in;
    strm.z.avail_out = 0;
    dctx.bounce = strm.bounce;
    while (1) {
    dctx.avail_out = strm.z.avail_out;
    dctx.inbuf_sz = strm.z.avail_in;
    reason = z_erofs_stream_switch_bufs(&dctx,
    (void **)&strm.z.next_out,
    (void **)&strm.z.next_in, pgpl);
    if (reason)
    break;
    strm.z.avail_out = dctx.avail_out;
    strm.z.avail_in = dctx.inbuf_sz;
    zerr = zlib_inflate(&strm.z, Z_SYNC_FLUSH);
    if (zerr != Z_OK || !(rq.outputsize + strm.z.avail_out)) {
    if (zerr == Z_OK && rq.partial_decoding)
    break;
    if (zerr == Z_STREAM_END && !rq.outputsize)
    break;
    reason = (zerr == Z_DATA_ERROR ?
    "corrupted compressed data" :
    "unexpected end of stream");
    break;
    }
    }
    if (zlib_inflateEnd(&strm.z) != Z_OK && !reason)
    reason = ERR_PTR(-EIO);
    if (dctx.kout)
    kunmap_local(dctx.kout);
    failed_zinit:
    kunmap_local(dctx.kin);
// 4. push back DEFLATE stream context to the global list
    spin_lock(&z_erofs_deflate_lock);
    strm.next = z_erofs_deflate_head;
    z_erofs_deflate_head = strm;
    spin_unlock(&z_erofs_deflate_lock);
    wake_up(&z_erofs_deflate_wq);
    return reason;
    }
    static const char *z_erofs_deflate_decompress(struct z_erofs_decompress_req *rq,
    struct page **pgpl)
    {

    int err;
    if (!rq.partial_decoding) {
    err = z_erofs_crypto_decompress(rq, pgpl);
    if (err != -EOPNOTSUPP)
    return ERR_PTR(err);
    }

    return __z_erofs_deflate_decompress(rq, pgpl);
    }
    const struct z_erofs_decompressor z_erofs_deflate_decomp = {
    .config = z_erofs_load_deflate_config,
    .decompress = z_erofs_deflate_decompress,
    .init = z_erofs_deflate_init,
    .exit = z_erofs_deflate_exit,
    .name = "deflate",
    };
