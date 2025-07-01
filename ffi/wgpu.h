#ifndef WGPU_H_
#define WGPU_H_

#include "webgpu-headers/webgpu.h"

typedef enum WGPUNativeSType {
    // Start at 6 to prevent collisions with webgpu STypes
    WGPUSType_DeviceExtras = 0x60000001,
    WGPUSType_AdapterExtras = 0x60000002,
    WGPUSType_LimitsExtras = 0x60000003,
    WGPUSType_PipelineLayoutExtras = 0x60000004,
    WGPUSType_ShaderSourceGLSL = 0x60000005,
    WGPUSType_InstanceExtras = 0x60000006,
    WGPUSType_SwapChainDescriptorExtras = 0x60000007,
    WGPUNativeSType_Force32 = 0x7FFFFFFF
} WGPUNativeSType;

typedef enum WGPUNativeFeature {
    WGPUNativeFeature_PUSH_CONSTANTS = 0x60000001,
    WGPUNativeFeature_TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES = 0x60000002,
    WGPUNativeFeature_MULTI_DRAW_INDIRECT = 0x60000003,
    WGPUNativeFeature_MULTI_DRAW_INDIRECT_COUNT = 0x60000004,
    WGPUNativeFeature_VERTEX_WRITABLE_STORAGE = 0x60000005,
    WGPUFeatureName_WgpuPipelineStatisticsQuery = 0x60000006,
} WGPUNativeFeature;

typedef enum WGPUWgpuQueryType {
    WGPUQueryType_WgpuPipelineStatistics = 0x60000001,
    WGPUWgpuQueryType_Force32 = 0x7FFFFFFF
} WGPUWgpuQueryType WGPU_ENUM_ATTRIBUTE;

typedef enum WGPULogLevel {
    WGPULogLevel_Off = 0x00000000,
    WGPULogLevel_Error = 0x00000001,
    WGPULogLevel_Warn = 0x00000002,
    WGPULogLevel_Info = 0x00000003,
    WGPULogLevel_Debug = 0x00000004,
    WGPULogLevel_Trace = 0x00000005,
    WGPULogLevel_Force32 = 0x7FFFFFFF
} WGPULogLevel;

typedef enum WGPUInstanceBackend {
    WGPUInstanceBackend_Vulkan = 1 << 1,
    WGPUInstanceBackend_GL = 1 << 5,
    WGPUInstanceBackend_Metal = 1 << 2,
    WGPUInstanceBackend_DX12 = 1 << 3,
    WGPUInstanceBackend_DX11 = 1 << 4,
    WGPUInstanceBackend_BrowserWebGPU = 1 << 6,
    WGPUInstanceBackend_Primary = WGPUInstanceBackend_Vulkan | WGPUInstanceBackend_Metal |
        WGPUInstanceBackend_DX12 |
        WGPUInstanceBackend_BrowserWebGPU,
    WGPUInstanceBackend_Secondary = WGPUInstanceBackend_GL | WGPUInstanceBackend_DX11,
    WGPUInstanceBackend_None = 0x00000000,
    WGPUInstanceBackend_Force32 = 0x7FFFFFFF
} WGPUInstanceBackend;
typedef WGPUFlags WGPUInstanceBackendFlags;

typedef enum WGPUDx12Compiler {
    WGPUDx12Compiler_Undefined = 0x00000000,
    WGPUDx12Compiler_Fxc = 0x00000001,
    WGPUDx12Compiler_Dxc = 0x00000002,
    WGPUDx12Compiler_Force32 = 0x7FFFFFFF
} WGPUDx12Compiler;

typedef struct WGPUInstanceExtras {
    WGPUChainedStruct chain;
    WGPUInstanceBackendFlags backends;
    WGPUDx12Compiler dx12ShaderCompiler;
    WGPUStringView dxilPath;
    WGPUStringView dxcPath;
} WGPUInstanceExtras;

typedef struct WGPUAdapterExtras {
    WGPUChainedStruct chain;
    WGPUBackendType backend;
} WGPUAdapterExtras;

typedef struct WGPUDeviceExtras {
    WGPUChainedStruct chain;
    WGPUStringView tracePath;
} WGPUDeviceExtras;

typedef struct WGPULimitsExtras {
    WGPUChainedStruct chain;
    uint32_t maxPushConstantSize;
    uint32_t maxInterStageShaderComponents;
} WGPURequiredLimitsExtras;

typedef struct WGPUPushConstantRange {
    WGPUShaderStage stages;
    uint32_t start;
    uint32_t end;
} WGPUPushConstantRange;

typedef struct WGPUPipelineLayoutExtras {
    WGPUChainedStruct chain;
    uint32_t pushConstantRangeCount;
    WGPUPushConstantRange* pushConstantRanges;
} WGPUPipelineLayoutExtras;

typedef uint64_t WGPUSubmissionIndex;

typedef struct WGPUWrappedSubmissionIndex {
    WGPUQueue queue;
    WGPUSubmissionIndex submissionIndex;
} WGPUWrappedSubmissionIndex;

