import { AgentPubKey, Signature, ActionHash } from '@holochain/client';

type KeyGeneration = {
  new_key: AgentPubKey;
  new_key_signing_of_author: Signature;
};

type KeyRegistration = {
  Create: KeyGeneration;
};
// | { Delete: KeyRevocation };

type KeyRevocation = {
  prior_key_registration: ActionHash;
  revoction_authorization: [];
};

// pub enum KeyState {
//   NotFound,
//   Invalidated,
//   Valid,
// }
