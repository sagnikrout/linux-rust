//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/via/via-core.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 1998-2009 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
// Copyright 2009 Jonathan Corbet <corbet@lwn.net>
//
// Core code for the Via multifunction framebuffer device.
//

//
// The default port config.
//
    static struct via_port_cfg adap_configs[] = {
    [VIA_PORT_26]	= { VIA_PORT_I2C,  VIA_MODE_I2C, VIASR, 0x26 },
    [VIA_PORT_31]	= { VIA_PORT_I2C,  VIA_MODE_I2C, VIASR, 0x31 },
    [VIA_PORT_25]	= { VIA_PORT_GPIO, VIA_MODE_GPIO, VIASR, 0x25 },
    [VIA_PORT_2C]	= { VIA_PORT_GPIO, VIA_MODE_I2C, VIASR, 0x2c },
    [VIA_PORT_3D]	= { VIA_PORT_GPIO, VIA_MODE_GPIO, VIASR, 0x3d },
    { 0, 0, 0, 0 }
    };
//
// The OLPC XO-1.5 puts the camera power and reset lines onto
// GPIO 2C.
//
    static struct via_port_cfg olpc_adap_configs[] = {
    [VIA_PORT_26]	= { VIA_PORT_I2C,  VIA_MODE_I2C, VIASR, 0x26 },
    [VIA_PORT_31]	= { VIA_PORT_I2C,  VIA_MODE_I2C, VIASR, 0x31 },
    [VIA_PORT_25]	= { VIA_PORT_GPIO, VIA_MODE_GPIO, VIASR, 0x25 },
    [VIA_PORT_2C]	= { VIA_PORT_GPIO, VIA_MODE_GPIO, VIASR, 0x2c },
    [VIA_PORT_3D]	= { VIA_PORT_GPIO, VIA_MODE_GPIO, VIASR, 0x3d },
    { 0, 0, 0, 0 }
    };
//
// We currently only support one viafb device (will there ever be
// more than one?), so just declare it globally here.
//
    static struct viafb_dev global_dev;
