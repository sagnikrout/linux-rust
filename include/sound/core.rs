//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/core.h
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
// Main header file for the ALSA driver
// Copyright (c) 1994-2001 by Jaroslav Kysela <perex@perex.cz>
//

// number of supported soundcards

// forward declarations
// device allocation stuff
// type of the object used in snd_device_*()
// this also defines the calling order
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_device_type {
    SNDRV_DEV_LOWLEVEL,
    SNDRV_DEV_INFO,
    SNDRV_DEV_BUS,
    SNDRV_DEV_CODEC,
    SNDRV_DEV_PCM,
    SNDRV_DEV_COMPRESS,
    SNDRV_DEV_RAWMIDI,
    SNDRV_DEV_TIMER,
    SNDRV_DEV_SEQUENCER,
    SNDRV_DEV_HWDEP,
    SNDRV_DEV_JACK,
    SNDRV_DEV_CONTROL,	/* NOTE: this must be the last one */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_device_state {
    SNDRV_DEV_BUILD,
    SNDRV_DEV_REGISTERED,
    SNDRV_DEV_DISCONNECTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_device_ops {
    pub dev): *mut *mut int (dev_free)(struct snd_device,
    pub dev): *mut *mut int (dev_register)(struct snd_device,
    pub dev): *mut *mut int (dev_disconnect)(struct snd_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_device {
    pub /: *mut *mut list_head list; / list of registered devices,
    pub /: *mut *mut *mut snd_card card; / card which holds this device,
    pub /: *mut *mut snd_device_state state; / state of the device,
    pub /: *mut *mut snd_device_type type; / device type,
    pub /: *mut *mut *mut void device_data; / device structure,
    pub /: *const *const *const snd_device_ops ops; / operations,
}

//
// A simple reference counter with a wait queue;
// typically used for usage counts, and you can synchronize at finishing
// via snd_refcount_sync(), which is woken up when the refcount reaches to
// zero again.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_refcount {
    pub count: core::sync::atomic::AtomicI32,
    pub waiter: wait_queue_head_t,
}

extern "C" {
    pub fn snd_refcount_init(ref: *mut snd_refcount);
}
extern "C" {
    pub fn snd_refcount_put(ref: *mut snd_refcount);
}
extern "C" {
    pub fn snd_refcount_sync(ref: *mut snd_refcount);
}
// main structure for soundcard
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_card {
    pub to: *mut *mut int number; / number of soundcard (index,
    pub /: *mut *mut char id[16]; / id string of this card,
    pub /: *mut *mut char driver[16]; / driver name,
    pub /: *mut *mut char shortname[32]; / short name of this soundcard,
    pub /: *mut *mut char longname[80]; / name of this soundcard,
    pub /: *mut *mut char irq_descr[32]; / Interrupt description,
    pub /: *mut *mut char mixername[80]; / mixer name,
    pub /: *mut *mut *mut char components; / card components, space-delimited,
    pub /: *mut *mut unsigned int components_alloc_size; / current allocation size of components,
    pub /: *mut *mut *mut module module; / top-level module,
    pub /: *mut *mut *mut void private_data; / private data for soundcard,
    pub of: *mut *mut *mut *mut void (private_free) (struct snd_card card); / callback for freeing,
    pub /: *mut *mut list_head devices; / devices,
    pub /: *mut *mut *mut device ctl_dev; / control device,
    pub /: *mut *mut unsigned int last_numid; / last used numeric ID,
    pub /: *mut *mut rw_semaphore controls_rwsem; / controls lock (list and values),
    pub /: *mut *mut rwlock_t controls_rwlock; / lock for lookup and ctl_files list,
    pub /: *mut *mut int controls_count; / count of all controls,
    pub controls.: size_t user_ctl_alloc_size; // current memory allocation by user,
    pub /: *mut *mut list_head controls; / all controls for this card,
    pub /: *mut *mut list_head ctl_files; / active control files,

    pub /: *mut *mut xarray ctl_numids; / hash table for numids,
    pub /: *mut *mut xarray ctl_hash; / hash table for ctl id matching,
    pub /: *mut *mut bool ctl_hash_collision; / ctl_hash collision seen?,

    pub /: *mut *mut *mut snd_info_entry proc_root; / root for soundcard specific files,
    pub /: *mut *mut *mut proc_dir_entry proc_root_link; / number link to real id,
    pub /: *mut *mut list_head files_list; / all files associated to this card,
    pub shutdown: *mut *mut *mut snd_shutdown_f_ops s_f_ops; / file operations in the,
    pub /: *mut *mut spinlock_t files_lock; / lock the files for this card,
    pub /: *mut *mut int shutdown; / this card is going down,
    pub release_completion: *mut completion,
    pub /: *mut *mut *mut device dev; / device assigned to this card,
    pub /: *mut *mut device card_dev; / cardX object for sysfs,
    pub /: *const *const *const attribute_group dev_groups[4]; / assigned sysfs attr,
    pub /: *mut *mut bool registered; / card_dev is registered?,
    pub /: *mut *mut bool managed; / managed via devres,
    pub /: *mut *mut bool releasing; / during card free process,
    pub /: *mut *mut int sync_irq; / assigned irq, used for PCM sync,
    pub remove_sleep: wait_queue_head_t,
    pub /: *mut *mut size_t total_pcm_alloc_bytes; / total amount of allocated buffers,
    pub /: *mut *mut mutex memory_mutex; / protection for the above,

    pub /: *mut *mut *mut dentry debugfs_root; / debugfs root for card,

    pub /: *mut *mut *mut snd_ctl_elem_value value_buf; / buffer for kctl->put() verification,

    pub /: *mut *mut unsigned int power_state; / power state,
    pub power_sleep: wait_queue_head_t,
    pub power_ref: snd_refcount,

    pub mixer_oss: *mut snd_mixer_oss,
    pub mixer_oss_change_count: c_int,

    pub long)): unsigned char private_data_area[] __aligned(__alignof__(unsigned long,
}

extern "C" {
    pub fn READ_ONCE(_arg: card->power_state) -> return;
}
//
// snd_power_ref - Take the reference count for power control
// @card: sound card object
//
// The power_ref reference of the card is used for managing to block
// the snd_power_sync_ref() operation.  This function increments the reference.
// The counterpart snd_power_unref() has to be called appropriately later.
//
// snd_power_unref - Release the reference count for power control
// @card: sound card object
//
// snd_power_sync_ref - wait until the card power_ref is freed
// @card: sound card object
//
// This function is used to synchronize with the pending power_ref being
// released.
//
// init.c
extern "C" {
    pub fn snd_power_wait(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_power_ref_and_wait(card: *mut snd_card) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_minor {
    pub /: *mut *mut int type; / SNDRV_DEVICE_TYPE_XXX,
    pub /: *mut *mut int card; / card number,
    pub /: *mut *mut int device; / device number,
    pub /: *const *const *const file_operations f_ops; / file operations,
    pub /: *mut *mut *mut void private_data; / private data for f_ops->open,
    pub /: *mut *mut *mut device dev; / device for sysfs,
    pub /: *mut *mut *mut snd_card card_ptr; / assigned card instance,
}

// return a device pointer linked to each sound device as a parent
// sound.c

extern "C" {
    pub fn snd_request_card(card: c_int);
}
extern "C" {
    pub fn snd_device_alloc(dev_p: *mut device, card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_unregister_device(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn snd_unregister_oss_device(type: c_int, card: *mut snd_card, dev: c_int) -> c_int;
}

extern "C" {
    pub fn snd_minor_info_init() -> c_int;
}
// sound_oss.c

extern "C" {
    pub fn snd_minor_info_oss_init() -> c_int;
}

// memory.c
extern "C" {
    pub fn copy_to_user_fromio(dst: *mut void __user, src: *const volatile void __iomem, count: usize) -> c_int;
}
extern "C" {
    pub fn copy_from_user_toio(dst: *mut volatile void __iomem, src: *const void __user, count: usize) -> c_int;
}
// init.c
extern "C" {
    pub fn snd_card_locked(card: c_int) -> c_int;
}

pub const SND_MIXER_OSS_NOTIFY_REGISTER: c_int = 0;
pub const SND_MIXER_OSS_NOTIFY_DISCONNECT: c_int = 1;
pub const SND_MIXER_OSS_NOTIFY_FREE: c_int = 2;
extern "C" {
    pub fn int(card: *mut *mut snd_mixer_oss_notify_callback)(struct snd_card, cmd: c_int) -> extern;
}

extern "C" {
    pub fn snd_card_disconnect(card: *mut snd_card);
}
extern "C" {
    pub fn snd_card_disconnect_sync(card: *mut snd_card);
}
extern "C" {
    pub fn snd_card_free(card: *mut snd_card);
}
extern "C" {
    pub fn snd_card_free_when_closed(card: *mut snd_card);
}
extern "C" {
    pub fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
}
extern "C" {
    pub fn snd_card_set_id(card: *mut snd_card, id: *const c_char);
}
extern "C" {
    pub fn snd_card_register(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_card_info_init() -> c_int;
}
extern "C" {
    pub fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_card_file_add(card: *mut snd_card, file: *mut file) -> c_int;
}
extern "C" {
    pub fn snd_card_file_remove(card: *mut snd_card, file: *mut file) -> c_int;
}
//
// snd_card_unref - Unreference the card object
// @card: the card object to unreference
//
// Call this function for the card object that was obtained via snd_card_ref()
// or snd_lookup_minor_data().
//

// device.c
extern "C" {
    pub fn snd_device_register(card: *mut snd_card, device_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn snd_device_register_all(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn snd_device_disconnect(card: *mut snd_card, device_data: *mut c_void);
}
extern "C" {
    pub fn snd_device_disconnect_all(card: *mut snd_card);
}
extern "C" {
    pub fn snd_device_free(card: *mut snd_card, device_data: *mut c_void);
}
extern "C" {
    pub fn snd_device_free_all(card: *mut snd_card);
}
// isadma.c

pub const DMA_MODE_NO_ENABLE: c_uint = 0x0100;
extern "C" {
    pub fn snd_dma_program(dma: c_ulong, addr: c_ulong, size: c_uint, mode: c_ushort);
}
extern "C" {
    pub fn snd_dma_disable(dma: c_ulong);
}
extern "C" {
    pub fn snd_dma_pointer(dma: c_ulong, size: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_devm_request_dma(dev: *mut device, dma: c_int, name: *const c_char) -> c_int;
}

// misc.c
extern "C" {
    pub fn release_and_free_resource(res: *mut resource);
}
// ---

//
// snd_BUG - give a BUG warning message and stack trace
//
// Calls WARN() if CONFIG_SND_DEBUG is set.
// Ignored when CONFIG_SND_DEBUG is not set.
//

//
// snd_BUG_ON - debugging check macro
// @cond: condition to evaluate
//
// Has the same behavior as WARN_ON when CONFIG_SND_DEBUG is set,
// otherwise just evaluates the conditional and returns the value.
//

// for easier backward-porting

// PCI quirk list helper
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pci_quirk {
    pub /: *mut *mut unsigned short subvendor; / PCI subvendor ID,
    pub /: *mut *mut unsigned short subdevice; / PCI subdevice ID,
    pub /: *mut *mut unsigned short subdevice_mask; / bitmask to match,
    pub /: *mut *mut int value; / value,

    pub /: *const *const *const char name; / name of the device (optional),

}

// async signal helpers
extern "C" {
    pub fn snd_kill_fasync(fasync: *mut snd_fasync, signal: c_int, poll: c_int);
}
extern "C" {
    pub fn snd_fasync_free(fasync: *mut snd_fasync);
}
