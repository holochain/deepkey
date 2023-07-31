import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { KeysetRoot } from './types';

@customElement('create-keyset-root')
export class CreateKeysetRoot extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  firstDeepkeyAgent!: AgentPubKey;

  @property()
  rootPubKey!: AgentPubKey;

  @property()
  fdaPubkeySignedByRootKey!: string;



  isKeysetRootValid() {
    return true;
  }

  async createKeysetRoot() {
    const keysetRoot: KeysetRoot = { 
        first_deepkey_agent: this.firstDeepkeyAgent,
        root_pub_key: this.rootPubKey,
        fda_pubkey_signed_by_root_key: this.fdaPubkeySignedByRootKey,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_keyset_root',
        payload: keysetRoot,
      });

      this.dispatchEvent(new CustomEvent('keyset-root-created', {
        composed: true,
        bubbles: true,
        detail: {
          keysetRootHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the keyset root: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Keyset Root</span>


        <mwc-button 
          raised
          label="Create Keyset Root"
          .disabled=${!this.isKeysetRootValid()}
          @click=${() => this.createKeysetRoot()}
        ></mwc-button>
    </div>`;
  }
}
