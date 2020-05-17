#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use jni::*;
use jni::errors::jni_error_code_to_result;
use jni::objects::*;
use std::io::{Error, ErrorKind};
use std::io::Result;
extern crate libloading as lib;

fn get_system_classloader<'a>(env: &'a JNIEnv) -> Result<JObject<'a>> {
    let loader_class = env
        .find_class("java/lang/ClassLoader")
        .expect("Unable to find Java ClassLoader class");

    let loader_value = env
        .call_static_method(
            loader_class,
            "getSystemClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )
        .expect("Unable to get system class loader");

    return match loader_value {
        JValue::Object(jobject) => Ok(jobject),
        _ => Err(Error::new(
            ErrorKind::NotFound,
            "Unable to get system class loader",
        )),
    };
}


fn load_main_class<'a>(env: &'a JNIEnv, system_loader: &'a JObject) -> Result<JObject<'a>> {
    let main_class_str = "com.maddox.il2.game.GameWin3D";

    let main_class_name = env
        .new_string(main_class_str)
        .expect("Unable to create Java String");

    let class_value = env
        .call_method(
            *system_loader, "loadClass", "(Ljava/lang/String;Z)Ljava/lang/Class;",
            &[JValue::Object(*main_class_name), JValue::Bool(1)]
        )
        .expect("Unable to load main class");

    return match class_value {
        JValue::Object(jobject) => Ok(jobject),
        _ => Err(Error::new(
            ErrorKind::NotFound,
            "Unable to load main class"
        ))
    };
}

fn call_main_method<'a>(env: &'a JNIEnv) -> Result<()> {
    let string_class = env
        .find_class("java/lang/String")
        .expect("Unable to find Java String class");

    let main_args = env
        .new_object_array(0, string_class, JObject::null())
        .expect("Error creating main args array");

    let _ = env.call_static_method(
        "com/maddox/il2/game/GameWin3D",
        "main", "([Ljava/lang/String;)V",
        &[JValue::Object(main_args.into())]
    );

    return Ok(())
}

fn main() -> std::io::Result<()> {
    let java_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Djava.class.path=.")
        .option("-agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=5005")
        .build()
        .expect("Failed to create Java VM args");

    println!("Created Java VM args");

    // Ugly workaround for inner field of InitArgs being private;
    // we need it to call the JNI_CreateJavaVM function dynamically
    struct VMInitArgs {
        pub inner: sys::JavaVMInitArgs,
        pub opts: Vec<sys::JavaVMOption>
    }

    let mut raw_java_args: VMInitArgs = unsafe { std::mem::transmute(java_args) };

    let lib = lib::Library::new("./bin/server/jvm.dll").expect("Unable to find jvm.dll");
    let mut raw_java_vm: *mut sys::JavaVM = std::ptr::null_mut();
    let mut raw_env: *mut sys::JNIEnv = std::ptr::null_mut();

    println!("Found jvm.dll");

    let JNI_CreateJavaVM: lib::Symbol<unsafe extern fn(
        pvm: *mut *mut sys::JavaVM,
        penv: *mut *mut sys::JNIEnv,
        args: *mut sys::JavaVMInitArgs,
    ) -> sys::jint> = unsafe {
        lib
            .get(b"JNI_CreateJavaVM\0")
            .expect("Unable to find JNI_CreateJavaVM function")
    };

    println!("Found JNI_CreateJavaVM function");

    unsafe {
        jni_error_code_to_result(JNI_CreateJavaVM(
            &mut raw_java_vm,
            &mut raw_env,
            &mut raw_java_args.inner
        )).expect("Error creating Java VM")
    };

    let java_vm = unsafe { JavaVM::from_raw(raw_java_vm).unwrap() };

    let attach_guard = java_vm
        .attach_current_thread()
        .expect("Error attaching current thread to Java VM");

    println!("Attached current thread to Java VM");

    let env = &*attach_guard;

    let system_loader = get_system_classloader(&env)?;

    println!("Fetched system classloader");

    load_main_class(env, &system_loader)?;

    println!("Loaded main class");

    call_main_method(env)?;

    if env.exception_check().unwrap() {
        env.exception_describe().unwrap()
    };

    return Ok(())
}