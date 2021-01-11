#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use anyhow::{bail, Context, Result};
use jni::errors::jni_error_code_to_result;
use jni::objects::*;
use jni::*;
extern crate clap;
extern crate libloading as lib;
use clap::{App, Arg};

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn get_system_classloader(env: JNIEnv<'_>) -> Result<JObject<'_>> {
    let loader_class = env.find_class("java/lang/ClassLoader").with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to find Java ClassLoader class"
    })?;

    let loader_value = env
        .call_static_method(
            loader_class,
            "getSystemClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Unable to get system class loader"
        })?;

    match loader_value {
        JValue::Object(jobject) => Ok(jobject),
        _ => bail!("Unable to get system class loader"),
    }
}

fn load_class<'a>(
    env: JNIEnv<'a>,
    system_loader: JObject<'a>,
    class_name_str: &str,
) -> Result<JObject<'a>> {
    let class_name = env.new_string(class_name_str).with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to create Java String"
    })?;

    let class_value = env
        .call_method(
            system_loader,
            "loadClass",
            "(Ljava/lang/String;Z)Ljava/lang/Class;",
            &[JValue::Object(*class_name), JValue::Bool(1)],
        )
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Unable to load main class"
        })?;

    match class_value {
        JValue::Object(jobject) => Ok(jobject),
        _ => bail!("Unable to load main class"),
    }
}

fn load_physfs_class<'a>(env: JNIEnv<'a>, system_loader: JObject<'a>) -> Result<JObject<'a>> {
    let physfs_class_str = "com.maddox.rts.PhysFS";
    load_class(env, system_loader, physfs_class_str)
}

fn load_main_class<'a>(env: JNIEnv<'a>, system_loader: JObject<'a>) -> Result<JObject<'a>> {
    let main_class_str = "com.maddox.il2.game.GameWin3D";
    load_class(env, system_loader, main_class_str)
}

fn mount_files_zip(env: JNIEnv<'_>) -> Result<()> {
    let files_zip = env.new_string("files.zip").with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to create Java String"
    })?;

    env.call_static_method(
        "com/maddox/rts/PhysFS",
        "mountArchive",
        "(Ljava/lang/String;)V",
        &[JValue::Object(*files_zip)],
    )
    .with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to call PhysFS mountArchive method"
    })?;

    Ok(())
}

fn call_main_method(env: JNIEnv<'_>) -> Result<()> {
    let string_class = env.find_class("java/lang/String").with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to find Java String class"
    })?;

    let main_args = env
        .new_object_array(0, string_class, JObject::null())
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Error creating main args array"
        })?;

    env.call_static_method(
        "com/maddox/il2/game/GameWin3D",
        "main",
        "([Ljava/lang/String;)V",
        &[JValue::Object(main_args.into())],
    )
    .with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to call main method"
    })?;

    Ok(())
}

