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
        :error-messages="titleErr"
        v-model="title"
      >
      </v-text-field>
    </div>
    <div class="mt-5">
      <v-textarea
        label="Description"
        variant="solo"
        auto-grow
        clearable
        rows="12"
        bg-color="grey-darken-2"
        v-model="description"
        :error-messages="descriptionErr"
      ></v-textarea>
    </div>
    <div class="text-right mt-2">
      <v-btn
        class="text-capitalize"
        color="grey-darken-4"
        size="small"
        @click="invokeAddNewNoteCommand()"
      >
        <i class="fas fa-save" style="font-size: 11px; margin-top: 2px"></i>
        <span class="mt-1 ml-1">Save</span>
      </v-btn>
    </div>
  </v-container>
</template>

<script>
import { invoke } from "@tauri-apps/api";

export default {
  name: "CreateNotePage",

  data() {
    return {
      title: "",
      description: "",

      titleErr: "",
      descriptionErr: "",
    };
  },

  methods: {
    invokeAddNewNoteCommand: async function () {
      this.titleErr = "";
      this.descriptionErr = "";

      try {
        let res = await invoke("add_new_note", {
          title: this.title,
          description: this.description,
        });
        if (res == "success") this.$router.push("/notes-page");
      } catch (err) {
        if (err == "InvalidTitle") {
          this.titleErr = "Invalid Title";
        } else if (err == "InvalidDescription") {
          this.descriptionErr = "Invalid Description";
        }
      }
    },
  },
};
</script>
