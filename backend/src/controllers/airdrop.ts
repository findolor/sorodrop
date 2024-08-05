import { Elysia, t } from "elysia"

const airdropController = new Elysia({ prefix: "/airdrop" })
  .get("/", () => "Airdrop")
  .post("/claim/:id", () => "Claim", {
    body: t.Object({
      key1: t.String(),
      key2: t.String(),
    }),
    query: t.Object({
      param1: t.String(),
      param2: t.String(),
    }),
    params: t.Object({
      id: t.String(),
    }),
  })
  .get("/status", () => "Status")

export { airdropController }
