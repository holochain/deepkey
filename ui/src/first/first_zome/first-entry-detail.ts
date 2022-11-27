import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, AppWebsocket, EntryHash, Record, ActionHash, InstalledAppInfo } from '@holochain/client';
import { contextProvided } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';



import './edit-first-entry';

import { appInfoContext, appWebsocketContext } from '../../contexts';
import { FirstEntry } from './types';

@customElement('first-entry-detail')
export class FirstEntryDetail extends LitElement {
  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  firstEntryHash!: ActionHash;

  _fetchRecord = new Task(this, ([firstEntryHash]) => this.appWebsocket.callZome({
      cap_secret: null,
      cell_id: this.cellData.cell_id,
      zome_name: 'first_zome',
      fn_name: 'get_first_entry',
      payload: firstEntryHash,
      provenance: this.cellData.cell_id[1]
  }) as Promise<Record | undefined>, () => [this.firstEntryHash]);

  @state()
  _editing = false;

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfoContext })
  appInfo!: InstalledAppInfo;

  get cellData() {
    return this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'first')!;
  }

  async deleteFirstEntry() {
    try {
      await this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: this.cellData.cell_id,
        zome_name: 'first_zome',
        fn_name: 'delete_first_entry',
        payload: this.firstEntryHash,
        provenance: this.cellData.cell_id[1]
      });
      this.dispatchEvent(new CustomEvent('first-entry-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          firstEntryHash: this.firstEntryHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the first entry: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const firstEntry = decode((record.entry as any).Present.entry) as FirstEntry;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">First Entry</span>

          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteFirstEntry()}></mwc-icon-button>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span><strong>Name</strong></span>
 	  <span style="white-space: pre-line">${ firstEntry.name }</span>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span><strong>Description</strong></span>
 	  <span style="white-space: pre-line">${ firstEntry.description }</span>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span><strong>Age</strong></span>
 	  <span style="white-space: pre-line">${ firstEntry.age }</span>
        </div>

      </div>
    `;
  }
  
  renderFirstEntry(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested first entry was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-first-entry
    	  .originalFirstEntryHash=${this.firstEntryHash}
    	  .currentRecord=${maybeRecord}
    	  @first-entry-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-first-entry>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderFirstEntry(maybeRecord),
      error: (e: any) => html`<span>Error fetching the first entry: ${e.data.data}</span>`
    });
  }
}
