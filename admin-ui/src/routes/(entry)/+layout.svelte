<section id="entry_layout">
  <slot />
</section>

<style lang="scss">
  @font-face {
    font-family: "Urbanist";
    src: url("../../lib/fonts/Urbanist-Regular.woff2")
  }
  section#entry_layout {
	height: 100%;
	width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
	background-image: linear-gradient(to right, #ffffff 0%, #bbb9b1 100%);
  font-family: "Urbanist", sans-serif;
  }
</style>
