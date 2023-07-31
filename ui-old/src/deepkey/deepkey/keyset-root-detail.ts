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
import { KeysetRoot } from './types';

@customElement('keyset-root-detail')
export class KeysetRootDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  keysetRootHash!: ActionHash;

  _fetchRecord = new Task(this, ([keysetRootHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_keyset_root',
      payload: keysetRootHash,
  }) as Promise<Record | undefined>, () => [this.keysetRootHash]);



  renderDetail(record: Record) {
    const keysetRoot = decode((record.entry as any).Present.entry) as KeysetRoot;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">Keyset Root</span>

        </div>

      </div>
    `;
  }
  
  renderKeysetRoot(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested keyset root was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderKeysetRoot(maybeRecord),
      error: (e: any) => html`<span>Error fetching the keyset root: ${e.data.data}</span>`
    });
  }
}
