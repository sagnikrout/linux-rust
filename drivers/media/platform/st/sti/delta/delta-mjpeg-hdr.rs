//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/st/sti/delta/delta-mjpeg-hdr.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics SA 2013
// Author: Hugues Fruchet <hugues.fruchet@st.com> for STMicroelectronics.
//

pub const MJPEG_SOF_0: c_uint = 0xc0;
pub const MJPEG_SOF_1: c_uint = 0xc1;
pub const MJPEG_SOI: c_uint = 0xd8;
pub const MJPEG_MARKER: c_uint = 0xff;
    static char *header_str(struct mjpeg_header *header,
    char *str,
    unsigned int len)
    {
    char *cur = str;
    let mut left: c_uint = len;
    if (!header)
    return "";
    snprintf(cur, left, "[MJPEG header]\n"
    "|- length     = %d\n"
    "|- precision  = %d\n"
    "|- width      = %d\n"
    "|- height     = %d\n"
    "|- components = %d\n",
    header.length,
    header.sample_precision,
    header.frame_width,
    header.frame_height,
    header.nb_of_components);
    return str;
    }
    static int delta_mjpeg_read_sof(struct delta_ctx *pctx,
    unsigned char *data, unsigned int size,
    struct mjpeg_header *header)
    {
    struct delta_dev *delta = pctx.dev;
    let mut offset: c_uint = 0;
    if (size < 64)
    goto err_no_more;
    memset(header, 0, sizeof(*header));
    header.length           = be16_to_cpu(*(__be16 *)(data + offset));
    offset += sizeof(u16);
    header.sample_precision = *(u8 *)(data + offset);
    offset += sizeof(u8);
    header.frame_height     = be16_to_cpu(*(__be16 *)(data + offset));
    offset += sizeof(u16);
    header.frame_width      = be16_to_cpu(*(__be16 *)(data + offset));
    offset += sizeof(u16);
    header.nb_of_components = *(u8 *)(data + offset);
    offset += sizeof(u8);
    if (header.nb_of_components >= MJPEG_MAX_COMPONENTS) {
    dev_err(delta.dev,
    "%s   unsupported number of components (%d > %d)\n",
    pctx.name, header.nb_of_components,
    MJPEG_MAX_COMPONENTS);
    return -EINVAL;
    }
    if ((offset + header.nb_of_components *
    sizeof(header.components[0])) > size)
    goto err_no_more;
    return 0;
    err_no_more:
    dev_err(delta.dev,
    "%s   sof: reached end of %d size input stream\n",
    pctx.name, size);
    return -ENODATA;
    }
    int delta_mjpeg_read_header(struct delta_ctx *pctx,
    unsigned char *data, unsigned int size,
    struct mjpeg_header *header,
    unsigned int *data_offset)
    {
    struct delta_dev *delta = pctx.dev;
    unsigned char str[200];
    let mut ret: c_uint = 0;
    let mut offset: c_uint = 0;
    let mut soi: c_uint = 0;
    if (size < 2)
    goto err_no_more;
    offset = 0;
    while (1) {
    if (data[offset] == MJPEG_MARKER)
    switch (data[offset + 1]) {
    case MJPEG_SOI:
    soi = 1;
// data_offset = offset;
    break;
    case MJPEG_SOF_0:
    case MJPEG_SOF_1:
    if (!soi) {
    dev_err(delta.dev,
    "%s   wrong sequence, got SOF while SOI not seen\n",
    pctx.name);
    return -EINVAL;
    }
    ret = delta_mjpeg_read_sof(pctx,
    &data[offset + 2],
    size - (offset + 2),
    header);
    if (ret)
    goto err;
    goto done;
    default:
    break;
    }
    offset++;
    if ((offset + 2) >= size)
    goto err_no_more;
    }
    done:
    dev_dbg(delta.dev,
    "%s   found header @ offset %d:\n%s", pctx.name,
// data_offset,
    header_str(header, str, sizeof(str)));
    return 0;
    err_no_more:
    dev_err(delta.dev,
    "%s   no header found within %d bytes input stream\n",
    pctx.name, size);
    return -ENODATA;
    err:
    return ret;
    }
