<section id="entry_layout">
  <slot />
</section>

<style lang="scss">
  @font-face {
    font-family: "rem";
    src: url("../../lib/fonts/rem.woff2") format("woff2");
  }
  section#entry_layout {
	height: 100%;
	width: 100%;
  font-family: "rem", sans-serif;
  display: grid;
  grid-template-areas: "sidebar input";
  grid-template-columns: 250px 1fr;
  grid-template-rows: 1fr;
  font-family: "rem";
  }
</style>
