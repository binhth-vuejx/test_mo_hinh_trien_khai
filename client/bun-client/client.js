import { Hono } from 'hono'
import { Agent } from 'http';
const app_plugin = require('./app.min.js')

const vuejxAgent = new Agent({ keepAlive: true })
const headerFetch = {
  agent: vuejxAgent,
  maxContentLength: Infinity,
  maxBodyLength: Infinity,
  keepAlive: true
}
const app = new Hono()

if (app_plugin?.info?.app) app.route(`/${app_plugin.info.plugin_name}`, app_plugin.info.app)

app.get('/rest1kb', async (c) => {
  const response = await fetch("http://192.168.68.105:50031/rest1kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest10kb', async (c) => {
  const response = await fetch("http://192.168.68.105:50031/rest10kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest100kb', async (c) => {
  const response = await fetch("http://192.168.68.105:50031/rest100kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest1mb', async (c) => {
  const response = await fetch("http://192.168.68.105:50031/rest1mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest5mb', async (c) => {
  const response = await fetch("http://192.168.68.105:50031/rest5mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest10mb', async (c) => {
  const response = await fetch("http://192.168.68.105:50031/rest10mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest1kb_bad_2', async (c) => {
  await fetch("http://192.168.68.105:50031/rest1kb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest1kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest10kb_bad_2', async (c) => {
  await fetch("http://192.168.68.105:50031/rest10kb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest10kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest100kb_bad_2', async (c) => {
  await fetch("http://192.168.68.105:50031/rest100kb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest100kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest1mb_bad_2', async (c) => {
  await fetch("http://192.168.68.105:50031/rest1mb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest1mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest5mb_bad_2', async (c) => {
  await fetch("http://192.168.68.105:50031/rest5mb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest5mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest10mb_bad_2', async (c) => {
  await fetch("http://192.168.68.105:50031/rest10mb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest10mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest1kb_bad_3', async (c) => {
  await fetch("http://192.168.68.105:50031/rest1kb", headerFetch);
  await fetch("http://192.168.68.105:50031/rest1kb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest1kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest10kb_bad_3', async (c) => {
  await fetch("http://192.168.68.105:50031/rest10kb", headerFetch);
  await fetch("http://192.168.68.105:50031/rest10kb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest10kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest100kb_bad_3', async (c) => {
  await fetch("http://192.168.68.105:50031/rest100kb", headerFetch);
  await fetch("http://192.168.68.105:50031/rest100kb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest100kb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest1mb_bad_3', async (c) => {
  await fetch("http://192.168.68.105:50031/rest1mb", headerFetch);
  await fetch("http://192.168.68.105:50031/rest1mb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest1mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest5mb_bad_3', async (c) => {
  await fetch("http://192.168.68.105:50031/rest5mb", headerFetch);
  await fetch("http://192.168.68.105:50031/rest5mb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest5mb", headerFetch);
  return c.json(await response.json())
})
app.get('/rest10mb_bad_3', async (c) => {
  await fetch("http://192.168.68.105:50031/rest10mb", headerFetch);
  await fetch("http://192.168.68.105:50031/rest10mb", headerFetch);
  const response = await fetch("http://192.168.68.105:50031/rest10mb", headerFetch);
  return c.json(await response.json())
})


app.get('/best_rest1kb', async (c) => {
  const response = await app_plugin.func_news1KB()
  return c.json(response)
})
app.get('/best_rest10kb', async (c) => {
  const response = await app_plugin.func_news10KB()
  return c.json(response)
})
app.get('/best_rest100kb', async (c) => {
  const response = await app_plugin.func_news100KB()
  return c.json(response)
})
app.get('/best_rest1mb', async (c) => {
  const response = await app_plugin.func_news1MB()
  return c.json(response)
})
app.get('/best_rest5mb', async (c) => {
  const response = await app_plugin.func_news5MB()
  return c.json(response)
})
app.get('/best_rest10mb', async (c) => {
  const response = await app_plugin.func_news10MB()
  return c.json(response)
})
app.get('/best_rest1kb_bad_2', async (c) => {
  await app_plugin.func_news1KB()
  const response = await app_plugin.func_news1KB()
  return c.json(response)
})
app.get('/best_rest10kb_bad_2', async (c) => {
  await app_plugin.func_news10KB()
  const response = await app_plugin.func_news10KB()
  return c.json(response)
})
app.get('/best_rest100kb_bad_2', async (c) => {
  await app_plugin.func_news100KB()
  const response = await app_plugin.func_news100KB()
  return c.json(response)
})
app.get('/best_rest1mb_bad_2', async (c) => {
  await app_plugin.func_news1MB()
  const response = await app_plugin.func_news1MB()
  return c.json(response)
})
app.get('/best_rest5mb_bad_2', async (c) => {
  await app_plugin.func_news5MB()
  const response = await app_plugin.func_news5MB()
  return c.json(response)
})
app.get('/best_rest10mb_bad_2', async (c) => {
  await app_plugin.func_news10MB()
  const response = await app_plugin.func_news10MB()
  return c.json(response)
})
app.get('/best_rest1kb_bad_3', async (c) => {
  await app_plugin.func_news1KB()
  await app_plugin.func_news1KB()
  const response = await app_plugin.func_news1KB()
  return c.json(response)
})
app.get('/best_rest10kb_bad_3', async (c) => {
  await app_plugin.func_news10KB()
  await app_plugin.func_news10KB()
  const response = await app_plugin.func_news10KB()
  return c.json(response)
})
app.get('/best_rest100kb_bad_3', async (c) => {
  await app_plugin.func_news100KB()
  await app_plugin.func_news100KB()
  const response = await app_plugin.func_news100KB()
  return c.json(response)
})
app.get('/best_rest1mb_bad_3', async (c) => {
  await app_plugin.func_news1MB()
  await app_plugin.func_news1MB()
  const response = await app_plugin.func_news1MB()
  return c.json(response)
})
app.get('/best_rest5mb_bad_3', async (c) => {
  await app_plugin.func_news5MB()
  await app_plugin.func_news5MB()
  const response = await app_plugin.func_news5MB()
  return c.json(response)
})
app.get('/best_rest10mb_bad_3', async (c) => {
  await app_plugin.func_news10MB()
  await app_plugin.func_news10MB()
  const response = await app_plugin.func_news10MB()
  return c.json(response)
})

console.log("🚀 Server started successfully");

process.on("uncaughtException", (error) => {
  console.error("Uncaught exception:", error);
});
process.on("unhandledRejection", (reason, promise) => {
  console.error("Unhandled rejection:", reason);
});

const port = 50030
export default {
  port,
  reusePort: true,
  fetch: app.fetch
}