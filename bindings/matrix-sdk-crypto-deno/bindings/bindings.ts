// Auto-generated with deno_bindgen
import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.1/plug.ts"
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}
function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}
function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as bigint)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}
const opts = {
  name: "matrix_sdk_crypto_deno",
  url: (new URL("../../target/debug", import.meta.url)).toString(),
  policy: CachePolicy.NONE,
}
const _lib = await prepare(opts, {})
/**
 * An encryption algorithm to be used to encrypt messages sent to a
 * room.
 */
export type EncryptionAlgorithm = /**
   * Olm version 1 using Curve25519, AES-256, and SHA-256.
   */
  | "OlmV1Curve25519AesSha2"
  | /**
   * Megolm version 1 using AES-256 and SHA-256.
   */
  "MegolmV1AesSha2"
/**
 * Represent the type of a request.
 */
export type RequestType = /**
   * Represents a `KeysUploadRequest`.
   */
  | "KeysUpload"
  | /**
   * Represents a `KeysQueryRequest`.
   */
  "KeysQuery"
  | /**
   * Represents a `KeysClaimRequest`.
   */
  "KeysClaim"
  | /**
   * Represents a `ToDeviceRequest`.
   */
  "ToDevice"
  | /**
   * Represents a `SignatureUploadRequest`.
   */
  "SignatureUpload"
  | /**
   * Represents a `RoomMessageRequest`.
   */
  "RoomMessage"
  | /**
   * Represents a `KeysBackupRequest`.
   */
  "KeysBackup"
/**
 * A Matrix [user ID].
 *
 * [user ID]: https://spec.matrix.org/v1.2/appendices/#user-identifiers
 */
export type SimpleUserId = {
  id: number
}
/**
 * The verification state of the device that sent an event to us.
 */
export type VerificationState = /**
   * The device is trusted.
   */
  | "Trusted"
  | /**
   * The device is not trusted.
   */
  "Untrusted"
  | /**
   * The device is not known to us.
   */
  "UnknownDevice"
/**
 * A request that will back up a batch of room keys to the server
 * ([specification]).
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3room_keyskeys
 */
export type KeysBackupRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"rooms": …}
   * ```
   */
  body: string
}
/**
 * Data for a request to the `/keys/query` API endpoint
 * ([specification]).
 *
 * Returns the current devices and identity keys for the given users.
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysquery
 */
export type KeysQueryRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"timeout": …, "device_keys": …, "token": …}
   * ```
   */
  body: string
}
/**
 * A customized owned request type for sending out room messages
 * ([specification]).
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3roomsroomidsendeventtypetxnid
 */
export type RoomMessageRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"room_id": …, "txn_id": …, "content": …}
   * ```
   */
  body: string
}
/**
 * Data for a request to the `/keys/claim` API endpoint
 * ([specification]).
 *
 * Claims one-time keys that can be used to establish 1-to-1 E2EE
 * sessions.
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysclaim
 */
export type KeysClaimRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"timeout": …, "one_time_keys": …}
   * ```
   */
  body: string
}
/**
 * Data for a request to the `/keys/upload` API endpoint
 * ([specification]).
 *
 * Publishes end-to-end encryption keys for the device.
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysupload
 */
export type KeysUploadRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"device_keys": …, "one_time_keys": …}
   * ```
   */
  body: string
}
/**
 * Data for a request to the `/sendToDevice` API endpoint
 * ([specification]).
 *
 * Send an event to a single device or to a group of devices.
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3sendtodeviceeventtypetxnid
 */
export type ToDeviceRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"event_type": …, "txn_id": …, "messages": …}
   * ```
   */
  body: string
}
/**
 * Who can see a room's history.
 */
export type HistoryVisibility = /**
   * Previous events are accessible to newly joined members from
   * the point they were invited onwards.
   *
   * Events stop being accessible when the member's state changes
   * to something other than *invite* or *join*.
   */
  | "Invited"
  | /**
   * Previous events are accessible to newly joined members from
   * the point they joined the room onwards.
   *
   * Events stop being accessible when the member's state changes
   * to something other than *join*.
   */
  "Joined"
  | /**
   * Previous events are always accessible to newly joined members.
   *
   * All events in the room are accessible, even those sent when
   * the member was not a part of the room.
   */
  "Shared"
  | /**
   * All events while this is the `HistoryVisibility` value may be
   * shared by any participating homeserver with anyone, regardless
   * of whether they have ever joined the room.
   */
  "WorldReadable"
/**
 * Data for a request to the `/keys/signatures/upload` API endpoint
 * ([specification]).
 *
 * Publishes cross-signing signatures for the user.
 *
 * [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keyssignaturesupload
 */
export type SignatureUploadRequest = {
  /**
   * The request ID.
   */
  id: string
  /**
   * A JSON-encoded object of form:
   *
   * ```json
   * {"signed_keys": …, "txn_id": …, "messages": …}
   * ```
   */
  body: string
}
