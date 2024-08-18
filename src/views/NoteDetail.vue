<template>
  <v-container>
    <div>
      <v-btn
        class="text-capitalize"
        color="grey-darken-4"
        size="small"
        @click="$router.push('/notes-page')"
      >
        <i
          class="fas fa-chevron-left"
          style="font-size: 11px; margin-top: 2px"
        ></i>
        <span class="mt-1">Back</span>
      </v-btn>
    </div>
    <div class="mt-5">
      <v-text-field
        label="Title"
        variant="underlined"
        bg-color="grey-darken-2"
        v-model="note.title"
        @input="handleTitleChange"
      >
      </v-text-field>
    </div>
    <div class="mt-5">
      <v-textarea
        variant="solo"
        auto-grow
        clearable
        rows="12"
        bg-color="grey-darken-2"
        v-model="note.description"
        @input="handleDescriptionChange"
      ></v-textarea>
    </div>
  </v-container>
</template>

<script>
import { invoke } from "@tauri-apps/api";

export default {
  name: "NoteDetail",
  data() {
    return {
      note: Object,
    };
  },
  created() {
    this.note = JSON.parse(this.$route.query.data);
  },
  methods: {
    handleTitleChange: async function () {
      try {
        let res = await invoke("update_title", {
          title: this.note.title,
          noteId: this.note.id,
        });
      } catch (_) {}
    },
    handleDescriptionChange: async function () {
      try {
        let res = await invoke("update_description", {
          description: this.note.description,
          noteId: this.note.id,
        });
      } catch (_) {}
    },
  },
};
</script>
