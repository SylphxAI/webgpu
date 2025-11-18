import { describe, test, expect, beforeAll } from 'bun:test'
import { Gpu, type AdapterInfo, type AdapterLimits } from '../index.js'

describe('GPU Instance', () => {
  let gpu: ReturnType<typeof Gpu.create>

  beforeAll(() => {
    gpu = Gpu.create()
  })

  test('should create GPU instance', () => {
    expect(gpu).toBeDefined()
    expect(gpu).toHaveProperty('requestAdapter')
    expect(gpu).toHaveProperty('enumerateAdapters')
  })

  test('should enumerate adapters', () => {
    const adapters = gpu.enumerateAdapters()
    expect(Array.isArray(adapters)).toBe(true)
    expect(adapters.length).toBeGreaterThan(0)

    // Each adapter should be a string with name and backend
    adapters.forEach(adapter => {
      expect(typeof adapter).toBe('string')
      expect(adapter.length).toBeGreaterThan(0)
    })
  })

  test('should request adapter', async () => {
    const adapter = await gpu.requestAdapter()
    expect(adapter).toBeDefined()
    expect(adapter).toHaveProperty('getInfo')
    expect(adapter).toHaveProperty('getFeatures')
    expect(adapter).toHaveProperty('getLimits')
    expect(adapter).toHaveProperty('requestDevice')
  })

  test('should request high-performance adapter', async () => {
    const adapter = await gpu.requestAdapter('high-performance')
    expect(adapter).toBeDefined()

    const info = adapter.getInfo()
    expect(info).toBeDefined()
    expect(info.name).toBeTruthy()
  })

  test('should request low-power adapter', async () => {
    const adapter = await gpu.requestAdapter('low-power')
    expect(adapter).toBeDefined()
  })
})

describe('GPU Adapter', () => {
  let adapter: Awaited<ReturnType<ReturnType<typeof Gpu.create>['requestAdapter']>>

  beforeAll(async () => {
    const gpu = Gpu.create()
    adapter = await gpu.requestAdapter()
  })

  test('should get adapter info', () => {
    const info: AdapterInfo = adapter.getInfo()

    expect(info).toBeDefined()
    expect(typeof info.name).toBe('string')
    expect(info.name.length).toBeGreaterThan(0)
    expect(typeof info.vendor).toBe('number')
    expect(typeof info.device).toBe('number')
    expect(typeof info.deviceType).toBe('string')
    expect(typeof info.backend).toBe('string')

    // Backend should be one of the known types
    expect(['Metal', 'Vulkan', 'Dx12', 'Dx11', 'GL']).toContain(info.backend)
  })

  test('should get adapter features', () => {
    const features = adapter.getFeatures()

    expect(Array.isArray(features)).toBe(true)

    // Each feature should be a string
    features.forEach(feature => {
      expect(typeof feature).toBe('string')
    })
  })

  test('should get adapter limits', () => {
    const limits: AdapterLimits = adapter.getLimits()

    expect(limits).toBeDefined()
    expect(typeof limits.maxTextureDimension1D).toBe('number')
    expect(typeof limits.maxTextureDimension2D).toBe('number')
    expect(typeof limits.maxTextureDimension3D).toBe('number')
    expect(typeof limits.maxBindGroups).toBe('number')
    expect(typeof limits.maxBufferSize).toBe('number')

    // Sanity checks for reasonable values
    expect(limits.maxTextureDimension1D).toBeGreaterThan(0)
    expect(limits.maxTextureDimension2D).toBeGreaterThan(0)
    expect(limits.maxTextureDimension3D).toBeGreaterThan(0)
    expect(limits.maxBindGroups).toBeGreaterThanOrEqual(4)
    expect(limits.maxBufferSize).toBeGreaterThan(0)
  })

  test('should request device', async () => {
    const device = await adapter.requestDevice()

    expect(device).toBeDefined()
    expect(device).toHaveProperty('createBuffer')
    expect(device).toHaveProperty('createShaderModule')
    expect(device).toHaveProperty('createCommandEncoder')
    expect(device).toHaveProperty('queueSubmit')
    expect(device).toHaveProperty('poll')
    expect(device).toHaveProperty('destroy')
  })
})
