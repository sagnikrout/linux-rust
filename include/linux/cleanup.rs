//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cleanup.h
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
// DOC: scope-based cleanup helpers
//
// The "goto error" pattern is notorious for introducing subtle resource
// leaks. It is tedious and error prone to add new resource acquisition
// constraints into code paths that already have several unwind
// conditions. The "cleanup" helpers enable the compiler to help with
// this tedium and can aid in maintaining LIFO (last in first out)
// unwind ordering to avoid unintentional leaks.
//
// As drivers make up the majority of the kernel code base, here is an
// example of using these helpers to clean up PCI drivers. The target of
// the cleanups are occasions where a goto is used to unwind a device
// reference (pci_dev_put()), or unlock the device (pci_dev_unlock())
// before returning.
//
// The DEFINE_FREE() macro can arrange for PCI device references to be
// dropped when the associated variable goes out of scope::
//
// DEFINE_FREE(pci_dev_put, struct pci_dev *, if (_T) pci_dev_put(_T))
// ...
// struct pci_dev *dev __free(pci_dev_put) =
// pci_get_slot(parent, PCI_DEVFN(0, 0));
//
// The above will automatically call pci_dev_put() if @dev is non-NULL
// when @dev goes out of scope (automatic variable scope). If a function
// wants to invoke pci_dev_put() on error, but return @dev (i.e. without
// freeing it) on success, it can do::
//
// return no_free_ptr(dev);
//
// ...or::
//
// return_ptr(dev);
//
// The DEFINE_GUARD() macro can arrange for the PCI device lock to be
// dropped when the scope where guard() is invoked ends::
//
// DEFINE_GUARD(pci_dev, struct pci_dev *, pci_dev_lock(_T), pci_dev_unlock(_T))
// ...
// guard(pci_dev)(dev);
//
// The lifetime of the lock obtained by the guard() helper follows the
// scope of automatic variable declaration. Take the following example::
//
// func(...)
// {
// if (...) {
// ...
// guard(pci_dev)(dev); // pci_dev_lock() invoked here
// ...
// } // <- implied pci_dev_unlock() triggered here
// }
//
// Observe the lock is held for the remainder of the "if ()" block not
// the remainder of "func()".
//
// The ACQUIRE() macro can be used in all places that guard() can be
// used and additionally support conditional locks::
//
// DEFINE_GUARD_COND(pci_dev, _try, pci_dev_trylock(_T))
// ...
// ACQUIRE(pci_dev_try, lock)(dev);
// rc = ACQUIRE_ERR(pci_dev_try, &lock);
// if (rc)
// return rc;
// // @lock is held
//
// Now, when a function uses both __free() and guard()/ACQUIRE(), or
// multiple instances of __free(), the LIFO order of variable definition
// order matters. GCC documentation says:
//
// "When multiple variables in the same scope have cleanup attributes,
// at exit from the scope their associated cleanup functions are run in
// reverse order of definition (last defined, first cleanup)."
//
// When the unwind order matters it requires that variables be defined
// mid-function scope rather than at the top of the file.  Take the
// following example and notice the bug highlighted by "!!"::
//
// LIST_HEAD(list);
// DEFINE_MUTEX(lock);
//
// struct object {
// struct list_head node;
// };
//
// static struct object *alloc_add(void)
// {
// struct object *obj;
//
// lockdep_assert_held(&lock);
// obj = kzalloc(sizeof(*obj), GFP_KERNEL);
// if (obj) {
// LIST_HEAD_INIT(&obj->node);
// list_add(obj->node, &list):
// }
// return obj;
// }
//
// static void remove_free(struct object *obj)
// {
// lockdep_assert_held(&lock);
// list_del(&obj->node);
// kfree(obj);
// }
//
// DEFINE_FREE(remove_free, struct object *, if (_T) remove_free(_T))
// static int init(void)
// {
// struct object *obj __free(remove_free) = NULL;
// int err;
//
// guard(mutex)(&lock);
// obj = alloc_add();
//
// if (!obj)
// return -ENOMEM;
//
// err = other_init(obj);
// if (err)
// return err; // remove_free() called without the lock!!
//
// no_free_ptr(obj);
// return 0;
// }
//
// That bug is fixed by changing init() to call guard() and define +
// initialize @obj in this order::
//
// guard(mutex)(&lock);
// struct object *obj __free(remove_free) = alloc_add();
//
// Given that the "__free(...) = NULL" pattern for variables defined at
// the top of the function poses this potential interdependency problem
// the recommendation is to always define and assign variables in one
// statement and not group variable definitions at the top of the
// function when __free() is used.
//
// Lastly, given that the benefit of cleanup helpers is removal of
// "goto", and that the "goto" statement can jump between scopes, the
// expectation is that usage of "goto" and cleanup helpers is never
// mixed in the same function. I.e. for a given routine, convert all
// resources that need a "goto" cleanup to scope-based cleanup, or
// convert none of them.
//
// DEFINE_FREE(name, type, free):
// simple helper macro that defines the required wrapper for a __free()
// based cleanup function. @free is an expression using '_T' to access the
// variable. @free should typically include a NULL test before calling a
// function, see the example below.
//
// __free(name):
// variable attribute to add a scoped based cleanup to the variable.
//
// no_free_ptr(var):
// like a non-atomic xchg(var, NULL), such that the cleanup function will
// be inhibited -- provided it sanely deals with a NULL value.
//
// NOTE: this has __must_check semantics so that it is harder to accidentally
// leak the resource.
//
// return_ptr(p):
// returns p while inhibiting the __free().
//
// Ex.
//
// DEFINE_FREE(kfree, void *, if (_T) kfree(_T))
//
// void *alloc_obj(...)
// {
// struct obj *p __free(kfree) = kmalloc(...);
// if (!p)
// return NULL;
//
// if (!init_obj(p))
// return NULL;
//
// return_ptr(p);
// }
//
// NOTE: the DEFINE_FREE()'s @free expression includes a NULL test even though
// kfree() is fine to be called with a NULL value. This is on purpose. This way
// the compiler sees the end of our alloc_obj() function as:
//
// tmp = p;
// p = NULL;
// if (p)
// kfree(p);
// return tmp;
//
// And through the magic of value-propagation and dead-code-elimination, it
// eliminates the actual cleanup call and compiles into:
//
// return p;
//
// Without the NULL test it turns into a mess and the compiler can't help us.
//

