//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/qxl/qxl_debugfs.c
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
// Copyright (C) 2009 Red Hat <bskeggs@redhat.com>
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors:
// Alon Levy <alevy@redhat.com>
//

    static int
    qxl_debugfs_irq_received(struct seq_file *m, void *data)
    {
    struct drm_info_node *node = (struct drm_info_node *) m.private;
    struct qxl_device *qdev = to_qxl(node.minor.dev);
    seq_printf(m, "%d\n", atomic_read(&qdev.irq_received));
    seq_printf(m, "%d\n", atomic_read(&qdev.irq_received_display));
    seq_printf(m, "%d\n", atomic_read(&qdev.irq_received_cursor));
    seq_printf(m, "%d\n", atomic_read(&qdev.irq_received_io_cmd));
    seq_printf(m, "%d\n", qdev.irq_received_error);
    return 0;
    }
    static int
    qxl_debugfs_buffers_info(struct seq_file *m, void *data)
    {
    struct drm_info_node *node = (struct drm_info_node *) m.private;
    struct qxl_device *qdev = to_qxl(node.minor.dev);
    struct qxl_bo *bo;
    list_for_each_entry(bo, &qdev.gem.objects, list) {
    struct dma_resv_iter cursor;
    struct dma_fence *fence;
    let mut rel: c_int = 0;
    dma_resv_iter_begin(&cursor, bo.tbo.base.resv,
    DMA_RESV_USAGE_BOOKKEEP);
    dma_resv_for_each_fence_unlocked(&cursor, fence) {
    if (dma_resv_iter_is_restarted(&cursor))
    rel = 0;
    ++rel;
    }
    seq_printf(m, "size %ld, pc %d, num releases %d\n",
    (unsigned long)bo.tbo.base.size,
    bo.tbo.pin_count, rel);
    }
    return 0;
    }
    static struct drm_info_list qxl_debugfs_list[] = {
    { "irq_received", qxl_debugfs_irq_received, 0, core::ptr::null_mut() },
    { "qxl_buffers", qxl_debugfs_buffers_info, 0, core::ptr::null_mut() },
    };

    void
    qxl_debugfs_init(struct drm_minor *minor)
    {

    struct qxl_device *dev = to_qxl(minor.dev);
    drm_debugfs_create_files(qxl_debugfs_list, QXL_DEBUGFS_ENTRIES,
    minor.debugfs_root, minor);
    qxl_ttm_debugfs_init(dev);

    }
    void qxl_debugfs_add_files(struct qxl_device *qdev,
    struct drm_info_list *files,
    unsigned int nfiles)
    {
    unsigned int i;
    for (i = 0; i < qdev.debugfs_count; i++) {
    if (qdev.debugfs[i].files == files) {
// Already registered
    return;
    }
    }
    i = qdev.debugfs_count + 1;
    if (i > QXL_DEBUGFS_MAX_COMPONENTS) {
    DRM_ERROR("Reached maximum number of debugfs components.\n");
    DRM_ERROR("Report so we increase QXL_DEBUGFS_MAX_COMPONENTS.\n");
    return;
    }
    qdev.debugfs[qdev.debugfs_count].files = files;
    qdev.debugfs[qdev.debugfs_count].num_files = nfiles;
    qdev.debugfs_count = i;

    drm_debugfs_create_files(files, nfiles,
    qdev.ddev.primary.debugfs_root,
    qdev.ddev.primary);

    }
