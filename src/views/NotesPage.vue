<template>
  <v-container>
    <div class="text-right px-7">
      <Search @searchTyping="handleSearchTyping" />
      <v-btn
        class="text-capitalize"
        color="teal-darken-3"
        @click="$router.push('/create-note-page')"
      >
        <i class="fas fa-plus-square"></i>
        <span class="mt-1 ml-1">New Note</span>
      </v-btn>
    </div>

    <div v-if="noNoteFound">
      <h3 class="text-center mt-5">No note found</h3>
    </div>
    <div v-else class="mt-5" id="notes-list">
      <div v-for="(note, index) in allNotes" :key="index">
        <Note :note="note" @deleteNoteBtnOnClick="handleDeleteNoteBtnOnClick" />
      </div>
    </div>

    <v-dialog
      v-model="showDeleteNoteConfirmDialog"
      width="400"
      height="170"
      persistent
    >
      <v-card
        width="400"
        height="170"
        class="px-4 py-5"
        style="background-color: #212121; color: white"
      >
        <h4 class="text-center">Are you sure you want to delete</h4>
        <h4 class="text-center">{{ selectedNoteToDelete.title }} ?</h4>
        <div class="text-right mt-10">
          <v-btn
            class="text-capitalize pt-1 mr-3"
            size="small"
            @click="closeDeleteNoteConfirmDialog()"
          >
            Cancel
          </v-btn>
          <v-btn
            class="text-capitalize pt-1"
            size="small"
            color="red"
            @click="invokeDeleteNoteCommand()"
          >
            Delete Now
          </v-btn>
        </div>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<script>
import { invoke } from "@tauri-apps/api";
import Note from "../components/Note.vue";
import Search from "../components/Search.vue";

export default {
  name: "NotesPage",
  components: {
    Note,
    Search,
  },
  async created() {
    await this.invokeGetAllNotesCommand();
  },
  data() {
    return {
      allNotes: [],
      showDeleteNoteConfirmDialog: false,
      selectedNoteToDelete: Object,
      noNoteFound: false,
    };
  },
  methods: {
    invokeGetAllNotesCommand: async function () {
      this.noNoteFound = false;
      try {
        let notes = await invoke("get_all_notes");
        if (notes.length > 0) {
          this.allNotes = notes;
        } else {
          this.noNoteFound = true;
        }
      } catch (_) {}
    },

    handleDeleteNoteBtnOnClick: function (note) {
      this.selectedNoteToDelete = note;
      this.showDeleteNoteConfirmDialog = true;
    },

    closeDeleteNoteConfirmDialog: function () {
      this.showDeleteNoteConfirmDialog = false;
      setTimeout(() => {
        this.selectedNoteToDelete = Object;
      }, 500);
    },

    invokeDeleteNoteCommand: async function () {
      try {
        let res = await invoke("delete_note", {
          noteId: this.selectedNoteToDelete.id,
        });
        if (res == "success") {
          for (const i in this.allNotes) {
            if (this.allNotes[i].id == this.selectedNoteToDelete.id) {
              this.allNotes.splice(i, 1);
              this.closeDeleteNoteConfirmDialog();
              break;
            }
          }
        }
      } catch (_) {}
    },

    invokeSearchNotesCommand: async function (searchText) {
      this.noNoteFound = false;

      try {
        let results = await invoke("search_notes", { searchText: searchText });
        if (results.length > 0) {
          this.allNotes = results;
        } else {
          this.noNoteFound = true;
        }
      } catch (_) {}
    },

    handleSearchTyping: async function (searchText) {
      if (searchText.trim() === "") await this.invokeGetAllNotesCommand();
      else await this.invokeSearchNotesCommand(searchText);
    },
  },
};
</script>

<style scoped>
#notes-list {
  height: 70vh;
  overflow-y: auto;
  padding-right: 18px;
  padding-left: 18px;
}
</style>
