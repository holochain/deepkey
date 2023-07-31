import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { EntryHash, Record, ActionHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import './edit-key-anchor';

import { clientContext } from '../../contexts';
import { KeyAnchor } from './types';

@customElement('key-anchor-detail')
export class KeyAnchorDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  keyAnchorHash!: ActionHash;

  _fetchRecord = new Task(this, ([keyAnchorHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_key_anchor',
      payload: keyAnchorHash,
  }) as Promise<Record | undefined>, () => [this.keyAnchorHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.keyAnchorHash === undefined) {
      throw new Error(`The keyAnchorHash property is required for the key-anchor-detail element`);
    }
  }

  async deleteKeyAnchor() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'delete_key_anchor',
        payload: this.keyAnchorHash,
      });
      this.dispatchEvent(new CustomEvent('key-anchor-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          keyAnchorHash: this.keyAnchorHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the key anchor: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const keyAnchor = decode((record.entry as any).Present.entry) as KeyAnchor;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteKeyAnchor()}></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderKeyAnchor(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested key anchor was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-key-anchor
    	  .currentRecord=${maybeRecord}
    	  @key-anchor-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-key-anchor>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderKeyAnchor(maybeRecord),
      error: (e: any) => html`<span>Error fetching the key anchor: ${e.data.data}</span>`
    });
  }
}
