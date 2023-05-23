import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { EntryHash, Record, ActionHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';


import { clientContext } from '../../contexts';
import { DeviceInviteAcceptance } from './types';

@customElement('device-invite-acceptance-detail')
export class DeviceInviteAcceptanceDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  deviceInviteAcceptanceHash!: ActionHash;

  _fetchRecord = new Task(this, ([deviceInviteAcceptanceHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_device_invite_acceptance',
      payload: deviceInviteAcceptanceHash,
  }) as Promise<Record | undefined>, () => [this.deviceInviteAcceptanceHash]);



  renderDetail(record: Record) {
    const deviceInviteAcceptance = decode((record.entry as any).Present.entry) as DeviceInviteAcceptance;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">Device Invite Acceptance</span>

        </div>

      </div>
    `;
  }
  
  renderDeviceInviteAcceptance(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested device invite acceptance was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderDeviceInviteAcceptance(maybeRecord),
      error: (e: any) => html`<span>Error fetching the device invite acceptance: ${e.data.data}</span>`
    });
  }
}