fn main() -> Result<()> {
    let cli_args = App::new("OpenIL2")
        .version(build_info::PKG_VERSION)
        .author(build_info::PKG_AUTHORS)
        .about("A modernised launcher for IL-2 Sturmovik 1946")
        .arg(
            Arg::with_name("transform-classes")
                .long("transform-classes")
                .short("t")
                .help("Transforms and dumps the game classes as they are loaded"),
        )
        .arg(
            Arg::with_name("jmx-monitoring")
                .long("jmx-monitoring")
                .short("j")
                .help("Enable JMX monitoring"),
        )
        .arg(
            Arg::with_name("jmx-port")
                .long("jmx-port")
                .takes_value(true)
                .default_value_if("jmx-monitoring", None, "9010")
                .value_name("port")
                .help("The port to use for JMX monitoring"),
        )
        .arg(
            Arg::with_name("await-debug")
                .long("await-debug")
                .short("d")
                .help("Suspend and await debug on the given port after startup"),
        )
        .arg(
            Arg::with_name("debug-port")
                .long("debug-port")
                .takes_value(true)
                .default_value_if("await-debug", None, "5005")
                .value_name("port")
                .help("The port to use for attaching a debugger"),
        )
        .get_matches();

    let mut java_arg_bldr = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Djava.class.path=.;physfs_java.jar")
        .option("-Djava.locale.providers=COMPAT")
        .option("-XX:+UseShenandoahGC")
        .option("-XX:+AlwaysPreTouch")
        .option("-XX:+DisableExplicitGC")
        .option("-XX:-UseBiasedLocking")
        .option("-Xms1000m")
        .option("-Xmx1000m");

    if cli_args.is_present("transform-classes") {
        java_arg_bldr = java_arg_bldr.option("-javaagent:classload_agent.jar");
    }

    if cli_args.is_present("await-debug") {
        let debug_port = cli_args.value_of("debug-port").unwrap();
        java_arg_bldr = java_arg_bldr.option(&format!(
            "-agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=127.0.0.1:{}",
            debug_port
        ));
    }

    if cli_args.is_present("jmx-monitoring") {
        let jmx_port = cli_args.value_of("jmx-port").unwrap();
        java_arg_bldr = java_arg_bldr
            .option("-Dcom.sun.management.jmxremote.host=127.0.0.1")
            .option(&format!("-Dcom.sun.management.jmxremote.port={}", jmx_port))
            .option(&format!(
                "-Dcom.sun.management.jmxremote.rmi.port={}",
                jmx_port
            ))
            .option("-Dcom.sun.management.jmxremote.authenticate=false")
            .option("-Dcom.sun.management.jmxremote.ssl=false");
    }

    let java_args = java_arg_bldr
        .build()
        .context("Failed to create Java VM args")?;

    // Ugly workaround for inner field of InitArgs being private;
    // we need it to call the JNI_CreateJavaVM function dynamically
    struct VMInitArgs {
        pub inner: sys::JavaVMInitArgs,
        pub opts: Vec<sys::JavaVMOption>,
    }

    let mut raw_java_args: VMInitArgs = unsafe { std::mem::transmute(java_args) };

    let lib = lib::Library::new("./bin/server/jvm.dll").context("Unable to find jvm.dll")?;
    let mut raw_java_vm: *mut sys::JavaVM = std::ptr::null_mut();
    let mut raw_env: *mut sys::JNIEnv = std::ptr::null_mut();

    let JNI_CreateJavaVM: lib::Symbol<
        unsafe extern "C" fn(
            pvm: *mut *mut sys::JavaVM,
            penv: *mut *mut sys::JNIEnv,
            args: *mut sys::JavaVMInitArgs,
        ) -> sys::jint,
    > = unsafe {
        lib.get(b"JNI_CreateJavaVM\0")
            .context("Unable to find JNI_CreateJavaVM function")?
    };

    unsafe {
        jni_error_code_to_result(JNI_CreateJavaVM(
            &mut raw_java_vm,
            &mut raw_env,
            &mut raw_java_args.inner,
        ))
        .context("Error creating Java VM")?
    };

    let java_vm = unsafe { JavaVM::from_raw(raw_java_vm)? };

    let attach_guard = java_vm
        .attach_current_thread()
        .context("Error attaching current thread to Java VM")?;

    let env = *attach_guard;

    let system_loader = get_system_classloader(env)?;

    load_class(env, system_loader, "com.maddox.rts.PhysFS")?;

    mount_files_zip(env)?;

    load_class(env, system_loader, "com.maddox.rts.PhysFSLoader")?;

    let physfs_loader = env.new_object(
        "com/maddox/rts/PhysFSLoader",
        "(Ljava/lang/ClassLoader;)V", 
        &[JValue::Object(system_loader)],
    ) 
    .with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to create PhysFSLoader"
    })?;

    load_class(env, physfs_loader, "com.maddox.il2.game.GameWin3D")?;

    call_main_method(env)?;

    if env.exception_check().unwrap() {
        env.exception_describe().unwrap()
    };

    Ok(())
}
