<script lang="ts">
	import KeyAltIcon from '~icons/iconoir/key-alt-remove';
</script>

<aside class="alert variant-ghost m-5 bg-gradient-to-br variant-gradient-secondary-primary">
	<!-- Icon -->
	<KeyAltIcon class="h-12 w-12" />
	<!-- Message -->
	<div class="alert-message">
		<h3 class="h3">Key Revocation Request</h3>
		<p>You have received a request to revoke key 0x</p>
	</div>
	<!-- Actions -->
	<div class="alert-actions">
		<button type="button" class="btn variant-ghost-error">Sign Revocation</button>
		<button type="button" class="btn variant-ghost-surface">Delete Request</button>
	</div>
</aside>
