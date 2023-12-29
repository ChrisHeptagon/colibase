<script lang="ts">
    export let type: "login" | "register" = "login";
    let title = type === "login" ? "Login" : "Register";

    
</script>

<section class="entry_form">
      <h1>
        {title}
      </h1>
      <form method="POST">
        
      </form>
    </section>