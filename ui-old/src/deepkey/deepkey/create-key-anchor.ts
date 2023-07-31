import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { KeyAnchor } from './types';

@customElement('create-key-anchor')
export class CreateKeyAnchor extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  bytes!: Array<number>;


  
  firstUpdated() {
    if (this.bytes === undefined) {
      throw new Error(`The bytes input is required for the create-key-anchor element`);
    }
  }

  isKeyAnchorValid() {
    return true;
  }

  async createKeyAnchor() {
    const keyAnchor: KeyAnchor = { 
        bytes: this.bytes,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_key_anchor',
        payload: keyAnchor,
      });

      this.dispatchEvent(new CustomEvent('key-anchor-created', {
        composed: true,
        bubbles: true,
        detail: {
          keyAnchorHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the key anchor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Key Anchor</span>


        <mwc-button 
          raised
          label="Create Key Anchor"
          .disabled=${!this.isKeyAnchorValid()}
          @click=${() => this.createKeyAnchor()}
        ></mwc-button>
    </div>`;
  }
}
