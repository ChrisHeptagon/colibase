<script lang="ts">
    export let type: "login" | "register" = "login";
    let title = type === "login" ? "Login" : "Register";
    export let schema: any = {};
    let formKeys = Object.keys(schema).sort((a, b) =>
      schema[a].order > schema[b].order ? 1 : -1
    );
</script>

<section class="entry_form">
      <h1>
        {title}
      </h1>
      <form method="POST" novalidate action={`/api/${type}`} enctype="multipart/form-data">
        {#each formKeys as key}
          <label for={key}>
            {key}
          </label>
          <input type={schema[key].form_type} 
          id={key}
          name={key}
          pattern={schema[key].pattern}
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
    box-shadow: #ccc 0px 0px 10px 0px;
    width: 500px;
    height: 80%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: #fff;
    padding: 2rem;
    border-radius: 1rem;
    text-shadow: #ccc 0px 0px 10px 0px;
    h1 {
      font-size: 2rem;
      margin-bottom: 1rem;
    }
    form {
      display: flex;
      flex-direction: column;
      justify-content: center;
      width: 100%;
      label {
        font-size: 1rem;
        margin-bottom: 0.5rem;
      }
      input {
        font-size: 1rem;
        padding: 0.5rem;
        margin-bottom: 1rem;
        border: 1px solid #ccc;
        border-radius: 0.5rem;
      }
      button {
        font-size: 1.25rem;
        font-family: inherit;
        padding: 0.5rem;
        margin-bottom: 1rem;
        border: 1px solid #ccc;
        border-radius: 0.5rem;
        background-color: #09b372;
        color: #fff;
        cursor: pointer;
      }
    }
  }
  
</style>