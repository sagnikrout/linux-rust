//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/pwc/pwc-uncompress.c
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
// Linux driver for Philips webcam
    Decompression frontend.
    (C) 1999-2003 Nemosoft Unv.
    (C) 2004-2006 Luc Saillard (luc@saillard.org)
    NOTE: this version of pwc is an unofficial (modified) release of pwc & pcwx
    driver and thus may have bugs that are not present in the original version.
    Please send bug reports and support requests to <luc@saillard.org>.
    The decompression routines have been implemented by reverse-engineering the
    Nemosoft binary pwcx module. Caveat emptor.
//

#[no_mangle]
pub unsafe extern "C" fn pwc_decompress(pdev: *mut pwc_device, fbuf: *mut pwc_frame_buf) -> c_int {
    int pwc_decompress(struct pwc_device *pdev, struct pwc_frame_buf *fbuf)
    {
    int n, line, col;
    void *yuv, *image;
    u16 *src;
    u16 *dsty, *dstu, *dstv;
    image = vb2_plane_vaddr(&fbuf.vb.vb2_buf, 0);
    yuv = fbuf.data + pdev.frame_header_size;  /* Skip header */
// Raw format; that's easy...
    if (pdev.pixfmt != V4L2_PIX_FMT_YUV420)
    {
    struct pwc_raw_frame *raw_frame = image;
    raw_frame.type = cpu_to_le16(pdev.type);
    raw_frame.vbandlength = cpu_to_le16(pdev.vbandlength);
// cmd_buf is always 4 bytes, but sometimes, only the
// first 3 bytes is filled (Nala case). We can
// determine this using the type of the webcam
    memcpy(raw_frame.cmd, pdev.cmd_buf, 4);
    memcpy(raw_frame.rawframe, yuv, pdev.frame_size);
    vb2_set_plane_payload(&fbuf.vb.vb2_buf, 0,
    struct_size(raw_frame, rawframe, pdev.frame_size));
    return 0;
    }
    vb2_set_plane_payload(&fbuf.vb.vb2_buf, 0,
    pdev.width * pdev.height * 3 / 2);
    if (pdev.vbandlength == 0) {
// Uncompressed mode.
//
// We do some byte shuffling here to go from the
// native format to YUV420P.
//
    src = (u16 *)yuv;
    n = pdev.width * pdev.height;
    dsty = (u16 *)(image);
    dstu = (u16 *)(image + n);
    dstv = (u16 *)(image + n + n / 4);
    for (line = 0; line < pdev.height; line++) {
    for (col = 0; col < pdev.width; col += 4) {
// dsty++ = *src++;
    if (line & 1)
// dstv++ = *src++;
    else
// dstu++ = *src++;
    }
    }
    return 0;
    }
//
// Compressed;
// the decompressor routines will write the data in planar format
// immediately.
//
    if (DEVICE_USE_CODEC1(pdev.type)) {
// TODO & FIXME
    PWC_ERROR("This chipset is not supported for now\n");
    return -ENXIO; /* No such device or address: missing decompressor */
    } else {
    pwc_dec23_decompress(pdev, yuv, image);
    }
    return 0;
    }
