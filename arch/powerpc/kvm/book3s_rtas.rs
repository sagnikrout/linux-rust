//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_rtas.c
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
// Copyright 2012 Michael Ellerman, IBM Corporation.
//

#[no_mangle]
unsafe extern "C" fn kvm_rtas_set_xive(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    static void kvm_rtas_set_xive(struct kvm_vcpu *vcpu, struct rtas_args *args)
    {
    u32 irq, server, priority;
    int rc;
    if (be32_to_cpu(args.nargs) != 3 || be32_to_cpu(args.nret) != 1) {
    rc = -3;
    goto out;
    }
    irq = be32_to_cpu(args.args[0]);
    server = be32_to_cpu(args.args[1]);
    priority = be32_to_cpu(args.args[2]);
    if (xics_on_xive())
    rc = kvmppc_xive_set_xive(vcpu.kvm, irq, server, priority);
    else
    rc = kvmppc_xics_set_xive(vcpu.kvm, irq, server, priority);
    if (rc)
    rc = -3;
    out:
    args.rets[0] = cpu_to_be32(rc);
    }
#[no_mangle]
unsafe extern "C" fn kvm_rtas_get_xive(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    static void kvm_rtas_get_xive(struct kvm_vcpu *vcpu, struct rtas_args *args)
    {
    u32 irq, server, priority;
    int rc;
    if (be32_to_cpu(args.nargs) != 1 || be32_to_cpu(args.nret) != 3) {
    rc = -3;
    goto out;
    }
    irq = be32_to_cpu(args.args[0]);
    server = priority = 0;
    if (xics_on_xive())
    rc = kvmppc_xive_get_xive(vcpu.kvm, irq, &server, &priority);
    else
    rc = kvmppc_xics_get_xive(vcpu.kvm, irq, &server, &priority);
    if (rc) {
    rc = -3;
    goto out;
    }
    args.rets[1] = cpu_to_be32(server);
    args.rets[2] = cpu_to_be32(priority);
    out:
    args.rets[0] = cpu_to_be32(rc);
    }
#[no_mangle]
unsafe extern "C" fn kvm_rtas_int_off(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    static void kvm_rtas_int_off(struct kvm_vcpu *vcpu, struct rtas_args *args)
    {
    u32 irq;
    int rc;
    if (be32_to_cpu(args.nargs) != 1 || be32_to_cpu(args.nret) != 1) {
    rc = -3;
    goto out;
    }
    irq = be32_to_cpu(args.args[0]);
    if (xics_on_xive())
    rc = kvmppc_xive_int_off(vcpu.kvm, irq);
    else
    rc = kvmppc_xics_int_off(vcpu.kvm, irq);
    if (rc)
    rc = -3;
    out:
    args.rets[0] = cpu_to_be32(rc);
    }
#[no_mangle]
unsafe extern "C" fn kvm_rtas_int_on(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    static void kvm_rtas_int_on(struct kvm_vcpu *vcpu, struct rtas_args *args)
    {
    u32 irq;
    int rc;
    if (be32_to_cpu(args.nargs) != 1 || be32_to_cpu(args.nret) != 1) {
    rc = -3;
    goto out;
    }
    irq = be32_to_cpu(args.args[0]);
    if (xics_on_xive())
    rc = kvmppc_xive_int_on(vcpu.kvm, irq);
    else
    rc = kvmppc_xics_int_on(vcpu.kvm, irq);
    if (rc)
    rc = -3;
    out:
    args.rets[0] = cpu_to_be32(rc);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_handler {
    pub args): *mut *mut *mut void (handler)(struct kvm_vcpu vcpu, struct rtas_args,
    pub name: *mut c_char,
}

    static struct rtas_handler rtas_handlers[] = {

    { .name = "ibm,set-xive", .handler = kvm_rtas_set_xive },
    { .name = "ibm,get-xive", .handler = kvm_rtas_get_xive },
    { .name = "ibm,int-off",  .handler = kvm_rtas_int_off },
    { .name = "ibm,int-on",   .handler = kvm_rtas_int_on },

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_token_definition {
    pub list: list_head,
    pub handler: *mut rtas_handler,
    pub token: u64,
}

#[no_mangle]
unsafe extern "C" fn rtas_name_matches(s1: *mut c_char, s2: *mut c_char) -> c_int {
    static int rtas_name_matches(char *s1, char *s2)
    {
    struct kvm_rtas_token_args args;
    return !strncmp(s1, s2, sizeof(args.name));
    }
#[no_mangle]
unsafe extern "C" fn rtas_token_undefine(kvm: *mut kvm, name: *mut c_char) -> c_int {
    static int rtas_token_undefine(struct kvm *kvm, char *name)
    {
    struct rtas_token_definition *d, *tmp;
    lockdep_assert_held(&kvm.arch.rtas_token_lock);
    list_for_each_entry_safe(d, tmp, &kvm.arch.rtas_tokens, list) {
    if (rtas_name_matches(d.handler.name, name)) {
    list_del(&d.list);
    kfree(d);
    return 0;
    }
    }
// It's not an error to undefine an undefined token
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtas_token_define(kvm: *mut kvm, name: *mut c_char, token: u64) -> c_int {
    static int rtas_token_define(struct kvm *kvm, char *name, u64 token)
    {
    struct rtas_token_definition *d;
    struct rtas_handler *h = core::ptr::null_mut();
    bool found;
    int i;
    lockdep_assert_held(&kvm.arch.rtas_token_lock);
    list_for_each_entry(d, &kvm.arch.rtas_tokens, list) {
    if (d.token == token)
    return -EEXIST;
    }
    found = false;
    for (i = 0; i < ARRAY_SIZE(rtas_handlers); i++) {
    h = &rtas_handlers[i];
    if (rtas_name_matches(h.name, name)) {
    found = true;
    break;
    }
    }
    if (!found)
    return -ENOENT;
    d = kzalloc_obj(*d);
    if (!d)
    return -ENOMEM;
    d.handler = h;
    d.token = token;
    list_add_tail(&d.list, &kvm.arch.rtas_tokens);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_vm_ioctl_rtas_define_token(kvm: *mut kvm, argp: *mut void __user) -> c_int {
    int kvm_vm_ioctl_rtas_define_token(struct kvm *kvm, void __user *argp)
    {
    struct kvm_rtas_token_args args;
    int rc;
    if (copy_from_user(&args, argp, sizeof(args)))
    return -EFAULT;
    mutex_lock(&kvm.arch.rtas_token_lock);
    if (args.token)
    rc = rtas_token_define(kvm, args.name, args.token);
    else
    rc = rtas_token_undefine(kvm, args.name);
    mutex_unlock(&kvm.arch.rtas_token_lock);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_rtas_hcall(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmppc_rtas_hcall(struct kvm_vcpu *vcpu)
    {
    struct rtas_token_definition *d;
    struct rtas_args args;
    rtas_arg_t *orig_rets;
    gpa_t args_phys;
    int rc;
//
// r4 contains the guest physical address of the RTAS args
// Mask off the top 4 bits since this is a guest real address
//
    args_phys = kvmppc_get_gpr(vcpu, 4) & KVM_PAM;
    kvm_vcpu_srcu_read_lock(vcpu);
    rc = kvm_read_guest(vcpu.kvm, args_phys, &args, sizeof(args));
    kvm_vcpu_srcu_read_unlock(vcpu);
    if (rc)
    goto fail;
//
// args->rets is a pointer into args->args. Now that we've
// copied args we need to fix it up to point into our copy,
// not the guest args. We also need to save the original
// value so we can restore it on the way out.
//
    orig_rets = args.rets;
    if (be32_to_cpu(args.nargs) >= ARRAY_SIZE(args.args)) {
//
// Don't overflow our args array: ensure there is room for
// at least rets[0] (even if the call specifies 0 nret).
//
// Each handler must then check for the correct nargs and nret
// values, but they may always return failure in rets[0].
//
    rc = -EINVAL;
    goto fail;
    }
    args.rets = &args.args[be32_to_cpu(args.nargs)];
    mutex_lock(&vcpu.kvm.arch.rtas_token_lock);
    rc = -ENOENT;
    list_for_each_entry(d, &vcpu.kvm.arch.rtas_tokens, list) {
    if (d.token == be32_to_cpu(args.token)) {
    d.handler.handler(vcpu, &args);
    rc = 0;
    break;
    }
    }
    mutex_unlock(&vcpu.kvm.arch.rtas_token_lock);
    if (rc == 0) {
    args.rets = orig_rets;
    rc = kvm_write_guest(vcpu.kvm, args_phys, &args, sizeof(args));
    if (rc)
    goto fail;
    }
    return rc;
    fail:
//
// We only get here if the guest has called RTAS with a bogus
// args pointer or nargs/nret values that would overflow the
// array. That means we can't get to the args, and so we can't
// fail the RTAS call. So fail right out to userspace, which
// should kill the guest.
//
// SLOF should actually pass the hcall return value from the
// rtas handler call in r3, so enter_rtas could be modified to
// return a failure indication in r3 and we could return such
// errors to the guest rather than failing to host userspace.
// However old guests that don't test for failure could then
// continue silently after errors, so for now we won't do this.
//
    return rc;
    }
    EXPORT_SYMBOL_GPL(kvmppc_rtas_hcall);
#[no_mangle]
pub unsafe extern "C" fn kvmppc_rtas_tokens_free(kvm: *mut kvm) {
    void kvmppc_rtas_tokens_free(struct kvm *kvm)
    {
    struct rtas_token_definition *d, *tmp;
    list_for_each_entry_safe(d, tmp, &kvm.arch.rtas_tokens, list) {
    list_del(&d.list);
    kfree(d);
    }
    }
