//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/au0828/au0828-vbi.c
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
    au0828-vbi.c - VBI driver for au0828
    Copyright (C) 2010 Devin Heitmueller <dheitmueller@kernellabs.com>
    This work was sponsored by GetWellNetwork Inc.
//

// ------------------------------------------------------------------
    static int vbi_queue_setup(struct vb2_queue *vq,
    unsigned int *nbuffers, unsigned int *nplanes,
    unsigned int sizes[], struct device *alloc_devs[])
    {
    struct au0828_dev *dev = vb2_get_drv_priv(vq);
    let mut size: c_ulong = dev.vbi_width * dev.vbi_height * 2;
    if (*nplanes)
    return sizes[0] < size ? -EINVAL : 0;
// nplanes = 1;
    sizes[0] = size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vbi_buffer_prepare(vb: *mut vb2_buffer) -> c_int {
    static int vbi_buffer_prepare(struct vb2_buffer *vb)
    {
    struct au0828_dev *dev = vb2_get_drv_priv(vb.vb2_queue);
    unsigned long size;
    size = dev.vbi_width * dev.vbi_height * 2;
    if (vb2_plane_size(vb, 0) < size) {
    pr_err("%s data will not fit into plane (%lu < %lu)\n",
    __func__, vb2_plane_size(vb, 0), size);
    return -EINVAL;
    }
    vb2_set_plane_payload(vb, 0, size);
    return 0;
    }
    static void
    vbi_buffer_queue(struct vb2_buffer *vb)
    {
    struct au0828_dev *dev = vb2_get_drv_priv(vb.vb2_queue);
    struct vb2_v4l2_buffer *vbuf = to_vb2_v4l2_buffer(vb);
    struct au0828_buffer *buf =
    container_of(vbuf, struct au0828_buffer, vb);
    struct au0828_dmaqueue *vbiq = &dev.vbiq;
    let mut flags: c_ulong = 0;
    buf.mem = vb2_plane_vaddr(vb, 0);
    buf.length = vb2_plane_size(vb, 0);
    spin_lock_irqsave(&dev.slock, flags);
    list_add_tail(&buf.list, &vbiq.active);
    spin_unlock_irqrestore(&dev.slock, flags);
    }
    const struct vb2_ops au0828_vbi_qops = {
    .queue_setup     = vbi_queue_setup,
    .buf_prepare     = vbi_buffer_prepare,
    .buf_queue       = vbi_buffer_queue,
    .prepare_streaming = v4l_vb2q_enable_media_source,
    .start_streaming = au0828_start_analog_streaming,
    .stop_streaming  = au0828_stop_vbi_streaming,
    };