typedef struct WGPUShaderDefine {
    WGPUStringView name;
    WGPUStringView value;
} WGPUShaderDefine;

typedef struct WGPUShaderSourceGLSL {
    WGPUChainedStruct chain;
    WGPUShaderStage stage;
    WGPUStringView code;
    uint32_t defineCount;
    WGPUShaderDefine * defines;
} WGPUShaderSourceGLSL;

typedef struct WGPUStorageReport {
    size_t numOccupied;
    size_t numVacant;
    size_t numError;
    size_t elementSize;
} WGPUStorageReport;

typedef struct WGPUHubReport {
    WGPUStorageReport adapters;
    WGPUStorageReport devices;
    WGPUStorageReport pipelineLayouts;
    WGPUStorageReport shaderModules;
    WGPUStorageReport bindGroupLayouts;
    WGPUStorageReport bindGroups;
    WGPUStorageReport commandBuffers;
    WGPUStorageReport renderBundles;
    WGPUStorageReport renderPipelines;
    WGPUStorageReport computePipelines;
    WGPUStorageReport querySets;
    WGPUStorageReport buffers;
    WGPUStorageReport textures;
    WGPUStorageReport textureViews;
    WGPUStorageReport samplers;
} WGPUHubReport;

typedef struct WGPUGlobalReport {
    WGPUStorageReport surfaces;
    WGPUBackendType backendType;
    WGPUHubReport vulkan;
    WGPUHubReport metal;
    WGPUHubReport dx12;
    WGPUHubReport dx11;
    WGPUHubReport gl;
} WGPUGlobalReport;

typedef struct WGPUSwapChainDescriptorExtras {
    WGPUChainedStruct chain;
    WGPUCompositeAlphaMode alphaMode;
    size_t viewFormatCount;
    WGPUTextureFormat const * viewFormats;
} WGPUSwapChainDescriptorExtras;

typedef void (*WGPULogCallback)(WGPULogLevel level, WGPUStringView message, void * userdata);

#ifdef __cplusplus
extern "C" {
#endif

void wgpuGenerateReport(WGPUInstance instance, WGPUGlobalReport* report);

WGPUSubmissionIndex wgpuQueueSubmitForIndex(WGPUQueue queue, uint32_t commandCount, WGPUCommandBuffer const * commands);

// Returns true if the queue is empty, or false if there are more queue submissions still in flight.
WGPUBool wgpuDevicePoll(WGPUDevice device, WGPUBool wait, WGPUWrappedSubmissionIndex const * wrappedSubmissionIndex);

void wgpuSetLogCallback(WGPULogCallback callback, void * userdata);

void wgpuSetLogLevel(WGPULogLevel level);

uint32_t wgpuGetVersion(void);

void wgpuRenderPassEncoderSetPushConstants(WGPURenderPassEncoder encoder, WGPUShaderStage stages, uint32_t offset, uint32_t sizeBytes, void* const data);

void wgpuRenderPassEncoderMultiDrawIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);
void wgpuRenderPassEncoderMultiDrawIndexedIndirect(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, uint32_t count);

void wgpuRenderPassEncoderMultiDrawIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);
void wgpuRenderPassEncoderMultiDrawIndexedIndirectCount(WGPURenderPassEncoder encoder, WGPUBuffer buffer, uint64_t offset, WGPUBuffer count_buffer, uint64_t count_buffer_offset, uint32_t max_count);

void wgpuInstanceDrop(WGPUInstance instance);
void wgpuAdapterDrop(WGPUAdapter adapter);
void wgpuBindGroupDrop(WGPUBindGroup bindGroup);
void wgpuBindGroupLayoutDrop(WGPUBindGroupLayout bindGroupLayout);
void wgpuBufferDrop(WGPUBuffer buffer);
void wgpuCommandBufferDrop(WGPUCommandBuffer commandBuffer);
void wgpuCommandEncoderDrop(WGPUCommandEncoder commandEncoder);
void wgpuRenderPassEncoderDrop(WGPURenderPassEncoder renderPassEncoder);
void wgpuComputePassEncoderDrop(WGPUComputePassEncoder computePassEncoder);
void wgpuRenderBundleEncoderDrop(WGPURenderBundleEncoder renderBundleEncoder);
void wgpuComputePipelineDrop(WGPUComputePipeline computePipeline);
void wgpuDeviceDrop(WGPUDevice device);
void wgpuPipelineLayoutDrop(WGPUPipelineLayout pipelineLayout);
void wgpuQuerySetDrop(WGPUQuerySet querySet);
void wgpuRenderBundleDrop(WGPURenderBundle renderBundle);
void wgpuRenderPipelineDrop(WGPURenderPipeline renderPipeline);
void wgpuSamplerDrop(WGPUSampler sampler);
void wgpuShaderModuleDrop(WGPUShaderModule shaderModule);
void wgpuSurfaceDrop(WGPUSurface surface);
void wgpuTextureDrop(WGPUTexture texture);
void wgpuTextureViewDrop(WGPUTextureView textureView);

#ifdef __cplusplus
} // extern "C"
#endif

#endif
