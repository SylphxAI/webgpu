import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

const readJson = (path) => JSON.parse(readFileSync(path, 'utf8'))
const readText = (path) => readFileSync(path, 'utf8')

test('project manifest remains valid project metadata without GroundAtlas product dogfood', () => {
  const manifest = readJson('project.manifest.json')

  assert.equal(manifest.schemaVersion, 1)
  assert.equal(manifest.project.id, 'webgpu')
  assert.equal(manifest.project.repository, 'https://github.com/SylphxAI/webgpu')
  assert.equal(manifest.project.visibility, 'open-source')
  assert.equal(manifest.adoption.status, 'adopted')
  assert.equal(manifest.truth.agentAdapter, 'AGENTS.md')
  assert.ok(
    manifest.surfaces.some(
      (surface) =>
        surface.path === '.doctrine/project.json' &&
        surface.description.includes('not the vendor-neutral GroundAtlas default'),
    ),
  )
  const commandNames = (manifest.commands || []).map((c) => c.name)
  assert.ok(!commandNames.includes('groundatlas:fleet'))
  assert.ok(
    String(manifest.adoption?.notes || '').includes('ADR-0014') ||
      String(manifest.adoption?.notes || '').toLowerCase().includes('retired'),
  )
})

test('Doctrine adapter remains Sylphx-specific and native release proof stays package-owned', () => {
  const doctrine = readJson('.doctrine/project.json')

  assert.equal(doctrine.project.repo, 'SylphxAI/webgpu')
  assert.equal(doctrine.adoption.status, 'migrating')
  assert.ok(
    doctrine.boundaries.publicSurfaces.some(
      (surface) => surface.type === 'manifest' && surface.location === 'project.manifest.json',
    ),
  )
  assert.ok(!String(doctrine.delivery?.productionProof || '').toLowerCase().includes('groundatlas'))
  assert.ok(doctrine.delivery.productionProof.includes('native artifact readback'))
})

test('CI does not pin GroundAtlas package/action for native build matrix', () => {
  const workflow = readText('.github/workflows/ci.yml')

  assert.ok(!workflow.includes('uses: SylphxAI/groundatlas@'))
  assert.ok(!workflow.includes('package-spec: groundatlas@'))
  assert.ok(!workflow.includes('needs: groundatlas'))
  assert.ok(workflow.includes('project.manifest.json') || workflow.includes('project-control') || workflow.includes('Build'))
})

test('release workflow keeps trusted publishing and package readback boundary', () => {
  const workflow = readText('.github/workflows/release.yml')

  assert.ok(workflow.includes('id-token: write'))
  assert.ok(workflow.includes('postpublish: bun run release:readback'))
  assert.ok(workflow.includes('SylphxAI/.github/.github/workflows/release.yml@main'))
})
