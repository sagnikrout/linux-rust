//! Automatically rewritten from C to Rust
//! Source: sound/core/control_compat.c
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
// compat ioctls for control API
//
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
//
// this file included from control.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_list32 {
    pub offset: u32,
    pub space: u32,
    pub used: u32,
    pub count: u32,
    pub pids: u32,
    pub reserved: [c_uchar; 50],
    pub /: *mut *mut } / don't set packed attribute here,
    static int snd_ctl_elem_list_compat(struct snd_card *card,
    struct snd_ctl_elem_list32 __user *data32)
    {
    pub {}: snd_ctl_elem_list data =,
    pub ptr: compat_caddr_t,
    pub err: c_int,
// offset, space, used, count
    if (copy_from_user(&data, data32, 4 * sizeof(u32)))
    pub -EFAULT: return,
// pids
    if (get_user(ptr, &data32.pids))
    pub -EFAULT: return,
    pub compat_ptr(ptr): data.pids =,
    pub &data): err = snd_ctl_elem_list(card,,
    if (err < 0)
    pub err: return,
// copy the result
    if (copy_to_user(data32, &data, 4 * sizeof(u32)))
    pub -EFAULT: return,
    pub 0: return,
    }
//
// control element info
// it uses union, so the things are not easy..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info32 {
    pub same: snd_ctl_elem_id id; // the size of is,
    pub type: i32,
    pub access: u32,
    pub count: u32,
    pub owner: i32,
    union {
    struct {
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub integer: },
    struct {
    pub min: u64,
    pub max: u64,
    pub step: u64,
    pub integer64: },
    struct {
    pub items: u32,
    pub item: u32,
    pub name: [c_char; 64],
    pub names_ptr: u64,
    pub names_length: u32,
    pub enumerated: },
    pub reserved: [c_uchar; 128],
    pub value: },
    pub reserved: [c_uchar; 64],
    pub __packed: },
    static int snd_ctl_elem_info_compat(struct snd_ctl_file *ctl,
    struct snd_ctl_elem_info32 __user *data32)
    {
    pub ctl->card: *mut *mut snd_card card =,
    pub err: c_int,
    struct snd_ctl_elem_info *data __free(kfree) =
    if (! data)
    pub -ENOMEM: return,
// copy id
    if (copy_from_user(&data.id, &data32.id, sizeof(data.id)))
    pub -EFAULT: return,
// we need to copy the item index.
// hope this doesn't break anything..
//
    if (get_user(data.value.enumerated.item, &data32.value.enumerated.item))
    pub -EFAULT: return,
    pub snd_power_ref_and_wait(card): err =,
    if (err < 0)
    pub err: return,
    pub data): err = snd_ctl_elem_info(ctl,,
    if (err < 0)
    pub err: return,
// restore info to 32bit
// id, type, access, count
    if (copy_to_user(&data32.id, &data.id, sizeof(data.id)) ||
    copy_to_user(&data32.type, &data.type, 3 * sizeof(u32)))
    pub -EFAULT: return,
    if (put_user(data.owner, &data32.owner))
    pub -EFAULT: return,
    switch (data.type) {
    case SNDRV_CTL_ELEM_TYPE_BOOLEAN:
    case SNDRV_CTL_ELEM_TYPE_INTEGER:
    if (put_user(data.value.integer.min, &data32.value.integer.min) ||
    put_user(data.value.integer.max, &data32.value.integer.max) ||
    put_user(data.value.integer.step, &data32.value.integer.step))
    pub -EFAULT: return,
    case SNDRV_CTL_ELEM_TYPE_INTEGER64:
    if (copy_to_user(&data32.value.integer64,
    &data.value.integer64,
    sizeof(data.value.integer64)))
    pub -EFAULT: return,
    case SNDRV_CTL_ELEM_TYPE_ENUMERATED:
    if (copy_to_user(&data32.value.enumerated,
    &data.value.enumerated,
    sizeof(data.value.enumerated)))
    pub -EFAULT: return,
    default:
    }
    pub 0: return,
    }
// read / write
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value32 {
    pub id: snd_ctl_elem_id,
    pub /: *mut *mut unsigned int indirect; / bit-field causes misalignment,
    union {
    pub integer: [i32; 128],
    pub data: [c_uchar; 512],    pub integer64: [i64; 64],
    pub value: },
    pub reserved: [c_uchar; 128],
}

