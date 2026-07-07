import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

const readJson = (path) => JSON.parse(readFileSync(path, 'utf8'))
const readText = (path) => readFileSync(path, 'utf8')

test('project manifest is the vendor-neutral GroundAtlas control file', () => {
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
  assert.ok(doctrine.delivery.productionProof.includes('GroundAtlas package dogfood'))
  assert.ok(doctrine.delivery.productionProof.includes('native artifact readback'))
})

test('CI gates native build matrix behind the released GroundAtlas package and action', () => {
  const workflow = readText('.github/workflows/ci.yml')

  assert.ok(workflow.includes('groundatlas:'))
  assert.ok(workflow.includes('needs: groundatlas'))
  assert.ok(workflow.includes('needs.groundatlas.result'))
  assert.ok(workflow.includes('uses: SylphxAI/groundatlas@v0.1.3'))
  assert.ok(workflow.includes('package-spec: groundatlas@0.1.3'))
  assert.ok(workflow.includes('fleet-markdown-report-path'))
  assert.ok(workflow.includes('require-atlas: "true"'))
  assert.ok(workflow.includes('strict: "true"'))
  assert.ok(workflow.includes('project.manifest.json'))
  assert.ok(workflow.includes('.doctrine/project.json'))
  assert.ok(workflow.includes('# GroundAtlas fleet adoption report'))
  assert.ok(workflow.includes('Summary: 1 adopted, 0 warning, 0 blocked, 1 total.'))
})

test('release workflow keeps trusted publishing and package readback boundary', () => {
  const workflow = readText('.github/workflows/release.yml')

  assert.ok(workflow.includes('id-token: write'))
  assert.ok(workflow.includes('postpublish: bun run release:readback'))
  assert.ok(workflow.includes('SylphxAI/.github/.github/workflows/release.yml@main'))
})
