import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { JoiningProof } from './types';

@customElement('create-joining-proof')
export class CreateJoiningProof extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  keysetProof!: string;
  @property()
  membraneProof!: string;


  isJoiningProofValid() {
    return true;
  }

  async createJoiningProof() {
    const joiningProof: JoiningProof = { 
        keyset_proof: this.keysetProof,
        membrane_proof: this.membraneProof,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_joining_proof',
        payload: joiningProof,
      });

      this.dispatchEvent(new CustomEvent('joining-proof-created', {
        composed: true,
        bubbles: true,
        detail: {
          joiningProofHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the joining proof: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Joining Proof</span>


        <mwc-button 
          raised
          label="Create Joining Proof"
          .disabled=${!this.isJoiningProofValid()}
          @click=${() => this.createJoiningProof()}
        ></mwc-button>
    </div>`;
  }
}
