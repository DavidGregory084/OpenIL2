#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, bail, Context, Result};
use jni::errors::jni_error_code_to_result;
use jni::objects::*;
use jni::*;
extern crate clap;
extern crate libloading as lib;
use clap::{App, Arg};
use std::io::Read;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::processenv::GetCurrentDirectoryA;
use winapi::um::winnt::CHAR;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn get_system_classloader(env: JNIEnv<'_>) -> Result<JObject<'_>> {
    let loader_value = env
        .call_static_method(
            "java/lang/ClassLoader",
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
    loader: JObject<'a>,
    class_name_str: &str,
) -> Result<JClass<'a>> {
    let class_name = env.new_string(class_name_str).with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to create Java String"
    })?;

    let class_value = env
        .call_method(
            loader,
            "loadClass",
            "(Ljava/lang/String;Z)Ljava/lang/Class;",
            &[JValue::Object(*class_name), JValue::Bool(1)],
        )
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Unable to load class"
        })?;

    match class_value {
        JValue::Object(jobject) => Ok(jobject.into()),
        _ => bail!("Unable to load main class"),
    }
}

fn load_class_from_physfs_jar<'a>(
    env: JNIEnv<'a>,
    loader: JObject<'a>,
    class_name: &str,
) -> Result<JClass<'a>> {
    let binary_name = class_name.to_string().replace(".", "/");
    let physfs_jar = std::fs::File::open("physfs_java.jar")?;
    let mut zip_archive = zip::ZipArchive::new(physfs_jar)?;
    let zip_entry_name = format!("{}.class", binary_name);
    let mut zip_entry = zip_archive
        .by_name(&zip_entry_name)
        .with_context(|| anyhow!("Couldn't find entry {} in the PhysFS JAR", zip_entry_name))?;
    let mut class_data: Vec<u8> = Vec::new();
    zip_entry.read_to_end(&mut class_data)?;
    env.define_class(binary_name, loader, &class_data)
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            }
            "Unable to load class from the PhysFS JAR"
        })
}

fn load_class_from_zip<'a>(
    env: JNIEnv<'a>,
    loader: JObject<'a>,
    class_name: &str,
) -> Result<JClass<'a>> {
    let binary_name = class_name.to_string().replace(".", "/");
    let files_zip = std::fs::File::open("files.zip")?;
    let mut zip_archive = zip::ZipArchive::new(files_zip)?;
    let zip_entry_name = format!("{}.class", binary_name.to_ascii_uppercase());
    let mut zip_entry = zip_archive.by_name(&zip_entry_name).with_context(|| {
        anyhow!(
            "Couldn't find entry {} in the zip file files.zip",
            zip_entry_name
        )
    })?;
    let mut class_data: Vec<u8> = Vec::new();
    zip_entry.read_to_end(&mut class_data)?;
    env.define_class(binary_name, loader, &class_data)
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            }
            "Unable to load class from files.zip"
        })
}

fn mount_files_zip(env: JNIEnv<'_>, physfs_class: JClass<'_>) -> Result<()> {
    let files_zip = env.new_string("files.zip").with_context(|| {
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap()
        };
        "Unable to create Java String"
    })?;

    env.call_static_method(
        physfs_class,
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

fn create_object_instance<'a>(env: JNIEnv<'a>, class: JClass<'a>) -> Result<JObject<'a>> {
    let object_value = env
        .call_method(class, "newInstance", "()Ljava/lang/Object;", &[])
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            anyhow!("Unable to create new instance of {:?}", class)
        })?;

    match object_value {
        JValue::Object(jobject) => Ok(jobject.into()),
        _ => bail!("Unable to create new instance of {:?}", class),
    }
}

fn call_loadnative_method(env: JNIEnv<'_>, inputstream_class: JClass<'_>) -> Result<()> {
    env.call_static_method(inputstream_class, "_loadNative", "()V", &[])
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Unable to call _loadNative method"
        })?;

    Ok(())
}

fn call_preload_method(env: JNIEnv<'_>, physfs_loader: JObject<'_>) -> Result<()> {
    env.call_method(physfs_loader, "preload", "()V", &[])
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Unable to preload game classes"
        })?;

    Ok(())
}

fn call_main_method(env: JNIEnv<'_>, main_class: JClass<'_>) -> Result<()> {
    let main_args = env
        .new_object_array(0, "java/lang/String", JObject::null())
        .with_context(|| {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap()
            };
            "Error creating main args array"
        })?;

    env.call_static_method(
        main_class,
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

struct PhysFS {}

impl Drop for PhysFS {
    fn drop(&mut self) {
        unsafe {
            if PHYSFS_isInit() > 0 {
                if PHYSFS_deinit() == 0 {
                    panic!("Unable to deinitialise PhysFS");
                }
            }
        }
    }
}

fn init_physfs() -> Result<PhysFS> {
    unsafe {
        if PHYSFS_init(std::ptr::null()) == 0 {
            bail!("Unable to initialise PhysFS");
        } else {
            let mut vec = vec![0 as CHAR; MAX_PATH as usize];
            let c_str = &mut vec[..];

            if GetCurrentDirectoryA(MAX_PATH as u32, c_str.as_mut_ptr() as *mut CHAR) == 0 {
                bail!("Unable to get current directory");
            } else {
                if PHYSFS_setWriteDir(c_str.as_mut_ptr() as *mut CHAR) == 0 {
                    bail!("Unable to set current directory as PhysFS write directory");
                } else {
                    if PHYSFS_addToSearchPath(c_str.as_mut_ptr() as *mut CHAR, 0) == 0 {
                        bail!("Unable to add current directory to PhysFS search path");
                    }
                }
            }
        }
    }

    Ok(PhysFS {})
}

fn main() -> Result<()> {
    let _physfs = init_physfs()?;

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
                .help("The port to use for JMX monitoring, default 9010"),
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
                .help("The port to use for attaching a debugger, default 5005"),
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
        .option("-Xms512m")
        .option("-Xmx512m");

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

    let physfs_loader_class = load_class(env, system_loader, "com.maddox.rts.PhysFSLoader")?;

    let physfs_inputstream_class =
        load_class_from_physfs_jar(env, system_loader, "com.maddox.rts.PhysFSInputStream")?;

    // Load physfs_jni.dll in the system loader
    call_loadnative_method(env, physfs_inputstream_class)?;

    // Create PhysFS loader
    let physfs_loader = create_object_instance(env, physfs_loader_class)?;

    let physfs_reader_class =
        load_class_from_physfs_jar(env, physfs_loader, "com.maddox.rts.PhysFSReader")?;

    // Load physfs_rts.dll in the PhysFS loader
    call_loadnative_method(env, physfs_reader_class)?;

    let physfs_class = load_class_from_physfs_jar(env, system_loader, "com.maddox.rts.PhysFS")?;

    // Mount files.zip, ensuring all game classes are on PhysFS search path
    mount_files_zip(env, physfs_class)?;

    // Load DT.dll and rts.dll in the PhysFS loader
    let sfs_inputstream_class =
        load_class_from_zip(env, physfs_loader, "com.maddox.rts.SFSInputStream")?;

    call_loadnative_method(env, sfs_inputstream_class)?;

    // Preload game classes by loading com.maddox.il2.game.Main
    call_preload_method(env, physfs_loader)?;

    // Load the main class of the game
    let main_class = load_class(env, physfs_loader, "com.maddox.il2.game.GameWin3D")?;

    call_main_method(env, main_class)?;

    Ok(())
}
