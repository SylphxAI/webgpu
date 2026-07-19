import { describe, expect, it } from "bun:test";
import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..");

describe("native package behavior", () => {
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

  it("native binding exports GPU device constructors", () => {
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const gpu = require(join(root, "index.js")) as Record<string, unknown>;
    expect(gpu.GpuDevice || gpu.Gpu).toBeDefined();
    expect(gpu.GpuBuffer).toBeDefined();
  });
});
