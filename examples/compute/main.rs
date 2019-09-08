use dxplr::{d3d12, d3d, dxgi};
use dxplr::d3d12::{ICommandAllocator, IDebug, IGraphicsCommandList, ICommandQueue, IDevice, IDescriptorHeap, IResource, IFence};
use dxplr::d3d::IBlob;
use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    let _d3d12_debug = {
        let debug = d3d12::get_debug_interface::<d3d12::Debug>().unwrap();
        debug.enable_debug_layer();
        debug
    };
    let device = d3d12::create_device::<d3d12::Device>(None, d3d::FeatureLevel(12, 0)).unwrap();
    let cmd_allocator = device.create_command_allocator::<d3d12::CommandAllocator>(d3d12::CommandListType::Direct).unwrap();
    let cmd_queue = device.create_command_queue::<d3d12::CommandQueue>(
        &d3d12::CommandQueueDesc::new()
            .list_type(d3d12::CommandListType::Direct)
    ).unwrap();
    let cmd_list = device.create_command_list::<d3d12::GraphicsCommandList>(
        0,
        d3d12::CommandListType::Direct,
        &cmd_allocator,
        None,
    ).unwrap();
    cmd_list.close();
    let fence = device.create_fence::<d3d12::Fence>(0, None).unwrap();
    let event = dxplr::EventHandle::new();

    let root_signature = {
        let desc = d3d12::RootSignatureDesc {
            parameters: Some(vec![
                d3d12::RootParameter::DescriptorTable {
                    descriptor_ranges: vec![d3d12::DescriptorRange {
                        range_type: d3d12::DescriptorRangeType::UAV,
                        num_descriptors: 1,
                        base_shader_register: 0,
                        register_space: 0,
                        offset_in_descriptors_from_table_start: d3d12::DescriptorRange::OFFSET_APPEND,
                    }],
                    shader_visibility: d3d12::ShaderVisibility::All,
                },
            ]),
            static_samplers: None,
            flags: None,
        };
        let data = d3d12::serialize_root_signature(&desc, d3d::RootSignatureVersion(1, 0)).unwrap();
        device.create_root_signature(0, data.as_slice()).unwrap()
    };
    let pipeline = {
        let file = File::open("examples/compute/compute.hlsl").unwrap();
        let mut reader = BufReader::new(file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();
        let cs_bin = d3d::compile(
            &data,
            Some("compute.hlsl"),
            None,
            "cs_main",
            "cs_5_0",
            Some(d3d::CompileFlags::Debug),
            None
        ).unwrap();
        device.create_compute_pipeline_state(
            &d3d12::ComputePipelineStateDesc::new()
                .root_signature(&root_signature)
                .cs(cs_bin.into())
        ).unwrap()
    };

    let descriptor_heap = device.create_descriptor_heap::<d3d12::DescriptorHeap>(
        &d3d12::DescriptorHeapDesc::new()
            .heap_type(d3d12::DescriptorHeapType::CBVSRVUAV)
            .num_descriptors(1)
            .flags(d3d12::DescriptorHeapFlags::ShaderVisible),
    ).unwrap();
    let buffer = device.create_committed_resource::<d3d12::Resource>(
        &d3d12::HeapProperties::new()
            .heap_type(d3d12::HeapType::Default),
        None,
        &d3d12::ResourceDesc::new()
            .dimension(d3d12::ResourceDimension::Buffer)
            .width(256)
            .height(1)
            .format(dxgi::Format::Unknown)
            .layout(d3d12::TextureLayout::RowMajor)
            .flags(d3d12::ResourceFlags::AllowUnorderedAccess),
        d3d12::ResourceStates::UnorderedAccess,
        None
    ).unwrap();
    unsafe {
        let handle = descriptor_heap.get_cpu_descriptor_handle_for_heap_start();
        device.create_unordered_access_view(
            Some(&buffer),
            None,
            Some(&d3d12::UnorderedAccessViewDesc::Buffer {
                format: dxgi::Format::Unknown,
                first_element: 0,
                num_elements: 64,
                structure_byte_stride: 4,
                counter_offset_in_bytes: 0,
                flags: None,
            }),
            handle
        );
    }
    let upload_buffer = device.create_committed_resource::<d3d12::Resource>(
        &d3d12::HeapProperties::new()
            .heap_type(d3d12::HeapType::Upload),
        None,
        &d3d12::ResourceDesc::new()
            .dimension(d3d12::ResourceDimension::Buffer)
            .width(256)
            .height(1)
            .format(dxgi::Format::Unknown)
            .layout(d3d12::TextureLayout::RowMajor),
        d3d12::ResourceStates::GenericRead,
        None
    ).unwrap();
    unsafe {
        let data = upload_buffer.map(0, None).unwrap();
        let a = (0..10).collect::<Vec<_>>();
        std::ptr::copy_nonoverlapping(a.as_ptr() as *const u8, data.as_mut_ptr() as *mut u8, (std::mem::size_of::<u32>() * 10) as usize);
        upload_buffer.unmap(0, None);
    }
    let readback_buffer = device.create_committed_resource::<d3d12::Resource>(
        &d3d12::HeapProperties::new()
            .heap_type(d3d12::HeapType::ReadBack),
        None,
        &d3d12::ResourceDesc::new()
            .dimension(d3d12::ResourceDimension::Buffer)
            .width(256)
            .height(1)
            .format(dxgi::Format::Unknown)
            .layout(d3d12::TextureLayout::RowMajor),
        d3d12::ResourceStates::CopyDest,
        None
    ).unwrap();

    cmd_allocator.reset().unwrap();
    cmd_list.reset(&cmd_allocator, Some(&pipeline)).unwrap();
    cmd_list.resource_barrier(&[d3d12::ResourceBarrier::Transition {
        flags: None,
        resource: &buffer,
        subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
        state_before: d3d12::ResourceStates::UnorderedAccess,
        state_after: d3d12::ResourceStates::CopyDest
    }]);
    cmd_list.copy_resource(&buffer, &upload_buffer);
    cmd_list.resource_barrier(&[d3d12::ResourceBarrier::Transition {
        flags: None,
        resource: &buffer,
        subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
        state_before: d3d12::ResourceStates::CopyDest,
        state_after: d3d12::ResourceStates::UnorderedAccess
    }]);
    cmd_list.close();
    cmd_queue.execute_command_lists(&[cmd_list.as_command_list()]);
    cmd_queue.signal(&fence, 1).unwrap();
    if fence.get_completed_value() < 1 {
        fence.set_event_on_completion(1, &event).unwrap();
        event.wait(None);
    }

    cmd_allocator.reset().unwrap();
    cmd_list.reset(&cmd_allocator, Some(&pipeline)).unwrap();
    cmd_list.set_descriptor_heaps(&[&descriptor_heap]);
    cmd_list.set_compute_root_signature(&root_signature);
    cmd_list.set_compute_root_descriptor_table(0, descriptor_heap.get_gpu_descriptor_handle_for_heap_start());
    cmd_list.dispatch(1, 1, 1);
    cmd_list.resource_barrier(&[d3d12::ResourceBarrier::Transition {
        flags: None,
        resource: &buffer,
        subresource: d3d12::RESOURCE_BARRIER_ALL_SUBRESOURCES,
        state_before: d3d12::ResourceStates::UnorderedAccess,
        state_after: d3d12::ResourceStates::CopySource
    }]);
    cmd_list.copy_resource(&readback_buffer, &buffer);
    cmd_list.close();
    cmd_queue.execute_command_lists(&[cmd_list.as_command_list()]);
    cmd_queue.signal(&fence, 2).unwrap();
    if fence.get_completed_value() < 2 {
        fence.set_event_on_completion(2, &event).unwrap();
        event.wait(None);
    }

    let mut values: [u32; 10] = [0; 10];
    unsafe {
        let data = readback_buffer.map(0, Some(d3d12::Range::new(0, 256))).unwrap();
        std::ptr::copy_nonoverlapping(data.as_ptr(), values.as_mut_ptr() as *mut u8, std::mem::size_of::<u32>() * 10);
        readback_buffer.unmap(0, None);
    }
    for i in &values {
        print!("{}, ", i);
    }
    println!("");
}