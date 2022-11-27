import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, EntryHash, AgentPubKey, Record, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { contextProvided } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@material/mwc-textarea';
import '@material/mwc-slider';

import { appWebsocketContext, appInfoContext } from '../../contexts';
import { FirstEntry } from './types';

@customElement('edit-first-entry')
export class EditFirstEntry extends LitElement {

  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalFirstEntryHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentFirstEntry() {
    return decode((this.currentRecord.entry as any).Present.entry) as FirstEntry;
  }
 
  @state()
  _name!: string;

  @state()
  _description!: string;

  @state()
  _age!: number;


  isFirstEntryValid() {
    return true && this._name !== undefined && this._description !== undefined && this._age !== undefined;
  }

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfoContext })
  appInfo!: InstalledAppInfo;
  
  connectedCallback() {
    super.connectedCallback();
    this._name = this.currentFirstEntry.name;
    this._description = this.currentFirstEntry.description;
    this._age = this.currentFirstEntry.age;
  }

  async updateFirstEntry() {
    const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'first')!;

    const firstEntry: FirstEntry = { 
      name: this._name!,
      description: this._description!,
      age: this._age!,
    };

    try {
      const updateRecord: Record = await this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: cellData.cell_id,
        zome_name: 'first_zome',
        fn_name: 'update_first_entry',
        payload: {
          original_first_entry_hash: this.originalFirstEntryHash,
          previous_first_entry_hash: this.currentRecord.signed_action.hashed.hash,
          updated_first_entry: firstEntry
        },
        provenance: cellData.cell_id[1]
      });
  
      this.dispatchEvent(new CustomEvent('first-entry-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalFirstEntryHash: this.originalFirstEntryHash,
          previousFirstEntryHash: this.currentRecord.signed_action.hashed.hash,
          updatedFirstEntryHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the first entry: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit First Entry</span>
          <div style="margin-bottom: 16px">
          <mwc-textarea outlined label="Name" .value=${ this._name } @input=${(e: CustomEvent) => { this._name = (e.target as any).value;} } required></mwc-textarea>    
          </div>

          <div style="margin-bottom: 16px">
          <mwc-textarea outlined label="Description" .value=${ this._description } @input=${(e: CustomEvent) => { this._description = (e.target as any).value;} } required></mwc-textarea>    
          </div>

          <div style="margin-bottom: 16px">
          <div style="display: flex; flex-direction: row">
            <span style="margin-right: 4px">Age</span>
          
            <mwc-slider .value=${ (this._age) } @input=${(e: CustomEvent) => { this._age = e.detail.value; } }></mwc-slider>
          </div>    
          </div>



        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            @click=${() => this.dispatchEvent(new CustomEvent('edit-canceled', {
              bubbles: true,
              composed: true
            }))}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>
          <mwc-button 
            raised
            label="Save"
            .disabled=${!this.isFirstEntryValid()}
            @click=${() => this.updateFirstEntry()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
