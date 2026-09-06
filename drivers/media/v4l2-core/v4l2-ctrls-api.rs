//! Automatically rewritten from C to Rust
//! Source: drivers/media/v4l2-core/v4l2-ctrls-api.c
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
// V4L2 controls framework uAPI implementation:
//
// Copyright (C) 2010-2021  Hans Verkuil <hverkuil@kernel.org>
//

// Internal temporary helper struct, one for each v4l2_ext_control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ctrl_helper {
// Pointer to the control reference of the master control
    pub mref: *mut v4l2_ctrl_ref,
// The control ref corresponding to the v4l2_ext_control ID field.
    pub ref: *mut v4l2_ctrl_ref,
//
// v4l2_ext_control index of the next control belonging to the
// same cluster, or 0 if there isn't any.
//
    pub next: u32,
}

//
// Helper functions to copy control payload data from kernel space to
// user space and vice versa.
//
// Helper function: copy the given control value back to the caller
    static int ptr_to_user(struct v4l2_ext_control *c,
    struct v4l2_ctrl *ctrl,
    union v4l2_ctrl_ptr ptr)
    {
    u32 len;
    if (ctrl.is_ptr && !ctrl.is_string)
    return copy_to_user(c.ptr, ptr.p_const, c.size) ?
    -EFAULT : 0;
    switch (ctrl.type) {
    case V4L2_CTRL_TYPE_STRING:
    len = strlen(ptr.p_char);
    if (c.size < len + 1) {
    c.size = ctrl.elem_size;
    return -ENOSPC;
    }
    return copy_to_user(c.string, ptr.p_char, len + 1) ?
    -EFAULT : 0;
    case V4L2_CTRL_TYPE_INTEGER64:
    c.value64 = *ptr.p_s64;
    break;
    default:
    c.value = *ptr.p_s32;
    break;
    }
    return 0;
    }
