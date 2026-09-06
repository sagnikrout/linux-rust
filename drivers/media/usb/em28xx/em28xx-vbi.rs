//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/em28xx/em28xx-vbi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// em28xx-vbi.c - VBI driver for em28xx
//
// Copyright (C) 2009 Devin Heitmueller <dheitmueller@kernellabs.com>
//
// This work was sponsored by EyeMagnet Limited.

// ------------------------------------------------------------------
    static int vbi_queue_setup(struct vb2_queue *vq,
    unsigned int *nbuffers, unsigned int *nplanes,
    unsigned int sizes[], struct device *alloc_devs[])
    {
    struct em28xx *dev = vb2_get_drv_priv(vq);
    struct em28xx_v4l2 *v4l2 = dev.v4l2;
    let mut size: c_ulong = v4l2.vbi_width * v4l2.vbi_height * 2;
    if (*nbuffers < 2)
// nbuffers = 2;
    if (*nplanes) {
    if (sizes[0] < size)
    return -EINVAL;
    size = sizes[0];
    }
// nplanes = 1;
    sizes[0] = size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vbi_buffer_prepare(vb: *mut vb2_buffer) -> c_int {
    static int vbi_buffer_prepare(struct vb2_buffer *vb)
    {
    struct em28xx        *dev  = vb2_get_drv_priv(vb.vb2_queue);
    struct em28xx_v4l2   *v4l2 = dev.v4l2;
    unsigned long        size;
    size = v4l2.vbi_width * v4l2.vbi_height * 2;
    if (vb2_plane_size(vb, 0) < size) {
    dev_info(&dev.intf.dev,
    "%s data will not fit into plane (%lu < %lu)\n",
    __func__, vb2_plane_size(vb, 0), size);
    return -EINVAL;
    }
    vb2_set_plane_payload(vb, 0, size);
    return 0;
    }
    static void
    vbi_buffer_queue(struct vb2_buffer *vb)
    {
    struct vb2_v4l2_buffer *vbuf = to_vb2_v4l2_buffer(vb);
    struct em28xx *dev = vb2_get_drv_priv(vb.vb2_queue);
    struct em28xx_buffer *buf =
    container_of(vbuf, struct em28xx_buffer, vb);
    struct em28xx_dmaqueue *vbiq = &dev.vbiq;
    let mut flags: c_ulong = 0;
    buf.mem = vb2_plane_vaddr(vb, 0);
    buf.length = vb2_plane_size(vb, 0);
    spin_lock_irqsave(&dev.slock, flags);
    list_add_tail(&buf.list, &vbiq.active);
    spin_unlock_irqrestore(&dev.slock, flags);
    }
    const struct vb2_ops em28xx_vbi_qops = {
    .queue_setup    = vbi_queue_setup,
    .buf_prepare    = vbi_buffer_prepare,
    .buf_queue      = vbi_buffer_queue,
    .start_streaming = em28xx_start_analog_streaming,
    .stop_streaming = em28xx_stop_vbi_streaming,
    };
