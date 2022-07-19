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
 * Settings for an encrypted room.
 *
 * This determines the algorithm and rotation periods of a group
 * session.
 */
export type EncryptionSettings = {
  /**
   * The encryption algorithm that should be used in the room.
   */
  algorithm: EncryptionAlgorithm
  /**
   * How long the session should be used before changing it,
   * expressed in microseconds.
   */
  rotation_period: number
  /**
   * How many messages should be sent before changing the session.
   */
  rotation_period_messages: number
  /**
   * The history visibility of the room when the session was
   * created.
   */
  history_visibility: HistoryVisibility
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
export type SimpleUserId = {
  id: number
}
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
