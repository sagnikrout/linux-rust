//! Automatically rewritten from C to Rust
//! Source: kernel/module/decompress.c
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
// Copyright 2021 Google LLC.
//

#[no_mangle]
unsafe extern "C" fn module_extend_max_pages(info: *mut load_info, extent: c_uint) -> c_int {
    static int module_extend_max_pages(struct load_info *info, unsigned int extent)
    {
    struct page **new_pages;
    let mut new_max: c_uint = info.max_pages + extent;
    new_pages = kvrealloc(info.pages,
    size_mul(new_max, sizeof(*info.pages)),
    GFP_KERNEL);
    if (!new_pages)
    return -ENOMEM;
    info.pages = new_pages;
    info.max_pages = new_max;
    return 0;
    }
    static struct page *module_get_next_page(struct load_info *info)
    {
    struct page *page;
    int error;
    if (info.max_pages == info.used_pages) {
    error = module_extend_max_pages(info, info.used_pages);
    if (error)
    return ERR_PTR(error);
    }
    page = alloc_page(GFP_KERNEL | __GFP_HIGHMEM);
    if (!page)
    return ERR_PTR(-ENOMEM);
    info.pages[info.used_pages++] = page;
    return page;
    }

//
// Calculate length of the header which consists of signature, header
// flags, time stamp and operating system ID (10 bytes total), plus
// an optional filename.
//
#[no_mangle]
unsafe extern "C" fn module_gzip_header_len(buf: *const u8, size: usize) -> usize {
    static size_t module_gzip_header_len(const u8 *buf, size_t size)
    {
    const u8 signature[] = { 0x1f, 0x8b, 0x08 };
    let mut len: usize = 10;
    if (size < len || memcmp(buf, signature, sizeof(signature)))
    return 0;
    if (buf[3] & 0x08) {
    do {
//
// If we can't find the end of the file name we must
// be dealing with a corrupted file.
//
    if (len == size)
    return 0;
    } while (buf[len++] != '\0');
    }
    return len;
    }
    static ssize_t module_gzip_decompress(struct load_info *info,
    const void *buf, size_t size)
    {
    let mut s: z_stream_s = { 0 };
    let mut new_size: usize = 0;
    size_t gzip_hdr_len;
    ssize_t retval;
    int rc;
    gzip_hdr_len = module_gzip_header_len(buf, size);
    if (!gzip_hdr_len) {
    pr_err("not a gzip compressed module\n");
    return -EINVAL;
    }
    s.next_in = buf + gzip_hdr_len;
    s.avail_in = size - gzip_hdr_len;
    s.workspace = kvmalloc(zlib_inflate_workspacesize(), GFP_KERNEL);
    if (!s.workspace)
    return -ENOMEM;
    rc = zlib_inflateInit2(&s, -MAX_WBITS);
    if (rc != Z_OK) {
    pr_err("failed to initialize decompressor: %d\n", rc);
    retval = -EINVAL;
    goto out;
    }
    do {
    struct page *page = module_get_next_page(info);
    if (IS_ERR(page)) {
    retval = PTR_ERR(page);
    goto out_inflate_end;
    }
    s.next_out = kmap_local_page(page);
    s.avail_out = PAGE_SIZE;
    rc = zlib_inflate(&s, 0);
    kunmap_local(s.next_out);
    new_size += PAGE_SIZE - s.avail_out;
    } while (rc == Z_OK);
    if (rc != Z_STREAM_END) {
    pr_err("decompression failed with status %d\n", rc);
    retval = -EINVAL;
    goto out_inflate_end;
    }
    retval = new_size;
    out_inflate_end:
    zlib_inflateEnd(&s);
    out:
    kvfree(s.workspace);
    return retval;
    }

    static ssize_t module_xz_decompress(struct load_info *info,
    const void *buf, size_t size)
    {
    static const u8 signature[] = { 0xfd, '7', 'z', 'X', 'Z', 0 };
    struct xz_dec *xz_dec;
    struct xz_buf xz_buf;
    enum xz_ret xz_ret;
    let mut new_size: usize = 0;
    ssize_t retval;
    if (size < sizeof(signature) ||
    memcmp(buf, signature, sizeof(signature))) {
    pr_err("not an xz compressed module\n");
    return -EINVAL;
    }
    xz_dec = xz_dec_init(XZ_DYNALLOC, (u32)-1);
    if (!xz_dec)
    return -ENOMEM;
    xz_buf.in_size = size;
    xz_buf.in = buf;
    xz_buf.in_pos = 0;
    do {
    struct page *page = module_get_next_page(info);
    if (IS_ERR(page)) {
    retval = PTR_ERR(page);
    goto out;
    }
    xz_buf.out = kmap_local_page(page);
    xz_buf.out_pos = 0;
    xz_buf.out_size = PAGE_SIZE;
    xz_ret = xz_dec_run(xz_dec, &xz_buf);
    kunmap_local(xz_buf.out);
    new_size += xz_buf.out_pos;
    } while (xz_buf.out_pos == PAGE_SIZE && xz_ret == XZ_OK);
    if (xz_ret != XZ_STREAM_END) {
    pr_err("decompression failed with status %d\n", xz_ret);
    retval = -EINVAL;
    goto out;
    }
    retval = new_size;
    out:
    xz_dec_end(xz_dec);
    return retval;
    }

    static ssize_t module_zstd_decompress(struct load_info *info,
    const void *buf, size_t size)
    {
    static const u8 signature[] = { 0x28, 0xb5, 0x2f, 0xfd };
    ZSTD_outBuffer zstd_dec;
    ZSTD_inBuffer zstd_buf;
    zstd_frame_header header;
    size_t wksp_size;
    void *wksp = core::ptr::null_mut();
    ZSTD_DStream *dstream;
    size_t ret;
    let mut new_size: usize = 0;
    int retval;
    if (size < sizeof(signature) ||
    memcmp(buf, signature, sizeof(signature))) {
    pr_err("not a zstd compressed module\n");
    return -EINVAL;
    }
    zstd_buf.src = buf;
    zstd_buf.pos = 0;
    zstd_buf.size = size;
    ret = zstd_get_frame_header(&header, zstd_buf.src, zstd_buf.size);
    if (ret != 0) {
    pr_err("ZSTD-compressed data has an incomplete frame header\n");
    retval = -EINVAL;
    goto out;
    }
    if (header.windowSize > (1 << ZSTD_WINDOWLOG_MAX)) {
    pr_err("ZSTD-compressed data has too large a window size\n");
    retval = -EINVAL;
    goto out;
    }
    wksp_size = zstd_dstream_workspace_bound(header.windowSize);
    wksp = kvmalloc(wksp_size, GFP_KERNEL);
    if (!wksp) {
    retval = -ENOMEM;
    goto out;
    }
    dstream = zstd_init_dstream(header.windowSize, wksp, wksp_size);
    if (!dstream) {
    pr_err("Can't initialize ZSTD stream\n");
    retval = -ENOMEM;
    goto out;
    }
    do {
    struct page *page = module_get_next_page(info);
    if (IS_ERR(page)) {
    retval = PTR_ERR(page);
    goto out;
    }
    zstd_dec.dst = kmap_local_page(page);
    zstd_dec.pos = 0;
    zstd_dec.size = PAGE_SIZE;
    ret = zstd_decompress_stream(dstream, &zstd_dec, &zstd_buf);
    kunmap_local(zstd_dec.dst);
    retval = zstd_get_error_code(ret);
    if (retval)
    break;
    new_size += zstd_dec.pos;
    } while (zstd_dec.pos == PAGE_SIZE && ret != 0);
    if (retval) {
    pr_err("ZSTD-decompression failed with status %d\n", retval);
    retval = -EINVAL;
    goto out;
    }
    retval = new_size;
    out:
    kvfree(wksp);
    return retval;
    }

#[no_mangle]
pub unsafe extern "C" fn module_decompress(info: *mut load_info, buf: *const c_void, size: usize) -> c_int {
    int module_decompress(struct load_info *info, const void *buf, size_t size)
    {
    unsigned int n_pages;
    ssize_t data_size;
    int error;

    info.compressed_len = size;

//
// Start with number of pages twice as big as needed for
// compressed data.
//
    n_pages = DIV_ROUND_UP(size, PAGE_SIZE) * 2;
    error = module_extend_max_pages(info, n_pages);
    if (error)
    return error;
    data_size = MODULE_DECOMPRESS_FN(info, buf, size);
    if (data_size < 0) {
    error = data_size;
    goto err;
    }
    info.hdr = vmap(info.pages, info.used_pages, VM_MAP, PAGE_KERNEL);
    if (!info.hdr) {
    error = -ENOMEM;
    goto err;
    }
    info.len = data_size;
    return 0;
    err:
    module_decompress_cleanup(info);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn module_decompress_cleanup(info: *mut load_info) {
    void module_decompress_cleanup(struct load_info *info)
    {
    int i;
    if (info.hdr)
    vunmap(info.hdr);
    for (i = 0; i < info.used_pages; i++)
    __free_page(info.pages[i]);
    kvfree(info.pages);
    info.pages = core::ptr::null_mut();
    info.max_pages = info.used_pages = 0;
    }

    static ssize_t compression_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, __stringify(MODULE_COMPRESSION) "\n");
    }
    let mut module_compression_attr: static struct kobj_attribute = __ATTR_RO(compression);
#[no_mangle]
unsafe extern "C" fn module_decompress_sysfs_init() -> int __init {
    static int __init module_decompress_sysfs_init(void)
    {
    int error;
    error = sysfs_create_file(&module_kset.kobj,
    &module_compression_attr.attr);
    if (error)
    pr_warn("Failed to create 'compression' attribute");
    return 0;
    }
    late_initcall(module_decompress_sysfs_init);
