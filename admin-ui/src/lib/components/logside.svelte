<script lang="ts">
  import Darkmode from "./darkmode.svelte";

</script>

<aside>
<Darkmode />
</aside>

<style lang="scss">
  aside {
    grid-area: sidebar;
    background-image: linear-gradient(
      90deg,
      hsl(20, 88%, 60%) 0%,
      hsl(20, 88%, 50%) 100%
    );
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: end;
    padding-left: 20px;
    padding-right: 20px;
    padding-top: 50px;
    padding-bottom: 50px;
    
  }
</style>
