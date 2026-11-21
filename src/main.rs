//! Catalog for the 100 metal-rust examples.
//!
//! The root package carries no Metal dependency; it prints the example index
//! so you can browse what is available and pick a `cargo run -p` command.
//! Every real example lives in its own workspace member under `examples/`.

#![forbid(unsafe_code)]

/// One catalog row: number, member folder, title, chapter.
type Row = (&'static str, &'static str, &'static str, &'static str);

/// The 100 examples, in curriculum order.
const CATALOG: [Row; 100] = [
    (
        "001",
        "ex001_hello_metal",
        "Hello Metal: create the default GPU device",
        "Devices & Setup",
    ),
    (
        "002",
        "ex002_device_identity",
        "Device identity: name, architecture and location",
        "Devices & Setup",
    ),
    (
        "003",
        "ex003_device_capabilities",
        "Boolean device capability matrix",
        "Devices & Setup",
    ),
    (
        "004",
        "ex004_device_numeric_properties",
        "Numeric device properties without narrowing",
        "Devices & Setup",
    ),
    (
        "005",
        "ex005_gpu_families",
        "GPU family support checks",
        "Devices & Setup",
    ),
    (
        "006",
        "ex006_max_threads_and_samples",
        "Threadgroup limits and sample counts",
        "Devices & Setup",
    ),
    (
        "007",
        "ex007_device_tiers",
        "Argument-buffer and read/write texture tiers",
        "Devices & Setup",
    ),
    (
        "008",
        "ex008_sample_timestamps",
        "Synchronized CPU and GPU timestamps",
        "Devices & Setup",
    ),
    (
        "009",
        "ex009_heap_allocations",
        "Heap size and alignment queries",
        "Devices & Setup",
    ),
    (
        "010",
        "ex010_device_ownership",
        "Device ownership: Clone and thread binding",
        "Devices & Setup",
    ),
    (
        "011",
        "ex011_first_command_queue",
        "Create and label a command queue",
        "Command Queues & Buffers",
    ),
    (
        "012",
        "ex012_bounded_command_queue",
        "Limit in-flight command buffers",
        "Command Queues & Buffers",
    ),
    (
        "013",
        "ex013_first_buffer",
        "Allocate a shared buffer",
        "Command Queues & Buffers",
    ),
    (
        "014",
        "ex014_buffer_writes",
        "Checked CPU writes into buffers",
        "Command Queues & Buffers",
    ),
    (
        "015",
        "ex015_buffer_with_bytes",
        "Initialize buffers from Rust data",
        "Command Queues & Buffers",
    ),
    (
        "016",
        "ex016_storage_modes",
        "SHARED, MANAGED, PRIVATE and MEMORYLESS storage",
        "Command Queues & Buffers",
    ),
    (
        "017",
        "ex017_gpu_addresses",
        "Buffer GPU addresses and identity",
        "Command Queues & Buffers",
    ),
    (
        "018",
        "ex018_blit_copy_buffer",
        "GPU-side buffer copies with readback",
        "Command Queues & Buffers",
    ),
    (
        "019",
        "ex019_blit_fill_buffer",
        "Fill buffer ranges on the GPU",
        "Command Queues & Buffers",
    ),
    (
        "020",
        "ex020_completion_handlers",
        "on_complete callbacks with channels",
        "Command Queues & Buffers",
    ),
    (
        "021",
        "ex021_compile_inline_msl",
        "Compile MSL source at runtime",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "022",
        "ex022_compile_options",
        "CompileOptions: strict, fast-math, logging",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "023",
        "ex023_library_functions",
        "Enumerate functions in a library",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "024",
        "ex024_function_metadata",
        "Function names, types and constants",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "025",
        "ex025_async_compilation",
        "Asynchronous library compilation",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "026",
        "ex026_compute_pipeline",
        "Compute pipelines and their limits",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "027",
        "ex027_async_pipeline",
        "Asynchronous compute pipeline compilation",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "028",
        "ex028_pipeline_reflection",
        "Reflection from compute pipelines",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "029",
        "ex029_pipeline_descriptor_labels",
        "ComputePipelineDescriptor labels",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "030",
        "ex030_render_pipeline_descriptor",
        "Vertex/fragment pipeline descriptors",
        "Shaders, Libraries & Pipelines",
    ),
    (
        "031",
        "ex031_vector_add",
        "Classic GPU vector addition",
        "Compute Dispatch",
    ),
    (
        "032",
        "ex032_dispatch_sizing",
        "dispatch_1d with threads per group",
        "Compute Dispatch",
    ),
    (
        "033",
        "ex033_threadgroup_grid",
        "dispatch_threadgroups with grid positions",
        "Compute Dispatch",
    ),
    (
        "034",
        "ex034_dispatch_threads_2d",
        "2D dispatch writing a gradient",
        "Compute Dispatch",
    ),
    (
        "035",
        "ex035_gpu_math_functions",
        "sin, sqrt and pow on the GPU",
        "Compute Dispatch",
    ),
    (
        "036",
        "ex036_struct_io",
        "Repr(C) structs between Rust and MSL",
        "Compute Dispatch",
    ),
    (
        "037",
        "ex037_atomic_counters",
        "Device atomics in kernels",
        "Compute Dispatch",
    ),
    (
        "038",
        "ex038_threadgroup_reduction",
        "Shared memory reduction with barriers",
        "Compute Dispatch",
    ),
    (
        "039",
        "ex039_inline_arguments",
        "set_bytes for small kernel arguments",
        "Compute Dispatch",
    ),
    (
        "040",
        "ex040_multi_kernel_pass",
        "Two kernels with a memory barrier",
        "Compute Dispatch",
    ),
    (
        "041",
        "ex041_offscreen_triangle",
        "Render into a texture and read pixels back",
        "Offscreen Rendering",
    ),
    (
        "042",
        "ex042_clear_pass",
        "Color attachment clears without draws",
        "Offscreen Rendering",
    ),
    (
        "043",
        "ex043_vertex_colors",
        "Per-vertex color interpolation",
        "Offscreen Rendering",
    ),
    (
        "044",
        "ex044_uniform_constants",
        "Vertex uniforms via set_vertex_bytes",
        "Offscreen Rendering",
    ),
    (
        "045",
        "ex045_viewport_and_scissor",
        "Viewport and scissor rectangles",
        "Offscreen Rendering",
    ),
    (
        "046",
        "ex046_gradient_triangle",
        "Interpolated gradients from position",
        "Offscreen Rendering",
    ),
    (
        "047",
        "ex047_instanced_draws",
        "draw_primitives_instanced with base_instance",
        "Offscreen Rendering",
    ),
    (
        "048",
        "ex048_indexed_draw",
        "draw_indexed_primitives with UInt16 indices",
        "Offscreen Rendering",
    ),
    (
        "049",
        "ex049_culling_and_fill",
        "Winding, cull mode and fill mode",
        "Offscreen Rendering",
    ),
    (
        "050",
        "ex050_depth_stencil_state",
        "Depth-stencil states without a window",
        "Offscreen Rendering",
    ),
    (
        "051",
        "ex051_texture_descriptors",
        "Descriptor factories and validation",
        "Textures & Samplers",
    ),
    (
        "052",
        "ex052_texture_properties",
        "Texture dimensions and pixel formats",
        "Textures & Samplers",
    ),
    (
        "053",
        "ex053_texture_usage_flags",
        "TextureUsage bit flags",
        "Textures & Samplers",
    ),
    (
        "054",
        "ex054_texture_upload",
        "replace_region_2d pixel uploads",
        "Textures & Samplers",
    ),
    (
        "055",
        "ex055_texture_alignments",
        "Linear texture alignment queries",
        "Textures & Samplers",
    ),
    (
        "056",
        "ex056_sampler_descriptors",
        "Address modes and LOD clamps",
        "Textures & Samplers",
    ),
    (
        "057",
        "ex057_sampler_states",
        "Build sampler state objects",
        "Textures & Samplers",
    ),
    (
        "058",
        "ex058_texture_copies",
        "copy_texture and copy_texture_region",
        "Textures & Samplers",
    ),
    (
        "059",
        "ex059_mipmap_generation",
        "generate_mipmaps on the GPU",
        "Textures & Samplers",
    ),
    (
        "060",
        "ex060_shared_textures",
        "Cross-process shareable textures",
        "Textures & Samplers",
    ),
    (
        "061",
        "ex061_command_buffer_labels",
        "Labels, device and queue back-references",
        "Sync, Events & Errors",
    ),
    (
        "062",
        "ex062_debug_groups",
        "Push/pop debug group boundaries",
        "Sync, Events & Errors",
    ),
    (
        "063",
        "ex063_gpu_submission_times",
        "GPU and kernel start/end timestamps",
        "Sync, Events & Errors",
    ),
    (
        "064",
        "ex064_encode_events",
        "encode_wait and encode_signal_event",
        "Sync, Events & Errors",
    ),
    (
        "065",
        "ex065_blit_fences",
        "update_fence and wait_for_fence in blit passes",
        "Sync, Events & Errors",
    ),
    (
        "066",
        "ex066_shared_events",
        "Shareable events and handles",
        "Sync, Events & Errors",
    ),
    (
        "067",
        "ex067_submission_ordering",
        "Sequential command buffers",
        "Sync, Events & Errors",
    ),
    (
        "068",
        "ex068_status_transitions",
        "CommandBufferStatus across a submission",
        "Sync, Events & Errors",
    ),
    (
        "069",
        "ex069_error_taxonomy",
        "InvalidArgument, Unsupported and Metal errors",
        "Sync, Events & Errors",
    ),
    (
        "070",
        "ex070_scheduled_callbacks",
        "on_scheduled and wait_until_scheduled",
        "Sync, Events & Errors",
    ),
    (
        "071",
        "ex071_timestamp_counter_heap",
        "Metal 4 counter heaps",
        "Metal 4, Timing & Capture",
    ),
    (
        "072",
        "ex072_counter_heap_limits",
        "MAX_TIMESTAMP_COUNTERS bounds",
        "Metal 4, Timing & Capture",
    ),
    (
        "073",
        "ex073_counter_sets",
        "Device counter sets",
        "Metal 4, Timing & Capture",
    ),
    (
        "074",
        "ex074_gpu_timing_measurement",
        "TimestampFrequency conversions",
        "Metal 4, Timing & Capture",
    ),
    (
        "075",
        "ex075_capture_session",
        "GPU trace capture to a file",
        "Metal 4, Timing & Capture",
    ),
    (
        "076",
        "ex076_capture_scopes",
        "Capture scopes for devices",
        "Metal 4, Timing & Capture",
    ),
    (
        "077",
        "ex077_metal_cpp_version",
        "metal-cpp version compatibility",
        "Metal 4, Timing & Capture",
    ),
    (
        "078",
        "ex078_layer_drawable",
        "CAMetalLayer drawable ownership",
        "Metal 4, Timing & Capture",
    ),
    (
        "079",
        "ex079_buffer_debug_markers",
        "add_debug_marker ranges",
        "Metal 4, Timing & Capture",
    ),
    (
        "080",
        "ex080_texture_barriers",
        "texture_barrier between draws",
        "Metal 4, Timing & Capture",
    ),
    (
        "081",
        "ex081_process_information",
        "NSProcessInfo through Foundation",
        "Foundation & Rust Interop",
    ),
    (
        "082",
        "ex082_foundation_values",
        "Number, URL and error domains",
        "Foundation & Rust Interop",
    ),
    (
        "083",
        "ex083_notification_center",
        "NSNotificationCenter observers",
        "Foundation & Rust Interop",
    ),
    (
        "084",
        "ex084_packed_float_math",
        "PackedFloat3, 4x3 and quaternion",
        "Foundation & Rust Interop",
    ),
    (
        "085",
        "ex085_string_extensions",
        "StringExt and Array helpers",
        "Foundation & Rust Interop",
    ),
    (
        "086",
        "ex086_repr_c_layout",
        "Fixed-layout Rust structs for MSL",
        "Foundation & Rust Interop",
    ),
    (
        "087",
        "ex087_error_handling",
        "Result pipelines and error kinds",
        "Foundation & Rust Interop",
    ),
    (
        "088",
        "ex088_bench_loop",
        "Measure repeated GPU submissions",
        "Foundation & Rust Interop",
    ),
    (
        "089",
        "ex089_feature_matrix",
        "Iterator-driven capability report",
        "Foundation & Rust Interop",
    ),
    (
        "090",
        "ex090_encoder_lifetimes",
        "Encoder borrows and end_encoding",
        "Foundation & Rust Interop",
    ),
    (
        "091",
        "ex091_mandelbrot_fractal",
        "Compute-rendered fractal saved to PPM",
        "Mini Projects",
    ),
    (
        "092",
        "ex092_matrix_multiplication",
        "8x8 f32 matmul with CPU verification",
        "Mini Projects",
    ),
    (
        "093",
        "ex093_game_of_life",
        "Conway's Life steps on the GPU",
        "Mini Projects",
    ),
    (
        "094",
        "ex094_histogram_256",
        "Atomics-driven byte histogram",
        "Mini Projects",
    ),
    (
        "095",
        "ex095_dot_product_reduction",
        "Parallel reduction to a scalar",
        "Mini Projects",
    ),
    (
        "096",
        "ex096_sobel_edge_filter",
        "Sobel edge detection on a pattern",
        "Mini Projects",
    ),
    (
        "097",
        "ex097_render_to_ppm",
        "Full render pipeline exported to PPM",
        "Mini Projects",
    ),
    (
        "098",
        "ex098_metalfx_spatial",
        "MetalFX spatial upscaling setup",
        "Mini Projects",
    ),
    (
        "099",
        "ex099_metalfx_temporal",
        "MetalFX temporal upscaling setup",
        "Mini Projects",
    ),
    (
        "100",
        "ex100_capstone_showcase",
        "End-to-end compute + render + readback",
        "Mini Projects",
    ),
];

/// Formats one run command for an example folder.
fn run_command(folder: &str) -> String {
    format!("cargo run -p {folder}")
}

fn main() {
    println!("metal-rust-examples — 100 examples for the metal-rust crate");
    println!("Run any example with the printed command. macOS with Metal required.\n");
    let mut chapter = "";
    for (number, folder, title, entry_chapter) in CATALOG {
        if entry_chapter != chapter {
            chapter = entry_chapter;
            println!("== {chapter} ==");
        }
        println!("  {number}. {title}");
        println!("       {}", run_command(folder));
    }
    println!("\n100 examples total. Happy GPU hacking!");
}
