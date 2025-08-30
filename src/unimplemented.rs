use crate::native;

#[no_mangle]
pub extern "C" fn wgpuGetProcAddress(
    _device: native::WGPUDevice,
    _proc_name: *const ::std::os::raw::c_char,
) -> native::WGPUProc {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupSetLabel(
    _bind_group: native::WGPUBindGroup,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupLayoutSetLabel(
    _bind_group_layout: native::WGPUBindGroupLayout,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferGetMapState(_buffer: native::WGPUBuffer) -> native::WGPUBufferMapState {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferGetSize(_buffer: native::WGPUBuffer) -> u64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferGetUsage(_buffer: native::WGPUBuffer) -> native::WGPUBufferUsage {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferSetLabel(
    _buffer: native::WGPUBuffer,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandBufferSetLabel(
    _command_buffer: native::WGPUCommandBuffer,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderSetLabel(
    _command_encoder: native::WGPUCommandEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderSetLabel(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePipelineSetLabel(
    _compute_pipeline: native::WGPUComputePipeline,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateComputePipelineAsync(
    _device: native::WGPUDevice,
    _descriptor: *const native::WGPUComputePipelineDescriptor,
    _callback: native::WGPUCreateComputePipelineAsyncCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceCreateRenderPipelineAsync(
    _device: native::WGPUDevice,
    _descriptor: *const native::WGPURenderPipelineDescriptor,
    _callback: native::WGPUCreateRenderPipelineAsyncCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDevicePopErrorScope(
    _device: native::WGPUDevice,
    _callback_info: native::WGPUPopErrorScopeCallbackInfo,
) -> bool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDevicePushErrorScope(
    _device: native::WGPUDevice,
    _filter: native::WGPUErrorFilter,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceSetLabel(
    _device: native::WGPUDevice,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceProcessEvents(_instance: native::WGPUInstance) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuPipelineLayoutSetLabel(
    _pipeline_layout: native::WGPUPipelineLayout,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetDestroy(_query_set: native::WGPUQuerySet) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetGetCount(_query_set: native::WGPUQuerySet) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetGetType(_query_set: native::WGPUQuerySet) -> native::WGPUQueryType {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetSetLabel(
    _query_set: native::WGPUQuerySet,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQueueOnSubmittedWorkDone(
    _queue: native::WGPUQueue,
    _callback: native::WGPUQueueWorkDoneCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQueueSetLabel(
    _queue: native::WGPUQueue,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderSetLabel(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderBeginOcclusionQuery(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _query_index: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderEndOcclusionQuery(
    _render_pass_encoder: native::WGPURenderPassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetLabel(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPipelineSetLabel(
    _render_pipeline: native::WGPURenderPipeline,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSamplerSetLabel(
    _sampler: native::WGPUSampler,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleGetCompilationInfo(
    _shader_module: native::WGPUShaderModule,
    _callback: native::WGPUCompilationInfoCallback,
    _userdata: *mut ::std::os::raw::c_void,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleSetLabel(
    _shader_module: native::WGPUShaderModule,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetDepthOrArrayLayers(_texture: native::WGPUTexture) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetDimension(
    _texture: native::WGPUTexture,
) -> native::WGPUTextureDimension {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetFormat(_texture: native::WGPUTexture) -> native::WGPUTextureFormat {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetHeight(_texture: native::WGPUTexture) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetMipLevelCount(_texture: native::WGPUTexture) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetSampleCount(_texture: native::WGPUTexture) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureGetUsage(_texture: native::WGPUTexture) -> native::WGPUTextureUsage {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn wgpuTextureGetWidth(_texture: native::WGPUTexture) -> u32 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureSetLabel(
    _texture: native::WGPUTexture,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureViewSetLabel(
    _texture_view: native::WGPUTextureView,
    _label: *const ::std::os::raw::c_char,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuGetInstanceFeatures(
    _features: *mut native::WGPUSupportedInstanceFeatures,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuGetInstanceLimits(
    _limits: *mut native::WGPUInstanceLimits,
) -> native::WGPUStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuHasInstanceFeature(
    _feature: native::WGPUInstanceFeatureName,
) -> native::WGPUBool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuAdapterGetFeatures(
    _adapter: native::WGPUAdapter,
    _features: *mut native::WGPUSupportedFeatures,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuAdapterAddRef(
    _adapter: native::WGPUAdapter,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuAdapterRelease(
    _adapter: native::WGPUAdapter,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupAddRef(
    _bind_group: native::WGPUBindGroup,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupRelease(
    _bind_group: native::WGPUBindGroup,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupLayoutAddRef(
    _bind_group_layout: native::WGPUBindGroupLayout,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBindGroupLayoutRelease(
    _bind_group_layout: native::WGPUBindGroupLayout,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferReadMappedRange(
    _buffer: native::WGPUBuffer,
    _offset: usize,
    _data: *mut ::std::os::raw::c_void,
    _size: usize,
) -> native::WGPUStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferWriteMappedRange(
    _buffer: native::WGPUBuffer,
    _offset: usize,
    _data: *const ::std::os::raw::c_void,
    _size: usize,
) -> native::WGPUStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferAddRef(
    _buffer: native::WGPUBuffer,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuBufferRelease(
    _buffer: native::WGPUBuffer,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandBufferAddRef(
    _command_buffer: native::WGPUCommandBuffer,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandBufferRelease(
    _command_buffer: native::WGPUCommandBuffer,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderBeginComputePass(
    _command_encoder: native::WGPUCommandEncoder,
    _descriptor: *const native::WGPUComputePassDescriptor,
) -> native::WGPUComputePassEncoder {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderBeginRenderPass(
    _command_encoder: native::WGPUCommandEncoder,
    _descriptor: *const native::WGPURenderPassDescriptor,
) -> native::WGPURenderPassEncoder {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderClearBuffer(
    _command_encoder: native::WGPUCommandEncoder,
    _buffer: native::WGPUBuffer,
    _offset: u64,
    _size: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyBufferToBuffer(
    _command_encoder: native::WGPUCommandEncoder,
    _source: native::WGPUBuffer,
    _source_offset: u64,
    _destination: native::WGPUBuffer,
    _destination_offset: u64,
    _size: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyBufferToTexture(
    _command_encoder: native::WGPUCommandEncoder,
    _source: *const native::WGPUTexelCopyBufferInfo,
    _destination: *const native::WGPUTexelCopyTextureInfo,
    _copy_size: *const native::WGPUExtent3D,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyTextureToBuffer(
    _command_encoder: native::WGPUCommandEncoder,
    _source: *const native::WGPUTexelCopyTextureInfo,
    _destination: *const native::WGPUTexelCopyBufferInfo,
    _copy_size: *const native::WGPUExtent3D,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderCopyTextureToTexture(
    _command_encoder: native::WGPUCommandEncoder,
    _source: *const native::WGPUTexelCopyTextureInfo,
    _destination: *const native::WGPUTexelCopyTextureInfo,
    _copy_size: *const native::WGPUExtent3D,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderFinish(
    _command_encoder: native::WGPUCommandEncoder,
    _descriptor: *const native::WGPUCommandBufferDescriptor,
) -> native::WGPUCommandBuffer {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderInsertDebugMarker(
    _command_encoder: native::WGPUCommandEncoder,
    _marker_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderPopDebugGroup(
    _command_encoder: native::WGPUCommandEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderPushDebugGroup(
    _command_encoder: native::WGPUCommandEncoder,
    _group_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderResolveQuerySet(
    _command_encoder: native::WGPUCommandEncoder,
    _query_set: native::WGPUQuerySet,
    _first_query: u32,
    _query_count: u32,
    _destination: native::WGPUBuffer,
    _destination_offset: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderWriteTimestamp(
    _command_encoder: native::WGPUCommandEncoder,
    _query_set: native::WGPUQuerySet,
    _query_index: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderAddRef(
    _command_encoder: native::WGPUCommandEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuCommandEncoderRelease(
    _command_encoder: native::WGPUCommandEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderDispatchWorkgroups(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _workgroup_count_x: u32,
    _workgroup_count_y: u32,
    _workgroup_count_z: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderDispatchWorkgroupsIndirect(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _indirect_buffer: native::WGPUBuffer,
    _indirect_offset: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderEnd(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderInsertDebugMarker(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _marker_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderPopDebugGroup(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderPushDebugGroup(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _group_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderSetBindGroup(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _group_index: u32,
    _group: native::WGPUBindGroup,
    _dynamic_offset_count: usize,
    _dynamic_offsets: *const u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderSetPipeline(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
    _pipeline: native::WGPUComputePipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderAddRef(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePassEncoderRelease(
    _compute_pass_encoder: native::WGPUComputePassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePipelineAddRef(
    _compute_pipeline: native::WGPUComputePipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuComputePipelineRelease(
    _compute_pipeline: native::WGPUComputePipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceGetAdapterInfo(
    _device: native::WGPUDevice,
    _adapter_info: *mut native::WGPUAdapterInfo,
) -> native::WGPUStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceGetFeatures(
    _device: native::WGPUDevice,
    _features: *mut native::WGPUSupportedFeatures,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceGetLostFuture(
    _device: native::WGPUDevice,
) -> native::WGPUFuture {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceAddRef(
    _device: native::WGPUDevice,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuDeviceRelease(
    _device: native::WGPUDevice,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceGetWGSLLanguageFeatures(
    _instance: native::WGPUInstance,
    _features: *mut native::WGPUSupportedWGSLLanguageFeatures,
) -> native::WGPUStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceHasWGSLLanguageFeature(
    _instance: native::WGPUInstance,
    _feature: native::WGPUWGSLLanguageFeatureName,
) -> native::WGPUBool {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceWaitAny(
    _instance: native::WGPUInstance,
    _future_count: usize,
    _futures: *mut native::WGPUFutureWaitInfo,
    _timeout_ns: u64,
) -> native::WGPUWaitStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceAddRef(
    _instance: native::WGPUInstance,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuInstanceRelease(
    _instance: native::WGPUInstance,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuPipelineLayoutAddRef(
    _pipeline_layout: native::WGPUPipelineLayout,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuPipelineLayoutRelease(
    _pipeline_layout: native::WGPUPipelineLayout,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetAddRef(
    _query_set: native::WGPUQuerySet,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQuerySetRelease(
    _query_set: native::WGPUQuerySet,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQueueAddRef(
    _queue: native::WGPUQueue,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuQueueRelease(
    _queue: native::WGPUQueue,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleSetLabel(
    _render_bundle: native::WGPURenderBundle,
    _label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleAddRef(
    _render_bundle: native::WGPURenderBundle,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleRelease(
    _render_bundle: native::WGPURenderBundle,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderDraw(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _vertex_count: u32,
    _instance_count: u32,
    _first_vertex: u32,
    _first_instance: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderDrawIndexed(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _index_count: u32,
    _instance_count: u32,
    _first_index: u32,
    _base_vertex: i32,
    _first_instance: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderDrawIndexedIndirect(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _indirect_buffer: native::WGPUBuffer,
    _indirect_offset: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderDrawIndirect(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _indirect_buffer: native::WGPUBuffer,
    _indirect_offset: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderFinish(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _descriptor: *const native::WGPURenderBundleDescriptor,
) -> native::WGPURenderBundle {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderInsertDebugMarker(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _marker_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderPopDebugGroup(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderPushDebugGroup(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _group_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderSetBindGroup(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _group_index: u32,
    _group: native::WGPUBindGroup,
    _dynamic_offset_count: usize,
    _dynamic_offsets: *const u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderSetIndexBuffer(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _buffer: native::WGPUBuffer,
    _format: native::WGPUIndexFormat,
    _offset: u64,
    _size: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderSetPipeline(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _pipeline: native::WGPURenderPipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderSetVertexBuffer(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
    _slot: u32,
    _buffer: native::WGPUBuffer,
    _offset: u64,
    _size: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderAddRef(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderBundleEncoderRelease(
    _render_bundle_encoder: native::WGPURenderBundleEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderDraw(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _vertex_count: u32,
    _instance_count: u32,
    _first_vertex: u32,
    _first_instance: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderDrawIndexed(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _index_count: u32,
    _instance_count: u32,
    _first_index: u32,
    _base_vertex: i32,
    _first_instance: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderDrawIndexedIndirect(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _indirect_buffer: native::WGPUBuffer,
    _indirect_offset: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderDrawIndirect(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _indirect_buffer: native::WGPUBuffer,
    _indirect_offset: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderEnd(
    _render_pass_encoder: native::WGPURenderPassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderExecuteBundles(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _bundle_count: usize,
    _bundles: *const native::WGPURenderBundle,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderInsertDebugMarker(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _marker_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderPopDebugGroup(
    _render_pass_encoder: native::WGPURenderPassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderPushDebugGroup(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _group_label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetBindGroup(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _group_index: u32,
    _group: native::WGPUBindGroup,
    _dynamic_offset_count: usize,
    _dynamic_offsets: *const u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetBlendConstant(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _color: *const native::WGPUColor,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetIndexBuffer(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _buffer: native::WGPUBuffer,
    _format: native::WGPUIndexFormat,
    _offset: u64,
    _size: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetPipeline(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _pipeline: native::WGPURenderPipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetScissorRect(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _x: u32,
    _y: u32,
    _width: u32,
    _height: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetStencilReference(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _reference: u32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetVertexBuffer(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _slot: u32,
    _buffer: native::WGPUBuffer,
    _offset: u64,
    _size: u64,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderSetViewport(
    _render_pass_encoder: native::WGPURenderPassEncoder,
    _x: f32,
    _y: f32,
    _width: f32,
    _height: f32,
    _min_depth: f32,
    _max_depth: f32,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderAddRef(
    _render_pass_encoder: native::WGPURenderPassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPassEncoderRelease(
    _render_pass_encoder: native::WGPURenderPassEncoder,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPipelineAddRef(
    _render_pipeline: native::WGPURenderPipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuRenderPipelineRelease(
    _render_pipeline: native::WGPURenderPipeline,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSamplerAddRef(
    _sampler: native::WGPUSampler,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSamplerRelease(
    _sampler: native::WGPUSampler,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleAddRef(
    _shader_module: native::WGPUShaderModule,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuShaderModuleRelease(
    _shader_module: native::WGPUShaderModule,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSupportedFeaturesFreeMembers(
    _supported_features: native::WGPUSupportedFeatures,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSupportedInstanceFeaturesFreeMembers(
    _supported_instance_features: native::WGPUSupportedInstanceFeatures,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSupportedWGSLLanguageFeaturesFreeMembers(
    _supported_wgsl_language_features: native::WGPUSupportedWGSLLanguageFeatures,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceConfigure(
    _surface: native::WGPUSurface,
    _config: *const native::WGPUSurfaceConfiguration,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceGetCurrentTexture(
    _surface: native::WGPUSurface,
    _surface_texture: *mut native::WGPUSurfaceTexture,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfacePresent(
    _surface: native::WGPUSurface,
) -> native::WGPUStatus {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceSetLabel(
    _surface: native::WGPUSurface,
    _label: native::WGPUStringView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceUnconfigure(
    _surface: native::WGPUSurface,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceAddRef(
    _surface: native::WGPUSurface,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceRelease(
    _surface: native::WGPUSurface,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuSurfaceCapabilitiesFreeMembers(
    _surface_capabilities: native::WGPUSurfaceCapabilities,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureAddRef(
    _texture: native::WGPUTexture,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureRelease(
    _texture: native::WGPUTexture,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureViewAddRef(
    _texture_view: native::WGPUTextureView,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wgpuTextureViewRelease(
    _texture_view: native::WGPUTextureView,
) {
    unimplemented!();
}

