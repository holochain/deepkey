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
import { AuthoritySpec } from './types';

@customElement('authority-spec-detail')
export class AuthoritySpecDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  authoritySpecHash!: ActionHash;

  _fetchRecord = new Task(this, ([authoritySpecHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_authority_spec',
      payload: authoritySpecHash,
  }) as Promise<Record | undefined>, () => [this.authoritySpecHash]);



  renderDetail(record: Record) {
    const authoritySpec = decode((record.entry as any).Present.entry) as AuthoritySpec;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">Authority Spec</span>

        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span><strong>Sigs Required</strong></span>
 	  <span style="white-space: pre-line">${ authoritySpec.sigs_required }</span>
        </div>

      </div>
    `;
  }
  
  renderAuthoritySpec(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested authority spec was not found.</span>`;
    
    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderAuthoritySpec(maybeRecord),
      error: (e: any) => html`<span>Error fetching the authority spec: ${e.data.data}</span>`
    });
  }
}
