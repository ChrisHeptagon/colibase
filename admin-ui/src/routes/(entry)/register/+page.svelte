<script lang="ts">
  import Loginit from "$lib/components/loginit.svelte";
  import type { PageData } from './$types';
	
	export let data: PageData;
</script>

<Loginit schema={data.schema} type="register" />