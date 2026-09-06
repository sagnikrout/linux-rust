//! Automatically rewritten from C to Rust
//! Source: sound/sound_core.c
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
// Sound core.  This file is composed of two parts.  sound_class
// which is common to both OSS and ALSA and OSS sound core which
// is used OSS or emulation of it.
//
// First, the common part.
//

    static int __init init_oss_soundcore(void);
    static void cleanup_oss_soundcore(void);

    static inline int init_oss_soundcore(void)	{ return 0; }
    static inline void cleanup_oss_soundcore(void)	{ }

    MODULE_DESCRIPTION("Core sound module");
    MODULE_AUTHOR("Alan Cox");
    MODULE_LICENSE("GPL");
    static char *sound_devnode(const struct device *dev, umode_t *mode)
    {
    if (MAJOR(dev.devt) == SOUND_MAJOR)
    return core::ptr::null_mut();
    return kasprintf(GFP_KERNEL, "snd/%s", dev_name(dev));
    }
    const struct class sound_class = {
    .name = "sound",
    .devnode = sound_devnode,
    };
    EXPORT_SYMBOL(sound_class);
#[no_mangle]
unsafe extern "C" fn init_soundcore() -> int __init {
    static int __init init_soundcore(void)
    {
    int rc;
    rc = init_oss_soundcore();
    if (rc)
    return rc;
    rc = class_register(&sound_class);
    if (rc) {
    cleanup_oss_soundcore();
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_soundcore() -> void __exit {
    static void __exit cleanup_soundcore(void)
    {
    cleanup_oss_soundcore();
    class_unregister(&sound_class);
    }
    subsys_initcall(init_soundcore);
    module_exit(cleanup_soundcore);

//
// OSS sound core handling. Breaks out sound functions to submodules
//
// Author:		Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// Fixes:
//
// --------------------
//
// Top level handler for the sound subsystem. Various devices can
// plug into this. The fact they don't all go via OSS doesn't mean
// they don't have to implement the OSS API. There is a lot of logic
// to keeping much of the OSS weight out of the code in a compatibility
// module, but it's up to the driver to rember to load it...
//
// The code provides a set of functions for registration of devices
// by type. This is done rather than providing a single call so that
// we can hide any future changes in the internals (eg when we go to
// 32bit dev_t) from the modules and their interface.
//
// Secondly we need to allocate the dsp, dsp16 and audio devices as
// one. Thus we misuse the chains a bit to simplify this.
//
// Thirdly to make it more fun and for 2.3.x and above we do all
// of this using fine grained locking.
//
// FIXME: we have to resolve modules and fine grained load/unload
// locking at some point in 2.3.x.
//

pub const SOUND_STEP: c_int = 16;
    struct sound_unit
    {
    int unit_minor;
    const struct file_operations *unit_fops;
    struct sound_unit *next;
    char name[32];
    };
//
// By default, OSS sound_core claims full legacy minor range (0-255)
// of SOUND_MAJOR to trap open attempts to any sound minor and
// requests modules using custom sound-slot/service-* module aliases.
// The only benefit of doing this is allowing use of custom module
// aliases instead of the standard char-major-* ones.  This behavior
// prevents alternative OSS implementation and is scheduled to be
// removed.
//
// CONFIG_SOUND_OSS_CORE_PRECLAIM and soundcore.preclaim_oss kernel
// parameter are added to allow distros and developers to try and
// switch to alternative implementations without needing to rebuild
// the kernel in the meantime.  If preclaim_oss is non-zero, the
// kernel will behave the same as before.  All SOUND_MAJOR minors are
// preclaimed and the custom module aliases along with standard chrdev
// ones are emitted if a missing device is opened.  If preclaim_oss is
// zero, sound_core only grabs what's actually in use and for missing
// devices only the standard chrdev aliases are requested.
//
// All these clutters are scheduled to be removed along with
// sound-slot/service-* module aliases.
//
    let mut preclaim_oss: static int = IS_ENABLED(CONFIG_SOUND_OSS_CORE_PRECLAIM);
    module_param(preclaim_oss, int, 0444);
    static int soundcore_open(struct inode *, struct file *);
    static const struct file_operations soundcore_fops =
    {
// We must have an owner or the module locking fails
    .owner	= THIS_MODULE,
    .open	= soundcore_open,
    .llseek = noop_llseek,
    };
//
// Low level list operator. Scan the ordered list, find a hole and
// join into it. Called with the lock asserted
//
#[no_mangle]
unsafe extern "C" fn __sound_insert_unit(s: *mut *mut sound_unit, list: *mut sound_unit, fops: *const file_operations, index: c_int, low: c_int, top: c_int) -> c_int {
    static int __sound_insert_unit(struct sound_unit * s, struct sound_unit **list, const struct file_operations *fops, int index, int low, int top)
    {
    let mut n: c_int = low;
    if (index < 0) {	/* first free */
    while (*list && (*list).unit_minor<n)
    list=&((*list).next);
    while(n<top)
    {
// Found a hole ?
    if(*list==core::ptr::null_mut() || (*list).unit_minor>n)
    break;
    list=&((*list).next);
    n+=SOUND_STEP;
    }
    if(n>=top)
    return -ENOENT;
    } else {
    n = low+(index*16);
    while (*list) {
    if ((*list).unit_minor==n)
    return -EBUSY;
    if ((*list).unit_minor>n)
    break;
    list=&((*list).next);
    }
    }
//
// Fill it in
//
    s.unit_minor=n;
    s.unit_fops=fops;
//
// Link it
//
    s.next=*list;
// list=s;
    return n;
    }
//
// Remove a node from the chain. Called with the lock asserted
//
    static struct sound_unit *__sound_remove_unit(struct sound_unit **list, int unit)
    {
    while(*list)
    {
    struct sound_unit *p=*list;
    if(p.unit_minor==unit)
    {
// list=p->next;
    return p;
    }
    list=&(p.next);
    }
    printk(KERN_ERR "Sound device %d went missing!\n", unit);
    return core::ptr::null_mut();
    }
//
// This lock guards the sound loader list.
//
    static DEFINE_SPINLOCK(sound_loader_lock);
//
// Allocate the controlling structure and add it to the sound driver
// list. Acquires locks as needed
//
#[no_mangle]
unsafe extern "C" fn sound_insert_unit(list: *mut sound_unit, fops: *const file_operations, index: c_int, low: c_int, top: c_int, name: *const c_char, mode: umode_t, dev: *mut device) -> c_int {
    static int sound_insert_unit(struct sound_unit **list, const struct file_operations *fops, int index, int low, int top, const char *name, umode_t mode, struct device *dev)
    {
    struct sound_unit *s = kmalloc_obj(*s);
    int r;
    if (!s)
    return -ENOMEM;
    spin_lock(&sound_loader_lock);
    retry:
    r = __sound_insert_unit(s, list, fops, index, low, top);
    spin_unlock(&sound_loader_lock);
    if (r < 0)
    goto fail;
#[no_mangle]
pub unsafe extern "C" fn if(SOUND_STEP: r <) -> else {
    else if (r < SOUND_STEP)
    sprintf(s.name, "sound/%s", name);
    else
    sprintf(s.name, "sound/%s%d", name, r / SOUND_STEP);
    if (!preclaim_oss) {
//
// Something else might have grabbed the minor.  If
// first free slot is requested, rescan with @low set
// to the next unit; otherwise, -EBUSY.
//
    r = __register_chrdev(SOUND_MAJOR, s.unit_minor, 1, s.name,
    &soundcore_fops);
    if (r < 0) {
    spin_lock(&sound_loader_lock);
    __sound_remove_unit(list, s.unit_minor);
    if (index < 0) {
    low = s.unit_minor + SOUND_STEP;
    goto retry;
    }
    spin_unlock(&sound_loader_lock);
    r = -EBUSY;
    goto fail;
    }
    }
    device_create(&sound_class, dev, MKDEV(SOUND_MAJOR, s.unit_minor),
    core::ptr::null_mut(), "%s", s.name+6);
    return s.unit_minor;
    fail:
    kfree(s);
    return r;
    }
//
// Remove a unit. Acquires locks as needed. The drivers MUST have
// completed the removal before their file operations become
// invalid.
//
#[no_mangle]
unsafe extern "C" fn sound_remove_unit(list: *mut sound_unit, unit: c_int) {
    static void sound_remove_unit(struct sound_unit **list, int unit)
    {
    struct sound_unit *p;
    spin_lock(&sound_loader_lock);
    p = __sound_remove_unit(list, unit);
    spin_unlock(&sound_loader_lock);
    if (p) {
    if (!preclaim_oss)
    __unregister_chrdev(SOUND_MAJOR, p.unit_minor, 1,
    p.name);
    device_destroy(&sound_class, MKDEV(SOUND_MAJOR, p.unit_minor));
    kfree(p);
    }
    }
//
// Allocations
//
// 0	*16		Mixers
// 1	*8		Sequencers
// 2	*16		Midi
// 3	*16		DSP
// 4	*16		SunDSP
// 5	*16		DSP16
// 6	--		sndstat (obsolete)
// 7	*16		unused
// 8	--		alternate sequencer (see above)
// 9	*16		raw synthesizer access
// 10	*16		unused
// 11	*16		unused
// 12	*16		unused
// 13	*16		unused
// 14	*16		unused
// 15	*16		unused
//
    static struct sound_unit *chains[SOUND_STEP];
//
// register_sound_special_device - register a special sound node
// @fops: File operations for the driver
// @unit: Unit number to allocate
// @dev: device pointer
//
// Allocate a special sound device by minor number from the sound
// subsystem.
//
// Return: The allocated number is returned on success. On failure,
// a negative error code is returned.
//
    int register_sound_special_device(const struct file_operations *fops, int unit,
    struct device *dev)
    {
    let mut chain: c_int = unit % SOUND_STEP;
    let mut max_unit: c_int = 256;
    const char *name;
    char _name[16];
    switch (chain) {
    case 0:
    name = "mixer";
    break;
    case 1:
    name = "sequencer";
    if (unit >= SOUND_STEP)
    goto __unknown;
    max_unit = unit + 1;
    break;
    case 2:
    name = "midi";
    break;
    case 3:
    name = "dsp";
    break;
    case 4:
    name = "audio";
    break;
    case 5:
    name = "dspW";
    break;
    case 8:
    name = "sequencer2";
    if (unit >= SOUND_STEP)
    goto __unknown;
    max_unit = unit + 1;
    break;
    case 9:
    name = "dmmidi";
    break;
    case 10:
    name = "dmfm";
    break;
    case 12:
    name = "adsp";
    break;
    case 13:
    name = "amidi";
    break;
    case 14:
    name = "admmidi";
    break;
    default:
    {
    __unknown:
    sprintf(_name, "unknown%d", chain);
    if (unit >= SOUND_STEP)
    strcat(_name, "-");
    name = _name;
    }
    break;
    }
    return sound_insert_unit(&chains[chain], fops, -1, unit, max_unit,
    name, 0600, dev);
    }
    EXPORT_SYMBOL(register_sound_special_device);
#[no_mangle]
pub unsafe extern "C" fn register_sound_special(fops: *const file_operations, unit: c_int) -> c_int {
    int register_sound_special(const struct file_operations *fops, int unit)
    {
    return register_sound_special_device(fops, unit, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(register_sound_special);
//
// register_sound_mixer - register a mixer device
// @fops: File operations for the driver
// @dev: Unit number to allocate
//
// Allocate a mixer device. Unit is the number of the mixer requested.
// Pass -1 to request the next free mixer unit.
//
// Return: On success, the allocated number is returned. On failure,
// a negative error code is returned.
//
#[no_mangle]
pub unsafe extern "C" fn register_sound_mixer(fops: *const file_operations, dev: c_int) -> c_int {
    int register_sound_mixer(const struct file_operations *fops, int dev)
    {
    return sound_insert_unit(&chains[0], fops, dev, 0, 128,
    "mixer", 0600, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(register_sound_mixer);
//
// DSP's are registered as a triple. Register only one and cheat
// in open - see below.
//
// register_sound_dsp - register a DSP device
// @fops: File operations for the driver
// @dev: Unit number to allocate
//
// Allocate a DSP device. Unit is the number of the DSP requested.
// Pass -1 to request the next free DSP unit.
//
// This function allocates both the audio and dsp device entries together
// and will always allocate them as a matching pair - eg dsp3/audio3
//
// Return: On success, the allocated number is returned. On failure,
// a negative error code is returned.
//
#[no_mangle]
pub unsafe extern "C" fn register_sound_dsp(fops: *const file_operations, dev: c_int) -> c_int {
    int register_sound_dsp(const struct file_operations *fops, int dev)
    {
    return sound_insert_unit(&chains[3], fops, dev, 3, 131,
    "dsp", 0600, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(register_sound_dsp);
//
// unregister_sound_special - unregister a special sound device
// @unit: unit number to allocate
//
// Release a sound device that was allocated with
// register_sound_special(). The unit passed is the return value from
// the register function.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_sound_special(unit: c_int) {
    void unregister_sound_special(int unit)
    {
    sound_remove_unit(&chains[unit % SOUND_STEP], unit);
    }
    EXPORT_SYMBOL(unregister_sound_special);
//
// unregister_sound_mixer - unregister a mixer
// @unit: unit number to allocate
//
// Release a sound device that was allocated with register_sound_mixer().
// The unit passed is the return value from the register function.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_sound_mixer(unit: c_int) {
    void unregister_sound_mixer(int unit)
    {
    sound_remove_unit(&chains[0], unit);
    }
    EXPORT_SYMBOL(unregister_sound_mixer);
//
// unregister_sound_dsp - unregister a DSP device
// @unit: unit number to allocate
//
// Release a sound device that was allocated with register_sound_dsp().
// The unit passed is the return value from the register function.
//
// Both of the allocated units are released together automatically.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_sound_dsp(unit: c_int) {
    void unregister_sound_dsp(int unit)
    {
    sound_remove_unit(&chains[3], unit);
    }
    EXPORT_SYMBOL(unregister_sound_dsp);
    static struct sound_unit *__look_for_unit(int chain, int unit)
    {
    struct sound_unit *s;
    s=chains[chain];
    while(s && s.unit_minor <= unit)
    {
    if(s.unit_minor==unit)
    return s;
    s=s.next;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn soundcore_open(inode: *mut inode, file: *mut file) -> c_int {
    static int soundcore_open(struct inode *inode, struct file *file)
    {
    int chain;
    let mut unit: c_int = iminor(inode);
    struct sound_unit *s;
    const struct file_operations *new_fops = core::ptr::null_mut();
    chain=unit&0x0F;
    if(chain==4 || chain==5)	/* dsp/audio/dsp16 */
    {
    unit&=0xF0;
    unit|=3;
    chain=3;
    }
    spin_lock(&sound_loader_lock);
    s = __look_for_unit(chain, unit);
    if (s)
    new_fops = fops_get(s.unit_fops);
    if (preclaim_oss && !new_fops) {
    spin_unlock(&sound_loader_lock);
//
// Please, don't change this order or code.
// For ALSA slot means soundcard and OSS emulation code
// comes as add-on modules which aren't depend on
// ALSA toplevel modules for soundcards, thus we need
// load them at first.	  [Jaroslav Kysela <perex@jcu.cz>]
//
    request_module("sound-slot-%i", unit>>4);
    request_module("sound-service-%i-%i", unit>>4, chain);
//
// sound-slot/service-* module aliases are scheduled
// for removal in favor of the standard char-major-
// module aliases.  For the time being, generate both
// the legacy and standard module aliases to ease
// transition.
//
    if (request_module("char-major-%d-%d", SOUND_MAJOR, unit) > 0)
    request_module("char-major-%d", SOUND_MAJOR);
    spin_lock(&sound_loader_lock);
    s = __look_for_unit(chain, unit);
    if (s)
    new_fops = fops_get(s.unit_fops);
    }
    spin_unlock(&sound_loader_lock);
    if (!new_fops)
    return -ENODEV;
//
// We rely upon the fact that we can't be unloaded while the
// subdriver is there.
//
    replace_fops(file, new_fops);
    if (!file.f_op.open)
    return -ENODEV;
    return file.f_op.open(inode, file);
    }
    MODULE_ALIAS_CHARDEV_MAJOR(SOUND_MAJOR);
#[no_mangle]
unsafe extern "C" fn cleanup_oss_soundcore() {
    static void cleanup_oss_soundcore(void)
    {
// We have nothing to really do here - we know the lists must be
    empty */
    unregister_chrdev(SOUND_MAJOR, "sound");
    }
#[no_mangle]
unsafe extern "C" fn init_oss_soundcore() -> int __init {
    static int __init init_oss_soundcore(void)
    {
    if (preclaim_oss &&
    register_chrdev(SOUND_MAJOR, "sound", &soundcore_fops) < 0) {
    printk(KERN_ERR "soundcore: sound device already in use.\n");
    return -EBUSY;
    }
    return 0;
    }
