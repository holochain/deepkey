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
import { DeviceInvite } from './types';

@customElement('device-invite-detail')
export class DeviceInviteDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  deviceInviteHash!: ActionHash;

  _fetchRecord = new Task(this, ([deviceInviteHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_device_invite',
      payload: deviceInviteHash,
  }) as Promise<Record | undefined>, () => [this.deviceInviteHash]);



  renderDetail(record: Record) {
    const deviceInvite = decode((record.entry as any).Present.entry) as DeviceInvite;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">Device Invite</span>

        </div>

      </div>
    `;
  }
  
  renderDeviceInvite(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested device invite was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderDeviceInvite(maybeRecord),
      error: (e: any) => html`<span>Error fetching the device invite: ${e.data.data}</span>`
    });
  }
}
