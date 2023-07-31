import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { ChangeRule } from './types';

@customElement('edit-change-rule')
export class EditChangeRule extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalChangeRuleHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentChangeRule() {
    return decode((this.currentRecord.entry as any).Present.entry) as ChangeRule;
  }
 

  isChangeRuleValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
  }

  async updateChangeRule() {
    const changeRule: ChangeRule = { 
      keyset_root: this.currentChangeRule.keyset_root,
      keyset_leaf: this.currentChangeRule.keyset_leaf,
      spec_change: this.currentChangeRule.spec_change,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'update_change_rule',
        payload: {
          original_change_rule_hash: this.originalChangeRuleHash,
          previous_change_rule_hash: this.currentRecord.signed_action.hashed.hash,
          updated_change_rule: changeRule
        },
      });
  
      this.dispatchEvent(new CustomEvent('change-rule-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalChangeRuleHash: this.originalChangeRuleHash,
          previousChangeRuleHash: this.currentRecord.signed_action.hashed.hash,
          updatedChangeRuleHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the change rule: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Change Rule</span>


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
            .disabled=${!this.isChangeRuleValid()}
            @click=${() => this.updateChangeRule()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
