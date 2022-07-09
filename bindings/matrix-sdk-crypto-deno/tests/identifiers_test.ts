// identifiers_test.ts
import { assertEquals } from "https://deno.land/std@0.147.0/testing/asserts.ts";
import { SimpleUserId } from "../bindings/bindings.ts"

Deno.test("UserId test", () => {
    const myUserId: SimpleUserId = { id: 1 };
    const theirUserId: SimpleUserId = { id: 1 };
    assertEquals(myUserId, theirUserId);
});