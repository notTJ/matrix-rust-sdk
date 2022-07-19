use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
// use error::into_err;
// use js_sys::{Array, Map, Promise, Set};
use ruma::{
    events::room::encrypted::OriginalSyncRoomEncryptedEvent, DeviceKeyAlgorithm,
    OwnedTransactionId, UInt,
};
use serde_json::Value as JsonValue;
use zeroize::Zeroize;
use crate::Result;
// use crate::JsValue;

use crate::{
    //downcast,
    encryption,
    // future::future_to_promise,
    identifiers, requests,
    requests::OutgoingRequest,
    responses::{self, response_from_string},
    sync_events,
    errors::into_err,
    either,
};

// #[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct OlmMachine {
    inner: matrix_sdk_crypto::OlmMachine,
}

impl OlmMachine {

pub async fn initialize(
    user_id: &identifiers::UserId,
    device_id: &identifiers::DeviceId,
    store_path: Option<String>,
    mut store_passphrase: Option<String>,
) -> Result<OlmMachine> {
    let store = store_path
        .map(|store_path| {
            matrix_sdk_sled::CryptoStore::open_with_passphrase(
                store_path,
                store_passphrase.as_deref(),
            )
            .map(Arc::new)
            .map_err(into_err)
        })
        .transpose()?;

    store_passphrase.zeroize();

    Ok(OlmMachine {
        inner: match store {
            Some(store) => matrix_sdk_crypto::OlmMachine::with_store(
                user_id.inner.as_ref(),
                device_id.inner.as_ref(),
                store,
            )
            .await
            .map_err(into_err)?,
            None => {
                matrix_sdk_crypto::OlmMachine::new(
                    user_id.inner.as_ref(),
                    device_id.inner.as_ref(),
                )
                .await
            }
        },
    })
}


