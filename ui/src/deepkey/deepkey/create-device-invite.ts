import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { DeviceInvite } from './types';

@customElement('create-device-invite')
export class CreateDeviceInvite extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  keysetRoot!: ActionHash;

  @property()
  parent!: ActionHash;

  @property()
  invitee!: AgentPubKey;



  isDeviceInviteValid() {
    return true;
  }

  async createDeviceInvite() {
    const deviceInvite: DeviceInvite = { 
        keyset_root: this.keysetRoot,
        parent: this.parent,
        invitee: this.invitee,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_device_invite',
        payload: deviceInvite,
      });

      this.dispatchEvent(new CustomEvent('device-invite-created', {
        composed: true,
        bubbles: true,
        detail: {
          deviceInviteHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the device invite: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Device Invite</span>


        <mwc-button 
          raised
          label="Create Device Invite"
          .disabled=${!this.isDeviceInviteValid()}
          @click=${() => this.createDeviceInvite()}
        ></mwc-button>
    </div>`;
  }
}
