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
import { JoiningProof } from './types';

@customElement('joining-proof-detail')
export class JoiningProofDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  joiningProofHash!: ActionHash;

  _fetchRecord = new Task(this, ([joiningProofHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_joining_proof',
      payload: joiningProofHash,
  }) as Promise<Record | undefined>, () => [this.joiningProofHash]);



  renderDetail(record: Record) {
    const joiningProof = decode((record.entry as any).Present.entry) as JoiningProof;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
        </div>

      </div>
    `;
  }
  
  renderJoiningProof(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested joining proof was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderJoiningProof(maybeRecord),
      error: (e: any) => html`<span>Error fetching the joining proof: ${e.data.data}</span>`
    });
  }
}