    /// It's not possible to construct an `OlmMachine` with its
    /// constructor because building an `OlmMachine` is
    /// asynchronous. Please use the `finalize` method.
    // #[napi(constructor)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<()> {
        Err(crate::errors::DenoError::from_reason(
            "To build an `OldMachine`, please use the `initialize` method",
        ))
    }

    /// The unique user ID that owns this `OlmMachine` instance.
    // #[napi(getter)]
    pub fn user_id(&self) -> identifiers::UserId {
        identifiers::UserId::from(self.inner.user_id().to_owned())
    }

    /// The unique device ID that identifies this `OlmMachine`.
    // #[napi(getter)]
    pub fn device_id(&self) -> identifiers::DeviceId {
        identifiers::DeviceId::from(self.inner.device_id().to_owned())
    }

    /// Get the public parts of our Olm identity keys.
    // #[napi(getter)]
    pub fn identity_keys(&self) -> IdentityKeys {
        self.inner.identity_keys().into()
    }

    /// Handle a to-device and one-time key counts from a sync response.
    ///
    /// This will decrypt and handle to-device events returning the
    /// decrypted versions of them, as a JSON-encoded string.
    ///
    /// To decrypt an event from the room timeline, please use
    /// `decrypt_room_event`.
    ///
    /// # Arguments
    ///
    /// * `to_device_events`, thhe to-device events of the current sync
    ///   response.
    /// * `changed_devices`, the list of devices that changed in this sync
    ///   response.
    /// * `one_time_keys_count`, the current one-time keys counts that the sync
    ///   response returned.
    // #[napi]
    pub async fn receive_sync_changes(
        &self,
        to_device_events: String,
        changed_devices: &sync_events::DeviceLists,
        one_time_key_counts: HashMap<String, u32>,
        unused_fallback_keys: Vec<String>,
    ) -> Result<String> {
        let to_device_events = serde_json::from_str(to_device_events.as_ref()).map_err(into_err)?;
        let changed_devices = &changed_devices.inner;
        let one_time_key_counts = one_time_key_counts
            .iter()
            .map(|(key, value)| (DeviceKeyAlgorithm::from(key.as_str()), UInt::from(*value)))
            .collect::<BTreeMap<_, _>>();
        let unused_fallback_keys = Some(
            unused_fallback_keys
                .into_iter()
                .map(|key| DeviceKeyAlgorithm::from(key.as_str()))
                .collect::<Vec<_>>(),
        );

        serde_json::to_string(
            &self
                .inner
                .receive_sync_changes(
                    to_device_events,
                    changed_devices,
                    &one_time_key_counts,
                    unused_fallback_keys.as_deref(),
                )
                .await
                .map_err(into_err)?,
        )
        .map_err(into_err)
    }

    /// Get the outgoing requests that need to be sent out.
    ///
    /// This returns a list of `KeysUploadRequest`, or
    /// `KeysQueryRequest`, or `KeysClaimRequest`, or
    /// `ToDeviceRequest`, or `SignatureUploadRequest`, or
    /// `RoomMessageRequest`, or `KeysBackupRequest`. Those requests
    /// need to be sent out to the server and the responses need to be
    /// passed back to the state machine using `mark_request_as_sent`.
    // #[napi]
    // pub async fn outgoing_requests(
    //     &self,
    // ) -> Result<
    //     Vec<
    //         // We could be tempted to use `requests::OutgoingRequests` as its
    //         // a type alias for this giant `Either7`. But `napi` won't unfold
    //         // it properly into a valid TypeScript definition, so…  let's
    //         // copy-paste :-(.
    //         Either7<
    //             requests::KeysUploadRequest,
    //             requests::KeysQueryRequest,
    //             requests::KeysClaimRequest,
    //             requests::ToDeviceRequest,
    //             requests::SignatureUploadRequest,
    //             requests::RoomMessageRequest,
    //             requests::KeysBackupRequest,
    //         >,
    //     >,
    // > {
    //     self.inner
    //         .outgoing_requests()
    //         .await
    //         .map_err(into_err)?
    //         .into_iter()
    //         .map(requests::OutgoingRequest)
    //         .map(TryFrom::try_from)
    //         .collect::<Result<Vec<_>, _>>()
    //         .map_err(into_err)
    // }

    // pub fn outgoing_requests(&self) -> Promise {
    //     let me = self.inner.clone();

    //         Ok(me
    //             .outgoing_requests()
    //             .await?
    //             .into_iter()
    //             .map(OutgoingRequest)
    //             .map(TryFrom::try_from)
    //             .collect::<Result<Vec<JsValue>, _>>()?
    //             .into_iter()
    //             .collect::<Array>());
    // }

    /// Mark the request with the given request ID as sent.
    ///
    /// # Arguments
    ///
    /// * `request_id`, the unique ID of the request that was sent out. This is
    ///   needed to couple the response with the now sent out request.
    /// * `request_type`, the request type associated to the request ID.
    /// * `response`, the response that was received from the server after the
    ///   outgoing request was sent out.
    // #[napi]
    pub async fn mark_request_as_sent(
        &self,
        request_id: String,
        request_type: requests::RequestType,
        response: String,
    ) -> Result<bool> {
        let transaction_id = OwnedTransactionId::from(request_id);
        let response = response_from_string(response.as_str()).map_err(into_err)?;
        let incoming_response = responses::OwnedResponse::try_from((request_type, response))?;

        self.inner
            .mark_request_as_sent(&transaction_id, &incoming_response)
            .await
            .map(|_| true)
            .map_err(into_err)
    }

    /// Get the a key claiming request for the user/device pairs that
    /// we are missing Olm sessions for.
    ///
    /// Returns `null` if no key claiming request needs to be sent
    /// out.
    ///
    /// Sessions need to be established between devices so group
    /// sessions for a room can be shared with them.
    ///
    /// This should be called every time a group session needs to be
    /// shared as well as between sync calls. After a sync some
    /// devices may request room keys without us having a valid Olm
    /// session with them, making it impossible to server the room key
    /// request, thus it’s necessary to check for missing sessions
    /// between sync as well.
    ///
    /// Note: Care should be taken that only one such request at a
    /// time is in flight, e.g. using a lock.
    ///
    /// The response of a successful key claiming requests needs to be
    /// passed to the `OlmMachine` with the `mark_request_as_sent`.
    ///
    /// # Arguments
    ///
    /// * `users`, the list of users that we should check if we lack a session
    ///   with one of their devices. This can be an empty array or `null` when
    ///   calling this method between sync requests.
    // #[napi]
    pub async fn get_missing_sessions(
        &self,
        users: Option<Vec<&identifiers::UserId>>,
    ) -> Result<Option<requests::KeysClaimRequest>> {
        match self
            .inner
            .get_missing_sessions(identifiers::lower_user_ids_to_ruma(users.unwrap_or_default()))
            .await
            .map_err(into_err)?
        {
            Some((transaction_id, keys_claim_request)) => Ok(Some(
                requests::KeysClaimRequest::try_from((
                    transaction_id.to_string(),
                    &keys_claim_request,
                ))
                .map_err(into_err)?,
            )),

            None => Ok(None),
        }
    }

    /// Update the tracked users.
    ///
    /// This will mark users that weren’t seen before for a key query
    /// and tracking.
    ///
    /// If the user is already known to the Olm machine it will not be
    /// considered for a key query.
    ///
    /// # Arguments
    ///
    /// * `users`, an array over user IDs that should be marked for tracking.
    // #[napi]
    pub async fn update_tracked_users(&self, users: Vec<&identifiers::UserId>) {
        self.inner.update_tracked_users(identifiers::lower_user_ids_to_ruma(users)).await;
    }

    /// Get to-device requests to share a room key with users in a room.
    ///
    /// # Arguments
    ///
    /// * `room_id`, the room ID of the room where the room key will be used.
    /// * `users`, the list of users that should receive the room key.
    /// * `encryption_settings`, the encryption settings.
    // #[napi]
    pub async fn share_room_key(
        &self,
        room_id: &identifiers::RoomId,
        users: Vec<&identifiers::UserId>,
        encryption_settings: &encryption::EncryptionSettings,
    ) -> Result<String> {
        let room_id = room_id.inner.as_ref();
        let users = identifiers::lower_user_ids_to_ruma(users);
        let encryption_settings =
            matrix_sdk_crypto::olm::EncryptionSettings::from(encryption_settings);

        serde_json::to_string(
            &self
                .inner
                .share_room_key(room_id, users, encryption_settings)
                .await
                .map_err(into_err)?,
        )
        .map_err(into_err)
    }

    /// Encrypt a JSON-encoded content for the given room.
    ///
    /// # Arguments
    ///
    /// * `room_id`, the ID of the room for which the message should be
    ///   encrypted.
    /// * `event_type`, the plaintext type of the event.
    /// * `content`, the JSON-encoded content of the message that should be
    ///   encrypted.
    // #[napi]
    pub async fn encrypt_room_event(
        &self,
        room_id: &identifiers::RoomId,
        event_type: String,
        content: String,
    ) -> Result<String> {
        let room_id = room_id.inner.as_ref();
        let content: JsonValue = serde_json::from_str(content.as_str()).map_err(into_err)?;

        serde_json::to_string(
            &self
                .inner
                .encrypt_room_event_raw(room_id, content, event_type.as_ref())
                .await
                .map_err(into_err)?,
        )
        .map_err(into_err)
    }

    /// Decrypt an event from a room timeline.
    ///
    /// # Arguments
    ///
    /// * `event`, the event that should be decrypted.
    /// * `room_id`, the ID of the room where the event was sent to.
    // #[napi]
    pub async fn decrypt_room_event(
        &self,
        event: String,
        room_id: &identifiers::RoomId,
    ) -> Result<responses::DecryptedRoomEvent> {
        let event: OriginalSyncRoomEncryptedEvent =
            serde_json::from_str(event.as_str()).map_err(into_err)?;
        let room_id = room_id.inner.as_ref();
        let room_event = self.inner.decrypt_room_event(&event, room_id).await.map_err(into_err)?;

        Ok(room_event.into())
    }
}

