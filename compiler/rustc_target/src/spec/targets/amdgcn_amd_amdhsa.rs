use crate::spec::{
    LinkSelfContainedDefault, LinkerFlavor, MergeFunctions, PanicStrategy, Target, TargetOptions,
};

pub fn target() -> Target {
    Target {
        arch: "amdgpu".into(),
        data_layout: "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-p7:160:256:256:32-p8:128:128-p9:192:256:256:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7:8:9".into(),
        llvm_target: "amdgcn-amd-amdhsa".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("--emit=asm generates PTX code that runs on AMD GPUs".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,

        options: TargetOptions {
            os: "amdhsa".into(),
            vendor: "amd".into(),
            linker_flavor: LinkerFlavor::Ptx,
            // The linker can be installed from `crates.io`.
            linker: Some("rust-ptx-linker".into()),

            // With `ptx-linker` approach, it can be later overridden via link flags.
            cpu: "native".into(),

            // FIXME: create tests for the atomics.
            max_atomic_width: Some(64),

            // Unwinding on CUDA is neither feasible nor useful.
            panic_strategy: PanicStrategy::Abort,

            // Needed to use `dylib` and `bin` crate types and the linker.
            dynamic_linking: true,

            // Avoid using dylib because it contain metadata not supported
            // by LLVM AMDGPU backend.
            only_cdylib: true,

            // Let the `ptx-linker` to handle LLVM lowering into MC / assembly.
            obj_is_bitcode: true,

            // Convenient and predicable naming scheme.
            dll_prefix: "".into(),
            dll_suffix: ".ptx".into(),
            exe_suffix: ".ptx".into(),

            // Disable MergeFunctions LLVM optimisation pass because it can
            // produce kernel functions that call other kernel functions.
            // This behavior is not supported by PTX ISA.
            merge_functions: MergeFunctions::Disabled,

            // The LLVM backend does not support stack canaries for this target
            supports_stack_protector: false,

            // Support using `self-contained` linkers like the llvm-bitcode-linker
            link_self_contained: LinkSelfContainedDefault::True,

            ..Default::default()
        },
    }
}
