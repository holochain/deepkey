
import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, Record, AppAgentClient, EntryHash, ActionHash, AgentPubKey } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-circular-progress';
import { Task } from '@lit-labs/task';

import { clientContext } from '../../contexts';
import './device-invite-acceptance-detail';

@customElement('device-invite-acceptances-for-device-invite')
export class DeviceInviteAcceptancesForDeviceInvite extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal.toString() !== oldVal.toString()
  })
  deviceInviteHash!: ActionHash;

  _fetchDeviceInviteAcceptances = new Task(this, ([deviceInviteHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_device_invite_acceptances_for_device_invite',
      payload: deviceInviteHash,
  }) as Promise<Array<ActionHash>>, () => [this.deviceInviteHash]);

  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No device invite acceptances found for this device invite.</span>`;
    
    return html`
      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash =>
          html`<device-invite-acceptance-detail .deviceInviteAcceptanceHash=${hash}></device-invite-acceptance-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchDeviceInviteAcceptances.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (hashes) => this.renderList(hashes),
      error: (e: any) => html`<span>Error fetching device invite acceptances: ${e.data.data}.</span>`
    });
  }
}
