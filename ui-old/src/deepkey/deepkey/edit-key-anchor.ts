import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { KeyAnchor } from './types';

@customElement('edit-key-anchor')
export class EditKeyAnchor extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  
  @property()
  currentRecord!: Record;
 
  get currentKeyAnchor() {
    return decode((this.currentRecord.entry as any).Present.entry) as KeyAnchor;
  }
 

  isKeyAnchorValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-key-anchor element`);
    }
    
  }

  async updateKeyAnchor() {
    const keyAnchor: KeyAnchor = { 
      bytes: this.currentKeyAnchor.bytes,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'update_key_anchor',
        payload: {
          previous_key_anchor_hash: this.currentRecord.signed_action.hashed.hash,
          updated_key_anchor: keyAnchor
        },
      });
  
      this.dispatchEvent(new CustomEvent('key-anchor-updated', {
        composed: true,
        bubbles: true,
        detail: {
          previousKeyAnchorHash: this.currentRecord.signed_action.hashed.hash,
          updatedKeyAnchorHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the key anchor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Key Anchor</span>


        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            @click=${() => this.dispatchEvent(new CustomEvent('edit-canceled', {
              bubbles: true,
              composed: true
            }))}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>
          <mwc-button 
            raised
            label="Save"
            .disabled=${!this.isKeyAnchorValid()}
            @click=${() => this.updateKeyAnchor()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
