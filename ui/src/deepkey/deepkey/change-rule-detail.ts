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

import './edit-change-rule';

import { clientContext } from '../../contexts';
import { ChangeRule } from './types';

@customElement('change-rule-detail')
export class ChangeRuleDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  changeRuleHash!: ActionHash;

  _fetchRecord = new Task(this, ([changeRuleHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'deepkey',
      zome_name: 'deepkey',
      fn_name: 'get_change_rule',
      payload: changeRuleHash,
  }) as Promise<Record | undefined>, () => [this.changeRuleHash]);

  @state()
  _editing = false;


  renderDetail(record: Record) {
    const changeRule = decode((record.entry as any).Present.entry) as ChangeRule;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
          <span style="font-size: 18px; flex: 1;">Change Rule</span>

          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderChangeRule(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested change rule was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-change-rule
    	  .originalChangeRuleHash=${this.changeRuleHash}
    	  .currentRecord=${maybeRecord}
    	  @change-rule-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-change-rule>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderChangeRule(maybeRecord),
      error: (e: any) => html`<span>Error fetching the change rule: ${e.data.data}</span>`
    });
  }
}