// x32 has a different alignment for 64bit values from ia32
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_x32 {
    pub id: snd_ctl_elem_id,
    pub /: *mut *mut unsigned int indirect; / bit-field causes misalignment,
    union {
    pub integer: [i32; 128],
    pub data: [c_uchar; 512],
    pub integer64: [i64; 64],
    pub value: },
    pub reserved: [c_uchar; 128],
}

// get the value type and count of the control
    static int get_ctl_type(struct snd_card *card, struct snd_ctl_elem_id *id,
    int *countp)
    {
    struct snd_kcontrol *kctl;
    int err;
    guard(rwsem_read)(&card.controls_rwsem);
    kctl = snd_ctl_find_id(card, id);
    if (!kctl)
    return -ENOENT;
    struct snd_ctl_elem_info *info __free(kfree) =
    kzalloc_obj(*info);
    if (info == core::ptr::null_mut())
    return -ENOMEM;
    info.id = *id;
    err = kctl.info(kctl, info);
    if (err >= 0) {
    err = info.type;
// countp = info->count;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn get_elem_size(type: snd_ctl_elem_type_t, count: c_int) -> c_int {
    static int get_elem_size(snd_ctl_elem_type_t type, int count)
    {
    switch (type) {
    case SNDRV_CTL_ELEM_TYPE_INTEGER64:
    return sizeof(s64) * count;
    case SNDRV_CTL_ELEM_TYPE_ENUMERATED:
    return sizeof(int) * count;
    case SNDRV_CTL_ELEM_TYPE_BYTES:
    return 512;
    case SNDRV_CTL_ELEM_TYPE_IEC958:
    return sizeof(struct snd_aes_iec958);
    default:
    return -1;
    }
    }
    static int copy_ctl_value_from_user(struct snd_card *card,
    struct snd_ctl_elem_value *data,
    void __user *userdata,
    void __user *valuep,
    int *typep, int *countp)
    {
    struct snd_ctl_elem_value32 __user *data32 = userdata;
    int i, type, size;
    int count;
    unsigned int indirect;
    if (copy_from_user(&data.id, &data32.id, sizeof(data.id)))
    return -EFAULT;
    if (get_user(indirect, &data32.indirect))
    return -EFAULT;
    if (indirect)
    return -EINVAL;
    type = get_ctl_type(card, &data.id, &count);
    if (type < 0)
    return type;
    if (type == SNDRV_CTL_ELEM_TYPE_BOOLEAN ||
    type == SNDRV_CTL_ELEM_TYPE_INTEGER) {
    for (i = 0; i < count; i++) {
    s32 __user *intp = valuep;
    int val;
    if (get_user(val, &intp[i]))
    return -EFAULT;
    data.value.integer.value[i] = val;
    }
    } else {
    size = get_elem_size(type, count);
    if (size < 0) {
    dev_err(card.dev, "snd_ioctl32_ctl_elem_value: unknown type %d\n", type);
    return -EINVAL;
    }
    if (copy_from_user(data.value.bytes.data, valuep, size))
    return -EFAULT;
    }
// typep = type;
// countp = count;
    return 0;
    }
// restore the value to 32bit
    static int copy_ctl_value_to_user(void __user *userdata,
    void __user *valuep,
    struct snd_ctl_elem_value *data,
    int type, int count)
    {
    struct snd_ctl_elem_value32 __user *data32 = userdata;
    int i, size;
    if (type == SNDRV_CTL_ELEM_TYPE_BOOLEAN ||
    type == SNDRV_CTL_ELEM_TYPE_INTEGER) {
    for (i = 0; i < count; i++) {
    s32 __user *intp = valuep;
    int val;
    val = data.value.integer.value[i];
    if (put_user(val, &intp[i]))
    return -EFAULT;
    }
    } else {
    size = get_elem_size(type, count);
    if (copy_to_user(valuep, data.value.bytes.data, size))
    return -EFAULT;
    }
    if (copy_to_user(&data32.id, &data.id, sizeof(data32.id)))
    return -EFAULT;
    return 0;
    }
    static int __ctl_elem_read_user(struct snd_card *card,
    void __user *userdata, void __user *valuep)
    {
    int err, type, count;
    struct snd_ctl_elem_value *data __free(kfree) =
    kzalloc_obj(*data);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    err = copy_ctl_value_from_user(card, data, userdata, valuep,
    &type, &count);
    if (err < 0)
    return err;
    err = snd_ctl_elem_read(card, data);
    if (err < 0)
    return err;
    return copy_ctl_value_to_user(userdata, valuep, data, type, count);
    }
    static int ctl_elem_read_user(struct snd_card *card,
    void __user *userdata, void __user *valuep)
    {
    int err;
    err = snd_power_ref_and_wait(card);
    if (err < 0)
    return err;
    err = __ctl_elem_read_user(card, userdata, valuep);
    snd_power_unref(card);
    return err;
    }
    static int __ctl_elem_write_user(struct snd_ctl_file *file,
    void __user *userdata, void __user *valuep)
    {
    struct snd_card *card = file.card;
    int err, type, count;
    struct snd_ctl_elem_value *data __free(kfree) =
    kzalloc_obj(*data);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    err = copy_ctl_value_from_user(card, data, userdata, valuep,
    &type, &count);
    if (err < 0)
    return err;
    err = snd_ctl_elem_write(card, file, data);
    if (err < 0)
    return err;
    return copy_ctl_value_to_user(userdata, valuep, data, type, count);
    }
    static int ctl_elem_write_user(struct snd_ctl_file *file,
    void __user *userdata, void __user *valuep)
    {
    struct snd_card *card = file.card;
    int err;
    err = snd_power_ref_and_wait(card);
    if (err < 0)
    return err;
    err = __ctl_elem_write_user(file, userdata, valuep);
    snd_power_unref(card);
    return err;
    }
    static int snd_ctl_elem_read_user_compat(struct snd_card *card,
    struct snd_ctl_elem_value32 __user *data32)
    {
    return ctl_elem_read_user(card, data32, &data32.value);
    }
    static int snd_ctl_elem_write_user_compat(struct snd_ctl_file *file,
    struct snd_ctl_elem_value32 __user *data32)
    {
    return ctl_elem_write_user(file, data32, &data32.value);
    }

    static int snd_ctl_elem_read_user_x32(struct snd_card *card,
    struct snd_ctl_elem_value_x32 __user *data32)
    {
    return ctl_elem_read_user(card, data32, &data32.value);
    }
    static int snd_ctl_elem_write_user_x32(struct snd_ctl_file *file,
    struct snd_ctl_elem_value_x32 __user *data32)
    {
    return ctl_elem_write_user(file, data32, &data32.value);
    }

// add or replace a user control
    static int snd_ctl_elem_add_compat(struct snd_ctl_file *file,
    struct snd_ctl_elem_info32 __user *data32,
    int replace)
    {
    struct snd_ctl_elem_info *data __free(kfree) =
    kzalloc_obj(*data);
    if (! data)
    return -ENOMEM;
// id, type, access, count */ \
    if (copy_from_user(&data.id, &data32.id, sizeof(data.id)) ||
    copy_from_user(&data.type, &data32.type, 3 * sizeof(u32)))
    return -EFAULT;
    if (get_user(data.owner, &data32.owner))
    return -EFAULT;
    switch (data.type) {
    case SNDRV_CTL_ELEM_TYPE_BOOLEAN:
    case SNDRV_CTL_ELEM_TYPE_INTEGER:
    if (get_user(data.value.integer.min, &data32.value.integer.min) ||
    get_user(data.value.integer.max, &data32.value.integer.max) ||
    get_user(data.value.integer.step, &data32.value.integer.step))
    return -EFAULT;
    break;
    case SNDRV_CTL_ELEM_TYPE_INTEGER64:
    if (copy_from_user(&data.value.integer64,
    &data32.value.integer64,
    sizeof(data.value.integer64)))
    return -EFAULT;
    break;
    case SNDRV_CTL_ELEM_TYPE_ENUMERATED:
    if (copy_from_user(&data.value.enumerated,
    &data32.value.enumerated,
    sizeof(data.value.enumerated)))
    return -EFAULT;
    data.value.enumerated.names_ptr =
    (uintptr_t)compat_ptr(data.value.enumerated.names_ptr);
    break;
    default:
    break;
    }
    return snd_ctl_elem_add(file, data, replace);
    }
    enum {
    SNDRV_CTL_IOCTL_ELEM_LIST32 = _IOWR('U', 0x10, struct snd_ctl_elem_list32),
    SNDRV_CTL_IOCTL_ELEM_INFO32 = _IOWR('U', 0x11, struct snd_ctl_elem_info32),
    SNDRV_CTL_IOCTL_ELEM_READ32 = _IOWR('U', 0x12, struct snd_ctl_elem_value32),
    SNDRV_CTL_IOCTL_ELEM_WRITE32 = _IOWR('U', 0x13, struct snd_ctl_elem_value32),
    SNDRV_CTL_IOCTL_ELEM_ADD32 = _IOWR('U', 0x17, struct snd_ctl_elem_info32),
    SNDRV_CTL_IOCTL_ELEM_REPLACE32 = _IOWR('U', 0x18, struct snd_ctl_elem_info32),

    SNDRV_CTL_IOCTL_ELEM_READ_X32 = _IOWR('U', 0x12, struct snd_ctl_elem_value_x32),
    SNDRV_CTL_IOCTL_ELEM_WRITE_X32 = _IOWR('U', 0x13, struct snd_ctl_elem_value_x32),

    };
#[no_mangle]
pub unsafe extern "C" fn snd_ctl_ioctl_compat(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static inline long snd_ctl_ioctl_compat(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct snd_ctl_file *ctl;
    struct snd_kctl_ioctl *p;
    void __user *argp = compat_ptr(arg);
    int err;
    ctl = file.private_data;
    if (snd_BUG_ON(!ctl || !ctl.card))
    return -ENXIO;
    switch (cmd) {
    case SNDRV_CTL_IOCTL_PVERSION:
    case SNDRV_CTL_IOCTL_CARD_INFO:
    case SNDRV_CTL_IOCTL_CARD_BYTES:
    case SNDRV_CTL_IOCTL_SUBSCRIBE_EVENTS:
    case SNDRV_CTL_IOCTL_POWER:
    case SNDRV_CTL_IOCTL_POWER_STATE:
    case SNDRV_CTL_IOCTL_ELEM_LOCK:
    case SNDRV_CTL_IOCTL_ELEM_UNLOCK:
    case SNDRV_CTL_IOCTL_ELEM_REMOVE:
    case SNDRV_CTL_IOCTL_TLV_READ:
    case SNDRV_CTL_IOCTL_TLV_WRITE:
    case SNDRV_CTL_IOCTL_TLV_COMMAND:
    return snd_ctl_ioctl(file, cmd, (unsigned long)argp);
    case SNDRV_CTL_IOCTL_ELEM_LIST32:
    return snd_ctl_elem_list_compat(ctl.card, argp);
    case SNDRV_CTL_IOCTL_ELEM_INFO32:
    return snd_ctl_elem_info_compat(ctl, argp);
    case SNDRV_CTL_IOCTL_ELEM_READ32:
    return snd_ctl_elem_read_user_compat(ctl.card, argp);
    case SNDRV_CTL_IOCTL_ELEM_WRITE32:
    return snd_ctl_elem_write_user_compat(ctl, argp);
    case SNDRV_CTL_IOCTL_ELEM_ADD32:
    return snd_ctl_elem_add_compat(ctl, argp, 0);
    case SNDRV_CTL_IOCTL_ELEM_REPLACE32:
    return snd_ctl_elem_add_compat(ctl, argp, 1);

    case SNDRV_CTL_IOCTL_ELEM_READ_X32:
    return snd_ctl_elem_read_user_x32(ctl.card, argp);
    case SNDRV_CTL_IOCTL_ELEM_WRITE_X32:
    return snd_ctl_elem_write_user_x32(ctl, argp);

    }
    guard(rwsem_read)(&snd_ioctl_rwsem);
    list_for_each_entry(p, &snd_control_compat_ioctls, list) {
    if (p.fioctl) {
    err = p.fioctl(ctl.card, ctl, cmd, arg);
    if (err != -ENOIOCTLCMD)
    return err;
    }
    }
    return -ENOIOCTLCMD;
    }
