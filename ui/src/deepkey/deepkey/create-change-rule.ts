import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { ChangeRule } from './types';

@customElement('create-change-rule')
export class CreateChangeRule extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  keysetRoot!: ActionHash;

  @property()
  keysetLeaf!: ActionHash;

  @property()
  specChange!: ActionHash;



  isChangeRuleValid() {
    return true;
  }

  async createChangeRule() {
    const changeRule: ChangeRule = { 
        keyset_root: this.keysetRoot,
        keyset_leaf: this.keysetLeaf,
        spec_change: this.specChange,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'deepkey',
        zome_name: 'deepkey',
        fn_name: 'create_change_rule',
        payload: changeRule,
      });

      this.dispatchEvent(new CustomEvent('change-rule-created', {
        composed: true,
        bubbles: true,
        detail: {
          changeRuleHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the change rule: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Change Rule</span>


        <mwc-button 
          raised
          label="Create Change Rule"
          .disabled=${!this.isChangeRuleValid()}
          @click=${() => this.createChangeRule()}
        ></mwc-button>
    </div>`;
  }
}
