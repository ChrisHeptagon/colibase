<script lang="ts">
  import Loginit from "$lib/components/loginit.svelte";
  import Logside from "$lib/components/logside.svelte";
  import type { PageData } from './$types';
	
	export let data: PageData;
</script>

<Logside type="register" />
<Loginit schema={data.schema} type="register" />