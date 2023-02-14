
import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, Record, AppAgentClient, EntryHash, ActionHash, AgentPubKey } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-circular-progress';
import { Task } from '@lit-labs/task';

import { clientContext } from '../../contexts';
import './device-invite-detail';

@customElement('device-invites-for-keyset-root')
export class DeviceInvitesForKeysetRoot extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal.toString() !== oldVal.toString()
  })
  keysetRootHash!: ActionHash;

  _fetchDeviceInvites = new Task(this, ([keysetRootHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_device_invites_for_keyset_root',
      payload: keysetRootHash,
  }) as Promise<Array<ActionHash>>, () => [this.keysetRootHash]);

  renderList(hashes: Array<ActionHash>) {
    if (hashes.length === 0) return html`<span>No device invites found for this keyset root.</span>`;
    
    return html`
      <div style="display: flex; flex-direction: column">
        ${hashes.map(hash =>
          html`<device-invite-detail .deviceInviteHash=${hash}></device-invite-detail>`
        )}
      </div>
    `;
  }

  render() {
    return this._fetchDeviceInvites.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (hashes) => this.renderList(hashes),
      error: (e: any) => html`<span>Error fetching device invites: ${e.data.data}.</span>`
    });
  }
}