// Helper function: copy the current control value back to the caller
#[no_mangle]
unsafe extern "C" fn cur_to_user(c: *mut v4l2_ext_control, ctrl: *mut v4l2_ctrl) -> c_int {
    static int cur_to_user(struct v4l2_ext_control *c, struct v4l2_ctrl *ctrl)
    {
    return ptr_to_user(c, ctrl, ctrl.p_cur);
    }
// Helper function: copy the new control value back to the caller
    static int new_to_user(struct v4l2_ext_control *c,
    struct v4l2_ctrl *ctrl)
    {
    return ptr_to_user(c, ctrl, ctrl.p_new);
    }
// Helper function: copy the request value back to the caller
    static int req_to_user(struct v4l2_ext_control *c,
    struct v4l2_ctrl_ref *ref)
    {
    return ptr_to_user(c, ref.ctrl, ref.p_req);
    }
// Helper function: copy the initial control value back to the caller
#[no_mangle]
unsafe extern "C" fn def_to_user(c: *mut v4l2_ext_control, ctrl: *mut v4l2_ctrl) -> c_int {
    static int def_to_user(struct v4l2_ext_control *c, struct v4l2_ctrl *ctrl)
    {
    ctrl.type_ops.init(ctrl, 0, ctrl.p_new);
    return ptr_to_user(c, ctrl, ctrl.p_new);
    }
// Helper function: copy the minimum control value back to the caller
#[no_mangle]
unsafe extern "C" fn min_to_user(c: *mut v4l2_ext_control, ctrl: *mut v4l2_ctrl) -> c_int {
    static int min_to_user(struct v4l2_ext_control *c, struct v4l2_ctrl *ctrl)
    {
    ctrl.type_ops.minimum(ctrl, 0, ctrl.p_new);
    return ptr_to_user(c, ctrl, ctrl.p_new);
    }
// Helper function: copy the maximum control value back to the caller
#[no_mangle]
unsafe extern "C" fn max_to_user(c: *mut v4l2_ext_control, ctrl: *mut v4l2_ctrl) -> c_int {
    static int max_to_user(struct v4l2_ext_control *c, struct v4l2_ctrl *ctrl)
    {
    ctrl.type_ops.maximum(ctrl, 0, ctrl.p_new);
    return ptr_to_user(c, ctrl, ctrl.p_new);
    }
// Helper function: copy the caller-provider value as the new control value
#[no_mangle]
unsafe extern "C" fn user_to_new(c: *mut v4l2_ext_control, ctrl: *mut v4l2_ctrl) -> c_int {
    static int user_to_new(struct v4l2_ext_control *c, struct v4l2_ctrl *ctrl)
    {
    int ret;
    u32 size;
    ctrl.is_new = 0;
    if (ctrl.is_dyn_array &&
    c.size > ctrl.p_array_alloc_elems * ctrl.elem_size) {
    void *old = ctrl.p_array;
    void *tmp = kvzalloc(2 * c.size, GFP_KERNEL);
    if (!tmp)
    return -ENOMEM;
    memcpy(tmp, ctrl.p_new.p, ctrl.elems * ctrl.elem_size);
    memcpy(tmp + c.size, ctrl.p_cur.p, ctrl.elems * ctrl.elem_size);
    ctrl.p_new.p = tmp;
    ctrl.p_cur.p = tmp + c.size;
    ctrl.p_array = tmp;
    ctrl.p_array_alloc_elems = c.size / ctrl.elem_size;
    kvfree(old);
    }
    if (ctrl.is_ptr && !ctrl.is_string) {
    let mut elems: c_uint = c.size / ctrl.elem_size;
    if (copy_from_user(ctrl.p_new.p, c.ptr, c.size))
    return -EFAULT;
    ctrl.is_new = 1;
    if (ctrl.is_dyn_array)
    ctrl.new_elems = elems;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ctrl->is_array) -> else {
    else if (ctrl.is_array)
    ctrl.type_ops.init(ctrl, elems, ctrl.p_new);
    return 0;
    }
    switch (ctrl.type) {
    case V4L2_CTRL_TYPE_INTEGER64:
// ctrl->p_new.p_s64 = c->value64;
    break;
    case V4L2_CTRL_TYPE_STRING:
    size = c.size;
    if (size == 0)
    return -ERANGE;
    if (size > ctrl.maximum + 1)
    size = ctrl.maximum + 1;
    ret = copy_from_user(ctrl.p_new.p_char, c.string, size) ? -EFAULT : 0;
    if (!ret) {
    let mut last: c_char = ctrl.p_new.p_char[size - 1];
    ctrl.p_new.p_char[size - 1] = 0;
//
// If the string was longer than ctrl->maximum,
// then return an error.
//
    if (strlen(ctrl.p_new.p_char) == ctrl.maximum && last)
    return -ERANGE;
    ctrl.is_new = 1;
    }
    return ret;
    default:
// ctrl->p_new.p_s32 = c->value;
    break;
    }
    ctrl.is_new = 1;
    return 0;
    }
//
// VIDIOC_G/TRY/S_EXT_CTRLS implementation
//
// Some general notes on the atomic requirements of VIDIOC_G/TRY/S_EXT_CTRLS:
//
// It is not a fully atomic operation, just best-effort only. After all, if
// multiple controls have to be set through multiple i2c writes (for example)
// then some initial writes may succeed while others fail. Thus leaving the
// system in an inconsistent state. The question is how much effort you are
// willing to spend on trying to make something atomic that really isn't.
//
// From the point of view of an application the main requirement is that
// when you call VIDIOC_S_EXT_CTRLS and some values are invalid then an
// error should be returned without actually affecting any controls.
//
// If all the values are correct, then it is acceptable to just give up
// in case of low-level errors.
//
// It is important though that the application can tell when only a partial
// configuration was done. The way we do that is through the error_idx field
// of struct v4l2_ext_controls: if that is equal to the count field then no
// controls were affected. Otherwise all controls before that index were
// successful in performing their 'get' or 'set' operation, the control at
// the given index failed, and you don't know what happened with the controls
// after the failed one. Since if they were part of a control cluster they
// could have been successfully processed (if a cluster member was encountered
// at index < error_idx), they could have failed (if a cluster member was at
// error_idx), or they may not have been processed yet (if the first cluster
// member appeared after error_idx).
//
// It is all fairly theoretical, though. In practice all you can do is to
// bail out. If error_idx == count, then it is an application bug. If
// error_idx < count then it is only an application bug if the error code was
// EBUSY. That usually means that something started streaming just when you
// tried to set the controls. In all other cases it is a driver/hardware
// problem and all you can do is to retry or bail out.
//
// Note that these rules do not apply to VIDIOC_TRY_EXT_CTRLS: since that
// never modifies controls the error_idx is just set to whatever control
// has an invalid value.
//
// Prepare for the extended g/s/try functions.
// Find the controls in the control array and do some basic checks.
//
    static int prepare_ext_ctrls(struct v4l2_ctrl_handler *hdl,
    struct v4l2_ext_controls *cs,
    struct v4l2_ctrl_helper *helpers,
    struct video_device *vdev,
    bool get)
    {
    struct v4l2_ctrl_helper *h;
    let mut have_clusters: bool = false;
    u32 i;
    for (i = 0, h = helpers; i < cs.count; i++, h++) {
    struct v4l2_ext_control *c = &cs.controls[i];
    struct v4l2_ctrl_ref *ref;
    struct v4l2_ctrl *ctrl;
    let mut id: u32 = c.id & V4L2_CTRL_ID_MASK;
    cs.error_idx = i;
    if (cs.which &&
    (cs.which < V4L2_CTRL_WHICH_DEF_VAL ||
    cs.which > V4L2_CTRL_WHICH_MAX_VAL) &&
    V4L2_CTRL_ID2WHICH(id) != cs.which) {
    dprintk(vdev,
    "invalid which 0x%x or control id 0x%x\n",
    cs.which, id);
    return -EINVAL;
    }
//
// Old-style private controls are not allowed for
// extended controls.
//
    if (id >= V4L2_CID_PRIVATE_BASE) {
    dprintk(vdev,
    "old-style private controls not allowed\n");
    return -EINVAL;
    }
    ref = find_ref_lock(hdl, id);
    if (!ref) {
    dprintk(vdev, "cannot find control id 0x%x\n", id);
    return -EINVAL;
    }
    h.ref = ref;
    ctrl = ref.ctrl;
    if (ctrl.flags & V4L2_CTRL_FLAG_DISABLED) {
    dprintk(vdev, "control id 0x%x is disabled\n", id);
    return -EINVAL;
    }
    if (!(ctrl.flags & V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX) &&
    (cs.which == V4L2_CTRL_WHICH_MIN_VAL ||
    cs.which == V4L2_CTRL_WHICH_MAX_VAL)) {
    dprintk(vdev,
    "invalid which 0x%x or control id 0x%x\n",
    cs.which, id);
    return -EINVAL;
    }
    if (ctrl.cluster[0].ncontrols > 1)
    have_clusters = true;
    if (ctrl.cluster[0] != ctrl)
    ref = find_ref_lock(hdl, ctrl.cluster[0].id);
    if (ctrl.is_dyn_array) {
    let mut max_size: c_uint = ctrl.dims[0] * ctrl.elem_size;
    let mut tot_size: c_uint = ctrl.elem_size;
    if (cs.which == V4L2_CTRL_WHICH_REQUEST_VAL)
    tot_size *= ref.p_req_elems;
    else
    tot_size *= ctrl.elems;
    c.size = ctrl.elem_size * (c.size / ctrl.elem_size);
    if (get) {
    if (c.size < tot_size) {
    c.size = tot_size;
    return -ENOSPC;
    }
    c.size = tot_size;
    } else {
    if (c.size > max_size) {
    c.size = max_size;
    return -ENOSPC;
    }
    if (!c.size)
    return -EFAULT;
    }
    } else if (ctrl.is_ptr && !ctrl.is_string) {
    let mut tot_size: c_uint = ctrl.elems * ctrl.elem_size;
    if (c.size < tot_size) {
//
// In the get case the application first
// queries to obtain the size of the control.
//
    if (get) {
    c.size = tot_size;
    return -ENOSPC;
    }
    dprintk(vdev,
    "pointer control id 0x%x size too small, %d bytes but %d bytes needed\n",
    id, c.size, tot_size);
    return -EFAULT;
    }
    c.size = tot_size;
    }
// Store the ref to the master control of the cluster
    h.mref = ref;
//
// Initially set next to 0, meaning that there is no other
// control in this helper array belonging to the same
// cluster.
//
    h.next = 0;
    }
//
// We are done if there were no controls that belong to a multi-
// control cluster.
//
    if (!have_clusters)
    return 0;
//
// The code below figures out in O(n) time which controls in the list
// belong to the same cluster.
//
// This has to be done with the handler lock taken.
    mutex_lock(hdl.lock);
// First zero the helper field in the master control references
    for (i = 0; i < cs.count; i++)
    helpers[i].mref.helper = core::ptr::null_mut();
    for (i = 0, h = helpers; i < cs.count; i++, h++) {
    struct v4l2_ctrl_ref *mref = h.mref;
//
// If the mref->helper is set, then it points to an earlier
// helper that belongs to the same cluster.
//
    if (mref.helper) {
//
// Set the next field of mref->helper to the current
// index: this means that the earlier helper now
// points to the next helper in the same cluster.
//
    mref.helper.next = i;
//
// mref should be set only for the first helper in the
// cluster, clear the others.
//
    h.mref = core::ptr::null_mut();
    }
// Point the mref helper to the current helper struct.
    mref.helper = h;
    }
    mutex_unlock(hdl.lock);
    return 0;
    }
//
// Handles the corner case where cs->count == 0. It checks whether the
// specified control class exists. If that class ID is 0, then it checks
// whether there are any controls at all.
//
#[no_mangle]
unsafe extern "C" fn class_check(hdl: *mut v4l2_ctrl_handler, which: u32) -> c_int {
    static int class_check(struct v4l2_ctrl_handler *hdl, u32 which)
    {
    if (which == 0 || (which >= V4L2_CTRL_WHICH_DEF_VAL &&
    which <= V4L2_CTRL_WHICH_MAX_VAL))
    return 0;
    return find_ref_lock(hdl, which | 1) ? 0 : -EINVAL;
    }
//
// Get extended controls. Allocates the helpers array if needed.
//
// Note that v4l2_g_ext_ctrls_common() with 'which' set to
// V4L2_CTRL_WHICH_REQUEST_VAL is only called if the request was
// completed, and in that case p_req_valid is true for all controls.
//
    int v4l2_g_ext_ctrls_common(struct v4l2_ctrl_handler *hdl,
    struct v4l2_ext_controls *cs,
    struct video_device *vdev)
    {
    struct v4l2_ctrl_helper helper[4];
    struct v4l2_ctrl_helper *helpers = helper;
    int ret;
    int i, j;
    bool is_default, is_request, is_min, is_max;
    is_default = (cs.which == V4L2_CTRL_WHICH_DEF_VAL);
    is_request = (cs.which == V4L2_CTRL_WHICH_REQUEST_VAL);
    is_min = (cs.which == V4L2_CTRL_WHICH_MIN_VAL);
    is_max = (cs.which == V4L2_CTRL_WHICH_MAX_VAL);
    cs.error_idx = cs.count;
    cs.which = V4L2_CTRL_ID2WHICH(cs.which);
    if (!hdl)
    return -EINVAL;
    if (cs.count == 0)
    return class_check(hdl, cs.which);
    if (cs.count > ARRAY_SIZE(helper)) {
    helpers = kvmalloc_objs(helper[0], cs.count);
    if (!helpers)
    return -ENOMEM;
    }
    ret = prepare_ext_ctrls(hdl, cs, helpers, vdev, true);
    cs.error_idx = cs.count;
    for (i = 0; !ret && i < cs.count; i++)
    if (helpers[i].ref.ctrl.flags & V4L2_CTRL_FLAG_WRITE_ONLY)
    ret = -EACCES;
    for (i = 0; !ret && i < cs.count; i++) {
    struct v4l2_ctrl *master;
    let mut is_volatile: bool = false;
    let mut idx: u32 = i;
    if (!helpers[i].mref)
    continue;
    master = helpers[i].mref.ctrl;
    cs.error_idx = i;
    v4l2_ctrl_lock(master);
//
// g_volatile_ctrl will update the new control values.
// This makes no sense for V4L2_CTRL_WHICH_DEF_VAL,
// V4L2_CTRL_WHICH_MIN_VAL, V4L2_CTRL_WHICH_MAX_VAL and
// V4L2_CTRL_WHICH_REQUEST_VAL. In the case of requests
// it is v4l2_ctrl_request_complete() that copies the
// volatile controls at the time of request completion
// to the request, so you don't want to do that again.
//
    if (!is_default && !is_request && !is_min && !is_max &&
    ((master.flags & V4L2_CTRL_FLAG_VOLATILE) ||
    (master.has_volatiles && !is_cur_manual(master)))) {
    for (j = 0; j < master.ncontrols; j++)
    cur_to_new(master.cluster[j]);
    ret = call_op(master, g_volatile_ctrl);
    is_volatile = true;
    }
    if (ret) {
    v4l2_ctrl_unlock(master);
    break;
    }
//
// Copy the default value (if is_default is true), the
// request value (if is_request is true and p_req is valid),
// the new volatile value (if is_volatile is true) or the
// current value.
//
    do {
    struct v4l2_ctrl_ref *ref = helpers[idx].ref;
    if (is_default)
    ret = def_to_user(cs.controls + idx, ref.ctrl);
#[no_mangle]
pub unsafe extern "C" fn if(ref->p_req_array_enomem: is_request &&) -> else {
    else if (is_request && ref.p_req_array_enomem)
    ret = -ENOMEM;
#[no_mangle]
pub unsafe extern "C" fn if(ref->p_req_valid: is_request &&) -> else {
    else if (is_request && ref.p_req_valid)
    ret = req_to_user(cs.controls + idx, ref);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_min) -> else {
    else if (is_min)
    ret = min_to_user(cs.controls + idx, ref.ctrl);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_max) -> else {
    else if (is_max)
    ret = max_to_user(cs.controls + idx, ref.ctrl);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_volatile) -> else {
    else if (is_volatile)
    ret = new_to_user(cs.controls + idx, ref.ctrl);
    else
    ret = cur_to_user(cs.controls + idx, ref.ctrl);
    idx = helpers[idx].next;
    } while (!ret && idx);
    v4l2_ctrl_unlock(master);
    }
    if (cs.count > ARRAY_SIZE(helper))
    kvfree(helpers);
    return ret;
    }
    int v4l2_g_ext_ctrls(struct v4l2_ctrl_handler *hdl, struct video_device *vdev,
    struct media_device *mdev, struct v4l2_ext_controls *cs)
    {
    if (cs.which == V4L2_CTRL_WHICH_REQUEST_VAL)
    return v4l2_g_ext_ctrls_request(hdl, vdev, mdev, cs);
    return v4l2_g_ext_ctrls_common(hdl, cs, vdev);
    }
    EXPORT_SYMBOL(v4l2_g_ext_ctrls);
// Validate a new control
#[no_mangle]
unsafe extern "C" fn validate_new(ctrl: *const v4l2_ctrl, p_new: union v4l2_ctrl_ptr) -> c_int {
    static int validate_new(const struct v4l2_ctrl *ctrl, union v4l2_ctrl_ptr p_new)
    {
    return ctrl.type_ops.validate(ctrl, p_new);
    }
// Validate controls.
    static int validate_ctrls(struct v4l2_ext_controls *cs,
    struct v4l2_ctrl_helper *helpers,
    struct video_device *vdev,
    bool set)
    {
    unsigned int i;
    let mut ret: c_int = 0;
    cs.error_idx = cs.count;
    for (i = 0; i < cs.count; i++) {
    struct v4l2_ctrl *ctrl = helpers[i].ref.ctrl;
    union v4l2_ctrl_ptr p_new;
    cs.error_idx = i;
    if (ctrl.flags & V4L2_CTRL_FLAG_READ_ONLY) {
    dprintk(vdev,
    "control id 0x%x is read-only\n",
    ctrl.id);
    return -EACCES;
    }
//
// This test is also done in try_set_control_cluster() which
// is called in atomic context, so that has the final say,
// but it makes sense to do an up-front check as well. Once
// an error occurs in try_set_control_cluster() some other
// controls may have been set already and we want to do a
// best-effort to avoid that.
//
    if (set && (ctrl.flags & V4L2_CTRL_FLAG_GRABBED)) {
    dprintk(vdev,
    "control id 0x%x is grabbed, cannot set\n",
    ctrl.id);
    return -EBUSY;
    }
//
// Skip validation for now if the payload needs to be copied
// from userspace into kernelspace. We'll validate those later.
//
    if (ctrl.is_ptr)
    continue;
    if (ctrl.type == V4L2_CTRL_TYPE_INTEGER64)
    p_new.p_s64 = &cs.controls[i].value64;
    else
    p_new.p_s32 = &cs.controls[i].value;
    ret = validate_new(ctrl, p_new);
    if (ret)
    return ret;
    }
    return 0;
    }
// Try or try-and-set controls
    int try_set_ext_ctrls_common(struct v4l2_fh *fh,
    struct v4l2_ctrl_handler *hdl,
    struct v4l2_ext_controls *cs,
    struct video_device *vdev, bool set)
    {
    struct v4l2_ctrl_helper helper[4];
    struct v4l2_ctrl_helper *helpers = helper;
    unsigned int i, j;
    int ret;
    cs.error_idx = cs.count;
// Default/minimum/maximum values cannot be changed
    if (cs.which == V4L2_CTRL_WHICH_DEF_VAL ||
    cs.which == V4L2_CTRL_WHICH_MIN_VAL ||
    cs.which == V4L2_CTRL_WHICH_MAX_VAL) {
    dprintk(vdev, "%s: cannot change default/min/max value\n",
    video_device_node_name(vdev));
    return -EINVAL;
    }
    cs.which = V4L2_CTRL_ID2WHICH(cs.which);
    if (!hdl) {
    dprintk(vdev, "%s: invalid null control handler\n",
    video_device_node_name(vdev));
    return -EINVAL;
    }
    if (cs.count == 0)
    return class_check(hdl, cs.which);
    if (cs.count > ARRAY_SIZE(helper)) {
    helpers = kvmalloc_objs(helper[0], cs.count);
    if (!helpers)
    return -ENOMEM;
    }
    ret = prepare_ext_ctrls(hdl, cs, helpers, vdev, false);
    if (!ret)
    ret = validate_ctrls(cs, helpers, vdev, set);
    if (ret && set)
    cs.error_idx = cs.count;
    for (i = 0; !ret && i < cs.count; i++) {
    struct v4l2_ctrl *master;
    let mut idx: u32 = i;
    if (!helpers[i].mref)
    continue;
    cs.error_idx = i;
    master = helpers[i].mref.ctrl;
    v4l2_ctrl_lock(master);
// Reset the 'is_new' flags of the cluster
    for (j = 0; j < master.ncontrols; j++)
    if (master.cluster[j])
    master.cluster[j].is_new = 0;
//
// For volatile autoclusters that are currently in auto mode
// we need to discover if it will be set to manual mode.
// If so, then we have to copy the current volatile values
// first since those will become the new manual values (which
// may be overwritten by explicit new values from this set
// of controls).
//
    if (master.is_auto && master.has_volatiles &&
    !is_cur_manual(master)) {
// Pick an initial non-manual value
    let mut new_auto_val: i32 = master.manual_mode_value + 1;
    let mut tmp_idx: u32 = idx;
    do {
//
// Check if the auto control is part of the
// list, and remember the new value.
//
    if (helpers[tmp_idx].ref.ctrl == master)
    new_auto_val = cs.controls[tmp_idx].value;
    tmp_idx = helpers[tmp_idx].next;
    } while (tmp_idx);
//
// If the new value == the manual value, then copy
// the current volatile values.
//
    if (new_auto_val == master.manual_mode_value)
    update_from_auto_cluster(master);
    }
//
// Copy the new caller-supplied control values.
// user_to_new() sets 'is_new' to 1.
//
    do {
    struct v4l2_ctrl *ctrl = helpers[idx].ref.ctrl;
    ret = user_to_new(cs.controls + idx, ctrl);
    if (!ret && ctrl.is_ptr) {
    ret = validate_new(ctrl, ctrl.p_new);
    if (ret)
    dprintk(vdev,
    "failed to validate control %s (%d)\n",
    v4l2_ctrl_get_name(ctrl.id), ret);
    }
    idx = helpers[idx].next;
    } while (!ret && idx);
    if (!ret)
    ret = try_or_set_cluster(fh, master,
    !hdl.req_obj.req && set, 0);
    if (!ret && hdl.req_obj.req && set) {
    for (j = 0; j < master.ncontrols; j++) {
    struct v4l2_ctrl_ref *ref =
    find_ref(hdl, master.cluster[j].id);
    new_to_req(ref);
    }
    }
// Copy the new values back to userspace.
    if (!ret) {
    idx = i;
    do {
    ret = new_to_user(cs.controls + idx,
    helpers[idx].ref.ctrl);
    idx = helpers[idx].next;
    } while (!ret && idx);
    }
    v4l2_ctrl_unlock(master);
    }
    if (cs.count > ARRAY_SIZE(helper))
    kvfree(helpers);
    return ret;
    }
    static int try_set_ext_ctrls(struct v4l2_fh *fh,
    struct v4l2_ctrl_handler *hdl,
    struct video_device *vdev,
    struct media_device *mdev,
    struct v4l2_ext_controls *cs, bool set)
    {
    int ret;
    if (cs.which == V4L2_CTRL_WHICH_REQUEST_VAL)
    return try_set_ext_ctrls_request(fh, hdl, vdev, mdev, cs, set);
    ret = try_set_ext_ctrls_common(fh, hdl, cs, vdev, set);
    if (ret)
    dprintk(vdev,
    "%s: try_set_ext_ctrls_common failed (%d)\n",
    video_device_node_name(vdev), ret);
    return ret;
    }
    int v4l2_try_ext_ctrls(struct v4l2_ctrl_handler *hdl,
    struct video_device *vdev,
    struct media_device *mdev,
    struct v4l2_ext_controls *cs)
    {
    return try_set_ext_ctrls(core::ptr::null_mut(), hdl, vdev, mdev, cs, false);
    }
    EXPORT_SYMBOL(v4l2_try_ext_ctrls);
    int v4l2_s_ext_ctrls(struct v4l2_fh *fh,
    struct v4l2_ctrl_handler *hdl,
    struct video_device *vdev,
    struct media_device *mdev,
    struct v4l2_ext_controls *cs)
    {
    return try_set_ext_ctrls(fh, hdl, vdev, mdev, cs, true);
    }
    EXPORT_SYMBOL(v4l2_s_ext_ctrls);
//
// VIDIOC_G/S_CTRL implementation
//
// Helper function to get a single control
#[no_mangle]
unsafe extern "C" fn get_ctrl(ctrl: *mut v4l2_ctrl, c: *mut v4l2_ext_control) -> c_int {
    static int get_ctrl(struct v4l2_ctrl *ctrl, struct v4l2_ext_control *c)
    {
    struct v4l2_ctrl *master = ctrl.cluster[0];
    let mut ret: c_int = 0;
    int i;
// Compound controls are not supported. The new_to_user() and
// cur_to_user() calls below would need to be modified not to access
// userspace memory when called from get_ctrl().
//
    if (!ctrl.is_int && ctrl.type != V4L2_CTRL_TYPE_INTEGER64)
    return -EINVAL;
    if (ctrl.flags & V4L2_CTRL_FLAG_WRITE_ONLY)
    return -EACCES;
    v4l2_ctrl_lock(master);
// g_volatile_ctrl will update the current control values
    if (ctrl.flags & V4L2_CTRL_FLAG_VOLATILE) {
    for (i = 0; i < master.ncontrols; i++)
    cur_to_new(master.cluster[i]);
    ret = call_op(master, g_volatile_ctrl);
    if (!ret)
    ret = new_to_user(c, ctrl);
    } else {
    ret = cur_to_user(c, ctrl);
    }
    v4l2_ctrl_unlock(master);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn v4l2_g_ctrl(hdl: *mut v4l2_ctrl_handler, control: *mut v4l2_control) -> c_int {
    int v4l2_g_ctrl(struct v4l2_ctrl_handler *hdl, struct v4l2_control *control)
    {
    struct v4l2_ctrl *ctrl = v4l2_ctrl_find(hdl, control.id);
    struct v4l2_ext_control c;
    int ret;
    if (!ctrl || !ctrl.is_int)
    return -EINVAL;
    ret = get_ctrl(ctrl, &c);
    if (!ret)
    control.value = c.value;
    return ret;
    }
    EXPORT_SYMBOL(v4l2_g_ctrl);
// Helper function for VIDIOC_S_CTRL compatibility
#[no_mangle]
unsafe extern "C" fn set_ctrl(fh: *mut v4l2_fh, ctrl: *mut v4l2_ctrl, ch_flags: u32) -> c_int {
    static int set_ctrl(struct v4l2_fh *fh, struct v4l2_ctrl *ctrl, u32 ch_flags)
    {
    struct v4l2_ctrl *master = ctrl.cluster[0];
    int ret;
    int i;
// Reset the 'is_new' flags of the cluster
    for (i = 0; i < master.ncontrols; i++)
    if (master.cluster[i])
    master.cluster[i].is_new = 0;
    ret = validate_new(ctrl, ctrl.p_new);
    if (ret)
    return ret;
//
// For autoclusters with volatiles that are switched from auto to
// manual mode we have to update the current volatile values since
// those will become the initial manual values after such a switch.
//
    if (master.is_auto && master.has_volatiles && ctrl == master &&
    !is_cur_manual(master) && ctrl.val == master.manual_mode_value)
    update_from_auto_cluster(master);
    ctrl.is_new = 1;
    return try_or_set_cluster(fh, master, true, ch_flags);
    }
// Helper function for VIDIOC_S_CTRL compatibility
    static int set_ctrl_lock(struct v4l2_fh *fh, struct v4l2_ctrl *ctrl,
    struct v4l2_ext_control *c)
    {
    int ret;
    v4l2_ctrl_lock(ctrl);
    ret = user_to_new(c, ctrl);
    if (!ret)
    ret = set_ctrl(fh, ctrl, 0);
    if (!ret)
    ret = cur_to_user(c, ctrl);
    v4l2_ctrl_unlock(ctrl);
    return ret;
    }
    int v4l2_s_ctrl(struct v4l2_fh *fh, struct v4l2_ctrl_handler *hdl,
    struct v4l2_control *control)
    {
    struct v4l2_ctrl *ctrl = v4l2_ctrl_find(hdl, control.id);
    let mut c: v4l2_ext_control = { control.id };
    int ret;
    if (!ctrl || !ctrl.is_int)
    return -EINVAL;
    if (ctrl.flags & V4L2_CTRL_FLAG_READ_ONLY)
    return -EACCES;
    c.value = control.value;
    ret = set_ctrl_lock(fh, ctrl, &c);
    control.value = c.value;
    return ret;
    }
    EXPORT_SYMBOL(v4l2_s_ctrl);
//
// Helper functions for drivers to get/set controls.
//
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_g_ctrl(ctrl: *mut v4l2_ctrl) -> i32 {
    s32 v4l2_ctrl_g_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_ext_control c;
// It's a driver bug if this happens.
    if (WARN_ON(!ctrl.is_int))
    return 0;
    c.value = 0;
    get_ctrl(ctrl, &c);
    return c.value;
    }
    EXPORT_SYMBOL(v4l2_ctrl_g_ctrl);
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_g_ctrl_int64(ctrl: *mut v4l2_ctrl) -> i64 {
    s64 v4l2_ctrl_g_ctrl_int64(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_ext_control c;
// It's a driver bug if this happens.
    if (WARN_ON(ctrl.is_ptr || ctrl.type != V4L2_CTRL_TYPE_INTEGER64))
    return 0;
    c.value64 = 0;
    get_ctrl(ctrl, &c);
    return c.value64;
    }
    EXPORT_SYMBOL(v4l2_ctrl_g_ctrl_int64);
#[no_mangle]
pub unsafe extern "C" fn __v4l2_ctrl_s_ctrl(ctrl: *mut v4l2_ctrl, val: i32) -> c_int {
    int __v4l2_ctrl_s_ctrl(struct v4l2_ctrl *ctrl, s32 val)
    {
    lockdep_assert_held(ctrl.handler.lock);
// It's a driver bug if this happens.
    if (WARN_ON(!ctrl.is_int))
    return -EINVAL;
    ctrl.val = val;
    return set_ctrl(core::ptr::null_mut(), ctrl, 0);
    }
    EXPORT_SYMBOL(__v4l2_ctrl_s_ctrl);
#[no_mangle]
pub unsafe extern "C" fn __v4l2_ctrl_s_ctrl_int64(ctrl: *mut v4l2_ctrl, val: i64) -> c_int {
    int __v4l2_ctrl_s_ctrl_int64(struct v4l2_ctrl *ctrl, s64 val)
    {
    lockdep_assert_held(ctrl.handler.lock);
// It's a driver bug if this happens.
    if (WARN_ON(ctrl.is_ptr || ctrl.type != V4L2_CTRL_TYPE_INTEGER64))
    return -EINVAL;
// ctrl->p_new.p_s64 = val;
    return set_ctrl(core::ptr::null_mut(), ctrl, 0);
    }
    EXPORT_SYMBOL(__v4l2_ctrl_s_ctrl_int64);
#[no_mangle]
pub unsafe extern "C" fn __v4l2_ctrl_s_ctrl_string(ctrl: *mut v4l2_ctrl, s: *const c_char) -> c_int {
    int __v4l2_ctrl_s_ctrl_string(struct v4l2_ctrl *ctrl, const char *s)
    {
    lockdep_assert_held(ctrl.handler.lock);
// It's a driver bug if this happens.
    if (WARN_ON(ctrl.type != V4L2_CTRL_TYPE_STRING))
    return -EINVAL;
    strscpy(ctrl.p_new.p_char, s, ctrl.maximum + 1);
    return set_ctrl(core::ptr::null_mut(), ctrl, 0);
    }
    EXPORT_SYMBOL(__v4l2_ctrl_s_ctrl_string);
    int __v4l2_ctrl_s_ctrl_compound(struct v4l2_ctrl *ctrl,
    enum v4l2_ctrl_type type, const void *p)
    {
    lockdep_assert_held(ctrl.handler.lock);
// It's a driver bug if this happens.
    if (WARN_ON(ctrl.type != type))
    return -EINVAL;
// Setting dynamic arrays is not (yet?) supported.
    if (WARN_ON(ctrl.is_dyn_array))
    return -EINVAL;
    memcpy(ctrl.p_new.p, p, ctrl.elems * ctrl.elem_size);
    return set_ctrl(core::ptr::null_mut(), ctrl, 0);
    }
    EXPORT_SYMBOL(__v4l2_ctrl_s_ctrl_compound);
//
// Modify the range of a control.
//
    int __v4l2_ctrl_modify_range(struct v4l2_ctrl *ctrl,
    s64 min, s64 max, u64 step, s64 def)
    {
    bool value_changed;
    let mut range_changed: bool = false;
    int ret;
    lockdep_assert_held(ctrl.handler.lock);
    switch (ctrl.type) {
    case V4L2_CTRL_TYPE_INTEGER:
    case V4L2_CTRL_TYPE_INTEGER64:
    case V4L2_CTRL_TYPE_BOOLEAN:
    case V4L2_CTRL_TYPE_MENU:
    case V4L2_CTRL_TYPE_INTEGER_MENU:
    case V4L2_CTRL_TYPE_BITMASK:
    case V4L2_CTRL_TYPE_U8:
    case V4L2_CTRL_TYPE_U16:
    case V4L2_CTRL_TYPE_U32:
    if (ctrl.is_array)
    return -EINVAL;
    ret = check_range(ctrl.type, min, max, step, def);
    if (ret)
    return ret;
    break;
    default:
    return -EINVAL;
    }
    if (ctrl.minimum != min || ctrl.maximum != max ||
    ctrl.step != step || ctrl.default_value != def) {
    range_changed = true;
    ctrl.minimum = min;
    ctrl.maximum = max;
    ctrl.step = step;
    ctrl.default_value = def;
    }
    cur_to_new(ctrl);
    if (validate_new(ctrl, ctrl.p_new)) {
    if (ctrl.type == V4L2_CTRL_TYPE_INTEGER64)
// ctrl->p_new.p_s64 = def;
    else
// ctrl->p_new.p_s32 = def;
    }
    if (ctrl.type == V4L2_CTRL_TYPE_INTEGER64)
    value_changed = *ctrl.p_new.p_s64 != *ctrl.p_cur.p_s64;
    else
    value_changed = *ctrl.p_new.p_s32 != *ctrl.p_cur.p_s32;
    if (value_changed)
    ret = set_ctrl(core::ptr::null_mut(), ctrl, V4L2_EVENT_CTRL_CH_RANGE);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: range_changed) -> else {
    else if (range_changed)
    send_event(core::ptr::null_mut(), ctrl, V4L2_EVENT_CTRL_CH_RANGE);
    return ret;
    }
    EXPORT_SYMBOL(__v4l2_ctrl_modify_range);
    int __v4l2_ctrl_modify_dimensions(struct v4l2_ctrl *ctrl,
    u32 dims[V4L2_CTRL_MAX_DIMS])
    {
    let mut elems: c_uint = 1;
    unsigned int i;
    void *p_array;
    lockdep_assert_held(ctrl.handler.lock);
    if (!ctrl.is_array || ctrl.is_dyn_array)
    return -EINVAL;
    for (i = 0; i < ctrl.nr_of_dims; i++)
    elems *= dims[i];
    if (elems == 0)
    return -EINVAL;
    p_array = kvzalloc(2 * elems * ctrl.elem_size, GFP_KERNEL);
    if (!p_array)
    return -ENOMEM;
    kvfree(ctrl.p_array);
    ctrl.p_array_alloc_elems = elems;
    ctrl.elems = elems;
    ctrl.new_elems = elems;
    ctrl.p_array = p_array;
    ctrl.p_new.p = p_array;
    ctrl.p_cur.p = p_array + elems * ctrl.elem_size;
    for (i = 0; i < ctrl.nr_of_dims; i++)
    ctrl.dims[i] = dims[i];
    ctrl.type_ops.init(ctrl, 0, ctrl.p_cur);
    cur_to_new(ctrl);
    send_event(core::ptr::null_mut(), ctrl, V4L2_EVENT_CTRL_CH_VALUE |
    V4L2_EVENT_CTRL_CH_DIMENSIONS);
    return 0;
    }
    EXPORT_SYMBOL(__v4l2_ctrl_modify_dimensions);
// Implement VIDIOC_QUERY_EXT_CTRL
#[no_mangle]
pub unsafe extern "C" fn v4l2_query_ext_ctrl(hdl: *mut v4l2_ctrl_handler, qc: *mut v4l2_query_ext_ctrl) -> c_int {
    int v4l2_query_ext_ctrl(struct v4l2_ctrl_handler *hdl, struct v4l2_query_ext_ctrl *qc)
    {
    let mut next_flags: c_uint = V4L2_CTRL_FLAG_NEXT_CTRL | V4L2_CTRL_FLAG_NEXT_COMPOUND;
    let mut id: u32 = qc.id & V4L2_CTRL_ID_MASK;
    struct v4l2_ctrl_ref *ref;
    struct v4l2_ctrl *ctrl;
    if (!hdl)
    return -EINVAL;
    mutex_lock(hdl.lock);
// Try to find it
    ref = find_ref(hdl, id);
    if ((qc.id & next_flags) && !list_empty(&hdl.ctrl_refs)) {
    bool is_compound;
// Match any control that is not hidden
    let mut mask: c_uint = 1;
    let mut match: bool = false;
    if ((qc.id & next_flags) == V4L2_CTRL_FLAG_NEXT_COMPOUND) {
// Match any hidden control
    match = true;
    } else if ((qc.id & next_flags) == next_flags) {
// Match any control, compound or not
    mask = 0;
    }
// Find the next control with ID > qc->id
// Did we reach the end of the control list?
    if (id >= node2id(hdl.ctrl_refs.prev)) {
    ref = core::ptr::null_mut(); /* Yes, so there is no next control */
    } else if (ref) {
    struct v4l2_ctrl_ref *pos = ref;
//
// We found a control with the given ID, so just get
// the next valid one in the list.
//
    ref = core::ptr::null_mut();
    list_for_each_entry_continue(pos, &hdl.ctrl_refs, node) {
    is_compound = pos.ctrl.is_array ||
    pos.ctrl.type >= V4L2_CTRL_COMPOUND_TYPES;
    if (id < pos.ctrl.id &&
    (is_compound & mask) == match) {
    ref = pos;
    break;
    }
    }
    } else {
    struct v4l2_ctrl_ref *pos;
//
// No control with the given ID exists, so start
// searching for the next largest ID. We know there
// is one, otherwise the first 'if' above would have
// been true.
//
    list_for_each_entry(pos, &hdl.ctrl_refs, node) {
    is_compound = pos.ctrl.is_array ||
    pos.ctrl.type >= V4L2_CTRL_COMPOUND_TYPES;
    if (id < pos.ctrl.id &&
    (is_compound & mask) == match) {
    ref = pos;
    break;
    }
    }
    }
    }
    mutex_unlock(hdl.lock);
    if (!ref)
    return -EINVAL;
    ctrl = ref.ctrl;
    memset(qc, 0, sizeof(*qc));
    if (id >= V4L2_CID_PRIVATE_BASE)
    qc.id = id;
    else
    qc.id = ctrl.id;
    strscpy(qc.name, ctrl.name, sizeof(qc.name));
    qc.flags = user_flags(ctrl);
    qc.type = ctrl.type;
    qc.elem_size = ctrl.elem_size;
    qc.elems = ctrl.elems;
    qc.nr_of_dims = ctrl.nr_of_dims;
    memcpy(qc.dims, ctrl.dims, qc.nr_of_dims * sizeof(qc.dims[0]));
    qc.minimum = ctrl.minimum;
    qc.maximum = ctrl.maximum;
    qc.default_value = ctrl.default_value;
    if (ctrl.type == V4L2_CTRL_TYPE_MENU ||
    ctrl.type == V4L2_CTRL_TYPE_INTEGER_MENU)
    qc.step = 1;
    else
    qc.step = ctrl.step;
    return 0;
    }
    EXPORT_SYMBOL(v4l2_query_ext_ctrl);
    void v4l2_query_ext_ctrl_to_v4l2_queryctrl(struct v4l2_queryctrl *to,
    const struct v4l2_query_ext_ctrl *from)
    {
    to.id = from.id;
    to.type = from.type;
    to.flags = from.flags;
    strscpy(to.name, from.name, sizeof(to.name));
    switch (from.type) {
    case V4L2_CTRL_TYPE_INTEGER:
    case V4L2_CTRL_TYPE_BOOLEAN:
    case V4L2_CTRL_TYPE_MENU:
    case V4L2_CTRL_TYPE_INTEGER_MENU:
    case V4L2_CTRL_TYPE_STRING:
    case V4L2_CTRL_TYPE_BITMASK:
    to.minimum = from.minimum;
    to.maximum = from.maximum;
    to.step = from.step;
    to.default_value = from.default_value;
    break;
    default:
    to.minimum = 0;
    to.maximum = 0;
    to.step = 0;
    to.default_value = 0;
    break;
    }
    }
    EXPORT_SYMBOL(v4l2_query_ext_ctrl_to_v4l2_queryctrl);
// Implement VIDIOC_QUERYCTRL
#[no_mangle]
pub unsafe extern "C" fn v4l2_queryctrl(hdl: *mut v4l2_ctrl_handler, qc: *mut v4l2_queryctrl) -> c_int {
    int v4l2_queryctrl(struct v4l2_ctrl_handler *hdl, struct v4l2_queryctrl *qc)
    {
    let mut qec: v4l2_query_ext_ctrl = { qc.id };
    int rc;
    rc = v4l2_query_ext_ctrl(hdl, &qec);
    if (rc)
    return rc;
    v4l2_query_ext_ctrl_to_v4l2_queryctrl(qc, &qec);
    return 0;
    }
    EXPORT_SYMBOL(v4l2_queryctrl);
// Implement VIDIOC_QUERYMENU
#[no_mangle]
pub unsafe extern "C" fn v4l2_querymenu(hdl: *mut v4l2_ctrl_handler, qm: *mut v4l2_querymenu) -> c_int {
    int v4l2_querymenu(struct v4l2_ctrl_handler *hdl, struct v4l2_querymenu *qm)
    {
    struct v4l2_ctrl *ctrl;
    let mut i: u32 = qm.index;
    ctrl = v4l2_ctrl_find(hdl, qm.id);
    if (!ctrl)
    return -EINVAL;
    qm.reserved = 0;
// Sanity checks
    switch (ctrl.type) {
    case V4L2_CTRL_TYPE_MENU:
    if (!ctrl.qmenu)
    return -EINVAL;
    break;
    case V4L2_CTRL_TYPE_INTEGER_MENU:
    if (!ctrl.qmenu_int)
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    if (i < ctrl.minimum || i > ctrl.maximum)
    return -EINVAL;
// Use mask to see if this menu item should be skipped
    if (i < BITS_PER_LONG_LONG && (ctrl.menu_skip_mask & BIT_ULL(i)))
    return -EINVAL;
// Empty menu items should also be skipped
    if (ctrl.type == V4L2_CTRL_TYPE_MENU) {
    if (!ctrl.qmenu[i] || ctrl.qmenu[i][0] == '\0')
    return -EINVAL;
    strscpy(qm.name, ctrl.qmenu[i], sizeof(qm.name));
    } else {
    qm.value = ctrl.qmenu_int[i];
    }
    return 0;
    }
    EXPORT_SYMBOL(v4l2_querymenu);
//
// VIDIOC_LOG_STATUS helpers
//
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_log_status(file: *mut file, priv: *mut c_void) -> c_int {
    int v4l2_ctrl_log_status(struct file *file, void *priv)
    {
    struct video_device *vfd = video_devdata(file);
    if (vfd.v4l2_dev) {
    struct v4l2_fh *vfh = file_to_v4l2_fh(file);
    v4l2_ctrl_handler_log_status(vfh.ctrl_handler,
    vfd.v4l2_dev.name);
    }
    return 0;
    }
    EXPORT_SYMBOL(v4l2_ctrl_log_status);
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_subdev_log_status(sd: *mut v4l2_subdev) -> c_int {
    int v4l2_ctrl_subdev_log_status(struct v4l2_subdev *sd)
    {
    v4l2_ctrl_handler_log_status(sd.ctrl_handler, sd.name);
    return 0;
    }
    EXPORT_SYMBOL(v4l2_ctrl_subdev_log_status);
//
// VIDIOC_(UN)SUBSCRIBE_EVENT implementation
//
    static int v4l2_ctrl_add_event(struct v4l2_subscribed_event *sev,
    unsigned int elems)
    {
    struct v4l2_ctrl *ctrl = v4l2_ctrl_find(sev.fh.ctrl_handler, sev.id);
    if (!ctrl)
    return -EINVAL;
    v4l2_ctrl_lock(ctrl);
    list_add_tail(&sev.node, &ctrl.ev_subs);
    if (ctrl.type != V4L2_CTRL_TYPE_CTRL_CLASS &&
    (sev.flags & V4L2_EVENT_SUB_FL_SEND_INITIAL))
    send_initial_event(sev.fh, ctrl);
    v4l2_ctrl_unlock(ctrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn v4l2_ctrl_del_event(sev: *mut v4l2_subscribed_event) {
    static void v4l2_ctrl_del_event(struct v4l2_subscribed_event *sev)
    {
    struct v4l2_ctrl *ctrl = v4l2_ctrl_find(sev.fh.ctrl_handler, sev.id);
    if (!ctrl)
    return;
    v4l2_ctrl_lock(ctrl);
    list_del(&sev.node);
    v4l2_ctrl_unlock(ctrl);
    }
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_replace(old: *mut v4l2_event, new: *const v4l2_event) {
    void v4l2_ctrl_replace(struct v4l2_event *old, const struct v4l2_event *new)
    {
    let mut old_changes: u32 = old.u.ctrl.changes;
    old.u.ctrl = new.u.ctrl;
    old.u.ctrl.changes |= old_changes;
    }
    EXPORT_SYMBOL(v4l2_ctrl_replace);
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_merge(old: *const v4l2_event, new: *mut v4l2_event) {
    void v4l2_ctrl_merge(const struct v4l2_event *old, struct v4l2_event *new)
    {
    new.u.ctrl.changes |= old.u.ctrl.changes;
    }
    EXPORT_SYMBOL(v4l2_ctrl_merge);
    const struct v4l2_subscribed_event_ops v4l2_ctrl_sub_ev_ops = {
    .add = v4l2_ctrl_add_event,
    .del = v4l2_ctrl_del_event,
    .replace = v4l2_ctrl_replace,
    .merge = v4l2_ctrl_merge,
    };
    EXPORT_SYMBOL(v4l2_ctrl_sub_ev_ops);
    int v4l2_ctrl_subscribe_event(struct v4l2_fh *fh,
    const struct v4l2_event_subscription *sub)
    {
    if (sub.type == V4L2_EVENT_CTRL)
    return v4l2_event_subscribe(fh, sub, 0, &v4l2_ctrl_sub_ev_ops);
    return -EINVAL;
    }
    EXPORT_SYMBOL(v4l2_ctrl_subscribe_event);
    int v4l2_ctrl_subdev_subscribe_event(struct v4l2_subdev *sd, struct v4l2_fh *fh,
    struct v4l2_event_subscription *sub)
    {
    if (!sd.ctrl_handler)
    return -EINVAL;
    return v4l2_ctrl_subscribe_event(fh, sub);
    }
    EXPORT_SYMBOL(v4l2_ctrl_subdev_subscribe_event);
//
// poll helper
//
#[no_mangle]
pub unsafe extern "C" fn v4l2_ctrl_poll(file: *mut file, wait: *mut poll_table_struct) -> __poll_t {
    __poll_t v4l2_ctrl_poll(struct file *file, struct poll_table_struct *wait)
    {
    struct v4l2_fh *fh = file_to_v4l2_fh(file);
    poll_wait(file, &fh.wait, wait);
    if (v4l2_event_pending(fh))
    return EPOLLPRI;
    return 0;
    }
    EXPORT_SYMBOL(v4l2_ctrl_poll);
