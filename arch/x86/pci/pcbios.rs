//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/pcbios.c
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
// BIOS32 and PCI BIOS handling.
//

// BIOS32 signature: "_32_"

// PCI signature: "PCI "

// PCI service signature: "$PCI"

// PCI BIOS hardware mechanism flags
pub const PCIBIOS_HW_TYPE1: c_uint = 0x01;
pub const PCIBIOS_HW_TYPE2: c_uint = 0x02;
pub const PCIBIOS_HW_TYPE1_SPEC: c_uint = 0x10;
pub const PCIBIOS_HW_TYPE2_SPEC: c_uint = 0x20;
//
// Returned in EAX:
// - AH: return code
//

    int pcibios_enabled;
#[no_mangle]
unsafe extern "C" fn pcibios_get_return_code(eax: u32) -> u8 {
    static u8 pcibios_get_return_code(u32 eax)
    {
    return FIELD_GET(PCIBIOS_RETURN_CODE, eax);
    }
// According to the BIOS specification at:
// http://members.datafast.net.au/dft0802/specs/bios21.pdf, we could
// restrict the x zone to some pages and make it ro. But this may be
// broken on some bios, complex to handle with static_protections.
// We could make the 0xe0000-0x100000 range rox, but this can break
// some ISA mapping.
//
// So we let's an rw and x hole when pcibios is used. This shouldn't
// happen for modern system with mmconfig, and if you don't want it
// you could disable pcibios...
//
#[no_mangle]
pub unsafe extern "C" fn set_bios_x() {
    static inline void set_bios_x(void)
    {
    pcibios_enabled = 1;
    set_memory_x(PAGE_OFFSET + BIOS_BEGIN, (BIOS_END - BIOS_BEGIN) >> PAGE_SHIFT);
    if (__supported_pte_mask & _PAGE_NX)
    printk(KERN_INFO "PCI: PCI BIOS area is rw and x. Use pci=nobios if you want it NX.\n");
    }
//
// This is the standard structure used to identify the entry point
// to the BIOS32 Service Directory, as documented in
// Standard BIOS 32-bit Service Directory Proposal
// Revision 0.4 May 24, 1993
// Phoenix Technologies Ltd.
// Norwood, MA
// and the PCI BIOS specification.
//
    union bios32 {
    struct {
    unsigned long signature;	/* _32_ */
    unsigned long entry;		/* 32 bit physical address */
    unsigned char revision;		/* Revision level, 0 */
    unsigned char length;		/* Length in paragraphs should be 01 */
    unsigned char checksum;		/* All bytes must add up to zero */
    unsigned char reserved[5]; 	/* Must be zero */
    } fields;
    char chars[16];
    };
//
// Physical address of the service directory.  I don't know if we're
// allowed to have more than one of these or not, so just in case
// we'll make pcibios_present() take a memory start parameter and store
// the array there.
//
    static struct {
    unsigned long address;
    unsigned short segment;
    } bios32_indirect __initdata = { 0, __KERNEL_CS };
//
// Returns the entry point for the given service, NULL on error
//
#[no_mangle]
unsafe extern "C" fn bios32_service(service: c_ulong) -> unsigned long __init {
    static unsigned long __init bios32_service(unsigned long service)
    {
    unsigned char return_code;	/* %al */
    unsigned long address;		/* %ebx */
    unsigned long length;		/* %ecx */
    unsigned long entry;		/* %edx */
    unsigned long flags;
    local_irq_save(flags);
    __asm__("lcall *(%%edi); cld"
    : "=a" (return_code),
    "=b" (address),
    "=c" (length),
    "=d" (entry)
    : "0" (service),
    "1" (0),
    "D" (&bios32_indirect));
    local_irq_restore(flags);
    switch (return_code) {
    case 0:
    return address + entry;
    case 0x80:	/* Not present */
    printk(KERN_WARNING "bios32_service(0x%lx): not present\n", service);
    return 0;
    default: /* Shouldn't happen */
    printk(KERN_WARNING "bios32_service(0x%lx): returned 0x%x -- BIOS bug!\n",
    service, return_code);
    return 0;
    }
    }
    static struct {
    unsigned long address;
    unsigned short segment;
    } pci_indirect __ro_after_init = {
    .address = 0,
    .segment = __KERNEL_CS,
    };
    static int pci_bios_present __ro_after_init;
#[no_mangle]
unsafe extern "C" fn check_pcibios() -> int __init {
    static int __init check_pcibios(void)
    {
    u32 signature, eax, ebx, ecx;
    u8 status, major_ver, minor_ver, hw_mech;
    unsigned long flags, pcibios_entry;
    if ((pcibios_entry = bios32_service(PCI_SERVICE))) {
    pci_indirect.address = pcibios_entry + PAGE_OFFSET;
    local_irq_save(flags);
    __asm__(
    "lcall *(%%edi); cld\n\t"
    "jc 1f\n\t"
    "xor %%ah, %%ah\n"
    "1:"
    : "=d" (signature),
    "=a" (eax),
    "=b" (ebx),
    "=c" (ecx)
    : "1" (PCIBIOS_PCI_BIOS_PRESENT),
    "D" (&pci_indirect)
    : "memory");
    local_irq_restore(flags);
    status = pcibios_get_return_code(eax);
    hw_mech = eax & 0xff;
    major_ver = (ebx >> 8) & 0xff;
    minor_ver = ebx & 0xff;
    if (pcibios_last_bus < 0)
    pcibios_last_bus = ecx & 0xff;
    DBG("PCI: BIOS probe returned s=%02x hw=%02x ver=%02x.%02x l=%02x\n",
    status, hw_mech, major_ver, minor_ver, pcibios_last_bus);
    if (status || signature != PCI_SIGNATURE) {
    printk (KERN_ERR "PCI: BIOS BUG #%x[%08x] found\n",
    status, signature);
    return 0;
    }
    printk(KERN_INFO "PCI: PCI BIOS revision %x.%02x entry at 0x%lx, last bus=%d\n",
    major_ver, minor_ver, pcibios_entry, pcibios_last_bus);

    if (!(hw_mech & PCIBIOS_HW_TYPE1))
    pci_probe &= ~PCI_PROBE_CONF1;
    if (!(hw_mech & PCIBIOS_HW_TYPE2))
    pci_probe &= ~PCI_PROBE_CONF2;

    return 1;
    }
    return 0;
    }
    static int pci_bios_read(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 *value)
    {
    let mut result: c_ulong = 0;
    unsigned long flags;
    let mut bx: c_ulong = (bus << 8) | devfn;
    let mut number: u16 = 0, mask = 0;
    WARN_ON(seg);
    if (!value || (bus > 255) || (devfn > 255) || (reg > 255))
    return -EINVAL;
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    switch (len) {
    case 1:
    number = PCIBIOS_READ_CONFIG_BYTE;
    mask = 0xff;
    break;
    case 2:
    number = PCIBIOS_READ_CONFIG_WORD;
    mask = 0xffff;
    break;
    case 4:
    number = PCIBIOS_READ_CONFIG_DWORD;
    break;
    }
    __asm__("lcall *(%%esi); cld\n\t"
    "jc 1f\n\t"
    "xor %%ah, %%ah\n"
    "1:"
    : "=c" (*value),
    "=a" (result)
    : "1" (number),
    "b" (bx),
    "D" ((long)reg),
    "S" (&pci_indirect));
//
// Zero-extend the result beyond 8 or 16 bits, do not trust the
// BIOS having done it:
//
    if (mask)
// value &= mask;
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    return pcibios_get_return_code(result);
    }
    static int pci_bios_write(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 value)
    {
    let mut result: c_ulong = 0;
    unsigned long flags;
    let mut bx: c_ulong = (bus << 8) | devfn;
    let mut number: u16 = 0;
    WARN_ON(seg);
    if ((bus > 255) || (devfn > 255) || (reg > 255))
    return -EINVAL;
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    switch (len) {
    case 1:
    number = PCIBIOS_WRITE_CONFIG_BYTE;
    break;
    case 2:
    number = PCIBIOS_WRITE_CONFIG_WORD;
    break;
    case 4:
    number = PCIBIOS_WRITE_CONFIG_DWORD;
    break;
    }
    __asm__("lcall *(%%esi); cld\n\t"
    "jc 1f\n\t"
    "xor %%ah, %%ah\n"
    "1:"
    : "=a" (result)
    : "0" (number),
    "c" (value),
    "b" (bx),
    "D" ((long)reg),
    "S" (&pci_indirect));
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    return pcibios_get_return_code(result);
    }
//
// Function table for BIOS32 access
//
    static const struct pci_raw_ops pci_bios_access = {
    .read =		pci_bios_read,
    .write =	pci_bios_write
    };
//
// Try to find PCI BIOS.
//
#[no_mangle]
unsafe extern "C" fn pci_find_bios() -> *const pci_raw_ops __init {
    static const struct pci_raw_ops *__init pci_find_bios(void)
    {
    union bios32 *check;
    unsigned char sum;
    int i, length;
//
// Follow the standard procedure for locating the BIOS32 Service
// directory by scanning the permissible address range from
// 0xe0000 through 0xfffff for a valid BIOS32 structure.
//
    for (check = (union bios32 *) __va(0xe0000);
    check <= (union bios32 *) __va(0xffff0);
    ++check) {
    long sig;
    if (get_kernel_nofault(sig, &check.fields.signature))
    continue;
    if (check.fields.signature != BIOS32_SIGNATURE)
    continue;
    length = check.fields.length * 16;
    if (!length)
    continue;
    sum = 0;
    for (i = 0; i < length ; ++i)
    sum += check.chars[i];
    if (sum != 0)
    continue;
    if (check.fields.revision != 0) {
    printk("PCI: unsupported BIOS32 revision %d at 0x%p\n",
    check.fields.revision, check);
    continue;
    }
    DBG("PCI: BIOS32 Service Directory structure at 0x%p\n", check);
    if (check.fields.entry >= 0x100000) {
    printk("PCI: BIOS32 entry (0x%p) in high memory, "
    "cannot use.\n", check);
    return core::ptr::null_mut();
    } else {
    let mut bios32_entry: c_ulong = check.fields.entry;
    DBG("PCI: BIOS32 Service Directory entry at 0x%lx\n",
    bios32_entry);
    bios32_indirect.address = bios32_entry + PAGE_OFFSET;
    set_bios_x();
    if (check_pcibios())
    return &pci_bios_access;
    }
    break;	/* Hopefully more than one BIOS32 cannot happen... */
    }
    return core::ptr::null_mut();
    }
//
// BIOS Functions for IRQ Routing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_routing_options {
    pub size: u16,
    pub table: *mut irq_info,
    pub segment: u16,
    pub __attribute__((packed)): },
#[no_mangle]
pub unsafe extern "C" fn pcibios_get_irq_routing_table() -> *mut irq_routing_table {
    struct irq_routing_table * pcibios_get_irq_routing_table(void)
    {
    pub opt: irq_routing_options,
    pub NULL: *mut *mut irq_routing_table rt =,
    pub map: int ret,,
    pub page: c_ulong,
    if (!pci_bios_present)
    pub NULL: return,
    pub __get_free_page(GFP_KERNEL): page =,
    if (!page)
    pub NULL: return,
    pub page: *mut *mut opt.table = (struct irq_info ),
    pub PAGE_SIZE: opt.size =,
    pub __KERNEL_DS: opt.segment =,
    pub "): DBG("PCI: Fetching IRQ routing table...,
    __asm__("push %%es\n\t"
    "push %%ds\n\t"
    "pop  %%es\n\t"
    pub cld\n\t": *mut *mut "lcall (%%esi);,
    "pop %%es\n\t"
    "jc 1f\n\t"
    "xor %%ah, %%ah\n"
    "1:"
    : "=a" (ret),
    "=b" (map),
    "=m" (opt)
    : "0" (PCIBIOS_GET_ROUTING_OPTIONS),
    "1" (0),
    "D" ((long) &opt),
    "S" (&pci_indirect),
    "m" (opt)
    pub "memory"): :,
    pub map): DBG("OK ret=%d, size=%d, map=%x\n", ret, opt.size,,
    pub pcibios_get_return_code(ret): ret =,
    if (ret) {
    pub ret): printk(KERN_ERR "PCI: Error %02x when fetching IRQ routing table.\n",,
    } else if (opt.size) {
    pub GFP_KERNEL): rt = kmalloc(sizeof(struct irq_routing_table) + opt.size,,
    if (rt) {
    pub irq_routing_table)): memset(rt, 0, sizeof(struct,
    pub irq_routing_table): rt->size = opt.size + sizeof(struct,
    pub map: rt->exclusive_irqs =,
    pub opt.size): *mut *mut memcpy(rt->slots, (void ) page,,
    pub Table\n"): printk(KERN_INFO "PCI: Using BIOS Interrupt Routing,
    }
    }
    pub rt: return,
    }
#[no_mangle]
pub unsafe extern "C" fn pcibios_set_irq_routing(dev: *mut pci_dev, pin: c_int, irq: c_int) -> c_int {
    int pcibios_set_irq_routing(struct pci_dev *dev, int pin, int irq)
    {
    pub ret: c_int,
    pub cld\n\t": *mut *mut __asm__("lcall (%%esi);,
    "jc 1f\n\t"
    "xor %%ah, %%ah\n"
    "1:"
    : "=a" (ret)
    : "0" (PCIBIOS_SET_PCI_HW_INT),
    "b" ((dev.bus.number << 8) | dev.devfn),
    "c" ((irq << 8) | (pin + 10)),
    pub (&pci_indirect)): "S",
    pub PCIBIOS_SUCCESSFUL: return pcibios_get_return_code(ret) ==,
    }
#[no_mangle]
pub unsafe extern "C" fn pci_pcbios_init() -> void __init {
    void __init pci_pcbios_init(void)
    {
    if ((pci_probe & PCI_PROBE_BIOS)
    && ((raw_pci_ops = pci_find_bios()))) {
    pub 1: pci_bios_present =,
    }
    }