//
// Basic register access; spinlock required.
//
#[no_mangle]
pub unsafe extern "C" fn viafb_mmio_write(reg: c_int, v: u32) {
    static inline void viafb_mmio_write(int reg, u32 v)
    {
    iowrite32(v, global_dev.engine_mmio + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn viafb_mmio_read(reg: c_int) -> c_int {
    static inline int viafb_mmio_read(int reg)
    {
    return ioread32(global_dev.engine_mmio + reg);
    }
// ----------------------------------------------------------------------
//
// Interrupt management.  We have a single IRQ line for a lot of
// different functions, so we need to share it.  The design here
// is that we don't want to reimplement the shared IRQ code here;
// we also want to avoid having contention for a single handler thread.
// So each subdev driver which needs interrupts just requests
// them directly from the kernel.  We just have what's needed for
// overall access to the interrupt control register.
//
// Which interrupts are enabled now?
//
    static u32 viafb_enabled_ints;
#[no_mangle]
unsafe extern "C" fn viafb_int_init() {
    static void viafb_int_init(void)
    {
    viafb_enabled_ints = 0;
    viafb_mmio_write(VDE_INTERRUPT, 0);
    }
//
// Allow subdevs to ask for specific interrupts to be enabled.  These
// functions must be called with reg_lock held
//
#[no_mangle]
pub unsafe extern "C" fn viafb_irq_enable(mask: u32) {
    void viafb_irq_enable(u32 mask)
    {
    viafb_enabled_ints |= mask;
    viafb_mmio_write(VDE_INTERRUPT, viafb_enabled_ints | VDE_I_ENABLE);
    }
    EXPORT_SYMBOL_GPL(viafb_irq_enable);
#[no_mangle]
pub unsafe extern "C" fn viafb_irq_disable(mask: u32) {
    void viafb_irq_disable(u32 mask)
    {
    viafb_enabled_ints &= ~mask;
    if (viafb_enabled_ints == 0)
    viafb_mmio_write(VDE_INTERRUPT, 0);  /* Disable entirely */
    else
    viafb_mmio_write(VDE_INTERRUPT,
    viafb_enabled_ints | VDE_I_ENABLE);
    }
    EXPORT_SYMBOL_GPL(viafb_irq_disable);
// ----------------------------------------------------------------------
//
// Currently, the camera driver is the only user of the DMA code, so we
// only compile it in if the camera driver is being built.  Chances are,
// most viafb systems will not need to have this extra code for a while.
// As soon as another user comes long, the ifdef can be removed.
//

//
// Access to the DMA engine.  This currently provides what the camera
// driver needs (i.e. outgoing only) but is easily expandable if need
// be.
//
// There are four DMA channels in the vx855.  For now, we only
// use one of them, though.  Most of the time, the DMA channel
// will be idle, so we keep the IRQ handler unregistered except
// when some subsystem has indicated an interest.
//
    static int viafb_dma_users;
    static DECLARE_COMPLETION(viafb_dma_completion);
//
// This mutex protects viafb_dma_users and our global interrupt
// registration state; it also serializes access to the DMA
// engine.
//
    static DEFINE_MUTEX(viafb_dma_lock);
//
// The VX855 DMA descriptor (used for s/g transfers) looks
// like this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_vx855_dma_descr {
    pub /: *mut *mut u32 addr_low; / Low part of phys addr,
    pub /: *mut *mut u32 addr_high; / High 12 bits of addr,
    pub /: *mut *mut u32 fb_offset; / Offset into FB memory,
    pub /: *mut *mut u32 seg_size; / Size, 16-byte units,
    pub /: *mut *mut u32 tile_mode; / "tile mode" setting,
    pub /: *mut *mut u32 next_desc_low; / Next descriptor addr,
    pub next_desc_high: u32,
    pub /: *mut *mut u32 pad; / Fill out to 64 bytes,
}

//
// Flags added to the "next descriptor low" pointers
//
pub const VIAFB_DMA_MAGIC: c_uint = 0x01  /* ??? Just has to be there */;
pub const VIAFB_DMA_FINAL_SEGMENT: c_uint = 0x02  /* Final segment */;
//
// The completion IRQ handler.
//
#[no_mangle]
unsafe extern "C" fn viafb_dma_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t viafb_dma_irq(int irq, void *data)
    {
    int csr;
    let mut ret: irqreturn_t = IRQ_NONE;
    spin_lock(&global_dev.reg_lock);
    csr = viafb_mmio_read(VDMA_CSR0);
    if (csr & VDMA_C_DONE) {
    viafb_mmio_write(VDMA_CSR0, VDMA_C_DONE);
    complete(&viafb_dma_completion);
    ret = IRQ_HANDLED;
    }
    spin_unlock(&global_dev.reg_lock);
    return ret;
    }
//
// Indicate a need for DMA functionality.
//
#[no_mangle]
pub unsafe extern "C" fn viafb_request_dma() -> c_int {
    int viafb_request_dma(void)
    {
    let mut ret: c_int = 0;
//
// Only VX855 is supported currently.
//
    if (global_dev.chip_type != UNICHROME_VX855)
    return -ENODEV;
//
// Note the new user and set up our interrupt handler
// if need be.
//
    mutex_lock(&viafb_dma_lock);
    viafb_dma_users++;
    if (viafb_dma_users == 1) {
    ret = request_irq(global_dev.pdev.irq, viafb_dma_irq,
    IRQF_SHARED, "via-dma", &viafb_dma_users);
    if (ret)
    viafb_dma_users--;
    else
    viafb_irq_enable(VDE_I_DMA0TDEN);
    }
    mutex_unlock(&viafb_dma_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(viafb_request_dma);
#[no_mangle]
pub unsafe extern "C" fn viafb_release_dma() {
    void viafb_release_dma(void)
    {
    mutex_lock(&viafb_dma_lock);
    viafb_dma_users--;
    if (viafb_dma_users == 0) {
    viafb_irq_disable(VDE_I_DMA0TDEN);
    free_irq(global_dev.pdev.irq, &viafb_dma_users);
    }
    mutex_unlock(&viafb_dma_lock);
    }
    EXPORT_SYMBOL_GPL(viafb_release_dma);
//
// Do a scatter/gather DMA copy from FB memory.  You must have done
// a successful call to viafb_request_dma() first.
//
#[no_mangle]
pub unsafe extern "C" fn viafb_dma_copy_out_sg(offset: c_uint, sg: *mut scatterlist, nsg: c_int) -> c_int {
    int viafb_dma_copy_out_sg(unsigned int offset, struct scatterlist *sg, int nsg)
    {
    struct viafb_vx855_dma_descr *descr;
    void *descrpages;
    dma_addr_t descr_handle;
    unsigned long flags;
    int i;
    let mut ret: c_int = 0;
    struct scatterlist *sgentry;
    dma_addr_t nextdesc;
//
// Get a place to put the descriptors.
//
    descrpages = dma_alloc_coherent(&global_dev.pdev.dev,
    nsg*sizeof(struct viafb_vx855_dma_descr),
    &descr_handle, GFP_KERNEL);
    if (descrpages == core::ptr::null_mut()) {
    dev_err(&global_dev.pdev.dev, "Unable to get descr page.\n");
    return -ENOMEM;
    }
    mutex_lock(&viafb_dma_lock);
//
// Fill them in.
//
    descr = descrpages;
    nextdesc = descr_handle + sizeof(struct viafb_vx855_dma_descr);
    for_each_sg(sg, sgentry, nsg, i) {
    let mut paddr: dma_addr_t = sg_dma_address(sgentry);
    descr.addr_low = paddr & 0xfffffff0;
    descr.addr_high = ((u64) paddr >> 32) & 0x0fff;
    descr.fb_offset = offset;
    descr.seg_size = sg_dma_len(sgentry) >> 4;
    descr.tile_mode = 0;
    descr.next_desc_low = (nextdesc&0xfffffff0) | VIAFB_DMA_MAGIC;
    descr.next_desc_high = ((u64) nextdesc >> 32) & 0x0fff;
    descr.pad = 0xffffffff;  /* VIA driver does this */
    offset += sg_dma_len(sgentry);
    nextdesc += sizeof(struct viafb_vx855_dma_descr);
    descr++;
    }
    descr[-1].next_desc_low = VIAFB_DMA_FINAL_SEGMENT|VIAFB_DMA_MAGIC;
//
// Program the engine.
//
    spin_lock_irqsave(&global_dev.reg_lock, flags);
    init_completion(&viafb_dma_completion);
    viafb_mmio_write(VDMA_DQWCR0, 0);
    viafb_mmio_write(VDMA_CSR0, VDMA_C_ENABLE|VDMA_C_DONE);
    viafb_mmio_write(VDMA_MR0, VDMA_MR_TDIE | VDMA_MR_CHAIN);
    viafb_mmio_write(VDMA_DPRL0, descr_handle | VIAFB_DMA_MAGIC);
    viafb_mmio_write(VDMA_DPRH0,
    (((u64)descr_handle >> 32) & 0x0fff) | 0xf0000);
    (void) viafb_mmio_read(VDMA_CSR0);
    viafb_mmio_write(VDMA_CSR0, VDMA_C_ENABLE|VDMA_C_START);
    spin_unlock_irqrestore(&global_dev.reg_lock, flags);
//
// Now we just wait until the interrupt handler says
// we're done.  Except that, actually, we need to wait a little
// longer: the interrupts seem to jump the gun a little and we
// get corrupted frames sometimes.
//
    wait_for_completion_timeout(&viafb_dma_completion, 1);
    msleep(1);
    if ((viafb_mmio_read(VDMA_CSR0) & VDMA_C_DONE) == 0) {
    printk(KERN_ERR "VIA DMA timeout!\n");
    ret = -ETIMEDOUT;
    }
//
// Clean up and we're done.
//
    viafb_mmio_write(VDMA_CSR0, VDMA_C_DONE);
    viafb_mmio_write(VDMA_MR0, 0); /* Reset int enable */
    mutex_unlock(&viafb_dma_lock);
    dma_free_coherent(&global_dev.pdev.dev,
    nsg*sizeof(struct viafb_vx855_dma_descr), descrpages,
    descr_handle);
    return ret;
    }
    EXPORT_SYMBOL_GPL(viafb_dma_copy_out_sg);

// ----------------------------------------------------------------------
//
// Figure out how big our framebuffer memory is.  Kind of ugly,
// but evidently we can't trust the information found in the
// fbdev configuration area.
//
    static u16 via_function3[] = {
    CLE266_FUNCTION3, KM400_FUNCTION3, CN400_FUNCTION3, CN700_FUNCTION3,
    CX700_FUNCTION3, KM800_FUNCTION3, KM890_FUNCTION3, P4M890_FUNCTION3,
    P4M900_FUNCTION3, VX800_FUNCTION3, VX855_FUNCTION3, VX900_FUNCTION3,
    };
// Get the BIOS-configured framebuffer size from PCI configuration space
// of function 3 in the respective chipset
#[no_mangle]
unsafe extern "C" fn viafb_get_fb_size_from_pci(chip_type: c_int) -> c_int {
    static int viafb_get_fb_size_from_pci(int chip_type)
    {
    int i;
    let mut offset: u8 = 0;
    u32 FBSize;
    u32 VideoMemSize;
// search for the "FUNCTION3" device in this chipset
    for (i = 0; i < ARRAY_SIZE(via_function3); i++) {
    struct pci_dev *pdev;
    pdev = pci_get_device(PCI_VENDOR_ID_VIA, via_function3[i],
    core::ptr::null_mut());
    if (!pdev)
    continue;
    DEBUG_MSG(KERN_INFO "Device ID = %x\n", pdev.device);
    switch (pdev.device) {
    case CLE266_FUNCTION3:
    case KM400_FUNCTION3:
    offset = 0xE0;
    break;
    case CN400_FUNCTION3:
    case CN700_FUNCTION3:
    case CX700_FUNCTION3:
    case KM800_FUNCTION3:
    case KM890_FUNCTION3:
    case P4M890_FUNCTION3:
    case P4M900_FUNCTION3:
    case VX800_FUNCTION3:
    case VX855_FUNCTION3:
    case VX900_FUNCTION3:
// case CN750_FUNCTION3:
    offset = 0xA0;
    break;
    }
    if (!offset)
    break;
    pci_read_config_dword(pdev, offset, &FBSize);
    pci_dev_put(pdev);
    }
    if (!offset) {
    printk(KERN_ERR "cannot determine framebuffer size\n");
    return -EIO;
    }
    FBSize = FBSize & 0x00007000;
    DEBUG_MSG(KERN_INFO "FB Size = %x\n", FBSize);
    if (chip_type < UNICHROME_CX700) {
    switch (FBSize) {
    case 0x00004000:
    VideoMemSize = (16 << 20);	/*16M */
    break;
    case 0x00005000:
    VideoMemSize = (32 << 20);	/*32M */
    break;
    case 0x00006000:
    VideoMemSize = (64 << 20);	/*64M */
    break;
    default:
    VideoMemSize = (32 << 20);	/*32M */
    break;
    }
    } else {
    switch (FBSize) {
    case 0x00001000:
    VideoMemSize = (8 << 20);	/*8M */
    break;
    case 0x00002000:
    VideoMemSize = (16 << 20);	/*16M */
    break;
    case 0x00003000:
    VideoMemSize = (32 << 20);	/*32M */
    break;
    case 0x00004000:
    VideoMemSize = (64 << 20);	/*64M */
    break;
    case 0x00005000:
    VideoMemSize = (128 << 20);	/*128M */
    break;
    case 0x00006000:
    VideoMemSize = (256 << 20);	/*256M */
    break;
    case 0x00007000:	/* Only on VX855/875 */
    VideoMemSize = (512 << 20);	/*512M */
    break;
    default:
    VideoMemSize = (32 << 20);	/*32M */
    break;
    }
    }
    return VideoMemSize;
    }
//
// Figure out and map our MMIO regions.
//
#[no_mangle]
unsafe extern "C" fn via_pci_setup_mmio(vdev: *mut viafb_dev) -> c_int {
    static int via_pci_setup_mmio(struct viafb_dev *vdev)
    {
    int ret;
//
// Hook up to the device registers.  Note that we soldier
// on if it fails; the framebuffer can operate (without
// acceleration) without this region.
//
    vdev.engine_start = pci_resource_start(vdev.pdev, 1);
    vdev.engine_len = pci_resource_len(vdev.pdev, 1);
    vdev.engine_mmio = ioremap(vdev.engine_start,
    vdev.engine_len);
    if (vdev.engine_mmio == core::ptr::null_mut())
    dev_err(&vdev.pdev.dev,
    "Unable to map engine MMIO; operation will be "
    "slow and crippled.\n");
//
// Map in framebuffer memory.  For now, failure here is
// fatal.  Unfortunately, in the absence of significant
// vmalloc space, failure here is also entirely plausible.
// Eventually we want to move away from mapping this
// entire region.
//
    if (vdev.chip_type == UNICHROME_VX900)
    vdev.fbmem_start = pci_resource_start(vdev.pdev, 2);
    else
    vdev.fbmem_start = pci_resource_start(vdev.pdev, 0);
    ret = vdev.fbmem_len = viafb_get_fb_size_from_pci(vdev.chip_type);
    if (ret < 0)
    goto out_unmap;
// try to map less memory on failure, 8 MB should be still enough
    for (; vdev.fbmem_len >= 8 << 20; vdev.fbmem_len /= 2) {
    vdev.fbmem = ioremap_wc(vdev.fbmem_start, vdev.fbmem_len);
    if (vdev.fbmem)
    break;
    }
    if (vdev.fbmem == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto out_unmap;
    }
    return 0;
    out_unmap:
    iounmap(vdev.engine_mmio);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn via_pci_teardown_mmio(vdev: *mut viafb_dev) {
    static void via_pci_teardown_mmio(struct viafb_dev *vdev)
    {
    iounmap(vdev.fbmem);
    iounmap(vdev.engine_mmio);
    }
//
// Create our subsidiary devices.
//
    static struct viafb_subdev_info {
    char *name;
    struct platform_device *platdev;
    } viafb_subdevs[] = {
    {
    .name = "viafb-gpio",
    },
    {
    .name = "viafb-i2c",
    },

    {
    .name = "viafb-camera",
    },

    };

    static int via_create_subdev(struct viafb_dev *vdev,
    struct viafb_subdev_info *info)
    {
    int ret;
    info.platdev = platform_device_alloc(info.name, -1);
    if (!info.platdev) {
    dev_err(&vdev.pdev.dev, "Unable to allocate pdev %s\n",
    info.name);
    return -ENOMEM;
    }
    info.platdev.dev.parent = &vdev.pdev.dev;
    info.platdev.dev.platform_data = vdev;
    ret = platform_device_add(info.platdev);
    if (ret) {
    dev_err(&vdev.pdev.dev, "Unable to add pdev %s\n",
    info.name);
    platform_device_put(info.platdev);
    info.platdev = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn via_setup_subdevs(vdev: *mut viafb_dev) -> c_int {
    static int via_setup_subdevs(struct viafb_dev *vdev)
    {
    int i;
//
// Ignore return values.  Even if some of the devices
// fail to be created, we'll still be able to use some
// of the rest.
//
    for (i = 0; i < N_SUBDEVS; i++)
    via_create_subdev(vdev, viafb_subdevs + i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn via_teardown_subdevs() {
    static void via_teardown_subdevs(void)
    {
    int i;
    for (i = 0; i < N_SUBDEVS; i++)
    if (viafb_subdevs[i].platdev) {
    viafb_subdevs[i].platdev.dev.platform_data = core::ptr::null_mut();
    platform_device_unregister(viafb_subdevs[i].platdev);
    }
    }
//
// Power management functions
//
    static __maybe_unused LIST_HEAD(viafb_pm_hooks);
    static __maybe_unused DEFINE_MUTEX(viafb_pm_hooks_lock);
#[no_mangle]
pub unsafe extern "C" fn viafb_pm_register(hooks: *mut viafb_pm_hooks) {
    void viafb_pm_register(struct viafb_pm_hooks *hooks)
    {
    INIT_LIST_HEAD(&hooks.list);
    mutex_lock(&viafb_pm_hooks_lock);
    list_add_tail(&hooks.list, &viafb_pm_hooks);
    mutex_unlock(&viafb_pm_hooks_lock);
    }
    EXPORT_SYMBOL_GPL(viafb_pm_register);
#[no_mangle]
pub unsafe extern "C" fn viafb_pm_unregister(hooks: *mut viafb_pm_hooks) {
    void viafb_pm_unregister(struct viafb_pm_hooks *hooks)
    {
    mutex_lock(&viafb_pm_hooks_lock);
    list_del(&hooks.list);
    mutex_unlock(&viafb_pm_hooks_lock);
    }
    EXPORT_SYMBOL_GPL(viafb_pm_unregister);
#[no_mangle]
unsafe extern "C" fn via_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused via_suspend(struct device *dev)
    {
    struct viafb_pm_hooks *hooks;
//
// "I've occasionally hit a few drivers that caused suspend
// failures, and each and every time it was a driver bug, and
// the right thing to do was to just ignore the error and suspend
// anyway - returning an error code and trying to undo the suspend
// is not what anybody ever really wants, even if our model
// _allows_ for it."
// -- Linus Torvalds, Dec. 7, 2009
//
    mutex_lock(&viafb_pm_hooks_lock);
    list_for_each_entry_reverse(hooks, &viafb_pm_hooks, list)
    hooks.suspend(hooks.private);
    mutex_unlock(&viafb_pm_hooks_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn via_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused via_resume(struct device *dev)
    {
    struct viafb_pm_hooks *hooks;
// Now bring back any subdevs
    mutex_lock(&viafb_pm_hooks_lock);
    list_for_each_entry(hooks, &viafb_pm_hooks, list)
    hooks.resume(hooks.private);
    mutex_unlock(&viafb_pm_hooks_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn via_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int via_pci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    int ret;
    ret = aperture_remove_conflicting_pci_devices(pdev, "viafb");
    if (ret)
    return ret;
    ret = pci_enable_device(pdev);
    if (ret)
    return ret;
//
// Global device initialization.
//
    memset(&global_dev, 0, sizeof(global_dev));
    global_dev.pdev = pdev;
    global_dev.chip_type = ent.driver_data;
    global_dev.port_cfg = adap_configs;
    if (machine_is_olpc())
    global_dev.port_cfg = olpc_adap_configs;
    spin_lock_init(&global_dev.reg_lock);
    ret = via_pci_setup_mmio(&global_dev);
    if (ret)
    goto out_disable;
//
// Set up interrupts and create our subdevices.  Continue even if
// some things fail.
//
    viafb_int_init();
    via_setup_subdevs(&global_dev);
//
// Set up the framebuffer device
//
    ret = via_fb_pci_probe(&global_dev);
    if (ret)
    goto out_subdevs;
    return 0;
    out_subdevs:
    via_teardown_subdevs();
    via_pci_teardown_mmio(&global_dev);
    out_disable:
    pci_disable_device(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn via_pci_remove(pdev: *mut pci_dev) {
    static void via_pci_remove(struct pci_dev *pdev)
    {
    via_teardown_subdevs();
    via_fb_pci_remove(pdev);
    via_pci_teardown_mmio(&global_dev);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id via_pci_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_CLE266_DID),
    .driver_data = UNICHROME_CLE266 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_K400_DID),
    .driver_data = UNICHROME_K400 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_K800_DID),
    .driver_data = UNICHROME_K800 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_PM800_DID),
    .driver_data = UNICHROME_PM800 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_CN700_DID),
    .driver_data = UNICHROME_CN700 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_CX700_DID),
    .driver_data = UNICHROME_CX700 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_CN750_DID),
    .driver_data = UNICHROME_CN750 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_K8M890_DID),
    .driver_data = UNICHROME_K8M890 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_P4M890_DID),
    .driver_data = UNICHROME_P4M890 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_P4M900_DID),
    .driver_data = UNICHROME_P4M900 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_VX800_DID),
    .driver_data = UNICHROME_VX800 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_VX855_DID),
    .driver_data = UNICHROME_VX855 },
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, UNICHROME_VX900_DID),
    .driver_data = UNICHROME_VX900 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, via_pci_table);
    static const struct dev_pm_ops via_pm_ops = {

    .suspend	= via_suspend,
    .resume		= via_resume,
    .freeze		= core::ptr::null_mut(),
    .thaw		= via_resume,
    .poweroff	= core::ptr::null_mut(),
    .restore	= via_resume,

    };
    static struct pci_driver via_driver = {
    .name		= "viafb",
    .id_table	= via_pci_table,
    .probe		= via_pci_probe,
    .remove		= via_pci_remove,
    .driver.pm	= &via_pm_ops,
    };
#[no_mangle]
unsafe extern "C" fn via_core_init() -> int __init {
    static int __init via_core_init(void)
    {
    int ret;
    if (fb_modesetting_disabled("viafb"))
    return -ENODEV;
    ret = viafb_init();
    if (ret)
    return ret;
    viafb_i2c_init();
    viafb_gpio_init();
    ret = pci_register_driver(&via_driver);
    if (ret) {
    viafb_gpio_exit();
    viafb_i2c_exit();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn via_core_exit() -> void __exit {
    static void __exit via_core_exit(void)
    {
    pci_unregister_driver(&via_driver);
    viafb_gpio_exit();
    viafb_i2c_exit();
    viafb_exit();
    }
    module_init(via_core_init);
    module_exit(via_core_exit);