/// An Ed25519 public key, used to verify digital signatures.
// #[napi]
#[derive(Clone)]
pub struct Ed25519PublicKey {
    inner: vodozemac::Ed25519PublicKey,
}

// #[napi]
impl Ed25519PublicKey {
    /// The number of bytes an Ed25519 public key has.
    // #[napi(getter)]
    pub fn length(&self) -> u32 {
        vodozemac::Ed25519PublicKey::LENGTH as u32
    }

    /// Serialize an Ed25519 public key to an unpadded base64
    /// representation.
    // #[napi]
    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}

/// A Curve25519 public key.
// #[napi]
#[derive(Clone)]
pub struct Curve25519PublicKey {
    inner: vodozemac::Curve25519PublicKey,
}

// #[napi]
impl Curve25519PublicKey {
    /// The number of bytes a Curve25519 public key has.
    // #[napi(getter)]
    pub fn length(&self) -> u32 {
        vodozemac::Curve25519PublicKey::LENGTH as u32
    }

    /// Serialize an Curve25519 public key to an unpadded base64
    /// representation.
    // #[napi]
    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }
}

/// Struct holding the two public identity keys of an account.
// #[napi]
pub struct IdentityKeys {
    ed25519: Ed25519PublicKey,
    curve25519: Curve25519PublicKey,
}

// #[napi]
impl IdentityKeys {
    /// The Ed25519 public key, used for signing.
    // #[napi(getter)]
    pub fn ed25519(&self) -> Ed25519PublicKey {
        self.ed25519.clone()
    }

    /// The Curve25519 public key, used for establish shared secrets.
    // #[napi(getter)]
    pub fn curve25519(&self) -> Curve25519PublicKey {
        self.curve25519.clone()
    }
}

impl From<matrix_sdk_crypto::olm::IdentityKeys> for IdentityKeys {
    fn from(value: matrix_sdk_crypto::olm::IdentityKeys) -> Self {
        Self {
            ed25519: Ed25519PublicKey { inner: value.ed25519 },
            curve25519: Curve25519PublicKey { inner: value.curve25519 },
        }
    }
}
