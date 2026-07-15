import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

const root = join(import.meta.dir, "..");

describe("native authority matrix (adversarial ts_deleted admission)", () => {
  it("ledger records all capabilities ts_deleted", () => {
    const ledger = JSON.parse(
      readFileSync(join(root, "docs/specs/migration-ledger.json"), "utf8"),
    ) as { capabilities: Array<{ id: string; state: string }>; summary: { ts_deleted: number } };
    for (const c of ledger.capabilities) {
      expect(c.state).toBe("ts_deleted");
    }
    expect(ledger.summary.ts_deleted).toBe(ledger.capabilities.length);
  });

  it("ships a native .node artifact", () => {
    const natives = [
      ...readdirSync(root).filter((f) => f.endsWith(".node")),
      ...(existsSync(join(root, "npm"))
        ? readdirSync(join(root, "npm"), { withFileTypes: true })
            .filter((d) => d.isDirectory())
            .flatMap((d) =>
              readdirSync(join(root, "npm", d.name))
                .filter((f) => f.endsWith(".node"))
                .map((f) => join("npm", d.name, f)),
            )
        : []),
    ];
    expect(natives.length).toBeGreaterThan(0);
  });

  it("check-native-authority gate passes", () => {
    const r = spawnSync("bash", ["scripts/check-native-authority.sh"], {
      cwd: root,
      encoding: "utf8",
    });
    expect(r.status).toBe(0);
    expect(r.stdout).toContain("PASS");
  });

  it("native binding exports GPU device constructors", () => {
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const gpu = require(join(root, "index.js")) as Record<string, unknown>;
    expect(gpu.GpuDevice || gpu.Gpu).toBeDefined();
    expect(gpu.GpuBuffer).toBeDefined();
  });
});
