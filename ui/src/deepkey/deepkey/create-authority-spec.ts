import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-slider';

import { clientContext } from '../../contexts';
import { AuthoritySpec } from './types';

@customElement('create-authority-spec')
export class CreateAuthoritySpec extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  signers!: Array<AgentPubKey>;


  @state()
  _sigsRequired: number | undefined;


  isAuthoritySpecValid() {
    return true && this._sigsRequired !== undefined;
  }

  async createAuthoritySpec() {
    const authoritySpec: AuthoritySpec = { 
        sigs_required: this._sigsRequired!,
        signers: this.signers,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_authority_spec',
        payload: authoritySpec,
      });

      this.dispatchEvent(new CustomEvent('authority-spec-created', {
        composed: true,
        bubbles: true,
        detail: {
          authoritySpecHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the authority spec: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Authority Spec</span>

          <div style="margin-bottom: 16px">
            <div style="display: flex; flex-direction: row">
              <span style="margin-right: 4px">Sigs Required</span>
            
              <mwc-slider  @input=${(e: CustomEvent) => { this._sigsRequired = e.detail.value; } } discrete></mwc-slider>
            </div>          
          </div>
            

        <mwc-button 
          raised
          label="Create Authority Spec"
          .disabled=${!this.isAuthoritySpecValid()}
          @click=${() => this.createAuthoritySpec()}
        ></mwc-button>
    </div>`;
  }
}
