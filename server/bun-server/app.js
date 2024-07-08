var content1KB = require("./1KB.js");
var content10KB = require("./10KB.js");
var content100KB = require("./100KB.js");
var content10MB = require("./10MB.js");
var content5MB = require("./5MB.js");
var content1MB = require("./1MB.js");
const news1KB = { message: content1KB.content.msg }
const news10KB = { message: content10KB.content.msg }
const news100KB = { message: content100KB.content.msg }
const news5MB = { message: content5MB.content.msg }
const news10MB = { message: content10MB.content.msg }
const news1MB = { message: content1MB.content.msg }
import { Hono } from 'hono'


export const func_news1KB = async () => { return news1KB }
export const func_news10KB = async () => { return news10KB }
export const func_news100KB = async () => { return news100KB }
export const func_news1MB = async () => { return news1MB }
export const func_news5MB = async () => { return news5MB }
export const func_news10MB = async () => { return news10MB }

const app = new Hono()

app.get('/rest1kb', async (c) => {
  return c.json(news1KB)
})
app.get('/rest10kb', async (c) => {
  return c.json(news10KB)
})
app.get('/rest100kb', async (c) => {
  return c.json(news100KB)
})
app.get('/rest1mb', async (c) => {
  return c.json(news1MB)
})
app.get('/rest5mb', async (c) => {
  return c.json(news5MB)
})
app.get('/rest10mb', async (c) => {
  return c.json(news10MB)
})
console.log("🚀 Server started successfully");
const port = 50031

process.on("uncaughtException", (error) => {
  console.error("Uncaught exception:", error);
});
process.on("unhandledRejection", (reason, promise) => {
  console.error("Unhandled rejection:", reason);
});
const info = {
  app: app,
  type: 'hono',
  plugin_name: 'test',
  plugin: null
};
export { info };

export default {
  port,
  reusePort: true,
  fetch: app.fetch
}