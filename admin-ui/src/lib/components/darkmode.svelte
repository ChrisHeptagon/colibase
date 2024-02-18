<script lang="ts">
    import MOON from "$lib/img/moon.svg"
    import SUN from "$lib/img/sun.svg"
  import { onMount } from "svelte";
    let isChecked = false;
    let mode: "light" | "dark" = "light";
    $: mode = isChecked ? "dark" : "light";

    function changeDM() {
        localStorage.setItem("dm", mode);
        applyTheme();
    }

    function applyTheme() {
            document.documentElement.classList.toggle("dm-dark", isChecked);
    }

    onMount(() => {
        const dm = localStorage.getItem("dm");
        if (dm) {
            isChecked = dm === "dark";
        } else {
            mode = "light"
        }
    });

</script>

<label for="toggle" class={mode}>
    <input type="checkbox" id="toggle" bind:checked={isChecked} on:change={changeDM} />
    <span class={isChecked ? "def" : "dif"}>
        <img src="{mode === 'light' ? MOON : SUN}" alt="{mode === 'light' ? 'moon' : 'sun'}" />
    </span>
</label>

<style lang="scss">
    label {
        width: 60px;
        height: 30px;
        border-radius: 25px;
        transition: all 0.3s;
        &.light {
            background-color: #fff;
            span {
                background-color: #000;
                img {
                    filter: invert(1);
                }
            }
        }
        &.dark {
            background-color: #000;
            span {
                background-color: #fff;
            }
        }
    }
    input {
        display: none;
    }
    span {
        display: block;
        width: 30px;
        height: 30px;
        border-radius: 50%;
        transition: 0.3s;
        cursor: pointer;

        img {
            width: 20px;
            height: 20px;
            position: absolute;
            top: 5px;
            left: 5px;
            user-select: none;
            pointer-events: none;
        }
    }
    .def {
        transform: translateX(30px);
    }
    .dif {
        transform: translateX(0px);
    }

</style>