// __ptr = nullvalue;         \

//
// Only for situations where an allocation is handed in to another function
// and consumed by that function on success.
//
// struct foo *f __free(kfree) = kzalloc(sizeof(*f), GFP_KERNEL);
//
// setup(f);
// if (some_condition)
// return -EINVAL;
// ....
// ret = bar(f);
// if (!ret)
// retain_and_null_ptr(f);
// return ret;
//
// After retain_and_null_ptr(f) the variable f is NULL and cannot be
// dereferenced anymore.
//

//
// DEFINE_CLASS(name, type, exit, init, init_args...):
// helper to define the destructor and constructor for a type.
// @exit is an expression using '_T' -- similar to FREE above.
// @init is an expression in @init_args resulting in @type
//
// EXTEND_CLASS(name, ext, init, init_args...):
// extends class @name to @name@ext with the new constructor
//
// CLASS(name, var)(args...):
// declare the variable @var as an instance of the named class
//
// CLASS_INIT(name, var, init_expr):
// declare the variable @var as an instance of the named class with
// custom initialization expression.
//
// Ex.
//
// DEFINE_CLASS(fdget, struct fd, fdput(_T), fdget(fd), int fd)
//
// CLASS(fdget, f)(fd);
// if (fd_empty(f))
// return -EBADF;
//
// // use 'f' without concern
//

