import { Elysia } from "elysia"
import type { AfterResponseHandler } from "elysia"
import { v4 as uuid } from "uuid"

import { logger } from "./libs/logger"
import { airdropController } from "./controllers/airdrop"

const app = new Elysia()
  .derive(() => {
    return {
      requestId: uuid(),
    }
  })
  .onRequest(() => {})
  .onAfterResponse((handler: AfterResponseHandler) => {
    const { requestId } = handler
    const { method, url, headers } = handler.request
    const { status } = handler.set
    logger.info({
      requestId,
      method,
      url,
      status,
      headers,
      params: handler.params || {},
      query: handler.query || {},
      body: handler.body || {},
    })
  })
  .get("/health", () => {})
  .use(airdropController)
  .listen(3001)

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
)
