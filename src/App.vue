<template>
  <div id="app">
    <v-container>
      <router-view></router-view>

      <v-dialog v-model="showErrDialog" width="300" height="120" persistent>
        <v-card
          width="300"
          height="120"
          class="px-4 py-5"
          style="background-color: #212121; color: white"
        >
          <h3 class="text-center">Failed to initalize note file</h3>
          <div class="text-right mt-4">
            <v-btn
              class="text-capitalize pt-1"
              size="small"
              @click="invokeExitAppCommand()"
            >
              Exit Now
            </v-btn>
          </div>
        </v-card>
      </v-dialog>
    </v-container>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api";

export default {
  name: "App",
  data() {
    return {
      showErrDialog: false,
    };
  },
  async created() {
    await this.invokeInitializeNoteFileCommand();
  },
  methods: {
    invokeInitializeNoteFileCommand: async function () {
      try {
        let res = await invoke("initialize_note_file");
        if (res === "success") {
          this.$router.push("/notes-page");
        }
      } catch (_) {
        this.showErrDialog = true;
      }
    },
    invokeExitAppCommand: async function () {
      await invoke("exit_app");
    },
  },
};
</script>
