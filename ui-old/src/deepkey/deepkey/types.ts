import { 
  Record, 
  ActionHash, 
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type DeepkeySignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'KeyAnchor'; } & KeyAnchor)
 | ({ type: 'KeyRegistration'; } & KeyRegistration)
 | ({ type: 'KeyRevocation'; } & KeyRevocation)
 | ({ type: 'KeyGeneration'; } & KeyGeneration)
 | ({ type: 'DeviceInviteAcceptance'; } & DeviceInviteAcceptance)
 | ({ type: 'DeviceInvite'; } & DeviceInvite)
 | ({ type: 'ChangeRule'; } & ChangeRule)
 | ({ type: 'AuthorizedSpecChange'; } & AuthorizedSpecChange)
 | ({ type: 'AuthoritySpec'; } & AuthoritySpec)
 | ({  type: 'KeysetRoot'; } & KeysetRoot);


export interface KeysetRoot { 
  first_deepkey_agent: AgentPubKey;

  root_pub_key: AgentPubKey;

  fda_pubkey_signed_by_root_key: string;
}




export interface AuthoritySpec { 
  sigs_required: number;

  signers: Array<AgentPubKey>;
}




export interface AuthorizedSpecChange { 
  new_spec: ActionHash;

  authorization_of_new_spec: Array<number>;
}




export interface ChangeRule { 
  keyset_root: ActionHash;

  keyset_leaf: ActionHash;

  spec_change: ActionHash;
}




export interface DeviceInvite { 
  keyset_root: ActionHash;

  parent: ActionHash;

  invitee: AgentPubKey;
}




export interface DeviceInviteAcceptance { 
  keyset_root_authority: ActionHash;

  invite: ActionHash;
}





// export interface KeyGeneration { 
//   new_key: AgentPubKey;

//   new_key_signing_of_author: ActionHash;
// }





// export interface KeyRevocation { 
//   prior_key_registration: ActionHash;

//   revocation_authorization: Array<ActionHash>;
// }




// export interface Create {
//   type:  
//     | 'Update'
//         | 'Delete'
//     ;
// }

// export interface KeyRegistration { 
//   key_registration: Create;
// }





// export interface KeyAnchor { 
//   bytes: Array<number>;
// }


