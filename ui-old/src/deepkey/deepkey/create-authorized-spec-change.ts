import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { AuthorizedSpecChange } from './types';

@customElement('create-authorized-spec-change')
export class CreateAuthorizedSpecChange extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  newSpec!: ActionHash;

  @property()
  authorizationOfNewSpec!: Array<number>;



  isAuthorizedSpecChangeValid() {
    return true;
  }

  async createAuthorizedSpecChange() {
    const authorizedSpecChange: AuthorizedSpecChange = { 
        new_spec: this.newSpec,
        authorization_of_new_spec: this.authorizationOfNewSpec,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_authorized_spec_change',
        payload: authorizedSpecChange,
      });

      this.dispatchEvent(new CustomEvent('authorized-spec-change-created', {
        composed: true,
        bubbles: true,
        detail: {
          authorizedSpecChangeHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the authorized spec change: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Authorized Spec Change</span>


        <mwc-button 
          raised
          label="Create Authorized Spec Change"
          .disabled=${!this.isAuthorizedSpecChangeValid()}
          @click=${() => this.createAuthorizedSpecChange()}
        ></mwc-button>
    </div>`;
  }
}
