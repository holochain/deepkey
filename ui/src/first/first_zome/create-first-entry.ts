import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { contextProvided } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@material/mwc-textarea';
import '@material/mwc-slider';

import { appWebsocketContext, appInfoContext } from '../../contexts';
import { FirstEntry } from './types';

@customElement('create-first-entry')
export class CreateFirstEntry extends LitElement {

  @state()
  _name: string | undefined;

  @state()
  _description: string | undefined;

  @state()
  _age: number | undefined;


  isFirstEntryValid() {
    return true && this._name !== undefined && this._description !== undefined && this._age !== undefined;
  }

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfoContext })
  appInfo!: InstalledAppInfo;

  async createFirstEntry() {
    const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'first')!;

    const firstEntry: FirstEntry = { 
        name: this._name!,
        description: this._description!,
        age: this._age!,
    };

    try {
      const record: Record = await this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: cellData.cell_id,
        zome_name: 'first_zome',
        fn_name: 'create_first_entry',
        payload: firstEntry,
        provenance: cellData.cell_id[1]
      });

      this.dispatchEvent(new CustomEvent('first-entry-created', {
        composed: true,
        bubbles: true,
        detail: {
          firstEntryHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the first entry: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create First Entry</span>

          <div style="margin-bottom: 16px">
            <mwc-textarea outlined label="Name"  @input=${(e: CustomEvent) => { this._name = (e.target as any).value;} } required></mwc-textarea>          
          </div>
            
          <div style="margin-bottom: 16px">
            <mwc-textarea outlined label="Description"  @input=${(e: CustomEvent) => { this._description = (e.target as any).value;} } required></mwc-textarea>          
          </div>
            
          <div style="margin-bottom: 16px">
            <div style="display: flex; flex-direction: row">
              <span style="margin-right: 4px">Age</span>
            
              <mwc-slider  @input=${(e: CustomEvent) => { this._age = e.detail.value; } }></mwc-slider>
            </div>          
          </div>
            

        <mwc-button 
          raised
          label="Create First Entry"
          .disabled=${!this.isFirstEntryValid()}
          @click=${() => this.createFirstEntry()}
        ></mwc-button>
    </div>`;
  }
}
