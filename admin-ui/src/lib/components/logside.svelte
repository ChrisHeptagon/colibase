<script lang="ts">
  export let type: "login" | "register" = "login";



</script>

<aside>

</aside>

<style lang="scss">
  aside {
    grid-area: sidebar;
    background-image: linear-gradient(90deg, hsl(20, 88%, 60%) 0%, hsl(20, 88%, 50%) 100%);
    border-radius: 0 20px 20px 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-left: 20px;
    padding-right: 20px;
    padding-top: 50px;
  }
</style>
