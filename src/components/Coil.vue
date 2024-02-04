<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const state = ref(false);
const props = defineProps(['id']);


// Yes very pretty, works for now 10/10 error propagation.
async function set_coil(val: boolean) {
    if (val === state.value) { return; }

    try {
        let result = await invoke("update_coil", {
        switchId: parseInt(props.id),
        state: val
    });
    if (typeof result === 'boolean') { state.value = result; }


    } catch (e) {
        console.error(e);
    }

}
</script>

<template>
    <div class="coil-container">
        <div class="coil-info">
            <p>Coil: {{ props.id }}</p>
            <p class="coil-state">State: <span class="coil-indicator" :class="{ coil_active: state }"></span></p>
            <div class="coil-switch">
                <button @click="set_coil(!state)">Toggle</button>
            </div>
        </div>
    </div>
</template>
