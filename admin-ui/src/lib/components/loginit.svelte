<script lang="ts">
    export let type: "login" | "register" = "login";
    let title = type === "login" ? "Login" : "Register";
    export let schema: any = {};
    let formKeys = Object.keys(schema).sort((a, b) =>
      schema[a].order > schema[b].order ? 1 : -1
    );
    import cbaselogo from "$lib/img/cbaselogo.svg";
</script>

<section class="entry_form">
  <h1>
    {#if type === "login"}
    Welcome Back! <br> Login to access Colibase.
    {/if}
    {#if type === "register"}
    New here? <br> Welcome to Colibase! <br> Create an account to get started.
    {/if}
    </h1>

  <img alt="Colibase Logo" src={cbaselogo} />

      <h1>
        {#if type === "login"}
          Login
        {/if}
        {#if type === "register"}
          Initalize Account
        {/if}
      </h1>
      <form method="POST" novalidate action={`/api/${type}`} enctype="multipart/form-data">
        {#each formKeys as key}
          <label for={key}>
            {key}
          </label>
          <input type={schema[key].form_type} 
          id={key}
          name={key}
          pattern={schema[key].pattern}NNNN
          required={schema[key].required}
          />
        {/each}
        <button type="submit">
          {title}
        </button>
      </form>
    </section>

<style lang="scss">
  section.entry_form {
    grid-area: input;
    box-shadow: #ccc 0px 0px 10px 0px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: var(--bg-color);
    color: var(--text-color);
    transition: all 0.5s;
    min-width: 500px;
    padding: 40px;
    img {
      grid-area: logo;
      width: 150px;
      filter: drop-shadow(-4px 6px 4px #000);
    }
    h1 {
      text-align: center;
      font-size: 2rem;
    }
    form {
      label {
        font-size: 1.5rem;
        margin-bottom: 5px;
        display: block;
      }
      input {
        font-size: 1.5rem;
        margin: 10px 0;
        padding: 10px;
        border-radius: 10px;
        border: 1px solid #ccc;
      }
      button {
        font-family: "rem";
        display: block;
        width: 100%;
        font-size: 1.25rem;
        padding: 0.5rem;
        margin-bottom: 1rem;
        border: 1px solid #ccc;
        border-radius: 10px;
        background-image: linear-gradient(45deg, hsl(93, 88%, 55%) 0%, hsl(93, 88%, 45%) 100%);
        color: #0b0d41;
        cursor: pointer;
      }
    }
  }
  
</style>                              