//
// DEFINE_GUARD(name, type, lock, unlock):
// trivial wrapper around DEFINE_CLASS() above specifically
// for locks.
//
// DEFINE_GUARD_COND(name, ext, condlock)
// wrapper around EXTEND_CLASS above to add conditional lock
// variants to a base class, eg. mutex_trylock() or
// mutex_lock_interruptible().
//
// guard(name):
// an anonymous instance of the (guard) class, not recommended for
// conditional locks.
//
// scoped_guard (name, args...) { }:
// similar to CLASS(name, scope)(args), except the variable (with the
// explicit name 'scope') is declard in a for-loop such that its scope is
// bound to the next (compound) statement.
//
// for conditional locks the loop body is skipped when the lock is not
// acquired.
//
// scoped_cond_guard (name, fail, args...) { }:
// similar to scoped_guard(), except it does fail when the lock
// acquire fails.
//
// Only for conditional locks.
//
// ACQUIRE(name, var):
// a named instance of the (guard) class, suitable for conditional
// locks when paired with ACQUIRE_ERR().
//
// ACQUIRE_ERR(name, &var):
// a helper that is effectively a PTR_ERR() conversion of the guard
// pointer. Returns 0 when the lock was acquired and a negative
// error code otherwise.
//

//
// Default binary condition; success on 'true'.
//

//
// Helper macro for scoped_guard().
//
// Note that the "!__is_cond_ptr(_name)" part of the condition ensures that
// compiler would be sure that for the unconditional locks the body of the
// loop (caller-provided code glued to the else clause) could not be skipped.
// It is needed because the other part - "__guard_ptr(_name)(&scope)" - is too
// hard to deduce (even if could be proven true for unconditional locks).
//

//
// Additional helper macros for generating lock guards with types, either for
// locks that don't have a native type (eg. RCU, preempt) or those that need a
// 'fat' pointer (eg. spin_lock_irqsave).
//
// DEFINE_LOCK_GUARD_0(name, lock, unlock, ...)
// DEFINE_LOCK_GUARD_1(name, type, lock, unlock, ...)
// DEFINE_LOCK_GUARD_1_COND(name, ext, condlock)
//
// will result in the following type:
//
// typedef struct {
// type *lock;		// 'type := void' for the _0 variant
// __VA_ARGS__;
// } class_##name##_t;
//
// As above, both _lock and _unlock are statements, except this time '_T' will
// be a pointer to the above struct.
//

// _T __maybe_unused = &_t;			\

//
// To support Context Analysis, we need to allow the compiler to see the
// acquisition and release of the context lock. However, the "cleanup" helpers
// wrap the lock in a struct passed through separate helper functions, which
// hides the lock alias from the compiler (no inter-procedural analysis).
//
// To make it work, we introduce an explicit alias to the context lock instance
// that is "cleaned" up with a separate cleanup helper. This helper is a dummy
// function that does nothing at runtime, but has the "_unlock" attribute to
// tell the compiler what happens at the end of the scope.
//
// To generalize the pattern, the WITH_LOCK_GUARD_1_ATTRS() macro should be used
// to redefine the constructor, which then also creates the alias variable with
// the right "cleanup" attribute, *after* DECLARE_LOCK_GUARD_1_ATTRS() has been
// used.
//
// Example usage:
//
// DECLARE_LOCK_GUARD_1_ATTRS(mutex, __acquires(_T), __releases(*(struct mutex **)_T))
// #define class_mutex_constructor(_T) WITH_LOCK_GUARD_1_ATTRS(mutex, _T)
//
// Note: To support the for-loop based scoped helpers, the auxiliary variable
// must be a pointer to the "class" type because it is defined in the same
// statement as the guard variable. However, we initialize it with the lock
// pointer (despite the type mismatch, the compiler's alias analysis still works
// as expected). The "_unlock" attribute receives a pointer to the auxiliary
// variable (a double pointer to the class type), and must be cast and
// dereferenced appropriately.
//

// __UNIQUE_ID(unlock) __cleanup(__class_##_name##_cleanup_ctx) = (void *)(unsigned long)(_T)

