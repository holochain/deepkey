import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { DeviceInviteAcceptance } from './types';

@customElement('create-device-invite-acceptance')
export class CreateDeviceInviteAcceptance extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  keysetRootAuthority!: ActionHash;

  @property()
  invite!: ActionHash;



  isDeviceInviteAcceptanceValid() {
    return true;
  }

  async createDeviceInviteAcceptance() {
    const deviceInviteAcceptance: DeviceInviteAcceptance = { 
        keyset_root_authority: this.keysetRootAuthority,
        invite: this.invite,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_device_invite_acceptance',
        payload: deviceInviteAcceptance,
      });

      this.dispatchEvent(new CustomEvent('device-invite-acceptance-created', {
        composed: true,
        bubbles: true,
        detail: {
          deviceInviteAcceptanceHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the device invite acceptance: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Device Invite Acceptance</span>


        <mwc-button 
          raised
          label="Create Device Invite Acceptance"
          .disabled=${!this.isDeviceInviteAcceptanceValid()}
          @click=${() => this.createDeviceInviteAcceptance()}
        ></mwc-button>
    </div>`;
  }
}
