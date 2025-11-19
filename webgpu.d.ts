/**
 * WebGPU Standard API Type Definitions
 *
 * This file provides TypeScript definitions for the WebGPU-standard wrapper API.
 * The wrapper provides 100% WebGPU standard-compliant API signatures.
 */

// Import native types
import * as Native from './index'

// Re-export native types that don't need wrapping
export {
    GpuShaderModule,
    GpuCommandEncoder,
    GpuCommandBuffer,
    GpuBuffer,
    GpuTexture,
    GpuTextureView,
    GpuBindGroupLayout,
    GpuBindGroup,
    GpuPipelineLayout,
    GpuComputePipeline,
    GpuRenderPipeline,
    GpuSampler,
    GpuQuerySet,
    GpuRenderBundle,
    GpuQueue,
    GpuSupportedFeatures,
    GpuComputePassEncoder,
    GpuRenderPassEncoder,
    BufferDescriptor,
    ShaderModuleDescriptor,
    TextureDescriptor,
    SamplerDescriptor,
    QuerySetDescriptor,
    CommandEncoderDescriptor,
    BindGroupLayoutDescriptor,
    ComputePassDescriptor,
    RenderPassDescriptor,
    AdapterInfo,
    AdapterLimits,
} from './index'

// WebGPU Standard Constants (UPPER_SNAKE_CASE)
export interface GPUBufferUsageFlags {
    readonly MAP_READ: number
    readonly MAP_WRITE: number
    readonly COPY_SRC: number
    readonly COPY_DST: number
    readonly INDEX: number
    readonly VERTEX: number
    readonly UNIFORM: number
    readonly STORAGE: number
    readonly INDIRECT: number
    readonly QUERY_RESOLVE: number
}

export interface GPUMapModeFlags {
    readonly READ: number
    readonly WRITE: number
}

export interface GPUTextureUsageFlags {
    readonly COPY_SRC: number
    readonly COPY_DST: number
    readonly TEXTURE_BINDING: number
    readonly STORAGE_BINDING: number
    readonly RENDER_ATTACHMENT: number
}

export const GPUBufferUsage: GPUBufferUsageFlags
export const GPUMapMode: GPUMapModeFlags
export const GPUTextureUsage: GPUTextureUsageFlags

// Legacy exports (backwards compatibility)
export const bufferUsage: GPUBufferUsageFlags
export const mapMode: GPUMapModeFlags
export const textureUsage: GPUTextureUsageFlags

// WebGPU Standard API Descriptors

export interface GPURequestAdapterOptions {
    powerPreference?: 'low-power' | 'high-performance'
    forceFallbackAdapter?: boolean
}

export interface GPUDeviceDescriptor {
    label?: string
    requiredFeatures?: string[]
    requiredLimits?: Record<string, number>
}

// Bind Group - WebGPU Standard
export interface GPUBindingResource {
    buffer?: Native.GpuBuffer
    offset?: number
    size?: number
}

export interface GPUBufferBinding {
    buffer: Native.GpuBuffer
    offset?: number
    size?: number
}

export type GPUBindingResourceType =
    | { buffer: Native.GpuBuffer; offset?: number; size?: number }
    | Native.GpuTextureView
    | Native.GpuSampler

export interface GPUBindGroupEntry {
    binding: number
    resource: GPUBindingResourceType
}

export interface GPUBindGroupDescriptor {
    label?: string
    layout: Native.GpuBindGroupLayout
    entries: GPUBindGroupEntry[]
}

// Pipeline Layout - WebGPU Standard
export interface GPUPipelineLayoutDescriptor {
    label?: string
    bindGroupLayouts: Native.GpuBindGroupLayout[]
}

// Compute Pipeline - WebGPU Standard
export interface GPUProgrammableStage {
    module: Native.GpuShaderModule
    entryPoint: string
}

export interface GPUComputePipelineDescriptor {
    label?: string
    layout: Native.GpuPipelineLayout | 'auto'
    compute: GPUProgrammableStage
}

// Render Pipeline - WebGPU Standard
export interface GPURenderPipelineDescriptor {
    label?: string
    layout: Native.GpuPipelineLayout | 'auto'
    vertex: {
        module: Native.GpuShaderModule
        entryPoint: string
        buffers?: any[]  // TODO: Add proper VertexBufferLayout types
    }
    primitive?: any  // TODO: Add proper PrimitiveState types
    depthStencil?: any  // TODO: Add proper DepthStencilState types
    multisample?: any  // TODO: Add proper MultisampleState types
    fragment?: {
        module: Native.GpuShaderModule
        entryPoint: string
        targets: any[]  // TODO: Add proper ColorTargetState types
    }
}

// GPU Classes - WebGPU Standard

export declare class GpuAdapter {
    readonly features: Native.GpuSupportedFeatures
    readonly limits: any
    readonly info: Native.AdapterInfo
    readonly isFallbackAdapter: boolean

    requestDevice(descriptor?: GPUDeviceDescriptor): Promise<GpuDevice>
}

export declare class GpuDevice {
    readonly queue: Native.GpuQueue
    readonly features: Native.GpuSupportedFeatures
    readonly limits: any
    readonly label: string | null

    // Error scopes
    pushErrorScope(filter: 'validation' | 'out-of-memory' | 'internal'): void
    popErrorScope(): Promise<string | null>

    // Resource creation
    createBuffer(descriptor: Native.BufferDescriptor): Native.GpuBuffer
    createTexture(descriptor: Native.TextureDescriptor): Native.GpuTexture
    createSampler(descriptor: Native.SamplerDescriptor): Native.GpuSampler
    createBindGroupLayout(descriptor: Native.BindGroupLayoutDescriptor): Native.GpuBindGroupLayout
    createShaderModule(descriptor: Native.ShaderModuleDescriptor): Native.GpuShaderModule
    createQuerySet(descriptor: Native.QuerySetDescriptor): Native.GpuQuerySet
    createCommandEncoder(descriptor?: Native.CommandEncoderDescriptor): Native.GpuCommandEncoder

    // WebGPU Standard API
    createBindGroup(descriptor: GPUBindGroupDescriptor): Native.GpuBindGroup
    createPipelineLayout(descriptor: GPUPipelineLayoutDescriptor): Native.GpuPipelineLayout
    createComputePipeline(descriptor: GPUComputePipelineDescriptor): Native.GpuComputePipeline
    createRenderPipeline(descriptor: GPURenderPipelineDescriptor): Native.GpuRenderPipeline
}

export declare class Gpu {
    requestAdapter(options?: GPURequestAdapterOptions): Promise<GpuAdapter | null>
}

// Main exports
export function Gpu(): Gpu

// Native bindings (advanced users)
export { Native as native }
