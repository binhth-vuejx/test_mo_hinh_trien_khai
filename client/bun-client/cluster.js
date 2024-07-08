import os from "node:os";
const numCPUs = os.cpus().length;
for (let i = 0; i < numCPUs; i++) {
  await Bun.spawn(["bun", "run", "--bun", "client.js"], {
    stdio: ["inherit", "inherit", "inherit"],
    cwd: `/app/`,
    env: { ...process.env }
  })
}