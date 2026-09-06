//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vboxvideo/vbox_irq.c
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2016-2017 Oracle Corporation
// This file is based on qxl_irq.c
// Copyright 2013 Red Hat Inc.
// Authors: Dave Airlie
// Alon Levy
// Michael Thayer <michael.thayer@oracle.com,
// Hans de Goede <hdegoede@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn vbox_clear_irq() {
    static void vbox_clear_irq(void)
    {
    outl((u32)~0, VGA_PORT_HGSMI_HOST);
    }
#[no_mangle]
unsafe extern "C" fn vbox_get_flags(vbox: *mut vbox_private) -> u32 {
    static u32 vbox_get_flags(struct vbox_private *vbox)
    {
    return readl(vbox.guest_heap + HOST_FLAGS_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn vbox_report_hotplug(vbox: *mut vbox_private) {
    void vbox_report_hotplug(struct vbox_private *vbox)
    {
    schedule_work(&vbox.hotplug_work);
    }
#[no_mangle]
unsafe extern "C" fn vbox_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t vbox_irq_handler(int irq, void *arg)
    {
    struct drm_device *dev = (struct drm_device *)arg;
    struct vbox_private *vbox = to_vbox_dev(dev);
    let mut host_flags: u32 = vbox_get_flags(vbox);
    if (!(host_flags & HGSMIHOSTFLAGS_IRQ))
    return IRQ_NONE;
//
// Due to a bug in the initial host implementation of hot-plug irqs,
// the hot-plug and cursor capability flags were never cleared.
// Fortunately we can tell when they would have been set by checking
// that the VSYNC flag is not set.
//
    if (host_flags &
    (HGSMIHOSTFLAGS_HOTPLUG | HGSMIHOSTFLAGS_CURSOR_CAPABILITIES) &&
    !(host_flags & HGSMIHOSTFLAGS_VSYNC))
    vbox_report_hotplug(vbox);
    vbox_clear_irq();
    return IRQ_HANDLED;
    }
//
// Check that the position hints provided by the host are suitable for GNOME
// shell (i.e. all screens disjoint and hints for all enabled screens) and if
// not replace them with default ones.  Providing valid hints improves the
// chances that we will get a known screen layout for pointer mapping.
//
#[no_mangle]
unsafe extern "C" fn validate_or_set_position_hints(vbox: *mut vbox_private) {
    static void validate_or_set_position_hints(struct vbox_private *vbox)
    {
    struct vbva_modehint *hintsi, *hintsj;
    let mut valid: bool = true;
    let mut currentx: u16 = 0;
    int i, j;
    for (i = 0; i < vbox.num_crtcs; ++i) {
    for (j = 0; j < i; ++j) {
    hintsi = &vbox.last_mode_hints[i];
    hintsj = &vbox.last_mode_hints[j];
    if (hintsi.enabled && hintsj.enabled) {
    if (hintsi.dx >= 0xffff ||
    hintsi.dy >= 0xffff ||
    hintsj.dx >= 0xffff ||
    hintsj.dy >= 0xffff ||
    (hintsi.dx <
    hintsj.dx + (hintsj.cx & 0x8fff) &&
    hintsi.dx + (hintsi.cx & 0x8fff) >
    hintsj.dx) ||
    (hintsi.dy <
    hintsj.dy + (hintsj.cy & 0x8fff) &&
    hintsi.dy + (hintsi.cy & 0x8fff) >
    hintsj.dy))
    valid = false;
    }
    }
    }
    if (!valid)
    for (i = 0; i < vbox.num_crtcs; ++i) {
    if (vbox.last_mode_hints[i].enabled) {
    vbox.last_mode_hints[i].dx = currentx;
    vbox.last_mode_hints[i].dy = 0;
    currentx +=
    vbox.last_mode_hints[i].cx & 0x8fff;
    }
    }
    }
// Query the host for the most recent video mode hints.
#[no_mangle]
unsafe extern "C" fn vbox_update_mode_hints(vbox: *mut vbox_private) {
    static void vbox_update_mode_hints(struct vbox_private *vbox)
    {
    struct drm_connector_list_iter conn_iter;
    struct drm_device *dev = &vbox.ddev;
    struct drm_connector *connector;
    struct vbox_connector *vbox_conn;
    struct vbva_modehint *hints;
    u16 flags;
    bool disconnected;
    unsigned int crtc_id;
    int ret;
    ret = hgsmi_get_mode_hints(vbox.guest_pool, vbox.num_crtcs,
    vbox.last_mode_hints);
    if (ret) {
    DRM_ERROR("vboxvideo: hgsmi_get_mode_hints failed: %d\n", ret);
    return;
    }
    validate_or_set_position_hints(vbox);
    drm_modeset_lock(&dev.mode_config.connection_mutex, core::ptr::null_mut());
    drm_connector_list_iter_begin(dev, &conn_iter);
    drm_for_each_connector_iter(connector, &conn_iter) {
    vbox_conn = to_vbox_connector(connector);
    hints = &vbox.last_mode_hints[vbox_conn.vbox_crtc.crtc_id];
    if (hints.magic != VBVAMODEHINT_MAGIC)
    continue;
    disconnected = !(hints.enabled);
    crtc_id = vbox_conn.vbox_crtc.crtc_id;
    vbox_conn.mode_hint.width = hints.cx;
    vbox_conn.mode_hint.height = hints.cy;
    vbox_conn.vbox_crtc.x_hint = hints.dx;
    vbox_conn.vbox_crtc.y_hint = hints.dy;
    vbox_conn.mode_hint.disconnected = disconnected;
    if (vbox_conn.vbox_crtc.disconnected == disconnected)
    continue;
    if (disconnected)
    flags = VBVA_SCREEN_F_ACTIVE | VBVA_SCREEN_F_DISABLED;
    else
    flags = VBVA_SCREEN_F_ACTIVE | VBVA_SCREEN_F_BLANK;
    hgsmi_process_display_info(vbox.guest_pool, crtc_id, 0, 0, 0,
    hints.cx * 4, hints.cx,
    hints.cy, 0, flags);
    vbox_conn.vbox_crtc.disconnected = disconnected;
    }
    drm_connector_list_iter_end(&conn_iter);
    drm_modeset_unlock(&dev.mode_config.connection_mutex);
    }
#[no_mangle]
unsafe extern "C" fn vbox_hotplug_worker(work: *mut work_struct) {
    static void vbox_hotplug_worker(struct work_struct *work)
    {
    struct vbox_private *vbox = container_of(work, struct vbox_private,
    hotplug_work);
    vbox_update_mode_hints(vbox);
    drm_kms_helper_hotplug_event(&vbox.ddev);
    }
#[no_mangle]
pub unsafe extern "C" fn vbox_irq_init(vbox: *mut vbox_private) -> c_int {
    int vbox_irq_init(struct vbox_private *vbox)
    {
    struct drm_device *dev = &vbox.ddev;
    struct pci_dev *pdev = to_pci_dev(dev.dev);
    INIT_WORK(&vbox.hotplug_work, vbox_hotplug_worker);
    vbox_update_mode_hints(vbox);
// PCI devices require shared interrupts.
    return request_irq(pdev.irq, vbox_irq_handler, IRQF_SHARED, dev.driver.name, dev);
    }
#[no_mangle]
pub unsafe extern "C" fn vbox_irq_fini(vbox: *mut vbox_private) {
    void vbox_irq_fini(struct vbox_private *vbox)
    {
    struct drm_device *dev = &vbox.ddev;
    struct pci_dev *pdev = to_pci_dev(dev.dev);
    free_irq(pdev.irq, dev);
    flush_work(&vbox.hotplug_work);
    }
