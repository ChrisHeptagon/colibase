<section id="entry_layout">
  <slot />
</section>

<style lang="scss">
  @font-face {
    font-family: "Urbanist";
    src: url("../../lib/fonts/Gabarito.woff2") format("woff2");
  }
  section#entry_layout {
	height: 100%;
	width: 100%;
    display: flex;
    flex-direction: column;
    align-items: end;
    justify-content: center;
	background-image: linear-gradient(to right, #ffffff 0%, #bbb9b1 100%);
  font-family: "Urbanist", sans-serif;
  display: grid;
  grid-template-areas: "sidebar input";
  grid-template-columns: 500px 1fr;
  grid-template-rows: 1fr;
  font-family: "rem";
  }
</style>
